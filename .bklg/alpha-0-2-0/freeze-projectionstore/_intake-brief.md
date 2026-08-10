---
item: HS-P0001
stage: intake
created: 2026-08-10T02:59:44.099Z
updated: 2026-08-10T02:59:44.099Z
template_sig: 56ad54cb
rendered_sig: 4841551a
---

# Intake Brief — Phase 6: Freeze `ProjectionStore`

## Problem

`grep -rn "ProjectionStore for"` matches nothing in this workspace, so the port is
currently shaped like nothing at all. A port frozen by one implementation is shaped
like that implementation; a port frozen by none is a guess with a GAT in it. §4's
clauses carry `[PROVISIONAL]` for exactly this reason, and the port has a cold-start
problem on top: implementing it fails with `E0195` unless you spell the parameter
`Self::Batch<'_>`, nothing says so, and there is nothing to copy.

## Desired Outcome

The port settled against two structurally different implementations, its conformance
suite written and emitted through phase 1's registry so it inherits the tokio,
blocking and wasm flavours, and a reference implementation shipped so the port has an
oracle and a doctest that cannot rot.

## Constraints

- **The GAT goes.** Phase 2's evidence: `type Batch<'a> = rusqlite::Transaction<'a>`
  compiles on the **bare** flavour and fails on `SendProjectionStore` for two
  independent reasons — `Connection` is `Send` but not `Sync`, so `&Self` is not
  `Send`; and `Transaction<'_>` is not `Send`, so the `commit` future cannot be. The
  GAT's stated justification is therefore unearned on the flavour every native
  adapter will implement.
- **Do not claim the owned shape closes the foreign-batch hazard.** That was compiled
  and refuted. A lifetime names a region, not an instance, and two `&Store` references
  unify to a common region; even tying the batch to the receiver's lifetime accepts
  `let b = a.begin(); other.commit(b);`. Only a generative brand rejects it, and PS-15
  stays provisional rather than pretending otherwise.
- **"The skeletons compile unchanged" is unsatisfiable and is not the bar.** This
  phase drops the lifetime, so every skeleton spelling `type Batch<'a>` must change.
  The bar that carries information is narrower: same underlying `Batch` type, same
  error type, same bodies, and only the lifetime parameter's removal differs.
- **Non-goal.** No adapter is finished here. Ladybug is phase 11.

## Open Questions

Each is owned by an ADR this phase writes, and the ADRs come before the code they
constrain:

- What does a batch own, what vocabulary writes into it, and what happens when it is
  dropped? (ADR-0017, PS-4 – PS-15)
- How is a projection returned to "never run", what is that operation's transactional
  scope, and what may refuse it? (ADR-0018, PS-16 – PS-20)
- What happens when `apply` fails? (ADR-0019, PS-26 – PS-30)
- Read-your-writes within a chunk: is the batch read-your-writes, does the runner
  guarantee one event per batch, or may a projection not read what it writes? All
  three are design decisions and E2E-21 forces one. (PS-12)
- Does the port ship at 0.1, or behind an off-by-default `unstable-projection`
  feature with a documented semver exemption — the `tokio_unstable` idiom? The second
  decouples publication from this phase entirely and is the honest option if the two
  batch shapes disagree. (PS-3)

## Proof artefact

**`CheckpointOnlyStore` failing the projection suite**, and two implementations at
opposite ends of the batch-shape axis passing it.

`CheckpointOnlyStore` commits the checkpoint and silently drops the read-model write.
It is this phase's whole bar: **if it passes, the port is not frozen.** A suite that
cannot reject it is decorative by `CLAUDE.md`'s own corollary — and it is reachable
only if the port grows a write seam, because generic suite code holding a `P::Batch`
can otherwise only pass it to `commit` or `rollback`, which leaves three of the six
projection rules unable to observe the read model at all.

The two passing implementations are the phase-2 rusqlite skeleton fleshed out far
enough to commit a real transaction, and `MemoryProjectionStore` behind the `memory`
feature — about 50 lines, and the thing application authors can test against before
any real adapter exists.

## Clauses

Discharges **PS-1 – PS-37**. PS-32, PS-33 and PS-35 leave the clause space entirely.
PS-15 stays `[PROVISIONAL]` deliberately. PS-31 records that outward-writing
projections are out of scope — a documented exclusion is not adapter-checkable, and it
is in the clause space because silence is what produces the wrong implementation.
PS-7 states the Drop contract: dropping a `Batch` without `commit`/`rollback` MUST
roll back **and** MUST release any resource `begin` acquired — a reviewer's probe found
a dropped batch permanently losing the connection, after which the store returned
`Busy` forever, so the rule is not "rolls back", it is "and the store remains usable".

ADR-0007's Context is corrected here (PS-32): a runner that itself writes into the
batch cannot be written today; a callback-driven one can, and was compiled.

## Gate: Intake

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/intake.md` and will not leave `intake` until
every one is ticked.

- [x] The problem and desired outcome are stated.
- [x] Constraints and non-goals are recorded.
- [x] Open questions are captured for distillation.
- [x] The proof artefact is named, and it would not exist if the design were wrong.
- [x] For a port freeze: the axis it is most likely to be wrong about is named, and something in the workspace sits at the other end of it.
- [x] The clauses this work discharges or amends are listed by id.
- [x] Where this brief and `SPECIFICATION.md` disagree, the specification wins.
