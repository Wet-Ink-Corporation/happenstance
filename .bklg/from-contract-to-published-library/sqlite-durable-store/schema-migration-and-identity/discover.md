---
item: HS-S0036
stage: discover
created: 2026-08-12T13:01:59.165Z
updated: 2026-08-12T13:01:59.165Z
template_sig: 86ce4036
rendered_sig: b02ad4f7
---

# Discover — Migration 1: the amended schema, persisted StoreId, and idempotent concurrent open

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: land migration 1 as ADR-0022 amends it — `event_tag` keyed `(tag, position)` with `event_type` as a covering column, `tag_cardinality`, the `EventId` origin pair `UNIQUE`, `recorded_at`, and a `StoreId` minted once and persisted — idempotent under two concurrent opens, with WAL, a stated `synchronous` and a finite busy timeout. | `_storymap.md`, *Slices* table, row `durable-event-store` / `schema-migration-and-identity` | Seven schema obligations plus three connection pragmas. The pragmas are part of the *schema* story because two of them (WAL, busy timeout) are what make the fixture's second connection possible at all. |
| **AC-001** — the suite runs, whole: every rule in `for_each_event_store_rule!` appears as `Ran` or `Skipped{reason}`, none absent. | `project.md`, AC-001; `crates/happenstance-testkit/src/registry.rs:94` | This story's half of AC-001 is "schema makes any rule runnable" (`_storymap.md`, *Coverage*). Before migration 1 exists, every rule fails at `open`. |
| **AC-004** — an acknowledged write survives a reopen: `REOPEN` available, `acknowledged_writes_survive_a_reopen` green across a genuine close-and-reopen; `recorded_time_survives_a_reopen` gets the negative control it has lacked since phase 4. | `project.md`, AC-004; `RUNBOOK.md:3205-3207` | This story's half is the **architectural precondition**: a persisted `StoreId` and a `recorded_at` that is read back rather than re-stamped. The rules themselves are `sqlite-fixture-and-whole-suite`'s; the negative control is `reopen-negative-control-and-durability-verdicts`'. |
| `dependsOn: adr-0022-append-condition-strategy` (HS-S0035) — it supplies the tag layout, the amended schema, the journal mode, the `synchronous` setting and the busy-timeout value this migration writes down. | `_storymap.md`, *Merge order* item 2; `_decomposition.md`, *Architecture brief*, §12 | Not a sequencing preference: the tag layout **is** the schema. Writing migration 1 before ADR-0022 would settle the ADR by construction. |
| The schema sketch in the crate's own module doc is **known to be wrong** in a way that serialises every writer, and the correction lives in the runbook rather than in the code. | `crates/happenstance-sqlite/src/event_store.rs:36-54` against `RUNBOOK.md:4178-4187` | "Implementing the doc comment as written is the most likely way to ship a slow, conformant adapter" (`_decomposition.md`, *Architecture brief*, §7). The module doc is *published documentation*; correcting it is part of this work. |
| The amendment, precisely: carry `event_type` as a covering column on `event_tag`, key left `(tag, position)` so the range stays sorted by position, and add a `tag_cardinality` table — because multi-tag arms must be probed most-selective-tag-first and `ANALYZE` stores only an average. | `RUNBOOK.md:4178-4187`; `_decomposition.md`, *Architecture brief*, §7 | Without the covering column a `QueryItem`'s type constraint becomes a join back to `event`, walked **under the `BEGIN IMMEDIATE` write lock**. |
| `AUTOINCREMENT` is load-bearing: positions must never be reused after a delete, and plain `rowid` does not guarantee it. | `RUNBOOK.md:4193-4197`; `project.md`, *In scope* | `AUTOINCREMENT` also *produces gaps*, which the specification permits — so no code and no rule this project writes may assume `+1` (`project.md`, risks; `CLAUDE.md`). |
| `EventId::new(StoreId, SequencePosition)` — so `contains_event_id`'s probe needs the **origin** store and origin position stored and `UNIQUE` together. | `crates/happenstance-core/src/identity.rs:97`; `crates/happenstance-sqlite/src/event_store.rs:237-244` | One index serves two purposes: the probe's lookup, and the constraint that stops ingest storing one event twice. |
| The store's own `StoreId` is minted **once at schema creation and persisted**. VT-6 permits mint-per-open in general; the testkit's own `DurableFixture` switched to mint-once for exactly this reason and says so. | `_decomposition.md`, *Architecture brief*, §7; `crates/happenstance-testkit/tests/fixture_instruments.rs:75-83` | "Mint-per-open is the wrong choice for a file-backed store" — it is what makes `reopened_store_does_not_reissue_an_event_id` and `recorded_time_survives_a_reopen` askable at all. |
| `SqliteEventStore::open` calls `migrate` on **every** connect, and the fixture connects twice onto one file. | `crates/happenstance-sqlite/src/event_store.rs:120-124`, `:139` | Two `CREATE TABLE`s can race. Migration must be idempotent *and* safe under a concurrent open: `IF NOT EXISTS` inside a `BEGIN IMMEDIATE`, and `INSERT OR IGNORE` then read-back for the `StoreId` row. |
| ADR-0013's global visibility invariant, and its concrete corollary already written into the skeleton: `head` queries fresh every time and must never become a cached field `append` updates, "because one file backs several handles." | `.kb/decisions/0013-position-assignment-and-visibility.md`; `crates/happenstance-sqlite/src/event_store.rs:226-235` | A cached head is a schema-adjacent temptation — a `store_head` row updated on append reads as "a fact the schema holds" and goes stale the instant a second connection commits. |
| The projection store's checkpoint schema migrates independently, on its own connection. | `crates/happenstance-sqlite/src/projection_store.rs:33-43`; `_decomposition.md`, *Architecture brief*, §7 | "An event store and a projection store on one file are two connections, not one" — so both migrations must be independently idempotent. |
| CF-14 names `PRAGMA synchronous = OFF` **by name** as a wrong implementation the reopen rule exists to reject. | `spec/SPECIFICATION.md:7471-7490` | The `synchronous` value chosen here is watched by a named clause; it is a documented property of the adapter, not a local tuning choice. |

## Questions

Open questions to resolve before specifying.

1. **Is migration versioned, or is it a single idempotent `CREATE … IF NOT
   EXISTS` block?** Deferred to `spec`. Nothing in the ACs requires a migration
   table, and this is migration **1** with no predecessor to upgrade from. What
   the spec must not do is defer it silently — a schema with no version marker is
   a schema that cannot be migrated later, and this crate is about to become
   publishable.
2. **Is WAL set per connection or persisted in the file?** Deferred to `spec` and
   flagged in the architecture brief's §11 as deliberately left to the
   implementer. The constraint discover fixes: whichever it is, it must hold for
   the *second* connection the fixture opens, not only the first.
3. **Where is `recorded_at` sourced, and in what representation?** Deferred to
   `spec`. The one property discover fixes is behavioural and is AC-004's:
   `recorded_at` is written once and **read back**, never re-stamped on open. Note
   CF-33 constrains *rules* reading clocks and says nothing about an adapter
   stamping a recorded time — the two are easy to conflate.
4. **Does the `StoreId` row live in its own table or in a general key/value
   `meta` table?** Deferred to `spec`. Discover fixes only that it is minted once,
   persisted, and read back under the same `BEGIN IMMEDIATE` that creates it, via
   `INSERT OR IGNORE` + read-back so two concurrent opens agree.
5. **What are the three declared ceilings' numbers?** *Not this story's.*
   `append-atomicity-and-store-limits` (HS-S0037) enforces them and
   `sqlite-fixture-and-whole-suite` (HS-S0040) declares them. This story must not
   encode a ceiling in a column type (e.g. a `VARCHAR(n)`) and thereby decide one
   by accident — ADR-0015 is explicit that a capacity limit "must never be
   enforced by a constructor".
6. **The append-condition SQL strategy.** Consumed here, not decided here.
   ADR-0022 owns it; this story writes the indexes that strategy requires and
   nothing more. If the tag layout ADR-0022 chose turns out not to support the
   probe, that is a finding to escalate to the ADR queue, not a schema improvised
   at implementation time.

## Decision

Every rule in the suite fails at `open` until this schema exists, and two of the
project's headline claims — that an acknowledged write survives a reopen, and
that a reopened store does not reissue an `EventId` — are not even *askable*
without a `StoreId` that outlives the connection that minted it. This slice lands
migration 1 in the shape `RUNBOOK.md:4178-4187` amends rather than the shape the
crate's own module doc describes, and makes it safe to run twice at once, because
the fixture that will drive the whole suite opens two connections onto one file
and calls `migrate` on each. The spec will cover: `event` with
`position INTEGER PRIMARY KEY AUTOINCREMENT`, `event_type`, payload, metadata and
`recorded_at`; the origin `(StoreId, position)` pair stored and `UNIQUE` together
for `contains_event_id`; `event_tag` `WITHOUT ROWID` keyed `(tag, position)` with
`event_type` carried as a covering column; `tag_cardinality`; a `StoreId` minted
once at schema creation, persisted, and read back; idempotent migration under a
concurrent open (`IF NOT EXISTS` inside `BEGIN IMMEDIATE`, `INSERT OR IGNORE`
then read-back); WAL, the `synchronous` setting ADR-0022 fixed, and a **finite**
busy timeout; the projection store's checkpoint migration made independently
idempotent on its own connection; and the correction of the known-wrong module-doc
sketch at `crates/happenstance-sqlite/src/event_store.rs:36-54` in the same change,
since it is published documentation. This story adds **no conformance rule**, so
the literal-position bar is vacuous — but it is the story that makes gaps real
(`AUTOINCREMENT` never reuses a position after a delete), which every later story
inherits. No `[FROZEN]` clause is amended; CF-14 is `[DEFERRED]` and is *complied
with* here by not choosing `synchronous = OFF`.

## The wrong implementation

**The mutant: `migrate` that runs `CREATE TABLE IF NOT EXISTS` outside a
transaction, on every connect.** It is what the skeleton's shape invites —
`open` already calls `migrate` unconditionally
(`crates/happenstance-sqlite/src/event_store.rs:120-124`) and `IF NOT EXISTS`
*reads* as the idempotence guard, so the transaction looks redundant.

It passes everything, almost always. Single-handle rules never race. Even
`two_handles_observe_each_others_appends` usually wins, because the second
`connect()` finds the tables already there. The defect is a **window**: two
connections calling `migrate` concurrently — which is exactly what a fixture
whose `connect()` opens a real second connection does, and what
`event_store_concurrency_conformance!` does `CONTENDERS` times — can interleave
between the existence check and the create, producing `SQLITE_BUSY` or a
duplicate-object error from one of them. That surfaces as `AppendError::Store` →
`Attempt::Failed` (`crates/happenstance-testkit/src/concurrency.rs:219-231`), i.e.
as a *conformance verdict about atomicity*, in perhaps one run in twenty. A flake
that names the wrong rule is worse than a failure: it teaches the next reader to
re-run, which `racers.rs:20-34` already says in terms about non-deterministic
mutants. The control is structural and belongs in this story: the whole migration
inside one `BEGIN IMMEDIATE`, plus an adapter-owned test in
`crates/happenstance-sqlite/tests/` that opens N connections onto one fresh path
from N threads and asserts every one succeeds and they all read back the same
`StoreId`. It cannot go in the testkit's `mutation_coverage/` — that harness is
built from `Rc`/`RefCell` primitives on one thread specifically to have no I/O
(`crates/happenstance-testkit/tests/mutation_coverage/harness.rs:24-30`), so a
SQL migration race has no expression there.

**The second mutant, and it is the one the suite is genuinely blind to today: a
store that mints a fresh `StoreId` on every `open`.** VT-6 permits mint-per-open
as a general mechanism, so this is not obviously wrong — it is a defensible
reading of the contract that happens to be fatal for a file-backed store. It
passes every sequential rule, passes `two_handles_observe_each_others_appends`,
and — critically — passes `reopened_store_does_not_reissue_an_event_id` and
`recorded_time_survives_a_reopen` **by skipping them**, because those three rules
are on `MUST_SKIP` (`crates/happenstance-testkit/tests/mutation_coverage.rs:3157-3163`)
and skip whenever a fixture declines `REOPEN`. So the mutant plus a fixture that
quietly declines `REOPEN` is a fully green run, and `RUNBOOK.md:692`'s durability
far end stays empty while looking filled. The testkit already documents the
correct shape rather than enforcing it: `DurableFixture` keeps "the incarnation
this store keeps across a reopen — the mint once at construction" and says the
other permitted mechanism is what it is testing against
(`crates/happenstance-testkit/tests/fixture_instruments.rs:75-83`). This story's
obligation is the persistence half; the *negative control* — a store that
re-stamps rather than reads back — is `reopen-negative-control-and-durability-verdicts`
(HS-S0043) and is named there, so the two stories together close it rather than
each assuming the other did.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
