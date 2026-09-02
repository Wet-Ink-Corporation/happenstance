---
item: HS-S0006
stage: spec
created: 2026-08-12T13:46:00.598Z
updated: 2026-08-12T13:46:00.598Z
template_sig: 87bbf1d0
rendered_sig: cd73d1c9
---

# Spec — MemoryProjectionStore as oracle, doctest target and cold-start fix

## Scope lock

| What | Path |
| --- | --- |
| Initiative | `.bklg/from-contract-to-published-library/initiative.md` — BR/AC/DoD spine; **DoD 7** is the one this story moves |
| Project | `.bklg/from-contract-to-published-library/projection-store-freeze/project.md` — AC-012 is this story's, verbatim |
| This spec | `.bklg/from-contract-to-published-library/projection-store-freeze/memory-projection-store/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` — Architecture Notes 1 (the seam map), 4 (the probe and the powerset widening), 9 (the rustdoc hazard, `begin` staying infallible); Testing brief rows **AC-012** and **AC-009**; UX brief **AC-U05**, **AC-U06** |
| Grounding | `.bklg/from-contract-to-published-library/projection-store-freeze/_grounding.md` §1 (what the port is today), §5 (RUNBOOK phase 6 read closely) |
| Signed-off design | `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` — records `surfaces: []`, **no user-facing surface**; its `## Items` / `## Signatures` blocks are N/A, so the signature authority for this story is `spec/SPECIFICATION.md` §4.0 and §4.11's probe block, per the intake gate's "the specification wins on conflict" |
| Roadmap pointer | `RUNBOOK.md:3848-3965` (phase 6 in full); `RUNBOOK.md:3898-3903` is this story's bullet |
| Story map row | `.bklg/from-contract-to-published-library/projection-store-freeze/_storymap.md`, milestone `projection-port-and-probe` |

## One-line PR slice

Ship `MemoryProjectionStore` behind the `memory` feature as the oracle, the
doctest target and the cold-start fix, implementing `ProjectionProbe` under
`conformance`, with the `error[E0195]` spelling trap documented where an
implementer meets it and the module named defensively so a `memory`-gated
intra-doc link cannot break `cargo doc`.

## Executive summary

Two slice-mates land the *shape*: `owned-batch-port-shape` (HS-S0004) puts
§4.0's trait into `crates/happenstance-core/src/projection.rs` with `type Batch;`
and the four new types, and `projection-probe-conformance-feature` (HS-S0005)
adds `ProjectionProbe` behind a new `conformance` feature. Neither lands a store.
Until one exists, the port has **zero** implementations that run: the five impls
in the tree are `todo!()`-bodied skeletons (`_grounding.md` §2), so nothing
executes `begin` → write → `commit` → read-back, and the suite the next slice
builds would have nothing to validate itself against.

This PR closes that. It adds one real, ~50-line implementation of the port
(`RUNBOOK.md:3898-3903`) doing for `ProjectionStore` exactly what
`MemoryEventStore` does for `EventStore` — the three reasons are already written
down at `crates/happenstance-core/src/memory.rs:16-34`: it is the oracle a
failing adapter is measured against, it makes the crate's examples runnable so
the docs cannot drift, and it lets application code be written before any real
adapter exists.

The delta beyond "port a known pattern": this store is the **first of AC-004's
two structurally unlike batch shapes** — the apply-on-write end, against which
`buffering-conformant-variant` (HS-S0011) later builds the replay-at-commit end —
so what it declares about read-through is load-bearing rather than incidental.
And it is the story that disposes of the `error[E0195]` cold start honestly:
PS-5's own *Rejects* clause claims the owned batch "removes `error[E0195]`
entirely" (`spec/SPECIFICATION.md:4869-4878`), and this is the first impl in the
workspace in a position to prove or refute that by compiling.

## Context pack

**1. The cold start is the problem, and it is a documented one.** An adapter
author implementing this port today fails with `error[E0195]: lifetime parameters
or bounds on method 'commit' do not match the trait declaration` unless they
spell the parameter `Self::Batch<'_>` literally — "nothing says so, and there is
nothing to copy" (`RUNBOOK.md:3898-3903`, confirmed independently at
`references/adapter-shapes.md:186-194`). That is the whole explanation for zero
adapters (`spec/SPECIFICATION.md:5554-5560`). The persona is P2, the adapter
author, whose stated want is "an executable definition of *correct* they can run
against their own storage system" (`_decomposition.md` UX brief); the thing this
story hands them is the *first* half of that — something to copy.

**2. This story does not re-decide a single signature.** HS-S0004 lands §4.0's
trait verbatim (`spec/SPECIFICATION.md:4632-4731`): `type Batch;` with no
lifetime, `fn begin(&self) -> Self::Batch` neither `async` nor fallible,
`checkpoint -> Checkpoint`, `commit(batch, id, position, Authority) ->
Result<(), CommitError<Self::Error>>`, `reset(batch, id) -> Result<(),
ResetError<Self::Error>>`, `rollback`. If implementing it wants a signature
changed, that is a finding for HS-S0004 and an ADR-0017 clause — **not** an edit
made here. In particular do not "improve" `begin` back to `async fn begin() ->
Result<…>` for symmetry with `EventStore`: the infallible spelling is
load-bearing for Neon's one-shot transport (`spec/SPECIFICATION.md:4884-4897`;
`_decomposition.md` Architecture Note 9).

**3. Apply-on-write is a decision, not a default — and it fixes
`READS_THROUGH_BATCH = true`.** This store is the apply-on-write end of the
batch-shape axis; `buffering-conformant-variant` (HS-S0011) is the
replay-at-commit end, built inside the testkit's own `tests/` as §4.11's CF-5
conformant variant (`spec/SPECIFICATION.md:5686-5691`; `_decomposition.md`
Architecture Note 3). Concretely: the batch is a materialised delta the store can
read back through, so `ProjectionProbe::READS_THROUGH_BATCH` is `true` here and
`probe_read_through` is real rather than `unimplemented!()`. If it were `false`
on both shapes, PS-12's rule
(`batch_reads_reflect_pending_writes`) would be skipped by everything in the
workspace and the pair would be vacuous. Declaring `false` here is therefore a
*finding* handed to `read-through-and-rebuild-rules` (HS-S0013), not a quiet
flip.

**4. Atomicity is the invariant the store exists to demonstrate, and it is
`[FROZEN]`.** The read-model write and the checkpoint write become durable
together or not at all (PS-1, `spec/SPECIFICATION.md:4733-4759`). For an
in-memory store that means: nothing a batch holds is visible through
`probe_read` or `checkpoint` until `commit` takes the batch, and `commit`
installs both under one lock acquisition. A `commit` that publishes rows first
and the checkpoint second is exactly `CheckpointOnlyStore`'s defect with the
timing changed, and the store whose job is to be the oracle must not model it.

**5. `commit` carries `Authority`, and the checkpoint is a three-variant enum.**
`Authority::Live` records `Checkpoint::Live { through }`, `Authority::Rebuilding`
records `Checkpoint::Rebuilding { through }`, an id never seen (or freshly
`reset`) reads `Checkpoint::NeverRun`. The enum exists rather than
`(Option<SequencePosition>, bool)` because the tuple can spell `(None, true)` —
"authoritative, never run — which means nothing" (`spec/SPECIFICATION.md:4643-4658`).
The position is the position *considered*, not the position applied, so an empty
batch still advances the checkpoint (PS-21/PS-22, `RUNBOOK.md:3908-3910`).

**6. The foreign-batch hazard survived the owned batch, and is discharged at run
time.** Dropping the lifetime does not make `b.commit(a.begin())`
unrepresentable — a lifetime names a region, not an instance, and only a
generative brand would reject it, which fights `async`
(`spec/SPECIFICATION.md:5099-5125`). So `begin` stamps an identity minted per
store instance, `commit` and `reset` compare it, and a mismatch is
`CommitError::ForeignBatch` / `ResetError::ForeignBatch`. With an owned batch the
stamp is a field and the check is an integer comparison. PS-15 stays
`[PROVISIONAL]`; this story implements the run-time discharge, it does not close
the clause.

**7. The error type is uninhabited, and that is an argument rather than a
shortcut.** `MemoryStoreError` is an empty enum whose doc says why: "it proves
the contract does not *require* a fallible read path, and it documents at the
type level that this store has no failure modes of its own"
(`crates/happenstance-core/src/memory.rs:283-291`). The projection store takes
the same shape with its **own** type — the existing one's `Display` names the
event store, and the two are re-exported separately — and the store loses
nothing the suite needs, because `CommitError` still carries `ForeignBatch` and
`CheckpointRegression`. Adapter-error paths
(`failed_commit_leaves_both_unchanged`) are supplied by hostile mutant stores in
the testkit, never by the oracle.

**8. A library's mount point is two places, and half a mount is invisible.** The
crate's export block (`crates/happenstance-core/src/lib.rs:98-124`) and the
feature table (`crates/happenstance-core/Cargo.toml` `[features]`). Copy the
`memory` pattern exactly: private module gated `#[cfg(feature = "memory")]` +
`#[cfg_attr(docsrs, doc(cfg(feature = "memory")))]` at `lib.rs:101-103`,
re-export under the same pair at `lib.rs:120-122`. The `ProjectionProbe` impl is
gated `#[cfg(all(feature = "memory", feature = "conformance"))]` — the two
features are independent and **`conformance` without `memory` must compile**
(`_decomposition.md` Architecture Note 4).

**9. The rustdoc hazard has already been paid for once in this workspace.** An
intra-doc link into a module absent on some documented configuration is a *hard*
rustdoc error, not a warning: `cargo doc --no-default-features` is a gate step,
and the crate root already spells `MemoryEventStore` plainly for exactly this
reason — "the name is deliberately not a link here" (`lib.rs:66-74`, D13). The
testkit's own record of paying for it is `crates/happenstance-testkit/src/lib.rs:112-132`.
So: no intra-doc link to `MemoryProjectionStore` or to its module from any page
that renders without `memory`, and none to the probe impl from any page that
renders without `conformance`. This is what "named defensively" means in the
one-line slice.

**10. Two doctests, doing two different jobs.** The port's own rustdoc carries a
**toy-store impl** — PS-34's stated rule is "a doctest on `ProjectionStore`
implementing the port for a toy store, which cannot rot because CI runs it"
(`spec/SPECIFICATION.md:5546-5553`) — and it must not depend on the `memory`
feature, so it defines its own two-line store rather than naming this one.
`MemoryProjectionStore`'s own rustdoc carries the **runnable walkthrough**
(`begin` → write → `commit` → read back the rows and the checkpoint), mirroring
`crates/happenstance-core/src/memory.rs:36-60`. Both run under `cargo test
--doc`; `_design.md` records no `## The doctest` block because it records no
surface, so these two are the doctest obligation for this story and
`standards/rust/62-doctests-and-harnesses.md` is the atom that governs them.

**11. The `E0195` disposition is a compile-time result, not a paragraph.** PS-5
claims the owned batch "removes `error[E0195]` entirely: with no lifetime on the
associated type there are no lifetime parameters on `commit` to mismatch, so
`async fn commit(&self, batch: MyBatch, …)` compiles"
(`spec/SPECIFICATION.md:4869-4878`), against `references/adapter-shapes.md:186-194`,
which recorded the opposite while the GAT was still on the port. This story is
where that is settled by compiling: write the impl with the **concrete** batch
type, not `Self::Batch<'_>`. If it compiles, the trap is retired and the port's
rustdoc says so once, at the item whose unusual shape bought it — naming the
alternative that lost, per `standards/rust/70-rustdoc-obligations.md` (RS-70-5)
and UX brief AC-U05. If it does not compile, that is a finding against HS-S0004's
shape and it stops this story. Either way **PS-34's maturity marker is not edited
here**: clause disposition is `unstable-projection-gate-and-clause-disposition`'s
(HS-S0016), and `cargo xtask spec-trace` is its gate.

**12. PS-36 is `[FROZEN]` and outlives the E0195 trap.** The `Send` flavour
transitively requires `Batch: Send`, `type Batch: Send;` cannot be written
because `trait_variant` copies associated-type bounds verbatim into the `!Send`
flavour and would break wasm32, so **documentation is the only available
mechanism** (`spec/SPECIFICATION.md:5601-5627`). Where this story writes a
`compile_fail` doctest for it, it is spelled **bare** — never
`compile_fail,E0308` — because rustdoc on 1.97.1 silently ignores a code the
diagnostic does not carry, making the stricter-looking spelling the weaker check
(same clause; `crates/happenstance-testkit/src/contract.rs:400-403`).

**13. Both flavours, one impl, and only one name in scope.** Implement
`SendProjectionStore`; the bare flavour comes free from
`#[trait_variant::make(SendProjectionStore: Send)]` at
`crates/happenstance-core/src/projection.rs:87`, which is how
`MemoryEventStore` is written (`memory.rs:293-294`). Generic code — including
this story's own tests — binds `ProjectionStore`, the weaker requirement, and
imports exactly one of the two names per module (CLAUDE.md binding constraint 4;
`standards/rust/20-two-flavour-ports.md`). Never `#[async_trait]` (ADR-0001,
`.kb/decisions/0001-async-port-flavours.md`).

**14. There is no conformance suite yet, and this story must not invent one.**
`projection_store_conformance!`, the `ProjectionFixture` trait and the first
rules are `projection-suite-entry-point`'s (HS-S0007). This story's proof is
therefore `happenstance-core`'s own tests plus its doctests — which is exactly
what the Testing brief's AC-012 and AC-009 rows specify (Unit + Static, with the
probe round-trip "gated by `conformance`" in the crate's own tests). Do not add
a fixture, do not add a rule, do not touch `happenstance-testkit`.

**15. What this story is explicitly not allowed to fix in passing.**
`ProjectionId::new` stays infallible — its inconsistency with `EventType`/`Tag`
is an open question owned elsewhere
(`.kb/open-questions/projection-id-is-unvalidated.md`;
`crates/happenstance-core/src/projection.rs:44-64`; Architecture brief AC-A09).
No `[FROZEN]` clause is line-edited (CLAUDE.md, "Changing a `[FROZEN]` clause
requires a new ADR, not an edit"). And no verdict is offered on whether the port
ships behind `unstable-projection` — that is HS-P0016's, on HS-S0015's evidence.

## Integration contract

- **Archetype**: `foundation` — real in-tree substrate, consumed inside this
  initiative by `projection-suite-entry-point` (HS-S0007), which runs its first
  two rules against this store, and by every rule story after it. No double, no
  `todo!()`, no fixme.
- **Slice / milestone**: `projection-port-and-probe`. Slice-mates:
  `owned-batch-port-shape` (HS-S0004) and `projection-probe-conformance-feature`
  (HS-S0005). The three are implemented in one context and mounted as one
  surface; this story lands **last** within the slice, because it is the first
  thing that compiles against both of the others
  (`_storymap.md`, Merge order 2).
- **Mount point**: `crates/happenstance-core/src/lib.rs` — the export block at
  `:98-124`, paired with `crates/happenstance-core/Cargo.toml`'s `[features]`
  table. Both, or the item is invisible: a library has no render tree, and these
  two places are where a new item is either reachable or not
  (`_decomposition.md` Architecture Note 1). An item mounted at one is the
  library equivalent of a constructed-but-unmounted component.
- **Wires into**:
  - `crates/happenstance-core/src/projection.rs` — `ProjectionStore` /
    `SendProjectionStore`, `ProjectionId`, and HS-S0004's `Checkpoint`,
    `Authority`, `CommitError`, `ResetError`.
  - `crates/happenstance-core/src/projection.rs` (`conformance`) —
    HS-S0005's `ProjectionProbe`, implemented here for the first time
    (`spec/SPECIFICATION.md:4998-5013`).
  - `crates/happenstance-core/src/event.rs` — `SequencePosition` (`NonZeroU64`,
    `FIRST`), the type both `commit` and `Checkpoint` carry.
  - `crates/happenstance-core/src/memory.rs` — the pattern to mirror, not to
    extend: `:16-34` (why a reference implementation exists), `:36-60` (the
    runnable walkthrough), `:283-294` (the uninhabited error and the
    `Send`-flavour impl).
  - `crates/happenstance-core/Cargo.toml` — `memory = ["std"]`, plus HS-S0005's
    `conformance`.
- **Renders surfaces**: **none.** `_design.md` records `surfaces: []` and states
  the no-surface determination as what the human signed off; its `## Items` and
  `## Signatures` blocks are N/A, so this story claims no design-item `path` id.
  The signature authority is `spec/SPECIFICATION.md:4632-4731` (§4.0) and
  `:4998-5013` (the probe), and the design sign-off is not contradicted by
  naming them — it is what sends us there.
- **Conformance rule(s)**: **none yet, and that is a schedule fact rather than
  an exemption.** No projection rule exists in the tree until HS-S0007. This
  store is the *subject* the following rules will observe, and they are named
  here so the implementer knows what the store is about to be measured against:
  `commit_advances_the_checkpoint` and `commit_is_atomic_with_the_read_model`
  (HS-S0007), `dropped_batch_leaves_store_usable`, `rollback_leaves_both_unchanged`,
  `commit_rejects_a_foreign_batch`, `commit_rejects_a_regressing_position`
  (HS-S0010), the `reset` family (HS-S0012) and
  `batch_reads_reflect_pending_writes` (HS-S0013). If any of them cannot be
  written against this store's shape, the store is wrong and the finding belongs
  to that story.
- **Clause(s)**: discharges **PS-5**'s stated compile-time observation —
  "`MemoryProjectionStore` and one real adapter compiling without the `where
  Self: 'a` clause" (`spec/SPECIFICATION.md:4860-4872`) — for the
  `MemoryProjectionStore` half; produces the finding **PS-34**
  (`:5546-5560`) is contingent on, without editing its marker; documents
  **PS-36** (`:5601-5627`, `[FROZEN]`, documentation is its only mechanism); and
  implements **PS-11**'s probe obligation (`:4977-4996`) for the first time. It
  is the oracle for PS-1, PS-6, PS-7, PS-8, PS-12, PS-15, PS-16 – PS-22.
  **No clause is amended and no `[FROZEN]` clause is edited**; every maturity
  marker and rule citation is HS-S0016's, under `cargo xtask spec-trace`.
- **Advances DoD scenario**: initiative **DoD 7** — "the projection suite
  discriminates. Two structurally unlike batch shapes pass it" — first shape,
  and the oracle that makes the second one's disagreement legible
  (`.bklg/from-contract-to-published-library/initiative.md:377-380`). Project DoD 2 likewise. It does not move DoD 8
  (the freeze verdict), which is HS-P0015's.

## PR boundary

```
crates/happenstance-core/src/**
crates/happenstance-core/Cargo.toml
crates/happenstance-core/tests/**
CHANGELOG.md
.bklg/from-contract-to-published-library/projection-store-freeze/memory-projection-store/**
```

**In this PR.** The new `memory`-gated module and its `MemoryProjectionStore` +
batch + uninhabited error type; the `SendProjectionStore` impl; the
`ProjectionProbe` impl under `all(feature = "memory", feature = "conformance")`;
the two doctests (toy-store impl on the port, runnable walkthrough on the store);
the port-side rustdoc that disposes of the `E0195` trap and documents PS-36's
`Batch: Send` transitivity; the crate root's feature-flag list and "Getting
started" prose gaining the store, spelled with no intra-doc link; the export
block and `[features]` mount; the crate's own tests for atomicity, authority,
foreign batch, regression, rollback/drop and `reset`; a `CHANGELOG.md`
`[Unreleased] / ### Added` entry.

Touching `crates/happenstance-core/src/lib.rs` and `Cargo.toml` **is the mount**,
not scope drift; touching `projection.rs` is confined to **documentation and the
doctest** — its signatures are HS-S0004's and are not edited here.

**Explicitly not in this PR.** Any file under `crates/happenstance-testkit/`
(no fixture, no rule, no mutant, no registry entry — HS-S0007 onward); the
buffering CF-5 variant (HS-S0011); `spec/SPECIFICATION.md` in any form,
including maturity markers and rule citations (HS-S0016); any `.kb/` atom, new
or amended (ADRs are HS-S0002's, and by AC-008 they are already accepted before
this slice starts); the adapter skeletons (HS-S0004 restated them); any change to
`ProjectionId`; `happenstance` or any adapter crate.

**Merge DoD, one line.** `cargo xtask ci --fast` is green and
`cargo xtask affected --base main` is green for `happenstance-core`, with
`cargo test -p happenstance-core --all-features --doc` and the
`--no-default-features` doc build both passing — the full `cargo xtask ci`
(spec-trace, the mandatory wasm32 steps) is the *project* boundary's bar, met at
`whole-gate-run-and-proof-artefact` (HS-S0017).

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The store exists, mounted at both halves | Private module gated `#[cfg(feature = "memory")]` + `#[cfg_attr(docsrs, doc(cfg(feature = "memory")))]`; `MemoryProjectionStore`, its batch type and its error type re-exported under the same pair beside `MemoryEventStore`. `memory` already implies `std`. No new dependency. | `crates/happenstance-core/src/lib.rs:101-103,120-122`; `crates/happenstance-core/Cargo.toml` `[features]`; `_decomposition.md` Architecture Note 1 |
| `begin` is neither `async` nor fallible | `fn begin(&self) -> Self::Batch` returns an owned, empty delta stamped with this store instance's identity. Opening a buffer cannot fail; an adapter needing a round trip takes it at `commit`. Do not restore `async`/`Result` for symmetry with `EventStore`. | `spec/SPECIFICATION.md:4884-4897` (PS-6); `_decomposition.md` Architecture Note 9 |
| `commit` is one unit of work | Read-model rows and the checkpoint are installed under one lock acquisition, or neither is. Nothing in an open batch is observable through `probe_read` or `checkpoint`. | `spec/SPECIFICATION.md:4733-4759` (PS-1, `[FROZEN]`) |
| `commit` records authority | `Authority::Live` → `Checkpoint::Live { through }`; `Authority::Rebuilding` → `Checkpoint::Rebuilding { through }`. A never-seen or freshly `reset` id reads `Checkpoint::NeverRun`. The tuple form is refused for the reason the enum exists. | `spec/SPECIFICATION.md:4643-4658`, `:4632-4731` |
| Position is *considered*, not applied | A `commit` whose batch wrote nothing still advances the checkpoint to `position`. A `position` below the recorded checkpoint returns `CommitError::CheckpointRegression { current, attempted }`. | `RUNBOOK.md:3908-3910` (PS-21, PS-22); `spec/SPECIFICATION.md:4632-4731` |
| Distinct projections advance independently | Checkpoints are keyed by `ProjectionId`; committing one id leaves every sibling id's checkpoint untouched. | `crates/happenstance-core/src/projection.rs:37-42`; `spec/SPECIFICATION.md` §4 |
| Foreign batches are rejected at run time | `begin` stamps a per-instance identity; `commit`/`reset` compare and return `CommitError::ForeignBatch` / `ResetError::ForeignBatch` on mismatch. The owned batch did **not** close this hazard statically. | `spec/SPECIFICATION.md:5099-5125` (PS-15, `[PROVISIONAL]`), `:5126-5155` |
| `rollback` and bare `Drop` both leave the store usable | `rollback(batch)` discards; a batch dropped without `commit`/`rollback` discards **and** the store serves a subsequent `begin`/`commit` normally. The second half is the clause, not decoration — a reviewer's probe found a store answering `Busy` forever. | `spec/SPECIFICATION.md:4898-4917` (PS-7 `[FROZEN]`, PS-8); `RUNBOOK.md:3893-3897` |
| `reset` clears rows and checkpoint as one unit, scoped to one id | The caller's batch carries the deletes — the port has no idea what the read model is — and the id returns to `NeverRun` in the same unit. Sibling ids and their rows are untouched. This store does not refuse, so `ResetError::Refused` is unreachable here and the rule that needs it is served by a hostile store elsewhere. | `spec/SPECIFICATION.md:5165-5170`, `:5200-5217`; `RUNBOOK.md:3904-3907` |
| `ProjectionProbe` is implemented, and reads through the batch | Under `all(feature = "memory", feature = "conformance")`: `READS_THROUGH_BATCH = true`, real `probe_write` / `probe_delete_all` / `probe_read` / `probe_read_through`. `probe_delete_all` exists so `reset` is checkable without the suite knowing what a read model is. | `spec/SPECIFICATION.md:4998-5013`, `:5052-5074` (CF-18); `_decomposition.md` Architecture Note 4 |
| A generic round-trip proves the seam is generic | A test in `happenstance-core`'s own tests, gated by `conformance`, writes through `probe_write` and reads through `probe_read` bound on `ProjectionStore` + `ProjectionProbe` only — never through an inherent method on the concrete store. | `_decomposition.md` Testing brief, AC-009 row |
| One impl, two flavours | `impl SendProjectionStore for MemoryProjectionStore`; the bare flavour derives from `#[trait_variant::make]`. Tests and helpers bind `ProjectionStore` and import one name per module. No `#[async_trait]`. | `crates/happenstance-core/src/projection.rs:87`; `crates/happenstance-core/src/memory.rs:293-294`; `.kb/decisions/0001-async-port-flavours.md`; CLAUDE.md constraint 4 |
| Uninhabited error type | An empty enum with a `thiserror` message naming *this* store, mirroring `MemoryStoreError`'s argument: it proves the contract does not require a fallible path and documents the absence at the type level. `CommitError`/`ResetError` remain meaningfully fallible. | `crates/happenstance-core/src/memory.rs:283-291`; `standards/rust/30-error-taxonomy.md` |
| The impl spells the concrete batch type | `commit(&self, batch: MemoryProjectionBatch, …)` — not `Self::Batch<'_>`. Compiling is the evidence PS-5's claim to have removed `error[E0195]` is true; failing to compile is a finding against HS-S0004, not a workaround here. | `spec/SPECIFICATION.md:4860-4878`; `references/adapter-shapes.md:186-194` |
| Two doctests, two jobs | Toy-store impl doctest on `ProjectionStore` (no feature dependency, PS-34's stated rule); runnable `begin`→write→`commit`→read-back walkthrough on `MemoryProjectionStore`. Both run under `cargo test --doc`. | `spec/SPECIFICATION.md:5546-5553`; `crates/happenstance-core/src/memory.rs:36-60`; `standards/rust/62-doctests-and-harnesses.md` |
| PS-36 documented, and any `compile_fail` spelled bare | The `Send` flavour transitively requires `Batch: Send`; `type Batch: Send;` is unwritable without breaking wasm32, so the port documents it. A `compile_fail` doctest carries **no** error code. | `spec/SPECIFICATION.md:5601-5627`; `crates/happenstance-testkit/src/contract.rs:400-403` |
| Rustdoc obligations | Every fallible public item carries `# Errors` naming conditions rather than the error type; the unusual shapes (`begin` infallible, `reset` taking the caller's deletes, the owned batch) each name the alternative that lost, once, at the item. | `standards/rust/70-rustdoc-obligations.md` (RS-70-5); `_decomposition.md` UX brief AC-U05 |
| No gated intra-doc link | Nothing that renders without `memory` links to the store or its module; nothing that renders without `conformance` links to the probe impl. The crate root already spells `MemoryEventStore` plainly for this reason. | `crates/happenstance-core/src/lib.rs:66-74` (D13); `crates/happenstance-testkit/src/lib.rs:112-132` |
| The feature matrix stays green | Host powerset and the wasm32 powerset both widen with `conformance`; `conformance` without `memory`, `memory` without `conformance`, and `--no-default-features` (`no_std`) must each compile, and `happenstance-core` still builds for `wasm32-unknown-unknown`. | `xtask/src/main.rs:546-556`, `:558-591`; `standards/rust/51-features-and-no-std.md`, `standards/rust/52-wasm32-and-target-cfg.md` |
| The change is announced | A `[Unreleased] / ### Added` entry naming the store, its feature and what it unblocks — the changelog is written as the work lands, not reconstructed at release. | `CHANGELOG.md:1-25` |

## Data and migrations

**N/A — and the reason is the point of the store.** `MemoryProjectionStore` holds
its read model and its checkpoints in process memory behind a lock; there is no
schema, no file, no connection and no persisted representation, so there is
nothing to migrate and no upgrade path to define. Its state is created by
`::new()` and dropped with the value.

Three adjacent things this story deliberately does not touch, recorded so their
absence is a decision rather than an omission:

- **No wire format.** Nothing here is serialised, and `serde` stays off the
  contract crate's default features — payloads are opaque `Bytes` and the
  `serde` feature covers envelope types only (ADR-0003,
  `.kb/decisions/0003-opaque-payloads.md`; CLAUDE.md binding constraint 2).
- **No published-surface migration.** `happenstance-core` is unpublished, so
  adding items behind `memory` and `conformance` breaks no downstream consumer;
  the semver promise starts at first publish (`publication-and-positioning`,
  HS-P0016). The `CHANGELOG.md` entry is the whole of the obligation today.
- **No checkpoint persistence format.** What a *real* adapter stores for a
  checkpoint row — and the fact that `ProjectionId` is an unvalidated primary key
  in it — is `.kb/open-questions/projection-id-is-unvalidated.md`, owned
  elsewhere and explicitly not repaired as a side effect of this story
  (`_decomposition.md` Architecture brief AC-A09).

## Acceptance criteria

The persona throughout is **P2, the adapter author**
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:114-181`),
whose goal is "an executable definition of *correct* they can run against their
own storage system" and whose journey step 2 is the one this story repairs — the
compiler today accepts a `todo!()` body as readily as a real one, and the first
person to write a real one meets `error[E0195]` with nothing to copy. Where a
criterion is written from **P1, the application author** (`:42-113`), the row
says so: P1 is the reader of the runnable walkthrough, and the person for whom
"application code can be written before any real adapter exists"
(`crates/happenstance-core/src/memory.rs:16-34`).

Every criterion below is a claim about behaviour a caller can observe through
the crate's **public** surface. That is why the behavioural tests live in
`crates/happenstance-core/tests/projection_memory.rs` — an integration test file
compiles against the published API only, so a store that exists but is not
re-exported fails to compile rather than passing quietly. The mount is proven by
construction, not by a separate assertion.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an adapter author who has added `happenstance-core` with default features and is looking for something to copy, **WHEN** they write `use happenstance_core::MemoryProjectionStore;` and construct one, **THEN** the type, its batch type and its error type resolve from the crate root under the `memory` feature — the same gate pair as `MemoryEventStore` — and `CHANGELOG.md`'s `[Unreleased] / ### Added` says the store now exists and what it unblocks. A type mounted in `lib.rs` but absent from `Cargo.toml`'s `[features]`, or the reverse, does not satisfy this. | Unit: `crates/happenstance-core/tests/projection_memory.rs` compiles at all — it can name only public items, so the re-export at `crates/happenstance-core/src/lib.rs:98-124` is load-bearing for the whole file; `store_is_reachable_from_the_public_surface` constructs one and reads a checkpoint. Static: review of `crates/happenstance-core/Cargo.toml` `[features]` (no new dependency; `memory = ["std"]` unchanged) and of the `CHANGELOG.md` entry. |
| AC-002 | **GIVEN** an adapter author copying this store's shape onto a `Send` runtime and, separately, onto `wasm32`, **WHEN** they read how the one impl serves both, **THEN** they find `impl SendProjectionStore for MemoryProjectionStore` with the bare flavour derived by `#[trait_variant::make]` and no `#[async_trait]` anywhere; and **WHEN** they call `begin`, **THEN** it is neither `async` nor fallible and hands back an owned, empty batch — so a transport that cannot afford a round trip to open a buffer is still able to implement the port. | Unit: `crates/happenstance-core/tests/projection_memory.rs::begin_is_synchronous_and_infallible` (binds the value directly, no `.await`, no `?`) and `::send_impl_satisfies_the_bare_bound` (a generic `fn` bound on `ProjectionStore` accepting `&MemoryProjectionStore`). Static: `cargo clippy --workspace --all-targets --all-features -D warnings`; a grep-level review that no `async_trait` import entered `happenstance-core` (ADR-0001, `.kb/decisions/0001-async-port-flavours.md`). |
| AC-003 | **GIVEN** a rule author about to write `commit_is_atomic_with_the_read_model` and needing a store they can trust to be *right*, **WHEN** they open a batch against this store, write through it, and inspect the store before committing, **THEN** neither the rows nor the checkpoint have moved; and **WHEN** `commit` returns `Ok`, **THEN** both are visible — installed under one lock acquisition, never rows-then-checkpoint. A store that publishes rows first and the checkpoint second is `CheckpointOnlyStore`'s defect with the timing changed, and the oracle must not model it. | Unit: `crates/happenstance-core/tests/projection_memory.rs::open_batch_is_invisible_until_commit` (probe-read and `checkpoint` both unchanged while a written batch is open) and `::commit_installs_rows_and_checkpoint_together`. Static: review that `commit` takes exactly one write guard and returns without an intermediate publish. |
| AC-004 | **GIVEN** an operator's projection that is rebuilding, and a sibling that is live, **WHEN** each commits, **THEN** the store records `Checkpoint::Rebuilding { through }` for one and `Checkpoint::Live { through }` for the other, an id never committed (or freshly `reset`) reads `Checkpoint::NeverRun`, and neither id's checkpoint moves when the other commits; **AND WHEN** a batch that wrote nothing is committed at a position, **THEN** the checkpoint still advances to it — the position is the one *considered*, not the one applied — while a position below the recorded checkpoint returns `CommitError::CheckpointRegression { current, attempted }` and changes nothing. | Unit: `crates/happenstance-core/tests/projection_memory.rs::commit_records_the_authority_it_was_given`, `::empty_batch_still_advances_the_checkpoint`, `::regressing_position_is_rejected_and_changes_nothing`, `::distinct_projections_advance_independently`. Never asserts a literal position — compares against positions handed in (CLAUDE.md, "never assert on literal position values"). |
| AC-005 | **GIVEN** an adapter author who has two stores in scope and writes `b.commit(a.begin(), …)` by mistake — which the owned batch did **not** make unrepresentable, because a lifetime names a region and not an instance — **WHEN** the call runs, **THEN** it returns `CommitError::ForeignBatch` and store `b` is unchanged; the same for `reset` with `ResetError::ForeignBatch`. The identity is stamped per store instance at `begin` and compared as an integer. | Unit: `crates/happenstance-core/tests/projection_memory.rs::commit_rejects_a_foreign_batch` and `::reset_rejects_a_foreign_batch`, each asserting both the error arm and that the receiving store's rows and checkpoint are untouched. |
| AC-006 | **GIVEN** an application author whose apply loop panics or returns early mid-batch, **WHEN** the batch is dropped bare — no `commit`, no `rollback` — **THEN** nothing it held is visible, **AND** the store serves a *subsequent* `begin` → write → `commit` normally. The second half is the clause, not decoration: a reviewer's probe found a store that answers `Busy` forever afterwards. `rollback(batch)` has the same effect, stated explicitly. | Unit: `crates/happenstance-core/tests/projection_memory.rs::dropped_batch_leaves_store_usable` (drop bare, then open and commit a second batch on the same handle and assert it took effect) and `::rollback_leaves_both_unchanged`. |
| AC-007 | **GIVEN** an operator rebuilding one projection from zero while its siblings keep serving, **WHEN** they call `reset` with a batch carrying the deletes — the port has no idea what the read model is, so the caller supplies them — **THEN** that id's rows and its checkpoint are cleared **in one unit**, the id reads `Checkpoint::NeverRun` again, and every sibling id's rows and checkpoint are untouched. `commit(empty, id, FIRST)` is not this: it silently skips event 1, and the store must not make the two look alike. | Unit: `crates/happenstance-core/tests/projection_memory.rs::reset_clears_rows_and_checkpoint_together`, `::reset_is_scoped_to_one_projection`, `::reset_returns_the_projection_to_never_run`. |
| AC-008 | **GIVEN** a rule author who must observe a read model without knowing what a read model is, **WHEN** they write generic code bound on `ProjectionStore + ProjectionProbe` only — never an inherent method on the concrete store — **THEN** `probe_write` then `probe_read` round-trips the value against `MemoryProjectionStore`; **AND** because this store applies on write, `READS_THROUGH_BATCH` is `true` and `probe_read_through` really returns a pending write rather than `unimplemented!()`. Declaring `false` here would make PS-12's rule skippable by everything in the workspace and is a **finding** for `read-through-and-rebuild-rules`, not a quiet flip. | Unit, gated `#[cfg(feature = "conformance")]`: `crates/happenstance-core/tests/projection_memory.rs::probe_round_trip_through_the_traits_only` (the helper takes `S: ProjectionStore + ProjectionProbe`) and `::probe_read_through_sees_a_pending_write`; plus `::probe_delete_all_supports_reset` so `reset` is checkable without the suite knowing the read model's shape. |
| AC-009 | **GIVEN** the adapter author of persona 2's journey step 2, who today writes an impl and is stopped by `error[E0195]: lifetime parameters or bounds on method 'commit' do not match the trait declaration` with nothing in the workspace saying `Self::Batch<'_>` is required, **WHEN** this story's impl spells the **concrete** batch type in `commit` and `rollback` and the crate compiles, **THEN** PS-5's claim to have removed the trap is discharged by a compiler rather than by a paragraph — and the port's own rustdoc says so **once, at the item whose unusual shape bought it**, naming the alternative that lost, together with PS-36's `Batch: Send` transitivity, which `type Batch: Send;` cannot express without breaking wasm32. If it does **not** compile, that is a finding against `owned-batch-port-shape` and this story stops. | Static: the crate compiling with the concrete type **is** the evidence (`cargo check -p happenstance-core --all-features`); recorded in the implementation report as the transcript, or as the halt. Unit: the toy-store doctest in `crates/happenstance-core/src/projection.rs` also spells a concrete batch type, so `cargo test -p happenstance-core --all-features --doc` re-proves it on every run. Static: review that the trap and PS-36 each appear once, at the item, per `standards/rust/70-rustdoc-obligations.md` RS-70-5 / UX brief AC-U05, AC-U06; any `compile_fail` doctest is spelled **bare**, never `compile_fail,E0308`. |
| AC-010 | **GIVEN** two different readers — an adapter author who needs a minimal impl to copy that does not force them to enable a feature, and an application author (P1) who needs to see the loop actually run — **WHEN** they open the two pages, **THEN** `ProjectionStore`'s rustdoc carries a **toy-store impl** doctest with no dependency on the `memory` feature, and `MemoryProjectionStore`'s rustdoc carries a **runnable walkthrough** (`begin` → write → `commit` → read back the rows *and* the checkpoint) mirroring `crates/happenstance-core/src/memory.rs:36-60`. Both execute; neither is a `no_run` or `ignore` sketch. | Unit: `cargo test -p happenstance-core --all-features --doc` runs both. Static: review that the port doctest names no `memory`-gated item, and that the walkthrough asserts on the checkpoint as well as the rows — a walkthrough that stops at the rows demonstrates half the invariant the store exists to demonstrate. |
| AC-011 | **GIVEN** a `no_std` consumer, a `wasm32` consumer, a consumer who wants `conformance` without `memory`, and docs.rs, **WHEN** each configuration is built, **THEN** all of them succeed: no page that renders without `memory` intra-doc-links the store or its module, no page that renders without `conformance` links the probe impl, `cargo doc --no-default-features` (a hard error, not a warning, for a broken link — D13) is green, the nightly `--cfg docsrs` build renders both feature badges, and the host and `wasm32` feature powersets are green over the widened combination set. The crate root gains the store in its feature-flag list and "Getting started" prose, spelled plainly, exactly as `MemoryEventStore` already is. | Static, all inside `cargo xtask ci`: the `--no-default-features` doc build; `cargo hack check --workspace --feature-powerset --no-dev-deps` (`xtask/src/main.rs:546-556`); `cargo hack check -p happenstance-core … --target wasm32-unknown-unknown --feature-powerset --no-dev-deps` (`:558-591`); the mandatory plain `wasm32` build of `happenstance-core`; the nightly `--cfg docsrs` rustdoc build. Review: `crates/happenstance-core/src/lib.rs:66-83` gains the store with the link deliberately absent, per the note already there. |

**Coverage of the traced project AC.** Project **AC-012** — "`MemoryProjectionStore`
ships behind the `memory` feature, is the doctest target for the port, and the
`E0195` spelling trap is documented where an implementer meets it (PS-34,
PS-36)" (`project.md:218-220`) — is carried by **AC-001** (ships behind
`memory`), **AC-009** (the trap, disposed of by compiling and documented at the
item, with PS-36), **AC-010** (doctest target, both halves) and **AC-011** (the
`docsrs` render that the Testing brief's AC-012 row names as the Static
instrument). AC-002 – AC-008 are the store's behaviour: not separately traced,
because they are what makes the store an *oracle* rather than a compiling
example, and project AC-004's "first shape" and every rule story from
`projection-suite-entry-point` onward measure themselves against them
(`_storymap.md`, Coverage table, AC-004 row).

## Interaction quality

**This story renders no surface, and that is a signed-off determination rather
than an assumption.** `_design.md` records `surfaces: []`, its `## Items`,
`## Signatures` and `## Anti-patterns` blocks are all N/A, and the sign-off says
what was approved is "the no-surface determination itself"
(`.bklg/from-contract-to-published-library/projection-store-freeze/_design.md:48-50,84-101`).
`design.capture` is absent from `.redkiln/config.yaml`, so the perceptual review
is a **declared skip**, not a silent pass.

So the RFC §6.7/D6 families do not evaporate — they land in the two non-visual
surfaces `_design.md` *does* name (`:24-39`): the **type surface** a caller
writes Rust against, and the **rendered rustdoc** a reader browses. Composition
authority for those two, in the absence of `_design.md` `## Items`, is the UX
brief's AC-U03 – AC-U06 (`_decomposition.md:110-148`) and
`standards/rust/70-rustdoc-obligations.md`, which is where the intake gate sends
us. Every invariant below is already an **AC-### row in the table above**; this
section says which row carries it and how it is verified — nothing here is a
free-floating bullet.

**State family.**

| Invariant | Translated to this medium | Carried by | Verified by |
| --- | --- | --- | --- |
| Reversibility | An abandoned unit of work is undoable *and* leaves the thing usable: `rollback`, and bare `Drop`, both discard and the store still serves the next `begin`/`commit`. The "still usable" half is the one a store fails in practice. | **AC-006** | `dropped_batch_leaves_store_usable`, `rollback_leaves_both_unchanged` |
| In-place, not a context jump | The caller never has to leave the crate or reach past the port to see the effect of their write: `probe_write` → `probe_read` through the traits only, no inherent method on the concrete store. | **AC-008** | `probe_round_trip_through_the_traits_only` |
| Non-occlusion | No configuration hides the surface from the reader: an item that renders on one feature set must not break the doc build on another, and the badge says which feature it needs. | **AC-011** | `cargo doc --no-default-features`; nightly `--cfg docsrs` |
| Preserved state across the operation | Committing one projection preserves every sibling's checkpoint and rows; `reset` is scoped to one `(store, id)`. The analogue of preserved selection: acting on one item does not disturb the others. | **AC-004**, **AC-007** | `distinct_projections_advance_independently`, `reset_is_scoped_to_one_projection` |
| Reachability without a special mode | The item is reachable from the crate root under a documented default feature — not behind a `--cfg`, not through a private path. An integration test that can only name public items is the check. | **AC-001** | `crates/happenstance-core/tests/projection_memory.rs` compiling |

**Composition family.** For a library, "an unstyled render satisfies every ARIA
assertion" has an exact counterpart: **a correct signature with a bare doc
comment satisfies every compiler check and teaches nobody.** These are the rows
that make that fail.

| Invariant | Translated to this medium | Carried by | Verified by |
| --- | --- | --- | --- |
| Presentation exists at all | Every public item carries real composed rustdoc, not a restated signature: `# Errors` naming *conditions* rather than the error type, and a runnable example where the item is the entry point. | **AC-010**, **AC-009** | `cargo test --doc`; review against `standards/rust/70-rustdoc-obligations.md` |
| Placement — at the item, not in an ADR | The `E0195` trap and PS-36's `Batch: Send` transitivity are documented on the port's own rustdoc, the page an implementer already has open. "A trap documented only in ADR-0017 is documented where the person who needs it is not" (UX brief AC-U06). | **AC-009** | review; the doctest is on `ProjectionStore` itself |
| Transience — persistent, revealed, or on demand | Three tiers, deliberately: the crate root's feature list and "Getting started" prose are *persistent chrome*, seen by everyone; the trap and the alternatives-that-lost are *revealed* at the item you are implementing; the long reasoning stays in the ADR and the specification, *opened on demand*. | **AC-011** (chrome), **AC-009** (revealed) | review of `lib.rs:66-83`; review of the item docs |
| Density budget, with its number | **Once.** Each unusual shape names the alternative that lost exactly one time, at its own item — `begin` infallible, `reset` taking the caller's deletes, the owned batch, `Batch: Send`. Twice is two things to update and one that goes stale; zero is the state that produced zero adapters. The walkthrough doctest is likewise **one** end-to-end loop, not four fragments. | **AC-009**, **AC-010** | review (RS-70-5, `standards/rust/70-rustdoc-obligations.md`) |
| Hierarchy | Crate root → module → item, each answering a different question, none repeating the one below it: the root says the store exists and which feature; the module says what a reference implementation is *for*; the item says how to drive it. Mirrors `crates/happenstance-core/src/memory.rs:16-34` then `:36-60`. | **AC-010**, **AC-011** | review against `memory.rs`'s structure |
| Named anti-pattern — the gated intra-doc link | The one this workspace has already paid for: a link into a `cfg`-absent module is a **hard** rustdoc error under `--no-default-features`, which is a gate step. The root already spells `MemoryEventStore` plainly for this reason (`lib.rs:70-73`), and the testkit paid for it once (`crates/happenstance-testkit/src/lib.rs:112-132`). | **AC-011** | `cargo doc -p happenstance-core --no-default-features` |
| Named anti-pattern — the stricter-looking weaker check | `compile_fail,E0308` on the PS-36 doctest. rustdoc on 1.97.1 silently ignores a code the diagnostic does not carry, so the annotated spelling asserts *less* than the bare one. Bare, always. | **AC-009** | review; `spec/SPECIFICATION.md:5601-5615`; `crates/happenstance-testkit/src/contract.rs:400-403` |
| Named anti-pattern — the tuple that can lie | `Checkpoint` stays a three-variant enum, and the sentence explaining why — `(None, true)` is "authoritative, never run, which means nothing" — sits where the reader meets `checkpoint`. This story consumes that decision; it does not re-open it (UX brief AC-U03). | **AC-004** | `commit_records_the_authority_it_was_given`; review of the rustdoc |
| Named anti-pattern — half a mount | An item in `lib.rs` with no `[features]` entry, or the reverse, is the library equivalent of a constructed-but-unmounted component: it compiles for whoever wrote it and is invisible to everyone else. | **AC-001** | the integration test file compiling; `cargo hack` powerset |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | `commit` is handed a batch minted by a different store instance | `Err(CommitError::ForeignBatch)`; the receiving store's rows and checkpoint are unchanged. The stamp is compared, not the pointer — an integer comparison now that the batch is owned (`spec/SPECIFICATION.md:5099-5125`). AC-005. |
| **EC-002** | `reset` is handed a foreign batch | `Err(ResetError::ForeignBatch)`, same non-mutation guarantee. AC-005. |
| **EC-003** | `commit` is given a `position` strictly below the id's recorded checkpoint | `Err(CommitError::CheckpointRegression { current, attempted })`, and **nothing** is applied — not the rows, not the checkpoint. The check happens before either write, inside the same guard. AC-004. |
| **EC-004** | The store's own adapter error variant | Unreachable by construction: the error type is an **uninhabited** enum, mirroring `MemoryStoreError`'s argument at `crates/happenstance-core/src/memory.rs:283-291` — it proves the contract does not *require* a fallible read path and documents the absence at the type level. `CommitError` and `ResetError` stay meaningfully fallible via EC-001 – EC-003, so nothing the suite needs is lost. It is a distinct type from `MemoryStoreError`, whose `Display` names the event store. |
| **EC-005** | `ResetError::Refused` | Never produced by this store — it has no policy to protect. That is not a gap: PS-20's rule (`refused_reset_changes_nothing`) is served by a hostile store in the testkit, owned by `reset-rules` (HS-S0012). Do **not** invent a refusal mode here so the arm is "covered"; a store that refuses arbitrarily is a worse oracle. |
| **EC-006** | Lock poisoning — a panic in another thread while the store's guard was held | Recovered, not propagated, exactly as the event store already does and for the same stated reason: every mutation happens in one guarded operation and the validation preceding it does not mutate, so poisoning carries no information (`crates/happenstance-core/src/memory.rs:190-199`, `PoisonError::into_inner`). A spurious `Err` from the oracle would be a rule failure attributed to the adapter under test. |
| **EC-007** | The impl fails to compile with `error[E0195]` when `commit` spells the concrete batch type | **Halt condition, not a workaround.** Do not "fix" it by writing `Self::Batch<'_>`; that would prove PS-5 false while hiding it. Report it as a finding against `owned-batch-port-shape` (HS-S0004) and stop this story (`spec/SPECIFICATION.md:4860-4878` vs `references/adapter-shapes.md:186-194`). |
| **EC-008** | A gated intra-doc link breaks `cargo doc --no-default-features` | A **hard** rustdoc error and a gate failure, not a warning. Fix by spelling the name plainly, the way `crates/happenstance-core/src/lib.rs:70-73` already does — never by removing the doc build from the gate. AC-011. |
| **EC-009** | The `conformance` feature is enabled without `memory` (or vice versa) | Must compile. The two features are independent; the probe impl is gated `#[cfg(all(feature = "memory", feature = "conformance"))]` and nothing outside that gate may reference it (`_decomposition.md` Architecture Note 4). The powerset step is what catches a `cfg` written as `any` or omitted. AC-011. |

## Non-functional

| id | requirement | why, and how it is held |
| --- | --- | --- |
| **NF-001** | **No new dependency, in any feature combination.** | `memory = ["std"]` already costs nothing extra (`Cargo.toml`, "Costs no extra dependencies and backs every doctest in this crate"). A projection store that needed a map crate or a hasher would put a supply-chain cost on the crate everything else in the workspace depends on. `cargo deny check` and the `[dependencies]` diff are the check. |
| **NF-002** | **Obviously correct beats fast.** | This is the oracle: when a rule fails, the adapter is presumed wrong and this store is presumed right, so its correctness must be readable in one sitting (`RUNBOOK.md:3898-3903` sizes it at ~50 lines). No optimisation that splits `commit` into more than one guarded section — that is precisely the shape `CheckpointOnlyStore` fails for. |
| **NF-003** | **Builds for `wasm32-unknown-unknown` under every feature combination the powerset reaches.** | `happenstance-core` is the crate the mandatory wasm32 step names on purpose (CLAUDE.md, "Commands"), and it is the standing guard on ADR-0001. `std::sync::RwLock` is available on `wasm32-unknown-unknown`, which is why the event store already compiles there — do not introduce threads, time, or `std::thread` in the new module. `standards/rust/52-wasm32-and-target-cfg.md`. |
| **NF-004** | **MSRV 1.97.1, and no silent movement.** | ADR-0029 raised it deliberately; what stays forbidden is moving it without an ADR (CLAUDE.md, binding constraint 5). Let-chains are available. The `msrv` CI job is the check; nothing in a ~50-line in-memory store should approach the floor. |
| **NF-005** | **`no_std` stays intact.** | The new module is `std`-only via `memory`, gated the same way `memory.rs` is; `--no-default-features` must still build *and* document. `standards/rust/51-features-and-no-std.md`. |
| **NF-006** | **Feature-powerset cost is accepted, not accidental.** | `conformance` widens the host and wasm32 powersets (Architecture Note 4 sizes it as "eight combinations to thirty-two"). That growth is `projection-probe-conformance-feature`'s decision, already taken; this story must not add a *third* new feature to the crate and must not make an existing combination fail. |
| **NF-007** | **Rustdoc renders without warnings on the `docsrs` configuration.** | `[package.metadata.docs.rs]` already sets `all-features = true` and `--cfg docsrs`; both new gates need `#[cfg_attr(docsrs, doc(cfg(…)))]` so the badge appears, or a reader on docs.rs cannot tell which feature they need. |

## Implementation notes (non-prescriptive)

Shape only — the signatures are HS-S0004's and the probe is HS-S0005's.

**Order that keeps the feedback tight.** Land the store and its behavioural tests
first, with `conformance` off; add the probe impl and its gated tests second;
write the two doctests third; do the crate-root prose, the feature badges and the
`CHANGELOG.md` entry last, then run the doc builds. Reversing the first two makes
a probe failure and a store failure look alike.

**Mirror `memory.rs`, do not extend it.** `crates/happenstance-core/src/memory.rs`
is the pattern: `:16-34` for why a reference implementation exists at all (three
reasons — the oracle, the runnable docs, application code before adapters),
`:36-60` for the walkthrough's shape, `:190-199` for poison recovery and its
justification, `:283-294` for the uninhabited error and the `Send`-flavour impl.
A new module is the right home; the event store's file is not the place for a
second store.

**State shape.** One guard over a struct holding the read model and the
checkpoints — keeping them behind *one* lock is what makes PS-1's "under one
lock acquisition" true by construction rather than by discipline. Checkpoints are
keyed by `ProjectionId`; an absent key is `Checkpoint::NeverRun`, which is why
`reset` can restore that state by removing rather than by writing a sentinel.

**The batch.** Owned, holding a materialised delta plus the minting store's
stamp. Apply-on-write means the delta is the thing `probe_read_through` reads
from, layered over committed state — that is the whole of `READS_THROUGH_BATCH =
true`. The stamp can be as simple as a monotonic instance counter; what matters
is that it is per *instance*, not per type, or `commit_rejects_a_foreign_batch`
becomes untestable.

**Where the tests go, and why there.** `crates/happenstance-core/tests/projection_memory.rs`
— an integration test file, so it can name only public items and therefore proves
the mount by compiling. Gate the probe tests with `#[cfg(feature = "conformance")]`
inside that one file rather than adding a second file. `tokio` is already a
dev-dependency with `macros` and `rt`.

**Halt conditions, stated so they are reported rather than absorbed.** (a) The
impl does not compile with the concrete batch type — EC-007, a finding for
HS-S0004. (b) Apply-on-write cannot honestly report `READS_THROUGH_BATCH = true`
— a finding for `read-through-and-rebuild-rules`, not a flip to `false`. (c) A
signature needs changing to make any of this work — a finding for HS-S0004 and an
ADR-0017 clause, never an edit here.

**What good looks like when you are done.** An adapter author can open
`ProjectionStore`'s docs, copy the toy impl, compile it, and never meet
`error[E0195]`; and `projection-suite-entry-point` can point its first two rules
at this store and have them mean something.

## Tests and CI (merge gate)

Tier vocabulary is this project's own (`_decomposition.md` Testing brief:
**Static** reads source without executing it; **Unit** is `cargo test` in
process; **Integration** is a conformance rule driving a store; **E2E** is
`cargo xtask ci` whole). No projection rule exists until HS-S0007, so this story
has **no Integration row** — that is a schedule fact, and inventing one here
would mean building the suite inside this PR.

| tier | command / path | proves |
| --- | --- | --- |
| Static | `cargo fmt --check`; `cargo clippy --workspace --all-targets --all-features -D warnings` | House style and lint floor; catches an `async_trait` import or a stray `todo!()` reaching the new module (AC-002). |
| Static | `cargo check -p happenstance-core --all-features` | **The `E0195` disposition.** The impl spelling the concrete batch type and compiling *is* PS-5's evidence; its failure is EC-007's halt (AC-009). |
| Static | `cargo doc -p happenstance-core --no-default-features` (gate step) | No gated intra-doc link. A broken link here is a hard error, which is exactly why this step exists (AC-011, EC-008). |
| Static | `cargo hack check --workspace --feature-powerset --no-dev-deps` (`xtask/src/main.rs:546-556`) | `conformance` without `memory`, `memory` without `conformance`, and `--no-default-features` all compile (AC-011, EC-009, NF-005). |
| Static | `cargo hack check -p happenstance-core -p happenstance-neon -p happenstance-testkit --target wasm32-unknown-unknown --feature-powerset --no-dev-deps` (`:558-591`), above the mandatory plain wasm32 build of `happenstance-core` | The store and probe compile on the `!Send` target; ADR-0001's standing guard still holds with a new module in the crate (AC-011, NF-003). |
| Static | nightly `--cfg docsrs` rustdoc build (mandatory-if-tool-present) | Both feature badges render — the Testing brief's named Static instrument for project AC-012 (AC-011, NF-007). |
| Static | review of `crates/happenstance-core/Cargo.toml` `[dependencies]` diff; `cargo deny check` | No new dependency entered the crate everything depends on (NF-001). |
| Unit | `cargo test -p happenstance-core --all-features --doc` | Both doctests run: the toy-store impl on `ProjectionStore` and the walkthrough on `MemoryProjectionStore` (AC-009, AC-010). |
| Unit | `crates/happenstance-core/tests/projection_memory.rs` — the file compiling | The mount is real: an integration test can name only public items, so a missing re-export fails here (AC-001). |
| Unit | `crates/happenstance-core/tests/projection_memory.rs` — `open_batch_is_invisible_until_commit`, `commit_installs_rows_and_checkpoint_together` | PS-1's atomicity, on the store every later rule is measured against (AC-003). |
| Unit | same file — `commit_records_the_authority_it_was_given`, `empty_batch_still_advances_the_checkpoint`, `regressing_position_is_rejected_and_changes_nothing`, `distinct_projections_advance_independently` | Checkpoint semantics: authority, position-considered, regression, per-id scope (AC-004, EC-003). |
| Unit | same file — `commit_rejects_a_foreign_batch`, `reset_rejects_a_foreign_batch` | PS-15's run-time discharge of a hazard the owned batch did not close statically (AC-005, EC-001, EC-002). |
| Unit | same file — `dropped_batch_leaves_store_usable`, `rollback_leaves_both_unchanged` | PS-7/PS-8, including the half a store actually fails: still usable afterwards (AC-006). |
| Unit | same file — `reset_clears_rows_and_checkpoint_together`, `reset_is_scoped_to_one_projection`, `reset_returns_the_projection_to_never_run` | `reset` as one scoped unit of work (AC-007). |
| Unit | same file, `#[cfg(feature = "conformance")]` — `probe_round_trip_through_the_traits_only`, `probe_read_through_sees_a_pending_write`, `probe_delete_all_supports_reset` | The write seam is generic, and `READS_THROUGH_BATCH = true` is honest (AC-008). |
| Unit | `cargo test --workspace --all-features` | Nothing in `happenstance-testkit`, the examples or the skeletons regressed from a new module in `happenstance-core`. |
| E2E | `cargo xtask affected --base main` and `cargo xtask ci --fast` | **This story's merge bar** — the story/slice grain the project's non-terminal `--fast` rule sets (`_decomposition.md` Testing brief, "Merge-gate commands"). |
| E2E | `cargo xtask ci` (whole) | The **project** boundary's bar, met at `whole-gate-run-and-proof-artefact` (HS-S0017), not here. Named so the two grains are not confused: `--fast` omits `spec-trace` and the mandatory wasm32 steps, and this story's AC-011 leans on the latter — so run the wasm32 and doc steps directly (`cargo xtask wasm`, the powerset commands above) even at story grain. |

**Not run here, deliberately.** `cargo xtask spec-trace` may be run for
information but proves nothing this story owns: no clause marker or rule
citation is edited in this PR, and their disposition is HS-S0016's.

## Risks and coupling (PR-scoped)

| risk | why it bites here | the move |
| --- | --- | --- |
| **The slice's two dependencies land in the same context, and a signature is still warm.** `owned-batch-port-shape` and `projection-probe-conformance-feature` merge immediately before this. The temptation is to "just fix" a signature while implementing against it. | It would make the port's shape a residue of what was convenient to implement, which is exactly the failure the ADR sequencing (AC-008: ADRs accepted *before* the port change) exists to prevent. | Any signature change is a finding for HS-S0004 and an ADR-0017 clause. Report and stop; do not edit `projection.rs` beyond documentation and the doctest. |
| **`E0195` does not go away** (EC-007). | PS-34 is `[PROVISIONAL — contingent on PS-5]` and `references/adapter-shapes.md:186-194` recorded the opposite result while the GAT was still on the port. This is the first compile that can settle it. | Halt, report to HS-S0004, and do not spell `Self::Batch<'_>` as a workaround. The story is blocked, not degraded. |
| **The oracle bakes in apply-on-write assumptions the port does not require.** | Every rule written after this is written *against this store*. If a rule accidentally depends on read-through, `buffering-conformant-variant` (HS-S0011) discovers it three slices later, and CF-5's whole point — a legally unlike shape — is compromised. | Keep `READS_THROUGH_BATCH` as the only place read-through is assumed, and say in the store's rustdoc that it is one end of an axis whose other end is coming. Anything a rule needs beyond that is HS-S0013's finding. |
| **A gated intra-doc link slips in** (EC-008). | `cargo doc --no-default-features` runs late in the gate, so the failure arrives after everything else is green — and the temptation is then to add the link back "just in the docsrs build". | Run the `--no-default-features` doc build early and locally. The precedent and its wording are already in the tree at `lib.rs:70-73` and `crates/happenstance-testkit/src/lib.rs:112-132`. |
| **Scope creep into the testkit.** | Writing the store makes the first two conformance rules feel two lines away, and this PR's boundary explicitly excludes `crates/happenstance-testkit/`. | The rules are HS-S0007's, with a `ProjectionFixture` this story does not design. If a rule seems unwritable against this store, that is the finding — and it belongs in the report, not in a preemptive fixture. |
| **The uninhabited error type is read as a shortcut and "improved" into a populated one.** | A reviewer unfamiliar with `MemoryStoreError`'s argument may see an empty enum as an omission. | The rustdoc carries the argument at the type, as `memory.rs:283-291` does. It is a claim about the contract — that a fallible read path is not *required* — and populating it would delete the claim. |
| **Coupling to `SequencePosition`'s `NonZeroU64` shape.** | `Checkpoint` carries it, and `commit(empty, id, FIRST)` is the substitute `reset-rules` must later reject. A store that stored checkpoints as `u64` with `0` meaning "never run" would re-introduce exactly the `(None, true)` ambiguity the enum exists to forbid. | Store the enum, not a number. AC-004's `reset_returns_the_projection_to_never_run` is the test that notices. |
| **CHANGELOG drift.** `CHANGELOG.md:19-22` already tells readers `ProjectionStore` ships behind `unstable-projection` — a gate that does not exist yet (it is HS-S0016's, AC-014's second arm). | Adding this store without touching that paragraph is correct: the discrepancy is pre-existing and owned elsewhere. | Add only the `[Unreleased] / ### Added` entry. Do not "correct" the `unstable-projection` paragraph in passing — that is HS-S0016's, and editing it here would split one decision across two PRs. |

## Dependencies

**Blocks on** — both are slice-mates in `projection-port-and-probe`, and both
land before this story within the single implementation context (`_storymap.md`,
Merge order 2):

- **`owned-batch-port-shape`** (HS-S0004) — supplies `type Batch;` with no
  lifetime, the non-`async` infallible `begin`, and `Checkpoint`, `Authority`,
  `CommitError`, `ResetError`. Without it there is nothing to implement, and the
  `E0195` question cannot even be asked.
- **`projection-probe-conformance-feature`** (HS-S0005) — supplies
  `ProjectionProbe` and the `conformance` feature. AC-008 and the
  `all(feature = "memory", feature = "conformance")` gate are void without it.

Both are `foundation`, both are in-tree, neither is a double. The slice also
transitively depends on `decisions-and-design-record` completing first —
project AC-008's "ADRs accepted *before* the port change lands" ordering is
itself the check — so ADR-0017/0018/0019 are accepted atoms by the time this
story starts, and nothing here writes or amends one.

**Unlocks** —

- **`projection-suite-entry-point`** (HS-S0007) — names this story explicitly in
  its `depends_on`; runs `commit_advances_the_checkpoint` and
  `commit_is_atomic_with_the_read_model` against this store. It is the first
  green projection rule in the repository's history.
- **`projection-mutant-registry`** (HS-S0009) and every rule story after it —
  each mutant is defined as a deviation from what this store does.
- **`buffering-conformant-variant`** (HS-S0011) — the second of project AC-004's
  two structurally unlike batch shapes; this story is the first, and DoD 7's
  "two shapes pass" is unreadable without it.
- **`ps3-batch-shape-finding`** (HS-S0012) — the disagreement it reports is
  measured *between* this store and the buffering variant.

**No cycle.** Every edge above runs forward through `_storymap.md`'s merge order:
slice 1 → this slice → slices 3 – 8.

## Anchors (progressive disclosure)

Open these when the row says to. Everything load-bearing is linked, not pasted —
but nothing load-bearing is optional.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `crates/happenstance-core/src/memory.rs` | The pattern this story ports. `:16-34` gives the three reasons a reference implementation exists (oracle, runnable docs, application code before adapters); `:36-60` is the walkthrough doctest's shape; `:190-199` is poison recovery *and its justification*; `:283-294` is the uninhabited error type's argument and the `Send`-flavour impl. Copying without reading `:16-34` produces a store that compiles and is not an oracle. | Before writing the first line of the new module — and again before the walkthrough doctest. | AC-010, AC-002, EC-004, EC-006 |
| `crates/happenstance-core/src/lib.rs` | The mount, both halves of it, and the anti-pattern in situ: `:98-124` is the export block to extend; `:66-83` is the crate-root prose plus the sentence explaining why `MemoryEventStore` is *deliberately not a link*, which is the exact hazard AC-011 must not re-introduce. | At the mount step, and before touching any crate-root prose. | AC-001, AC-011, EC-008 |
| `crates/happenstance-core/src/projection.rs` | The port being implemented. Read the module doc and `:87` (`#[trait_variant::make(SendProjectionStore: Send)]`) before choosing which flavour to implement, and `:44-64` for `ProjectionId` — including the fact that `new` is unvalidated and is **not** repaired here. Note the file's `type Batch<'a> where Self: 'a` is the *pre*-HS-S0004 state; if it is still there when you start, HS-S0004 has not landed. | First — as the precondition check that the two blocking stories have landed. | AC-002, AC-009 |
| `spec/SPECIFICATION.md` §4 | The signature and behaviour authority, since `_design.md` records no surface. `:4632-4731` is §4.0's trait; `:4643-4658` is why `Checkpoint` is an enum; `:4733-4759` is PS-1 (`[FROZEN]`, atomicity); `:4884-4897` is PS-6 (`begin` infallible); `:4898-4917` is PS-7/PS-8 (drop and rollback); `:5099-5125` is PS-15 (the foreign-batch hazard surviving the owned batch); `:5165-5217` is `reset`; `:4977-5013` is the probe. | Open the specific range as you implement each behaviour; do not read §4 whole. | AC-003, AC-004, AC-005, AC-006, AC-007, AC-008 |
| `spec/SPECIFICATION.md` (PS-34, PS-36) | The two clauses this story's documentation discharges: `:5546-5560` is PS-34 — the `E0195` trap, the doctest rule, and the sentence naming it "the entire explanation for zero adapters"; `:5601-5627` is PS-36 (`[FROZEN]`) — why `type Batch: Send;` is unwritable and why the `compile_fail` annotation must be bare. | Before writing the port-side rustdoc, and before any `compile_fail` doctest. | AC-009 |
| `references/adapter-shapes.md:186-194` | The recorded compiler transcript of the `E0195` failure, from when the GAT was still on the port. It is the "before" against which this story's compile is the "after" — and the reason EC-007 is a halt rather than a workaround. | When writing the impl's `commit`, and immediately if it fails to compile. | AC-009, EC-007 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` | The briefs. Architecture Note 1 (the two-place mount for a library), Note 3 (why the second batch shape lives in the testkit, not downstream), Note 4 (the probe, the `conformance` gate, and the powerset widening), Note 9 (`begin` staying infallible for Neon's one-shot transport); UX brief `:128-148` (AC-U05, AC-U06 — the alternative-that-lost, once, at the item); Testing brief rows AC-012 and AC-009. | Before the mount (Note 1), before the probe gate (Note 4), and before writing rustdoc (UX brief). | AC-001, AC-008, AC-009, AC-011 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` | The signed-off design. Read `:48-50` (`surfaces: []`) and `:92-101` (the sign-off) to confirm the no-surface determination is what a human approved, and `:24-39` for the two non-visual surfaces the Interaction-quality section is written against. Skipping it invites re-deciding a decided thing. | Before the Interaction-quality work — i.e. before writing any rustdoc or the crate-root prose. | AC-009, AC-010, AC-011 |
| `standards/rust/70-rustdoc-obligations.md` | RS-70-5 is the rule behind the density budget: the alternative that lost is named once, at the item. Also the `# Errors` obligation (conditions, not types) that AC-010's review checks. | While writing every doc comment in this PR. | AC-009, AC-010 |
| `standards/rust/62-doctests-and-harnesses.md` | Governs both doctests — what must be asserted rather than merely shown, and when `no_run`/`ignore` is and is not permitted (here: never). | Before writing either doctest. | AC-010 |
| `standards/rust/20-two-flavour-ports.md` | Why `SendProjectionStore` is the impl and `ProjectionStore` is the bound, and why exactly one of the two names is imported per module. Getting this backwards compiles and then makes every downstream generic helper reject the wasm32 flavour. | Before writing the `impl` line and before writing the generic probe test helper. | AC-002, AC-008 |
| `standards/rust/30-error-taxonomy.md` | The adapter error as a type parameter rather than a `String`, and why two error enums stay two. The uninhabited type is an instance of this atom's reasoning, not an exception to it. | When declaring the store's error type. | EC-004, AC-005 |
| `standards/rust/51-features-and-no-std.md`, `standards/rust/52-wasm32-and-target-cfg.md` | The `cfg`/`cfg_attr(docsrs)` pairing, the `all(…)` gate for the probe impl, and what must not enter a module that has to compile for `wasm32-unknown-unknown`. | At the mount step and before the probe impl's gate. | AC-011, NF-003, NF-005, EC-009 |
| `crates/happenstance-testkit/src/lib.rs:112-132` | This workspace's written record of paying for the gated intra-doc-link hazard once already, with the reasoning. Faster than rediscovering it from a rustdoc error. | Before writing any doc link that names a `cfg`-gated item. | AC-011, EC-008 |
| `crates/happenstance-testkit/src/contract.rs:400-403` | The in-tree evidence that `compile_fail,E0308` is the weaker check on 1.97.1. Cite this, not a memory of it, if a reviewer asks for the stricter-looking annotation. | If and when a `compile_fail` doctest is written for PS-36. | AC-009 |
| `xtask/src/main.rs:546-591` | The two powerset steps, with the comments explaining why the wasm32 one exists separately and why neither carries `--locked`. Tells you exactly which command reproduces an AC-011 failure locally. | When an AC-011 check fails, or before claiming AC-011. | AC-011, EC-009, NF-006 |
| `RUNBOOK.md:3848-3965` | Phase 6 in full — this story's bullet is `:3898-3903` (the store, the cold start, "nothing says so, and there is nothing to copy"); `:3893-3897` is the drop-then-reuse scenario and the store that answers `Busy` forever; `:3904-3910` is `reset` and the "position considered, not applied" pair. The plan-of-record framing the ACs are distilled from. | Once, before starting; re-open `:3893-3910` when writing AC-006 and AC-007's tests. | AC-006, AC-007, AC-004 |
| `.kb/decisions/0001-async-port-flavours.md` | The binding constraint behind AC-002: never `#[async_trait]`, because it injects `+ Send` and makes wasm32 impossible. Accepted and immutable. | Before writing the `impl`. | AC-002, NF-003 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:114-181` | Persona 2's journey — specifically step 2, "the compiler accepts a `todo!()` body as readily as a real one", which is the sentence every AC in this story is written from. Also `:42-113` for P1, the walkthrough doctest's reader. | Before writing or reviewing the doctests, when deciding what to explain and to whom. | AC-009, AC-010 |
| `.kb/open-questions/projection-id-is-unvalidated.md` | Names, as deliberately open, the thing that will look like a bug while implementing (`ProjectionId::new` is infallible, unlike `EventType`/`Tag`). Open it so the impulse to fix it is spent in ten seconds rather than in a PR. | The moment `ProjectionId::new` looks wrong. | AC-007 |

## Clarifications resolved during spec

1. **The AC set is exactly the eleven the front half enumerated** — AC-001
   through AC-011, none added, none dropped. AC-002 – AC-008 are behavioural and
   are not separately traced to a project AC; they are what makes this store an
   *oracle* rather than a compiling example, and the coverage note under the
   table says which four rows carry project AC-012.

2. **Project AC-009 is supported here but not owned here.** The Testing brief's
   AC-009 row names "a small standalone test in `happenstance-core`'s own tests,
   gated by `conformance`, against `MemoryProjectionStore`" — which is this
   story's AC-008. Ownership stays with `owned-batch-port-shape` (signature half)
   and `projection-probe-conformance-feature` (seam half) per `_storymap.md`'s
   Coverage table; this story's `traces_to` remains `AC-012` alone, and AC-008
   is the instrument those two stories' AC is discharged *through*.

3. **Test location settled: `crates/happenstance-core/tests/projection_memory.rs`,
   one file.** The Testing brief said "a rule (or a small standalone test in
   `happenstance-core`'s own `#[cfg(test)]`)". An integration test file was
   chosen over an inline `#[cfg(test)] mod tests` for one reason: it can name
   only public items, so it proves the AC-001 mount **by compiling** rather than
   by a separate assertion. Inline tests would pass against an unexported type.
   The probe tests are `#[cfg(feature = "conformance")]` inside the same file
   rather than in a second file, so one `cargo test` invocation covers the story.

4. **Module file name: `crates/happenstance-core/src/memory_projection.rs`.**
   Settled here so the ledger's `verifying_test` paths are real on landing. It is
   a naming choice, not a constraint — if the implementer picks differently, the
   module name changes and the ledger's paths are updated in the same commit.
   What is *not* negotiable is the defensive **spelling in prose**: no page that
   renders without `memory` may intra-doc-link into it (AC-011), which is what
   "named defensively" means in the one-line slice.

5. **`ResetError::Refused` is deliberately unreachable in this store** (EC-005).
   It looked at first like an AC gap. It is not: PS-20's rule is served by a
   hostile store owned by `reset-rules` (HS-S0012), and inventing a refusal
   policy in the oracle would make it a worse oracle. Recorded so the absence
   reads as a decision.

6. **The interaction-quality families are not waived for a library.** The
   temptation, given `surfaces: []`, was to write "N/A — no surface". Instead
   both families are translated onto the two surfaces `_design.md` itself names
   (`:24-39`) — the type surface and the rendered rustdoc — and every invariant
   is carried by a numbered AC row, because a bullet in that section would never
   reach the ledger and would therefore never be gated.

7. **`CHANGELOG.md`'s existing `unstable-projection` paragraph is left alone**
   (`:19-22`). It describes a gate that does not exist yet. That discrepancy is
   real, is pre-existing, and belongs to
   `unstable-projection-gate-and-clause-disposition` (HS-S0016) under AC-014's
   second arm. This story adds only an `[Unreleased] / ### Added` entry;
   correcting the paragraph here would split one decision across two PRs.

8. **`cargo xtask ci --fast` is this story's bar, but AC-011 needs more than
   `--fast` gives.** `--fast` omits the mandatory wasm32 steps, which are exactly
   what AC-011 leans on. Resolved by naming the wasm32 and doc-build commands
   directly in the Tests table so they are run at story grain, while the full
   `cargo xtask ci` remains the *project* boundary's bar at HS-S0017 — the two
   grains stated rather than blurred.
