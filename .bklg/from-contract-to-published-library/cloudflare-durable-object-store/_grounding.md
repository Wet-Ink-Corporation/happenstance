# Grounding — The edge store, run rather than asserted (HS-P0013 / cloudflare-durable-object-store)

Companion file. Not an item; the CLI never writes this path. Cites real repo
paths only — every claim below was checked against the tree before being
written down.

## 1. Accepted decision atoms that constrain this project

- **ADR-0001 — async ports in two flavours** (`.kb/decisions/0001-async-port-flavours.md`).
  Defines `EventStore` with no `Send` bound and derives `SendEventStore` via
  `#[trait_variant::make(SendEventStore: Send)]`; `read` returns its `Stream`
  at the top level rather than from an `async fn`. This is the design the
  whole project exists to prove by execution. **Its `provisional` marker was
  already lifted at phase 1** — not by this project — when
  `LocalMemoryEventStore` passed the suite natively and on `wasm32`; ADR-0008
  records the lift (`.kb/decisions/0008-one-derivation-for-both-ports.md:22`).
  RUNBOOK.md's phase-9 entry (`RUNBOOK.md:4280-4282`) describes this project's
  proof artefact as lifting that marker "formally" — read that as *citing the
  adapter that finally makes the already-accepted decision's premise true
  under a real runtime*, not as a second lift. Briefs should not claim ADR-0001
  is still provisional.

- **ADR-0008 — one derivation for both ports** (`.kb/decisions/0008-one-derivation-for-both-ports.md`).
  Settles that a rule asserting `Send`-ness on the future's `Output`, not the
  future itself, is what a `!Send`-error adapter can falsify — this is exactly
  what `happenstance-cloudflare`'s `send_shape::send_flavour` module already
  probes (`crates/happenstance-cloudflare/src/send_shape.rs`, cited at
  `.kb/decisions/0008-one-derivation-for-both-ports.md`'s own related list).

- **ADR-0009 — error stays unbounded, strength moves to a marker trait**
  (`.kb/decisions/0009-error-send-sync.md`). This is ES-6, already **decided**,
  not merely deferred: `Error` keeps `core::error::Error + 'static` on both
  ports/flavours; a separate blanket-impl marker
  (`ThreadSafeEventStore: SendEventStore<Error: Send + Sync>`) is what generic
  code opts into for the stronger bound. The decision was reached by compiling
  four things against `happenstance-cloudflare` specifically — its `Rc<str>`
  error is the one crate-wide check that fails a workspace-wide `+ Send + Sync`
  probe. **AC-005's "bound added, or ADR-0009's deferral confirmed" framing is
  stale relative to the KB**: ADR-0009 is Accepted, not deferred, and this
  project's job under it is to keep `CloudflareEventStoreError` genuinely
  `!Send` (already true — see §2) and to write the committed
  reconstruct-from-caller-visible-error test AC-005 asks for, not to re-litigate
  whether the bound gets added. A brief that re-opens ES-6 as still-undecided
  contradicts an Accepted atom; flag this explicitly rather than resolve it
  silently.

- **ADR-0011 — a read is one sample with a ceiling, `&Query` stays**
  (`.kb/decisions/0011-read-laziness-and-isolation.md`). This is the single
  most load-bearing citation for the architecture brief. It corrects the
  port's promise from "lazy" to **"evaluated against one state sampled no
  later than the first poll"**: laziness is *permitted, never required*, and
  — the operative mechanism — **"an adapter issuing more than one statement
  per read must capture a position ceiling no later than the first poll and
  bound every later statement by it"** (lines 19-21 of the atom body), and
  this is stated to discharge isolation **across all three store shapes in
  the workspace, one of which is named literally as "chunked cursor"**
  (line 71). That is this adapter's shape. See §3 for the tension this
  creates against the crate's current framing of the cursor problem as an
  open capability trade-off.

- **ADR-0014 — the store mints identity, records a time**
  (`.kb/decisions/0014-event-identity-and-recorded-time.md`). `contains_event_id`
  is a required port method (not provided, because the provided form needs
  `Self: Sync`, which this deliberately `!Sync` bare-flavour adapter cannot
  satisfy — this is the ADR's own reasoning, and it names this adapter's
  flavour by construction). `EventId` is a `StoreId` + position pair behind
  private fields; the schema needs identity columns to answer it, which is
  why `CloudflareEventStore::contains_event_id`'s `todo!()`
  (`crates/happenstance-cloudflare/src/event_store.rs:187-195`) says the
  intended schema "predates ingest" and names the columns
  (`origin_store`, `origin_position`) it still needs.

- **ADR-0003 — opaque payloads** (`.kb/decisions/0003-opaque-payloads.md`,
  restated in `CLAUDE.md`). `serde` stays out of `happenstance-core`'s default
  features; irrelevant to this adapter's own dependency surface (it will take
  `worker`, not `serde`) but binds the schema's `data`/`metadata` columns to
  stay `Bytes`-opaque, matching `happenstance-sqlite`'s `BLOB` columns.

- **ADR-0004 / ADR-0029 — MSRV** (`.kb/decisions/0029-msrv-raised-to-1-97-1.md`,
  amending `.kb/decisions/0004-edition-and-msrv.md`). MSRV 1.97.1, still
  `provisional` until phase 12 (first publish). Applies to any Rust this
  project writes; `worker` as a new dependency is a candidate for widening
  the five-crates-with-no-`rust-version` gap the atom names, and CLAUDE.md's
  MSRV section should be checked once `worker`'s own `rust-version` (if any)
  is known.

## 2. Existing code and what it already establishes as fact

`crates/happenstance-cloudflare/` is a real skeleton, not a stub with holes
guessed at — every `todo!()` site names exactly what phase 9 (this project)
must fill in, and the crate's module docs already record four *compiled*
findings that are inputs to this project, not open questions it re-derives:

1. `JsValue` **is** `Send + Sync` on non-`atomics` `wasm32-unknown-unknown` —
   `wasm-bindgen` 0.2.126's own `unsafe impl`, quoted verbatim at
   `crates/happenstance-cloudflare/src/lib.rs:33-47`. `Rc<str>`, not `JsValue`,
   is what makes `JsHandle` (`crates/happenstance-cloudflare/src/js.rs`) and
   therefore `CloudflareEventStoreError` genuinely `!Send`.
2. Stringifying a thrown value (`js::StringifiedThrow` vs. `js::JsThrow`) loses
   forward-compatibility, not the one signal a caller needs today — a Durable
   Object surfaces SQLite's own text through the thrown `Error`'s `message`
   with no numeric code either way
   (`crates/happenstance-cloudflare/src/lib.rs:63-73`).
3. The DCB conflict signal never travels in `Self::Error` on *any* adapter —
   `happenstance-core` lifts it into `AppendError::ConditionViolated` before
   `Self::Error` is constructed — which is why
   `CloudflareEventStoreError` deliberately has no `ConditionViolated` variant
   (`crates/happenstance-cloudflare/src/event_store.rs:100-104`).
4. `send_shape::send_flavour::SendStoreWithLocalError` compiles: the derived
   `Send` flavour does **not** imply a `Send` error
   (`crates/happenstance-cloudflare/src/lib.rs:87-94`,
   `crates/happenstance-cloudflare/src/send_shape.rs`). This is why ES-6's
   named rule `store_error_crosses_a_join_handle` was unwritable against the
   old port for every adapter — see the open-question atom in §4.

`crates/happenstance-cloudflare/src/sql_storage.rs` models the four load-
bearing properties of the real Workers `SqlStorage` API (synchronous `exec`,
non-snapshot cursor, `!Send`/`!Sync` throughout, single-threaded-but-
re-entrant object) with a doc comment citing Cloudflare's own documentation
for each. `SqlError::AlreadyBorrowed` exists specifically because a Durable
Object is re-entrant: two `append` futures from one handle can be polled
alternately, both holding `&self`
(`crates/happenstance-cloudflare/src/sql_storage.rs:101-109`) — an adapter
detail worth citing in the testing brief, since it is the concrete shape of
the re-entrancy conformance family the crate doc table already lists
(`crates/happenstance-testkit/src/lib.rs:149`, "Re-entrancy" row).

The intended schema is declared **identical** to `happenstance-sqlite`'s
(`crates/happenstance-cloudflare/src/event_store.rs:8-28` vs.
`crates/happenstance-sqlite/src/event_store.rs:34-54`), on the stated
reasoning that the storage engine *is* SQLite and everything this crate exists
to test is above the schema. Follow that schema, adding the identity columns
ADR-0014 requires (`happenstance-sqlite`'s own schema comment does not yet
carry them either — `happenstance-sqlite` is itself still a skeleton owned by
`sqlite-durable-store`, so there is no finished sibling to copy verbatim;
`crates/happenstance-sqlite/src/event_store.rs:238`'s comment names the same
two columns this adapter needs).

`happenstance_testkit::event_store_conformance!` already has a third emitter
built and demonstrated for exactly this target:
`crates/happenstance-testkit/tests/memory_conformance_wasm.rs` invokes it with
`emit = happenstance_testkit::__emit_wasm` against `MemoryFixture` under
`#[cfg(target_arch = "wasm32")]`. `__emit_wasm` itself
(`crates/happenstance-testkit/src/registry.rs:260-291`) wraps each rule in
`#[wasm_bindgen_test::wasm_bindgen_test]`; `wasm-bindgen-test` is already a
dev-dependency of `happenstance-testkit` (`crates/happenstance-testkit/Cargo.toml:54`)
but **not yet** of `happenstance-cloudflare`, which will need it added. This
is the pattern AC-002 asks the Cloudflare fixture to reuse, not invent.

**The gap AC-002/AC-004 exist to close is already documented in-tree, not
hypothetical.** `xtask/src/main.rs`'s "wasm32 check of the conformance
harnesses" step (`xtask/src/main.rs:219-244`) runs `cargo check --tests
--target wasm32-unknown-unknown`, and its own comment says exactly what that
buys: "Type-checking them here is what stops `__emit_wasm` from rotting into
a macro nobody has compiled since the day it was written" — it explicitly
does **not** run the tests. `memory_conformance_wasm.rs`'s own doc comment
concurs: "`cargo xtask wasm` type-checks it, which is the part that can rot."
Likewise "wasm32 build of the Cloudflare adapter" (`xtask/src/main.rs:245-264`)
is a `cargo check`, not a run. **No step in `xtask/src/main.rs` currently
executes anything under `wasm-bindgen-test-runner` or `workerd`.** AC-004's
"one `cargo xtask ci` … executes it" is therefore new work, not a wiring
change to an existing runner — the deployment brief needs to answer the
intake brief's own named open question, "whether a `workerd` runner can be
made to exist in CI at acceptable cost," from nothing. RUNBOOK.md's phase-9
work item names a concrete direction — **`vitest-pool-workers` as its own CI
job** (`RUNBOOK.md:4267-4268`) — as the vehicle for the `workerd` harness; that
is a plan-of-record pointer, not an accepted decision, and should be treated
as the starting hypothesis for the deployment brief rather than as settled.

`happenstance-cloudflare/Cargo.toml` currently has `publish = false` and no
`LICENSE-MIT`, `LICENSE-APACHE` or `README.md` in its crate directory
(confirmed: `crates/happenstance-cloudflare/` holds only `Cargo.toml` and
`src/`), unlike `happenstance-core`, `happenstance` and `happenstance-testkit`,
each of which carries all three
(`xtask/src/main.rs:751-752` names the `cargo package --list` assertion this
gap will fail until AC-012 adds them).

## 3. A tension worth flagging explicitly

The crate's own module documentation (`crates/happenstance-cloudflare/src/lib.rs:96-112`)
frames "a lazy read stream is not a stable snapshot" as one of two
**capability limits that are not type errors** — something the fixture may
have to *decline* or trade off, alongside the buffer-vs-stream choice it
poses as open. But **ADR-0011 is Accepted** and already prescribes a specific
mechanism for exactly this shape — "chunked cursor" is one of the three named
store shapes the ceiling technique is stated to discharge
(`.kb/decisions/0011-read-laziness-and-isolation.md:69-71`): capture a
position ceiling no later than the first poll, and bound every later
statement by it. `event_store.rs`'s current `SqlRowStream` holds a live
`SqlCursor` across polls and only *detects* invalidation after the fact
(`check_cursor_still_valid`, `crates/happenstance-cloudflare/src/event_store.rs:292-306`)
rather than avoiding it by paging with a captured ceiling. The architecture
brief should treat ADR-0011's mechanism as the leading candidate resolution —
not a fresh capability trade-off to invent — and only fall back to a declined
capability if the ceiling-and-page approach turns out not to fit the Workers
`SqlStorageCursor` API. This reading is consistent with the project's own
AC-003, which already offers "honoured" as the first of three acceptable
outcomes for this exact clause.

## 4. Open-question atoms this project is named as forcing or exercising

- `.kb/open-questions/cf-40-fixture-limits-ownership.md` — CF-40's ownership
  is contested between ADR-0015 and ADR-0012 by the *same* ADR's own
  self-contradicting text; forced by "the first adapter with real limits,"
  which this project is (AC-008 must coordinate the resolution with
  `sqlite-durable-store` per the decomposition's non-goals list,
  `.bklg/from-contract-to-published-library/_decomposition.md:44`).
- `.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md`
  (WF-11) — explicitly "Owned by phase 9, the Cloudflare Durable Object
  adapter"; the falsifier is a peer that must forward a payload larger than it
  can buffer whole. AC-011 tests this falsifier here without touching the wire
  format.
- `.kb/open-questions/es-6-names-an-unwritable-rule.md` — records that ES-6's
  named rule was unwritable *before* ADR-0009; note this atom pre-dates
  ADR-0009's acceptance and should be read alongside it, not instead of it —
  ADR-0009 states the rule is now writable against its marker.

## 5. Non-goals confirmed against the decomposition of record

`.bklg/from-contract-to-published-library/_decomposition.md:44` states this
project's non-goals verbatim: it does not own the projection-suite
capability-reporting *policy* (`projection-store-freeze` — this project is
its first real exerciser), reopening the tail seam (post-0.1, outside this
initiative — AC-010 records a verdict, does not act on it), or publishing the
crate (`publication-and-positioning` — AC-012 makes the crate *publish-ready*
only). Line 155 confirms this project is one of the decomposition's two DAG
roots (alongside `projection-store-freeze`), consistent with the intake
brief's "no upstream blockers."
