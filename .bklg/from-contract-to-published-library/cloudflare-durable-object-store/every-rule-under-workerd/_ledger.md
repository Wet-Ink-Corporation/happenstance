---
item: HS-S0054
stage: implement
created: 2026-08-12
updated: 2026-08-12
---

# Acceptance ledger — Every event-store conformance rule executed on the target, in the same gate run

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
    GIVEN a gate reader on a clean checkout who has been told this workspace paid for a two-flavour
    `!Send` port design so an edge store could exist, WHEN they run one `cargo xtask ci` and read the
    output top to bottom without opening a second tool, a second CI job or a machine they do not
    have, THEN a named wasm32 step reports every rule of the `for_each_event_store_rule!` enumeration
    having **executed** against `CloudflareFixture` inside a real Durable Object runtime — a per-rule
    pass or a named failure, never a compile line — in the same terminal scroll as the rest of the
    gate.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — the declared executed-target registry row for happenstance-cloudflare/durable_object_conformance, driven by the named wasm32 execution Step in xtask/src/main.rs's const REQUIRED (:105) inside run_ci (:828) and run_fast (:853)"
  verifying_test: "crates/happenstance-cloudflare/tests/durable_object_conformance.rs executed by `cargo xtask ci` (and `cargo xtask ci --fast`, `cargo xtask wasm`)"
- id: AC-002
  criterion: >-
    GIVEN an adapter author on the *Learn when you are finished* journey who suspects the edge suite
    is quietly a subset — because a bespoke emitter is the one subset nothing in this tree can grep
    for, WHEN they read the entire diff and search the workspace for a second rule list, THEN the
    harness is the shipped `happenstance_testkit::event_store_conformance!` with
    `emit = happenstance_testkit::__emit_wasm` and `fixture = CloudflareFixture::new()` verbatim in
    three lines, `crates/happenstance-cloudflare/` defines no `macro_rules!` taking a rule list, no
    `#[cfg]` sits over any individual rule, and `for_each_event_store_rule!` is still the only
    enumeration in the tree.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/tests/durable_object_conformance.rs — the three-line shipped-macro invocation, registered as a row in xtask/src/proof.rs's executed-target registry"
  verifying_test: "crates/happenstance-testkit/src/registry.rs `no_orphan_rules` under `cargo test --workspace --all-features`, plus `rg -n \"macro_rules!\" crates/happenstance-cloudflare/` returning nothing and diff review against crates/happenstance-testkit/tests/memory_conformance_wasm.rs:19-27"
- id: AC-003
  criterion: >-
    GIVEN the gate maintainer who built the wasm32 execution seam in `wasm-execution-gate-step` and
    committed in its AC-006 that the next target arrives as a row, WHEN this story mounts the
    Cloudflare conformance target, THEN the whole `xtask` delta is one row in the declared
    executed-target registry in `xtask/src/proof.rs` naming `happenstance-cloudflare` and
    `durable_object_conformance` — no second `Step` in `const REQUIRED`, no second runner-env wiring,
    no second `--target` plumbing — and `cargo xtask wasm` picks it up by name with no further edit.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — the declared executed-target registry (the ARTEFACTS-shaped list at :133-149), selected by name through wasm_steps() (xtask/src/main.rs:784-791) and steps_named (:816-826)"
  verifying_test: "new #[cfg(test)] unit tests in xtask (xtask/src/proof.rs and xtask/src/main.rs) run by `cargo test -p xtask`, plus `cargo xtask wasm` executing the target"
- id: AC-004
  criterion: >-
    GIVEN a gate reader burned once by a step that a deletion fails and an emptying passes
    (`xtask/src/proof.rs:9-23`), WHEN the conformance target is truncated to its attributes, wrapped
    in `#[cfg(not(target_arch = "wasm32"))]`, has a rule renamed or `#[ignore]`d, or is fed by a
    hand-written emitter that silently drops three rules, THEN the gate fails before the run, with a
    message naming exactly which expected rules are missing — never exiting 0 on `running 0 tests`
    and never printing green over a suite three rules short.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — the new registry row's expectation list, derived from happenstance_testkit::__emit_rule_names! (crates/happenstance-testkit/src/registry.rs:290-295) and mod_name-prefixed, asserted out of `cargo test -- --list` before the run (proof.rs:183-240, :292-325)"
  verifying_test: "`cargo xtask proof-artefact` covering the new row; a #[cfg(test)] unit test in xtask/src/proof.rs asserting the expectation list is non-empty and matches the enumeration (`cargo test -p xtask`); the two negative controls performed by hand and their failure output recorded in the implementation report"
- id: AC-005
  criterion: >-
    GIVEN a gate reader asking *what could this runtime not do, and why* (project DoD 2), WHEN they
    read the wasm32 step's output in the same scroll, THEN every rule guarding a capability
    `CloudflareFixture` declines has still run, and its
    "SKIP <rule>: fixture declines `<CAPABILITY>` — <reason>" line is visible in the gate's captured
    output — not swallowed by libtest capture, and not silently discarded by a `println!` on a target
    that has no stdout.
  satisfied: false
  evidence: ""
  mount_point: "the wasm32 execution step's captured output in `cargo xtask ci`, produced by the registry row in xtask/src/proof.rs and routed through RuleOutcome::skip_line (crates/happenstance-testkit/src/contract.rs:500-503) into __emit_wasm's console_log! (crates/happenstance-testkit/src/registry.rs:276-288)"
  verifying_test: "crates/happenstance-cloudflare/tests/durable_object_conformance.rs run under `cargo xtask ci` with the runner's --nocapture equivalent; the emitted SKIP lines pasted verbatim into the implementation report (or, if the fixture declines nothing, a recorded-and-reverted scratch decline demonstrating the path)"
- id: AC-006
  criterion: >-
    GIVEN a constrained-runtime developer reading this crate's rendered documentation and asking
    whether *conformant* means the same thing here as for the SQLite adapter, WHEN they read the
    crate docs, THEN the concurrency family's non-invocation is stated with its reason —
    `event_store_concurrency_conformance!` binds `F::Store: EventStore + Send`, its module is
    `#[cfg(not(target_arch = "wasm32"))]`, and a `!Send` adapter cannot invoke it and is not expected
    to — and the model family's status is stated in the same paragraph, that `proptest` is off by
    default and `fixtures::strategies`/`model` carry the same target condition because a feature is
    not target-scoped, so it is not in the graph on `wasm32` at all: a documented, reasoned
    non-invocation rather than an unexplained absence.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs — the crate-level documentation a consumer lands on, rendered by the docs step of `cargo xtask ci`"
  verifying_test: "`cargo xtask ci` docs step and the --no-default-features doc build; review of the paragraph against crates/happenstance-testkit/src/lib.rs:101-110, :168-173, :176-183 and standards/rust/70-rustdoc-obligations.md"
- id: AC-007
  criterion: >-
    GIVEN a library consumer who will later `cargo add happenstance-cloudflare` on the *Event-source
    at the edge* journey, WHEN they resolve the crate after this merges, THEN nothing this story
    added reaches them: `wasm-bindgen-test` lives only in
    `[target.'cfg(target_arch = "wasm32")'.dev-dependencies]` of this crate (because the attribute
    resolves in the caller's scope), `[dependencies]` is untouched, no Cargo feature was invented to
    carry a test harness, and the runtime graph `cargo deny` and `cargo hack` see is unchanged.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/Cargo.toml — the new [target.'cfg(target_arch = \"wasm32\")'.dev-dependencies] block, mirroring crates/happenstance-testkit/Cargo.toml's own"
  verifying_test: "the `cargo hack` feature-powerset, `cargo deny` and `package-check` steps of `cargo xtask ci`, plus `cargo tree -p happenstance-cloudflare -e normal` identical before and after"
- id: AC-008
  criterion: >-
    GIVEN the implementers of `measured-store-limits` (HS-S0055) and `adr-0023-and-atom-resolutions`
    (HS-S0058), who read this story's report as their input, WHEN they open it, THEN the run is
    scoped honestly: the ceilings it ran at are named as HS-S0053's declarations and explicitly not
    CF-40's discharge; any rule that failed only on `wasm32` is recorded as a named finding with its
    divergence rather than `#[cfg]`-ed away; every standing detector is still green; and nothing
    `[FROZEN]` was amended and nothing was written under `.kb/`.
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/cloudflare-durable-object-store/every-rule-under-workerd/ — the implementation report and captured gate output HS-S0055 and HS-S0058 consume, produced by the registry row's run in xtask/src/proof.rs"
  verifying_test: "`cargo xtask spec-trace`; `cargo test -p happenstance-cloudflare` (the four !Send probes incl. the_probe_is_not_vacuous, and send_shape::send_flavour::SendStoreWithLocalError still compiling); `cargo test -p happenstance-core` (both read-shape tests in crates/happenstance-core/src/memory.rs); `git diff --stat` showing no .kb/** and no spec/SPECIFICATION.md path"
```
