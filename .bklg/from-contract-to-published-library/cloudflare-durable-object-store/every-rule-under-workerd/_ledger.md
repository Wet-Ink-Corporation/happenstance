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

> **Criterion row AC-001 was amended on 2026-08-20 — through the spec path, not here.** The
> ledger's own rule above is that a criterion is never re-worded by an implementer, and it is
> not being: `spec.md`'s **Amendment ADR-0023-A** is the named decision that moved it (with this
> story's Merge DoD, `project.md`'s AC-002 / AC-004 / DoD 1, `measured-store-limits`'
> AC-001–AC-003 and Merge DoD, and `initiative.md`'s DoD 4), on the condition the human gate of
> 2026-08-19 set and `kb-decision-0023` has since met. The row below is re-copied from the
> amended spec so the two cannot disagree; what is still unsettled is
> `kb-open-question-workerd-runner-absent-001`.


```yaml
- id: AC-001
  criterion: >-
    GIVEN a gate reader on a clean checkout who has been told this workspace paid for a two-flavour
    `!Send` port design so an edge store could exist, WHEN they run one `cargo xtask ci` and read the
    output top to bottom without opening a second tool, a second CI job or a machine they do not
    have, THEN a named wasm32 step reports every rule of the `for_each_event_store_rule!` enumeration
    having **executed** against `CloudflareFixture` on `wasm32-unknown-unknown` under
    `wasm-bindgen-test-runner` against a real `SqlStorage` mapping, with the platform runtime an
    open question — a per-rule
    pass or a named failure, never a compile line — in the same terminal scroll as the rest of the
    gate, and the artefacts that run lands on name what it does not prove: no isolate, no eviction,
    no hibernation, no I/O gate and none of the platform's own storage ceilings. (Amended
    2026-08-20 by Amendment ADR-0023-A in spec.md — kb-decision-0023,
    kb-open-question-workerd-runner-absent-001.)
  satisfied: true
  evidence: "**All 89 event-store rules executed and passed** against a real `SqlStorage` mapping on wasm32-unknown-unknown under wasm-bindgen-test-runner — a node:sqlite-backed DurableObjectState shim, NOT workerd — in the gate's own step: `cargo run -p xtask -- wasm-conformance` prints `happenstance-cloudflare/durable_object_conformance: 89 rules enumerated, 10 named, executing on wasm32-unknown-unknown` and then `test result: ok. 89 passed; 0 failed; 0 ignored`. Per-rule pass lines, not compile lines, in the same terminal scroll as the rest of the gate. The target is crates/happenstance-cloudflare/tests/durable_object_conformance.rs; the row is xtask/src/proof.rs WASM_TARGETS[3]; the driving step is `wasm32 run of the conformance rules` in xtask/src/main.rs's REQUIRED, unchanged."
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
  satisfied: true
  evidence: "crates/happenstance-cloudflare/tests/durable_object_conformance.rs:60-64 is the three-line invocation, `event_store_conformance!(mod_name = dcb_conformance_wasm, emit = happenstance_testkit::__emit_wasm, fixture = CloudflareFixture::new())` — `memory_conformance_wasm.rs` with one expression changed. `rg -n \"macro_rules!\" crates/happenstance-cloudflare/` returns nothing; no `#[cfg]` sits over any individual rule; `for_each_event_store_rule!` is still the only enumeration and `registry::no_orphan_rules` is green. This is mechanical rather than a review promise: `xtask`'s guard derives all 89 names from the enumeration and asserts them out of the target's own `--list` before the run, and `xtask/src/proof.rs::tests::the_executed_wasm_targets_name_no_rule_of_their_own` refuses a target that writes a rule list in its code."
  verifying_test: "crates/happenstance-testkit/src/registry.rs `no_orphan_rules` under `cargo test --workspace --all-features`, plus `rg -n \"macro_rules!\" crates/happenstance-cloudflare/` returning nothing and diff review against crates/happenstance-testkit/tests/memory_conformance_wasm.rs:19-27"
- id: AC-003
  criterion: >-
    GIVEN the gate maintainer who built the wasm32 execution seam in `wasm-execution-gate-step` and
    committed in its AC-006 that the next target arrives as a row, WHEN this story mounts the
    Cloudflare conformance target, THEN the whole `xtask` delta is one row in the declared
    executed-target registry in `xtask/src/proof.rs` naming `happenstance-cloudflare` and
    `durable_object_conformance` — no second `Step` in `const REQUIRED`, no second runner-env wiring,
    no second `--target` plumbing — and `cargo xtask wasm` picks it up by name with no further edit.
  satisfied: true
  evidence: "The whole `xtask` conformance delta is one `WasmTarget` row (xtask/src/proof.rs, WASM_TARGETS[3]) naming `happenstance-cloudflare`, `durable_object_conformance`, `dcb_conformance_wasm` and `EVENT_STORE_FAMILY`. `xtask/src/proof.rs::tests::the_cloudflare_conformance_target_is_a_row_and_not_a_second_step` asserts each of those AND counts the steps in `REQUIRED` whose name contains `wasm32 run of`, requiring exactly 1 — so a second `Step` fails the test rather than passing every other check in the file. RED beat: the same test failed with `the Cloudflare conformance target has no row in WASM_TARGETS` before the row existed. `xtask/src/main.rs` is unchanged; `cargo xtask wasm-conformance` picks the row up by name with no further edit. One repair to the seam WAS needed and is reported as a finding — see the implementation report, `unregistered_wasm_harnesses`'s row count."
  verifying_test: "new #[cfg(test)] unit tests in xtask (xtask/src/proof.rs and xtask/src/main.rs) run by `cargo test -p xtask`, plus `cargo xtask wasm` executing the target"
- id: AC-004
  criterion: >-
    GIVEN a gate reader burned once by a step that a deletion fails and an emptying passes
    (`xtask/src/proof.rs:9-23`), WHEN the conformance target is truncated to its attributes, wrapped
    in `#[cfg(not(target_arch = "wasm32"))]`, has a rule renamed or `#[ignore]`d, or is fed by a
    hand-written emitter that silently drops three rules, THEN the gate fails before the run, with a
    message naming exactly which expected rules are missing — never exiting 0 on `running 0 tests`
    and never printing green over a suite three rules short.
  satisfied: true
  evidence: "**Both negative controls were performed and both failed before the run.** (A) Target emptied to its attributes: `cargo xtask wasm-conformance-enumeration` → `crates/happenstance-cloudflare/tests/durable_object_conformance.rs no longer invokes the conformance suite; an emptied target exits 0 on `running 0 tests` (looked for `event_store_conformance!`)`, and `cargo xtask wasm-conformance` → `is missing 89 of the 89 rules `for_each_event_store_rule!` declares … Listed: 0 name(s).` (B) Target wrapped in `#![cfg(not(target_arch = \"wasm32\"))]`: same 89-of-89 failure, naming the missing rules. Both were reverted and the guard is green again (`89 rules enumerated, 10 named`). The expectation is DERIVED, not hand-copied: `enumerated_rules(wasm.family)` parses `for_each_event_store_rule!` itself and prefixes with the row's `module`. The transcribed `CLOUDFLARE_WASM_RULES` list is the second, rename-catching half, and `xtask/src/proof.rs::tests::every_named_wasm_rule_is_one_the_enumeration_declares` asserts it is non-empty and wholly contained in the enumeration."
  verifying_test: "`cargo xtask proof-artefact` covering the new row; a #[cfg(test)] unit test in xtask/src/proof.rs asserting the expectation list is non-empty and matches the enumeration (`cargo test -p xtask`); the two negative controls performed by hand and their failure output recorded in the implementation report"
- id: AC-005
  criterion: >-
    GIVEN a gate reader asking *what could this runtime not do, and why* (project DoD 2), WHEN they
    read the wasm32 step's output in the same scroll, THEN every rule guarding a capability
    `CloudflareFixture` declines has still run, and its
    "SKIP <rule>: fixture declines `<CAPABILITY>` — <reason>" line is visible in the gate's captured
    output — not swallowed by libtest capture, and not silently discarded by a `println!` on a target
    that has no stdout.
  satisfied: true
  evidence: "The fixture declines two things and the run prints exactly three SKIP lines for them, verbatim from `cargo run -p xtask -- wasm-conformance`: `SKIP arming_a_mid_batch_fault_makes_the_append_fail: fixture declines `MID_BATCH_FAULT` — this Durable Object host can throw on a chosen statement, so the mechanism exists; what has not been settled against an executed conformance run is which statement of this adapter's write path is the k-th row's, and CF-39 requires a fixture claiming the capability to name the mechanism rather than to hope`; the same for `append_is_atomic_under_a_mid_batch_fault`; and `SKIP append_reports_exceeded_store_limits: fixture declines `MAX_EVENT_DATA_LEN, MAX_TAGS_PER_EVENT, MAX_EVENTS_PER_BATCH` — this fixture states no ceiling for any store limit …`. Every one of those rules RAN and reported; none was `#[cfg]`-ed out. The three `REOPEN` rules print no skip at all, because this fixture is the workspace's first to answer that capability SUPPORTED — a fact only visible because the skip channel works. The path is `RuleOutcome::skip_line` → `__emit_wasm`'s `console_log!`, surfaced by the step's `--nocapture`."
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
  satisfied: true
  evidence: "crates/happenstance-cloudflare/src/lib.rs:28-73 — a section headed *Conformance: what has run, and what is deliberately not asked to*, landing where a consumer of the rendered docs lands. It states the concurrency family's non-invocation WITH its reason (`event_store_concurrency_conformance!` binds `F::Store: EventStore + Send`, its module is `#[cfg(not(target_arch = \"wasm32\"))]`, and a `!Send` adapter on a threadless target cannot invoke it and is not expected to) and says what that does and does not cost — a Durable Object is a single-threaded actor with exclusive storage ownership, so there is no second writer for a race, and the event-store family still runs the single-threaded shapes of the same question. The model family's status is in the same section: it is behind the off-by-default `proptest` feature, whose module carries a target condition on top BECAUSE a Cargo feature is not target-scoped, so `proptest` is not in the graph on wasm32 at all. `cargo doc -p happenstance-cloudflare --no-deps` is clean; the same prose is restated at the fixture, crates/happenstance-cloudflare/tests/support/mod.rs:36-59."
  verifying_test: "`cargo xtask ci` docs step and the --no-default-features doc build; review of the paragraph against crates/happenstance-testkit/src/lib.rs:101-110, :168-173, :176-183 and standards/rust/70-rustdoc-obligations.md"
- id: AC-007
  criterion: >-
    GIVEN a library consumer who will later `cargo add happenstance-cloudflare` on the *Event-source
    at the edge* journey, WHEN they resolve the crate after this merges, THEN nothing this story
    added reaches them: `wasm-bindgen-test` lives only in
    `[target.'cfg(target_arch = "wasm32")'.dev-dependencies]` of this crate (because the attribute
    resolves in the caller's scope), `[dependencies]` is untouched, no Cargo feature was invented to
    carry a test harness, and the runtime graph `cargo deny` and `cargo hack` see is unchanged.
  satisfied: true
  evidence: "`cargo tree -p happenstance-cloudflare -e normal --depth 1` after this slice: `futures-core`, `happenstance-core`, `thiserror`, `worker` — identical to before. `[dependencies]` was not touched, no Cargo feature was invented to carry a harness, and there is no `build.rs`. `wasm-bindgen-test` remains in `[target.'cfg(target_arch = \"wasm32\")'.dev-dependencies]`, mirroring crates/happenstance-testkit/Cargo.toml, because the runtime attribute resolves in the caller's scope. DEVIATION, deliberate and reported: `happenstance-testkit`, `happenstance-core` and `futures-core` are HOST dev-dependencies rather than target-scoped ones (crates/happenstance-cloudflare/Cargo.toml `[dev-dependencies]`), because an integration target is a second compilation unit that does not inherit the library's `[dependencies]`, and because the fixture's declaration-level criteria must be reachable by an ordinary `cargo test` — see the implementation report. All three are existing workspace members or existing workspace dependencies, so `cargo deny`'s licence/advisory surface gains nothing; MSRV is unaffected, since a dev-dependency is invisible to CI's `--no-dev-deps` job and covered by its full `cargo test` at the 1.97.1 floor."
  verifying_test: "the `cargo hack` feature-powerset, `cargo deny` and `package-check` steps of `cargo xtask ci`, plus `cargo tree -p happenstance-cloudflare -e normal` identical before and after"
- id: AC-008
  criterion: >-
    GIVEN the implementers of `measured-store-limits` (HS-S0055) and `adr-0023-and-atom-resolutions`
    (HS-S0058), who read this story's report as their input, WHEN they open it, THEN the run is
    scoped honestly: the ceilings it ran at are named as HS-S0053's declarations and explicitly not
    CF-40's discharge; any rule that failed only on `wasm32` is recorded as a named finding with its
    divergence rather than `#[cfg]`-ed away; every standing detector is still green; and nothing
    `[FROZEN]` was amended and nothing was written under `.kb/`.
  satisfied: true
  evidence: "The implementation report beside this ledger scopes the run honestly: it names the three ceilings as HS-S0053's DECLARATIONS (all `None`) and states in terms that the green run is **not** CF-40's discharge, with `measured-store-limits` named as where those numbers become facts. **No rule failed on wasm32**, so there is no divergence finding to record — 89 of 89 passed on the first run. Standing detectors green: `cargo test -p happenstance-cloudflare` runs the four `!Send` probes including `the_probe_is_not_vacuous` and `send_shape::send_flavour::SendStoreWithLocalError` still compiles; `cargo test -p happenstance-core` keeps both `read`-shape tests; `cargo run -p xtask -- spec-trace` reports `traceability: no problems found` over 201 clauses and 401 citations; `cargo run -p xtask -- lint-constitution` reports 27 atoms consistent. `git status` shows no `.kb/**` path and no `spec/SPECIFICATION.md` path in this slice. Two `standards/rust/` atoms carry repaired `file:line` citations, reported as a finding rather than absorbed."
  verifying_test: "`cargo xtask spec-trace`; `cargo test -p happenstance-cloudflare` (the four !Send probes incl. the_probe_is_not_vacuous, and send_shape::send_flavour::SendStoreWithLocalError still compiling); `cargo test -p happenstance-core` (both read-shape tests in crates/happenstance-core/src/memory.rs); `git diff --stat` showing no .kb/** and no spec/SPECIFICATION.md path"
```
