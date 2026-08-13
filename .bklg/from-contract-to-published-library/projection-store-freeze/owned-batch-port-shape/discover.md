---
item: HS-S0004
stage: discover
created: 2026-08-12T13:01:12.349Z
updated: 2026-08-12T13:01:12.349Z
template_sig: 86ce4036
rendered_sig: fcddd4c3
---

# Discover — The owned-batch port shape, mounted and with the skeletons restated

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: land §4.0's trait verbatim — `type Batch;` with no lifetime, non-async infallible `begin`, `Checkpoint`/`Authority`/`CommitError`/`ResetError`, `reset` — re-export the four new types from `lib.rs`, and restate the Postgres, Ladybug and SQLite skeleton signatures with the same `Batch` type, same `Error` type and same `todo!()` bodies, recorded as a reviewed diff | `../_storymap.md:56` | Two halves: the port change, and the evidence that the skeletons absorbed it without redesign. The second half is a *reviewed diff*, not a compiler result |
| **AC-009** — `type Batch` carries no lifetime parameter | `../project.md:209-211` | This story owns the signature half; `projection-probe-conformance-feature` owns the write-seam half (`../_storymap.md:87`) |
| **AC-013** — the Ladybug and Postgres skeletons compile with **no change other than the removal of the batch's lifetime parameter** — same underlying `Batch` type, same error type, same bodies | `../project.md:221-224` | This story owns the reviewed diff; `whole-gate-run-and-proof-artefact` owns the whole-workspace gate (`../_storymap.md:91`) |
| `dependsOn: projection-decision-atoms, projection-api-design-record` — the first supplies the accepted ADRs that justify each of the five changes and `LiveHandleProjectionStore`'s disposition; the second supplies the signature-level review of what the shape costs a caller | `../_storymap.md:54-56,108` | AC-008's ordering *is* the check: this is the commit that must land strictly after three green `redkiln validate --kb` atoms |
| The port today: `type Batch<'a> where Self: 'a`, `async fn begin(&self) -> Result<Self::Batch<'_>, Self::Error>`, `checkpoint → Option<SequencePosition>`, `commit(batch, id, position) -> Result<(), Self::Error>`, `rollback`. No `reset`. 140 lines total | `crates/happenstance-core/src/projection.rs:97-99,110,117,126-131,138` | Every one of the five rows in the architecture brief's change table is verified against the file as it stands |
| Architecture brief Note 7: the shape changes **five** things, not one, so every skeleton's method signatures must be restated — "AC-013's bar is therefore about the adapter's own choices, not about the diff's size" | `../_decomposition.md:579-604` | The bar to spec is: same `Batch` representation, same `Error` type, same storage strategy, same `todo!()` bodies. The weaker literal bar ("compile unchanged") is unsatisfiable by this phase's own central decision (`RUNBOOK.md:3942-3948`) |
| The two skeletons AC-013 names already bind **owned** types to the GAT precisely so dropping it is "a deletion here rather than a redesign": `Transaction<'static, Postgres>` and `GraphWriteSet` | `crates/happenstance-postgres/src/projection_store.rs:99-104`; `crates/happenstance-ladybug/src/projection_store.rs:261-270`; `../_decomposition.md:596-604` | If an edit to either goes beyond restating a signature, "the port change is what is suspect, not the skeleton" |
| `LiveHandleProjectionStore` is the exception: `type Batch<'a> = GraphWriteHandle<'a>`, a genuinely borrowed live handle with **real bodies**, which cannot survive `type Batch;` as written | `crates/happenstance-ladybug/src/live_handle.rs:154-219`; `../_decomposition.md:606-616` | Its disposition is ADR-0017's, decided upstream in `projection-decision-atoms`. This story executes that decision and does not make it (Architecture brief AC-A05, `../_decomposition.md:312-316`) |
| `crates/happenstance-sqlite/src/projection_store.rs` already binds an **owned** `SqliteBatch`, and is *not* named in AC-013 because `sqlite-durable-store` owns shipping it | `../_grounding.md:61-68` | It is still restated by this story — the storymap's row names all three crates — but its bar is the same and its product decisions belong to HS-P0012 |
| Dropping the lifetime does **not** close the foreign-batch hazard: tying the batch to the receiver's lifetime was compiled and refuted, because a lifetime names a region, not an instance, so `b.commit(a.begin())` still builds. PS-15 stays `[PROVISIONAL]` and is discharged at run time by `CommitError::ForeignBatch` and a per-instance stamp | `spec/SPECIFICATION.md:5099-5125,5126-5155`; `../_decomposition.md:629-637`; `../_grounding.md:268-276` | The stamp is a **field on the owned batch** and the check is an integer comparison — so the shape this story lands must make room for it, even though the rule lands in `commit-rollback-and-drop-rules` |
| Mount points: `Checkpoint`, `Authority`, `CommitError`, `ResetError` re-exported beside `ProjectionId, ProjectionStore, SendProjectionStore` in the contract crate's export block | `../_decomposition.md:348`; `crates/happenstance-core/src/lib.rs` export block | "Mounted" for a library is the `lib.rs` export block plus the feature table (`../_storymap.md:46-49`). An item at one and not the other is an item no adapter can name |
| ADR-0001 stays in force: `#[trait_variant::make(SendProjectionStore: Send)]` is already on the trait and keeps it; `type Batch: Send;` cannot be written because `trait_variant` copies associated-type bounds verbatim into the `!Send` flavour and would break wasm32 (PS-36) | `crates/happenstance-core/src/projection.rs:87`; `../_decomposition.md:374-381`; `spec/SPECIFICATION.md:5601-5627` | The `Send` obligation on `Batch` is documented, not enforced. An "improvement" here breaks the constrained target |
| `begin` becoming infallible is load-bearing for one deployment: `async` + fallible *implies a round trip* Neon's one-shot HTTP transport cannot afford and does not need | `spec/SPECIFICATION.md:4884-4897`; `../_decomposition.md:700-704` | "Do not 'improve' it back to `async fn begin() -> Result<…>` for symmetry with `EventStore`" |
| Testing brief tiers AC-013 as **Static** and says why a compiler cannot discharge it: "a compiler cannot tell 'the minimal restatement' from 'an unrelated edit that also compiles'" | `../_decomposition.md:783` | The reviewed diff is the instrument. That sentence is the whole reason the wrong implementation below is invisible to `cargo xtask ci` |

## Questions

Open questions to resolve before specifying.

1. **Does `reset` take a batch, and what is `Authority`'s exact shape?**
   Answered by the specification, which states the target trait in full
   (`spec/SPECIFICATION.md:4632-4731`) — `reset(batch, id) -> Result<(),
   ResetError<E>>`, `Checkpoint` as `NeverRun`/`Live`/`Rebuilding`. Where this
   story and the specification disagree, the specification wins
   (`../_decomposition.md:277-279`).
2. **Where does the PS-15 identity stamp live on the owned batch?** Deferred to
   `spec`. The shape must admit it — a field minted per store instance — but the
   rule that exercises it is `commit_rejects_a_foreign_batch` in
   `commit-rollback-and-drop-rules`, and PS-15 stays provisional either way.
3. **Is `LiveHandleProjectionStore` deleted, moved or annotated?** Answered
   upstream: whichever ADR-0017 recorded. This story executes it and must not
   silently choose (`../_decomposition.md:312-316`).
4. **Does the module doc's provisional block change here?** Answered: no. That
   is AC-014 and belongs to `unstable-projection-gate-and-clause-disposition`
   (`../_storymap.md:68`). This story changes the trait, not the maturity claim.
5. **None beyond what the project brief already carries** on the remaining
   points — the eleven `[FROZEN]` PS clauses that presuppose this shape are
   inherited as specification, not re-argued (`../_decomposition.md:272-279`).

## Decision

Generic code cannot write into a projection batch, the batch borrows from its
store so only `'static`-ish stores can implement the port at all, and the port
carries no `reset` — three facts that together make a conformance suite
impossible to write. This slice lands §4.0's trait as specified: `type Batch;`
with no lifetime, a `begin` that is neither `async` nor fallible, `Checkpoint`
replacing `Option<SequencePosition>`, `commit` taking an `Authority` and
returning `CommitError<E>`, and `reset` returning `ResetError<E>` — then proves
the three skeletons absorbed it by restating their signatures and nothing else.
The spec will fix: the trait text (deferring to `spec/SPECIFICATION.md:4632-4731`
verbatim rather than restating it); the four new types' definitions, derives and
`#[non_exhaustive]` status; the export-block and feature-table mount points; the
per-skeleton diff bar, stated as *same `Batch` representation, same `Error` type,
same storage strategy, same `todo!()` bodies*, with the reviewed diff named as
the instrument because no compiler can check it; and `LiveHandleProjectionStore`'s
execution of whatever ADR-0017 decided. It touches no `[FROZEN]` clause text: the
eleven frozen `PS` clauses that presuppose this shape are what the code is being
brought into line *with*.

## The wrong implementation

**`PostgresProjectionStore` with `type Batch = Vec<String>`.** The lifetime is
gone, the signatures restate cleanly, the diff is small, every body is still
`todo!()`, and `cargo xtask ci` is green — AC-013's literal words could even be
argued, since the only *visible* change is the lifetime's removal. It is wrong
because the reason `happenstance-postgres` is in this workspace is that it sits
at the far end of the axis this port is most likely to be wrong about: it holds a
real interactive transaction (`Transaction<'static, Postgres>`,
`crates/happenstance-postgres/src/projection_store.rs:99-104`). Swapping it for a
statement buffer to make the restatement easy converts the instrument into a
second copy of the buffering shape, and the port is then frozen against
`MemoryProjectionStore` wearing three hats — the exact monoculture `CLAUDE.md`'s
spread rule and PS-2's **Rejects** clause both name
(`spec/SPECIFICATION.md:4771-4775`). Nothing executable convicts it. The
instrument is the reviewed diff the testing brief already specifies
(`../_decomposition.md:783`), and the spec must state the bar as a property of
the adapter's *type choices* so a reviewer has something to compare against.

**The same failure with the loudest possible symptom, and it is still silent:**
`git rm crates/happenstance-ladybug/src/live_handle.rs`. Under `type Batch;` that
impl cannot compile as written, so deleting it is the shortest path to green.
`cargo xtask ci` passes, the workspace shrinks, and the only compiled evidence
*against* the decision ADR-0017 is making has been discarded — the module exists
to be the counter-example to §4.2's argument and its own doc says so
(`crates/happenstance-ladybug/src/live_handle.rs:16-31`). If ADR-0017 decided it
moves to `experiments/`, this story moves it; if ADR-0017 decided it goes, this
story records that it went and why. A deletion with no decision behind it is the
failure (`../_decomposition.md:624-627`).

**The one that looks like tidying:** restoring `async fn begin(&self) ->
Result<Self::Batch, Self::Error>` for symmetry with `EventStore`. It compiles,
every skeleton is happy, the suite would still be writable, and it costs Neon's
one-shot HTTP transport a round trip it cannot afford and does not need
(`spec/SPECIFICATION.md:4884-4897`). No adapter in the tree can fail it today —
`happenstance-neon` is a skeleton — which is exactly why the rustdoc must carry
the alternative that lost at the call site (UX brief AC-U05,
`../_decomposition.md:129-138`), and why PS-6 stays provisional and owned by the
Neon phase rather than being quietly re-decided here.

**And the reintroduction nobody would notice:** a `where Self: 'a`-shaped bound
sneaking back onto the fixture side as `type Store<'a> where Self: 'a`. It is one
of five ingredients of a rustc ICE this repository already minimised and which
still reproduces on 1.97.1 (`crates/happenstance-testkit/src/contract.rs:97-111`;
`experiments/rustc-ice-gat-foreign-trait/`). Dropping the port's GAT is not a
licence to add one anywhere else.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.

The two judgement boxes. **Literal positions**: this story adds no conformance
rule — it changes a trait and restates three skeletons — so the box is vacuously
true. It is worth noting anyway that `commit` now takes a caller-supplied
position, so the rules built on this shape in later stories compare against
values they handed in, never against an assumption about what a store assigned.
**`[FROZEN]` clauses**: eleven frozen `PS` clauses presuppose this trait shape
(`../_decomposition.md:272-279`); this story changes none of their text, it makes
the code match what they already say. Where a frozen clause needed repair — PS-1,
PS-19, and anything the sweep found — the new atom was written first, in
`projection-decision-atoms`, which is this story's own `dependsOn` and the
storymap's stated ordering check (`../_storymap.md:86,108`).
