---
item: HS-S0109
stage: discover
created: 2026-08-12T13:03:30.760Z
updated: 2026-08-12T13:03:30.760Z
template_sig: 86ce4036
rendered_sig: 069af8ae
---

# Discover — The runner keeps the constrained runtime its runtime

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: land the runner that fans out over peers and advances the owned resume token, bound on `EventStore`/`SyncPeer`/`IngestStore` and never on a `Send` flavour (one name of each pair per module), proved by a real `tokio::spawn` with the `!Send` peer sitting mid-chain rather than at a leaf, and by the `wasm32` gate steps actually running it | `_storymap.md`, *Slices* table, `runner-and-topologies` row 1 | The runner, plus the two instruments that make its bounds falsifiable |
| **Depends on `ingest-store-and-memory-peer-round-trip` (HS-S0102)** for a peer and an ingest path with real bodies to fan out over | `_storymap.md`, *Slices* `depends_on` | A runner over `todo!()` bodies proves nothing at runtime |
| **Depends on `gate-mounts-for-the-sync-suite` (HS-S0104)** for the two `wasm32` steps that make AC-009 a gate fact rather than a claim | `_storymap.md`, *Slices* `depends_on` | *"this is the file that decides whether that sentence is true"* (`_decomposition.md`, *Gate mounts*) |
| **AC-009** — the ingest path and the sync runner bind `EventStore`, not `SendEventStore`, and the whole path is *"built and exercised for `wasm32` inside the gate rather than asserted in prose"* | `project.md`, *Acceptance criteria*, AC-009 | Two halves: the bounds, and the compile that proves them |
| **SY-17 `[FROZEN]`** — the sync port and its runner MUST be written against `EventStore`, not `SendEventStore`; rule: *"the existing CLAUDE.md rule 4, checked by `happenstance-sync-testkit` compiling its own suite against a `!Send` fixture peer holding a `!Send` store behind an `Rc`"* | `spec/SPECIFICATION.md:6341-6360` | The clause's enforcement is a **compile**, and the compile belongs to the testkit rather than to a `#[test]` |
| SY-17's `Rejects:` — *"any runner that reaches for `SendEventStore` in order to `tokio::spawn` a per-peer task"*; the wasm32 requirement is stated but **understated**: in Kestrel Cold Chain the `!Send` peer sits in the **middle** of the chain, a spoke to the Neon estate store and a hub to 138 tablets, *"so a runner bound on the `Send` flavour excludes the hub from its own topology, and the exclusion is discovered when the adapter is written, not when the runner is"* | `spec/SPECIFICATION.md:6350-6360` | The failure is **deferred**, which is why a green gate at this story does not settle it |
| **CLAUDE.md binding constraint 4** — bind `EventStore`, not `SendEventStore`; it is the weaker requirement and accepts both flavours. Import only one of the two names per module or method calls go ambiguous with `error[E0034]` | `CLAUDE.md`, *Binding constraints* 4; `crates/happenstance-sync/src/peer.rs:26-28` | A module-level import discipline with a compiler error attached. It applies to all three pairs here |
| **CLAUDE.md binding constraint 3** and its history: an assertion on a *concrete* stream passed by auto-trait leakage whatever the trait said; `send_flavour_stream_is_send_in_generic_code` writes the bound **at the definition**, and `spawns_from_generic` is what rejects the refactor, *"because it holds the stream across an await inside a real `tokio::spawn`"* | `CLAUDE.md`, *Binding constraints* 3 | The workspace has already learned that a type-level `Send` assertion can be satisfied by the wrong thing. Reuse the fix, not the mistake |
| *"a green `#[tokio::test]` says nothing about `Send`"* — a test that never spawns proves nothing about the property | `standards/rust/62-doctests-and-harnesses.md` RS-62-3; `_decomposition.md`, testing brief *Integration* | The AC-009 tripwire needs a **real spawn**, not a type check and not a multi-thread runtime flag |
| The `Resume` gap, stated at the port and named as a gap rather than a decision: *"`trait_variant` marks the derived futures `Send` and leaves associated types alone, so a runner that wants to `tokio::spawn` a per-peer task and carry the token out of it needs to write `S::Resume: Send` at its own bound. `E0277` when it does not"* | `crates/happenstance-sync/src/peer.rs:107-118` | The runner will meet this immediately, and the *fix* to it is the exact shape that tempts a blanket `Send` bound |
| **SY-8 `[FROZEN]`** — fan-out, ordering between peers and reconciliation of disagreement MUST live in a runner above the port; a rule needing two peer handles would be testing the runner | `spec/SPECIFICATION.md:6070-6086` | The runner is where the policy goes, and it is deliberately outside the conformance suite's grain |
| **SY-16 `[FROZEN]`** — resume state is an owned, transferable value the caller supplies on each call; `Rejects:` an in-memory cursor, *"which passes every test written against a process that stays alive and fails the deployment the crate exists for"* | `spec/SPECIFICATION.md:6323-6340`; `crates/happenstance-sync/src/peer.rs:95-118` | The runner owns the token, and the token must survive the peer handle being dropped and reconstructed |
| **ADR-0001 / ADR-0009** — no `#[async_trait]`, two flavours via `trait_variant`; `Error` keeps exactly `core::error::Error + 'static` on every port and flavour, with the stronger property in a downstream marker trait | `.kb/decisions/0001-async-port-flavours.md`; `.kb/decisions/0009-error-send-sync.md`; `spec/SPECIFICATION.md:7006-7019` | *"the risk is a runner that adds `+ Send + Sync` on the way past"* (`_decomposition.md`) |
| The runner's shape is deliberately **not** prescribed: fan-out, ordering and merge policy sit above the port, in the same division of labour that puts the projection runner above `ProjectionStore`; whether that is one type, a trait or a function is open, but *"add a second peer" must stay a runner configuration rather than a breaking change to the port* | `crates/happenstance-sync/src/lib.rs:42-50`; `crates/happenstance-sync/src/peer.rs:65-73`; `_decomposition.md`, *Non-prescriptive implementation notes* | Real design latitude, with one invariant attached |
| `AC-A07` — no `SendEventStore`, `SendSyncPeer` or `SendIngestStore` appears in a generic bound on the ingest path or the runner, and no module imports both names of a pair | `_decomposition.md`, AC-A07 | Two greppable, checkable conditions |
| The `wasm32` steps this story's proof rides on: a build of `happenstance-sync` **and** a check of the sync conformance harness, beside the four that exist | `xtask/src/main.rs:203-283`; `_decomposition.md`, *Gate mounts*, third bullet | A crate build is not a harness check; a harness behind `cfg(target_arch = "wasm32")` compiles to nothing natively |

## Questions

**Does ingest re-check the writer's asserted append conditions? — Answered
elsewhere; the runner's obligation is not to reintroduce it.** SY-1 forbids
refusal as a function of local state. A runner that retried, quarantined or
back-pressured on a per-event basis in response to a receiver's opinion would
recreate the rejection path above the port, where no clause is looking. SY-4
forbids quarantine outright (`spec/SPECIFICATION.md:5932`).

**Hub-and-spoke versus peer-to-peer — deferred to
`hub-and-spoke-and-peer-to-peer-topologies` (HS-S0110), and this story must not
foreclose it.** SY-9 is `[FROZEN]`: hub-ness is an edge property, so the runner
must not take a role parameter or hold two peer collections typed by role. SY-10
`[PROVISIONAL]` is where the abstraction question lives and ADR-0027 answers it;
this story builds whatever shape that atom landed and no more.

**What is the runner — a type, a trait, or a function? — Deferred to spec, with
one invariant.** `crates/happenstance-sync/src/lib.rs:42-50` and
`peer.rs:65-73` settle only that policy lives above the port and that adding a
second peer stays a configuration change. Anything satisfying that is admissible.

**Does the runner need `S::Resume: Send`? — Answered yes if it spawns, and this
is the exact place the mistake happens.** The port documents the gap:
`trait_variant` marks derived futures `Send` and leaves associated types alone,
so a spawning runner writes `S::Resume: Send` **at its own bound** and gets
`E0277` when it does not (`crates/happenstance-sync/src/peer.rs:107-118`). The
correct fix is a bound on the *token*; the tempting fix is a bound on the *store
and the peer*, which is SY-17's named rejection. Spec must write the narrow one
and say why the broad one was refused.

**How is the `!Send` chain proved? — Answered: by a real `tokio::spawn` with the
`!Send` peer mid-chain, plus the `wasm32` harness compile.** Not by
`fn assert_send<T: Send>()` on a concrete type, which the workspace has already
seen pass by auto-trait leakage while the trait said otherwise
(`CLAUDE.md`, binding constraint 3), and not by a `#[tokio::test]` that never
spawns (RS-62-3). Mid-chain, not at a leaf, because that is the arrangement that
tempts the wrong bound.

**Does the runner get conformance rules? — Deferred, and the default is no.**
SY-8 says a rule needing two peer handles would be testing the runner
(`spec/SPECIFICATION.md:6074-6077`), so runner behaviour is integration-tier in
`happenstance-sync`'s own `tests/` rather than in the suite. SY-17 is the
exception and it is discharged by a **compile** in the testkit, not by a rule.

## Decision

Fan-out, ordering and merge policy have been deliberately kept off the port since
phase 2, on the argument that policy on the port makes every adapter author
inherit the merge problem — so a runner has to exist for the port to be finished,
and it does not exist yet. The runner is also the single place in this workspace
where binding constraints 1 and 4 are most likely to be broken for a good reason:
the natural implementation spawns a task per peer, `tokio::spawn` demands `Send`,
and the shortest path to a compile is to bind the `Send` flavours of all three
ports. SY-17 is `[FROZEN]` against exactly that, and its reasoning is the part
worth carrying: the `!Send` peer is not at a leaf of the topology but in the
**middle** of it — a Durable Object that is a spoke to a Neon estate store and a
hub to 138 tablets — so a `Send`-bound runner excludes the hub from its own
topology, and *"the exclusion is discovered when the adapter is written, not when
the runner is"*. This story lands the runner on the weaker bounds and, more
importantly, lands the two instruments that would fail if it had not: a real
`tokio::spawn` with a `!Send` peer mid-chain, and a `wasm32` compile of the sync
conformance harness inside the gate. The spec stage will cover: the runner's
shape, given ADR-0027's topology answer; its generic bounds and the per-module
import discipline that keeps `E0277`/`E0034` out; the narrow `S::Resume: Send`
bound and the refusal of the broad one; the resume token's ownership and its
survival across a dropped peer handle (SY-16); the mid-chain spawn test; and the
two `wasm32` steps that must actually compile this code.

## The wrong implementation

**`SendBoundRunner`, and its distinguishing feature is that this story's own gate
cannot fail it.** Write `where S: SendEventStore, P: SendSyncPeer, I:
SendIngestStore` so each per-peer task can be `tokio::spawn`ed. Everything is
green: the native test suite passes, because every peer in the tree at this
point is `Send`; `cargo clippy -D warnings` passes; and — this is the part that
matters — **the `wasm32` build of `happenstance-sync` also passes**, because a
generic function's `Send` bounds are satisfied at instantiation and a library
crate that never instantiates the runner with a `!Send` type compiles happily for
any target. The gate goes green on a runner that has already excluded the
Cloudflare peer, and nothing says so until HS-S0111 tries to write that peer and
meets `error[E0277]` in a crate it does not own. This is SY-17's named rejection
verbatim and it is a *deferred* failure, which is why the instrument has to be an
instantiation rather than a build: `happenstance-sync-testkit` compiling its own
suite against a `!Send` fixture peer holding a `!Send` store behind an `Rc`
(`spec/SPECIFICATION.md:6345-6349`) is the only thing that turns the exclusion
into a compile error **here**.

**`AssertSendRunner` — the mutant that is a test rather than a runner, and this
workspace has shipped it once.** Prove the `!Send` claim with
`fn assert_send<T: Send>() {}` applied to a concrete runner future, or with a
`#[tokio::test(flavor = "multi_thread")]` that awaits the runner without spawning
it. Both are green and neither proves anything: the first passes by auto-trait
leakage on a concrete type whatever the trait declared — the exact defect
`send_flavour_stream_is_send_in_generic_code` was written to replace by putting
the bound **at the definition** — and the second is RS-62-3's *"a green
`#[tokio::test]` says nothing about `Send`"*. The workspace's own answer is that
it takes **both** shapes and that `spawns_from_generic` is the one that rejects
the refactor, because it holds the value across an await inside a real
`tokio::spawn` (`CLAUDE.md`, binding constraint 3). The sync version needs the
same pair, with the `!Send` peer in the middle rather than at the end.

**`HeldCursorRunner` — the resume-token mutant, invisible in a process that stays
alive.** Keep the resume token inside the peer object, or inside a long-lived
per-peer task's local state, and advance it in place. Every test passes, because
every test runs in a process that lives longer than the exchange. In the field the
normal termination path *is* the handle going away — a Worker cancelled
mid-flight, a Durable Object evicted, a tablet losing signal — so the token that
"advanced correctly" never existed anywhere durable, and the peer resumes from
whatever it last persisted, which is nothing. SY-16's rule
`resume_survives_a_dropped_peer_handle` is the generalisation — drop the peer
between two exchanges, reconstruct it, resume from the token, assert **no gap and
no duplicate** (`spec/SPECIFICATION.md:6327-6331`) — and the runner is what has to
hand the token back out for that to be possible.

**And the mutant that puts rejection back above the port.** A runner that catches
an ingest error, parks the batch, and retries it later — or that back-pressures a
peer whose events "conflict". It never calls a conditional append, so
`ingest_never_rejects` is green; the rejection has simply moved to where no clause
is looking. SY-4 forbids quarantine (`spec/SPECIFICATION.md:5932`) and SY-1's
convergence argument applies to the whole path, not to one call site.

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

**Box 6.** This story adds no conformance rule — SY-8 keeps runner behaviour out
of a suite whose rules each take one peer handle. Its integration tests do assert
on progress, and the anchoring discipline holds in the strongest form this project
has: a runner's notion of progress is the **resume token**, which is opaque and
per-peer by design (`crates/happenstance-sync/src/peer.rs:95-106`), so the tests
assert token advancement and receiver contents keyed by `EventId`, never a
position value and never a comparison between two peers' positions — SY-19 says
no expression relates them (`spec/SPECIFICATION.md:6421-6428`).

**Box 7.** This story edits no `[FROZEN]` clause. It implements SY-8, SY-16 and
SY-17, and SY-17's frozen `Rule:` field is what dictates the shape of its
strongest instrument. The runner's design latitude is authorised by ADR-0026 and
ADR-0027, both written first in slice 1. If the `S::Resume: Send` gap turns out to
need a port change rather than a runner-side bound, that is
`RUNBOOK.md:462-466`'s residual risk landing on a `[FROZEN]` clause: stop, record
the finding, and raise a new decision atom and a re-plan — not a `Send` flavour
quietly bound one level up.
