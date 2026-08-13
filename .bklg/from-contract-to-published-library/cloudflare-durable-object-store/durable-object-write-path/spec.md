---
item: HS-S0050
stage: spec
created: 2026-08-12T13:46:48.476Z
updated: 2026-08-12T13:46:48.476Z
template_sig: 87bbf1d0
rendered_sig: 74ac96d4
---

# Spec — Append, head, contains_event_id and migrate against a real Durable Object

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/project.md` |
| This spec | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/durable-object-write-path/spec.md` |
| This story's discover (answers carried forward, not re-litigated) | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/durable-object-write-path/discover.md` |
| Key briefs — architecture (§1 seam, §2 atoms, §3 data flow, §4a mount), testing (§1 tiers, §3 seams) | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_decomposition.md` |
| Story map (this row, its slice, its merge position) | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_storymap.md` |
| Signed-off design | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_design.md` — **no user-facing surface**, approved 2026-08-12. This story renders none. |
| Grounding (atoms + code facts, pre-verified) | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_grounding.md` |
| Roadmap pointer | `RUNBOOK.md` — phase 9, "Cloudflare Durable Object" (`:160` the row, `:4241` the phase heading) |

## One-line PR slice

Implement `migrate`, `append`, `head` and `contains_event_id` against the real bindings — identity
columns (`origin_store`, `origin_position`, `crates/happenstance-cloudflare/src/event_store.rs:187-195`),
ADR-0014 store-id incarnation, DCB conflicts classified into `AppendError::ConditionViolated` *before*
`Self::Error` exists, capacity refusals into `AppendError::ExceedsStoreLimit`, and the 2^53 ceiling
surfaced as `CloudflareEventStoreError::StoredPosition` rather than by truncation.

## Executive summary

Four `todo!()` bodies become four real ones: `CloudflareEventStore::migrate`
(`crates/happenstance-cloudflare/src/event_store.rs:85`), `append` (`:176`), `head` (`:184`) and
`contains_event_id` (`:194`). The *delta* is small in SQL and large in classification. The Durable
Object hands atomicity over for free — one object is one consistency boundary, `exec` is synchronous,
so a `SELECT` for the condition followed by `INSERT … RETURNING position` has nothing awaiting between
them (`crates/happenstance-cloudflare/src/event_store.rs:30-38`,
`crates/happenstance-cloudflare/src/sql_storage.rs:1-12`). What is left, and what this PR is actually
about, is **which channel each failure travels in**: a DCB conflict must become
`AppendError::ConditionViolated` before `Self::Error` is constructed, because the adapter's error type
deliberately has no arm for it; a capacity refusal must become `AppendError::ExceedsStoreLimit` naming a
`StoreLimit`, never `AppendError::Store` and never a truncation; and a position that will not round-trip
through a JS number must arrive as `CloudflareEventStoreError::StoredPosition`, never as a quietly
narrowed integer.

Two structural changes ride with it. The schema gains `origin_store` and `origin_position` — the
intended schema at `event_store.rs:8-28` predates ingest and cannot answer `contains_event_id` at all
without them — and `migrate` becomes where ADR-0014's store-id incarnation is minted and persisted.

Not restated here: the project's objective, the initiative's goals, or why the crate exists. Those live
in `project.md` and `initiative.md` and are unchanged by this story.

## Context pack

The decisions this story must honor, stated as decisions. Everything deeper is a signposted anchor.

**The port shape is not negotiable, and this adapter is why it exists.** Implement the **bare**
`EventStore`, never `SendEventStore`; never `#[async_trait]`, because it injects `+ Send` and makes
`wasm32` impossible; `read` stays a non-`async` method returning the stream at the top level. This story
touches three `async fn`s and one inherent method and must not "tidy" the fourth
(`.kb/decisions/0001-async-port-flavours.md`, `.kb/decisions/0008-one-derivation-for-both-ports.md`,
`CLAUDE.md` constraints 1–4). The two tests in `crates/happenstance-core/src/memory.rs` that pin
`read`'s shape stay green and neither may be deleted.

**The DCB conflict signal is not in the error type, on any adapter.** `happenstance-core` lifts it into
`AppendError::ConditionViolated` (`crates/happenstance-core/src/error.rs:214-217`), and
`CloudflareEventStoreError` has **no** `ConditionViolated` variant — that absence is a finding made
structural, documented at `crates/happenstance-cloudflare/src/event_store.rs:100-104` and
`crates/happenstance-cloudflare/src/lib.rs:75-85`. So the classification happens *early* or it cannot
happen at all: the thrown value's `message` text is read and classified before any
`CloudflareEventStoreError` is constructed. Workers surfaces SQLite's own text (e.g.
`UNIQUE constraint failed: …`) through the thrown `Error`'s `message` and exposes no numeric code
(`crates/happenstance-cloudflare/src/lib.rs:63-73`). Adding a `ConditionViolated` variant would look
like a fix and would in fact be the defect.

**A capacity refusal is a different fact from a store failure, and the caller branches on it.**
`AppendError::ExceedsStoreLimit { limit, len }` exists precisely so a caller can tell "this will never
fit here, park it and tell a human" from "the disk is full, retry"; a store MUST report a capacity
refusal through it rather than through `Store`, and MUST NOT truncate instead
(`crates/happenstance-core/src/error.rs:227-244`; CF-40, `spec/SPECIFICATION.md:7661-7674`). This story
owns the **channel**, not the numbers: `MAX_EVENT_DATA_LEN` / `MAX_TAGS_PER_EVENT` /
`MAX_EVENTS_PER_BATCH` are `measured-store-limits`'. Build the refusal path so that story inherits a
store that can pass in both directions rather than one that can only fail.

**The order of refusals in `append` is a conformance property, not an implementation detail.** An empty
batch is refused **before** the condition is evaluated (ES-20/ES-21; rules `append_rejects_empty_batch`
and `empty_batch_is_refused_before_the_condition_is_evaluated`,
`crates/happenstance-testkit/src/suite.rs:2841`, `:2870`), and a registered mutant already rejects the
other order (`ConditionBeforeEmptinessStore`,
`crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:1397`).

**`contains_event_id` is about an identity, not a position.** ADR-0014 makes it a *required* port
method because the only provided form would need `Self: Sync`, which this deliberately `!Sync` adapter
cannot supply — the atom names this adapter's flavour by construction
(`.kb/decisions/0014-event-identity-and-recorded-time.md:27-29`). The rule flips every byte of the
store's own `StoreId` and asks about a position the store definitely assigned
(`crates/happenstance-testkit/src/suite.rs:2585-2600`), so `SELECT 1 FROM event WHERE position = ?`
answers the easy half correctly and the hard half wrongly. It has a registered mutant
(`PositionOnlyMembershipStore`, `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:3195`)
and a clause (ES-41, `spec/SPECIFICATION.md:4381`, `:4436`). The schema change is what makes the
distinction expressible at all.

**Store-id incarnation is minted once, at `migrate`, and read back.** Mint at first open, persist it in
the object's own storage, read it back on every later open, and re-mint only on a detectable restore or
clone; otherwise a fresh incarnation on every open
(`.kb/decisions/0014-event-identity-and-recorded-time.md:23`; architecture brief Notes §2 and §4a). An
adapter that re-mints per handle answers `contains_event_id` `false` for events it minted five seconds
ago through a second handle onto the same object — and `SECOND_HANDLE` is a MUST for this fixture
(`crates/happenstance-testkit/src/contract.rs:135-161`).

**The 2^53 ceiling is a decode error on read-back, not a `StoreLimit` and not a declined capability.**
Workers SQL widens integers through a JS number, so a `SequencePosition` above
`Number.MAX_SAFE_INTEGER` is not round-trippable even though `NonZeroU64` permits it
(`crates/happenstance-cloudflare/src/lib.rs:108-112`,
`crates/happenstance-cloudflare/src/sql_storage.rs:41-42`). It is reported through
`CloudflareEventStoreError::StoredPosition` (`event_store.rs:135-142`), never by truncation. Settled at
discover and not reopened here: it is not a capacity refused at write time, so it is not a `StoreLimit`;
and `Fixture`'s three ceilings are about payload bytes, tags and batch size, so there is nothing to
decline (`crates/happenstance-testkit/src/contract.rs:214-279`).

**The boundary test may not assert a literal position.** The specification permits gaps and a conformant
adapter may leave them (`CLAUDE.md`, *The rule that matters*). The 2^53 case is therefore constructed —
seed the position column directly through `exec`, a store-side fact this adapter is allowed to arrange —
and asserts on the **variant returned**, never on a number.

**Payloads stay opaque.** `data` and `metadata` are `Bytes` in and `BLOB` out through `SqlValue::Blob`;
this crate adds no `serde` and never inspects a payload (`.kb/decisions/0003-opaque-payloads.md`;
`crates/happenstance-cloudflare/src/sql_storage.rs:36-49`).

**Mount and slice.** This is one of three capability stories in `real-worker-bindings`, implemented in
one context on top of `worker-binding-layer`'s real `SqlStorage`. It mounts at the existing `impl
EventStore for CloudflareEventStore` block, reached through the one constructor
`CloudflareEventStore::new(sql)` (`crates/happenstance-cloudflare/src/event_store.rs:70-87`) — the store
takes its storage by **injection** and must never construct one, because in production the handle comes
off a Durable Object's `State::storage().sql()` and in the fixture off whatever the harness has.

**Who observes this.** There is no screen. The persona is the **adapter author** of the project's
backbone row B — *"store and replay events inside a Durable Object"* — whose observable is
`crates/happenstance-cloudflare/src/event_store.rs` with no `todo!()` in the write path
(`_storymap.md`, **Backbone**). The **gate reader** sees it only later, when
`every-rule-under-workerd` runs the suite over it.

## Integration contract

- **Archetype**: `capability`.
- **Slice / milestone**: `real-worker-bindings`. Slice-mates, implemented in one context and mounted as
  one integrated surface: `worker-binding-layer` (foundation, merges first), `durable-object-read-path`
  (unordered with respect to this story), `caller-visible-error-verdict` (depends on this one).
- **Mount point**: `crates/happenstance-cloudflare/src/event_store.rs` — the existing
  `impl EventStore for CloudflareEventStore` block (`:145-196`), reached through the single construction
  root `CloudflareEventStore::new(sql)` (`:70-87`) and its schema seam `migrate()` (`:79-87`). Not a new
  type, not a parallel path: the four bodies land in the impl that is already wired into the crate's
  public re-exports (`crates/happenstance-cloudflare/src/lib.rs:133`).
- **Wires into**:
  - `crates/happenstance-core/src/error.rs:214-249` — `AppendError`'s four arms, which are the caller-visible
    channels this story chooses between.
  - `crates/happenstance-core/src/limits.rs:54-77` — `StoreLimit` and its `guaranteed_minimum()`.
  - `happenstance_core::{Event, EventId, StoreId, SequencePosition, AppendCondition, Query, ReadOptions,
    SequencedEvent}` as already imported at `crates/happenstance-cloudflare/src/event_store.rs:44-47`.
  - `crates/happenstance-cloudflare/src/sql_storage.rs` — `SqlStorage::exec`, `SqlValue`, `SqlRow`,
    `SqlError`, in the real-bindings form `worker-binding-layer` lands. The four modelled properties
    (`:1-24`) are assumed true of the replacement.
  - `crates/happenstance-testkit/src/suite.rs` — consumed as the observer, not edited.
- **Renders surfaces**: **none.** `_design.md` records no user-facing surface for this project and that
  determination is what was signed off; there are no `## Items` ids to claim.
- **Conformance rule(s) that observe this story** (all already enumerated by
  `for_each_event_store_rule!`, `crates/happenstance-testkit/src/lib.rs:84-89` — this story adds no
  rule): `append_is_atomic` (`suite.rs:2651`), `append_returns_last_written_position` (`:2623`),
  `append_rejects_empty_batch` (`:2841`), `empty_batch_is_refused_before_the_condition_is_evaluated`
  (`:2870`), `condition_rejection_is_reported_as_condition_violated` (`:4949`),
  `condition_rejection_leaves_store_unchanged` (`:4910`), `append_stamps_a_local_event_id` (`:2024`),
  `contains_event_id_reports_membership` (`:2551`), `head_of_an_empty_store_is_none` (`:1731`),
  `head_is_the_highest_visible_position` (`:1798`), `head_advances_across_two_handles` (`:1855`),
  `append_reports_exceeded_store_limits` (`:4282`). They execute only once
  `durable-object-host-and-fixture` and `every-rule-under-workerd` land — which is why this story also
  carries its own targeted tests rather than deferring all evidence to a run it cannot yet trigger.
- **Clause(s) discharged, none amended**: ES-18, ES-19, ES-20, ES-21, ES-24, ES-25–ES-28 (append
  conditions), ES-30 (`head`), ES-41 and VT-7 (`contains_event_id`), CF-40 (the refusal channel; the
  numbers are `measured-store-limits`'). Every `[FROZEN]` clause here is **honoured, not touched**; CF-39,
  CF-40 and ES-17 are `[PROVISIONAL]` and are discharged rather than amended
  (`spec/SPECIFICATION.md:8580-8625`, `:8750-8751`). If the real `SqlStorage` API made the
  `ExceedsStoreLimit` channel unreachable, that is a new decision atom and a re-plan before any code — not
  a clause edit.
- **Advances DoD scenario**: initiative **DoD 4** — *"the constrained-runtime store passes the suite on
  its own target"* (`initiative.md`, **Definition of Done** 4). This story lands the write half of the
  store that DoD 4 is about; it does not by itself turn DoD 4 green, and says so.

## PR boundary

**In this PR**

- The four bodies: `migrate`, `append`, `head`, `contains_event_id`.
- The schema, including `origin_store` / `origin_position` and the store-id incarnation row, plus the
  shared position decoder that `head`, `contains_event_id` and (later) the read path all call.
- The classification of a thrown value into `ConditionViolated` / `ExceedsStoreLimit` / `Store`.
- This crate's own targeted tests for the 2^53 boundary, the refusal channels and the foreign-identity
  half of `contains_event_id`.
- Crate docs updated where they say a body is `todo!()`.
- Mounting is *in* scope by definition: the bodies land in the existing impl reached by the existing
  constructor. Touching the composition/wiring files named in the Integration contract to do that is not
  scope drift.

**Explicitly not in this PR**

- `read`, `render_read`, `decode_row` and `SqlRowStream`'s state machine — `durable-object-read-path`.
- The `worker` dependency, the stand-in swap and the `!Send` probe survival — `worker-binding-layer`
  (merges first; this story assumes it).
- The `Fixture` impl, the Durable Object host and the conformance target — `durable-object-host-and-fixture`
  and `every-rule-under-workerd`.
- The numeric values of the three `Option<usize>` ceilings, and any `MID_BATCH_FAULT` `SUPPORTED` claim —
  `measured-store-limits`.
- The committed caller-visible error reconstruction test — `caller-visible-error-verdict` (which depends
  on this story).
- Any `.kb/` write, ADR-0023, and any open-question resolution — `adr-0023-and-atom-resolutions`, through
  `/redkiln:kb-ingest` only.
- Removing the scoped `#![allow(clippy::todo)]` **unless** this story is the one that removes the last
  `todo!()` in the crate; the story map assigns it to whichever of the write and read paths lands last
  (`_storymap.md`, **Coverage**, AC-001 row).

**Merge DoD one-liner.** `cargo xtask affected --base main` is green, this crate's targeted tests pass,
no `todo!()` remains on the write path, and no `ConditionViolated` variant appeared on
`CloudflareEventStoreError`.

The narrowest honest path set — `redkiln verify --grain story` reads the first fenced block below:

```
crates/happenstance-cloudflare/src/event_store.rs
crates/happenstance-cloudflare/src/lib.rs
crates/happenstance-cloudflare/src/sql_storage.rs
crates/happenstance-cloudflare/Cargo.toml
crates/happenstance-cloudflare/tests/**
.bklg/from-contract-to-published-library/cloudflare-durable-object-store/durable-object-write-path/**
```

`sql_storage.rs` and `Cargo.toml` are in the set only for what this story genuinely needs from the
binding layer — a read-only accessor the classifier calls, or a dev-dependency its targeted tests need.
The module replacement and the `worker` row itself are `worker-binding-layer`'s, and a diff that
rewrites them here has taken a slice-mate's work.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| `migrate()` is idempotent and creates the whole schema | `CREATE TABLE IF NOT EXISTS` for `event`, `event_tag` and the metadata row; safe to call on every open, which is what the fixture will do once per instance. Stays a non-`async` inherent method — `exec` is synchronous, so there is nothing to await | `crates/happenstance-cloudflare/src/event_store.rs:79-87`; `crates/happenstance-cloudflare/src/sql_storage.rs:1-12`; architecture brief §4a |
| The schema gains identity columns | `origin_store` and `origin_position` on `event`, alongside the intended schema's five columns. Without them `contains_event_id` cannot distinguish a foreign identity from a local one at all | `crates/happenstance-cloudflare/src/event_store.rs:8-28`, `:187-195`; architecture brief Notes §2 (ADR-0014) |
| The store id is minted once and read back | Minted at `migrate`/first open, persisted through `exec` in the object's own SQL storage, read back on every later open; re-minted only on a detectable restore or clone. Persisted via SQL rather than the DO's KV so the store keeps **one** injected seam — the constructor takes only `SqlStorage`, and a second handle would be a second thing every fixture and every host must supply | `.kb/decisions/0014-event-identity-and-recorded-time.md:23`; `crates/happenstance-cloudflare/src/event_store.rs:70-87`; `crates/happenstance-testkit/src/contract.rs:135-161` (SECOND_HANDLE is a MUST) |
| `append` order of refusals is fixed | (1) empty batch → `AppendError::NoEvents`; (2) per-event and per-batch capacity checks → `ExceedsStoreLimit`; (3) condition probe → `ConditionViolated`; (4) insert. Emptiness is checked **before** the condition, never after | `crates/happenstance-testkit/src/lib.rs:145`; `crates/happenstance-testkit/src/suite.rs:2841`, `:2870`; mutant `ConditionBeforeEmptinessStore`, `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:1397` |
| `append` is atomic for free, and must stay that way | `SELECT` for the condition, then `INSERT … RETURNING position` for the batch, with **nothing awaited between them**. The object is one consistency boundary and `exec` is synchronous; inserting an `await` between probe and insert would forfeit the only guarantee this runtime gives away | `crates/happenstance-cloudflare/src/event_store.rs:30-38`; architecture brief Notes §3; rule `append_is_atomic` (`suite.rs:2651`) |
| `append` returns the **last written** position | The position of the final event in the batch, not the store's head, and the batch's positions follow slice order | ES-19 (`spec/SPECIFICATION.md:8601`); rule `append_returns_last_written_position` (`suite.rs:2623`) |
| A DCB conflict is classified before `Self::Error` exists | The thrown value's `message` text is inspected and mapped to `AppendError::ConditionViolated`; there is no numeric code to switch on and no `ConditionViolated` arm on the adapter's error to fall back to. Adding one is the defect, not the fix | `crates/happenstance-cloudflare/src/event_store.rs:100-104`; `crates/happenstance-cloudflare/src/lib.rs:63-73`, `:75-85`; ARCH-AC-07 in `_decomposition.md` |
| A capacity refusal names a `StoreLimit` | `AppendError::ExceedsStoreLimit { limit, len }` with the matching `StoreLimit` variant and the offending count — never `AppendError::Store`, never a silent truncation, and never a clamp where a chunk was meant (`&events[..CEILING]` vs `events.chunks(CEILING)`) | `crates/happenstance-core/src/error.rs:227-244`; CF-40 (`spec/SPECIFICATION.md:7661-7674`); mutants `PayloadCeilingStore` (`mutants.rs:2433`), `TruncatingPayloadStore` (`:2486`), `BatchParameterCeilingStore` (`:2627`), `ChunkLosingBatchStore` (`:2683`) |
| The refusal channel exists before the numbers do | The three ceilings stay `None` here, so `append_reports_exceeded_store_limits` still skips through `NO_STORE_LIMITS` — the path is proven by this story's own targeted tests, so `measured-store-limits` inherits a store that can pass in both directions rather than one that can only fail | `crates/happenstance-testkit/src/contract.rs:214-279`, `:442`; `crates/happenstance-testkit/tests/mutation_coverage.rs:3157` (the `MUST_SKIP` list and why that rule is on it) |
| `head` reads the highest visible position | `SELECT max(position) FROM event`, decoded through the shared position decoder; `None` on an empty store. No `await` on the way, so the storage handle is never held across a suspension point and the `Sync` bound the provided body would want is never needed | `crates/happenstance-cloudflare/src/event_store.rs:179-185`; ES-30; rules `head_of_an_empty_store_is_none` (`suite.rs:1731`), `head_is_the_highest_visible_position` (`:1798`), `head_advances_across_two_handles` (`:1855`) |
| `contains_event_id` answers about the identity | `SELECT 1 FROM event WHERE origin_store = ? AND origin_position = ? LIMIT 1`. A foreign `StoreId` at a position this store did assign must read `false`; a `WHERE position = ?` implementation is the named wrong one | `crates/happenstance-cloudflare/src/event_store.rs:187-195`; `crates/happenstance-testkit/src/suite.rs:2585-2600`; ES-41 (`spec/SPECIFICATION.md:4381`, `:4436`); mutant `PositionOnlyMembershipStore` (`mutants.rs:3195`) |
| One shared position decoder, landed here | `head`, `contains_event_id` and (later) the read path's `decode_row` all decode a stored integer into a `SequencePosition`. This story lands that one function, including the `> 2^53` and `< 1` rejections; `durable-object-read-path` consumes it rather than writing a second. Stated so neither story writes it twice nor assumes the other did | `crates/happenstance-cloudflare/src/event_store.rs:135-142`, `:316-319`; `crates/happenstance-cloudflare/src/sql_storage.rs:40-42` |
| A non-round-trippable position surfaces as `StoredPosition` | Anything below 1 or above 2^53 comes back as `CloudflareEventStoreError::StoredPosition { raw }`, never narrowed, never truncated, never a panic. Tested by seeding the position column through `exec` and asserting on the **variant**, never on a literal position value | `crates/happenstance-cloudflare/src/lib.rs:108-112`; `crates/happenstance-cloudflare/src/event_store.rs:135-142`; `CLAUDE.md`, *The rule that matters* |
| Payloads pass through opaque | `data`/`metadata` as `SqlValue::Blob`; `metadata: None` and `Some(<empty>)` stay two distinct values; no `serde` enters this crate's dependency surface | `.kb/decisions/0003-opaque-payloads.md`; `crates/happenstance-cloudflare/src/sql_storage.rs:36-49` |
| The port shape is unchanged | Bare `EventStore` only; no `#[async_trait]`; `read` still non-`async` with the stream at the top level; `CloudflareEventStoreError` gains no variant beyond what already exists and stays genuinely `!Send`; `#[non_exhaustive]` and every `pub` item's visibility unchanged; every fallible public function keeps an `# Errors` section naming conditions rather than types | `.kb/decisions/0001-async-port-flavours.md`; `.kb/decisions/0008-one-derivation-for-both-ports.md`; `crates/happenstance-cloudflare/src/lib.rs:137-259` (the four probe tests); `standards/rust/70-rustdoc-obligations.md` |
| The `MID_BATCH_FAULT` seam is affordable but unclaimed | The write path leaves the schema able to host a one-shot armed fault (a `CHECK` constraint or trigger armed for exactly one write) without a later schema change. This story declares **nothing**: the `Capability` constant is `durable-object-host-and-fixture`'s and the decision to claim `SUPPORTED` is `measured-store-limits`'. Recorded here because discover deferred it explicitly, to settle jointly at spec | `crates/happenstance-testkit/src/contract.rs:207-211`, `:281-307`; CF-39 (`spec/SPECIFICATION.md:7631`); `discover.md`, **Questions** |

## Data and migrations

**In scope, and it is the one place this story changes durable shape.**

The schema this crate documents (`crates/happenstance-cloudflare/src/event_store.rs:8-28`) predates
ADR-0014's ingest work and is incomplete. It gains, in `migrate`:

- **`event.origin_store`** and **`event.origin_position`** — the two columns `contains_event_id` reads.
  Already named in the method's own comment (`:187-195`); the intended-schema block must be updated in
  the same change so the documentation and the DDL do not disagree.
- **A metadata row for the store's incarnation** — one `StoreId`, written on first `migrate` and read
  back on every subsequent open. Through `exec`, in the object's own SQL storage, because the store's
  only injected seam is `SqlStorage` (`:70-87`) and reaching a Durable Object's KV would add a second
  handle every fixture and every host would then have to supply.

**No migration framework, and no version column.** `migrate` is `CREATE TABLE IF NOT EXISTS` DDL, called
once per instance by the fixture and once at startup by a host. There is no schema-version table and none
is proposed: there is no deployed data anywhere to migrate, and the crate is still `publish = false`
(`crates/happenstance-cloudflare/Cargo.toml`). Introducing a versioning scheme would be inventing a
migration story for a store with no users — a decision for a later phase, taken with an ADR, not a side
effect of this one.

**`happenstance-sqlite` is a shape to follow, never a thing to depend on.** Its schema
(`crates/happenstance-sqlite/src/event_store.rs`) is an unfinished skeleton owned by
`sqlite-durable-store` and does not carry the identity columns either. No adapter may depend on another
adapter (`CLAUDE.md`, *Dependency rule*): copy the shape, do not wait for the sibling and do not reach
into it.

**Backwards compatibility: none owed.** Every `todo!()` body means no store has ever written a row with
this crate, so there is no existing on-disk state and nothing to read forward from.

## Acceptance criteria

Ten criteria, each stated as a goal someone is trying to reach through the whole stack — the
**constrained-runtime developer** (*Event-source at the edge without hand-rolling it*), the **adapter
author** (*Learn when you are finished*) and the **application author** (*Choose a contract before a
database*), carried from
`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` via
`initiative.md`, **Referenced personas & journeys**.

Two things about the verification column. The **targeted tests** live in this crate's own tree and are
this story's evidence; they are named as `event_store::write_path_tests::<name>` and are what the ledger
cites. The **conformance rules** named beside them are the same facts observed later by the suite; they
execute only once `durable-object-host-and-fixture` and `every-rule-under-workerd` land, so they are
listed as the downstream observer, never as this story's proof. No test in either column asserts a
literal position value (`CLAUDE.md`, *The rule that matters*).

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | GIVEN a constrained-runtime developer who wants an event store inside a Durable Object without hand-rolling one, WHEN they construct `CloudflareEventStore::new(state.storage().sql())`, call `migrate()` and then `append(&events, None)`, THEN real SQL executes and a `SequencePosition` comes back — no `todo!()` remains in `migrate`, `append`, `head` or `contains_event_id`, and the port shape is untouched (bare `EventStore`, no `#[async_trait]`, `read` still non-`async` with the stream at the top level). | `event_store::write_path_tests::append_then_head_round_trips`; `rg -n "todo!" crates/happenstance-cloudflare/src/event_store.rs` returns only the two read-path sites (`:313`, `:318`); `cargo clippy --workspace --all-targets --all-features -- -D warnings`; the four probe tests at `crates/happenstance-cloudflare/src/lib.rs:137-259` still compile and pass |
| AC-002 | GIVEN an adapter author whose Durable Object is evicted and re-created between requests, WHEN `migrate()` runs on every open, THEN the first call creates `event` (carrying `origin_store` and `origin_position`), `event_tag` and the incarnation row, every later call is a no-op, and the intended-schema comment at `crates/happenstance-cloudflare/src/event_store.rs:8-28` names exactly the columns the DDL creates. | `event_store::write_path_tests::migrate_is_idempotent`; `event_store::write_path_tests::schema_carries_the_identity_columns` (asserts the two columns are writable and readable, not that a comment exists) |
| AC-003 | GIVEN a constrained-runtime developer holding two handles onto one Durable Object, WHEN the second handle opens the store and asks `contains_event_id` about an id the first minted seconds earlier, THEN the answer is `true` — the incarnation `StoreId` is minted once at first `migrate`, persisted through `exec`, and read back on every later open rather than re-minted per handle. | `event_store::write_path_tests::store_id_is_read_back_on_a_second_handle`; `event_store::write_path_tests::store_id_is_not_reminted_per_handle`; downstream: `SECOND_HANDLE` is a MUST for this fixture (`crates/happenstance-testkit/src/contract.rs:135-161`) and `head_advances_across_two_handles` (`crates/happenstance-testkit/src/suite.rs:1855`) |
| AC-004 | GIVEN an application author whose retry loop branches on `AppendError::is_condition_violated`, WHEN they accidentally call `append(&[], Some(&condition))` against a store the condition matches, THEN they receive `AppendError::NoEvents` and never `ConditionViolated` — because an empty batch is refused before the condition is evaluated, so the loop terminates instead of retrying a batch that will still be empty next time. | `event_store::write_path_tests::an_empty_batch_is_refused_before_the_condition`; downstream: `append_rejects_empty_batch` (`suite.rs:2841`), `empty_batch_is_refused_before_the_condition_is_evaluated` (`:2870`); the wrong order is already a registered mutant, `ConditionBeforeEmptinessStore` (`crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:1397`) |
| AC-005 | GIVEN two writers racing to append against the same DCB condition on one object, WHEN one loses, THEN the loser receives `AppendError::ConditionViolated` — classified from the thrown value's `message` text *before* any `CloudflareEventStoreError` is constructed — the store is left byte-for-byte as it was, and `CloudflareEventStoreError` still carries no `ConditionViolated` variant. | `event_store::write_path_tests::a_unique_constraint_message_classifies_as_condition_violated`; `event_store::write_path_tests::a_rejected_condition_writes_nothing`; `event_store::write_path_tests::the_error_type_has_no_condition_violated_variant` (an exhaustive `match` over the variants, so adding one fails to compile); downstream: `condition_rejection_is_reported_as_condition_violated` (`suite.rs:4949`), `condition_rejection_leaves_store_unchanged` (`:4910`) |
| AC-006 | GIVEN a sync runner that must tell "this will never fit here, park it and tell a human" from "the disk is full, retry", WHEN a batch crosses a capacity ceiling this store declares, THEN it is refused as `AppendError::ExceedsStoreLimit { limit, len }` naming the matching `StoreLimit` and the offending count — never `AppendError::Store`, never truncated, never clamped where a chunk was meant — and a batch at exactly the ceiling is accepted. | `event_store::write_path_tests::a_capacity_refusal_names_a_store_limit`; `event_store::write_path_tests::exactly_at_the_ceiling_is_accepted` (both driven through a test-visible ceiling, because the three real numbers are `measured-store-limits`'); downstream: `append_reports_exceeded_store_limits` (`suite.rs:4282`) once those numbers exist, with mutants `PayloadCeilingStore` (`mutants.rs:2433`), `TruncatingPayloadStore` (`:2486`), `BatchParameterCeilingStore` (`:2627`), `ChunkLosingBatchStore` (`:2683`) |
| AC-007 | GIVEN a developer appending a five-event batch that models one decision, WHEN `append` returns, THEN it returns the position of the **last** event in slice order, positions follow slice order, and either every event in the batch is visible or none is — the condition probe and the `INSERT … RETURNING position` have nothing awaited between them. | `event_store::write_path_tests::append_returns_the_last_written_position`; `event_store::write_path_tests::a_failed_batch_leaves_no_partial_rows`; downstream: `append_is_atomic` (`suite.rs:2651`), `append_returns_last_written_position` (`:2623`), `append_stamps_a_local_event_id` (`:2024`) |
| AC-008 | GIVEN an application author resuming a projection after the object was evicted, WHEN they call `head()` on an empty store and again after appending, THEN they get `None` and then the highest visible position, decoded through the one shared position decoder, with no `await` taken while the storage handle is held. | `event_store::write_path_tests::head_of_an_empty_store_is_none`; `event_store::write_path_tests::head_is_the_highest_position_this_store_assigned` (compares against positions `append` actually returned, never a literal); downstream: `head_of_an_empty_store_is_none` (`suite.rs:1731`), `head_is_the_highest_visible_position` (`:1798`) |
| AC-009 | GIVEN a replication ingest asking whether an event it just received is already stored here, WHEN the `EventId` carries a **foreign** `StoreId` at a position this store *did* assign, THEN `contains_event_id` answers `false`, and answers `true` only when both halves match — the query is over `origin_store` **and** `origin_position`, never over `position` alone. | `event_store::write_path_tests::a_foreign_origin_is_not_a_member`; `event_store::write_path_tests::a_local_identity_is_a_member`; downstream: `contains_event_id_reports_membership` (`suite.rs:2551`, its foreign half at `:2585-2600`), mutant `PositionOnlyMembershipStore` (`mutants.rs:3195`) |
| AC-010 | GIVEN a constrained-runtime developer on a store whose position column holds a value Workers SQL cannot round-trip through a JS number, WHEN `head` or `contains_event_id` decodes that row, THEN the caller receives `CloudflareEventStoreError::StoredPosition { raw }` — never a narrowed or truncated position, never a panic — so a ceiling that is real on this runtime is reported rather than silently passed. | `event_store::write_path_tests::a_position_above_two_pow_53_is_reported_not_narrowed`; `event_store::write_path_tests::a_position_below_one_is_reported`; both seed the column directly through `exec` and assert on the returned **variant**, never on a number (`CLAUDE.md`, *The rule that matters*) |

**Traceability.** Project AC-001 (*the adapter is real*) is discharged in the write half by AC-001 through
AC-009; project AC-007(b) (*the 2^53 ceiling reported through `StoredPosition`*) is discharged by AC-010
(`.bklg/from-contract-to-published-library/cloudflare-durable-object-store/project.md`, **Acceptance
criteria**). The read half of project AC-001 and AC-007(a) are `durable-object-read-path`'s and are not
claimed here.

## Interaction quality

**Composition family: N/A, and that is a signed-off determination rather than an omission.**
`.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_design.md` records **no
user-facing surface** for this project — no `## Items`, no signatures, no density budget, no named
visual anti-patterns — and it was approved in that form on 2026-08-12 with the perceptual review a
*declared* skip. This story renders nothing, so there is no presentation, placement, transience,
density or hierarchy invariant to carry, and no AC row claims one. Inventing a surface here would
contradict a signed-off design.

**State family: applies, translated to the medium this story actually has.** The "state" a caller of
this port observes is the store's durable state and the error channel a failure arrives in; every
invariant below that has meaning here is already carried as an `AC-###` **row in the table above**, which
is where `redkiln verify` reads them from.

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Reversibility** — a refused operation leaves the caller exactly where they were, with nothing half-done | AC-005 (a rejected condition writes nothing), AC-007 (all-or-none for the batch) | `a_rejected_condition_writes_nothing`, `a_failed_batch_leaves_no_partial_rows`; downstream `condition_rejection_leaves_store_unchanged`, `append_is_atomic` |
| **Non-occlusion** — the real fault stays visible instead of being covered by a generic one | AC-006 (a capacity refusal is not flattened into `Store`), AC-010 (a non-round-trippable position is not flattened into a narrowed integer) | `a_capacity_refusal_names_a_store_limit`, `a_position_above_two_pow_53_is_reported_not_narrowed`; the underlying `SqlError` stays reachable through `#[error(transparent)]` (`crates/happenstance-cloudflare/src/event_store.rs:109-111`) |
| **Preserved identity across a context change** — the analogue of preserved focus/selection: who this store *is* survives eviction, re-open and a second handle | AC-003 | `store_id_is_read_back_on_a_second_handle`, `store_id_is_not_reminted_per_handle` |
| **In-place, not a context jump** — the capability appears where the caller already is, not through a parallel API | AC-001, AC-002 | the bodies land in the existing `impl EventStore for CloudflareEventStore` (`:145-196`) reached through the one constructor (`:70-87`); no new public type, no second construction path — asserted by the tests going through `CloudflareEventStore::new` and nothing else |
| **Reachability** — the keyboard-reachability analogue: every capability is reachable through the public port, with nothing behind a private or test-only door | AC-001, AC-006, AC-009 | the targeted tests call only `EventStore` methods and `migrate`; a test-visible ceiling for AC-006 may be `pub(crate)`, but the *refusal* it drives must be observable through `append`'s public return type |
| **Ordering the caller can rely on** — the analogue of predictable focus order: refusals arrive in a fixed sequence, so the same call never means two things | AC-004 | `an_empty_batch_is_refused_before_the_condition`; downstream `empty_batch_is_refused_before_the_condition_is_evaluated` |

Not applicable, stated rather than skipped: keyboard reachability proper, scroll/selection restoration,
modal occlusion, and every composition invariant — there is no rendered control anywhere in this diff.

## Error conditions

| id | Condition | Required behaviour | Evidence path |
| --- | --- | --- | --- |
| EC-001 | `append` called with an empty slice | `AppendError::NoEvents`, decided before the condition is read from | `crates/happenstance-core/src/error.rs:219-225`; AC-004 |
| EC-002 | The DCB condition matches — another writer got there first | `AppendError::ConditionViolated(_)`, classified from the thrown `message` before `Self::Error` is constructed; the adapter's error type gains no variant | `crates/happenstance-core/src/error.rs:214-217`; `crates/happenstance-cloudflare/src/lib.rs:75-85`; AC-005 |
| EC-003 | A batch crosses a capacity ceiling this store declares (payload bytes, tags per event, events per batch) | `AppendError::ExceedsStoreLimit { limit, len }` naming the matching `StoreLimit`; the check runs before the insert, so no side effect precedes the refusal | `crates/happenstance-core/src/error.rs:227-244`; `crates/happenstance-core/src/limits.rs:45-77`; CF-40 (`spec/SPECIFICATION.md:7661-7674`); AC-006 |
| EC-004 | The object's **total** SQL storage is exhausted with no declared ceiling crossed (`SqlError::StorageLimitExceeded`) | A genuine store failure: `AppendError::Store(CloudflareEventStoreError::Sql(..))`. This is the "disk is full, retry" case the `ExceedsStoreLimit`/`Store` split exists to keep separate — the defect discover names is routing a *declared ceiling* through this arm, not routing object-wide exhaustion through it | `crates/happenstance-cloudflare/src/sql_storage.rs:119-121`; `discover.md`, **The wrong implementation**; AC-006 draws the line |
| EC-005 | A stored position is below 1 or above 2^53 on read-back | `CloudflareEventStoreError::StoredPosition { raw }`; never narrowed, never truncated, never a panic | `crates/happenstance-cloudflare/src/event_store.rs:135-142`; `crates/happenstance-cloudflare/src/lib.rs:108-112`; AC-010 |
| EC-006 | Two futures created from one handle are polled alternately and the SQL state is already borrowed | `SqlError::AlreadyBorrowed` surfaced through `Store`, never a `borrow_mut` panic — a Durable Object is single-threaded but re-entrant | `crates/happenstance-cloudflare/src/sql_storage.rs:102-109` |
| EC-007 | A row comes back with the wrong column count or an undecodable `SqlValue` | `CloudflareEventStoreError::RowShape` / `::ColumnType` naming the column — the schema on disk is not the schema this build expects | `crates/happenstance-cloudflare/src/event_store.rs:113-129` |
| EC-008 | `migrate` fails, or the incarnation row is absent when it should exist | `CloudflareEventStoreError::Sql(..)` out of `migrate`; a store whose incarnation cannot be established fails loudly rather than minting a fresh one and answering `contains_event_id` wrongly for its own past | `crates/happenstance-cloudflare/src/event_store.rs:79-87`; `.kb/decisions/0014-event-identity-and-recorded-time.md:23`; AC-003 |

## Non-functional

| id | Requirement | Why it binds here | Check |
| --- | --- | --- | --- |
| NF-001 | The port shape is unchanged: bare `EventStore` only, no `#[async_trait]`, `read` still non-`async` with the stream at the top level, and `CloudflareEventStoreError` stays genuinely `!Send`/`!Sync` | This adapter is the workspace's only `!Send` store and the standing guard on ADR-0001; a `+ Send` reintroduced here makes `wasm32` impossible | `.kb/decisions/0001-async-port-flavours.md`; `.kb/decisions/0008-one-derivation-for-both-ports.md`; the four probe tests (`crates/happenstance-cloudflare/src/lib.rs:137-259`) and `send_shape::send_flavour` still compiling |
| NF-002 | No new dependency surface: no `serde` in this crate, and no crate added beyond what `worker-binding-layer` already priced against ADR-0029's MSRV floor and `cargo deny` | Payloads are opaque `Bytes` and this crate never inspects one; the `worker` row and its licence graph belong to the slice-mate that merges first | `.kb/decisions/0003-opaque-payloads.md`; `crates/happenstance-cloudflare/Cargo.toml`; `cargo deny` step of `cargo xtask ci` |
| NF-003 | Every fallible public function keeps an `# Errors` section naming *conditions*, not types; crate docs stop describing bodies as `todo!()` where they no longer are | The rustdoc obligations are a gate step, and stale docs on a now-real adapter mislead the first reader the crate has | `standards/rust/70-rustdoc-obligations.md`; `cargo doc` step of `cargo xtask ci` |
| NF-004 | The crate still builds for `wasm32-unknown-unknown` and on the host, and clears MSRV 1.97.1 | The target is the only reason this adapter exists; ADR-0029 raised the floor and CI's `msrv` job is what a consumer sees | `cargo xtask wasm`; `.kb/decisions/0029-msrv-raised-to-1-97-1.md` |
| NF-005 | No `unsafe`, no `unwrap`/`expect`/panic on any write path, and no `todo!()` left in the four bodies | `unsafe_code = "forbid"` is workspace policy, and a panic inside a Durable Object takes the object down rather than returning an error a caller can branch on | `cargo clippy … -D warnings`; `crates/happenstance-cloudflare/src/lib.rs:121-126` (the scoped allow, removed only if this story lands the last `todo!()`) |
| NF-006 | One `append` is one round of statements against `exec`, not one statement per event where a batch would do — but no benchmark is owed and no number is promised | Cost matters on a metered runtime; measuring it is `measured-store-limits`' and `wf-11-memory-ceiling-falsifier`'s, so this story states the shape and refuses to invent a target | `_decomposition.md`, Architecture brief Notes §6 (implementer's latitude) |

## Implementation notes (non-prescriptive)

Shape, not instructions — the architecture brief's Notes §6 explicitly reserves this latitude
(`.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_decomposition.md`).

- **One classifier, one place.** The mapping from a thrown value to `ConditionViolated` /
  `ExceedsStoreLimit` / `Store` wants to be a single private function taking the thrown error and
  returning `AppendError<CloudflareEventStoreError>`, called from exactly one place in `append`. Two
  call sites is how the two arms drift apart, and the message text is the only signal available
  (`crates/happenstance-cloudflare/src/lib.rs:63-73`) so it should be matched in one spot that a future
  Workers text change can be re-pointed at.
- **Prefer refusing before the insert to classifying after it.** Where a ceiling is *known* — once
  `measured-store-limits` supplies the numbers — the capacity check is a pure function of the batch and
  should run before any SQL. Classifying a thrown storage error after the fact is the fallback, not the
  design, because it cannot distinguish which ceiling was crossed and CF-40 requires naming one.
- **The ceiling values want a seam, not a literal.** Land the refusal path reading its ceilings from one
  private place (a `const` block, an associated const, a small struct — the implementer's call) so that
  `measured-store-limits` changes numbers and nothing else. A test-visible ceiling is what lets AC-006
  prove both directions today.
- **Store-id persistence goes through `exec` like everything else.** A one-row metadata table keeps the
  store's only injected seam `SqlStorage` (`:70-87`). Reaching a Durable Object's KV would add a second
  handle every fixture and every host then has to supply.
- **The position decoder is one function and it lands here.** `head` and `contains_event_id` need it now,
  `decode_row` needs it in `durable-object-read-path`; write it once, including the `< 1` and `> 2^53`
  rejections, and let the read path consume it.
- **Where the targeted tests live.** In this crate's own tree, mirroring `not_send_probe`'s existing
  `#[cfg(all(test, not(target_arch = "wasm32")))]` module (`crates/happenstance-cloudflare/src/lib.rs:154-163`).
  The testing brief flags as open whether host linking survives the `worker` swap
  (`_decomposition.md`, Testing brief Notes §2); if it does not, the module moves behind
  `#[cfg(all(test, target_arch = "wasm32"))]` and re-emits through `wasm_bindgen_test`. **The test names
  do not change either way** — the ledger cites names, not a target — and the four existing probe
  assertions must stay reachable from an ordinary `cargo test`.
- **`MID_BATCH_FAULT`: leave the seam affordable, declare nothing.** A `CHECK` constraint or trigger armed
  for exactly one write must remain addable without a later schema change. The `Capability` constant is
  `durable-object-host-and-fixture`'s and the `SUPPORTED` claim is `measured-store-limits`'
  (`crates/happenstance-testkit/src/contract.rs:207-211`, `:281-307`).
- **The scoped `#![allow(clippy::todo)]` is not automatically yours.** It leaves with the last `todo!()`
  in the crate. If `durable-object-read-path` has already merged, remove it here; if not, leave it and say
  so in the PR body (`_storymap.md`, **Coverage**, AC-001 row).

## Tests and CI (merge gate)

Grounded in the testing brief's tiers (`_decomposition.md`, Testing brief Notes §1) and its merge-gate
section (§4).

| Tier | Command / path | Proves |
| --- | --- | --- |
| Static | `cargo fmt --all --check`; `cargo clippy --workspace --all-targets --all-features -- -D warnings` | AC-001 and NF-005: no surviving `todo!()` in the four bodies, no `unwrap`/`panic` the lints catch, and the scoped allow either gone or still justified |
| Static | `cargo xtask spec-trace` | The clauses this story discharges still resolve — ES-19, ES-20/21, ES-30, ES-41, CF-40 (`spec/SPECIFICATION.md`) — and no marker rotted into decoration |
| Static | `cargo xtask wasm` | NF-004: the crate still builds for `wasm32-unknown-unknown`, which is the only target it exists for |
| Unit (host today; see Notes §2) | `cargo test -p happenstance-cloudflare` → `crates/happenstance-cloudflare/src/event_store.rs`, `mod write_path_tests` | AC-001 … AC-010, one named test per row. This is the story's own evidence and the ledger's `verifying_test` column |
| Unit (host, pre-existing) | `cargo test -p happenstance-cloudflare` → `not_send_probe` (`crates/happenstance-cloudflare/src/lib.rs:137-259`) | NF-001: the error type is still `!Send`, the probe is still non-vacuous, and this diff did not restore `Send`-ness by accident |
| Compile-time proof (no runtime) | the crate compiling at all → `send_shape::send_flavour::SendStoreWithLocalError` (`crates/happenstance-cloudflare/src/lib.rs:87-94`) | NF-001: a `Send` flavour still does not imply a `Send` error, so ES-6's shape is unchanged |
| Mutation meta-suite (not this crate's, must stay green) | `cargo test -p happenstance-testkit --test mutation_coverage` | The six mutants this story's behaviour is defined against are still rejected by their rules (`crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:1397`, `:2433`, `:2486`, `:2627`, `:2683`, `:3195`) |
| Story gate | `cargo xtask affected --base main` | The narrow bar every `redkiln advance` seam runs: only what this diff could break |
| Project gate | `cargo xtask ci --fast` | `REQUIRED` only — the bar this **non-terminal** project is held to (`xtask/src/main.rs`, `run_fast`) |
| Integration — **deferred, named, not this story's** | `event_store_conformance!` against `CloudflareFixture` under `workerd` | The twelve rules listed in the Integration contract observe these same facts. They execute once `durable-object-host-and-fixture` and `every-rule-under-workerd` land; this story does not claim their green |

**Merge DoD.** `cargo xtask affected --base main` green, every `write_path_tests::*` passing, `cargo xtask
ci --fast` green, and the ledger's ten rows flipped with cited evidence.

## Risks and coupling (PR-scoped)

| Risk | Likelihood / impact | Mitigation inside this PR |
| --- | --- | --- |
| `worker-binding-layer` has not merged, so there is no real `SqlStorage` to write against | Medium / High — it is the declared dependency | Do not start against the stand-in and "swap later": the classification this story is *about* depends on a real thrown value. If the foundation is not merged, the story is blocked, and saying so is the correct outcome |
| Host linking breaks once `worker` is a real dependency, taking the targeted tests with it | Medium / Medium — flagged as open by the testing brief, not settled | The tests are written so the `cfg` gate is the only thing that changes: same module, same names, `wasm_bindgen_test` re-emission if needed (`_decomposition.md`, Testing brief Notes §2) |
| The refusal channel is built but never exercised, because the fixture's three ceilings are still `None` and `append_reports_exceeded_store_limits` skips through `NO_STORE_LIMITS` | High / High — this is discover's named wrong implementation and it passes every existing check *structurally* | AC-006 is proven by this story's own tests in **both** directions against a test-visible ceiling, so `measured-store-limits` inherits a store that can pass rather than one that can only fail (`crates/happenstance-testkit/src/contract.rs:442`; `crates/happenstance-testkit/tests/mutation_coverage.rs:3148-3164`) |
| Someone "fixes" the classification by adding a `ConditionViolated` variant to `CloudflareEventStoreError` | Medium / High — it looks like the obvious fix and is the defect | AC-005 carries an exhaustive-`match` test so the variant cannot be added without a compile failure, and the absence is documented at `event_store.rs:100-104` and `lib.rs:75-85` |
| Message-text classification is brittle: Workers changes the SQLite error string and every conflict silently becomes a transport fault | Low / High | One classifier in one place (Implementation notes), tested against the exact string the crate documentation already names (`lib.rs:63-73`); `caller-visible-error-verdict` then commits the caller-visible reconstruction on top of it |
| The schema lands without `origin_store`/`origin_position`, or lands them without updating the intended-schema comment | Medium / Medium | AC-002 and AC-009 are separate rows: one proves the columns are real, the other proves the query uses both halves |
| This story and `durable-object-read-path` both write a position decoder, or neither does | Medium / Medium — they touch the same file in the same slice | Stated as this story's deliverable in the Behavior table and again in Implementation notes; the read path consumes it |
| Scope drift into the slice-mates' diffs (`sql_storage.rs` replacement, the `worker` Cargo row, the `Fixture`) | Medium / Medium | The PR boundary's path set is deliberately narrow and says what each entry is *for*; a diff that rewrites `sql_storage.rs` has taken `worker-binding-layer`'s work |
| A `[FROZEN]` clause looks like it needs amending to make the code work | Low / High | It does not get amended. That is a new decision atom and a re-plan before any code (`CLAUDE.md`, **Open questions**) |

## Dependencies

**Blocks on**

- `worker-binding-layer` — supplies the real Durable Object `SqlStorage` bindings, the synchronous `exec`,
  and the `Rc`-shaped thrown error this story classifies. Without it there is nothing real to classify.
  It merges first within `real-worker-bindings` (`_storymap.md`, **Merge order** §2).

**Unlocks**

- `caller-visible-error-verdict` — depends on this story directly; it commits the ES-6 reconstruction test
  on top of the classification landed here.
- `durable-object-host-and-fixture` — depends on this story and on `durable-object-read-path`; a fixture
  needs a store with real bodies to connect to.
- `measured-store-limits` — inherits the `ExceedsStoreLimit` channel and supplies the three numbers.
- `every-rule-under-workerd` — where the twelve rules named in the Integration contract actually execute.

**Unordered with respect to**

- `durable-object-read-path` — same slice, same file, same bindings, but a different failure mode; both
  depend only on `worker-binding-layer`.

## Anchors (progressive disclosure)

Open these when the bound criterion is the one being implemented — not before, and not all at once.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `crates/happenstance-cloudflare/src/event_store.rs` | The mount point itself: the four `todo!()` bodies (`:85`, `:176`, `:184`, `:194`), the intended-schema comment (`:8-28`), the constructor (`:70-87`), the error type (`:89-143`) and the `impl EventStore` block (`:145-196`) | First, before writing any code | AC-001, AC-002 |
| `crates/happenstance-cloudflare/src/lib.rs` | The crate documentation's four findings — why the conflict signal never travels in `Self::Error` (`:75-85`), what the thrown `message` carries (`:63-73`), and the 2^53 ceiling (`:108-112`) — plus the `!Send` probes (`:137-259`) this diff must not weaken | Before writing the classifier, and again before touching anything near the error type | AC-005, AC-010 |
| `crates/happenstance-core/src/error.rs` | `AppendError`'s four arms with the caller's reasoning written on them: why `ExceedsStoreLimit` is distinct from `Store` (`:227-244`) and why `NoEvents` is a caller bug (`:219-225`) | Before deciding which arm any failure travels in | AC-004, AC-005, AC-006 |
| `crates/happenstance-core/src/limits.rs` | `StoreLimit`'s three variants and `guaranteed_minimum()` — the vocabulary a capacity refusal must name | While implementing the refusal path | AC-006 |
| `crates/happenstance-testkit/src/suite.rs` | The rules that will later observe this code, with their reasoning in the doc comments — especially `empty_batch_is_refused_before_the_condition_is_evaluated` (`:2870`) and `contains_event_id_reports_membership` (`:2551`, foreign half at `:2585-2600`) | When writing the targeted test for the matching AC, to make it assert the same fact the rule will | AC-004, AC-007, AC-008, AC-009 |
| `crates/happenstance-testkit/src/contract.rs` | `Capability`, `SECOND_HANDLE` as a MUST (`:135-161`), the three `Option<usize>` ceilings (`:214-279`), `MID_BATCH_FAULT` (`:207-211`, `:281-307`) and the `NO_STORE_LIMITS` skip (`:442`) | Before deciding what the store must survive across handles, and before assuming the ceilings rule will exercise anything | AC-003, AC-006 |
| `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` | The six wrong implementations this story's behaviour is defined against, written out as compiling stores rather than described | When a targeted test feels like it cannot fail — read the mutant it is supposed to reject | AC-004, AC-006, AC-009 |
| `.kb/decisions/0014-event-identity-and-recorded-time.md` | Why `contains_event_id` is a *required* port method (`:27-29` names this `!Sync` adapter by construction) and the store-id incarnation rule (`:23`) | Before implementing `migrate`'s incarnation row or `contains_event_id` | AC-003, AC-009 |
| `.kb/decisions/0013-position-assignment-and-visibility.md` | Positions may have gaps and are the store's own fact — which is why the 2^53 test constructs its condition instead of asserting a number | Before writing the boundary test | AC-010 |
| `.kb/decisions/0001-async-port-flavours.md` | The two-trait derivation and the `+ Send` injection this adapter exists to falsify | Only if a signature, a bound or an `async` keyword is about to change | AC-001 |
| `.kb/decisions/0003-opaque-payloads.md` | Payloads are opaque `Bytes`; the contract crate carries no `serde` and this adapter never inspects one | While writing the `INSERT` binding for `data`/`metadata` | AC-001, AC-007 |
| `spec/SPECIFICATION.md` | The normative clauses: CF-40's ceiling promise (`:7661-7674`), ES-19's last-written position, ES-20/21's refusal order, ES-30's `head`, ES-41's identity membership — each carrying its maturity marker | When an AC's exact wording matters, or when a clause looks like it needs amending (it does not) | AC-004, AC-006, AC-007, AC-008, AC-009 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_decomposition.md` | Architecture brief §3 (data flow), §4a (the composition root), Notes §2 (the accepted atoms and the three tensions), §6 (the implementer's latitude), §7 (standing detectors); Testing brief Notes §1-§3 (tiers, the host-vs-wasm32 question, the seams) | Before the first line of code, then again when choosing where the targeted tests live | AC-001, AC-003, AC-006 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/durable-object-write-path/discover.md` | The named wrong implementations — "the ceiling nobody declared", the clamp-instead-of-chunk defect, and `WHERE position = ?` — with the reasoning that produced them | Before writing the AC-006 and AC-009 tests, so they reject something real | AC-006, AC-009 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_storymap.md` | The slice, the merge order, the split of project AC-001 across three stories, and the standing detectors that must not go quiet | When deciding whether something belongs in this PR or a slice-mate's | AC-001 |

## Clarifications resolved during spec

1. **The AC set is exactly the ten the front half enumerated** — AC-001 through AC-010, none added and
   none dropped. The ledger carries the same ten ids.
2. **The interaction-quality composition family is N/A by signed-off design**, not by omission.
   `_design.md` records no user-facing surface and was approved in that form; the state family is
   translated into the medium this story has (durable state and error channels) and every applicable
   invariant is carried as an existing `AC-###` row rather than as prose.
3. **`MID_BATCH_FAULT` is settled as discover asked**: the seam stays *affordable* — no schema decision
   here forecloses a one-shot armed fault — and this story declares **nothing**. The `Capability`
   constant is `durable-object-host-and-fixture`'s, the `SUPPORTED` claim `measured-store-limits`'.
4. **Object-wide storage exhaustion versus a declared ceiling.** Both can surface as
   `SqlError::StorageLimitExceeded`, and they are different facts: a batch crossing a ceiling this store
   declares is `ExceedsStoreLimit` naming that `StoreLimit` (EC-003), while exhaustion with no declared
   ceiling crossed is a genuine store failure and travels as `Store` (EC-004). Once the numbers exist,
   the capacity check runs *before* the insert, so the distinction stops depending on thrown text at all.
5. **Where the targeted tests live, and what happens if the `worker` swap breaks host linking.** They live
   in this crate's own tree as `event_store::write_path_tests`, mirroring `not_send_probe`'s `cfg`. If
   host linking does not survive, the module moves behind `#[cfg(all(test, target_arch = "wasm32"))]`
   with `wasm_bindgen_test`; the **test names are unchanged**, so the ledger's `verifying_test` column
   stays valid either way. The testing brief flags this as genuinely open and this spec does not pretend
   to have measured it.
6. **AC-006 is provable today even though the ceilings are `None`.** The numbers are
   `measured-store-limits`'; the *channel* is this story's, and it is exercised in both directions through
   a test-visible ceiling. Nothing here states a public numeric limit.
7. **The shared position decoder is this story's deliverable**, consumed by `durable-object-read-path`
   rather than written twice. Recorded so neither story assumes the other did it.
8. **The scoped `#![allow(clippy::todo)]` is conditional.** It is removed here only if this story lands
   the crate's last `todo!()`; otherwise it stays and the PR body says why (`_storymap.md`, **Coverage**).
