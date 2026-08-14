---
item: HS-P0010
stage: design
created: 2026-08-12T04:39:23.000Z
updated: 2026-08-12T04:39:23.000Z
---

# API surface design — Freeze ProjectionStore behind a suite that can fail

## Surfaces

**No user-facing surface.** This project ships nothing a person looks at or
clicks. That is confirmed two ways rather than assumed: the initiative charter
declares `userFacing: false` and states that `interaction-patterns.md` was
"deliberately not commissioned" at the intake gate
(`.bklg/from-contract-to-published-library/_decomposition.md:411-412,479-482`),
and this project's own UX brief says it outright — *"There is no screen"*
(`.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md:25`).
A repo-wide search of `_discovery/distillation/` confirms no
`interaction-patterns.md` was ever produced for this initiative; only
`opportunities.md` and `personas-and-journeys.md` exist.

What a human meets instead is two non-visual surfaces, both verified against
the current tree:

1. **A type surface.** `ProjectionStore`, its `Batch<'a>` associated type, and
   `ProjectionId` already exist at `crates/happenstance-core/src/projection.rs`;
   the `Checkpoint` / `Authority` / `CommitError` / `ResetError` types this
   project adds are the target shape recorded in this project's own brief
   (`_decomposition.md:17,121,149,348,556,589-590`), not yet in the file. A
   caller meets this surface by writing Rust code against a trait — there is
   no route, no DOM, nothing to capture.
2. **A text surface.** The one line a conformance run prints for a declined
   capability, verified at
   `crates/happenstance-testkit/src/contract.rs:500-507` (`skip_line`, called
   by `report` at line 532): `SKIP {rule}: fixture declines`
   `` `{capability}` `` `— {reason}`. It reaches a human through stdout / CI
   logs, never a screen.

Per `CLAUDE.md`'s *Where the work lives*: `design.capture` is undeclared in
`.redkiln/config.yaml`, so the perceptual review is a declared skip, not a
silent pass — there is no app to screenshot. The bundled `_design.md` template
itself exists for exactly this case, substituting a compiled doctest for a
capture-based review. That doctest is what the rest of this file is for; the
surface manifest below stays empty because a manifest built for routes and DOM
selectors has nothing to index in a medium with neither.

```yaml
surfaces: []
```

## Items

The nine public items this project introduces or amends, **ordered as the trait
declares them**, not alphabetically. Every one is claimed by a later story; none is
implemented here. Ids are the paths a caller writes.

| id | Kind | Introduced or amended by | Claimed by |
| --- | --- | --- | --- |
| `happenstance_core::Checkpoint` | enum, 3 variants | new (PS-19, PS-20, PS-24) | `owned-batch-port-shape` |
| `happenstance_core::Authority` | enum, 2 variants | new (PS-24) | `owned-batch-port-shape` |
| `happenstance_core::CommitError` | enum, generic over `E` | new (PS-15, PS-22) | `owned-batch-port-shape` |
| `happenstance_core::ResetError` | enum, generic over `E` | new (PS-15, PS-18) | `owned-batch-port-shape` |
| `happenstance_core::ProjectionStore` | trait | **amended** (PS-5, PS-6, PS-16, PS-21) | `owned-batch-port-shape` |
| `happenstance_core::SendProjectionStore` | trait, derived | unchanged mechanism (PS-35, PS-36) | `owned-batch-port-shape` |
| `happenstance_core::ProjectionProbe` | trait, behind `conformance` | new (PS-11, PS-12) | `projection-probe-conformance-feature` |
| `happenstance_core::MemoryProjectionStore` | struct, behind `memory` | new (PS-34's replacement) | `memory-projection-store` |
| `happenstance_testkit::projection_store_conformance!` + the projection fixture trait | macro + trait | new (PS-2, PS-11) | `projection-suite-entry-point`, `projection-capability-skips` |

**The text surface has no `## Items` id and needs none.** It is the one line
``SKIP {rule}: fixture declines `{capability}` — {reason}`` that
`RuleOutcome::skip_line` already renders
(`crates/happenstance-testkit/src/contract.rs:500-507`). It is **reused
unchanged**: no projection-local skip type, no second line shape, because an
author reading one CI log must not have to learn two (Architecture brief AC-A06,
UX brief AC-U08).

## Signatures

Transcribed from `spec/SPECIFICATION.md:4632-4731`, which gives the target shape in
full, and from `:4998-5013` for the probe. **The specification wins on conflict**;
this block is a transcription, not a redesign. Each signature carries the `PS`
clause that binds it.

```rust
/// PS-19, PS-20, PS-24. Three variants, not `(Option<SequencePosition>, bool)`.
///
/// `PartialEq` + `Debug` are **load-bearing, not habit**: every rule that
/// observes a checkpoint — and the doctest below — spells it `assert_eq!`, and
/// neither compiles without both. `Copy` because the type is two words at most
/// (`SequencePosition` is a `NonZeroU64` and is itself `Copy`,
/// `crates/happenstance-core/src/event.rs:239`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Checkpoint {
    /// PS-19: never run, or reset, or rebuilding with nothing committed yet.
    NeverRun,
    /// PS-24: considered through `through`; the read model is authoritative.
    Live { through: SequencePosition },
    /// PS-24: a rebuild is in flight; rows are not authoritative.
    Rebuilding { through: SequencePosition },
}

/// PS-24. What a commit claims about the rows it leaves behind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Authority { Live, Rebuilding }

/// PS-15, PS-22. Generic over the adapter's error — never a `String`.
///
/// The derive set is `AppendError`'s, item for item
/// (`crates/happenstance-core/src/error.rs:212`), including
/// `thiserror::Error` — without it `commit(…).await?` cannot rise through a
/// `Box<dyn core::error::Error>`, which is what every doctest and every rule
/// body does. `#[error(transparent)]` on `Store` is the same choice made there:
/// the adapter's own message, unwrapped.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum CommitError<E> {
    /// PS-15: the batch was begun on a different store instance.
    #[error("the batch was begun on a different store instance")]
    ForeignBatch,
    /// PS-22: `position` is below the checkpoint already recorded.
    #[error("checkpoint regression: {current} is recorded, {attempted} was attempted")]
    CheckpointRegression { current: SequencePosition, attempted: SequencePosition },
    #[error(transparent)]
    Store(E),
}

/// PS-15, PS-18. A separate enum, so `commit`'s caller never matches `Refused`.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum ResetError<E> {
    /// PS-15.
    #[error("the batch was begun on a different store instance")]
    ForeignBatch,
    /// PS-18: this store declines to reset this projection.
    #[error("this store declines to reset this projection")]
    Refused,
    #[error(transparent)]
    Store(E),
}

#[trait_variant::make(SendProjectionStore: Send)]
pub trait ProjectionStore {
    /// PS-35, PS-36. One derivation for both ports; the `Send` flavour
    /// transitively requires `Batch: Send` and the port documents it.
    type Error: core::error::Error + 'static;

    /// PS-4, PS-5. Owned; **no lifetime parameter**, and not required to be a
    /// live transaction.
    type Batch;

    /// PS-6. Neither `async` nor fallible.
    fn begin(&self) -> Self::Batch;

    /// PS-19, PS-24. Returns the three-variant enum, never an `Option`.
    async fn checkpoint(&self, id: &ProjectionId) -> Result<Checkpoint, Self::Error>;

    /// PS-1, PS-21, PS-22, PS-23. Applies the batch and moves `id`'s checkpoint
    /// to `position`, as one unit; exactly one `ProjectionId` advances.
    async fn commit(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
        position: SequencePosition,
        authority: Authority,
    ) -> Result<(), CommitError<Self::Error>>;

    /// PS-16, PS-17, PS-18. `commit`'s dual: the caller's batch carries the
    /// deletes, and `id` returns to `NeverRun` as one unit.
    async fn reset(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
    ) -> Result<(), ResetError<Self::Error>>;

    /// PS-7, PS-8, PS-30. Exists because `Drop` cannot await.
    async fn rollback(&self, batch: Self::Batch) -> Result<(), Self::Error>;
}

/// PS-11. In the **contract crate**, behind `feature = "conformance"`, bare
/// flavour only — the suite binds the weaker trait (CLAUDE.md rule 4).
pub trait ProjectionProbe: ProjectionStore {
    /// PS-12. The declaration mechanism, and the `capability` a skip names.
    const READS_THROUGH_BATCH: bool;

    /// PS-11.
    fn probe_write(&self, batch: &mut Self::Batch, key: &str, value: u64);
    /// PS-16. Exists so the suite can exercise `reset` without knowing the
    /// read model.
    fn probe_delete_all(&self, batch: &mut Self::Batch);
    /// PS-11.
    async fn probe_read(&self, key: &str) -> Result<Option<u64>, Self::Error>;
    /// PS-12. Only called when `READS_THROUGH_BATCH`; may be `unimplemented!()`
    /// otherwise.
    fn probe_read_through(&self, batch: &Self::Batch, key: &str) -> Option<u64>;
}
```

**Where this block and an accepted ADR disagree, the atom wins.** ADR-0017,
ADR-0018 and ADR-0019 are **accepted atoms** — `status: accepted`, phase 6, at
[`.kb/decisions/0017-what-a-projection-batch-owns.md`](../../../.kb/decisions/0017-what-a-projection-batch-owns.md),
[`…/0018-returning-a-projection-to-never-run.md`](../../../.kb/decisions/0018-returning-a-projection-to-never-run.md)
and [`…/0019-what-happens-when-apply-fails.md`](../../../.kb/decisions/0019-what-happens-when-apply-fails.md),
ingested by the `2026-08-13-projection-adrs` wave at `493a194`; the long-form
records they were folded from remain at `references/adr/0017-*.md`, `0018-*.md`
and `0019-*.md`, and hold the transcripts a ~100-line atom cannot.

This block was first written against those records and has since been **re-read
against the accepted atoms, clause by clause**: `type Batch;` owned with no
lifetime and no universal write vocabulary, `ProjectionProbe` in the contract
crate behind `feature = "conformance"` (ADR-0017); `reset(batch, id)` as one unit
with the caller's own deletes, scope cited to ADR-0007, refusal as a port
mechanism, and the three-variant `Checkpoint` over `(Option<SequencePosition>,
bool)` (ADR-0018); the port growing nothing for apply failure, and `rollback`
surviving because PS-30's `AssertUnwindSafe` promise rests on it (ADR-0019).
**No sentence has had to yield** — that is now a claim checked against the atoms
rather than against the records alone. Two points where an atom deliberately
hands a question here are answered here and nowhere else: whether
`ResetError::Refused` carries a reason (`0018-…:108-110` — it stays bare, see
*The states the API must express*), and what the probe's `conformance` gate costs
an outside author (DT-8 below). If a later superseding atom differs from this
block on any point, this block yields and the sentence that yielded is named
here rather than the atom being edited — an accepted atom is immutable
(`.kb/decisions/README.md`).

## Shape decision

Two tensions were parked at the `/redkiln:plan` design gate and are settled here.
The rest of the shape is the specification's and the ADR records', cited rather
than re-decided.

### DT-3 — one authoritative source for "this guarantee does not apply"

**Resolution: the suite's own output is authoritative. Per-adapter documentation
may repeat it and may not contradict it.**

That is a statement about entitlement, not a preference: when the two disagree, a
consumer is entitled to rely on what the run printed. The reason is that the run
is the only source produced by the code under test — prose in an adapter's README
is written once and drifts, while a `Capability::declined` reason is recompiled on
every build and re-printed on every run.

**All three reason-writers that exist in this tree are disposed of** — the
fixture-written reason, the testkit-written reason, and per-adapter prose — because
naming one authoritative source and leaving the others undisposed is how the second
divergent declension policy gets minted — *"one policy, or a stated reason for
two"* (`project.md` *Risks*).

| Writer | Where | Disposition |
| --- | --- | --- |
| **The fixture** — `Capability::declined(reason)` | `crates/happenstance-testkit/src/contract.rs:355-419` | **Authoritative, and the default.** The reason is the adapter's account of a trade only the adapter can describe; the private field forces it through a `const fn` `assert!`, so `Declined("")` cannot exist. Every projection capability below uses this unless the row says otherwise. |
| **The testkit** — `NO_STORE_LIMITS` / `NO_CEILING_REASON` | `contract.rs:435-456` | **Authoritative for its one case, and the stated exception.** Its own doc says it is *"the one place the skip machinery here differs"*, and the reason is sound: *"this store has no ceiling" is the same sentence for every store that says it*, so asking each fixture to phrase it buys a paraphrase per adapter and no information. The projection family adds **no** new testkit-written reason — see the capability table. |
| **Per-adapter prose** | an adapter's own rustdoc / README | **Subordinate, and non-normative.** It may explain a declension at length; it may not be the only place one is stated, and where it disagrees with the run, the run wins. An adapter whose README claims a guarantee its fixture declines is wrong, and the run is what shows it. |

So there is **one policy** — the fixture writes the reason — with **one stated
exception**, the testkit-written reason for a fact no adapter should paraphrase.
Both live in the same rendered line, so a reader never has to know which wrote it.

**CF-40 is cited as authoritative, not re-litigated.**
`.kb/open-questions/cf-40-fixture-limits-ownership.md` records that ADR-0015
claims and disclaims CF-40's ownership *in the same document*, with ADR-0012 the
adjacent claimant, and that both decisions are accepted and immutable so the
contradiction stands as imported. **This record does not prefer a reading.** What
it states is what the *projection* fixture does given that CF-40's ownership is
unresolved: it declares **no numeric-limit constants at all**, so it never reaches
the surface CF-40 is about. The projection port has no `ExceedsStoreLimit`
analogue — `CommitError` and `ResetError` carry no capacity variant
(`spec/SPECIFICATION.md:4667-4690`) — so the question does not arise here, and
this record deliberately does not create a second instance of it.

**The projection fixture's capability set**, enumerated rather than implied, so
`projection-capability-skips` (HS-S0008) implements it rather than inventing it:

| Constant | Where declared | MUST in `SECOND_HANDLE`'s sense? | Who writes the reason | Gates |
| --- | --- | --- | --- | --- |
| `RESET_REFUSAL` | the projection **fixture** trait | **No** — declinable. A store with no protection policy has nothing to refuse, and PS-18 is `[PROVISIONAL]` on exactly that count. | the fixture (`Capability::declined`) | `refused_reset_changes_nothing` (PS-18, `spec/SPECIFICATION.md:5200-5217`) |
| `SECOND_HANDLE` | the projection **fixture** trait | **Yes — a MUST**, as on the event-store fixture. `commit_is_atomic_with_the_read_model` reads the row and the checkpoint *through fresh handles*; a fixture that cannot open a second handle cannot observe PS-1 at all, and PS-1 is the invariant the port exists for. | n/a — supported or the adapter cannot be certified | `commit_is_atomic_with_the_read_model`, `failed_commit_leaves_both_unchanged`, `commit_advances_the_checkpoint` (PS-1, PS-2) |
| `READS_THROUGH_BATCH` | **`ProjectionProbe`**, *not* the fixture | **No** — declinable, and declining is a conformant answer: it is PS-12's second arm. | the **testkit**, because "this batch exposes no read path" is the same sentence for every store that says it — the `NO_CEILING_REASON` precedent applied to its second case | `batch_reads_reflect_pending_writes` (PS-12, `:5052-5074`) |

Three notes on that table, each load-bearing:

1. **`READS_THROUGH_BATCH` is a probe const, not a fixture const.** The decline is
   a property of the batch *type*, not of the fixture's environment, and PS-12
   makes declining-by-omission compile-time enforceable. It is the one place the
   projection family adds a testkit-written reason, and it is the same argument
   `NO_CEILING_REASON` already won.
2. **CF-18's rule survives the move**: an adapter declaring `false` MUST still emit
   the rule as a **reported skip** carrying its reason, never omit it — *"a rule
   absent from the binary is indistinguishable in CI output from a rule that
   passed"* (`spec/SPECIFICATION.md:5059-5065`).
3. **The set is not empty, and that matters.** The UX brief's *"what would make
   this brief wrong"* names the failure: a projection fixture with **no declinable
   capability at all** would make the reporting discipline decorative on this port,
   and a suite with nothing to decline cannot demonstrate initiative AC-05. Two of
   the three rows above are declinable, so the failure does not obtain — recorded
   here as the finding it would have been, rather than left to be noticed later.

**DT-3's answer survives the constrained target, and this is the half a plausible
resolution gets wrong.** `RuleOutcome::report` is a **no-op on
`wasm32-unknown-unknown`** — measured under `wasm-bindgen-test-runner` with
`--nocapture`, not assumed (`crates/happenstance-testkit/src/contract.rs:511-531`).
That target's `std` has no host stdio and `wasm-bindgen-test` hooks `console.*`
rather than `println!`. Three consequences the resolution owns:

- The projection wasm harness routes `skip_line` to `console_log!` exactly as
  `__emit_wasm` already does
  (`crates/happenstance-testkit/tests/memory_conformance_wasm.rs:25`). AC-016's run
  is not the one where the stated reason silently disappears.
- Natively, libtest suppresses a *passing* test's stdout unless `--show-output` is
  passed, which is why the gate passes it — and the same doc is blunt that this
  *"makes the line reachable by a human; it does not make anyone read it."*
- **The machine-checked half is a `RuleOutcome`-value assertion, never stdout.**
  The projection family owes a sibling of `capability_skips_are_reported`
  (`crates/happenstance-testkit/tests/mutation_coverage.rs:3184`), which is exactly
  the instrument project AC-005 names. A promise checked only by stdout is a
  promise checked by nobody.

### DT-8 — whose adapter-author bar the suite holds

**Resolution: the bar is held for an author this repository did not write.** The
extension surface is the documented pair **`projection_store_conformance!` +
`ProjectionProbe`**.

**Its cost to that author, stated:** *one feature flag on a dependency the adapter
already has, and no new edge in the dependency graph.* That bound is the reason
`ProjectionProbe` lives in `happenstance-core` behind `conformance` rather than in
the testkit, and the argument is the specification's own
(`spec/SPECIFICATION.md:5015-5031`): an adapter crate implementing a *testkit* trait
for its own type is legal, but the natural home for that impl is the adapter's
`tests/` directory — *a different crate*, where neither the trait nor the type is
local, so the orphan rule rejects it. Putting the trait in the testkit therefore
forces every adapter into a **non-dev** dependency on `happenstance-testkit` and a
feature to gate it. Putting it in the contract crate costs the flag and nothing
else.

Concretely, what an outside author pays:

```toml
[dev-dependencies]
happenstance-core = { version = "…", features = ["conformance"] }
happenstance-testkit = "…"
```

— and one `impl ProjectionProbe for MyStore` beside their `impl ProjectionStore`.
No new crate in their graph that was not already there.

**The obligation this arm creates, named so it cannot be quietly dropped.**
`documented-extension-surface` (HS-S0015) must build a projection fixture **from
the documentation alone** — not copied from
`crates/happenstance-testkit/src/fixtures.rs` — and clear the mutant-registry
exactness check and the capability-skip rule. That is the only instrument that can
distinguish a bar held in fact from a bar claimed in prose, and taking this arm is
what makes it owed. HS-S0015 is scopable from this section without opening
`initiative.md`.

**What is *not* claimed.** The bar is held for an author writing a **projection
store adapter** against a port that ships behind `unstable-projection` with a
documented semver exemption (PS-3). It is not a claim that the port is stable, and
`publication-and-positioning` (HS-P0016) owns whether the feature is exposed at
publish at all. A bar held for outsiders on a surface labelled unstable is
coherent; a bar that is internal-only in fact and unqualified in public is the
failure DT-8 names, and this record takes the other arm precisely to avoid it.

### The rest of the shape, cited rather than re-decided

`type Batch;` with no lifetime, the refusal of a universal write vocabulary, the
probe's home, `reset` taking the caller's deletes, refusal as a port mechanism, and
the port growing nothing for apply failure are ADR-0017's, ADR-0018's and
ADR-0019's. All three are **accepted atoms** under `.kb/decisions/` as of the
`2026-08-13-projection-adrs` wave (`493a194`), with their long-form records still
in `references/adr/`. Nothing in this record re-argues them, and where one of them
is quoted it is quoted as settled — which it now is in the schema as well as in
the prose.

## Placement and re-export

A library has no render tree, so **the composition root is two places**: the
crate's `lib.rs` export block and the feature table that gates it. An item mounted
at one and not the other is an item no adapter can name (Architecture brief
Note 1).

| Item | Export block | Feature table |
| --- | --- | --- |
| `Checkpoint`, `Authority`, `CommitError`, `ResetError` | `crates/happenstance-core/src/lib.rs:98-124` — extend the existing `pub use projection::{ProjectionId, ProjectionStore, SendProjectionStore};` at `:116` | ungated; they are part of the port |
| `ProjectionProbe` | same block, behind `#[cfg(feature = "conformance")]` + `#[cfg_attr(docsrs, doc(cfg(feature = "conformance")))]`, copying the `memory` pattern at `lib.rs:101-103` and `:120-122` | new `conformance` feature in `crates/happenstance-core/Cargo.toml` `[features]` — probe only, **no new dependency** |
| `MemoryProjectionStore` | same block, beside `MemoryEventStore` at `lib.rs:120-122` | existing `memory` feature, which already means `std` |
| `projection_store_conformance!` + the projection fixture trait | `crates/happenstance-testkit/src/lib.rs:265-357`, beside `event_store_conformance!` | — but the fixture trait **must** also be added to `__private` (`:359-364`), or the macro expansion cannot name it in the adapter's crate |
| The port itself | `crates/happenstance-core/src/projection.rs:1-139` | `unstable-projection`, per **Visibility and stability** below |

## Visibility and stability

Specific enough to be **violated** — a later story either does these things or
visibly does not. Three lines per item: visibility, feature, placement.

**`Checkpoint`**
- Visibility: `pub`, `#[non_exhaustive]` (RS-40-5, `standards/rust/40-public-surface-and-evolution.md:212`). Three variants today and a fourth is conceivable; the attribute is what keeps adding one from being breaking.
- Feature: ungated, but reachable only when `unstable-projection` is on, since the module is.
- Placement: `crates/happenstance-core/src/projection.rs`, re-exported at `lib.rs:98-124`.

**`Authority`**
- Visibility: `pub`, `#[non_exhaustive]`.
- Feature: as `Checkpoint`.
- Placement: as `Checkpoint`.

**`CommitError<E>`**
- Visibility: `pub`, `#[non_exhaustive]`. Generic over the adapter error, never a `String` (`standards/rust/30-error-taxonomy.md:15,71`).
- Feature: as `Checkpoint`.
- Placement: as `Checkpoint`.

**`ResetError<E>`**
- Visibility: `pub`, `#[non_exhaustive]`. Kept **separate** from `CommitError`; see *What it costs a caller*.
- Feature: as `Checkpoint`.
- Placement: as `Checkpoint`.

**`ProjectionStore` / `SendProjectionStore`**
- Visibility: `pub`; `SendProjectionStore` is derived by `#[trait_variant::make(SendProjectionStore: Send)]` and is never hand-written (PS-35).
- Feature: `unstable-projection`, off by default, with a documented semver exemption — **this is the arm Architecture brief AC-A04 takes**, because PS-2's freeze bar is not clearable inside this project (PS-3, `spec/SPECIFICATION.md:4776-4787`).
- Placement: `projection.rs`, re-exported at `lib.rs:116`.

**`ProjectionProbe`**
- Visibility: `pub`, and **bare flavour only** — no `Send` variant. The suite binds the weaker trait (CLAUDE.md rule 4) and the per-test wrapper is a parameter (CF-23).
- Feature: `conformance`, off by default, no new dependency.
- Placement: `projection.rs` (or a sibling module), re-exported behind `#[cfg(feature = "conformance")]` + `#[cfg_attr(docsrs, doc(cfg(…)))]`.

**`MemoryProjectionStore`**
- Visibility: `pub`.
- Feature: `memory`, which already exists and already means `std`.
- Placement: a new module under `crates/happenstance-core/src/`, gated exactly like `memory` at `lib.rs:101-103`, re-exported at `:120-122`.

**`projection_store_conformance!` + the projection fixture trait**
- Visibility: `pub` macro; the fixture trait `pub` **and** re-exported through `__private`.
- Feature: none on the testkit side.
- Placement: `crates/happenstance-testkit/src/lib.rs:265-357` and `:359-364`.

**The rustdoc hazard, stated because this workspace has already paid for it once.**
An **intra-doc link into a feature-gated module is a hard error** when the feature
is off: the link target does not exist, and `cargo doc` fails rather than warns.
`crates/happenstance-testkit/src/lib.rs:112-132` carries the shape that survives
it. So every doc link to `ProjectionProbe` or `MemoryProjectionStore` from
ungated documentation must either sit inside a `#[cfg_attr(…)]`-gated doc block or
be spelled as plain code text rather than a link. A `--no-default-features` doc
build of `happenstance-core` is already a gate step, so this fails loudly rather
than quietly.

## What it costs a caller

Four choices are unusual enough that a reader fluent in the domain and new to the
idiom will assume a mistake. Each names the alternative that lost, **once**, and
each sentence's home is the item's own rustdoc — not only an ADR (RS-70-5,
`standards/rust/70-rustdoc-obligations.md:243`). *A trap documented only in
ADR-0017 is documented where the person who needs it is not.*

| Choice | The sentence, and where it lives | The alternative that lost |
| --- | --- | --- |
| **`Checkpoint` is a three-variant enum** | on `Checkpoint`: *an enum rather than `(Option<SequencePosition>, bool)` because the tuple can spell `(None, true)` — authoritative, never run — which means nothing* (`spec/SPECIFICATION.md:4643-4658`) | the tuple, and its weaker form a bare `Option<SequencePosition>`, which cannot distinguish a rebuild in flight from an authoritative read model at all |
| **`CommitError` and `ResetError` stay two enums** | on `ResetError`: *separate from `CommitError` so a caller matching `commit`'s result need never match `Refused`, which `commit` cannot produce; `#[non_exhaustive]` already forces a wildcard arm without also forcing dead ones* (`:4722-4727`) | one merged `ProjectionError<E>`. Each carries the adapter's error as a **type parameter**, never a `String` |
| **`begin` is neither `async` nor fallible** | on `begin`: *opening a buffer cannot fail, and an adapter that needs a round trip takes it at `commit`* (`:4692-4694`) | an `async fn begin() -> Result<…>`, which implies a round trip Neon's **one-shot** HTTP transport cannot afford and does not require |
| **`reset` takes the caller's own deletes** | on `reset`: *the port has no idea what the read model is, so `reset` is `commit`'s dual — the caller's batch carries the deletes* (`:5165-5170`) | a `reset` that clears the rows itself, which would require the adapter to know which tables belong to a `ProjectionId` — exactly the knowledge PS-9 keeps out of the port |

Each of the four appears **once**. Repeating a rationale across the trait doc, the
item doc and an ADR is the density failure RS-70-5 names, and it is why the table
above assigns each sentence a single home.

## What a user meets first

The port's own page, and — for an adapter author — `error[E0195]` about ten minutes
later. Today that trap has *nothing to copy from*
(`references/adapter-shapes.md:186-194`), which is *"the entire explanation for
zero adapters"* (`spec/SPECIFICATION.md:5553-5557`).

After this project the first thing met is the module doc on
`crates/happenstance-core/src/projection.rs`, which must carry, in this order:

1. **What the port is for** — the invariant it exists to hold: the read-model write
   and the checkpoint write become durable together or not at all (PS-1).
2. **The `unstable-projection` qualification and its reason** — PS-2's bar is not
   met, PS-3's exemption is documented, and the reader is told what would change
   that.
3. **`Batch` is owned and is not required to be a live transaction** (PS-4, PS-5) —
   because the module's current doc says the opposite (`projection.rs:22-26`) and
   that sentence is what `owned-batch-port-shape` replaces.
4. **The `Send`-flavour bound** — the `Send` flavour transitively requires
   `Batch: Send`, documented rather than left to be discovered, because
   `type Batch: Send;` cannot be stated on one flavour only: `trait_variant` copies
   associated-type bounds verbatim and would impose `Send` on the `!Send` flavour,
   breaking wasm32 (PS-36, `spec/SPECIFICATION.md:5601-5627`).
5. **A link to the copyable impl** — `MemoryProjectionStore`, which is the
   projection port's answer to *"`MemoryEventStore` exists partly to be something
   an adapter author can copy"*.

## The states the API must express

Every state the port can report is one a caller can **name**, and every state it
cannot produce is **unrepresentable**.

| State | How it is expressed | Why it cannot be conflated |
| --- | --- | --- |
| never run, or reset, or rebuilding with nothing committed | `Checkpoint::NeverRun` | PS-19: distinguishable from `commit(empty, id, FIRST, Live)`. Under an `Option`, the operator's workaround reads back as `Some(1)`, the runner advances past it, and event 1 is skipped permanently and silently |
| authoritative, considered through *P* | `Checkpoint::Live { through }` | PS-20: a runner resumes strictly **after** *P*, and `checkpoint.next()` is a correct resume point even on a store with gaps, because `ReadOptions::from` is an inclusive lower bound rather than a seek |
| rebuilding, considered through *P* | `Checkpoint::Rebuilding { through }` | PS-24: a checkpoint that reports "caught up to *N*" while holding half a graph manufactures the conflict the projection exists to prevent |
| this commit's rows are / are not authoritative | `Authority::Live` / `Authority::Rebuilding` | PS-24: the claim is made at write time, so `checkpoint` can report it without inference |
| the batch came from another store instance | `CommitError::ForeignBatch` / `ResetError::ForeignBatch` | PS-15: **run-time**, not compile-time. Tying the batch to the receiver's lifetime was compiled and refuted — a lifetime names a region, not an instance |
| the position is below the current checkpoint | `CommitError::CheckpointRegression { current, attempted }` | PS-22: both values, so a caller can log the gap rather than re-derive it |
| this store declines to reset this projection | `ResetError::Refused` | PS-18: a refusal is not success, and it leaves both halves unchanged |
| the adapter itself failed | `CommitError::Store(E)` / `ResetError::Store(E)` / `Self::Error` | a generic caller cannot name a variant inside an adapter's `#[non_exhaustive]` error type, which is why port-level outcomes get port-level enums (`crates/happenstance-core/src/error.rs:186-193`) |
| a rule did not run, and why | `RuleOutcome::Skipped { capability, reason }` | reused unchanged; there is no projection-local skip type and no second line shape |

**`ResetError::Refused` carries no payload, and that is decided out loud rather
than defaulted.** The variant stays **bare**. The reason a caller is expected to
find instead is the store's own documentation and its own logs — because the
clause's reasoning is that *"the port supplies the mechanism; the domain decides
what to protect"* (`spec/SPECIFICATION.md:5210-5216`), and a `&'static str` on the
variant would push a domain sentence through a port type that cannot validate it,
cannot localise it, and cannot keep it in step with the policy that produced it. An
operator learns **that** their reset was declined, from a typed variant they can
match; they learn **which policy** declined it from the store that holds the
policy. What is forbidden is silence about the choice — and the choice is recorded
here, with the cost named: an operator holding only the error value must go one
place further to find out why. `reset-rules` (HS-S0012) checks the behaviour with
`refused_reset_changes_nothing`; if a later adapter shows the extra hop is real
harm, the variant can gain a payload without breaking anyone, because it is
`#[non_exhaustive]`.

## Anti-patterns

Each is a move that **compiles**, which is why it needs writing down, and each
carries its evidence. An anti-pattern without a path or an atom id is an opinion.

| Forbidden | Why | Evidence |
| --- | --- | --- |
| `#[async_trait]` anywhere on this port | it injects `+ Send`, which makes the `wasm32` / Workers target impossible | `.kb/decisions/0001-async-port-flavours.md`; CLAUDE.md binding constraint 1 |
| `type Batch: Send;` | `trait_variant` copies associated-type bounds **verbatim**, so the bound lands on the `!Send` flavour too and breaks wasm32 | PS-36, `spec/SPECIFICATION.md:5601-5627` |
| any borrowing GAT on the fixture (`type Store<'a> where Self: 'a`) | one of five independently necessary ingredients of a rustc **ICE** this repository minimised, still reproducing on 1.97.1 | `crates/happenstance-testkit/src/contract.rs:97-111`; `experiments/rustc-ice-gat-foreign-trait/` |
| merging `CommitError` and `ResetError` | forces `commit`'s caller to consider `Refused`, which `commit` cannot produce | `spec/SPECIFICATION.md:4722-4727` |
| `async fn begin` | implies a round trip Neon's one-shot transport cannot afford | `spec/SPECIFICATION.md:4692-4694`; Architecture brief Note 9 |
| a second skip vocabulary, or a projection-local skip type | an author reading one CI log must not have to learn two shapes | Architecture brief AC-A06; UX brief AC-U08; `contract.rs:458-537` |
| literal position assertions in a rule (`[1, 2, 3]`) | the specification permits gaps and a conformant adapter may leave them | CLAUDE.md, *The rule that matters* |
| `compile_fail,E0080` (or any coded `compile_fail`) | rustdoc on 1.97.1 **silently ignores** an error-code annotation it cannot match, so the stricter-looking spelling is the weaker check | `crates/happenstance-testkit/src/contract.rs:400-403` |
| hardening `ProjectionId::new` | the identifier is not repaired as a side effect of freezing the port around it | Architecture brief AC-A09; `.kb/open-questions/projection-id-is-unvalidated.md` |
| binding a concrete batch type in an impl while the port still declares `Self::Batch<'_>` | `error[E0195]`; the literal `Self::Batch<'_>` stays mandatory until the GAT leaves the port | `references/adapter-shapes.md:186-194` |

**A gap flagged to the reviewer rather than papered over.** The signed-off block
below approved *"the anti-patterns recorded above"*, and at the time that section
read `N/A — no user-facing surface`. This table is therefore **new material inside
a section the prior sign-off already referenced**, drawn from CLAUDE.md's binding
constraints and the Architecture brief rather than invented here. It is called out
so the reviewer can see it is an addition, not a re-reading of what they approved.

## The doctest

The example, its home, and the gate step that compiles it. **This record cannot
compile it** — a described example is not a checked one — so it names who does.

- **Home:** the **`MemoryProjectionStore` module page**, in the new
  `memory`-gated module under `crates/happenstance-core/src/`. **Not** the
  ungated `ProjectionStore` trait doc, and that is a correction rather than a
  preference — see *Why the port's own page does not carry this example* below.
- **Gate step:** `cargo test --doc`, inside `cargo xtask ci`'s `tests` step,
  which runs `cargo test --locked --workspace --all-features`
  (`xtask/src/main.rs:143-152`). It runs today and needs no new step. The
  `--all-features` there is what turns `conformance` on, and the hidden `cfg`
  in the example is what keeps a bare `cargo test -p happenstance-core --doc`
  green as well.
- **Compiled by:** `memory-projection-store` (Testing brief row AC-012), whose
  own AC-010 already owes this page a *"runnable `begin` → write → `commit` →
  read-back walkthrough"*. It cannot compile until `MemoryProjectionStore` and
  `ProjectionProbe` exist, which is why it is specified here and run there.

```rust
/// ```
/// # #[cfg(feature = "conformance")]
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> Result<(), Box<dyn core::error::Error>> {
/// use happenstance_core::{
///     Authority, Checkpoint, MemoryProjectionStore, ProjectionId, ProjectionProbe,
///     ProjectionStore, SequencePosition,
/// };
///
/// let store = MemoryProjectionStore::new();
/// let id = ProjectionId::new("van_stock");
///
/// // Never run: the enum says so, and no `Option` is involved.
/// assert_eq!(store.checkpoint(&id).await?, Checkpoint::NeverRun);
///
/// // `begin` is neither async nor fallible: opening a buffer cannot fail.
/// let mut batch = store.begin();
/// store.probe_write(&mut batch, "depot-7", 12);
///
/// // One unit of work: the row and the checkpoint, or neither.
/// store
///     .commit(batch, &id, SequencePosition::FIRST, Authority::Live)
///     .await?;
///
/// assert_eq!(store.probe_read("depot-7").await?, Some(12));
/// assert_eq!(
///     store.checkpoint(&id).await?,
///     Checkpoint::Live { through: SequencePosition::FIRST },
/// );
///
/// // The dual: the caller's own batch carries the deletes.
/// let mut clearing = store.begin();
/// store.probe_delete_all(&mut clearing);
/// store.reset(clearing, &id).await?;
///
/// assert_eq!(store.probe_read("depot-7").await?, None);
/// assert_eq!(store.checkpoint(&id).await?, Checkpoint::NeverRun);
/// # Ok(())
/// # }
/// # #[cfg(not(feature = "conformance"))]
/// # fn main() {}
/// ```
```

**Four things in that block are load-bearing, and each was wrong in the first
draft of this record.** They are written out because `memory-projection-store` is
told to compile this example verbatim, and a specification that cannot compile is
worse than none — it is the one thing this repurposed `_design.md` exists to
supply (`CLAUDE.md`, *a doctest in place of a mock*).

1. **`ProjectionProbe` is in the `use` list.** `probe_write` and
   `probe_delete_all` are its methods, not `ProjectionStore`'s
   (`## Signatures` above; `spec/SPECIFICATION.md:4998-5013`), and a trait's
   methods are not callable without the trait in scope — `error[E0599]`, with
   rustc's *"items from traits can only be used if the trait is in scope"* note.
2. **The whole example is `cfg`-gated on `conformance`, in hidden lines.**
   `ProjectionProbe` is behind `feature = "conformance"`, off by default
   (*Visibility and stability* above), so an ungated example naming it breaks
   `cargo test -p happenstance-core --doc` for anyone who has not passed
   `--all-features`. The `#[cfg(not(…))] fn main() {}` arm is what keeps the
   block a compiling no-op in that configuration rather than a failure; an
   explicit `fn main` suppresses rustdoc's implicit wrapper, so both arms must
   be spelled.
3. **The harness is `#[tokio::main(flavor = "current_thread")]`, not a bare
   `async fn example()`.** An async fn that nobody polls compiles and **runs
   nothing**: every `assert_eq!` inside it would be dead. This is the crate's own
   idiom, at `crates/happenstance-core/src/memory.rs:45-46`, and it is what makes
   `memory-projection-store`'s AC-010 — *"both execute; neither is a `no_run` or
   `ignore` sketch"* — true rather than nominal.
4. **The assertions need derives that the port's types must carry.**
   `assert_eq!` on a `Checkpoint` requires `PartialEq` **and** `Debug`, and
   `.await?` on `commit` requires `CommitError<E>: core::error::Error` to rise
   through `Box<dyn core::error::Error>`. Both are now stated in
   `## Signatures`, where a later story can be held to them.

**Why the port's own page does not carry this example.** Two rules already in
this record forbid it, and they were in tension with the earlier draft's stated
home:

- The `ProjectionStore` trait doc renders **ungated**, while
  `MemoryProjectionStore` is behind `memory` and `ProjectionProbe` behind
  `conformance`. *The rustdoc hazard* above is about intra-doc links, but the
  same page carrying a doctest that names two feature-gated items is the same
  mistake one layer down.
- `memory-projection-store`'s AC-010 fixes the division independently: the port's
  page carries a **toy-store impl** doctest *"with no dependency on the `memory`
  feature"* (PS-34's stated rule, `spec/SPECIFICATION.md:5546-5553`) and the
  store's page carries this walkthrough. Two doctests, two jobs — the port page
  answers *"what do I implement"* and this one answers *"what does it do when I
  run it"*.

Two things the example is deliberately shaped to teach, beyond compiling:

- **The `E0195` trap is answered by omission.** There is no `Self::Batch<'_>`
  anywhere, because there is no lifetime to spell. An implementer copying this
  writes `type Batch = MyBatch;` and `async fn commit(&self, batch: MyBatch, …)`
  and it compiles — which is the whole of PS-5's promise, and which today's port
  does not keep (`references/adapter-shapes.md:186-194`).
- **`NeverRun` is asserted twice**, before the first commit and after the reset,
  which is PS-19's distinction shown rather than stated.

**One `compile_fail` doctest is owed alongside it**, on the projection fixture's
capability constant, demonstrating that a reasonless declension fails the build. It
is spelled **bare `compile_fail`**, never `compile_fail,E0080`, and its doc says
where the check actually fires: a free `const` fails at `cargo check`, but an
**associated** const is evaluated lazily and fails at **codegen** — so `cargo build`
and `cargo test` catch it and `cargo check` and `cargo clippy` do not
(`crates/happenstance-testkit/src/contract.rs:400-419`). Owner:
`projection-capability-skips`.
## Sign-off

N/A — no user-facing surface.

**Approved.** Ryan Britton (repository owner), 2026-08-12, at the `/redkiln:plan` design
sign-off gate. No mock was produced or owed: this project records no user-facing surface, and
that is stated explicitly here rather than left as a silent skip. What was approved is the
no-surface determination itself together with the anti-patterns recorded above — the design
stage's `design.capture` perceptual review remains a **declared** skip, per `CLAUDE.md`.
Conditions: none.

---

## Amendment sign-off

**Unsigned.** This block covers the material added on **2026-08-13** by
`projection-api-design-record` (HS-S0004) and is offered to the design gate as an
**amendment**. It is deliberately separate from the block above so that declining
it costs the reviewer nothing already banked.

**What is added.** The DT-3 resolution (one authoritative source, all three
reason-writers disposed, CF-40 cited rather than re-litigated, the projection
fixture's capability set enumerated, and what the `wasm32` no-op costs the answer);
the DT-8 resolution (the outside-author arm, its bound of one feature flag and no
new graph edge, and the obligation it creates on `documented-extension-surface`);
and the public-API surface record for the two **non-visual** surfaces this file
already enumerated at lines 23–38 — the type surface and the text surface — filling
`## Items`, `## Signatures`, `## Shape decision`, `## Placement and re-export`,
`## Visibility and stability`, `## What it costs a caller`,
`## What a user meets first`, `## The states the API must express`,
`## Anti-patterns` and `## The doctest`, in the bundled template's own slots and
its own order.

**What is untouched, and stays approved.** The no-screen determination, the
`surfaces: []` manifest and the sign-off block above are preserved **verbatim**;
nothing in lines 1–50 or in `## Sign-off` was edited, and no heading was renamed,
reordered or dropped. The only lines removed anywhere in this file are the ten
reading `N/A — no user-facing surface.` beneath the section headings this
amendment fills.

**What changed after the slice review, before this block was ever signed.** Three
corrections inside the material above, all of them defects in the amendment rather
than changes of position, listed so the reviewer is signing what is here now:

1. **`## The doctest` was respecified.** As first written the example could not
   have compiled — `ProjectionProbe` was missing from the `use` list though the
   example calls its methods; its stated home was the **ungated** `ProjectionStore`
   trait doc although both `MemoryProjectionStore` and `ProjectionProbe` are
   feature-gated; and its harness was an `async fn` nobody polls, so every
   assertion in it would have been dead. The home is now the
   `MemoryProjectionStore` module page — which is where `memory-projection-store`'s
   own AC-010 already puts a runnable walkthrough — and the four load-bearing
   points are written out beside the block.
2. **`## Signatures` gained the derives the block always needed** — `Debug`,
   `Clone`, `Copy`, `PartialEq`, `Eq` on `Checkpoint` and `Authority`;
   `AppendError`'s set including `thiserror::Error` on `CommitError` and
   `ResetError` — because `assert_eq!` and `?` do not work without them. No
   signature changed shape.
3. **The ADR relationship was re-read against the *accepted* atoms.** ADR-0017,
   ADR-0018 and ADR-0019 were staged when this amendment was drafted and are
   accepted now (`493a194`). The reconciliation was re-run clause by clause and
   **no sentence had to yield**; the passages that said "staged for the next wave"
   say so no longer.

**One thing to look at first.** The prior approval covered *"the anti-patterns
recorded above"* at a moment when `## Anti-patterns` read `N/A`. That section now
carries ten entries, each with a path or an atom id, drawn from `CLAUDE.md`'s
binding constraints and the Architecture brief rather than invented here. It is
flagged in the section itself as an addition rather than a re-reading of what was
approved.

**If the determination was meant to cover the type surface too.** Then project
DoD 7 — the design-stage review of the public projection API surface — cannot be
met by any story in this project, and that is a **re-plan reported at this story's
boundary**, not something to be resolved by widening a section quietly
(`_decomposition.md`, Architecture brief Note 10). Nothing in this amendment
assumes the answer.

**Conditions:** none proposed. Approve, decline, or return with conditions.
