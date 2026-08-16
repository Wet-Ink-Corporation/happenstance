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
  satisfied: true
  evidence: "crates/happenstance-testkit/src/lib.rs:337 (`pub use faulty::{FaultyStore, FaultyStoreError, SendFaultyStore};`) and :338-340 (the `memory`-gated `pub use gappy::GappyMemoryStore;` with its `doc(cfg)` attribute) — no private module in any of the four paths and no `#[doc(hidden)]`. Verified by tests/faulty_store_instruments.rs::instruments_are_reachable_from_the_crate_root (PASS), which imports all four by crate-root path and constructs each, and by the root-path-only doctests on src/faulty.rs:87, :203, :233 and src/gappy.rs:77 (cargo test --doc -p happenstance-testkit: 20 passed, 4 ignored)"
  mount_point: "crates/happenstance-testkit/src/lib.rs"
  verifying_test: "crates/happenstance-testkit/tests/faulty_store_instruments.rs::instruments_are_reachable_from_the_crate_root (plus the root-path-only doctests in crates/happenstance-testkit/src/faulty.rs and src/gappy.rs)"

- id: AC-002
  criterion: "**GIVEN** an author whose command loop must survive a store that refuses an append without naming the conflict — the case `happenstance-neon` produces routinely and no in-process store produces at all, **WHEN** they arm `violate_next(1)` and append, **THEN** the call returns `AppendError::ConditionViolated` whose `conflicting_position` is **`None`**, and the inner store is left exactly as it was — the same events, the same head, the same assigned positions — because the inner `append` was never called"
  satisfied: true
  evidence: "crates/happenstance-testkit/src/faulty.rs:340-343 and :389-392 — `take_violation` returns above the delegation, so the inner `append` is never called, and the value returned is `AppendError::ConditionViolated(ConditionViolated::unspecified())` (:410-412). Verified by tests/faulty_store_instruments.rs::injected_violation_reports_no_conflicting_position (PASS; asserts the variant *and* `conflicting_position.is_none()`) and ::injected_violation_does_not_touch_the_inner_store (PASS; before/after `head()` and full read compared against store-returned values, under a condition no correct store here can violate on its own)"
  mount_point: "crates/happenstance-testkit/src/lib.rs"
  verifying_test: "crates/happenstance-testkit/tests/faulty_store_instruments.rs::injected_violation_reports_no_conflicting_position and ::injected_violation_does_not_touch_the_inner_store"

- id: AC-003
  criterion: "**GIVEN** an author who wrote the retry loop everyone writes first — retry only when the store tells me *which* event conflicted, **WHEN** that loop runs against `FaultyStore::new(..).violate_next(1)`, **THEN** it fails **in their own test suite, in under a second, with no database**, while the sibling loop that branches on `is_condition_violated()` succeeds on its second attempt — and the counter, having reached zero, delegates from then on, so the fixture is reversible rather than permanently poisoned"
  satisfied: true
  evidence: "tests/faulty_store_instruments.rs::retry_gated_on_some_conflicting_position_never_retries (PASS) asserts the loop's *returned error* and an empty log — not a bare `#[should_panic]`; ::retry_gated_on_is_condition_violated_succeeds_on_the_second_attempt (PASS) asserts `submitted == 2`, that the committed position equals the store's own `head()`, and that a third append with the counter at zero delegates. Reversibility comes from the checked decrement at crates/happenstance-testkit/src/faulty.rs:47-53; ::arming_zero_arms_nothing (PASS) carries EC-001"
  mount_point: "crates/happenstance-testkit/src/lib.rs"
  verifying_test: "crates/happenstance-testkit/tests/faulty_store_instruments.rs::retry_gated_on_some_conflicting_position_never_retries and ::retry_gated_on_is_condition_violated_succeeds_on_the_second_attempt"

- id: AC-004
  criterion: "**GIVEN** an author testing the read path of a handler that must not fall over when the store dies mid-stream, **WHEN** they arm `fail_next_read(1)` and call `read`, **THEN** nothing fails at call time — `read` is still not `async` and still hands back the stream at the top level — the failure arrives as the **first polled item** (`Err(FaultyStoreError::Injected)`), and on the `Send` flavour that stream is still `Send`, provably: it survives being held across an `await` inside a real `tokio::spawn`"
  satisfied: true
  evidence: "crates/happenstance-testkit/src/faulty.rs:318-330 and :370-382 — `read` is not `async`, returns the stream at the top level, and the injected failure is produced at the first poll (:140-155). Verified by tests/faulty_store_instruments.rs::injected_read_failure_is_the_first_polled_item_not_a_call_time_error (PASS; first item `Err(FaultyStoreError::Injected)`, the inner store's own events behind it, arming spent) and tests/faulty_store_send_guard.rs::spawns_from_generic_over_send_faulty_store (PASS; bound written at the definition, stream held across an await inside a real `tokio::spawn`)"
  mount_point: "crates/happenstance-testkit/src/lib.rs"
  verifying_test: "crates/happenstance-testkit/tests/faulty_store_instruments.rs::injected_read_failure_is_the_first_polled_item_not_a_call_time_error and crates/happenstance-testkit/tests/faulty_store_send_guard.rs::spawns_from_generic_over_send_faulty_store"

- id: AC-005
  criterion: "**GIVEN** an adapter author who will copy whatever this crate does, and an application author who must be able to trust that a *disarmed* fixture is not quietly lying about ordering or positions, **WHEN** `SendFaultyStore` is put through the full conformance suite with nothing armed, **THEN** it passes every rule — ordering, `ReadOptions` in both directions, `head()`, atomicity, identity — because the wrapper delegates rather than reimplements, and its own fixture declines any capability it cannot offer **with a stated reason** rather than vanishing from the binary"
  satisfied: true
  evidence: "tests/faulty_store_conformance.rs — `happenstance_testkit::event_store_conformance!(FaultyFixture::new())` over an unarmed `SendFaultyStore<MemoryHandle>`: 89 rules, 89 passed, 0 failed. The fixture follows `MemoryFixture` (handle newtype through `MemoryFixture::connect`, `SECOND_HANDLE` supported, `REOPEN` declined with a non-empty stated reason at tests/faulty_store_conformance.rs:38-44, which the suite reports as a skip)"
  mount_point: "crates/happenstance-testkit/src/lib.rs"
  verifying_test: "crates/happenstance-testkit/tests/faulty_store_conformance.rs — happenstance_testkit::event_store_conformance!(FaultyFixture::new()) over an unarmed SendFaultyStore<MemoryHandle>"

- id: AC-006
  criterion: "**GIVEN** an author who read the design's item list and expects `GappyMemoryStore` to exist in a default `cargo add happenstance-testkit`, **WHEN** they build with default features and again with `--all-features`, **THEN** the type is present in both — because the `memory` feature the `#[cfg]` names is **declared** and is **in `default`** — and on docs.rs it renders with a `doc_cfg` gate badge naming that feature rather than appearing ungated or not at all"
  satisfied: true
  evidence: "crates/happenstance-testkit/Cargo.toml:63-64 declares `default = [memory]` and `memory = [happenstance-core/memory]`. tests/gappy_store_instruments.rs:20 carries `#![cfg(all(not(target_arch = wasm32), feature = memory))]` and is collected under both a default and an `--all-features` run of `cargo test -p happenstance-testkit`; `cargo hack --feature-powerset check -p happenstance-testkit` is green over all 6 combinations. The badge is rendered: target/doc/happenstance_testkit/index.html carries `<span class=stab portability title=Available on crate feature memory only>` on the `GappyMemoryStore` row under `RUSTDOCFLAGS=--cfg docsrs -D warnings cargo +nightly doc`"
  mount_point: "crates/happenstance-testkit/src/lib.rs"
  verifying_test: "crates/happenstance-testkit/tests/gappy_store_instruments.rs (carries #![cfg(feature = \"memory\")], collected under both default and --all-features runs of cargo test -p happenstance-testkit); gate badge proved by the nightly --cfg docsrs rustdoc step in xtask/src/main.rs"

- id: AC-007
  criterion: "**GIVEN** an author writing their first read-model handler, who computed the next position as `previous + 1` because that is what every store they have ever used did, **WHEN** that handler runs against `GappyMemoryStore::with_stride(..)`, **THEN** its computed position **disagrees with the position the store actually assigned** and the test says so as a value comparison — while the sibling handler that resumes with `ReadOptions::from(checkpoint.next())` and answers *am I caught up?* by comparing against `head()` processes every event exactly once"
  satisfied: true
  evidence: "crates/happenstance-testkit/src/gappy.rs:110-128 — the allocator takes the previous **position** and adds the stride, never a length. Verified by tests/gappy_store_instruments.rs::handler_assuming_position_plus_one_disagrees_with_the_store (PASS; `assert_ne!(computed, second.get())` with both values captured from the store, plus the assertion that the computed position belongs to no event at all) beside ::handler_resuming_past_an_inclusive_checkpoint_sees_every_event_once (PASS; resumes through `SequencePosition::next` + `ReadOptions::from` and answers *caught up?* by equality against `head()`)"
  mount_point: "crates/happenstance-testkit/src/lib.rs"
  verifying_test: "crates/happenstance-testkit/tests/gappy_store_instruments.rs::handler_assuming_position_plus_one_disagrees_with_the_store and ::handler_resuming_past_an_inclusive_checkpoint_sees_every_event_once"

- id: AC-008
  criterion: "**GIVEN** an adapter author who will read this crate to learn what a legitimate store may do, **WHEN** `GappyMemoryStore` is put through the conformance suite, **THEN** it **passes** — proving its gaps are the freedom VT-11 grants and not a defect — ES-9's interior-gap branch runs against it, and no assertion anywhere in this story names a literal position value: every position claim is a *relation* (strictly increasing; some consecutive pair separated by more than one) measured against what the store returned"
  satisfied: true
  evidence: "tests/gappy_memory_conformance.rs — `event_store_conformance!(GappyFixture::new())` over an `Arc`-shared `GappyHandle`: 89 rules, 89 passed, 0 failed, which includes `read_from_a_gap_position` and its interior-gap branch (crates/happenstance-testkit/src/suite.rs:1533-1553), reachable because the fixture's stride is 7. tests/gappy_store_instruments.rs::assigned_positions_are_strictly_increasing_with_at_least_one_gap (PASS) states the relation only. Reviewer check `grep -nE 'assert_eq!\(.*\[[[:space:]]*[0-9]' crates/happenstance-testkit/tests/gappy_store_instruments.rs` returns nothing"
  mount_point: "crates/happenstance-testkit/src/lib.rs"
  verifying_test: "crates/happenstance-testkit/tests/gappy_memory_conformance.rs — event_store_conformance!(GappyFixture::new()), exercising crates/happenstance-testkit/src/suite.rs:1533-1553; plus crates/happenstance-testkit/tests/gappy_store_instruments.rs::assigned_positions_are_strictly_increasing_with_at_least_one_gap"

- id: AC-009
  criterion: "**GIVEN** an author whose test must distinguish *the fixture broke this on purpose* from *my store failed*, **WHEN** either failure reaches them, **THEN** they match on `FaultyStoreError::Injected` versus `FaultyStoreError::Store(e)` — a typed, `#[non_exhaustive]`, `core::error::Error` value whose `Store` arm keeps the inner error reachable through `source()` rather than occluding it behind a rendered string — and the wrapper compiles over a store whose `Error` is **uninhabited** (`pub enum MemoryStoreError {}`) with no `unwrap`, `expect` or `unreachable!` anywhere in the path"
  satisfied: true
  evidence: "crates/happenstance-testkit/src/faulty.rs:85-116 — `#[non_exhaustive] pub enum FaultyStoreError<E> { Injected, Store(E) }` with a hand-written `Display` and a `core::error::Error` whose `source()` (:110-115) returns the inner error rather than a rendered string. Verified by tests/faulty_store_instruments.rs::injected_and_store_errors_are_distinguishable_and_the_source_chain_survives (PASS; asserts the discriminant on both arms and downcasts through `source()`). The uninhabited case is discharged by the crate compiling at all over `MemoryEventStore`. Reviewer check `grep -nE 'unwrap\(\)|expect\(|unreachable!' crates/happenstance-testkit/src/faulty.rs crates/happenstance-testkit/src/gappy.rs` returns nothing"
  mount_point: "crates/happenstance-testkit/src/lib.rs"
  verifying_test: "crates/happenstance-testkit/tests/faulty_store_instruments.rs::injected_and_store_errors_are_distinguishable_and_the_source_chain_survives"

- id: AC-010
  criterion: "**GIVEN** a reader who meets these four items on the testkit's rendered crate-root page and has not read this spec, **WHEN** they scan the item table and open one item, **THEN** each row shows a **complete first-sentence claim of ≤ 80 characters that is not truncated with an ellipsis**, each identifier is ≤ 24 characters, each item page carries a runnable doctest driven by `happenstance_testkit::block_on` (no runtime dependency added), an `# Errors` section on every fallible function naming conditions rather than types, and — on each store — the **named wrong implementation it exists to reject**, so the instrument teaches what it is for without this document"
  satisfied: true
  evidence: "`cargo test --doc -p happenstance-testkit` compiles and runs every new doctest (src/faulty.rs:71, :182, :219; src/gappy.rs:56), each driven by `happenstance_testkit::block_on` with no runtime dependency added. `RUSTDOCFLAGS=-D warnings cargo doc -p happenstance-testkit --no-deps --all-features` is clean. Measured against the rendered item table in target/doc/happenstance_testkit/index.html: `FaultyStore` 69 chars, `SendFaultyStore` 68, `GappyMemoryStore` 68, `FaultyStoreError` 55 — every one a complete claim under 80 characters with no ellipsis; identifiers 11/15/16/16 are all under 24. `# Errors`/`# Panics` are on the fallible and panicking items (src/gappy.rs:45-52), and each store's doc names the wrong implementation it rejects (src/faulty.rs:164-168, src/gappy.rs:33-39)"
  mount_point: "crates/happenstance-testkit/src/lib.rs"
  verifying_test: "cargo test --doc -p happenstance-testkit over crates/happenstance-testkit/src/faulty.rs and src/gappy.rs, plus cargo doc under -D warnings; reviewed against _design.md:845-857 (density) and _design.md:983-987 (anti-patterns 5 and 6)"
```
