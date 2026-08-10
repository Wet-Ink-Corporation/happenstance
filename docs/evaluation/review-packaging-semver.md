# Review: crate layout, features, semver surface, supply chain, ADR-0006

**Lens:** packaging / features / semver / supply chain / CI gate / release readiness
**Date:** 2026-08-05 · **Tree:** `main` @ `9fd2337` (dirty: modified docs, untracked `.idea/`, `.mcp.json`, `docs/RUNBOOK.md`, `docs/adr/0006-*`)

Everything below was executed against the tree, not inferred. Commands run: `cargo xtask ci`
(green, 12s warm), `cargo hack check --workspace --feature-powerset --no-dev-deps` (22 combos,
green — cargo-hack installed during this review), `cargo hack check --workspace --no-dev-deps
--rust-version` (green under `rustup run 1.85`), `cargo deny check` (green with warnings —
cargo-deny installed during this review), `cargo package -p happenstance --list`,
`cargo tree -e features`, `cargo doc --no-default-features` (**fails**), plus four external
probe crates and four crates.io API queries.

---

## Headline

Three things must change before anything is published, and two of them are breaking:
`Query::Items` is a public tuple variant that lets any external crate construct the empty query
the crate's own constructor rejects — an `AppendCondition` built from it can never fire; and
`EventStore::Error` carries no `Send + Sync`, so no generic code can convert a store error into
`anyhow::Error`. Neither is fixable after 0.1. Separately, **both crates.io names are still
unreserved** — I checked the registry during this review — and nothing in the RUNBOOK's ledger
tracks it.

The packaging fundamentals are otherwise unusually good for a pre-1.0 workspace. The
dependency set is four crates, all healthy; only two of them leak into the public API and both
leaks are deliberate and documented; the feature powerset actually builds; the MSRV job actually
works. Most of what follows is closing holes in a gate that is already better than most.

---

## 1. The ADR-0006 rename, as an execution problem

### What must change

The mechanical inventory, from `grep` over the tree (excluding `target/`):

| Surface | Count / location | Change |
|---|---|---|
| Crate directories | `crates/happenstance/`, `crates/happenstance-runtime/` | swap |
| `[package] name` | 2 manifests | swap |
| `[workspace.dependencies]` | `Cargo.toml:24` | key `happenstance` → `happenstance-core`, add new `happenstance` |
| Member dep keys | testkit, sqlite, ladybug, sync, example — 5 manifests | `happenstance` → `happenstance-core` |
| Rust paths / rustdoc links | **53 occurrences of `happenstance::` across 22 files** | → `happenstance_core::` |
| Doctests | every `use happenstance::…` in `event.rs`, `query.rs`, `append.rs`, `error.rs`, `store.rs`, `tag.rs`, `memory.rs` | same |
| `xtask` wasm step | `xtask/src/main.rs:69` — `"-p", "happenstance"` | → `happenstance-core` |
| CI semver job | `.github/workflows/ci.yml:77` — `package: happenstance, happenstance-testkit` | → `happenstance-core, …` |
| `docs.rs` metadata | `crates/happenstance/Cargo.toml:36-38` | moves with the crate; the *new* `happenstance` needs its own |
| Docs | `README.md` (status table, quick start, design section), `CLAUDE.md` (repo map + 5 name mentions in the constraints), `CONTRIBUTING.md`, `docs/RUNBOOK.md` (27 mentions) | rewrite |
| ADR cross-references | ADR-0001/0003/0004 (2, 5, 2 mentions) | see below — a real decision, not a `sed` |

The single highest-risk item is `xtask/src/main.rs:69`. The wasm step is the *only* thing keeping
constraint 1 honest, and if the `-p happenstance` argument is not repointed it will silently start
checking the typed layer on `wasm32` — which will compile, and will prove nothing. A rename that
leaves that line alone produces a green gate that has stopped testing the design it exists to
protect.

### The ADR bodies are the non-mechanical part

`CONTRIBUTING.md:8-10` and ADR-0005's "On the historical record" section establish that a
superseded ADR body is left verbatim, but that ADRs 0001/0003/0004 *were* rewritten for the
`eventum` → `happenstance` rename "because renaming a project changes an identifier, not a
decision." ADR-0006 does not say which rule applies to it, and the distinction matters because
this rename **inverts the meaning of a live constraint**:

> **CLAUDE.md, constraint 2:** "Never put `serde` in `happenstance`'s default features."

After the rename that sentence is not merely stale, it is *false*: `serde` is exactly what the new
`happenstance` is for (`Codec`, JSON/CBOR/postcard, ADR-0006's decision table). ADR-0006
§"The `serde` boundary moves with the contract, not with the name" anticipates the confusion
without instructing an edit. The same applies to ADR-0003's body and to standing constraints 2 and
3 in `docs/RUNBOOK.md:472-477`.

**Recommendation:** ADR-0006 gains an explicit "On the historical record" section saying that
ADR-0001/0003/0004 and the two constraint restatements are rewritten to `happenstance-core`,
because they were always about *the contract crate*, not about a string — and that ADR-0005's
body stays verbatim because it is a factual record about which name went where.

### Do it now, not in phase 3

The RUNBOOK schedules the rename as the first checkbox of phase 3
(`docs/RUNBOOK.md:223-225`) and calls it "Mechanical, and cheapest before the worked example is
rewritten." That is the right instinct applied one phase too late.

- The cost is monotonically increasing and phases 1–2 are documentation-heavy: a SQLite ADR,
  a projection conformance suite, a port-freeze ADR, and the projection runner. All of it would be
  authored against names already known to be wrong.
- Drift has already started. `crates/happenstance-runtime/src/lib.rs:10` says *"Even the crate name
  is provisional"* and lines 37-40 list the projection runner as belonging to that crate — both
  contradicted by ADR-0006, which was accepted the same day.
- Doing it in phase 3 means one commit that mixes a 22-file mechanical rename with the design of
  the typed layer. That is the worst possible commit to review or bisect.
- ADR-0006 itself already rejected deferral on exactly this reasoning, for the *decision*
  ("The cost of deciding rises monotonically and the information does not arrive," lines 143-145).
  The same argument applies verbatim to the *execution*, and the ADR does not notice.

The one honest objection is that between the rename and phase 3, the bare name points at a crate
whose entire public API is `pub const STATUS: &str`, making `README.md:82-100`'s `cargo add
happenstance` quick start a lie. **That objection dissolves for five lines of code**: make the new
`happenstance` a facade *immediately* —

```rust
//! The happenstance event sourcing library. Re-exports the contract from
//! `happenstance-core`; the typed layer lands in phase 3.
pub use happenstance_core::*;
pub use happenstance_core as core_contract;
```

— which is what ADR-0006 says the crate does anyway ("Re-exports the contract"). The README stays
true, `cargo add happenstance` works from day one, and phase 3 becomes purely additive.

### Reserve both names — this week

ADR-0005 §Follow-up flags it; ADR-0006 §Consequences repeats it ("still outstanding and now covers
**two** names"). Nothing tracks it: it is not a row in the RUNBOOK decision ledger
(`docs/RUNBOOK.md:61-75`) and not a checkbox in phase 7 (lines 435-450). I queried the registry
during this review:

```
GET https://crates.io/api/v1/crates?q=happenstance  →  0 crates matching the prefix
```

Both `happenstance` and `happenstance-core` are free as of 2026-08-05. Effort to close: publish two
`0.0.0` placeholders with a description and repository link (~15 minutes). Consequence of not
doing it: two accepted ADRs reopened and a third rename. This is the highest
value-per-minute item in the whole review.

---

## 2. Feature flags

### `memory` in the default set — correct, and I tested the objection

The concern is that a *contract* crate defaults to shipping a reference implementation. I measured
it: two release builds of an external crate that depends on `happenstance` and never mentions
`MemoryEventStore`, one with defaults and one with `default-features = false, features = ["std"]`,
`lto = "thin"`, `codegen-units = 1`, `panic = "abort"`:

```
with memory (default): 124416 bytes
without memory:        124416 bytes
```

Byte-identical. `MemoryEventStore` is a concrete, non-generic type; its symbols land in a dedicated
section and `--gc-sections` drops them. The only residual costs are ~370 lines of compile time and
the fact that `MemoryEventStore`, `MemoryStoreError`, `with_events`, `snapshot` and
`last_position` are permanent public API. Both are worth paying: the store is the oracle the
conformance suite validates against, it backs every doctest so the docs cannot rot, and after
ADR-0006 it lives in `happenstance-core` — the crate *adapter authors* pin, who are precisely the
people who want a reference store to diff against. **Keep it.**

### `std` vs `no_std` + `alloc` — earning its keep, and the brief's claim about it is wrong

The assignment brief says "the `no_std` path is never built for any target." That is not correct,
and I checked before writing it up. `cargo hack check --workspace --feature-powerset
--no-dev-deps` runs each combination with `--no-default-features`, so the `no_std` configuration
*is* compiled on the host — and `#![no_std]` makes `std::` paths unresolvable regardless of target,
so a host `cargo check` catches accidental `std` usage exactly as well as an embedded one would.
I confirmed all three by hand:

```
cargo check -p happenstance --no-default-features                              → ok
cargo check -p happenstance --no-default-features --features serde             → ok
cargo check -p happenstance --target wasm32-unknown-unknown --no-default-features → ok
```

The narrower true statement is that `wasm32 × no_std` is never checked *in the gate*
(`xtask/src/main.rs:64-77` pins `--features std`), and the powerset never runs against wasm32 at
all. That is a small hole with a one-line fix — see finding `wasm-step-only-checks-std`. The
`std`/`alloc` split itself costs a handful of `alloc::` imports and three feature lines; keep it.

### `serde` wiring — one real defect

`std = ["thiserror/std", "bytes/std", "futures-core/std", "serde?/std"]` uses the weak-dependency
`?` syntax correctly, and `serde = ["dep:serde", "bytes/serde"]` correctly enables the transitive
`Bytes` impls. But `serde_impls` calls `String::deserialize` (`tag.rs:305`, `event.rs:317`) and
`Vec::<Tag>::deserialize` (`tag.rs:319`), and those impls live behind serde's own `alloc` feature.
`happenstance` never asks for it — the workspace entry is
`serde = { version = "1", default-features = false, features = ["derive"] }` (`Cargo.toml:31`).
It compiles today only because `bytes` happens to enable it:

```
$ cargo tree -p happenstance --no-default-features --features serde -e features
happenstance v0.1.0
├── bytes v1.12.1
│   └── serde feature "alloc"      ← the only thing turning it on
```

One word fixes it: `serde = ["dep:serde", "bytes/serde", "serde/alloc"]`.

### Adapter `event-store` / `projection-store` features

Wired correctly (`happenstance-sqlite/src/lib.rs:38-42`) and all four combinations build. But they
currently gate only module declarations while `futures-core` and `thiserror` are unconditional
dependencies, so `--no-default-features` saves nothing. The features only start earning their keep
if the driver is `optional = true` and named in them —
`event-store = ["dep:rusqlite"]` — which is the shape to commit to now, before phase 1 wires
`rusqlite` in unconditionally out of habit.

---

## 3. Semver surface

Full public inventory of `happenstance`, by stability risk.

**Landmines (fix before 0.1; both fixes are breaking):**

| Item | Location | Risk |
|---|---|---|
| `Query::Items(Box<[QueryItem]>)` | `query.rs:143-150` | public variant, enum not `#[non_exhaustive]` → external construction of an illegal empty query; see finding 1 |
| `type Error: core::error::Error + 'static` | `store.rs:99`, `projection.rs:73` | no `Send + Sync` → `anyhow`/`eyre` interop impossible in generic code; see finding 4 |

**Protected, but not where the RUNBOOK thinks:**

`SequencedEvent` (`event.rs:275-282`), `ReadOptions` (`query.rs:223-232`) and `AppendCondition`
(`append.rs:50-61`) all carry `#[non_exhaustive]` with `pub` fields. That combination is exactly
right — the attribute blocks external struct-literal construction (so a field can be added
non-breakingly) while keeping the fields readable and assignable. The RUNBOOK's "release hazard"
(lines 390-412) says event identity "may require a field on `Event`… That is a change to
`happenstance`'s **public API**" and cites `#[non_exhaustive]` as the mitigation. The attribute is
not the binding constraint. **The constructors are:**

```rust
pub const fn new(position: SequencePosition, event: Event) -> Self   // event.rs:286
```

Adding a store-assigned `EventId` to `SequencedEvent` breaks `SequencedEvent::new` for every
adapter that ever calls it — and every adapter must, since it is how a store hands back what it
read. Same for `Event::new` (`event.rs:197`) and `Event::into_parts`
(`event.rs:244`, a 4-tuple that becomes a 5-tuple). Plan the identity change around the
constructors, not the fields.

**Deliberate, correct, and worth naming as such:**

- `AppendError<E>` generic over the adapter error rather than boxing (`error.rs:150`). Preserves
  the concrete type, costs nothing, and `map_store` gives the wrapping escape hatch.
- `#[non_exhaustive]` on every public struct and enum *except* `Query`.
- Errors are `#[non_exhaustive]` enums with `#[error(transparent)]` on the wrapped cases — the
  right shape for a foundation crate.
- The GAT `type Batch<'a> where Self: 'a` (`projection.rs:80-82`). GATs are the only way to express
  "a transaction borrowing its connection" without boxing, and the reasoning is written down at
  `projection.rs:23-26`. The risk is not the GAT, it is that the port is **provisional and
  publicly re-exported** (`lib.rs:95`), so shipping 0.1 freezes a shape the RUNBOOK says is a guess
  until phase 2.

**`cargo-semver-checks` currently does nothing.** `.github/workflows/ci.yml:66-77` runs it on pull
requests against `package: happenstance, happenstance-testkit`, and neither is published — there is
no baseline to compare to, so the job is at best a no-op and at worst a hard error nobody has seen
(the branch is `main` with direct commits; there may never have been a PR). `CONTRIBUTING.md:80-81`
tells contributors it protects them. It does not. Fix in one line: `cargo-semver-checks` accepts
`--baseline-rev`, which works today with no registry involvement.

---

## 4. Dependencies and supply chain

Four runtime dependencies. I pulled current registry data for each rather than relying on memory:

| Crate | In-tree | Latest | Last release | Recent downloads | Public API? |
|---|---|---|---|---|---|
| `bytes` | 1.12.1 | 1.12.1 | 2026-07-08 | 211.5M | **yes** — `Event::data`, and `pub use bytes` (`lib.rs:106`) |
| `futures-core` | 0.3.33 | 0.3.33 | 2026-07-18 | — | **yes** — `EventStore::read` return type |
| `thiserror` | 2.0.19 | 2.0.19 | 2026-07-18 | 313.9M | no — derives `Display`/`Error` only |
| `trait-variant` | 0.1.3 | 0.1.3 | 2026-07-22 | 3.1M | no — generates a trait happenstance owns |

No 2.x of `bytes` exists; no 0.4.x of `futures-core` exists, and 0.3 has been the stable line since
2019. So the "what if `bytes` 2.0 lands" question is a genuine tail risk with no evidence behind it
today, and the right answer is the one `tokio`, `hyper` and `axum` all took: accept it and write the
policy down. Add to `CONTRIBUTING.md` or ADR-0004's Policy section: *a major bump of `bytes` or
`futures-core` is a major bump of `happenstance-core`, because both appear in the public API.*
That is the whole mitigation, and stating it is worth more than avoiding it.

**The asymmetry is the actual bug.** `bytes` is re-exported (`lib.rs:104-106`) with a comment
explaining that adapters should not need their own dependency — but `futures_core` is *not*, even
though `Stream` appears in the trait signature and therefore every single adapter must name it.
`happenstance-sqlite/Cargo.toml:16` already adds `futures-core.workspace = true` for exactly this
reason. Inside the workspace it unifies; a third-party adapter author writing
`futures-core = "0.3"` also unifies, but the one that is *mandatory* to name is the one that is not
re-exported. `pub use futures_core;` costs one line.

**`thiserror` in a `no_std` contract crate is the right call.** thiserror 2 targets
`core::error::Error` when `std` is off, it requires Rust 1.71 (well under the 1.85 MSRV), it is
314M-downloads maintained, and — the part that matters for a foundation crate — *no thiserror type
appears in the public API*. Hand-writing the five error types would save two build-time
dependencies (`thiserror-impl` + `syn`) at the cost of ~120 lines of `Display` boilerplate that
will rot. Not worth it. Same verdict on `trait-variant`: 0.1.x looks alarming, but it is a
`rust-lang/impl-trait-utils` crate with 3.1M recent downloads and a release two weeks ago, and it
is load-bearing for constraint 1. The escape hatch, if it ever matters, is ~40 lines of
hand-written `SendEventStore` + blanket impl.

**`deny.toml`.** Passes today (`advisories ok, bans ok, licenses ok, sources ok`) with two classes
of warning. The licence allowlist is well-chosen; `wildcards = "deny"` and
`unknown-registry/unknown-git = "deny"` are the right defaults. `multiple-versions = "warn"` is
also right at this stage — it fires once, on `syn 2.0.119` vs `syn 3.0.3`, caused solely by
`trait-variant` not yet having moved to syn 3. That costs a few seconds of proc-macro compile and
nothing else; promoting it to `deny` now would fail the gate over someone else's dependency.
Revisit at 1.0.

The one gap is temporal: `cargo deny check advisories` only runs on push and pull request
(`ci.yml:3-7`). Advisories are published continuously; a repo with a quiet fortnight has no
advisory coverage for a fortnight. Add `schedule: - cron:` weekly.

---

## 5. The CI gate

What it does today, verified end to end (`cargo xtask ci`, 12s warm, green):
fmt · clippy `--all-targets --all-features -D warnings` · `cargo test --workspace --all-features`
(79 tests) · one wasm32 check · `cargo doc --workspace --all-features --no-deps
--document-private-items` · feature powerset (22 combos) · `cargo deny check`. Three OSes.
Separate MSRV job. This is a better gate than most 1.0 crates ship with.

Confirmed working, against my own suspicion: **the MSRV job is real.** `cargo hack --rust-version`
shells out to `rustup run 1.85 cargo check` per package, and `rustup run` overrides
`rust-toolchain.toml` — so the pinned 1.97.1 does *not* shadow it. I ran it locally and it passes,
which also answers ADR-0004's recorded caveat that the MSRV "has never been checked against a local
1.85 toolchain" (line 14). It has now.

Also confirmed: `broken_intra_doc_links = "deny"` in `[workspace.lints.rustdoc]` really does reach
rustdoc — I saw the exact diagnostic (`requested on the command line with
-D rustdoc::broken-intra-doc-links`). The lints table is wired correctly.

What is missing, in priority order:

1. **`cargo doc --no-default-features`.** It fails *today*, with three hard errors — see finding 7.
   Never run.
2. **A docs.rs simulation.** `crates/happenstance/Cargo.toml:36-38` sets
   `rustdoc-args = ["--cfg", "docsrs"]`, `lib.rs:75` opts into
   `#![cfg_attr(docsrs, feature(doc_cfg))]`, and `lib.rs:89/101` carry `doc(cfg(...))` attributes.
   **None of that is ever compiled by CI** — the `docsrs` cfg is set only on docs.rs, i.e. after
   publish, when a failure is unfixable except by a new release. Add a nightly job:
   `RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc --all-features --no-deps`.
3. **`cargo-semver-checks --baseline-rev`** — turns a dead job into a live one, today.
4. **`cargo package` / `--dry-run` in CI** — would have caught the missing licence files (finding 5).
5. **Serde round-trip tests** — finding 8.
6. **Scheduled advisory run** — §4 above.
7. **MSRV against the powerset**, not just default features:
   `cargo hack check --workspace --no-dev-deps --rust-version --feature-powerset`.
8. **`--locked`** on the CI build so a stale committed `Cargo.lock` fails rather than being
   silently regenerated.
9. **Benchmarks.** The stated goal is "fast, efficient" and there is not one number anywhere in the
   repository. The on-brand answer is not a `benches/` directory in the contract crate but a
   `criterion` harness in `happenstance-testkit` that adapters inherit the way they inherit the
   conformance suite — `event_store_benchmarks!(MyStore::new())`. That makes "which adapter is
   faster" answerable by the same mechanism that makes "is this adapter correct" answerable, and it
   is the sort of thing that gets a library talked about.

Two things I looked at and decided **not** to recommend:

- **miri.** `unsafe_code = "forbid"` is set workspace-wide (`Cargo.toml:42`) and there is no
  `unsafe` anywhere, so miri would only exercise `bytes`' internals, which `bytes` already tests.
  Low value; skip until an adapter introduces `unsafe`.
- **Code coverage.** The conformance suite is a behavioural contract, not a line-count exercise;
  a coverage number would measure the wrong thing and invite padding. Revisit after the SQLite
  adapter, where uncovered error paths become a real question.

One nit in the gate's own definition: `xtask/src/main.rs:121` selects the wasm step by array index
(`&REQUIRED[3..4]`). Inserting a step above it silently changes what `cargo xtask wasm` runs. Match
on `step.name` instead.

---

## 6. Publishing readiness — everything blocking a credible 0.1

| Blocker | Evidence | Effort |
|---|---|---|
| `Query::Items` constructible empty | finding 1 — breaking to fix | small |
| `Self::Error` lacks `Send + Sync` | finding 4 — breaking to fix | small |
| No licence files in the package | `cargo package --list` shows 13 files, none of them `LICENSE-*` | trivial |
| `readme = false` in the generated manifest | crates.io renders an empty page | small |
| No `CHANGELOG.md` | absent from the tree | small |
| `clippy::todo = "allow"` workspace-wide | `Cargo.toml:59` | trivial |
| `cargo-semver-checks` has no baseline | `ci.yml:66-77` | trivial |
| Projection port frozen (phase 2) or feature-gated | `projection.rs:1-11`, `lib.rs:95` | medium |
| Event identity (phase 3) | RUNBOOK release hazard | medium |
| Names unreserved | crates.io query above | trivial |
| Serde wire format untested | finding 8 | small |
| `docsrs` cfg never compiled | §5 item 2 | trivial |

Not blocking but worth doing in the same pass: `[workspace.package] homepage` and `documentation`
keys (both absent — `documentation` defaults to docs.rs, which is fine, but `homepage` is what
crates.io links); `SECURITY.md`; a `dependabot.yml`; and the phase-0 decision on `.idea/` and
`.mcp.json`, which are still untracked and make "clean tree" ambiguous
(`docs/RUNBOOK.md:92-93`).

On `clippy::todo`: it is `allow` to accommodate exactly **two** `todo!()`s, both in
`happenstance-sqlite/src/event_store.rs` (lines 66 and 78), both in a `publish = false` crate. The
workspace-wide allow means a `todo!()` introduced into `happenstance` or `happenstance-testkit`
today would never be caught — in the two crates that *are* publishable. Scope it now:
`#![allow(clippy::todo)]` at the top of the two stub crates, `todo = "deny"` in
`[workspace.lints.clippy]`. That closes the hole permanently and deletes a phase-7 checklist item
(`docs/RUNBOOK.md:443-446`) rather than deferring it.

---

## What is right (do not relitigate)

- **The workspace dependency table**, and specifically `default-features = false` on the *internal*
  deps with the comment at `Cargo.toml:20-23` explaining that Cargo forbids a member from removing
  a workspace default. That is a subtle, correct call that most workspaces get wrong.
- **`[lints] workspace = true` including a `rustdoc` table.** Verified to reach rustdoc.
- **`memory` in the default feature set.** Measured: zero binary-size cost.
- **`std` / `no_std` + `alloc`.** Builds on host and on wasm32; costs almost nothing; keep.
- **The MSRV job.** Verified to actually run 1.85 and to pass.
- **The dependency set.** Four crates, all healthy, all current; only `bytes` and `futures-core`
  leak into the public API and both leaks are deliberate.
- **`AppendError<E>` generic rather than boxed.**
- **`#[non_exhaustive]` discipline** on every public struct and enum except `Query`.
- **Adapters as separate crates, not features** (`README.md:128-131`) — correct, and the
  `happenstance-ladybug` manifest comment (lines 18-22) explaining why `lbug` is deliberately not a
  dependency yet is the right kind of note to leave.
- **The xtask pattern.** One gate, one definition, 12 seconds, green.
- **`deny.toml`.** Passes; the allowlist is right; `multiple-versions = "warn"` is the correct
  setting at this stage.
