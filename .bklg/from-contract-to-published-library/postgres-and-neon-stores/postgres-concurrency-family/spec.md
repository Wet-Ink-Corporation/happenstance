---
item: HS-S0063
stage: spec
created: 2026-08-12T13:47:02.142Z
updated: 2026-08-12T13:47:02.142Z
template_sig: 87bbf1d0
rendered_sig: c706b8d2
---

# Spec — The concurrency family green on unserialised writers

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DoD 5 at `:373` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project item | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/project.md` — AC-003 at `:234-236`, DR-2 at `:174-179` |
| Project briefs (architecture / testing / deployment) | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md` — architecture AC-003 row at `:59`, §6 at `:326-330`, §7 at `:333-352`; testing AC-003 row at `:513`, Notes §3 at `:557-565`, §7 at `:640-641` |
| Project grounding | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_grounding.md` |
| Signed-off design | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_design.md` — **no user-facing surface**; that determination is what was approved (`_design.md:10-41`, `:86-95`) |
| Story map (this story's row) | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_storymap.md:62`, expanded at `:110-113`, merge order at `:246-250` |
| This spec | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/postgres-concurrency-family/spec.md` |
| Roadmap / plan of record | `RUNBOOK.md:4368-4372` — phase 10's **Proof artefact**: "the concurrency macro green under a multi-thread runtime against a store that **does not serialise its writers** — the first time any adapter in the portfolio clears that bar" |

Traces to project **AC-003** (`project.md:234-236`) — sole owner
(`_storymap.md:219`). `depends_on`: **`postgres-append-and-frontier-head`**,
which supplies the `append` / `head` bodies these five rules are asked of;
transitively `postgres-schema-and-live-fixture`, which supplies migration 1, the
`testcontainers`-backed fixture and the live-Postgres CI job.

## One-line PR slice

`event_store_concurrency_conformance!` runs green at `CONTENDERS = 8` under a
multi-thread runtime — the first time any adapter in the portfolio clears that
bar on a store whose writers are not serialised.

## Executive summary

**Pointer.** The family already exists and is already proven capable of failing.
Five rules, one enumeration, two emitters and a `ConcurrentFixture` supertrait
ship in `crates/happenstance-testkit/src/concurrency.rs`; six racing stores in
`crates/happenstance-testkit/tests/mutation_coverage.rs:2524-2609` pin exactly
which rule each defect trips, and every one of their provenance paragraphs
describes a **Postgres** adapter shape. `MemoryEventStore` passes the family
today for a structural reason its own harness states plainly: it is an `RwLock`
around a `Vec`, so "this harness is the demonstration that the rules can be
passed, not the demonstration that they can fail"
(`crates/happenstance-testkit/tests/memory_concurrency_conformance.rs:10-16`).

**Delta this PR lands.** The same five rules, unmodified, invoked against a live
pinned Postgres from a new test target in `crates/happenstance-postgres/tests/`,
green at `CONTENDERS = 8`, inside the live-Postgres CI job and invisible to the
default gate. Three things have to become true for that to mean anything, and
each is a real piece of work rather than a wiring detail:

1. **Eight contenders must actually contend.** One fixture instance, eight
   handles, eight independent connection resources. A pool sized below
   `CONTENDERS` does not fail — it deadlocks, and the suite cannot tell the two
   apart because CF-33 forbids a watchdog (`concurrency.rs:198-206`, `:24-43`).
   A pool sized at 1 is worse: the family goes green while every writer has been
   serialised behind the pool, which is the failure this story exists to catch
   wearing a disguise nobody looks for.
2. **The adapter's futures must survive being driven off the runtime.** Each
   contender is a bare OS thread running the testkit's park-loop `block_on`,
   outside any ambient reactor. `concurrency.rs:61-68` names `sqlx` as the case
   this does not serve and predicts phase 10 will have to do something about it.
   This story is phase 10.
3. **Green must be shown not to have been bought by serialising writers.** The
   family cannot distinguish a store that overlaps its writers from one that
   does not — `LockedStore`, one mutex across the whole append, is its
   *conformant control* (`mutation_coverage.rs:2525-2532`). So this story owes a
   second, non-conformance instrument: evidence from the server that two appends
   were in flight at once.

**What it deliberately does not land.** No `append`, `head` or
`contains_event_id` body — those are `postgres-append-and-frontier-head`'s, and
this story consumes them. No choice of ES-10 mechanism and no throughput number:
ADR-0024 is `adr-0024-position-visibility-mechanism`'s and it depends on this
story, not the other way round (`_storymap.md:64`). No new conformance rule, no
edit to any existing one, no clause change.

## Context pack

The decisions this story must honour, stated as decisions. Read this section and
you can start; everything deeper is a signposted anchor below.

**1. The bar is a store whose writers are not serialised — not a store that
passes.** Every implementation that has ever passed this suite serialises its
writers and assigns positions under a lock held to commit: `MemoryEventStore`, a
`RefCell` store, the rusqlite skeleton, a Durable Object stand-in — one storage
shape wearing four hats (`project.md:44-49`). The mechanism that makes Postgres
pass trivially is the serialised sequence table, and the crate's own module doc
says what it costs: "an adapter that funnels all writes through a single lock is
`MemoryEventStore` with network latency, and freezing the port against it would
freeze it against the shape the workspace already has four of"
(`crates/happenstance-postgres/src/event_store.rs:43-53`). AC-003 makes retained
write concurrency an acceptance criterion rather than a side effect
(`project.md` risk table, `:335`). **A green family with a serialised writer path
does not satisfy this story.**

**2. This story adds no rule and edits none.** The five are frozen in place and
each is already cited by a clause it belongs to:
`exactly_one_of_n_contenders_commits` → **ES-25** (`spec/SPECIFICATION.md:3724`),
`a_concurrent_reader_never_sees_a_partial_batch` → **ES-18** (`:3341`),
`append_returns_the_callers_own_last_position` → **ES-19** (`:3452`),
`positions_are_unique_under_concurrent_appends` → **VT-11** (`:1030`). The fifth,
`k_disjoint_boundaries_admit_exactly_k_commits`, is cited by **no clause in
`SPECIFICATION.md`** — verified by search, not assumed. That is an observation to
**record**, not to fix here: minting or amending a clause is an ADR act
(`CLAUDE.md`), and the clause ledger belongs to `far-end-discharge-record`
(AC-013). Note it in this story's ledger and move on.

**3. The mutant table is why "green" is worth something, and it names the
Postgres mistakes by hand.** `RACERS`
(`crates/happenstance-testkit/tests/mutation_coverage.rs:2524-2609`) pins six
stores against these five rules, and the implementer should read the provenance
column as a list of ways to get this adapter wrong:

- `RacingProbeStore` — `SELECT 1 FROM events WHERE …` then `INSERT`, no `BEGIN`
  between and no `SERIALIZABLE` under. Fails `exactly_one_of_n_contenders_commits`
  and `k_disjoint_boundaries_admit_exactly_k_commits` by electing **too many**
  winners (`:2533-2554`).
- `GlobalVersionStore` — "a `SERIALIZABLE` adapter mapping `40001
  serialization_failure` onto `ConditionViolated`". Fails
  `k_disjoint_boundaries_admit_exactly_k_commits` from the **opposite** side: it
  elects *no* winner on boundaries it never touched (`:2555-2569`). This is the
  single most tempting shortcut available to a Postgres author reaching for
  isolation to buy the append condition, and the rule set catches it.
- `RacingSequenceStore` — `SELECT max(position)` before `BEGIN`. Fails
  `positions_are_unique_under_concurrent_appends` (`:2570-2583`).
- `GlobalHeadStore` — `INSERT …;` then `SELECT max(position)`. Fails
  `append_returns_the_callers_own_last_position` (`:2584-2594`).
- `RowAtATimeStore` — the per-row insert loop with the `BEGIN` forgotten. Fails
  the **live** half of `a_concurrent_reader_never_sees_a_partial_batch`
  (`:2595-2609`).

CLAUDE.md's "name a plausible wrong implementation it rejects" obligation is
therefore already discharged for this family, in-tree, by name. This story does
not owe a new mutant; it owes not becoming one.

**4. The contenders are OS threads, and that is the seam this story has to
close.** `race` spawns one `std::thread::scope` thread per contender behind a
`std::sync::Barrier` starting gate, and each rule body drives its future with
`crate::block_on` — a park loop, no runtime (`concurrency.rs:236-302`, `:347-371`,
`registry.rs:330-345`). The module states the consequence in advance: "A
contender drives its future to completion on a bare thread, outside any ambient
reactor. A store whose futures need one — `sqlx` is the case in this workspace —
cannot be driven that way, and phase 10's adapter will need the *spawn* to become
a parameter the harness supplies" (`concurrency.rs:61-68`). **Two routes are
open and the choice is this story's to make and record.**

- *Adapter/fixture side, preferred.* The emitter is spelled
  `#[tokio::test(flavor = "multi_thread")]` precisely so "an adapter whose own
  futures need a reactor under them" has one alive
  (`concurrency.rs:1059-1067`), and `connect()` is awaited **on** that runtime
  before the handles are moved onto threads. A handle can therefore capture the
  runtime and re-enter it around each `poll` — futures and the `read` stream
  alike. The hard constraint: **an `EnterGuard` may not be held across an
  `.await`**, because it is `!Send` and would take `PostgresEventStore`'s
  `SendEventStore` impl with it (`event_store.rs:118-121`, and the
  `store_is_send_and_sync` test at `:193-197`). Per-`poll` entry keeps the guard
  inside one poll and off the future's type.
- *Testkit side, fallback.* Make the spawn a harness parameter as
  `concurrency.rs:61-68` predicts. This is a change to
  `crates/happenstance-testkit/src/**`, which the architecture brief fences off
  ("must not change", `_decomposition.md:102-104`) with one caveat that is not
  this one. Taking this route is permitted but is a **recorded decision** in this
  story's ledger naming why the adapter-side route failed — not a quiet edit.

Whichever route is taken, it may not be `#[cfg]`-ing a rule out of the macro
expansion; DR-5 draws that line and it is the one hard constraint on gating
(`project.md:192-196`, `_decomposition.md:606-611`).

**5. One fixture instance, eight handles, and the pool is load-bearing.**
`CONTENDERS = 8` is "the number an adapter author has to size a connection pool
against: a fixture whose pool is smaller than this deadlocks rather than failing,
and the suite has no way to tell them apart (CF-33 — there is no watchdog)"
(`concurrency.rs:198-206`). `connect_many` simply calls `fixture.connect()` N
times (`:983-989`), and `PostgresEventStore::new` takes a pool it does not own
(`event_store.rs:102-116`) — so what a handle *is* decides whether the race is
real. Eight clones of a `max_connections = 2` pool is a serialisation point that
no rule can see. `a_concurrent_reader_never_sees_a_partial_batch` asks for
`WRITERS + 1 = 5` handles at once (`concurrency.rs:762-768`) and the other rules
for 8; size for 8 and state the number. The fixture itself is
`postgres-schema-and-live-fixture`'s deliverable — this story is where its pool
sizing stops being a guess, and closing a gap found there is not scope drift.

**6. `ConcurrentFixture` is a blanket impl — there is nothing to write, only
something to prove.** `impl<F> ConcurrentFixture for F where F: Fixture, F::Store:
Send` (`concurrency.rs:186-193`). The trait exists so the macro's opaque
`impl ConcurrentFixture` return type carries `F::Store: Send` to the call site,
which `impl Fixture` cannot. So the fixture needs no second trait impl; it needs
`PostgresEventStore: Send`, which the crate already asserts
(`event_store.rs:193-197`). The testing brief's "two fixture-shaped things, or one
type implementing both" (`_decomposition.md:557-565`) resolves, on reading the
blanket impl, to **one type** — record that resolution rather than building the
second type the brief left open.

**7. The visibility frontier and this family have to be reconciled, and nobody
else does it.** ES-10 is `[FROZEN]` and an adapter buying it with `xid8` +
`pg_snapshot_xmin` "reports a **frontier** from `head()` rather than
`max(position)`, and therefore does **not** satisfy read-your-own-writes"
(`spec/SPECIFICATION.md:2833-2846`). Three of these five rules end with a
**post-hoc read on a fresh handle** that must see every acknowledged event —
`Query::all()` counted at `CONTENDERS * BATCH`, at `WRITERS * ROUNDS * BATCH`,
and the winner's batch read back by query (`concurrency.rs:602-616`, `:790-812`,
`:395-403`). Under a frontier those reads pass only while nothing pins
`pg_snapshot_xmin` — so a fixture that leaves an idle-in-transaction connection
in its pool turns a conformant adapter red. Equally: ES-25's `iff` "is sound only
where visibility order agrees with position order" (`:2860-2862`), so the append
**condition** must be evaluated against committed state and not against the
frontier-filtered view, or two contenders both probe clean and both commit and
`exactly_one_of_n_contenders_commits` fails (`:2859-2862`). Neither reconciliation is stated in
any upstream brief. Finding it is part of this story; recording it feeds
`postgres-structural-bill` (AC-009) and ADR-0024.

**8. The witness that writers overlap is this story's, the number is ADR-0024's.**
AC-003 reads "the ADR's own measurement shows write concurrency was retained"
(`project.md:234-236`) and the ADR story depends on this one. The split: this
story produces the **observation** — server-side evidence that at least two
append transactions were in flight simultaneously, plus the structural claim that
the shipped append path contains no store-wide serialisation point, with the SQL
quoted; ADR-0024 produces the **throughput number**, re-measured against the real
adapter (`_decomposition.md:527-540`). CF-33 binds *conformance rules* and this
witness is not one — it lives in the Postgres crate's own tests — but the spirit
holds: no wall-clock assertion, no operation-count assertion, and a failure
message that names serialisation as the suspected cause rather than timing out
mutely (`concurrency.rs:24-43`). Absence of observed overlap is evidence, not
proof, and must be reported as such.

**9. The default gate stays Docker-free and network-free.** Not in
`xtask/src/main.rs`'s `REQUIRED` array; yes in the live-Postgres job created by
`postgres-schema-and-live-fixture`, whose invocation the testing brief already
sketches as `cargo test -p happenstance-postgres --all-features -- --ignored
--show-output` (`_decomposition.md:613-624`). Gating is **whole-invocation**
(`#[ignore]`, `required-features`, or an env read) and must match the mechanism
that story chose — a second, different mechanism in the same crate is two ways to
run the same tests and one of them will rot.

**The persona slice.** The user here is an adapter author and a consuming
application (`_storymap.md:19-23`). This story is the moment the port stops being
a claim about one storage shape: an adapter author with a store that does *not*
serialise its writers finds out, from a suite rather than from production, whether
their probe was atomic, whether their isolation level is discriminating between
boundaries or shouting about all of them, and whether the position they handed
back was their own or somebody else's. The consuming application's stake is the
same fact one layer down — a projection checkpointed at another writer's position
silently skips events (`spec/SPECIFICATION.md:3452-3459`).

## Integration contract

| Field | Value |
| ----- | ----- |
| **Archetype** | `capability` — a user-observable slice in this repository's only medium: a conformance family that runs and reports against a real server (`_storymap.md:19-23`). |
| **Slice / milestone** | `postgres-live-suite`. **Slice-mates:** `postgres-schema-and-live-fixture` (foundation), `postgres-append-and-frontier-head`, `postgres-rule-controls`. Implemented together in one context and mounted as one integrated surface; within the slice this story follows `postgres-append-and-frontier-head` and is independent of `postgres-rule-controls` (`_storymap.md:246-250`). |
| **Mount point** | `crates/happenstance-postgres/tests/concurrency_conformance.rs` — a **new test target** invoking `happenstance_testkit::event_store_concurrency_conformance!(PostgresFixture::new())`. This is the composition root for a conformance family in this repository: "an adapter that compiles but has not run the conformance suite is not an adapter" (`CLAUDE.md`), and the architecture brief names this exact seam for AC-003 (`_decomposition.md:59`). Sibling naming follows `crates/happenstance-testkit/tests/memory_concurrency_conformance.rs`. A macro invocation that exists but is never executed by the live job is the "constructed but unmounted" failure in this medium — the CI job wiring below is part of the mount, not a follow-up. |
| **Wires into** | `crates/happenstance-testkit/src/concurrency.rs` — `event_store_concurrency_conformance!` (`:1129-1165`), `for_each_concurrency_rule!` (`:1044-1055`), `__emit_concurrency_tokio` (`:1070-1082`), `ConcurrentFixture`'s blanket impl (`:186-193`), `CONTENDERS` (`:206`). `PostgresFixture` from `postgres-schema-and-live-fixture` — consumed, and its pool sizing tightened here if it is short of `CONTENDERS`. `crates/happenstance-postgres/src/event_store.rs:121-175` — the `SendEventStore` impl whose `append`/`head` bodies `postgres-append-and-frontier-head` lands; read-only here except where §4's reactor seam forces a change. `crates/happenstance-postgres/Cargo.toml` — `[dev-dependencies]` already carries `tokio` with `macros`, `rt`, `rt-multi-thread` (`:24-26`); any addition (e.g. a runtime-context helper) is checked against `deny.toml`'s allowlist. `.github/workflows/ci.yml` — the live-Postgres job, sibling of `gate` (`:31`), `backlog` (`:102`), `wasm-conformance` (`:204`), `msrv` (`:241`), `semver` (`:279`), `advisories` (`:325`). |
| **Renders surfaces** | **None.** `_design.md` records no user-facing surface for this project and the sign-off approved that determination (`_design.md:10-41`, `:86-95`); `design.capture` is deliberately absent from `.redkiln/config.yaml`, making the perceptual review a declared skip. |
| **Public items** | **None** in the workspace's published surface. This story adds a test target and, if §4's adapter-side route is taken, at most a `#[cfg(test)]`/dev-only wrapper. Any item that becomes `pub` on `happenstance-postgres` here is a semver promise nobody made and belongs to `deskeleton-and-package-readiness`. |
| **Conformance rule(s)** | The five of `for_each_concurrency_rule!` (`concurrency.rs:1044-1055`): `exactly_one_of_n_contenders_commits` (`:347`), `k_disjoint_boundaries_admit_exactly_k_commits` (`:436`), `positions_are_unique_under_concurrent_appends` (`:566`), `append_returns_the_callers_own_last_position` (`:637`), `a_concurrent_reader_never_sees_a_partial_batch` (`:735`). **All five run; none is added, edited, skipped or `#[cfg]`-ed out.** No rule of this family is capability-gated, so none may report `Skipped` — a skip here means the family grew a `require!` nobody accounted for (`mutation_coverage.rs:2633-2635`). |
| **Clause(s)** | Discharged against a live unserialised store, not amended: **ES-25** (`spec/SPECIFICATION.md:3695`), **ES-18** (`:3330`), **ES-19** (`:3435`), **VT-11** (`:1019`), and **ES-10** (`:2816`) at the point where the frontier meets the post-hoc reads. `k_disjoint_boundaries_admit_exactly_k_commits` is cited by no clause — **recorded, not fixed** (Context pack §2). No `[FROZEN]` marker moves; no ADR is owed by this story. |
| **Advances DoD scenario** | Initiative **DoD 5** — "a store that does not serialise its writers passes the suite, with the position-visibility cost measured rather than estimated" (`initiative.md:373`). This story lands the *passes the suite* half and produces the overlap observation the measured half is argued from; the measurement itself is ADR-0024's (`_storymap.md:64`). |

## PR boundary

`redkiln verify --grain story` reads the first fenced block under this heading and
fails on any file changed outside it.

```
crates/happenstance-postgres/tests/**
crates/happenstance-postgres/src/**
crates/happenstance-postgres/Cargo.toml
.github/workflows/ci.yml
.bklg/from-contract-to-published-library/postgres-and-neon-stores/postgres-concurrency-family/**
```

`crates/happenstance-postgres/src/**` is inside the boundary and deliberately
narrow in intent: it is here for the reactor seam of Context pack §4 and for the
fixture's pool sizing, **not** for `append`/`head` semantics, which are
`postgres-append-and-frontier-head`'s and arrive merged. `.github/workflows/ci.yml`
is the composition-root wiring this slice mounts through and touching it is not
scope drift. `crates/happenstance-testkit/**` is **outside** the boundary: taking
Context pack §4's fallback route means widening it here, deliberately, with the
decision recorded — never widening it to make a red gate go green.

**In this PR**

- A new test target under `crates/happenstance-postgres/tests/` invoking
  `event_store_concurrency_conformance!` against the live `PostgresFixture`,
  gated whole-invocation by the same mechanism the fixture story chose.
- Whatever it takes to make the adapter's futures drivable from a bare contender
  thread, chosen between Context pack §4's two routes and recorded, without
  weakening `PostgresEventStore`'s `SendEventStore` impl.
- Pool/handle sizing sufficient for `CONTENDERS = 8` genuinely independent
  writers, stated as a number, with a diagnosable failure rather than a deadlock
  when it is short.
- The overlap witness: server-side evidence that two append transactions were in
  flight at once, clock-free, with its result recorded either way.
- The live-Postgres CI job extended to run this target (or confirmed already to,
  if `postgres-schema-and-live-fixture` wired the whole package).
- The ledger rows `adr-0024-position-visibility-mechanism` and
  `far-end-discharge-record` read: which rules ran, at what `CONTENDERS`, against
  which pinned Postgres minor, and the two observations of Context pack §2 and §7.

**Explicitly not in this PR**

- `append`, `head`, `contains_event_id`, migration 1, the append-condition SQL,
  the fixture's construction, or the CI job's creation — the two upstream stories'.
- The ES-10 mechanism **choice**, the throughput number, or ADR-0024 in any form.
- The naive-`nextval()` control and the mutant-control mount — `postgres-rule-controls`.
- Any change to `spec/SPECIFICATION.md`, including minting a clause for
  `k_disjoint_boundaries_admit_exactly_k_commits` or moving an ES-10 marker.
- Any new or edited conformance rule; any benchmark (`CF-34`; benchmarks are
  `sqlite-durable-store`'s, `project.md:157-159`).
- Neon anything. The concurrency family is Postgres-only by design and its
  absence there is not a gap (`_storymap.md:174-175`).

**Merge DoD one-liner.** `cargo xtask ci --fast` green tree-local with no Docker
and no network, and the live-Postgres CI job green with all five concurrency
rules reporting `Ran` at `CONTENDERS = 8`, on a store whose writers were observed
overlapping.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The family is invoked as one whole, from the adapter's own test target | `event_store_concurrency_conformance!(PostgresFixture::new())` expands to a module with `async fn __conformance_fixture() -> impl ConcurrentFixture` and one `#[tokio::test(flavor = "multi_thread")]` per rule. The single-argument arm is the right one; `mod_name =`/`emit =` arms exist for a second harness and this story needs one. | `crates/happenstance-testkit/src/concurrency.rs:1129-1165`, `:1070-1082`; `crates/happenstance-testkit/tests/memory_concurrency_conformance.rs:26-35` |
| All five rules run and report; none skips | No concurrency rule is capability-gated, so `RuleOutcome::Skipped` is impossible by construction — a skip means the family grew a `require!`. `#[cfg]`-ing a rule out is forbidden outright (DR-5). | `crates/happenstance-testkit/src/concurrency.rs:305-310`; `mutation_coverage.rs:2633-2635`; `project.md:192-196` |
| Exactly one of eight contenders deciding from one snapshot commits | Eight handles append the same tagged event under one `AppendCondition::after`; seven must be told `ConditionViolated` and the store must afterwards hold exactly the one batch it acknowledged. Rejects the probe-then-insert shape (`RacingProbeStore`) that every sequential rule passes. | `concurrency.rs:347-403`; `spec/SPECIFICATION.md:3695` (ES-25), `:3724`; `mutation_coverage.rs:2533-2554` |
| Disjoint boundaries do not conflict — K boundaries, exactly K commits | Four boundaries × three contenders. Fails in **both** directions: too many winners on one boundary, and *no* winner on a boundary a coarse concurrency control shouted about. A `SERIALIZABLE` adapter mapping `40001 serialization_failure` onto `AppendError::ConditionViolated` is `GlobalVersionStore` and fails here. | `concurrency.rs:436-549`; `mutation_coverage.rs:2555-2569` |
| Positions are unique under concurrent appends | Eight contenders × a two-event batch, unconditional; every contender commits, every event is present, no two events anywhere share a position. Rejects `SELECT max(position)` taken outside the transaction that consumes it. The rule deliberately does **not** assert the returned positions are distinct — that defect belongs to the next row. | `concurrency.rs:566-635`; `spec/SPECIFICATION.md:1019` (VT-11), `:1030`; `mutation_coverage.rs:2570-2583` |
| `append` returns the caller's own last position, not the store's head | Eight writers × two tagged events each; each contender's returned position must be its own batch's last. `INSERT …; SELECT max(position)` is indistinguishable from `INSERT … RETURNING position` until a second writer exists. A caller checkpoints a projection on this value. | `concurrency.rs:637-733`; `spec/SPECIFICATION.md:3435` (ES-19), `:3452-3459`; `mutation_coverage.rs:2584-2594` |
| A concurrent reader never sees a partial batch | Four writers × four rounds × three events, one reader looping alongside. Two halves: a sampling live half (cannot flake against a conformant store) and a deterministic post-hoc half — every acknowledged event present, every batch whole. Rejects the row-at-a-time insert loop with the `BEGIN` forgotten. | `concurrency.rs:735-812`; `spec/SPECIFICATION.md:3330` (ES-18), `:3341`; `mutation_coverage.rs:2595-2609` |
| Contenders are OS threads with no ambient reactor; the adapter must cope | `race` spawns one `std::thread::scope` thread per contender behind a `Barrier` starting gate and each rule drives its future with the testkit's park-loop `block_on`. `sqlx` is named in advance as the case this does not serve. Resolution is adapter/fixture-side by preference (re-enter the runtime per `poll`), testkit-side only as a recorded decision. | `concurrency.rs:236-302`, `:45-68`, `:1059-1067`; `crates/happenstance-testkit/src/registry.rs:330-345` |
| The `Send` claim is not weakened to buy the reactor fix | `PostgresEventStore` implements `SendEventStore`; a `!Send` `EnterGuard` held across an `.await` would demote it and break constraint 4's "bind `EventStore`, not `SendEventStore`" arithmetic from the other side. The existing `store_is_send_and_sync` and `send_flavour_satisfies_the_bare_bound` tests are the tripwire and must stay green. | `crates/happenstance-postgres/src/event_store.rs:118-121`, `:177-197`; `CLAUDE.md` constraints 1 and 4; `.kb/decisions/0001-async-port-flavours.md` |
| One fixture instance, eight independent handles, pool sized for them | `connect_many` calls `connect()` N times; `PostgresEventStore::new` wraps a pool it does not own. Eight handles sharing a pool smaller than eight is a serialisation point no rule can see, and a pool below `CONTENDERS` deadlocks rather than failing (no watchdog — CF-33). The number is stated, not implied. | `concurrency.rs:983-989`, `:198-206`, `:24-43`; `crates/happenstance-postgres/src/event_store.rs:102-116` |
| `ConcurrentFixture` needs no new impl — only `F::Store: Send` | Blanket impl for every `Fixture` whose `Store` is `Send`. The supertrait exists so the macro's opaque return type elaborates `F::Store: Send` at each call site; `impl Fixture` would not. The brief's open "two fixture-shaped things or one" resolves to one, and that resolution is recorded. | `concurrency.rs:186-193`, `:160-186`; `_decomposition.md:557-565` |
| The visibility frontier must not break the post-hoc reads | Three rules end with a fresh-handle read that must see every acknowledged event. Under `xid8` + `pg_snapshot_xmin` those pass only while nothing pins the frontier — an idle-in-transaction pooled connection turns a conformant adapter red. Conversely the append **condition** must be evaluated against committed state, since ES-25 is sound only where visibility order agrees with position order. | `concurrency.rs:395-403`, `:602-616`, `:790-812`; `spec/SPECIFICATION.md:2833-2846`, `:2859-2862` |
| Writers are observed to overlap; serialisation is ruled out, not assumed | The family's own conformant control is `LockedStore` — one mutex across the whole append — so passing proves nothing about concurrency. A non-conformance, clock-free witness in this crate's own tests supplies the server-side evidence, and the shipped append path is claimed free of any store-wide serialisation point with its SQL quoted. Reported honestly when overlap is not observed. | `mutation_coverage.rs:2525-2532`; `crates/happenstance-postgres/src/event_store.rs:43-53`; `project.md:174-179`, `:335`; `concurrency.rs:24-43` |
| The default gate never needs Docker or a network | The target is gated whole-invocation by the mechanism `postgres-schema-and-live-fixture` chose and runs only in the live-Postgres CI job. `cargo test --workspace --all-features` with no server exits zero; `cargo xtask ci --fast` is green tree-local. | `_decomposition.md:333-352`, `:606-624`; `.redkiln/config.yaml` (`verify.integration_scoped`); `project.md:212-217` |
| No literal position values anywhere this story writes | Positions are compared against what the store assigned, never against `[1, 2, 3]`. The rules already do this; the witness and any helper must too, and `cargo xtask lint-position-literals` runs in the story grain. | `CLAUDE.md` ("the rule that matters"); `_storymap.md:107-108`; `_decomposition.md:355-356`; `spec/SPECIFICATION.md:1019` (VT-11, gaps permitted) |

## Data and migrations

**No new migration, and no schema change of this story's own.** Migration 1 —
identity and time columns, tag storage, and whatever column ADR-0024's mechanism
adds — is `postgres-schema-and-live-fixture`'s and arrives merged
(`_storymap.md:60`; intended schema at
`crates/happenstance-postgres/src/event_store.rs:7-20`). This story reads and
writes through the `SendEventStore` impl only; it issues no DDL of its own.

Two data-adjacent obligations that are this story's and are easy to mistake for
someone else's:

1. **Isolation between contenders is store-level, not schema-level.** One fixture
   instance is one isolated backing store and each `connect()` is one handle onto
   *that* store (`CLAUDE.md`, "the rule that matters"). Eight handles must reach
   the same schema; the family is meaningless if the fixture hands out eight
   databases. Whether that isolation is a container, a database or a schema per
   fixture instance is the fixture story's decision — this story asserts the
   consequence.
2. **Nothing may hold a transaction open across rules.** A pooled connection left
   idle-in-transaction pins `pg_snapshot_xmin` and, under a frontier mechanism,
   makes acknowledged events invisible to the post-hoc reads three of these rules
   end with (`spec/SPECIFICATION.md:2833-2846`). The long-running-transaction
   scenario DR-3 asks to be *measured* (`RUNBOOK.md:4370-4372`) is
   ADR-0024's; what this story owes is that the suite does not create one by
   accident and that the hazard is written down where
   `postgres-structural-bill` can pick it up.

## Acceptance criteria

The personas are the story map's two (`_storymap.md:19-23`): the **adapter
author** building a store whose writers are not serialised, and the **consuming
application** whose projection checkpoints on whatever `append` handed back.
Each criterion is that person's goal crossing the whole stack — fixture, pool,
adapter, server — not a capability restated.

Test paths below are the macro's real expansion: the single-argument arm names
the module `dcb_concurrency_conformance` and emits one
`#[tokio::test(flavor = "multi_thread")]` per rule
(`crates/happenstance-testkit/src/concurrency.rs:1129-1165`, `:1070-1082`), so a
rule's test id is
`crates/happenstance-postgres/tests/concurrency_conformance.rs::dcb_concurrency_conformance::<rule>`.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | GIVEN an adapter author has built a Postgres store and has no instrument that tells them whether it is correct under contention, WHEN they run the crate's live test target against a pinned Postgres, THEN all five rules of the concurrency family execute and report by name — none absent, none reporting `Skipped`, none `#[cfg]`-ed out of the macro expansion — AND WHEN a colleague with no Docker and no network runs the default gate, the same target does not run and the command still exits zero. | `crates/happenstance-postgres/tests/concurrency_conformance.rs::dcb_concurrency_conformance` — the whole module, run in the live-Postgres CI job as `cargo test -p happenstance-postgres --all-features -- --ignored --show-output` (`_decomposition.md:613-624`). The five names read out of `--show-output` and compared against `for_each_concurrency_rule!`'s list (`concurrency.rs:1044-1055`); a `Skipped` outcome or a missing name fails the AC. Docker-free half: `cargo xtask ci --fast` green on a checkout with no server (`_decomposition.md:601-611`). |
| **AC-002** | GIVEN eight instances of an application each decide, from the same read of the same consistency boundary, that they may append, WHEN all eight append under that one `AppendCondition::after` at once, THEN exactly one is acknowledged, the other seven are told `ConditionViolated` rather than being silently accepted, and a reader opening a fresh handle afterwards finds exactly the one batch that was acknowledged — so the probe-then-insert shape that passes every sequential rule is caught here instead of in production. | `…::dcb_concurrency_conformance::exactly_one_of_n_contenders_commits` at `CONTENDERS = 8` (`concurrency.rs:347-403`, `:206`). Discharges **ES-25** (`spec/SPECIFICATION.md:3695`, `:3724`). The wrong implementation it must reject is `RacingProbeStore` (`mutation_coverage.rs:2533-2554`). |
| **AC-003** | GIVEN four teams' commands touch four unrelated consistency boundaries — the independence property Dynamic Consistency Boundary exists to provide — WHEN three writers contend on each of the four at once, THEN exactly four commits result: one winner per boundary, no boundary left with none, and no boundary with two — so an author who reached for `SERIALIZABLE` and mapped `40001 serialization_failure` onto `ConditionViolated` learns from the suite that their store now refuses commands that never conflicted. | `…::dcb_concurrency_conformance::k_disjoint_boundaries_admit_exactly_k_commits` (`concurrency.rs:436-549`, `BOUNDARIES = 4` at `:444`). Both failure directions are live: `RacingProbeStore` for too many winners, `GlobalVersionStore` for too few (`mutation_coverage.rs:2533-2554`, `:2555-2569`). This rule is cited by **no clause** — see AC-009's record and `.kb/open-questions/disjoint-boundaries-have-no-clause.md`. |
| **AC-004** | GIVEN eight writers append unconditionally at the same moment and every one of them is acknowledged, WHEN a consuming application later reads the store to build any position-ordered view, THEN no two events anywhere in the store share a position — so a `SELECT max(position)` taken outside the transaction that consumes it cannot hand two writers the same slot and make one event unreachable by position. | `…::dcb_concurrency_conformance::positions_are_unique_under_concurrent_appends` (`concurrency.rs:566-635`, `BATCH = 2` at `:574`). Discharges **VT-11** (`spec/SPECIFICATION.md:1019`, `:1030`). Rejects `RacingSequenceStore` (`mutation_coverage.rs:2570-2583`). Positions are compared against what the store assigned, never against literals — `cargo xtask lint-position-literals` (`xtask/src/lints.rs:628`) is the standing enforcement. |
| **AC-005** | GIVEN a consuming application appends a batch and checkpoints its projection at the position `append` returned, WHEN seven other writers were committing to the same store at that instant, THEN the position it received is the last position of **its own** batch and not the store's head — so the projection resumes from its own work rather than skipping every event another writer landed in between. | `…::dcb_concurrency_conformance::append_returns_the_callers_own_last_position` (`concurrency.rs:637-733`). Discharges **ES-19** (`spec/SPECIFICATION.md:3435`, and the skipped-events consequence at `:3452-3459`). Rejects `GlobalHeadStore` — `INSERT …; SELECT max(position)` — which is indistinguishable from `INSERT … RETURNING position` until a second writer exists (`mutation_coverage.rs:2584-2594`). |
| **AC-006** | GIVEN a projection runner reads the store continuously while four writers append three-event batches around it, WHEN it samples the stream mid-flight and again after every writer has finished, THEN it never observes a batch half-present, and the post-hoc read on a fresh handle sees **every** acknowledged event — including under whatever ES-10 mechanism the adapter buys visibility with, so a frontier `head` and a pooled connection left idle-in-transaction cannot make acknowledged events invisible and turn a conformant adapter red. | `…::dcb_concurrency_conformance::a_concurrent_reader_never_sees_a_partial_batch` (`concurrency.rs:735-812`; `WRITERS = 4`, `ROUNDS = 4`, `BATCH = 3` at `:743-748`), both halves. Discharges **ES-18** (`spec/SPECIFICATION.md:3330`, `:3341`). Rejects `RowAtATimeStore` (`mutation_coverage.rs:2595-2609`). The frontier half is additionally evidenced by the post-hoc reads inside AC-002 and AC-004 (`concurrency.rs:395-403`, `:602-616`) passing green against a store whose `head` is a frontier (`spec/SPECIFICATION.md:2833-2846`). |
| **AC-007** | GIVEN the family is only an instrument if the eight contenders genuinely contend, WHEN one `PostgresFixture` instance hands out `CONTENDERS = 8` handles onto one backing store, THEN each handle holds an independent connection resource — the pool admits at least eight concurrent checkouts, stated in the code as a number derived from `happenstance_testkit::concurrency::CONTENDERS` rather than hard-coded — AND WHEN it is short, the run fails with a message naming pool exhaustion instead of hanging, because CF-33 forbids the watchdog that would otherwise tell the two apart. | A `#[tokio::test]` in `crates/happenstance-postgres/tests/concurrency_conformance.rs`, sibling to the macro's module, asserting the fixture's configured concurrent-checkout capacity `>= happenstance_testkit::concurrency::CONTENDERS` and failing with that message — plus the family green, which is the behavioural half. Grounded in `concurrency.rs:983-989` (`connect_many` = N × `connect()`), `:198-206` (the deadlock warning) and `crates/happenstance-postgres/src/event_store.rs:102-116` (the store wraps a pool it does not own). |
| **AC-008** | GIVEN each contender drives its future to completion on a bare OS thread with no ambient reactor, WHEN an adapter whose futures need one is put under that harness, THEN the family runs to a real verdict rather than panicking on a missing reactor — AND the fix does not cost the adapter its `SendEventStore` impl, because a `!Send` `EnterGuard` held across an `.await` would demote the store and break every consumer that binds the `Send` flavour. The route taken is recorded, not left to be inferred from the diff. | The five rules green (they cannot pass at all if the futures are undrivable) plus the existing tripwires still green under `cargo test -p happenstance-postgres`: `store_is_send_and_sync` and its sibling at `crates/happenstance-postgres/src/event_store.rs:177-197`. The recorded route lands in `.bklg/from-contract-to-published-library/postgres-and-neon-stores/postgres-concurrency-family/_evidence.md`, naming which of Context pack §4's two routes was taken and why the other lost; if the testkit-side route was taken, that note also records the widened PR boundary. |
| **AC-009** | GIVEN AC-003 of the project requires that write concurrency was **retained** and not traded away, and GIVEN the family's own conformant control is `LockedStore` — one mutex across the whole append — so a green run proves nothing about concurrency on its own, WHEN the author of ADR-0024 opens this story's record, THEN they find server-side, clock-free evidence that at least two append transactions were in flight simultaneously (or an honest statement that overlap was not observed, reported as evidence rather than proof), the shipped append path's SQL quoted with the claim that it contains no store-wide serialisation point, and the run's parameters — which five rules ran, at `CONTENDERS = 8`, against which pinned Postgres minor — together with the two structural observations this story surfaced and does not settle. | `crates/happenstance-postgres/tests/write_overlap_witness.rs` — a non-conformance test in the live job, clock-free (no wall-clock and no operation-count assertion, per CF-33's spirit, `concurrency.rs:24-43`), plus the record at `.bklg/from-contract-to-published-library/postgres-and-neon-stores/postgres-concurrency-family/_evidence.md`. Traces project **AC-003** (`project.md:234-236`) and DR-2 (`project.md:174-179`); the throughput **number** is explicitly ADR-0024's, not this AC's (`_decomposition.md:527-540`). |

Coverage of the traced project AC: **AC-003** (`project.md:234-236`) has two
halves — "`event_store_concurrency_conformance!` passes under a multi-thread
runtime at `concurrency::CONTENDERS`" is AC-001 through AC-008 above, and "write
concurrency was retained rather than traded away" is AC-009. No half is left to
be inferred from the other.

## Interaction quality

**Composition family — N/A by an approved determination, not by omission.** This
project renders no user-facing surface: `_design.md` records that explicitly and
the sign-off approved *that determination itself*
(`_design.md:10-41`, `:86-95`), and `design.capture` is deliberately absent from
`.redkiln/config.yaml`, which makes the perceptual review a declared skip rather
than a silent pass (`CLAUDE.md`). There is no presentation, placement,
transience, density budget, hierarchy or named visual anti-pattern to bind here,
and inventing one would contradict a signed-off artifact.

**State family — it does apply, in this repository's medium.** The thing a human
perceives is the run: a test binary's report, read from a CI log. Each invariant
below is carried by an AC row in the table above; none of them is stated only
here, because a bullet in this section gets no ledger row and is never gated.

| Invariant (state family) | Carried by | How it is verified |
| --- | --- | --- |
| **Non-occlusion.** Every rule's outcome is visible in the run; nothing is hidden behind a passing aggregate and nothing vanishes from the binary. | **AC-001** | Five named results read out of `--show-output` and compared against `for_each_concurrency_rule!`. A `Skipped` outcome is a failure here, not a state to accommodate (`mutation_coverage.rs:2633-2635`). |
| **In-place, not context-jump.** The family is invoked from the adapter's own crate against the real fixture — no parallel harness, no hand-picked subset, no second copy of the rules living in the Postgres crate. | **AC-001** | The mount is one macro invocation of the single-argument arm; the `mod_name =` / `emit =` arms are unused (`concurrency.rs:1129-1165`). |
| **Reversibility / non-destructive default.** A developer with no Docker and no credentials runs the default gate and nothing they own changes state; the live target does not execute and does not fail. | **AC-001** | `cargo xtask ci --fast` green tree-local, whole-invocation gating only — never `#[cfg]`-ing a rule out (DR-5, `project.md:192-196`). |
| **Diagnosability instead of a mute hang.** A short pool must fail with a message naming pool exhaustion; CF-33 forbids the watchdog that would otherwise distinguish a deadlock from a failure. | **AC-007** | The pre-flight capacity assertion carries the message; `concurrency.rs:198-206` is the reason it exists. |
| **Preserved intent under contention.** What the caller is handed back is its own — its own position, its own verdict — not the store's most recent state. | **AC-005**, **AC-002** | `append_returns_the_callers_own_last_position`; `exactly_one_of_n_contenders_commits`. |
| **Honest reporting of a negative.** Absence of observed write overlap is reported as absence of evidence, never as proof that writers were serialised or that they were not. | **AC-009** | The `_evidence.md` record states the witness's result either way; the AC is not satisfiable by omitting an inconvenient outcome. |
| **Keyboard reachability / focus / scroll / selection** | — | N/A. No interactive surface exists; see the composition paragraph above. |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | The fixture's pool admits fewer than `CONTENDERS = 8` concurrent checkouts. | Fail with a message naming pool exhaustion **before** the family runs. A hang is the forbidden outcome: there is no watchdog (CF-33) and the suite cannot tell a deadlock from a slow store (`concurrency.rs:198-206`, `:24-43`). Sizing the pool at 1 is worse than sizing it short — the family goes green with every writer serialised behind the pool, which is exactly the failure this story exists to catch. |
| **EC-002** | An adapter future is polled on a contender thread with no reactor and panics (`sqlx`'s case, named in advance at `concurrency.rs:61-68`). | Resolve at the seam, per Context pack §4, and record the route. The forbidden repair is holding a `!Send` `EnterGuard` across an `.await`, which would silently demote `PostgresEventStore` out of `SendEventStore` — `store_is_send_and_sync` (`crates/happenstance-postgres/src/event_store.rs:177-197`) is the tripwire and must not be relaxed to make this compile. |
| **EC-003** | A pooled connection is left idle-in-transaction, pinning `pg_snapshot_xmin`; under a frontier mechanism the post-hoc reads of three rules stop seeing acknowledged events. | Treat as a **defect in the harness**, not a rule failure: the suite must not create a long-running transaction by accident. The hazard is written into `_evidence.md` for `postgres-structural-bill` (AC-009 of the project) rather than fixed by weakening a rule (`spec/SPECIFICATION.md:2833-2846`). |
| **EC-004** | A rule reports `RuleOutcome::Skipped`. | Fail the run. No rule in this family is capability-gated, so a skip is impossible by construction — it means the family grew a `require!` nobody accounted for (`concurrency.rs:305-310`, `mutation_coverage.rs:2633-2635`). Do not add a `Capability` to make the skip legitimate; that is a testkit change and a decision, not a fix. |
| **EC-005** | The append path maps `40001 serialization_failure` onto `AppendError::ConditionViolated`. | `k_disjoint_boundaries_admit_exactly_k_commits` fails, correctly, from the too-few-winners side (`GlobalVersionStore`, `mutation_coverage.rs:2555-2569`). The repair is in the adapter's error mapping, never in the rule. This is the most tempting shortcut available to a Postgres author reaching for isolation to buy the append condition. |
| **EC-006** | The live job starts with no reachable Postgres, or the container fails to come up. | The job **fails**. A run that executes zero tests and reports success is the same false green as a `#[cfg]`-ed-out rule; the whole point of gating whole-invocation is that the absence is visible in the job that is supposed to run it (`_decomposition.md:606-624`). |
| **EC-007** | The live-Postgres job goes flaky. | Fixed or reported — never quietly made non-blocking. DR-9 and the project risk table are explicit, and there is no sanctioned "revert to skip" path (`project.md` risk table, last row; deployment brief, *Rollback posture*). A flake in a concurrency family is a signal about the adapter, not about CI. |

## Non-functional

| id | requirement | why, and where it is checked |
| --- | --- | --- |
| **NF-001** | The default gate stays Docker-free and network-free. `cargo test --workspace --all-features` with no server exits zero; `cargo xtask ci --fast` is green tree-local; nothing this story adds enters `xtask/src/main.rs`'s `REQUIRED` array. | DR-9 / project AC-011 (`project.md:212-217`; architecture brief §7). This is the one requirement proven by the **absence** of infrastructure. |
| **NF-002** | No clock. No wall-clock assertion, no `sleep`-based synchronisation, no operation-count assertion — in the rules (already true), in the pool pre-flight, or in the overlap witness. | CF-33 (`concurrency.rs:24-43`); `cargo xtask lint-clock` (`xtask/src/main.rs:370`, `:683`) is the grep that enforces it where a type cannot. The witness must therefore establish overlap from server state, not from timing. |
| **NF-003** | No literal position values anywhere this story writes. Positions are compared against what the store assigned; the specification permits gaps and a conformant Postgres adapter will leave them. | CF-6; `cargo xtask lint-position-literals` (`xtask/src/lints.rs:628`), which `cargo xtask affected` runs in the story grain (`xtask/src/affected.rs:121`). |
| **NF-004** | No new item becomes `pub` on `happenstance-postgres` in this story. Any helper the reactor seam needs is `#[cfg(test)]` or dev-only. | A `pub` item here is a semver promise nobody made; publish readiness belongs to `deskeleton-and-package-readiness` and the `semver` CI job (`ci.yml:279`) is what notices. |
| **NF-005** | Any new dev-dependency is admissible under `deny.toml`'s allowlist (MIT / Apache-2.0 / BSD-2 / BSD-3 / ISC / Unicode-3.0 / Zlib, `[graph] all-features = true`) and compiles at the MSRV. | `cargo deny check` runs in the default gate on this machine (`CLAUDE.md`, *Commands*); the `msrv` job pins 1.97.1 (`ci.yml:241`). The five database crates in this workspace declare no `rust-version`, so only running the compiler finds a floor violation. |
| **NF-006** | The live job pins a specific Postgres minor and the run records which one. A concurrency verdict against an unnamed server version is not reproducible and cannot be cited by ADR-0024. | Deployment brief, *CI implication* (`testcontainers` pinned to a specific minor); AC-009's record carries the number. |
| **NF-007** | The run is bounded without a timeout hack: five rules × `CONTENDERS = 8`, one fixture instance each, no retries and no artificial backoff. If it needs a longer CI timeout than its siblings, that is a finding about the adapter and belongs in `_evidence.md`. | CF-33 again — a timeout knob is a clock wearing a job-configuration hat. |

## Implementation notes (non-prescriptive)

Options, not instructions. Every one of these is the implementer's to overturn
with a reason recorded in `_evidence.md`.

**The reactor seam (AC-008).** The adapter-side route is preferred because it
keeps `crates/happenstance-testkit/src/**` outside the PR boundary. The shape
that fits: the fixture's `connect()` is awaited **on** the multi-thread runtime
the emitter created, so `tokio::runtime::Handle::current()` is available there
and can be captured into the handle; the handle's futures then re-enter that
runtime around each `poll` rather than holding an `EnterGuard` across an
`.await`. Per-`poll` entry is what keeps the `!Send` guard off the future's type
and preserves `SendEventStore`. A wrapper future in the Postgres crate's own
`tests/` (or `#[cfg(test)]`) is enough; it does not need to be `pub`, and it
should not be. If this cannot be made to work, Context pack §4's fallback — the
spawn as a harness parameter, exactly as `concurrency.rs:61-68` predicts — is
permitted, and taking it widens the PR boundary deliberately with the reason
written down.

**Pool sizing (AC-007).** `a_concurrent_reader_never_sees_a_partial_batch` wants
`WRITERS + 1 = 5` handles live at once and the other four rules want 8, so 8 is
the binding number — but 8 is the floor, not the target. Sizing at
`CONTENDERS + 2` leaves room for the pre-flight assertion and for whatever the
fixture itself holds, and costs nothing. Derive it from
`happenstance_testkit::concurrency::CONTENDERS` rather than writing `8`: the
constant is the testkit's to change, and a hard-coded 8 would go quietly wrong.
The fixture is `postgres-schema-and-live-fixture`'s deliverable — tightening its
sizing here is closing a gap that story left open, not scope drift.

**The overlap witness (AC-009).** It must be clock-free (NF-002), so it cannot
be "two appends finished within N milliseconds". Postgres offers a structural
answer instead: two transactions overlapped **iff** one observed the other as
in-flight. `pg_current_xact_id()` inside each append transaction and
`pg_current_snapshot()`'s in-progress list read from a concurrent one gives that
directly, with no clock anywhere; `pg_stat_activity` sampled from a third
connection is the cruder alternative and is closer to a clock than it looks.
Whichever is chosen, run it as a *separate* test target from the conformance
family so a witness failure and a rule failure are never debugged as the same
failure — the testing brief makes exactly this argument for the Neon transport
(`_decomposition.md`, Notes §5). Pair it with the structural claim: quote the
shipped `append`'s SQL and state that it contains no store-wide serialisation
point, because a witness that happens not to catch an overlap on a quiet CI
runner is weak evidence and the structural reading is not.

**What to write down rather than settle.** Three observations belong in
`_evidence.md` and in no code change: that
`k_disjoint_boundaries_admit_exactly_k_commits` is cited by no clause — already
an open question, `.kb/open-questions/disjoint-boundaries-have-no-clause.md`, so
confirm it against `HEAD` rather than re-discovering it; that the testing brief's
"two fixture-shaped things, or one type implementing both" resolves to **one
type**, because `ConcurrentFixture` is a blanket impl over every `Fixture` whose
`Store: Send` (`concurrency.rs:186-193`); and the frontier reconciliation of
Context pack §7, which `postgres-structural-bill` and ADR-0024 both need and
neither upstream brief states.

**Mount naming.** Follow `crates/happenstance-testkit/tests/memory_concurrency_conformance.rs`
— the only in-tree invocation of this macro and the closest thing to a reference
implementation for what the file should contain (`:10-16`, `:26-35`). Its own
header says why it is not sufficient evidence on its own, which is the sentence
this story is written against.

## Tests and CI (merge gate)

Grounded in the project testing brief's two-list split — the tree-local gate
every story runs, and the live jobs that gate the project
(`_decomposition.md`, Testing brief §Intent and Notes §6).

| tier | command / path | proves |
| --- | --- | --- |
| **Conformance (live Postgres job)** | `cargo test -p happenstance-postgres --all-features -- --ignored --show-output`, running `crates/happenstance-postgres/tests/concurrency_conformance.rs::dcb_concurrency_conformance` | AC-001 through AC-006 and the behavioural half of AC-008: five named rules, all reporting, at `CONTENDERS = 8`, under `#[tokio::test(flavor = "multi_thread")]`, against a pinned Postgres minor. |
| **Pre-flight (same target, live job)** | the capacity assertion in `crates/happenstance-postgres/tests/concurrency_conformance.rs`, sibling to the macro's module | AC-007 / EC-001: eight independent checkouts available, failure message names pool exhaustion, no hang. |
| **Witness (live job, non-conformance)** | `crates/happenstance-postgres/tests/write_overlap_witness.rs` | AC-009's observation half: two append transactions in flight at once, established from server state with no clock (NF-002). |
| **Unit (tree-local, no server)** | `cargo test -p happenstance-postgres` — `store_is_send_and_sync` and its sibling at `crates/happenstance-postgres/src/event_store.rs:177-197` | AC-008's constraint half: the reactor fix did not cost the adapter its `SendEventStore` impl. |
| **Story grain (tree-local)** | `cargo xtask affected --base main` (`verify.affected_gate`, `.redkiln/config.yaml`) | NF-002 and NF-003 — `lint-clock` and `lint-position-literals` over what this story wrote — plus fmt, clippy `-D warnings` and the affected packages' tests. |
| **Project bar (tree-local)** | `cargo xtask ci --fast` (`verify.integration_scoped`) | NF-001: green with no Docker and no network. This is the AC-011 evidence this story must not erode. |
| **Static / process** | `redkiln verify --grain story` | The PR boundary fence and the acceptance ledger: every AC-### present, satisfied, with cited evidence. |
| **Static / process** | `cargo xtask spec-trace` (already `REQUIRED`) | That no clause marker moved. This story discharges ES-18, ES-19, ES-25, VT-11 and ES-10 against a live store; it amends none of them. |
| **Static / process** | `cargo xtask package-check`, `cargo deny check` (already in the gate) | NF-004 and NF-005 — no new public surface, no inadmissible dev-dependency. |

The two live jobs are `.github/workflows/ci.yml` siblings of `gate` (`:31`),
`backlog` (`:102`), `wasm-conformance` (`:204`), `msrv` (`:241`), `semver`
(`:279`) and `advisories` (`:325`) — never new steps inside `gate`
(deployment brief, AC-011 row). If `postgres-schema-and-live-fixture` wired the
job to the whole package, this story adds nothing there and says so; if it wired
named targets, this story's two targets are added to that list.

## Risks and coupling (PR-scoped)

| risk | why it is live in *this* PR | mitigation |
| --- | --- | --- |
| **Green is bought by serialising writers and nobody notices.** | The family cannot tell an overlapping store from a serialised one — `LockedStore`, one mutex across the whole append, is its *conformant control* (`mutation_coverage.rs:2525-2532`). If ADR-0024's arm lands as a serialised sequence table, every rule here goes green and the adapter is `MemoryEventStore` with network latency (`crates/happenstance-postgres/src/event_store.rs:43-53`). | AC-009 is not optional and is not satisfiable by the family passing. The witness plus the structural SQL reading are what make AC-003 mean what `project.md:234-236` says. |
| **The pool is the serialisation point.** | Eight handles cloned off a `max_connections = 2` pool is a lock no rule can see, and eight off a pool of 1 makes the whole family decorative. | AC-007's pre-flight assertion, derived from `CONTENDERS` rather than a literal. This is the highest-value cheap check in the story. |
| **The reactor fix demotes the store to `!Send`.** | The obvious repair — hold an `EnterGuard` for the duration — is `!Send` and would take `SendEventStore` with it, breaking constraint 4's arithmetic for every consumer that binds the `Send` flavour (`CLAUDE.md`; `.kb/decisions/0001-async-port-flavours.md`). | EC-002; the existing `Send` assertions stay green and are named as a merge gate tier. Per-`poll` entry keeps the guard off the future's type. |
| **Coupling to ADR-0024: the append path may change after this PR merges.** | The ES-10 mechanism is chosen *downstream* of this story (`_storymap.md:64`), and a mechanism change edits the append path these five rules exercise. | The family is in the live job, so ADR-0024's story re-runs it by construction. This story's record states the mechanism in force when it ran, so a later green is not mistaken for the same evidence. |
| **Coupling to `postgres-schema-and-live-fixture`: pool sizing and the CI job are its artefacts.** | Both are touched here, and two stories editing one fixture is how a merge conflict becomes a silent behaviour change. | Slice-mates land in one context and in the map's order (`_storymap.md:246-250`); the sizing change is additive and derived from a testkit constant, so a later fixture edit that re-narrows it fails AC-007's assertion loudly. |
| **The testkit fence gets crossed quietly.** | The fallback route in Context pack §4 edits `crates/happenstance-testkit/src/**`, which the architecture brief marks must-not-change (`_decomposition.md:102-104`) and which is outside this story's PR boundary. | Taking it is permitted and is a **recorded decision** in `_evidence.md` naming why the adapter-side route failed. Widening the boundary to turn a red gate green, without that record, is the thing forbidden. |
| **Frontier vs. post-hoc reads produces a confusing red.** | Three rules end with a fresh-handle read that must see everything acknowledged; under `xid8` + `pg_snapshot_xmin` an idle-in-transaction pooled connection makes that read short and the failure looks like a rule bug. | EC-003 names it in advance and routes the fix to the harness, not the rule. `.kb/open-questions/postgres-arm-c-structural-cost.md` is the standing statement that the experiment measured strategies, not adapters. |
| **Scope creep into the mutant control.** | `postgres-rule-controls` is independent of this story and lands beside it in the same slice; both are about "would the rule have noticed?" and the boundary is easy to blur. | The PR boundary's *Explicitly not in this PR* list names the naive-`nextval()` arm and the mutant-control mount as that story's. This story's negative evidence is the in-tree `RACERS` table, already discharged. |

## Dependencies

**Blocks on**

- **`postgres-append-and-frontier-head`** — supplies the `append`, `head` and
  `contains_event_id` bodies these five rules exercise. Without them every rule
  fails on a `todo!()` and the family reports nothing about concurrency
  (`_storymap.md:61`).
- *(transitively)* **`postgres-schema-and-live-fixture`** — migration 1, the
  `testcontainers`-backed `PostgresFixture`, the in-crate gating mechanism and
  the live-Postgres CI job (`_storymap.md:60`). Not this story's `depends_on`
  edge, because the direct predecessor already carries it, but the artefacts this
  story tightens are that story's.

**Unlocks**

- **`adr-0024-position-visibility-mechanism`** — depends on this story by name
  (`_storymap.md:64`). It re-measures against the real adapter and needs AC-009's
  observation as the qualitative half of its argument; the throughput number is
  its own.
- **`postgres-structural-bill`** (downstream of the ADR) — consumes Context pack
  §7's frontier reconciliation and EC-003's hazard from `_evidence.md`.
- **`far-end-discharge-record`** — consumes the run's parameters and the
  no-clause observation for the project's AC-013 ledger.

**Independent of** — `postgres-rule-controls` (same slice, either order,
`_storymap.md:246-250`) and everything Neon: the concurrency family is
Postgres-only by design and its absence there is not a gap
(`_storymap.md:174-175`).

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than dropped. Open each at the moment named;
do not preload the set.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `crates/happenstance-testkit/src/concurrency.rs` | The whole family: five rule bodies, `race`'s `std::thread::scope` + `Barrier` harness, `connect_many`, `CONTENDERS`, `ConcurrentFixture`'s blanket impl, the two emitters and the macro's three arms. Every AC in this story is a statement about something in this file. | First, before writing the mount — and again at `:61-68` the moment a future panics for want of a reactor. | AC-001 |
| `crates/happenstance-testkit/tests/memory_concurrency_conformance.rs` | The only in-tree invocation of the macro: what the new file should look like, and a header (`:10-16`) that states plainly why a passing `MemoryEventStore` is not evidence about concurrency. | Immediately before creating `crates/happenstance-postgres/tests/concurrency_conformance.rs`. | AC-001 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | `RACERS` at `:2524-2609` — six wrong stores pinned against these five rules, every provenance paragraph describing a Postgres shape. It is the list of ways to get this adapter wrong, written before the adapter existed. | Before writing or reviewing the append path's SQL and error mapping; again when a rule fails and the cause is unclear. | AC-002 |
| `crates/happenstance-postgres/src/event_store.rs` | `:43-53` states the axis this story defends; `:102-116` shows the store wraps a pool it does not own (so what a handle *is* decides whether the race is real); `:118-121` and `:177-197` are the `Send` claim and its tripwire tests. | Before sizing the pool (AC-007) and before attempting any reactor fix (AC-008). | AC-008 |
| `crates/happenstance-testkit/src/registry.rs` | `:330-345` — the park-loop `block_on` each contender drives its future with. This is *why* there is no ambient reactor; reading the harness second-hand is how the fix gets aimed at the wrong layer. | When diagnosing a missing-reactor panic, before deciding between Context pack §4's two routes. | AC-008 |
| `crates/happenstance-testkit/src/contract.rs` | `Fixture` itself — `connect()`, the `Capability` constants and the isolation contract that makes one fixture instance one backing store. `ConcurrentFixture` adds nothing but a bound. | When tightening `PostgresFixture`'s pool sizing, to confirm what a handle is obliged to be. | AC-007 |
| `spec/SPECIFICATION.md` | The clauses discharged here: ES-25 (`:3695`, `:3724`), ES-18 (`:3330`), ES-19 (`:3435` and the skipped-events consequence at `:3452-3459`), VT-11 (`:1019`), and ES-10 with the frontier/no-RYOW consequences at `:2833-2846` and the `iff`-soundness note at `:2859-2862`. | When a rule fails and the question is whether the rule or the adapter is wrong — and before writing AC-006's frontier reconciliation. | AC-006 |
| `.kb/decisions/0013-position-assignment-and-visibility.md` | The accepted decision behind ES-10 — what the visibility invariant actually promises, which is what AC-006's post-hoc reads are asserting against. | Before reconciling the frontier with the post-hoc reads; do not infer the invariant from the rule bodies. | AC-006 |
| `.kb/decisions/0001-async-port-flavours.md` | Why there is no `#[async_trait]`, why `Send` is a separate flavour rather than a bound on the port, and therefore what is actually lost if the reactor fix demotes `PostgresEventStore`. | Before writing any wrapper that touches the store's future types. | AC-008 |
| `.kb/open-questions/disjoint-boundaries-have-no-clause.md` | The no-clause observation is **already** an accepted open question, held in `UNCLAIMED_PENDING_ADR` — so AC-009's record confirms it against `HEAD` rather than re-discovering it, and must not silently mint a clause. | When writing `_evidence.md`, before claiming this story found something new. | AC-003 |
| `.kb/open-questions/postgres-arm-c-structural-cost.md` | States that the position-visibility experiment measured four SQL *strategies*, not four `SendEventStore` implementations — no pooling, no transaction lifetime tied to a trait method, no cursor. It is the reason AC-009's evidence is owed at all. | When drafting the overlap witness and the structural claim, and before citing `experiments/position-visibility/`. | AC-009 |
| `experiments/position-visibility/README.md` | The prior measurement (arm C at 0.987–1.026× under `fsync=on`) whose limits the open question above names. Useful context for the witness; **not** a substitute for it, and the re-measurement is ADR-0024's. | Only after the witness design is settled, to state what is already known and what is not. | AC-009 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md` | Testing brief Notes §3 (the fixture shape for this family), Notes §6 (`:601-624`, the gating mechanism and the live job's invocation) and architecture §7 (`:333-352`, the CI topology and DR-5's hard constraint on gating). | Before choosing how the target is gated and before touching `ci.yml`. | AC-001 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/project.md` | DR-2 at `:174-179` and the risk table's first row at `:335` — the charter-level statement that retained write concurrency is an acceptance criterion, not a side effect. | Before writing `_evidence.md`, to keep AC-009's claim aimed at what the project actually asked for. | AC-009 |
| `.github/workflows/ci.yml` | The job topology this story mounts into — `gate` (`:31`), `backlog` (`:102`), `wasm-conformance` (`:204`), `msrv` (`:241`), `semver` (`:279`), `advisories` (`:325`) — and `backlog`'s credentialed pattern at `:108-132` as the nearest in-tree precedent for a job the default gate does not run. | When wiring or confirming the live-Postgres job runs these targets. | AC-001 |

## Clarifications resolved during spec

1. **The AC set is exactly the nine the front half enumerated**, AC-001 through
   AC-009: one for the whole-family mount and its gating, one per conformance
   rule (five), one for handle independence, one for the reactor seam and the
   `Send` claim, and one for the evidence that write concurrency was retained.
   None added, none dropped. The five-rule split is deliberate rather than
   bundled: each rule rejects a *different* wrong Postgres adapter named by hand
   in `RACERS`, so collapsing them into one row would make the ledger unable to
   say which shape was proven absent.
2. **The testing brief's "two fixture-shaped things, or one type implementing
   both" (`_decomposition.md`, Notes §3) resolves to one type**, and the
   resolution is a reading of the code rather than a choice:
   `impl<F> ConcurrentFixture for F where F: Fixture, F::Store: Send` is a
   blanket impl (`concurrency.rs:186-193`). The trait exists only so the macro's
   opaque `impl ConcurrentFixture` return type carries `F::Store: Send` to the
   call site, which `impl Fixture` could not. Recorded in `_evidence.md`; no
   second type is built.
3. **`k_disjoint_boundaries_admit_exactly_k_commits` is cited by no clause, and
   this story does not fix that.** It is already an accepted open question
   (`.kb/open-questions/disjoint-boundaries-have-no-clause.md`), which also
   records that ES-25 is the *wrong* attachment point. Minting or widening a
   clause is an ADR act; the ledger entry belongs to `far-end-discharge-record`
   (project AC-013). AC-009's record confirms the state and stops.
4. **The overlap witness is this story's; the throughput number is ADR-0024's.**
   Project AC-003's phrase "the ADR's own measurement" could be read as making
   the whole of it wait for the ADR, but the ADR story depends on this one
   (`_storymap.md:64`), so the dependency direction settles it: this story
   produces the observation the ADR argues from, and produces it clock-free.
5. **The witness lives in its own test target**, not inside the conformance
   file, so a witness failure and a rule failure are never debugged as one
   failure — the same argument the testing brief makes for proving the Neon
   transport before wiring it to a macro.
6. **Gating mechanism is inherited, not chosen here.** Whichever of `#[ignore]`,
   `required-features` or an env read `postgres-schema-and-live-fixture` picked
   is what this story's targets use. A second mechanism in the same crate is two
   ways to run the same tests and one of them rots
   (`_decomposition.md:606-624`).
7. **No `Interaction quality` composition invariants are asserted.** Not an
   omission: `_design.md` records no user-facing surface and the sign-off
   approved that determination (`_design.md:86-95`). The state-family invariants
   that *do* apply in this medium are each carried by an AC row, never by a
   prose bullet, because `redkiln verify` extracts ACs from table cells and
   bullets — a bullet in that section would be ungated and untested.
8. **`crates/happenstance-postgres/tests/` does not exist yet.** It is created by
   this slice; the mount point is a path this story brings into being, which is
   why it is named in the Integration contract but is not cited as an anchor.
