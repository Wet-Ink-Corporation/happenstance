---
item: HS-P0014
stage: briefs
---

# Briefs — The two stores that disagree with the port

Companion to [`project.md`](project.md), [`_grounding.md`](_grounding.md) and
[`_intake-brief.md`](_intake-brief.md). Per `../_decomposition.md` *Warranted
briefs*, this project carries **three**: `architecture`, `testing` and
`deployment`. They live in this one file, one `##` section each. No `ux` section
belongs here — there is no surface to perceive, and `design.capture` is
deliberately absent from `.redkiln/config.yaml`.

Where this file and `spec/SPECIFICATION.md` disagree, the clause wins. Where it
and an Accepted atom under `.kb/decisions/` disagree, the atom wins.

## Architecture brief

### Intent

Scopes **HS-P0014** only: turning `crates/happenstance-postgres/` and
`crates/happenstance-neon/` from skeletons — real associated types over
`todo!()` bodies, `publish = false`, a scoped `#![allow(clippy::todo)]` naming
this phase (`crates/happenstance-postgres/src/lib.rs:58-63`,
`crates/happenstance-neon/src/lib.rs:101-105`) — into two adapters that have
**run** `happenstance-testkit` and passed it, or whose failures are recorded as
declined capabilities or as an accepted decision amending a clause.

This brief decides **seams, not bodies**. It names the composition roots each
capability must mount into, the sibling contracts each one wires to, the
Accepted decisions that bind, and the four places where the existing gate will
reject the work if the wiring is skipped. It writes no production code, invents
no API, and does not choose the ES-10 mechanism — **that choice is AC-001's, it
belongs in ADR-0024, and ADR-0024 does not exist yet.** Nothing below may be
cited as if it did.

Two structural facts set everything else. First, this is deliberately **one
project, not two** (`project.md` *Coupling notes*): Neon is Postgres over
one-shot HTTP and inherits migration 1, the tag storage and the append-condition
SQL, so the seam between them is vertical (two adapters, each complete) and not
horizontal (schema below, transport above). Second, the crates are *instruments*
before they are targets (`crates/happenstance-postgres/src/lib.rs:10-26`) — a
mechanism that makes Postgres pass by serialising its writers has not
implemented the adapter, it has deleted the axis, and AC-003 exists to catch
exactly that.

### Acceptance Criteria

One row per `project.md` AC-###, naming the **seam it mounts into** and the real
sibling contract it wires to. A capability that lands anywhere else — a helper
module nothing calls, a fixture no macro is invoked with, a CI step no workflow
runs — is not delivered.

| AC | Seam it mounts into | The contract it wires to |
| --- | --- | --- |
| **AC-001** ADR-0024 accepted | `.kb/_intake/` → `/redkiln:kb-ingest` → a new atom under `.kb/decisions/`, plus its row in `.kb/maps/decision-map.md` | KbFrontmatter (`src/schema/kb.ts`); `redkiln validate --kb`. The full record goes to `references/adr/` and the atom links it — CLAUDE.md's two-places rule. **Not hand-authored**: hand-writing atoms is what `0269720` reverted. |
| **AC-002** CF-13 green, and hard-won | `impl SendEventStore for PostgresEventStore` — `append` and `head` at `crates/happenstance-postgres/src/event_store.rs:136-161` | `happenstance_testkit::rules::nothing_below_an_observed_position_appears_later` (`crates/happenstance-testkit/src/suite.rs:5880`), reached through `event_store_conformance!`. Plus the **poll-padding decorator over `PreCommitPositionStore`** that `spec/SPECIFICATION.md:2846-2850` names and owes to *this phase* — see Notes §9. |
| **AC-003** concurrency green, writers unserialised | `crates/happenstance-postgres/tests/` invoking `event_store_concurrency_conformance!(PostgresFixture::…)` | `concurrency::ConcurrentFixture`, whose bound is `F::Store: EventStore + Send` (`crates/happenstance-testkit/src/concurrency.rs:1104-1165`), at `concurrency::CONTENDERS = 8` (`:206`), under `tokio` `rt-multi-thread` — already in the crate's dev-deps (`crates/happenstance-postgres/Cargo.toml`). |
| **AC-004** whole suite + mutant control | `crates/happenstance-postgres/tests/` invoking `event_store_conformance!` and `event_store_model_conformance!` | `Fixture` (`crates/happenstance-testkit/src/contract.rs:120-353`); the model family is behind the testkit's `proptest` feature and `not(target_arch="wasm32")` (`crates/happenstance-testkit/src/lib.rs:181-183`) — the dev-dependency must enable it. **The mutant-harness half needs a seam decision: see Notes §9.1.** |
| **AC-005** `PostgresProjectionStore` | `crates/happenstance-postgres/src/projection_store.rs` — `type Batch<'a> = sqlx::Transaction<'static, Postgres>` (`:26-45`) | The `ProjectionStore` port and its suite as frozen by `projection-store-freeze` (HS-P0010). **Blocked**: that project reads `stage: storymap`, `status: planning` today. Sequence last. |
| **AC-006** Neon runs the suite over HTTP | `crates/happenstance-neon/tests/` invoking `event_store_conformance!(NeonFixture::…)`, over a **real** `SqlTransport` impl | `SqlTransport` (`crates/happenstance-neon/src/transport.rs:241-265`). No real impl exists — `NullTransport` fails every round trip (`:267-298`). Building one is unbudgeted scope; see Notes §9.2. The crate has **no `[dev-dependencies]` at all** today. |
| **AC-007** nothing skips silently | `impl Fixture for NeonFixture`'s `Capability` constants | `Capability::declined(reason)` (`crates/happenstance-testkit/src/contract.rs:374-419`). A declined capability still emits a test returning `RuleOutcome::Skipped` (`:26-42`); `#[cfg]`-ing a rule out is the forbidden move. `SECOND_HANDLE` is a **MUST** and its rule panics on a decline (`:135-161`). |
| **AC-008** `conflicting_position` settled | `NeonEventStore::conditional_append_request` + `decode_append_response` (`crates/happenstance-neon/src/event_store.rs:159-165`, `:249-255`) | The CTE already written out in full at `crates/happenstance-neon/src/lib.rs:54-68` and `event_store.rs:134-148`, plus `IsolationLevel::Serializable` as `NeonConfig`'s default (`transport.rs:56-85`). This AC is **verification of an in-tree answer**, not an open design question — see Notes §3, tension 3. |
| **AC-009** the structural bill is written | The Postgres crate's rustdoc (`lib.rs`, `event_store.rs`) **and** the decision recording whether a clause is owed instead | `spec/SPECIFICATION.md:2838-2844` already states the frontier/no-RYOW/cluster-staleness consequences under ES-10 for the arm-C shape. AC-009 records whether that is *sufficient* or a clause of its own is owed — as a decision, not a default. |
| **AC-010** both stores declare real limits | `Fixture::MAX_EVENT_DATA_LEN` / `MAX_TAGS_PER_EVENT` / `MAX_EVENTS_PER_BATCH` on both fixtures | `Option<usize>`, **not** `Capability` (`contract.rs:213-279`) — a ceiling is a fact, a decline is a trade. Neon's is anchored by `MAX_RESPONSE_BYTES` (`transport.rs:45-54`), halved in effect by hex `bytea` rendering (`transport.rs:87-102`). Refusals go through `AppendError::ExceedsStoreLimit { limit: StoreLimit::… }`, never `AppendError::Store` (`contract.rs:237-247`). |
| **AC-011** default gate stays Docker-free | `xtask/src/main.rs`'s `REQUIRED` step array + `.github/workflows/ci.yml` jobs | Two **new** workflow jobs beside `gate` / `wasm-conformance` / `msrv` / `semver` / `advisories` (`ci.yml:31,204,241,279,325`). The Neon wasm32 step (`xtask/src/main.rs:264-282`) and its name-selected entry in `wasm_steps()` (`:788-800`) must not regress. |
| **AC-012** neither crate is a skeleton | Both manifests, both `lib.rs` attribute lines, **and `xtask/src/package.rs`** | `PUBLISHABLE` + `reconcile` (`xtask/src/package.rs:94,172-212`) fails in **both** directions. Deleting `publish = false` without adding both names and copying `LICENSE-MIT`, `LICENSE-APACHE`, `README.md` into each crate directory fails the gate — neither directory holds any of the three today. `cargo xtask reserve` already knows both names (`xtask/src/reserve.rs:99,105`). |
| **AC-013** discharge recorded for the audit | A written record `publication-and-positioning` (HS-P0016) can read | `spec/SPECIFICATION.md`'s clause rows for ES-10/11/12 (`:8592-8594`), ES-41/42 (`:8623-8624`), VT-21–24 (`:8547-8550`). Any marker change is re-generated by `cargo xtask spec-trace --write` over §7.1–§7.2, never hand-edited. |

### Notes

#### 1. The seams this project touches, and the ones it must not

Per CLAUDE.md's repository map and its dependency rule — everything depends on
`happenstance-core`; `happenstance-core` depends on nothing here; **no adapter
depends on another adapter**.

**Owned outright (bodies change):**

- `crates/happenstance-postgres/src/` — `lib.rs`, `error.rs`, `event_store.rs`,
  `read_stream.rs`, `projection_store.rs`, plus new `tests/` and a migration
  source.
- `crates/happenstance-neon/src/` — `lib.rs`, `config.rs`, `error.rs`,
  `transport.rs`, `wire.rs`, `event_store.rs`, `projection_store.rs`, plus new
  `tests/` and a real transport module.

**Wired into (additive, must not regress):**

- `xtask/src/main.rs` (the `REQUIRED` step array; `wasm_steps()`), `xtask/src/package.rs`
  (`PUBLISHABLE`), `xtask/src/proof.rs` (`ARTEFACTS`), `.github/workflows/ci.yml`.
- `.kb/decisions/`, `.kb/maps/decision-map.md`, `references/adr/`, `spec/SPECIFICATION.md`.
- `references/adapter-shapes.md` and `RUNBOOK.md`'s phase-10 session log (DoD 6).

**Must not change:**

- `crates/happenstance-core/**`. Every clause this project touches is `[FROZEN]`
  or `[PROVISIONAL]` in the contract crate, and a `[FROZEN]` clause changes by
  ADR, never by edit (CLAUDE.md). If Neon cannot pass a rule, the *clause* gives
  — by a second accepted decision atom with the suite re-run, which is exactly
  what initiative DoD 6 contemplates.
- `crates/happenstance-testkit/src/**` — with one caveat at §9.1. If a rule
  seems wrong, fix the rule and explain why in the same change; do not skip it.
- `crates/happenstance-sqlite/`, `-cloudflare/`, `-ladybug/`, `-sync/` — disjoint,
  and their projects parallelise freely (`RUNBOOK.md:239-242`).

#### 2. Composition roots — where each capability must mount

There is no application here to render into, so the "composition root" is
whichever *already-existing* call site makes the code run in anger. Four of
them, and every capability lands in one:

**Root A — the port impls.** `impl SendEventStore for PostgresEventStore`
(`crates/happenstance-postgres/src/event_store.rs:121-175`) and
`impl EventStore for NeonEventStore<T>`
(`crates/happenstance-neon/src/event_store.rs:168-230`). Not a helper module, not
a free function beside them: the four methods are the surface, and a
`todo!()` body type-checks against any signature (`RUNBOOK.md:3061-3068`), so
"it compiles" counts for nothing here.

Note which flavour each mounts, because it is compiled fact and not preference.
Postgres implements **`SendEventStore`** and gets `EventStore` free
(`event_store.rs:118-121`). Neon implements only the **bare `EventStore`**, on
both targets; adding a second `SendEventStore` impl is `error[E0119]` against
`trait_variant`'s blanket impl, so the flavours are mutually exclusive per type
(`crates/happenstance-neon/src/lib.rs:79-89`,
`crates/happenstance-neon/src/event_store.rs:4-34`). ADR-0001 and CLAUDE.md
constraint 4 are what that rests on; nothing here relaxes either.

**Root B — the conformance macros.** `event_store_conformance!(<fixture expr>)`
(`crates/happenstance-testkit/src/lib.rs:311-357`) invoked from each adapter's
own `tests/`. Neither crate has a `tests/` directory today. The macro takes an
**expression building a `Fixture`**, and its own docs say why: "a Postgres
fixture needs a connection URL, and a type with an argument-less constructor
would have to reach into the environment for it" (`lib.rs:270-274`). The
adapter's fixture is therefore the integration point, and it is the piece that
decides whether the run is real.

Three families, three invocations (`crates/happenstance-testkit/src/lib.rs:91-110`):
`event_store_conformance!` (always), `event_store_model_conformance!` (needs the
testkit's `proptest` feature) and `event_store_concurrency_conformance!`
(opt-in, `F::Store: EventStore + Send` — **Postgres only**; Neon's bare-flavour
store cannot invoke it and is not expected to).

**Root C — the gate.** `xtask/src/main.rs`'s `REQUIRED` array is the single
definition of what CI runs (CLAUDE.md *Commands*). Two facts an implementer
trips over:

- `wasm_steps()` selects steps **by name**, and panics on a miss, precisely so
  that inserting a step cannot silently repoint `cargo xtask wasm`
  (`xtask/src/main.rs:788-800`). Renaming "wasm32 build of the Neon adapter" is
  a gate change, not a cosmetic one.
- `xtask/src/package.rs`'s `reconcile` compares the hand list `PUBLISHABLE`
  against what `cargo metadata` says Cargo *will* publish, and fails on drift in
  **either** direction (`:172-212`). AC-012's `publish = false` deletion is
  therefore a three-part change: manifest, `PUBLISHABLE`, and the three
  `REQUIRED_FILES` copied into each crate directory (`:94`).

**Root D — the knowledge base.** ADR-0024 (and any DoD-6 amendment record)
reaches `.kb/decisions/` through `.kb/_intake/` and `/redkiln:kb-ingest`, never
by hand. Accepted atoms are immutable and `redkiln validate --kb` checks them
against `HEAD`; a correction is a new superseding atom. The long-form record
lives in `references/adr/` and the atom links it — the split is deliberate and
`spec-trace` cites the long form by line.

#### 3. Accepted decisions that bind, and where they pull

| Atom | What it fixes here | Tension |
| --- | --- | --- |
| `.kb/decisions/0013-position-assignment-and-visibility.md` | The invariant is **global**; `head()` is a **frontier**, not `max(position)`; read-your-own-writes does **not** hold; staleness is bounded by the longest open write transaction *anywhere in the cluster*. Arm C measured 0.987–1.026×. | The mechanism is *established as affordable*, not *chosen for the adapter* — ADR-0013 says so in terms. ADR-0024 owns the choice. And the global premise rests on the projection checkpoint being global, which is **unsettled** and owned by HS-P0010 (`.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`). ADR-0024 inherits that risk without owning it: say so in the record. |
| `.kb/decisions/0001-async-port-flavours.md` | Never `#[async_trait]`. `read` returns the stream at the top level and is not `async`. | Binding on both crates and already honoured by both read paths. `PgReadStream` exists *because* `read` cannot await (`read_stream.rs:1-77`). |
| `.kb/decisions/0009-error-send-sync.md` | Two independent `#[non_exhaustive]` `thiserror` enums, one per port; no forced `Send`; **no `ConditionViolated` variant on either** (`crates/happenstance-postgres/src/error.rs:1-58`). | None. Extend the enums for the mechanism's new decode failures rather than flattening `sqlx::Error::Database` per-`SQLSTATE` (`error.rs:31-39`). |
| `.kb/decisions/0011-read-laziness-and-isolation.md` | ES-11 (one read, one snapshot) and ES-12 (all `Query` items share it); ES-42's no-`Unpin` return. | Postgres buys ES-11 from `BEGIN REPEATABLE READ` + `DECLARE CURSOR` (`read_stream.rs:63-77`); keyset pagination is named and rejected. Neon buys it for free — one round trip *is* one snapshot. Both are PROVISIONAL clauses AC-013 must report on. |
| `.kb/decisions/0012-append-shape-and-preconditions.md` | `append` keeps its borrowed batch; the empty-batch refusal precedes condition evaluation. | Phase 4 explicitly declined to measure position-allocation cost against a live adapter. That measurement is this project's. |
| `.kb/decisions/0015-validated-identifiers-and-store-limits.md` | VT-21–VT-24's guaranteed minima, and where CF-40 was minted. | CF-40's ownership is *contested* between ADR-0012 and ADR-0015 (`.kb/open-questions/cf-40-fixture-limits-ownership.md`). DR-6: this project **consumes** whatever a sibling settles; it does not settle it unilaterally. |

Three further tensions, stated so no story quietly resolves one:

1. **ADR-0024 is a deliverable, not prior art.** No `0024-*` file exists under
   `.kb/decisions/`. Any brief, spec or story citing it is citing something
   AC-001 must write.
2. **AC-005 is blocked.** `.bklg/from-contract-to-published-library/projection-store-freeze/project.md`
   is at `stage: storymap`. `PostgresProjectionStore` cannot be written against
   a `Batch` shape that is not frozen, and writing it early and reworking it is
   the failure `project.md`'s risk table names.
3. **`conflicting_position` is already known-wrong in the ledger.** The Neon
   crate's own doc says the collapse *keeps* it, "contrary to the standing
   assumption in the decision ledger" (`crates/happenstance-neon/src/lib.rs:46-77`).
   AC-008 is a live-endpoint verification plus a ledger correction — not the
   discovery of a new limitation. Plan it as such.

#### 4. Postgres — contracts and data flow

`PostgresEventStore` holds a `PgPool`, not a connection, and that is the shape
difference that makes it an instrument: readers and writers do not queue
(`event_store.rs:89-116`). Four flows:

**`append`** (`:136-144`, `todo!()`). One transaction. The append condition, the
insert, and — under whichever mechanism ADR-0024 chooses — the visibility
machinery. `event_store.rs:43-75` already works through what each candidate
costs the *types* and the *append path*: a serialised sequence table leaves the
signature untouched and serialises every writer on one row (and so fails AC-003
by construction); a transaction-scoped advisory lock leaves the signature
untouched but, in its only interesting form, needs a lock key derived from an
`AppendCondition` — the first thing that pushes back toward the port; `xid8` +
`pg_snapshot_xmin` leaves writers entirely unserialised and moves the whole cost
to the read side. `position` is deliberately **not** `bigserial` (`:22-24`).

**`read`** (`:124-134`) is the one method whose body is already real, and it must
stay lazy: `read` is not `async`, nothing is acquired until the first poll, and
`PgReadStream` holds the transaction's **owner** rather than a borrowed stream
(`read_stream.rs:37-60`). Under a frontier mechanism the visibility predicate
composes *into* the cursor's `DECLARE`, inside the same `REPEATABLE READ`
transaction — which is what keeps `pg_current_snapshot()` stable across every
`FETCH` instead of moving under the caller. That composition is the read-side
half of AC-002 and is easy to get wrong by evaluating the frontier per chunk.

**`head`** (`:146-161`, `todo!()`) must return the **visibility frontier**. Under
either lock it collapses to `max(position)`; under `xid8` it is
`max(position) WHERE xid < pg_snapshot_xmin(pg_current_snapshot())` and trails
the maximum. ES-30's `head_is_the_highest_visible_position` asserts a *bound*
rather than an equality precisely to admit this (`spec/SPECIFICATION.md:2838-2842`).

**`contains_event_id`** (`:163-174`, `todo!()`) needs `origin_store` and
`origin_position` columns the intended schema does not have, and it inherits a
question rather than only a blockage: under a frontier mechanism, answering
`true` for a committed row above the frontier makes this method disagree with
`read`; answering `false` makes ingest re-accept an event the store holds. That
is a replication question. ES-41 is `[PROVISIONAL]` (`spec/SPECIFICATION.md:4381`,
row at `:8623`) and `replication-identity-and-ingest` (HS-P0017) is downstream;
record the answer, do not settle replication semantics here.

**Migration 1** carries phase 5's identity and time columns (`EventId`,
`recorded_at`) — already discharged and named so nobody re-derives it
(`RUNBOOK.md:249-253`) — plus whatever column ADR-0024's mechanism adds. `sqlx`
is currently configured with no `migrate` and no `macros` feature
(`crates/happenstance-postgres/Cargo.toml`), both with stated reasons; changing
either is a manifest decision to make deliberately, not incidentally.

#### 5. Neon — contracts and data flow

Neon inherits the Postgres schema and adds one constraint that reshapes
everything above it: the `/sql` endpoint has no connection, no transaction
handle, no cursor, exactly one round trip per operation, and a hard 64 MiB
response cap (`crates/happenstance-neon/src/lib.rs:18-24`). The only thing it
offers beyond a single statement is a **non-interactive** batch — statements run
server-side inside one `BEGIN`/`COMMIT` at a chosen isolation level, atomic and
unable to branch (`transport.rs:29-35`).

**`append`** is already wired end to end and awaits three `todo!()`s:
`conditional_append_request` builds the one-statement CTE (`event_store.rs:159-165`),
`decode_append_response` turns the single row into `AppendOutcome::Appended` or
`::Conflict` (`:249-255`), and the impl already maps `Conflict` to
`AppendError::ConditionViolated(ConditionViolated::at(position))` and refuses the
empty batch first (`:183-206`). The CTE is quoted in full in two places and needs
`IsolationLevel::Serializable`, which is why `NeonConfig` defaults there rather
than to Postgres' `ReadCommitted` (`transport.rs:56-85`).

**`read`** is the honest part: `NeonReadStream` is a *shape*, not a capability —
one round trip, then a buffer drained (`event_store.rs:265-386`). The one port
property it genuinely honours is laziness, which is load-bearing on `wasm32`
where issuing a `fetch` outside a polled future happens off the runtime's event
loop (`:277-282`). `decode_read_response` (`:257-263`) is the remaining `todo!()`.

**`ProbeThenWriteStore`** (`:388-503`) stays. It is the named wrong
implementation CLAUDE.md's conformance section asks every rule to have: it
type-checks against `EventStore` perfectly and is silently unsound, because two
statements are two implicit transactions with a network-latency-wide window
between them. Do not delete it, do not fix it, and do not leave it merely
documented — see §9.1 for where it must be *run*.

**The transport is missing.** `SqlTransport` is a one-method trait and
`NullTransport` fails every round trip (`transport.rs:241-298`). AC-006's "real
Neon `/sql` endpoint" requires a real implementation, and the crate documents
honestly why it does not own one: a host client drags in a TLS stack and
`wasm32-unknown-unknown` has none, so they are two different clients
(`transport.rs:1-15`, `lib.rs:91-99`). See §9.2.

#### 6. The two fixtures — the real sibling contracts

`MemoryFixture` (`crates/happenstance-testkit/src/fixtures.rs:230-280`) is the
reference to read first. Copy two things from it: `connect` hands out a handle
that **owns a refcount** rather than borrowing a lifetime from the fixture —
which is what lets `Fixture::Store` be an ordinary associated type instead of a
GAT, and the GAT shape is one of the five ingredients of a rustc ICE that still
reproduces on 1.97.1 (`contract.rs:97-111`) — and `REOPEN` is declined **with
the real reason**.

Both new fixtures must answer, deliberately:

- **`SECOND_HANDLE`** — a MUST (`contract.rs:135-161`). Postgres: a second pool
  checkout, or a second pool onto the same schema. Neon: a second client against
  the same branch. A decline here makes
  `two_handles_observe_each_others_appends` **panic**, quoting the fixture's own
  words, rather than skip.
- **`REOPEN`** — a SHOULD. Both stores are durable, so both can plausibly
  support it; if a fixture declines, the reason must be real, not convenient.
  Durability's far end is `sqlite-durable-store`'s, so this project owes an
  honest answer and not an investment.
- **`MID_BATCH_FAULT`** — defaults to declined (`contract.rs:207-211`). Postgres
  can genuinely offer it (a trigger raising on the third insert; a `CHECK`
  armed for one write). Neon, over one non-interactive statement, plausibly
  cannot. Either answer is fine; a *silent* default on the store that could have
  co-operated is the one to avoid.
- **The three ceilings** (`contract.rs:213-279`) — `Option<usize>`, not
  `Capability`. Stating a number commits the store: the rule appends exactly
  that many bytes and requires acceptance, then one more and requires
  `AppendError::ExceedsStoreLimit { limit: StoreLimit::EventDataLen, .. }`. A
  number that is not where the real ceiling sits fails in one direction or the
  other. A store MAY state a ceiling below the VT-21 floor and will then
  correctly fail `store_accepts_the_guaranteed_minimum_payload` — the two rules
  are independent and both are owed an answer.

Two fixture mechanics that bite:

- `connect` **panics** rather than returning `Result`, deliberately: a fixture
  that cannot connect is a broken test environment, not a non-conformant
  adapter, and a `Result` would put "the database is down" into the same channel
  as "the adapter is wrong" (`contract.rs:309-321`). Live-infrastructure
  fixtures must keep that distinction visible in the panic message.
- `Capability::declined("")` is rejected by an `assert!` in a `const fn` — but
  for an **associated** const that fires at *codegen*, so `cargo build` and
  `cargo test` catch it and `cargo clippy` does not (`contract.rs:404-410`).
  Do not conclude from a green `clippy` that the constants are well-formed.

The concurrency family needs `concurrency::ConcurrentFixture`, not `Fixture` —
an opaque return type carries only the bounds written on it, so
`impl Fixture` would leave every rule's `F::Store: Send` unprovable at the call
site (`concurrency.rs:1138-1146`). One fixture instance must hand out
`CONTENDERS = 8` handles onto one backing store.

#### 7. Gate and CI wiring

The default gate must stay runnable with **no Docker and no network** (DR-9,
AC-011). That constrains where the live suites mount:

- **Not** in `xtask/src/main.rs`'s `REQUIRED` array. Everything there runs on
  every `cargo xtask ci`, and a step that needs a container or a credential
  breaks `cargo xtask affected` and `cargo xtask ci --fast` for every other
  project in the initiative.
- **Yes** in two new `.github/workflows/ci.yml` jobs, siblings of `gate`,
  `wasm-conformance`, `msrv`, `semver` and `advisories`. `testcontainers` pinned
  to a specific Postgres minor for one; a real Neon endpoint and its credential
  for the other. The deployment brief owns how the credential is reached without
  putting it in the default gate.
- The tests themselves should therefore be **ignored-by-default or
  env-gated** in-crate, so that `cargo test --workspace --all-features` on a
  developer machine with no server still exits zero. Which mechanism — `#[ignore]`,
  a `required-features` gate, or a `cfg` on an env var — is the implementer's
  call, with one hard constraint from DR-5: **it may not be `#[cfg]`-ing a
  conformance rule out of the expansion.** Gating the whole invocation on
  infrastructure availability is a different act from making one rule vanish,
  and only the second is forbidden.

Also in the gate's blast radius: `cargo xtask lint-position-literals` (CF-6 — no
literal position values, and this is the project where gaps stop being
hypothetical); `cargo xtask spec-trace`, whose §7.1–§7.2 region is generated and
must be rewritten with `--write` if a marker moves; `cargo deny check`, whose
allowlist is MIT / Apache-2.0 / BSD-2 / BSD-3 / ISC / Unicode-3.0 / Zlib
(`deny.toml:8-20`) with `[graph] all-features = true`; and `xtask/src/proof.rs`'s
`ARTEFACTS`, which asserts each phase's proof artefact still holds the tests its
clauses name.

#### 8. Sequencing

The story map should respect four edges. Nothing below is a story split — that
is `_storymap.md`'s call, and `project.md` already flags this as the largest
non-trunk item (~11 runbook-days) and the most reasonable split candidate.

1. **Name claims first** (`cargo xtask reserve`), per phase 0's rule that a name
   is reserved when its phase starts.
2. **The Postgres event store, then ADR-0024.** The ADR owes a *number*
   (DR-3), and the number includes behaviour with a long-running transaction
   deliberately held open on the same database. That measurement needs a real
   adapter, not the phase-2 SQL harness — which is the whole content of
   `.kb/open-questions/postgres-arm-c-structural-cost.md`: the experiment
   "measured four SQL strategies, not four implementations of `SendEventStore`",
   with no pooling, no transaction lifetime tied to a trait method, no cursor
   and no error mapping. Expect the ADR to land *after* a working append path,
   not before it.
3. **Neon after the Postgres schema settles**, since it inherits migration 1,
   the tag storage and the append-condition SQL — but the transport work
   (§9.2) has no such dependency and can start immediately.
4. **AC-005 last**, gated on HS-P0010.

#### 9. Left to the implementer, and the two seams that need a decision first

Everything in this section is a real choice this brief deliberately does not
make. Two of them need deciding *before* the stories that consume them.

**9.1 — Where the Postgres mutant control and the `ProbeThenWriteStore`
negative test mount.** This is the one place the obvious reading of an AC does
not work.

AC-004 says "the phase-3 mutant harness runs with `PostgresEventStore` in the
pass column". The harness is `crates/happenstance-testkit/tests/mutation_coverage.rs`
and its `REGISTRY` (`:324`) plus the registered-fixture list (`:2054-2102`); the
correct core it compares against is entirely in-memory, single-threaded
(`Rc`/`RefCell`) and built so that no `AssertUnwindSafe` appears anywhere, because
the probes are `UnwindSafe` function pointers (`mutation_coverage/correct.rs:1-35`,
`mutation_coverage/harness.rs:436-454`). Registering a live Postgres store there
would (a) make `happenstance-testkit`'s test binary depend on
`happenstance-postgres` — inverting the dependency direction CLAUDE.md sets — and
(b) put a live server inside the default `cargo test --workspace`, which DR-9
forbids.

The seam that does work: run the same *rules* from the Postgres side. The rule
functions are public — `happenstance_testkit::rules::<name>(open: impl AsyncFn() -> F) -> RuleOutcome`
(`crates/happenstance-testkit/src/lib.rs:189`, `src/suite.rs:89`) — so a test in
`crates/happenstance-postgres/tests/`, inside the live CI job, can drive them
directly and record the control. The same seam serves the Neon negative test:
`ProbeThenWriteStore` must be shown to *fail* a rule the real store passes, and
`catch_unwind` over a non-capturing probe is the in-tree pattern for asserting
that a rule rejects an implementation (`mutation_coverage/harness.rs:454`).

**Decide which, and record it.** If the implementer concludes the registry entry
is the right answer after all, that is a change to a dependency rule CLAUDE.md
states, and it needs a written decision — not a `Cargo.toml` edit.

**9.2 — What implements `SqlTransport`.** Unbudgeted scope, named by
`_grounding.md` tension 5, and gating AC-006/AC-007. The crate documents why it
owns no client: a host client needs TLS and `wasm32` has none, so they are two
clients (`transport.rs:1-15`). Three questions the implementer must answer, in
this order:

1. **Does it ship, or is it a dev-dependency?** A transport in
   `[dev-dependencies]`, or behind an off-by-default feature, keeps the
   published crate's dependency surface as it is today and keeps the wasm32
   powerset step honest (`xtask/src/main.rs:558-593`). A transport in
   `[dependencies]` makes `happenstance-neon` a batteries-included crate and
   changes what AC-012's `publish = false` removal actually publishes.
2. **Does the licence allowlist admit it?** The sibling crate's TLS note is
   measured, not assumed: `sqlx`'s `tls-rustls` fails `cargo deny` on
   `webpki-roots` (`CDLA-Permissive-2.0`) — *not* on `ring`, which passes
   (`crates/happenstance-postgres/src/lib.rs:38-47`). Run `cargo deny check`
   against the candidate before committing to it. Growing `deny.toml`'s
   allowlist is a gate weakening and needs a recorded decision.
3. **Does `wasm32` still build?** `happenstance-neon` is the only crate that must
   build for both targets and is checked twice (`xtask/src/main.rs:264-282`). A
   host-only transport must be `cfg`-gated so the wasm32 step stays green; DR-8
   makes that non-negotiable, and the wasm32 *feature powerset* step is what
   catches a feature that is not target-scoped.

**9.3 — Genuinely open, and fine to decide in a story.** The ES-10 mechanism
itself (AC-001's whole content); the tag-matching strategy — join table, `text[]`
with GIN, or `jsonb`, with `Tags` canonically sorted so `@>` stays available
(`crates/happenstance-postgres/src/lib.rs:37-41`); whether the append-condition
SQL for Postgres reuses Neon's CTE shape or exploits the interactive transaction
it has and Neon does not; error-enum growth for the mechanism's decode failures;
schema-per-fixture-instance versus database-per-fixture-instance for isolation;
and whether `sqlx`'s `migrate` feature earns its place.

#### 10. Deviations and downstream risks to carry forward

- **No deviation from any Accepted decision atom is proposed by this brief.**
  Every shape above is either already in-tree or is the shape an existing atom
  requires. If implementation forces one — and DoD 6 explicitly contemplates the
  contract giving — it is a new decision atom with the suite re-run, never an
  edit.
- **The global-invariant premise is inherited, not owned.** If HS-P0010's
  checkpoint design turns out boundary-scoped, ADR-0013 reopens and ADR-0024's
  measurement reopens with it. ADR-0024 should say so rather than present the
  global invariant as permanently settled.
- **`spec/SPECIFICATION.md:2846-2850` owes this phase an instrument.** The
  clause records that `nothing_below_an_observed_position_appears_later` has a
  strength that varies with the adapter's poll shape — against a store whose
  `append` needs three polls, the A/B/B/A interleaving window never opens where
  the rule looks — and names the bounding instrument, *a poll-padding decorator
  over `PreCommitPositionStore`*, as **owed by phase 10 (ADR-0024)**. A real
  Postgres `append` is exactly the multi-poll shape that motivates it. AC-002's
  "a rule the adapter had to work to pass" is not fully evidenced until this is
  either built or the ADR records why it did not need to be.
- **CF-40's contested ownership** (`.kb/open-questions/cf-40-fixture-limits-ownership.md`)
  can stall AC-010. DR-6 says consume, do not settle. If no sibling has settled
  it when the story lands, state the ceilings honestly and record the
  unresolved ownership rather than minting a policy here.

## Testing brief

### Intent

Scopes **HS-P0014** only: the test mix, merge-gate commands and fixtures/seams
that prove `PostgresEventStore`, `NeonEventStore` and (sequenced last, per
Architecture Notes §8) `PostgresProjectionStore` are passing implementations
rather than skeletons. This brief adds no rule to `happenstance-testkit` — per
Architecture Notes §1 the testkit is a **must-not-change** seam here, with the
one caveat §9.1 names — and it does not choose the ES-10 mechanism; it only
states how the mechanism's consequences get proven once ADR-0024 lands.

The project carries a genuine split the test mix must honour: two conformance
families run against **live infrastructure** (a real Postgres, a real Neon
endpoint) and DR-9/AC-011 forbid that infrastructure from entering the default
`cargo xtask ci` path. So the "merge-gate commands" section below is not one
list but two — the tree-local gate every story runs, and the two live jobs
that gate the project as a whole — and getting a story green on the first
without ever exercising the second would satisfy nothing this project exists
to satisfy.

Traces to `project.md` AC-001 … AC-013, DR-1 … DR-9, DoD 1–6. It does not
restate the charter; see `project.md` for the full acceptance-criteria text.

### Acceptance Criteria

One row per project AC-###, mapped to the tier(s) that prove it and where. Tier
names follow CLAUDE.md's own vocabulary (`cargo test --workspace --all-features`
for unit/integration; `event_store_conformance!` and its two sibling macros for
the conformance tier; the gate commands for static/process).

| AC | Tier | How it is proven |
| --- | --- | --- |
| **AC-001** ADR-0024 accepted, numbers included | **Static / process** | `redkiln validate --kb` against the new atom; `.kb/maps/decision-map.md` carries its row. The **numbers themselves** are not asserted by any test — they are read out of a run of the experiment harness at `experiments/position-visibility/README.md` extended to a real adapter (Notes §1), and the ADR body is the artefact that carries them. This AC is proven by the record existing and validating, not by a pass/fail rule. |
| **AC-002** `nothing_below_an_observed_position_appears_later` green, and hard-won | **Conformance** (live Postgres job) | `happenstance_testkit::rules::nothing_below_an_observed_position_appears_later` (`crates/happenstance-testkit/src/suite.rs:5880`), reached through `event_store_conformance!(PostgresFixture::…)` inside the Postgres CI job. "Hard-won" is evidenced by the **control** at Notes §2: the same rule run against a `nextval()`-based mechanism (a throwaway arm, not the shipped one) must fail it first, or the ADR's claim of difficulty is undischarged. |
| **AC-003** concurrency green, writers unserialised | **Conformance / concurrency** (live Postgres job) | `event_store_concurrency_conformance!(PostgresFixture::…)`, which needs `concurrency::ConcurrentFixture` (not `Fixture` — Architecture §6) at `CONTENDERS = 8` under `tokio` `rt-multi-thread`. Runs inside `crates/happenstance-postgres/tests/`, gated to the live job (Notes §3). |
| **AC-004** whole suite + mutant control | **Conformance + unit** (live Postgres job) | `event_store_conformance!` and `event_store_model_conformance!` (behind the testkit's `proptest` feature, `not(target_arch = "wasm32")`) both green; the mutant-control half runs the rule functions directly per Notes §4 / Architecture §9.1 — **not** a `mutation_coverage.rs` `REGISTRY` entry, which would invert the dependency direction CLAUDE.md forbids. |
| **AC-005** `PostgresProjectionStore` passes the projection suite | **Conformance** (live Postgres job) | Whatever macro `projection-store-freeze` (HS-P0010) freezes, invoked against `type Batch<'a> = sqlx::Transaction<'static, Postgres>`. **Blocked**: cannot be written, let alone run, until HS-P0010 leaves `stage: storymap`. Sequence its story last (Architecture §8, point 4); do not report this AC's tier as satisfied by a stub. |
| **AC-006** Neon runs the suite over one-shot HTTP | **Conformance** (live Neon job) | `event_store_conformance!(NeonFixture::…)` against a real `SqlTransport` impl (Notes §5 / Architecture §9.2) hitting a live Neon `/sql` endpoint — never a pooled Postgres connection standing in, which is the exact shortcut DR-4 forbids and the risk table's row 3 names. |
| **AC-007** nothing skips silently | **Conformance + static** (live Neon job) | Every rule the run touches reports `pass`, `fail`, or `RuleOutcome::Skipped` carrying `NeonFixture`'s stated `Capability::declined(reason)` — read from `--show-output` (CLAUDE.md's `tests` step already requests it; Notes §7 covers CI capture). Statically, `capability_skips_are_reported` (the testkit's own meta-test, already in-tree) is what fails the build if a decline goes unreported; this AC is that meta-test plus a human read of the live job's log. |
| **AC-008** `conflicting_position` settled by live evidence | **Conformance** (live Neon job) + **decision record** | The rule(s) exercising `ConditionViolated::conflicting_position` (part of the same `event_store_conformance!` run as AC-006) against the real endpoint, plus a decision-record correction to the ledger's standing assumption (Static, `redkiln validate --kb`) — see project.md AC-008 and Architecture §3 tension 3. Both halves are required; a green run with no ledger correction leaves the known-wrong assumption on record. |
| **AC-009** structural bill written where a consumer meets it | **Static** | Rustdoc build (`cargo doc --workspace --all-features --no-deps --document-private-items`, already `REQUIRED`) plus a manual read confirming `head`'s frontier behaviour, RYOW's absence and the staleness bound are stated on the Postgres crate's public docs; `redkiln validate --kb` if the record is a decision atom rather than doc prose. |
| **AC-010** both stores declare real limits | **Conformance** | `store_accepts_the_guaranteed_minimum_payload` (VT-21–24) and the `MAX_EVENT_DATA_LEN` / `MAX_TAGS_PER_EVENT` / `MAX_EVENTS_PER_BATCH` boundary rules, run once per fixture inside each live job. Refusal path asserted as `AppendError::ExceedsStoreLimit { limit: StoreLimit::… }`, never `AppendError::Store` — a rule failure here is a defect, not a decline. |
| **AC-011** default gate stays Docker-free | **Static / process** | `cargo xtask ci` on a clean checkout with no Docker, no credentials, exits zero (Notes §6). This is the one AC proven by the **absence** of infrastructure rather than its presence — the merge-gate command list below is the evidence. |
| **AC-012** neither crate is a skeleton | **Static** | `cargo xtask package-check` (`REQUIRED`, already runs in the default gate) — `xtask/src/package.rs`'s `reconcile` fails in either direction once `publish = false` is removed without `PUBLISHABLE` and `REQUIRED_FILES` updated (Architecture AC-012 row). `cargo clippy -D warnings` catches a stray `#![allow(clippy::todo)]` only if a `todo!()` remains under it; a manual grep for `todo!()` in both crates' `src/` is the belt to clippy's suspenders. |
| **AC-013** discharge recorded for the audit | **Static** | `cargo xtask spec-trace` (regenerates §7.1–§7.2's markers; `--write` if a marker moved) plus a written record `publication-and-positioning` can read, per Architecture AC-013 row. |

### Notes

**1. The number AC-001 owes is not a test result.** `experiments/position-visibility/README.md`
already measured four SQL *strategies* at 0.987–1.026× (arm C) under
`fsync=on`; `.kb/open-questions/postgres-arm-c-structural-cost.md` states
plainly that this "measured four SQL strategies, not four implementations of
`SendEventStore`" — no pooling, no transaction lifetime tied to a trait
method, no cursor, no error mapping. AC-001's number is a re-measurement
against the **real adapter** built for AC-002/AC-003, including the
long-running-transaction scenario DR-3 names. The harness that produces it is
not part of `cargo xtask ci` at any tier — it is a one-off run whose output
becomes ADR-0024 prose, the same way `experiments/` is documented in
CLAUDE.md's repository map as "measurements... and not in the gate."

**2. AC-002's control.** CLAUDE.md's conformance-suite rule is explicit: "before
adding [a rule], name a plausible wrong implementation it rejects." That
instrument already exists for the *fixture* shape
(`PreCommitPositionStore`, named in the instrument-portfolio table
`RUNBOOK.md`); this project owes the **adapter**-shape control the same
table says phase 10 was always going to need. Concretely: before wiring
ADR-0024's chosen mechanism into `PostgresEventStore::append`, run the same
`nothing_below_an_observed_position_appears_later` rule against a build using
plain `nextval()` (no visibility machinery) and record that it fails. This is
not a permanent fixture or a `Fixture` impl kept in the tree — a throwaway
branch or a feature-gated code path exercised once and recorded in the ADR is
enough; keeping a second, deliberately-broken fixture alive long-term would
itself need a decision, since `happenstance-testkit`'s own conformance
section already keeps one canonical wrong implementation per axis
(`ProbeThenWriteStore` for Neon) and duplicating the pattern for Postgres
without a stated reason is exactly the kind of undecided addition CLAUDE.md's
"if a rule seems wrong, fix the rule and explain why" spirit warns against.

**3. AC-003's fixture is the concurrency-family shape, not `Fixture`.**
`concurrency::ConcurrentFixture`'s bound is `F::Store: EventStore + Send`
(`crates/happenstance-testkit/src/concurrency.rs:1104-1165`) — an opaque
`impl Fixture` return type would leave that bound unprovable at the call site
(Architecture §6). The Postgres fixture module therefore needs **two**
fixture-shaped things, or one type implementing both traits: `Fixture` for
`event_store_conformance!`/`event_store_model_conformance!`, and
`ConcurrentFixture` for the concurrency macro. `MemoryFixture` is the
reference for the former; there is no in-tree reference for a **live**
`ConcurrentFixture` today, so this is new ground, not a copy.

**4. AC-004's mutant control runs rule functions directly, not through the
registry.** Per Architecture §9.1, `happenstance_testkit::rules::<name>(open:
impl AsyncFn() -> F) -> RuleOutcome` (`src/lib.rs:189`, `src/suite.rs:89`) is
public precisely so a test in `crates/happenstance-postgres/tests/` can drive
the rule set against a live Postgres inside the **live CI job** and record a
pass column, without making `happenstance-testkit`'s own test binary depend
on `happenstance-postgres` (the forbidden adapter-on-adapter dependency,
CLAUDE.md) or putting a live server inside `cargo test --workspace`
(DR-9). The same seam serves `ProbeThenWriteStore`'s negative test on the Neon
side: drive the same rules against it and assert at least one **fails**,
using `catch_unwind` over a non-capturing probe per the in-tree pattern at
`mutation_coverage/harness.rs:454`. Which crate's `tests/` directory hosts
this — Postgres only, or a shared helper both adapters' test targets import —
is the implementer's call; Architecture §9.1 flags it as a decision to record
if the answer is "a new dependency edge," not a decision this brief pre-empts.

**5. AC-006's live-endpoint requirement is downstream of a transport that does
not exist.** `SqlTransport`'s only in-tree implementation is `NullTransport`,
which fails every round trip by design (`transport.rs:267-298`). Before
`event_store_conformance!(NeonFixture::…)` can run against anything real,
Architecture §9.2's three questions (ship vs. dev-only, `cargo deny`
admissibility, `wasm32` `cfg`-gating) must be answered and a working
`SqlTransport` built. That work is **not** itself proven by the conformance
tier — it is proven by a much smaller unit test (one HTTP round trip against
the live `/sql` endpoint decodes correctly) that should exist and pass
*before* the fixture is wired to the macro, so a transport bug and a
conformance failure are never debugged as the same failure.

**6. AC-011's merge-gate commands, tree-local (every story, every machine, no
Docker, no network):**

```console
cargo xtask ci --fast    # verify.integration_scoped — this project's non-terminal bar (.redkiln/config.yaml:55)
cargo xtask affected --base <base>    # verify.affected_gate — the story grain
```

Both already exclude the two live conformance families by construction: the
Postgres and Neon `tests/` targets this project adds must therefore be
`#[ignore]`d, `required-features`-gated, or `cfg`-gated on an environment
variable so that `cargo test --workspace --all-features` — which **is** part
of `cargo xtask ci --fast`'s `tests` step — exits zero with no server
reachable. Architecture §7 leaves the mechanism to the implementer with one
hard constraint: the gating must be **whole-invocation**, never a `#[cfg]`
hiding one rule out of a macro's expansion (DR-5).

**Merge-gate commands, project-level (the two new CI jobs, live infrastructure
required, per Architecture §7):**

```console
# Postgres job — testcontainers, pinned Postgres minor, sibling of `gate` /
# `wasm-conformance` / `msrv` / `semver` / `advisories` in .github/workflows/ci.yml
cargo test -p happenstance-postgres --all-features -- --ignored --show-output

# Neon job — a real Neon /sql endpoint and its credential (deployment brief
# owns how the credential is reached)
cargo test -p happenstance-neon --all-features -- --ignored --show-output
```

(The exact invocation depends on which gating mechanism the implementer
chooses per Notes §6 — `--ignored`, a `--features live` flag, or an env var
read inside the test binary. Either way, both jobs must run
`event_store_conformance!`'s full macro expansion, not a hand-picked subset,
or AC-007's "no rule is absent from the run" is unproven by construction.)

**7. Fixtures/seams to build, and the one seam not to rebuild.**

- **`PostgresFixture`** (new) — `connect` hands out a pool checkout per
  Architecture §6's copy-from-`MemoryFixture` guidance; `SECOND_HANDLE`
  answered `SUPPORTED` (a second checkout or a second pool onto the same
  schema); `REOPEN` answered honestly, not declined for convenience;
  `MID_BATCH_FAULT` genuinely answerable (a trigger or armed `CHECK`) rather
  than defaulted; the three `Option<usize>` ceilings stated as real numbers.
- **`ConcurrentFixture` for Postgres** (new) — `CONTENDERS = 8` handles onto
  one backing store, per Notes §3.
- **`NeonFixture`** (new) — `connect` against a live Neon client built on the
  new `SqlTransport` impl; `SECOND_HANDLE` a MUST (a second client against
  the same branch, panicking if declined); `REOPEN` and `MID_BATCH_FAULT`
  answered per Architecture §6; `MAX_RESPONSE_BYTES`-anchored ceilings (halved
  in effect by hex `bytea` rendering, `transport.rs:87-102`).
- **`ProbeThenWriteStore`** — already in-tree (`crates/happenstance-neon/src/event_store.rs:388-503`).
  Do not rebuild it; wire it into the negative test per Notes §4. This is the
  one fixture-adjacent instrument this project **consumes** rather than
  authors.
- **Nothing is mocked in the live jobs.** A `testcontainers`-backed Postgres
  and a real Neon endpoint are the seams under test, per DR-4's "substituting
  a pooled Postgres connection... is not an acceptable fixture" for Neon and
  the symmetric point for Postgres: a mocked driver would prove the trait
  compiles, not that the adapter passed the suite (CLAUDE.md's "the rule that
  matters").

**8. What this brief does not add.** No new rule to
`happenstance-testkit/src/suite.rs` (the one exception, §9.1's seam decision,
is a *composition* of existing public rule functions, not a new rule body).
No benchmark suite — `event_store_benchmarks!` is out of scope per
`project.md` (`sqlite-durable-store`'s work item; CF-34 already distinguishes
benchmarks from conformance). No test of replication/ingest semantics against
`contains_event_id`'s frontier-disagreement question (Architecture §4) — that
is `replication-identity-and-ingest`'s (HS-P0017).

## Deployment brief

### Intent

Scopes **HS-P0014** only: what changes about how this repository is built,
gated and released once two skeleton crates become real adapters. There is no
running service to roll out — both crates are libraries, and "deployment"
here means three things: how the two new live-infrastructure CI jobs are
introduced without weakening the default gate (DR-9), what changes in the
publish-readiness surface (AC-012), and what a consuming application is told
about running either adapter in production, including the one configuration
this project explicitly does not build (Hyperdrive).

Traces to `project.md` AC-011, AC-012, DR-9, DoD 4, and the risk-table row on
live-infrastructure jobs becoming flaky. Where this brief and `project.md`
disagree, `project.md` wins.

### Acceptance Criteria

| AC | Deployment dimension | What is required |
| --- | --- | --- |
| **AC-011** default gate stays Docker-free and network-free | CI job topology | Two **new** GitHub Actions jobs, siblings of `gate`, `backlog`, `wasm-conformance`, `msrv`, `semver`, `advisories` in `.github/workflows/ci.yml` (`:31,102,204,241,279,325`) — not new steps inside `gate`. `xtask/src/main.rs`'s `REQUIRED` array is unchanged in kind; the two crates' new `tests/` targets are excluded from it by the gating mechanism Testing brief Notes §6 names. |
| **AC-012** neither crate is a skeleton; both names claimed | Package/publish readiness | `publish = false` removed from both manifests; `xtask/src/package.rs`'s `PUBLISHABLE` const grows from three names to five; `LICENSE-MIT`, `LICENSE-APACHE`, `README.md` copied into both crate directories; `cargo xtask reserve` already knows both names (`xtask/src/reserve.rs:99,105`) and must be **run**, not merely available, at the start of this project's work (Architecture §8 point 1). |
| **(unlabeled, project.md In-scope)** Hyperdrive deployment note | Documentation, not code | A rustdoc or `README.md` note describing Hyperdrive + a `worker::Socket`-backed driver, stated as **not supported** — it needs a forked driver with unnamed-statement support and a hand-rolled binding (`RUNBOOK.md:4364-4366`). No code, no CI job, no test. |

### Notes

**Feature flags / config gating.** None are introduced by this project in the
sense of a `cargo add --features x` a consumer flips. Two existing feature
splits are relevant and unchanged in shape:

- `happenstance-postgres`'s `event-store` / `projection-store` features
  (`crates/happenstance-postgres/Cargo.toml`, `default = ["event-store",
  "projection-store"]`) — already in the manifest, already default-on; AC-005's
  `PostgresProjectionStore` fills the second feature's body rather than adding
  a new one.
- Whatever cfg-gating Architecture §9.2 settles for a real `SqlTransport`
  implementation on `happenstance-neon`. If the implementer's answer there is
  "ships behind an off-by-default feature," that feature is this project's
  one genuine new flag, and the wasm32 feature-powerset OPTIONAL step
  (`xtask/src/main.rs:564-593`, `cargo hack`-gated) is what catches a feature
  not correctly `cfg`-scoped to the host target. If the answer is "stays a
  dev-dependency," there is no new flag at all. Either way, record which was
  chosen — the deployment posture differs materially (a published crate that
  can make live HTTP calls at runtime vs. one that cannot).

**Migration and backfill.** `happenstance-postgres` owns exactly one migration,
migration 1, already carrying phase-4's `EventId` and `recorded_at` columns
(`RUNBOOK.md:249-253`) plus whichever column ADR-0024's chosen mechanism adds
(an `xid8` column under arm C, or nothing under a lock-based arm). This is a
**schema-authoring** concern, not a backfill concern: there is no prior
production deployment of either crate to migrate data out of — both are
`publish = false` today and have never shipped a schema to a real consumer.
`sqlx` currently ships with no `migrate` feature enabled
(`crates/happenstance-postgres/Cargo.toml`); whether this project turns it on
is Architecture §9.3's open question, not a deployment decision this brief
makes for it. N/A for backfill; real for schema authorship, and the schema
change is exactly what AC-002/AC-004's live Postgres job proves against.

**Rollback posture.** N/A in the conventional sense — nothing is deployed to a
running environment by this project. The two postures that do apply:

- **Decision rollback.** If DoD 6 forces a contract amendment (a `[FROZEN]`
  clause giving because Neon genuinely cannot meet it), that is DR-7's second
  decision record, not a code revert — "a `[FROZEN]` clause changes by ADR,
  never by edit" (CLAUDE.md). There is no partial-rollback state to design
  for; either the amendment is accepted with the suite re-run, or it is not
  and the story is not done.
- **CI rollback.** If a live-infrastructure job goes flaky, DR-9 and the
  risk table are explicit: fixed or reported, never quietly made
  non-blocking. There is no "revert to skip" path this brief sanctions;
  a flaky job is a defect routed per CLAUDE.md's usual channel (the
  `support` initiative, `.redkiln/config.yaml:5`), not a gate the project
  weakens to get green.

**CI implication, concretely.**

- Two new jobs in `.github/workflows/ci.yml`. The Postgres job needs
  `testcontainers` as a new dev-dependency (not present in the tree today —
  confirmed by search) pinned to a specific Postgres minor, following the
  same "the default gate must stay runnable without Docker" reasoning the
  repository already applies to Ladybug's C++ build (`RUNBOOK.md:4356-4358`).
  The Neon job needs a real Neon project/branch and its connection credential
  reachable as a GitHub Actions secret — parallel to how the `backlog` job
  already reaches a private redkiln repository via `REDKILN_TOKEN`
  (`.github/workflows/ci.yml:108-132`) is the nearest in-tree precedent for
  "a job that needs a credential the default `gate` job does not."
- `cargo xtask package-check` (already `REQUIRED`, hence already inside
  `gate`) starts asserting licences and a README for **five** crates instead
  of three the moment `PUBLISHABLE` grows — this is a change in what the
  *existing* default-gate step checks, not a new step.
- `cargo deny check` (already `REQUIRED` when the tool resolves, which it
  does on this machine per CLAUDE.md's *Commands* section) starts walking
  both crates' full dependency graphs once `publish = false` no longer
  excludes them from consideration the way an unpublishable crate implicitly
  narrows attention — Architecture §9.2 point 2 already names the one known
  risk (`sqlx`'s `tls-rustls` failing on `webpki-roots`), and any transport
  dependency Neon gains must clear the same allowlist (`deny.toml:8-20`:
  MIT / Apache-2.0 / BSD-2 / BSD-3 / ISC / Unicode-3.0 / Zlib) before it can
  ship, live-infra job or not.
- The wasm32 build of `happenstance-neon` (already `REQUIRED`,
  `xtask/src/main.rs:264-282`) must stay green throughout — it is the one
  default-gate step this project's own crate already participates in, and
  DR-8 makes regressing it non-negotiable regardless of what the live jobs
  do.

**Release path if a published version changes.** Nothing releases from this
project directly. `PUBLISHABLE`'s growth to five names makes both crates
**publish-ready** (AC-012), but the actual `cargo publish` act, the crates.io
`0.2.0` release, the semver diff and the clause-ledger audit are explicitly
out of scope here and belong to `publication-and-positioning` (HS-P0016),
which "deliberately waits for all four adapter projects" precisely so its
compliance claim can name every implementation it was checked against
(`project.md` *Unlocks*; `_decomposition.md`, *Decisions taken at the gate*,
item 1). This project's deployment surface is therefore **readiness, not
release**: by the time it closes, `cargo package -p happenstance-postgres
--list` and `cargo package -p happenstance-neon --list` both succeed and
carry the required files, and nothing more is claimed. The workspace version
is already `0.2.0` (`Cargo.toml:6`, `version.workspace = true`), so neither
crate needs a version bump of its own when `publish = false` is removed —
they inherit the version the rest of the workspace is already carrying.
