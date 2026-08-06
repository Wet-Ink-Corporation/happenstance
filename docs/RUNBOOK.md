# Runbook

The plan for finishing happenstance, and the record of how far it has got.

The project's state currently lives in the README's status table, CLAUDE.md's
open questions, and prose inside each stub's `lib.rs`. Those say what *is*. This
says what happens next, in what order, and what "finished" means for each step —
and it carries the progress state, so a session that starts cold can read this
one file and know where to pick up.

## How to use this

1. **The runbook is updated in the same commit as the work it describes.** A
   phase whose boxes are ticked but whose code is not committed did not happen.
2. **A phase is done when `cargo xtask ci` is green *and* every exit criterion
   is ticked.** Not before. "It compiles" is not an exit criterion anywhere in
   this document.
3. **Decisions are settled in ADRs, not here.** This file names each open
   decision and the phase that owns it; [`docs/adr/`](adr/) holds the answer.
   Settling one silently in passing is the failure mode CLAUDE.md warns about,
   and the [decision ledger](#decision-ledger) exists to make it visible.

## Session protocol

```
1. Read the status table below, then `git log --oneline -10`.
2. Run `cargo xtask ci`. Establish the baseline is green before touching anything.
3. Pick the first phase that is not `done` and whose dependencies are `done`.
4. Work it. Settle its decisions in an ADR *before* writing the code they constrain.
5. Re-run the gate. Tick the exit criteria. Add a dated line to that phase's session log.
6. Commit the code and this file together.
```

## Status

| # | Phase | Depends on | State | Exit gate |
|---|---|---|---|---|
| 0 | [Baseline](#phase-0--baseline) | — | in progress | gate green on a clean tree, runbook committed |
| 1 | [SQLite event store](#phase-1--sqlite-event-store) | 0 | not started | conformance suite green |
| 2 | [Projection port and its suite](#phase-2--projection-port-and-its-suite) | 1 | not started | port frozen, SQLite projection store green |
| 3 | [`happenstance-runtime`](#phase-3--happenstance-runtime) | 2 | not started | worked example rewritten against it |
| 4 | [Ladybug projection store](#phase-4--ladybug-projection-store) | 2 | not started | projection conformance green |
| 5 | [Cloudflare Durable Object](#phase-5--cloudflare-durable-object-adapter) | 1 | not started | conformance green under wasm |
| 6 | [`happenstance-sync`](#phase-6--happenstance-sync) | 3, 5 | not started | replication round-trip test |
| 7 | [Publish 0.1](#phase-7--publish-01) | 2, 3, 6 | not started | crates on crates.io, docs.rs green |

State is one of `not started`, `in progress`, `blocked`, `done`. Edit it in
place.

Phases 4 and 5 both depend on earlier work but not on each other; either order
is fine, and 5 is the riskier one, which is an argument for doing it first.

## Decision ledger

Every question that is deliberately open, and the phase that owns the answer.
Nothing here gets settled outside its phase without an ADR saying why.

A row marked **decided** has an agreed answer but no ADR yet; it is not settled
until the ADR lands with the code, per rule 3 above.

| Decision | Phase | Status | ADR |
|---|---|---|---|
| SQLite driver: `rusqlite` vs `sqlx` | 1 | **decided** — `rusqlite`, one writer connection + read pool, WAL, `busy_timeout`, `prepare_cached` | pending |
| Append-condition strategy: `BEGIN IMMEDIATE` + `EXISTS` probe vs conditional `INSERT ... SELECT ... WHERE NOT EXISTS` vs monotonic-position guard | 1 | **decided** — `BEGIN IMMEDIATE` + a probe returning the *conflicting position*; `max(position)` fast path deferred until measured | pending |
| Tag storage: join table vs canonical blob vs JSON1 | 1 | **decided** — blob on `event` for storage, `event_tag` as a derived index; SQLite cannot index JSON containment | pending |
| Do projections poll, or does `EventStore` grow a tail/subscription seam | 1 | open — N views means N independent reads of the log today. Owned here because it changes the *port*, and adding a required method once adapters exist breaks every one of them; the runner that pays the cost is phase 3 | — |
| Does the `ProjectionStore` port survive contact with a real transaction API | 2 | **decided** — survives, with amendments: conditional `commit`, drop-rolls-back documented, harness-based conformance | pending |
| Does the port grow an `apply` seam a generic runner can drive, or is applying an event adapter-bound by design | 2 | **decided** — both. The port grows the seam; the `Batch` written through stays adapter-specific, so a projection targets exactly one store and one spanning two does not compile | pending |
| Read-model lifecycle: where DDL and migrations live, and how a projection is reset for a rebuild | 2 | open — there is no `reset`, so discarding a checkpoint means going around the port | — |
| Event identity across instances: UUIDv7 vs content hash, metadata key vs contract type | 3 | **decided** — Lamport pair `(origin, origin_position)` as `EventId` on `SequencedEvent`; store-assigned | pending |
| Is `happenstance-runtime` the right name and the right seam | 3 | **decided** — no. Bare name inverts to the typed layer; contract becomes `happenstance-core`; projection runner moves into it | [ADR-0006](adr/0006-bare-name-to-the-typed-layer.md) |
| Where the projection runner lives, and whether it hands the application decoded events | 3 | **decided** — it splits. The checkpoint pump stays in `happenstance-core`; the typed `Projection` trait and the runner an application uses move to `happenstance`. Partly supersedes the row above | [ADR-0007](adr/0007-projection-runner-decodes.md) |
| Projection failure policy: when `apply` fails on event N — retry, skip, halt that projection, or dead-letter | 3 | open — with many views, one poisoned projection must not stall the others | — |
| Ladybug checkpoint: a node in the graph vs beside it | 4 | open | — |
| How a projection expresses graph mutations: raw Cypher vs a typed builder | 4 | open | — |
| `lbug`'s synchronous API: `spawn_blocking` wrapper vs blocking-only adapter | 4 | open | — |
| Conformance harness for a `!Send`, non-tokio runtime | 5 | **decided** — no testkit restructure needed; add a rule *registry* macro + a `!Send` reference store (pulled forward to phase 1); `vitest-pool-workers` as its own CI job | pending |
| Does sync ingest re-check append conditions, or is replication unconditional append-of-facts-already-decided | 6 | open — *the* central question of that crate | — |
| Merge rule for two independently-ordered logs | 6 | open | — |
| Is replication a port — transport and ingest as separate seams — or one concrete crate | 6 | open — nothing in that crate is a trait today; deferring it leaks `EventId` and a tail seam back into `EventStore` | — |

Sources, for when a row needs its full context back: `happenstance-sqlite/src/lib.rs`,
`happenstance-ladybug/src/lib.rs`, `happenstance-sync/src/lib.rs`,
`happenstance-runtime/src/lib.rs`, `happenstance/src/projection.rs`.

---

## Phase 0 — Baseline

**Goal.** A green gate on a clean tree, and this runbook in the repository.

**Why here.** Every later phase's exit criterion is "the gate is green". That
claim is worth nothing unless it was green to begin with.

**Work**

- [x] `cargo xtask ci` passes on a clean checkout.
- [ ] Decide what happens to the untracked `.idea/` and `.mcp.json` — gitignore
      or commit. Leaving them untracked makes "clean tree" ambiguous forever.
- [ ] Optionally `cargo install cargo-hack cargo-deny --locked`, so the local
      gate runs the two steps that currently print `skipped` and are enforced
      only in CI.
- [ ] Commit this runbook, the CLAUDE.md pointer and the README link.

**Exit criteria**

- [ ] `cargo xtask ci` green.
- [ ] `git status` clean.

**Session log**

- 2026-08-05 — runbook authored. Gate verified green at `9fd2337`; `cargo hack`
  and `cargo deny` skipped locally as expected. Remaining: the `.idea/` /
  `.mcp.json` decision, then commit.

---

## Phase 1 — SQLite event store

**Goal.** `SqliteEventStore` implements `SendEventStore` and passes the full
conformance suite.

**Why here.** It is the first adapter that is not the in-memory reference store,
which makes it the first real test of whether the 27 conformance rules describe
*behaviour* or merely describe `MemoryEventStore`. Everything downstream — the
runtime's command loop, replication, the Cloudflare adapter's schema — is easier
to design once one durable store exists. It is also the store the motivating
local-first application actually ships.

**Decisions to settle first** — one ADR covering all three; they interact.

- Driver: `rusqlite` (synchronous, bundles SQLite, the natural local-first fit)
  vs `sqlx` (async-native, a path to Postgres later).
- Append-condition strategy. The condition check and the write must be one
  atomic step.
- Tag storage. `Tags` is canonically sorted at construction precisely so the
  blob option stays open — this is where that choice gets spent.

And one that is not a SQLite decision, but is forced by this phase's timing and
needs its own ADR:

- **Do projections poll, or does `EventStore` grow a tail/subscription seam?**
  Every projection reads the log independently today, so N materialised views
  cost N reads. The answer changes the **port**, not this adapter — and a
  required method added once adapters exist breaks all of them, so it is far
  cheaper to answer before the first one ships than after. Answering "poll" is a
  legitimate outcome; leaving it unanswered until phase 3 is not.

**Work**

- [ ] ADR for the three decisions above.
- [ ] Schema and migrations, starting from the sketch in
      `crates/happenstance-sqlite/src/event_store.rs`. The `AUTOINCREMENT`
      rationale there is load-bearing: positions must never be reused after a
      delete, and plain `rowid` does not guarantee that.
- [ ] `read` — replace the `Pending` stand-in stream with the real one. The
      stream is returned at the top level and `read` is not `async`; see
      [constraint 3](#standing-constraints).
- [ ] `append` — condition evaluation and write in one transaction, returning
      the position of the last event written.
- [ ] Real variants on `SqliteEventStoreError`, replacing `Unimplemented`.
- [ ] `crates/happenstance-sqlite/tests/conformance.rs` invoking
      `happenstance_testkit::event_store_conformance!` against a store built in
      a fresh temporary directory — the macro re-evaluates the expression per
      test, so it must be a genuinely fresh, empty store each time.

**Exit criteria**

- [ ] All 27 conformance rules pass. Expect
      `racing_conditional_appends_elect_one_winner` to be the one that hurts —
      it is the case DCB exists to prevent, and the one a naive
      read-then-write implementation fails.
- [ ] No `todo!()` remains on the event-store path.
- [ ] `cargo xtask ci` green.

**Session log**

---

## Phase 2 — Projection port and its suite

**Goal.** A conformance suite for `ProjectionStore`, a SQLite implementation
that passes it, and the port frozen against both.

**Why here.** These are one phase because neither half is trustworthy alone: a
port without a conformance suite is a guess, and a suite written without a real
adapter to check it against is a guess about a guess. Phase 1 supplies the
transaction API to check it against.

The port is currently marked provisional in
`crates/happenstance/src/projection.rs`. Freezing it changes `happenstance`'s
public API, so **this must land before phase 7.**

**Work**

- [ ] `projection_store_conformance!` in the testkit, mirroring the event-store
      macro's shape: one `#[tokio::test]` per rule, so a failure names the rule.
- [ ] Rules built around the invariant the port exists to enforce — the
      read-model write and the checkpoint write commit together or not at all.
      Cover: a fresh projection has no checkpoint; commit advances it; rollback
      leaves both unchanged; a dropped batch (the crash case) leaves both
      unchanged; distinct `ProjectionId`s advance independently.
- [ ] `SqliteProjectionStore`, with `Batch<'a>` as a real SQLite transaction.
- [ ] Add the apply seam the port lacks, per
      [ADR-0007](adr/0007-projection-runner-decodes.md). `Batch` carries no trait
      bounds today, so generic code can open a batch and commit it but cannot
      *write* to one — which is why no projection runner can be written against
      the port in either crate. Settle its shape here; the ADR settles only that
      it exists, and that a projection's store is an associated type so one
      spanning two stores does not compile.
- [ ] Settle the read-model lifecycle: where DDL and migrations live, and how a
      projection is reset for a rebuild. There is no `reset`, so discarding a
      checkpoint currently means going around the port — and a rebuild is the
      most common thing anyone does to a read model.
- [ ] Freeze the port: remove the provisional status from the module docs, and
      write the ADR recording what the real transaction API proved about it.
      If the shape had to change, the ADR says what and why.

**Exit criteria**

- [ ] Projection conformance suite green against the SQLite implementation.
- [ ] `crates/happenstance/src/projection.rs` no longer says "provisional".
- [ ] `cargo xtask ci` green.

**Session log**

---

## Phase 3 — `happenstance-runtime`

**Goal.** The typed layer an application actually programs against, proven by
rewriting the worked example on top of it.

**Why here.** It needs a durable store to be worth demonstrating (phase 1) and
a projection store to run projections into (phase 2). It is also the last
sensible moment to answer the event-identity question before publishing —
see the [release hazard](#release-hazard).

**Decisions to settle first**

- ~~The crate's name and seam.~~ Settled by
  [ADR-0006](adr/0006-bare-name-to-the-typed-layer.md): the contract crate
  becomes `happenstance-core`, this crate becomes `happenstance`, and the
  projection runner moves into the contract crate because the discriminator for
  this seam is *encoding*, not orchestration.
- Event identity — pulled forward from phase 6 deliberately. See below.

**Work**

- [ ] Execute the ADR-0006 rename: `happenstance` → `happenstance-core`,
      `happenstance-runtime` → `happenstance`, adapters repointed. Mechanical,
      and cheapest before the worked example is rewritten.
- [ ] ADR on event identity (UUIDv7 or content hash; metadata key or a field on
      `Event`). Answer only the identity question — the rest of replication
      stays in phase 6.
- [ ] `Codec` — JSON, CBOR, postcard. Events carry a codec tag so one store can
      hold more than one encoding at a time, which is what makes a migration
      possible.
- [ ] `DomainEvent` — a Rust type's mapping to its `EventType` and `Tags`. No
      derive macro: `happenstance-macros` stays deliberately absent until there
      is something worth deriving, and is **out of scope for 0.1**.
- [ ] `DecisionModel` — folds read events into decidable state and produces the
      matching `Query`. Composing several into one query is the mechanism that
      makes a dynamic consistency boundary dynamic; that composition is the part
      to get right.
- [ ] The command loop — read, decide, append, retry on
      `AppendError::ConditionViolated`. Retry policy is a decision worth stating
      in the docs, not just implementing.
- [ ] The typed projection runner, per
      [ADR-0007](adr/0007-projection-runner-decodes.md): the `Projection` trait
      an application implements — decoded events, the projection's `Query`, the
      store's own `Batch` — layered over the checkpoint pump that stays in
      `happenstance-core`. A projection nominates its events with `Query`, the
      same type a decision model uses; there is no second filter vocabulary.
- [ ] Settle the projection failure policy: when `apply` fails on event N, does
      the runner retry, skip, halt that projection, or dead-letter it? With many
      views, one poisoned projection must not stall the others. State the policy
      in the docs, not only in the code.
- [ ] Rewrite `examples/course-subscriptions` against the runtime and the SQLite
      store.

**Exit criteria**

- [ ] `cargo run -p course-subscriptions` demonstrates the library an
      application would actually use — typed events and a decision model, not
      hand-rolled byte payloads.
- [ ] `publish = false` removed from the crate.
- [ ] `cargo xtask ci` green.

**Session log**

---

## Phase 4 — Ladybug projection store

**Goal.** A second implementation of the frozen projection port, against a
transaction API deliberately unlike SQLite's.

**Why here.** One implementation freezes a port; two prove it. Ladybug is a
graph engine with a Cypher interface — if the port does not fit it, that is the
discovery this phase exists to make, and the right response is a superseding ADR
rather than a workaround in the adapter.

Projections only. This crate will not offer an event store: an event log needs a
monotonic append with a conditional write, and forcing that onto an engine built
for analytical traversal would produce something that satisfies the trait and
not the specification.

**Decisions to settle**

- Whether the checkpoint lives in the graph as a node or beside it — this hinges
  on what Ladybug's transaction API actually guarantees.
- How a projection expresses graph mutations: raw Cypher or a typed builder.
- Whether `lbug`'s synchronous API is wrapped in `spawn_blocking` or the adapter
  is offered as blocking-only.

**Work**

- [ ] ADR for the above.
- [ ] Add the `lbug` dependency and measure the cold build. It compiles C++
      through `cxx` and `cmake`; a multi-minute native build is exactly why this
      is a separate crate and not a feature flag.
- [ ] Implement `SendProjectionStore`.
- [ ] Invoke the projection conformance suite.
- [ ] CI: if the build cost is material, give this crate its own job rather than
      slowing the three-OS gate everyone runs.

**Exit criteria**

- [ ] Projection conformance green.
- [ ] Build cost measured and the CI decision recorded here.
- [ ] `publish = false` removed.
- [ ] `cargo xtask ci` green.

**Session log**

---

## Phase 5 — Cloudflare Durable Object adapter

**Goal.** A new crate implementing the bare `EventStore` flavour against a
Durable Object's SQLite, passing conformance in a Workers runtime.

**Why here.** This is the payoff of ADR-0001. Until now the two-flavour port
design has been justified by a `cargo check` for `wasm32` and nothing else — no
`!Send` implementation exists. This phase is what makes that design earn its
keep, or reveals that it does not.

It is **not** a feature of `happenstance-sqlite`. A Durable Object's SQLite is
reached through the Workers `SqlStorage` API, not a SQLite driver, and its
futures are `!Send`.

**The risk, stated first.** `event_store_conformance!` expands to
`#[tokio::test]` functions. Those do not run in a Workers runtime. Designing the
harness — `wasm-bindgen-test`, driving `workerd` from an integration test, or
splitting the rules out from their tokio wrapper so a second macro can wrap them
differently — comes *before* writing the adapter. If the rules need restructuring
to be runnable off tokio, that is a testkit change and it belongs in this phase.

**Work**

- [ ] Decide the crate name and add it to the workspace.
- [ ] Decide and build the conformance harness. Restructure the testkit if
      required.
- [ ] Implement `EventStore` (not `SendEventStore` — the bound cannot be
      satisfied here).
- [ ] Run the suite.

**Exit criteria**

- [ ] Every conformance rule runs and passes under the Workers runtime.
- [ ] `cargo xtask ci` green, including the existing `wasm32` check.

**Session log**

---

## Phase 6 — `happenstance-sync`

**Goal.** Replication between two happenstance instances.

**Why here.** Last of the code, because it needs a second real store to
replicate against (phases 1 and 5) and the typed layer to exercise it with
(phase 3).

The easy part is already paid for: because payloads are opaque `Bytes`, a peer
forwards events without deserialising them — it never needs the sender's domain
types, cannot fail to parse a payload it does not understand, and cannot corrupt
one by re-encoding it.

**Decisions to settle** — one ADR.

- **Whether ingest re-checks append conditions**, or replication is defined as
  unconditional append of facts already decided. This is *the* central design
  question of the crate; a condition checked against the local log says nothing
  about the remote one.
- The merge rule. The specification requires a total order per store, so a
  replicated event's local position will differ from its origin position, and
  something has to decide the interleaving.
- Idempotent ingest. A peer will see the same event more than once; re-delivery
  must be harmless. Depends on the event identity settled in phase 3.
- **Whether replication is a port at all.** Nothing in this crate is a trait
  today, so there is no seam a second sync provider could be written against. If
  providers are meant to be pluggable — a Durable Object over a WebSocket, a
  hosted relay, snapshot exchange — then the *transport* and the *ingest policy*
  are probably two seams rather than one, and the ingest question above has to be
  settled before either can be stable. Note that the Cloudflare adapter (phase 5)
  is an **event store**, not a sync provider; it plugs into `EventStore`, which
  is already a port.

**Work**

- [ ] ADR for the above.
- [ ] Envelope types behind `happenstance/serde` — the envelope is serialised,
      the payload passes through untouched.
- [ ] Ingest, bound on `EventStore` rather than `SendEventStore`: the Cloudflare
      side is single-threaded ([constraint 4](#standing-constraints)).
- [ ] A round-trip test between two stores — ideally a native SQLite store and
      the phase 5 adapter, at minimum two independent SQLite stores — asserting
      that both converge and that replaying the same batch twice changes
      nothing.

**Exit criteria**

- [ ] Round-trip test green, including the duplicate-delivery case.
- [ ] `publish = false` removed.
- [ ] `cargo xtask ci` green.

**Session log**

---

## Release hazard

Event identity across instances may require a field on `Event` or a reserved
metadata key. That is a change to `happenstance`'s **public API**. Publishing 0.1
before the question is answered means either a breaking 0.2 or a replication
design bent to fit a type that was already published.

Two acceptable resolutions — choose one explicitly rather than letting the
default happen:

- **(a) Answer the identity question in phase 3 and publish with it.** It is one
  decision, it is cheap now, and `#[non_exhaustive]` is already on the public
  types precisely to leave room for it. **Recommended, and what phase 3 above
  assumes.**
- **(b) Publish 0.1 knowingly as a pre-sync release** and accept that 0.2 is
  breaking.

**Resolved: (a)** — but the framing above is wrong and is kept only so the
correction is legible. Nothing is published, so changing the public API costs
nobody anything, and "breaking 0.2" is not a hazard that exists yet. The real
reason to answer identity before publishing is simpler: **an event store whose
events have no identity is not finished.** Publication is not the forcing
function; completeness is.

The general form of that error is worth watching for. Treating an unpublished
API as something to protect will quietly bias every design decision toward the
option that changes least, which is not the same as the option that is best. See
the provisional markers on [ADR-0001](adr/0001-async-port-flavours.md),
[ADR-0003](adr/0003-opaque-payloads.md) and
[ADR-0004](adr/0004-edition-and-msrv.md), and
[ADR-0006](adr/0006-bare-name-to-the-typed-layer.md) for a worked example of the
same bias in the naming decision.

---

## Phase 7 — Publish 0.1

**Goal.** `happenstance` and `happenstance-testkit` on crates.io, rendering
correctly on docs.rs, with the adapters that have earned it alongside them.

**Why here.** Publishing freezes the public API, so it comes after the two
phases that change it: the projection port freeze (2) and event identity (3).

**Work**

- [ ] `crates/happenstance/README.md` and a `readme` key in its `Cargo.toml`.
      The manifest currently carries a comment saying this is missing and needed
      before first publish — it is what crates.io renders on the package page.
- [ ] The same for `happenstance-testkit`.
- [ ] `CHANGELOG.md`.
- [ ] `cargo doc --all-features` locally, checking the `docs.rs` metadata and
      the `docsrs` feature-badge cfg actually render.
- [ ] Set the workspace lint `clippy::todo` back to `warn` — it is `allow` only
      because the stubs were intentionally unimplemented. A clean build with it
      denied is the proof that no stub survives.
- [ ] Remove `publish = false` from every crate that has passed its suite.
- [ ] `cargo publish --dry-run` per crate.
- [ ] Publish in dependency order: `happenstance` → `happenstance-testkit` →
      adapters. Each must be live on crates.io before the next resolves against
      it.
- [ ] Tag and cut the GitHub release.

**Exit criteria**

- [ ] Crates live, docs.rs builds green.
- [ ] The `cargo-semver-checks` PR job now compares against a real published
      baseline — verify on the next pull request that it does something.
- [ ] README status table updated: no more 🔲.

**Session log**

---

## Standing constraints

These come from [`docs/adr/`](adr/) and are restated here because phases 3
through 6 write the most new adapter code and are the likeliest to trip over
them. Full reasoning is in [CLAUDE.md](../CLAUDE.md) and the ADRs.

1. **Never introduce `#[async_trait]`.** It injects `+ Send`, which makes the
   `wasm32` target impossible. ([ADR-0001](adr/0001-async-port-flavours.md))
2. **Never put `serde` in `happenstance`'s default features.** Payloads are
   opaque; the `serde` feature covers envelope types only.
   ([ADR-0003](adr/0003-opaque-payloads.md))
3. **`EventStore::read` returns the stream at the top level and is not
   `async`.** Nesting it in a future silently drops `+ Send` from the stream on
   the `Send` flavour, defeating the whole two-trait design. A unit test asserts
   this. ([ADR-0001](adr/0001-async-port-flavours.md))
4. **Bind `EventStore`, not `SendEventStore`, in generic code** — the weaker
   requirement accepts both. Import one of the two names per module; both in
   scope makes method calls ambiguous.
5. **No let-chains.** Stable only from 1.88; the MSRV is 1.85.
   ([ADR-0004](adr/0004-edition-and-msrv.md))

And the rule that outranks the rest: **an adapter that has not run the
conformance suite is not an adapter.** If a rule looks wrong, fix the rule and
say why in the same change — do not skip it.
