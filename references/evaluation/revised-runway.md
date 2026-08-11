# Runbook (revised)

The plan for finishing happenstance, and the record of how far it has got.

This replaces `RUNBOOK.md`. It is not an edit of it — the ordering changed,
and the thing each phase has to produce changed with it.

## What changed, and why

The old plan sequenced by **artefact**: build the SQLite store, then the
projection suite, then the typed layer, then the exotic adapters, then publish.
Every phase's exit gate was "the conformance suite is green" or "`cargo xtask ci`
is green".

That order optimises for having something to show. It is the wrong order for a
library whose entire value proposition is a contract, because it settles the
contract's shape **last**, in the phase that is most expensive to revise, and it
accepts a green gate as evidence that a *design* is right when the gate only
tests the code that exists.

This plan sequences by **blast radius**: the decision that shapes the most
downstream code is settled first, and it is settled against something that
compiles rather than against an argument. Three consequences follow.

**1. A port is proven by what can be written against it, and that proof costs a
skeleton, not an adapter.** A `rusqlite` skeleton that declares its real
`Batch` type, its real error enum and its real stream type — and `todo!()`s
every body — proves or refutes the projection port's GAT in an afternoon. The
old plan bought the same information for the price of two phases (1 and 2) and
then called the port "frozen" on the evidence of a single adapter. Phase 2 below
is nothing but skeletons, and it exists so that phases 3–5 are decided on
evidence.

**2. "The gate is green" is never a phase's proof.** Every phase below names a
specific artefact that constitutes proof — an object that did not exist before,
whose existence makes a provisional ADR into precedent. Where a phase's proof is
a test, the test must be one that *fails* against a plausible wrong
implementation; where it is a design, the proof is a second, structurally
different implementation of the same interface.

**3. Publication is decoupled from replication.** The old plan made phase 7
depend on phase 6 (`happenstance-sync`). Once event identity is a field on
`SequencedEvent` (phase 4), nothing in replication touches the contract's public
surface, so nothing about sync needs to precede a release. Removing that
dependency takes roughly three weeks off the critical path and costs nothing.

### The bias this plan is written against

The old runbook diagnosed it correctly at its own lines 414–421 and then did not
act on it: *treating an unpublished API as something to protect biases every
decision toward the option that changes least.* The symptom is a plan where
publication sits behind five phases and no version reaches anyone but the author
before the API is declared frozen. The correction is not to publish sooner for
its own sake — it is to stop using "publishing freezes the API" as the reason
for an ordering, and to use "which decision, if wrong, costs the most to undo"
instead. Every reordering below has that as its one-line justification.

## How to use this

1. **The runbook is updated in the same commit as the work it describes.**
2. **A phase is done when its proof artefact exists in the repository and every
   exit criterion is ticked.** `cargo xtask ci` being green is a precondition
   for looking at the exit criteria, not one of them.
3. **Decisions are settled in ADRs, written *before* the code they constrain.**
   The [ADR queue](#adr-queue) lists them in the order they must be written and
   the one question each answers.
4. **A `decided` ledger row is not settled.** It records an agreed answer with
   no ADR yet. If a phase body and a ledger row disagree, the ledger is newer;
   fix the phase body in the same commit.

## Session protocol

```
1. Read the status table, then `git log --oneline -10`.
2. Run `cargo xtask ci`. Establish the baseline is green before touching anything.
3. Pick the first phase that is not `done` and whose dependencies are `done`.
4. Write the phase's ADRs first. Then the code they constrain.
5. Build the phase's proof artefact. If you cannot, the phase is not done —
   say so in the session log rather than ticking the box.
6. Re-run the gate. Tick the exit criteria. Add a dated session-log line.
7. Commit the code and this file together.
```

---

## Status

| # | Phase | Was | Depends on | State | Proof artefact |
|---|---|---|---|---|---|
| 0 | [Ground clear](#phase-0--ground-clear) | 0 | — | not started | a publishable `.crate` and the reserved names |
| 1 | [The `!Send` proof](#phase-1--the-send-proof-and-the-trait-derivation-decision) | 5 | 0 | not started | conformance green on `wasm32` against a `!Send` store |
| 2 | [Adapter skeletons](#phase-2--adapter-skeletons-five-shapes-no-behaviour) | — (new) | 1 | not started | five compiling skeletons + a capability table |
| 3 | [Freeze `EventStore`](#phase-3--freeze-the-eventstore-port) | 1 (part) | 2 | not started | seven injected-bug stores the suite catches |
| 4 | [Freeze the value types](#phase-4--freeze-the-value-types-event-identity-and-the-wire-format) | 3 (part) | 2 | not started | a two-format wire round-trip proptest |
| 5 | [Freeze `ProjectionStore`](#phase-5--freeze-the-projectionstore-port) | 2 | 3, 4 | not started | two unlike stores passing one suite |
| 6 | [Typed layer + example](#phase-6--the-typed-layer-and-the-worked-example) | 3 | 3, 4, 5 | not started | a `trybuild` compile-fail proving fold/query agreement |
| — | **`0.1.0-alpha.1`** | — | 6 | — | — |
| 7 | [`happenstance-sqlite`](#phase-7--happenstance-sqlite) | 1, 2 | 3, 4, 5 | not started | concurrency suite green + a measured probe |
| 8 | [Cloudflare Durable Object](#phase-8--cloudflare-durable-object-adapter) | 5 | 2, 3 | not started | the suite green under `workerd` |
| 8b | [`happenstance-postgres`](#phase-8b--happenstance-postgres) | — (new) | 3, 4, 5 | not started | the suite green on a store that does *not* serialise writers |
| 9 | [Ladybug projection store](#phase-9--ladybug-projection-store) | 4 | 5 | not started | the projection suite green on a third shape |
| 10 | [**Publish `0.1.0`**](#phase-10--publish-010) | 7 | 6, 7 | not started | docs.rs green, `cargo-semver-checks` live |
| 11 | [`happenstance-sync`](#phase-11--happenstance-sync-the-third-port) | 6 | 7, 8, 8b, 10 | not started | one suite green against two unlike peers + a byte-identical round trip |

Phases moved, one line each:

- **`!Send` proof 5 → 1.** ADR-0001 shapes every signature in the workspace, has
  never been executed by a `!Send` implementer, and its cheapest proof is four
  days of work — discovering it fifth means four phases of code written against
  a design that may not hold.
- **Skeletons — new phase 2.** The information the old phases 1 and 2 were
  spending weeks to buy is available for the cost of three files that compile.
- **`EventStore` freeze 1 (part) → 3.** The port's signatures and semantics were
  never a phase; they were implicit in "write the SQLite adapter", which is the
  order that discovers an API defect after an adapter depends on it.
- **Value types and identity 3 (part) → 4.** `recorded_at` and `EventId` are
  columns in the first SQLite migration; they cannot be decided after the schema
  they belong in.
- **Projection freeze 2 → 5.** Freezing a port against one adapter shapes the
  port like that adapter; phase 5 requires two structurally different ones and
  gets the second from a skeleton rather than from Ladybug.
- **Typed layer 3 → 6, and before the finished SQLite adapter.** It is the
  consumer that discovers contract defects, it needs only `MemoryEventStore` to
  do so, and the old order bought that discovery *after* the adapter it would
  force a revision of.
- **SQLite 1 → 7.** It is the flagship, not the forcing function; it is where
  frozen decisions get spent, not where they get made.
- **Publish 7 → 10, and no longer depends on sync.** With identity settled in
  phase 4, replication touches nothing public.
- **Sync 6 → 11, after publish.** Its two open questions are internal to that
  crate; nothing about them constrains `happenstance-core`'s surface.

Two phases added after this revision, for one reason stated once:

- **Postgres — new phase 8b, and two extra skeletons in phase 2.** Every adapter
  on the roadmap serialises its writers and assigns positions under a lock it
  holds until commit — `MemoryEventStore`, the `RefCell` store, rusqlite and the
  Durable Object alike. They differ in async flavour, not in storage semantics.
  That means ADR-0010's position-visibility invariant would be frozen with
  nothing in the workspace able to violate it, which is the exact thing this
  plan's standing rule forbids of a conformance suite. Postgres is the cheapest
  adapter that breaks the monoculture, and it breaks it on the axis with the
  largest blast radius. It enters as an *instrument* in phase 2 and as an
  *adapter* in 8b, off the 0.1 path — see
  [Why Postgres and Neon, and why now](#why-postgres-and-neon-and-why-now).
- **Sync 11 becomes a port, not a protocol.** It was a single crate implementing
  one topology against one peer type. A local-first application that syncs to a
  Durable Object today and adds a second peer tomorrow needs a seam, and a seam
  with one implementation is shaped like that implementation — the failure this
  plan exists to prevent for `EventStore` and `ProjectionStore`, unnoticed for
  the third port because nobody had called it a port.

---

## ADR queue

In the order they must be written. Each line is the single question that ADR
answers; if an ADR cannot be stated as one question, it is two ADRs.

| ADR | Phase | The question it answers |
|---|---|---|
| **0007** | 1 | Is the `Send` flavour *derived* by `trait_variant` or hand-written — given that `SendEventStore::Error` must be `Send + Sync` while `EventStore::Error` must not, and `trait_variant` copies associated-type bounds verbatim? |
| **0008** | 3 | What does `EventStore::read` promise about laziness and isolation — when is the store's state sampled, and may a held stream observe a concurrent append? |
| **0009** | 3 | What shape does `append` take, and what are its preconditions — who owns the batch, what an empty batch is, and whether a batch's own events can violate its own condition? |
| **0010** | 3 | What does a store promise about position assignment and visibility — gaps, reuse across a reopen, and the visibility invariant that makes `AppendCondition::after` sound? |
| **0011** | 3 | What is the conformance suite's own proof obligation — what must it be demonstrated to *fail*, and how are runtime-agnostic, concurrent and durability rules expressed? |
| **0012** | 4 | What does an event carry beyond type, data and tags — identity, recorded-at, who assigns them, and what constructor shape survives adding a field? |
| **0013** | 4 | How is a validated identifier constructed — const or fallible — and is `Tag` equality byte equality? |
| **0014** | 4 | What is the contract crate's public dependency and wire-format surface — `bytes`, `futures-core`, the serde representation, and which formats the `serde` feature supports? |
| **0015** | 5 | What does a projection batch own, and what happens when it is dropped? |
| **0016** | 6 | How does a decision model guarantee that its query and its fold cannot disagree? |
| **0017** | 6 | How does a payload's shape evolve — codec tag, versioned event types, upcasting, and does the read path need a hook it does not have? |
| **0018** | 7 | SQLite: driver, schema, tag storage, and the append-condition strategy. |
| **0019** | 8 | Cloudflare: the `SqlStorage` mapping and the off-tokio conformance harness. |
| **0020** | 9 | Ladybug: checkpoint placement, how a projection expresses graph mutations, and the blocking API. |
| **0021** | 8b | Postgres: driver, schema, and — the one that is not a storage preference — *how* the adapter buys the position-visibility invariant when `BIGSERIAL` allocates outside the transaction, measured rather than argued. |
| **0022** | 11 | What is a sync *peer* — what ingest promises, what makes re-delivery harmless, and what the port may assume about a transport it cannot see? |
| **0023** | 11 | How do two logs reconcile — does ingest re-check append conditions, what is the merge rule, and is the topology peer-to-peer, hub-and-spoke, or both? |

Numbering note: 0021 was previously the single replication ADR. It is now the
Postgres one, because the queue is ordered by *when an ADR must be written* and
phase 8b precedes phase 11. The replication question split into 0022 (the port)
and 0023 (the policy) for the reason ADR queues exist: it could not be stated as
one question, so it is two.

Amendments to existing ADRs, scheduled:

- **ADR-0001** loses `provisional` in phase 1 (or is superseded by ADR-0007).
- **ADR-0003** loses `provisional` in phase 11, when a payload round-trips
  byte-identically between two stores.
- **ADR-0004** loses `provisional` at publish (phase 10), when the MSRV becomes
  a promise.
- **ADR-0006** gains an "On the historical record" section in phase 0, stating
  that ADR-0001/0003/0004 and CLAUDE.md's constraints are rewritten to
  `happenstance-core` because they were always statements about the ports crate.

---

## Decision ledger

Every question deliberately open, and the phase that owns the answer. Rows added
in this revision are marked ★ — they were open questions being settled by
accident because nothing recorded them.

| Decision | Phase | Status | ADR |
|---|---|---|---|
| ★ Derived vs hand-written `Send` flavour; `Sync` on the variant; `Error: Send + Sync` asymmetry | 1 | **open — highest blast radius in the workspace** | 0007 |
| ★ `read` laziness and read isolation | 3 | open — the contract and the reference store currently disagree | 0008 |
| ★ `append` ownership, empty batch, self-conflict | 3 | open | 0009 |
| ★ Position visibility invariant; reuse across reopen | 3 | open | 0010 |
| ★ Does `EventStore` grow `head` / `count` before publish, and as provided or required methods | 3 | **decided** — provided, overridable, pushdown-capable; blocked on 0007 | 0009 |
| ★ Suite's own proof obligation; concurrency and durability macros | 3 | open | 0011 |
| Event identity across instances | 4 | **decided** — Lamport pair `(origin, origin_position)` as `EventId` on `SequencedEvent`; store-assigned | 0012 |
| ★ Store-assigned `recorded_at` on `SequencedEvent` | 4 | open — must be settled with the schema, not after it | 0012 |
| ★ Const-constructible `EventType` / `Tag`; Unicode normalisation position | 4 | open | 0013 |
| ★ `bytes` / `futures-core` as permanent public dependencies; serde wire representation and supported formats | 4 | open | 0014 |
| Does the `ProjectionStore` port survive contact with a real transaction API | 5 | **decided — amended.** The borrowed GAT does *not* survive on the `Send` flavour; the batch becomes owned | 0015 |
| ★ Ship the projection port behind `unstable-projection` at 0.1, or freeze it | 5 | open | 0015 |
| Is `happenstance-runtime` the right name and seam | 0 | **decided** — executed in phase 0 | [0006](../../docs/adr/0006-bare-name-to-the-typed-layer.md) |
| ★ How the decision model's query and fold are kept in agreement | 6 | **decided** — the fold takes a decoded domain enum; the query derives from the same declaration | 0016 |
| ★ Is `happenstance-macros` in scope for 0.1 | 6 | open — criterion stated in phase 6 | 0016 |
| ★ Payload schema evolution: version suffix, upcasting, read-path hook | 6 | open — contract-shaping, so it cannot wait | 0017 |
| ★ Live subscription / catch-up tail primitive on `EventStore` | 8 | open — deferred; phase 6's runner polls. Revisit once two adapters exist | — |
| ★ Snapshotting decision-model state | post-0.1 | deferred — DCB queries are narrow by construction; revisit if replay cost is measured | — |
| ★ `tracing` spans and metrics: where, and default-on or opt-in | post-0.1 | deferred — purely additive, no port change | — |
| ★ Command-retry idempotency: is `AppendCondition` the documented pattern, or a metadata key | 6 | open — documentation, not a primitive | 0016 |
| ★ Benchmark harness in the testkit | 7 | **decided** — `event_store_benchmarks!`, so adapters inherit it | 0018 |
| SQLite driver, append-condition strategy, tag storage | 7 | **decided** — `rusqlite`; `BEGIN IMMEDIATE` + a probe returning the conflicting position; blob on `event` with `event_tag` as a derived index carrying `event_type` as a covering column | 0018 |
| Ladybug checkpoint placement; mutation expression; blocking API | 9 | open | 0020 |
| ★ Is the position model tested by any adapter capable of violating it | 2 | **decided** — no, and that is the defect. Postgres and Neon skeletons join phase 2 as instruments | 0010 |
| ★ Does `append` admit a store that cannot probe and write separately — is `conflicting_position` a promise or a hint | 3 | open — Neon-over-HTTP is the forcing case, and it wants the shape §2.2 of the implementability review rejected for SQLite | 0009 |
| ★ Is a Postgres or Neon adapter *shipped* in 0.1 | — | **decided** — no. Instruments before the freeze, adapters after; see [the scope note](#why-postgres-and-neon-and-why-now) | — |
| ★ How a Postgres adapter buys position visibility: `xid8` + `pg_snapshot_xmin`, advisory locks, or a serialised sequence table | 8b | open — deliberately. Each costs something real and the choice is a measurement, not an argument | 0021 |
| ★ Is sync a port with adapters, or one protocol | 11 | **decided** — a port. `happenstance-sync` holds it, `happenstance-sync-testkit` holds its suite, peers are sibling crates | 0022 |
| ★ Does the sync port live in `happenstance-core` | 11 | **decided** — no. Keeping it in `happenstance-sync` is what preserves this plan's decoupling of publication from replication | 0022 |
| ★ Does the sync port know the peer set | 11 | **decided** — no. `SyncPeer` describes one peer; a `SyncRunner` fans out, the same split as the projection runner and `ProjectionStore` | 0022 |
| Does sync ingest re-check append conditions | 11 | open — *the* central question of that crate | 0023 |
| Merge rule for two independently-ordered logs | 11 | open | 0023 |
| ★ Hub-and-spoke as a first-class topology beside peer-to-peer | 11 | open — both must be expressible; the crate currently documents only the second | 0023 |

---

## Why Postgres and Neon, and why now

This plan's method is that a port is proven by what can be written against it,
and that the proof costs a skeleton rather than an adapter. The method is only as
good as the *spread* of the skeletons. Here is the spread it had:

| Port | Adapters planned | Distinct shapes actually represented |
|---|---|---|
| `EventStore` | `MemoryEventStore`, `LocalMemoryEventStore` (`RefCell`), rusqlite, Durable Object `SqlStorage` | `Send`/`!Send`: **2**. Storage semantics: **1** — all four serialise writers and assign positions under a lock held until commit |
| `ProjectionStore` | `MemoryProjectionStore`, SQLite, Ladybug | **2** — a SQL transaction and a graph write handle, but both in-process, both local, neither networked |
| Sync | one protocol, one implied peer | **0 ports, 1 shape** |

A portfolio that is four adapters wide and one shape deep on the axis that
matters. Three things follow, and each is a decision this plan already schedules
and would otherwise settle on a single data point.

**1. Position visibility (phase 3, ADR-0010) — the one that matters.** Postgres
`nextval()` allocates outside the transaction, so a transaction that starts later
can commit earlier and an event can become visible at a position *below* one a
reader has already observed. A projection checkpointing on `ReadOptions::after`
skips those events: data loss, no error, no failing test. Phase 3 already plans
to write the invariant down. What it cannot do today is *test* it — every
adapter on the roadmap satisfies it for free, so the rule would ship having never
rejected anything. That is precisely what this plan's closing amendment forbids:
**a conformance suite that has not been shown to fail a wrong adapter is not a
suite.** The remedies each cost something real, which is why 8b owes a
measurement rather than an argument: `xid8` + `pg_snapshot_xmin` is two lines of
SQL but lets any long-running transaction anywhere stall every reader;
transaction-scoped advisory locks avoid the stall and add machinery; a serialised
sequence table dissolves the problem and throws away the write concurrency that
was the reason to reach for Postgres.

**2. The append shape (phase 3, ADR-0009).** The implementability review §2.2
*rejected* `INSERT … SELECT … WHERE NOT EXISTS` for SQLite, because it yields a
boolean and cannot produce `ConditionViolated::conflicting_position` without a
second probe. Neon's SQL-over-HTTP endpoint supports single statements and
non-interactive batched transactions only — no interactive transaction, so the
shape SQLite rejected is the only shape it can express. Two adapters want
opposite things from one signature. The port has to decide, before it freezes,
whether `conflicting_position` is a promise or a hint, and that question is
invisible to every adapter currently planned.

**3. Read laziness (phase 3, ADR-0008).** `store.rs:104-108` promises a
million-event replay streams without buffering; the implementability review
already measured that false on SQLite for the plan the docs steer toward. Over
Neon HTTP it is *structurally* impossible — one response body, capped at 64 MB.
Three unlike answers force ADR-0008 to describe a class of stores rather than
choose between two, which is the difference between a contract and a description
of the reference implementation.

**And one confirmation, recorded honestly.** `sqlx::Pool::begin()` yields
`Transaction<'static, Postgres>`, which owns its `PoolConnection` and is `Send`.
That *agrees* with phase 5's decision to drop the GAT lifetime and make `Batch`
owned, and it agrees from a networked pool rather than a local handle. Phase 2's
exit criterion demands a rejection per skeleton; this is the skeleton that
produced agreement instead, and the capability table should say so rather than
manufacture a failure.

### The scope this deliberately does not take

**Postgres does not ship in 0.1, and is not the flagship.** Two reasons, both
load-bearing.

The first is this plan's own logic: phase 7 exists because an adapter is where
frozen decisions get *spent*, not where they get made. All of the value above is
extracted before the freeze, by skeletons, for about two days. A finished
Postgres adapter buys durability that none of the feedback 0.1 wants depends on,
and costs ten days on a critical path built to settle the contract first.

The second is positioning. `review-blind-spots.md` §7 argues that
"the event sourcing library for local-first Rust applications that sync" is the
one position nobody else occupies, and that `disintegrate` owns DCB-on-Postgres
in Rust today. Leading with a Postgres adapter competes with disintegrate on its
own ground, from behind, using the phase budget that the actual differentiator
needs. Postgres earns its place here as an *instrument that keeps the contract
honest* — and the README should say that in as many words rather than let a
reader infer a server-side flagship.

---

## Phase 0 — Ground clear

**Goal.** Every mechanical, undesigned thing that gets more expensive with each
commit, done in one pass, before any design work starts.

**Why here.** Nothing in it needs a decision, and everything in it corrupts a
later diff if deferred. The rename in particular has a cost that rises
monotonically with every document phases 1–5 will produce.

**Work**

- [ ] **Execute the ADR-0006 rename**, in its own commit with nothing else in
      it. `happenstance` → `happenstance-core`; `happenstance-runtime` →
      `happenstance`, shipping day one as a five-line facade
      (`pub use happenstance_core::*;`) so `cargo add happenstance` is true
      throughout. Two places a mechanical rename goes wrong without failing:
      `xtask/src/main.rs:66-75` hard-codes `-p happenstance` in the wasm32 step
      (repoint to `happenstance-core`, or the gate silently starts checking the
      *typed* layer and constraint 1 goes unguarded behind a green gate), and
      `xtask/src/main.rs:121` selects that step as `&REQUIRED[3..4]` by index —
      select it by name. `.github/workflows/ci.yml:77`'s `package:` list needs
      the same repoint.
- [ ] **Reserve the names.** Publish `0.0.0` placeholders for `happenstance`,
      `happenstance-core`, `happenstance-testkit`, `happenstance-sqlite`,
      `happenstance-ladybug`, `happenstance-sync`, `happenstance-sync-testkit`,
      `happenstance-postgres`, `happenstance-neon`, and the eventual Cloudflare
      name. Both principal names are free *today* and both naming ADRs list
      losing them as the one risk that reopens them. An hour of work; the window
      closes silently. Note a `0.0.x` baseline is useless to
      `cargo-semver-checks` — 0.0.x versions are mutually incompatible in Cargo
      — so this does not substitute for the `--baseline-rev` fix below.
- [ ] **Rewrite the documents the rename inverts.** CLAUDE.md's "What this is",
      repository map, dependency rule and constraint 2 (which after the rename
      would forbid `serde` to the crate whose job is encoding, and permit it into
      the contract crate — the exact inversion of ADR-0003). ADR-0001/0003/0004
      bodies. RUNBOOK standing constraints. `happenstance-runtime/src/lib.rs:10`
      ("Even the crate name is provisional"). Add the "On the historical record"
      section to ADR-0006. Append to CLAUDE.md constraint 5: *the MSRV is a
      preference until first publish — weigh it, do not obey it* (ADR-0004
      already says so; the file a session actually loads does not).
- [ ] **Make the README compile.** `#![cfg_attr(doctest, doc = include_str!("../README.md"))]`
      in `happenstance-core/src/lib.rs`, then fix README.md:87-100 and :29-50
      with hidden `# #[tokio::main] async fn main() -> Result<(), Box<dyn Error>> {`
      wrappers. Re-fence the deliberate fragment as `rust,ignore`.
- [ ] **Make the package publishable.** Copy `LICENSE-MIT` and `LICENSE-APACHE`
      into each publishable crate directory (Cargo will not follow paths outside
      the package root, and Windows makes symlinks awkward); write a per-crate
      `README.md`; add `readme = "README.md"`; add `homepage` to
      `[workspace.package]`. Add `cargo package -p <crate> --list` to the gate,
      asserting the licence files and README are in the artifact — that check is
      the one that would have caught this, and `cargo publish --dry-run` does
      not warn.
- [ ] **Declare what you use.** `serde = ["dep:serde", "bytes/serde", "serde/alloc"]`
      — the feature currently compiles only because `bytes` happens to enable
      `serde/alloc` transitively.
- [ ] **Close the lint hole.** `clippy::todo = "deny"` at the workspace, with
      `#![allow(clippy::todo)]` in the stub crates that need it. Same reasoning
      for `unwrap_used = "deny"`, since the test modules already opt out locally.
- [ ] **Close the gate's blind spots.** Add `cargo doc -p happenstance-core
      --no-default-features --no-deps` (three unconditional intra-doc links to
      `MemoryEventStore` currently make the `no_std` configuration a hard error,
      and the powerset step runs `check`, not `doc`, so neither sees it). Add a
      nightly `RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc` step
      behind a toolchain probe. Widen the wasm32 step to the feature powerset
      against `wasm32-unknown-unknown`, keeping one mandatory plain `cargo check`
      so the constraint-1 guard cannot become skippable. Add `--locked`.
- [ ] **Repoint `cargo-semver-checks`** at `--baseline-rev ${{ github.event.pull_request.base.sha }}`
      so the job does something today, and add the sentence to CONTRIBUTING.md
      saying the registry baseline arrives at 0.1.
- [ ] Weekly `schedule:` job running only `cargo deny check advisories`.
- [ ] `CHANGELOG.md` with an `## [Unreleased]` section, started now rather than
      reconstructed from ten phases of history at publish time. ADR-0004 already
      promises MSRV bumps are "called out in the changelog", against a file that
      does not exist.
- [ ] Decide `.idea/` (gitignore) and `.mcp.json` (commit — it is project
      configuration a collaborator needs). Every phase's gate references a clean
      tree that is currently undefined.
- [ ] Soften README's "with batteries" tagline to what phase 10 will actually
      ship, or move the missing items into phase 6's scope explicitly. Pick one;
      the tagline currently outruns the roadmap.

**Proof artefact.** `cargo package -p happenstance-core --list` and
`cargo package -p happenstance --list` each show both licence files and a
README; `cargo test --doc -p happenstance-core` compiles the README's Quick
start; and crates.io shows both principal names owned. Three checkable facts,
none of which is "the gate is green".

**Exit criteria**

- [ ] All seven names owned on crates.io.
- [ ] The README's code blocks are compiled by CI.
- [ ] `cargo package --list` assertion is a gate step.
- [ ] No document in the repository names `happenstance` when it means the
      contract crate.
- [ ] `git status` clean; `cargo xtask ci` green with the four new steps.

**Blockers folded in.** `readme-quickstart-does-not-compile` (both),
`no-license-or-readme-in-published-package`,
`serde-feature-relies-on-bytes-to-enable-serde-alloc`,
`clippy-todo-allowed-workspace-wide`, `claude-md-names-wrong-crate-after-rename`,
`runbook-ledger-contradicts-phase-bodies` (this document is the fix),
`batteries-claim-outruns-roadmap` (tagline half). Non-blocking items pulled
forward because they are cheaper here: `execute-adr-0006-rename-now-with-a-facade`,
`crates-io-names-unreserved`, `no-name-reservation-phase`,
`adr-0006-inverts-two-written-constraints`, `doc-no-default-features-fails`,
`docsrs-cfg-never-compiled`, `wasm-step-only-checks-std`,
`semver-checks-has-no-baseline`, `contributing-claims-nonexistent-semver-baseline`,
`no-scheduled-advisory-run`, `publishing-readiness-inventory`,
`claude-md-constraint-5-drops-msrv-nuance`.

**Estimate.** Half a day, plus an hour for the reservations.

---

## Phase 1 — The `!Send` proof and the trait-derivation decision

**Goal.** Turn ADR-0001 from a recorded intention into precedent, and settle how
the second trait flavour is produced — because that one line decides whether the
port can ever gain a method, and whether generic code can use `?` into an
application's error type.

**Why here (was 5).** ADR-0001 shapes every signature in the workspace and says
so itself: *"no `!Send` implementation of these ports exists anywhere, not even
a reference one."* Its own stated lift condition — a `RefCell`-backed reference
store — costs four days. The old plan spent four phases writing code against it
first. This is the single most expensive discovery available in the plan and it
was scheduled fifth.

**Decisions to settle first — ADR-0007.**

Three facts, each reproduced by a reviewer, that interact and can only be
resolved together:

1. `trait_variant` does not rewrite default `async fn` bodies (E0728), so any
   provided method must be hand-desugared to `-> impl Future` with an
   `async move` block.
2. The desugared body captures `&self` across an await, so the `Send` flavour
   requires `Self: Sync` — meaning the declaration must be
   `#[trait_variant::make(SendEventStore: Send + Sync)]`. This costs nothing:
   every native adapter is `Sync` already because it lives behind `Arc`, and an
   impl written in the documented `async fn append(&self, ..)` style *already*
   requires `Self: Sync` today.
3. `SendEventStore::Error` should be `Send + Sync + 'static` — without it no
   generic helper can convert a store error into `anyhow`, `eyre` or
   `Box<dyn Error + Send + Sync>` without a viral bound at every call site. But
   `EventStore::Error` must **not** carry it, because a Cloudflare adapter's
   error wraps `JsValue`, which is `!Send`. `trait_variant` copies
   associated-type bounds verbatim and cannot differentiate them.

Fact 3 is the one that may force the decision. If the two traits must differ in
their associated-type bounds, they cannot be derived from one declaration, and
the choice is between hand-writing both (the `umadb-dcb` shape ADR-0001
rejected) and accepting the weaker bound. **Do not settle this by argument.**
Write both variants, compile both against the phase-2 skeletons, and record
which one a `!Send` Cloudflare-shaped error type and a `Send` rusqlite-shaped
error type can both satisfy.

**Work**

- [ ] Replace the hand-duplicated rule list in
      `crates/happenstance-testkit/src/lib.rs:93-129` with a **rule registry**
      using the callback ("x-macro") pattern: a `for_each_rule!($callback:path)`
      that hands the list to a named emitter. Two gotchas verified by a
      reviewer: the emitter must be `#[macro_export]`ed even when
      `#[doc(hidden)]`, because `macro_rules` items live in a flat crate-root
      textual namespace; and the callback must be `$crate::`-prefixed or it
      resolves in the caller's scope, which is the exact bug being fixed. Emit
      three flavours: `__emit_tokio`, `__emit_blocking`
      (`futures::executor::block_on` — consider making this the default so
      downstream adapters need no tokio dev-dependency), `__emit_wasm`
      (`#[wasm_bindgen_test]`). Do **not** reach for `libtest-mimic`: the case
      list is statically known and one `#[test]` per rule is what makes
      `cargo test <rule_name>` and IDE gutters work.
- [ ] `LocalMemoryEventStore` — `RefCell<Vec<SequencedEvent>>`, implementing
      the bare `EventStore` and nothing else. It is `!Send`, so it exercises the
      path no code in the workspace has ever exercised.
- [ ] A CI job running the full registry under `wasm-bindgen-test` on
      `wasm32-unknown-unknown` against that store. No Cloudflare, no Durable
      Object, no `workerd` — this proves ADR-0001's three actual assertions (a
      `!Send` type can implement `EventStore`; generic code binding the weak
      trait accepts it; the blanket impl creates no coherence conflict) plus
      that the rules run off tokio.
- [ ] A **generic** Send-composition compile test. The existing one
      (`memory.rs:328-340`) asserts the property on a concrete type, where
      auto-trait leakage makes it pass regardless of whether the design works.
      Write the bound at the definition:
      `fn spawns_from_generic<S: SendEventStore + Send + Sync + 'static>(s: Arc<S>) { tokio::spawn(async move { … }); }`
      so the obligation is discharged before monomorphisation. `trait-variant`
      0.1.3 shipped after a two-year gap; the expansion is not frozen, and if it
      changes, today's test stays green while every native user's
      `tokio::spawn` breaks.
- [ ] Whichever way ADR-0007 lands, prove a provided method is writable: add a
      throwaway `fn probe(&self) -> impl Future<Output = ()> { async move {} }`
      to the port, confirm an adapter that overrides it and one that does not
      both compile on both flavours, then delete it. The real provided methods
      land in phase 3.

**Proof artefact.** A green CI job named for what it proves — every conformance
rule passing against a `!Send` store on `wasm32-unknown-unknown` — plus a
committed compile test that a generic `SendEventStore`-bound function is
spawnable. Together they are the two halves of ADR-0001's claim, and neither
existed before.

**Exit criteria**

- [ ] ADR-0007 written, with the compiled evidence for the derived-vs-hand-written
      choice quoted in it.
- [ ] ADR-0001's `provisional` marker removed, or ADR-0001 superseded.
- [ ] The rule list exists in exactly one place.
- [ ] The wasm32 conformance job is in `ci.yml` and green.

**Blockers folded in.** `send-composition-untested`,
`send-flavour-cannot-gain-default-methods`, `error-assoc-type-lacks-send-sync`,
`wasm-proof-fifth-invalidates-earlier-phases`, and the *mechanism* half of
`no-extension-trait-strategy`. Non-blocking: `build-the-rule-registry-now-not-in-phase-5`,
`testkit-headline-doctests-are-inert` (the registry makes them compilable).

**Estimate.** 4 days.

---

## Phase 2 — Adapter skeletons: five shapes, no behaviour

**Goal.** Five in-tree adapters that compile against the ports on their real
targets and `todo!()` every body, with a table recording exactly which trait
flavour and which associated types each one can satisfy — plus a first sketch of
the third port, so that sync stops being the one seam with no skeleton behind it.

**Why here (new).** This is the phase that makes phases 3–5 decisions rather
than guesses. A port's shape is falsified by a *type*, not by a behaviour — the
projection GAT's failure is `E0515` and "`Transaction<'_>` is not `Send`", both
of which a skeleton surfaces in an afternoon. The old plan bought the same
information by finishing two adapters across two phases, and then froze the port
against the first of them.

**Work**

Each skeleton declares real types and stubs real bodies. The point is the type
checker, not the behaviour.

- [ ] **`happenstance-sqlite`** — `rusqlite::Connection` behind a `Mutex`, a
      real `SqliteEventStoreError` enum, a real `read` stream type (a chunked
      cursor or a `spawn_blocking`-backed one — note that because `read` is not
      `async`, `tokio::task::spawn_blocking` *panics* if called at `read` time
      outside a runtime, so the spawn must be deferred into `poll_next`;
      laziness stops being a nicety and becomes load-bearing), and a real
      `Batch` type for the projection store. Implement `SendEventStore` and
      `SendProjectionStore`. This is the *serialising, `Send`, native* shape.
- [ ] **`happenstance-cloudflare`** (name it; add it to the workspace) — a
      `RefCell`-backed shape standing in for `SqlStorage`, an error type that is
      `!Send` (wrap a `PhantomData<*mut u8>` if `worker` is not a dependency
      yet), implementing the bare `EventStore` only. This is the *`!Send`,
      wasm32* shape, and it is the one that decides ADR-0007's fact 3.
- [ ] **`happenstance-ladybug`** — a stand-in write-handle type with `lbug`'s
      `Send`/lifetime characteristics but no `lbug` dependency, implementing
      `ProjectionStore`. This is the *owned-handle, possibly-`!Send`, non-SQL*
      shape. Deferring the real dependency keeps the cold C++ build out of the
      gate until phase 9.
- [ ] **`happenstance-postgres`** — `sqlx` with the `postgres` feature behind a
      pool. A real `PostgresEventStoreError`, a real `read` stream over a pooled
      cursor, and `type Batch = sqlx::Transaction<'static, Postgres>` — which
      owns its `PoolConnection` and is therefore `Send`, unlike
      `rusqlite::Transaction<'a>`. Implements `SendEventStore` and
      `SendProjectionStore`. This is the *networked, pooled, non-serialising*
      shape, and it is the only one on the roadmap that can violate ADR-0010.
- [ ] **`happenstance-neon`** — one-shot HTTP against Neon's `/sql` endpoint.
      **No connection, no transaction handle, no cursor, one round trip per
      operation, and a 64 MB ceiling on a response.** Implements the bare
      `EventStore`, and compiles on both the host and `wasm32-unknown-unknown` —
      making it the only adapter that satisfies both flavours from one body of
      code, which is itself a data point about the two-trait design. Do **not**
      reach for `tokio-postgres` over Hyperdrive here: it works in a Rust Worker
      but needs a forked driver with unnamed-statement support and a hand-rolled
      binding. HTTP is the wasm32 Postgres story; Hyperdrive is a phase-8b
      deployment note.
- [ ] **A `SyncPeer` sketch** in `happenstance-sync`, plus a `MemorySyncPeer`
      that compiles. Not the protocol — just enough of a trait to learn whether a
      peer can be stated without naming a transport, given that the two real
      peers are a Durable Object reached over a socket and a Postgres reached
      over one-shot HTTP. If it cannot be, that is phase 11's most useful input
      and it is worth knowing nine phases early.
- [ ] Write the capability table into `references/adapter-shapes.md`: for each
      skeleton, which flavour it implements, what its `Error` is, what its
      `Batch` is, and — importantly — every signature it *could not* satisfy and
      the compiler error it produced. Give it a second kind of row for
      **capability** rejections: `happenstance-neon` will compile against
      signatures it cannot honour (an interactive `append` probe, a streaming
      `read`), and a table that records only `error[E….]` would show it as the
      most compatible adapter in the workspace when it is the least.

**Proof artefact.** `references/adapter-shapes.md`: five columns, one per skeleton,
listing the exact `error[E….]` each rejected signature produced and, for the
adapters whose limits are not type errors, the capabilities each cannot honour.
That document is the evidence base for ADR-0007 through ADR-0015, and it is the
difference between "the port survives contact with a real transaction API"
(which the old ledger asserted) and knowing which flavour it survives on, on how
many unlike shapes.

**Exit criteria**

- [ ] Five skeletons compile: three on the host target, the Cloudflare one on
      `wasm32-unknown-unknown`, and `happenstance-neon` on both.
- [ ] `references/adapter-shapes.md` records at least one rejection — a signature with
      its compiler error, or a capability with the reason — for each skeleton. A
      table with no rejections means the skeletons were not ambitious enough: go
      back and try the borrowed `Batch<'a> = rusqlite::Transaction<'a>` on
      `SendProjectionStore`, and an `append` that probes and writes in two
      statements on `happenstance-neon`, until they fail, and record how. The
      one skeleton permitted to record agreement instead is
      `happenstance-postgres` on `Batch`, and only because the agreement is
      itself the finding.
- [ ] A `SyncPeer` trait exists and `MemorySyncPeer` compiles against it, or the
      session log records why a transport-neutral peer could not be stated.
- [ ] `cargo xtask ci` green (with `clippy::todo` allowed per-crate in the
      skeletons only — now five crates, not three).

**Blockers folded in.** `projection-gat-unimplementable-for-rusqlite-send`
(evidence), `projection-batch-gat-uninhabitable-by-rusqlite` (evidence),
`projection-port-frozen-against-one-adapter-off-publish-path` (the structural
answer: the second shape now exists on the publish path, and it is free).
Newly folded in by this revision: the adapter-portfolio monoculture, and the
absence of any skeleton at all behind the sync seam.

**Estimate.** 5 days — 3 for the original three skeletons, 2 for the Postgres
and Neon ones and the `SyncPeer` sketch. That is the whole cost on the critical
path of everything in
[Why Postgres and Neon](#why-postgres-and-neon-and-why-now).

---

## Phase 3 — Freeze the `EventStore` port

**Goal.** Every signature and every semantic promise on `EventStore` settled,
each promise paired with a conformance rule, and the suite turned from a set of
27 examples into an instrument that can be shown to catch a wrong adapter.

**Why here (was implicit inside phase 1).** The port's shape was never a phase;
it was assumed to fall out of writing the SQLite adapter. That order discovers
an API defect after an adapter depends on it, which is the most expensive
possible sequence, and it is exactly what produced the twenty-odd signature
findings in this evaluation.

**Decisions — ADR-0008 (laziness/isolation), ADR-0009 (append), ADR-0010
(positions), ADR-0011 (the suite's own obligation).**

### 3a. Signatures

- [ ] **`append` takes the events by value.** `&[Event]` forces an owning
      adapter to clone — `Bytes` is a refcount bump but `EventType(Box<str>)` is
      one allocation and `Tags(Box<[Tag]>)` is n+1 — and it makes
      `Event::into_parts`, whose doc comment says it exists to avoid that clone,
      *unreachable through the port*. `memory.rs:219-221` does exactly the clone
      the method was written to prevent. Choose between
      `impl IntoIterator<Item = Event>` and `Vec<Event>` on the phase-2
      evidence, and weigh one specific cost: `impl IntoIterator` makes `append`
      a generic method, and `dynosaur` — which ADR-0001 names as the escape
      hatch for the missing `dyn EventStore` — cannot erase generic methods.
      Note this is not "every adapter must clone": a *serialising* adapter (the
      SQLite one) reads straight out of the borrow and clones nothing. The
      defect is narrower and still real.
- [ ] **`read` takes the query by value** (`Query`, or `Arc<Query>`). Under
      RPITIT the opaque return type captures every in-scope lifetime including
      the query's, so a caller cannot return a stream from a function, store one
      in a struct, or `tokio::spawn` a replay without keeping the `Query` alive.
      `Query` is a `Box<[QueryItem]>`; one clone per read is nothing beside the
      I/O, and every adapter translates it to SQL immediately.
- [ ] **`EventStoreExt`**, a blanket extension trait
      (`impl<S: EventStore + ?Sized> EventStoreExt for S {}`), the
      `Iterator`/`Itertools` and `Future`/`FutureExt` pattern. Everything
      derivable from `read`/`append` lives here and can be added later
      non-breakingly: `read_to_vec`, `fold_decision_model` (streaming, so a
      decision model over a hot tag never materialises), `decision_slice`.
- [ ] **Provided `head()` and `count(&Query)` on the port itself**, hand-desugared
      per ADR-0007, with draining default bodies. These must *not* go on the
      blanket ext trait, because a blanket impl cannot be overridden and a SQLite
      adapter wants `SELECT max(position)`, not a scan. `head` is already needed:
      the projection runner ADR-0006 moves into this crate has to answer "am I
      caught up?".
- [ ] `collect` → `try_collect`, matching `TryStreamExt::try_collect`'s exact
      prior art; `read_decision_model` returns a
      `#[non_exhaustive] struct DecisionSlice { events, last_position }` with
      `fn close(&self, query: Query) -> AppendCondition`, so the operation every
      caller performs next is written once in the library rather than once per
      application.
- [ ] `ReadOptions::limit → Option<usize>`; `limit(0)` returns nothing, matching
      SQL and every paging API. Today it silently means *unlimited*, so a paging
      loop writing `.limit(budget - fetched)` that reaches parity reads the whole
      log.
- [ ] `ReadOptions::after(SequencePosition)` (exclusive), so resuming a
      projection is `after_opt(checkpoint)` with no arithmetic. `from` is
      inclusive and `AppendCondition::after` is exclusive; the checkpoint recipe
      at `projection.rs:86-88` currently requires the reader to get that
      asymmetry right by hand, and getting it wrong re-applies the last event on
      every restart.
- [ ] `Query::Items` becomes non-constructible from outside the crate
      (`#[non_exhaustive]` on the enum *and* the variant, or a private field),
      so `Query::Items(Box::new([]))` — an `AppendCondition` that can never be
      violated — stops being reachable. Drop the `Default` derive. Make
      `Query::from_item` infallible, as its own doc already says it is. Add
      `Query::index_arms()` (the items × types cross product) with a paragraph
      on emitting one bare compound arm per arm.
- [ ] `SequencePosition::next()` uses `NonZeroU64::checked_add`, so the method
      whose sole purpose is signalling overflow can actually do it.
- [ ] `happenstance_core::prelude` exporting `EventStore` (not `SendEventStore`)
      and the common types, so the default import path cannot produce E0034;
      `pub use futures_core;` beside `pub use bytes;`, since `Stream` appears in
      `read`'s signature and every adapter is forced to name it.
- [ ] `ConditionViolated`'s `Display` interpolates the `conflicting_position`
      the store already populates, and names the remedy.

### 3b. Semantics — each of these is a documented sentence *and* a rule

- [ ] **Laziness and isolation (ADR-0008).** The trait says the stream is lazy
      and "nothing is executed until it is first polled"; `MemoryEventStore`
      snapshots under the lock at call time and documents that it does. The
      contract is over-promising on behalf of every adapter, and the difference
      is observable: `let s = store.read(..); store.append(..).await?;
      try_collect(s).await?` yields the pre-append set on one adapter and the
      post-append set on another. Pick one of *snapshot at call*, *snapshot at
      first poll*, or *explicitly unspecified plus a no-duplicates/no-reorder
      rule* — and note that DCB does not need snapshot reads, because the append
      condition re-checks, which makes the cheaper chunked-pagination plan
      legitimate. Decide it against **three** shapes, not two:
      snapshot-under-a-lock (`MemoryEventStore`), a chunked cursor (rusqlite),
      and a single response body that cannot stream at all and is capped at
      64 MB (`happenstance-neon`). The third is what forces the ADR to describe a
      class of stores rather than choose between the two that happen to exist.
- [ ] **Self-conflict (ADR-0009).** State that the condition is evaluated
      against the store *before* any event in the batch is written; a batch can
      never conflict with itself. The reference store answers this by ordering
      alone. An adapter that re-probes per row — what a trigger-maintained index
      or a conditional `INSERT` naturally produces — rejects *every* conditional
      append, because the standard DCB uniqueness shape has the condition query
      matching the very event being written. Rule:
      `condition_ignores_the_batchs_own_events`, with a failure message that
      names post-insert evaluation as the cause.
- [ ] **Empty batch (ADR-0009).** Either make it unrepresentable
      (`EventBatch(Vec<Event>)` with `From<Event>` and `TryFrom<Vec<Event>>`,
      deleting `AppendError::NoEvents` and the rule that tests it) or pin the
      precedence explicitly — emptiness first, before the lock is taken, because
      it is a pure precondition on the argument. Today `append(&[], Some(&c))`
      returns `NoEvents` or `ConditionViolated` depending on store contents, and
      a caller whose retry logic branches on `is_condition_violated()` loops
      forever against the reference store.
- [ ] **Position visibility (ADR-0010).** The invariant that makes `after` sound
      is stated nowhere: *once a reader has observed position P, no event may
      subsequently become visible at a position ≤ P.* SQLite with
      `BEGIN IMMEDIATE` is immune because it serialises writers, so the flagship
      adapter will not surface it — but `happenstance-sqlite/src/lib.rs:16`
      already names a Postgres path, where `BIGSERIAL` exhibits it exactly, and
      sync's whole job is merging independently-ordered logs. Add the obligation
      and the "an adapter that allocates positions before commit does NOT
      satisfy this" warning. **This is the rule with a skeleton behind it as of
      this revision** — `happenstance-postgres` is the adapter that can fail it,
      and rule (g) below is the store that proves the rule can. Without them the
      obligation ships having never rejected anything.
- [ ] **Can a store that cannot probe-then-write satisfy `append`? (ADR-0009.)**
      The implementability review §2.2 rejected
      `INSERT … SELECT … WHERE NOT EXISTS` for SQLite because it yields a boolean
      and cannot produce `ConditionViolated::conflicting_position` without a
      second probe. Neon-over-HTTP has no interactive transaction, so that shape
      is the *only* one it can express. Decide explicitly whether
      `conflicting_position` is a promise every adapter owes or a hint an adapter
      may omit, and say which in the field's own documentation — a caller writing
      a retry loop against it needs to know before it is frozen, not after.
- [ ] **`from` is a threshold, not an identity.** State that the read is bounded
      by `position >= from` (or `<=` backwards) and that `from` need not name an
      existing event. Under exact-seek semantics the projection resume recipe
      stalls silently on any gapped log while passing all 27 rules.

### 3c. The suite becomes an instrument (ADR-0011)

- [ ] **The injected-bug harness.** Seven deliberately-wrong stores in the
      testkit's own `tests/`, each asserted to fail at least one *named* rule:
      (a) `LIMIT` applied before the tag filter; (b) an OR-ed query returning a
      matching event twice; (c) `COUNT(*)+1` position allocation, observable
      only across a reopen; (d) probe-then-insert outside the transaction;
      (e) metadata `None` and `Some(empty)` conflated; (f) `from` handled as an
      index rather than a threshold; (g) **`PreCommitPositionStore`** — positions
      allocated before commit, so a slow writer's event becomes visible *below* a
      position a reader has already observed. Plus a control store that passes
      everything. A reviewer measured four of the first six passing the current
      suite.

      (g) is the one this revision adds, and it is the only one whose bug is a
      faithful model of a real database rather than an implementation slip:
      it is what Postgres does by default. It needs the concurrency macro below,
      so its marginal cost is the store itself. **If it passes the suite, the
      position-visibility obligation is decorative and phase 3 is not done** —
      that assertion is the phase's cheapest and highest-value single check.
- [ ] `read_limit_applies_after_filtering` and its backwards mirror — an
      interleaved sequence, a tag-constrained query, `limit(2)`, asserting the
      first two *matching* events. The existing `read_limit_truncates` uses
      `Query::all()`, so `SELECT … WHERE type IN (..) LIMIT 2` followed by
      in-memory tag filtering — the classic event-store bug — passes.
- [ ] Condition-query shapes: `Query::all()`, multi-item, and **tag-only** (an
      empty `types` list, the most likely `IN ()` generator). Every condition in
      the suite today is single-item with at least one type.
- [ ] `after` at or beyond the store's head, and beyond the last *matching*
      position while matching events exist — the clause the specification
      singles out with a Note and the one with zero coverage.
- [ ] **A fixture-shaped factory.** The macro takes an expression contractually
      required to produce a fresh, empty store, so no rule that needs a second
      handle can be written — which forecloses durability, reopen, and genuine
      multi-connection rules all at once. Change it to take a fixture that can
      hand out repeated connections to one underlying store, and gate the reopen
      rules on adapters that supply one. Do this before phase 7 writes the
      SQLite conformance file, not after.
- [ ] **`event_store_model_conformance!`** — a stateful model-based proptest,
      generating **symbolic** position anchors (`None|First|Middle|Head|BeyondHead`)
      resolved at execution against positions the store actually assigned, which
      is the generalisation of the no-literal-positions house rule and what lets
      one test run against dense and gapped stores alike. Build the model on
      `Query::matches` and `AppendCondition::is_violated_by` (not circular —
      those are already property-tested against naive definitions, and it makes
      the model ~30 lines). **Predict** each conditional append's outcome before
      calling and assert agreement in both directions. Hand-roll `Vec<Op>` with
      `prop::collection::vec`; do not add `proptest-state-machine`.
- [ ] **`event_store_concurrency_conformance!`** — opt-in, `S: SendEventStore +
      Send + Sync + 'static`, `#[tokio::test(flavor = "multi_thread")]`. The
      existing `racing_conditional_appends_elect_one_winner` is sequential *by
      construction and says so*; it cannot distinguish an atomic
      check-and-write from a probe followed by an insert. Assert: exactly one
      winner of N contenders; a capacity invariant (K seats, exactly K commit);
      positions unique after N unconditional appends; `append`'s return is the
      caller's own last event, not the global head; a concurrent reader never
      sees a partial batch; every task terminates within a timeout. Then correct
      `RUNBOOK` phase-1's claim that the sequential rule is "the one a naive
      read-then-write implementation fails" — it is not, and README.md:146 says
      the same thing more mildly.
- [ ] Merge `append_is_atomic` with `condition_rejection_leaves_store_unchanged`
      (they assert the same property) and add a real one using a fault-injecting
      decorator, since the existing rule triggers rejection via the condition
      check, which precedes any write.
- [ ] Value edges: empty payload; `metadata` `None` vs `Some(empty)`; an
      untagged event matching `Query::all()` (an `INNER JOIN event_tag` silently
      drops these); max-length tag and event type round trip;
      non-ASCII; 64 tags; 1 MiB payload; reading an empty store.

**Proof artefact.** The seven injected-bug stores, committed, each with an
`#[should_panic]`-style assertion naming the rule that catches it. That is what
turns "27 rules" into "the bar every adapter must clear" — a suite whose own
test suite demonstrates what it rejects. Secondary proof: the five phase-2
skeletons still compile against every changed signature, unmodified except for
the signatures themselves.

**Exit criteria**

- [ ] ADR-0008, 0009, 0010, 0011 written and merged before the code they
      constrain.
- [ ] Every semantic sentence added in 3b has a rule in 3c that fails without it.
- [ ] Seven injected-bug stores; each caught; the control passes.
- [ ] The model-based test and the concurrency macro run in CI against
      `MemoryEventStore`.
- [ ] The five skeletons compile.
- [ ] `conflicting_position` is documented as a promise or as a hint, and
      `happenstance-neon` is cited for why the question was asked.

**Blockers folded in.** `append-takes-slice-forces-clone-into-parts-unreachable`,
`append-takes-slice-forces-clone`, `append-takes-slice-so-into-parts-is-unreachable`,
`empty-batch-representable-and-precedence-undefined`,
`empty-batch-vs-condition-precedence`, `empty-batch-condition-ordering-unpinned`,
`self-conflict-unspecified`, `batch-self-conflict-unspecified`,
`read-laziness-unspecified-oracle-contradicts-docs`,
`read-laziness-unenforced-and-violated`,
`read-laziness-documented-false-in-oracle-untested`,
`read-laziness-and-isolation-unspecified`, `position-visibility-invariant-unstated`,
`position-uniqueness-untested-across-reopen`,
`read-opaque-type-captures-the-query-lifetime`, `readoptions-limit-zero-means-unlimited`,
`readoptions-from-inclusive-checkpoint-footgun`,
`query-items-variant-constructible-empty`, `query-default-is-read-everything`,
`query-from-item-documented-infallible-returns-result`,
`non-exhaustive-audit-query-and-memory-error`, `sequenceposition-next-saturates`,
`read-decision-model-returns-bare-tuple`, `collect-free-function-name`,
`futures-core-not-reexported`, `no-prelude-e0034-will-be-the-top-support-question`,
`condition-violated-message-not-actionable`, `conformance-suite-does-not-pin-read`,
`suite-passes-four-of-six-injected-adapter-bugs`,
`no-stateful-model-based-conformance-test`,
`no-parallel-conformance-and-the-stated-reason-is-wrong`,
`suite-cannot-fail-read-then-write`, `condition-queries-only-ever-single-item`,
`after-beyond-last-match-never-exercised`, `no-extension-trait-strategy`,
and the `append`/`NoEvents` half of `no-adr-for-contract-shaping-defaults`.

**Estimate.** 7 days. This is the largest phase and it should be, because it is
the one that determines whether every other phase is building on rock.

---

## Phase 4 — Freeze the value types, event identity, and the wire format

**Goal.** `Event`, `SequencedEvent`, `EventType`, `Tag`, `Tags` and the serde
representation settled, with identity and recorded-at on `SequencedEvent`
*before* the schema that must hold them is written.

**Why here (was inside phase 3, after two adapter phases).** `recorded_at` and
`EventId` are columns in migration 1 of every store. The old plan wrote the
SQLite schema in phase 1 and decided what an event carries in phase 3. Runs in
parallel with phase 3 for most of its length — the two touch different files —
but both must land before phase 5.

**Decisions — ADR-0012 (what an event carries), ADR-0013 (validated identifiers),
ADR-0014 (public dependencies and wire format).**

**Work**

- [ ] **`EventType` and `Tag` become `Cow<'static, str>`** with
      `pub const fn from_static(&'static str) -> Self`, validating with
      `assert!` in a const context. A reviewer compiled this on the pinned
      toolchain: `const COURSE_DEFINED: EventType = EventType::from_static("CourseDefined");`
      works, `from_static("")` is a compile error, and `Cow`'s
      `PartialEq`/`Ord`/`Hash` all delegate so `Borrowed("A") == Owned("A")` and
      both hash identically — every existing derive stays correct. Cost: 16 → 24
      bytes on a heap-indirected type. **State the trade in the ADR**: this buys
      compile-time validation and a zero-allocation hot path at the price of
      "an `EventType` value is always validated" becoming "always validated, but
      some of that validation happened at compile time". Rejected: a proc macro
      (validates at expansion, still allocates, adds a dependency to the contract
      crate); `&'static str` only (replication ingest needs runtime-derived
      types); `new_unchecked` (`forbid(unsafe_code)` makes it a footgun with no
      perf story). Note this wins less for `Tag`, whose *value* half is usually
      runtime data.
- [ ] **`Event::new` accepts an already-built `EventType`.** The bound
      `TryInto<EventType, Error = InvalidEventType>` excludes the infallible
      identity conversion, so a typed layer interning one `EventType` per
      `DomainEvent` must round-trip through `&str` and re-validate on every
      construction. The crate already knows the fix and applied it two files
      over: `QueryItem::new` (query.rs:57-61) uses
      `T: TryInto<EventType>, InvalidQuery: From<T::Error>` plus
      `impl From<Infallible> for InvalidQuery`, and its doc comment says that is
      exactly what it is for. Add `impl From<Infallible> for InvalidEventType`
      and the same bound. Add an infallible `Event::of(EventType, impl Into<Bytes>)`.
- [ ] **One umbrella validation error.** `InvalidTag`, `InvalidEventType`,
      `InvalidQuery` and `AppendError<E>` do not unify, so the only worked
      example needs `anyhow` — permitted for examples, but it demonstrates a
      style a library author cannot copy, and the library ships nothing to copy
      instead. Export `happenstance_core::Error` with `From` impls from all
      three.
- [ ] **Control characters and normalisation.** `char::is_control` is Unicode
      `Cc`, not ASCII; four sites say "ASCII" (tag.rs:46, event.rs:36,
      error.rs:29, error.rs:53). Fix the wording. Then take a position on
      `Cf` format characters — U+200B and U+202E can produce two visually
      identical tags that are two different consistency boundaries — and on
      normalisation: `"café"` in NFC and NFD are different `Tag`s, so a macOS
      client and a Linux client writing the "same" tag silently fail to
      conflict. Either normalise at construction (a `unicode-normalization`
      dependency, which breaks the `no_std` + `alloc` cleanliness and means the
      stored tag is no longer byte-identical to what the caller passed —
      breaking ADR-0003's byte-for-byte forwarding promise) or document that
      `Tag` equality is byte equality and normalisation is the caller's job.
      **Doing nothing and saying nothing is the only option that is definitely
      wrong.**
- [ ] **`SequencedEvent` gains `id` and `recorded_at`, and the constructor shape
      is decided with them.** `#[non_exhaustive]` does *not* make this free —
      it makes `SequencedEvent::new` the only way a downstream crate can
      construct one, so the attribute's whole mitigation is "no struct literal"
      and the constructor it forces everyone through is exactly what a field
      addition breaks. Add via a builder (`new(position, event).with_id(id)`) or
      a second constructor, not by widening `new`. Use a plain `u64` of
      milliseconds since the epoch for `recorded_at`, not `SystemTime` or a
      `chrono`/`jiff` dependency — the crate is `no_std`-capable and the store is
      the only thing that ever sets it. Make `into_parts` return a struct rather
      than a tuple, so its arity stops being public API. Add a rule that
      `recorded_at`, when present, is non-decreasing with position.
- [ ] **`ProjectionId`** — validate it the way `EventType` is validated
      (`InvalidProjectionId`, `TryFrom`, `FromStr`, `AsRef`, `Borrow<str>`,
      serde) or state in the docstring that it is a deliberately opaque
      operator-chosen key. It becomes the primary key of a checkpoint row, and
      the unexplained inconsistency with its two validating siblings teaches
      the reader that validation here is optional.
- [ ] **Delete every `skip_serializing_if`.** Five of them make `Event`,
      `SequencedEvent`, `QueryItem`, `Query` and `AppendCondition` produce bytes
      that cannot be deserialised in *any* non-self-describing format, for their
      most common shapes — because `skip_serializing_if` shortens the field
      count passed to `serialize_struct`, and a format with no field names feeds
      the deserializer exactly `FIELDS.len()` values positionally. Serialisation
      succeeds silently and produces plausible bytes. Postcard and bincode are
      exactly what a Worker-side wire would choose. Keep `#[serde(default)]` so
      existing JSON still parses; cost is ~10 bytes of JSON per event.
- [ ] **Tag `Query` explicitly on the wire.** `Query::All` currently serialises
      to JSON `null`, so the most destructive value in the protocol — the one an
      `AppendCondition` uses to reject any append at all — is the one a buggy
      peer produces by accident, and deserialising `{}` also yields it. Use
      `{"all": true}` / `{"items": [...]}` or a `"*"` sentinel, so the protocol
      fails closed. Separately, `Some(Query::All)` and `None` are currently
      indistinguishable in a self-describing format, so `Option<Query>` loses
      information; the tagged form fixes that too.
- [ ] Decide `Bytes`' human-readable representation: either document that the
      `serde` feature targets binary formats, or branch on
      `serializer.is_human_readable()` and emit base64. A 1 KiB payload is
      currently ~4 KiB of unreadable JSON on a path ADR-0003 justifies partly on
      efficiency grounds.
- [ ] `Borrow<str>` and `FromStr` for `Tag`/`EventType` (a
      `HashMap<EventType, Handler>` dispatch table is the core data structure of
      a codec registry, and without `Borrow` every lookup allocates); `Hash` on
      `Event`, `SequencedEvent`, `Query`, `QueryItem`, `AppendCondition` (phase
      11's dedup needs it); owned `IntoIterator for Tags`; `Extend<Tag>`
      re-canonicalising per call; `Tags::value_of(key)` as a `partition_point`
      on the `"key:"` prefix — O(log n) rather than the O(n) scan the worked
      example writes by hand twice, and the crate already pays for the sort.
      While adding `value_of`, decide whether repeated keys (`student:s1`,
      `student:s2` on one event) are legal; they are today, and `Option` would
      silently pick one.

**Proof artefact.** `crates/happenstance-core/tests/wire.rs`: a
`round_trip<T>(v: &T)` proptest asserting equality through **both** `serde_json`
and `postcard`, exercising the sparse shapes specifically — a bare `Event` with
no tags, a types-only `QueryItem`, a tags-only `QueryItem`, `Query::All`, an
`AppendCondition` with and without `after` — plus a direct test that
deliberately unsorted and duplicated tags come back canonical, and that an
over-length `Tag` is rejected on the way in. Two formats, because a
self-describing and a non-self-describing format break differently, and the
whole class of defect above is invisible to one of them.

Secondary proof, for the ergonomics half: a committed before/after count of `?`
and lines in the worked example's `define_course`. The claim is that six `?`
become two — the read and the append, which are the only two operations that
can genuinely fail at runtime. If the number does not move, `from_static` did
not earn its 8 bytes.

**Exit criteria**

- [ ] ADR-0012, 0013, 0014 written first.
- [ ] The wire proptest passes in `serde_json` and `postcard`.
- [ ] `SequencedEvent` carries `id` and `recorded_at`, and adding a *third*
      field would not break `new`.
- [ ] `EventType::from_static("")` is a compile error, demonstrated by a
      `trybuild` case.
- [ ] A stated position on Unicode normalisation exists in `Tag`'s docs.

**Blockers folded in.** `newtypes-cannot-be-const-constructed`,
`event-new-rejects-eventtype`, `event-new-rejects-prebuilt-eventtype`,
`char-is-control-is-unicode-cc-not-ascii`, `validation-question-tax-forces-anyhow`,
`projectionid-unvalidated-and-bare`,
`no-store-assigned-timestamp-schema-precedes-decision`,
`serde-skip-breaks-non-selfdescribing-roundtrip` (the one *critical*),
`serde-wire-format-hazards`, `option-query-collapses-in-serde`,
`serde-wire-format-has-no-tests`, `serde-feature-has-zero-tests`, and the
public-dependency half of `no-adr-for-contract-shaping-defaults`.

**Estimate.** 5 days, overlapping phase 3.

---

## Phase 5 — Freeze the `ProjectionStore` port

**Goal.** The port settled against **two structurally different**
implementations, its conformance suite written, and a reference implementation
shipped so the port has an oracle and a doctest that cannot rot.

**Why here (was 2, and was frozen against one adapter).** A port frozen by one
implementation is a port shaped like that implementation — which is the risk the
old phase 4 existed to catch, on a phase that was not a dependency of publish.
Phase 2's skeletons make the second shape free, and it is now on the publish
path.

**Decisions — ADR-0015.** Phase 2's `references/adapter-shapes.md` is the evidence.
What it will show, from a reviewer's compilation:
`type Batch<'a> = rusqlite::Transaction<'a>` compiles on the **bare** flavour
(store owns the `Connection`, `begin` uses `unchecked_transaction(&self)`) and
fails on `SendProjectionStore` for two independent reasons — `Connection` is
`Send` but not `Sync`, so `&Self` is not `Send`; and `Transaction<'_>` is not
`Send`, so the `commit`/`rollback` futures cannot be. The GAT's stated
justification at `projection.rs:80-82` is therefore unearned on the flavour
every native adapter will implement.

The Postgres skeleton supplies the confirming half, and it is worth stating
because a freeze decided only by what *fails* is a freeze decided by one family
of evidence. `sqlx::Pool::begin()` yields `Transaction<'static, Postgres>`: it
owns its `PoolConnection`, carries no borrow of the store, and is `Send`. The
owned `Batch` below is therefore not merely what rusqlite forces — it is also
what a networked pool produces naturally, from a driver that had every
opportunity to hand back a borrowed handle and did not.

**Work**

- [ ] **Drop the lifetime: `type Batch;`** An adapter buffers its read-model
      writes into an owned value and opens the transaction inside `commit`,
      alongside the checkpoint write — which preserves the one-transaction
      invariant the port exists for, satisfies `Send`, and removes the
      `where Self: 'a` noise from every impl for a lifetime no `Send`-capable
      design uses. This also makes the foreign-batch hazard unrepresentable: the
      elided lifetime in `batch: Self::Batch<'_>` is a fresh method-level
      lifetime unrelated to `&self`'s, so nothing today stops store A's
      transaction being committed through store B's connection — breaking the
      module's own headline invariant at runtime with no compiler complaint.
- [ ] **A stated Drop contract.** "Dropping a `Batch` without `commit`/`rollback`
      MUST roll back AND MUST release any resource `begin` acquired." A reviewer's
      probe found a dropped batch permanently losing the connection, after which
      the store returned `Busy` forever — so the rule is not just "rolls back",
      it is "and the store remains usable".
- [ ] **`MemoryProjectionStore`** in `happenstance-core` behind the `memory`
      feature. Same three-part rationale `memory.rs:16-23` gives for
      `MemoryEventStore`: an oracle for the suite, doctests that cannot rot, and
      something application authors can test against before any real adapter
      exists. It is ~50 lines. Its absence is why the port has a cold-start
      problem — implementing it currently fails with `E0195` unless you spell the
      parameter `Self::Batch<'_>`, and nothing says so and there is nothing to
      copy.
- [ ] `projection_store_conformance!`, emitted through the phase-1 registry so
      it inherits the tokio/blocking/wasm flavours. Rules: a fresh projection has
      no checkpoint; commit advances it; rollback leaves both unchanged; a
      dropped batch leaves both unchanged *and the store usable*; distinct
      `ProjectionId`s advance independently; a failed commit leaves the store
      unchanged.
- [ ] Move the projection runner into `happenstance-core` per ADR-0006, built on
      `head()` from phase 3 so it can answer "am I caught up?".
- [ ] **Decide the 0.1 exposure.** Either the port is genuinely frozen, or it
      ships behind an off-by-default `unstable-projection` feature with a doc
      paragraph exempting it from semver until a third adapter has cleared the
      suite — the `tokio_unstable` idiom. The second decouples publication from
      this phase entirely and is the honest option if the two shapes disagree.

**Proof artefact.** Two `SendProjectionStore` implementations passing the same
conformance suite, chosen to be as unlike each other as the workspace allows:
the phase-2 `rusqlite` skeleton fleshed out enough to commit a real transaction,
and `MemoryProjectionStore`. Plus the Ladybug **and Postgres** skeletons
compiling against the frozen shape without amendment — the second of those being
the only one whose transaction crosses a network. Four shapes, one suite; that
is what "frozen" has to mean.

**Exit criteria**

- [ ] ADR-0015 written, quoting the compiler errors from
      `references/adapter-shapes.md` rather than asserting the port "survives".
- [ ] Two implementations green against the projection suite.
- [ ] `projection.rs` no longer says "provisional", **or** the module is behind
      `unstable-projection` and says why.
- [ ] The Ladybug and Postgres skeletons compile unchanged.

**Blockers folded in.** `projection-gat-unimplementable-for-rusqlite-send`,
`projection-batch-gat-uninhabitable-by-rusqlite`,
`projection-commit-accepts-foreign-batch`,
`projection-port-frozen-against-one-adapter-off-publish-path`. Non-blocking:
`projection-batch-lifetime-e0195-undocumented`, `projectionid-unvalidated`,
`projection-port-could-ship-feature-gated`.

**Estimate.** 4 days, starting once 3 and 4 have both landed.

---

## Phase 6 — The typed layer and the worked example

**Goal.** `happenstance` — `DomainEvent`, `DecisionModel`, `Codec`, the command
loop, a testing DSL — and the worked example rewritten on top of it.

**Why here (was 3, after two adapter phases).** This is the consumer that
discovers contract defects, and it needs only `MemoryEventStore` to do so. The
old plan scheduled it after the SQLite adapter and the projection freeze, which
meant every defect it found forced a revision of code already written against
the old shape. ADR-0006 already removed the phase-2 dependency by moving the
projection runner into the contract crate; the dependency graph was never
updated.

**Decisions — ADR-0016 (fold/query agreement), ADR-0017 (payload evolution).**

**The hazard this phase exists to close.** A DCB handler names its event set
twice — in the `Query` it reads with, and in the `match` arms it folds with.
Add an event type to the fold and forget the query, and the decision is made on
state that excludes those events *and* the append condition fails to guard
against them. Both halves of DCB break from one omission, with no compiler,
clippy or conformance signal. It is undetectable by review because the omission
is an absence, and undetectable at runtime because the store behaves exactly as
instructed. `examples/course-subscriptions/src/main.rs:114-173` demonstrates it
today; it is the single largest correctness hazard in hand-rolled DCB and it is
currently one hundred percent on the user.

**The mechanism, stated as a design constraint rather than a macro-scheduling
decision.** Exhaustiveness comes from folding over a *decoded* enum, not from a
proc macro. If `DecisionModel::apply` takes `Self::Event` — a domain enum —
rather than a `SequencedEvent`, the `match` is exhaustive and the compiler
catches the divergence. The derive then only removes boilerplate from the
`EventType`/`Tags` mapping. So: **the fold must take a decoded type, and the
query must derive from the same declaration as the decode.** With that in place,
deferring `happenstance-macros` past 0.1 is defensible.

**Work**

- [ ] `DomainEvent` with `const EVENT_TYPES: &'static [EventType]` (const-
      constructible after phase 4), designed against a hand-written expansion of
      what the derive would emit, so the trait does not have to move when
      `happenstance-macros` lands.
- [ ] `DecisionModel` — `type Event: DomainEvent`, `fn apply(&mut self, Self::Event)`,
      `fn query(&self) -> Query` derived from `EVENT_TYPES` and the model's tag
      constraints, never maintained by hand.
- [ ] Composition for tuples `(P1, P2)`, `(P1, P2, P3)`, … by a small macro —
      the axum extractor-tuple trick — so N is compile-time and each model's
      query fragment is OR'd automatically. This is the Rust-idiomatic form of
      what `disintegrate` derives and what PHP's `CompositeProjection` does at
      runtime.
- [ ] `Codec` — JSON first, CBOR and postcard behind features. Events carry a
      codec tag so one store can hold more than one encoding, which is what makes
      a migration possible.
- [ ] The command loop: read → decide → append → retry on
      `ConditionViolated`. **State the retry policy in the docs, not just in the
      code** — including that after a violation you must re-read and re-decide,
      which is why consuming the batch (phase 3) makes the correct thing the easy
      thing.
- [ ] A given/when/then testing DSL: seed a `MemoryEventStore`, invoke the
      decision, assert on emitted events or the error. This is the ergonomic
      sugar that most directly produces the "delight" the project is aiming at,
      and it is cheap because the seeding target already exists.
- [ ] `FaultyStore<S>` in `happenstance-testkit` — fail the first N appends,
      fail every Nth, fail with an adapter error rather than a violation, fail
      reads — so a user's retry loop is testable at all. Pair with a
      `GappyMemoryStore` that deliberately leaves position gaps, so an
      application handler that assumes `position + 1` fails in a test rather than
      in production. CLAUDE.md already forbids the conformance suite from
      asserting literal positions for this reason; application authors deserve
      the same protection.
- [ ] One paragraph on command-retry idempotency — an HTTP client retrying a
      POST — showing `AppendCondition` and `Event::metadata` as the intended
      mechanism. The crate discusses idempotency twice (sync ingest,
      projection.rs:29) and neither time is about command retry.
- [ ] ADR-0017: payload evolution. Does `EventType` carry a version suffix? Does
      an upcaster need a read-path hook `EventStore` does not have? Both touch
      the contract's public surface, which is why this cannot wait until it is
      needed.
- [ ] Rewrite `examples/course-subscriptions`: typed events, a decision model,
      `const` event types, no `format!("…").into_bytes()` payloads
      (main.rs:99), no hand-rolled `parse_capacity` (main.rs:230-236).

**Proof artefact.** A `trybuild` compile-fail case: add a variant to the worked
example's domain event enum, and the crate fails to compile until the decision
model's fold handles it. That is the mechanised proof that the query/fold
divergence hazard is closed, and it is the single most important thing this
layer has to demonstrate. A test that passes proves nothing here; a test that
*must fail to compile* is the whole claim.

Secondary: the before/after diff of `examples/course-subscriptions` — line
count, `?` count, and whether any `&b"{}"[..]` remains.

**Exit criteria**

- [ ] ADR-0016 and ADR-0017 written first.
- [ ] The `trybuild` compile-fail case exists and is in the gate.
- [ ] `cargo run -p course-subscriptions` demonstrates the library an
      application would actually use.
- [ ] The stated criterion for `happenstance-macros` is evaluated in the
      session log: *if the rewritten example carries more mapping boilerplate
      than domain logic, the derive is in scope for 0.1.* Record the answer
      either way.
- [ ] `publish = false` removed from `happenstance`.

**Blockers folded in.** `query-and-fold-diverge-silently`,
`decision-model-composition-not-yet-designed`, `derive-macro-deferred-out-of-0-1`,
`typed-layer-scheduled-after-two-adapter-phases`, the scope half of
`batteries-claim-outruns-roadmap`, and the evolution half of
`no-performance-or-evolution-phase`. Non-blocking:
`no-application-level-testing-dsl-planned`, `typed-event-ergonomics-gap`,
`no-fault-injecting-store`, `command-side-idempotency-only-addressed-for-replication`,
`derive-macro-out-of-scope-without-a-criterion`, `read-decision-model-cannot-stream`.

**Estimate.** 8 days.

---

## Release: `0.1.0-alpha.1`, here

**Recommendation: publish `0.1.0-alpha.1` immediately after phase 6, and not
before.**

The two goals people conflate — reserving a name and gathering feedback — have
different deadlines and different instruments.

**Name reservation is urgent and is already handled in phase 0.** Registration
is first-come, both principal names are free today, and both naming ADRs list
losing one as the risk that reopens them. A `0.0.0` placeholder pointing at the
repository costs an hour and commits you to nothing. It buys nothing for
`cargo-semver-checks` — 0.0.x versions are mutually incompatible in Cargo — which
is why phase 0 also fixes the baseline separately.

**Feedback is only worth having about something a person can use.** Today the
installable surface is a contract crate plus an in-memory store: a reader can
review it but cannot build anything, so the feedback is feedback on a header
file. Between phases 3 and 5 the API changes daily *by design*, and an alpha
there generates issues about churn rather than about design. After phase 6 a
person can write a real domain model, test it with the given/when/then DSL, and
run it — against memory, with SQLite arriving as a drop-in. That is the first
version whose feedback is about the decisions you actually want challenged: the
decision-model composition, the command loop's retry policy, and whether the
typed layer's ergonomics clear the disintegrate bar.

**Waiting for SQLite (phase 7) costs two more weeks of not listening** for a
benefit — durability — that none of the feedback you want depends on. It also
front-loads the discovery that the typed layer's shape is wrong while phases
7–9 are still moving and cheap to redirect.

**The risk of an alpha being treated as a commitment is real and manageable.**
Cargo will not resolve a pre-release without an explicit pre-release requirement,
so nobody gets it by accident; the population that opts in is exactly the
population whose feedback is worth having. Mitigate it three ways, all cheap:

1. A `## Stability` section in the README naming the phase at which the API
   stops moving, and saying in one sentence that every alpha may break every
   other alpha.
2. A `CHANGELOG.md` that lists what broke between alphas. This is the artefact
   that converts "it churned" into "it churned for these stated reasons", and
   it is the difference between an alpha that builds trust and one that spends
   it.
3. Yank an alpha when the next one lands, so the resolvable set is always one
   version.

The failure mode to actually guard against is not users; it is **you**. An
alpha in the wild makes "we can't change that now" available as an argument, and
that argument is exactly the bias the old runbook diagnosed and did not act on.
Write the stability sentence for yourself as much as for anyone else.

**Do not publish a `0.1.0-alpha` before phase 3 lands.** The API is
deliberately unstable there and an alpha would be actively misleading.

---

## Phase 7 — `happenstance-sqlite`

**Goal.** The flagship adapter, complete: event store and projection store, both
conformance macros green, plus a benchmark harness.

**Why here (was 1 and 2).** It is where frozen decisions get spent, not where
they get made. Every one of its ADR-0018 decisions is a performance or storage
choice that presupposes a settled port; making them first is what put the
schema two phases ahead of the decision about what an event carries.

**Decisions — ADR-0018.** Already `decided` in the ledger; the ADR records them
with the schema. Two amendments the evaluation supplies:

- The sketch's `event_tag(tag, position)` carries no type, so a `QueryItem`'s
  type constraint becomes a join back to `event` — which collapses SQLite's
  `MERGE (UNION)` streaming plan into a co-routine over a temp b-tree, and makes
  the append-condition probe walk the full posting list through a join *while
  holding the `BEGIN IMMEDIATE` write lock*. That serialises every writer.
  Carry `event_type` as a covering payload column on `event_tag`, keeping the
  key `(tag, position)` so the range stays sorted by position:

  ```sql
  CREATE TABLE event_tag (
      tag        TEXT    NOT NULL,
      position   INTEGER NOT NULL,
      event_type TEXT    NOT NULL,   -- covering: type filter without a join
      PRIMARY KEY (tag, position)
  ) WITHOUT ROWID;
  CREATE INDEX event_type_idx ON event(event_type, position);
  CREATE TABLE tag_cardinality (tag TEXT PRIMARY KEY, n INTEGER NOT NULL) WITHOUT ROWID;
  ```

  The cardinality table is not decoration: multi-tag arms must be probed
  most-selective-tag-first, and SQLite cannot supply per-value cardinality —
  `ANALYZE` stores only an average, so `sqlite_stat1` for a large `event_tag` is
  a single row while actual values range over three orders of magnitude.
- Record `BEGIN IMMEDIATE` + a `min(position)` probe as the append strategy,
  because it yields `ConditionViolated::conflicting_position` for free, whereas
  `INSERT … SELECT … WHERE NOT EXISTS` yields only a boolean and cannot
  distinguish `ConditionViolated` from a caller error without a second probe.

**Work**

- [ ] ADR-0018, including the schema above and the identity/`recorded_at`
      columns from phase 4 in migration 1.
- [ ] `read` — the real stream. The `spawn_blocking` deferral from the phase-2
      skeleton is load-bearing: because `read` is not `async`,
      `tokio::task::spawn_blocking` panics if called at `read` time outside a
      runtime, so the spawn must happen in `poll_next`.
- [ ] `append` — condition evaluation and write in one transaction.
- [ ] `SqliteProjectionStore` against the phase-5 owned `Batch`.
- [ ] Handle `Query::index_arms()` exceeding SQLite's pushdown limits by chunking
      (prepare `ceil(arms/400)` statements and merge their cursors in Rust)
      rather than failing. `Query` bounds nothing by design — the limit is
      adapter-specific and a hard cap in the contract would be wrong.
- [ ] `event_store_benchmarks!(factory)` in the testkit behind a `bench`
      feature: append throughput, conditional append under contention, replay of
      N events with and without a tag filter. Adapters inherit it the way they
      inherit conformance, which no other Rust DCB library does.
- [ ] All four macros: conformance, model, concurrency, reopen.

**Proof artefact.** The concurrency macro green under a multi-thread runtime at
64 contenders across 25 rounds — because a read-then-write adapter fails that
within a handful of iterations and passes the sequential rule forever — plus a
committed benchmark number for the append-condition probe, since the ledger
already declared that a performance decision and a decision deferred to a
measurement that never happens is not a decision. Third: the phase-3
injected-bug harness re-run with `SqliteEventStore` as the control, showing it
in the pass column.

**Exit criteria**

- [ ] Four macros green.
- [ ] A benchmark number in the ADR, not a claim.
- [ ] No `todo!()` on any SQLite path.
- [ ] `publish = false` removed.

**Blockers folded in.** The benchmark half of
`no-performance-or-evolution-phase`. Non-blocking:
`schema-sketch-forces-the-slow-plan`, `no-index-arm-decomposition-on-query`,
`query-arm-count-vs-sqlite-compound-limit`, `count-query-not-expressible`,
`no-benchmarks-for-a-performance-claim`, `no-durability-rule-factory-shape-forbids-one`.

**Estimate.** 10 days. Runs in parallel with 8.

---

## Phase 8 — Cloudflare Durable Object adapter

**Goal.** The phase-2 skeleton finished against the real `SqlStorage` API,
passing the suite under `workerd`.

**Why here (was 5, but its *design* risk moved to phase 1).** Phase 1 already
proved a `!Send` store can implement the port and that the rules run off tokio,
so this phase is integration rather than design validation — which is what makes
it safe to run in parallel with phase 7 and safe to sit off the 0.1 critical
path.

**Work**

- [ ] ADR-0019: the `SqlStorage` mapping and the `workerd` harness
      (`vitest-pool-workers` as its own CI job).
- [ ] Finish the skeleton; run the registry's `__emit_wasm` flavour.
- [ ] Record whether the `!Send` `Error` bound asymmetry ADR-0007 predicted
      actually bites — this is the adapter that decides it.

**Proof artefact.** Every conformance rule green in a real Workers runtime, and
ADR-0001's `provisional` marker formally retired with this adapter cited. The
phase-1 `RefCell` store lifted the marker; this is the full proof ADR-0001 named.

**Estimate.** 8 days, parallel with 7.

---

## Phase 8b — `happenstance-postgres`

**Goal.** The phase-2 skeleton finished: event store and projection store against
a real Postgres, all four conformance macros green, and a *measured* answer to
the one question no other adapter in the workspace has to ask.

**Why here (new, and off the 0.1 path).** Its design contribution was already
spent in phases 2, 3 and 5 — the skeleton falsified the position model, the
capability table fed three ADRs, and `Transaction<'static, Postgres>` confirmed
the projection freeze. What is left is an adapter, and an adapter is where frozen
decisions get spent. It ships after 0.1 for the reason set out in
[the scope note](#the-scope-this-deliberately-does-not-take): ten days on the
critical path buys durability that none of the feedback 0.1 wants depends on,
and leading with Postgres competes with `disintegrate` on its own ground.

**Decisions — ADR-0021.** The driver and the schema are ordinary storage choices.
One decision is not, and it is the whole reason this phase exists:

**How does the adapter buy the position-visibility invariant?** `nextval()`
allocates outside the transaction, so without deliberate machinery a Postgres
store violates ADR-0010 by construction. Three strategies, each with a real cost,
and the ADR owes a number rather than a preference:

- **`xid8` + `pg_snapshot_xmin(pg_current_snapshot())`.** An extra column and a
  `WHERE transaction_id < pg_snapshot_xmin(…)` clause. Cheap to write; the
  failure mode is that *any* long-running transaction anywhere in the database
  becomes a ceiling, so one forgotten `BEGIN` in an unrelated application stalls
  every reader and every projection.
- **Transaction-scoped advisory locks.** Readers derive a safe watermark from
  `pg_locks`. No long-transaction stall, materially more machinery.
- **A serialised sequence table.** Dissolves the problem, and throws away the
  write concurrency that was the reason to reach for Postgres at all.

Measure the third under contention and the first under a deliberately held
transaction. A decision deferred to a measurement that never happens is not a
decision — the ledger already says so about the SQLite probe.

**Work**

- [ ] ADR-0021, with the visibility strategy chosen on numbers and the
      `recorded_at`/`EventId` columns from phase 4 in migration 1.
- [ ] The event store: `read` as a real pooled stream, `append` evaluating the
      condition and writing in one transaction.
- [ ] `PostgresProjectionStore` against the phase-5 owned `Batch`.
- [ ] All four macros: conformance, model, concurrency, reopen — plus the
      injected-bug harness re-run with `PostgresEventStore` as a control.
- [ ] `testcontainers` for the test fixture, pinned to a specific Postgres minor.
      **Keep it out of `cargo xtask ci`'s default path** and give it its own CI
      job: the gate must stay runnable without Docker, the same reasoning the
      repository already applies to Ladybug's C++ build.
- [ ] A deployment note for Neon — Hyperdrive plus a `worker::Socket`-backed
      driver on the Worker side, the HTTP endpoint where TCP is unavailable, and
      branch-per-test as a CI fixture. A note, not a supported configuration.

**Proof artefact.** The concurrency macro green under a multi-thread runtime
against a store that **does not serialise its writers** — the first time any
adapter in the portfolio clears that bar, and the reason this phase exists. Plus
a committed number for the visibility strategy's cost, including its behaviour
with a long-running transaction held open on the same database.

**Exit criteria**

- [ ] Four macros green against a real Postgres.
- [ ] `PreCommitPositionStore`'s rule is the one this adapter had to work to
      pass, and ADR-0021 says what it cost.
- [ ] The default `cargo xtask ci` path still runs without Docker.
- [ ] No `todo!()` on any Postgres path; `publish = false` removed.

**Estimate.** 8 days, parallel, after 0.1.

---

## Phase 9 — Ladybug projection store

**Goal.** The third implementation of the projection port, against a transaction
API deliberately unlike SQLite's.

**Why here (was 4).** Its *structural* contribution — a non-SQL batch shape —
was pulled into phase 2 as a skeleton and consumed by phase 5's freeze, so the
remaining work is the adapter itself, which is genuinely off the 0.1 path.
Projections only, for the reason the old runbook gave and which still holds.

**Work**

- [ ] ADR-0020: checkpoint placement, mutation expression, `spawn_blocking` vs
      blocking-only.
- [ ] Add `lbug`; measure the cold build; give this crate its own CI job if the
      C++ build cost is material.
- [ ] Implement and run the projection suite.

**Proof artefact.** The projection suite green on a third shape, and a written
answer to whether phase 5's freeze needed amending. If it did, that is a
superseding ADR and a data point that the two-implementation freeze rule should
have been three.

**Estimate.** 6 days, parallel, may follow 0.1.

---

## Phase 10 — Publish `0.1.0`

**Goal.** `happenstance-core`, `happenstance`, `happenstance-testkit` and
`happenstance-sqlite` on crates.io, rendering on docs.rs.

**Why here (was 7, and depended on sync).** Publication no longer waits on
replication: with identity settled in phase 4, nothing in `happenstance-sync`
touches the contract's public surface. Removing that dependency takes roughly
three weeks off the path and costs nothing.

Note also what "why here" is *not*: it is not "publishing freezes the public
API". Publishing starts the feedback loop; the API was frozen in phases 3–5, on
evidence, which is what makes publishing safe. Phase 2's freeze language should
read *stabilise*, not *freeze*, for the same reason.

**Work**

Most of the old phase 7's list moved to phase 0, where it was cheaper. What
remains is the release itself:

- [ ] `CHANGELOG.md` finalised for 0.1.0 (it has been accumulating since phase 0).
- [ ] `cargo publish --dry-run` per crate; verify each `.crate` against the
      phase-0 `--list` assertion.
- [ ] Publish in dependency order: `happenstance-core` → `happenstance-testkit`
      → `happenstance` → `happenstance-sqlite`.
- [ ] Tag; cut the GitHub release.
- [ ] ADR-0004 loses `provisional`; the MSRV becomes a promise.
- [ ] Repoint `cargo-semver-checks` to keep *both* baselines — registry for
      release safety, `--baseline-rev` for review signal.
- [ ] README status table: no more 🔲 for what shipped.

**Proof artefact.** docs.rs green for every published crate under
`--all-features` *and* the `docsrs` cfg (phase 0's nightly job already proves the
second, which is the one that cannot be fixed after the fact because a crates.io
release cannot be edited), plus a `cargo-semver-checks` run on the next pull
request that demonstrably reports something.

**Estimate.** 2 days.

---

## Phase 11 — `happenstance-sync`, the third port

**Goal.** Replication between happenstance instances, expressed as a **port with
adapters** rather than a protocol with one peer.

**Why here (was 6, before publish).** Unchanged: the hard questions are internal
to the crate, and the easy part is already paid for by ADR-0003 — a peer forwards
opaque bytes and cannot corrupt a payload it does not understand.

**What changed in this revision.** Sync was the one seam in the workspace treated
as an implementation rather than a port. The crate documents exactly one
topology — a device syncing with SQLite inside a Durable Object — and defines no
trait at all. But the deployment this library is *for* is a local-first
application that syncs to a Durable Object today and adds a second peer, or
swaps it for a Postgres, without touching a line of application code. That is a
port by definition, and this plan's whole thesis is that a port with one
implementation is shaped like that implementation. Every argument phases 2 and 5
make about `ProjectionStore` applies here verbatim; nobody had made it, because
nobody had called sync a port.

Two structural decisions follow, both settled:

- **The port lives in `happenstance-sync`, not `happenstance-core`.** Symmetry
  would argue for putting it beside `EventStore` and `ProjectionStore`. Doing so
  would put replication back on the publish path and undo the single largest
  saving this revision of the plan made. `happenstance-sync` becomes the port
  crate — the same relationship `happenstance-core` has to its adapters — with
  `happenstance-sync-testkit` holding the conformance suite and peers shipping as
  sibling crates. Third parties can still write a peer against a published port;
  it just publishes at 0.2 rather than blocking 0.1.
- **`SyncPeer` describes one peer; a `SyncRunner` fans out.** Multi-peer
  reconciliation, primary/secondary ordering and what to do when two peers
  disagree are *policy*, and policy on the port makes every adapter author
  inherit the merge problem and makes the conformance suite test a policy rather
  than a transport. This is the same split as the projection runner and
  `ProjectionStore`, and it makes "add Neon as a secondary" a runner
  configuration instead of a breaking change.

**Decisions — ADR-0022 (what a peer is), ADR-0023 (how logs reconcile).**

**Work**

- [ ] ADR-0022. What ingest promises, what makes re-delivery harmless, and what
      the port may assume about a transport it cannot see. The two real peers are
      a Durable Object over a socket and a Postgres over one-shot HTTP with no
      interactive transaction; a `SyncPeer` that cannot be implemented by the
      second is a `SyncPeer` shaped like the first. The phase-2 sketch is the
      evidence.
- [ ] ADR-0023. Does ingest re-check append conditions — *the* central question —
      the merge rule for two independently-ordered logs, and **hub-and-spoke as a
      first-class topology beside peer-to-peer.** Many devices syncing to one
      authoritative store is at least as common as two symmetric peers, and the
      crate's doc comment currently describes only the second while its own
      motivating deployment is arguably the first.
- [ ] `sync_peer_conformance!`, emitted through the phase-1 rule registry so it
      inherits the tokio/blocking/wasm flavours for free.
- [ ] `MemorySyncPeer` in `happenstance-sync` behind a `memory` feature — the
      oracle, the doctest target, and something an application author can test
      against before any real peer exists. The same three-part rationale
      `memory.rs:16-23` gives for `MemoryEventStore`, and the same reason the
      projection port had a cold-start problem without one.
- [ ] Envelope types behind `happenstance-core/serde`, on the wire format phase 4
      tested.
- [ ] Ingest bound on `EventStore`, not `SendEventStore` — the Cloudflare side is
      single-threaded.
- [ ] Two real peers: the phase-8 Durable Object and the phase-8b Postgres. Round
      trip between a native SQLite store and each.

**Proof artefact.** `sync_peer_conformance!` green against `MemorySyncPeer` and
**two structurally unlike networked peers** — a socket-reachable Durable Object
and a one-shot-HTTP Postgres — plus the round-trip test asserting the payload
`Bytes` are **byte-identical** end to end and that replaying the same batch twice
changes nothing. The byte-identity half is what lifts ADR-0003 from provisional.
The two-peer half is what makes this a port rather than a protocol: one peer
proves an implementation works, two prove the seam is in the right place.

**Exit criteria**

- [ ] ADR-0022 and ADR-0023 written before the code they constrain.
- [ ] `SyncPeer` implemented by three peers, one of which cannot hold a
      transaction open across a round trip.
- [ ] Hub-and-spoke and peer-to-peer are both expressible, and the crate's module
      doc no longer describes only one.
- [ ] The byte-identical round trip is green, and ADR-0003 loses `provisional`.

**Estimate.** 10 days, after 0.1 — 8 for the original scope, 2 for the port, the
suite and the memory peer.

---

## Critical path and parallelism

```
0 ──▶ 1 ──▶ 2 ──┬──▶ 3 ──┐
                │        ├──▶ 5 ──▶ 6 ──▶ [0.1.0-alpha.1] ──▶ 7 ──▶ 10 ──▶ 11
                └──▶ 4 ──┘                                     │
                                                    8 ─────────┤  (parallel)
                                                    8b ────────┘  (parallel, follows 10)
                                                    9 (parallel, may follow 10)
```

**Serial and unavoidable: 0 → 1 → 2 → 3 → 5 → 6 → 7 → 10.**

| | Phase | Days | Cumulative |
|---|---|---|---|
| 0 | Ground clear | 0.5 | 0.5 |
| 1 | `!Send` proof | 4 | 4.5 |
| 2 | Skeletons | 5 | 9.5 |
| 3 | Freeze `EventStore` | 7 | 16.5 |
| 4 | Freeze value types | 5 | *overlaps 3* |
| 5 | Freeze `ProjectionStore` | 4 | 20.5 |
| 6 | Typed layer | 8 | 28.5 |
| — | **alpha** | — | **≈6 weeks** |
| 7 | SQLite | 10 | 38.5 |
| 10 | Publish 0.1 | 2 | 40.5 |

**≈ 8 weeks of focused solo work to 0.1**, against roughly 11–12 for the old
ordering — and that is before counting the rework the old ordering builds in by
settling the contract after two adapters depend on it.

The two extra skeleton days are the entire cost of widening the portfolio on the
critical path. Everything else Postgres and Neon contribute — phase 8b, the sync
peers — sits after 0.1 and can slip freely.

**What can genuinely run in parallel:**

- **3 and 4** — different files (`store.rs`/`query.rs`/testkit vs
  `event.rs`/`tag.rs`/serde), one shared merge point at phase 5. This is the
  only overlap worth taking on a solo project; the rest below assume a second
  pair of hands.
- **8 (Cloudflare) with 7 (SQLite)** — both depend only on phases 2 and 3, and
  they touch disjoint crates. 8 is off the 0.1 path, so slipping it is free.
- **8b (Postgres) with 8 and 9** — its design contribution was spent in phases 2,
  3 and 5; what is left touches only its own crate.
- **9 (Ladybug) with anything after 5** — its skeleton already voted in phase 5;
  the adapter itself blocks nothing.
- **11 (sync) is strictly after 7, 8, 8b and 10** and cannot be parallelised
  usefully — it now needs *three* real stores, because proving a port takes two
  unlike peers and one oracle.

**What must not be parallelised:**

- **1 before 2.** The skeletons are written against whatever ADR-0007 decides.
  Writing them first means writing them twice.
- **2 before 3 and 5.** The whole point of the skeletons is to be the evidence.
  A freeze written before them is a freeze written on intention, which is what
  this plan exists to stop.
- **4 before 7.** `recorded_at` and `EventId` are columns in migration 1.
- **6 before 7.** The typed layer is the consumer that discovers contract
  defects; discovering them after the flagship adapter is written is precisely
  the sequence the old plan had.

---

## Standing constraints

Unchanged in substance; restated against the post-rename names.

1. **Never introduce `#[async_trait]`.** It injects `+ Send`, which makes the
   `wasm32` target impossible. (ADR-0001, ADR-0007)
2. **Never put `serde` in `happenstance-core`'s default features.** Payloads are
   opaque; the `serde` feature covers envelope types only. **This attaches to the
   ports crate, not to the string on the front of it** — after the rename,
   `happenstance` is the crate whose job *is* encoding and it depends on serde by
   design. (ADR-0003, ADR-0006)
3. **`EventStore::read` returns the stream at the top level and is not `async`.**
   Nesting it in a future silently drops `+ Send` from the stream on the `Send`
   flavour. A unit test asserts this. (ADR-0001)
4. **Bind `EventStore`, not `SendEventStore`, in generic code.** Import one of
   the two names per module, or prefer `happenstance_core::prelude` once phase 3
   ships it.
5. **No let-chains.** Stable from 1.88; the MSRV is 1.85 — but until first
   publish the MSRV is a preference, not a promise. Weigh it; do not obey it.
   (ADR-0004)

And the rule that outranks the rest: **an adapter that has not run the
conformance suite is not an adapter** — with two amendments. The first, from this
plan: **a conformance suite that has not been shown to fail a wrong adapter is
not a suite.** That is what phase 3's proof artefact is for.

The second, from this revision: **a port whose adapters all share a shape is a
port shaped like that shape.** Four adapters that differ in async flavour and
agree on storage semantics are one data point wearing four hats, and a freeze
decided on them is a freeze decided on one implementation. Before any port is
frozen, name the axis it is most likely to be wrong about and check that some
skeleton in the workspace sits on the other end of it. For `EventStore` that axis
is position assignment; the skeleton is `happenstance-postgres`. For `SyncPeer`
it is the transport; the skeleton is `happenstance-neon`.

---

## Index: every `blocksFirstPublish` finding, and the phase that owns it

| Phase | Findings |
|---|---|
| **0** | `readme-quickstart-does-not-compile` (×2) · `no-license-or-readme-in-published-package` · `serde-feature-relies-on-bytes-to-enable-serde-alloc` · `clippy-todo-allowed-workspace-wide` · `claude-md-names-wrong-crate-after-rename` · `runbook-ledger-contradicts-phase-bodies` · `batteries-claim-outruns-roadmap` (tagline) |
| **1** | `send-composition-untested` · `send-flavour-cannot-gain-default-methods` · `error-assoc-type-lacks-send-sync` · `wasm-proof-fifth-invalidates-earlier-phases` · `no-extension-trait-strategy` (mechanism) |
| **2** | `projection-gat-unimplementable-for-rusqlite-send` (evidence) · `projection-batch-gat-uninhabitable-by-rusqlite` (evidence) · `projection-port-frozen-against-one-adapter-off-publish-path` (structure) |
| **3** | `append-takes-slice-forces-clone-into-parts-unreachable` · `append-takes-slice-forces-clone` · `append-takes-slice-so-into-parts-is-unreachable` · `empty-batch-representable-and-precedence-undefined` · `empty-batch-vs-condition-precedence` · `empty-batch-condition-ordering-unpinned` · `self-conflict-unspecified` · `batch-self-conflict-unspecified` · `read-laziness-unspecified-oracle-contradicts-docs` · `read-laziness-unenforced-and-violated` · `read-laziness-documented-false-in-oracle-untested` · `read-laziness-and-isolation-unspecified` · `position-visibility-invariant-unstated` · `position-uniqueness-untested-across-reopen` · `read-opaque-type-captures-the-query-lifetime` · `readoptions-limit-zero-means-unlimited` · `readoptions-from-inclusive-checkpoint-footgun` · `query-items-variant-constructible-empty` · `query-default-is-read-everything` · `query-from-item-documented-infallible-returns-result` · `non-exhaustive-audit-query-and-memory-error` · `sequenceposition-next-saturates` · `read-decision-model-returns-bare-tuple` · `collect-free-function-name` · `futures-core-not-reexported` · `no-prelude-e0034-will-be-the-top-support-question` · `condition-violated-message-not-actionable` · `conformance-suite-does-not-pin-read` · `suite-passes-four-of-six-injected-adapter-bugs` · `no-stateful-model-based-conformance-test` · `no-parallel-conformance-and-the-stated-reason-is-wrong` · `suite-cannot-fail-read-then-write` · `condition-queries-only-ever-single-item` · `after-beyond-last-match-never-exercised` · `no-extension-trait-strategy` (the trait) · `no-adr-for-contract-shaping-defaults` (append/NoEvents) |
| **4** | `serde-skip-breaks-non-selfdescribing-roundtrip` **(critical)** · `newtypes-cannot-be-const-constructed` · `event-new-rejects-eventtype` · `event-new-rejects-prebuilt-eventtype` · `char-is-control-is-unicode-cc-not-ascii` · `validation-question-tax-forces-anyhow` · `projectionid-unvalidated-and-bare` · `no-store-assigned-timestamp-schema-precedes-decision` · `serde-wire-format-hazards` · `option-query-collapses-in-serde` · `serde-wire-format-has-no-tests` · `serde-feature-has-zero-tests` · `no-adr-for-contract-shaping-defaults` (public deps) |
| **5** | `projection-gat-unimplementable-for-rusqlite-send` (fix) · `projection-batch-gat-uninhabitable-by-rusqlite` (fix) · `projection-commit-accepts-foreign-batch` · `projection-port-frozen-against-one-adapter-off-publish-path` (fix) |
| **6** | `query-and-fold-diverge-silently` · `decision-model-composition-not-yet-designed` · `derive-macro-deferred-out-of-0-1` · `typed-layer-scheduled-after-two-adapter-phases` · `batteries-claim-outruns-roadmap` (scope) · `no-performance-or-evolution-phase` (evolution) |
| **7** | `no-performance-or-evolution-phase` (benchmarks) |

Nothing is orphaned. Every finding marked `blocksFirstPublish` lands in a phase
that precedes phase 10.
