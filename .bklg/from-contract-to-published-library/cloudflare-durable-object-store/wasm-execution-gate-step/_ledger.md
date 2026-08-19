---
item: "HS-S0048"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — A wasm32 conformance run inside cargo xtask ci, not beside it

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
  criterion: "GIVEN a gate reader on a clean checkout who has been told this workspace's !Send port design is real, WHEN they run one `cargo xtask ci` (and one `cargo xtask ci --fast`) and read the output top to bottom, THEN a named wasm32 step reports conformance rules that actually executed on wasm32-unknown-unknown under wasm-bindgen-test-runner — a per-rule pass or a named failure, not a compile line — and the gate is green on every runner the gate job matrices over"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — `const REQUIRED: &[Step]` at :105, iterated by run_ci (:828-833) and run_fast (:853-860)"
  verifying_test: "`cargo xtask ci` and `cargo xtask ci --fast` end to end over the new REQUIRED row, executing crates/happenstance-testkit/tests/memory_conformance_wasm.rs; the CI `gate` job on ubuntu/windows/macos (.github/workflows/ci.yml:34-39)"
- id: AC-002
  criterion: "GIVEN an adapter author who will later insert a step above this one, WHEN they run `cargo xtask wasm` or read `cargo xtask --help`, THEN the execution step is selected by name and not by index — it is a REQUIRED row whose `name` string appears in wasm_steps(), an unresolvable name panics in steps_named, `cargo xtask wasm` runs it too, and print_help's wasm description no longer claims the wasm32 tasks merely check that things build"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — the new step's name added to wasm_steps() at :784-791, resolved through steps_named at :816-826; print_help at :735-737"
  verifying_test: "new `#[cfg(test)]` unit tests in xtask/src/main.rs asserting wasm_steps() resolves five steps and contains the execution step's exact name, run by `cargo test -p xtask`; plus `cargo xtask wasm`"
- id: AC-003
  criterion: "GIVEN a gate reader who has been burned by a step that a deletion fails and an emptying passes (xtask/src/proof.rs:9-23), WHEN the executed target is truncated to its #![cfg(…)] attribute, or a rule test is renamed or #[ignore]d, THEN the gate fails with a message naming the missing test rather than exiting 0 on `running 0 tests` — because the executed wasm32 target is registered in the xtask/src/proof.rs shape and its named rules are asserted out of --list before the run, or, where the runner cannot enumerate, an equivalent mandatory assertion is made over the run's own reported count against the rule enumeration"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — a fourth registered entry in the ARTEFACTS shape (:133-149), reached from REQUIRED through the `each phase's proof artefacts` step (xtask/src/main.rs:176-191)"
  verifying_test: "`cargo xtask proof-artefact` (xtask/src/main.rs:681) covering the new entry; a `#[cfg(test)]` test in xtask/src/proof.rs asserting the wasm entry's expectation list is non-empty and each name appears in crates/happenstance-testkit/src/registry.rs:94-140; the emptied-target negative control recorded in the implementation report"
- id: AC-004
  criterion: "GIVEN an adapter author on a machine without wasm-bindgen-cli installed, WHEN they run `cargo xtask ci --fast`, THEN what happens is a stated decision they can read in the file, and in no configuration does the wasm32 execution silently not run while the gate still prints green: either the step is `probe: None` and the gate fails outright, or it is probe-gated AND a mandatory non-skippable assertion over the target and its rule enumeration runs regardless — with the choice, its cost and its rejected alternative written into the step's own comment as ADR-0023 material"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — the new step's `probe` field in REQUIRED (:105), against the skip path at :873-878 and the rule stated at :192-202"
  verifying_test: "a unit test in xtask/src/main.rs asserting the execution step's probe is None (or that a `probe: None` compensator row exists in REQUIRED), run by `cargo test -p xtask`; a manual `cargo xtask ci --fast` with the runner removed from PATH, output recorded in the implementation report"
- id: AC-005
  criterion: "GIVEN a gate reader asking what could this runtime not do, and why, WHEN they read the wasm32 step's output in the same terminal scroll as the rest of the gate, THEN every rule the run touched is attributable to the one for_each_event_store_rule! enumeration with no wasm-only subset list anywhere in the tree, and any declined capability's `SKIP <rule>: <reason>` line is visible rather than swallowed — the step passes the runner's --nocapture equivalent, because println! is a silent discard on this target"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — the new step's `args` (the --nocapture equivalent) and `env` in REQUIRED at :105; the observed enumeration is crates/happenstance-testkit/src/registry.rs:94-140 via crates/happenstance-testkit/tests/memory_conformance_wasm.rs:23-27"
  verifying_test: "the executed step's captured output in `cargo xtask ci`, pasted into the implementation report; `registry::no_orphan_rules` green under `cargo test --workspace --all-features` (crates/happenstance-testkit/src/registry.rs:413-424)"
- id: AC-006
  criterion: "GIVEN the adapter author of every-rule-under-workerd (HS-S0054), WHEN they come to run the Cloudflare conformance target under the same runner, THEN they add it by registering a row in a declared list of executed targets — the way ARTEFACTS takes a row — and write no second execution step, no second runner wiring and no second Step.env; an implementation that hard-codes `-p happenstance-testkit --test memory_conformance_wasm` into Step.args is the named wrong implementation this criterion rejects"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — the declared executed-target list in the ARTEFACTS shape (:133-149) and its shared argument builder cargo_args (:164-174), reached from REQUIRED (xtask/src/main.rs:105)"
  verifying_test: "a unit test in xtask walking the declared executed-target list and asserting it is a list of rows each carrying its own package and target rather than fixed arguments, run by `cargo test -p xtask`; code review against the named wrong implementation"
- id: AC-007
  criterion: "GIVEN a gate reader looking at CI after this merges, WHEN they compare .github/workflows/ci.yml against the in-gate step, THEN there are not two unexplained wasm32 execution paths: the standalone wasm-conformance job is either retired in favour of the in-gate step — with the wasm-bindgen-cli-version-resolved-from-Cargo.lock behaviour preserved wherever the work now lives, never hard-coded — or kept with its continuing purpose stated in its own comment, its stale `Those arrive at phase 9` sentence corrected either way; and the recorded verdict says whether the runner is available and deterministic on all three matrix runners, any platform restriction carrying its reason, with an escalated blocking finding rather than a downgrade of AC-004 to a `cargo check` if it is not"
  satisfied: false
  evidence: ""
  mount_point: ".github/workflows/ci.yml — the wasm-conformance job at :189-237 and the gate matrix at :34-39, against the new REQUIRED row in xtask/src/main.rs:105"
  verifying_test: "review of the .github/workflows/ci.yml diff against ../_decomposition.md DEPLOY-AC-05 and Deployment Notes §2; the three-runner `gate` job green (or the documented restriction with its reason in the file); `cargo xtask spec-trace` green; the verdict recorded in the implementation report"
```
