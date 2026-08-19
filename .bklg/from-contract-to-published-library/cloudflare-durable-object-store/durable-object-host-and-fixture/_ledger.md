---
item: "HS-S0053"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — A Durable Object host and the CloudflareFixture mounted on it

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
  criterion: "**The adapter author has something to hang the store off, and it is reachable from where a suite lives.** GIVEN a `happenstance-cloudflare` whose read and write paths are complete but which no Durable Object hosts, WHEN the adapter author compiles the crate's test targets, THEN a Durable Object host class and the entrypoint the wasm32 runner addresses exist, are declared in a module list the real build compiles (`crates/happenstance-cloudflare/src/lib.rs`), and are **nameable from a second compilation unit** — not a bare `#[cfg(test)]` module in `src/lib.rs`, which an integration test cannot reach"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs (module list + re-export block at :128-135); if the host lives in a tests/ support module, the test target that declares it"
  verifying_test: "crates/happenstance-cloudflare/tests/fixture_contract.rs::the_host_is_reachable_from_an_integration_test"

- id: AC-002
  criterion: "**The fixture the author hands to the macro is bound on the weaker flavour, and stays `!Send`.** GIVEN the only `!Send` adapter in the workspace, WHEN `CloudflareFixture` implements `Fixture` with `type Store = CloudflareEventStore`, THEN the bound is the bare `EventStore` (never `SendEventStore`), only one of the two trait names is in scope per module, `Store` is an owned associated type rather than a GAT, every handle the fixture holds is `Rc`-shaped, and the four standing `!Send` probes including `the_probe_is_not_vacuous` still pass"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs"
  verifying_test: "crates/happenstance-cloudflare/src/lib.rs::not_send_probe (all four, :180-259) plus crates/happenstance-cloudflare/tests/fixture_contract.rs::the_fixture_store_is_not_send"

- id: AC-003
  criterion: "**Two fixture instances share nothing, so the suite's isolation rule can bite.** GIVEN the one adapter mistake no upstream test can observe — every fresh fixture instance quietly pointed at the same Durable Object storage (`_decomposition.md:668-681`) — WHEN two `CloudflareFixture` instances are alive at the same time and each appends, THEN neither instance's store reads any event the other appended, because a fresh instance is a fresh object id / namespace entry and **not** a shared object cleared in between"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs"
  verifying_test: "crates/happenstance-cloudflare/tests/fixture_contract.rs::two_instances_alive_at_once_observe_none_of_each_others_appends"

- id: AC-004
  criterion: "**`connect()` gives a second handle onto one object, and the schema seam runs once per object.** GIVEN CF-16 `[FROZEN]` (`spec/SPECIFICATION.md:7548-7568`), whose rule uses `must!` and fails rather than skips on a declined value, WHEN the adapter author calls `connect()` twice on one fixture instance, THEN `SECOND_HANDLE` is `Capability::SUPPORTED`, each handle observes the other's appends, `migrate()` has run exactly once for that instance, and no second store-id incarnation was minted for the second handle (ADR-0014)"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs (fixture); crates/happenstance-cloudflare/src/event_store.rs:70-87 (the one construction seam)"
  verifying_test: "crates/happenstance-cloudflare/tests/fixture_contract.rs::two_handles_from_one_instance_observe_each_others_appends, ::migrate_runs_once_per_instance_not_per_connect, ::a_second_handle_does_not_re_mint_the_store_id"

- id: AC-005
  criterion: "**The gate reader is told what this runtime cannot do, and why it cannot.** GIVEN that a `#[cfg]`-ed-out rule is indistinguishable from a passing one in the emitted binary (`crates/happenstance-testkit/src/contract.rs:26-42`, CF-18 `[FROZEN]` at `spec/SPECIFICATION.md:7597`), WHEN any `Capability` on this fixture is declined, THEN its reason is non-empty and names *why this runtime cannot* rather than *that it cannot*, no rule is `#[cfg]`-ed out of this crate, and the reason survives into a line a human reads — `RuleOutcome::skip_line` into `console_log!` under `__emit_wasm`, because `report`'s `println!` is a measured no-op on `wasm32-unknown-unknown`"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs"
  verifying_test: "crates/happenstance-cloudflare/tests/fixture_contract.rs::every_declined_capability_names_this_runtime"

- id: AC-006
  criterion: "**Nothing this fixture claims is claimed without the method behind it.** GIVEN that the trait ties no `SUPPORTED` constant to its override and the provided bodies **panic** (`contract.rs:290-307`, `:330-352`), and that the reachable author path is *declare supported, forget the override, run the suite*, WHEN `REOPEN` or `MID_BATCH_FAULT` is declared `SUPPORTED`, THEN `reopen()` / `arm_mid_batch_fault()` is overridden with a real seam; and WHEN either is declined, THEN it is **restated explicitly in this impl** with this runtime's own words rather than inherited from the trait default, so no sentence written for a different store appears in this run's log"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs"
  verifying_test: "crates/happenstance-cloudflare/tests/fixture_contract.rs::a_supported_capability_has_its_method_overridden, ::mid_batch_fault_is_restated_not_inherited"

- id: AC-007
  criterion: "**The shipped macro can consume this fixture unchanged, with its limits stated deliberately and its hand-offs named.** GIVEN that a limit is a fact rather than a trade and a guessed number fails `append_reports_exceeded_store_limits` in one direction or the other (`contract.rs:214-279`, CF-40 `[PROVISIONAL]` at `spec/SPECIFICATION.md:7661`), WHEN the adapter author writes the five-line `event_store_conformance!` invocation, THEN `fixture = CloudflareFixture::new()` type-checks against the general arm unchanged, `MAX_EVENT_DATA_LEN` / `MAX_TAGS_PER_EVENT` / `MAX_EVENTS_PER_BATCH` are each **written out** in this impl — never inherited by omission — with `measured-store-limits` named as the owner of the values, and the concurrency family's non-invocation is documented with its reason rather than left as an absence"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs (fixture + crate docs); consumed at crates/happenstance-testkit/src/lib.rs:312-357"
  verifying_test: "crates/happenstance-cloudflare/tests/fixture_contract.rs::the_fixture_expression_satisfies_the_macro_arm, ::the_three_store_limits_are_stated_here"
```
