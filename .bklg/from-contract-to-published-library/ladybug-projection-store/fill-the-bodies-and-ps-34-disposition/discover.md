---
item: HS-S0077
stage: discover
created: 2026-08-12T13:02:47.047Z
updated: 2026-08-12T13:02:47.047Z
template_sig: 86ce4036
rendered_sig: 06b5cdf2
---

# Discover — Fill the four bodies, delete the allow, record PS-34's disposition

## Signal Ledger

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line — fill the four `projection_store.rs` bodies against the real driver, delete `#![allow(clippy::todo)]`, unit-test the `INT64`/`NonZeroU64` narrowing both ways, repair the six `SPECIFICATION.md` citations in the same commit, and record PS-34's disposition from a context that has not read the old transcript | `_storymap.md:43` | Four obligations in one merge, and three of them are coupled: the allow's deletion is what makes clippy catch a forgotten `todo!()`, and the citation repairs are what keeps the gate green. |
| **AC-003** (second half) — `grep -rn "todo!" crates/happenstance-ladybug/` returns nothing and `#![allow(clippy::todo)]` is gone from `lib.rs` | `project.md:189-192`; `crates/happenstance-ladybug/src/lib.rs:68-72` | The allow's own comment already names this phase as the one that removes it. |
| **DR-1** — the allow must be **deleted rather than narrowed**; a `todo!()` body type-checks against any signature, so a skeleton counted as an adapter is decorative by construction | `project.md:138-141` | Narrowing the allow to one module would satisfy a diff review and leave the tier that catches a forgotten `todo!()` switched off. |
| The four bodies are `checkpoint`, `begin`, `commit`, `rollback` at `:270-292`, each `todo!("phase 11 implements this")` | `crates/happenstance-ladybug/src/projection_store.rs:270-292` | `begin` yields an owned empty `GraphWriteSet` touching no database; `rollback` on a deferred write set has nothing to undo. Both are correct as near-trivial bodies and are the two easiest to mistake for stubs. |
| The data flow is already argued in the module docs and is not to be re-derived: `commit` opens one connection, runs `BEGIN TRANSACTION`, replays the statements in order, writes the checkpoint as **the last statement before `COMMIT`**, and commits — which *is* PS-1 | `_decomposition.md:220-241`; `crates/happenstance-core/src/projection.rs:13-30` | The checkpoint statement is already written out in the instrument: `MERGE (c:ProjectionCheckpoint {id: $id}) SET c.position = $position`, with the `i64::try_from` narrowing (`crates/happenstance-ladybug/src/live_handle.rs:202-217`). |
| The `INT64` ↔ `NonZeroU64` narrowing is checked in **both** directions; `MalformedCheckpoint` and `PositionOutOfRange` exist because collapsing a corrupt checkpoint into `Ok(None)` *"would silently replay a projection from the beginning"* | `crates/happenstance-ladybug/src/projection_store.rs:175-202`; `project.md:288-292` | Neither variant may be simplified away while filling in `checkpoint`. This is the strongest testable claim in the story. |
| `WriteTransactionInUse` is **routine**, not exceptional — LadybugDB permits many readers and exactly one writer, so two commits racing is normal, and the answer is never "retry until green" | `crates/happenstance-ladybug/src/projection_store.rs:204-213`; `project.md:278-283` | Whether a concurrency-shaped rule meeting it is a declared capability limit or a rule defect is a verdict question, and this story must surface the behaviour rather than absorb it. |
| **AC-006 / T2** — PS-34 is `[PROVISIONAL — contingent on PS-5; dead the moment `type Batch;` lands]`, so the AC has **two mutually exclusive, both-valid discharges**, and the merged port picks | `project.md:202-205`; `_decomposition.md:333-359`; `spec/SPECIFICATION.md:5546-5560` | If `type Batch;` landed, the honest outcome is *"the trap is retired"* — no `error[E0195]`, no `where Self: 'a` in any impl — citing `references/adapter-shapes.md:186-191`'s note that binding an owned type to the **GAT** bought none of that relief, so the observation is not vacuous. If the GAT survived, it is the literal re-test. |
| **T2's constraint is on *who*** — the first two pairs of hands wrote `happenstance-sqlite` and `live_handle.rs` in phase 2, so the record must come from a context that has **not** read `live_handle.rs:68-83` first, and must state what documentation it *did* have | `_decomposition.md:352-359`; `_storymap.md:86-96` | Freshness cannot be reconstructed after the impl is written, which is why the disposition is captured inside this story rather than as a later one. |
| **M7 / AC-008** — six `SPECIFICATION.md` citations name lines this story moves (`:372`, `:4590`, `:4592`, `:4605`, `:4612`, `:8095`); all six are **non-normative**, none is inside a `[FROZEN]` clause body, and `spec-trace` additionally checks the cited line is within twelve lines of its subject | `_decomposition.md:193-202`; `xtask/src/spec_trace.rs:291-372` | Repairing them is permitted and required, in the same commit as the bodies, because `spec-trace` is a gate step and a bodies-only merge leaves the gate red (`_storymap.md:67-70`). |
| `depends_on: real-lbug-driver-swap` — supplies a crate that compiles against the real `lbug` types with `stand_in` gone, which is the only state in which these bodies can be written at all | `_storymap.md:43`, `:60-66` | The swap is what makes the bodies real; the bodies are what prove the swap was not decorative. |
| This project **reports** on PS-34; moving its marker is `projection-store-freeze`'s | `_decomposition.md:355-359`; `RUNBOOK.md:602` | The row reads "6, re-tested 11". A disposition that edits the marker crosses a project boundary and AC-008. |

## Questions

**Which PS-34 discharge fires? — answered, conditionally, and the condition is
read from the merged port rather than chosen.** T2 settles both branches in
advance, and both are honest outcomes: *retired* if `type Batch;` landed, *literal
re-test* if the GAT survived. What is not deferred is the requirement that the
record states which branch fired, why, and what the implementer was looking at
when they wrote the impl.

**Who counts as the third implementer? — answered, and it is a staffing
constraint rather than a formality.** A context that has not read
`crates/happenstance-ladybug/src/live_handle.rs:68-83`, whose record names the
documentation it *did* have — the port's own rustdoc, `MemoryProjectionStore` if
it exists as PS-34's named remedy, and nothing else. This has an ordering
consequence for `spec`: the AC-006 evidence must be produced *before* the impl is
reviewed against the transcript, because a fresh reading cannot be re-created.

**Does the disposition move PS-34's marker? — answered: no.** This project
reports; `projection-store-freeze` moves markers (`RUNBOOK.md:602`). If the
disposition implies the marker should move, that routes through a decision atom
and a re-plan (`project.md:109-111`), not an edit here.

**How does `lbug`'s blocking API meet a non-blocking port (ADR-0025)? — answered
upstream and *implemented* here, which is the one place the answer becomes real
code.** The bodies must implement exactly what ADR-0025's Q3 recorded and nothing
else. If writing the bodies reveals that the recorded answer does not work — for
example that the chosen bridge cannot be expressed without a dependency the ADR
did not sanction — that is a superseding decision atom, not a body written the
other way with a comment. The falsifier
`crates/happenstance-ladybug/tests/blocking_bridge.rs` named in
`adr-0025-three-answers` (HS-S0075) lands with these bodies, because there is
nothing to drive before them.

**What counts as "structurally unlike"? — already on disk and read-only here.**
Committed by `preflight-and-unlike-axes` (HS-S0074) before this story opened. The
bodies are where two of the axes stop being claims and become observable — the
deferred write set replayed inside one transaction, and the synchronous driver
behind an `async fn`. Where an axis turns out to be wrong, the axes document is
superseded under M9's lifecycle and the finding goes to the verdict.

**Deferred to `spec`:** the Cypher schema for read models and for the checkpoint
node — label names, property names, indexes — which `_decomposition.md:411-413`
leaves to the implementer with only transaction membership fixed; and whether the
checkpoint node is per-`ProjectionId` or one node with a property per id, which
ADR-0025's Q1 residue settles for this adapter while PS-23 remains
`projection-store-freeze`'s (`_decomposition.md:270-274`).

## Decision

Four `todo!()` bodies and a crate-level `#![allow(clippy::todo)]` are the exact
shape of an adapter that exists on paper, and while both remain nothing this
project later claims — that the suite ran, that the freeze was tested — can be
believed; at the same time, the person writing those bodies is the third pair of
hands ever to meet this port, and their experience of it is evidence that expires
the moment they read the old transcript. So this story fills `checkpoint`,
`begin`, `commit` and `rollback` against the real driver, deletes the allow so
clippy becomes the tier that catches a forgotten `todo!()`, unit-tests the
`INT64`/`NonZeroU64` narrowing in both directions, repairs the six
`SPECIFICATION.md` citations that the body edits invalidate, and captures PS-34's
disposition from a context deliberately kept fresh. The spec will cover the four
bodies against the data flow already argued in the module docs (one connection,
`BEGIN TRANSACTION`, statements replayed in order, checkpoint last, `COMMIT` —
which is PS-1), the two narrowing error variants as behaviour rather than
defensive clutter, `WriteTransactionInUse` surfaced rather than retried, the
allow's deletion, the citation repairs as line-number-only edits, and the PS-34
record naming its branch, its implementer's prior context and its documentation.
Nothing `[FROZEN]` is touched: all six citations sit in non-normative framing
prose with the nearest clause headings at `spec/SPECIFICATION.md:4791` and
`:7985`, and the story's own check is a `git diff` of `spec/SPECIFICATION.md`
against the project-start commit confirming no clause marker or clause sentence
moved.

## The wrong implementation

**`checkpoint` collapsing a corrupt value into `Ok(None)`.** The
`MERGE (c:ProjectionCheckpoint {id: $id})` node holds an `INT64`; the read path
does `u64::try_from(value).ok().and_then(NonZeroU64::new)` and returns
`Ok(None)` when either step fails. It is shorter, it is what a careful-looking
implementer writes to avoid an `unwrap`, and it passes everything: every
conformance rule that reads back a checkpoint the store itself wrote sees a valid
positive `INT64` and never exercises the branch, `clippy -D warnings` is silent,
the whole gate is green. What it does in production is turn a corrupted
checkpoint into "this projection has never run" and **silently replay from the
beginning** — which is the exact sentence
`crates/happenstance-ladybug/src/projection_store.rs:178-181` gives as the reason
both variants exist. No suite rule can catch it, because no suite rule can plant
a driver-specific corrupt value. The falsifier is adapter-local and belongs in
this crate, not the testkit (`crates/happenstance-testkit/**` is not this
project's to edit, `_decomposition.md:86-88`): tests in
`crates/happenstance-ladybug/` that write `0` and a negative value onto the
checkpoint node directly through a connection and assert
`MalformedCheckpoint { .. }` rather than `Ok(None)`, plus one that commits a
`SequencePosition` above `i64::MAX` and asserts `PositionOutOfRange`. Both assert
on **error variants and on positions the store was handed**, never on a literal
position the store assigned.

**`commit` without a transaction.** Four `execute` calls in a loop, the checkpoint
statement last, `Ok(())` returned — no `BEGIN TRANSACTION`, no `COMMIT`. On a
happy path it is indistinguishable from the correct implementation: every
statement lands, the checkpoint lands, every read-back rule passes. It violates
PS-1 only under a fault between statements, and it is `CheckpointOnlyStore`'s
failure mode wearing the other hat. The rule that rejects it,
`commit_is_atomic_with_the_read_model` (PS-2, `spec/SPECIFICATION.md:4760-4774`),
is `projection-store-freeze`'s and already exists — so this story's obligation is
not to write a rule but to make sure the bodies are shaped so the rule can bite,
and `ladybug-fixture-and-conformance-run` (HS-S0078) is where it actually runs.
Worth stating because the deferred write set makes autocommit *look* harmless:
nothing is held open, so nothing seems to be at risk.

**The PS-34 disposition written after reading the transcript.** The most
dangerous mutant here, because the artefact is byte-identical either way: a record
saying "the impl was written with the concrete owned type, no `error[E0195]` was
met, no `where Self: 'a` appeared in any impl" is exactly what a genuinely fresh
implementer would write and exactly what someone who read
`live_handle.rs:68-83` first would write. Nothing in the tree distinguishes them,
and there is no test that can. The only guard is the one T2 specifies and it is a
content requirement: the record must name what documentation the implementer had
and state that the transcript was not among it, which turns an unfalsifiable
claim into one a reviewer can at least challenge. A record that omits that
sentence does not discharge AC-006 however green the build is.

**The citation "repair" that moves the clause instead of the citation.** Six
`file:line` references point at lines these bodies shift; `cargo xtask spec-trace`
goes green if the citation is corrected *or* if the prose around it is rewritten
to match the code, and the second is easier when the sentence has drifted. One of
those two edits a specification body. The guard is a reviewed
`git diff <project-start>..HEAD -- spec/SPECIFICATION.md` showing line-number-only
changes on M7's six, which is AC-008's stated proof
(`_decomposition.md:478`).

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
