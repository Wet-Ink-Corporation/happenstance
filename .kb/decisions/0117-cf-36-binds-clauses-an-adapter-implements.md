---
id: kb-decision-0117
title: CF-36 binds the clauses an adapter implements, which empties CF36_UNDISCHARGED
kind: decision
status: proposed
authority_tier: decision
adr_id: ADR-0117
reversibility: medium
phase: 20
supersedes: null
superseded_by: null
summary: >-
  Proposed 2026-11-25 by the v1 AFK run for the owner (#100). CF-36 forbids a clause backed only by
  integration- or scenario-level cases from naming a conformance rule, because an adapter author
  cannot run such a rule against their own crate. spec-trace checks a wider thing: every
  backticked name on a Rule line. PS-29, PS-30 and SY-34 are the last three entries in
  CF36_UNDISCHARGED, and each names a rule that lives in this workspace's own crate tests
  (run_fan_out's, and happenstance-sync's Edge), registered Elsewhere, not a conformance rule.
  Recommended: state that CF-36 binds clauses an adapter implements, narrow the checker to rules
  the conformance suites resolve, and empty the list, which closes #100. The flip is a decision
  that the runners or Edge are meant to be reimplemented by third parties.
depends_on:
  - kb-decision-0066
related:
  - kb-open-question-cf-36-unperformed-cross-reference-001
source_paths:
  - spec/SPECIFICATION.md
  - xtask/src/spec_trace.rs
  - xtask/src/spec_trace/sync_family.rs
last_reviewed: 2026-11-25
---

# CF-36 binds the clauses an adapter implements, which empties `CF36_UNDISCHARGED`

**Proposed 2026-11-25** by the unattended v1 run, for the owner (#100). Nothing below has been
applied.

## Context

CF-36 (`[FROZEN]`, `spec/SPECIFICATION.md` §6, about `:10457`): "A clause backed only by
integration- or scenario-level cases MUST NOT name a conformance rule." Its `Rejects:` gives the
reason: such a rule "cannot be run by an adapter author against their own crate". The same
paragraph still calls `happenstance-sync-testkit` "not yet existing", which has been stale since
`0.4.4`.

The checker is wider than the clause. `check_case_levels` and `reconcile_cf36`
(`xtask/src/spec_trace.rs`) test every name `rules_of` collects from a `Rule:` line, that is every
backticked identifier, conformance rule or not. `0.8.0` took the breaches from thirteen to three
(#521). The three left in `CF36_UNDISCHARGED`:

| Clause | Constrains | Its rule lives in | Its only case |
| --- | --- | --- | --- |
| PS-29 | the typed fan-out runner, `run_fan_out` | `crates/happenstance/tests/fan_out/rules.rs` | E2E-28 (integration) |
| PS-30 | the same runner | the same file | E2E-28 (integration) |
| SY-34 | `happenstance-sync`'s shipped `Edge` | `crates/happenstance-sync/tests/runner/main.rs` | E2E-45 (integration) |

None of the three rules is a conformance rule; each is registered `Unresolvable::Elsewhere`. None
of the three clauses constrains anything an adapter implements: they constrain library code this
workspace ships and tests in its own crate.

## Options

- **A. State the scope CF-36 already has (recommended).** Append to CF-36: *"This clause binds the
  clauses an adapter implements. A clause whose rule runs against library code this workspace
  ships — the typed runners, `happenstance-sync`'s `Edge` — and is registered `Elsewhere` in that
  crate's own tests names no conformance rule and is outside it."* Fix the stale sync-testkit
  sentence. In `spec_trace.rs`, count only rules the conformance suites resolve, so an
  `Elsewhere` rule is not a breach; the three entries then stop being breaches, the "can only
  shrink" reconciliation fails until they go, and `CF36_UNDISCHARGED` and `reconcile_cf36` can be
  removed. Arguably a clarification: the clause already says *conformance rule*.
- B. Give each of the three a contract-level case. There is no contract-level case to give: the
  runner and `Edge` are not adapter surfaces, so any such case would be invented to satisfy the
  check.
- C. Keep the list, and phase 21's audit reads three recorded breaches as accepted. Leaves a
  standing exception list in a 1.0 audit that says it has none.

## Decision (proposed)

A. On acceptance: CF-36 gains the scoping sentence (a `[FROZEN]` clause edit, made by this record);
`spec_trace.rs` narrows the check, empties or removes `CF36_UNDISCHARGED`, updates its summary line
and retargets or drops `a_cf36_record_that_outlives_its_breach_is_reported` and
`the_real_documents_breach_cf36_exactly_where_recorded`; #100 closes.

## What would flip it

A decision that the typed runners or `Edge` are meant to be reimplemented by third parties behind
a trait an outside implementer satisfies. Their rules would then belong in a family others mount,
and CF-36 would bind them as written.

## Door

One-way: it edits a `[FROZEN]` clause's text, though it admits the same set of adapters.
