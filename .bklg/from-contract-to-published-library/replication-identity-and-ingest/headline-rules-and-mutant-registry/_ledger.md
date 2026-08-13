---
item: HS-S0105
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The two headline rules, green, with mutants that fail them by name

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
  criterion: "**GIVEN** an adapter author whose peer receives a batch carrying a position-free append condition, **WHEN** they run `sync_peer_conformance!(MyFixture::new())`, **THEN** `ingest_never_rejects` runs and reports pass only if — after two stores each accepted a conflicting fact under **byte-identical, position-free** conditions and a full exchange ran in both directions — **both facts are present in both logs, keyed by `EventId`**; a peer that returns `Ok` and drops or adjudicates away the foreign fact fails, and neither `Ok`-ness, event counts nor positions are ever the assertion"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/src/registry.rs — the `ingest_never_rejects` arm of `for_each_sync_peer_rule!`, reached by all three harnesses"
  verifying_test: "crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs::ingest_never_rejects (green against MemorySyncPeerFixture), with crates/happenstance-sync-testkit/tests/mutation_coverage.rs proving ConditionEvaluatingIngest fails it"

- id: AC-002
  criterion: "**GIVEN** an adapter author whose receiving peer has a domain that adjudicates an ingested fact against one it already holds, **WHEN** they run the suite, **THEN** `compensation_is_atomic_with_the_losing_event` reads the local log **at the point the fixture's `apply` returns and before anything else runs** and reports pass only if no reader could have observed the losing event with nothing resolving it — a receiver that ingests, returns, and compensates from a follow-up fails, even though its log converges at rest"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/src/registry.rs — the `compensation_is_atomic_with_the_losing_event` arm of `for_each_sync_peer_rule!`"
  verifying_test: "crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs::compensation_is_atomic_with_the_losing_event, with crates/happenstance-sync-testkit/tests/mutation_coverage.rs proving CompensateAfterCommit fails it at its pinned assertion"

- id: AC-003
  criterion: "**GIVEN** an adapter author whose peer is handed an `EventGroup` whose `guard` carries `after: Some(p)` — where `p` is a position the **origin** store actually assigned — **WHEN** they run the suite, **THEN** `wire_condition_with_after_is_refused` reports pass only if the receiving side both (a) returns `Err` from `apply` and (b) leaves its log unchanged against a head **it itself reported**; the rule matches no error variant, so a peer whose error enum is its own still passes"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/src/registry.rs — the `wire_condition_with_after_is_refused` arm of `for_each_sync_peer_rule!`"
  verifying_test: "crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs::wire_condition_with_after_is_refused, with crates/happenstance-sync-testkit/tests/mutation_coverage.rs proving PositionPortingPeer fails it"

- id: AC-004
  criterion: "**GIVEN** an adapter author who wants to know the suite can tell them apart from a plausible mistake, **WHEN** they read `tests/`, **THEN** `ConditionEvaluatingIngest` exists as a compiled peer that evaluates the origin's *position-free* guard locally and routes `AppendError::ConditionViolated` into a rejection, is not built by wrapping `MemorySyncPeer`, declares **exactly** `ingest_never_rejects`, and **passes the other two rules**"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/tests/mutation_coverage.rs — the `Declared` const table row for ConditionEvaluatingIngest, run against the enumeration in crates/happenstance-sync-testkit/src/registry.rs"
  verifying_test: "crates/happenstance-sync-testkit/tests/mutation_coverage.rs::mutants_fail_exactly_their_declared_rules"

- id: AC-005
  criterion: "**GIVEN** the same author, **WHEN** they read `tests/`, **THEN** `PositionPortingPeer` exists as a compiled peer that accepts an `after`-carrying guard and evaluates it **in its own numbering**, evaluates *only* `after: Some(_)` guards, declares **exactly** `wire_condition_with_after_is_refused`, and passes the other two — the mutant this project is named after and the one a counting rule cannot see"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/tests/mutation_coverage.rs — the `Declared` const table row for PositionPortingPeer, with its per-rule `expect` pin"
  verifying_test: "crates/happenstance-sync-testkit/tests/mutation_coverage.rs::mutants_fail_exactly_their_declared_rules"

- id: AC-006
  criterion: "**GIVEN** the same author, **WHEN** they read `tests/`, **THEN** `CompensateAfterCommit` exists as a compiled peer that ingests the losing event, returns, and authors the compensation afterwards, declares `DOMAIN_ADJUDICATION` **available** so the rule it targets cannot skip past it, declares **exactly** `compensation_is_atomic_with_the_losing_event`, and passes the other two"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/tests/mutation_coverage.rs — the `Declared` const table row for CompensateAfterCommit"
  verifying_test: "crates/happenstance-sync-testkit/tests/mutation_coverage.rs::mutants_fail_exactly_their_declared_rules and ::every_rule_has_a_mutant"

- id: AC-007
  criterion: "**GIVEN** an adapter author who does not trust a suite's self-report, **WHEN** the sync suite runs, **THEN** the mutant set is **data** — a `const` table of `Declared { name, kind, fails, provenance, mode, expect }` in `tests/` in the shape the event-store registry already uses — and five meta-tests hold it: every rule has a mutant (CF-1), the registry is exhaustive (CF-2), each mutant fails exactly its declared rules and passes every other (CF-3), every mutant states its provenance (CF-4), and the conformant variants pass everything (CF-5)"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/tests/mutation_coverage.rs — the `Declared` const table, driven by the enumeration in crates/happenstance-sync-testkit/src/registry.rs"
  verifying_test: "crates/happenstance-sync-testkit/tests/mutation_coverage.rs::{every_rule_has_a_mutant, mutant_registry_is_exhaustive, mutants_fail_exactly_their_declared_rules, every_mutant_states_its_provenance, conformant_variants_pass_everything}"

- id: AC-008
  criterion: "**GIVEN** an adapter author reading a failure report, **WHEN** a mutant is named in it, **THEN** its `provenance` is non-empty and argues, at the register of the strings already in the tree, why *someone would ship this* — SY-1's *\"the natural first cut… it passes every event-store conformance rule\"*, SY-6's *\"worse than no check, because it looks like enforcement\"*, SY-2's hub log that says one physical compressor is held twice — never `AlwaysWrong`, never a label"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/tests/mutation_coverage.rs — the `provenance` field of each `Declared` row"
  verifying_test: "crates/happenstance-sync-testkit/tests/mutation_coverage.rs::every_mutant_states_its_provenance"

- id: AC-009
  criterion: "**GIVEN** an adapter author whose store assigns positions with gaps or outside the transaction, **WHEN** they run the three new rules, **THEN** none of them asserts on a literal position value: membership is by `EventId`, adjacency is measured relative to a head the receiving store reported, and SY-6's `after` is built from a position the origin store actually assigned"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/src/rules.rs — the three rule bodies, inside the lint scope gate-mounts-for-the-sync-suite added to xtask/src/lints.rs"
  verifying_test: "cargo xtask lint-position-literals over the sync-suite scope (xtask/src/lints.rs), plus crates/happenstance-sync-testkit/tests/mutation_coverage.rs::conformant_variants_pass_everything"

- id: AC-010
  criterion: "**GIVEN** an evaluator or adapter author relying on payloads staying opaque, **WHEN** the whole suite runs, **THEN** no rule and no wrong peer reads `Event::data` or `Event::metadata` structurally on any path — facts are distinguished by `EventId`, `EventType` and `Tags` alone, so the suite could never certify a peer that decodes"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/src/rules.rs and crates/happenstance-sync-testkit/tests/** — every path the suite runs"
  verifying_test: "cargo test -p happenstance-sync-testkit --all-features (payload Bytes compared only for equality) plus the reviewed absence of any Event::data / Event::metadata read in crates/happenstance-sync-testkit/src/rules.rs"

- id: AC-011
  criterion: "**GIVEN** a reader of the changelog deciding whether to upgrade the testkit, **WHEN** they read the entry for this release, **THEN** each of the three rules has its own entry naming the **defect it detects** (CF-29), and the registry's own doc states which defects the set covers and which axes it leaves uncovered — **never a pass rate**, because the denominator is a choice"
  satisfied: false
  evidence: ""
  mount_point: "CHANGELOG.md — the three per-rule entries, read by the changelog lint over the RULE_FILES set at xtask/src/lints.rs:505-540"
  verifying_test: "cargo xtask lints — the changelog-per-rule lint (xtask/src/lints.rs:505-540)"

- id: AC-012
  criterion: "**GIVEN** an adapter author whose peer has **no adjudicating domain**, **WHEN** they run the suite, **THEN** they still get a verdict on every rule: `SyncPeerFixture` carries the one required `apply` seam (documented, with an `# Errors` section) through which all three rules reach the receiving side, and a declined `DOMAIN_ADJUDICATION` leaves `compensation_is_atomic_with_the_losing_event` **emitted, run, and reported as skipped with the fixture's own stated reason** — never `#[cfg]`-ed away, never silently green"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/src/contract.rs — the `apply` method and `DOMAIN_ADJUDICATION` on `SyncPeerFixture`, implemented by crates/happenstance-sync-testkit/src/fixtures.rs"
  verifying_test: "crates/happenstance-sync-testkit/tests/mutation_coverage.rs (the declining fixture reporting its stated reason for compensation_is_atomic_with_the_losing_event) and the rustdoc example on `SyncPeerFixture::apply` run by the testkit's out-of-package doctest harness"

- id: AC-013
  criterion: "**GIVEN** an adapter author who wrote one `sync_peer_conformance!` invocation and edited no harness, **WHEN** they run it under tokio, under the blocking bridge and under `wasm32`, **THEN** all three rules appear **by name** in all three, because the only place they are written down is `for_each_sync_peer_rule!` — and the record is left true: `spec/SPECIFICATION.md` is unedited, with the `(new)` markers on SY-1/SY-2/SY-6 and the `SyncPeer::push` doc-comment finding recorded for `frozen-clause-repairs` (HS-S0112)"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/src/registry.rs — `for_each_sync_peer_rule!`, the single enumeration all three harnesses expand"
  verifying_test: "crates/happenstance-sync-testkit/src/registry.rs::no_orphan_sync_rules plus the three harness runs at crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs, memory_peer_conformance_blocking.rs and memory_peer_conformance_wasm.rs; cargo xtask spec-trace green with spec/ absent from the diff"
```
