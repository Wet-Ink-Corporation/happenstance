---
item: "HS-S0104"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The gate is taught about a fourth suite

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
  criterion: >-
    GIVEN an adapter author reading SY-1 to find out what the ingest bar actually is, WHEN they
    follow the clause's `Rule:` field to the named rule, THEN `spec-trace` resolves it against a
    file it genuinely sweeps — because `RULE_FILES` names the sync suite's rule file(s) as well as
    the three under `crates/happenstance-testkit/src/` — and the clause stops rendering as
    *scheduled* for the reason that it could not be found; AND a sync rule claimed by no `SY`
    clause fails `spec-trace` check 6 by name, with the array's own message offering claim, retire,
    or `UNCLAIMED_PENDING_ADR`.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/spec_trace.rs:85-89 — RULE_FILES, the sweep set consumed by check_rule_ownership (:765), retired_rules (:857), all_rules (:1748) and lints::no_position_literals (:628)"
  verifying_test: "cargo xtask spec-trace (Mode::Check) + new #[cfg(test)] mod tests in xtask/src/spec_trace.rs asserting every RULE_FILES entry exists under the workspace root and is parseable by collect_rules"
- id: AC-002
  criterion: >-
    GIVEN an adapter author who reads a green gate as "the sync suite was checked", WHEN the sync
    suite is made to violate each widened check in turn — a rule claimed by no clause, a literal
    position inside a rule, `version.workspace = true` in the sync manifest, a clock construct
    under the sync suite's `src/`, and a rule with no `CHANGELOG.md` entry — THEN each check fails,
    naming the sync crate's file and the reason, so the decorative-suite mutant (land the suite,
    change nothing under `xtask/`, stay green) is dead. Demonstrating each check runs over the sync
    crate is explicitly not sufficient.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lints.rs:231, :289, :525, :628 and xtask/src/spec_trace.rs:765 — the five widened checks, each reached from cargo xtask lints and cargo xtask affected"
  verifying_test: "cargo test -p xtask — five negative controls in the new mod tests in xtask/src/lints.rs and xtask/src/spec_trace.rs, each driving the check's scope-taking inner function against a fixture tree; transcript fallback recorded in implementation-report.md only for a check that resists the refactor"
- id: AC-003
  criterion: >-
    GIVEN an adapter author running the sync conformance suite on a loaded CI runner, WHEN any sync
    conformance rule under `crates/happenstance-sync-testkit/src/` reads a clock, THEN CF-33's lint
    fails naming that file and line — because `TESTKIT_SRC` has become a list of scopes covering
    both testkits' `src/` — AND `tests/` in either crate stays outside the scope and no exclusion
    is added, because widening exclusions is the direction that ends with the check switched off.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lints.rs:33-42 (TESTKIT_SRC, CF-33's scope) consumed by no_clock (:231); reached from cargo xtask lints and, unconditionally, from xtask/src/affected.rs:120"
  verifying_test: "cargo xtask lint-clock reporting a per-scope scanned count for both scopes; unit test in xtask/src/lints.rs asserting each scope resolves to a non-empty .rs set and that the per-scope empty-directory bail! is preserved"
- id: AC-004
  criterion: >-
    GIVEN an adapter author who must know whether a new sync rule was a semver-MINOR change to the
    bar, WHEN `crates/happenstance-sync-testkit/Cargo.toml` inherits its version from the
    workspace, THEN CF-32 fails and the message names which manifest is at fault and repeats why
    that number must move independently of the contract's — a message that interpolates one
    constant while covering two crates reports the wrong file half the time.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lints.rs:44-45 (TESTKIT_MANIFEST, CF-32's scope) consumed by testkit_version (:289); reached from cargo xtask lints and xtask/src/affected.rs:122"
  verifying_test: "cargo xtask lint-testkit-version over both manifests; unit test in xtask/src/lints.rs asserting the failure message for each manifest contains that manifest's own path"
- id: AC-005
  criterion: >-
    GIVEN an adapter author who reaches `CHANGELOG.md` to learn what a new bar rejects, WHEN a sync
    rule lands with no entry naming a defect, THEN CF-29 fails from the day that rule appears — the
    lint resolves its rule set from `RULE_FILES`, so AC-001's edit is the whole mechanism — AND the
    success line states the file count actually swept, so a later narrowing of the sweep set is
    visible in the output rather than silent.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lints.rs:525 — changelog_names_every_rule, which resolves its rule set out of spec_trace::RULE_FILES and reads CHANGELOG.md; reached from cargo xtask lints and xtask/src/affected.rs:121"
  verifying_test: "cargo xtask lint-changelog green at the zero-sync-rule baseline with a success line naming the swept file count; the AC-002 negative control for a sync rule with no entry, in xtask/src/lints.rs mod tests"
- id: AC-006
  criterion: >-
    GIVEN an adapter author sent by a gate failure to a file to fix a rule, WHEN the message says
    where the rule was looked for, THEN it names the file set actually searched rather than the
    hard-coded `SUITE` — check 4's "not found" message and `retired_rules`' `defined_in` fallback
    both currently send the reader to `crates/happenstance-testkit/src/suite.rs` regardless of
    which of the now-four files was swept, which is a wrong answer delivered with confidence.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/spec_trace.rs:690-710 (check 4's message) and :886-920 (retired_rules' defined_in fallback to SUITE)"
  verifying_test: "cargo xtask spec-trace and cargo xtask lint-retired-rules over a rule defined outside suite.rs; unit test in xtask/src/spec_trace.rs asserting the \"looked for in\" text enumerates the swept set"
- id: AC-007
  criterion: >-
    GIVEN the edge Rust developer who needs replication on Cloudflare Workers, where `Send` is
    unavailable, WHEN the gate runs, THEN a fifth `wasm32` step builds `happenstance-sync` for
    `wasm32-unknown-unknown` — so the port, the ingest seam and the runner are proved to compile
    without `Send` rather than asserted to in prose — AND the step sits in `REQUIRED` beside the
    existing four rather than replacing any of them.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:203-283 — the REQUIRED step table's wasm32 block, the gate's composition root; selected by name in wasm_steps() (:784-791)"
  verifying_test: "cargo xtask wasm and cargo xtask ci --fast both executing `cargo check --locked -p happenstance-sync --target wasm32-unknown-unknown`; unit test in xtask/src/main.rs asserting the step is present in REQUIRED"
- id: AC-008
  criterion: >-
    GIVEN the same developer relying on SY-17's [FROZEN] promise that the sync port binds
    `EventStore` and not `SendEventStore`, WHEN the gate runs, THEN a sixth `wasm32` step
    type-checks the sync conformance harness (`--tests`, not `--all-targets`) on that target —
    which is the only thing that performs the compile SY-17's `Rule:` field literally names, a
    suite compiled against a `!Send` fixture peer holding a `!Send` store behind an `Rc` — AND the
    crate build of AC-007 is not accepted as discharging it, because a harness behind
    `cfg(target_arch = "wasm32")` compiles to nothing on a native run.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:219-244 — the REQUIRED step table beside the existing conformance-harness wasm32 step; selected by name in wasm_steps() (:784-791)"
  verifying_test: "cargo xtask wasm executing `cargo check --locked -p happenstance-sync-testkit --tests --target wasm32-unknown-unknown`; cargo xtask spec-trace resolving SY-17's citation (spec/SPECIFICATION.md:6341-6360)"
- id: AC-009
  criterion: >-
    GIVEN an adapter author on a machine missing an optional tool, WHEN they run the gate, THEN
    neither new `wasm32` step can silently skip — both carry `probe: None` and live in `REQUIRED`,
    and both are selected in `wasm_steps()` by name, where a name resolving to nothing panics
    rather than quietly selecting a neighbour — so `cargo xtask wasm` reports six steps, not four;
    AND the `wasm32` feature powerset gains the sync crates and stays in `OPTIONAL` behind the
    `cargo hack` probe, because it widens coverage above a mandatory plain check rather than
    replacing one.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:196-218 (REQUIRED / probe: None), :558-592 (the OPTIONAL wasm32 feature powerset) and :769-791 (wasm_steps() name selector)"
  verifying_test: "cargo xtask wasm listing six steps; unit test in xtask/src/main.rs asserting every name passed to wasm_steps() resolves in REQUIRED and that both new steps carry probe: None; cargo xtask ci --fast running both"
- id: AC-010
  criterion: >-
    GIVEN a maintainer who cannot tell a vacuous pass from a real one by reading an exit code, WHEN
    any widened constant is pointed at a directory the rules do not live in, or at a file that has
    moved, THEN the gate fails rather than printing a success line: every `RULE_FILES` entry exists
    and parses, the aggregate rule set over `RULE_FILES` is non-empty, every lint scope resolves to
    a non-empty `.rs` set, and every manifest path exists. Asserting the array's length is
    explicitly not this criterion — a count passes while pointing at the wrong tree.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/spec_trace.rs:873-882 (the existing aggregate emptiness bail!) and xtask/src/lints.rs:236-238 (the per-scope empty-directory bail!), both reached unconditionally from xtask/src/affected.rs:112-126"
  verifying_test: "cargo test -p xtask — new #[cfg(test)] mod tests in xtask/src/spec_trace.rs and xtask/src/lints.rs, in the module-local style at xtask/src/affected.rs:597 and xtask/src/package.rs:409"
- id: AC-011
  criterion: >-
    GIVEN the evaluator reading `spec/SPECIFICATION.md` as public evidence of what this library
    actually checks, WHEN they read CF-6, CF-29, CF-32 and CF-33 after this change, THEN each
    `Rule:` field describes the mechanism that now exists — both suites, not one — with every MUST
    verbatim, every `Rejects:` line and maturity marker untouched, and §7.1/§7.2 regenerated by
    `cargo xtask spec-trace --write` rather than by hand; AND if any correction would change the
    set of implementations a clause admits, it is a gap: the story stops, records the finding and
    raises a blocker to ADR-0026's author, and writes no smaller edit.
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md — the Rule: fields of CF-6 (:7233-7249), CF-29 (:8141-8168), CF-32 (:8200-8228), CF-33 (:8236-8262), plus the §7.1/§7.2 generated regions held equal to xtask/src/spec_trace.rs:735-744"
  verifying_test: "cargo xtask spec-trace green in Mode::Check after cargo xtask spec-trace --write; `git diff spec/SPECIFICATION.md` reviewed against project DoD 7 showing only Rule: prose and generated regions; the repair-or-gap test's answer recorded in implementation-report.md"
```
