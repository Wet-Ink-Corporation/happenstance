# Changelog

Notable changes to `happenstance`, `happenstance-core`, `happenstance-testkit`,
`happenstance-sqlite` and `happenstance-cloudflare` — every crate this
workspace currently publishes.

`cargo xtask lints` derives that set from `xtask/src/package.rs`'s
`PUBLISHABLE` and fails this file if the two disagree, so the scope above
stays exactly as wide as what actually ships.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and
the project follows [semantic versioning](https://semver.org/spec/v2.0.0.html)
from `0.1.0` onward.

Started at the beginning rather than reconstructed at release time. A changelog
assembled from twelve phases of `git log` records what the commits said, which is
not the same as what a user needed to be told.

**Two things this project versions unusually, stated once here:**

- **`happenstance-testkit` has its own version**, independent of the other
  crates. Adding a conformance rule is a semver-*minor* change that can turn a
  passing adapter's CI red, so treat a minor bump there as breaking and pin it
  exactly.
- **`ProjectionStore` ships behind an off-by-default `unstable-projection`
  feature** — declared on `happenstance-core`, and forwarded by `happenstance`
  for the typed runner built over it — and is exempt from semver until two
  adapters at opposite ends of the batch-shape axis have passed its conformance
  suite. See [`spec/SPECIFICATION.md`](spec/SPECIFICATION.md) §4.

## [Unreleased]

### Added

<<<<<<< HEAD
- **A conformance rule for the one pair of read options the suite never put on
  the same read — breaking in practice, so pin `happenstance-testkit` exactly
  before taking it.** `happenstance-testkit` gains
  **`read_to_composes_with_limit`**, the ninety-third event-store rule: a closed
  window and a row budget together, with the **budget the smaller** of the two.

  The defect it detects is a budget that never reaches the windowed statement,
  because a closed window is a different statement from a page:

  ```text
  if let Some(to) = options.to {
      self.read_window(options.from, to)   // <- limit never reaches here
  } else {
      self.read_paged(options.from, options.limit)
  }
  ```

  The reasoning behind it is an argument rather than a slip: *the caller gave me
  both ends of the window, so the window is the bound that matters and the row
  budget is redundant.* It is redundant exactly while the budget is the larger of
  the two — which is the case an author checks by hand — and it is the whole
  point when the budget is smaller, which is the case a backfill worker is in on
  every call but its last. VT-29 already states the converse of the same
  confusion: `limit` cannot stand in for `to`. Neither stands in for the other.

  `WindowedPagingBudgetStore` is that adapter, and it **passed all ninety-two
  rules that preceded this one** — measured, by registering it with an empty
  failure list and letting the meta-test drive every rule at it, not argued. The
  bound's own three rules issue no budget; the budget's own rules issue no bound;
  and with the budget smaller than the window even `ToBoundIgnoredStore` and
  `ToIsExclusiveStore` answer this read correctly, because the budget masks the
  bound. What a caller loses is the page it sized: it asked for five hundred
  events of its window and got the window, with `Ok` everywhere.

  **One argument this retires.** `read_from_composes_with_limit` landed with a
  note saying no such rule was owed, because `to` and `limit` both cut the back
  of a read and therefore commute. The commutation is real, and it has been
  measured: the wrong implementation an adversarial review proposed for the pair
  — the budget applied before the bound — answers every read exactly as the
  reference implementation does, in both directions, and is deliberately **not**
  registered. But commuting is a statement about the *order* two options are
  applied in, and this defect is about *applicability*: it drops one option
  because the other is present. The note is corrected in place rather than
  deleted.
- **A conformance rule that turns a model-only defect into an ordinary one —
  breaking in practice, so pin `happenstance-testkit` exactly before taking it.**
  `happenstance-testkit` gains **`read_to_composes_with_multi_item_query`**, the
  ninety-second event-store rule: `ReadOptions::to` against a **filtering**
  query.

  The defect it detects is the textbook operator-precedence bug on the upper
  bound:

  ```text
  WHERE type = ? OR tag = ? AND position <= ?
  ```

  `AND` binds tighter than `OR`, so the window's top is conjoined with the last
  disjunct alone and every event matching an earlier item comes back from above
  the window. It is what an adapter produces when the `to` clause is appended to
  a `WHERE` string that already carries a disjunction someone else built — which
  is how a `WHERE` clause gets built anywhere the driver cannot take a query
  tree. The caller it breaks is a bounded backfill worker owning `[1, H]` while a
  tail worker owns everything above it: the backfill reads past its own window
  and re-delivers events the tail worker has already processed, with `Ok`
  everywhere and no error to log.

  **What is new here is not the store but where it can be seen.**
  `UnparenthesisedToPredicateStore` was already in the registry, filed as caught
  by the *model* family alone — an honest record of a real hole, and a worse one
  than it looked: that family is behind the optional `proptest` dependency **and**
  behind `cfg(not(target_arch = "wasm32"))`. The two adapters in this workspace
  that will build their predicate by concatenation and run on that target,
  `happenstance-cloudflare` and `happenstance-neon`, ran nothing at all that
  could see it. All three existing `to` rules issue `Query::all()`, where there
  is nothing for the `OR` to bind wrongly across, and the lower bound is
  conjoined correctly, so `read_from_composes_with_multi_item_query` passes it
  too. It is an ordinary mutant now, failing exactly one rule.

  CF-12 closed this gap for `from` at phase 3; ES-16 is the clause that closes it
  for `to`, and its rule list names the new rule.
=======
- **`happenstance-cloudflare` publishes the two query widths it plans against:
  `CloudflareEventStore::MAX_QUERY_ARMS_PER_STATEMENT` (400),
  `MAX_QUERY_PARAMETERS_PER_STATEMENT` (30,000) and
  `planned_statement_count`.** They are the sibling's numbers, and deliberately
  so: both are properties of the SQLite underneath a Durable Object's storage
  rather than of either adapter, so two adapters over one engine disagreeing
  about them would be two guesses rather than one measurement.

  They are **not** a fourth row of the crate's *capacity limits* table and must
  not be read as one. Every row there is a value the store **refuses**, naming
  the `StoreLimit` it refuses with; a query wider than either of these is
  chunked and merged, never refused. The front page carries them under *The
  query widths this store does not refuse*, with
  `planned_statement_count` as the way to ask the question directly.

  Published because VT-23 does *not* ask for them. Unlike VT-21, VT-22 and
  VT-24 it imposes no documentation obligation, and that silence is how two
  adapters shipped under one contract at one version came to have materially
  different query capability with nothing on either page to compare. Additive,
  and free only until `0.2.0`.
- **`happenstance-sqlite` publishes its second query ceiling:
  `SqliteEventStore::MAX_QUERY_PARAMETERS_PER_STATEMENT`.** SQLite pushes back in
  two units and the crate declared one of them. `MAX_QUERY_ARMS_PER_STATEMENT`
  bounds the arms of a `UNION` against `SQLITE_MAX_COMPOUND_SELECT`; this bounds
  the bound parameters of one statement against `SQLITE_MAX_VARIABLE_NUMBER`,
  which the translation spends one per tag and one per type of every item in a
  chunk. The two axes move independently: 400 items of a single tag each is 400
  arms and 400 parameters, and the same 400 items at `MAX_TAGS_PER_EVENT` tags
  apiece is still 400 arms and **51,200** parameters against a limit of 32,766.

  It is public for the reason the arm width is public — a test that has to guess
  the boundary is a test that stops crossing it — and its value is `30_000`, the
  same `PARAMETER_BUDGET` the multi-row tag insert has chunked to since the write
  path was written. Additive, and free only until `0.2.0` turns it into a
  promise.
>>>>>>> lane/query-ceilings
- **`happenstance` no longer glob re-exports `happenstance-core`, and the
  contract's projection surface no longer arrives without the feature that
  gates it.** The crate root's `pub use happenstance_core::*;` became an
  explicit, `#[cfg]`-carrying list of every contract item, name by name.

  The glob re-exported whatever the **compiled** contract crate exposed, and the
  contract gates its projection items on **its own** `unstable-projection`, not
  on this crate's. `happenstance-testkit` is this crate's dev-dependency and
  enables that feature unconditionally, so under `cargo test` —
  and in any consumer graph where a second crate asks for it —
  `happenstance::Checkpoint`, `::ProjectionId`, `::ProjectionStore`,
  `::SendProjectionStore`, `::Authority`, `::CommitError`, `::ResetError`,
  `::ProjectionProbe`, `::projection` and `::MemoryProjectionStore` all resolved
  with `unstable-projection` **off**. The manifest promises the opposite: *"A
  reader has to type the word `unstable` before any of them is in their build."*
  The user-visible shape is an auto-import — `happenstance::Checkpoint` offered
  by an editor, accepted into library code, compiling under `cargo test` and
  failing under `cargo build`.

  **Breaking, narrowly**, and free only at `0.2.0-alpha.1`: anyone who reached a
  leaked path loses it and gains the feature flag that was always meant to be
  the door. `ProjectionProbe` is gone from this crate outright — the contract
  gates it on `conformance`, a feature `happenstance` does not forward, so no
  consumer of this crate could turn it on *or* off. It is a test double and it
  lives in `happenstance-testkit`.

  The glob's second cost was quieter and is closed by the same change: every
  future addition to `happenstance-core` was an addition to `happenstance`'s
  public surface with nobody reviewing it, and a name added to both crates was a
  hard break in a crate that had not changed. An explicit list makes that
  collision `error[E0255]` in the commit that causes it.

  `crates/happenstance/tests/contract_surface.rs` is what holds it: it derives
  each item's gate from `happenstance-core`'s own crate root and compares it
  against the hand-written list here, so a contract item added behind a feature
  and mirrored here without one fails at home rather than on docs.rs.

- **A second conformance rule, breaking in practice for the same reason: pin
  `happenstance-testkit` exactly before taking this.** `happenstance-testkit`
  gains **`arming_a_read_fault_makes_the_stream_yield_an_error`**, and with it
  `Fixture::READ_FAULT` and `Fixture::arm_read_fault` — both **defaulted**, so no
  existing fixture has to change and a fixture that says nothing declines and
  reports a skip.

  It closes a hole with a shape worth stating: `EventStore::read` yields
  `Result<SequencedEvent, Self::Error>` **per item**, and until now nothing in
  the suite ever reached that `Err` arm. `Fixture` carried `MID_BATCH_FAULT` for
  the write path and no read-path analogue, so no rule could induce a read
  fault, no mutant modelled one, and the wrong implementation is one line:

  ```text
  let Ok(page) = fetch().await else { return Poll::Ready(None) };
  ```

  A failed fetch reported as the end of the log. Every consumer downstream reads
  `Ok`: a projection runner replays a short prefix, checkpoints at the truncation
  point, and never applies the rest — with no error anywhere to log. The store
  measured **0 of 89** rules failed with no way to arm it, and **22** with the
  fault armed by hand; the suite was not blind to the consequence, it had no way
  to produce one. It is now `SwallowedReadFaultStore` in the testkit's own
  mutation registry, and `PagedStreamStore` beside it is the same paging store
  meeting the same fault and yielding the `Err` the port provides for.

  A fixture whose store can absorb every read fault it is able to arm MUST
  decline the capability with that as its stated reason, which is CF-39's shape
  one path over.
- **One conformance rule, and it is breaking in practice: pin
  `happenstance-testkit` exactly before taking this.** `happenstance-testkit`
  gains **`read_from_composes_with_limit`**, the ninetieth event-store rule. An
  adapter that passes today can go red on it, which is what the note at the top
  of this file means when it says to treat a minor bump of this crate as a break;
  CF-29 makes that policy rather than advice. `0.2.0` is the cheap moment to take
  it, because it is the release that creates the adapter population — the same
  rule landing at `0.2.1` costs every adapter a red build.

  The defect it detects is a forward read that branches on the presence of a
  cursor and never threads the caller's budget into that branch:

  ```text
  if let Some(from) = options.from {
      self.read_resume(from)          // <- limit never reaches here
  } else {
      self.read_paged(options.limit)
  }
  ```

  That is the order a SQL adapter is written in — the paging query first, the
  resume cursor threaded through it afterwards — and it survived every rule the
  suite had. Three `[FROZEN]` clauses name forwards `from` composed with `limit`
  as the consumer that motivates them: ES-14's budget over the whole ordered
  result, VT-28's paging loop writing `.limit(budget - fetched)`, and CF-12's
  cursor over a multi-item query. The suite composed `from` with `to`, with
  `backwards`, and with `backwards` **and** `limit` — and issued forwards `from`
  with `limit` at no call site at all. A store with exactly that defect and its
  backwards branch left correct passed all eighty-nine rules and was
  indistinguishable from a conformant one.

  What a caller loses without the rule is a page it sized. A projection runner
  that asks for five hundred events from its checkpoint is handed the whole
  stream instead, the read model behind it buffers a batch nobody budgeted for,
  and the loop's own arithmetic is arithmetic about a number the store ignored.
  Nothing errors, so nothing retries, and the ceiling that the budget existed to
  hold stops holding. The rule reads through a **two-item** query at a window
  that straddles both items' matches, so it also rejects the two adapter shapes
  that get the merged result wrong under a budget — the unparenthesised
  `WHERE a OR b AND position >= ?`, whose cursor binds to the last item alone,
  and the store that emits one statement per query item and writes `LIMIT n` onto
  each of them.
- **Every published crate now re-exports the crates its own public signatures
  name.** `happenstance-core` re-exports `futures_core` (it already re-exported
  `bytes`); `happenstance-sqlite` re-exports `rusqlite` and `happenstance_core`;
  `happenstance-cloudflare` re-exports `worker` and `happenstance_core`; and
  `happenstance` re-exports `happenstance_core`. Before this, `pub use
  happenstance_core;` appeared in **no** crate, so a consumer implementing
  against `SqliteEventStore` had no way to name `Query` or `AppendCondition`
  except by adding a `happenstance-core` line the resolver was free to fork on.

  What a re-export buys is *type identity and discoverability* — the
  `rusqlite::Error` you match on is the one the adapter's error enum actually
  carries, rather than a second copy that prints the same and meets you as
  `error[E0308]` on the error path. It is not a substitute for your own
  dependency line and it forwards no features you did not enable. Each path is
  proved to resolve from outside its crate by a doctest.

- **WF-11's falsifier has been fired at, and the answer is on file.** The clause
  has carried a `[PROVISIONAL]` marker since phase 5 on a falsifier needing two
  things in one place — a memory ceiling that is real, and a payload large enough
  to hit it — and until the conformance suite executed on `wasm32` this workspace
  had neither. `crates/happenstance-cloudflare/tests/wf11_memory_ceiling.rs` is
  the first memory measurement this repository has ever taken: it walks
  `core::arch::wasm32::memory_grow` up the isolate under the gate's own `wasm32`
  step, then drives payloads sized against what it finds through
  `happenstance-core`'s real human-readable payload encode, with the binary
  encode of the *same* bytes beside it as the control that stops the number being
  vacuous.

  What it found, on the runner the gate actually has: no ceiling. The host
  granted every one of the 2,047 pages the probe asked for — taking linear memory
  to 142,147,584 bytes, past the platform's own documented 128 MiB per-isolate
  limit — and refused nothing. So the verdict is *(c) the condition is not
  constructible here*, which is an answer rather than a shrug: it names the
  missing element (an isolate that enforces the documented ceiling) and what
  would supply it, and it records that at that ceiling the arithmetic puts the
  firing payload at 36,604,834 bytes — thirty-five times the 1 MiB this store
  will accept.

  The encoder was exercised, and that is asserted rather than assumed. At the
  340 KiB reference payload the human-readable path costs 464,218 bytes against
  the binary path's 348,163 — the two figures ADR-0016's own measurement table
  publishes, reproduced here on `wasm32` from the same published seed — and it
  grew linear memory by 31 pages against 17 on identical bytes. A run in which
  those two agreed would mean the encoder was never reached, and the probe fails
  rather than reporting one.

  No wire format changed, no clause moved, and no public API gained anything:
  the three dependencies this needed live only in
  `[target.'cfg(target_arch = "wasm32")'.dev-dependencies]`, so
  `cargo tree -p happenstance-cloudflare -e normal` is unchanged and `base64` —
  the crate `happenstance-core`'s `serde` feature adds — is absent from that
  graph entirely.

- **The Cloudflare Durable Object adapter declares its capacity limits and arms a
  mid-batch fault, so three rules that skipped everywhere now run somewhere.**
  `CloudflareFixture` states `MAX_EVENT_DATA_LEN = 1 MiB`,
  `MAX_TAGS_PER_EVENT = 1024` and `MAX_EVENTS_PER_BATCH = 1024`, and
  `CloudflareEventStore` refuses above each of them before issuing any SQL — which
  is what lets the refusal name *which* ceiling was crossed.

  `append_reports_exceeded_store_limits` detects a store that accepts a payload
  one byte over its stated ceiling, that refuses it through `AppendError::Store`
  where a caller cannot tell a capacity refusal from a transient failure, that
  truncates instead of refusing, or that clamps a batch to what one statement can
  carry and answers `Ok` for the part it wrote. Until now it had never run against
  a real adapter: every fixture in the tree left all three constants at `None`, so
  it reported a skip everywhere and certified nothing.

  `MID_BATCH_FAULT` is claimed for the first time in the workspace, with the
  mechanism stated as CF-39 requires: a real SQLite trigger on the `event` table
  — `BEFORE INSERT … RAISE(ABORT, …)` behind a countdown — armed by the fixture,
  so it is SQLite that refuses the *k*-th row, inside the statement the adapter
  itself issued. The store cannot absorb it: a Durable Object rejects transaction
  control through `sql.exec()`, so there is no `SAVEPOINT` to roll back to and
  the adapter undoes the batch itself.
  `arming_a_mid_batch_fault_makes_the_append_fail` detects a fixture that claims
  the capability with an empty arm, and `append_is_atomic_under_a_mid_batch_fault`
  detects a store that leaves the rows it managed to write behind. The negative
  control was run: with the arm removed, the first goes red naming the
  `NoopFaultFixture` shape.

  A trigger rather than a hook on the JavaScript host this crate ships for its
  own tests, and the difference is the claim's whole content: a host-armed fault
  is a property of the **double**, and swapping the runtime underneath the
  adapter would take the capability with it while two rules went on printing
  green. The standing control is
  `fixture_contract::the_armed_fault_is_a_real_trigger_inside_the_store`, which
  reads the trigger's own `RAISE` text back out of the caller-visible error —
  a string that exists nowhere but in SQL.

  **The three numbers are this adapter's declared refusal policy, not
  measurements of a physical wall.** No per-value wall was observable on the
  executing host at 8 MiB of payload, 16,384 tags or 8,192 consecutive inserts,
  and that host is a Node process rather than `workerd`, so it enforces none of
  the Durable Object platform's documented caps. Each number is instead seeded
  from Cloudflare's documented 2 MiB row cap and reduced by this adapter's
  measured per-row overhead; the failure to locate the wall is recorded as the
  central finding of `experiments/durable-object-limits/`, which is reproducible
  and deliberately not in the gate. What the gate checks on every run is the
  promise CF-40 actually makes — the declared value accepted, one more refused,
  naming the ceiling it crossed — in both directions. The constant that enforces
  them is called `Ceilings::DECLARED` and the adapter-local guard is called
  `the_three_store_limits_are_declared_not_defaulted`, so a reader who greps a
  name and never opens the experiment is told the same thing this paragraph
  says.

- **The Cloudflare Durable Object adapter now runs the event-store conformance
  suite, on `wasm32-unknown-unknown`, inside `cargo xtask ci`.** Every rule
  `for_each_event_store_rule!` declares is executed against a
  `CloudflareFixture` under `wasm-bindgen-test-runner` — not compiled, not
  asserted in prose, executed, in the same command and the same terminal scroll
  as the rest of the gate. It is CF-23's third harness (`wasm-bindgen-test`)
  applied for the first time to an adapter rather than to the testkit's own
  fixture, and it is the first time any rule has run against the workspace's only
  `!Send` store on the target the two-flavour port design (ADR-0001) was paid
  for.

  **What it runs against, precisely: not `workerd`.** The storage under the
  adapter is a `DurableObjectState`-shaped shim shipped in this crate, backed by
  Node's own `node:sqlite` — real SQLite reached through `worker`'s real
  `wasm-bindgen` externs by the same `state.storage().sql()` a
  `#[durable_object]` class calls, so what is doubled is the runtime and never
  the adapter. There is no isolate, no eviction, no hibernation and none of the
  platform's own storage ceilings. Getting a `workerd`-class runner
  (`wrangler` / `miniflare` / `vitest-pool-workers`) inside `cargo xtask ci` is
  an escalated **blocking finding** rather than a degradation this project
  absorbed, and it is ADR-0023's to settle; until it is, the two provisional
  answers are the capacity limits above and what a real isolate restart would do
  to an acknowledged write.

  The harness is three lines of the shipped `event_store_conformance!` macro. It
  defines no emitter, names no rule and carries no `#[cfg]` over any individual
  rule, because the one wrong implementation nothing in this tree could grep for
  is a bespoke emitter that quietly drops the rules a runtime cannot pass:
  `no_orphan_rules` reads `suite.rs`'s source and never looks at a harness, and
  `capability_skips_are_reported` runs inside the testkit's own binary. The
  detector is therefore outside the process — `xtask`'s executed-target registry
  asserts every enumerated rule out of the target's own `cargo test -- --list`
  before the run starts, so an emptied, `cfg`-ed-away, renamed or under-emitting
  target fails with a message naming what is missing rather than exiting 0 on
  `running 0 tests`.

  The adapter joined by **adding a row** to that registry: no second gate step,
  no second runner wiring, no second `--target` plumbing.

  `CloudflareFixture` declines nothing, so this run prints **no** `SKIP` line of
  its own — which would have left CF-18's reporting path with no live instance
  anywhere in the gate. `fixture_contract` supplies one: a declining fixture
  defined beside the real one, handed to two of the shipped capability-gated
  rules, whose `SKIP <rule>: fixture declines <CAPABILITY> — <reason>` lines are
  emitted through the same `console_log!` sink `__emit_wasm` uses and asserted
  on. The same fixture is what makes the reason-quality guard non-vacuous: it
  had been walking three `SUPPORTED` capabilities and asserting nothing.

  What this run does not claim is stated where a reader lands, in the crate's
  own documentation: the
  concurrency family is not invoked, because it binds `Store: Send` and needs
  threads to spawn, and a `!Send` adapter on a single-threaded target cannot
  invoke it and is not expected to; the model family is not in the dependency
  graph on this target at all.

- **`cargo xtask ci` now *executes* the conformance rules on
  `wasm32-unknown-unknown`, rather than compiling harnesses that run
  elsewhere.** Two rows join the gate. `wasm32 run of the conformance rules`
  drives every `wasm32`-capable conformance target in `happenstance-testkit`
  through `wasm-bindgen-test-runner`: the event-store rules against
  `MemoryFixture` (`memory_conformance_wasm`), the same rules against
  `LocalMemoryEventStore` — the workspace's only genuinely `!Send` store
  (`local_conformance`) — and the projection rules against
  `MemoryProjectionFixture` (`projection_conformance_wasm`), each held to its own
  enumeration. In the same command a contributor types, and on all three runners
  the CI `gate` job matrices over rather than on `ubuntu-latest` alone. Before
  this, every wasm32 step in the gate was a `cargo check`, and `#[tokio::test]`
  type-checks for that target and then cannot run on it: precisely the gap CF-23
  is about. The standalone `wasm-conformance` GitHub Actions job that used to
  carry this claim is retired in favour of the in-gate step, and its
  `Cargo.lock`-resolved `wasm-bindgen-cli` version and install moved into the
  `gate` job.

  The second row, `wasm32 conformance targets are non-vacuous`, is what makes
  the first safe to probe. `cargo test` exits 0 on `running 0 tests`, so naming
  a target is checking a filename — the argument `xtask/src/proof.rs` was
  written for, now applied to targets nobody was running. Before anything
  executes, every rule the target's own enumeration declares must appear in its
  `--list`, which is what a wasm32-only subset would fail; and a second,
  **mandatory** step reads each target's source with no runner at all, so an
  emptied file, one rewired away from its `wasm32` emitter, one carrying a
  hand-written rule list, or a deleted registration fails on every machine —
  including the machines where the run itself prints `skipped:`. That row also
  scans the harness directory and fails a `wasm32` harness with no registration,
  which is what holds the list to the retired job's coverage rather than to
  whoever last edited it. The executed targets are a declared list, so the
  Cloudflare conformance target arrives as a row rather than as a second step.

  The run passes `--nocapture`, and that is not verbosity: `println!` is a
  silent discard on `wasm32-unknown-unknown`, so a fixture's
  `SKIP <rule>: <reason>` line reaches the terminal only when the runner is told
  not to capture. It is this target's idiom for the `--show-output` the host
  `tests` step carries, and not a synonym — `wasm-bindgen-test-runner` rejects
  `--show-output` outright. The gate also refuses a runner whose version does
  not match the `wasm-bindgen` in `Cargo.lock`, naming both versions and the
  `cargo install` line that reconciles them, because the alternative is a schema
  mismatch surfacing part way through a test binary as though the suite had
  broken.

- **`recorded_time_survives_a_reopen` finally has a negative control that reaches
  its own sentence.** The rule asserts three things in order — the event survived
  the reopen, it is at the same position, and *`recorded_at` is unchanged* — and
  only the third is its reason for existing. Until now the one registered store
  that failed it, `LosingFixture`, failed at the **first** assertion: its events
  are gone, so the stamp comparison never executed and the rule's headline
  sentence had never been shown to bite. `RestampingFixture` is the control that
  does. The defect it encodes is a real first schema: a migration that stores
  payload, type and tags and **no `recorded_at` column**, so opening the store
  reconstructs the log by replaying rows and stamping them at open time. Every
  event still returns, at its own position, under its own identity — the single
  thing lost is the one clock reading whose provenance the log itself attested,
  and an adapter that ships it hands every auditor the time of the last restart
  with no error and no symptom. It fails exactly that one rule and passes
  `acknowledged_writes_survive_a_reopen` and
  `reopened_store_does_not_reissue_an_event_id` beside it, which is what makes it
  a scalpel rather than a second `LosingFixture`; its registry row pins the
  headline message, so a failure at the survival anchor is reported as the wrong
  failure instead of counted as a pass. The re-stamped value is derived from a
  per-fixture reopen generation rather than from a clock: the binary's correct
  stamp is a **constant**, so a naive re-stamp lands on the value it replaced and
  is invisible — that run happened, and the harness reported the store as
  *declared to fail and passed* — while a wall clock would both violate the
  no-clock rule and make the comparison a race at millisecond resolution.
- **`happenstance-testkit` ships a benchmark harness, behind an off-by-default
  `bench` feature — and it is deliberately not part of the bar.**
  `event_store_benchmarks!(MyFixture::new())` is inherited exactly as
  conformance is, one line against the fixture you already wrote, and it adds
  **no** conformance rule: the rule-name list this crate publishes is
  byte-for-byte what it was, `suite.rs` gained no rule, and the family carries
  its own enumeration (`for_each_event_store_benchmark!`) beside the scenarios
  it names. The specification requires performance to be measured by a separate
  harness which is not part of the conformance bar, and this is that harness —
  so **no result it produces can fail a merge**: there is no threshold in it, at
  any budget, and no watchdog. Three scenarios, fixed by their two consumers
  rather than chosen: append throughput over a batch of *n*; conditional append
  under *k* contenders, reporting the committed and rejected counts *separately*
  and distinguishing a condition violation from a store failure, because a run
  in which nobody collided is a measurement of the wrong thing however fast it
  was; and replay of *N* events, once unfiltered and once behind a tag filter.
  *n*, *k* and *N* are the caller's, supplied at the call site. **The clock is
  the caller's too.** Nothing in this crate's `src/` may read one, so the
  harness reports counts and the per-scenario wrapper is the `emit` parameter —
  which puts `criterion`, `divan` or a CSV writer in *your* `dev-dependencies`
  and leaves this crate's exactly two. Four public items behind the feature, in
  the new `bench` module: `BenchmarkParams`, `BenchmarkRecord`, `BenchmarkPass`
  and `scenarios`, plus the macro and two emitters. The feature is additive and
  off by default, so nothing an existing consumer sees moves; it is also
  target-gated, because a Cargo feature is not target-scoped and
  `--all-features` would otherwise reach `wasm32-unknown-unknown`.
  `tests/memory_benchmarks.rs` runs the whole family against `MemoryFixture` on
  every `cargo test --features bench`, so it ships having been executed rather
  than merely compiled.
- **The Durable Object adapter reads.** `happenstance-cloudflare` — still
  `publish = false`, and noted here because it is the workspace's `!Send`
  instrument rather than because it ships — implements `EventStore::read` as
  ADR-0011's **ceiling-and-page**: a position ceiling captured no later than the
  first poll, every statement after the first bounded by it, and no
  `SqlStorageCursor` held across a suspension point. That one mechanism is what
  makes a read on a runtime whose cursor is documented as *not* a stable
  snapshot still be **one sample** (ES-11) that **all items of one query share**
  (ES-12); both clauses name this adapter as the falsifier they were most at
  risk from, and it does not bite. No conformance rule was added or changed, so
  nothing an adapter author is held to moved. With it the crate's last
  `todo!()` is gone and the scoped `#![allow(clippy::todo)]` left with it.
- **The Durable Object adapter's `append` is all-or-none, and now by doing
  rather than by believing.** It had documented the guarantee as the runtime's:
  nothing is awaited mid-batch, so the turn's implicit transaction was taken to
  cover it. That conflated isolation with atomicity. A Durable Object commits
  the turn's writes when the handler returns *normally*, and this adapter
  converts a thrown statement into `Err(…)` and returns normally — so the rows
  written before the throw would have committed. `append` now compensates
  explicitly, discarding the position range a failed batch was assigned, which
  is exact rather than best-effort precisely because nothing is awaited in
  between. A discarded batch leaves a **gap**, never a reused position, because
  an `EventId` is `(store, position)`. When the discard itself fails — an object
  out of room fails the `DELETE` as readily as the `INSERT` — the caller is told
  through the new `CloudflareEventStoreError::PartialBatch`, which carries both
  failures and is the one outcome of `append` after which a retry is unsafe.
- **The gate executes the Cloudflare adapter's `wasm32` tests instead of only
  compiling them.** `xtask` gained `WASM_UNIT_TARGETS`, a registry beside
  `WASM_TARGETS` for `wasm32` test targets that run no conformance rules, and a
  completeness scan over every crate's `src` tree so a
  `#[cfg(all(test, target_arch = "wasm32"))]` module cannot execute nowhere
  unnoticed — the miss the previous milestone's harness scan could not see,
  because it read one directory. `cargo xtask ci` now runs the adapter's
  eighty-one cases on `wasm32-unknown-unknown` wherever the runner resolves.
- **CF-17 says what declaring `REOPEN` commits a fixture to, and the registry
  carries the fixture that lies about it.** A fixture declaring
  `Fixture::REOPEN` supported MUST make `reopen` discard process state over a
  medium that outlives the process's hold on it, MUST state the mechanism, and —
  where its store has no such medium — MUST decline the capability with that as
  its stated reason. **No rule enforces it, and that is the finding rather than
  an omission.** `MID_BATCH_FAULT` is closable because arming it forces the
  append to answer `Err`, which is what CF-39 is written on; reopening has no
  port-observable consequence at all, so a rule rejecting an empty `reopen` over
  a `Vec` would reject an honest one over a real file with it.

  The claim is stronger than "no rule happens to catch it", and the testkit now
  carries the evidence rather than the argument: `LiveHandleReopenFixture` is
  `happenstance-sqlite`'s fixture in miniature — an honest reopen that closes the
  connections the *fixture* holds and leaves the medium alone, because reopening
  a file does not replace it — and it is indistinguishable from an empty `reopen`
  on every observation anyone has proposed. An honest reopen that *replaces* the
  live log is distinguishable, so two honest fixtures sit on opposite sides of
  that partition with the liar on one of them.

  What is new is that the hazard is *stated* and its wrong implementation is
  driven. `NoopReopenFixture` — `REOPEN: SUPPORTED`, `async fn reopen(&self) {}`,
  over a completely correct volatile store — is pinned by
  `reopen_over_claiming_is_undetectable_and_this_is_the_record`, which drives it
  through every rule and asserts the two things that are measurable: it fails
  none of them, and it converts `acknowledged_writes_survive_a_reopen`,
  `reopened_store_does_not_reissue_an_event_id` and
  `recorded_time_survives_a_reopen` — this suite's whole durability certification
  — from reported skips into passes, while an honest twin one line apart reports
  them as skips. The incentive inversion is now measured in-tree and goes red if
  it ever stops being true. No conformance rule was added, so no adapter's build
  changes.

### Removed

- **`happenstance-sqlite` no longer re-exports `tokio`.** It did, briefly and
  unreleased. `tokio::task::JoinError` and `tokio::runtime::TryCurrentError` are
  variants of this crate's error enums, so `tokio` qualified for the set on the
  arithmetic above — but this crate takes it at `features = ["rt"]`, so
  `happenstance_sqlite::tokio` was a **partial** `tokio`, with no `macros`, no
  `rt-multi-thread` and no `time`. A consumer who reached the type through that
  path and then wrote `#[tokio::main]` got an `error[E0433]` *further* from its
  cause than the mismatch the re-export was there to prevent, which makes it a
  longer route to the type rather than a shorter one.

  **If you match on `JoinError` or `TryCurrentError`, add `tokio = "1"` to your
  own manifest.** Cargo unifies it with this crate's for any semver-compatible
  requirement, so type identity survives for every consumer who already had a
  `tokio` line; a consumer who pins a different *major* gets the
  two-types-that-print-identically failure, and `cargo tree -d` names it. The
  omission is fenced by a `compile_fail,E0433` doctest on the old path
  (`crates/happenstance-sqlite/src/lib.rs:154`), so re-adding the re-export
  turns a test red rather than passing unnoticed.

### Changed

- **`SqliteEventStore::planned_statement_count` counts the real partition, so
  the number it returns for a query of wide items has changed.** The signature is
  untouched and the count is unchanged for every query whose items are narrow —
  which is every query any test in this workspace had built. For a query of 400
  items carrying 128 tags each it used to return `1`, and that one statement
  could not be prepared. It now returns the number of statements that will
  actually run.

  Anyone who had pinned the old number for a wide-tag query was pinning a plan
  the driver refuses. Whether this function continues to mean *"arms"* or is
  understood from here as *"the partition"*, and whether
  `MAX_QUERY_ARMS_PER_STATEMENT` stays public now that it is no longer the whole
  partition, are public-surface decisions deliberately not settled here; a brief
  for both is staged in `.kb/_intake/`.
- **BREAKING (`happenstance-testkit`, `proptest` feature): `Op::Read` gained a
  `to` field, and the model can now disagree about an upper bound.** The variant
  was documented as carrying *"every read option in play"* and carried four of
  five. `to` was the missing one, so the generator emitted no upper bound,
  `Model::apply` never called `.to(..)`, and the two `to` branches of
  `Model::select` were dead code — under a comment stating, correctly, that a
  model which ignores an option *"would agree with every implementation, which
  is the one thing a reference model must not do"*.

  It had agreed with three. `ToBoundIgnoredStore` (the options struct matched on
  the fields the adapter recognises), `ToIsExclusiveStore` and
  `BackwardsToIsAnUpperBoundStore` were all recorded as passing the model, and
  all three are rejected now without any of them changing. A fourth store was
  written for this release and exists only because the model can see it:
  `UnparenthesisedToPredicateStore`, `WHERE a OR b AND position <= ?` — the
  precedence bug the suite already registers for the *lower* bound, one bound
  over. Every rule that exercises `to` issues `Query::all()`, so no rule in the
  ninety can see it; the wrong outcome is a bounded backfill worker reading past
  its own window and re-delivering events the tail worker has already processed.

  What breaks: `Op` is reachable as `happenstance_testkit::model::Op` whenever
  the `proptest` feature is on, and adding a field to a struct-form variant of a
  `pub enum` with no `#[non_exhaustive]` breaks any downstream `match` written
  with a struct pattern, and any construction. `Op::Read { query, from,
  backwards, limit }` becomes `Op::Read { query, from, to, backwards, limit }`,
  with `to: Anchor::Unset` reproducing the old behaviour. Whether the variant
  should also carry `#[non_exhaustive]` — so that the *next* field is not a
  second break — is deliberately not settled here; it belongs with the crate's
  public surface at first publish, and a brief for it is staged in
  `.kb/_intake/`.

  The `to` bound is generated weighted towards absent, four reads in five, and
  that weighting is measured rather than tidy: sampling it the way the other
  four options are sampled dilutes every combination of them, and the first
  version of the change lost `LimitPerItemStore` — a defect the model had
  rejected for three phases. `MODEL_COVERAGE` is what noticed.

### Fixed

- **`happenstance-cloudflare` chunks a wide query instead of planning it as one
  statement SQLite cannot take.** `positions_matching` was an unbounded
  `.join(" UNION ")` with no `max_arms`, no chunk and no parameter budget
  anywhere in the crate: 1,000 query items became 1,000 terms of one compound
  `SELECT` against `SQLITE_MAX_COMPOUND_SELECT`'s 500, and 400 items carrying
  this store's own declared `tags_per_event` of 1,024 apiece bound 409,600
  parameters against `SQLITE_MAX_VARIABLE_NUMBER`'s 32,766. Both walls sit above
  shapes a conformant caller may build, and on the append path the failure would
  have arrived inside the turn as `AppendError::Store` carrying a raw driver
  string, with the caller's decision already taken.

  It is replaced by one entry point, `query_sql::chunks`, partitioning on both
  limits — and by one entry point only: keeping a second, unchunked spelling
  beside a chunked one is exactly how the sibling's write path stayed unchunked
  while its module doc claimed otherwise, and that note is why. Both callers
  merge: the append-condition guard folds the per-chunk maxima by `max`, which is
  exact because a guard is an inequality on the highest match; the read path
  merges pages.

  **The read merge diverges from the sibling deliberately.** It sorts,
  de-duplicates and truncates after *every* statement rather than after all of
  them, bounding resident rows at one page plus one chunk instead of
  `chunks x page`. A Durable Object is a single isolate with a real memory
  ceiling, and the incremental truncation is exact rather than approximate: a row
  already beyond the *n*-th position of a prefix of the chunks is beyond it in
  the full union too.

  One consequence worth stating, because it is observable: a row whose
  `position` column is not an integer at all now ends the read one step earlier
  on a multi-statement plan than on a single-statement one, because the merge
  has to order by it. A one-statement plan — every query any conformance rule
  builds — takes neither the sort nor that decode, and is byte-identical to the
  read this replaced.

  It is proven by execution rather than by arithmetic.
  `tests/wide_query_ceiling.rs` drives a query past each wall — 1,000 compound
  terms against a limit of 500, and 32,800 bound parameters against 32,766 —
  through `EventStore::read` and through an append-condition guard, against the
  real SQLite behind the `DurableObjectState` shape, and its first case is a
  control that hands this runtime the unpartitioned statement and watches the
  driver refuse it. Against the previous implementation six of its seven cases
  fail with SQLite's own *"too many terms in compound SELECT"* and *"too many
  SQL variables"*; the seventh is the control, which passes either way because
  it asserts a fact about the engine. The conformance suite cannot reach either
  wall — its widest query is 128 items at one tag each — so the target is
  registered in `xtask/src/proof.rs`'s `WASM_UNIT_TARGETS` and runs under
  `cargo xtask wasm-conformance`, in 0.35 s.
- **`happenstance-sqlite` no longer fails past `SQLITE_MAX_VARIABLE_NUMBER` on a
  wide query — on either path, and the append path is the one that held the write
  lock.** `query_sql::chunks` partitioned on item count alone, so a query of 400
  items — the crate's own chunk width, exactly — carrying `MAX_TAGS_PER_EVENT`
  tags apiece was planned as one statement binding 51,200 of SQLite's 32,766
  parameters. `Selectivity::read_for` was not partitioned at all: it accumulates
  every distinct tag of every multi-tag item across the *whole* query into one
  `IN (…)`, so 16,384 ordinary two-tag items reached the same wall with no wide
  item anywhere, and it reached it *first*, because it runs before the chunking
  on both callers.

  Both shapes are ones a conformant caller may construct — nothing in
  `happenstance-core`'s `Query` bounds tags per item, and 128 is this store's own
  `MAX_TAGS_PER_EVENT` — and both arrived as `AppendError::Store` wrapping
  SQLite's *"too many SQL variables"*. On the append path that is inside
  `BEGIN IMMEDIATE`, with the write lock held and the caller's decision already
  taken, which is the timing VT-24 rejects by name; VT-23's `Rejects:` line names
  the implementation itself, *"an adapter that generates one SQL parameter per
  item and silently fails past a driver limit"*.

  Both sites now partition on both of SQLite's pushdown limits, at the one entry
  point the module already had. Nothing is refused that was served before: a
  wider query becomes more statements, merged exactly as the arm partition's
  already are. `crates/happenstance-sqlite/tests/wide_tags.rs` is the standing
  guard — the conformance suite cannot reach either wall, because its floor is
  128 items at one tag each — and it asserts the partition one parameter under
  the budget, exactly at it, and one over.

## [0.2.0-alpha.1] — 2026-08-16

**The first published release, and it is a pre-release on purpose.** The API is
expected to move until the stable `0.2.0`; only one alpha resolves at a time,
and each is yanked when the next lands. `happenstance-testkit` publishes on its
own number, `0.2.0-alpha.1`, which moves *down* from its in-tree `0.2.0`: CF-32
gives it an independent number, not an independent maturity, and a stable suite
over a moving port is the promise this release refuses to make.

Everything under `### Added`, `### Changed` and `### Fixed` below was written as
it landed, phase by phase, rather than reconstructed from `git log` at release
time.

### Added

- **`happenstance-testkit` ships two stores that misbehave on purpose, because
  every store a consumer could reach behaves perfectly.** `FaultyStore<S>` and
  its `Send` sibling `SendFaultyStore<S>` wrap any event store and can be armed
  — `violate_next(n)`, `fail_next_read(n)` — to refuse an append with
  `ConditionViolated` naming **no** conflicting event, or to fail a read at its
  first polled item. The defect that detects: a retry loop that branches on
  `ConditionViolated::conflicting_position` being `Some`. It works against every
  in-process store and never retries against one reached over one-shot HTTP,
  which reports `None` conformantly — so until now the input that separates a
  correct caller from an incorrect one did not exist in-process at all.
  `GappyMemoryStore::with_stride(NonZeroU64)` is the second: a fully conformant
  store whose positions advance by a caller-chosen stride. The defect that
  detects: a read-model handler that computes its next position by adding one.
  Both stores run the full conformance suite themselves — the wrapper *unarmed*,
  so a disarmed fixture is proved not to be lying about ordering, positions or
  identity, and the gapped store with its stride, so its gaps are proved to be
  the freedom VT-11 grants rather than a defect. No conformance rule is added,
  so no adapter's CI can turn red because of this. Four items, all at the crate
  root: `FaultyStore`, `SendFaultyStore`, `FaultyStoreError<E>` — the projected
  error the port's associated type forces, because `MemoryStoreError` is
  uninhabited and there is no `S::Error` to fabricate — and `GappyMemoryStore`.
- **`happenstance-testkit` declares a `memory` feature, and it is on by
  default.** It gates `GappyMemoryStore` and forwards to a `happenstance-core`
  feature this crate already enabled unconditionally, so it adds no dependency
  and changes no resolution. What it buys is a switch `--no-default-features`
  can turn off, which is what keeps the feature powerset honest about a `#[cfg]`
  over a `pub` item.
- **`projection_store_conformance!` — a fourth rule family, and the first line
  an adapter author can write against `ProjectionStore`.** One line in your own
  `tests/` expands to one test per projection rule, named after the rule, on any
  of three runtimes: `__emit_projection_tokio` (the default),
  `__emit_projection_blocking` (no async runtime at all) and
  `__emit_projection_wasm`, which routes a skipped rule's stated reason to
  `console_log!` because `println!` writes nowhere on
  `wasm32-unknown-unknown`. The set is written in
  `for_each_projection_store_rule!` and nowhere else, and
  `no_orphan_projection_rules` fails by name if a rule is declared without being
  registered. It takes a `ProjectionFixture` — a new trait beside `Fixture`,
  whose `Store` is bound on `ProjectionProbe` rather than on `ProjectionStore`,
  so an adapter that cannot be observed cannot invoke the suite.
  `fixtures::MemoryProjectionFixture` and `fixtures::MemoryProjectionHandle` are
  the reference implementation to read first. `happenstance-testkit` now enables
  `happenstance-core`'s `conformance` feature unconditionally; that adds no
  dependency and no feature of its own.
- **A declined projection capability is a reported skip, never a silent
  absence.** `ProjectionFixture` gains three `Capability` constants —
  `SECOND_HANDLE`, which is a **MUST**; `RESET_REFUSAL`, which a store with
  no protection policy declines honestly; and `COMMIT_FAULT`, which says
  whether the store can be made to fail a `commit` — in the vocabulary the event-store
  family already uses, with no projection-local skip type and no second line
  shape. The defect this machinery detects is the one CF-18 names by hand: a
  capability-gated rule `#[cfg]`-ed out of the expansion, so that an adapter
  author who declares a capability unsupported to turn a red build green gets a
  green build and **no record of the trade** — a rule absent from the binary
  being indistinguishable, in CI output, from a rule that passed. The check is
  `mutation_coverage::projection_capability_skips_are_reported`, which drives
  the whole projection enumeration against a fixture that declines everything
  and asserts on `RuleOutcome` values rather than on stdout, because libtest
  exposes nothing programmatically. Declining `SECOND_HANDLE` **fails** rather
  than skips, carrying the fixture's own stated reason: a projection store whose
  commit is visible only to the connection that made it cannot be observed to
  keep PS-1 at all, so treating that as a recordable trade would certify an
  adapter nothing ever looked at twice.
- **`commit_advances_the_checkpoint`** — the first of the projection family's
  rules, and the baseline the rest of §4.11 is differential against. It detects
  a `commit` that returns `Ok` and makes **neither** the read-model row nor the
  checkpoint durable — an adapter that reports success and advances nothing, so
  a runner re-reads the same events forever and the read model never moves.
  That defect is not a straw man: PS-1's MUST is a *coupling* rather than a
  progress obligation, so "neither durable" satisfies the clause through its "or
  not at all" arm and passes the atomicity rule below. This is the rule that
  fails it. The checkpoint is read back through a **fresh handle**, which also
  catches a store whose commit is visible only to the connection that made it,
  and it is compared against the position the commit was given rather than
  against any literal, because the specification permits gaps.
- **`commit_is_atomic_with_the_read_model`** — PS-1 itself, and the invariant
  the projection port exists for. It writes a probe row into a batch, commits at
  a position, then reads the row **and** the checkpoint back through fresh
  handles and requires both present or both absent, never one. The defect it
  detects is a store that advances the checkpoint and silently drops the
  read-model write — the natural shape for any adapter whose read model lives
  somewhere other than its checkpoint table, and one whose consequence is a
  projection that skips every event the discarded batch would have applied, with
  no error anywhere and nothing in the log to find afterwards. It asserts the
  coupling and deliberately nothing else, so a failure here means one half
  landed without the other rather than something the baseline rule already owns.
  Its wrong implementation, `CheckpointOnlyStore`, is registered in the
  projection mutant registry and fails this rule by name, so the rejection is
  demonstrated rather than documented.
- **`failed_commit_leaves_both_unchanged`** — PS-1's *second* conjunct, the arm
  about a commit that reported failure, and the first projection rule gated on a
  capability. The defect it detects is a partial apply: an adapter that writes
  its read-model rows one statement at a time and writes the checkpoint last,
  with no transaction around the pair, so a failure on the checkpoint write
  leaves the rows durable while `commit` reports the error honestly. The caller
  is told the batch was refused; part of it has silently been applied, and the
  checkpoint that would have recorded it is not there. Every other rule in the
  family only ever sees a commit *succeed*, so a store can keep the first
  conjunct perfectly and still leak half of every failed batch. Reaching the
  failure needs the store's co-operation — nothing a caller holds can make a
  conformant `commit` fail — so the rule is gated on the new
  `ProjectionFixture::COMMIT_FAULT` and reports a skip carrying the fixture's own
  reason where the store has no fault to arm. `MemoryProjectionFixture` is such a
  store and declines, so the reference run prints a `SKIP` line for this rule;
  `PartialCommitStore` is the registered wrong implementation that fails the rule
  by name. (It is not the only one — `refused_reset_changes_nothing` below is
  declined too. `assert_reference_projection_declensions` pins the whole set by
  equality rather than any count written in prose.)
- **A projection mutant registry, and nine wrong stores in it.** The projection
  suite can now be shown to *fail* something, which is a different claim from
  passing against the oracle and is the only one worth anything to an adapter
  author. `tests/projection_mutation_coverage.rs` carries a hand-written
  `REGISTRY` of stores and the exact rules each fails, four meta-tests over it —
  every rule has a mutant, the registry and the store enumeration agree, each
  mutant fails **exactly** what it declares, and every row states the adapter
  shape that makes it plausible — and a `Defect` seam in which each hostile store
  overrides one step of a correct core. No pass rate is quoted anywhere over the
  set: the denominator is an author's choice, so a fraction reports how
  representative the author was while reading as though it reported how good the
  suite is. CF-5's conformant variant is named as an open hole rather than
  asserted over an empty set.
- **`rollback_leaves_both_unchanged`** — PS-8, and the reason `rollback` stays on
  the port even though a buffered batch could just be dropped: Rust has no
  `async Drop`, so an adapter holding a real transaction has no way to issue
  `ROLLBACK` and await it from a destructor. The defect it detects is an adapter
  whose `rollback` clears its own statement buffer, drops the guard and hands the
  connection back **without ever sending `ROLLBACK`** — every driver with
  implicit transaction handling makes that available, `Ok` comes back, and the
  rows the batch carried are still there afterwards. A test asserting only that
  `rollback` returned `Ok` certifies it. The rule reads both halves back through
  a *fresh* handle and compares the checkpoint against what it was before the
  rollback rather than against a position, so it asserts preservation and makes
  no claim about progress.
- **`dropped_batch_leaves_store_usable`** — PS-7, and the half of it that is
  actually the rule. "A dropped batch rolls back" alone certifies a store that
  has permanently lost its only writer, so this rule drops a batch **bare** — no
  `commit`, no `rollback` — and then opens and commits a *second* batch on the
  same handle and requires that second row to read back. The defect is an adapter
  whose `begin` checks a connection out of a pool and whose `Drop` returns it to
  nothing: `commit` and `rollback` both give it back, so only the path nobody
  writes a test for leaks, and the store answers `Busy` for ever after. A
  reviewer's probe found exactly that store, which is why the clause has a second
  half at all.
- **`commit_rejects_a_foreign_batch`** — PS-15. A batch begun on one store
  instance and committed on another must be refused as
  `CommitError::ForeignBatch`, and **neither store may move**. The defect is an
  adapter that stamps the batch per *type* rather than per instance — a `const`,
  a `Default`, a hash of the connection string — which is indistinguishable from
  correct in any test holding one store and lets a runner with two stores commit
  one projection's rows into the other's database. The check is at run time
  because the type-level fix was compiled and refuted: a lifetime names a region
  rather than an instance. This is the one rule in the family that does **not**
  want a second handle onto one store; it wants two isolated stores, which two
  `open()` calls already produce.
- **`commit_accepts_a_position_the_batch_did_not_write`** — PS-21, and the clause
  that makes a checkpoint a high-water mark of *consideration* rather than of
  application. The defect is an adapter that validates `position` against what
  the batch wrote, and registering it matters precisely because the misreading is
  **reasonable**: "advances `id`'s checkpoint to `position`" reads like a claim
  about applied work, and without this rule a validating store would be exactly
  as conformant as one that accepts. Two backends could disagree and both pass,
  which is a silent interoperability difference rather than a capability gap.
  What it costs in the field is a narrow projection — forty matches in
  thirty-seven thousand events — re-scanning the same range for ever on every
  restart.
- **`commit_rejects_a_regressing_position`** — PS-22. A commit naming a position
  strictly below the current checkpoint is refused as
  `CheckpointRegression { current, attempted }`, carrying both values so a caller
  can log the gap rather than re-derive it, and neither half moves. The defect is
  `UPDATE checkpoint SET position = ?` issued unconditionally, which is what
  everyone writes and which is correct until two runners share an id: under a
  redeploy where an old pod has not yet exited, the stale runner drags the
  checkpoint backwards and every event between the two positions is applied
  twice. The rule deliberately asserts nothing about an **equal** position,
  because the clause permits accepting one.
- **`distinct_projections_advance_independently`** — PS-23. Two projections in
  one store advance at their own rates, and neither one's commit may disturb the
  other's checkpoint or its rows. The defect is a checkpoint table with one row,
  one position column and no key — what a store that has only ever run one
  projection will write. The fastest projection drags every other one's
  checkpoint forward, and the slower ones skip every event between the two
  positions permanently, with nothing reported. It passes every other rule in the
  family, which is why this one has to exist separately.
- **`reset_clears_rows_and_checkpoint_together`** — PS-16, and the rule that
  makes `reset` one unit of work rather than two statements that usually both
  run. The defect it detects is the runbook procedure: clear the read model on
  one connection, update the checkpoint on another. That is what Norvant's night
  desk executed at 02:46:31, and the pod died at 02:46:33 before the second
  statement — so the runner restarted, read the *old* checkpoint, resumed past
  it, applied sixty-one events into an empty table and reported healthy. Both
  halves are read back through a **fresh handle** and asserted together, because
  the pairing is the claim and neither half alone is one. The rule also carries
  PS-16's failure-injecting arm without needing a fixture that can arm a fault:
  a batch begun on a different store instance is refused as
  `ResetError::ForeignBatch`, and a `reset` that errored must leave both halves
  exactly as they were.
- **`reset_is_scoped_to_one_projection`** — PS-17. The defect it detects is a
  `reset` that truncates the checkpoint table — one statement, no `WHERE`,
  obviously correct until a second projection shares the file. Kestrel Cold
  Chain's does: `van_stock` is rebuilt several times a day across 138 devices,
  and `fgas_ledger` is a hash chain a regulator already holds and must never be
  rebuilt at all. The rule commits rows and a checkpoint under **two** ids in one
  store and asserts the sibling's rows and checkpoint are untouched, which is
  what makes it non-decorative: without the sibling the same code passes and the
  ledger is destroyed in the field. It hands `reset` an **empty** batch on
  purpose, so that every row that disappears is the store's own doing rather than
  the caller's `probe_delete_all`.
- **`refused_reset_changes_nothing`** — PS-18, and the family's second
  capability-gated rule. An adapter must be able to refuse a reset for a
  projection its domain protects, and the operative half is that a refusal
  **changes nothing**: the defects it detects are a policy enforced anywhere
  except inside `reset`, so the call returns `Ok` and does the work, and a policy
  checked *after* the deletes have gone out, so the refusal is reported perfectly
  honestly over a read model that is already gone. A rule stopping at
  `matches!(err, ResetError::Refused)` certifies the second. Reaching a refusal
  needs the store's own policy, so the rule is gated on
  `ProjectionFixture::RESET_REFUSAL` and names the projection to protect through
  the new `ProjectionFixture::protect_from_reset`; `MemoryProjectionStore` holds
  no protection policy and declines, so the reference run prints a second `SKIP`
  line carrying that store's own words.
- **`fresh_projection_has_no_checkpoint`** — PS-38's second sentence, and the
  defect it detects is a checkpoint that answers for a projection nobody has ever
  built. A `checkpoint` resolving a missing row with
  `.unwrap_or(Checkpoint::Live { through: FIRST })` — what an author writes when
  the position column is `NOT NULL DEFAULT 1` — tells a runner that a read model
  that does not exist is authoritative and already considered through the first
  position. The runner then resumes *past* the events it has never applied, and
  every event at that position is skipped on the first run of every projection the
  store has never seen. The rule asks for one id no commit has named and requires
  the `Checkpoint::NeverRun` **variant**; it compares against no position, because
  the position it would compare against is the exact value the defective store
  writes.

  **It arrived a day late, and the delay is the interesting part.** §4.11 filed
  the rule under PS-19, whose `MUST` is scoped *after a successful `reset`* and
  therefore says nothing about an id never seen — so the first version of this
  rule convicted adapters of an obligation no sentence in the specification
  stated, and `PresumedLiveCheckpointStore`, the store registered to fail it, was
  **conformant**. The rule and its store were withdrawn rather than argued around.
  What brought them back is a clause and not a re-reading: ADR-0030 minted PS-38,
  whose second sentence is *"a `ProjectionId` no successful `commit` has named
  MUST read as `Checkpoint::NeverRun`"*, and the rule now cites that. No
  `[FROZEN]` clause was edited to make this pass; PS-19 is byte-identical across
  the whole episode. If you are writing an adapter, the practical consequence is
  that this obligation is `[PROVISIONAL]` — PS-38 falls to a store that answers
  `checkpoint` from a replica that may lag its own `commit`, and the clause names
  that falsifier itself.
- **`reset_is_not_commit_at_first`** — PS-19 and PS-20, and the rule that turns
  RUNBOOK's observation into an enforced rejection. The defect it detects is
  `commit(empty_batch, id, SequencePosition::FIRST, Live)` used as a substitute
  for a reset — **six deployment scenarios out of six reached for it and all six
  got it wrong**. It compiles, it returns `Ok` and the checkpoint moves, so
  everything anybody checks afterwards looks right; what it costs is event 1,
  permanently and silently, because a runner resumes strictly after the position
  it reads. The rule performs both operations on two ids in one store, asserts
  the checkpoints differ **by variant**, and then derives a resume point from
  each under the port's own rule — strictly after a recorded position, inclusive
  from the store's first position when the checkpoint is `NeverRun` — asserting
  the event at the first position is applied in the reset case and not in the
  substitute's. No runner is built: the derivation is four lines the rule owns.
- **`batch_reads_reflect_pending_writes`** — PS-12, and the first projection rule
  whose gate is not on the fixture. The defect it detects is a batch `get`
  implemented as one round trip on the connection the batch is already holding,
  so it answers from **committed** state: a projection doing `get` then `set`
  inside one batch reads the value from before the batch began, and every
  increment after the first is lost with no error anywhere — least visibly when
  the chunk is largest. The rule asserts the *value* it wrote, not merely that
  something came back, so a store answering `Some(0)` for everything fails it.
  An adapter whose batch offers no read path at all declares
  `ProjectionProbe::READS_THROUGH_BATCH = false`, which PS-12 permits outright,
  and gets a reported skip naming that constant — the switch is on the probe
  beside the store, because whether a batch can be read through is a property of
  the batch type rather than of the fixture's environment.
- **`rebuild_is_chunk_size_invariant`** — PS-13 and PS-14, both `[FROZEN]`. A
  rebuild runs at whatever chunk size fits the operator's memory budget, and this
  rule is what stops that becoming a correctness variable nobody logs. It replays
  one fixed sequence — `a, a, b, a, b, a` — at chunk sizes 1, 3 and whole-log
  against three isolated stores, each step a read-modify-write **through the open
  batch**, and requires the three runs' read models to be identical per key. The
  defects it detects are a batch that reads from committed state (which loses
  every repeat inside a chunk) and a batch that stages writes with
  `entry().or_insert(…)`, keeping the **first** value for a key — the natural
  spelling when a batch is thought of as a dedup buffer, whose reads are honest
  and which therefore fails only here. The increment is load-bearing: a plain
  `set` is chunk-insensitive by construction and would certify both clauses on
  nothing.
- **`rebuilding_is_distinguishable_from_live`** — PS-24. A reader deciding
  whether the rows in front of it are authoritative gets its answer from the
  checkpoint's variant, and the defect this detects is a rebuild in place behind
  a single position field: `Authority` arrives at `commit` and is dropped, so the
  checkpoint says `Live` over a half-built read model and every reader that asked
  is told yes. The rule commits two chunks claiming `Authority::Rebuilding` and
  one claiming `Authority::Live`, asserting the **variant** after each and never
  the position it carries. It builds no runner: everything it needs is on the
  port's own signature. It asserts nothing immediately after the reset that opens
  it, because a rebuild that has committed nothing correctly reads `NeverRun`.
- **A conformant projection variant, so both arms of a gate have a fixture.**
  `NoBatchReadStore` declares `READS_THROUGH_BATCH = false` and leaves
  `probe_read_through` `unimplemented!()` — the buffering and write-behind shape,
  whose batch has nothing to read from until it is sent — and is registered as
  `Kind::ConformantVariant` with an empty `fails` list. It is the projection
  registry's first conformant variant, so
  `projection_conformant_variants_pass_everything` lands with it: the only
  assertion in that binary that can point at a **rule** rather than at a store,
  because a rule over-specified beyond its clause fails a store that is
  deliberately, legally different. `NO_BATCH_READ_PATH` and
  `NO_BATCH_READ_PATH_REASON` are exported beside `NO_STORE_LIMITS` and
  `NO_CEILING_REASON` and are the second and last instance of the
  testkit-written-reason exception.
- **Five more wrong projection stores, and three declarations grown.**
  `TwoStatementResetStore` (the 02:46:31 truncate whose second statement never
  ran), `TruncatingResetStore` (`DELETE FROM projection_checkpoints`, no
  `WHERE`), `CommitAtFirstResetStore` (the substitute six scenarios reached for),
  and `RefusalAsSuccessStore` and `RefusalAfterTheFactStore` (the two halves of a
  protection policy that is not in the write path) are registered in the
  projection mutant registry, each failing exactly what it declares at a pinned
  assertion. Three existing rows grew rather than the new rules being weakened to
  preserve them, which is the exactness meta-test working: a store that makes
  nothing durable, one that stamps its batches per type, and one that validates a
  commit's position are each visible to a reset rule as well. Still no pass rate
  anywhere over the set. A sixth store — `PresumedLiveCheckpointStore`, a missing
  checkpoint row read as `Live` — is **held** with the rule it fails, for the
  reason stated above: it is conformant with PS-19 as the clause is written
  today.
- **`MemoryProjectionStore`, behind the existing `memory` feature** — the
  projection port's answer to `MemoryEventStore`, and the first implementation of
  that port anywhere that actually runs. It is the oracle a failing adapter is
  measured against, the target of a runnable `begin` → write → `commit` →
  read-back walkthrough on its own page, and the fix for the port's cold start:
  an adapter author now has something to copy. Costs no new dependency, and
  `MemoryProjectionBatch` and `MemoryProjectionStoreError` are exported beside it.
  With `conformance` also on, it implements `ProjectionProbe` and declares
  `READS_THROUGH_BATCH = true` — the apply-on-write end of the batch-shape axis
  the projection suite has to span.
- **`ProjectionProbe`, behind a new off-by-default `conformance` feature** on
  `happenstance-core` — the write seam the projection conformance suite drives an
  adapter's read model through. It lives beside the port rather than in
  `happenstance-testkit` because an adapter's own `tests/` directory is a third
  crate, where the orphan rule rejects the impl; here it costs an adapter author
  one feature flag on a dependency they already have and **no new edge in their
  dependency graph**. The feature is `[]`: no dependency, and it implies neither
  `std` nor `memory`.
- **Six adapter skeletons, as instruments rather than as adapters.**
  `happenstance-cloudflare`, `happenstance-postgres` and `happenstance-neon` are
  new; `happenstance-sqlite`, `happenstance-ladybug` and `happenstance-sync` grew
  real associated types. Every body is `todo!()` and every crate is
  `publish = false`, so nothing here changes what a consumer sees — but the ports
  have now been disagreed with by five storage shapes instead of one, and
  [`references/adapter-shapes.md`](references/adapter-shapes.md) records what each one said.
- [ADR-0009](.kb/decisions/0009-error-send-sync.md), settling ES-6 — the highest
  blast-radius open question in the workspace, and the last one that was
  semver-visible. **`Error` keeps its bound**; the stronger property becomes a
  marker trait that generic code opts into, and which turns out not to need the
  contract crate at all.
- Two mandatory gate steps building `happenstance-cloudflare` and
  `happenstance-neon` for `wasm32-unknown-unknown`. Both crates asserted that
  target in their own documentation and nothing checked it.

- **The conformance suite no longer requires `tokio`.** The rule set is
  enumerated in exactly one place, `for_each_event_store_rule!`, and the per-test
  wrapper is a parameter rather than something the testkit chooses. Three
  emitters ship — `__emit_tokio` (still the default, so existing callers are
  unaffected), `__emit_blocking` and `__emit_wasm` — and a runtime none of them
  covers needs no release of `happenstance-testkit`: write a `macro_rules!` that
  takes a list of identifiers and hand it to the registry yourself.
- `happenstance_testkit::block_on`, a twenty-line single-threaded executor with
  no `Send` bound and no dependencies. It is what makes the runtime-free harness
  runtime-free.
- A `no_orphan_rules` meta-test. A rule added to the suite without being
  registered used to produce no signal of any kind — it compiled, `cargo test`
  reported green, and an adapter was certified against twenty-six rules while
  its author believed it was twenty-seven.
- **The workspace's first `!Send` implementer of either port.**
  `LocalMemoryEventStore` — `Rc<RefCell<Vec<SequencedEvent>>>`, implementing the
  bare `EventStore` and nothing else — passes every rule under four harnesses,
  including `wasm-bindgen-test` on `wasm32-unknown-unknown` in CI.
  Until now the entire evidence base for the two-flavour port design was a
  `cargo check`: a compile of the trait, with nothing implementing the flavour
  that design exists for.
- A `wasm-conformance` CI job that **runs** the suite on
  `wasm32-unknown-unknown`, and a gate step that type-checks the harnesses for
  that target on every run. The two are different claims: `#[tokio::test]`
  type-checks for wasm32 and then cannot run there.
- [ADR-0008](.kb/decisions/0008-one-derivation-for-both-ports.md), which puts
  `ProjectionStore` under the same derivation scheme as `EventStore` — it had
  carried the identical construction since it was written and appeared in no ADR
  at all — and states what a provided body owes both flavours.
- A CONTRIBUTING section on adding a method to a port. The specification already
  claimed this was documented there; it was not.
- `cargo xtask reserve <name>`, which generates the `0.0.0` placeholder used to
  claim a crates.io name. Names are claimed one per phase, as the crate that
  justifies each becomes real.
- `happenstance`, `happenstance-core` and `happenstance-testkit` reserved on
  crates.io at `0.0.0`. Placeholders with no functionality; the first functional
  release will be `0.1.0-alpha.1`.
- A CI job running the full test suite at the 1.85 MSRV, dev-dependencies
  included. The existing job used `--no-dev-deps`, which is exactly the flag that
  hid `proptest` and `tokio` — both of which declare `rust-version = "1.85"` with
  no headroom, so the claim had never been checked.
- A gate step building the contract crate's documentation with
  `--no-default-features`.
- An inbound-equals-outbound licensing statement in `CONTRIBUTING.md`.
- `cargo xtask spec-trace`, which checks the architectural specification against
  the conformance suite and the case catalogue — maturity markers, falsifiers,
  rule names, case numbers and `file:line` citations — and **generates** §7.1 and
  §7.2 of the specification, failing the gate when the committed copy and the
  computed one disagree.
- A gate step asserting that each publishable crate's packaged artifact really
  contains `LICENSE-MIT`, `LICENSE-APACHE` and `README.md`. It parses the file
  list rather than trusting the exit status, and derives the set of publishable
  crates from `cargo metadata`, so promoting a stub is caught rather than
  remembered.
- Gate steps for the wasm32 feature powerset and, on nightly, the `docsrs`
  documentation configuration. The latter is the only thing that compiles
  `#![cfg_attr(docsrs, feature(doc_cfg))]` before docs.rs does — that is to say,
  before publication, which is the last moment it can be fixed.
- A weekly `cargo deny check advisories` job. A new advisory against an unchanged
  dependency is the one failure that arrives with no commit to trigger CI.

- **A fixture contract, replacing the bare `Fn() -> S` factory.** A conformance
  rule is now handed *how to make a fixture*, and a `Fixture` is a trait: one
  instance is one isolated backing store, each `connect()` is one handle onto
  that store, and two instances share nothing. That distinction is what the old
  signature could not express — it documented "a fresh, empty store" while
  accepting `|| store.clone()`, so no rule could call it twice without knowing
  which of the two it had been given, which foreclosed durability, reopen and
  every genuinely multi-connection check at once.
- **Capabilities, declared as associated `const`s.** `SECOND_HANDLE` and
  `REOPEN`, each either `Capability::SUPPORTED` or
  `Capability::declined("<reason>")`. A rule requiring a capability the fixture
  declines is still emitted as a test; it returns `RuleOutcome::Skipped` and the
  harness prints the stated reason. `#[cfg]`-ing it out instead would make a
  skipped rule indistinguishable in CI output from one that passed. The empty
  reason is unrepresentable: `declined` is a `const fn` that asserts.
- **The founding rules, named at last — each with the defect it detects.** The
  suite's first twenty-seven rules were only ever *counted* here. Nothing has
  been released, so every one of them ships in `0.1.0` and CF-29 obliges each to
  arrive with a changelog entry naming what it catches; the gate lint added this
  phase is what noticed that seventeen of them had none. The defects are the ones
  the mutant registry names, which is the only source for them that has been
  compiled.

  - **`append_is_atomic`** — probe-then-insert with no transaction around the
    pair: autocommit, a `SELECT` that answers the append condition, and rows
    already durable by the time the answer comes back, so `Err` is the only thing
    left to return. It is the only rule in the suite that rejects that shape
    through the *condition* path (ES-18), and a reviewer measured it passing
    every other rule as the suite then stood.
  - **`append_rejects_empty_batch`** — an insert of zero rows is a successful
    no-op in every driver, so refusing it is a decision the adapter has to
    *make*. One that passes the caller's slice straight through returns a
    position nothing was written at, which the caller then checkpoints.
  - **`append_returns_last_written_position`** — a position answered from a
    per-session cache rather than from the row just written. Strengthened where
    the fixture can open a second handle: the same claim read back through a
    connection that did not make the write, which is where a cache and a store
    stop agreeing.
  - **`condition_after_ignores_events_at_the_boundary`** — the textbook
    off-by-one. `after` is exclusive and `from` is inclusive, and one comparison
    serves both in the first draft, so every command whose caller read exactly to
    its own last event is rejected as contention it did not have.
  - **`condition_after_ignores_non_matching_events`** — `SELECT EXISTS(SELECT 1
    FROM events WHERE position > ?)`, the fast path someone adds when the tag
    join is the expensive half. It answers "has anything happened since" instead
    of "has anything I care about happened since", so a busy log refuses every
    conditional append in it.
  - **`condition_after_rejects_events_beyond_the_boundary`** — a position anchor
    spent as a row count. `OFFSET` is the parameter already in the paging query
    and a `u64` position slots into it without complaint; here spending it wrongly
    *admits* a write a concurrent command has already invalidated.
  - **`condition_rejection_is_reported_as_condition_violated`** — the DCB
    uniqueness shape implemented as a partial unique index, with the driver's
    `23505 unique_violation` passed straight through. Callers distinguish *retry*
    from *something broke* on this alone, and a store error is not retryable.
  - **`condition_without_after_allows_non_match`** — interning event types to
    avoid a string per row, where an unknown type yields an empty id list,
    `type_id IN ()` is a syntax error, and the clause is dropped rather than the
    query refused. The probe widens instead of narrowing and refuses a command
    nothing conflicts with.
  - **`condition_without_after_rejects_any_match`** — `condition.after.unwrap_or(FIRST)`,
    the `Option` collapsed at the boundary because the SQL wanted a value. A
    condition with no anchor stops meaning "nothing may match" and starts meaning
    "nothing after the first event may match", which is precisely the shape that
    lets a duplicate through.
  - **`positions_are_unique`** — `INSERT … VALUES (?1, …), (?1, …)`, which is
    correct while `?1` is `nextval()` and wrong the moment the position is
    precomputed in Rust and bound once for the whole statement. Every
    single-event append in the suite assigns one position and cannot see it; a
    two-event batch gives two events the same position, and every consistency
    boundary anchored on one of them then covers the other.
  - **`query_item_combines_types_and_tags_with_and`** — the clause vector
    `join(" OR ")`ed at both levels. Harmless whenever an item carries a single
    constraint, which is why it survives every other rule in the suite; an item
    naming a type *and* a tag then matches events carrying either.
  - **`query_item_tags_are_and`** — `WHERE (k=? AND v=?) OR (k=? AND v=?)` with
    no `GROUP BY … HAVING COUNT(*) = n`. The per-tag predicate is right and only
    the aggregation is missing, so a two-tag item matches an event carrying
    either tag and the consistency boundary widens to every entity sharing one.
  - **`query_item_rejects_partial_tag_overlap`** — the same missing aggregation
    seen from the other side, and a separate rule because an over-wide match and
    a missing exclusion are different assertions: an event holding one of an
    item's two tags must *not* come back, and under an `OR` it does.
  - **`query_item_tags_match_supersets`** — tags serialised into one canonical
    column and compared with `=`, which is what an adapter does to avoid a side
    table and a join. Every exact-match rule still passes; an event carrying an
    *extra* tag quietly stops matching a query that selects it.
  - **`query_item_types_are_or`** — an item's type list read as a conjunction,
    which matches nothing at all, and any tag join written as an `INNER JOIN`,
    since a type-only item is exactly the query whose events may have no tag row
    to join to.
  - **`query_matching_nothing_yields_empty`** — a query naming a type no event
    carries. Under interning the id lookup returns nothing, the empty clause is
    dropped rather than the query refused, and "no matches" comes back as *every
    event in the store*.
  - **`read_from_is_inclusive`** — `from` handled as an index rather than a
    threshold, for the same reason as above: `OFFSET` is already in the paging
    query. A projection resuming from its checkpoint then skips the event it
    checkpointed at, once per restart, with no error anywhere.
  - **`read_backwards_from_with_limit`** — three defects meet here, which is why
    the combination is its own rule: a direction that is a `bool` in one struct
    and a hard-coded `ASC` in the string another struct builds, an anchor bound
    to `OFFSET`, and the `LIMIT n + 1` every cursor-paging implementation fetches
    to answer "is there more" and this one forgot to trim.
  - **`read_backwards_limit_applies_after_filtering`** — the mirror of
    `read_limit_applies_after_filtering`, and separate because the direction and
    the limit are applied by different code: a store that filters in the
    application after `LIMIT n` returns fewer than *n* matches, sometimes none,
    while a caller reading "no more events" stops.
- **Three conformance rules, 27 → 30.**
  `two_fixture_instances_observe_none_of_each_others_appends` (two fixtures share
  nothing), `two_handles_observe_each_others_appends` (an append through one
  handle is visible to a read *and* to a tagged append condition through a
  second — which rejects a cached `max(position)` fast path, a per-connection
  repeatable-read snapshot and an advisory lock scoped to one pool member, all
  three of which passed every earlier rule), and
  `acknowledged_writes_survive_a_reopen`, gated on `REOPEN`.
- **`happenstance_testkit::fixtures::MemoryFixture`**, the reference `Fixture`
  and the worked example to copy. It is also the workspace's first fixture to
  *decline* a capability with a stated reason, which is what makes the skip
  machinery non-vacuous.
- **The proptest generators are public**, in `fixtures::strategies`, behind an
  off-by-default `proptest` feature, alongside a re-export of the `proptest` they
  speak — exporting a generator without the trait it returns is only half an
  export. The testkit always claimed an adapter pushing query matching into SQL
  should be able to reuse them; they were private functions in an
  integration-test binary, reachable by nobody. Off by default also means the
  testkit's own property tests only run under `--all-features`, which is what
  `cargo xtask ci` passes; a bare `cargo test -p happenstance-testkit` now runs
  none of them and says so only by their absence.
- **Two wrong implementations in the testkit's own `tests/`**, because a rule no
  implementation can fail is decorative. `CachedHeadFixture` refreshes its
  `max(position)` cache on its own appends and nothing else, and fails
  `two_handles_observe_each_others_appends` on the append side while reading
  correctly; `DurableFixture` rebuilds its store from a durable log on `reopen`
  and is the first fixture in the workspace to *support* `REOPEN`, so
  `acknowledged_writes_survive_a_reopen` executes rather than reporting a skip
  everywhere — with a losing sibling that acknowledges before committing to
  prove it can fail.

- **Nine conformance rules for the value edges, each with the wrong
  implementation it rejects.** Everything the rest of the suite exercises sits in
  the *middle* of a range — `append_preserves_event_payload` builds one event
  with a thirteen-byte payload and eight bytes of metadata, which are exactly the
  two values that make a lossy column mapping look total. The new rules sit on
  the boundaries. **The four minima are `[PROVISIONAL]` and phase 4 freezes
  them**, so those rules assert the clause's numbers rather than a constant,
  which does not exist yet.

  - **`append_preserves_an_empty_payload`** — one row-mapping helper written as
    `(!blob.is_empty()).then_some(blob)` and bound to both blob columns, because
    `metadata` genuinely is optional and `data` is not. Against a `data BLOB NOT
    NULL` column that is a constraint violation on a perfectly legal event;
    against a nullable one it is silent loss.
  - **`metadata_distinguishes_absent_from_empty`** — the nullable half of the
    same helper. `metadata: None` means *the writer attached none* and
    `Some(&[])` means *the writer attached a zero-length blob* — a codec that
    emits nothing for an empty struct produces the second — and a driver mapping
    a zero-length value to `NULL` collapses the pair. A consumer branching on
    `metadata.is_some()` then reads two different writers identically.
  - **`store_accepts_a_max_length_identifier`** — `VARCHAR(64)` chosen by
    eyeballing a domain whose longest event type is thirty characters. MySQL in
    non-strict mode truncates rather than erroring, so the write succeeds and the
    read returns a name that is nearly right; the same column at the byte/character
    boundary loses a multi-byte identifier that fits by specification.
  - **`store_accepts_non_ascii_identifiers`** — a `latin1` column, or a
    `VARCHAR` where the driver wanted `NVARCHAR`. The transcode is silent and
    every codepoint outside the charset becomes `?`, which turns Persian,
    Devanagari and an emoji ZWJ sequence into the same identifier.
  - **`store_accepts_the_guaranteed_minimum_payload`** — an undocumented row-size
    ceiling met at write time. VT-21 obliges a store to carry 65,536 bytes of
    payload; a 4 KiB page limit or an inline-blob threshold refuses an event the
    contract says is legal, and the caller has no way to know where the line is.
  - **`store_accepts_the_guaranteed_minimum_tag_count`** — tags joined into one
    `VARCHAR(255)` column to avoid a side table. VT-22's sixty-four tags overflow
    it, the overflow is dropped unreported, and the events that vanish from a
    query are exactly the ones carrying the most tags.
  - **`store_evaluates_a_query_at_the_guaranteed_minimum_item_count`** — one
    bound parameter per query item, chunked to fit the driver's limit, and the
    chunks never unioned. VT-23's 128-item query returns the first chunk's
    matches; a decision model built from it is missing whole entities.
  - **`store_accepts_the_guaranteed_minimum_batch_size`** — a multi-row `INSERT`
    meeting its parameter ceiling. VT-24's 128-event batch is refused *after* the
    caller has taken its side effects, which is the one point in a command where
    a failure cannot be retried cleanly.

- **`append_is_atomic_under_a_mid_batch_fault`, and a third fixture capability to
  make it possible.** ES-18's two existing rules both reach the write path
  through the append *condition*, where a conformant store decides before it
  writes anything — so the case where two rows land and the third fails was never
  reachable. It still is not reachable from outside: a decorator sits above
  `append`, which is the unit the port makes atomic. So `Fixture` grows
  `MID_BATCH_FAULT` and `arm_mid_batch_fault(after)`, both **defaulted to
  declined**, which means existing fixtures need no change and a store with no
  fault to inject reports an honest skip rather than a silent pass.

- **Five gate steps for the four clauses that named one and did not have one**,
  plus the disposed-rule check §7.4 scheduled. Each is a file read, each states
  in its own documentation what it does *not* verify, and each was demonstrated
  to reject something before being trusted — which is how the array-literal half
  of CF-6's found its own off-by-one, having silently matched nothing on the run
  that reported the tree clean.

  - `lint-clock` (CF-33) rejects `std::time`, `Instant`, `elapsed` and `sleep`
    in `happenstance-testkit/src`. It matches code only — comments removed,
    string contents blanked — because `concurrency.rs` explains at length why
    the watchdog it does not have is forbidden, and a lint that fired on that
    sentence would be repaired by deleting it.
  - `lint-position-literals` (CF-6) rejects an integer-list literal outside
    index position, and a `SequencePosition` built from a literal, across all
    three files rules live in. `GappedPositionStore` remains the enforcement;
    this is the cheap second line and says so.
  - `lint-changelog` (CF-29) requires every rule in all three files rules live
    in to be named in this file, in an entry carrying at least 120 characters of
    prose per rule it names. **Its first run found twenty-five of fifty-five
    rules with no entry at all** — see the founding-rules entry above, which
    exists because of it. Three more surfaced when it stopped reading `suite.rs`
    alone and stopped matching rule names as bare substrings: a rule that is a
    *prefix* of a longer one was being discharged by the longer one's entry, so
    `append_is_atomic` and `positions_are_unique` were reported satisfied while
    carrying nothing of their own.
  - `lint-testkit-version` (CF-32) fails if the testkit's `[package]` version is
    absent or inherited from the workspace.
  - `lint-retired-rules` (§7.4) fails when a rule named by a clause's `Retires:`
    line is still defined in any of the three. `spec-trace`'s check 6 accepts
    `claimed || retired`, so a disposition satisfies it forever — the phase-3
    retirements sat in that state for a phase and all three were wrong. It also
    parses a fixture clause of its own on every run, because there is no
    `Retires:` field left in the document to exercise the parse against: without
    that, renaming the field would leave the check printing green for ever.

- **A mutant registry, and the suite's proof that it discriminates**
  ([ADR-0010](.kb/decisions/0010-the-suite-must-prove-itself.md)). Fifty-two stores
  in `crates/happenstance-testkit/tests/mutation_coverage/`: fifty wrong
  implementations, each naming the real adapter shape that makes it plausible,
  and two *conformant variants* that must pass everything. Each is declared as
  data — the exact set of rules it fails, why it fails them, and where — and
  eight meta-tests hold the declaration to what actually happens, in both
  directions.
  Every rule now has at least one implementation that fails it; before this,
  none had ever been shown to reject anything, and a reviewer had measured four
  plausible wrong stores passing the suite. This is the artefact that makes a
  decorative rule *unwriteable*: adding one fails
  `mutation_coverage::every_rule_has_a_mutant` until its wrong implementation is
  named. **Adding a conformance rule now costs a rule, a mutant and a changelog
  entry** (CF-29) — see CONTRIBUTING.

  A mutant is held to failing *for the reason it claims*, not merely to failing
  (CF-2). A declared assertion failure must have been raised in `suite.rs` — so a
  store that falls over, or a fixture whose modelled defect has been deleted,
  is rejected rather than counted; a declared store panic must **not** have been.
  And each `(mutant, rule)` pair may pin the exact assertion message it trips, so
  a mutant that starts failing the right rule at the wrong assertion goes red
  instead of quietly ceasing to demonstrate what its provenance claims.
- **Two conformant controls**, which are the half that catches
  over-specification. `GappedPositionStore` assigns positions in steps of seven
  starting at 4,096 — legal for any adapter allocating from a sequence with
  `CACHE 7`, from a transaction id, or with a shard id in the low bits — and
  `PagedStreamStore`'s read stream is never ready on first poll, which is what
  any store with a network under it looks like. A rule either of them fails is a
  rule asserting more than the specification permits, and nothing in the
  workspace could catch that before.
- **`nothing_below_an_observed_position_appears_later`, 30 → 31 rules** (ES-10,
  CF-13). Once any reader has observed an event at position *P*, no later read
  may yield an event at a position ≤ *P* that was not already visible. **The
  defect it detects is what Postgres does by default**: `nextval()` allocates
  outside the transaction, so a writer takes a low number, does its work, and
  publishes its row after a later-numbered writer has already committed — at
  which point a caller conditioning on `after: 100` never saw 99,
  `is_violated_by` reports no violation, and the consistency boundary stops
  enforcing with no error anywhere. The rule creates two `append` futures from
  one handle and drives them by hand, one poll at a time, in the order A, B, B,
  A; there is no thread, no executor, no `Send` bound and no clock, so it is
  deterministic on every target the suite runs on. `PreCommitPositionStore` in
  the testkit's own `tests/` is the registered mutant that fails it. An adapter
  that allocates its positions under a lock it holds until commit — every
  adapter in the workspace — passes it on the first poll and has nothing to do.
- `cargo xtask proof-artefact`, a mandatory gate step that asserts the
  meta-tests are present — out of `cargo test -- --list` — before running them.
  Naming the test target catches its deletion; `cargo test` exits 0 on `running
  0 tests`, so it does not catch its emptying.
- **`event_store_model_conformance!`, a second and additive rule family.**
  Behind the testkit's off-by-default `proptest` feature. Its single rule is
  `ops_agree_with_the_model`, and the defect it detects is the one no *named*
  rule can be written for: an adapter that is right about every sequence anybody
  thought to enumerate and wrong about a sequence nobody did — a `from` bound
  that is an offset only after a batch of two, a condition probe correct except
  when the anchor is the head. It takes the same
  fixture and drives your store through generated sequences of appends,
  conditional appends and reads, checking each against a model of the log and
  *predicting* every conditional append's outcome before calling. Positions are
  never generated: the generator emits a **symbolic anchor** — first, middle,
  head, one beyond the head — resolved at execution against what your store
  actually assigned, so the same test runs unchanged against a store whose
  positions are dense from 1 and one whose positions step by seven from 4,096.
  It replaces no rule. Measured against the proof artefact
  (`the_model_rule_rejects_exactly_what_it_claims`, the seventh meta-test), it
  rejects thirty of the fifty mutants and neither conformant variant; the
  twenty it misses are three shapes — nine unreachable (empty batch, second
  handle, second fixture instance, durability, and the three whose defect is a
  window between overlapping futures), one that does not exist until a *fixture*
  is armed, and ten outside the generators' value range, which is what the named
  value-edge rules are for.
- **`event_store_concurrency_conformance!`, a third and additive rule family —
  opt-in, and with no timeout in it anywhere.** Eight contenders on real OS
  threads against one backing store, checking five things a sequential rule
  structurally cannot — each, as CF-29 asks, with the defect it detects:

  - **`exactly_one_of_n_contenders_commits`** — exactly one winner of eight
    handlers that decided from one snapshot. The defect is a probe followed by
    an insert with a window between them: `SELECT 1 FROM events WHERE …` and
    then `INSERT`, with no `BEGIN` between and no `SERIALIZABLE` under, which is
    what an adapter writes when its driver's convenience API is one statement
    per call. It answers correctly and acts on an answer that has gone stale,
    and with one caller at a time nothing exists that could make it stale — so
    every sequential rule in the suite passes it.
  - **`k_disjoint_boundaries_admit_exactly_k_commits`** — four *disjoint*
    consistency boundaries admitting exactly four commits. The defect is a
    conflict check coarser than the condition: optimistic concurrency control on
    one global version number, `UPDATE … WHERE version = ?`, or a `SERIALIZABLE`
    adapter mapping `40001 serialization_failure` onto
    `AppendError::ConditionViolated`. A sequential writer never overlaps anybody
    and so never serialisation-fails; under contention it reports conflicts
    between commands that share no query at all, which is the steady state of a
    busy store with many independent entities.
  - **`positions_are_unique_under_concurrent_appends`** — positions unique under
    concurrent appends. The defect is a head read across a suspension: the
    in-process form of `SELECT max(position) FROM events` issued *before* the
    transaction that will use it, so two writers that read the counter together
    assign the same rows. The sequential form does not exist — with one writer
    the read and the write-back are adjacent and the counter is never stale.
  - **`append_returns_the_callers_own_last_position`** — `append` returning the
    caller's own last position rather than the store's head. The defect is
    `INSERT …;` followed by `SELECT max(position) FROM events` as two statements
    with no transaction around them, which is what an adapter writes when its
    driver cannot give it `RETURNING` on a multi-row insert. Sequentially the two
    values are the same number. The caller checkpoints a projection at the wrong
    one, or builds the next `AppendCondition::after` from it, and silently skips
    every event another writer landed in between.
  - **`a_concurrent_reader_never_sees_a_partial_batch`** — a reader that never
    sees a batch part-written. The defect is a row-at-a-time insert loop with no
    transaction around it: `for event in batch { conn.execute(INSERT, …)? }`
    with the `BEGIN` forgotten. Every row lands, so the store is correct at rest
    and every sequential rule passes it; a reader looking *while* the loop runs
    sees a command that half-happened and can build a decision model from it.

  Three things to know before invoking it. **It is opt-in**, and its bound is
  `F::Store: EventStore + Send` — a `!Send` adapter such as a Durable Object
  cannot run it and is not expected to; the module records why the *`Send`
  flavour* of the port turns out not to be needed, which was a surprise.
  **It replaces nothing**: `racing_conditional_appends_elect_one_winner` stays,
  because it fixes the semantics this family then tests under contention.
  **There is deliberately no watchdog** — CF-33 forbids a conformance rule from
  reading a clock, and liveness rests on your CI job timeout instead, so a store
  that deadlocks hangs rather than naming a rule.

  Non-vacuity is measured rather than claimed. Six thread-safe stores — one
  correct control and five that are each wrong in one way no sequential rule in
  the suite can see, from `SELECT max(position)` before `BEGIN` to a global
  version check that reports contention between commands sharing no query — are
  driven through all five rules with every verdict pinned, in both directions,
  by an eighth meta-test.
- **Two naive-oracle property tests**, `query_matches_agrees_with_a_naive_definition`
  and `is_violated_by_agrees_with_a_naive_definition`. These are what make the
  model above non-circular, and they had to be written first: `Query::matches`
  was constrained only algebraically — every one of those laws is satisfied by a
  function returning `true` — and `AppendCondition::is_violated_by` had no
  property at all.
- **`fixtures::strategies` gained `any_query_item`, `any_query` and `any_event`.**
  Public for CF-21's reason: an adapter pushing query matching down into SQL is
  asserting the same laws about its `WHERE` clause, and generators reachable by
  nobody made that claim false.

- **Fifteen conformance rules closing the measured gaps, 31 → 46** (CF-7 – CF-12,
  ES-9, ES-14, ES-15, ES-20, ES-25, ES-27, ES-28, ES-36), each with the
  wrong implementation it rejects. Fourteen new mutants land with them, 27 → 41
  registered subjects. The two that matter most to an adapter author:
  **`condition_with_an_unheld_tag_does_not_reject`** rejects a probe that drops
  the tag join and matches on type alone — the canonical DCB uniqueness shape,
  which rejects *every* command touching any entity of that type while passing
  every other rule, and which nothing could see before because every condition in
  the suite was built from types; and
  **`read_limit_applies_after_filtering`** with its backwards mirror rejects
  `SELECT … LIMIT n` with the query's predicate applied to the rows that come
  back, which a reviewer measured passing the suite as it stood.

  The rest, each with the defect it detects, because a bare rule name gives an
  adapter author whose CI has gone red no way to tell "my adapter has a defect"
  from "the suite changed" (CF-29):

  - **`condition_matches_on_tags`** — a condition probe that compares serialised
    tag sets with `=` instead of matching supersets, so a stored event carrying
    an *extra* tag stops violating a condition it does violate.
  - **`duplicate_items_do_not_duplicate_events`** — a tag side-table join without
    `DISTINCT`: an event carrying three tags comes back three times from
    `Query::all()`, the query every projection runner starts from.
  - **`untagged_events_match_query_all`** — the same join written as an `INNER
    JOIN`: an event with no tags has no row to join to and disappears entirely.
    Silent data loss.
  - **`query_item_order_does_not_change_the_result_set`** — one statement per
    query item, unioned client-side and never merge-sorted, so the result order
    is the item order rather than position order.
  - **`reading_an_empty_store_yields_nothing`** — a paging window anchored on
    `max(position)` decoded into a column type that cannot hold `NULL`. The
    adapter fails on a store whose only fault is being new.
  - **`read_from_composes_with_multi_item_query`** — `WHERE a OR b AND position
    >= ?` without parentheses. The cursor binds to the last item alone, so a
    resuming projection re-delivers already-checkpointed events forever with no
    error anywhere.
  - **`empty_batch_is_refused_before_the_condition_is_evaluated`** — an empty
    batch answered by the condition check, so `append(&[], Some(&c))` reports
    `ConditionViolated` — "retry" — for what is unambiguously the caller's own
    bug. This was the reference implementation's own defect; see *Fixed* below.
  - **`condition_against_an_empty_store_admits_the_append`** — an `EXISTS` probe
    whose SQL returns a `NULL` the code reads as true on an empty table, so every
    adapter's very first conditional append is rejected.
  - **`condition_after_beyond_head_admits_the_append`** — an adapter that
    validates an incoming `after` against its own head and errors on a position
    it has not assigned. Fatal to a peer resuming after a gap.
  - **`condition_after_beyond_the_last_matching_position_admits_the_append`** —
    a probe that ANDs two uncorrelated predicates, *does any event match* and *is
    the head above `after`*, which is what the check becomes when the existence
    test and the position test are written as separate subqueries. It rejects
    every command whose caller read past the last event touching its own entity:
    the steady state of a quiet entity in a busy store, reported as contention.
  - **`interleaved_appends_on_one_handle_elect_one_winner`** — an adapter that
    drops its borrow around an `.await` and leaves a window between its condition
    probe and its write, so two writers each see a store missing the other's row
    and both commit. For a `RefCell` store the same shape panics; for a pooled
    SQL adapter holding one connection it deadlocks.
  - **`a_live_read_stream_does_not_block_an_append`** — a read stream that keeps
    a borrow of the store alive while the caller holds it: rusqlite yielding rows
    from an open statement, an `Rc`-shared cursor, a pooled adapter that checks a
    connection out in `read` and returns it in `Drop`.
- **The re-entrancy pair is registered.** `BorrowHoldingStore` — a `read` stream
  that keeps a borrow of the store alive — and `AwaitAcrossBorrowStore` — an
  `append` that holds an exclusive one across an `.await` — were the two wrong
  stores CF-2 rejects the *shape* of: each was driven by a hand-written
  `#[should_panic]` recording only that something failed. They could not move
  into the registry until the rules they fail existed, and they moved in the same
  change as those rules. `tests/local_conformance.rs`'s `mutants` and
  `reentrancy` modules are gone. **One asymmetry is worth knowing before you
  write an adapter:** for a `RefCell` store this defect is a panic; for a pooled
  SQL adapter holding one connection it is a **deadlock**, and a hung conformance
  run names no rule at all.

- **Nine conformance rules for read options, the query algebra and read
  isolation, 55 → 64.** Phase 4 is the phase that could write them: three needed
  `ReadOptions::to`, which did not exist; one needed `limit` to be
  `Option<usize>`, because a zero was discarded by the builder before any adapter
  saw it; and two needed nothing but the observation that a rule can supply its
  own pause point by polling a stream once, which is what ADR-0011 corrected. Each
  ships with the wrong implementation it rejects, and
  [ADR-0011](.kb/decisions/0011-read-laziness-and-isolation.md) is the decision behind
  all nine.

  - **`read_to_is_inclusive`** — an adapter that accepts `ReadOptions` by value,
    matches on the fields it recognises and ignores the rest. It is the shape
    every `#[non_exhaustive]` options struct invites: the code compiles unchanged
    when a field is added, and a backfill worker given the closed window [1, *H*]
    reads to the end of the log instead. There is no error anywhere — the tail
    worker beside it simply processes the overlap a second time. The second
    implementation it rejects reads the bound as exclusive, which costs one event
    at every chunk boundary and is invisible until the chunks are reassembled.

  - **`read_from_and_to_bound_a_closed_window`** — the two bounds applied to
    different halves of the same statement. A store that honours the upper bound
    and treats the lower one as an `OFFSET` — the parameter already sitting in its
    paging query — slides the whole window down by the lower bound's numeric
    value, which is a different window entirely on any store whose positions do
    not start at one. The result is the right *number* of events and the wrong
    ones, which is the failure mode nobody notices in a smoke test.

  - **`read_to_under_backwards_bounds_the_older_end`** — `WHERE position <= ?`
    copied verbatim into the descending branch, so the upper bound never swaps
    ends. It is *correct* reading forwards, which is what makes it survivable:
    every forward rule passes and a backwards read comes back with the oldest
    events instead of the newest. This is its own rule rather than a third
    assertion elsewhere precisely because one registered adapter fails it and
    nothing else in the suite sees that adapter at all.

  - **`read_limit_zero_yields_nothing`** — a budget of zero read as no budget,
    which is the DCB reference implementation's `if (limit)` falsiness and,
    equivalently, `happenstance-core`'s own pre-phase-4 `NonZeroUsize::new(limit)`.
    The caller it breaks is the one who computed the zero: a paging loop writing
    `.limit(budget - fetched)` that reaches parity does not read nothing, it reads
    **the entire log, unbounded, silently**, and the memory ceiling it was
    protecting stops being a ceiling with no error and no failing test. The second
    implementation it rejects is the `LIMIT n + 1` cursor probe, which hands back
    one event when it was asked for none.

  - **`limit_applies_across_items_not_per_item`** — a store that cannot express a
    disjunction in one statement, emits one per query item, and writes the row
    budget onto each of them, because that is where the paging clause lives. The
    union is then merge-sorted perfectly well, so every ordering rule still
    passes, and a caller who asked for four events is handed a page it did not
    size — while its own paging arithmetic goes on being computed from the number
    it asked for.

  - **`read_from_a_gap_position`** — an anchor implemented as an equality seek or
    a `rowid` offset rather than as a range scan, which is plausible wherever
    positions came from a dense counter and the author assumed density. The
    specification permits gaps, so a projection resuming at a position nothing
    occupies must be given the next event past it; a seek returns empty and the
    projection stalls forever with no error anywhere. The rule anchors on the one
    unoccupied position every store has — the one above its head — and reads
    backwards from it, which is a case nothing in the suite reached before.

  - **`query_union_is_item_concatenation`** — a store that interns a query's items
    **by their type list**, so a second item carrying the same types is dropped
    and its tag constraint goes with it. `QueryItem::new` already sorts and
    deduplicates *types*, so extending the idea one level up looks like the same
    move; the query then selects a strictly smaller set than the caller asked for.
    No order-invariance rule can catch it — deduplicating the items is precisely
    what makes their order stop mattering — so only a match-set claim over an item
    whose presence changes the set rejects it.

  - **`read_result_is_stable_under_concurrent_append`** — a store with no cursor
    issuing an independent statement per page against whatever it holds *now*,
    which is the natural shape for a transport that answers one buffered document
    per round trip. The result grows under the caller's feet, and the damage is
    not the extra event: the caller derives its append condition's boundary from
    the maximum position the read observed, that maximum sits above an event the
    read never showed it, and the condition then instructs the store to ignore
    exactly what was missed. A torn read does not become a rejected append; it
    becomes an accepted one.

  - **`query_items_share_one_snapshot`** — the same tear one level finer: an
    adapter emitting one SQL statement per query item, so an event matching the
    first item that lands between two statements is silently absent while the
    observed maximum position sits above it. It needs no visibility hole and no
    sequence trick, only two statements, and it is the natural shape for a
    one-round-trip peer. The rule is portable because the pause point is the poll
    boundary the rule itself controls rather than a fixture that can be halted
    between round trips — which is the correction ADR-0011 makes to the
    specification's own account of why this was blocked.

- **Seven wrong implementations behind those rules**, in the testkit's mutation
  registry, because a rule no implementation can fail is decorative:
  `ToBoundIgnoredStore`, `ToIsExclusiveStore`, `BackwardsToIsAnUpperBoundStore`,
  `LimitZeroIsUnlimitedStore`, `LimitPerItemStore`, `ItemDedupByTypeStore` and
  `RefetchingPagedStore`. The last of those is the self-paginating adapter
  ES-11 has always named and nothing had compiled: it is the *recommended*
  pagination with the position ceiling missing, which is one line rather than a
  redesign, and is why phase 4 promotes that ceiling from advice in a
  justification paragraph to a MUST.

- **Nine conformance rules for the facts a store assigns rather than the caller.**
  `SequencedEvent` grew an `EventId` and a `RecordedAt` this phase, and a field
  nothing can be wrong about is a field no rule needs. These nine are what makes
  them checkable, together with seven new mutants in the testkit's own registry.
  Two of the nine are gated on `Capability::REOPEN` and report a skip against any
  fixture with no durable medium behind it.

  - `append_stamps_identity_and_time` catches the adapter that **makes an event's
    identity or time up at read time instead of persisting it at write time** — a
    row mapper that synthesises an `EventId` from the row's ordinal in the result
    set, or fills `recorded_at` from the connection's clock, because the column was
    never added. Such a store agrees with itself perfectly under any single query,
    which is why the rule reaches one event twice, once through `Query::all()` and
    once through a query that selects it alone, and compares the whole
    `SequencedEvent`. It is the defect that passes everything until the first
    reopen, and then loses an audit trail rather than an assertion.

- `append_stamps_a_local_event_id` catches two stores that disagree with the
    contract about where a locally appended event's identity comes from. One
    computes it from something other than the position it assigned — a content
    hash, or a `RETURNING` value read once per statement — so `id.position()` and
    `position` part company for an event that never left the store it was written
    to. The other mints a fresh `StoreId` for every **append** rather than for
    every open: it never reissues a pair, so it satisfies the letter of the
    incarnation rule, and it makes every event its own origin, which is what a
    peer's watermark and the replication sort both degenerate under.

- `event_ids_are_unique_within_a_store` catches the store that lets one
    `EventId` name two events, which the specification makes the store's
    obligation and not the caller's. The realistic shape is a multi-row `INSERT …
    RETURNING` whose returned identity is read once and bound to every row of the
    batch — the positions stay correct, so both position rules pass and nothing
    else in the suite notices, and the store is left holding two events it has no
    way to tell apart. An ingest path that deduplicates on that identity then drops
    a real fact with no error and no symptom.

- `appending_equal_events_yields_two_events` catches a **content-hash identity
    scheme, and any store that deduplicates on payload equality**. Appending two
    structurally equal events in one batch must produce two events, at two
    positions, with two identities; a store that derives identity from the bytes
    collapses them into one. The failure has no error path: a refrigeration
    engineer consuming two of the same part on one work order writes two
    byte-identical events, the second disappears, and the van's stock balance is
    permanently one unit high with nothing reporting it.

- `event_id_is_not_matchable_by_query` catches the **tag-materialised identity**:
    a store that writes an extra row into its tag side table so that membership can
    be answered out of the index it already has. The tempting part is that the tag
    is the adapter's rather than the caller's, so events still round-trip
    byte-for-byte and every payload-fidelity rule keeps passing. What it costs is
    structural — identity is a point lookup on a unique key and a query item is a
    set-superset predicate, so grafting one onto the other gives the item a third
    semantic and breaks the union identity the fan-out runner depends on.

- `reopened_store_does_not_reissue_an_event_id` catches **a store that keeps its
    incarnation across a reopen and restarts its position counter**, which is what
    a restored backup looks like from the inside: genuinely new events are minted
    with pairs the store has already issued to different ones, every peer's
    deduplication treats them as already-seen, and real facts are silently dropped
    — the one failure mode in the replication design with no error path and no
    observable symptom. It replaces the rule the specification originally named,
    which would have failed the perfectly legal adapter that mints a fresh
    incarnation on every open. Gated on `Capability::REOPEN`.

- `append_stamps_a_recorded_time` catches the store whose recorded time is a
    property of the **read** rather than of the append — a row mapper filling the
    field from the connection's clock because the column was added to the port
    after the schema was written. It asserts presence and stability and nothing
    else: comparing two events' times, comparing a time against a position, or
    checking one against the harness's own clock are all forbidden, because the
    contract states no relationship between recorded-time order and position order
    and a rule that asserted one would state it on the contract's behalf.

- `recorded_time_survives_a_reopen` catches the store that **restamps on
    replay**: one that rebuilds its log from a durable medium carrying the events
    but not the times it recorded them at, so every auditor is handed the time of
    the last restart instead. The one clock reading whose provenance the log itself
    attests is then gone, with no error and no symptom, and the question the field
    exists to answer — which side of midnight did this land — cannot be asked. It
    asserts only that the same event's own value is unchanged. Gated on
    `Capability::REOPEN`.

- `contains_event_id_reports_membership` catches the store that answers a
    membership question **by position alone, ignoring which store minted the
    identity**. `SELECT 1 FROM events WHERE position = ?` is what an adapter writes
    when its table has a position column and no origin columns yet, which is every
    adapter before it implements ingest; it passes every single-store rule in the
    suite, because a store that has ingested nothing only ever holds its own
    incarnation. Against a peer it reports a foreign event as already present
    whenever the local log happens to be at least that long, and the batch carrying
    it is dropped.

- **`batch_positions_follow_slice_order`** — a conformance rule for ES-19's
  second sentence, which nothing checked. `append_returns_last_written_position`
  asks *which position came back* and never *which event got it*, so a store that
  writes a batch backwards and returns the maximum satisfies it on any quiescent
  store. The defect it catches is a multi-row `INSERT` assembled by draining a
  stack, or one that groups a batch by event type to bind one interned type id per
  group and does not notice that grouping is reordering: the batch reads back in
  the wrong order, so a decision that appended `Held` then `Released` replays as
  `Released` then `Held` and the projection is wrong with no error anywhere.

- **`batch_is_not_evaluated_against_its_own_condition`** — a batch can never
  conflict with itself (ES-21), and until now the reference store answered that
  correctly only by accident of implementation order. The defect it catches is the
  per-row conditional `INSERT … SELECT … WHERE NOT EXISTS`, which is a live
  candidate for the append-condition SQL strategy and the only shape a store with
  no interactive transaction can express: carried per row, the guard travels with
  every statement, so the second row of a batch is checked against a store that
  already holds the first. On the canonical DCB uniqueness shape — where the
  condition names the very type being written — such an adapter refuses **every**
  conditional append and passes every other rule in the suite.

- **`dropped_append_future_leaves_no_partial_batch`** — ES-22, and the half of
  cancellation a conformance rule can see. In Rust, cancelling is dropping the
  future, and at the edge that is the *normal* termination path: a client
  disconnect, a CPU limit, a Durable Object eviction, a pod eviction. The defect it
  catches is a batch executed as one statement per row with an `.await` between
  them and no transaction around them — a drop after the first poll leaves rows in
  the log that no caller was ever told about and that no `Result` exists to report,
  because a dropped future produces none. `MemoryEventStore` passes it trivially,
  which is exactly why the reference store cannot answer this question.

- **`arming_a_mid_batch_fault_makes_the_append_fail`** — the rule that makes
  `append_is_atomic_under_a_mid_batch_fault` non-vacuous, and a new specification
  clause (CF-39) behind it. A fixture whose `arm_mid_batch_fault` has an empty body
  passed the atomicity rule for free: no fault, `Ok`, every row present,
  all-or-nothing satisfied — a capability declared, nothing contributed, and a
  green atomicity result for a store that has never been faulted. A fixture
  declaring the capability must now arm a fault its store cannot absorb, so the
  append returns `Err`; a store that can absorb every fault it is able to arm must
  decline the capability and say so. A *forgotten* override was never the hazard:
  the trait's provided body panics and names this mistake.

- **`reissued_conditional_batch_lands_once`** — ES-24's guarantee, made
  checkable. A conditional append whose condition matches its own events is
  at-most-once under verbatim reissue, which is how a caller resolves the outcome
  of a dropped future with no identity, no idempotency key and no new API. The
  defect it catches is a store that writes before it decides: autocommit plus a
  separate probe puts the batch into the set its own condition reads, so the
  **first** attempt is refused while its rows stay down — and a caller following
  the documented resolution procedure reads that refusal as "my write already
  landed", stops, and has written nothing at all.

- **`reissued_unconditional_batch_lands_twice`** — a rule that pins a
  *non*-guarantee, which is unusual enough to say why. ES-24 states that an
  unconditional append has no at-most-once property, and a guarantee whose limits
  are unstated is read as universal: callers write retry loops against the adapter
  they happened to test on. The defect it catches is a content-addressed store,
  or one built to be safe under at-least-once ingest, that hashes
  `(event_type, tags, data)` and quietly refuses a duplicate — it would ship as a
  feature, and it makes the same retry loop double-charge on the next adapter and
  not on this one, with nothing in either CI to say so.

- **`reissued_batch_conditioned_on_other_events_lands_twice`** — ES-24's second
  stated limit, which the clause has carried in prose and given no rule since it
  was written, and which is the *common* shape rather than an exotic corner. A
  decision that reads one thing and writes another — conditioning on
  `CourseCapacityChanged` while appending `StudentSubscribed` — leaves a retry
  indistinguishable from a first attempt, because nothing the retry wrote is in
  the set its own condition looks at. Without it a caller reads the guarantee
  above as "conditional appends are idempotent"; they are not, and the difference
  is one line in the caller's decision model.

- **`append_reports_exceeded_store_limits`** — VT-25's variant, made checkable
  by a fixture that states its store's ceilings (a new clause, CF-40). The
  distinction it protects is a sync runner's: *this event will never fit here,
  park it and tell a human* against *the disk is full, retry*. A runner that
  cannot tell them apart guesses, and a runner that guesses wrong drops an event
  permanently. Four defects are caught, in two pairs of one column configured two
  ways: a payload ceiling and a driver parameter ceiling reported through
  `AppendError::Store` — which is what every adapter does today, because until now
  there was nowhere else to put it — and the same two limits met by storing what
  fits, answering `Ok` with a real position, and telling the caller the whole
  write landed.

- **`condition_guards_carry_independent_boundaries`** — VT-30's rule, and what
  makes VT-27's frozen refusal sound. An application whose decision model is
  assembled from fragments read separately gets one boundary per fragment and is
  told to issue one read per fragment; that is only sound if the boundaries can be
  carried into one condition. The defect it catches is the application-side
  workaround promoted into an adapter: a condition collapsed to `min(p₁…p₄)`. It
  never admits an append it should have refused, so it is *sound* and it is a
  liveness failure — the quiet fragment's stale boundary governs the busy one, a
  consistency boundary that never conflicted starts refusing, and the deployment
  reads the rejection rate as contention. It passes the entire existing
  `condition_after_*` family, because every rule in that family carries one guard.

- **`condition_with_one_guard_behaves_as_today`** — VT-30's compatibility half,
  and the regression the guard refactor invites. The defect it catches is a store
  with two code paths that disagree: an adapter generalising to N guards keeps a
  fast path for one, because one guard is the overwhelmingly common case and a
  `UNION` per guard is pure overhead there — and that fast path is the *old*
  statement, the uniqueness probe written before boundaries existed, kept because
  it was already working while the general path was the one that got reviewed. The
  result is a store whose answer depends on how many fragments the caller's
  decision model happened to read. Its coverage overlaps the single-guard family by
  construction, and `spec/SPECIFICATION.md` VT-30 records that.

- **Seven wrong implementations** in `happenstance-testkit`'s own `tests/`, one
  per new rule and none of them a saboteur: `ReverseOrderBatchStore` (a bulk
  insert that reorders the batch), `PerRowConditionStore` (the guard carried per
  row, rolling back so that it fails ES-21's rule rather than the atomicity one),
  `YieldingRowAtATimeStore` (a suspension point between two rows),
  `PayloadDedupStore` (`ON CONFLICT (content_hash) DO NOTHING`),
  `MinCollapseStore` and `SingleGuardFastPathStore` (the two mirror-image ways a
  guard fold goes wrong), and `NoopFaultFixture` (a declared fault capability whose
  arm does nothing).

- **Three rules for `head()`, 55 → 58 rules** (ES-30, ES-33). `head` became a
  required method of the port at phase 4 and nothing checked it; a required
  method no rule exercises is a signature, not a contract.

  - **`head_of_an_empty_store_is_none`** — a store that holds nothing reports a
    position anyway. `MAX(position)` over an empty table is `NULL`, the driver's
    scalar decode wants a column type that can hold what comes back,
    `IFNULL(…, 0)` is the one-token fix, and `SequencePosition` is a `NonZero`
    newtype whose constructor returns an `Option` that library code may not
    `unwrap` — so `unwrap_or(SequencePosition::FIRST)` is the shortest spelling
    that satisfies every local rule and reports position 1 on a store with no
    events in it. ES-11 prescribes anchoring a paginating read on `head()` at the
    first poll and ES-31 makes "am I caught up?" a comparison against it, so a
    runner starting against a fresh store checkpoints past an event that does not
    exist and the first event ever appended is the one it skips.

  - **`head_is_the_highest_visible_position`** — a head that covers the part of
    the log some default query matches rather than the store. In a two-table tag
    schema there is one joined view, the read path is built around it, and
    reusing it for `SELECT max(position)` is the obvious move; an event carrying
    no tags has no row on the other side of the `INNER JOIN`, so the store
    under-reports its head to every projection runner and every paginating caller
    and the events past the under-reported head are exactly the ones nothing else
    in the deployment tags. The rule asserts a **bound** rather than an equality
    against what `append` returned, because an adapter buying ES-10's visibility
    invariant with `xid8` + `pg_snapshot_xmin` reports a frontier and does not
    satisfy read-your-own-writes — ADR-0013 §4 has the measurement and the
    argument.

  - **`head_advances_across_two_handles`** — a handle that answers `head()` from
    the position its own last `append` returned. A field, a session variable, or
    Postgres' `currval()`, which is session-scoped by documentation and is
    therefore the version of this defect the database hands you ready-made. It is
    exactly right against a single handle, which is every test anybody writes
    before they have a connection pool, and stale the moment one store is reached
    two ways — which is every deployment with a pool. It is the second of the two
    places one cached head gets spent: `CachedHeadFixture` spends it on the
    condition probe, `LastWrittenHeadStore` on the head itself, and the two are
    separate registry rows because a mutant with two defects evidences neither.

- **Three rules for the identifier edges, 58 → 61 rules** (VT-1, VT-15, VT-17).
  Every one of them writes an identifier the rest of the suite does not, because
  the middle of a range is what every other rule exercises and a lossy mapping
  only looks total there.

  - **`tags_differing_only_by_unicode_normalisation_are_distinct`** — a tag
    column under a normalising or case-insensitive collation.
    `CREATE COLLATION … (provider = icu, deterministic = false)` is one line and
    is what somebody reaches for when a search stops matching an accented word;
    a normaliser called in the row mapper "because tags should be canonical" is
    the same defect written by hand. `"café"` in NFC and in NFD render
    identically in every console, so a macOS client and a Linux client writing
    the "same" tag silently stop conflicting — two consistency boundaries where
    the application intended one, with no visible cue at all — and the tag read
    back is no longer the tag written, which breaks byte-faithful replication.

  - **`tags_may_repeat_a_key`** — a tag index shaped as a key-to-value map. A
    `JSONB` object, a `HashMap<String, String>` column, or a side table under
    `UNIQUE (event_id, key)` written with `ON CONFLICT DO UPDATE`: all three are
    natural schemas for something the contract itself invites you to read as a
    pair, and all three keep one value per key.
    `Tags::from_pairs([("tenant", "a"), ("tenant", "b")])` is a **two**-element
    set, because deduplication is on the whole `key:value` string. The event
    stays in the store and stops matching one of the two queries that should
    select it, so on a 4,200-tenant shared log the tenant whose tag was dropped
    stops seeing its own events and nothing anywhere reports a fault.

  - **`append_preserves_event_type_and_tags_byte_for_byte`** — an adapter that
    trims an identifier, because a trailing space "must be a typo". `TRIM()` in
    the insert or `value.trim()` in the row mapper moves the decision about what
    an identifier *is* out of the application and into the store, and the value
    the caller wrote is then no longer in the log, so nothing downstream can
    detect what happened. Kestrel Rotor replicated `turbine:HW2-A14 ` with a
    trailing space; `Tag::new` accepts it, `contains_all` is a strict merge-scan
    on equality that does not match the unpadded tag, and a lot-recall query
    silently missed a turbine. `append_preserves_event_payload` cannot see any of
    this: the identifiers it writes are `"A"` and `"course:c1"`, which are fixed
    points of every transformation an adapter might apply.

- **Six mutants, and the eighth `Defect` step that two of them needed.**
  `EmptyHeadIsFirstStore`, `DefaultQueryHeadStore`, `NormalisingTagStore`,
  `KeyedTagMapStore`, `TrimmingIdentifierStore` and `LastWrittenHeadStore` are
  the compiled wrong implementations for the six rules above, one rule each.
  `Defect` gained `head_of`, so a mutant of `head` is one step from correct in
  the same way a mutant of the read path is; `correct::head_of` had been written
  as a free function against exactly that possibility and says so.
  `contains_event_id` deliberately did **not** become a step: no rule calls it,
  so a defect there would be a claim nothing evaluates.

- **`happenstance-testkit`'s crate page now tells an outside author how to
  conform, in six steps.** *Writing a projection adapter from outside this
  workspace* names the two dependencies and which section each belongs in, says
  where the `ProjectionProbe` impl must live and what the orphan rule answers if
  you put it in `tests/`, and says why the expansion reaches the fixture trait
  through a hidden `__private` module you never name. **No item became `pub` and
  no rule changed**, so this is not a MINOR event on the bar this crate's version
  is a promise about — the surface was already complete, and what was missing was
  the page that says so.

- **`examples/outside-projection-adapter/` — the page's falsifier, kept in the
  tree.** A `publish = false` workspace member implementing `ProjectionStore` and
  `ProjectionProbe` from the rendered documentation alone, passing every rule in
  `for_each_projection_store_rule!` — one of them as a declared skip carrying its
  own stated reason — with a checkpoint-only sibling beside it that
  `commit_is_atomic_with_the_read_model` rejects by name. It is the only crate in
  the workspace where the orphan rule and the non-dev dependency graph behave as
  they do for a stranger, which is what makes it able to fail the wrong version
  of that placement decision: `cargo tree --edges normal` over it reaches
  `happenstance-core` and nothing else.

- **A gate step that reads the documents a consumer reads, and fails when they
  state a rule count this workspace does not have.** `cargo xtask
  lint-rule-counts`, mandatory in `cargo xtask ci` and in the story-grain
  `affected` gate. Every other step in the gate holds code to a document; this
  one runs the other way, because the failure it catches had already happened
  four times in three file formats at once — the crates.io README, the crate
  page's feature list, the manifest comment beside the feature, and the
  changelog entry for the outside-author example all said the projection suite
  was two rules of seventeen, or sixteen, through the commits that made it
  seventeen. `cargo package --list` proves the README is *inside* the artifact
  (D11); nothing proved it was *true*, and three consecutive audits raised it as
  a finding rather than a red build.

  It reads a cardinal — digits or words, `eighty-nine` included — qualifying
  `rule` or `rules` inside a paragraph about the suite, and compares it against
  the rule files themselves. `one test per rule` is a rate and is skipped;
  `CHANGELOG.md` is excluded, because a released entry that was true when it was
  written must not be rewritten to keep a check green. The counts come from the
  same parse CF-29 and `spec-trace` use, so landing a rule moves the bar with no
  edit here.

### Changed

- **The gate now holds the projection family's proof-artefact names, and harness
  parity is enforced by a test rather than by review.** `cargo xtask ci`'s *each
  phase's proof artefacts* step asserts the projection mutant registry's
  meta-tests and the new harness-parity target out of `cargo test -- --list`
  **before** running them, so renaming, `#[ignore]`-ing or emptying one of them
  fails the gate by name instead of exiting 0 with nothing to say. Each
  `happenstance-testkit` row now prints **its own** registry's row count; it used
  to select the count by package, so a second row in that package would have
  printed the event-store registry's number beside the projection target.

  The parity guard is `crates/happenstance-testkit/tests/projection_harness_parity.rs`.
  It takes the rule names from `for_each_projection_store_rule!` and asserts that
  no harness source contains one as an identifier and that each of the three
  carries exactly one `projection_store_conformance!` invocation. A harness that
  listed rules by hand type-checks exactly as well as a generated one, so the
  mandatory `wasm32` check could not have seen it and a reviewed `rg` could not
  have failed twice.

- **`unstable-projection` exists, and the promise at the top of this file is now
  true.** `ProjectionStore`, `ProjectionId`, `Checkpoint`, `Authority`,
  `CommitError`, `ResetError`, `SendProjectionStore`, `ProjectionProbe` and
  `MemoryProjectionStore` are behind an off-by-default feature on
  `happenstance-core`; `cargo add happenstance-core` no longer hands you the
  projection port, and a build that names one of those items without the flag
  fails to compile instead. `conformance` implies it, so an adapter author running
  the projection suite still writes one flag. `memory` does **not** — the memory
  projection store needs both, because it implements the port the gate is on.
  Nothing else in the crate changed shape, and opting back out is deleting the
  flag.

  **The reason is not that nothing tests it.** All seventeen conformance rules
  §4.11 assigns to an adapter's own suite are written and drive the port, a store
  that writes a checkpoint without its read model fails one by name, and two
  structurally unlike batch shapes pass all of them. The reason is the bar §4's PS-2 sets for *freezing* it
  — two adapters at opposite ends of the batch-shape axis — and both shapes that
  clear the suite today are instruments this workspace wrote. What retires the
  exemption is that same suite green against a projection adapter over storage
  this workspace does not control. The module header states it where the compiler
  error sends you.

- **Every `PS` clause's rule citation is now checked, and §4's clause count moved
  by one.** `cargo xtask spec-trace` used to abstain on the whole `PS` family
  because the projection suite did not exist; it does, so the exclusion went, and
  ten citations that had never been resolved by anything were dispositioned —
  three were parser artefacts naming a doctest annotation, a probe method and a
  projection callback rather than rules, and seven are rules that genuinely are
  not written yet and now say so. **PS-38** is new
  ([ADR-0030](references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md)):
  a successful `commit` MUST advance the checkpoint, and an id no commit has named
  MUST read as `NeverRun`. Four rules already enforced that and no clause stated
  it. No `[FROZEN]` sentence was edited — PS-1, PS-19, PS-21 and PS-22 are
  byte-identical, and each gained a recorded finding instead.

- **`EventStore::append`'s documentation no longer offers the returned position
  as a follow-up `AppendCondition::after`.** It never was one: positions may be
  gapped, and a second writer may hold a position below the one you were handed
  and never showed you, so a condition built from it asserts something the caller
  has not read. The sound `after` comes from a read — `read_decision_model`, or
  `AppendCondition::after_opt` over what that read returned. ES-19 has forbidden
  this since it was frozen; the doc comment was recommending it anyway. If you
  followed the old sentence, the condition it produced was weaker than it looked.
- **`Event::into_parts` is documented as what it is.** Its summary claimed it
  avoided "a clone in adapter write paths", which no adapter can do: `append`
  takes `&[Event]`, so no store implementation ever owns an `Event` and none can
  reach the method. It is for callers and for the typed layer's wire encoders,
  and an owning adapter clones — cheaply, since the payload is a `Bytes` refcount
  bump.

- **`event_store_conformance!` takes `fixture =`, not `factory =`, and the
  expression must now build a `Fixture` rather than a store.** There is no
  deprecated arm: nothing in this workspace is published yet, and this is the
  last release in which that is true.
- **A rule's signature changed with it, which matters to anyone who wrote their
  own emitter** — CF-23 makes that a documented extension point, so this is a
  break, not an internal detail. A rule is now
  `<F: Fixture>(open: impl AsyncFn() -> F) -> RuleOutcome`; the hoisted item is
  `async fn __conformance_fixture()`, not `fn __conformance_store()`; and the
  emitter must report what comes back — `.report(::core::stringify!($name))`, or
  `.skip_line(…)` on a target with no stdout. `RuleOutcome` is `#[must_use]`, so
  an emitter that drops it warns, which is an error in any workspace that denies
  warnings. The worked emitter is in
  `crates/happenstance-testkit/tests/local_conformance.rs`, and the contract is
  written out at the top of `crates/happenstance-testkit/src/registry.rs`.
- **The contract crate is now `happenstance-core`; `happenstance` is the typed
  layer** ([ADR-0006](.kb/decisions/0006-bare-name-to-the-typed-layer.md)). Today
  `happenstance` re-exports the contract unchanged, so it is published from the
  start and `cargo add happenstance` is true throughout. Adapter authors should
  depend on `happenstance-core`.
- `clippy::todo` and `clippy::unwrap_used` are denied workspace-wide rather than
  allowed and warned. All six skeleton crates carry a scoped `#![allow]` naming
  the phase that removes it.
- `tokio` is a real workspace dependency rather than a dev-only one:
  `happenstance-sqlite` names `JoinError` and `JoinHandle` in its public error
  type and its read stream, and `happenstance-postgres` reaches it through
  `sqlx`'s `runtime-tokio`. None of that is in the published graph, all three
  publishable crates are unaffected, but the manifest had said otherwise.
- **`cargo xtask ci`'s `tests` step now passes `--show-output`, and this is
  user-visible in every adapter's CI too.** libtest suppresses a *passing*
  test's stdout, so CF-18's `SKIP <rule>: <reason>` lines — the whole
  capability-skip reporting machinery, and the thing that tells an adapter
  author a rule did not run rather than passed — were being written into a void.
  What it detects: a suite that reports green while silently skipping rules.
- The optional wasm32 feature-powerset step and the nightly `--cfg docsrs`
  rustdoc step now both name `happenstance-testkit`. The powerset one compiles
  the `proptest`-feature-on-wasm32 combination that `fixtures::strategies`'
  second `cfg` condition exists for — a feature is not target-scoped, so
  `--all-features` sets `proptest` on a target where the crate is not in the
  graph at all, and nothing was checking that the guard held.

- **The wire format changed, and anything written under `serde` before this
  release is a different document now** ([ADR-0016](.kb/decisions/0016-the-wire-format.md)).
  There is no compatibility arm and none is offered: nothing publishable has
  shipped and no peer is deployed, so this is the last release in which that
  is true. Concretely — `Event`, `QueryItem` and `Guard` previously omitted a
  field that carried its default; every field is now always written, on both
  ends. `Query` gains a shape it never had: `Query::All` serialises as the
  externally tagged `"All"` where it used to be JSON `null`, and
  `Query::Items(..)` as `{"Items":[...]}`. `StoreId` serialises as a
  32-character lowercase hex string in JSON, not an array of sixteen
  integers, and rejects anything else — including the string it was
  previously accepting with separators, `0f1e2d3c-...`, which now fails to
  parse. Event and metadata payloads serialise as base64, not as an array of
  per-byte integers. `ReadOptions` no longer implements `Serialize` or
  `Deserialize` at all — it never crossed the wire correctly, and phase 5
  stopped pretending it did rather than fix an encoding nothing consumed.
  `happenstance-sync` gains `wire::Envelope<T>`, the one type this workspace
  puts on a wire: it carries a `format_version` next to the message and is
  the only place a peer should look to find out whether it can read what it
  was just sent.

### Fixed

- **`MemoryProjectionStore::default()` handed every instance the same identity,
  which disabled the foreign-batch check.** The struct derived `Default`, and the
  derive fills the per-instance `stamp` with `0` while the counter starts at `1`
  — so two stores built with `default()` compared equal, `b.commit(a.begin(), ..)`
  and `b.reset(a.begin(), ..)` were both *accepted*, and rows and checkpoint were
  mutated by a batch the receiving store never opened. `CommitError::ForeignBatch`
  and `ResetError::ForeignBatch` were unreachable through that constructor while
  the tests, which all used `new()`, stayed green. `Default` is now hand-written
  as `Self::new()`, mirroring `MemoryEventStore`, and
  `commit_rejects_a_foreign_batch_from_default_stores` plus its `reset` twin fail
  on the derive. This matters more than an ordinary bug: this store is the oracle
  a failing adapter is presumed wrong against.

- **`cargo doc -p happenstance-core` failed on the crate's *default* feature set
  while both existing doc steps passed.** `MemoryProjectionStore`'s page linked
  `ProjectionProbe::READS_THROUGH_BATCH`, an item gated on `conformance`, and
  `rustdoc::broken_intra_doc_links` is `deny` — so the configuration a consumer
  gets from `cargo add` was a hard error, invisible to `--all-features` (the gate
  is open, the link resolves) and to `--no-default-features` (the page is never
  rendered). The link is now spelled plainly, and a third gate step,
  `documentation (default features)`, builds that configuration so the blind spot
  cannot reopen. Same defect class as D13, one feature axis over.

- **`happenstance` and `happenstance-core`'s READMEs promised "MSRV 1.85,
  checked in CI".** Phase 2 raised the floor to 1.97.1
  ([ADR-0029](.kb/decisions/0029-msrv-raised-to-1-97-1.md)) and neither README moved
  with it, so the one artefact `cargo package` ships to a reader who has not
  cloned the repository carried a compatibility promise twelve minor versions
  below the manifest's own `rust-version`. Nothing checks a README's prose
  against a manifest, which is why it survived a phase: the gate asserts the
  file is *present* in the package, not that it is true.

- **`cargo xtask spec-trace` rendered a clause whose rule is a meta-test as
  though it were checked, and it is not — not by `spec-trace`.** `collect_rules`
  scans `suite.rs` only, so the CF clauses whose rules live in
  `happenstance-testkit`'s `tests/` resolved to nothing and §7.2 printed a cell
  that *looked* verified. `rules_of` now classifies `meta-test` as `elsewhere`,
  beside `unit test` and `compile test`, so those cells render in the clause's
  own words and say plainly that the checker did not look. What it detects: the
  same staleness class this phase has already fixed twice — a citation that
  passes a range check while pointing at nothing.

- **`MemoryEventStore::append(&[], Some(&condition))` returned
  `ConditionViolated` where it should return `NoEvents`** (D8, ES-20, CF-11).
  Which of the two you got depended on what the store happened to hold, so two
  conformant adapters could disagree about the same call — and
  `ConditionViolated` is the DCB concurrency signal, documented as *rebuild the
  decision model and retry*, so a caller whose retry loop branches on
  `is_condition_violated()` never terminated. An empty batch is the caller's own
  bug and will still be empty next time. The emptiness check now precedes the
  condition check and is taken *above the lock*, because it is a precondition on
  the argument rather than a question about the store. `NoEvents` is now
  documented on `EventStore::append`, which it was not.
  `empty_batch_is_refused_before_the_condition_is_evaluated` is the rule, and
  `ConditionBeforeEmptinessStore` — the old order — is the mutant that fails it.
  The same ordering had been copied into `LocalMemoryEventStore` deliberately
  and into the mutant registry's correct core by accident; all three are fixed.
- **`condition_rejection_leaves_store_unchanged` passed for a store that showed
  nothing.** It compared two reads for equality, and two empty reads are equal.
  It now asserts that both appended events are visible first — the same
  non-vacuity anchor `read_defaults_to_ascending_order` carries, for the same
  reason — and `InnerJoinTagStore` fails it.
- **Five ordering rules were satisfiable by the wrong index.**
  `query_all_matches_every_event`, `read_defaults_to_ascending_order`,
  `read_backwards_reverses_order`, `read_limit_truncates` and
  `positions_are_strictly_monotonic` appended event types in *ascending*
  alphabetical order, so insertion order and type order were the same sequence
  and an adapter answering from a covering index on `(type, position)` — the
  first index anyone adds, and one the planner will use with no outer sort —
  passed all five while returning its own order. They now append descending
  types (`"Cee"`, `"Bee"`, `"Ay"`). **An adapter pinning the testkit and taking
  this bump should expect these five to go red if it orders by anything but
  position**; that is the defect they now detect, not a change of contract.
  `SortByEventTypeStore` is the registered mutant.
- **`query_items_are_or` could not tell a union from a concatenation.** No event
  matched more than one query item, so returning each item's matches in turn
  passed it. One event now matches both items, and `ItemsAreAndStore` is the
  registered mutant for the other half.
- **A fixture declining `SECOND_HANDLE` no longer buys a green suite.** It is a
  MUST (CF-16), not a trade, and the capability constant could not tell the two
  apart — so an adapter author meeting a red
  `two_handles_observe_each_others_appends` could decline the capability and get
  a pass plus one `SKIP` line. The rule now fails, quoting the fixture's own
  stated reason. `REOPEN` is unaffected: it is a genuine `SHOULD` and still
  reports a skip.

- **A declined capability was reported nowhere on `wasm32-unknown-unknown`** —
  the one target the two-flavour port design exists for, and the only one whose
  conformance harness CI executes rather than type-checks. `println!` is a silent
  discard there: that target's `std` has no host stdio and declares no import
  that would give it one, and `wasm-bindgen-test` hooks `console.*` rather than
  `println!`. Measured under `wasm-bindgen-test-runner --nocapture` before and
  after. `RuleOutcome::skip_line` is the sinkless half that fixes it, and
  `__emit_wasm` routes it to `console_log!` — resolved through the caller's own
  `wasm-bindgen-test`, so the testkit gains no dependency.
- **The one test guarding the two-flavour design could not fail.**
  `read_stream_is_send` asserted `Send` on a *concrete* stream, where auto-trait
  leakage satisfies it whatever the trait promises — it passed unchanged with
  `Send` struck from the derivation attribute entirely. It is replaced by two
  generic tests, and it takes two: writing the bound at the definition fixes the
  leakage, but under the `async fn read(..) -> Result<impl Stream, E>` refactor
  the outermost item is the *future*, so an assertion on the call's result is
  discharged against the future while the stream stays `!Send`. Only
  `spawns_from_generic`, which holds a read across an await inside a real
  `tokio::spawn`, rejects that.
- **`ADR-0001`'s provisional marker is lifted**, against the condition its own
  banner named. The full proof — a real `!Send` adapter — is still the Cloudflare
  work, and the banner now says which half is settled.

- **The published `.crate` contained no licence text and no README** (D11). Both
  licence files lived at the repository root, and Cargo packages only what is
  inside the crate directory — so every crate would have shipped
  `MIT OR Apache-2.0` in its metadata and nothing in the tarball.
- **The README never compiled** (D10). Its Quick start used `?` and `.await` at
  the top level, and the command-loop example referenced a `store` that was never
  defined. Both are now compiled as doctests.
- **`cargo doc --no-default-features` failed** (D13). Three intra-doc links named
  `MemoryEventStore`, which does not exist without the `memory` feature, and
  rustdoc treats a broken intra-doc link as a hard error rather than a warning.
- **The `serde` feature depended on `serde/alloc` arriving transitively** (D12)
  through `bytes/serde`, which is free to stop providing it in a patch release.
- **The specification's §7.1 summary had been wrong since the document was
  assembled.** It totalled 137 `[FROZEN]` / 41 `[PROVISIONAL]` / 14 `[DEFERRED]`
  / 1 `[NON-NORMATIVE]`, while §7.2's own rows aggregate to 132 / 46 / 13 / 2 —
  so it disagreed with the table twenty lines below it and with §1.3 six thousand
  lines above it. Both miscounts inflated `[FROZEN]`. Regenerating it also removed
  thirteen phantom conformance-rule names that were ordinary words in code
  formatting, and surfaced six pairs of rules named differently by §6 and §3.
- **The gate denied no rustdoc lint.** `RUSTFLAGS: -D warnings` reaches every
  `rustc` invocation and no `rustdoc` one, so the documentation step had been
  reporting warnings and exiting 0. Both documentation steps now set
  `RUSTDOCFLAGS`, and the three warnings this exposed are fixed.
- **Nothing in CI installed a nightly toolchain**, so the `docsrs` step's probe
  failed and it printed `skipped` on every runner while two comments and the
  runbook asserted it was running.
- Documents that the `happenstance-core` rename had inverted rather than merely
  dated — including a module that restated the "no `serde` in the contract crate"
  constraint against the crate whose entire purpose is encoding, and an accepted
  ADR linking to a source path that no longer exists.

- **A bare `Event` with no tags could not be read back under `postcard`, and
  next to another value in the same buffer it decoded to the wrong value
  instead of an error** (D1). An empty `tags` field and an omitted one are the
  same eight bytes in postcard's format, which has no field names to tell them
  apart by, so a zero-tag event either failed to decode on its own or, with a
  neighbour in the buffer, quietly consumed that neighbour's bytes and
  produced a different, valid-looking `Event`. Every field that used to be
  skipped when it held its default is now always written, which is what makes
  the boundary between one value and the next unambiguous.
- **`{"guards":[{}]}` and `{"guards":[{"query":null}]}` both decoded to an
  `AppendCondition` that matched every event in the store** (D6). Both are
  what a `Guard` missing its `query` field defaulted to, and the default for
  an append condition's query was `Query::All` — so the most destructive value
  the protocol can express was also the one a peer produced by leaving a field
  out, whether by a bug or by economy on the wire. `Guard::query` is no longer
  optional on the wire: an append condition now has to name what it is
  guarding, and a document that omits it is rejected rather than decoded.

[0.2.0-alpha.1]: https://github.com/Wet-Ink-Corporation/happenstance/commits/main
