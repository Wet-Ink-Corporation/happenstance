---
id: kb-decision-0075
title: The projection port's 1.0 clauses — PS-15 narrowed and frozen, PS-23 and PS-24 frozen, PS-38 documented
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0075
reversibility: medium
phase: 17
supersedes: null
superseded_by: null
summary: >-
  Settles the four projection-port clauses phase 16 gave phase 17, none of which changes a
  published signature. PS-15's MUST is narrowed to commit and reset, which are what its rules
  check and what the port can say: rollback returns Self::Error and cannot carry the port-level
  ForeignBatch, and no rule checks it, so rollback leaves the MUST and what is said of it is
  non-normative. That is the route ADR-0066 prescribed. PS-15 is then frozen, because no other
  disposition fits:
  its falsifier, a zero-cost type-level construction that names an instance, would change begin
  and commit if it fired, so ADR-0066 section 2 forbids renewing it past 1.0, and no phase before
  1.0 has an instrument to run it. 1.0's promise is the run-time refusal, whatever a later
  construction finds, and adopting one after 1.0 is a major. PS-23 is frozen: one commit advances
  exactly one ProjectionId, and a multi-id atomic commit, if it is ever wanted, arrives as an
  additive provided commit_all whose default refuses. The provided-method spike showed that route
  additive under trait_variant 0.1.3 with a CommitError::Unsupported variant, on one mechanical
  condition: the default drops the batch before its future exists. This record then chooses a
  separate error type for the refusal, for the reason projection.rs gives for keeping CommitError
  and ResetError apart; that shape was not compiled, and it costs a re-export from happenstance. PS-24 is frozen as a kept
  variant: removing Authority::Rebuilding would be a major, and a swap protocol is additive. PS-38's
  no-lagging-replica obligation is written into ProjectionStore::checkpoint's rustdoc, marked as
  PS-38's provisional reading, and into Neon's README and constructor, and the clause stays
  freeze-by-18.
depends_on:
  - kb-decision-0063
  - kb-decision-0066
  - kb-decision-0028
related:
  - kb-decision-wi-ff17f4
  - kb-decision-0030
  - kb-decision-0072
  - kb-decision-0074
source_paths:
  - experiments/provided-method-spike/README.md
  - crates/happenstance-core/src/projection.rs
  - crates/happenstance-core/src/projection_memory.rs
  - crates/happenstance-neon/src/projection_store.rs
  - crates/happenstance-neon/README.md
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-29
---

# The projection port's 1.0 clauses — PS-15 narrowed and frozen, PS-23 and PS-24 frozen, PS-38 documented

## The question

Phase 16 gave phase 17 four `ProjectionStore` clauses (`runbook/ledgers.md`, *The 1.0
dispositions*): PS-15, PS-23 and PS-24 as `freeze-by-17`, and PS-38's documented obligation
ahead of its `freeze-by-18`. The port is `[FROZEN]` (ADR-0063), so any answer that moves a
signature is a break on `happenstance-core` and five adapters. This record takes the answers that
move none. It is paired with ADR-0074, which settles the typed layer's clauses.

## PS-15 — narrowed to `commit` and `reset`, then frozen

**What was wrong with it.** The MUST named `commit`, `reset` **and** `rollback`, refusing
*"through `CommitError::ForeignBatch` / `ResetError::ForeignBatch`"*. But `rollback` returns
`Result<(), Self::Error>` (`crates/happenstance-core/src/projection.rs:867`), so it cannot return
either port-level variant. No rule checks a rollback leg: `commit_rejects_a_foreign_batch`
checks `commit`, and `reset_clears_rows_and_checkpoint_together` checks `reset`
(`crates/happenstance-testkit/src/projection.rs:896`, `:1201-1204`). ADR-0066 refuted a freeze
on exactly that.

**The narrowing.** The port-level refusal is a MUST on `commit` and `reset` only, which is the
route ADR-0066 prescribed for this clause (`references/adr/0066-what-1-0-promises.md:164`).
`rollback` leaves the MUST. What the specification says of it is **non-normative**: a rollback
handed a foreign batch changes neither store under any in-tree shape, and every adapter but one
refuses it through its own error. It is not a MUST, because a frozen MUST that no rule checks is
the ground ADR-0066 gave for refusing this clause's earlier freeze.
- The breaking alternative was a `RollbackError<E>` on the frozen port. It would touch five
  adapters and the outside-adapter example, to protect nothing durable. A foreign rollback
  changes neither store's durable state under any in-tree shape. A buffered batch is dropped. A
  live batch's transaction belongs to the other store's pool and rolls back on drop (PS-7).
- Every adapter but one already refuses through its own error's `ForeignBatch` variant: SQLite,
  Postgres, live Postgres, Neon and Ladybug. `MemoryProjectionStore` does not check. Adding the
  check there is additive if uniformity is wanted.
- **Making the rollback behaviour normative needs a clause of its own**, carrying a rule (begin on
  one store, roll back on the other, assert both unchanged) and a mutant that mutates a store on a
  foreign rollback. Phase 18 is the next phase that adds projection rules, and its work list
  carries that as an option, not as an obligation of this clause.

**Why freeze, and why nothing else fits.** ADR-0066 §2 allows three dispositions:
- **`renew-past-1.0`** is allowed only where the falsifier firing would be additive or would relax
  an obligation. PS-15's falsifier is a zero-cost type-level construction that names an instance,
  composes with `async fn`, and lets a batch be held in a collection keyed by store. If it fired,
  `begin` and `commit` would carry the instance in their types. That changes a frozen signature,
  which is a **break**, so renewal is forbidden.
- **`freeze-by-18`** needs phase 18 to have an instrument. It has none: phase 18 builds the typed
  runner, not a type-level instance brand. The two constructions tried are in the specification's
  §4.5, a lifetime tie (refuted) and a generative brand (unusable). They are evidence the search
  has been made, not a schedule for another.
- **`outside-1.0`** is false. `commit` and `reset` are on the port 1.0 promises.

So the narrowed clause is **frozen**, on this ground: **1.0 promises the run-time refusal,
whatever a later construction finds.** The falsifier is restated as what it now is, a post-1.0
reopening. A construction found after 1.0 that turns the refusal into a compile error would be a
major, carried by a new record, and the run-time refusal would stay correct until then. This is
not ADR-0066's refuted pattern of calling a falsifier decorative. The falsifier can still
discriminate; what changed is that firing it is a major in either case, and a marker that can
only fire as a major is a `[FROZEN]` clause's reopening condition, not a provisional one's.

**For the owner.** This is not a deviation. ADR-0066 gave PS-15 `freeze-by-17` and named this
narrowing as one of its two routes; the other, a port-level foreign-batch refusal on `rollback`, is
breaking. The phase-17 plan's note to keep PS-15 `[PROVISIONAL]` is not a disposition ADR-0066 §2
allows, for the reasons above.

## PS-23 — exactly one `ProjectionId` per commit, frozen

`commit(&self, batch, id: &ProjectionId, position, authority)`
(`crates/happenstance-core/src/projection.rs:829-835`) is frozen by ADR-0063, and PS-38's text
embeds the one-id commit. The marker's falsifier is a pair of read models in one store that must
be mutually consistent at every observable instant. Norvant's control tower is named as a
candidate, but it is a scenario and not a consumer. The in-tree evidence runs the other way:
`examples/rebuilding-read-models` presents two views at two checkpoints as the arrangement, not as
a defect.

**The decision: "exactly one" is the 1.0 promise.** A multi-id atomic commit, if it ever arrives,
is an **additive provided method**, for example `commit_all`, whose default refuses. It is never
a change to `commit`. A default that calls `commit` twice would be non-atomic, so the default
refuses. The provided-method spike showed the route is additive
(`experiments/provided-method-spike/README.md`, use 2):
- it compiled for every in-tree implementor, Send and `!Send`, host and wasm32;
- it spawned from `S: SendProjectionStore + 'static, S::Batch: Send` with no `Sync`;
- `cargo-semver-checks` 0.50.0 reported nothing against `0.3.2`, and a required-method control
  was reported as major.

What the spike compiled refused through a new `CommitError::Unsupported` variant. It passed on
one mechanical condition, and this record adds one design choice the spike did not compile:
1. **The default drops the batch before its future exists** and returns `async { Err(..) }`.
   Moving `Self::Batch` into the future fails in the derived flavour, because `Batch` carries no
   `Send` bound (`results/commit-all-captures-batch.txt`). rustc's suggested fix, an
   attribute-level `Batch: Send`, is ES-3's trap: it would push the bound onto the bare flavour's
   wasm32 implementors. Dropping bare is a rollback by PS-7, so this is sound.
2. **The refusal gets its own error type, not a `CommitError::Unsupported` variant.** This is
   this record's choice, not a condition the spike tested: its README left it *"recorded, not
   decided"*, and the separate type was never compiled or semver-checked. `CommitError` and `ResetError` are separate so that *"a
   caller matching `commit`'s result never has to consider `Refused`, which `commit` cannot
   produce"* (`crates/happenstance-core/src/projection.rs:588-593`). An `Unsupported` variant on
   `CommitError` would give every `commit` caller a variant `commit` cannot produce, which is the
   same defect in the other direction. The spike's variant also broke an exhaustive in-crate match.
   The type's name and shape are left to whoever adds `commit_all`, and so is compiling it. A new
   public type in `happenstance-core` must also be re-exported from `happenstance`, on its own
   `use` line (spike finding 3: `contract_surface.rs:266`, `doc_surface.rs:139`).

**Restated falsifier:** a read model pair that must be mutually consistent at every observable
instant, **and** that `commit_all` with a refusing default cannot serve. The first alone is
answered additively. Phase 18's fan-out runner (PS-30) is where such a pair would first appear.

## PS-24 — `Authority::Rebuilding` is kept, frozen

`Authority::{Live, Rebuilding}` and `Checkpoint::Rebuilding` are published in
`happenstance-core` 0.3.x (`Checkpoint` at `crates/happenstance-core/src/projection.rs:507`,
`Authority` at `:216`), and
`rebuilding_is_distinguishable_from_live` passes on every adapter. The marker's falsifier is
rebuild-in-place turning out to be always wrong. Its discriminating measurement, a store that
cannot hold two copies of the read model, is unrun.

**The freeze claims only that 1.0 keeps the port able to say "rebuilding".** It does not claim
that rebuild in place is right. Even if the falsifier fired, `Rebuilding` would become unused,
not removed:
- removing a variant from a published enum is a major;
- a swap protocol, if one is needed, is additive: a second `ProjectionId` and an application-level
  swap need nothing new from the port.

**Restated falsifier:** a store that must *refuse* a `Rebuilding` commit to stay correct. Only
that would make keeping the variant wrong rather than unused. **Phase 18 decides whether the typed
runner ever emits it.** Today the runner emits only `Live` and resumes from `Rebuilding` as if it
were `Live`.

## PS-38 — the no-lagging-replica obligation, documented; stays `freeze-by-18`

PS-38's falsifier is a store answering `checkpoint` from a replica that may lag its own
`commit`. The Neon run used one primary endpoint, so the shape was never exercised (ADR-0066).
This record writes the obligation where an implementer and an operator meet it, as documentation
only:
- **`ProjectionStore::checkpoint`'s rustdoc**
  (`crates/happenstance-core/src/projection.rs`), marked as PS-38's provisional obligation: it
  reflects every commit this store has acknowledged, through any handle onto it, and an adapter
  over replicated storage answers from the primary. If PS-38's falsifier fires, the obligation
  narrows to the same handle and that sentence changes with it.
- **`happenstance-neon`'s README and `NeonProjectionStore::new`'s rustdoc**: point the store at a
  read-write primary endpoint, never a read replica. A replica may lag the primary's commits, so a
  runner resuming from a stale checkpoint re-applies events it already committed, and a caller
  reading a checkpoint it just advanced reads it behind.

**This is a documented operator obligation, not a conformance property.** No in-process rule can
observe a lagging replica. A documented obligation is a control that nothing enforces, and the
record says so rather than dress it up. The marker stays `[PROVISIONAL]`, and phase 18 freezes it with
the fan-out runner. PS-23's freeze above makes the one-id text PS-38 embeds stable.

## What changes

The specification's markers and text for PS-15, PS-23 and PS-24 move to `[FROZEN]`, PS-15's MUST
covering `commit` and `reset` only, and PS-38 is annotated. The rustdoc and the README gain PS-38's obligation. No signature changes, and nothing
here needs a `0.4.0` semver row.

## Falsifier

This record is wrong if any of the following happens:
- An adapter must answer `rollback` of a foreign batch by changing a store.
- A consumer's need for a multi-id atomic commit cannot be met by an additive, refusing
  `commit_all`.
- A store must refuse a `Rebuilding` commit to stay correct.

Each one reopens its clause with a new record, and the first two would be majors after 1.0.
