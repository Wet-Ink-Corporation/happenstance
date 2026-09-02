---
item: "HS-S0024"
stage: implement
created: "2026-08-16"
updated: "2026-08-16"
---

# Implementation Report — FaultyStore<S> and GappyMemoryStore in the testkit

**All ten ACs are satisfied. Nothing is blocked and nothing is deferred.**

Four public items land in `happenstance-testkit` and are `pub use`d at its crate root:
`FaultyStore<S>`, `SendFaultyStore<S>`, `FaultyStoreError<E>` and — behind the newly
declared, on-by-default `memory` feature — `GappyMemoryStore`. Both new stores were put
through `event_store_conformance!` and both pass all 89 rules. The crate-root page gains
one region, *Stores that misbehave on purpose*, so a reader meets the instruments where
they meet the suite.

No conformance rule was added, so no adapter's CI can turn red because of this. Nothing
under `crates/happenstance-core/src/**`, `crates/happenstance-testkit/src/suite.rs` or
`crates/happenstance-testkit/tests/mutation_coverage/` was touched — `GappedPositionStore`,
CF-5's conformant control, is exactly as it was.

## TDD Evidence

Two red observations were recorded, and the second is the meaningful one. The first is the
unavoidable Rust shape — a test naming an item that does not exist is a compile error. The
second is the one a reviewer should read: an **inert** implementation was landed first
(arming counters recorded but never consulted; the gappy allocator advancing by one), so
every assertion failed on the value it was written to assert rather than on a missing
symbol.

| AC | Test | Red | Green |
| --- | --- | --- | --- |
| AC-001 | `faulty_store_instruments.rs::instruments_are_reachable_from_the_crate_root` | `error[E0432]: unresolved imports happenstance_testkit::FaultyStore, happenstance_testkit::FaultyStoreError — no FaultyStore in the root` | PASS once `src/lib.rs:337-340` re-exports the four items |
| AC-002 | `::injected_violation_reports_no_conflicting_position`, `::injected_violation_does_not_touch_the_inner_store` | FAILED at `tests/faulty_store_instruments.rs:134` and `:164` against the inert wrapper — the armed append delegated and **succeeded**, so there was no violation to inspect | PASS once `faulty.rs:340-343` returns above the delegation |
| AC-003 | `::retry_gated_on_some_conflicting_position_never_retries`, `::retry_gated_on_is_condition_violated_succeeds_on_the_second_attempt` | FAILED at `:253` and `:279` — with nothing injected the wrong loop *also* succeeded, which is exactly the blindness the instrument removes | PASS; `submitted == 2`, committed position equals the store's own `head()`, third append delegates |
| AC-004 | `::injected_read_failure_is_the_first_polled_item_not_a_call_time_error`, `faulty_store_send_guard.rs::spawns_from_generic_over_send_faulty_store` | FAILED at `:344` — the first item was the store's own event, not `Err(FaultyStoreError::Injected)` | PASS; and the spawn guard compiles, which is the assertion that `read` still returns the stream at the top level |
| AC-005 | `faulty_store_conformance.rs` → `event_store_conformance!(FaultyFixture::new())` | n/a — this rule set is the *control*; it was green from the first compiling version and stayed green, which is the claim | 89 passed, 0 failed |
| AC-006 | `gappy_store_instruments.rs` (`#![cfg(… feature = "memory")]`) | `warning: unexpected cfg condition value: memory … expected values for feature are: default and proptest` — the file compiled to **nothing**, which is the failure the AC names | PASS; `Cargo.toml:63-64` declares the feature and puts it in `default` |
| AC-007 | `::handler_assuming_position_plus_one_disagrees_with_the_store`, `::handler_resuming_past_an_inclusive_checkpoint_sees_every_event_once` | FAILED at `gappy_store_instruments.rs:94` — against the dense allocator the wrong handler's `previous + 1` **agreed** with the store | PASS; `gappy.rs:110-128` allocates `previous + stride` |
| AC-008 | `gappy_memory_conformance.rs`, `::assigned_positions_are_strictly_increasing_with_at_least_one_gap` | FAILED at `gappy_store_instruments.rs:56` — *"this instrument exists to leave a hole, and left none"* | PASS; 89 rules green, and the relation holds |
| AC-009 | `::injected_and_store_errors_are_distinguishable_and_the_source_chain_survives` | FAILED at `:438` — no injected error was produced, so there was no discriminant to distinguish | PASS; both arms asserted, `source()` downcast walked |
| AC-010 | `cargo test --doc -p happenstance-testkit`; `cargo doc` under `-D warnings` | The items did not exist, so neither did their pages | 20 doctests pass, 3 compile-fail pass, `cargo doc` clean, item-table summaries measured on the rendered HTML |

The red transcripts, in order:

```
test result: FAILED. 2 passed; 6 failed   (faulty_store_instruments)
test result: FAILED. 1 passed; 2 failed   (gappy_store_instruments)
```

and after the two inversions were removed:

```
test result: ok. 8 passed  (faulty_store_instruments)
test result: ok. 1 passed  (faulty_store_send_guard)
test result: ok. 3 passed  (gappy_store_instruments)
test result: ok. 89 passed (faulty_store_conformance)
test result: ok. 89 passed (gappy_memory_conformance)
```

No test was weakened, skipped or deleted between the two.

## Commits

One checkpoint commit, carrying the whole story:

```
feat(typed-layer-and-alpha-release): Misbehaving testkit stores
Story: typed-layer-and-alpha-release/misbehaving-testkit-stores
```

Its short SHA is reported in this run's slice digest and is recoverable here with
`git log --grep "Story: typed-layer-and-alpha-release/misbehaving-testkit-stores" --oneline`
— a commit cannot cite its own hash, and this report is inside it.

## Changes

| File | Shape of the change |
| --- | --- |
| `crates/happenstance-testkit/src/faulty.rs` | **new.** `FaultyStore<S: EventStore>` implementing `EventStore`, `SendFaultyStore<S: SendEventStore>` implementing `SendEventStore`, one private `Armed` core behind an `Arc`, the projected `FaultyStoreError<E>`, and the `InjectedRead<St>` stream. Every delegating call is fully qualified, because both flavour names are in scope in one file |
| `crates/happenstance-testkit/src/gappy.rs` | **new.** `GappyMemoryStore` — a standalone `SendEventStore` with its own strided allocator, modelled on the reference store's append/read bodies and reusing `Query::matches`, `AppendCondition::is_violated_by` and `ReadOptions` |
| `crates/happenstance-testkit/src/lib.rs` | **mount.** `mod faulty;`, `#[cfg(feature = "memory")] mod gappy;`, the four crate-root `pub use`s with the `doc_cfg` attribute on the gated one, and a new module-doc region — *Stores that misbehave on purpose* — carrying a three-column table of instrument / what it produces / what it rejects |
| `crates/happenstance-testkit/Cargo.toml` | `memory = ["happenstance-core/memory"]` and `default = ["memory"]`, with the reasoning inline. No new dependency; `Cargo.lock` is byte-identical |
| `crates/happenstance-testkit/tests/faulty_store_instruments.rs` | **new.** The two wrong callers and their conformant siblings, EC-001's inert arming, EC-004's `BrokenStore` (`!Send` by construction so it can implement the bare flavour directly), and a hand-written `drain` that keeps error items instead of discarding them the way `collect` does |
| `crates/happenstance-testkit/tests/faulty_store_send_guard.rs` | **new.** `spawns_from_generic_over_send_faulty_store`, modelled on `happenstance-core/src/memory.rs:643-680` |
| `crates/happenstance-testkit/tests/faulty_store_conformance.rs` | **new.** `FaultyFixture` over an unarmed `SendFaultyStore<MemoryHandle>`, delegating its isolation to `MemoryFixture` |
| `crates/happenstance-testkit/tests/gappy_store_instruments.rs` | **new.** The `position + 1` handler and the checkpoint-resuming sibling, plus the position relation. No literal position anywhere |
| `crates/happenstance-testkit/tests/gappy_memory_conformance.rs` | **new.** `GappyHandle` (a newtype, for the coherence reason `MemoryHandle` records) and `GappyFixture` at stride 7 |
| `CHANGELOG.md` | One `## [Unreleased] → Added` pair of entries naming both instruments, the defect each detects, and the new feature |

## Gates

| Gate | Command | Result |
| --- | --- | --- |
| Story gate | `cargo xtask affected --base main` | **affected gate passed** — fmt, clippy `-D warnings`, the five file-reading lints, `spec-trace`, and the tests for `happenstance-testkit` and its dependents |
| Formatter | `cargo fmt --all --check` | clean |
| Lint | `cargo clippy -p happenstance-testkit --all-targets --all-features -- -D warnings` | clean |
| Tests | `cargo test -p happenstance-testkit` | every target green; the two new conformance targets contribute 89 + 89 |
| Doctests | `cargo test --doc -p happenstance-testkit` | 20 passed, 4 ignored; 3 compile-fail passed |
| Feature powerset | `cargo hack --feature-powerset check -p happenstance-testkit` | 6/6 green, including `--no-default-features` |
| wasm32 | `cargo xtask wasm` | all four steps green, including the conformance-harness check |
| Docs | `RUSTDOCFLAGS="-D warnings" cargo doc -p happenstance-testkit --no-deps --all-features` | clean |
| docs.rs badge | `RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc …` | `GappyMemoryStore` renders `Available on crate feature memory only` |
| NF-001 | `git status --short Cargo.lock` | empty — no new third-party dependency |

Reviewer greps, both empty:

```
grep -nE "unwrap\(\)|expect\(|unreachable!" crates/happenstance-testkit/src/faulty.rs crates/happenstance-testkit/src/gappy.rs
grep -nE "assert_eq!\(.*\[[[:space:]]*[0-9]" crates/happenstance-testkit/tests/gappy_store_instruments.rs
```

NF-005's observation, recorded rather than gated: the two new conformance expansions add
178 tests to a crate that already ran several hundred, and both finish in well under a
tenth of a second. Neither dominates the test step.

## Notes

Four deviations from the spec's letter, each with its reason.

**DG-1 — `FaultyStoreError<E>` is a fifth public item the signed-off design does not
list.** Raised here as a design amendment rather than absorbed silently, exactly as the
spec's *Clarifications resolved during spec* item 3 asks. The port's associated `Error`
type forces it: `MemoryStoreError` is uninhabited, so `fail_next_read` has literally no
`S::Error` value to yield, and reusing `S::Error` is therefore impossible while a `String`
is forbidden by RS-30-2.

**DG-2 — the error is hand-written, not `thiserror`-derived.** The behaviour table says
"`thiserror`-derived"; NF-001 says no new third-party dependency and *"`git diff --stat
Cargo.lock` is empty"*. `happenstance-testkit` does not depend on `thiserror` today and
`tests/mutation_coverage/correct.rs` already records that fact as a deliberate house
choice. Adding it would have changed `Cargo.lock`. The `Display` and `core::error::Error`
impls are therefore written by hand at `src/faulty.rs:98-116`; `source()` returns the inner
error, which is the property AC-009 actually asserts, and the derive was only ever a means
to it.

**DG-3 — `impl SendEventStore for SendFaultyStore<S>` requires `S: Sync`.** The compiler's
demand, not a preference: every method of the `Send` flavour holds `&self` across an await,
and `&S: Send` is `S: Sync`. `SendEventStore` promises only `Send`, which is RS-21's whole
point. `MemoryHandle` never had to write it because it is concrete and already `Sync`. The
diagnostic, before the bound was added:

```
error: future cannot be sent between threads safely
   |     ^^^^^ has type `&SendFaultyStore<S>` which is not `Send`,
   |           because `SendFaultyStore<S>` is not `Sync`
```

**DG-4 — EC-006 and EC-008, the two residuals the spec left to the implementer.**

- **EC-006 (position-space exhaustion)** is a **documented panic**, named in a `# Panics`
  section on `GappyMemoryStore` (`src/gappy.rs:45-52`). A refusal is not expressible: the
  store's `Error` is `MemoryStoreError`, uninhabited, so there is no value to return. It is
  documented rather than asserted because reaching it needs about `2^64 / stride` appends,
  and a test that cannot run is not evidence.
- **EC-008 (an armed read that is never polled)** decrements at the point the **stream is
  produced**, not at first poll. Stated in `fail_next_read`'s rustdoc and asserted by
  AC-004's test, whose closing lines drain a second read and find it delegating.

One thing the spec asked for and this PR did not need: no defect in the frozen contract was
found, so nothing is routed to `defect-log-and-macros-verdict` from here.
