---
item: HS-S0111
stage: discover
created: 2026-08-12T13:03:33.066Z
updated: 2026-08-12T13:03:33.066Z
template_sig: 86ce4036
rendered_sig: a565059f
---

# Discover — One suite, three peers, two of them genuinely unlike

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: confirm both environments exist from HS-P0013/HS-P0014 first, then implement `SyncPeer` and `IngestStore` in `happenstance-cloudflare` and `happenstance-neon` (local type, foreign trait — the only place coherence allows it) and run the one suite against all three peers, with an unavailable environment reported as a declined capability carrying the fixture's stated reason and never as a vanished target | `_storymap.md`, *Slices* table, `live-unlike-peers` | Confirmation first, then two adapters, then one suite run three times |
| **Depends on `headline-rules-and-mutant-registry` (HS-S0105)** for the rules, on `message-set-on-the-envelope` (HS-S0106) for the wire, and on `send-free-sync-runner` (HS-S0109) for the runner | `_storymap.md`, *Slices* `depends_on`; `_storymap.md`, *Merge order* step 6 | *"This is DoD 4's proof artefact; it needs the suite, the wire and the runner all in the tree"* |
| **AC-005** — one suite green against `MemorySyncPeer`, a socket-reachable Durable Object peer, and a one-shot-HTTP Postgres peer *"that cannot hold a transaction open across a round trip"* | `project.md`, *Acceptance criteria*, AC-005 | Three peers, and the third's defining property is a *negative* capability |
| **DoD 4** — the proof artefact *"would not exist if the design were wrong"*: one suite green against three peers, two structurally unlike, with a byte-identical payload round-tripped across a store boundary | `project.md`, *Definition of done* 4; `RUNBOOK.md:4597-4602` | *"the two-peer half is what makes this a port rather than a protocol"* |
| **DR-6** — the port is proved by **spread**, not by count: three peers, at least one of which cannot hold a transaction open across a round trip | `project.md`, DR-6; `RUNBOOK.md:4607-4608` | Count is the metric that is easy to satisfy and proves nothing |
| The workspace's standing rule: *"a port with one implementation is shaped like that implementation"*, which is why the trait is settled against two peers *"as unlike each other as the deployment story allows"* | `crates/happenstance-sync/src/lib.rs:30-35` | The reason this story exists at all, stated by the crate about itself |
| Before freezing a port, *"name the axis it is most likely to be wrong about and check that something in the workspace sits at the other end of it"* — four adapters that all serialise their writers and assign positions under a lock are *"SQLite wearing four hats"* | `CLAUDE.md`, *The rule that matters*, second corollary | The axis here is **held state between calls**. Neon is the other end of it |
| The two stand-ins with *"the type properties of the real thing and none of its dependencies"*, and their own honest limit: *"A stand-in's `Send`-ness is asserted by whoever wrote it rather than by `reqwest`, and that is the honest limit of this evidence."* Their disagreement table: `Send` no/yes, ambient runtime none/tokio, state between calls a live socket/nothing at all, cursor possible/impossible, error `Rc<JsValue>`/status code | `crates/happenstance-sync/tests/real_peer_shapes.rs:1-25` | When the real impls land, **the stand-ins are superseded and the file's finding is not** (`_decomposition.md`, *Composition root* §4) |
| `tests/cursor_shape_probe.rs` is the compiled proof that `impl Stream` admits the one-shot-HTTP peer *by buffering a whole response into a `Vec` and replaying it* — *"Legal, `Send`, and a lie"* | `crates/happenstance-sync/src/peer.rs:43-50`; `crates/happenstance-sync/tests/cursor_shape_probe.rs` | The in-tree demonstration that a peer can satisfy a port's shape while contradicting its intent. It must keep compiling |
| Consequently the round-trip property *"has to be checked by a fixture peer that counts its own round trips, in the conformance suite — the port itself cannot express it"* | `crates/happenstance-sync/src/peer.rs:47-50` | The counter HS-S0103 put on the fixture is what makes the Neon axis observable rather than asserted |
| **SY-15 `[FROZEN]`** — every port method completable in one round trip; `Rejects:` *"the shape every socket-based design produces: `open()` returning a handle, `next_batch(&mut handle)`, `close(handle)`. It is the right shape for the Durable Object reached over a WebSocket and it is unimplementable on the Neon peer"* | `spec/SPECIFICATION.md:6292-6311` | The clause names both peers and the exact way one would exclude the other |
| **SY-17 `[FROZEN]`** — the runner binds `EventStore`, not `SendEventStore`, checked by the testkit compiling its own suite against a `!Send` fixture peer holding a `!Send` store behind an `Rc` | `spec/SPECIFICATION.md:6341-6360` | The Durable Object is where a `Send` bound landed in HS-S0109 would finally fail — and this is the story that finds it |
| Coherence: an adapter crate writes the mirror image of the sketch's impl — foreign trait, own local type — and that compiles; a **third** crate cannot write it on an adapter's behalf | `crates/happenstance-sync/src/ingest.rs:26-37` | The two peers need **no core change**. Only the in-memory oracle did (`_decomposition.md`, *Composition root* §5, note on option (d)) |
| **DR-8** — a declined capability still runs and reports the fixture's stated reason; it never vanishes from the binary | `project.md`, DR-8; `crates/happenstance-testkit/src/contract.rs:31-42` | *"an adapter author who declines a capability to turn a red build green gets a green build and no record of the trade"* |
| Risk row: two of the three peers are networked and cost real infrastructure; *"The Neon axis cannot be faked without destroying the thing it exists to test."* This project carries **no deployment brief**, so the infrastructure must already exist from HS-P0013 and HS-P0014 — *confirm that at the architecture brief, not at implementation* | `project.md`, *Risks and coupling notes*; `_decomposition.md`, *Non-prescriptive implementation notes* | If either environment is absent, that is **a blocker to raise, never a stand-in to write** |
| The residual risk's second scheduled mitigation: ADR-0026 must be written against **two** unlike peers, not one | `RUNBOOK.md:462-466` | This story is where that mitigation is either discharged or shown to have failed |
| `happenstance-cloudflare` is the workspace's only `!Send` store, `wasm32`; `happenstance-neon` is Postgres over one-shot HTTP with *"no connection, no interactive transaction, no cursor"*, host **and** `wasm32` | `CLAUDE.md`, *Repository map* | Both crates already claim these targets in their own documentation |

## Questions

**Does ingest re-check the writer's asserted append conditions? — Answered
elsewhere, and this story is the first place a real transport could smuggle the
old answer back in.** A one-shot-HTTP peer cannot hold a transaction open across
a round trip, so an implementer who wants group atomicity is under real pressure
to push the guard to the server and let *it* decide — which is receiver-side
re-evaluation with a network in the middle. SY-1 and SY-6 apply to the peer
regardless of transport, and `ingest_never_rejects` runs against all three peers
unchanged.

**Hub-and-spoke versus peer-to-peer — not this story's**, but it consumes the
answer: the Durable Object is the canonical mid-chain node, a spoke to a Neon
estate store and a hub to many tablets (`spec/SPECIFICATION.md:6350-6360`), so
`one_adapter_serves_both_roles` should be exercised against it and not only
against the oracle.

**Do both environments exist? — Unanswered here by design, and it is the first
task of the slice rather than a question spec can settle.** The architecture
brief puts the confirmation at slice 1 and this story restates it as its own
gate: confirm the Durable Object and Neon environments HS-P0013 and HS-P0014
built, **before** writing either peer. This project carries no deployment brief,
so it owns no path to create them. If either is absent, the correct output is a
raised blocker.

**How does the suite run against a `wasm32` Durable Object? — Deferred to spec,
with the constraint that no new harness is invented.** *"Runs the suite against a
real or locally-emulated Worker, per whatever harness HS-P0013 already
established for that crate; this project adds the sync suite as a second thing
that harness runs, not a new harness"* (`_decomposition.md`, testing brief
*Integration*).

**What happens when the Neon environment is unavailable in a given run? —
Answered: a declined capability with the fixture's stated reason, never a
skipped or absent target.** *"This leg cannot be gated behind a feature flag that
silently no-ops"* (`_decomposition.md`, testing brief *Integration*). The
mechanism already exists — `Capability::declined` rejects an empty reason string
(`crates/happenstance-testkit/src/contract.rs:389-400`) — and the reason must
name the missing environment, not say "unsupported".

**What happens to `tests/real_peer_shapes.rs`? — Answered: retire the `todo!()`s,
keep the record.** The stand-ins are superseded the moment the real impls land;
the file's *finding* is not, because it is the transcript for ADR-0026's *"the
type checker did not force that choice"*
(`crates/happenstance-sync/src/lib.rs:130-132`; `_decomposition.md`, *Composition
root* §4). `tests/cursor_shape_probe.rs` keeps compiling either way.

**Does either peer need a change to `happenstance-core` or to the port? —
Answered: no, and if that turns out false it is a stop condition.** Coherence
lets each adapter write `impl SyncPeer for MyType` in its own crate. If a real
peer needs a seam on the **port**, that is `RUNBOOK.md:462-466`'s residual risk
landing hard: raise a new decision atom and a re-plan
(`_decomposition.md`, *Composition root* §5).

## Decision

Everything this project has built so far has been measured against
`MemorySyncPeer` — an in-process object, in one thread, in a process that stays
alive for the whole exchange — and the workspace's own standing rule says a port
with one implementation is shaped like that implementation. The specific axis the
sync port is most likely to be wrong about is already identified and already has
a name: **held state between calls**. `pull` returns a bounded batch and an owned
token rather than a stream precisely because a peer reached over one-shot HTTP
has no connection, session or transaction to hold a cursor in — and the crate is
honest that *the type checker did not force that choice*, because the cursor
shape compiled against both peers, satisfied by the HTTP peer buffering a whole
response into a `Vec` and replaying it: *"Legal, `Send`, and a lie."* So a port
frozen on type-checking alone is frozen on nothing. This story runs the one suite
against three peers that genuinely sit at different points on that axis: the
in-process oracle, a socket-reachable Durable Object that is `!Send` and has no
ambient runtime to spawn into, and a Postgres over one-shot HTTP with no
interactive transaction and no cursor — with the fixture's own round-trip counter
as the instrument, because the port cannot express the property it is being
tested for. The spec stage will cover: the environment confirmation and the
blocker path if either is missing; the two `impl SyncPeer` / `impl IngestStore`
blocks and where they live; each fixture's capability constants and the stated
reason for every declension; how the Durable Object leg reuses HS-P0013's
existing harness; the round-trip counter's assertions per peer; the retirement of
`real_peer_shapes.rs`'s `todo!()`s with its finding preserved; and the coverage
report — which axes the three peers span and which they do not.

## The wrong implementation

**The mutant this story exists to prevent, and it is a *test-suite* configuration
rather than a peer: three peers that are all the same shape.** Implement the
Durable Object peer against a local in-process emulator that keeps a live object
between calls, and implement the "Neon" peer as an in-process struct that answers
synchronously and holds its state in a `Vec`. Both compile, both implement
`SyncPeer` and `IngestStore` honestly, both are genuinely different *types*, and
the suite is green three times. AC-005's wording — three peers, one of them
unable to hold a transaction open — reads as satisfied, because the fake Neon
peer *chooses* not to hold one. The port is then frozen against one transport
wearing three hats, and the workspace has done this before at a larger scale:
`MemoryEventStore`, a `RefCell` store, rusqlite and a Durable Object are *"four
adapters, one storage shape"*, which is why `happenstance-postgres` and
`happenstance-neon` exist as instruments (`CLAUDE.md`, *The rule that matters*).
The refutation is not a mutant in `tests/` — it is the **live leg**, and the
project's own risk table says why: *"The Neon axis cannot be faked without
destroying the thing it exists to test"*, because a mock of "no interactive
transaction" asserts only what the mock's author already believed that constraint
permits. What *does* belong in `crates/happenstance-sync-testkit/tests/` is the
round-trip-counting conformant variant from HS-S0103 (`OneShotHttpFixture`),
which fails any rule that quietly assumes a second call within one operation, and
whose passing is the mechanical claim that the suite is transport-honest.

**`SilentlySkippedNeonLeg` — the version that turns the above into an accident.**
Gate the Neon leg behind `#[cfg(feature = "neon-live")]`, off by default. CI runs
green, the suite reports two peers passing, and the third simply is not in the
binary — indistinguishable from a passing test, which is the exact argument the
testkit's own documentation makes for reporting rather than `#[cfg]`-ing a
skipped rule (`crates/happenstance-testkit/src/contract.rs:31-42`). It is not
even dishonest at the moment it is written; it becomes dishonest three months
later when nobody remembers the feature exists. The required shape is a declined
`Capability` carrying a non-empty reason naming the missing environment, so the
run reports *"Neon: declined — NEON_DATABASE_URL unset"* and the trade is on the
record (DR-8).

**`BufferingCursorPeer` — the peer that satisfies the port and contradicts it.**
Implement the one-shot-HTTP peer by fetching the entire remote log into a `Vec`
on the first `pull` and serving subsequent calls from memory. Every rule passes.
Round-trip counts even look plausible if the counter is placed at the wrong
layer. It is the compiled lie `cursor_shape_probe.rs` already demonstrates —
*"the one-shot HTTP peer satisfies `impl Stream` by buffering a whole response
into a `Vec` and replaying it. Legal, `Send`, and a lie"*
(`crates/happenstance-sync/src/peer.rs:43-50`) — and it fails in the field on the
first log too large to buffer inside a Worker's memory limit, which is the same
constraint WF-11's falsifier names. The instrument is the fixture's counter
placed **at the transport**, asserting one network call per port operation, plus
a bounded-memory fixture; a counter placed at the port method counts the wrong
thing.

**And `LeafSendPeer` — the arrangement that hides SY-17's failure.** Wire the
`!Send` Durable Object peer only at the *end* of a chain, never mid-chain. A
runner bound on `SendEventStore` (HS-S0109's named mutant) survives this
configuration, because nothing forces the `!Send` type through a spawned task.
Three peers, green, and the `Send` bound still excludes the hub from its own
topology — *"the exclusion is discovered when the adapter is written, not when
the runner is"* (`spec/SPECIFICATION.md:6350-6360`), and a leaf-only wiring
defers that discovery one more phase.

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

**Box 6, and this is the story that would expose a violation committed earlier.**
This story adds no new rule — it runs the existing suite against two more peers —
and that is precisely the check: a Postgres-backed peer assigns positions from a
sequence, so any rule that had quietly assumed dense positions from one would
pass against `MemorySyncPeer` and fail here, which is the behaviour
`GappedPositionStore` was built to simulate in advance
(`crates/happenstance-testkit/tests/mutation_coverage.rs:326-338`). If a rule
fails on the Neon leg for a position-shaped reason, the defect is in the rule and
the fix is in the rule. Additionally, no fixture may expose a foreign position as
a bare number for a rule to assert on; positions cross this boundary only inside
an `EventId`.

**Box 7.** This story edits no `[FROZEN]` clause. It is the first real test of
SY-15 and SY-17 against transports that can fail them, and its most likely
outcome that touches a clause is the opposite of an edit: a `Rejects:` field that
names a stand-in in `tests/real_peer_shapes.rs` becomes stale when the stand-in
is retired, which is a repair owned by `frozen-clause-repairs` (HS-S0112) under
`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`. ADR-0026 —
written first, in slice 1, and required by `RUNBOOK.md:462-466` to have been
written against **two** unlike peers — is the authorising decision. If a real peer
proves the port wrong, the output is a recorded finding, a new decision atom and a
re-plan, never a rule relaxed to let a peer pass.
