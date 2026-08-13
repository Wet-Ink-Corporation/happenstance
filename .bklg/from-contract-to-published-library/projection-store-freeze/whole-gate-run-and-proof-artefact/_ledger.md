---
item: HS-S0017
stage: implement
created: 2026-08-12T13:46:12.125Z
updated: 2026-08-12T13:46:12.125Z
---

# Acceptance ledger — One clean cargo xtask ci run, and the proof artefact recorded

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
  criterion: "GIVEN P4 is deciding whether this library's projection port is worth adopting and has been handed a claim that the workspace is green, WHEN they open references/evaluation/phase-6-projection-proof.md, THEN it records one full `cargo xtask ci` — not `--fast` — run on a working tree with no uncommitted changes, after `unstable-projection-gate-and-clause-disposition` (HS-S0015) and `documented-extension-surface` (HS-S0016) merged, pinned to the commit sha the run was made on, with the exit status and the wall-clock date; and it records that happenstance-postgres, happenstance-ladybug and happenstance-sqlite compiled inside that run with every affected body still todo!() and no skeleton body changed in this PR. A run recorded mid-slice, a --fast run, or a run on a dirty tree does not satisfy this row."
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/phase-6-projection-proof.md (registered from references/evaluation/README.md), recording the run of `cargo xtask ci` defined at xtask/src/main.rs"
  verifying_test: "cargo xtask ci (whole, not --fast) at the pinned sha — xtask/src/main.rs REQUIRED + OPTIONAL step tables; `git status --porcelain` empty at that sha"
- id: AC-002
  criterion: "GIVEN P2 is weighing whether this conformance suite can actually reject their mistake, WHEN they read the artefact's first section, THEN it names two tests and says which is which — the conformance rule CheckpointOnlyStore fails (commit_is_atomic_with_the_read_model, per projection-suite-entry-point's AC-006) and the meta-test that asserts it fails exactly there (the projection sibling of mutants_fail_exactly_their_declared_rules) — plus one copy-pasteable command that shows the rule failing on its own, so the reader does not have to already know the harness catches the panic. Quoting only the meta-test, or only 'the suite is green', fails this row."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — the phase-6 ARTEFACTS row for happenstance-testkit/projection_mutation_coverage, read by the REQUIRED gate step at xtask/src/main.rs:178-190; content mirrored in references/evaluation/phase-6-projection-proof.md"
  verifying_test: "xtask/src/proof.rs #[cfg(test)] — the artefact names both the conformance rule and the meta-test; plus the ARTEFACTS name assertion in xtask/src/proof.rs:183-240"
- id: AC-003
  criterion: "GIVEN P2 suspects 'two batch shapes pass' could mean the oracle wearing a hat, WHEN they read the artefact's second section, THEN it names both fixtures — MemoryProjectionFixture (apply-on-write) and the CF-5 buffering conformant variant (replay-at-commit) — states how each was observed, and states that both were observed inside the same `cargo xtask ci` invocation, so the evidence is one gate run naming two fixtures rather than two runs a reviewer reconciles by hand."
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/phase-6-projection-proof.md, written from the `tests` step of the single `cargo xtask ci` run; both fixtures reached through crates/happenstance-testkit/tests/projection_conformance.rs"
  verifying_test: "crates/happenstance-testkit/tests/projection_conformance.rs and the CF-5 buffering variant driven to completion in one run; xtask/src/proof.rs #[cfg(test)] asserting both fixture names are present in the artefact"
- id: AC-004
  criterion: "GIVEN the two names DoD 1 and DoD 2 rest on are only as durable as what holds them, WHEN a later change renames, #[ignore]s or empties the projection meta-tests, THEN `cargo xtask ci` fails at the 'each phase's proof artefacts' step with a message naming the absent tests — because ARTEFACTS gained the phase-6 rows: one for happenstance-testkit/projection_mutation_coverage whose tests list was read out of `cargo test --locked -p happenstance-testkit --all-features --test projection_mutation_coverage -- --list` rather than guessed, and one for the parity target. The check stays a subset check, so a ninth projection meta-test lands with no gate edit."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — the ARTEFACTS const (:133-149), reached from the REQUIRED step 'each phase's proof artefacts' at xtask/src/main.rs:178-190"
  verifying_test: "the proof step itself — xtask/src/proof.rs:183-240 run by `cargo xtask ci`; negative control: rename a listed test and observe the bail at xtask/src/proof.rs:207-216"
- id: AC-005
  criterion: "GIVEN a maintainer reads the gate's own stdout to learn how many mutants the projection registry declares, WHEN the proof step prints a row for each happenstance-testkit artefact, THEN each row's registry count is the count of that target's registry — because the count is keyed to the artefact (package and target, or a per-artefact optional registry path) rather than to the package alone. Printing the event-store registry's row count beside the projection target is the defect this row forbids."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — check()'s registry selection (:216-227) and registry_len() (:267-290), printed by the REQUIRED proof-artefacts gate step"
  verifying_test: "xtask/src/proof.rs #[cfg(test)] — every artefact declaring a registry names a file under its own package/target and no two share one; plus the gate's printed output showing two distinct counts"
- id: AC-006
  criterion: "GIVEN P3 ships to wasm32 and needs the suite to keep covering them without anyone remembering to update a list, WHEN an eighteenth projection rule is registered in the single enumeration and a developer forgets the wasm harness, THEN a host test target fails by name — crates/happenstance-testkit/tests/projection_harness_parity.rs obtains the rule names from for_each_projection_store_rule!(happenstance_testkit::__emit_rule_names) and asserts (a) that no harness source (projection_conformance.rs, projection_conformance_blocking.rs, projection_conformance_wasm.rs) contains any enumerated rule name as an identifier, and (b) that each of the three carries exactly one projection_store_conformance! invocation. The failure message names the offending harness and rule."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/projection_harness_parity.rs, mounted by cargo target auto-discovery and held by its own xtask/src/proof.rs ARTEFACTS row"
  verifying_test: "crates/happenstance-testkit/tests/projection_harness_parity.rs — every_enumerated_rule_reaches_all_three_emitters / no_harness_lists_a_rule_by_hand (mirroring no_orphan_rules at crates/happenstance-testkit/src/registry.rs:410-436)"
- id: AC-007
  criterion: "GIVEN P4 knows a green gate with four skipped steps is a weaker claim than a green gate with none, WHEN they read the artefact's run ledger, THEN every OPTIONAL step appears as a row with its probe and its outcome — feature powerset, wasm32 feature powerset, licences and advisories, docs.rs configuration (nightly) — and the ledger states explicitly that both feature powersets ran, since they are the only steps that would catch `conformance` being silently coupled to `memory` or `unstable-projection` failing to compile with the port gated off. A ledger that omits a step, or that reports 'green' without distinguishing ran from skipped, fails this row."
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/phase-6-projection-proof.md's run ledger, transcribed from run_steps' stdout (xtask/src/main.rs:862-892) over the OPTIONAL table (xtask/src/main.rs:535-638)"
  verifying_test: "cargo xtask ci (whole) stdout — one artefact row per OPTIONAL step with its probe and outcome; review against xtask/src/main.rs:535-638"
- id: AC-008
  criterion: "GIVEN P4's whole reason for reading evidence is that they do not trust a summary, WHEN they reach the artefact's limits section, THEN it states what this run does not cover — the MSRV (CI's 'minimum supported Rust version' job; the local gate never checks it), wasm32 execution (CI's 'conformance on wasm32' job, attributed to the CI run that was read, because the local gate only type-checks), PS-2's bar, which two testkit instruments do not clear, and the CI-side advisories/semver jobs — and makes no freeze verdict, no unstable-projection exposure verdict, and no ratio over the mutant set in any form."
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/phase-6-projection-proof.md's limits section, attributed to .github/workflows/ci.yml:204-238 and :241-278"
  verifying_test: "xtask/src/proof.rs #[cfg(test)] — the artefact matches no 'N of M' / 'N/M' ratio over mutants and its limits section names both CI jobs; ADR-0010 at .kb/decisions/0010-the-suite-must-prove-itself.md is the prohibition"
- id: AC-009
  criterion: "GIVEN the closeout and HS-P0015 must find this evidence a month from now without being told where it is, WHEN a reader opens references/evaluation/README.md, THEN the new document appears as a row in 'Later additions, which are neither' carrying its date, its pinned commit and its supersede-rather-than-edit lifecycle — so a later correction lands as a second document naming this one, never as an in-place edit — and every file:line citation inside the document resolves at the pinned commit."
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md — the 'Later additions, which are neither' section (:40-55), the enumeration that makes the document reachable"
  verifying_test: "review of the registration row in references/evaluation/README.md plus resolution of every file:line citation in references/evaluation/phase-6-projection-proof.md at the pinned sha"
- id: AC-010
  criterion: "GIVEN a maintainer reading CHANGELOG.md wants to know what the gate enforces that it did not before, WHEN they read the [Unreleased] section, THEN one entry names the new obligation — that the projection proof artefact's test names are now held by the gate, and that harness parity is enforced by a test — without restating rule names (that is CF-29's obligation and belongs to the stories that add rules) and without implying a published version, since nothing is published."
  satisfied: false
  evidence: ""
  mount_point: "CHANGELOG.md — the [Unreleased] section (:24-30)"
  verifying_test: "diff review of CHANGELOG.md's [Unreleased] section against CF-29's scope at spec/SPECIFICATION.md:8141-8153"
```
