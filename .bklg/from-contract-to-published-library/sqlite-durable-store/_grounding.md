# Grounding — The first adapter that is not an instrument (HS-P0012, sqlite-durable-store)

Companion file to `project.md` and `_intake-brief.md`. Not an item file; no
frontmatter, no `redkiln` write-path constraints. Cites paths for the
architecture/UX/testing/deployment briefs to build on.

## Accepted decision atoms that constrain this project

- **`.kb/decisions/0012-append-shape-and-preconditions.md`** (ADR-0012, phase 4,
  `depends_on: kb-decision-0010`). `EventStore::append` keeps `events: &[Event]`
  — **`[PROVISIONAL]`, not frozen**, precisely because the evidence needed to lift
  it (an adapter with a real write path and a benchmark harness, "under a
  realistic batch and rejection mix") "does not exist yet; nothing in the
  workspace can produce it before phase 8 builds the SQLite adapter's multi-row
  insert benchmark." This project *is* that measurement. The decision names the
  successor now so a later phase cannot invent a third option: the binary
  question this project's ADR-0022 inherits is "keep the borrow, or move to
  `Vec<Event>`" — nothing else. Also load-bearing for `append`'s implementation:
  an empty batch is refused before any condition is evaluated; a condition is
  evaluated only against events the store already held when `append` began,
  never against the batch being appended; `AppendCondition` is a non-empty
  sequence of private `Guard { query, after }` behind a `guards()` accessor
  (`crates/happenstance-core/src/append.rs`); CF-39 (`MID_BATCH_FAULT`) requires
  an armed fault to make `append` return `Err`, and a fixture whose store absorbs
  every fault it can arm must decline the capability rather than pass vacuously.
- **`.kb/decisions/0015-validated-identifiers-and-store-limits.md`** (ADR-0015,
  phase 4). Mints `AppendError::ExceedsStoreLimit { limit: StoreLimit, len }`
  with exactly three `StoreLimit` variants (query-item ceilings are not an
  `append` outcome and route to the ingest policy seam instead), and mints CF-40
  so a `Fixture` can declare real numeric ceilings as `Option<usize>`
  associated constants. States plainly: "a capacity limit must never be enforced
  by a constructor and must never be enforced in `Deserialize`" — the quarantine
  argument. **CF-40's *ownership* — which ADR (0012 or 0015) the clause belongs
  to — is contradicted between this ADR's own header/decision-8 prose and its own
  Consequences section**, and is recorded as an open question rather than settled
  by either atom (`.kb/open-questions/cf-40-fixture-limits-ownership.md`,
  `status: accepted`, explicitly says the KB has no standing to resolve it). AC-009
  is written to match this exactly: "CF-40's clause home is recorded as still
  open, not settled in passing."
- **`.kb/decisions/0013-position-assignment-and-visibility.md`** (ADR-0013, phase
  4). Freezes the global (not per-boundary) visibility invariant: once any reader
  has observed an event at position P, no later read may yield an event at or
  below P that was not already visible, and an adapter must never make an event
  visible below one it has already exposed. The freeze's own grounding experiment
  is Postgres-specific (`xid8` + `pg_snapshot_xmin`) and does not bind this
  project's SQL strategy directly, but the invariant itself does: `SqliteEventStore::head`
  is documented in `crates/happenstance-sqlite/src/event_store.rs:226-235` as
  needing to query fresh every time rather than caching a value `append` updates,
  because "one file backs several handles" and a cached head would go stale the
  moment a second connection commits — this is ES-30 stated in the adapter's own
  `todo!()` comment and is a direct corollary of ADR-0013's global invariant.
  `SequencePosition::next` uses `NonZeroU64::checked_add` (not `saturating_add`).
- **`.kb/decisions/0011-read-laziness-and-isolation.md`** (ADR-0011, phase 4).
  `read` keeps `query: &Query`. The port's promise is "evaluated against one
  state sampled no later than the first poll" — laziness permitted, never
  required. **Directly binds this adapter's read path**: "an adapter issuing more
  than one statement per `read` must capture a position ceiling no later than the
  first poll and bound every later statement by it." `SqliteEventStore::read`
  pages at `PAGE_SIZE = 512` rows per `spawn_blocking` hop
  (`crates/happenstance-sqlite/src/event_store.rs:81`), which is unambiguously
  "more than one statement per read" for any log over 512 events — so
  `ReadCursor` (same file, `:307-328`) needs a snapshot ceiling captured at first
  poll, not merely the `resume_from`/`remaining` pagination state it already
  carries. No such ceiling field exists in the skeleton today; the architecture
  brief should treat adding it as required by ADR-0011, not as a design choice.
  `read` takes no `+ Unpin` bound (provisional, phase-12 deadline — not this
  project's to touch). `ReadOptions::from` is the caller-side inclusive lower
  bound; `AppendCondition`'s `after` is exclusive — the two are one field apart in
  the port and `event_store.rs:313-323`'s own doc comment on `resume_from` records
  a real historical bug where this crate's skeleton conflated them.
- **`.kb/decisions/0009-error-send-sync.md`** (ADR-0009, phase 2).
  `EventStore::Error` stays exactly `core::error::Error + 'static` on both
  flavours; `Send + Sync` is **not** required by the port and must not be added
  to `SqliteEventStoreError`/`SqliteProjectionStoreError` as a bound — the
  stronger property lives in a separate blanket marker trait
  (`ThreadSafeEventStore`) that generic code opts into, not in the port itself.
- **ADR-0001, ADR-0008** (async port flavours, one derivation for both ports) —
  already summarized in `CLAUDE.md`'s binding constraints; this project's
  `#[trait_variant::make]`-derived `SendEventStore`/`SendProjectionStore` impls
  and the hand-written `SqliteReadStream` state machine
  (`crates/happenstance-sqlite/src/event_store.rs:248-426`) are the concrete
  instance CLAUDE.md constraint 1 and 3 describe. The extensive doc comment on
  `SqliteReadStream` (`:248-271`) already states, and this project must preserve,
  *why* it holds no `!Send` rusqlite handle (`Statement`, `Rows`, `Transaction`)
  across a `poll_next` boundary.
- **ADR-0022 does not exist yet.** No `.kb/decisions/0022-*.md`, no
  `references/adr/0022-*.md`. AC-013 ("ADR-0022 is committed before the
  implementation") is this project's obligation to create it, not to consume an
  existing one — it is the one ADR this project is charged with authoring, per
  the CLAUDE.md/MEMORY.md rule that ADR authorship stays with the runbook's own
  ADR pass (which phase 8 *is*, per `RUNBOOK.md:4175`: "**Decisions it settles.**
  ADR-0022.").

## Existing code patterns and modules to follow

- **`crates/happenstance-sqlite/src/lib.rs`, `event_store.rs`,
  `projection_store.rs`** — the skeleton this project completes. Every SQL-facing
  body is `todo!()`; the crate-scoped `#![allow(clippy::todo)]`
  (`lib.rs:76-80`) is the marker AC-014 requires deleted, "named the phase that
  removes it." The module docs already record two compiled results worth
  reusing verbatim in the architecture brief rather than re-deriving: (1)
  `type Batch<'a> = rusqlite::Transaction<'a>` fails on `SendProjectionStore` for
  two independent, already-diagnosed reasons (`Connection: Send` but not `Sync`;
  `Transaction<'_>` itself `!Send`) — `projection_store.rs:9-31`; (2) binding an
  owned type to the GAT does not exempt the impl from writing `Self::Batch<'_>`
  literally — `error[E0195]` — `projection_store.rs:216-224`. The intended
  schema (event/event_tag/index, and the amendment in `RUNBOOK.md:4178-4187`
  adding `event_type` as a covering column on `event_tag` plus a
  `tag_cardinality` table) is documented at `event_store.rs:34-58`.
- **`SqliteEventStoreError`/`SqliteProjectionStoreError`** (`event_store.rs:151-193`,
  `projection_store.rs:179-206`) already enumerate the real failure modes this
  driver produces (`rusqlite::Error`, `ConnectionPoisoned`, `Worker(JoinError)`,
  `NoRuntime(TryCurrentError)`, `InvalidPosition`) — a testing brief should assume
  these variants exist and are exercised, not invent new ones without reason.
  `SqliteEventStoreError::NoRuntime` exists specifically because
  `tokio::runtime::Handle::try_current()` turns a would-be `spawn_blocking` panic
  under a non-tokio executor into an ordinary stream error — `event_store.rs:27-32`.
- **`crates/happenstance-testkit/src/contract.rs`** — the `Fixture` trait
  `SqliteFixture` must implement. `SECOND_HANDLE` is the one **MUST**
  (`:135-160`): `two_handles_observe_each_others_appends` panics rather than
  skips on a decline. `REOPEN` is a `SHOULD` (`:163-173`) and is deliberately the
  *weaker* of two possible meanings of "restart" — "every outstanding handle's
  process-level state discarded," not a hard process kill — which is exactly
  what closing and reopening a `rusqlite::Connection` onto the same file path
  gives for free. `MID_BATCH_FAULT` defaults declined (`:207-211`); nothing in
  this project's ACs asks for it, and stating that explicitly (with a reason, the
  same shape `MemoryFixture` and friends use) avoids an unstated skip. The three
  numeric limits (`MAX_EVENT_DATA_LEN`, `MAX_TAGS_PER_EVENT`,
  `MAX_EVENTS_PER_BATCH`, default `None`, `:213-279`) are **facts, not trades** —
  AC-009 wants them stated as the adapter's real ceilings, not left at the
  `None` default (`NO_STORE_LIMITS`, `:442`, is the sentinel for "declared no
  limits at all").
- **`crates/happenstance-testkit/src/fixtures.rs:243-273`** — `MemoryFixture`
  (`Arc<MemoryEventStore>`, `SECOND_HANDLE: Capability::SUPPORTED`) is the
  reference `Fixture` implementation CLAUDE.md names. `SqliteFixture` cannot copy
  its `Arc`-clone shape for `SECOND_HANDLE` — AC-003 explicitly rejects that:
  "A fixture returning a `Clone` of one store still passes the rule and does not
  satisfy this criterion." It must open a second, independent
  `rusqlite::Connection` onto the same file path.
- **`crates/happenstance-testkit/src/concurrency.rs:206`** —
  `pub const CONTENDERS: usize = 8;`, parallelism via `std::thread::scope` (not a
  runtime). The doc note at `RUNBOOK.md:155-165` is explicit that this is a
  **known, named discrepancy**: phase 8's and phase 10's own proof artefacts
  (`RUNBOOK.md:159`, `:4217-4222`) both read "the concurrency macro green ...
  at 64 contenders," while the constant in the testkit today is 8. AC-005 is
  written directly against this gap and requires one of two outcomes, not
  silence: raise `CONTENDERS` with a stated reason, or amend *both* proof
  artefacts.
- **`crates/happenstance-testkit/tests/mutation_coverage/`** — `mutants.rs`,
  `variants.rs`, `harness.rs`, `correct.rs`; registry format enforced by
  `xtask/src/proof.rs:72-89` (`mutation_coverage::every_rule_has_a_mutant`,
  `mutant_registry_is_exhaustive`, `mutants_fail_exactly_their_declared_rules`,
  `every_mutant_states_its_provenance`). AC-006's "phase-3 mutant harness is
  re-run with `SqliteEventStore` in the pass column" targets this file set.
  `crates/happenstance-testkit/README.md:108-113` documents the registry-row
  convention new adapters follow.
- **`xtask/src/package.rs`** — the packaging gate AC-015 must satisfy.
  `PUBLISHABLE` (`:86`) is currently exactly `["happenstance-core", "happenstance",
  "happenstance-testkit"]`; `REQUIRED_FILES` (`:94`) is
  `["LICENSE-MIT", "LICENSE-APACHE", "README.md"]`. The module's own doc comment
  (`:20-22`) names `happenstance-sqlite` **by name** as the wrong implementation
  this check exists to reject: "`cargo package -p happenstance-sqlite --list`
  exits 0, lists seven files, and none of them is one of the three" — true today
  (`crates/happenstance-sqlite/` has no `README.md`, no `LICENSE-*`; confirmed by
  directory listing). `reconcile()` (`:163-218`) derives the actually-publishable
  set from `cargo metadata` and fails loudly in **either** direction — a crate
  promoted (its `publish = false` gone) but absent from `PUBLISHABLE`, or a name
  in `PUBLISHABLE` that Cargo will not actually publish.
- **`crates/happenstance-sqlite/Cargo.toml`** — `publish = false` (`:12`);
  `description = "SQLite event store and projection store adapters for
  happenstance. Not yet implemented."` (`:3`) is stale text this project should
  replace since the crate stops being "not yet implemented," but the manifest
  field this project must **not** touch is `publish`.

## Tensions with Accepted decisions / stated plans

- **`RUNBOOK.md:4230`'s own phase-8 exit criterion literally reads `"[ ] publish
  = false removed."`** This directly contradicts AC-015's wording for this
  project — "The crate is publishable, **and publishing stays someone else's
  decision**" — and the initiative decomposition's explicit boundary line,
  `.bklg/from-contract-to-published-library/_decomposition.md:43`: "`sqlite-durable-store`
  does not own ... whether adapter crates are published at all
  (`publication-and-positioning`)." If `publish = false` is deleted here,
  `xtask/src/package.rs`'s `reconcile()` fails CI the moment `happenstance-sqlite`
  is promoted but `PUBLISHABLE` (owned by `publication-and-positioning`, per the
  decomposition) has not yet been updated to include it — exactly the
  "promoted-but-unreconciled" failure mode the module's own doc comment names.
  RUNBOOK.md is the older plan-of-record text and the intake brief/initiative ACs
  are the current authority; the architecture/deployment brief should state
  explicitly that `publish = false` stays, and record RUNBOOK.md's stale
  checkbox as superseded by the AC rather than silently diverging from it.
- **AC-008's `index_arms()` API does not exist in `happenstance-core` today.**
  `crates/happenstance-core/src/query.rs` has no `index_arms`, `arm_count`, or
  `IndexArm` — confirmed by grep across the crate. The name is inherited from
  `references/evaluation/ARCHITECTURAL-EVALUATION.md:827` (finding P1) and
  `references/evaluation/review-adapter-implementability.md:205-206`, which
  *propose* adding `Query::index_arms() -> impl Iterator<Item = IndexArm<'_>>` and
  `Query::arm_count()` as "purely additive inherent methods, so no semver
  hazard" — a proposal, not a landed decision, and no `.kb/decisions/` atom
  currently mints it (ADR-0011's read-side atom does not mention it; ADR-0012's
  append-side atom does not either). `RUNBOOK.md:4203-4206` carries the same
  work item under phase 8 verbatim. Two live options the architecture brief must
  choose between and record: (a) add `Query::index_arms()`/`IndexArm` to
  `happenstance-core` as the evaluation proposes — a change to a crate whose
  `EventStore` port is stated `[FROZEN]` at `spec/SPECIFICATION.md:371`, even if
  additive and to `Query` rather than to `EventStore` itself, and therefore
  worth flagging in ADR-0022 explicitly rather than landing silently; or (b)
  decompose `Query` into pushdown arms **inside** `happenstance-sqlite` using the
  already-public `Query::items()` / `QueryItem::types()` / `QueryItem::tags()`
  accessors (`crates/happenstance-core/src/query.rs:118-260`), keeping the 400-arm
  chunking logic adapter-private. Nothing in the ACs requires the public name
  `index_arms`; AC-008 only requires the *behavior* (chunked, not refused, at
  SQLite's pushdown limit, VT-23's 128-item floor passing).
- **CONTENDERS discrepancy is pre-existing and named, not introduced by this
  project** (see above) — surfaced here as grounding, not as a new finding;
  AC-005 already exists to force a resolution.
- **CF-40 ownership contradiction is pre-existing and explicitly left open** by
  its own `.kb/open-questions/` atom; AC-009 already anticipates this and asks
  only that the project *record* the open status, not resolve it.

## Anchors for the briefs to cite

- `.kb/decisions/0009-error-send-sync.md`, `0011-read-laziness-and-isolation.md`,
  `0012-append-shape-and-preconditions.md`, `0013-position-assignment-and-visibility.md`,
  `0015-validated-identifiers-and-store-limits.md`
- `.kb/open-questions/cf-40-fixture-limits-ownership.md`
- `crates/happenstance-sqlite/src/lib.rs`, `event_store.rs`, `projection_store.rs`
- `crates/happenstance-sqlite/Cargo.toml`
- `crates/happenstance-testkit/src/contract.rs`, `fixtures.rs`, `concurrency.rs`
- `crates/happenstance-testkit/tests/mutation_coverage/` (`mutants.rs`,
  `variants.rs`, `harness.rs`, `correct.rs`), `crates/happenstance-testkit/README.md:108-113`
- `xtask/src/package.rs`, `xtask/src/proof.rs:72-89`
- `RUNBOOK.md:159`, `:685-700` (portfolio table), `:2680-2700` (CONTENDERS note),
  `:3200-3212` (ES-35/CF-17 unfinished mutant note), `:3805-3825` (standing
  exit-criterion rule), `:4166-4235` (Phase 8 section, full), `:4227-4228`
- `spec/SPECIFICATION.md` — ES-35 (`:4142-4149`), CF-14 (`:7471-7490`), CF-17
  (`:7570-7583`), CF-34 (`:8263-8265`, `:8774`), VT-21–VT-24 (`:1483-1574`),
  frozen `EventStore` cell (`:371`)
- `references/evaluation/ARCHITECTURAL-EVALUATION.md:827` (P1, `index_arms`
  proposal), `references/evaluation/review-adapter-implementability.md:205-206`
- `.bklg/from-contract-to-published-library/_decomposition.md:43` (this
  project's non-owned boundaries), `:255-257` (freeze-column table),
  `:295-300` (DoD 7 watched edge)
