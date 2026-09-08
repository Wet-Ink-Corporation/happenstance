# ADR-0025 — The Ladybug projection adapter: checkpoint placement, write vocabulary, and the blocking API

- **Status:** proposed
- **Date:** 2026-09-08
- **Phase:** 11
- **Supersedes:** nothing
- **Evidence:** [`experiments/ladybug-driver-probes/`](../../experiments/ladybug-driver-probes/README.md)

## The question

What must a projection adapter do to satisfy PS-1 on an engine with no
transaction handle type, a blocking driver and Cypher as its only mutation
surface — and which half of PS-4's falsifier can actually fire against it?

`RUNBOOK.md`'s ADR queue reserved **0025** for this and has done since the plan
was written. The number is taken from that reservation rather than from the end of
the sequence; `.kb/decisions/` and `references/adr/` together hold 0001–0024 and
0029–0059, and 0025–0028 are the four slots the queue holds open for phases 11,
13, 13 and 14.

## Context, and why the crate's own prose could not answer it

`crates/happenstance-ladybug` has been a skeleton with real associated types since
phase 2, deliberately: it exists to disagree with the type checker before it
exists to store anything. Its notes were calibrated against `lbug` **0.16.1**, and
three of the load-bearing ones turned out to be wrong about **0.20.3**. All three
were found by running the driver rather than by reading it, which is the reason
this record cites an experiment rather than a source file for its premises.

## Decision

### §1 The checkpoint is a node in the graph, over `UINT64`

`MERGE (c:__hs_checkpoint {projection: $id}) SET c.position = $position,
c.authority = $authority` — one node per `ProjectionId`, written as the **last
statement before `COMMIT`**.

In the graph rather than beside it, because a `BEGIN TRANSACTION` … `COMMIT` on
one connection is the only atomicity LadybugDB offers, and PS-1 requires the
read-model write and the checkpoint write to become durable together. A sidecar
file or a second store would put them in two failure domains, which is the
invariant's whole subject.

**`UINT64`, and `INT64` is refuted.** The probe stored `u64::MAX - 1` into a
`UINT64` column and read it back exactly, so `SequencePosition`'s `NonZeroU64`
fits with **no narrowing at all** — unlike `happenstance-sqlite` and
`happenstance-postgres`, where a signed `bigint` loses the top half of the domain
and both adapters carry a fallible converter for it.

Two consequences, and the first is a deletion:

- **`LadybugProjectionStoreError::PositionOutOfRange` goes.** Its stated cause
  does not exist on this engine and no code path can construct it. A variant no
  implementation can reach is decorative by this repository's own corollary about
  rules, applied to an error enum.
- **`MalformedCheckpoint` stays and narrows.** Its field changes from `i64` to
  `u64` and it means one thing: a stored `0`. That is a corrupt checkpoint rather
  than a missing one, and collapsing it into `Checkpoint::NeverRun` would silently
  replay a projection from event 1.

### §2 The write vocabulary is raw parameterised Cypher

`GraphStatement` as it already stands. It is the shape `Connection::execute`
hands you, and it is the only parameterised mutation surface `lbug` exposes.

A typed builder was considered and loses on the ordinary grounds: PS-9 says a
projection writes through the adapter's **inherent** API, so a builder binds
nobody outside this crate, and it would need a `raw()` escape hatch anyway — at
which point it is raw Cypher plus a maintenance burden.

**One asymmetry with `happenstance-sqlite` is inherited rather than chosen, and
is named here rather than left silent.** SQLite's `push` takes `&'static str`, so
a statement assembled at run time out of decoded event data — a SQL injection
whose source is the event log — does not compile; `push_raw_sql` is the
separately-named escape hatch, so reaching for it is a decision.
`GraphStatement::new` takes `impl Into<String>` and closes nothing. Phase 11
should mirror the seam on the push, not on the constructor, and this record is
the place that says the current constructor is the weaker shape.

### §3 Blocking-only. No `tokio`, no `spawn-blocking` feature at `0.2.0`

The adapter's methods block the caller's thread. `tokio` in a runtime-agnostic
adapter is the cost the crate's module documentation already names, and its
`[dev-dependencies]` entry is not a precedent for a normal one.

**And the module documentation's stated reason is wrong, which matters because
somebody will reach for it.** It says `spawn_blocking` is *"available here"*
because the store is `'static`. It is not available: `spawn_blocking` needs
`FnOnce + Send + 'static`, and every port method takes `&self`. Whether the
closure can be `'static` is a decision about the store's **fields** — an
`Arc<Database>` rather than a `Database` — not about the call site. §6 takes that
field decision for an unrelated reason, which is what would make the feature
possible later; it is not what makes it unnecessary now.

**The decision is made falsifiable for free.** `projection_store_conformance!` is
invoked twice: once under `__emit_projection_blocking`, which needs no runtime,
and once under the default tokio emitter. A store that reached for
`spawn_blocking` panics under the first.

Claim that narrowly. The tokio emitter expands to `#[tokio::test]`, which is a
**current-thread** runtime running one task, so blocking in place starves nothing
and the two emitters cannot diverge on account of blocking. What the pair
falsifies is **runtime-agnosticism**, not the cost of blocking. The cost claim
would need concurrent work on a current-thread runtime, which the conformance
suite does not have and which this record does not pretend to.

### §4 PS-4's second condition has two readings and they get different answers

PS-4's falsifier names *"if its write handle must exist before a traversal that
the projection's own logic depends on."*

**The Cypher-level reading — statement *n* matching what statement *n−1* wrote —
did not fire.** The probe opened a transaction, created a node and matched it
back inside the same transaction, and the write was visible. A deferred write set
answers this correctly because replay is one connection, one transaction, in
order.

State it narrowly: **PS-4's condition did not fire *for this adapter*, on this
engine.** That is not a discharge of a clause which generalises over write-behind
shapes this probe says nothing about, and phase 11's verdict must not read as one.

**The Rust-level reading — `apply` needing the traversal's *value* to decide what
to buffer next — is foreclosed by the port for every batch shape**, because
`Projection::apply` is synchronous and a traversal is I/O. That is a finding about
the port rather than about LadybugDB, and it is true of `happenstance-sqlite` and
`happenstance-postgres` identically.

### §5 `READS_THROUGH_BATCH` is `false`

For the reason both existing adapters give: a `GraphWriteSet` has been sent to the
engine exactly never, and answering from committed state is what PS-12 forbids by
name.

**A pre-registered falsifier naming `rebuild_is_chunk_size_invariant` would be
unreachable**, because that rule is gated on `READS_THROUGH_BATCH` and is skipped
outright for a fixture declaring `false`. It is deliberately not in §8's list; a
verdict rubric containing a condition that cannot fire is the decorative-rule
defect applied to the one document in this phase that must not have one.

### §6 The store owns an `Arc<Database>`, and a second handle is a second `Connection`

Forced by measurement rather than chosen. A second `Database::new` on the same
directory is **refused by a file lock**, so `SECOND_HANDLE` cannot be a second
`Database`. Two `Connection`s over one shared `Arc<Database>` work, and the second
observes what the first committed — which is the out-of-connection observability
PS-1's coupling is only visible through.

### §7 `commit`'s error path must not issue a `ROLLBACK`

The probe's most surprising result. On a statement error inside a transaction,
**LadybugDB aborts the whole transaction itself**: the read-model write made
earlier in the same transaction was already gone before any rollback was
attempted, and the subsequent `ROLLBACK` was **refused**, because there was no
longer a transaction to roll back.

So the error path reports the first error and does not try to clean up after it.
Issuing a `ROLLBACK` there produces a second error that masks the first, which is
the failure this section exists to prevent.

It is also good news for PS-1: atomicity is enforced by the engine rather than by
this adapter remembering to ask for it.

### §8 The verdict on phase 6's freeze, pre-registered

Committed **before** any body is written, so that it cannot be written to fit
whatever happens.

**Predicted capability profile** — the same profile `SqliteProjectionFixture`
declares, with each declension in this store's own words rather than byte-identical
to another's:

| Capability | Predicted | Because |
|---|---|---|
| `SECOND_HANDLE` | `SUPPORTED` | §6 — a second `Connection` over one `Arc<Database>` |
| `RESET_REFUSAL` | declined | this adapter holds no protection policy, exactly as the other two do not |
| `COMMIT_FAULT` | `SUPPORTED` | a `CREATE` against a pre-planted primary key raises; `MERGE` would not, because it matches rather than conflicts |
| `READS_THROUGH_BATCH` | `false` | §5 |

**"It did not hold" is any one of these**, each citing its clause:

1. A rule fails that `SqliteProjectionStore` and `PostgresProjectionStore` both
   pass, and the cause is the port rather than this adapter (PS-4, PS-6).
2. The capability profile differs from the prediction in either direction — a
   capability this record predicted and the adapter cannot offer, or one it can
   offer that the prediction missed (PS-18, CF-18).
3. `GraphWriteSet` needs a field the port cannot express, or `commit` needs an
   argument it is not given (PS-5, PS-11).
4. The `E0195` spelling trap fires for a third implementer (PS-34).

**"It held" is the absence of all four, and is a result** — it must be recorded as
one rather than as silence.

**What it would be worth is bounded, and this record says so up front.** PS-2
requires *two adapters at opposite ends of the batch-shape axis*. Ladybug is the
**fifth** owned-buffered-batch implementer, not the second shape: `MemoryProjectionStore`,
the testkit's buffering variant, `SqliteProjectionStore`,
`examples/outside-projection-adapter` and now this one all buffer. A fifth
agreement is weak evidence, and phase 10b established that the axis's other end is
not merely unbuilt but **forbidden by the port** for the two drivers PS-2 names.
Phase 11 fills the *write-vocabulary* axis — Cypher rather than SQL, a graph
rather than tables — and that is what its verdict should claim.

### §9 The build, and what it costs the gate

Measured, in [`experiments/ladybug-driver-probes/`](../../experiments/ladybug-driver-probes/README.md):
the driver arrives as a prebuilt **1.44 GB static archive** (cmake is not needed
and was never invoked) and needs an OpenSSL toolchain that no feature turns off.

That cannot sit on `cargo test --workspace --all-features`, which is gate step 3
and which *links*. `cargo check` and `cargo clippy` do not link, so the cost lands
on one step — and it lands hard, on every machine and all three CI platforms.

**Therefore the `lbug` dependency ships behind an off-by-default feature**, the
workspace steps exclude this crate, and the conformance run is a probed step that
prints `skipped` when the driver is not configured — the same shape `cargo deny`
already has. That is the crate's own founding argument (*"which is also the reason
this lives in its own crate rather than behind a feature flag"*) one level further
in, reached by measurement rather than by preference.

## Consequences

- Four prose defects in the crate become false statements a reader would act on
  and are corrected in the same pass: the `spawn_blocking` availability claim
  (§3), the `INT64` narrowing (§1), the build-from-source note (§9), and the claim
  that this crate is one of PS-9/PS-11's *data points* — it is evidence about
  those clauses' **cost**, and their falsifier names a second generic consumer,
  which is a different thing owned by a different phase.
- `crates/happenstance-ladybug/src/stand_in.rs` is deleted when `lbug` lands. Its
  four `Send`/`Sync` assertions are **re-pointed at the real types rather than
  deleted with it**, and that is an exit criterion rather than a step: they are
  the only artefact that re-checks the `SendProjectionStore` flavour choice after
  the swap, and they reach the same conclusion by a different mechanism —
  `unsafe_code = "forbid"` forced the stand-in to get `Send + Sync` by
  construction, where `lbug` gets it by `unsafe impl` over a C++ pointer.

## Falsifier

This record is reopened if the conformance run contradicts §8's prediction in any
of its four listed ways, or if a later adapter shows that the checkpoint-in-graph
placement forces a graph shape an application would not otherwise choose.
