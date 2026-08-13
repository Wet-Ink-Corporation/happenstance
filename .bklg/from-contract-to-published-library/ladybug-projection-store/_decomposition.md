# Briefs — The unlike batch shape, and the freeze verdict (HS-P0015)

Companion file, not an item file. It holds this project's warranted briefs
(`architecture`, `testing`), each under its own `## …` heading. Grounding for all
of them is [`_grounding.md`](_grounding.md); the charter is
[`project.md`](project.md).

---

## Architecture brief

### Intent

Turn `happenstance-ladybug` from a type-checking skeleton into an adapter that
has *run* — the real `lbug` driver behind `LadybugProjectionStore`, the
projection conformance suite `projection-store-freeze` ships invoked against a
Ladybug fixture, and a dated verdict on whether the freeze held. The brief's job
is to name the seams that are touched, the upstream contracts this project
consumes rather than invents, the **composition roots each capability must mount
into** so that nothing here is a component behind a stub, and the three tensions
[`_grounding.md`](_grounding.md) left open — settled explicitly, so the story map
does not discover them mid-implementation.

Nothing below prescribes an implementation. It records what is already decided,
what the compiler and the gate will refuse, and where a decision is genuinely the
implementer's (and therefore ADR-0025's).

### Acceptance Criteria

These are architecture-grain; they sit under the project's AC-001 – AC-011 and do
not replace them.

- **AC-A01 — The upstream preflight is asserted, not assumed.** Before any body is
  filled in, the implementer confirms against the merged tree that
  `crates/happenstance-core/src/projection.rs` declares `type Batch;` with no
  lifetime, that the write seam exists, and that
  `projection_store_conformance!` resolves from `happenstance-testkit`. None of
  the three exists today (`crates/happenstance-core/src/projection.rs:97-99`;
  `rg projection_store_conformance crates/` returns nothing). A red preflight
  halts the project — it does not get worked around locally. (Traces project AC-003,
  AC-004; DR-1.)
- **AC-A02 — Every capability mounts into a real composition root named in *Mount
  points* below, and the gate proves it.** In particular the conformance
  invocation is a `tests/` target registered in `xtask/src/proof.rs`'s `ARTEFACTS`,
  and the crate's package-completeness is registered in `xtask/src/package.rs`'s
  `PUBLISHABLE` — not asserted in prose. (Traces AC-004, AC-010.)
- **AC-A03 — The three ADR-0025 questions are each answered against the constraint
  already on record, with its losers named** (see *ADR-0025's three questions*).
  Re-deriving a constraint the skeleton already recorded, or answering a question
  the port has already closed, is a defect in the ADR. (Traces AC-002; DR-6.)
- **AC-A04 — The three tensions below are honoured as decided, or the deviation is
  written into the verdict.** T1 (the skeleton that does not survive the freeze),
  T2 (what AC-006 is actually asking), T3 (`stand_in`'s blast radius). (Traces
  AC-003, AC-006, AC-007.)
- **AC-A05 — `cargo xtask spec-trace` is green *and* no `[FROZEN]` clause text
  moved.** Filling in `projection_store.rs`'s bodies invalidates six `file:line`
  citations in `spec/SPECIFICATION.md`; repairing them is required work in the same
  change, and every one of them sits in non-normative framing prose. (Traces AC-008.)
- **AC-A06 — The CI-shape decision is made from the measured number against the
  levers that actually exist**, which do not include a feature flag (see *Build cost
  is a workspace-wide fact*). (Traces AC-009.)

### Notes

#### 1. Modules and seams touched

Per CLAUDE.md's repository map. Nothing outside this list is this project's to
edit, and two of the entries are edits in *other* crates that the gate forces.

| Path | What changes | Why it is here |
|---|---|---|
| `crates/happenstance-ladybug/src/projection_store.rs` | the four `todo!()` bodies at `:270-292`; `Value` and `Error` re-pointed from `stand_in` to `lbug` | the adapter (DR-1) |
| `crates/happenstance-ladybug/src/live_handle.rs` | retired or re-pointed — see **T1**, **T3** | it imports `crate::stand_in` at `:92` and shares the error enum |
| `crates/happenstance-ladybug/src/stand_in.rs` | deleted | DR-2 |
| `crates/happenstance-ladybug/src/lib.rs` | `#![allow(clippy::todo)]` at `:69-72` deleted; the driver-absent section at `:24-32` rewritten; the open-decisions list at `:51-66` replaced by a link to ADR-0025 | DR-1; the comment already names this phase |
| `crates/happenstance-ladybug/Cargo.toml` | real `lbug` dependency replacing the NOTE at `:18-22`; `publish = false` at `:12` removed | AC-003, AC-010 |
| `crates/happenstance-ladybug/tests/` | a new conformance target; `port_shape.rs` extended, not replaced | AC-004 |
| `crates/happenstance-ladybug/{README.md,LICENSE-MIT,LICENSE-APACHE}` | added | AC-010 |
| `xtask/src/package.rs:86` | `PUBLISHABLE` gains the crate | forced — see *Mount points* |
| `xtask/src/proof.rs:133` | `ARTEFACTS` gains phase 11's row | forced — see *Mount points* |
| `spec/SPECIFICATION.md` | six citations repaired, no clause text | AC-008 |
| `RUNBOOK.md:4411-4444` | phase 11 checkboxes ticked in place, build number recorded | DoD 6 |
| `.kb/decisions/0025-*.md`, `references/adr/0025-*.md`, `.kb/maps/decision-map.md` | ADR-0025 atom, long-form record, map entry | AC-002 |
| `references/evaluation/` | the "structurally unlike" axes, then the verdict | AC-001, AC-007 |

**Not touched:** `crates/happenstance-core/src/projection.rs` (the port is
`projection-store-freeze`'s), `crates/happenstance-testkit/**` (the suite and its
rules are too), and any event store anywhere.

#### 2. What this project consumes rather than defines

Three contracts land upstream and are read, not written. Each is cited by the
clause that specifies it, because the shape that actually ships is
`projection-store-freeze`'s and may differ from the sketch:

- **`type Batch;`, owned, no lifetime** — `spec/SPECIFICATION.md:4868-4883` (PS-5).
  Today's `type Batch<'a> where Self: 'a` at
  `crates/happenstance-core/src/projection.rs:97-99` is the pre-freeze snapshot,
  and the skeleton's `type Batch<'a> = GraphWriteSet where Self: 'a`
  (`projection_store.rs:265-268`) was written so that dropping the lifetime is *a
  deletion here rather than a redesign* — the comment at `:261-264` says so.
- **The write seam.** Generic code can `begin` and `commit` and cannot write
  anything in between; that is the whole of
  `.kb/open-questions/projection-store-batch-has-no-apply-seam.md`, and its
  sub-questions 1 and 2 are what phase 6 answers. Two shapes are in play and this
  project must not guess between them: PS-11 sketches a `ProjectionProbe:
  ProjectionStore` in the contract crate behind `feature = "conformance"`
  (`spec/SPECIFICATION.md:4977-5005`), while HS-P0010's own charter commits
  instead to a **fixture-level** `write_probe(&mut Batch)` plus an out-of-band
  `read_probe(&Store)` ([`../projection-store-freeze/project.md`](../projection-store-freeze/project.md),
  In scope item 4). **Read what shipped.** If neither can express a replayable
  parameterised Cypher statement, that is not an adapter problem to route around —
  it is the freeze not holding, and it goes in the verdict
  ([`_intake-brief.md`](_intake-brief.md), Constraints).
- **`projection_store_conformance!`** — mirroring `event_store_conformance!`
  (`crates/happenstance-testkit/src/lib.rs:311-357`), emitted through the existing
  registry so it inherits the tokio / blocking / wasm emitters unchanged
  (`crates/happenstance-testkit/src/registry.rs:93-103`; the emitter table is
  `lib.rs:63-67`).

#### 3. Mount points — where each capability integrates

This is the section to read before writing anything. Every item is a real file
that already exists and already has a shape; five of them will fail the gate if
this project adds its capability without wiring it in.

**M1 — the conformance run.** A new test target under
`crates/happenstance-ladybug/tests/`, whose whole body is the macro invocation
over a Ladybug fixture, exactly as CLAUDE.md's rule that matters spells it:

> `happenstance_testkit::event_store_conformance!(MyFixture::new());`

with the projection macro's name and a `LadybugFixture`. Not a hand-rolled test
module, and not a subset. The fixture obeys
`crates/happenstance-testkit/src/contract.rs:1-63`'s contract — *one fixture
instance is one isolated backing store; each `connect()` is one handle onto it* —
and `crates/happenstance-testkit/src/fixtures.rs:243-271`'s `MemoryFixture` is the
reference to read first. Isolation is a **rule**, not a convention: the fixture
must hand each instance its own LadybugDB directory, because pointing every
instance at one temporary path is the adapter mistake
`two_fixture_instances_observe_none_of_each_others_appends` exists to catch
(`contract.rs:17-23`).

**M2 — capability declension.** Every rule the fixture cannot satisfy declines
through `Capability::declined("…")` with a real reason and still appears in the
run as a reported skip (`contract.rs:25-63`, `:355-433`; `RuleOutcome` at
`:458-537`). Never `#[cfg]`. Two mechanical traps worth knowing before the first
red run: an empty reason on an *associated* const is caught at **codegen**, so
`cargo clippy` passes and `cargo build`/`cargo test` fail (`contract.rs:404-410`);
and the `SKIP` line is only visible because the gate passes `--show-output`
(`xtask/src/main.rs:131-151`). DR-5 consumes
`.kb/open-questions/cf-40-fixture-limits-ownership.md` — it does not re-decide
where fixture constants live.

**M3 — the compile-time port-shape test.** `crates/happenstance-ladybug/tests/port_shape.rs`
already instantiates generic code at both batch shapes through a `const _` block
(`:75-80`), with `weak_flavour` and `send_flavour` in separate modules because
having both trait names in scope makes every method call `error[E0034]`
(CLAUDE.md constraint 4; the comment is at `:11-14`). **Extend it, do not replace
it.** Its `send_flavour::spawn_a_batch_across_an_await` bound
`for<'a> S::Batch<'a>: Send` (`:61`) collapses to `S::Batch: Send` once the
lifetime is gone, and that collapse is itself a finding for the verdict — PS-5's
predicted ergonomic win, observed (`:50-57`).

**M4 — the proof-artefact registry, `xtask/src/proof.rs:133`.** Phase 11's proof
artefact is *"the projection suite green on a third shape"* (`RUNBOOK.md:4427-4430`),
and **nothing else in the gate would notice if the test target ceased to exist**:
`cargo test` exits 0 on `running 0 tests`, so an emptied file passes the step a
deleted one fails (`xtask/src/main.rs:170-178`). Add an `Artefact` row naming
`package: "happenstance-ladybug"`, the target, and the conformance test names the
verdict cites. This is the difference between a proof artefact and a file.

**M5 — the packaging registry, `xtask/src/package.rs:86`.** `PUBLISHABLE` is a
hand-declared set reconciled against what the manifests actually say
(`package.rs:172-218`). Removing `publish = false` from `Cargo.toml:12` **without**
adding `"happenstance-ladybug"` to `PUBLISHABLE` fails the gate with the message
at `package.rs:188-195`. `REQUIRED_FILES` is `LICENSE-MIT`, `LICENSE-APACHE`,
`README.md` (`package.rs:94`), and they must be **copied into the crate
directory** — Cargo will not follow a path outside it (`package.rs:152-157`).
`crates/happenstance-core/` is the layout to copy, including the explicit
`readme = "README.md"` key and its rationale (`crates/happenstance-core/Cargo.toml:13-16`).
Note the ordering consequence: AC-010 makes the crate *ready*; whether it is
published at `0.2.0` is `publication-and-positioning`'s.

**M6 — the gate itself, `xtask/src/main.rs`.** `clippy` and `tests` both run
`--workspace --all-features` (`:116-129`, `:143-151`), and the workspace is
`members = ["crates/*", …]` (`Cargo.toml:3`). The `lbug` dependency therefore
lands in **every** contributor's `cargo xtask ci` the moment it is added. See
*Build cost* below — this is AC-009's composition root, and it is a step-args
decision in this file plus a job in `.github/workflows/ci.yml:31-40`, not a
feature flag.

**M7 — `spec/SPECIFICATION.md`'s citations.** `cargo xtask spec-trace` resolves
every `file:line` citation and additionally checks the cited line is within twelve
lines of the subject the sentence attributes to it
(`xtask/src/spec_trace.rs:291-372`, tolerance at `:374-378`). Six citations name
lines this project moves: `:372` (the status table),
`:4590`, `:4592`, `:4605`, `:4612` (§4's framing prose) and `:8095` (the portfolio
table). All six are **non-normative** — the nearest clause headings are §4.1a at
`:4791` and §6.5 at `:7985`, and none of the six is inside a `[FROZEN]` clause
body — so repairing them is permitted and required. Repair them in the same commit
as the bodies; do not touch a clause marker or a clause sentence (AC-008).

**M8 — the knowledge base.** ADR-0025 is authored as a `.kb/decisions/` atom with
valid `KbFrontmatter` **and** a long-form record under `references/adr/`, per
CLAUDE.md's two-places rule: link the atom, cite the record by `file:line`. Reach
it from `.kb/maps/decision-map.md`. Atoms are authored by `/redkiln:kb-ingest`
from `.kb/_intake/`, never by hand — CLAUDE.md says why, and names the commit
(`0269720`) where hand-writing them was reverted.

**M9 — the two documents.** `references/evaluation/` is where dated, commit-pinned
evidence lives, and it already carries this exact genre in
`phase-4-reconciliation.md` and `phase-4-5-reconciliation.md`; its README states
the lifecycle — *"immutable evidence… dated, pinned to a commit… must be
superseded rather than edited"* (`references/evaluation/README.md:1-13`). The
"structurally unlike" axes and the freeze verdict go there, as two files in two
commits, because AC-001 is an **ordering claim about commits** and one file
amended in place cannot make it.

#### 4. The data flow, once

One `commit` is the whole adapter, and its shape is already argued in the module
docs — do not re-derive it, and do not change it without saying so in ADR-0025.

A runner calls `begin()`, which yields an owned, empty `GraphWriteSet`
(`projection_store.rs:105-119`): no connection, no transaction, no borrow, so it
is `Send + 'static` and survives an await. The projection appends parameterised
`GraphStatement`s (`:70-103`) — parameters carried *beside* the text, because
replay goes through `prepare`/`execute` and a stringified statement cannot be
prepared once and executed many times (`:72-75`). Nothing has touched the database
yet. `commit(batch, id, position)` opens one connection via `connect()`
(`:238-249`), runs `BEGIN TRANSACTION`, replays the statements in order, writes
the checkpoint as **the last statement before `COMMIT`**, and commits. The
checkpoint statement is already written out, in the instrument:
`MERGE (c:ProjectionCheckpoint {id: $id}) SET c.position = $position`
(`live_handle.rs:203-217`), with the `i64::try_from` narrowing that
`PositionOutOfRange` exists for.

That single transaction *is* PS-1 — read-model write and checkpoint write in one
transaction, the invariant `crates/happenstance-core/src/projection.rs:13-30`
states and the only reason the port has this shape.

Three properties of this flow are load-bearing and easy to erode:

- **The store owns its `Database` and connects per call.** `Connection::new` takes
  `&'db Database`, so a struct holding both is self-referential; and a store that
  carries a lifetime **ICEs rustc 1.97.1** rather than diagnosing the region error
  (`projection_store.rs:24-37`, transcript at `live_handle.rs:38-66`, minimised in
  `experiments/rustc-ice-gat-foreign-trait/`). Per-call connections are also
  LadybugDB's own documented pattern.
- **The `INT64` ↔ `NonZeroU64` narrowing is checked in both directions.**
  `MalformedCheckpoint` and `PositionOutOfRange` (`projection_store.rs:175-202`)
  are not defensive clutter: collapsing a corrupt checkpoint into `Ok(None)`
  *"would silently replay a projection from the beginning"*. Do not simplify
  either away while filling in `checkpoint`.
- **`WriteTransactionInUse` is routine.** LadybugDB permits many readers and
  exactly one writer (`:204-213`), so any concurrency-shaped rule will meet it.
  Whether that is a declared capability limit or a rule defect is a question for
  the verdict; the answer is **never** "retry until green" (project.md, Risks).

#### 5. ADR-0025's three questions, and what already constrains each

DR-6 requires the losers named **per question**, not once for the record.

**Q1 — checkpoint placement: a node in the graph, or beside it.** Nearly settled
already, and the argument is recorded: as a node property is *"the only way to
satisfy the port's transactional invariant"* because it is what keeps the write
inside the same `BEGIN TRANSACTION` (`lib.rs:53-58`). The loser to name is the
sidecar — a file or table outside the graph — and the reason it loses is PS-1,
not preference. The residue that is genuinely open is the narrowing above, plus
whether the checkpoint node is per-`ProjectionId` or one node with a property per
id (PS-23 asks whether one commit may advance two ids —
`spec/SPECIFICATION.md:5317`; that clause is `projection-store-freeze`'s, and
this adapter is a data point for it, per `spec/SPECIFICATION.md:4814`).

**Q2 — the graph-mutation vocabulary: raw Cypher, or a typed builder.**
`GraphStatement` is the raw-Cypher answer and it is deliberately *the port's*
answer too: PS-9 and PS-11 ask whether generic code needs a write vocabulary on
`Batch`, *"and this crate is one of the two data points"* (`lib.rs:63-66`;
clauses at `spec/SPECIFICATION.md:4947`, `:4977`). The binding constraint is
`.kb/decisions/0008-one-derivation-for-both-ports.md` — a provided body cannot
hold the `Batch` GAT across a suspension point under any remedy tried — which
rules out shapes before they are weighed. Losers to name: a typed builder (what
it buys, and what it costs a projection that needs Cypher the builder does not
model), and a bound on `Batch` versus a method on the port versus a second
associated type, which is sub-question 1 of
`.kb/open-questions/projection-store-batch-has-no-apply-seam.md`. Keep whatever
is chosen compatible with ADR-0007's indicative `Projection::apply` signature
(`.kb/decisions/0007-projection-runner-decodes.md`) — the runner is
`typed-layer-and-alpha-release`'s, and an incompatible vocabulary here is a bill
that arrives there.

**Q3 — `lbug` blocks and the port does not.** The skeleton records that the
owned-`Database` layout leaves both options open and why neither was taken:
`spawn_blocking` is *available* because the store is `'static` and can build a
connection inside the closure, *"but it is not used, because reaching for it
would put a tokio dependency in a runtime-agnostic adapter"*
(`projection_store.rs:39-47`). Three candidates, each with a real cost to state:
a runtime-gated `spawn_blocking` feature; blocking the executor thread and
documenting it; offering a blocking-only adapter. Two hard constraints frame the
choice — **`#[async_trait]` is never an option** (ADR-0001,
`.kb/decisions/0001-async-port-flavours.md`: it injects `+ Send` and makes wasm32
impossible), and the testkit already ships a `__emit_blocking` harness needing no
async runtime at all (`crates/happenstance-testkit/src/lib.rs:63-67`), so "the
suite needs tokio" is not an argument for any of them. Note `Cargo.toml:24-25`
already carries tokio in dev-dependencies, which is fine and is not a precedent
for a normal dependency. The `Send` flavour itself is **not** open: it is chosen
on the evidence that `lbug`'s `Database` and `Connection` are `Send + Sync`
(`projection_store.rs:252-257`), and BR-12's `!Send` proof belongs to
`cloudflare-durable-object-store`.

#### 6. The three tensions, settled

**T1 — `type Batch;` is a hard precondition, and `live_handle.rs` does not
survive it.** `projection-store-freeze`'s charter claims *"the Ladybug and
Postgres skeletons compile with no change other than the lifetime parameter's
removal — same `Batch` type, same error type, same bodies"*
([`../projection-store-freeze/project.md`](../projection-store-freeze/project.md),
In scope item 10). That is true of `LadybugProjectionStore`, whose `Batch` is the
owned `GraphWriteSet`. It is **false** of `LiveHandleProjectionStore`, which binds
`type Batch<'a> = GraphWriteHandle<'a>` (`live_handle.rs:177-180`) — a genuinely
borrowed handle with no lifetime-free spelling for a `'static` store. Under a
frozen `type Batch;` that impl cannot exist. Two consequences the implementer
inherits: (a) surface this to HS-P0010 rather than absorbing it, because the crate
this project starts from will otherwise not compile; (b) the instrument's
retirement is itself a verdict finding — *the freeze deletes the counter-example
that was evidence for freezing* — and it costs nothing evidentially, because both
transcripts are preserved outside the crate in
`references/adapter-shapes.md:169-195` and `:363-367` and quoted in
`spec/SPECIFICATION.md:4600-4615`. `port_shape.rs:77,79` instantiate generic code
at `LiveHandleProjectionStore` and must drop those lines with it.

**T2 — AC-006 is a *disposition* claim, not a reproduction.** PS-34 is marked
`[PROVISIONAL — contingent on PS-5; dead the moment `type Batch;` lands]`
(`spec/SPECIFICATION.md:5546-5560`), and PS-5 is exactly what HS-P0010 plans to
land. So AC-006 has two mutually exclusive discharges, and the implementer takes
whichever the merged port dictates:

- *If `type Batch;` landed* — the honest outcome is **"the trap is retired"**, not
  a reproduction. The record states: the third implementer wrote the impl with the
  concrete owned type, met no `error[E0195]`, and the `where Self: 'a` bound was
  absent from every impl — which is PS-5's own predicted relief
  (`spec/SPECIFICATION.md:4877-4883`), observed by someone who did not argue for
  it. Note the second, quieter half: `references/adapter-shapes.md:186-191` records
  that binding an *owned* type to the **GAT** bought none of that relief, so the
  observation is not vacuous.
- *If the GAT survived* — PS-34 is binding, the trap is live, and the re-test is
  literal: write the impl with the concrete type, capture the diagnostic, and
  record whether the documented remedy (PS-34's required doctest, plus
  `MemoryProjectionStore` as the thing to copy) was sufficient.

Either way, **who** counts as the third implementer is a constraint on the story,
not a formality: the first two pairs of hands wrote `happenstance-sqlite` and
`live_handle.rs` in phase 2. The AC-006 work must be done by a context that has
**not** read `live_handle.rs:68-83` first, and the record must state what
documentation it did have. And this project **reports** on PS-34; moving PS-34's
marker is `projection-store-freeze`'s (`RUNBOOK.md:602`, "6, re-tested 11"). If
the verdict implies the marker should move, that routes through a decision atom
and a re-plan (project.md, Out of scope), not an edit here.

**T3 — `stand_in` retires from the whole crate, `live_handle.rs` included.** The
narrow reading of AC-003 ("no `stand_in` type on any path the *suite* exercises")
would exempt the instrument, and it is wrong here for a mechanical reason rather
than a stylistic one: the error enum is **shared**. `LadybugProjectionStoreError`
carries `Driver(#[from] stand_in::Error)` (`projection_store.rs:158-163`) and
`live_handle.rs:91` imports that same enum. The moment `Driver` wraps `lbug`'s
error, `live_handle.rs` stops compiling — the two modules cannot sit in different
type universes while sharing one error type, and `projection_store.rs:152-157`
says the sharing is deliberate. `port_shape.rs`'s `const _` block instantiates
both stores on a path `cargo xtask ci` runs, so "the suite never touches it" is
not even true of the gate. DR-2's stronger reading therefore wins on a compile
argument: `stand_in.rs` is deleted, and `live_handle.rs` is retired with it
(**T1**) or re-pointed at `lbug` in the same change.

#### 7. Build cost is a workspace-wide fact, and a feature flag does not fix it

AC-009 has no precedent to copy — nothing else in this workspace builds C++ — so
the design is from first principles, but the *constraints* are checkable facts,
not estimates:

- `clippy` and `tests` in the gate are `--workspace --all-features`
  (`xtask/src/main.rs:116-129`, `:143-151`), and `cargo hack check --workspace
  --feature-powerset` runs above them when the tool resolves (`:546-556`, and
  CLAUDE.md notes `cargo-hack` **does** resolve on this machine). **Putting `lbug`
  behind an off-by-default feature therefore saves nothing**: every one of those
  steps turns it on. `default-members` does not help either — `--workspace`
  selects all members regardless.
- The levers that actually exist are: `--exclude happenstance-ladybug` on the
  shared gate steps plus a dedicated step or CI job; or accept the cost in the
  three-OS matrix (`.github/workflows/ci.yml:31-40`). The trade is stated in
  project.md's risk register — left in the gate it taxes every contributor on
  every commit; split out, it can silently stop running — so whichever is chosen,
  the *other* failure mode is what the recorded rationale must address.
- **Measure before deciding, and record the number** (`RUNBOOK.md:4420-4425`,
  `:4437`). A defensible protocol: a genuinely cold build (fresh
  `CARGO_TARGET_DIR`, `cargo build -p happenstance-ladybug --locked`) on each of
  the three matrix runners, plus one warm incremental figure, each stamped with
  toolchain and machine — the same discipline `references/evaluation/README.md`
  requires of any measurement kept as evidence.
- **docs.rs is a known, separate failure.** `lib.rs:32` records that docs.rs
  itself fails to build `lbug` 0.19.1. Removing `publish = false` makes that a
  real consequence rather than a note, and it is `publication-and-positioning`'s
  call (`RUNBOOK.md:162` names docs.rs green under `--all-features` as phase 12's
  proof). This project's obligation is to hand that project the number and the
  `[package.metadata.docs.rs]` question, modelled on
  `crates/happenstance-core/Cargo.toml:54-56`.

#### 8. Deliberately not prescribed

Left to the implementer, and to ADR-0025 where a decision is owed:

- The Cypher schema for read models and for the checkpoint node — label names,
  property names, indexes. Only the *transaction membership* is fixed (Q1).
- Whether `GraphWriteSet` grows a typed constructor surface or stays a `Vec` of
  raw statements (Q2), and whether `GraphStatement::parameters`' `Vec<(Box<str>,
  Value)>` survives contact with `lbug`'s real value type.
- The `LadybugFixture`'s isolation mechanism (temp directory strategy) and its
  answers to the projection fixture's capability constants — as long as every
  declension carries a real reason (M2).
- Which projection AC-005's read-your-own-writes case models. What is fixed is the
  *shape* of the question — a projection that `MATCH`es a node it created earlier
  in the same batch (`projection_store.rs:57-64`) — and that the answer is
  recorded as a supported behaviour or a stated capability limit, never as a
  hypothesis. PS-12 is where that answer is owed
  (`spec/SPECIFICATION.md:5052-5075`), and it names Ladybug as its candidate
  falsifier; §4.1a's question 1 gates PS-4, PS-5, PS-6, PS-12 and PS-34 together
  and names *"the Ladybug skeleton"* as the confirming instrument
  (`spec/SPECIFICATION.md:4809`).
- Whether AC-005's projection also makes E2E-19 and E2E-24's third-shape halves
  writable in this project or merely writable (`RUNBOOK.md:4440`;
  `spec/E2E-CASES.md:502-520`, `:622-640`).

#### 9. Standing prohibitions this project is most likely to trip

- Never `#[async_trait]` (ADR-0001), never `serde` in `happenstance-core`'s
  defaults (ADR-0003) — neither is plausibly needed here, both are workspace law.
- Never assert on literal position values; the specification permits gaps
  (CLAUDE.md, *The rule that matters*).
- Never `#[cfg]` a rule out to make a build green (M2).
- Never edit an accepted decision atom's body — supersede it (CLAUDE.md, *Where
  the work lives*); `redkiln validate --kb` checks this against `HEAD`.
- Never hand-edit `.bklg/**` frontmatter; the CLI is the only writer, and a
  `PreToolUse` hook denies it.
- Never run `redkiln adopt --templates` (CLAUDE.md).
- Do not weaken `port_shape.rs` to make it compile; if a bound has to change, the
  change is the finding.

## Testing brief

### Intent

Where each of this project's eleven ACs is proven, and with what commands. This
project's central testing fact is that its most important tier is a **conformance
suite it does not write** — `projection_store_conformance!`, mirroring
`crates/happenstance-testkit/src/lib.rs:311-357`'s `event_store_conformance!`
shape — invoked here for the first time against a batch shape unlike anything
that froze the port (no transaction handle type, a deferred `Send + 'static`
write set, a synchronous driver, single-writer concurrency). The suite is the
oracle; this brief's job is to say which tier catches a regression the suite
cannot see (a `todo!()` left in, a `stand_in` type leaking, a citation gone
stale) and which fixture/seam decisions the implementer owns versus inherits.
Traces to [`project.md`](project.md) AC-001 – AC-011 and its Definition of
done; does not restate their text.

### Acceptance Criteria

One row per project AC-###, mapped to the tier(s) that prove it.

| AC | Tier | How it is proven |
| --- | --- | --- |
| **AC-001** — "structurally unlike" defined before the first run | **Static / process** | A commit under `references/evaluation/` (per M9 above) naming the axes, at a commit that `git log` shows precedes the first commit touching a `projection_store_conformance!` invocation against a Ladybug fixture. Checked by commit order, not by content review — DR-3's whole point is that the definition cannot be dated after the result. |
| **AC-002** — ADR-0025 accepted, three questions answered with losers named | **Static** | `redkiln validate --kb` against `.kb/decisions/0025-*.md`; reachability confirmed from `.kb/maps/decision-map.md`. Content check (each of Q1/Q2/Q3 names a rejected alternative and why) is a review-tier check, not automatable — the reviewer reads the atom against *ADR-0025's three questions* (architecture brief §5) and the losers named there. |
| **AC-003** — no `todo!()`, no `stand_in` on any suite-exercised path, builds against real `lbug` | **Static + Integration** | `grep -rn "todo!" crates/happenstance-ladybug/` returns nothing (static, part of the merge-gate below); `#![allow(clippy::todo)]` absent from `lib.rs` (`cargo clippy --workspace --all-targets --all-features -D warnings` fails loud if a `todo!()` remains, since the crate-level allow is gone); `cargo build -p happenstance-ladybug --locked` against the real `lbug` dependency (integration — this is the step that pays AC-009's cold-build cost). "No `stand_in` on any path the suite exercises" is proven by `stand_in.rs` no longer existing in the tree (T3: the file is deleted, not narrowed), which `port_shape.rs`'s `const _` block would fail to compile against if a caller still referenced it. |
| **AC-004** — suite runs against a Ladybug fixture, every rule reports pass or a reasoned declension | **Integration** | The `projection_store_conformance!(LadybugFixture::new())` test target (M1) run with `--show-output` (`xtask/src/main.rs:131-151`, the flag that makes `SKIP` lines visible); rule count in the run equals the count `projection-store-freeze` registered (a diff against that project's own suite-registration list, not a guess). Every `Capability::declined("…")` carries a non-empty reason — the empty-reason trap on an associated const is a **codegen**-tier failure (`contract.rs:404-410`), so `cargo build`/`cargo test` catch it even though `cargo clippy` will not. |
| **AC-005** — read-your-own-writes inside one batch answered | **Integration** | One conformance-suite rule (or, if `projection-store-freeze` does not register one generic enough, a fixture-level test alongside it, but the suite invocation is the primary evidence per M1's "not a subset" instruction) exercising a projection that `MATCH`es a node created earlier in the same batch (`projection_store.rs:57-64`). Outcome recorded as pass or a declined capability with a stated reason — never silently absent. This is also the finding referenced in the architecture brief's *Deliberately not prescribed* §8, item 4: PS-12 is the clause it feeds. |
| **AC-006** — PS-34's E0195 trap re-tested by a third implementer | **Integration (compile-fail) + process** | A `trybuild`-style or manual `cargo build` capture of the impl written against whatever `Batch` shape merged, by a context that has **not** read `live_handle.rs:68-83` first (T2's constraint on *who*, not just *what*). Two mutually exclusive, both-valid outcomes per T2: if `type Batch;` (no lifetime) landed, the recorded outcome is "trap retired — no `error[E0195]`, no `where Self: 'a` bound in any impl," citing `references/adapter-shapes.md:186-191`'s note that binding an owned type to the *GAT* bought none of that relief, so retirement is not a vacuous claim; if the GAT survived, the outcome is the literal re-test — diagnostic captured, checked against PS-34's documented remedy. Either way this is process evidence (a dated record naming the implementer's prior context) as much as a compiler transcript, because the AC is about whether documentation was *sufficient for someone*, which a green build alone does not establish. |
| **AC-007** — freeze verdict exists, held or not | **Static / process** | A dated document in `references/evaluation/` (M9) exists and is non-empty; content check is that it names the implementation, the rules run, the PS clause ids (PS-2, PS-4 – PS-6, PS-9, PS-11, PS-12, PS-15, PS-34 per project.md's "How this advances the initiative" and PS-3 as a data point) and the commit SHA. Reviewed, not scripted — "held either way" is a presence check the merge gate cannot fabricate on its own. |
| **AC-008** — `[FROZEN]` clause text and markers unmoved; six citations repaired | **Static** | `cargo xtask spec-trace` green, part of the merge-gate below — it resolves every `file:line` citation and checks the cited line sits within twelve lines of the sentence's subject (`xtask/src/spec_trace.rs:291-372`, tolerance `:374-378`). `git diff <project-start-commit>..HEAD -- spec/SPECIFICATION.md` reviewed by hand to confirm the six repaired citations (M7's list) are line-number-only edits, with no `[FROZEN]` clause body or marker text touched. |
| **AC-009** — build cost measured, CI shape decided and recorded | **Process (measurement), not a suite tier** | Not proven by a test at all: a genuinely cold build (fresh `CARGO_TARGET_DIR`, `cargo build -p happenstance-ladybug --locked`) on each of the three matrix runners plus one warm incremental figure, each stamped with toolchain and machine (architecture brief §7, the same discipline `references/evaluation/README.md` requires of kept evidence). The CI-shape decision itself — `--exclude happenstance-ladybug` plus a dedicated job, versus accepting the cost in `.github/workflows/ci.yml:31-40`'s three-OS matrix — is recorded in `RUNBOOK.md`'s phase 11 body (DoD 6) and is a design decision this brief does not adjudicate; see architecture brief §7. |
| **AC-010** — name held, crate package-complete | **Static** | `cargo package --list -p happenstance-ladybug` — the same assertion `xtask/src/main.rs:519` runs, checked here in isolation rather than only inside the whole-gate run — confirms `LICENSE-MIT`, `LICENSE-APACHE`, `README.md` are present *in the crate directory* (`xtask/src/package.rs:94`, `:152-157`: Cargo will not follow a path outside it). `xtask/src/package.rs:86`'s `PUBLISHABLE` set containing `"happenstance-ladybug"` is checked by the same `cargo xtask ci` run failing with the message at `package.rs:188-195` if it is missing while `publish = false` is gone (M5). crates.io name-claim itself is outside any test tier — a manual, one-time `cargo publish --dry-run` plus the actual claim, recorded in the verdict or `RUNBOOK.md`. |
| **AC-011** — verdict merged before `publication-and-positioning` opens | **Process** | `git log` timestamp/commit-order check: the verdict document's merge commit precedes the commit at which `publication-and-positioning`'s gate opens. Not a CI-enforced check (no such gate exists in `.redkiln/config.yaml`) — a human-checked ordering claim recorded in this project's closeout, the same discipline AC-001 uses for its own commit-order proof. |

### Notes

**Test mix, summarised by tier.**

- **Static** — `cargo fmt --check`, `cargo clippy --workspace --all-targets
  --all-features -D warnings` (the crate-level `#![allow(clippy::todo)]`
  removal is what makes this tier actually catch a forgotten `todo!()`;
  with the allow still in place clippy would stay silent, which is exactly
  why DR-1 requires deletion, not narrowing), `cargo xtask spec-trace`,
  `grep -rn "todo!" crates/happenstance-ladybug/`, `cargo package --list -p
  happenstance-ladybug`, `redkiln validate --kb`, `redkiln doctor`. These
  prove AC-002 (partially), AC-003 (partially), AC-008 and AC-010.
- **Unit** — `cargo test --workspace --all-features` picks up
  `happenstance-ladybug`'s own unit tests (the `INT64`/`NonZeroU64`
  narrowing paths for `MalformedCheckpoint`/`PositionOutOfRange`,
  `projection_store.rs:175-202`) alongside every other crate's existing
  suite. This project adds unit coverage for the narrowing logic but its
  primary proof obligation sits one tier up.
- **Integration** — the `projection_store_conformance!` invocation (M1),
  `port_shape.rs`'s extended compile-only `const _` blocks (M3, both
  `weak_flavour` and `send_flavour` modules kept separate per CLAUDE.md
  constraint 4), and the cold build against the real `lbug` dependency.
  This is the tier that proves AC-003 (build), AC-004, AC-005 and half of
  AC-006.
- **E2E / process** — the freeze verdict document (AC-007), the
  commit-order proofs (AC-001, AC-011), the ADR-0025 content review
  (AC-002), and the build-cost measurement protocol (AC-009). None of these
  is a test-framework assertion; each is evidence assembled and dated the
  same way `references/evaluation/README.md` already requires of anything
  kept as evidence — "immutable, dated, pinned to a commit, superseded
  rather than edited." Treating a process artefact as if a green CI run
  substitutes for it is the failure mode DR-4 exists to name (a verdict
  that says only "held" is not a verdict).

**Merge-gate commands.** The whole gate, run on this project's tree, per
CLAUDE.md's *Commands* section — this project does not get a `--fast` bar
because it is the one place in the portfolio the wasm32 / cargo-hack /
package-completeness / spec-trace steps below are freshly exercised against a
*new* dependency graph (`lbug`'s `cxx`/`cmake` build):

```console
cargo xtask ci                          # the whole gate: fmt, clippy -D warnings,
                                         #   tests (incl. the conformance suite),
                                         #   4 wasm32 steps, docs, spec-trace,
                                         #   --no-default-features doc build,
                                         #   cargo package --list, cargo-hack,
                                         #   cargo-deny, nightly docsrs (tool-gated)
cargo xtask spec-trace                  # isolated re-run when repairing AC-008's
                                         #   six citations, before the whole gate
redkiln validate --kb && redkiln doctor # ADR-0025's atom, AC-002
```

`cargo xtask ci --fast` is not this project's bar for the same reason
`closeout-and-durable-audience`'s testing brief gives for its own terminal
gate: a subset already met by every sibling proves nothing new. Here the
inverse risk is sharper — `--fast` may not even exercise the newly-added
`lbug` dependency's build path, which is the one thing AC-009 needs measured.

**Fixtures / seams.**

- **`LadybugFixture`** — the one fixture this project writes. It must obey
  `crates/happenstance-testkit/src/contract.rs:1-63`'s "one fixture instance
  is one isolated backing store; each `connect()` is one handle onto it,"
  the same contract `crates/happenstance-testkit/src/fixtures.rs:243-271`'s
  `MemoryFixture` demonstrates and the reference to read first (M1). Its
  isolation mechanism is a **temp-directory-per-instance** strategy — pointing
  every fixture instance at one shared LadybugDB path is the exact mistake
  `two_fixture_instances_observe_none_of_each_others_appends`
  (`contract.rs:17-23`) exists to catch, and that rule is inherited, not
  written here, so a fixture that fails it fails an existing rule rather than
  a new one.
- **`SECOND_HANDLE` / `REOPEN` capability constants** — declared per
  `Capability` (`contract.rs:25-63`); every declension must carry a real,
  fixture-specific reason (M2, DR-5), consuming
  `.kb/open-questions/cf-40-fixture-limits-ownership.md` rather than
  re-deciding where the constants live. LadybugDB's single-writer rule
  (`projection_store.rs:204-213`) is the most likely source of a genuine
  declension here — `WriteTransactionInUse` under a concurrency-shaped rule
  is a candidate for a stated capability limit, not a bug to route around
  with a retry loop (project.md, Risks; also flagged in the architecture
  brief §5, Q3 area).
- **Nothing from `stand_in.rs` is a seam to keep.** It is deleted (DR-2, T3),
  not mocked forward — this project's fixture talks to the real `lbug`
  driver from the start; there is no double standing in for it once the
  bodies are filled in.
- **`live_handle.rs`'s instrument role is not a test seam** — it is retired
  or re-pointed (T1/T3), and the E0195/ICE transcripts it produced are
  preserved as evidence in `references/adapter-shapes.md:169-195`, `:363-367`
  and quoted in `spec/SPECIFICATION.md:4600-4615`, not re-run from a live
  fixture. AC-006's re-test is a fresh compile against whatever port shape
  merged, by a fresh implementer — not a re-execution of the old instrument.
- **No mocking of `lbug` itself.** The whole point of this project (CLAUDE.md,
  *The rule that matters*: "An adapter that compiles but has not run the
  suite is not an adapter") is that the suite runs against the real driver;
  substituting a double for `lbug` anywhere on the suite-exercised path would
  falsify AC-004's own premise the same way `closeout-and-durable-audience`'s
  testing brief refuses to mock anything it claims to have re-run.
