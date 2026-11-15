# #151 + #154: ES-38's and ES-40's rules over a removed-from store

Branch `claude/project-thread-xxl8lr-151` at `origin/phase-14` `5f5f8fea`. Architect's plan, 2026-11-15.
Nothing is implemented yet.

## Goal

Two conformance rules join the event-store suite. Both run against CF-27's instrument and against
every fixture that grants `Fixture::REMOVAL`:

- **`positions_are_not_reused_after_removal`** (ES-38, #151). After a removal outside the port,
  the store never hands a removed position out again: not after a tail removal, and not after
  removing everything. `read(Query::all())` and `head` describe exactly what the store still
  holds, and a removed identity stays unheld.
- **`condition_over_removed_history_does_not_reject`** (ES-40, #154). A condition whose only
  match was removed **admits** the append (the vacuous pass, ADR-0028 decision 2). A condition
  over a surviving event still refuses.

Each rule ships with the wrong stores in `crates/happenstance-testkit/tests/` that it rejects.
ES-38's named wrong implementation, a rowid table without `AUTOINCREMENT`, is among them.
`AppendCondition`'s rustdoc names the ES-40 rule as the check behind the vacuous pass.

## Constraints and non-goals

- **No `Cargo.toml` edits and no new dependencies.** Everything here uses what the testkit,
  core, sqlite and xtask already depend on. New files under `tests/` are picked up by autotests,
  or are modules of an existing target.
- **Additive.** Two new `pub async fn` in `happenstance_testkit::rules` (a minor change). The
  rest is test code, docs, xtask data and spec prose. No `Fixture` or `EventStore` surface
  changes. `cargo-semver-checks` stays silent against `0.6.0`.
- **Spec edits are line-neutral**, and `cargo xtask spec-trace` stays green. **ES-38's
  `[FROZEN]` body is not touched** (D2). No marker moves: ES-40 stays `[PROVISIONAL]` and CF-27
  stays `[DEFERRED]`. Freezing them is #163's job.
- **No runbook ticks, no ADR, no CLAUDE.md edit** (`runbook/afk.md`).
- **Never assert a literal position.** Every position compared is one an `append` returned or a
  `read` yielded.
- **Never `#[async_trait]`.** Nothing here adds a trait method.
- Out of scope: #156's reader experiment, #158's SY-32, #163's freeze, any change to
  `is_violated_by`.

## Invariants (each one is asserted by a rule step or a test below)

1. A removed position is never assigned again: not after removing the tail, and not after
   removing every event the store holds.
2. After a removal (once settled), `read(Query::all())` yields exactly the survivors, at their
   assigned positions. `head` equals the last of them, or `None` if none survive.
3. A removed event's `EventId` stays unheld after later appends.
4. A condition whose only matching event was removed admits the append, and the admitted event
   reads back at the position the append returned.
5. A condition matching a survivor still refuses after the removal. This holds for a survivor
   below the hole and for one above it.
6. Every mutant fails **exactly** its declared rules, with the pinned message, and every
   conformant control passes both rules.

## Design

### Placement (ARC-02, ARC-06)

| What | Where | Why |
|---|---|---|
| Both rules, plus one shared helper | **end of** `crates/happenstance-testkit/src/suite.rs`, inside `pub mod rules`, after `assert_membership_follows_removal` (current last item, ends at line 7416) | spec-trace resolves event-store rule names only in `suite.rs` (`xtask/src/spec_trace.rs:71`, `RULE_FILES[0]`). The end of the module keeps every existing `suite.rs:N` citation stable. **Deviation from ARC-06/CRF-02** (the file is already 7.4k lines): splitting it would need an xtask `RULE_FILES` change and would move citations. Not worth it in a leaf. |
| Rule enumeration | `src/registry.rs:229-230`, inside the existing `// --- Removal outside the port (CF-27) ---` group, after the guard | The workerd dispatcher, the wasm emitter, the completeness record and `no_orphan_rules` all derive from this list. |
| `REMOVAL` rustdoc amendment (D1) | `src/contract.rs:752-758` ("Why it is not the decorator") and `:768-771` (the CF-39 sentence) | **Line-neutral**, so that no `contract.rs:N` citation below it moves (ProjectionFixture and the rest). If it cannot be line-neutral, run `cargo xtask lint-constitution`, `spec-trace` and `grep -rn 'contract.rs:'`, and repoint. |
| Instrument grant (D1) | `tests/completeness_instrument/fixture.rs` (`PrunedMemoryFixture`) plus `pass_list.txt` | See D1. |
| The new wrong stores | **new** `tests/mutation_coverage/removed_history.rs`, declared `pub(crate) mod removed_history;` at the end of `tests/mutation_coverage/mutants.rs`, after `pub(crate) mod removal;` | `removal.rs`'s pattern (a module of its own, declared at the end so that no cited line moves and no `#[path]` is needed). |
| `Log` sub-steps | `tests/mutation_coverage/correct.rs`, inside `impl Log`, **below** line 545 | `correct.rs:479` is cited by `standards/rust/00-prime-directives.md:309`. Nothing is inserted above that line. |
| SQLite fidelity test (D6) | `crates/happenstance-sqlite/tests/migration.rs`, after `assert_event_table` | It already imports `rusqlite::Connection` and asserts the `AUTOINCREMENT` keyword (`:221-228`). |
| `AppendCondition` doc | `crates/happenstance-core/src/append.rs`, inside the "A claim about one store's log" section (`:29-47`) | ES-40's documentation half. |

The dependency direction is unchanged. Core gains no dependency: the doc names the testkit rule
in code format, **not** as an intra-doc link, because core cannot link into a crate it does not
depend on, and `rustdoc -D warnings` would fail on the broken link.

### The shared helper (suite.rs, private)

```rust
/// Asks the fixture to remove `positions` outside the port and fails, naming the forgotten
/// override, unless it answers `Removed::Deleted`.
async fn remove_outside_port_or_fail<F: Fixture>(fixture: &F, positions: &[SequencePosition])
```

The assertion message is the guard's message today, **verbatim** (`suite.rs:7313-7319`,
starting "the answer: a fixture declaring `REMOVAL` supported MUST override
`remove_outside_port` …"). The guard switches to the helper, so the three rules share one
message and `ForgottenRemovalFixture`'s pin ("MUST override `remove_outside_port`") holds in
all three. Ownership: `&F` because the fixture is shared and every sibling helper borrows it;
`&[SequencePosition]` because `remove_outside_port` takes that type (OWN-01). Both positions are
`Copy`, and the call sites pass array literals.

### Rule 1: `positions_are_not_reused_after_removal<F: Fixture>(open: impl AsyncFn() -> F) -> RuleOutcome`

Rustdoc must cover: ES-38 and ADR-0028 decision 2; the named wrong implementation and every
mutant below; the line "passes on CF-27's instrument by delegation (the store under the decorator
still holds every withheld event), so the instrument's pass is not evidence about any allocator;
the adapters' passes are"; and "asserts nothing about positions before the first removal, which
are other rules' business".

Steps. Tags are `("reuse", "<name>")` and event types are distinct, so that a failure message
can name events by type.

1. `require!(F: REMOVAL)`. `fixture = open().await`, then `store = fixture.connect().await`,
   **once**.
2. `a`, `b`, `c` = `append_ok` of three single-event batches: `BeforeTheTail`, `Survivor`,
   `RemovedTail`. Use single-event batches, never a multi-event batch. A batch is not a write
   group, but keeping each batch to one event leaves the SY-39 partial-group caveat on
   `remove_outside_port` vacuous here.
3. `settle!(fixture)`. `before = read_ok(&store, &Query::all(), ReadOptions::new())`. Anchor:
   `positions_of(&before) == [a, b, c].map(get)`. Bind `[_, _, removed_tail] = before.as_slice()`
   with `if let`, as the guard does (H-04).
4. **Stage 1: the tail.** `remove_outside_port_or_fail(&fixture, &[c])`.
5. `settle!`. `after_tail = read_ok(all)`.
   - Assert `positions_of(&after_tail) == [a, b]`. Message: **"the removal did not take"**, and
     it continues: the rule judges only a store the named events have left, and the guard
     `removal_outside_the_port_removes_exactly_the_named_events` names this defect.
   - Assert `head_ok(&store) == Some(b)`, expressed as `after_tail.last().map(|e| e.position)`.
     Message: **"head names an event the store no longer holds"**, and it continues:
     "`head` is the highest position visible in the store; a head read from the allocator
     (`sqlite_sequence`, a sequence's `last_value`) still names the removed tail at {c}".
6. `d = append_ok([AfterTheTail])`. Assert `d > c`. Message: **"a removed position was handed
   out again"**, and it continues: "the append after removing the tail at {c} took {d}. A rowid
   table without `AUTOINCREMENT` allocates one above the largest rowid it still holds, so it
   hands the deleted tail's position out again (ES-38's named wrong implementation). A store
   rebuilt from its survivors does the same (ES-38's Rejects)".
7. Assert `store.contains_event_id(removed_tail.id).await` is `Ok(false)`. Message: "the removed
   identity is held again: …". This is ES-41 store-relative, and an adapter whose `EventId` is
   `(store, position)` fails it exactly when it reuses.
8. `settle!`. Assert `positions_of(&read_ok(all)) == [a, b, d]`. This is "`Query::all()` means
   every event this store holds"; "unique and strictly monotonic" follows from equality with
   ascending returned positions.
9. **Stage 2: everything.** `remove_outside_port_or_fail(&fixture, &[a, b, d])`. `settle!`.
   - Assert `read_ok(all)` is empty ("the removal did not take", the same message prefix).
   - Assert `head_ok == None`. Message: "head names an event the store no longer holds", which
     is the same prefix (D5).
10. `e = append_ok([AfterEverything])`. Assert `e > d`. Message: **"a store emptied by removal
    handed out a position it had already assigned"**, and it continues: "… restarting from the
    top of an empty table, which is what SQLite does without `AUTOINCREMENT` and what
    `TRUNCATE … RESTART IDENTITY` does to a Postgres sequence. `REMOVAL`'s contract forbids
    rewinding the allocator: that makes a different store".
11. `settle!`. Assert `positions_of(&read_ok(all)) == [e]` and `head_ok == Some(e)`.
12. `RuleOutcome::Ran`.

Keep the body under clippy's 100-line `too_many_lines`. Lift stage 2 (steps 9 to 11) into a
private `async fn assert_emptied_store_does_not_restart<F: Fixture>(fixture: &F, store:
&F::Store, held: [SequencePosition; 3]) -> Result<(), RuleOutcome>`. The `Result` is needed
because `settle!` early-returns a skip. If that does not read cleanly, inline it and split the
assertions into `assert_*` helpers, as the guard does. Do not suppress the lint (H-05).

### Rule 2: `condition_over_removed_history_does_not_reject<F: Fixture>(open) -> RuleOutcome`

Rustdoc must cover: ES-40 and ADR-0028 decision 2; why the rule asserts **admission** (D4); the
two readers (an adapter author reads the vacuous pass as specified, an ingest author reads it as
a hazard, E2E-47); the mutants below; and why there are survivors on both sides of the hole (the
instrument's two evaluation paths, at-or-below `W` and above it, `withholding.rs:19-36`, and a
SQL store's index range on either side of a hole).

Helper (private): `fn over(claim: &str) -> AppendCondition { condition(query_of(&["ClaimRegistered"], &[("claim", claim)])) }`.
Every helper it uses already exists in `fixtures.rs`. The event type `ClaimRegistered` and the
tag key `claim` collide with nothing `PrunedMemoryFixture` seeds.

1. `require!(F: REMOVAL)`, then open and connect once.
2. `low`, `gone`, `high` = `append_ok` of three single-event batches, each of type
   `ClaimRegistered` and tagged `claim` = `kept-below`, `removed` and `kept-above`. `gone` is
   **interior**.
3. **Non-vacuity anchor**: `store.append(&[note()], Some(&over("removed")))` must be `is_err()`.
   `note()` is `tagged_event("EngineerNoteFiled", &[("claim", "removed")])`, a type outside the
   condition's query, so admitting it never creates a match. Message: "the probe is live: while
   the store holds the matching event the condition must refuse, or the vacuous pass below proves
   nothing". Use `is_err` rather than the `ConditionViolated` discriminant, for
   `condition_against_an_empty_store_admits_the_append`'s stated reason (`suite.rs:5757-5759`):
   the discriminant is ES-25's business. This deviates from TST-05, on house precedent.
4. `settle!`. `before = read_ok(all)`. Anchor: `positions_of == [low, gone, high]`.
5. `remove_outside_port_or_fail(&fixture, &[gone])`.
6. `settle!`. `after = read_ok(all)`. Assert that the **events by content**
   (`after.iter().map(|e| &e.event)`) equal those of `before[0]` and `before[2]`. Message:
   **"the removal did not leave exactly the survivors"**, which defers to the guard. Content and
   not position: positions are ES-38's business here, and this keeps `RenumberingFixture`
   passing this rule.
7. **The vacuous pass.** `admitted = store.append(&[note()], Some(&over("removed"))).await`.
   Assert `matches!(admitted, Ok(_))`. Message: **"passes vacuously"**, and it continues:
   "ES-40, ADR-0028: a condition over removed history passes vacuously. This store refused it
   ({admitted:?}). It is still judging the condition against an event that `read` no longer
   yields and `contains_event_id` no longer holds: a soft delete the condition path never
   learned about, or E2E-47's 'this log cannot evaluate' outcome, which ADR-0028 rejects for
   1.x". Then, `if let Ok(note_at) = admitted`: `settle!`, read `query_tagged(&[("claim",
   "removed")])`, and assert that its positions equal `[note_at]` and its types equal
   `["EngineerNoteFiled"]` (TST-05: whole value). This rejects an `Ok` that wrote nothing,
   which is another rule's defect, cheaply.
8. **Survivors still guard.** For `claim` in `["kept-below", "kept-above"]`:
   `store.append(&[tagged_event("EngineerNoteFiled", &[("claim", claim)])], Some(&over(claim)))`
   must be `is_err()`. Message: **"a removal is not a licence"**, and it continues: "the store
   still holds the `{claim}` registration, so a condition over it MUST still refuse. A store
   that stops evaluating conditions once it has lost history has read ES-40's vacuous pass as
   'conditions no longer apply'".
9. `RuleOutcome::Ran`.

### Why the rule asserts admission and not "MAY" (D4)

ES-40's MAY is the caller's reading: a caller must not rely on a refusal. On the store's side, a
refusal over a removed event names an event that the store's own `read` does not yield and its
own `contains_event_id` denies. That contradicts ES-38's "every clause … with respect to the
events it still holds", and it is half a removal: the same argument as the guard's membership
check. ADR-0028 rejected the fail-closed outcome for 1.x in so many words. ES-40's `Rule:` line
already says "an asserted-and-documented outcome", and its marker says the instrument supplies
the assertion "by … observing the vacuous pass". The rule writes that down.

### The wrong stores: `tests/mutation_coverage/removed_history.rs`

One generic fixture, on `removal.rs`'s `WrongRemoval` and `mutants.rs`'s `Defect` pattern, so
that each mutant differs from a correct store in exactly one step:

```rust
/// What the store holds, plus what a removal took (kept so a defect can misuse it).
#[derive(Debug)]
pub(crate) struct Tombstoned { pub(crate) log: Log, pub(crate) removed: Vec<SequencedEvent> }

pub(crate) trait AfterRemoval: 'static {
    const NAME: &'static str;
    /// The correct removal: tombstone the named events, then `Log::remove` (keeps the high-water mark).
    fn remove(state: &mut Tombstoned, positions: &[SequencePosition]) -> Removed { /* correct */ }
    /// The correct append: `state.log.append(events, condition)`.
    fn append(state: &mut Tombstoned, events: &[Event], condition: Option<&AppendCondition>)
        -> Result<SequencePosition, AppendError<LogError>> { /* correct */ }
    /// The correct head: the log's last held position.
    fn head(state: &Tombstoned) -> Option<SequencePosition> { /* correct */ }
}

pub(crate) struct AfterRemovalFixture<D> { state: Rc<RefCell<Tombstoned>>, defect: PhantomData<D> }
pub(crate) struct AfterRemovalStore<D> { state: Rc<RefCell<Tombstoned>>, defect: PhantomData<D> }
```

- `AfterRemovalStore<D>: EventStore` (bare flavour, `Error = LogError`). `read` uses
  `Snapshot::new(try_borrow … log.select(..))`, as `LogStore` does. `append` calls `D::append`
  under `try_borrow_mut`, with no `.await` while the borrow is held (H-14). `head` calls
  `D::head`. `contains_event_id` calls `log.contains(id)`. Mirror `LogStore`'s comments on why
  `read` is not async (ADR-0008).
- The fixture uses `SECOND_HANDLE = SUPPORTED`. `REOPEN` is declined with `removal.rs`'s
  `NO_DURABLE_MEDIUM` words (re-declare it locally, or make it `pub(crate)` in `removal.rs`).
  `REMOVAL = SUPPORTED`. `remove_outside_port` calls `D::remove(&mut state.borrow_mut(), positions)`.
  `Subject::NAME = D::NAME`. `open()` returns a fresh `Tombstoned { log: Log::new(dense), removed: vec![] }`.
- **The correct default removal** dedupes `positions`, pushes the events at them (from
  `log.select(&Query::all(), ReadOptions::new())`) onto `removed`, calls `log.remove`, and asserts
  that the count removed equals the distinct count. That is test code, so a broken environment
  may panic, per `remove_outside_port`'s contract.

`correct.rs` additions (all `pub(crate)`, all inside `impl Log`, all **correct** sub-steps):

- `remove` splits into two steps. `fn remember_high_water(&mut self)` holds the existing first
  two lines. `fn discard(&mut self, positions: &[SequencePosition]) -> usize` holds the retain
  and the count. `remove` = `remember_high_water(); discard(positions)`, and its behaviour is
  unchanged. The doc on `discard` says it is the step a store without a high-water mark stops
  at.
- `fn head(&self) -> Option<SequencePosition>` (`head_of(&self.events)`),
  `fn contains(&self, id: EventId) -> bool` (`contains(&self.events, id)`), and
  `fn high_water(&self) -> Option<SequencePosition>`.

The mutants. Each one overrides **one** method:

| Registry name | `D` overrides | Defect, and its real-world shape (CF-4 provenance) | Fails exactly | Pin |
|---|---|---|---|---|
| `RowidReuseFixture` | `remove`: tombstone, then `log.discard(..)` without `remember_high_water` | **ES-38's named wrong implementation.** `INTEGER PRIMARY KEY` without `AUTOINCREMENT`: SQLite keeps `sqlite_sequence` only for `AUTOINCREMENT` tables, so nothing remembers the largest rowid ever used, and the next insert takes max(held)+1, which is the deleted tail's position. The fidelity test (D6) shows it on real SQLite. | ES-38 | "a removed position was handed out again" |
| `SequenceHeadFixture` | `head`: `log.high_water().max(log.head())` | An O(1) head read from the allocator (`SELECT seq FROM sqlite_sequence`, a sequence's `last_value`) instead of `max(position)`. It agrees with every rule until a removal, then names an event the store no longer holds. | ES-38 | "head names an event the store no longer holds" |
| `TruncatingWhenEmptyFixture` | `remove`: correct, **except** when the named positions are everything held, which replaces the log with `Log::new(dense)` | A retention job that purges everything with `TRUNCATE … RESTART IDENTITY` because it is faster, or that resets the sequence on an empty table. Positions restart under the same identity, which `REMOVAL`'s doc already calls a different store. Nothing checked that sentence until now. | ES-38 | "a store emptied by removal" |
| `RememberingConditionFixture` | `append`: if `events` is non-empty and `violation(&state.removed, condition)` is `Some(p)`, return `ConditionViolated::at(p)`; otherwise the correct append | The 0.7.0 plan's mutant. A soft delete (`deleted_at`) whose read, head and membership queries learned to filter and whose condition `EXISTS` did not. It is also `WithholdingStore` without its condition rewrite (`withholding.rs:21-23`). | ES-40 | "passes vacuously" |
| `FailClosedFixture` | `append`: once `removed` is non-empty, any non-empty conditional append returns `ConditionViolated::at(highest removed)` | The sympathetic one (the `.bklg` spec's `GapAwareRejectingStore`): an adapter that knows it has been pruned and refuses every condition rather than risk an unsound admit. That is E2E-47's third outcome, which ADR-0028 rejects for 1.x, spelled in the only "no" the port has. | ES-40 | "passes vacuously" |
| `ConditionBlindFixture` | `append`: once `removed` is non-empty, `log.append(events, None)` | The opposite misreading: "a pruned store's conditions are unsound, so stop evaluating them". It admits a write the store still holds the evidence to refuse. | ES-40 | "a removal is not a licence" |

Emptiness first (ES-20): every `append` override checks `events.is_empty()` before the
condition, by delegating the empty batch to the correct `log.append`. Otherwise the override
picks up a second defect (`ConditionBeforeEmptinessStore`'s).

**The existing removal mutants' rows grow** (`tests/mutation_coverage.rs:2182-2245`). These are
the predicted results. **Measure them, and do not edit a row to match without understanding
the miss:**

| Mutant | guard | ES-38 | ES-40 | New pins |
|---|---|---|---|---|
| `ForgottenRemovalFixture` | fails | **fails** | **fails** | "MUST override `remove_outside_port`" on both new rules (the shared helper) |
| `NoopRemovalFixture` | fails | **fails** | **fails** | "the removal did not take" (ES-38); "the removal did not leave exactly the survivors" (ES-40) |
| `OverRemovingFixture` | fails | passes (stage 1 is a tail; stage 2 is everything, and `>= min` is then exact) | **fails** (it takes `kept-above`) | "the removal did not leave exactly the survivors" |
| `RenumberingFixture` | fails | **fails** (stage 1: survivors keep 1 and 2 densely, so `d` = `c`) | passes (content anchor; conditions are right) | "a removed position was handed out again" |
| the six new | passes | as in the table above | as in the table above | as above |
| `GappedPositionFixture` (honest control, `ConformantVariant`) | passes | passes | passes | — |

The provenance strings of the four existing rows gain one clause each, saying why they now fail
the new rules too (for example Noop: "and ES-38's and ES-40's rules refuse to judge a store
nothing left"). Changing a mutant's declared `fails` set to **add** rules strengthens the
expectation. It does not weaken one. Still, it is listed under D3 so the owner sees it (H-12/H-16).

Registry ripple in `tests/mutation_coverage.rs`: six `Declared` rows (`Kind::Mutant`,
`FailureMode::Assertion`, one `expect` pin per failing rule), inserted **after** the
`RenumberingFixture` row. Six entries in `for_each_mutant!` (after `removal::…Renumbering`,
`:2981`). Six `ModelOutcome::Agreed` entries in `MODEL_COVERAGE` (after `RenumberingFixture`,
`:3389`; the model never removes, so each is its correct store as far as the model sees). The
table's doc list at `:3251` and any count it states. `MUST_SKIP` (`:5128-5136`) gains both rule
names, because `DecliningFixture` declines `REMOVAL`. **Citations that move:**
`tests/mutation_coverage.rs:5538` is cited from `src/projection.rs:2424`, `src/fixtures.rs:532`
and `tests/projection_conformance.rs:37`. Repoint all three to wherever
`fn assert_reference_projection_declensions` lands. Then
`grep -rn 'mutation_coverage.rs:' crates standards xtask spec docs .kb` and check that every
citation above line 2245 is unchanged.

### The instrument leg (D1): `PrunedMemoryFixture` grants `REMOVAL`

- `const REMOVAL: Capability = Capability::SUPPORTED;`.
  `async fn remove_outside_port(&self, positions) -> Removed { self.store.withhold(positions).await.expect("…names only positions the inner store holds…"); Removed::Deleted }`.
  This is test code, and `remove_outside_port`'s contract permits a panic on a broken
  environment. Replace the declining comment (`fixture.rs` `REMOVAL` block) with the reason:
  - at every seam of the port a withheld event behaves as removed (that is CF-27's claim, and
    `accuracy.rs` proves the report);
  - the guard and ES-40's rule therefore judge exactly what they judge on an adapter;
  - ES-38's passes by delegation, because the inner store still holds every position.
- `pass_list.txt`: the guard line flips `skipped … REMOVAL` to `ran`, and two `event-store ran`
  lines are added in registry order. The header gains one sentence: "The fixture grants
  `REMOVAL` by forwarding to `withhold`, so the removal rules run here; ES-38's passes by
  delegation." The existing header sentence ("Holes inside a rule's own history … ES-38's and
  ES-40's rules (#151, #154) supply it") already anticipates this.
- `tests/completeness_instrument.rs`'s module doc gains the same sentence.
- `contract.rs` `REMOVAL` doc, **line-neutral**:
  - "Why it is not the decorator" keeps its argument, that hiding cannot show an allocator
    surviving a deletion. It adds: a fixture over the decorator may still grant the capability
    by forwarding to `withhold`, and CF-27's own does; there the guard and ES-40's rule judge
    what they judge everywhere, and ES-38's rule passes by delegation, which says nothing about
    any allocator.
  - The CF-39 sentence becomes: "a fixture whose removal does not reach every seam of the port
    — `read`, `head`, `contains_event_id` and the condition `append` evaluates — MUST decline
    this capability, rather than declare it and contribute a green result about a store nothing
    was removed from."

  Both edits are unreleased (`#147` is on `phase-14` only), so this is a two-way doc change.

### `AppendCondition` rustdoc (#154's documentation criterion)

Append one paragraph to the "A claim about one store's log, not about the world" section
(`append.rs:29-47`):

> This is specified behaviour, and it is checked: the conformance suite's
> `condition_over_removed_history_does_not_reject` (ES-40) removes the only event a condition
> matches, outside the port, and requires the append to be admitted, against CF-27's
> completeness instrument and against every adapter fixture that can remove events. A store that
> refuses there has kept part of what it removed: a refusal must name an event the store still
> holds.

Write it in code format, not as a link (see Placement). Add no doctest: `MemoryEventStore`
cannot remove, and a doctest staging the pass would need the testkit. The paragraph adds lines
to `append.rs`. Run `grep -rn 'append.rs:' spec standards .kb references crates` and repoint
any citation below line 47 (`append.rs:239-253` is cited by ES-40's prose and by ADR-0028,
whose accepted body may repoint a citation under `kb-governance-referent-not-reasoning-001`).
**Prefer to keep it line-neutral if a sentence in the section can be tightened.** Otherwise
repoint.

### SQLite fidelity test (D6), `crates/happenstance-sqlite/tests/migration.rs`

`#[test] fn a_rowid_table_without_autoincrement_hands_out_the_deleted_tails_position()`. Use an
in-memory `rusqlite::Connection` with two tables: `bare (id INTEGER PRIMARY KEY, v TEXT)` and
`counted (id INTEGER PRIMARY KEY AUTOINCREMENT, v TEXT)`. Insert three rows into each, delete
each table's highest row, then insert one more. Assert that `bare`'s new id equals the deleted
id, and that `counted`'s is above it. Then delete everything and insert again: `bare` restarts
at its first id, and `counted` does not. Compare only ids that `last_insert_rowid()` returned.
Doc: this is the evidence that `RowidReuseFixture` in the testkit models real SQLite, and that
`assert_event_table`'s `AUTOINCREMENT` check is what ES-38 stands on for this adapter. Test code
may `expect`.

### Bookkeeping

- `registry.rs`: two names (see Placement).
- `tests/mutation_coverage/lagging/inventory.rs`: add both names to `READS_BACK_ITS_OWN_WRITES`
  after the guard, and change the doc from "The 85 named rules" to 87. `LaggingFixture` and
  `NeverAsked` already forward `REMOVAL`.
- `crates/happenstance-testkit/README.md:4`: "Ninety-eight rules" becomes "One hundred rules".
- `xtask/src/proof.rs` `CLOUDFLARE_WASM_RULES` (`:829-841`): add both rules, and extend the
  guard paragraph (`:824-828`): "and ES-38's and ES-40's rules, which stand on the same grant".
- `xtask/src/spec_trace.rs` `UNRESOLVABLE_RULE_NAMES` (`:1032`, `[_; 67]`): delete the ES-38 and
  ES-40 entries (`:1285-1299`) and make the length 65. Keep ES-39's. Check 4 reports both as
  stale ("now resolves") until they go.
- `CHANGELOG.md` `[Unreleased]`, `happenstance-testkit` / Added: one entry per rule naming the
  defect it detects (CF-29 lint: ≥120 characters of prose per rule). Also: `PrunedMemoryFixture`
  now grants `REMOVAL`, which is test-only and needs no entry. Under `happenstance-core`:
  "`AppendCondition`'s documentation names the rule that checks the vacuous pass" (doc only).
- `docs/pass-conformance-suite.md` `REMOVAL` paragraph: one sentence saying that granting it now
  also runs ES-38's and ES-40's rules, and that a fixture whose removal rewinds the allocator
  (`TRUNCATE … RESTART IDENTITY`) fails ES-38's.
- **Spec (line-neutral, then `cargo xtask spec-trace --write`):**
  - §7.1 rows `:10599` (ES-38) and `:10601` (ES-40) lose their `†`. This is generated.
  - ES-40's `Rule:` bullet (`:4940-4943`, 4 lines, `[PROVISIONAL]`, editable) becomes, in 4
    lines: "`condition_over_removed_history_does_not_reject` (#154), against CF-27's instrument
    and every fixture granting `REMOVAL` — an asserted-and-documented outcome rather than a
    desired one, so that an adapter author reads the vacuous pass as specified behaviour and an
    ingest author reads it as a hazard." The marker is untouched.
  - CF-27 (`:9890-9891`, 2 lines, `[DEFERRED]`): "…are the two rules that already / have
    assertions and are waiting only on this instrument to be written against." becomes "…are
    the two rules / written against it and against every fixture granting `REMOVAL` (#151,
    #154)." Each line keeps its length within the file's habit.
  - §3.8 (`:5114-5118`, 5 lines): the heading line becomes "**~~Blocked on the completeness
    instrument (CF-27)~~, written at phase 14:**". The body says that both rules run against
    `WithholdingStore` and every fixture granting `REMOVAL` (#151, #154). Still 5 lines.
  - **ES-38 (`:4859-4883`) is not edited.** See D2.
- **`.kb/_intake/es-38-rule-line-after-151.md`** (D2): a raw intake note, not an atom and not an
  ADR. It carries the exact 11-line replacement proposed for ES-38's `Rule:` bullet
  (`SPECIFICATION.md:4869-4879`), for #163's freeze record to adopt or reject:

  ```
  - **Rule:** `positions_are_not_reused_after_removal` (#151), run against CF-27's
    instrument (`PrunedMemoryFixture` over `WithholdingStore`, where it passes by
    delegation) and against every fixture that grants `Fixture::REMOVAL` (#147):
    SQLite, Postgres, Neon, the Durable Object over the shim and under workerd. It
    removes the tail and then everything, and requires each next append above every
    position ever assigned, `head` to name a held event, and the removed identity to
    stay unheld. It rejects `RowidReuseFixture`, the named wrong implementation (a
    rowid table without `AUTOINCREMENT` reusing the deleted tail's highest rowid),
    `SequenceHeadFixture` and `TruncatingWhenEmptyFixture`. It stands on the guard
    `removal_outside_the_port_removes_exactly_the_named_events`. Amended by the
    phase 14 freeze record (#163): the phase 4 note that no capability existed is gone.
  ```

  Also include: why it is stale (the "`Fixture` declares no such capability" and "That
  instrument does not exist" sentences); that the edit is line-neutral (11 for 11); and that
  `kb-governance-referent-not-reasoning-001` does **not** cover it (the change is reasoning, not
  a referent), so it needs the ADR. Comment on #163 with a pointer to the file.

### Concurrency and resources

There are no tasks, channels or locks in library code. The new test store borrows a `RefCell`
inside one poll, with no `.await` in between (H-14), exactly like `LogStore`. Both rules hold one
handle and run sequentially. On Postgres and Neon, each read is preceded by `settle!` (the
fixtures decline `READ_YOUR_OWN_WRITES`). `settle!` is idempotent, and is called before every
read or `head` that must reflect an append made since the last one.

### Alternatives considered

- **Two PRs** (#151, then #154 stacked). Rejected (D3): both touch the same rows of
  `mutation_coverage.rs` (the four existing removal mutants' `fails` and `expect` change for
  each rule), the same registry group, `MUST_SKIP`, the lagging inventory count, the README
  count, `CLOUDFLARE_WASM_RULES`, adjacent `spec_trace.rs` entries, `pass_list.txt` and
  `CHANGELOG.md`. They also share the helper and the instrument grant. The second PR would rebase
  every one of those.
- **A second instrument fixture granting `REMOVAL`**, leaving `PrunedMemoryFixture` declining
  (D1-B). It costs a second fixture and a second mount, and the recorded pass list would keep
  saying "skipped" for the three rules CF-27's evidence most wants to see run.
- **A hand-written instrument test** that copies each rule's body against `WithholdingStore`
  (D1-C). The rule itself would never run on the instrument (the exit criterion), and there
  would be two bodies to keep in step.
- **Rules in a child module file** (`src/suite/rules/…`). This would need `RULE_FILES` and
  spec-trace changes, and it splits the removal group from its guard.
- **Asserting `ConditionViolated` on the survivor checks.** The house precedent leaves the
  discriminant to ES-25's rule.
- **An interior removal in ES-38's rule.** It adds nothing that the guard and `RenumberingFixture`
  do not already cover, and it would make `OverRemovingFixture` fail ES-38 for the guard's
  reason, which blurs the attribution.

## Test list (drive in this order; one PR, two commits)

**Commit 1: #151 (ES-38)**

- [x] **T1.** `correct.rs` unit test `a_discarded_tail_is_handed_out_again`: after
  `log.discard(&[tail])`, `log.append` returns the tail's position. Red: `discard` does not
  exist. Then `remove` = `remember_high_water` + `discard`. `a_removed_tail_is_not_handed_out_again`
  stays green unchanged.
- [x] **T2.** Add `"positions_are_not_reused_after_removal"` to `MUST_SKIP`. Red: compile error
  or `capability_skips_are_reported` fails, because the rule does not exist. Green: a rule with
  only `require!` and `RuleOutcome::Ran`, plus the registry entry and the `READS_BACK` entry.
  Expect `every_rule_has_a_mutant` to go red now. That is the red that drives T3.
- [x] **T3.** `RowidReuseFixture` with its registry, `for_each_mutant!` and `MODEL_COVERAGE`
  rows. Red: `mutants_fail_exactly_their_declared_rules` (it passes the skeleton rule). Green:
  steps 1 to 8 of rule 1, minus the head assertion.
- [x] **T4.** `SequenceHeadFixture` row. Red: it passes. Green: the head assertion in step 5.
- [x] **T5.** `TruncatingWhenEmptyFixture` row. Red: it passes. Green: steps 9 to 11.
- [x] **T6.** Update the existing rows for Forgotten, Noop and Renumbering with ES-38 added to
  `fails`, with pins. The shared helper replaces the guard's inline answer check. Red first: the
  meta-test reports the undeclared failures. Confirm that `OverRemovingFixture` passes ES-38, as
  predicted.
- [x] **T7.** `GappedPositionFixture` passes the whole suite (`conformant_variants_pass_everything`,
  `capable_fixtures_skip_only_what_they_decline`) with no edit to their expectations.
- [x] **T8.** The lagging meta-tests are green with the inventory at 86.
- [x] **T9.** Instrument (D1): `PrunedMemoryFixture` grants `REMOVAL`, and `pass_list.txt` is
  regenerated from the failure message. `instrument_report_is_accurate_and_the_suite_cannot_tell`
  is green with the guard and ES-38 recorded as `ran`. The mounted
  `event_store_conformance!(PrunedMemoryFixture…)` runs both.
- [x] **T10.** SQLite: `cargo test -p happenstance-sqlite --test conformance` runs ES-38 green,
  and the fidelity test (D6) in `--test migration` is green.
- [x] **T11.** `spec_trace.rs`: delete the ES-38 entry. `cargo test -p xtask` is green
  (`every_declaration_holds_against_the_real_tree`, `the_table_declares_each_pair_once`).
- [x] **T12.** `proof.rs` list, README count, CHANGELOG entry, docs sentence, intake note.

**Commit 2: #154 (ES-40)**

- [x] **T13.** `MUST_SKIP` gains `"condition_over_removed_history_does_not_reject"`. The
  skeleton rule, registry and `READS_BACK` (87) entries are added, as in T2.
- [x] **T14.** `RememberingConditionFixture`. Red: it passes the skeleton. Green: steps 1 to 7
  (the anchor, the content check and the vacuous pass with its read-back).
- [x] **T15.** `FailClosedFixture`. It should already be red against T14's body. Confirm the
  pin "passes vacuously" and record it in the Progress section; no new code.
- [x] **T16.** `ConditionBlindFixture`. Red: it passes. Green: step 8 (survivors still guard).
- [x] **T17.** Existing rows for Forgotten, Noop and OverRemoving have ES-40 added, with pins.
  Confirm that `RenumberingFixture` passes ES-40, as predicted.
- [x] **T18.** `GappedPositionFixture` and the instrument (pass list: one more `ran` line) are
  green. SQLite `--test conformance` is green.
- [x] **T19.** `AppendCondition` paragraph. `cargo test -p happenstance-core --doc` and the core
  doc builds (`--no-default-features` and default) are green with `-D warnings`.
- [x] **T20.** `spec_trace.rs`: delete the ES-40 entry and set the length to 65. Make the spec
  edits (ES-40 bullet, CF-27, §3.8) line-neutral, then run `cargo xtask spec-trace --write`,
  then `cargo xtask spec-trace` (green). Also run `cargo xtask lint-constitution`,
  `cargo xtask lints` (CF-29 changelog) and `cargo xtask lint-kb`.
- [x] **T21.** `proof.rs` list, CHANGELOG entry and README count (one hundred).

**The gate.** Disk is tight, so drive the work with
`CARGO_INCREMENTAL=0 cargo test -p happenstance-testkit --test mutation_coverage` and
`--test completeness_instrument`, plus `-p happenstance-sqlite --test conformance --test
migration` and `-p xtask`. Then run `cargo xtask affected --base origin/phase-14`. Before handing
back, run `cargo xtask ci --fast` at minimum. The full `cargo xtask ci` is preferred if disk
allows. **Not verifiable locally:** Postgres and Neon (the `live-postgres` and `live-neon`
jobs), the Cloudflare shim (the `wasm` job; `cargo xtask wasm` checks that it compiles) and
workerd (the `workerd` job; it dispatches the two new rules by name through the registry, with no
harness edit). Say so in the PR.

## Decisions (H-16)

**One-way doors:** none in this PR. Everything below is unreleased (`phase-14` only) and
reversible by a follow-up commit. Two things become one-way **later, and they are the owner's**:
accepting the ES-38 amendment (an ADR, in #163), and freezing ES-40 "with its rule" (#163),
which makes D4's reading binding for 1.x. Publishing `0.7.0` also makes both rule names
published API, but the spec already fixes those names.

| # | Decision | Door | Options | Recommendation | Decided by | Outcome |
|---|---|---|---|---|---|---|
| D1 | How the rules run "against the instrument" | two-way | **A.** `PrunedMemoryFixture` grants `REMOVAL` by forwarding to `withhold`; the pass list records the guard, ES-38 and ES-40 as `ran`; `REMOVAL`'s doc is amended line-neutrally ("every seam of the port", not "own storage"). **B.** A second test-only fixture grants it, and the pruned one keeps declining (more code; the pass list keeps three skips). **C.** A hand-written instrument test (the rule never runs on the instrument). | **A.** CF-27's text already says ES-38's rule runs "against the real adapters as well as the decorator". The pass list header already says ES-38's and ES-40's rules "supply" the holes. #147's plan left the grant to #151 ("or leave it to #151"). | architect; owner may flip | recommended |
| D2 | ES-38's stale `[FROZEN]` `Rule:` text | the intake is two-way; the ADR that adopts it is one-way and the owner's | **A.** This PR stages the exact 11-line, line-neutral replacement in `.kb/_intake/es-38-rule-line-after-151.md` (raw, not an atom, not an ADR) and points #163 at it. **B.** Leave it entirely to #163. **C.** Edit ES-38 here (forbidden: a `[FROZEN]` body changes only through an ADR). | **A.** The author of the rule has the exact facts. `afk.md` forbids writing an ADR, not staging intake. Intake is unchecked by `lint-kb` and deletable. The owner accepts or rewrites it in #163's freeze record in the morning. | owner (in #163) | recommended |
| D3 | One PR or two; and widening four existing mutants' declared `fails` sets | two-way | **A.** One PR, two commits (`Closes #151`, `Closes #154`). **B.** Two stacked PRs. | **A.** Near-total file overlap, shared rows, a shared helper and a shared instrument grant (see Alternatives). Widening `fails` adds rules a mutant is now shown to fail. That strengthens the expectation and is not loosening (H-12), but it is listed so the owner sees it. | architect | recommended |
| D4 | ES-40's rule asserts **admission** (rejecting fail-closed and soft-delete stores), not just "MAY" | two-way now; one-way at #163's freeze | **A.** Assert admission, plus survivors still refusing. **B.** Assert only that survivors still refuse, and allow either outcome on removed history (the rule would then not test ES-40's subject). | **A.** ADR-0028 decision 2 rejects the third outcome for 1.x. ES-40's `Rule:` says "asserted-and-documented outcome". A refusal over a removed event names an event `read` and `contains_event_id` deny (ES-38). | owner-visible; architect recommends | recommended |
| D5 | ES-38's rule asserts `head` (it names the last survivor, and is `None` after removing everything) | two-way | **A.** Assert it, which rejects `SequenceHeadFixture`. **B.** Assert nothing about head (the guard deferred head to "ES-38's"). | **A.** Every adapter's head is `max(position)` over visible held rows (SQLite `event_store.rs:2185`, Postgres `:653`, Neon `:993`, Cloudflare `max_position`, `WithholdingStore::head`), so all pass. Watch for: a runner whose checkpoint sits above a `None` head after a total purge. That is PS-22's "reset" case and already documented. | architect | recommended |
| D6 | A real-SQLite fidelity test for the rowid model | two-way | include (~45 lines in `happenstance-sqlite/tests/migration.rs`) / skip | **include.** It makes `RowidReuseFixture`'s provenance a measurement and not a claim. | architect | recommended |
| D7 | Reconcile non-frozen spec prose now (ES-40 `Rule:`, CF-27, §3.8) or leave it to #163 | two-way | now (line-neutral) / #163 | **now.** It is session protocol step 6, the markers are untouched, and it keeps #163's diff to marker moves and ES-38. | architect | recommended |
| D8 | Mutants beyond the two required (`SequenceHead`, `TruncatingWhenEmpty`, `FailClosed`, `ConditionBlind`) | two-way | include / required pair only | **include.** Each one is the only thing that fails one assertion, so no assertion is decorative (CLAUDE.md's corollary). `FailClosed` is ADR-0028's rejected outcome made executable. | architect | recommended |

## Risks

- **Predicted `fails` sets are wrong.** For example, `OverRemovingFixture` fails ES-38 after all,
  or a new mutant trips the guard. The meta-test says so with the rule and the message. Stop and
  find the cause. Do not adjust the row to match (H-12).
- **`settle!` placement on frontier stores.** A read after an append without a fresh `settle!`
  fails only on Postgres and Neon, which are live-only. The lagging meta-test (T8) catches a
  missing **first** settle, not a missing later one. Review every `read_ok` and `head_ok` in both
  rules against "is there an append since the last settle?".
- **SQLite `tag_cardinality` at zero** after removing the only `claim=removed` event. The
  condition planner seeds from the most selective tag, and a zero-count row must still yield "no
  match". T18 runs it on SQLite locally. If it fails, that is an adapter finding (an issue
  against `happenstance-sqlite`), not a rule to soften.
- **Postgres/Neon head after a total removal** relies on `max(position) … WHERE <frontier>`
  returning `NULL`. That is true by SQL. It is only provable in the live jobs.
- **`contract.rs` and `append.rs` doc edits that are not line-neutral** move citations. Grep and
  repoint as described above. `lint-constitution` and `spec-trace` catch some of them, not all.
- **`mutation_coverage.rs:5538`** citations from published `src/` files go stale unless they are
  repointed. Nothing gates this, so review it by grep.
- **Merge order with #156 and #158**, which may also touch `pass_list.txt`, `CHANGELOG.md` and
  the registry. Whoever lands second rebases. The pass-list failure prints the replacement.
- **The workerd job** runs the two rules on a real Durable Object for the first time. A failure
  there is evidence about the platform's `sqlite_sequence`. Record it rather than skipping it.

## Size (lower bound)

About 19 files and about 1,050 added lines (about 60 removed or changed):
- `src/suite.rs`: about 330 lines, for two rules with rustdoc, the helper and the stage helper.
- `src/registry.rs`: 2 lines. `src/contract.rs`: about 12 lines changed, net 0.
- `tests/mutation_coverage/removed_history.rs` (new): about 320.
- `tests/mutation_coverage.rs`: about 150, for six rows, four row updates, the macro, the model
  table and `MUST_SKIP`.
- `correct.rs`: about 50. `mutants.rs`: 3. `lagging/inventory.rs`: 3.
- Completeness instrument (`fixture.rs`, `pass_list.txt`, `completeness_instrument.rs`): about 30.
- `happenstance-sqlite/tests/migration.rs`: about 45.
- `happenstance-core/src/append.rs`: about 7.
- `xtask` (`proof.rs`, `spec_trace.rs`): about 10 added and 15 removed.
- Spec (net 0), CHANGELOG about 20, README 1, docs about 5, intake note about 40.

## Progress

- 2026-11-15: plan written (architect). Nothing implemented.
- 2026-11-15, commit 1 (#151). T1: `discard`/`remember_high_water` written with
  the unit test in one step (red was the missing method); green. T2/T3 red,
  skeleton rule (`require!` + `Ran`) against the three new rows:
  `mutants_fail_exactly_their_declared_rules` -> "`RowidReuseFixture` declares
  that it fails `positions_are_not_reused_after_removal`, but the rule passed";
  `an_unsettling_lag_skips_exactly_the_read_back_rules` and
  `a_settle_that_reveals_nothing_is_caught_at_the_anchors` red (the skeleton
  does not settle). Full rule body -> green for the three new rows with their
  pins. T6 measured: with the existing rows unchanged the meta-test reported
  `ForgottenRemovalFixture` failing ES-38 undeclared ("MUST override
  `remove_outside_port`"); after adding ES-38 to Forgotten, Noop and
  Renumbering exactly as predicted, the meta-test (exact in both directions,
  pins included) is green, so OverRemoving passes ES-38 as predicted and no
  new mutant trips the guard. T7/T8 green unchanged. T9: pass list red, then
  regenerated (guard `ran`, ES-38 `ran`). T10: SQLite runs ES-38 green; the
  fidelity test is green. Deviation: the fidelity test sits at the end of
  `migration.rs`, not after `assert_event_table`, so that
  `.kb/open-questions/remint-identity-precondition-is-trust-only.md`'s
  `migration.rs:501` does not move. T11: the ES-38 entry in
  `UNRESOLVABLE_RULE_NAMES` is replaced by a five-line comment (VT-6's
  precedent) rather than deleted, so `spec_trace.rs:N` citations below it hold.
  Registry insertions moved five `registry.rs:N` citations in
  `standards/rust/`; repointed (lint-constitution green).
- 2026-11-15, commit 2 (#154). T13/T14 red, skeleton rule against the three
  new rows: "`RememberingConditionFixture` declares that it fails
  `condition_over_removed_history_does_not_reject`, but the rule passed", plus
  the two lagging tests. T17 rows for Forgotten, Noop and OverRemoving updated
  to the predicted sets before the body was written. Steps 1-7 written: the
  meta-test's only complaint was "`ConditionBlindFixture` declares ... but the
  rule passed" (it is last in `for_each_mutant!`), so every row before it,
  including FailClosed with "passes vacuously" (T15) and Renumbering passing
  ES-40, matched exactly. Step 8 -> green. T18: instrument pass list gains the
  `ran` line, 113 green; SQLite runs ES-40 green, so no `tag_cardinality`
  finding. T19: `AppendCondition`'s paragraph is line-neutral (12 lines for 12,
  the section tightened), so no `append.rs:N` citation moves. T20: ES-40 entry
  replaced by a five-line comment, table length 65; spec edits line-neutral
  (`assert` on newline count); `spec-trace --write` then green;
  lint-constitution green after repointing the five `registry.rs:N`
  citations once more; `mutation_coverage.rs:5538` citations now point at
  `:5717`.
