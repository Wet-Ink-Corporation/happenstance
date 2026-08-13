---
item: "HS-S0024"
stage: implement
created: "2026-08-12T13:46:21.090Z"
updated: "2026-08-12T13:46:21.090Z"
---

# Acceptance ledger — FaultyStore<S> and GappyMemoryStore in the testkit

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
  criterion: "**GIVEN** an application author who has just run `cargo add --dev happenstance-testkit` and wants to test a retry loop without adopting a database, **WHEN** they write `use happenstance_testkit::{FaultyStore, SendFaultyStore, GappyMemoryStore, FaultyStoreError};` and construct `SendFaultyStore::new(MemoryEventStore::new()).violate_next(1)`, **THEN** all four names resolve at the **crate root** with no private module in the path, no `#[doc(hidden)]`, and no second crate to add — the instruments are opened-on-demand as *one* act (adding the testkit), never two"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/lib.rs"
  verifying_test: "crates/happenstance-testkit/tests/faulty_store_instruments.rs::instruments_are_reachable_from_the_crate_root (plus the root-path-only doctests in crates/happenstance-testkit/src/faulty.rs and src/gappy.rs)"

- id: AC-002
  criterion: "**GIVEN** an author whose command loop must survive a store that refuses an append without naming the conflict — the case `happenstance-neon` produces routinely and no in-process store produces at all, **WHEN** they arm `violate_next(1)` and append, **THEN** the call returns `AppendError::ConditionViolated` whose `conflicting_position` is **`None`**, and the inner store is left exactly as it was — the same events, the same head, the same assigned positions — because the inner `append` was never called"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/lib.rs"
  verifying_test: "crates/happenstance-testkit/tests/faulty_store_instruments.rs::injected_violation_reports_no_conflicting_position and ::injected_violation_does_not_touch_the_inner_store"

- id: AC-003
  criterion: "**GIVEN** an author who wrote the retry loop everyone writes first — retry only when the store tells me *which* event conflicted, **WHEN** that loop runs against `FaultyStore::new(..).violate_next(1)`, **THEN** it fails **in their own test suite, in under a second, with no database**, while the sibling loop that branches on `is_condition_violated()` succeeds on its second attempt — and the counter, having reached zero, delegates from then on, so the fixture is reversible rather than permanently poisoned"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/lib.rs"
  verifying_test: "crates/happenstance-testkit/tests/faulty_store_instruments.rs::retry_gated_on_some_conflicting_position_never_retries and ::retry_gated_on_is_condition_violated_succeeds_on_the_second_attempt"

- id: AC-004
  criterion: "**GIVEN** an author testing the read path of a handler that must not fall over when the store dies mid-stream, **WHEN** they arm `fail_next_read(1)` and call `read`, **THEN** nothing fails at call time — `read` is still not `async` and still hands back the stream at the top level — the failure arrives as the **first polled item** (`Err(FaultyStoreError::Injected)`), and on the `Send` flavour that stream is still `Send`, provably: it survives being held across an `await` inside a real `tokio::spawn`"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/lib.rs"
  verifying_test: "crates/happenstance-testkit/tests/faulty_store_instruments.rs::injected_read_failure_is_the_first_polled_item_not_a_call_time_error and crates/happenstance-testkit/tests/faulty_store_send_guard.rs::spawns_from_generic_over_send_faulty_store"

- id: AC-005
  criterion: "**GIVEN** an adapter author who will copy whatever this crate does, and an application author who must be able to trust that a *disarmed* fixture is not quietly lying about ordering or positions, **WHEN** `SendFaultyStore` is put through the full conformance suite with nothing armed, **THEN** it passes every rule — ordering, `ReadOptions` in both directions, `head()`, atomicity, identity — because the wrapper delegates rather than reimplements, and its own fixture declines any capability it cannot offer **with a stated reason** rather than vanishing from the binary"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/lib.rs"
  verifying_test: "crates/happenstance-testkit/tests/faulty_store_conformance.rs — happenstance_testkit::event_store_conformance!(FaultyFixture::new()) over an unarmed SendFaultyStore<MemoryHandle>"

- id: AC-006
  criterion: "**GIVEN** an author who read the design's item list and expects `GappyMemoryStore` to exist in a default `cargo add happenstance-testkit`, **WHEN** they build with default features and again with `--all-features`, **THEN** the type is present in both — because the `memory` feature the `#[cfg]` names is **declared** and is **in `default`** — and on docs.rs it renders with a `doc_cfg` gate badge naming that feature rather than appearing ungated or not at all"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/lib.rs"
  verifying_test: "crates/happenstance-testkit/tests/gappy_store_instruments.rs (carries #![cfg(feature = \"memory\")], collected under both default and --all-features runs of cargo test -p happenstance-testkit); gate badge proved by the nightly --cfg docsrs rustdoc step in xtask/src/main.rs"

- id: AC-007
  criterion: "**GIVEN** an author writing their first read-model handler, who computed the next position as `previous + 1` because that is what every store they have ever used did, **WHEN** that handler runs against `GappyMemoryStore::with_stride(..)`, **THEN** its computed position **disagrees with the position the store actually assigned** and the test says so as a value comparison — while the sibling handler that resumes with `ReadOptions::from(checkpoint.next())` and answers *am I caught up?* by comparing against `head()` processes every event exactly once"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/lib.rs"
  verifying_test: "crates/happenstance-testkit/tests/gappy_store_instruments.rs::handler_assuming_position_plus_one_disagrees_with_the_store and ::handler_resuming_past_an_inclusive_checkpoint_sees_every_event_once"

- id: AC-008
  criterion: "**GIVEN** an adapter author who will read this crate to learn what a legitimate store may do, **WHEN** `GappyMemoryStore` is put through the conformance suite, **THEN** it **passes** — proving its gaps are the freedom VT-11 grants and not a defect — ES-9's interior-gap branch runs against it, and no assertion anywhere in this story names a literal position value: every position claim is a *relation* (strictly increasing; some consecutive pair separated by more than one) measured against what the store returned"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/lib.rs"
  verifying_test: "crates/happenstance-testkit/tests/gappy_memory_conformance.rs — event_store_conformance!(GappyFixture::new()), exercising crates/happenstance-testkit/src/suite.rs:1533-1553; plus crates/happenstance-testkit/tests/gappy_store_instruments.rs::assigned_positions_are_strictly_increasing_with_at_least_one_gap"

- id: AC-009
  criterion: "**GIVEN** an author whose test must distinguish *the fixture broke this on purpose* from *my store failed*, **WHEN** either failure reaches them, **THEN** they match on `FaultyStoreError::Injected` versus `FaultyStoreError::Store(e)` — a typed, `#[non_exhaustive]`, `core::error::Error` value whose `Store` arm keeps the inner error reachable through `source()` rather than occluding it behind a rendered string — and the wrapper compiles over a store whose `Error` is **uninhabited** (`pub enum MemoryStoreError {}`) with no `unwrap`, `expect` or `unreachable!` anywhere in the path"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/lib.rs"
  verifying_test: "crates/happenstance-testkit/tests/faulty_store_instruments.rs::injected_and_store_errors_are_distinguishable_and_the_source_chain_survives"

- id: AC-010
  criterion: "**GIVEN** a reader who meets these four items on the testkit's rendered crate-root page and has not read this spec, **WHEN** they scan the item table and open one item, **THEN** each row shows a **complete first-sentence claim of ≤ 80 characters that is not truncated with an ellipsis**, each identifier is ≤ 24 characters, each item page carries a runnable doctest driven by `happenstance_testkit::block_on` (no runtime dependency added), an `# Errors` section on every fallible function naming conditions rather than types, and — on each store — the **named wrong implementation it exists to reject**, so the instrument teaches what it is for without this document"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/lib.rs"
  verifying_test: "cargo test --doc -p happenstance-testkit over crates/happenstance-testkit/src/faulty.rs and src/gappy.rs, plus cargo doc under -D warnings; reviewed against _design.md:845-857 (density) and _design.md:983-987 (anti-patterns 5 and 6)"
```
