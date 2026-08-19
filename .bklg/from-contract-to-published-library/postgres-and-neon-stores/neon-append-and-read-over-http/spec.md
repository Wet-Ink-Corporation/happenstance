---
item: HS-S0069
stage: spec
created: 2026-08-12T13:47:07.871Z
updated: 2026-08-12T13:47:07.871Z
template_sig: 87bbf1d0
rendered_sig: 79c0b875
---

# Spec — The suite runs over one-shot HTTP

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DoD 6 at `:375-376` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project item | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/project.md` |
| Project briefs (architecture / testing / deployment) | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md` — Architecture §2 (composition roots, `:107-164`), §5 (Neon data flow, `:240-278`), §6 (fixtures, `:279-331`), §7 (gate wiring, `:332-363`), §9.2 (the transport seam, `:420-442`); Testing §6 (`:596-630`), §7 (`:632-656`); Deployment *Notes* (`:692-771`) |
| Project grounding | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_grounding.md` |
| Signed-off design | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_design.md` — **no user-facing surface**, and that determination is what a human approved (`_design.md:10-41`, `:86-95`) |
| Story map (this story's row) | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_storymap.md:68`, expanded at `:168-178` |
| This spec | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/neon-append-and-read-over-http/spec.md` |
| Roadmap / plan of record | `RUNBOOK.md:4311-4390` (phase 10 in full); `RUNBOOK.md:685-702` (the instrument portfolio — **transport** is the axis this story fills) |

Traces to project **AC-006** (`project.md:243-247`), **AC-007** (`project.md:248-252`)
and **AC-010** (`project.md:263-266`), sharing AC-006 with `neon-sql-transport`,
AC-007 with `neon-fixture-and-live-job` and AC-010 with
`postgres-append-and-frontier-head` on the splits `_storymap.md:222-226` states.
`depends_on`: **`neon-fixture-and-live-job`** — `NeonFixture`, its capability and
ceiling constants, the whole-invocation gating mechanism and the credentialed Neon
CI job all arrive from it (and, transitively, the real `SqlTransport` from
`neon-sql-transport`); and **`postgres-append-and-frontier-head`** — migration 1,
the tag storage and the append-condition SQL are inherited from it, which is the
reason Postgres and Neon are one project and not two
(`project.md:345-352`, `_storymap.md:176-178`).

## One-line PR slice

`event_store_conformance!` runs to completion against a store with no connection,
no interactive transaction and no cursor, every rule reporting pass, fail, or a
skip carrying the fixture's stated reason.

## Executive summary

**Pointer.** `crates/happenstance-neon` is the workspace's deliberate far end of
the **transport** axis — an axis `RUNBOOK.md:685-702` marks empty at both ends.
The crate is unusually far along for a skeleton: `append`'s control flow is
already written and already correct in shape — it refuses the empty batch first,
sends exactly one round trip, and maps `AppendOutcome::Conflict` to
`AppendError::ConditionViolated(ConditionViolated::at(position))`
(`crates/happenstance-neon/src/event_store.rs:183-206`) — and `NeonReadStream`'s
state machine is real, lazy and honest about being a buffer rather than a stream
(`:265-386`). What is missing is everything that touches SQL or JSON.

**Delta this PR lands.** The Neon store's SQL and its decoders, for real, and the
conformance suite invoked against them from `crates/happenstance-neon/tests/`
over a live `/sql` endpoint. Concretely: `read_request` compiling a whole `Query`
and `ReadOptions` into **one** `SELECT`; `conditional_append_request` compiling
the whole append into the one CTE the crate already documents;
`decode_read_response` and `decode_append_response` turning one buffered JSON
document into `SequencedEvent`s and into an `AppendOutcome`; `head` returning the
**visibility frontier** rather than `max(position)`; `contains_event_id`
answering from migration 1's origin columns; the store-limit refusal happening
**before** the round trip; and `event_store_conformance!(… NeonFixture …)` green —
or reporting a named failure and a decision record — in the credentialed Neon CI
job, with `cargo xtask ci` still Docker-free, network-free and green on the
wasm32 build of this very crate.

**A correction to the story map, made here rather than discovered mid-slice.**
`_storymap.md:168-171` names "the three remaining `todo!()`s" —
`conditional_append_request`, `decode_append_response`, `decode_read_response`.
There are **nine** `todo!()`s in `event_store.rs`, and three more of them are on
the path `event_store_conformance!` walks: `read_request` (`:114-116`), `head`
(`:218`) and `contains_event_id` (`:228`). A suite run cannot complete without
them, so they are in this PR. The remaining three —
`probe_request` (`:121-123`), `insert_request` (`:126-128`),
`decode_probe_response` (`:494-498`) and `decode_last_position` (`:501-503`) —
are reachable **only** through `ProbeThenWriteStore`, the named wrong
implementation this project consumes rather than authors
(`_decomposition.md:265-270`), and they belong to `neon-conflicting-position-verdict`,
which is the story that runs it. That is why `#![allow(clippy::todo)]`
(`crates/happenstance-neon/src/lib.rs:101-105`) survives this PR.

**What it deliberately does not land.** It does not build the transport
(`neon-sql-transport`), the fixture, its constants or the CI job
(`neon-fixture-and-live-job`), and it does not settle whether the CTE keeps
`conflicting_position` on a live endpoint or run `ProbeThenWriteStore` as a
negative control (`neon-conflicting-position-verdict`, project AC-008). It
removes no skeleton marker and touches no manifest publish flag
(`deskeleton-and-package-readiness`).

## Context pack

The decisions this story must honour, stated as decisions. Read this section and
you can start; everything deeper is a signposted anchor.

**1. One operation is one statement, and that is the whole adapter.** The
endpoint has no connection, no transaction handle a later request can join, no
cursor, and a hard 64 MiB response cap
(`crates/happenstance-neon/src/lib.rs:18-24`). The only thing above a single
statement is a **non-interactive** batch — an array run server-side inside one
`BEGIN`/`COMMIT` at a chosen isolation level, atomic and *unable to branch*
(`crates/happenstance-neon/src/transport.rs:29-35`). Every SQL shape in this
story follows from that one sentence: the append is one CTE, the read is one
`SELECT`, `head` is one `SELECT`, `contains_event_id` is one `SELECT`. There is
no fallback that issues a second statement and decides between them.

**2. The read is one `SELECT` over the whole `Query`, not one per item — and the
specification already names the wrong shape.** E2E-03 rejects "a one-round-trip
adapter that emits one SQL statement per `QueryItem` and unions the results
client-side… the natural shape for the Neon peer"
(`spec/E2E-CASES.md:103-124`). A multi-item query compiles to one statement whose
`WHERE` is the disjunction of the items, each item being *types OR* within itself
and *tags AND* within itself, with tags matched by superset (`@>`) against the
canonical sorted encoding. Do this and ES-11 and ES-12 come for **free** — one
round trip *is* one statement *is* one snapshot — which is precisely the property
the Postgres sibling has to buy with `BEGIN REPEATABLE READ` and a cursor
(`_decomposition.md:173`). Get it wrong and the adapter fails the axis it exists
to occupy while every rule still passes locally.

**3. `read_request` is total, because the signature gives it nowhere to fail.**
`read` is not `async` and returns the stream at the top level (ADR-0001/ADR-0008,
`CLAUDE.md` constraint 3, ES-2 at `spec/SPECIFICATION.md:2492`), and
`read_request(&self, &Query, ReadOptions) -> SqlRequest` returns no `Result`
(`event_store.rs:114-116`). So a builder that could fail has nowhere to put the
failure: either make the failure unrepresentable, or carry it into
`NeonReadStream`'s state so it surfaces on the first `poll_next` as a stream
item. Do not widen the signature and do not panic. The laziness is load-bearing
on `wasm32`, where issuing a `fetch` outside a polled future happens off the
runtime's event loop (`event_store.rs:277-282`).

**4. The interleaving rule is the one that will actually fail, and the reason is
isolation.** `interleaved_appends_on_one_handle_elect_one_winner`
(`crates/happenstance-testkit/src/suite.rs:5411-5484`) builds two `append`
futures over the same boundary, polls each once so **both round trips are in
flight at the same time**, and then requires exactly one `Ok` *and* at least one
`Err(AppendError::ConditionViolated(_))` — "the loser must learn that it lost, as
a condition violation rather than as an adapter error" (`:5463-5469`). Two
consequences, both sharp:

  - At `ReadCommitted` both CTEs take their snapshot before either commits,
    neither probe sees the other, and **both** commit. Two `Ok`s fails the rule.
  - At `Serializable` one side aborts with SQLSTATE `40001`, which arrives as
    `NeonError::Sql` and would naturally be returned as `AppendError::Store` —
    which *also* fails the rule. On a **conditional** append, a serialisation
    abort is the concurrency signal and must be reported as
    `ConditionViolated`; `ConditionViolated::unspecified()` exists for exactly
    the case where the adapter knows it lost but not to whom
    (`crates/happenstance-core/src/error.rs:150-157`,
    `crates/happenstance-neon/src/error.rs:39-49`).

Note the asymmetry: `racing_conditional_appends_elect_one_winner` (`:5325`) is
*sequential* — it awaits the first append fully before starting the second — so
it passes at any isolation level and proves nothing here.

**5. The crate's isolation claim is, as written, not what the wire does.**
`lib.rs:74-77` says the CTE "needs `IsolationLevel::Serializable` to be sound,
which is why `NeonConfig`'s default is `Serializable`" (`config.rs:13-23`). But
the level travels as the `Neon-Batch-Isolation-Level` **header**, and
`SqlRequest::headers()` returns an empty vector for fewer than two statements
because the endpoint ignores it there (`transport.rs:56-60`, `:190-209`). A
one-statement CTE therefore runs at the endpoint's default. This story must end
with the claim **true or corrected** — by sending the append in a form that
carries the level, by establishing on the live endpoint that a single statement
honours it after all and fixing the transport's doc, or by an alternative that is
sound at `ReadCommitted` and says so. Whichever it is, it is recorded. What is
forbidden is leaving a green run to imply a soundness claim nothing checked.

**6. And no rule this adapter can run falsifies §5.** The family that would —
`event_store_concurrency_conformance!` — binds `F::Store: EventStore + Send` and
is Postgres-only by design; Neon's bare-flavour store cannot invoke it and is not
expected to (`crates/happenstance-testkit/src/lib.rs:101-110`,
`_decomposition.md:139-143`). §4's rule is the strongest instrument available
here, and it is a two-in-flight interleave on one thread, not contention. State
that limit in the ledger rather than letting the suite's silence stand in for
evidence — this is the same "a rule no adapter can fail is decorative" discipline
read from the other end (`CLAUDE.md`).

**7. `head` is a frontier here too, because Neon inherits Postgres' schema.**
This story depends on `postgres-append-and-frontier-head` for migration 1, the
tag storage and the append-condition SQL (`_storymap.md:176-178`). The visibility
mechanism wired there therefore governs this crate's `read`, `head` and
`contains_event_id` as well: the frontier predicate goes into every one of the
single statements, `head` reports the frontier and not `max(position)`,
read-your-own-writes does not hold, and ES-30's rule asserts a **bound** rather
than an equality precisely to admit that
(`spec/SPECIFICATION.md:2816-2842`, `:3941`;
`crates/happenstance-testkit/src/suite.rs:1798`). The `todo!()` on `head` says so
in terms: "Phase 10 writes this body against whatever `happenstance-postgres`
measures its way to, not before" (`event_store.rs:208-218`). Do **not** cite
ADR-0024 — no `.kb/decisions/0024-*` file exists, and citing it is the failure
`_decomposition.md:179-181` names.

**8. The module doc's schema is now stale, and stale documentation here is a
wrong claim.** `event_store.rs:36-54` states `position bigserial PRIMARY KEY` and
then explains why that is broken. Migration 1 does not do that — `position` is
deliberately not `bigserial` — and this crate runs against migration 1. Reconcile
the doc block with the schema the adapter actually issues SQL against, or the
crate documents a store nobody built.

**9. A ceiling is refused before the wire, and a refusal has one spelling.**
`AppendError::ExceedsStoreLimit { limit: StoreLimit::… }` — never
`AppendError::Store`, never a truncation
(`crates/happenstance-testkit/src/suite.rs:4282-4457`,
`crates/happenstance-core/src/limits.rs:54-74`). For this adapter that means the
check happens **before** `round_trip`: an over-limit request that goes to the
wire comes back as a 4xx and becomes `NeonError::Http` inside
`AppendError::Store`, which is the forbidden spelling arriving by accident. The
ceilings are anchored on `MAX_RESPONSE_BYTES` (64 MiB, `transport.rs:45-54`) and
are **halved in effect** because parameters are `serde_json::Value` and a `bytea`
travels as a `\x…` hex string (`transport.rs:87-102`). Independently of any
ceiling, the guaranteed minima must be *accepted*: 65,536 bytes of payload, 64
tags, a 128-item query, a 128-event batch
(`crates/happenstance-core/src/limits.rs:22-43`, VT-21 – VT-24 at
`spec/SPECIFICATION.md:1483`, `:1513`, `:1535`, `:1556`). The two rules are
independent and both are owed an answer. The fixture *constants* are
`neon-fixture-and-live-job`'s; the **adapter behaviour behind them** is this
story's, and CF-40's contested ownership is consumed, not settled
(`.kb/open-questions/cf-40-fixture-limits-ownership.md`, DR-6).

**10. Bare flavour, both targets, and there is no way back up.**
`NeonEventStore` implements the **bare** `EventStore`; adding a second
`SendEventStore` impl is `error[E0119]` against `trait_variant`'s blanket impl,
so the flavours are mutually exclusive per type
(`crates/happenstance-neon/src/lib.rs:79-89`, `event_store.rs:4-34`). Never
`#[async_trait]` (`CLAUDE.md` constraint 1, ADR-0001). The wasm32 `cargo check` of
this crate is a `REQUIRED` gate step selected **by name**
(`xtask/src/main.rs:271`, `:784-789`), so it must stay green and the step's name
must not move.

**11. Decoding is OID-directed, and the endpoint lies about types on purpose.**
Everything arrives as JSON: `bigint` as a **string** to avoid IEEE-754 loss,
`bytea` as a `\x…` hex string, and the column's Postgres OID in
`FieldDescription::data_type_id` is the only thing distinguishing them
(`crates/happenstance-neon/src/wire.rs:13-45`). Both response envelopes — the
bare `ResultSet` for one statement and `{"results": […]}` for the batch — are
already modelled, and `ResponseBody::result_sets` normalises them (`:47-73`). A
`position` that will not fit `SequencePosition`'s `NonZeroU64` is
`NeonError::InvalidPosition`, not a panic and not a silent clamp
(`crates/happenstance-neon/src/error.rs:106-115`).

**12. Nothing skips silently, and a skip is not a `#[cfg]`.** A rule whose
capability the fixture declines is **still emitted as a test**, returns
`RuleOutcome::Skipped`, and the harness prints the fixture's stated reason
(`crates/happenstance-testkit/src/lib.rs:46-53`, `contract.rs:213-235`).
`SECOND_HANDLE` is the one MUST: declining it makes
`two_handles_observe_each_others_appends` **panic**, quoting the fixture's own
words (`contract.rs:135-161`). `MID_BATCH_FAULT` defaults to declined and Neon
over one non-interactive statement plausibly cannot offer it
(`contract.rs:207-211`, `_decomposition.md:300-304`). What this story owes is
that the adapter never makes a rule *unable to report*: no `#[cfg]` removing a
rule from the expansion, no hand-picked subset, and the full macro expansion in
the live job (DR-5, `project.md:192-196`).

**13. Never assert on literal position values.** `cargo xtask lint-position-literals`
is a `REQUIRED` gate step (`xtask/src/main.rs:389`, `:687`;
`xtask/src/lints.rs:628`), and this is the project where gaps stop being
hypothetical (CF-6).

**The persona slice.** The user here is an adapter author and the consuming
application behind them (`_storymap.md:19-23`). Their fear, stated in the
research, is "discovering — late, expensively… that the port they implemented
against quietly assumed something their storage system cannot provide"
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:144-152`).
Every adapter that has ever passed this suite holds a connection, can open a
transaction and can stream from a cursor. This story is the first run of the
suite against a store that has none of the three — and E2E-02, whose `Spans` line
still carries "⚠ `happenstance-neon` (does not exist)"
(`spec/E2E-CASES.md:78-100`), becomes writable against an adapter that can
genuinely fail it. That is initiative **DoD 6** and initiative **AC-06**: the
adapter author's storage shape stops being quietly assumed.

## Integration contract

| Field | Value |
| ----- | ----- |
| **Archetype** | `capability` — user-observable in this repository's medium: a port impl a caller meets, and a conformance run that reports pass, a named failure, or a skip carrying a stated reason. |
| **Slice / milestone** | `neon-live-suite`. **Slice-mates**, implemented in one context and mounted as one integrated surface: `neon-fixture-and-live-job` (foundation, upstream of this story) and `neon-conflicting-position-verdict` (downstream of it) — `_storymap.md:67-69`, merge order at `:255-257`. `neon-sql-transport` is the upstream foundation in the `neon-transport` milestone (`:66`). |
| **Mount point** | `crates/happenstance-neon/src/event_store.rs` — the `impl EventStore for NeonEventStore<T>` block (`:168-230`) and the four request/decode helpers it calls (`:110-166`, `:249-263`), which is Architecture §2's **Root A** (`_decomposition.md:113-128`). It is mounted *for real* at **Root B**: the integration-test target under `crates/happenstance-neon/tests/` — the crate has no `tests/` directory today — where `event_store_conformance!(mod_name = …, emit = …, fixture = NeonFixture::…)` is invoked (`crates/happenstance-testkit/src/lib.rs:311-357`). Filling the bodies without that invocation is the "constructed but unmounted" failure in this medium: a `todo!()` body type-checks against any signature (`RUNBOOK.md:3061-3068`). |
| **Wires into** | **Consumed from `neon-fixture-and-live-job`**: `NeonFixture`, its `SECOND_HANDLE` / `REOPEN` / `MID_BATCH_FAULT` answers, its three `Option<usize>` ceilings, the whole-invocation gating mechanism and the credentialed Neon CI job. **Consumed from `neon-sql-transport`**: the real `SqlTransport` impl and its `cfg` scoping. **Consumed from `postgres-append-and-frontier-head`**: migration 1, the tag storage, the append-condition SQL and the visibility mechanism the frontier predicate expresses. **In-crate**: `SqlRequest` / `SqlStatement` / `IsolationLevel` / `HttpResponse` (`transport.rs:87-239`), `NeonConfig` (`config.rs`), `NeonError` and `NeonSqlError` (`error.rs`), `wire::{ResponseBody, ResultSet, FieldDescription}` (`wire.rs`), `NeonReadStream` (`event_store.rs:265-386`). **Contract types**: `EventStore`, `Event`, `SequencedEvent`, `Query`, `ReadOptions`, `AppendCondition`, `AppendError`, `ConditionViolated`, `SequencePosition`, `EventId` (`crates/happenstance-core/src/store.rs`, `error.rs`), and `StoreLimit` / `MIN_SUPPORTED_*` (`crates/happenstance-core/src/limits.rs:22-74`). |
| **Renders surfaces** | **None.** `_design.md` records no user-facing surface for this project and the sign-off approved that determination (`_design.md:10-41`, `:86-95`). `design.capture` is deliberately absent from `.redkiln/config.yaml:75-80`, making the perceptual review a declared skip, not a silent pass. |
| **Public items** | No new `pub` type and no new free function. What changes is the *behaviour* behind four already-public method signatures on `impl EventStore for NeonEventStore<T>`, plus possible new `#[non_exhaustive]` variants on `NeonError<E>` for decode failures the existing six do not name (`error.rs:63-116`). `AppendOutcome` is and stays private, and its `#[expect(dead_code)]` (`event_store.rs:237-247`) must be **removed** in this PR — the reason string says "until phase 10", and an `expect` that no longer fires is itself a warning under `-D warnings`. `publish = false` stays (`Cargo.toml:12`); its removal is `deskeleton-and-package-readiness`'. |
| **Conformance rule(s)** | Observed by the **whole** `event_store_conformance!` expansion (`crates/happenstance-testkit/src/registry.rs`), never a subset. Named because they are the ones this adapter can plausibly get wrong: `interleaved_appends_on_one_handle_elect_one_winner` (`suite.rs:5411`) — the isolation question in §4; `nothing_below_an_observed_position_appears_later` (CF-13, `:5880`); `racing_conditional_appends_elect_one_winner` (`:5325`) and `condition_rejection_is_reported_as_condition_violated` (`:4949`); `query_items_share_one_snapshot` (`:5696`) and `read_result_is_stable_under_concurrent_append` (`:5592`) — ES-11/ES-12, free from one round trip; `limit_applies_across_items_not_per_item` (`:1402`) and `read_from_a_gap_position` (`:1490`); `head_is_the_highest_visible_position` (`:1798`) and `head_of_an_empty_store_is_none` (`:1731`); `contains_event_id_reports_membership` (`:2551`); `append_reports_exceeded_store_limits` (`:4282`) and the four `store_accepts_the_guaranteed_minimum_*` rules (`:3775`, `:3829`, `:3880`, `:3954`); `two_handles_observe_each_others_appends` (`:265`) and `acknowledged_writes_survive_a_reopen` (`:332`). **This story adds no rule to `happenstance-testkit`** — the testkit is a must-not-change seam here (`_decomposition.md:485-488`). The concurrency family's absence is by design, not a gap (`_storymap.md:174-175`). |
| **Clause(s)** | **Discharges evidence for** ES-11 (`spec/SPECIFICATION.md:2926`, `[PROVISIONAL]`, transport axis) and ES-12 (`:2998`, `[PROVISIONAL]`, and "strictly harder" per `:3003`) — the two clauses that hold the CF-25 residual for this axis (`:371`); ES-10 (`:2816`, `[FROZEN]`) through the frontier predicate; ES-2 (`:2492`) and ES-42 (`:3079`) through the unchanged lazy `read`; ES-20 (`:3480`), ES-25 (`:3695`), ES-30 (`:3941`); VT-21 – VT-24 (`:1483` – `:1556`) and VT-25 (`:1580`). **Amends none.** No `[FROZEN]` marker moves in this PR. If the live endpoint forces one — DoD 6 explicitly contemplates it — that is a new accepted decision atom with the suite re-run, never an edit (`CLAUDE.md`, DR-7), and it is authored through `.kb/_intake/` and `/redkiln:kb-ingest`, never by hand (`_decomposition.md:159-164`). ES-41 (`:4381`) is touched by `contains_event_id` and stays `[PROVISIONAL]`. Regenerating §7.1–§7.2's markers is `far-end-discharge-record`'s, by `cargo xtask spec-trace --write`. |
| **Advances DoD scenario** | Initiative **DoD 6** — *a store with no connection, no interactive transaction and no cursor passes the suite, or the contract is amended by decision record and the suite re-run* (`initiative.md:375-376`). This story is the run itself; `neon-fixture-and-live-job` supplies what it runs on and `neon-conflicting-position-verdict` closes the ledger question behind it, so DoD 6 is not green until all three land. It also fills the **transport** far end of the instrument portfolio (`RUNBOOK.md:685-702`) and makes **E2E-02** and **E2E-03** writable against an adapter that can genuinely fail them (`spec/E2E-CASES.md:78-124`). |

## PR boundary

`redkiln verify --grain story` reads the first fenced block under this heading and
fails on any file changed outside it.

```
crates/happenstance-neon/src/**
crates/happenstance-neon/tests/**
crates/happenstance-neon/Cargo.toml
.bklg/from-contract-to-published-library/postgres-and-neon-stores/neon-append-and-read-over-http/**
```

The boundary is one crate wide on purpose. `happenstance-core` and
`happenstance-testkit` are must-not-change seams here (`_decomposition.md:73-105`,
`:485-488`), and no other adapter is touched — the dependency rule forbids it,
and `crates/happenstance-postgres/**` is deliberately **outside** this boundary
even though this story consumes its migration. The implementer may also touch the
composition-root and wiring files named in the Integration contract to mount this
slice; here that is the `tests/` target and the crate's own `Cargo.toml`, both
inside the boundary already, and that is not scope drift.

**In this PR**

- `read_request` (`event_store.rs:114-116`) — one `SELECT` for a whole `Query`
  and `ReadOptions`, total, carrying the frontier predicate.
- `conditional_append_request` (`:159-165`) — the whole append as one CTE
  statement, in the shape the crate already documents (`lib.rs:54-68`).
- `decode_read_response` (`:257-263`) and `decode_append_response` (`:249-255`),
  and the removal of `AppendOutcome`'s `#[expect(dead_code)]` (`:237-247`).
- `head` (`:208-218`) as the visibility frontier, and `contains_event_id`
  (`:221-229`) against migration 1's origin columns.
- The store-limit pre-flight refusal, spelled `AppendError::ExceedsStoreLimit`.
- The isolation resolution of Context pack §5 — implemented **and** recorded,
  including a correction to `lib.rs:74-77` or `transport.rs:56-60` if that is
  where the evidence points.
- The module-doc schema reconciliation of Context pack §8.
- `event_store_conformance!(… NeonFixture …)` invoked from
  `crates/happenstance-neon/tests/`, behind the dependency story's
  whole-invocation gate, running in the Neon CI job.
- `Cargo.toml` only for dev-dependencies the invocation genuinely needs —
  `happenstance-testkit` and `tokio` with `macros`/`rt` are not in this crate's
  manifest today (`Cargo.toml:14-32`) — recorded as a deliberate manifest
  decision, and coordinated with `neon-fixture-and-live-job`, which may already
  have added them.
- New `#[non_exhaustive]` `NeonError` variants, if and only if a decode failure
  the existing six cannot name actually occurs.
- This story's own `_ledger.md`.

**Explicitly not in this PR**

- **The transport.** `SqlTransport`'s real implementation, its ship-vs-dev
  decision, its `cargo deny` admissibility and its `cfg` scoping —
  `neon-sql-transport` (`_decomposition.md:420-442`). `NullTransport` stays as it
  is (`transport.rs:267-298`).
- **The fixture and the job.** `NeonFixture`, its capability constants, its three
  ceilings, the gating mechanism and the credentialed CI job —
  `neon-fixture-and-live-job` (project AC-007, AC-011).
- **`ProbeThenWriteStore` run as a negative control, and the
  `conflicting_position` verdict.** `probe_request`, `insert_request`,
  `decode_probe_response` and `decode_last_position` stay `todo!()`, the type is
  neither deleted nor "fixed" (`_decomposition.md:265-270`), and the decision
  ledger's standing assumption (`crates/happenstance-core/src/error.rs:139-146`)
  is left standing for `neon-conflicting-position-verdict` to correct
  (project AC-008).
- **Removing any skeleton marker.** `#![allow(clippy::todo)]`
  (`lib.rs:101-105`), `publish = false` (`Cargo.toml:12`), `PUBLISHABLE`
  (`xtask/src/package.rs:86`) and the licence/README copies are
  `deskeleton-and-package-readiness`'.
- **`NeonProjectionStore`**, which stays `todo!()` (`src/projection_store.rs`) —
  `projection-store-freeze` has not frozen the `Batch` shape
  (`_decomposition.md:182-185`).
- **Choosing the ES-10 mechanism, or citing ADR-0024.** No
  `.kb/decisions/0024-*` file exists; the mechanism is inherited from
  `postgres-append-and-frontier-head` on ADR-0013's prior authority
  (`.kb/decisions/0013-position-assignment-and-visibility.md`).
- Any change to `happenstance-core`, `happenstance-testkit`, another adapter, the
  `REQUIRED` step array in `xtask/src/main.rs`, `.github/workflows/ci.yml`, or
  `spec/SPECIFICATION.md`'s generated marker regions.
- Running the suite on `wasm32` under a real edge runtime — that is
  `cloudflare-durable-object-store`'s (initiative DoD 4, `project.md:133-138`).
  This crate's wasm32 **build** stays green; nothing more is claimed.

**Merge DoD (one line).** `event_store_conformance!` runs its full expansion
against a live Neon `/sql` endpoint in the project's Neon CI job with no rule
absent and every non-pass carrying a stated reason, `cargo xtask ci` is green on
a clean checkout with no Docker and no credentials — including the wasm32 build of
this crate — and `cargo xtask affected --base main` is green on the story's tree.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The suite is invoked entire, against a real endpoint | `event_store_conformance!(mod_name = …, emit = …, fixture = NeonFixture::…)` from `crates/happenstance-neon/tests/`, full macro expansion, never a hand-picked subset. A pooled Postgres connection standing in for the endpoint is the exact shortcut DR-4 forbids and destroys the axis the crate exists to occupy. → AC-001 | `crates/happenstance-testkit/src/lib.rs:311-357`; `project.md:188-191`; `_decomposition.md:516`, `:651-656` |
| A read is one `SELECT` for the whole `Query` | Items disjoined in one `WHERE`; within an item, types OR and tags AND, tags matched as a superset against the canonical sorted encoding. `ReadOptions` compiles to `ORDER BY position` (reversed under `backwards`), inclusive `from`/`to` bounds, and `LIMIT` applied to the *whole* filtered result after ordering. One statement per operation, never one per item. → AC-002 | `spec/E2E-CASES.md:103-124` (E2E-03 rejects the per-item shape); `crates/happenstance-testkit/src/suite.rs:1402`, `:1190`, `:883`, `:912`; `spec/SPECIFICATION.md:3111` (ES-14) |
| ES-11 and ES-12 are bought by the transport, not by machinery | One round trip is one statement is one snapshot, so a read is a snapshot and all items of one `Query` share it with nothing else to do. That is this adapter's one genuine advantage over its Postgres sibling and it must not be given away by a second statement. → AC-002 | `.kb/decisions/0011-read-laziness-and-isolation.md`; `_decomposition.md:173`; `spec/SPECIFICATION.md:2926`, `:2998`; `suite.rs:5696`, `:5592` |
| `read` stays lazy and `read_request` cannot fail | `read` is not `async` and returns the stream at the top level; nothing is sent until the first `poll_next` (`ReadState::Unsent`). `read_request` returns `SqlRequest`, not `Result<SqlRequest, _>`, so a fallible builder has nowhere to report — make the failure unrepresentable or carry it into the stream's state. Do not widen the signature, do not panic, and do not make `read` `async`. → AC-002, AC-010 | `CLAUDE.md` constraint 3; `.kb/decisions/0001-async-port-flavours.md`; `crates/happenstance-neon/src/event_store.rs:114-116`, `:171-181`, `:277-282`, `:345-386` |
| A response is decoded OID-first, and a too-large one is refused | `bigint` arrives as a JSON **string**, `bytea` as a `\x…` hex string, and `FieldDescription::data_type_id` is the only discriminator. Both envelopes (bare `ResultSet`, `{"results": […]}`) go through `ResponseBody::result_sets`. `EventId`, `recorded_at` and tags come from migration 1's columns. A body over `config.max_response_bytes` is `NeonError::ResponseTooLarge`, terminal for that read — never a truncated result set, because a truncated read is a silently short prefix. → AC-003 | `crates/happenstance-neon/src/wire.rs:13-73`; `crates/happenstance-neon/src/error.rs:88-100`; `crates/happenstance-neon/src/config.rs:59-72`; `crates/happenstance-neon/src/event_store.rs:310-328` |
| A `position` that will not fit is a named error | Postgres `bigint` is signed and `SequencePosition` is a `NonZeroU64`; the conversion is fallible and the failure is `NeonError::InvalidPosition { value }`, not a panic, not a clamp, not a `0`. → AC-003 | `crates/happenstance-neon/src/error.rs:106-115`; `crates/happenstance-core/src/error.rs:135-166` |
| The append is one CTE statement, and the empty batch is refused first | Zero events returns `AppendError::NoEvents` before any request is built — already written and must stay (`:188-190`). The condition and the insert are computed on one snapshot inside one statement, projecting both the appended position and the conflicting one; there is never a probe followed by a write. → AC-004 | `.kb/decisions/0012-append-shape-and-preconditions.md`; `spec/SPECIFICATION.md:3480` (ES-20); `crates/happenstance-neon/src/lib.rs:46-77`; `event_store.rs:130-166`, `:183-206` |
| The append's single row is decoded unambiguously | `appended` non-null ⇒ `AppendOutcome::Appended`; `conflict` non-null ⇒ `AppendOutcome::Conflict`. Both null, or both non-null, is a malformed answer and gets an error — never an `Ok` and never a silently swallowed conflict. `AppendOutcome`'s `#[expect(dead_code)]` is removed in the same change, because an `expect` that stops firing is a warning under `-D warnings`. → AC-005 | `crates/happenstance-neon/src/event_store.rs:232-255`; `crates/happenstance-neon/src/error.rs:102-104` |
| Two appends in flight elect one winner, and the loser is told as a condition violation | `interleaved_appends_on_one_handle_elect_one_winner` polls both futures once, so both round trips overlap. Exactly one `Ok`; at least one `Err(AppendError::ConditionViolated(_))`; exactly one event in the store. A SQLSTATE `40001` abort on a **conditional** append is that signal — report it as `ConditionViolated::unspecified()`, never as `AppendError::Store`. An abort on an **unconditional** append is a store error and stays one. → AC-006 | `crates/happenstance-testkit/src/suite.rs:5411-5484`; `crates/happenstance-core/src/error.rs:150-157`; `crates/happenstance-neon/src/error.rs:39-49` |
| The isolation level the append runs at is the one the crate claims | The level travels as a header that `SqlRequest::headers()` suppresses below two statements, so a one-statement CTE does not carry it. Resolve it — a form that carries the level, live evidence that a single statement honours it, or a shape sound at `ReadCommitted` — implement it, and correct whichever doc is wrong. → AC-007 | `crates/happenstance-neon/src/transport.rs:56-85`, `:190-209`; `crates/happenstance-neon/src/config.rs:13-23`; `crates/happenstance-neon/src/lib.rs:74-77` |
| The soundness claim's unfalsifiability is written down, not implied | `event_store_concurrency_conformance!` binds `F::Store: EventStore + Send` and cannot be invoked by a bare-flavour store, so no rule available here puts real contention on the append. Record what the green run does and does not evidence rather than letting silence stand for proof. → AC-007 | `crates/happenstance-testkit/src/lib.rs:101-110`; `_decomposition.md:139-143`; `_storymap.md:174-175`; `.kb/open-questions/poll-count-bounds-the-visibility-rule.md` |
| `head` is the visibility frontier, on one round trip | One `SELECT` carrying the same frontier predicate the read does — not `max(position)`. `None` on an empty store. Read-your-own-writes does not hold and is not claimed. ES-30's rule asserts a bound, deliberately. The mechanism is inherited from `postgres-append-and-frontier-head`; ADR-0024 is not cited, because it does not exist. → AC-008 | `crates/happenstance-neon/src/event_store.rs:208-218`; `spec/SPECIFICATION.md:2816-2842`, `:3941`; `crates/happenstance-testkit/src/suite.rs:1731`, `:1798`; `.kb/decisions/0013-position-assignment-and-visibility.md` |
| `contains_event_id` answers from real origin columns | `SELECT EXISTS (… WHERE origin_store = $1 AND origin_position = $2)`, one statement, one round trip, against migration 1's columns. It inherits the same frontier disagreement the Postgres sibling records; take the same answer it took and say so. ES-41 stays `[PROVISIONAL]`; replication semantics are HS-P0017's. → AC-008 | `crates/happenstance-neon/src/event_store.rs:221-229`; `spec/SPECIFICATION.md:4381`; `_decomposition.md:224-231` |
| An over-limit append is refused before the wire | `AppendError::ExceedsStoreLimit { limit: StoreLimit::EventDataLen \| TagsPerEvent \| EventsPerBatch, .. }`, decided from the encoded request size **before** `round_trip`. Never `AppendError::Store`, never a truncation, never a 4xx from the endpoint doing the refusing. The ceilings are anchored on `MAX_RESPONSE_BYTES` and halved in effect by hex `bytea` rendering. → AC-009 | `crates/happenstance-testkit/src/suite.rs:4282-4457`; `crates/happenstance-core/src/limits.rs:54-74`; `crates/happenstance-neon/src/transport.rs:45-54`, `:87-102`; `spec/SPECIFICATION.md:1580` (VT-25) |
| The guaranteed minima are accepted, independently of any ceiling | 65,536 bytes of payload, 64 tags, a 128-item query, a 128-event batch — accepted, not refused. Against a hex-doubling JSON transport a 128×65,536-byte batch is a large request body, and that is the point: the floor is a floor. The two rules are independent and both are owed an answer. → AC-009 | `crates/happenstance-testkit/src/suite.rs:3775`, `:3829`, `:3880`, `:3954`; `crates/happenstance-core/src/limits.rs:22-43`; `crates/happenstance-testkit/src/contract.rs:237-252` |
| The `!Send` story does not regress | Bare `EventStore` only, on both targets. No second `SendEventStore` impl (`error[E0119]` against `trait_variant`'s blanket impl), no `#[async_trait]`, `read` still not `async`, the in-crate flavour tests still green, and the `REQUIRED` wasm32 step — selected by name — still green under its current name. → AC-010 | `CLAUDE.md` constraints 1 and 4; `crates/happenstance-neon/src/lib.rs:79-89`; `event_store.rs:4-34`, `:505-549`; `xtask/src/main.rs:271`, `:784-789` |
| Every rule reports, and none is hidden | A declined capability produces an emitted test returning `RuleOutcome::Skipped` with the fixture's stated reason, printed by the job's `--show-output`. No `#[cfg]` removes a rule from the expansion; gating the *whole invocation* on infrastructure is a different act and is the only permitted one. → AC-011 | `crates/happenstance-testkit/src/lib.rs:46-53`; `crates/happenstance-testkit/src/contract.rs:213-235`; `_decomposition.md:346-353`, `:596-630`; `project.md:192-196` |
| The default gate stays Docker-free and network-free | `cargo test --workspace --all-features` exits zero with no endpoint reachable and no credential present, and `cargo xtask ci` is green on a clean checkout. The live invocation runs only in the Neon CI job. → AC-011 | `.redkiln/config.yaml:40`, `:55`; `_decomposition.md:332-353`; `project.md:212-217` (DR-9) |
| No literal position values anywhere in this story's tests | Compare against positions the store assigned. The lint is a `REQUIRED` gate step, so this fails the build rather than a review. → AC-012 | `CLAUDE.md` (conformance section, CF-6); `xtask/src/main.rs:389`, `:687`; `xtask/src/lints.rs:628` |
| The crate documents the schema it actually runs against | The `bigserial` schema block and the paragraph beneath it describe a store this adapter no longer talks to once migration 1 is inherited. Reconcile both, or the crate's public docs assert a design that was measured away. → AC-012 | `crates/happenstance-neon/src/event_store.rs:36-54`; `crates/happenstance-postgres/src/event_store.rs:22-35`; `_storymap.md:176-178` |
| Evidence is owed per criterion | `require_ledger: true`, and `require_commit_provenance: true` for any file changed inside the declared boundary. | `.redkiln/config.yaml:67`, `:73` |

## Data and migrations

**Inherited, not authored — and that inheritance is the reason this project is
one project.** `happenstance-neon` owns no migration and adds none. Migration 1
lives in `crates/happenstance-postgres/migrations/`, is authored by
`postgres-schema-and-live-fixture` and amended by
`postgres-append-and-frontier-head`, and this story consumes it: the identity and
time columns (`EventId`, `recorded_at`, discharged at phase 4,
`RUNBOOK.md:249-253`), the tag storage, the visibility mechanism's column, and
the `origin_store` / `origin_position` columns `contains_event_id` needs
(`_storymap.md:176-178`, `_decomposition.md:233-238`). `crates/happenstance-postgres/**`
is outside this story's PR boundary; if the SQL this adapter issues needs a
column migration 1 does not have, that is a coordination item with the Postgres
slice, not a second migration file authored here.

**A Neon branch is provisioned, not migrated.** The endpoint has no interactive
transaction, so this crate cannot run a migration in the sense
`sqlx::migrate!` means, and it does not try. How the CI branch arrives already
carrying migration 1 — and how its credential is reached without entering the
default gate — is `neon-fixture-and-live-job`'s, following the deployment brief's
`REDKILN_TOKEN` precedent (`_decomposition.md:742-753`). What this story owes is
that the SQL it emits and the schema that branch carries are the same schema.

**Backfill: N/A, and the brief says so in terms.** Neither crate has ever shipped
a schema to a real consumer — both are `publish = false` today — so there is no
prior deployment to migrate data out of. This is a schema-*authoring* concern
only (`_decomposition.md:713-724`).

**One documentation change that is a data statement.** The intended schema in
`crates/happenstance-neon/src/event_store.rs:36-54` declares
`position bigserial PRIMARY KEY` and explains, correctly, why `nextval()` loses
the invariant. Migration 1 does not use `bigserial`
(`crates/happenstance-postgres/src/event_store.rs:22-35`). Leaving that block
unreconciled leaves the crate's public documentation asserting a schema the
adapter does not use and a defect the adapter does not have — which is worse than
silence, because a reader will believe it.

**Encoding is where this adapter's data cost lives.** Parameters are
`serde_json::Value`, so a `bytea` travels as a `\x…` hex string and roughly
doubles against the 64 MiB cap **in both directions**
(`crates/happenstance-neon/src/transport.rs:87-102`, `:45-54`). Tags travel as a
`text[]` in the canonical sorted encoding so `@>` stays available on the read
side, and `unnest($2::text[], $3::bytea[], $4::bytea[], $5::text[][])` is the
insert shape the documented CTE already assumes (`lib.rs:60-64`). This is a real
cost a binary-protocol adapter does not pay, and it is what the ceilings in
Context pack §9 are anchored on.

## Acceptance criteria

The persona is the **adapter author** of
`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:130-160`
and the application behind them, whose fear is "discovering — late, expensively…
that the port they implemented against quietly assumed something their storage
system cannot provide" (`:144-152`). Every criterion below is that person
crossing the whole stack — contract, adapter, SQL, wire, harness — not a
capability restated.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | GIVEN an adapter author whose store is one-shot HTTP, WHEN they run `event_store_conformance!` against a `NeonFixture` backed by a real Neon `/sql` endpoint, THEN the **entire** macro expansion executes to completion and every rule in it reports pass, fail, or `Skipped` with the fixture's stated reason — no rule absent, no hand-picked subset, and no pooled-Postgres stand-in for the endpoint. | The `event_store_conformance!(mod_name = …, emit = …, fixture = NeonFixture::…)` invocation in `crates/happenstance-neon/tests/conformance.rs`, run in the Neon CI job as `cargo test -p happenstance-neon --all-features -- --ignored --show-output`; the run's rule count equals the registry's (`crates/happenstance-testkit/src/registry.rs`). Traces project **AC-006**. |
| AC-002 | GIVEN an author whose store cannot open a cursor or a transaction, WHEN their application reads a multi-item `Query` with `ReadOptions`, THEN one `SELECT` — items disjoined, types OR within an item, tags AND matched by superset — returns the whole result in **one** round trip, so all items share one snapshot for free; and `read` is still not `async`, still returns the stream at the top level, and still sends nothing until the first `poll_next`. | `query_items_share_one_snapshot` (`crates/happenstance-testkit/src/suite.rs:5696`), `read_result_is_stable_under_concurrent_append` (`:5592`), `limit_applies_across_items_not_per_item` (`:1402`), `read_from_a_gap_position` (`:1490`) — all in the live run; plus an offline in-crate test in `crates/happenstance-neon/src/event_store.rs` `mod tests` asserting `read_request` emits exactly one `SqlStatement` for a ≥2-item query. Traces **AC-006**. |
| AC-003 | GIVEN a JSON transport that renders `bigint` as a string and `bytea` as `\x…` hex, WHEN the author's application reads events back, THEN every `SequencedEvent` round-trips byte-identically by OID-directed decoding, a body over `max_response_bytes` fails the read as `NeonError::ResponseTooLarge` rather than returning a silently short prefix, and a `position` that will not fit `NonZeroU64` is `NeonError::InvalidPosition` — never a panic, a clamp, or a zero. | The payload and ordering rules in the live run; offline in-crate tests over `wire::ResponseBody` fixtures in `crates/happenstance-neon/src/event_store.rs` `mod tests` covering both envelopes (bare `ResultSet` and `{"results": […]}`), the oversized body, and an out-of-range `position` string. Traces **AC-006**. |
| AC-004 | GIVEN an author appending under an `AppendCondition`, WHEN the append is issued, THEN the condition check and the insert are computed on one snapshot inside **one** CTE statement in one round trip — never a probe followed by a write — and a zero-event batch is refused as `AppendError::NoEvents` before any request is built. | `condition_rejection_is_reported_as_condition_violated` (`suite.rs:4949`) and `racing_conditional_appends_elect_one_winner` (`:5325`) in the live run; an offline in-crate test asserting `conditional_append_request` produces exactly one `SqlStatement`, and that `append` with an empty slice returns `NoEvents` without touching the transport (`NullTransport` fails every round trip, so reaching the wire is observable). Traces **AC-006**. |
| AC-005 | GIVEN a conditional append that lost, WHEN the single returned row is decoded, THEN `appended` non-null yields `AppendOutcome::Appended` and `conflict` non-null yields `AppendOutcome::Conflict`, both-null or both-non-null is a decode error rather than an `Ok`, and the author's application therefore never sees a conflict silently swallowed into success. | `condition_rejection_is_reported_as_condition_violated` (`suite.rs:4949`) in the live run; offline in-crate tests over all four null/non-null combinations of `decode_append_response`; `cargo clippy -D warnings` proves `AppendOutcome`'s `#[expect(dead_code)]` (`event_store.rs:237-247`) was removed, because an `expect` that stops firing is itself a warning. Traces **AC-006**. |
| AC-006 | GIVEN two of the author's writers appending to the same boundary at the same moment, WHEN both round trips are in flight simultaneously, THEN exactly one succeeds, the store holds exactly one event, and the loser learns it lost as `AppendError::ConditionViolated(_)` — including when the endpoint reports a SQLSTATE `40001` serialisation abort, which on a **conditional** append is reported as `ConditionViolated::unspecified()` and never as `AppendError::Store`. An abort on an **unconditional** append stays a store error. | `interleaved_appends_on_one_handle_elect_one_winner` (`crates/happenstance-testkit/src/suite.rs:5411-5484`) in the live run — the rule polls each future once before awaiting either, which is what makes the round trips overlap; plus an offline in-crate test mapping a synthetic `NeonSqlError` carrying `40001` (`crates/happenstance-neon/src/error.rs:39-49`) through the conditional and unconditional paths to the two different `AppendError` spellings. Traces **AC-006**. |
| AC-007 | GIVEN an author reading the crate's own soundness claim, WHEN they ask what isolation level the append actually ran at, THEN the answer in the docs is the answer on the wire — the CTE either carries the level it claims, or is shown sound at the endpoint's default and the claim at `lib.rs:74-77` / `transport.rs:56-60` is corrected — **and** the story records that no rule available to a bare-flavour store puts real contention on the append, so a green run is not read as evidence it never gave. | The resolution is implemented in `crates/happenstance-neon/src/{event_store.rs,transport.rs,config.rs}` and asserted by an offline in-crate test over the emitted `SqlRequest` (headers present, or the doc corrected and the test asserting the corrected invariant); the unfalsifiability statement is a findings note in the story's `implementation-report.md` citing `crates/happenstance-testkit/src/lib.rs:101-110`. Traces project **AC-007**. |
| AC-008 | GIVEN a store whose positions become visible after they are assigned, WHEN the author's application calls `head` or `contains_event_id`, THEN `head` reports the **visibility frontier** in one `SELECT` carrying the same predicate the read does (not `max(position)`), returns `None` on an empty store, read-your-own-writes is not claimed, and `contains_event_id` answers from migration 1's `origin_store` / `origin_position` columns in one round trip. | `head_of_an_empty_store_is_none` (`suite.rs:1731`), `head_is_the_highest_visible_position` (`:1798` — asserts a **bound**, deliberately), `nothing_below_an_observed_position_appears_later` (`:5880`), `contains_event_id_reports_membership` (`:2551`) in the live run. Traces **AC-006**. |
| AC-009 | GIVEN an author whose events sit near the transport's ceiling, WHEN they append something too large, THEN the adapter refuses it **before** the round trip as `AppendError::ExceedsStoreLimit { limit: … }` — never as an `AppendError::Store` wrapping a 4xx, never a truncation; and independently, the guaranteed minima are **accepted**: 65,536 bytes of payload, 64 tags, a 128-item query, a 128-event batch, notwithstanding hex `bytea` roughly doubling them on the wire. | `append_reports_exceeded_store_limits` (`suite.rs:4282-4457`) and the four `store_accepts_the_guaranteed_minimum_*` rules (`:3775`, `:3829`, `:3880`, `:3954`) in the live run; an offline in-crate test asserting an over-ceiling append returns `ExceedsStoreLimit` against `NullTransport`, whose designed round-trip failure makes reaching the wire observable as the wrong error. Traces project **AC-010**. |
| AC-010 | GIVEN an author targeting `wasm32` where nothing is `Send`, WHEN they depend on this crate, THEN it still implements only the **bare** `EventStore`, has no `#[async_trait]`, has no second `SendEventStore` impl, keeps `read` non-`async` and lazy, and the `REQUIRED` wasm32 `cargo check` of `happenstance-neon` — selected by name in `xtask/src/main.rs:271`, `:784-789` — is still green under its current step name. | `both_stores_are_bare_event_stores`, `a_store_can_be_built_without_a_transport` and `the_generic_helper_call_site_compiles` in `crates/happenstance-neon/src/event_store.rs` `mod tests` (`:505-549`), still green; `cargo xtask wasm` and the wasm32 step inside `cargo xtask ci --fast`. Traces **AC-006**. |
| AC-011 | GIVEN a contributor with no Neon credential and no Docker, WHEN they run the default gate, THEN `cargo test --workspace --all-features` and `cargo xtask ci` are green with nothing reachable, because the live invocation is gated **whole** — and GIVEN the credentialed job, WHEN it runs, THEN every declined capability appears as an emitted test returning `RuleOutcome::Skipped` printing the fixture's own stated reason, with no `#[cfg]` removing any rule from the expansion. | `cargo xtask ci --fast` and `cargo xtask affected --base main` green on a clean checkout with no credential; the Neon job's `--show-output` transcript showing each skip's reason string; an assertion over `crates/happenstance-neon/tests/**` that no per-rule `#[cfg]` exists. Traces project **AC-007**. |
| AC-012 | GIVEN the next reader of this crate — an adapter author copying it as a worked example — WHEN they read its module documentation and its tests, THEN the schema block at `event_store.rs:36-54` describes the schema the adapter actually issues SQL against (migration 1, not `bigserial`), and no test anywhere in this story asserts a literal position value, because the specification permits gaps and this is the project where gaps stop being hypothetical. | `cargo xtask lint-position-literals`, a `REQUIRED` gate step (`xtask/src/main.rs:389`, `:687`; `xtask/src/lints.rs:628`); `cargo xtask ci`'s `docs` step over the reconciled block; review of the diff against `crates/happenstance-postgres/src/event_store.rs:22-35`. Traces **AC-006**. |

Coverage of the traced project ACs: **AC-006** by AC-001 – AC-006, AC-008,
AC-010 and AC-012; **AC-007** by AC-007 and AC-011 (and AC-001's "no rule
absent"); **AC-010** by AC-009. Project AC-007's other half — the fixture's
capability *declarations* and their written reasons — is
`neon-fixture-and-live-job`'s, per `_storymap.md:222-226`.

## Interaction quality

RFC §6.7/D6. Every invariant that applies is carried by an **AC-### row in the
table above**; this section says which row carries which, and nothing here is a
free-floating bullet.

**Composition family — declared N/A, not skipped.** This story renders no
surface. `_design.md` records `N/A — no user-facing surface` for Surfaces,
Items, Signatures, Placement, States, Anti-patterns and the doctest
(`_design.md:10-41`, `:44-84`), and a human approved *that determination itself*
at the sign-off gate (`:86-95`). `design.capture` is deliberately absent from
`.redkiln/config.yaml:75-80`, which makes the perceptual review a declared skip
rather than a silent pass. There is therefore no composed presentation, density
budget, transience policy or hierarchy to gate here, and inventing one would
contradict a signed-off design.

**State family, translated to this repository's medium.** The "surface" an
adapter author meets is the conformance run's output and the port's own
behaviour, and the state invariants have exact analogues here — each already an
AC row, so each is extractable, gated and tested:

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Non-occlusion** — nothing the author needs to see is hidden by something else | **AC-011** | A declined capability produces an *emitted* test returning `RuleOutcome::Skipped` with the fixture's stated reason (`crates/happenstance-testkit/src/lib.rs:46-53`, `contract.rs:213-235`); a `#[cfg]` that deletes a rule from the expansion is the occlusion this forbids. Verified by the job's `--show-output` transcript and by the rule count matching the registry. |
| **In-place, not a context jump** — the failure is reported where the author is looking | **AC-005**, **AC-006**, **AC-009** | A conflict surfaces as `ConditionViolated` at the append call, a ceiling as `ExceedsStoreLimit` at the append call, a decode ambiguity as an error rather than an `Ok` — never as an opaque `AppendError::Store` the author must go elsewhere to interpret. Verified by the named suite rules plus the offline error-mapping tests. |
| **Preserved state** — the author's position in the stream is not silently lost | **AC-003** | A response over the cap is terminal (`NeonError::ResponseTooLarge`), never a truncated result set, because a truncated read is a short prefix indistinguishable from the end of the stream. Verified by the offline oversized-body test. |
| **Reversibility / no destructive surprise** | **AC-004** | The empty batch is refused before a request exists, and there is never a probe-then-write window in which a partial effect can be left behind: one statement, atomic. Verified by the empty-batch rule and the one-statement offline assertion. |
| **Reachability** — every capability the port advertises is reachable through the real mount, not merely compilable | **AC-001**, **AC-010** | The suite is invoked from `crates/happenstance-neon/tests/`, which does not exist today; a `todo!()` body type-checks against any signature (`RUNBOOK.md:3061-3068`), so filling bodies without the invocation is the "constructed but unmounted" failure in this medium. |
| **Honest state reporting** — the artefact does not claim a state it is not in | **AC-007**, **AC-012** | The isolation claim and the schema block are two places the crate currently asserts something the wire and migration 1 do not support. Both are reconciled or corrected; a green run that leaves either standing implies evidence nothing produced. |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | Zero events handed to `append` | `AppendError::NoEvents`, decided before any `SqlRequest` is built (`crates/happenstance-neon/src/event_store.rs:188-190`). Already written; must survive. → AC-004 |
| **EC-002** | The append's CTE returns a `conflict` row | `AppendError::ConditionViolated(ConditionViolated::at(position))` (`event_store.rs:196-206`). Never `Store`, never a swallowed `Ok`. → AC-005 |
| **EC-003** | The endpoint aborts a **conditional** append with SQLSTATE `40001` | `AppendError::ConditionViolated(ConditionViolated::unspecified())` (`crates/happenstance-core/src/error.rs:150-157`), because on a conditional append a serialisation abort *is* the concurrency signal. → AC-006 |
| **EC-004** | The endpoint aborts an **unconditional** append with `40001` | `AppendError::Store(NeonError::Sql(..))`. The asymmetry is deliberate: nothing was conditioned, so nothing was violated, and reporting one would fabricate a concurrency signal. → AC-006 |
| **EC-005** | The append's returned row has both `appended` and `conflict` null, or both non-null | A decode error inside `AppendError::Store`. Never an `Ok`, never a guess. → AC-005 |
| **EC-006** | An append exceeds a declared ceiling | `AppendError::ExceedsStoreLimit { limit: StoreLimit::EventDataLen \| TagsPerEvent \| EventsPerBatch, .. }` (`crates/happenstance-core/src/limits.rs:54-74`), decided **before** `round_trip`. A 4xx from the endpoint arriving as `NeonError::Http` inside `AppendError::Store` is the forbidden spelling reaching the author by accident. → AC-009 |
| **EC-007** | A response body exceeds `config.max_response_bytes` | `NeonError::ResponseTooLarge`, terminal for that read. Never a truncated result set (`crates/happenstance-neon/src/error.rs:88-100`, `config.rs:59-72`). → AC-003 |
| **EC-008** | A `position` column will not fit `SequencePosition`'s `NonZeroU64` (negative, zero, or beyond range) | `NeonError::InvalidPosition { value }` (`crates/happenstance-neon/src/error.rs:106-115`). Not a panic, not a clamp, not a silent `0`. → AC-003 |
| **EC-009** | A column arrives with an OID the decoder does not handle, or a `\x…` body that is not valid hex | A named decode error — a new `#[non_exhaustive]` `NeonError` variant if and only if the existing six cannot name it (`error.rs:63-116`). Not `unwrap`, not a lossy default. → AC-003 |
| **EC-010** | `read_request` is handed a `Query`/`ReadOptions` combination it cannot compile | The signature returns `SqlRequest`, not `Result` (`event_store.rs:114-116`), so the failure is either made unrepresentable or carried into `NeonReadStream`'s state and surfaced on the first `poll_next` as a stream item. Widening the signature, making `read` `async`, and panicking are all forbidden (ADR-0001/ADR-0008, `CLAUDE.md` constraint 3). → AC-002 |
| **EC-011** | The credential or endpoint is absent on a contributor's machine | The whole invocation does not run and the default gate is green. Never a failing test, never a `#[cfg]` that removes individual rules. The mechanism is `neon-fixture-and-live-job`'s; honouring it is this story's. → AC-011 |
| **EC-012** | The transport itself fails (DNS, TLS, 5xx, timeout) during a suite run | `NeonError::Http`/transport error surfacing as `AppendError::Store` or a stream error — a **reported failure**, not a skip. A transport fault laundered into a capability decline makes infrastructure flakiness indistinguishable from a store limitation (`project.md` risk table; `_decomposition.md:596-630`). → AC-001 |

## Non-functional

| id | requirement | why, and how it is observed |
| --- | --- | --- |
| **NF-001** | **One round trip per port operation.** `append`, `read`, `head` and `contains_event_id` each issue exactly one HTTP request per invocation. | This is the axis the crate exists to occupy; a second request is a different adapter. Observed by the offline statement-count tests and the transport's request count in the live job. → AC-002, AC-004, AC-008 |
| **NF-002** | **The default gate stays offline.** `cargo xtask ci` on a clean checkout with no Docker, no credential and no network is green, and this story adds no new required step. | DR-9 (`project.md:212-217`); `.redkiln/config.yaml:40`, `:55`. → AC-011 |
| **NF-003** | **The wasm32 build stays green under its current step name.** The `REQUIRED` step is selected by name (`xtask/src/main.rs:271`, `:784-789`); renaming it silently removes a standing guard. | `CLAUDE.md` constraint 1; DR-8. → AC-010 |
| **NF-004** | **No `unwrap`/`expect`/`panic!` on any decode path.** Every wire-shaped failure is a named `NeonError` variant. | `standards/rust/00-prime-directives.md`; a panic inside a conformance rule reports as a harness fault rather than an adapter verdict, which destroys the run's evidentiary value. → AC-003, EC-009 |
| **NF-005** | **Ceilings are anchored on a real number, and the halving is accounted for.** `MAX_RESPONSE_BYTES` is 64 MiB (`crates/happenstance-neon/src/transport.rs:45-54`) and hex `bytea` rendering roughly doubles a payload in both directions (`:87-102`), so an effective ceiling is about half the nominal one. | A ceiling asserted without the halving passes in CI and fails in production. → AC-009 |
| **NF-006** | **The live job is bounded and reproducible.** The full expansion runs in one credentialed job whose duration is recorded, and no rule depends on wall-clock sleeps to observe concurrency. | `_decomposition.md:596-630`; the interleave rule uses poll ordering, not timing (`suite.rs:5411-5484`). → AC-001 |
| **NF-007** | **No new dependency edge outside the crate.** `happenstance-core` and `happenstance-testkit` are must-not-change seams; `happenstance-testkit` enters only as a **dev**-dependency of `happenstance-neon`, and no adapter-on-adapter edge is created. | `CLAUDE.md` dependency rule; `_decomposition.md:73-105`, `:485-488`. → AC-001 |

## Implementation notes (non-prescriptive)

Observations that will save time, not instructions. Where one conflicts with the
Context pack, the Context pack wins.

- **Start with `read_request` and the offline decode tests, not with the live
  job.** Every SQL shape in this story appears in `read_request` first — the
  disjunction, the tag superset match, the frontier predicate, the ordering and
  the limit. Getting it right offline against handcrafted `wire::ResultSet`
  documents means the first live run debugs credentials, not SQL.
- **`NullTransport` is a genuine test instrument, not a placeholder.** It fails
  every round trip by design (`transport.rs:267-298`), which makes it the ideal
  witness for "this decision happened *before* the wire": if `append` returns
  `ExceedsStoreLimit` or `NoEvents` against `NullTransport`, the refusal provably
  preceded the request.
- **Do the transport's own smoke test before wiring the fixture to the macro.**
  The testing brief asks for exactly this — one HTTP round trip against the live
  `/sql` endpoint decoding correctly, passing *before* the conformance macro runs
  — so a transport bug and a conformance failure are never debugged as the same
  failure (`_decomposition.md:596-608`). That test belongs to `neon-sql-transport`;
  if it is not there when this story starts, write the five-line version locally
  rather than debugging through the suite.
- **The interleave rule is where the schedule will go.** Expect the first live run
  to fail `interleaved_appends_on_one_handle_elect_one_winner`, and expect the
  failure to be either two `Ok`s (isolation too weak) or an `AppendError::Store`
  carrying `40001` (isolation right, mapping wrong). Both are informative; neither
  is a reason to touch the rule.
- **`ConditionViolated::unspecified()` exists for this.** It is not a degradation
  to be apologised for — it is the constructor for "the adapter knows it lost but
  not to whom" (`crates/happenstance-core/src/error.rs:150-157`). Whether the CTE
  can do better is `neon-conflicting-position-verdict`'s question, deliberately
  not this story's.
- **Tag encoding is a read-side decision made on the write side.** Store tags as a
  `text[]` in the canonical sorted encoding so `@>` remains available; the
  documented CTE's `unnest($2::text[], $3::bytea[], $4::bytea[], $5::text[][])`
  already assumes it (`crates/happenstance-neon/src/lib.rs:60-64`). Changing the
  encoding would need migration 1 to change, which is outside the boundary.
- **When the docs and the code disagree, fix the docs in the same commit as the
  code.** Both known disagreements — the isolation header and the `bigserial`
  block — are cheap now and become archaeology later.
- **Coordinate the manifest edit with the slice-mate.** `happenstance-testkit` and
  `tokio` are not dev-dependencies of this crate today
  (`crates/happenstance-neon/Cargo.toml:14-32`); `neon-fixture-and-live-job` may
  have added them already. Adding them twice is an avoidable merge conflict.
- **If the endpoint forces a contract amendment, stop and stage it.** DoD 6
  contemplates it explicitly. The route is `.kb/_intake/` and
  `/redkiln:kb-ingest`, never a hand-written atom and never an edit to a
  `[FROZEN]` clause (`_decomposition.md:159-164`, DR-7).

## Tests and CI (merge gate)

| tier | command / path | proves |
| --- | --- | --- |
| **Unit — offline, in-crate** | `cargo test -p happenstance-neon --all-features` over `crates/happenstance-neon/src/event_store.rs` `mod tests` (`:505-549`, extended) | One statement per operation; `NoEvents` and `ExceedsStoreLimit` decided before the wire (witnessed by `NullTransport`); `decode_append_response` over all four null combinations; `decode_read_response` over both envelopes, an oversized body and an out-of-range `position`; the `40001` mapping on both the conditional and unconditional paths; the flavour assertions still green. → AC-002 – AC-007, AC-009, AC-010 |
| **Conformance — live, credentialed** | `cargo test -p happenstance-neon --all-features -- --ignored --show-output` in the Neon CI job, driving `event_store_conformance!` from `crates/happenstance-neon/tests/conformance.rs` (`_decomposition.md:632-656`) | The whole suite against a store with no connection, no interactive transaction and no cursor: the interleave winner, CF-13, ES-11/ES-12, the limit rules, the guaranteed minima, `head`, `contains_event_id`, `SECOND_HANDLE` and `REOPEN` — every rule present and reporting, every skip carrying a stated reason. → AC-001, AC-003, AC-006, AC-008, AC-009, AC-011 |
| **Story gate** | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | The story grain: fmt, clippy `-D warnings` (which is what proves the `#[expect(dead_code)]` removal), the tests for `happenstance-neon` and its dependents, plus the five file-reading lints and `spec-trace` unconditionally. → AC-005, AC-012 |
| **Project gate (non-terminal)** | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | The whole `REQUIRED` set with no network and no Docker: all four wasm32 steps including the `happenstance-neon` build, docs, `spec-trace`, `package-check`, and the position-literal lint. → AC-010, AC-011, AC-012 |
| **Targeted lint** | `cargo xtask lint-position-literals` (`xtask/src/main.rs:389`, `:687`; `xtask/src/lints.rs:628`) | No test in this story asserts a literal position value. `REQUIRED`, so it fails the build rather than a review. → AC-012 |
| **wasm32** | `cargo xtask wasm` (`xtask/src/main.rs:271`, `:784-789`) | The bare-flavour, `!Send` story survives the SQL and decode work, under the step's current name. → AC-010 |
| **Docs** | the `docs` step inside `cargo xtask ci --fast` | The reconciled module-doc schema block builds and its intra-doc links resolve. → AC-012 |
| **Ledger** | `redkiln verify --grain story` against this story's `_ledger.md` (`.redkiln/config.yaml:67`, `:73`) | Every AC-### carries cited evidence, and changes inside the declared boundary carry commit provenance. → all |

**Not in this story's gate:** `event_store_concurrency_conformance!` (binds
`F::Store: EventStore + Send`; Postgres-only by design,
`crates/happenstance-testkit/src/lib.rs:101-110`), the Postgres `testcontainers`
job, `ProbeThenWriteStore` run as a negative control
(`neon-conflicting-position-verdict`), and running the suite on `wasm32` under a
real edge runtime (`cloudflare-durable-object-store`).

## Risks and coupling (PR-scoped)

| risk | likelihood | what it looks like | mitigation inside this PR |
| --- | --- | --- | --- |
| **The interleave rule cannot be passed at any isolation level this transport can express** | Medium-high — Context pack §4 and §5 together are this story's central uncertainty | Two `Ok`s at `ReadCommitted`, or `AppendError::Store` carrying `40001` at `Serializable`, with no header the one-statement path can carry | Resolve in the order §5 states: try a form that carries the level (a two-element batch, or a statement that sets it) before concluding the level is unreachable; map `40001` on a conditional append to `ConditionViolated::unspecified()`. If no sound shape exists, the deliverable is a DoD-6 amendment record staged through `.kb/_intake/` — **not** a weakened rule and **not** a skipped one. |
| **A live-infrastructure flake is mistaken for an adapter verdict** | Medium — the project risk table names it | A transport fault laundered into a capability decline, or a red job everyone learns to ignore | EC-012: transport faults surface as reported failures, never as skips. The transport smoke test (`neon-sql-transport`) runs before the macro, so the two failure classes stay separable. |
| **The story is blocked on migration 1's shape, which is authored outside its boundary** | Medium | The SQL needs a column migration 1 does not have | `crates/happenstance-postgres/**` is deliberately outside the PR boundary and the merge order puts `postgres-append-and-frontier-head` first (`_storymap.md:255-257`). A missing column is a coordination item raised loudly, never a second migration file authored here. |
| **`neon-fixture-and-live-job` and this story both edit `Cargo.toml`** | Medium | Duplicate `[dev-dependencies]` entries, or a conflict on the same lines | Slice-mates are implemented in one context (`neon-live-suite`); the manifest edit is recorded once as a deliberate decision, whichever story lands it. |
| **The offline gate silently stops covering anything** | Low-medium | The whole-invocation gate is written such that the invocation never runs anywhere, and a green Neon job means "nothing ran" | AC-001's rule-count check: the run's reported rule count must equal the registry's, so a zero-rule "pass" fails. |
| **Scope creep into `neon-conflicting-position-verdict`** | Medium — the CTE's `conflicting_position` question is right there | `probe_request` / `insert_request` / `decode_probe_response` / `decode_last_position` filled in, or `ProbeThenWriteStore` "fixed" | The PR boundary and the explicit not-in-this-PR list; `#![allow(clippy::todo)]` survives this PR by design, which is the visible signal that four `todo!()`s are meant to remain. |
| **A `[FROZEN]` clause gets edited to make a rule pass** | Low likelihood, high blast radius | `spec/SPECIFICATION.md` appears in the diff | `spec/SPECIFICATION.md` is outside the PR boundary; `cargo xtask spec-trace` runs unconditionally in the affected gate; DR-7 requires a decision atom, and atoms are authored through `/redkiln:kb-ingest`. |
| **Coupling: the testkit is treated as adjustable** | Low | A rule reworded, a bound relaxed, a rule added | `happenstance-testkit` is outside the PR boundary (`_decomposition.md:485-488`). If a rule seems wrong, that is a finding for the project review, not an edit here. |

## Dependencies

**Blocks on (must be merged first):**

| story slug | what this story consumes from it |
| --- | --- |
| `neon-fixture-and-live-job` | `NeonFixture` itself; its `SECOND_HANDLE` answer (a MUST — declining it panics `two_handles_observe_each_others_appends`), its `REOPEN` and `MID_BATCH_FAULT` answers; its three `Option<usize>` ceilings anchored on `MAX_RESPONSE_BYTES`; the whole-invocation gating mechanism; and the credentialed Neon CI job this story's run executes in. Transitively brings `neon-sql-transport`'s real `SqlTransport`, without which every round trip is `NullTransport`'s designed failure. |
| `postgres-append-and-frontier-head` | Migration 1 as amended — the tag storage, the identity and time columns, the `origin_store` / `origin_position` columns, and the visibility mechanism whose frontier predicate this crate's `read`, `head` and `contains_event_id` must carry. This is the edge that makes Postgres and Neon one project rather than two (`project.md:345-352`, `_storymap.md:176-178`). |

**Unlocks:**

| story slug | why it waits on this one |
| --- | --- |
| `neon-conflicting-position-verdict` | It runs `ProbeThenWriteStore` as a negative control against a live endpoint and settles whether the CTE keeps `conflicting_position` — both need a working append and decode path to exist first (project AC-008). |
| `deskeleton-and-package-readiness` | It cannot honestly remove `#![allow(clippy::todo)]` or `publish = false` until the last `todo!()` on the Neon path is gone; four survive this story on purpose and the story above removes them. |
| `far-end-discharge-record` | It records what ES-10, ES-11 and ES-12 discharged for the transport axis, which is exactly the evidence this run produces. |

**Not a dependency, deliberately:** `adr-0024-position-visibility-mechanism`. The
mechanism reaches this crate through migration 1 and
`postgres-append-and-frontier-head`; ADR-0024 is not cited here and no
`.kb/decisions/0024-*` file exists (`_decomposition.md:179-181`).

## Anchors (progressive disclosure)

Link, do not paste. Each row says why the artefact is load-bearing and the moment
to open it.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `crates/happenstance-neon/src/event_store.rs` | The mount point itself: the six `todo!()`s on the suite's path, the already-correct `append` control flow, `NeonReadStream`'s state machine, the stale `bigserial` doc block, and the in-crate flavour tests. | First, before writing a line. | AC-002, AC-003, AC-004, AC-005, AC-008, AC-010, AC-012 |
| `crates/happenstance-neon/src/lib.rs` | Carries the crate's own account of the CTE shape (`:46-77`), the isolation claim this story must make true or correct (`:74-77`), the flavour rationale (`:79-89`) and the skeleton allow (`:101-105`). | Alongside `event_store.rs`, before designing the append statement. | AC-004, AC-007, AC-010 |
| `crates/happenstance-neon/src/transport.rs` | Where the header suppression below two statements lives (`:56-60`, `:190-209`), where `MAX_RESPONSE_BYTES` is defined (`:45-54`), where hex `bytea` rendering halves the ceilings (`:87-102`), and `NullTransport` (`:267-298`) as the offline witness. | Before deciding how the isolation level travels, and before choosing ceiling numbers. | AC-007, AC-009 |
| `crates/happenstance-neon/src/wire.rs` | The two response envelopes, `ResponseBody::result_sets`'s normalisation, and `FieldDescription::data_type_id` as the only type discriminator — decoding is OID-directed or it is guessing. | Before writing either decoder. | AC-003 |
| `crates/happenstance-neon/src/error.rs` | The six existing variants, `InvalidPosition` (`:106-115`), `ResponseTooLarge` (`:88-100`) and `NeonSqlError`'s SQLSTATE carrier (`:39-49`) — the vocabulary every EC-### must be spelled in. | When writing the first error path, and again before adding any new variant. | AC-003, AC-006 |
| `crates/happenstance-neon/src/config.rs` | `NeonConfig`'s `Serializable` default (`:13-23`) and `max_response_bytes` (`:59-72`) — the two knobs the isolation and oversize behaviour are read from. | Alongside `transport.rs`, when resolving the isolation question. | AC-003, AC-007 |
| `crates/happenstance-testkit/src/suite.rs` | The rule bodies this run is judged by — `interleaved_appends_on_one_handle_elect_one_winner` at `:5411-5484` above all, and the limit and minima rules at `:4282`, `:3775`, `:3829`, `:3880`, `:3954`. Read the rule, never a paraphrase. | `:5411-5484` before designing the append's isolation; the limit rules before choosing refusal points. | AC-006, AC-009 |
| `crates/happenstance-testkit/src/contract.rs` | `Capability`, the declined-with-reason path (`:213-235`), the `SECOND_HANDLE` MUST that panics when declined (`:135-161`), and the ceilings' `Option<usize>` shape (`:237-252`). | Before wiring the macro invocation, to know what a skip must look like. | AC-009, AC-011 |
| `crates/happenstance-testkit/src/lib.rs` | The macro's arms (`:311-357`) — the exact invocation form — and the concurrency family's `Send` bound (`:101-110`) that explains why no rule here puts real contention on the append. | When creating `tests/conformance.rs`, and when writing AC-007's unfalsifiability note. | AC-001, AC-007 |
| `crates/happenstance-testkit/src/registry.rs` | The registry the macro expands from — the source of truth for how many rules a complete run must report. | When asserting AC-001's rule count. | AC-001 |
| `crates/happenstance-core/src/limits.rs` | `MIN_SUPPORTED_*` (`:22-43`) and `StoreLimit` (`:54-74`) — the guaranteed minima that must be accepted and the exact spelling a refusal takes. | Before implementing the pre-flight refusal. | AC-009 |
| `crates/happenstance-core/src/error.rs` | `ConditionViolated::at` vs `::unspecified()` (`:135-166`) and the standing assumption about `conflicting_position` (`:139-146`) that this story leaves standing. | When mapping the `40001` abort. | AC-005, AC-006 |
| `spec/E2E-CASES.md` | E2E-03 (`:103-124`) names the per-`QueryItem` statement shape as the wrong implementation and calls it "the natural shape for the Neon peer"; E2E-02 (`:78-100`) still carries "⚠ `happenstance-neon` (does not exist)". | Before compiling the read statement — this is the specification pre-rejecting the easy shape. | AC-002 |
| `spec/SPECIFICATION.md` | The clauses this run discharges evidence for: ES-10 (`:2816-2842`), ES-11 (`:2926`), ES-12 (`:2998`), ES-20 (`:3480`), ES-30 (`:3941`), ES-41 (`:4381`), VT-21 – VT-25 (`:1483` – `:1580`). Markers are read, never edited. | When a rule fails and the question is whether the adapter or the clause is wrong. | AC-002, AC-008, AC-009 |
| `.kb/decisions/0011-read-laziness-and-isolation.md` | The accepted decision behind `read`'s laziness and the snapshot obligation — why one round trip buys ES-11/ES-12 outright here. | Before touching `read` or `read_request`. | AC-002 |
| `.kb/decisions/0012-append-shape-and-preconditions.md` | The accepted decision fixing the append as condition-and-insert on one snapshot, with the empty batch refused first. | Before writing the CTE. | AC-004 |
| `.kb/decisions/0013-position-assignment-and-visibility.md` | The prior authority for the frontier — the reason `head` is not `max(position)` and read-your-own-writes is not claimed. ADR-0024 does not exist; this is what to cite. | Before implementing `head`. | AC-008 |
| `.kb/decisions/0001-async-port-flavours.md` | Why `read` is not `async`, why `+ Send` is never injected, and why a second `SendEventStore` impl is `error[E0119]`. | Before any change to a port method signature — i.e. before concluding `read_request` needs a `Result`. | AC-002, AC-010 |
| `.kb/open-questions/cf-40-fixture-limits-ownership.md` | The contested ownership of fixture limits between ADR-0012 and ADR-0015. This story **consumes** the surface; DR-6 forbids settling it unilaterally. | The moment a ceiling number starts to feel like a decision this story gets to make. | AC-009 |
| `.kb/open-questions/poll-count-bounds-the-visibility-rule.md` | What a poll-count-bounded rule can and cannot evidence — the frame for stating what the green run does not prove. | When writing AC-007's unfalsifiability note. | AC-007 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md` | Architecture §2 (composition roots, `:107-164`), §5 (Neon data flow, `:240-278`), §6 (fixtures, `:279-331`), §7 (gate wiring, `:332-363`), §9.2 (the transport seam, `:420-442`); Testing §6 (`:596-630`) and §7 (`:632-656`) for the exact live-job invocation. | §5 and §9.2 before the SQL; Testing §6–§7 before creating `tests/`. | AC-001, AC-002, AC-011 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/project.md` | The thirteen project ACs and the nine derived requirements — DR-4 (no pooled-Postgres stand-in), DR-5 (no silent skip, no `#[cfg]`), DR-8 (`!Send` must not regress), DR-9 (offline gate). | Before the first commit, and again before claiming the story done. | AC-001, AC-010, AC-011 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_storymap.md` | The slice boundaries and AC splits (`:222-226`), the merge order (`:255-257`), and this story's own row (`:168-178`) — including the "three `todo!()`s" claim the Executive summary corrects to six. | When a piece of work feels like it might belong to a slice-mate. | AC-001, AC-007 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_design.md` | The signed-off determination that this project renders **no** user-facing surface (`:10-41`, `:86-95`) — what makes the composition family of interaction quality N/A rather than skipped. | Before writing or reviewing anything in Interaction quality. | AC-011 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | The adapter author's stated fear (`:144-152`) and the four-adapters-one-storage-shape finding (`:130-143`) — the reason this run exists at all. | When an acceptance criterion needs to be argued from intent rather than mechanism. | AC-001 |
| `RUNBOOK.md` | Phase 10 in full (`:4311-4390`), the instrument portfolio and its empty transport axis (`:685-702`), and "a `todo!()` body type-checks against any signature" (`:3061-3068`). | At the start, for the plan of record; at the end, when writing the session-log entry. | AC-001, AC-010 |
| `crates/happenstance-postgres/src/event_store.rs` | The sibling adapter's schema commentary (`:22-35`) — the source of truth for what migration 1 declares, and therefore for what the Neon doc block is reconciled against. Read-only: outside this PR's boundary. | When reconciling the `bigserial` block. | AC-012 |
| `xtask/src/main.rs` | The `REQUIRED` step array (`:271`, `:389`, `:687`, `:784-789`) — which steps are named, and why renaming the wasm32 step silently removes a guard. | Before assuming a gate step exists or may be renamed. | AC-010, AC-012 |
| `.redkiln/config.yaml` | The `verify:` block (`:40`, `:55`), `require_ledger` and `require_commit_provenance` (`:67`, `:73`), and the deliberate absence of `design.capture` (`:75-80`). | Before running the gate, and before filling in `_ledger.md`. | AC-011, AC-012 |

## Clarifications resolved during spec

1. **The AC set is exactly the twelve the front half enumerated.** AC-001 –
   AC-012 are the `→ AC-nnn` annotations of the *Behavior and interfaces* table,
   one criterion per annotated obligation, with the paired rows collapsed where
   they are one obligation (the limits refusal with the guaranteed minima into
   AC-009; `head` with `contains_event_id` into AC-008; the isolation resolution
   with its unfalsifiability record into AC-007). Nothing was added and nothing
   dropped; `_ledger.md` carries exactly these twelve ids.
2. **Composition-family interaction quality is N/A by signed-off design, not by
   omission.** `_design.md` records no user-facing surface and a human approved
   that determination itself (`_design.md:86-95`); `design.capture` is
   deliberately absent from `.redkiln/config.yaml:75-80`. The state family was
   therefore translated into this repository's medium — the conformance run's
   output and the port's own error behaviour — and every invariant bound to an
   existing AC row rather than written as a prose bullet, so each one is
   extractable by `redkiln verify` and actually gated.
3. **Project AC-007 is split, and this story owns the run half.** The fixture's
   capability *declarations* and their written reasons are
   `neon-fixture-and-live-job`'s; what this story owes is that the adapter never
   makes a rule unable to report — no `#[cfg]`, no subset, full expansion
   (`_storymap.md:222-226`). AC-007 and AC-011 carry it.
4. **The isolation question is resolved *in* this story, not deferred to the
   verdict story.** `neon-conflicting-position-verdict` owns whether the CTE
   returns the *conflicting position*; whether the CTE runs at the level the crate
   claims is a soundness question this story's own rule
   (`interleaved_appends_on_one_handle_elect_one_winner`) forces, so it cannot be
   left open behind a green run. AC-007 carries it, including the obligation to
   correct whichever document is wrong.
5. **A serialisation abort maps differently on conditional and unconditional
   appends.** Both directions are stated (EC-003, EC-004) because mapping *every*
   `40001` to `ConditionViolated` would report a violation where nothing was
   conditioned — a fabricated concurrency signal, which is its own defect.
6. **ADR-0024 is cited nowhere in this story.** No `.kb/decisions/0024-*` file
   exists; the visibility mechanism reaches this crate through migration 1 and
   `postgres-append-and-frontier-head`, on ADR-0013's prior authority. That is why
   `adr-0024-position-visibility-mechanism` is not a `depends_on` edge.
7. **The `#[expect(dead_code)]` removal is an acceptance obligation, not a
   tidy-up.** Its reason string says "until phase 10", and an `expect` that stops
   firing is itself a warning under `-D warnings` — so the story cannot pass its
   own gate without removing it. AC-005 carries it.
8. **Four `todo!()`s survive this PR on purpose.** `probe_request`,
   `insert_request`, `decode_probe_response` and `decode_last_position` are
   reachable only through `ProbeThenWriteStore`, which
   `neon-conflicting-position-verdict` runs. `#![allow(clippy::todo)]` therefore
   stays, and its survival is the visible signal that the crate is not yet
   de-skeletoned — that is `deskeleton-and-package-readiness`'.
9. **`crates/happenstance-neon/tests/conformance.rs` is a path this story
   creates.** The crate has no `tests/` directory today; the file name is named
   here so the ledger's `verifying_test` column points somewhere definite, and the
   `mod_name` / `emit` arguments follow `crates/happenstance-testkit/src/lib.rs:311-357`.
