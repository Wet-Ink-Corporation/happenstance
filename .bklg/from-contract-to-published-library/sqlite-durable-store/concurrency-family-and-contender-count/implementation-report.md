---
item: "HS-S0041"
stage: implement
created: "2026-08-17"
updated: "2026-08-17"
---

# Implementation Report — The concurrency family green, and the 8-versus-64 discrepancy closed

> **STATUS: seven of seven ACs satisfied.** The racing family is mounted against
> a real SQLite file and all five rules report `Ran` and green — at **64
> contenders**, because the constant was raised rather than the runbook amended.
>
> The seam ADR-0022 §9 decided was **already in the tree** when this story
> started, landed by `schema-migration-and-identity` / `sqlite-fixture-and-whole-suite`.
> That changes what the interesting work was, and it is written up honestly
> below: the story's contribution is the **mount**, the **contender-count
> decision with its verification**, and — the part worth the most — the
> **negative control that shows what a half-done seam looks like from inside the
> family's own output**.

## TDD Evidence

| AC | Test | Red, and for what reason | Green |
| --- | --- | --- | --- |
| **AC-001** | `dcb_concurrency_conformance::{exactly_one_of_n_contenders_commits, k_disjoint_boundaries_admit_exactly_k_commits, positions_are_unique_under_concurrent_appends, append_returns_the_callers_own_last_position, a_concurrent_reader_never_sees_a_partial_batch}` | **Absent from the binary.** Before this story `cargo test -p happenstance-sqlite --test concurrency` was `error: no test target named 'concurrency'` — the family did not exist for this adapter at all, which is the red an unmounted capability has | 9 passed; 0 failed; 0 ignored |
| **AC-002** | `concurrency.rs:96 store_serves_a_bare_thread_with_no_ambient_runtime` | **Red on demand, and it was run.** See *The negative control* below — with the captured handle removed the test fails at `concurrency.rs:128` with `no tokio runtime is available to run the blocking SQLite query` | Green with the seam in place |
| **AC-002** (second half) | `concurrency.rs:160 a_store_with_no_runtime_anywhere_reports_no_runtime` | Would go red if anyone implemented ADR-0022's rejected option (b), because the read would succeed inline and the assertion says so in as many words | Green |
| **AC-003** | `concurrency.rs:202 every_connection_carries_the_declared_busy_timeout` | Reads `PRAGMA busy_timeout` back off the **second and third** `connect()`, so it goes red for a store that configures only its first connection — the shape `schema-migration-and-identity`'s own test could not see | Green on all three handles at 5,000 ms |
| **AC-004** | the four racing rules above | Falsified permanently by `mutation_coverage::the_concurrency_rules_reject_exactly_what_they_claim`, re-run green **at 64** | Green |
| **AC-005** | `dcb_concurrency_conformance::a_concurrent_reader_never_sees_a_partial_batch` | **Observed red for the wrong reason on purpose** — under the removed handle it reported `a reader observed 128 batch(es) part-written`, every sighting a `NoRuntime`. That is the criterion's whole point | Green, with the seam test green beside it |
| **AC-006** | `cargo test -p happenstance-testkit --all-features`, `--test concurrency` | The cost of the raise was **measured, not argued**: 0.37 s at 8 against 2.13 s at 64 for the five-rule family | Everything green at 64 |
| **AC-007** | `tests/shapes.rs`, `tests/conformance.rs`, `concurrency.rs:237` | `the_shared_fixture_is_a_module_and_not_a_third_test_target` goes red the moment anyone moves the fixture to `tests/support.rs` — where it would become a binary that builds `rusqlite`, runs nothing and reports success | 10 + 89 passed |

### The negative control, run once, and what it proves

AC-002's spec asks for the seam test's negative control to be *run once and cited*.
It was. `crates/happenstance-sqlite/src/event_store.rs:330` was temporarily changed
from `runtime: Handle::try_current().ok()` to `runtime: None`, and the whole target
re-run:

```text
test store_serves_a_bare_thread_with_no_ambient_runtime ... FAILED
  read from a bare thread failed: no tokio runtime is available to run the
  blocking SQLite query: there is no reactor running

test dcb_concurrency_conformance::a_concurrent_reader_never_sees_a_partial_batch ... FAILED
  a reader observed 128 batch(es) part-written: ["a concurrent read failed: no
  tokio runtime is available …", … ×128]

test dcb_concurrency_conformance::exactly_one_of_n_contenders_commits ... ok
test dcb_concurrency_conformance::k_disjoint_boundaries_admit_exactly_k_commits ... ok
test dcb_concurrency_conformance::positions_are_unique_under_concurrent_appends ... ok
test dcb_concurrency_conformance::append_returns_the_callers_own_last_position ... ok
```

**Four rules green and one rule wrong about atomicity.** The reader rule renders a
store error as a *sighting*, so the verdict reads `128 batches part-written` — a
claim about ES-18 — for a defect that has nothing to do with ES-18. That is
precisely the failure the spec's Context pack calls *the specific way this story
fails silently*, and it is why the seam test exists beside the family rather than
being trusted to fall out of it. The patch was reverted; `git diff` on
`event_store.rs` is empty.

## Commits

- `0b0e29f` — `feat(sqlite-durable-store): The concurrency family, at the contender count the DoD asks for`

## Changes

| File | Shape of the change |
| --- | --- |
| `crates/happenstance-sqlite/tests/support/mod.rs` | **New location** (`git mv` from `tests/conformance.rs`). Carries `SqliteFixture` and nothing else; the macro invocation moved out. Module doc rewritten to explain the `tests/support/mod.rs`-not-`tests/support.rs` choice. `pub` → `pub(crate)` on the type and `new`, because `unreachable_pub` is on in a test binary that no longer exports it |
| `crates/happenstance-sqlite/tests/conformance.rs` | **Rewritten to a mount.** `mod support;` + `use support::SqliteFixture;` + `event_store_conformance!(SqliteFixture::new())`. 24 lines |
| `crates/happenstance-sqlite/tests/concurrency.rs` | **New test target.** The macro mount, plus four adapter-local tests: the bare-thread seam test, the `NoRuntime`-still-reachable control, the per-connection busy-timeout read-back, and the filesystem assertion that no third binary was minted. Module doc records the **one-emitter decision** (EC-005) and the no-watchdog constraint |
| `crates/happenstance-testkit/src/concurrency.rs` | `CONTENDERS` 8 → **64**, with the doc comment rewritten from four lines to a section stating the reason, ADR-0022 §12's measurement, and the workspace-wide cost by name. **No other line of the testkit changed** — no bound, no emitter, no rule membership |
| `RUNBOOK.md` | Phase 8's proof artefact keeps its 64 and loses `across 25 rounds`, which matched nothing; the resolution of the discrepancy is recorded where a phase-8 reader meets it |

Nothing under `crates/happenstance-sqlite/src/**` changed. That was not the plan —
the spec expects the seam to be this story's main body of work — and the reason is
in *Notes*.

## Gates

| Command | Result |
| --- | --- |
| `cargo test -p happenstance-sqlite --test concurrency` | **9 passed; 0 failed; 0 ignored** — five rules + four adapter-local tests, 2.13 s at 64 contenders |
| `cargo test -p happenstance-sqlite --test conformance` | **89 passed; 0 failed** — the sequential suite survives the fixture relocation intact |
| `cargo test -p happenstance-sqlite --test shapes` | **10 passed** — the `Handle` field costs the shape assertions nothing |
| `cargo test -p happenstance-testkit --all-features` | **green at 64 across every target**, including `memory_concurrency_conformance` (10) and `mutation_coverage` (10, 4.92 s) |
| `cargo xtask wasm` | green, four steps unmoved — the family does not exist on that target |
| `cargo xtask affected --base main` | **`affected gate passed`** — fmt, clippy `-D warnings`, tests, the five file lints and `spec-trace` |
| `cargo fmt --all -- --check` | clean |

**Wall clock, recorded as NF-001 asks** (a human timing a command is not a rule
reading a clock — CF-33 binds *rules*): the five-rule family takes **0.37 s at
`CONTENDERS = 8`** and **2.13 s at 64** on this machine, a factor of 5.8 rather than
ADR-0022's measured 10-20x for a bare race — the difference is fixture setup, which
does not scale with the contender count. Comfortably inside any CI job timeout.

## Notes

**The seam was already there, and that is a deviation worth stating plainly.**
The spec's *Implementation notes* open with *"Do the seam first, then mount"* and
treat the runtime capture as the body of the work. It was in the tree before this
story started: `SqliteEventStore` already carried `runtime: Option<Handle>`
(`src/event_store.rs:188`), captured at `with_store_id` (`:330`) and preferred at
`poll_next` (`:1469-1475`), with `ReadCursor` carrying it through. So the story's
code contribution reduced to the mount and the constant.

That could have been reported as "AC-002 was already satisfied", and it would have
been the wrong report. An AC is satisfied by *reachable behaviour with a test that
can fail*, and before this story nothing in the workspace drove this adapter from a
thread with no ambient runtime. The seam test and its negative control are what turn
an existing field into a checked claim — and the negative control is the single most
informative artefact this story produced, because it shows the family's own output
lying about what is wrong.

**Branch A on the contender count, and the third option was declined.** ADR-0022 §12
made a third option visible — invoke the harness at 64 without moving the shared
constant — and it is genuinely attractive: the cost would land only here. It was not
taken. The constant's own documentation says it is *the number an adapter author has
to size a connection pool against*; a per-adapter override makes that sentence false
for every reader, and the discrepancy would survive in a new form. AC-006 also names
exactly two acceptable outcomes and a third is not one of them.

**`RUNBOOK.md`'s historical session logs were left alone.** The same discrepancy is
recorded at `:2031` and `:2686` inside phase-3 and phase-7 session records. Those are
what was believed *then*; rewriting them would be editing a log. Only the live
phase-8 proof artefact was corrected, which is where a reader acts on it.

**Nothing was flipped to make the run look fuller.** `MID_BATCH_FAULT` is still
declined with the sentence `sqlite-fixture-and-whole-suite` wrote, byte for byte,
and the blocking emitter is declined **in the module doc with its mechanical
reason** rather than silently omitted — under it there is no tokio runtime on any
thread, including the one that constructs the fixture, so ADR-0022 §9's captured
handle would be `None` and every rule would fail with `NoRuntime`.
