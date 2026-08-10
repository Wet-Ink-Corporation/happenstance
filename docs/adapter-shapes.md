# Adapter shapes

What six skeletons told the type checker, and what the type checker said back.

## What this document is, and what it is not

Phase 2 built six adapter skeletons because **a port's shape is falsified by a
type, not by a behaviour**. Every crate below declares real associated types and
`todo!()`s its bodies. That split is the whole design: a skeleton that stubs its
associated types has stubbed the only part of it a type checker can disagree
with.

**This document is not the proof.** A document that records outcomes exists
whatever the outcomes are, and the previous revision of the runbook named one as
phase 2's proof artefact and thereby named something that survives any result.
What cannot be faked is six type checkers agreeing. This file is the *evidence
base* — where the transcripts are pasted so ADR-0009 through ADR-0017 can quote
them instead of re-deriving them.

Two kinds of row, and the second kind is why the first is not enough:

- **Signature rows** — what each skeleton declared, the most ambitious signature
  attempted, and what happened. *Both outcomes are results.* An earlier draft of
  phase 2 required a rejection per skeleton, which is an exit criterion that
  mandates its own outcome; `sqlx::Transaction<'static, Postgres>` **agreeing**
  with the owned-batch hypothesis, from a networked pool that had every
  opportunity to hand back a borrowed handle, is exactly the finding that rule
  would have suppressed.
- **Capability rows** — limits that are **not type errors**.
  `happenstance-neon` compiles against signatures it cannot honour, and a table
  showing only `error[E….]` would rank it the most compatible adapter in the
  workspace when it is the least.

Every skeleton is `publish = false` and carries a scoped
`#![allow(clippy::todo)]` naming the phase that removes it.

---

## 1. The portfolio at a glance

| Crate | Port and flavour | `Error` | `Batch` | Read stream | Target |
|---|---|---|---|---|---|
| `happenstance-sqlite` | `SendEventStore` + `SendProjectionStore` | `SqliteEventStoreError` (7 variants over `rusqlite::Error`, `JoinError`, `TryCurrentError`) | `SqliteBatch` — owned buffer of `(SQL, Vec<rusqlite::types::Value>)` | `SqliteReadStream` — hand-written 4-state machine | host |
| `happenstance-cloudflare` | **bare `EventStore` only** | `CloudflareEventStoreError` — genuinely `!Send` | — | `SqlRowStream` | `wasm32-unknown-unknown` + host |
| `happenstance-ladybug` | `SendProjectionStore`, **twice** | `LadybugProjectionStoreError` | `GraphWriteSet` (owned) *and* `GraphWriteHandle<'a>` (borrowed) | — | host |
| `happenstance-postgres` | `SendEventStore` + `SendProjectionStore` | `PostgresEventStoreError` | `sqlx::Transaction<'static, Postgres>` | `PgReadStream` — pooled cursor | host |
| `happenstance-neon` | **bare** `EventStore` + `ProjectionStore` | `NeonError<T::Error>` | `NeonWriteBatch` (owned) | `NeonReadStream` — buffered, not a cursor | host **and** `wasm32` |
| `happenstance-sync` | `SyncPeer` / `IngestStore` sketch | per-peer, no `Send` bound | — | — | host + `wasm32` |

The two `wasm32` claims are no longer prose: `cargo xtask ci` carries
`wasm32 build of the Cloudflare adapter` and `wasm32 build of the Neon adapter`
as mandatory steps. Until those existed, both crates asserted a target in their
own rustdoc and nothing checked it.

---

## 2. Signature rows

### 2.1 The three attempts phase 2 was required to make

#### `type Batch<'a> = rusqlite::Transaction<'a>` on `SendProjectionStore` — **rejected, for two independent reasons**

Reproduced against `rusqlite 0.40.1` on rustc 1.97.1, as two impls in one scratch
test: a store owning a `Connection` directly, and a store wrapping one in a
`Mutex` but binding the live transaction as its batch. **Six errors, and the
split is the finding.**

*Reason 1 — the store.* `rusqlite::Connection` is `Send` and **not** `Sync`, so
`&Self` is not `Send`, so every future in the trait is rejected — **including
`checkpoint`, which never touches a batch at all**. Four errors:

```text
error: future cannot be sent between threads safely
  --> tests\zz_scratch_gat.rs:26:54          (checkpoint)
   = help: within `DirectStore`, the trait `Sync` is not implemented for
           `RefCell<rusqlite::inner_connection::InnerConnection>`
  --> crates\happenstance-core\src\projection.rs:70:44
```
…and the same at `:30` (`begin`), `:39` (`commit`) and `:43` (`rollback`).

*Reason 2 — the batch.* Wrap the connection in a `Mutex` and the store becomes
`Sync`; `checkpoint` and `begin` now compile. `commit` and `rollback` still do
not, because `rusqlite::Transaction<'_>` is itself `!Send` and they are rejected
**on the parameter alone**. Two errors:

```text
error: future cannot be sent between threads safely
  --> tests\zz_scratch_gat.rs:78:10          (commit)
   = help: within `Connection`, the trait `Sync` is not implemented for
           `RefCell<rusqlite::inner_connection::InnerConnection>`
```

Reason 1 is fixable with a `Mutex` — and the shipped adapter does exactly that.
Reason 2 cannot be fixed at all without giving up the live handle. That is why
`happenstance-sqlite` buffers.

**These six diagnostics carry no error code.** `--message-format=json` reports
`code: None` on every one. That is not sloppiness in the transcript: there is no
code to quote. It generalises ADR-0008's PS-36 finding from one diagnostic to
this whole family, and it has a consequence — a `compile_fail,E….` doctest
cannot pin any of them, so pinning needs a `trybuild`-style stderr snapshot,
which is the dependency decision **phase 6 owns**.

#### A two-statement probe-then-write `append` on `happenstance-neon` — **accepted, and that is the result**

`ProbeThenWriteStore` (`crates/happenstance-neon/src/event_store.rs:405`)
implements `EventStore` in full, with a compiling call site, and is **silently
wrong**. Two statements are two round trips, each its own implicit transaction,
with a network-latency-wide window between them and no snapshot spanning it. A
conflicting append committed inside that window is invisible to the probe and
unopposed by the insert.

The port cannot reject it. This is the single strongest argument in the wave for
the capability rows in §3 existing at all.

A second, unlooked-for result came with it: **the condition and the write do
collapse into one statement, and the collapse keeps
`ConditionViolated::conflicting_position`.** A CTE computes the probe and the
insert on one snapshot and projects both
(`crates/happenstance-neon/src/event_store.rs:130-165`). The decision ledger
names Neon as the forcing case for demoting `conflicting_position` to a hint;
**it does not force it.** The price is one extra aggregate index scan on every
append including uncontended ones, plus `Serializable` isolation for soundness —
both real, neither a `None`. This bears on the ledger's open
`conflicting_position` row and on ADR-0012.

#### `Error: Send + Sync` on the Cloudflare skeleton — **rejected, and the rejection is narrow**

Adding `+ Send + Sync` to `EventStore::Error` in `happenstance-core/src/store.rs:100`
and running `cargo check --workspace --all-features`: **every crate in the
workspace still compiles except `happenstance-cloudflare`**, which fails with
four `error[E0277]`.

```text
error[E0277]: `Rc<str>` cannot be shared between threads safely
   --> crates\happenstance-cloudflare\src\event_store.rs:147:18
    |
147 |     type Error = CloudflareEventStoreError;
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: within `CloudflareEventStoreError`, the trait `Sync` is not
            implemented for `Rc<str>`
note: required because it appears within the type `JsHandle`  (js.rs:52)
note: required because it appears within the type `JsThrow`   (js.rs:101)
note: required because it appears within the type `SqlError`  (sql_storage.rs:95)
note: required by a bound in `happenstance_core::EventStore::Error`
   --> crates\happenstance-core\src\store.rs:100:45
```

Two sites × `Send` and `Sync` each. The second site is `send_shape.rs:138`, and
it matters: the bound is reported against **`SendEventStore::Error` as well as
`EventStore::Error`**, from one edit to one declaration. That is ES-5's claim —
the associated type cannot be varied between flavours — observed rather than
argued.

The full transcript and the ES-6 argument it feeds are in
[ADR-0009](../.kb/decision/0009-error-send-sync.md).

### 2.2 The attempts nobody required, which produced more

#### `sqlx::Transaction<'static, Postgres>` — **accepted**

`happenstance-postgres` binds an owned, `Send`, `'static` transaction to today's
GAT (`projection_store.rs:100`). The lifetime parameter is *satisfiable and
unused*. This is the one adapter with a pooled network connection and a real
interactive transaction, so it is the one that could most plausibly have demanded
a borrow — and it did not, because `Pool::begin()` hands back a transaction that
owns its `PoolConnection`.

#### `GraphWriteHandle<'a>` — **accepted, and it complicates phase 6**

`happenstance-ladybug` ships the owned `GraphWriteSet` as its working hypothesis
*and* a second, compiling, GAT-**borrowed** impl
(`live_handle.rs:174`), exercised across a real `tokio::spawn`. So the borrowed
GAT does **not** fail on the `Send` flavour.

That is a result, and it cuts against §4.2 of the specification, whose argument
for dropping the GAT is derived from rusqlite alone. **Phase 6 must weigh it**:
the rusqlite rejection above is about `rusqlite::Connection` being `!Sync`, not
about GATs, and a different driver with a `Send` handle keeps the borrow.

#### Omitting `where Self: 'a` on the impl — **accepted**

`happenstance-sqlite` records that rustc accepts the clause present or absent
when the bound type is owned (`projection_store.rs:214-217`).

#### Writing the concrete type instead of `Self::Batch<'_>` — **rejected, `error[E0195]`**

Binding an owned type buys **none** of the E0195 relief PS-5 promises: the trait
method still declares `batch: Self::Batch<'_>`, so writing `batch: SqliteBatch`
in the impl is `error[E0195]: lifetime parameters or bounds on method 'commit' do
not match the trait declaration`. The literal `Self::Batch<'_>` stays mandatory
until the GAT leaves the port itself. PS-34 is therefore still binding, and
ADR-0008 was right that the trap outlives the change everyone expects to retire
it.

#### A second `SendEventStore` impl beside the bare one — **rejected, `error[E0119]`**

`happenstance-neon` records that adding
`impl<T: SqlTransport + Send + Sync> SendEventStore for NeonEventStore<T>`
alongside the bare impl gives `error[E0119]: conflicting implementations of trait
EventStore`, naming `trait_variant`'s blanket impl. **The two flavours are
mutually exclusive per type**, not a lattice an adapter can sit at two points of.
An adapter that wants both must be two types.

---

## 3. Capability rows — limits that are not type errors

This is the half a `cargo check` cannot produce.

| Crate | Limit | Why no type error |
|---|---|---|
| `happenstance-neon` | Probe-then-write `append` races | Two round trips, two implicit transactions, no spanning snapshot. Compiles perfectly (§2.1) |
| `happenstance-neon` | No cursor — a read is one buffered JSON document | `NeonReadStream` satisfies `Stream` by draining a `Vec`. Laziness is honoured in *form*; a million-event replay is buffered whole |
| `happenstance-neon` | 64 MiB hard response cap | A read whose result set exceeds it fails as a transport error, not a stream that yields fewer items |
| `happenstance-neon` | No connection, no interactive transaction | Nothing in the port asks for one, so nothing rejects its absence |
| `happenstance-neon` | Exactly one round trip per operation | Any port method that assumed a peer could hold state open between calls would exclude this adapter silently |
| `happenstance-neon` | Owns no HTTP client | The host needs a TLS stack, `wasm32` needs `fetch`; the crate proves the shape above them needs no opinion, **not** that a licence-clean client exists for both |
| `happenstance-cloudflare` | A lazy read stream is not a stable snapshot | Cloudflare documents that a `SqlStorageCursor` held across an `await` gives no stable snapshot. ES-9 requires laziness, so laziness and snapshot isolation cannot both be honoured — and the port cannot see the trade |
| `happenstance-cloudflare` | Positions bounded by 2^53, not 2^64 | Workers SQL widens integers through a JS number. `NonZeroU64` permits positions this adapter cannot round-trip |
| `happenstance-sqlite` | `read` must not spawn | `spawn_blocking` **panics** with no runtime in scope, and `read` is not `async`, so it runs on whatever thread called it. The spawn is deferred into `poll_next`; laziness stops being a nicety and becomes load-bearing |
| `happenstance-sqlite` | Serialises its writers by construction | The `Mutex` that buys `Self: Sync` is the same `Mutex` that funnels every writer. It is what makes the `Send` flavour implementable at all |
| `happenstance-postgres` | Positions allocated outside the transaction | `nextval()`. Nothing in the port's *types* can express the visibility invariant it breaks — which is why it is measured rather than compiled: see §4 |
| `happenstance-sync` | A peer cannot be asked to hold state open | `SyncPeer::pull` returns a bounded batch and an owned resume token rather than a stream. **The type checker did not force that choice**, and the transcript showing it did not is the most useful thing the sketch produced |

---

## 4. The one thing that is a number, not a type

Position allocation is the axis the pressure test ranked first, and the only one
where what is missing is a measurement rather than a signature. `nextval()`
allocates outside the transaction, so a Postgres store violates ES-10's
visibility invariant by construction unless it buys its way out.

The experiment — four arms, a deterministic two-connection inversion detector,
and `pgbench` throughput ratios against an unguarded baseline on the same
instance — lives in
[`docs/experiments/position-visibility/`](experiments/position-visibility/README.md).
**ES-10 is affordable, and `xid8` + `pg_snapshot_xmin` is the mechanism.**

| Arm | Shared-key writers | Disjoint-key writers | Throughput at 64 |
|---|---|---|---|
| Baseline — `nextval()` | **inversion** | **inversion** | 1.00 |
| A — serialised sequence table | pass by *serialisation* | pass by *serialisation* | **0.062** |
| B-const — advisory lock, constant key | pass by *serialisation* | pass by *serialisation* | **0.033** |
| B-tag — advisory lock, tag-derived key | pass by *serialisation* | **inversion** | 0.935 |
| C — `xid8` + `pg_snapshot_xmin` | **pass** | **pass** | 1.026 |

Both positive controls fired: the baseline inverts (`A=6, B=7, H=7` — 6
materialises beneath an already-observed 7), and arm A at 64 writers collapses to
674 tps against 10,844, with p99 60× worse. An arm that cannot reproduce the
inversion is not measuring anything, and a serialisation point that does not hurt
means the server was never loaded.

Three findings the freeze should carry:

- **B-tag is cheap because it buys a weaker invariant.** With disjoint keys it is
  byte-identical to the baseline. It buys a **per-boundary** invariant; ES-10
  states a **global** one. Whether the global statement is what happenstance
  actually needs — DCB evaluates conditions against a boundary — is a clause
  question this experiment deliberately leaves open for phase 4.
- **Arm C's cost is real but structural, not throughput.** `head` becomes a
  frontier rather than a maximum, read-your-own-writes does not hold, and
  staleness is bounded by **the longest open write transaction anywhere in the
  cluster**: a five-second write in an *unrelated database* moved the frontier
  from 0.7 ms to 4,010 ms. That is a capability row, not a number.
- **A and B-const are correct and unusable**, at 16× and 30×. They buy ES-10 by
  deleting the reason `happenstance-postgres` is in the tree — it is the
  workspace's only non-serialising writer.

A methods note worth keeping, because it nearly produced a wrong answer: the
sequential design this phase specified — measure the arms, re-run the baseline
last to bound drift — **failed**. The baseline moved 2.7× at one client and 3.0×
at 64, larger than two of the three effects it was meant to measure. The numbers
above come from a **paired** design instead, re-measuring the baseline between
every pair of arms; residual drift is 1.03–1.13×. The discarded pass is kept in
`results/discarded-sequential/` rather than deleted.

Note what the experiment is *not*: it is not `happenstance-postgres`, which is
phase 10, and it is not CF-13's conformance fixture, which is phase 3. It runs
CF-13's **predicate** against a real server, because an in-memory Rust mutant
store cannot exercise `pg_snapshot_xmin`.

---

## 5. What a skeleton does not prove

CF-25 forbids declaring a port frozen while an axis has no **passing**
implementation at its far end. Six skeletons do not fill six far ends, and the
distinction is the difference between an instrument and a target.

| Axis | What the skeleton settled | What is still empty |
|---|---|---|
| Position allocation | That `nextval()`'s cost is a measurement, not a signature — the port's types cannot express the invariant | A Postgres adapter that **passes** the visibility rule. Phase 10 |
| Transport | That a store with no connection, no cursor and no interactive transaction satisfies today's `EventStore` — and that this is a capability limit rather than a type error | A one-shot HTTP adapter that passes the suite. Phase 10 |
| Async flavour | That a genuinely `!Send` error and a genuinely `!Send` store compile against the bare flavour, on `wasm32` | A Durable Object with real `worker` bindings. Phase 9 |
| Batch shape | That an owned batch works for SQL, HTTP **and** a graph handle — and that a *borrowed* GAT also still works | The projection conformance suite, which does not exist. Phase 6 |
| Handle multiplicity | Nothing. No skeleton has two handles onto one backing store | Phase 3's fixture, phase 8's real store |
| Durability | Nothing. Every body is `todo!()` | Phase 8 |
| Completeness | Nothing | Phase 14 |

A skeleton falsifies a signature. A far end is a passing implementation, and
phase 2 produced none.

---

## 6. A rustc bug, recorded so it does not expire

`happenstance-ladybug` found a **reproducible internal compiler error** while
attempting a third batch shape: a store that is not `'static` implementing
`SendProjectionStore`.

```text
thread 'rustc' panicked at compiler\rustc_trait_selection\src\errors\note_and_explain.rs:27:22:
DefId::expect_local: `DefId(… happenstance_core[…]::projection::SendProjectionStore::commit)` isn't local

query stack during panic:
#0 [compare_impl_item] checking assoc item `<impl …>::commit::{anon_assoc#0}` is compatible with trait definition
#1 [check_well_formed] checking that `<impl …>` is well-formed

note: rustc 1.97.1 (8bab26f4f 2026-07-14) running on x86_64-pc-windows-msvc
```

rustc crashes **while diagnosing a region error** — the underlying user error is
genuine, and rustc reports it cleanly as `E0477` when the trait is local. The
crash is in the path that explains a lifetime bound for an item in a *foreign*
crate, which is every adapter in this workspace by construction.

Minimised to two crates, no dependencies, no macros, and — contrary to the
upstream report — **no `async` and no `Send`**:

```rust
// tcrate.rs
pub trait Store {
    type Batch<'a> where Self: 'a;
    fn commit(&self, batch: Self::Batch<'_>) -> impl Sized;
}
// icrate.rs
pub struct Borrowing<'a>(&'a ());
impl tcrate::Store for Borrowing<'_> {
    type Batch<'a> = () where Self: 'a;
    fn commit(&self, _batch: Self::Batch<'_>) -> impl Sized {}
}
```

Five ingredients, each independently necessary: the trait in a **different
crate**; the GAT's **`where Self: 'a`**; an **RPITIT** return; `Self::Batch<'_>`
in that method's signature; and a **non-`'static`** impl self type. Removing any
one gives a clean `E0477` or compiles. `async` mattered only because it desugars
to an RPITIT.

Reproduces on 1.85.1, 1.97.1 and 1.99.0-nightly, on editions 2018, 2021 and 2024
— **not a regression, and not fixed on nightly**. It duplicates the open
[rust-lang/rust#158983](https://github.com/rust-lang/rust/issues/158983), whose
summary says two crates and `async` are required; the minimisation above shows
`async` is not, and that is
[reported there](https://github.com/rust-lang/rust/issues/158983#issuecomment-5218463761)
rather than filed as a new issue.

The reproduction, the bisection and a script that regenerates the table live in
[`docs/experiments/rustc-ice-gat-foreign-trait/`](experiments/rustc-ice-gat-foreign-trait/README.md).

This matters beyond the bug report. **`where Self: 'a` on the port's GAT is one
of the five ingredients**, and phase 6 decides whether that GAT survives. If it
goes, so does the workspace's exposure to this ICE.
