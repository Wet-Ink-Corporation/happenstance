---
item: HS-S0063
stage: discover
created: 2026-08-12T13:02:31.366Z
updated: 2026-08-12T13:02:31.366Z
template_sig: 86ce4036
rendered_sig: dd520696
---

# Discover — The concurrency family green on unserialised writers

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one line: `event_store_concurrency_conformance!` green at `CONTENDERS = 8` under a multi-thread runtime — the first time any adapter in the portfolio clears that bar on a store whose writers are not serialised | `_storymap.md`, *Slices* table, `postgres-concurrency-family` row | The novelty is not the macro. It is the *store underneath it*, and the story's whole risk is that the macro cannot tell |
| **AC-003** — the concurrency family is green on a store that does not serialise its writers, **and** the ADR's own measurement shows write concurrency was retained rather than traded away | `project.md`, *Acceptance criteria*, AC-003 | Two halves, and the second is not a test result. The green family alone does not discharge this AC |
| `depends_on: postgres-append-and-frontier-head` (HS-S0062) | manifest; `_storymap.md`, *Merge order* item 2 | Supplies a real `append` and a frontier `head`; there is nothing to race until those bodies exist |
| Transitively, HS-S0061 supplies `ConcurrentFixture` handing out `CONTENDERS = 8` handles onto **one** backing store | `_storymap.md`, *Slices* table, `postgres-schema-and-live-fixture` row; `crates/happenstance-testkit/src/concurrency.rs:206` | The fixture is the instrument. A fixture whose eight handles queue on one connection makes this story green and meaningless — recorded as HS-S0061's named mutant |
| The family is opt-in, its bound is `F::Store: EventStore + Send`, and a `!Send` adapter cannot invoke it and is not expected to | `crates/happenstance-testkit/src/lib.rs:101-104` | Postgres-only by design. Neon's absence from this family is not a gap, and DR-8 depends on that staying true |
| The parallelism is in `std::thread::scope`, not in the runtime; the macro emits `#[tokio::test(flavor = "multi_thread")]` anyway, because an adapter's own futures may need a reactor | `crates/happenstance-testkit/src/concurrency.rs:53-54`, `:150-153`, `:1060`, `:1073` | `sqlx` needs the tokio reactor, so the tokio emitter is the correct one here — and the `block_on` emitter "races exactly as hard" (`:1084-1098`), so the choice is about the reactor, not about the race |
| `ConcurrentFixture` is `Fixture<Store: Send>` written as a supertrait bound, not a `where` clause | `crates/happenstance-testkit/src/concurrency.rs:182-186` | An opaque `impl Fixture` at the call site would leave the bound unprovable. The invocation must name a concrete fixture type |
| `PostgresEventStore` holds a `PgPool`, not a connection, and that is the shape difference that makes the crate an instrument | `crates/happenstance-postgres/src/event_store.rs:93-95` | "readers and writers do not queue behind one another, so nothing about the storage layer supplies ES-10 for free" |
| The serialised sequence table "makes this adapter stop being an instrument… `MemoryEventStore` with network latency" | `crates/happenstance-postgres/src/event_store.rs:43-53` | The risk table's first row, quoted from the crate itself. This story is the instrument meant to catch it, and the section below argues that it cannot on its own |
| Benchmarks are not conformance (CF-34), and `event_store_benchmarks!` is `sqlite-durable-store`'s work item | `project.md`, *Out of scope* | No rule may be added to this family that fails a store for being slow. That closes the obvious way to catch the mutant below |
| Phase 10's proof artefact: "the concurrency macro green under a multi-thread runtime against a store that **does not serialise its writers** — the first time any adapter in the portfolio clears that bar" | `RUNBOOK.md:4368-4372` | Paired in the same sentence with "a committed number for the visibility strategy's cost". The runbook already treats the two as one artefact |
| Four existing implementations all serialise their writers and assign positions under a lock held to commit | `RUNBOOK.md:672-674` | One storage shape wearing four hats. This story is the first evidence that the port was not frozen against that shape alone |

## Questions

**Answered.**

1. *Which emitter?* The tokio one
   (`crates/happenstance-testkit/src/concurrency.rs:1060-1073`). Not because the
   race needs it — `std::thread::scope` supplies the parallelism either way
   (`:1084-1098`) — but because `sqlx`'s futures need a reactor to make progress.
   `spec` records that reason so nobody "simplifies" it later.
2. *Can this family fail a store that serialises its writers?* **No**, and that is
   the story's central finding. Serialisation is the safest possible implementation
   of every rule in the family: one winner is elected, no update is lost, no
   inversion is observable. AC-003's second half exists precisely because a green
   run is necessary and not sufficient.
3. *Does the family need a new rule?* No. Adding one that fails a store for being
   slow would violate CF-34 and would be a rule with no principled threshold. The
   missing instrument is a *measurement*, and it is ADR-0024's.

**Deferred to the owning stories.**

4. *How the adapter buys ES-10's visibility invariant when `nextval()` allocates
   outside the transaction.* `adr-0024-position-visibility-mechanism` (HS-S0065),
   which consumes this story's result as evidence. This story does not choose a
   mechanism; it reports whether the chosen one left writers unserialised.
5. *Whether `conflicting_position` is a promise every adapter owes or a hint one may
   omit.* `neon-conflicting-position-verdict` (HS-S0070). Relevant here only in that
   `racing_conditional_appends_elect_one_winner` asserts the *semantics* of a race
   without depending on the conflicting position's presence
   (`crates/happenstance-testkit/src/suite.rs:5325`), so nothing in this story
   forces the question either way.

**Deferred to `spec`.**

6. *The concrete witness that the eight handles were genuinely concurrent* — pool
   capacity observed at or above `CONTENDERS`, overlapping in-flight appends
   observed rather than assumed. Named below as a deliverable; its exact form is
   `spec`'s.

## Decision

The problem this slice solves is that the port has never been raced against a store
whose writers do not queue. `MemoryEventStore`, a `RefCell` store, the rusqlite
skeleton and a Durable Object stand-in all serialise their writers and assign
positions under a lock they hold until commit — one storage shape wearing four hats
(`RUNBOOK.md:672-674`) — so every concurrency rule in the suite has, to date, been
satisfied by construction rather than by design. This story invokes
`event_store_concurrency_conformance!` against `PostgresFixture` at `CONTENDERS = 8`
inside the live job and requires it green, which is the first genuine test of the
family. The spec will cover: the invocation and the concrete fixture type it names;
the tokio multi-thread emitter and why the reactor, not the race, selects it; the
concurrency witness that makes "eight handles" mean eight *simultaneous* handles;
and the statement — carried forward into ADR-0024 — that a green family is
necessary and not sufficient for AC-003. No clause is changed and no rule is added,
so no ADR is owed by this story; ADR-0024 is where its result is spent.

## The wrong implementation

**`SerialisedSequenceStore`: `PostgresEventStore` whose `append` allocates with
`UPDATE hs_sequence SET n = n + 1 RETURNING n` inside the append transaction.** The
row lock is held to commit, so allocation order *is* commit order and ES-10 holds
trivially. `SendEventStore::append`'s signature is untouched and no round trip is
added — the `UPDATE … RETURNING` folds into the statement that was going to run
anyway (`crates/happenstance-postgres/src/event_store.rs:43-53`). It passes
`event_store_conformance!`, `event_store_model_conformance!` and **this story's
family, at every contender count**, more reliably than the right implementation
does, because serialisation is the strongest guarantee any of those rules can ask
for. `cargo xtask ci` is green, the live job is green, AC-002 is green, AC-004 is
green, and the adapter is now `MemoryEventStore` with network latency: the far end
of the position-allocation axis (`RUNBOOK.md:687`) is filled by a fifth copy of the
shape the workspace already had four of, and the port stands frozen against
SQLite wearing five hats.

Nothing in `happenstance-testkit` can reject it, and nothing should be added that
tries: throughput is not conformance (CF-34), and a rule with a latency threshold
would be a rule with no principled value. The instrument is a **number** — the
measurement ADR-0024 owes, at 16× and 30× for the serialising arms
(`RUNBOOK.md:520`; `experiments/position-visibility/README.md:12-19`) — and the
control belongs in `crates/happenstance-postgres/tests/` as a feature-gated
throwaway arm exercised once and recorded in the ADR, on the same terms
`postgres-rule-controls` (HS-S0064) sets for the naive `nextval()` arm
(`_decomposition.md`, *Testing brief* → *Notes* §2). Keeping a second,
deliberately-serialised fixture alive in the tree would itself need a stated reason.

Its fixture-side twin is HS-S0061's named mutant — a pool of one — and the two are
worth holding together, because they produce identical output: a green concurrency
family over eight writers that never overlapped. That is why the concurrency witness
above is a deliverable of *this* story rather than a nicety.

For contrast, one plausible mutant the family **does** catch: a `ConcurrentFixture`
whose `connect()` hands out handles onto separate schemas. All eight contenders then
win their conditional append, and
`racing_conditional_appends_elect_one_winner` (`crates/happenstance-testkit/src/suite.rs:5325`)
fails loudly. The family bites on isolation; it is blind to serialisation. Stating
both is what keeps this story from being read as a stronger result than it is.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
