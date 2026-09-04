# Do adapter crates re-export their driver, and is `happenstance-core` re-exported by the crates that expose it in their public API — one answer for `rusqlite`, `worker`, `tokio` and `futures_core`, taken once for the workspace?

Decision record: **D-1-D-4-driver-reexport**. Brief only — no atom, no prose, no status line.

---

## Why this is owed

Three independent things in the tree already argue for a decision, and none of them
has been taken.

**1. A constitution atom says to do it, and cites the gap as an unfixed defect.**
RS-40-4, *Name a signature's types through the defining crate's own re-export*
(`standards/rust/40-public-surface-and-evolution.md:171`), opens with the argument
in the workspace's own voice:

> `happenstance-core` does `pub use bytes;` on purpose: it costs the crate a major
> bump whenever `bytes` takes one, and buys the guarantee that a caller and an
> adapter cannot be holding two `Bytes` types that look identical. Where there is
> no such re-export the guarantee is the caller's problem, and the first symptom is
> `error[E0433]`.
> — `standards/rust/40-public-surface-and-evolution.md:173-177`

Its **Not** block is a live negative control, not prose:

```rust,compile_fail,E0433
fn spawnable<S: happenstance_core::futures_core::Stream>(_s: S) {}
```
— `standards/rust/40-public-surface-and-evolution.md:195-198`

and its Evidence line closes with `[RUNBOOK](../../RUNBOOK.md)` *(`pub use futures_core;`
is proposed and not landed)* (`standards/rust/40-public-surface-and-evolution.md:210`).
The atom therefore both argues the rule **and** ships a compiled assertion that the
workspace has not obeyed it.

**2. The RUNBOOK carries the same item, unchecked and unscheduled.**

```
- [ ] `happenstance_core::prelude` exporting `EventStore` (not `SendEventStore`),
      so the default import path cannot produce E0034; `pub use futures_core;`
      beside `pub use bytes;`, since `Stream` appears in `read`'s signature and
      every adapter is forced to name it.
```
— `RUNBOOK.md:2869-2872`

**3. The audit files it as two entries and routes it here without deciding it.**
`references/evaluation/review-pre-publication-2026-09-03.md:2279-2367` (D-1 · D-4),
whose **Routing** paragraph says the decision "belongs to whoever owns that RUNBOOK
line, decided once for the workspace rather than three times, and it wants an ADR
because RS-40-4's 'Evidence' line will need repointing at whatever lands"
(`:2366`). The same review's **unopened item #2**
(`references/evaluation/review-pre-publication-2026-09-03.md:228-247`) extends the
identical argument to the workspace's own crate and adds the lockstep question:

> And the lockstep is undecided: nothing states whether the five versions move
> together, which is the question `happenstance-testkit`'s separate version key
> half-answers without saying so.
> — `:242-244`

**No specification clause governs the *re-export* half; one governs the *versioning*
half, and this brief originally missed it.** The sentence that stood here — "No
specification clause governs this" — has been **removed as falsified**, not
rewritten around. It rested on `grep -n "re-export\|reexport\|pub use"
spec/SPECIFICATION.md`, which returns five hits — `:349`, `:4900`, `:8181`, `:8277`,
`:8402` — none about a third-party dependency's visibility. That grep is a sound
coverage check for re-exports and **cannot reach a clause about version keys**, which
is what **CF-32 [FROZEN]** (`spec/SPECIFICATION.md:8684`) is. Option D, as originally
filed, then bolted a versioning promise onto a question whose coverage check had only
ever covered re-exports. CF-32 is quoted and its collision worked through under
Option D below; the short form is that CF-32 mandates `happenstance-testkit`'s own
`version` key and reasons that a shared number between the contract and the testkit
is "wrong in both directions" (`spec/SPECIFICATION.md:8703-8712`).

For the re-export half the grep result stands. The nearest clause is **ES-6 [FROZEN]**
(`spec/SPECIFICATION.md:2677`, marker at `:2679`), and it points the other way: it
*endorses* the wrapping, citing `SqliteEventStoreError` as "twelve real variants
over `rusqlite::Error`, `JoinError`, `TryCurrentError` and the crate's own decode
failures" (`spec/SPECIFICATION.md:2688-2690`) as the instrument that made ES-6
decidable at all.

**No decision atom governs the re-export half, and that half would supersede nothing.**
(The original heading said "this decision would supersede nothing" without
qualification; that has been **narrowed to the re-export half**, because the
versioning half runs into CF-32 [FROZEN] and a `[FROZEN]` clause is changed by a new
ADR, not by a brief.)
`grep -rln "pub use bytes" references/adr/ .kb/` returns **nothing**: the
`pub use bytes;` precedent everything cites exists as code plus a doc comment
(`crates/happenstance-core/src/lib.rs:184-186`) and has never been recorded as a
decision. ADR-0003's Decision section (`.kb/decisions/0003-opaque-payloads.md:45-58`)
settles that `Event::data` is `bytes::Bytes` and that the crate carries no `serde`;
it says nothing about re-exporting the crate. So the re-export policy is currently
held **only** by RS-40-4, an atom that binds below a clause and above
`references/evaluation/*`, and whose own doctest currently asserts the workspace
does not follow it.

---

## What is true today

### The export surfaces, in full

| crate | driver in public signatures | re-exports it? | publishable? | registry state (verified 2026-09-03) |
|---|---|---|---|---|
| `happenstance-core` | `bytes`, `futures_core` | `bytes` **yes**, `futures_core` **no** | yes | `0.0.0`, `0.2.0-alpha.1` — both live, unyanked |
| `happenstance` | (none of its own) | glob of core's *items*, not the crate | yes | `0.0.0`, `0.2.0-alpha.1` — both live, unyanked |
| `happenstance-testkit` | — | — | yes | `0.0.0`, `0.2.0-alpha.1` — both live, unyanked |
| `happenstance-sqlite` | `rusqlite`, `tokio` | **no** — zero `pub use` at all | yes | **`0.0.0` live, unyanked** (2026-08-18), zero dependencies |
| `happenstance-cloudflare` | `worker` | **no** | yes | **`0.0.0` live, unyanked** (2026-08-20) |
| `happenstance-neon` | `serde_json` | **no** | `publish = false` | not on the registry |
| `happenstance-postgres` | `sqlx` | **yes** | `publish = false` | not on the registry |
| `happenstance-ladybug` | — | own types only | `publish = false` | not on the registry |
| `happenstance-sync` | — | own types only | `publish = false` | not on the registry |

The last column replaces two cells that read **"yes, unpublished"** for
`happenstance-sqlite` and `happenstance-cloudflare`. Those cells were **false and
have been removed**: both crates are on crates.io at `0.0.0` and neither is yanked.
See *Registry state, verified* below for what a live `0.0.0` does and does not cost
this decision — the answer is argued, not assumed.

`happenstance-sqlite`'s entire export surface is three module declarations and no
`pub use` whatsoever:

```rust
#[cfg(any(feature = "event-store", feature = "projection-store"))]
pub mod connection;

#[cfg(feature = "event-store")]
pub mod event_store;
...
#[cfg(feature = "projection-store")]
pub mod projection_store;
```
— `crates/happenstance-sqlite/src/lib.rs:82-95`

`happenstance-cloudflare` re-exports three lines of its own types and no `worker`:

```rust
pub use event_store::{CloudflareEventStore, CloudflareEventStoreError, SqlRowStream};
pub use js::{JsHandle, JsThrow, StringifiedThrow};
pub use sql_storage::{SqlCursor, SqlError, SqlRow, SqlStorage, SqlValue};
```
— `crates/happenstance-cloudflare/src/lib.rs:492-494`

`happenstance-neon` is the same shape — twelve of its own names, at
`crates/happenstance-neon/src/lib.rs:118-129`, and no `serde_json`.

### The two crates the audit says get it right

Both, and only both, write the doc sentence that names the reason:

```rust
/// Re-exported so adapters and callers can name payload types without a
/// direct dependency on a specific `bytes` version.
pub use bytes;
```
— `crates/happenstance-core/src/lib.rs:184-186`

```rust
/// Re-exported so callers can build a pool without pinning their own `sqlx`
/// version against this crate's.
pub use sqlx;
```
— `crates/happenstance-postgres/src/lib.rs:76-78`

**Both exemplars are weaker than they look, and the brief should say so.**
`happenstance-postgres` carries `publish = false`
(`crates/happenstance-postgres/Cargo.toml:12`) and every body that would touch a
server is `todo!()` (`crates/happenstance-postgres/src/lib.rs:5-6`), so its
`pub use sqlx;` has never been observed by a consumer and cannot be. And
`happenstance-core`'s exemplar is **half-executed**: `futures_core::Stream` is in
`read`'s return type at `crates/happenstance-core/src/store.rs:171`
(`) -> impl Stream<Item = Result<SequencedEvent, Self::Error>>;`, with the import at
`:78`), and `futures_core` is not re-exported. The contract crate obeys its own rule
for one of the two crates it puts in a public signature.

### The foreign types a caller cannot name without adding a dependency

**`happenstance-sqlite` — `rusqlite`, on three separate paths.**

Error, both enums:

```rust
    Sqlite(#[from] rusqlite::Error),
    Worker(#[from] JoinError),
    NoRuntime(#[from] TryCurrentError),
```
— `crates/happenstance-sqlite/src/event_store.rs:928, 942, 950`; repeated verbatim in
`SqliteProjectionStoreError` at `crates/happenstance-sqlite/src/projection_store.rs:452, 464, 468`.
Imports at `crates/happenstance-sqlite/src/event_store.rs:128-131`.

Construction (bring-your-own-connection):

```rust
    pub fn new(connection: Connection) -> Result<Self, SqliteEventStoreError> {
```
— `crates/happenstance-sqlite/src/event_store.rs:359`; and
`pub fn new(connection: Connection) -> Self` at
`crates/happenstance-sqlite/src/projection_store.rs:216`.

Configuration, returning the driver's own `Result` alias:

```rust
pub fn open_configured(path: impl AsRef<Path>) -> rusqlite::Result<Connection> {
pub fn configure(connection: &Connection) -> rusqlite::Result<()> {
    pub fn read_back(connection: &Connection) -> rusqlite::Result<Self> {
```
— `crates/happenstance-sqlite/src/connection.rs:78, 89, 186`

Projection write vocabulary:

```rust
    pub fn push(&mut self, sql: impl Into<String>, params: impl IntoIterator<Item = Value>) {
    pub fn params(&self) -> &[Value] {
```
— `crates/happenstance-sqlite/src/projection_store.rs:423, 382`, where `Value` is
`rusqlite::types::Value` (`:111`), documented as such at `:367-369`.

**The audit's own correction stands, and I re-verified it.** The happy path is
rusqlite-free: `pub fn open(path: impl AsRef<Path>)` at
`crates/happenstance-sqlite/src/event_store.rs:400` and `pub fn open_in_memory()` at
`:377` name no foreign type. A consumer can build, append and read without ever
writing `rusqlite`; the coupling bites when they first try to *inspect* an error or
supply their own connection.

**`happenstance-sqlite` — `tokio`, only in the error enums.** `JoinError` and
`TryCurrentError` are public (`event_store.rs:942, 950`). `JoinHandle` appears only
in the **private** `enum ReadState` at `event_store.rs:1207-1212`, so it is not on
the public surface. `tokio` is a non-optional dependency at
`crates/happenstance-sqlite/Cargo.toml:37`, taken at `features = ["rt"]`.

**`happenstance-cloudflare` — `worker`, on every construction path and the error
path.**

```rust
    pub fn new(sql: worker::SqlStorage) -> Self {
    pub fn from_state(state: &worker::State) -> Self {
```
— `crates/happenstance-cloudflare/src/sql_storage.rs:247, 262`

```rust
    pub fn from_error(error: worker::Error) -> Self {
    pub fn error(&self) -> &worker::Error {
```
— `crates/happenstance-cloudflare/src/js.rs:188, 197`, over the private field
`error: Rc<worker::Error>` at `:172`.

`worker` is unconditional (`crates/happenstance-cloudflare/Cargo.toml:81`), and the
crate declares **no `[features]` table at all** — there is no build of it that does
not link `worker` 0.8.5 (`Cargo.toml:122`).

**`happenstance-neon` — `serde_json`, twice.**

```rust
    pub fn with_params(query: impl Into<String>, params: Vec<serde_json::Value>) -> Self {
    pub fn body(&self) -> Result<Vec<u8>, serde_json::Error> {
```
— `crates/happenstance-neon/src/transport.rs:114, 183`

Neon owns **no driver**: `SqlTransport` is its own one-method trait and
`NullTransport` the in-tree implementation, because "a real client needs a TLS stack
on the host and `wasm-bindgen`'s `fetch` on `wasm32`" (`crates/happenstance-neon/src/lib.rs:100-106`).
So for Neon the question "does the adapter re-export its driver" has no referent —
what it exposes is `serde_json`, and the crate is `publish = false`
(`crates/happenstance-neon/Cargo.toml:12`).

### `pub use happenstance_core;` — the audit's claim verified

`grep -rn "pub use happenstance_core"` over the workspace returns exactly one
non-test hit, and it is a **glob of items, not the crate**:

```rust
pub use happenstance_core::*;
```
— `crates/happenstance/src/lib.rs:242`

`pub use happenstance_core;` appears in **no** crate. Four tests pin the glob's exact
text (`crates/happenstance/tests/doc_surface.rs:133, 326`,
`crates/happenstance/tests/doc_budget.rs:260`,
`crates/happenstance/tests/docs_composition.rs:197`), so changing that line is not a
silent edit.

One consequence of the glob is already load-bearing and worth recording, because it
predicts what a `futures_core` re-export would do: the glob **carries core's own
crate re-exports through**. `happenstance::bytes::Bytes` resolves today and is used
across the test suite — `crates/happenstance/tests/codec_round_trip.rs:7`,
`command_loop.rs:13`, `flavours.rs:17`, `mounted_at_the_crate_root.rs:10`, and it is
the path the public doc example uses at `crates/happenstance/src/testing/mod.rs:352`.
So `pub use futures_core;` in core would give `happenstance::futures_core` for free,
with no edit to `happenstance`.

What the glob does **not** give is a path to the crate: `happenstance::happenstance_core`
does not exist, and no adapter re-exports core at all.

**What a consumer must therefore add.** To write
`fn ingest<S: EventStore>(store: S)` over both `happenstance::Event` and
`happenstance_sqlite::SqliteEventStore`, they add `happenstance-core` to their own
manifest and choose a version. Every crate here pins it at one value:

```toml
happenstance-core = { version = "0.2.0-alpha.1", path = "crates/happenstance-core", default-features = false }
happenstance      = { version = "0.2.0-alpha.1", path = "crates/happenstance", default-features = false }
happenstance-testkit = { version = "0.2.0-alpha.1", path = "crates/happenstance-testkit" }
```
— `Cargo.toml:39-41`, over `version = "0.2.0-alpha.1"` at `Cargo.toml:15`

`happenstance-testkit` is the one with its own version key rather than
`version.workspace = true` (`crates/happenstance-testkit/Cargo.toml:21`), at the same
value.

**The brief's gloss on that key — that it "half-answers the lockstep question without
saying so" — has been removed as false.** It cited only `Cargo.toml:21`, the *value*,
and never read `:4-20`, the *reason*, which is seventeen lines of comment saying
exactly why the number is independent and why it moved down to a pre-release. Nor is
the reason confined to a comment: **CF-32 [FROZEN]** states it as a clause and gives
it a machine check. So the key does not half-answer the lockstep question without
saying so — it answers it, out loud, in the negative, and the audit's `:242-244` is
wrong on the same point. What is genuinely undecided is narrower: whether the *four
other* publishable crates move together, and how a consumer is told which
`happenstance-core` a given adapter's number implies.

### The publishable set, and where the version window actually is

`xtask/src/package.rs:86` declares `PUBLISHABLE`, and `reconcile` fails in **both**
directions if it and the manifests disagree (`xtask/src/package.rs:185-237`).
`happenstance-sqlite` and `happenstance-cloudflare` are in it
(`:101, :107`) and carry no `publish = false` — `crates/happenstance-sqlite/Cargo.toml:23`
says the absence is deliberate. `happenstance-neon`, `happenstance-postgres`,
`happenstance-ladybug` and `happenstance-sync` all carry `publish = false`
(`crates/happenstance-{neon,postgres,ladybug,sync}/Cargo.toml:12`).

### Registry state, verified

The audit states that `happenstance`, `happenstance-core` and `happenstance-testkit`
are on crates.io at `0.2.0-alpha.1` and the two adapters are not
(`references/evaluation/review-pre-publication-2026-09-03.md:2362`). **The first half
is right; the second half is wrong, and so was this brief's repetition of it.** The
registry was queried on 2026-09-03 (`/api/v1/crates/<name>`):

- `happenstance-core`, `happenstance`, `happenstance-testkit`: `0.0.0` and
  `0.2.0-alpha.1`, all six live and unyanked.
- `happenstance-sqlite`: **`0.0.0`, published 2026-08-18, not yanked.**
  `/api/v1/crates/happenstance-sqlite/0.0.0/dependencies` returns an **empty list**.
- `happenstance-cloudflare`: **`0.0.0`, published 2026-08-20, not yanked.**

**Why a live `0.0.0` does not rescue or kill anything here, argued rather than
asserted.** The temptation is to say the crate "isn't really published"; that is the
move ADR-0029 made when it leaned on "nothing is published", and it is not available
twice. The argument has to be about what the published artifact *promised*:

1. **It exposed no driver type, because it had no driver.** `0.0.0` of
   `happenstance-sqlite` carries **zero dependencies** — it does not link `rusqlite`,
   so no consumer of it can be holding a `rusqlite::Error` from it. There is no
   coupling to retract, and nothing that a later seal (Option C) would break.
2. **`0.0.x` is its own compatibility range.** Under Cargo's semver rules every
   `0.0.z` is incompatible with every other version; `0.0.0` confers no compatibility
   obligation whatsoever on `0.2.0`. A consumer cannot have written
   `happenstance-sqlite = "0.2"` against it.
3. **What it did buy is the name, and that is all it was for.** Both `0.0.0`
   publications land in the same August window as `happenstance-core 0.0.0` and
   `happenstance 0.0.0` — a name reservation across the family, not a release of an
   adapter.

So Option C remains *additive-shaped* today, on the narrow ground that no published
artifact of these two crates has ever put a driver type in front of a consumer. That
is a weaker and more specific claim than "the crates are unpublished", which is what
the brief used to say and which was simply untrue.

### Two facts the audit did not check

**`happenstance-sqlite`'s README is not compiled.**
`grep -rn 'doc = include_str!("../README.md")' crates/*/src/lib.rs` returns
`happenstance-cloudflare/src/lib.rs:19`, `happenstance-core/src/lib.rs:7`,
`happenstance-testkit/src/lib.rs:7`, `happenstance/src/lib.rs:10` — **not** sqlite.
So the audit's remedy of adding a `rusqlite` line to the install block at
`crates/happenstance-sqlite/README.md:33-36` lands as unverified prose, while
`happenstance-cloudflare`'s README example — which names `worker::State` at
`crates/happenstance-cloudflare/README.md:44` — is a compiled doctest, and it is
compiled today only because `worker` is already in that crate's own dependency
graph, not because a reader was told to add it.

**Landing `pub use futures_core;` breaks the constitution's own doctest.**
`xtask/src/constitution.rs:106-109` includes
`standards/rust/40-public-surface-and-evolution.md` as doctests, and CLAUDE.md's
Commands section records that `cargo test -p xtask --doc` compiles every example in
the corpus. The `compile_fail,E0433` block at
`standards/rust/40-public-surface-and-evolution.md:195-198` asserts that
`happenstance_core::futures_core::Stream` does **not** resolve. The moment it does,
that doctest fails — the atom must be amended in the same change. This is a
one-line edit, not an obstacle, but it means the `futures_core` half of this
decision cannot land without touching RS-40-4, exactly as its Evidence line at
`:210` anticipates.

---

## Options

### Option A — Re-export every foreign crate that appears in a public signature

`pub use rusqlite;` and `pub use tokio;` in `happenstance-sqlite`; `pub use worker;`
in `happenstance-cloudflare`; `pub use futures_core;` in `happenstance-core`;
`pub use serde_json;` in `happenstance-neon`. Each with the doc sentence
`happenstance-core` and `happenstance-postgres` already wrote. This is the audit's
Remediation (`:2364`) taken literally.

- **Costs a caller:** nothing, and gains them a nameable path to the exact type —
  `use happenstance_sqlite::rusqlite::Error;` instead of guessing a version.
- **Costs an adapter author:** a public, permanent obligation to bump their own major
  whenever the driver takes one — RS-40-4's own stated price
  (`standards/rust/40-public-surface-and-evolution.md:173-174`). For `rusqlite` at
  `0.40` (`Cargo.toml:72`) and `worker` at `0.8.5` (`Cargo.toml:122`), both pre-1.0,
  that is a real and frequent cadence. Also one `cargo hack` powerset dimension of
  nothing — a `pub use` adds no feature.
- **Semver class:** **additive** everywhere. Nothing is removed, nothing is renamed.
- **Forecloses:** any later attempt to *seal* the surface (Option C) becomes a
  breaking change against a documented item rather than against an incidental one.
  Also foreclosed is treating the driver as an implementation detail in
  documentation: once `happenstance_sqlite::rusqlite` is public API, the crate has
  said out loud that its consumers are expected to hold rusqlite types.

**Where the evidence does not support this option as filed.** Applied literally it
re-exports `serde_json` from `happenstance-neon` — a `publish = false` skeleton
(`crates/happenstance-neon/Cargo.toml:12`) whose bodies are `todo!()`, where the
re-export can never be observed and adds a semver promise to a crate that makes none.
The audit's own exemplar has the same defect: `happenstance-postgres` "gets this
right" while being unable to demonstrate the benefit, and the audit itself notes the
`publish = false` at `:2354` without drawing the conclusion. **The discriminator that
actually predicts harm is not "is it a driver" — it is "will a consumer of a
*published* crate hold this type, and does the dependency take majors often?"** On
that test `worker` scores highest (guaranteed second copy, pre-1.0, `:2358`),
`rusqlite` next (likely second copy, pre-1.0), `futures_core` next (`0.3` is
effectively frozen but every adapter names `Stream`), and `tokio` lowest — `1.x`,
whose major has not moved since 2020, and whose two exposed types are error-enum
payloads a consumer rarely destructures.

### Option B — Status quo: re-export nothing new

- **Costs a caller:** they add `rusqlite = "0.40"` / `worker = "0.8.5"` /
  `happenstance-core = "0.2.0-alpha.1"` themselves, from a version number that is
  written nowhere they will read — `crates/happenstance-sqlite/README.md:33-36`
  offers `happenstance-sqlite = "0.2.0-alpha.1"` and no companion line. The failure
  is late and the diagnostic is bad: two `rusqlite::Error` types that print
  identically, `E0308`, discovered the first time they try to act on a store error.
- **Costs an adapter author:** nothing, today. The coupling stays undeclared but does
  not stop existing — `SqliteEventStore::new(rusqlite::Connection)` already makes
  every `rusqlite` major a breaking release of `happenstance-sqlite` whether or not
  anyone wrote it down (audit, `:2362`).
- **Semver class:** **none** — no change.
- **Forecloses:** nothing structurally. It preserves the option to seal the surface
  later, and preserves the option to re-export later. What it spends is the interval:
  every consumer who installs 0.2.0 learns the manifest-line idiom, and that
  population does not un-learn it.

### Option C — Seal the surface where the foreign type is removable; re-export only where it is not

Wrap `rusqlite::Error` in an opaque `SqliteDriverError` with accessors
(`sqlite_error_code()`, `Display`); replace `PendingStatement`'s
`rusqlite::types::Value` with the crate's own value enum, the way
`happenstance-cloudflare` already models `SqlValue`
(`crates/happenstance-cloudflare/src/sql_storage.rs:51-82`, whose `to_binding` is
private); keep `open`/`open_in_memory` as the documented construction path and demote
`new(Connection)` and `connection::open_configured`. Then re-export only where the
type is genuinely unavoidable — `worker` in `happenstance-cloudflare`, because the
consumer writes the `#[durable_object]` class the README's own example takes a
`&worker::State` from (`crates/happenstance-cloudflare/README.md:44`), and
`futures_core` in `happenstance-core`, because `Stream` is in `read`'s return type
(`crates/happenstance-core/src/store.rs:171`) and no adapter can avoid naming it.

- **Costs a caller:** loses `rusqlite::Error::sqlite_error_code()` unless the wrapper
  re-offers it; loses the bring-your-own-`Connection` path, which is the one thing an
  application already using rusqlite for its own tables most wants.
- **Costs an adapter author:** the most work by a wide margin — a wrapper type, an
  accessor surface, and a value enum with a conversion, all needing tests. And it
  argues *against* ES-6 [FROZEN], which cites `SqliteEventStoreError`'s twelve
  variants over the real driver types as the instrument that made the clause
  decidable (`spec/SPECIFICATION.md:2688-2690`).
- **Semver class:** **breaking**, and only additive-shaped *before* publication —
  after `happenstance-sqlite` ships, removing `rusqlite::Error` from a public variant
  is a major bump. Note the `#[non_exhaustive]` at
  `crates/happenstance-sqlite/src/event_store.rs:924` protects *addition* of variants,
  not *change* of a variant's payload.
- **Forecloses:** the connection-injection story, and any future in which the adapter
  wants to hand the driver's own richer error back to a caller.

### Option D — One policy, stated as a test ~~plus the lockstep sentence~~

*As filed, D was: "a **published** crate re-exports any crate whose type appears in
one of its public signatures, **and states in its own documentation whether
`happenstance-core`'s version is part of its semver contract**." The emphasised
second conjunct is withdrawn — see the strike below. What remains is:*

Adopt a written rule of the form *"a **published** crate re-exports any crate whose
type appears in one of its public signatures"*, and execute it against today's tree:

- `happenstance-core`: add `pub use futures_core;` beside `pub use bytes;`
  (`crates/happenstance-core/src/lib.rs:186`), and amend RS-40-4's `compile_fail`
  block in the same change (`standards/rust/40-public-surface-and-evolution.md:195-198`),
  which retires the RUNBOOK item at `RUNBOOK.md:2869-2872`.
- `happenstance-sqlite`: `pub use rusqlite;` and `pub use tokio;`, plus the README
  install line.
- `happenstance-cloudflare`: `pub use worker;`.
- `happenstance`, `happenstance-sqlite`, `happenstance-cloudflare`,
  `happenstance-testkit`: `pub use happenstance_core;` — the crate, alongside (not
  instead of) `happenstance`'s existing glob at `crates/happenstance/src/lib.rs:242`,
  whose exact text four tests pin.
- `happenstance-neon`, `happenstance-postgres`, `happenstance-ladybug`,
  `happenstance-sync`: **nothing owed while `publish = false`**; the rule attaches at
  the moment that key is removed, which is the same gate `xtask/src/package.rs:185`
  already polices in both directions.
- ~~One sentence per publishable crate stating that `happenstance-core`'s version is
  part of that crate's semver contract and that all five move together — which
  `Cargo.toml:39-41` already makes true in fact and nothing states.~~
  **WITHDRAWN — this clause is falsified.** See immediately below.

**The lockstep clause collides with CF-32 [FROZEN] and has been struck from the
option.** CF-32 (`spec/SPECIFICATION.md:8684-8712`) requires
`happenstance-testkit` to carry its own `version` key and states the reason at
length:

> Under a shared version key the two crates cannot move independently in either
> direction, and both directions are wrong. Adding a rule bumps the testkit's minor,
> which drags `happenstance-core` to the same number and republishes an unchanged
> contract — after which a semver-checking tool has no earlier version of that
> contract to diff against. A patch release of the contract republishes the testkit
> and forces every adapter to re-run a bar that did not change.
> — `spec/SPECIFICATION.md:8703-8711`

`happenstance-testkit` is one of the five members of `PUBLISHABLE`
(`xtask/src/package.rs:86`), so "all five move together" is a *published promise* that
contradicts a `[FROZEN]` clause. It is worse than a plain contradiction: CF-32's guard
is a **manifest check** that reads the `[package]` table, so a prose lockstep promise
in five READMEs reinstates precisely the coupling CF-32 forbids, in the one medium
that check cannot see. The manifest would stay compliant while the documentation
promised the opposite.

The option is therefore reduced to its re-export half. What survives is **Option A
with a publication test attached**, which is one scoping word on A's rule rather than
a separate option — recorded honestly in the Recommendation below. Lockstep, if it is
wanted at all, is a *different* decision: it must amend or supersede a `[FROZEN]`
clause and so requires its own ADR, and it cannot ride along on a re-export brief
whose coverage grep never covered it.

- **Costs a caller:** nothing; gains a nameable path to the exact type.
- **Costs an adapter author:** the Option A obligation, but only for crates that
  actually ship.
- **Semver class:** **additive** for every `pub use`.
- **Forecloses:** Option C for the crates it touches. It does **not** foreclose
  sealing `happenstance-neon`'s or `happenstance-postgres`'s surface, since the rule
  does not attach to them yet, and it no longer forecloses independent versioning of
  the five publishable crates — CF-32 already settled that, in the opposite direction
  from the withdrawn clause.

---

## Recommendation

**FLIPPED. The recommendation is now Option A, scoped to published crates** — that
is, Option D's re-export half, with its lockstep sentence withdrawn. Land the four
re-exports (`futures_core` in core, `rusqlite` and `tokio` in sqlite, `worker` in
cloudflare) plus `pub use happenstance_core;` where the contract is in a public
signature; route lockstep to its own ADR.

**What flipped it.** The brief recommended Option D, and D's *only* stated edge over
A was the publication test — which is one scoping word on A's rule, not a separate
option. Everything else D added was the lockstep clause, and that clause collides
with **CF-32 [FROZEN]** (`spec/SPECIFICATION.md:8684`). With the clause struck, D is
A-scoped-to-published under a different name, and the honest label is the one that
names the surviving option rather than the compound that no longer exists. The
brief's coverage grep (`re-export|reexport|pub use` over `spec/`) could never reach
CF-32, a clause about version keys — so the versioning half of D was never checked
against the specification at all. That failure is recorded in *Why this is owed*
above, where the sentence "No specification clause governs this" has been removed.

The substantive judgement below is unchanged: re-exporting beats the status quo and
beats sealing, for the reasons that follow, and the qualification on `tokio` still
stands.

It beats **Option B** because the coupling Option B "avoids" already exists. Every
`rusqlite` major is already a breaking release of `happenstance-sqlite`, because
`rusqlite::Connection` is in `new`'s parameter list (`event_store.rs:359`) and
`rusqlite::Error` is in a public variant (`:860`). A re-export does not create the
obligation; it makes an existing one *nameable and discoverable*. The only thing
Option B buys is the ability to not say so out loud, and RS-40-4 already says the
first symptom of that silence is `error[E0433]` at the consumer
(`standards/rust/40-public-surface-and-evolution.md:176-177`).

It needs the **publication scope** because A applied *literally* re-exports
`serde_json` from a `publish = false` skeleton and thereby adds a semver promise to a
crate that makes none — and because A's own exemplar, `happenstance-postgres`, is
unfalsifiable for exactly that reason (`crates/happenstance-postgres/Cargo.toml:12`).
The scoped rule is A's rule with a publication test attached, and the test is one the
gate already enforces in both directions (`xtask/src/package.rs:185-237`). (This
paragraph used to read "It beats Option A"; that framing is withdrawn, because a
scoping word does not make a rival option.)

It beats **Option C** on a cost-of-delay asymmetry that the audit did not surface and
that I consider the decisive fact in this brief: **the re-export is additive at any
time; Option C is additive now and breaking later.** If the decision goes to C it
must go there *before* `happenstance-sqlite` publishes **an adapter** — which, per
*Registry state, verified*, it has not: its only release is a zero-dependency `0.0.0`
name reservation in a `0.0.x` range that confers nothing on `0.2.0`. (The original
sentence said "before `happenstance-sqlite` publishes", full stop, on the false
premise that the crate was unpublished; that premise has been removed and the claim
narrowed to what the registry actually supports.) If it goes to the re-export, it can
go there at 0.2.0, 0.3.0 or 1.0 at identical mechanical cost. So the option that is
genuinely urgent is the one I am **not** recommending — which is the reason C must be
ruled in or out in this pass rather than deferred.

Note that this comparison, and the two above it, are independent of publication
timing: A/D's obligation already exists (the coupling is in the signatures), C costs
the most work and argues against ES-6 [FROZEN], and neither of those facts moves when
a `0.0.0` turns out to be live.

**It does not answer the lockstep question, and the claim that it did has been
removed.** The brief argued that D "answers the lockstep question in the same breath"
because, as the audit's unopened item #2 puts it, "a consumer on
`happenstance = "0.3"` and `happenstance-sqlite = "0.2"` gets two `Event` types that
print identically" (`references/evaluation/review-pre-publication-2026-09-03.md:240-242`),
and that a `pub use happenstance_core;` without a lockstep statement gives them a
*path* to the right type and no reason to believe the path is stable. The first half
of that is still true and is a real hole; the second half — that this decision should
close it — is withdrawn, because closing it means contradicting CF-32 [FROZEN]. The
two halves are **not** one hole: one is a re-export, the other is an amendment to a
frozen packaging clause. They are separated here on purpose, and the second is filed
in *What this does not settle*.

**The qualification.** The `tokio` re-export is the weakest member of the set and
should be decided on its own evidence rather than carried by the policy. `tokio` is
`1.x` (`Cargo.toml:129`), its major has not moved in years, and the only types it puts
on `happenstance-sqlite`'s public surface are two error-enum payloads
(`event_store.rs:942, 950`) — `JoinHandle` is private (`:1212`). Worse, the
re-export would be **feature-thin**: `happenstance-sqlite` takes tokio at
`features = ["rt"]` (`crates/happenstance-sqlite/Cargo.toml:37`), so a consumer who
reaches tokio *only* through the re-export gets a partial tokio and a confusing
`E0433` of a different kind. That is a general property worth writing into whatever
lands: **a driver re-export is a type-identity and discoverability guarantee, not a
substitute for the consumer's own dependency.** It does not prevent the duplicate —
a consumer holding `rusqlite = "0.37"` for their own tables still gets two copies —
it gives them a path to the correct one and tells them which version this crate
resolved.

### The strongest arguments against, in their own words

**1. The registry objection (real; does not change the recommendation).** Recorded
verbatim as filed:

> The brief rests on "the two adapters are unpublished" and names that fact as the
> hinge keeping Option C alive; the registry falsifies it. The recommendation
> survives on inspection — sqlite's published 0.0.0 carries zero dependencies, so it
> is a name squat that never exposed rusqlite::Error, and 0.0.x is its own cargo
> compatibility range conferring nothing on 0.2.0. C is still additive-shaped, and D
> still beats B, A and C on grounds independent of publication timing. But three
> passages must be rewritten before anyone decides from this brief: the table's
> "unpublished" column, :563's "entirely pre-consumer", and item 6, which now reads
> as an untriggered escape hatch that has in fact triggered. Crucially the brief must
> argue why a live 0.0.0 does not count, rather than assert the crate does not exist
> — that is exactly the move ADR-0029 failed to make when it relied on "nothing is
> published".
>
> FALSIFIED PREMISE: The table (:91-92) marks happenstance-sqlite and
> happenstance-cloudflare "yes, unpublished"; :563 calls them "entirely pre-consumer".
> crates.io says both are published and unyanked: sqlite 0.0.0 (2026-08-18),
> cloudflare 0.0.0 (2026-08-20). That fires the brief's own kill switch at :598-602 —
> "if happenstance-sqlite turns out to be published already, Option C is already off
> the table" — which it filed as untriggered only because it could not reach the
> registry.

All three passages named have been rewritten, the registry independently re-queried
on 2026-09-03 (results in *Registry state, verified*), and the kill switch resolved
in the argued form the objection demands rather than the asserted one.

**2. The lockstep objection — this one flipped the recommendation.** Recorded
verbatim as filed:

> The rejected option that survives is A scoped to published crates — D minus its
> lockstep sentence. D's only stated edge over A is the publication test, which is
> one scoping word on A's rule, not a separate option. Everything else D adds is the
> lockstep clause, and that collides with CF-32 [FROZEN] (spec:8684), which mandates
> testkit's own version key and reasons core and testkit moving together is "wrong in
> both directions". Testkit is one of the five in PUBLISHABLE, so "all five move
> together" is a published promise contradicting a frozen clause — and CF-32's guard
> is a manifest check, so a prose lockstep promise reinstates that coupling in the one
> medium the check cannot see. Land the four re-exports; route lockstep to its own ADR.
>
> FALSIFIED PREMISE: "No specification clause governs this" (:60) and "would supersede
> no accepted atom" (:606) — true of the re-export half only. The coverage grep
> searched spec/ for re-export|reexport|pub use, so it could never reach CF-32, a
> [FROZEN] clause about version keys; Option D then added a versioning half that grep
> never covered. The brief also calls testkit's own key a half-answer given "without
> saying so" (:286), citing only Cargo.toml:21 — the value, not the reason at :4-20.

Accepted in full and acted on: the lockstep clause is struck from Option D, the
recommendation is restated as A-scoped-to-published, and the three falsified sentences
are removed rather than rewritten around.

### The strongest argument against the re-export itself, in its own words

> `happenstance-core` does `pub use bytes;` on purpose: **it costs the crate a major
> bump whenever `bytes` takes one**.
> — `standards/rust/40-public-surface-and-evolution.md:173-174`

Stated at full strength: a public `pub use rusqlite;` on a crate that intends to
reach 1.0 chains that 1.0 to a `0.x` dependency's release cadence, in public, for
the life of the crate. `rusqlite` is at `0.40` (`Cargo.toml:72`) and `worker` at
`0.8.5` (`Cargo.toml:122`) — the audit notes the ecosystem "treats `0.8 → 0.9` as a
major and ships it without 1.0-grade caution" (`:2358`). A `happenstance-sqlite 1.0`
that must go to `2.0` because `rusqlite` went to `0.41` is a worse promise than one
whose driver is an implementation detail, and Option C is the only option that ever
gets to make the better promise. The honest counter to this counter is that
`happenstance-sqlite` is *already* in that position and merely has not said so; but
"already in it and silent" is a position you can still walk out of before publishing,
and the re-export is the move that welds the door shut.

---

## Cost of delay

**Not symmetric across the options, and the audit's framing understates one half
while overstating the other.**

**For Option A/D (re-export): cheap now, cheap forever, and the interval is what you
pay.** A `pub use` is additive at any version. The audit's "0.2.0 is the last moment
this costs one line" (`:2362`) is rhetorically strong and mechanically imprecise — it
costs one line at 0.3.0 too, and a consumer who wrote their own `rusqlite = "0.40"`
against 0.2.0 is not broken by a 0.3.0 re-export; their line simply becomes
redundant. What 0.2.0 actually buys is that **no consumer ever learns the wrong
idiom**, and that the version a caller must match is stated where they read it rather
than inferred from `cargo tree -d`. The audit's "a re-export added in 0.3.0 does not
retract the coupling a 0.2.0 consumer already compiled against" is true and is a
statement about the *population of consumers*, not about the API.

**For Option C (seal the surface): free now, breaking after publish.** This is the
half with a real deadline. Removing `rusqlite::Error` from a public variant, or
`rusqlite::types::Value` from `SqliteBatch::push`, is a major bump once
`happenstance-sqlite` is on the registry. `#[non_exhaustive]`
(`crates/happenstance-sqlite/src/event_store.rs:924`) does not help — it protects the
addition of variants, not the change of a payload.

~~**For the lockstep sentence: free while the five versions still agree.** They agree
today: `Cargo.toml:15` and `:39-41` put all of them at `0.2.0-alpha.1`, with
`happenstance-testkit` restating the same value under its own key
(`crates/happenstance-testkit/Cargo.toml:21`). Declaring lockstep now costs a
sentence per crate. Declaring it after the versions have drifted costs a
re-synchronisation release across five crates.~~

**REMOVED.** The whole paragraph rested on lockstep being cheap-now-costly-later, and
that pricing is beside the point once CF-32 [FROZEN] is in view: lockstep across the
five is not a cheap sentence at any date, because one of the five is
`happenstance-testkit`, whose independent number is a frozen clause with a manifest
check behind it. Its true cost is an ADR that amends a `[FROZEN]` clause, and that
cost does not fall over time. The observation that the five versions agree today is
kept only as a fact about the manifests, not as an argument for writing the agreement
down as a promise.

**For `pub use happenstance_core;` specifically: the window on `happenstance` is
already partly shut.** It is live at `0.2.0-alpha.1` without it (verified on the
registry, not merely per the audit) — but as a pre-release alpha whose manifest
comment says the API "is expected to keep moving" (`Cargo.toml:6-8`), the promise
being broken is thin.

The sentence that followed — "`happenstance-sqlite` and `happenstance-cloudflare` are
still entirely pre-consumer" — **has been removed as false.** Both are on crates.io.
What is true, and is all that this brief may rely on, is that their only release is a
zero-dependency `0.0.0` name reservation in a compatibility range disjoint from
`0.2.0`, so no consumer has ever received a driver type from either of them. That is
the narrower claim; the argument for it is in *Registry state, verified*.

---

## What this does not settle

1. **Whether `happenstance-cloudflare` gains a `[features]` table before it
   publishes.** The audit routes this as a second, separate decision (`:2366`) and I
   agree: the crate has no `[features]` table at all, `worker` is unconditional
   (`crates/happenstance-cloudflare/Cargo.toml:81`), and `worker` reaches the type of
   every construction path — so a feature that excludes it excludes the adapter. That
   is a design decision, not a re-export, and it is a candidate for
   `.kb/open-questions/` if 0.2.0 ships without it.
2. **`happenstance_core::prelude`.** It shares a RUNBOOK bullet with
   `pub use futures_core;` (`RUNBOOK.md:2869-2872`) and shares nothing else — the
   prelude is about `E0034` ambiguity between `EventStore` and `SendEventStore`
   (CLAUDE.md, binding constraint 4), not about type identity across crate versions.
   Retiring half of that bullet leaves the other half live.
3. **Whether `happenstance` should re-export the adapters.** ADR-0006 says
   `happenstance` "re-exports the contract and feature-gates the adapters"
   (`.kb/decisions/0006-bare-name-to-the-typed-layer.md:65-66`), and today it does
   neither of those two things in the stated form: the re-export is a glob of items
   (`crates/happenstance/src/lib.rs:242`) and there is no adapter dependency in
   `crates/happenstance/Cargo.toml` at all. If the adapters ever move behind
   `happenstance`'s features, the driver-re-export question changes shape and may not
   need this answer.
4. **What the `#[non_exhaustive]` error enums promise about driver payloads.** ES-6
   [FROZEN] (`spec/SPECIFICATION.md:2677`) endorses wrapping driver errors; nothing
   states whether the *wrapped type* is part of the promise. Option C depends
   entirely on the answer and this brief does not supply it.
5. **Whether RS-40-4 is the right home for the rule at all.** It is a constitution
   atom, and CLAUDE.md's precedence has an atom binding below a specification clause.
   If this becomes a decision atom, RS-40-4's Why/Not/Evidence all need repointing —
   which is what the audit's Routing paragraph means by "RS-40-4's 'Evidence' line
   will need repointing at whatever lands" (`:2366`).
6. ~~**Registry state.** I could not verify from this environment which crates are
   actually published at `0.2.0-alpha.1`. Every claim about that in this brief is the
   audit's, attributed. If `happenstance-sqlite` turns out to be published already,
   Option C is already off the table and the brief's central asymmetry collapses to
   "D or B".~~ **RESOLVED, not deferred — this escape hatch has fired.** The registry
   was queried on 2026-09-03: `happenstance-sqlite` and `happenstance-cloudflare` are
   both published and unyanked at `0.0.0`. The hatch as written would have knocked
   Option C out on a bare fact; it does not, but only because of an argument the
   original item did not contain — the published artifact carries no dependencies and
   lives in a disjoint `0.0.x` range, so it exposed no driver type to anyone. That
   argument is in *Registry state, verified* and is the thing a decider should attack
   if they want C off the table. **What is now genuinely unsettled in its place:**
   whether a live `0.0.0` name reservation should be yanked before a real `0.2.0`
   ships, so that this ambiguity does not recur at the next decision.
7. **The lockstep question, in full.** Whether the publishable crates state a version
   relationship to `happenstance-core`, and in what form, is now explicitly *not*
   settled here. It requires amending or superseding **CF-32 [FROZEN]**
   (`spec/SPECIFICATION.md:8684-8712`), so it needs its own ADR and its own brief. Two
   sub-questions it must separate, which the withdrawn clause conflated: (a) whether
   the *testkit* moves with the contract — CF-32 already answers this, **no**; and
   (b) whether an *adapter*'s number implies a `happenstance-core` version, which is
   the real consumer-facing hole the audit's item #2 describes and which CF-32 does
   not touch.

### Supersession

**The recommended decision — the re-export half — would supersede no accepted atom.**
The unqualified form of this sentence ("This decision would supersede no accepted
atom") has been **removed as falsified**: it was true of the re-export half and false
of the withdrawn lockstep half, which would have contradicted **CF-32 [FROZEN]**
(`spec/SPECIFICATION.md:8684`). With the lockstep clause struck, the statement holds
again, and it holds *because* the scope was narrowed rather than because the original
check was sound — the check (`grep` for `re-export|reexport|pub use` over `spec/`)
could not have found CF-32 whatever the answer was.

There is no decision atom
governing re-export policy: `grep -rln "pub use bytes" references/adr/ .kb/` returns
nothing, and ADR-0003's Decision section
(`.kb/decisions/0003-opaque-payloads.md:45-58`) settles the payload type without
mentioning the re-export that implements it. The only governing artifact is RS-40-4
(`standards/rust/40-public-surface-and-evolution.md:171-210`), a constitution atom
that is amended rather than superseded, and the unchecked RUNBOOK item at
`RUNBOOK.md:2869-2872`. Whatever lands should probably say so explicitly — that the
`pub use bytes;` precedent everyone cites has never itself been recorded as a
decision.

---

## Revision record

Revised 2026-09-03 after two adversarial critiques, both of which falsified a premise
the brief relied on. Nothing below was rewritten around; each falsified claim was
removed in place, with the removal marked.

**1. The recommendation flipped: Option D → Option A scoped to published crates.**

- *What flipped it:* Option D's distinguishing content was two things — a publication
  test on Option A's rule, and a lockstep sentence promising that the five publishable
  crates move together. The lockstep sentence collides with **CF-32 [FROZEN]**
  (`spec/SPECIFICATION.md:8684-8712`), which mandates `happenstance-testkit`'s own
  `version` key and states that a shared number is "wrong in both directions".
  `happenstance-testkit` is in `PUBLISHABLE` (`xtask/src/package.rs:86`), so the
  promise contradicts a frozen clause — and CF-32's guard is a *manifest* check, so a
  prose promise reinstates the coupling in the one medium the check cannot see. With
  the clause struck, D is Option A plus a scoping word, and the brief now says so.
- *Removed:* the lockstep bullet from Option D and its cost/foreclosure lines; the
  Recommendation paragraph claiming D "answers the lockstep question in the same
  breath"; the Cost-of-delay paragraph pricing lockstep as cheap-now-costly-later.
- *Substantively unchanged:* re-exporting still beats the status quo and still beats
  sealing, on the same evidence, and the `tokio` qualification stands.

**2. Falsified: "No specification clause governs this" (Why this is owed).**
Removed. The coverage check was `grep -n "re-export\|reexport\|pub use"` over
`spec/SPECIFICATION.md` — sound for re-exports, and structurally incapable of reaching
CF-32, a clause about version keys. Option D then added a versioning half that check
had never covered. The section now scopes the claim to the re-export half and names
CF-32 for the other.

**3. Falsified: "this decision would supersede no accepted atom" (Supersession).**
Removed in its unqualified form; restated as true of the re-export half only, with the
note that it holds because the scope narrowed, not because the original grep was sound.

**4. Falsified: testkit's own version key "half-answers the lockstep question without
saying so".** Removed. The claim cited `crates/happenstance-testkit/Cargo.toml:21` —
the value — and never read `:4-20`, seventeen lines of comment giving the reason;
CF-32 states the same reason as a clause with a machine check. The key answers the
question out loud, in the negative. The audit's `:242-244` is wrong on the same point
and the brief now says so instead of repeating it.

**5. Falsified: `happenstance-sqlite` and `happenstance-cloudflare` are unpublished.**
Removed from three places — the export-surface table's "publishable?" column, the
Cost-of-delay sentence calling them "entirely pre-consumer", and the "What this does
not settle" item that filed registry state as unverifiable. crates.io was queried
directly on 2026-09-03: both are published and unyanked at `0.0.0` (2026-08-18 and
2026-08-20 respectively), and `happenstance-core`, `happenstance` and
`happenstance-testkit` each carry `0.0.0` and `0.2.0-alpha.1`. A new *Registry state,
verified* subsection carries the data.

**6. Added: an argument for why a live `0.0.0` does not settle the question, replacing
an assertion that the crates did not exist.** `happenstance-sqlite 0.0.0` has **zero
dependencies** (`/api/v1/crates/happenstance-sqlite/0.0.0/dependencies` returns an
empty list), so it never linked `rusqlite` and never put a driver type in front of a
consumer; and `0.0.x` is its own Cargo compatibility range, conferring nothing on
`0.2.0`. Option C therefore remains additive-shaped, on that narrow ground rather than
on "the crate is unpublished" — the move ADR-0029 made with "nothing is published" and
which is not available a second time.

**7. The brief's own kill switch fired and is resolved rather than deleted.** Item 6
said that if `happenstance-sqlite` turned out to be published, "Option C is already
off the table". It is published; C stays on the table, but only via the argument in
point 6, which is now written down where a decider can attack it. In its place, item 6
records a new open question — whether the `0.0.0` reservations should be yanked before
a real release, so the same ambiguity does not recur — and a new item 7 files the
lockstep question as needing its own ADR against CF-32.

**Both critiques are reproduced verbatim** under *Recommendation → The strongest
arguments against*, in the order received.

---

## Ratified, and one member of the set removed — 2026-09-04

**Status: this brief is settled.** The repository owner ratified the recommendation,
the code landed (`D-1`/`D-4`), and then ruled on the one member the brief itself had
flagged as weakest.

**`tokio` is out of `happenstance-sqlite`'s re-export set.** The brief carried it on the
arithmetic Option A/C states — `tokio::task::JoinError` and
`tokio::runtime::TryCurrentError` are variants of the crate's public error enums, so the
crate's signatures name `tokio` — and recorded, at the site and in `RS-40-4`, that the
crate takes `tokio` at `features = ["rt"]`, so `happenstance_sqlite::tokio` was a
*partial* `tokio`. The owner ruled that the caveat does not rescue the path: a consumer
who reaches the type that way and then writes `#[tokio::main]` meets an `error[E0433]`
*further* from its cause than the `error[E0308]` the whole set exists to prevent, which
makes it a longer route to the type rather than a shorter one, and documentation that
has to be read twice before a path is safe is not a fix.

**A third option was offered and declined**, and it is recorded because it remains
available at `0.3.0` if the removal proves wrong: re-export only the two types that
actually appear in the signatures (`pub use tokio::task::JoinError;` and
`pub use tokio::runtime::TryCurrentError;`), which satisfies RS-40-4's arithmetic while
making nothing partial reachable. The owner chose the whole removal over it.

**What that does to `RS-40-4`, and why the atom was amended rather than left standing.**
As written, the rule said the re-export set is *"**exactly** the crates whose types
appear in that crate's own public signatures"*. Under this decision
`happenstance-sqlite` is a standing counter-example to its own constitution, so the atom
now says what is actually true: the arithmetic is a **necessary** condition and not a
sufficient one — a crate whose types appear in your signatures is a *candidate*, and it
earns the re-export only if the path handed to the reader is shorter than the one they
would have walked anyway. A crate taken at a partial feature set fails that second test.
The amendment carries the worked case, the requirement to state the omission at the
item, and the requirement to fence it.

**The omission is compiled, not merely documented.** `crates/happenstance-sqlite/src/lib.rs`
carries a `compile_fail,E0433` doctest on the exact path a reader following the old
documentation would take. It was proven non-vacuous by restoring `pub use tokio;` and
watching it report *"Test compiled successfully, but it's marked `compile_fail`"*. It is
in the **lib**, not an integration-test target, for `F1-04`'s reason: cargo never hands
those to a compiler, and a fence that is never compiled is decoration.

**What this does not settle.** Whether the residual identity risk is worth a line in
`SECURITY.md`-adjacent release notes: without the re-export, a consumer's `tokio` and
this crate's unify only while the two requirements stay semver-compatible, and a
consumer pinning a different *major* gets the two-types-that-print-identically failure
the set exists to prevent. `cargo tree -d` names it, and the `CHANGELOG.md` entry says
so; whether that is the right place for the warning to live is the release owner's.
