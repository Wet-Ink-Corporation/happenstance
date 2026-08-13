---
item: HS-S0026
stage: discover
created: 2026-08-12T13:01:48.427Z
updated: 2026-08-12T13:01:48.427Z
template_sig: 86ce4036
rendered_sig: "97989338"
---

# Discover — The application-facing Projection trait and its streaming runner

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice: land the application-facing `Projection` trait and its runner in `crates/happenstance/`, layered over `crates/happenstance-core/src/projection.rs`'s checkpoint port — nominating with `Query`, decoding, opening `begin` and committing read-model write + checkpoint in the single `commit`, advancing past the **inclusive** `from`, and **streaming, never `collect`ing** — with its feature gating stated explicitly rather than inherited | `_storymap.md:57` (M5 row) | Five properties, four of which are silent failures under test and only fail in production or at scale |
| AC-006 — the `Projection` trait and its runner exist in `happenstance`, layered over `happenstance-core`'s checkpoint pump, nominating events with `Query` | `project.md:181-184` | ADR-0007's shape, consumed rather than re-derived |
| `depends_on: codec-and-feature-forwarding` — supplies `Codec` and `CodecError`, because the runner's whole reason to sit above the port is that it **decodes** | `_storymap.md:57`, `:124-126`; ADR-0007 | M5 is sequenced after M3 for exactly this reason |
| ADR-0007's three shape decisions, not open: `Query` as the only nomination vocabulary, `Projection::Store` as an associated type so a cross-store projection is unrepresentable, and checkpoints per `(store, ProjectionId)` | `_decomposition.md:689-700` (Tensions 3), citing `.kb/decisions/0007-projection-runner-decodes.md:60-68` | Design against these today; they do not wait on HS-P0010 |
| The transactional invariant, read from the port itself: `commit(batch, id, position)` *"commits the batch and advances `id`'s checkpoint to `position`, **as one atomic unit**"*, and *"the batch is consumed either way; a failed commit must leave the store unchanged."* There is deliberately **no** `apply()` / `set_checkpoint()` pair | `crates/happenstance-core/src/projection.rs:112-130`; `_grounding.md:84-95` | The runner must not synthesise the pair the port refuses to offer |
| **`ReadOptions::from` is inclusive**, and the port's own doc says to feed the checkpoint to it *"after advancing past it"*; `SequencePosition::next` returns `Option` and the `None` arm is real | `crates/happenstance-core/src/projection.rs:101-110`; `_decomposition.md:509-514` | An off-by-one re-applies one event on every restart — invisible for an idempotent projection, corrupting for a counter |
| **AC-A05 — the runner streams; only the decision path buffers.** The command loop may use `read_decision_model`, which collects; the runner must not, because `EventStore::read` returns the stream at the top level precisely so a million-event replay is not buffered | `_decomposition.md:391-398`; `crates/happenstance-core/src/store.rs:110-123` | A runner that calls `collect` defeats the reason the port has the shape ADR-0001/ADR-0008 fought for and makes E2E-25's chunked rebuild unwritable |
| The binding surface: `Projection { type Event: DomainEvent; type Store: ProjectionStore; id(&self) -> &ProjectionId; scope(&self) -> &Tags; apply(&mut self, event, batch) -> Result<(), …> }` and `run_projection(events, models, projection, codec, chunk: NonZeroUsize) -> Result<Progressed, ProjectionError<…>>` | `_design.md:536-570` | `chunk` is `NonZeroUsize` and required; `Progressed { through, applied }` names how far the checkpoint moved (`_design.md:951-955`) |
| **Feature gating, settled here rather than inherited: `unstable-projection`, off by default**, forwarded to `happenstance-core/unstable-projection` *if* HS-P0010 lands it, gating only this crate's items if it does not. Shipping on by default was rejected because the port beneath is *"not yet frozen"* in its own module doc | `_design.md:647`, `:746`, `:828`; `project.md:267` (risk row 3) | The feature name **is** the semver promise, which is explicitly none |
| `CHANGELOG.md` already asserts *"`ProjectionStore` ships behind an off-by-default `unstable-projection` feature"* while no manifest carries such a feature — verified: the claim is at `CHANGELOG.md:19-22` and `crates/happenstance-core/Cargo.toml`'s `[features]` block does not have it | `CHANGELOG.md:19-22`; `_decomposition.md:701-710` (Tensions 4) | Today it is a claim in an unpublished file; at this project's exit it becomes a claim a stranger can check |
| `Batch<'a>` borrows from the store, so it is **not `'static`** and cannot be held across a `tokio::spawn` boundary — a fan-out runner cannot move a batch into a task | `_decomposition.md:697-700`; `crates/happenstance-core/src/projection.rs:96-99` | Constrains PS-30's subject, which `projection-clause-verdicts` settles |
| Bind `EventStore`, not `SendEventStore`; where a runner holds a stream across an await inside a spawned task, state the bound the way `spawns_from_generic` does | `_decomposition.md:430`; `_grounding.md:96-104`, citing `crates/happenstance-core/src/memory.rs:643-680` | CLAUDE.md binding constraints 3 and 4, applied to new code rather than to the frozen port |
| **`MemoryProjectionStore` does not exist.** AC-006's transactional test has no in-memory `ProjectionStore` to run against; shipping that fixture is `projection-store-freeze`'s (HS-P0010) own AC-012, and writing a throwaway here would freeze a fixture shape this project does not own | `_storymap.md:141-146`; `_decomposition.md:843-857`; `_design.md:1260-1263` | A project-level dependency already recorded in `project.md`, *Dependencies* — not a gap to close locally |

## Questions

**Answered here.**

- *Does the runner buffer?* No. It streams and commits in chunks of `chunk`
  (`_design.md:557-570`), and AC-A05 makes the distinction explicit: the *decision* path may
  buffer because DCB queries are narrow by construction; the *replay* path may not.
- *How does it resume?* From `ProjectionStore::checkpoint`, advanced **past** with
  `SequencePosition::next`, because `ReadOptions::from` is inclusive
  (`crates/happenstance-core/src/projection.rs:101-110`). `None` means never run, and it is a
  real arm rather than a degenerate case.
- *How does it write?* `begin` → apply decoded events into the adapter's batch → a single
  `commit(batch, id, position)`. Never an `apply()`/`set_checkpoint()` pair; the port does not
  offer one, on purpose.
- *Is the runner on by default?* No — `unstable-projection`, off by default, and the design
  settles this rather than leaving it to PS-3's verdict, which stays HS-P0016's
  (`_design.md:647`).

**Deferred, with owners named.**

- **Blocked on HS-P0010: `MemoryProjectionStore`.** The transactional test — the one that
  proves read-model write and checkpoint move together and that a failed commit leaves the
  store unchanged — has nothing to run against until `projection-store-freeze` ships the
  in-memory fixture behind its `memory` feature, which is that project's own AC-012. This
  story does **not** write a throwaway: doing so would freeze a fixture shape it does not own
  (`_storymap.md:141-146`; `_design.md:1260-1263`). Request it as an explicit input from
  HS-P0010 before implementation begins; if declined, escalate.
- **Provisional on HS-P0010: the trait's batch-facing signature.** The port is *"not yet
  frozen"* in its own module doc. What is *not* provisional and can be specified today: the
  transactional invariant, the `Batch<'a>` GAT borrowing from the store, and ADR-0007's three
  shape decisions (`_decomposition.md:689-700`).
- *Whether `Projection::apply` is synchronous or asynchronous.* Deliberately not prescribed
  (`_decomposition.md:712-723`). If async, it is `trait_variant`, never `#[async_trait]`, and
  the cost is a doubled surface an application must implement — a DT-2 cost as much as a
  technical one. The design shows it synchronous (`_design.md:550-554`); spec confirms.
- *The `CHANGELOG.md` / manifest discrepancy.* Named here because this story is where the
  feature is decided; **reconciling it before publish is `publish-0-2-0-alpha-1`'s** and it is
  release-blocking there (`_decomposition.md:1003-1023`).

**Not blocked on `trybuild`** — that is `compile-fail-proof-artefact`'s input.

## Decision

The problem this slice solves is that `happenstance-core` deliberately holds a checkpoint
*port* and nothing that runs: an application that wants a read model has a transactional
write seam and no way to get decoded events into it, and ADR-0007 says why the decode
belongs one crate up. This story lands the trait an application implements and the runner
that drives it — nominating events with the same `Query` vocabulary everything else in the
crate uses, decoding them with the same `Codec`, opening the adapter's batch, and committing
the read-model write and the checkpoint as the one atomic unit the port insists on — while
streaming rather than buffering, so a replay of a million events is a replay and not an
out-of-memory error. The spec for this story covers `Projection`'s associated types and
`apply` signature, `run_projection`'s bounds and its `chunk: NonZeroUsize`, `Progressed` and
`ProjectionError`, the resume arithmetic including the `None` arm of `SequencePosition::next`,
the explicit `unstable-projection` gating with its `doc_cfg` badge and its stated absence of
a semver promise, the `EventStore`-not-`SendEventStore` bound discipline, and the
transactional test's dependency on HS-P0010's `MemoryProjectionStore` recorded as a blocking
input rather than worked around. No `[FROZEN]` clause is amended; the PS-\* clauses this
runner makes answerable are settled by the sibling story that owns them.

## The wrong implementation

**The mutant: a runner that collects the stream before applying it.**

```rust
let events: Vec<SequencedEvent> = collect(store.read(&query, options)).await?;
let mut batch = models.begin().await?;
for e in events { projection.apply(P::Event::decode(codec, …)?, &mut batch)?; }
models.commit(batch, projection.id(), last).await?;
```

It is shorter, it is easier to reason about, and it passes **every check this project can
run**. `MemoryEventStore` holds tens of events in a `Vec`, so the collected version is
indistinguishable from the streaming one in every test; `cargo xtask ci` is green; the
transactional invariant is honoured — one `begin`, one `commit`, no `apply`/`set_checkpoint`
pair; all four `wasm32` steps pass; clippy is silent. Reviewed on its own, it is correct
code.

It is wrong because `EventStore::read` returns the stream **at the top level** rather than
behind an `async fn` for one reason, and that reason is this call site: a projection replay
is the unbounded read in this system, and buffering it converts a rebuild into an
out-of-memory failure proportional to the log. CLAUDE.md's binding constraint 3 and the two
tests in `memory.rs` exist to protect the *shape*; nothing protects the *use*, and this is
the use. It also makes E2E-25's chunked rebuild unwritable, because there are no chunks —
the checkpoint advances once, at the end, so a crash mid-replay redoes everything
(`_decomposition.md:391-398`).

Nothing in this repository fails on it today, and honestly nothing can: the discriminator is
that `run_projection` takes `chunk: NonZeroUsize` and commits per chunk
(`_design.md:557-570`), so the spec must require a test that seeds more events than one
chunk holds and asserts the checkpoint advanced **more than once** — observable through
`Progressed` and through the checkpoint read back between chunks. That test fails against
the collecting runner, and it is the only thing that does.

**A second mutant: resuming from the checkpoint directly.**

```rust
let from = models.checkpoint(projection.id()).await?;
let options = ReadOptions::new().from_opt(from);     // inclusive!
```

Compiles, reads naturally, and passes every test that runs the projection **once**. Because
`ReadOptions::from` is inclusive and the port's own doc says to advance past the checkpoint
first (`crates/happenstance-core/src/projection.rs:101-110`), every restart re-applies
exactly one event: the last one. For a projection that writes `SET name = …` this is
invisible forever. For a projection that writes `count = count + 1` it is silent, permanent
corruption that grows by one per restart. The test that rejects it runs the runner, stops it,
runs it again with no new events, and asserts the read model is unchanged — a test nobody
writes unless the failure is named first.

**A third mutant: synthesising the write/checkpoint pair.** Applying into the batch,
committing it, and then advancing the checkpoint in a second call — or, in the absence of a
`set_checkpoint`, committing an *empty* batch carrying only the new position. It typechecks
against the port and passes in-process, and it reintroduces exactly the two-write window the
port's module doc exists to close: a crash between the two leaves a read model ahead of its
checkpoint, which is re-application, or behind it, which is silent data loss. The port
offers no `apply()`/`set_checkpoint()` pair on purpose
(`crates/happenstance-core/src/projection.rs:112-130`), and a runner that reconstructs one
has defeated the invariant while satisfying the type checker.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

The two judgement boxes. **Literal positions:** no conformance rule is added — the
projection port has no conformance suite yet, and building one is HS-P0010's, explicitly out
of scope here (`project.md:114-116`). This story's own tests do observe positions, since a
checkpoint *is* one, and discovery fixes the rule: every assertion compares against the
position the store actually assigned or against `Progressed`'s reported values, never
against a literal — which matters more here than usual, because gaps are exactly what the
resume arithmetic must survive. **Frozen clauses:** the `ProjectionStore` port is provisional
rather than frozen and this story consumes it unchanged; nothing under
`crates/happenstance-core/src/**` is touched. PS-33, PS-27 and PS-30 are made *answerable*
by this runner's existence but are settled by `projection-clause-verdicts`, and they are
`[PROVISIONAL]`/`[DEFERRED]`, not `[FROZEN]`.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
