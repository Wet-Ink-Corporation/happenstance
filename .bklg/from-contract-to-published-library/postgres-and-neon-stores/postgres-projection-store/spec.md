---
item: HS-S0071
stage: spec
created: 2026-08-12T13:47:09.744Z
updated: 2026-08-12T13:47:09.744Z
template_sig: 87bbf1d0
rendered_sig: 67cd43cd
---

# Spec — PostgresProjectionStore against the frozen Batch

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DoD 7 and DoD 8; the adapter-author persona |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — the ten-project portfolio and its DAG |
| Project | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/project.md` — **AC-005**, the *Depends on* edge to `projection-store-freeze` (HS-P0010), and the risk row "the projection batch shape moves after `PostgresProjectionStore` is written" |
| This spec | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/postgres-projection-store/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md` — *Architecture brief* §1 (the seams, and `happenstance-core/**` as must-not-change), §2 Root B (the conformance macros are the mount), §7 (gate and CI wiring), §8 point 4 ("**AC-005 last**, gated on HS-P0010"); *Testing brief* AC-005 row (`:515`) |
| Upstream contract owner | `.bklg/from-contract-to-published-library/projection-store-freeze/_storymap.md` — the seventeen stories that land the port, the probe, the fixture trait and the suite this story is run by; `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md:575-600` — the five-row signature-change table |
| Signed-off design | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_design.md` — **N/A by sign-off**: this project records no user-facing surface, approved 2026-08-12 by the repository owner. This story renders none. |
| Story map row | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_storymap.md` — slice `postgres-projection-store`, the single-story slice; and its longer paragraph at *"What each story is, in slightly more than one line"* |
| Roadmap pointer | `RUNBOOK.md:4376` — phase 10's first exit criterion, "**Four** macros green against a real Postgres". This story is the fourth. |

## One-line PR slice

`PostgresProjectionStore` is written against the `Batch` shape
`projection-store-freeze` freezes and passes the projection suite in the live
Postgres job, with any declined capability reported by name and reason.

## Executive summary

**Lands:** the first *shipping* `ProjectionStore` adapter — a real SQL store,
against a real server, run by the suite that was written to fail it.

Today `crates/happenstance-postgres/src/projection_store.rs` is four `todo!()`
bodies under a module doc that argues, at length and correctly, why the batch
can be owned (`:1-63`). Nothing runs it: there is no projection rule anywhere in
`crates/happenstance-testkit/src/suite.rs` — all 89 rules there are written
against `Fixture::Store: EventStore`, which `spec/SPECIFICATION.md:4636-4638`
states in terms. This PR is where the four bodies become real and where the
seventeen new rules `spec/SPECIFICATION.md:5652-5676` oblige meet a Postgres.

**Delta against the project charter.** `project.md` AC-005 is the sole AC this
story traces to, and it is the one AC in the project written against a freeze
that lands **outside** the project: "against the owned `Batch` frozen by
`projection-store-freeze`, with any declined capability reported by name and
reason." So the deliverable has an unusual precondition and this spec treats it
as a first-class one — see Context pack §1. Writing the adapter early against an
unfrozen `Batch` and reworking it is precisely the failure `project.md`'s risk
table names.

**Delta against the story map.** The map's paragraph for this story says the
shape "is already recorded as `type Batch<'a> = sqlx::Transaction<'static,
Postgres>`" and cites `references/adapter-shapes.md`, where that binding is
recorded as **accepted** evidence for PS-5 (`:160-168`). That record was made by
a skeleton, and a `todo!()` body type-checks against any signature
(`RUNBOOK.md:3061-3068`). The frozen port makes `begin` neither `async` nor
fallible (`spec/SPECIFICATION.md:4692-4694`), and a `sqlx::Transaction` cannot
be obtained from a pool without awaiting. **This story is where the type checker
gets to disagree with the record**, and the delta is that resolving that
disagreement — and writing down which way it went — is in scope here rather
than assumed away. See Context pack §2 and §10.

**Not in the delta.** No new conformance rule, no port change, no `[FROZEN]`
clause edit, no ADR-0024 content, and no de-skeletoning of the crate.

## Context pack

The load-bearing decisions, stated as decisions. Everything deeper is a
signposted anchor; nothing here needs an anchor opened before work starts.

### 1. The port this is written against is not the port in the tree today

`crates/happenstance-core/src/projection.rs` as it stands is the **provisional**
shape (`:1-11` say so). The shape this adapter implements is
`spec/SPECIFICATION.md:4632-4731`, landed by HS-P0010's `owned-batch-port-shape`
story. Five things change, not one
(`.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md:586-590`):

| Today (`projection.rs:87-139`) | The shape this story implements | Clause |
| --- | --- | --- |
| `type Batch<'a> where Self: 'a` | `type Batch;` | PS-5 |
| `async fn begin(&self) -> Result<Batch, E>` | `fn begin(&self) -> Self::Batch` | PS-6 |
| `checkpoint -> Option<SequencePosition>` | `checkpoint -> Checkpoint` (`NeverRun` / `Live { through }` / `Rebuilding { through }`) | PS-19, PS-24 |
| `commit(batch, id, position) -> Result<(), E>` | `commit(batch, id, position, Authority) -> Result<(), CommitError<E>>` | PS-15, PS-21, PS-22, PS-24 |
| — | `reset(batch, id) -> Result<(), ResetError<E>>` | PS-16 – PS-18 |

**This is a hard precondition, not a soft one.** `projection-store-freeze`
(HS-P0010) is at `stage: storymap` at the time of writing
(`.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md:182-183`),
and `project.md`'s *Depends on* names it as the supplier of three separate
things this story consumes: the frozen port, the projection suite, and the
capability-declension policy the skips are reported under. If any of the three
is absent from the tree when this story starts, **halt and report** — do not
stub the missing half, do not invent a capability name, and do not write the
adapter against today's provisional signatures on the theory that it will be
easy to migrate. The migration is five signature changes and one storage-model
change; it is not easy.

### 2. `begin` is synchronous and infallible, and that is where the recorded shape stops working

The frozen `begin` is `fn begin(&self) -> Self::Batch` — no `async`, no
`Result` — and the port's own rustdoc must say why: *"opening a buffer cannot
fail, and an adapter that needs a round trip takes it at `commit`"*
(`spec/SPECIFICATION.md:4692-4694`). PS-6 carries a `[PROVISIONAL]` marker whose
falsifier is an adapter that must reserve something from the server before the
first write, and it is **owned by the Neon phase**, not by this story
(`spec/SPECIFICATION.md:4884-4897`).

`sqlx::PgPool::begin` is `async fn` and returns `Result` — the crate's own module
doc quotes the signature (`crates/happenstance-postgres/src/projection_store.rs:15-23`).
So `Self::Batch = Transaction<'static, Postgres>` cannot be *produced* by the
frozen `begin`, whatever the skeleton's binding says. The expected resolution is
the one `spec/E2E-CASES.md:622-651` (E2E-24) describes and PS-4 permits: **the
batch is a deferred write set, and the transaction is opened at `commit`.**
That is the shape the port was reshaped for, and the correction folded into
E2E-24 says it in terms — *"the whole write set must be buffered and emitted as
one statement at commit."*

Three moves are forbidden, each for a stated reason:

- **Do not block on the runtime inside `begin`.** `futures::executor::block_on`
  or `Handle::block_on` inside a `fn` that a tokio worker thread calls is a
  panic on a current-thread runtime and a deadlock risk on a multi-thread one.
  It also puts a network round trip inside a call the port has just promised
  cannot fail — the promise being what makes the signature legal at all.
- **Do not "improve" `begin` back to `async fn … -> Result<…>` for symmetry with
  `EventStore`.** It is named as a hazard by the upstream brief in exactly those
  words
  (`.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md:702-706`),
  and the port is `happenstance-core`'s, which this project must not change
  (*Architecture brief* §1).
- **Do not lazily acquire a connection on first write and call it a buffer.** If
  the first `probe_write` awaits a checkout, `commit` no longer owns the only
  round trip and the atomicity argument in §3 moves to a place nothing tests.

If the implementer finds a construction that keeps a live `Transaction` under a
synchronous infallible `begin`, that is a **finding**, not a licence: record it
per §10 and take it to HS-P0010 rather than reopening PS-6 here.

### 3. The invariant is one transaction, and `commit` is where it is bought

A read-model write and its checkpoint write become durable together or not at
all (PS-1, `[FROZEN]`, `spec/SPECIFICATION.md:4733-4759`). Under a deferred
write set that means: `commit` opens one transaction, replays the batch's own
statements into it, performs the checkpoint `UPSERT` on the same transaction,
and commits **once**. The crate's own module doc already describes the shape
correctly for the transaction-per-commit reading
(`crates/happenstance-postgres/src/projection_store.rs:44-51`) — the sentence
survives the change of when the transaction is opened.

Two rules watch this and they are differential against each other:
`commit_advances_the_checkpoint` is the baseline, and
`commit_is_atomic_with_the_read_model` reads the row and the checkpoint **back
through fresh handles**, so a store that satisfies one connection's view and not
another's is caught (`spec/SPECIFICATION.md:5661-5662`, the two rule rows). `CheckpointOnlyStore`
is the store that fails the second and passes the first, and it is the bar
initiative DoD 7 is stated in terms of.

### 4. The foreign-batch check is a run-time stamp, and the fix everyone reaches for first does not work

PS-15 requires `commit`, `reset` and `rollback` to reject a batch begun on a
*different instance of the same store type*, through
`CommitError::ForeignBatch` / `ResetError::ForeignBatch`, leaving both stores
unchanged (`spec/SPECIFICATION.md:5126-5145`). Do not reach for a lifetime:
tying the batch to the receiver was **compiled and refuted** — a lifetime names
a region of the program, not an instance, and two `&Store` references unify to a
common region without complaint (`spec/SPECIFICATION.md:5099-5124`). The only
type-level construction that names an instance is a generative brand, which
forces `store.with_batch(|batch| …)` and forbids the batch escaping the closure —
defeating the caller the hazard is about.

So: `begin` stamps an identity minted per store instance, `commit`/`reset`
compare, and with an owned batch that stamp is a field and the check is an
integer comparison
(`.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md:630-640`).

**One decision this story cannot dodge:** `PostgresProjectionStore` is
`#[derive(Clone)]` today (`projection_store.rs:72-76`) and a fixture hands out
several handles onto **one** backing store. A clone that mints a fresh stamp and
a clone that copies its parent's are observably different implementations, and
`commit_rejects_a_foreign_batch` is the arbiter. Decide which one "a different
instance" means for a pooled, cloneable handle, and write the reason where an
implementer meets it — not only in the ledger.

### 5. `Checkpoint` is three variants because two of the four tuple states are nonsense

`checkpoint` returns `Checkpoint`, not `Option<SequencePosition>`, because
`(Option<SequencePosition>, bool)` can spell `(None, true)` — *"authoritative,
never run — which means nothing"* (`spec/SPECIFICATION.md:4645-4660`).
`Authority::Live` / `Authority::Rebuilding` is what `commit` passes to say which
one it is claiming.

The storage consequence is the one to get right: **`Rebuilding` must survive a
process restart**, because the reader who needs it is the one who arrives after
the rebuilding process died (E2E-25, `spec/E2E-CASES.md:653-680`). It is a
stored column on the checkpoint row, never an in-memory flag on the store
handle. `rebuilding_is_distinguishable_from_live` rejects "rebuild in place with
one position field" (`spec/SPECIFICATION.md:5676`).

### 6. A declined capability is a reported skip in the fixture's own words — never an absent test

CF-18's rule, and it is the reason `RuleOutcome` exists: a rule absent from the
binary is indistinguishable in CI output from a rule that passed. The skip line
is ``SKIP {rule}: fixture declines `{capability}` — {reason}``, produced by
`RuleOutcome::skip_line` and printed by `report`
(`crates/happenstance-testkit/src/contract.rs:495-537`). Under libtest a
*passing* test's stdout is
suppressed unless `--show-output` is passed, which is why the live job passes it.

**The projection capability set is HS-P0010's to fix, not this story's to
invent.** `projection-capability-skips` states that the set is "fixed by DT-3's
recorded resolution rather than invented here", and DT-3 is resolved in that
project's `_design.md` against
`.kb/open-questions/cf-40-fixture-limits-ownership.md` rather than by minting a
second declension policy. Consume the names that exist; if a capability this
adapter needs to decline has no name upstream, that is a report, not a local
constant.

The one gate this story genuinely answers with a fact rather than a policy is
`ProjectionProbe::READS_THROUGH_BATCH` (PS-12, `spec/SPECIFICATION.md:5052-5074`):
declaring `true` obliges the probe's read to see a pending write through the
batch, and `batch_reads_reflect_pending_writes` rejects *"the buffering adapter
that offers a `get` answering from the committed table"* — which is the natural
shape and the one that silently loses a write. Under §2's deferred write set,
`true` is earned by consulting the buffer before the table, and `false` is an
honest answer with a stated reason. Both are admissible; a `true` that reads
past the buffer is not.

### 7. `ProjectionProbe` is how the suite sees the read model, and it costs one feature flag

The suite cannot look at a read model it does not know the shape of, so
`happenstance-core` grows `ProjectionProbe` (`READS_THROUGH_BATCH`,
`probe_write`, `probe_delete_all`, `probe_read`, `probe_read_through`) behind a
new `conformance` feature, landed by HS-P0010's
`projection-probe-conformance-feature`. The whole point of putting it in the
contract crate is that an adapter author pays **one feature flag on a dependency
they already have, and no new edge in the graph**
(`spec/SPECIFICATION.md:5016-5031`).

For this crate that means a real `conformance` feature in
`crates/happenstance-postgres/Cargo.toml` forwarding to
`happenstance-core/conformance`, mounted in the feature table beside
`event-store` and `projection-store` (`:28-34`) — not a `#[cfg(test)]` block.
`cargo hack`'s feature powerset runs in the gate and will take the new
combinations, so `conformance` without `projection-store` must compile, as must
`projection-store` without `conformance`.

### 8. The gate stays Docker-free and network-free; the live job is inherited, not duplicated

DR-9 and project AC-011. `cargo test --workspace --all-features` must exit zero
on a machine with no server, so the projection tests target carries the same
**whole-invocation** gating the slice upstream chose — `#[ignore]`, a
`required-features` gate, or an env read. The one hard constraint is DR-5's: it
may **not** be a `#[cfg]` hiding a rule out of the macro's expansion. Gating an
invocation on infrastructure availability and making one rule vanish are
different acts and only the second is forbidden (*Architecture brief* §7).

The live-Postgres job already exists by the time this story runs — created by
`postgres-schema-and-live-fixture` as a sibling of `gate` in
`.github/workflows/ci.yml` (`:31`). This story **extends** it to run the
projection target. It does not add a third job, and it inherits that story's
assertion that the job cannot be green having executed no tests.

### 9. Never assert a literal position, and the `bigint` gap already has an error variant

CF-6: the specification permits gaps and `cargo xtask lint-position-literals`
runs in the default gate (`xtask/src/main.rs:389,687`). Compare against
positions the store actually assigned.

The domain gap between storage and contract is already modelled and must be
used, not re-invented: `SequencePosition` is a `NonZeroU64` and a checkpoint
column is a signed `bigint` that admits `0` and negatives, so a row outside the
domain is `PostgresProjectionStoreError::CheckpointOutOfRange { value }` —
already declared, still unconstructed
(`crates/happenstance-postgres/src/error.rs:60-80`). A `panic!`, an `unwrap`, or
a silent clamp in a library is the wrong answer to a row a real database can
hand back.

### 10. What contradicts the record gets reported, not absorbed

Two in-tree records will be under pressure from this story's real bodies:
`references/adapter-shapes.md:160-168` (the `Transaction<'static, Postgres>`
binding, recorded as **accepted** evidence that a pooled networked driver did not
need a borrowed batch) and `RUNBOOK.md:1447-1451` (the same claim in the phase-2
session log). Both were written by a skeleton.

Project DoD 6 already asks for exactly this: *"`references/adapter-shapes.md`
and `RUNBOOK.md`'s phase-10 session log record what the two adapters told the
type checker and the server that the skeletons could not, **including anything
that contradicts a prior assumption**"* (`project.md`, *Definition of done*). So
the finding is a deliverable of this story, in those two files, and it is
additive — the existing text is corrected in place only where it states
something now known to be false, with what actually happened written beside it.

What the finding is **not**: a licence to edit a `[FROZEN]` PS clause, to change
`happenstance-core`, or to quietly re-open PS-5. If the evidence for PS-5 is
weaker than recorded, that is input to HS-P0010's `ps3-batch-shape-finding` and
to `publication-and-positioning`'s clause audit — a re-plan input, not a fix
taken here.

### 11. The persona and the journey this realises

Persona 2, the adapter author
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:114-152`).
Their fear is a contract defect discovered while implementing a port. The slice
they get here is narrow and specific: *the projection port, having been frozen
against an in-memory oracle and a testkit-internal buffering variant, is run for
the first time by an adapter that talks to a server over a pool* — and whatever
it disagrees about is written down where the next author reads it, instead of
being discovered again.

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `capability` — user-observable in the only medium this repository has: a conformance family that runs and reports against a real server, and a public adapter type a caller meets (`_storymap.md`, preamble). |
| **Slice / milestone** | `postgres-projection-store`. **Slice-mates: none** — it is the only story in its slice, deliberately, so it can slide right against HS-P0010 without moving anything else (`_storymap.md`, *Merge order* step 5). |
| **Mount point** | `crates/happenstance-postgres/tests/postgres_projection_conformance.rs` (**new**) — Root B of *Architecture brief* §2: the file that invokes `projection_store_conformance!(<fixture expr>)`. The macro takes an **expression building a fixture**, not a type, precisely because "a Postgres fixture needs a connection URL, and a type with an argument-less constructor would have to reach into the environment for it" (`crates/happenstance-testkit/src/lib.rs:270-274`, stated for the event-store macro and inherited by its projection sibling). Second mount, inseparable from the first: the live-Postgres job in `.github/workflows/ci.yml`, **extended** to run this target. A macro no workflow runs is not delivered. |
| **Wires into** | The frozen port and its four new types — `ProjectionStore` / `SendProjectionStore`, `Checkpoint`, `Authority`, `CommitError`, `ResetError` — re-exported from `crates/happenstance-core/src/lib.rs:116` (`ProjectionId, ProjectionStore, SendProjectionStore` today; the four new names land beside them via HS-P0010's `owned-batch-port-shape`). `ProjectionProbe` behind `happenstance-core`'s `conformance` feature. `ProjectionFixture`, `Capability` and `RuleOutcome` in `crates/happenstance-testkit/src/contract.rs` (`Capability`, `RuleOutcome`, `NO_CEILING_REASON`, `NO_STORE_LIMITS` are exported at `crates/happenstance-testkit/src/lib.rs:186`). `projection_store_conformance!` and `for_each_projection_store_rule!` from the testkit. `PostgresProjectionStore::new(PgPool)` and `pool()` (`crates/happenstance-postgres/src/projection_store.rs:78-92`). `PostgresProjectionStoreError` (`crates/happenstance-postgres/src/error.rs:60-80`). Migration 1 and the container harness from `postgres-schema-and-live-fixture`. `MemoryProjectionStore` as the oracle to read, **never to import** — no adapter depends on another adapter, and the reference store lives in `happenstance-core` behind `memory`. |
| **Renders surfaces** | **none.** `_design.md` records this project as having no user-facing surface, signed off 2026-08-12; its `## Items` and `## Signatures` blocks are `N/A`. There is no surface id to claim, and `design.capture` is deliberately absent from `.redkiln/config.yaml`, which makes the perceptual review a declared skip rather than a silent pass. |
| **Conformance rule(s)** | This story adds **no rule** — `crates/happenstance-testkit/**` is a must-not-change seam here (*Architecture brief* §1) and every projection rule is authored by HS-P0010. What it does is make all seventeen of them *observe a Postgres*: `fresh_projection_has_no_checkpoint`, `commit_advances_the_checkpoint`, `commit_is_atomic_with_the_read_model`, `failed_commit_leaves_both_unchanged`, `rollback_leaves_both_unchanged`, `dropped_batch_leaves_store_usable`, `distinct_projections_advance_independently`, `commit_accepts_a_position_the_batch_did_not_write`, `commit_rejects_a_regressing_position`, `commit_rejects_a_foreign_batch`, `reset_clears_rows_and_checkpoint_together`, `reset_is_scoped_to_one_projection`, `reset_is_not_commit_at_first`, `refused_reset_changes_nothing`, `batch_reads_reflect_pending_writes`, `rebuild_is_chunk_size_invariant`, `rebuilding_is_distinguishable_from_live` (`spec/SPECIFICATION.md:5660-5676`). Two are this adapter's characteristic hazards rather than generic ones: `dropped_batch_leaves_store_usable`, whose named rejection is *"a pooled connection `Drop` never returns"*, and `batch_reads_reflect_pending_writes`, whose named rejection is the buffering `get` that answers from the committed table. |
| **Clause(s)** | Discharges none unilaterally; **exercises** PS-1, PS-4, PS-7, PS-8, PS-11 – PS-24 and PS-36 against a networked pooled adapter for the first time, and supplies evidence for PS-5's and PS-6's provisional markers without disposing of either — disposition is HS-P0010's (`unstable-projection-gate-and-clause-disposition`) and the Neon phase's respectively. **Amends nothing.** No `[FROZEN]` clause is touched, so **no ADR is owed by this story**. If one appears to be, that is the halt condition in Context pack §10. |
| **Advances DoD scenario** | Initiative **DoD 8** — "the freeze verdict is written… with what it was checked against" (`initiative.md:380-382`). This story does not write the verdict; `ladybug-projection-store` does (`project.md`, *Out of scope*). What it contributes is one of the implementations the verdict is checked against, and the only one with a pool and a server behind it. It also strengthens **DoD 7**'s claim by adding a third structurally distinct batch shape beyond the two HS-P0010 ships in-repo. It observes neither on its own, and this spec does not claim it does. |

## PR boundary

**In this PR**

- Real bodies for `checkpoint`, `begin`, `commit`, `reset` and `rollback` in
  `crates/happenstance-postgres/src/projection_store.rs`, against the frozen
  port, with the `Batch` shape decided and its reason written where an
  implementer meets it.
- The per-instance stamp that discharges PS-15, and the `Clone` semantics
  decision it forces (Context pack §4).
- `projection_checkpoint` added to migration 1 (Data and migrations, below), plus
  whatever the conformance probe's read model needs — kept out of a consumer's
  schema.
- `ProjectionProbe` implemented for `PostgresProjectionStore` behind a new
  `conformance` feature in `crates/happenstance-postgres/Cargo.toml`, forwarding
  to `happenstance-core/conformance`.
- The projection fixture (a `ProjectionFixture` impl, its capability constants
  answered deliberately, `connect`-equivalent handing out an owning handle onto
  one isolated backing store) built on the container harness
  `postgres-schema-and-live-fixture` landed.
- `crates/happenstance-postgres/tests/postgres_projection_conformance.rs` — the
  invocation of `projection_store_conformance!`, with whole-invocation gating.
- The `.github/workflows/ci.yml` live-Postgres job **extended** to run the new
  target with `--show-output`.
- The finding recorded in `references/adapter-shapes.md` and `RUNBOOK.md`'s
  phase-10 session log (Context pack §10).
- This story's own backlog folder (`spec.md`, `_ledger.md`).

**Explicitly not in this PR**

- Any change to `crates/happenstance-core/**` or `crates/happenstance-testkit/**`.
  The port, the probe, the fixture trait, the macro, the seventeen rules and the
  hostile stores are all HS-P0010's. A rule that seems wrong is reported, not
  edited here.
- `crates/happenstance-postgres/src/event_store.rs`, `read_stream.rs`, the
  append-condition SQL, ADR-0024 and its mechanism column. The event-store slice
  owns all of it.
- Removing `publish = false`, removing the scoped `#![allow(clippy::todo)]`
  (`crates/happenstance-postgres/src/lib.rs:58-63`), or growing
  `xtask/src/package.rs`'s `PUBLISHABLE` — `deskeleton-and-package-readiness`
  owns the whole de-skeletoning move, and it depends on this story rather than
  the other way round.
- `crates/happenstance-neon/**`, including `NeonProjectionStore`. Not in the
  project's story map at all.
- A projection **runner**, a `Projection` trait, or any of the six integration-level
  rules CF-36 puts in the workspace e2e crate
  (`spec/SPECIFICATION.md:5694-5703`).
- Any freeze verdict on `ProjectionStore`, and any disposition of PS-5 or PS-6.
- Widening `deny.toml`'s licence allowlist. If a new dependency is needed at all
  — none is expected — that is a finding to report.

The implementer MAY touch the composition-root and wiring files named in the
Integration contract — the new tests target, `crates/happenstance-postgres/Cargo.toml`,
the migration source and `.github/workflows/ci.yml` — to mount this slice. That
is the mount, not scope drift.

**Merge DoD.** `projection_store_conformance!` runs to completion against a live
pinned Postgres with all seventeen rules present and reporting, `cargo xtask ci
--fast` is green on a checkout with the Docker daemon stopped and the network
unplugged, and `crates/happenstance-postgres/src/projection_store.rs` holds no
`todo!()`.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The suite is invoked from the crate's own `tests/`** | `projection_store_conformance!(<expression building the fixture>)`, one file, one invocation, all seventeen rules emitted. Not a hand-picked subset: "no rule is absent from the run" is otherwise unproven by construction. | `crates/happenstance-testkit/src/lib.rs:265-276` (the expression-not-a-type rationale, stated for the sibling macro); `spec/SPECIFICATION.md:5652-5676` |
| **`begin` is synchronous and infallible, and returns an owned batch** | `fn begin(&self) -> Self::Batch`. `type Batch;` carries no lifetime, so `where Self: 'a` disappears from the impl and `Self::Batch` is written without one. | `spec/SPECIFICATION.md:4690-4694`; the five-row change table at `.bklg/…/projection-store-freeze/_decomposition.md:586-590` |
| **The batch shape is decided here, and the decision is written down** | The deferred write set is the expected arm (E2E-24's shape, PS-4's permission). Whatever is chosen, the module doc says which alternative lost and why — the existing `:1-63` argument for an owned `Transaction` is rewritten, not left standing beside a body that contradicts it. | `spec/E2E-CASES.md:622-651`; `crates/happenstance-postgres/src/projection_store.rs:1-63`; `standards/rust/70-rustdoc-obligations.md` (name the alternative at the call site) |
| **`Batch: Send`, so `SendProjectionStore` is the flavour implemented** | The batch is captured by `commit`'s and `rollback`'s futures, which `trait_variant` marks `Send`; a `!Send` batch fails **at the adapter**, not at the port. `type Batch: Send;` cannot be written on the port — the attribute copies bounds verbatim into the bare flavour and would break wasm32 — so this is the adapter's obligation to honour, and the bare flavour comes free. | `spec/SPECIFICATION.md:5601-5627` (PS-36); the existing `send_flavour_satisfies_the_bare_bound` test, `crates/happenstance-postgres/src/projection_store.rs:135-141` |
| **`commit` is one transaction and one commit** | Open the transaction, replay the batch, `UPSERT` the checkpoint, commit once. A failed commit leaves the store unchanged; the batch is consumed either way. | `spec/SPECIFICATION.md:4733-4759` (PS-1); `crates/happenstance-postgres/src/projection_store.rs:44-51` |
| **The checkpoint `UPSERT` is conditional, and reports what it refused** | `commit_rejects_a_regressing_position` rejects the unconditional `UPDATE checkpoint SET position = ?`. `CommitError::CheckpointRegression { current, attempted }` carries the current value, so the statement must return it — `ON CONFLICT … DO UPDATE … WHERE` with a `RETURNING`, or a read in the same transaction. Reading it on a second connection is the defect `commit_is_atomic_with_the_read_model` exists to find. | `spec/SPECIFICATION.md:4668-4675` (`CommitError`), `:5297-5316` (PS-22), `:5668` |
| **`commit` does not validate the position against the batch** | `commit_accepts_a_position_the_batch_did_not_write` rejects an adapter that validates it. A projection that consumed events and wrote nothing still advances. `ValidatingCommitStore` is the mutant. | `spec/SPECIFICATION.md:5277-5296` (PS-21), `:5667`, `:5678-5685` |
| **A foreign batch is refused at run time and changes nothing** | Per-instance stamp minted with the store, compared in `commit`, `reset` and `rollback`; `CommitError::ForeignBatch` / `ResetError::ForeignBatch`. Both stores unchanged after the refusal. Not a lifetime — that fix was compiled and refuted. | `spec/SPECIFICATION.md:5099-5145` (§4.5 and PS-15); `.bklg/…/projection-store-freeze/_decomposition.md:630-640` |
| **`reset` clears the caller's rows and the checkpoint in one unit, scoped to one id** | The caller's batch carries the deletes because the port has no idea what the read model is; the adapter's half is deleting *that* projection's checkpoint row and committing once. `reset_is_scoped_to_one_projection` rejects a checkpoint-table truncate — `TruncatingResetStore` is the mutant. | `spec/SPECIFICATION.md:5165-5199` (PS-16, PS-17), `:5670-5671`, `:5678-5685` |
| **Whether this store ever refuses a reset is answered, not defaulted** | `ResetError::Refused` exists so a store *can* decline; a Postgres adapter with no policy to protect plausibly never does, and `refused_reset_changes_nothing` then reports as the suite's declension policy directs rather than passing vacuously. Say which, and why, where a caller reads it. | `spec/SPECIFICATION.md:4677-4683` (`ResetError`), `:5200-5217` (PS-18), `:5673` |
| **`checkpoint` distinguishes never-run from rebuilding from live** | Three variants, and `Rebuilding` survives a restart because it is a column. `fresh_projection_has_no_checkpoint` rejects `Live { through: FIRST }` for an unseen id; `reset_is_not_commit_at_first` rejects `checkpoint.unwrap_or(FIRST)` with an inclusive `from`; `rebuilding_is_distinguishable_from_live` rejects one position field. | `spec/SPECIFICATION.md:4645-4660` (`Checkpoint`), `:5218-5249` (PS-19), `:5330-5352` (PS-24), `:5660`, `:5672`, `:5676`; `spec/E2E-CASES.md:653-680` (E2E-25) |
| **Distinct projections advance independently** | A checkpoint row per `ProjectionId`, primary-keyed on it. `distinct_projections_advance_independently` rejects a single-row checkpoint table. Note `ProjectionId::new` is **infallible and accepts the empty string** — that is a recorded open question, not a licence to reject it here. | `spec/SPECIFICATION.md:5317-5329` (PS-23), `:5666`; `crates/happenstance-core/src/projection.rs:44-64`; `.kb/open-questions/projection-id-is-unvalidated.md` |
| **A dropped batch rolls back and leaves the store usable** | `dropped_batch_leaves_store_usable` drops a batch bare, then opens and commits a second one. Its named rejection is this adapter's family: *"a pooled connection `Drop` never returns"*. Under a deferred write set nothing is checked out at drop time, which is an argument for the shape rather than an excuse to skip the rule. | `spec/SPECIFICATION.md:4898-4910` (PS-7), `:5665` |
| **`rollback` exists because `Drop` cannot await** | It is the place a batch holding a real resource releases it. It must also reject a foreign batch (PS-15 names all three methods). | `spec/SPECIFICATION.md:4716-4718`; `:4911-4946` (PS-8), `:5664` |
| **`ProjectionProbe` is implemented behind a `conformance` feature** | `READS_THROUGH_BATCH`, `probe_write`, `probe_delete_all`, `probe_read`, `probe_read_through`, forwarding to `happenstance-core/conformance`. One feature flag on a dependency the crate already has; no new edge in the graph. `cargo hack`'s powerset takes the new combinations. | `spec/SPECIFICATION.md:5016-5031`; `crates/happenstance-postgres/Cargo.toml:28-34`; `crates/happenstance-core/Cargo.toml:34-52` (the `memory` / `serde` feature pattern to copy) |
| **`READS_THROUGH_BATCH` is answered as a fact about the chosen shape** | `true` obliges the probe read to see a pending write *through the batch*; `false` is honest and emits `batch_reads_reflect_pending_writes` as a **reported skip** with a reason. What is not admissible is `true` served from the committed table. | `spec/SPECIFICATION.md:5052-5074` (PS-12), `:5674` |
| **`rebuild_is_chunk_size_invariant` runs at 1, 3 and whole-log** | The probe's write is defined as an increment of what the batch can see, so a store violating PS-12 diverges. Nothing in the adapter may read the read model out of band. | `spec/SPECIFICATION.md:5075-5098` (PS-13, PS-14), `:5675` |
| **Every declined capability prints its own reason** | `RuleOutcome::Skipped { capability, reason }` → ``SKIP {rule}: fixture declines `{capability}` — {reason}``. libtest suppresses passing-test stdout without `--show-output`, so the live job passes it. Assertions about skips are on `RuleOutcome` **values**, never on stdout. | `crates/happenstance-testkit/src/contract.rs:495-535` |
| **Capability constants are checked at codegen, not by clippy** | `Capability::declined("")` fires an `assert!` in a `const fn`, and for an *associated* const that lands at codegen: `cargo build` and `cargo test` catch it, `cargo clippy` does not. A green clippy proves nothing about the constants. | `crates/happenstance-testkit/src/contract.rs:404-410` |
| **One fixture instance is one isolated backing store** | CLAUDE.md's fixture rule, and the same per-instance isolation scheme `postgres-schema-and-live-fixture` chose (schema-per-instance or database-per-instance). Reuse it; do not invent a second one for projections. | `.bklg/…/postgres-schema-and-live-fixture/spec.md` *Behavior and interfaces*, "Per-instance isolation"; `crates/happenstance-testkit/src/fixtures.rs:252-267` |
| **The gating is whole-invocation** | `#[ignore]`, `required-features` or an env read — never a `#[cfg]` hiding a rule out of the expansion (DR-5). `cargo test --workspace --all-features` exits zero with no server, and that command is inside `cargo xtask ci --fast`. | *Architecture brief* §7; `.redkiln/config.yaml:55` |
| **A row outside the contract's domain is an error variant, not a panic** | `bigint` is signed and admits zero; `SequencePosition` is a `NonZeroU64`. `PostgresProjectionStoreError::CheckpointOutOfRange { value }` is already declared and is the intended answer — a library does not panic on a row a real database can hand back. | `crates/happenstance-postgres/src/error.rs:60-80` |
| **No literal position values** | CF-6, enforced in the default gate by `cargo xtask lint-position-literals`. Compare against positions the store actually assigned. | `xtask/src/main.rs:389,687`; CLAUDE.md *The rule that matters* |
| **Errors are documented by condition, not by type** | Every fallible public function carries an `# Errors` section naming the conditions. `CommitError` and `ResetError` stay two enums so a `commit` caller never matches `Refused`, which `commit` cannot produce. | `spec/SPECIFICATION.md:4722-4727`; `standards/rust/30-error-taxonomy.md` |
| **`error[E0195]` is the trap, and it survives the GAT's removal for `EventStore`-shaped impls** | Writing the concrete batch type in the impl instead of the trait's spelling is `error[E0195]: lifetime parameters or bounds on method 'commit' do not match the trait declaration`. Under `type Batch;` the lifetime is gone from the projection port, so the trap should not fire here — if it does, that is a finding about the port change, not a local workaround. | `references/adapter-shapes.md:186-194` |
| **The finding is written where the next author reads it** | `references/adapter-shapes.md` and `RUNBOOK.md`'s phase-10 session log, per project DoD 6 — including anything contradicting the recorded `Transaction<'static, Postgres>` shape. | `project.md` *Definition of done* item 6; `references/adapter-shapes.md:160-168`; `RUNBOOK.md:1447-1451` |

## Data and migrations

**Real, and this story owns the projection half of it.**

**Migration 1 grows; migration 2 is not opened.** `happenstance-postgres` carries
exactly one migration, authored by `postgres-schema-and-live-fixture`, and that
story states the discipline explicitly: the slice-mate that wires ADR-0024's
mechanism "adds its column to migration 1 rather than opening a migration 2,"
and the one-migration discipline "ends at first publish, which is
`publication-and-positioning`'s (HS-P0016) to decide." The same reasoning holds
here and for the same reason — the crate is `publish = false`
(`crates/happenstance-postgres/Cargo.toml:12`), so no consumer has ever applied
a schema and "migration" is a schema-authoring concern, not a data-movement one.
No backfill, no compatibility window.

**The table this story authors.** The intended shape is already written in the
crate (`crates/happenstance-postgres/src/projection_store.rs:52-62`) and needs
one column the frozen port added:

| Column | Shape | Why |
| --- | --- | --- |
| `projection_id` | `text PRIMARY KEY` | One row per `ProjectionId`, which is what makes `distinct_projections_advance_independently` pass; a single-row table is its named rejection. `ProjectionId` is an unvalidated `Box<str>` and may be empty — `text` accepts that, and rejecting it here would settle an open question in passing (`.kb/open-questions/projection-id-is-unvalidated.md`). |
| `position` | `bigint NOT NULL` | `SequencePosition` is a `NonZeroU64`; the column is signed and wider in the wrong direction, which is exactly what `CheckpointOutOfRange` is for (`crates/happenstance-postgres/src/error.rs:70-80`). |
| the authority discriminator | a `boolean NOT NULL` or a constrained `text` — **the choice is recorded** | `Checkpoint::Rebuilding` must survive a process restart, so it is stored, not held on the handle (`spec/E2E-CASES.md:653-680`). Two variants today (`Authority::Live` / `Rebuilding`) and both enums are `#[non_exhaustive]`, so pick the encoding that a third variant would not force a data migration on, and say which you picked. |

**Absence of a row is `Checkpoint::NeverRun`.** Not a sentinel position, not
`position = 0`. `fresh_projection_has_no_checkpoint` and
`reset_is_not_commit_at_first` are both written against the confusion a sentinel
creates, and the second names `checkpoint.unwrap_or(FIRST)` with an inclusive
`from` as its rejection.

**Read-model tables are the application's business.** This adapter owns the
checkpoint and the transaction that carries it, and nothing else
(`crates/happenstance-postgres/src/projection_store.rs:60-62`).

**The probe's read model is a conformance artefact and must not reach a
consumer's database.** `ProjectionProbe::probe_write` / `probe_read` /
`probe_delete_all` need somewhere to put a value. Two admissible homes: a
separate conformance-only SQL source applied by the fixture, or DDL issued by
the probe implementation itself (Postgres runs DDL inside a transaction, so this
is cheap and rolls back cleanly). The inadmissible home is migration 1 — a test
table in every consumer's schema, shipped forever, to satisfy a feature that is
off by default. Whichever is chosen, it lives behind the `conformance` feature
and the reason is recorded in the ledger.

**Indexes.** None beyond the primary key are expected: every access is a point
lookup or a point upsert on `projection_id`. If the live run says otherwise, that
is a measurement worth recording rather than a guess worth adding.

## Acceptance criteria

The persona is the adapter author (Context pack §11). Each criterion is stated as
a goal of theirs crossing the whole stack — port, adapter, schema, server, suite,
CI log — not as a capability of a function. Every row's verification names a real
path. "Live job" means the live-Postgres job `postgres-schema-and-live-fixture`
created in `.github/workflows/ci.yml`, **extended** by this story.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | GIVEN an adapter author who has just written a `ProjectionStore` body and wants an executable answer to "am I done", WHEN they run the live-Postgres job's projection command, THEN `crates/happenstance-postgres/tests/postgres_projection_conformance.rs` invokes `projection_store_conformance!` over its **full expansion** — all seventeen rules present in the binary, each reporting pass, fail or a declared skip — and not one hand-picked subset. | `crates/happenstance-postgres/tests/postgres_projection_conformance.rs` — the invocation itself, compiled by the default gate's clippy step; at run time the live job's `cargo test -p happenstance-postgres --all-features -- --ignored --list` assertion checks every name `for_each_projection_store_rule!` emits is present, modelled on `xtask/src/proof.rs:199-217` and on the sibling story's AC-007. |
| AC-002 | GIVEN the recorded shape `type Batch<'a> = sqlx::Transaction<'static, Postgres>` was written by a `todo!()` body and the frozen `begin` is neither `async` nor fallible, WHEN the adapter author implements `begin`, THEN it returns an owned `Self::Batch` without a round trip, a runtime block or a lazily-deferred checkout, its module doc names the alternative that lost and why, and where the outcome contradicts `references/adapter-shapes.md:160-168` that record and `RUNBOOK.md`'s phase-10 session log are corrected in place with what actually happened. | `crates/happenstance-postgres/tests/postgres_projection_conformance.rs::begin_does_not_touch_the_server` — a `begin` against a pool whose backing server is unreachable (a lazily-connected `PgPool`) returns a usable batch; plus the default gate's clippy step compiling `fn begin(&self) -> Self::Batch`; plus the two corrected records cited by `file:line` in the ledger. |
| AC-003 | GIVEN the adapter author's whole reason to reach for a projection port is that a read model and its checkpoint never disagree, WHEN a `commit` succeeds against a live Postgres, THEN the read-model rows and the checkpoint are both visible **through a fresh handle**, and when it fails neither is — one transaction, one commit, the batch consumed either way. | `crates/happenstance-postgres/tests/postgres_projection_conformance.rs::commit_is_atomic_with_the_read_model` (the differential rule; `CheckpointOnlyStore` is its named rejection), with `commit_advances_the_checkpoint` and `failed_commit_leaves_both_unchanged` as its baseline and its negative. Live job. |
| AC-004 | GIVEN a pooled adapter hands out several cloneable handles onto one backing store and a batch is an owned value a caller can carry anywhere, WHEN a batch begun on one store instance is passed to `commit`, `reset` or `rollback` on a different instance, THEN it is refused as `CommitError::ForeignBatch` / `ResetError::ForeignBatch` with both stores unchanged — and whether a **clone** counts as the same instance is a decided, documented answer rather than an accident of `#[derive(Clone)]`. | `crates/happenstance-postgres/tests/postgres_projection_conformance.rs::commit_rejects_a_foreign_batch` (live job) plus `crates/happenstance-postgres/tests/postgres_projection_conformance.rs::a_clone_shares_its_parents_batch_identity` — a story-local gated test asserting the chosen `Clone` semantics in the direction the module doc states, so the choice cannot silently invert. |
| AC-005 | GIVEN the reader who most needs to know a projection is mid-rebuild is the one who arrives after the rebuilding process died, WHEN `checkpoint` is called from a **new process** against the same database, THEN it still answers `Rebuilding { through }` for a projection last committed under `Authority::Rebuilding`, `NeverRun` for an id with no row at all, and `Live { through }` otherwise — the discriminator being a stored column, never a flag on the handle, and absence of a row never a sentinel position. | `crates/happenstance-postgres/tests/postgres_projection_conformance.rs::rebuilding_is_distinguishable_from_live`, `::fresh_projection_has_no_checkpoint`, `::reset_is_not_commit_at_first` (live job) plus `::rebuilding_survives_a_new_handle` — a story-local gated test that commits under `Rebuilding`, drops every handle, opens a fresh store on the same database and re-reads (E2E-25's shape). |
| AC-006 | GIVEN an adapter author rebuilding one projection must not damage another, WHEN `reset` runs for one `ProjectionId`, THEN that projection's caller-supplied deletes and its checkpoint row disappear together in one unit and every other projection's checkpoint is untouched — and whether this store ever answers `ResetError::Refused` is stated with its reason where a caller reads it rather than defaulted to "never". | `crates/happenstance-postgres/tests/postgres_projection_conformance.rs::reset_clears_rows_and_checkpoint_together`, `::reset_is_scoped_to_one_projection` (its named rejection is the checkpoint-table truncate, `TruncatingResetStore`) and `::refused_reset_changes_nothing`, live job; the refusal policy is prose on the impl, cited by `file:line` in the ledger. |
| AC-007 | GIVEN a projection that consumed a thousand events and wrote nothing must still be able to advance, and a checkpoint that goes backwards silently is how a read model loses events forever, WHEN `commit` is handed a position the batch never wrote, THEN it is accepted; and WHEN it is handed a position at or below the stored one, THEN it is refused as `CommitError::CheckpointRegression { current, attempted }` carrying the value read **inside the same transaction**. | `crates/happenstance-postgres/tests/postgres_projection_conformance.rs::commit_accepts_a_position_the_batch_did_not_write` (its rejection is `ValidatingCommitStore`) and `::commit_rejects_a_regressing_position` (its rejection is the unconditional `UPDATE … SET position = ?`), live job. |
| AC-008 | GIVEN the suite cannot look at a read model whose shape it does not know, and an adapter author should pay for that with one feature flag rather than a new edge in their dependency graph, WHEN they enable `conformance` on `happenstance-postgres`, THEN `ProjectionProbe` is implemented for `PostgresProjectionStore` forwarding to `happenstance-core/conformance`, `READS_THROUGH_BATCH` states a **fact** about the chosen batch shape (a `true` that answers from the committed table is a defect, a `false` is honest and reported), and the feature composes in every direction the powerset takes. | `crates/happenstance-postgres/tests/postgres_projection_conformance.rs::batch_reads_reflect_pending_writes` and `::rebuild_is_chunk_size_invariant` at 1, 3 and whole-log (live job); `cargo hack --feature-powerset check -p happenstance-postgres` in the default gate for `conformance` without `projection-store` and the reverse; `crates/happenstance-postgres/Cargo.toml` `[features]` diff. |
| AC-009 | GIVEN "a rule absent from the binary is indistinguishable in CI output from a rule that passed", WHEN this adapter cannot honour something the suite asks for, THEN the run prints ``SKIP {rule}: fixture declines `{capability}` — {reason}`` in the fixture's own Postgres-specific words under `--show-output`, using a capability name that already exists upstream — no invented constant, no `#[cfg]`-ed-away rule, no empty reason. | `crates/happenstance-postgres/tests/postgres_projection_conformance.rs::declined_capabilities_carry_a_postgres_specific_reason` — assertions on `RuleOutcome` **values** (never on stdout), against `crates/happenstance-testkit/src/contract.rs:495-537`; plus the live job's `--show-output` log as evidence, and codegen (`cargo test`, not `cargo clippy`) catching an empty `Capability::declined("")`. |
| AC-010 | GIVEN every other contributor to this initiative runs the gate on a laptop, WHEN they run `cargo xtask ci --fast` with the Docker daemon stopped and the network unplugged, THEN it is green with the new target **compiled, linted and reported as ignored** rather than cfg'd out of existence — while in CI the projection command runs inside the **existing** live-Postgres job (no third job) and that job cannot be green having executed zero tests. | `cargo xtask ci --fast` transcript with Docker stopped and networking disabled, showing `ignored` not `0 tests`; the `.github/workflows/ci.yml` diff showing the live job extended rather than duplicated and `xtask/src/main.rs`'s `REQUIRED` array untouched; the job's nonzero executed-test-count assertion (modelled on `xtask/src/proof.rs:199-231`) plus a deliberate negative exercise with the gate flag off, shown failing. |

**Coverage of the traced project AC.** Project **AC-005** — "`PostgresProjectionStore`
passes the projection suite against the owned `Batch` frozen by
`projection-store-freeze`, with any declined capability reported by name and
reason" — is carried by **AC-001** (the suite runs, whole), **AC-003** – **AC-008**
(it *passes*, rule family by rule family, against the frozen shape) and **AC-009**
(the declension half, which is a conjunct of the project AC and not a footnote to
it). **AC-002** and **AC-010** carry the two obligations the project AC assumes
rather than states: that the `Batch` shape is the frozen one and the record says so
(`project.md` DoD 6), and that none of this reaches the default gate
(`project.md` AC-011, DR-9).

## Interaction quality

**Composition family — N/A by sign-off.** `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_design.md`
records this project as having no user-facing surface, approved by the repository
owner on 2026-08-12, with *Surfaces*, *Items*, *Signatures*, *Shape decision*,
*Placement and re-export*, *The states the API must express* and *Anti-patterns*
all `N/A`. This story renders none. There is therefore no composition, placement,
transience policy, density budget, hierarchy or named design anti-pattern to
honour, and no AC row carries a composition invariant. Inventing one here would be
re-deciding, in a story, a design a human already signed off as absent —
`design.capture` is deliberately undeclared in `.redkiln/config.yaml`, which makes
the perceptual review a **declared** skip rather than a silent pass.

**State family — applicable, in the only medium this repository has.** What an
adapter author actually meets is a command's output, a CI job's log and a
rustdoc page. The state invariants translate directly, and each one lives in a
numbered AC row above so `redkiln verify` extracts it:

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not a context jump** — nothing this story lands may change what a contributor to any *other* project has to do to run the gate. No new daemon, no new credential, no new tool on the default path, and no third CI job to learn. | **AC-010** | `cargo xtask ci --fast` green with Docker stopped and the network unplugged; the workflow diff shows the existing live job extended, not duplicated. |
| **Non-occlusion** — nothing this story adds may hide information the run already surfaced. A declined capability's reason stays visible, a gated test appears as *ignored* rather than vanishing from the count, and no rule leaves the binary. | **AC-001**, **AC-009**, **AC-010** | The `--list` assertion proves every rule name is present (AC-001); skip lines assert on `RuleOutcome` values and print under `--show-output` (AC-009); the default run reports `ignored`, not `0 tests` (AC-010). |
| **Preserved position** — the default gate's step list and its selection-by-name survive untouched, and the sibling story's isolation scheme, container harness and job are reused rather than re-invented beside them. | **AC-010** | Reviewed diff against `xtask/src/main.rs`'s `REQUIRED` array and `wasm_steps()`; the fixture builds on `postgres-schema-and-live-fixture`'s harness. |
| **Reversibility** — an abandoned or failed run leaves nothing behind that poisons the next one: a failed `commit` changes neither half, a dropped batch releases whatever it holds, and one fixture instance's effects stay inside its own isolated backing store. | **AC-003**, **AC-004** | `failed_commit_leaves_both_unchanged`, `rollback_leaves_both_unchanged` and `dropped_batch_leaves_store_usable` (whose named rejection — "a pooled connection `Drop` never returns" — is this adapter's family) under AC-003; a refused foreign batch leaving both stores unchanged under AC-004. |
| **Reachability** — the capability is reachable by the command an adapter author would actually type, and by reading the docs, not only by a bespoke incantation. | **AC-001**, **AC-008** | `cargo test -p happenstance-postgres --all-features -- --ignored --show-output` is both the documented command and what the job runs; `conformance` is a real manifest feature beside `event-store` and `projection-store`, not a `#[cfg(test)]` block. |
| **Honest state, and honest failure** — the run must be unable to report success for a state that is not success, and the store must be unable to report a state it cannot actually distinguish. | **AC-005**, **AC-010** | `Rebuilding` re-read from a new process (AC-005) is the store-side half — a rebuild-in-place with one position field reports "live" for a projection that is not; the nonzero executed-test-count assertion plus the negative exercise (AC-010) is the job-side half. |
| **Keyboard-reachability's analogue — no hidden prerequisite.** Enabling `conformance` must not require a second undocumented flag, a nightly toolchain, or a network fetch at build time. | **AC-008** | The feature powerset in the default gate takes `conformance` on its own and with each sibling feature; no new dependency edge (NF-005). |

The last row of the state table is this story's equivalent of the unstyled render
that satisfies every data-attribute assertion: a job that compiled everything and
executed nothing, and a store that answers `Live` because it has nowhere to put
`Rebuilding`, both look exactly like success. AC-010 and AC-005 are what make them
fail.

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | The checkpoint row holds a `bigint` outside `SequencePosition`'s domain — zero, negative, or written by something that is not this adapter. | `PostgresProjectionStoreError::CheckpointOutOfRange { value }`, already declared and still unconstructed (`crates/happenstance-postgres/src/error.rs:60-80`). Never a `panic!`, never an `unwrap`, never a clamp to `FIRST`: a library does not panic on a row a real database can hand back, and a clamp turns a corrupt checkpoint into silent event loss. |
| **EC-002** | The server is unreachable, the pool checkout times out, or the transaction is rolled back by the server at `commit`. | `CommitError::Store(PostgresProjectionStoreError::…)` with the driver error preserved as source. The batch is consumed either way — that is `commit`'s signature, not a choice — and **neither** half of the write is durable. `failed_commit_leaves_both_unchanged` is the rule that observes it. |
| **EC-003** | `commit` is handed a position at or below the stored checkpoint. | `CommitError::CheckpointRegression { current, attempted }`, where `current` is read inside the same transaction as the conditional `UPSERT`. Reading it on a second connection is precisely the defect `commit_is_atomic_with_the_read_model` exists to find. Not `Ok(())`, and not a store error. |
| **EC-004** | A batch begun on one store instance reaches `commit`, `reset` or `rollback` on another (PS-15 names all three). | `CommitError::ForeignBatch` / `ResetError::ForeignBatch`, both stores observably unchanged afterwards. A run-time stamp comparison — not a lifetime, which was compiled and refuted (`spec/SPECIFICATION.md:5099-5124`). |
| **EC-005** | A batch is dropped without `commit` or `rollback`. | Nothing blocks in `Drop` and the store stays usable for a subsequent `begin`/`commit`. Under a deferred write set nothing is checked out at drop time; that is an argument **for** the shape, not a licence to decline `dropped_batch_leaves_store_usable`. |
| **EC-006** | A `ProjectionId` is empty, or is long enough to be awkward as a key. | Accepted. `ProjectionId::new` is infallible and admits the empty string, and that is a **recorded open question** (`.kb/open-questions/projection-id-is-unvalidated.md`), not a licence to reject it here. A `CHECK (projection_id <> '')` in migration 1 would settle it in passing. |
| **EC-007** | Two commits to the *same* `ProjectionId` contend, and Postgres answers with a serialization failure or a deadlock. | Surfaced as a store error, not swallowed and not silently retried. If a retry policy is wanted, it is a decision recorded on the impl with its reason — an invisible retry loop inside `commit` changes the meaning of the port's error for every caller. |
| **EC-008** | HS-P0010's artefacts are not in the tree when work starts — no `Checkpoint`/`Authority`/`CommitError`/`ResetError`, no `ProjectionProbe`, no `ProjectionFixture`, no `projection_store_conformance!`, or no named projection capability for something this adapter must decline. | **Halt and report.** Do not stub the missing half, do not mint a local capability constant, and do not write the adapter against today's provisional signatures (Context pack §1 and §6). This is the story's one hard precondition and the project risk table names it. |
| **EC-009** | The probe's read-model table is proposed for migration 1. | Refused. A conformance table in every consumer's schema, shipped forever, for a feature that is off by default. It lives behind the `conformance` feature — a conformance-only SQL source applied by the fixture, or DDL issued by the probe (Postgres runs DDL transactionally) — and the choice is recorded. |
| **EC-010** | The live job's container fails to start, or the gating flag is wrong so the projection tests never execute. | The job **fails**. It may not skip, may not `continue-on-error`, and may not be made non-blocking — DR-9 and `project.md`'s risk table are explicit that a flaky live job is fixed or reported, never quietly weakened. Inherited wholesale from `postgres-schema-and-live-fixture`'s AC-010, extended to cover the new target. |

## Non-functional

| id | Requirement | Why, and how it is held |
| --- | --- | --- |
| **NF-001** | The deferred write set's memory cost is bounded by what the caller puts in it, and that is said out loud where the caller reads it. | A batch that buffers until `commit` holds the whole write set in memory — the honest trade for a synchronous infallible `begin`. `rebuild_is_chunk_size_invariant` runs at 1, 3 and whole-log precisely because chunking is the caller's lever; the rustdoc must say that a rebuild of an unbounded log in one batch is the caller's decision, not a store the port protects them from. |
| **NF-002** | The projection target must not turn the live job into a timeout. | The suite constructs a fixture instance per rule and this story adds seventeen more rules to the same job. Reuse `postgres-schema-and-live-fixture`'s per-instance isolation scheme (schema- or database-per-instance against one shared container) rather than a container per instance, and record the job's total wall clock before and after. |
| **NF-003** | Everything added builds at the MSRV, **1.97.1**. | The `msrv` job runs `cargo hack check --no-dev-deps --rust-version` and then a full `cargo test --workspace --all-features`, and the second half is the one that sees dev-dependencies. Five of the five database crates in this workspace declare no `rust-version` at all, so only running the compiler finds a floor violation (CLAUDE.md, binding constraint 5). No dependency addition is expected; one would need its own justification. |
| **NF-004** | The new `conformance` feature is additive and composes in every direction. | `cargo hack`'s feature powerset runs in the default gate and will take `conformance` alone, with `projection-store`, and without it. `happenstance-core`'s `--no-default-features` doc build is also a gate step. A feature that only compiles in the `--all-features` corner is a feature that breaks the first consumer who is selective. |
| **NF-005** | No new dependency edge, and `deny.toml`'s allowlist is unchanged. | `sqlx` and `happenstance-core` are already in the manifest; `ProjectionProbe` is behind a feature on a dependency the crate already has, which is the whole argument for putting it in the contract crate (`spec/SPECIFICATION.md:5016-5031`). If a new dependency turns out to be needed, that is a finding to report, not a line to add. |
| **NF-006** | Every fallible public function carries an `# Errors` section naming the **conditions**, and the `conformance` feature is visible on docs.rs. | `standards/rust/30-error-taxonomy.md` and `standards/rust/70-rustdoc-obligations.md`. The nightly `--cfg docsrs` rustdoc build is a gate step, so a feature-gated item without its `doc(cfg(…))` annotation is invisible to exactly the reader it exists for. |
| **NF-007** | No literal position values anywhere in the new tests or fixtures. | CF-6; `cargo xtask lint-position-literals` is wired into the default gate (`xtask/src/main.rs:389,687`). The specification permits gaps and a conformant adapter may leave them — compare against positions the store actually assigned. |
| **NF-008** | The crate stays `publish = false` and its scoped `#![allow(clippy::todo)]` stays in place. | `deskeleton-and-package-readiness` owns the whole de-skeletoning move and depends on this story rather than the reverse (`_storymap.md`, *Merge order* step 6). Removing the allow here while `event_store.rs` still holds `todo!()` bodies would fail the gate for a reason that has nothing to do with this PR. |

## Implementation notes (non-prescriptive)

These are the paths the evidence points down. None is binding; a better-reasoned
alternative that satisfies the ACs is welcome, and the reason it was chosen is
what the ledger wants.

**Start by reading the frozen port, not this spec's summary of it.** Context pack
§1's five-row table is orientation. `crates/happenstance-core/src/projection.rs`
as it stands when you start is the truth, and if it does not match the table,
EC-008 applies.

**Order of work that fails fastest.** The schema and the checkpoint round trip
first (`checkpoint`, then `commit` with the conditional `UPSERT`), because
`fresh_projection_has_no_checkpoint` and `commit_advances_the_checkpoint` are the
two rules whose failure means the storage model is wrong rather than the plumbing.
Then `reset`, then the probe, then `READS_THROUGH_BATCH`. The macro is invoked
last only in the sense of being *green* last — write the tests file early and let
it fail loudly.

**The batch is likely a `Vec` of statements plus an identity stamp.** Something
along the shape of "the store's instance id, and an ordered list of what to
execute" — replayed into one transaction at `commit`. That gives EC-005 for free
(nothing to release), makes PS-15's check an integer comparison, and makes
`READS_THROUGH_BATCH = true` reachable by consulting the buffer before the table.
The cost is NF-001, and the cost is the thing to document.

**The `Clone` question deserves a paragraph, not a line.** A cloned handle onto
the same pool is, for every other purpose in this crate, the same store. If the
stamp is minted in `new` and cloned along with the handle, `commit_rejects_a_foreign_batch`
is satisfied by two separately-constructed stores and clones interoperate — which
is almost certainly what a caller expects from a pooled handle. Minting a fresh
stamp per clone is defensible too, and stricter. What is not defensible is
discovering which one you built by watching a rule fail.

**The conditional `UPSERT`.** `INSERT … ON CONFLICT (projection_id) DO UPDATE SET
… WHERE checkpoint.position < EXCLUDED.position` returning the row is one shape;
a `SELECT … FOR UPDATE` followed by a write in the same transaction is another and
is easier to read. Both keep the read of `current` inside the transaction, which
is the only property EC-003 and AC-003 actually require.

**Take the gating mechanism from the sibling, and take its finding with it.**
`postgres-schema-and-live-fixture` recorded that `required-features` does **not**
work as a gate here: the default gate runs `cargo test --locked --workspace
--all-features`, and `--all-features` enables the required feature, so the target
builds *and runs* against a server that is not there. Use whatever that story
landed — expected to be `#[ignore]` plus `-- --ignored` in the live job — and do
not re-derive the choice.

**The `conformance` feature is on `happenstance-postgres`, forwarding.**
`conformance = ["happenstance-core/conformance"]` beside `event-store` and
`projection-store` in `crates/happenstance-postgres/Cargo.toml:28-34`. Copy the
feature-declaration pattern from `crates/happenstance-core/Cargo.toml`'s `memory`
/ `serde` split rather than inventing a third style.

**When a rule looks wrong, it is a report.** `crates/happenstance-testkit/**` and
`crates/happenstance-core/**` are must-not-change seams here. A rule that seems to
demand something Postgres cannot do goes to HS-P0010 with the transcript, not into
a local edit — and per Context pack §10, so does anything that contradicts
`references/adapter-shapes.md`.

## Tests and CI (merge gate)

Grounded in the *Testing brief*'s AC-005 row
(`.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md:515`)
and its Notes §6 two-list split — the tree-local gate every story runs, and the
live job that gates the project.

| tier | command / path | proves |
| --- | --- | --- |
| **Static — compile** | `cargo clippy --workspace --all-targets --all-features -- -D warnings` (default gate) | AC-001 and AC-002 at compile time: `projection_store_conformance!`'s full expansion type-checks against the fixture, and the impl matches the frozen `fn begin(&self) -> Self::Batch` signature. Note the limit — `Capability::declined("")` fires at **codegen**, so clippy alone proves nothing about the constants (AC-009). |
| **Static — feature powerset** | `cargo hack --feature-powerset check -p happenstance-postgres` (default gate, tool present on this machine) | AC-008 and NF-004: `conformance` compiles alone, with `projection-store`, and `projection-store` compiles without it. |
| **Static — docs** | `cargo doc --workspace --all-features --no-deps` and the nightly `--cfg docsrs` build (default gate) | NF-006: the `# Errors` sections and the feature-gated probe impl are visible to the reader they exist for. |
| **Static — position literals** | `cargo xtask lint-position-literals` (`xtask/src/main.rs:389,687`, default gate) | NF-007: no `[1, 2, 3]` anywhere in the new tests or fixture. |
| **Default gate, no infrastructure** | `cargo xtask ci --fast` with the Docker daemon stopped and the network unplugged (`.redkiln/config.yaml:55`) | AC-010: green, with the projection target compiled, linted and reported as `ignored` — never `0 tests`, never cfg'd out. |
| **Story grain** | `cargo xtask affected --base <base>` (`.redkiln/config.yaml:40`) | The story-grain gate: only what this diff could break, run on every push. |
| **Live — mount assertion** | *(existing live-Postgres job, new step)* `cargo test -p happenstance-postgres --all-features -- --ignored --list` | AC-001: every name `for_each_projection_store_rule!` emits is present in the binary, asserted **before** the run, modelled on `xtask/src/proof.rs:199-217`. |
| **Live — the suite** | *(existing live-Postgres job, extended)* `cargo test -p happenstance-postgres --all-features -- --ignored --show-output` against the pinned `testcontainers` Postgres | AC-003 – AC-008: seventeen rules against a real server, and `--show-output` so AC-009's skip lines land in the log. |
| **Live — story-local tests** | *(same job)* `begin_does_not_touch_the_server`, `a_clone_shares_its_parents_batch_identity`, `rebuilding_survives_a_new_handle`, `declined_capabilities_carry_a_postgres_specific_reason` in `crates/happenstance-postgres/tests/postgres_projection_conformance.rs` | AC-002, AC-004, AC-005, AC-009 — the four obligations no conformance rule can observe on its own, because they are about *this* adapter's construction rather than the port's contract. |
| **Live — vacuity guard** | *(same job)* nonzero executed-test count, modelled on `xtask/src/proof.rs:199-231`, plus a deliberate negative exercise with the gate flag off | AC-010: the job cannot be green having run nothing. Extended to count the projection target, not only the event-store one. |
| **MSRV** | the existing `msrv` job (`.github/workflows/ci.yml:241`) | NF-003: everything new builds at 1.97.1, including whatever the tests target pulls in — the half `--no-dev-deps` hides. |
| **Backlog / KB** | the existing `backlog` job (`.github/workflows/ci.yml:102`); `redkiln validate --kb && redkiln doctor` | The ledger exists and every AC row is present; `doctor` still reports exactly the six expected `template-drift` advisories. No ADR is owed by this story (Integration contract, *Clause(s)*). |

**Merge DoD, restated as commands.** `cargo xtask ci --fast` green with no Docker
and no network; the live-Postgres job green on the same tree with the projection
target executed and its `--list` and count assertions passing; no `todo!()` left in
`crates/happenstance-postgres/src/projection_store.rs`.

## Risks and coupling (PR-scoped)

| Risk | Why it is live in *this* PR | Mitigation |
| --- | --- | --- |
| **The `Batch` shape moves after this adapter is written.** | This is the one story in the project written against a freeze that lands in another project, and `project.md`'s risk table names it in terms. | The dependency is a project-level `blocked_by` on HS-P0010, and EC-008 makes "start anyway" a halt rather than a judgement call. If the shape moves after merge, that is a re-plan input, not a silent fix. |
| **The recorded `Transaction<'static, Postgres>` binding is treated as settled and the synchronous `begin` is "fixed" to accommodate it.** | The record says accepted, the crate's module doc argues for it at length, and the shortest path from `PgPool` to a batch is `async fn begin`. | Context pack §2 forbids all three variants (block-on, re-`async`-ing the port, lazy checkout) with a stated reason each; AC-002 makes the shape decision and the corrected record deliverables rather than side effects. |
| **A capability is declined with a locally-invented name.** | The projection capability set is HS-P0010's, and the fastest way past a rule this adapter cannot satisfy is a fresh constant. | AC-009 requires an upstream name; a capability that has no upstream name is EC-008's halt-and-report, not a local `const`. |
| **The probe's read-model table lands in migration 1.** | It is one line, it makes the fixture simpler, and nobody would notice until a consumer applied the schema. | EC-009 refuses it outright, and the *Data and migrations* section names both admissible homes. |
| **The live job's wall clock grows past patience and someone weakens it.** | Seventeen rules, each constructing a fixture instance, appended to an already-live job. | NF-002 asks for the before/after measurement; DR-9 and `project.md`'s risk row forbid making a slow or flaky live job non-blocking without a recorded decision. |
| **`--all-features` defeats a `required-features` gate and the default gate starts trying to reach a server.** | It is the natural first choice for gating a live test, and it silently does the opposite of what it looks like. | Inherit `postgres-schema-and-live-fixture`'s chosen mechanism and its recorded finding (Implementation notes); AC-010's Docker-stopped, network-unplugged run is the check that catches it. |
| **`Rebuilding` is implemented as a flag on the handle because it is easier and the in-process tests pass.** | Every test in one process will agree with an in-memory flag; only a new process disagrees. | AC-005's `rebuilding_survives_a_new_handle` is a story-local test precisely because it is the reader after the crash that E2E-25 is about, and `rebuilding_is_distinguishable_from_live` is the suite's half. |
| **Scope drifts into de-skeletoning.** | With the last `todo!()` in `projection_store.rs` gone it is tempting to remove `publish = false` and the scoped allow in the same PR. | NF-008 and the *PR boundary*: `deskeleton-and-package-readiness` owns that move and depends on this story. `event_store.rs` still holds `todo!()` bodies until its own slice lands. |

**Coupling to the sibling story.** This PR does not re-derive the container
harness, the isolation scheme or the job — it consumes all three from
`postgres-schema-and-live-fixture`. If that story's isolation scheme turns out not
to suit a projection fixture, the answer is to say so and change it there, not to
grow a second scheme beside it (Context pack, *Behavior and interfaces*, "One
fixture instance is one isolated backing store").

## Dependencies

**Blocks on**

- `postgres-schema-and-live-fixture` — the only story edge. It supplies migration
  1 (which this story's checkpoint table extends rather than replaces), the
  `testcontainers` harness and per-instance isolation scheme the projection
  fixture is built on, and the live-Postgres CI job this story **extends** rather
  than duplicates. Its AC-010 vacuity guard is inherited and widened here.

**Blocks on, at the project grain rather than the story grain**

- `projection-store-freeze` (HS-P0010) — a project-level `blocked_by` recorded on
  `project.md` (*Depends on*), not a story edge, which is why it does not appear
  in this story's `depends_on`. It supplies three things this story consumes and
  none of which it may stub: the frozen `ProjectionStore` port and its four new
  types, the projection conformance suite and its macro, and the
  capability-declension policy the skips are reported under. EC-008 is the halt
  condition if any is absent.

**Unlocks**

- `deskeleton-and-package-readiness` — cannot honestly run until the last
  `todo!()` in `happenstance-postgres` is gone, and the four bodies this story
  fills are among them (`_storymap.md`, *Merge order* step 6).
- `far-end-discharge-record` — reports on what the slices above it discharged,
  and this story's PS-clause exercise record is part of what it reads.
- At the project grain, `publication-and-positioning` (HS-P0016), whose published
  compliance claim must name which implementations it was checked against — this
  adapter being one of them.

**Not coupled.** `ladybug-projection-store` is the other `ProjectionStore` adapter
in this initiative and touches a disjoint crate; the two may be built in either
order. It, not this story, writes the freeze verdict (`project.md`, *Out of
scope*).

## Anchors (progressive disclosure)

Nothing below needs opening before work starts — the Context pack is the
must-read core. Each row says why the artefact is load-bearing and the moment to
open it. Every path was checked to exist.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `crates/happenstance-core/src/projection.rs` | The port as it actually is in the tree at implementation time. If it does not match Context pack §1's five-row table, EC-008 applies and the story halts. | **First**, before writing anything — it is the precondition check. | AC-002, EC-008 |
| `spec/SPECIFICATION.md` | §7's PS clauses: the frozen port's signatures (`:4632-4731`), `Checkpoint`'s three-variant argument (`:4645-4660`), the seventeen-rule table (`:5652-5676`), the refuted lifetime fix (`:5099-5124`), PS-12's read-through obligation (`:5052-5074`), PS-36's `Batch: Send` reasoning (`:5601-5627`). The clause wins wherever a summary disagrees with it. | Per rule family, as you implement it — not in one pass. | AC-001 – AC-008 |
| `spec/E2E-CASES.md` | E2E-24 (`:622-651`) is the deferred-write-set shape stated as observable behaviour; E2E-25 (`:653-680`) is the reader who arrives after the rebuilding process died — the case AC-005's restart test is built from. | Before choosing the batch representation (E2E-24); before implementing the authority column (E2E-25). | AC-002, AC-005 |
| `crates/happenstance-postgres/src/projection_store.rs` | Four `todo!()` bodies under a module doc that argues at length for an owned `Transaction` (`:1-63`), plus the intended checkpoint table (`:52-62`) and the existing `send_flavour_satisfies_the_bare_bound` test (`:135-141`). The doc is rewritten by this story, not left standing beside a body that contradicts it. | Before the first line of implementation. | AC-002, AC-003 |
| `crates/happenstance-postgres/src/error.rs` | `PostgresProjectionStoreError::CheckpointOutOfRange { value }` at `:60-80` — declared, unconstructed, and the intended answer to a `bigint` outside `SequencePosition`'s domain. | When mapping the checkpoint read. | EC-001 |
| `crates/happenstance-postgres/Cargo.toml` | The `[features]` block at `:28-34` — `default = ["event-store", "projection-store"]`, both empty today. The new `conformance` feature is mounted here, forwarding, not as a `#[cfg(test)]` block. | When adding the probe. | AC-008 |
| `crates/happenstance-testkit/src/contract.rs` | `Capability`'s `const fn` assertion (`:404-410`, which fires at codegen and not under clippy) and `RuleOutcome::skip_line` / `report` (`:495-537`) — the exact skip-line format AC-009 is asserted against. | When answering the fixture's capability constants. | AC-009 |
| `crates/happenstance-testkit/src/lib.rs` | Why the macro takes an **expression building a fixture** rather than a type (`:265-276`): a Postgres fixture needs a connection URL and an argument-less constructor would have to reach into the environment for it. Also the re-export list at `:186`. | When writing the tests-target invocation. | AC-001 |
| `crates/happenstance-testkit/src/fixtures.rs` | `MemoryFixture` (`:252-267`) — the reference implementation of "one fixture instance is one isolated backing store, each `connect()` a handle onto it". Read for shape, never for storage assumptions. | When writing the projection fixture. | AC-001, AC-004 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/postgres-schema-and-live-fixture/spec.md` | The sibling this story stands on: the isolation scheme with its reason, the container harness, the live job's shape, the vacuity guard, and the recorded finding that `required-features` does not survive `--all-features`. | Before building the fixture and before touching the workflow. | AC-010, NF-002 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` | The five-row signature-change table (`:586-590`), the per-instance-stamp reasoning (`:630-640`) and the named hazard of "improving" `begin` back to `async fn` (`:702-706`). The upstream project's own account of what it froze and why. | When the frozen port's *reasoning*, not just its shape, is needed — chiefly for AC-002 and AC-004. | AC-002, AC-004 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_storymap.md` | Which upstream story lands which artefact — the port shape, the probe feature, the suite entry point, the capability skips. Turns "it is missing" into "it is missing and here is who owns it" for EC-008's report. | Only if a precondition is missing. | EC-008 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md` | *Architecture brief* §1 (must-not-change seams), §2 (the macros are the mount), §7 (gate and CI wiring), §8 point 4 (AC-005 last); *Testing brief* AC-005 row (`:515`) and Notes §6 (the two merge-gate command lists, `:596-630`). | When wiring the tests target and the workflow. | AC-001, AC-010 |
| `references/adapter-shapes.md` | The `Transaction<'static, Postgres>` binding recorded as **accepted** evidence at `:160-168`, and the `error[E0195]` trap at `:186-194`. This is one of the two files AC-002's finding is written into. | When the type checker disagrees with the record — and again when writing the finding. | AC-002 |
| `RUNBOOK.md` | Phase 10's exit criteria (`:4376`, "**Four** macros green against a real Postgres" — this story is the fourth) and the phase-2 session log's restatement of the batch-shape claim (`:1447-1451`), the second file AC-002's finding lands in. | At the end, when recording the finding and checking the phase's exit bar. | AC-002 |
| `.kb/open-questions/projection-id-is-unvalidated.md` | Why `projection_id` is `text` with no `CHECK`: `ProjectionId::new` is infallible and admits the empty string, and rejecting it in this migration would settle an open question in passing. | When writing the checkpoint table's DDL. | EC-006 |
| `.kb/open-questions/cf-40-fixture-limits-ownership.md` | The contested ownership of fixture limits that the declension policy sits on top of — consumed here, never settled here. | Only if a capability answer seems to require settling it. | AC-009 |
| `.kb/decisions/0001-async-port-flavours.md` | Why no `#[async_trait]`, why two flavours, and why a `Send` bound cannot be written onto the port — the reason PS-36 makes `Batch: Send` the *adapter's* obligation rather than the port's. | When the `SendProjectionStore` flavour or a `Send` bound is in question. | AC-001 |
| `standards/rust/91-adapter-authoring-recipe.md` | The house recipe for exactly this act — writing an adapter against a two-flavour port — with a compiled example and a named wrong implementation. | Before the first impl block. | AC-002, AC-003 |
| `standards/rust/30-error-taxonomy.md` | Error-type shape and the `# Errors`-by-condition obligation that NF-006 is stated from; why `CommitError` and `ResetError` stay two enums. | When declaring or mapping errors. | EC-001 – EC-003, NF-006 |
| `standards/rust/51-features-and-no-std.md` | How a feature is declared so the powerset does not find a hole, and how a feature-gated item stays visible on docs.rs. | When adding the `conformance` feature. | AC-008, NF-004 |
| `xtask/src/proof.rs` | The in-tree `--list` and nonzero-count assertions (`:199-231`) written for exactly this failure — `cargo test` exits 0 on `running 0 tests`. The model for both of AC-001's and AC-010's job-side assertions. | When extending the live job. | AC-001, AC-010 |
| `.github/workflows/ci.yml` | The job topology: `gate` (`:31`), `backlog` (`:102`), `wasm-conformance` (`:204`), `msrv` (`:241`), `semver` (`:279`), `advisories` (`:325`), plus the live-Postgres job the sibling story adds beside them. This story edits **one** of those jobs and adds none. | When mounting the second half of the mount point. | AC-010 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | Persona 2, the adapter author (`:114-152`) — the goal, the fear and the journey the acceptance criteria are framed from. | If an AC's framing is ever in doubt. | AC-001 – AC-010 |

## Clarifications resolved during spec

1. **The ten AC ids the first pass enumerated are kept exactly** — AC-001 …
   AC-010, none added, none dropped. The ledger matches them one for one. Two of
   the ten carry obligations project AC-005 assumes rather than states (AC-002's
   record correction, AC-010's gate hygiene); they are numbered ACs rather than
   prose because `redkiln verify` only sees rows.
2. **The composition family of *Interaction quality* is N/A, and that is a
   sign-off, not an omission.** `_design.md` records no user-facing surface for
   this project, approved 2026-08-12. No composition invariant appears as an AC
   row, and no story-level design decision was taken to fill the gap. The state
   family *is* applicable and every one of its invariants is carried by a numbered
   AC.
3. **`references/adapter-shapes.md`'s `Transaction<'static, Postgres>` binding is
   treated as evidence under test, not as settled.** The spec does not pre-declare
   the deferred write set as the answer — it names it as the expected arm, forbids
   the three shortcuts, and makes the decision plus the corrected record an
   acceptance criterion (AC-002). If a construction is found that keeps a live
   transaction under a synchronous infallible `begin`, that is a finding for
   HS-P0010, not a licence to reopen PS-6 here.
4. **The gating mechanism is inherited, not re-chosen.** The sibling story
   recorded that `required-features` is defeated by the default gate's
   `--all-features`, leaving `#[ignore]` (or an env read that fails loudly) as the
   admissible answer. This story consumes that finding rather than re-deriving it,
   and AC-010 is the check either way.
5. **The `Clone` identity question is left to the implementer, but not left
   open.** Both answers are defensible; what the spec fixes is that the answer is
   deliberate, documented where an implementer meets it, and pinned by a
   story-local test (AC-004) so it cannot silently invert later. The story does not
   pick for them because the fixture's handle model — which lands upstream in
   HS-P0010 — is the input that ought to decide it.
6. **The probe's read model has two admissible homes and one forbidden one.** A
   conformance-only SQL source applied by the fixture, or DDL issued by the probe
   itself; never migration 1 (EC-009). The choice and its reason are recorded in
   the ledger.
7. **No ADR is owed by this story.** No `[FROZEN]` clause is touched, and PS-5's
   and PS-6's provisional markers are *supplied evidence*, not disposed of —
   disposition belongs to HS-P0010's `unstable-projection-gate-and-clause-disposition`
   and to the Neon phase respectively. If an ADR appears to be owed, that is Context
   pack §10's halt condition, and per this repository's practice an ADR is never
   written as a side effect of implementation.
8. **HS-P0010's absence is a halt, not a workaround (EC-008).** This is the single
   most likely way the story goes wrong, so it is stated as a hard precondition in
   the Context pack, as an error condition, as a risk-table row and as the trigger
   for the `projection-store-freeze/_storymap.md` anchor. Three of the four are
   redundant on purpose.
