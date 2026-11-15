# #147: a removal capability on `Fixture` (`REMOVAL` + `remove_outside_port`)

## Goal

`happenstance-testkit`'s `Fixture` gains a capability through which a fixture deletes
events from its store **outside the port**, declined by default on `MID_BATCH_FAULT`'s
precedent. The SQLite, Postgres, Neon and Cloudflare fixtures (and the workerd harness
fixture) grant it with a raw `DELETE`. A new conformance rule in CF-39's shape fails any
fixture that declares the capability and does not actually remove what it names. That
rule is what makes #151 (ES-38) and #154 (ES-40) possible against the real adapters.

Owner decisions already taken (2026-11-15, `kb-decision-d0dce7c81f`): the name is
`const REMOVAL: Capability` plus `remove_outside_port(&self, positions: &[SequencePosition])`.
Cloudflare grants it if its fixture can issue raw SQL, which it can (`tests/support/mod.rs:385-417`
already runs `sql.exec` for the mid-batch trigger). Either way, its docs say that a Durable
Object wiped by `delete_all()` is a **different** store (VT-6, ADR-0086, ADR-0028 "Left open").

## Constraints and non-goals

- **Additive public API only.** The testkit is published, and phase 14 ships as `0.7.0`
  (`kb-decision` wi-7d6bd7bab0). `cargo-semver-checks` against `0.6.0` must stay silent: a
  defaulted associated const, a provided method, a new enum and a new `pub const` are all minor.
- **No new dependency**, and no `thiserror` in the testkit.
- **No `panic!` in library code (H-03).** The provided body returns `Removed::Nothing`, which is
  the `Armed` / `Reopened` / `Settled` pattern (`contract.rs:1691-1744`) and not the `panic!`
  pattern `arm_mid_batch_fault` and `arm_read_fault` still use (`contract.rs:514-558`). Overrides
  that live under `tests/` may panic on a broken environment, as `connect` documents
  (`contract.rs:565-571`). The workerd fixture lives in `src/`, so it records a harness fault
  instead.
- **Never `#[async_trait]`.** The method is spelled `-> impl Future` (`contract.rs:94-107`).
- **Out of scope:** #151's `positions_are_not_reused_after_removal` (ES-38), #154's
  `condition_over_removed_history_does_not_reject` (ES-40), #144's `WithholdingStore`, and any
  `EventStore` change (ADR-0028 decision 1: deletion stays outside the port through 1.x).
- **No spec marker moves.** CF-27 stays `[DEFERRED]` and ES-38 stays `[FROZEN]` with its body
  untouched: a `[FROZEN]` body is edited only through an ADR. #163 freezes. The only spec
  edits are the reconciliation of CF-27's text and its `Rule:` line (D2).
- **No runbook ticks and no ADR**, per `runbook/afk.md`.
- **Never assert a literal position.** Every position is one an `append` returned.
- The testkit must still compile on `wasm32-unknown-unknown`. Nothing here is thread-only.

## Invariants (each is a test below)

1. A fixture that says nothing about `REMOVAL` declines it, and the reason is
   `UNSTATED_REMOVAL`. A fixture that declares it and does not override the method gets
   `Removed::Nothing`.
2. Every rule gated on `REMOVAL` reports `Skipped { capability: "REMOVAL", reason }` against a
   declining fixture, carrying that fixture's own words (CF-18).
3. A fixture that declares `REMOVAL` and answers `Removed::Deleted` has removed **exactly** the
   named events. Through a handle the caller already held: `read(Query::all())` yields every
   other event, byte-identical (position, id, type, tags, data, metadata, recorded time), and
   none of the named ones. `contains_event_id` answers `false` for a removed event's id and
   `true` for a survivor's.
4. Removal is from the **same store**. It does not re-mint the `StoreId` and does not reset the
   position allocator. A survivor's `EventId` is unchanged, which the byte-identity in invariant
   3 already asserts.
5. The real adapters' raw `DELETE` leaves the position high-water mark where it was. The next
   append lands above every removed position. That is ES-38's content, so the per-adapter smoke
   test records it as evidence and leaves the rule to #151.

## Design

### Placement (ARC-02)

| What | Where | Why |
|---|---|---|
| `REMOVAL`, `remove_outside_port`, `Removed`, `UNSTATED_REMOVAL` | `crates/happenstance-testkit/src/contract.rs`, re-exported from `lib.rs:483-488` | Beside `HELD_ABOVE_FRONTIER` / `arm_held_above_frontier` / `Armed`, the closest precedent. Add the const and method **after** `settle_frontier` (the end of the trait, `contract.rs:730-733`) and add `Removed` and `UNSTATED_REMOVAL` **after** `Settled` (end of file). This keeps every `contract.rs:N` citation above them stable, as the frontier rule's placement did (`suite.rs:7111-7117`). |
| The guard rule | the end of `rules` in `crates/happenstance-testkit/src/suite.rs` (after line 7222) | The same citation-stability reason. |
| Rule enumeration | `crates/happenstance-testkit/src/registry.rs`, a new `// --- Removal outside the port (CF-27) ---` group at the end of `for_each_event_store_rule!` | The workerd dispatcher, the wasm emitter and `no_orphan_rules` all derive from this list. |
| Adapter grants | each adapter's `tests/support/mod.rs` and `harness/workerd/src/fixture.rs` | Fixtures are dev-only (`happenstance-cloudflare/tests/support/mod.rs:8-17`). No adapter `src/` changes. |

The dependency direction is unchanged: the testkit depends on core only, and adapters
dev-depend on the testkit.

### Public surface (exact)

```rust
// in `pub trait Fixture`, after `settle_frontier`

/// Whether this fixture can **remove events from its store outside the port** …
/// (full rustdoc: see "Rustdoc obligations" below)
const REMOVAL: Capability = Capability::declined(UNSTATED_REMOVAL);

/// Removes the events this store holds at `positions`, by a mechanism outside
/// `EventStore` — a raw `DELETE` on the adapter's own storage — and says whether it did.
/// …
fn remove_outside_port(&self, positions: &[SequencePosition]) -> impl Future<Output = Removed> {
    // `let _ = positions;` is load-bearing for the same reason as `reopen`'s
    // `let _ = self;` (contract.rs:591-594): the gate denies `unused_variables`.
    let _ = positions;
    core::future::ready(Removed::Nothing)
}
```

```rust
/// What [`Fixture::remove_outside_port`] reports it did.
/// … Deliberately exhaustive, like [`Armed`]: a removal either happened or it did not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use = "the rule asserts on this answer; dropping it hides a forgotten override"]
pub enum Removed {
    /// The override deleted every event the store held at the named positions, from
    /// the store's own storage, and nothing else.
    Deleted,
    /// Nothing was removed. Only the provided body returns this.
    Nothing,
}

/// What [`Fixture::REMOVAL`] says when a fixture does not.
pub const UNSTATED_REMOVAL: &str = "this fixture has not said whether it can remove events from \
     its store outside the port; `EventStore` has no delete (ADR-0028), so a removal has to come \
     from the adapter's own storage, and this is that answer given by default rather than by its \
     author";
```

**Ownership, parameter by parameter (OWN-01, OWN-03).**
- `&self`: a fixture is shared by every handle, and every sibling method takes `&self`.
- `positions: &[SequencePosition]`: read-only, and `SequencePosition` is `Copy`. A slice is the
  shape `WithholdingStore::withhold(&self, &[SequencePosition])` takes in #144
  (`plan-144.md`), so a fixture over that decorator forwards its argument unchanged.
  `impl IntoIterator` was rejected because it would make the trait method generic, and a
  generic method in a trait cannot take a provided body that a `LaggingFixture<F, _>` forwards
  without naming the type.
- Return: `impl Future<Output = Removed>`. In edition 2024 an RPIT in a trait captures every
  in-scope lifetime, so an override may hold `positions` across an `.await` (Postgres binds it
  to a query). This needs no annotation, and none is written.
- The future carries no `Send` bound, which is ADR-0001 at the declaration, as for every other
  `Fixture` method.

**Why `Removed` and not `Result`, and why not a count.**
- `Fixture` methods never return `Result`: a broken environment is not a non-conformant adapter
  (`contract.rs:565-571`).
- The only thing a rule must judge is whether the override ran. That is a two-valued fact, so
  the enum is exhaustive, like `Armed`, and not `#[non_exhaustive]`. A third variant would be
  an answer the rule could not judge.
- A count was rejected. The rule reads the store back, which is stronger than any count the
  fixture could report about itself.

**Contract on the argument** (stated in the method's docs, not checked by the trait):
- Every named position is one this store holds. A rule names only positions an `append`
  returned and that nothing has removed since.
- Duplicates are one removal (set semantics, as `withhold` dedupes).
- An empty slice removes nothing, and an override answers `Deleted`.
- An override MAY treat a position it does not hold as a broken test environment and panic,
  which is what the `tests/` overrides below do by comparing rows affected.
- Removal MUST reach a handle the caller already holds, as `arm_read_fault`'s arming must
  (`contract.rs:537-540`), because the rule connects first.
- Removal MUST NOT mint a new incarnation or reset the allocator. A wipe that does either (a
  Durable Object's `delete_all()`, `DROP SCHEMA` followed by a migration, `TRUNCATE … RESTART
  IDENTITY` on Postgres) makes a different store. That is VT-6's business and not removal.

### Rustdoc obligations (standards/rust/70)

`REMOVAL`'s docs carry the following sections, mirroring `READ_FAULT` (`contract.rs:344-387`):

- **What it is.** It names ES-38 and ES-40 as the clauses it exists to make checkable, and
  ADR-0028's "the capability, for sub-question 3".
- **"Why this is a fixture capability and not a port method."** ADR-0028 decision 1. A raw
  `DELETE`, a `TRUNCATE` and a `delete_all()` are invisible to the adapter, so the deletion has
  to come from the fixture's side of the storage.
- **"Why it is not the decorator."** CF-27's instrument *hides* events from a store that still
  holds them, so non-reuse holds there by delegation. Only a deletion in the adapter's own
  storage can show that its allocator survives one. That is the `INTEGER PRIMARY KEY` without
  `AUTOINCREMENT` shape ADR-0028 names. The decorator is named in prose, not linked, so that
  this doc does not depend on #144's merge order.
- **"What declaring it commits the fixture to."** It links
  `rules::removal_outside_the_port_removes_exactly_the_named_events`. It states CF-39's
  sentence one capability over: a fixture whose removal cannot reach the store's own storage
  MUST decline, rather than declare it and contribute a green result about a store nothing was
  removed from.
- **"Same store, not a new one."** The VT-6 paragraph above.
- **"Defaulted, and not checked by CF-18's emitted check."** This depends on D1. It explains
  that `UNSTATED_REMOVAL`'s words ("has not said … given by default") are true of every silent
  fixture, which is `UNSTATED_HELD_ABOVE_FRONTIER`'s argument (`contract.rs:1676-1683`).

`remove_outside_port`'s docs carry the argument contract above and "# The provided body says it
removed nothing". There is no `# Panics` section, because the provided body cannot panic. The
docs of `Removed` and `UNSTATED_REMOVAL` follow their siblings. Every new `pub` item has a doc
example or a link to the rule that uses it.

`assert_declensions_are_stated`'s docs (`contract.rs:1562`) and the comment above the
`UNSTATED_*` constants (`contract.rs:1398`) both say "Seven capabilities default to a
declension…". Both become eight, with a sentence saying why `REMOVAL` is not in
the list it checks (D1).

### The guard rule: `removal_outside_the_port_removes_exactly_the_named_events`

```rust
pub async fn removal_outside_the_port_removes_exactly_the_named_events<F: Fixture>(
    open: impl AsyncFn() -> F,
) -> RuleOutcome
```

Steps:
1. `require!(F: REMOVAL)`.
2. `open().await`, then `connect().await` **once**. The rule needs no second handle.
3. Append four single-event batches, `Kept`, `RemovedInterior`, `KeptBetween` and
   `RemovedTail`, each with its own tag. Keep the four positions the appends returned as `p0`
   to `p3`.
4. `settle!(fixture)` (CF-42), because the rule reads back its own writes.
5. `before = read_ok(&store, &Query::all(), ReadOptions::new())`. Assert that its positions are
   exactly `[p0, p1, p2, p3]`. This is a sanity anchor, and it compares against returned
   positions only.
6. `assert_eq!(fixture.remove_outside_port(&[p1, p3]).await, Removed::Deleted, …)`. The message
   names the forgotten override.
7. `after = read_ok(&store, &Query::all(), ReadOptions::new())` on the **same** handle. Assert
   `after == [before[0].clone(), before[2].clone()]`, using `get` and not an index (H-04). There
   are three distinct failure messages, chosen by inspecting `after`:
   - `after == before`: the removal did nothing. "Declared and answered `Deleted`, removed
     nothing."
   - a survivor is missing: over-removal. "`DELETE … WHERE position >= ?` is the discard shape,
     not removal of named positions."
   - a survivor's position or id changed: renumbering or rebuilding. "A store rebuilt from its
     survivors is a different store (ES-38 Rejects, VT-6)."
8. Assert `contains_event_id(before[1].id) == Ok(false)`, the same for `before[3].id`, and
   `contains_event_id(before[0].id) == Ok(true)`. An event removed from `read` but still
   reported held is half a removal.
9. `RuleOutcome::Ran`.

The removal is **interior and tail** together. The interior removal rejects a fixture that can
only truncate a suffix: the purge is scattered (ES-39's argument). The tail removal is the case
#151 needs, and a fixture that refuses it fails here, before #151 lands.

The rule does **not** assert anything about `head`, which is ES-38's and is a frontier on
Postgres and Neon. It does not assert anything about the next position either, which is #151's
rule.

Ripple, every one mechanical:
- `registry.rs`: the enumeration.
- `tests/mutation_coverage/lagging/inventory.rs`: `READS_BACK_ITS_OWN_WRITES`, and its count
  doc goes from 84 to 85.
- `tests/mutation_coverage/lagging/parity.rs`: only if its meta-test demands a row.
- `xtask/src/proof.rs` `CLOUDFLARE_WASM_RULES` (`proof.rs:822`) gains the rule, with a doc
  paragraph. Cloudflare grants the capability, so a regression to a decline would otherwise
  turn a `Ran` into a quiet skip, which is the argument `proof.rs:802-809` makes for the
  mid-batch pair.
- `spec/SPECIFICATION.md` CF-27: D2. Then run `cargo xtask spec-trace --write` to regenerate
  the §7.1 row at line 10724.

### Grants and defaults: every `impl Fixture for` (event-store `Fixture` only)

Measured with `grep -rn "impl.*Fixture for" crates harness examples`. `ProjectionFixture`,
`IngestFixture`, `SyncFixture`, `RestoreFixture` and `ConvergenceFixture` impls are a different
trait and are untouched. `happenstance-ladybug` is retired and excluded.

**Grant (`REMOVAL = SUPPORTED`, with an override):**

| Fixture | File | Mechanism |
|---|---|---|
| `SqliteFixture` | `crates/happenstance-sqlite/tests/support/mod.rs:130` | Open a second `rusqlite::Connection` on `self.path`, as `tests/append.rs::a_failure_mid_batch_leaves_nothing` does for its trigger. Give it a busy timeout of the shipped length. In one `BEGIN IMMEDIATE` transaction, for each distinct position: (a) `UPDATE tag_cardinality SET events = events - 1 WHERE tag IN (SELECT tag FROM event_tag WHERE position = ?)`; (b) `DELETE FROM event_tag WHERE position = ?`; (c) `DELETE FROM event WHERE position = ?`, asserting one row affected. Then `COMMIT`. Tag rows go first: `event_tag.position REFERENCES event(position)`, and `PRAGMA foreign_keys = ON` (`src/connection.rs:154`) would refuse the reverse. `tag_cardinality` is decremented so that a retention script copied from this fixture leaves the planner's selectivity statistic true (D6). `sqlite_sequence` is not touched. |
| `PostgresFixture` | `crates/happenstance-postgres/tests/support/mod.rs:419` | `sqlx::query("DELETE FROM event WHERE position = ANY($1)").bind(&positions_i64).execute(&pool)` on `self.pool().await`, whose `search_path` is the instance's schema. Assert `rows_affected` equals the distinct count. Use one table: tags are a `text[]` column on `event`, so there is no tag table. The doc says why `DELETE` and never `TRUNCATE … RESTART IDENTITY`: `event_position_seq` is `OWNED BY event.position` (`migrations/0001_event_log.sql`), so `RESTART IDENTITY` would rewind it. |
| `NeonFixture` | `crates/happenstance-neon/tests/support/mod.rs:117` | `self.migrate().await`, then a single statement through the file's existing `execute` helper (`support/mod.rs:662`): `DELETE FROM {self.config.qualified_event()} WHERE position = ANY($1::bigint[])` with `SqlStatement::with_params`. Check the API in `src/transport.rs` against source before writing it. Assert the affected-row count if the response exposes one; the guard rule is the backstop. The `TRUNCATE` note is the same as Postgres's, and so is the Neon-branch note (`kb-open-question-postgres-neon-store-id-no-restore-001`): a branch reset rewinds under the same `StoreId`, and that is not removal. |
| `CloudflareFixture` | `crates/happenstance-cloudflare/tests/support/mod.rs:167` | `let sql = self.sql.borrow();` For each distinct position: `DELETE FROM event_tag WHERE position = ?`, then `DELETE FROM event WHERE position = ?`, with `.expect` per statement (test code). This is the order of `CloudflareEventStore::discard_from` (`src/event_store.rs:1003-1030`), and it is that function's exact-set sibling and not a copy: `discard_from` deletes `>= from`, which is the over-removal mutant below. The VT-6 paragraph is mandatory (see below). |
| `WorkerdFixture<'_>` | `harness/workerd/src/fixture.rs:107` | The same two statements against `{ns}_event_tag` and `{ns}_event`, through `current_sql()` and the fixture's recording `exec`. It answers `Removed::Deleted` only if every statement succeeded. Otherwise it records the fault (with "removing outside the port" in the message, not the hard-coded "arming the mid-batch fault" `exec` uses today, so `exec` takes a `what: &str`) and answers `Removed::Nothing`. The dispatcher reports the harness fault beside the rule's failure. This is `src/` code, so it has no `expect` and no `panic!` (H-02, H-03). It is the only place the guard and #151's rule run on a real Durable Object's `sqlite_sequence` (D4). |

**The Cloudflare VT-6 paragraph.** It goes in the docs of both Cloudflare fixtures and is
paraphrased into each one's `REMOVAL` doc:
- "This is a removal from the **same** store. `store_meta` and `sqlite_sequence` are not
  touched, so the `StoreId` and the `AUTOINCREMENT` high-water mark both survive it."
- "A Durable Object wiped by `ctx.storage.deleteAll()` (`delete_all()`,
  `worker-0.8.5/src/durable.rs:449`) is a **different** store. Every table goes, `store_meta`
  with them, so the next `migrate` mints a new `StoreId`. `AUTOINCREMENT` restarts with it, and
  positions restart under the new incarnation (VT-6, ADR-0086, ADR-0028:212-213)."
- "A checkpoint held elsewhere can then sit above the new head. That is a checkpoint and
  `StoreId` mismatch for the typed runner, and the remedy is `reset`. It is not ES-38's case,
  and this capability does not model it."

**Decline with a stated reason (the default is overridden by words):**

| Fixture | File | Reason |
|---|---|---|
| `MemoryFixture` | `crates/happenstance-testkit/src/fixtures.rs:272` | "MemoryEventStore offers no way to delete an event, and it allocates the next position from its log's length, so any removal reached from outside it would hand a removed position out again: that is ES-38's named wrong implementation, not a removal. Removal in memory is staged by CF-27's completeness instrument, a decorator that hides events from a store that still holds them." This is D5. Its Store type stays `MemoryHandle` (public API). |
| `DecliningFixture` | `crates/happenstance-testkit/tests/mutation_coverage/variants.rs:846` | Add `REMOVAL_REASON`: "this instrument declines everything, and its store is a Vec behind an Rc with no storage of its own to delete from in any case". Its doc says it declines *everything*, and `capability_skips_are_reported` reads its reasons back. |

**Forward (wrappers whose constants already forward the inner fixture's):**

| Fixture | File | Change |
|---|---|---|
| `LaggingFixture<F, M>` | `tests/mutation_coverage/lagging.rs:303` | `const REMOVAL = F::REMOVAL;`, and `remove_outside_port` forwards to `self.inner`. |
| `NeverAsked<F>` | `tests/mutation_coverage/lagging/never_asked.rs:17` | The same. |

`Declined<S>` (`trailing.rs:467`) restates only what it changes and forwards nothing else, so it
keeps the default. The model family never removes.

**Grant, in the testkit's own tests (the honest control):**

`GappedPositionFixture` (`tests/mutation_coverage/variants.rs:276`) grants it. That fixture
is "the one fixture in the binary that supports *everything*" (`variants.rs:282-285`), and
`capable_fixtures_skip_only_what_they_decline` (`mutation_coverage.rs:5205-5230`) asserts
that its only skip is the frontier rule. Granting it here keeps that test's expectation
**unchanged** (H-12). The alternative, a new control, would have changed the expectation, which
is an H-16 item. Granting needs three changes:

1. **`correct.rs` `Log`** gains `high_water: Option<SequencePosition>`, the highest position
   ever assigned, and these methods:
   - `pub(crate) fn remove(&mut self, positions: &[SequencePosition]) -> usize`. Before
     retaining, it records `high_water = max(high_water, events.last().position)`.
   - `pub(crate) fn snapshot(&self) -> Self` (or `Clone` on `Log`), for the durable side.

   `commit` gains a sibling, `commit_from(stored, events, condition, allocate, floor:
   Option<SequencePosition>)`, whose head is `max(stored.last(), floor)`. `commit` calls it with
   `None`, so **no existing mutant changes**. `Log::append` calls `commit_from(…, self.high_water)`.
2. **`GappedPositionFixture`'s durable side becomes the restored `Log`, not replayed `Event`s.**
   `committed: Rc<RefCell<Log>>` is set to the live log's snapshot after each successful
   append and after each removal, and `reopen` installs a clone. This is the change the
   variant's own doc anticipates (`variants.rs:118-123`: "give the durable side
   `SequencedEvent`s rather than … weaken the rule"). It also stops a replay renumbering the
   survivors of a removal. A renumbering replay is exactly what ES-38 forbids, and #151 would
   catch it.
3. **The override.** `remove_outside_port` removes from both `live` and `committed`, asserts the
   removed count equals the distinct count, and answers `Deleted`.

**Wrong fixtures (mutants), one defect each, in `tests/mutation_coverage/mutants.rs`:** each
one is over a correct dense `Log`, declares `REMOVAL`, declines `REOPEN` with a reason, and is
registered in `for_each_mutant!`, `REGISTRY` (`fails: &[the guard]`, with an `expect` pin per
row) and `MODEL_COVERAGE` (`ModelOutcome::Agreed`, because the model family never removes).

| Mutant | Defect | Who writes it | Pinned message |
|---|---|---|---|
| `ForgottenRemovalFixture` | declares `REMOVAL` and leaves the provided body | the author who declares first and wires later | "MUST override `remove_outside_port`" |
| `NoopRemovalFixture` | overrides with an empty body that answers `Deleted` | `NoopFaultFixture`'s author, one capability over (CF-39's shape) | "removed nothing" |
| `OverRemovingFixture` | removes every position `>= min(positions)` | someone copying `CloudflareEventStore::discard_from`, the workspace's only existing raw delete | "a survivor is missing" |
| `RenumberingFixture` | rebuilds the log from its survivors through `append` | the obvious removal on a store with no delete; ES-38's *Rejects* | "a survivor's position or id changed" |

A fifth mutant, a removal that leaves membership answering `true` (a separate id index), is
**optional**. It needs a store whose `contains` reads a second structure. Add it only if it
is a one-step `Defect` override.

**Keep the default (no change; silence is honest because D1 does not check it):**
- In the testkit tests: `SilentFrontier`, `StatedFrontier` and `NoFrontier`
  (`frontier_fixture_declension.rs`); the `racers.rs:487` macro fixtures; `MutantFixture<D>` and
  every other mutant in `mutants.rs` (`YieldingRowAtATime`, `NoTransaction`, `NoopFault`,
  `UnarmedFrontier`, `EmptyArmFrontier`, `NoopReopen`, `HonestVolatile`, `Closing`,
  `LiveHandleReopen`, `CachedHead`, `LastWrittenHead`, `Losing`, `Restamping`, `SharedBacking`,
  `PreCommitPosition`, `PollPaddedPosition`, `BorrowHolding`, `AwaitAcrossBorrow`,
  `RefetchingPaged`, `SwallowedReadFault`); `TrailingFrontierFixture<T>`; `Declined<S>`;
  `PagedStreamFixture`; `FrontierFixture<M>`; `LocalFixture` (`local_conformance.rs:370`);
  `FlakyReadFixture`; `GappyFixture`; `BatchCeilingFixture`; `SeedViolatesFixture`;
  `FaultyFixture`; `ContendedFixture`; `DurableFixture`.
- In the adapters: `ImpatientFixture` (`happenstance-sqlite/src/event_store.rs:3852`),
  `AppendOnlyFixture` (`happenstance-sqlite/tests/append.rs:101`), `Defaulted`
  (`happenstance-postgres/tests/postgres_conformance.rs:395`, and
  `happenstance-neon/tests/neon_conformance.rs:372`; both exist to *name* the defaults), and
  `DecliningFixture` (`happenstance-cloudflare/tests/fixture_contract.rs:74`, which is handed
  only two named rules).
- In the sync testkit: `OracleFixture`
  (`happenstance-sync-testkit/tests/memory_ingest_event_store_conformance.rs:23`).

`GappyMemoryStore` would pass ES-38 (it allocates from a counter, `gappy.rs:11-13`), but it has
no removal API, and adding one is a published-API addition no rule needs yet. It keeps the
default.

**#144's `PrunedMemoryFixture`** (test-only, `tests/completeness_instrument/fixture.rs`) MAY
grant `REMOVAL` by forwarding to `WithholdingStore::withhold`. An `Err` from `withhold` panics
in test code. That puts the guard, and later ES-38 and ES-40, on the decorator as well. Do it in
whichever of #144 and #147 merges second, or leave it to #151. It is not a dependency of this
issue.

### ES-38 already? (schema reading, confirmed by each smoke test's last assertion)

| Store | Allocator | Survives a raw `DELETE`? | Caveat named in the fixture doc |
|---|---|---|---|
| SQLite | `INTEGER PRIMARY KEY AUTOINCREMENT` (`src/event_store.rs:300-301`, rationale `:113-116`) | **Yes.** `sqlite_sequence` keeps the high-water mark, and neither `VACUUM` nor deleting every row resets it. | Only `DELETE FROM sqlite_sequence` or a rebuilt file rewinds it, and both are a different store. |
| Cloudflare (shim and workerd) | the same `AUTOINCREMENT` (`src/event_store.rs:218-219`, `:49-52`) | **Yes.** `discard_from` already relies on it (`:1014-1019`). | `delete_all()`: a different store (VT-6). The ingest `SAVEPOINT` emulation rewinds the counter only for a rolled-back, never-acknowledged write (`src/event_store/ingest/model.rs:28`). |
| Postgres | `event_position_seq`, `nextval` (`migrations/0001_event_log.sql`) | **Yes.** A sequence never rewinds on `DELETE`. | `TRUNCATE … RESTART IDENTITY` and `setval` rewind it, and so do `pg_restore` and a Neon branch reset, under the **same** `StoreId` (open question, phase 13). |
| Neon | the same migration, templated (`src/migration.rs`, `@sequence@`) | **Yes.** | The same as Postgres. |
| `MemoryEventStore` | the log's length | **No.** It would reuse positions, which is ES-38's named wrong implementation. | It declines (D5). |

### Concurrency and resources

- No new tasks, channels or locks in library code.
- The SQLite override opens one extra connection and drops it before returning. It runs under
  `BEGIN IMMEDIATE` with the shipped busy timeout, so a rule that held a read statement open
  would make it wait, not fail. No rule does that across a removal.
- The Postgres override uses the fixture's pool. Neon's uses one HTTP round trip.
- `CloudflareFixture`'s `RefCell` borrow is taken and dropped inside the synchronous body and is
  never held across an `.await` (H-14). The future is `core::future::ready`.

### Alternatives considered

- **`panic!` in the provided body**, as `arm_mid_batch_fault` does. Rejected: the H-03 hook
  refuses it, and the `Armed` pattern catches a forgotten override on every run.
- **`Result<(), RemovalError>`.** Rejected: it puts environment failure in the conformance
  channel (`contract.rs:565-571`). It would also need a hand-written error type in the testkit
  for a case the rule cannot act on.
- **A new honest control in place of granting on `GappedPositionFixture`.** Rejected: it
  changes `capable_fixtures_skip_only_what_they_decline`'s expectation.
- **Granting on `MemoryFixture` through `WithholdingStore`.** Rejected: it changes
  `MemoryFixture::Store`, a public associated type, which is a breaking change, and it turns the
  reference fixture into an instrument (D5).
- **A truncate-only capability (`remove_through(position)`).** Rejected: a floor is the shape
  ES-39 rejects, and the owner fixed the slice signature.

## Test list (drive in this order)

Testkit contract (`crates/happenstance-testkit`):
- [x] **T1.** In a new `tests/removal_declension.rs` (native-only, mirroring
  `frontier_fixture_declension.rs`), a fixture silent on `REMOVAL` reads
  `REMOVAL.reason() == Some(UNSTATED_REMOVAL)`. Red first: it fails to compile against the
  missing const.
- [x] **T2.** Same file: `remove_outside_port` on that fixture answers `Removed::Nothing`, via
  `block_on`.
- [x] **T3.** Same file: `assert_declensions_are_stated` accepts a fixture that is silent on
  `REMOVAL` (D1 = A). If D1 = B, this becomes "rejects", plus a "stated is accepted" corner.
- [x] **T4.** The guard rule exists and skips. `capability_skips_are_reported` over
  `DecliningFixture` shows `removal_outside_the_port_removes_exactly_the_named_events` as
  `Skipped { capability: "REMOVAL", reason: DecliningFixture::REMOVAL_REASON }`. This
  drives the rule's `require!` gate, the registry entry and `REMOVAL_REASON`.
- [x] **T5.** `ForgottenRemovalFixture` fails exactly the guard, on the `Removed::Deleted`
  assertion (registry row and `expect` pin). This drives step 6.
- [x] **T6.** `NoopRemovalFixture` fails exactly the guard, on "removed nothing". This drives
  step 7's first arm.
- [x] **T7.** `OverRemovingFixture` fails exactly the guard, on "a survivor is missing".
- [x] **T8.** `RenumberingFixture` fails exactly the guard, on "a survivor's position or id
  changed".
- [x] **T9.** `GappedPositionFixture` grants `REMOVAL` and passes the whole suite, the guard
  included. `capable_fixtures_skip_only_what_they_decline` stays green and unchanged.
  `acknowledged_writes_survive_a_reopen` and `recorded_time_survives_a_reopen` stay green
  through the restore-not-replay change. Drive the `Log::high_water` change with a unit test
  in `correct.rs` first: after removing the tail, `Log::append` assigns above it. This test is
  the honest twin of #151's future mutant.
- [x] **T10.** The lagging meta-tests (`the_read_back_inventory_partitions_the_named_rules`,
  `an_unsettling_lag_skips_exactly_the_read_back_rules`, `settling_costs_no_mutant_coverage`)
  are green with the rule in `READS_BACK_ITS_OWN_WRITES`. The `LaggingFixture` and
  `NeverAsked` forwarding is what makes them so.
- [x] **T11.** `every_rule_has_a_mutant`, `mutant_registry_is_exhaustive`,
  `mutants_fail_exactly_their_declared_rules` and the model-coverage table are green.
- [x] **T12.** The `registry::no_orphan_rules` unit test and the wasm emitter
  (`memory_conformance_wasm.rs`) list the rule. `MemoryFixture` reports the skip with its own
  reason.

Per-adapter smoke tests (each in a target that already declares `mod support;`, and each using
only the trait and the port, so the support module grows no item that is dead in its other
declaring targets):
- [x] **T13. SQLite**, in `tests/conformance.rs`:
  `removal_outside_the_port_deletes_the_row_and_keeps_the_allocator`.
  1. Append three events with distinct tags and remove the middle and the tail.
  2. `read(Query::all())` is the first event only, and a tag query for a removed event's tag
     yields nothing.
  3. A fresh append's `id.store()` equals the survivor's: same incarnation.
  4. Its position is above the removed tail. This records ES-38 for this adapter and leaves the
     rule to #151.
- [ ] **T14. Postgres**, in `tests/postgres_conformance.rs`: the same four assertions, live
  only (the target's existing gating), with `settle_frontier` before each read, because the
  fixture declines `READ_YOUR_OWN_WRITES`. Extend
  `capability_constants_are_answered_not_defaulted` (no server) with
  `assert!(PostgresFixture::REMOVAL.is_supported())`.
- [ ] **T15. Neon**, in `tests/neon_conformance.rs`: the same as T14, plus the no-server
  constant assertion.
- [ ] **T16. Cloudflare**, in `tests/fixture_contract.rs`:
  1. Add `REMOVAL` to `a_supported_capability_has_its_method_overridden` (wasm), asserting
     `Deleted` and the read-back.
  2. Add a `#[wasm_bindgen_test]`, `removal_keeps_the_objects_identity_and_counter`. After the
     removal, a new append has the same `StoreId` and a position above the removed tail. This is
     the executable half of the VT-6 paragraph.
  3. Add `REMOVAL` to `declared_capabilities()`. It is supported, so the vocabulary check skips
     it, but the list stays complete.
- [ ] **T17. workerd.** No new Rust test. The guard runs through `dispatch` by name
  (`harness/workerd/test/conformance.test.ts` enumerates `/rules`). A plain `cargo check` on
  `harness/workerd` for wasm32 is enough here. The `workerd` CI job is where it runs.

Gate and documentation:
- [x] **T18.** `cargo xtask spec-trace` is green after D2's edit and `--write`.
  `cargo xtask lint-constitution` is green. `cargo test -p happenstance-testkit --doc` passes,
  so every new doc example compiles.
- [x] **T19.** The doc builds at all three `happenstance-testkit` configurations the gate runs.
  No broken intra-doc link to `rules::removal_…`: check it with `--no-default-features`.
X `CHANGELOG.md` `[Unreleased]`, under `happenstance-testkit` / Added:
  `Fixture::REMOVAL`, `Fixture::remove_outside_port`, `Removed`, `UNSTATED_REMOVAL`, and the
  rule. One line says that a fixture which says nothing skips the rule, so no existing adapter's
  CI changes colour. Under the four adapters: "the conformance fixture grants `REMOVAL`" (one
  line, no API change). `docs/pass-conformance-suite.md` gains a `REMOVAL` paragraph beside
  `MID_BATCH_FAULT` / `READ_FAULT` (`:79-90`), with the VT-6 sentence.

Disk is tight: drive T1 to T12 with `cargo test -p happenstance-testkit --test mutation_coverage`
and `--test removal_declension` (`CARGO_INCREMENTAL=0`). T13 is
`cargo test -p happenstance-sqlite --test conformance`. Leave T14, T15 and T16 to their CI jobs
(`live-postgres`, `live-neon`, `wasm`).

## Decisions (H-16)

| # | Decision | Door | Options | Recommendation | Decided by | Outcome |
|---|---|---|---|---|---|---|
| D0 | Name and shape | one-way (published trait) | as given | `REMOVAL` + `remove_outside_port(&self, &[SequencePosition]) -> impl Future<Output = Removed>`, `Removed { Deleted, Nothing }` exhaustive, `UNSTATED_REMOVAL` | owner chose the name, 2026-11-15; the return enum was delegated | Return shape chosen here, on the `Armed` precedent. Flag it if you want a different enum name. |
| D1 | Does CF-18's `assert_declensions_are_stated` reject a fixture that is silent on `REMOVAL`? | two-way in code, but either flip changes downstream CI colour | **A.** No, the default's words are true of silence (`HELD_ABOVE_FRONTIER`'s argument). **B.** Yes, unconditionally, like `MID_BATCH_FAULT` and `READ_FAULT`: every third-party fixture turns red on `0.7.0` until it states a reason, and about 12 in-workspace suite-mounting fixtures need a stated reason. **C.** Conditionally (no natural condition exists). | **A** | owner | open |
| D2 | Which clause owns the guard rule (spec-trace check 6)? | two-way (CF-27 is `[DEFERRED]`, editable until #163) | **A.** Add it to CF-27's `Rule:` line as `(new)`, and reconcile CF-27's capability sentence to name `Fixture::REMOVAL` / `remove_outside_port`. **B.** Mint a new CF clause mirroring CF-39 (adds a row to #163's freeze set). **C.** List it as owing a decision in `spec_trace.rs`, as the read-fault rule is (debt). | **A**: ADR-0028 already places the capability inside CF-27 | owner (spec) | open |
| D3 | `MemoryFixture` | two-way (declining now, granting later stays additive only through a non-breaking route) | **A.** Decline with the length-allocation reason. **B.** Grant through `WithholdingStore` (breaks `MemoryFixture::Store`). **C.** Add removal to `MemoryEventStore` in core (a core API addition, and it still needs a high-water mark). | **A** | architect (H-16-adjacent: published reason text) | recommended |
| D4 | The workerd harness grants | two-way (never published) | grant / keep the default | **grant**: the only real-Durable-Object run of the guard and of #151 | architect | recommended |
| D5 | The honest control | two-way (tests only) | grant on `GappedPositionFixture` (restore-not-replay) / a new control (changes a test expectation) | **grant on `GappedPositionFixture`** | architect | recommended |
| D6 | SQLite `tag_cardinality` on removal | two-way (fixture only) | decrement / leave it stale | **decrement** (planner statistic stays true; no rule can see either) | architect | recommended |

Only **D1** and **D2** need the owner. Neither is a one-way door. **D0** is the one one-way door,
and the owner has already taken it, except for the enum name, which is delegated and flagged.

## Risks

- **Merge conflict with #144** in CF-27's text, `CHANGELOG.md` `[Unreleased]` and `lib.rs`'s
  export list. Rebase whichever lands second. Neither issue uses the other's types.
- **The restore-not-replay change to `GappedPositionFixture`** touches the most-used conformant
  control. `mutants_fail_exactly_their_declared_rules` and the variant's own full-suite run are
  the net. If a mutant's declared set shifts, the control changed behaviour; stop and look, and
  do not edit the row.
- **ES-38's `[FROZEN]` body still says "`Fixture` declares no such capability"**
  (`SPECIFICATION.md:4873-4877`). After this change that is stale, and it cannot be edited here
  (an ADR is needed). #151 or #163 owns it. Note it in the PR.
- **Write groups (SY-39).** A raw delete of a group's first row drops `group_guard`
  (`write_group` / `group_guard` columns, migration 2 / `0003_write_group.sql`). No #147 rule
  removes part of a group. #158 and ingest rules that remove must pick whole groups. Say so in
  `REMOVAL`'s doc as a sentence on the argument contract: "removal of part of a write group is
  outside what ES-38 describes".
- **Cloudflare foreign keys.** The tag-rows-first order is right whether or not the object
  enforces `REFERENCES`. T16 is the only proof on the shim. On workerd it is the `workerd` job.
- **The support module compiled per target.** A helper added to `SqliteFixture`,
  `PostgresFixture` or `NeonFixture` for a smoke test would be dead code in the other declaring
  targets (Postgres has nine). The smoke tests therefore use only the trait and the port.
- **Neon transport API** (`SqlStatement::with_params`, `SqlRequest::…` single-statement
  constructor). It was read at the type level only. Verify it against `src/transport.rs` before
  writing.
- **Downstream CI.** With D1 = A, the only colour change for a third-party adapter is a new
  `SKIP removal_outside_the_port_removes_exactly_the_named_events: <UNSTATED_REMOVAL>` line.

## Size (lower bound)

About 26 files and about 1,300 lines, most of them docs and test code:
- testkit `src` (contract, suite, registry, lib, fixtures): about 330
- testkit `tests` (correct, variants, mutants, `mutation_coverage.rs` rows, lagging,
  never_asked, a new declension test): about 520
- adapters (four `support` overrides with docs, four smoke tests, two constant asserts,
  `fixture_contract`): about 380
- workerd: about 50
- xtask `proof.rs`: about 10
- spec, CHANGELOG, docs: about 50

## Progress

- 2026-11-15: plan written (architect). Nothing implemented.
- 2026-11-15: implemented (implementer). T1–T13 and T18–T20 done and green.
  Red evidence: T1–T3 failed to compile against the missing `REMOVAL`,
  `remove_outside_port`, `Removed` and `UNSTATED_REMOVAL`; T4 failed in
  `capability_skips_are_reported` (rule absent from `MUST_SKIP`); T5–T8 failed
  in `mutants_fail_exactly_their_declared_rules` before their rows existed. The
  `Log::high_water` unit test (`a_removed_tail_is_not_handed_out_again`) was
  written with the fix and not observed red on its own.
  T14 (Postgres), T15 (Neon), T16 (Cloudflare wasm) and T17 (workerd) compile,
  including the wasm32 checks; their live legs were not run here.
  Defaulted: D1 = A, D2 = A (owner). D3/D5: `MemoryFixture` declines,
  `GappedPositionFixture` grants, restoring a durable copy at reopen rather than
  replaying. D4: workerd grants. D6: SQLite decrements `tag_cardinality`.
  `PrunedMemoryFixture` declines (hiding is not deleting). Answer enum named
  `Removed { Deleted, Nothing }`.
