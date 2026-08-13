# Briefs — The edge store, run rather than asserted (HS-P0013)

Companion file. Not an item; the CLI never writes this path. It holds this
project's warranted briefs — architecture, testing, deployment — one section
each. `ux` is not warranted for this project
(`.bklg/from-contract-to-published-library/_decomposition.md:258`).

Every normative claim below cites a real path in this worktree. Where a brief
and a clause in `spec/SPECIFICATION.md` disagree, the clause wins; where a brief
and an **accepted** atom under `.kb/decisions/` disagree, the atom wins and the
brief is the thing that is wrong.

## Architecture brief

### Intent

Turn `crates/happenstance-cloudflare/` from an instrument into an adapter: bind
it to the real Workers `SqlStorage` API, give it a `Fixture`, and run the whole
event-store conformance family under `workerd` on `wasm32` **inside** one
`cargo xtask ci`. The architectural job is not "write SQL" — it is to decide the
five seams where this runtime disagrees with the port (`!Send` propagation, a
non-snapshot cursor, a 2^53 position ceiling, an error that carries a live JS
value, and a test harness that is not tokio) and to wire each of them into
machinery that already exists rather than beside it.

Scope boundary: this brief covers the **adapter's internal architecture, the
fixture seam and the gate wiring**. How the `workerd` process is obtained,
configured and made deterministic in CI belongs to the deployment brief; which
rules are written and what each falsifies belongs to the testing brief.

### Acceptance Criteria

Architecture-grain and checkable by reading the diff plus one gate run. Each
traces to a project AC in `project.md`.

- **ARCH-AC-01 (project AC-001).** `CloudflareEventStore` implements the **bare**
  `EventStore` (`crates/happenstance-cloudflare/src/event_store.rs:145-196`),
  `read` is still a non-`async` method returning `impl Stream` at the top level,
  no `#[async_trait]` appears anywhere, and no `todo!()` or
  `#![allow(clippy::todo)]` (`crates/happenstance-cloudflare/src/lib.rs:121-126`)
  survives.
- **ARCH-AC-02 (project AC-001, AC-005).** The `!Send` probe module and its four
  tests (`crates/happenstance-cloudflare/src/lib.rs:137-259`) survive the swap to
  `worker` **unchanged in intent** and still pass: `JsHandle`,
  `CloudflareEventStore`, `SqlRowStream` and `CloudflareEventStoreError` are
  `!Send`, and `the_probe_is_not_vacuous` still holds. If holding a
  `worker::Error` (whose `JsValue` payload is `Send + Sync` on non-`atomics`
  wasm32 — `crates/happenstance-cloudflare/src/lib.rs:33-61`) would restore
  `Send`-ness, the error type keeps an `Rc`-shaped field so it does not.
- **ARCH-AC-03 (project AC-003, AC-007a).** The read path implements ADR-0011's
  ceiling-and-page mechanism — a position ceiling captured no later than the
  first poll, bounding every later statement
  (`.kb/decisions/0011-read-laziness-and-isolation.md:19-21`, `:69-75`) — **or**
  the brief-time analysis that ruled it out against the real `SqlStorageCursor`
  is recorded in ADR-0023 with the compiled reason. `check_cursor_still_valid`
  (`crates/happenstance-cloudflare/src/event_store.rs:292-306`) is either deleted
  as unnecessary or kept as a belt-and-braces assertion, never as the primary
  mechanism.
- **ARCH-AC-04 (project AC-002).** The conformance harness is a **test target in
  this crate** invoking `happenstance_testkit::event_store_conformance!` with
  `emit = happenstance_testkit::__emit_wasm`, shaped exactly like
  `crates/happenstance-testkit/tests/memory_conformance_wasm.rs:19-27`. No new
  emitter, no rule list, no `#[cfg]` over any individual rule.
- **ARCH-AC-05 (project AC-003, AC-008).** One `impl Fixture` exists
  (`crates/happenstance-testkit/src/contract.rs:120-353`) in which a fixture
  instance is one Durable Object's storage and each `connect()` is one handle
  onto it; `SECOND_HANDLE` is `SUPPORTED` (it is a MUST —
  `contract.rs:135-161`); `REOPEN`, `MID_BATCH_FAULT` and the three
  `Option<usize>` ceilings each carry a real, measured answer.
- **ARCH-AC-06 (project AC-004).** The `workerd` execution is a `Step` in
  `xtask/src/main.rs`'s `REQUIRED` array (`xtask/src/main.rs:105`), selected by
  **name** in `wasm_steps()` (`xtask/src/main.rs:784-791`) through `steps_named`
  (`:816-826`), and its non-execution is a failure rather than a skip — see the
  guard discussion in Notes §5.
- **ARCH-AC-07 (project AC-001, AC-005).** `CloudflareEventStoreError` gains no
  `ConditionViolated` variant (`event_store.rs:100-104`): the DCB conflict signal
  is classified **before** `Self::Error` is constructed and travels in
  `AppendError::ConditionViolated`, and capacity refusals travel in
  `AppendError::ExceedsStoreLimit` naming a `StoreLimit`
  (`spec/SPECIFICATION.md:7661-7674`), never in `AppendError::Store`.
- **ARCH-AC-08 (project AC-012).** `crates/happenstance-cloudflare/` carries
  `LICENSE-MIT`, `LICENSE-APACHE` and `README.md` alongside `Cargo.toml` and
  `src/`, and `publish = false` (`crates/happenstance-cloudflare/Cargo.toml:12`)
  is removed, so `cargo xtask package-check` (`xtask/src/main.rs:750-752`) would
  hold for it.

### Notes

#### 1. The seam, module by module

Only two crates are edited by this project, plus one manifest and one gate file.
The dependency rule holds throughout: **no adapter may depend on another
adapter** (`CLAUDE.md`, *Dependency rule*), so nothing here may reach into
`crates/happenstance-sqlite/`, however identical the schema is.

| Path | What changes | What must not |
|---|---|---|
| `crates/happenstance-cloudflare/Cargo.toml` | `worker` added; `wasm-bindgen-test` added under `[target.'cfg(target_arch = "wasm32")'.dev-dependencies]`; `publish = false` removed | the deliberate no-`worker` note at `:19-25` is replaced, not deleted silently — it records why the stand-in existed |
| `src/js.rs`, `src/sql_storage.rs` | the stand-in is replaced by real bindings | the four modelled properties (`sql_storage.rs:1-24`) stay true of the replacement, and the `Rc`-shaped `!Send`-ness stays |
| `src/event_store.rs` | `migrate`, `append`, `head`, `contains_event_id`, `render_read`, `decode_row`, and the read state machine | `read` stays non-`async`, returning the stream at the top level (`:148-169`); `SqlRowStream`'s hand-written state machine stays hand-written (`:198-233` says why: `Send`-ness decided by fields, not inferred by a coroutine) |
| `src/send_shape.rs` | nothing structural | `send_flavour::SendStoreWithLocalError` keeps compiling — it is finding 4's standing proof (`lib.rs:87-94`) |
| `src/lib.rs` | crate docs rewritten from "not implemented"; `#![allow(clippy::todo)]` deleted | the `not_send_probe` autoref-specialisation module and its tests (`:137-259`) survive |
| `tests/` (new) | the `__emit_wasm` conformance target and the fixture | it is *the* shipped macro, not a bespoke harness |
| `xtask/src/main.rs`, `xtask/src/proof.rs` | one new `Step`; one new `Artefact` row | selection by name, never by index (`main.rs:769-775`) |

#### 2. The accepted atoms that constrain this, and the three tensions

**ADR-0001 / ADR-0008 — the two-flavour port**
(`.kb/decisions/0001-async-port-flavours.md`,
`.kb/decisions/0008-one-derivation-for-both-ports.md`). This adapter is the
reason both exist. Implement the bare `EventStore`; never `#[async_trait]`;
`read` returns the stream at the top level and is not `async`. Two tests in
`crates/happenstance-core/src/memory.rs` pin that shape and neither may be
deleted (`CLAUDE.md`, constraint 3).

*Tension (flagged, not resolved here).* `RUNBOOK.md:4280-4282` describes this
project as retiring ADR-0001's provisional marker. The marker was already lifted
at phase 1 by `LocalMemoryEventStore`, and ADR-0008 records the lift
(`.kb/decisions/0008-one-derivation-for-both-ports.md`). Accepted atoms are
immutable and `redkiln validate --kb` checks them against `HEAD`. **Do not edit
ADR-0001.** This project furnishes the *real-runtime evidence* behind an
already-accepted decision; the citation lands in ADR-0023 and in the long-form
record under `references/adr/`.

**ADR-0009 — `Error` stays unbounded** (`.kb/decisions/0009-error-send-sync.md`).
ES-6 is **settled, not deferred**: `Error` keeps `core::error::Error + 'static`
on both ports and flavours, and the stronger property lives in a downstream
blanket-implemented marker (`ThreadSafeEventStore: SendEventStore<Error: Send +
Sync>`), which deliberately does not ship in `happenstance-core`
(`.kb/decisions/0009-error-send-sync.md:68-85`).

*Tension (flagged).* Initiative AC-005 is worded as though ES-6 were open
("bound added, or ADR-0009's deferral confirmed"). That wording predates the
atom's acceptance. The real work under AC-005 is (a) keeping
`CloudflareEventStoreError` genuinely `!Send` once it carries a real
`worker::Error`, and (b) the committed test that reconstructs
constraint-violation-versus-transport from a **caller-visible** error. Re-opening
the bound question would contradict an accepted atom; if the implementer
believes the atom is wrong, that is a new decision record and a re-plan, never a
line edit.

**ADR-0011 — a read is one sample with a ceiling**
(`.kb/decisions/0011-read-laziness-and-isolation.md`). The single most
load-bearing citation here. The port's promise is *"evaluated against one state
sampled no later than the first poll"*; laziness is permitted, never required;
and the operative mechanism is stated: **an adapter issuing more than one
statement per `read` must capture a position ceiling no later than the first poll
and bound every later statement by it** — stated to discharge both isolation
clauses across three store shapes, one of which is named literally as *chunked
cursor* (`:69-75`). That is this adapter.

*Tension (flagged, with a leading candidate).* The crate's own docs
(`crates/happenstance-cloudflare/src/lib.rs:96-107`) frame the non-snapshot
cursor as an open capability trade-off between laziness and buffering, and
`SqlRowStream` only *detects* invalidation after the fact
(`event_store.rs:292-306`). ADR-0011 is accepted and already prescribes the fix.
**Treat ceiling-and-page as the leading resolution** — capture
`max(position)` (or bind `ReadOptions::to`) at first poll, then issue bounded,
`limit`-sized statements, re-`exec`ing rather than holding one cursor across an
await — and fall back to a declined capability only if the real
`SqlStorageCursor` API defeats it, with the compiled reason in ADR-0023. This is
consistent with project AC-007 listing "honoured" first.

**ADR-0014 — the store mints identity** (`.kb/decisions/0014-event-identity-and-recorded-time.md`).
`contains_event_id` is a **required** port method precisely because the provided
form would need `Self: Sync`, which this deliberately `!Sync` adapter cannot
supply — the ADR names this adapter's flavour by construction. `EventId` is a
`StoreId` + position pair behind private fields, so the schema needs identity
columns to answer it: `event_store.rs:187-195` already names them
(`origin_store`, `origin_position`). The intended schema (`:8-28`) predates
ingest and **must gain them**. `happenstance-sqlite`'s schema
(`crates/happenstance-sqlite/src/event_store.rs:34-54`) does not carry them
either and is itself an unfinished skeleton owned by `sqlite-durable-store`
(`:238` names the same two columns) — so follow the *shape*, and do not wait for
a finished sibling to copy. ADR-0014 also governs store-id incarnation: mint once
and re-mint only on a detectable restore or clone, otherwise mint a fresh
incarnation on every open.

**ADR-0003 — opaque payloads** (`.kb/decisions/0003-opaque-payloads.md`).
`data`/`metadata` stay opaque `Bytes` on the way through: `BLOB` columns,
`SqlValue::Blob` (`sql_storage.rs:37-49`), no `serde` in this crate's dependency
surface. The constraint binds `happenstance-core`'s features; the practical
consequence here is that the adapter never inspects a payload.

**ADR-0013 — position assignment and visibility**
(`.kb/decisions/0013-position-assignment-and-visibility.md`) meets a real
narrowing: Workers SQL widens integers through a JS number, so positions above
2^53 are not round-trippable even though `SequencePosition` is a `NonZeroU64`
(`lib.rs:108-112`, `sql_storage.rs:41-42`). This is a **declared store limit**
reported through `CloudflareEventStoreError::StoredPosition`
(`event_store.rs:136-142`), never an excused rule. Note the interaction with
`CLAUDE.md`'s rule that no conformance rule may assert on literal position values
— the ceiling is the store's fact, not the suite's assumption.

**ADR-0004 / ADR-0029 — MSRV 1.97.1**
(`.kb/decisions/0029-msrv-raised-to-1-97-1.md`). Adding `worker` widens the
"crates that declare no `rust-version`" surface the atom names. Check `worker`'s
own `rust-version` on introduction; if the floor has to move, that is an ADR, not
a silent edit (`CLAUDE.md`, binding constraint 5).

#### 3. Interfaces and data flow

**The port surface implemented** (all four on the bare flavour,
`event_store.rs:145-196`):

- `read(&self, query: &Query, options: ReadOptions) -> impl Stream<Item = Result<SequencedEvent, Self::Error>>`
  — no `async`, no `+ Send`. `exec` is synchronous
  (`sql_storage.rs:1-12`), so the deferral to the first poll is a choice about
  *sampling*, not about awaiting.
- `async fn append(&self, &[Event], Option<&AppendCondition>) -> Result<SequencePosition, AppendError<Self::Error>>`.
- `async fn head(&self) -> Result<Option<SequencePosition>, Self::Error>`.
- `async fn contains_event_id(&self, EventId) -> Result<bool, Self::Error>`.

**Read data flow (the ceiling-and-page shape).**

```
read()  ──▶ StreamState::Deferred { sql, query, options }        (nothing executed)
first poll ─▶ capture ceiling C = head()/max(position), synchronously via exec
           ─▶ render_read(&query, options, ceiling = C) ─▶ (statement, bindings)
           ─▶ exec ─▶ page of rows, every statement bounded by `position <= C`
later polls ─▶ drain the page; when exhausted and more may exist,
               re-exec bounded by (C, last_yielded_position) — never one cursor
               held across an await
Done
```

The state machine stays hand-written for the reason its docs give
(`event_store.rs:198-204`): a coroutine's `Send`-ness is inferred, and this
adapter's whole value is that its `Send`-ness is decided by its fields.
`ReadOptions::limit` truncates **matches**, not scanned rows
(`crates/happenstance-testkit/src/lib.rs:143`), so paging must not consume the
caller's limit against rows a query item excluded. `ReadOptions` gained an
inclusive `to` for exactly this ceiling (`.kb/decisions/0011-…:79-81`) — the
caller-side spelling of the same mechanism, so the adapter should compose the two
bounds rather than treat them as alternatives.

**Append data flow.** The whole object is one consistency boundary
(`event_store.rs:30-38`): single-threaded, exclusive storage, no second writer.
A `SELECT` for the condition followed by an `INSERT … RETURNING position` is
atomic provided **nothing awaits between them** — which is free here because
`exec` is synchronous. Order of refusals matters and is fixed by the suite: an
empty batch is refused *before* the condition is evaluated
(`crates/happenstance-testkit/src/lib.rs:145`).

**Error data flow, and the one classification that must happen early.**

```
worker::Error (thrown)  ─▶ SqlError::Thrown(JsThrow)          sql_storage.rs:95-100
                        ─▶ CloudflareEventStoreError::Sql      event_store.rs:105-143
                        ─▶ AppendError::Store(..)
                                  ▲
  constraint violation ───────────┘ NO. classify first, from the thrown
                                     Error's `message` text (there is no numeric
                                     code — lib.rs:63-73), and return
                                     AppendError::ConditionViolated
  capacity refusal ────────────────▶ AppendError::ExceedsStoreLimit { StoreLimit… }
```

Two structural facts hold this in place: there is deliberately **no**
`ConditionViolated` variant on the adapter's error (`event_store.rs:100-104`),
and CF-40 requires a stated ceiling to be refused as `ExceedsStoreLimit` naming
the corresponding `StoreLimit`, "never as `AppendError::Store`, and never by
truncating" (`crates/happenstance-testkit/src/contract.rs:239-247`,
`spec/SPECIFICATION.md:7661-7674`).

**Stand-in → real binding map** (what each item in the skeleton must become; the
exact spelling is ADR-0023's, the *obligations* are not):

| Stand-in | Real | Obligation carried over |
|---|---|---|
| `SqlStorage::exec` (`sql_storage.rs:176`) | `ctx.storage.sql.exec()` | stays **synchronous** — no future, no connection acquisition |
| `SqlCursor` (`:200-275`) | `SqlStorageCursor` | not a snapshot; do not hold across an await (see ADR-0011 above) |
| `SqlError::AlreadyBorrowed` (`:101-109`) | re-entrancy on one handle | two `append` futures from one handle polled alternately is a real case, and the suite has a rule family for it (`crates/happenstance-testkit/src/lib.rs:149`) |
| `SqlError::StorageLimitExceeded` (`:119-121`) | the DO's SQL storage cap | the input to the fixture's `MAX_*` constants |
| `JsHandle` / `JsThrow` (`src/js.rs`) | a `worker::Error`-carrying handle | must stay `Rc`-shaped so the type is genuinely `!Send` (`lib.rs:49-61`) |

#### 4. Composition roots — where each capability MOUNTS

This is the part most easily got wrong on a library, because there is no screen
to look at. Four real roots, each an existing file:

**(a) The adapter's construction root — `CloudflareEventStore::new(sql)`**
(`event_store.rs:70-87`). The store takes its `SqlStorage` by **injection**; it
must never construct one itself, because in production the handle comes off
`State::storage().sql()` inside a Durable Object class and in the fixture it
comes off whatever the harness has. Keep the constructor's shape; `migrate()`
(`:84-86`) is the schema seam and is what the fixture calls once per instance.
This is also where ADR-0014's store-id incarnation is decided — mint at
`migrate`/first open, persist it in the object's own storage, and read it back
so `contains_event_id` can answer for foreign origins.

**(b) The Durable Object host.** A conformance run needs a real DO class to hang
the store off. Nothing in the tree provides one today — `Cargo.toml:19-25`
records that `worker` was deliberately absent — so the project introduces it (an
`#[durable_object]`-shaped entry point plus a Worker entrypoint that the runner
can address). It is a **test-and-example** surface, not a second public API: the
adapter stays a library type that any DO can hold. Whether it lives behind
`#[cfg(test)]`, in `examples/`, or in a small `tests/` support module is an
implementation choice; what is not a choice is that the conformance fixture and
the documented production wiring reach the store through the *same* constructor.

**(c) The suite's mount point — one test target invoking the shipped macro.**
Copy the shape of `crates/happenstance-testkit/tests/memory_conformance_wasm.rs`
verbatim:

```rust
#![cfg(target_arch = "wasm32")]

happenstance_testkit::event_store_conformance!(
    mod_name = dcb_conformance_wasm,
    emit = happenstance_testkit::__emit_wasm,
    fixture = CloudflareFixture::new()
);
```

The macro's general arm is `mod_name = …, emit = …, fixture = …`
(`crates/happenstance-testkit/src/lib.rs:315-342`); `__emit_wasm` wraps each rule
in `#[::wasm_bindgen_test::wasm_bindgen_test]` and routes skips through
`RuleOutcome::skip_line` into `console_log!`, because `println!` writes nowhere on
this target (`crates/happenstance-testkit/src/registry.rs:260-288`,
`crates/happenstance-testkit/src/contract.rs:515-531`). That requires
`wasm-bindgen-test` in **this crate's** wasm-target dev-dependencies, mirroring
`crates/happenstance-testkit/Cargo.toml`'s
`[target.'cfg(target_arch = "wasm32")'.dev-dependencies]` block — the attribute
resolves in the caller's scope, never in the testkit.

The rule set is not restated anywhere: it comes from
`for_each_event_store_rule!` (`crates/happenstance-testkit/src/lib.rs:84-89`),
which is what makes "every rule, no wasm-only subset" a structural property
rather than a promise. CF-23 is `[FROZEN]` and already names this shape
(`spec/SPECIFICATION.md:7920-7951`).

*The concurrency family is a stated non-invocation, not an omission.*
`event_store_concurrency_conformance!` binds `F::Store: EventStore + Send` and
its module is `#[cfg(not(target_arch = "wasm32"))]`
(`crates/happenstance-testkit/src/lib.rs:100-110`, `:172-173`) — a `!Send`
adapter cannot invoke it and is not expected to. Say so in the crate's own docs
and in ADR-0023 with the reason; leaving it silently absent is exactly the
failure BR-13 exists to prevent.

**(d) The fixture — `impl Fixture for CloudflareFixture`**
(`crates/happenstance-testkit/src/contract.rs:120-353`). The mapping to this
runtime, and it is the design decision the testing brief inherits:

- `type Store = CloudflareEventStore` — bound on `EventStore`, the weaker
  flavour (`contract.rs:120-125`).
- **One fixture instance = one Durable Object's storage** (one object id / one
  fresh namespace entry). Two instances must share nothing — that isolation is
  itself a conformance rule (`contract.rs:17-23`), and pointing every instance at
  one object is the adapter mistake it exists to catch.
- **`connect()` = one handle onto that object** — a second stub, or a clone of
  the `SqlStorage` handle, whose aliasing semantics `SqlStorage`'s `Clone`
  already models (`sql_storage.rs:135-147`).
- `SECOND_HANDLE` is a **MUST** and must be `SUPPORTED`; the rule *panics* on a
  declined value rather than skipping (`contract.rs:135-161`).
- `REOPEN`: the trait's own documentation names a Durable Object as the
  motivating case — storage outlives the isolate, so handle state can be
  discarded and the store read again, while restarting the isolate from inside a
  test is not possible (`contract.rs:163-173`). Support it if the runner permits
  discarding handles; decline with that exact reason if not.
- `MID_BATCH_FAULT` defaults to declined (`contract.rs:207-211`). A DO *can*
  arm one (a `CHECK` constraint or trigger armed for one write), so an
  honest `SUPPORTED` is available here and would make this the first adapter to
  exercise CF-39. Whether to is the testing brief's call; the architecture only
  requires that `arm_mid_batch_fault` be overridden if the constant says
  supported (`contract.rs:281-307`).
- `MAX_EVENT_DATA_LEN`, `MAX_TAGS_PER_EVENT`, `MAX_EVENTS_PER_BATCH` are
  `Option<usize>` **facts, not trades** (`contract.rs:214-279`). Stating a number
  is a promise: exactly that many bytes accepted, one more refused as
  `ExceedsStoreLimit`. Measure them against the real runtime; a guessed number
  fails `append_reports_exceeded_store_limits` in one direction or the other, and
  that is the rule doing its job.
- A declined capability is **still emitted as a test** and reports the fixture's
  stated reason (`contract.rs:26-42`, `RuleOutcome::Skipped` at `:473-483`).
  Never `#[cfg]` a rule out.

**(e) The gate root — `xtask/src/main.rs`.** The step list is a
`const REQUIRED: &[Step]` (`:105`) of `Step { name, program, args, env, probe }`
(`:72-103`). A `workerd` execution step is a new row there, and
`wasm_steps()` (`:784-791`) gains its **name** — never an index; the module doc
at `:769-782` records that an index-selected step once pointed at clippy while
printing green. `steps_named` panics on an unresolvable name (`:816-826`), which
is the intended failure mode.

Two existing wasm32 steps stay and are not what this replaces: `cargo check
--tests` of the testkit harnesses (`:219-244`, whose own comment says it stops
`__emit_wasm` rotting and explicitly does **not** run anything) and the `cargo
check` of this crate (`:245-264`). AC-004's execution step is **new
infrastructure**, not a rewiring — no step in `xtask/src/main.rs` runs anything
under `wasm-bindgen-test-runner` or `workerd` today.

**(f) The anti-vacuity root — `xtask/src/proof.rs`.** DoD 1 requires that
*emptying* the conformance target fails the gate, not just deleting the step.
That machinery exists: `Artefact { package, target, tests }` (`proof.rs:58-72`)
asserts named tests out of `cargo test --list` before running them, precisely
because "`cargo test` exits 0 on `running 0 tests`" (`main.rs:156-191`).
`ARTEFACTS` currently carries three rows (`proof.rs:133-148`). Register the
Cloudflare conformance target as a fourth, or state in ADR-0023 why the
`workerd` runner's own output already discharges it. Do not leave this
unaddressed — it is the difference between a gate and a decoration.

#### 5. A probe-gated step is not a guard

`xtask` skips an OPTIONAL step when its probe fails, and `main.rs:192-202`
states the rule this project must not break: *a constraint whose only check is
skippable is unguarded on every machine that lacks one tool.* The mandatory
`wasm32` check of `happenstance-core` is deliberately a plain `cargo check` for
exactly that reason, with the powerset widening left optional above it.

`workerd` is an external tool, so the honest options are: (i) mandatory step,
gate fails without the tool; (ii) probe-gated step **plus** a mandatory
non-skippable assertion that the target and its rule enumeration exist (the
`proof-artefact` shape); or (iii) a documented blocking finding. Project AC-004
demands that "a configuration in which the step silently does not run fails the
gate rather than skipping it", which rules out a bare (i)-shaped probe with no
compensating mandatory check. Choosing between (i) and (ii), and pricing
`workerd`/`vitest-pool-workers` in CI, is the **deployment brief's** decision —
`RUNBOOK.md:4267-4268` proposes `vitest-pool-workers` in its own CI job, which is
a starting hypothesis and *not* the same artefact as "the same run as the rest of
the gate" (AC-07/DoD 4). If no `workerd` runner can exist inside `cargo xtask ci`
at acceptable cost, that is a blocking finding to escalate, not a degradation to
absorb.

#### 6. Deliberately non-prescriptive — the implementer's latitude

These are open by design, and every one of them is ADR-0023's material rather
than this brief's:

- **The SQL rendering strategy for a DCB query.** One statement or a bounded
  page per item; how tags AND within an item and OR across items are expressed;
  whether `event_tag` is joined or the `tags` blob is filtered. `event_store.rs:308-314`
  argues for one statement on the grounds that the object is the consistency
  boundary; ceiling-and-page may change that, and that is allowed — ADR-0011
  anticipates a multi-statement read, it just requires the ceiling.
- **How the ceiling is captured** — `max(position)` read, `ReadOptions::to`
  composition, or a monotonic column — and what it costs.
- **Whether the thrown value is kept live (`JsThrow`) or stringified
  (`StringifiedThrow`).** The trade is recorded (`lib.rs:63-73`): stringifying
  loses *forward* compatibility, not the one signal a caller branches on. But
  stringifying would also make the error `Send + Sync` and destroy the ES-6
  instrument (`sql_storage.rs:83-92`), so the default is to keep it live — argue
  the case if you deviate.
- **The exact `worker` API surface bound** (`SqlStorage` vs `storage().sql()`,
  cursor iteration style, blob marshalling), and the resulting `Cargo.toml`
  feature set / `cargo deny` consequences.
- **The fixture's numeric limits**, which are measurements, not choices.
- **CF-40's ownership** (`.kb/open-questions/cf-40-fixture-limits-ownership.md`)
  is contested between ADR-0015 and ADR-0012 *by ADR-0015's own
  self-contradicting text*, and the atom names phase 8 (`sqlite-durable-store`,
  merge position 3) as what forces it while this project merges at position 4.
  **Neither project may mint an answer independently.** Whichever reaches it
  first owns the resolution and the other cites it; the atom is *resolved, not
  deleted*. Coordinate before authoring, per the decomposition's non-goals
  (`.bklg/from-contract-to-published-library/_decomposition.md:44`).
- **WF-11's falsifier** (`.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md`)
  — serde has no streaming entry point for a human-readable string, so *any*
  such encoding materialises the whole payload. This runtime is the first place a
  memory ceiling and a large payload meet. Gather the evidence; **change no wire
  format** — that is `replication-identity-and-ingest`'s.
- **ES-32 (the tail seam)** — one recorded paragraph, not acted on
  (`spec/SPECIFICATION.md:4021`, `RUNBOOK.md:4273-4275`).

#### 7. Standing detectors that must survive the change

If any of these goes quiet, the change is wrong even if the gate is green:

1. The four `!Send` probe tests (`lib.rs:180-259`), including
   `the_probe_is_not_vacuous` — without the positive control the module would
   pass with a broken probe.
2. `send_shape::send_flavour::SendStoreWithLocalError` still compiling
   (`lib.rs:87-94`) — finding 4, which is what makes ADR-0009's marker necessary.
3. The two `read`-shape tests in `crates/happenstance-core/src/memory.rs`
   (`CLAUDE.md`, constraint 3) — `spawns_from_generic` is the one that rejects an
   `async fn read` refactor.
4. `registry::no_orphan_rules` and the single `for_each_event_store_rule!`
   enumeration — a wasm-only rule list anywhere in the tree fails AC-002 by
   construction (`spec/SPECIFICATION.md:7952-7960`).
5. `cargo xtask spec-trace` — CF-14, CF-23, CF-27, CF-39, CF-40, ES-6, ES-7,
   ES-9, ES-17, ES-32 and WF-11 all carry citations this project touches, and
   markers may not rot into decoration.

## Testing brief

### Intent

Prove that `happenstance-cloudflare` is an adapter, not an instrument, by the
same discipline every other store in this workspace is held to (`CLAUDE.md`,
*The rule that matters*): invoke `happenstance_testkit::event_store_conformance!`
and pass it, then go further than any sibling adapter has had to, because this is
the one project whose whole proof artefact is that a rule *actually executes* on
its target rather than merely type-checking there
(`.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_intake-brief.md`,
**Proof artefact**). The test mix below has four tiers and a fifth, standing one:
static checks that need no runtime, host-native unit tests that survive the swap
to `worker`, the wasm32 conformance run that is this project's actual deliverable,
a small set of targeted runtime tests for the two things the conformance suite
cannot see (ES-6's caller-visible error, WF-11's falsifier), and the pre-existing
standing detectors the architecture brief already named (Notes §7) that this brief
does not repeat.

**What this project does *not* own.** Whole-initiative DoD 1–15, re-observed as a
set on the assembled library, is `closeout-and-durable-audience`'s (HS-P0019),
the only project wired to the terminal `verify.e2e` grain
(`.bklg/from-contract-to-published-library/cloudflare-durable-object-store/project.md`,
**Out of scope**). There is no true end-to-end tier here — the highest tier this
project reaches is "every conformance rule, executed for real on the target
runtime," which is this project's own boundary, not the initiative's.

### Acceptance Criteria

Every project `AC-###` (`project.md`, **Acceptance criteria**) mapped to at least
one test tier, the seam that proves it, and the command that runs it in the gate.

| AC | Tier | Proven by | Gate command |
|---|---|---|---|
| AC-001 (real adapter, no `todo!()`) | static + unit | `cargo clippy … -D warnings` fails on any surviving `#![allow(clippy::todo)]`; `crates/happenstance-cloudflare/src/lib.rs` grep finds none (DoD 3) | `cargo xtask ci` clippy step (`xtask/src/main.rs:131-143`) |
| AC-002 (every rule runs, no wasm-only subset) | integration | `event_store_conformance!` expansion against `CloudflareFixture`, emitted through `__emit_wasm`; `registry::no_orphan_rules` (cited in architecture brief Notes §7.4) proves the rule set is the one true enumeration, not a hand-copied subset | the new `workerd`-execution `Step` (see Deployment brief) |
| AC-003 (declined capabilities report a reason) | integration + unit | `RuleOutcome::Skipped` lines in the wasm test-runner's output (`crates/happenstance-testkit/src/contract.rs:473-483`); a unit-level check that `CloudflareFixture`'s declared `Capability` constants each carry a non-empty reason (`Capability::declined` rejects an empty one, `contract.rs:115-117`) | `--show-output` on the run (mirroring `xtask/src/main.rs:131-143`'s reasoning, ported to the new step) |
| AC-004 (workerd inside the gate, not beside it) | integration + static (anti-vacuity) | the `workerd` `Step` executes as part of a single `cargo xtask ci`; `xtask/src/proof.rs`'s `Artefact`-style assertion (Architecture brief §4f) makes an emptied target a hard failure rather than a silent `running 0 tests` pass | `cargo xtask ci` end to end, plus `cargo xtask proof-artefact` if a fourth `Artefact` row is added |
| AC-005 (ES-6 decided with an artefact) | unit (host, today) → unit (wasm32, once `worker` lands — see Notes §2) | `the_error_type_is_not_send` and the other three probe tests (`lib.rs:180-259`) survive the swap; a new committed test reconstructs constraint-violation-vs-transport from a caller-visible `CloudflareEventStoreError` built over a real `worker::Error` | `cargo test -p happenstance-cloudflare` (host) or the wasm32 run (post-swap; Notes §2 flags the target question as open) |
| AC-006 (ADR-0023 accepted) | process gate, not a test | `redkiln validate --kb` checks the new atom's `KbFrontmatter`; the atom is authored via `/redkiln:kb-ingest`, never hand-written (`CLAUDE.md`, **Where the work lives**) | `redkiln validate --kb && redkiln doctor` |
| AC-007 (cursor + position-ceiling resolutions stated) | integration (ceiling-and-page path) + unit (the 2^53 boundary) | a conformance rule exercising a read that would span the cursor invalidation window, run under `workerd`; a targeted unit/wasm test asserting `CloudflareEventStoreError::StoredPosition` is returned exactly at the boundary (never by silent truncation, per `CLAUDE.md`'s rule against literal-position assertions — the boundary is the store's declared fact, so the test constructs the condition rather than asserting a bare number) | conformance run + `cargo test -p happenstance-cloudflare` |
| AC-008 (CF-39/CF-40 discharged, ownership resolved) | integration (numeric limits) + process gate (KB atom) | `append_reports_exceeded_store_limits` family against `CloudflareFixture`'s measured `MAX_EVENT_DATA_LEN`/`MAX_TAGS_PER_EVENT`/`MAX_EVENTS_PER_BATCH`; `.kb/open-questions/cf-40-fixture-limits-ownership.md` moved to resolved | conformance run + `redkiln validate --kb` |
| AC-009 (CF-14/CF-27 re-read) | static (documentation, spec-traced) | a written statement per clause; `cargo xtask spec-trace` confirms the citations still resolve and the markers are not stale | `cargo xtask spec-trace` |
| AC-010 (ES-32 verdict recorded, not acted on) | none automated — reviewed | one `RUNBOOK.md` paragraph; no test asserts prose, so this is a review-gate item, not a tier | code review of the diff (no command) |
| AC-011 (WF-11's falsifier tested) | integration | a targeted test that forwards a payload sized to exceed this runtime's practical memory ceiling through the adapter's actual (non-streaming) encode/decode path, run under `workerd` where the ceiling is real, and asserts what happens (works, or fails legibly) | the `workerd` execution step |
| AC-012 (publish-ready, gate green) | static | `cargo xtask package-check` (`xtask/src/main.rs:750-752`) asserts `LICENSE-MIT`, `LICENSE-APACHE`, `README.md` are inside the packaged artifact; `cargo xtask ci` green including all `wasm32` steps | `cargo xtask ci` (`package-check` step) |

DoD 1, 2 and 4 (`project.md`, **Definition of done**) are exactly AC-004, AC-003
and AC-005 restated at the boundary and are not re-listed as separate rows. DoD 3
is AC-001's grep. DoD 5 and 6 are AC-006/AC-008/AC-010's KB and RUNBOOK checks.
DoD 7 (`cargo xtask affected --base main` per story, `cargo xtask ci --fast` for
this non-terminal project) is the merge-gate section below, not a per-AC row.

### Notes

#### 1. The test mix, by tier

**Static** — no runtime, same for every crate in the workspace (`CLAUDE.md`,
**Commands**): `cargo fmt --all --check`; `cargo clippy --workspace --all-targets
--all-features -- -D warnings`; `cargo xtask spec-trace`; `cargo xtask lint-constitution`;
`cargo xtask package-check`; the four (soon five, Deployment brief) `wasm32` steps
that are plain `cargo check`/`cargo build` rather than a run
(`xtask/src/main.rs:192-283`); the optional `cargo hack` powerset and `cargo deny`
steps, both of which resolve on this machine and therefore run rather than skip
(`CLAUDE.md`, **Commands**). This tier is where AC-001, AC-009 and AC-012 are
mostly proven, and where a `worker` dependency's own `rust-version` is checked
against ADR-0029's floor (Architecture brief Notes §2, ADR-0004/ADR-0029) and
against `cargo deny`'s widened licence/advisory graph (Deployment brief §3).

**Unit (host-native, today)** — `crates/happenstance-cloudflare/src/lib.rs`'s
`not_send_probe` module and its four tests (`lib.rs:154-259`) run today as an
ordinary `#[cfg(all(test, not(target_arch = "wasm32")))]` block, proven by plain
`cargo test -p happenstance-cloudflare`. `send_shape::send_flavour` (Architecture
brief §7 finding 2) is a standing compile-time proof, not a runtime test, and is
checked by the same `cargo test`/`cargo check` invocation compiling the crate at
all. See Notes §2 for why this tier's *target* is an open question once `worker`
lands.

**Integration (the conformance suite, on the real target)** — this project's
actual deliverable. One test target,
`crates/happenstance-cloudflare/tests/…` (Architecture brief §4c), invoking
`event_store_conformance!` with `emit = happenstance_testkit::__emit_wasm`, run
under a wasm32-capable, single-threaded runtime that models a Durable Object.
This is the tier AC-002, AC-003, AC-007, AC-008 and AC-011 are proven on, and it
is the tier this project has to build the execution seam for (Deployment brief).
It subsumes what the reference native adapters get for free from
`__emit_tokio`/`#[tokio::test]` — the testkit's own doc table names all three
emitters and this is the third one demonstrated for real
(`crates/happenstance-testkit/src/lib.rs:60-67`).

**Targeted runtime tests, outside the conformance macro** — AC-005's
constraint-vs-transport reconstruction and AC-007(b)'s position-boundary case are
*not* rules the conformance suite states, because they are properties of this
adapter's error type and this adapter's numeric ceiling, not of the DCB
specification every adapter shares. They are written as ordinary `#[test]`s (or
`#[wasm_bindgen_test]`s, per Notes §2) in this crate's own tree, the same way the
`not_send_probe` module already is one for ES-6's `!Send` half.

**Process-gate tier (not a test at all)** — AC-006, AC-008's ownership half and
AC-010 are proven by `redkiln validate --kb`, `redkiln doctor`, and `cargo xtask
spec-trace`, plus a human reading the diff for AC-010's prose paragraph. No test
runner is the right tool for "is this decision atom accepted and immutable" —
that is what the redkiln CLI and the KB frontmatter schema already check
(`CLAUDE.md`, **Where the work lives**).

#### 2. An open question this brief flags rather than resolves: which target hosts the unit tier after the swap

Today `not_send_probe` and its tests compile and run on the **host**
(`#[cfg(all(test, not(target_arch = "wasm32")))]`, `lib.rs:154-155`), because the
stand-in `js::JsHandle`/`sql_storage` types have no platform dependency. Once
`worker` is a real dependency (Architecture brief, the seam table), `JsHandle`'s
replacement carries a `worker::Error`, and `worker`'s own bindings are declared
through `wasm-bindgen`/`web-sys` externs whose *linking* — not type-checking —
target-gates to `wasm32-unknown-unknown` in practice. Two consequences the
architecture brief's ARCH-AC-02 does not settle and this brief will not settle
either, because it is implementation-grain:

- If `cargo test -p happenstance-cloudflare` (no `--target`) stops linking once
  `worker` lands, the `not_send_probe` tests have to move behind
  `#[cfg(all(test, target_arch = "wasm32"))]` and re-emit through
  `wasm_bindgen_test` — mirroring exactly the reasoning the module's own doc
  comment already gives for why it is gated off `wasm32` *today* in the other
  direction (`lib.rs:151-163`: "a dev-dependency that only exists to run four
  assertions is a dev-dependency `cargo deny` has to clear on every run" — an
  argument that inverts once `wasm-bindgen-test` is already a dependency for the
  conformance target anyway).
- If it keeps linking (because `worker::Error` can be constructed without a live
  Durable Object binding, only a thrown `JsValue`), the unit tier stays
  host-native and AC-005's committed test can live there too, which is simpler
  and is the default this brief recommends absent evidence otherwise.

Whichever way it falls, the **four existing assertions must keep passing
somewhere reachable by an ordinary `cargo test`**, per Architecture brief Notes
§7 item 1 — a probe that only runs under `workerd` is a probe an ordinary
contributor's inner loop never exercises.

#### 3. Fixtures and seams to mock

- **`CloudflareEventStore::new(sql: SqlStorage)`** (Architecture brief §4a) is the
  one seam every tier goes through. The fixture, the AC-005 error test and any
  future test all construct or receive a `SqlStorage` handle and hand it to this
  constructor — never let a test construct a store some other way, or the
  fixture's "one instance, one object" invariant (Architecture brief §4d) has a
  second, untested code path.
- **The Durable Object host** (Architecture brief §4b) is the seam the conformance
  tier's fixture reaches through; it is a test-and-example surface, and the
  fixture and the documented production wiring must reach the store through the
  *same* constructor, so a test never verifies a shape production does not use.
- **`MID_BATCH_FAULT`** (`contract.rs:207-307`) — if the fixture claims
  `SUPPORTED`, `arm_mid_batch_fault` needs a real seam: a `CHECK` constraint or
  trigger armed for exactly one write. This is the only capability in the suite
  this adapter can plausibly support that no other adapter in the workspace has
  exercised yet (Architecture brief §4d) — worth calling out because it is new
  *conformance suite* coverage, not just new adapter coverage, and belongs in
  the changelog entry `mutation_coverage.rs`'s CF-29 lint requires for any rule
  a fixture newly exercises for real (`xtask/src/main.rs:20-24`).
- **A thrown `worker::Error` for AC-005** — mocked by constructing a `JsValue`
  carrying the exact message text a Durable Object's SQLite surfaces for a
  `UNIQUE constraint failed` (`lib.rs:63-73` already names the real string), not
  by a live Durable Object round-trip. This keeps the ES-6 test cheap and
  independent of whether the conformance harness's `workerd` runner exists yet.
- **A too-large payload for AC-011** — sized against whatever this runtime's real
  memory ceiling turns out to be (measured, not guessed, same discipline as the
  fixture's `MAX_EVENT_DATA_LEN`), forwarded through the adapter's actual
  encode/decode path with no format change (project non-goal, `project.md`
  **Out of scope**).

#### 4. Merge-gate commands

Per `CLAUDE.md`, **Commands**, and `.redkiln/config.yaml`'s `verify:` wiring
(referenced there): `cargo xtask affected --base main` is the story-grain check
every `redkiln advance` seam runs; `cargo xtask ci --fast` (`REQUIRED` only, no
`OPTIONAL` powerset/deny/nightly-docs steps — `xtask/src/main.rs:835-843`) is the
bar `verify.integration_scoped` holds this **non-terminal** project to; the full
`cargo xtask ci` is reserved for the terminal project's `verify.e2e` grain
(`closeout-and-durable-audience`). One consequence worth stating plainly because
it is a real cost, not a formality: **if the new `workerd`/wasm32-execution step
is placed in `REQUIRED`** (Deployment brief's central decision), then every
`cargo xtask ci --fast` run during this project's own story-by-story
implementation needs a working `workerd`/`wasm-bindgen-test-runner` toolchain
locally, not just in CI — `run_fast` runs `REQUIRED` unmodified
(`xtask/src/main.rs:853-860`). That cost belongs to the deployment brief's choice
of §5's options (i)/(ii)/(iii), not to this one, but it is this brief's job to
flag it as a testing-workflow consequence rather than let it surface as a
surprise mid-project.

#### 5. What a wrong implementation would slip past a weaker rule set

Per `CLAUDE.md`'s corollary ("a rule that no adapter can fail is decorative"):
a `CloudflareFixture` that declared `SECOND_HANDLE: Capability::declined(...)`
would be caught immediately — it is a MUST and the rule panics rather than skips
(`contract.rs:145-161`, cited in Architecture brief §4d) — so that failure mode
is already guarded and this brief does not need a new rule for it. The genuinely
new failure mode this project's own tests must catch, because nothing upstream
can: a `CloudflareFixture` that quietly points every fresh instance at the *same*
Durable Object storage would pass every rule that does not specifically construct
two fixture instances and check isolation — which is exactly
`crates/happenstance-testkit/src/contract.rs:17-23`'s isolation rule, already in
the shared suite. No new rule is owed here; the obligation is only to *run* the
existing one for real, which is this whole project's point.

## Deployment brief

### Intent

Get the `workerd`/wasm32 execution that AC-004 and DoD 4 require **inside**
`cargo xtask ci`'s single invocation, on every runner the `gate` job's matrix
covers (`.github/workflows/ci.yml:34-39`: ubuntu, windows, macos), without
creating a second, separately-maintained wasm32 subset — the exact outcome
DR-3/AC-07 forbid
(`.bklg/from-contract-to-published-library/cloudflare-durable-object-store/project.md`,
**Derived requirements**). Claim `happenstance-cloudflare` on crates.io as a
*reservation*, remove `publish = false`, and leave an actual, versioned publish to
`publication-and-positioning` (HS-P0016), per this project's own out-of-scope list
(`project.md`, **Out of scope**). There is no running service to roll out and no
data to migrate — this is a library crate — so most of the classic deployment
dimensions are N/A by construction, and this brief says so explicitly rather than
leaving them unaddressed.

### Acceptance Criteria

- **DEPLOY-AC-01 (project AC-004, DoD 1).** A single `cargo xtask ci` on a clean
  checkout, on all three `gate` matrix runners, executes the conformance suite
  under a real wasm32 single-threaded runtime and is green; deleting the
  execution step or emptying its target fails the gate rather than passing
  quietly.
- **DEPLOY-AC-02 (project AC-004, Architecture brief Notes §5).** The step's
  placement — mandatory in `REQUIRED`, or probe-gated in `OPTIONAL` paired with a
  mandatory anti-vacuity assertion — is a stated decision recorded in ADR-0023,
  not an implicit default. Whichever is chosen, "the step silently does not run"
  is a gate failure on every machine, per AC-004's own wording.
- **DEPLOY-AC-03 (project AC-012).** `happenstance-cloudflare/Cargo.toml` loses
  `publish = false`; `cargo xtask reserve happenstance-cloudflare`-shaped
  reservation exists per `RUNBOOK.md`'s phase-0 rule that a name is claimed when
  its phase starts (`xtask/src/reserve.rs`, `RUNBOOK.md:4254-4261`, `:4301`); no
  real version of the crate is published from this project.
- **DEPLOY-AC-04.** Adding `worker` as a dependency is checked against ADR-0029's
  MSRV floor and `cargo deny`'s licence/advisory surface before it lands, with the
  outcome recorded rather than discovered later by a red `msrv`/`advisories` job.
- **DEPLOY-AC-05.** The existing `.github/workflows/ci.yml` `wasm-conformance` job
  (`:203-234`) is either explicitly retired in favour of the new in-gate step, or
  its continuing purpose is stated — it must not be left as an unexplained
  second, weaker wasm32 execution path once the stronger one exists inside
  `cargo xtask ci`.

### Notes

#### 1. There is no service, so most rollout dimensions are N/A — stated, not skipped

- **Feature flags / config gating.** N/A. `happenstance-cloudflare` has no
  Cargo features today (`Cargo.toml:15-25`) and this project does not add a
  runtime feature flag — the `wasm-bindgen-test` dependency it adds is
  **target-scoped**, not feature-scoped, following the exact reasoning
  `happenstance-testkit`'s own `[target.'cfg(target_arch = "wasm32")'.dev-dependencies]`
  block already uses and that `crates/happenstance-testkit/src/lib.rs:105-113`
  documents as load-bearing (a *feature* is not target-scoped; `--all-features`
  would otherwise try to set it on every target).
- **Migration / backfill.** N/A. `CloudflareEventStore::migrate` (`event_store.rs:84-86`)
  is schema *creation* for a fresh Durable Object, not a backfill of existing
  data — nothing has ever written through this adapter, because it has never
  run. There is no prior schema version to migrate away from.
- **Rollback posture.** N/A in the conventional sense (nothing is deployed to
  roll back), but there is a real analogue: if the `workerd` execution step
  cannot be made to work inside `cargo xtask ci` at acceptable cost, the
  posture is an explicit **blocking finding**, not a silent downgrade to a
  weaker check — this is `_intake-brief.md`'s own named open question
  ("whether a `workerd` runner can be made to exist in CI at acceptable
  cost … a blocking finding, not a degradation to absorb") and `project.md`'s
  first risk entry repeats it verbatim. Merging a version of this project that
  quietly reduces AC-004 to a `cargo check` and calls it done is the one outcome
  this brief must foreclose.
- **KB rollback.** Also not conventional rollback: ADR-0023 and any atom this
  project resolves (CF-40, WF-11) are accepted decision atoms once merged, and
  `redkiln validate --kb` checks them against `HEAD` forever
  (`CLAUDE.md`, **Where the work lives**). Reversing one is a new, superseding
  atom, never a revert of the old one's body.

#### 2. The CI implication is the whole of this brief's real work

`.github/workflows/ci.yml`'s existing `wasm-conformance` job is direct evidence
of the gap this project closes, in its own comment: *"No Cloudflare and no
`workerd`. Those arrive at phase 9; the property under test here is the target's
single-threaded, `!Send` execution model, and `wasm-bindgen-test` on node
exercises that without a platform SDK"* (`:200-203`). Two structural facts about
that job matter for the decision this brief has to make:

1. **It already proves the mechanism can work.** `wasm-bindgen-test` driven under
   node, with `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER: wasm-bindgen-test-runner`
   set as the per-target runner env var and `wasm-bindgen-cli` installed at the
   exact version resolved out of `Cargo.lock` (`:213-234`), is a working,
   demonstrated pattern for running `#[wasm_bindgen_test]`-wrapped tests for
   real. The new step for `happenstance-cloudflare` can reuse this shape rather
   than invent one — the same reuse-over-novelty principle the architecture
   brief applies to the fixture and the harness.
2. **It runs on `ubuntu-latest` only** (`:206`), while the `gate` job — the one
   `cargo xtask ci` actually is — matrices across `ubuntu-latest`,
   `windows-latest` and `macos-latest` (`:34-39`). Folding a Cloudflare-shaped
   execution step into `cargo xtask ci` means answering, and recording in
   ADR-0023, whether `wasm-bindgen-test-runner` (or whatever the real `workerd`
   harness turns out to need — `RUNBOOK.md:4267-4268`'s `vitest-pool-workers`
   hypothesis is a Node.js tool with its own cross-platform story) is available
   and deterministic on all three, or whether the step needs a documented
   platform restriction with a stated reason — never a silent one.

**The decision this brief is for:** does the new step replace `wasm-conformance`,
extend it, or sit beside it with a stated division of labour? A plausible answer,
consistent with DR-3's "same run as the rest of the gate": retire the standalone
job once the new mandatory-or-proof-artefact-guarded step inside `cargo xtask ci`
subsumes what it proved (the testkit's own harness executing for real), and add
the Cloudflare-specific execution as a **new** step rather than a rewrite of an
existing one — because AC-004's own wording ("no step in `xtask/src/main.rs`
currently executes anything under `wasm-bindgen-test-runner` or `workerd`",
Architecture brief §4e) is about `xtask`, and `wasm-conformance` is a GitHub
Actions job, not an `xtask` step at all: folding its logic into `xtask` is itself
new work, not a rewiring. This is a leading candidate, not a settled choice —
ADR-0023 is where it is decided, per the architecture brief's own framing of
ADR-0023 as the authoritative record for exactly these open implementation
choices (Architecture brief §6).

#### 3. Pricing `workerd` against the probe-vs-mandatory choice

Architecture brief Notes §5 already lays out the three honest options: (i)
mandatory step, gate fails without the tool; (ii) probe-gated step plus a
mandatory non-skippable assertion that the target and its rule enumeration exist
(the `proof-artefact` shape, `xtask/src/proof.rs`); (iii) a documented blocking
finding if neither is achievable. This brief adds the cost side of that choice:

- **(i) mandatory** means every contributor and every CI runner needs
  `wasm-bindgen-cli` (or the real `workerd`/`vitest-pool-workers` toolchain) — the
  existing `wasm-conformance` job already pays this cost via
  `taiki-e/install-action@v2` (`:224-230`), so the *CI-side* cost is a known
  quantity; the *local* cost (Testing brief Notes §4: `cargo xtask ci --fast`
  during story-by-story implementation) is new and real for the duration of this
  project.
- **(ii) probe-gated + proof-artefact** keeps local iteration cheap on a machine
  without the tool, at the cost of the extra machinery `xtask/src/proof.rs`
  already models (`Artefact { package, target, tests }`, asserting named tests out
  of `cargo test --list` before running them — `proof.rs:58-72`, `:133-148`).
  This is the same shape `postgres-and-neon-stores` faces for Docker
  (`RUNBOOK.md:4356-4358`, `:4382`, cited in `project.md`'s risk register) — a
  workspace-recurring pattern, not a one-off invention.
- **(iii)** is the fallback if neither prices acceptably, and per the intake
  brief is an escalation, not a thing this project is permitted to absorb
  quietly.

`cargo-hack` and `cargo-deny` both already resolve on the CI runners and
therefore run rather than skip (`CLAUDE.md`, **Commands**) — precedent that this
workspace's CI environment is already provisioned generously enough that adding
one more tool (`wasm-bindgen-cli`, already installed for `wasm-conformance`; or a
`workerd`/`vitest-pool-workers` toolchain, not yet installed anywhere) is a real
but boundable cost, not an open-ended one.

#### 4. Dependency-surface consequences of `worker`

`Cargo.toml:19-25`'s own note names the trade this project reverses: `worker`
"drags a large `wasm-bindgen`/`js-sys`/`web-sys` surface and its own licence
graph through `cargo deny` on every run of the gate." Once it lands:

- **MSRV.** Check `worker`'s declared `rust-version` against the 1.97.1 floor
  (ADR-0029) before merging; if it forces the floor higher, that is a new ADR
  amending ADR-0029, never a silent bump (`CLAUDE.md`, binding constraint 5).
  The `msrv` CI job (`.github/workflows/ci.yml:241-273`) will surface a
  disagreement, but the ADR is owed regardless of whether the job catches it
  first.
- **`cargo deny`.** The optional `cargo deny` gate step and the weekly
  `advisories` job (`.github/workflows/ci.yml:326-...`) both widen their
  surface. This is exactly the cost the `Cargo.toml` note pre-registered — it is
  not a surprise, but it should be checked once, not discovered by a red
  `advisories` run on the Tuesday cron after merge.

#### 5. The release path if a published version changes

**N/A for this project by design.** `project.md`'s **Out of scope** section is
explicit: "Publishing this crate, the registry landing page, the MSRV promise,
the semver diff and the clause-ledger audit → `publication-and-positioning`
(HS-P0016). Which crates actually publish at `0.2.0` is that project's
deployment brief's." This project's deployment surface stops at *publish-ready*
(AC-012: licences, README, `publish = false` removed, gate green) and a
name-reservation placeholder (`xtask/src/reserve.rs`'s documented mechanism: a
standalone `0.0.0` crate sharing only metadata, never the real crate at a real
version — its own module docs explain why publishing the real crate early would
either cascade version pins through every dependent manifest or freeze the API
before the contract-freeze phases the runbook plans for). Any change to what
version of `happenstance-cloudflare` actually ships is `publication-and-positioning`'s
decision to make and to record, not this project's.
