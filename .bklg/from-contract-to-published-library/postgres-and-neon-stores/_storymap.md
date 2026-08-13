---
item: HS-P0014
stage: storymap
created: 2026-08-12T03:30:17.471Z
updated: 2026-08-12T03:30:17.471Z
template_sig: 1c63534a
rendered_sig: d4856143
---

# Story Map — The two stores that disagree with the port

Fourteen stories in seven slices, covering `project.md` AC-001 … AC-013. The
grain follows the architecture brief's four sequencing edges
(`_decomposition.md` *Architecture brief* §8) and its two undecided seams
(§9.1 the mutant-control mount, §9.2 what implements `SqlTransport`), both of
which are given a story rather than left to be discovered mid-slice.

The "user" here is an **adapter author and a consuming application**, not a
screen. A capability story is therefore user-observable in the only medium this
repository has: a conformance family that runs and reports, a public API a
caller meets, or an accepted decision atom a future reader is bound by. There is
no `ux` brief for this project and there is nothing to perceive
(`.redkiln/config.yaml` deliberately omits `design.capture`).

## Backbone

Six activities, left to right, in the order a real run of this project performs
them.

1. **Claim the ground** — reserve both crate names before the work starts, per
   phase 0's rule (`RUNBOOK.md:4346-4348`, `xtask/src/reserve.rs:99,105`).
2. **Make Postgres real** — migration 1, a live fixture, an append path that
   buys ES-10, and the three conformance families green against a pinned server
   in their own CI job.
3. **Decide the mechanism on numbers** — ADR-0024, measured against the adapter
   rather than against four SQL strategies
   (`.kb/open-questions/postgres-arm-c-structural-cost.md`), and the structural
   bill written where a consumer meets it.
4. **Reach the endpoint** — a real `SqlTransport`, which does not exist today
   (`crates/happenstance-neon/src/transport.rs:241-298` — `NullTransport` fails
   every round trip by design).
5. **Make Neon real** — the CTE, the decode paths, the suite over one-shot HTTP,
   and every decline reported by name and reason.
6. **Close the ledger** — the projection store against the frozen `Batch`,
   de-skeleton both crates, and write the far-end discharge down in a form
   `publication-and-positioning` (HS-P0016) can read at audit time.

## Slices

Stories sharing a Milestone are implemented together in one context and mounted
as one integrated surface. Every fixture ships in the same story as the CI job
and the gating that make it runnable — there is no "build the fixture" story
separated from a "wire it into the macro" story, because a fixture no macro is
invoked with is not delivered (`_decomposition.md` *Architecture brief*,
Acceptance-Criteria preamble).

| Milestone | Story | Archetype | One-line slice | depends_on | traces_to |
| --------- | ----- | --------- | -------------- | ---------- | --------- |
| `crate-name-claims` | `claim-crate-names` | foundation | Run `cargo xtask reserve` so `happenstance-postgres` and `happenstance-neon` are held on crates.io before a line of this project's work lands. | — | AC-012 |
| `postgres-live-suite` | `postgres-schema-and-live-fixture` | foundation | Migration 1 (identity, time, tag storage, plus the mechanism's column), a `testcontainers`-backed `PostgresFixture` + `ConcurrentFixture` handing out `CONTENDERS = 8` handles onto one backing store, an in-crate gating mechanism that keeps `cargo test --workspace --all-features` green with no server, and the new live-Postgres CI job beside `gate`. | — | AC-011 |
| `postgres-live-suite` | `postgres-append-and-frontier-head` | capability | An adapter author gets a real `append`/`head`/`contains_event_id` that buys ES-10's visibility invariant, refuses over-limit batches with `ExceedsStoreLimit`, and turns `event_store_conformance!` and `event_store_model_conformance!` green against a live pinned Postgres. | `postgres-schema-and-live-fixture` | AC-002, AC-004, AC-010 |
| `postgres-live-suite` | `postgres-concurrency-family` | capability | `event_store_concurrency_conformance!` runs green at `CONTENDERS = 8` under a multi-thread runtime — the first time any adapter in the portfolio clears that bar on a store whose writers are not serialised. | `postgres-append-and-frontier-head` | AC-003 |
| `postgres-live-suite` | `postgres-rule-controls` | capability | The CF-13 rule is shown to *reject* a naive `nextval()` arm and the rule set is driven directly from the Postgres side as the mutant control, so "it passed" means the adapter had to work for it. | `postgres-append-and-frontier-head` | AC-002, AC-004 |
| `position-visibility-decision` | `adr-0024-position-visibility-mechanism` | capability | ADR-0024 lands accepted, choosing the mechanism on a number re-measured against the real adapter — steady state and with a long-running transaction held open on the same database — and naming the two arms that lost. | `postgres-append-and-frontier-head`, `postgres-concurrency-family`, `postgres-rule-controls` | AC-001 |
| `position-visibility-decision` | `postgres-structural-bill` | capability | A consumer reading the Postgres crate's public docs meets the frontier `head`, the absence of read-your-own-writes and the cluster-wide staleness bound, and the choice between doc prose and a clause of its own is recorded as a decision rather than defaulted. | `adr-0024-position-visibility-mechanism` | AC-009 |
| `neon-transport` | `neon-sql-transport` | foundation | A real `SqlTransport` — host client and `wasm32` client, correctly `cfg`-scoped — proven by one live `/sql` round trip, with ship-vs-dev-dependency, `cargo deny` admissibility and the wasm32 build all answered before any fixture depends on it. | — | AC-006, AC-011 |
| `neon-live-suite` | `neon-fixture-and-live-job` | foundation | `NeonFixture` against a live branch: `SECOND_HANDLE` answered as the MUST it is, `REOPEN` and `MID_BATCH_FAULT` answered honestly, the 64 MiB-anchored ceilings stated as numbers, plus the credentialed Neon CI job that runs it. | `neon-sql-transport` | AC-007, AC-011 |
| `neon-live-suite` | `neon-append-and-read-over-http` | capability | `event_store_conformance!` runs to completion against a store with no connection, no interactive transaction and no cursor, every rule reporting pass, fail, or a skip carrying the fixture's stated reason. | `neon-fixture-and-live-job`, `postgres-append-and-frontier-head` | AC-006, AC-007, AC-010 |
| `neon-live-suite` | `neon-conflicting-position-verdict` | capability | The in-tree CTE's claim that the collapse *keeps* `conflicting_position` is confirmed or refuted against a real endpoint, `ProbeThenWriteStore` is shown to fail a rule the real store passes, and the decision ledger's standing assumption is corrected either way. | `neon-append-and-read-over-http` | AC-008 |
| `postgres-projection-store` | `postgres-projection-store` | capability | `PostgresProjectionStore` is written against the `Batch` shape `projection-store-freeze` freezes and passes the projection suite in the live Postgres job, with any declined capability reported by name and reason. | `postgres-schema-and-live-fixture` | AC-005 |
| `publish-readiness-and-audit` | `deskeleton-and-package-readiness` | capability | Neither crate is a skeleton any more: no `todo!()`, no scoped `#![allow(clippy::todo)]`, no `publish = false`, `PUBLISHABLE` grown to five with licences and READMEs in place, and the Hyperdrive note written as explicitly unsupported. | `claim-crate-names`, `postgres-structural-bill`, `neon-conflicting-position-verdict`, `postgres-projection-store` | AC-012 |
| `publish-readiness-and-audit` | `far-end-discharge-record` | capability | The status of ES-10, ES-11, ES-12, ES-41, ES-42 and VT-21 – VT-24 after this work — discharged, still exposed, or amended, and against which store — is written where the publication audit reads it instead of re-deriving it. | `deskeleton-and-package-readiness` | AC-013 |

### What each story is, in slightly more than one line

**`claim-crate-names`** (foundation). `cargo xtask reserve` already knows both
names (`xtask/src/reserve.rs:99,105`); this story *runs* it. Consumed by
`deskeleton-and-package-readiness`, which cannot honestly remove `publish =
false` from a name nobody holds.

**`postgres-schema-and-live-fixture`** (foundation). The substrate every other
Postgres story consumes. Copy `MemoryFixture`'s shape
(`crates/happenstance-testkit/src/fixtures.rs:230-280`): `connect` hands out a
handle owning a refcount rather than borrowing the fixture's lifetime, and
`connect` panics rather than returning `Result` so "the database is down" never
enters the same channel as "the adapter is wrong"
(`crates/happenstance-testkit/src/contract.rs:309-321`). Two fixture-shaped
things or one type implementing both `Fixture` and
`concurrency::ConcurrentFixture` — an opaque `impl Fixture` leaves
`F::Store: Send` unprovable at the call site
(`crates/happenstance-testkit/src/concurrency.rs:1104-1165`). The gating is
whole-invocation (`#[ignore]`, `required-features`, or an env read), never a
`#[cfg]` hiding a rule out of a macro expansion — that distinction is DR-5's and
it is the one hard constraint on the mechanism.

**`postgres-append-and-frontier-head`** (capability). Root A, `impl
SendEventStore for PostgresEventStore`
(`crates/happenstance-postgres/src/event_store.rs:121-175`). `head` returns the
**visibility frontier**, not `max(position)`; the frontier predicate composes
*into* the cursor's `DECLARE` inside the same `REPEATABLE READ` transaction, so
the snapshot cannot move under the caller between `FETCH`es — evaluating it per
chunk is the easy way to get the read half wrong
(`crates/happenstance-postgres/src/read_stream.rs:37-77`). The mechanism wired
here is the one ADR-0013 established as affordable; the *choice* is
`adr-0024-position-visibility-mechanism`'s, which is why that story comes after
this one and not before (Architecture §8 point 2). Never assert on literal
position values — this is the project where gaps stop being hypothetical
(CF-6, `cargo xtask lint-position-literals`).

**`postgres-concurrency-family`** (capability). The instrument that catches the
failure mode the risk table names first: a mechanism that makes Postgres pass by
serialising every writer on one row has not implemented the adapter, it has
deleted the axis (`crates/happenstance-postgres/src/event_store.rs:43-53`).

**`postgres-rule-controls`** (capability). Two controls, one story, because both
answer "would this rule have noticed?". The naive `nextval()` arm must be shown
to **fail** `nothing_below_an_observed_position_appears_later` before the chosen
mechanism is credited with passing it — a feature-gated path exercised once and
recorded, not a second deliberately-broken fixture kept alive
(*Testing brief* Notes §2). The mutant control drives the public rule functions
(`happenstance_testkit::rules::<name>`, `crates/happenstance-testkit/src/lib.rs:189`)
from `crates/happenstance-postgres/tests/`, **not** a `mutation_coverage.rs`
`REGISTRY` entry, which would invert the dependency direction `CLAUDE.md` sets
and put a live server inside `cargo test --workspace` (Architecture §9.1). If
the implementer concludes the registry entry is right after all, that is a
recorded decision, not a `Cargo.toml` edit. This story also settles the
poll-padding decorator over `PreCommitPositionStore` that
`spec/SPECIFICATION.md:2846-2850` names as owed by this phase: build it, or
record in ADR-0024 why a real multi-poll `append` did not need it.

**`adr-0024-position-visibility-mechanism`** (capability). Root D: `.kb/_intake/`
→ `/redkiln:kb-ingest` → an atom under `.kb/decisions/` with its row in
`.kb/maps/decision-map.md`, the long form in `references/adr/`, and the atom
linking it. Never hand-authored — hand-writing atoms is what `0269720` reverted.
The four ordered sub-questions in
`.kb/open-questions/postgres-arm-c-structural-cost.md` are the shape of the
"measured cost" section, and the record must say plainly that it inherits
ADR-0013's global premise without owning it
(`.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`, owned
by phase 6).

**`postgres-structural-bill`** (capability). `spec/SPECIFICATION.md:2838-2844`
already states the frontier / no-RYOW / cluster-staleness consequences under
ES-10. This story records whether that is *sufficient* or a clause of its own is
owed — as a decision, not a default — and puts the consumer-facing half in the
crate's rustdoc where a caller actually meets it.

**`neon-sql-transport`** (foundation). Architecture §9.2's three questions
answered in order: ships or dev-dependency; `cargo deny` admissibility against
`deny.toml:8-20` (the sibling crate's note is measured — `sqlx`'s `tls-rustls`
fails on `webpki-roots`, *not* on `ring`); and whether `wasm32` still builds,
which `xtask/src/main.rs:264-282` and the feature-powerset step enforce. Proven
by one small live round trip *before* any fixture is wired to a macro, so a
transport bug and a conformance failure are never debugged as the same failure
(*Testing brief* Notes §5). Consumed by both Neon capability stories.

**`neon-fixture-and-live-job`** (foundation). `SECOND_HANDLE` is a MUST and
declining it makes `two_handles_observe_each_others_appends` **panic** quoting
the fixture's own words (`crates/happenstance-testkit/src/contract.rs:135-161`).
Ceilings are `Option<usize>`, not `Capability` — a ceiling is a fact, a decline
is a trade (`contract.rs:213-279`) — anchored on `MAX_RESPONSE_BYTES`
(`crates/happenstance-neon/src/transport.rs:45-54`), halved in effect by hex
`bytea` rendering (`transport.rs:87-102`). Note that
`Capability::declined("")` fires at *codegen*, so a green `clippy` proves
nothing about the constants (`contract.rs:404-410`).

**`neon-append-and-read-over-http`** (capability). The three remaining
`todo!()`s: `conditional_append_request`, `decode_append_response`,
`decode_read_response` (`crates/happenstance-neon/src/event_store.rs:159-165`,
`:249-255`, `:257-263`). The impl above them is already wired — it maps
`Conflict` to `AppendError::ConditionViolated` and refuses the empty batch
first. Bare `EventStore` only, on both targets; a second `SendEventStore` impl
is `error[E0119]` against `trait_variant`'s blanket impl
(`crates/happenstance-neon/src/lib.rs:79-89`). The concurrency family is
Postgres-only by design and its absence here is not a gap. Depends on
`postgres-append-and-frontier-head` because Neon inherits migration 1, the tag
storage and the append-condition SQL — the reason these are one project and not
two.

**`neon-conflicting-position-verdict`** (capability). Verification of an in-tree
answer plus a ledger correction, not the discovery of a new limitation: the
crate's own doc already says the collapse **keeps** `conflicting_position`
"contrary to the standing assumption in the decision ledger"
(`crates/happenstance-neon/src/lib.rs:46-77`), at the cost of an aggregate index
scan per append and a `Serializable` default. `ProbeThenWriteStore`
(`event_store.rs:388-503`) is consumed here, never rebuilt and never fixed. If
the live endpoint refutes the CTE, the deliverable is the DoD-6 amendment record
with the suite re-run — a `[FROZEN]` clause changes by ADR, never by edit.

**`postgres-projection-store`** (capability). Sequenced last of the
implementation work and gated on HS-P0010 leaving `stage: storymap`; the shape
is already recorded as `type Batch<'a> = sqlx::Transaction<'static, Postgres>`
(`crates/happenstance-postgres/src/projection_store.rs:26-45`,
`references/adapter-shapes.md`). Writing it early against an unfrozen `Batch`
and reworking it is precisely the failure `project.md`'s risk table names.

**`deskeleton-and-package-readiness`** (capability). A three-part change in both
directions: manifest, `PUBLISHABLE` (`xtask/src/package.rs:94,172-212`), and the
three `REQUIRED_FILES` copied into each crate directory — neither directory holds
any of the three today. `cargo xtask package-check` is already `REQUIRED`, so it
starts asserting five crates the moment the list grows. Readiness, not release:
publishing is HS-P0016's.

**`far-end-discharge-record`** (capability). `cargo xtask spec-trace --write`
regenerates §7.1–§7.2's markers if one moved — never hand-edited — and the
written record lands where HS-P0016 reads it, alongside the
`references/adapter-shapes.md` and RUNBOOK phase-10 session-log entries DoD 6
asks for.

## Coverage

Every project AC-### maps to at least one story, and no two stories own the same
responsibility for one AC (where an AC appears twice, the split is stated).

| Project AC | Stories | Split, where there is one |
| --- | --- | --- |
| **AC-001** ADR-0024 accepted, decided on numbers | `adr-0024-position-visibility-mechanism` | sole owner |
| **AC-002** CF-13 green, and hard-won | `postgres-append-and-frontier-head`, `postgres-rule-controls` | the pass; the proof it was not free (naive arm fails, decorator built or excused) |
| **AC-003** concurrency green, writers unserialised | `postgres-concurrency-family` | sole owner |
| **AC-004** whole suite + mutant control | `postgres-append-and-frontier-head`, `postgres-rule-controls` | the two conformance families; the mutant control column |
| **AC-005** `PostgresProjectionStore` passes | `postgres-projection-store` | sole owner |
| **AC-006** Neon runs the suite over one-shot HTTP | `neon-sql-transport`, `neon-append-and-read-over-http` | the real endpoint reached; the suite run against it |
| **AC-007** nothing skips silently | `neon-fixture-and-live-job`, `neon-append-and-read-over-http` | capabilities declared with real reasons; every rule present in the run and reporting |
| **AC-008** `conflicting_position` settled by evidence | `neon-conflicting-position-verdict` | sole owner |
| **AC-009** the structural bill is written | `postgres-structural-bill` | sole owner |
| **AC-010** both stores declare real limits | `postgres-append-and-frontier-head`, `neon-append-and-read-over-http` | Postgres ceilings + `ExceedsStoreLimit` refusal + VT-21 – VT-24; the same for Neon, anchored on `MAX_RESPONSE_BYTES` |
| **AC-011** default gate stays Docker-free and network-free | `postgres-schema-and-live-fixture`, `neon-fixture-and-live-job`, `neon-sql-transport` | the Postgres job; the Neon job; the wasm32 build that must not regress |
| **AC-012** neither crate is a skeleton | `claim-crate-names`, `deskeleton-and-package-readiness` | the names held; the skeleton markers removed and the package surface reconciled |
| **AC-013** far-end discharge recorded for the audit | `far-end-discharge-record` | sole owner |

Thirteen ACs, thirteen rows, no orphans. Read the other way: every one of the
fourteen stories carries at least one AC, so no story is scope this project did
not accept.

## Merge order

Foundation before the capability slices that consume it; cross-slice
`depends_on` edges are acyclic. `crate-name-claims` and `neon-transport` have no
inbound edges and `neon-transport` can start on day one — the transport work does
**not** wait on the Postgres schema (Architecture §8 point 3), which is the one
place this plan buys real parallelism.

1. **`crate-name-claims`** — `claim-crate-names`. First, per phase 0's rule
   (`RUNBOOK.md:4346-4348`). No code depends on it; `deskeleton-and-package-readiness`
   does.
2. **`postgres-live-suite`** — `postgres-schema-and-live-fixture` (foundation),
   then `postgres-append-and-frontier-head`, then
   `postgres-concurrency-family` and `postgres-rule-controls` (independent of
   each other, either order). **`neon-transport`** — `neon-sql-transport` — may
   run entirely in parallel with this slice.
3. **`position-visibility-decision`** — `adr-0024-position-visibility-mechanism`,
   then `postgres-structural-bill`. The ADR lands *after* a working append path
   because the number it owes is a re-measurement against the adapter, not
   against four SQL strategies.
4. **`neon-live-suite`** — `neon-fixture-and-live-job` (foundation), then
   `neon-append-and-read-over-http`, then `neon-conflicting-position-verdict`.
   Needs `neon-transport` merged and the Postgres schema settled.
5. **`postgres-projection-store`** — `postgres-projection-store`. Gated on
   `projection-store-freeze` (HS-P0010) freezing the `Batch` shape and its
   suite; that gate is a project-level `blocked_by`, not a story edge, and the
   slice sits here so it can slide right without moving anything else.
6. **`publish-readiness-and-audit`** — `deskeleton-and-package-readiness`, then
   `far-end-discharge-record`. Both are terminal by construction: the first
   cannot honestly run until the last `todo!()` in either crate is gone, and the
   second reports on what the five slices above it actually discharged.

### Grain notes

- `project.md` flags this as the largest non-trunk item (~11 runbook-days,
  `RUNBOOK.md:4387`) and the most reasonable split candidate. If the split is
  taken, the seam is between slices 3 and 4 — Postgres-and-its-decision, then
  Neon — and **not** between "schema" and "transport", which would recreate the
  horizontal seam the decomposition gate rejected.
- Two seams are decided by a story rather than assumed: §9.1 (where the mutant
  control and the `ProbeThenWriteStore` negative test mount) belongs to
  `postgres-rule-controls`, and §9.2 (what implements `SqlTransport`) belongs to
  `neon-sql-transport`. Both decisions are recorded, not merely made.
- Nothing here depends on substrate owned outside this initiative. The two
  external gates — HS-P0010's frozen `Batch` and whatever sibling settles CF-40's
  contested ownership (`.kb/open-questions/cf-40-fixture-limits-ownership.md`) —
  are consumed, never re-decided here; if CF-40 is still unowned when
  `neon-append-and-read-over-http` lands, the ceilings are stated honestly and
  the unresolved ownership is recorded rather than a policy minted (DR-6).
