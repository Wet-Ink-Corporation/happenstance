# ADR-0086 — Postgres and Neon keep mint-once, earned by a documented re-mint, and mint-per-open is declined

- **Status:** proposed. The owner decides. Two calls in it are one-way, §8 and §9.
- **Date:** 2026-10-07
- **Phase:** 17 (the breaking window). The item is at `runbook/phases/17-breaking-window.md:128-136`.
- **Answers:** the phase-17 half of
  [`kb-open-question-postgres-neon-store-id-no-restore-001`](../../.kb/open-questions/postgres-neon-store-id-has-no-restore-detection.md).
  That half is whether either adapter takes mint-per-open, the one remedy that is a behaviour
  change. The record also answers the question's five sub-questions at the grain of a decision.
  The building stays with phase 13 (`runbook/phases/13-sync.md:66-83`).
- **Supersedes:** nothing.
  [ADR-0014](../../.kb/decisions/0014-event-identity-and-recorded-time.md) wrote VT-6's
  conditional MUST (`references/adr/0014-event-identity-and-recorded-time.md:238-242`). It is
  cited, not amended, and its body is immutable.
- **Evidence:** a code walk of `main` at `4fbfefa`, whose tree is identical to `7a8b898`, the
  checkout every citation below was re-verified against (`4fbfefa` is not an ancestor of it, so
  cite the tree rather than the history), and re-checked at `6235224`, where the
  `happenstance-sqlite` citations were repointed. Nothing new was measured. Every claim below
  that would need a live server is marked *unmeasured*.

---

## 1. What is true today

### The clause

VT-6 (`spec/SPECIFICATION.md:848-906`) is `[PROVISIONAL]`. ADR-0066 marks it freeze-by-13
(`.kb/decisions/0066-what-1-0-promises.md:161`; `runbook/ledgers.md:275`). It says four things.

1. A store MUST NOT issue a `(StoreId, SequencePosition)` pair twice (`:852-854`).
2. Mint-once is allowed **only if** the adapter can detect a restore or clone, **or** the
   deployment is documented to invoke the re-mint. Otherwise the adapter MUST mint on every open
   (`:894-897`).
3. The chosen mechanism MUST be recorded in `references/adapter-shapes.md` (`:897-898`).
4. The marker's falsifier is mint-per-open producing so many incarnations that peer watermarks
   grow without bound (`:856-861`). A `Watermark` keeps one `(StoreId, SequencePosition)` entry
   per origin (`crates/happenstance-sync/src/identity.rs:81-95`).

### The four persistent adapters

| | Mint | Where the append gets the id | Re-mint | Detection | adapter-shapes row |
| --- | --- | --- | --- | --- | --- |
| `happenstance-sqlite` | once, in `migrate` | re-read inside the append's `BEGIN IMMEDIATE`, refusing a stale handle with `IdentityMoved` (`crates/happenstance-sqlite/src/event_store.rs:746`, `:1253-1264`, rationale `:699-732`) | `SqliteEventStore::remint_identity(path)` (`:597-646`) | none, and none is possible | present (`references/adapter-shapes.md:382`) |
| `happenstance-postgres` | once, migration 1 (`migrations/0001_event_log.sql:102-118`) | `Arc<OnceLock<StoreId>>` filled on first use (`src/event_store.rs:183`, `:396-420`), read once per append (`:537`), bound on every row by `insert_batch` (`:1001-1043`) | none | none | "undecided" (`references/adapter-shapes.md:385`) |
| `happenstance-neon` | once, migration 1 (`migrations/0001_neon_log.sql:126-141`) | a scalar subquery inside the `INSERT … SELECT`, `local_origin()` (`src/event_store.rs:564-575`), shared by append (`:527`) and ingest compensation (`:701`) | none | none | "undecided" (`:385`) |
| `happenstance-cloudflare` | once, at the first `migrate` | `RefCell<Option<StoreId>>` (`src/event_store.rs:364`, `:575-591`), pinned by `store_id_is_not_reminted_per_handle` (`:2254`) | none | none | "undecided" (`:385`) |

`happenstance-ladybug` is listed beside them in that "undecided" row, but it never had an event
store. It was a projection store, and it is retired (ADR-0078). It never owed a VT-6 row.

**Neon has a latent hole.** `origin_store` is nullable (`0001_neon_log.sql:71-72`), and a scalar
subquery that finds no row yields NULL. An append against a schema with no meta row would
therefore commit rows with a NULL origin. The failure surfaces only on read, as `UnstampedEvent`
(`src/event_store.rs:1224-1238`), whose own documentation names the absent row as "the reachable
cause" (`src/error.rs:145-156`). Postgres does not have this hole today, because `store_id()`
refuses with `MissingIdentity` before the transaction opens.

## 2. A correction: mint-per-open does not fail the suite

The open question says that mint-per-open "fails `reopened_store_does_not_reissue_an_event_id`
outright" (`.kb/open-questions/postgres-neon-store-id-has-no-restore-detection.md:124-127`).
`references/adapter-shapes.md:387-393` said the same, citing `suite.rs:2310`. The claim is stale,
and the change that lands this record corrects both texts.

- ADR-0014 replaced VT-6's original rule, `store_id_is_stable_across_reopen`, **because** that
  rule failed mint-per-open (`references/adr/0014-event-identity-and-recorded-time.md:247-256`).
  The replacement's own documentation says so
  (`crates/happenstance-testkit/src/suite.rs:2594-2601`).
- The rule today (`:2625-2693`) asserts two things only. Two events appended in one open share a
  `StoreId`. An event appended after a reopen has an `EventId` matching neither of them.
  Mint-per-open, or mint-per-handle, passes both.
- The suite rejects per-*append* or per-*event* minting, through the first assertion and through
  `append_stamps_a_local_event_id` (`:2339-2386`, whose comment at `:2328-2335` names
  `PerEventStoreIdStore`).

So the case against mint-per-open has to rest on cost and coverage, which §4 makes. Both texts are
corrected in the change that proposes this record, because they are false whatever the owner
decides.

## 3. The options, classified

"One-way" here means it cannot be undone after the `0.4.0` window without a major release.

| | Option | Semver class | Door |
| --- | --- | --- | --- |
| A | Mint per open (per `new`, per handle or per pool) | behaviour change on two published crates; on Neon also a signature change (§4.5) | one-way |
| B | A re-mint operation, plus Postgres reading the id inside the append | additive: new inherent methods, a private field removed | two-way |
| C | Detection: a stored fingerprint compared on use | storing it is additive; **refusing by default is a behaviour change** | storing is two-way; default refusal is one-way |
| D | A documented procedure plus the adapter-shapes rows | rustdoc and README prose, exempt (`.kb/decisions/0066-what-1-0-promises.md:265`) | two-way |

## 4. Why mint-per-open lost

### 4.1 A server store has no natural "open"

`PostgresEventStore::new(pool)` is synchronous and infallible by design (`src/event_store.rs:305`,
with the reasoning on the field at `:175-182`). `NeonEventStore::new` is a `const fn`
(`crates/happenstance-neon/src/event_store.rs:311`), and on Workers it is typically built per
request, because the transport's credentials arrive with the request's environment. That is
inferred: no deployment in the tree shows it, and a long-lived global handle is possible. The
nearest thing to an "open" is the first append on a handle. So:

- **On Neon**, mint-per-open means about one incarnation per request. That is the harm VT-6's
  marker describes: so many incarnations that every peer's watermark grows without bound
  (`spec/SPECIFICATION.md:856-861`). It is not literally the marker's falsifier, which presumes
  the mint-once path is *unavailable* and names a Durable Object as its candidate. Here mint-once
  is available, so taking A would incur that harm on a published adapter by our own choice.
- **On Postgres**, the incarnation count is roughly process starts × replicas × deploys. That is
  smaller, but every peer's `Watermark` still keeps every one of them for ever.

### 4.2 It does not close the gap on these platforms

ADR-0014 called mint-per-open safe by construction, "because the restored copy is a different
incarnation before it writes anything"
(`references/adr/0014-event-identity-and-recorded-time.md:229-233`). That holds where every storage
discontinuity coincides with an open: a file reopened after a copy, or an isolate revived after
eviction. It does not hold here.

- A `PgPool` is long-lived, and it re-establishes connections on demand. A `pg_restore --clean`
  into the same database, or a PITR, rewinds `event_position_seq`
  (`0001_event_log.sql:73`) underneath a handle that is still running. The handle then stamps the
  id it minted at its own "open" onto positions it has already issued.
- A Neon handle is stateless across requests. A branch *reset* or *restore* rewinds the branch
  underneath a handle that is still in use.

So A buys none of the self-enforcement that is its only advantage, and it pays the full cost
of §4.1.

### 4.3 Neon has nothing to mint with

`happenstance-core` mints nothing (`spec/SPECIFICATION.md:900-902`). The Neon crate takes no
randomness dependency either: its only "mint" is a process-global counter for projection batch
stamps (`crates/happenstance-neon/src/projection_store.rs:107-122`), which is not 128 random bits.
There are two ways to get an id, and both are bad:

- **An extra `SELECT gen_random_uuid()` round trip per handle.** This is the store whose whole
  design is "two statements in one request is safe; two round trips is not" (`src/lib.rs:115-119`).
- **`gen_random_uuid()` inlined into each append statement.** That mints per *append*, which the
  first assertion of `reopened_store_does_not_reissue_an_event_id` and the whole of
  `append_stamps_a_local_event_id` reject (`suite.rs:2339-2386`).

### 4.4 It fragments one log across many origins

VT-6 itself names this as the cost of the second mechanism (`spec/SPECIFICATION.md:888-890`). The
per-origin causal continuity that VT-5's convergent-fold argument wants (`:885-887`) is lost. The
item phase 17 pinned for phase 13 — "an unheld event claiming this store's own `StoreId` … wedges
the local append" (`runbook/phases/17-breaking-window.md:322-324`) — turns from an equality check
into membership in an unbounded set of "our" ids.

### 4.5 On Neon it is also a signature problem — Rust detail

`NeonEventStore<T>` is `#[derive(Clone)]` with two private fields (`src/event_store.rs:189-193`),
and its constructor is `pub const fn new` (`:311`). Here is what minting per handle would need,
and why each shape costs something.

- **`Arc<OnceLock<StoreId>>`, as Postgres has.** `Arc::new` allocates, and allocation is not
  allowed in a `const fn`. So `new` would have to lose `const`. Removing `const` from a public
  function is a **major** change, because a caller may use it in a `static` or `const` item, and
  that caller stops compiling. `cargo-semver-checks` reports exactly this.
- **A bare `OnceLock<StoreId>` field.** `OnceLock::new` *is* `const`, so `new` keeps its
  signature. But `#[derive(Clone)]` on a struct holding a `OnceLock<T: Clone>` clones the *cell*.
  A clone made before the first append gets its own empty cell and mints its own id. "One
  incarnation per handle" silently becomes "one per clone made early". Wherever a store is
  cloned before its first append (for example into a closure on Workers), that is a further
  multiplier on §4.1.
- **Auto traits.** `NeonEventStore<T>` is `Send` and `Sync` exactly when `T` is, because Rust
  derives auto traits from the fields ("auto-trait leakage"). `OnceLock<StoreId>` is `Send + Sync`,
  so this one would not break. The general point stands, though: on a published type, every new
  private field is a semver question, even when the field is invisible.

Postgres has neither problem. Its `new` is not `const`, and its cache already lives behind an
`Arc` that clones share. Its objection is §4.1 and §4.2.

### 4.6 Multiple handles, replicas and contenders

The conformance concurrency family runs 64 contenders. Under per-handle minting, that is 64 ids
for one log. Read replicas would see the writers' ids. Nothing breaks, and nothing is gained.

## 5. The semver of the remedies that remain — Rust detail

Each of these is additive. The reasons are spelled out because they are not obvious from the
diff.

- **A new inherent method** (`PostgresEventStore::remint_identity`,
  `NeonEventStore::remint_identity`) is a *minor* change under Cargo's SemVer reference. The one
  hazard is shadowing. In method-call syntax an inherent method wins over a trait method of the
  same name, so adding one can silently change which function an existing `store.foo()` call
  resolves to. No trait in `happenstance-core`, the testkit or the sync crate names
  `remint_identity`, so nothing is shadowed. SQLite's is an associated function taking a path
  (`crates/happenstance-sqlite/src/event_store.rs:631`), because a file can be re-minted with no
  store open. A server store re-mints through the pool it already has, so `&self` is the natural
  receiver. The name is what matches, not the shape.
- **A new free function or `pub const`** in each crate's `migration` module (`remint_request(&NeonConfig)`
  beside `migration::request` at `crates/happenstance-neon/src/migration.rs:86`; a
  `REMINT_IDENTITY: &str` beside `MIGRATION_1` at `crates/happenstance-postgres/src/migration.rs:36`)
  is additive. It is also the stronger tool for operators. `MIGRATION_1` is published because
  "tooling needs the text, and the alternative is that they copy it out of the repository and it
  drifts" (`:29-31`). Exactly the same is true of a statement a `pg_restore` script or a Neon
  branch hook runs.
- **Removing the private `store_id` field** from `PostgresEventStore` is invisible to dependents.
  The struct is `#[non_exhaustive]` with only private fields (`src/event_store.rs:171-183`), so no
  downstream crate can name, construct or destructure it. Its auto traits are unchanged, because
  `PgPool` and the remaining fields are already `Send + Sync`.
- **A new error variant** (`NeonError::MissingIdentity`) is additive because `NeonError` is
  `#[non_exhaustive]` (`crates/happenstance-neon/src/error.rs:64-65`). Every downstream `match`
  already needs a `_` arm. Behaviourally it turns an append that used to "succeed" into an
  error. Phase 13 can make that change because the old success wrote rows nobody could read back
  (`UnstampedEvent`), so no working deployment depends on it. That is the difference between this
  refusal and §8's.
- **An opt-in detection switch** after 1.0 is additive too. `NeonConfig` is `#[non_exhaustive]`
  (`crates/happenstance-neon/src/config.rs:46-47`), so a new field plus a `with_*` builder is
  minor. A new builder method on `PostgresEventStore` is minor. What is *not* additive is changing
  what an existing constructor does by default. That is a behaviour change, and no semver tool
  can see it, because the signatures are identical and only the outcomes differ.

The last point is why this record exists in the window at all. ADR-0079 met the same trap with
`planned_statement_count` (its §3): a changed value is invisible to `cargo-semver-checks`, so
policy, not a tool, has to say when it may change.

## 6. The documented procedure

**Whether this arm is earned without code is the owner's reading, not a fact.** VT-6's first
mechanism is to "mint once at schema creation and provide an explicit re-mint operation the
deployment invokes after a restore" (`spec/SPECIFICATION.md:884-886`), and the permission at
`:894-896` is for that mechanism. The open question described the arm as "a
`remint_identity`-shaped operation, plus the procedure"
(`.kb/open-questions/postgres-neon-store-id-has-no-restore-detection.md:115-118`), and SQLite
earned it with a method. This record reads a published SQL statement, documented in the crate
root, as the operation the deployment invokes. A stricter reading, that the *adapter* must
provide it, leaves both crates outside VT-6 in `0.4.0` until phase 13's `remint_identity`, and
on Postgres the procedure's restart step exists only because the adapter's own cache defeats the
statement. Neither reading changes §8 or §9.

**When it lands.** Nothing in this section is in a crate root, a README or
`references/adapter-shapes.md` while the record is proposed. The procedure and the VT-6 mechanism
rows are acceptance edits: they land in the PR that flips the atom to `accepted`, and Appendix A
carries their exact text so that PR can paste it. Until then the adapter-shapes row for Postgres
and Neon (`references/adapter-shapes.md:385`) still says "undecided", which is true.

The statement is the same one migration 1 runs, turned from an insert into an update. A later run
of migration 1 cannot undo it, because that insert is `ON CONFLICT (k) DO NOTHING`
(`0001_event_log.sql:116-118`).

**The procedure.** It is the same in the atom, here and in Appendix A. It applies after a
`pg_restore`, a point-in-time recovery, a promoted replica that lost acknowledged writes, any
other copy, or a Neon branch that is created, reset or restored.

1. **Stop every writer.** Nothing appends to the copy until step 4.
2. **Run the re-mint** against the copy:
   ```sql
   UPDATE store_meta SET v = uuid_send(gen_random_uuid()) WHERE k = 'store_id';
   ```
   On Neon, use the configured schema and meta table (`NeonConfig::qualified_meta`).
3. **Postgres only: restart every process holding a `PostgresEventStore`** on that database (or
   drop every handle and every clone of it and construct a new one), so that no cached id survives.
4. **Resume appends.**

**Why the order is what it is, on Postgres.** `PostgresEventStore` caches the identity in an
`Arc<OnceLock<StoreId>>` (`crates/happenstance-postgres/src/event_store.rs:183`). `store_id()`
fills it from `store_meta` on first use and never reads the row again (`:396-420`), and every
clone shares the one cell, because cloning an `Arc` clones the pointer, not the cell. A `OnceLock`
cannot be reset through `&self` (§7.1), so only dropping every clone empties it.

- *Stale-cache race, so step 1 is first.* A writer that keeps running after the restore stamps
  its cached, retired id onto positions the rewound sequence hands out again. Those pairs repeat
  ones already issued. Running the `UPDATE` does not help that writer, because it never re-reads
  the row.
- *Early-recache race, so step 3 is after step 2.* A process restarted before the `UPDATE`
  commits fills its fresh cell from the retired row on its first append, and keeps it for its
  whole lifetime. Restarting first and re-minting second leaves exactly the hazard the restart was
  for.
- *Step 4 is last* because a resumed writer that was only paused, not restarted, still holds a
  filled cell.

**On Neon there is no cache.** `local_origin` stamps the identity with a scalar subquery inside
each append's `INSERT … SELECT` (`crates/happenstance-neon/src/event_store.rs:570-575`), so the
first statement after the `UPDATE` commits reads the new row. Step 3 does not apply. Steps 1 and 2
still do, because an append that reaches the copy before the `UPDATE` commits stamps the parent's
id onto a rewound position.

**Why the failover trigger is named, though it is unmeasured.** Under asynchronous replication, a
failover can lose the tail of the primary's WAL. PostgreSQL WAL-logs a sequence ahead of use, 32
values at a time. Whether a promoted replica can therefore hand out a position the old primary
already issued and acknowledged depends on whether the sequence's own WAL record was among what
was lost. Nobody has measured that here. Naming it costs nothing, and leaving it out could cost a
silent drop. It also appears as a falsifier (§11), because a failover has no operator in the loop
to follow a procedure.

## 7. What phase 13 (or 17b) builds

### 7.1 Postgres: stamp in the transaction, and drop the cache

`insert_batch` (`src/event_store.rs:1001-1043`) takes the identity from the transaction instead of
as a parameter. There are two shapes, and the choice is a two-way implementation call.

- **A `SELECT v FROM store_meta WHERE k = 'store_id' FOR SHARE`** on the transaction before the
  insert, decoded by the code `store_id()` already has (`:409-415`). It keeps the `QueryBuilder`
  shape, and `FOR SHARE` makes a concurrent re-mint wait for the append to commit. It costs one
  more statement round trip per append.
- **A join in the insert itself**, so the identity costs no extra round trip:
  `INSERT … SELECT …, m.v, … FROM (VALUES …) AS b(…) CROSS JOIN store_meta AS m WHERE m.k = 'store_id'`.
  Compare the affected-row count against the batch length, and refuse with `MissingIdentity` on a
  short count.

**The wrong implementation each must reject is a bare scalar subquery in `VALUES`.** That subquery
writes NULL into the nullable `origin_store` (CHECK at `0001_event_log.sql:49-50`) and commits an
unreadable row. It is Neon's current hole (§1), and it is what the naive edit produces.

Then the `store_id` field, the `store_id()` method and the call at `:537` go. Whether to keep a
public getter is separate. No `IdentityMoved` analogue is needed, because once the handle holds no
copy there is nothing stale to compare. SQLite needed the compare because its handle caches the id
read at construction (`crates/happenstance-sqlite/src/event_store.rs:699-729`). Here the persisted
row *is* the value used. This is the Rust idiom of making the invalid state unrepresentable: delete
the cached copy rather than guard it.

**Why not invalidate the cache instead?** `OnceLock` can be written once and cannot be reset
through `&self`. Only `take(&mut self)` resets it, and a cell shared through an `Arc` never gives
anyone `&mut`. A resettable cell, such as an `RwLock<Option<StoreId>>`, would still need a signal
that a re-mint happened. A re-mint run from `psql`, or from another process, sends none.

**Concurrency (sub-question 2).** The extra read is one primary-key lookup in a transaction the
append already holds. A re-mint racing an in-flight append is harmless. The re-mint does not
rewind the sequence, so rows stamped with the old id still take fresh positions and no pair
repeats. Under `SERIALIZABLE` (conditional appends, `:907-912`), one read-write dependency on
`store_meta` does not on its own form a dangerous structure.

### 7.2 Neon: refuse on a missing row

Turn `local_origin`'s scalar subquery (`src/event_store.rs:570-575`) into a join that yields zero
rows when the meta row is absent. Detect the short insert and refuse with a new
`NeonError::MissingIdentity`. The re-mint itself is one statement against
`config.qualified_meta()` (`src/config.rs:171-173`). Nothing on the stamping side changes, because
every append already reads the row.

### 7.3 Tests, each naming the implementation it rejects

- **`remint_changes_the_id_stamped_by_an_already_open_handle`** (Postgres, live). Open handle 1,
  append, re-mint through handle 2, then append through handle 1. Assert that the second event's
  `id.store()` is the re-minted id and differs from the first event's. It rejects the shipped
  `OnceLock` cache. That cache passes any test that opens a *fresh* handle after the re-mint,
  which is why this test reuses the old one.
- **`append_refuses_without_an_identity_row`** (both). Delete the meta row and append. Expect an
  error, and expect zero rows written. It rejects the scalar-subquery stamping in §7.1, and it is
  red on Neon today.
- **`restored_peer_does_not_reissue_identities`** (sync-testkit, named by VT-6 at
  `spec/SPECIFICATION.md:865-866`). Append, rewind the store to an earlier state, re-mint, append,
  and assert that the new pair is not among the old ones. The rewind can be a schema copy, or a
  `setval` on the sequence plus deleting the tail rows. Its **negative control** runs the same
  sequence without the re-mint and must collide; without that control the rule is decorative. A
  server fixture needs a new capability, for example `RESTORE`. Adding it as a defaulted associated
  `const` on `Fixture` is additive, because every existing `impl Fixture` inherits the default.

## 8. The window-bound decision: default-refusing detection

This is the one thing in the lane that cannot wait. Detection that **refuses** an append on a
fingerprint mismatch makes an append that succeeds on `0.3.x` start failing. The clearest case is
a Neon preview branch that is appending today. That is a behaviour change. After `0.4.0` it can
only arrive opt-in, through a builder or config field (§5), or report-only, through something like
`pub async fn identity_fingerprint_matches(&self) -> Result<bool, _>`.

The record recommends ruling default refusal out, for two reasons.

1. **No candidate catches the hazard that matters most.** All three are unmeasured.
   - `pg_control_system()`'s `system_identifier`: whether a role that is not a superuser can read
     it is unverified, and Neon branches probably share it.
   - `pg_database.oid` for `current_database()`: it catches a restore into a *second* database. It
     misses a `pg_restore` into the same database, and probably a copy-on-write Neon branch.
   - Neon's `neon.timeline_id`: it might detect a branch, but a branch reset or restore keeps the
     timeline.

   None catches a same-database restore from an older dump. So detection is at best a supplement
   to the procedure, never a replacement for it, and a mandatory refusal would be a breaking change
   bought for partial coverage.
2. **Building and measuring it in `0.4.0`** means live probes on both platforms, inside a phase
   already re-estimated at 25–30 days (`runbook/phases/17-breaking-window.md:287-292`).

If the owner wants default refusal, it must be built in this window. Otherwise it is opt-in for
ever. Appendix B, question 2, puts this to the owner.

## 9. The other one-way call: mint-per-open

This is §4's conclusion, stated as the call. Neither adapter mints per open. After `0.4.0`,
taking mint-per-open is a post-1.0 major (`runbook/phases/13-sync.md:80-83`). The phase-17
session log records the decline.

## 10. Consequences

- Safety rests on a procedure, which is the cost VT-6 attaches to this arm: "a procedure a human
  can forget" (`spec/SPECIFICATION.md:886-887`).
- **Neon preview branches are the weakest point.** The platform's control plane creates them, so
  the operator's branch tooling (CI, branch hooks) has to run the re-mint. The statement builder in
  §5 exists for that tooling.
- Until phase 13 lands §7.1, a Postgres re-mint needs a process restart. The procedure says so.
- **Phase 13's exit criteria are unchanged in substance.** `restored_peer_does_not_reissue_identities`
  must be green against a Postgres-backed peer with its negative control red, and both crates
  document what they do on a restore (`runbook/phases/13-sync.md:160-161`, exit criterion `:226-229`). The second
  half is discharged by the acceptance edits in Appendix A.
- **The VT-6 clause is unchanged.** It stays `[PROVISIONAL]`, freeze-by-13. Nothing here touches
  its text, and the clause permits the arm taken.

## 11. Falsifiers

Each of these reopens the record. Reversing it after `0.4.0` would be a major.

1. A recorded incident or measurement in which a deployment that followed the documented procedure
   still re-issued a pair.
2. Neon's control plane is shown to offer no point at which a statement can run on a new or reset
   branch before its first application append. The documented arm would then be unfollowable for
   the platform's standard workflow.
3. An automatic failover under asynchronous replication is measured to rewind
   `event_position_seq` below positions already acknowledged (§6). That discontinuity has no
   operator, so no procedure can cover it. It would push Postgres towards detection or towards a
   sequence floor, not towards mint-per-open, for the reasons in §4.2.

## 12. Not decided here

- **Cloudflare's earning arm.** Its mechanism is mint-once, but whether it can be restored
  underneath itself is unexamined. Durable Objects with SQLite storage are documented by the
  platform as supporting point-in-time recovery, which nothing in the tree mentions; this is
  unverified here. It is proposed as its own open question (Appendix C). Its adapter-shapes
  row says "mint once; earning arm undecided" once the Appendix A rows land.
- Which report-only fingerprint, if any, is built, after measuring which survive which operations.
- The method's placement: inherent, a `migration` function or constant, or both. This record
  recommends both on Neon and leaves Postgres open. The CTE-versus-`SELECT` choice in §7.1 is also
  open.
- ADR-0014's body. It is cited, not edited (`.kb/decisions/0014-event-identity-and-recorded-time.md:21-22`, `:81-84`).

---

## Appendix A. The acceptance edits, verbatim

These land in the PR that flips the atom to `status: accepted`, and not before. Line numbers are
against this record's base; re-check them when that PR is cut. Outer fences here are `~~~` so the
inner `sql` fences survive a paste.

### A.1 `references/adapter-shapes.md:385`, the VT-6 mechanism rows

Replace the one "undecided" row with three. Ladybug is dropped: it never had an event store, and
it is retired (ADR-0078). The file grows by two lines, so the two citations of
`references/adapter-shapes.md:395-401` (in
`.kb/open-questions/postgres-neon-store-id-has-no-restore-detection.md` and
`.kb/open-questions/remint-identity-precondition-is-trust-only.md`) become `:397-403`.

~~~
| `happenstance-postgres` | **mint once, in migration 1** | The documented re-mint (ADR-0086). After `pg_restore`, PITR, a failover that lost acknowledged writes, or any copy: stop every writer, run the re-mint statement on the copy, restart every process holding a `PostgresEventStore` (a handle and its clones cache the identity they read first), then resume appends. Detection is not taken: no candidate fingerprint has been measured, and none is expected to change under a restore into the same database | `crates/happenstance-postgres/migrations/0001_event_log.sql:102-118`; the procedure is in the crate root and README. `remint_identity` and stamping inside the append transaction are phase 13's |
| `happenstance-neon` | **mint once, in migration 1** | The documented re-mint (ADR-0086). On every branch that is created, reset or restored, and after any restore: stop every writer, run the statement against `NeonConfig::qualified_meta`, then resume appends. No restart is needed, because every append reads the row | `crates/happenstance-neon/migrations/0001_neon_log.sql:126-141`; the procedure is in the crate root and README |
| `happenstance-cloudflare` | **mint once, at the first `migrate`**, cached per handle | **Undecided.** Durable Object storage outlives the isolate, so eviction is not a restore (`store_id_is_not_reminted_per_handle`). Whether the platform's point-in-time recovery is a restore this adapter must answer for is open (Appendix C) | `crates/happenstance-cloudflare/src/event_store.rs:364`, `:575-591`, `:2254` |
~~~

### A.2 `crates/happenstance-postgres/src/lib.rs`, before `//! # Still open`

~~~
//! # After a restore, re-mint before the first append
//!
//! This store's identity, the `StoreId` half of every `EventId` it mints, is
//! one row that migration 1 writes once. A `pg_restore`, a point-in-time
//! recovery, a promoted replica that lost acknowledged writes, or any other copy
//! brings that row back together with an older position sequence, and the copy
//! would then re-issue `EventId`s it has already issued, which replication drops
//! silently (VT-6). In this order:
//!
//! 1. Stop every process that appends to the copy.
//! 2. Run, against the copy:
//!    ```sql
//!    UPDATE store_meta SET v = uuid_send(gen_random_uuid()) WHERE k = 'store_id';
//!    ```
//! 3. Restart every process holding a `PostgresEventStore` on that database. A
//!    handle and all its clones read the identity once and keep it, so a process
//!    started before step 2 would keep the retired one.
//! 4. Resume appends.
//!
//! Events already in the database keep the identity they were stamped with,
//! which is correct. Minting afresh on every open was considered and declined
//! (ADR-0086).
//!
~~~

### A.3 `crates/happenstance-postgres/README.md`, before `## Limits this store enforces`

~~~
## After a restore, re-mint before the first append

A `pg_restore`, a point-in-time recovery, a promoted replica that lost acknowledged writes, or any
other copy of this store's database brings back its identity row together with an older position
sequence. The copy would then re-issue event identities it has already issued, and a replication
peer would silently drop the new events as already seen. In this order:

1. Stop every process that appends to the copy.
2. Run, against the copy:
   ```sql
   UPDATE store_meta SET v = uuid_send(gen_random_uuid()) WHERE k = 'store_id';
   ```
3. Restart every process holding a `PostgresEventStore` on that database. A handle reads the
   identity once and keeps it, so a process started before step 2 would keep the retired one.
4. Resume appends.
~~~

### A.4 `crates/happenstance-neon/src/lib.rs`, before `//! # Which flavour, and which claim`

~~~
//! # After a branch or a restore, re-mint before the first append
//!
//! This store's identity, the `StoreId` half of every `EventId` it mints, is
//! one row that migration 1 writes once. A Neon branch that is created, reset
//! or restored, like a `pg_restore` or a point-in-time recovery, carries its
//! parent's identity together with an older position sequence, and would then
//! re-issue `EventId`s the parent has already issued, which replication drops
//! silently (VT-6). Stop every writer to the copy, then run against it, with the
//! schema and table `NeonConfig::qualified_meta` names (`"public"."store_meta"`
//! by default):
//!
//! ```sql
//! UPDATE "public"."store_meta" SET v = uuid_send(gen_random_uuid()) WHERE k = 'store_id';
//! ```
//!
//! Then resume appends. No restart is needed: every append reads the row inside
//! its own statement. A branch is made by the platform's control plane rather
//! than by this crate, so put the statement in whatever tooling makes branches.
//! Minting afresh on every open was considered and declined (ADR-0086).
//!
~~~

### A.5 `crates/happenstance-neon/README.md`, before `## Limits this store enforces`

~~~
## Re-mint every branch before its first append

A Neon branch is a copy of its parent, so it carries the parent's store identity together with an
older position sequence. A branch that is created, reset or restored, and equally a `pg_restore` or
a point-in-time recovery, would re-issue event identities its parent already issued, and a
replication peer would silently drop the new events as already seen. Stop every writer to the
branch, run this against it with your `NeonConfig`'s schema and meta table, then resume appends:

```sql
UPDATE "public"."store_meta" SET v = uuid_send(gen_random_uuid()) WHERE k = 'store_id';
```

No restart is needed. The platform creates branches, not this crate, so put the statement in the
tooling that does.
~~~

### A.6 Bookkeeping in the same PR

- The atom: `status: proposed` becomes `accepted`.
- `kb-open-question-postgres-neon-store-id-no-restore-001`: `status: superseded`,
  `superseded_by: kb-decision-0086`, a closing note answering its five sub-questions from §6–§8,
  and the matching entries in `.kb/maps/open-questions-index.md`, `.kb/maps/domain-map.md` and
  `.kb/maps/decision-map.md`.
- `runbook/phases/17-breaking-window.md:128-136`: tick the item and name ADR-0086;
  `runbook/phases/13-sync.md:79-83`: say mint-per-open was declined and list §7's build;
  `runbook/ledgers.md`: the 0086 queue row and the VT-6 disposition row.

## Appendix B. Questions put to the owner

1. **One-way.** Do `happenstance-postgres` or `happenstance-neon` mint a `StoreId` on every open in
   `0.4.0`, or do both keep mint-once, earned by VT-6's documented-re-mint arm? Recommended: keep
   mint-once (§4, §9). It flips on evidence that the procedure cannot be followed on Neon's standard
   branch workflow, or on a measured operator-free discontinuity only per-open minting would cover
   (§11).
2. **One-way.** Does `0.4.0` ship detection that refuses appends by default on a fingerprint
   mismatch, or is any detection opt-in or report-only for ever? Recommended: the second (§8). It
   flips on a measured fingerprint that survives everything except restore, clone and branch, on
   both platforms, and is readable by an ordinary role.
3. **Two-way.** Is a published statement plus a restart "an explicit re-mint operation" under
   VT-6 (`spec/SPECIFICATION.md:884-886`)? A strict reading leaves both crates outside VT-6 until
   phase 13's `remint_identity` (§6).
4. **Two-way.** On acceptance, is the open question superseded (recommended) or only narrowed, and
   do the Appendix A edits land in `0.4.0` (recommended) or in 17b? If 17b, `0.4.0` ships outside
   VT-6.

## Appendix C. The proposed Cloudflare open question

To be written at acceptance as
`.kb/open-questions/cloudflare-store-id-and-object-recovery.md`, with the id
`kb-open-question-cloudflare-store-id-and-object-recovery-001`, owned by phase 13.

> `happenstance-cloudflare` mints its `StoreId` once, at the first `migrate`, and caches it per
> handle in a `RefCell<Option<StoreId>>` (`crates/happenstance-cloudflare/src/event_store.rs:364`,
> `:575-591`); `store_id_is_not_reminted_per_handle` (`:2254`) pins it. Eviction is answered:
> Durable Object storage outlives the isolate. Not answered: (1) whether the platform can rewind an
> object's SQLite storage, by point-in-time recovery or otherwise, and whether a rewind includes
> `store_meta`, which needs the platform's documentation and a deployed-object probe under
> `harness/workerd/`; (2) if it can, a `remint_identity` and a procedure; if it cannot, the
> adapter-shapes row says why mint-once is earned with no procedure. Mint-per-open is declined by
> ADR-0086's reasoning, because a Durable Object has no natural open either.
