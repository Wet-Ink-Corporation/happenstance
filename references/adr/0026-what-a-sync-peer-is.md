# ADR-0026 — What a sync peer is: one relationship, one round trip, harmless re-delivery, and classified errors

- **Status:** accepted. The owner made the one new call on 2026-10-14 (`wi-cbaec7`, option A) and
  merges this record. It is `door:one-way`: an accepted record is immutable.
- **Date:** 2026-10-14
- **Phase:** 13 (`happenstance-sync` and its testkit). The work item is
  `runbook/phases/13-sync.md:85-89`; #119, WF-1's envelope section, is folded in
  (`runbook/phases/13-sync.md:136-138`).
- **Answers:** the question the original queue reserved this number for
  (`runbook/ledgers.md:52`): what is a sync *peer*, what may the port assume about a transport it
  cannot see, and what does ingest promise? And the one question the specification had left open
  inside it: how a peer's error tells the runner what to do next (#243, LiveStore audit
  finding L-16).
- **Supersedes:** nothing. The published-surface half was moved to phase 17 and settled there by
  [ADR-0073](../../.kb/decisions/0073-the-foreign-identity-write-path-is-the-adapters.md); this
  record cites it for that half.
- **Mints:** SY-36, `[PROVISIONAL]` (`spec/SPECIFICATION.md:8002-8040`).
- **Evidence:** a reading of `main` at `a69ded4`. Nothing new was measured. The phase-2 sketch
  is the type-level evidence; the LiveStore prior-art audit of 2026-10-11 (read at
  `livestorejs/livestore@3cb74e62` and `livestorejs/livestore-contrib@c375b5ab`) is the
  evidence about the rejected design. #247 files that audit under `references/evaluation/`; until
  it lands, its findings are cited here by their ids (L-01 …) and their upstream paths.

---

## 1. What is true today

### The two peers the port has to admit

Phase 13's proof artefact names two networked peers that are as unlike each other as the
deployment allows: a Durable Object reached over a socket, and a Postgres reached over one-shot
HTTP with no interactive transaction (`runbook/phases/13-sync.md:205-211`). Phase 2 stood both in
and pointed them at the port (`crates/happenstance-sync/tests/real_peer_shapes.rs:1-25`). The
table at the top of that file is the shape of the problem:

| | Durable Object | Postgres over one-shot HTTP |
|---|---|---|
| `Send` | no — `Rc`, `JsValue`, single-threaded | yes |
| ambient runtime | none to spawn into | tokio |
| state between calls | a live socket | nothing at all |
| cursor | possible | impossible |
| error | wraps a `JsValue` string behind `Rc` | wraps a status code |

(`crates/happenstance-sync/tests/real_peer_shapes.rs:7-13`.) A port the second column cannot
implement is a port shaped like the first.

### The sketch

`SyncPeer` and its derived `SendSyncPeer` (`crates/happenstance-sync/src/peer.rs:81-82`) have
four items: `type Error` (`:91`), `type Resume` (`:118`), `pull` (`:138`), `push` (`:148`) and
the synchronous `limits` (`:158`). Both stand-ins implement it — the Durable Object the bare
flavour, the HTTP peer the `Send` one. The cursor-shaped alternative was also compiled against
both, and **it compiled**: the one-shot HTTP peer satisfied `impl Stream` by buffering a whole
response into a `Vec` and replaying it (`crates/happenstance-sync/tests/cursor_shape_probe.rs:20-37`).
That result is the reason "one round trip, no held state" is a conformance rule (SY-15) rather
than a property of the signature.

### The question the specification left open

`type Error: core::error::Error + 'static` (`crates/happenstance-sync/src/peer.rs:91`) carries
no `Send` bound, for a measured reason: the Durable Object stand-in's error holds an `Rc<str>`
and fails `Send + Sync` with two `error[E0277]`s (`peer.rs:84-90`), which ADR-0009 generalises to
every port. It also carries no *class*. A runner holding a `P::Error` can display it and can
return it; it cannot tell what to do with it.

The sketch already knows the classes; it writes them in prose where no runner can read them:

- `SyncError::BelowRetentionFloor` is documented "Not retryable"
  (`crates/happenstance-sync/src/peer.rs:396`), and `SyncError::BatchTooLarge` "Retryable only
  after splitting" (`:404`).
- `MemoryPeerError::BatchTooLarge` says retrying "will fail identically; it has to be split"
  (`crates/happenstance-sync/src/memory.rs:62-74`).
- `PullBatchLimit::admits` says an oversized single event "returns `false` for ever", and that
  the runner has to tell "wait" from "this will never move" (`peer.rs:349-351`).
- VT-25 exists so that a sync runner can tell "this will never be accepted here, park it" from
  "the disk is full, retry" (`spec/SPECIFICATION.md:7519-7521`, inside SY-18).

## 2. The decision: a peer's error states its class

### 2.1 What the owner decided

`wi-cbaec7`, option A, verbatim: *"SyncPeer::Error gains a class() -> #[non_exhaustive]
FailureClass {Transient, Split, Park, Stop}; testkit rule checks every refusal's class."*

| Class | Meaning | Remedy |
|---|---|---|
| `Transient` | the exchange failed, and the same exchange can succeed later | retry, with jittered and capped backoff |
| `Split` | the batch is over a limit the peer has; its events are not | halve the batch and retry |
| `Park` | this event can never land on this peer — VT-25's `ExceedsStoreLimit` | hold the event, report it, carry on with the rest |
| `Stop` | authorisation or configuration | halt this edge and report it |

SY-33 permits exactly two kinds of refusal, transport and authorisation, and names their opposite
remedies (`spec/SPECIFICATION.md:7958-7981`): a transport refusal is retried, an authorisation
refusal is not. `Transient` and `Stop` are those two. `Split` and `Park` are the two capacity
outcomes SY-18 and VT-25 already distinguish. There is no class for a condition-level refusal,
because SY-33 forbids one; a peer that wants that class has found a violation.

### 2.2 The shape, and why it is a bound on the associated type

Sketched, not final; #243 chooses the identifiers:

```rust
pub trait Classify {
    fn class(&self) -> FailureClass;
}

#[non_exhaustive]
pub enum FailureClass { Transient, Split, Park, Stop }

#[trait_variant::make(SendSyncPeer: Send)]
pub trait SyncPeer {
    type Error: core::error::Error + Classify + 'static;
    // pull, push, limits as today
}
```

Three Rust choices in it, each with an alternative:

**A bound on `Error`, not a method on the peer.** The alternative is
`fn classify(&self, error: &Self::Error) -> FailureClass` on `SyncPeer`. It works only while the
peer is in hand. An error does not stay beside its peer: the runner logs it, boxes it into a
report, and returns it across an `.await` after the peer handle may have been dropped — which
SY-16 says is the normal termination path at the edge (`spec/SPECIFICATION.md:7448-7464`). A
bound on the associated type makes the class travel with the value. It is the same placement
ADR-0009 chose for the error's *thread-safety*: a property of the error, stated on the error.

**`Classify` carries no `Send` bound and is object-safe.** It has one `&self` method with no
generics, so `&dyn Classify` and `Box<dyn core::error::Error + …>` both still work, and the
Durable Object's `Rc`-holding error implements it unchanged. Adding `Send` here would undo the
measurement at `peer.rs:84-90` for no gain.

**`#[non_exhaustive]` on the enum.** Outside the defining crate, a `match` on a non-exhaustive
enum must have a wildcard arm, so adding a fifth class is not a breaking change for any runner or
adapter. The price is that the runner has to decide what an unknown class means; that decision is
#244's, with the rest of the runner's per-class behaviour. Without the attribute, a fifth class
after `happenstance-sync` is published would be a major release.

### 2.3 The class is a function of the error alone

SY-36 says so, and it is what makes the rule testable: the fixture declares, per refusal, the
class it should report, and the rule compares. It is also what the stand-ins show is needed. The
same Rust type spans classes:

- `OneShotHttpError::Status { code, .. }` (`crates/happenstance-sync/tests/real_peer_shapes.rs:131-141`)
  is `Stop` at 401 or 403, `Split` at 413, and `Transient` at 503. `Transport` is `Transient`.
  `ResponseTooLarge` is `Split`. `Malformed` is `Stop` — a peer that answers with something this
  side cannot parse is a version or configuration mismatch, and retrying it will not help.
- `DurableObjectError` (`real_peer_shapes.rs:54-62`): `SocketClosed` is `Transient`. `SqlStorage`
  is `Park` when SQLite refuses a value over its row limit and `Stop` otherwise. `JsError` is a
  stringified `JsValue`, and only the adapter that produced it knows which.

A runner-side table from error type to class could not be written for either, which is why the
adapter states the class and the runner never infers one.

### 2.4 `SyncError` states a class too

The runner's own enum (`crates/happenstance-sync/src/peer.rs:393-421`) implements the same trait,
so a runner handles its own failures and a peer's through one path: `BatchTooLarge` is `Split`,
`BelowRetentionFloor` is `Stop` (the gap needs a re-seed or an acceptance, `:396`), and
`UnknownPeer` is `Stop`. ADR-0016 left "`SyncError` extension" to this record
(`references/adr/0016-the-wire-format.md:11`); it gains no variant here. The message set it would
grow with is ADR-0027's.

### 2.5 The clause and the rule

SY-36 is minted `[PROVISIONAL]` (`spec/SPECIFICATION.md:8002-8040`). In this repository a marker
moves only when its rule exists in the tree, and `every_refusal_states_its_class` does not exist
yet: `happenstance-sync-testkit` is #111's. The rule drives the fixture peer into every refusal it
can produce and asserts each reports the class the fixture declares. Its mutant reports every
failure as one class — the wrong implementation the rule exists to reject, and LiveStore's shipped
one. The 1.0 disposition is `freeze-by-13`.

The falsifier is a refusal from a real peer — the Durable Object, the one-shot-HTTP Postgres, or
#121's KV-capped peer — whose right remedy depends on something the error does not carry: how
often it has been retried, or which batch carried it. That is `wi-cbaec7`'s flip condition. A
refusal needing a fifth remedy is not a falsifier, because the class set is non-exhaustive.

## 3. What this record does not decide again

Each row below is already in the specification, most `[FROZEN]`. They are written down here
because this is the record that was owed them, not because any is open.

| Question | Answer | Clause |
|---|---|---|
| May ingest refuse an event? | No. A conflict is compensated by an ordinary append, atomic with the losing event; the port supplies atomicity, identity and idempotence; nothing is quarantined; ingest appends at the tail; a wire condition is evidence; at most one peer authors a compensation per fact family | SY-1 – SY-7, `spec/SPECIFICATION.md:6962-7183` |
| What is a peer? | One relationship. Fan-out, ordering between peers and disagreement are a runner's policy | SY-8, `:7195-7213` |
| Is hub-ness a type? | No. A Durable Object can be a spoke to an estate store and a hub to 138 tablets | SY-9, `:7215-7233` |
| What makes re-delivery harmless? | It is a no-op, deduped on the store-assigned `EventId`, never on a hash of the wire bytes | SY-11 – SY-13, `:7260-7357` |
| What may the port assume about a transport? | Nothing held between calls: one round trip per method, resume state an owned value the caller supplies, and a bound on `EventStore` so the `!Send` peer mid-chain is admitted | SY-15 – SY-17, `:7417-7485` |
| How does a foreign identity get written? | Through the adapter's own row writer; `happenstance-core` grows nothing | VT-10, frozen by ADR-0073 |
| Do peers declare limits? | Yes, readable before a push. Whether that prevents the failure or relocates it is #121's experiment | SY-18, `:7487-7528`, `[DEFERRED]` |

SY-14 (bulk ingest in bounded round trips) and SY-18 stay with their own experiments. SY-19 – SY-31
are ADR-0027's.

## 4. The rejected design: one total order, and client rebase

LiveStore is a shipped, carefully documented library at the other end of the axis §5 decides
(audit finding L-01). Its backend arbitrates one total order per log and refuses any push that
is not a fast-forward (`sync-cf/src/cf-worker/do/push.ts:74-78`); clients then rebase their
pending events on top, rewriting local history (L-02; `01-syncstate/spec.md:98-105`,
`materialize-event.ts:207-223`).

Each half breaks a frozen clause here:

- **The refusal.** Whether the push is refused depends on what the receiver already holds. SY-33
  forbids exactly that refusal, and it needs one central writer, which SY-10's peer-to-peer half
  rules out.
- **The rebase.** A rebased event lands after facts its author's append condition never queried.
  In DCB terms the consistency decision it was written under quietly stops holding, and nobody is
  told. Rewriting the client's local history also breaks SY-5 (ingest appends at the tail) and
  SY-34 (a refused spoke keeps its log).

LiveStore's own RFC 0002 names the cost: *"Event rebasing gives us convergence without
correctness"* (`livestorejs/livestore@3cb74e62`, `contributor-docs/rfcs/0002-command-replay.md`).
Its proposed repair, replaying commands rather than events (L-03), is independent support for
SY-6: the condition's query is what an adjudicator needs, and happenstance already ships it as
`EventGroup::guard`. That citation belongs to ADR-0027, which owns SY-6's merge rule.

## 5. Two testing consequences from the same audit

**Device-local state goes through `ProjectionStore`, not into the log (L-22).** LiveStore has
client-only events: written to the device's log, never pushed, numbered on a separate counter.
Copying that breaks `[FROZEN]` SY-24, which says a peer forwards every event it holds
(`spec/SPECIFICATION.md:7683`). State that belongs to one device is derived state, and SY-31
already puts the per-peer watermark in a projection store for the same reason. If a domain needs
local-only *facts*, that is a case for SY-27's scoped replication, which ADR-0027 decides.

**A fault capability a fixture cannot provide is declined with a reason, never stubbed (L-28).**
LiveStore runs one provider conformance suite across seven providers, and `turnBackendOffline` is
a no-op for six of them, so drop-and-resume is tested on one
(`tests/sync-provider/src/sync-provider.test.ts`). `sync_peer_conformance!` (#111) takes the
event-store testkit's pattern: a fixture that cannot inject a disconnect declines the capability
and states why, and the rule reports that reason instead of passing. Otherwise
`resume_survives_a_dropped_peer_handle` (SY-16) can go green on a stub.

## 6. Envelope section: WF-1, DCB wire interop, renewed past 1.0

This is #119's record, and the one ADR-0066's dispositions point at
(`references/adr/0066-what-1-0-promises.md:150`; `runbook/ledgers.md`, the WF-1 row).

**Named.** WF-1 (`spec/SPECIFICATION.md:2060`) keeps happenstance's wire format private: the
`serde` encoding moves events between happenstance instances and is not an interoperability
surface.

**Deferred, past 1.0.** If interop is wanted, it is a separate `happenstance-dcb-interop` crate,
beside the peer adapters and outside both the port and the contract crate, with its own ADR. It is
additive, because WF-8 versions the envelope and the format is ours to extend
(`.kb/decisions/0016-the-wire-format.md`). Nothing in 1.0 waits on it.

**The experiment is a specific implementation.** ADR-0016's W7 reading found that the DCB
reference implementation publishes no wire format at all
(`.kb/open-questions/dcb-reference-publishes-no-wire-format.md`). That is still the reference's
position. A non-reference implementation now publishes a service one can bridge against: UmaDB, a
Rust DCB event store, serves a gRPC API whose protobuf definition ships in its `umadb-proto`
crate, and a third-party Elixir client already speaks it. The experiment that would settle WF-1
is therefore a bridge in `happenstance-dcb-interop` that reads a UmaDB server's log over
`umadb.proto` and ingests it through `IngestStore`, preserving the origin's identity and
payload bytes. Its question is whether a foreign log's identity and tags map onto VT-5's
`EventId` and VT-15's `Tags` without loss. That reading of UmaDB was a web search on 2026-10-14,
not a deep read of the proto; the bridge's own ADR does that.

**Why UmaDB's service does not fire the renewal now.** WF-1's renewal is against a DCB
implementation publishing a wire-level encoding *or* a user needing to read another
implementation's log. `umadb.proto` is one server's client API, not an interchange format other
DCB implementations agree on, and no user has asked to read one. It is named here so the first
user who does has a bridge target rather than a discovery.

## 7. Consequences

- Every peer author implements one more trait, for every error variant. The owner accepted that
  cost for a runner that recovers without a human.
- A runner never retries an oversized event for ever and never pages a human for a dropped
  connection; both are LiveStore's shipped failures.
- The specification gains SY-36, and its census moves from 205 clause IDs to 206: 27
  `[PROVISIONAL]`, ten of them in §5.
- #243 builds the trait and the rule; #244 builds what the runner does per class; #111 hosts the
  rule; #121's peer is SY-36's third instrument.

## 8. What reopens it

A real peer's refusal whose right remedy depends on context the error does not carry. Before
`happenstance-sync` is published, that reopens it cheaply (#243 calls it two-way until then);
after, it needs its own record.

## 9. Alternatives that lost

- **B — an opaque error.** The runner can treat every failure only one way. If it retries
  everything, an oversized event retries for ever and blocks the edge behind it; if it stops on
  everything, a dropped connection needs a human. LiveStore's core classifies failures as
  `IsOfflineError`, `UnknownError` and `ServerAheadError` (`common/src/sync/errors.ts:22-28`), but
  two of its four providers, Electric and S2, report everything as `UnknownError`, and as a result
  "error out instead of reconverging" (`livestorejs/livestore-contrib@c375b5ab`,
  `context/sync/electric/.delta/DELTA-001-no-rebase-signal.md`; S2 `DELTA-002`).
- **C — one concrete `SyncError`.** Dropping the associated type for one shared enum loses each
  adapter's own cause, its `sqlx` or HTTP error, which every other port in the workspace keeps.
  It would also put the Durable Object's `!Send` cause and the HTTP peer's `Send` one in one type,
  which ADR-0009's reasoning rejects.
- **A runner-side mapping from error type to class.** §2.3: one status type spans three classes.
- **A method on the peer instead of a bound on its error.** §2.2: the class has to survive the
  peer handle being dropped.
- **One total order with client rebase.** §4.
