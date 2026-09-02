---
item: HS-S0077
stage: spec
created: 2026-08-12T13:47:15.210Z
updated: 2026-08-12T13:47:15.210Z
template_sig: 87bbf1d0
rendered_sig: 778f6688
---

# Spec — Fill the four bodies, delete the allow, record PS-34's disposition

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project charter | `.bklg/from-contract-to-published-library/ladybug-projection-store/project.md` |
| This spec | `.bklg/from-contract-to-published-library/ladybug-projection-store/fill-the-bodies-and-ps-34-disposition/spec.md` |
| Key brief — architecture | `.bklg/from-contract-to-published-library/ladybug-projection-store/_decomposition.md` (*Architecture brief*: seams table, mount points M1–M9, ADR-0025's three questions, tensions T1–T3) |
| Key brief — testing | `.bklg/from-contract-to-published-library/ladybug-projection-store/_decomposition.md` (*Testing brief*: the AC→tier table, merge-gate commands, fixtures/seams) |
| Story map (this story's row) | `.bklg/from-contract-to-published-library/ladybug-projection-store/_storymap.md` (slice `real-adapter`; *PS-34, and who is allowed to write it*) |
| Signed-off design | `.bklg/from-contract-to-published-library/ladybug-projection-store/_design.md` — **no user-facing surface**; approved 2026-08-12 as a no-surface determination. `## Items`, `## Signatures` and `## The doctest` are all `N/A`, so this story renders no surface and takes its API obligations from the port and from the phase-2 skeleton's own rustdoc instead |
| Roadmap pointer | `RUNBOOK.md:4393-4444` (phase 11 goal, work list, proof artefact, exit criteria); `RUNBOOK.md:588-610` (the provisional ledger, and PS-34's row marked "6, re-tested 11") |

## One-line PR slice

Fill the four `projection_store.rs` bodies against the real driver, delete
`#![allow(clippy::todo)]`, unit-test the `INT64`/`NonZeroU64` narrowing both ways, repair the six
`SPECIFICATION.md` citations in the same commit, and record PS-34's disposition from a context that
has not read the old transcript.

## Executive summary

`real-lbug-driver-swap` (HS-S0076) leaves the crate compiling against the real `lbug` types with
four `todo!()` bodies still in place and `#![allow(clippy::todo)]` still at
`crates/happenstance-ladybug/src/lib.rs:69-72`. **This PR is the delta that makes those bodies
real** — `checkpoint`, `begin`, `commit`, `rollback` on
`impl SendProjectionStore for LadybugProjectionStore`
(`crates/happenstance-ladybug/src/projection_store.rs:270-292`) — and then deletes the allow, which
is what re-arms the workspace-wide `clippy::todo` deny over this crate.

Three obligations ride with it because nothing else can carry them:

- **The narrowing gets its own unit tests.** `MalformedCheckpoint` and `PositionOutOfRange`
  (`…/projection_store.rs:175-202`) exist to stop a corrupt checkpoint collapsing into `Ok(None)`
  and silently replaying a projection from the beginning. Once the bodies exist there is a code path
  to test; before this PR there is not.
- **Six `spec/SPECIFICATION.md` citations break the moment these lines move**, and
  `cargo xtask spec-trace` is a gate step — so a bodies-only merge leaves the gate red. They are
  repaired here, in the same commit.
- **PS-34's disposition is captured here or not at all.** AC-006 is a claim about whether the
  documented diagnostic was sufficient *for someone meeting the port fresh*. Freshness cannot be
  reconstructed after the impl is written, which is why the record is taken inside this story rather
  than by a later one.

This PR does **not** run the conformance suite — no fixture exists yet
(`ladybug-fixture-and-conformance-run`, HS-S0078) — and does not touch packaging or CI shape. What
it delivers is the thing that story needs to exist: a `LadybugProjectionStore` whose methods do
something when called.

## Context pack

Read this section before opening anything. Everything deeper is behind a signposted anchor.

### The decision that shapes every body: one `commit`, one transaction, checkpoint last

The whole adapter is one method. **Do not re-derive this flow and do not change it without saying so
in ADR-0025** — it is already argued in `crates/happenstance-ladybug/src/projection_store.rs`'s
module docs and the four bodies exist to execute it, not to reopen it:

`begin()` yields an owned, empty `GraphWriteSet` (`…/projection_store.rs:105-119`) — no connection,
no transaction, no borrow, therefore `Send + 'static` and safe to hold across an await. A projection
appends parameterised `GraphStatement`s (`…:70-103`); **parameters are carried beside the text, never
interpolated into it**, because replay goes through `prepare`/`execute` and a stringified statement
cannot be prepared once and executed many times. Nothing has touched the database yet.
`commit(batch, id, position)` opens one connection via `connect()` (`…:238-249`), executes
`BEGIN TRANSACTION`, replays the statements **in order**, writes the checkpoint as **the last
statement before `COMMIT`**, and commits.

That single transaction *is* PS-1 — read-model write and checkpoint write in one transaction, the
invariant `crates/happenstance-core/src/projection.rs:13-30` states and the only reason the port has
this shape. A `commit` that writes the checkpoint on a second connection, or outside the
`BEGIN`…`COMMIT` pair, satisfies the trait and violates the specification.

### The decision about narrowing: a corrupt checkpoint is not a missing one

LadybugDB's widest integer property is `INT64`; `SequencePosition` is a `NonZeroU64`. The round trip
is a **checked conversion in both directions**, and both error variants are load-bearing:

- reading back `0` or a negative value is `MalformedCheckpoint`, **not** `Ok(None)` — collapsing them
  "would silently replay a projection from the beginning" (`…/projection_store.rs:175-191`);
- writing a position above `i64::MAX` is `PositionOutOfRange` — unreachable in practice and cheap to
  state, "which is the correct trade for a conversion that would otherwise want an `unwrap`"
  (`…:192-202`).

Neither may be simplified away while filling in `checkpoint` and `commit` (project charter, *Risks
and coupling notes*). Put the narrowing in **free functions** so it is unit-testable without a
LadybugDB instance — `happenstance-neon` already established that shape, with `decode_checkpoint` and
`decode_commit` as free functions rather than port methods (`spec/SPECIFICATION.md:8095`).

### The decision about contention: `WriteTransactionInUse` is routine, and retrying is forbidden

LadybugDB permits many concurrent readers and exactly one writer, so two commits racing is *routine
rather than exceptional* — which is why the variant exists at all rather than arriving as an opaque
`Driver` string (`…/projection_store.rs:204-213`). The body's job is to **map it and hand it back**.
An internal retry loop inside `commit` is explicitly out (project charter, *Risks*: the answer "must
not be 'retry until green'"), because it would convert a declarable capability limit into a hidden
timing behaviour and rob the freeze verdict of the finding.

### The decision about who writes PS-34's disposition, and in what order

AC-006 constrains **who**, not only what: the first two pairs of hands wrote `happenstance-sqlite`
and `live_handle.rs` in phase 2, so the record must come from a context that has **not** read the old
E0195 transcript first, and **must state what documentation it did have**
(`_decomposition.md`, T2/T3; `_storymap.md`, *PS-34, and who is allowed to write it*).

**This spec therefore deliberately does not quote the transcript**, and neither should any note the
implementer writes before the capture. The sequencing inside this PR is a hard constraint:

1. Write the four bodies against whatever `Batch` shape actually merged, using only the port's own
   rustdoc, ADR-0025, and `MemoryProjectionStore` if `projection-store-freeze` shipped one.
2. Record the disposition immediately, naming the documentation that context did have.
3. *Only then* open `crates/happenstance-ladybug/src/live_handle.rs:68-83` (if
   `real-lbug-driver-swap` has not already retired it),
   `references/adapter-shapes.md`'s *Writing the concrete type instead of `Self::Batch<'_>`* section,
   `spec/SPECIFICATION.md:4600-4615` and PS-34's own body at `:5546-5560` — which the citation repair
   in step 4 requires reading anyway.

**Both discharges are valid and the merged port picks which.** If `type Batch;` landed (PS-5), the
honest outcome is *"the trap is retired"*: the impl was written with the concrete owned type, no
`error[E0195]` appeared, and no `where Self: 'a` bound was needed in any impl. That claim is **not**
vacuous — `references/adapter-shapes.md` records that binding an *owned* type to the **GAT** bought
none of that relief, so retirement is attributable to the GAT leaving the port and to nothing else.
If the GAT survived, PS-34 is binding and the re-test is literal: capture the diagnostic and record
whether the documented remedy was sufficient.

**This project *reports* on PS-34; it does not move its marker.** That is
`projection-store-freeze`'s (`RUNBOOK.md:602`, "6, re-tested 11"). A disposition implying the marker
should move routes through a decision atom and a re-plan, never an edit to `spec/SPECIFICATION.md`.

### The decision about the six citations: `spec-trace` green is necessary and not sufficient

`cargo xtask spec-trace` resolves every `file:line` citation **and** checks the cited line sits within
twelve lines of the sentence's subject (`xtask/src/spec_trace.rs:291-372`, tolerance at `:374-378`).
Six citations name lines this PR moves: `spec/SPECIFICATION.md:372` (the status table), `:4590`,
`:4592`, `:4605`, `:4612` (§4's framing prose) and `:8095` (the portfolio table). All six sit in
**non-normative** prose — the nearest clause headings are §4.1a at `:4791` and §6.5 at `:7985`, and
none is inside a `[FROZEN]` clause body — so repairing them is both permitted and required
(`_decomposition.md`, M7).

Two things about the repair are decisions, not mechanics:

- **A stale citation can still pass.** `references/evaluation/review-citation-drift.md` §1 records six
  citations that resolved, passed `spec-trace`, and pointed at the wrong line. So the gate step is
  the floor; the bar is a hand-reviewed
  `git diff <slice-base>..HEAD -- spec/SPECIFICATION.md` confirming no `[FROZEN]` marker or clause
  sentence differs (project AC-008).
- **Two of the six make a factual claim that this PR falsifies, not just a line that moves.**
  `:372` and `:8095` both say *"**Two** of the five are `todo!()` throughout — `LadybugProjectionStore`
  … and `PostgresProjectionStore`"*. After this PR only `PostgresProjectionStore` is. A
  line-number-only edit there would leave a resolving citation attached to a false sentence. The
  decision: **renumber where the sentence stays true; make the minimal truthful correction where it
  does not**, staying inside the same non-normative prose and touching no clause marker, no clause
  sentence, and no maturity table row.

### Where this sits in the slice

`real-adapter` is delivered as one integrated surface. `real-lbug-driver-swap` merges first and
*may* leave `spec-trace` red — its retirement of `live_handle.rs` invalidates `:4592`, `:4605` and
`:4612` — and this story is the one that repairs all six. **The slice boundary is where
`cargo xtask ci` must be green**, not the intermediate commit
(`_storymap.md`, *Why the slices are cut here*: "the six citation repairs ride with the bodies").

### The persona slice this realizes

The actor is **an adapter author meeting `ProjectionStore` for the third time** — the fresh pair of
hands the project charter is organised around — and secondarily **the reviewer who will be asked to
believe the freeze verdict** (`_storymap.md`, *Backbone*, activity **B**). What that actor gets from
this PR is the first `ProjectionStore` implementation in this workspace whose methods do something
when called, and a written answer to "was the documentation enough for me?" that was taken while the
answer was still true.

## Integration contract

- **Archetype**: `capability`. It closes a slice through the port, the driver, the crate's lint
  posture and the specification's own cross-references — not a component behind a stub.
- **Slice / milestone**: `real-adapter`. Slice-mate: **`real-lbug-driver-swap`** (HS-S0076), which
  merges first and is this story's only `depends_on`. The two are implemented in one context and the
  gate bar is applied at the slice boundary.
- **Mount point**: **`crates/happenstance-ladybug/src/lib.rs`** — the crate root. It is the
  composition root in the literal sense here: it declares the modules, re-exports
  `LadybugProjectionStore` and `LadybugProjectionStoreError` at `:79-81`, and carries
  `#![allow(clippy::todo)]` at `:69-72`. **Deleting that attribute is the mounting act** — it is what
  puts this crate back under the workspace-wide `clippy::todo` deny, so that a forgotten body fails
  `cargo clippy --workspace --all-targets --all-features -D warnings` instead of passing silently.
  Its `# Status: skeleton` heading at `:3-8` stops being true in the same commit and is corrected
  with it.
- **Wires into** (real siblings consumed, not invented):
  - `crates/happenstance-core/src/projection.rs` — the port. Read, never edited; it is
    `projection-store-freeze`'s (`_decomposition.md`, *Not touched*). Its `type Batch` declaration at
    `:97-99` is the pre-freeze snapshot; **read what actually merged**.
  - `crates/happenstance-ladybug/src/projection_store.rs` — `GraphStatement`, `GraphWriteSet`,
    `LadybugProjectionStoreError`, `LadybugProjectionStore::connect`. All already `pub` and already
    documented by the phase-2 skeleton.
  - `crates/happenstance-ladybug/tests/port_shape.rs` — the existing compile-time call site. Its
    `const _` block at `:75-80` instantiates `weak_flavour::advance` and
    `send_flavour::spawn_a_batch_across_an_await` at `LadybugProjectionStore`, which is where the
    trait obligations are actually discharged. **Extend, never weaken**: the two flavour modules stay
    separate because having both trait names in scope makes every method call `error[E0034]`
    (CLAUDE.md constraint 4, and the comment at `:11-14`).
  - `lbug`'s `Database` / `Connection` / `Value` / `Error`, landed by `real-lbug-driver-swap`.
- **Renders surfaces**: **none.** `_design.md` records a no-surface determination, approved
  2026-08-12; its `## Items` and `## Signatures` blocks are `N/A`, so there is no signed-off item id
  for this story to claim. The public *items* it fills bodies behind —
  `LadybugProjectionStore::{checkpoint, begin, commit, rollback}` — were declared by the phase-2
  skeleton and are unchanged in signature by this PR. The one design note that does bind is
  `_design.md`'s *Public API surface note*: `LadybugProjectionStoreError`'s `Driver`/`Commit` fields
  re-source from `stand_in::Error` to `lbug`'s error type as a mechanical re-source of an existing
  `#[from]`/`#[source]` field on an already-`pub`, already-`#[non_exhaustive]` enum — no item is
  added, renamed or removed here.
- **Conformance rule(s)**: **none run in this PR, and that is the point of the next story.** No
  projection fixture exists yet, so nothing in `crates/happenstance-testkit/src/suite.rs` observes
  these bodies until `ladybug-fixture-and-conformance-run` (HS-S0078) writes `LadybugFixture` and
  invokes `projection_store_conformance!`. The behaviour that *is* adapter-observable and is proven
  here is proven at two other tiers: the compile-time `const _` instantiation in
  `tests/port_shape.rs`, and this crate's own unit tests over the narrowing free functions. **This
  story adds no conformance rule and changes no port**, so the "a port change that names no rule"
  hazard does not apply — but the bodies it writes are exactly what HS-S0078 will run rules against,
  and any behaviour this spec asserts and the suite later contradicts is a finding for the verdict,
  not a body to quietly adjust.
- **Clause(s)**: discharges nothing on its own and **amends nothing**. It supplies evidence toward
  **PS-1** (checkpoint and read-model write in one transaction), **PS-4** and **PS-5** (the deferred
  write set as `Batch`), and **PS-34** (its disposition, recorded not moved). It repairs six
  non-normative `file:line` citations in `spec/SPECIFICATION.md` and touches **no** clause marker and
  no clause sentence. Nothing `[FROZEN]` moves; if the implementation implies one should, that routes
  through a decision atom and a re-plan (project charter, *Out of scope*).
- **Advances DoD scenario**: initiative **DoD 7** — *"the projection suite discriminates … two
  structurally unlike batch shapes pass it"* — by turning the unlike end of the batch-shape axis from
  a `todo!()` skeleton into a real implementation, which is the precondition the suite run needs.
  Through that it advances **DoD 8** (*"after the unlike batch shape exists, a written verdict…"*),
  and it keeps **DoD 13** (`cargo xtask ci` green including `spec-trace`) reachable at the slice
  boundary. It is project **DoD 5** in full (`project.md`: no `todo!()` and no
  `#![allow(clippy::todo)]` remain).

## PR boundary

The paths this story is allowed to touch, as globs. `redkiln verify --grain story` reads the first
fenced block under this heading and fails on any file changed outside it.

```
crates/happenstance-ladybug/src/**
crates/happenstance-ladybug/tests/**
crates/happenstance-ladybug/Cargo.toml
spec/SPECIFICATION.md
references/evaluation/ps-34-third-implementer-disposition.md
.bklg/from-contract-to-published-library/ladybug-projection-store/fill-the-bodies-and-ps-34-disposition/**
```

**In this PR**

- The four `SendProjectionStore` bodies on `LadybugProjectionStore`
  (`crates/happenstance-ladybug/src/projection_store.rs:270-292`), against the real driver.
- The `INT64` ↔ `NonZeroU64` narrowing extracted into free functions, with unit tests covering both
  directions and both error variants.
- Deleting `#![allow(clippy::todo)]` (`…/lib.rs:69-72`) and correcting the two `# Status: skeleton`
  headings that stop being true (`…/lib.rs:3-8`, `…/projection_store.rs:216-222`).
- `crates/happenstance-ladybug/Cargo.toml`'s `description`, whose "Not yet implemented." is false the
  moment the bodies land and would otherwise be published by
  `package-completeness-and-name-claim`. **One line; nothing else in the manifest.**
- Extending `tests/port_shape.rs` where the merged `Batch` shape changes a bound — and recording that
  change as a finding rather than weakening the test to fit.
- Repairing the six `spec/SPECIFICATION.md` citations, per the decision above.
- `references/evaluation/ps-34-third-implementer-disposition.md` — the dated, commit-pinned PS-34
  record, in the genre `references/evaluation/README.md:8-13` defines (immutable, dated, pinned,
  superseded rather than edited).
- This story's own backlog folder (ledger, report).

**Explicitly not in this PR**

- **The fixture and the conformance run** → `ladybug-fixture-and-conformance-run` (HS-S0078).
  `LadybugFixture`, `projection_store_conformance!`, the new `tests/` target and the
  `xtask/src/proof.rs` `ARTEFACTS` row are all that story's.
- **Read-your-own-writes** → `read-your-own-writes-projection` (HS-S0079). This PR must not record an
  *answer* to PS-12; a body that happens to demonstrate it is not the recorded answer.
- **Adding the `lbug` dependency, deleting `stand_in.rs`, retiring `live_handle.rs`, rewriting
  `lib.rs`'s driver-absent and open-decisions sections** → `real-lbug-driver-swap` (HS-S0076), the
  slice-mate that merges first. Cargo.lock is untouched here.
- **`publish = false`, the licences and README, `xtask/src/package.rs`'s `PUBLISHABLE`** →
  `package-completeness-and-name-claim` (HS-S0080).
- **The build-cost measurement and the CI shape** → `cold-build-cost-and-ci-shape` (HS-S0081).
  `xtask/src/main.rs`, `.github/workflows/ci.yml` and `RUNBOOK.md` are not touched here.
- **The freeze verdict itself** → `freeze-verdict-document` (HS-S0082). This PR produces one input to
  it (the PS-34 disposition) and states no held / did-not-hold judgement.
- **Any change to `crates/happenstance-core/**` or `crates/happenstance-testkit/**`.** The port and
  the suite are `projection-store-freeze`'s. If the merged port cannot be implemented, that is
  surfaced upstream, not worked around locally (`_decomposition.md`, AC-A01).
- **Any `[FROZEN]` clause marker or clause sentence in `spec/SPECIFICATION.md`.** Citations only.
- **ADR-0025** → `adr-0025-three-answers` (HS-S0075), already merged. This PR *obeys* it; it does not
  amend it, and atoms are never hand-edited (CLAUDE.md, *Where the work lives*).

**Merge DoD one-liner** — `cargo xtask ci` (the whole gate, not `--fast`) is green on the slice,
`grep -rn "todo!" crates/happenstance-ladybug/` returns nothing, `cargo xtask spec-trace` is green
with the `spec/SPECIFICATION.md` diff hand-reviewed as citations-only, and
`references/evaluation/ps-34-third-implementer-disposition.md` exists, is dated and names the
documentation its author had.

## Behavior and interfaces

Signatures are unchanged from the phase-2 skeleton except where the merged port's `Batch` shape
dictates. `Self::Batch<'_>` collapses to `Self::Batch` if and only if `type Batch;` landed — **read
the merged port, do not assume either spelling** (`_decomposition.md`, AC-A01).

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| `begin()` yields an owned, empty write set | Returns `GraphWriteSet::new()`. Opens no connection, starts no transaction, holds no borrow — so it stays `Send + 'static` and survives an await. `begin` must not be where the transaction starts; that is what makes this the *deferred* end of the batch-shape axis. | `crates/happenstance-ladybug/src/projection_store.rs:105-119`, `:121-130`; `crates/happenstance-ladybug/tests/port_shape.rs:58-69` |
| `commit()` is one connection, one transaction, statements in order | `connect()` → `BEGIN TRANSACTION` → replay each buffered `GraphStatement` through `prepare`/`execute` with its parameters bound (never string-interpolated) → the checkpoint statement → `COMMIT`. Order is the buffer's order. | `…/projection_store.rs:70-103`, `:238-249`; module docs `:1-22` |
| The checkpoint write is the **last** statement before `COMMIT` | This is PS-1: read-model write and checkpoint write in one transaction. A second connection, a separate transaction, or a checkpoint written first all satisfy the trait and violate the specification. The statement shape is already written out in the instrument: `MERGE (c:ProjectionCheckpoint {id: $id}) SET c.position = $position`. | `crates/happenstance-core/src/projection.rs:13-30`; `crates/happenstance-ladybug/src/live_handle.rs:203-217` (**open only after the PS-34 capture**) |
| An **empty** write set still commits | An empty batch is still worth committing: the checkpoint must advance past events that produced no graph mutation, or a restart replays them forever. `commit` must not short-circuit on `batch.is_empty()`. | `…/projection_store.rs:143-149` (the doc comment on `is_empty`) |
| `checkpoint()` returns `Ok(None)` for a projection that has never run | Absence of the checkpoint node — or of the property — is `None`. Absence is the only thing that maps to `None`. | `crates/happenstance-core/src/projection.rs:100-110`; `…/projection_store.rs:270-273` |
| A stored value outside `NonZeroU64` is `MalformedCheckpoint`, never `Ok(None)` | Zero or negative is a *corrupted* checkpoint, not a missing one. Collapsing them "would silently replay a projection from the beginning". The variant carries `projection` and the raw `value` so the operator learns which and what. | `…/projection_store.rs:175-191` |
| A position above `i64::MAX` is `PositionOutOfRange`, never an `unwrap` | The other direction of the same narrowing, on the write side of `commit`. | `…/projection_store.rs:192-202` |
| The narrowing lives in free functions, unit-testable without a database | Both directions get direct tests; no LadybugDB instance, no temp directory, no new dev-dependency. `happenstance-neon` set this precedent with `decode_checkpoint` / `decode_commit` as free functions rather than port methods. | `spec/SPECIFICATION.md:8095`; `_decomposition.md`, *Testing brief* → **Unit** |
| `WriteTransactionInUse` is mapped and returned, never retried | Many readers, one writer, so contention is routine. The body maps the driver's single-writer refusal onto the dedicated variant and hands it to the caller. **No retry loop, no backoff, no sleep** — the answer to contention is a verdict finding, not a hidden timing behaviour. | `…/projection_store.rs:204-213`; `project.md`, *Risks and coupling notes* |
| `Commit` stays distinct from `Driver` | `COMMIT` failing after every statement was accepted is the one failure where the caller learns nothing about *which* write was at fault; the correct response is to rebuild the batch, not fix a statement. Do not collapse the two variants while wiring up the real error type. | `…/projection_store.rs:164-173` |
| `rollback()` discards without executing | Nothing was executed, so nothing is undone: drop the write set. A `checkpoint()` after a `rollback()` returns exactly what it returned before. It must not open a connection to issue `ROLLBACK` — there is no transaction to roll back. | `…/projection_store.rs:289-292`; module docs `:14-22` |
| No `todo!()` remains, and the crate-level allow is deleted rather than narrowed | A `todo!()` has type `!` and coerces to anything, so a skeleton counted as an adapter is decorative by construction. Deleting the allow is what makes clippy able to catch a regression at all — narrowing it leaves clippy silent. | `crates/happenstance-ladybug/src/lib.rs:68-72`; `project.md`, DR-1 |
| The crate's own docs stop claiming to be a skeleton | `lib.rs:3-8` and `projection_store.rs:216-222` both say `# Status: skeleton` / "the bodies are `todo!()`". Both become false in this commit. Every fallible public function keeps an `# Errors` section naming *conditions*, not error types. | `crates/happenstance-ladybug/src/lib.rs:1-8`; `…/projection_store.rs:216-250`; `standards/rust/70-rustdoc-obligations.md` |
| Generic code still instantiates at both flavours | `tests/port_shape.rs`'s `const _` block must still compile with real bodies behind it. If `type Batch;` landed, `for<'a> S::Batch<'a>: Send` at `:61` collapses to `S::Batch: Send` — **that collapse is a finding for the verdict** (PS-5's predicted ergonomic win, observed), not a silent edit. Never weaken a bound to make the file compile. | `crates/happenstance-ladybug/tests/port_shape.rs:33-80`; `_decomposition.md`, M3 |
| The `Send` flavour is the one implemented, on evidence | `lbug`'s `Database` and `Connection` are both `Send + Sync`, so `impl SendProjectionStore` is the honest choice; had they been `!Send`, the bare flavour would have been, "and *that* would have been the finding". `#[async_trait]` is never an option (ADR-0001). BR-12's `!Send` proof belongs to `cloudflare-durable-object-store`. | `…/projection_store.rs:252-257`; `.kb/decisions/0001-async-port-flavours.md` |
| PS-34's disposition is recorded fresh, and states its own prior context | One of two mutually exclusive outcomes — "trap retired, no `error[E0195]`, no `where Self: 'a` in any impl" if `type Batch;` landed, or the literal re-test if the GAT survived. The record names the documentation the author *did* have and is dated and commit-pinned. It reports; it does not move PS-34's marker. | `_decomposition.md`, T2; `_storymap.md`, *PS-34, and who is allowed to write it*; `references/evaluation/README.md:8-13`; `RUNBOOK.md:602` |
| Six `SPECIFICATION.md` citations resolve *and* point at the right subject | `spec-trace` checks resolution plus a ±12-line subject tolerance, and a stale citation can still pass — so the gate step is the floor and the hand-reviewed diff is the bar. Renumber where the sentence stays true; make the minimal truthful correction at `:372` and `:8095`, whose `todo!()`-count claim this PR falsifies. No clause marker, no clause sentence. | `xtask/src/spec_trace.rs:291-378`; `spec/SPECIFICATION.md:372`, `:4590`, `:4592`, `:4605`, `:4612`, `:8095`; `references/evaluation/review-citation-drift.md` §1 |

## Data and migrations

**No schema migration, and no migration path is owed** — nothing is published, no LadybugDB database
written by this adapter exists anywhere, and `happenstance-ladybug` still carries `publish = false`
at the start of this PR (`crates/happenstance-ladybug/Cargo.toml:12`). There is no prior on-disk
shape to migrate from.

There *is* a persisted data shape, and this PR is where it becomes real rather than illustrative:

| Element | Shape | Constraint that fixes it |
| --- | --- | --- |
| Checkpoint node | A node in the graph, not a sidecar file or table | ADR-0025 Q1. As a node property is the only way to keep the checkpoint write inside the same `BEGIN TRANSACTION` as the read-model write, which is PS-1. The sidecar loses on the invariant, not on preference (`crates/happenstance-ladybug/src/lib.rs:51-58`). |
| Label and property names | The instrument already writes `MERGE (c:ProjectionCheckpoint {id: $id}) SET c.position = $position` | Not prescribed by any brief — "the Cypher schema for read models and for the checkpoint node — label names, property names, indexes" is explicitly the implementer's, with only transaction membership fixed (`_decomposition.md`, §8). Follow the instrument unless ADR-0025 says otherwise; deviating is a doc obligation on the module. |
| One node per `ProjectionId`, or one node with a property per id | **Open, and ADR-0025's residue.** PS-23 asks whether one commit may advance two ids (`spec/SPECIFICATION.md:5317`); that clause is `projection-store-freeze`'s and this adapter is a data point for it. Pick what ADR-0025 states; if ADR-0025 left it open, pick per-`ProjectionId` (the shape `MERGE … {id: $id}` already implies) and say so in the module docs. | `crates/happenstance-ladybug/src/lib.rs:51-58`; `_decomposition.md`, §5 Q1 |
| `position` property type | `INT64`, LadybugDB's widest integer, against a `NonZeroU64` domain type | The asymmetry is the whole reason `MalformedCheckpoint` and `PositionOutOfRange` exist. Both directions are checked; neither is `unwrap`ed (`…/projection_store.rs:175-202`). |
| Read-model nodes and edges | Whatever the projection's own Cypher writes; this adapter buffers and replays, it does not model | `GraphStatement` is the raw-Cypher answer and it is deliberately *the port's* answer too — PS-9 and PS-11 ask whether generic code needs a write vocabulary on `Batch`, and this crate is one of the two data points (`…/lib.rs:62-66`). |

**Do not add an index, a constraint or a schema-creation step outside the committed transaction.**
Anything this adapter writes has to be reachable from inside one `BEGIN TRANSACTION` … `COMMIT`, or
the atomicity claim the port exists for stops holding.

## Acceptance criteria

Story-grain, and **numbered independently of the project's**. Where the sections above say
"project AC-003 / AC-006 / AC-008" they mean `project.md`'s list; the ids below are this story's and
are what `_ledger.md` and `redkiln verify --grain story` read. The right-hand column maps each back.

The actor throughout is the one the story map names: **an adapter author meeting `ProjectionStore`
for the third time**, and behind them **the reviewer who will be asked to believe the freeze verdict**
(`_storymap.md`, *Backbone*, activity **B**). Every criterion is written as something that actor can
do, or see, that they could not before this PR.

| id | criterion | verification | traces |
| --- | --- | --- | --- |
| AC-001 | **The four bodies do something when called.** GIVEN the adapter author has the real `lbug` driver merged and a temporary LadybugDB directory, WHEN they call `begin()`, append one parameterised `GraphStatement`, `commit()` it at a position and then call `checkpoint()`, THEN that position comes back and no method panics — this is the first `ProjectionStore` implementation in the workspace whose methods execute rather than type-check. | `crates/happenstance-ladybug/tests/round_trip.rs` → `checkpoint_round_trips_through_a_real_commit` (a crate-local integration test against a real temp LadybugDB — **not** the conformance fixture, which is HS-S0078's). Plus `grep -rn "todo!" crates/happenstance-ladybug/` returning nothing. | project AC-003; DoD 5 |
| AC-002 | **The checkpoint and the read-model write land in one transaction, checkpoint last.** GIVEN a batch carrying two statements, WHEN the author commits it, THEN exactly one connection is opened, one `BEGIN TRANSACTION` … `COMMIT` pair wraps the replay, the buffered statements execute in the buffer's order, and the checkpoint statement is the **last** statement before `COMMIT` — so a reviewer reading `commit` sees PS-1 held rather than inferred. | `crates/happenstance-ladybug/src/projection_store.rs` `#[cfg(test)] mod tests` → `commit_writes_the_checkpoint_last_in_one_transaction` (asserts over the ordered statement sequence `commit` executes, without a database), and `crates/happenstance-ladybug/tests/round_trip.rs` → `a_failed_statement_leaves_the_store_unchanged`. | project AC-003; PS-1 |
| AC-003 | **An empty batch still advances the checkpoint.** GIVEN events that produced no graph mutation, WHEN the runner commits an empty `GraphWriteSet` at their position, THEN `commit` does not short-circuit on `is_empty()` and a subsequent `checkpoint()` returns the new position — so a restart does not replay those events forever. | `crates/happenstance-ladybug/tests/round_trip.rs` → `an_empty_write_set_still_advances_the_checkpoint`. | project AC-003; PS-1 |
| AC-004 | **A corrupt checkpoint is reported, never mistaken for a missing one.** GIVEN a checkpoint property holding `0` or a negative value on the read side, or a position above `i64::MAX` on the write side, WHEN the author reads or writes it, THEN they get `MalformedCheckpoint { projection, value }` or `PositionOutOfRange { position }` — never `Ok(None)`, never an `unwrap`, never a variant simplified away — and both directions are proven without a LadybugDB instance. | `crates/happenstance-ladybug/src/projection_store.rs` `#[cfg(test)] mod tests` → `zero_and_negative_are_malformed_not_absent`, `positions_above_i64_max_are_out_of_range`, `every_valid_position_round_trips`. Unit tier, over the narrowing free functions. | project AC-003; PS-1 |
| AC-005 | **Single-writer contention is handed back, not hidden.** GIVEN a second commit racing an open write transaction, WHEN the driver refuses, THEN the caller receives `LadybugProjectionStoreError::WriteTransactionInUse` rather than an opaque `Driver` string, and the crate contains no retry loop, backoff or sleep — so the freeze verdict inherits the finding instead of a hidden timing behaviour. | `crates/happenstance-ladybug/tests/round_trip.rs` → `a_second_concurrent_commit_reports_write_transaction_in_use` (fallback in *Implementation notes* if a deterministic in-process race is not reachable), plus `grep -rniE "retry|backoff|sleep" crates/happenstance-ladybug/src/` returning only doc prose. | project AC-003; verdict input |
| AC-006 | **Rollback is free, and it is reversible in the only sense that matters.** GIVEN a batch with statements appended, WHEN the author calls `rollback()`, THEN the write set is dropped without opening a connection or issuing `ROLLBACK`, and a subsequent `checkpoint()` returns exactly what it returned before `begin()` — the store's observable state is where the author left it. | `crates/happenstance-ladybug/tests/round_trip.rs` → `rollback_leaves_the_checkpoint_where_it_was`. | project AC-003 |
| AC-007 | **The crate stops claiming to be a skeleton, and clippy can catch it if it starts again.** GIVEN a reviewer who trusts the gate over prose, WHEN they run the workspace clippy step and read the crate's own docs, THEN `#![allow(clippy::todo)]` is **gone** from `lib.rs` (deleted, not narrowed), both `# Status: skeleton` headings are corrected, `Cargo.toml`'s "Not yet implemented." description is corrected, and every fallible public function still carries an `# Errors` section naming *conditions* rather than error types. | `cargo clippy --workspace --all-targets --all-features -- -D warnings`; `grep -n "allow(clippy::todo)" crates/happenstance-ladybug/src/lib.rs` empty; `grep -rn "Status: skeleton" crates/happenstance-ladybug/` empty; `cargo doc -p happenstance-ladybug` clean under the gate's docs step. | project AC-003; DoD 5; `standards/rust/70-rustdoc-obligations.md` |
| AC-008 | **Generic code still instantiates at both flavours, with real bodies behind it.** GIVEN whatever `Batch` shape the frozen port merged, WHEN `port_shape.rs` compiles, THEN `weak_flavour::advance` and `send_flavour::spawn_a_batch_across_an_await` still instantiate at `LadybugProjectionStore` with the two flavour modules kept separate, and any bound that had to change — notably `for<'a> S::Batch<'a>: Send` collapsing to `S::Batch: Send` — is **recorded as a finding**, in the PR description and in the PS-34 record, never silently weakened. | `cargo test -p happenstance-ladybug --test port_shape` → the `const _` block at `crates/happenstance-ladybug/tests/port_shape.rs:75-80` and `both_batch_shapes_satisfy_the_same_generic_code`; the finding text reviewed in `references/evaluation/ps-34-third-implementer-disposition.md`. | project AC-003, AC-006 |
| AC-009 | **PS-34's disposition is on disk, dated, and honest about the context that wrote it.** GIVEN the third pair of hands has just written the four bodies **without** having read `live_handle.rs:68-83`, WHEN they record the outcome, THEN `references/evaluation/ps-34-third-implementer-disposition.md` exists, is dated and commit-pinned, names the documentation the author *did* have, states exactly one of the two mutually exclusive outcomes (trap retired — no `error[E0195]`, no `where Self: 'a` in any impl; or the literal re-test with the diagnostic captured and the documented remedy judged sufficient or not), and moves no marker in `spec/SPECIFICATION.md`. | Presence and content review of `references/evaluation/ps-34-third-implementer-disposition.md` against `references/evaluation/README.md:1-13`'s genre (immutable, dated, commit-pinned); `git log --follow` shows it committed in this PR; `git diff <slice-base>..HEAD -- spec/SPECIFICATION.md` shows no `[PROVISIONAL]`/`[FROZEN]` marker changed. | project AC-006; DR-7 |
| AC-010 | **The specification's cross-references still point at the truth after the bodies land.** GIVEN six citations naming lines this PR moves, WHEN the reviewer runs the trace step and reads the diff, THEN `cargo xtask spec-trace` is green **and** the diff shows only non-normative prose changed: `:4590`, `:4592`, `:4605` and `:4612` renumbered, `:372` and `:8095` minimally corrected because this PR falsifies their "**Two** of the five are `todo!()` throughout" claim, and no `[FROZEN]` marker, clause sentence or maturity-table row differs. | `cargo xtask spec-trace` (also a `cargo xtask ci` step); hand-reviewed `git diff <slice-base>..HEAD -- spec/SPECIFICATION.md`, recorded in the implementation report — the gate is the floor, the diff is the bar (`references/evaluation/review-citation-drift.md` §1). | project AC-008; AC-A05 |

## Interaction quality

**This story renders no surface.** `_design.md` is a signed-off **no-surface determination**, approved
2026-08-12: `## Surfaces`, `## Items`, `## Signatures`, `## The states the API must express`,
`## Anti-patterns` and `## The doctest` are all `N/A`. So the composition family has no screen to be
measured against, and saying so is the honest discharge rather than inventing one. What survives is
the library analogue of each invariant, and **every invariant that applies is already an AC row
above** — this section only says which row carries it and how it is verified.

### State invariants

| Invariant | Library reading here | Carried by | Verified by |
| --- | --- | --- | --- |
| **Reversibility** | A discarded batch costs nothing and leaves no residue: `rollback` opens no connection and issues no `ROLLBACK`, because nothing executed. A *failed* `commit` leaves the store unchanged (`crates/happenstance-core/src/projection.rs:124-125`). | **AC-006**, **AC-002** | `rollback_leaves_the_checkpoint_where_it_was`; `a_failed_statement_leaves_the_store_unchanged` |
| **Preserved state** (the focus/scroll/selection analogue: what the caller can still observe afterwards) | After `rollback`, `checkpoint()` returns exactly its prior value; after an empty commit, it returns the *new* position rather than silently staying put. | **AC-006**, **AC-003** | `rollback_leaves_the_checkpoint_where_it_was`; `an_empty_write_set_still_advances_the_checkpoint` |
| **Non-occlusion** (nothing hides the state the caller needs to act on) | Three occlusions are forbidden by name: a corrupt checkpoint collapsing into `Ok(None)`; single-writer refusal arriving as an opaque `Driver` string; `Commit` collapsed into `Driver` so the caller cannot tell "rebuild the batch" from "fix a statement". | **AC-004**, **AC-005**, and **EC-005** below | the three narrowing unit tests; `a_second_concurrent_commit_reports_write_transaction_in_use`; review of the error enum's arms |
| **In-place, not a context jump** | The lint posture changes where it lives — the crate-root allow is *deleted*, not pushed down to four `#[allow]` attributes at the bodies or narrowed to a module. Narrowing leaves clippy silent, which is the same failure wearing a smaller hat. | **AC-007** | `grep -n "allow(clippy::todo)"` empty; workspace clippy `-D warnings` |
| **Reachability without an escape hatch** (the keyboard-reachability analogue: everything is usable through the ordinary path) | Every behaviour this PR adds is reachable through the port's own four methods from *generic* code bound on `ProjectionStore` / `SendProjectionStore` — no `LadybugProjectionStore`-only inherent method is required to drive it. | **AC-008** | the `const _` instantiation in `tests/port_shape.rs:75-80` |

### Composition invariants

| Invariant | Status | Carried by |
| --- | --- | --- |
| **Presentation exists at all** (the "not bare markup" analogue) | **Applies.** A library's presentation is its rustdoc: `# Errors` sections naming *conditions*, and headings that are true. Two headings currently assert `# Status: skeleton` and one manifest line says "Not yet implemented." — all three become false in this commit. | **AC-007** |
| **Composition / placement, transience, hierarchy, density budget** | **N/A, on a signed-off determination**, not by omission. There is no screen, no chrome and no revealed-vs-persistent axis; `_design.md`'s `## Items` and `## Signatures` are `N/A`, so there is no signed-off item id for this story to claim and no density number to hold to. The one design note that *does* bind — `LadybugProjectionStoreError`'s `Driver`/`Commit` fields re-sourcing from `stand_in::Error` to `lbug`'s error type on an already-`pub`, already-`#[non_exhaustive]` enum — adds, renames and removes no item. | — (recorded in *Integration contract*) |
| **Named anti-patterns** | **Applies.** `_design.md` names none (no surface), so the binding anti-patterns are the project's own, and each is an AC row rather than a bullet: *retry until green* (`project.md`, Risks); *simplify a narrowing variant away* (`_decomposition.md` §4); *weaken `port_shape.rs` to make it compile* (`_decomposition.md` §9); *narrow the allow instead of deleting it* (DR-1). | **AC-005**, **AC-004**, **AC-008**, **AC-007** |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | The checkpoint property holds `0` or a negative `INT64`. | `MalformedCheckpoint { projection, value }`, carrying both the projection name and the raw value. **Never `Ok(None)`** — that is the variant's whole reason for existing (`crates/happenstance-ladybug/src/projection_store.rs:175-191`). Covered by AC-004. |
| **EC-002** | A `SequencePosition` above `i64::MAX` is handed to `commit`. | `PositionOutOfRange { position }` returned *before* anything is written. No `unwrap`, no `as` cast, no truncation (`…/projection_store.rs:192-202`). Covered by AC-004. |
| **EC-003** | Another write transaction is already open when `commit` runs. | `WriteTransactionInUse` returned to the caller unchanged. No retry, no backoff, no sleep, no widening into `Driver`. Whether this is a declared capability limit or a rule defect is the verdict's question, not this PR's (`project.md`, Risks). Covered by AC-005. |
| **EC-004** | A statement inside the transaction fails after others were accepted. | The transaction is not committed and the store is left unchanged — the port requires it in as many words: *"a failed commit must leave the store unchanged"* (`crates/happenstance-core/src/projection.rs:124-125`). The batch is consumed either way. Covered by AC-002. |
| **EC-005** | `COMMIT` itself fails after every statement was accepted. | `Commit(#[source] …)`, kept **distinct** from `Driver`: it is the one failure where the caller learns nothing about *which* write was at fault and the correct response is to rebuild the batch, not fix a statement (`…/projection_store.rs:164-173`). Do not collapse the two while re-sourcing the error type. |
| **EC-006** | `connect()` cannot open a connection (bad path, locked database, driver init failure). | `Driver(#[from] …)` from `LadybugProjectionStore::connect` (`…/projection_store.rs:238-249`), propagated unchanged out of `checkpoint` and `commit`. `begin` must not be able to reach this condition at all — it opens no connection. |
| **EC-007** | The merged port cannot be implemented as written (a `Batch` shape, a bound or a signature this adapter cannot satisfy). | **Halt and surface upstream to `projection-store-freeze` (HS-P0010).** Do not work around it locally, do not edit `crates/happenstance-core/**`, and record it as a verdict finding (`_decomposition.md`, AC-A01, T1). |
| **EC-008** | After repairing the citations, `cargo xtask spec-trace` is still red, or a repair would land inside a `[FROZEN]` clause body. | Stop. A citation that can only be repaired by touching clause text is a scope escape: it routes through a decision atom and a re-plan, never an edit here (`project.md`, Out of scope). Covered by AC-010. |

## Non-functional

| id | Requirement | Why, and how it is checked |
| --- | --- | --- |
| **NF-001** | **No `#[async_trait]`, ever.** | It injects `+ Send` and makes the `wasm32` target impossible; the ports are defined once without a `Send` bound and `trait_variant` derives the flavour (`.kb/decisions/0001-async-port-flavours.md`). Checked by `grep -rn "async_trait" crates/happenstance-ladybug/` returning nothing. |
| **NF-002** | **No runtime dependency enters this crate.** `spawn_blocking` is *available* — the store is `'static` and can build a connection inside the closure — and is deliberately not reached for, because it would put tokio in a runtime-agnostic adapter (`…/projection_store.rs:39-47`). `tokio` stays a dev-dependency. Q3's answer is ADR-0025's; this PR does not pre-empt it by adding one. | `crates/happenstance-ladybug/Cargo.toml`'s `[dependencies]` gains nothing in this PR (the `lbug` line is the slice-mate's). `Cargo.lock` is untouched here. |
| **NF-003** | **No new dependency at all**, normal or dev, and no `Cargo.toml` change beyond the one-line `description`. | Keeps the packaging story (`package-completeness-and-name-claim`) and the build-cost measurement (`cold-build-cost-and-ci-shape`) measuring what they think they are measuring. Checked by reviewing the `Cargo.toml` diff. |
| **NF-004** | **The MSRV stays 1.97.1** and nothing in these bodies needs more. | ADR-0029 raised it deliberately once; moving it in silence is what is forbidden (`CLAUDE.md`, binding constraint 5). Checked by CI's `msrv` job. |
| **NF-005** | **Determinism over timing.** No test in this PR may depend on a sleep, a wall-clock margin or a thread-scheduling race for its *pass* result. | A flaky adapter test is worse than a missing one here, because the freeze verdict will cite this run. AC-005's contention test is the one place a race is inherent — see *Implementation notes* for the declared fallback. |
| **NF-006** | **No literal position values asserted anywhere.** | The specification permits gaps and a conformant adapter may leave them; compare against positions the store actually assigned (`CLAUDE.md`, *The rule that matters*). |
| **NF-007** | **The gate bar is the whole `cargo xtask ci`, at the slice boundary** — not `--fast`, and not necessarily at this intermediate commit. `real-lbug-driver-swap` may leave `spec-trace` red; this story is what makes it green. | `_storymap.md`, *Why the slices are cut here*; `_decomposition.md`, *Merge-gate commands*. |

## Implementation notes (non-prescriptive)

Nothing here is binding. It is the shape the constraints above tend to produce, written down so the
implementer spends their thinking on the parts that are genuinely open.

**Order the work so the PS-34 capture stays honest.** The sequencing in the Context pack is a hard
constraint, and it has a mechanical consequence worth restating: **repair the citations last.** Every
edit to `projection_store.rs` and `lib.rs` moves the lines the six citations name, so repairing them
before the bodies are final means repairing them twice. Bodies → PS-34 record → *then* open
`live_handle.rs`, `references/adapter-shapes.md` and `spec/SPECIFICATION.md:4600-4615` → citations →
`cargo xtask spec-trace` → whole gate.

**Read the merged port before writing a single signature.** `Self::Batch<'_>` collapses to
`Self::Batch` if and only if `type Batch;` landed. The skeleton was written so that dropping the
lifetime is *a deletion here rather than a redesign* (`…/projection_store.rs:261-268`), which is a
prediction this PR gets to confirm or falsify — and falsifying it is a verdict finding, not a problem
to absorb.

**A statement plan is easier to test than a connection.** Both the ordering claim (AC-002) and the
empty-batch claim (AC-003) are about *which statements execute, in what order* — a question a pure
function can answer. Extracting the plan (`&GraphWriteSet`, `&ProjectionId`, `SequencePosition` → the
ordered statement sequence, checkpoint last) alongside the two narrowing free functions gives the unit
tier something real to assert on, and leaves `commit` as connect-begin-replay-commit. This is a
suggestion; a live assertion against the round-trip test is equally valid. What is **not** valid is a
recording double standing in for `lbug` on any path the conformance suite will later exercise
(`_decomposition.md`, *Fixtures / seams*: no mocking of `lbug` itself).

**Free functions, because `happenstance-neon` already paid for that lesson.** `decode_checkpoint` and
`decode_commit` are free functions there rather than port methods precisely so they can be tested
without a backend (`spec/SPECIFICATION.md:8095`). Same shape, same reason.

**AC-005's declared fallback.** If a deterministic in-process second writer is not reachable against
`lbug` — two connections from one `Database`, one holding an open write transaction — then prove the
mapping at the unit tier (the driver-error → `WriteTransactionInUse` classifier is a free function
over an error value) and record the reason the live race was not written, in the implementation
report, as an input to HS-S0078's fixture capability constants. Do **not** write a sleep-based race to
manufacture the failure (NF-005), and do not delete the criterion.

**Temp directories are per test, not per crate.** The crate-local `round_trip.rs` needs an isolated
LadybugDB directory per test for the same reason `LadybugFixture` will need one per instance
(`crates/happenstance-testkit/src/contract.rs:17-23`). This is not the fixture and must not become
one by accident — no `Fixture` impl, no `Capability` constants, no `projection_store_conformance!`
here (HS-S0078 owns all three).

**The two `# Status: skeleton` headings are load-bearing prose, not decoration.** `lib.rs:1-8` also
carries a *What this skeleton establishes* list whose four findings are still true and still cited
elsewhere; correcting the status heading is not licence to delete them. The driver-absent and
open-decisions sections are the slice-mate's (`real-lbug-driver-swap`), so leave them alone even if
they read oddly at the intermediate commit.

**The `Cargo.toml` edit is one line.** `description` only. Everything else in that manifest —
`publish = false`, licences, README, `PUBLISHABLE` — belongs to `package-completeness-and-name-claim`,
and touching it here would split one gate failure across two PRs (`_decomposition.md`, M5).

## Tests and CI (merge gate)

Grounded in `_decomposition.md`'s *Testing brief* — its tier table, its merge-gate commands, and its
"the suite is the oracle, and it is not this story's" framing. This story sits **below** the oracle:
its job is to make the thing the suite will run against exist, and to catch the regressions the suite
cannot see (a `todo!()` left in, a narrowing simplified away, a citation gone stale).

| Tier | Command / path | Proves |
| --- | --- | --- |
| **Static** | `grep -rn "todo!" crates/happenstance-ladybug/` (empty); `grep -n "allow(clippy::todo)" crates/happenstance-ladybug/src/lib.rs` (empty); `grep -rn "Status: skeleton" crates/happenstance-ladybug/` (empty) | AC-001 (no body left unfilled), AC-007. These are the checks the compiler cannot make for you: a `todo!()` has type `!` and satisfies any signature. |
| **Static** | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | AC-007 — and only *because* the crate-level allow is deleted. With the allow narrowed, clippy stays silent and this row proves nothing (DR-1). |
| **Static** | `cargo xtask spec-trace` | AC-010's floor: every `file:line` citation resolves and sits within twelve lines of its subject (`xtask/src/spec_trace.rs:291-378`). |
| **Static (human)** | `git diff <slice-base>..HEAD -- spec/SPECIFICATION.md`, reviewed line by line | AC-010's bar: no `[FROZEN]` marker, clause sentence or maturity-table row differs, and the two falsified sentences at `:372` / `:8095` were corrected truthfully rather than renumbered. A stale citation can pass the gate step (`references/evaluation/review-citation-drift.md` §1). |
| **Unit** | `cargo test -p happenstance-ladybug --lib` → `#[cfg(test)] mod tests` in `crates/happenstance-ladybug/src/projection_store.rs`: `zero_and_negative_are_malformed_not_absent`, `positions_above_i64_max_are_out_of_range`, `every_valid_position_round_trips`, `commit_writes_the_checkpoint_last_in_one_transaction` | AC-004 in full, AC-002's ordering half. No database, no temp directory, no new dev-dependency — this is the tier `_decomposition.md` assigns the narrowing to. |
| **Integration (compile-only)** | `cargo test -p happenstance-ladybug --test port_shape` → the `const _` block at `crates/happenstance-ladybug/tests/port_shape.rs:75-80` | AC-008. Instantiating generic code is what discharges the trait obligations; the two flavour modules stay separate because both names in scope is `error[E0034]` (CLAUDE.md constraint 4). |
| **Integration (live driver)** | `cargo test -p happenstance-ladybug --test round_trip` → `checkpoint_round_trips_through_a_real_commit`, `an_empty_write_set_still_advances_the_checkpoint`, `rollback_leaves_the_checkpoint_where_it_was`, `a_failed_statement_leaves_the_store_unchanged`, `a_second_concurrent_commit_reports_write_transaction_in_use` | AC-001, AC-002, AC-003, AC-005, AC-006. Against the real `lbug` driver — no double. This is also the first commit at which every contributor's `cargo test --workspace --all-features` pays the C++ build cost, which is why `cold-build-cost-and-ci-shape` exists. |
| **Process** | `references/evaluation/ps-34-third-implementer-disposition.md` present, dated, commit-pinned; reviewed against `references/evaluation/README.md:1-13` | AC-009. Not a test-framework assertion and cannot be made one: the claim is about whether documentation was sufficient *for someone*, which a green build does not establish (`_decomposition.md`, *Testing brief*, AC-006 row). |
| **Merge gate (slice)** | `cargo xtask ci` — the **whole** gate, not `--fast` | The bar for the `real-adapter` slice: fmt, clippy `-D warnings`, `--workspace --all-features` tests, four wasm32 steps, docs, `spec-trace`, the `--no-default-features` doc build, `cargo package --list`, plus `cargo-hack` / `cargo-deny` / nightly docsrs where the tool resolves. `--fast` is explicitly not this project's bar (`_decomposition.md`, *Merge-gate commands*). |
| **Story grain (redkiln)** | `cargo xtask affected --base main` (wired as `verify.affected_gate` in `.redkiln/config.yaml:40`) | That this story's diff cannot break its own package or its dependents, and that the file-reading lints and `spec-trace` ran even though part of this diff maps to no package. |
| **Ledger** | `.bklg/from-contract-to-published-library/ladybug-projection-store/fill-the-bodies-and-ps-34-disposition/_ledger.md` (`verify.require_ledger: true`) | Every AC above flipped to `satisfied: true` with cited `file:line` / test-id evidence. A green gate proves something works, never that these ten criteria are the things that work. |

## Risks and coupling (PR-scoped)

- **The `Batch` shape is unknown at planning time and is the one thing every signature depends on.**
  `type Batch;` versus `type Batch<'a> where Self: 'a` changes `port_shape.rs`'s bounds, PS-34's
  disposition and half the doc prose. *Mitigation*: nothing in this spec asserts a spelling; AC-A01's
  preflight (`preflight-and-unlike-axes`, HS-S0074) has already asserted what merged, and EC-007
  halts rather than works around a port that cannot be implemented.
- **Contamination of the PS-34 record.** The single highest-integrity risk in this story: reading
  `live_handle.rs:68-83`, `references/adapter-shapes.md`'s E0195 section, or
  `spec/SPECIFICATION.md:4600-4615` *before* the capture destroys the freshness the criterion is
  about, irreversibly and invisibly. *Mitigation*: the Context pack quotes none of them; the ordering
  is written as steps 1–3; the record must state what documentation it *did* have, which makes a
  contaminated record self-evident to the reviewer.
- **Line-number churn between the bodies and the citations.** Six citations name lines this PR moves,
  and the moves are not finished until the last doc-comment edit. *Mitigation*: repair last, then
  `cargo xtask spec-trace`, then the whole gate.
- **Two of the six citations carry a claim this PR falsifies.** Renumbering alone leaves a resolving
  citation attached to a false sentence — the exact failure `review-citation-drift.md` §1 documents.
  *Mitigation*: AC-010 requires the diff to be read, not just the gate to be green.
- **The C++ build cost lands on every local iteration from this PR onward.** `cargo test --workspace
  --all-features` now compiles LadybugDB through `cxx`/`cmake`. *Mitigation*: none available here —
  the levers are `xtask/src/main.rs`'s step args and `.github/workflows/ci.yml`, both owned by
  `cold-build-cost-and-ci-shape` (HS-S0081), and a feature flag is explicitly not one of them
  (`_decomposition.md` §7). Budget the wall-clock; do not "fix" it locally.
- **Scope creep toward the fixture.** A live round-trip test is one refactor away from being a
  `Fixture`, and writing it here would take HS-S0078's mount point (`tests/` + the `ARTEFACTS` row)
  with it, leaving that story with nothing to integrate. *Mitigation*: the PR boundary, and the
  explicit prohibition in *Implementation notes*.
- **Answering PS-12 in passing.** A `commit` body that happens to demonstrate read-your-own-writes is
  not the recorded answer, and recording one here would pre-empt `read-your-own-writes-projection`
  (HS-S0079) and weaken its evidence. *Mitigation*: this PR states no PS-12 outcome, in code comments
  or in the disposition record.
- **Coupling to the slice-mate is tight and one-directional.** `real-lbug-driver-swap` (HS-S0076)
  merges first and may leave `spec-trace` red and `live_handle.rs` retired; this story assumes neither
  a specific error type nor the file's survival. *Mitigation*: the two are implemented in one context
  and the gate bar is applied at the slice boundary, so a red intermediate commit is expected, not a
  defect.
- **Coupling downstream.** `freeze-verdict-document` (HS-S0082) cites this PR's PS-34 record and its
  `port_shape.rs` bound finding by name. An undated, unpinned or hedged record is a hole in the
  verdict that cannot be filled later, because the context that could fill it will have read
  everything by then.

## Dependencies

**Blocks on**

- **`real-lbug-driver-swap`** — the slice-mate that merges first. It lands the real `lbug` dependency,
  re-points `Value`/`Error` off `stand_in`, deletes `stand_in.rs`, retires or re-points
  `live_handle.rs`, and rewrites `lib.rs`'s driver-absent and open-decisions sections at ADR-0025.
  Without it there is no real driver to write bodies against, and `LadybugProjectionStoreError`'s
  `Driver`/`Commit` arms still wrap `stand_in::Error`. This is the story's **only** `depends_on`.

Transitively, through that story: `adr-0025-three-answers` (the checkpoint placement, mutation
vocabulary and blocking-API answers these bodies execute) and `preflight-and-unlike-axes` (the
assertion that the merged port really shipped what this spec assumes, and the dated "structurally
unlike" axes).

**Unlocks**

- **`ladybug-fixture-and-conformance-run`** — needs a store whose methods do something before
  `projection_store_conformance!` can mean anything. Its `LadybugFixture`, its new `tests/` target and
  its `xtask/src/proof.rs` `ARTEFACTS` row are all deliberately left undone here.
- **`package-completeness-and-name-claim`** — publishing a crate whose `description` says "Not yet
  implemented." while its bodies are real is the mismatch this PR's one-line manifest edit prevents.
- **`read-your-own-writes-projection`** (through the fixture story) and **`freeze-verdict-document`**,
  which consumes this PR's PS-34 disposition and its `port_shape.rs` bound finding as named inputs.

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. The Context pack above is self-sufficient for
starting; open these at the moment named, and not before — the ordering constraint on rows 5 and 6 is
part of AC-009, not a courtesy.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `crates/happenstance-ladybug/src/projection_store.rs` | The four `todo!()` bodies at `:270-292`, the error taxonomy at `:152-214`, `GraphStatement`/`GraphWriteSet` at `:70-150`, `connect` at `:238-249`, and the module docs that argue the one-commit-one-transaction flow. Every AC in this story is about something in this file. | First, before writing anything. | AC-001 |
| `crates/happenstance-core/src/projection.rs` | The port itself, and the invariant paragraph at `:13-30` that is PS-1. Also `:124-125` — *"a failed commit must leave the store unchanged"* — which EC-004 is quoting. **Read what actually merged**; `:97-99`'s `type Batch<'a> where Self: 'a` is the pre-freeze snapshot. | Before writing the first signature, to settle the `Batch` spelling. | AC-002 |
| `crates/happenstance-ladybug/src/lib.rs` | Carries `#![allow(clippy::todo)]` at `:69-72` — deleting it is the mounting act — plus the `# Status: skeleton` heading at `:3-8` and the *What this skeleton establishes* findings that stay true. | When deleting the allow and correcting the status prose. | AC-007 |
| `crates/happenstance-ladybug/tests/port_shape.rs` | The existing compile-time call site. `:33-70` explains why `for<'a> S::Batch<'a>: Send` is there and what it collapses to; `:75-80` is the `const _` block that discharges the obligations. Extend, never weaken. | After the bodies compile, before declaring AC-008. | AC-008 |
| `crates/happenstance-ladybug/src/live_handle.rs` | Holds the E0195 transcript at `:68-83`, the rustc ICE transcript at `:38-66`, and the already-written checkpoint statement at `:203-217`. **Do not open before the PS-34 capture** — reading `:68-83` first is what destroys AC-009's freshness. | *Only after* the disposition record is written; then for the checkpoint statement shape and the citation repair. | AC-009 |
| `references/adapter-shapes.md` | Records at `:169-195` and `:363-367` that binding an *owned* type to the GAT bought none of PS-5's predicted relief — which is what makes "the trap is retired" a non-vacuous claim rather than a shrug. | Immediately after the capture, when writing the disposition's reasoning. | AC-009 |
| `references/evaluation/README.md` | Defines the genre the disposition must be written in: immutable, dated, pinned to a commit, superseded rather than edited (`:1-13`). | Before creating `ps-34-third-implementer-disposition.md`. | AC-009 |
| `spec/SPECIFICATION.md` | The six citation sites — `:372` (status table), `:4590`, `:4592`, `:4605`, `:4612` (§4 framing prose), `:8095` (portfolio table) — plus PS-34's own body at `:5546-5560` and the E0195 discussion at `:4600-4615`. The clause wins wherever this spec disagrees with it. | At the citation-repair step, after the bodies are final. | AC-010 |
| `xtask/src/spec_trace.rs` | `:291-372` is the resolution-plus-subject check and `:374-378` the twelve-line tolerance — i.e. exactly how much drift the gate tolerates, which is why AC-010's bar is the hand-read diff. | While repairing citations, to know what the gate will and will not catch. | AC-010 |
| `references/evaluation/review-citation-drift.md` | §1 records six citations that resolved, passed `spec-trace`, and pointed at the wrong line. The precedent that makes the human diff review mandatory. | Before signing off AC-010. | AC-010 |
| `standards/rust/70-rustdoc-obligations.md` | The `# Errors`-names-conditions rule and the rest of the doc obligations the corrected module docs must still satisfy. | When rewriting the two status headings and the four methods' docs. | AC-007 |
| `.kb/decisions/0001-async-port-flavours.md` | Why `#[async_trait]` is never available and why `SendProjectionStore` is a derived flavour rather than a second hand-written trait — the constraint behind NF-001 and behind the `Send`-flavour choice these bodies implement. | Before reaching for any async convenience while wiring a synchronous driver into an async port. | AC-001 |
| `.kb/decisions/0008-one-derivation-for-both-ports.md` | One derivation for both ports, and the finding that a provided body cannot hold the `Batch` GAT across a suspension point under any remedy tried — which rules out shapes before they are weighed. | If the merged port kept the GAT and a bound in `port_shape.rs` has to change. | AC-008 |
| `.kb/open-questions/projection-store-batch-has-no-apply-seam.md` | The open question these bodies are a data point for: generic code can `begin` and `commit` and cannot write anything in between. Consumed as evidence, not re-decided here. | While deciding how `GraphWriteSet` is populated in the round-trip test. | AC-001 |
| `crates/happenstance-testkit/src/contract.rs` | `:17-23` states the isolation rule (`two_fixture_instances_observe_none_of_each_others_appends`) that the crate-local temp-directory test must not violate — and the shape this story must stop short of, because the fixture is HS-S0078's. | When writing `round_trip.rs`'s temp-directory strategy. | AC-003 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/_decomposition.md` | The architecture brief's mount points M1–M9 (M7 is the citation list), the data-flow section §4, and T2's two mutually exclusive PS-34 discharges. The testing brief's tier table is the source for the merge-gate section above. | Whenever this spec seems to under-determine something — it is the next layer down, not a different opinion. | AC-010 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/_storymap.md` | *PS-34, and who is allowed to write it* states the constraint on **who**; *Why the slices are cut here* states why the citations ride with the bodies and where the gate bar applies. | Before the capture, and when deciding what belongs in this PR versus the slice-mate's. | AC-009 |
| `RUNBOOK.md` | `:588-610` is the provisional ledger, with PS-34's row marked *"6, re-tested 11"* — the line that makes this project a *reporter* on PS-34 rather than its owner. `:4393-4444` is phase 11's goal and exit criteria. | When tempted to move a marker in `spec/SPECIFICATION.md`. | AC-009 |
| `crates/happenstance-ladybug/Cargo.toml` | The one-line `description` edit this PR owns, and the `publish = false` at `:12` it must **not** touch. | At the end, with the doc corrections. | AC-007 |

## Clarifications resolved during spec

1. **Story ACs are numbered independently of the project's, and the front half's references are to
   the project's.** The Context pack and Integration contract say "AC-006" and "project AC-008"
   meaning `project.md`'s list. This spec's `AC-001`–`AC-010` are the story's, and the traces column
   maps them: story AC-001 – AC-008 → project AC-003; story AC-009 → project AC-006; story AC-010 →
   project AC-008. The ledger carries the story ids. No AC was added or dropped relative to the front
   half's declared set.
2. **Where the ten split.** The one-line PR slice names five obligations (bodies, allow, narrowing
   tests, citations, PS-34). They split into ten because "the bodies do something" and "the bodies do
   the *right* thing" are different failures with different tests: AC-001 catches a `todo!()` left in,
   AC-002/AC-003/AC-006 catch a body that runs and violates PS-1, the empty-batch rule or rollback's
   no-op contract. Collapsing them would leave a green test suite and a wrong adapter.
3. **This story runs live tests, and it is still not the conformance run.** AC-001, AC-002, AC-003,
   AC-005 and AC-006 need a real LadybugDB, so a crate-local `tests/round_trip.rs` is in the PR
   boundary. That is not a `Fixture`, not `projection_store_conformance!`, and not an `ARTEFACTS` row
   — all three remain `ladybug-fixture-and-conformance-run`'s, and *Implementation notes* says so
   explicitly so the boundary does not erode under the temptation to "just make it a fixture".
4. **AC-005's live race is allowed a declared fallback, and the criterion is not.** Single-writer
   contention may not be deterministically reachable in-process against `lbug`. The fallback is a
   unit-tier proof of the error mapping plus a recorded reason — never a sleep-based race (NF-005),
   never a deleted criterion, and never a retry loop (which is the anti-pattern the AC exists to
   forbid).
5. **The interaction-quality section is discharged as a no-surface determination, not skipped.**
   `_design.md` was approved 2026-08-12 with `## Items`, `## Signatures` and `## Anti-patterns` all
   `N/A`. The state family maps onto real library invariants and is carried by AC-002 – AC-008; the
   composition family is `N/A` except *presentation exists at all* (rustdoc, AC-007) and *named
   anti-patterns* (taken from `project.md` and `_decomposition.md`, since the design names none). Both
   exceptions are AC rows, not prose bullets, so `redkiln verify` can see them.
6. **ADR-0025's atom is cited by name, never by path.** `.kb/decisions/0025-*.md` does not exist yet —
   `adr-0025-three-answers` (HS-S0075) authors it through `.kb/_intake/` and `/redkiln:kb-ingest`, and
   atoms are never hand-written (CLAUDE.md, *Where the work lives*). So it appears in the anchors table
   nowhere, and every anchor listed above resolves in the tree today.
7. **`references/evaluation/ps-34-third-implementer-disposition.md` is an output, not an anchor.**
   The filename is fixed here so that `freeze-verdict-document` can cite it before it exists; it is in
   the PR boundary and in AC-009's verification, and deliberately absent from the anchors table, whose
   every row must resolve today.
