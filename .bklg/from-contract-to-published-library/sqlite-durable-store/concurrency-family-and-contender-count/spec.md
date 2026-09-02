---
item: HS-S0041
stage: spec
created: 2026-08-12T13:46:39.662Z
updated: 2026-08-12T13:46:39.662Z
template_sig: 87bbf1d0
rendered_sig: b5469da7
---

# Spec — The concurrency family green, and the 8-versus-64 discrepancy closed

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/sqlite-durable-store/project.md` |
| This spec | `.bklg/from-contract-to-published-library/sqlite-durable-store/concurrency-family-and-contender-count/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md` — Architecture brief §3 (the runtime seam), §6 (the write path, `BEGIN IMMEDIATE`, `SQLITE_BUSY`), §9 (the `CONTENDERS` tension); Testing brief §4 (the seam and CF-33 restated for testing), §6 (merge-gate commands) |
| Signed-off design | `.bklg/from-contract-to-published-library/sqlite-durable-store/_design.md` — **no user-facing surface**, approved 2026-08-12. This story renders none and adds none. |
| Story map row | `.bklg/from-contract-to-published-library/sqlite-durable-store/_storymap.md` — slice `race-model-and-durability` |
| Roadmap pointer | `RUNBOOK.md:4166-4237` (phase 8); the discrepancy itself at `RUNBOOK.md:2686-2696` |

## One-line PR slice

Take `event_store_concurrency_conformance!` green against `SqliteFixture` with
ADR-0022's runtime seam in place — a captured `Handle`, not `NoRuntime` from
every contender — and close the 8-versus-64 discrepancy by raising `CONTENDERS`
with a stated reason **or** amending both proof artefacts.

## Executive summary

`sqlite-fixture-and-whole-suite` (HS-S0040) leaves the workspace with its first
green `event_store_conformance!` run against a real file. Every rule in that run
is **sequential**: one caller at a time, so nothing between the probe and the
insert. This PR adds the second caller.

The delta is three things, and only the first is code most people would call a
feature:

1. **The runtime seam.** Concurrency contenders are bare OS threads under
   `std::thread::scope`, each driving its own future with the testkit's park-loop
   `block_on` (`crates/happenstance-testkit/src/concurrency.rs:45-68`). `tokio`'s
   runtime context is thread-local, so `Handle::try_current()` fails inside a
   contender and today's skeleton turns that into
   `SqliteEventStoreError::NoRuntime` by design
   (`crates/happenstance-sqlite/src/event_store.rs:26-32`, `:172-178`). Every
   contender would fail for a reason that has nothing to do with atomicity. The
   store must acquire a runtime handle where one exists — at construction — and
   carry it.
2. **A second mount.** A new `crates/happenstance-sqlite/tests/concurrency.rs`,
   shaped exactly like `crates/happenstance-testkit/tests/memory_concurrency_conformance.rs`,
   because the family is opt-in, `!Send`-hostile by construction, and does not
   exist on `wasm32` (`concurrency.rs:1105-1116`).
3. **A number that three phases assert and no code implements.** Both stated
   proof artefacts read *64 contenders*; `concurrency::CONTENDERS` is **8**
   (`crates/happenstance-testkit/src/concurrency.rs:206`). AC-005 permits exactly
   two outcomes and names the third — leaving it — as a failure, not a deferral.

This is where AC-007's *rejection* half lands: the sequential suite cannot tell
an atomic check-and-write from a probe followed by an insert, and this family is
the only thing in the workspace that can (`concurrency.rs:16-23`). The
implementation this rejects — `BEGIN DEFERRED`, probe outside the write lock —
was already written by `append-atomicity-and-store-limits`'s ADR-0022 reading;
this PR is where a store that got it wrong is *observed* to be wrong.

## Context pack

The decisions this story must honor. Read this section before opening anything;
everything deeper is an anchor below.

**The runtime seam is decided, not discovered.** Architecture brief §3 recommends
**option (a)**: capture a `tokio::runtime::Handle` at construction
(`SqliteEventStore::open` / `::new`, which run on the harness's own thread, inside
the tokio test), prefer it for every blocking hop, and keep
`Handle::try_current()` as the fallback. A `Handle` is `Clone + Send + Sync +
Unpin`, so carrying one in the store, in `ReadCursor` and in `SqliteReadStream`
costs `crates/happenstance-sqlite/tests/shapes.rs` nothing. Option (b) — run the
statement inline on the calling thread when no runtime is found — is defensible
for a synchronous driver but makes `NoRuntime` unreachable, which means the
variant, the module-doc paragraph at `event_store.rs:26-32` and
`SqliteProjectionStoreError::NoRuntime` must all be removed or rewritten in the
**same** change rather than left documenting a state that cannot occur. **ADR-0022
records which won and why**; this story implements whichever it recorded, and if
ADR-0022 chose (b) then the doc-comment demolition is part of this PR, not a
follow-up. What is not available is discovering the problem in a red run.

**The seam is wider than `read`.** `project.md`'s first risk states it for the
read path only. Writers call `store.append(...)` inside `crate::block_on` on a
scoped thread (`concurrency.rs:884-895`), and the reader in
`a_concurrent_reader_never_sees_a_partial_batch` calls
`happenstance_core::collect(store.read(...))` on its own scoped thread
(`concurrency.rs:948-953`) where a failed read is **reported as a sighting** —
`"a concurrent read failed: {err}"`. A `NoRuntime` there does not surface as an
obviously-wrong failure; it surfaces as a conformance verdict about atomicity.
That is the specific way this story fails silently if the seam is half-done.

**`SQLITE_BUSY` is an architecture problem, not a tuning knob** (architecture
brief §6). With `CONTENDERS` connections on one file, `BEGIN IMMEDIATE` on a busy
database returns `SQLITE_BUSY` *immediately* unless a busy handler is configured,
and that error becomes `AppendError::Store` → `Attempt::Failed`
(`concurrency.rs:215-232`) → a red rule that is not about the adapter's logic. The
adapter configures a **finite and generous** busy timeout, at the value ADR-0022
records. Infinite is forbidden: an unbounded busy handler converts a livelock into
a hung CI job that names no rule.

**There is no watchdog, and there must not be one.** CF-33 is `[FROZEN]`
(`spec/SPECIFICATION.md:8236`) — no conformance rule may read a clock, measure
elapsed time or assert on an operation count — and the family's own module doc
argues the case for a racing rule specifically (`concurrency.rs:24-43`). Liveness
rests on the CI job timeout. **If a rule hangs locally, that is evidence for
ADR-0022's busy-timeout paragraph, not a reason to add `#[timeout]`, a retry loop,
or a watchdog thread** (testing brief §4). CF-33 constrains a *rule*; the
adapter's own retry policy is a different thing and is allowed — that distinction
is the one an implementer under schedule pressure collapses.

**The contender count has exactly two acceptable outcomes.** AC-005:
either `concurrency::CONTENDERS` is raised **with a stated reason**, or **both**
proof artefacts are amended (`RUNBOOK.md:159`, `RUNBOOK.md:4217-4222`). The
architecture brief §9 recommends raising it, because the constant's own doc says
it is not a tuning knob and the rules assert *set* properties — *exactly one*,
*all distinct* — that hold at any size above one, so no rule is rewritten, and
because 64 is the number both artefacts already claim. **The cost is not small and
must be verified rather than assumed**: the constant is workspace-wide, so
`MemoryFixture`, the five racing mutant stores behind
`the_concurrency_rules_reject_exactly_what_they_claim`
(`crates/happenstance-testkit/tests/mutation_coverage.rs:3407`) and every future
fixture re-run at the new size, and 64 `rusqlite::Connection`s onto one file is a
real file-descriptor and busy-contention claim. If verification says otherwise,
amend both artefacts — and note that `RUNBOOK.md:4217-4218`'s phrase is *"64
contenders across 25 rounds"*, while the only `ROUNDS` in the testkit is a
rule-local `4` (`concurrency.rs:745`); an amendment must reconcile that phrase
too rather than copy it forward.

**A lost race is `Rejected`, never `Failed`.** `Attempt::of`
(`concurrency.rs:227-233`) maps `AppendError::ConditionViolated` to
`Attempt::Rejected` and *everything else* to `Attempt::Failed`. The racing rules
distinguish them by construction. So a store that probes outside the write lock
fails "exactly one winner" within a handful of iterations, and a store that leaks
`SQLITE_BUSY` as `AppendError::Store` fails the same rules for the wrong reason
and looks identical in the output. Both are red; only one is a finding about
`append`.

**No fixture in this project may decline `SECOND_HANDLE`** (testing brief §4). A
green sequential suite with a skipped or hanging concurrency family is not partial
credit. Equally: `MID_BATCH_FAULT` stays declined **with its real reason**
(architecture brief §9) — that is `reopen-negative-control-and-durability-verdicts`'s
territory, not this story's, and it is not a capability to quietly flip on to make
a run look fuller.

**The persona slice.** The user here is an adapter author and the library's
consumer, and the increment this story hands them is the one thing a sequential
suite cannot: *the store held its consistency boundary while N callers raced for
it, at a size someone chose on purpose*. `RUNBOOK.md:691` records that no fixture
in the workspace has ever opened a connection twice; this is the first story where
that second connection is put under contention rather than merely observed.

**What this story does not touch.** It does not tighten `ConcurrentFixture`'s
bound or otherwise change the testkit's *seam* — the architecture brief §3 is
explicit that the adapter solves this inside itself, because tightening a
published crate's bound for one adapter's benefit is a breaking change and
`postgres-and-neon-stores` (HS-P0014) is the project that will have the evidence
for whether the seam generalises. Raising a `const` is not tightening a bound.

## Integration contract

- **Archetype**: `capability` — a user-observable slice ending at a real `cargo
  test` target, per the story map's rule that no capability here ends at a module
  only `cargo check` sees (architecture brief AC-A01).
- **Slice / milestone**: `race-model-and-durability`. Slice-mates, implemented in
  one context and mounted as one integrated surface:
  `model-family-and-mutant-pass-column` (depends on this story) and
  `reopen-negative-control-and-durability-verdicts` (parallel).
- **Mount point**: **`crates/happenstance-sqlite/tests/concurrency.rs`** (new) —
  the real composition root for this capability. It carries
  `#![cfg(not(target_arch = "wasm32"))]` and
  `happenstance_testkit::event_store_concurrency_conformance!(SqliteFixture::new())`,
  shaped after `crates/happenstance-testkit/tests/memory_concurrency_conformance.rs`,
  which the macro's own doc names as the pattern to copy
  (`crates/happenstance-testkit/src/concurrency.rs:1114-1116`). A separate target
  rather than a `mod` inside `tests/conformance.rs`: the family is opt-in, has a
  different bound, and does not exist on `wasm32`.
- **Wires into**:
  - `SqliteFixture`, built by `sqlite-fixture-and-whole-suite` (HS-S0040). Cargo
    compiles each file directly under `tests/` as its own binary, so a fixture
    shared by two targets must live at `crates/happenstance-sqlite/tests/support/mod.rs`
    (a `tests/support.rs` would itself become a third test target). Relocating it
    there is wiring this story is permitted to do — see *PR boundary*.
  - `crates/happenstance-testkit/src/concurrency.rs` — `ConcurrentFixture`
    (`:186`), `CONTENDERS` (`:206`), `Attempt` (`:215-232`),
    `for_each_concurrency_rule!` (`:1044`),
    `event_store_concurrency_conformance!` (`:1129`).
  - `crates/happenstance-testkit/src/contract.rs` — `Fixture`, `Capability`,
    `RuleOutcome`; `SECOND_HANDLE` must stay available.
  - `crates/happenstance-sqlite/src/event_store.rs` — `SqliteEventStore`,
    `ReadCursor`, `SqliteReadStream`, `SqliteEventStoreError::NoRuntime`.
  - `crates/happenstance-sqlite/tests/shapes.rs` (exists) — the type-level guard
    that catches a field added carelessly.
  - ADR-0022, the accepted decision atom under `.kb/decisions/` plus its long
    record under `references/adr/`, authored by
    `adr-0022-append-condition-strategy`: the runtime seam, the busy timeout, the
    `synchronous` and journal settings, and the `CONTENDERS` resolution.
- **Renders surfaces**: **none.** `_design.md` records *no user-facing surface*
  for this project and that determination is what was signed off; this story adds
  no public API to `happenstance-core` or `happenstance` and mints no new `pub`
  item beyond what ADR-0022's seam requires on `SqliteEventStore`'s own
  constructors.
- **Conformance rule(s) observed**: the five in
  `crates/happenstance-testkit/src/concurrency.rs` —
  `exactly_one_of_n_contenders_commits` (`:347`),
  `k_disjoint_boundaries_admit_exactly_k_commits` (`:436`),
  `positions_are_unique_under_concurrent_appends` (`:566`),
  `append_returns_the_callers_own_last_position` (`:637`),
  `a_concurrent_reader_never_sees_a_partial_batch` (`:735`). This story adds **no
  new rule**; it makes an adapter face the existing five.
- **Clause(s)**: CF-33 `[FROZEN]` (`spec/SPECIFICATION.md:8236`) is *honored*, not
  amended — no watchdog, anywhere. ES-25's racing semantics are exercised
  (`concurrency.rs:5-14`). No clause is edited by this story; if contact with a
  real database says a frozen clause is wrong, that is a new decision atom and a
  re-plan (`project.md`, *Out of scope*).
- **Advances DoD scenario**: **initiative DoD 3** — *"The durable store passes the
  suite for real … including the concurrency case at 64 contenders."* This story
  is the only place in the initiative where the *concurrency* half of DoD 3 moves,
  and the only place the "64" in its text is reconciled with the code.

## PR boundary

`redkiln verify --grain story` reads the first fenced block under this heading and
fails on any file changed outside it.

```
crates/happenstance-sqlite/src/**
crates/happenstance-sqlite/tests/**
crates/happenstance-testkit/src/concurrency.rs
crates/happenstance-testkit/tests/**
RUNBOOK.md
.bklg/from-contract-to-published-library/sqlite-durable-store/concurrency-family-and-contender-count/**
```

**In this PR**

- The runtime seam in `crates/happenstance-sqlite/src/event_store.rs` as ADR-0022
  recorded it, carried through `SqliteEventStore`, `ReadCursor` and
  `SqliteReadStream`, plus the busy timeout and journal/`synchronous` pragmas at
  the values ADR-0022 states — applied on **every** connection the fixture opens,
  not only the first.
- The new mount `crates/happenstance-sqlite/tests/concurrency.rs`, and the
  relocation of `SqliteFixture` to `crates/happenstance-sqlite/tests/support/mod.rs`
  if HS-S0040 left it inline. **This is the composition-root wiring named in the
  Integration contract and is not scope drift.**
- The `CONTENDERS` resolution: either the constant at
  `crates/happenstance-testkit/src/concurrency.rs:206` raised with its doc comment
  rewritten to state the reason and the cost, with the whole workspace re-run at
  the new size; or `RUNBOOK.md:159` and `RUNBOOK.md:4217-4222` both amended to the
  number the code implements, the "25 rounds" phrase reconciled with
  `concurrency.rs:745`, and the verification that rejected 64 recorded.
- Whichever branch is taken, the evidence goes in this story's `_ledger.md`, and
  the ADR-0022 paragraph it discharges is cited by path.

**Explicitly not in this PR**

- `event_store_model_conformance!`, and adding `SqliteEventStore` to the mutant
  pass column or the `BEGIN DEFERRED` registry row —
  `model-family-and-mutant-pass-column` (HS-S0042), which depends on this story.
- The reopen family's negative control and the ES-35 / CF-14 / CF-17 verdicts —
  `reopen-negative-control-and-durability-verdicts`.
- Any change to `ConcurrentFixture`'s bound, to `for_each_concurrency_rule!`'s
  membership, or to the emitter contract (CF-23). Raising a `const` is permitted;
  changing the seam is HS-P0014's evidence to gather.
- Flipping `MID_BATCH_FAULT` to available, deleting the last `todo!()` or the
  scoped `#![allow(clippy::todo)]` (`instrument-markers-removed-and-gate-green`),
  and anything touching `publish = false` or `PUBLISHABLE` (architecture brief
  AC-A05).
- Writing ADR-0022. It is a hard predecessor
  (`adr-0022-append-condition-strategy`); this story *implements and cites* it and
  never authors a decision as a side effect.

**Merge DoD**: `cargo test -p happenstance-sqlite --test concurrency` green with
all five rules reporting `Ran`, the whole workspace green at the settled
`CONTENDERS`, and `cargo xtask affected --base main` green
(`.redkiln/config.yaml:40`).

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| A contender never sees `NoRuntime` | `SqliteEventStore` acquires a `tokio::runtime::Handle` where one exists — at construction, on the harness's own thread inside the tokio test — and prefers it for every blocking hop; `Handle::try_current()` stays the fallback. Covers `append` and the read path alike. If ADR-0022 recorded option (b) instead, the inline-execution path replaces the seam **and** the `NoRuntime` variant plus its documentation are removed in the same change. | `.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md` §3; `crates/happenstance-sqlite/src/event_store.rs:26-32`, `:172-178` |
| The shape guard still holds | `Handle` is `Clone + Send + Sync + Unpin`, so the new field costs nothing at the type level: `SqliteReadStream: Send + Unpin` and `SqliteEventStore: Send + Sync` still hold, unchanged in intent. | `crates/happenstance-sqlite/tests/shapes.rs`; architecture brief AC-A01 |
| Five concurrency rules run, none skipped | `event_store_concurrency_conformance!(SqliteFixture::new())` expands `for_each_concurrency_rule!` under `#[tokio::test(flavor = "multi_thread")]`; every rule reports `Ran`. `SECOND_HANDLE` stays available — declining it is not an option for this project. | `crates/happenstance-testkit/src/concurrency.rs:347,436,566,637,735`, `:1044`, `:1129`; testing brief §4 |
| The mount is a real, separately-gated test target | `crates/happenstance-sqlite/tests/concurrency.rs` with `#![cfg(not(target_arch = "wasm32"))]`, modeled on the testkit's own memory harness, which the macro doc names as the pattern. | `crates/happenstance-testkit/tests/memory_concurrency_conformance.rs`; `crates/happenstance-testkit/src/concurrency.rs:1105-1116` |
| The fixture is shared without minting a third target | A fixture used by both `tests/conformance.rs` and `tests/concurrency.rs` lives at `crates/happenstance-sqlite/tests/support/mod.rs`; a `tests/support.rs` would be compiled as its own test binary. | Cargo target layout; architecture brief §1 (fixture lives under `crates/happenstance-sqlite/tests/`, never in the testkit) |
| A lost race is `ConditionViolated`, not a driver error | The probe and the insert are one decision inside `BEGIN IMMEDIATE`, so a loser returns `AppendError::ConditionViolated` → `Attempt::Rejected`. `SQLITE_BUSY` must not reach the caller as `AppendError::Store` → `Attempt::Failed`: the adapter configures a finite, generous busy timeout at ADR-0022's value, on every connection. | `crates/happenstance-testkit/src/concurrency.rs:215-232`, `:227-233`; architecture brief §6 |
| Positions are compared, never asserted literally | The rules assert set properties — *exactly one*, *all distinct* — against positions the store actually assigned. `AUTOINCREMENT` permits gaps and no code or rule may assume `+1`. | `CLAUDE.md`, *The rule that matters*; `crates/happenstance-testkit/src/concurrency.rs:198-206` |
| No clock is added, anywhere | No `#[timeout]`, no watchdog thread, no retry loop wrapped around a rule. A hang is a finding about the busy-timeout value and is escalated to ADR-0022, not papered over. The adapter's *own* busy handler is not a rule reading a clock and is permitted. | `spec/SPECIFICATION.md:8236` (CF-33, `[FROZEN]`); `crates/happenstance-testkit/src/concurrency.rs:24-43`; testing brief §4 |
| The contender count becomes a decision with a reason | Branch A: `CONTENDERS` raised at `crates/happenstance-testkit/src/concurrency.rs:206`, its doc comment restated to give the reason and the workspace-wide cost, and every existing fixture — `MemoryFixture` and the five racing mutant stores — re-run green at the new size. Branch B: `RUNBOOK.md:159` and `RUNBOOK.md:4217-4222` both amended to the implemented number, with the verification that rejected 64 recorded. Leaving it is a failure of AC-005. | `RUNBOOK.md:2686-2696`; architecture brief §9; `crates/happenstance-testkit/tests/mutation_coverage.rs:3407` |
| Raising the constant is verified, not assumed | 64 `rusqlite::Connection`s onto one file is a file-descriptor and lock-contention claim. Whatever number is settled, the evidence that the workspace still passes at it — not an argument that it should — is what the ledger cites. | architecture brief §9; `project.md` AC-005 |
| The testkit's seam is unchanged | No change to `ConcurrentFixture`'s bound, to the emitter contract (CF-23), or to `for_each_concurrency_rule!`'s membership. The adapter solves the runtime problem inside itself; whether the seam generalises is HS-P0014's evidence. | architecture brief §3, closing paragraph; `crates/happenstance-testkit/src/concurrency.rs:186`, `:61-68` |

## Data and migrations

**No schema change.** Migration 1 is `schema-migration-and-identity`'s
(HS-S0038) and is already applied by the time this story runs; this story adds no
table, no column and no index, and must not amend the migration to make a race
pass — a schema that only works under contention is a finding for ADR-0022, not
an edit here.

Two **connection-level** settings are in scope, and they are not migrations
because they are per-connection or per-file properties rather than schema:

- **`busy_timeout`** — finite and generous, at ADR-0022's stated value, applied to
  every connection `SqliteFixture::connect()` opens rather than only the first.
  Infinite is forbidden: it converts a livelock into a hung CI job that names no
  rule (architecture brief §6; CF-33's no-watchdog trade).
- **Journal mode and `synchronous`** — WAL for two-connection concurrency, and a
  `synchronous` setting that supports the durability claim. CF-14
  (`spec/SPECIFICATION.md:7482-7485`) names `PRAGMA synchronous = OFF` **by name**
  as a wrong implementation the reopen rule exists to reject, so this story may
  not weaken it to make a race faster. Whether WAL is set per connection or
  persisted in the file is ADR-0022's call; this story applies what it says and
  does not decide it.

## Acceptance criteria

Story-grain, `AC-001` – `AC-007`. Each is written from the intent of a persona
this initiative names — the **adapter author** on *Learn when you are finished*
and the **application author** on *Replay a log* / *Write an event durably*
(`.bklg/from-contract-to-published-library/initiative.md:241-250`;
`.bklg/from-contract-to-published-library/sqlite-durable-store/_storymap.md:33-35`)
— and crosses the whole stack from the fixture's `connect()` to a `rusqlite`
transaction on a real file. Where the Context pack above says "AC-005" or
"AC-007" it means the **project's** criteria in `project.md`; the ids in this
table are this story's own. The mapping is: project AC-005 ← AC-001, AC-003,
AC-006; project AC-007 ← AC-002, AC-004, AC-005; AC-007 is the regression guard
both inherit.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an adapter author who has just taken `event_store_conformance!` green against a real SQLite file and now needs the one answer a sequential suite cannot give — whether the store holds its consistency boundary when a *second* caller is fitted between the probe and the insert (`crates/happenstance-testkit/src/concurrency.rs:17-22`) — **WHEN** they run the concurrency target, **THEN** all five rules named by `for_each_concurrency_rule!` appear in the run and every one reports `Ran`: none absent from the binary, none `Skipped`, and none aborted by the panic a declined `SECOND_HANDLE` raises (`crates/happenstance-testkit/src/contract.rs:135-161`). A green sequential suite beside a skipped concurrency family is not partial credit. | `crates/happenstance-sqlite/tests/concurrency.rs` — the new target mounting `happenstance_testkit::event_store_concurrency_conformance!(SqliteFixture::new())` under `#![cfg(not(target_arch = "wasm32"))]`, run by `cargo test -p happenstance-sqlite --test concurrency` and inside `cargo xtask ci --fast`. Evidence is the run's own per-rule output, not a summary line. |
| AC-002 | **GIVEN** every contender is a bare OS thread under `std::thread::scope` driving its own future with the testkit's park-loop `block_on`, outside any ambient reactor (`crates/happenstance-testkit/src/concurrency.rs:45-68`), and today's skeleton answers a missing runtime with `SqliteEventStoreError::NoRuntime` by design (`crates/happenstance-sqlite/src/event_store.rs:26-32`, `:176-178`), **WHEN** a contender calls `append` and when the partial-batch rule's reader calls `read`, **THEN** neither depends on `Handle::try_current()` succeeding *on that thread*: the store carries the runtime handle ADR-0022 recorded, captured where one exists at `SqliteEventStore::open` / `::new` (`crates/happenstance-sqlite/src/event_store.rs:108-132`), and no `Attempt::Failed` and no `"a concurrent read failed"` sighting anywhere in the run names a missing runtime. If ADR-0022 recorded option (b) instead, `NoRuntime` and its documentation are gone in this same change rather than left describing a state that cannot occur. | New adapter-local test `crates/happenstance-sqlite/tests/concurrency.rs::store_serves_a_bare_thread_with_no_ambient_runtime` — store constructed inside `#[tokio::test(flavor = "multi_thread")]`, then `append` and a fully drained `read` driven from a `std::thread::scope` thread via `happenstance_testkit::block_on`, both asserted `Ok`. Its negative control is the same test with the captured handle removed, which must fail; run once and cited in the ledger. Plus the five rules of AC-001 green. |
| AC-003 | **GIVEN** `CONTENDERS` + 1 `rusqlite::Connection`s onto one file and a write path that opens `BEGIN IMMEDIATE`, where SQLite returns `SQLITE_BUSY` *immediately* unless a busy handler is configured, **WHEN** contenders collide, **THEN** every non-winner is `Attempt::Rejected` because the store returned `AppendError::ConditionViolated`, and never `Attempt::Failed` (`crates/happenstance-testkit/src/concurrency.rs:224-233`) — the finite, generous busy timeout ADR-0022 states is applied by **every** connection `SqliteFixture::connect()` opens rather than only the first, and no infinite busy handler is configured anywhere. An adapter author who cannot tell a lost race from a contended driver has been handed a verdict about the wrong thing. | New test `crates/happenstance-sqlite/tests/concurrency.rs::every_connection_carries_the_declared_busy_timeout` — reads `PRAGMA busy_timeout` back from the second and third `connect()` of one fixture and asserts the declared finite value on each. Plus: the five rules green with no `Failed` variant printed in the run. |
| AC-004 | **GIVEN** the named wrong implementation is a store that probes *outside* its write lock — `BEGIN DEFERRED`, then insert — which passes the sequential rule forever and is exactly what `append-atomicity-and-store-limits` was written not to be, **WHEN** `CONTENDERS` callers race for one consistency boundary and, separately, for k disjoint ones, **THEN** exactly one commits, k disjoint boundaries admit exactly k commits, every committed position is distinct, and each winner is returned its own last position — all compared against positions the store actually assigned, never against literals, because `AUTOINCREMENT` permits gaps (`CLAUDE.md`, *The rule that matters*). | The four racing rules in the generated `dcb_concurrency_conformance` module of `crates/happenstance-sqlite/tests/concurrency.rs`: `exactly_one_of_n_contenders_commits`, `k_disjoint_boundaries_admit_exactly_k_commits`, `positions_are_unique_under_concurrent_appends`, `append_returns_the_callers_own_last_position`. Falsifiability is carried by `crates/happenstance-testkit/tests/mutation_coverage.rs::the_concurrency_rules_reject_exactly_what_they_claim` (`:3407`), re-run green at the settled `CONTENDERS`; the permanent `BEGIN DEFERRED` registry row belongs to `model-family-and-mutant-pass-column`. |
| AC-005 | **GIVEN** an application author replaying a log while writers commit — four writers each appending four rounds of three-event batches while a fifth handle reads (`crates/happenstance-testkit/src/concurrency.rs:742-763`) — and given that a failed read in that rule is reported as a **sighting**, `"a concurrent read failed: {err}"` (`:948-953`), so a driver error there arrives dressed as a verdict about atomicity, **WHEN** the reader drains `read` repeatedly during the race, **THEN** it never observes some-but-not-all events of any batch, and every sighting in the run is a genuine observation rather than a store error. This is the rule where a half-done runtime seam fails *silently*. | `dcb_concurrency_conformance::a_concurrent_reader_never_sees_a_partial_batch` in `crates/happenstance-sqlite/tests/concurrency.rs`, read together with AC-002's seam test — a green verdict here with a `NoRuntime` in the sightings is a failure of AC-002 and must be read as one. |
| AC-006 | **GIVEN** both stated proof artefacts read *64 contenders* (`RUNBOOK.md:159`, `RUNBOOK.md:4217-4218`) while `concurrency::CONTENDERS` is **8** (`crates/happenstance-testkit/src/concurrency.rs:206`), so an evaluator reading the runbook and an adapter author reading the code are told different things, **WHEN** this story merges, **THEN** exactly one of two outcomes is true and the third — leaving it — is a failure of this criterion, not a deferral: **(A)** `CONTENDERS` raised, its doc comment rewritten to state the reason and the workspace-wide cost, and the whole workspace green at the new size; or **(B)** `RUNBOOK.md:159` and `RUNBOOK.md:4217-4222` both amended to the number the code implements, the *"across 25 rounds"* phrase reconciled against the rule-local `ROUNDS = 4` (`crates/happenstance-testkit/src/concurrency.rs:745`), and the verification that rejected 64 recorded rather than asserted. | `cargo test --workspace --all-features` and `cargo xtask affected --base main` green at the settled number — which necessarily re-runs `crates/happenstance-testkit/tests/memory_concurrency_conformance.rs` and `crates/happenstance-testkit/tests/mutation_coverage.rs::the_concurrency_rules_reject_exactly_what_they_claim` at that size. The decision itself is checked by reading the rewritten doc comment at `crates/happenstance-testkit/src/concurrency.rs:195-206` (branch A) or both RUNBOOK amendments (branch B); the ledger cites which branch and the evidence for it. |
| AC-007 | **GIVEN** the seam adds a field to `SqliteEventStore`, `ReadCursor` and `SqliteReadStream`, and `tests/shapes.rs` exists precisely to catch a field added carelessly, and given that this story relocates `SqliteFixture` to a shared module so two targets can use it, **WHEN** the affected gate runs, **THEN** nothing the previous story earned is spent: `SqliteReadStream: Send + Unpin`, `SqliteEventStore: Send + Sync` and the error's `Send + Sync + 'static` bound all still hold; the sequential suite stays green with the fixture at `tests/support/mod.rs` and no third test binary minted; `MID_BATCH_FAULT` stays declined with its real reason; and the gate's four `wasm32` steps are untouched, because the family does not exist on that target. | `crates/happenstance-sqlite/tests/shapes.rs` (`read_stream_is_send_and_unpin`, `store_is_send_and_sync`, `error_is_send_sync_and_static`, `send_flavour_stream_is_send_in_generic_code`), `crates/happenstance-sqlite/tests/conformance.rs` green, `cargo xtask affected --base main`, and `cargo xtask wasm`. |

## Interaction quality

RFC §6.7/D6. **This story renders no user-facing surface**, and that is a
signed-off determination rather than an omission: `_design.md` records *"N/A — no
user-facing surface"* for the whole project and was approved on 2026-08-12 with
the `design.capture` perceptual review a declared skip
(`.bklg/from-contract-to-published-library/sqlite-durable-store/_design.md:10-17`,
`:80-89`). There is therefore **no COMPOSITION family here** — no chrome, no
density budget, no placement, no hierarchy, no transience policy — and inventing
one would contradict a design a human has already signed.

What *is* real is the STATE family, and it applies in the only medium this story
has: the conformance run itself is the adapter author's surface on the *Learn
when you are finished* journey, and `RuleOutcome::report` is what it renders.
Every invariant below is carried as a **row in the acceptance-criteria table
above** — none of them is a prose bullet here, because `redkiln verify` extracts
ACs from `| AC-### |` cells and a bullet in this section would never be gated.

| STATE invariant | Read here as | Carried by | How verified |
| --- | --- | --- | --- |
| Non-occlusion | A rule may never vanish from the binary or be hidden behind a skip. All five appear and report `Ran`; a declined `SECOND_HANDLE` panics with the fixture's own words rather than quietly reducing the run. | **AC-001** | Per-rule output of `cargo test -p happenstance-sqlite --test concurrency`; `crates/happenstance-testkit/src/contract.rs:135-161` |
| Honest failure attribution (no misleading state) | A red verdict names the thing that is actually wrong. A missing runtime or a contended driver must not be rendered as an atomicity failure or as a reader sighting. | **AC-002**, **AC-003**, **AC-005** | The seam test, the `PRAGMA busy_timeout` read-back, and the absence of `Failed` / `NoRuntime` in the run's text |
| Reversibility, and no hidden retry | No watchdog, no `#[timeout]`, no retry loop wrapped around a rule — a hang stays a legible finding escalated to ADR-0022 rather than being papered over. The adapter's own busy handler is not a rule reading a clock and is permitted. | **AC-003** (finite handler, no infinite one), **AC-001** (no rule wrapped) | `spec/SPECIFICATION.md:8236` (CF-33 `[FROZEN]`); `crates/happenstance-testkit/src/concurrency.rs:24-43`; review of the diff for any added clock |
| In-place, not a context jump | The capability is added *beside* the existing suite, not by rewriting it: the sequential target keeps its behaviour, the fixture moves without minting a third test binary, and the testkit's seam is unchanged. | **AC-007** | `crates/happenstance-sqlite/tests/conformance.rs` and `crates/happenstance-sqlite/tests/shapes.rs` green; no diff to `ConcurrentFixture`'s bound |
| Reachable by one documented command | The new capability is reachable from a command the repository already defines, not from a bespoke incantation: `cargo test -p happenstance-sqlite --test concurrency`, and inside `cargo xtask ci --fast` without extra flags. | **AC-001**, **AC-006** | `.redkiln/config.yaml:40`, `:55`; testing brief §6 |
| Preserved state across the change | The numbers and verdicts other fixtures already earned survive the contender-count decision — `MemoryFixture` and the five racing mutant stores re-run at the settled size rather than being exempted from it. | **AC-006** | `crates/happenstance-testkit/tests/memory_concurrency_conformance.rs`; `crates/happenstance-testkit/tests/mutation_coverage.rs:3407` |

**Named anti-pattern, inherited from the signed-off design's spirit and from the
testing brief**: making the run *look* fuller than it is. Flipping
`MID_BATCH_FAULT` to available, declining `SECOND_HANDLE` to get past a red rule,
or splitting the family across targets so a failure is not in the same output as
the rest — each is rejected by AC-001 and AC-007 rather than by a reviewer's
memory (`.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md`,
testing brief §4).

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | A contender reaches SQLite with no ambient tokio runtime on its thread and the store has no captured handle. | `SqliteEventStoreError::NoRuntime` surfaces as `Attempt::Failed` in a racing rule and as a *sighting* in the reader rule — a red verdict about atomicity for a reason that has nothing to do with atomicity. This is a **defect of this story**, not a fixture limitation: it is what AC-002 forbids and what `store_serves_a_bare_thread_with_no_ambient_runtime` exists to catch before the family is run. |
| EC-002 | `BEGIN IMMEDIATE` finds the database locked and no busy handler is configured. | `SQLITE_BUSY` must not reach the caller as `AppendError::Store`. `Attempt::of` maps everything that is not `ConditionViolated` to `Failed` (`crates/happenstance-testkit/src/concurrency.rs:227-233`), so a contended driver and a store that probes outside its write lock fail the *same* rules and look identical in the output. The adapter configures ADR-0022's finite busy timeout on every connection; a leaked `SQLITE_BUSY` is a bug, not an environment fact. |
| EC-003 | A rule hangs — a genuine deadlock, or a busy handler that never gives up. | There is no watchdog and none may be added (CF-33 `[FROZEN]`, `spec/SPECIFICATION.md:8236`). Liveness rests on the CI job timeout. The required response is to treat the hang as evidence about ADR-0022's busy-timeout paragraph and escalate it there — **not** `#[timeout]`, not a retry loop, not a watchdog thread (testing brief §4). An infinite busy handler converts this from a red rule into a hung job that names nothing. |
| EC-004 | The fixture declines `SECOND_HANDLE` to get past a failure. | `two_handles_observe_each_others_appends` **panics** on a declined `SECOND_HANDLE` and quotes the fixture's own stated reason back (`crates/happenstance-testkit/src/contract.rs:142-160`) — the run goes red again, with an explanation. No fixture in this project may decline it; it is the one capability that is a MUST. |
| EC-005 | The blocking emitter is mounted alongside the tokio one, copying `crates/happenstance-testkit/tests/memory_concurrency_conformance.rs:32-36` verbatim. | `__emit_concurrency_blocking` generates a plain `#[test]` driven by `block_on` (`crates/happenstance-testkit/src/concurrency.rs:1093-1101`), so there is **no tokio runtime anywhere** — including on the thread that constructs the fixture. Under ADR-0022's option (a) the captured handle is `None` there and every rule fails with `NoRuntime`. This mount therefore uses the **default (tokio) emitter only**, and that is a stated decision rather than an oversight; a second emitter arrives only if ADR-0022 recorded option (b), and then as part of that same change. |
| EC-006 | `CONTENDERS` is raised and the process runs out of file descriptors, or the fixture's connections exceed what the host allows on one file. | `CONTENDERS` is documented as *"the number an adapter author has to size a connection pool against: a fixture whose pool is smaller than this deadlocks rather than failing"* (`crates/happenstance-testkit/src/concurrency.rs:195-200`). This adapter has **no pool** by decision (`crates/happenstance-sqlite/src/lib.rs`, driver settled as `rusqlite` without one), so each `connect()` is a fresh connection and the ceiling is the host's. If the raised number does not hold, AC-006's branch B is the answer — amend both artefacts and record the verification that rejected 64 — never a silent partial raise. |
| EC-007 | A racing rule fails and the temptation is to compare positions against literals to "see what happened". | Forbidden. The specification permits gaps, `AUTOINCREMENT` produces them after a delete, and a rule or a debugging assertion written against `[1, 2, 3]` is wrong against a conformant store (`CLAUDE.md`, *The rule that matters*). Compare against the positions the store actually assigned. |

## Non-functional

| id | requirement | how it is held |
| --- | --- | --- |
| NF-001 | The family completes well inside the CI job's timeout at the settled `CONTENDERS`, because the job timeout is the only liveness mechanism there is. | Wall-clock of `cargo test -p happenstance-sqlite --test concurrency` recorded once in the ledger as a human observation. CF-33 forbids a **rule** reading a clock; it does not forbid an implementer timing a command and writing the number down, and that distinction is the one to keep. |
| NF-002 | The run's resource claim is stated, not discovered: `CONTENDERS` + 1 connections per racing rule instance, each a real `rusqlite::Connection` on one file, with no pool. | AC-006's verification is the whole-workspace run at the settled size; EC-006 names the failure mode. |
| NF-003 | Nothing this story adds costs the `!Send` flavour or the `wasm32` target anything — the project's DR-08. | The family stays opt-in behind `F::Store: EventStore + Send`, the mount carries `#![cfg(not(target_arch = "wasm32"))]`, and `cargo xtask wasm` is part of AC-007's verification. |
| NF-004 | No new dependency is added to `happenstance-core` or `happenstance-testkit`, and no `serde` anywhere near the contract crate. | PR boundary above; `CLAUDE.md` binding constraint 2. `tokio` is already a dependency of `happenstance-sqlite` (`crates/happenstance-sqlite/src/event_store.rs:73-74`). |
| NF-005 | The suite stays deterministic — green means green, and a re-run is not a strategy. | The rules assert *set* properties that hold at any size above one (`crates/happenstance-testkit/src/concurrency.rs:202-206`); no clock, no sleep, no ordering assumption is introduced by the mount or by the adapter-local tests. A flaky rule here is a finding to escalate, because a flaky conformance suite teaches adapter authors to re-run until green (`:29-37`). |
| NF-006 | The diff is legible as one decision per hunk: the seam, the mount, the count. | The PR boundary's three "In this PR" bullets; the ledger cites each separately. |

## Implementation notes (non-prescriptive)

Guidance, not instruction — ADR-0022 outranks every sentence here, and where the
two disagree the ADR wins and this paragraph is the one that was wrong.

- **Do the seam first, then mount.** The mount is four lines; the seam is the
  work. Landing `tests/concurrency.rs` before the store carries a handle produces
  five red rules that all say the same uninformative thing, and the temptation
  then is to debug the *rules*. Getting
  `store_serves_a_bare_thread_with_no_ambient_runtime` green first turns the
  family's first run into a real signal.
- **Read ADR-0022's seam paragraph before writing a line.** The architecture
  brief recommends option (a) — capture a `Handle` at construction, prefer it,
  keep `try_current()` as the fallback — and the ADR records what actually won.
  Option (b) is a *larger* change than it looks: it deletes a variant, a
  module-doc paragraph and `SqliteProjectionStoreError::NoRuntime`, and leaving
  any of those behind ships documentation for an unreachable state.
- **`Handle` is `Clone + Send + Sync + Unpin`**, which is why it can sit in the
  store, in `ReadCursor` and in `SqliteReadStream` without disturbing
  `tests/shapes.rs`. If a shape assertion goes red, the field that was added is
  not a `Handle` and something else changed.
- **The fixture relocation is mechanical but load-bearing.** Cargo compiles each
  file directly under `tests/` as its own binary, so `tests/support.rs` would
  become a third test target; `tests/support/mod.rs` with `mod support;` in both
  targets is the shape that does not.
- **Take the contender-count branch deliberately, in one commit, with the
  workspace re-run.** Branch A's cost is real and is spread across
  `MemoryFixture`, the five racing mutant stores and every future fixture; branch
  B's cost is two RUNBOOK edits plus reconciling the *"25 rounds"* phrase that
  matches nothing in the code. Do not raise the constant "to see", leave it
  raised, and discover the cost in review.
- **If a rule hangs, stop and measure rather than instrument.** The busy timeout
  and the `BEGIN IMMEDIATE` lock hold are the two candidates, and both belong in
  ADR-0022's paragraph. Adding a clock is the one response the specification
  forbids outright.
- **`MemoryFixture` is the reference implementation to read first**
  (`crates/happenstance-testkit/src/fixtures.rs`), and
  `crates/happenstance-testkit/tests/memory_concurrency_conformance.rs` is the
  mount to copy — minus its second, blocking-emitter invocation, per EC-005.

## Tests and CI (merge gate)

Tiers as the project's testing brief defines them (§1, §6); every row is a real
command against a real target. No tier here doubles `rusqlite`, the filesystem or
the runtime — testing brief §3, AC-T03.

| tier | command / path | proves |
| --- | --- | --- |
| Static | `cargo xtask lints && cargo xtask spec-trace` (`reachability_static`, `.redkiln/config.yaml:48`) | No clock was added to a conformance rule (CF-33's lint step over `happenstance-testkit/src`), and no clause citation was broken by the RUNBOOK or doc-comment edits AC-006 may make. |
| Unit / type-level | `cargo test -p happenstance-sqlite --test shapes` → `crates/happenstance-sqlite/tests/shapes.rs` | AC-007. The `Handle` field cost the shape assertions nothing: `SqliteReadStream: Send + Unpin`, `SqliteEventStore: Send + Sync`, the error still `Send + Sync + 'static`, and `read` still returns the named stream type at the top level. |
| Adapter-local behavioural | `cargo test -p happenstance-sqlite --test concurrency` → `store_serves_a_bare_thread_with_no_ambient_runtime`, `every_connection_carries_the_declared_busy_timeout` | AC-002 and AC-003 *before* the family's verdict depends on them — the two failure modes that otherwise arrive disguised as atomicity findings. |
| Conformance (this story's real proof) | `cargo test -p happenstance-sqlite --test concurrency` → `crates/happenstance-sqlite/tests/concurrency.rs`, generated module `dcb_concurrency_conformance` | AC-001, AC-004, AC-005. Five rules, all `Ran`: exactly one winner, k disjoint boundaries admit exactly k, positions distinct, each caller's own last position returned, no partial batch observed by a concurrent reader. |
| Falsifiability (the check on the checker) | `cargo test -p happenstance-testkit --test mutation_coverage` → `the_concurrency_rules_reject_exactly_what_they_claim` (`crates/happenstance-testkit/tests/mutation_coverage.rs:3407`) | That the rules AC-004 relies on can still fail at the settled `CONTENDERS` — five racing stores, each wrong in one way, with their verdicts pinned. The permanent `BEGIN DEFERRED` row is `model-family-and-mutant-pass-column`'s, not this story's. |
| Regression (the count's blast radius) | `cargo test --workspace --all-features`, including `crates/happenstance-testkit/tests/memory_concurrency_conformance.rs` | AC-006 branch A's cost is verified rather than assumed: every existing fixture and mutant re-runs green at the new size. Under branch B this run is what the amendment's recorded verification is drawn from. |
| Story gate | `cargo xtask affected --base main` (`affected_gate`, `.redkiln/config.yaml:40`) | The merge bar for this story: fmt, clippy `-D warnings` and the tests for the affected packages and their dependents — which is `happenstance-sqlite` plus `happenstance-testkit` and everything downstream of it once `CONTENDERS` moves. |
| Target guard | `cargo xtask wasm` | NF-003. The family does not exist on `wasm32` and the mount's `cfg` keeps it that way; the gate's four wasm32 steps are unmoved. |
| Project integration grain (not run per story) | `cargo xtask ci --fast` (`integration_scoped`, `.redkiln/config.yaml:55`) | The project's own bar, which this story must not break; `instrument-markers-removed-and-gate-green` owns taking it green. |

**Not in any gate**: `event_store_benchmarks!` (CF-34 — a benchmark is never a
conformance rule, and this story adds it to no script), and the initiative-wide
`cargo xtask ci`, which is `closeout-and-durable-audience`'s.

## Risks and coupling (PR-scoped)

- **ADR-0022 is a hard predecessor, and this story cannot invent its answers.**
  The seam option, the busy-timeout value, the journal and `synchronous` settings
  and the `CONTENDERS` resolution all come from it. If the ADR is silent on the
  busy timeout when this story starts, that is a blocking gap to escalate to the
  runbook's ADR queue — **not** a number to pick in an adapter and document after
  the fact.
- **The half-done seam is the silent failure.** `append` and `read` are two
  different call paths and the reader's failure is reported as a sighting rather
  than an error. A seam applied to `append` only produces a family that is four
  rules green and one rule wrong *about atomicity*, which reads as an adapter bug.
  AC-002's dedicated test exists because the family's own output cannot tell you
  this.
- **Raising `CONTENDERS` is a workspace-wide edit inside another crate.** It
  touches `happenstance-testkit`, which every adapter and every mutant store
  depends on. The PR boundary admits it deliberately, but a raise that is not
  accompanied by a whole-workspace green run has moved the cost onto whoever
  merges next.
- **A hang costs more than a red run.** With no watchdog anywhere, a deadlock
  hangs the binary and only the CI job timeout notices — a cost `RUNBOOK.md`
  states is to be *chosen* at this phase rather than discovered. `BEGIN IMMEDIATE`
  with a lock held to commit is precisely where it lives, and an infinite busy
  handler is how it gets in.
- **Coupling to `sqlite-fixture-and-whole-suite`.** This story depends on
  `SqliteFixture` existing and on its `connect()` opening a genuinely second
  connection. If that story left the fixture inline in `tests/conformance.rs`,
  relocating it is in scope; if it left `connect()` returning a clone of one
  store, this story's rules will pass for the wrong reason and the finding belongs
  upstream, not here.
- **Coupling to `model-family-and-mutant-pass-column`** (its dependent, same
  slice): it adds the `BEGIN DEFERRED` registry row that makes AC-004's rejection
  permanent. Do not pull that row forward into this PR to make the falsifiability
  story feel complete — the two are mounted as one integrated surface at slice
  review, which is where completeness is judged.
- **Schedule pressure.** This project is the longest single adapter on the trunk
  and `publication-and-positioning` is blocked behind it. The specific temptation
  here is to declare the family green with a capability quietly declined, or with
  `CONTENDERS` left at 8 and the discrepancy re-deferred. AC-001 and AC-006 are
  written to make both of those a red ledger row rather than a judgement call.

## Dependencies

**Blocks on**

- `sqlite-fixture-and-whole-suite` — supplies `SqliteFixture` (one instance is one
  fresh temp file, each `connect()` a second `rusqlite::Connection` onto it,
  `SECOND_HANDLE` and `REOPEN` available, `MID_BATCH_FAULT` declined with a
  reason) and the first green `event_store_conformance!` this story must not
  break. This is the story's only declared `depends_on`.

Transitively upstream through that story, and load-bearing enough to name:
`adr-0022-append-condition-strategy` (the seam, the busy timeout, the journal and
`synchronous` settings, the `CONTENDERS` resolution) →
`schema-migration-and-identity` (migration 1, WAL, the stated `synchronous`, the
finite busy timeout on the first connection) → `append-atomicity-and-store-limits`
(the one-`BEGIN IMMEDIATE` append these rules race against).

**Unlocks**

- `model-family-and-mutant-pass-column` — declares this story as its `depends_on`;
  it takes `event_store_model_conformance!` green and adds the `BEGIN DEFERRED`
  probe-then-insert row to the mutation registry.
- `instrument-markers-removed-and-gate-green` — indirectly, via the above; the
  project's `--fast` gate cannot be claimed green while a declared family is
  unmounted.

**Parallel, same slice, no edge either way**:
`reopen-negative-control-and-durability-verdicts`.

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. Every path below exists in this
worktree; open each at the moment named, not before.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md` | The architecture brief §3 is the full argument for the runtime seam, including both options and why (a) is recommended; §6 is the write path and the `SQLITE_BUSY` reasoning; §9 is the `CONTENDERS` tension with its cost analysis; §10 is the testkit blast radius. The testing brief §4 restates the seam and CF-33 as testing obligations and §6 lists the merge-gate commands by grain. | Before the first line of the seam, and again before taking the `CONTENDERS` branch. | AC-002, AC-003, AC-006 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/adr-0022-append-condition-strategy/spec.md` | The predecessor that authors ADR-0022 — its acceptance criteria state exactly what the decision record is obliged to carry, so it tells you what to expect to find in the atom and what to escalate if it is missing. | If ADR-0022 is silent on the seam, the busy timeout or the contender count when implementation starts. | AC-002, AC-003, AC-006 |
| `crates/happenstance-testkit/src/concurrency.rs` | The whole family: the module doc's CF-33 argument (`:24-43`), why contenders are OS threads and the reactor limitation this story is the first to hit (`:45-68`), `ConcurrentFixture` (`:186`), `CONTENDERS` and its doc comment (`:195-206`), `Attempt::of`'s Rejected-versus-Failed mapping (`:224-233`), the five rules (`:347`, `:436`, `:566`, `:637`, `:735`), the two emitters (`:1070`, `:1093`) and the mount macro (`:1129`). | Before writing the mount, and again before any debugging of a red rule. | AC-001, AC-003, AC-004, AC-005, AC-006 |
| `crates/happenstance-testkit/tests/memory_concurrency_conformance.rs` | The mount pattern the macro's own documentation names as the one to copy, including the load-bearing `#![cfg(not(target_arch = "wasm32"))]` — and its second, blocking-emitter invocation, which is exactly what this adapter must **not** copy (EC-005). | When creating `crates/happenstance-sqlite/tests/concurrency.rs`. | AC-001 |
| `crates/happenstance-testkit/src/contract.rs` | `Fixture`, `Capability` and `RuleOutcome`; `SECOND_HANDLE`'s MUST status and the panic-rather-than-skip enforcement (`:135-161`); `REOPEN` and `MID_BATCH_FAULT`'s default declension with its reason. | Before touching any capability constant on `SqliteFixture`. | AC-001, AC-007 |
| `crates/happenstance-sqlite/src/event_store.rs` | The module doc's `spawn_blocking` / `NoRuntime` reasoning (`:14`, `:26-32`), the constructors a handle would be captured in (`:108-132`), the `NoRuntime` variant (`:176-178`), and the `Handle::try_current()` call inside `poll_next` (`:390-394`) that is the actual site of the change. | Immediately before implementing the seam. | AC-002 |
| `crates/happenstance-sqlite/tests/shapes.rs` | The type-level guard that catches a field added carelessly — and the file that tells you instantly whether what you added is really a `Handle`. | Right after adding the field, before running anything expensive. | AC-007 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | `the_concurrency_rules_reject_exactly_what_they_claim` (`:3407`) is the five racing mutant stores whose verdicts are pinned; it is both AC-004's falsifiability evidence and the largest single cost of raising `CONTENDERS`. | Before raising the constant, and again to read AC-004's evidence. | AC-004, AC-006 |
| `spec/SPECIFICATION.md` | CF-33 `[FROZEN]` at `:8236` — the no-clock clause and its `Rejects:` paragraph; CF-14 at `:7480-7487`, which names `PRAGMA synchronous = OFF` by name as a wrong implementation, so the durability settings may not be weakened to make a race faster. | When tempted to add a timeout, and before touching any pragma. | AC-003, AC-005 |
| `RUNBOOK.md` | The discrepancy stated in full at `:2686-2696` — including *"leaving it is the third option and it is the one that rots"*; the two proof artefacts at `:159` and `:4215-4222` that branch B must amend; phase 8 in full at `:4166-4237`. | Before choosing the `CONTENDERS` branch; again if branch B is taken, to edit both places. | AC-006 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/project.md` | Project AC-005 and AC-007 in their own words, the risk list this story inherits (the harness-cannot-drive-the-read-path risk, the no-watchdog risk), and the *Out of scope* line that forbids amending anything `[FROZEN]`. | When the traceability of a ledger row is in question, or when a frozen clause looks wrong. | AC-004, AC-006 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/sqlite-fixture-and-whole-suite/spec.md` | The predecessor's own contract for `SqliteFixture` — where it lives, what `connect()` does, which capabilities it declares and with what reasons. Determines whether the relocation to `tests/support/mod.rs` is needed and whether a green rule here is green for the right reason. | Before mounting the second target. | AC-001, AC-007 |
| `.kb/decisions/0001-async-port-flavours.md` | Why there is no `#[async_trait]` and why the `Send` flavour is a derived trait — the reason the family is opt-in behind `F::Store: EventStore + Send` and the reason a `!Send` adapter is excluded rather than failed. | If the bound ever looks like the thing to change. | AC-007 |
| `.kb/decisions/0008-one-derivation-for-both-ports.md` | Why `EventStore::read` returns the stream at the top level and is not `async` — which is why the read path needs a runtime handle at `poll_next` time and cannot simply be awaited into one. | While implementing the seam on the read half. | AC-002, AC-005 |
| `.kb/decisions/0012-append-shape-and-preconditions.md` | The append contract these racing rules are asserting: one decision, one transaction, the caller's own last position returned. | When a racing rule's expectation looks wrong. | AC-004 |
| `.kb/decisions/0013-position-assignment-and-visibility.md` | Position assignment and the visibility invariant — the reason `positions_are_unique_under_concurrent_appends` is a contract statement and not a stress test, and the reason literal positions may never be asserted. | Before debugging a position-shaped failure. | AC-004 |
| `.kb/decisions/0011-read-laziness-and-isolation.md` | The snapshot promise the concurrent reader depends on: one state sampled no later than the first poll. It is what makes "never a partial batch" a checkable claim during a live race. | Before reading `a_concurrent_reader_never_sees_a_partial_batch`'s failure output. | AC-005 |
| `crates/happenstance-testkit/src/fixtures.rs` | `MemoryFixture`, the reference implementation — and the fixture that must still pass at whatever `CONTENDERS` becomes. | Before AC-006's whole-workspace re-run. | AC-006 |
| `.redkiln/config.yaml` | The verify grains this story is measured by: `affected_gate` (`:40`), `reachability_static` (`:48`), `integration_scoped` (`:55`), `require_ledger` (`:67`). | When assembling the merge evidence. | AC-006, AC-007 |

## Clarifications resolved during spec

1. **The AC namespace collision is deliberate and is resolved here.** The
   Executive summary and Context pack above refer to "AC-005" and "AC-007"
   meaning `project.md`'s criteria, because they were written against the project
   spine. This story's own criteria are `AC-001` – `AC-007` in the table above and
   are the ids `_ledger.md` carries. The front half is left verbatim; the mapping
   is stated at the head of the acceptance-criteria table. Nothing was added to or
   dropped from the front half's enumerated set.
2. **The mount uses the tokio emitter only.** The testkit's own memory harness
   invokes the macro twice — once per shipped emitter — and copying that verbatim
   would be the natural move. It is wrong here: `__emit_concurrency_blocking`
   generates a plain `#[test]`, so no tokio runtime exists on any thread including
   the constructing one, and a handle captured at construction is `None`. Recorded
   as EC-005 rather than left for an implementer to rediscover in five red rules.
3. **Falsifiability for AC-004 is the existing pinned mutant verdicts, not a new
   registry row.** `the_concurrency_rules_reject_exactly_what_they_claim` already
   drives five racing stores that are each wrong in one way. The `BEGIN DEFERRED`
   probe-then-insert row is `model-family-and-mutant-pass-column`'s deliverable
   and is explicitly excluded from this PR, so this story cites the existing
   evidence rather than pulling its dependent's work forward.
4. **The busy-timeout obligation is "on every connection", not "configured".**
   `schema-migration-and-identity` already lands WAL, a stated `synchronous` and a
   finite busy timeout; what is new here is that they must apply to the second,
   third and Nth connection `SqliteFixture::connect()` opens. That is why AC-003's
   test reads the pragma back from a later handle rather than asserting the
   constructor was called.
5. **Timing a command is not a rule reading a clock.** NF-001 asks for a
   wall-clock figure in the ledger, which looks adjacent to CF-33. CF-33 binds
   *conformance rules*; a human recording how long a `cargo test` invocation took
   is not one, and the adapter's own busy handler is not one either. The
   distinction is written into both the interaction-quality table and EC-003
   because it is the one an implementer under pressure collapses in the wrong
   direction.
6. **No user-facing surface, and the composition family is genuinely absent.**
   `_design.md` records the no-surface determination as the thing that was signed
   off. The Interaction quality section therefore carries the STATE family only,
   mapped onto the conformance run as the adapter author's surface, and says so
   rather than fabricating chrome or a density budget for a library test target.
7. **`RUNBOOK.md:4222`'s exit criterion "`publish = false` removed" is not this
   story's.** It appears inside the same phase-8 block AC-006 may amend under
   branch B. Editing it here would collide with
   `crates-io-name-and-packaging-facts` and with architecture brief AC-A05; branch
   B touches `RUNBOOK.md:159` and the proof-artefact paragraph at `:4217-4222`
   only.
