---
item: HS-S0008
stage: implement
created: 2026-08-12T13:46:02.497Z
updated: 2026-08-12T13:46:02.497Z
---

# Acceptance ledger — A declined capability is a reported skip, never a silent absence

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
  criterion: "GIVEN P2, an adapter author whose projection store genuinely cannot provide a guarantee the port assumes, WHEN they open `ProjectionFixture` looking for the constant that lets them say so, THEN they find exactly the capability constants `_design.md` recorded under DT-3 — each carrying a reason written by the author DT-3 named, fixture prose through `Capability::declined` or a testkit-written const beside `NO_CEILING_REASON` — and no constant that record does not contain; if the record is silent on the set, the story halts and reports rather than minting one here."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/contract.rs — the `ProjectionFixture` capability-constant block, re-exported at crates/happenstance-testkit/src/lib.rs:187 and through `__private` at lib.rs:362-364"
  verifying_test: "crates/happenstance-testkit/tests/mutation_coverage.rs::projection_capability_reasons_are_authored_once (plus the reviewed diff against .bklg/from-contract-to-published-library/projection-store-freeze/_design.md)"
- id: AC-002
  criterion: "GIVEN an adapter author who declares every declinable projection capability unsupported — the cheapest way to turn a red build green — WHEN they run the projection suite, THEN the run still emits one test per registered projection rule, the number of observed outcomes equals the number of registered rules, and every registered rule name is present; so `#[cfg]`-ing a gated rule out of the expansion, which is the wrong implementation CF-18 names by hand, fails the suite's own meta-test instead of producing a green build with no record of the trade."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/contract.rs — the `ProjectionFixture` capability-constant block, driving `for_each_projection_store_rule!` through the harnesses under crates/happenstance-testkit/tests/"
  verifying_test: "crates/happenstance-testkit/tests/mutation_coverage.rs::projection_capability_skips_are_reported (the projection sibling of capability_skips_are_reported at :3184-3236)"
- id: AC-003
  criterion: "GIVEN whoever reads that run's CI log and has never seen this suite before, WHEN a projection rule is skipped, THEN the run prints exactly one line for it in the shape already in the tree — SKIP {rule}: fixture declines `{capability}` — {reason} — rule name first, then the constant they can actually change, then the fixture's own words, with no second format, no projection-local skip type and no extra summary block; and the test compares the reason against the fixture's own `const` rather than a literal repeated in the test, since two copies would let the report carry someone else's words."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/contract.rs — `RuleOutcome::skip_line` / `report` (:458-537) reached from the `ProjectionFixture` capability block; emitted by crates/happenstance-testkit/src/registry.rs:222-295"
  verifying_test: "crates/happenstance-testkit/tests/mutation_coverage.rs::projection_capability_skips_are_reported (value + rendered-line assertions), with the human half observed in the tokio/blocking projection harnesses under crates/happenstance-testkit/tests/"
- id: AC-004
  criterion: "GIVEN the initiative's promise that a guarantee which does not apply is reported rather than absent, WHEN the projection suite's own meta-test checks that promise, THEN it reads `RuleOutcome` / `Verdict` values and never captured stdout — because libtest exposes nothing programmatically and suppresses a passing test's output without `--show-output` — and its mirror, run against a projection fixture that supports everything, reports an empty `skipped()`, so a gate wired to the wrong constant fails in both directions rather than only the convenient one."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/contract.rs — `RuleOutcome` (:458-537, `#[must_use]` at :462-472) asserted through crates/happenstance-testkit/tests/mutation_coverage/harness.rs::Verdict"
  verifying_test: "crates/happenstance-testkit/tests/mutation_coverage.rs::projection_capability_skips_are_reported, including the fully-capable mirror (the projection sibling of :3307-3316)"
- id: AC-005
  criterion: "GIVEN an adapter author who reads the skip machinery as licence to opt out of a MUST, WHEN they decline a projection capability the recorded set marks as one, THEN that rule panics carrying their own stated reason rather than skipping, it never appears in `skipped()`, and the projection `MUST_REJECT` / `MUST_SKIP` slices are checked in both directions — so a rule that gains a gate without a listing, a listed rule that stops skipping, and a `must!` quietly downgraded to `require!` each fail by name instead of rotting into decoration as later stories add rules."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/suite.rs — `must!` (:49-84) and `require!` (:37-46) used unchanged from the projection rules module that `projection-suite-entry-point` lands under crates/happenstance-testkit/src/"
  verifying_test: "crates/happenstance-testkit/tests/mutation_coverage.rs::projection_capability_skips_are_reported — its projection MUST_REJECT / MUST_SKIP slices (siblings of :3271-3305)"
- id: AC-006
  criterion: "GIVEN a future contributor who deletes or bypasses a rule's `require!` gate — or a reviewer asked to believe a meta-test that has nothing to fire on — WHEN the suite runs, THEN `DecliningProjectionFixture::connect` panics on the second call so an ungated rule surfaces as `Verdict::Panicked` rather than a quiet pass; and where no gated projection rule exists at this story's merge point, the meta-test's own doc comment says in those words that it is a guard on future registrations rather than a demonstrated skip, naming `read-through-and-rebuild-rules` as the in-project forcing function."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/mutation_coverage/variants.rs — `DecliningProjectionFixture` beside `DecliningFixture` (:502-565), driven through the projection enumeration"
  verifying_test: "crates/happenstance-testkit/tests/mutation_coverage.rs::projection_capability_skips_are_reported (no rule reaches Verdict::Panicked) plus the reviewed doc comment on that test"
- id: AC-007
  criterion: "GIVEN P3, the local-first / edge developer running the conformance suite on `wasm32-unknown-unknown`, WHEN a projection rule skips on that target, THEN the reason still reaches them — routed as `skip_line`'s `String` into `console_log!` the way `__emit_wasm` already routes the event-store family — rather than through `RuleOutcome::report`, which is a measured no-op there, so AC-016's whole-suite wasm run is not the one where the stated reason silently disappears."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs:276-291 — `__emit_wasm`'s `console_log!` route, reached by the projection wasm harness under crates/happenstance-testkit/tests/"
  verifying_test: "xtask/src/main.rs:231 — the mandatory `wasm32 check of the conformance harnesses` step (`cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown`), run by `cargo xtask ci`"
- id: AC-008
  criterion: "GIVEN a fixture author who declines a capability and forgets the reason — `Capability::declined(\"\")` — WHEN they build, THEN the `const fn` `assert!` fails at codegen, caught by `cargo build` and `cargo test` and missed by `cargo check` and `cargo clippy`; `ProjectionFixture`'s own rustdoc says exactly that, so nobody trusts a green `check`; any doctest demonstrating it is spelled bare `compile_fail`, never `compile_fail,E0080`, because rustdoc on 1.97.1 silently ignores an error code it cannot match; and `CHANGELOG.md` gains an entry naming the defect this machinery detects — a capability-gated projection rule vanishing from the binary."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/contract.rs — `ProjectionFixture`'s rustdoc beside `Capability::declined` (:400-419); plus CHANGELOG.md"
  verifying_test: "cargo test -p happenstance-testkit --doc — the bare `compile_fail` doctest on `ProjectionFixture` in crates/happenstance-testkit/src/contract.rs"
```
