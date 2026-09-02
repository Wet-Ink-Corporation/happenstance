---
item: HS-S0005
stage: spec
created: 2026-08-12T13:45:59.653Z
updated: 2026-08-12T13:45:59.653Z
template_sig: 87bbf1d0
rendered_sig: adee8869
---

# Spec — ProjectionProbe behind happenstance-core's conformance feature

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project charter | `.bklg/from-contract-to-published-library/projection-store-freeze/project.md` |
| This spec | `.bklg/from-contract-to-published-library/projection-store-freeze/projection-probe-conformance-feature/spec.md` |
| This story's discover | `.bklg/from-contract-to-published-library/projection-store-freeze/projection-probe-conformance-feature/discover.md` |
| Key briefs | `.bklg/.../projection-store-freeze/_decomposition.md` — Architecture **AC-A02** (`:299-302`), the mount-point table (`:340-356`), Note 4's write seam (`:464-487`), Note 9's DT-8 blast radius (`:689-693`), the rustdoc hazard (`:694-699`), Testing brief AC-009 (`:779`) |
| Signed-off design | `.bklg/.../projection-store-freeze/_design.md` — `surfaces: []`, no user-facing surface, approved 2026-08-12 |
| Story map row | `.bklg/.../projection-store-freeze/_storymap.md:57` (slice `projection-port-and-probe`, merge order 2) |
| Grounding | `.bklg/.../projection-store-freeze/_grounding.md:27-37` — "there is **no `apply` seam**" |
| Specification (wins on conflict) | `spec/SPECIFICATION.md:4977-5031` (PS-11 + the probe verbatim + the coherence argument), `:5041-5074` (PS-12 and CF-18's gate) |
| Roadmap pointer | `RUNBOOK.md:3882-3889` — "**The write seam** (PS-9 – PS-11). The port grows one *because the suite needs one*"; phase 6 in full at `RUNBOOK.md:3848-3965` |

## One-line PR slice

Add `ProjectionProbe` (`READS_THROUGH_BATCH`, `probe_write`, `probe_delete_all`,
`probe_read`, `probe_read_through`) to `happenstance-core` behind a new
`conformance` feature — mounted in the export block with
`#[cfg_attr(docsrs, doc(cfg(…)))]` and in `Cargo.toml`'s feature table — with a
round-trip test that writes and reads through nothing but the trait, and the host
and wasm32 feature-powersets green over the widened combination set.

## Executive summary

**Today generic code holding an adapter's projection batch can do exactly two
things with it: `commit` it or `rollback` it** (`crates/happenstance-core/src/projection.rs:126-138`).
There is no way to put a row *into* it and no way to look at the read model
afterwards, which means the rule that carries this port's entire reason for
existing — read-model write and checkpoint write become durable together or not
at all, PS-1, `[FROZEN]` — cannot be written. A suite built on today's port
degenerates into a checkpoint test, and `CheckpointOnlyStore` (the store this
project exists to fail) passes it.

This PR lands the seam that closes that gap, and only that. It adds one trait,
one feature, and one test:

- **The trait**: `ProjectionProbe: ProjectionStore`, taken **verbatim** from
  `spec/SPECIFICATION.md:4998-5031` — five members, bare flavour only, no
  `trait_variant::make`.
- **The feature**: `conformance` in `happenstance-core`'s `[features]`, pulling
  in no dependency and **not** implying `memory`.
- **The mounts**: the `pub use` in `lib.rs`'s export block under the same
  `#[cfg]`/`#[cfg_attr(docsrs, doc(cfg(…)))]` pair the `memory` items already
  carry (`crates/happenstance-core/src/lib.rs:101-103,120-122`), plus the row in
  the crate doc's `# Feature flags` list (`lib.rs:75-82`).
- **The falsifier**: one integration test, generic over `P: ProjectionProbe`,
  that writes a probe row and reads it back through nothing but the trait.

**Delta against the story's dependency.** `owned-batch-port-shape` lands §4.0's
trait — `type Batch;` with no lifetime — and that is what makes this story
writable at all: a probe method taking `&mut Self::Batch` is unspellable against
today's borrowing GAT `type Batch<'a> where Self: 'a`
(`projection.rs:97-99`). The story map splits AC-009 exactly there: the
**signature half** is the dependency's, the **seam half** is this one's
(`_storymap.md:87`).

**What this PR does not settle.** No conformance rule is written here (they begin
at `projection-suite-entry-point`), no `MemoryProjectionStore` is shipped (that
is the slice-mate `memory-projection-store`), and no `[FROZEN]` clause is
touched: PS-11 and PS-12 are both `[PROVISIONAL]`
(`spec/SPECIFICATION.md:4980,5055`) and this story implements them rather than
amending their text.

## Context pack

Read this section before writing code. Everything deeper is an anchor.

### D1 — The probe lives in `happenstance-core`, not `happenstance-testkit`. This is settled and is not re-litigable on diff size.

Architecture brief **AC-A02** (`_decomposition.md:299-302`) fixes the placement,
and `spec/SPECIFICATION.md:5015-5031` gives the reason in full. Restated so it
does not have to be rediscovered:

An adapter crate implementing a *testkit* trait for its own type is legal — the
type is local, the orphan rule is satisfied. But the natural place to write such
an impl is the adapter's `tests/` directory, and **that is a different crate**.
There, neither the trait nor the type is local, so `impl ProjectionProbe for
MyStore` is rejected by the orphan rule, and the only way out is a **non-dev
dependency** on `happenstance-testkit`, feature-gated. Putting the trait beside
the port costs one feature flag on a dependency the adapter already has, and no
new edge in the graph.

**Nothing inside this workspace can fail the wrong version of this decision.**
Every fixture here already lives in a crate that depends on the testkit; the
trait would be local, the impls local, the orphan rule silent, and
`cargo xtask ci` green in every step including both powersets. The falsifier is
`documented-extension-surface` (HS-S0015), five slices later, which builds an
outside author's fixture from documentation alone (`_decomposition.md:689-693`).
Write that fact into the probe's rustdoc so the placement is not quietly
"simplified" in the meantime.

### D2 — Take the trait verbatim. Where this spec and the specification disagree, the specification wins.

`spec/SPECIFICATION.md:4998-5031` gives the shape, including its key and value
types (`&str` / `u64`) and its doc comments:

```rust
// the contract crate, behind `feature = "conformance"`.
pub trait ProjectionProbe: ProjectionStore {
    /// Whether this adapter offers any read path on an open batch. See PS-12.
    const READS_THROUGH_BATCH: bool;

    fn probe_write(&self, batch: &mut Self::Batch, key: &str, value: u64);
    fn probe_delete_all(&self, batch: &mut Self::Batch);
    async fn probe_read(&self, key: &str) -> Result<Option<u64>, Self::Error>;

    /// Only called when `READS_THROUGH_BATCH`; may be `unimplemented!()` otherwise.
    fn probe_read_through(&self, batch: &Self::Batch, key: &str) -> Option<u64>;
}
```

This closes discover's deferred questions 3 and 4 (`discover.md:44-52`): the key
and value types are `&str`/`u64` as given, and the probe is a **supertrait
bound**, not a free-standing trait — `ProjectionProbe: ProjectionStore`, so
`Self::Batch` and `Self::Error` resolve through the port rather than being
re-declared. Two members are load-bearing beyond the round trip and must not be
dropped because nothing calls them yet:

- **`probe_delete_all`** exists so `reset` (PS-16) is checkable *without the
  suite knowing what a read model is* (`spec/SPECIFICATION.md:5027`). It is not
  optional garnish; dropping it makes `reset_clears_rows_and_checkpoint_together`
  unwritable two slices later.
- **`READS_THROUGH_BATCH`** is a `const` gate, and CF-18 requires an adapter
  declaring `false` to emit the PS-12 rule as a **reported skip** carrying its
  reason, never to omit it (`spec/SPECIFICATION.md:5052-5074`). The const is
  declared here; its `false` arm becomes observable in
  `read-through-and-rebuild-rules`.

Note the specification's spelling in `probe_read_through`'s doc: `unimplemented!()`,
not `todo!()`. That is enforceable rather than stylistic — this workspace denies
`clippy::todo` (`Cargo.toml` `[workspace.lints.clippy]`) and does not lint
`unimplemented`, so a declining adapter that copies the doc comment compiles and
one that reaches for `todo!()` does not.

### D3 — Bare flavour only, and one trait serves both flavours anyway.

The probe is **not** run through `#[trait_variant::make]`. The specification's
own comment gives the reason: suite code binds the weaker trait (CLAUDE.md rule
4) and the per-test wrapper is a parameter (CF-23), so no `Send` bound is needed
anywhere.

That is not a gap in the "both flavours" obligation, and the Rust reason is worth
having in hand: `trait_variant` emits a **blanket impl**, so `SendProjectionStore`
implies `ProjectionStore` (`.kb/decisions/0001-async-port-flavours.md:54`,
`references/adr/0001-async-port-flavours.md:72-76`). A `Send` adapter that
implements `SendProjectionStore` therefore already satisfies `ProjectionProbe`'s
supertrait bound, and one probe trait covers both flavours. Do not add a
`SendProjectionProbe`; it would collide with that blanket impl for exactly the
reason ADR-0008 records (`references/adr/0008-one-derivation-for-both-ports.md:103`,
`error[E0275]`).

**Consequence at the call site**: import `ProjectionStore` *or*
`SendProjectionStore` in a module, never both — with both in scope, `P::Batch`
and the method calls become ambiguous (CLAUDE.md, binding constraint 4).

### D4 — The item is `#[cfg]`-gated inside `projection.rs`. It does not get its own module.

The alternative — a new `conformance` module gated like `memory`
(`lib.rs:101-103`) — loses on two counts. First, the probe needs `Self::Batch`
and `Self::Error` from the port it is a supertrait of; it is part of the port's
contract (PS-11: an adapter **MUST** implement it), not a satellite. Second, a
gated *module* adds a second name that intra-doc links can fail to resolve on
configurations this crate is documented under, and that hazard is one this
workspace has already paid for once (D13). Keeping the gate at item level means
there is exactly one gated name to be careful about.

What is copied from the `memory` pattern is the **attribute pair on the
re-export** — `#[cfg(feature = "conformance")]` plus
`#[cfg_attr(docsrs, doc(cfg(feature = "conformance")))]`, matching
`lib.rs:120-122` — so docs.rs renders the feature badge (`all-features = true` is
already set at `Cargo.toml:54-56`).

### D5 — The rustdoc hazard is a hard error, and the gate step that finds it is already there.

`rustdoc::broken_intra_doc_links` is `deny` at the workspace root, and
`cargo xtask ci` runs `cargo doc -p happenstance-core --no-default-features` with
`RUSTDOCFLAGS=-D warnings` (`xtask/src/main.rs:494-514`). **Any intra-doc link
from ungated documentation into the `conformance`-gated `ProjectionProbe` fails
that step.** The established discipline is to spell the name plainly instead, and
the two places that already do it say why: `crates/happenstance-core/src/lib.rs:71-73`
("The name is deliberately not a link here… D13") and
`crates/happenstance-testkit/src/lib.rs:112-132`. Follow it. Links *out* of the
gated item into ungated ones (`ProjectionStore`, `ProjectionId`) are fine — the
danger is only inbound.

### D6 — `conformance` must not imply `memory`, and the powerset is what proves it.

Adding one feature to `happenstance-core` widens two gate steps: the workspace
feature powerset (`xtask/src/main.rs:546-556`) and the **wasm32** powerset, which
names `happenstance-core` explicitly (`xtask/src/main.rs:564-593`). Core has
three features today (`std`, `memory`, `serde`); `conformance` takes eight
combinations to sixteen on each target, and **every one must compile — including
`conformance` without `memory`, and `conformance` under `--no-default-features`,
which is `no_std`** (`_decomposition.md:483-487`).

This is the second named wrong implementation (`discover.md:95-103`): a
`conformance` surface that only compiles alongside `memory` is trivially arrived
at, because the round-trip test wants a store and `MemoryProjectionStore` is the
obvious store. It is green on default features and on `--all-features`, and it is
caught only by a step people skip locally. Making the independence a *stated
deliverable of this story* rather than a consequence of the gate is the
difference between finding it here and finding it in
`whole-gate-run-and-proof-artefact`.

Practically: the trait body may name only `core`/`alloc` and the port's own
types. `probe_read` returns `Result<Option<u64>, Self::Error>`, all of which is
already `no_std`-clean.

### D7 — The round trip is generic over the trait and does not use the oracle.

Testing brief AC-009 (`_decomposition.md:779`) asks for **Static** plus **Unit**:
`type Batch;` with no lifetime is enforced at every call site by `cargo check`,
and a test calls `probe_write` then `probe_read` "through nothing but the trait,
proving the seam is generic rather than merely present."

The decision this spec makes, with the alternative named: the round trip is a
**generic function bound on `P: ProjectionProbe`**, instantiated by a **small
store defined in the test file itself**, not by `MemoryProjectionStore`. Three
reasons:

1. `MemoryProjectionStore` lands in the slice-mate `memory-projection-store`,
   which depends on this story. A test that needs it is not writable at this
   story's own boundary.
2. This crate already has the precedent and the argument:
   `crates/happenstance-core/tests/frozen_signatures.rs:4-8` — "A consumer
   written against `MemoryEventStore` proves nothing about the port… the two only
   look alike while there is one implementation."
3. It keeps the test file free of the word `memory` entirely, which makes D6's
   independence visible by inspection rather than only by a powerset step nobody
   runs locally.

The test file follows `frozen_signatures.rs`'s shape: file-level
`#![cfg(feature = "conformance")]`, generic bounds, and a compile that is most of
the proof. `memory-projection-store` may add a second instantiation of the same
helper against the oracle; that is its story, not this one's.

### D8 — The persona-journey slice, and the one thing a human observes.

`_design.md` records **no user-facing surface** (`surfaces: []`, approved
2026-08-12); this story renders none. The "user" here is the adapter author, and
the two non-visual surfaces the design names are a **type surface** and a **text
surface** (`_design.md:24-38`). This story touches only the first. What the
adapter author gains is precise: after this PR they can write `impl
ProjectionProbe for TheirStore` against a dependency they already have, with one
feature flag and no new edge in their dependency graph — and until this PR they
cannot make their store observable to any projection rule at all.

### D9 — The named reshape trigger. Report it; do not absorb it.

Architecture brief Note 10 item 1 (`_decomposition.md:713-715`): if a projection
rule is found that can observe the read model **without** the probe, PS-11 is
over-built and this seam should shrink. That is a finding worth an ADR paragraph,
**not a quiet deletion**. If the implementer meets it, it is reported at the
slice boundary rather than acted on here.

## Integration contract

| Field | Value |
| ----- | ----- |
| **Archetype** | `foundation` — real in-tree substrate consumed by the capability slices of this project; never a double, never a `todo!()`. |
| **Slice / milestone** | `projection-port-and-probe`. Slice-mates, implemented in one context and mounted as one integrated surface: `owned-batch-port-shape` (lands first — supplies `type Batch;`), **this story**, `memory-projection-store` (lands third — consumes the probe). |
| **Mount point** | `crates/happenstance-core/src/lib.rs:98-124` — the export block — paired with `crates/happenstance-core/Cargo.toml` `[features]`. A library has no render tree; the architecture brief fixes the composition root as exactly these two places, "the two places a new item is either reachable or invisible" (`_decomposition.md:340-346`). An item mounted at one and not the other is an item no adapter can name. A third, weaker mount is the crate doc's `# Feature flags` list (`lib.rs:75-82`): an item reachable but undocumented is one nobody finds. |
| **Wires into** | `crates/happenstance-core/src/projection.rs` — `ProjectionStore` (supertrait), `Self::Batch` and `Self::Error` (associated types, post-`owned-batch-port-shape`), `ProjectionId`. `crates/happenstance-core/src/event.rs` — `SequencePosition`, via `commit` in the round trip. `trait_variant`'s blanket impl (`.kb/decisions/0001-async-port-flavours.md:54`) is what makes the bare supertrait bound serve `Send` adapters too. No new dependency, in the manifest or in the feature. |
| **Renders surfaces** | **none.** `_design.md` declares `surfaces: []` and the perceptual review is a declared skip (`design.capture` absent from `.redkiln/config.yaml`, per CLAUDE.md). The design's `## Items` / `## Signatures` blocks are `N/A — no user-facing surface`, so the binding signature source for this story is `spec/SPECIFICATION.md:4998-5031`, per D2. |
| **Conformance rule(s)** | **None added here, deliberately** — and the story is still adapter-observable, because the probe is *the mechanism every downstream projection rule runs through*: PS-11's own Rule line reads "`commit_is_atomic_with_the_read_model` and every rule downstream of it; the probe is the mechanism, not a rule of its own" (`spec/SPECIFICATION.md:4982-4983`). The rules that consume it arrive at `projection-suite-entry-point` and after. This story's own falsifier is the round-trip test (D7), and its real mutant — the testkit-placement error of D1 — is `documented-extension-surface` (HS-S0015). |
| **Clause(s)** | Implements **PS-11** (`spec/SPECIFICATION.md:4977-5031`) and declares the gate **PS-12** needs (`:5052-5074`). Both are `[PROVISIONAL]` (`:4980`, `:5055`). **No `[FROZEN]` clause is amended, and `spec/SPECIFICATION.md` is not edited by this story** — maturity markers and rule citations for PS-1 – PS-37 belong to `unstable-projection-gate-and-clause-disposition`. |
| **Advances DoD scenario** | Initiative **DoD 7** — "The projection suite discriminates… a deliberately wrong implementation that writes a checkpoint without its read model *fails* it, by name" (`initiative.md:377-380`). This story does not move DoD 7 to green; it makes it *reachable*, because without a write/read-back seam no rule can tell `CheckpointOnlyStore` from a correct store. Also a precondition for **DoD 13** (`cargo xtask ci` green on the assembled whole), which this story must not regress. |

## PR boundary

**In this PR**

- `ProjectionProbe` in `crates/happenstance-core/src/projection.rs`, gated
  `#[cfg(feature = "conformance")]` at item level (D4), with all five members and
  their rustdoc — including the placement argument of D1 and `# Errors` on
  `probe_read`.
- The `conformance` feature in `crates/happenstance-core/Cargo.toml`
  `[features]`: no `dep:` entries, no implied `memory`, no implied `std`.
- The re-export in `crates/happenstance-core/src/lib.rs`'s export block under the
  `#[cfg]` + `#[cfg_attr(docsrs, doc(cfg(…)))]` pair, and the `conformance` row in
  the crate doc's `# Feature flags` list.
- One integration test file, `crates/happenstance-core/tests/projection_probe_round_trip.rs`,
  file-level `#![cfg(feature = "conformance")]`, generic over `P: ProjectionProbe`,
  with its own minimal store (D7).
- This story's own backlog folder (ledger, report).

**Explicitly not in this PR**

- `type Batch;` and §4.0's `Checkpoint` / `Authority` / `CommitError` /
  `ResetError` — `owned-batch-port-shape` (the dependency). This story consumes
  that shape; if it is not in the tree, stop and say so rather than landing a
  second version of it.
- `MemoryProjectionStore` and its `ProjectionProbe` impl —
  `memory-projection-store` (slice-mate).
- Any conformance rule, the `for_each_projection_store_rule!` enumeration,
  `ProjectionFixture`, `projection_store_conformance!`, and every harness file —
  `projection-suite-entry-point`.
- Any mutant or registry entry, including `CheckpointOnlyStore` —
  `projection-mutant-registry`.
- Any edit to `spec/SPECIFICATION.md`, any maturity marker, and the
  `unstable-projection` gate — `unstable-projection-gate-and-clause-disposition`.
- Any ADR. ADR-0017/0018/0019 land in slice 1 and are a **precondition** of this
  slice (project AC-008); an ADR written as a side effect of this story is a
  process violation, not a bonus.
- Any adapter skeleton edit. The lifetime-removal restatement is
  `owned-batch-port-shape`'s reviewed diff (project AC-013).

**Merge DoD one-liner** — `cargo xtask ci` is green on the workspace, including
both feature powersets and the `--no-default-features` doc build of
`happenstance-core`, and `happenstance_core::ProjectionProbe` is nameable from a
crate that enables `conformance` and nothing else.

```
crates/happenstance-core/src/projection.rs
crates/happenstance-core/src/lib.rs
crates/happenstance-core/Cargo.toml
crates/happenstance-core/tests/projection_probe_round_trip.rs
.bklg/from-contract-to-published-library/projection-store-freeze/projection-probe-conformance-feature/**
```

The implementer **may** additionally touch the composition-root files named in
the Integration contract to mount this slice — that is `lib.rs` and `Cargo.toml`,
both already in the list above, and it is not scope drift. Widening the block for
anything else is a decision to be made here, in the spec, or a reason to split
the story.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The trait, verbatim** | `pub trait ProjectionProbe: ProjectionStore` with `const READS_THROUGH_BATCH: bool`, `fn probe_write(&self, batch: &mut Self::Batch, key: &str, value: u64)`, `fn probe_delete_all(&self, batch: &mut Self::Batch)`, `async fn probe_read(&self, key: &str) -> Result<Option<u64>, Self::Error>`, `fn probe_read_through(&self, batch: &Self::Batch, key: &str) -> Option<u64>`. Key `&str`, value `u64`; three members sync and infallible, one `async` and fallible, one `const`. Nothing is added, renamed or re-typed. | `spec/SPECIFICATION.md:4998-5031`; `_decomposition.md:299-302` (AC-A02) |
| **Supertrait, not a free-standing trait** | The bound is `: ProjectionStore` (bare flavour), so `Self::Batch` and `Self::Error` come from the port. `Send` adapters are served through `trait_variant`'s blanket impl; **no `SendProjectionProbe` is derived or hand-written.** | `spec/SPECIFICATION.md:5000-5003`; `.kb/decisions/0001-async-port-flavours.md:54`; `references/adr/0008-one-derivation-for-both-ports.md:103` |
| **`&mut Self::Batch` is only spellable after the dependency lands** | `probe_write`/`probe_delete_all` take `&mut Self::Batch` with no lifetime. Against today's `type Batch<'a> where Self: 'a` the signature does not exist; `owned-batch-port-shape` removes the GAT. If `projection.rs` still declares `Batch<'a>`, the dependency has not landed — halt, do not invent a lifetime. | `crates/happenstance-core/src/projection.rs:97-99`; `_storymap.md:56,87,108` |
| **Feature declaration** | `conformance = []` in `[features]` — no `dep:` entry, no `std`, no `memory`. Documented in the manifest comment beside `memory` and `serde` in the house style those two already use. | `crates/happenstance-core/Cargo.toml:34-52`; `_decomposition.md:349` |
| **Feature independence** | `conformance` compiles alone, with `--no-default-features` (`no_std`), and in every combination with `std`/`memory`/`serde`, on host **and** `wasm32-unknown-unknown`. The trait body names only `core`/`alloc` and the port's own types. | `xtask/src/main.rs:546-556` (workspace powerset), `:564-593` (wasm32 powerset); `_decomposition.md:483-487` |
| **Export-block mount** | `#[cfg(feature = "conformance")] #[cfg_attr(docsrs, doc(cfg(feature = "conformance")))] pub use projection::ProjectionProbe;` in the export block, matching the `memory` pair exactly. `happenstance_core::ProjectionProbe` resolves; `projection::ProjectionProbe` also resolves, since `projection` is a `pub mod`. | `crates/happenstance-core/src/lib.rs:98-124`, pattern at `:101-103,120-122`; `_decomposition.md:348` |
| **Feature-table mount** | A `conformance` bullet in the crate doc's `# Feature flags` list, stating what it is for (adapter authors invoking the conformance suite) and that it is off by default and implies nothing. | `crates/happenstance-core/src/lib.rs:75-82` |
| **Docs gating is `docsrs`-aware** | `#![cfg_attr(docsrs, feature(doc_cfg))]` is already set and `[package.metadata.docs.rs] all-features = true`, so the badge renders on docs.rs and in the gate's nightly `--cfg docsrs` rustdoc build. | `crates/happenstance-core/src/lib.rs:85`; `crates/happenstance-core/Cargo.toml:54-56` |
| **No inbound intra-doc link** | No ungated doc comment links to `ProjectionProbe` or its members; the name is spelled plainly where mentioned, with the reason stated in a comment, as the crate already does for `MemoryEventStore`. Outbound links from the gated item are permitted. | `crates/happenstance-core/src/lib.rs:71-73`; `crates/happenstance-testkit/src/lib.rs:112-132`; `xtask/src/main.rs:494-514` |
| **Round trip through the trait alone** | `async fn round_trip<P: ProjectionProbe>(store: &P)`: `begin` → `probe_write(&mut batch, "k", 7)` → `commit(batch, &id, position)` → `probe_read("k") == Some(7)`. No inherent method of the concrete store is called anywhere in the assertion path; the concrete store appears only at the instantiation site. | `_decomposition.md:779` (Testing brief AC-009); precedent and rationale at `crates/happenstance-core/tests/frozen_signatures.rs:1-15,4-8` |
| **All five members are reachable generically** | The same generic helper also exercises `probe_read_through` (under `P::READS_THROUGH_BATCH`) and `probe_delete_all`, so no member is dead on arrival and `reset-rules` two slices later inherits a seam that has been compiled against. | `spec/SPECIFICATION.md:5027`; `_decomposition.md:476-481` |
| **Test-local store, not the oracle** | The instantiating store is defined inside the test file: a `HashMap`-backed read model plus a batch of pending writes, declaring `READS_THROUGH_BATCH = true`. It is an instrument, not a shipped item; it is not a mutant and is not registered anywhere. | D7 above; `crates/happenstance-core/tests/frozen_signatures.rs:4-8` |
| **Rustdoc obligations** | Every public item and member carries a doc comment (`missing_docs` = warn, CI runs `-D warnings`); `probe_read` carries `# Errors` naming the *conditions*, not the error type (`missing_errors_doc` = warn). `probe_read_through`'s doc reproduces the specification's `unimplemented!()` guidance verbatim — the spelling matters, because `clippy::todo` is `deny` workspace-wide and `unimplemented` is not linted. | `Cargo.toml` `[workspace.lints]`; `standards/rust/70-rustdoc-obligations.md` |
| **Placement argument is written down** | The probe's rustdoc restates D1's coherence argument and names its own falsifier — `documented-extension-surface` (HS-S0015) — so a later reader does not "simplify" the trait into the testkit on grounds of diff size. | `spec/SPECIFICATION.md:5015-5031`; `_decomposition.md:689-693`; `discover.md:79-93` |
| **One flavour name per module** | Any module in this PR imports `ProjectionStore` **or** `SendProjectionStore`, never both; with both in scope `P::Batch` and the method calls are ambiguous. | `CLAUDE.md` binding constraint 4 |
| **Nothing regresses** | The event-store suite, the three existing harnesses and all four `wasm32` steps are untouched and green. `cargo xtask spec-trace` is unaffected: no clause text, marker or rule citation changes. | `xtask/src/main.rs` (`MANDATORY` step list); `spec/SPECIFICATION.md:4980,5055` |

## Data and migrations

**N/A — no schema, no stored data, no wire format.** This story adds a trait
declaration and a Cargo feature to a `no_std`-capable contract crate. There is no
database, no serialised representation, and no persisted artifact to migrate.
ADR-0003's constraint is untouched: `conformance` pulls in no dependency at all,
least of all `serde`, and the crate's default features are unchanged
(`crates/happenstance-core/Cargo.toml:35`).

The one migration-shaped fact is worth naming so it is not mistaken for one. The
crate's **feature surface** changes — three features become four — and for a
published crate that would be an additive, semver-minor change requiring no
action from a consumer, since `conformance` is off by default and implies
nothing. `happenstance-core` is **not yet published**: making it publishable is
what this whole initiative is for, so the addition lands before any consumer
exists to migrate. The obligation this creates is downstream, not here — the
feature must appear in the published crate's rendered feature documentation,
which is initiative **DoD 10**'s bar and `publication-and-positioning`'s to meet.

## Acceptance criteria

The persona throughout is **Persona 2, the adapter author**
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:114-181`),
whose goal is "an executable definition of *correct* they can run against their
own storage system, rather than a prose specification they have to interpret"
(`:127-130`), and whose journey today stalls at step 2 — "the compiler accepts a
`todo!()` body as readily as a real one, so passing the type checker confirms
nothing" (`:157-160`). This story is the step that makes their read model
*observable* at all; without it every projection rule downstream can only inspect
a checkpoint, which is the exact shape of the failure the persona fears.

Where a criterion names a test that does not exist yet, that test is this story's
to write.

| id | criterion | verification |
| -- | --------- | ------------ |
| AC-001 | **GIVEN** an adapter author with a working `ProjectionStore` impl who needs a bar that tells them when they are done, **WHEN** they enable `conformance` and write `impl ProjectionProbe for TheirStore`, **THEN** the trait they must satisfy is exactly the five members the specification publishes — `const READS_THROUGH_BATCH: bool`, `probe_write(&self, &mut Self::Batch, &str, u64)`, `probe_delete_all(&self, &mut Self::Batch)`, `async fn probe_read(&self, &str) -> Result<Option<u64>, Self::Error>`, `probe_read_through(&self, &Self::Batch, &str) -> Option<u64>` — declared as `pub trait ProjectionProbe: ProjectionStore` in the **bare** flavour with no `SendProjectionProbe` anywhere, so `Self::Batch` and `Self::Error` resolve through the port they already implement and a `Send` adapter is served by `trait_variant`'s blanket impl rather than by a second trait to implement. Nothing is added, renamed or re-typed relative to `spec/SPECIFICATION.md:4998-5031`. | **Unit + Static.** `crates/happenstance-core/tests/projection_probe_round_trip.rs::probe_shape_matches_the_specification` — coerces each member to an explicitly written `fn` pointer / async signature through a generic `P: ProjectionProbe`, in the manner of `crates/happenstance-core/tests/frozen_signatures.rs`, so a renamed or re-typed member is `error[E0308]`, not a silent pass. Static: `rg -n "SendProjectionProbe" crates/` returns nothing, and `cargo clippy --workspace --all-targets --all-features -D warnings` inside `cargo xtask ci`. |
| AC-002 | **GIVEN** the same author, who is already paying for one dependency edge on `happenstance-core` and will not accept a second, **WHEN** they add `happenstance-core = { version = "…", features = ["conformance"] }` to the `[dependencies]` (not `[dev-dependencies]`) of the crate their store lives in, **THEN** the feature exists in `happenstance-core`'s `[features]` table as `conformance = []` — pulling in **no** dependency, **not** implying `std`, and **not** implying `memory` — so enabling it costs them exactly one flag on a dependency they already have and no new edge in their graph. | **Static.** `cargo hack check --workspace --feature-powerset --no-dev-deps` inside `cargo xtask ci` (`xtask/src/main.rs:546-556`) compiles the `conformance`-only combination; a reviewed diff of `crates/happenstance-core/Cargo.toml` `[features]` shows the empty list literally (an implied feature is a text fact, and an implication that *happens* to compile is exactly what a powerset cannot distinguish from independence — see AC-005 for the arm that can). |
| AC-003 | **GIVEN** an author reading the crate's docs to find out whether the seam exists at all, **WHEN** they open `happenstance-core`'s docs.rs page or type `happenstance_core::Projection…`, **THEN** the item is *reachable and announced* at both mount points and neither alone: a `pub use projection::ProjectionProbe;` in the export block (`crates/happenstance-core/src/lib.rs:98-124`) carrying the `#[cfg(feature = "conformance")]` + `#[cfg_attr(docsrs, doc(cfg(feature = "conformance")))]` pair the `memory` items already carry at `:120-122` — so the rendered page shows the feature badge rather than an unexplained absence — **and** a `conformance` bullet in the crate doc's `# Feature flags` list (`:75-82`) stating what it is for and that it is off by default and implies nothing. An item mounted at one and not the other is an item nobody finds. | **Unit + Static.** Unit: `projection_probe_round_trip.rs` imports the trait by its **crate-root** path, `use happenstance_core::ProjectionProbe;` — an item left unexported is `error[E0432]`, so the export mount is proven by compilation rather than by inspection. Static: the nightly `--cfg docsrs` rustdoc build inside `cargo xtask ci` renders the badge, and a reviewed diff of `lib.rs:75-82` shows the feature-flags row (a doc-list entry is prose; no compiler can assert its presence). |
| AC-004 | **GIVEN** an author who wants proof that the seam is *generic* — usable by suite code that has never heard of their store — and not merely present on one type, **WHEN** the round-trip test runs, **THEN** a function bound only on `P: ProjectionProbe` performs `begin` → `probe_write(&mut batch, "k", 7)` → `commit(batch, &id, position)` → `probe_read("k") == Some(7)`, calling **no** inherent method of the concrete store anywhere on the assertion path, and the same helper also drives `probe_delete_all` and (under `P::READS_THROUGH_BATCH`) `probe_read_through`, so no member is dead on arrival. The instantiating store is defined in the test file itself, not `MemoryProjectionStore`, which does not exist yet at this story's boundary and whose use would prove only that the seam works for the oracle. | **Unit.** `crates/happenstance-core/tests/projection_probe_round_trip.rs::writes_are_visible_through_the_trait_alone` and `::all_five_members_are_reachable_generically`, run by `cargo test --workspace --all-features` inside `cargo xtask ci`. Rationale and precedent: `crates/happenstance-core/tests/frozen_signatures.rs:1-15`. |
| AC-005 | **GIVEN** the local-first / edge developer (Persona 3) whose target is `wasm32` and whose build has no `std`, **WHEN** they enable `conformance` in any combination of the crate's other features — including `conformance` **without** `memory`, and `conformance` under `--no-default-features`, which is `no_std` — **THEN** it compiles on the host **and** on `wasm32-unknown-unknown`, because the trait body names only `core`/`alloc` and the port's own types. The widened combination set is 16 per target, up from 8. | **Static.** `cargo hack check --workspace --feature-powerset --no-dev-deps` (`xtask/src/main.rs:546-556`) and the wasm32 powerset, which names `happenstance-core` explicitly (`xtask/src/main.rs:564-593`), both inside `cargo xtask ci`; plus the mandatory `--no-default-features` `wasm32` build of `happenstance-core` (`xtask/src/main.rs:205-215`). The named wrong implementation this rejects — a `conformance` surface that only compiles alongside `memory` — is green on default features and on `--all-features` and is caught nowhere else. |
| AC-006 | **GIVEN** an author (or the evaluator, Persona 4) who meets this trait *only* through its rendered documentation, **WHEN** they read `ProjectionProbe`'s page, **THEN** every member carries a doc comment, `probe_read` carries `# Errors` naming the *conditions* rather than the error type, `probe_read_through`'s doc reproduces the specification's `unimplemented!()` guidance verbatim (the spelling is enforceable: `clippy::todo` is `deny` workspace-wide, `unimplemented` is not linted), and the trait's own doc restates why it lives in the contract crate rather than the testkit — the orphan-rule argument of D1 — naming `documented-extension-surface` as the story that can falsify it. **AND** no ungated doc comment anywhere in the crate links to `ProjectionProbe` or its members: the name is spelled plainly where mentioned, as `lib.rs:71-73` already does for `MemoryEventStore`, because a link into a `cfg`-gated item is a **hard error** under the gate's `--no-default-features` doc build. | **Static.** `cargo doc -p happenstance-core --no-default-features` with `RUSTDOCFLAGS=-D warnings` (`xtask/src/main.rs:494-514`) fails on any inbound intra-doc link; `cargo clippy … -D warnings` enforces `missing_docs` and `missing_errors_doc` (`Cargo.toml` `[workspace.lints]`). A reviewed diff confirms the placement argument and the `unimplemented!()` spelling, per `standards/rust/70-rustdoc-obligations.md`. |
| AC-007 | **GIVEN** everyone already depending on this workspace's green gate, **WHEN** this PR is merged, **THEN** `cargo xtask ci` is green whole — the event-store conformance suite, the three existing harnesses and all four `wasm32` steps unchanged and passing — **and** the story's own boundary held: `spec/SPECIFICATION.md` is not edited (PS-11 and PS-12 stay `[PROVISIONAL]` at `:4980` and `:5055`, and `cargo xtask spec-trace` is therefore unaffected), no conformance rule, fixture, mutant or `MemoryProjectionStore` is added, no adapter skeleton is touched, and no ADR is written as a side effect of this story. If the reshape trigger of D9 is met — a projection rule that can observe the read model *without* the probe — it is **reported at the slice boundary**, not absorbed by shrinking the seam here. | **E2E + Static.** `cargo xtask ci` run whole from a clean tree, its output recorded in the implementation report; `cargo xtask spec-trace` green as one of its steps; `git diff --stat` against the merge base contains only the paths listed in the PR boundary block above. |

Every criterion above serves project **AC-009**'s *seam half*
(`.bklg/from-contract-to-published-library/projection-store-freeze/_storymap.md:87`,
"signature half, then seam half") — the signature half, `type Batch;` with no
lifetime, belongs to `owned-batch-port-shape` and is consumed here, not
re-asserted.

## Interaction quality

This story renders **no user-facing surface**: the signed-off
`_design.md` declares `surfaces: []` and answers `N/A — no user-facing surface`
to every one of its `## Items`, `## Signatures`, `## Shape decision`,
`## Anti-patterns` and `## The doctest` blocks
(`.bklg/from-contract-to-published-library/projection-store-freeze/_design.md:52-96`).
That is an approved determination, not an omission: the sign-off records that
"what was approved is the no-surface determination itself" (`:100-105`), and the
perceptual review is a declared skip because `design.capture` is absent from
`.redkiln/config.yaml`.

The design does not, however, leave the quality bar empty. It names the two
**non-visual** surfaces a human actually meets — a *type surface* and a *text
surface* (`_design.md:24-38`) — and this story touches the first. The invariants
below are those two families read in that medium. **Every one is carried by a row
in the acceptance-criteria table above**; nothing here is a free-floating bullet,
because a bullet in this section would never be extracted into the ledger and so
would never be gated.

**STATE family** — read as: what changes for someone who already has this crate.

| Invariant | Read in this medium as | Carried by | Verified by |
| --------- | ---------------------- | ---------- | ----------- |
| **In place, not a context jump** | The trait lands *inside* `projection.rs` beside the port it is a supertrait of, gated at item level; no new module, no second name for intra-doc resolution to fail on (D4). | AC-001, AC-003 | compilation of the crate-root import path; reviewed diff |
| **Non-occlusion** | Nothing already reachable stops being reachable. Default features are unchanged (`std`, `memory`); a consumer who never enables `conformance` sees a byte-identical public surface, and the crate still builds `no_std` under `--no-default-features`. | AC-005, AC-007 | both feature powersets; the whole `cargo xtask ci` |
| **Preserved focus / continuity** | The existing suite, harnesses and `wasm32` steps keep passing untouched — the author's current green run does not become a red one because a seam was added for someone else. | AC-007 | `cargo xtask ci` whole |
| **Reversibility** | The feature is off by default and implies nothing, so enabling it is a one-line change a consumer can undo with a one-line change; and because nothing is published yet, the addition is additive-and-free rather than a promise. | AC-002, AC-005 | reviewed `[features]` diff; powerset |
| **Reachability** (the keyboard-reachability analogue) | Reachable by the name a caller would guess, `happenstance_core::ProjectionProbe`, from the crate root — not only via `projection::`. | AC-003 | `use happenstance_core::ProjectionProbe;` in the test compiles |

**COMPOSITION family** — the design records no composition to bind to, so the
binding source in this medium is the one the design itself points at when it
substitutes a compiled artefact for a capture-based review (`_design.md:44-50`):
the crate's own documentation conventions, `standards/rust/70-rustdoc-obligations.md`,
and the presentation the `memory` feature already receives.

| Invariant | Read in this medium as | Carried by | Verified by |
| --------- | ---------------------- | ---------- | ----------- |
| **Presentation exists at all** | The item is not merely `pub`. It carries the same *composed* presentation `memory`'s exports carry: the `#[cfg]` + `#[cfg_attr(docsrs, doc(cfg(…)))]` pair so docs.rs renders a feature badge, a doc comment on the trait and on every member, and `# Errors` on the fallible one. A `pub use` with no attributes and no docs is this medium's bare markup — it compiles and satisfies every structural assertion, and tells a reader nothing. | AC-003, AC-006 | nightly `--cfg docsrs` rustdoc build; `-D warnings` over `missing_docs`/`missing_errors_doc` |
| **Placement** | Item-level gate inside `projection.rs`; re-export in the export block at `lib.rs:98-124`; documentation row in the `# Feature flags` list at `lib.rs:75-82`. Exactly the two mount points the architecture brief fixes, plus the weaker third. | AC-003 | compilation + reviewed diff |
| **Transience** | Persistent-chrome analogue: the `# Feature flags` list is always visible on the crate's front page, so the feature is discoverable *before* an author needs it. Revealed-on-demand: the trait's own page, reached from the export. Never opened-on-demand-only — an item whose sole announcement is its own rustdoc page is one nobody knows to look for. | AC-003 | reviewed diff of `lib.rs:75-82`; docs build |
| **Density budget (real numbers)** | 5 members + 1 associated const on one trait; **0** new dependencies, in the manifest and in the feature; **3 → 4** features on the crate; **8 → 16** powerset combinations per target, on 2 targets; **1** new test file; **0** new modules; **0** new crate-graph edges for a consuming adapter. Anything above these numbers is scope this spec did not authorise. | AC-001, AC-002, AC-005 | powerset step; `git diff --stat`; reviewed `[features]` diff |
| **Hierarchy** | The trait reads as *part of the port* — supertrait bound on `ProjectionStore`, associated types inherited, gated beside it — not as a satellite testing utility. That subordination is the whole of D1's placement argument and is stated in the rustdoc so a later reader cannot invert it. | AC-001, AC-006 | signature test; reviewed rustdoc |
| **Named anti-patterns** | `_design.md` records none (it declares no surface), so the anti-patterns this story is held to are the two the briefs name explicitly, both already ACs: (a) the trait "simplified" into `happenstance-testkit`, invisible to every check inside this workspace and fatal to an outside author's `tests/` directory; (b) a `conformance` feature that silently only compiles alongside `memory`. | AC-006 (the rustdoc that forbids (a) being re-argued), AC-005 (which rejects (b)) | `--no-default-features` doc build; both powersets |

## Error conditions

| id | condition | required behaviour |
| -- | --------- | ------------------ |
| **EC-001** | `crates/happenstance-core/src/projection.rs` still declares `type Batch<'a> where Self: 'a` — the dependency `owned-batch-port-shape` has not landed. | **Halt and report.** `probe_write(&self, batch: &mut Self::Batch, …)` is unspellable against a GAT. Do **not** invent a lifetime parameter, do **not** land a second version of §4.0's trait. The slice's merge order is `owned-batch-port-shape` → this story → `memory-projection-store` (`_storymap.md` "Merge order", item 2). |
| **EC-002** | `cargo doc -p happenstance-core --no-default-features` fails with `error: unresolved link to ProjectionProbe`. | An ungated doc comment linked into the gated item. Fix by spelling the name plainly and stating why in a comment, as `crates/happenstance-core/src/lib.rs:71-73` and `crates/happenstance-testkit/src/lib.rs:112-132` already do. Do **not** fix it by ungating the item or by adding `#[doc(hidden)]`. |
| **EC-003** | A powerset combination fails — typically `--features conformance` alone, or `conformance` without `std`. | The trait body reached for `std`, or the feature implies something it must not. Fix the trait body (`core`/`alloc` and the port's own types only) or the `[features]` entry; do **not** fix it by adding `conformance = ["std"]` or `conformance = ["memory"]`, which is precisely the wrong implementation AC-005 exists to reject. |
| **EC-004** | `error[E0275]` / an overlapping-impl error appears when adding a `Send` flavour of the probe. | Expected, and the design already accounts for it: `trait_variant`'s blanket impl means `SendProjectionStore` implies `ProjectionStore`, so `SendProjectionProbe` collides for the reason ADR-0008 records (`references/adr/0008-one-derivation-for-both-ports.md:103`). Remove the second trait; one probe serves both flavours. |
| **EC-005** | Method calls on `P::Batch` become ambiguous in the test or the trait's module. | Both `ProjectionStore` and `SendProjectionStore` are in scope. Import exactly one per module (CLAUDE.md binding constraint 4). |
| **EC-006** | `cargo clippy` denies `clippy::todo` in a doc example or a declining implementation. | The doc guidance is `unimplemented!()`, verbatim from `spec/SPECIFICATION.md:5030`. `todo!()` is denied workspace-wide; the spelling is load-bearing, not stylistic. |
| **EC-007** | The implementer finds a projection rule that can observe the read model **without** the probe (Architecture brief Note 10 item 1, `_decomposition.md:713-715`). | **Report at the slice boundary; do not act on it here.** PS-11 would be over-built and the seam should shrink — that is an ADR paragraph, not a quiet deletion inside a story whose scope forbids writing ADRs. |
| **EC-008** | The round-trip test needs a store and `MemoryProjectionStore` is not in the tree. | Expected. Define the minimal store inside the test file (D7). A test that waits on `memory-projection-store` inverts the slice's merge order and makes this story's own falsifier untestable at its own boundary. |

## Non-functional

| id | requirement | how it is held |
| -- | ----------- | -------------- |
| **NF-001** | **Zero dependency cost.** `conformance` adds no entry to `[dependencies]`, no `dep:` in the feature, and no new edge in a consuming adapter's graph. This is the entire economic argument for the placement (D1); if it acquires a dependency, the argument is void and the decision must be re-opened by ADR. | reviewed `Cargo.toml` diff; `cargo deny` inside `cargo xtask ci` |
| **NF-002** | **`no_std` cleanliness.** The trait compiles under `--no-default-features`. Only `core`/`alloc` and the port's own types are named. | `--no-default-features` `wasm32` build (`xtask/src/main.rs:205-215`) and the powersets |
| **NF-003** | **Gate cost is bounded and accepted.** Both powersets double for `happenstance-core` (8 → 16 combinations per target). This is a known, named cost of the story, not a regression to investigate; it is the price of the only automated check that the feature is independently coherent (`_decomposition.md:483-487`). | `cargo xtask ci` wall time recorded in the implementation report |
| **NF-004** | **Additive public surface.** The change is semver-minor by construction: a new trait behind an off-by-default feature that implies nothing. Nothing existing is renamed, re-typed or removed. | `standards/rust/40-public-surface-and-evolution.md`; reviewed diff |
| **NF-005** | **MSRV unchanged.** No feature of the language newer than 1.97.1 is used; the trait is an ordinary trait with one `async fn` (RPITIT), already used throughout the crate. | CI's `msrv` job (CLAUDE.md, "Commands") |
| **NF-006** | **The default build is byte-for-byte unaffected.** A consumer on default features sees no new item, no new dependency, and no changed signature. | powerset; `cargo xtask ci` |

## Implementation notes (non-prescriptive)

These are aids, not instructions. Where one disagrees with the Context pack or
`spec/SPECIFICATION.md`, those win.

**A plausible order.** (1) Confirm `type Batch;` has no lifetime in
`projection.rs` — if it does, EC-001. (2) Add `conformance = []` to
`[features]` with a comment in the house style the `memory` and `serde` entries
already use. (3) Write the trait at the bottom of `projection.rs` under
`#[cfg(feature = "conformance")]`, members and rustdoc together — the rustdoc is
not a follow-up pass, because D1's argument is the part most likely to be lost.
(4) Mount: the `pub use` with both attributes, then the `# Feature flags` row.
(5) Write the test. (6) `cargo hack check -p happenstance-core --feature-powerset
--no-dev-deps` locally before the whole gate — it is the step most likely to be
the one that fails, and it is cheap to run alone.

**On the test's minimal store.** A `RefCell<BTreeMap<String, u64>>` read model
plus a batch holding pending writes and the checkpoint is enough; declare
`READS_THROUGH_BATCH = true` so `probe_read_through` is exercised rather than
merely declared. `BTreeMap` over `HashMap` is worth a thought if the test file is
ever built without `std` — but the test is an integration test on the host and
`std` is available, so either is fine; note it only so the choice is deliberate.
The store is an instrument: it is not a mutant, is not registered anywhere, and
does not belong in `fixtures.rs`.

**On `probe_read` being `async` while the other three are not.** That is the
specification's shape and it is not arbitrary: the sync members mutate a batch the
caller owns and cannot fail, while a read of the committed read model is real I/O
against the store. Taking it as given avoids re-deriving ADR-0017's inputs
(`_decomposition.md:374-390`).

**On what "generic" means for the assertion path.** The test's helper takes
`&P` and `P::Batch` and nothing else. If the concrete store's name appears
anywhere except the instantiation line, the helper is not proving what AC-004
claims.

**Where to write D1's argument.** On the trait, not the module — a module doc
under an item-level gate is easy to miss, and the falsifier
(`documented-extension-surface`, HS-S0015) reads the trait's own page.

## Tests and CI (merge gate)

Tier vocabulary is the project testing brief's
(`.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md:756-767`):
**Static** reads source/config without executing the code under test; **Unit** is
`cargo test` running a function in-process; **Integration** is a conformance rule
actually driving a `ProjectionStore` impl; **E2E** is `cargo xtask ci` run whole.
This story has **no Integration row on purpose** — the first rule that drives a
store arrives at `projection-suite-entry-point`, and claiming an Integration tier
here would be claiming a rule this story does not write.

| tier | command / path | proves |
| ---- | -------------- | ------ |
| Static | `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings` (via `cargo xtask ci`) | `missing_docs`, `missing_errors_doc` and `clippy::todo` are enforced, so AC-006's rustdoc obligations are compiler-checked rather than review-checked (`Cargo.toml` `[workspace.lints]`). |
| Static | `cargo hack check --workspace --feature-powerset --no-dev-deps` (`xtask/src/main.rs:546-556`) | AC-002, AC-005 — `conformance` compiles alone, without `memory`, and in all 16 combinations on the host. |
| Static | `cargo hack check -p happenstance-core -p happenstance-neon -p happenstance-testkit --target wasm32-unknown-unknown --feature-powerset --no-dev-deps` (`xtask/src/main.rs:564-593`) | AC-005 on the target that ADR-0001 exists for. |
| Static | `cargo check -p happenstance-core --no-default-features --target wasm32-unknown-unknown` (`xtask/src/main.rs:205-215`, mandatory) | NF-002 — the `no_std` arm, which is the combination a powerset can skip if `cargo-hack` is absent. |
| Static | `cargo doc -p happenstance-core --no-default-features` with `RUSTDOCFLAGS=-D warnings` (`xtask/src/main.rs:494-514`) | AC-006 — no inbound intra-doc link into the gated item. This is where the D5 hazard fires, and it fires as a hard error. |
| Static | nightly `cargo doc` with `--cfg docsrs` (mandatory-if-toolchain-present) | AC-003 — the `doc(cfg(feature = "conformance"))` badge renders. |
| Unit | `crates/happenstance-core/tests/projection_probe_round_trip.rs::probe_shape_matches_the_specification` | AC-001 — every member's signature, checked by coercion, so a rename or a re-type is a compile error. |
| Unit | `crates/happenstance-core/tests/projection_probe_round_trip.rs::writes_are_visible_through_the_trait_alone` | AC-004 — the round trip, through the trait alone. This is the story's own falsifier. |
| Unit | `crates/happenstance-core/tests/projection_probe_round_trip.rs::all_five_members_are_reachable_generically` | AC-004 — `probe_delete_all` and `probe_read_through` are exercised, not merely declared, so `reset-rules` and `read-through-and-rebuild-rules` inherit a seam that has been compiled against. |
| Unit | `cargo test --workspace --all-features` (via `cargo xtask ci`) | AC-007 — the event-store suite and the three existing harnesses are unchanged and green. |
| E2E | `cargo xtask ci` run whole on a clean tree, output recorded in the implementation report | AC-007, NF-003 — every step above in one run, plus `cargo xtask spec-trace`, `cargo deny`, and the `cargo package --list` assertion. |
| Story grain | `cargo xtask affected --base main` + `cargo xtask ci --fast` | The per-commit bar during implementation (CLAUDE.md "Commands"; `_storymap.md` "Merge order", closing paragraph). `--fast` omits `spec-trace` and the wasm32 conformance-harness check, which is acceptable *within* the story and not acceptable at the slice boundary. |

## Risks and coupling (PR-scoped)

| risk | why it bites here | containment |
| ---- | ----------------- | ----------- |
| **The dependency has not landed.** | `&mut Self::Batch` does not exist against a GAT; the temptation is to add a lifetime "just for now", which quietly re-introduces the borrowing shape the whole slice exists to remove. | EC-001: halt and report. The merge order is the containment (`_storymap.md`, "Merge order" item 2). |
| **The placement gets "simplified" into the testkit.** | Nothing in this workspace can fail the wrong version: every fixture here already depends on the testkit, so the orphan rule stays silent and `cargo xtask ci` stays green in every step (D1). The failure is *deferred* to an outside author, five slices away. | The argument lives in the trait's rustdoc, naming its own falsifier (AC-006). The falsifier is `documented-extension-surface` (HS-S0015). |
| **`conformance` silently coupled to `memory`.** | The round-trip test wants a store and the oracle is the obvious one; the coupling is green on default features and on `--all-features`, and fails only in a step people skip locally. | AC-004's test-local store keeps the word `memory` out of the test file entirely; AC-005's powerset is the automated backstop. Both, not either. |
| **Doc-build breakage from an inbound intra-doc link.** | This workspace has already paid for this once (`crates/happenstance-testkit/src/lib.rs:112-132`), and the gate treats it as a hard error, not a warning. | AC-006; the plain-spelling convention with the reason stated in a comment. |
| **Slice coupling — three stories in one context.** | `owned-batch-port-shape` → this story → `memory-projection-store` are implemented together. A change to §4.0's trait made while this story is in flight silently changes what `Self::Batch` means. | The dependency's shape is consumed, never re-declared here; if it moves, this story's test is the first thing that fails, which is the intended coupling. |
| **Scope creep into the first rule.** | Once the probe exists, writing `commit_is_atomic_with_the_read_model` is a twenty-line temptation — and it would need `ProjectionFixture`, the enumeration, and an emitter decision Note 4 explicitly leaves open. | The PR boundary block forbids it; those belong to `projection-suite-entry-point`. |
| **An ADR written as a side effect.** | D9's reshape trigger and the `unimplemented!()`/`todo!()` question both feel ADR-shaped mid-implementation. | Project AC-008 makes ADR-0017/18/19 a *precondition* of this slice, accepted in slice 1. A new ADR here is a process violation; record the gap and report it (EC-007). |

## Dependencies

**Blocks on** (must be in the tree before this story starts):

- `owned-batch-port-shape` — supplies §4.0's `type Batch;` with no lifetime, plus
  `Checkpoint` / `Authority` / `CommitError` / `ResetError` and the re-exports.
  Without it every probe signature taking `&mut Self::Batch` is unspellable
  (EC-001). Same slice, immediately prior in merge order
  (`_storymap.md`, "Merge order" item 2).

Transitively, through that story: `projection-decision-atoms` and
`projection-api-design-record` (slice 1) — project AC-008 requires ADR-0017/0018/0019
accepted **before** the port change lands, and the ordering is itself the check.

**Unlocks** (stories that cannot start until this lands):

- `memory-projection-store` — implements `ProjectionProbe` under `conformance`;
  slice-mate, next in merge order.
- `projection-suite-entry-point` — every rule it emits observes the read model
  through this seam.
- `read-through-and-rebuild-rules` — `batch_reads_reflect_pending_writes` is
  gated on `READS_THROUGH_BATCH`, declared here.
- `reset-rules` — `reset_clears_rows_and_checkpoint_together` is written through
  `probe_delete_all`, which is why dropping that member is not a simplification.
- `documented-extension-surface` — builds an outside author's fixture from the
  documented pair `projection_store_conformance!` + `ProjectionProbe`, and is the
  only story in this project that can falsify D1's placement.

## Anchors (progressive disclosure)

Open these when the row says to, not before. Everything load-bearing enough to
be read *first* is already in the Context pack.

| anchor | why it is load-bearing | when to open | serves |
| ------ | ---------------------- | ------------ | ------ |
| `spec/SPECIFICATION.md` (PS-11 at `:4977-5031`; PS-12 and CF-18 at `:5041-5074`) | The trait verbatim — the binding signature source, since `_design.md` declares no surface. Also carries the coherence argument in full and CF-18's reported-skip requirement. Where this spec and the specification disagree, the specification wins. | Before writing the trait, with the file open beside the editor — not from memory. | AC-001, AC-006 |
| `crates/happenstance-core/src/projection.rs` | The port this trait is a supertrait of, and the file it is gated inside. Confirms whether `owned-batch-port-shape` has landed (`type Batch;` vs `type Batch<'a>`), and shows the commit/rollback pair that is today's entire vocabulary. | First thing, before any edit — the EC-001 check. | AC-001 |
| `crates/happenstance-core/src/lib.rs` (`:75-82` feature flags; `:98-124` export block; `:71-73` the plain-spelling comment) | Both mount points and the intra-doc-link discipline, all three in one file. `:101-103` and `:120-122` are the `memory` attribute pair to copy exactly. | When mounting the item, and again when writing any doc comment that mentions it. | AC-003, AC-006 |
| `crates/happenstance-core/Cargo.toml` (`:34-52` features; `:54-56` docs.rs metadata) | The house style for a feature entry — every existing one carries a comment explaining what it costs and why it is or is not default — and the `all-features`/`docsrs` metadata that makes the badge render. | When adding `conformance = []`. | AC-002, AC-003 |
| `crates/happenstance-core/tests/frozen_signatures.rs` (`:1-15`, and the argument at `:4-8`) | The precedent for both halves of this story's test: signature assertions by coercion, and the recorded reason a consumer written against the reference implementation "proves nothing about the port". | Before writing `projection_probe_round_trip.rs` — copy its shape, not just its idea. | AC-001, AC-004 |
| `xtask/src/main.rs` (`:205-215` mandatory wasm build; `:494-514` the `--no-default-features` doc build; `:546-556` host powerset; `:564-593` wasm32 powerset) | The four gate steps this story widens or trips, with their exact flags — so a failure can be reproduced in isolation instead of by re-running the whole gate. | When a gate step fails, and before claiming AC-005 or AC-006. | AC-005, AC-006, AC-007 |
| `crates/happenstance-testkit/src/lib.rs:112-132` | The workspace's worked example of the feature-gated intra-doc-link hazard, with the reasoning written out where it was paid for. | When writing any doc comment that names `ProjectionProbe` from outside the gate. | AC-006 |
| `standards/rust/70-rustdoc-obligations.md` | The rustdoc rules with their mechanism attached — `# Errors` naming conditions, `missing_docs`, and what the gate does about each. CLAUDE.md deliberately does not repeat them. | Before writing the trait's rustdoc. | AC-006 |
| `standards/rust/51-features-and-no-std.md` | The house rules for adding a feature: additivity, what a feature may and may not imply, and the `no_std` obligations a new gate inherits. | When adding `conformance = []` and when EC-003 fires. | AC-002, AC-005 |
| `standards/rust/40-public-surface-and-evolution.md` | What counts as an additive change and what quietly is not — the check behind NF-004, on a crate whose whole initiative is about becoming publishable. | When reviewing the diff before the gate run. | AC-002, AC-007 |
| `.kb/decisions/0001-async-port-flavours.md` (`:54`, the blanket impl) | The Accepted decision that makes one bare-flavour probe serve `Send` adapters too — the reason no `SendProjectionProbe` is needed and the constraint that forbids `#[async_trait]`. | Before deciding anything about flavours; immediately if EC-004 fires. | AC-001 |
| `references/adr/0008-one-derivation-for-both-ports.md:103` | The long-form record with the `error[E0275]` transcript a second flavour produces. The atom states the decision; this states what the compiler actually said. | Only if a `Send` probe is attempted or EC-004 fires. | AC-001 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` (AC-A02 at `:299-302`; mount points at `:340-356`; Note 4's write seam at `:464-487`; Note 9/DT-8 at `:689-699`; Note 10 at `:713-715`; Testing brief AC-009 at `:779`) | The project's own settled decisions: placement, what "mounted" means for a library, the powerset consequence, the rustdoc hazard, and the named reshape trigger. | When tempted to re-decide placement, module structure, or the test's tier. | AC-001, AC-005, AC-007 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` (`:10-50` the two non-visual surfaces; `:52-96` the `N/A` blocks; `:98-105` the sign-off) | The signed-off determination that there is no user-facing surface, and the two non-visual surfaces that replace it. It is what the Interaction quality section binds to. | Before assuming any presentation obligation beyond the type surface. | AC-003, AC-006 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:114-181` | Persona 2's journey — why "it compiles" is not evidence, and what step of their journey this seam unblocks. The acceptance criteria's GIVEN clauses come from here. | When judging whether a criterion is really user-intent-shaped or just a capability restated. | AC-001, AC-002, AC-005 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/projection-probe-conformance-feature/discover.md` (`:44-52` the deferred questions; `:79-103` the named wrong implementations) | This story's own signal ledger, including the two wrong implementations the spec's ACs are built to reject. | If a criterion above seems arbitrary — this is where it came from. | AC-004, AC-005 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_storymap.md` (`:57` this row; `:87` the AC-009 split; "Merge order" item 2) | The slice's merge order and the signature-half/seam-half split that defines this story's boundary against its dependency. | When deciding whether something belongs here or in a slice-mate. | AC-007 |

## Clarifications resolved during spec

1. **The probe's key and value types** (discover question 3, deferred to spec):
   `&str` and `u64`, taken from `spec/SPECIFICATION.md:4998-5031` rather than
   re-derived. The specification wins on conflict, per `_decomposition.md:277-279`.
2. **Supertrait, not a free-standing trait** (discover question 4, deferred to
   spec): `ProjectionProbe: ProjectionStore`, so `Self::Batch` and `Self::Error`
   resolve through the port. The alternative — a standalone trait re-declaring
   both associated types — was rejected because it would let an adapter's probe
   disagree with its own store about what a batch is, which no compiler would
   catch.
3. **No `SendProjectionProbe`**, and this is not a gap in the two-flavour
   obligation: `trait_variant`'s blanket impl means a `Send` adapter already
   satisfies the bare supertrait bound (`.kb/decisions/0001-async-port-flavours.md:54`).
   A second flavour would collide, exactly as ADR-0008 records.
4. **Item-level `#[cfg]` inside `projection.rs`, not a `conformance` module.**
   Decided here (D4). The `memory` precedent is a gated module, but `memory` is a
   satellite implementation while the probe is part of the port's contract — and
   a second gated *module* name is a second intra-doc-link hazard.
5. **The round trip instantiates a test-local store, not `MemoryProjectionStore`.**
   Decided here (D7), against the testing brief's parenthetical "against
   `MemoryProjectionStore`" (`_decomposition.md:779`). The brief's tier and its
   substantive requirement — "through nothing but the trait, proving the seam is
   generic rather than merely present" — are met exactly; only the instantiating
   store differs, and it must, because `MemoryProjectionStore` lands in a story
   that **depends on this one**. A second instantiation against the oracle is
   `memory-projection-store`'s to add. This is a deliberate, recorded divergence,
   not an oversight.
6. **No conformance rule is written here, and the story is still
   adapter-observable.** PS-11's own Rule line says the probe "is the mechanism,
   not a rule of its own" (`spec/SPECIFICATION.md:4982-4983`). The story's
   falsifier is its round-trip test; the mutant that would catch the *placement*
   error is `documented-extension-surface`'s, five slices later, and that
   distance is why the argument is written into the rustdoc.
7. **AC count.** Seven criteria, exactly the ids the first pass declared
   (AC-001 – AC-007); none added, none dropped. AC-005 absorbs both powersets and
   the `no_std` arm as one criterion because they prove one property — feature
   independence — and splitting them would produce two ledger rows flipped by the
   same evidence.
8. **`unimplemented!()` versus `todo!()`** in `probe_read_through`'s doc: the
   specification's spelling is kept, and the reason is mechanical rather than
   stylistic — `clippy::todo` is `deny` workspace-wide and `unimplemented` is not
   linted, so a declining adapter that copies the doc compiles and one that
   reaches for `todo!()` does not (EC-006).
