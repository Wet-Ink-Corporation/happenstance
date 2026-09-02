---
item: HS-S0061
stage: spec
created: 2026-08-12T13:46:59.764Z
updated: 2026-08-12T13:46:59.764Z
template_sig: 87bbf1d0
rendered_sig: 006a9967
---

# Spec — Migration 1, the live Postgres fixture, and the CI job that runs it

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DoD 5 and DoD 6; BR-02, BR-03, BR-13 |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — the ten-project portfolio and its DAG |
| Project | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/project.md` — AC-011, DR-9, the risk row on flaky live jobs |
| This spec | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/postgres-schema-and-live-fixture/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md` — *Architecture brief* §4 (migration 1), §6 (the two fixtures), §7 (gate and CI wiring), §9.3 (what is left open); *Testing brief* Notes §3, §6, §7; *Deployment brief* Notes (migration/backfill, CI implication) |
| Signed-off design | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_design.md` — **N/A by sign-off**: this project records no user-facing surface, approved 2026-08-12. This story renders none. |
| Story map row | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_storymap.md` — slice `postgres-live-suite`, row 2 |
| Roadmap pointer | `RUNBOOK.md:4350-4362` — phase 10's work items, including "`testcontainers` for the fixture, pinned to a specific Postgres minor, in **its own CI job**" |

## One-line PR slice

Migration 1 (identity, time, tag storage, plus the mechanism's column), a
`testcontainers`-backed `PostgresFixture` + `ConcurrentFixture` handing out
`CONTENDERS = 8` handles onto one backing store, an in-crate gating mechanism
that keeps `cargo test --workspace --all-features` green with no server, and the
new live-Postgres CI job beside `gate`.

## Executive summary

**Lands:** the substrate every other Postgres story in this project stands on —
a real schema, a real fixture, and a real place for it to run.

Today `crates/happenstance-postgres/` has no `tests/` directory, no migration
source, and no fixture; its schema is prose in a rustdoc comment
(`crates/happenstance-postgres/src/event_store.rs:9-20`) and its `Cargo.toml`
disables `sqlx`'s `migrate` feature with the stated reason "the schema is prose
in `event_store.rs` until phase 10 makes it real"
(`crates/happenstance-postgres/Cargo.toml:18-22`). This PR is phase 10 making it
real.

**Delta against the project charter.** `project.md` AC-011 is the AC this story
traces to, and AC-011 is the one criterion in the whole project proven by the
**absence** of infrastructure rather than its presence (*Testing brief* AC table,
AC-011 row). So the deliverable is deliberately two-sided: live Postgres reachable
in a dedicated CI job, and a default gate that stays runnable on a laptop with no
Docker daemon and no network. Getting either half without the other satisfies
nothing.

**Delta against the story map.** The map's rule is that "a fixture no macro is
invoked with is not delivered," so this PR also creates the tests target that
invokes `event_store_conformance!(PostgresFixture::…)`. It does **not** make that
invocation pass: `append`, `head` and `contains_event_id` are still `todo!()`
(`crates/happenstance-postgres/src/event_store.rs:143,160,173`) and become real in
the slice-mate `postgres-append-and-frontier-head`. The slice is green as a slice,
in one context; this story is green as the thing the slice-mate mounts into.

## Context pack

The load-bearing decisions this story must honor. Everything deeper is a
signposted anchor; nothing below is optional reading.

**1. The default gate must stay Docker-free and network-free, and that decides
where the live suite mounts.** DR-9 and `project.md` AC-011 forbid live
infrastructure from entering `xtask/src/main.rs`'s `REQUIRED` array, because
everything in that array runs on every `cargo xtask ci` — and `.redkiln/config.yaml`
wires `cargo xtask affected` and `cargo xtask ci --fast` as the story and
non-terminal-integration verify grains for **every other project in the
initiative** (`.redkiln/config.yaml:40,55`). A step that needs a container breaks
all of them. The live suite therefore mounts in a new `.github/workflows/ci.yml`
job, a sibling of `gate` (`:31`), `backlog` (`:102`), `wasm-conformance` (`:204`),
`msrv` (`:241`), `semver` (`:279`) and `advisories` (`:325`) — **not** a new step
inside `gate`.

**2. The gating is whole-invocation, and that distinction is a hard constraint,
not a style preference.** The tests target this story adds must be `#[ignore]`d,
`required-features`-gated, or gated on an environment variable read inside the
test binary — the implementer's call (*Architecture brief* §7) — with one thing
forbidden: **it may not be a `#[cfg]` hiding a conformance rule out of a macro's
expansion.** DR-5 draws that line, and the reason is CF-18's: a rule silently
omitted is indistinguishable in CI output from a rule that passed. Gating a whole
invocation on infrastructure availability is a different act from making one rule
vanish, and only the second is forbidden.

**3. A live job that is green because it ran nothing is the failure mode this
story is most likely to ship.** It is the direct consequence of decision 2 — the
same flag that makes `cargo test --workspace --all-features` exit zero with no
server makes a misconfigured CI job exit zero with no tests. The risk table's
"live-infrastructure jobs become flaky and get quietly weakened" row
(`project.md` *Risks*) names the neighbouring failure; this is its quieter
sibling, and the job must be unable to pass without having executed the gated
tests.

**4. `connect` panics; it does not return `Result`, and live infrastructure is
exactly why.** `Fixture::connect` is `fn connect(&self) -> impl Future<Output = Self::Store>`
with no `Result` (`crates/happenstance-testkit/src/contract.rs:309-321`), and the
rationale is written there: a fixture that cannot connect is a broken **test
environment**, not a non-conformant adapter, and a `Result` would put "the database
is down" into the same channel as "the adapter is wrong." This is the first fixture
in the workspace where those two are genuinely different events, so the panic
message has to keep the distinction legible to whoever reads the CI log.

**5. A handle owns a refcount; it does not borrow the fixture.** `MemoryFixture`'s
`connect` is an `Arc` clone (`crates/happenstance-testkit/src/fixtures.rs:286-291`),
and that shape is not stylistic: a borrowing GAT (`type Store<'a> where Self: 'a`)
on a foreign trait is one of five independently necessary ingredients of a rustc
ICE that still reproduces on 1.97.1 (`contract.rs:97-111`). A pool-backed fixture
does the same thing with a `PgPool` clone, which is itself an `Arc` internally.
Do not reach for the GAT.

**6. Two fixture-shaped things, or one type implementing both traits — and never
an opaque `impl Fixture`.** `ConcurrentFixture` is a distinct trait,
`pub trait ConcurrentFixture: Fixture<Store: Send> {}`
(`crates/happenstance-testkit/src/concurrency.rs:186`), and it exists for one
line in the concurrency macro: an opaque return type carries only the bounds
written on it, so `impl Fixture` would leave every rule's `F::Store: Send`
unprovable at the call site (`concurrency.rs:1140-1146`). One fixture instance
must hand out `CONTENDERS = 8` (`concurrency.rs:206`) handles onto **one** backing
store. There is no in-tree reference for a *live* `ConcurrentFixture`, so this is
new ground rather than a copy (*Testing brief* Notes §3).

**7. Every capability constant is an answer, and a silent default on a store that
could have co-operated is the move to avoid.** `SECOND_HANDLE` is the only MUST
(`contract.rs:135-161`) and declining it makes
`two_handles_observe_each_others_appends` **panic** quoting the fixture's own
words — Postgres answers it with a second pool checkout or a second pool onto the
same schema. `REOPEN` is a SHOULD and Postgres is durable, so a decline needs a
real reason, not a convenient one. `MID_BATCH_FAULT` **defaults to declined**
(`contract.rs:207-211`) and Postgres can genuinely offer it (a trigger raising on
the third insert, a `CHECK` armed for one write); taking the default here is the
one outcome the architecture brief singles out as wrong (§6). And
`Capability::declined("")` is rejected by an `assert!` in a `const fn` that, for
an *associated* const, fires at **codegen** — so `cargo build` and `cargo test`
catch it and `cargo clippy` does not (`contract.rs:404-410`). A green clippy
proves nothing about these constants.

**8. The three ceilings are `Option<usize>` facts, not `Capability` trades.**
`MAX_EVENT_DATA_LEN`, `MAX_TAGS_PER_EVENT` and `MAX_EVENTS_PER_BATCH`
(`contract.rs:213-279`). Stating a number commits the store: the rule appends
exactly that many bytes and requires acceptance, then one more and requires
`AppendError::ExceedsStoreLimit { limit: StoreLimit::… }` — never
`AppendError::Store`. A number that is not where the real ceiling sits fails in
one direction or the other. CF-40's ownership is contested
(`.kb/open-questions/cf-40-fixture-limits-ownership.md`) and DR-6 says **consume,
do not settle**: if no sibling has settled it when this lands, state the ceilings
honestly and record the unresolved ownership rather than minting a policy here.

**9. `position` is deliberately not `bigserial`, and migration 1 must not
accidentally decide ADR-0024.** `crates/happenstance-postgres/src/event_store.rs:22-24`
says it in terms: "a `serial` column is `nextval()`, and `nextval()` is where the
invariant is lost." ADR-0024 does not exist and is a *deliverable* of the
`position-visibility-decision` slice, not prior art (*Architecture brief* §3,
tension 1). This story authors the columns that are settled independent of that
choice; the mechanism's column is the slice-mate's, added to the same migration.

**10. Nothing is mocked.** A `testcontainers`-backed Postgres pinned to a specific
minor is the seam under test. A mocked driver would prove the trait compiles, not
that the adapter passed the suite — CLAUDE.md's "the rule that matters," and the
symmetric point to DR-4's Neon prohibition (*Testing brief* Notes §7).

**The persona-journey slice.** The "user" here is an adapter author, and this
story is the moment their crate stops being an instrument nobody can run. Before
it, `cargo test -p happenstance-postgres` compiles a skeleton and asserts nothing
about a server. After it, there is one command that reaches a real pinned Postgres
and reports rule by rule, and one command that does not need a server at all — and
the adapter author can tell at a glance which one they just ran.

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `foundation` — real in-tree substrate consumed by the capability slices beside it. No double, no fixme. |
| **Slice / milestone** | `postgres-live-suite`. Slice-mates: `postgres-append-and-frontier-head`, `postgres-concurrency-family`, `postgres-rule-controls`. Implemented together in one context; mounted as one integrated surface. |
| **Mount point** | `crates/happenstance-postgres/tests/postgres_conformance.rs` (**new**) — Root B of *Architecture brief* §2: the file that invokes `event_store_conformance!(PostgresFixture::…)`. The macro takes an **expression building a `Fixture`**, precisely because "a Postgres fixture needs a connection URL, and a type with an argument-less constructor would have to reach into the environment for it" (`crates/happenstance-testkit/src/lib.rs:267-274`). Second mount, and inseparable from the first: the new live-Postgres job in `.github/workflows/ci.yml`, sibling of `gate` (`:31`). A fixture no macro is invoked with, or a macro no workflow runs, is not delivered. |
| **Wires into** | `Fixture` and `Capability` (`crates/happenstance-testkit/src/contract.rs:120-353`, `:355-430`); `concurrency::ConcurrentFixture` and `concurrency::CONTENDERS` (`crates/happenstance-testkit/src/concurrency.rs:186,206`); `event_store_conformance!` (`crates/happenstance-testkit/src/lib.rs:311-357`); `PostgresEventStore::new(PgPool)` (`crates/happenstance-postgres/src/event_store.rs:102-116`); `EventId` / `StoreId` / `RecordedAt` as the schema's identity and time columns (`crates/happenstance-core/src/identity.rs:44,97,158`); `MemoryFixture` as the reference shape to copy, not to import (`crates/happenstance-testkit/src/fixtures.rs:219-292`). |
| **Renders surfaces** | **none.** `_design.md` records this project as having no user-facing surface, signed off 2026-08-12. There is no surface id to claim and nothing to perceive; `design.capture` is deliberately absent from `.redkiln/config.yaml`, which makes the perceptual review a declared skip rather than a silent pass. |
| **Conformance rule(s)** | This story adds **no rule** to `happenstance-testkit` — the testkit is a must-not-change seam here (*Architecture brief* §1). What it does is make existing rules *reachable* against a live store. The rules whose reachability this story is responsible for, and which its slice-mates then turn green: `two_handles_observe_each_others_appends` (gated on `SECOND_HANDLE`, and it panics rather than skips on a decline), `two_fixture_instances_observe_none_of_each_others_appends` (the isolation rule this story's per-instance isolation decision is answerable to), `append_reports_exceeded_store_limits` and `store_accepts_the_guaranteed_minimum_payload` (gated on the three ceilings), and the whole `for_each_concurrency_rule!` family at `CONTENDERS = 8`. |
| **Clause(s)** | Discharges none on its own; **enables** the phase-10 owner rows for ES-10, ES-11 and ES-12 (`RUNBOOK.md:606`) by giving them a server to be checked against. It amends **no** clause — no `[FROZEN]` clause is touched, so no ADR is owed by this story. CF-6 (no literal position values, enforced by `cargo xtask lint-position-literals`, `xtask/src/main.rs:389,687`) and CF-18 (a skip must be reported, never omitted) both bind the code this story writes. |
| **Advances DoD scenario** | Initiative **DoD 5** — "a store that does not serialise its writers passes the suite, with the position-visibility cost measured rather than estimated" (`.bklg/from-contract-to-published-library/initiative.md:373-374`). This story does not observe DoD 5; it is the only thing standing between the slice and being *able* to observe it, since every remaining step needs a schema, a fixture and a server. |

## PR boundary

**In this PR**

- One migration source owned by `happenstance-postgres`, carrying the event
  table, its identity and time columns, its tag storage and its indexes.
- The mechanism by which that migration is applied to a fresh database, and the
  per-fixture-instance isolation scheme (schema-per-instance or
  database-per-instance — *Architecture brief* §9.3 leaves this open and this
  story closes it).
- `PostgresFixture`: `Fixture` impl, all five capability/ceiling constants
  answered deliberately, `connect` handing out a pool-backed owning handle.
- The `ConcurrentFixture` shape — one type implementing both traits, or a second
  fixture-shaped type — such that `event_store_concurrency_conformance!`
  type-checks with `F::Store: Send` provable at the call site.
- `testcontainers` as a dev-dependency, pinned to a specific Postgres minor,
  wired through `[workspace.dependencies]` in the root `Cargo.toml`.
- The new tests target that invokes `event_store_conformance!` (and its
  concurrency sibling), plus the whole-invocation gating that keeps it out of
  the default gate.
- The new live-Postgres job in `.github/workflows/ci.yml`, and the assertion
  that it cannot pass without having executed the gated tests.
- This story's own backlog folder (`spec.md`, `_ledger.md`).

**Explicitly not in this PR**

- `append`, `head` or `contains_event_id` bodies — they stay `todo!()`;
  `postgres-append-and-frontier-head` owns them.
- ADR-0024, the choice of ES-10 mechanism, or the column that choice adds. This
  story must not encode the answer by accident (Context pack §9).
- Any change to `crates/happenstance-testkit/**` or `crates/happenstance-core/**`.
- Any change to `xtask/src/main.rs`'s `REQUIRED` array, `xtask/src/package.rs`'s
  `PUBLISHABLE`, `publish = false`, or the scoped `#![allow(clippy::todo)]`
  (`crates/happenstance-postgres/src/lib.rs:58-63`) —
  `deskeleton-and-package-readiness` owns all four.
- `crates/happenstance-neon/**` and `crates/happenstance-postgres/src/projection_store.rs`.
- Any widening of `deny.toml`'s licence allowlist. If the pinned
  `testcontainers` graph fails `cargo deny check`, that is a finding to report,
  not an allowlist to grow (*Architecture brief* §9.2 point 2).

The implementer MAY touch the composition-root and wiring files named in the
Integration contract — the new tests target, the root `Cargo.toml`,
`crates/happenstance-postgres/Cargo.toml` and `.github/workflows/ci.yml` — to
mount this slice. That is the mount, not scope drift.

```
crates/happenstance-postgres/migrations/**
crates/happenstance-postgres/src/**
crates/happenstance-postgres/tests/**
crates/happenstance-postgres/Cargo.toml
Cargo.toml
Cargo.lock
.github/workflows/ci.yml
.bklg/from-contract-to-published-library/postgres-and-neon-stores/postgres-schema-and-live-fixture/**
```

**Merge DoD.** `cargo xtask ci --fast` is green on a checkout with the Docker
daemon stopped and the network unplugged, the new live-Postgres job is green on
the same tree with a nonzero count of executed tests, and `crates/happenstance-postgres`
still holds every `todo!()` it held before.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **Migration 1 is a file, not a doc comment** | One SQL source under the crate, carrying `event(position, event_type, data, metadata, tags, …)` plus the identity and time columns, the GIN index on `tags` and the `(event_type, position)` index. `position` is `bigint PRIMARY KEY` and **not** `bigserial`. | `crates/happenstance-postgres/src/event_store.rs:9-24`; `RUNBOOK.md:249-253` (phase 4's columns are migration-1 columns in every store) |
| **The identity columns are the `EventId` pair** | `EventId` is `(StoreId, SequencePosition)`, so the schema's `origin_store` (16 bytes) and `origin_position` (`bigint`) *are* it. `contains_event_id` needs exactly these two and the prose schema lacks them; this story adds them so the slice-mate is not blocked on a schema change. | `crates/happenstance-core/src/identity.rs:44,97-125`; `crates/happenstance-postgres/src/event_store.rs:163-174` |
| **`recorded_at` is milliseconds, stored as an integer** | `RecordedAt` is `RecordedAt(i64)` — milliseconds since the epoch, admitting times **before** it. Store it as `bigint`. A `timestamptz` column is a lossy conversion the port never asked for, and the negative case is already a named core test. | `crates/happenstance-core/src/identity.rs:158-172`, `:414` (`recorded_at_round_trips_and_admits_times_before_the_epoch`) |
| **Tag storage is chosen here** | `text[]` + GIN, a join table, or `jsonb`. `Tags` is canonically sorted so the containment operator `@>` stays available; the prose schema already leans `text[]` with a GIN index. Whichever is picked, record why — this is one of *Architecture brief* §9.3's genuinely open choices. | `crates/happenstance-postgres/src/lib.rs:37-39`; `crates/happenstance-postgres/src/event_store.rs:15-18` |
| **How the migration is applied** | `sqlx`'s `migrate` feature is deliberately off today, with the reason recorded in the manifest. Turning it on is a **manifest decision to make deliberately, not incidentally** (*Architecture brief* §4); the alternative is executing the SQL source directly from the fixture. Either is admissible; the choice is recorded. | `crates/happenstance-postgres/Cargo.toml:18-22` |
| **Per-instance isolation** | One `PostgresFixture` instance is one isolated backing store (CLAUDE.md's fixture rule). Schema-per-instance or database-per-instance — §9.3 leaves it open. Whichever is chosen must make `two_fixture_instances_observe_none_of_each_others_appends` pass, which is the rule that catches "every fixture points at one temporary path." | `crates/happenstance-testkit/src/fixtures.rs:252-267` (`MemoryFixture::sharing` is the in-tree implementation that *fails* that rule) |
| **`PostgresFixture::connect` returns an owning handle** | A `PgPool` clone (internally an `Arc`), never a borrow of the fixture. `Fixture::Store` stays an ordinary associated type; the GAT shape is the ICE ingredient to keep away from. | `crates/happenstance-testkit/src/contract.rs:97-111`; `crates/happenstance-testkit/src/fixtures.rs:225-229,286-291` |
| **`connect` panics, informatively** | No `Result`. The panic message must make "the container is not up / the URL is wrong" unmistakable as an environment failure rather than an adapter defect — the distinction the trait's own docs say the signature exists to protect. | `crates/happenstance-testkit/src/contract.rs:309-321` |
| **`SECOND_HANDLE = Capability::SUPPORTED`** | A second pool checkout, or a second pool onto the same schema. It is the only MUST among the capabilities, and declining it makes the rule **panic** quoting the fixture's own words rather than skip. | `crates/happenstance-testkit/src/contract.rs:135-161` |
| **`REOPEN` answered honestly** | Postgres is durable, so it can plausibly support it; a decline needs a real reason, not a convenient one. Durability's far end belongs to `sqlite-durable-store`, so this story owes an honest answer, not an investment. | `crates/happenstance-testkit/src/contract.rs:167-173`; *Architecture brief* §6 |
| **`MID_BATCH_FAULT` is answered, not defaulted** | It defaults to declined. Postgres can genuinely offer it — a trigger raising on the third insert, a `CHECK` armed for one write. Either answer is acceptable; a *silent* default on the store that could have co-operated is the one to avoid. If supported, `arm_mid_batch_fault` must be overridden or the provided body panics. | `crates/happenstance-testkit/src/contract.rs:207-211,281-307` |
| **The three ceilings are real numbers or honest `None`s** | `Option<usize>`, not `Capability`. Stating a number commits the store to accepting exactly that many bytes and refusing one more as `AppendError::ExceedsStoreLimit { limit: StoreLimit::… }`, never `AppendError::Store`. CF-40's ownership stays contested and is consumed, not settled. | `crates/happenstance-testkit/src/contract.rs:213-279`; `.kb/open-questions/cf-40-fixture-limits-ownership.md` |
| **Capability constants are checked at codegen** | `Capability::declined("")` fires an `assert!` in a `const fn`, but for an *associated* const that lands at codegen: `cargo build` and `cargo test` catch it, `cargo clippy` does not. Do not conclude from a green clippy that the constants are well-formed. | `crates/happenstance-testkit/src/contract.rs:404-410` |
| **`ConcurrentFixture` at `CONTENDERS = 8`** | `pub trait ConcurrentFixture: Fixture<Store: Send> {}`. One instance hands out 8 handles onto **one** backing store. An opaque `impl Fixture` at the call site leaves `F::Store: Send` unprovable — the macro's own comment says so. Needs `tokio` `rt-multi-thread`, already in the crate's dev-deps. | `crates/happenstance-testkit/src/concurrency.rs:186,206,1140-1146`; `crates/happenstance-postgres/Cargo.toml:24-26` |
| **The fixture is mounted into the macro** | `event_store_conformance!(PostgresFixture::…)` from the crate's own `tests/`. The macro takes an expression, not a type, exactly so a fixture can be handed a connection URL rather than reaching into the environment for one. | `crates/happenstance-testkit/src/lib.rs:267-274,311-357` |
| **The gating is whole-invocation** | `#[ignore]`, `required-features`, or an env read inside the test binary — implementer's call. **Never** a `#[cfg]` hiding a rule out of a macro's expansion (DR-5). `cargo test --workspace --all-features` must exit zero on a machine with no server, and that command is part of `cargo xtask ci --fast`. | *Architecture brief* §7; *Testing brief* Notes §6; `.redkiln/config.yaml:55` |
| **The live job runs the full expansion** | Not a hand-picked subset — otherwise "no rule is absent from the run" is unproven by construction. `--show-output` so declined capabilities and their reasons land in the log. | *Testing brief* Notes §6 (the two project-level merge-gate commands) |
| **The live job cannot be green by running nothing** | The direct consequence of whole-invocation gating: the same flag that makes the default gate green with no server makes a misconfigured job green with no tests. The job must fail if the gated tests did not execute. | `project.md` *Risks* — "live-infrastructure jobs become flaky and get quietly weakened"; CF-18's argument, `crates/happenstance-testkit/src/contract.rs:230-235` |
| **The job is a sibling, not a step** | A new job in `.github/workflows/ci.yml` beside `gate`, `backlog`, `wasm-conformance`, `msrv`, `semver`, `advisories`. `xtask/src/main.rs`'s `REQUIRED` array is unchanged in kind, and `wasm_steps()` selects steps **by name** and panics on a miss — so nothing here may repoint `cargo xtask wasm`. | `.github/workflows/ci.yml:31,102,204,241,279,325`; `xtask/src/main.rs:265-282,784-790,816-823` |
| **A new dev-dependency must clear the licence allowlist** | `deny.toml` allows MIT / Apache-2.0 (incl. the LLVM exception) / BSD-2 / BSD-3 / ISC / Unicode-3.0 / Zlib, with `[graph] all-features = true`. Run `cargo deny check` against the pinned `testcontainers` graph **before** committing to the version. Growing the allowlist is a gate weakening and needs a recorded decision — not this story's to take. | `deny.toml:1-2,10-19`; *Architecture brief* §9.2 point 2 |
| **No literal position values anywhere** | The specification permits gaps and this is the project where gaps stop being hypothetical. Enforced by `cargo xtask lint-position-literals`, which runs in the default gate. | `xtask/src/main.rs:389,687`; CLAUDE.md *The rule that matters* |
| **The `todo!()`s stay** | `append`, `head` and `contains_event_id` are untouched by this story, and so is the scoped `#![allow(clippy::todo)]` that names phase 10. A green run of this PR's own tests proves the fixture and the plumbing, not the adapter. | `crates/happenstance-postgres/src/event_store.rs:136-174`; `crates/happenstance-postgres/src/lib.rs:58-63` |

## Data and migrations

**Real, and this story owns it.** `happenstance-postgres` carries exactly **one**
migration and this PR authors it.

**No backfill, and no compatibility window.** Neither crate has ever shipped a
schema to a real consumer — both are `publish = false` today
(`crates/happenstance-postgres/Cargo.toml:12`) — so "migration" here is a
schema-authoring concern, not a data-movement one (*Deployment brief*, *Migration
and backfill*). It stays **one** migration for as long as that is true: the
slice-mate that wires ADR-0024's mechanism adds its column to migration 1 rather
than opening a migration 2, and the one-migration discipline ends at first
publish, which is `publication-and-positioning`'s (HS-P0016) to decide.

**The column set this story authors** — every one of these is settled independent
of ADR-0024:

| Column | Shape | Why it is settled here |
| --- | --- | --- |
| `position` | `bigint PRIMARY KEY`, **not** `bigserial` | "a `serial` column is `nextval()`, and `nextval()` is where the invariant is lost" (`event_store.rs:22-24`). How the value is produced is ADR-0024's; that it is not a sequence default is already decided. |
| `event_type` | `text NOT NULL` | Prose schema, `event_store.rs:12` |
| `data` | `bytea NOT NULL` | Payloads are opaque `Bytes` (ADR-0003) |
| `metadata` | `bytea` (nullable) | Prose schema, `event_store.rs:14` |
| `tags` | Tag storage per the choice recorded above; `text[] NOT NULL`, canonically sorted, is the prose schema's lean | `event_store.rs:15`; `lib.rs:37-39` |
| `origin_store` | 16 bytes exactly, round-tripping unmodified (`bytea` with a length `CHECK`, or `uuid`) | `StoreId([u8; 16])` — `crates/happenstance-core/src/identity.rs:44` |
| `origin_position` | `bigint NOT NULL` | The other half of `EventId` — `identity.rs:97-125` |
| `recorded_at` | `bigint NOT NULL` (milliseconds since the epoch, may be negative) | `RecordedAt(i64)` — `identity.rs:158-172` |

**Indexes.** `event_tags_idx` on `tags` (GIN, if `text[]` is chosen) and
`event_type_idx` on `(event_type, position)`, both from the prose schema
(`event_store.rs:18-19`). Whether the append-condition SQL wants more is
`postgres-append-and-frontier-head`'s to find out against a real server.

**Deliberately absent.** No visibility-mechanism column. Under `xid8` +
`pg_snapshot_xmin` migration 1 gains an `xid8` column; under either lock arm it
gains nothing (*Deployment brief*, *Migration and backfill*). Adding one now
would encode ADR-0024's answer by accident, which is precisely what
`event_store.rs:158-159` warns against for `head`'s body and is no less true of a
schema.

## Acceptance criteria

The persona is the **adapter author** (`_storymap.md` preamble: "the 'user' here is
an adapter author and a consuming application, not a screen"), and the journey
slice is the one in the Context pack — the moment `happenstance-postgres` stops
being an instrument nobody can run. Each criterion crosses the full stack the
story owns: a SQL schema, a fixture that reaches it, a macro invocation, and a
place for that invocation to execute.

Verification note that binds every row below: **`append`, `head` and
`contains_event_id` are still `todo!()` after this story** (PR boundary). So no
row here is verified by a conformance *rule outcome* — those belong to
`postgres-append-and-frontier-head` and `postgres-concurrency-family`. What this
story proves at runtime, it proves through `PostgresEventStore::pool()`
(`crates/happenstance-postgres/src/event_store.rs:113`) and raw SQL; what it
proves at compile time, it proves through the default gate's
`clippy --workspace --all-targets --all-features -- -D warnings`
(`xtask/src/main.rs:115-129`), which compiles the new tests target and therefore
the macros' full expansion. See *Clarifications resolved during spec*, item 1.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | GIVEN an adapter author with an empty Postgres database and a checkout of this crate, WHEN a `PostgresFixture` instance brings its backing store up, THEN the event table exists — created from a versioned SQL source under `crates/happenstance-postgres/migrations/`, not from prose in a doc comment — carrying `position`, `event_type`, `data`, `metadata`, `tags`, `origin_store`, `origin_position` and `recorded_at` with the shapes the *Data and migrations* table fixes, plus the tag index and the `(event_type, position)` index. | `crates/happenstance-postgres/tests/postgres_conformance.rs::migration_1_creates_the_settled_column_set` — gated, live job. Applies migration 1 to a fresh instance and asserts the column names, data types and nullability out of `information_schema.columns`, and both indexes out of `pg_indexes`. |
| AC-002 | GIVEN ADR-0024 is undecided and is a deliverable of a *later* slice, WHEN the adapter author reads migration 1, THEN nothing in it has answered the position-visibility question by accident: `position` is `bigint` with **no** column default and is not `serial` / `bigserial` / `GENERATED … AS IDENTITY`, and the table carries **no** visibility-mechanism column (`xid8` or otherwise). | Same target, `migration_1_does_not_preempt_adr_0024` — asserts `column_default IS NULL` and `is_identity = 'NO'` for `position`, and asserts the column set is **exactly** the eight settled columns, so a ninth added in passing fails the test rather than passing silently. |
| AC-003 | GIVEN a suite run that constructs more than one fixture instance in one process, WHEN each instance's handle writes to the event table, THEN neither instance can observe the other's rows — one `PostgresFixture` instance is one isolated backing store, and the isolation scheme (schema-per-instance or database-per-instance) is recorded with its reason. | Same target, `two_fixture_instances_are_two_backing_stores` — two instances, a raw `INSERT` through each `store.pool()`, and a `SELECT count(*)` on each showing only its own row. This is the story-local stand-in for `rules::two_fixture_instances_observe_none_of_each_others_appends` (`crates/happenstance-testkit/src/suite.rs:210`), which the slice-mate turns green through `append`. |
| AC-004 | GIVEN `SECOND_HANDLE` is the one MUST among the capabilities and declining it makes `two_handles_observe_each_others_appends` **panic** quoting the fixture's own words, WHEN the adapter author opens two handles from one `PostgresFixture` instance, THEN both address the same backing store and each observes the other's writes — and each handle owns its own refcount rather than borrowing the fixture's lifetime. | Same target, `two_handles_from_one_instance_share_a_backing_store` — two `connect()`s, a raw `INSERT` through the first, a `SELECT` through the second. Compile-time half: `PostgresFixture::SECOND_HANDLE` is asserted `Capability::SUPPORTED` in a `const`-checked unit test, and `Fixture::Store` is a plain associated type (no GAT), which the trait's own signature enforces. |
| AC-005 | GIVEN a capability constant is an *answer* and a silent default is the failure mode, WHEN the adapter author reads `PostgresFixture`'s constants, THEN `SECOND_HANDLE`, `REOPEN` and `MID_BATCH_FAULT` each carry a deliberate value with a non-empty, Postgres-specific reason where declined, `MAX_EVENT_DATA_LEN` / `MAX_TAGS_PER_EVENT` / `MAX_EVENTS_PER_BATCH` are stated as honest `Option<usize>` facts, and CF-40's contested ownership is consumed rather than settled here. | Same target, `capability_constants_are_answered_not_defaulted` — pins all six values, asserts every declined reason is non-empty and names a Postgres-specific cause, and asserts `MID_BATCH_FAULT` is not left at the trait's provided default by comparing against it explicitly. Plus, where a capability is claimed supported, a live exercise of it: `reopen_survives_a_reopen` (raw row written, `reopen()`, row still there) and/or `mid_batch_fault_actually_faults`. `cargo build` — not `cargo clippy` — is what catches a `declined("")` (`crates/happenstance-testkit/src/contract.rs:404-410`). |
| AC-006 | GIVEN the concurrency family starts `CONTENDERS = 8` contenders and a fixture whose pool is smaller than that **deadlocks rather than fails**, with no watchdog to tell the two apart (CF-33), WHEN the adapter author points `event_store_concurrency_conformance!` at `PostgresFixture`, THEN it type-checks with `F::Store: Send` provable at the call site, and one fixture instance genuinely hands out `CONTENDERS` simultaneous live handles onto one backing store without deadlocking. | Compile-time: the `event_store_concurrency_conformance!(PostgresFixture::…)` invocation in the mount file, compiled by the default gate's clippy step — an opaque `impl Fixture` fails to compile here (`crates/happenstance-testkit/src/concurrency.rs:1140-1146`). Runtime: same target, `contenders_handles_open_concurrently_without_deadlock` — `concurrency::CONTENDERS` concurrent `connect()`s under `tokio` `rt-multi-thread`, each executing a trivial query, all held simultaneously. |
| AC-007 | GIVEN "a fixture no macro is invoked with is not delivered," WHEN the adapter author looks for where the Postgres suite runs, THEN `crates/happenstance-postgres/tests/postgres_conformance.rs` exists and invokes `event_store_conformance!(PostgresFixture::…)` and `event_store_concurrency_conformance!(…)` over their **full expansion** — no hand-picked subset — so that every rule the suite defines is present in the binary and reports pass, fail or a declared skip. | Compile-time: the default gate's clippy step compiles both expansions. Runtime: the live job runs `cargo test -p happenstance-postgres --all-features -- --ignored --list` and asserts every name `for_each_rule!`/`for_each_concurrency_rule!` emits is present, modelled on `xtask/src/proof.rs:199-217`'s `--list` assertion. Rule *outcomes* are the slice-mates'. |
| AC-008 | GIVEN every other project in this initiative is gated by `cargo xtask affected` and `cargo xtask ci --fast` (`.redkiln/config.yaml:40,55`), WHEN a maintainer of any of them runs the default gate on a laptop with the Docker daemon stopped and the network unplugged, THEN it is green — including the `tests` step's `cargo test --locked --workspace --all-features -- --show-output` — and the new tests target still **compiles and lints** rather than being cfg'd out of existence. | `cargo xtask ci --fast` executed with Docker stopped and networking disabled, transcript cited in the implementation report. Machine half: the default run's output shows the gated tests as `ignored`, not as `0 tests` — an absent target and a skipped one are different observations and only one of them is acceptable. |
| AC-009 | GIVEN live infrastructure must not enter `xtask/src/main.rs`'s `REQUIRED` array, WHEN CI runs on this PR, THEN a new job in `.github/workflows/ci.yml` sits **beside** `gate` (`:31`), `backlog` (`:102`), `wasm-conformance` (`:204`), `msrv` (`:241`), `semver` (`:279`) and `advisories` (`:325`) — not as a step inside `gate` — starts a `testcontainers` Postgres pinned to a specific minor, and runs the gated tests with `--show-output` so every declined capability's reason lands in the log. | The job's own run on this PR (workflow run URL cited in the ledger), plus a reviewed diff showing `xtask/src/main.rs`'s `REQUIRED` array and `wasm_steps()`'s by-name selection (`xtask/src/main.rs:784-790,816-823`) untouched. `cargo xtask ci` on the same tree enumerates the same step names it did before. |
| AC-010 | GIVEN the same flag that makes the default gate green with no server makes a misconfigured live job green with no tests, WHEN the container fails to start or the gating flag is wrong, THEN the live job **fails** rather than reporting success on `running 0 tests` — it cannot be green without having executed the gated tests. | The job asserts a nonzero executed-test count (the `--list` assertion of AC-007 plus a post-run count check), modelled on `xtask/src/proof.rs:199-231`, which exists in-tree for exactly this failure ("`cargo test` exits 0 on `running 0 tests`, so an emptied file passes a step that a deleted one fails"). Evidenced by a deliberate negative exercise: the job's command run with the gate flag off, shown failing, transcript in the implementation report. |

**Coverage of the traced project AC.** `project.md` **AC-011** ("the default gate
stays Docker-free and network-free; the Postgres and Neon suites live in their own
CI jobs") is discharged for the Postgres half by **AC-008** (the gate stays green
without infrastructure), **AC-009** (the suite lives in its own job) and **AC-010**
(that job is not vacuously green). AC-001 – AC-007 are the substrate without which
AC-009's job would have nothing to run — the *Testing brief*'s AC-011 row calls
this "the one AC proven by the **absence** of infrastructure rather than its
presence," and absence only proves something when the presence exists somewhere
else. The Neon half of AC-011 is `neon-fixture-and-live-job`'s and
`neon-sql-transport`'s.

## Interaction quality

**Composition family — N/A by sign-off.** `_design.md` records this project as
having no user-facing surface (signed off 2026-08-12: "N/A — no user-facing
surface" under *Surfaces*, *Items*, *Signatures*, *Shape decision* and *Placement
and re-export*), and this story renders none. There is no composition, transience
policy, density budget, hierarchy or named anti-pattern to honour, and
`design.capture` is deliberately absent from `.redkiln/config.yaml`, which makes
the perceptual review a declared skip rather than a silent pass. No AC row carries
a composition invariant, and inventing one here would be re-deciding a design a
human already signed off as absent.

**State family — applicable, in the only medium this repository has.** The
"surface" an adapter author meets is a command's output and a CI job's log. The
state invariants translate directly, and each is carried by a numbered AC row
rather than a bullet, so `redkiln verify` extracts it:

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not a context jump** — landing this story must not change what any *other* project's contributor has to do to run the gate. No new tool, no new daemon, no new credential enters the default path. | **AC-008** | `cargo xtask ci --fast` green with Docker stopped and the network unplugged; `.redkiln/config.yaml:40,55`'s two grains unchanged. |
| **Non-occlusion** — nothing this story adds may hide information the run already surfaced. A declined capability's reason must remain visible, and a gated test must appear as *ignored* rather than vanishing from the count. | **AC-005**, **AC-008**, **AC-009** | Declined reasons asserted non-empty and Postgres-specific (AC-005) and printed by `--show-output` in the live job (AC-009); the default run shows `ignored`, not `0 tests` (AC-008). |
| **Preserved position** — the default gate's step list, its names and its selection-by-name must survive unchanged, because `wasm_steps()` selects by name and panics on a miss. | **AC-009** | Reviewed diff plus `cargo xtask ci`'s enumerated step names (`xtask/src/main.rs:784-790,816-823`). |
| **Reversibility** — a fixture instance's effects are confined to its own isolated backing store, so a failed or abandoned run leaves nothing behind that poisons the next one. | **AC-003** | Two instances, disjoint rows; the isolation scheme recorded with its reason. |
| **Reachability** — the capability is reachable by the command an adapter author would actually type, not only by a bespoke incantation. | **AC-007**, **AC-009** | `cargo test -p happenstance-postgres --all-features -- --ignored --show-output` is the documented command and is what the job runs. |
| **Honest failure state** — the run must be unable to report success for a state that is not success. | **AC-010** | Nonzero executed-test count asserted; negative exercise recorded. |

The last row is the one that would be satisfied by a perfectly unstyled render's
equivalent here: a job that exits zero having compiled everything and executed
nothing looks identical, in a green check mark, to a job that ran the suite. AC-010
is what makes that fail.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | The Docker daemon is unavailable, the pinned image cannot be pulled, or the container is up but not accepting connections when `connect()` is called. | `connect` **panics** — it has no `Result` by design (`crates/happenstance-testkit/src/contract.rs:309-321`) — with a message that makes "this is a broken test environment" unmistakable and does not read as "the adapter is wrong." This is the first fixture in the workspace where those two are genuinely different events. In the live job the panic must fail the job (EC-006), never be swallowed. |
| **EC-002** | The fixture's `PgPool` is sized below `concurrency::CONTENDERS`. | Forbidden. The concurrency doc is explicit: "a fixture whose pool is smaller than this deadlocks rather than failing, and the suite has no way to tell them apart (CF-33 — there is no watchdog)" (`crates/happenstance-testkit/src/concurrency.rs:204-209`). Size the pool at `CONTENDERS` **plus** headroom for `SECOND_HANDLE`'s extra handle and any bookkeeping connection, derive it from the constant rather than hard-coding `8`, and prove it with AC-006's runtime test. A hang here reads as a CI timeout, which is the least diagnosable failure this story can ship. |
| **EC-003** | A capability constant is written `Capability::declined("")`. | Rejected by an `assert!` in a `const fn`, but for an *associated* const that fires at **codegen** — `cargo build` and `cargo test` catch it, `cargo clippy` does not (`crates/happenstance-testkit/src/contract.rs:404-410`). Do not conclude from a green clippy that the constants are well-formed; AC-005's test is the belt to that suspender. |
| **EC-004** | Migration 1 fails, is applied twice, or is applied to the wrong schema for a fixture instance. | Must fail at fixture construction with a message naming the environment, not partway through a conformance rule. A rule that fails because the table is missing is an environment failure wearing an adapter defect's clothes, which is exactly the confusion `connect`'s panic-not-`Result` signature exists to prevent. Whatever isolation scheme AC-003 picks, applying the migration must be idempotent-or-loud per instance. |
| **EC-005** | The pinned `testcontainers` dependency graph fails `cargo deny check` under `[graph] all-features = true`. | Report it as a finding and pick a different pin or a different crate. **Do not** widen `deny.toml`'s allowlist (MIT / Apache-2.0 incl. the LLVM exception / BSD-2 / BSD-3 / ISC / Unicode-3.0 / Zlib — `deny.toml:1-2,10-19`). Growing it is a gate weakening and needs a recorded decision that is not this story's to take (*Architecture brief* §9.2 point 2). |
| **EC-006** | The live job's runner has no Docker, or the gating flag is misspelled so the gated tests never execute. | The job **fails**. It may not skip, may not `continue-on-error`, and may not be made non-blocking — DR-9 and the risk table are explicit that a flaky live job is fixed or reported, never quietly weakened. This is AC-010's whole content. |
| **EC-007** | `required-features` is chosen as the gating mechanism. | It does not work here, and the reason is mechanical: the default gate runs `cargo test --locked --workspace --all-features` and `cargo clippy --workspace --all-targets --all-features` (`xtask/src/main.rs:115-152`), and `--all-features` *enables* the required feature, so the target builds **and runs** against a server that is not there. Only `#[ignore]` (run in the live job with `-- --ignored`) — or an env read that fails loudly rather than passing quietly, paired with AC-010's count assertion — satisfies AC-008. The *Architecture brief* §7 lists all three as admissible; one of them is not, and that is a finding this story records. |
| **EC-008** | An env-var gate is chosen and its "no server" path returns early and **passes**. | Indistinguishable in CI output from a rule that passed — the same argument CF-18 makes about an omitted skip. If this mechanism is chosen, AC-010's nonzero-count assertion is not optional but load-bearing, and the early-return path must print a line that a human reading the log can tell apart from a pass. |
| **EC-009** | A conformance rule is hidden from a macro's expansion by `#[cfg]` to make the default gate green. | **Forbidden by DR-5.** Gating a whole invocation on infrastructure availability is a different act from making one rule vanish, and only the second is forbidden. AC-007's `--list` assertion is what would catch it. |

## Non-functional

| id | requirement | why, and where it is checked |
| --- | --- | --- |
| **NF-001** | The default gate's wall clock must not materially change. The new tests target adds compilation and linting, not execution, to `cargo xtask ci --fast`. | It is the bar every other project in the initiative meets on every story (`.redkiln/config.yaml:40,55`). Observed against the pre-change baseline in the implementation report. |
| **NF-002** | Per-fixture-instance setup must be cheap enough that the suite's many instances do not turn the live job into a timeout. | The conformance suite constructs a fixture instance per rule; a container per instance is admissible only if measured and survivable. Schema-per-instance against one shared container, or `CREATE DATABASE … TEMPLATE`, are the cheaper arms. Whichever is chosen, record the per-instance cost and the job's total wall clock. |
| **NF-003** | The Postgres image is pinned to a specific minor, never a floating tag. | An unpinned image makes a red job unattributable to a change in this repository — `RUNBOOK.md:4356-4358` and `project.md`'s in-scope bullet both say "pinned to a specific Postgres minor". |
| **NF-004** | `cargo deny check` stays green with `[graph] all-features = true`, with no allowlist growth. | `deny.toml:1-2,10-19`; the tool resolves on this machine so the step runs rather than printing `skipped` (CLAUDE.md *Commands*). |
| **NF-005** | The Postgres job requires **no credential**. | Unlike the Neon job, `testcontainers` needs only a daemon, so this job can run on PRs without secrets. Say so in the workflow, because the Neon job's credential handling is a different story's problem and conflating them is how a secret ends up gating a container. |
| **NF-006** | `testcontainers` and its graph must build on **1.97.1**. | The `msrv` CI job runs `cargo hack check --no-dev-deps --rust-version` (which hides dev-deps) **and then** a full `cargo test --workspace --all-features` at 1.97.1, which does not. `testcontainers` is a dev-dependency and lands squarely in the second half. Five of the five database crates in this workspace declare no `rust-version` at all, so only running the compiler finds this (CLAUDE.md, binding constraint 5). |
| **NF-007** | The panic message from `connect` must be diagnosable from the CI log alone. | The whole point of the panic-not-`Result` signature. A message that says "connection refused" without naming the fixture, the container and the URL costs a maintainer a local reproduction they should not need. |

## Implementation notes (non-prescriptive)

**`ConcurrentFixture` needs no second impl — the trait is blanket-implemented.**
`impl<F> ConcurrentFixture for F where F: Fixture, F::Store: Send {}` sits at
`crates/happenstance-testkit/src/concurrency.rs:188-194`. So the *Testing brief*'s
"two fixture-shaped things, or one type implementing both traits" (§3) is a
statement about the **call site**, not about writing two impls: what you owe is a
`Fixture::Store` that is `Send` — `PostgresEventStore` holds a `PgPool`, which is
`Send + Sync` (`crates/happenstance-postgres/src/event_store.rs:99,118`) — and a
**named** fixture type at the macro invocation. An opaque `impl Fixture` returned
from a helper is what breaks it, because an opaque type carries only the bounds
written on it. Verify this before writing a second trait impl you do not need.

**Isolation, three arms, all admissible.** One shared container plus
schema-per-instance (`CREATE SCHEMA`, `search_path` on the pool's
`after_connect`); one shared container plus `CREATE DATABASE … TEMPLATE t` per
instance; or a container per instance. The first is cheapest and interacts with
the migration mechanism: if `sqlx::migrate!` is turned on, its `_sqlx_migrations`
bookkeeping table is per-schema, which is either convenient or surprising
depending on the arm. §9.3 leaves this open; AC-003 closes it and asks for the
reason in writing.

**Turning on `sqlx`'s `migrate` feature is a manifest decision, not an
incidental one.** The manifest currently disables it with a stated reason ("the
schema is prose in `event_store.rs` until phase 10 makes it real",
`crates/happenstance-postgres/Cargo.toml:18-22`). Phase 10 is here. The
alternative — `include_str!` the SQL and execute it from the fixture — keeps the
published dependency surface exactly as it is today, which matters because
`deskeleton-and-package-readiness` removes `publish = false` later. Either way,
update the comment: leaving a manifest comment that says the schema is prose,
once it is not, is how the next reader is misled.

**`#[ignore]` composes with `--all-features`; `required-features` does not.**
See EC-007. `#[ignore]` also gives the live job a one-word invocation
(`-- --ignored`) that matches the *Testing brief* Notes §6's stated command
verbatim, and keeps the target compiled and linted by the default gate — which is
what stops the mount from rotting.

**The nonzero-count assertion already has an in-tree pattern.**
`xtask/src/proof.rs:199-217` lists a target's tests with `cargo test -- --list`,
asserts the names the gate cares about are present, and only then runs them —
written because "`cargo test` exits 0 on `running 0 tests`, so an emptied file
passes a step that a deleted one fails" (`xtask/src/proof.rs:12-15`). AC-007 and
AC-010 are the same problem one layer out. Reuse the shape; do not add a new
`ARTEFACTS` row, which would put a live server inside the default gate.

**Do not repoint anything the gate selects by name.** `wasm_steps()` selects
`REQUIRED` steps by name and panics on a miss (`xtask/src/main.rs:784-790,816-823`),
and that panic exists because an index-selected step was once found pointing at
the wrong thing. Adding a CI job is not a reason to touch either.

**Ceilings: measure before you state.** A stated `MAX_EVENT_DATA_LEN` commits the
store to accepting exactly that many bytes and refusing one more as
`AppendError::ExceedsStoreLimit`. With `append` still `todo!()`, this story cannot
*prove* a ceiling through the rule; state the number the schema and driver actually
imply (Postgres's 1 GB field limit, `bind` limits, whatever the tag storage
imposes), record how it was derived, and let `postgres-append-and-frontier-head`'s
`append_reports_exceeded_store_limits` be the thing that finds you wrong. An
`Option<usize>` of `None` is an honest answer; a guessed number is not.

**Read `MemoryFixture` before writing this one**
(`crates/happenstance-testkit/src/fixtures.rs:219-292`), and read
`MemoryFixture::sharing` (`:252-267`) as the in-tree implementation that
deliberately *fails* the two-instances isolation rule — it is the wrong
implementation AC-003 exists to reject.

## Tests and CI (merge gate)

Two lists, not one, and that split is the story (*Testing brief* Intent).

| tier | command / path | proves |
| --- | --- | --- |
| **Static — format & lint** | `cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` (`xtask/src/main.rs:106-129`) | The new tests target **compiles**, including both macro expansions — which is the compile-time half of AC-006 and AC-007. An opaque `impl Fixture` or a non-`Send` `Store` fails here, in the default gate, with no server involved. |
| **Unit / integration, tree-local** | `cargo test --locked --workspace --all-features -- --show-output` (`xtask/src/main.rs:132-153`) | AC-008: exits zero with no Docker and no network, reporting the gated tests as `ignored`. Also runs the const-level capability assertions that do not need a server. |
| **Static — supply chain** | `cargo deny check` (OPTIONAL, and the tool resolves on this machine so it runs) | NF-004 / EC-005: the pinned `testcontainers` graph clears the existing allowlist without growing it. |
| **Static — position literals** | `cargo xtask lint-position-literals` (`xtask/src/main.rs:389,687`) | CF-6. No literal position values enter the new test code; the specification permits gaps and this is the project where gaps stop being hypothetical. |
| **Gate — story grain** | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | The story bar redkiln wires. Only what this diff could break. |
| **Gate — project grain** | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | The non-terminal project bar, and the command AC-008 is executed under with the daemon stopped. |
| **Live — conformance mount** | *(new job)* `cargo test -p happenstance-postgres --all-features -- --ignored --list` | AC-007: every rule name both macros emit is present in the binary. Runs **before** the tests, so "no rule is absent from the run" is proven rather than assumed. |
| **Live — this story's own tests** | *(new job)* `cargo test -p happenstance-postgres --all-features -- --ignored --show-output` against a pinned `testcontainers` Postgres (*Testing brief* Notes §6, project-level command) | AC-001 – AC-006: the schema, the isolation, the two handles, the capability answers, and `CONTENDERS` handles open at once. `--show-output` so declined capabilities' reasons land in the log (AC-009). |
| **Live — vacuity guard** | *(new job step)* nonzero executed-test count, modelled on `xtask/src/proof.rs:199-231` | AC-010: the job cannot be green having run nothing. |
| **MSRV** | the existing `msrv` job (`.github/workflows/ci.yml:241`) | NF-006: `testcontainers` builds at 1.97.1, which `--no-dev-deps` would have hidden and the full `cargo test` half does not. |

The live conformance families themselves — `event_store_conformance!`'s and
`event_store_concurrency_conformance!`'s rule *outcomes* — turn green in
`postgres-append-and-frontier-head` and `postgres-concurrency-family`, in the same
slice and the same context. This story's merge bar is the *Merge DoD* in the PR
boundary, not those outcomes.

## Risks and coupling (PR-scoped)

| risk | why it is live in this PR | mitigation in this PR |
| --- | --- | --- |
| **The live job is green because it ran nothing.** | The direct consequence of whole-invocation gating: one flag makes the default gate green with no server and a misconfigured job green with no tests. It is the quieter sibling of the risk table's "live-infrastructure jobs become flaky and get quietly weakened" (`project.md` *Risks*). | AC-010, with the in-tree precedent (`xtask/src/proof.rs`) and a deliberate negative exercise recorded, not merely a claim. |
| **A pool smaller than `CONTENDERS` deadlocks instead of failing.** | CF-33: no watchdog. The symptom is a CI timeout, the least diagnosable failure available. | EC-002 and AC-006's runtime test; the pool size derived from `concurrency::CONTENDERS`, not from the literal `8`. |
| **`required-features` looks admissible and is not.** | The brief lists it as one of three choices; `--all-features` in the default gate defeats it. An implementer who picks it discovers this only when the gate goes red on someone else's laptop. | EC-007 states the mechanism and the reason; AC-008 is executed with the daemon actually stopped rather than reasoned about. |
| **Migration 1 quietly answers ADR-0024.** | A `bigserial` here, or an `xid8` column added "while we are in there", makes a later decision record describe a schema that already chose. `event_store.rs:22-24` warns about exactly this. | AC-002 asserts no default, no identity, and an **exact** eight-column set, so a ninth column fails a test rather than passing review. |
| **A green `clippy` is read as proof the constants are well-formed.** | `Capability::declined("")` fires at codegen, which clippy does not reach (`contract.rs:404-410`). | EC-003 and AC-005; the gate's `tests` step is the one that catches it, and it already runs. |
| **`testcontainers` drags in a licence the allowlist refuses, or a crate that will not build at 1.97.1.** | A new dev-dependency crosses two gates that are easy to forget until CI says so. | EC-005 (report, do not widen) and NF-006; run `cargo deny check` against the candidate **before** committing to the pin. |
| **Scope creep into `append`.** | The fixture is only satisfying to write against a working store, and `append` is one file away. | The PR boundary forbids it and `postgres-append-and-frontier-head` owns it. The `todo!()`s and the scoped `#![allow(clippy::todo)]` (`crates/happenstance-postgres/src/lib.rs:58-63`) are asserted still present at merge. |
| **Coupling: the slice-mate inherits every choice made here.** | Tag storage, isolation scheme, migration mechanism and the ceilings all constrain `postgres-append-and-frontier-head`'s SQL and `postgres-projection-store`'s transaction handling. | Each open choice is recorded *with its reason* (AC-001, AC-003, AC-005) rather than merely made, so the slice-mate can disagree with an argument instead of a fact. The slice is implemented in one context precisely so that disagreement is cheap. |
| **Coupling: `crates/happenstance-neon` inherits migration 1.** | The two crates are one project because Neon is Postgres over one-shot HTTP, inheriting the schema, the tag storage and the append-condition SQL (`project.md` *Coupling notes*). | Nothing in migration 1 may assume an interactive transaction or a cursor. Neon's stories are downstream and out of this PR, but the schema is not. |

## Dependencies

**Blocks on:** *(none)* — `depends_on: []`. This is the first story of the
`postgres-live-suite` slice and the second story of the project overall; only
`claim-crate-names` precedes it, and that edge is a merge-order convention
(`_storymap.md` *Merge order* step 1), not a `depends_on`. Nothing here needs
`projection-store-freeze` (HS-P0010): that project-level `blocked_by` gates
`postgres-projection-store`, not the schema or the fixture.

**Unlocks:**

- `postgres-append-and-frontier-head` — needs the schema to write `append` against
  and the fixture to run the two conformance families through. Its `depends_on`
  names this story directly (`_storymap.md`, row 3).
- `postgres-concurrency-family` — needs `ConcurrentFixture` at `CONTENDERS = 8`
  and the live job to run it in (transitively, via
  `postgres-append-and-frontier-head`).
- `postgres-rule-controls` — needs the live job as the place its mutant control
  and its naive-`nextval()` arm can reach a real server without entering
  `cargo test --workspace` (transitively).
- `postgres-projection-store` — `depends_on` names this story directly
  (`_storymap.md`, row 12): the projection store needs the same container, the
  same isolation scheme and the same job.
- Downstream of the whole slice, `adr-0024-position-visibility-mechanism` cannot
  re-measure against a real adapter without a server to measure on.

## Anchors (progressive disclosure)

Open these when the bound AC is what you are working on. Everything load-bearing
that is not already distilled in the *Context pack* is here; nothing here is
optional at the moment it is named.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `crates/happenstance-postgres/src/event_store.rs` | Lines 9-24 are the prose schema this story turns into SQL, and the sentence that keeps `position` off `bigserial` ("`nextval()` is where the invariant is lost"). Lines 26-70 are the three candidate mechanisms — read to know what **not** to encode. Lines 93-118 give `PgPool`, `new`, the public `pool()` accessor every story-local runtime test goes through, and why the impl is `SendEventStore`. | Before writing a line of migration SQL, and again before writing any story-local test that needs to reach the database. | AC-001, AC-002, AC-004 |
| `crates/happenstance-testkit/src/contract.rs` | The `Fixture` trait in full: `:97-111` the rustc ICE that forbids the GAT shape; `:135-161` `SECOND_HANDLE` as the only MUST and the panic a decline causes; `:167-173` `REOPEN`; `:207-211` `MID_BATCH_FAULT`'s declined default; `:213-279` the three ceilings and what stating a number commits you to; `:281-307` `arm_mid_batch_fault`'s provided body that panics if you claim support without overriding it; `:309-321` why `connect` panics; `:404-410` the codegen-time `declined("")` assert. | Before writing `impl Fixture for PostgresFixture` — and specifically before choosing any constant's value. | AC-004, AC-005 |
| `crates/happenstance-testkit/src/fixtures.rs` | `MemoryFixture` is the reference shape: `:219-292` the whole impl, `:286-291` `connect` as an `Arc` clone (the owning-refcount shape a `PgPool` clone mirrors), `:252-267` `MemoryFixture::sharing` — the in-tree fixture that deliberately **fails** the two-instances isolation rule, i.e. the wrong implementation AC-003 must reject. | Read `:219-292` before writing the fixture; read `:252-267` before choosing the isolation scheme. | AC-003, AC-004 |
| `crates/happenstance-testkit/src/concurrency.rs` | `:186-194` `ConcurrentFixture` **and its blanket impl** — the fact that changes the implementation plan (no second impl is owed). `:204-209` `CONTENDERS = 8` and the CF-33 warning that an undersized pool deadlocks with no watchdog. `:1129-1165` the macro and the comment explaining why an opaque `impl Fixture` leaves `F::Store: Send` unprovable. | Before writing the concurrency mount, and before choosing the pool size. | AC-006 |
| `crates/happenstance-testkit/src/lib.rs` | `:267-274` why `event_store_conformance!` takes an **expression building a fixture** rather than a type — written with a Postgres connection URL as the motivating example, i.e. this exact fixture. `:311-357` the macro itself. `:189` the public `rules::<name>` seam a later slice-mate uses. | When writing the invocation in the mount file. | AC-007 |
| `crates/happenstance-postgres/Cargo.toml` | `:12` `publish = false` (stays). `:18-22` the comment disabling `sqlx`'s `migrate` feature *and the reason it gives*, which this story either satisfies or must update. `:24-26` the existing dev-dependencies, including `tokio` with `rt-multi-thread` already present for AC-006. | Before adding `testcontainers` or turning on `migrate`. | AC-001, AC-006 |
| `crates/happenstance-core/src/identity.rs` | `:44` `StoreId([u8; 16])`, `:97-125` `EventId` as the `(StoreId, SequencePosition)` pair, `:158-172` `RecordedAt(i64)` as milliseconds admitting times **before** the epoch, and `:414` the core test that names that case — which is why `recorded_at` is `bigint` and not `timestamptz`. | While writing the identity and time columns of migration 1. | AC-001 |
| `xtask/src/main.rs` | `:105-153` the `REQUIRED` array's first three steps — the exact `--all-features` invocations that make `required-features` gating unworkable (EC-007) and that compile the new tests target for free. `:784-790,816-823` `wasm_steps()` selecting by name and panicking on a miss. `:389,687` the position-literal lint. | Before choosing the gating mechanism, and before touching anything in `xtask`. | AC-007, AC-008, AC-009 |
| `xtask/src/proof.rs` | `:12-15` the argument in full — "`cargo test` exits 0 on `running 0 tests`, so a file truncated to its `#![cfg(…)]` attributes passes." `:199-231` the `--list`-then-assert implementation. This is the in-tree pattern AC-007 and AC-010 reuse; do not invent a second one. | When writing the live job's vacuity guard. | AC-007, AC-010 |
| `.github/workflows/ci.yml` | The six sibling jobs the new one joins — `gate` `:31`, `backlog` `:102`, `wasm-conformance` `:204`, `msrv` `:241`, `semver` `:279`, `advisories` `:325` — and their shape: toolchain pinning, caching, and how each declares what it needs. `msrv` `:241` is also where NF-006 bites. | Before adding the job; copy a sibling's skeleton rather than inventing one. | AC-009, AC-010 |
| `deny.toml` | `:1-2` `[graph] all-features = true`; `:10-19` the exact licence allowlist a `testcontainers` graph must clear. The measured in-tree precedent for how this bites is `crates/happenstance-postgres/src/lib.rs:38-47` — `sqlx`'s `tls-rustls` fails on `webpki-roots`, *not* on `ring`. | Before pinning a `testcontainers` version. | AC-001 (EC-005) |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md` | *Architecture brief* §4 (migration 1 and the manifest decision), §6 (both fixtures, capability by capability, and the two mechanics that bite), §7 (gate and CI wiring, and DR-5's one hard constraint), §9.3 (the genuinely open choices this story closes). *Testing brief* Notes §3 (no in-tree reference for a live `ConcurrentFixture`), §6 (the two merge-gate command lists, verbatim), §7 (the fixture inventory and "nothing is mocked"). | §6 before the fixture; §7 before the CI job; §9.3 when a choice feels arbitrary — it probably is one this brief deliberately left you. | AC-003, AC-005, AC-006, AC-008, AC-009 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/project.md` | DR-9 and AC-011 in the charter's own words, and the risk row "live-infrastructure jobs become flaky and get quietly weakened" with its mitigation ("fixed or reported, never made non-blocking without a recorded decision"). | Before proposing any weakening of the live job. | AC-008, AC-009, AC-010 |
| `.kb/open-questions/cf-40-fixture-limits-ownership.md` | The contested ownership behind the three ceilings. DR-6's instruction is **consume, do not settle**: if no sibling has settled it when this lands, state the ceilings honestly and record the unresolved ownership. | When writing the three `Option<usize>` constants. | AC-005 |
| `.kb/decisions/0013-position-assignment-and-visibility.md` | The accepted atom establishing ES-10 as a **global** invariant — the premise ADR-0024 inherits without owning. Read so that nothing in migration 1 quietly assumes a per-boundary reading. | Before deciding that a column or an index "obviously" belongs to visibility. | AC-002 |
| `.redkiln/config.yaml` | `:40` and `:55` — the two verify grains (`cargo xtask affected`, `cargo xtask ci --fast`) that every other project in this initiative is gated by, and which AC-008 protects. Also the deliberately absent `design.capture` that makes the perceptual review a declared skip. | Before concluding that "one container in the default gate would be fine". | AC-008 |
| `RUNBOOK.md` | `:4350-4362` phase 10's work items, including "`testcontainers` for the fixture, pinned to a specific Postgres minor, in **its own CI job**". `:249-253` phase 4's identity and time columns as migration-1 columns in every store. `:606` the ES-10/ES-11/ES-12 residual-exposure row this slice exists to discharge. | For the plan-of-record reading of what this phase owes. | AC-001, AC-009 |

## Clarifications resolved during spec

1. **This story's ACs are not verified by conformance rule outcomes, and that is
   deliberate.** `append`, `head` and `contains_event_id` stay `todo!()` (PR
   boundary), so every rule in `event_store_conformance!`'s expansion would panic
   if run. The story therefore proves its fixture through two channels that are
   available now: **compilation** of the mount file under the default gate's
   `clippy --all-targets --all-features -- -D warnings` (which is a real proof —
   an opaque `impl Fixture` or a non-`Send` `Store` fails there), and **story-local
   gated tests** that reach the database through the public
   `PostgresEventStore::pool()` accessor (`crates/happenstance-postgres/src/event_store.rs:113`)
   with raw SQL. Rule outcomes belong to `postgres-append-and-frontier-head` and
   `postgres-concurrency-family`, in the same slice and the same context. The
   slice's merge bar is the whole suite green; this story's is the *Merge DoD* in
   the PR boundary.
2. **`required-features` is struck from the admissible gating mechanisms.** The
   *Architecture brief* §7 and *Testing brief* Notes §6 both list it beside
   `#[ignore]` and an env read. It cannot work: the default gate runs
   `cargo test --locked --workspace --all-features` and
   `cargo clippy --workspace --all-targets --all-features`
   (`xtask/src/main.rs:115-152`), and `--all-features` enables the required
   feature, so the target builds *and executes* against a server that is not
   there. Recorded as **EC-007** rather than silently avoided, because the briefs
   say otherwise and the next reader deserves the reason. `#[ignore]` is the
   mechanism that composes with `--all-features` and matches the *Testing brief*'s
   own project-level command (`-- --ignored --show-output`) verbatim.
3. **`ConcurrentFixture` is blanket-implemented, so no second impl is owed.**
   `impl<F> ConcurrentFixture for F where F: Fixture, F::Store: Send {}`
   (`crates/happenstance-testkit/src/concurrency.rs:188-194`). The *Testing brief*
   Notes §3's "two fixture-shaped things, or one type implementing both traits" is
   a call-site requirement — a **named** fixture type rather than an opaque
   `impl Fixture` — not an instruction to write two impls. What the story actually
   owes is a `Send` `Store` (satisfied: `PgPool` is `Send + Sync`) and a pool sized
   at or above `CONTENDERS`. Reflected in AC-006 and the implementation notes.
4. **The ceilings are stated but not proven here, and the spec says so.** Stating
   a number commits the store to a refusal path (`AppendError::ExceedsStoreLimit`)
   that only a working `append` can exercise. AC-005 therefore requires the numbers
   to be *derived and recorded*, not *demonstrated*; `append_reports_exceeded_store_limits`
   is what finds them wrong, and it belongs to the slice-mate. `None` is an honest
   answer; a guessed number is not.
5. **The AC set is exactly the ten ids the first pass decided** — AC-001 … AC-010.
   None added, none dropped. AC-008, AC-009 and AC-010 carry project **AC-011**;
   AC-001 – AC-007 are the substrate that gives AC-009's job something to run, and
   the coverage note under the acceptance table says so explicitly.
6. **No AC row carries a composition invariant.** `_design.md` records this project
   as having no user-facing surface, signed off 2026-08-12, and this story renders
   none. The *Interaction quality* section maps the applicable **state**-family
   invariants onto existing AC rows rather than minting prose bullets, because
   `redkiln verify` extracts ACs from table cells and bullets — a blocking
   invariant written only as prose is never gated and never tested.
