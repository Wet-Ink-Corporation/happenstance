---
item: HS-S0061
stage: discover
created: 2026-08-12T13:02:29.401Z
updated: 2026-08-12T13:02:29.401Z
template_sig: 86ce4036
rendered_sig: 304d825a
---

# Discover — Migration 1, the live Postgres fixture, and the CI job that runs it

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one line: migration 1, a `testcontainers`-backed `PostgresFixture` + `ConcurrentFixture` handing out `CONTENDERS = 8` handles onto **one** backing store, in-crate gating that keeps `cargo test --workspace --all-features` green with no server, and the live-Postgres CI job beside `gate` | `_storymap.md`, *Slices* table, `postgres-schema-and-live-fixture` row | Four deliverables in one story because a fixture no macro is invoked with, in a job no workflow runs, is not delivered |
| **AC-011** — the default gate stays Docker-free and network-free | `project.md`, *Acceptance criteria*, AC-011 | This story owns the Postgres third of AC-011. The Neon job is `neon-fixture-and-live-job`'s and the wasm32 build that must not regress is `neon-sql-transport`'s (`_storymap.md`, *Coverage*, AC-011 row) |
| No inbound `depends_on` | manifest (`dependsOn: []`); `_storymap.md`, *Merge order* item 2 | It is the substrate every other Postgres story consumes, and it can start the moment the branch exists |
| `MemoryFixture` is the reference to copy structurally | `crates/happenstance-testkit/src/fixtures.rs:243`, `:270`, `:273`, `:280`, `:286` | `connect` hands out a handle owning a refcount rather than borrowing the fixture's lifetime, which is what keeps `Fixture::Store` an ordinary associated type instead of a GAT |
| The GAT shape is one of five ingredients of a rustc ICE that still reproduces on 1.97.1 | `crates/happenstance-testkit/src/contract.rs:97-111` | Not a stylistic preference. The refcount-handle shape is load-bearing and must be copied, not improved on |
| `connect` **panics** rather than returning `Result`, deliberately | `crates/happenstance-testkit/src/contract.rs:316-321` | A live-infrastructure fixture must keep "the database is down" out of the same channel as "the adapter is wrong". The panic message is the interface |
| `ConcurrentFixture` is written `Fixture<Store: Send>`, not a `where` clause, and the difference is not cosmetic | `crates/happenstance-testkit/src/concurrency.rs:182-186` | An opaque `impl Fixture` return type carries only the bounds written on it, so `F::Store: Send` would be unprovable at the call site. Either one type implements both traits, or two named types do |
| `CONTENDERS = 8` | `crates/happenstance-testkit/src/concurrency.rs:206` | The literal number of handles onto one backing store the concurrency fixture must supply. `postgres-concurrency-family` consumes it |
| `MID_BATCH_FAULT` defaults to declined; the ceilings are `Option<usize>`, not `Capability` | `crates/happenstance-testkit/src/contract.rs:207`, `:253`, `:262`, `:279` | Postgres can genuinely offer a mid-batch fault (a trigger, an armed `CHECK`). A silent default on the store that could have co-operated is the outcome to avoid |
| `Capability::declined("")` is rejected by an `assert!` in a `const fn`, which fires at **codegen** | `crates/happenstance-testkit/src/contract.rs:404-412` | `cargo build` and `cargo test` catch it; `cargo clippy` does not. A green clippy proves nothing about this story's constants |
| Migration 1 already owes phase 5's identity and time columns, and phase 4 is done | `RUNBOOK.md:249-251` | `EventId` and `recorded_at` are columns in migration 1 of *every* store. Not re-derived here, and not optional |
| `sqlx` currently ships with no `migrate` and no `macros` feature, both with stated reasons | `crates/happenstance-postgres/Cargo.toml` (`[dependencies]` comment) | Turning either on is a deliberate manifest decision, not an incidental one. `macros` needs a live `DATABASE_URL` this crate does not have |
| The testkit dev-dependency does **not** enable `proptest` today | `crates/happenstance-postgres/Cargo.toml` `[dev-dependencies]`; `crates/happenstance-testkit/Cargo.toml` `[features]` (`default = []`) | `event_store_model_conformance!` will not compile until the dev-dependency asks for the feature. A manifest fact `postgres-append-and-frontier-head` (HS-S0062) depends on and this story is the right place to fix |
| `testcontainers` is not in the tree today, and the live suites must not enter the `REQUIRED` array | `_decomposition.md`, *Deployment brief* → *Notes* → "CI implication, concretely"; `RUNBOOK.md:4356` | A pinned Postgres minor in a job of its own, sibling to `gate` / `backlog` / `wasm-conformance` / `msrv` / `semver` / `advisories` (`.github/workflows/ci.yml:31`, `:102`, `:204`, `:241`, `:279`, `:325`) |
| **DR-5**: the gating must be whole-invocation, never a `#[cfg]` hiding one rule out of a macro's expansion | `project.md`, *Derived requirements*, DR-5; `_decomposition.md`, *Architecture brief* §7 | The one hard constraint on the mechanism. `#[ignore]`, `required-features` and an env read are all admissible; making a rule vanish is not |
| **DR-9**: the default gate must stay runnable with no Docker and no network | `project.md`, *Derived requirements*, DR-9 | `cargo xtask affected` and `cargo xtask ci --fast` are the story and integration grains for *every other project* in the initiative (`.redkiln/config.yaml`, `verify:`) |

## Questions

**Answered.**

1. *One fixture type or two?* One type implementing both `Fixture` and
   `concurrency::ConcurrentFixture`, or two named types — never a function returning
   `impl Fixture`, because `crates/happenstance-testkit/src/concurrency.rs:182-186`
   spells out that the opaque type would drop the `Store: Send` bound at the call
   site. `spec` picks between the two admissible shapes; the third is closed.
2. *Which column does migration 1 carry for the visibility mechanism?* The one
   ADR-0013 already established as affordable (`xid8`), authored here, with
   ADR-0024 owning the **choice** rather than the schema
   (`.kb/decisions/0013-position-assignment-and-visibility.md`;
   `_storymap.md`, *What each story is*, `postgres-append-and-frontier-head`).
   If ADR-0024 chooses otherwise, migration 1 is edited, not migrated: nothing has
   ever shipped a schema to a real consumer, so there is no backfill
   (`_decomposition.md`, *Deployment brief* → *Notes* → "Migration and backfill").
3. *Does the model family need a manifest change?* Yes — the testkit's `proptest`
   feature is off by default and the Postgres dev-dependency does not ask for it.
   Recorded above; `spec` states it as a manifest deliverable of this story so
   HS-S0062 does not discover it as a compile error.

**Deferred to `spec`.**

4. *Which gating mechanism* — `#[ignore]`, `required-features`, or an env read
   inside the test binary. All three satisfy DR-9; DR-5 rules out only the fourth.
   Deciding it needs the CI job's shape in hand, which is `spec`'s.
5. *Schema-per-fixture-instance or database-per-fixture-instance* for isolation, and
   *whether `sqlx`'s `migrate` feature earns its place*
   (`_decomposition.md`, *Architecture brief* §9.3).
6. *Whether `MID_BATCH_FAULT` is offered, and `REOPEN`'s honest answer.* Both stores
   are durable, so both can plausibly support `REOPEN`; durability's far end is
   `sqlite-durable-store`'s, so this project owes an honest answer and not an
   investment (`_decomposition.md`, *Architecture brief* §6).

**Deferred to the owning stories.**

7. *How the adapter buys ES-10's visibility invariant when `nextval()` allocates
   outside the transaction.* ADR-0024's, owned by
   `adr-0024-position-visibility-mechanism` (HS-S0065). This story authors a column
   and a fixture; it does not decide a mechanism, and `spec` must not let the schema
   become an unrecorded decision.
8. *Whether `conflicting_position` is a promise or a hint.*
   `neon-conflicting-position-verdict` (HS-S0070). ES-25 already answers it as a
   hint (`spec/SPECIFICATION.md:3746-3748`); what is open is the ledger's
   assumption about Neon, and nothing here touches it.

## Decision

The problem this slice solves is that `happenstance-postgres` has no way to be run
against anything: there is no schema, no fixture, no test target and no job, so
every later Postgres story would otherwise have to invent its own substrate and
they would disagree. This story builds the substrate once — migration 1 carrying
phase 5's identity and time columns plus the visibility mechanism's column, a
fixture whose `connect()` hands out refcount-owning handles onto one backing store
in the shape `MemoryFixture` established, the same fixture (or its named sibling)
satisfying `ConcurrentFixture` at `CONTENDERS = 8`, whole-invocation gating that
leaves `cargo test --workspace --all-features` green on a machine with no Docker,
and a `testcontainers` job pinned to one Postgres minor beside `gate`. The spec
will cover: the migration's columns and indexes; the fixture's `Store` type and
`connect` panic message; deliberate answers for `SECOND_HANDLE`, `REOPEN`,
`MID_BATCH_FAULT` and the three `Option<usize>` ceilings; the gating mechanism and
why it is whole-invocation; the workflow job, its image pin and its trigger; and the
`proptest` dev-dependency feature the model family needs. No `[FROZEN]` clause is
changed here, so no ADR is owed by this story.

## The wrong implementation

**`PostgresFixture` backed by a pool of one.** Build the fixture on a `PgPool` with
`max_connections(1)` — or, worse, on one `PgConnection` cloned behind a mutex —
because it makes the container start faster and the tests deterministic. Every
existing check passes, and passes *more* cleanly than the right implementation:
`event_store_conformance!` is green, `event_store_model_conformance!` is green, and
`event_store_concurrency_conformance!` at `CONTENDERS = 8` is green too, because
eight handles queueing on one connection satisfy every word of
`concurrency::ConcurrentFixture` — its whole requirement is `F::Store: Send` and
eight handles onto one backing store (`crates/happenstance-testkit/src/concurrency.rs:182-186`,
`:206`), and nothing in it asks whether those handles can execute *at the same
time*. The result is `MemoryEventStore` with network latency
(`crates/happenstance-postgres/src/event_store.rs:48-53`): the fixture deletes the
axis the adapter exists to occupy, downstream `postgres-concurrency-family` reports
AC-003 satisfied, and ADR-0024 records a measurement taken against a serialised
store. This is the sharpest mutant in the project because the *adapter* is correct
and the *instrument* is the lie.

It cannot live in `crates/happenstance-testkit/tests/` — the testkit may not depend
on an adapter (CLAUDE.md's dependency rule; `_decomposition.md`, *Architecture
brief* §9.1) and a live server may not enter `cargo test --workspace` (DR-9). Its
home is `crates/happenstance-postgres/tests/`, inside the live job, as a witness
assertion the fixture must carry with it: that `CONTENDERS` handles are genuinely
concurrent — pool capacity observed at or above `CONTENDERS`, and overlapping
in-flight appends observed rather than assumed. Without that witness the concurrency
family is decorative against this fixture, which is exactly the failure CLAUDE.md's
"a rule that no adapter can fail is decorative" warns about, one level down.

**The second mutant is the gating.** Wrap individual rules in
`#[cfg(feature = "live")]` rather than gating the whole macro invocation. The live
job prints forty green rules and no line of output says the other twelve were never
compiled. DR-5 forbids it, and nothing in the tree would catch it: the testkit's
`capability_skips_are_reported` meta-test reads *capabilities*
(`crates/happenstance-testkit/src/suite.rs:58`,
`crates/happenstance-testkit/src/contract.rs:521`), and a rule removed by `#[cfg]`
declines nothing — it simply is not there.

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
