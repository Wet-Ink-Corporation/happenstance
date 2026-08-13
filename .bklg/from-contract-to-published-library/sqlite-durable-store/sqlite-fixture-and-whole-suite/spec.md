---
item: HS-S0040
stage: spec
created: 2026-08-12T13:46:38.591Z
updated: 2026-08-12T13:46:38.591Z
template_sig: 87bbf1d0
rendered_sig: 8ebdff19
---

# Spec — SqliteFixture, and the first green conformance run against a file

## Scope lock

| What | Path |
| --- | --- |
| Initiative | [`.bklg/from-contract-to-published-library/initiative.md`](../../initiative.md) — BR-02, BR-03, BR-13, DoD 3 |
| Project | [`.bklg/from-contract-to-published-library/sqlite-durable-store/project.md`](../project.md) — AC-001, AC-002, AC-003, AC-004, AC-009; DR-03, DR-04, DR-05 |
| This spec | `.bklg/from-contract-to-published-library/sqlite-durable-store/sqlite-fixture-and-whole-suite/spec.md` |
| Story card (frontmatter only) | [`story.md`](story.md) — `HS-S0040`, `archetype: capability`, `slice: durable-event-store` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — architecture §1 (the mount table and the three integration rules), §7 (migration under a concurrent open), §9 (`MID_BATCH_FAULT` and ES-35's residual falsifier), §11 (the ceiling numbers and the temp-file question); testing §1–§3 (tiers, AC→proof, what may not be doubled), §4 (no watchdog), §5 (the missing negative control) |
| Signed-off design | [`../_design.md`](../_design.md) — **no user-facing surface**; `## Items`, `## Signatures` and `## The doctest` are all recorded `N/A`, approved 2026-08-12. This story therefore renders no surface and claims no `path` id. |
| Story map row | [`../_storymap.md`](../_storymap.md) — slice `durable-event-store`, row `sqlite-fixture-and-whole-suite` |
| Roadmap pointer | `RUNBOOK.md:4166-4237` (phase 8 in full); `RUNBOOK.md:691-692` (the two empty instrument axes this story fills) |

## One-line PR slice

Mount `SqliteFixture` in `crates/happenstance-sqlite/tests/conformance.rs` — one instance is one fresh temp file, each `connect()` a second `rusqlite::Connection` onto it — declaring `SECOND_HANDLE`, `REOPEN` and real ceilings, declining `MID_BATCH_FAULT` with its reason, and take `event_store_conformance!` green whole.

## Executive summary

Two predecessor stories put real SQL behind `append` and `read`
(`append-atomicity-and-store-limits`, `wide-query-chunked-not-refused`, over
`schema-migration-and-identity`'s migration 1). Nothing has yet **run** them
against the bar. This PR builds the composition root that does: a `Fixture`
implementation living in the adapter's own `tests/` tree, and the single macro
invocation that turns every entry in `for_each_event_store_rule!` into a
`#[tokio::test]` driven against a real file on disk.

The delta over the project charter is not "the suite passes" — that is the
charter's AC-001. It is **what the fixture is made of**, because the fixture is
where three of the five project ACs this story traces to are won or silently
lost:

- `connect()` returning a `Clone` of one in-process store passes
  `two_handles_observe_each_others_appends` and fails project AC-003. The rule
  cannot tell; only the fixture's body can (`../project.md`, AC-003).
- Pointing every instance at one path fails `two_fixture_instances_observe_none_of_each_others_appends`,
  and that rule exists precisely because it is an *adapter's* mistake no
  testkit meta-test can see (`crates/happenstance-testkit/src/contract.rs:17-23`).
- Leaving the three ceilings at their `None` defaults produces a *green* suite
  in which `append_reports_exceeded_store_limits` reports a skip — a fact-shaped
  skip that is indistinguishable from a pass to anyone reading only the exit
  code (`contract.rs:213-253`).

So this story's real deliverable is a fixture that means what `Fixture` says,
plus the first green whole-suite run in the workspace against durable storage,
plus the honest accounting of the one capability it declines.

## Context pack

Read this and you can start. Everything deeper is a signposted anchor.

**The mount is a test target, not a module.** An adapter that compiles is not an
adapter (`CLAUDE.md`, *The rule that matters*). The composition root is
`crates/happenstance-sqlite/tests/conformance.rs`, a new integration target, and
its whole content is `SqliteFixture` plus
`happenstance_testkit::event_store_conformance!(SqliteFixture::new());`. The
one-argument arm expands to `mod dcb_conformance` and emits one
`#[tokio::test]` per rule through `__emit_tokio`
(`crates/happenstance-testkit/src/lib.rs:312-357`). `crates/happenstance-testkit/tests/memory_conformance.rs`
is the shape to copy for the mount, and `MemoryFixture` is the shape **not** to
copy for the fixture.

**The expression builds a `Fixture`, not a store, and the two operations are
named apart on purpose.** One fixture instance is one isolated backing store;
each `connect()` is one handle onto *that* store; two instances share nothing
(`contract.rs:13-15`). Concretely here: `SqliteFixture::new()` mints one fresh
temp file path and nothing else; `connect()` calls
`SqliteEventStore::open(path)` (`crates/happenstance-sqlite/src/event_store.rs:120`),
which is a *new* `rusqlite::Connection`. **`SqliteEventStore::open_in_memory()`
(`event_store.rs:132`) is the wrong constructor and it is the one an implementer
reaches for first** — a private in-memory database is per-*connection*, so the
second `connect()` would open a second empty database and
`two_handles_observe_each_others_appends`, which is a MUST that panics rather
than skips, would fail (architecture brief §1, third bullet).

**`SECOND_HANDLE` is the one capability that is a MUST, not a trade.** CF-16
(`spec/SPECIFICATION.md:7548`) requires it, and
`two_handles_observe_each_others_appends` *panics* on a declined one, quoting the
fixture's own words, rather than reporting a skip
(`contract.rs:135-161`). Declaring it `SUPPORTED` here is not paperwork: it is
the claim that this adapter has the handle-multiplicity far end
`RUNBOOK.md:691` says nothing in the workspace has ever had — *"every fixture
still hands out refcount clones of one in-process object, so no connection has
been opened twice."*

**`REOPEN` is the durability far end, and it must discard process state without
discarding the medium.** `acknowledged_writes_survive_a_reopen`
(`crates/happenstance-testkit/src/suite.rs:332-360`) appends two events through
one handle, calls `fixture.reopen()`, connects again, and requires both events
*and* the exact position `append` returned to still be there. Handles taken
before the call may stop working (`contract.rs:323-328`), so `reopen()` drops
the live connections and leaves the file. This is the axis `RUNBOOK.md:692`
records as empty. It also makes `recorded_time_survives_a_reopen` and
`reopened_store_does_not_reissue_an_event_id` askable — both of which depend on
the predecessor story having persisted the `StoreId` once at schema creation
rather than minting one per open (architecture brief §7; the testkit's own
`DurableFixture` at `crates/happenstance-testkit/tests/fixture_instruments.rs:75-90`
switched to mint-once for exactly this reason).

**`MID_BATCH_FAULT` is declined here, explicitly and in this fixture's own
words.** The default on the trait is already a decline
(`contract.rs:207-211`), so inheriting it would still be green — and would put a
generic sentence in this adapter's CI log where a specific one belongs.
Architecture brief §9 is explicit: this adapter supplies the *reopen* far end and
does **not** supply the *fault* far end, and ES-35's `[PROVISIONAL]` marker keeps
a live falsifier — "a store that loses a write to a fault rather than to an
instruction" — that this project cannot retire. Writing the real reason is what
keeps that honest. The two rules it gates,
`append_is_atomic_under_a_mid_batch_fault` and
`arming_a_mid_batch_fault_makes_the_append_fail`, still appear in the binary and
still print a `SKIP` line, which is CF-18's whole argument
(`spec/SPECIFICATION.md:7597`).

**Limits are facts, not trades — and stating them is what makes a rule run.**
`MAX_EVENT_DATA_LEN`, `MAX_TAGS_PER_EVENT` and `MAX_EVENTS_PER_BATCH` are
`Option<usize>`, deliberately not `Capability`, because a store either has a
ceiling or it does not and neither answer owes anybody a justification
(`contract.rs:44-54`). Stating a number commits this store to *both* directions:
`append_reports_exceeded_store_limits` appends exactly that many bytes and
requires acceptance, then one byte more and requires
`AppendError::ExceedsStoreLimit { limit: StoreLimit::EventDataLen, .. }` —
never `AppendError::Store`, never a truncation (`contract.rs:237-247`). Two hard
constraints on the numbers, both visible from here: they must sit **above** the
VT floors — 65,536 bytes (`crates/happenstance-core/src/limits.rs:22`), 64 tags
(`:27`), 128 events (`:43`) — and **far below** SQLite's own ~1 GB, because the
rule allocates a payload of exactly the stated size and a gigabyte makes the
suite unrunnable (architecture brief §11).

**CF-40's clause home is not this story's to settle.** This story needs the
*capability* — a fixture stating numeric ceilings — and gets no say in which ADR
owns the clause; ADR-0015 both claims and disclaims it and the KB records that as
open (`.kb/open-questions/cf-40-fixture-limits-ownership.md`). Record and
escalate; do not resolve in passing.

**Green by passing, not by silencing.** Three moves are available to make this
suite green and all three are failures of the story: `#[ignore]` on a rule,
declining a capability the store can actually do, and adding a timeout or
watchdog around a hang. The last is specifically forbidden — CF-33 keeps a clock
out of the suite and there is no watchdog anywhere in it, so a hang here is
evidence about the busy-timeout value the predecessor story set, not something a
test may paper over (`crates/happenstance-testkit/src/concurrency.rs:24-43`;
testing brief §4).

**The persona slice.** The user is an adapter author and the library's consumer,
and the only increment either can observe is the same one: *a store that has
passed the bar, against durable storage, through two real handles*
(`../_storymap.md`, preamble). This is that increment. It is not "SQLite works";
it is the first time in this workspace that the sentence "the suite is green"
refers to a file on disk.

## Integration contract

- **Archetype**: `capability` — the first user-observable increment of the slice,
  and the one every earlier story in it exists to make possible.
- **Slice / milestone**: `durable-event-store`. Slice-mates, implemented in one
  context and mounted as one integrated surface:
  `schema-migration-and-identity`, `append-atomicity-and-store-limits`,
  `lazy-read-with-snapshot-ceiling`, `wide-query-chunked-not-refused`, and this
  story, which closes the slice. Splitting "implement `append`" from "mount the
  fixture that runs it" would recreate exactly the compiles-but-never-ran failure
  this project exists to retire (`../_storymap.md`, *Slice coherence notes*).
- **Mount point**: `crates/happenstance-sqlite/tests/conformance.rs` — a **new**
  integration target, auto-discovered by `cargo test -p happenstance-sqlite`, and
  therefore reached by `cargo xtask ci --fast` and by
  `cargo xtask affected --base main` without any script edit. This is the file
  named in the architecture brief's §1 mount table.
- **Wires into**:
  - `crates/happenstance-testkit/src/contract.rs` — the `Fixture` trait,
    `Capability`, `RuleOutcome`, `NO_STORE_LIMITS` / `NO_CEILING_REASON`.
  - `crates/happenstance-testkit/src/lib.rs:312-357` — the
    `event_store_conformance!` macro and its `__emit_tokio` default.
  - `crates/happenstance-testkit/src/registry.rs:94` —
    `for_each_event_store_rule!`, the enumeration this run must cover whole.
  - `crates/happenstance-sqlite/src/event_store.rs` — `SqliteEventStore::open`
    (`:120`), the `EventStore` impl, and `SqliteEventStoreError`.
  - `crates/happenstance-sqlite/tests/shapes.rs` — the existing type-level guard;
    it must still pass unchanged in intent after the store and stream gained
    fields (architecture brief AC-A01).
  - `crates/happenstance-testkit/tests/memory_conformance.rs` and
    `src/fixtures.rs:243-292` — the reference mount to copy and the reference
    fixture *not* to copy.
- **Design-system primitives consumed**: none in the UI sense. The equivalent
  primitives here are the testkit's contract types above; the project's
  `_design.md` records **no user-facing surface** and no `## Items` block, so
  there is no `path` id to claim.
- **Renders surfaces**: **none.** `../_design.md` records `N/A — no user-facing
  surface` for every section, approved at the `/redkiln:plan` design sign-off
  gate. This story does not re-open that determination.
- **Conformance rule(s) observed**: the whole of `for_each_event_store_rule!`.
  The ones this story's own decisions decide, rather than merely run:
  `two_fixture_instances_observe_none_of_each_others_appends`,
  `two_handles_observe_each_others_appends`, `head_advances_across_two_handles`,
  `acknowledged_writes_survive_a_reopen`, `recorded_time_survives_a_reopen`,
  `reopened_store_does_not_reissue_an_event_id`,
  `append_reports_exceeded_store_limits`,
  `store_accepts_the_guaranteed_minimum_payload`,
  `store_accepts_the_guaranteed_minimum_tag_count`,
  `store_accepts_the_guaranteed_minimum_batch_size`,
  `append_is_atomic_under_a_mid_batch_fault` and
  `arming_a_mid_batch_fault_makes_the_append_fail` (the two declared skips).
- **Clause(s)**: discharges CF-15 (fixture is a trait; instances are isolated),
  CF-16 (`spec/SPECIFICATION.md:7548`, second handle), CF-17 (`:7570`, reopen),
  CF-18 (`:7597`, declared capabilities and reported skips) and CF-40 (`:7661`,
  stated ceilings) **against a real adapter for the first time**. It amends
  nothing. ES-35 (`:4142`) and CF-14 (`:7471`) are *exercised* here and their
  written verdicts belong to `reopen-negative-control-and-durability-verdicts`;
  CF-40's clause home stays open. No `[FROZEN]` clause is touched, so no new ADR
  is owed by this story.
- **Advances DoD scenario**: initiative **DoD 3** — *"The durable store passes
  the suite for real."* This story lands two of its three halves:
  `event_store_conformance!` green against the durable adapter's fixture, and an
  acknowledged write surviving a process reopen. The 64-contender half is
  `concurrency-family-and-contender-count`'s, which is blocked on this story.

## PR boundary

**In this PR**

- `crates/happenstance-sqlite/tests/conformance.rs` (new): `SqliteFixture` — its
  temp-file ownership and `Drop`, `connect`, `reopen`, the five associated
  constants — and the `event_store_conformance!` invocation.
- Whatever minimum is needed on the adapter's own side to let a *test* own the
  file lifecycle honestly (for example, a `pub` accessor for the path a store was
  opened at, if the fixture's `reopen` needs one). Any change here is
  additive and mechanical; behaviour changes to SQL belong to the predecessor
  stories.
- This story's backlog folder: `_ledger.md` and the report the second pass and the
  implement stage author.

**Explicitly not in this PR**

- **Any SQL body.** `append`, `read`, `head`, `contains_event_id`, migration 1
  and the wide-query chunking are `append-atomicity-and-store-limits`,
  `lazy-read-with-snapshot-ceiling`, `wide-query-chunked-not-refused` and
  `schema-migration-and-identity`. If a rule fails because the SQL is wrong, that
  is a red run against a predecessor's spec, not a licence to edit it here.
- **`event_store_model_conformance!` and `event_store_concurrency_conformance!`.**
  Both are `race-model-and-durability`'s (`../_storymap.md`), and both are
  blocked on this story. Adding either here imports the runtime-seam and
  `CONTENDERS` questions into a story that has no AC for them.
- **The negative control for `recorded_time_survives_a_reopen`** and the ES-35 /
  CF-14 / CF-17 written verdicts — `reopen-negative-control-and-durability-verdicts`
  (testing brief §5). This story makes the rule *runnable*; that story makes it
  *falsifiable*.
- **Anything in `crates/happenstance-testkit/`.** No rule is added, removed,
  reordered or reworded. The fixture goes in the adapter's `tests/`, never in
  `src/fixtures.rs`: the testkit is published, is built for `wasm32` by
  `cargo xtask wasm`, and depends today only on `happenstance-core` and
  `futures-core` — a `rusqlite`-backed fixture there would put a bundled C
  library into the dependency graph of the crate whose whole value is needing
  nothing (architecture brief §1, first bullet).
- **`publish = false`, `xtask/src/package.rs`'s `PUBLISHABLE`, the crate's
  README and licence files, the `#![allow(clippy::todo)]`, and the module-doc
  schema sketch** — `crates-io-name-and-packaging-facts` and
  `instrument-markers-removed-and-gate-green` (architecture brief §8).
- **A new workspace dependency.** `tempfile` is not in
  `[workspace.dependencies]`, and adding one is a manifest change `cargo deny`
  sees (architecture brief §11). See *Data and migrations*.

**Merge DoD (one line):** `cargo test -p happenstance-sqlite --test conformance`
is green with every rule in `for_each_event_store_rule!` reporting `Ran` or a
`SKIP` line naming this fixture's own reason, and
`cargo xtask affected --base main` is green.

The implementer may touch the wiring files named in the *Integration contract*
to mount this slice; that is not scope drift.

```
crates/happenstance-sqlite/tests/**
crates/happenstance-sqlite/src/event_store.rs
crates/happenstance-sqlite/Cargo.toml
.bklg/from-contract-to-published-library/sqlite-durable-store/sqlite-fixture-and-whole-suite/**
```

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| `SqliteFixture::new()` mints one fresh backing store | One instance owns **one** temp file path, created fresh and used by nothing else. Two instances get two paths. This is the single decision project AC-002 turns on, and the wrong implementation — every instance pointed at one directory — is named in the trait's own docs as the adapter mistake the rule exists to catch. | `crates/happenstance-testkit/src/contract.rs:17-23`; rule body at `crates/happenstance-testkit/src/suite.rs:210-238` |
| `Fixture::connect()` opens a **new** `rusqlite::Connection` | `connect()` calls `SqliteEventStore::open(path)`. Not an `Arc`/`Rc` clone of one store, and not `open_in_memory()` — a private in-memory database is per-connection, so a second `connect()` would open a second *empty* database and fail a MUST. The fixture's body is the evidence for project AC-003, because a `Clone`-shaped fixture passes the rule and fails the criterion. | `crates/happenstance-sqlite/src/event_store.rs:120`, `:132`; `../_decomposition.md` architecture §1; `../project.md` AC-003 |
| `type Store` is owned, and there is one flavour of `Fixture` | `Fixture` has no `Send` flavour and no GAT: the handle owns its connection outright rather than borrowing the fixture's lifetime. That is not a style choice — `where Self: 'a` on a GAT implemented for a foreign trait is one ingredient of a rustc ICE this repository has already minimised and which still reproduces on 1.97.1. | `crates/happenstance-testkit/src/contract.rs:76-111`; `experiments/rustc-ice-gat-foreign-trait/` |
| `SECOND_HANDLE = Capability::SUPPORTED` | A MUST, and the only one. The rule panics on a decline rather than skipping, quoting the fixture's reason — so declaring it is a claim, not a formality. It buys the handle-multiplicity far end. | `contract.rs:126-161`; `spec/SPECIFICATION.md:7548` (CF-16); `RUNBOOK.md:691` |
| `REOPEN = Capability::SUPPORTED`, and `reopen()` discards process state only | Drops every live handle's connection and leaves the file. After it returns, a fresh `connect()` observes exactly what was durably committed and nothing else; handles taken before may stop working, and the rules drop theirs. Two events appended through one handle, plus the exact position `append` returned, must survive. | `contract.rs:163-173`, `:323-352`; `crates/happenstance-testkit/src/suite.rs:332-360`; `RUNBOOK.md:692` |
| `MID_BATCH_FAULT` declined **explicitly**, in this adapter's words | The trait default is already a decline, so this is about the sentence, not the outcome: the reason is printed on every run and is the only record of the trade. It states that this adapter supplies the reopen far end and not the fault far end, which is also ES-35's residual falsifier. Both gated rules stay in the binary and print `SKIP`. | `contract.rs:176-211`, `:355-419`; `../_decomposition.md` architecture §9; `spec/SPECIFICATION.md:4142` (ES-35), `:7597` (CF-18) |
| Three ceilings stated as numbers, above the VT floors and below SQLite's | `MAX_EVENT_DATA_LEN`, `MAX_TAGS_PER_EVENT`, `MAX_EVENTS_PER_BATCH`. Each must be **≥** its floor — 65,536 / 64 / 128 — and small enough that the rule can allocate a payload of exactly that size. Stating a number commits the store in both directions: accept at the limit, `ExceedsStoreLimit` one byte past it, never `AppendError::Store` and never a truncation. The enforcement lives in `append` (predecessor story); the *declaration* is here. | `contract.rs:213-279`; `crates/happenstance-core/src/limits.rs:22`, `:27`, `:43`; `spec/SPECIFICATION.md:1483` (VT-21), `:1556` (VT-24), `:7661` (CF-40) |
| Leaving a ceiling `None` is a green suite with a hidden hole | With all three `None`, `append_reports_exceeded_store_limits` reports `Skipped { capability: NO_STORE_LIMITS, reason: NO_CEILING_REASON }` — a fact-shaped skip, not a pass. Project AC-009 requires the rule to **run**. | `contract.rs:228-235`, `:435-456`; `../project.md` AC-009 |
| The mount is one macro call, on the default arm | `happenstance_testkit::event_store_conformance!(SqliteFixture::new());` expands to `mod dcb_conformance` and emits one `#[tokio::test]` per rule via `__emit_tokio`. The adapter already has `tokio` with `macros`, `rt` and `rt-multi-thread` in `[dev-dependencies]`, which is what the emitter needs. | `crates/happenstance-testkit/src/lib.rs:312-357`, `registry.rs:229`; `crates/happenstance-sqlite/Cargo.toml` (`[dev-dependencies]`); reference mount at `crates/happenstance-testkit/tests/memory_conformance.rs` |
| Every rule appears in the run — `Ran` or `Skipped`, never absent | The suite reports skips rather than `#[cfg]`-ing rules out, because a rule omitted from the binary is indistinguishable in CI output from one that passed. The run is inspected with `--show-output`/`--nocapture`, since libtest suppresses a passing test's stdout. | `contract.rs:31-37`, `:509-536`; `crates/happenstance-testkit/src/registry.rs:94-218` |
| No watchdog, no `#[ignore]`, no retry loop around a rule | CF-33 keeps a clock out of the suite and there is no watchdog anywhere in it. A hang is a finding about the busy-timeout the predecessor story configured — evidence for ADR-0022's paragraph — not something this target may bound. | `crates/happenstance-testkit/src/concurrency.rs:24-43`; `../_decomposition.md` testing §4 |
| No rule asserts on literal positions, and this story adds none | The specification permits gaps and `AUTOINCREMENT` produces them after a delete. This story writes no rule at all; it is called out because the temptation at a red run is to "fix" a rule rather than the adapter. | `CLAUDE.md`, *The rule that matters*; `../project.md`, risk on gapped positions |
| `tests/shapes.rs` still passes | The type-level guard on `SqliteReadStream: Send + Unpin`, `SqliteEventStore: Send + Sync` and the error bounds. It is the cheap tripwire for a field added carelessly by a predecessor story, and it must be green before the conformance target is worth reading. | `crates/happenstance-sqlite/tests/shapes.rs`; `../_decomposition.md` architecture AC-A01 |

## Data and migrations

**No schema change, and a real on-disk footprint.** Migration 1 — the `event`,
`event_tag`, `tag_cardinality` and store-identity tables — belongs entirely to
`schema-migration-and-identity`. This story adds no column, no index and no
migration step, and must not amend one to make a rule pass.

What it *does* own is the file lifecycle, and there are four consequences worth
deciding before writing the fixture rather than discovering them in a red run.

1. **The temp file, without a new workspace dependency.** `tempfile` is not in
   `[workspace.dependencies]`, so reaching for it is a workspace manifest change
   that `cargo deny` will see on a gate that runs it. A process-local ordinal
   plus a `Drop` cleanup needs no dependency and has precedent in the testkit's
   own `crates/happenstance-testkit/tests/fixture_instruments.rs:94-102`. Either
   answer is acceptable; the manifest consequence is not optional to think about
   (architecture brief §11).

2. **WAL leaves sidecars.** The predecessor story sets journal mode to WAL for
   two-connection concurrency, which means the backing store on disk is
   `<name>.db` **plus** `<name>.db-wal` and `<name>.db-shm`. A `Drop` that
   removes only the first leaves two files per fixture instance behind, once per
   rule, on every run. Remove the set, or contain the whole fixture in its own
   directory and remove that.

3. **`reopen()` must drop connections, not delete data.** Deleting and recreating
   the file would make `acknowledged_writes_survive_a_reopen` fail correctly, and
   a `reopen()` that does nothing at all would make it pass *vacuously* — which is
   precisely why `MemoryFixture` declines the capability rather than faking it
   (`crates/happenstance-testkit/src/fixtures.rs`, the `REOPEN` decline). The
   honest implementation drops every handle it is holding so the OS closes the
   connections, and leaves the WAL to be recovered by the next `open`.

4. **`connect()` runs migration 1 every time, and this fixture is what races
   it.** `SqliteEventStore::open` calls `migrate` on every connect
   (`crates/happenstance-sqlite/src/event_store.rs:120-124`), and this is the
   first fixture in the workspace that opens two connections onto one file — so
   two `CREATE TABLE`s can genuinely race here for the first time. Idempotency
   under a concurrent open is `schema-migration-and-identity`'s obligation
   (architecture brief §7); a failure of it surfaces *here*, and the correct
   response is a red run reported against that story, not a lock added to the
   fixture.

**No data migration, no backfill, no fixture data files.** The suite constructs
every event it needs.

## Acceptance criteria

The persona is the one `../_storymap.md`'s preamble names — **an adapter author
and the library's consumer** — and both want the same single thing from this
story: *a store that has passed the bar, against durable storage, through two
real handles*. Each criterion below is that want, crossing the whole stack from
the fixture's constructor to the run's own output, stated as GIVEN / WHEN /
THEN. "The rule is green" is never the criterion on its own, because for three
of these a wrong fixture is green too.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **The bar is cleared whole, and the run says so.** GIVEN an adapter author who has been told all phase that "an adapter that compiles is not an adapter", WHEN they run `cargo test -p happenstance-sqlite --test conformance -- --nocapture` against a `SqliteFixture` backed by a real file on disk, THEN `event_store_conformance!` is green and **every** rule named in `for_each_event_store_rule!` appears in that output as either `Ran` or `Skipped` carrying this fixture's own stated reason — none is absent from the binary, none is `#[ignore]`d, and none was removed to make the run green. | `cargo test -p happenstance-sqlite --test conformance` (the whole `dcb_conformance` module emitted by `__emit_tokio`, `crates/happenstance-testkit/src/lib.rs:312-357`), read with `--nocapture`; the rule list checked against `crates/happenstance-testkit/src/registry.rs:94-218`. Serves project AC-001. |
| AC-002 | **Two fixtures are two stores, so one rule's data can never explain another's pass.** GIVEN an adapter author who has just been handed a green suite and wants to know whether it means anything, WHEN two `SqliteFixture` instances are constructed in the same process and each appends through its own handle, THEN neither observes the other's events — because `SqliteFixture::new()` mints one *fresh* temp file per instance and two instances get two paths, which is the fixture-contract mistake `contract.rs:17-23` says no testkit meta-test can catch for an adapter. | `dcb_conformance::two_fixture_instances_observe_none_of_each_others_appends` (rule body `crates/happenstance-testkit/src/suite.rs:210-238`) green, **plus** review of `SqliteFixture::new` confirming a per-instance path rather than a shared directory. Serves project AC-002. |
| AC-003 | **The second handle is a second connection, not a second reference.** GIVEN a library consumer who intends to run this store from more than one place in their process, WHEN the fixture declares `SECOND_HANDLE = Capability::SUPPORTED` and a rule calls `connect()` twice on one instance, THEN it receives two distinct `rusqlite::Connection`s onto one file — `SqliteEventStore::open(path)`, never an `Arc`/`Rc` clone of one in-process store and never `open_in_memory()` — and an append through one is visible to the other, filling the handle-multiplicity far end `RUNBOOK.md:691` records as empty. | `dcb_conformance::two_handles_observe_each_others_appends` (a MUST that panics rather than skips, `contract.rs:135-161`) and `dcb_conformance::head_advances_across_two_handles` green, **plus** review of `SqliteFixture::connect`'s body — a `Clone`-shaped fixture passes both rules and fails this criterion (`../project.md` AC-003). Serves project AC-003. |
| AC-004 | **An acknowledged write outlives the process that acknowledged it.** GIVEN a consumer who is choosing this adapter *because* it is durable, WHEN the fixture declares `REOPEN = Capability::SUPPORTED` and `reopen()` drops every live connection while leaving the file and its WAL intact, THEN a fresh `connect()` afterwards still sees both events, the exact `SequencePosition` `append` returned, the same `recorded_at` read back rather than re-stamped, and no reissued `EventId` — filling the durability far end `RUNBOOK.md:692` records as empty. | `dcb_conformance::acknowledged_writes_survive_a_reopen` (rule body `crates/happenstance-testkit/src/suite.rs:332-360`), `dcb_conformance::recorded_time_survives_a_reopen` and `dcb_conformance::reopened_store_does_not_reissue_an_event_id` green, **plus** review that `reopen()` drops handles rather than deleting or recreating the file (a no-op `reopen()` passes the first rule vacuously). Serves project AC-004. |
| AC-005 | **The store's ceilings are stated numbers, so the rule that needs them runs.** GIVEN an adapter author who must know what this store refuses before they ship on it, WHEN the fixture states real values for `MAX_EVENT_DATA_LEN`, `MAX_TAGS_PER_EVENT` and `MAX_EVENTS_PER_BATCH` — each at or above its VT floor of 65,536 bytes / 64 tags / 128 events and each small enough that the suite can allocate a payload of exactly that size — THEN `append_reports_exceeded_store_limits` reports **`Ran`**, accepting at the limit and returning `AppendError::ExceedsStoreLimit` one byte past it, rather than reporting the `NO_STORE_LIMITS` / `NO_CEILING_REASON` skip that a `None` leaves behind and that reads as a pass to anyone with only the exit code. | `dcb_conformance::append_reports_exceeded_store_limits` observed as `Ran`, not `Skipped` (`contract.rs:213-253`, `:435-456`); `dcb_conformance::store_accepts_the_guaranteed_minimum_payload`, `..._tag_count` and `..._batch_size` green (VT-21/VT-22/VT-24, floors at `crates/happenstance-core/src/limits.rs:22`, `:27`, `:43`). Serves project AC-009. |
| AC-006 | **The one capability this adapter declines says so in its own words, and the rules it gates stay visible.** GIVEN an adapter author reading this crate's CI log to learn what it does *not* do, WHEN `MID_BATCH_FAULT` is declined with a reason written for this adapter — that it supplies the reopen far end and not the fault far end, which is ES-35's live residual falsifier — rather than inherited from the trait's generic default, THEN `append_is_atomic_under_a_mid_batch_fault` and `arming_a_mid_batch_fault_makes_the_append_fail` both still appear in the binary and both print a `SKIP` line quoting that sentence, which is CF-18's whole argument. | Both rules observed as `Skipped { capability: "MID_BATCH_FAULT", reason: <this fixture's sentence> }` under `--nocapture`; review that the constant is written in `SqliteFixture`, not inherited from `contract.rs:207-211`. `spec/SPECIFICATION.md:4142` (ES-35), `:7597` (CF-18). Serves project AC-001 and DR-05. |
| AC-007 | **It went green by passing, and the repository's own gate is what says so.** GIVEN a reviewer who has been told this is the first green run against durable storage, WHEN they check *how* it went green, THEN the target contains no `#[ignore]`, no timeout, watchdog or retry loop around any rule (CF-33 keeps a clock out of the suite), nothing under `crates/happenstance-testkit/` was added, reworded or reordered, no SQL body was edited to make a rule pass, `crates/happenstance-sqlite/tests/shapes.rs` still passes unchanged in intent, and the new target is reached by `cargo xtask affected --base main` with no script edit because `tests/` is auto-discovered. | `cargo xtask affected --base main` green (`.redkiln/config.yaml:40`); `cargo test -p happenstance-sqlite --test shapes` green; `rg -n "#\[ignore\]|timeout|sleep" crates/happenstance-sqlite/tests/conformance.rs` empty; `git diff --stat` showing no change under `crates/happenstance-testkit/`. Serves project AC-001, AC-014 (precondition). |
| AC-008 | **The ceiling numbers leave as documented facts, and the clause question they touch leaves as still open.** GIVEN a planner who must not have CF-40's ownership contradiction silently resolved by an implementer choosing three integers, WHEN the three ceilings are recorded in the ledger as facts about this adapter with the reasoning for each number, THEN `.kb/open-questions/cf-40-fixture-limits-ownership.md` is still an open question atom — untouched in status, escalated to the ADR queue, and cited from this story's evidence — because this story needs the *capability* and gets no say in which ADR owns the clause. | Review: the three numbers and their rationale present in `_ledger.md`'s AC-005 and AC-008 evidence; `.kb/open-questions/cf-40-fixture-limits-ownership.md` unchanged (`git diff` empty for that path) and cited; `redkiln validate --kb` clean. Serves project AC-009. |

## Interaction quality

**The composition family is a declared skip, and this is where it is declared.**
`../_design.md` records `N/A — no user-facing surface` for `## Items`,
`## Signatures`, `## The states the API must express` and `## Anti-patterns`,
approved by the repository owner on 2026-08-12 at the `/redkiln:plan` design
sign-off gate, and `.redkiln/config.yaml` deliberately omits `design.capture` so
the perceptual review is a skip rather than a silent pass (`CLAUDE.md`). There is
no control, no placement, no transience policy and no density budget in the UI
sense, so **no composition invariant applies and none is carried by an AC**.
Stating that here is the point: a story that renders nothing must say so, or
"presentation exists at all" reads as satisfied by absence.

**The state family does apply, in this story's own medium.** The only artifact
either persona perceives is the conformance run's output, and every state
invariant has a real analogue in it — each one already carried by an AC row
above, never by a bullet here.

| State invariant | Its form here | Carried by | How it is verified |
| --- | --- | --- | --- |
| **Non-occlusion** | A rule is never hidden from the reader. Every entry in `for_each_event_store_rule!` is present in the binary and in the output; `#[cfg]`-ing one out or `#[ignore]`-ing it is occlusion, and is indistinguishable in CI from a pass. | **AC-001**, **AC-007** | The run read with `--nocapture` against `registry.rs:94-218`; `rg` for `#[ignore]` in the target. |
| **Transience — persistent, not revealed on demand** | A declined capability's reason is printed on **every** run, not surfaced only when someone goes looking. It is the persistent chrome of this surface: the sole standing record of the one trade this adapter makes. | **AC-006** | Both `MID_BATCH_FAULT` rules observed printing `SKIP` with this fixture's sentence. |
| **Reversibility** | `reopen()` discards process state and nothing else. It is undo of the *connection*, not of the data — a `reopen()` that deletes the file is destructive, and one that does nothing is a no-op wearing undo's label. | **AC-004** | The reopen rules green, plus review that `reopen()` drops handles and leaves the file and its WAL. |
| **Preserved selection** | Comparisons are made against the positions the store actually assigned, never against literals like `[1, 2, 3]`. The specification permits gaps and `AUTOINCREMENT` produces them; asserting on literals is the equivalent of clobbering the user's selection on refresh. | **AC-001**, **AC-007** | This story writes no rule; review confirms none was added or reworded (`CLAUDE.md`, *The rule that matters*). |
| **In-place, not a context jump** | The mount is a new `tests/` target auto-discovered by `cargo test -p happenstance-sqlite`, so it is reached by the gate commands the repository already runs. A capability reachable only by a bespoke command the reviewer must be told about is a context jump. | **AC-007** | `cargo xtask affected --base main` and `cargo xtask ci --fast` reach the target with no script edit. |
| **Density budget — with its real numbers** | The one place this story carries literal quantities: three ceilings, each **≥** its floor of **65,536** bytes, **64** tags and **128** events, and each far below SQLite's ~1 GB so the rule that allocates exactly that many bytes stays runnable. Both bounds are blocking; either violated makes the surface unreadable — one by failing a VT rule, the other by making the suite unrunnable. | **AC-005**, **AC-008** | The three VT minimum rules green, `append_reports_exceeded_store_limits` reporting `Ran`, and the numbers recorded with their rationale in the ledger. |

**The named anti-patterns**, taken from this project's own briefs rather than
from a design system: a fixture whose `connect()` clones one store
(architecture §1); `open_in_memory()` as the fixture's constructor (§1, third
bullet); a ceiling left `None` (§11); a `reopen()` that is a no-op; and a
watchdog around a hang (testing §4). Each is rejected by an AC above — AC-003,
AC-003, AC-005, AC-004 and AC-007 respectively — not by a bullet in this
section.

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | `connect()` written against `SqliteEventStore::open_in_memory()` (`crates/happenstance-sqlite/src/event_store.rs:132`) — the constructor an implementer reaches for first. | The second `connect()` opens a second **empty** private database and `two_handles_observe_each_others_appends` **panics** rather than skipping (`contract.rs:135-161`). This is not a fixture bug to route around: use `open(path)`. |
| **EC-002** | An implementer, faced with a red `SECOND_HANDLE` rule, declines the capability to go green. | Forbidden outright — testing brief §4: *"No fixture in this project may decline `SECOND_HANDLE`."* The rule panics on a decline by design, quoting the fixture's own reason, precisely so this move is loud. A decline here fails AC-003 and project AC-003. |
| **EC-003** | One or more ceilings left at the `None` default. | `append_reports_exceeded_store_limits` reports `Skipped { capability: NO_STORE_LIMITS, reason: NO_CEILING_REASON }` (`contract.rs:228-235`, `:435-456`). The suite is green and the hole is invisible to anyone reading the exit code. Fails AC-005 and project AC-009. |
| **EC-004** | A ceiling set to SQLite's own ~1 GB, on the reasoning that it is the true limit. | The rule allocates a payload of exactly `MAX_EVENT_DATA_LEN` bytes and then one byte more, so the suite becomes unrunnable or OOMs (architecture §11). The declared number is an **adapter-enforced policy ceiling** above the VT floor, documented as a fact and enforced in `append`. |
| **EC-005** | `reopen()` implemented as a no-op, or as `async {}` left at the trait default while `REOPEN` is declared `SUPPORTED`. | `acknowledged_writes_survive_a_reopen` passes **vacuously** — which is why `MemoryFixture` declines the capability rather than faking it (`crates/happenstance-testkit/src/fixtures.rs`). Declaring `REOPEN` without overriding `reopen()` also trips the trait's own default panic (`contract.rs:339-352`). Fails AC-004. |
| **EC-006** | `reopen()` implemented as delete-and-recreate the file. | The reopen rules fail — correctly. The correct response is to drop the live connections and leave the medium, never to decline `REOPEN` to make the red go away. |
| **EC-007** | A `Drop` that removes `<name>.db` only, while the predecessor story set journal mode to WAL. | `<name>.db-wal` and `<name>.db-shm` accumulate — two stray files per fixture instance, once per rule, on every run. Remove the whole set, or contain each fixture in its own directory and remove the directory. |
| **EC-008** | Two concurrent `connect()`s race migration 1's `CREATE TABLE`s — the first time in the workspace two connections open one file (`event_store.rs:120-124`). | A red run reported against `schema-migration-and-identity`, whose obligation idempotency-under-concurrent-open is (architecture §7). **Not** a lock added inside the fixture to hide it. |
| **EC-009** | A rule hangs — `SQLITE_BUSY`, or a lock held to commit deadlocking two connections. | Nothing in the suite notices; there is no watchdog anywhere in it and there must not be one (CF-33, `crates/happenstance-testkit/src/concurrency.rs:24-43`). Only the CI job timeout ends it. The hang is **evidence about the busy-timeout value** the predecessor story configured, escalated to ADR-0022's timeout paragraph — never bounded by a `#[timeout]` in this target. |
| **EC-010** | A rule returns `SqliteEventStoreError::NoRuntime` (`event_store.rs:26-32`). | Should not occur in this target: `__emit_tokio` emits `#[tokio::test]`, so every sequential rule runs inside a runtime and `spawn_blocking` has a handle. If it *does* occur here, it is a genuine finding about the runtime seam and belongs to `concurrency-family-and-contender-count`, reported rather than patched with a local runtime hack. |
| **EC-011** | A rule fails because the SQL is wrong. | A red run against a **predecessor's** spec — `append-atomicity-and-store-limits`, `wide-query-chunked-not-refused`, `lazy-read-with-snapshot-ceiling` or `schema-migration-and-identity`. Report it there. Editing a SQL body from this story is scope drift; editing the *rule* is worse (`CLAUDE.md`: if a rule seems wrong, fix the rule and explain why in the same change — which is not this change). |

## Non-functional

| id | Requirement | Why it binds here |
| --- | --- | --- |
| **NF-001** | **The testkit's dependency graph is unchanged.** Nothing is added to `crates/happenstance-testkit/`; the fixture lives in `crates/happenstance-sqlite/tests/`. | The testkit is published, is built for `wasm32` by `cargo xtask wasm`, and depends today on `happenstance-core` and `futures-core` and nothing else. A `rusqlite`-backed fixture inside it would put a bundled C library into the graph of the crate whose whole value is needing nothing (architecture §1). |
| **NF-002** | **No new workspace dependency without a recorded decision.** `tempfile` is not in `[workspace.dependencies]`; a process-local ordinal plus `Drop` needs none and has precedent at `crates/happenstance-testkit/tests/fixture_instruments.rs:94-102`. Either answer is acceptable; the manifest consequence is not optional to think about. | `deny.toml` is real and `cargo deny` runs in the gate, so a manifest change is a gate-visible change (architecture §11). |
| **NF-003** | **The target is safe under libtest's default parallelism.** Rules run on parallel threads, so many `SqliteFixture` instances exist at once. Path minting must be process-globally unique and atomic, and cleanup must not race a sibling. | Every rule constructs its own fixture; a non-atomic ordinal or a shared directory produces intermittent cross-talk that looks like an isolation failure and is not (AC-002 becomes unfalsifiable). |
| **NF-004** | **The run leaves no files behind.** After `cargo test -p happenstance-sqlite --test conformance` returns, no `.db`, `-wal` or `-shm` file from this run remains. | ~90 rules × 1–2 fixtures each, every run, on every developer machine and CI runner. See EC-007. |
| **NF-005** | **The run is repeatable and its duration is recorded.** The target passes on three consecutive runs, and its wall-clock time is written into the ledger as a fact. | There is no watchdog to catch a slow drift into a hang (CF-33), so the only defence is a recorded baseline a later story can compare against. This is a recorded number, not a gate threshold — a threshold would be a clock in the suite by another route. |
| **NF-006** | **`cargo xtask ci --fast` stays green**, this project's integration grain (`.redkiln/config.yaml:55`, `../project.md` DoD 5). | A precondition for reading any criterion, never a substitute for one (`RUNBOOK.md:38-42`). |

## Implementation notes (non-prescriptive)

Calls the compiler, a measurement or a review is better placed to make than this
document — recorded so they are made deliberately rather than discovered.

- **Temp-file ownership.** Two shapes work: a `tempfile` dependency (a workspace
  manifest change `cargo deny` sees) or a process-local `AtomicUsize` ordinal
  under `std::env::temp_dir()` with a `Drop` that removes the whole set. The
  second has in-tree precedent; neither is mandated (architecture §11).
- **Whether the fixture holds its handles.** `reopen()` must drop live
  connections, which means the fixture needs some way to reach them — holding
  `Weak`s, holding the stores in a `Mutex<Vec<_>>`, or arranging that the rules'
  own handles are the only ones and relying on `contract.rs:323-328`'s licence
  that handles taken before `reopen()` may stop working. Read the rule bodies
  before choosing; `acknowledged_writes_survive_a_reopen` drops its handle before
  calling.
- **Whether the adapter needs a `pub` path accessor.** If `reopen()` needs the
  path a store was opened at, an additive accessor on `SqliteEventStore` is in
  scope; keep it mechanical and out of the SQL.
- **One target or several.** Architecture §1 and testing §7 both allow the model
  and concurrency macros to live in this target or in siblings — but neither is
  in *this* story. Leaving `conformance.rs` holding only `event_store_conformance!`
  keeps the later split free.
- **Reading the run.** libtest suppresses a passing test's stdout, so the
  `Ran`/`Skipped` accounting AC-001 and AC-006 need is only visible under
  `-- --nocapture` (or `--show-output`). Capture that output as ledger evidence
  rather than re-deriving it later.
- **Copy the mount, not the fixture.** `crates/happenstance-testkit/tests/memory_conformance.rs`
  is the shape of the file; `crates/happenstance-testkit/src/fixtures.rs`'s
  `MemoryFixture` is the shape of the fixture to avoid. Reading them in that
  order is the cheapest way to get both halves right.

## Tests and CI (merge gate)

Tiers as the testing brief defines them (§1, §6), each pinned to a command the
repository already runs. No tier here is an intention without an invocation.

| Tier | Command / path | Proves |
| --- | --- | --- |
| **Static** | `cargo xtask lints && cargo xtask spec-trace` (`reachability_static`, `.redkiln/config.yaml:48`) | fmt/clippy `-D warnings` over the new target, and that the specification's citations still resolve after this story touches CF-16 – CF-18 and CF-40's *exercise* without amending a clause. |
| **Unit / type-level** | `cargo test -p happenstance-sqlite --test shapes` — `crates/happenstance-sqlite/tests/shapes.rs` | `SqliteReadStream: Send + Unpin`, `SqliteEventStore: Send + Sync` and the error bounds still hold after the predecessor stories added fields. The cheap tripwire that must be green before the conformance output is worth reading. **AC-007.** |
| **Conformance (this story's real proof)** | `cargo test -p happenstance-sqlite --test conformance` — `crates/happenstance-sqlite/tests/conformance.rs` (new) | `event_store_conformance!(SqliteFixture::new())` green against a real file through real `rusqlite::Connection`s. **AC-001 – AC-005.** This *is* the integration tier: there is no separate one, because the target is a real file on disk (testing §1). |
| **Conformance, output-inspected** | `cargo test -p happenstance-sqlite --test conformance -- --nocapture` | Every rule accounted for as `Ran` or `Skipped{reason}`, and the two `MID_BATCH_FAULT` rules printing this fixture's own sentence. **AC-001, AC-006.** libtest hides passing tests' stdout, so this is not optional. |
| **Repeatability** | the conformance command, three consecutive runs, wall-clock recorded | **NF-003, NF-005** — parallel-safe path minting and a recorded duration baseline, in the absence of any watchdog. |
| **Hygiene** | list the temp directory before and after a run | **NF-004** — no `.db`/`-wal`/`-shm` residue. |
| **Story gate** | `cargo xtask affected --base main` (`affected_gate`, `.redkiln/config.yaml:40`) | The new target is auto-discovered and reached without a script edit. **AC-007.** |
| **Project integration grain** | `cargo xtask ci --fast` (`integration_scoped`, `:55`) | The whole non-terminal bar — `../project.md` DoD 5. **NF-006.** |
| **Review, not a command** | `SqliteFixture::new` / `connect` / `reopen` bodies; `git diff --stat` over `crates/happenstance-testkit/`; `.kb/open-questions/cf-40-fixture-limits-ownership.md` unchanged | The three criteria a green rule cannot establish: a `Clone`-shaped `connect` passes AC-003's rules, a no-op `reopen` passes AC-004's first rule, and choosing three integers must not settle CF-40's clause home. **AC-002, AC-003, AC-004, AC-008.** |
| **Deliberately not run here** | `event_store_model_conformance!`, `event_store_concurrency_conformance!`, `event_store_benchmarks!`, the full `cargo xtask ci` | Owned by `model-family-and-mutant-pass-column`, `concurrency-family-and-contender-count`, `benchmark-harness` and `closeout-and-durable-audience` respectively. Mounting either family here imports the runtime-seam and `CONTENDERS` questions into a story with no AC for them. |

**Merge gate for this story:** the conformance target green and output-inspected,
`shapes.rs` green, `cargo xtask affected --base main` green, every AC row in
`_ledger.md` flipped with cited evidence.

## Risks and coupling (PR-scoped)

- **A red run here is almost always a predecessor's bug, and the cheapest wrong
  move is to fix it here.** This story mounts four stories' worth of SQL for the
  first time. The discipline is to report the failure against
  `append-atomicity-and-store-limits`, `wide-query-chunked-not-refused`,
  `lazy-read-with-snapshot-ceiling` or `schema-migration-and-identity` and let it
  be fixed there (EC-011). The slice is implemented in one context precisely so
  that is a short round trip, not a blocked story.
- **The migration race is genuinely new here.** This is the workspace's first
  fixture that opens two connections onto one file, so two `CREATE TABLE`s can
  race for the first time (EC-008). Expect it; do not absorb it.
- **A hang costs the whole binary and there is nothing to catch it** (EC-009,
  CF-33). `BEGIN IMMEDIATE` with a lock held to commit is exactly where that
  lives, and `RUNBOOK.md:2669-2674` names phase 8 — this project — as where the
  cost was accepted rather than discovered.
- **`recorded_time_survives_a_reopen` may pass on the first attempt with nothing
  in the workspace able to fail it** (`RUNBOOK.md:3205-3207`). If it does, that is
  a claim to check, not a result to accept — `CLAUDE.md`'s "a rule that no adapter
  can fail is decorative". The negative control is
  `reopen-negative-control-and-durability-verdicts`'s, and this story's ledger
  should say so rather than imply the rule is falsified.
- **CF-40's ownership contradiction is forced by choosing three integers**
  (`.kb/open-questions/cf-40-fixture-limits-ownership.md`, which names phase 8 as
  what forces it). Record and escalate to the ADR queue; resolving it in passing
  is the failure mode AC-008 exists to block, and ADR authorship never happens as
  a side effect of a code change.
- **Scope pressure from the two unlocked stories.** Both
  `concurrency-family-and-contender-count` and
  `reopen-negative-control-and-durability-verdicts` are blocked on this one, and
  the temptation at a green run is to add one more macro invocation "while the
  file is open". That imports the runtime seam and the 8-versus-64 contender
  decision into a story with no AC for either.
- **This project is the longest single stretch on the trunk** with
  `publication-and-positioning` behind it (`../project.md`, last risk), and the
  temptation schedule pressure creates is exactly the one AC-005 and AC-006
  block: declaring the crate done while a capability is quietly declined or a
  ceiling quietly left `None`.

## Dependencies

**Blocks on** (must be merged first — both are slice-mates in
`durable-event-store`, implemented in the same context):

- **`append-atomicity-and-store-limits`** — supplies the enforcement side of the
  three ceilings this story *declares*. Without it, AC-005's
  `append_reports_exceeded_store_limits` fails at the "one byte more" assertion
  rather than skipping, and the empty-batch and atomicity rules have no body.
- **`wide-query-chunked-not-refused`** — supplies the widest read family the
  suite exercises, over `lazy-read-with-snapshot-ceiling`'s stream and
  `schema-migration-and-identity`'s migration 1. Without it, VT-23 and the
  multi-item query rules fail on a limit rather than on a defect.

**Unlocks** (both name this story in their own `depends_on`, `../_storymap.md`):

- **`concurrency-family-and-contender-count`** — mounts
  `event_store_concurrency_conformance!` on top of this green sequential suite,
  with ADR-0022's runtime seam, and closes the 8-versus-64 discrepancy.
- **`reopen-negative-control-and-durability-verdicts`** — gives
  `recorded_time_survives_a_reopen` the negative control it has lacked since
  phase 4, and lands the CF-17 / ES-35 / CF-14 verdicts this story only
  *exercises*.

Downstream of both: `model-family-and-mutant-pass-column`, then
`instrument-markers-removed-and-gate-green` and the
`publishable-and-reconciled` slice.

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. Open each at the moment named.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `crates/happenstance-testkit/src/contract.rs` | The `Fixture` trait itself: the five constants, `Capability`, `RuleOutcome`, the trait defaults that panic when a capability is declared without an override (`:294-352`), and the limits-are-facts-not-trades distinction (`:44-54`, `:213-279`) with `NO_STORE_LIMITS` / `NO_CEILING_REASON` (`:442-456`). Nothing else in the repository states what a fixture is *obliged* to be. | Before writing `SqliteFixture`'s five associated constants and its two overridden methods — i.e. first. | AC-003, AC-004, AC-005, AC-006 |
| `crates/happenstance-testkit/src/suite.rs` | The rule bodies. `two_fixture_instances_observe_none_of_each_others_appends` at `:210-238` and `acknowledged_writes_survive_a_reopen` at `:332-360` are what the fixture is actually measured against — including which handles they drop and which exact `SequencePosition` they re-check after a reopen. | When a rule goes red, **before** changing anything: read what it asserts rather than inferring it from the failure message. | AC-001, AC-002, AC-004 |
| `crates/happenstance-testkit/src/registry.rs` | `for_each_event_store_rule!` at `:94-218` is the canonical enumeration AC-001's "whole" means. It is also the only place the rule *families* are grouped, which is how a missing family is spotted in a long output. | When auditing the `--nocapture` output for a rule that is absent rather than skipped. | AC-001 |
| `crates/happenstance-testkit/src/lib.rs` | `event_store_conformance!`'s arms and the `__emit_tokio` default at `:312-357` — what the one-line mount expands to, why the module is called `dcb_conformance`, and why the adapter needs `tokio` with `macros`/`rt` in `[dev-dependencies]`. | Before writing the single macro invocation, and again if the emitted test names do not match what the ledger cites. | AC-001 |
| `crates/happenstance-testkit/tests/memory_conformance.rs` | The reference **mount** — the exact file shape to copy for `conformance.rs`. | When creating `crates/happenstance-sqlite/tests/conformance.rs`. | AC-001 |
| `crates/happenstance-testkit/src/fixtures.rs` | The reference **fixture not to copy**: `MemoryFixture`'s `connect` is an `Arc` clone and it *declines* `REOPEN` rather than faking it. Both are the named wrong shapes for this story, and seeing them is faster than being warned about them. | Immediately after the mount, before writing `connect()` and `reopen()`. | AC-003, AC-004 |
| `crates/happenstance-testkit/tests/fixture_instruments.rs` | `DurableFixture` — the workspace's only prior art for a fixture that survives a reopen: the mint-once `StoreId` at `:75-90` (why `recorded_time_survives_a_reopen` and `reopened_store_does_not_reissue_an_event_id` are askable at all) and the process-local ordinal + `Drop` cleanup at `:94-102` (the no-new-dependency temp-file precedent). | Before deciding how the fixture owns its temp file and its handles. | AC-002, AC-004 |
| `crates/happenstance-sqlite/src/event_store.rs` | `SqliteEventStore::open` at `:120` versus `open_in_memory` at `:132` — the single line of the codebase that decides AC-003, plus the `migrate`-on-every-connect call at `:120-124` that makes EC-008 possible and the `NoRuntime` path at `:26-32` behind EC-010. | Before writing `connect()`. | AC-003 |
| `crates/happenstance-sqlite/tests/shapes.rs` | The existing type-level tripwire the new target sits beside; it must still pass unchanged in intent (architecture AC-A01) after the predecessor stories added fields to the store and stream. | Before declaring the conformance run green — a red `shapes.rs` makes the conformance output unreliable. | AC-007 |
| `crates/happenstance-core/src/limits.rs` | The VT floors the three declared ceilings must sit at or above: 65,536 bytes (`:22`), 64 tags (`:27`), 128 events (`:43`). One half of AC-005's density budget; SQLite's ~1 GB is the other. | When choosing the three numbers. | AC-005 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md` | Both project briefs. Architecture §1 (the mount table and the three integration rules an implementer building in isolation gets wrong), §7 (migration idempotency under a concurrent open), §9 (`MID_BATCH_FAULT`, the CF-40 escalation, the 8-versus-64 note), §11 (ceilings and the temp-file manifest consequence); testing §1 (the tier table), §3 (what may **not** be doubled), §4 (no watchdog, and no fixture may decline `SECOND_HANDLE`), §5 (the negative control this story does *not* own), §6 (the gate commands by grain). | Before writing the fixture; and again before writing ledger evidence for AC-005, AC-006 or AC-007. | AC-002, AC-005, AC-006, AC-007 |
| `.kb/open-questions/cf-40-fixture-limits-ownership.md` | The open question this story's three integers force. It names phase 8 as what forces it, and ADR-0015 both claims and disclaims the clause. Must leave this story open and escalated, not resolved. | When writing AC-005's and AC-008's ledger evidence. | AC-008 |
| `spec/SPECIFICATION.md` | The clauses this story discharges against a real adapter for the first time: CF-16 (`:7548`, second handle), CF-17 (`:7570`, reopen), CF-18 (`:7597`, declared capabilities and reported skips), CF-40 (`:7661`, stated ceilings), plus ES-35's `[PROVISIONAL]` marker and its residual falsifier (`:4142`) and VT-21/VT-24 (`:1483`, `:1556`). | When writing the `MID_BATCH_FAULT` decline sentence and the ceilings' rationale — both must quote a clause, not a preference. | AC-004, AC-005, AC-006, AC-008 |
| `RUNBOOK.md` | `:691-692` — the two instrument-portfolio far ends this story fills, in the runbook's own words ("no *connection* has been opened twice"; "Nothing yet loses a write to a *fault*"), which is the sentence AC-003's and AC-004's ledger evidence should be measured against. `:4166-4237` is phase 8 in full; `:3205-3207` is the missing negative control; `:38-42` is why a green gate is never a substitute for a criterion. | When writing ledger evidence for AC-003 and AC-004, and when tempted to treat a green reopen rule as proof. | AC-003, AC-004 |
| `crates/happenstance-testkit/src/concurrency.rs` | CF-33's no-watchdog reasoning at `:24-43` — why there is no timeout anywhere in the suite and why adding one here is forbidden — and the opt-in concurrency family this story deliberately does not mount. | If a rule hangs (EC-009), or when tempted to bound the target. | AC-007 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/_design.md` | The signed-off no-surface determination — approved 2026-08-12, every section `N/A`, with `design.capture` deliberately absent from `.redkiln/config.yaml` so the perceptual review is a declared skip. It is what makes this story's *Interaction quality* composition family legitimately empty rather than overlooked. | When the composition-invariant question is raised in review. | AC-007 |

## Clarifications resolved during spec

1. **The AC set is exactly the eight the first pass decided** — AC-001 through
   AC-008 — and none was added or dropped. They cover all five traced project
   ACs: project AC-001 by story AC-001, AC-006 and AC-007; AC-002 by AC-002;
   AC-003 by AC-003; AC-004 by AC-004; AC-009 by AC-005 and AC-008.
2. **The composition family of *Interaction quality* is empty by sign-off, not
   by omission.** `../_design.md` records no user-facing surface for the whole
   project and was approved at the design gate; this story does not re-open that
   determination. The state family is mapped to the run's own output, and every
   invariant that applies is carried by an AC row rather than by a bullet.
3. **"The suite is green" is not an acceptance criterion on its own.** Three
   criteria — AC-002, AC-003, AC-004 — are partly discharged by *review of the
   fixture's body*, because a fixture that shares one directory, clones one
   store, or no-ops `reopen()` produces a green run for each of them. That is the
   project's own framing (`../project.md` AC-003: *"observable in the fixture's
   own code, not merely asserted"*), carried into the verification column rather
   than left as an assumption.
4. **A declined `MID_BATCH_FAULT` gets its own AC (AC-006) even though the trait
   default already declines it.** The outcome is identical; the *sentence* is
   not, and the sentence is the only standing record of this adapter's one trade.
   Without an AC, "we inherited the default" and "we wrote our reason" are
   indistinguishable in the ledger.
5. **`SECOND_HANDLE` may not be declined, and that is a project-level rule, not a
   judgement made here** (testing brief §4). Recorded as EC-002 so the escape
   hatch is named and closed rather than silently unavailable.
6. **The `recorded_time_survives_a_reopen` negative control is explicitly *not*
   this story's.** This story makes the rule runnable; `reopen-negative-control-and-durability-verdicts`
   makes it falsifiable (testing §5). AC-004 therefore requires the rule to pass
   and requires the ledger to say the falsification is still owed — it does not
   claim the rule has been proved able to fail.
7. **CF-40's clause home stays open**, and AC-008 makes leaving it open a
   checkable obligation rather than an omission. Choosing three integers is the
   act that forces the question; recording and escalating is the whole of this
   story's duty toward it.
8. **NF-005's recorded run duration is a fact, not a threshold.** A time limit
   enforced by the target would be a clock in the suite by another route, which
   CF-33 forbids; a recorded baseline gives a later story something to compare
   against without one.
