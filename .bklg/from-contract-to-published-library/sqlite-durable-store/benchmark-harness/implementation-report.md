---
item: "HS-S0034"
stage: implement
created: "2026-08-16"
updated: "2026-08-16"
---

# Implementation Report — event_store_benchmarks! in the testkit, provably not conformance

> **STATUS: eight of eight ACs satisfied.** `happenstance-testkit` has a fourth
> macro family, and it is the only one that checks nothing. The claim that
> benchmarks are not the bar is now a property of the tree rather than a
> sentence in a report: `git diff --name-only HEAD` over `suite.rs`,
> `registry.rs`, `model.rs`, `concurrency.rs`, `projection.rs`, `xtask/` and
> `spec/` returns **empty**, and the registered rule-name count is 89 on both
> sides.
>
> The harness ships **executed**, not compiled: `tests/memory_benchmarks.rs`
> runs all three scenarios against `MemoryFixture` through three different
> emitters, one of which is written in the caller's crate and owns the clock.

## TDD Evidence

RED first, from the mount written before the module existed. The mount is an
*integration* test file, so it can name only public items — the whole file fails
to compile if the capability is not really on the crate's surface, which is what
makes this red load-bearing rather than a formality:

```text
error[E0432]: unresolved import `happenstance_testkit::bench`
  --> crates\happenstance-testkit\tests\memory_benchmarks.rs:36:27
   |
36 | use happenstance_testkit::bench::{BenchmarkParams, BenchmarkRecord, scenarios};
   |                           ^^^^^ could not find `bench` in `happenstance_testkit`

error[E0433]: cannot find `event_store_benchmarks` in `happenstance_testkit`
  --> crates\happenstance-testkit\tests\memory_benchmarks.rs:82:23
```

The `bench = []` feature was added *before* that run on purpose. Without it the
target's `#![cfg(all(feature = "bench", …))]` compiles the file to nothing and
`cargo test` reports a vacuous green — a false red is a worse failure than a
loud one.

GREEN: **18 of 18** in `tests/memory_benchmarks.rs`, plus 5 in `bench`'s own
`#[cfg(test)] mod tests`, plus 23 doctests.

| AC | Test | Red → Green |
| --- | --- | --- |
| AC-001 | `all_three_scenarios_complete_against_the_reference_fixture`, plus the nine generated `dcb_benchmarks{,_blocking,_timed_csv}::*` tests and the file compiling at all | `E0432: could not find bench in happenstance_testkit` → 18 passed |
| AC-002 | `registry::no_orphan_rules` (unchanged, still green), `cargo xtask spec-trace`, `cargo xtask lints` | green before, green after, same 89/112 counts either side |
| AC-003 | `cargo hack check -p happenstance-testkit --target wasm32-unknown-unknown --feature-powerset --no-dev-deps` | 12/12 combinations green, including the four that set `bench` on wasm32 |
| AC-004 | `dcb_benchmarks_timed_csv::*` (a caller-written emitter using `std::time::Instant`) and `dcb_benchmarks_blocking::*` | `E0433` on the macro → both emitters driving the same three scenarios |
| AC-005 | `the_append_scenario_appends_the_callers_batch_size` (n = 1, 9), `a_contended_run_accounts_for_every_contender` (k = 2, 8), `replay_reports_a_filtered_and_an_unfiltered_pass` (N = 20) | red on the missing `BenchmarkParams` → green at three distinct parameter sets |
| AC-006 | `a_contended_run_accounts_for_every_contender`, `bench::tests::the_three_refusal_kinds_stay_apart` | red on the missing record → `committed == 1`, `rejected == k - 1`, `failed == 0`, four counters summing to k |
| AC-007 | `cargo run -p xtask -- lint-clock`, `a_record_summarises_itself_in_one_line`, `bench::tests::a_summary_is_one_line` | 10 files scanned before, **11** after, still `read no clock` |
| AC-008 | both `cargo doc` runs under `RUSTDOCFLAGS=-D warnings` | one real red — `unresolved link to Fixture` in `bench.rs`'s module docs under `--all-features` — fixed by qualifying it `[…](crate::Fixture)`, which is the D13 trap firing on the new module before it could reach `main` |

Two ECs got tests of their own rather than prose. EC-003 is three
`#[should_panic(expected = "<parameter name>")]` tests; EC-004 needed a store
the reference implementation cannot be, so `BatchCeilingFixture`
(`tests/memory_benchmarks.rs:108`) states `MAX_EVENTS_PER_BATCH = Some(2)` and
`a_declared_batch_ceiling_is_reported_as_a_refusal` asserts the append is
counted as `refused`, not `committed`, and does not panic.

## Commits

`feat(sqlite-durable-store): Add event_store_benchmarks! to happenstance-testkit`
— the first story checkpoint of slice `bench-harness-and-adr`, on
`initiative/from-contract-to-published-library`, immediately after `00fd2f9`
(`Record HS-P0012 run 1 and its budget checkpoint`).

Named by subject and by predecessor rather than by hash, because this report is
committed *inside* the commit it describes and no hash it could quote would
survive being written into it.
`git log --grep "Story: sqlite-durable-store/benchmark-harness"` resolves it.

| SHA | Subject |
| --- | ------- |
| *(see above)* | `feat(sqlite-durable-store): Add event_store_benchmarks! to happenstance-testkit` |

## Changes

| File | Shape of the change |
| --- | --- |
| `crates/happenstance-testkit/src/bench.rs` | **New, 978 lines.** `BenchmarkParams` (three caller-supplied numbers, a `SMOKE` const, degenerate values refused by name), `BenchmarkPass` and `BenchmarkRecord` (counts, never durations), the private `Outcome` four-way split, the private `Interleaved` future that drives *k* appends round-robin on one thread, `pub mod scenarios` with the three workloads, `for_each_event_store_benchmark!`, two emitters and `event_store_benchmarks!` with five arms. |
| `crates/happenstance-testkit/src/lib.rs` | The gated `pub mod bench;` declaration with `model`'s double `cfg` and `doc(cfg(...))`; a crate-doc paragraph placing the family beside the other three; the D13 paragraph extended from two modules to three. |
| `crates/happenstance-testkit/Cargo.toml` | `bench = []` in `[features]`, with the reasoning in a comment. `[dependencies]` unchanged. |
| `crates/happenstance-testkit/tests/memory_benchmarks.rs` | **New, the mount.** Three invocations, a caller-written timing emitter, a limit-declaring fixture, and seven assertions the generated tests cannot make. |
| `crates/happenstance-testkit/README.md` | A `### The benchmark harness — which is *not* the bar` section with a four-row family table whose last column is *Can fail a merge*. |
| `CHANGELOG.md` | `## [Unreleased]` / `### Added`, naming the feature, the CF-34 boundary and the caller-owned clock. |
| `standards/rust/{41,52,62,91}-*.md` | Nine `file:line` citations into `happenstance-testkit`'s `lib.rs` and `Cargo.toml` repointed, because inserting the module declaration moved every line below it. Content unchanged; `cargo xtask lint-constitution` names each one it wants moved. |
| `.bklg/…/benchmark-harness/{_ledger.md,implementation-report.md,report.md}` | Eight AC rows flipped with cited evidence; these two reports. |

Nothing under `crates/happenstance-sqlite/`, `.kb/`, `references/adr/`,
`spec/`, `xtask/` or `.redkiln/config.yaml` moved.

## Gates

| Command | Result |
| --- | --- |
| `cargo test -p happenstance-testkit --features bench` | 18 + 5 + 23 passed, 0 failed |
| `cargo xtask affected --base main` | **green** — fmt, clippy `-D warnings`, tests for the affected set, the five file-reading lints and `spec-trace` |
| `cargo run -p xtask -- lint-clock` | `CF-33: 11 file(s) in crates/happenstance-testkit/src read no clock` |
| `cargo xtask lints` | green, including `CF-29: all 112 rules in 4 file(s)`, `27 atoms, all consistent` |
| `cargo xtask spec-trace` | `201 clauses … 112 conformance rules … traceability: no problems found` |
| `cargo clippy -p happenstance-testkit --all-features --all-targets` | clean |
| `cargo hack check -p happenstance-testkit --feature-powerset --no-dev-deps` | 12/12 |
| `cargo hack check -p happenstance-testkit --target wasm32-unknown-unknown --feature-powerset --no-dev-deps` | 12/12 — **run explicitly**, since `--fast` skips it and it is the only step that compiles `bench` on wasm32 |
| `RUSTDOCFLAGS=-D warnings cargo doc -p happenstance-testkit --no-deps` | green (default features) |
| `RUSTDOCFLAGS=-D warnings cargo doc -p happenstance-testkit --all-features --no-deps --document-private-items` | green |
| `cargo fmt --all -- --check` | clean |

## Notes

**Five macro arms, not three.** The spec's *Behavior and interfaces* table says
the entry point takes "the same three arms" as the other families, and AC-005
says *n*, *k* and *N* are supplied at the call site. Both cannot be true of three
arms, so the three are mirrored exactly and `params = …` is threaded through as
an optional keyword: `($fixture)`, `(mod_name, fixture)`, `(mod_name, emit,
fixture)` behave identically to their siblings and default to
`BenchmarkParams::SMOKE`, and two further arms accept `params`. A caller who has
invoked one family has invoked this one.

**Three invocations in the mount, not two.** The spec asks for two emitters. The
third exists because AC-004's GIVEN is an author who wants `criterion` or a CSV
row — and the strongest available proof that the *timer* is theirs is an emitter
written in the caller's crate that reads a clock the testkit may not spell.
`emit_timed_csv!` (`tests/memory_benchmarks.rs:51`) is that, and it also
demonstrates that a bare macro name resolves in the caller's scope.

**`RULE_FILES` is four elements, not three.** AC-002's text says "still a
three-element array"; it has been four since the projection family
(`xtask/src/spec_trace.rs:87-93`). It is unchanged at four, which is the
criterion's content — the count of *files a rule may live in* did not move
because no rule was added. Recorded in the ledger rather than silently
satisfied.

**NF-006's arithmetic.** The note says the host powerset goes "from 2
combinations to 4". The crate already declared three features (`default`,
`memory`, `proptest`), so `cargo hack` ran 6 before and runs 12 now. The
substance of NF-006 — exactly one new feature, no sub-features, no
`bench-criterion`, no `default`-implied variant — holds.

**The clock lint reshaped the record type, as the spec predicted it would.**
`BenchmarkRecord` carries no duration field at all, not even one an emitter
fills in: counts are what a harness under CF-33 can produce, and the emitter
already has the number. `tests/memory_benchmarks.rs`'s CSV emitter prints
`record.summary()` and its own nanoseconds side by side, which is the shape
ADR-0022 will consume.

**Contention is interleaved, and against `MemoryFixture` it produces a real
rejection mix.** The spec flagged this as a finding to report if it did not:
`Interleaved` builds all *k* futures before polling any, and against a store
that serialises its writers the result is `committed == 1, rejected == k - 1,
failed == 0` at k = 2 and k = 8. That is the ADR-0012 mix, on one thread, with
no `Send` bound anywhere in the family. **The caveat HS-S0035 inherits**: an
adapter whose `append` performs real I/O will interleave differently, and an
adapter that wants OS-thread contention supplies it from its own emitter — the
family binds `Fixture` and will not be widened to buy it.

**Nine constitution citations moved.** Inserting `pub mod bench;` near the top of
`lib.rs` shifted every line below it, and `cargo xtask lint-constitution` — part
of `cargo xtask lints`, and therefore of the story gate — failed with nine
"citation points at the wrong place" problems. The anchors were repointed to
the same anchor text; no atom's prose changed. This is the standing cost of
`file:line` citations into a moving file, and the lint is what makes it visible
in the same change rather than three phases later.
