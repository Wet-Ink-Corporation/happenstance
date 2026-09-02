---
item: HS-S0078
stage: implement
created: 2026-08-12
updated: 2026-08-12
---

# Acceptance ledger — `LadybugFixture` and the projection conformance run, registered as a proof artefact

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
    GIVEN the reviewer holding the projection freeze, who has been told `ProjectionStore` will be
    re-tested against a batch shape unlike the ones that froze it, WHEN they run `cargo xtask ci` on
    this branch and read the `tests` step top to bottom without opening a second tool, THEN a Ladybug
    conformance target has executed the **whole** `projection_store_conformance!` suite over a
    `LadybugFixture` — every registered rule accounted for by a pass or a reported skip, none absent —
    and the number of rules that reported is reconciled, in the run record, against the number
    `projection-store-freeze` registered, derived from the suite's own enumeration and never from this
    story's expectations.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — the ARTEFACTS row (:133-150) for happenstance-ladybug's conformance target, which mounts crates/happenstance-ladybug/tests/<conformance target>.rs into the gate; the target itself runs in `cargo xtask ci`'s tests step (xtask/src/main.rs:143-151)"
  verifying_test: "crates/happenstance-ladybug/tests/<conformance target>.rs — the projection_store_conformance!(LadybugFixture::new()) invocation, run by `cargo xtask ci`; rule count reconciled against the projection analogue of crates/happenstance-testkit/src/registry.rs:93-103 and recorded in this story's run record"
- id: AC-002
  criterion: >-
    GIVEN an adapter author who will copy this fixture the next time a store adapter needs one, and
    who knows the tempting shortcut is one LadybugDB directory shared by every instance because a
    per-instance directory costs a filesystem create *and* a C++ engine open, WHEN the suite constructs
    two `LadybugFixture` instances in one process and writes through each, THEN neither observes
    anything the other wrote — because each instance owns a fresh temp directory and its own
    `Database`, and each `connect()` is one handle onto *that* store — and the rule that proves it is
    the suite's **inherited** isolation rule, not a local assertion this story wrote to congratulate
    itself.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-ladybug/tests/ — the shared LadybugFixture module included by both test targets, mounted into the gate through the xtask/src/proof.rs ARTEFACTS row"
  verifying_test: "the projection analogue of two_fixture_instances_observe_none_of_each_others_appends (crates/happenstance-testkit/src/suite.rs:210) passing inside crates/happenstance-ladybug/tests/<conformance target>.rs under `cargo xtask ci`, against the fixture contract at crates/happenstance-testkit/src/contract.rs:14-24"
- id: AC-003
  criterion: >-
    GIVEN the same reviewer, now asking the only question a green run cannot answer — *what could this
    store not do, and why* — WHEN they read the gate's captured output for the conformance target,
    THEN every rule the fixture cannot satisfy has still **run** and has printed
    `SKIP <rule>: <reason>` carrying the fixture's own fixture-specific reason (LadybugDB's
    many-readers-one-writer rule where that is the truth), no rule anywhere is `#[cfg]`-ed out of the
    binary, and no declension carries an empty reason.
  satisfied: false
  evidence: ""
  mount_point: "the `tests` step of `cargo xtask ci` (xtask/src/main.rs:131-151), whose --show-output flag is what makes a passing capability-gated rule's SKIP line reachable; the capability constants are declared on LadybugFixture in crates/happenstance-ladybug/tests/"
  verifying_test: "crates/happenstance-ladybug/tests/<conformance target>.rs run under `cargo xtask ci` with --show-output, with every emitted SKIP line pasted verbatim into this story's run record; the empty-reason trap is caught at codegen by `cargo build`/`cargo test` per crates/happenstance-testkit/src/contract.rs:404-410"
- id: AC-004
  criterion: >-
    GIVEN a gate reader who has already been burned once by a step that a *deletion* fails and an
    *emptying* passes (`xtask/src/proof.rs:9-24`), WHEN the conformance target is truncated to its
    attributes, has its rules renamed, or is replaced by a file that emits nothing, THEN
    `cargo xtask ci` fails **before** the run with a message naming which expected tests are missing —
    never exiting 0 on `running 0 tests` — because `ARTEFACTS` carries a row for this crate's target
    whose `tests` list is non-empty and fully qualified with the emitted module prefix, copied out of
    `cargo test --test <target> -- --list` rather than guessed.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — the ARTEFACTS constant at :133-150 gains a fourth Artefact row naming package `happenstance-ladybug`, the conformance target, and its fully qualified test names (the qualification requirement is proof.rs:58-70)"
  verifying_test: "`cargo xtask proof-artefact` — its own step in the gate (xtask/src/main.rs:170-178), asserting the row's names out of --list before running them (xtask/src/proof.rs:195-217); plus a hand negative control (target emptied, gate re-run, failure output recorded, target restored)"
- id: AC-005
  criterion: >-
    GIVEN the reviewer reading the verdict's ergonomics section, who was promised that dropping the
    `Batch` lifetime would relieve callers of higher-ranked bounds (PS-5), WHEN they open
    `crates/happenstance-ladybug/tests/port_shape.rs` after this merges, THEN the file has been
    **extended** — `weak_flavour` and `send_flavour` still in separate modules, generic code still
    instantiated through the `const _` block at the batch shape that actually merged — and the fate of
    `for<'a> S::Batch<'a>: Send` is written down as an observation (collapsed to `S::Batch: Send`, or
    survived), with no bound weakened and no instantiation deleted except the ones that go with the
    store HS-S0077 retired.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-ladybug/tests/port_shape.rs — extended in place (M3), keeping the weak_flavour/send_flavour module split at :11-14 that CLAUDE.md constraint 4 requires; run by the tests step of `cargo xtask ci`"
  verifying_test: "`cargo test -p happenstance-ladybug --test port_shape` (compile-only const _ instantiations, per standards/rust/61-compile-time-assertions.md) inside `cargo xtask ci`, plus diff review that the :50-61 bound's fate is recorded in this story's run record rather than silently edited"
- id: AC-006
  criterion: >-
    GIVEN the reviewer who knows the one way this project can fail while looking successful — a
    "graph" fixture that is a row store with a Cypher accent, passing every rule and re-testing the
    freeze against the shape that froze it — WHEN they read the run record, THEN a `RowShapedFixture`
    (one node label, scalar properties, no relationships, no traversal) has been run against the
    **same** suite and its outcome is **recorded, not asserted green**: a full pass is written down as
    the finding *"the registered rules do not distinguish a graph batch from a row batch"* and routed
    to `projection-store-freeze`; a rejection names the rule that bit on the unlike axis. The control
    is committed and its outcome written **before** the real run is interpreted.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-ladybug/tests/ — the RowShapedFixture and the target that runs the same projection_store_conformance! macro over it (it cannot live in the testkit, per _decomposition.md:86-88); run by `cargo xtask ci`"
  verifying_test: "the negative-control test target under crates/happenstance-ladybug/tests/ run by `cargo xtask ci`, with its outcome written as a named finding in this story's run record per standards/rust/60-what-a-test-must-prove.md and discover.md, *The wrong implementation*"
- id: AC-007
  criterion: >-
    GIVEN a stranger who did not write this adapter and is asked to believe *"passed against N
    adapters"*, WHEN they open this story's folder, THEN they find the exact command, the emitted rule
    list, every declension with its stated reason, the negative control's outcome, the `port_shape.rs`
    observation and the commit SHA — enough to re-take the snapshot themselves and enough for
    `freeze-verdict-document` to name *which implementation, which rules, which clauses, at which
    commit* without re-running anything.
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/ladybug-projection-store/ladybug-fixture-and-conformance-run/ — the run record freeze-verdict-document (HS-S0082) consumes; deliberately not references/evaluation/, which _decomposition.md:210-218 reserves for the two commit-ordered documents"
  verifying_test: "a reviewer re-running the recorded command at the recorded commit and obtaining the recorded rule list and declensions; the record's completeness checked against project.md:151-153 (DR-4) and :172-174 (DR-9)"
- id: AC-008
  criterion: >-
    GIVEN the maintainer of the suite this story is the first outside consumer of, who has to trust
    that a green run here was not bought by editing the thing being tested, WHEN they read the whole
    diff, THEN nothing under `crates/happenstance-testkit/**` or in
    `crates/happenstance-core/src/projection.rs` was touched, no rule was `#[cfg]`-ed out, retried
    until green, or made to pass by weakening the fixture; any rule that looked wrong is raised with
    `projection-store-freeze` **with its reason in this same change**; no `[FROZEN]` clause marker or
    sentence in `spec/SPECIFICATION.md` moved and `cargo xtask spec-trace` is green;
    `happenstance-ladybug` gains **no new `pub` item**; and the one new dev-dependency is pinned in
    `[workspace.dependencies]`, named `…workspace = true` by the crate, and clears `cargo deny`
    without a `deny.toml` exemption.
  satisfied: false
  evidence: ""
  mount_point: "the PR boundary block in spec.md (crates/happenstance-ladybug/tests/**, crates/happenstance-ladybug/Cargo.toml, Cargo.toml, xtask/src/proof.rs and this story's folder), enforced by `redkiln verify --grain story`; the dependency pin lands in Cargo.toml's [workspace.dependencies] (:17-19)"
  verifying_test: "`cargo xtask ci` — spec-trace, cargo deny, the cargo hack feature-powerset and docs steps all green; plus `git diff --stat` showing no path under crates/happenstance-testkit/, crates/happenstance-core/, spec/, .kb/ or references/, and `rg -n \"#\\[cfg\" crates/happenstance-ladybug/tests/` returning no rule-level gate"
```
