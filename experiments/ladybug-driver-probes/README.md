# Ladybug driver probes — what `lbug` 0.20.3 actually does

**Run:** 2026-09-08, `x86_64-pc-windows-msvc`, rustc 1.97.1, `lbug` 0.20.3.
**Out of the workspace and out of the gate**, per this directory's standing rule:
it is a measurement, and `crates/happenstance-ladybug` still declares no
dependency on `lbug`.

Four questions ADR-0025 cannot be written without, each of which the crate's
existing prose answers wrongly or not at all. `probes.rs` is the program;
`results/probe-output.txt` is its output, verbatim.

## Getting the driver to build at all, which is most of the story

`crates/happenstance-ladybug/Cargo.toml`'s NOTE says `lbug` *"compiles LadybugDB's
C++ via cxx and cmake from source, which would put a multi-minute native build on
every `cargo check` of this workspace."* **That is not what happens**, and the
distinction changes phase 11's CI decision:

1. `build.rs` tries a **prebuilt download first** — `try_download_prebuilt_lbug`,
   which shells `scripts/download_lbug.ps1` on Windows — and only falls back to
   the cmake source build if that fails. It succeeded here. **CMake is not
   installed on this machine and was never needed.**
2. What it downloads is a single static archive, **`lbug.lib`, 1,444,941,838
   bytes — 1.44 GB**, plus `lbug.h` and `lbug.hpp`.
3. The Rust side is 17 crates and unremarkable. The link then **fails**:
   `LINK : fatal error LNK1181: cannot open input file 'libssl.lib'`.
4. OpenSSL is not optional. `link_libraries` calls `link_openssl()` and then
   emits `cargo:rustc-link-lib=dylib=libssl` / `=libcrypto` unconditionally in
   static mode, and in the dylib branch too. No feature or environment variable
   turns it off. `OPENSSL_DIR` is honoured and only adds a link-search path, so
   **any directory holding `libssl.lib` and `libcrypto.lib` will do**.
5. `winget install ShiningLight.OpenSSL.Dev` raises a UAC prompt and blocks — no
   good unattended. A user-space `vcpkg` building `openssl:x64-windows-static-md`
   took **5.6 minutes** and needs no administrator.

**And a trap worth more than the fix.** Setting `OPENSSL_DIR` after a failed
build changes nothing, because `lbug`'s `build.rs` emits **no
`cargo:rerun-if-env-changed` for `OPENSSL_DIR`** — only for `LBUG_SHARED`. The
second attempt reuses the cached script output and fails identically. `cargo clean
-p lbug` between attempts is what makes the variable take effect, and anyone
debugging this without knowing it will conclude the variable does not work.

Working invocation:

```console
OPENSSL_DIR=<vcpkg>/installed/x64-windows-static-md cargo run
```

## P0 — what a second handle is

```
first Database::new: ok
second Database::new on the SAME directory: REFUSED — IO exception:
  Could not set lock on file : …\lbug-probe-p0 (Error: 33)
two Connections over one Arc<Database>: ok / ok
connection B sees A's committed write: Some("[Int64(1)]")
```

**Two `Database`s over one directory is refused by a file lock.** So
`ProjectionFixture::SECOND_HANDLE` cannot be a second `Database`; it must be a
second `Connection` over one shared `Arc<Database>`, and that works — connection
B observes a write connection A committed, which is exactly the out-of-connection
observability PS-1's coupling is only visible through.

This also settles the store's constructor shape: whatever owns the `Database` must
be shareable, because two handles cannot each open one.

## P1 — `UINT64` holds the whole domain

```
stored 18446744073709551614 into a UINT64 column: ok
read back: Some([UInt64(18446744073709551614)])
```

`u64::MAX - 1` round-trips exactly. `SequencePosition` is a `NonZeroU64`, so
**there is no narrowing** — unlike Postgres and SQLite, where a signed `bigint`
loses the top half of the domain and the adapters carry a fallible converter for
it.

Two consequences for `crates/happenstance-ladybug/src/projection_store.rs`:
`LadybugProjectionStoreError::PositionOutOfRange` describes a gap that does not
exist and no code path can construct it — a decorative variant by this
repository's own corollary. `MalformedCheckpoint` survives and narrows to the one
honest case: a stored `0` is a corrupt checkpoint rather than a missing one, and
collapsing it into "never run" silently replays from event 1.

## P2 — read-your-own-writes inside a transaction

```
BEGIN TRANSACTION accepted as a query
READ-YOUR-OWN-WRITES inside the transaction: Some([Int64(1)])
after COMMIT: Some([Int64(1)])
ROLLBACK accepted
after ROLLBACK (expect 1): Some([Int64(1)])
```

`BEGIN TRANSACTION`, `COMMIT` and `ROLLBACK` are plain Cypher statements on a
connection, and a statement **sees what earlier statements in the same
transaction wrote**.

**This is phase 11's contribution, and it is the answer to PS-4's second
condition in its Cypher-level reading.** `crates/happenstance-ladybug/src/projection_store.rs:52-67`
records that a deferred write set gives a different answer from a live
transaction *if* the projection's logic depends on a traversal of what it has
already written. It does not, here: replay is one connection, one transaction, in
order, so statement *n* matches what statement *n−1* wrote.

State the result narrowly. This says PS-4's condition **did not fire for this
adapter**, on this engine. It is not a discharge of the clause, which generalises
over write-behind shapes this probe says nothing about.

## P3 — an injectable commit fault, and an unexpected finding beside it

```
duplicate-key CREATE raised: Query execution failed: Runtime exception:
  Found duplicated primary key value planted, which violates the uniqueness
  constraint of the primary key column.
read-model rows visible after the fault: Some([Int64(0)])
ROLLBACK after the fault: refused
read-model rows finally (expect 0): Some([Int64(0)])
```

A `CREATE` against a pre-planted primary key **does** raise, so `COMMIT_FAULT`
has a real injection: plant a row, and make the last statement of the transaction
collide with it. `MERGE` would not work — it matches an existing node and cannot
conflict — so the injection has to be a `CREATE`.

**And the finding nobody was looking for: LadybugDB aborts the whole transaction
itself on a statement error.** The read-model write made earlier in the same
transaction is already gone (`count = 0`) *before* any rollback, and the
subsequent `ROLLBACK` is **refused** because there is no longer a transaction to
roll back.

That shapes two bodies rather than one. `commit`'s error path must not assume it
can roll back after a failed statement — the engine has already done it, and
issuing `ROLLBACK` produces a second error that would mask the first. And it is
*good* news for PS-1: atomicity is enforced by the engine rather than by this
adapter remembering to ask for it.

## Reproducing

`probes.rs` is the whole program and `Cargo.toml.reference` is the manifest it
was built with, in a scratch crate outside this workspace. It is not a workspace
member on purpose: adding `lbug` here would put the 1.44 GB archive and the
OpenSSL requirement on every `cargo check` of the repository, which is the cost
phase 11 has to decide about rather than inherit.
