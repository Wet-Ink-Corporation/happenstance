---
item: HS-S0041
stage: implement
created: 2026-08-12T13:46:39.662Z
updated: 2026-08-12T13:46:39.662Z
---

# Acceptance ledger — The concurrency family green, and the 8-versus-64 discrepancy closed

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
  criterion: "GIVEN an adapter author who has just taken `event_store_conformance!` green against a real SQLite file and now needs the one answer a sequential suite cannot give — whether the store holds its consistency boundary when a *second* caller is fitted between the probe and the insert (`crates/happenstance-testkit/src/concurrency.rs:17-22`) — WHEN they run the concurrency target, THEN all five rules named by `for_each_concurrency_rule!` appear in the run and every one reports `Ran`: none absent from the binary, none `Skipped`, and none aborted by the panic a declined `SECOND_HANDLE` raises (`crates/happenstance-testkit/src/contract.rs:135-161`). A green sequential suite beside a skipped concurrency family is not partial credit."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/concurrency.rs — the new test target mounting happenstance_testkit::event_store_concurrency_conformance!(SqliteFixture::new()) under #![cfg(not(target_arch = \"wasm32\"))]"
  verifying_test: "cargo test -p happenstance-sqlite --test concurrency — the generated dcb_concurrency_conformance module, all five rules of for_each_concurrency_rule! reporting Ran in the run's own per-rule output"

- id: AC-002
  criterion: "GIVEN every contender is a bare OS thread under `std::thread::scope` driving its own future with the testkit's park-loop `block_on`, outside any ambient reactor (`crates/happenstance-testkit/src/concurrency.rs:45-68`), and today's skeleton answers a missing runtime with `SqliteEventStoreError::NoRuntime` by design (`crates/happenstance-sqlite/src/event_store.rs:26-32`, `:176-178`), WHEN a contender calls `append` and when the partial-batch rule's reader calls `read`, THEN neither depends on `Handle::try_current()` succeeding *on that thread*: the store carries the runtime handle ADR-0022 recorded, captured where one exists at `SqliteEventStore::open` / `::new` (`crates/happenstance-sqlite/src/event_store.rs:108-132`), and no `Attempt::Failed` and no `\"a concurrent read failed\"` sighting anywhere in the run names a missing runtime. If ADR-0022 recorded option (b) instead, `NoRuntime` and its documentation are gone in this same change rather than left describing a state that cannot occur."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/concurrency.rs — reached through the runtime seam in crates/happenstance-sqlite/src/event_store.rs (SqliteEventStore, ReadCursor, SqliteReadStream)"
  verifying_test: "crates/happenstance-sqlite/tests/concurrency.rs::store_serves_a_bare_thread_with_no_ambient_runtime — store constructed inside #[tokio::test(flavor = \"multi_thread\")], append and a fully drained read driven from a std::thread::scope thread via happenstance_testkit::block_on; negative control (same test with the captured handle removed) run once and cited"

- id: AC-003
  criterion: "GIVEN `CONTENDERS` + 1 `rusqlite::Connection`s onto one file and a write path that opens `BEGIN IMMEDIATE`, where SQLite returns `SQLITE_BUSY` *immediately* unless a busy handler is configured, WHEN contenders collide, THEN every non-winner is `Attempt::Rejected` because the store returned `AppendError::ConditionViolated`, and never `Attempt::Failed` (`crates/happenstance-testkit/src/concurrency.rs:224-233`) — the finite, generous busy timeout ADR-0022 states is applied by **every** connection `SqliteFixture::connect()` opens rather than only the first, and no infinite busy handler is configured anywhere. An adapter author who cannot tell a lost race from a contended driver has been handed a verdict about the wrong thing."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/concurrency.rs — against SqliteFixture::connect() in crates/happenstance-sqlite/tests/support/mod.rs"
  verifying_test: "crates/happenstance-sqlite/tests/concurrency.rs::every_connection_carries_the_declared_busy_timeout — reads PRAGMA busy_timeout back from the second and third connect() of one fixture; corroborated by the five conformance rules running with no Failed variant in the output"

- id: AC-004
  criterion: "GIVEN the named wrong implementation is a store that probes *outside* its write lock — `BEGIN DEFERRED`, then insert — which passes the sequential rule forever and is exactly what `append-atomicity-and-store-limits` was written not to be, WHEN `CONTENDERS` callers race for one consistency boundary and, separately, for k disjoint ones, THEN exactly one commits, k disjoint boundaries admit exactly k commits, every committed position is distinct, and each winner is returned its own last position — all compared against positions the store actually assigned, never against literals, because `AUTOINCREMENT` permits gaps (`CLAUDE.md`, *The rule that matters*)."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/concurrency.rs — generated dcb_concurrency_conformance module"
  verifying_test: "dcb_concurrency_conformance::exactly_one_of_n_contenders_commits, ::k_disjoint_boundaries_admit_exactly_k_commits, ::positions_are_unique_under_concurrent_appends, ::append_returns_the_callers_own_last_position in crates/happenstance-sqlite/tests/concurrency.rs; falsifiability from crates/happenstance-testkit/tests/mutation_coverage.rs::the_concurrency_rules_reject_exactly_what_they_claim (:3407) re-run green at the settled CONTENDERS"

- id: AC-005
  criterion: "GIVEN an application author replaying a log while writers commit — four writers each appending four rounds of three-event batches while a fifth handle reads (`crates/happenstance-testkit/src/concurrency.rs:742-763`) — and given that a failed read in that rule is reported as a **sighting**, `\"a concurrent read failed: {err}\"` (`:948-953`), so a driver error there arrives dressed as a verdict about atomicity, WHEN the reader drains `read` repeatedly during the race, THEN it never observes some-but-not-all events of any batch, and every sighting in the run is a genuine observation rather than a store error. This is the rule where a half-done runtime seam fails *silently*."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/concurrency.rs — generated dcb_concurrency_conformance module, reaching SqliteReadStream through the runtime seam"
  verifying_test: "dcb_concurrency_conformance::a_concurrent_reader_never_sees_a_partial_batch in crates/happenstance-sqlite/tests/concurrency.rs, read together with store_serves_a_bare_thread_with_no_ambient_runtime — a green verdict here with a NoRuntime among the sightings is a failure of AC-002 and must be recorded as one"

- id: AC-006
  criterion: "GIVEN both stated proof artefacts read *64 contenders* (`RUNBOOK.md:159`, `RUNBOOK.md:4217-4218`) while `concurrency::CONTENDERS` is **8** (`crates/happenstance-testkit/src/concurrency.rs:206`), so an evaluator reading the runbook and an adapter author reading the code are told different things, WHEN this story merges, THEN exactly one of two outcomes is true and the third — leaving it — is a failure of this criterion, not a deferral: (A) `CONTENDERS` raised, its doc comment rewritten to state the reason and the workspace-wide cost, and the whole workspace green at the new size; or (B) `RUNBOOK.md:159` and `RUNBOOK.md:4217-4222` both amended to the number the code implements, the *\"across 25 rounds\"* phrase reconciled against the rule-local `ROUNDS = 4` (`crates/happenstance-testkit/src/concurrency.rs:745`), and the verification that rejected 64 recorded rather than asserted."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/concurrency.rs:195-206 (branch A, the constant and its doc comment) or RUNBOOK.md:159 and RUNBOOK.md:4217-4222 (branch B, both proof artefacts) — the branch taken is named in the evidence"
  verifying_test: "cargo test --workspace --all-features and cargo xtask affected --base main green at the settled number, necessarily re-running crates/happenstance-testkit/tests/memory_concurrency_conformance.rs and crates/happenstance-testkit/tests/mutation_coverage.rs::the_concurrency_rules_reject_exactly_what_they_claim at that size; plus the reviewed doc comment or the two RUNBOOK amendments"

- id: AC-007
  criterion: "GIVEN the seam adds a field to `SqliteEventStore`, `ReadCursor` and `SqliteReadStream`, and `tests/shapes.rs` exists precisely to catch a field added carelessly, and given that this story relocates `SqliteFixture` to a shared module so two targets can use it, WHEN the affected gate runs, THEN nothing the previous story earned is spent: `SqliteReadStream: Send + Unpin`, `SqliteEventStore: Send + Sync` and the error's `Send + Sync + 'static` bound all still hold; the sequential suite stays green with the fixture at `tests/support/mod.rs` and no third test binary minted; `MID_BATCH_FAULT` stays declined with its real reason; and the gate's four `wasm32` steps are untouched, because the family does not exist on that target."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/shapes.rs and crates/happenstance-sqlite/tests/conformance.rs, with SqliteFixture relocated to crates/happenstance-sqlite/tests/support/mod.rs"
  verifying_test: "cargo test -p happenstance-sqlite --test shapes (read_stream_is_send_and_unpin, store_is_send_and_sync, error_is_send_sync_and_static, send_flavour_stream_is_send_in_generic_code), cargo test -p happenstance-sqlite --test conformance, cargo xtask affected --base main, cargo xtask wasm"
```
