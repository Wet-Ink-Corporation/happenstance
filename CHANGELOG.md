# Changelog

Notable changes to `happenstance`, `happenstance-core` and `happenstance-testkit`.

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
  feature** and is exempt from semver until two adapters at opposite ends of the
  batch-shape axis have passed its conformance suite. See
  [`docs/architecture/SPECIFICATION.md`](docs/architecture/SPECIFICATION.md) §4.

## [Unreleased]

### Added

- **Six adapter skeletons, as instruments rather than as adapters.**
  `happenstance-cloudflare`, `happenstance-postgres` and `happenstance-neon` are
  new; `happenstance-sqlite`, `happenstance-ladybug` and `happenstance-sync` grew
  real associated types. Every body is `todo!()` and every crate is
  `publish = false`, so nothing here changes what a consumer sees — but the ports
  have now been disagreed with by five storage shapes instead of one, and
  [`docs/adapter-shapes.md`](docs/adapter-shapes.md) records what each one said.
- [ADR-0009](docs/adr/0009-error-send-sync.md), settling ES-6 — the highest
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
- [ADR-0008](docs/adr/0008-one-derivation-for-both-ports.md), which puts
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
  ([ADR-0010](docs/adr/0010-the-suite-must-prove-itself.md)). Fifty-two stores
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

### Changed

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
  layer** ([ADR-0006](docs/adr/0006-bare-name-to-the-typed-layer.md)). Today
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

### Fixed

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

[Unreleased]: https://github.com/Wet-Ink-Corporation/happenstance/commits/main
