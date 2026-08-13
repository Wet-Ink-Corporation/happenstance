---
item: HS-S0007
stage: implement
created: 2026-08-12T13:46:01.460Z
updated: 2026-08-12T13:46:01.460Z
---

# Acceptance ledger — projection_store_conformance! — one enumeration, one test per rule

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Five notes specific to this story, so no row is flipped on the wrong evidence:

- **The mount is proven by compilation, not by an assertion.** The three harness files under
  `crates/happenstance-testkit/tests/` can name only public items, so each file building at all is
  part of AC-001's and AC-007's evidence (`spec.md`, Acceptance criteria).
- **Two ACs are proven by steps `cargo xtask ci --fast` does not run.** AC-008 needs the mandatory
  wasm32 `--tests` check and AC-010 needs `cargo xtask spec-trace`; both must be run **explicitly**
  and their output cited. A green `--fast` run is not evidence for either (`spec.md`, Tests and CI).
- **AC-004 needs a failability transcript, not just a green test.** Flip it only with the recorded
  deliberate-orphan run: add an unregistered `pub async fn`, observe `no_orphan_projection_rules`
  fail by name, revert. A meta-test that has never been seen to fail is decorative
  (`spec.md`, EC-002/EC-003).
- **Three rows may resolve as a halt.** If EC-011 (an edit to the event-store emitters or an existing
  harness is required), EC-012 (a rule cannot be written through `ProjectionProbe` alone) or EC-013
  (a dependency's shape is wrong) fires, the affected row is **not** flipped and is not worked
  around — it is reported as a finding against the named story and this story stops.
- **A green run is not evidence of discrimination.** AC-005 and AC-006 are satisfied by rules that
  pass against the oracle `MemoryProjectionStore`; that the suite can *fail* a wrong store is
  `projection-mutant-registry` (HS-S0008)'s to prove. Do not cite a green run as evidence for more
  than the row claims (`.kb/decisions/0010-the-suite-must-prove-itself.md`).

```yaml
- id: AC-001
  criterion: |-
    GIVEN an adapter author (P2) who has written a `ProjectionStore` impl and wants to know whether it is correct, WHEN they add `happenstance-testkit` as a dev-dependency and write the single line `happenstance_testkit::projection_store_conformance!(MyProjectionFixture::new());` in a file under their own `tests/`, THEN it expands to one `#[tokio::test]` per projection rule, each named after the rule, with no rule name written by the caller and no second line of setup — and WHEN a rule fails, the failing test is named after the rule, not after the macro. A macro that compiles but is unreachable from the crate root, or whose expansion cannot name `ProjectionFixture` in the caller's crate, does not satisfy this.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/lib.rs (entry macro beside event_store_conformance! at :265-357; __private at :359-364; module list at :165-189) + crates/happenstance-testkit/Cargo.toml [dependencies] happenstance-core features"
  verifying_test: "crates/happenstance-testkit/tests/projection_conformance.rs (the file compiling at all) + `cargo test -p happenstance-testkit --test projection_conformance -- --list` naming exactly commit_advances_the_checkpoint and commit_is_atomic_with_the_read_model"

- id: AC-002
  criterion: |-
    GIVEN the same author, who must declare *how to make an isolated store* rather than hand over one store, WHEN they implement `ProjectionFixture`, THEN the trait asks for an owned `type Store: ProjectionProbe` (never a borrowing GAT, never `SendProjectionStore`) and an `async fn connect` that panics rather than returning `Result` — so "the database is down" can never be reported through the same channel as "the adapter is wrong"; and WHEN a rule calls `open()` twice, THEN it gets two genuinely isolated backing stores, which is what `commit_rejects_a_foreign_batch` will need one slice later. Binding `ProjectionStore` instead of `ProjectionProbe` in the associated type does not satisfy this: PS-11 makes the probe the thing that turns "you must be observable" into a compile error.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/contract.rs (ProjectionFixture beside Fixture at :120-353) + crates/happenstance-testkit/src/lib.rs:359-364 (__private) and the public re-export beside Fixture"
  verifying_test: "crates/happenstance-testkit/src/projection.rs::two_opens_make_two_isolated_stores; plus `cargo check -p happenstance-testkit --all-features` and `cargo clippy … -D warnings` (async_fn_in_trait)"

- id: AC-003
  criterion: |-
    GIVEN a rule author adding the seventeenth projection rule six months from now, WHEN they add it, THEN there is exactly one place to write its name — `for_each_projection_store_rule!` — and every harness on every runtime picks it up with no edit; AND GIVEN a reader of any of the three harness files, WHEN they open one, THEN it contains zero rule names, which is the observable form of CF-22's "exactly one place, per rule family". A second list — in a harness, in the crate doc as the authority, or in a `const RULES: &[&str]` — is the defect this criterion forbids.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/projection.rs (for_each_projection_store_rule! beside its own rules module), re-exported through crates/happenstance-testkit/src/lib.rs:165-189"
  verifying_test: "crates/happenstance-testkit/tests/projection_conformance_wasm.rs and ..._blocking.rs containing one macro invocation and no rule identifier; `rg -n \"commit_advances_the_checkpoint\" crates/happenstance-testkit` hitting only src/projection.rs"

- id: AC-004
  criterion: |-
    GIVEN a rule author who writes a rule and forgets to register it — the exact failure CF-24 exists for — WHEN the test suite runs, THEN a meta-test fails by name, listing the orphaned rule and saying no harness runs it; AND WHEN the enumeration lists a rule the module no longer declares (renamed, moved), THEN the same meta-test fails in the other direction; AND if the source scan ever matches nothing, it reports every registered rule as missing rather than passing silently. A meta-test that cannot fail in a demonstrable way is decorative.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/projection.rs (the meta-test beside the enumeration it consumes), reached through the module list at crates/happenstance-testkit/src/lib.rs:165-189"
  verifying_test: "crates/happenstance-testkit/src/projection.rs::no_orphan_projection_rules (sibling of crates/happenstance-testkit/src/registry.rs:410-436), plus the deliberate-orphan failability transcript recorded in the implementation report"

- id: AC-005
  criterion: |-
    GIVEN an adapter author whose `commit` returns `Ok` without making anything durable — legal under PS-1's "or not at all" arm, and therefore an implementation the clause admits and the suite must still reject — WHEN they run the suite against a fresh store, THEN `commit_advances_the_checkpoint` fails; and WHEN their store is correct, THEN the rule opens a store, `begin`s, writes through the probe, commits at a position, and reads `checkpoint(&id)` back through a fresh handle, asserting it reports the position the store actually assigned — never a literal `1`, `2`, `3`, because the specification permits gaps. The rule's rustdoc names the wrong implementation it rejects.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/projection.rs (pub mod rules), registered in for_each_projection_store_rule! and reached from crates/happenstance-testkit/src/lib.rs:165-189"
  verifying_test: "crates/happenstance-testkit/tests/projection_conformance.rs::commit_advances_the_checkpoint (and its blocking sibling), driving MemoryProjectionStore via MemoryProjectionFixture"

- id: AC-006
  criterion: |-
    GIVEN an adapter that advances the checkpoint and silently drops the read-model write — `CheckpointOnlyStore`, the defect this whole project exists to catch — WHEN the suite runs, THEN `commit_is_atomic_with_the_read_model` reads the probe row and the checkpoint through fresh handles and asserts both present or both absent, never one, failing at that assertion and nowhere else; and GIVEN a reviewer asking whether the rule is decorative, THEN its rustdoc names `CheckpointOnlyStore` and the story that lands it (`projection-mutant-registry`, HS-S0008) — a knowingly-carried one-story debt with a named discharger, not an exemption. Without `probe_write`/`probe_read` the rule cannot see the read model at all and the suite degenerates into a checkpoint test a broken store passes.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/projection.rs (pub mod rules), registered in for_each_projection_store_rule! and reached from crates/happenstance-testkit/src/lib.rs:165-189"
  verifying_test: "crates/happenstance-testkit/tests/projection_conformance.rs::commit_is_atomic_with_the_read_model, observing the read model through ProjectionProbe only"

- id: AC-007
  criterion: |-
    GIVEN an adapter author who has never written a fixture and wants something to copy, WHEN they look in `happenstance_testkit::fixtures` — the module the specification names by path and the crate doc points at — THEN they find `MemoryProjectionFixture` as a published item with rustdoc, not a test helper buried in a `#[cfg(test)]` block, handing back an owned handle by refcount bump (`core::future::ready`, not `async move`, so it does not pretend to do I/O), in the same idiom `MemoryFixture` already uses. A fixture reachable only from inside the testkit's own tests fails this: the reference implementation is the documentation.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/fixtures.rs (beside MemoryFixture at :243-292), public via crates/happenstance-testkit/src/lib.rs:52-53"
  verifying_test: "crates/happenstance-testkit/tests/projection_conformance.rs naming happenstance_testkit::fixtures::MemoryProjectionFixture from outside the crate (compiles only if public); plus `cargo doc -p happenstance-testkit` rendering its doc comment"

- id: AC-008
  criterion: |-
    GIVEN P3, the local-first / edge developer, whose store runs on `wasm32-unknown-unknown` where there is no libtest and no stdout, WHEN they pick a harness, THEN all three exist and all three run the same rule set: `projection_conformance.rs` (tokio, default arm), `projection_conformance_blocking.rs` (`#![cfg(not(target_arch = "wasm32"))]`, `emit = __emit_projection_blocking`, driving `block_on`) and `projection_conformance_wasm.rs` (`#![cfg(target_arch = "wasm32")]`, `emit = __emit_projection_wasm`); AND WHEN a rule is skipped on wasm, THEN its stated reason reaches a human through `console_log!` rather than through `RuleOutcome::report`, which is a measured no-op on that target. Costs no new gate step: the existing mandatory wasm32 conformance-harness check type-checks the new file.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/ (the three harness files) driven by the emitters in crates/happenstance-testkit/src/projection.rs, under the existing gate step at xtask/src/main.rs:231-243"
  verifying_test: "`cargo check --locked -p happenstance-testkit --tests --target wasm32-unknown-unknown` (xtask/src/main.rs:231-243) + `cargo test -p happenstance-testkit --test projection_conformance_blocking` passing the same two rules"

- id: AC-009
  criterion: |-
    GIVEN an adapter author landing on the testkit's front page — the crate doc and the README are the only orientation they get before writing code — WHEN they read "Where the rule set lives" and the "What is checked" table, THEN both describe four rule families rather than three, the projection family's paragraph says which one line to write and which harness to pick, and every gated module name is spelled plainly rather than as an intra-doc link, because a link into a `cfg`-absent module is a hard error under the `--no-default-features` doc build this workspace has already paid for once. A correct macro documented as if it did not exist is the library equivalent of a component that is built and never mounted.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/lib.rs:84-92 and :134-151 (the two family-scoped crate-doc sections) + crates/happenstance-testkit/README.md"
  verifying_test: "`cargo doc -p happenstance-testkit` and the gate's `--no-default-features` doc build, both clean; review of lib.rs:84-92, :112-132, :134-151 and README.md"

- id: AC-010
  criterion: |-
    GIVEN the maintainer who found, at stage 6, that CF-29's changelog lint read `suite.rs` alone and printed "all 55 suite rules have a changelog entry" — a sentence that is true and answers a different question — WHEN a fourth rule file lands, THEN it is in `RULE_FILES` and every new rule has a `CHANGELOG.md` entry naming the defect it detects, carrying at least 120 characters of prose per rule the entry names; AND WHEN `cargo xtask spec-trace` runs, THEN check 6 finds both rules claimed by PS-1 and the regenerated §7.1–§7.2 region is the only hunk in `spec/SPECIFICATION.md` — no clause body, no maturity marker, nothing `[FROZEN]` line-edited. A rules file absent from `RULE_FILES` reproduces the stage-6 defect one family later.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/spec_trace.rs:71-89 (RULE_FILES) + CHANGELOG.md [Unreleased] + the generated spec/SPECIFICATION.md §7.1–§7.2 region"
  verifying_test: "`cargo xtask spec-trace` run explicitly (green) + the CF-29 changelog lint step inside `cargo xtask ci` (xtask/src/lints.rs:504-580) + `git diff spec/SPECIFICATION.md` showing only the generated region"
```
