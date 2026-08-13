---
item: HS-S0042
stage: implement
created: 2026-08-12
updated: 2026-08-12
---

# Acceptance ledger — The model family green, and SqliteEventStore in the mutant pass column

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Three rows carry a **recorded verdict** as part of their evidence, not just a green test: AC-005 (why
testkit-side pass-column registration was refused), AC-007 (why no `mutants.rs` `REGISTRY` row is
possible for this defect), and AC-004 (the shrunk op sequence behind any adapter-side fix). A green
gate is a precondition for reading these criteria, never a substitute (`RUNBOOK.md:38-42`).

```yaml
- id: AC-001
  criterion: "The generated space answers for this store. GIVEN an adapter author who has event_store_conformance! green against SqliteFixture and knows that green only means \"the examples somebody wrote down pass\", WHEN they run cargo test -p happenstance-sqlite, THEN event_store_model_conformance!(SqliteFixture::new()) runs at the same mount point and the single rule ops_agree_with_the_model reports Ran — not absent, not Skipped — across the generator's full query × from × backwards × limit × condition × position policy space against a real file, so the author learns whether their SQL agrees with the contract on sequences nobody chose."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/conformance.rs"
  verifying_test: "cargo test -p happenstance-sqlite --test conformance -- dcb_model_conformance::ops_agree_with_the_model"

- id: AC-002
  criterion: "No second flag, no second command. GIVEN the same author, who has been told once that cargo test -p happenstance-sqlite is this project's conformance command (_decomposition.md testing brief §2, §6), WHEN they type exactly that with no extra feature flag, THEN the model family appears in the run — because the proptest feature is enabled at the dev-dependency site (happenstance-testkit = { workspace = true, features = [\"proptest\"] }) and the invocation is gated on not(target_arch = \"wasm32\") alone, a crate being unable to cfg on a dependency's feature — and a wasm32 check of this crate still compiles with the family absent rather than broken."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/conformance.rs (invocation cfg) + crates/happenstance-sqlite/Cargo.toml [dev-dependencies]"
  verifying_test: "cargo test -p happenstance-sqlite (no --features) shows dcb_model_conformance; cargo xtask wasm"

- id: AC-003
  criterion: "The emitter that cannot work is declined out loud, not omitted. GIVEN an author who reads the reference harness and sees it invoke both shipped emitters (memory_model_conformance.rs:21-25, 31-37) and wonders why this adapter invokes one, WHEN they open the harness module doc at the mount point, THEN they find the blocking emitter declined with its mechanical reason — __emit_model_blocking runs under the testkit's own block_on with no tokio runtime, and this adapter's read stream resolves spawn_blocking via Handle::try_current, so the read path yields SqliteEventStoreError::NoRuntime — and the note says CF-23 is satisfied for this family by the testkit's own harness, so the reader is not left to conclude that a second emitter was forgotten."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/conformance.rs (module doc)"
  verifying_test: "cargo test -p happenstance-sqlite --test conformance (module doc compiles) + review against crates/happenstance-testkit/src/model.rs:731-766 and crates/happenstance-sqlite/src/event_store.rs:20-32"

- id: AC-004
  criterion: "A generated disagreement is fixed in the adapter, and the sequence that found it is kept. GIVEN a red model run, WHEN the author diagnoses it, THEN the fix lands in crates/happenstance-sqlite/src/** (the wide-query merge, a page boundary, a condition probe) rather than in a narrowed invocation or a weakened rule; no assertion added anywhere in this story names a literal position value, because AUTOINCREMENT may gap after a delete and the generator emits symbolic Anchors resolved against what the store actually assigned; and the ledger row for AC-001 cites the failing op sequence. If instead the disagreement is genuinely a density or position-policy assumption in the rule, that is escalated as a CF-6 finding and not absorbed here."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/src/event_store.rs (read path) reached from crates/happenstance-sqlite/tests/conformance.rs"
  verifying_test: "cargo test -p happenstance-sqlite --test conformance green after the fix; rg -n \"ReadOptions::from\\(|position\\(\\)\\s*==\\s*[0-9]\" crates/happenstance-sqlite/tests returns nothing added; cargo xtask spec-trace"

- id: AC-005
  criterion: "The pass-column claim is discharged where it can honestly be discharged, and the refusal is findable. GIVEN a reviewer holding RUNBOOK.md:4221-4222 (\"the phase-3 mutant harness re-run with SqliteEventStore in the pass column\") and about to file \"the adapter was never registered\" as a gap, WHEN they read this story's ledger row and the harness module doc, THEN they find the verdict: every registered rule driven against a conformant SqliteFixture is what conformant_variants_pass_everything asserts (architecture brief §10's recommended reading), plus the two reasons testkit-side registration was refused — a published testkit dev-depending on its own consumer, and rusqlite + tokio dragged into a harness built to need neither — and the harness's own meta-tests are re-run green in the same PR."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:156-191 (the REQUIRED 'each phase's proof artefacts' gate step) + crates/happenstance-sqlite/tests/conformance.rs (module doc verdict)"
  verifying_test: "cargo xtask ci --fast → mutation_coverage::conformant_variants_pass_everything and mutation_coverage::the_model_rule_rejects_exactly_what_it_claims (xtask/src/proof.rs:85-102)"

- id: AC-006
  criterion: "The racing falsifier reads in this adapter's own spelling. GIVEN an author who has just written BEGIN DEFERRED somewhere and wants to know whether the suite would catch the probe-then-insert it enables, WHEN they open RacingProbeStore, THEN its RACERS provenance and doc comment name BEGIN DEFERRED as the SQLite form of the defect — not only the driver-convenience-API spelling it carries today (racers.rs:352-366) — while its fails set and both expect pins are unchanged, so the store still fails exactly exactly_one_of_n_contenders_commits and k_disjoint_boundaries_admit_exactly_k_commits and passes every sequential rule. ES-25 [FROZEN] is discharged by re-running this demonstration, never edited."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/mutation_coverage/racers.rs:352-366 + the RACERS row in crates/happenstance-testkit/tests/mutation_coverage.rs:2534-2555"
  verifying_test: "cargo xtask ci --fast → mutation_coverage::the_concurrency_rules_reject_exactly_what_they_claim and mutation_coverage::every_mutant_states_its_provenance; cargo xtask spec-trace over spec/SPECIFICATION.md:3695-3745"

- id: AC-007
  criterion: "The impossible instruction is corrected in the tree, not dropped in silence. GIVEN the next reader — six months on — who reaches _storymap.md's line telling this story to add a BEGIN DEFERRED row to mutation_coverage/mutants.rs's REGISTRY and finds no such row, WHEN they look at the RACERS row and this story's ledger, THEN they find why it cannot exist: mutant_registry_is_exhaustive rejects a row whose fails list is empty, a probe-then-insert store fails no sequential rule by construction, and a concurrency rule's wrong store belongs in racers.rs/RACERS — cited to crates/happenstance-testkit/README.md:106-113 and crates/happenstance-testkit/tests/mutation_coverage.rs:3389-3395 — so _storymap.md's AC-007 split (\"the registry row that proves the rejection is live\") is discharged by a live falsifier plus a written correction, and is never re-opened as a gap."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/mutation_coverage.rs (comment on the RACERS row) + this _ledger.md"
  verifying_test: "cargo xtask ci --fast → mutation_coverage::mutant_registry_is_exhaustive green with no REGISTRY row added (xtask/src/proof.rs:87); review against crates/happenstance-testkit/README.md:106-113"
```
