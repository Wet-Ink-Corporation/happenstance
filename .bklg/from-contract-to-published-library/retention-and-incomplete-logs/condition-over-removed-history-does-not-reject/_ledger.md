---
item: HS-S0117
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — ES-40's rule, the vacuous pass as specified behaviour, and its documentation

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
  criterion: "**The adapter author's suite says \"this is the contract\", not \"your store is broken\".** GIVEN a store that has had matching history removed — HS-S0114's instrument, in **both** DA-1 configurations (suffix and scattered) — WHEN the adapter author runs `event_store_conformance!` over it, THEN `condition_over_removed_history_does_not_reject` **passes**, because an `append` carrying a condition whose query ranges over the removed events returned `Ok`; and no assertion anywhere in that body demands rejection for the removed-history case. The rule asserts ES-40's *specified* outcome (`spec/SPECIFICATION.md:4353-4357`), never the outcome a first reader wants"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs:94 — the `for_each_event_store_rule!` invocation list; the run reached through `event_store_conformance!` on HS-S0114's instrument target"
  verifying_test: "crates/happenstance-testkit/src/suite.rs::rules::condition_over_removed_history_does_not_reject, run against both configurations via `cargo test -p happenstance-testkit --test completeness_instrument` (or whatever target HS-S0114 landed)"

- id: AC-002
  criterion: "**The constrained-runtime developer's wasm32 CI runs the same rule, from the same one line.** GIVEN the rule body exists in `suite.rs`, WHEN its name is added exactly once to `for_each_event_store_rule!` at `crates/happenstance-testkit/src/registry.rs:94`, THEN it is emitted by all four emitters — `__emit_tokio`, `__emit_blocking`, `__emit_wasm`, `__emit_rule_names` — and by `for_each_mutant!`, with no second registration and no `#[cfg]` narrowing it to a subset of targets; the body is generic over `F: Fixture` whose `Store: EventStore` (the weaker bound, both flavours) with only one of `EventStore`/`SendEventStore` in scope"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs:94 — the `for_each_event_store_rule!` invocation list, the only place the rule set is written down"
  verifying_test: "crates/happenstance-testkit/tests/memory_conformance_wasm.rs and memory_conformance_blocking.rs (the emitted rule present), plus `cargo xtask wasm` and `crates/happenstance-testkit/src/registry.rs::no_orphan_rules`"

- id: AC-003
  criterion: "**The rule cannot pass by doing nothing.** GIVEN a store whose `append` never rejects any condition at all, WHEN the rule runs against it, THEN the rule **fails** — because the body pairs the admission with a liveness mirror asserting that the *same* condition, evaluated where the matching history is still retained, is rejected. GIVEN the capability gate means most registered mutants skip this rule, THEN the mirror plus this story's own mutant are the only things standing between it and decoration, and that is stated in the rule's rustdoc"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/suite.rs — the rule body registered at crates/happenstance-testkit/src/registry.rs:94"
  verifying_test: "crates/happenstance-testkit/tests/mutation_coverage.rs::every_rule_has_a_mutant (:2734) via `cargo test -p happenstance-testkit --test mutation_coverage`, plus the inversion demonstration recorded in implementation-report.md"

- id: AC-004
  criterion: "**The rule stays true on a store whose positions look nothing like the instrument's.** GIVEN a conformant store that leaves gaps or starts numbering anywhere it likes, WHEN the rule builds its condition, THEN every boundary it uses (`AppendCondition::after_opt`) came from a position that store actually assigned — captured from the append's return or from a read — and no literal position value appears anywhere in the body. CF-6 is live here because the instrument's first retained position differs between the two DA-1 configurations"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/suite.rs — the rule body's condition construction, registered at crates/happenstance-testkit/src/registry.rs:94"
  verifying_test: "`cargo xtask lints` — the CF-6 check at xtask/src/lints.rs:600-655; plus GappedPositionStore in crates/happenstance-testkit/tests/mutation_coverage/mutants.rs via `cargo test -p happenstance-testkit --test mutation_coverage`"

- id: AC-005
  criterion: "**A store that cannot forget says so, and no existing `Fixture` impl breaks.** GIVEN a fixture that cannot be made to remove history (`MemoryFixture`, `LocalMemoryEventStore`, `DurableFixture`, every mutant but this story's), WHEN the suite runs, THEN the rule still emits a test which reports `RuleOutcome::Skipped` carrying that fixture's own stated reason — never `#[cfg]`-ed out of the binary — and the removal seam it gates on is HS-S0116's if HS-S0116 landed one, or otherwise exactly one **defaulted** `Capability` const plus one **defaulted** panicking method in `MID_BATCH_FAULT`'s shape, with its matching line in `declines()`. No required trait item is added"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/contract.rs — the `Fixture` capability seam consumed by `require!` in crates/happenstance-testkit/src/suite.rs:37; the rule registered at crates/happenstance-testkit/src/registry.rs:94"
  verifying_test: "crates/happenstance-testkit/tests/memory_conformance.rs and local_conformance.rs (the emitted test reporting its skip reason), plus crates/happenstance-testkit/tests/mutation_coverage/harness.rs::declines (:520-546) exercised by `cargo test -p happenstance-testkit --test mutation_coverage`"

- id: AC-006
  criterion: "**A store that fails closed on its own gap is caught by name, not by \"conformance failed\".** GIVEN `GapAwareRejectingStore` — otherwise conformant, declaring the removal capability `SUPPORTED`, and rejecting (or erroring on; exactly one of the two, with the loser's reason recorded) a condition whose query ranges over the hole once removal has been requested — WHEN the mutation harness runs, THEN it fails **exactly** `condition_over_removed_history_does_not_reject`, its `REGISTRY` row carries a non-empty `fails`, a real `provenance`, a `mode` and an `expect` pin naming the exact assertion tripped, and both meta-tests are green in both directions"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/mutation_coverage.rs:324 — the `REGISTRY`, driven through `for_each_mutant!` (:2056) against the rule registered at crates/happenstance-testkit/src/registry.rs:94"
  verifying_test: "crates/happenstance-testkit/tests/mutation_coverage.rs::mutant_registry_is_exhaustive (:2754) and ::mutants_fail_exactly_their_declared_rules (:2889) via `cargo test -p happenstance-testkit --test mutation_coverage`"

- id: AC-007
  criterion: "**The ingest author recognises their own situation in the sentence, and the adapter author recognises theirs.** GIVEN ES-40's documentation MUST, WHEN the two rustdoc blocks land — on `AppendCondition::is_violated_by` (`crates/happenstance-core/src/append.rs:225-234`) stating that a condition is a claim about the log **the evaluating store holds**, and on `EventStore::append` (`crates/happenstance-core/src/store.rs:213`) stating the same fact as the hazard that re-evaluating an origin condition against a pruned slice and concluding \"no match\" is the **normal** path under unconditional ingest — THEN each links the other and both link `.kb/concepts/torn-reads-and-the-append-condition-boundary.md` rather than restating it; and the diff for those two files contains **no line outside a `///` block**, so no signature in `happenstance-core` changed"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/append.rs (`AppendCondition::is_violated_by`, :225-234) and crates/happenstance-core/src/store.rs:213 (`EventStore::append`) — rustdoc only, no signature change"
  verifying_test: "`cargo test -p happenstance-core --doc` and `cargo doc -p happenstance-core` (intra-doc links resolve); plus the diff check that every changed line in those two files is inside a `///` block, and `cargo-semver-checks` at the PR grain per CONTRIBUTING.md:291-296"

- id: AC-008
  criterion: "**The evaluator is not told a case is discharged when it is not.** GIVEN E2E-47 asks for a third outcome — *\"this log cannot evaluate this condition\"* rather than \"no match\" (`spec/E2E-CASES.md:1232-1246`) — and ES-40 forbids implying one exists, WHEN this rule lands, THEN one sentence in its rustdoc and one row in this story's evidence state that E2E-47's `THEN` is **not** met, name DA-7 row 3 (a new condition-evaluation outcome, a `0.3.0`) as the only shape that would meet it, and route the escalation to HS-S0124; and no such type, variant or signature is added here"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/suite.rs — the rule's own rustdoc, beside the body registered at crates/happenstance-testkit/src/registry.rs:94"
  verifying_test: "`cargo doc -p happenstance-testkit` (the sentence compiles in place) plus the diff check that no new public type or variant appears in crates/happenstance-core/src/append.rs, store.rs or error.rs, confirmed by `cargo-semver-checks` at the PR grain"

- id: AC-009
  criterion: "**The record the evaluator reads is true on the day this merges.** GIVEN writing the body makes ES-40's rule name resolvable, WHEN the commit lands, THEN §7.2's ES-40 cell has lost its `†` via `cargo xtask spec-trace --write` (nothing else in the file changed except ES-40's `append.rs` citation if the rustdoc moved those lines), `CHANGELOG.md`'s `[Unreleased]` carries an entry of at least 120 characters naming the defect the rule detects, and ES-40's `[PROVISIONAL]` marker, the `(new)` on its `Rule:` line and §1.3's hand census are **untouched** — those are HS-S0123's"
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md §7.2 (the generated region) and CHANGELOG.md `[Unreleased]`, gated through `cargo xtask affected --base main` per .redkiln/config.yaml:36-40"
  verifying_test: "`cargo xtask spec-trace` (checks 4, 6, 8, 9 — xtask/src/spec_trace.rs:695-696, :727-733, :1174-1179), `cargo xtask lints` CF-29 (xtask/src/lints.rs:525), and `cargo xtask affected --base main`"
```
