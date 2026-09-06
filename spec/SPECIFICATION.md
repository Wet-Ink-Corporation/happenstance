# happenstance — architectural specification

- **Date:** 2026-08-06
- **Commit:** `2a65d76`
- **Status:** current. This document supersedes `references/evaluation/ARCHITECTURAL-EVALUATION.md` §3 and
  `references/evaluation/revised-runway.md`, which are retained for their record only.

This is what is true about happenstance's design *now*. An ADR records why a decision was taken and
when; this document records the decision's current form, and where the two disagree the ADR is
history and this is current. Every normative statement is a numbered clause carrying a maturity
marker, the conformance rule that checks it, the end-to-end cases it serves, and — because a rule no
adapter can fail is decorative — the wrong implementation it forbids. A clause that names no wrong
implementation is marked non-normative and demoted to prose rather than left to look load-bearing.

The evidence base is `references/evaluation/PRESSURE-TEST.md`, which adjudicated the prior evaluation by
compiling its claims, and `references/scenarios/`, six deployments walked line by line against the contract
as it exists. Where this document freezes something, it freezes it against those two.

## Contents

- [1. Foundations](#1-foundations)
  - [1.1 What this document is](#11-what-this-document-is)
  - [1.2 Conformance language](#12-conformance-language)
  - [1.3 Maturity markers](#13-maturity-markers)
  - [1.4 Clause identifiers, and the traceability obligation](#14-clause-identifiers-and-the-traceability-obligation)
  - [1.5 The dependency rule](#15-the-dependency-rule)
  - [1.6 The three ports](#16-the-three-ports)
  - [1.7 What is settled, and what will move](#17-what-is-settled-and-what-will-move)
  - [1.8 The evidence base](#18-the-evidence-base)
- [2. Value types and the wire format](#2-value-types-and-the-wire-format)
  - [How to read a clause](#how-to-read-a-clause)
  - [2.1 `Event`](#21-event)
  - [2.2 `SequencedEvent`, identity, and time](#22-sequencedevent-identity-and-time)
  - [2.3 `SequencePosition`](#23-sequenceposition)
  - [2.4 `EventType`, `Tag` and `Tags`](#24-eventtype-tag-and-tags)
  - [2.5 Bounds](#25-bounds)
  - [2.6 `Query`, `QueryItem` and `ReadOptions`](#26-query-queryitem-and-readoptions)
  - [2.7 The wire format](#27-the-wire-format)
  - [2.8 Where the remaining bodies are](#28-where-the-remaining-bodies-are)
- [3. The `EventStore` port](#3-the-eventstore-port)
  - [3.1 Derivation — why there are two traits, and what that costs](#31-derivation--why-there-are-two-traits-and-what-that-costs)
  - [3.2 `read`](#32-read)
  - [3.3 `append`](#33-append)
  - [3.4 Append conditions](#34-append-conditions)
  - [3.5 Read-side operations](#35-read-side-operations)
  - [3.6 Multi-writer, re-entrancy and durability](#36-multi-writer-re-entrancy-and-durability)
  - [3.7 Completeness — what a store that has been deleted from may look like](#37-completeness--what-a-store-that-has-been-deleted-from-may-look-like)
  - [3.8 New conformance rules this section requires](#38-new-conformance-rules-this-section-requires)
- [4. The `ProjectionStore` port](#4-the-projectionstore-port)
  - [4.0 Numbering and the shape being specified](#40-numbering-and-the-shape-being-specified)
  - [4.1 Status, and what would freeze the port](#41-status-and-what-would-freeze-the-port)
  - [4.1a What is actually open](#41a-what-is-actually-open)
  - [4.2 What a `Batch` is](#42-what-a-batch-is)
  - [4.3 The write seam](#43-the-write-seam)
  - [4.4 Read-your-writes](#44-read-your-writes)
  - [4.5 The foreign-batch hole](#45-the-foreign-batch-hole)
  - [4.6 Reset](#46-reset)
  - [4.7 Checkpoint semantics](#47-checkpoint-semantics)
  - [4.8 Failure policy](#48-failure-policy)
  - [4.9 The runner split](#49-the-runner-split)
  - [4.10 Derivation and the two flavours](#410-derivation-and-the-two-flavours)
  - [4.11 The suite this section obliges](#411-the-suite-this-section-obliges)
  - [4.12 What this section does not settle](#412-what-this-section-does-not-settle)
- [5. The `SyncPeer` port](#5-the-syncpeer-port)
  - [5.1 The bound decision: ingest never rejects](#51-the-bound-decision-ingest-never-rejects)
  - [5.2 One peer, and a runner above it](#52-one-peer-and-a-runner-above-it)
  - [5.3 Identity and idempotent ingest](#53-identity-and-idempotent-ingest)
  - [5.4 The transport floor](#54-the-transport-floor)
  - [5.5 Convergence and the merge rule](#55-convergence-and-the-merge-rule)
  - [5.6 Transitivity](#56-transitivity)
  - [5.7 Causality](#57-causality)
  - [5.8 Scope: whole-log or filtered replication](#58-scope-whole-log-or-filtered-replication)
  - [5.9 Where a per-peer watermark lives](#59-where-a-per-peer-watermark-lives)
  - [5.10 Retention across a peer set, and refusal](#510-retention-across-a-peer-set-and-refusal)
  - [5.11 The rule ADR-0003 bought, and what it costs](#511-the-rule-adr-0003-bought-and-what-it-costs)
  - [5.12 Indicative shape](#512-indicative-shape)
  - [5.13 What this section does not decide](#513-what-this-section-does-not-decide)
- [6. The conformance obligation](#6-the-conformance-obligation)
  - [6.1 The suite's own proof obligation](#61-the-suites-own-proof-obligation)
  - [6.2 The measured gaps](#62-the-measured-gaps)
  - [6.3 The fixture contract](#63-the-fixture-contract)
  - [6.4 Runtime independence](#64-runtime-independence)
  - [6.5 The instrument portfolio](#65-the-instrument-portfolio)
  - [6.6 Compatibility policy for the testkit](#66-compatibility-policy-for-the-testkit)
  - [6.7 Benchmarks are not conformance](#67-benchmarks-are-not-conformance)
  - [6.8 Traceability](#68-traceability)
  - [What this section does not settle](#what-this-section-does-not-settle)
- [7. Traceability](#7-traceability)
  - [7.1 Summary](#71-summary)
  - [7.2 The table](#72-the-table)
  - [7.3 Clauses with no conformance rule](#73-clauses-with-no-conformance-rule)
  - [7.4 Conformance rules no clause names](#74-conformance-rules-no-clause-names)
  - [7.5 Clauses that name no case](#75-clauses-that-name-no-case)
  - [7.6 E2E cases no clause claims](#76-e2e-cases-no-clause-claims)

---

## 1. Foundations

### 1.1 What this document is

This is the normative architectural specification for happenstance — the project,
not the crate of that name, which is the typed layer and one consumer of what
follows. It states what is true now about the three ports the workspace defines,
the values that cross them, and the conformance obligation that decides whether an
adapter has implemented one. Everything downstream — adapter crates, the testkit,
the typed layer, the worked examples — is measured against it.

It is not a design discussion. Where a question is open, the clause says so and
names the experiment that closes it; where a question is closed, the clause says
what is forbidden and what would prove it wrong. Nothing here is aspirational:
each clause either constrains an implementation that can fail it, or is marked
`[NON-NORMATIVE]` and demoted to prose.

**Its relationship to `.kb/decisions/`.** An ADR records *why* a decision was taken
and *when*. This document records *what is true now*. The two are different
tenses of the same design, and they drift, because a decision is recorded at the
moment it is taken and executed at some later moment or not at all.

The rule, stated once:

> Where an ADR and this document disagree, the ADR is history and this document
> is current. Changing a `[FROZEN]` clause requires a new ADR — not an edit to
> this file.

That is not a contradiction. The ADR is the *instrument* by which this document
changes; it is not a competing statement of current truth. A FROZEN clause may be
withdrawn, narrowed or reversed, and the ADR that does it explains why to whoever
reads the git history in three years. What an ADR may not do is silently continue
to describe the world after this document has recorded a different one.

The drift is not hypothetical, and the worked example is this document's own crate
names. [ADR-0006](../.kb/decisions/0006-bare-name-to-the-typed-layer.md) decided that
`happenstance-runtime` ceases to exist, that the bare name goes to the typed layer
and that the contract crate becomes `happenstance-core` — and for the length of a
phase none of that was true on disk. `ls crates/` returned `happenstance-runtime`,
`CLAUDE.md` still listed it as a "named seam" and still carried "whether
`happenstance-runtime` is the right name" as an open question, and
`PRESSURE-TEST.md:389-393` records the consequence: an accepted-but-unexecuted
rename issuing wrong instructions to the two documents an agent loads first.
Phase 0 executed it, and every citation below names `happenstance-core` because
that is what the tree now holds. The lesson outlives the fix. An accepted ADR is a
decision, not a description, and the interval between the two is where readers are
misled — which is the argument for CF-35 through CF-38 making the citations
machine-checked rather than proof-read.

Three of the seven ADRs are marked *"accepted — provisional"* in their own front
matter for exactly this reason —
[0001:5-12](../.kb/decisions/0001-async-port-flavours.md),
[0003:5-12](../.kb/decisions/0003-opaque-payloads.md) and
[0004:5-12](../.kb/decisions/0004-edition-and-msrv.md) each say they were authored before
any of the code they constrain existed. Their provisionality is discharged here,
clause by clause, against code that now exists and scenarios that have been walked
against it.

### 1.2 Conformance language

`MUST`, `MUST NOT`, `REQUIRED`, `SHALL`, `SHALL NOT`, `SHOULD`, `SHOULD NOT`,
`RECOMMENDED`, `MAY` and `OPTIONAL` are used in the RFC 2119 sense, and only when
capitalised. Lower-case "must" in surrounding prose is English, not a
requirement.

`MAY` is the one worth restating, because it is the one a conformance suite is
most likely to erase by accident. A `MAY` grants an adapter a freedom, and a rule
that asserts on one arm of a `MAY` converts it into a `MUST` without an ADR. The
canonical example is position gaps: the specification permits them (VT-11), so no
rule may assert on a literal position value (CF-6). That prohibition exists to
protect a `MAY`.

### 1.3 Maturity markers

Every clause carries exactly one marker, on its own line.

**`[FROZEN]`** — settled. Changing it requires a new ADR, not an edit. Nothing is
published today, so "binding" means binding on the next pass rather than on a
downstream user; that is a statement about who pays, not about whether the clause
holds.

**A marker binds the decision. Whether it also binds the *release* is a separate
question, answered per port.** The two coincide in §3 and come apart in §4, and
the difference is deliberate rather than an inconsistency:

- `EventStore` ships stable at `0.2.0`, so a `[FROZEN]` `ES` clause is semver-binding
  as well as decision-binding.
- `ProjectionStore` does not. PS-2 forbids freezing the port until a hostile store
  has been failed and two adapters at opposite ends of the batch-shape axis have
  passed, and PS-3 ships it behind an off-by-default `unstable-projection` feature
  with a documented semver exemption until then. So a `[FROZEN]` `PS` clause binds
  the design — the next pass may not quietly redecide it — while the port's public
  surface stays movable. That is what lets §4 answer eighteen questions now
  instead of deferring them all to an adapter that does not exist.
- `SyncPeer` is not published at 0.1 at all, so `SY` clauses bind only the design.

The gate is always a clause, never a convention: PS-2 for `ProjectionStore`, CF-25
for every port. A port whose gating clause is unsatisfied is not frozen no matter
how many of its clauses are.

**`[PROVISIONAL — <what would falsify it>]`** — settled for now, with a named
falsification test. The clause is binding until the named thing happens. A
provisional marker whose falsifier is "further thought" is not a falsifier; it
must name an artefact, a measurement or a deployment that would produce a
different answer.

**`[DEFERRED — <the experiment that settles it, and its owning phase>]`** —
deliberately open. The clause records the shape of the question and the thing
that answers it. A deferral with no owning phase is a decision the next pass makes
by accident.

**`[NON-NORMATIVE]`** — a statement that was drafted as a clause, could not name
an implementation that would violate it, and was demoted to prose. Its ID is
retained so that citations resolve. CF-30 is the worked example.

A `[PROVISIONAL]` or `[DEFERRED]` marker with an empty falsifier or experiment is
forbidden, and CF-38 makes it a build failure rather than a convention. The reason
is stated there and is worth repeating: a provisional marker with no falsifier is
indistinguishable from a decision nobody wanted to make, and by the time anyone
notices it has been load-bearing for a year.

As assembled, this document carries 201 clause IDs, of which 196 are normative:
**137 `[FROZEN]`**, **47 `[PROVISIONAL]`**, **12 `[DEFERRED]`** and **five
`[NON-NORMATIVE]`** (CF-30; VT-12, a retained pointer to ES-10; and PS-32,
PS-33 and PS-35, the three §4 clauses whose subject was this document's own work
list and which left the clause space at the typed layer's phase exit, their IDs
retained so that every citation still resolves). Section 7 breaks that out per
clause.

**One standing qualification on every `[FROZEN]` port clause.** CF-25 forbids
declaring a port frozen while an axis of §6.5's instrument portfolio has no
passing implementation at its far end, unless the freeze names the axis and the
ADR accepting the risk.

**Seven axes, and exactly three of them — durability, handle multiplicity and
batch shape — have an adapter instrument at their far ends.** Phase 3 produced
none, and could not have: it builds fixtures. What it did move is the other
half. Four axes — async
flavour, handle multiplicity, durability and, since CF-13's fixture landed,
position allocation — carry a **fixture** instrument, which by CF-26 discharges
CF-25's falsifiability half and explicitly not its implementability half. Phase 8
moved the other half on two of those four, both through
`happenstance-sqlite`'s `SqliteFixture`: it runs the reopen rules against a real
file on disk, and its `connect` opens a second `rusqlite::Connection` onto that
file rather than an `Arc` clone. Each tick is partial in a way §6.5 states — the
*reopen* half of durability and not the *fault* half, which is what ES-35 stays
`[PROVISIONAL]` against; a second real connection and not a **pool**. The third
tick is batch shape's, on `ProjectionStore` rather than on `EventStore`, and it
is *one-sided*: `SqliteProjectionStore` passes
`projection_store_conformance!` at the replay-at-commit far end, while the
live-transaction near end holds nothing that has run anything — so PS-2, which
wants the suite green at both ends, is not cleared by it. The remaining two —
transport and completeness — have nothing at
either end. So
the qualification is not hypothetical, it is not discharged by asserting it, and
it is not discharged by a fixture tick either; §6.5 says which axes are which,
how far the adapter column has got, and why the distinction is the whole point of
that table.

It is discharged three ways, and the split matters more than the total:

- **Position allocation is measured *and* accepted, and it takes both.** The
  measurement is what made the acceptance possible; the acceptance is what keeps
  the axis on the ledger after ES-10 stopped disclosing it. It is the axis the pressure
  test and all six scenarios independently ranked first, and what was missing was
  not an adapter but a number: which of `xid8` + `pg_snapshot_xmin`,
  transaction-scoped advisory locks, or a serialised sequence table buys the
  invariant, and what each costs. Phase 2 ran the probe against a real Postgres
  ([`experiments/position-visibility/README.md`](../experiments/position-visibility/README.md)):
  `xid8` + `pg_snapshot_xmin` at 0.99–1.03× baseline, the other two correct and
  16×/30× slower at 64 writers. That is the one affordable answer ES-10 says
  lifts it to `[FROZEN]` at phase 4; the mechanism is settled and its
  *structural* costs are not, and phase 10 pays them. What phase 3 added is the
  other half — a *fixture* that violates the invariant, so the rule is now known
  to bite — and CF-26 is explicit that this is not the adapter the axis is
  waiting for. ES-10 is `[FROZEN]` as of phase 4, so its `[PROVISIONAL]` marker
  no longer carries the disclosure and ADR-0013's CF-25 acceptance carries it
  instead.
- **Four `ES` clauses carry the residual exposure and say so in their own
  markers** rather than in a preamble a reader skips: **ES-11** and **ES-12**
  (transport — a one-shot-HTTP adapter that self-paginates may be unable to meet
  either), **ES-35** (durability) and **ES-40** (completeness). Each is
  `[PROVISIONAL]` with its axis named and its falsifier the far end that has not
  been built — for ES-11, ES-12 and ES-40 that far end is still an adapter; for
  ES-35 it is narrower since phase 8 built the adapter, and is now a fixture that
  arms a real fault against a real medium. ES-10 was the fifth until phase 4
  froze it.
- **The remaining axes are accepted in the ADRs that land this document.**
  ADR-0013 accepts four by name — position allocation, async flavour, handle
  multiplicity, and batch shape *pro forma*, that last one because
  `EventStore` does not sit on it and an acceptance recorded for an axis a frozen
  port does not sit on names the exposure without discharging it (CF-25).
  Accepted, explicitly, with the axis named — which is what CF-25 asks for and is
  a different act from not having noticed.

The distinction the rest of §3 relies on: a `[FROZEN]` marker binds the design and
makes changing it an ADR rather than an edit. It does not claim the far end has
voted. Where the far end could plausibly vote against, the clause is provisional
instead — and there are four of those, named above, not forty.

### 1.4 Clause identifiers, and the traceability obligation

Clause IDs are stable and are never renumbered. The prefix names the surface:

| Prefix | Surface | Section |
|---|---|---|
| `VT-n` | value types — the vocabulary every port shares | §2 |
| `WF-n` | the wire format | §2.7 |
| `ES-n` | the `EventStore` port | §3 |
| `PS-n` | the `ProjectionStore` port | §4 |
| `SY-n` | the `SyncPeer` port | §5 |
| `CF-n` | conformance obligations — what the suite owes | §6 |

Numbering runs within a prefix and nowhere else. A withdrawn or demoted clause
keeps its number; the sequence is a set of names, not a count.

Every clause carries four things besides its marker:

- **`Rule:`** — the conformance rule that can observe a violation. An existing
  rule is named as it appears in
  [`crates/happenstance-testkit/src/suite.rs`](../crates/happenstance-testkit/src/suite.rs);
  a rule this document specifies but which does not exist yet is marked as new.
  `WF-n` clauses name tests in `crates/happenstance-core/tests/wire.rs` instead,
  because the wire format has exactly one implementation and a conformance suite
  exists to police a plurality — the reasoning is in §2's preamble.
- **`Cases:`** — the numbered cases in
  [`spec/E2E-CASES.md`](E2E-CASES.md) the clause serves.
- **`Rejects:`** — the plausible wrong implementation the clause forbids. This is
  CLAUDE.md's standing rule applied to specification clauses: *a rule that no
  adapter can fail is decorative.* A clause that cannot name a wrong
  implementation is `[NON-NORMATIVE]` and belongs in prose.

The obligation to keep those four honest is itself normative, and lives in §6.8 as
CF-35 through CF-38: every clause names a rule and cases, a clause backed only by
integration- or scenario-level cases may not name a conformance rule, every E2E
case names the clauses it exercises, and `cargo xtask ci` runs a traceability
check that fails on any dangling end. **Section 7 is the current state of that
obligation, computed by hand because the checker does not exist yet.** Its three
defect lists are the specification's own bug report.

**Section 1 carries no clauses**, and that is deliberate rather than an omission.
Everything in it is either meta — already normative as CF-35 through CF-38 — or
orientation, which is prose by construction. A clause here would be decorative by
the document's own test.

### 1.5 The dependency rule

**Everything depends on `happenstance-core`; `happenstance-core` depends on
nothing in this workspace.** No adapter may depend on another adapter. The
workspace is `members = ["crates/*", "examples/*", "xtask"]` (`Cargo.toml:3`), and
the rule is what keeps that glob from becoming a graph. `happenstance` — the typed
layer, which today re-exports the contract and adds nothing — is downstream like
everything else; ADR-0006 gave it the bare name, not the root of the graph.

One deliberate exception, and it is a port relationship rather than a dependency
between adapters. `happenstance-sync` is itself a port crate: peer adapters depend
on it the way store adapters depend on `happenstance-core`
(`crates/happenstance-sync/Cargo.toml:14-19`), and its conformance suite will live
in `happenstance-sync-testkit`. It stays out of the contract crate so that
publishing `happenstance-core` never waits on replication.

Two Rust-specific reasons this matters more here than the equivalent rule does in
a language with open classes.

**Coherence decides where a trait can live.** Rust's orphan rule says an `impl
Trait for Type` must live in the crate that defines the trait or the crate that
defines the type. Nowhere else — not in a third crate, not behind a feature, not
with a wrapper unless you are willing to newtype and forward every method. So the
crate holding a port trait is the only crate that can grow the port, and the only
crate every implementer must depend on. That is why `SyncPeer` and `IngestStore`
are specified into `happenstance-sync` (VT-10, SY-8) rather than onto
`EventStore`: putting a replication-shaped method on the store port would oblige
every store adapter to have an opinion about replication, and coherence gives them
no way to opt out. The alternative — define the trait in `happenstance-core` and
let the sync crate implement it — loses because it inverts the dependency: the
contract crate would then be the thing that changes when replication changes.

**Cargo features are additive across the whole build graph.** If two crates in one
build enable different feature sets of a shared dependency, Cargo unifies them and
every consumer gets the union. That is why ADR-0003's guarantee — `serde` is never
in `happenstance-core`'s default features — is only enforceable while nothing in
the graph turns it on for someone who did not ask. `happenstance-sync` turns it on
non-optionally and deliberately (`crates/happenstance-sync/Cargo.toml:15-17`:
*"`serde` is non-optional here: replication is the reason `happenstance-core`'s
serde feature exists"*), which is sound because sync is a leaf that an
application opts into. An adapter-to-adapter dependency would make that opt-in
transitive and silent, and the wasm target would acquire `serde` because some
unrelated crate wanted replication.

### 1.6 The three ports

| Port | Where it lives | What exists today | Maturity | What would freeze it |
|---|---|---|---|---|
| **`EventStore`** | `crates/happenstance-core/src/store.rs:141-316` | Four methods; 89 conformance rules; one reference implementation (`memory.rs:293`) and, since phase 8, one file-backed adapter passing the same suite (`happenstance-sqlite`) | **Frozen at 0.1**, conditional on CF-25 risk acceptance | Already frozen — §3. The exposure, stated precisely because phase 4's freeze cites this cell: §6.5's portfolio carries **seven axes and, at the freeze, no adapter instrument at any far end**; phase 8 put two at far ends this port sits on — durability's and handle multiplicity's, both through `SqliteFixture` — and nowhere else among them; the third it filled, batch shape's, belongs to `ProjectionStore` and is one-sided (§6.5). Four — async flavour, handle multiplicity, durability, position allocation — have a fixture instrument, which by CF-26 buys falsifiability and not implementability; two — transport and completeness — are empty at both ends. Four `ES` clauses hold the residual and are `[PROVISIONAL]` for it (ES-11, ES-12, ES-35, ES-40) — ES-10 was the fifth until phase 4 froze it, and position allocation moved from that list into ADR-0013's CF-25 acceptance rather than off the ledger; the other axes are accepted risk in the landing ADRs |
| **`ProjectionStore`** | `crates/happenstance-core/src/projection.rs` | The trait, and five impls straddling the batch-shape axis — four of them still skeletons — owned write sets in `happenstance-sqlite`, `happenstance-neon` and `happenstance-ladybug`, a live borrowed handle in `live_handle.rs:174`, a `Transaction<'static, Postgres>` in `happenstance-postgres`. **Two** of the five are `todo!()` throughout — `LadybugProjectionStore` (`crates/happenstance-ladybug/src/projection_store.rs:270-292`) and `PostgresProjectionStore` (`crates/happenstance-postgres/src/projection_store.rs:105-127`). The other three carry real bodies: `NeonProjectionStore` in all four methods (`crates/happenstance-neon/src/projection_store.rs:164-202` — its two `todo!()`s are in the free functions `decode_checkpoint` and `decode_commit`, which are not port methods), `LiveHandleProjectionStore` in `begin`, `commit` and `rollback` with only `checkpoint` outstanding (`experiments/live-handle-projection-batch/live_handle.rs:187-223`), and `SqliteProjectionStore` in **all four** since phase 8, `begin` through `rollback` (`crates/happenstance-sqlite/src/projection_store.rs:529-679`). That distinction is the whole reason the count is stated: a `todo!()` has type `!` and coerces to anything, so a body of them proves a signature is nameable, not that it can be satisfied. One of the five now runs against a suite — `SqliteProjectionStore`, through `happenstance_testkit::projection_store_conformance!` at `crates/happenstance-sqlite/tests/projection.rs`, against a real temporary file; the other four still run against nothing | **Provisional**, behind an off-by-default `unstable-projection` feature (PS-3) | PS-2: a hostile store that commits the checkpoint and drops the read-model write must *fail* the suite, and two adapters at opposite ends of the batch-shape axis must pass it |
| **`SyncPeer`** | `crates/happenstance-sync/src/lib.rs` | Two ports in two flavours each — `SyncPeer` (`peer.rs:82`) and `IngestStore` (`ingest.rs:120`) — a `memory` reference peer, and two stand-in peers in the crate's own `tests/`. A phase-2 sketch built to be falsified by a type checker, not the protocol (`lib.rs:3-16`) | **Shape specified, experiments deferred** — 5 of 35 clauses `[DEFERRED]`, 9 `[PROVISIONAL]` | The phase that builds the port against two real peers; §5's deferred clauses name it individually |

The asymmetry is the point. `EventStore` is frozen because it has evidence:
eighty-nine rules — each one shown to reject a named wrong implementation — a
reference implementation, six deployment scenarios walked line by line, and one
pressure test whose contested claims were settled by compiling them. `ProjectionStore` is not frozen because it has none: no adapter
has ever been written against it, and the suite that would freeze it cannot
currently observe half of the invariant the port exists to defend
(`PRESSURE-TEST.md:220-234`). `SyncPeer` is specified before it is built because
the alternative is worse — `RUNBOOK.md:438-439` records that deferring the
sync port *"leaks `EventId` and a tail seam back into `EventStore`"*, and a leak
into a frozen port is not a deferral, it is a decision taken by omission in the
crate that can least afford it.

### 1.7 What is settled, and what will move

**Build on these.** They are `[FROZEN]`, they have named rejects, and the
scenarios have been walked against them.

- **The query algebra.** Types-OR within an item, tags-AND with superset
  matching, items-OR across the query. Thirty-odd boundaries across six domains,
  including several spanning what would classically be three or four aggregates,
  expressed without strain and without one request for a filter the language
  cannot express (`references/scenarios/README.md:1946-1950`). VT-26 through VT-31, and
  ES-15.
- **The two-flavour derivation.** One definition, `trait_variant` derives the
  `Send` flavour, generic code binds the weaker one, `read` returns the stream at
  the top level and is not `async` (ES-1, ES-2). The `Self: Sync` rule that makes
  provided methods and extension traits work is stated once, in ES-3, and `head()`
  and `count()` are shippable today — the claim that they could never be added was
  compiled and refuted (`PRESSURE-TEST.md:32-48`).
- **Event identity.** `EventId` is a store-assigned pair `(StoreId,
  SequencePosition)` on `SequencedEvent`; `StoreId` names a store *incarnation*,
  never a device and never a peer; a foreign identity arrives through
  `IngestStore` in the sync crate rather than through `EventStore::append`. VT-5
  through VT-10.
- **The visibility invariant.** Once a position is visible, nothing below it
  becomes visible later. VT-12 and ES-10. This is the invariant a Postgres adapter
  violates by construction unless it does something about it, which is why
  `happenstance-postgres` is an instrument before it is a target.
- **Ingest never rejects.** A replicated event is never refused for a reason that
  is a function of the receiving store's state; where a local condition would have
  been violated, the losing event and a domain-supplied compensating event land as
  one atomic append. SY-1 through SY-7. The rationale is convergence, not
  politeness: rejection is a function of local state, so a rejecting ingest never
  converges, and a compensation is an append rather than a refusal, so no peer
  deletes a fact its user was told had landed.
- **The wire format is private.** The `serde` feature moves events between
  happenstance instances; it is not an interoperability surface. So the encoding
  is fixed freely — `Query::All` gets an unambiguous tag, every field of every wire
  struct is always present, and a bound change is not a wire change. WF-1 through
  WF-12.

**Expect these to move.** Do not design a downstream crate that depends on the
answer.

- **Whether `happenstance-core` ships the `Send + Sync` marker trait itself**
  (ES-6). The capability question is closed: `Error` keeps
  `core::error::Error + 'static`, and the strength lives in a marker a downstream
  crate can declare for itself over a foreign trait, which is ordinary coherence
  (ADR-0009). What is left is naming and surface, and phase 4 owns it.
- **The whole of `ProjectionStore`** until PS-2's bar is met. The batch shape, the
  write seam, reset, read-your-writes and the failure policy are all specified,
  and all of them are specified against zero adapters.
- **Retention, deletion and completeness** (ES-39, CF-27). A store that has been
  deleted from is currently indistinguishable from a young one at every value in
  §2, and four of the six scenarios reach that from unrelated doors.
- **The scope of replication** — whole-log or filtered (SY-27). It decides whether
  the sync suite's round-trip rule can assert log equality at all.
- **A store-assigned time on `SequencedEvent`** (VT-9), provisional.
- **The tail or subscription seam.** Absent at 0.1 and stated as absent (ES-32),
  not omitted.

**And the honest caveat that outranks all of them.** Section 6.5's portfolio has
seven axes and **three adapter instruments, at three far ends, all three in one
crate**. Four of the seven —
async flavour, handle multiplicity, durability and position allocation — carry a
*fixture* instrument, which proves the rules at that end can fail and says
nothing about whether a real implementation there can pass (CF-26). Two of those
four also carry the other kind, both since phase 8 and both through
`happenstance-sqlite`: it passes the reopen rules against a real file, which is a
real implementation at durability's far end and still not a store that has met a
*fault*; and its fixture hands out second `rusqlite::Connection`s onto one file,
which is a real implementation at handle multiplicity's far end and still not a
connection pool. The third far end is batch shape's, on the other port and from
the same crate: `SqliteProjectionStore` passes `projection_store_conformance!`
at the replay-at-commit end, and the live-transaction end holds nothing that has
run anything, which is why PS-2 is not cleared by it. The other two —
transport and completeness — are empty at both ends. Every
port in this document has been checked against a population of implementations
that agree with it: `MemoryEventStore`, a rusqlite adapter and a planned Durable
Object all serialise their writers and assign positions under a lock they hold
until commit. That is one storage shape wearing several hats. The clauses that
will turn out to be wrong are the clauses that assume a property all of them
happen to share, and §6.5 names which ones those are.

### 1.8 The evidence base

Four documents and one tree. Cited throughout by `file:line`, read at commit
`2a65d76`.

**[`references/evaluation/PRESSURE-TEST.md`](../references/evaluation/PRESSURE-TEST.md)** is
authoritative. It adjudicated the prior evaluation and roadmap by compiling the
discriminating cases rather than by preferring one auditor, and where it
contradicts either of them, it wins. Its §5 ranks the open issues by blast radius
on this specification; its §7 lists what only an experiment can settle, and those
are the `[DEFERRED]` markers' experiments.

**[`spec/E2E-CASES.md`](E2E-CASES.md)** is the case
catalogue: 56 numbered cases at three levels — contract, integration, scenario
(`:17-28`) — each stating what it falsifies and what wrong implementation it
rejects. Its *"What cannot be written yet"* section (`:1490-1585`) lists the eleven
decisions this specification exists to settle. Every normative clause below is
reachable from at least one case.

**[`references/scenarios/README.md`](../references/scenarios/README.md)** is the catalogue of six
deployments, each designed to sit at the far end of a different axis and each
walked line by line against the contract on disk. Its closing section, *"What the
six agree on"* (`:1873-1912`), is the convergence signal — nine holes found
independently by dissimilar deployments, and one thing the design got right.

**`.kb/decisions/0001` through `0007`** are the decisions on record, with the tense
rule of §1.1 applied.

**Superseded, kept for the record only.**
[`ARCHITECTURAL-EVALUATION.md`](../references/evaluation/ARCHITECTURAL-EVALUATION.md) §3 and
[`revised-runway.md`](../references/evaluation/revised-runway.md) are superseded by this
document. Read them as history, not as instruction.

Two of their claims are known false and must not be reintroduced:

- `ARCHITECTURAL-EVALUATION.md:24-26` — that
  `#[trait_variant::make(SendEventStore: Send)]` forbids `EventStore` from ever
  gaining a defaulted method, and therefore that `head()` and `count()` can never
  be added. Compiled and refuted (`PRESSURE-TEST.md:32-54`). A provided method in
  the hand-desugared form `fn f(&self) -> impl Future<Output = T> where Self: Sync
  { async move { … } }` compiles under the existing attribute, and its future is
  `Send` in generic code. Only the `async fn` spelling fails, with `E0728`. That
  false claim is the stated reason for the whole ordering of that document's §3,
  which is why §3 as a whole is superseded rather than corrected.
- `revised-runway.md:1101-1104` — that an owned `Batch` makes the foreign-batch
  hazard unrepresentable. Compiled and refuted (`PRESSURE-TEST.md:179-201`): a
  lifetime names a region, not an instance, and two `&Store` references unify to a
  common region. PS-15 states what actually has to happen instead.

What is *not* superseded is `ARCHITECTURAL-EVALUATION.md` §4, defects D1–D13. All
thirteen were reproduced by execution against the crate
(`PRESSURE-TEST.md:73-82`), and the clauses below that fix them say so where they
do.

---

## 2. Value types and the wire format

This section specifies the values that cross every port in the workspace and the
bytes they become when one instance sends them to another. It settles three of
the eleven decisions [`spec/E2E-CASES.md`](E2E-CASES.md)
lists as blocked: the shape of `EventId` (item 1, in all three of its parts), the
visibility invariant (item 6), and the deletion-adjacent half of item 11 that is
about what a `SequencePosition` means rather than about what a store may do.

Three positions are taken as given by the repository owner and are not reopened
here. **The wire format is private to happenstance** — the `serde` feature moves
events between happenstance instances and is not an interoperability surface, so
the encoding may be fixed freely and the DCB-interop question is deferred with a
named home (WF-1). **Sync ingest is unconditional, with compensation** — the
value-type consequence of that is `EventId`, which is what makes a compensation
idempotent, and it is specified here; the port that carries it is §5's. **All
three ports are in scope** — this section supplies the vocabulary all three share.

Section numbering, because several clauses below hand work across a seam: §3 is
the `EventStore` port, §4 the `ProjectionStore` port, §5 the `SyncPeer` port, §6
the conformance obligation.

### How to read a clause

`MUST`, `MUST NOT`, `SHOULD`, `SHOULD NOT` and `MAY` are RFC 2119. Every clause
carries a maturity marker, the conformance rule that checks it, the E2E cases it
serves, and the wrong implementation it forbids. A clause with no wrong
implementation to forbid is decorative, and where one appears below it is marked
`[NON-NORMATIVE]` and demoted to prose.

Two conventions differ from the other sections and are deliberate.

**Wire rules are tests, not conformance rules.** A conformance suite exists to
police a plurality of implementations. The wire format has exactly one
implementation — the `serde` impls in `happenstance-core` — and by CLAUDE.md's own
standard a rule no adapter can fail is decorative. So `WF-n` clauses name tests
in `crates/happenstance-core/tests/wire.rs` rather than rules in
`happenstance-testkit`. They are no less binding; they are simply in the only
place that can fail them.

**`0.2.0` is published.** `happenstance-core` is on crates.io, so a signature
change below now costs a version bump and a changelog entry rather than only the
edit. Where a clause reverses a decision recorded in the code, it says so.

---

### 2.1 `Event`

#### VT-1 — An `Event` carries what its writer knows and nothing its store knows

An `Event` MUST carry exactly four things: an `EventType`, an opaque `data`
payload, a `Tags` set, and optional opaque `metadata`. An `Event` MUST NOT carry
an identity, a position, a time, or the `AppendCondition` under which it was
appended.

`[FROZEN]`
`Rule:` `append_preserves_event_payload`;
`append_preserves_event_type_and_tags_byte_for_byte`;
`append_preserves_an_empty_payload`;
`metadata_distinguishes_absent_from_empty`
`Cases:` E2E-33, E2E-34, E2E-43
`Rejects:` a store that stamps a writer-supplied identity into the log, which
makes history forgeable by any caller and gives two peers no way to agree on
which of two identically-identified events is the real one. And, on the two
fields that look optional, the adapter that stores `data` and `metadata` in
nullable columns and writes an empty slice as `NULL`: `metadata: None` and
`Some(&[])` both read back as `None`, a zero-length `data` reads back as
absent, and one row mapper produces both. The existing round-trip rule cannot
see either, because the single event `append_preserves_event_payload` builds
carries a thirteen-byte payload and eight bytes of `metadata` — the two values
that make the mapping look total.

The four fields are what `crates/happenstance-core/src/event.rs:321-326` already has,
so this clause changes nothing about `Event` and exists to close the four
questions repeatedly asked of it.

*Identity, position and time are store-assigned* and live on `SequencedEvent`
(VT-4). The split is not stylistic. An `Event` is a value a command handler
constructs before it knows whether the append will be accepted; a store-assigned
fact is by definition unavailable at that moment. Putting any of the three on
`Event` would force every constructor to supply a value it cannot have, and
would make `Event::new` fallible for a second, unrelated reason.

*The append condition is refused as a field.* Kestrel Rotor's `conflict-queue`
projection wants it (`references/scenarios/README.md:1768-1777`): its whole job is to
see which conditions were evaluated. The refusal is not "we forgot" — a
condition recorded on the event is an unattested self-report, because the store
evaluates the condition passed to `append` and never checks that the copy on the
event matches it. A field that can silently disagree with the thing it claims to
record is worse than an absent one. If a deployment needs the audit trail, the
domain writes a fact recording it, and then the fact is subject to the same
scrutiny as every other fact.

#### VT-2 — Structural equality is not identity

`Event` implements `PartialEq` structurally. Appending two structurally equal
`Event` values MUST produce two distinct events with two distinct positions and
two distinct `EventId`s. No store, peer or projection may treat structural
equality as evidence that two events are the same event.

`[FROZEN]`
`Rule:` `appending_equal_events_yields_two_events`
`Cases:` E2E-33, E2E-36
`Rejects:` a content-hash identity scheme, and any ingest that deduplicates on
payload equality. A refrigeration engineer consuming two of the same part on one
work order writes two byte-identical `VanStockConsumed` events; a content hash
collapses them and the van's stock balance is permanently one unit high, with
nothing reporting it.

This is the reason the identity in VT-5 is a store-assigned pair rather than a
hash of the event's contents. It is also why an `Event` is cheap to clone —
`Bytes` clones are refcount bumps — and why `crates/happenstance-core/src/memory.rs`
cloning on the write path is a performance note rather than a correctness one.

#### VT-3 — Everything replication must reason about is in the type or the tags

The contract layer, a store adapter, and a peer MUST NOT parse `data` or
`metadata`. Any value that a store, a peer, a conformance rule or a query must
be able to see MUST be carried in the `EventType` or in `Tags`.

`[FROZEN]`
`Rule:` `append_preserves_event_payload` checks the mechanical half
— that the payload survives byte-for-byte. The "MUST NOT parse" half is not
mechanically checkable and is a review obligation; it is stated as a clause
rather than prose because the *positive* requirement — put it in the tags — is
what an adapter author needs, and it is checkable at review by grepping an
adapter for a payload decode.
`Cases:` E2E-34, E2E-42
`Rejects:` an ingest path
that writes the origin's identity into `Event::metadata`. `Tags` and
`EventType` are the only things `QueryItem::matches` looks at
(`crates/happenstance-core/src/query.rs:113-116`), so an identity in `metadata`
is structurally invisible to the port, and a peer that must deserialise opaque
bytes to decide whether it has already seen an event has broken ADR-0003 at the
exact point ADR-0003 claims to win.

The exemplar this clause was written against was `happenstance-sync`'s own
proposal — "a UUIDv7 or a content hash in the event's **metadata**" — and it
was withdrawn: the crate now proposes `(StoreId, SequencePosition)`
(`crates/happenstance-sync/src/identity.rs:87-101`), which is what VT-5
mandates. The replacement is stronger than the proposal was, because the tree
now *forces* the wrong answer rather than merely suggesting it.
`SequencedEvent` carried a position and an event and nothing else when the
finding was made, so the crate's own compiled finding is that
`impl IngestStore for MemoryEventStore` "cannot be written truthfully" —
nowhere to put an accepted `EventId`, nowhere
to read one back to answer `holds`
(`crates/happenstance-sync/src/ingest.rs:38-50`). `Event::with_metadata`
(`crates/happenstance-core/src/event.rs:377-382`) is then the only free-form
slot left on the type, and it is the one field the contract never inspects.
That is where an implementer under deadline will put it, which is what makes
this clause bite until VT-5 lands on `SequencedEvent`.

This is the standing constraint the scenario catalogue arrived at from six
directions and states in its closing line
(`references/scenarios/README.md:1955-1958`). It has a positive proof as well as a
negative one: Kestrel Cold Chain's hub adjudicates a contested claim and authors
a compensation without parsing a payload, because `outcome`, `unit`, `epoch` and
`supersedes` are all tags (`references/scenarios/README.md:136-155`).

The cost is real and should be stated where a vocabulary is being designed
rather than discovered: a value promoted to a tag becomes indexable, queryable,
and — per E2E-49 — **unerasable**, because crypto-shredding covers `data` and
cannot cover `tags`. "Put it in the tags" is not free advice.

**Prose, not a clause:** `Event::into_parts` (`event.rs:404-427`) used to document
itself as "avoiding a clone in adapter write paths". That was false as written,
because `EventStore::append` takes `&[Event]` (`store.rs:261-265`) and no trait
implementation can reach an owned `Event` at all. The method still earns its
keep on the ingest path, where a peer owns an `Event` it decoded from the wire
and wants to move its parts into an adapter's row struct. The doc comment now
says that instead, as of `3c704d3`. ES-17 owns the correction and records the
same discharge; whether `append` should take an owned batch is §3's question, not
this one's.

---

### 2.2 `SequencedEvent`, identity, and time

#### VT-4 — `SequencedEvent` carries three store-assigned facts

A `SequencedEvent` MUST carry a `SequencePosition`, an `EventId`, a `RecordedAt`
and the `Event`. All four fields MUST be public and the struct MUST remain
`#[non_exhaustive]`.

`[FROZEN]`
`Rule:` `append_stamps_identity_and_time`
`Cases:` E2E-34, E2E-41, E2E-43
`Rejects:` an adapter that returns `SequencedEvent` values whose identity or
time it made up at read time rather than persisting at write time. Such an
adapter passes every rule that reads back what it just wrote inside one process
and fails the first reopen, because the values are not stable.

`SequencedEvent` was `{ position, event }` and `SequencedEvent::new` a
two-argument `const fn` that every adapter and the testkit called. Adding two
fields was therefore one signature change across the workspace, which is exactly
why E2E-CASES insists that identity and time be decided in one pass rather than
two (`E2E-CASES.md:1146-1148`). This clause took that pass, and `new` is
**superseded rather than widened**: arity two to four in one commit, with no
deprecated two-argument arm, because a shim would have to invent a `StoreId` and
a time — which is the wrong implementation the `Rejects:` line above names,
shipped as a convenience.

`#[non_exhaustive]` is what makes `new` the *entire* compatibility surface, which
is the opposite of the protection the attribute is usually credited with. It does
not stop anyone constructing the type; it stops anyone outside the crate
constructing it with a struct literal (`error[E0639]`). So every downstream
construction routes through `new`, and a further **required** store-assigned
fact would supersede it again — `error[E0061]`, with no signature that avoids it.
A *defaultable* field is additive under the `with_*` shape
`SequencedEvent::with_recorded_at` establishes.

`#[non_exhaustive]` is already on the struct (`event.rs:483-484`) and stays: a
per-store `ingested_at`, a retention marker or a redaction flag are all
plausible future additions, and each of them is additive if and only if callers
cannot construct the struct with a literal.

#### VT-5 — `EventId` is a store-assigned pair, minted at append and preserved under replication

An `EventId` MUST be the pair `(StoreId, SequencePosition)` — the incarnation of
the store that first accepted the event, and the position that store assigned
it. A store MUST mint an `EventId` for every event it accepts from a local
append, using its own `StoreId`. A store MUST preserve, unchanged, the `EventId`
of an event it accepts through ingest.

`[FROZEN]`
`Rule:` `event_ids_are_unique_within_a_store`,
`append_stamps_a_local_event_id`; the preservation half by
`happenstance-sync-testkit`'s new `ingest_preserves_origin_identity`
`Cases:` E2E-33, E2E-34, E2E-36, E2E-41, E2E-42
`Rejects:` a store that re-mints an `EventId` on ingest. That is the
implementation the port invites today — `append` has no slot for a foreign
identity (`store.rs:261-265`) so an ingesting peer's store assigns its own — and
it makes re-delivery of a dropped batch produce a second copy of every event.
E2E-33 shows the failure is worse than a duplicate: the hub's own copy now
matches the origin condition, the re-delivered group takes the violated branch,
and the hub **supersedes the event it just accepted**.

Why a pair and not a UUID. Both give uniqueness; only the pair gives a
deterministic, peer-independent total order for free. Sorting merged events by
`(StoreId, SequencePosition)` is a total order every peer computes identically
from data it already holds, which is what E2E-41's convergence check needs and
what Kestrel Rotor's `unit-ledger` is missing
(`references/scenarios/README.md:1754-1758`). A UUIDv7 would give an approximate time
order that disagrees between peers whose clocks disagree, which is the failure
that scenario already suffered with a 2.4-second skew. The pair also costs
nothing to generate: no entropy source, no clock, no allocation — which matters
on `wasm32`, where a random UUID requires a JavaScript binding and a Durable
Object's clock is frozen between I/O operations.

`EventId` MUST be a newtype, not a tuple alias. `type EventId = (StoreId,
SequencePosition)` would give no `Display`, no place to hang the wire encoding
of WF-6, and — decisively — no coherence room: `impl Serialize for (StoreId,
SequencePosition)` is an implementation of a foreign trait for a foreign type
and the orphan rule refuses it outright. A newtype makes both possible and makes
`EventId` and a bare position non-interchangeable at every call site.

`EventId`'s two fields are **private**, with `store()` and `position()`
accessors returning by value. That is ADR-0014's decision and not this clause's,
and the distinction matters because this clause is frozen and that one is not:
every argument above is satisfied by a newtype with public fields. What decides
it is the hazard a transparent pair leaves open — `EventId { store: theirs,
position: mine }` compiles without a word, and that is the exact value the type
exists to make deliberate. It is also why VT-4's "all four fields MUST be
public" must not be over-generalised from `SequencedEvent` to every new value
type: those four are four independent facts a reader wants by name, and these
two are one fact with two halves.

#### VT-6 — `StoreId` names a store incarnation, not a device and not a peer

A `StoreId` MUST be a 128-bit value minted when a store's persistent state is
created. It MUST NOT be derived from a hostname, a device identifier, a peer
name or any other value that survives a restore. A store MUST NOT issue an
`EventId` whose `(StoreId, SequencePosition)` pair it has previously issued for a
different event.

`[PROVISIONAL — falsified by a deployment in which the mint-once path is
unavailable *and* mint-per-open produces so many incarnations that every peer's
watermark, which carries one row per origin for ever, grows without bound. The
candidate is a Cloudflare Durable Object, whose isolate is evicted and revived
as a matter of routine; the first observation is phase 9 and the sync testkit's
restore-and-diverge scenario measures the harm at phase 13]`
`Rule:` `reopened_store_does_not_reissue_an_event_id`, gated on
`Fixture::REOPEN`, which asserts both that two events appended in one open
share a `StoreId` and that an event appended after a reopen has an `EventId`
matching neither; the restore half by `happenstance-sync-testkit`'s new
`restored_peer_does_not_reissue_identities`
`Cases:` E2E-34, E2E-42
`Rejects:` an adapter that keys `StoreId` on the device it runs on. A tablet
restored from Friday's backup then reissues Monday's positions to different
events, every peer's dedup treats the new events as already-seen, and **real
facts are silently dropped** — the one failure mode in the replication design
with no error path and no observable symptom.

The distinction between a device and an incarnation is what E2E-CASES flags as
unsettled (`E2E-CASES.md:1581-1586`) and it has a consequence the ledger row does
not carry: the peer's stable, human-meaningful identity — "the Yorkshire hub",
"tablet 88" — cannot be the `StoreId`, because that name must survive a restore
and the `StoreId` must not. Peer naming therefore belongs to `happenstance-sync`
as configuration, and `EventId` carries an opaque 128-bit value that no operator
will ever recognise. That is the right trade: an operator can look up which peer
an incarnation belonged to; a dedup index cannot recover a fact it dropped.

The clause states the invariant rather than the mechanism because two mechanisms
satisfy it and they suit different adapters. An adapter MAY mint once at schema
creation and provide an explicit re-mint operation the deployment invokes after a
restore — this preserves per-origin causal continuity, which the convergent-fold
argument in VT-5 wants, at the cost of a procedure a human can forget. An adapter
MAY instead mint a fresh incarnation on every open — self-enforcing and
impossible to forget, at the cost of splitting one store's history into many
origins whose interleaving is no longer recoverable.

The preference is a conditional with a stated default rather than a bare SHOULD,
which is what makes the undetectable case safe by construction instead of by
procedure. An adapter MAY take the first mechanism **only if** it can detect that
its state was restored or cloned, **or** the deployment is documented to invoke
the re-mint. An adapter that can do neither **MUST** mint a fresh incarnation on
every open. Either way it MUST record which mechanism it chose in
`references/adapter-shapes.md`.

`happenstance-core` mints nothing: `StoreId::from_bytes` is the only
constructor, and there is deliberately no random one — the crate has no entropy
source, is `no_std`-capable, and on `wasm32` randomness is a JavaScript binding.
A `StoreId` MUST be persisted as bytes or as text and **never** as an integer or
a pair of integers, because Workers SQL widens integers through a JS number and
bounds them at 2^53 (`references/adapter-shapes.md:220`), and a 128-bit value cannot
survive that at all.

#### VT-7 — `EventId` is outside the query language

`Query` and `QueryItem` MUST NOT gain an identity dimension. `EventId` MUST NOT
be represented as a `Tag`. A peer MUST be able to ask whether a store already
holds a given `EventId` through a dedicated port operation, without constructing
a `Query` and without parsing any payload.

`[FROZEN]`
`Rule:` `event_id_is_not_matchable_by_query`; the membership operation by §3's
`contains_event_id_reports_membership`
`Cases:` E2E-32, E2E-34, E2E-36
`Rejects:` the tag-materialised identity Kestrel Rotor proposes —
`oid:sov-aurora-3f9c#38103` on every event — which is presented as free and is
not. It puts a maximally high-cardinality entry in the one column adapters are
told to index; it enters every `contains_all` merge-scan
(`crates/happenstance-core/src/tag.rs:347-361`) on every query in the system; it makes
identity writer-forgeable; and because tags participate in matching, every
tag-only query in every domain now ranges over an identity dimension nobody
asked for.

There is a structural reason as well as a cost reason, and it is the decisive
one. `Query`'s algebra is types-OR within an item, tags-AND with superset
matching, items-OR across the query — and VT-31 freezes it, because E2E-32's
fan-out runner is correct only if `Items(a) ∪ Items(b) == Items(a ++ b)`. An
identity axis is not a set-superset predicate; it is a point lookup on a unique
key. Grafting it onto `QueryItem::matches` would give the item a third semantic
with different composition rules, and the union identity would stop holding.
Dedup is a membership test, so it gets a membership operation.

#### VT-8 — `EventId` uniqueness is a store-level guarantee

A store MUST hold at most one event per `EventId`. This MUST be enforced by the
store, not by the caller.

`[FROZEN]`
`Rule:` `event_ids_are_unique_within_a_store`
`Cases:` E2E-33, E2E-36
`Rejects:` an ingest implementation that establishes idempotence by reading
first and appending second. Two round trips instead of one, which is fatal in
Kestrel Rotor's 34-minute satellite window, and — worse — an unclosed race when a
depot ingests from two peers concurrently, because nothing between the read and
the append excludes the other ingest.

This clause is what makes E2E-36 answerable in a bounded number of round trips
without adding per-event conditions to `append`. The catalogue lists three ways
to get idempotent bulk ingest and rejects all three
(`E2E-CASES.md:935-957`); the fourth is a uniqueness constraint the store already
has to maintain an index for, and the cost is one unique index on
`(store_id, origin_position)`. E2E-36's own conclusion names it: "a
**store-level uniqueness guarantee on `EventId`** that the port states and the
testkit checks. The second is cheaper and lands with the already-decided
identity row."

What a store does when ingest presents an `EventId` it already holds — skip that
event and continue, rather than fail the batch — is SY-11's clause in §5, because
it is a property of the ingest operation and not of the value. The uniqueness is
here because it is a property of the store's contents. SY-14 defers the round-trip
cost of exploiting it, and that deferral does not weaken this clause: the
uniqueness guarantee is required either way.

#### VT-9 — A store records when it accepted an event, and that time is not an ordering key

A store MUST stamp every event it accepts from a local append with a
`RecordedAt`: milliseconds since the Unix epoch, UTC, as a signed 64-bit value.
A store MUST preserve, unchanged, the `RecordedAt` of an event it accepts through
ingest. No store, runner or projection may derive an ordering from `RecordedAt`,
and the contract MUST NOT state any relationship between `RecordedAt` order and
`SequencePosition` order.

`[PROVISIONAL — falsified by a target that cannot supply a wall clock at append
time; a Cloudflare Durable Object returning a frozen clock between I/O
operations is the candidate, and the Workers skeleton is the instrument]`
`Rule:` `append_stamps_a_recorded_time`, `recorded_time_survives_a_reopen`;
the "not an ordering key" half by §5's new
`convergent_projection_is_interleaving_independent` (SY-20), which fails any fold
that reads either the position or the time. Neither of the first two rules may
compare two `RecordedAt` values in either direction, compare a `RecordedAt`
against a `SequencePosition`, or check a value for plausibility against the
harness's own clock — which CF-33 forbids independently.
`Cases:` E2E-41, E2E-43
`Rejects:` a receiving store that overwrites an ingested event's `RecordedAt`
with its own arrival time. That destroys the only audit answer the field exists
to give — Kestrel Rotor's "which side of midnight on 30 September did this issue
fall on" — and replaces it with a value the reader already has in
`SequencePosition`. It also rejects a projection that folds on `RecordedAt` to
resolve a conflict, which looks like the obvious fix for Rotor's divergence and
is not one: the two clocks in that scenario were 2.4 seconds apart and one tablet
in Kestrel Cold Chain was six minutes fast after a factory reset.

Every timestamp in all six scenarios today is a writer's clock inside an opaque
payload, which means it is unverifiable, unqueryable and, per ADR-0003,
unreadable by anything in the workspace. A store-assigned time is a different
kind of value: it is the one clock reading whose provenance the log itself
attests.

The prohibition on ordering binds the *contract*, and the conformance suite is
the contract's executable form: an adapter author reads a failing rule as a
requirement, so a rule asserting an order states one. That is why the two rules
above assert only presence and stability, and it is why `RUNBOOK.md`'s "a rule
that it is non-decreasing with position" was struck rather than written. A store
on a machine whose clock steps backwards under an NTP correction is conformant.

`RecordedAt` MUST be a newtype over `i64`, not an alias. Three reasons, and the
third is the one that decides it. An alias gives no `Display`, so every log line
prints a bare integer. An alias gives no protection against passing a
position-derived integer where a time is expected. And an alias forecloses the
`std` conversions: `impl From<RecordedAt> for SystemTime` is permitted because
`RecordedAt` is local, whereas `impl From<i64> for SystemTime` is a foreign trait
implemented for a foreign type with no local type anywhere in the trait
reference, and coherence refuses it. The conversions live behind the `std`
feature; the contract crate gains no time dependency.

Milliseconds rather than nanoseconds because an `i64` of nanoseconds runs out in
2262 and no question any scenario asks needs sub-millisecond resolution. Signed
rather than unsigned because pre-1970 costs nothing to represent and an unsigned
epoch is a well-known source of wrapping arithmetic in code that subtracts two
timestamps.

#### VT-10 — An ingesting peer supplies a foreign identity through `IngestStore`, in the sync crate

`EventStore::append` MUST NOT accept a caller-supplied `EventId` or `RecordedAt`.
The operation that accepts them MUST be defined by a separate trait, `IngestStore`,
in `happenstance-sync`, implemented by a store adapter behind an optional feature.

`[PROVISIONAL — falsified if a store adapter cannot implement `IngestStore`
without duplicating `append`'s write path, in which case the operation belongs on
`EventStore` after all; the SQLite adapter is the instrument and it settles the
question the first time it implements both]`
`Rule:` §5's `happenstance-sync-testkit` suite — `IngestStore` is the trait every
`SY-n` ingest clause is written against, and SY-5, SY-11 and SY-12 are its
behavioural rules; here, the compile-level obligation is a new testkit compile
test `append_does_not_accept_a_foreign_identity`
`Cases:` E2E-33, E2E-35, E2E-36, E2E-39, E2E-42
`Rejects:` an `EventStore::append` that takes `&[(Event, Option<EventId>)]` or
similar. Every local command handler in every application then carries an
identity slot it must pass `None` for, and any caller can forge a foreign
identity into the log — including one that collides with a peer's real event,
which is the one collision VT-6 spends an entire clause preventing.

Three options were live and PRESSURE-TEST §5 issue 2 records that only two of
them were written down.

*Change `append`.* Rejected above. It also reopens a frozen signature for the
benefit of a code path most deployments never run.

*A provided or required method on `EventStore`.* This is the option nobody wrote
down as rejected, and it is closer than it looks: PRESSURE-TEST §1 establishes by
compilation that `trait_variant` accepts provided methods in the hand-desugared
form, so `ingest` could ship as a defaulted method today. It loses on scope, not
on mechanics. Ingest is meaningless without the rest of the replication design —
the compensation, the peer watermark, the condition policy — and putting its
entry point in the contract crate puts `happenstance-core`'s publish schedule behind
`happenstance-sync`'s design, which is precisely what
`crates/happenstance-sync/src/lib.rs:37-40` says the crate exists to avoid.

*A trait in `happenstance-sync`.* Chosen. It is the posture that crate already
adopts for itself — "this crate defines the third [port]"
(`crates/happenstance-sync/src/lib.rs:19-23`) — and the coherence question resolves cleanly: the adapter
crate depends on both `happenstance-core` and `happenstance-sync`, so
`impl IngestStore for SqliteEventStore` has a local type in the trait reference
and the orphan rule permits it. What is *not* possible, and is worth saying
before someone tries, is a blanket `impl<S: EventStore> IngestStore for S`: there
is no way to express "insert this row with this identity" in terms of `read` and
`append`, and even if there were, a blanket implementation would make it
impossible for an adapter to supply a better one, because coherence forbids the
overlap.

The feature gate matters for a mundane reason: `happenstance-sync` depends on
`happenstance-core/serde` non-optionally (`crates/happenstance-sync/Cargo.toml:15-17`),
so an adapter that implements `IngestStore` unconditionally drags `serde` into
every build of that adapter. `sqlite/sync` and its siblings keep the default
build free of it.

---

### 2.3 `SequencePosition`

#### VT-11 — Positions are unique, strictly increasing in assignment order, and may have gaps

Within one store, every event MUST have a unique `SequencePosition`. Each append
MUST be assigned positions strictly greater than every position the store has
previously assigned. Gaps are permitted. A `SequencePosition` MUST NOT be treated
as a count, and the difference between two positions MUST NOT be treated as a
number of events.

`[FROZEN]`
`Rule:` `positions_are_unique`, `positions_are_strictly_monotonic`, plus the
concurrency family's `positions_are_unique_under_concurrent_appends` — the
uniqueness sentence asked of a store with several writers inside it at once.
Sequentially the read of the counter and the write that consumes it are
adjacent, so neither rule above can separate a store that allocates atomically
from one that reads its head, suspends, and then allocates from the value it
read; that is `RacingSequenceStore`, the in-process form of a
`SELECT max(position)` taken outside the transaction that will use it.
`Cases:` E2E-10, E2E-46
`Rejects:` an adapter that reuses a position after a delete, and one that packs
positions into a dense range on compaction. Both are natural first cuts on a
store that has performed a retention purge, and both break every checkpoint and
every stored `after` in the deployment.

The two existing rules are, on their own, close to vacuous — PRESSURE-TEST §3.9
establishes that both read back through `read`, which every adapter returns in
position order, so a store that sorts on read satisfies them for free. They are
retained because they are cheap and because in combination with VT-12 they stop
being free: a store cannot both sort on read and honour visibility ordering
unless the underlying assignment really is ordered.

The prohibition on arithmetic is already documented at `event.rs:215-217` and is
repeated here because a purge marker built as
`{positionsDeleted: 63, lowest: …, highest: …}` — Kestrel Motor's own compliance
artefact — makes exactly this mistake, and it is a false compliance record rather
than a slow query.

#### VT-12 — The visibility invariant is stated once, at ES-10

The requirement that nothing below an observed position becomes visible later is
an obligation on a **store**, not a property of a position, so it lives with the
port that binds stores: see **ES-10** (§3.3), which is its sole statement and
carries its maturity marker, its rule and its rejected implementation.

This ID is retained, and deliberately not reused, because `PS` and `SY` clauses
cite it and because the history is instructive: the invariant was originally
stated here *and* at ES-10, and the two copies acquired different maturity markers
within a single editing pass. A requirement stated twice is a requirement that
will be half-amended. Where §2 needs the invariant — VT-11's position semantics,
VT-30's condition boundary — it cites ES-10 rather than restating it.

`[NON-NORMATIVE]` — a pointer, not a requirement.
`Rule:` none of its own; ES-10 names `nothing_below_an_observed_position_appears_later`.
`Cases:` see ES-10 (E2E-01, E2E-02, E2E-08).


#### VT-13 — `next()` signals overflow, and the inclusive/exclusive asymmetry is deliberate

`SequencePosition::next()` MUST return `None` on overflow. `ReadOptions::from`
MUST remain an inclusive lower bound and `AppendCondition`'s `after` MUST remain
an exclusive lower bound; the contract MUST document that `checkpoint.next()` is
therefore the correct resume idiom and that it is sound on a store with gaps.

`[FROZEN]`
`Rule:` unit test `position_next_signals_overflow`; `read_from_is_inclusive`,
`condition_after_ignores_events_at_the_boundary`; new
`read_from_a_gap_position` (ES-9's name for it)
`Cases:` E2E-10, E2E-16
`Rejects:` for the overflow half, the implementation this clause was written
against — `Self::new(self.0.get().saturating_add(1))` returned `Some(u64::MAX)`
where its own documentation promised `None`, because `saturating_add` cannot
signal overflow and the method whose sole purpose is signalling overflow
therefore never did. Repaired at phase 4: `event.rs:272-282` is
`NonZeroU64::checked_add` in `match` form, which is `const`, costs no byte —
the newtype's forbidden zero is the niche `Option` spends on `None` — and is
asserted by `position_next_signals_overflow`. For the resume half, an adapter that implements
`from` as an equality seek or a `rowid` offset rather than a range predicate —
plausible on a store whose positions came from a counter, and fatal after the
first purge.

The blast radius of the overflow bug is nil in practice; it is fixed because a
contract crate returning a wrong answer through a signature that has a way to say
"I cannot" is the wrong trade, and because library code may not `unwrap` and will
therefore have to branch on the `Option` anyway.

The asymmetry is worth stating rather than leaving to be re-derived. Norvant
confirms it is sound (`references/scenarios/README.md:1032-1036`): because `from` is an
inclusive lower bound rather than a seek, and gaps are permitted,
`checkpoint.next()` resumes correctly even when the very next position does not
exist. Every adapter author will otherwise reconstruct that argument from
scratch, and `projection.rs:101-110`'s instruction to "advance past" the checkpoint
by hand is what sends them looking.

**Discharged in the code at phase 4.** `SequencePosition::next`'s own
documentation (`event.rs:259-271`) now states that this is the resume idiom on
**every** adapter and that its soundness over gaps comes from `ReadOptions::from`
being a threshold (ES-9) rather than from the adapter allocating densely. The
sentence it replaced — *"only meaningful for adapters that allocate positions
densely"* — was the exact belief that produces an exact-seek `from`.

---

### 2.4 `EventType`, `Tag` and `Tags`

#### VT-14 — Character validation rejects control characters and explicit bidirectional overrides

`EventType::new` and `Tag::new` MUST reject an empty value, a value longer than
the applicable byte bound, any character in Unicode general category `Cc`, and
the seven explicit bidirectional formatting controls (U+202A–U+202E,
U+2066–U+2069). They MUST NOT reject Unicode category `Cf` generally.

`[PROVISIONAL — falsified by a legitimate event type or tag that requires an
explicit bidirectional formatting control; no such value is known and the
scenarios contain none]`
`Rule:` unit tests `rejects_invalid_event_types`, `rejects_invalid_tags`, and
`validate`'s own `rejects_c0_del_and_c1_controls`,
`rejects_all_seven_bidirectional_controls`,
`accepts_the_format_characters_scripts_need` and
`accepts_neighbours_of_the_closed_list`, which cover every arm this clause
states — the C1 range, the seven controls, the `Cf` characters scripts need, and
the four codepoints bracketing the two closed runs;
`store_accepts_a_max_length_identifier`, which VT-20 claims too — one round trip,
two clauses with a stake in it, and the boundary here is the boundary of
*validity*; `store_accepts_non_ascii_identifiers` — a Persian tag, a Devanagari
one and an emoji sequence carrying U+200D, appended and read back byte-for-byte. The constructors' half of this clause is a unit test; the store's
half is a rule, because the value the constructor accepts still has to survive a
column.
`Cases:` E2E-40
`Rejects:` a validator that bans all of `Cf`, which is the natural
over-correction and which breaks Persian, Hindi and every emoji sequence by
rejecting U+200C and U+200D. And a validator that bans only ASCII C0, which is
what four doc comments in the crate currently claim happens. On the store side it
rejects an ASCII-only column — `VARCHAR` under a `latin1` collation, or a `CHECK`
written against a `[[:ascii:]]` class — and an adapter that honours the 255-byte
bound by truncating bytes, which splits a codepoint and stores a tag that is not
valid UTF-8 and therefore matches nothing, including itself.

The behaviour was already right and the documentation was already wrong when this
clause was written. `char::is_control` — then the check in `event.rs` and
`tag.rs` — is Unicode `Cc`, which includes the C1 range U+0080–U+009F, while four
doc sites said "ASCII control characters". Four sites, one two-word fix, and the
fix was to the prose.

**Both halves were carried out at phase 4.** `validate::check`
(`crates/happenstance-core/src/validate.rs`) is a single `const fn` byte walk
shared by `EventType::new`, `EventType::from_static`, `Tag::new` and
`Tag::from_static`, so the four constructors cannot diverge (VT-32); it rejects
the C1 range and the seven bidirectional controls and accepts `Cf` otherwise;
and the four doc comments now say "control characters (Unicode `Cc`)". The
compiled finding worth keeping is that a **byte walk sees all of this**: this
clause's own text implied a general-category predicate was the only way to reach
`Cc`, and the byte walk was checked against a `char` walk over all 1,112,064
scalar values, each alone and each embedded between two ASCII letters, with zero
disagreements (ADR-0015, experiment E11). That is what made one validator
affordable, and one validator is what makes `from_static` exactly as strong as
`new`.

The bidirectional half is a new decision and it is narrow on purpose. U+202E in
an event type is a log-spoofing vector — a type that renders as one thing in
every console and matches another in every query — and the seven override and
isolate characters are the only `Cf` codepoints no script needs. The check is a
`matches!` over a closed list with no dependency, which is the reason it is
affordable when a general-category predicate is not: `core` exposes no such
predicate and pulling in a Unicode tables crate to get one would be the largest
dependency in the contract crate by an order of magnitude.

Zero-width space (U+200B) and the other invisible `Cf` characters remain legal
and remain a hazard: two visually identical tags that are two different
consistency boundaries. That is the price of not banning `Cf`, it is named here
so it is a decision rather than an oversight, and the mitigation belongs to the
typed layer's validating constructors and to a lint, not to the contract.

#### VT-32 — Identifiers are const-constructible, and both constructors enforce one rule

`EventType` and `Tag` MUST be backed by `Cow<'static, str>` and MUST offer
`pub const fn from_static(&'static str) -> Self`. `from_static` and `new` MUST
accept exactly the same set of values. `Eq`, `Hash` and `Ord` MUST be
hand-written and MUST agree with `str`'s. The documentation MUST state that an
associated `const` which is never read is never evaluated, and therefore that an
invalid one survives `check`, `clippy`, `build` and `test`.

`[FROZEN]`
`Rule:` unit tests `from_static_and_new_agree`,
`from_static_rejects_a_bidirectional_control` and
`from_static_rejects_a_c1_control`, which walk the control, C1 and
bidirectional boundaries through *both* constructors; and a `compile_fail`
doctest on `EventType::from_static` at a **free `const`** call site, which is
the only site where rustc is obliged to evaluate the constant. A `trybuild`
snapshot would pin the diagnostic as well as the failure; phase 6 owns that
dependency decision (`references/adapter-shapes.md:97-102`).
`Cases:` E2E-40, E2E-51
`Rejects:` a `from_static` that skips validation, and a `new_unchecked` beside
it — either makes the `const` path a hole in a type whose whole value is that
an invalid one is unrepresentable. Any pair of constructors enforcing different
rules, which is the divergence a `const`-compatible byte walk *appears* to force
and does not: one `const fn` serves both. And a derived `Eq`/`Hash`, which is
correct today over a single-field tuple struct and silently changes meaning the
moment a second field lands — phase 4 is adding fields to neighbouring types for
exactly that reason (VT-4), and a `HashMap` whose key type has an `Eq` its
`Hash` does not respect loses entries with no diagnostic anywhere.

`Cow<'static, str>` rather than `&'static str` or `Box<str>`, and the reason is
ingest. `Cow`'s two arms express "written in the source and baked into the
binary" and "arrived from a peer at run time and had to be allocated" in one
type; a newtype over `&'static str` can only express the first and makes a
runtime-derived event type unrepresentable. It costs eight bytes against
`Box<str>` — 24 against 16 — in exchange for removing an allocation and a
validation from every event construction on the hot path. A `const` is inlined
at each mention rather than moved, so what is shared is the `&'static str` in
`.rodata` and not the value; the gain is zero allocations per construction, not
one object, and claiming the other one would be wrong.

**The hazard the documentation MUST carry is sharper than it looks.** `assert!`
in a `const fn` turns invalid input into a compile error only where the compiler
is obliged to evaluate the constant, and Rust's rules about when that happens
are not intuitive. Measured, all four cases run rather than argued (ADR-0015,
E11): a free `const` fails at `cargo check`; an associated `const` that is
**read** somewhere fails at `build`/`test` but not at `check` or `clippy`; an
associated `const` that is **never read** is never evaluated and passes all
four; and a `let` binding compiles and panics at run time. The third is the one
to design around — a `pub const TY: EventType` in a codec registry that nothing
references holds an invalid value and nothing in the gate sees it.

#### VT-33 — The standard-library trait surface

`Tag` and `EventType` MUST implement `Borrow<str>` and `FromStr`. `Tags` MUST
implement owned `IntoIterator` and `Extend<Tag>`, and MUST document that
`extend` re-canonicalises and is therefore **not** the amortised O(1) `Extend`
usually implies. `Tags` MUST offer `values_of(key)` returning every value under
a key. `Event` MUST NOT implement `Hash`.

`[FROZEN]`
`Rule:` unit tests `a_borrowed_and_an_owned_tag_are_one_value`,
`a_map_keyed_by_event_type_is_probed_by_str` — which is a real `HashMap` probe
by `&str`, because `Borrow`'s extra obligation is that the borrowed form hashes
*and* compares as the owner and a comment claiming it is not a check —
`extend_re_canonicalises`, `owned_into_iterator_yields_canonical_order`, and the
six `values_of_*` tests in `tag.rs`.
`Cases:` E2E-40, E2E-51
`Rejects:` a codec registry whose every `HashMap` probe constructs an
`EventType` — an allocation and a re-validation to look up a handler, which is
what the absence of `Borrow<str>` costs at every call site without ever
producing a diagnostic that names the cause. A `Borrow<str>` whose hash
disagrees with `str`'s, which loses map entries silently and is the specific
risk hand-writing `Eq`/`Hash` (VT-32) takes on. And a content-hash dedup on
`Event`, which treats two legitimately identical domain events as one — a
`SeatReleased` for the same seat twice is not a duplicate — where the correct
instrument is the store-assigned `EventId` (ADR-0014) and the policy is phase
13's.

The `Event`-`Hash` prohibition is the one part of this clause a human should
push back on if they disagree, and it is priced rather than waved through:
`[FROZEN]` means undoing it costs a superseding ADR, so phase 13 cannot simply
add the impl if it later finds a job a content hash can do that `EventId`
cannot. That is the intended cost — the impl is one line, the wrong use of it
is one line, and neither is visible in review — but it is a frozen MUST NOT
written on phase 13's behalf on phase 4's evidence. The alternative offered and
not taken was to leave the prohibition in `Rejects:` only, where it would
explain the wrong implementation without forbidding a right one.

#### VT-15 — Equality is byte equality; the contract normalises nothing

`Tag` and `EventType` equality MUST be byte equality over the UTF-8 encoding. The
contract, and every adapter, MUST NOT apply Unicode normalisation, case folding,
trimming or any other transformation to a tag or an event type. Whitespace is
significant everywhere, including leading and trailing whitespace. `Tag`'s
documentation MUST state that no normalisation is applied and MUST name the
NFC/NFD case.

`[FROZEN]`
`Rule:` `tags_differing_only_by_unicode_normalisation_are_distinct`,
`append_preserves_event_type_and_tags_byte_for_byte`
`Cases:` E2E-40, E2E-49
`Rejects:` an adapter that NFC-normalises tags on write — reasonable-looking,
and it rewrites a caller's data so that the tag read back is not the tag written,
which breaks byte-faithful replication and makes the store's index disagree with
any external system holding the original string. Also rejects a Postgres adapter
using a case-insensitive or `ICU` collation on the tag column, which silently
merges two consistency boundaries.

The position is the same one `tag.rs` already takes about trimming
(`references/scenarios/README.md:1786-1793`) and it must be stated because everyone
assumes the opposite. The DCB specification treats a tag as an opaque string; a
contract that normalises is deciding what a caller's identifier means.

The cost is concrete and belongs next to the rule. Kestrel Rotor replicated a
`SerialisedUnitConsumed` carrying `turbine:HW2-A14 ` with a trailing space;
`Tag::new` accepts it, `Tags` sorts it adjacent to the real tag, `contains_all`
is a strict merge-scan on equality (`tag.rs:347-361`) and does not match it, and
a lot-recall query silently missed a turbine. `"café"` in NFC and NFD is the same
failure with no visible cue at all. Both are real, both are the application's to
prevent, and the contract's obligation is to say so loudly rather than to guess
which normal form the caller meant.

#### VT-16 — `Tags` is canonical: sorted, deduplicated, set-equal

A `Tags` value MUST be sorted by the byte ordering of the whole tag string and
MUST contain no duplicates. Equality MUST be set equality. `Tags::contains_all`
MUST implement subset semantics — every tag of the argument present in the
receiver — and a partial overlap MUST NOT match.

`[FROZEN]`
`Rule:` `query_item_tags_are_and`, `query_item_tags_match_supersets`,
`query_item_rejects_partial_tag_overlap`
`Cases:` E2E-32, E2E-40
`Rejects:` an adapter that implements a tagged query with `IN`-style semantics —
any tag matching rather than all tags matching. It passes the OR rules and the
superset rule and fails only the partial-overlap rule, which is exactly why that
rule exists. Also rejects an adapter that persists tags in insertion order and
compares them positionally, which makes two equal `Tags` values unequal after a
round trip.

Canonicalisation at construction is what buys the linear merge-scan, set-valued
`PartialEq` and `Hash`, and a stable serialisation for an index — `tag.rs:246-253`
states all three. The infallible `FromIterator` impl (`tag.rs:479-487`) is
retained, because after VT-19 the only invariant `Tags` carries is canonicality
and `FromIterator` cannot violate it. This reverses a proposal to remove it: the
argument for removal was that an infallible constructor could produce a value the
fallible one rejects, which is the D2 mistake one type over, and that argument
evaporates once tag *count* is a store limit rather than a type invariant.

#### VT-17 — `key:value` is a convention and the contract does not enforce it

The contract MUST NOT enforce that a tag contains a colon, that a key appears at
most once in a `Tags` set, or that `Tag::key` returns `Some`. `Tags` MUST NOT
offer a `get(key)` accessor.

`[FROZEN]`
`Rule:` `tags_may_repeat_a_key`
`Cases:` E2E-57, written at phase 4 to close this clause's own hole — it comes
from Wattline D3 (`references/scenarios/README.md:628-635`) and §7.5 carried it as a
defect until then. It stays a clause rather than prose because it is
mechanically checkable and because the wrong implementation is one an adapter
author will reach for.
`Rejects:` an adapter that indexes tags as a key→value map. `Tags::from_pairs([
("tenant", "a"), ("tenant", "b")])` succeeds and yields a two-element set —
deduplication is on the whole `key:value` string (`tag.rs:374-381`) — so a
map-shaped index silently drops one of them, and the event then matches only one
of the two tenants' queries. On a 4,200-tenant shared log that is a cross-tenant
correctness failure produced by an indexing choice.

Enforcing key-uniqueness would put a convention in the contract and exceed the
DCB specification, which treats tags as opaque strings. The absence of
`Tags::get` is the same decision seen from the read side: an accessor that
returns one value where two may exist would make the ambiguity invisible rather
than resolving it. An accessor returning **every** value under a key —
`Tags::values_of(key)` — is permitted and is what the contract ships, because it
does not have the defect this paragraph describes; the MUST NOT is on
`get(key)`'s shape, not on prefix lookup. The documentation on `Tags` MUST say
the convention is unenforced and MUST name the repeated-key case, because a
reader of `Tag::key_value` reasonably assumes otherwise.

#### VT-18 — Constructors accept values the caller already holds, and their errors compose

`Event::new` MUST accept an already-constructed `EventType`. The three
validation error types MUST compose: `InvalidQuery` MUST implement
`From<InvalidTag>` in addition to its existing `From<InvalidEventType>`.

`[FROZEN]`
`Rule:` compile tests `event_new_accepts_a_held_event_type` and
`command_handler_composes_validation_errors` in `crates/happenstance-core/tests/`
`Cases:` E2E-51
`Rejects:` the signature this clause was written against. `Event::new` took
`impl TryInto<EventType, Error = InvalidEventType>`, and the
blanket `impl<T, U: From<T>> TryFrom<T> for U` gives `Error = Infallible`, so the
equality constraint excluded the one conversion that cannot fail. Passing a
`const COURSE_DEFINED: EventType` produced `error[E0271]`. That blocked the typed
layer from interning one `EventType` per `DomainEvent`, which is the whole point
of interning it, and it had to land before any code was written against
`Event::new`.

**Discharged at phase 4.** `Event::new` is
`T: TryInto<EventType>, InvalidEventType: From<T::Error>`
(`event.rs:351-361`), with the losing alternative recorded in the comment beside
it (`event.rs:353-358`), and `impl From<Infallible> for InvalidEventType` at
`error.rs:75-87`.

The crate already knows the fix and applied it two files over.
`QueryItem::new` writes `T: TryInto<EventType>, InvalidQuery: From<T::Error>`
(`query.rs:57-61`) with `impl From<Infallible> for InvalidQuery`
(`error.rs:117-124`), and its own doc comment says that is what lets it accept
both. A `From` bound admits both conversions where an equality constraint admits
one, because `Infallible` converts into anything by matching on an uninhabited
value — `match never {}` — which is why the impl at `error.rs:121-123` has no arms.
`Event::new` needed the same shape plus
`impl From<core::convert::Infallible> for InvalidEventType`, and that is what it
now has.

The error composition is the second half of the same ergonomic problem. A command
handler in a library crate cannot use `anyhow`, calls `Tags::from_pairs`
(`InvalidTag`), `QueryItem::new` and `Query::from_items` (`InvalidQuery`), and
propagates with `?` — and needed a bespoke union enum before it wrote a line of
domain logic. Every code snippet in all six scenarios has this shape, and every
one of them compiled only inside a `Box<dyn core::error::Error>` doctest, which
is how the crate's own doctests escape it (`query.rs:135-148`, `append.rs:54-63`).
**Discharged at phase 4:** `InvalidQuery::Tag(#[from] InvalidTag)`
(`error.rs:106-114`) is the variant and the `From` impl in one. Collapsing the
three enums into one `InvalidInput` was the alternative and it loses: the three
are returned by three different constructors and matching on which one failed is
worth keeping, `#[non_exhaustive]` enums are cheap, and the conversion direction
is unambiguous because a query can contain tags and a tag cannot contain a query.

---

### 2.5 Bounds

The scenarios found that the contract bounds the two values no storage engine
struggles with — `MAX_EVENT_TYPE_LEN` and `MAX_TAG_LEN`, both 255
(`event.rs:16`, `tag.rs:16`) — and leaves unbounded the three that cost money:
payload size, tag count and batch size. The correction is not four more
constants. It is a distinction between two kinds of bound, and then floors rather
than ceilings.

#### VT-19 — There are two kinds of bound and they are enforced in different places

A **validity invariant** MUST be enforced by the value's constructor and MUST be
re-enforced on deserialisation; a value violating one is unrepresentable. A
**capacity limit** MUST NOT be enforced by a constructor and MUST NOT be
enforced on deserialisation; it is enforced at the store boundary and produces a
distinguishable runtime error.

`[FROZEN]`
`Rule:` `wire::decode_rejects_a_non_canonical_tag_set`,
`wire::decode_accepts_an_over_capacity_value`,
`append_reports_exceeded_store_limits`
`Cases:` E2E-40, E2E-42
`Rejects:` an implementation that enforces a capacity limit in
`Deserialize`. That is the natural reading of "validate on the way in", and it
destroys the quarantine path: a peer running a tighter bound than its neighbour
cannot decode the event at all, so it cannot identify it, cannot report which
event was refused, and cannot park it for a human. E2E-42 then bites — an event
a peer never appended has no local position, is never forwarded, and disappears
permanently from a third peer's view.

This distinction is what answers PRESSURE-TEST §5.4's observation that every new
bound is "a wire-compatibility break with no quarantine path". It is not a wire
break at all: the bytes are unchanged and the acceptance set changes. See WF-9.

#### VT-20 — The two length bounds stay at 255 and stay validity invariants

`MAX_EVENT_TYPE_LEN` and `MAX_TAG_LEN` MUST remain public constants equal to 255
bytes, and MUST remain enforced by `EventType::new` and `Tag::new` and on
deserialisation.

`[FROZEN]`
`Rule:` unit tests `rejects_invalid_event_types` and `rejects_invalid_tags`;
`store_accepts_a_max_length_identifier` — named for what the store does rather
than for what the test does, on the rule the six §6-versus-§3 pairs were settled
on: the name states the observable behaviour, not the mechanism. VT-14 claims the
same rule, and deliberately. There is one round trip and two clauses with a stake
in it — VT-14 owns the boundary of *validity*, this clause the boundary of the
*constant* — so a second rule under a second name would buy two tests that can
only ever fail together.
`Cases:` E2E-40
`Rejects:` an adapter that truncates an over-length tag to fit its column. It
produces an event whose tags do not match the query that should have selected it,
with no error anywhere.

These two are validity invariants rather than capacity limits because they are
the two values an adapter declares a column width against
(`tag.rs:12-16` says exactly that). A value exceeding them cannot be stored by
any conformant adapter, so there is no store that would accept it and nothing is
gained by deferring the refusal to the write.

#### VT-21 — `MIN_SUPPORTED_EVENT_DATA_LEN` is a floor of 65,536 bytes

There MUST NOT be a `MAX_EVENT_DATA_LEN` constant. Every store MUST accept an
event whose `data` is at least `MIN_SUPPORTED_EVENT_DATA_LEN` = 65,536 bytes. A
store MAY accept more, MUST document its actual limit, and MUST refuse an
oversized payload with `AppendError::ExceedsStoreLimit` rather than truncating.

`[PROVISIONAL — falsified by a named target that cannot honour 64 KiB; the
legacy KV-backed Durable Object with a 128 KiB value cap is the closest and it
clears the floor with room for the envelope]`
`Rule:` `store_accepts_the_guaranteed_minimum_payload` — written at phase 3
against **this clause's number rather than against a constant**, because
`MIN_SUPPORTED_EVENT_DATA_LEN` does not exist until phase 4 introduces it;
`append_reports_exceeded_store_limits`, which VT-25's variant and CF-40's fixture
ceilings made writable at phase 4
`Cases:` E2E-42; and Turnstile's peer D
(`references/scenarios/README.md:1549-1554`), which has no case number
`Rejects:` a ceiling. A single `MAX_EVENT_DATA_LEN` is either a straitjacket on
Postgres — which will happily hold Turnstile's 340 KB seat map — or a lie on the
KV peer, which cannot hold it at all. And it would freeze at 0.1 with everything
else, so the number could never be revised. Also rejects the current state:
`Event::data` unbounded and undeclared, so a 340 KB payload is durable at origin
and structurally unrepresentable at a peer, and the incompatibility is discovered
at ingest, after the write has already committed somewhere else.

A floor is the shape that carries information across a peer set. It tells an
application what it may write and still expect to replicate, and it gives the
sync layer a value to compare a peer's declared limit against before it starts
pushing.

#### VT-22 — `MIN_SUPPORTED_TAGS_PER_EVENT` is a floor of 64

There MUST NOT be a `MAX_TAGS` constant and `Tags` MUST NOT enforce a count.
Every store MUST accept an event carrying at least
`MIN_SUPPORTED_TAGS_PER_EVENT` = 64 tags, MUST document its actual limit, and
MUST refuse beyond it with `AppendError::ExceedsStoreLimit`.

`[PROVISIONAL — falsified by a domain event legitimately carrying more than 64
tags; the richest event in the six scenarios is Wattline's `SessionStarted` at
eight]`
`Rule:` `store_accepts_the_guaranteed_minimum_tag_count`
`Cases:` E2E-40
`Rejects:` a `Tags` constructor that enforces a count, which would make the
capacity limit a type invariant and re-create the decode-time refusal VT-19
forbids. Also rejects an adapter that silently drops tags past its own limit,
which produces exactly the map-shaped-index failure of VT-17 with a different
cause.

Sixty-four is eight times the observed maximum and keeps a multi-row tag insert
for a hundred-event batch inside SQLite's `SQLITE_MAX_VARIABLE_NUMBER` of 32,766
at three parameters per tag.

#### VT-23 — `MIN_SUPPORTED_QUERY_ITEMS` is a floor of 128

There MUST NOT be a `MAX_QUERY_ITEMS` constant and `Query::from_items` MUST NOT
enforce a count. Every store MUST evaluate a query of at least
`MIN_SUPPORTED_QUERY_ITEMS` = 128 items. A store or an ingest policy MAY refuse a
larger one.

`[PROVISIONAL — falsified by a decision model in a real domain that legitimately
needs more than 128 items; the largest in the six scenarios is four]`
`Rule:` `store_evaluates_a_query_at_the_guaranteed_minimum_item_count`
`Cases:` E2E-36, E2E-40
`Rejects:` an adapter that generates one SQL parameter per item and silently
fails past a driver limit, and an adapter that emits one statement per item
without an upper bound on the round trips that implies. It does *not* reject a
policy refusal: E2E-40's hub must be able to decline a peer-authored query it
does not want to execute, and that refusal happens at the ingest policy seam
(§4), above this floor.

The floor is what makes the seam expressible. Without a stated minimum, "the hub
refuses expensive conditions" is indistinguishable from "the hub is broken".

#### VT-24 — `MIN_SUPPORTED_EVENTS_PER_BATCH` is a floor of 128

There MUST NOT be a `MAX_EVENTS_PER_BATCH` constant and no `EventBatch` type MUST
be introduced to carry one. Every store MUST accept an append of at least
`MIN_SUPPORTED_EVENTS_PER_BATCH` = 128 events, MUST document its actual limit,
and MUST refuse beyond it with `AppendError::ExceedsStoreLimit`.

`[PROVISIONAL — falsified by a domain whose smallest indivisible unit of work
exceeds 128 events; Kestrel Cold Chain's largest single conditioned group is
under forty]`
`Rule:` `store_accepts_the_guaranteed_minimum_batch_size`
`Cases:` E2E-35, E2E-36, E2E-39
`Rejects:` an adapter that builds a multi-row `INSERT` with one parameter set per
event and per tag and discovers `SQLITE_MAX_VARIABLE_NUMBER` at write time — that
is, after the caller has already made its decision and taken its side effects.
Also rejects the `EventBatch` newtype as the enforcement point: it puts a
constructor in front of every `append` call site in every application in order to
enforce a bound that is adapter-specific, so the type would either carry the
wrong number or carry no number at all.

Rotor's 1,840-event ingest does not argue against this floor, because ingest
decomposes into independently-conditioned groups (E2E-35) and the batch bound
applies to a group.

**VT-21, VT-22 and VT-24's documentation obligation is discharged at phase 8, and
the markers stay where they are.** Each of those three says a store MUST document
its actual limit; until phase 8 no store in this workspace had one to document,
and a MUST nothing can fail is a MUST nothing has met. `happenstance-sqlite`
states all three as public constants — `MAX_EVENT_DATA_LEN` = 1,048,576 bytes,
`MAX_TAGS_PER_EVENT` = 128 and `MAX_EVENTS_PER_BATCH` = 256
(`crates/happenstance-sqlite/src/event_store.rs:284`, `:291`, `:300`) — enforces
them as `AppendError::ExceedsStoreLimit` rather than by truncating (`:558`,
`:565`, `:571`), and mirrors them onto its fixture so
`append_reports_exceeded_store_limits` reads them rather than a literal. All
three clear their floors with room, VT-23's 128-item floor is evaluated by the
same adapter through chunked statements rather than refused, and each number is
checked from both sides — the limit accepted, one more refused
(`crates/happenstance-sqlite/tests/append.rs:238-286`). **The markers do not
move.** Their falsifiers are about a *named target that cannot honour the floor*,
and one adapter clearing four floors comfortably is not that target; discharging
the documentation half is a different claim from settling the number.

#### VT-25 — A capacity refusal is distinguishable from a store failure

`AppendError` MUST carry a variant `ExceedsStoreLimit { limit: StoreLimit, len:
usize }`, where `StoreLimit` is a `#[non_exhaustive]` enum naming which limit was
exceeded. A store MUST NOT report a capacity refusal through
`AppendError::Store`.

`[FROZEN]`
`Rule:` `append_reports_exceeded_store_limits`, which reads the ceilings CF-40
puts on `Fixture` and skips with a stated reason against a store that has none
`Cases:` E2E-35, E2E-42
`Rejects:` reporting an over-limit append as an adapter-specific error, which is
what every adapter had to do while there was no other variant. The caller
then cannot tell "this event will never be accepted here, park it and tell a
human" from "the disk is full, retry" — and the sync runner, which must make
exactly that distinction to avoid the E2E-42 disappearance, has nothing to
switch on. `AppendError`'s three variants as this clause was written contained no
"refused, park this" and PRESSURE-TEST §5.4 named the gap.

**Discharged at phase 4.** `AppendError::ExceedsStoreLimit { limit, len }` is in
the tree (`error.rs:227-244`), `StoreLimit` is the `#[non_exhaustive]` enum
naming which limit was exceeded (`limits.rs:52-61`), and the variant is carried
through `map_store` (`error.rs:268`). `AppendError` was already
`#[non_exhaustive]` (`error.rs:213`) and its documentation already told callers a
wildcard arm is required (`error.rs:210-211`), so the variant landed additively,
as predicted.

---

### 2.6 `Query`, `QueryItem` and `ReadOptions`

#### VT-26 — A `Query` with zero items is unrepresentable from outside the crate

`Query::Items` MUST NOT be constructible outside `happenstance-core`. The variant
MUST carry `#[non_exhaustive]`.

`[FROZEN]`
`Rule:` compile test `query_items_is_not_constructible_downstream`;
`condition_without_after_rejects_any_match`
`Cases:` E2E-40
`Rejects:` `Query::Items(Vec::new().into_boxed_slice())` from any downstream
crate — the exact state `Query::from_items` rejects with `InvalidQuery::NoItems`
(`query.rs:185-191`). `Query::matches` then returns `false` for everything, so an
`AppendCondition` built from it can never be violated: a conditional append that
is silently unconditional. The crate's headline design claim is false at the
boundary that matters, and it is one line of downstream code away.

Variant-level `#[non_exhaustive]` rather than enum-level is the right instrument
here and the distinction is worth explaining. On the *enum*, it would force every
downstream `match` to carry a wildcard arm, which is a real cost for a two-variant
enum whose variants are the whole of its meaning. On the *variant*, construction
outside the crate is forbidden while matching still works — but **not** in the
tuple spelling this clause first named. Measured on 1.97.1, `Query::Items(..)`
downstream is `error[E0603]`, the tuple variant reported as private: a tuple
pattern resolves through the variant's *constructor*, and `#[non_exhaustive]` is
precisely what makes that constructor crate-private. The struct spellings do not
name the constructor, so `Query::Items { .. }` and `Query::Items { 0: held, .. }`
both compile, and the trailing `..` is the compiler telling the reader that the
payload is the crate's business. The property is unchanged — readable, not
constructible — and only the spelling was wrong; `query.rs`'s own doc was
corrected first (`query.rs:156-168`). `Query::items()` (`query.rs:203-209`)
remains the ordinary read path and needs no change.

#### VT-27 — A `Query` carries no positions, and a decision model composes by reading its fragments separately

`Query` and `QueryItem` MUST NOT carry a `SequencePosition` or any other
store-local value. `ReadOptions::from` and `ReadOptions::to` apply to a whole
read, not per item. An application that needs different bounds for different
fragments of a composed decision model MUST issue one read per fragment.

`[FROZEN]`
`Rule:` compile-level; enforced by the wire tests, which would otherwise have a
position to encode. The behavioural half is checked by the guards of VT-30.
`Cases:` E2E-04, E2E-05, E2E-37, E2E-38
`Rejects:` a per-item `from` inside `Query`. `Query` is the *replicable* half of
an `AppendCondition` and `after` is the *non-replicable* half — that partition is
what makes E2E-37's blunt ingest rule ("refuse any wire condition carrying
`after: Some(_)`") expressible at all, and what makes Kestrel Cold Chain's
epoch-tagging trick work by converting a position boundary into a tag boundary.
Putting N store-local integers inside the query destroys the partition and makes
every wire condition carry N values with no referent at the receiver instead of
one.

This is the section's new finding and it deserves its cost stated plainly rather
than resolved by a field.

`ReadOptions` carries one `from` for the entire read (`query.rs:269-289`) and
`read` applies one `ReadOptions` to the whole `Query` (`store.rs:167-171`). So
Wattline's four-item `StartSession` boundary cannot bound its 828,000-event fleet
item without blinding the three items whose definitional events sit far below any
recent snapshot. Setting `from` at all folds an absent circuit rating, a vanished
connector commissioning and an unknown token validity; setting it to `None` reads
828,000 events. There is no third option and no compile error — the handler
decides on a fold with holes in it (E2E-04).

**The refusal is on the read side and the payment is on the condition side.** The
application issues one read per fragment, which is what E2E-05 already describes
as the workaround, and VT-30 makes that workaround *sound* by giving the
resulting condition one boundary per fragment instead of forcing `min(p₁…p₄)`.
Without VT-30 the workaround is a liveness failure — the quiet token item's stale
boundary becomes the boundary the busy circuit item is checked from, and the
resulting rejection rate is attributed to contention rather than to the collapse.
With it, the composition is exact.

The cost of the refusal, stated: N reads is N round trips, and on a one-shot HTTP
transport with no session that is N HTTP calls for one decision. An adapter MAY
coalesce them; the port does not, and nothing in the contract makes the
coalescing observable. That is a real price paid to keep positions out of the
replicable half, and it is the right one, because a query that carries positions
cannot cross a store boundary at all and a decision model that needs an extra
round trip merely costs latency.

#### VT-28 — `limit(0)` returns nothing

`ReadOptions::limit` MUST be `Option<usize>` and a limit of `0` MUST yield no
events.

`[FROZEN]`
`Rule:` `read_limit_zero_yields_nothing`; `read_limit_truncates`
`Cases:` E2E-11, E2E-13
`Rejects:` the implementation this clause was written against.
`self.limit = NonZeroUsize::new(limit)` turned zero into `None`, documented as
"A `limit` of zero is ignored, since requesting nothing is never what the caller
meant". The premise is true for a literal and false for a computed value: a
paging loop writing `.limit(budget - fetched)` that reaches parity did not read
zero events, it read **the entire log, unbounded, silently**.

Phase 3 could not write the rule and the reason was the type rather than the
effort: `limit` took a `usize` and stored a `NonZeroUsize`, so `limit(0)` was
`None` before any adapter saw it and the input the rule needed was not
expressible through the API. A rule recorded as unwritable, with an owner and a
reason, is not a gap — that is the ownership distinction §7.4 turns on, and it is
why the name was left here rather than dropped until it could be written.

**Discharged at phase 4.** `limit` is `Option<usize>` (`query.rs:286-288`), the
builder stores `Some(0)` verbatim (`query.rs:343-347`),
`zero_limit_means_zero_events` (`query.rs:553-566`) asserts it, and
`read_limit_zero_yields_nothing` is in the suite
(`crates/happenstance-testkit/src/suite.rs:1539`) and in
`for_each_event_store_rule!`
(`crates/happenstance-testkit/src/registry.rs:138`).

This is a deliberate divergence from the DCB reference implementation, which
treats `limit: 0` as unlimited through JavaScript falsiness, and the ADR that
lands it must say so rather than presenting it as a correction of a bug in
precedent. It matches SQL `LIMIT 0` and every paging API in existence. The cost
is eight bytes on a `Copy` struct passed by value. `limit(NonZeroUsize)` was the
type-safe alternative and it loses: it forces every caller with a computed budget
to handle the zero case at the call site, which is precisely the caller who has
just computed zero legitimately.

#### VT-29 — A read can be given an inclusive upper bound

`ReadOptions` MUST carry a `to: Option<SequencePosition>`, an inclusive upper
bound in position order. Under `backwards`, `from` remains the starting (higher)
bound and `to` the stopping (lower) bound.

`[FROZEN]`
`Rule:` `read_to_is_inclusive`, `read_from_and_to_bound_a_closed_window`,
`read_to_under_backwards_bounds_the_older_end`
`Cases:` E2E-11
`Rejects:` the workaround, which is `from(H).backwards()`, buffer everything,
reverse in memory. That genuinely is a pinned snapshot and it works at Kestrel
Motor's 211-event erasure; it is fatal at the 53-million-event regulatory
backfill, because the whole window must be materialised before the first event is
yielded. A backfill worker given the closed window [1, *H*] while a tail worker
owns (*H*, ∞) is the shape every backfill-beside-tail deployment wants, and
`limit` cannot stand in for it because `event.rs:215-217` forbids treating
position arithmetic as a count.

**Discharged at phase 4.** The field is on the struct
(`query.rs:269-289`), the builder is `const` and inclusive
(`query.rs:318-330`), and `to_is_recorded_and_independent_of_from`
(`query.rs:568-578`) asserts that direction reverses what the two fields bound
rather than reassigning them. The MUST stands over every adapter still to come.

**This clause is not a fix for E2E-04.** An upper bound does not let one item of
a composed query start above a snapshot while the others start at the beginning.
It is a separate and real capability, and it is stated separately so that
shipping it is not mistaken for closing the per-item bound question, which VT-27
closes by refusal.

ES-16 is the same decision seen from the port side and carries the read
semantics. It was drafted as a deferral and is frozen here for the reason its own
deferral text gives: `ReadOptions` is `#[non_exhaustive]` and passed by value, so
`to` is not a trait signature change, but it *is* a new obligation on every
adapter, and an adapter that ignores an unknown field silently returns too much.
The field is therefore cheap now and expensive after the first adapter ships, and
no adapter has shipped.

#### VT-34 — `ReadOptions` carries exactly one lower bound and it is inclusive

`ReadOptions` MUST carry exactly one lower bound, `from`, and it MUST be
inclusive. An exclusive `after` MUST NOT be added beside it.

`[FROZEN]` — deliberately with **no falsifier**, because the argument is complete
now and nothing an adapter or a runner could later measure bears on it. Reopening
it takes a new ADR; the thing that would justify one is a phase 6/7 projection
runner that cannot express its resume through `checkpoint.next()`. That is a
trigger for an ADR and **not** a marker falsifier, and the two must not be
confused — writing it into the marker would make a frozen clause carry a
provisional's furniture.
`Rule:` compile-level, plus the existing `read_from_is_inclusive`.
`Rejects:` adding `ReadOptions::after` beside `from`, giving one
`#[non_exhaustive]` options struct two optional lower bounds with opposite
inclusivity and no precedence rule. An adapter must then invent a verdict for
`from(5).after(7)`, and two conformant adapters will invent different ones — the
shape ES-16's own `Rejects:` already predicts: "an adapter that accepts
`ReadOptions` by value, matches on the fields it knows and ignores the rest".

Three reasons, and the second and third are cross-references rather than new
arguments. ES-26 settles that `AppendCondition`'s `after` is exclusive *because*
it means "everything I did not see", while `ReadOptions::from` is inclusive
because it means "start where I stopped"; putting both conventions on one struct
destroys the distinction that clause exists to draw. And the idiom `after` would
replace is itself frozen and sound: VT-13 names `checkpoint.next()` as the
documented resume idiom and fixes it to signal overflow, and ES-9 makes it sound
over gaps, because `from` is a range predicate rather than a seek — so resuming
at an unoccupied position yields the higher neighbour rather than an error.
`next()` is not a caller doing arithmetic on an opaque key; it is the one named
method on the position type whose entire purpose is this.

**This clause is frozen with no falsifier by design, and the cost is worth
stating.** Nothing in the gate fails if a future contributor adds
`ReadOptions::after`: the absence of a field is not observable through the port
at run time. The only thing standing between the codebase and that field is this
clause. ES-30 accepted the same position for `count()`, and it is the right one —
but a reader who expects the gate to defend every frozen sentence should know
this is one it cannot.

**ID note.** This clause was drafted as VT-32. ADR-0015's const-constructor
clause took that number in the same slice, so it moved rather than colliding.

#### VT-30 — An `AppendCondition` is one or more guards, each with its own boundary

An `AppendCondition` MUST hold a non-empty sequence of guards, each a pair of a
`Query` and an `Option<SequencePosition>` boundary. The append MUST be rejected
if any guard is violated. `AppendCondition::new(query)` MUST continue to produce
a single unbounded guard, and `after`/`after_opt` MUST continue to apply the
given boundary to every guard.

`[PROVISIONAL — falsified if no adapter can push a multi-guard condition into a
single statement without one self-join per guard, or if the `min()` collapse
measures as immaterial on a Wattline-shaped workload; the Postgres adapter and
the benchmark harness are the two instruments and both are unbuilt]`
`Rule:` `condition_guards_carry_independent_boundaries`,
`condition_with_one_guard_behaves_as_today`; the existing
`condition_after_*` family becomes the single-guard case
`Cases:` E2E-04, E2E-05, E2E-06
`Rejects:` the application-side fix E2E-05 names — four reads, four last-seen
positions, and a condition on `min(p₁…p₄)`. That is sound and it re-admits every
event above the minimum for **all four** items, so the quiet fragment's stale
boundary governs the busy one and the deployment reads the resulting rejection
rate as physics.

**Before commit `1b2a565`**, `AppendCondition` was `{ fail_if_events_match:
Query, after: Option<SequencePosition> }` with public fields on a
`#[non_exhaustive]` struct. It is now `{ guards: Box<[Guard]> }`
(`append.rs:102-119`) with `pub struct Guard { pub query: Query, pub after:
Option<SequencePosition> }` (`:130-141`); the clause landed at phase 4 and it
stays `[PROVISIONAL]` until its two instruments are built. The
`Guard` type MUST itself be `#[non_exhaustive]` with public fields, for the same
reason `AppendCondition` is: E2E-37's ingest rule must *read* `after` on a
peer-supplied condition without being able to construct one literally, and
readable-but-not-literal-constructible is precisely what enables that refusal.

`is_violated_by` (`append.rs:225-234`), delegating to `Guard::is_violated_by` at
`:239-253`, becomes a fold over the guards; the
single-guard path is byte-for-byte the current behaviour, which is why every
existing condition rule survives unchanged. Adapters pushing this into SQL must
generate `(item AND position > p) OR …` with explicit parentheses — the textbook
precedence bug that the scenarios' S7 already flags for the single-boundary case
becomes N times more likely here, and the conformance rule above is what catches
it.

What this clause does **not** do is change what a batch's own events are checked
against. A batch's events MUST NOT be evaluated against its own condition
(E2E-06); `MemoryEventStore` already behaves that way by accident of ordering
(`memory.rs:372-388`) and §3 owns turning that accident into a rule.

#### VT-31 — The query algebra is fixed and adapters may not change the match set

For any queries *a* and *b* built from items, the match set of a query whose
items are the concatenation of *a*'s and *b*'s items MUST equal the union of
their match sets. Any query unioned with `Query::All` MUST match everything. An
adapter MAY reorder or deduplicate items, and MUST NOT do so in a way that
changes the match set.

`[FROZEN]`
`Rule:` `query_items_are_or`,
`query_item_order_does_not_change_the_result_set`,
`query_union_is_item_concatenation`,
`duplicate_items_do_not_duplicate_events` — the same four ES-15 names, because
the algebra is one property stated from the value side here and from the port
side there
`Cases:` E2E-32
`Rejects:` an adapter that deduplicates or reorders a query's items as an
optimisation in a way that is not match-set preserving — natural to reach for,
because `QueryItem::new` already sorts and deduplicates *types*
(`query.rs:62-67`), so extending the idea to items looks like the same move. Such
an adapter passes every existing rule and breaks the fan-out projection runner,
which reads one union query and re-filters each projection's stream locally with
`Query::matches` (`query.rs:216-226`) and is correct only if the algebra holds.

`Query::matches` is `pub` precisely so the local re-filter is available. Nothing
in the crate states the algebra it depends on and no rule pins it, which is the
gap this clause closes.

---

### 2.7 The wire format

#### WF-1 — The format is private, and DCB interoperability is deferred

The `serde` encoding of happenstance's types is private to happenstance. It MUST
NOT be described as, documented as, or constrained by any other DCB
implementation's encoding. Compatibility with the DCB reference implementation's
published shape is deferred.

`[DEFERRED — settled by a bridge exercised against the DCB reference
implementation's encoding, in a `happenstance-dcb-interop` crate with its own
ADR, owned by the phase that first needs to read another implementation's log.
No phase currently needs it. The DCB specification and its reference TypeScript
library publish **no** wire format: `EventStore.ts` contains no serialisation
code, and the specification's JSON snippets are labelled a "potential JSON
representation" beside an explicit disclaimer that "implementations are not
required to use the same terms or function/field names". There is nothing to
build a bridge against, which makes the deferral stronger rather than weaker
(ADR-0016 §1, reading of 2026-08-05).]`
`Rule:` `wire::query_all_is_unambiguous` and `wire::empty_object_is_not_a_condition`
are the two tests that fail if this clause is abandoned in practice; the scope
half is a review obligation, and the clause says so rather than pretending
otherwise. It is a clause rather than prose because every `WF-n` below depends on
it and because silence here is how the previous documents left the format
"neither private nor interoperable" (PRESSURE-TEST §5.4).
`Cases:` E2E-33, E2E-42 — the two cases that move bytes between instances and
would be the first to acquire a foreign-implementation expectation
`Rejects:` the change this clause exists to forbid — re-encoding `Query::All` as
`[]`, or restoring the `skip_serializing_if` attributes, on the grounds of
matching the DCB reference implementation's published shape. That is the concrete
form the pressure to interoperate takes, it arrives as a small compatibility
patch rather than as a design decision, and it reintroduces WF-2's positional
desynchronisation and WF-3's match-everything-by-accident in one commit.

Two divergences from the reference are known and are not defects under this
clause. Both halves of them were recorded backwards. **happenstance** emitted a
bare sequence — `Query::Items` went through `serialize_some` until ADR-0016 §7,
and now goes through `QueryWire::Items` (`query.rs:415-423`), the mirror whose
own doc comment records the encoding it replaced (`query.rs:396-403`) — while
**the reference** is the `{items: […]}` side: its `Query` is `{ items:
QueryItem[]; matchesEvent(…); merge(…) }` and `queryAll()` returns `{items: []}`.
happenstance did not fail to *parse* `[]` before ADR-0016 §7 either: it parsed to
an empty item list and rejected it semantically through `Query::from_items`.
Under the externally tagged encoding `[]` is refused by the representation
itself, which strengthens the divergence — happenstance spells match-all as a
value that cannot syntactically collide with a filtered query, the reference as
`{items: []}`, structurally identical to an illegal empty filtered query. Three
further facts a bridge needs: the reference's `QueryItem` tags are bare opaque
strings, not key/value `Tags`; its `Query` carries methods and is not
serialisable without a step somebody must define; and its `AppendCondition` is a
single `{failIfEventsMatch, after?}` guard rather than a sequence. Both are
recorded so the future bridge starts from a list rather than from a discovery.

#### WF-2 — Every field of every wire struct is always present

No wire struct may use `skip_serializing_if`, and no attribute may make the
number of serialised fields depend on a value. Every field MUST be written, in
declaration order, on every serialisation. **This obligation binds the
encoder.** A `Deserialize` that accepts an absent `Option` field does not violate
it: serde's `missing_field` succeeds for any type whose `Deserialize` calls
`deserialize_option`, with or without `#[serde(default)]` (measured, ADR-0016
§5), so the read side is asymmetric at `Event::metadata` and `Guard::after`. The
asymmetry is deliberate and fails closed — a missing `after` decodes to `None`,
which checks the whole log. `#[serde(default)]` is nevertheless deleted
everywhere, on ADR-0016 §4's own preference.

`[FROZEN]`
`Rule:` `wire::round_trips_in_postcard` over every envelope shape, including the
all-defaults shape. After ADR-0016 §3 no type in `happenstance-core` can fail
these rules, because every wire struct writes every field.
**`wire::round_trips_in_postcard` is therefore paired with a negative control** —
a mirror struct carrying `#[serde(skip_serializing_if = "…")]`, asserted to fail
the same round trip — and that control is the whole of its discriminating power
below maximum size: deleting it is deleting the rule.
`Cases:` E2E-33, E2E-35, E2E-42
`Rejects:` the five `skip_serializing_if` attributes ADR-0016 §3 deleted — two on
`EventWire`'s `tags` and `metadata` (`event.rs:739-750`), two on `QueryItemWire`'s
`types` and `tags` (`query.rs:370-375`) and one on `GuardWire`'s `after`
(`append.rs:286-291`), each anchored on the struct that carried it rather than on
the line it sat on, because a line number for deleted code can only ever be
wrong. The attribute shortens the field count passed to `serialize_struct`; a
non-self-describing format feeds the deserializer exactly `FIELDS.len()` values
positionally, so a skipped field desynchronises the stream. Serialisation
*succeeds* and produces plausible-looking bytes. `Event::new("A", data)` with no
tags — what every doctest builds — is a failing input, as are
`QueryItem::of_types([…])`,
`QueryItem::tagged(…)` and `AppendCondition::new(q)`. Measured: a **lone**
`Event` fails to round-trip in postcard for three of its four shapes, erroring
with `Hit the end of buffer` at 7, 18 and 11 bytes; only the all-fields-present
shape (22 bytes) survives. The **silent** wrong value needs a neighbouring value
in the same buffer, and the neighbour matters: of nine tried after the tags-only
`Event`, three decode to a wrong `Event` with no error, two decode to a value
equal to the original while consuming the neighbour, and four error. The rule
needs both a solo shape and a framed pair.

The cost is **+26 bytes in JSON** for the two restored fields — `,"tags":[]` is
10 and `,"metadata":null` is 16 — which for a bare `Event` with a four-byte
single-digit payload is 35 → 61, for the same shape carrying `11 22 33 44` is
39 → 65, and for a zero-length payload 28 → 54; and **one byte per absent field
in postcard** — measured, the tagged single-field case is 18 → 19 and the
all-defaults two-field case 7 → 9. That is still the correct direction to trade.

#### WF-3 — `Query::All` has an unambiguous encoding

`Query` MUST be encoded as an externally tagged enum with two variants, `All`
(unit) and `Items` (sequence). `Query::All` MUST NOT be encoded as `null`, as an
absent field, or as an empty sequence. `Option<Query>` MUST round-trip, with
`Some(Query::All)` distinguishable from `None`.

`[FROZEN]`
`Rule:` `wire::query_all_is_unambiguous`, `wire::option_query_round_trips`
`Cases:` E2E-40
`Rejects:` the implementation this clause replaced. `Serialize` mapped `All` to
`serialize_none` and `Items` to `serialize_some` — the encoding the mirror that
took its place now records as the one that lost (`query.rs:396-403`) — so
`Query::All` became JSON `null`, `Some(Query::All)` and `None` were
indistinguishable, and serde's `missing_field` succeeds for `Option`-shaped
types — which meant a *missing* field decoded to the broadest condition in the
protocol. A peer that emitted `null` by accident got match-everything. Inside an
`AppendCondition` that fails closed, which is a spurious rejection; inside an
ingest policy that is E2E-40's unbounded scan inside a single-threaded actor
with a fixed CPU ceiling. The tense is past because the encoding is gone; the
clause stays because restoring it is a small commit.

This reverses a documented decision, not an oversight: the comment ADR-0016 §7
deleted stated the intent — "`None` is the match-all query; `Some(items)` is a
filtered one" — and the ADR that lands a reversal must say it is reversing a
choice. That is why the mirror which replaced it spends ten lines on the
encoding it beat (`query.rs:396-403`) rather than simply describing itself. The
`Option`-shaped encoding was chosen for compactness in JSON and it is exactly
the compactness that makes the two values indistinguishable.

An externally tagged enum is chosen over an internally tagged one because
internally tagged representations require a self-describing format and postcard
is not one. In postcard the encoding is a varint discriminant followed by the
payload; in JSON it is `"All"` or `{"Items":[…]}`.

#### WF-4 — An `AppendCondition` encodes its guards explicitly, and an empty object is not one

`AppendCondition` MUST encode as a struct with a single `guards` field holding a
non-empty sequence. Each guard MUST **serialise** both `query` and `after`, both
always written, with `after` an explicit `Option`; as in WF-2 the presence
obligation binds the encoder, and a decoder that accepts an absent `after` fails
closed to `None`. `serde_json::from_str::<AppendCondition>("{}")` MUST fail —
satisfied since ADR-0012's guard sequence landed, and retained as a floor rather
than a live obligation — **and so MUST `{"guards":[{}]}` and
`{"guards":[{"query":null}]}`, which are the inputs the rule actually tests.**
The field is named `query` because VT-30 moved the boundary into `Guard { query,
after }` (ADR-0012 §9), and this clause's earlier `fail_if_events_match` named a
field that move deleted; the MUST is about presence (E2E-37) and not about
spelling. **VT-30 is itself `[PROVISIONAL]` (`:1781-1784`) with both instruments
unbuilt, so this noun tracks a provisional clause's landed implementation:
falsifying VT-30 withdraws the guard sequence and returns this clause to a
single-guard spelling. It does not reopen the `after`-presence obligation, which
is E2E-37's and independent of the guard count.**

`[FROZEN]`
`Rule:` `wire::empty_object_is_not_a_condition`,
`wire::condition_after_is_visible_to_an_ingest_policy`
`Cases:` E2E-35, E2E-37, E2E-40
`Rejects:` the input that still decoded to match-everything after ADR-0012's
guard sequence landed. Measured before ADR-0016, `"{}"`, `"[]"` and
`{"guards":[]}` all already errored. What was **not** closed is `{"guards":[{}]}`,
which returned `Ok(AppendCondition { guards: [Guard { query: All, after: None }]
})`, as did `{"guards":[{"query":null}]}` — and
`AppendCondition::new(Query::all())` serialised as exactly that. It succeeded
only because `Query`'s `Deserialize` was
`Option`-shaped; WF-3 gave `Query` an explicit tag and both are now hard errors
(measured: ``missing field `query` `` and `expected value`), which is why the two
changes landed in one commit. `GuardWire` (`append.rs:286-291`) carries the
standing prohibition on `#[serde(default)]` in its own doc comment
(`append.rs:264-285`), because "add a default so `{}` parses" is the patch that
restores this and it will look like a kindness.

`after` is always present because E2E-37's ingest rule depends on seeing it. A
store-local position on the wire has no referent at the receiver —
`is_violated_by` compares raw positions (`append.rs:225-234`, delegating to
`Guard::is_violated_by` at `:239-253`, where `position <= after` at `:250` is the
comparison), so `after: 288455` in hub numbering names an unrelated recent event
and the check passes vacuously,
which is worse than being rejected because it looks like enforcement. An ingest
policy can only refuse what it can see.

The explicit `guards` sequence is also what makes E2E-35's decomposition visible.
A peer's push of thirty-nine events across seven independently-conditioned groups
is seven appends, not one; a flat event list makes that emergent, makes ingest
cost look like O(events) when it is O(distinct conditions), and turns a partial
ingest into something discovered in production.

#### WF-5 — `SequencedEvent` carries identity and time on the wire

The `SequencedEvent` wire form MUST carry `position`, `id`, `recorded_at` and
`event`, all always present.

`[FROZEN]`
`Rule:` `wire::sequenced_event_round_trips`
`Cases:` E2E-33, E2E-34, E2E-41, E2E-43
`Rejects:` a wire form that omits `id` on the grounds that the receiver can
recompute it. It cannot: the origin is not derivable from the transport, and a
forwarded event may have travelled through an intermediary (E2E-42's A—B—C
topology), so the identity must travel with the value or transitive convergence
is impossible.

#### WF-6 — `EventId` and `StoreId` encode differently in human-readable and binary formats

`StoreId` MUST encode as 32 lowercase hexadecimal digits without separators in
human-readable formats and as sixteen raw bytes otherwise, selected with
`Serializer::is_human_readable` / `Deserializer::is_human_readable`. `EventId`
MUST encode as a struct of **`store`** and `position`. **The field is `store`
because that is `EventId`'s own field and accessor (`identity.rs:97-100`,
`:110-113`), named as such by VT-5's body at `:770-772`.** The binary half is
already satisfied at HEAD: `StoreId` serialises as `[u8; 16]` in the
non-human-readable arm (`identity.rs:252-263`) and postcard renders sixteen raw
bytes with no length prefix. **The wrong implementation both rules exist to
reject is an inverted `is_human_readable` branch, invisible to every round-trip
rule** — measured, an
inverted branch round-trips cleanly in both formats — so
`wire::store_id_encodes_as_hex_in_json` asserts the JSON value is the
32-character string and `wire::store_id_encodes_as_bytes_in_postcard` asserts
sixteen raw bytes and no ASCII hex.

`[FROZEN]`
`Rule:` `wire::store_id_encodes_as_hex_in_json`,
`wire::store_id_encodes_as_bytes_in_postcard`
`Cases:` E2E-34, E2E-42
`Rejects:` encoding a `StoreId` as a byte sequence in JSON, which produces a
**sixteen**-element array of integers — `StoreId` is `[u8; 16]`
(`identity.rs:44`), one element per byte — that no operator can read. Measured:
59 bytes against 34 of quoted hex, a factor of **1.7353**; the arithmetic
maximum, an all-`0xff` id, is 65 bytes = 1.9118×, and an all-`0x01` id is 33
bytes = 0.9706×, i.e. **cheaper** than hex. "Four times the bytes" is not
reachable, and the argument that carries this clause is illegibility rather than
size. Also rejects encoding it in UUID form with hyphens and version
nibbles: it is not a UUID, and formatting it as one invites the next reader to
parse a version field that carries no meaning and to assume a time ordering that
does not exist.

`is_human_readable` is serde's standard mechanism for exactly this — the `uuid`
crate uses it — and the consequence must be stated rather than discovered: the
same value has two encodings, and a format's human-readability is therefore part
of the format's identity. A peer negotiating a transport negotiates which of the
two it is speaking.

#### WF-7 — JSON and postcard are supported; everything else is expected to work

The wire format MUST be exercised by the round-trip tests in `serde_json` and
`postcard`. `bincode` and MessagePack are expected to work and are not covered.

`[FROZEN]`
`Rule:` `wire::round_trips_in_json`, `wire::round_trips_in_postcard`, both over
every envelope shape including all-defaults and maximum-size values. After
ADR-0016 §3 no type in `happenstance-core` can fail these rules, because every
wire struct writes every field. **`wire::round_trips_in_postcard` is therefore
paired with a negative control** — a mirror struct carrying
`#[serde(skip_serializing_if = "…")]`, asserted to fail the same round trip —
and that control is the whole of its discriminating power below maximum size:
deleting it is deleting the rule. **`wire::round_trips_in_json` takes no such
control, and must not be given one:** the same mirror round-trips cleanly in
serde_json — measured, all four `Event` shapes do — which is the very fact this
clause exists to state. Its own job is the **maximum-size** half, shared with
WF-9's decode rule: an `Event` whose `data` is `MIN_SUPPORTED_EVENT_DATA_LEN`
(65,536 bytes, `limits.rs:22`), which is where a truncating payload encoder or a
`Deserialize` that grew a length check first shows.
`Cases:` E2E-33, E2E-42
`Rejects:` a test matrix of one format. One self-describing format and one
non-self-describing format is the minimum that discriminates: D1's five
attributes serialise and deserialise perfectly in JSON and in postcard are
**undecodable for three of an `Event`'s four shapes** — `Hit the end of buffer`
at 7, 18 and 11 bytes — and silently decode to a **wrong** value when a
neighbour follows them in the buffer (measured). A JSON-only matrix certifies a
broken format either way; the correction strengthens the clause, because all
four shapes round-trip in `serde_json`. A third and fourth format add
maintenance and no information, because every failure mode
either belongs to the self-describing class or to the positional class.

Postcard specifically, rather than bincode, because it is `no_std`-friendly and
is the format a constrained replication peer would actually reach for.

#### WF-8 — The envelope is versioned, once per message, and an unknown version is refused whole

Every replication message MUST carry a `format_version`, and a receiver MUST read
and check it **before any part of the message is decoded**. In a positional
format the version MUST be the first field, so that `take_from_bytes::<u16>` on
the front of the buffer yields it; in a self-describing format the format itself
imposes no order — measured, a derived `Deserialize` accepts
`{"message":null,"format_version":999}` and returns `Ok` — so the obligation
there is discharged by an explicit check in a hand-written `Deserialize` and not
by position. **That check is what gives key order its one meaning there, and it
is exactly one:** a document whose `message` arrives before an accepted
`format_version` MUST be refused, because a version read second cannot be checked
before the message decodes, and buffering the message into a self-describing
intermediate to defer the decision is itself the partial decode this clause
forbids (`crates/happenstance-sync/src/wire.rs:268-296`; found during
implementation, ADR-0016 §11). The version MUST be bumped whenever the shape of
any wire type changes. A receiver that does not implement a version MUST refuse
the entire message with a distinguishable error and MUST NOT attempt a partial
decode.

`[FROZEN]`
`Rule:` `wire::rejects_an_unknown_format_version`,
`wire::version_is_readable_before_the_message`, replacing the unwritten
wire::version_is_the_first_field — unbackticked deliberately, because a `Rule:`
field may backtick rule names and nothing else (ADR-0016 §15). **The replacement
is a witness test, not a byte-layout test**: measured, a derived `Deserialize`
with a post-hoc version check returns the same `Err` in both formats *and*
produces byte-identical postcard framing — `[01 07]` at version 1, `[80 03 07]`
at 384, same `take_from_bytes::<u16>` value and `[07]` remainder — so neither the
error nor the front-of-buffer remainder distinguishes it from the hand-written
impl this clause requires. The rule decodes an `Envelope<Witness>` at an unknown
version, where `Witness`'s `Deserialize` records that it ran, and asserts the
count is **zero** in both formats; the derive records **one**. The byte-position
spelling was rejected separately, because a `u16` varint is one, two or three
bytes.
`Cases:` E2E-35, E2E-42
`Rejects:` per-type versioning, which costs bytes on every event in the log and
still cannot express a change to the *relationship* between two types — for
instance, the move of `after` from `AppendCondition` into `Guard` (VT-30), which
changes no single type's shape in isolation. Also rejects, **in a positional
format**, a version field placed anywhere but first: a decoder that has already
consumed two fields incorrectly cannot recover to read a version, so a trailing
version is a version nobody can read when they need it. In a self-describing
format the equivalent refusal is a decoder that reads the message before checking
the version — key order is not observable there, so position is not the property
and a pre-message check is.

The version lives on the sync message envelope, in `happenstance-sync`, not on
`Event` or `Query`. The value types are not independently versioned because they
are never independently transmitted.

#### WF-9 — A bound change is not a wire change

Changing a capacity limit MUST NOT change the encoding and MUST NOT require a
`format_version` bump. A peer whose limit is lower than a received value's size
MUST decode the value successfully, MUST refuse to append it with
`AppendError::ExceedsStoreLimit`, and MUST make the refusal reportable to the
sync runner.

`[FROZEN]`
`Rule:` `wire::decode_accepts_an_over_capacity_value`;
`append_reports_exceeded_store_limits`
`Cases:` E2E-42
`Rejects:` the assumption in PRESSURE-TEST §5.4 that each new bound is "a
wire-compatibility break with no quarantine path". It is a break only if the
bound is enforced in `Deserialize`, which VT-19 forbids for exactly this reason.
It also rejects the alternative repair — bumping the format version whenever a
limit changes — which would make every peer in a heterogeneous deployment
unreachable from every other the moment one of them raised a limit. The rule
decodes an `Event` whose `data` exceeds `MIN_SUPPORTED_EVENT_DATA_LEN` (65,536
bytes, `limits.rs:22`) — a floor adapters must support, not a limit the contract
crate applies — and asserts `Ok`. The wrong implementation it rejects is a
`Deserialize` that grew a length check, a live hazard precisely because WF-10
asks the same impls to re-run their constructor's invariants; that
implementation is written into `crates/happenstance-core/tests/wire.rs` as a
negative control.

Raising a floor is always safe: a peer that accepts more accepts everything it
accepted before. Lowering one is a semver-major change to the contract and an
operational break in the deployment, and the deployment-level mechanism is the
refuse-and-park path above, not a version negotiation.

#### WF-10 — Deserialisation re-runs every validity invariant and no capacity limit

The `Deserialize` implementation of every value type MUST re-run its
constructor's validity invariants, so that no wire value can become an in-memory
value the constructor would have rejected. It MUST NOT enforce any capacity
limit.

`[FROZEN]`
`Rule:` `wire::decode_rejects_a_non_canonical_tag_set`,
`wire::decode_rejects_an_unconstrained_query_item`,
`wire::decode_rejects_a_zero_item_query`,
`wire::decode_accepts_an_over_capacity_value`
`Cases:` E2E-40
`Rejects:` a `Deserialize` derived straight onto the private fields, which is the
obvious implementation and which would let a peer construct every value the
constructors exist to prevent — an unsorted `Tags`, an unconstrained `QueryItem`,
a zero-item `Query` — directly in the receiver's memory. The current impls
already do this correctly (`tag.rs:538-545` re-canonicalises, `query.rs:387-392`
re-validates through `QueryItem::new`, `query.rs:425-440` routes `Query` through
`from_items`, and `append.rs:315-343` rejects an empty guard sequence) and E2E-40
credits them: "the envelope defends its own invariants; only cost is unguarded".
This clause is what stops that being an accident.

One consequence retires a concern recorded in PRESSURE-TEST §6: re-canonicalising
`Tags` on the way in means a foreign peer's hash over its own tag order will not
agree with the receiver's. That mattered only under a content-hash identity,
which VT-2 and VT-5 reject on independent grounds. Under `(StoreId, position)`
identity the re-canonicalisation costs nothing.

#### WF-11 — Payloads encode as base64 in human-readable formats

`Event::data` and `Event::metadata` MUST encode as standard-alphabet base64 in
human-readable formats and as raw byte strings otherwise.

`[PROVISIONAL — falsified if a peer must forward a payload too large to buffer,
which would make this MUST unsatisfiable on that peer and force the clause to be
re-scoped to formats rather than to peers. **This is a property of serde's data
model rather than of base64** — read from serde's `Serializer` API rather than
measured: `serialize_str` takes a `&str`, so **any** human-readable payload
encoding, hex included, must materialise the whole payload. It therefore
falsifies human-readable payload encoding as a category, and its instrument is
the Workers peer under a memory limit at **phase 9**. The `no_std` half of the
earlier falsifier is **closed**: measured at phase 5, a `#![no_std]` + `alloc`
crate depending on `base64` with `default-features = false, features = ["alloc"]`
checks clean for `wasm32-unknown-unknown` (ADR-0016 §10)]`
`Rule:` `wire::payload_is_base64_in_json`, `wire::payload_is_raw_in_postcard`
`Cases:` E2E-33
`Rejects:` the default. `bytes::Bytes` serialises through `serialize_bytes`, and
`serde_json` renders that as an array of decimal integers — so Turnstile's seat
map (`references/scenarios/README.md:1367`), taken as 340 KiB = 348,160 bytes, becomes
**1,243,464 bytes** of JSON that no human can read — 1.243 MB in 10⁶ units, 1.186
MiB in 2²⁰, and 3.5715× the raw payload. Base64 is 464,218 bytes (453.34 KiB,
1.3333×) and 2.6786× smaller than the array of integers; lowercase hex is 696,322
bytes (680.00 KiB, 2.0000×); postcard is 348,163 bytes (1.0000×). Measured; the
programs are in `experiments/wire-format/`. The format that exists for
diagnostics should be diagnosable.

The dependency is confined to the optional `serde` feature and is one small
crate. That is a real addition to a crate whose dependency graph is deliberately
tiny (ADR-0003), which is why the clause is provisional rather than frozen. The
cost is measured and it is the smallest a dependency has: `base64 0.22.1` is
already in this workspace's graph, pulled by `sqlx-core` and `sqlx-postgres`, and
has no transitive dependencies of its own — zero new crates for the workspace,
exactly one leaf crate for a consumer taking the `serde` feature without sqlx.
`base64` also appears in no public signature, so it does not touch the semver
surface ADR-0003 protects. The feature line becomes `serde = ["dep:serde",
"dep:base64", "base64/alloc", "bytes/serde", "serde/alloc"]`. Both named rules
exist to reject an **inverted `is_human_readable` branch**, invisible to WF-7's
rules: measured, `[de ad be ef]` must render as the JSON string `"3q2+7w=="` and
as four raw bytes in postcard, and an inverted implementation renders
`[222,173,190,239]` and the ASCII of `"3q2+7w=="` respectively while passing both
round trips.

#### WF-12 — `ReadOptions` is not part of the wire format

`ReadOptions` MUST NOT implement `Serialize` or `Deserialize`.

`[FROZEN]`
`Rule:` `read_options_is_not_serialisable`, a **const-evaluation assertion in
`crates/happenstance-core/tests/wire.rs`**, and **not a compile test** — that
phrase is kept deliberately, because `spec_trace`'s `elsewhere` trigger reads it
and a `const _` is not a `#[test]` any rule resolver could find. A `compile_fail`
doctest passes whenever the snippet fails to compile for any reason: measured, of
four spellings only the honest one detected that `ReadOptions` is serialisable at
HEAD, while a type-name typo, a misspelt trait and a wrong crate path all
reported green — and `RUNBOOK.md:3126-3137` already recorded that
`compile_fail,E0080` is "the same check wearing a claim". The assertion is
`const _: () = assert!(!Detect::<ReadOptions>::IS_SERIALIZE, …)`, where an
inherent `impl<T: Serialize> Detect<T> { const IS_SERIALIZE: bool = true; }`
shadows a defaulted trait constant of `false`, because inherent associated items
win over trait ones; a misspelt type is then `error[E0425]`, a build failure
rather than a green test. The `Deserialize` half needs
`impl<T: serde::de::DeserializeOwned> Detect<T>`, not `impl<T:
serde::Deserialize>` — the latter is `error[E0106]`, since `Deserialize<'de>` is
lifetime-parameterised. What enforces the whole is that the test target must
build, which `xtask/src/proof.rs` already does in order to list it.
`Cases:` E2E-58, written at phase 5 to close this clause's own hole — §7.5
carried it as the section's one genuine defect until then. The clause removes a
surface rather than adding one, and it stays a clause because the impls existed
when it was written and something had to say to delete them. No other case is
named here on purpose: a `Cases:` line is parsed for every `E2E-nn` it contains,
so a comparison drawn in it would be read as a claim.
`Rejects:` a replication protocol that ships a `ReadOptions` as its "send me
more" request. That is the obvious use and it is wrong twice over: `from` is a
store-local position with no meaning at the sender, and `backwards`/`limit` are
traversal options for a local reader that a peer has no business setting. A sync
request message carries its own fields, defined in `happenstance-sync`, whose
meaning is negotiated between the two peers.

The impls this clause deleted were on no wire path that existed and were
maintenance the crate does not owe. What stands in their place is a comment
saying so where the next author will look for them (`query.rs:442-448`), because
a deletion nothing explains is a deletion somebody undoes. **The claim that their
whole-struct `#[serde(default)]` was a fifth hazard of WF-2's class is
withdrawn**, measured:
it is byte-for-byte inert in postcard on both encode and decode, because postcard
treats running out of bytes as a hard parse error rather than an end-of-sequence
signal, so the derive's default-fallback path never executes. Its only observable
effect is in a self-describing format, where it lets the decoder accept a
document the encoder cannot produce — a reason to delete it everywhere (ADR-0016
§4), but **not** the positional class, and `ReadOptionsWire` carried no
`skip_serializing_if` at all.

---

### 2.8 Where the remaining bodies are

Four things this section touches are decided elsewhere and are named here so the
seams are visible rather than implied.

**How a store evaluates a multi-guard condition, and whether the items of one
`Query` share one snapshot** (E2E-03) belong to §3, which owns `EventStore`.
VT-30 fixes the value; §3 fixes what the store does with it.

**What ingest does with a duplicate `EventId`, how a compensation is made atomic
with the losing event, and what a peer may refuse** belong to §5 — SY-11, SY-2
and SY-33 respectively. VT-5 through VT-10 supply the identity that makes all
three expressible; none of them specifies the operation.

**Whether a projection is handed an `EventId` rather than a `SequencePosition`**
is SY-21, in §5, because it is a convergence property rather than a projection-store
one; §4 owns the port the projection writes through. VT-9's prohibition on
ordering by time and VT-5's peer-independent order are the inputs; the runner's
signature is not this section's.

**Whether a store can say what history it does not hold** (E2E-46 through E2E-49)
is not settled here and is not settled by silence. `SequencePosition` gaps are
permitted (VT-11) and a purged log is therefore indistinguishable from a young
one at every value in this section. §3 takes the written refusal — ES-37 puts
deletion out of scope for the port and ES-38 says what a store that has been
deleted from is permitted to look like — and ES-39 defers the remaining half, the
retained-history primitive, against the completeness instrument CF-27 owns.
Nothing in this section forecloses either answer, and the `#[non_exhaustive]` on
`SequencedEvent` (VT-4) is what keeps a per-event retention marker additive if
that is where it lands.

---

## 3. The `EventStore` port

This is the port the specification freezes. Everything else in the workspace is
downstream of it: the conformance suite measures adapters against it, the
projection runner reads through it, and the sync port replicates what it holds.
It is also the port with the most evidence behind it — eighty-nine conformance
rules, one reference implementation, six deployment scenarios walked line by line
against it, and one adjudicated pressure test. Where the evidence runs out, the
clause says so and names the experiment that ends the argument.

Clauses are `ES-n`. Each carries a maturity marker, the conformance rule that
checks it, the end-to-end cases it serves, and the wrong implementation it
forbids. A clause with no wrong implementation to name is not a clause; two such
statements appear below as prose and are labelled.

Rule names in `code font` that appear in
[`crates/happenstance-testkit/src/suite.rs`](../crates/happenstance-testkit/src/suite.rs)
today are existing rules. Rule names marked **(new)** are specified here and do
not exist yet; §3.8 collects them.

---

### 3.1 Derivation — why there are two traits, and what that costs

The port is declared once without any `Send` requirement and the `Send` flavour
is derived (`store.rs:141`, ADR-0001). The mechanism is not folklore and its
consequences are not guesses: `trait-variant 0.1.3` is 248 lines of `syn`
rewriting and every claim below cites it at
`~/.cargo/registry/src/index.crates.io-*/trait-variant-0.1.3/src/variant.rs`,
hereafter `variant.rs`.

Three facts about the expander, because every clause in this subsection is a
consequence of one of them:

1. **The variant is an independent trait, not a subtrait.** `mk_variant`
   (`variant.rs:106-126`) clones the `ItemTrait`, renames it, and appends the
   attribute's bounds to its **supertrait** list. `SendEventStore` does not have
   `EventStore` as a supertrait; the two are related only by the blanket impl.
2. **The attribute's bound list is appended to every return type — futures
   *and* streams.** `transform_item` (`variant.rs:129-169`) has two arms. For an
   `async fn` it rewrites the signature to `-> impl Future<Output = T> + <bounds>`
   (`:133-145`). For a method already returning `-> impl Trait` it chains the
   bounds onto the existing bound list (`:147-155`). `read` takes the second arm,
   which is exactly why `read` returning the stream at the top level is the whole
   design and not a style preference.
3. **A provided body is cloned into the variant verbatim, with `asyncness`
   stripped.** `transform_item` returns `TraitItemFn { sig: Signature {
   asyncness: None, output: <rewritten>, ..sig.clone() }, ..fn_item.clone() }`
   (`variant.rs:161-168`). The `..fn_item.clone()` carries the `default` block
   across untouched. This single line explains both the `async fn` failure and
   the obligation nobody has written down.

#### ES-1 — One definition, two flavours, and generic code binds the weaker one

`EventStore` MUST be defined without any `Send` bound. The `Send` flavour MUST be
derived by `#[trait_variant::make(SendEventStore: Send)]` rather than
hand-written. `#[async_trait]` MUST NOT be introduced. Generic code in this
workspace and in adapter crates MUST bind `EventStore`, not `SendEventStore`, and
MUST import only one of the two names per module.

**[FROZEN]**

- **Rule:** the suite binds the bare flavour and nothing else. Since the fixture
  contract, that is carried by `Fixture::Store: EventStore`
  (`crates/happenstance-testkit/src/contract.rs:125`) rather than by a bound
  written out on each rule — the rules are generic over `F: Fixture`, and the
  only `<S: EventStore>` bounds left in `suite.rs` are on its four private
  helpers. One associated-type bound is a better falsifier than thirty identical
  ones, not a weaker one: it cannot be relaxed for a single rule, only for the
  whole suite at once, and doing so fails every harness. An adapter implementing
  only `SendEventStore` proves the implication by passing the suite — which
  `MemoryEventStore` does (`memory.rs:293`). `cargo xtask wasm` proves the bare
  flavour still builds for `wasm32-unknown-unknown`.
- **Cases:** E2E-52, E2E-53, E2E-54.
- **Rejects:** any redefinition that injects `+ Send` unconditionally. Such a port
  compiles and passes every behavioural rule on native and fails
  `cargo xtask wasm`, which is why that step is in the gate rather than in a
  comment.

The naming departs from `trait_variant`'s own convention, which would call the
bare flavour `LocalEventStore`. "Local" already names a local-first
application's on-device store in this project; the collision would be permanent
(ADR-0001:93-96).

#### ES-2 — `read` returns the stream at the top level and is not `async`

`read` MUST return `impl Stream<Item = Result<SequencedEvent, Self::Error>>` as
the outermost item of its return type, and MUST NOT be `async`.

**[FROZEN]**

- **Rule:** two unit tests in `happenstance-core`, and it takes both.
  `send_flavour_stream_is_send_in_generic_code` (`memory.rs:613-640`) writes the
  bound at the *definition* — `fn assert<S: SendEventStore>(s: &S, q: &Query) {
  fn is_send<T: Send>(_: &T) {} is_send(&SendEventStore::read(s, q,
  ReadOptions::new())); }` — so the obligation is discharged before
  monomorphisation and only the *trait*'s promise can satisfy it. Its predecessor
  asserted `Send` on a *concrete* stream, where auto-trait leakage from the hidden
  type satisfies it whatever the trait says. That is necessary and not sufficient:
  under the refactor below the outermost item is the *future*, `trait_variant`
  marks the future `Send`, and the assertion is discharged against the wrong
  thing. `spawns_from_generic` (`memory.rs:642-710`) is what rejects it, because
  it holds the stream across an await inside a real `tokio::spawn`
  (ADR-0008:220-228).
- **Cases:** E2E-52.
- **Rejects:** the refactor to `async fn read(..) -> Result<impl Stream, E>`.
  `transform_item`'s `async fn` arm (`variant.rs:133-145`) appends `Send` to the
  outermost `impl Future` and nothing else, so the future would be `Send` and the
  stream would not: a caller could not hold a read across an await inside
  `tokio::spawn`, which is the entire reason the `Send` flavour exists. The
  refactor compiles, passes every behavioural rule, and silently deletes the
  design. CLAUDE.md constraint 3 protects this and names both tests, for the
  reason above: a clause naming only the first would have been satisfied by the
  refactor it forbids.

#### ES-3 — The `Self: Sync` rule, stated once

Any provided method, extension-trait method, or generic helper whose body holds
`&self` across an `await` MUST carry `where Self: Sync` at the point of use.
`Sync` MUST NOT be added to the trait's own bounds or to the `trait_variant`
attribute in order to satisfy such a body.

**[FROZEN]**

The Rust reason, because it is the same reason four times over and stating it
once is the point of the clause: an `async move` block that captures `&self`
holds a `&Self`, and `&Self: Send` holds exactly when `Self: Sync`. So the
provided body's future, the extension trait's future, the future handed to
`tokio::spawn`, and any generic helper taking `&S` all want the same thing, and
all four can take it at the call site instead of the definition site. Requiring
it at the definition site would impose it on every adapter including those that
never call the method.

- **Rule:** `provided_method_future_is_send_in_generic_code` **(new)** — assert
  `Send` on the future returned by each provided method from a function bound
  `S: SendEventStore + Sync`.
- **Cases:** E2E-13, E2E-53.
- **Rejects:** `#[trait_variant::make(SendEventStore: Send + Sync)]`. It looks
  like one line. `transform_item`'s second arm (`variant.rs:147-155`) appends the
  *whole* bound list to `read`'s stream, so a stream whose hidden type is `Send`
  but not `Sync` — a `Cell` in a cursor, an `Rc` in a page buffer — is rejected
  with `error[E0277]: cannot be shared between threads safely`. That leg carries
  the clause on its own, which is as well, because the second leg this bullet used
  to offer is false: it said `memory.rs:154` and
  `crates/happenstance-sqlite/src/event_store.rs:998` "both write `+ Send` and
  would both need `+ Send + Sync`", and they would not. Flipping the attribute and
  running `cargo check --workspace --all-features` produced *zero* errors and
  touched neither impl — an RPITIT impl need not restate the trait's auto-trait
  bounds, and rustc checks the hidden type instead (ADR-0008:201-211). It is
  recorded as refuted rather than deleted because it is the argument a reader
  reconstructs unprompted, and because the real hazard is sharper than the one it
  named: an adapter author who writes `+ Send` at the impl site is silently held
  to `+ Send + Sync`, and learns of it only when some future hidden type fails,
  far from the cause.

#### ES-4 — One provided body must type-check under both flavours simultaneously

A provided method's body MUST be written so that it type-checks under the bare
flavour's bounds and the derived flavour's bounds at once. A provided method MUST
NOT be written as `async fn`.

**[FROZEN]**

`transform_item` clones the `default` block into the variant while setting
`asyncness: None` (`variant.rs:161-168`). It does not rewrite the body. So an
`async fn` provided method becomes a non-`async` function containing `.await` and
fails with `error[E0728]: await is only allowed inside async functions and
blocks`. The hand-desugared form is what survives the clone:

```rust
fn head(&self) -> impl Future<Output = Result<Option<SequencePosition>, Self::Error>>
where
    Self: Sync,
{
    async move { /* … */ }
}
```

In the variant this becomes the same body behind
`-> impl Future<..> + Send where Self: Sync`, and the `Sync` supplied at the point
of use is what makes the captured `&Self` `Send`. This was compiled and refuted
the evaluation's claim that `head()` and `count()` "can never be added"
(PRESSURE-TEST §1). The blanket impl overrides the default anyway for any type
that implements the variant (`variant.rs:194-237` emits
`<Self as SendEventStore>::head(self)`), so a provided body is a fallback for the
bare flavour, not a shared implementation.

- **Rule:** `provided_method_future_is_send_in_generic_code` **(new)**, as ES-3.
- **Cases:** E2E-13.
- **Rejects:** the `async fn` spelling of any future provided method — it fails to
  compile, which is the cheapest possible rejection, but only if someone knows to
  expect it. It is documented in CONTRIBUTING for exactly that reason.

#### ES-5 — Associated-type bounds are identical on both flavours by construction

The `Error` associated type's bounds MUST be stated once on the bare trait. They
MUST NOT be varied between flavours.

**[FROZEN]**

`transform_item` returns non-`Fn` trait items unchanged (`variant.rs:129-132`), so
`type Error: core::error::Error + 'static` (`store.rs:149`) is copied into the
variant verbatim, and the blanket impl forwards it as
`type Error = <Self as SendEventStore>::Error` (`variant.rs:238-245`). The other
spelling is carried by a different line, and it is the one worth naming, because a
`where` clause is the first thing anyone reaches for and it is the one
`transform_item` never sees: `mk_variant` builds the variant with `..tr.clone()`
(`variant.rs:123`), which copies the trait's generics — the `where` clause
included — across untouched. There is no mechanism by which `Error: Send + Sync`
could apply to the derived flavour alone. Whatever ES-6 settles applies to
`wasm32` too.

- **Rule:** `error_bound_is_identical_on_both_flavours` **(new)** — a static
  assertion in `happenstance-core`'s own tests over both flavours' `Error`
  projections.
- **Cases:** E2E-53.
- **Rejects:** the plausible compromise — abandoning `trait_variant` for two
  hand-written traits so the bound can differ. That is the `umadb-dcb` shape
  ADR-0001:99-100 rejected, it doubles the surface the suite must cover, and it
  loses the blanket impl that makes ES-1's "bind the weaker one" work.

#### ES-6 — `Error` carries no `Send` or `Sync` bound, and the strength is a marker

**[FROZEN]**

`type Error: core::error::Error + 'static` (`store.rs:149`) carries no `Send` or
`Sync` bound, so an adapter error holding a value that is not thread-safe
satisfies it, and a spawned handler's error cannot cross a `JoinHandle` — which
is E2E-53's failure. Whether that trade was the right one could not be settled
while nothing in the tree could fail the bound, and for a long time nothing
could: `MemoryStoreError` is uninhabited (`memory.rs:284-291`) and
`SqliteEventStoreError` *was* a single placeholder variant. Phase 2 built errors
that can, and phase 8 built the rest of them: `SqliteEventStoreError` is now
twelve real variants over `rusqlite::Error`, `JoinError`, `TryCurrentError` and
the crate's own decode failures
(`crates/happenstance-sqlite/src/event_store.rs:1011-1104`), and
`CloudflareEventStoreError` is `!Send` and `!Sync` transitively because
`SqlError::Thrown` carries a `JsThrow`, whose payload is an `Rc<worker::Error>`
(`crates/happenstance-cloudflare/src/js.rs:168-173`).

**The `Rc` is the instrument, and it is not what this clause first assumed.** A
real `JsValue` **is** `Send + Sync` on the target Workers actually builds:
`wasm-bindgen` carries an `unsafe impl` of each under
`cfg(not(target_feature = "atomics"))`, and Workers builds
`wasm32-unknown-unknown` without atomics
(`crates/happenstance-cloudflare/src/js.rs:28-35`). So `worker::Error` is not the
hazard — an instrument whose `!Send`-ness is one `cfg` away from evaporating
cannot falsify a bound, which is why phase 9's real bindings hold every JS-side
value behind an `Rc` rather than bare, keeping the thrown value live and the
auto trait off it. Nor is that hatch available here: the workspace sets
`unsafe_code = "forbid"`, so an adapter can only ever *inherit* it by holding a
`JsValue`, never write it.

Compiled against that instrument, `+ Send + Sync` costs exactly one crate — the
`wasm32` target the two-trait design exists to serve — and ES-5 leaves no
mechanism to scope it to the native flavour.
[ADR-0009](../.kb/decisions/0009-error-send-sync.md) therefore settles the clause the other
way: **`Error` keeps `core::error::Error + 'static` on both ports and both
flavours, and the stronger property becomes a marker trait that generic code asks
for** (`references/adr/0009-error-send-sync.md:117-126`). Nothing is left to time
against publication. The bound would have been semver-visible and one-way; a
marker declared downstream is additive, and needs nothing from the contract
crate.

- **Rule:** `store_error_crosses_a_join_handle` **(new)**, in a rule group whose
  bound is the marker rather than `SendEventStore` — the bare flavour cannot be
  spawned by construction, and the derived one does not imply a `Send` error. It
  asserts on the future's `Output`, not on the future: `Send` on a future is a
  property of what it holds across a suspension point, so a future with no
  suspension point is `Send` whatever it returns, and a rule that spawns `append`
  and reports "it compiled" passes against an error that can never cross a
  `JoinHandle` (`references/adr/0009-error-send-sync.md:98-115`).
- **Cases:** E2E-53, E2E-52.
- **Rejects:** deciding it by argument. Both prior documents did, in opposite
  directions, from the same file. The implementation the rule rejects is already
  in the tree: `SendStoreWithLocalError`
  (`crates/happenstance-cloudflare/src/send_shape.rs:137`) satisfies every `Send`
  obligation the derived flavour states and carries a `!Send` error, and compiles.

#### ES-7 — A downstream crate may implement the bare flavour directly

Implementing `EventStore` for a local type in a downstream crate MUST NOT collide
with the blanket impl the derivation emits.

**[PROVISIONAL — falsified by `error[E0119]: conflicting implementations` on a
downstream `impl EventStore for LocalType`. The named test is the `!Send`
reference store, which lives in `happenstance-testkit` — a genuinely downstream
crate — and which is also ADR-0001's own lift condition.]**

`mk_blanket_impl` (`variant.rs:171-192`) emits `impl<T: SendEventStore>
EventStore for T`. ADR-0001:70-76 recorded a spike confirming a downstream
direct impl is accepted, and for as long as nothing in the tree corroborated it
the ADR stayed provisional. Four impls now do, and one of them is the proof
rather than the breadth: `LocalMemoryEventStore`
(`crates/happenstance-testkit/tests/local_conformance.rs:198`) is a direct
`impl EventStore for` in a genuinely downstream crate, sitting beside the
blanket impl without `error[E0119]` and passing every rule natively and on
`wasm32`. `CloudflareEventStore`
(`crates/happenstance-cloudflare/src/event_store.rs:992`) and
`happenstance-neon`'s two (`crates/happenstance-neon/src/event_store.rs:168`,
`:405`) are skeletons and
widen the evidence without adding to it. ADR-0001's provisional marker was
lifted on that basis at phase 1 (`references/adr/0001-async-port-flavours.md:5-6`);
what remains open, and is phase 9's, is whether a real platform SDK fits.

- **Rule:** not a new rule but a new *invocation* of the existing suite — the
  `Rc`-backed `!Send` reference store CF-28 requires in
  `happenstance-testkit`'s own `tests/`, running the whole suite. CF-28 owns the
  instrument; this clause is what it is evidence for.
- **Cases:** E2E-52, E2E-09, E2E-54.
- **Rejects:** shipping 0.1 on a coherence property that has never been compiled
  outside the crate that declares it.

---

### 3.2 `read`

```rust
fn read(&self, query: &Query, options: ReadOptions)
    -> impl Stream<Item = Result<SequencedEvent, Self::Error>>;
```

The signature is settled by ES-2. What follows is what it means, including the
two things the contract has never said and that three of six scenarios tripped
over.

#### ES-8 — Ordering

Events MUST be yielded in ascending `SequencePosition` order, or descending when
`ReadOptions::backwards` is set. `ReadOptions::from` MUST be **inclusive** in both
directions: forwards it is a lower bound, backwards it is an upper bound.

**[FROZEN]**

- **Rule:** `read_defaults_to_ascending_order`, `read_from_is_inclusive`,
  `read_backwards_reverses_order`, `read_backwards_from_with_limit`.
- **Cases:** E2E-10, E2E-12.
- **Rejects:** an adapter that implements `from` as a lower bound irrespective of
  direction — the natural reading of `WHERE position >= ?` copied into the
  backwards branch. It passes `read_from_is_inclusive` and fails
  `read_backwards_from_with_limit`, which is why that rule earns its keep.

#### ES-9 — `from` names a position, not an index

`ReadOptions::from(p)` MUST be evaluated as a range predicate over assigned
positions. When no event occupies `p`, the read MUST yield the next matching
event above `p` (forwards) or below `p` (backwards). It MUST NOT error and MUST
NOT return empty on that ground alone.

**[FROZEN]**

- **Rule:** `query_matching_nothing_yields_empty` is the
  complementary half already in the suite — an empty result is the correct
  answer when nothing matches, and never an error — and it is named here
  because no other clause names it. `read_from_a_gap_position` — read `from` the position immediately **above the
  head**, which is unoccupied on every store whatever it allocates and wherever
  it starts, because nothing has been appended since. Forwards it must yield
  nothing and must not error; **backwards it must yield the next matching event
  below it**, which is the half of this clause no existing rule reaches.
  `read_from_is_inclusive` and `read_backwards_from_with_limit` are the only two
  rules that pass `from` at all, and both hand it a position the store actually
  assigned. Where the fixture's own allocator leaves an *interior* gap — a
  sequence with `CACHE 7`, a transaction id, a shard id in the low bits — the
  rule additionally reads from inside it in both directions; on a dense store
  that half is skipped rather than faked, and the clause says so here so that a
  reader of the rule finds the reason at the clause rather than in a comment.
  The arrangement this clause used to suggest — remove an event through the
  adapter's own fixture, or use an adapter that allocates sparsely — cannot be
  performed by a portable rule: the port has no delete, `Fixture` declares no
  such capability, and the second describes an adapter other than the one under
  test. `FromIsAnOffsetStore` is the `rowid`-offset lookup this clause describes
  in prose, and `BackwardsIgnoredStore` is the second registered store it
  rejects — a backwards branch that never reaches the SQL turns the same
  predicate into a floor and the read comes back empty.
  `reading_an_empty_store_yields_nothing` is the same answer one
  degenerate step further out — a read against a store that has never been
  appended to yields an empty stream and no error — and every rule in the suite
  seeded the store before it read. It landed at phase 3 with `NullHeadPagingStore`
  beside it, and the pairing is worth recording because this rule looked like the
  suite's best candidate for *having no plausible failing implementation*, which
  ADR-0010 says to record in the clause and skip rather than to invent a mutant
  for. It has one, and this clause's own `Rejects:` had already named it: the
  paginating adapter ES-11 prescribes, anchoring its window on the head at the
  first poll. `max(position)` over no rows is `NULL`, and decoding it into a
  non-nullable integer is a decode error rather than a zero. The rule also
  exercises the read options against the empty store, because the missing head is
  *spent* on the paging arithmetic rather than merely observed.

  **One half of that rule has a saboteur and the other half does not, and
  ADR-0010 §1 requires the asymmetry stated rather than papered over.**
  `reading_an_empty_store_yields_nothing` makes two claims — the read yields
  nothing, and it does not error. `NullHeadPagingStore` is registered against the
  *error* half, and it is deliberately modelled as an `Err` item on the stream
  rather than as a panic, so the rule's own `read_ok` raises the rejection and
  the registry can hold the row to an origin check. The "yields the wrong
  events" half has **no plausible failing implementation**: nobody ships a store
  that invents rows for a log nothing has been written to, and inventing a
  saboteur for it would be the strawman CF-4 rejects. That half is retained
  because it is one line and it is the sentence a reader of the rule expects to
  find, not because anything demonstrates it.
- **Cases:** E2E-10.
- **Rejects:** an adapter implementing `from` as an equality seek or a
  `rowid`-offset lookup rather than a range scan — plausible wherever positions
  came from a dense counter and the author assumed density. `event.rs:215-217` says
  gaps are permitted; nothing checks that anyone believed it. It also rejects the
  paginating adapter ES-11 prescribes, which anchors its window on `head()` at
  the first poll: on an empty store `head()` is `None` (ES-30), and the arithmetic
  written against that bound errors or panics on a store whose only fault is
  being new — the state every adapter is in on its first run.

#### ES-10 — Position order is visibility order

Once any reader has observed an event at position *P*, no subsequent read
against that store MAY yield an event at a position ≤ *P* that was not already
visible. An adapter MUST NOT make an event visible at a position below one it has
already exposed.

**[FROZEN]**

Freezing this clause moves the **position-allocation** axis *into* ADR-0013's
CF-25 acceptance list rather than out of it. The `[PROVISIONAL]` marker was that
axis's disclosure — it named the axis, the unbuilt adapter far end and the
falsifier — so lifting it deletes the disclosure, and the acceptance has to move
in the same act or the exposure disappears silently. The far end is still a
fixture (`PreCommitPositionStore`, CF-13) and still no adapter, which by CF-26
buys falsifiability and not implementability.

A conformant adapter buying this invariant with `xid8` + `pg_snapshot_xmin`
reports a **frontier** from `head()` rather than `max(position)`, and therefore
does **not** satisfy read-your-own-writes: `append` returning `Ok(P)` does not
promise that the next `head()` is at or above *P*. Staleness is bounded by the
longest open write transaction *anywhere in the cluster* — 0.688 ms with no
holder and 4010.719 ms behind an unrelated five-second write in an unrelated
database, both measured
(`experiments/position-visibility/results/staleness_pinned.txt`). That is a
documented capability limit, not a tuning parameter, and it is why ES-30's
`head_is_the_highest_visible_position` asserts a bound rather than an equality.

`nothing_below_an_observed_position_appears_later` has a strength that varies
with the adapter's poll shape. The rule polls two `append` futures A, B, B, A and
`Fixture` cannot express a poll budget, so against a store whose `append` needs
three polls the interleaving window never opens where the rule looks and the rule
cannot fail. The bounding instrument — a poll-padding decorator over
`PreCommitPositionStore` — is named and owed by **phase 10** (ADR-0024); if it
fires, the rule changes and this clause does not.

**This clause is the sole statement of the visibility invariant.** VT-12 carried a
second copy; the two drifted apart within one editing pass, which is the argument
against stating a requirement twice. VT-12 is retained as a cross-reference so
existing citations resolve, and every `PS` and `SY` clause that depends on the
invariant cites this one.

It was provisional on cost, not on correctness, and the distinction decided what
a bad result would have meant. ES-25 and ES-26 are sound only where visibility
order agrees with position order, so they are built on top of this clause: if it
fell they fell with it, and that is a contract change rather than an adapter
inconvenience. Phase 2's probe ran against a real PostgreSQL with `fsync=on`
([`experiments/position-visibility/`](../experiments/position-visibility/README.md))
and settled which mechanism buys the invariant. **Arm C — `xid8` +
`pg_snapshot_xmin` — is the only arm that both passes the inversion detector on
both writer pairs and leaves writers unserialised**, at a throughput ratio to a
bracketing baseline of 0.987 / 0.993 / 1.015 / 1.026 at 1 / 8 / 32 / 64 clients.
The one-client figure is `results/ratios-c1long.csv`'s separate 90-second pass;
`ratios.csv` records 0.628 there against a baseline that drifted 3.70×, and
reading that series alone reports arm C as 37% slower at one writer
(`experiments/position-visibility/README.md:248-256` explains the
substitution). Both declared positive controls
fired, which is what makes the positive worth anything: the baseline reproduces
the inversion, and arm A at 64 clients collapses to 0.062× of baseline with p99
60× worse. **Three unaffordable answers would not have made the clause wrong;
they would have made it expensive, and the decision then would have been whether
happenstance requires something Postgres cannot cheaply give.** That
counterfactual is kept because a freeze that never had a way to fail is not a
freeze. The invariant is also kept **global** rather than per-boundary
deliberately: arm B-tag's tag-keyed advisory lock is nearly free (0.935 at 64
clients) and buys a per-boundary invariant, which makes `AppendCondition` sound
and the projection checkpoint unsound — `head()` is not query-scoped (ES-30), so
a runner checkpoints on a global position covering boundaries it never reads.
ADR-0013 §3 has the argument and its falsifier.

This is the property that makes `AppendCondition::after` mean anything.
`is_violated_by` compares position *values* (`append.rs:239-253`); nothing in the
contract requires an event becoming visible later to carry a higher position.
`event.rs:215-217` documents uniqueness, monotonicity and permitted gaps — all
properties of *assignment*, none of *visibility*.

- **Rule:** `nothing_below_an_observed_position_appears_later`. The rule is
  worthless without something that can fail it, and that store is now in the tree.
  `PreCommitPositionStore`, in
  `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs`, takes its
  position from a sequence before its transaction commits and publishes the row
  afterwards, so a writer that started first and commits last becomes visible
  underneath a position a reader has already observed. The rule creates two
  `append` futures from one handle, drives them one poll at a time in the order
  A, B, B, A, and reads after every step; CF-13 records why the reversal on
  resumption is load-bearing and why the whole thing is single-threaded.
  `positions_are_unique` and `positions_are_strictly_monotonic` both read a
  quiescent store through a single sequential writer and are vacuous here — indeed
  vacuous generally, since both read back through `read`, which every adapter
  returns in position order.
- **Cases:** E2E-01, and the read-side half of E2E-02.
- **Rejects:** a Postgres adapter allocating positions with `nextval()` outside
  the transaction. A position is taken before the work and released at commit, so
  a transaction that started later can become visible earlier; a caller then
  conditions on `after: 100` while 99 is still invisible, and the boundary silently
  stops enforcing. It passed every rule in the suite until the rule above and
  ES-36's landed — the same window seen twice, which is why
  `PreCommitPositionStore`, the adapter in its non-concurrent deterministic form,
  is registered against both.

  Wattline is the worked failure, and it is worth carrying the numbers because
  "the window is small" is the argument that gets made here. A handler reads a
  decision model that ends at position 100 and appends with `after: Some(100)`.
  Position 99 belongs to a transaction that started earlier and commits nineteen
  milliseconds later. When it becomes visible, `is_violated_by` evaluates
  `99 <= 100` and returns false, so the guard that exists to catch exactly this
  reports no violation. The result is two active sessions on one connector: no
  error, no rejected append, no failing test, and a read model that is correct
  about a state the business forbids.

#### ES-11 — A read is a snapshot

One `read` call MUST be evaluated against a single consistent state of the store,
fixed no later than the first poll of the returned stream. Events appended after
that instant MUST NOT be yielded. No event that was visible at that instant and
matches the query MAY be omitted.

**[PROVISIONAL — axis: **transport**, whose far end is unbuilt. `MemoryEventStore` snapshots under the lock at call time (`memory.rs:296-336`) and satisfies this for free; an adapter reaching its store over one-shot HTTP with no cursor can only self-paginate, and a self-paginating read is not a snapshot. Falsified by the first one-shot-HTTP adapter that cannot meet this in one round trip — which is the outcome to expect, and the reason to have settled it before the freeze rather than after.]**

This settles D7 and E2E-02. Two conformant adapters have opposite semantics
today: `MemoryEventStore` filters, orders and truncates under the read lock and
streams from the resulting `Vec` (`memory.rs:296-336`), so its answer is a
snapshot; a self-paginating adapter over one-shot HTTP issuing an independent
`WHERE position > $last ORDER BY position LIMIT 5000` per chunk grows under the
caller's feet. The observable difference is one event versus two.

The obligation is stronger than laziness, and laziness survives it. The sample
may be taken **at or before** the first poll and a caller may not depend on
which. This is defect D7, and it is what makes `MemoryEventStore` (which filters,
orders and truncates under the read lock at call time) and `happenstance-sqlite`
(which does not: `read` is not `async` and may legally be called with no runtime
in scope, where `spawn_blocking` panics, so its work moves into `poll_next` and
its ceiling is sampled there — `Ceiling::Unsampled` at
`crates/happenstance-sqlite/src/event_store.rs:1337` is the state the first poll
resolves) both conformant. ADR-0022 §9 settled that seam rather than leaving it
to the call site: the store captures a `tokio::runtime::Handle` at construction
and falls back to `Handle::try_current`, so the lazy spawn has a runtime to hop
onto even under the concurrency family's bare OS threads, and
`SqliteEventStoreError::NoRuntime` keeps a reachable meaning instead of becoming
dead code. Laziness is
therefore **permitted and never required**, and building a stream and never
polling it is not guaranteed to be free. What no adapter may do is take a *new*
snapshot per chunk. An adapter that issues **more than one statement per `read`**
MUST capture a position ceiling *H* no later than the first poll, and MUST bound
every statement after the first by `position <= H` — or `>= H` under
`backwards` — one extra round trip at most, and zero if the first statement
returns the head alongside the first page.

The ceiling is only a ceiling if nothing below *H* can become visible later,
which is ES-10. **ES-11 and ES-12 reduce to ES-10 plus a ceiling**: neither is
implementable on any multi-statement adapter without ES-10, and both are nearly
free with it. The MUST has one already-documented failure mode: on an empty store
`head()` is `None` (ES-30), and a ceiling computed as arithmetic on that bound
errors or panics on a store whose only fault is being new — the state every
adapter is in on its first run. `reading_an_empty_store_yields_nothing` is the
rule and `NullHeadPagingStore` the registered adapter; ES-9 owns both. **This is the reason ES-30 (`head`) is in the port and not an inherent
method**: without it, the cheapest correct pagination is unavailable through the
port, and Neon's inability to meet the obligation in one round trip becomes an
argument for weakening the obligation rather than for supplying the primitive.

- **Rule:** `read_result_is_stable_under_concurrent_append` — bind the query to
  a named local, build the stream, poll it once, append, drain, and assert the
  drained set is exactly the pre-append set. The rule polls the stream **once
  before it appends**, and that is the reason it is portable at all rather than
  an incidental detail: the sample may be taken at or before the first poll and
  a caller may not depend on which, so a rule that appended before the first
  poll would fail a conformant deferring adapter at random while passing a
  buffering one, and the failure would read as a flake rather than as a rule
  asking the wrong question. `RefetchingPagedStore` in the testkit's own
  `tests/` is the registered adapter that fails it: a store with no cursor
  issuing an independent statement per page against whatever it holds now.
- **Cases:** E2E-02, E2E-01.
- **Rejects:** the self-paginating one-shot-HTTP adapter above — the natural shape
  for `happenstance-neon`, and one of the two instruments the workspace has
  already chosen. It is conformant today.

**A coverage hole opened when this rule's neighbour was promoted, and it is
recorded here so it is not rediscovered.** `a_live_read_stream_does_not_block_an_append`
was a `#[tokio::test]` in the testkit's own `tests/` until phase 3, and it
asserted that the drained stream held **one** event — "the stream is a snapshot
taken at read". Promoting it to a portable rule correctly dropped that
assertion, because a genuinely streaming adapter may legally see the later row
and this clause is the one that decides whether it may; the promoted rule asserts
only that the pre-existing event is still yielded and defers the rest here. The
consequence is that **nothing in the workspace now asserts that any store's read
stream is a snapshot** — `MemoryEventStore` and `LocalMemoryEventStore` both are
one, and neither is checked. This rule is what restores it portably. A unit test
in `happenstance-core` would restore the specific coverage sooner without
over-specifying the port, and is the cheaper half if this stays blocked.

#### ES-12 — All items of one `Query` share one snapshot

Every item of a `Query` MUST be evaluated against the same snapshot as every
other item of that same `read` call.

**[PROVISIONAL — axis: **transport**, with ES-11, and strictly harder: an adapter that issues one statement per query item satisfies ES-11 per item and still fails this. Falsified by the same adapter, and the failure mode is quieter — an event matching item 1 that lands between two statements is missed while the observed maximum position sits above it, so a condition built on that boundary admits a write it should have rejected, with no error anywhere.]**

- **Rule:** `query_items_share_one_snapshot` — a two-item query whose items
  carry distinct type lists, built, **polled once**, then an event matching the
  **last** item appended, then drained: the drained set must be exactly the
  pre-append set. It is `read_result_is_stable_under_concurrent_append`'s shape
  with a multi-item query and a late-matching append, and it is writable without
  any hostile fixture — the pause point is the poll boundary the rule itself
  controls, the same instrument CF-13 records for ES-10. The assertion is
  deliberately the strong, non-disjunctive one: an earlier draft of this field
  offered "either the late event is in the result **or** the maximum position
  observed is below it", which an adapter that tears and then reports a maximum
  below the tear satisfies — exactly the caller-visible failure this clause's
  own falsifier describes.
  Two things follow. **ES-12 is discharged by ES-11's ceiling rather than by a
  second mechanism**: an event matching item 1 that lands between item 1's
  statement and item 4's has a position above the ceiling captured at the first
  poll and is excluded by the same predicate, provided nothing below *H* can
  become visible later, which is ES-10. And the tear this clause's `Rejects:`
  names — one SQL statement per `QueryItem`, unioned client-side — has **no
  separately registered adapter**: `RefetchingPagedStore`, which takes a fresh
  sample per page, is the same defect at a different granularity and is what
  rejects this rule. A second store for the per-item granularity would buy a
  shape in the catalogue and no coverage, which is a consequence of the bullet
  above rather than a gap.
- **Cases:** E2E-03, E2E-05.
- **Rejects:** a one-round-trip adapter that emits one SQL statement per
  `QueryItem` and unions the results client-side. That needs no `nextval()` and no
  visibility hole — only two statements — and it is the natural shape for the Neon
  peer. E2E-03's note is load-bearing and worth repeating: this is *not* E2E-10's
  defect. On a store where visibility order agrees with position order, a lazily
  chunked *forward* read cannot miss a matching event, because anything committing
  after the cursor has passed lands above the cursor. The tear is real only across
  separate statements in separate snapshots.

#### ES-13 — `read` takes `&Query`, and the signature does not change

`read` MUST continue to take `query: &Query` by reference.

**[FROZEN]**

Writing ES-11's and ES-12's rules requires binding the query to a named local:

```rust
let query = Query::all();                          // named local, lives long enough
let stream = store.read(&query, ReadOptions::new());
```

Not because of a lint, but because `-> impl Stream` in return position captures
the lifetimes of all of the method's inputs, including `&query`. Writing
`store.read(&Query::all(), ..)` and binding the stream to a `let` creates a
temporary `Query` that is dropped at the end of that statement while the stream
still borrows it: `error[E0716]: temporary value dropped while borrowed`. The
existing rules never hit it because they pass the temporary and drain the stream
inside one expression, which is what the suite's `read_ok` helper does.

The alternative was taking `Query` by value. It loses on cost and on retry
ergonomics: `Query::Items` is a `Box<[QueryItem]>` (`query.rs:170`) whose items
each own a `Box<[EventType]>` and a `Tags`, so by-value would allocate on every
read, and a command handler that reads with a query and then reuses it in the
append condition — the documented shape (`append.rs:49-63`) — would clone it
anyway. A borrow costs the caller one named local in the one situation where the
stream outlives the statement.

- **Rule:** `read_result_is_stable_under_concurrent_append` does not compile
  without this shape, so the rule is the check. It landed at phase 4, and the
  substantive reason it discriminates is not its name: ADR-0011's dossier E3
  compiled that a by-value `read` supports every shape this rule needs, so what
  the rule pins is the *lifetime* the returned stream captures — the named local
  its body binds the query to is the whole demonstration — rather than the
  isolation property it is named for.
- **Cases:** E2E-02, E2E-03.
- **Rejects:** the "fix" that changes `read` to take `Query` by value in order to
  make the E0716 go away. It compiles, it breaks every adapter signature, and it
  puts an allocation on the hottest path in the port.

#### ES-42 — `read`'s return type carries no `Unpin` bound

`read`'s return type MUST NOT carry a `+ Unpin` bound.

**[PROVISIONAL — falsified by a consumer that needs `dyn EventStore` where the
port has acquired a method the hand-written `Pin<Box<…>>` wrapper cannot box: a
generic method, which is not dyn-compatible, or a return whose lifetime the
wrapper cannot name. Inconvenience is not the falsifier. Must be re-evaluated
before phase 12: adding a bound to an opaque return type after publish is
breaking, so this clause expires rather than drifts.]**

- **Rule:** compile-level — the erasure wrapper of ADR-0011's E11 compiles and
  round-trips against `MemoryEventStore` through a hand-written `dyn` wrapper,
  with no unsafe code and no change to the port. `Pin<Box<dyn Stream + 'a>>` is
  itself a `Stream` via the standard blanket impl over `Pin<P>`, and
  `Pin<Box<T>>` is `Unpin` whatever `T` is — which is why boxing is sufficient,
  why no pin projection is needed, and therefore why no unsafe code appears in a
  workspace that forbids it outright.
- **Rejects:** the "fix" that adds `+ Unpin` so a macro-generated erasure
  compiles. It buys an erasure that fifteen lines of downstream code already
  provide (E11), and it permanently forbids a generator-backed stream —
  compiled at E12, `error[E0277]: … cannot be unpinned` on a `stream!` macro —
  which is the natural implementation of the chunked-cursor shape, the shape
  with no implementation in the workspace and therefore no vote.

No `Cases:` line: `spec_trace` requires only that named cases exist, and this
clause names none.

**ID note.** This clause was drafted as ES-41. ADR-0014's membership clause took
that number in the same slice, so it moved rather than colliding — recorded
because both drafts are in the branch's history.

#### ES-14 — `limit` truncates the whole result, after ordering

`ReadOptions::limit(n)` MUST yield the first *n* events of the ordered result set,
not *n* per query item and not an arbitrary *n*.

**[FROZEN]**

- **Rule:** `read_limit_truncates`, `read_backwards_from_with_limit`, plus
  `limit_applies_across_items_not_per_item` — a two-item query matching
  three events each, `limit(4)`, asserting four events in position order — and
  `read_limit_applies_after_filtering` with its mirror
  `read_backwards_limit_applies_after_filtering`: a store laid out
  `Miss, Hit, Hit, Hit, Miss`, read at `limit(2)` through a query selecting the
  hits, asserting two *matches* rather than whatever survives among the first two
  rows scanned. The mirror is not decoration and the layout is why: `LIMIT`
  pushed into a descending scan takes the *highest* rows, so a non-matching event
  at only one end would let one direction pass while the other failed. CF-12 is
  the obligation to run any read option against a filtering query at all, and
  names `limit` and `backwards` as belonging in the same pass.
  `read_from_composes_with_limit` is the sixth, added at phase 12: the budget
  spent on a read that also carries a **forwards** cursor. Until it landed, every
  rule that paired `limit` with `from` also carried `backwards`, so a store that
  applied the budget everywhere except its forward resume branch —
  `ForwardPagingBudgetStore` — passed all eighty-nine and was indistinguishable
  from a conformant one. The caller that composition serves is the one this
  clause and VT-28 are both written about: a paging loop resuming from its
  checkpoint.
  `read_to_composes_with_limit` is the seventh, and it is the budget beside an
  **upper bound** — ES-16's window and this clause's budget on one read, with the
  budget the smaller of the two. It was argued to be unnecessary and the argument
  was half right: `to` and `limit` both cut the back of the read and commute
  exactly, so a store that applies them in the wrong order answers every read
  correctly. Commuting is a statement about *order*, and the defect that arrives
  here is about *applicability*: `WindowedPagingBudgetStore` answers a closed
  window with its own statement — `BETWEEN ? AND ?` rather than `LIMIT ?` — and
  the budget was threaded into the paged statement alone, on the argument that a
  window makes a budget redundant. It is redundant only while the budget is the
  larger of the two, which is the case an author checks by hand and the opposite
  of the one a backfill worker is in on every call but its last. That store
  passed all ninety-two rules that preceded this one, measured with an empty
  `fails` list rather than argued.
- **Cases:** E2E-12, E2E-13.
- **Rejects:** an adapter implementing a multi-item query as one statement per
  item with `LIMIT n` on each — the same shape ES-12 rejects, failing here for an
  independent reason, which is why both rules are worth having. It also rejects a
  backwards adapter that truncates before reversing; `MemoryEventStore` reverses
  first (`memory.rs:312-332`) and this is the behaviour E2E-12 depends on. And it
  rejects the adapter that pushes `LIMIT` down into the scan and applies the
  query's predicate to the rows that come back, which returns fewer than *n*
  matches and sometimes none. Nothing in the suite could see that until
  `read_limit_applies_after_filtering` and its mirror landed: every read-option
  rule issued `Query::all()` — `read_defaults_to_ascending_order`,
  `read_from_is_inclusive`, `read_limit_truncates` and
  `read_backwards_from_with_limit` alike — where the scanned set and the matched
  set are the same set. `LimitBeforeFilterStore` is the compiled adapter, and a
  reviewer measured the shape passing the suite as it stood.

#### ES-15 — Query item algebra: order-free, duplicate-free, and All absorbs

For queries *a* and *b*, the set selected by `Query::from_items(items(a) ++
items(b))` MUST equal the union of the sets selected by *a* and by *b*. Item order
MUST NOT change the result set. An event matching two or more items MUST be
yielded exactly once. An adapter MUST NOT deduplicate, reorder or otherwise
rewrite a query's *items*.

**[FROZEN]**

- **Rule:** `duplicate_items_do_not_duplicate_events` — a store holding
  one multi-tagged event, read through `Query::all()` and through a two-item query
  both of whose items match it, asserting it appears once;
  `query_item_order_does_not_change_the_result_set`;
  `untagged_events_match_query_all` — one event with no tags at all,
  yielded by `Query::all()`; and `query_all_matches_every_event`.
- **Cases:** E2E-32.

**Two of those four are *hosted* here rather than entailed by the sentence
above, and saying so is the point of saying it.** The clause's MUSTs are about
item algebra: union across items, order-independence, yield-exactly-once.
`untagged_events_match_query_all` and `query_all_matches_every_event` assert a
**totality** property of `Query::all()` — that it selects every event in the
store, including one carrying no tags — and no MUST in this document states it.
They are here because this is the clause whose retirement would have lost them
and because the title's "All absorbs" gestures at it; that is a placement, not a
derivation. The consequence to be aware of is narrow and real: editing the
algebra sentence above would put nobody on notice that these two rules'
justification had moved. A totality clause of its own is owed, and it is an
ADR's to write rather than an edit's, because this clause is `[FROZEN]`.

**`query_all_matches_every_event` was retired here, and the retirement is
reversed.** The paragraph in this position said the rule was superseded by
`duplicate_items_do_not_duplicate_events`, on the reasoning that its three
*untagged* events cannot produce the fan-out a tag join without `DISTINCT`
gives, so it was weaker than the clause it would protect. The first half of that
is still true and is not the question. What the disposition missed is that a
retired rule is a rule nobody owns, and this one was doing work no other rule
does: phase 3 rewrote its three events into **descending** types `Cee, Bee, Ay`,
which is four characters and the difference between a rule and a decoration —
with `A, B, C` the expected answer is simultaneously insertion order and type
order, so `ORDER BY type, position`, which is what a covering index on
`(type, position)` gives you, passes. `SortByEventTypeStore` is that adapter and
`InnerJoinTagStore` is the other one; both are registered against this rule and
nothing else in the suite rejects the first on the untagged `Query::all()` path.

The lesson is ES-18's, arrived at a second time and worth stating here rather
than only cross-referenced: **reasoning about what a rule can reject is exactly
as unreliable as reasoning about whether an adapter is conformant.** The
retirement was written by reading the rule; the reversal by strengthening it and
then compiling the adapters it rejects. A reader who finds only the outcome will
re-derive the merge.

- **Rejects:** three real implementations. A tag join without `DISTINCT`: an event
  carrying three tags yields three rows, and nothing in the suite read
  `Query::all()` on a store holding a multi-tagged event until
  `duplicate_items_do_not_duplicate_events` landed — `TagJoinFanOutStore` is
  registered against it. The
  same join written as an `INNER JOIN`, which is the other half of getting one
  join wrong: an event with no tags has no row to join to and disappears from
  `Query::all()` entirely, which is silent data loss on the query every
  projection runner starts from. And
  an adapter that sorts and dedups *items* as an optimisation —
  `QueryItem::new` already sorts and dedups *types* (`query.rs:62-67`), so
  extending it one level up is the natural next step.

  **That third one has no rule, and saying so is the honest answer rather than a
  gap.** `query_item_order_does_not_change_the_result_set` cannot catch it:
  sorting the items is precisely what makes their order stop mattering, so an
  adapter that sorts them passes an order-invariance rule by construction. What
  the rule does catch — and what the registry's `ItemOrderedUnionStore` is — is a
  store whose *output* order is the item order, one statement per item unioned
  client-side and never merge-sorted. The sorting adapter is caught by a
  match-*set* rule with an item whose presence changes the set, which is VT-31's
  `query_union_is_item_concatenation`. It landed at phase 4 with
  `ItemDedupByTypeStore` beside it: a store that interns a query's items **by
  their type list**, so a second item carrying the same types is dropped and its
  tag constraint goes with it. That store passes every order-invariance rule by
  construction, which is the whole content of the paragraph above, and it is the
  registered adapter this clause's third `Rejects:` entry had been describing
  with nothing to point at. The fan-out projection
  runner (one read of the union query, twenty local re-filters through
  `Query::matches`, `query.rs:216-226`) rests entirely on this algebra, and
  nothing in the crate states it.

#### ES-16 — An upper bound on a read

`read` MUST honour `ReadOptions::to` as an **inclusive** upper bound in position
order, and MUST NOT yield an event beyond it. Under `backwards`, `from` remains
the starting (higher) bound and `to` the stopping (lower) one. An adapter MUST
NOT ignore the field.

**[FROZEN]**

The field itself is VT-29's; this clause is the read semantics it implies, and it
is stated here because `read` is where an adapter can get it wrong. This was
drafted as a deferral and the deferral's own argument is why it is frozen:
`ReadOptions` is `#[non_exhaustive]` and passed by value, so adding `to` was not a
trait signature change — but it *is* a new obligation on every adapter, and an
adapter that ignores an unknown field silently returns too much. **If `to` landed
at all it had to land before the first adapter shipped**, and none has.

**Discharged at phase 4.** `ReadOptions` now carries `from`, `to`, `backwards`
and `limit` (`query.rs:269-289`), with `to` inclusive in both directions
(`query.rs:318-330`) and asserted by `to_is_recorded_and_independent_of_from`
(`query.rs:568-578`). The state this clause was written against is the one it
forbids: with `from`, `backwards` and `limit` and no upper bound, a backfill
worker could not be given the closed window [1, *H*] while a tail worker owned
(*H*, ∞), and `limit` could not stand in, because `event.rs:215-217` forbids
treating position arithmetic as a count. The workaround —
`from(H).backwards()`, buffer, reverse in memory — *is* a pinned window and it
works at 211 events and is fatal at 53 million.

- **Rule:** `read_to_is_inclusive`, `read_from_and_to_bound_a_closed_window`,
  `read_to_under_backwards_bounds_the_older_end`,
  `read_to_composes_with_multi_item_query` — VT-29 names the first three, which
  are the ones about the *field*. The fourth is this clause's alone and landed
  later; see the amendment below.
  `ToBoundIgnoredStore` is the options struct matched on the fields it knows and
  `ToIsExclusiveStore` the exclusive reading; `BackwardsToIsAnUpperBoundStore` is
  the third, which this clause did not predict and which only the backwards rule
  sees — `WHERE position <= ?` copied verbatim into the descending branch, the
  shape ES-8's `Rejects:` already names for `from`, one bound over. It is
  *correct* reading forwards, so it passes the other two rules, and a backwards
  read comes back with the oldest events instead of the newest. That is why
  `read_to_under_backwards_bounds_the_older_end` is a rule of its own rather than
  a third assertion in `read_to_is_inclusive`.
- **Cases:** E2E-11.
- **Rejects:** an adapter that accepts `ReadOptions` by value, matches on the
  fields it knows and ignores the rest — the shape every `#[non_exhaustive]`
  options struct invites, and the one that turns a bounded backfill into an
  unbounded one with no error anywhere. It also rejects an adapter that reads
  `to` as exclusive, which yields a window one event short at every chunk
  boundary and is invisible until the chunks are reassembled.

**Amended: the three rules above all issue `Query::all()`, and one shape needed a
fourth.** `read_to_composes_with_multi_item_query` is the upper bound against a
*filtering* query, and it is CF-12's finding one bound over rather than a second
statement of this clause's own. The adapter it rejects generates
`WHERE a OR b AND position <= ?` — `AND` binds tighter than `OR`, so the window's
top is conjoined with the last disjunct alone and every event matching an earlier
item comes back above the window. With a single-item query there is nothing for
the `OR` to bind wrongly across, so all three rules above pass it, and the lower
bound is conjoined correctly, so `read_from_composes_with_multi_item_query`
passes it too. `UnparenthesisedToPredicateStore` is that store. Until this rule
it was registered as caught by the **model family alone** — which is behind an
optional dependency and `cfg(not(target_arch = "wasm32"))`, so the two adapters
in this workspace that will build their `WHERE` clause by concatenation and run
on that target, `happenstance-cloudflare` and `happenstance-neon`, ran nothing
that could see it.

A fifth rule exercises `to` and is **ES-14's** rather than this clause's:
`read_to_composes_with_limit` puts a window and a row budget on one read, with
the budget the smaller. What it rejects is a store that drops the budget because
the window is present, which is a failure of the *budget* obligation met through
this clause's field — so it is listed there, and named here so that a reader
counting this clause's rules knows where the fifth went.

---

### 3.3 `append`

```rust
async fn append(&self, events: &[Event], condition: Option<&AppendCondition>)
    -> Result<SequencePosition, AppendError<Self::Error>>;
```

#### ES-17 — The batch is borrowed, not owned

`append` MUST continue to take `events: &[Event]`.

**[PROVISIONAL — falsified by a measurement on a real adapter showing the
per-event clone is a material fraction of append cost. The named measurement is
the SQLite adapter's multi-row insert benchmark, in the phase that builds it. A
positive result changes the signature to take `Vec<Event>` **and** obliges the
contract to give callers a cheap way to keep a copy for retry.]**

An owning adapter must clone — `memory.rs:388-400` does. The consequence nobody
had written down is that `Event::into_parts` (`event.rs:404-427`) is unreachable
from any trait impl, so its doc comment — which read "Decomposes the event,
avoiding a clone in adapter write paths" — was **false as written**. This
specification required that comment corrected; `into_parts` is retained for
callers and wire encoders in the typed layer, which can reach it, rather than for
adapters, which cannot.

**That correction landed in `3c704d3`.** `event.rs:404-413` now says the opposite
of what it said, and says which one it used to be: *"Not a clone-avoidance route
for adapters, which is what this line used to claim."* The obligation stands over
every future edit to that comment — the reason the sentence was wrong is
structural, not a slip, and an adapter author reading a clone-avoidance promise
would go looking for a method they cannot call.

The borrow wins on three grounds. `Event`'s expensive fields are `Bytes`
(`event.rs:323`, `:325`), so a clone bumps a refcount rather than copying the
payload; the remaining cost is one `Box<str>` and one boxed tag slice, bounded by
the tag count. A rejected append clones **nothing** — `memory.rs:377-383` returns
before the `extend` — and rejection is the routine outcome under contention. And
`ConditionViolated` obliges the caller to keep its events across the call, so
by-value would move the clone from the adapter's success path to the caller's
every path.

- **Rule:** `append_preserves_event_payload` — the round-trip
  that catches a lossy clone.
- **Cases:** E2E-36, E2E-39.
- **Rejects:** an adapter that loses metadata or tag order while copying into its
  own row type — concretely, the SQLite shape that stores tags in a side table
  (`crates/happenstance-sqlite/src/event_store.rs:62-67`) and forgets to write
  `metadata`. `append_preserves_event_payload` compares the whole `Event` and
  catches it. That shape is no longer hypothetical: ADR-0022 landed
  `event_tag(tag, position)` as the tag index, beside a `tags` column on `event`
  itself (`:53`) which is what the round trip reads back — so the defect this
  bullet names is a real omission an implementer of *that* schema can make,
  rather than a sketch's.

#### ES-18 — Atomicity

Either every event in the batch lands or none does. A rejected append MUST leave
the store byte-identical.

**[FROZEN]**

- **Rule:** `append_is_atomic`, `condition_rejection_leaves_store_unchanged`,
  plus `append_is_atomic_under_a_mid_batch_fault` — failing the write of the
  *k*-th event of a batch, and asserting the store afterwards holds all of the
  batch or none of it, with which of the two decided by what `append` answered.
  Plus the concurrency family's `a_concurrent_reader_never_sees_a_partial_batch`,
  which is this clause's first sentence asked while the append is still running.
  Sequentially it cannot be asked at all: by the time a reader looks the call has
  returned, so every rule above sees only the final state and none of them can
  separate a store that applies a batch as one unit from one that applies it row
  by row and gets to the end. `RowAtATimeStore` is that store — every row lands,
  it is correct at rest, and it is wrong for exactly as long as the loop runs.
- **Cases:** E2E-39, E2E-48, E2E-07.
- **Rejects:** a per-row conditional `INSERT ... SELECT ... WHERE NOT EXISTS`,
  which can write the first event of a batch and refuse the second; and
  probe-then-insert outside a transaction, which writes the whole batch and then
  discovers it should not have. The first needs a fault injected between two
  rows, which is what the new rule is for. The second needs nothing injected at
  all, which is the correction below.

**`append_is_atomic` was retired here, and the retirement is reversed.** The
paragraph that stood in this position said the rule was merged into
`condition_rejection_leaves_store_unchanged`, on the reasoning that a rejection
comes from the condition check, the condition check precedes any write, and so a
rejected batch never reaches the write path and no partial write was ever
possible. Every step of that is true of `MemoryEventStore` and none of it is true
of the contract. **The reasoning was refuted by compiling a store.**

`WriteThenCheckStore` — autocommit plus a separate probe, which is the
non-concurrent form of the "probe-then-insert outside the transaction" shape a
reviewer measured passing the suite — extends its log with all three events,
*then* probes, then returns `Err`. The batch reaches the write path, the store is
left holding a partial batch it has just told the caller it refused, and
`append_is_atomic` is what catches it. `AfterDefaultsToFirstStore` and
`InnerJoinTagStore` fail it too; three registry rows name it.

The two rules are **complementary rather than successive**, and the distinction
is the whole reason this paragraph is here rather than just an outcome: one
rejects a store that writes before it decides, the other a store that cannot roll
back what it wrote. Whoever writes
`append_is_atomic_under_a_mid_batch_fault` must not delete the first while
writing the second. Stage 6 wrote it and did not.

**Stage 5 looked for it in the concurrency family and did not find it there.**
That is worth recording, because the concurrency family is where the next reader
will look too. A fault injected between two rows of a batch is **not a race**:
the rule that needs it has one caller and no threads, and putting it among the
racing rules would put it behind a `Send` bound it has no use for. Stage 5 gave
the mechanism as "a decorator over a store" and stage 6 refuted that — the
paragraph below is where it goes, and the fault turns out to be the *fixture's* —
but the conclusion the mechanism was offered in support of is unaffected: one
caller, no threads, wrong family. What the concurrency family *did* land is the
adjacent half, and the two
are easy to mistake for each other — `a_concurrent_reader_never_sees_a_partial_batch`
rejects a store whose batch is **visible** part-written, against
`RowAtATimeStore`, whose rows all land in the end. That is visibility. The rule
still owed is the one where a row does **not** land and the store has to undo the
rest, which is rollback, and it belongs with the fault instruments.

Worth recording as method, because it is the phase's own thesis applied to
itself: *reasoning about which implementations a rule can reject is exactly as
unreliable as reasoning about whether an adapter is conformant.* The retirement
was written by reading the rule; the reversal by writing the store. ADR-0010's
amendment carries both, and a reader who finds only the outcome will re-derive
the merge.

**"A fault-injecting decorator over any `EventStore`" was this clause's own
words, and there is no such thing.** It said so from stage 1 until stage 6 tried
to build it. A decorator sits *above* `append`, which is the unit the port makes
atomic: the only faults it can inject are before the call and after it. The one
shape that looks like it works — append `events[..k]`, then return `Err` — is a
decorator writing a partial batch and then asserting that the **store** should
have undone it, which is a test of the decorator. Nothing an outside caller holds
can reach between two rows of one transaction, and that is not an oversight in
the port; it is the property under test.

So the injection is the **fixture's**, and the rule is capability-gated on
`Fixture::MID_BATCH_FAULT` with `arm_mid_batch_fault(after)` beside it. Every
store that can fail one row does it differently — a trigger that raises on the
third insert, a constraint armed for one write, a connection killed
mid-statement — which is what makes it a capability rather than testkit
machinery, and a store with none declines and the rule reports a skip. It is
defaulted-declined on the trait, unlike `SECOND_HANDLE` and `REOPEN`, because an
in-memory store has no fault to offer and demanding the answer would buy
boilerplate rather than information. `GappedPositionStore` supplies it and rolls
back; `NoTransactionStore` supplies it with the `BEGIN` removed and is the
registry row.

This clause is what makes the ingest decision implementable without a new seam.
Under unconditional ingest with compensation, the receiving side appends
`[losing_event, compensating_event]` as one batch under a guard keyed on the
compensation's own identity — one call, `append(&[losing, superseded],
Some(&guard))`, all-or-nothing by this clause and conformance-tested by
`append_is_atomic` and by
`append_is_atomic_under_a_mid_batch_fault`. No reader
ever observes the losing event unresolved. The half
of that design that needs new machinery is the identity that makes the guard
idempotent, not the atomicity.

#### ES-19 — The returned position

`append` MUST return the position assigned to the **last** event of the batch.
Positions within one batch MUST be assigned in slice order and MUST be strictly
ascending.

**[FROZEN]**

- **Rule:** `append_returns_last_written_position`, plus
  `batch_positions_follow_slice_order` — append a batch of three distinguishable
  events and assert that the positions the store assigned them **strictly
  ascend in slice order**, rather than inferring the order from `all.last()` as
  the existing rule does. It compares positions found by *type* rather than the
  order the read returned them in, so that a store whose defect is the read path
  fails `read_defaults_to_ascending_order` and not this; and it does not
  re-assert the returned position, which is this clause's first sentence and
  `append_returns_last_written_position`'s. Plus the concurrency family's
  `append_returns_the_callers_own_last_position`, which is the only rule that
  separates the two: sequentially a single writer's last event *is* the head, so
  `INSERT …; SELECT max(position)` — what an adapter writes when its driver
  cannot `RETURNING` on a multi-row insert — is indistinguishable from
  `INSERT … RETURNING position` until a second writer exists. That is
  the `Rejects:` shape below, named by the rule that catches it.
- **Cases:** E2E-13, E2E-23.
- **Rejects:** an adapter that returns the store head rather than its own batch's
  last position — identical on a quiescent store and wrong under a second writer
  (E2E-08) — and an adapter whose bulk insert returns generated keys in
  unspecified order.

The contract MUST NOT require a batch's positions to be *contiguous* or free of
interleaving by another writer; gaps are permitted (`event.rs:215-217`) and a store
that allocates outside its transaction cannot promise it. One consequence belongs
in the port's documentation and is stated here as prose rather than as a clause,
because no rule can check a caller: the returned position is for checkpointing and
for reporting. It is **not** a sound `after` for a follow-up condition unless the
caller has read up to it, because a foreign event may hold a position below it
that the caller never saw. `store.rs` offered it for exactly that use — "every
caller that checkpoints a projection **or builds a follow-up append condition**
needs it" — and had to be corrected. **That correction landed in `3c704d3`.** The
`append` doc no longer offers the returned position for a follow-up condition; it
states the refusal and points at the read instead (`store.rs:179-187`). The
obligation stands over every future edit to that doc: the sound `after` comes
from a read — `read_decision_model` (`store.rs:517-527`) — which is what the DCB
loop already does.

#### ES-20 — An empty batch is refused, and refused first

`append(&[], _)` MUST return `AppendError::NoEvents`. The emptiness check MUST
precede the condition check, so that an empty batch under a condition that would
have been violated still returns `NoEvents`.

**[FROZEN]**

The precedence is not arbitrary. `ConditionViolated` is the DCB concurrency
signal and its documented meaning is "rebuild the decision model and retry"
(`error.rs:126-132`). An empty batch is a caller bug: the retry can never succeed,
so reporting the violation puts a correct client into a loop that never
terminates and attributes a code defect to contention. `NoEvents` names the actual
fault.

- **Rule:** `append_rejects_empty_batch` covers only
  `append(&[], None)`, so it cannot see the precedence.
  `empty_batch_is_refused_before_the_condition_is_evaluated` — seed an
  event, then `append(&[], Some(&condition_that_matches_it))`, assert `NoEvents`.
- **Cases:** E2E-06; PRESSURE-TEST §6 S3.
- **Rejects:** `MemoryEventStore` as it stood, which is the sharpest possible
  answer to "can this rule fail anything". It evaluated the condition first and
  returned `ConditionViolated` where `NoEvents` is the caller's actual bug. **A
  rule whose first casualty is the reference implementation is the opposite of
  decorative.**

  D8 is fixed and it had **two** sites, which is itself the argument for the
  rule. `MemoryEventStore` now checks emptiness first and does it *above the
  lock*, because emptiness is a precondition on the argument and nothing behind
  the lock can change the answer; `LocalMemoryEventStore` in the testkit's own
  `tests/` copied the ordering deliberately and said so in a comment, and the
  comment moved with the fix. The mutant registry's correct core copied it a
  third time. Three implementations, one defect, and every non-empty batch made
  them indistinguishable — which is how it survived. `ConditionBeforeEmptinessStore`
  is the old order, kept as the mutant that fails the rule.

#### ES-21 — A batch's own events are not evaluated against its own condition

A condition MUST be evaluated only against events the store already held. Events
in the batch being appended MUST NOT be considered.

**[FROZEN]**

- **Rule:** `batch_is_not_evaluated_against_its_own_condition` — a two-event
  batch under a condition whose query matches both, `after` set past everything
  stored, asserting success and both events landing. It closes with the same
  batch reissued under the same condition, which must now be refused: without
  that mirror the rule's whole content is `is_ok()`, which a store whose probe
  answers `None` to everything satisfies for free.
- **Cases:** E2E-06.
- **Rejects:** the conditional `INSERT ... SELECT ... WHERE NOT EXISTS` strategy
  applied per row, which the decision ledger carries as a live candidate
  (`RUNBOOK.md:2870-2876`). It self-rejects on any single-event batch whose condition
  names the type it is appending — the shape of `RegisterChargePoint`,
  `ClaimPooledUnit`, `PlaceHold` and `ReserveTourAllocation` across four
  scenarios — and on any multi-event batch whose second event matches. The
  reference answer is "no" only by accident of implementation order
  (`memory.rs:372-388` checks `stored` before extending). `PerRowConditionStore`
  in the testkit's own `tests/` is the compiled version: it carries the condition
  with every row and **rolls back** on rejection, which is the point of the shape
  rather than a detail — a store that self-rejected and kept the rows it had
  written would fail `append_is_atomic` instead, and the registry could not then
  say which of the two defects that rule catches. `WriteThenCheckStore` fails it
  from the other side: writing before deciding puts the batch into the set its own
  probe reads.

#### ES-22 — Dropping an `append` future leaves no partial batch

If an `append` future is dropped before completion, the store MUST be in one of
the two states ES-18 permits: fully applied, or unchanged. It MUST NOT be
partially applied.

**[FROZEN]**

- **Rule:** `dropped_append_future_leaves_no_partial_batch` — build the future,
  poll it once, drop it, then read the whole store and assert the batch is
  present in full or absent in full. The single poll is load-bearing: an
  `async fn` body runs nothing until its first `poll`, so a future that is built
  and dropped has provably done nothing and the rule would assert over an
  untouched store.
- **Cases:** E2E-07.
- **Rejects:** an adapter that executes a batch as several statements outside a
  transaction and relies on running to completion. `YieldingRowAtATimeStore` in
  the testkit's own `tests/` is that adapter — one `INSERT` per row, awaited,
  with the `BEGIN` forgotten — and it is **not** a rename of `RowAtATimeStore`,
  which `a_concurrent_reader_never_sees_a_partial_batch` owns and whose rows all
  land in the end: that one tests the *visibility* of a batch mid-flight, this
  one tests rows that never land at all. Nor is it `NoTransactionStore`, which
  needs a fault *armed* and answers `Err` over a partial log; here nobody
  answers anything, because a dropped future produces no `Result`.
  `MemoryEventStore` passes trivially, because `memory.rs:338-403` contains no
  `.await` at all — which is exactly why the reference store cannot answer this
  question and a real adapter must.

#### ES-23 — Cancellation outcome is unspecified, and that is the contract

An adapter MAY commit an append whose future was dropped. A caller MUST NOT treat
a dropped future as evidence that the append did not commit. The port MUST
document this in an explicit `# Cancellation` section, and each adapter MUST state
which of the two it does.

**[FROZEN]**

The honest paragraph, since silence is what the pressure test calls the single
most under-specified thing in the repository. At the edge, dropping the future is
the *normal* termination path: a client disconnect, a CPU limit, a Durable Object
eviction, a pod eviction. `AppendError` has four variants and none means "the
outcome is unknown" (`error.rs:212-249`) — and adding one would not help, because
a dropped future produces no `Result` at all. There is nowhere for the value to
go. So the contract's statement is not an error variant; it is a documented
absence of a promise, plus ES-22 bounding what the absence can cost, plus the
recovery mechanism deferred to ES-24.

- **Rule:** none. The adapter-visible half is ES-22; this clause's remaining
  content constrains callers and adapter documentation, which no conformance rule
  can observe. It is stated as a clause rather than as prose only because
  "unspecified" is itself the normative content and an adapter author must read
  it as a requirement to document, not as an omission to fill in silently.
- **Cases:** E2E-07.
- **Rejects:** a pooled rusqlite adapter that does its work in `spawn_blocking`
  and presents itself as cancellation-safe. A dropped `JoinHandle` does not cancel
  the closure: the `COMMIT` executes, the caller is told nothing, an operator
  retries, and the payment is issued twice.

#### ES-24 — Resolving an unknown outcome

**[FROZEN]**

**A conditional append is at-most-once under verbatim reissue, and the contract
MUST say so.** Where the events being appended are themselves matched by the
condition's query, reissuing the identical batch after a dropped future resolves
the outcome: `ConditionViolated` means the first attempt landed, `Ok` means it had
not and now has. Either way the store holds exactly one copy. The caller needs no
identity, no idempotency key and no new API — the guarantee falls out of the
condition it already wrote.

**The contract MUST equally state where this does not hold**, because a guarantee
whose limits are unstated is read as universal:

- An **unconditional** append has no such property. Reissuing it appends a second
  copy, and nothing in the port can prevent that.
- A **conditional** append whose query does not match its own events has no such
  property either — conditioning on `CourseCapacityChanged` while appending
  `StudentSubscribed` leaves the retry indistinguishable from a first attempt.
  This is the common shape in a decision that reads one thing and writes another,
  so it is not an exotic case.
- The reissue MUST be **verbatim**. This is not the `ConditionViolated` re-decide
  path, where the correct response is to re-read, rebuild the decision model and
  produce different events. Collapsing the two is the mistake this clause exists
  to prevent: one says "your write already happened", the other says "the world
  moved, decide again", and they arrive as the same error value.

Callers who need at-most-once for the second and third shapes must supply their own
dedup in the domain — a natural key in the tags is the mechanism, and it is
queryable, which an identity in `metadata` is not.

The unknown outcome of ES-23 is resolvable only if a caller can ask "did my event
land?" without knowing a position. Today it cannot: `Event` is
`(event_type, data, tags, metadata)` (`event.rs:321-326`) and carries no identity,
`append` has no slot for a caller-supplied one (`store.rs:261-265`), and a query
matches on type and tags only (`query.rs:113-116`), so an identity in `metadata`
is structurally unqueryable and breaks ADR-0003 at the point ADR-0003 claims to
win.

- **Rule:** `reissued_conditional_batch_lands_once` — append a batch under a
  condition its own events match, append the identical batch again, assert the
  second attempt is **refused** and the store holds one copy. Refused rather
  than `ConditionViolated` specifically: which error a rejection is reported as
  is ES-25's MUST and `condition_rejection_is_reported_as_condition_violated`
  owns it, so demanding the discriminant here would make
  `ViolationAsStoreErrorStore` fail a rule about reissue for a reason that has
  nothing to do with reissue. Writable today: it needs no identity, which is
  what makes this clause severable from `EventId` and is why the deferral it
  carried was larger than the question.
  Two companions pin the two stated limits, so that an adapter cannot quietly
  strengthen the guarantee and leave callers depending on behaviour the contract
  disclaims. `reissued_unconditional_batch_lands_twice` pins the first;
  `reissued_batch_conditioned_on_other_events_lands_twice` pins the second,
  which this clause had stated in prose and given no rule since it was written —
  and which is the *common* shape, a decision that reads one thing and writes
  another. Both make duplicate-landing a MUST, which makes a store with natural
  payload dedup non-conformant; that is deliberate, because a contract whose
  idempotency varies silently by adapter is worse than one with none.
- **Cases:** E2E-07, E2E-33, E2E-36.
- **Rejects:** a caller-supplied dedup key written into `Event::metadata`
  (`event.rs:377-382`) — and equally the naive fix of promoting identity to a
  tag: maximal cardinality in the column adapters are told to index, an entry
  in every `contains_all` merge-scan (`tag.rs:347-361`), a writer-forgeable
  identity, and an identity dimension visible to every tag-only query. The
  metadata half used to point at `happenstance-sync`'s own proposal; that
  proposal was withdrawn in favour of `(StoreId, SequencePosition)`
  (`crates/happenstance-sync/src/identity.rs:87-101`), and what replaces it is
  not a proposal but the shape the tree leaves open — `metadata` is the only
  slot on `Event` that no store, query or peer is permitted to look inside,
  which is exactly what makes it the tempting one.

---

### 3.4 Append conditions

The shape on disk since commit `1b2a565` is `pub struct AppendCondition { guards:
Box<[Guard]> }` with `pub struct Guard { pub query: Query, pub after:
Option<SequencePosition> }` (`append.rs:102-119`, `:130-141`). VT-30 landed at
phase 4.

VT-30 replaced the single `fail_if_events_match` / `after` pair with that
non-empty sequence of guards, each a `(Query, Option<SequencePosition>)` pair,
and that clause is still `[PROVISIONAL]`. Every clause
in this subsection is written for one guard and generalises to N by conjunction:
the append is rejected if **any** guard is violated, and each guard is evaluated
by exactly the rules stated here. Nothing below depends on there being only one,
and the existing `condition_after_*` rules become the single-guard case rather
than being rewritten — which is why VT-30 can stay provisional under clauses that
are frozen.

#### ES-25 — Condition semantics

The store MUST reject the append if and only if it holds at least one event
matching a guard's query at a position strictly greater than that guard's
`after`. With `after: None` any match at all MUST reject. A rejection MUST be
reported as `AppendError::ConditionViolated`, never as `AppendError::Store`. A
`ConditionViolated` MUST be returned only when nothing was written.

**[FROZEN]**

- **Rule:** `condition_without_after_rejects_any_match`,
  `condition_without_after_allows_non_match`,
  `condition_after_ignores_non_matching_events`,
  `condition_rejection_is_reported_as_condition_violated`,
  `condition_rejection_leaves_store_unchanged`,
  `condition_after_beyond_the_last_matching_position_admits_the_append`
  — matching events at and below `after`, non-matching events above it,
  asserting the append is admitted. This is the "if and only if" seen from the
  side nothing covers, and the case the DCB specification singles out in a Note:
  `condition_after_ignores_non_matching_events` (`:512`) reaches it only with the
  matching set empty, so an adapter can pass every rule in the suite without ever
  comparing a *matching* event's position against `after`.
  Plus `racing_conditional_appends_elect_one_winner`, which is this clause's
  sentence seen as two decisions taken from one snapshot: the second append's
  `after` sits below an event the first one wrote and matching its own query, so
  the rejection is compulsory and exactly one batch may land. ES-27 disposed of
  it and the disposition is reversed there; it belongs here because what it pins
  is the *iff*, not the tag algebra. Two of the three mutants it rejects are the
  shapes named below. The concurrency family's
  `exactly_one_of_n_contenders_commits` is the same question asked of a store
  that is genuinely contended rather than called twice in a row, and it belongs
  to this clause for the same reason: with one caller at a time the probe and the
  insert are adjacent, so the sequential rule cannot separate an atomic
  check-and-write from a probe followed by an insert. `RacingProbeStore` is the
  store that passes every sequential rule and fails this one.
- **Cases:** E2E-08, E2E-55, E2E-39.
- **Rejects:** an adapter that folds the violation into its own error type, which
  destroys the caller's only means of telling "retry the decision" from "something
  broke" without pattern-matching on strings; and an optimistic adapter that
  inserts, then probes, then reports the violation without rolling back. And an
  adapter whose probe ANDs two uncorrelated predicates — *does any event match the
  query* and *is the store's head above `after`* — which is what the check becomes
  when the existence test and the position test are written as separate
  subqueries. It is correct on every case the suite exercises, and it rejects
  every command whose caller read to a boundary above the last event touching its
  own entity: the steady state of a quiet entity in a busy store, reported as
  contention. That last one is `UncorrelatedProbeStore` in the mutant registry,
  and it is the reason the new rule exists rather than the other way round:
  `condition_after_ignores_non_matching_events` reaches the same shape only with
  the matching set *empty*, where the two probes agree.

`ConditionViolated::conflicting_position` (`error.rs:136-147`) is informational.
An adapter that detects the conflict without learning which event caused it — a
conditional insert — MUST be permitted to report `None`, and callers MUST NOT
depend on it. This is why idempotent bulk ingest cannot be built out of one
condition over many identities: one already-seen event rejects the batch and names
at most one culprit, so recovery is a serial peel (E2E-36).

#### ES-26 — The AC3 boundary: `after` is exclusive, `from` is inclusive

`AppendCondition::after` MUST be **exclusive**: an event at exactly `after` MUST
NOT reject the append. `ReadOptions::from` MUST be **inclusive** (ES-8). The
asymmetry is deliberate and MUST be documented as such.

**[FROZEN]**

The two fields answer different questions. `after` means "everything I did not
see": the caller *did* see the event at `after`, so it cannot invalidate a
decision the caller made with it in hand. `from` means "start where I stopped":
resume wants to name a position the caller holds without doing arithmetic on an
opaque ordering key. Given that reading, the DCB specification's "ignore the
events *before* the specified position" is a lower bound on what must be ignored
rather than a contradiction — underspecified upstream, settled here.

The pairing that falls out is exact and requires no arithmetic:
`read_decision_model` returns the maximum position observed and
`AppendCondition::after_opt` consumes it (`store.rs:517-527`, `append.rs:191-212`).
The pairing that does *not* fall out is the checkpoint resume path, where
`ProjectionStore::checkpoint` returns an inclusive-consumed position and
`ReadOptions::from` is inclusive, so the caller must advance by hand through
`SequencePosition::next()` (`projection.rs:101-110`) — a method whose
documentation said, until phase 4, that it was meaningful only on
densely-allocating adapters, and now states the opposite: it is the resume idiom
on **every** adapter, sound over gaps because `from` is a threshold rather than a
seek (`event.rs:259-271`, VT-13). PS-20 settles the off-by-one — resume strictly
after the checkpoint, inclusively from the store's first position when
`NeverRun` — and VT-13 fixes `next()` so the idiom is sound over gaps; it is
named here because it is the same boundary seen from the other side.

- **Rule:** `condition_after_ignores_events_at_the_boundary`,
  `condition_after_rejects_events_beyond_the_boundary`,
  `read_from_is_inclusive`.
- **Cases:** E2E-56, E2E-10; PRESSURE-TEST §6 S10.
- **Rejects:** an adapter that writes `position >= after` in its probe. It rejects
  the caller's own last-seen event, so every command on a busy boundary fails on
  its first attempt and the deployment reads it as contention. And once ingest
  exists the same off-by-one in the other direction is a **silent lost update**,
  which is why this clause is stated rather than assumed.

#### ES-27 — A condition matches on tags, not only on types

A condition's query MUST be evaluated by exactly the same rules as a read's query:
types OR within an item, tags AND within an item with superset matching, items OR
across the query.

**[FROZEN]**

- **Rule:** `condition_matches_on_tags` — two events sharing a type and
  differing in tags, a condition tagged for one of them, asserting rejection; and
  `condition_with_an_unheld_tag_does_not_reject`, its mirror, asserting
  acceptance for a tag no event carries. CF-7 and CF-8 are the obligations to
  have the pair, and the pair is not redundant: the first rejects a probe that
  compares serialised tags with `=` (which is why the event it names carries an
  *extra* tag), the second a probe that drops the tag join entirely. Each of the
  two wrong probes passes the other's rule. The read-side statement of the same
  semantics is pinned by
  `query_item_types_are_or`, `query_item_tags_are_and` and
  `query_item_combines_types_and_tags_with_and` — this
  clause's content is that the *condition* path is evaluated by those same rules,
  which no existing rule checks.
- **Cases:** E2E-55, E2E-03; PRESSURE-TEST §3.9.

**`racing_conditional_appends_elect_one_winner` was retired here, and the
retirement is reversed — the rule is retained and ES-25 claims it.** The
paragraph in this position said the rule was replaced by this clause's rule and
CF-7's, on the reasoning that its one tagged condition runs against a store where
a type-only probe returns an identical verdict, so it cannot observe *this*
clause's property, and that being sequential and single-handle it cannot observe
ES-34's either. Both halves are still true, and neither is a reason to delete a
rule. What the disposition confused is "this clause is not the rule's owner" with
"no clause is": what the rule actually pins is ES-25's *if and only if* seen as
two decisions taken from one snapshot — the second append holds a matching event
strictly above its `after`, so it MUST be rejected, and exactly one batch may
land. Three registry rows fail it and two of them are shapes ES-25's own
`Rejects:` names by hand: `WriteThenCheckStore` (insert, then probe, then report
without rolling back), `ViolationAsStoreErrorStore` (the violation folded into
the adapter's own error type) and `AfterIsAnOffsetStore`.

The lesson is ES-18's, met for the third time in one phase: a disposition written
by reading a rule is a hypothesis, and the evidence that settles it is a compiled
store. All three of this specification's retirements were written the same way
and two of the three were wrong.

- **Rejects:** an adapter that drops the tag join from its condition probe. This
  is the natural first cut, because the join is the expensive half and the landed
  SQLite schema puts tags in a separate table
  (`crates/happenstance-sqlite/src/event_store.rs:62-67`) — measured at roughly
  200x a single-tag boundary's cost at 50,000 events, which is why ADR-0022
  ships `tag_cardinality` and most-selective-tag-first probing as requirements.
  Until this clause's
  rules landed, **no rule's verdict depended on the condition path matching
  tags**: every append-condition rule built its condition from `query_of_types`,
  with two exceptions and neither closed the gap —
  `racing_conditional_appends_elect_one_winner` ran its tagged condition against
  a store where a type-only probe returns identical verdicts, and
  `two_handles_observe_each_others_appends` (CF-19) ran one where the property
  under test is *visibility across handles* rather than matching. The adapter
  that drops the join therefore passed the whole suite while rejecting every
  command that touches any tagged entity: a total-availability failure certified
  as conformant, on the canonical DCB uniqueness shape.
  `condition_matches_on_tags` and `condition_with_an_unheld_tag_does_not_reject`
  close it as a pair, and `TagBlindConditionStore` is the compiled adapter.

#### ES-28 — Degenerate condition inputs

A condition evaluated against an empty store MUST NOT reject. A condition whose
`after` names a position at or beyond the store's head MUST NOT reject and MUST
NOT error.

**[FROZEN]**

- **Rule:** `condition_against_an_empty_store_admits_the_append` and
  `condition_after_beyond_head_admits_the_append`. No rule evaluated a
  condition against an empty store before phase 3. The *at* half of "at or beyond" needs no
  rule of its own, and this is worth stating so nobody writes one:
  `condition_after_ignores_events_at_the_boundary` sets
  `after` to the position of the only event in the store, which is the head, so
  it already runs that case. A second rule over it would be one no adapter could
  fail — CLAUDE.md's first corollary — and the decorative rule is the one that
  survives longest, because nothing ever goes red to draw attention to it.
- **Cases:** E2E-56, E2E-08.
- **Rejects:** an `EXISTS`-probe adapter whose SQL returns a NULL the code reads
  as true on an empty table, and an adapter that validates `after` against its own
  head and errors on a position it has not assigned — plausible, defensible, and
  fatal to a peer resuming after a gap. Both are compiled and registered:
  `NullAggregateProbeStore` and `AfterValidatedAgainstHeadStore`. The second is
  the reason the anchor in the rule is `head + 1` derived from a position the
  store assigned rather than a literal — on a store allocating in steps, *every*
  position between two assigned ones is one it never assigned, so the case is the
  norm rather than the edge.

#### ES-29 — `after` is store-local

A `SequencePosition` is meaningful only within the store that assigned it. A store
MUST NOT evaluate an `AppendCondition` whose `after` was assigned by a different
store.

**[FROZEN]**

`AppendCondition` has a **hand-written** `Serialize` behind the `serde` feature —
not a derive: the impl mirrors the type through a private `Wire`/`GuardWire` pair
(`append.rs:286-297`, `:299-313`) — and puts `after` on the wire as a naked
integer (`append.rs:286-291`, `event.rs:722-726`), which makes the single most
dangerous replication mistake the path of least resistance. At a receiver,
`after: 288455` names an unrelated recent event, `is_violated_by` compares raw
values (`append.rs:239-253`), and the check runs over an arbitrary tail and passes
**vacuously** — worse than being rejected, because it looks like enforcement. Five
of six scenarios reached this independently.

The wire format is private to happenstance (there is no obligation to interoperate
with other DCB implementations), so the envelope is free to fix it. §2 has: WF-4
makes `after` **always present** on the wire precisely so an ingest policy can see
it, and SY-6 makes a wire condition carrying `after: Some(_)` a refusal at ingest
rather than something to evaluate. The condition therefore travels as evidence and
never as an instruction, and no receiver ever needs to interpret a foreign
position. DCB wire interoperability is explicitly deferred by WF-1 and its future
home is a separate `happenstance-dcb-interop` crate, so that deferring it costs
the private format nothing.

- **Rule:** `wire_condition_with_after_is_refused` **(new)**, in
  `happenstance-sync-testkit` — SY-6's rule, named once there and cited here; the
  crate is named at `crates/happenstance-sync/src/lib.rs:23-24` and does not exist.
- **Cases:** E2E-37, E2E-38, E2E-56.
- **Rejects:** a hub that deserialises a peer's condition and evaluates it
  verbatim. The readable-but-not-constructible property is `Guard`'s rather than
  `AppendCondition`'s, and VT-30 is why: `Guard` is `#[non_exhaustive]` with
  **public** fields (`append.rs:121-141`), while `AppendCondition`'s own `guards`
  field is **private** and reachable only through `guards()`
  (`append.rs:102-119`, `:179-183`). Either way the refusal is checkable today:
  readable but not literal-constructible is exactly what makes an ingest-side
  policy possible.

The position-free form is already the default — `AppendCondition::new` leaves
`after: None` and `after`/`after_opt` are opt-in builders (`append.rs:185-212`) — so
the API already makes the replicable shape the easy one. The crate's only read
helper, `read_decision_model`, exists to produce the *position-relative* form and
has no counterpart for the replicable one; if the sync design lands on
"conditions must be position-free to replicate", that helper is pointing the wrong
way, and this specification records it rather than inverting it silently.

---

### 3.5 Read-side operations

Three questions, one of which the evidence base got wrong. The claim that
`#[trait_variant::make]` forbids defaulted methods was compiled and refuted
(PRESSURE-TEST §1, and ES-4 above gives the mechanism). `head()` and `count()`
were never blocked on anything.

#### ES-30 — `head()` is a required method on the port

`EventStore` MUST declare
`async fn head(&self) -> Result<Option<SequencePosition>, Self::Error>`, returning
the highest position currently visible in the store, or `None` when the store
holds nothing. It MUST be a **required** method, not a provided one.

**[FROZEN]**

Required rather than provided, and the reason is ES-3 rather than taste. The
only provided form that survives the clone into the variant needs `where Self:
Sync` (ES-4). A single-threaded on-device store — a `RefCell` in a Durable
Object, an `Rc`-shared cursor — is `!Sync`, and that adapter is the entire
reason the bare flavour exists. A provided method the edge adapter cannot call
is a method the port does not have. The cost of "required" is seven impls
today, four of them skeletons — `happenstance-sqlite` was the fifth until phase
8 gave it real bodies and a green suite: `memory.rs:293`,
`crates/happenstance-testkit/tests/local_conformance.rs:198`,
`crates/happenstance-sqlite/src/event_store.rs:1106`,
`crates/happenstance-postgres/src/event_store.rs:121`,
`crates/happenstance-cloudflare/src/event_store.rs:992` and
`crates/happenstance-neon/src/event_store.rs:168`, `:405`. The blanket impl
forwards it for free (`variant.rs:194-237`), so generic code pays nothing and
only implementers do — two when this clause was written, seven now, and seven
is the figure a required method has to be re-costed against. The count is of
impls meant to be conformant. Three more exist and are excluded on that
criterion: `BorrowHoldingStore` and `AwaitAcrossBorrowStore`, the two mutants in
`crates/happenstance-testkit/tests/local_conformance.rs`'s `mutants` module,
which are written to fail, and `SendStoreWithLocalError`
(`crates/happenstance-cloudflare/src/send_shape.rs:137`), which exists to be
rejected by a bound.

`head` is deliberately **not** parameterised by a query. A narrow projection's
problem is that it cannot advance past events it examined and did not match; the
global head is what lets it checkpoint past them. A query-scoped head would
reintroduce exactly the cost the method exists to avoid.

- **Rule:** `head_of_an_empty_store_is_none`,
  `head_is_the_highest_visible_position` (append a batch holding at least one
  event a narrower default query would not match, read the whole store with
  `Query::all()`, and assert `head()` is not below the highest position the read
  yielded), `head_advances_across_two_handles` (depends on ES-33).
- **Cases:** E2E-13, E2E-02, E2E-25.
- **Rejects:** three implementations that all compile. A cached last-written
  position, which is stale the moment a second handle writes (E2E-08). A `head`
  that reports the highest position matching some default query rather than the
  store's head. And the workaround as a universal answer —
  `read(&Query::all(), ReadOptions::new().backwards().limit(1))` is one cheap
  statement on local SQLite and one full HTTP round trip on the adapter with the
  smallest latency budget in the system; an adapter that implements it by
  materialising all matches before truncating fails E2E-13 outright, while
  `MemoryEventStore` reverses before truncating (`memory.rs:312-332`) and passes.

**`count()` does not ship, and this is a decision rather than an omission.** A
count has no consumer among the fifty-six cases; `SequencePosition` is explicitly
not a count (`event.rs:215-217`), so a count cannot be derived from one and cannot
be used as a bound; and on a paginating transport a count is the one read-side
value with no bounded implementation. An adapter that can produce one cheaply
should expose it as an inherent method, where it costs no other adapter anything.
This paragraph is prose because the absence of a method is not checkable by any
rule.

#### ES-31 — "Am I caught up?" is a position comparison

Generic code determining whether a consumer is current MUST compare its
checkpoint position with `head()` for equality or ordering. It MUST NOT compute a
difference between two positions and interpret it as a count of outstanding
events.

**[FROZEN]**

- **Rule:** `checkpoint_lag_is_not_a_position_difference` **(new)**, in the
  projection suite — run a projection to completion against a sparsely-allocating
  fixture and assert it reports "caught up" when `checkpoint == head`, whatever
  the numeric distance from the previous position.
- **Cases:** E2E-13, E2E-23, E2E-25.
- **Rejects:** a runner or a dashboard computing `head - checkpoint` as an event
  lag. It reads plausibly on a dense store, and on a store with gaps — every
  Postgres adapter, every store that has ever rolled back a transaction — it
  reports a backlog that does not exist and pages someone.

#### ES-32 — No tail or subscription seam at 0.1

`EventStore` MUST NOT grow a tail, subscribe or notify method at 0.1. Consumers
poll.

**[PROVISIONAL — falsified if the fan-out runner of E2E-32 cannot hold N views
within their staleness budget at a measured poll interval on a real deployment.
The named measurement is the projection-runner benchmark that `RUNBOOK.md:3953-3955`'s
phase owes; the workspace has no benchmark harness and a conformance rule cannot
substitute for one, because complexity is a benchmark and not an assertion.]**

`RUNBOOK.md:494` owns this row and states the stake correctly: it changes the
*port*, and adding a required method once adapters exist breaks every one of
them. So it is answered here rather than deferred past the freeze.

Polling, and the reasons in order. A subscription is a push seam, and three of the
named deployment shapes cannot hold a subscriber open across the boundary where
the caller does not exist: one-shot HTTP with no connection, a Durable Object whose
execution is request-scoped, and an on-device store woken by a sync. A port method
that those adapters must implement by polling internally is a method that lies
about its cost, and it lies in the direction that makes the caller stop budgeting
for it. A store that genuinely can push — Postgres `LISTEN`/`NOTIFY`, a DO alarm —
loses nothing: it exposes an inherent method, or a separate `TailingEventStore`
sub-trait that composes without touching any existing adapter. That asymmetry is
the whole argument: adding the capability later is additive, while removing it
later is not. And the N-views cost that motivates the question is answered by the
*runner* — one read of the union query, twenty local re-filters — which is why
ES-15 pins the algebra that runner depends on.

- **Rule:** none for the absence. The checkable substitutes are ES-30 (a poll
  costs one bounded call) and ES-15 (the fan-out runner's algebra holds).
- **Cases:** E2E-32, E2E-28.
- **Rejects:** the shape this deferral would otherwise take — a `subscribe`
  method added in 0.2 that every existing adapter must implement, three of them by
  polling behind a name that says they do not.

---

### 3.6 Multi-writer, re-entrancy and durability

Everything in this subsection was unreachable until phase 3, and for one shared
reason. `event_store_conformance!` hoisted its `factory =` expression behind a
`fn __conformance_store()` that re-evaluated it on every call, and each rule
called that function exactly once. The signature `F: Fn() -> S` *permitted* two
handles onto one backing store and nothing *asked* for it, so durability, reopen
and genuine multi-connection rules were foreclosed by the fixture shape rather
than by any decision. ES-33 is therefore a precondition on the other three, and
it is the one that has since been discharged.

What replaced it is CF-15's `Fixture` trait
(`crates/happenstance-testkit/src/contract.rs:108-207`), which names the two
operations apart: one fixture instance is one isolated backing store, and each
`connect()` on it returns one handle onto that store. A rule is handed
`impl AsyncFn() -> F` — *how to make a fixture*, not a made one — so the rule
rather than the emitter decides how many instances it needs
(`crates/happenstance-testkit/src/registry.rs:28-59`).

#### ES-33 — The fixture yields handles onto one backing store

The conformance fixture MUST distinguish "a fresh backing store" from "a handle
onto it". It MUST create a fresh, empty backing store once per rule, and MUST be
able to yield two or more handles onto that same backing store within one rule.
An adapter MUST supply both.

**[FROZEN]**

This was discharged at phase 3. `$factory: Fn() -> S` became a two-level fixture
— a value created per rule, with a `connect()` returning an `S` — because "call
the factory twice" and "get two handles onto one store" cannot both be true of
one closure that also has to produce an empty store. CF-15 owns the fixture
contract and made it a trait; `connect()` is its spelling and this clause uses
it. `MemoryEventStore` needed no change in the end: `MemoryFixture` holds it
behind an `Arc` and `MemoryHandle` is the clone, which is the same move one level
out. A file-backed adapter points both handles at one temporary path; a pooled
adapter hands out two pool members. Both remain unwritten, which is why §6.5
records this axis as having a fixture instrument and no adapter one.

- **Rule:** this is a conformance obligation on the testkit rather than on an
  adapter's behaviour; it is the enabling condition for
  `two_handles_observe_each_others_appends` and
  `acknowledged_writes_survive_a_reopen`, both of which landed at phase 3 stage
  2, and for `head_advances_across_two_handles`, which landed at phase 4 with
  `head()` itself. It is stated as a clause because the suite could not grow any
  of the three until it landed.
- **Cases:** E2E-08, E2E-46; PRESSURE-TEST §6 S6.
- **Rejects:** an adapter whose "second handle" is a clone of the first that
  shares one connection and one cache — which is how a pooled adapter would
  accidentally satisfy the letter of the fixture while defeating every rule built
  on it. The fixture's own documentation must require independent handles, and
  `two_handles_observe_each_others_appends` is what detects the cheat.

#### ES-34 — Two handles share one consistency boundary

An append made through one handle MUST be visible to a condition evaluated
through another handle onto the same backing store.

**[FROZEN]**

The clause names the boundary and the rule names the observation, and that is the
right way round. `two_handles_share_one_consistency_boundary` was this clause's
own first spelling and CF-16's was the other; CF-16's wins, because an implementer
checks a rule against what it *does*, and what this one does is append through one
handle and see the append through a second. A shared boundary is the property; two
handles observing each other's appends is the evidence for it.

- **Rule:** `two_handles_observe_each_others_appends` — handle *A*
  appends an event matching a condition, handle *B* appends under that condition
  with `after` set before *A*'s write, asserting `ConditionViolated`.
- **Cases:** E2E-08.
- **Rejects:** three adapters that passed all twenty-seven rules the suite had
  before CF-19 landed, and that `CachedHeadFixture` in the testkit's own `tests/`
  now models for the first of them. A cached
  `max(position)` fast path — a strategy the ledger explicitly defers rather than
  rules out (`RUNBOOK.md:1786-1790`). A per-connection repeatable-read snapshot, where
  the probe is correct only within its own session. An advisory lock scoped to a
  single pool member. All three are correct single-handle and wrong for any
  deployment where one store is reached two ways, which is every deployment with a
  connection pool.
  `racing_conditional_appends_elect_one_winner` does not
  reach them: it is sequential *and* single-handle.

#### ES-35 — Acknowledged writes survive a reopen, where durability is claimed

An adapter that claims durability MUST make every append that returned `Ok`
readable after the backing store is closed and reopened. Durability is a declared
capability: an adapter that does not claim it MUST document that it does not, and
MUST NOT be presented as an event store of record.

**[PROVISIONAL — axis: **durability**. The question became expressible at CF-17: `Fixture::REOPEN` and `acknowledged_writes_survive_a_reopen` exist, and `DurableFixture` in the testkit's own `tests/` supplies the capability, so a store that returns `Ok` from `append` and loses the write now fails a named rule rather than passing everything. The first file-backed adapter has now answered *half* of it: `happenstance-sqlite`'s `SqliteFixture` closes every connection and checkpoints the write-ahead log, and all three reopen rules run and pass against a real file on disk. What remains unbuilt is the **fault** far end, and it is narrower than "the adapter far end" was: a store that loses a write to a *fault* — a process killed mid-commit, a disk that lied about `fsync` — rather than to an instruction. `SqliteFixture` declines `MID_BATCH_FAULT` **by scope, not by incapacity**, and says so in its own words. Falsified by an adapter fixture that arms a real fault against a real medium and observes what survives.]**

Opt-in rather than universal, because `MemoryEventStore` must keep passing and is
by construction not durable (`memory.rs:14-33`). A capability that the reference
implementation cannot have is a capability the base suite cannot require.

- **Rule:** `acknowledged_writes_survive_a_reopen`, using ES-33's
  fixture to close every handle and reopen the backing store. It is
  capability-gated by CF-18's mechanism — an associated `const` on the fixture
  plus a non-empty reason string — and **not** by a second macro an adapter may
  decline to invoke. The distinction is CF-18's whole point: a rule that does not
  appear in the test binary is indistinguishable in CI output from a rule that
  passed, so a volatile adapter must emit this rule as a reported skip rather
  than as nothing at all.
- **Cases:** E2E-46 group; PRESSURE-TEST §6 S6, §7.
- **Rejects:** an adapter that acknowledges before the write is durable — rusqlite
  with `synchronous = OFF`, a Durable Object handler that returns before
  `storage.put` resolves, a Postgres adapter with `synchronous_commit = off`.
  Nothing in the workspace could express the question until CF-17: the rule, the
  `REOPEN` capability and `DurableFixture` landed together, and `LosingFixture`
  in the mutant registry is a store that returns `Ok` from `append` and loses the
  write, failing a named rule rather than passing everything. **No fixture in
  that binary has a medium outside the process**, and that is still true of it:
  `LosingFixture` and `RestampingFixture` both **model** their defect rather
  than testing it, and a store that survives `reopen` there has survived a
  pointer swap. What has changed is the sentence that used to follow — *the axis
  has an instrument at one end and nothing at the other*. It has an adapter at
  the other end now: `happenstance-sqlite`'s `SqliteFixture` closes every
  `rusqlite::Connection`, checkpoints the write-ahead log with `TRUNCATE` so a
  commit that lives in the `-wal` sidecar is folded into the file, and never
  deletes or recreates it, and all three reopen rules run and pass across that
  boundary. What is *still* missing is narrower and is named in the marker
  above: the **fault** end, not the adapter end.

#### ES-36 — Two `append` futures on one `&self` interleave safely

Two `append` futures created from one store handle and polled alternately to
completion MUST both complete. Under mutually violating conditions exactly one
MUST succeed and the other MUST return `ConditionViolated`. Neither MUST panic.

**[FROZEN]**

- **Rule:** `interleaved_appends_on_one_handle_elect_one_winner` — two `append`
  futures created from one handle, each polled once before either is allowed to
  finish, then drained — and
  `a_live_read_stream_does_not_block_an_append`, the read/write
  pairing of the same property: build a `read` stream, do not drain it, append
  through the same handle, and require the append to complete. ES-11's
  stability rule performs exactly that choreography and asserts what the stream
  *yields*; against a store that cannot be re-entered it never reaches its
  assertion, and a hung test reports a timeout rather than a violation. This
  rule is what separates "the store cannot answer the question" from "the store
  answers it wrongly".

  This clause was drafted naming `tokio::join!` on a `current_thread` runtime,
  which is what the two tests it was drafted from used —
  `tests/local_conformance.rs`'s `reentrancy` module, now deleted, since a rule
  in the suite runs against that store under all four of its harnesses. The
  spelling changed and the choreography did not: the suite must run under an
  emitter that has **no runtime at all** (`block_on`), so a rule needing `tokio`
  would be a rule one shipped harness could not carry. Hand-polling reaches the
  same state — both futures exist, each has been entered, neither has finished.
  The rule also declines to say *which* of the two wins, and counts instead;
  fixing the winner would be asserting a scheduling order the contract does not
  give.
- **Cases:** E2E-09.
- **Rejects:** a `RefCell`-backed adapter that holds its borrow across an awaited
  storage call, which panics at runtime — `AwaitAcrossBorrowStore` in the mutant
  registry; and, more subtly, one that drops the
  borrow around the await and therefore leaves an unspecified window between the
  condition probe and the write, which the winner assertion catches —
  `PreCommitPositionStore`, which is registered against this rule as well as
  against ES-10's, because the window is one defect seen twice. This is the
  rule a Durable Object adapter would actually fail, and `MemoryEventStore` cannot
  surface it: `memory.rs` holds no lock across a suspension point because
  it has no suspension point. The same borrow taken by a *lazily streaming* read
  is what the second rule catches — a rusqlite adapter yielding rows from a live
  statement, or an `Rc`-shared cursor, each holding the handle for as long as the
  caller holds the stream; `BorrowHoldingStore` is that shape.
  `MemoryEventStore` survives it only by filtering,
  ordering and truncating under the read lock and streaming from the resulting
  `Vec`, so its stream holds nothing at all.

  **One asymmetry belongs in the clause rather than only in the registry.** For a
  `RefCell` store the failure is a panic; for a pooled SQL adapter holding one
  connection the identical defect is a **deadlock**. It is the only entry in the
  workspace's instrument catalogue where a real adapter hangs where the mutant
  falls over, and a hung conformance run names no rule at all. CF-33 forbids a
  clock inside a rule, so the suite cannot convert the hang into a message; what
  it can do is make the two cases distinguishable when the store *does* answer,
  which is what the second rule is for.

**A third obligation is enforced by a rule and stated by no clause, including
this one, and it is recorded here rather than left to be discovered by an
adapter that hangs.** ES-10's
`nothing_below_an_observed_position_appears_later` drives a full `read` to
completion at four points where one or both of its `append` futures are pinned,
entered and unfinished — so it requires that **a `read` issued while an `append`
on the same handle is suspended must complete**. This clause states the
append-versus-append direction; `a_live_read_stream_does_not_block_an_append`
states the read-then-append direction. Neither states this one.

The hazard is the same asymmetry one step worse. An adapter holding an exclusive
resource across its append's suspension point — one pooled connection, a
`futures::lock::Mutex`, a Durable Object storage transaction — has its `read`
block on what the suspended `append` still holds, and the suspended future cannot
be re-polled because the rule is blocked inside the read on the same stack. The
executor parks and never wakes. There is no watchdog by design (CF-33), so this
is a hung CI job naming no rule, and it is *harder* to diagnose than the pooled
case above because the thing that hangs is the read rather than the append.

Whether this becomes a fourth sentence here or a clause of its own is deferred:
this one is `[FROZEN]`, so it is an ADR's decision and not an edit's, and the
rule's own doc comment carries the warning in the meantime. What is not deferred
is the record that a rule introduced a MUST and no clause ratified it — which is
§7.4's failure mode seen from the other end.

---

### 3.7 Completeness — what a store that has been deleted from may look like

A store that holds only a suffix, or a scattered subset, of its own log is
indistinguishable from a complete one at every seam an ingest path or a runner can
see. Four scenarios reached this from different doors: a pruned device slice, a
regulated purge, a compacted peer, and a crypto-shred. The contract does not
assert completeness anywhere — `query_all_matches_every_event`
is store-relative by wording and therefore accidentally correct — so a pruned
store passes every rule unchanged. That is not the gap. The gap is
that nothing can *ask*.

#### ES-37 — `EventStore` is closed over insertion

`EventStore` MUST NOT grow a delete, truncate, redact, compact or tombstone method
at 0.1. Deletion is out of scope for the port.

**[FROZEN]**

This is the explicit written refusal E2E-48 asks for, and it is a refusal rather
than an oversight. The reason it is not free: a regulated purge and its
compliance marker must be one unit of work, so an adapter that supports deletion
must open a transaction *outside* the port — at which point the atomicity the port
spent its whole design defending is being provided by adapter-private code that no
rule observes. Naming that consequence is the price of the refusal, and it is the
strongest argument on record for revisiting it in 0.2 as a **redaction** seam
rather than a delete seam: shredding covers `data`, and it cannot cover `tags`,
which is the half the port has made indexable and queryable (`event.rs:437-447`
constructs a new value and there is no store-side update path).

- **Rule:** none — the absence of a method is not checkable. Recorded as a
  decision. The checkable consequences are ES-38 and ES-40.
- **Cases:** E2E-48, E2E-49, E2E-46.
- **Rejects:** the alternative that looks cheapest — adding `delete_before(p)` on
  the strength of the device-prune case alone. ES-39 explains why the shape is
  wrong.

#### ES-38 — What a store that has been deleted from is permitted to look like

A store from which events have been removed by any means outside the port MUST
continue to satisfy every clause of this section with respect to the events it
still holds. In particular it MUST NOT reuse a position it has previously
assigned, its remaining positions MUST remain unique and strictly monotonic, and
`Query::all()` MUST mean "every event this store holds".

**[FROZEN]**

- **Rule:** `positions_are_not_reused_after_removal` **(new)** and the existing
  rules re-run against the completeness fixture: a testkit-adjacent store that
  deliberately holds only a scattered subset of its own log, so that a runner or
  an ingest path written against it fails loudly rather than being accidentally
  correct. That instrument does not exist and CLAUDE.md names no adapter for this
  axis; it should. **The rule was examined at phase 4 and deliberately not
  written**: it needs a store events can be removed from, `Fixture` declares no
  such capability, and the instrument is CF-27's, which is `[DEFERRED]` with
  nothing planned before phase 14. **Owner: phase 14.** A `[FROZEN]` marker binds
  the design; it does not assert that anything checks it (ADR-0013, "What this
  ADR leaves open").
- **Cases:** E2E-46, E2E-10.
- **Rejects:** an adapter that renumbers on compaction — the obvious move for a
  device pruning to save space, and one that silently invalidates every checkpoint
  and every replicated `after` naming that store.

#### ES-39 — Whether a store can declare what it does not hold

**[DEFERRED — settled by building the completeness instrument named in ES-38 (a
store holding a deliberately scattered subset of its own log), then writing an
ingest path and a projection runner against it and recording which of the three
candidate primitives each needs: a floor (`earliest_position`), a set of retained
ranges, or a third outcome on condition evaluation. Owning phase: this is not a
ledger row anywhere and needs one — `RUNBOOK.md`'s ledger has no retention row at
all.]**

The analysis that must not be lost, because it is what makes the cheap answer
wrong. `earliest_position()` is the only primitive anyone has proposed. It is
exactly correct for a device pruning old history and useless for a regulated
purge, because that purge is **scattered, not a prefix**: a 2019 catastrophic-injury
file with a periodical payment order sits at a low position and must survive while
its neighbours are destroyed. A floor is the shape a prefix truncation has and
precisely the shape this purge does not, and it ships looking correct until a claim
runs long.

- **Rule:** `a_store_reports_the_history_it_does_not_hold` **(new)**, writable only
  once the primitive is chosen.
- **Cases:** E2E-46, E2E-47, E2E-56.
- **Rejects:** shipping `earliest_position()` on the strength of the prune case,
  which is the path of least resistance and the one that looks correct in every
  test anyone would write for it.

#### ES-40 — A conditional append is sound only over a complete store

The port's documentation MUST state that an `AppendCondition` is a claim about the
log the evaluating store holds, not about the world, and that a conditional append
is sound only where that store holds every event the condition's query ranges
over. A store that has had matching history removed MAY admit an append that
would have been rejected, and the contract MUST NOT imply otherwise.

**[PROVISIONAL — axis: **completeness**, whose far end is unbuilt and unplanned until CF-27's instrument exists. A pruned store and a young store are the same value at every seam the port exposes, so this clause currently describes a hazard no test can stage. Falsified — or given its assertion — by the suffix store of CF-27, which is why this clause and that instrument are one decision and not two.]**

`is_violated_by` is a pure predicate over events that still exist
(`append.rs:239-253`) and has no third outcome. Where history is gone the
information is missing from the store, not merely from the API, so the condition
passes **vacuously**. That is what actually happened in E2E-47: three engineer
notes landed on a claim with no registration, no policy, no reserve and no
closure, and the desktop projection materialised a row for it in four seconds.

- **Rule:** `condition_over_removed_history_does_not_reject` **(new)**, against
  ES-38's completeness fixture — an asserted-and-documented outcome rather than a
  desired one, so that an adapter author reads the vacuous pass as specified
  behaviour and an ingest author reads it as a hazard.
- **Cases:** E2E-47, E2E-56, E2E-44.
- **Rejects:** an ingest path that re-evaluates an origin condition against a
  pruned slice and concludes "no match" — which, given the unconditional-ingest
  decision, is not merely a hazard but the *normal* path, since ingest never
  rejects and the compensation decision is taken by the domain on the strength of
  a condition evaluated over a log that may be missing the very events it names.
  That is the strongest argument in this section for ES-39 landing before any
  peer runs a hub-side domain decision.

#### ES-41 — Membership is answered by a port operation, not by a query

`EventStore` MUST declare
`async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error>`,
reporting whether the store holds an event with that identity. It MUST be a
**required** method, not a provided one.

**[PROVISIONAL — falsified by an adapter that cannot answer membership without a
structure VT-8 does not already oblige it to keep. The exposed axes are
transport, because `contains_event_id` is one more round trip on a store with no
connection and no cursor, and completeness, because a store that cannot state
what it does not hold (ES-39) cannot distinguish "no such event" from "not
visible to me". The first named instrument has landed and answered the
**in-process, connection-holding** half: `happenstance-sqlite` implements the
method as a lookup on the `UNIQUE (origin_store, origin_position)` pair migration
1 already creates for ingest's duplicate guard
(`crates/happenstance-sqlite/src/event_store.rs:57`), so on that shape the
structure costs nothing VT-8 did not already oblige. What is still unanswered is
the **transport** half, and it is narrower than "an adapter" was: falsified by a
store with no connection, no interactive transaction and no cursor for which the
membership probe is a whole extra round trip it cannot fold into anything else —
`happenstance-cloudflare` at phase 9 and `happenstance-neon` at phase 10,
whichever lands first. Completeness is untouched by phase 8 and has an instrument
at neither end.]**

The `async fn` spelling is load-bearing and matches ES-30's: a clause written
without it describes a different method under `trait_variant`.

Required rather than provided, for ES-30's reason and not a fresh one. The only
provided form holds `&self` across an `await` and therefore needs `where Self:
Sync`, and the `!Send` edge adapter — the entire reason the bare flavour exists
— is `!Sync`, so the provided body is `error[E0277]` at the call for exactly the
adapter it was meant to spare. This is the **second** required method phase 4
adds to the port; `head` is ES-30's.

On `EventStore` rather than on `IngestStore`, where `happenstance-sync`'s sketch
already has it as `holds` (`crates/happenstance-sync/src/ingest.rs:164`).
VT-7 is frozen on §3, and — the substantive reason — VT-8 already obliges
**every** store to hold at most one event per `EventId`, so every store already
maintains the index that answers this and none is taxed with a new one. A store
that never replicates answers it without any index at all: if `id.store()` is not
its own incarnation the answer is `false`, because it has ingested nothing, and if
it is, the question reduces to whether that position exists. An adapter that
cannot hold its own incarnation between calls does not need that shortcut and
answers with the lookup itself — for `happenstance-neon` that is one
`WHERE origin_store = ? AND origin_position = ?` in the single round trip it is
allowed. `IngestStore::holds` therefore becomes a second operation answering one
question on the same concrete type, which is the duplicate-requirement failure
VT-12 warns about; removing it is `happenstance-sync`'s to do in the phase that
designs that port.

By value rather than `&EventId`: a `[u8; 16]` beside a `NonZeroU64`, both `Copy`,
where there is nothing to save by borrowing and an `&EventId` would put a
lifetime on a method every `dyn`-erasure and every blanket forward has to carry.
The contrast with `read(query: &Query)` is not an inconsistency — `Query` is not
`Copy` and a realistic two-clause query costs nine allocations and 212 bytes to
clone, which is why ES-13 borrows it.

`bool` rather than `Option<SequencePosition>`. Returning where the event landed
locally is strictly more useful and makes the operation answer two questions
instead of one — a store whose uniqueness is a database constraint rather than a
lookup table can prove membership without locating the row. Widening a `bool` to
an `Option` later is breaking; adding a second, locating method later is
additive. Of the two ways to be wrong, this is the cheaper one.

- **Rule:** `contains_event_id_reports_membership`.
- **Cases:** E2E-32, E2E-34, E2E-36 — VT-7's own, since this is the operation
  VT-7 requires.
- **Rejects:** a store that answers by position alone and ignores the `StoreId` —
  the natural `SELECT 1 FROM events WHERE position = ?`, which passes every
  single-store rule in the suite and reports a peer's event as present whenever
  the local log is long enough. `PositionOnlyMembershipStore` in
  `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` is that store,
  compiled and registered.

---

### 3.8 New conformance rules this section requires

The rules specified above did not exist when this subsection was written. They
fall into six groups by what they need, and the ordering matters because three
of the groups are blocked on
fixtures rather than on effort. The checks that live in `happenstance-core`'s own
tests rather than in the suite — `send_flavour_stream_is_send_in_generic_code`
and `spawns_from_generic` (both ES-2, and it takes both),
`provided_method_future_is_send_in_generic_code` (ES-3, ES-4) and
`error_bound_is_identical_on_both_flavours` (ES-5) — are not counted here,
because CF-22's enumeration covers the suite and not the contract crate's tests.

**Writable against the suite as it stands (single handle, one store).** Fourteen
of them were written at phase 3 stage 4 and are struck: ~~`reading_an_empty_store_yields_nothing`~~,
~~`a_live_read_stream_does_not_block_an_append`~~,
~~`read_limit_applies_after_filtering`~~,
~~`read_backwards_limit_applies_after_filtering`~~,
~~`duplicate_items_do_not_duplicate_events`~~, ~~`untagged_events_match_query_all`~~,
~~`query_item_order_does_not_change_the_result_set`~~,
~~`empty_batch_is_refused_before_the_condition_is_evaluated`~~,
~~`condition_matches_on_tags`~~,
~~`condition_with_an_unheld_tag_does_not_reject`~~,
~~`condition_against_an_empty_store_admits_the_append`~~,
~~`condition_after_beyond_head_admits_the_append`~~,
~~`condition_after_beyond_the_last_matching_position_admits_the_append`~~,
~~`interleaved_appends_on_one_handle_elect_one_winner`~~. The remaining eleven
were written at phase 4 and are struck with them, kept as a separate sentence
because the phase boundary is part of the claim this subsection makes:
~~`read_from_a_gap_position`~~,
~~`read_result_is_stable_under_concurrent_append`~~,
~~`limit_applies_across_items_not_per_item`~~,
~~`batch_positions_follow_slice_order`~~,
~~`batch_is_not_evaluated_against_its_own_condition`~~,
~~`dropped_append_future_leaves_no_partial_batch`~~,
~~`head_of_an_empty_store_is_none`~~,
~~`head_is_the_highest_visible_position`~~,
~~`read_to_is_inclusive`~~,
~~`read_from_and_to_bound_a_closed_window`~~,
~~`read_to_under_backwards_bounds_the_older_end`~~. **Nothing remains in this
group**, which is the first time it has emptied.

The struck names are kept struck rather than deleted because the *grouping* is
the claim this subsection makes — which rules need a fixture and which need only
effort — and a group that empties silently leaves no evidence the claim was ever
tested. Fourteen of them needed only effort, which is the first confirmation the
grouping has had.

One entry moved between groups rather than being discharged.
`read_from_composes_with_multi_item_query` was never listed here at all — it is
CF-12's and §6.2 carried it — and it belonged in this group throughout.

**Blocked on ES-33's fixture:** ~~`two_handles_observe_each_others_appends`~~,
~~`head_advances_across_two_handles`~~,
~~`acknowledged_writes_survive_a_reopen`~~. The fixture unblocked all three, and
that is itself the finding: this group held only rules the instrument was
missing, and it emptied when the instrument landed rather than when anybody
found more effort.

**Blocked on the completeness instrument (CF-27):**
`positions_are_not_reused_after_removal` (ES-38) and
`condition_over_removed_history_does_not_reject` (ES-40). Both are written against
a store that holds only a scattered subset of its own log, and CF-27 owns building
it.

**Blocked on a hostile fixture in the testkit's own `tests/`:**
~~`query_items_share_one_snapshot`~~. It ships with the store that can fail it or
it does not ship at all — a rule that has never failed anything is what CLAUDE.md
forbids, and it would otherwise pass on every adapter in the workspace on the day
it was written. It landed at phase 4, and the entry it was blocked on turned out
to be the wrong one: no *fixture* was needed. The pause point is the poll
boundary the rule itself controls, which is the same instrument CF-13 records for
ES-10, and `RefetchingPagedStore` is the store that fails it.

`nothing_below_an_observed_position_appears_later` (the name VT-12 and CF-13 also
use) was the second entry here and is no longer blocked: its hostile store is
`PreCommitPositionStore`, the rule is in `suite.rs`, and CF-13's `[DEFERRED]`
marker came off with it. It is left named here rather than deleted because the
paragraph's point is the *policy* — the rule and the store that fails it land
together — and one discharged example is the only evidence that the policy is
survivable.

**Blocked on machinery the suite does not have yet:**
`store_error_crosses_a_join_handle` (ES-6) needs the opt-in rule group whose
bound is ADR-0009's marker trait; the decision it waited on is taken, and what is
left is the group.

`append_is_atomic_under_a_mid_batch_fault` (ES-18) stood here until stage 6 and
is now written. The machinery it needed turned out **not** to be the
fault-injecting decorator this paragraph asked for — a decorator sits above
`append`, which is the unit the port makes atomic, so it can inject a fault
before the call or after it and nowhere in between. It is a *fixture* capability,
`MID_BATCH_FAULT`, and ES-18 carries the argument. Recorded rather than deleted,
because "the suite does not have the machinery" was the right diagnosis and the
machinery named was the wrong one, and the next entry in this group deserves the
same scepticism.

**Blocked on a decision:** `reissued_batch_after_a_dropped_future_lands_once`
(ES-24), `a_store_reports_the_history_it_does_not_hold` (ES-39),
`wire_condition_with_after_is_refused` (ES-29 and SY-6, and it lives in a crate
that does not exist).

Two existing rules are demoted rather than deleted. `positions_are_unique` and
`positions_are_strictly_monotonic` both read back through
`read`, which every adapter returns in position order, so both are vacuous for any
sorted read path; the rules that actually pin cross-batch ordering are the
append-condition rules and ES-19's new one. A third demotion has already landed
rather than being scheduled: `read_stream_is_send` no longer exists in
`crates/happenstance-core/src/memory.rs`. It asserted `Send` on a *concrete*
stream and passed by auto-trait leakage whatever the trait said, and the pair
named above replaced it —
`send_flavour_stream_is_send_in_generic_code`
(`crates/happenstance-core/src/memory.rs:613-640`), which writes the bound at the
definition, and `spawns_from_generic` (`:642-710`), which is what rejects the
`async fn read` refactor the first one cannot see.

---

## 4. The `ProjectionStore` port

The port declares itself provisional in its own first paragraph —
*"a port without a conformance suite is a guess"* (`crates/happenstance-core/src/projection.rs:3-11`).
This section's job is not to remove that marker. It is to state precisely what
would remove it, and to settle enough of the shape that an adapter can be built
against something other than a guess.

**So `[FROZEN]` means something narrower here than in §3, and §1.3 defines the
difference.** A frozen `PS` clause binds the *decision*: the next pass may not
redecide it without an ADR. It does not bind the *release*, because PS-2 forbids
freezing the port until two adapters at opposite ends of the batch-shape axis have
passed a suite that can fail, and PS-3 ships it behind `unstable-projection` until
they have. Nineteen clauses below are frozen in that first sense. None of them
commits the published surface of 0.1, and reading them as if they did would be
reading a stronger claim than this section makes.

Four facts frame everything below.

**The port has five implementers and, since phase 8, one adapter.** `grep -rn
"ProjectionStore for"` matches `SqliteProjectionStore`
(`crates/happenstance-sqlite/src/projection_store.rs:529`),
`PostgresProjectionStore`
(`crates/happenstance-postgres/src/projection_store.rs:94`),
`LadybugProjectionStore`
(`crates/happenstance-ladybug/src/projection_store.rs:258`),
`LiveHandleProjectionStore`
(`experiments/live-handle-projection-batch/live_handle.rs:174`) and
`NeonProjectionStore<T>`
(`crates/happenstance-neon/src/projection_store.rs:153`). Four are still phase-2
skeletons whose claims about transactions nothing has executed. The fifth is
not: `SqliteProjectionStore` has real bodies in all four port methods and
`crates/happenstance-sqlite/tests/projection.rs` mounts
`happenstance_testkit::projection_store_conformance!` against a real temporary
file, so **the suite it did not write now exists and it passes it**. What that
does not buy is PS-2, which wants the suite green against two adapters at
opposite ends of the batch-shape axis; this is a third replay-at-commit shape
beside `MemoryProjectionStore` and the testkit's buffering variant, and no
`rusqlite` adapter can supply the other end (a `rusqlite::Transaction<'_>` is
`!Send`). For the other four the *kind* of ignorance is still what it was: the
signatures have been disagreed with by a type checker and nothing more.

**`error[E0195]` is real, reproduced twice, and explains nothing about the
absence.** Spelling the impl with the concrete batch type — `async fn
commit(&self, _batch: MyBatch<'_>, …)` — fails against `projection.rs:126-131`;
only the literal `Self::Batch<'_>` compiles, and phase 2 hit it independently
from both ends (`crates/happenstance-sqlite/src/projection_store.rs:534-542`,
`experiments/live-handle-projection-batch/live_handle.rs:68-83`). It is a tax every
implementer pays, not a barrier: five impls were written straight through it.
The strongest available explanation for why none of them is an adapter is the
next fact down — there is nothing for one to pass. Two findings phase 2 *did*
produce are stronger arguments against the GAT than E0195 ever was, and both
belong to PS-5: a store carrying a lifetime **ICEs** rustc 1.97.1 rather than
diagnosing the region error
(`experiments/live-handle-projection-batch/live_handle.rs:38-66`), and any store generic
over a type parameter is forced to `'static` by the GAT whether or not its
batch borrows anything
(`crates/happenstance-neon/src/projection_store.rs:140-152`).

**The conformance suite cannot observe a read model.** `type Batch<'a>`
(`projection.rs:97-99`) carries no trait bounds, so generic code holding a
`P::Batch<'_>` can only hand it back to `commit` or `rollback`. The port exists
to defend read-model write and checkpoint write in one transaction
(`projection.rs:13-19`); a suite built on the port as written can test only the
second conjunct. By CLAUDE.md's own corollary — *"a rule that no adapter can
fail is decorative"* — such a suite is decorative in the exact place it matters,
because it cannot reject an adapter that commits the checkpoint and silently
drops the read-model write.

**All six scenarios hit the same two walls.** `references/scenarios/README.md:1923-1927`
records them as agreements 1 and 2 of nine: the port cannot reset, and `Batch`
has no write vocabulary. Six deliberately dissimilar deployments converging is
the signal this section is built on.

### 4.0 Numbering and the shape being specified

Clauses in this section are `PS-n` and are numbered only within it. Conformance
rules named below are **all new** — `crates/happenstance-testkit/src/suite.rs`
contains 89 rules, every one of them written against the `EventStore` port by
way of `Fixture::Store: EventStore`. There is no projection rule to reuse.

The shape the clauses add up to, given once so the rest reads as commentary on
it rather than as a puzzle:

```rust
/// Where a projection has been brought to, and whether its rows can be trusted.
///
/// An enum rather than `(Option<SequencePosition>, bool)` because the tuple can
/// spell `(None, true)` — authoritative, never run — which means nothing. The
/// crate's own "illegal states are unrepresentable" line, applied where a reader
/// will otherwise write `if let Some(p) = checkpoint` and get it wrong.
#[non_exhaustive]
pub enum Checkpoint {
    /// Never run, or reset, or rebuilding with nothing committed yet.
    NeverRun,
    /// Considered through `through`. The read model is authoritative.
    Live { through: SequencePosition },
    /// A rebuild is in flight, considered through `through`. Rows are not
    /// authoritative.
    Rebuilding { through: SequencePosition },
}

/// What a commit claims about the rows it leaves behind.
#[non_exhaustive]
pub enum Authority {
    Live,
    Rebuilding,
}

#[non_exhaustive]
pub enum CommitError<E> {
    /// The batch was begun on a different store instance.
    ForeignBatch,
    /// `position` is below the checkpoint already recorded.
    CheckpointRegression { current: SequencePosition, attempted: SequencePosition },
    Store(E),
}

#[non_exhaustive]
pub enum ResetError<E> {
    ForeignBatch,
    /// This store declines to reset this projection.
    Refused,
    Store(E),
}

#[trait_variant::make(SendProjectionStore: Send)]
pub trait ProjectionStore {
    type Error: core::error::Error + 'static;

    /// The adapter's write set. Owned, and not required to be a live
    /// transaction.
    type Batch;

    /// Opens a write set. Neither `async` nor fallible: opening a buffer cannot
    /// fail, and an adapter that needs a round trip takes it at `commit`.
    fn begin(&self) -> Self::Batch;

    async fn checkpoint(&self, id: &ProjectionId) -> Result<Checkpoint, Self::Error>;

    /// Applies the batch and moves `id`'s checkpoint to `position`, as one unit.
    async fn commit(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
        position: SequencePosition,
        authority: Authority,
    ) -> Result<(), CommitError<Self::Error>>;

    /// Applies the batch and returns `id` to `NeverRun`, as one unit. The dual
    /// of `commit`: the caller's batch carries the deletes, because only the
    /// caller knows which rows are the read model.
    async fn reset(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
    ) -> Result<(), ResetError<Self::Error>>;

    /// Discards the batch. Exists because `Drop` cannot await, so an adapter
    /// holding a real resource needs somewhere to release it.
    async fn rollback(&self, batch: Self::Batch) -> Result<(), Self::Error>;
}
```

Two enums rather than one `ProjectionError<E>` covering both operations. The
alternative was tried on paper and lost for the reason `AppendError`
(`crates/happenstance-core/src/error.rs:186-193`) is shaped the way it is: a caller
matching on the result of `commit` should not have to consider `Refused`, which
`commit` cannot produce, and `#[non_exhaustive]` already forces a wildcard arm
without also forcing dead ones.

---

### 4.1 Status, and what would freeze the port

**PS-1 — The read-model write and the checkpoint write MUST become durable
together or not at all.**
`[FROZEN]`
**Rule:** `commit_is_atomic_with_the_read_model` — write a probe row into a
batch, commit at position *P*, then read the row and the checkpoint through
fresh handles; both present or both absent, never one. §4.11 assigns this clause
two more: `commit_advances_the_checkpoint`, the baseline the rest of that suite
is differential against, and `failed_commit_leaves_both_unchanged`, which is the
second conjunct on its own — a partial apply that reports failure.

The second of those two follows from the sentence above and the first does not,
which is recorded here rather than quietly inherited from the table. **This
clause's MUST is a coupling, not a progress obligation.** A `commit` that returns
`Ok` and makes *neither* the read-model row nor the checkpoint durable satisfies
it through the "or not at all" arm, passes `commit_is_atomic_with_the_read_model`
— both absent is one of the two states that rule permits — and fails
`commit_advances_the_checkpoint`. That a successful commit *advances* anything is
stated by no clause's MUST in this document; PS-22 presupposes it and §4.1a's
prose asserts it non-normatively. Phase 6 owns the repair, which is either a
sentence in this clause or a clause of its own, and it is an ADR's rather than an
edit's because this clause is `[FROZEN]`.

**Phase 6's answer, recorded: a clause of its own, PS-38.** This clause's MUST is
byte-identical across it, which is what makes it a repair rather than a widening
— no implementation gains or loses conformance by anything written here. The
progress half of `commit_advances_the_checkpoint` now rests on PS-38 and the
coupling half still rests on this sentence; the reasoning, the two rejected
alternatives and the exposing implementation are in
[ADR-0030](../references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md).
One correction to the paragraph above travels with it and is stated rather than
edited in: *"stated by no clause's MUST"* was not quite true when it was written.
PS-23's *"One `commit` advances exactly one `ProjectionId`"* entails it, because
"exactly one" excludes zero — but it sits on a `[PROVISIONAL]` clause about
fan-out scope, where the word is almost certainly incidental
(`references/evaluation/ps-clause-pairing-sweep.md:355-395`). The obligation was
misfiled rather than absent, which changed the repair from *add a sentence* to
*put it where the rules can cite it*.
**Cases:** E2E-17, E2E-21, E2E-23.
**Rejects:** an adapter that writes read-model rows on one connection and the
checkpoint on another, which is the natural shape for any store whose read model
lives somewhere other than its checkpoint table. Today no rule can fail it: a
suite that cannot write a row cannot observe the first conjunct at all.

**PS-2 — The port MUST NOT be frozen until the conformance suite has failed a
store that commits the checkpoint and discards the read-model write, and until
two adapters at opposite ends of the batch-shape axis have passed it.**
`[FROZEN]`
**Rule:** a testkit-internal hostile store, `CheckpointOnlyStore`, in
`crates/happenstance-testkit/tests/`, asserted to fail
`commit_is_atomic_with_the_read_model`; plus the suite green against one adapter
holding a live transaction (rusqlite or `sqlx`) and one that cannot hold
anything across an await (Workers `SqlStorage` or Neon over one-shot HTTP).
**Cases:** E2E-17, E2E-24.
**Rejects:** the schedule that freezes this port against `MemoryProjectionStore`
and an in-process rusqlite transaction. Both serialise their writers, both hold
a real handle, both are the same storage shape wearing two hats — the exact
monoculture CLAUDE.md's spread rule exists to catch, and the one the plan's own
portfolio table already convicts for `EventStore`.

**PS-3 — Until PS-2's bar is met the port SHOULD ship behind an off-by-default
`unstable-projection` feature, with a documented exemption from semver.**
`[PROVISIONAL — falsified the moment PS-2's bar is met before 0.1, which happens
if a Workers or Neon projection adapter lands earlier than scheduled]`
**Rule:** `cargo hack --feature-powerset` in `cargo xtask ci`, which already
runs; the exemption is a doc obligation, not an adapter obligation.
**Cases:** none directly; it is what makes E2E-15 through E2E-25 safe to answer
now rather than after publication.
**Rejects:** publishing a 0.1 whose most defective surface is semver-binding.
The `tokio_unstable` idiom exists for exactly this, and it decouples publishing
`EventStore` from settling `ProjectionStore` — which is otherwise a hard
scheduling dependency between two unrelated ports.

What the projection suite showed when it was run against two batch shapes at
once is recorded in `references/evaluation/projection-batch-shape-evidence.md:1`,
which is evidence for this clause and deliberately not a verdict on it.

**PS-3's SHOULD is discharged as of phase 6, and its marker does not move.** The
feature exists: `unstable-projection` is declared in
`crates/happenstance-core/Cargo.toml:68`, is absent from `default`, and gates
`pub mod projection;` and its re-exports at
`crates/happenstance-core/src/lib.rs:110`; the documented exemption is the
module's own header (`crates/happenstance-core/src/projection.rs:3`) and the
`[Unreleased]` entry in `CHANGELOG.md`. The gate is held by
`the_projection_port_is_behind_an_off_by_default_feature` and
`the_gate_is_mounted_on_the_module_and_its_re_exports` in `xtask/src/main.rs`, so
the discharge is guarded rather than a claim about a moment in time. The
`[PROVISIONAL]` marker stays exactly where it is: its falsifier is *PS-2's bar met
before 0.1*, and that has not happened. A satisfied SHOULD is not a moved marker,
and nothing here says the port ships behind the feature at 0.1 — that is phase
12's decision (`RUNBOOK.md:601`).

---

### 4.1a What is actually open

Seventeen `PS` clauses are `[PROVISIONAL]`, which reads as a section that could
not make up its mind. It is not: **sixteen** of them are **six** questions, and
the clauses within each stand or fall together. A reader deciding whether to build
against this port needs the six, not the seventeen.

The seventeenth is **PS-3**, and it is deliberately not a row below, because it is
not an open question about the port's shape. Its falsifier names no design
uncertainty at all — it fires the moment PS-2's bar is met before 0.1 — so what
PS-3 waits on is the *schedule*, where the other sixteen wait on *answers*. It is
resolved by PS-2 and by nothing of its own, which is the gate §7.1 means when it
says PS-2 is the single one they all wait on. Giving it a row would put a seventh
question in this table that this section has never asked, and the table is meant
to be the questions.

| # | The question | Clauses | What answers it | Phase |
|---|---|---|---|---|
| 1 | Does any adapter need a live handle acquired **before** the first write? | PS-4, PS-5, PS-6, PS-12, PS-34 | The Ladybug skeleton. `lbug`'s graph writes are the named candidate: if they require a handle opened up front, the owned `Batch` is wrong, the GAT returns, and PS-34 becomes binding | 2, confirmed at 11 |
| 2 | Does generic code need a **write vocabulary** on `Batch`? | PS-9, PS-11 | A second generic consumer — any library code happenstance itself ships that must write into an unknown adapter's batch. One consumer (the suite) is served by PS-11's probe; two is a bound | 6, evaluated at 7 |
| 3 | What does a **rebuild** actually do? | PS-16, PS-22, PS-24, PS-25 | The first real rebuild, against a store whose read model is not a single table. In-place versus into-a-second-`ProjectionId`-and-swap is the fork | 6 |
| 4 | Does anyone **call** these? | PS-18, PS-27, PS-30 | Counting callers at the typed layer's phase exit. This is ADR-0007's own falsifier pattern, and it is the one that most needs an owner, because "nobody ever needed it" is not something you can observe by waiting | 7 |
| 5 | Can the **foreign-batch** hazard be made unrepresentable? | PS-15 | A zero-cost type-level construction naming an instance, which composes with `async fn` and permits a batch in a collection. Compiled and refuted once already: lifetimes name regions, not instances, so `b.commit(a.begin())` still builds | 6 |
| 6 | Must two views in one store be **mutually consistent**? | PS-23 | Norvant's control tower, which reads two views on one dashboard. If they must, one transaction must be able to advance two `ProjectionId`s | 6, from 11 |

Two things follow. **Question 1 gates five clauses and is answered by a skeleton
that phase 2 already builds** — so the largest cluster is also the cheapest to
close, and closing it early is worth more than its size suggests. And **question 4
is the one at risk of never being asked**: its clauses are provisional against a
condition nobody can observe passively, so the phase-7 exit criterion that counts
callers is not administrative tidiness — it is the only thing that makes those
three markers mean anything.

---

### 4.2 What a `Batch` is

The module documentation says a batch is a live transaction — *"a SQL
transaction, a Ladybug write handle"* (`projection.rs:22-23`) — and justifies
the GAT lifetime with *"a transaction cannot outlive its connection"*
(`:24-26`). Two of the four named deployments cannot honour that. Cloudflare
Workers' `SqlStorage` offers `transactionSync(callback)` and no handle that
survives an await; Neon over one-shot HTTP has no interactive transaction at
all. Both are forced to make `Batch` a buffer replayed in a single call at
commit (E2E-24). And `begin`, `commit` and `rollback` are all `async` with the
batch passed between them, so a batch is by construction held across await
points — which is the thing those two targets cannot do with a live handle.

The lifetime is unearned even where a live transaction exists.
`type Batch<'a> = rusqlite::Transaction<'a>` compiles on the bare flavour and
fails on `SendProjectionStore` twice over — `Connection` is `Send` and not
`Sync`, so `&Self` is not `Send`; and `Transaction<'_>` is not `Send`, so
neither future can be. The flavour every native adapter implements is the one
the lifetime does not serve. Meanwhile `sqlx::Pool::begin()` returns
`Transaction<'static, Postgres>`, which owns its pooled connection and borrows
nothing: a driver with every opportunity to hand back a borrowed handle chose
not to.

**PS-4 — A `Batch` MUST NOT be required to be a live transaction. An adapter MAY
back one with a live transaction; the port's obligation is PS-1, and PS-1 is
satisfiable by opening the transaction inside `commit` around a buffered write
set.**
`[PROVISIONAL — falsified by an adapter that can satisfy PS-1 only with a handle
acquired before the first write. LadybugDB is the named candidate: if `lbug`'s
graph mutations cannot be expressed as a replayable statement list, or if its
write handle must exist before a traversal that the projection's own logic
depends on, the deferred write set is not universal. Owned by the Ladybug phase
(RUNBOOK phase 4).]`
**Rule:** `commit_is_atomic_with_the_read_model` run against a buffering
adapter; the rule is shape-blind by construction, which is the point.
**Cases:** E2E-24.
**Rejects:** the port's own self-description at `projection.rs:22-26`, and any
suite rule written to assume a transaction is open between `begin` and `commit`
— for example a rule asserting that a concurrent reader is blocked during a
batch. Such a rule would certify rusqlite and fail a Durable Object for being
correct.

**PS-5 — `Batch` MUST NOT carry a lifetime parameter. It is `type Batch;`, an
owned value.**
`[PROVISIONAL — falsified together with PS-4; if a live handle turns out to be
required, the GAT returns and PS-34 becomes binding]`
**Rule:** `MemoryProjectionStore` and one real adapter compiling without the
`where Self: 'a` clause, which is a compile-time observation rather than a
runtime rule. Stated here because it is what PS-34 is contingent on.
**Cases:** E2E-19, E2E-24.
**Rejects:** nothing an adapter does at run time — this is a port-shape clause,
and it earns its place by removing two failures rather than by forbidding a
behaviour. It removes `error[E0195]` entirely: with no lifetime on the
associated type there are no lifetime parameters on `commit` to mismatch, so
`async fn commit(&self, batch: MyBatch, …)` compiles and the trap that has
blocked every adapter attempt disappears. And it removes the `where Self: 'a`
bound from every impl for a lifetime the `Send` flavour cannot use.

**PS-6 — `begin` MUST be neither `async` nor fallible.**
`[PROVISIONAL — falsified by an adapter that must reserve something from the
server before the first write, such as a batch identifier or an advisory lock
that cannot be taken at commit. Owned by the Neon phase.]`
**Rule:** the signature; no runtime rule. Enforced by the compiler on every
implementer.
**Cases:** E2E-24.
**Rejects:** a Neon adapter that spends a network round trip on `BEGIN` it does
not need, because the trait told it `begin` was allowed to be expensive. The
correction folded into E2E-24 is precise about this: Neon's problem is not that
a batch cannot borrow the client — it can, trivially — but that `begin` being
`async` and fallible implies a round trip that the one-shot HTTP transport
cannot afford and does not require.

**PS-7 — Dropping a `Batch` without `commit` or `reset` MUST roll back, and MUST
leave the store usable for subsequent batches.**
`[FROZEN]`
**Rule:** `dropped_batch_leaves_store_usable` — open a batch, write a probe row,
drop it, then open and commit a second batch; the first row is absent, the
second commit succeeds.
**Cases:** E2E-24.
**Rejects:** an adapter whose `begin` checks out a pooled connection that `Drop`
returns to nothing. A reviewer's probe already found exactly this: the store
returned `Busy` forever afterwards. The second half of the clause is not
decoration — "rolls back" alone certifies a store that has permanently lost its
only writer.

**PS-8 — `rollback` MUST remain on the port even though a buffered batch could
be dropped.**
`[FROZEN]`
**Rule:** `rollback_leaves_both_unchanged` — write a probe row, roll back, assert
the row and the checkpoint are both as they were.

**Recorded pairing finding, and it is an attribution one.** This MUST binds the
port's *definition* and is about the method's **existence**; the rule binds an
adapter and is about its **behaviour**. That *"rollback undoes"* is stated by no
`MUST` in §4, and the rule enforcing it hangs off the clause that keeps the method
on the port. The exposing implementation is a write-through adapter whose inherent
write API — which PS-9 explicitly blesses — hits the read model immediately and
defers only the checkpoint: it keeps `rollback` on the port, satisfying this
sentence verbatim, and fails the rule because the rows are already durable. The
strength is `dependent`, because that store also violates PS-1 and PS-7, so no
independent exposing implementation was found; ADR-0017 names the clause range and
did not repair it, and phase 6 records it rather than widening a `[FROZEN]`
sentence under cover of a packaging change
(`references/evaluation/ps-clause-pairing-sweep.md:265`).
**Cases:** E2E-24, E2E-28.
**Rejects:** the simplification that deletes it. Rust has no async `Drop`: an
adapter holding a real transaction has no way to issue `ROLLBACK` and await its
completion from a destructor, so removing `rollback` would confine PS-4's
"MAY back one with a live transaction" to adapters that can release a
transaction synchronously. It is also what makes a fan-out runner's
`AssertUnwindSafe` assertion honest rather than a lie (PS-30).

---

### 4.3 The write seam

`RUNBOOK.md:68` records this as **decided — both**, with no ADR: *"The port
grows the seam; the `Batch` written through stays adapter-specific, so a
projection targets exactly one store and one spanning two does not compile."*
Those two halves pull in opposite directions, and the reconciliation is that
there are two consumers with different needs.

An **application's runner** does not need a generic write vocabulary. ADR-0007's
pump compiles against `projection.rs` unchanged (compiled; PRESSURE-TEST §3.4),
because the caller-supplied closure `F: FnMut(&mut P::Batch<'_>, &SequencedEvent)`
is written where the concrete store is known and can call its inherent methods.
E2E-20 records the confirming counter-evidence: a Durable Object batch took
exactly that closure and worked. The unbounded associated type does not bite an
application that names its store.

The **conformance suite** does need one, because it is generic over an adapter
it has never seen. That is the whole of the problem, and it is why ADR-0007's
Context overstates itself when it says the runner "cannot be written against the
port as it stands — in either crate" (`0007:38-40`).

**PS-9 — `Batch` MUST NOT carry a universal write vocabulary. A projection
writes through the concrete adapter's inherent API.**
`[PROVISIONAL — falsified by a second generic consumer: any library code
happenstance itself ships that must write into an unknown adapter's batch. A
generic dead-letter recorder and a generic counter projection are the two
candidates; if either lands, `Batch` grows a bound and this clause is replaced.
Owned by the typed-layer phase (RUNBOOK phase 3).]`
**Rule:** none checks the absence of a bound. This is a port-shape clause; the
obligation it creates is PS-11, which is checkable.
**Cases:** E2E-20, E2E-29.
**Rejects:** the design that puts `fn put(&mut self, key: &str, value: &[u8])`
on a `ProjectionBatch` supertrait. It looks free and is not: it obliges a graph
store and a relational store each to carry a key-value table nobody asked for,
and it reintroduces at the read-model layer the opaque blob that ADR-0003
deliberately confined to event payloads. A read model exists to be queried by
the application; a blob keyed by string is not one.

**PS-10 — A projection MUST target exactly one store. `Projection::Store` is an
associated type, and a projection spanning two stores MUST NOT compile.**
`[FROZEN]` (ADR-0007:92-98, accepted)
**Rule:** a compile test on the `Projection` trait — a doctest annotated
`compile_fail,E0271` — showing
`error[E0271]: type mismatch resolving <NetworkTopology as Projection>::Store == Pg`.
The annotation is not a conformance rule and `spec-trace` was reading it as one;
naming it as a compile test is what makes §7.2 render this clause's own words
rather than dagger a name nothing ever looked for
(`references/evaluation/ps-clause-pairing-sweep.md:466-474`).
**Cases:** E2E-20, E2E-29.
**Rejects:** a heterogeneous supervisor holding fourteen Postgres views and six
Ladybug views in one collection. There is no cross-store transaction, so such a
supervisor would be lying about PS-1 at every commit. The constraint is correct;
what E2E-29 exists to force is that it is *documented*, so nobody designs the
supervisor first and meets `E0271` second. Runners are per store, and the port
should say so out loud.

**PS-11 — An adapter MUST implement the conformance probe. An adapter that does
not implement it cannot invoke the suite, and by CLAUDE.md's rule it does not
exist.**
`[PROVISIONAL — falsified by PS-9's falsifier: a real write vocabulary makes the
probe redundant]`
**Rule:** `commit_is_atomic_with_the_read_model` and every rule downstream of
it; the probe is the mechanism, not a rule of its own.
**Cases:** E2E-20, E2E-21, E2E-22, E2E-17.
**Rejects:** the current state of affairs, in which the invariant the port exists
for has no test.

The probe, and where it lives:

"The contract crate" throughout this document means `happenstance-core`, which is
`crates/happenstance-core` on disk. `happenstance` is the *typed* layer ADR-0006
gave the bare name to, and it is never what this document means by the contract
crate even where a paragraph predating the rename says otherwise. Nothing in this
clause depends on which name the crate carries, only on the probe living beside
the port rather than in the testkit — but the citations do, so they are anchored
on the tree as it now stands and CF-38's checker resolves them.

```rust
// the contract crate, behind `feature = "conformance"`.
// Bare flavour only: the suite binds the weaker trait (CLAUDE.md rule 4) and
// the per-test wrapper is a parameter (CF-23), so no `Send` bound is needed
// anywhere.
pub trait ProjectionProbe: ProjectionStore {
    /// Whether this adapter offers any read path on an open batch. See PS-12.
    const READS_THROUGH_BATCH: bool;

    fn probe_write(&self, batch: &mut Self::Batch, key: &str, value: u64);
    fn probe_delete_all(&self, batch: &mut Self::Batch);
    async fn probe_read(&self, key: &str) -> Result<Option<u64>, Self::Error>;

    /// Only called when `READS_THROUGH_BATCH`; may be `unimplemented!()` otherwise.
    fn probe_read_through(&self, batch: &Self::Batch, key: &str) -> Option<u64>;
}
```

It lives in the contract crate rather than the testkit for a coherence reason
worth spelling out, because it is the kind of thing that is discovered late and
expensively. An adapter crate implementing a testkit trait for its own type is
legal — the type is local, so the orphan rule is satisfied — but the natural
place to write such an impl is the adapter's `tests/` directory, and that is a
*different crate*: neither the trait nor the type is local there, and the impl is
rejected. Putting the trait in `tests/` therefore forces the adapter to take a
non-dev dependency on `happenstance-testkit` and gate it behind a feature.
Putting it in the contract crate behind `conformance` costs one feature flag on
a dependency the adapter already has, and no new edge in the graph.

`probe_delete_all` exists so that the suite can exercise `reset` (PS-16) without
knowing what the read model is.

---

### 4.4 Read-your-writes

Two events touching the same row in one `begin`/`commit` cycle. A SQLite
transaction gives read-your-writes; a Ladybug adapter that buffers Cypher and
flushes at commit does not; **both satisfy the trait as written**, and three of
Norvant's twenty projections are correct only under the first (E2E-21). Worse,
the difference is not merely a capability gap: under a write-behind batch, pairs
of events falling inside one chunk are silently missed, so the same log replayed
at chunk sizes 1, 100 and 5,000 produces three different read models (E2E-22).
A regulator-facing projection is the one that most needs a reproducible rebuild.

Three candidate answers were available: the batch is read-your-writes, the
runner guarantees one event per batch, or a projection may not read what it
writes. The second is the only one that is universally implementable, and its
cost is fatal — one transaction per event turns Wattline's 210-million-event
backfill into 210 million commits, and it makes the checkpoint the dominant
write in every projection. The third forbids three of Norvant's twenty and
`vehicle_contact_graph` outright. So the answer is the first, with the decline
made visible in the type system rather than in prose.

**PS-12 — An adapter MUST either make reads issued through an open `Batch`
reflect that batch's own pending writes, or expose no read path on the `Batch`
at all. It MUST NOT expose a read path that answers from committed state.**
`[PROVISIONAL — falsified by an adapter that can satisfy PS-1 but cannot answer
a read over its pending set at acceptable cost. Ladybug is the named candidate:
a graph traversal over buffered Cypher is not obviously cheaper than the
alternative. Owned by the Ladybug phase (RUNBOOK phase 4).]`
**Rule:** `batch_reads_reflect_pending_writes` — gated on
`ProjectionProbe::READS_THROUGH_BATCH`; write a probe row, read it back through
the same batch before commit, assert the pending value. CF-18 governs the gate:
the associated `const` is the declaration mechanism, and an adapter that declares
`false` MUST still emit the test as a **reported skip** carrying its reason, not
omit it — a rule absent from the binary is indistinguishable in CI output from a
rule that passed.
**Cases:** E2E-21, E2E-22.
**Rejects:** the buffering adapter that offers a `get` answering from the
committed table. That is the *natural* shape — `lbug` is synchronous and one
round trip is what you want — and it is the one that silently loses a write.
Declining by omitting the method is enforceable at compile time: a projection
that calls `batch.get(…)` simply does not compile against an adapter whose batch
has no `get`, so the capability is carried by the type rather than by a
paragraph. Declining by returning stale data is not detectable by anything.

**PS-13 — A projection MUST NOT read its own read model other than through the
batch it is writing.**
`[FROZEN]`
**Rule:** `rebuild_is_chunk_size_invariant`, below; a projection that reads
out-of-band fails it for any chunk size above 1.

**Recorded pairing finding, and it is the converse being asserted.** *"A projection
that reads out of band fails it"* is true; *"a failure means a projection read out
of band"* is not, and §7.2 prints the rule in a column headed *Conformance rule*,
so a reader takes it as this clause's falsifier. This MUST binds a **projection**,
and the rule runs in the **adapter** suite, where the projection is the testkit's
own probe and is PS-13-conformant by construction — so the rule can never observe
a violation of this sentence, and it *can* fail with a conformant projection: at
chunk size 3, against a write-behind adapter that declines
`ProjectionProbe::READS_THROUGH_BATCH` (which PS-4 and PS-12's second arm both
bless), the second increment in a chunk cannot see the first's pending write. The
strength is `dependent` — that adapter violates PS-14 — and the tension between
those two clauses is recorded at §4.11 rather than resolved here
(`references/evaluation/ps-clause-pairing-sweep.md:270`, `:404-421`).
**Cases:** E2E-21, E2E-22.
**Rejects:** the obvious implementation — `apply` takes `&self` on the store as
well as `&mut Batch` and issues its lookup through the store. It reads pre-batch
state, so its answer depends on where the chunk boundary fell, and it is correct
in exactly the test the author will write (chunk size 1).

**PS-14 — A rebuild MUST produce the same read model at every chunk size.**
`[FROZEN]`
**Rule:** `rebuild_is_chunk_size_invariant` — replay a fixed probe sequence at
chunk sizes 1, 3 and the whole log, comparing the probe read-back after each.
The probe's write must be defined as an increment of what the batch can see, so
that a store violating PS-12 or a projection violating PS-13 diverges.
**Cases:** E2E-22.
**Rejects:** any runner whose chunk size is a tuning parameter with a
correctness consequence. Rebuild determinism is a conformance property nowhere
in the workspace today, and chunk size is the first thing an operator turns.

---

### 4.5 The foreign-batch hole

`commit` accepts a batch begun on a *different store of the same type*: nothing
in `batch: Self::Batch`
(`crates/happenstance-core/src/projection.rs:474-480`) ties the parameter to
`&self` — the explanation used to be an elided lifetime, ADR-0017 removed it,
and the hole is exactly where it was — so `b.commit(a.begin(), …)` type-checks
and a runner holding a `HashMap<DepotId, SqliteProjectionStore>` can *write* the
program that puts one depot's inventory under another depot's checkpoint. What
that program can no longer do everywhere is *run*: the hole is closed at run
time by PS-15 rather than at the type level, and since phase 8 an adapter
outside the testkit closes it — `begin` mints the batch with the store's own
stamp (`crates/happenstance-sqlite/src/projection_store.rs:646`) and `commit`
compares it before the file is touched.

E2E-19 proposes tying the batch to the receiver's lifetime and calls it "the
cheapest fix in the entire catalogue". **It does not work, and this was
compiled** (PRESSURE-TEST §3.3): a port declared
`fn begin<'a>(&'a self) -> Self::Batch<'a>` and `fn commit<'a>(&'a self, batch: Self::Batch<'a>)`
still accepts `let batch = a.begin(); b.commit(batch);`. A lifetime names a
*region* of the program, not an *instance*, and two `&Store` references unify to
a common region without complaint. This is the single most useful Rust-specific
correction in this section, because the fix that does not work is the one every
reader reaches for first.

The only type-level construction that names an instance is a generative brand —
an invariant lifetime the caller cannot unify, in the style of `GhostCell`. It
works, and it is unusable here. A brand must be minted inside a closure that
owns the invariant lifetime, so `begin` becomes `store.with_batch(|batch| …)`;
that fights `async` at every turn, and it makes the batch unable to escape the
closure — which is precisely what a runner holding a map of stores needs it to
do. The construction defeats the caller the hazard is about.

**PS-15 — `commit`, `reset` and `rollback` MUST reject a batch begun on a
different instance of the same store type, at run time, through
`CommitError::ForeignBatch` / `ResetError::ForeignBatch`. A rejected call MUST
leave both stores unchanged.**
`[PROVISIONAL — falsified by a zero-cost type-level construction that names an
instance, composes with `async fn`, and permits a batch to be held in a
collection keyed by store. If one is found, this clause is replaced by a compile
error, which is strictly better.]`
**Rule:** `commit_rejects_a_foreign_batch` — build two stores from the fixture,
begin on the first, commit on the second, assert `ForeignBatch` and assert both
stores unchanged. This rule does **not** need CF-16's second handle: it wants two
*isolated* stores, which is exactly what CF-15's fixture instances already are,
and what two `open()` calls on the `impl AsyncFn() -> F` every rule is handed
already produce (`crates/happenstance-testkit/src/registry.rs:45-49`).
**Cases:** E2E-19.
**Rejects:** every adapter that can be written today, all of which corrupt
silently. The implementation is one word: `begin` stamps the batch with an
identity minted per store instance, and `commit` compares. With `Batch` owned
(PS-5) the stamp is a field, and the check is an integer comparison on a path
that is already doing I/O.

That the port carries a dedicated error variant rather than folding this into
the adapter's own `Self::Error` follows `AppendError`'s precedent exactly
(`error.rs:186-193`): a port-level outcome the caller must distinguish from an
adapter failure belongs in a port-level enum, because a generic caller — and the
suite is one — cannot name a variant inside an adapter's `#[non_exhaustive]`
error type.

---

### 4.6 Reset

All six scenarios. `checkpoint` returns an `Option` and `commit` takes a bare
`SequencePosition` backed by `NonZeroU64` (`event.rs:219-220`), so "never run"
is a state the port can **report** and no method can **produce**:
`store.commit(batch, id, None)` is `error[E0308]`. Rebuild is the most common
thing anyone does to a read model, and today it is done by reaching around the
port into the adapter's checkpoint table with raw DDL.

The design question is not whether to add `reset` but *how it clears the rows*,
because the port has no idea what the read model is. The answer is that it does
not clear them: `reset` is `commit`'s dual, taking a batch the caller has filled
with its own deletes. `commit(batch, id, position, authority)` moves the
checkpoint to `Some`; `reset(batch, id)` moves it to `NeverRun`. Same
transaction, same atomicity, no new knowledge required of the adapter.

**PS-16 — `reset(batch, id)` MUST apply the batch and return `id`'s checkpoint
to `NeverRun` as one unit of work.**
`[PROVISIONAL — falsified by an adapter whose read-model clearing cannot be
expressed through the same batch that carries ordinary writes; a store whose
`TRUNCATE` cannot participate in the checkpoint transaction is the shape to
watch. Owned by the projection-port phase (RUNBOOK phase 2).]`
**Rule:** `reset_clears_rows_and_checkpoint_together` — commit probe rows and a
checkpoint, then `reset` with a batch carrying `ProjectionProbe::probe_delete_all`;
assert both gone. Paired with a failure-injecting variant asserting that a `reset`
that errors leaves both halves as they were. (The probe method is qualified so
that `spec-trace` reads it as the seam it is rather than as a second conformance
rule — `references/evaluation/ps-clause-pairing-sweep.md:466-474`.)
**Cases:** E2E-15, E2E-17.
**Rejects:** the runbook procedure, which is two statements on two connections
and is what Norvant's night desk executed: the truncate committed at 02:46:31,
the pod died at 02:46:33, the runner restarted, read the old checkpoint, resumed
past it, applied sixty-one events into an empty table and **reported healthy.**

**PS-17 — Reset MUST be scoped to one `(store, ProjectionId)` pair. Other
projections in the same store MUST be untouched.**
`[FROZEN]` (ADR-0007:100-104 already fixes checkpoints per `(store, ProjectionId)`;
this is the same decision applied to the operation that removes one)
**Rule:** `reset_is_scoped_to_one_projection` — two ids in one store, both
committed; reset one; assert the other's rows and checkpoint are unchanged.
**Cases:** E2E-18.
**Rejects:** `SqliteProjectionStore::reset()` that truncates the checkpoint
table. Cheap, obvious, and it destroys the append-only regulatory ledger sharing
the file — Kestrel Cold Chain's `van_stock` must reset several times a day and
`fgas_ledger` must never.

**PS-18 — An adapter MUST be able to refuse a reset, through
`ResetError::Refused`. A refusal MUST leave both the read model and the
checkpoint unchanged, and MUST NOT be reported as success.**
`[DEFERRED — evaluated at the typed layer's phase exit and the count came back
unavailable rather than zero: the mechanism and its rule both exist now, and no
projection adapter has shipped to implement protection. Owned by
`projection-store-freeze` (HS-P0010), which takes the count when the first
adapter over storage this workspace does not control clears the projection
suite.]`
**Rule:** `refused_reset_changes_nothing` — run against a testkit fixture store
configured to protect one id; assert `Refused` and assert both halves intact.
**Cases:** E2E-18.
**Rejects:** the design that makes refusal purely a typed-layer concern. That is
where the *policy* belongs — the projection knows that `audit_trail_export` is a
hash chain a regulator already holds, and the store does not — but a policy with
no port-level mechanism is bypassed by anyone holding the store, which is every
operator with a runbook. The port supplies the mechanism; the domain decides
what to protect. This is the same division of labour the specification takes for
sync compensation.

**Evaluated at the typed layer's phase exit, and two things had changed — only
one of them expected.** The typed layer's planning pass recorded this clause's
subject as *absent from the tree*, and that is no longer true: `reset` is a
method on the port (`crates/happenstance-core/src/projection.rs:497`),
`ResetError::Refused` is a variant (`:287`), and
`refused_reset_changes_nothing` is a real rule registered in the projection
suite (`crates/happenstance-testkit/src/projection.rs:1360`, `:1909`). The
mechanism and its instrument are both here.

**What is still absent is an adapter, and the count is therefore unavailable
rather than zero — a distinction this marker exists to preserve.** No projection
adapter has shipped at all, and the single fixture in the workspace *declines*
the capability with the store's own reason: `MemoryProjectionStore` holds no
protection policy, so `reset` returns `Refused` on no path
(`crates/happenstance-testkit/src/fixtures.rs:446`). A fixture that claimed
`RESET_REFUSAL` to make this count come out would fail the very rule the count
is over. So the owner is named rather than the answer guessed, and a reader can
tell this from a question nobody has looked at — which is what moving the marker
buys and what leaving it `[PROVISIONAL]` would have destroyed.

**PS-19 — After a successful `reset`, `checkpoint(id)` MUST return
`Checkpoint::NeverRun`, and this MUST be distinguishable from
`commit(empty_batch, id, SequencePosition::FIRST, Live)`.**
`[FROZEN]`
**Rule:** `reset_is_not_commit_at_first` — perform both on two ids and assert
the checkpoints differ; then drive a replay from each and assert the event at
position 1 is applied in the first case and not in the second. §4.11 assigns
this clause `fresh_projection_has_no_checkpoint` as well, which is the same
distinction before any `reset` has happened: a store reporting
`Live { through: FIRST }` for an id it has never seen has already collapsed the
two states this clause requires to be told apart.

**That second rule reaches past this clause's MUST, and the gap is recorded
rather than papered over.** The sentence above is scoped *after a successful
`reset`*; the rule asks about an id that has never been seen. An adapter can
satisfy the MUST verbatim and fail the rule, and the shape that does it is the
natural one rather than a contrivance: `reset` writes an explicit `NeverRun`
sentinel row, and `checkpoint(id)` resolves a missing row with
`.unwrap_or(Checkpoint::Live { through: FIRST })`. Post-reset it answers
`NeverRun` and is distinguishable from a commit at `FIRST`; for an id it has
never seen it answers `Live`. No clause's MUST obliges an unseen id to read as
`NeverRun`. Phase 6 owns whether this clause widens or a new one says it.

**Phase 6's answer, recorded: a new one says it — PS-38's second sentence.** This
clause's MUST is byte-identical across that decision, and it stays scoped *after
a successful `reset`*; `fresh_projection_has_no_checkpoint` is PS-38's falsifier
and is listed against both clauses in §4.11 for that reason. The rule is written
and runs — it is in the single enumeration §4.11 names, so it carries no *new*
marker here or there, and §7.2 daggers neither of the two rows that name it
([ADR-0030](../references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md);
`references/evaluation/ps-clause-pairing-sweep.md:276`).
**Cases:** E2E-15, E2E-16.
**Rejects:** a runner that computes its resume point as
`checkpoint.unwrap_or(FIRST)` and reads `ReadOptions::from` that value —
`from` is inclusive (`query.rs:270`, `store.rs:166`) — which double-applies event
1 for a genuine checkpoint of `Some(1)`. The mirror wrong implementation is the
operator's workaround itself: `commit(empty, id, FIRST)` reads back as
`Some(1)`, the runner advances past it, and event 1 is skipped permanently,
silently, and nobody will ever find it. The rule should stay in the suite after
`reset` lands, because the idiom will outlive its necessity.

**PS-20 — A runner MUST resume strictly after the checkpoint's position, and
MUST start at the store's first position, inclusive, when the checkpoint is
`NeverRun`.**
`[FROZEN]`
**Rule:** covered by `reset_is_not_commit_at_first`'s second half; no separate
rule, because the property is only observable through a replay.
**Cases:** E2E-16, E2E-23.
**Rejects:** the off-by-one in both directions, and it names a dependency:
`ReadOptions::from` is inclusive while `Guard::after` is exclusive
(`append.rs:135-140`), and the only API for advancing a position is
`SequencePosition::next()`, which was `saturating_add` under a doc promising
`None` on overflow (defect D3). The runner MUST NOT be written against a `next()`
that cannot signal overflow, because at `u64::MAX` the saturating version resumes
at the position it just applied — an infinite reapply loop in the one place
nobody will test. **That obligation is discharged, not withdrawn:** `next()` is
`NonZeroU64::checked_add` in `match` form (`event.rs:272-282`) and
`position_next_signals_overflow` (`event.rs:908-939`) asserts it, so a runner may
now be written against it. VT-13 is the fix and freezes it, and it
also establishes the half a runner author will otherwise re-derive: because
`from` is an inclusive lower bound rather than a seek, `checkpoint.next()` is a
correct resume point on a store with gaps, even when the next position is
unoccupied.

---

### 4.7 Checkpoint semantics

**PS-21 — `commit` MAY name a position no applied event occupies. An adapter
MUST NOT validate `position` against what the batch wrote.**
`[FROZEN]`
**Rule:** `commit_accepts_a_position_the_batch_did_not_write` — commit an empty
batch at a position the store's event log actually assigned, assert success and
assert the checkpoint advanced.
**Cases:** E2E-23.
**Rejects:** an adapter that validates. That is a perfectly reasonable reading of
"advances `id`'s checkpoint to `position`" (`projection.rs:167-179`), it would be
equally conformant today, and it makes a narrow projection re-scan the same
range forever on every restart —
Norvant's `cold_chain_certificate_expiry` matches about 40 of 37,000 events a
day. Two adapters can disagree and both pass, which is the sharper hazard than a
capability gap: it is a silent interoperability difference between two stores an
application might swap.

The checkpoint is therefore a high-water mark of **consideration**, not of
application. That is what makes it a resume point rather than a progress report,
and it is why PS-24 needs a separate signal for authority.

**Recorded pairing finding.** `commit_accepts_a_position_the_batch_did_not_write`
asserts a second thing this MUST does not say — *that the checkpoint advanced* —
and the rule commits an **empty** batch by design, so a store that skips the
checkpoint write when there is nothing to write satisfies this sentence verbatim
and fails the rule. That obligation is PS-38's as of phase 6, not this clause's;
this MUST is unchanged and the finding is recorded here rather than repaired here
(`references/evaluation/ps-clause-pairing-sweep.md:278`).

**PS-22 — `commit` MUST reject a position strictly below the current checkpoint,
through `CommitError::CheckpointRegression`. Equal positions MAY be accepted.**
`[PROVISIONAL — falsified by a legitimate need to move a checkpoint backwards
without clearing rows. None of the six scenarios has one; a compacting store
that renumbers positions would, and that is the shape to watch. Owned by the
retention decision.]`
**Rule:** `commit_rejects_a_regressing_position` — commit at *P*, then attempt
*Q* < *P*; assert `CheckpointRegression { current, attempted }` and assert both
halves unchanged.
**Cases:** E2E-23, E2E-25.
**Rejects:** the adapter that issues `UPDATE checkpoint SET position = ?`
unconditionally, which is what everyone writes. Under a redeploy where an old
runner pod has not yet exited, two runners share an id and the stale one drags
the checkpoint backwards; every event between the two positions is then applied
twice, which is harmless only for projections that are idempotent — and
`projection.rs:28-30` offers idempotence as an escape hatch rather than
requiring it. The guard converts a silent double-apply into a reported error,
and it leaves exactly two ways to go backwards: `reset`, which is atomic with
clearing the rows, and nothing.

**Recorded pairing finding.** This MUST is vacuous against a store that never
advances a checkpoint at all: nothing is ever *strictly below* a current
checkpoint that does not exist, so the sentence is satisfied while
`commit_rejects_a_regressing_position` fails. The presupposition the rule needs is
PS-38's as of phase 6. This clause is `[PROVISIONAL]` and its marker does not move
on that account — the falsifier above is a compacting store, and no compacting
store has appeared (`references/evaluation/ps-clause-pairing-sweep.md:279`).

**PS-23 — One `commit` advances exactly one `ProjectionId`.**
`[PROVISIONAL — falsified by a pair of read models in one store that must be
mutually consistent at every observable instant. Norvant's control tower reading
two views together is the candidate; if it is real, `commit` takes a set of
`(ProjectionId, SequencePosition)` and every adapter's checkpoint write changes.
Owned by the typed-layer phase (RUNBOOK phase 3), where the fan-out runner is
built.]`
**Rule:** `distinct_projections_advance_independently` — commit two ids at
different positions, assert each reads back its own.
**Cases:** E2E-28, E2E-32.
**Rejects:** an adapter with a single-row checkpoint table, which is what a store
that has only ever run one projection will write. It passes every other rule.

**PS-24 — The checkpoint MUST distinguish an authoritative read model from one
being rebuilt. `commit` carries `Authority`; `checkpoint` returns
`Checkpoint::Live` or `Checkpoint::Rebuilding`.**
`[PROVISIONAL — falsified if rebuild-in-place turns out to be always wrong, in
which case the only correct procedure is to rebuild into a second `ProjectionId`
and swap, `Rebuilding` collapses, and the swap protocol is specified instead.
The discriminating measurement is storage: a store that cannot hold two copies
of a 4.1M-event read model has no swap available. Owned by the projection-port
phase.]`
**Rule:** `rebuilding_is_distinguishable_from_live` — reset, commit two chunks
with `Authority::Rebuilding`, assert `checkpoint` reports `Rebuilding` after
each; commit a third with `Authority::Live`, assert `Live`.
**Cases:** E2E-25.
**Rejects:** rebuild in place with a single position field, which is the obvious
reading of the port and is what `Option<SequencePosition>`
(`projection.rs:101-110`) permits and nothing else. On the Kestrel Cold Chain hub
the mid-rebuild read model is precisely what the next device's work slice is cut
from, so a checkpoint that reports "caught up to *N*" while holding half a graph
**manufactures the conflict the projection exists to prevent.** A rebuild that
has committed nothing reads as `NeverRun` rather than `Rebuilding`, which is
correct: both mean the rows are not authoritative, and the reader's decision is
the same.

**PS-38 — A successful `commit(batch, id, position, authority)` MUST advance
`id`'s checkpoint to `position`, and a `ProjectionId` no successful `commit` has
named MUST read as `Checkpoint::NeverRun`.**
`[PROVISIONAL — falsified by a store that answers `checkpoint` from a replica
that may lag its own `commit`, which is the shape a projection store over an
eventually-consistent read model has. If that is real the obligation narrows to
"a subsequent read through the same handle" and every rule downstream of it gains
a handle constraint. Owned by the first projection adapter over storage this
workspace does not control (RUNBOOK phase 7).]`
**Rule:** `commit_advances_the_checkpoint` — the baseline the rest of §4.11's
suite is differential against; and `fresh_projection_has_no_checkpoint` for the
second sentence, which asks a store for an id no `commit` has named and asserts
the `NeverRun` variant, comparing no position anywhere.
**Cases:** E2E-15, E2E-17, E2E-23.
**Rejects:** a store whose backing state lives per **handle** rather than per
store — one whose `connect()` mints a fresh map instead of a fresh handle onto a
shared one. It is a plausible first cut, and until this clause existed nothing in
§4 forbade it: it satisfies PS-1 through the *"or not at all"* arm, satisfies
PS-21's MUST by validating nothing, satisfies PS-22 vacuously because no
checkpoint is ever current enough to be regressed below, and satisfies PS-19's
MUST because a `reset` it does not remember is indistinguishable from one it
does. Three rules rested on an obligation no sentence stated.

**Why this is a new clause rather than a sentence added to PS-1.** PS-1 is
`[FROZEN]` and its MUST is a *coupling* — both writes durable or neither — which
a store that makes no writes durable satisfies. Adding progress to it would
change the set of implementations it admits, which is a gap rather than a repair
(`.kb/decisions/README.md:20-22`), and a gap is a decision's. The decision is
[ADR-0030](../references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md),
which took minting this clause over the two alternatives it names: widening PS-1,
and splitting PS-23's *"exactly one"*, which is the only sentence in the document
that entails progress today and does so as a side effect of a word chosen for a
different purpose (`references/evaluation/ps-clause-pairing-sweep.md:355-395`).
PS-1, PS-19, PS-21 and PS-22 are byte-identical across that decision; what
changed is that the obligation their rules assume is now written down.

**PS-25 — A checkpoint MUST NOT survive a change to the `Query` that produced
it. The `ProjectionId` a runner uses MUST be derived from the projection's name
and a digest of its `Query`.**
`[PROVISIONAL — falsified by a projection whose query is *narrowed*, where no
rebuild is needed and the derived id forces an expensive one anyway. If that
case is common, the digest moves into the checkpoint record as a separate
field and `commit` rejects a mismatch instead. Owned by the typed-layer phase.]`
**Rule:** the new `changed_query_starts_a_new_checkpoint` — commit under one
query's derived id, then read the checkpoint under a second query's derived id and
assert `NeverRun`. Contract-level only once `Query` has a canonical encoding;
until then it is a typed-layer rule, and it is unwritten for that reason rather
than by oversight.
**Cases:** E2E-50.
**Rejects:** every runner that keys a checkpoint on `ProjectionId` alone, which
is every runner the port permits. ADR-0007 pins a projection's subscription to
`Query` deliberately and correctly (`0007:85-90`); this is the unstated cost. In
Kestrel Motor the `retention-ledger` projection gained two hold types in 2024,
every event of those types below its checkpoint was never applied and never will
be, and it under-reported held claims for two years — and it is the projection
the retention job depends on. Deriving the id converts a silent wrong answer
into a visible rebuild.

This clause has a dependency on §2 and §2 does not supply it directly, so the
resolution is stated here rather than left as a blocker. A digest needs a
canonical encoding of `Query`. `QueryItem::new` already sorts and dedups types
(`query.rs:62-67`) and VT-16 makes `Tags` canonical, but **nothing canonicalises
item order and VT-31 deliberately declines to** — item order is not observable in
the match set, so making it part of the value would be pinning something the
algebra says is free. The digest therefore MUST impose its own order: encode each
item, sort the encodings, then hash. That is sound precisely because VT-31
guarantees order does not change the match set, it needs no wire change, and it
is available today. What is *not* available is hashing the serialised `Query`
verbatim, which is unstable across two runners that built the same query from
different iteration orders.

---

### 4.8 Failure policy

`RUNBOOK.md:3841-3846` is open. The scenarios settle the shape of the answer even
though they disagree on the answer itself, and the disagreement *is* the result:
halting is correct for a revenue ledger — one that skips an event is worse than
one that stops — and wrong for an availability board, where a stale board is
worse than one missing a connector. A deliberate crypto-shred needs a fourth
option that is neither retry, halt nor dead-letter, because the decode failure
is permanent and *intended*.

**PS-26 — The failure policy MUST be declared per projection, not per runner.**
`[FROZEN]`
**Rule:** the new `failure_policy_is_per_projection` — two projections in one
runner declaring `Halt` and `SkipAndRecord`; feed both a failing event; assert the
first stops at that position and the second advances past it. Integration-level;
it needs a runner, so it belongs in the workspace e2e crate rather than the
adapter suite, and it is unwritten because that crate is the typed-layer phase's.
**Cases:** E2E-27.
**Rejects:** a runner-level `on_error: SkipPolicy` configuration. It is the
obvious design, it is what a builder API invites, and it forces one wrong answer
onto one of Wattline's two projections.

**PS-27 — The policy MUST offer skip-and-record, and the record MUST be written
into the same batch that advances the checkpoint past the poisoned position.**
`[DEFERRED — evaluated at the typed layer's phase exit and the count is zero:
the alpha's runner halts on the first failure and offers no failure-policy seam
at all, so there is no path that could write a skip record. Not withdrawn — the
Kestrel Motor shred case still needs it. Owned by `projection-store-freeze`
(HS-P0010), which owns both the suite rule and the port surface a skip record
would be written through.]`
**Rule:** the new `skip_and_record_is_atomic` — a projection whose
`Projection::on_error` writes a probe row; feed it a failing event; assert the
probe row and the advanced checkpoint are both present after a crash injected
between them. Integration-level: it needs a runner, so §4.11 lists it apart from
the adapter suite and its home is the workspace e2e crate. (The callback is
qualified for the reason PS-16's probe method is —
`references/evaluation/ps-clause-pairing-sweep.md:466-474`.)
**Cases:** E2E-26, E2E-27.
**Rejects:** the implementation that swallows the skip. The skip *primitive*
already exists and nobody has noticed it: `begin()` followed immediately by
`commit(batch, id, poison_position, Live)` applies nothing and advances the
checkpoint atomically, with the port exactly as written. What does not exist is
any way to record that it happened — and in Kestrel Motor the skip is the
Article 17 evidence, so a swallowed skip is a compliance failure rather than a
missing log line. Routing the record through the projection's own batch means no
new port surface and no store-side knowledge of what a skip means.

**Evaluated at the typed layer's phase exit, and the count is zero — but not for
the reason the falsifier anticipated.** The falsifier expected *"record"* to
degrade into *"log a warning"*. It did not: `happenstance::run_projection`
(`crates/happenstance/src/runner.rs:401`) writes nothing anywhere, logs nothing,
and **halts** on the first failure it meets. There is no `on_error`, no
`SkipPolicy` and no policy argument of any kind, and that absence is deliberate
rather than unfinished — it is this clause's own *Rejects* read one layer up. A
runner-level policy forces a single answer onto every projection an application
runs, and two read models of different tolerance are exactly the case that makes
one answer wrong; failure policy belongs to the projection, and the projection
trait has no seam for it yet.

**So the requirement is deferred, not discharged and not withdrawn.** The skip
primitive is still reachable by hand — a caller who spells `begin()` and
`commit(batch, id, poison, Live)` themselves skips atomically today — and what
is still missing is the typed way to record that it happened.
`projection-store-freeze` (HS-P0010) owns both halves: the suite rule
`skip_and_record_is_atomic` and the port surface the record would ride on. The
artefact that reopens this clause is a per-projection failure policy on
`Projection`; until one exists, an executed test holds the count
(`crates/happenstance/tests/projection_clauses.rs`), so the day the runner grows
one, the clause that says it has not goes red.

**PS-28 — A failing `apply` MUST report the position it failed at, and MUST be
able to carry an application error type distinct from the projection store's.**
`[FROZEN]`
**Rule:** the new `pump_reports_the_failing_position` — integration-level; assert
the returned error names *P* and that `checkpoint` sits at the last good position.

**Recorded pairing finding, and it is undetermined rather than sound.** The second
assertion is not in this MUST, and whether it reaches past it turns entirely on
what *"the last good position"* means, which the rule does not say. Under *last
successfully applied event* a chunked runner fails it while satisfying this
sentence, because its checkpoint sits at the previous chunk boundary after a
mid-chunk failure; under *last successfully committed position* the two coincide
and the pairing is sound. Both readings are available from the text. What resolves
it is defining the phrase when the rule is written, which is the typed-layer
phase's (`references/evaluation/ps-clause-pairing-sweep.md:285`).
**Cases:** E2E-26.
**Rejects:** ADR-0007's own signature. It types the callback's error as
`P::Error` (`0007:62-67`) — a **projection store** error — so a *decode* failure
has no representable home: the application must forge one into the adapter's
`#[non_exhaustive]` error enum, which belongs to the adapter and is
`#[non_exhaustive]` precisely so that outsiders cannot construct it, or it must
panic. It will panic. The pump's error therefore needs a third parameter,
`PumpError<E::Error, P::Error, A>` with an `Apply { position, error: A }`
variant. Three type parameters is a real cost and the alternative —
`Box<dyn core::error::Error>` — is barred in library code by house style and by
`no_std`.

**PS-29 — One poisoned projection MUST NOT stall the others, and its terminal
state MUST be observable through the API.**
`[FROZEN]`
**Rule:** the new `one_poisoned_projection_does_not_stall_the_others` — twenty
projections over one log, one failing; assert the other nineteen advance *and*
that the supervisor reports the failure without being polled for it.
Integration-level, and unwritten because the runner is the typed-layer phase's.

**Recorded pairing finding.** The rule's second assertion — *without being polled
for it* — reaches past this MUST, which requires only that the terminal state be
**observable through the API**. A supervisor exposing
`fn failures(&self) -> Vec<Poisoned>` satisfies the sentence verbatim, requires
polling, and fails the rule; a query method is the obvious API and is precisely
what the *Rejects* field below describes losing. The gap is
`independent` — no other `PS` MUST rejects that supervisor — and it is recorded
here rather than repaired here: the rule does not exist yet, and the observability
design ADR-0019 defers to the typed-layer phase is where the two sentences get
reconciled (`references/evaluation/ps-clause-pairing-sweep.md:286`).
**Cases:** E2E-28.
**Rejects:** the half of this that already fails. Isolation itself works, and it
works structurally: `Projection::Store` is an associated type and checkpoints are
per `(store, ProjectionId)` (ADR-0007:92-104), so a failure cannot span two.
The second half is what Norvant lost — nineteen projections kept running,
nothing alerted, because the `JoinHandle` went into a set nobody drained. A
supervisor whose failure reporting depends on the caller remembering to await a
handle is a supervisor that reports nothing.

**PS-30 — A fan-out runner that catches a panic in `apply` MUST `rollback` the
batch before continuing.**
`[DEFERRED — evaluated at the typed layer's phase exit and the fan-out runner is
not built, so the MUST binds nothing today. Its contingency is a benchmark
rather than an assertion and is deliberately outside the gate under CF-34; the
harness now exists as `experiments/polling-cost`, which is the artefact that
reopens this clause. Owned by `projection-store-freeze` (HS-P0010) once a
fan-out runner is buildable at all.]`
**Rule:** the new `panicking_apply_rolls_back` — integration-level; a projection
that panics; assert no partial rows survive and the checkpoint did not move.
**Cases:** E2E-28.
**Rejects:** a fan-out runner that wraps `&mut P::Batch` in `AssertUnwindSafe`
and carries on. The assertion is defensible *only* because `rollback` exists:
`AssertUnwindSafe` is a promise that no observer will see a half-mutated value,
and `rollback` is what discharges it. Without the rollback the promise is a lie,
and the workspace depends on two unwritten facts to make any of this work —
that the release profile does not set `panic = "abort"`, and that the cheap
runner (one read, one decode, twenty applies) and the safe runner (twenty
`tokio::spawn`s) are opposites the port adjudicates neither way.

**Evaluated at the typed layer's phase exit: the fan-out runner is not built,
and the obstacle is now stronger than *"nobody got round to it"*.** The port's
`Batch` stopped being a generic associated type and became a plain **owned** one
(`crates/happenstance-core/src/projection.rs:436`), so a write set cannot be
shared between tasks at all — and moving one into a `tokio::spawn` would
additionally require `Send`, which the flavour that exists for `wasm32` cannot
promise. `happenstance::run_projection`
(`crates/happenstance/src/runner.rs:401`) therefore drives exactly one
projection per call, catches no unwind, and spawns nothing; N read models cost N
independent reads.

**The contingency is named rather than left implicit.** This clause was made
conditional on a benchmark the workspace did not have. It has one now —
`experiments/polling-cost`, which records the delivery amplification N
independent reads impose — and CF-34 keeps it outside the gate by construction,
so a number moving does not turn a measurement into a build failure. That
harness is the artefact that reopens this clause: if its figure makes fan-out
worth building, the runner that results owes this MUST a rollback and
`projection-store-freeze` (HS-P0010) owns the rule. Until then the absence is
held by an executed test rather than by recollection
(`crates/happenstance/tests/projection_clauses.rs`), which fails the moment the
typed layer grows a `catch_unwind` or a spawn.

**PS-31 — A projection that emits events back into the event log is out of scope
for `ProjectionStore` at 0.1, and the port MUST say so.**
`[FROZEN]`

Drafted as `[DEFERRED — settled by the event-identity decision]`, and that
deferral does not survive §2. Identity is settled: VT-5 makes an `EventId` a
store-assigned `(StoreId, SequencePosition)` pair minted at append, and VT-10
forbids `EventStore::append` from accepting a caller-supplied one. An
outward-writing projection needs exactly what VT-10 refuses — a write whose
identity the *caller* chooses, so that a rebuild re-emitting the same event is a
no-op rather than a second fact. The exclusion at 0.1 is therefore a consequence
of a decision already taken, not a question still open. Revisiting it in 0.2
means a deliberate idempotent-emission seam, and VT-5 is what would make one
expressible.

**The second conjunct is discharged as of phase 6, and it was not before.** *"The
port MUST say so"* had no instrument and, as the phase-6 disposition pass found,
no discharge either: nothing in `projection.rs` mentioned the exclusion. It does
now — `crates/happenstance-core/src/projection.rs:56-71`, a section of the module
header that states the exclusion, derives it from VT-5 and VT-10, and names the
duplicate-on-rebuild failure silence produces. The sentence above is unchanged;
what changed is that it is true. There is still no instrument, and the shape it
shares with PS-3 and PS-36 — *a documentation obligation with no instrument* — is
recorded at `references/evaluation/ps-clause-pairing-sweep.md:433-454` as a
candidate open question rather than closed here.

**Rule:** none; a documented exclusion is not adapter-checkable. Stated as a
clause rather than as prose because silence here is what produces the wrong
implementation.
**Cases:** E2E-31.
**Rejects:** the obvious implementation — emit through a command handler after
`commit` returns — which re-emits on every rebuild. It also falsifies the port's
own escape hatch: `projection.rs:28-30` tells a projection that cannot be
transactional to be idempotent instead, and an outward-writing projection cannot
be. On the Kestrel Cold Chain hub both stores are the same SQLite file, so the
transaction is physically available and the two ports forbid expressing it. The
honest answers are two — the emitted event is written idempotently and
duplicates are tolerated, or process-manager emission is declared out of scope —
and today it is neither.

---

### 4.9 The runner split

**PS-32 — ADR-0007's Context MUST be corrected: a callback-driven pump *can* be
written against the port as it stands. What cannot be written is the conformance
suite.**
`[NON-NORMATIVE — a clause whose subject is another document's wording is a work
item, not a constraint on any implementation. The work item now exists and is
staged; the ID is retained so that citations resolve.]`
**Rule:** none — this is a correction to a document, and the artefact that
proves it is the compiled pump recorded in PRESSURE-TEST §3.4.

**Phase-6 disposition: recorded as owed, and deliberately not performed.**
ADR-0007 is an **accepted, immutable** decision atom, so correcting its Context is
a *superseding* atom's job and never an edit — reasoning inside a decision that
still stands is never touched
(`.kb/governance/rewrite-the-referent-never-the-reasoning.md:50-59`).
[ADR-0017](../references/adr/0017-what-a-projection-batch-owns.md#L356) already
records the correction as owed and states its shape without performing it, and
this pass changes nothing about that: performing it means writing the superseding
atom, which is `/redkiln:kb-ingest`'s to author from `.kb/_intake/`, and the
sentence being corrected belongs with whoever writes the runner.

**The runner is written, and the staging note the correction was waiting for now
exists.** `.kb/_intake/0032-adr-0031-the-runner-collapses-upward.md` carries both
halves in one document: PS-33's verdict, which supersedes ADR-0007's pump
allocation, and this clause's correction to the Context sentence that produced
it. The earlier staging note this paragraph pointed at was consumed by the wave
that produced
[ADR-0030](../.kb/decisions/0030-the-checkpoint-reports-the-commits-that-happened.md),
and pointing at a file a successful ingest deletes is the drift the reconciliation
above exists to catch.
**Cases:** E2E-20.
**Rejects:** the sentence at `0007:36-38`, "The runner ADR-0006 relocated
therefore cannot be written against the port as it stands — in either crate",
and the phase-2 work item `RUNBOOK.md:204-208` derives from it. The pump was
compiled with a real body — `checkpoint` → `begin` → per-event `apply` →
`commit`/`rollback` — because the closure's caller knows the concrete `Batch`.
Leaving the claim standing sends the port-freezing phase after the wrong
problem, and the right problem (§4.3) is the one that decides whether the freeze
is worth anything.

**PS-33 — ADR-0007's own falsifier MUST be evaluated at a named phase exit: if
the contract crate's checkpoint pump has acquired no caller other than the
typed runner by the time the typed-layer phase exits, the pump collapses upward
into `happenstance` and ADR-0007 is superseded.**
`[NON-NORMATIVE — the falsifier was evaluated at the typed layer's phase exit,
the verdict is recorded below, and a phase gate no adapter can fail belongs in
prose rather than in the clause space. The ID is retained so that citations
resolve.]`
**Rule:** none; it is a phase gate, not an adapter obligation, and no adapter can
fail it. It was recorded as a clause because `0007:119-121` sets the falsifier,
no phase evaluated it, and an unevaluated falsifier is indistinguishable from
none. That is no longer the case, which is why the clause can now leave.
**Cases:** none directly; it decided where the code E2E-26 through E2E-28 test
lives.
**Rejects:** keeping a seam because it was argued for — which is ADR-0007's own
phrasing and its own risk. The fallback is already written down and is the ADR's
narrowly-rejected alternative: one runner, in `happenstance`, with the
checkpoint invariant living one crate above the port that states it.

**Verdict, taken at the typed layer's phase exit: the falsifier fired, and the
pump collapses upward.** The count was taken over the tree rather than
remembered. `happenstance-core` publishes exactly two free functions — `collect`
(`crates/happenstance-core/src/store.rs:333`) and `read_decision_model`
(`:369`) — and **neither is a checkpoint pump; there is no pump function in the
contract crate at all.** So the caller count is not zero over a function that
exists, it is unavailable over a function that never landed, and both readings
fire the same falsifier: the pump has acquired no caller but the typed one,
because it has acquired no caller and no body. The runner an application calls
is `happenstance::run_projection`
(`crates/happenstance/src/runner.rs:401`), which drives the port's `checkpoint`,
`begin`, `commit` and `rollback` directly, one crate above the port that states
the invariant.

**The verdict is executed as ADR-0007's own narrowly-rejected alternative, and
it was staged rather than written here.** The superseding decision — one runner,
in `happenstance`, with `happenstance-core` keeping the port and no pump — was
staged for the human-invoked `/redkiln:kb-ingest`, and that wave ran on
2026-08-17: it is now the **accepted** atom `kb-decision-0031`
(`.kb/decisions/0031-the-runner-collapses-upward.md`). It was staged rather than
authored because an accepted decision atom is immutable and hand-writing one
produces the directory layout of the process without the process; ADR-0007's
Context is corrected by supersession, never by edit
(`.kb/governance/rewrite-the-referent-never-the-reasoning.md:50-59`), as PS-32.

**Why this clause leaves the space rather than becoming `[FROZEN]`.** Its
subject was a phase's obligation to look, and the phase has looked. There is
nothing left for an adapter to satisfy or violate, and §7.3 has recorded since
this document's first assembly that it *should be prose* and was kept only
because the work list did not exist. It does now, and this is it.

**PS-34 — If PS-5 is falsified and `Batch` keeps its lifetime, the port MUST
document that an implementer has to spell the parameter `Self::Batch<'_>`
literally, and MUST carry a doctest that does.**
`[PROVISIONAL — contingent on PS-5; dead the moment `type Batch;` lands]`
**Rule:** a doctest on `ProjectionStore` implementing the port for a toy store,
which cannot rot because CI runs it.
**Cases:** E2E-24.
**Rejects:** the current state, which is the entire explanation for zero
adapters. Writing the impl with the concrete type gives
`error[E0195]: lifetime parameters or bounds on method 'commit' do not match the
trait declaration`, pointing at `projection.rs:128`, and nothing in the crate or
the workspace says the literal `Self::Batch<'_>` is required. `MemoryEventStore`
exists partly to be something an adapter author can copy (`memory.rs:16-25`);
the projection port has no equivalent, which is why `MemoryProjectionStore`
belongs in the same phase as the freeze rather than after it.

---

### 4.10 Derivation and the two flavours

`projection.rs:87` carries `#[trait_variant::make(SendProjectionStore: Send)]` —
the identical construction to `store.rs:141`. ADR-0001 argued the scheme for
`EventStore` alone and never mentioned this port, so until phase 2 the attribute
appeared in no ADR at all: two ports with one construction between them and one
ADR covering half of it. That gap is what this subsection was written to close,
and [ADR-0008](../.kb/decisions/0008-one-derivation-for-both-ports.md) closed it by taking
both ports together. What the subsection holds open now is the *joint*-ness, not
the omission. The *value* of the bound is section 3's and is not re-decided here.

**PS-35 — The derivation decision MUST cover both ports in one ADR.
`EventStore` and `ProjectionStore` MUST NOT be given different flavour
schemes.**
`[NON-NORMATIVE — the artefact is an ADR, and a clause constraining the next
pass's paperwork is a work item rather than a constraint on any implementation.
Both ADRs it demanded exist, so the work item is closed; the ID is retained so
that citations resolve, and what the clause forbids survives as prose below.]`
The experiment this clause was held open for — one skeleton per adapter shape,
declaring its real future and stream types with `todo!()` bodies, which is the
only way to learn whether any real adapter produces a `!Sync` future — ran at
phase 2, and both halves of the demand are now discharged in documents that hold
both ports at once. [ADR-0008](../.kb/decisions/0008-one-derivation-for-both-ports.md)
takes the derivation scheme for `EventStore` and `ProjectionStore` together and
records where their consequences diverge;
[ADR-0009](../.kb/decisions/0009-error-send-sync.md) does the same for the `Error` bound
(ES-6), and there the consequences do not diverge at all.
**Rule:** none at adapter level; the ADR is the artefact.
**Cases:** E2E-30, E2E-52, E2E-53.
**Rejects:** settling `EventStore`'s bounds and leaving
`projection.rs:87` undiscussed — which was the state of the workspace when this
clause was written, two ports with identical constructions and one ADR between
them, and which ADR-0008 ended by arguing the scheme for both at once. What the
clause forbids from here is the same split arriving later: a change that gives
the two ports different flavour schemes, or that revisits one port's derivation
in an ADR which does not say what happens to the other. An application holding a
`!Send` projection store and a `Send` event store is not exotic — Norvant's
rusqlite and Ladybug batches sit beside Postgres ones — so the two decisions
compose at every call site and must be taken together.

**PS-36 — The `Send` flavour transitively requires `Batch: Send`, and the port
MUST document it rather than leaving it to be discovered.**
`[FROZEN]`
**Rule:** none that gates, and the reason is a finding rather than an omission.
The `compile_fail` doctest this clause first named cannot be pinned as specified:
the diagnostic carries no error code at all — `--message-format=json` reports
`code: None` — and rustdoc on stable 1.97.1 *silently ignores* a
`compile_fail,E0308` annotation, passing a doctest annotated with a code the
diagnostic demonstrably does not have. So the annotation asserts nothing and the
bare `compile_fail` passes on any compile error, a typo in the example included.
The doctest is worth keeping as documentation; it is not a gate. Pinning the
diagnostic needs a `trybuild`-style stderr snapshot, which is a dependency
decision phase 6 owns (ADR-0008:234-240).
**Cases:** E2E-30.
**Rejects:** the assumption that runner topology is a free choice. `Batch`
appears in `commit`'s and `rollback`'s parameter lists, so on
`SendProjectionStore` the batch is captured by a future the attribute has marked
`Send`, and the compiler reports *"future returned by `commit` is not `Send` …
captured value is not `Send`: `_batch`"* — at the adapter, not at the port. The
bound cannot be stated on one flavour only: `trait_variant` copies
associated-type bounds verbatim, so `type Batch: Send;` would impose `Send` on
the `!Send` flavour too and break the wasm target. Documentation is therefore the
only available mechanism, and its absence decides deployments: a `!Send` batch
forces the graph projections and the on-device SQLite store onto `LocalSet` or
`spawn_blocking` while the Postgres ones spawn normally, and Norvant's most
expensive `apply` — about 90 ms — is a graph projection.

**PS-37 — Any provided or extension method on `ProjectionStore` whose body holds
`&self` across an await MUST require `Self: Sync` at the point of use, and its
body MUST type-check under both flavours' bounds simultaneously.**
`[FROZEN]`
**Rule:** none at adapter level. The obligation is on the contract crate, and the
artefact that discharges it is a generic helper compiled against both flavours
in the crate's own tests.
**Cases:** E2E-52, E2E-53.
**Rejects:** the belief — refuted by compilation, PRESSURE-TEST §1 — that
`trait_variant` forbids provided methods outright. It does not: the
hand-desugared form
`fn f(&self) -> impl Future<Output = T> where Self: Sync { async move { … } }`
compiles under the existing attribute and its future is `Send` in generic code.
Only the `async fn` spelling fails, with
`error[E0728]: await is only allowed inside async functions and blocks`. The
obligation nobody has written down is the second half: `variant.rs` clones a
provided body into the derived trait, so one body must satisfy the bare
flavour's bounds and the `Send` flavour's bounds at once. A provided method that
compiles against `ProjectionStore` and not against `SendProjectionStore` is a
compile error in the contract crate, not in an adapter, and it will be found by
whoever adds the first defaulted method rather than by whoever designed it.

---

### 4.11 The suite this section obliges

Seventeen rules, emitted by `projection_store_conformance!` through the same
registry macro `event_store_conformance!` uses, so it inherits the tokio,
blocking and wasm flavours without a second mechanism.

**All seventeen now exist.** `for_each_projection_store_rule`
(`crates/happenstance-testkit/src/projection.rs:1884-1919`) is the single
enumeration they are emitted from, and `no_orphan_projection_rules` holds that
list and the module's own rules to each other in both directions — so the table
below is checkable against one place rather than counted by hand. Two sentences
have stood here and both were true when written: *"Every one is new"*, and the
one that said sixteen of the seventeen existed.
`fresh_projection_has_no_checkpoint` was the last to land, and it landed against
PS-38 rather than by widening `[FROZEN]` PS-19 — no rule in the table below is
daggered in §7.2.

| Rule | Clause | Rejects |
|---|---|---|
| `fresh_projection_has_no_checkpoint` | PS-19, PS-38 | a store that reports `Live { through: FIRST }` for an id it has never seen |
| `commit_advances_the_checkpoint` | PS-1, PS-38 | — (the baseline the rest are differential against) |
| `commit_is_atomic_with_the_read_model` | PS-1, PS-4, PS-11 | checkpoint on one connection, rows on another |
| `failed_commit_leaves_both_unchanged` | PS-1 | a partial apply that reports failure |
| `rollback_leaves_both_unchanged` | PS-8 | a rollback that only discards the buffer |
| `dropped_batch_leaves_store_usable` | PS-7 | a pooled connection `Drop` never returns |
| `distinct_projections_advance_independently` | PS-23 | a single-row checkpoint table |
| `commit_accepts_a_position_the_batch_did_not_write` | PS-21 | an adapter that validates the position |
| `commit_rejects_a_regressing_position` | PS-22 | unconditional `UPDATE checkpoint SET position = ?` |
| `commit_rejects_a_foreign_batch` | PS-15 | every adapter writable today |
| `reset_clears_rows_and_checkpoint_together` | PS-16 | the two-statement runbook procedure |
| `reset_is_scoped_to_one_projection` | PS-17 | a checkpoint-table truncate |
| `reset_is_not_commit_at_first` | PS-19, PS-20 | `checkpoint.unwrap_or(FIRST)` with an inclusive `from` |
| `refused_reset_changes_nothing` | PS-18 | a refusal reported as success |
| `batch_reads_reflect_pending_writes` | PS-12 | a batch `get` answering from committed state |
| `rebuild_is_chunk_size_invariant` | PS-13, PS-14 | a projection reading out of band |
| `rebuilding_is_distinguishable_from_live` | PS-24 | rebuild in place with one position field |

Three hostile stores belong in `crates/happenstance-testkit/tests/`, because a
rule that has never failed anything is a rule nobody has checked:
`CheckpointOnlyStore` (commits the checkpoint, discards the write set),
`TruncatingResetStore` (a `reset` that clears every id), and
`ValidatingCommitStore` (rejects a position the batch did not write). Each is a
plausible first cut, and each must fail the suite.

They are mutants in CF-1's sense and take CF-1 through CF-5 unchanged: registered
as data with the exact rule set each fails (CF-2), asserted in both directions so
a mutant broken in more ways than it claims is caught (CF-3), and carrying the
provenance string that names the adapter shape making it plausible (CF-4). The
projection suite also owes CF-5's conformant variant — a store legally different
from `MemoryProjectionStore` that passes everything — and the obvious one is the
buffering adapter PS-4 permits, which doubles as the far end of §6's batch-shape
axis.

Six further rules are integration-level and belong in the workspace e2e crate
rather than the adapter suite, because they need a runner and a domain:
`failure_policy_is_per_projection`, `skip_and_record_is_atomic`,
`pump_reports_the_failing_position`,
`one_poisoned_projection_does_not_stall_the_others`,
`panicking_apply_rolls_back`, and `checkpoint_lag_is_not_a_position_difference`
— the last belongs to ES-31, which states it from the event-store side and names
this suite as its home. CF-36 is why they are listed apart: a clause backed only
by integration-level cases must not name an adapter conformance rule.

### 4.12 What this section does not settle

Three questions are named here so that they are not settled in passing by
whoever implements the rest.

**Whether two views in one store can advance in one transaction** (PS-23). The
answer is no for 0.1 and the falsifier is a real pair of read models that must be
mutually consistent at every instant. Nobody has produced one.

**Whether rebuild-in-place or rebuild-and-swap is the correct procedure**
(PS-24). `Authority::Rebuilding` makes in-place honest; it does not make it
right. The swap protocol is already permitted by the port and documented
nowhere, and the discriminating constraint is storage rather than semantics.

**Whether the port needs a write vocabulary after all** (PS-9). The decision
rests on there being exactly one generic consumer — the suite — and the probe
serving it. A second generic consumer inside happenstance reverses it, and the
two candidates are both plausible enough to name.

---

## 5. The `SyncPeer` port

`happenstance-sync` is now a phase-2 sketch rather than an empty crate: two
ports in two flavours each — `SyncPeer`/`SendSyncPeer`
(`crates/happenstance-sync/src/peer.rs:81-82`) and
`IngestStore`/`SendIngestStore`
(`crates/happenstance-sync/src/ingest.rs:119-120`) — a `memory` reference peer, a
three-variant `SyncError` (`peer.rs:393`), and two stand-in peers in the
crate's own `tests/` chosen to be as unlike each other as the deployment
allows. The prose that used to end *"None of this is settled"* was replaced by
a per-question ledger (`crates/happenstance-sync/src/lib.rs:106-132`) recording
which of the six questions the sketch answered and which it left alone. None of
it is the protocol; it was built to be falsified by a type checker. This
section is still what settles the shape, and what changed is that several of
its clauses now have something in the tree to disagree with.

It was written before that code existed, and the clauses that depend on
measurement rather than on argument are consequently `[DEFERRED]` against the
phase that builds the port against two real peers — five of the thirty-five,
with a further nine `[PROVISIONAL]`. The rest are frozen, because the shape
does not wait on the transport. They are written anyway, and the reason is
recorded in the decision ledger: `RUNBOOK.md:438-439` says deferring the
sync port **"leaks `EventId` and a tail seam back into `EventStore`"**. That was a
claim about coupling, and phase 2 compiled it rather than arguing it: `impl
IngestStore for MemoryEventStore` — local trait, foreign type — compiles with
`happenstance-core` untouched, so the *trait* seam is discharged by coherence
and `append` keeps its signature. The **value-type** seam is not:
`SequencedEvent` has nowhere to hold an `EventId` it has accepted, so the body
is `todo!()` and would be `todo!()` with unlimited time
(`crates/happenstance-sync/src/ingest.rs:14-50`). The leak is smaller than the
warning claimed and real, and it is a claim about a struct's fields rather than
a port's signature — which is the cheaper of the two to land, and is VT-5's. A
deferral you have not written down is not a deferral; it is a decision the next
pass makes by accident, in the crate that can least afford it. So the port's
shape is specified here, and the *experiments* are deferred — not the shape.

**Phase 5 added a fifth module, and this section's inventory is the only thing
that moves for it.** `crates/happenstance-sync/src/wire.rs` — 359 lines,
`pub mod wire` at `lib.rs:147`, beside `identity`, `ingest`, `memory` and `peer` —
holds `FORMAT_VERSION`, the `Envelope<T>` that stamps it, its hand-written
`Deserialize`, and `WireError`, with two tests in
`crates/happenstance-sync/tests/wire.rs`. Nothing below restates it, because
nothing below owns it: the obligation the module discharges is **WF-8** in §2.7,
which specifies the check, names the two rules and cites the impl. What landed is
the envelope and not the message set — `PushBatch`, `EventGroup` and
`ReplicatedEvent` carry no derives and did not gain any (`lib.rs:86-94`), and
`SyncError` was not extended, a version refusal being `wire::WireError` rather
than a new variant — which is why the wire clauses stayed in §2.7 and why none of
this section's bound decisions moves for it. What is inventoried here, and only
here, is that the crate has a wire module at all, and that its `tests/` now holds
four files: `real_peer_shapes.rs`, which is the two stand-in peers named above,
`cursor_shape_probe.rs`, `ingest_reaches_a_foreign_store.rs`, and WF-8's
`wire.rs`.

Three inputs bound this section and are not re-opened in it:

- **The wire format is private to happenstance** (section 2, WF-1). The `serde`
  feature moves events between happenstance instances; it is not an
  interoperability surface. DCB wire interop is deferred to a future
  `happenstance-dcb-interop` crate, which would sit beside the peer adapters
  rather than inside the port — see WF-1 for the deferral and its owner. Nothing
  in this section may be read as a claim about interoperating with another DCB
  implementation.
- **Event identity is store-assigned** (section 2, VT-5): an `EventId` newtype
  over the pair `(StoreId, SequencePosition)` on `SequencedEvent` — the
  incarnation of the store that first accepted the event and the position that
  store assigned it — per `RUNBOOK.md:476`. VT-6 fixes what a `StoreId` names
  (a store incarnation, never a device and never a peer), VT-8 makes uniqueness a
  store-level guarantee, and VT-10 puts the operation that carries a *foreign*
  identity into an `IngestStore` trait in this crate rather than on
  `EventStore::append`. Every idempotence clause below depends on all four and
  none re-decides them.
- **Ingest is unconditional, with compensation.** That is SY-1 through SY-7, and
  it is the clause every other clause in this section is derived from.

---

### 5.1 The bound decision: ingest never rejects

The catalogue produced three answers from three domains and
`E2E-CASES.md:1592-1596` records them as mutually contradictory. They are not.
Two are the same answer and the third is a case where that answer costs nothing.

**Kestrel Rotor's argument is the general one, and it wins on generality alone.**
Rejection is a function of local state. Peer *A* holds a matching event and
rejects; peer *B* does not and accepts; the union of facts is now unreachable
from either side, and no amount of further syncing repairs it, because each
subsequent exchange re-runs the same local predicate against the same divergent
state. Convergence dies the moment ingest is allowed an opinion. This is not a
statement about Kestrel Rotor's spares logistics — it is a statement about what a
predicate over local state does to a replicated set.

**Kestrel Cold Chain's argument is not the opposite of it.** Cold Chain needs the
hub to *notice* a conflict and do something about it, and what it does is
`append(&[losing_event, PartClaimSuperseded], Some(&guard))` — one call,
all-or-nothing by `store.rs:189-192`, conformance-tested by `append_is_atomic`
and by `append_is_atomic_under_a_mid_batch_fault`. That is an
**append**. It adds a fact; it refuses nothing;
it deletes nothing. Both events land at the receiver and both are forwarded
onward like any other. Convergence is untouched. What Cold Chain calls
"re-checking" is a domain decision *about* facts already accepted, running after
the ingest, not a gate in front of it.

**Wattline is the case where the compensation is provably never needed.**
`SiteLeaseGranted` pre-partitions authority: the hub grants a site *N* fleet
slots before the offline writes happen, so no two peers can author conflicting
facts in the first place, and there is nothing for an adjudicator to adjudicate.
That is not a fourth policy. It is SY-1 with an empty conflict set, which is what
a well-designed vocabulary buys you.

The alternative that reads as the safe middle — quarantine — is worse than
either, and the reason is structural rather than a matter of taste. It is
SY-4.

---

**SY-1. Ingest MUST NOT refuse a replicated event for any reason that is a
function of the receiving store's state.**

An ingesting peer MUST append every event it receives that it does not already
hold. In particular it MUST NOT evaluate an `AppendCondition` — its own or the
origin's — as a precondition on the append.

`[FROZEN]`
Rule: `ingest_never_rejects` (new, `happenstance-sync-testkit`) — two peers each
accept a conflicting fact under byte-identical position-free conditions; assert
both facts are present in both logs after a full exchange.
Cases: E2E-33, E2E-39, E2E-42, E2E-45, E2E-47.

*Rejects:* an ingest that calls `append(events, Some(&origin_condition))` and
routes `AppendError::ConditionViolated` into a rejection path. The crate has
since taken the opposite position in prose — the origin's condition travels
as **evidence, not as an instruction**
(`crates/happenstance-sync/src/peer.rs:230-235`) — and that makes this
rejection more necessary rather than less, because prose is not a type
constraint: `EventGroup::guard` is a **public** `Option<AppendCondition>` field
on the wire type (`peer.rs:236-243`), so a receiver is handed exactly the value
it would need to re-evaluate, and whether it may use it for anything at all is
the crate's own stated open question. It is the natural first cut, because the
condition arrives on the wire already (`append.rs:286-291`) and the receiving
store's `append` will happily take it. It passes every event-store conformance
rule, because it is one correct `append` call. It produces a peer set that
never converges, and nothing in the workspace today can observe that.

---

**SY-2. Where the receiving peer's domain determines that an ingested event
conflicts with a fact it already holds, the compensation MUST be appended in the
same `append` call as the losing event.**

The losing event and its compensation are one batch:
`append(&[losing, compensation], Some(&guard))`. A reader MUST NOT be able to
observe a state in which the log holds the losing event with nothing resolving
it.

The store-side half of that guarantee is ES-18's, and phase 3 changes which rule
carries it. The rule that can actually reach a partly-written batch is the one
that injects a fault mid-append; a condition violation rejects before anything is
written, so it can never produce the state this clause's reader must not observe.
ES-18 owns the replacement and disposes of what it replaces.

`[FROZEN]`
Rule: `compensation_is_atomic_with_the_losing_event` (new,
`happenstance-sync-testkit`), riding for its store-side half on ES-18.
Cases: E2E-39.

*Rejects:* compensate-after-commit — ingest the losing event, return, then have a
projection or a follow-up command author the compensation. It is the obvious
shape because the adjudication is a domain decision and the ingest path is
generic. The window it opens is small and real: a device syncing inside it cuts
its next slice from a hub log that says one physical compressor is held twice,
and the projection that exists to prevent the conflict is what manufactures it.
The failure survives the window closing, because the slice does not.

---

**SY-3. The port MUST supply atomicity, identity and idempotence for a
compensation. It MUST NOT supply the compensation's content.**

The compensating event is constructed by domain-supplied code and handed to the
port as an `Event`. The port supplies: the single-batch guarantee (SY-2), the
`EventId` that makes re-authorship detectable (SY-11), and the guard condition
that makes re-authorship a no-op.

`[FROZEN]`
Rule: `compensation_is_idempotent_under_redelivery` (new,
`happenstance-sync-testkit`) — deliver a conflicting group twice; assert exactly
one compensation exists.
Cases: E2E-33, E2E-39.

*Rejects:* the adjudication seam as a trait method returning a domain type. A
port method `fn compensate(&self, losing: &Event) -> Event` forces the port to
name — and therefore to know — a domain vocabulary, which is ADR-0003's whole
prohibition arriving through a different door. The seam is a caller-supplied
closure for the same reason ADR-0007's `pump` takes one
(`references/adr/0007-projection-runner-decodes.md:62-67`): the caller knows the
concrete domain and the port does not, and a higher-ranked `FnMut` lets the
caller keep that knowledge without the port ever naming it.

> **[NON-NORMATIVE]** What a compensation *means* is the domain's. Kestrel Cold
> Chain's `PartClaimSuperseded` says a claim lost an adjudication; a different
> domain might emit an approval task, a credit note, or nothing at all where the
> conflict is tolerable. No rule can check the meaning of a domain event, and a
> clause asserting one would be decorative. It is prose.

---

**SY-4. A peer MUST NOT quarantine an ingested event.**

There is no holding area. Every accepted event is appended to the local log and
receives a local position.

`[FROZEN]`
Rule: `transitive_convergence_over_a_partial_mesh` (new,
`happenstance-sync-testkit`) — the A—B—C topology of SY-24 fails outright against
a quarantining peer.
Cases: E2E-42.

*Rejects:* the quarantine-and-review design, which is what an operations team
asks for and what a careful engineer reaches for first. Its defect is not
policy, it is arithmetic: a quarantined event has no local position, so it is
invisible to the scalar cursor a downstream peer resumes from (SY-16), so it is
never forwarded, so in an A—B—C topology where *A* and *C* never meet it
disappears permanently from *C*'s view. Fixing that by making the quarantine
itself an appended fact produces unconditional append with extra machinery and a
misleading name.

---

**SY-5. Ingest MUST append at the tail. An ingested event MUST NOT be assigned a
position below any position the store has already assigned.**

`[FROZEN]`
Rule: `ingested_events_land_above_the_local_head` (new,
`happenstance-sync-testkit`); anchored on positions the store actually assigned,
per CLAUDE.md's gap-permission rule.
Cases: E2E-42.

*Rejects:* order-preserving insertion — merging the incoming log into the local
one by origin timestamp or by origin position, so that a replicated event lands
where it "belongs". It is the intuitive merge and it silently destroys every
projection on the store: `ProjectionStore::checkpoint` is one scalar that
`ReadOptions::from` resumes at (`projection.rs:454-455`), so an event inserted
below an existing checkpoint is never read, never applied, and never reported
missing. The port has never chosen between the two, and this is the choice.

---

**SY-6. A wire-carried `AppendCondition` is evidence, not an instruction. Ingest
MUST NOT evaluate it as a local append condition, and a condition carrying a
position-relative boundary — any guard whose `after` is `Some(_)`, under VT-30's
multi-guard shape — MUST be refused as ingest input.**

The origin condition travels so that an adjudicator can see what the origin
decided against. It is not executed.

`[FROZEN]`
Rule: `wire_condition_with_after_is_refused` (new, `happenstance-sync-testkit`);
ES-29 cites the same rule from the store side.
Cases: E2E-37, E2E-38, E2E-56, E2E-14.

E2E-14 is the case this clause finally gives a home to, and it is the only case
in the catalogue that no other clause claims. Its own text says why: "an
unconditional append is distinguishable from a forgotten condition" rejects
nothing on its own and "earns its keep only if the sync port classifies
conditions by replicability". This clause is that classification — a condition
carrying `after` is unreplicable and refused, a position-free one is evidence an
adjudicator may read — so the taxonomy E2E-14 asks for exists here, at the wire,
rather than as a named constructor in the contract crate. What the clause does
**not** supply is the greppable local distinction E2E-14 also wants between "I
asserted nothing" and "I forgot to assert"; that remains a typed-layer concern,
and no clause in this specification claims it.

*Rejects:* the receiver that re-evaluates the origin's condition verbatim,
which is what Kestrel Cold Chain's D5 specified and what the wire type still
permits: the origin's condition is a **public** field on `EventGroup`
(`crates/happenstance-sync/src/peer.rs:236-243`). The crate's own prose now
rules it out — "evidence, not an instruction" (`peer.rs:230-235`) — and rules
it out on the first of the two defects below, but a doc comment does not stop
an implementer and the crate says so, calling what a receiver may do with the
guard its central open question. Two independent defects, either fatal:

- `after` is a `SequencePosition`, meaningful only inside the store that
  assigned it (`crates/happenstance-sync/src/lib.rs:100-104`), and it serialises
  as a naked integer (`append.rs:286-291`). `is_violated_by` compares raw
  position values (`append.rs:239-253`), so `after: 288455` interpreted in the
  receiver's numbering names an unrelated recent event and the check runs over
  an arbitrary tail and passes **vacuously**. That is worse than no check,
  because it looks like enforcement.
- Even with `after: None`, re-evaluation is not idempotent. On first delivery the
  receiver accepts and writes; on re-delivery the same condition now matches the
  receiver's own copy, returns `ConditionViolated`, and the receiver adjudicates
  *against the event it just accepted* — an inversion, not a duplicate. This is
  `E2E-33`, the sharpest single case in the catalogue.

The refusal is mechanically available today, and VT-30 is why it is `Guard`'s
property rather than `AppendCondition`'s: `Guard` is `#[non_exhaustive]` with
**public** fields (`append.rs:121-141`), so a peer can read `after` and reject on
it while being unable to construct the struct literally, while
`AppendCondition`'s own `guards` field is private and readable only through
`guards()` (`append.rs:102-119`, `:179-183`).
Readable-but-not-literal-constructible is precisely what makes this
checkable. WF-4 is the other half: `after` is **always
present** on the wire, never elided, because a policy can only refuse what it can
see. ES-29 states the same refusal from the store side.

---

**SY-7. Compensation authorship MUST be assigned to at most one peer per fact
family. A peer that is not the assigned adjudicator for a family MUST NOT author
a compensation for it.**

A fact family is identified by tag key (SY-35). Assignment is runner
configuration, not a property of the port.

`[PROVISIONAL — falsified if a deployment exists in which two peers must both be
able to adjudicate the same fact family and their compensations can be made
convergent. Falsification test: two adjudicators for one family, disconnected
from each other, each ingesting the same conflicting pair; assert the two logs
converge after they meet.]`
Rule: `only_the_adjudicator_compensates` (new, `happenstance-sync-testkit`).
Cases: E2E-39, E2E-42, E2E-45.

*Rejects:* every-peer-adjudicates, which is the reading SY-1 through SY-3 invite
if nothing forbids it, and which the Kestrel Rotor writeup correctly identifies
as the failure mode of unconditional-append-plus-compensation: every peer that
ingests the conflicting pair emits its own compensation, and the log accumulates
*N* compensations for one conflict. Guarding emission with a conditional append
does not fix it — a conditional append elects one winner only among the peers
currently connected, and the whole premise of this crate is that they are not.
The fix is not a better guard; it is that adjudication is a **role**, and a role
is exactly the kind of thing that belongs to a runner rather than to a transport.

---

### 5.2 One peer, and a runner above it

`crates/happenstance-sync/src/lib.rs:40-48` already states this and states it correctly. It is promoted
to a clause because the pressure test's own finding about the projection port —
that the *suite*, not the runner, is what forces a port's shape
(`PRESSURE-TEST.md:220-234`) — applies here identically, and because Kestrel
Rotor supplies the case that makes it non-obvious.

---

**SY-8. The `SyncPeer` port MUST describe exactly one peer relationship.
Fan-out across several peers, ordering between them, and reconciliation of
disagreement MUST live in a runner above the port.**

`[FROZEN]`
Rule: the whole of `happenstance-sync-testkit` — every rule in it takes exactly
one peer handle. A rule that needed two would be testing the runner.
Cases: E2E-33, E2E-35, E2E-36, E2E-42.

*Rejects:* a port method taking a peer set, or a `merge_policy` associated type.
Both make every adapter author — including the one writing a 200-line HTTP
client — inherit the merge problem, and both make the conformance suite a test of
a policy rather than of a transport. It is the same division of labour ADR-0007
draws for the projection runner
(`references/adr/0007-projection-runner-decodes.md:44-50`), and for the same reason:
"add a second peer" should be a runner configuration, not a breaking change to a
published trait.

---

**SY-9. Hub-ness MUST NOT be a property of the port's type or constructor.**

One adapter type MUST be usable simultaneously as a hub to one set of peers and
as a symmetric peer to another.

`[FROZEN]`
Rule: `one_adapter_serves_both_roles` (new, `happenstance-sync-testkit`) — a
compile-and-run test that constructs one peer adapter twice, drives one edge in
each role, and asserts both exchanges complete.
Cases: E2E-42, E2E-45.

*Rejects:* `Peer::new(is_hub: bool)`, or separate `HubPeer` / `SpokePeer` traits.
Kestrel Rotor is the case: the vessel is a hub to nine technician tablets over
ship's wifi and a symmetric peer to a shore depot over satellite, *at the same
time*. Hub-ness is a property of an **edge**, not of a node. A boolean on the
constructor forces the vessel to hold two incompatible adapter instances over one
store and gives neither of them a name for what the other is doing.

---

**SY-10. Hub-and-spoke and peer-to-peer are both first-class. The runner MUST
permit a different merge rule in each direction of one edge.**

`[PROVISIONAL — falsified if a hub and a spoke can be shown to need the same
merge rule in a deployment where the hub holds every log and the spoke holds
one. Falsification test: implement the hub's rule on a spoke and show the spoke's
projections stay correct.]`
Rule: `directional_merge_rules_compose` (new, `happenstance-sync-testkit`).
Cases: E2E-42, E2E-44, E2E-45.

*Rejects:* treating hub-and-spoke as peer-to-peer with one side declining to
push. `crates/happenstance-sync/src/lib.rs:57-63` already says why and the scenarios confirm it: the hub
sees every log and can therefore adjudicate; a spoke sees one and cannot. Kestrel
Cold Chain's edge tier is strictly hub-and-spoke for a *commercial* reason — a
spoke's slice is a confidentiality boundary, and Scottish subcontractors must not
hold Yorkshire's parts pricing — so peer-to-peer between spokes would dissolve a
constraint that has nothing to do with topology. A design that models the
asymmetric case as a degenerate symmetric one has no place to put that.

---

### 5.3 Identity and idempotent ingest

---

**SY-11. Re-delivery of an event the receiver already holds MUST be a no-op: no
second copy, no compensation, no error.**

`[FROZEN]`
Rule: `redelivery_of_an_accepted_group_is_a_no_op` (new,
`happenstance-sync-testkit`).
Cases: E2E-33, E2E-34, E2E-36.

*Rejects:* the design as Kestrel Cold Chain wrote it, and the failure is not the
one anybody expects. Its idempotence guard is a condition keyed on the
`supersedes:` tag, which protects the **compensation** path and leaves the
**accept** path unguarded. Re-delivering an accepted group therefore does not
produce a duplicate — it produces an *inversion*, because the origin condition
now matches the receiver's own copy and the receiver supersedes the event it
previously accepted. A duplicate is visible; an inversion looks like a decision.

---

**SY-12. The identity a peer dedupes on MUST be the store-assigned `EventId`,
MUST travel on the wire beside the event, and MUST be reachable by the receiving
peer without decoding `Event::data` or `Event::metadata`.**

`[FROZEN]`
Rule: `dedupe_reaches_identity_without_decoding` (new,
`happenstance-sync-testkit`) — the fixture supplies payloads and metadata that
are not valid UTF-8 and not valid in any codec; a conformant peer dedupes anyway.
Cases: E2E-33, E2E-34, E2E-36.

*Rejects:* a peer that carries the identity in `Event::metadata`. Metadata is
opaque `Bytes` (`event.rs:325`, `:400-401`) and `QueryItem::matches` filters on
type and tags only (`query.rs:113-116`), so the identity a peer is instructed
to carry is structurally unreachable from anything the port exposes. A peer
that must parse opaque bytes to dedupe has broken ADR-0003 at the exact point
ADR-0003 claims to win — and ADR-0003's own lift condition is
*"`happenstance-sync` round-trips an event between two stores without
deserialising its payload"* (`references/adr/0003-opaque-payloads.md:14-15`). A
metadata-borne identity fails the ADR's own test.

The exemplar was the sync crate's own proposal and it was withdrawn: the crate
now mints `(StoreId, SequencePosition)`
(`crates/happenstance-sync/src/identity.rs:87-101`) and carries it on
`ReplicatedEvent`, reachable without touching a payload byte, which is what
this clause requires. The rejection keeps a live target anyway, and phase 2
sharpened rather than removed it — `impl IngestStore for MemoryEventStore`
cannot be written truthfully because `SequencedEvent` has nowhere to hold an
accepted `EventId` and nowhere to read one back for `holds`
(`crates/happenstance-sync/src/ingest.rs:38-50`). Until VT-5's identity lands
on `SequencedEvent`, `metadata` is the only place an adapter author has left,
and this clause is what stands between them and it.

It also rejects the naïve fix, which is why this clause names `EventId` rather
than "some queryable identity". Promoting identity to a `Tag` makes it queryable
and costs more than it looks: a maximally high-cardinality entry in the column
adapters are told to index; an entry in every `contains_all` merge-scan
(`tag.rs:347-361`); a writer-forgeable identity; and an identity dimension
visible to every tag-only query in the domain. It also has a hole — a peer's own
locally-originated writes would carry no such tag, so the peer could not order
its own events against ingested ones. The mechanism only works if **the store
stamps its own writes at append time**, which is the store-assigned Lamport pair
`RUNBOOK.md:476` already decided.

> **Rust note.** `EventId` is a newtype over the pair, not a type alias, and the
> first half of the pair is a `StoreId` — a store incarnation, not a peer name
> (VT-6). A `type EventId = (StoreId, SequencePosition)` is transparent: every
> function taking one accepts any tuple of that shape, including `(local_store,
> local_position)` — which is precisely the confusion this whole section exists
> to prevent. The newtype also buys the impls: the orphan rule forbids
> `impl Display for (StoreId, SequencePosition)` in the crate that declares
> neither the trait nor the tuple. A newtype makes the type local and the impl
> legal. The consequence for this port is worth naming, because it reads as a
> defect on first encounter: the operator-meaningful peer name — "the Yorkshire
> hub", "tablet 88" — is *not* in the `EventId`, because a peer name must survive
> a restore and a `StoreId` must not. Peer naming is this crate's configuration.

---

**SY-13. A peer MUST NOT derive event identity from a hash over the wire bytes.**

`[FROZEN]`
Rule: `identity_survives_a_tag_reordering_peer` (new,
`happenstance-sync-testkit`) — the fixture peer emits its tags in a
non-canonical order; assert dedupe still works.
Cases: E2E-33, E2E-34.

*Rejects:* content-addressed identity, which is the standard answer in this
problem space and which this contract quietly forbids. `Tags` is canonical by
construction — `sort_unstable` then `dedup` (`tag.rs:481-485`) — and the
deserialiser **re-canonicalises on the way in** so a peer cannot smuggle a
non-canonical `Tags` into memory (`tag.rs:538-545`). Both are correct and both
are load-bearing elsewhere. Their joint consequence is that a foreign peer's hash
over its own serialisation of its own tag order does not agree with the hash the
receiver would compute after canonicalisation, so a content hash is stable only
if every peer canonicalises identically *before* hashing — which is an
unenforceable obligation on code this workspace does not own. A hash whose
stability depends on a foreign implementation's field ordering is not an
identity.

---

**SY-14. Bulk ingest of a batch containing already-seen events MUST complete in a
number of round trips bounded independently of the batch size.**

`[DEFERRED — settled by the experiment named in PRESSURE-TEST.md:688-693 and by
the Kestrel Rotor bulk-ingest measurement: 1,840 events over one-shot HTTP inside
a 34-minute window, against a store-level `EventId` uniqueness guarantee versus a
per-event conditional append. Owning phase: the phase that builds the two peer
adapters.]`
Rule: `bulk_ingest_is_idempotent_in_bounded_round_trips` (new,
`happenstance-sync-testkit`).
Cases: E2E-36, E2E-35.

*Rejects:* all three shapes available against the port as it stands, which is why
this is deferred rather than decided.

- One `append` per event: 1,840 round trips, fatal on the one-shot HTTP peer.
- One condition of 1,840 single-identity items: legal, but one already-seen event
  rejects the other 1,839, and `ConditionViolated.conflicting_position` names at
  most one culprit (`error.rs:136-147`), so recovery is a serial peel.
- Read-then-filter-then-unconditional-append: two round trips, with nothing
  closing the race when a peer ingests from two sources concurrently — the
  `AppendCondition` is the only mechanism that could, and SY-1 has just forbidden
  using it that way.

Two answers were candidates: a per-event conditional append
(`append(&[(Event, Option<AppendCondition>)])`), or a **store-level uniqueness
guarantee on `EventId`** that the port states and the testkit checks. Section 2
has chosen the second — VT-8 makes `EventId` uniqueness a store-level guarantee,
`[FROZEN]`, enforced by the store and not by the caller, and VT-10 keeps the
foreign-identity write off `EventStore::append` entirely. So the *mechanism* is
no longer open and this clause must not be read as reopening it. What is deferred
is narrower and is genuinely a measurement: whether an ingest path built on that
guarantee reaches bounded round trips on a transport with one round trip per
operation, and what an already-written adapter pays to add the unique index. A
negative result does not restore the per-event condition — it changes the ingest
path's shape, or it says the one-shot HTTP peer cannot do bulk ingest at Rotor's
scale, which is a deployment fact rather than a port defect.

---

### 5.4 The transport floor

`crates/happenstance-sync/src/lib.rs:125-132` calls this the constraint the
sketch "bites hardest on", and settles `SyncPeer::pull` on a bounded batch and
an owned resume token rather than a stream because of it. It stays a normative
constraint on the port's shape rather than an adapter's choice, and phase 2
supplied the reason the port cannot carry it alone: **the type checker did not
force the decision.** The cursor shape was pointed at both peers and compiled
against both — the one-shot HTTP peer satisfies `impl Stream` by buffering a
whole response into a `Vec` and replaying it, which is legal, `Send`, and a lie
(`crates/happenstance-sync/src/peer.rs:43-50`, kept compiling as
`crates/happenstance-sync/tests/cursor_shape_probe.rs`). A port that a Neon
peer cannot implement is a port shaped like a Durable Object; a port a Neon
peer can *pretend* to implement is worse, and only a fixture that counts its
own round trips tells them apart.

---

**SY-15. Every port method MUST be completable in one round trip. The port MUST
NOT require a peer to hold state between calls.**

No connection, no interactive transaction, no cursor object, no session.

`[FROZEN]`
Rule: `peer_conformance!` is invoked against a fixture peer whose transport
asserts on its own round-trip count and panics on a second call within one
operation (new, `happenstance-sync-testkit`).
Cases: E2E-35, E2E-36.

*Rejects:* the shape every socket-based design produces: `open()` returning a
handle, `next_batch(&mut handle)`, `close(handle)`. It is the right shape for the
Durable Object reached over a WebSocket and it is unimplementable on the Neon
peer, which reaches its store over one-shot HTTP with one round trip per
operation and nothing to hold a handle *in*. This is the same finding E2E-24
reaches about `ProjectionStore::Batch`, arriving from a different port — and the
workspace should notice that two ports have now independently been asked to stop
assuming a held connection.

> **Rust note.** This is why the port's resume token is an owned value the caller
> passes back in, rather than a `Batch<'a>`-style borrow from the peer. A
> borrowed cursor exists to keep a lifetime alive across `.await` points; a
> one-shot transport has nothing for it to borrow *from*. `ProjectionStore`
> spells its batch `type Batch<'a> where Self: 'a` (`projection.rs:97-99`);
> E2E-24 argues that clause buys nothing on the majority of its targets and PS-5
> removes it, making `Batch` an owned value. Two ports have now independently
> reached the same conclusion, which is the strongest form the evidence takes.

---

**SY-16. Resume state MUST be an owned, transferable value the caller supplies
on each call, not a handle the peer holds.**

`[FROZEN]`
Rule: `resume_survives_a_dropped_peer_handle` (new,
`happenstance-sync-testkit`) — drop the peer between two exchanges, reconstruct
it, resume from the token, assert no gap and no duplicate.
Cases: E2E-35, E2E-36, E2E-44.

*Rejects:* an in-memory cursor, which passes every test written against a
process that stays alive and fails the deployment the crate exists for. Kestrel
Cold Chain's spokes are tablets that lose signal in a −22 °C cold store;
Turnstile's Workers are cancelled mid-flight; a Durable Object is evicted. The
normal termination path at the edge is *the handle going away*, and a resume
token that lives in the handle is a resume token that does not exist.

---

**SY-17. The sync port and its runner MUST be written against `EventStore`, not
`SendEventStore`.**

`[FROZEN]`
Rule: the existing CLAUDE.md rule 4, checked by
`happenstance-sync-testkit` compiling its own suite against a `!Send` fixture
peer holding a `!Send` store behind an `Rc`.
Cases: E2E-52, E2E-30.

*Rejects:* any runner that reaches for `SendEventStore` in order to
`tokio::spawn` a per-peer task. `crates/happenstance-sync/src/lib.rs:122-124` states the wasm32
requirement, but understates it: in Kestrel Cold Chain the `!Send` peer — SQLite
inside a Cloudflare Durable Object — sits in the **middle** of the chain, not at
a leaf. It is a spoke to the Neon estate store and a hub to 138 tablets. A runner
bound on the `Send` flavour therefore excludes the hub from its own topology, and
the exclusion is discovered when the adapter is written, not when the runner is.
`EventStore` is the weaker requirement and accepts both flavours
(`store.rs:26-29`).

---

**SY-18. A peer MUST be able to declare the limits it imposes on an event it can
accept, and the runner MUST be able to read them before pushing.**

`[DEFERRED — settled by building the Turnstile peer D shape: a KV-backed store
with a 128 KiB value cap, against an origin store that has already durably
committed a 340 KB payload. The experiment is whether a capability declaration
prevents the failure or merely relocates it. Owning phase: the phase that builds
the two peer adapters.]`
Rule: `peer_declares_its_own_limits` (new, `happenstance-sync-testkit`).
Cases: E2E-35.

*Rejects:* the asynchronous form of the declaration, which is the shape a peer
with one round trip per operation invites. The sketch has already built what
this clause asks for — `SyncPeer::limits()` is **not** `async`
(`crates/happenstance-sync/src/peer.rs:150-158`) and returns a `PeerLimits`
carrying `max_event_bytes`, `max_batch_bytes`, `max_batch_events` and a
`retention_floor` (`peer.rs:282-302`) — on the reasoning that a runner made to
await a round trip to learn a size cap will skip the check, and a limit nobody
checks is a limit discovered at ingest. What the sketch cannot do is show the
declaration *prevents* the failure rather than relocating it:
`PeerLimits::admits` is explicitly advisory (`peer.rs:319-324`), so the
deferral below is untouched by the type existing. The contract bounds the two
fields no engine struggles with, `MAX_EVENT_TYPE_LEN` and `MAX_TAG_LEN`, both
255 (`event.rs:16`, `tag.rs:16`), and leaves `Event::data` unbounded. So an
event that is durable at its origin can be structurally unrepresentable at a
peer, and the incompatibility is discovered at ingest — after the write has
already committed somewhere else, which is the one moment at which nothing
useful can be done about it.

Section 2 supplies two of the three things this clause needs and stops short of
the third. VT-21 gives every store a **floor** it must accept
(`MIN_SUPPORTED_EVENT_DATA_LEN`, 65,536 bytes) rather than a ceiling, and obliges
a store to document its actual limit; VT-25 gives the refusal a distinguishable
error, `AppendError::ExceedsStoreLimit`, so a sync runner can tell "this will
never be accepted here, park it and tell a human" from "the disk is full, retry".
What is still missing, and is what this clause defers, is the *declaration* — a
value a runner can read **before** pushing rather than an error it discovers
after. The floor makes the declaration meaningful (a peer below 64 KiB is
non-conformant, not merely small); `ExceedsStoreLimit` makes the failure
survivable; neither makes it avoidable.

---

### 5.5 Convergence and the merge rule

---

**SY-19. A replicated event's local position MUST be treated as arrival order.
The specification does NOT require two peers to agree on a total order over the
same set of events, and MUST NOT be read as requiring it.**

`[FROZEN]`
Rule: `two_peers_disagree_on_total_order` (new, `happenstance-sync-testkit`) —
an assertion that the *store* contract survives the disagreement, not that the
disagreement is absent.
Cases: E2E-41, E2E-42.

*Rejects:* the belief that DCB's per-store total order
(`event.rs:215-217`) extends across a peer set. Kestrel Rotor's failure is exactly
this belief holding: the same fourteen events sit at vessel position 38,102 and
depot position 3,918,442, each store internally correct, each peer's total order
naming the other as the loser. There is no expression relating the two numbers
and none can be constructed, because `SequencePosition` carries no origin
(`crates/happenstance-sync/src/lib.rs:100-104`).

---

**SY-20. A projection declared convergent MUST produce byte-identical read models
under any two ingest interleavings that agree on per-origin order and disagree
everywhere else.**

`[PROVISIONAL — falsified if a projection is found that must converge, cannot be
written as a commutative fold, and cannot be excluded under SY-22. Falsification
test: express it as a commutative fold over `EventId`-ordered events, or show
why it cannot be.]`
Rule: `convergent_projection_is_interleaving_independent` (new,
`happenstance-sync-testkit`) — takes a projection and a set of events from two
origins, applies them under two interleavings, asserts byte-identical read
models.
Cases: E2E-41, E2E-22.

*Rejects:* `unit-ledger` — a per-serial state machine folded over
`SequencedEvent::position` — and it is worth stating that this is not a
hypothetical wrong implementation but the one that actually produced Kestrel
Rotor's divergence. More usefully it rejects the *class*: any projection that
reads the position at all. `stock-on-hand` converges because someone chose
addition, not because anything checked, and this clause is what turns that from
luck into a property.

This is the clause that makes convergence **checkable rather than hoped for**.
The premise is now stated where an implementer meets it — `EventId::position`
is documented as the origin's position and "*not where it landed here*", local
position being arrival order and unrelated
(`crates/happenstance-sync/src/identity.rs:116-119`) — and the merge rule that
would reconcile the two is explicitly untouched, with the crate warning that
nothing in it should be read as choosing one (`lib.rs:120-121`). What neither
says is that the acceptance has a price, and that the price is paid by every
fold downstream. This clause is where it is charged.

---

**SY-21. A convergent projection MUST be handed an `EventId`, and MUST NOT be
handed a `SequencePosition`, in the value it folds over.**

`[PROVISIONAL — falsified if a convergent projection needs the local position for
a reason other than ordering. Falsification test: name one and show the need
survives being given `EventId` plus the store's own checkpoint.]`
Rule: `convergent_projection_cannot_observe_local_position` (new,
`happenstance-sync-testkit`) — enforced by the type handed to `apply`, so the
rejection is a compile error rather than an assertion.
Cases: E2E-41.

*Rejects:* ADR-0007's `apply` signature as it currently stands, which hands the
projection `Sequenced<Self::Event>`
(`references/adr/0007-projection-runner-decodes.md:76-81`) — a value carrying exactly
the field a convergent fold must not touch. Handing a projection the one value
that breaks it and documenting that it should not look is not a design; it is a
comment. Removing the field from the convergent path makes the wrong program not
compile, which is the only enforcement mechanism this workspace trusts.

---

**SY-22. A projection MUST be able to declare itself non-convergent, and a
convergence check MUST exclude it rather than fail it.**

`[PROVISIONAL — falsified if every non-convergent projection in the catalogue
turns out to be repairable, making the escape hatch a licence rather than a
necessity. Falsification test: express `cost-layers` convergently.]`
Rule: `non_convergent_projections_are_excluded_not_failed` (new,
`happenstance-sync-testkit`).
Cases: E2E-41.

*Rejects:* a blanket convergence obligation on every projection, which would be a
rule an honest adapter fails. `cost-layers` is the case: FIFO consumption order
**is** the fold, so its answer is a function of the order events are applied in,
and no order exists that two peers can agree on. It is pinned to one depot's
arrival order and declared authoritative by fiat, and that is the correct design
for it. A convergence check with no exclusion mechanism forces such a projection
either to lie or to be excluded by not being registered — which loses every other
check at the same time.

---

**SY-23. Where a deployment needs a peer-independent deterministic order, that
order MUST be `EventId` order — the `(origin, origin_position)` pair, sorted.**

`[PROVISIONAL — falsified if a domain needs a peer-independent order that also
reflects real-world time or causality. Falsification test: name a requirement
that `EventId` order fails and a store-assigned timestamp satisfies.]`
Rule: `event_id_order_is_identical_on_every_peer` (new,
`happenstance-sync-testkit`).
Cases: E2E-41, E2E-43.

*Rejects:* the hybrid-logical-clock proposal, and it rejects it using the
scenario's own refutation. Kestrel Rotor's `conflict-queue` demands an HLC and
states its requirement as byte-identical output on every peer — which needs a
deterministic peer-independent total order, and sorting by the Lamport pair
delivers exactly that. The demand conflates determinism with causality. Only the
second needs an HLC and no stated requirement in the catalogue needs the second.

It does **not** reject the audit question, and this clause must not be read as
answering it. `SequencedEvent` was `{position, event}` and nothing else on disk
when this clause was written, so "which side of midnight did this fall on" was
unanswerable from the log, and every timestamp in all six scenarios is
a writer's clock in an opaque payload — including one drifted 2.4 seconds and one
six minutes fast after a factory reset. **A tiebreak is not a fact about the
world.** The audit answer is a separate decision and section 2 has taken it:
VT-9 adds a store-assigned `RecordedAt`, stamped at local append and preserved
verbatim under ingest, and it is taken in the same pass as `EventId` because both
change `SequencedEvent::new` — then a two-argument `const fn` every adapter and
the testkit called, now four (`event.rs:511-523`). VT-9 also forbids ordering by
it, which is what keeps that decision and this one from colliding: `RecordedAt`
answers the audit question and MUST NOT be a merge rule, and `EventId` order is
the merge rule and
MUST NOT be read as a time.

---

### 5.6 Transitivity

---

**SY-24. A peer MUST forward every event it holds, not only events it
originated.**

`[FROZEN]`
Rule: `transitive_convergence_over_a_partial_mesh` (new,
`happenstance-sync-testkit`) — three peers A—B—C, A and C never communicate, each
peer syncs only with its neighbour; assert all three hold the union.
Cases: E2E-42.

*Rejects:* origin-based forwarding, which is the natural reading of "replicate my
writes" and which leaves *C* never seeing *A* at all. It also rejects
quarantining (SY-4) a second time and from a different direction, which is worth
noting: the two clauses are independent statements that happen to convict the
same implementation, and an implementation that satisfies one and not the other
still fails this case.

---

**SY-25. Arrival order and forwarding order MUST be the same order.**

`[FROZEN]`
Rule: `forwarding_order_is_arrival_order` (new, `happenstance-sync-testkit`).
Cases: E2E-42.

*Rejects:* a peer that forwards in origin order, or that re-sorts its outbound
stream by `EventId`. The proof of transitive convergence is short and depends on
two things being true together: ingest is total and unconditional (SY-1), and it
appends at the tail (SY-5). Given both, an event arriving late from *A* receives
a *high* local position at *B*, and is therefore picked up by *C*'s scalar cursor
into *B*'s local order **even though it is old**. Break the correspondence — sort
the outbound stream by anything other than local position — and *C*'s cursor
skips past it forever. The scalar cursor is what makes the transport cheap; this
clause is its price.

---

### 5.7 Causality

The catalogue contains one clean statement of the problem: `e2`'s author had seen
`e1`, and nothing anywhere records that. `Event` has four fields — type, data,
tags, metadata (`event.rs:321-326`) — and none of them is a parent. The
`AppendCondition` that encoded what the author had looked at is taken as a
parameter, evaluated, and dropped (`store.rs:261-265`); it is not persisted by
anything, anywhere.

---

**SY-26. The contract MUST NOT carry a causal parent on `Event` or
`SequencedEvent`. A domain that needs causality MUST express it as a tag.**

`[FROZEN]`
Rule: `causality_is_expressible_without_a_contract_change` (new,
`happenstance-sync-testkit`) — a fixture domain expresses a parent as a tag,
replicates it across three peers, and reconstructs the causal edge on each
without decoding a payload.
Cases: E2E-42, E2E-41.

*Rejects:* two things, in opposite directions.

It rejects **adding a `parent: Option<EventId>` field**. It would be additive on
`SequencedEvent` (which is `#[non_exhaustive]`, `event.rs:483-484`) but not on
`Event`, and the field would be writer-supplied on a type the store does not
validate, so it would carry an unattested self-report — the same defect Kestrel
Rotor's `conflict-queue` hits when it tries to reason about conditions recorded
in `Event::metadata` by hand. It would also add a second identity dimension to a
contract that has spent this section arguing for exactly one.

It rejects, more importantly, **silence**. A domain that needs a causal parent
has a working answer today and it is one line: put the parent's `EventId` in a
tag. That makes it queryable (`query.rs:113-116` matches on tags), replicable
(tags travel), and reasonable-about without decoding — which is SY-35. The cost
is the cost of any high-cardinality tag and the domain pays it knowingly. What
must not happen is a domain discovering the absence at ingest time and inventing
a metadata convention no peer can read.

---

### 5.8 Scope: whole-log or filtered replication

This is the one place in the section where the honest answer is that nobody
knows, and `PRESSURE-TEST.md:688-693` names it as an experiment rather than a
decision. It is stated in full because the consequences reach into the suite.

---

**SY-27. Whether replication is whole-log or scoped is DEFERRED.**

`[DEFERRED — settled by the experiment at PRESSURE-TEST.md:688-693: build a
spoke holding a deliberately filtered subset of a hub's log and attempt a
position-based resume against it. The instrument the catalogue asks for is the
same one — a testkit-adjacent store that holds only a suffix or a filtered subset
of its own log (E2E-CASES.md:1595-1600). Owning phase: the phase that builds the
two peer adapters, which must also build that instrument.]`
Rule: `scoped_replication_resume_is_sound` (new, `happenstance-sync-testkit`),
unwritable until this is settled.
Cases: E2E-33, E2E-36, E2E-44, E2E-46.

*Rejects:* the assumption both plausible answers make and neither states. A spoke
holding a filtered subset **cannot distinguish "not yet received" from "filtered
out"**, so a scalar position watermark against the hub is unsound: the spoke
cannot tell whether the gap between watermark and next-received is a filter or a
loss. And if replication is scoped, the peer port carries a `Query` — a value
that bounds nothing about its own cost (E2E-40) and that would land in a write
path evaluated under the write lock. Kestrel Cold Chain's spokes hold a 90-day
slice chosen for size *and for commercial confidentiality*, so the scoped answer
is not merely an optimisation the deployment could decline.

---

**SY-28. The sync suite's round-trip rule MUST assert log equality only over the
scope the peers agreed on, never over the whole log, unless SY-27 settles on
whole-log replication.**

`[DEFERRED — same experiment as SY-27; this clause is its consequence and cannot
be written before it.]`
Rule: `round_trip_preserves_the_agreed_scope` (new,
`happenstance-sync-testkit`).
Cases: E2E-44, E2E-46.

*Rejects:* the round-trip rule everyone will write first — push everything, pull
everything, assert the two logs are equal. Against a scoped spoke it fails for a
conformant adapter, which makes it worse than useless: it will be "fixed" by
weakening it until it passes, and what it is weakened to will be whatever the
first scoped adapter happens to do.

---

**SY-29. A peer-supplied `Query` MUST be validated against a declared cost policy
before it is evaluated.**

`[PROVISIONAL — falsified if a cost policy expressible over `Query`'s surface
(types and tags only, `query.rs:111-115`) can be shown to admit an unbounded scan
anyway, in which case the seam is in the wrong place and belongs in the adapter.
Falsification test: construct a query that satisfies any tag-based policy and
still scans the log.]`
Rule: `ingest_refuses_an_unbounded_peer_query` (new,
`happenstance-sync-testkit`).
Cases: E2E-40.

*Rejects:* any ingest that treats a wire query as data to evaluate rather than
input to validate. The threat model is a **buggy** spoke, not a malicious one: a
peer pushes a condition of `Query::All`, and the receiver executes an arbitrary
peer-authored query against millions of rows inside a single-threaded actor with
a fixed CPU ceiling. This is the first concrete argument that the sync port needs
a policy seam and not only a transport, and it is sharper than anything currently
in the crate's prose.

What already defends itself, and should not be re-defended: `Query::All` cannot
be smuggled as an empty items list — `Query::from_items` rejects it
(`error.rs:94-98`) — and `QueryItem` and `Tags` re-validate on deserialisation
(`query.rs:387-392`, `tag.rs:538-545`), so non-canonical tags and fully
unconstrained items cannot reach memory. The envelope defends its own invariants.
Only cost is unguarded.

---

**SY-30. The push envelope MUST make the decomposition into independently
conditioned groups explicit.**

A peer's unit of work is `[(Option<AppendCondition>, Vec<Event>)]`, not a flat
event list.

`[PROVISIONAL — falsified if a peer is found whose unit of work genuinely spans
groups, requiring an atomicity the store cannot supply. Falsification test:
express it as one append and show `EventStore::append`'s single condition is
insufficient rather than merely inconvenient.]`
Rule: `push_envelope_preserves_group_boundaries` (new,
`happenstance-sync-testkit`).
Cases: E2E-35, E2E-39.

*Rejects:* the flat wire format, which is what a first implementation produces
and which hides a real fact about cost and atomicity. `EventStore::append` takes
exactly one `Option<&AppendCondition>` for the whole slice (`store.rs:261-265`),
so a 39-event push carrying seven independently-decided groups is seven appends
with nothing spanning them, and a crash mid-ingest leaves the receiver holding
four of seven. That is **correct DCB** — the boundary is the query, not the batch
— and it is stated nowhere in the contract, the testkit or the sync prose. A flat
list makes ingest cost look like O(events) when it is O(distinct conditions), and
turns a partial ingest into something discovered in production. The fix is the
envelope shape plus documentation; no contract change.

---

### 5.9 Where a per-peer watermark lives

---

**SY-31. A per-peer confirmation watermark MUST NOT be an event, and MUST be
written through `ProjectionStore` under a reserved `ProjectionId`.**

`[PROVISIONAL — falsified if a deployment needs the watermark inside a
transaction with a read model whose adapter is a different `ProjectionStore` than
the sync runner's. Falsification test: name one and show the two stores cannot be
the same handle.]`
Rule: `watermark_advances_transactionally_with_a_read_model` (new,
`happenstance-sync-testkit`).
Cases: E2E-33, E2E-44.

*Rejects:* two implementations and one plausible objection.

It rejects the **watermark as an appended event**. It changes on every sync, so
it puts a write in the log for every heartbeat, and it is device-local sync state
that no peer should replicate — a fact about a *link*, not about the world.

It rejects the **watermark in adapter-private storage**, reached around the port.
Kestrel Cold Chain's `today_schedule` is the case: its load-bearing column is
confirmation state (confirmed / unconfirmed-*N*m / superseded), which cannot be
computed from events at all and must be read *inside* the projection's
transaction. A watermark outside the port means the schedule rows and the
confirmation column live in two transactions that can disagree at an instant.

And it rejects the objection that there is no seam. There is: the watermark **is**
a local position, so `commit(batch, &ProjectionId::new("sync/<peer>"), local_position)`
type-checks against `projection.rs:126-131` today. What genuinely does not work
is a *generic* sync runner writing rows into a *generic* `Batch`, because
`type Batch<'a>` carries no trait bounds (`projection.rs:97-99`) — which is the
projection port's apply-seam question, and it is section 4's. PS-9 answers it:
the port does **not** grow a universal write vocabulary, so a generic sync runner
cannot write a read model beside the watermark. It writes the watermark through
the concrete adapter's inherent API, in the same batch, or it is not generic —
which is a constraint on the runner rather than a blocker on this clause, and it
is why the falsifier above is about handles rather than about bounds. The
signature also gains a fourth argument in section 4; the watermark supplies
`Authority::Live`.

---

### 5.10 Retention across a peer set, and refusal

---

**SY-32. A peer MUST be able to report the floor below which it no longer retains
history, and a runner MUST detect when a peer has been offline longer than
another peer's retention window.**

Convergence is a property of retention as well as of protocol. Where it has
become unreachable, the peer set MUST say so.

`[DEFERRED — settled by building the retention case: a peer offline for 120 days
against a 90-day compaction window, with a completeness instrument
(E2E-CASES.md:1595-1600) standing in for the compacted peer. The experiment is
whether a scalar floor is sufficient, which E2E-46 argues it is not — a
regulatory purge is scattered, not a prefix, and a floor is the shape a prefix
truncation has. Owning phase: the phase that builds the two peer adapters, jointly
with the deletion/redaction decision (E2E-CASES.md:1579-1585).]`
Rule: `retention_gap_is_reported_not_silent` (new,
`happenstance-sync-testkit`).
Cases: E2E-44, E2E-46, E2E-47.

*Rejects:* every retention implementation in all six scenarios, all of which
happen entirely outside the port. Silent non-convergence with nothing anywhere
reporting it is the current default behaviour, and it is the failure mode with
the longest latency in the catalogue: the vessel reconnects, ingests happily,
converges on a strict subset, and every projection on it reports healthy.

It also rejects `earliest_position()` as a sufficient primitive, and this is the
subtle half. It is exactly correct for a device pruning old history and useless
for a regulated purge, because the purge is scattered: a 2019 catastrophic-injury
file with a periodical payment order sits at a low position and must survive
while its neighbours are destroyed. A floor ships looking correct until a claim
runs long.

---

**SY-33. A peer MAY refuse a push for transport-level or authorisation-level
reasons. It MUST NOT refuse for a condition-level reason.**

Refusals permitted: unreachable, unauthenticated, unauthorised for the tag space
pushed, malformed envelope, quota. Refusals forbidden: any refusal whose truth
value depends on which events the receiver holds.

`[FROZEN]`
Rule: `refusal_is_never_condition_derived` (new, `happenstance-sync-testkit`) —
the fixture receiver is driven into every refusal it can produce; the rule
asserts none of them varies with the receiver's log contents.
Cases: E2E-45, E2E-39, E2E-47.

*Rejects:* the reading that will otherwise be re-litigated the first time someone
needs a spoke to be told "no". Under SY-1 "the hub refuses" can only mean a
transport or authorisation refusal — and the distinction is not pedantry, because
the two have opposite remedies. A transport refusal is retried and eventually
succeeds. A condition refusal can never succeed, because the condition is a
function of a state the spoke cannot change, so retrying is an infinite loop and
the only exit is for the spoke to discard a fact its user acted on. That is the
product failure the whole section is built to avoid, arriving through the one
door SY-1 did not close.

---

**SY-34. A spoke whose push is refused MUST leave its own log unchanged.**

It MUST NOT delete, rewrite or reorder a local event because a peer declined it.

`[FROZEN]`
Rule: `a_refused_spoke_retains_its_log` (new, `happenstance-sync-testkit`).
Cases: E2E-45.

*Rejects:* the assumption E2E-45 names as untested in the whole catalogue — that a
spoke's merge rule is the hub's merge rule read backwards. Every scenario walks
the hub's behaviour on ingest and none walks the spoke's behaviour on refusal.
The wrong implementation is the tidy one: on refusal, roll the spoke back to the
last acknowledged position so the two logs agree. It makes the spoke's log a
cache of the hub's, which is a defensible architecture and is **not this one** —
under SY-1 a spoke's writes are facts, and a fact that was durable and then was
not is the lie SY-33 exists to prevent, one tier down.

---

### 5.11 The rule ADR-0003 bought, and what it costs

The strongest single piece of evidence in the catalogue is that Kestrel Cold
Chain's hub **ran a domain decision, adjudicated a contested claim and authored a
compensation without ever parsing a payload** — because every fact the decision
needed was in the `EventType`, the `Tags` and the condition `Query` that
travelled with the batch. That is ADR-0003 surviving at the exact point most
likely to break it, and ADR-0003 is explicitly provisional pending exactly this
demonstration (`references/adr/0003-opaque-payloads.md:6-15`).

It survives only because the vocabulary was built for it. So the property is not
a consequence of the design; it is an **obligation on the domain**, and this
specification states it as one.

---

**SY-35. Anything replication must reason about MUST be in the tags.**

Any value an ingest path, an adjudicator, a compensation, a forwarding rule or a
convergence check needs to read MUST be present in the event's `EventType` or
`Tags`. It MUST NOT be reachable only from `Event::data` or `Event::metadata`.

`[FROZEN]`
Rule: `the_sync_suite_never_decodes` (new, `happenstance-sync-testkit`) — the
suite's own fixture domain supplies payloads that are not decodable in any codec,
and every rule in the suite must still pass. A rule that needs to decode is a rule
that has found a violation of this clause.
Cases: E2E-34, E2E-39, E2E-40, E2E-42.

*Rejects:* the vocabulary design that puts an adjudication-relevant value in the
payload, and it rejects it *statically* — the suite will not compile a fixture
that needs to decode. Two worked instances, one in each direction:

- Kestrel Cold Chain's `WorkOrderCompleted` carries `outcome:repaired` **as a
  tag**, which is the only reason the hub can author
  `WorkOrderCompletionContested` with `kept-outcome` and `contested-outcome`
  without a decoder. The scenario originally claimed the hub "cannot pick by
  outcome without decoding a payload, which ADR-0003 forbids the peer". Its own
  vocabulary refuted it.
- Wattline's `ChargePointTransferred` carries the *previous* tenant in the
  opaque payload deliberately, because two `tenant:` tags on one event would make
  it match both operators' queries. That is the same rule pointing the other way:
  the payload is the right home for a value that would be **harmful** as a tag.
  The rule is not "tag everything"; it is "tag what replication reasons about",
  and the two examples together are what make it a design rule rather than a
  slogan.

> **[NON-NORMATIVE]** The library cannot express which tag keys a peer owns, and
> should not try. Kestrel Rotor's `IssueBulkFromVesselStock` is partition-safe
> only because its tag set contains a `location:` key exactly one peer ever
> writes, so the local log is complete with respect to *that query* while
> incomplete with respect to everything else. Nothing lets a domain declare
> `location:*` peer-owned and nothing can check that a query is closed under a
> peer's write set. Putting ownership semantics into `Tag` would make it
> non-opaque and exceed the DCB specification — `Tag` is a `Box<str>`
> (`tag.rs:20-23`) with `key()`/`value()` offered as convention over
> `split_once(':')` (`tag.rs:153-165`), and a tag with no colon is legal. The
> declaration belongs to the sync **runner**, as a per-peer set of owned tag keys
> with a check that a decision's query is closed under it. Recorded here so the
> omission is a decision rather than an oversight.

---

### 5.12 Indicative shape

Non-normative. Names are the implementation's to settle; what the clauses bind is
the shape, not the spelling.

```rust
/// One peer relationship. Not a protocol, not a peer set.
#[trait_variant::make(SendSyncPeer: Send)]
pub trait SyncPeer {
    type Error: core::error::Error + 'static;

    /// Opaque, owned, transferable. SY-16: not a handle this peer holds.
    type Resume: Clone;

    /// One round trip. SY-15.
    async fn pull(
        &self,
        from: Option<&Self::Resume>,
    ) -> Result<(Push, Self::Resume), Self::Error>;

    /// One round trip. SY-15. Refusals are transport- or
    /// authorisation-level only — SY-33.
    async fn push(&self, push: &Push) -> Result<Ack, Self::Error>;

    /// SY-18: read before pushing, not discovered at ingest.
    fn limits(&self) -> PeerLimits;
}

/// SY-30: the group boundaries are on the wire, not emergent.
pub struct Push {
    pub groups: Box<[(Option<AppendCondition>, Box<[SequencedEvent]>)]>,
}
```

Four Rust-specific choices in that sketch, each of which had an alternative:

**`trait_variant` again, not `#[async_trait]`.** ADR-0001 binds it: `async_trait`
injects `+ Send`, which makes the wasm32 target impossible, and the wasm32 peer
here is not a leaf — it is the Kestrel Cold Chain hub in the middle of the chain
(SY-17). The two-flavour scheme is the only one that lets one body of code serve
both, and generic code binds the bare flavour.

**`type Resume` is an associated type, not a `SequencePosition`.** A position is
the obvious spelling and it is wrong under SY-27: if replication turns out to be
scoped, a scalar position cannot distinguish "not yet received" from "filtered
out", and the port would be frozen against the whole-log answer before the
experiment ran. An opaque associated type defers the question into the adapter
without deferring the port. The cost is that a runner cannot compare two peers'
resume tokens — which is correct, because SY-19 says two peers' orders are not
comparable.

**`pull` returns `(Push, Self::Resume)` rather than a stream.** `EventStore::read`
returns `impl Stream` at the top level precisely so the `Send` flavour can mark
the *stream* `Send` (`store.rs:153-158`, CLAUDE.md constraint 3) — and that is the
right shape for a store, where laziness buys a million-event replay without
buffering. It is the wrong shape here: a stream is a cursor, a cursor is state
held between polls, and SY-15 forbids it. The peer returns a bounded batch and a
token, and the runner loops. That is one round trip per call by construction
rather than by discipline.

**`Push` holds `SequencedEvent`, not `Event`.** The `EventId` must travel (SY-12)
and `Event` has no field for it (`event.rs:321-326`). This is the clause that
makes `EventId`'s placement on `SequencedEvent` load-bearing outside the store —
and it is the concrete form of `RUNBOOK.md:438-439`'s warning that
deferring this port leaks `EventId` back into `EventStore`. The leak is real, it
is here, and naming it is cheaper than discovering it.

---

### 5.13 What this section does not decide

Recorded so that silence is never mistaken for agreement.

- **DCB wire interoperability.** Out of scope by the bound decision recorded in
  section 2 as WF-1:
  the `serde` feature moves events between happenstance instances and is not an
  interoperability surface. If interop is wanted later its home is a separate
  crate beside the peer adapters, not the peer port and not the contract crate.
- **A store-assigned time on `SequencedEvent`.** E2E-43. Not undecided — section 2
  takes it as VT-9, provisionally, and this section neither adds to it nor relies
  on it. SY-23 gives a deterministic tiebreak and explicitly refuses to be read
  as giving a fact about the world; VT-9 gives the fact about the world and
  forbids ordering by it.
- **Deletion, redaction, and what a store that has been deleted from is permitted
  to look like.** E2E-46 through E2E-49. Section 3 takes the refusal half —
  ES-37 puts deletion out of scope for `EventStore` and ES-38 bounds what a store
  deleted from may look like — and ES-39 defers the primitive that lets a store
  say what it does not hold. SY-32 depends on ES-39 and cannot be settled ahead
  of it.
- **The projection port's apply seam.** SY-31 depends on it; section 4 owns it and
  PS-9 decides it against a universal write vocabulary.
- **`Error: Send + Sync` on either flavour.** Settled, and recorded here because
  it reaches this port through `type Error` in the sketch above rather than
  through anything section 5 decided. It was ranked first by blast radius in
  `PRESSURE-TEST.md:481-492` and held open on evidence until phase 2 built an
  instrument that could fail the bound; ES-6 is now `[FROZEN]` and
  [ADR-0009](../.kb/decisions/0009-error-send-sync.md) keeps
  `core::error::Error + 'static` on both ports and both flavours, putting the
  stronger property in a marker trait that generic code asks for. That answer was
  taken for the whole workspace at once, `SyncPeer` included, which is why this
  section neither adds to it nor may diverge from it. What is still open is
  narrower and is not section 5's: whether `happenstance-core` ships the marker
  itself, and under what name — a surface question deferred to phase 4 with the
  capability question already answered
  (`references/adr/0009-error-send-sync.md:210-213`).

---

## 6. The conformance obligation

The suite is the project's claim to exist. `happenstance-testkit`'s own module
documentation says an adapter "is not considered to exist until it invokes
`event_store_conformance!` and passes"
(`crates/happenstance-testkit/src/lib.rs:12-13`), and CLAUDE.md repeats it as the
rule that matters. Everything downstream of that sentence — every adapter, every
freeze, the whole "storage agnostic" claim — is worth exactly what the suite
measures.

Measured at the start of phase 3, it was worth less than it says. The
measurement is of *that* tree, `b4b593d`, line numbers included — which is why
those numbers do not resolve against the working copy and are not meant to. That
covers the rest of this paragraph as well as §6.1 and §6.2 below: the sentence
naming the scope and the first measurement it scopes are two clauses of the same
thought, and a reader who takes "§6.1 and §6.2 below" literally is left with
eight citations that look current and are not. §6.3 onward is written in the
present tense, and where CF-15 – CF-21 have since changed what the measurement
describes, the clause says so in place. Twenty-seven rules,
enumerated then as now in exactly one place
(`crates/happenstance-testkit/src/registry.rs:98-142`), and a store that ignores
tags entirely when evaluating an
append condition passes all of them; a store that assigns positions outside its
transaction passes all of them; a store that returns `Ok` from `append` and loses
the write on restart passes all of them, because nothing in the workspace can
express a restart. Those are not oversights of taste. They are structural: three
of them are foreclosed by the fixture's type, one by the harness's, and the rest
by the fact that every append-condition rule in the file builds its condition
from `query_of_types` (`suite.rs:390`, `:447`, `:465`, `:483`, `:501`, `:519`,
`:537`, `:561`).

This section is therefore not a list of rules to add. It is the specification of
what the *suite* must satisfy — the meta-rules that make a rule mean something —
plus the portfolio of instruments the workspace must hold before any freeze in
sections 2 through 5 is more than a statement about `MemoryEventStore`.

Clause IDs in this section are `CF-n`. A clause's `Rule:` names the mechanical
check that enforces it; for clauses about the suite itself that check is a
meta-test in the testkit's own `tests/` or a step in `cargo xtask ci`, not a
conformance rule, and the clause says which.

---

### 6.1 The suite's own proof obligation

CLAUDE.md's corollary — *a rule that no adapter can fail is decorative* — was
still enforced by a reviewer at `b4b593d`, but phase 2 had closed the half of the
gap that was about evidence. The testkit's `tests/` held five files, and
`local_conformance.rs` carried a `mutants` module with two deliberately wrong
stores: `BorrowHoldingStore`, whose `read` returned a stream that keeps the
`RefCell` borrow alive, and `AwaitAcrossBorrowStore`, whose `append` held the
exclusive borrow across an `.await`. Both type-checked, both were wrong in a way
an adapter author would plausibly be wrong, and both panicked when driven.

What was missing is everything between them and the rule set. Each was driven by
a bespoke `#[should_panic(expected = "already borrowed")]` test rather than by
the suite, so no rule was named, no rule was *shown* to reject them, and adding a
rule with no wrong implementation behind it still cost nothing. CF-2's rejected
shape was therefore already on disk — hand-written per-mutant tests standing in
for a registry — which is a better argument for this section than the absence it
used to make, because the thing being forbidden could be read rather than
imagined.

**Both are now registry rows**, and the two halves landed in the order this
section prescribes rather than in the order that would have been convenient: the
`#[should_panic]` drivers survived stage 3, when the registry was built, because
`mutant_registry_is_exhaustive` rejects a mutant whose `fails` list is empty and
against the rules that existed then that list *was* empty. They moved at stage 4
with ES-36's two rules, in the same change, and `tests/local_conformance.rs`'s
`mutants` and `reentrancy` modules are both gone. Deleting the drivers earlier to
satisfy CF-2's shape would have deleted the evidence and kept nothing.

The fix is to make the mutant a first-class artefact of the suite, registered as
data, and to let a meta-test rather than a reviewer decide whether the obligation
was met.

The **six** meta-tests — CF-1 through CF-5, and CF-18, which needed the same
machinery and is the clause the fixture contract left owing one — live in
`crates/happenstance-testkit/tests/mutation_coverage.rs`, outside the event-store
suite and outside CF-22's enumeration of it, because they take the rule set as
their *input* and a rule that enumerated them would be enumerating itself.
`cargo xtask spec-trace` reads only `suite.rs`, so it neither resolves their names
nor reports them as orphans, and §7.2 renders them in the clause's own words
rather than marking them `†`. This is the convention §1.4 already states for the
`WF` clauses' wire tests, applied to the second family that needed it.

A **seventh and an eighth** joined them at stage 5 and neither is one of these
six: `the_model_rule_rejects_exactly_what_it_claims` and
`the_concurrency_rules_reject_exactly_what_they_claim` both belong to CF-22,
which is where they are described. They share this file because they share the
wrong stores, not because they share the obligation — the eighth drives a second
table, `RACERS`, because every store in it fails no rule of the event-store
family and `mutant_registry_is_exhaustive` rejects a row with an empty `fails`
list.

That exemption leaves the file itself unguarded by the checker, so the gate names
it instead: `cargo xtask proof-artefact` asserts **all eight** names out of
`cargo test -- --list` before running them. Naming the target alone was not
enough — `cargo test` exits 0 on `running 0 tests`, so a file truncated to its
attributes passed the step that its deletion failed. The list is
`xtask/src/proof.rs`'s `META_TESTS`, and it is a *subset* check so that a ninth
meta-test needs no gate edit; the seventh and eighth are in it anyway, because
this section and CF-22 cite them by name and the list is what notices when a
cited name moves.

**Since measured, 2026-08-07.** The registry landed and this section's opening
measurement is of the tree before it. Three things it describes have moved, and
they are recorded here rather than by editing the measurement, so the argument
keeps its provenance:

* The testkit's `tests/` holds seven files, not five: `fixture_instruments.rs`
  arrived with the fixture contract and `mutation_coverage.rs` with the registry.
* `fixture_instruments.rs`'s hand-written `#[should_panic]` drivers — CF-2's
  rejected shape, cited above as being already on disk — were **deleted**. Its
  two wrong fixtures are now registered rows driven through every rule.
* `local_conformance.rs`'s two remained under `#[should_panic]` through stage 3,
  deliberately and with the argument written beside them: `BorrowHoldingStore`
  and `AwaitAcrossBorrowStore` failed no rule the suite had, because no rule held
  a read stream open across an append, and `mutant_registry_is_exhaustive`
  rejects a mutant with an empty `fails` list. **They moved at stage 4**, with
  ES-36's two rules, in the same change — which is the ordering CF-1 forces
  rather than one anybody chose. That file's `mutants` and `reentrancy` modules
  are both gone.

**Since that note, 2026-08-10.** Its first bullet's count has moved again, and is
recorded here for the same reason it was recorded there rather than by editing it:
a count with a date on it is auditable and a count without one is not. The
testkit's `tests/` holds **ten** files — `fixture_instruments.rs`,
`foreign_identity.rs`, `local_conformance.rs`,
`memory_concurrency_conformance.rs`, `memory_conformance.rs`,
`memory_conformance_blocking.rs`, `memory_conformance_wasm.rs`,
`memory_model_conformance.rs`, `mutation_coverage.rs` and `properties.rs` — beside
a `mutation_coverage/` directory, which is not one of them. Two of the harnesses
are §6.4's, which describes the families that produced them. The one nothing else
in §6 reaches is `foreign_identity.rs`: it holds
`append_does_not_accept_a_foreign_identity`, the compile-level obligation VT-10's
`Rule:` field puts in the testkit rather than in the contract crate.

**CF-1.** Every conformance rule MUST be paired with at least one *mutant store*
in `happenstance-testkit`'s own `tests/`, which fails that rule. A rule
introduced without one is a defect in the suite and MUST NOT be merged.
`[FROZEN]`
Rule: `mutation_coverage::every_rule_has_a_mutant` — a meta-test in
`crates/happenstance-testkit/tests/mutation_coverage.rs`.
Cases: all contract-level cases; the obligation is stated at E2E-CASES.md:8-15.
Rejects: `positions_are_unique` and `positions_are_strictly_monotonic`
(`suite.rs:336-365` in `b4b593d`) as they stood. Both read a quiescent store back
through `read`, which every adapter returns in position order, so both were
satisfied by sorting on the way out, and neither had ever been paired with a store
that could fail them.

*Resolved at stage 3, 2026-08-07, and the resolution is what the clause is for.*
Both now have one: `SharedBatchPositionStore` binds a single position for a whole
multi-event batch — `INSERT … VALUES (?1,…), (?1,…)`, correct when `?1` is
`nextval()` and wrong the moment the value is precomputed in Rust — so the batch
comes back with duplicate, non-increasing positions that no sort can repair. It is
declared against both rules and `mutants_fail_exactly_their_declared_rules`
passes. Two things this clause claims are therefore now demonstrated rather than
argued: that these two rules were decorative as written, and that writing the
wrong implementation is what settles the question. Note also what the mutant
needed — a **multi-event batch**; every other rule in the suite appends one event
at a time, which is why the defect was invisible.

**CF-2.** Mutants MUST be registered as data — a `const` table naming, per
mutant, the exact set of rules it fails — and MUST NOT be exercised only by
hand-written per-mutant tests. `[FROZEN]`
Rule: `mutation_coverage::mutant_registry_is_exhaustive` — a meta-test in
`crates/happenstance-testkit/tests/mutation_coverage.rs`.
Cases: all contract-level cases.
Rejects: the natural cheap version — a `tests/wrong_stores.rs` that invokes
`event_store_conformance!` against each mutant with `#[should_panic]`. That
records only *that* something failed, so a mutant which fails the right rule for
the wrong reason (a panic in its constructor, an unrelated regression) reads as
proof. The registry exists so the meta-test can assert the *set*, not the count.

**CF-3.** The meta-test MUST assert both directions: every registered mutant
fails every rule it declares, **and** passes every rule it does not. `[FROZEN]`
Rule: `mutation_coverage::mutants_fail_exactly_their_declared_rules` — a
meta-test in `crates/happenstance-testkit/tests/mutation_coverage.rs`.
Cases: all contract-level cases.
Rejects: a mutant that is broken in more ways than it claims — the commonest way
a mutant set decays. A store that drops tags from its condition probe *and*
returns events out of order proves the tag rule catches something, but not that
it catches the tag defect. The second assertion is what keeps a mutant a
scalpel.

**CF-4.** Every mutant MUST carry a non-empty provenance string naming the real
adapter shape or scenario that makes it plausible. `[FROZEN]`
Rule: `mutation_coverage::every_mutant_states_its_provenance` — a meta-test in
`crates/happenstance-testkit/tests/mutation_coverage.rs`.
Cases: E2E-01, E2E-08, E2E-32, E2E-55.
Rejects: the saboteur — `struct AlwaysWrong; impl EventStore for AlwaysWrong { … }`
— which satisfies CF-1 mechanically and proves nothing, because no author would
have written it. The mutant that earns its place is the one someone would ship:
dropping the tag join because tags live in a second table
(`crates/happenstance-sqlite/src/event_store.rs:62-67`), caching `max(position)` per
session, evaluating each `QueryItem` as its own statement.

**CF-5.** The testkit MUST also hold at least one *conformant variant* — a store
that is legally different from `MemoryEventStore` and MUST pass every rule.
`[FROZEN]`
Rule: `mutation_coverage::conformant_variants_pass_everything` — a meta-test in
`crates/happenstance-testkit/tests/mutation_coverage.rs`.
Cases: E2E-10.
Rejects: an over-specified rule. The specification permits gaps, and a store
assigning positions in steps of seven, or starting at 4,096, is conformant; a
rule that quietly assumes density passes against the reference store and fails
that adapter in the field. Mutants catch under-specification. Conformant variants
catch over-specification, and nothing in the workspace catches it today.

**CF-6.** No rule may assert on a literal sequence-position value. Every position
assertion MUST be anchored on a value the store under test assigned. `[FROZEN]`
Rule: `mutation_coverage::conformant_variants_pass_everything` (CF-5's gapped
variant is the enforcement), plus a `cargo xtask ci` lint step over the three
files rules live in — `suite.rs`, `model.rs` and `concurrency.rs` — rejecting an
integer-list literal outside index position and a `SequencePosition` built from a
literal. The variant is the check and the lint is the cheap second line, in that
order: the variant refutes a rule *behaviourally*, and a reader who takes the
grep for the enforcement will eventually delete the store that does the work. The
lint found nothing on its first run, which is what four phases of the convention
being honoured by hand looks like; what it buys is that the next violation fails
naming the rule rather than the mutant.
Cases: E2E-10.
Rejects: `assert_eq!(positions_of(&found), [1, 2, 3])`. The suite honours this
today — `query_item_combines_types_and_tags_with_and` compares against
`positions_of(&all[..1])` and says why at `suite.rs:189-191` — and prose is the
only thing holding it. CF-5's gapped variant turns the convention into a test.

*Not a clause, because no rule can check it.* A pass rate over an author-chosen
mutant set carries no information: the denominator is a choice. Report which
defects the set covers and which axes it leaves uncovered; never report a
fraction. This repeats PRESSURE-TEST.md:472-475 and is recorded here because the
mutant registry CF-2 mandates is exactly the artefact that invites the fraction.

---

### 6.2 The measured gaps

Each clause below names one hole verified against `suite.rs` in this tree, the
new rule that closes it, and the implementation the rule rejects. These are not
proposals for new *semantics* — sections 2 through 5 own the semantics. They are
the checks those semantics have been going without.

**CF-7.** At least one rule MUST demonstrate that an append condition's verdict
depends on **tags**: two stored events sharing a type and differing in tags, a
condition tagged to match one of them, asserting `ConditionViolated`. `[FROZEN]`
Rule: `condition_matches_on_tags` (ES-27 specifies it and names it, and this
clause is the obligation to have it).
Cases: E2E-55, E2E-56; scenario S3 in PRESSURE-TEST.md:590-596.
Rejects: an adapter that drops the tag join from its condition probe and keeps it
only in `read`. Every condition call site in the suite builds its query from
`query_of_types` (`fixtures.rs:64-67`), and the single tagged condition —
`racing_conditional_appends_elect_one_winner` at `suite.rs:603-605` — runs
against a store holding one untagged `CourseDefined` (`:600`) and one tagged
`StudentSubscribed` (`:607`), so a type-only probe returns the identical verdict
at `:612` and `:621`. CF-19 has since added a second tagged condition —
`two_handles_observe_each_others_appends` — and it does not close this gap: it
asks whether a committed append is *visible* across handles, and returns the
same verdict whether or not the probe joins on tags. The residual gap is
unchanged and is exactly what this clause names. The tag join is the expensive
half and lives in a separate
table in the landed SQLite adapter
(`crates/happenstance-sqlite/src/event_store.rs:62-67`), which makes dropping it the
natural first cut. Such an adapter rejects every command touching any course, and
passes all twenty-seven rules while doing it: a total-availability failure
certified as conformant, on the canonical DCB uniqueness shape.

**Discharged at phase 3 stage 4**, and the rule is narrower than the clause's own
sentence in one respect worth recording. The clause asks for two events sharing a
type and differing in tags; the rule adds that the one the condition names
carries an **extra** tag it does not, because without that the exact-match probe
— tags serialised to one column and compared with `=` — returns the correct
verdict and the rule catches only CF-8's defect. `ExactTagMatchConditionStore` is
the mutant, and it is `ExactTagMatchReadStore`'s sibling one code path over: the
pairing is CF-9's own argument, that the read path and the probe are different
code.

**CF-8.** At least one rule MUST demonstrate the mirror: a condition carrying a
tag no stored event carries MUST NOT reject the append, even when a stored event
matches the condition's types. `[FROZEN]`
Rule: `condition_with_an_unheld_tag_does_not_reject` (ES-27 names it as
CF-7's mirror, and this clause is the obligation to have it).
Cases: E2E-55, E2E-56.
Rejects: the over-rejecting adapter — one that treats a condition's tags as
advisory and rejects on type alone. CF-7 alone catches only the fail-open
direction; an adapter tuned to be "safe" by ignoring tags in the narrowing
direction is equally non-conformant and equally invisible today. The two clauses
are a pair and neither is sufficient.

**Discharged at phase 3 stage 4.** `TagBlindConditionStore` is the mutant, and it
passes CF-7's rule — a type-only probe finds the superset event and rejects,
correctly, by accident — which is the mechanical demonstration that the pair is a
pair.

**CF-9.** At least one rule MUST read `Query::all()` against a store holding an
event carrying several tags, and assert each event is yielded exactly once.
`[FROZEN]`
Rule: `duplicate_items_do_not_duplicate_events` (ES-15 specifies it and
names it, and this clause is the obligation to have its `Query::all()` half).
Cases: E2E-32.
Rejects: an adapter whose tag storage is a row-per-tag side table joined without
`DISTINCT`. `query_all_matches_every_event` (`suite.rs:70-80`) appends three
untagged events, so the fan-out cannot occur; `query_item_tags_match_supersets`
(`:123-145`) stores a three-tag event but reads it through a tag-constrained
query, which an adapter may satisfy with a different code path. The duplicate is
invisible in both.

**Discharged at phase 3 stage 4.** `TagJoinFanOutStore` models the fan-out only
where **no item constrains tags**, which is the faithful version rather than a
weakened one: the tag-AND path has to be written as `GROUP BY … HAVING COUNT(*) =
n` or "all of these tags" is wrong, and that grouping collapses the duplicates
for free. So the defect lives exactly on `Query::all()` — the query every
projection runner starts from, and the query every other rule in the suite reads
over untagged events.

**CF-10.** At least one rule MUST evaluate an append condition against an
**empty** store and assert the append succeeds. `[FROZEN]`
Rule: `condition_against_an_empty_store_admits_the_append` (ES-28
specifies it and names it).
Cases: E2E-47 (adjacent; a condition over history a store does not hold).
Rejects: an adapter whose condition probe is a correlated subquery or a
`MIN`/`MAX` aggregate that returns `NULL` on an empty table and then compares
against it — SQL's three-valued logic turns `NULL > ?` into "unknown", which
`WHERE` treats as false in some formulations and as a rejection in others
depending on how the predicate is nested. Every existing condition rule seeds the
store first (`suite.rs:442`, `:460`, `:481`, `:498`, `:516`, `:533`, `:556`,
`:600`), so the degenerate case has never run. This clause serves no numbered E2E
case directly; it comes from PRESSURE-TEST.md:352-356, and the traceability
obligation CF-37 requires the catalogue to gain one.

**Discharged at phase 3 stage 4**, with `NullAggregateProbeStore` as the nesting
that rejects. The rule asserts only that the append is admitted, deliberately: a
read-back afterwards would import another mutant's defect into a rule whose
subject is the probe.

**CF-11.** At least one rule MUST exercise the empty batch against a **non-null**
condition, and assert the precedence ES-20 fixes: `AppendError::NoEvents`, with
the emptiness check preceding the condition check. `[FROZEN]`
Rule: `empty_batch_is_refused_before_the_condition_is_evaluated` (ES-20
specifies it and names it).
Cases: E2E-06 (adjacent; the batch's own events against its own condition).
Rejects: `MemoryEventStore` as it stood, which is the sharpest possible answer
to "can this rule fail anything". `append_rejects_empty_batch` only ever called
`append(&[], None)` (`suite.rs:408`), while `MemoryEventStore` evaluated the
condition first and returned `ConditionViolated` for `append(&[], Some(&matching))`
(`memory.rs:197-216`), reporting a concurrency signal for what is unambiguously
the caller's own bug — and putting a correct client into a retry loop that can
never terminate. Two adapters can disagree here and both pass today; that
interoperability hazard is what the clause closes, and ES-20 is where the
direction is argued.

This clause was drafted `[PROVISIONAL]` against a precedence "section 2 settles".
Section 2 does not own it — the empty-batch/condition precedence is an `append`
semantic and §3 owns `append` — and ES-20 settles it `[FROZEN]`. The rule's
assertion is therefore writable now.

**Discharged at phase 3 stage 4, and D8 is fixed at both of its sites.**
`MemoryEventStore` now checks emptiness *above the lock*, because emptiness is a
precondition on the argument and nothing behind the lock can change the answer;
`LocalMemoryEventStore` in the testkit's own `tests/` copied the old ordering
deliberately and said so in a comment, and the comment moved with the fix. The
mutant registry's own correct core copied it a third time. Three implementations,
one defect, indistinguishable on every non-empty batch — which is how it
survived, and is the argument for the rule rather than against it.
`ConditionBeforeEmptinessStore` is the old order, retained as the mutant.

**CF-12.** At least one rule MUST compose `ReadOptions::from` with a
**multi-item** query. `[FROZEN]`
Rule: `read_from_composes_with_multi_item_query`,
`read_from_composes_with_limit`.
Cases: E2E-10; scenario S7 in PRESSURE-TEST.md:620-624.
Rejects: an adapter generating `WHERE a OR b AND position >= ?` without
parentheses — the textbook operator-precedence bug, which silently returns every
event matching item `a` regardless of the cursor and therefore re-delivers
already-checkpointed events to a resuming projection forever. Measured, the gap
is wider than the audit's phrasing: **no read option is exercised against a
filtering query at all.** All five read-option rules issue `Query::all()`
(`suite.rs:253`, `:266`, `:269`, `:283`, `:296`, `:314`, `:320`), and the two
rules that do read through a filtering query pass default options
(`:235-240`, `:627-632`). The
clause requires `from` × multi-item because that is where the generated SQL is
most likely to be wrong; `backwards` and `limit` against a filtering query
SHOULD be covered in the same rule.

**Discharged at phase 3 stage 4, and the SHOULD was taken.** All three options
appear against the same two-item query in the one rule, because the measured
finding is *one* gap — no read option against a filtering query at all — and three
rules would have suggested three. `UnparenthesisedPredicateStore` is the mutant
the clause names, and it is a scalpel: with a single item there is nothing for the
`OR` to bind wrongly across, so it is invisible to every read-option rule the
suite had. Four other mutants fail the rule as well, which is what a rule
exercising three options against a filtering query should do — `LimitBeforeFilterStore`,
`ItemOrderedUnionStore`, `FetchOneExtraStore` and `BackwardsIgnoredStore` each on
their own axis.

**Amended at phase 12: appearing is not composing.** The paragraph above said
"all three options are exercised" and was read for two phases as though the
composition existed. It did not. `read_from_composes_with_multi_item_query` issues
the three options as **three separate reads** — `from(anchor)`, then
`backwards()`, then `backwards().limit(2)` — so forwards `from` composed with
`limit` was issued at no call site in the suite, and neither were the other
combinations `from` did not happen to be paired with. That is the one composition
three `[FROZEN]` clauses name as the consumer that motivates them: ES-14's budget,
VT-28's `.limit(budget - fetched)` at parity, and this clause's cursor. A budget
loop that resumes carries `from`. `ForwardPagingBudgetStore` — `limit` applied
only where `from` is absent — passed all eighty-nine rules and was
indistinguishable from both conformant controls. `read_from_composes_with_limit`
is the rule that closes it, and it is the second rule this clause claims;
`UnparenthesisedPredicateStore` and `LimitPerItemStore` fail it too, which is
what makes it a statement about the *merged* result rather than about either
item.

**And the `to` side is ES-16's, not this clause's.** This clause's MUST names
`ReadOptions::from` and stays as written; what the same measurement said about
the *upper* bound — that no read-option rule composed it with a filtering query
either — is discharged by `read_to_composes_with_multi_item_query` under **ES-16**,
where the read semantics of `to` live. It is recorded here because the two rules
are twins and a reader arriving at the shape from this end should be sent one
clause over rather than left to conclude the gap is still open.

**CF-13.** A rule MUST assert the visibility invariant — that once a reader has
observed position *P*, no event at a position at or below *P* becomes visible
afterwards — and it MUST be accompanied by a fixture that fails it. `[FROZEN]`
Rule: `nothing_below_an_observed_position_appears_later` (ES-10 specifies it and
names it, and this clause is the obligation to have it).
Cases: E2E-01, E2E-02.
Rejects: a Postgres adapter allocating positions with `nextval()` outside the
transaction. Both existing position rules read a quiescent store after a single
sequential writer (`suite.rs:336-365` in `b4b593d`), so they measure assignment
and say
nothing about visibility — the property Wattline's nineteen-millisecond window
breaks and the property `AppendCondition::after` is sound only under. Shipping
this rule without the fixture would violate CF-1: it would be a rule that has
never failed anything, which is precisely what E2E-CASES.md:1555-1561 warns
against.

**The experiment this clause deferred has been run, and it is why the marker
moved.** It was drafted `[DEFERRED]` against a deterministic hostile fixture in
the testkit's own `tests/` — a store that withholds one append's row until a
second, later-positioned append has committed, driven from a single-threaded
harness — with a reserve deliverable named in the same breath: *if the fixture
cannot be made deterministic without a `Send + Sync` sub-trait to bind parallel
rules on, that sub-trait becomes the deliverable instead.* It could, and the
sub-trait was **not** needed. Recording that is the point of writing the reserve
option down in the first place.

What made it deterministic is that nothing is scheduled. Two `append` futures are
created from one handle and neither is polled by being created, because an
`async fn` body runs nothing until its first `poll`. The rule then polls them by
hand, one at a time, and reads after every step. There is no executor, no thread,
no `Send` bound anywhere, and — CF-33 — no clock: a wall-clock deadline inside a
conformance rule is a flake inside a conformance rule.

The **order** is A, B, B, A, and it is the half a reimplementation will get wrong.
Both writers take their positions, and then the writer that started *second* is
resumed first. Plain alternation is not a substitute and this was measured rather
than argued: under A, B, A, B the hostile store commits in allocation order, no
reader can tell it from a correct one, and
`mutants_fail_exactly_their_declared_rules` reports the rule as passing the store
it exists to reject. The reversal on resumption is what models the slow
transaction that took a low number and published it late.

`MemoryEventStore` passes the rule trivially — its `append` body contains no
`.await`, so the first poll runs it to completion and there is no window to have —
and that is the correct outcome rather than a weakness. A store with no window
cannot have a window bug, and the schedule skips a future that has already
finished rather than polling it again.

What is **not** discharged is CF-26's other half. The fixture proves the rule
bites; whether a real Postgres adapter can *pass* it, and at what cost among
`xid8` + `pg_snapshot_xmin`, transaction-scoped advisory locks and a serialised
sequence table, is a measurement that is still owed and still owned by the
position-allocation row of §6.5's portfolio table.

**CF-14.** A rule MUST assert that an append acknowledged with `Ok` survives a
reopen — CF-17's operation, which discards the store's process-level state and
leaves only what was durably committed. `[DEFERRED — the rule itself has landed
early, as the named exception recorded below; what is still deferred is the
experiment, which is whether a `reopen` capability can be honoured by rusqlite,
a Durable Object and a one-shot HTTP client with one shape, or whether "durable"
needs to be graded. **One of the three has now answered**: `happenstance-sqlite`
honours it with the one shape — close every connection, checkpoint the
write-ahead log, do not touch the file — and needed no grading. The deferral is
therefore confirmed and narrowed to the two implementations that have not:
`happenstance-cloudflare` (HS-P0013), where a "reopen" is a Durable Object's
storage surviving an eviction rather than a file being closed, and
`happenstance-neon` (HS-P0014), which has no connection to close at all.
Falsified by the first of those two that cannot express it with this shape.]`
Rule: `acknowledged_writes_survive_a_reopen` (new; ES-35 specifies it and names
it, and this clause is the obligation to have it).
Cases: E2E-07.
Rejects: an adapter that acknowledges before `COMMIT` — a pooled rusqlite store
doing its work in `spawn_blocking` and returning on the join, a Durable Object
relying on output-gate semantics it does not actually have, any store with
`PRAGMA synchronous = OFF` in its connection setup. Nothing in the workspace
could fail such a store while `event_store_conformance!` handed each rule one
factory call and there was no second call whose results would be meaningful.

The rule was landed early, as a **named exception to this clause's own
deferral**, and the name is here because a deferral nobody records is
indistinguishable from an oversight. It landed with CF-17 rather than at phase 8
because without it the fixture contract shipped with one capability-gated rule
and no second one to observe, leaving `REOPEN`'s provisional marker and the skip
machinery both untested. `DurableFixture` in the testkit's own `tests/` supplies
`REOPEN`, and `LosingFixture` beside it acknowledges before recording, so the
rule both runs and is shown to fail. What stayed deferred to phase 8 was the
adapter far end: a store that loses a write because of a real fault rather than
because a test told it to.

Phase 8 answered the *adapter* half and left the *fault* half open, and the two
are worth keeping apart because they were one sentence until now.
`happenstance-sqlite` runs all three reopen rules against a real file across a
real close-and-reopen, so a store whose durability claim is false now fails a
rule that has been driven against a medium outside the process. `SqliteFixture`
declines `MID_BATCH_FAULT` by scope rather than by incapacity — the trigger-based
injection the fixture contract names is demonstrably available to it — so the
fault far end is still supplied by nothing. That is ES-35's residual falsifier and
it is stated there rather than duplicated here.

---

### 6.3 The fixture contract

This subsection was written against the arrangement it replaced, and the
arrangement is worth stating in the past tense rather than deleted, because every
clause below is an answer to it. `event_store_conformance!` hoisted `$factory`
behind a `fn __conformance_store()` and the emitter expanded to
`$crate::rules::$name(__conformance_store).await`, so the expression was
re-evaluated on every call and each rule called it exactly once. `F: Fn() -> S`
was therefore doing two incompatible jobs at once and had been asked to promise
neither: the macro's documentation said the expression must build "a **fresh,
empty** store", while the *signature* would have accepted `|| store.clone()` —
two handles onto one backing store — and nothing in the suite either required or
forbade it. A rule could not call the factory twice, because it could not know
whether it got isolation or sharing. That ambiguity is what foreclosed CF-14,
CF-19 and every genuine multi-connection rule, and it was a type problem before
it was a coverage problem.

The remedy is to name the two operations separately, which means a trait rather
than a closure. A closure has one call signature and no place to hang an
associated store type, a capability declaration, or a second constructor; a trait
has all three, and in Rust the capability declaration can be an associated
`const`, which makes it available before the rule body runs and constant after
monomorphisation.

That trait is `Fixture` (`crates/happenstance-testkit/src/contract.rs:108-207`),
and the keyword is now `fixture =`. What follows is what it must satisfy.

**CF-15.** The fixture MUST be a trait, not a bare `Fn() -> S`. Each *fixture
instance* is one isolated backing store; each `connect()` on that instance
returns a handle onto it. Two fixture instances MUST share nothing. `[FROZEN]`
Rule: `two_fixture_instances_observe_none_of_each_others_appends` (the suite
rule; two instances, an append to one, an empty read from the other), and
`fixture_isolation` (the meta-test; a deliberately shared-backing fixture in the
testkit's own `tests/`, asserting the suite rule fails against it).
Cases: E2E-08, E2E-09.
Rejects: the file-backed adapter that points every fixture at one temp path — the
mistake the crate documentation used to warn about in prose and nothing detected
("*The expression is re-evaluated for every test; a file-backed adapter should
point it at a temporary directory*", `git show 23fd446:crates/happenstance-testkit/src/lib.rs`,
`:26-28`; the sentence is gone, and the rule is what replaced it). Under the
factory shape it produced cross-test contamination that surfaced as an unrelated
rule failing intermittently; under CF-15 it fails one named rule. It takes both,
and which does what is the whole point: the mistake is an **adapter's**, and a
meta-test over the testkit's own fixture never sees an adapter's fixture, so only
a rule the adapter runs can catch it. The meta-test is CF-1's half — the proof
that the rule can fail at all.

**CF-16.** The fixture MUST be able to open a **second handle** onto the same
backing store, and rules MAY require it. `[FROZEN]`
Rule: `two_handles_observe_each_others_appends`.
Cases: E2E-08.
Rejects: an adapter whose correctness is per-session — a cached `max(position)`
fast path, a per-connection repeatable-read snapshot, an advisory lock scoped to
one pool member. All three passed all twenty-seven rules the suite had before
this clause landed, and all three are strategies the decision ledger defers
rather than rules out.

It also rejects the escape route, and that is the half stage 3 added. The
capability is spelled `SECOND_HANDLE` on `Fixture`, so an adapter author meeting
a red `two_handles_observe_each_others_appends` can decline it — and under
CF-18's skip machinery that bought a green suite and one `SKIP` line for an
adapter nothing had reached through two connections. The rule therefore uses
`must!` rather than `require!`: a declined `SECOND_HANDLE` **fails** the rule,
quoting the fixture's own stated reason. The MUST is enforced in the rule an
adapter runs, not only in the testkit's own meta-tests, which never execute in an
adapter's CI. `DecliningFixture` in
`crates/happenstance-testkit/tests/mutation_coverage/variants.rs` is the wrong
implementation it is shown to reject.

**CF-17.** The fixture SHOULD be able to **reopen**: invalidate every
outstanding handle's process-level state such that a subsequent `connect()`
observes only what was durably committed. The capability constant is `REOPEN`.
A fixture declaring `REOPEN` supported MUST make `reopen` discard that state
**over a medium that outlives the process's hold on it**, so that the subsequent
`connect()` reads what was durably committed rather than what a live object still
happens to hold. The fixture MUST state the mechanism. A fixture over a store
with no medium outside the process MUST decline the capability with that as its
stated reason.
`[PROVISIONAL — falsified by a legitimate adapter that is durable and cannot
express even a reopen through this contract. The Durable Object is why the weaker
half is the one named: its storage outlives the isolate, so it can discard handle
state and read the store again, and its isolate cannot be restarted from inside a
test at all. **The rule shape is confirmed against the first real one.**
`happenstance-sqlite` is durable, file-backed, and expresses a reopen through this
contract with nothing added to it — close every `rusqlite::Connection`,
checkpoint the write-ahead log, leave the file alone — so the weaker operation was
sufficient for the first adapter that could have forced the split and did not. The
marker stays at this level rather than moving, because one adapter is not the
spread that freezes a capability: the two the deferral was written against
(HS-P0013, HS-P0014) have not answered, and moving a maturity marker is an ADR's
act.]`
Rule: `acknowledged_writes_survive_a_reopen` — the same rule CF-14 and ES-35
name. `DurableFixture` in the testkit's own `tests/` was for two phases the only
fixture in the workspace supplying this capability, and therefore the only reason
the rule executed rather than reporting a skip everywhere; since phase 8
`SqliteFixture` in `crates/happenstance-sqlite/tests/support/mod.rs` supplies it
too, and it is the first that supplies it over a medium outside the process.
Cases: E2E-07.
Rejects: `NoopReopenFixture` in the testkit's own `tests/` — declares the
capability supported and **overrides `reopen` with an empty body** over a
completely correct but entirely volatile store, so it never reaches the trait's
panic. It is what the declaration MUST added above is written against, and it is
**not** a registered mutant, because **no rule of this family can reject it**,
and that is a property of the capability rather than a gap in the rule set:
arming a mid-batch fault has a port-observable consequence — the append must
answer `Err`, which is what CF-39 is written on — and reopening has none. A
correct `reopen` over a durable medium and an empty one over a `Vec` produce
byte-identical observations through `EventStore`.
That is measured rather than argued. `LiveHandleReopenFixture` in the testkit's
own `tests/` is `SqliteFixture` in miniature — an honest reopen that closes the
connections the *fixture* holds and leaves the medium alone, because reopening a
file does not replace it — and it answers every proposed separating observation
exactly as `NoopReopenFixture` does, including the sharpest one: what a handle
taken *before* the call can still do afterwards. An honest reopen that
*replaces* the live log answers differently. So two honest fixtures sit on
opposite sides of that partition with the liar on one of them, and a rule built
from it rejects `happenstance-sqlite`.
`reopen_over_claiming_is_undetectable_and_this_is_the_record` is what carries the
hazard instead: it drives the fixture through every rule and pins the two things
that are measurable — it fails none, and it converts
`acknowledged_writes_survive_a_reopen`,
`reopened_store_does_not_reissue_an_event_id` and `recorded_time_survives_a_reopen`
from reported skips into passes, while an honest twin one line apart reports them
as skips. So the day a rule starts rejecting it, the record goes red.
What the declaration MUST buys is therefore what CF-39 bought for
the write path and no more: the hazard is *stated* rather than undetectable.
A *forgotten* override is a different mistake and is already handled: the trait's
provided body panics and its message names this hazard.
CF-14 still carries the rejection of a store that loses an acknowledged write.
The clause is `SHOULD` rather than `MUST` because `MemoryEventStore` is
legitimately volatile and must stay a first-class fixture; CF-18 is what stops
that from becoming an excuse, and the MUST added above is conditional on
declaring, which is a different sentence.

This clause's first draft deferred a split — `restart` into "reopen the handle"
and "restart the host" — to whichever adapter forced it. The split is
pre-empted rather than deferred: the contract names the weaker operation,
because that is the half every fixture in and planned for the workspace can
honour, and the stronger one gets its own clause when an adapter needs it.
Naming the stronger one first would have bought a capability every fixture
declares `false`, which under CF-18 is a skip reported on every run and
evidence of nothing.

**CF-18.** A fixture MUST declare its capabilities as associated `const`s, and a
rule whose capability requirement is unmet MUST still be emitted as a test that
**reports** the skip with the fixture's stated reason. A rule MUST NOT be
silently omitted. `[FROZEN]`
Rule: `mutation_coverage::capability_skips_are_reported` — a meta-test in
`crates/happenstance-testkit/tests/mutation_coverage.rs`: a fixture declaring no
capabilities must produce the full rule count, with the capability-gated ones
reported as skipped.
Cases: E2E-07, E2E-08.
Rejects: `#[cfg]`-ing capability-gated rules out of the expansion. A rule that
does not appear in the test binary is indistinguishable in CI output from a rule
that passed, so an adapter author who declares `REOPEN: false` to make a red
build green gets a green build and no record of the trade. Requiring a non-empty
reason string alongside each `false` puts the trade in the log where a reviewer
and a user of the adapter can both see it.

The clause governs a capability that is a genuine **trade**, which on `Fixture`
means `REOPEN` and `MID_BATCH_FAULT`. `SECOND_HANDLE` is a MUST (CF-16) and a
MUST is not skippable, so the rule requiring it fails rather than skips; the two
are deliberately not the same mechanism, and reading this clause as licence to
skip a MUST is the misreading CF-16's `Rejects:` now names in terms.

`MID_BATCH_FAULT` arrived at stage 6 with
`append_is_atomic_under_a_mid_batch_fault` and had no clause of its own. Phase 4
took the decision and **CF-39 is the answer**: what a fixture promises when it
declares the capability supported, in the way CF-16 and CF-17 do for the other
two. It remains unlike the other two in one way, and CF-39 keeps that property:
it is defaulted on the trait, so a fixture that never mentions it declines it,
which is the answer an in-memory store would give anyway.

The same section now also carries CF-40, which is the other half of the same
observation one level along — a fixture stating a *fact* about its store rather
than declining a capability.

**CF-39.** A fixture declaring `MID_BATCH_FAULT` supported MUST, when armed at
*k* < `events.len()`, cause the write of the *k*-th event to fail **inside the
store's own write path**, by a mechanism the store cannot absorb, so that the
append returns `Err`. The fixture MUST state the mechanism. A fixture whose
store can absorb every fault it is able to arm MUST decline the capability with
that as its stated reason.
`[PROVISIONAL — falsified by a real adapter whose only injectable mid-batch
fault is one its driver transparently absorbs, such as a connection killed
mid-statement behind a reconnect-and-retry pool; that would make "the append
returns Err" a promise no fixture over that adapter can keep. The instruments
are the rusqlite adapter at phase 8 and the Postgres adapter at phase 10, and no
adapter has armed a fault yet.]`
Rule: `arming_a_mid_batch_fault_makes_the_append_fail`, plus the existing
`append_is_atomic_under_a_mid_batch_fault`, which this one makes non-vacuous.
Cases: E2E-07, E2E-39.
Rejects: `NoopFaultFixture` in the testkit's own `tests/` — declares the
capability supported and **overrides `arm_mid_batch_fault` with an empty body**,
so it never reaches the trait's panic. It passes
`append_is_atomic_under_a_mid_batch_fault` today and would report a green
atomicity result for a store that has never been faulted. A *forgotten* override
is a different mistake and is already handled: the trait's provided body panics
and its message names this hazard.

The clause is deliberately about the **fixture's** promise rather than the
store's, for the reason ES-18 establishes: a decorator sitting above `append`
cannot reach between two rows of one transaction, so the injection has to be the
fixture's and there is nothing else to constrain. What it costs is recorded on
ES-18: the `Ok` branch of `append_is_atomic_under_a_mid_batch_fault` becomes
unreachable for a conformant fixture, and is retained deliberately.

**CF-40.** A fixture MUST state its store's capacity ceilings as
`Option<usize>` associated constants — `MAX_EVENT_DATA_LEN`,
`MAX_TAGS_PER_EVENT`, `MAX_EVENTS_PER_BATCH` — each defaulting to `None`. A
stated ceiling is a promise about the store: a value at exactly that limit MUST
be accepted and a value one larger MUST be refused as
`AppendError::ExceedsStoreLimit` naming the corresponding `StoreLimit`. `None`
means the store has no ceiling for that limit, and a rule that needs one MUST
report a skip rather than being omitted.

They MUST NOT be spelled as `Capability`. A declined capability is a **trade** —
the fixture could have co-operated and its stated reason is the record of what
was given up — and `None` here is a store reporting a **fact about itself**,
with nothing to justify and no reason it owes anybody. Requiring one would put a
fiction in the CI log. What the two share is the *reporting* obligation, and only
that: the skip carries the testkit's own `NO_CEILING_REASON`, because "this store
has no ceiling" is the same sentence for every store that says it.

`[PROVISIONAL — falsified by a real adapter whose ceiling is not a constant: a
Postgres row whose TOAST threshold depends on what else is in the row, or a KV
store whose per-value cap varies with the key, would make a single
`Option<usize>` unable to say where the boundary is, and the rule built on it
would be asserting a number the store cannot honour. The first named instrument
has landed and answered the **constant-ceiling** half: `happenstance-sqlite`
states all three, each mirrored from the adapter's own `pub const` rather than
restated (`crates/happenstance-sqlite/tests/support/mod.rs:189-195`, mirroring
`crates/happenstance-sqlite/src/event_store.rs:284`, `:291` and `:300`), and
`append_reports_exceeded_store_limits` runs there rather than skipping. On a
store whose ceilings *are* constants, an `Option<usize>` says exactly where the
boundary is and the rule asserts a number the store honours. What is still live
is the other half, and it is narrower than "an adapter" was: a store whose
ceiling is **not** a constant — the Postgres row whose TOAST threshold moves
with the rest of the row, the KV store whose per-value cap moves with the key —
for which no single number is honest. The instruments are the Cloudflare adapter
at phase 9 and the Postgres adapter at phase 10, whichever states a varying
ceiling first.]`
Rule: `append_reports_exceeded_store_limits`, which is unwritable without it —
a rule cannot locate a boundary the store has not named, and asserting a
hard-coded one would be a `MAX_EVENT_DATA_LEN` constant by another route, which
VT-21 forbids in terms.
Cases: E2E-35, E2E-42.
Rejects: a fixture that states a ceiling its store does not have, or does not
enforce distinguishably. Four are compiled and registered, in two pairs — one
pair refuses through `AppendError::Store` (`PayloadCeilingStore`,
`BatchParameterCeilingStore`) and one does not refuse at all
(`TruncatingPayloadStore`, `ChunkLosingBatchStore`). It also rejects the shape
this clause exists to prevent: a suite in which **no** fixture states a ceiling,
where `append_reports_exceeded_store_limits` skips everywhere and VT-25 is a
variant nothing exercises. `GappedPositionFixture` states all three at
VT-21 – VT-24's floors and is what makes the rule run.

**CF-19.** A rule MUST assert that an append made through one handle is visible
to an append condition evaluated through a second handle onto the same backing
store. `[FROZEN]`
Rule: `two_handles_observe_each_others_appends`. `CachedHeadFixture`, in the
testkit's own `tests/`, is the wrong implementation it is shown to reject: it
reads correctly through a second handle and evaluates its append condition
against a `max(position)` cache only its own appends refresh.
Cases: E2E-08.
Rejects: the same three session-scoped strategies as CF-16, from the append side
rather than the read side. `racing_conditional_appends_elect_one_winner` is the
rule this looks like and is not: it is sequential *and* single-handle, so it pins the
semantics of a race without ever running one.

**CF-20.** The fixture trait MUST be defined without a `Send` bound and MUST NOT
be `trait_variant`-derived. `[FROZEN]`
Rule: the wasm32 step of `cargo xtask ci` (`xtask/src/main.rs:192-283`), extended
to build the testkit for `wasm32-unknown-unknown`.
Cases: E2E-52, E2E-30.
Rejects: a fixture trait carrying `Self: Send`, which would make the suite
unusable against precisely the adapters ADR-0001 exists for. The asymmetry with
`EventStore` is deliberate and worth stating: `EventStore` needs two flavours
because a *caller* may need to `tokio::spawn` a store's future
(ADR-0001:24-27), whereas nothing ever spawns a fixture — the harness owns the
executor, and the fixture is only ever driven by the rule that holds it. A second
flavour would double the surface to buy a property no caller wants.

**CF-21.** The proptest generators MUST be exported from the testkit's public
`fixtures` module rather than living in a test binary. `[FROZEN]`
Rule: a doctest in `fixtures` constructing a strategy, which fails to compile if
the item is not public.
Cases: E2E-32.
Rejects: the arrangement this clause was written against, which made the
testkit's own claim false. `crates/happenstance-testkit/tests/properties.rs:6-10`
says the laws live in the testkit "because they are the same claims an adapter
must satisfy — an adapter that pushes query matching down into SQL ... should be
able to reuse the generators", and the generators were private functions in that
same integration-test binary, reachable by nothing. They now live in the
`strategies` module of `crates/happenstance-testkit/src/fixtures.rs`, beside a
re-export of the `proptest` they speak — exporting a generator without the trait
its return type names is only half an export — and `properties.rs` imports
them. An adapter author writing their own will not reproduce the five-symbol tag
alphabet, chosen so that collisions and duplicates actually occur, and their
property tests will therefore never generate the inputs that break a merge-scan
at its boundaries — which is why the alphabet is exported rather than described.

---

### 6.4 Runtime independence

`event_store_conformance!` emits `#[tokio::test]` — from `__emit_tokio`, in
`crates/happenstance-testkit/src/registry.rs`, which is where the attribute is
still written — and the crate documentation tells adapter authors to add tokio
to their dev-dependencies (`crates/happenstance-testkit/src/lib.rs:28-29`). That
is a hard exclusion of the
target the two-flavour port
design exists to serve: a Workers runtime cannot run a tokio test, so the wasm
adapter — the whole justification for ADR-0001 — cannot run the suite that would
prove it conformant. The rules are the asset. The `#[tokio::test]` attribute is
an implementation detail that has been welded to them.

The obvious fix — a `&[Rule]` registry of function pointers — does not compile,
and the reason is worth stating because it recurs. Each rule is generic over
`S: EventStore`, and `EventStore` is not dyn-compatible (ADR-0001:88-90), so a
rule cannot be a trait object. Each rule is also an `async fn`, so its return
type is a distinct anonymous opaque type; storing several uniformly requires
boxing to `Pin<Box<dyn Future<Output = ()> + '_>>`, and *that* forces the
registry to pick a flavour. A bare `dyn Future` is not `Send`, so a tokio
multi-threaded harness cannot spawn it; adding `+ Send` excludes the wasm
flavour. The registry would have to make exactly the choice ADR-0001 refuses to
make, one level up.

A macro that enumerates the rules and hands each to a caller-supplied callback
avoids the choice entirely: every harness monomorphises the same list at its own
bound.

**CF-22.** The rule set MUST be enumerated in exactly one place, as a macro
taking a callback — `for_each_event_store_rule!($emit)` — and every harness MUST
be built by invoking it. `[FROZEN]`
Rule: `registry::no_orphan_rules`, at the foot of
`crates/happenstance-testkit/src/registry.rs` — the same meta-test CF-24 claims, and deliberately so. Two clauses over one check is legitimate when
the check answers both: it walks the enumeration and a scan of `suite.rs` in both
directions, so a rule in one and not the other fails it whichever side is short.
There is no second meta-test named *every_rule_is_enumerated* — the name is
written unbacked here because it resolves to nothing, and this clause used to
claim it did.
What the check does *not* see is the half of this clause that is structural. A
second hand-maintained list would not make the enumeration and `suite.rs`
disagree; it would run a subset under a different name and the meta-test would
stay green. That half is held by review, and by there being exactly one macro to
call.
Cases: E2E-52, E2E-30, E2E-09.
Rejects: the arrangement this clause was written against, and it is worth naming
exactly because seven other passages in this document went on citing it after it
was gone. The rule bodies lived in `suite.rs`, and `event_store_conformance!`
expanded to an inner `macro_rules! conformance_test` followed by a
hand-maintained list of twenty-seven `conformance_test!(…)` invocations in the
same expansion (`git show 927d291:crates/happenstance-testkit/src/lib.rs`,
`:90-97` for the inner macro and `:100-136` for the list). The two agreed by hand
and nothing made them agree. That arrangement was retired at phase 1 (`23fd446`)
and `conformance_test!` no longer exists anywhere in the workspace:
`for_each_event_store_rule!`, in
`crates/happenstance-testkit/src/registry.rs`, has been the single enumeration
ever since, `event_store_conformance!`
(`crates/happenstance-testkit/src/lib.rs:613`) is built by invoking it, and
`no_orphan_rules` is what now makes the two agree. What the clause forbids from
here is a second hand-maintained list of the *same* family — reintroducing the
pair that drifts, under whatever name — which is the arrangement the paragraph
below distinguishes from the three per-family enumerations that are intended.

"Exactly one place" is per rule *family*, and phase 3 lands two more of them.
`event_store_model_conformance!` and `event_store_concurrency_conformance!` carry
their own enumerations because their bounds differ: the concurrency flavour needs
`S: SendEventStore + Send + Sync + 'static` and an `Arc<S>`, which is the bound
ADR-0001 refuses to put on `EventStore` and therefore the bound
`for_each_event_store_rule!` cannot carry. Each enumeration lives beside the
rules it enumerates — `suite.rs` for the event-store family, their own modules
for the other two, neither of which `spec-trace` reads. What this clause forbids
is a second list of the *same* family, because that is the pair that drifts;
three families with one list each is the arrangement, not the exception.

**The model family landed at stage 5**, and it is the worked example of the
paragraph above rather than a plan for one. Its enumeration is
`for_each_model_rule!` at `crates/happenstance-testkit/src/model.rs:772`, and it
lives beside the single rule it names, `ops_agree_with_the_model` at
`crates/happenstance-testkit/src/model.rs:658`. `event_store_model_conformance!`
is built by invoking it exactly as `event_store_conformance!` is built by
invoking `for_each_event_store_rule!`. Two things it settles, and both were
open:

* **The emitters are duplicated, not parameterised.** An emitter is invoked as
  `emitter!(rule_a, rule_b, …)` and the rules' *module path* is baked into its
  expansion rather than passed, so a second family needs `__emit_model_tokio`
  and `__emit_model_blocking`. That is duplication of eight lines against a
  change to a contract three shipped emitters, three in-tree harnesses and CF-23
  all depend on. The alternative that avoids both — re-exporting the model rules
  into `rules` so `__emit_tokio` resolves them — is refused: it puts an
  unregistered name into the module `no_orphan_rules` scans, in the one blind
  spot that test documents.
* **It is additive, and its blind spot is measured rather than argued.**
  `mutation_coverage::the_model_rule_rejects_exactly_what_it_claims` drives the
  model family against every store in the proof artefact and pins the answer for
  each. `REGISTRY` today holds seventy-nine rows — seventy-seven mutants and two
  conformant variants — and `MODEL_COVERAGE` pins thirty-eight `Rejected` against
  forty-one `Agreed`. So it rejects **thirty-eight of the seventy-seven** mutants
  and neither of the two conformant variants — `GappedPositionStore` included,
  which is what the symbolic anchor exists for (CF-6).

  **The thirty-nine it does not reject are more shapes than the partition below
  knows about, and the partition is what has gone stale rather than the
  argument.** What follows accounts for **twenty** of the thirty-nine by name. It
  was written when twenty was the whole of them, and nineteen further misses have
  been registered since — none of which refutes it and none of which it covers.
  `MODEL_COVERAGE`'s own doc comment carries the same twenty under the same
  three-shape heading, which is where the re-derivation belongs first: re-deriving
  the shapes against the current registry is analysis rather than a recount, and
  is owed as its own deliberate pass rather than folded into an arithmetic fix.

  Of that twenty, the first nine are *reachability*: the
  model drives one handle, on one fixture, through a strictly sequential stream
  of non-empty batches, and never reopens, so a defect whose content is the empty
  batch, a second handle, a second fixture instance, durability, or a *window*
  between two overlapping futures is unreachable from it however many cases it
  runs. Three of those are the concurrency family's own subject matter, which is
  the argument for that family arriving from the other direction.

  The tenth is a shape of its own and was mis-filed under the second until stage
  6's review: `NoTransactionStore`'s defect does not exist until a fault has been
  **armed**, and arming is a call on the *fixture*. A generator that emitted one
  would be generating a fixture call rather than a store operation, which is not
  what an `Op` is; unarmed, that store is an ordinary correct one.

  The other ten arrived at stage 6 and are *value range*: the generators emit
  typical values — a small non-empty payload, metadata that is `None` or
  non-empty, up to six tags from a five-symbol alphabet, queries of up to three
  items — and every value-edge mutant is wrong only at a boundary none of those
  reach. That is the honest shape of a property test's coverage and it is the
  reason the value edges are *named* rules rather than generated cases: two of
  the exclusions are written on the generator itself, where they were left
  because the clause that owned the question had not been written yet.

**The concurrency family landed at stage 5 as well**, and it settles the same
paragraph from the other end — *and corrects the bound this clause predicts for
it.* Its enumeration is `for_each_concurrency_rule!` in
`crates/happenstance-testkit/src/concurrency.rs`, beside the five rules it names,
and `event_store_concurrency_conformance!` is built by invoking it. Four things
it settles:

* **The predicted bound was wrong in three of its four parts, and the correction
  is a measurement rather than a preference.** This clause says the flavour needs
  `S: SendEventStore + Send + Sync + 'static` and an `Arc<S>`. What it needs is
  `F::Store: EventStore + Send`. `Sync` and `'static` are obligations of
  `tokio::spawn` rather than of the port, and the family's contenders are
  `std::thread::scope` threads, which impose neither — a contender may borrow the
  condition and the events from the rule's own frame. `SendEventStore` is not
  needed because **the future never crosses a thread boundary**: the *handle*
  does, and the future is created on the contender's thread by `block_on` and
  finishes there. Each was established by removing the bound and compiling, which
  is `spawns_from_generic`'s method. The normative content of this clause — one
  enumeration per family, and no second list of the same family — is untouched;
  what is corrected is the illustration, and the illustration was written before
  anybody had compiled the thing it describes.
* **It is opt-in and target-gated.** `wasm32-unknown-unknown` has no threads to
  race on, so the module does not exist there, and a `!Send` adapter cannot
  invoke it and must not be expected to. That is the same asymmetry CF-20 draws
  for the fixture trait, and it is why the *port* still carries no `Send` bound.
* **One shipped emitter would have been enough and two are shipped anyway.**
  `wasm_bindgen_test` is unreachable for a rule set that does not exist on that
  target, and the parallelism lives in `std::thread::scope` rather than in the
  runtime, so the blocking emitter races exactly as hard as the tokio one. CF-23's
  content is that the wrapper is a *parameter*; a family with exactly one emitter
  reads as a family that forgot.
* **Non-vacuity is measured, by a table of its own.**
  `mutation_coverage::the_concurrency_rules_reject_exactly_what_they_claim` drives
  six `Arc`/`Mutex` stores — one conformant control and five defects that no
  sequential rule in the suite can see — through all five rules and pins every
  verdict, in both of CF-3's directions and with a per-rule assertion pin. They
  are a second table (`RACERS`) rather than rows in `REGISTRY`, and the reason is
  mechanical: every one of them fails **no** rule of the event-store family, and
  `mutant_registry_is_exhaustive` rejects a row with an empty `fails` list.

**CF-23.** The testkit MUST NOT emit any runtime-specific attribute from its own
expansion. The per-test wrapper MUST be a parameter supplied by the adapter. The
reason is `wasm32` portability and not `Send`-ness: `tokio::spawn` requires
`Send`, but `Runtime::block_on` — which `#[tokio::test]` expands to — does not, so
the shipped tokio emitter already drives a `!Send` store, and
`crates/happenstance-testkit/tests/local_conformance.rs:470-478` — harness **2**,
the testkit's macro verbatim on the multi-threaded runtime — is the live
demonstration (ADR-0010:107-112). The citation named harness 1 for a stage, which
is the runtime-free `__emit_blocking` one whose own comment reads *"No async
runtime is involved at all"*: in range, so `check_citations` was silent, and
pointing at the harness that proves nothing about tokio. What no tokio attribute can do is exist on a
target that has no tokio. `[FROZEN]`
Rule: the wasm32 steps of `cargo xtask ci`, which compile a `wasm-bindgen-test`
harness over the same rule set and then execute it under
`wasm-bindgen-test-runner`, behind a mandatory guard that an emptied or
subsetted target fails.
Cases: E2E-52, E2E-30.
Rejects: a runtime attribute emitted from the testkit's own expansion — the
shape, not a line, because the line moves and the shape is the defect.
`#[tokio::test]` is still written inside this crate, but only inside
`__emit_tokio`, which is a *parameter* the adapter selects and can replace;
what this clause forbids is the version where no such parameter exists and every
harness gets tokio whether or not the target has one. It is rejected for where it
cannot run rather than for what it cannot drive; and equally a "fix" that swaps
it for a
`cfg`-selected attribute inside the testkit — that keeps the list of supported
runtimes in the testkit's source, so a runtime nobody anticipated (a `LocalSet`
harness, `futures::executor::block_on`, an embedded executor) needs a testkit
release to become usable. Three harnesses MUST be demonstrated in-tree: tokio,
a blocking single-threaded one, and `wasm-bindgen-test`. Two would let a
one-off accident pass for a design.

**CF-24.** No rule may exist in the suite without appearing in the enumeration.
`[FROZEN]`
Rule: `registry::no_orphan_rules`, at the foot of
`crates/happenstance-testkit/src/registry.rs`, which is on disk and green. It compares the enumeration against a scan of `suite.rs`'s own source,
baked in with `include_str!`, because Rust has no reflection and "the public
items of `rules`" cannot be asked for at run time. The scan's limits are stated
on the test: it cannot see a rule introduced by a macro expansion, by a
`pub use`, or from a `#[path]`-included file.

**It covers the event-store family and only that family.** `include_str!` names
`suite.rs`, so a `pub async fn` added to `concurrency::rules` or `model::rules`
and left out of `for_each_concurrency_rule!` or `for_each_model_rule!` is unrun
and unreported — the same silent no-op this clause is about, one family over.
That is a different gap from CF-22's, which records only that `spec-trace` does
not *read* those two modules: `the_concurrency_rules_reject_exactly_what_they_claim`
closes CF-1 for its family but not this clause, because it enumerates *from*
`for_each_concurrency_rule!` and a rule missing from that list is missing from
the check too. The mechanical fix is cheap and deliberately not taken yet —
extend `declared_rules()` to scan the two modules against their own
enumerations, behind the same `#[cfg(test)]` — because a third family is
plausible in phase 4 and one scanner written against three is better than three
written one at a time. CONTRIBUTING states the same gap where a rule author will
meet it.
Cases: all contract-level cases.
Rejects: the silent no-op — a rule written, reviewed, merged, and never run
because its registration line was forgotten. Under the two-list shape CF-22
describes, that failure produced no signal of any kind: the rule compiled,
`cargo test` reported green, and an adapter was certified against twenty-six
rules while its author believed it was twenty-seven. That shape was retired at
phase 1 (`23fd446`) together with the second list itself, so this clause is now
enforced rather than aspirational — and the two clauses agree, which they did not
while this one still called `no_orphan_rules` new.

---

### 6.5 The instrument portfolio

CLAUDE.md's standing rule: *a port is only as well-designed as the spread of what
implements it*, and before freezing a port you must name the axis it is most
likely to be wrong about and check that something in the workspace sits at the
other end. The rule is stated. It has been executed on four axes with a
*fixture* and, since phase 8, at three far ends with an adapter — durability's,
handle multiplicity's and batch shape's. All three arrived in the same crate,
which is the qualification the rows carry rather than a footnote to them: every
implementation the workspace can run is still one storage shape wearing several
hats.

Two kinds of instrument satisfy it and they answer different questions. A
**fixture instrument** lives in the testkit's `tests/` and exists to prove a rule
*can fail* — it is deterministic, cheap, and it is what CF-1 demands. An
**adapter instrument** is a real adapter at the far end of an axis and exists to
prove a real implementation at that end *can pass*. Neither substitutes for the
other: a rule with only a fixture instrument may be unsatisfiable in practice, and
an adapter with no fixture instrument may be passing a rule that could not have
failed.

**CF-25.** Before any port in this specification is declared `[FROZEN]`, every
axis in the portfolio table MUST have a passing implementation at both ends, or
the freeze MUST name the axis it is accepting risk on and the ADR that accepts
it. An acceptance recorded for an axis on which the frozen port does not sit is
**pro forma**: it names the exposure and does not discharge it for the port that
does sit on that axis. `[FROZEN]`
Rule: `cargo xtask spec-trace` (CF-38), which reads the portfolio table and the
maturity markers and fails when a `[FROZEN]` port clause has an axis with no
far-end row and no named risk acceptance.
Cases: E2E-01, E2E-02, E2E-24, E2E-46, E2E-52.
Rejects: freezing `EventStore` against `MemoryEventStore`, a `RefCell` store,
rusqlite and a Durable Object — four adapters, one storage shape, every one of
them serialising its writers and assigning positions under a lock it holds until
commit (`memory.rs:370-403`). A port frozen against that population is frozen
against SQLite wearing four hats, and the property it will be wrong about is the
one all four share.

**CF-26.** A fixture instrument satisfies CF-1 and CF-25's falsifiability half.
It does not satisfy CF-25's implementability half; that requires an adapter
instrument. `[FROZEN]`
Rule: the portfolio table's `Far end exists` column, checked by
`cargo xtask spec-trace` (CF-38).
Cases: E2E-01, E2E-24.
Rejects: declaring the position-allocation axis covered because the hostile
visibility fixture of CF-13 exists. It does exist now —
`PreCommitPositionStore` — which is what turns this from a hypothetical into the
live reading error. The fixture proves the rule bites. Whether a real Postgres
adapter can *pass* it — and at what cost among `xid8` + `pg_snapshot_xmin`,
transaction-scoped advisory locks and a serialised sequence table — is a
measurement, and PRESSURE-TEST.md:662-666 is right that it is owed one.

**CF-27.** The workspace MUST hold a **completeness instrument**: a
testkit-adjacent store that deliberately holds only a suffix of its own log, and
reports that it does. `[DEFERRED — the experiment is building it as a decorator
over any `EventStore` and running the full suite against it; the outcome that
matters is the list of rules that pass, which is the list of rules that cannot
tell a pruned store from a young one. Owned by the pass that settles retention
and deletion — E2E-CASES.md:1579-1585's eleventh open decision — because the
instrument's report shape and that decision's port surface are the same surface.]`
Rule: `suffix_store_is_distinguishable_from_a_young_store` (new; its assertion is
fixed by the retention decision, which is ES-39's). This is the instrument, not
the primitive: ES-39 defers the port surface a store would report through, and
ES-38's `positions_are_not_reused_after_removal` and ES-40's
`condition_over_removed_history_does_not_reject` are the two rules that already
have assertions and are waiting only on this instrument to be written against.
Cases: E2E-46, E2E-47, E2E-56, E2E-33.
Rejects: every conformant store's silence about its own history. A 90-day prune
happens entirely outside the port — `EventStore` has two methods and neither
deletes (`read` at `store.rs:167-171`, `append` at `store.rs:261-265`) — and
afterwards the store passes every rule
unchanged, including `query_all_matches_every_event`,
whose contract is store-relative by wording and therefore accidentally correct. A
holed log and a young log are the same value. Four of the six scenarios reach
this from unrelated doors — a pruned device slice, a regulated scattered purge, a
compacted peer, an epoch count derived from a slice — which is the strongest
available evidence that it belongs in the port rather than in an adapter, and
CLAUDE.md names an instrument for the two axes it had already noticed and nothing
for this one.

**CF-28.** The workspace MUST hold a `!Send` reference store in the testkit's own
`tests/`, and it MUST pass the suite. `Rc` is what supplies the `!Send`, not
`RefCell`: `RefCell<T>: Send where T: Send` — it surrenders `Sync`, not `Send` —
so a store built from `RefCell<Vec<_>>` alone is perfectly `Send` and would prove
nothing about the bare flavour, which is why the reference store holds its log
through an `Rc` (`crates/happenstance-testkit/tests/local_conformance.rs:59-74`,
ADR-0010:113-115). `[FROZEN]`
Rule: the existing suite, invoked against that `Rc<RefCell<Vec<_>>>`-backed store
under any harness that does not require `Send` — which, as CF-23 records, includes
the default multi-threaded `#[tokio::test]`, because the attribute expands to
`Runtime::block_on` and only `tokio::spawn` needs the bound.
Cases: E2E-52, E2E-09, E2E-30.
Rejects: ADR-0001's own provisional status, which this store is what retired. The
ADR stated plainly that **no `!Send` implementation of these ports exists
anywhere, not even a reference one** (`references/adr/0001-async-port-flavours.md:8-16`)
and named this exact store as the cheapest proof that lifts it; until it existed,
the two-flavour design's entire evidence base was a `cargo check` for `wasm32` — a
compile of the trait, not of an implementation. `LocalMemoryEventStore` landed at
phase 1 and the marker was lifted on 2026-08-06
(`references/adr/0001-async-port-flavours.md:5-6`). What the clause forbids from here is
deleting the store, or weakening it to a shape that would compile without the
`Rc` — which is the same reversal arriving as a tidy-up. The store also carries
E2E-09's re-entrancy question, which `MemoryEventStore` cannot:
`memory.rs:293-420` holds no lock across a suspension point because it contains no
`.await` at all.

#### The portfolio

| Axis | Near end — what exists | Far end | Far end exists? | Kind of instrument needed |
|---|---|---|---|---|
| **Position allocation** | Assigned under the lock held until commit — `memory.rs:370-403`, and every planned adapter | Allocated outside the transaction; visibility order ≠ position order (`nextval()`) | **Fixture yes, adapter no.** `PreCommitPositionStore` in the testkit's own `tests/` takes its position before commit and publishes after, and `nothing_below_an_observed_position_appears_later` (CF-13) fails it deterministically on one thread. `happenstance-postgres` is still a phase-2 skeleton (`crates/happenstance-postgres/src/event_store.rs:121`), which falsifies a signature and is not a far end | Fixture (CF-13) — **done**; then a Postgres adapter to prove it passable, and at what cost |
| **Transport** | In-process, a handle held across awaits — `MemoryEventStore`, rusqlite | One-shot HTTP: no connection, no interactive transaction, no cursor | **No.** `happenstance-neon` is a phase-2 skeleton (`crates/happenstance-neon/src/event_store.rs:168`), which falsifies a signature and is not a far end | Adapter |
| **Async flavour** | `Send` — `impl SendEventStore for MemoryEventStore` (`memory.rs:293`) | `!Send`: `Rc`-shared, single-threaded, futures that are not `Send` | **Fixture yes, adapter no.** `LocalMemoryEventStore` passes the suite natively and on `wasm32` (CF-28 satisfied, ADR-0008); no real `!Send` adapter until phase 9 | Fixture (CF-28) — **done**; then the Cloudflare adapter |
| **Batch shape** (`ProjectionStore`) | A live transaction held across awaits — `LiveHandleProjectionStore` binds a borrowed `GraphWriteHandle<'a>` on the **`Send`** flavour with real bodies (`experiments/live-handle-projection-batch/live_handle.rs:174-223`); `PostgresProjectionStore` binds `Transaction<'static, Postgres>` | A deferred write set buffered and replayed in one call at commit — `SqliteBatch`, `NeonWriteBatch`, `GraphWriteSet` | **Far end yes, near end no — and the suite that was missing now exists.** Five impls, of which two are `todo!()` throughout — `LadybugProjectionStore` and `PostgresProjectionStore`. `NeonProjectionStore` is real in all four methods, `LiveHandleProjectionStore` in all but `checkpoint`, and since phase 8 `SqliteProjectionStore` is real in **all four**, `begin` through `rollback` (`crates/happenstance-sqlite/src/projection_store.rs:552-679`). It is also the one that runs against something: `crates/happenstance-sqlite/tests/projection.rs` mounts `happenstance_testkit::projection_store_conformance!` against a real temporary file and passes it, so this far end carries a real adapter and not only a shape (`references/adapter-shapes.md:297`). What is empty is the **near** end — nothing holds a live transaction across an await and has run anything — and no rusqlite adapter can take it on the `Send` flavour, because `rusqlite::Transaction<'_>` is itself `!Send` and `commit` is rejected on the batch **parameter** even where the store is wrapped to be `Sync` (`crates/happenstance-sqlite/src/projection_store.rs:19-43`) | A live-transaction adapter at the near end. The projection conformance suite — what this cell used to ask for — landed at phase 8 |
| **Completeness** | A store holding its whole log — everything, everywhere | A store holding only a suffix, or a log with a scattered hole | **No, and nothing is planned.** New (CF-27) | Fixture first; a device adapter second |
| **Handle multiplicity** | One handle at a time — what every rule needed before CF-16, and what a rule could not ask past, because a factory call could not say whether it bought isolation or sharing | Two or more handles onto one backing store, concurrent | **Fixture yes, adapter yes — pooling still empty.** `Fixture::connect` (CF-16) is the seam; `MemoryFixture` and `LocalFixture` both declare `SECOND_HANDLE` supported, `two_handles_observe_each_others_appends` (CF-19) runs against both, and `CachedHeadFixture` in the testkit's `tests/` fails it. All three hand out refcount clones of one in-process object. Since phase 8 `SqliteFixture` does not: `connect` opens **another `rusqlite::Connection` onto the same file** (`crates/happenstance-sqlite/tests/support/mod.rs:208`), and `connect_many` races up to 64 of them through the concurrency family (`crates/happenstance-testkit/src/concurrency.rs:1083`). What is still unbuilt is a **pool** — handles a store draws from and returns rather than owns — and cross-*process* handles | Fixture (CF-16) — **done**; file-backed second connection — **done**; then a pool-backed adapter |
| **Durability** | Volatile — `MemoryEventStore` is a `Vec` behind an `RwLock`, and it declines `REOPEN` saying exactly that | Survives a reopen: an acknowledged write is visible to a handle that kept none of the old one's process state | **Fixture yes, adapter yes — fault far end still empty.** Expressible since CF-17: `DurableFixture` supplies `REOPEN`, `acknowledged_writes_survive_a_reopen` runs against it, and `LosingFixture` beside it fails. Since phase 8 `SqliteFixture` supplies it over a **real file**, and `RestampingFixture` is the second failing control — the one that reaches `recorded_time_survives_a_reopen`'s headline assertion instead of dying at its survival anchor. Nothing yet loses a write to a *fault* rather than to an instruction | Fixture (CF-17) — **done**; file-backed adapter — **done**; then a fixture that arms a real fault |

Seven axes, and **the adapter column carries three ticks — durability, handle
multiplicity and batch shape**, all three bought by phase 8, all three from
`happenstance-sqlite`, and each partial in a way its row states. Two axes are
empty at both ends: transport and completeness. Two carry a **fixture**
instrument and no adapter one — async flavour and, since CF-13's fixture landed,
position allocation — which by CF-26
satisfies the falsifiability half and not the implementability half. Durability
carries both ticks for the half phase 8 could buy: `SqliteFixture` runs the three
reopen rules against a real file, while a store that loses a write to a *fault*
rather than to an instruction is supplied by nothing. Handle multiplicity carries
both for the same shape of reason: `SqliteFixture::connect` opens a second
`rusqlite::Connection` onto one file rather than an `Arc` clone, and the
concurrency family races up to 64 of them, while a store drawing handles from a
**pool** it does not own — and handles in different *processes* — is supplied by
nothing. Batch shape's tick is the *one-sided* one, and it is the only axis in
this table whose two ends are not a matched pair of questions:
`SqliteProjectionStore` passes `projection_store_conformance!` at the
replay-at-commit far end, which was never the doubtful end — `MemoryProjectionStore`
and the testkit's buffering variant already sat there — while the live-transaction
near end still holds nothing that has run anything, and PS-2 wants the suite
green at **both**. Phase 3 produced four fixture instruments and zero adapter instruments,
which is the most it could produce; phase 8 produced the first three adapter ones,
and the rest of that column moves at phases 9, 10, 11 and 14 and nowhere earlier.
That is the honest state, and it is the reason this section exists before the
freeze rather than after it.

**The handle-multiplicity entry was left under re-reading by phase 8's durability
pass and is settled here.** That row read *"no connection has ever been opened
twice"*, and `SqliteFixture::connect` had already falsified it. The question
handed over was whether a second real connection puts an *adapter* instrument at
the axis's far end or only a second handle sharing a process. It puts one there:
the far end this table names is "two or more handles onto one backing store,
concurrent", the second connection is opened through `SqliteEventStore::open` and
runs migration 1 rather than bumping a refcount, and CF-19 plus the whole
concurrency family pass against handles obtained that way. The narrower reading —
that only a pool-backed adapter counts — was rejected because it renames the far
end after the fact: pooling is a property of how handles are *acquired*, and it
is recorded as what is still missing rather than as what was always meant. A
cross-process reader is missing for the same reason and is named beside it.

The distinction is worth holding on to now that four rows have moved, because the
temptation is to read the first tick as the axis being covered.
`LocalMemoryEventStore` proves the `!Send` rules *can* be run and *can* fail. It
says nothing about whether a Durable Object, with a real `SqlStorage` and a
`worker::Error`, can pass them. The three rows CF-16, CF-17 and CF-13 moved are
weaker still: an `Arc` clone is a second handle in the sense the *contract* needs
and in no sense a connection pool would recognise, a fixture that reopens by
replaying a `Vec` has never met a fault, and a store that suspends between
allocating and publishing because a rule polled it that way has never met a
transaction. ES-6 was settled on a purpose-built instrument rather than on this
store for the same reason: a reference store's error type is chosen by whoever
wrote the reference store. **Two** of those three rows have since gained a second
tick, and in both cases the second is a different claim rather than a stronger
version of the first: CF-17's `SqliteFixture` reopens a real file instead of
replaying a `Vec` and has still never met a fault, and CF-16's opens a real
second connection instead of an `Arc` clone and has still never been drawn from
a pool. CF-13's has not: nothing in the tree yet allocates a position outside a
transaction.

**Position allocation is the row where the gap between the two halves is widest,
and it is worth saying so where the tick is.** It is the axis the pressure test
and all six scenarios independently ranked first; what CF-13 bought is a rule that
bites, and what is still missing is a *number* — which of `xid8` +
`pg_snapshot_xmin`, transaction-scoped advisory locks or a serialised sequence
table buys the invariant on a real Postgres, and what each costs at write rate.
ES-10 stays `[PROVISIONAL]` against exactly that measurement, and CF-26's
`Rejects:` names reading this tick as coverage.

---

### 6.6 Compatibility policy for the testkit

Adding a conformance rule is a semver-minor change to `happenstance-testkit` that
turns every passing adapter's CI red. That is not a bug — a new rule usually
means a defect was found, and the red build is the point — but it is a policy
decision the crate has never made, and the crate is the moat.

**CF-29.** A conformance rule MAY be added in a minor release, and MUST land in
the same release as its mutant (CF-1) and its changelog entry naming the defect
it detects. `[FROZEN]`
Rule: `mutation_coverage::every_rule_has_a_mutant` (CF-1) enforces the mutant
half mechanically. The changelog half is a `cargo xtask ci` lint step, added at
phase 3 stage 6, and **it is the weakest check in the gate** — which is said here
because the alternative is a clause that reads as though the obligation is
mechanised. It asserts two things: that every rule name in `suite.rs` appears in
`CHANGELOG.md`, and that some entry naming it carries at least 120 characters of
prose per rule it names. The second is a proxy for substance and not a
measurement of it. It cannot tell whether the sentence names a defect, whether
the defect is that rule's, or whether it is true; a keyword test was written
first and produced nine false positives against nine entries that name a defect
as well as anything in the file, which is the evidence that the content question
is review's and not a grep's. What the step does catch is what actually happened:
its first run found **twenty-five of fifty-five rules** with no entry at all,
including every one of the seventeen founding rules, which had been counted here
since phase 1 and never named. The earlier text of this clause said no check was
possible and named the one shape that is not — "`CHANGELOG.md` changed in the
same commit as `suite.rs`", defeated by the amend and unavailable on a dirty tree
— and concluded from that that no check was worth having. That inference was
wrong, and the correction is recorded rather than the sentence deleted.
CONTRIBUTING's rule checklist still carries the half nothing can check.
Cases: all contract-level cases.
Rejects: a rule added quietly in a patch release. An adapter author who takes a
patch bump and finds their build red has no way to distinguish "my adapter has a
defect" from "the suite changed", and the second answer erodes the first's
authority permanently.

**CF-30 is `[NON-NORMATIVE]` and is prose.** It was drafted as a clause requiring
adapters to pin `happenstance-testkit` exactly
(`happenstance-testkit = "=0.4.2"`) in their `dev-dependencies`. Its own `Rejects:`
line conceded that no adapter behaviour violates it and that it "buys
predictability, not correctness", which is CLAUDE.md's definition of decorative,
and the definition binds this document as much as it binds the suite. So: the
recommendation stands and the clause does not. The advice, kept because the
reasoning is not obvious — an exact pin is normally poor practice in Rust because
it propagates: a pinned dependency of a *library* constrains every downstream
lockfile and causes duplicate-version conflicts. Dev-dependencies do not
propagate at all, because Cargo does not resolve the dev-dependencies of a
non-root package, so pinning here costs nobody anything and gives the adapter
author control of when they take a new bar. The testkit's documentation should
say so; nothing checks that it does, and the ID is retained so that citations of
CF-30 resolve to this paragraph rather than dangling.

**CF-31.** A major release of `happenstance-testkit` means a rule's **meaning**
changed, or a rule was removed or renamed — anything that can make a
previously-passing conformant adapter fail for a reason other than a
newly-detected defect. Such a release MUST cite the ADR that changed the
requirement. `[FROZEN]`
Rule: `cargo xtask spec-trace` (CF-38), which fails when a rule name referenced
by a clause no longer exists.
Cases: all contract-level cases.
Rejects: tightening an existing rule in place. If `read_from_is_inclusive` grows
an assertion about ordering, an adapter that was conformant under the
specification as written becomes non-conformant with no specification change —
which is the suite legislating rather than checking. New requirements get new
rules and new clauses; existing rules get corrections, not extensions.

**CF-32.** `happenstance-testkit` MUST carry its own `version` key rather than
`version.workspace = true`. `[FROZEN]`
Rule: a `cargo xtask ci` manifest check, added at phase 3 stage 6. It reads the
`[package]` table of `crates/happenstance-testkit/Cargo.toml` and fails on an
absent `version` key or on one whose value mentions `workspace`. It found nothing
on its first run and that is expected: the manifest has carried its own key, and
a comment saying why, since the clause was written — what had never existed was
anything that would notice the key being folded back into the workspace during a
tidy-up, which is a one-line edit that no test and no compile can see.
Cases: none — this is a packaging obligation with a machine check and no
behavioural case, which CF-35 permits so long as the clause says so. It does.
Rejects: the manifest as it stood when this clause was written, inheriting
`version = "0.1.0"` from the workspace root at `Cargo.toml:6`
(`git show 23fd446:crates/happenstance-testkit/Cargo.toml`). Under a
shared version key the two crates cannot move independently in either direction,
and both directions are wrong. Adding a rule bumps the testkit's minor, which
drags `happenstance-core` to the same number and republishes an unchanged
contract — after which a semver-checking tool has no earlier version of that
contract to diff against. A patch release of the contract republishes the testkit
and forces every adapter to re-run a bar that did not change. The version key is a
statement about what a number means, and these two crates' numbers mean different
things: the contract's is a promise about types, the testkit's is a promise about
the bar.

---

### 6.7 Benchmarks are not conformance

An adapter that scans where it should seek passes every rule that can be written.
Norvant's `SuspendLane` decision is the worked case: a mixed tagged/untagged
two-item query over a log where one item selects a handful of events and the
other selects millions. `MemoryEventStore` evaluates `query.matches` over every
event before applying `from` (`memory.rs:305-325`) and says outright that it is
not built for scale — and no conformance rule can catch that, because complexity
is a benchmark and not an assertion.

**CF-33.** No conformance rule may read a clock, measure elapsed time, or assert
on an operation count. `[FROZEN]`
Rule: a `cargo xtask ci` lint step over `happenstance-testkit/src`, rejecting
`std::time`, `Instant`, `elapsed` and `sleep`. It is a grep, not a type — there
is no lint that expresses "this crate may not observe time" — and saying so is
better than pretending otherwise. Landed at phase 3 stage 6, with three
properties worth stating because each is a place the next person will reach for
the wrong repair. It is scoped to `src/` **deliberately**: `tests/` is where the
concurrency racers live, and a wrong implementation that must lose an update
across two OS threads may legitimately synchronise — CF-33 constrains
conformance rules, which are the library's. It matches **code only**, with
comments removed and string-literal contents blanked, because `concurrency.rs`
explains at length why the watchdog it does not have is forbidden and the word
`sleep` appears four times in that explanation; a lint that fired on the sentence
justifying it would be repaired by deleting the sentence. And its list is the
four constructs above and no others — `Duration` and `timeout` are **not** on it,
because a step that quietly checks more than the clause specifies makes the gate
and the document disagree with the gate winning silently. Extending the list is
an edit to this line, in the same change.
Cases: none; this constrains the suite, not an adapter.
Rejects: the plausible-looking rule that asserts a `backwards().limit(1)` read
returns within some bound. It passes on the author's machine, fails on a loaded
CI runner, fails under a debug build, and makes the suite's verdict a property of
the hardware. A flaky conformance suite is worse than no conformance suite,
because it teaches adapter authors to re-run until green — and that habit is what
lets a real failure through.

**CF-34.** Performance MUST be measured by a separate harness, and that harness
MUST NOT be part of the conformance bar. An adapter that is slow is conformant.
`[PROVISIONAL — falsified if a complexity property turns out to be expressible as
a deterministic assertion rather than a timing. The candidate is an
instrumented fixture that counts rows examined rather than seconds elapsed, which
would be a conformance rule and not a benchmark; if that works, this clause
splits.]`
Rule: none — the harness is not the bar, which is the clause's content.
Cases: E2E-CASES.md:1671-1677 records this as one of the two things that are
neither blocked nor cases.
Rejects: a benchmark result gating a merge. A threshold nobody can justify
becomes a threshold everybody raises, and the number stops meaning anything the
second time it is moved. Benchmarks are published per adapter and compared
against that adapter's own history; they decide nothing about conformance.

**The separation stopped being a plan at phase 8.** `event_store_benchmarks!` now
exists as a fourth macro family in `crates/happenstance-testkit/src/bench.rs`,
behind an off-by-default `bench` feature
(`crates/happenstance-testkit/Cargo.toml:92`), and it is **not** in
`for_each_event_store_rule!` — so an adapter that runs the conformance macros
compiles none of it and the bar is unchanged by its existence. ADR-0022 is its
first paying customer: every number in that record was measured through this
harness rather than through an ad-hoc script. The marker stays `[PROVISIONAL]`
because its falsifier is about a *complexity property expressible as a
deterministic assertion*, which the harness landing neither supplies nor refutes.

---

### 6.8 Traceability

A specification whose clauses drift from the rules that check them is a
specification that is read once. The drift must be discoverable by running
something, not by reading everything — and the same argument that makes a
conformance suite worth having applies to this document.

**CF-35.** Every normative clause in this specification MUST name the rule that
checks it and the E2E cases it serves. A clause naming no rule is either wrong or
belongs in prose, and MUST say which. `[FROZEN]`
Rule: `cargo xtask spec-trace` (CF-38).
Cases: all.
Rejects: the decorative clause — a MUST with nothing that could observe its
violation. CLAUDE.md's rule for conformance rules applies unchanged to
specification clauses, and this document's own form rules exist to make the
violation visible at authoring time rather than at implementation time.

**CF-36.** A clause backed only by **integration**- or **scenario**-level cases
MUST NOT name a conformance rule. `[FROZEN]`
Rule: `cargo xtask spec-trace` (CF-38), cross-referencing each case's level
marker (E2E-CASES.md:19-28).
Cases: E2E-28, E2E-29, E2E-39, E2E-42 (the integration-level cases most likely to
be mistaken for contract-level ones).
Rejects: a rule in the conformance suite that needs two adapters, a domain
vocabulary, or a running deployment. Such a rule cannot be run by an adapter
author against their own crate, which is the one thing the suite is for; it
belongs in the e2e crate, or — for a contract-level case that cannot be expressed
against a single store handle — in `happenstance-sync-testkit`, named at
`crates/happenstance-sync/src/lib.rs:23-24` and not yet existing.

**CF-37.** Every E2E case MUST name the clause or clauses it exercises. `[FROZEN]`
Rule: `cargo xtask spec-trace` (CF-38).
Cases: all.
Rejects: a case that survives the decision it was written to force. E2E-CASES.md
was written before this specification and therefore names no clauses; adding the
back-reference is what makes "is every clause exercised by something concrete?"
and "did this decision leave a case orphaned?" both answerable by a command.

**CF-38.** `cargo xtask ci` MUST run a traceability check that fails on: a clause
naming a rule that does not exist; a conformance rule no clause names **and no
clause disposes of**; a clause naming a case that does not exist; a case naming no
clause; and a `[PROVISIONAL]` or `[DEFERRED]` marker with an empty falsifier or
experiment. `[FROZEN]`

**The checker is authoritative on all five, and §7.2 is its output rather than its
rival.** The table was computed by hand at `2a65d76` and has never been verified;
once the checker exists, §7.2 is generated and MUST NOT be hand-edited, because a
hand-edited generated table is a table that disagrees with the tool silently. What
stays authored is §7.3 through §7.6 — the judgement about *why* a gap exists,
which no checker can produce.

**Dispositions are how the two meet.** The second check would otherwise fail
forever on rules the specification deliberately leaves unclaimed. When this
clause was written, `query_all_matches_every_event` and
`racing_conditional_appends_elect_one_winner` were two such rules — orphaned on
the ground that they were being retired, with §7.4 saying so in prose the checker
cannot read. So a clause MAY dispose of a rule it does not claim, in the form
`Retires: <rule> — <reason>`, and a disposed rule satisfies the check. An orphan
with no disposition still fails. The distinction is the whole point: an unclaimed
rule is either a decision someone made or a rule nobody is responsible for
deleting, and only the author knows which.

**Both of those dispositions were later reversed**, along with the third named
below, and the mechanism outlived the examples it was introduced with: ES-15
claims `query_all_matches_every_event` and ES-25 claims
`racing_conditional_appends_elect_one_winner`, no `Retires:` field survives
anywhere in this document, and §7.4 records three reversals out of three. The
mechanism stays specified because the *next* disposition needs it; what the
history shows is that the examples were the weakest part of it.

**And it is the mechanism that hid a mistake for a phase, which is worth stating
where the mechanism is defined rather than only where the mistake was.** A
disposed rule satisfies check 6 *forever*. `append_is_atomic` was a third entry
in the list above; the rule stayed in `suite.rs`, ES-18's `Retires:` line named
it, the checker was silent, and the retirement turned out to be wrong — the rule
rejects three registered mutants and is the only thing in the suite that rejects
the write-then-check shape's non-atomicity. Nothing mechanical could have caught
that, because a `Retires:` line is an assertion about the *future* and the
checker only reads the present. §7.4 is where the judgement lives, and the moral
is that a `Retires:` line is a claim to re-examine rather than a filing.

The first run's disagreements with the hand-computed table are therefore a defect
list to work through, not a question about who wins. Expect it to find real
errors; that is what it is for.
Rule: `cargo xtask spec-trace` (new step in `xtask/src/main.rs`'s `REQUIRED`
list, `:33-90`).
Cases: all.
Rejects: this document's own decay. The last of the five checks is the one worth
naming separately: a `[PROVISIONAL]` marker with no falsifier is indistinguishable
from a decision nobody wanted to make, and by the time anyone notices, the
provisional clause has been load-bearing for a year. Making the empty falsifier a
build failure is what converts the form rule from a convention into a constraint.

---

### What this section does not settle

**DCB wire interoperability is out of scope here.** Section 2 records it as an
explicitly deferred decision with a named home — WF-1, whose home is a future
`happenstance-dcb-interop` crate — and this section only notes the consequence
for the suite. There are no conformance rules for the wire format, because the
wire format is private to happenstance and its only conformance question is "does
a happenstance instance round-trip its own envelopes" — which is a property test
over the contract's value types (CF-21's exported generators), not an adapter
obligation. Section 2 therefore names tests in
`crates/happenstance-core/tests/wire.rs`
where every other section names conformance rules, and **CF-38's traceability
check must resolve those names too**: a `WF-n` clause whose named test does not
exist is the same defect as an `ES-n` clause whose named rule does not, and a
checker that only reads `happenstance-testkit` would report the whole wire format
as untraceable. **That obligation is discharged.** ADR-0016 §15 taught
`backticked_idents` to keep a `::`-qualified name and gave check 4 and
`rule_cell` a second resolution source — the `#[test] fn` names inside
`crates/happenstance-core/tests/wire.rs` and
`crates/happenstance-sync/tests/wire.rs`, each qualified by its enclosing `mod
wire` — so a `wire::` name a clause cites now resolves, and an unwritten one
renders `†` instead of being silently dropped. Check 6 deliberately keeps
sweeping the suite set alone: resolution is one-way, clauses may name wire tests
and wire tests need not be named by clauses, for the reason this paragraph
already gives. If the format is ever opened to other DCB implementations, that
decision brings a `happenstance-wire-testkit` with it, and its meta-rules are
CF-1 through CF-6 unchanged.

**The projection suite's shape.** Section 4 owns it, and has decided. The
conformance consequence is the whole of the problem and is worth restating in
this section's terms: three of the six projection rules the roadmap specifies —
rollback leaves both unchanged, a dropped batch leaves both unchanged, a failed
commit leaves the store unchanged — cannot observe the read model at all, because
generic suite code holding a `P::Batch<'_>` can only pass it to `commit` or
`rollback` (`projection.rs:504-513`). By CF-1 those three are decorative until
something can write a row. That is not an argument about ergonomics; it is the
reason a suite that can test only the checkpoint half of a two-write invariant
cannot reject an adapter that commits the checkpoint and silently drops the
read-model write.

What section 4 does **not** conclude from that, and this section must not be read
as concluding either, is that `Batch` grows a universal write vocabulary. PS-9
refuses one: a `put(&mut self, key, value)` supertrait would oblige a graph store
and a relational store each to carry a key-value table nobody asked for, and
would reintroduce at the read-model layer the opaque blob ADR-0003 confined to
event payloads. PS-11 supplies the suite's need instead — a `ProjectionProbe`
trait in the contract crate behind a `conformance` feature, which an adapter
implements in order to be testable and which no application ever sees. The
generic consumer that needed a write path was always the suite alone; PS-9's
falsifier is a *second* generic consumer, and if one lands the refusal is
reconsidered there rather than here.

---

## 7. Traceability

Every clause in this document, its maturity, the conformance rule that can
observe a violation of it, and the end-to-end cases it serves. This is the
current state of the obligation CF-35 through CF-38 impose.

**§7.1 and §7.2 are generated.** `cargo xtask spec-trace` — the checker CF-38
requires — parses every clause out of this file, resolves each named rule against
[`crates/happenstance-testkit/src/suite.rs`](../crates/happenstance-testkit/src/suite.rs)
and each named case against
[`spec/E2E-CASES.md`](E2E-CASES.md), and emits the two
sections between the markers below. Run without `--write` it recomputes them and
**fails** when the committed copy and the computed one disagree, which is the
difference between an obligation discharged and an obligation described. Editing
inside the markers by hand is therefore not merely discouraged, it is inert: the
edit survives exactly until the next `cargo xtask ci`, which reports it as a
difference and sends whoever reads the failure to `--write`, which overwrites it.

The checker earned its keep on its first run. The hand-rolled summary claimed 137
`[FROZEN]` clauses where §7.2's own rows already said 132 — it had miscounted `VT`
by one and `ES` by four — so §7.1 had contradicted both §7.2 and §1.3 for as long
as the three had coexisted, and nothing could notice. It also surfaced six rules
that §6 names one way and §3 another for the same behaviour — CF-13's
`positions_are_visible_in_assignment_order` against ES-10's
`nothing_below_an_observed_position_appears_later` was the clearest — which the
hand table had silently normalised to the §3 spelling, so twelve names read as
six. None of them existed yet, which is the only reason the reconciliation was
cheap: the rules still to be written are six, not twelve, and CF-31 makes a rename
after the first one ships a major release. All six are settled in the clauses now,
on one rule — **the name states the observable behaviour, not the mechanism.**
Five kept §3's spelling and one, ES-34's, took §6's, which is the measure of what
the hand table's silent normalisation was worth.

**§7.3 through §7.6 are authored, and stay that way.** A parser can tell you that
a clause names no rule. It cannot tell you whether that is *correct* — CF-34 names
none because a rule enforcing it would violate CF-33, and PS-37 names none because
the compile *is* the check. Those four sections are exactly the judgements no
parser computes, and they are where a gap is either defended or admitted. A
generated table that swallowed them would look more complete and say less.

Five reading conventions for §7.2, all of them consequences of the checker
claiming only what it verified.

- A rule name marked **†** was looked for in
  `crates/happenstance-testkit/src/suite.rs`, and — for a `wire::`-qualified name
  — in `crates/happenstance-core/tests/wire.rs` and
  `crates/happenstance-sync/tests/wire.rs`, and not found in any of them. It must
  be written.
- Where a clause points somewhere the checker cannot resolve — a unit or compile
  test living in the crate the clause constrains — the cell carries the clause's
  own words rather than a name list, and carries no `†`. A dagger there would
  assert an absence nothing checked. §2.7's wire tests left that category at
  ADR-0016 §15: they resolve, so an unwritten one is a `†` like any other.
- Every cell is cut to 79 characters, the `…` included, list or prose alike. The
  clause is authoritative and the table is an index into it.
- In the Cases column, `*all*` is a clause whose own text says *all
  contract-level cases* — §6 states that of itself, and fifty-six numbers in a
  cell would hide it rather than show it. `*(none directly; cites …)*` is a clause
  that serves no case but names the ones that motivate it, which is the
  distinction §7.5 turns on.
- The maturity column carries the marker alone. A `[PROVISIONAL]` clause's
  falsifier, a `[DEFERRED]` clause's experiment and the §6.5 axis a port clause is
  provisional against all live in the clause; a marker with an empty one is a
  build failure under CF-38.

**What §7.1's shape says** is what §1.6 says in words. `ES` is 76 % frozen because
it has fifty-nine rules, a reference implementation and six scenarios behind it —
fifty-nine being the distinct names its clauses' `Rule:` fields cite that resolve
to a `pub async fn` in `suite.rs`, which is a narrower count than §1.6's
eighty-nine, that being every rule in the file however it is claimed, and a wider
one than the rules already written, since a `Rule:` field also names rules that do
not exist yet and instruments that live outside `suite.rs`.
`PS` is 51 % frozen because it has no implementation at all, and its provisional
clauses are provisional against the same missing adapter rather than against
seventeen different ones — PS-2 is the single gate they all wait on. `SY` sits
between them because its *shape* does not wait on a transport but its
*measurements* do.

<!-- BEGIN GENERATED: spec-trace §7.1–§7.2 -->

### 7.1 Summary

| Section | Prefix | Clauses | `[FROZEN]` | `[PROVISIONAL]` | `[DEFERRED]` | `[NON-NORMATIVE]` |
|---|---|---|---|---|---|---|
| §2.1–§2.6 value types | `VT` | 34 | 24 | 9 | 0 | 1 |
| §2.7 wire format | `WF` | 12 | 10 | 1 | 1 | 0 |
| §3 `EventStore` | `ES` | 42 | 32 | 9 | 1 | 0 |
| §4 `ProjectionStore` | `PS` | 38 | 17 | 15 | 3 | 3 |
| §5 `SyncPeer` | `SY` | 35 | 21 | 9 | 5 | 0 |
| §6 conformance | `CF` | 40 | 33 | 4 | 2 | 1 |
| **Total** | | **201** | **137** | **47** | **12** | **5** |

### 7.2 The table

#### `VT` — value types (§2.1–§2.6)

| Clause | Maturity | Conformance rule — † = does not exist yet | Cases |
|---|---|---|---|
| VT-1 | FROZEN | `append_preserves_event_payload`, `append_preserves_event_type_and_tags_byte_f… | E2E-33, E2E-34, E2E-43 |
| VT-2 | FROZEN | `appending_equal_events_yields_two_events` | E2E-33, E2E-36 |
| VT-3 | FROZEN | `append_preserves_event_payload` | E2E-34, E2E-42 |
| VT-4 | FROZEN | `append_stamps_identity_and_time` | E2E-34, E2E-41, E2E-43 |
| VT-5 | FROZEN | `event_ids_are_unique_within_a_store`, `append_stamps_a_local_event_id`, `inge… | E2E-33, E2E-34, E2E-36, E2E-41, E2E-42 |
| VT-6 | PROVISIONAL | `reopened_store_does_not_reissue_an_event_id`, `restored_peer_does_not_reissue… | E2E-34, E2E-42 |
| VT-7 | FROZEN | `event_id_is_not_matchable_by_query`, `contains_event_id_reports_membership` | E2E-32, E2E-34, E2E-36 |
| VT-8 | FROZEN | `event_ids_are_unique_within_a_store` | E2E-33, E2E-36 |
| VT-9 | PROVISIONAL | `append_stamps_a_recorded_time`, `recorded_time_survives_a_reopen`, `convergen… | E2E-41, E2E-43 |
| VT-10 | PROVISIONAL | §5's `happenstance-sync-testkit` suite — `IngestStore` is the trait every `SY-… | E2E-33, E2E-35, E2E-36, E2E-39, E2E-42 |
| VT-11 | FROZEN | `positions_are_unique`, `positions_are_strictly_monotonic`, `positions_are_uni… | E2E-10, E2E-46 |
| VT-12 | NON-NORMATIVE | *(none — see clause)* | E2E-01, E2E-02, E2E-08 |
| VT-13 | FROZEN | unit test `position_next_signals_overflow`; `read_from_is_inclusive`, `conditi… | E2E-10, E2E-16 |
| VT-14 | PROVISIONAL | unit tests `rejects_invalid_event_types`, `rejects_invalid_tags`, and `validat… | E2E-40 |
| VT-15 | FROZEN | `tags_differing_only_by_unicode_normalisation_are_distinct`, `append_preserves… | E2E-40, E2E-49 |
| VT-16 | FROZEN | `query_item_tags_are_and`, `query_item_tags_match_supersets`, `query_item_reje… | E2E-32, E2E-40 |
| VT-17 | FROZEN | `tags_may_repeat_a_key` | E2E-57 |
| VT-18 | FROZEN | compile tests `event_new_accepts_a_held_event_type` and `command_handler_compo… | E2E-51 |
| VT-19 | FROZEN | `wire::decode_rejects_a_non_canonical_tag_set`, `wire::decode_accepts_an_over_… | E2E-40, E2E-42 |
| VT-20 | FROZEN | unit tests `rejects_invalid_event_types` and `rejects_invalid_tags`; `store_ac… | E2E-40 |
| VT-21 | PROVISIONAL | `store_accepts_the_guaranteed_minimum_payload`, `append_reports_exceeded_store… | E2E-42 |
| VT-22 | PROVISIONAL | `store_accepts_the_guaranteed_minimum_tag_count` | E2E-40 |
| VT-23 | PROVISIONAL | `store_evaluates_a_query_at_the_guaranteed_minimum_item_count` | E2E-36, E2E-40 |
| VT-24 | PROVISIONAL | `store_accepts_the_guaranteed_minimum_batch_size` | E2E-35, E2E-36, E2E-39 |
| VT-25 | FROZEN | `append_reports_exceeded_store_limits` | E2E-35, E2E-42 |
| VT-26 | FROZEN | compile test `query_items_is_not_constructible_downstream`; `condition_without… | E2E-40 |
| VT-27 | FROZEN | compile-level; enforced by the wire tests, which would otherwise have a positi… | E2E-04, E2E-05, E2E-37, E2E-38 |
| VT-28 | FROZEN | `read_limit_zero_yields_nothing`, `read_limit_truncates` | E2E-11, E2E-13 |
| VT-29 | FROZEN | `read_to_is_inclusive`, `read_from_and_to_bound_a_closed_window`, `read_to_und… | E2E-11 |
| VT-30 | PROVISIONAL | `condition_guards_carry_independent_boundaries`, `condition_with_one_guard_beh… | E2E-04, E2E-05, E2E-06 |
| VT-31 | FROZEN | `query_items_are_or`, `query_item_order_does_not_change_the_result_set`, `quer… | E2E-32 |
| VT-32 | FROZEN | unit tests `from_static_and_new_agree`, `from_static_rejects_a_bidirectional_c… | E2E-40, E2E-51 |
| VT-33 | FROZEN | unit tests `a_borrowed_and_an_owned_tag_are_one_value`, `a_map_keyed_by_event_… | E2E-40, E2E-51 |
| VT-34 | FROZEN | `read_from_is_inclusive` | *(none — see clause)* |

#### `WF` — wire format (§2.7)

| Clause | Maturity | Conformance rule — † = does not exist yet | Cases |
|---|---|---|---|
| WF-1 | DEFERRED | `wire::query_all_is_unambiguous`, `wire::empty_object_is_not_a_condition` | E2E-33, E2E-42 |
| WF-2 | FROZEN | `wire::round_trips_in_postcard` | E2E-33, E2E-35, E2E-42 |
| WF-3 | FROZEN | `wire::query_all_is_unambiguous`, `wire::option_query_round_trips` | E2E-40 |
| WF-4 | FROZEN | `wire::empty_object_is_not_a_condition`, `wire::condition_after_is_visible_to_… | E2E-35, E2E-37, E2E-40 |
| WF-5 | FROZEN | `wire::sequenced_event_round_trips` | E2E-33, E2E-34, E2E-41, E2E-43 |
| WF-6 | FROZEN | `wire::store_id_encodes_as_hex_in_json`, `wire::store_id_encodes_as_bytes_in_p… | E2E-34, E2E-42 |
| WF-7 | FROZEN | `wire::round_trips_in_json`, `wire::round_trips_in_postcard`, `wire::round_tri… | E2E-33, E2E-42 |
| WF-8 | FROZEN | `wire::rejects_an_unknown_format_version`, `wire::version_is_readable_before_t… | E2E-35, E2E-42 |
| WF-9 | FROZEN | `wire::decode_accepts_an_over_capacity_value`, `append_reports_exceeded_store_… | E2E-42 |
| WF-10 | FROZEN | `wire::decode_rejects_a_non_canonical_tag_set`, `wire::decode_rejects_an_uncon… | E2E-40 |
| WF-11 | PROVISIONAL | `wire::payload_is_base64_in_json`, `wire::payload_is_raw_in_postcard` | E2E-33 |
| WF-12 | FROZEN | `read_options_is_not_serialisable`, a **const-evaluation assertion in `crates/… | E2E-58 |

#### `ES` — the `EventStore` port (§3)

| Clause | Maturity | Conformance rule — † = does not exist yet | Cases |
|---|---|---|---|
| ES-1 | FROZEN | the suite binds the bare flavour and nothing else. Since the fixture contract,… | E2E-52, E2E-53, E2E-54 |
| ES-2 | FROZEN | two unit tests in `happenstance-core`, and it takes both. `send_flavour_stream… | E2E-52 |
| ES-3 | FROZEN | `provided_method_future_is_send_in_generic_code` † | E2E-13, E2E-53 |
| ES-4 | FROZEN | `provided_method_future_is_send_in_generic_code` † | E2E-13 |
| ES-5 | FROZEN | `error_bound_is_identical_on_both_flavours` † | E2E-53 |
| ES-6 | FROZEN | `store_error_crosses_a_join_handle` † | E2E-53, E2E-52 |
| ES-7 | PROVISIONAL | not a new rule but a new *invocation* of the existing suite — the `Rc`-backed … | E2E-52, E2E-09, E2E-54 |
| ES-8 | FROZEN | `read_defaults_to_ascending_order`, `read_from_is_inclusive`, `read_backwards_… | E2E-10, E2E-12 |
| ES-9 | FROZEN | `query_matching_nothing_yields_empty`, `read_from_a_gap_position`, `read_from_… | E2E-10 |
| ES-10 | FROZEN | `nothing_below_an_observed_position_appears_later`, `positions_are_unique`, `p… | E2E-01, E2E-02 |
| ES-11 | PROVISIONAL | `read_result_is_stable_under_concurrent_append` | E2E-02, E2E-01 |
| ES-12 | PROVISIONAL | `query_items_share_one_snapshot` | E2E-03, E2E-05 |
| ES-13 | FROZEN | `read_result_is_stable_under_concurrent_append` | E2E-02, E2E-03 |
| ES-14 | FROZEN | `read_limit_truncates`, `read_backwards_from_with_limit`, `limit_applies_acros… | E2E-12, E2E-13 |
| ES-15 | FROZEN | `duplicate_items_do_not_duplicate_events`, `query_item_order_does_not_change_t… | E2E-32 |
| ES-16 | FROZEN | `read_to_is_inclusive`, `read_from_and_to_bound_a_closed_window`, `read_to_und… | E2E-11 |
| ES-17 | PROVISIONAL | `append_preserves_event_payload` | E2E-36, E2E-39 |
| ES-18 | FROZEN | `append_is_atomic`, `condition_rejection_leaves_store_unchanged`, `append_is_a… | E2E-39, E2E-48, E2E-07 |
| ES-19 | FROZEN | `append_returns_last_written_position`, `batch_positions_follow_slice_order`, … | E2E-13, E2E-23 |
| ES-20 | FROZEN | `append_rejects_empty_batch`, `empty_batch_is_refused_before_the_condition_is_… | E2E-06 |
| ES-21 | FROZEN | `batch_is_not_evaluated_against_its_own_condition` | E2E-06 |
| ES-22 | FROZEN | `dropped_append_future_leaves_no_partial_batch` | E2E-07 |
| ES-23 | FROZEN | *(none — see clause)* | E2E-07 |
| ES-24 | FROZEN | `reissued_conditional_batch_lands_once`, `condition_rejection_is_reported_as_c… | E2E-07, E2E-33, E2E-36 |
| ES-25 | FROZEN | `condition_without_after_rejects_any_match`, `condition_without_after_allows_n… | E2E-08, E2E-55, E2E-39 |
| ES-26 | FROZEN | `condition_after_ignores_events_at_the_boundary`, `condition_after_rejects_eve… | E2E-56, E2E-10 |
| ES-27 | FROZEN | `condition_matches_on_tags`, `condition_with_an_unheld_tag_does_not_reject`, `… | E2E-55, E2E-03 |
| ES-28 | FROZEN | `condition_against_an_empty_store_admits_the_append`, `condition_after_beyond_… | E2E-56, E2E-08 |
| ES-29 | FROZEN | `wire_condition_with_after_is_refused` † | E2E-37, E2E-38, E2E-56 |
| ES-30 | FROZEN | `head_of_an_empty_store_is_none`, `head_is_the_highest_visible_position`, `hea… | E2E-13, E2E-02, E2E-25 |
| ES-31 | FROZEN | `checkpoint_lag_is_not_a_position_difference` † | E2E-13, E2E-23, E2E-25 |
| ES-32 | PROVISIONAL | *(none — see clause)* | E2E-32, E2E-28 |
| ES-33 | FROZEN | `two_handles_observe_each_others_appends`, `acknowledged_writes_survive_a_reop… | E2E-08, E2E-46 |
| ES-34 | FROZEN | `two_handles_observe_each_others_appends` | E2E-08 |
| ES-35 | PROVISIONAL | `acknowledged_writes_survive_a_reopen` | E2E-46 |
| ES-36 | FROZEN | `interleaved_appends_on_one_handle_elect_one_winner`, `a_live_read_stream_does… | E2E-09 |
| ES-37 | FROZEN | *(none — see clause)* | E2E-48, E2E-49, E2E-46 |
| ES-38 | FROZEN | `positions_are_not_reused_after_removal` † | E2E-46, E2E-10 |
| ES-39 | DEFERRED | `a_store_reports_the_history_it_does_not_hold` † | E2E-46, E2E-47, E2E-56 |
| ES-40 | PROVISIONAL | `condition_over_removed_history_does_not_reject` † | E2E-47, E2E-56, E2E-44 |
| ES-41 | PROVISIONAL | `contains_event_id_reports_membership` | E2E-32, E2E-34, E2E-36 |
| ES-42 | PROVISIONAL | compile-level — the erasure wrapper of ADR-0011's E11 compiles and round-trips… | *(none — see clause)* |

#### `PS` — the `ProjectionStore` port (§4)

| Clause | Maturity | Conformance rule — † = does not exist yet | Cases |
|---|---|---|---|
| PS-1 | FROZEN | `commit_is_atomic_with_the_read_model`, `commit_advances_the_checkpoint`, `fai… | E2E-17, E2E-21, E2E-23 |
| PS-2 | FROZEN | `commit_is_atomic_with_the_read_model` | E2E-17, E2E-24 |
| PS-3 | PROVISIONAL | `cargo hack --feature-powerset` in `cargo xtask ci`, which already runs; the e… | *(none directly; cites E2E-15, E2E-25)* |
| PS-4 | PROVISIONAL | `commit_is_atomic_with_the_read_model` | E2E-24 |
| PS-5 | PROVISIONAL | `MemoryProjectionStore` and one real adapter compiling without the `where Self… | E2E-19, E2E-24 |
| PS-6 | PROVISIONAL | the signature; no runtime rule. Enforced by the compiler on every implementer. | E2E-24 |
| PS-7 | FROZEN | `dropped_batch_leaves_store_usable` | E2E-24 |
| PS-8 | FROZEN | `rollback_leaves_both_unchanged` | E2E-24, E2E-28 |
| PS-9 | PROVISIONAL | *(none — see clause)* | E2E-20, E2E-29 |
| PS-10 | FROZEN | a compile test on the `Projection` trait — a doctest annotated `compile_fail,E… | E2E-20, E2E-29 |
| PS-11 | PROVISIONAL | `commit_is_atomic_with_the_read_model` | E2E-20, E2E-21, E2E-22, E2E-17 |
| PS-12 | PROVISIONAL | `batch_reads_reflect_pending_writes` | E2E-21, E2E-22 |
| PS-13 | FROZEN | `rebuild_is_chunk_size_invariant` | E2E-21, E2E-22 |
| PS-14 | FROZEN | `rebuild_is_chunk_size_invariant` | E2E-22 |
| PS-15 | PROVISIONAL | `commit_rejects_a_foreign_batch` | E2E-19 |
| PS-16 | PROVISIONAL | `reset_clears_rows_and_checkpoint_together` | E2E-15, E2E-17 |
| PS-17 | FROZEN | `reset_is_scoped_to_one_projection` | E2E-18 |
| PS-18 | DEFERRED | `refused_reset_changes_nothing` | E2E-18 |
| PS-19 | FROZEN | `reset_is_not_commit_at_first`, `fresh_projection_has_no_checkpoint` | E2E-15, E2E-16 |
| PS-20 | FROZEN | `reset_is_not_commit_at_first` | E2E-16, E2E-23 |
| PS-21 | FROZEN | `commit_accepts_a_position_the_batch_did_not_write` | E2E-23 |
| PS-22 | PROVISIONAL | `commit_rejects_a_regressing_position` | E2E-23, E2E-25 |
| PS-23 | PROVISIONAL | `distinct_projections_advance_independently` | E2E-28, E2E-32 |
| PS-24 | PROVISIONAL | `rebuilding_is_distinguishable_from_live` | E2E-25 |
| PS-25 | PROVISIONAL | `changed_query_starts_a_new_checkpoint` † | E2E-50 |
| PS-26 | FROZEN | `failure_policy_is_per_projection` † | E2E-27 |
| PS-27 | DEFERRED | `skip_and_record_is_atomic` † | E2E-26, E2E-27 |
| PS-28 | FROZEN | `pump_reports_the_failing_position` † | E2E-26 |
| PS-29 | FROZEN | `one_poisoned_projection_does_not_stall_the_others` † | E2E-28 |
| PS-30 | DEFERRED | `panicking_apply_rolls_back` † | E2E-28 |
| PS-31 | FROZEN | *(none — see clause)* | E2E-31 |
| PS-32 | NON-NORMATIVE | *(none — see clause)* | E2E-20 |
| PS-33 | NON-NORMATIVE | *(none — see clause)* | *(none directly; cites E2E-26, E2E-28)* |
| PS-34 | PROVISIONAL | a doctest on `ProjectionStore` implementing the port for a toy store, which ca… | E2E-24 |
| PS-35 | NON-NORMATIVE | *(none — see clause)* | E2E-30, E2E-52, E2E-53 |
| PS-36 | FROZEN | *(none — see clause)* | E2E-30 |
| PS-37 | FROZEN | *(none — see clause)* | E2E-52, E2E-53 |
| PS-38 | PROVISIONAL | `commit_advances_the_checkpoint`, `fresh_projection_has_no_checkpoint` | E2E-15, E2E-17, E2E-23 |

#### `SY` — the `SyncPeer` port (§5)

| Clause | Maturity | Conformance rule — † = does not exist yet | Cases |
|---|---|---|---|
| SY-1 | FROZEN | `ingest_never_rejects` † | E2E-33, E2E-39, E2E-42, E2E-45, E2E-47 |
| SY-2 | FROZEN | `compensation_is_atomic_with_the_losing_event` † | E2E-39 |
| SY-3 | FROZEN | `compensation_is_idempotent_under_redelivery` † | E2E-33, E2E-39 |
| SY-4 | FROZEN | `transitive_convergence_over_a_partial_mesh` † | E2E-42 |
| SY-5 | FROZEN | `ingested_events_land_above_the_local_head` † | E2E-42 |
| SY-6 | FROZEN | `wire_condition_with_after_is_refused` † | E2E-37, E2E-38, E2E-56, E2E-14 |
| SY-7 | PROVISIONAL | `only_the_adjudicator_compensates` † | E2E-39, E2E-42, E2E-45 |
| SY-8 | FROZEN | the whole of `happenstance-sync-testkit` — every rule in it takes exactly one … | E2E-33, E2E-35, E2E-36, E2E-42 |
| SY-9 | FROZEN | `one_adapter_serves_both_roles` † | E2E-42, E2E-45 |
| SY-10 | PROVISIONAL | `directional_merge_rules_compose` † | E2E-42, E2E-44, E2E-45 |
| SY-11 | FROZEN | `redelivery_of_an_accepted_group_is_a_no_op` † | E2E-33, E2E-34, E2E-36 |
| SY-12 | FROZEN | `dedupe_reaches_identity_without_decoding` † | E2E-33, E2E-34, E2E-36 |
| SY-13 | FROZEN | `identity_survives_a_tag_reordering_peer` † | E2E-33, E2E-34 |
| SY-14 | DEFERRED | `bulk_ingest_is_idempotent_in_bounded_round_trips` † | E2E-36, E2E-35 |
| SY-15 | FROZEN | `peer_conformance!` is invoked against a fixture peer whose transport asserts … | E2E-35, E2E-36 |
| SY-16 | FROZEN | `resume_survives_a_dropped_peer_handle` † | E2E-35, E2E-36, E2E-44 |
| SY-17 | FROZEN | the existing CLAUDE.md rule 4, checked by `happenstance-sync-testkit` compilin… | E2E-52, E2E-30 |
| SY-18 | DEFERRED | `peer_declares_its_own_limits` † | E2E-35 |
| SY-19 | FROZEN | `two_peers_disagree_on_total_order` † | E2E-41, E2E-42 |
| SY-20 | PROVISIONAL | `convergent_projection_is_interleaving_independent` † | E2E-41, E2E-22 |
| SY-21 | PROVISIONAL | `convergent_projection_cannot_observe_local_position` † | E2E-41 |
| SY-22 | PROVISIONAL | `non_convergent_projections_are_excluded_not_failed` † | E2E-41 |
| SY-23 | PROVISIONAL | `event_id_order_is_identical_on_every_peer` † | E2E-41, E2E-43 |
| SY-24 | FROZEN | `transitive_convergence_over_a_partial_mesh` † | E2E-42 |
| SY-25 | FROZEN | `forwarding_order_is_arrival_order` † | E2E-42 |
| SY-26 | FROZEN | `causality_is_expressible_without_a_contract_change` † | E2E-42, E2E-41 |
| SY-27 | DEFERRED | `scoped_replication_resume_is_sound` † | E2E-33, E2E-36, E2E-44, E2E-46 |
| SY-28 | DEFERRED | `round_trip_preserves_the_agreed_scope` † | E2E-44, E2E-46 |
| SY-29 | PROVISIONAL | `ingest_refuses_an_unbounded_peer_query` † | E2E-40 |
| SY-30 | PROVISIONAL | `push_envelope_preserves_group_boundaries` † | E2E-35, E2E-39 |
| SY-31 | PROVISIONAL | `watermark_advances_transactionally_with_a_read_model` † | E2E-33, E2E-44 |
| SY-32 | DEFERRED | `retention_gap_is_reported_not_silent` † | E2E-44, E2E-46, E2E-47 |
| SY-33 | FROZEN | `refusal_is_never_condition_derived` † | E2E-45, E2E-39, E2E-47 |
| SY-34 | FROZEN | `a_refused_spoke_retains_its_log` † | E2E-45 |
| SY-35 | FROZEN | `the_sync_suite_never_decodes` † | E2E-34, E2E-39, E2E-40, E2E-42 |

#### `CF` — conformance obligations (§6)

| Clause | Maturity | Conformance rule — † = does not exist yet | Cases |
|---|---|---|---|
| CF-1 | FROZEN | `mutation_coverage::every_rule_has_a_mutant` — a meta-test in `crates/happenst… | *all* |
| CF-2 | FROZEN | `mutation_coverage::mutant_registry_is_exhaustive` — a meta-test in `crates/ha… | *all* |
| CF-3 | FROZEN | `mutation_coverage::mutants_fail_exactly_their_declared_rules` — a meta-test i… | *all* |
| CF-4 | FROZEN | `mutation_coverage::every_mutant_states_its_provenance` — a meta-test in `crat… | E2E-01, E2E-08, E2E-32, E2E-55 |
| CF-5 | FROZEN | `mutation_coverage::conformant_variants_pass_everything` — a meta-test in `cra… | E2E-10 |
| CF-6 | FROZEN | `mutation_coverage::conformant_variants_pass_everything` (CF-5's gapped varian… | E2E-10 |
| CF-7 | FROZEN | `condition_matches_on_tags` | E2E-55, E2E-56 |
| CF-8 | FROZEN | `condition_with_an_unheld_tag_does_not_reject` | E2E-55, E2E-56 |
| CF-9 | FROZEN | `duplicate_items_do_not_duplicate_events` | E2E-32 |
| CF-10 | FROZEN | `condition_against_an_empty_store_admits_the_append` | E2E-47 |
| CF-11 | FROZEN | `empty_batch_is_refused_before_the_condition_is_evaluated` | E2E-06 |
| CF-12 | FROZEN | `read_from_composes_with_multi_item_query`, `read_from_composes_with_limit` | E2E-10 |
| CF-13 | FROZEN | `nothing_below_an_observed_position_appears_later` | E2E-01, E2E-02 |
| CF-14 | DEFERRED | `acknowledged_writes_survive_a_reopen` | E2E-07 |
| CF-15 | FROZEN | `two_fixture_instances_observe_none_of_each_others_appends` (the suite rule; t… | E2E-08, E2E-09 |
| CF-16 | FROZEN | `two_handles_observe_each_others_appends` | E2E-08 |
| CF-17 | PROVISIONAL | `acknowledged_writes_survive_a_reopen` | E2E-07 |
| CF-18 | FROZEN | `mutation_coverage::capability_skips_are_reported` — a meta-test in `crates/ha… | E2E-07, E2E-08 |
| CF-19 | FROZEN | `two_handles_observe_each_others_appends` | E2E-08 |
| CF-20 | FROZEN | the wasm32 step of `cargo xtask ci` (`xtask/src/main.rs:192-283`), extended to… | E2E-52, E2E-30 |
| CF-21 | FROZEN | a doctest in `fixtures` constructing a strategy, which fails to compile if the… | E2E-32 |
| CF-22 | FROZEN | `registry::no_orphan_rules`, at the foot of `crates/happenstance-testkit/src/r… | E2E-52, E2E-30, E2E-09 |
| CF-23 | FROZEN | the wasm32 steps of `cargo xtask ci`, which compile a `wasm-bindgen-test` harn… | E2E-52, E2E-30 |
| CF-24 | FROZEN | `registry::no_orphan_rules`, at the foot of `crates/happenstance-testkit/src/r… | *all* |
| CF-25 | FROZEN | `cargo xtask spec-trace` (CF-38), which reads the portfolio table and the matu… | E2E-01, E2E-02, E2E-24, E2E-46, E2E-52 |
| CF-26 | FROZEN | the portfolio table's `Far end exists` column, checked by `cargo xtask spec-tr… | E2E-01, E2E-24 |
| CF-27 | DEFERRED | `suffix_store_is_distinguishable_from_a_young_store` †, `positions_are_not_reu… | E2E-46, E2E-47, E2E-56, E2E-33 |
| CF-28 | FROZEN | the existing suite, invoked against that `Rc<RefCell<Vec<_>>>`-backed store un… | E2E-52, E2E-09, E2E-30 |
| CF-29 | FROZEN | `mutation_coverage::every_rule_has_a_mutant` (CF-1) enforces the mutant half m… | *all* |
| CF-30 | NON-NORMATIVE | *(none — see clause)* | *(none — see clause)* |
| CF-31 | FROZEN | `cargo xtask spec-trace` (CF-38), which fails when a rule name referenced by a… | *all* |
| CF-32 | FROZEN | a `cargo xtask ci` manifest check, added at phase 3 stage 6. It reads the `[pa… | *(none — see clause)* |
| CF-33 | FROZEN | a `cargo xtask ci` lint step over `happenstance-testkit/src`, rejecting `std::… | *(none — see clause)* |
| CF-34 | PROVISIONAL | *(none — see clause)* | E2E-CASES.md:1671-1677 records this as one of the two things that are neither … |
| CF-35 | FROZEN | `cargo xtask spec-trace` (CF-38). | *all* |
| CF-36 | FROZEN | `cargo xtask spec-trace` (CF-38), cross-referencing each case's level marker (… | E2E-28, E2E-29, E2E-39, E2E-42 |
| CF-37 | FROZEN | `cargo xtask spec-trace` (CF-38). | *all* |
| CF-38 | FROZEN | `cargo xtask spec-trace` (new step in `xtask/src/main.rs`'s `REQUIRED` list, `… | *all* |
| CF-39 | PROVISIONAL | `arming_a_mid_batch_fault_makes_the_append_fail`, `append_is_atomic_under_a_mi… | E2E-07, E2E-39 |
| CF-40 | PROVISIONAL | `append_reports_exceeded_store_limits` | E2E-35, E2E-42 |

<!-- END GENERATED -->

### 7.3 Clauses with no conformance rule

Eight normative clauses name no rule, plus four withdrawn to prose. Under CF-35 a
clause naming no rule is either wrong or belongs in prose, and must say which.
Each of the eight says which, and the verdicts are not uniform. Three of the
withdrawals — PS-32, PS-33 and PS-35 — were on the normative side of that count
until the typed layer's phase exit, and this section named them as defects in
this document rather than facts about the design from the day it was written.

| Clause | Why no rule | Verdict |
|---|---|---|
| **ES-23** — cancellation outcome is unspecified | The normative content *is* the word "unspecified". A rule asserting either outcome would convert a `MAY` into a `MUST` | **Correct as a clause.** The adapter-visible half is ES-22, which is checkable |
| **ES-32** — no tail or subscription seam at 0.1 | The absence of a method is not observable by a rule | **Correct as a clause.** Its checkable substitutes are ES-30 and ES-15 |
| **ES-37** — `EventStore` is closed over insertion | Same: the absence of a delete method | **Correct as a clause.** Its checkable consequences are ES-38 and ES-40 |
| **PS-9** — `Batch` carries no universal write vocabulary | Nothing checks the absence of a supertrait bound | **Correct as a clause.** The obligation it creates, PS-11, is checkable |
| **PS-31** — outward-writing projections are out of scope | A documented exclusion is not adapter-checkable | **Correct as a clause**, because silence here is what produces the wrong implementation |
| **PS-32** — ADR-0007's Context must be corrected | It is an instruction to edit a document | **Moved to prose at the typed layer's phase exit,** ID retained. The work item it was waiting for exists: the correction is staged with PS-33's verdict in `.kb/_intake/0032-adr-0031-the-runner-collapses-upward.md`, and the clause body records where it went |
| **PS-33** — ADR-0007's falsifier must be evaluated at a named phase exit | A phase gate, not an adapter obligation | **Moved to prose at the typed layer's phase exit,** ID retained — and only because the phase *did* evaluate it. The falsifier fired: `happenstance-core` publishes two free functions and neither is a checkpoint pump, so the pump collapses upward and ADR-0007 is superseded. An unevaluated falsifier is indistinguishable from none, which is why this row could not be written until now |
| **PS-35** — the derivation decision must cover both ports in one ADR | The artefact is the ADR | **Moved to prose,** ID retained. It constrained the next pass's paperwork, not an adapter, and both ADRs it demanded — ADR-0008 and ADR-0009 — exist. What it forbids survives in the clause body as prose |
| **PS-36** — the `Send` flavour transitively requires `Batch: Send` | The clause says **"none that gates"**, which is not the same as none: the `compile_fail` doctest it first named cannot be pinned, because the diagnostic carries no error code and rustdoc 1.97.1 silently ignores an unmatched `compile_fail,E0308` annotation, so the annotation asserts nothing and the bare form passes on any compile error including a typo | **Correct as a clause, and the only entry here whose absence is a *finding*.** Every other row names a rule that would be wrong to write; this one names a rule that would be right to write and cannot be, on stable, without a `trybuild`-style stderr snapshot — a dependency decision phase 6 owns (ADR-0008:234-240). The doctest is kept as documentation and must not be read as a gate |
| **PS-37** — the `Self: Sync` rule applies to `ProjectionStore` | The obligation is on the contract crate; the artefact is a generic helper that compiles | **Correct as a clause,** and the compile *is* the check — a `cargo xtask ci` build failure is as binding as a rule. It is listed here because CF-38's checker will not find a rule name and must not treat that as a dangling reference |
| **CF-34** — performance is measured by a separate harness, which is not the bar | The clause's content is that no conformance rule may be the check | **Correct as a clause,** and self-referentially so: a rule enforcing it would violate CF-33 |
| **CF-30** — the testkit pin recommendation | Withdrawn: no adapter behaviour violates it | Already `[NON-NORMATIVE]`; the ID is retained so citations resolve |

Three clauses — PS-32, PS-33 and PS-35 — were this list's real content, and all
three have now moved. All three are in §4, all three were instructions to the
pass that lands this specification rather than constraints on an implementation,
and all three said here from this document's first assembly that they should move
into that pass's work list and out of the clause space **when it exists**. It
exists: the typed layer's phase is what wrote the runner PS-33's falsifier was
about, and the same pass took the count, staged the superseding ADR and moved all
three markers to `[NON-NORMATIVE]` with their IDs retained.

**The rows above are kept rather than deleted, for the reason §7.5 gives about
itself: a defect list that deletes its own entries cannot be audited.** Anyone
following a citation to PS-32, PS-33 or PS-35 — `RUNBOOK.md` carries three —
lands on a retained ID whose body says what happened to it. And the hazard this
group carried is worth naming as discharged rather than as absent: PS-33's
falsifier had already survived one document handover by being written in an ADR
that no phase read, and a falsifier that has already fired without changing
anything is a marker that has quietly become decoration. It fired, and something
changed.

Two further clauses name a rule that checks only part of them, and are recorded
here so the checker does not report them as clean. **VT-3** — the contract, a
store and a peer MUST NOT parse `data` or `metadata` — is checked mechanically
only in its positive half (`append_preserves_event_payload`); the prohibition is a
review obligation. **WF-1** — the wire format is private — has two tests that fail
if it is abandoned in practice, and its scope half is likewise a review
obligation. Neither is a defect; both are places where a reviewer is load-bearing
and the document says so rather than implying otherwise.

### 7.4 Conformance rules no clause names

**None.** Three rules were named by no clause's `Rule:` line when this section was
first written. All three were discussed in the surrounding prose of §3 and §6,
which is not the same thing — prose that discusses a rule does not put it under a
clause's protection, and CF-38 is specified to fail on exactly that gap. All
three were then disposed of by a `Retires:` line, and **all three dispositions
have since been reversed: every one of the rules is retained, and a clause now
claims each.** That is the more instructive outcome and is why this section
exists at all. What follows is the record of what was retired, on what reasoning,
and what refuted it.

**Three for three is the finding, not a coincidence.** Each disposition was
written by reading the rule and asking what it could reject; each was refuted by
building or strengthening the thing it said could not exist. The method that
produced the wrong answer three times is the same method §6.1 forbids for
adapters, applied to rules — so the rule this section leaves behind is:
**a `Retires:` line is a hypothesis about a wrong implementation, and it is
discharged the way CF-1 discharges any other, by naming the store and compiling
it.** Until then it stays a claim to re-examine.

The count is deliberately not stated as a fraction of the suite. §7.2 is generated
and the suite's rule count moves every phase, so a fraction here would be a second
census to keep in step with §1.3's, decaying the same way and for the same reason.
What is permanent is the disposition: which rule was retired, why it was weaker
than the clause it would have protected, and which rule replaces it.

**`query_all_matches_every_event` — retired at ES-15, and the retirement
reversed. The rule is retained and ES-15 claims it.** It appended three *untagged*
events and read them back through `Query::all()`. The retirement's reasoning was
that this is weaker than the clause it would be protecting, because with no
multi-tagged event in the store the fan-out a tag join without `DISTINCT`
produces cannot occur (CF-9). That much is true, and ES-15's
`duplicate_items_do_not_duplicate_events` is the strengthened successor for that
half. It is not a reason to delete the rule, because the successor does not carry
what the retired rule was good at, and there turned out to be *two* such halves
rather than one. An untagged event has no row to join to and an `INNER JOIN`
drops it from `Query::all()` entirely — that half went to
`untagged_events_match_query_all`. The other half nobody had noticed: the rule's
three events were rewritten to **descending** types, so a store answering
`ORDER BY type, position` — a covering index on `(type, position)`, which is what
anyone reaches for — fails it. `SortByEventTypeStore` is that adapter and
`InnerJoinTagStore` is the second; both are registered, and no other rule rejects
the first on the untagged `Query::all()` path. A retirement that silently narrows
coverage is the failure this section exists to catch, and this one was one edit
away from doing it twice.

**`racing_conditional_appends_elect_one_winner` — retired at ES-27, and the
retirement reversed. The rule is retained and ES-25 claims it.** It ran two
conditional appends sequentially, on one handle, and its single tagged condition
ran against a store where a type-only probe returns the identical verdict — one
untagged `CourseDefined`, one tagged `StudentSubscribed`. The disposition
concluded from that that it pinned neither of the two things its name claims. Not
the tags: ES-27's `condition_matches_on_tags` and its mirror
`condition_with_an_unheld_tag_does_not_reject` pin those as a pair — CF-7 and
CF-8 are the obligations to have them — because either alone leaves one direction
open. And not the race: CF-19's `two_handles_observe_each_others_appends` runs it
across two handles and ES-36's
`interleaved_appends_on_one_handle_elect_one_winner` runs it within one. ES-34
declined to name the rule for exactly this reason and said so before there was
anywhere to put the disposition — which is what a missing mechanism looks like
from the inside.

**Both halves of that are correct, and the conclusion drawn from them is not.**
"Neither of the two clauses whose names the rule evokes owns it" is not the same
proposition as "no clause owns it", and the rule's actual content is a third
clause's: ES-25's *if and only if*, seen as two decisions taken from one
snapshot. The second append's `after` sits below an event matching its own query,
so the rejection is compulsory and exactly one batch may land. Three registry
rows fail it — `WriteThenCheckStore`, `ViolationAsStoreErrorStore`,
`AfterIsAnOffsetStore` — and the first two are shapes ES-25's own `Rejects:`
paragraph names by hand, which is the evidence that the clause and the rule were
always about the same thing.

**`append_is_atomic` — retired at ES-18, and the retirement reversed. The rule is
retained and ES-18 claims it.** This one was found by asking CF-1's question of a
rule that already existed — name the wrong implementation it rejects — and the
answer given was: none. A three-event batch, refused, the store asserted
unchanged; but the condition is evaluated before anything is written, so the
batch never reaches the write path, a partial write was never possible, and the
rule asserts `condition_rejection_leaves_store_unchanged`'s property with a
longer batch and no more reach.

**That answer was wrong, and what refuted it was writing the store rather than
reading the rule.** `WriteThenCheckStore` — autocommit plus a separate probe,
the non-concurrent form of the probe-then-insert shape a reviewer measured
passing the suite — extends the log with all three events, *then* probes, then
returns `Err`. The batch reaches the write path; the rule catches the partial
write. `AfterDefaultsToFirstStore` and `InnerJoinTagStore` fail it too, and three
registry rows now name it. Retiring it would have removed the only rule in the
suite rejecting that shape's non-atomicity.

`append_is_atomic_under_a_mid_batch_fault` landed at stage 6 beside it, and the
two are complementary rather than successive: one rejects a store that writes
before it decides, the other a store that cannot roll back what it wrote. No
condition violation can manufacture a partial write, which is the part of the
original reasoning that survives.

The mechanism that let the mistake sit is CF-38's, and it is stated there as well
as here: **a disposed rule satisfies check 6 forever.** The rule was in
`suite.rs`, a `Retires:` line named it, and the checker was silent — which is
precisely the hole this section exists to close, seen from the inside. A
`Retires:` line is a claim to re-examine, not a filing.

All three are the same shape of error, seen twice at two levels. A specification
that only ever names the rules it wants written leaves the rules that already
exist unowned, and an unowned rule is one nobody is responsible for deleting when
it stops meaning anything — that is how the three came to be here. A rule
*disposed of* by a clause is owned by nobody either, and the disposition is the
only thing standing between it and deletion — that is how all three nearly went.

**The checker gap is closed.** `spec_trace.rs`'s check 6 accepts
`claimed || retired`, so it cannot tell a `Retires:` line that describes a
deleted rule from one that names a rule still in `suite.rs` and still rejecting
registered mutants. `cargo xtask lint-retired-rules` is the lint that closes it,
added at phase 3 stage 6 and mandatory in the gate.

**It is stricter than this section proposed, and the difference is the point.**
The lint scheduled here was *a rule named in any clause's `Retires:` that
`collect_rules` still finds in the suite is an error **unless the clause also
claims it***. The exemption was there to tolerate `retires_of`'s second trap —
that function reads the whole `Retires:` field, continuation lines included, so a
*successor* rule backticked in the reasoning registers as retired while being
legitimately claimed. Tolerating that shape is what leaves the trap armed: the
day someone drops the claim, the successor is silently disposed of forever, which
is this section's own failure one turn further round. So the lint carries no
exemption. A `Retires:` name still found in `suite.rs` fails whether or not a
clause claims it, with two different messages — *the document contradicts itself*
where it is claimed, *nothing else can notice this* where it is not — and the
rule it enforces is the simple one this section arrived at by other means: **a
`Retires:` line is discharged by deleting the rule, in the same change.** Until
the deletion lands, the line is a claim to re-examine.

The consequence for check 6 is worth stating because it looks like duplication
and is not. With the lint in place, a live rule must be *claimed*, and being
*retired* can no longer save one — so check 6's `retired` disjunct now only
decides which of the two steps reports the problem, and it is the lint's, whose
message names the remedy. It found nothing on its first run, exactly as this
section predicts: every disposition was reversed at stages 4 and 5 and no
`Retires:` line survives in the document. That is the one nil result in this
phase's lints that was known in advance, and it is why the lint was demonstrated
against a `Retires:` line added and removed rather than against the tree.

**A nil result demonstrated by hand is still a nil result, and stage 6's review
said so.** A lint whose input does not exist reports success identically whether
it works or has stopped parsing: rename the field to `Retired:`, or move it
inside a bold paragraph that `field_line`'s continuation loop breaks on, and it
prints the same green line for ever. Its two sibling lints both bail when what
they scan is empty and this one had no such guard. It now parses a fixture clause
of its own on every run — a constant in `xtask/src/spec_trace.rs`, held to the one
name it is known to dispose of — which converts "the field spelling still parses"
from an assumption into a failure. The guard earned its keep on the first run: the
probe as first written backticked a second identifier in its own reasoning, which
is precisely the trap the field's convention exists to avoid, and the check caught
it.

**Both this lint and CF-29's resolved rule names against `suite.rs` alone, which
is one file of the three rules live in.** Since stage 5 there are three: the
event-store family in `suite.rs`, the proptest family in `model.rs`, and the
threaded family in `concurrency.rs`. A `Retires:` line naming a live model or
concurrency rule passed, under a message asserting the thing the run had not
checked; a rule of either family could land with no changelog entry at all while
CF-29 reported every rule satisfied. Both now resolve over all three, and so
does check 6 — every rule is claimed by a clause — which was the last to widen
and the only one whose narrow scope came with an argument. The argument was that
only the event-store family's rules are claimed by clauses today. That is true,
and it is the case for widening rather than against it: they were unclaimed
*because* nothing required them to be. A check that looks only where ownership
already exists cannot tell a rule nobody has claimed from a rule somebody
decided to leave unclaimed, and telling those two apart is this section's whole
subject. The prediction made in the narrow scope's defence was correct and was
not a reason to keep it — widening did report every model and concurrency rule
no clause named, and six of them were named by none. Four were attribution
errors, where the clause already stated the proposition and already named the
wrong implementation and only the rule's name was absent: ES-18, ES-19, ES-25
and VT-11 claim them now. The other two are not errors. One states a proposition
no clause states; the other states several at once and belongs to no single
clause for that reason. Both are recorded in `UNCLAIMED_PENDING_ADR` in
`xtask/src/spec_trace.rs`, which prints every entry on every green run and fails
the day a clause claims one, so an unowned rule is now a standing report rather
than a silence. This section opens by answering its own question with **None.**;
that answer was written about the three suite-family rules below it, and is owed
a restatement now the question is asked of all three families.

**The changelog lint of CF-29 also matched rule names as bare substrings, and two
rules were passing on a collision.** `append_is_atomic` is a prefix of
`append_is_atomic_under_a_mid_batch_fault` and `positions_are_unique` is a prefix
of `positions_are_unique_under_concurrent_appends`; in both cases the longer
rule's entry was the only one in the file carrying either name, and it discharged
the shorter rule's obligation while inflating the per-rule prose share by
counting the same entry twice. Both now have entries of their own. The first is
not an incidental rule — this section records it above as the only thing in the
suite that rejects the write-then-check shape's non-atomicity.

**The clock lint's construct list is short on `wasm32`, and this is where that is
recorded rather than fixed.** CF-33 names four constructs — `std::time`,
`Instant`, `elapsed`, `sleep` — and the gate runs exactly those, because quietly
extending a `[FROZEN]` clause's specified check would leave the document and the
gate disagreeing about what the bar is, with the gate winning silently. But
`std::time::Instant::now()` *panics* on `wasm32-unknown-unknown`, and the gate
type-checks the testkit's tests for that target: the clock an author would reach
for there is `js_sys::Date::now()` or `chrono::Utc::now()`, and neither contains
any of the four needles. So the list forbids the spelling that cannot be used on
a target the crate advertises and permits the ones that can. (`web_time::Instant`
and `instant::Instant` are caught incidentally, by the type name; the absence of
`Duration` and `timeout` is correct, because a duration is not a clock read.)
Closing it means amending CF-33's own `Rule:` line to name the wasm-reachable
clocks in the same change that extends the array, which is an ADR-shaped edit to
a frozen clause and belongs to whoever writes the first `wasm32` conformance
rule that wants one.

### 7.5 Clauses that name no case

Five clauses name no E2E case at all, and one more names cases only in prose
after declaring "none directly" — PS-3. It was two until the typed layer's phase
exit; PS-33 was the other, and it has left the clause space. CF-35 requires every clause to name the cases it
serves, so this list is a defect list, not a note. Two further clauses were on it
and have been closed; they are kept in the table with their closures recorded,
because a defect list that deletes its own entries cannot be audited.

**Two of the five were missing from this table until phase 5's reconciliation,
and how they were found is the point.** §7.2 is generated and renders
`*(none — see clause)*` in the Cases column for every clause that names one;
§7.5 is authored. Nobody had compared the two, so VT-34 and ES-42 sat in the
generated table as caseless and in the authored table not at all — and the
headline count agreed with the rows it could see. A defect list assembled by hand
beside a machine-generated one that answers the same question is a defect list
that will drift, and the only reason this one was caught is that a reader
recounted it against §7.2 rather than against itself.

| Clause | What it names instead | Verdict |
|---|---|---|
| **VT-17** — `key:value` is convention, not enforced | **E2E-57**, since phase 4. It came from Wattline (`references/scenarios/README.md:628-635`) and nothing covered it when this list was written | **Closed at phase 4.** The defect was real and the case was writable exactly as described here — construct a `Tag` with no colon and one with two, assert both are accepted and that neither acquires structure — and E2E-57 is that case |
| **WF-12** — `ReadOptions` is not on the wire | **E2E-58**, since phase 5. Nothing when this list was written; it removes a surface | **Closed at phase 5.** Minor and trivial, as predicted: assert `ReadOptions` does not implement `Serialize`. Its instrument is a const-evaluation assertion, not a compile test (ADR-0016 §13) |
| **VT-34** — `ReadOptions` carries exactly one lower bound, inclusive | Nothing. Its `Rule:` is "compile-level, plus the existing `read_from_is_inclusive`", and the behaviour of `from` is E2E-covered through ES-16 rather than here | **Acceptable.** What this clause forbids is a *shape* — an `after` beside `from` — and the absence of a thing has no scenario. The case that would exist if it were violated is the one its `Rejects:` describes: two adapters inventing different verdicts for `from(5).after(7)`, which cannot be written against a contract that does not admit the call |
| **ES-42** — `read`'s return carries no `+ Unpin` | Nothing. Its `Rule:` is compile-level — ADR-0011's E11 erasure wrapper | **Acceptable, and the only one of the five that expires.** Its marker requires re-evaluation before phase 12, because adding a bound to an opaque return type after publish is breaking. A case naming it would be a case about what a *consumer* can write, not about what a store does, and E2E-CASES is a catalogue of store behaviour |
| **CF-32** — the testkit carries its own `version` key | Nothing; a packaging obligation with a machine check | **Acceptable.** CF-35 permits a clause whose check is a manifest check rather than a behavioural one, and there is no user-visible behaviour to write a case against |
| **CF-33** — no rule may read a clock | Nothing; it constrains the suite | **Acceptable**, same reason |
| **CF-34** — benchmarks are not conformance | `E2E-CASES.md:1671-1677`, which records the harness as one of the two things that are neither blocked nor cases | **Acceptable**, and the citation is the right one |
| **PS-3** — ship behind `unstable-projection` | "none directly", then E2E-15 through E2E-25 | **Acceptable.** It is a packaging decision that makes a range of cases safe to answer before publication |
| **PS-33** — evaluate ADR-0007's falsifier | "none directly", then E2E-26 through E2E-28 | **Closed at the typed layer's phase exit.** It was listed in §7.3 as belonging in a work list, and it has gone there: the clause is `[NON-NORMATIVE]` with its ID retained, so it no longer owes this table a case. The row stays because a defect list that deletes its own entries cannot be audited |

So: no genuine holes left, and five clauses where naming no case is the honest
answer and the clause says why. VT-17 closed at phase 4 with E2E-57 and WF-12
closed at phase 5 with E2E-58, which took the count down by two; VT-34 and ES-42
were found missing from this table in the same pass, which put it back. The
headline being five both before and after is a coincidence, and worth saying so
that nobody reads the unchanged number as evidence that nothing moved.

### 7.6 E2E cases no clause claims

**None. All 58 cases are claimed by at least one clause.**

That result is suspicious enough to state how it was reached, because "the defect
list is empty" is the shape of a list that was never computed.

The `Cases:` line of all 200 clauses was parsed, every `E2E-nn` reference
extracted, and the union compared against `E2E-01`…`E2E-58` as enumerated in
[`E2E-CASES.md`](E2E-CASES.md)'s index (`:39-46`) and confirmed
against its 58 `### E2E-nn` headings. The comparison was then run a second time
with a stricter rule — discarding any `Cases:` line beginning with the word
"none", so that the two clauses which say "none directly" and then cite a range in
prose contributed nothing. The orphan set was empty under both readings.

The distribution is uneven and the unevenness is informative rather than
alarming. Four cases are claimed by exactly one clause each: **E2E-14** (SY-6),
**E2E-31** (PS-31), **E2E-50** (PS-25) and **E2E-51** (VT-18). Single-claim cases
are the ones a later edit can orphan without anyone noticing, and three of the
four are the lifecycle cases whose surrounding decisions are still open. At the
other end, **E2E-42** — transitive convergence — is claimed by 26 clauses across
four sections, which is what a case that exercises a whole design looks like.

What this section does **not** claim is that every case is *satisfied*. A clause
claiming a case means the clause is reachable from it, not that anything runs:
most of the rules named in §7.2 do not exist, the sync testkit crate does not
exist, and the e2e crate `tests/e2e/` does not exist. The orphan list being empty
says the specification has no unreachable corners. It says nothing about coverage,
and CF-38's checker must not be written to imply otherwise.
