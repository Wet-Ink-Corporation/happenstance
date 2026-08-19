---
item: HS-S0040
stage: discover
created: 2026-08-12T13:02:03.407Z
updated: 2026-08-12T13:02:03.407Z
template_sig: 86ce4036
rendered_sig: 53e86f17
---

# Discover — SqliteFixture, and the first green conformance run against a file

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: mount `SqliteFixture` in `crates/happenstance-sqlite/tests/conformance.rs` — one instance is one fresh temp file, each `connect()` a second `rusqlite::Connection` onto it — declaring `SECOND_HANDLE`, `REOPEN` and real ceilings, declining `MID_BATCH_FAULT` with its reason, and take `event_store_conformance!` green whole. | `_storymap.md`, *Slices* table, row `durable-event-store` / `sqlite-fixture-and-whole-suite` | This is the composition root. "A fixture without `append` and `read` is a mount with nothing mounted", which is why the whole slice is one slice (`_storymap.md`, *Slice coherence notes*). |
| **AC-001** — the suite runs, whole: every rule in `for_each_event_store_rule!` appears as `Ran` or `Skipped` with the fixture's stated reason; **no rule is absent from the binary**. | `project.md`, AC-001; `crates/happenstance-testkit/src/registry.rs:94`, `:413-423` | This story owns the whole-run claim. The evidence is the run's own output, not a green exit code — a skipped rule and a passing rule are indistinguishable in a summary line, which is CF-18's entire argument (`crates/happenstance-testkit/src/contract.rs:32-42`). |
| **AC-002** — two fixture instances share nothing: the isolation rule passes against a fixture opening a fresh temporary file per instance; "pointing every instance at one directory is the adapter mistake the fixture contract exists to catch." | `project.md`, AC-002; `crates/happenstance-testkit/src/contract.rs:17-23` | The rule is `two_fixture_instances_observe_none_of_each_others_appends` (`registry.rs:105`). It exists as a *conformance rule* rather than a testkit meta-test precisely because no test the testkit writes about its own fixture could observe an adapter's mistake. |
| **AC-003** — the second handle is a second connection: `two_handles_observe_each_others_appends` passes through two distinct `rusqlite::Connection`s, "observable in the fixture's own code, not merely asserted. A fixture that returns a `Clone` of one store still passes the rule and does not satisfy this criterion." | `project.md`, AC-003 | The AC states outright that the rule is insufficient evidence for it. That is the whole of *The wrong implementation* below. |
| **AC-004** — an acknowledged write survives a reopen: `REOPEN` available and the reopen rules green across a genuine close-and-reopen of the file. | `project.md`, AC-004 | This story declares the capability and takes the rules green; the *negative control* is `reopen-negative-control-and-durability-verdicts` (HS-S0043). Two stories, split by clause (`_storymap.md`, *Coverage*). |
| **AC-009** — the fixture states its real numeric ceilings, VT-21 – VT-24 pass, and `append_reports_exceeded_store_limits` **runs rather than skipping**; CF-40's clause home stays recorded as open. | `project.md`, AC-009 | This story owns "the fixture states them"; `append-atomicity-and-store-limits` owns enforcement. Both halves are required for the rule to run rather than skip. |
| `dependsOn: append-atomicity-and-store-limits` (HS-S0037) — real `append`, the enforced ceilings, and the one-transaction write. `dependsOn: wide-query-chunked-not-refused` (HS-S0039) — the full read path including the chunked wide query VT-23's rule exercises. | `_storymap.md`, *Merge order* item 2 | Both edges are "the suite cannot be green without it", not sequencing preference. |
| `Fixture`: one instance is one backing store; each `connect` returns a handle onto **that** store; two instances share nothing. | `crates/happenstance-testkit/src/contract.rs:12-16`, `:120-125`, `:321` | The trait was reshaped exactly because a bare `Fn() -> S` "would happily have accepted `|| store.clone()`, so no rule could call it twice" (`contract.rs:5-11`). |
| `SECOND_HANDLE` is the one **MUST**: the rule requiring it **panics** rather than reporting a skip on a decline. `REOPEN` is a `SHOULD`. `MID_BATCH_FAULT` defaults declined. | `crates/happenstance-testkit/src/contract.rs:145`, `:161`, `:173`, `:207-211` | `REOPEN`'s meaning is deliberately the weaker of two: "every outstanding handle's process-level state discarded", not a process kill — which is exactly what closing and reopening a `rusqlite::Connection` gives for free (`_grounding.md`). |
| `MemoryFixture` is the reference implementation **and the shape not to copy**: `connect` is `core::future::ready(MemoryHandle(Arc::clone(&self.0)))`, with a comment saying "a real fixture's `connect` does I/O and this one does not." | `crates/happenstance-testkit/src/fixtures.rs:243`, `:286-291` | The wrong shape is the *documented reference*. An adapter author reads it first, by design (`crates/happenstance-testkit/Cargo.toml` dependency comment). |
| `MemoryFixture::sharing` exists solely so the isolation rule can be shown to fail — "handing every instance the same `Arc` reproduces, in memory, the file-backed adapter that points every fixture at one temporary path." | `crates/happenstance-testkit/src/fixtures.rs:251-266` | AC-002's mutant already exists in the testkit and already bites. AC-003's does not. |
| `SqliteEventStore::open_in_memory` is the wrong constructor for this fixture and "the one an implementer reaches for first": a private in-memory database is per-*connection*, so a second `connect()` opens a second, empty database. | `_decomposition.md`, *Architecture brief*, §1; `crates/happenstance-sqlite/src/event_store.rs:132-133` | This mistake **is** caught — by a MUST that panics. Worth stating beside the one that is not. |
| `SqliteFixture` lives in `crates/happenstance-sqlite/tests/`, never in `crates/happenstance-testkit/src/fixtures.rs`: the testkit is published, is built for `wasm32`, and its `[dependencies]` are `happenstance-core` and `futures-core` and nothing else. | `_decomposition.md`, *Architecture brief*, §1; `crates/happenstance-testkit/Cargo.toml` | Putting a `rusqlite`-backed fixture there would put a bundled C library into the dependency graph of the crate whose whole value is that it needs nothing. |
| `MID_BATCH_FAULT` should be declined **explicitly, with the real reason**, in the same shape `MemoryFixture` uses for `REOPEN` — "so the skip is stated rather than inherited." | `_decomposition.md`, *Architecture brief*, §9; `crates/happenstance-testkit/src/fixtures.rs:274-284` | This adapter supplies the *reopen* far end and not the *fault* far end. An inherited default skip records nothing about that. |
| No fixture in this project may decline `SECOND_HANDLE`, and a green sequential suite with a skipped concurrency family "is not partial credit". | `_decomposition.md`, *Testing brief*, §4 | Constrains what "green whole" is allowed to mean here. |
| Positions may contain gaps; `AUTOINCREMENT` produces them after a delete; `GappedPositionStore` — a conformant store assigning positions in steps of seven from 4,096 — is what fails a rule that forgets. | `project.md`, risks; `crates/happenstance-testkit/README.md:118-122` | Any assertion this story writes compares against positions the store actually assigned. |

## Questions

Open questions to resolve before specifying.

1. **How does the fixture own its temporary file?** Deferred to `spec` with the
   manifest consequence stated: `tempfile` is **not** in
   `[workspace.dependencies]`, so adding it is a workspace change `cargo deny`
   sees (the gate runs it). A process-local ordinal plus a `Drop` cleanup needs no
   dependency and has precedent at
   `crates/happenstance-testkit/tests/fixture_instruments.rs:95-102`. Either is
   fine; "the manifest consequence is not optional to think about"
   (`_decomposition.md`, *Architecture brief*, §11).
2. **One `tests/conformance.rs` target or several?** Deferred to `spec` — the
   testing brief §7 allows either and declines to add a preference beyond
   "reachable from `tests/`, not from a `mod`". The concurrency family may want
   its own target for process isolation; that is HS-S0041's call.
3. **What are the three declared ceilings' numbers?** Consumed from
   `append-atomicity-and-store-limits`, not chosen here. They must be the same
   numbers `append` enforces, in the same units — a fixture declaring a ceiling
   the store does not enforce at exactly that byte fails
   `append_reports_exceeded_store_limits` in one direction or the other
   (`crates/happenstance-testkit/src/contract.rs:237-247`).
4. **Does the fixture decline anything besides `MID_BATCH_FAULT`?** Answered:
   nothing else, and `MID_BATCH_FAULT` is declined **explicitly with the real
   reason** rather than inherited from the default. `SECOND_HANDLE` and `REOPEN`
   are both `SUPPORTED`; declining either would forfeit AC-003 or AC-004.
5. **What does `reopen()` do, exactly?** Deferred to `spec`, with the property
   fixed: it must discard every outstanding handle's process-level state — drop
   the `rusqlite::Connection`(s) and reopen the same path — not delete and
   recreate the file, and not be a no-op. `MemoryFixture`'s comment records why
   both degenerate answers are worthless
   (`crates/happenstance-testkit/src/fixtures.rs:274-279`).
6. **CF-40's clause home.** Recorded as still open and escalated to the ADR queue,
   per AC-009 and `.kb/open-questions/cf-40-fixture-limits-ownership.md`. This
   story needs the *capability* and gets no say in the clause's home.
7. **The append-condition SQL strategy.** Consumed, not touched.

## Decision

This is the story where `happenstance-sqlite` either becomes an adapter or does
not: `CLAUDE.md`'s rule that matters says an adapter that compiles but has not run
the conformance suite is not an adapter, and nothing in the workspace has ever run
that suite against a file. The slice mounts `SqliteFixture` in
`crates/happenstance-sqlite/tests/` and takes `event_store_conformance!` green
whole. The spec will cover: one fixture instance owning one fresh temporary path,
with two instances getting two paths (AC-002); `connect()` calling
`SqliteEventStore::open(path)` — a **new** `rusqlite::Connection` — rather than
cloning a handle, and explicitly not `open_in_memory()` (AC-003); `SECOND_HANDLE`
and `REOPEN` declared `SUPPORTED` with `reopen()` dropping process state and
reopening the same path (AC-004); `MID_BATCH_FAULT` declined explicitly with the
real reason rather than by default; the three numeric ceilings stated as the same
facts `append` enforces, so `append_reports_exceeded_store_limits` runs rather
than skips (AC-009); and the run's own per-rule output captured as AC-001's
evidence, showing every entry in `for_each_event_store_rule!` as `Ran` or
`Skipped{reason}` and none absent. This story adds **no conformance rule** — it
supplies the fixture the existing rules run against — so the literal-position bar
is vacuous for it, and any adapter-owned assertion it writes compares against
positions the store assigned. No `[FROZEN]` clause is amended.

## The wrong implementation

**The mutant: `SqliteFixture(Arc<SqliteEventStore>)`, whose `connect()` returns a
refcount clone.**

```
struct SqliteFixture { store: Arc<SqliteEventStore> }
impl Fixture for SqliteFixture {
    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    fn connect(&self) -> impl Future<Output = Self::Store> {
        core::future::ready(SqliteHandle(Arc::clone(&self.store)))
    }
}
```

It passes **every rule in the suite**, including the MUST. It is a real file on
disk, so `acknowledged_writes_survive_a_reopen` can be made to pass too if
`reopen()` re-opens the path. `two_handles_observe_each_others_appends` passes
trivially, because one store observes itself. Isolation passes, because two
fixture instances still get two files. `cargo xtask ci --fast` is green, all four
macros are green, and the project can be declared done — while
`RUNBOOK.md:691`'s handle-multiplicity far end stays **exactly as empty as it is
today**: "every fixture still hands out refcount clones of one in-process object,
so no *connection* has been opened twice." The instrument portfolio would record a
far end filled by an implementation that never opened a second connection, and
CF-26's distinction between a fixture and a passing implementation would be
quietly inverted.

It is not a strawman. It is what `MemoryFixture` does — `core::future::ready(
MemoryHandle(Arc::clone(&self.0)))`, `crates/happenstance-testkit/src/fixtures.rs:286-291`
— and `MemoryFixture` is the reference implementation `CLAUDE.md` tells an adapter
author to read first. Copying the reference is the default behaviour, not a lapse.

**Why the testkit cannot catch it, and where the control therefore lives.** The
testkit deliberately cannot see inside an adapter's `connect`: that is the stated
reason isolation is a *rule* rather than a meta-test
(`crates/happenstance-testkit/src/contract.rs:17-23`), and the same limit applies
one level up — no rule can distinguish "two connections onto one file" from "two
clones of one connection", because both satisfy every observable the port exposes.
So the control is two things, both owed by this story:

1. **A review check with a named target**, recorded in `_ledger.md` for AC-003:
   `SqliteFixture::connect` calls `SqliteEventStore::open(path)`, and the fixture
   struct holds a `PathBuf`, not an `Arc<SqliteEventStore>`. AC-003's own wording
   requires this ("observable in the fixture's own code, not merely asserted").
2. **An adapter-owned test in `crates/happenstance-sqlite/tests/`** that the
   clone shape cannot pass: connect handle A, append through it, **drop A
   entirely**, then connect handle B and read the appended positions back. A
   refcount clone shares one `Mutex`-guarded connection, so the test is about
   whether a *second* connection can be opened at all and whether the first's
   commit is visible to it — and it compares against the positions A actually
   returned, never a literal range.

**The mistake that is caught, stated beside it so the two are not confused:** a
fixture built on `SqliteEventStore::open_in_memory()`
(`crates/happenstance-sqlite/src/event_store.rs:132`). A private in-memory
database is per-*connection*, so the second `connect()` opens a second, empty
database and `two_handles_observe_each_others_appends` **panics** rather than
skipping (`crates/happenstance-testkit/src/contract.rs:145`). That one the suite
finds immediately. The clone shape is dangerous precisely because it is the one
the suite cannot find.

**The third mutant, and it already has a negative control:** every fixture
instance pointing at one temporary directory or one fixed filename. That fails
`two_fixture_instances_observe_none_of_each_others_appends`, and
`MemoryFixture::sharing` exists in the testkit for exactly this — "handing every
instance the same `Arc` reproduces, in memory, the file-backed adapter that points
every fixture at one temporary path"
(`crates/happenstance-testkit/src/fixtures.rs:251-266`). Nothing further is owed
for AC-002; it is named here so the ledger cites the existing control rather than
building a second one.

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
