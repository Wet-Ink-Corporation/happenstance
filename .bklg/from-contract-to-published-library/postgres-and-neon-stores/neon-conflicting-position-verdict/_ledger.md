---
item: "HS-S0070"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — conflicting_position settled by evidence, and the ledger corrected

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: "GIVEN an adapter author who has been told ProbeThenWriteStore is this workspace's named wrong implementation for Neon, WHEN they run the adapter's tests the ordinary way — the credentialed job's `cargo test -p happenstance-neon --all-features -- --ignored --show-output`, with no workflow edit — THEN the store actually executes against the live endpoint over its two round trips (all four remaining bodies real, compiled against the same NeonConfig table names and the same tag encoding the CTE uses) instead of panicking at a todo!(); AND the same invocation with no credential present exits zero rather than failing, because the gating is whole-invocation and never a #[cfg] hiding a rule out of a macro expansion (DR-5)."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/tests/conflicting_position.rs"
  verifying_test: "crates/happenstance-neon/tests/conflicting_position.rs::probe_then_write_store_appends_over_two_round_trips"

- id: AC-002
  criterion: "GIVEN an evaluator who knows that a single-client \"conflict comes back\" run proves nothing, WHEN they read what the harness did, THEN a real second writer landed a conflicting append onto the same fixture-isolated backing store, through a second handle, strictly between the probe's response and the insert's request — closed by an interposing SqlTransport, not by a sleep, a clock read, a thread pile-up or a retry-until-green loop; AND the run fails loudly if that interleaving did not in fact occur, rather than reporting a pass it did not earn."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/tests/conflicting_position.rs"
  verifying_test: "crates/happenstance-neon/tests/conflicting_position.rs::interposed_writer_lands_inside_the_probe_insert_window"

- id: AC-003
  criterion: "GIVEN an adapter author asking whether this workspace's compliance claim can distinguish a one-snapshot CTE from two lucky statements, WHEN the identical contended harness is driven over both stores in one run, THEN ProbeThenWriteStore fails a rule by name and NeonEventStore passes that same rule, the failure asserted through catch_unwind over a non-capturing probe (Probe::run stays a function pointer; no AssertUnwindSafe appears anywhere); AND the --show-output log names the rule that panicked and quotes the fixture's own words, so the verdict is legible from the log alone and not only from the exit code."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/tests/conflicting_position.rs"
  verifying_test: "crates/happenstance-neon/tests/conflicting_position.rs::probe_then_write_fails_the_rule_the_cte_passes"

- id: AC-004
  criterion: "GIVEN the same author, WHEN no rule in the existing set rejects ProbeThenWriteStore under contention, THEN that is recorded as a finding about the rule set and the rule is fixed in the same change — schedule and body only, never the name (spec-trace check 6 requires every rule in RULE_FILES to be claimed by a clause) — with its reason stated in the change and a CHANGELOG.md entry naming the defect it now detects; AND proceeding without either is not available."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/tests/conflicting_position.rs"
  verifying_test: "crates/happenstance-neon/tests/conflicting_position.rs::probe_then_write_fails_the_rule_the_cte_passes (with `cargo xtask spec-trace` and `cargo xtask lint-changelog`)"

- id: AC-005
  criterion: "GIVEN an evaluator reading the crate's claim that \"contrary to the standing assumption in the decision ledger, the collapse keeps ConditionViolated::conflicting_position\", WHEN the CTE meets a real /sql endpoint under the AC-002 contention, THEN the claim is settled by evidence: either the append is rejected with Some(p) where p is the position the store assigned to the interposed append — read back from the append that assigned it, never a literal (CF-6 holds here by discipline; lint-position-literals does not sweep crates/happenstance-neon/tests/) — or the refutation is recorded with what the endpoint actually returned."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/tests/conflicting_position.rs"
  verifying_test: "crates/happenstance-neon/tests/conflicting_position.rs::cte_reports_the_position_the_interposed_append_was_assigned"

- id: AC-006
  criterion: "GIVEN the constrained-runtime developer, for whom the CTE's soundness is the whole guarantee, WHEN they ask at what isolation the append actually ran, THEN the spec's open question is answered against the endpoint, not reasoned about: whether Neon-Batch-Isolation-Level reaches a one-statement request at all, and — if it does not — either the minimal re-shaping of the append into a form the header applies to, or the unsoundness recorded as the finding; AND a SQLSTATE 40001 serialization abort is surfaced through NeonSqlError::is_serialization_failure and recorded as a retryable abort, never laundered into either a pass or a conflict rejection."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/tests/conflicting_position.rs"
  verifying_test: "crates/happenstance-neon/tests/conflicting_position.rs::append_isolation_level_observed_at_the_endpoint"

- id: AC-007
  criterion: "GIVEN the publication-and-positioning (HS-P0016) auditor who will later have to say what the contract promises, WHEN the endpoint shows the CTE's append semantics unsound — a condition that fails to reject, not merely a missing field — THEN a decision record carrying the evidence is staged in .kb/_intake/ before any behaviour change, reaching .kb/decisions/ only through /redkiln:kb-ingest and never hand-authored; AND where the outcome is instead a missing conflicting_position, no record is owed at all and none is written, because ES-25 already permits None."
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/ (staged record) — the branch reached from crates/happenstance-neon/tests/conflicting_position.rs"
  verifying_test: "`redkiln validate --kb` && `redkiln doctor` (accepted-atom immutability + no hand-authored atom), read against spec/SPECIFICATION.md:3746-3751"

- id: AC-008
  criterion: "GIVEN anyone who later reads the plan of record to learn why conflicting_position is only a hint, WHEN they read it after this PR, THEN they meet the evidence and not the conjecture: RUNBOOK.md:479 and RUNBOOK.md:3113-3116 no longer read as they do today, references/adapter-shapes.md:116-125 carries the verdict beside its \"unlooked-for result\", and crates/happenstance-neon/src/lib.rs:46-77 carries the live-endpoint evidence and the isolation finding — the correction landing at the wrong sentences, not in a new document beside them; AND ADR-0012 is neither edited nor superseded, its body byte-identical to HEAD, because its conclusion is correct under both outcomes."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/tests/conflicting_position.rs (credential-free assertion); correction targets RUNBOOK.md:479, RUNBOOK.md:3113-3116, references/adapter-shapes.md:116-125, crates/happenstance-neon/src/lib.rs:46-77"
  verifying_test: "crates/happenstance-neon/tests/conflicting_position.rs::ledger_targets_are_corrected"
```
