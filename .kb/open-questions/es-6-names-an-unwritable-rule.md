---
id: kb-open-question-es-6-unwritable-rule-001
title: ES-6 is frozen and names a rule that cannot be written
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ES-6 is FROZEN and names store_error_crosses_a_join_handle as its rule, marked (new) and
  rendered with † where the legend reads "does not exist yet". The identifier occurs as no fn
  anywhere in the workspace — only in prose and in comments, one of which states the rule is
  unwritable against today's port for every adapter, with SendStoreWithLocalError as the probe.
  spec-trace's check 4 deliberately skips clauses whose rule is (new) or †, and that escape hatch
  has no expiry, so a rule scheduled forever is indistinguishable from one scheduled for next
  week. ADR-0009 is accepted and makes the rule writable, so this may be a scheduling gap rather
  than a design gap — but the clause is frozen and names an unwritten rule. Settled by writing the
  rule against ADR-0009's marker, or by deciding that a † with no owning phase is a hard failure.
  Found independently the same day by references/evaluation/review-citation-drift.md §2. ADR-0008
  and ADR-0009 are now imported as decision atoms: ADR-0008 adds that the rule, when written, must
  assert on the future's Output and not on the future, since a future-only check is decorative
  against a !Send error, and ADR-0009 supplies the ThreadSafeEventStore marker the rule's bound
  would name.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-repair-frozen-clause-001
  - kb-decision-0008
  - kb-decision-0009
  - kb-reference-port-traits-compiled-findings-001
  - kb-open-question-es-38-and-gap-read-unowned-001
source_paths:
  - .kb/_intake/gaps-owed-a-decision.md
  - .kb/_intake/0008-one-derivation-for-both-ports.md
  - .kb/_intake/0009-error-send-sync.md
  - spec/SPECIFICATION.md
  - xtask/src/spec_trace.rs
  - crates/happenstance-cloudflare/src/lib.rs
  - crates/happenstance-cloudflare/src/send_shape.rs
  - references/evaluation/review-citation-drift.md
  - docs/adr/0008-one-derivation-for-both-ports.md
  - docs/adr/0009-error-send-sync.md
last_reviewed: 2026-08-10
---

# ES-6 is frozen and names a rule that cannot be written

## What is true today

ES-6 (`spec/SPECIFICATION.md:2629`) is `[FROZEN]` and its `Rule:` field names
`store_error_crosses_a_join_handle`, marked **(new)** (`SPECIFICATION.md:2670`). §7.2's generated
table renders it with `†` (`SPECIFICATION.md:8588`), where the legend defines `†` as "does not
exist yet". That identifier occurs as no `fn` anywhere in the workspace. It occurs only in prose:
twice in `RUNBOOK.md`, once in `docs/adr/0008`, three times in `docs/adr/0009`, three times in
comments in `happenstance-cloudflare`, and in the specification itself. One of those comments
(`crates/happenstance-cloudflare/src/lib.rs:92`) states the rule "is unwritable against today's
port for *every* adapter, not merely for this one," and the probe backing that claim is
`SendStoreWithLocalError` (`crates/happenstance-cloudflare/src/send_shape.rs:94-98`), whose doc
comment reads: "If this compiles — and it does — then the *derived* flavour does not imply a
`Send` error either."

The gate does not catch this because check 4 in `xtask/src/spec_trace.rs:685` deliberately skips
any clause whose rule is declared new or marked `†`: `if c.schedules_new || !has_suite(&c.id) {
continue; }`, where `schedules_new` is set by `(new)`, `†`, a leading `new `, or `` new ` `` at
lines 1616-1620. This escape hatch is deliberate and correct in general — a clause may legitimately
schedule a rule the current phase has not written yet — but it has no expiry, so a rule scheduled
forever prints identically to one scheduled for next week.

A partial resolution already exists and has not been executed: ADR-0009 (accepted, in `docs/adr/`)
settles the underlying question that `Error` keeps `core::error::Error + 'static` on both ports and
both flavours, with the stronger property becoming a marker trait declared downstream. That makes
the rule writable, and the wrong implementation it must reject already exists in the tree. So this
may be a scheduling gap rather than a design gap — but the clause is `[FROZEN]` today and still
names an unwritten rule.

`references/evaluation/review-citation-drift.md` §2 reports the same finding independently, written
the same day from the `standards/rust/` work with no knowledge of this pass, and records the
identifier occurring in three comment locations in the crates — convergent evidence that this is a
real gap and not an artifact of one reading.

## What is not decided

Whether the rule gets written against ADR-0009's marker — after which ES-6's `(new)` and `†` come
off and the check-4 escape hatch stops applying to it — or whether a broader decision is taken
about `†` clauses that have no owning phase at all.

Both ADRs are now imported as decision atoms — `kb-decision-0008` and `kb-decision-0009` — and
between them they narrow this question without closing it. ADR-0008 was deliberately neutral on
ES-6 but recorded the shape of the rule for whoever writes it: the derived flavour genuinely admits
a `!Send` error, the failure lands at a call site demanding `F::Output: Send` rather than at the
declaration, and therefore `store_error_crosses_a_join_handle` **must assert on the future's
`Output`** — a rule that only checks the future is `Send` passes against a `!Send` error and is
decorative. ADR-0009 then supplies the bound the rule would name,
`ThreadSafeEventStore: SendEventStore<Error: Send + Sync>` with a blanket impl, compiled downstream
so the contract crate need not grow anything; it also names the rule's obligation as belonging to
an **opt-in marker-bound rule group** and moves ES-6 from `[DEFERRED]` to `[FROZEN]`. That is the
provenance of the `[FROZEN]`-with-an-unwritten-rule state this atom describes: the freeze and the
rule arrived in the same decision, and only the freeze was executed. Neither ADR writes the rule,
neither assigns it an owning phase — ADR-0009 explicitly leaves the suite to phase 3 and the
surface question of shipping the marker in `happenstance-core` to phase 4 — so the gap is a
scheduling gap with a named remedy and no owner, which is exactly the condition sub-question 2 is
about.

## What forces it

Nothing enforces a deadline today; the escape hatch's lack of expiry is exactly the mechanism that
lets this sit indefinitely without failing the gate. The intake's own judgment is that this is
worth deciding regardless of ES-6 specifically: **a `†` with no owning phase should probably be a
hard failure, and today it is silence.** That is a standing risk for every future clause marked
`(new)` or `†`, not only this one.

## Ordered sub-questions

1. Is ES-6 specifically closed first — write `store_error_crosses_a_join_handle` against
   ADR-0009's marker trait, using `SendStoreWithLocalError` as the negative probe it must reject —
   before the general policy question is settled?
2. Separately, should `xtask/src/spec_trace.rs`'s check 4 gain an expiry or an owning-phase
   requirement for any clause it currently exempts via `schedules_new`, so a `†` with no phase
   becomes a hard failure rather than permanent silence?
3. If the general policy changes, does it retroactively require an owning phase for every other
   clause in the specification currently marked `(new)` or `†`, and who audits that list once the
   policy exists?
