# happenstance — architectural specification

- **Date:** 2026-08-06
- **Commit:** `2a65d76`
- **Status:** current. This document supersedes `docs/evaluation/ARCHITECTURAL-EVALUATION.md` §3 and
  `docs/evaluation/revised-runway.md`, which are retained for their record only.

This is what is true about happenstance's design *now*. An ADR records why a decision was taken and
when; this document records the decision's current form, and where the two disagree the ADR is
history and this is current. Every normative statement is a numbered clause carrying a maturity
marker, the conformance rule that checks it, the end-to-end cases it serves, and — because a rule no
adapter can fail is decorative — the wrong implementation it forbids. A clause that names no wrong
implementation is marked non-normative and demoted to prose rather than left to look load-bearing.

The evidence base is `docs/evaluation/PRESSURE-TEST.md`, which adjudicated the prior evaluation by
compiling its claims, and `docs/scenarios/`, six deployments walked line by line against the contract
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

This is the normative architectural specification for `happenstance`. It states
what is true now about the three ports the workspace defines, the values that
cross them, and the conformance obligation that decides whether an adapter has
implemented one. Everything downstream — adapter crates, the testkit, the typed
layer, the worked examples — is measured against it.

It is not a design discussion. Where a question is open, the clause says so and
names the experiment that closes it; where a question is closed, the clause says
what is forbidden and what would prove it wrong. Nothing here is aspirational:
each clause either constrains an implementation that can fail it, or is marked
`[NON-NORMATIVE]` and demoted to prose.

**Its relationship to `docs/adr/`.** An ADR records *why* a decision was taken
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

The drift is not hypothetical. [ADR-0006](../adr/0006-bare-name-to-the-typed-layer.md)
decided that `happenstance-runtime` ceases to exist and the contract crate becomes
`happenstance-core`; `ls crates/` returns `happenstance-runtime` today and
`crates/happenstance-core/Cargo.toml:2` still names the contract crate `happenstance`.
`CLAUDE.md` still lists `happenstance-runtime` as a "named seam" and still carries
"whether `happenstance-runtime` is the right name" as an open question, which
`PRESSURE-TEST.md:389-393` records as an accepted-but-unexecuted rename already
producing wrong instructions to the two documents an agent loads first. An
accepted ADR is a decision, not a description.

Three of the seven ADRs are marked *"accepted — provisional"* in their own front
matter for exactly this reason —
[0001:5-12](../adr/0001-async-port-flavours.md),
[0003:5-12](../adr/0003-opaque-payloads.md) and
[0004:5-12](../adr/0004-edition-and-msrv.md) each say they were authored before
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

- `EventStore` ships stable at 0.1, so a `[FROZEN]` `ES` clause is semver-binding
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

As assembled, this document carries 193 clause IDs, of which 192 are normative: **132
`[FROZEN]`**, **46 `[PROVISIONAL]`**, **13 `[DEFERRED]`** and **two
`[NON-NORMATIVE]`** (CF-30, and VT-12 which is a retained pointer to ES-10). Section 7
breaks that out per clause.

**One standing qualification on every `[FROZEN]` port clause.** CF-25 forbids
declaring a port frozen while an axis of §6.5's instrument portfolio has no
passing implementation at its far end, unless the freeze names the axis and the
ADR accepting the risk. All seven axes have empty far ends today, so the
qualification is not hypothetical and it is not discharged by asserting it.

It is discharged three ways, and the split matters more than the total:

- **Position allocation is measured, not accepted.** It is the axis the pressure
  test and all six scenarios independently ranked first, and what is missing is
  not an adapter but a number: which of `xid8` + `pg_snapshot_xmin`,
  transaction-scoped advisory locks, or a serialised sequence table buys the
  invariant, and what each costs. That is a throwaway probe against a real
  Postgres, and the runbook's phase 2 owns it — before the phase-4 freeze, not
  after it at phase 10.
- **Five `ES` clauses carry the residual exposure and say so in their own
  markers** rather than in a preamble a reader skips: **ES-10** (position
  allocation), **ES-11** and **ES-12** (transport — a one-shot-HTTP adapter that
  self-paginates may be unable to meet either), **ES-35** (durability) and
  **ES-40** (completeness). Each is `[PROVISIONAL]` with its axis named and its
  falsifier the far-end adapter that has not been built.
- **The remaining six axes are accepted in the ADR that lands this document.**
  Accepted, explicitly, with the axis named — which is what CF-25 asks for and is
  a different act from not having noticed.

The distinction the rest of §3 relies on: a `[FROZEN]` marker binds the design and
makes changing it an ADR rather than an edit. It does not claim the far end has
voted. Where the far end could plausibly vote against, the clause is provisional
instead — and there are five of those, named above, not forty.

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
  [`crates/happenstance-testkit/src/suite.rs`](../../crates/happenstance-testkit/src/suite.rs);
  a rule this document specifies but which does not exist yet is marked as new.
  `WF-n` clauses name tests in `crates/happenstance/tests/wire.rs` instead,
  because the wire format has exactly one implementation and a conformance suite
  exists to police a plurality — the reasoning is in §2's preamble.
- **`Cases:`** — the numbered cases in
  [`docs/scenarios/E2E-CASES.md`](../scenarios/E2E-CASES.md) the clause serves.
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

**Everything depends on `happenstance`; `happenstance` depends on nothing in this
workspace.** No adapter may depend on another adapter. The workspace is
`members = ["crates/*", "examples/*", "xtask"]` (`Cargo.toml:3`), and the rule is
what keeps that glob from becoming a graph.

One deliberate exception, and it is a port relationship rather than a dependency
between adapters. `happenstance-sync` is itself a port crate: peer adapters depend
on it the way store adapters depend on `happenstance`
(`crates/happenstance-sync/Cargo.toml:14-19`), and its conformance suite will live
in `happenstance-sync-testkit`. It stays out of the contract crate so that
publishing `happenstance` never waits on replication.

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
no way to opt out. The alternative — define the trait in `happenstance` and let
the sync crate implement it — loses because it inverts the dependency: the
contract crate would then be the thing that changes when replication changes.

**Cargo features are additive across the whole build graph.** If two crates in one
build enable different feature sets of a shared dependency, Cargo unifies them and
every consumer gets the union. That is why ADR-0003's guarantee — `serde` is never
in `happenstance`'s default features — is only enforceable while nothing in the
graph turns it on for someone who did not ask. `happenstance-sync` turns it on
non-optionally and deliberately (`crates/happenstance-sync/Cargo.toml:15-17`:
*"`serde` is non-optional here: replication is the reason `happenstance`'s serde
feature exists"*), which is sound because sync is a leaf that an application opts
into. An adapter-to-adapter dependency would make that opt-in transitive and
silent, and the wasm target would acquire `serde` because some unrelated crate
wanted replication.

### 1.6 The three ports

| Port | Where it lives | What exists today | Maturity | What would freeze it |
|---|---|---|---|---|
| **`EventStore`** | `crates/happenstance-core/src/store.rs:117-145` | Two methods; 27 conformance rules; one reference implementation (`memory.rs:147`) | **Frozen at 0.1**, conditional on CF-25 risk acceptance | Already frozen — §3. The exposure is the seven empty far ends of §6.5's portfolio |
| **`ProjectionStore`** | `crates/happenstance-core/src/projection.rs` | The trait and nothing else. `grep -rn "ProjectionStore for"` matches nothing in the workspace (`PRESSURE-TEST.md:246-248`) | **Provisional**, behind an off-by-default `unstable-projection` feature (PS-3) | PS-2: a hostile store that commits the checkpoint and drops the read-model write must *fail* the suite, and two adapters at opposite ends of the batch-shape axis must pass it |
| **`SyncPeer`** | `crates/happenstance-sync/src/lib.rs` | No trait at all. A module doc comment and a one-variant error enum (`:103-112`), whose own prose ends *"None of this is settled"* (`:98-99`) | **Shape specified, experiments deferred** — 5 of 35 clauses `[DEFERRED]`, 9 `[PROVISIONAL]` | The phase that builds the port against two real peers; §5's deferred clauses name it individually |

The asymmetry is the point. `EventStore` is frozen because it has evidence:
twenty-seven rules, a reference implementation, six deployment scenarios walked
line by line, and one pressure test whose contested claims were settled by
compiling them. `ProjectionStore` is not frozen because it has none: no adapter
has ever been written against it, and the suite that would freeze it cannot
currently observe half of the invariant the port exists to defend
(`PRESSURE-TEST.md:220-234`). `SyncPeer` is specified before it is built because
the alternative is worse — `docs/RUNBOOK.md:80` records that deferring the sync
port *"leaks `EventId` and a tail seam back into `EventStore`"*, and a leak into a
frozen port is not a deferral, it is a decision taken by omission in the crate
that can least afford it.

### 1.7 What is settled, and what will move

**Build on these.** They are `[FROZEN]`, they have named rejects, and the
scenarios have been walked against them.

- **The query algebra.** Types-OR within an item, tags-AND with superset
  matching, items-OR across the query. Thirty-odd boundaries across six domains,
  including several spanning what would classically be three or four aggregates,
  expressed without strain and without one request for a filter the language
  cannot express (`docs/scenarios/README.md:1900-1903`). VT-26 through VT-31, and
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

- **Whether `Error` gains `Send + Sync`** (ES-6). Deferred against a compiled
  `!Send` error shape; `PRESSURE-TEST.md:139-153` shows both existing
  "confirmations" are free by construction, because `MemoryStoreError` is
  uninhabited and `SqliteEventStoreError` is a unit variant.
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
seven axes and seven empty far ends. Every port in this document has been checked
against a population of implementations that agree with it: `MemoryEventStore`, a
planned rusqlite adapter and a planned Durable Object all serialise their writers
and assign positions under a lock they hold until commit. That is one storage
shape wearing several hats. The clauses that will turn out to be wrong are the
clauses that assume a property all of them happen to share, and §6.5 names which
ones those are.

### 1.8 The evidence base

Four documents and one tree. Cited throughout by `file:line`, read at commit
`2a65d76`.

**[`docs/evaluation/PRESSURE-TEST.md`](../evaluation/PRESSURE-TEST.md)** is
authoritative. It adjudicated the prior evaluation and roadmap by compiling the
discriminating cases rather than by preferring one auditor, and where it
contradicts either of them, it wins. Its §5 ranks the open issues by blast radius
on this specification; its §7 lists what only an experiment can settle, and those
are the `[DEFERRED]` markers' experiments.

**[`docs/scenarios/E2E-CASES.md`](../scenarios/E2E-CASES.md)** is the case
catalogue: 56 numbered cases at three levels — contract, integration, scenario
(`:17-28`) — each stating what it falsifies and what wrong implementation it
rejects. Its *"What cannot be written yet"* section (`:1490-1585`) lists the eleven
decisions this specification exists to settle. Every normative clause below is
reachable from at least one case.

**[`docs/scenarios/README.md`](../scenarios/README.md)** is the catalogue of six
deployments, each designed to sit at the far end of a different axis and each
walked line by line against the contract on disk. Its closing section, *"What the
six agree on"* (`:1873-1912`), is the convergence signal — nine holes found
independently by dissimilar deployments, and one thing the design got right.

**`docs/adr/0001` through `0007`** are the decisions on record, with the tense
rule of §1.1 applied.

**Superseded, kept for the record only.**
[`ARCHITECTURAL-EVALUATION.md`](../evaluation/ARCHITECTURAL-EVALUATION.md) §3 and
[`revised-runway.md`](../evaluation/revised-runway.md) are superseded by this
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
the eleven decisions [`docs/scenarios/E2E-CASES.md`](../scenarios/E2E-CASES.md)
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
implementation — the `serde` impls in `happenstance` — and by CLAUDE.md's own
standard a rule no adapter can fail is decorative. So `WF-n` clauses name tests
in `crates/happenstance/tests/wire.rs` rather than rules in
`happenstance-testkit`. They are no less binding; they are simply in the only
place that can fail them.

**Nothing is published.** `happenstance` has never been released, so every
signature change below costs nothing but the edit. Where a clause reverses a
decision recorded in the code, it says so.

---

### 2.1 `Event`

#### VT-1 — An `Event` carries what its writer knows and nothing its store knows

An `Event` MUST carry exactly four things: an `EventType`, an opaque `data`
payload, a `Tags` set, and optional opaque `metadata`. An `Event` MUST NOT carry
an identity, a position, a time, or the `AppendCondition` under which it was
appended.

`[FROZEN]`
`Rule:` `append_preserves_event_payload` (`crates/happenstance-testkit/src/suite.rs:417`);
new `append_preserves_event_type_and_tags_byte_for_byte`
`Cases:` E2E-33, E2E-34, E2E-43
`Rejects:` a store that stamps a writer-supplied identity into the log, which
makes history forgeable by any caller and gives two peers no way to agree on
which of two identically-identified events is the real one.

The four fields are what `crates/happenstance-core/src/event.rs:183-188` already has,
so this clause changes nothing about `Event` and exists to close the four
questions repeatedly asked of it.

*Identity, position and time are store-assigned* and live on `SequencedEvent`
(VT-4). The split is not stylistic. An `Event` is a value a command handler
constructs before it knows whether the append will be accepted; a store-assigned
fact is by definition unavailable at that moment. Putting any of the three on
`Event` would force every constructor to supply a value it cannot have, and
would make `Event::new` fallible for a second, unrelated reason.

*The append condition is refused as a field.* Kestrel Rotor's `conflict-queue`
projection wants it (`docs/scenarios/README.md:1722-1732`): its whole job is to
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
`Rule:` new `appending_equal_events_yields_two_events`
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
`Rule:` `append_preserves_event_payload` checks the mechanical half — that the
payload survives byte-for-byte. The "MUST NOT parse" half is not mechanically
checkable and is a review obligation; it is stated as a clause rather than prose
because the *positive* requirement — put it in the tags — is what an adapter
author needs, and it is checkable at review by grepping an adapter for a
payload decode.
`Cases:` E2E-34, E2E-42
`Rejects:` the identity scheme `crates/happenstance-sync/src/lib.rs:73-75`
currently proposes — "a UUIDv7 or a content hash in the event's **metadata**".
`Tags` and `EventType` are the only things `QueryItem::matches` looks at
(`crates/happenstance-core/src/query.rs:113-116`), so an identity in `metadata` is
structurally invisible to the port. A peer that must deserialise opaque bytes to
decide whether it has already seen an event has broken ADR-0003 at the exact
point ADR-0003 claims to win.

This is the standing constraint the scenario catalogue arrived at from six
directions and states in its closing line
(`docs/scenarios/README.md:1910-1912`). It has a positive proof as well as a
negative one: Kestrel Cold Chain's hub adjudicates a contested claim and authors
a compensation without parsing a payload, because `outcome`, `unit`, `epoch` and
`supersedes` are all tags (`docs/scenarios/README.md:144-155`).

The cost is real and should be stated where a vocabulary is being designed
rather than discovered: a value promoted to a tag becomes indexable, queryable,
and — per E2E-49 — **unerasable**, because crypto-shredding covers `data` and
cannot cover `tags`. "Put it in the tags" is not free advice.

**Prose, not a clause:** `Event::into_parts` (`event.rs:243-246`) documents
itself as "avoiding a clone in adapter write paths". That is false as written,
because `EventStore::append` takes `&[Event]` (`store.rs:141-145`) and no trait
implementation can reach an owned `Event` at all. The method still earns its
keep on the ingest path, where a peer owns an `Event` it decoded from the wire
and wants to move its parts into an adapter's row struct. The doc comment must
say that instead. ES-17 owns the correction and states it identically; whether
`append` should take an owned batch is §3's question, not this one's.

---

### 2.2 `SequencedEvent`, identity, and time

#### VT-4 — `SequencedEvent` carries three store-assigned facts

A `SequencedEvent` MUST carry a `SequencePosition`, an `EventId`, a `RecordedAt`
and the `Event`. All four fields MUST be public and the struct MUST remain
`#[non_exhaustive]`.

`[FROZEN]`
`Rule:` new `append_stamps_identity_and_time`
`Cases:` E2E-34, E2E-41, E2E-43
`Rejects:` an adapter that returns `SequencedEvent` values whose identity or
time it made up at read time rather than persisting at write time. Such an
adapter passes every rule that reads back what it just wrote inside one process
and fails the first reopen, because the values are not stable.

Today `SequencedEvent` is `{ position, event }` (`event.rs:277-282`) and
`SequencedEvent::new` is a two-argument `const fn` (`:286`) that every adapter
and the testkit call. Adding two fields is therefore one signature change across
the workspace, which is exactly why E2E-CASES insists that identity and time be
decided in one pass rather than two (`E2E-CASES.md:1510-1512`). This clause takes
that pass.

`#[non_exhaustive]` is already on the struct (`event.rs:276`) and stays: a
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
`Rule:` new `event_ids_are_unique_within_a_store`, new
`append_stamps_a_local_event_id`; the preservation half by
`happenstance-sync-testkit`'s new `ingest_preserves_origin_identity`
`Cases:` E2E-33, E2E-34, E2E-36, E2E-41, E2E-42
`Rejects:` a store that re-mints an `EventId` on ingest. That is the
implementation the port invites today — `append` has no slot for a foreign
identity (`store.rs:141-145`) so an ingesting peer's store assigns its own — and
it makes re-delivery of a dropped batch produce a second copy of every event.
E2E-33 shows the failure is worse than a duplicate: the hub's own copy now
matches the origin condition, the re-delivered group takes the violated branch,
and the hub **supersedes the event it just accepted**.

Why a pair and not a UUID. Both give uniqueness; only the pair gives a
deterministic, peer-independent total order for free. Sorting merged events by
`(StoreId, SequencePosition)` is a total order every peer computes identically
from data it already holds, which is what E2E-41's convergence check needs and
what Kestrel Rotor's `unit-ledger` is missing
(`docs/scenarios/README.md:1710-1713`). A UUIDv7 would give an approximate time
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

#### VT-6 — `StoreId` names a store incarnation, not a device and not a peer

A `StoreId` MUST be a 128-bit value minted when a store's persistent state is
created. It MUST NOT be derived from a hostname, a device identifier, a peer
name or any other value that survives a restore. A store MUST NOT issue an
`EventId` whose `(StoreId, SequencePosition)` pair it has previously issued for a
different event.

`[PROVISIONAL — falsified by a deployment in which an adapter cannot detect that
its state was restored or cloned, and cannot be given an out-of-band re-mint
operation either; the sync testkit's restore-and-diverge scenario is the
instrument]`
`Rule:` new `store_id_is_stable_across_reopen`; the non-reissue half by
`happenstance-sync-testkit`'s new `restored_peer_does_not_reissue_identities`
`Cases:` E2E-34, E2E-42
`Rejects:` an adapter that keys `StoreId` on the device it runs on. A tablet
restored from Friday's backup then reissues Monday's positions to different
events, every peer's dedup treats the new events as already-seen, and **real
facts are silently dropped** — the one failure mode in the replication design
with no error path and no observable symptom.

The distinction between a device and an incarnation is what E2E-CASES flags as
unsettled (`E2E-CASES.md:1505-1509`) and it has a consequence the ledger row does
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
origins whose interleaving is no longer recoverable. Adapters SHOULD prefer the
first and MUST document which they chose.

#### VT-7 — `EventId` is outside the query language

`Query` and `QueryItem` MUST NOT gain an identity dimension. `EventId` MUST NOT
be represented as a `Tag`. A peer MUST be able to ask whether a store already
holds a given `EventId` through a dedicated port operation, without constructing
a `Query` and without parsing any payload.

`[FROZEN]`
`Rule:` new `event_id_is_not_matchable_by_query`; the membership operation by
§3's new `contains_event_id_reports_membership`
`Cases:` E2E-32, E2E-34, E2E-36
`Rejects:` the tag-materialised identity Kestrel Rotor proposes —
`oid:sov-aurora-3f9c#38103` on every event — which is presented as free and is
not. It puts a maximally high-cardinality entry in the one column adapters are
told to index; it enters every `contains_all` merge-scan
(`crates/happenstance-core/src/tag.rs:231-245`) on every query in the system; it makes
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
`Rule:` new `event_ids_are_unique_within_a_store`
`Cases:` E2E-33, E2E-36
`Rejects:` an ingest implementation that establishes idempotence by reading
first and appending second. Two round trips instead of one, which is fatal in
Kestrel Rotor's 34-minute satellite window, and — worse — an unclosed race when a
depot ingests from two peers concurrently, because nothing between the read and
the append excludes the other ingest.

This clause is what makes E2E-36 answerable in a bounded number of round trips
without adding per-event conditions to `append`. The catalogue lists three ways
to get idempotent bulk ingest and rejects all three
(`E2E-CASES.md:935-948`); the fourth is a uniqueness constraint the store already
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
`Rule:` new `append_stamps_a_recorded_time`, new
`recorded_time_survives_a_reopen`; the "not an ordering key" half by §5's
`convergent_projection_is_interleaving_independent` (SY-20), which fails any fold
that reads either the position or the time
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
entry point in the contract crate puts `happenstance`'s publish schedule behind
`happenstance-sync`'s design, which is precisely what
`crates/happenstance-sync/src/lib.rs:23-26` says the crate exists to avoid.

*A trait in `happenstance-sync`.* Chosen. It is the posture that crate already
adopts for itself — "this crate defines the third [port]"
(`crates/happenstance-sync/src/lib.rs:7-11`) — and the coherence question resolves cleanly: the adapter
crate depends on both `happenstance` and `happenstance-sync`, so
`impl IngestStore for SqliteEventStore` has a local type in the trait reference
and the orphan rule permits it. What is *not* possible, and is worth saying
before someone tries, is a blanket `impl<S: EventStore> IngestStore for S`: there
is no way to express "insert this row with this identity" in terms of `read` and
`append`, and even if there were, a blanket implementation would make it
impossible for an adapter to supply a better one, because coherence forbids the
overlap.

The feature gate matters for a mundane reason: `happenstance-sync` depends on
`happenstance/serde` non-optionally (`crates/happenstance-sync/Cargo.toml:15-17`),
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
`Rule:` `positions_are_unique` (`suite.rs:336`),
`positions_are_strictly_monotonic` (`suite.rs:355`)
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

The prohibition on arithmetic is already documented at `event.rs:92-97` and is
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
`Rule:` unit test `position_next_signals_overflow`; `read_from_is_inclusive`
(`suite.rs:262`), `condition_after_ignores_events_at_the_boundary`
(`suite.rs:477`); new `read_from_a_gap_position` (ES-9's name for it)
`Cases:` E2E-10, E2E-16
`Rejects:` for the overflow half, the current implementation —
`Self::new(self.0.get().saturating_add(1))` at `event.rs:138-144` returns
`Some(u64::MAX)` where its own documentation promises `None`, because
`saturating_add` cannot signal overflow and the method whose sole purpose is
signalling overflow therefore never does. `NonZeroU64::checked_add` is `const`
and returns the right thing. For the resume half, an adapter that implements
`from` as an equality seek or a `rowid` offset rather than a range predicate —
plausible on a store whose positions came from a counter, and fatal after the
first purge.

The blast radius of the overflow bug is nil in practice; it is fixed because a
contract crate returning a wrong answer through a signature that has a way to say
"I cannot" is the wrong trade, and because library code may not `unwrap` and will
therefore have to branch on the `Option` anyway.

The asymmetry is worth stating rather than leaving to be re-derived. Norvant
confirms it is sound (`docs/scenarios/README.md:1005-1013`): because `from` is an
inclusive lower bound rather than a seek, and gaps are permitted,
`checkpoint.next()` resumes correctly even when the very next position does not
exist. Every adapter author will otherwise reconstruct that argument from
scratch, and `projection.rs:84-93`'s instruction to "advance past" the checkpoint
by hand is what sends them looking.

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
`Rule:` unit tests `rejects_invalid_event_types` (`event.rs:407-418`) and
`rejects_invalid_tags` (`tag.rs:337-352`), extended; new
`store_round_trips_a_maximum_length_type_and_tag`
`Cases:` E2E-40
`Rejects:` a validator that bans all of `Cf`, which is the natural
over-correction and which breaks Persian, Hindi and every emoji sequence by
rejecting U+200C and U+200D. And a validator that bans only ASCII C0, which is
what four doc comments in the crate currently claim happens.

The behaviour today is already right and the documentation is already wrong.
`char::is_control` (`event.rs:46`, `tag.rs:56`) is Unicode `Cc`, which includes
the C1 range U+0080–U+009F; `event.rs:37`, `tag.rs:47`, `error.rs:29` and
`error.rs:53` all say "ASCII control characters". Four sites, one two-word fix,
and the fix is to the prose.

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

#### VT-15 — Equality is byte equality; the contract normalises nothing

`Tag` and `EventType` equality MUST be byte equality over the UTF-8 encoding. The
contract, and every adapter, MUST NOT apply Unicode normalisation, case folding,
trimming or any other transformation to a tag or an event type. Whitespace is
significant everywhere, including leading and trailing whitespace.

`[FROZEN]`
`Rule:` new `tags_differing_only_by_unicode_normalisation_are_distinct`, new
`append_preserves_event_type_and_tags_byte_for_byte`
`Cases:` E2E-40, E2E-49
`Rejects:` an adapter that NFC-normalises tags on write — reasonable-looking,
and it rewrites a caller's data so that the tag read back is not the tag written,
which breaks byte-faithful replication and makes the store's index disagree with
any external system holding the original string. Also rejects a Postgres adapter
using a case-insensitive or `ICU` collation on the tag column, which silently
merges two consistency boundaries.

The position is the same one `tag.rs` already takes about trimming
(`docs/scenarios/README.md:1744-1747`) and it must be stated because everyone
assumes the opposite. The DCB specification treats a tag as an opaque string; a
contract that normalises is deciding what a caller's identifier means.

The cost is concrete and belongs next to the rule. Kestrel Rotor replicated a
`SerialisedUnitConsumed` carrying `turbine:HW2-A14 ` with a trailing space;
`Tag::new` accepts it, `Tags` sorts it adjacent to the real tag, `contains_all`
is a strict merge-scan on equality (`tag.rs:231-245`) and does not match it, and
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
`Rule:` `query_item_tags_are_and` (`suite.rs:96`),
`query_item_tags_match_supersets` (`suite.rs:123`),
`query_item_rejects_partial_tag_overlap` (`suite.rs:148`)
`Cases:` E2E-32, E2E-40
`Rejects:` an adapter that implements a tagged query with `IN`-style semantics —
any tag matching rather than all tags matching. It passes the OR rules and the
superset rule and fails only the partial-overlap rule, which is exactly why that
rule exists. Also rejects an adapter that persists tags in insertion order and
compares them positionally, which makes two equal `Tags` values unequal after a
round trip.

Canonicalisation at construction is what buys the linear merge-scan, set-valued
`PartialEq` and `Hash`, and a stable serialisation for an index — `tag.rs:137-145`
states all three. The infallible `FromIterator` impl (`tag.rs:258-266`) is
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
`Rule:` new `tags_may_repeat_a_key`
`Cases:` none. This clause comes from Wattline D3
(`docs/scenarios/README.md:628-635`) and no E2E case covers it. It stays a clause
rather than prose because it is mechanically checkable and because the wrong
implementation is one an adapter author will reach for.
`Rejects:` an adapter that indexes tags as a key→value map. `Tags::from_pairs([
("tenant", "a"), ("tenant", "b")])` succeeds and yields a two-element set —
deduplication is on the whole `key:value` string (`tag.rs:258-265`) — so a
map-shaped index silently drops one of them, and the event then matches only one
of the two tenants' queries. On a 4,200-tenant shared log that is a cross-tenant
correctness failure produced by an indexing choice.

Enforcing key-uniqueness would put a convention in the contract and exceed the
DCB specification, which treats tags as opaque strings. The absence of
`Tags::get` is the same decision seen from the read side: an accessor that
returns one value where two may exist would make the ambiguity invisible rather
than resolving it. The documentation on `Tags` MUST say the convention is
unenforced and MUST name the repeated-key case, because a reader of
`Tag::key_value` reasonably assumes otherwise.

#### VT-18 — Constructors accept values the caller already holds, and their errors compose

`Event::new` MUST accept an already-constructed `EventType`. The three
validation error types MUST compose: `InvalidQuery` MUST implement
`From<InvalidTag>` in addition to its existing `From<InvalidEventType>`.

`[FROZEN]`
`Rule:` compile tests `event_new_accepts_a_held_event_type` and
`command_handler_composes_validation_errors` in `crates/happenstance/tests/`
`Cases:` E2E-51
`Rejects:` the current signature. `Event::new` takes
`impl TryInto<EventType, Error = InvalidEventType>` (`event.rs:197-200`), and the
blanket `impl<T, U: From<T>> TryFrom<T> for U` gives `Error = Infallible`, so the
equality constraint excludes the one conversion that cannot fail. Passing a
`const COURSE_DEFINED: EventType` produces `error[E0271]` pointing at
`event.rs:198`. This blocks the typed layer from interning one `EventType` per
`DomainEvent`, which is the whole point of interning it, and it must land before
any code is written against `Event::new`.

The crate already knows the fix and applied it two files over.
`QueryItem::new` writes `T: TryInto<EventType>, InvalidQuery: From<T::Error>`
(`query.rs:57-61`) with `impl From<Infallible> for InvalidQuery`
(`error.rs:77-84`), and its own doc comment says that is what lets it accept
both. A `From` bound admits both conversions where an equality constraint admits
one, because `Infallible` converts into anything by matching on an uninhabited
value — `match never {}` — which is why the impl at `error.rs:81-83` has no arms.
`Event::new` needs the same shape plus
`impl From<core::convert::Infallible> for InvalidEventType`.

The error composition is the second half of the same ergonomic problem. A command
handler in a library crate cannot use `anyhow`, calls `Tags::from_pairs`
(`InvalidTag`), `QueryItem::new` and `Query::from_items` (`InvalidQuery`), and
propagates with `?` — and today needs a bespoke union enum before it writes a
line of domain logic. Every code snippet in all six scenarios has this shape, and
every one of them compiles only inside a `Box<dyn core::error::Error>` doctest,
which is how the crate's own doctests escape it (`query.rs:141`, `append.rs:31`).
One `From` impl and a new `InvalidQuery::Tag` variant closes it. Collapsing the
three enums into one `InvalidInput` was the alternative and it loses: the three
are returned by three different constructors and matching on which one failed is
worth keeping, `#[non_exhaustive]` enums are cheap, and the conversion direction
is unambiguous because a query can contain tags and a tag cannot contain a query.

---

### 2.5 Bounds

The scenarios found that the contract bounds the two values no storage engine
struggles with — `MAX_EVENT_TYPE_LEN` and `MAX_TAG_LEN`, both 255
(`event.rs:14`, `tag.rs:14`) — and leaves unbounded the three that cost money:
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
`wire::decode_accepts_an_over_capacity_value`, new
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
`Rule:` `rejects_invalid_event_types`, `rejects_invalid_tags`, new
`store_round_trips_a_maximum_length_type_and_tag`
`Cases:` E2E-40
`Rejects:` an adapter that truncates an over-length tag to fit its column. It
produces an event whose tags do not match the query that should have selected it,
with no error anywhere.

These two are validity invariants rather than capacity limits because they are
the two values an adapter declares a column width against
(`tag.rs:10-14` says exactly that). A value exceeding them cannot be stored by
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
`Rule:` new `store_accepts_the_guaranteed_minimum_payload`, new
`append_reports_exceeded_store_limits`
`Cases:` E2E-42; and Turnstile's peer D
(`docs/scenarios/README.md:1517-1524`), which has no case number
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
`Rule:` new `store_accepts_the_guaranteed_minimum_tag_count`
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
`Rule:` new `store_evaluates_a_query_at_the_guaranteed_minimum_item_count`
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
`Rule:` new `store_accepts_the_guaranteed_minimum_batch_size`
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

#### VT-25 — A capacity refusal is distinguishable from a store failure

`AppendError` MUST gain a variant `ExceedsStoreLimit { limit: StoreLimit, len:
usize }`, where `StoreLimit` is a `#[non_exhaustive]` enum naming which limit was
exceeded. A store MUST NOT report a capacity refusal through
`AppendError::Store`.

`[FROZEN]`
`Rule:` new `append_reports_exceeded_store_limits`
`Cases:` E2E-35, E2E-42
`Rejects:` reporting an over-limit append as an adapter-specific error, which is
what every adapter will do today because there is no other variant. The caller
then cannot tell "this event will never be accepted here, park it and tell a
human" from "the disk is full, retry" — and the sync runner, which must make
exactly that distinction to avoid the E2E-42 disappearance, has nothing to
switch on. `AppendError`'s three variants (`error.rs:148-166`) contain no
"refused, park this" and PRESSURE-TEST §5.4 names the gap.

`AppendError` is already `#[non_exhaustive]` (`error.rs:149`) and its
documentation already tells callers a wildcard arm is required
(`error.rs:146-147`), so the variant is additive.

---

### 2.6 `Query`, `QueryItem` and `ReadOptions`

#### VT-26 — A `Query` with zero items is unrepresentable from outside the crate

`Query::Items` MUST NOT be constructible outside `happenstance`. The variant MUST
carry `#[non_exhaustive]`.

`[FROZEN]`
`Rule:` compile test `query_items_is_not_constructible_downstream`;
`condition_without_after_rejects_any_match` (`suite.rs:438`)
`Cases:` E2E-40
`Rejects:` `Query::Items(Vec::new().into_boxed_slice())` from any downstream
crate — the exact state `Query::from_items` rejects with `InvalidQuery::NoItems`
(`query.rs:166-168`). `Query::matches` then returns `false` for everything, so an
`AppendCondition` built from it can never be violated: a conditional append that
is silently unconditional. The crate's headline design claim is false at the
boundary that matters, and it is one line of downstream code away.

Variant-level `#[non_exhaustive]` rather than enum-level is the right instrument
here and the distinction is worth explaining. On the *enum*, it would force every
downstream `match` to carry a wildcard arm, which is a real cost for a two-variant
enum whose variants are the whole of its meaning. On the *variant*, construction
outside the crate is forbidden while matching still works — `Query::Items(items,
..)` — and the trailing `..` is the compiler telling the reader that the payload
is the crate's business. `Query::items()` (`query.rs:182-187`) remains the
ordinary read path and needs no change.

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

`ReadOptions` carries one `from` for the entire read (`query.rs:226-234`) and
`read` applies one `ReadOptions` to the whole `Query` (`store.rs:117-121`). So
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
`Rule:` new `read_limit_zero_yields_nothing`; `read_limit_truncates`
(`suite.rs:292`)
`Cases:` E2E-11, E2E-13
`Rejects:` the current implementation. `self.limit = NonZeroUsize::new(limit)`
(`query.rs:260-266`) turns zero into `None`, documented as "A `limit` of zero is
ignored, since requesting nothing is never what the caller meant". The premise is
true for a literal and false for a computed value: a paging loop writing
`.limit(budget - fetched)` that reaches parity does not read zero events, it
reads **the entire log, unbounded, silently**.

This is a deliberate divergence from the DCB reference implementation, which
treats `limit: 0` as unlimited through JavaScript falsiness, and the ADR that
lands it must say so rather than presenting it as a correction of a bug in
precedent. It matches SQL `LIMIT 0` and every paging API in existence. The cost
is eight bytes on a `Copy` struct passed by value. `limit(NonZeroUsize)` was the
type-safe alternative and it loses: it forces every caller with a computed budget
to handle the zero case at the call site, which is precisely the caller who has
just computed zero legitimately.

#### VT-29 — A read can be given an inclusive upper bound

`ReadOptions` MUST gain a `to: Option<SequencePosition>`, an inclusive upper
bound in position order. Under `backwards`, `from` remains the starting (higher)
bound and `to` the stopping (lower) bound.

`[FROZEN]`
`Rule:` new `read_to_is_inclusive`, new `read_from_and_to_bound_a_closed_window`,
new `read_to_under_backwards_bounds_the_older_end`
`Cases:` E2E-11
`Rejects:` the workaround, which is `from(H).backwards()`, buffer everything,
reverse in memory. That genuinely is a pinned snapshot and it works at Kestrel
Motor's 211-event erasure; it is fatal at the 53-million-event regulatory
backfill, because the whole window must be materialised before the first event is
yielded. A backfill worker given the closed window [1, *H*] while a tail worker
owns (*H*, ∞) is the shape every backfill-beside-tail deployment wants, and
`limit` cannot stand in for it because `event.rs:94-96` forbids treating position
arithmetic as a count.

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
`Rule:` new `condition_guards_carry_independent_boundaries`, new
`condition_with_one_guard_behaves_as_today`; the existing
`condition_after_*` family (`suite.rs:477-527`) becomes the single-guard case
`Cases:` E2E-04, E2E-05, E2E-06
`Rejects:` the application-side fix E2E-05 names — four reads, four last-seen
positions, and a condition on `min(p₁…p₄)`. That is sound and it re-admits every
event above the minimum for **all four** items, so the quiet fragment's stale
boundary governs the busy one and the deployment reads the resulting rejection
rate as physics.

Today `AppendCondition` is `{ fail_if_events_match: Query, after:
Option<SequencePosition> }` (`append.rs:50-61`) with public fields on a
`#[non_exhaustive]` struct. Replacing `fail_if_events_match` with `guards` is a
breaking field change and costs nothing, because nothing is published. The
`Guard` type MUST itself be `#[non_exhaustive]` with public fields, for the same
reason `AppendCondition` is: E2E-37's ingest rule must *read* `after` on a
peer-supplied condition without being able to construct one literally, and
readable-but-not-literal-constructible is precisely what enables that refusal.

`is_violated_by` (`append.rs:95-107`) becomes a fold over the guards; the
single-guard path is byte-for-byte the current behaviour, which is why every
existing condition rule survives unchanged. Adapters pushing this into SQL must
generate `(item AND position > p) OR …` with explicit parentheses — the textbook
precedence bug that the scenarios' S7 already flags for the single-boundary case
becomes N times more likely here, and the conformance rule above is what catches
it.

What this clause does **not** do is change what a batch's own events are checked
against. A batch's events MUST NOT be evaluated against its own condition
(E2E-06); `MemoryEventStore` already behaves that way by accident of ordering
(`memory.rs:197-221`) and §3 owns turning that accident into a rule.

#### VT-31 — The query algebra is fixed and adapters may not change the match set

For any queries *a* and *b* built from items, the match set of a query whose
items are the concatenation of *a*'s and *b*'s items MUST equal the union of
their match sets. Any query unioned with `Query::All` MUST match everything. An
adapter MAY reorder or deduplicate items, and MUST NOT do so in a way that
changes the match set.

`[FROZEN]`
`Rule:` `query_items_are_or` (`suite.rs:200`); new
`query_item_order_does_not_change_the_result_set`, new
`query_union_is_item_concatenation`, new
`duplicate_items_do_not_duplicate_events` — the same four ES-15 names, because
the algebra is one property stated from the value side here and from the port
side there
`Cases:` E2E-32
`Rejects:` an adapter that deduplicates or reorders a query's items as an
optimisation in a way that is not match-set preserving — natural to reach for,
because `QueryItem::new` already sorts and deduplicates *types*
(`query.rs:62-67`), so extending the idea to items looks like the same move. Such
an adapter passes all 27 existing rules and breaks the fan-out projection runner,
which reads one union query and re-filters each projection's stream locally with
`Query::matches` (`query.rs:194-204`) and is correct only if the algebra holds.

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
No phase currently needs it.]`
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
clause. The reference emits a query as a bare sequence of items where
happenstance emits an `{items: […]}`-shaped value, and it emits `[]` for
match-all where happenstance cannot parse `[]` at all. Both are recorded so the
future bridge starts from a list rather than from a discovery.

#### WF-2 — Every field of every wire struct is always present

No wire struct may use `skip_serializing_if`, and no attribute may make the
number of serialised fields depend on a value. Every field MUST be written, in
declaration order, on every serialisation.

`[FROZEN]`
`Rule:` `wire::round_trips_in_postcard` over every envelope shape, including the
all-defaults shape
`Cases:` E2E-33, E2E-35, E2E-42
`Rejects:` the five `skip_serializing_if` attributes at `event.rs:342`,
`event.rs:344`, `query.rs:281`, `query.rs:283` and `append.rs:121`. The attribute
shortens the field count passed to `serialize_struct`; a non-self-describing
format feeds the deserializer exactly `FIELDS.len()` values positionally, so a
skipped field desynchronises the stream. Serialisation *succeeds* and produces
plausible-looking bytes. `Event::new("A", data)` with no tags — what every
doctest builds — is a failing input, as are `QueryItem::of_types([…])`,
`QueryItem::tagged(…)` and `AppendCondition::new(q)`.

The cost is bytes in JSON — a bare `Event` goes from 35 to 61 — and nothing at
all in postcard, where an absent `Option` is one byte and an empty sequence is
one byte. That is the correct direction to trade: the format that is used for
diagnostics pays, the format that is used in bulk does not.

#### WF-3 — `Query::All` has an unambiguous encoding

`Query` MUST be encoded as an externally tagged enum with two variants, `All`
(unit) and `Items` (sequence). `Query::All` MUST NOT be encoded as `null`, as an
absent field, or as an empty sequence. `Option<Query>` MUST round-trip, with
`Some(Query::All)` distinguishable from `None`.

`[FROZEN]`
`Rule:` `wire::query_all_is_unambiguous`, `wire::option_query_round_trips`
`Cases:` E2E-40
`Rejects:` the current implementation. `Serialize` maps `All` to
`serialize_none` and `Items` to `serialize_some` (`query.rs:304-321`), so
`Query::All` becomes JSON `null`, `Some(Query::All)` and `None` are
indistinguishable, and serde's `missing_field` succeeds for `Option`-shaped
types — which means a *missing* field decodes to the broadest condition in the
protocol. A peer that emits `null` by accident gets match-everything. Inside an
`AppendCondition` that fails closed, which is a spurious rejection; inside an
ingest policy that is E2E-40's unbounded scan inside a single-threaded actor
with a fixed CPU ceiling.

This reverses a documented decision, not an oversight: `query.rs:306` states the
intent — "`None` is the match-all query; `Some(items)` is a filtered one" — and
the ADR that lands it must say it is reversing a choice. The `Option`-shaped
encoding was chosen for compactness in JSON and it is exactly the compactness
that makes the two values indistinguishable.

An externally tagged enum is chosen over an internally tagged one because
internally tagged representations require a self-describing format and postcard
is not one. In postcard the encoding is a varint discriminant followed by the
payload; in JSON it is `"All"` or `{"Items":[…]}`.

#### WF-4 — An `AppendCondition` encodes its guards explicitly, and an empty object is not one

`AppendCondition` MUST encode as a struct with a single `guards` field holding a
non-empty sequence. Each guard MUST encode both `fail_if_events_match` and
`after`, with `after` always present as an explicit `Option`.
`serde_json::from_str::<AppendCondition>("{}")` MUST fail.

`[FROZEN]`
`Rule:` `wire::empty_object_is_not_a_condition`, `wire::condition_round_trips`,
`wire::condition_after_is_visible_to_an_ingest_policy`
`Cases:` E2E-35, E2E-37, E2E-40
`Rejects:` the current pair of behaviours, which must be fixed together.
`AppendCondition::Wire.fail_if_events_match` carries no `#[serde(default)]`
(`append.rs:117-123`), and `from_str::<AppendCondition>("{}")` nevertheless
returns `Ok` with `Query::All` — it succeeds only because `Query`'s `Deserialize`
is `Option`-shaped. Once WF-3 makes `Query` explicitly tagged, `{}` becomes a
hard error, which is the intended outcome, and the two changes must land in one
commit or the intermediate state is a decode that fails for the wrong reason.

`after` is always present because E2E-37's ingest rule depends on seeing it. A
store-local position on the wire has no referent at the receiver —
`is_violated_by` compares raw positions (`append.rs:95-107`), so `after: 288455`
in hub numbering names an unrelated recent event and the check passes vacuously,
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
MUST encode as a struct of `origin` and `position`.

`[FROZEN]`
`Rule:` `wire::store_id_encodes_as_hex_in_json`,
`wire::store_id_encodes_as_bytes_in_postcard`
`Cases:` E2E-34, E2E-42
`Rejects:` encoding a `StoreId` as a byte sequence in JSON, which produces a
32-element array of integers that no operator can read and that costs four times
the bytes. Also rejects encoding it in UUID form with hyphens and version
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
every envelope shape including all-defaults and maximum-size values
`Cases:` E2E-33, E2E-42
`Rejects:` a test matrix of one format. One self-describing format and one
non-self-describing format is the minimum that discriminates: D1's five
attributes serialise and deserialise perfectly in JSON and desynchronise the
stream in postcard, so a JSON-only matrix certifies a broken format. A third and
fourth format add maintenance and no information, because every failure mode
either belongs to the self-describing class or to the positional class.

Postcard specifically, rather than bincode, because it is `no_std`-friendly and
is the format a constrained replication peer would actually reach for.

#### WF-8 — The envelope is versioned, once per message, and an unknown version is refused whole

Every replication message MUST carry a `format_version` as its first field. The
version MUST be bumped whenever the shape of any wire type changes. A receiver
that does not implement a version MUST refuse the entire message with a
distinguishable error and MUST NOT attempt a partial decode.

`[FROZEN]`
`Rule:` `wire::rejects_an_unknown_format_version`,
`wire::version_is_the_first_field`
`Cases:` E2E-35, E2E-42
`Rejects:` per-type versioning, which costs bytes on every event in the log and
still cannot express a change to the *relationship* between two types — for
instance, the move of `after` from `AppendCondition` into `Guard` (VT-30), which
changes no single type's shape in isolation. Also rejects a version field placed
anywhere but first: in a positional format, a decoder that has already consumed
two fields incorrectly cannot recover to read a version, so a trailing version is
a version nobody can read when they need it.

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
`Rule:` `wire::decode_accepts_an_over_capacity_value`; new
`append_reports_exceeded_store_limits`
`Cases:` E2E-42
`Rejects:` the assumption in PRESSURE-TEST §5.4 that each new bound is "a
wire-compatibility break with no quarantine path". It is a break only if the
bound is enforced in `Deserialize`, which VT-19 forbids for exactly this reason.
It also rejects the alternative repair — bumping the format version whenever a
limit changes — which would make every peer in a heterogeneous deployment
unreachable from every other the moment one of them raised a limit.

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
already do this correctly (`tag.rs:317-324` re-canonicalises, `query.rs:297-302`
re-validates through `QueryItem::new`) and E2E-40 credits them: "the envelope
defends its own invariants; only cost is unguarded". This clause is what stops
that being an accident.

One consequence retires a concern recorded in PRESSURE-TEST §6: re-canonicalising
`Tags` on the way in means a foreign peer's hash over its own tag order will not
agree with the receiver's. That mattered only under a content-hash identity,
which VT-2 and VT-5 reject on independent grounds. Under `(StoreId, position)`
identity the re-canonicalisation costs nothing.

#### WF-11 — Payloads encode as base64 in human-readable formats

`Event::data` and `Event::metadata` MUST encode as standard-alphabet base64 in
human-readable formats and as raw byte strings otherwise.

`[PROVISIONAL — falsified if the base64 implementation cannot be made `no_std`,
or if a peer must forward a payload too large to buffer through an encoder; the
Workers peer under a memory limit is the instrument]`
`Rule:` `wire::payload_is_base64_in_json`, `wire::payload_is_raw_in_postcard`
`Cases:` E2E-33
`Rejects:` the default. `bytes::Bytes` serialises through `serialize_bytes`, and
`serde_json` renders that as an array of decimal integers — so Turnstile's 340 KB
seat map becomes roughly 1.3 MB of JSON that no human can read. The format that
exists for diagnostics should be diagnosable.

The dependency is confined to the optional `serde` feature and is one small
crate. That is a real addition to a crate whose dependency graph is deliberately
tiny (ADR-0003), which is why the clause is provisional rather than frozen.

#### WF-12 — `ReadOptions` is not part of the wire format

`ReadOptions` MUST NOT implement `Serialize` or `Deserialize`.

`[FROZEN]`
`Rule:` compile test `read_options_is_not_serialisable`
`Cases:` none directly; this removes a surface rather than adding one, and it is
a clause because the impls exist today and something must say to delete them.
`Rejects:` a replication protocol that ships a `ReadOptions` as its "send me
more" request. That is the obvious use and it is wrong twice over: `from` is a
store-local position with no meaning at the sender, and `backwards`/`limit` are
traversal options for a local reader that a peer has no business setting. A sync
request message carries its own fields, defined in `happenstance-sync`, whose
meaning is negotiated between the two peers.

The impls at `query.rs:323-351` are on no wire path that exists and are
maintenance the crate does not owe. They also carry a fifth
`skip_serializing_if`-shaped hazard by using `#[serde(default)]` on the whole
struct, which is the same positional-desynchronisation class as WF-2.

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
It is also the port with the most evidence behind it — twenty-seven conformance
rules, one reference implementation, six deployment scenarios walked line by line
against it, and one adjudicated pressure test. Where the evidence runs out, the
clause says so and names the experiment that ends the argument.

Clauses are `ES-n`. Each carries a maturity marker, the conformance rule that
checks it, the end-to-end cases it serves, and the wrong implementation it
forbids. A clause with no wrong implementation to name is not a clause; two such
statements appear below as prose and are labelled.

Rule names in `code font` that appear in
[`crates/happenstance-testkit/src/suite.rs`](../../crates/happenstance-testkit/src/suite.rs)
today are existing rules. Rule names marked **(new)** are specified here and do
not exist yet; §3.8 collects them.

---

### 3.1 Derivation — why there are two traits, and what that costs

The port is declared once without any `Send` requirement and the `Send` flavour
is derived (`store.rs:91`, ADR-0001). The mechanism is not folklore and its
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

- **Rule:** every rule in `suite.rs` is generic over `S: EventStore`
  (`suite.rs:70`, and identically at each of the twenty-seven), so an adapter
  implementing only `SendEventStore` proves the implication by passing the suite —
  which `MemoryEventStore` does (`memory.rs:147`). `cargo xtask wasm` proves the
  bare flavour still builds for `wasm32-unknown-unknown`.
- **Cases:** E2E-52, E2E-53, E2E-54.
- **Rejects:** any redefinition that injects `+ Send` unconditionally. Such a port
  compiles and passes every behavioural rule on native and fails
  `cargo xtask wasm`, which is why that step is in the gate rather than in a
  comment.

The naming departs from `trait_variant`'s own convention, which would call the
bare flavour `LocalEventStore`. "Local" already names a local-first
application's on-device store in this project; the collision would be permanent
(ADR-0001:71-75).

#### ES-2 — `read` returns the stream at the top level and is not `async`

`read` MUST return `impl Stream<Item = Result<SequencedEvent, Self::Error>>` as
the outermost item of its return type, and MUST NOT be `async`.

**[FROZEN]**

- **Rule:** `send_flavour_stream_is_send_in_generic_code` **(new)**. The existing
  unit test at `memory.rs:328-340` is vacuous: it asserts `Send` on a *concrete*
  stream type, which holds by auto-trait leakage whatever the trait says. The
  replacement is a generic function — `fn assert<S: SendEventStore>(s: &S, q:
  &Query) { fn is_send<T: Send>(_: &T) {} is_send(&SendEventStore::read(s, q,
  ReadOptions::new())); }` — which type-checks only if the *trait* promises it.
- **Cases:** E2E-52.
- **Rejects:** the refactor to `async fn read(..) -> Result<impl Stream, E>`.
  `transform_item`'s `async fn` arm (`variant.rs:133-145`) appends `Send` to the
  outermost `impl Future` and nothing else, so the future would be `Send` and the
  stream would not: a caller could not hold a read across an await inside
  `tokio::spawn`, which is the entire reason the `Send` flavour exists. The
  refactor compiles, passes all twenty-seven rules, and silently deletes the
  design. CLAUDE.md constraint 3 protects this; the test it names must be replaced
  by the one above, because the one on disk cannot fail.

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
  with `error[E0277]: cannot be shared between threads safely`. It also is not one
  line at the impl sites, which spell the bound by hand: `memory.rs:154` and
  `crates/happenstance-sqlite/src/event_store.rs:65` both write `+ Send` and would both
  need `+ Send + Sync`. Nothing in the workspace has been checked against that.

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
`type Error: core::error::Error + 'static` (`store.rs:99`) is copied into the
variant verbatim, and the blanket impl forwards it as
`type Error = <Self as SendEventStore>::Error` (`variant.rs:238-245`). There is no
mechanism by which `Error: Send + Sync` could apply to the derived flavour alone.
Whatever ES-6 settles applies to `wasm32` too.

- **Rule:** `error_bound_is_identical_on_both_flavours` **(new)** — a static
  assertion in `happenstance`'s own tests over both flavours' `Error` projections.
- **Cases:** E2E-53.
- **Rejects:** the plausible compromise — abandoning `trait_variant` for two
  hand-written traits so the bound can differ. That is the `umadb-dcb` shape
  ADR-0001:99-100 rejected, it doubles the surface the suite must cover, and it
  loses the blanket impl that makes ES-1's "bind the weaker one" work.

#### ES-6 — Whether `Error` gains `Send + Sync`

**[DEFERRED — settled by the four-skeleton experiment: one adapter skeleton per
storage shape (rusqlite, tokio-postgres, a Workers HTTP client, Neon over
one-shot HTTP), each declaring its real `Error`, future and stream types with
`todo!()` bodies, compiled against both flavours; plus the `JsValue`
stringification probe on the Cloudflare skeleton. Owning phase: the phase that
produces ADR-0008, which cannot exit without it. PS-35 requires that ADR to
cover `ProjectionStore` in the same document, so this is one experiment and one
decision, not two.]**

`type Error: core::error::Error + 'static` (`store.rs:99`) carries no `Send` or
`Sync` bound today, so an adapter error holding a `JsValue` or an `Rc<str>`
satisfies it. A spawned handler's error therefore cannot cross a `JoinHandle`,
which is E2E-53's failure. Adding the bound is free on `wasm32` only if
stringifying a `JsValue` loses nothing the caller needs, and
`worker::Error::JsError(String)` suggests it does not — but *suggests* is not
evidence, and the two in-tree "confirmations" are free by construction:
`MemoryStoreError` is uninhabited (`memory.rs:143-145`) and
`SqliteEventStoreError` is a single placeholder variant
(`crates/happenstance-sqlite/src/event_store.rs:52-56`). Neither could fail the bound if
it were wrong.

This is **semver-visible and must be decided before publish** (ES-5: it cannot be
added to one flavour later). It is the highest-blast-radius open question in the
workspace, and the pressure test's adjudication is that the evaluation's verdict
imposing it must be withdrawn to open (PRESSURE-TEST §3.1).

- **Rule:** `store_error_crosses_a_join_handle` **(new)**, writable only once
  decided; it belongs in an opt-in `Send`-flavour rule group, because the bare
  flavour cannot be spawned by construction.
- **Cases:** E2E-53, E2E-52.
- **Rejects:** deciding it by argument. Both prior documents did, in opposite
  directions, from the same file.

#### ES-7 — A downstream crate may implement the bare flavour directly

Implementing `EventStore` for a local type in a downstream crate MUST NOT collide
with the blanket impl the derivation emits.

**[PROVISIONAL — falsified by `error[E0119]: conflicting implementations` on a
downstream `impl EventStore for LocalType`. The named test is the `!Send`
reference store, which lives in `happenstance-testkit` — a genuinely downstream
crate — and which is also ADR-0001's own lift condition.]**

`mk_blanket_impl` (`variant.rs:171-192`) emits
`impl<T: SendEventStore> EventStore for T`. ADR-0001:59-60 records a spike
confirming a downstream direct impl is accepted. Nothing in the tree corroborates
it: `grep -rn "impl EventStore for"` over the workspace returns **nothing**, and
both existing impls are `SendEventStore` (`memory.rs:147`,
`crates/happenstance-sqlite/src/event_store.rs:58`), one of which is `todo!()`. The bare
flavour — the entire justification for the two-trait design — has zero
implementers, which is precisely why ADR-0001 is still marked provisional.

- **Rule:** not a new rule but a new *invocation* of the existing suite — the
  `RefCell`-backed `!Send` reference store CF-28 requires in
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

- **Rule:** `read_defaults_to_ascending_order` (`suite.rs:248-259`),
  `read_from_is_inclusive` (`:262`), `read_backwards_reverses_order` (`:279`),
  `read_backwards_from_with_limit` (`:306`).
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

- **Rule:** `query_matching_nothing_yields_empty` (`suite.rs:231`) is the
  complementary half already in the suite — an empty result is the correct answer
  when nothing matches, and never an error — and it is named here because no
  other clause names it. `read_from_a_gap_position` **(new)** — arrange a store with a gap
  (append three events, remove one through the adapter's own fixture, or use an
  adapter that allocates sparsely), then read `from` a position between two
  assigned ones and assert the higher neighbour is yielded. Every existing rule
  feeds `from` a position the store actually assigned (`suite.rs:262-276`,
  `:306-329`), so none of them can fail this.
- **Cases:** E2E-10.
- **Rejects:** an adapter implementing `from` as an equality seek or a
  `rowid`-offset lookup rather than a range scan — plausible wherever positions
  came from a dense counter and the author assumed density. `event.rs:94-96` says
  gaps are permitted; nothing checks that anyone believed it.

#### ES-10 — Position order is visibility order

Once any reader has observed an event at position *P*, no subsequent read
against that store MAY yield an event at a position ≤ *P* that was not already
visible. An adapter MUST NOT make an event visible at a position below one it has
already exposed.

**[PROVISIONAL — axis: **position allocation**, whose far end is unbuilt. Every adapter in and planned for the workspace assigns positions under a lock it holds until commit (`memory.rs:195-223`), so nothing here can violate this clause and nothing here votes for it. Falsified if the phase-2 measurement finds no affordable Postgres mechanism among `xid8` + `pg_snapshot_xmin`, transaction-scoped advisory locks and a serialised sequence table — in which case the invariant is not free and ES-25 and ES-26, which are sound only where it holds, are reopened with it.]**

**This clause is the sole statement of the visibility invariant.** VT-12 carried a
second copy; the two drifted apart within one editing pass, which is the argument
against stating a requirement twice. VT-12 is retained as a cross-reference so
existing citations resolve, and every `PS` and `SY` clause that depends on the
invariant cites this one.

It is provisional on cost, not on correctness, and the distinction decides what a
bad result means. ES-25 and ES-26 are sound only where visibility order agrees
with position order, so they are built on top of this clause: if it falls they
fall with it, and that is a contract change rather than an adapter inconvenience.
What phase 2's probe settles is which mechanism buys the invariant — `xid8` +
`pg_snapshot_xmin`, a transaction-scoped advisory lock, or a serialised sequence
table — and at what write rate. One affordable answer lifts this clause to
`[FROZEN]` at phase 4. **Three unaffordable answers do not make the clause wrong;
they make it expensive, and the decision then is whether happenstance requires
something Postgres cannot cheaply give** — which is a decision worth reaching
deliberately at phase 2 rather than discovering at phase 10, after the alpha has
shipped against it.

This is the property that makes `AppendCondition::after` mean anything.
`is_violated_by` compares position *values* (`append.rs:95-107`); nothing in the
contract requires an event becoming visible later to carry a higher position.
`event.rs:92-97` documents uniqueness, monotonicity and permitted gaps — all
properties of *assignment*, none of *visibility*.

- **Rule:** `nothing_below_an_observed_position_appears_later` **(new)**. The rule
  is worthless without something that can fail it: the testkit's own `tests/` must
  carry a deliberately hostile store that holds one append's row back until a
  second, later-positioned append has committed. `positions_are_unique` and
  `positions_are_strictly_monotonic` (`suite.rs:336-365`) both read a quiescent
  store through a single sequential writer and are vacuous here — indeed vacuous
  generally, since both read back through `read`, which every adapter returns in
  position order.
- **Cases:** E2E-01, and the read-side half of E2E-02.
- **Rejects:** a Postgres adapter allocating positions with `nextval()` outside
  the transaction. A position is taken before the work and released at commit, so
  a transaction that started later can become visible earlier; a caller then
  conditions on `after: 100` while 99 is still invisible, and the boundary silently
  stops enforcing. It passes all twenty-seven rules today.

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

**[PROVISIONAL — axis: **transport**, whose far end is unbuilt. `MemoryEventStore` snapshots under the lock at call time (`memory.rs:155-181`) and satisfies this for free; an adapter reaching its store over one-shot HTTP with no cursor can only self-paginate, and a self-paginating read is not a snapshot. Falsified by the first one-shot-HTTP adapter that cannot meet this in one round trip — which is the outcome to expect, and the reason to have settled it before the freeze rather than after.]**

This settles D7 and E2E-02. Two conformant adapters have opposite semantics
today: `MemoryEventStore` filters, orders and truncates under the read lock and
streams from the resulting `Vec` (`memory.rs:155-181`), so its answer is a
snapshot; a self-paginating adapter over one-shot HTTP issuing an independent
`WHERE position > $last ORDER BY position LIMIT 5000` per chunk grows under the
caller's feet. The observable difference is one event versus two.

The obligation is stronger than laziness, and laziness survives it: an adapter MAY
defer all work to the first poll, which is what lets a million-event replay stream
without buffering (`store.rs:103-108`). What it may not do is take a *new*
snapshot per chunk. A paginating adapter meets this by capturing the head at the
first poll and bounding every subsequent page by `position <= H` — one extra
round trip at most, and zero if the first statement returns the head alongside the
first page. **This is the reason ES-30 (`head`) is in the port and not an inherent
method**: without it, the cheapest correct pagination is unavailable through the
port, and Neon's inability to meet the obligation in one round trip becomes an
argument for weakening the obligation rather than for supplying the primitive.

- **Rule:** `read_result_is_stable_under_concurrent_append` **(new)** — bind the
  query to a named local, build the stream, poll it once, append, drain, and
  assert the drained set is exactly the pre-append set.
- **Cases:** E2E-02, E2E-01.
- **Rejects:** the self-paginating one-shot-HTTP adapter above — the natural shape
  for `happenstance-neon`, and one of the two instruments the workspace has
  already chosen. It is conformant today.

#### ES-12 — All items of one `Query` share one snapshot

Every item of a `Query` MUST be evaluated against the same snapshot as every
other item of that same `read` call.

**[PROVISIONAL — axis: **transport**, with ES-11, and strictly harder: an adapter that issues one statement per query item satisfies ES-11 per item and still fails this. Falsified by the same adapter, and the failure mode is quieter — an event matching item 1 that lands between two statements is missed while the observed maximum position sits above it, so a condition built on that boundary admits a write it should have rejected, with no error anywhere.]**

- **Rule:** `query_items_share_one_snapshot` **(new)** — a multi-item query over
  disjoint tag families, an append matching item 1 interleaved between the
  adapter's evaluation of item 1 and item 4, asserting either that the late event
  is in the result or that the maximum position observed is below it. Deterministic
  only against a fixture that can be paused between statements, so this rule ships
  with a hostile fixture in the testkit's own `tests/` — the same instrument ES-10
  needs.
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
inside one expression (`suite.rs:46`).

The alternative was taking `Query` by value. It loses on cost and on retry
ergonomics: `Query::Items` is a `Box<[QueryItem]>` (`query.rs:149`) whose items
each own a `Box<[EventType]>` and a `Tags`, so by-value would allocate on every
read, and a command handler that reads with a query and then reuses it in the
append condition — the documented shape (`append.rs:18-32`) — would clone it
anyway. A borrow costs the caller one named local in the one situation where the
stream outlives the statement.

- **Rule:** `read_result_is_stable_under_concurrent_append` **(new)** does not
  compile without this shape, so the rule is the check.
- **Cases:** E2E-02, E2E-03.
- **Rejects:** the "fix" that changes `read` to take `Query` by value in order to
  make the E0716 go away. It compiles, it breaks every adapter signature, and it
  puts an allocation on the hottest path in the port.

#### ES-14 — `limit` truncates the whole result, after ordering

`ReadOptions::limit(n)` MUST yield the first *n* events of the ordered result set,
not *n* per query item and not an arbitrary *n*.

**[FROZEN]**

- **Rule:** `read_limit_truncates` (`suite.rs:292`),
  `read_backwards_from_with_limit` (`:306`), plus
  `limit_applies_across_items_not_per_item` **(new)** — a two-item query matching
  three events each, `limit(4)`, asserting four events in position order.
- **Cases:** E2E-12, E2E-13.
- **Rejects:** an adapter implementing a multi-item query as one statement per
  item with `LIMIT n` on each — the same shape ES-12 rejects, failing here for an
  independent reason, which is why both rules are worth having. It also rejects a
  backwards adapter that truncates before reversing; `MemoryEventStore` reverses
  first (`memory.rs:163-179`) and this is the behaviour E2E-12 depends on.

#### ES-15 — Query item algebra: order-free, duplicate-free, and All absorbs

For queries *a* and *b*, the set selected by `Query::from_items(items(a) ++
items(b))` MUST equal the union of the sets selected by *a* and by *b*. Item order
MUST NOT change the result set. An event matching two or more items MUST be
yielded exactly once. An adapter MUST NOT deduplicate, reorder or otherwise
rewrite a query's *items*.

**[FROZEN]**

- **Rule:** `duplicate_items_do_not_duplicate_events` **(new)** — a store holding
  one multi-tagged event, read through `Query::all()` and through a two-item query
  both of whose items match it, asserting it appears once; and
  `query_item_order_does_not_change_the_result_set` **(new)**.
- **Cases:** E2E-32.
- **Retires:** `query_all_matches_every_event` — superseded by this clause's
  `duplicate_items_do_not_duplicate_events`. The existing rule appends three
  *untagged* events, so the fan-out a tag join without `DISTINCT` produces cannot
  occur; it is not wrong, it is weaker than the clause it would be protecting, and
  a clause naming it would be claiming coverage it does not have. Phase 3 rewrites
  it rather than keeping both.
- **Rejects:** two real implementations. A tag join without `DISTINCT`: an event
  carrying three tags yields three rows, and nothing in the suite reads
  `Query::all()` on a store holding a multi-tagged event, so it passes today. And
  an adapter that sorts and dedups *items* as an optimisation —
  `QueryItem::new` already sorts and dedups *types* (`query.rs:62-67`), so
  extending it one level up is the natural next step. The fan-out projection
  runner (one read of the union query, twenty local re-filters through
  `Query::matches`, `query.rs:194-204`) rests entirely on this algebra, and
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
`ReadOptions` is `#[non_exhaustive]` and passed by value, so adding `to` is not a
trait signature change — but it *is* a new obligation on every adapter, and an
adapter that ignores an unknown field silently returns too much. **If `to` lands
at all it must land before the first adapter ships**, and none has.

`ReadOptions` has `from`, `backwards` and `limit` and no upper bound today
(`query.rs:226-234`). A backfill worker cannot be given the closed window [1, *H*]
while a tail worker owns (*H*, ∞). `limit` cannot stand in, because
`event.rs:94-96` forbids treating position arithmetic as a count. The workaround —
`from(H).backwards()`, buffer, reverse in memory — *is* a pinned window and it
works at 211 events and is fatal at 53 million.

- **Rule:** `read_to_is_inclusive` **(new)**,
  `read_from_and_to_bound_a_closed_window` **(new)**,
  `read_to_under_backwards_bounds_the_older_end` **(new)** — VT-29 names the same
  three.
- **Cases:** E2E-11.
- **Rejects:** an adapter that accepts `ReadOptions` by value, matches on the
  fields it knows and ignores the rest — the shape every `#[non_exhaustive]`
  options struct invites, and the one that turns a bounded backfill into an
  unbounded one with no error anywhere. It also rejects an adapter that reads
  `to` as exclusive, which yields a window one event short at every chunk
  boundary and is invisible until the chunks are reassembled.

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

An owning adapter must clone — `memory.rs:219-221` does. The consequence nobody
had written down is that `Event::into_parts` (`event.rs:243-246`) is unreachable
from any trait impl, so its doc comment, "Decomposes the event, avoiding a clone
in adapter write paths", is **false as written**. This specification requires that
comment corrected; `into_parts` is retained for callers and wire encoders in the
typed layer, which can reach it, rather than for adapters, which cannot.

The borrow wins on three grounds. `Event`'s expensive fields are `Bytes`
(`event.rs:185`, `:187`), so a clone bumps a refcount rather than copying the
payload; the remaining cost is one `Box<str>` and one boxed tag slice, bounded by
the tag count. A rejected append clones **nothing** — `memory.rs:205` returns
before the `extend` — and rejection is the routine outcome under contention. And
`ConditionViolated` obliges the caller to keep its events across the call, so
by-value would move the clone from the adapter's success path to the caller's
every path.

- **Rule:** `append_preserves_event_payload` (`suite.rs:417`) — the round-trip
  that catches a lossy clone.
- **Cases:** E2E-36, E2E-39.
- **Rejects:** an adapter that loses metadata or tag order while copying into its
  own row type — concretely, the SQLite shape that stores tags in a side table
  (`crates/happenstance-sqlite/src/event_store.rs:21-27`) and forgets to write
  `metadata`. `append_preserves_event_payload` compares the whole `Event` and
  catches it.

#### ES-18 — Atomicity

Either every event in the batch lands or none does. A rejected append MUST leave
the store byte-identical.

**[FROZEN]**

- **Rule:** `append_is_atomic` (`suite.rs:385`),
  `condition_rejection_leaves_store_unchanged` (`:529`).
- **Cases:** E2E-39, E2E-48, E2E-07.
- **Rejects:** a per-row conditional `INSERT ... SELECT ... WHERE NOT EXISTS`,
  which can write the first event of a batch and refuse the second.

This clause is what makes the ingest decision implementable without a new seam.
Under unconditional ingest with compensation, the receiving side appends
`[losing_event, compensating_event]` as one batch under a guard keyed on the
compensation's own identity — one call, `append(&[losing, superseded],
Some(&guard))`, all-or-nothing by this clause and conformance-tested by
`append_is_atomic`. No reader ever observes the losing event unresolved. The half
of that design that needs new machinery is the identity that makes the guard
idempotent, not the atomicity.

#### ES-19 — The returned position

`append` MUST return the position assigned to the **last** event of the batch.
Positions within one batch MUST be assigned in slice order and MUST be strictly
ascending.

**[FROZEN]**

- **Rule:** `append_returns_last_written_position` (`suite.rs:372`), plus
  `batch_positions_follow_slice_order` **(new)** — append a batch of three
  distinguishable events and assert their read-back order matches the slice order,
  rather than inferring it from `all.last()` as the existing rule does.
- **Cases:** E2E-13, E2E-23.
- **Rejects:** an adapter that returns the store head rather than its own batch's
  last position — identical on a quiescent store and wrong under a second writer
  (E2E-08) — and an adapter whose bulk insert returns generated keys in
  unspecified order.

The contract MUST NOT require a batch's positions to be *contiguous* or free of
interleaving by another writer; gaps are permitted (`event.rs:94-96`) and a store
that allocates outside its transaction cannot promise it. One consequence belongs
in the port's documentation and is stated here as prose rather than as a clause,
because no rule can check a caller: the returned position is for checkpointing and
for reporting. It is **not** a sound `after` for a follow-up condition unless the
caller has read up to it, because a foreign event may hold a position below it
that the caller never saw. `store.rs:126-128` currently offers it for exactly that
use and must be corrected. The sound `after` comes from a read —
`read_decision_model` (`store.rs:198-208`) — which is what the DCB loop already
does.

#### ES-20 — An empty batch is refused, and refused first

`append(&[], _)` MUST return `AppendError::NoEvents`. The emptiness check MUST
precede the condition check, so that an empty batch under a condition that would
have been violated still returns `NoEvents`.

**[FROZEN]**

The precedence is not arbitrary. `ConditionViolated` is the DCB concurrency
signal and its documented meaning is "rebuild the decision model and retry"
(`error.rs:90-92`). An empty batch is a caller bug: the retry can never succeed,
so reporting the violation puts a correct client into a loop that never
terminates and attributes a code defect to contention. `NoEvents` names the actual
fault.

- **Rule:** `append_rejects_empty_batch` (`suite.rs:406`) covers only
  `append(&[], None)`, so it cannot see the precedence.
  `empty_batch_is_refused_before_the_condition_is_evaluated` **(new)** — seed an
  event, then `append(&[], Some(&condition_that_matches_it))`, assert `NoEvents`.
- **Cases:** E2E-06; PRESSURE-TEST §6 S3.
- **Rejects:** `MemoryEventStore` as it stands. `memory.rs:197-216` evaluates the
  condition first and returns `ConditionViolated` where `NoEvents` is the caller's
  actual bug. A rule whose first casualty is the reference implementation is the
  opposite of decorative.

#### ES-21 — A batch's own events are not evaluated against its own condition

A condition MUST be evaluated only against events the store already held. Events
in the batch being appended MUST NOT be considered.

**[FROZEN]**

- **Rule:** `batch_is_not_evaluated_against_its_own_condition` **(new)** — a
  two-event batch under a condition whose query matches both, `after` set past
  everything stored, asserting success and both events landing.
- **Cases:** E2E-06.
- **Rejects:** the conditional `INSERT ... SELECT ... WHERE NOT EXISTS` strategy
  applied per row, which the decision ledger carries as a live candidate
  (`RUNBOOK.md:64`). It self-rejects on any single-event batch whose condition
  names the type it is appending — the shape of `RegisterChargePoint`,
  `ClaimPooledUnit`, `PlaceHold` and `ReserveTourAllocation` across four
  scenarios — and on any multi-event batch whose second event matches. The
  reference answer is "no" only by accident of implementation order
  (`memory.rs:197-221` checks `stored` before extending), and no rule covers it.

#### ES-22 — Dropping an `append` future leaves no partial batch

If an `append` future is dropped before completion, the store MUST be in one of
the two states ES-18 permits: fully applied, or unchanged. It MUST NOT be
partially applied.

**[FROZEN]**

- **Rule:** `dropped_append_future_leaves_no_partial_batch` **(new)** — build the
  future, poll it once with a no-op waker, drop it, then read the whole store and
  assert the batch is present in full or absent in full.
- **Cases:** E2E-07.
- **Rejects:** an adapter that executes a batch as several statements outside a
  transaction and relies on running to completion. `MemoryEventStore` passes
  trivially, because `memory.rs:184-224` contains no `.await` at all — which is
  exactly why the reference store cannot answer this question and a real adapter
  must.

#### ES-23 — Cancellation outcome is unspecified, and that is the contract

An adapter MAY commit an append whose future was dropped. A caller MUST NOT treat
a dropped future as evidence that the append did not commit. The port MUST
document this in an explicit `# Cancellation` section, and each adapter MUST state
which of the two it does.

**[FROZEN]**

The honest paragraph, since silence is what the pressure test calls the single
most under-specified thing in the repository. At the edge, dropping the future is
the *normal* termination path: a client disconnect, a CPU limit, a Durable Object
eviction, a pod eviction. `AppendError` has three variants and none means "the
outcome is unknown" (`error.rs:148-166`) — and adding one would not help, because
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
`(event_type, data, tags, metadata)` (`event.rs:183-188`) and carries no identity,
`append` has no slot for a caller-supplied one (`store.rs:141-145`), and a query
matches on type and tags only (`query.rs:113-116`), so an identity in `metadata`
is structurally unqueryable and breaks ADR-0003 at the point ADR-0003 claims to
win.

- **Rule:** `reissued_conditional_batch_lands_once` **(new)** — append a batch
  under a condition its own events match, append the identical batch again, assert
  the second attempt returns `ConditionViolated` and the store holds one copy.
  Writable today: it needs no identity, which is what makes this clause severable
  from `EventId` and is why the deferral it carried was larger than the question.
  Its companion, `reissued_unconditional_batch_lands_twice`, asserts the stated
  limit — a rule that pins a *non*-guarantee, so that an adapter cannot quietly
  strengthen it and leave callers depending on behaviour the contract disclaims.
- **Cases:** E2E-07, E2E-33, E2E-36.
- **Rejects:** the metadata-key proposal in `crates/happenstance-sync/src/lib.rs:73-75`,
  and equally the naive fix of promoting identity to a tag — maximal cardinality in
  the column adapters are told to index, an entry in every `contains_all`
  merge-scan (`tag.rs:231-245`), a writer-forgeable identity, and an identity
  dimension visible to every tag-only query.

---

### 3.4 Append conditions

The shape shown here is the one on disk today:

```rust
pub struct AppendCondition {
    pub fail_if_events_match: Query,
    pub after: Option<SequencePosition>,
}
```

VT-30 replaces it with a non-empty sequence of guards, each a `(Query,
Option<SequencePosition>)` pair, and that clause is `[PROVISIONAL]`. Every clause
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

- **Rule:** `condition_without_after_rejects_any_match` (`suite.rs:438`),
  `condition_without_after_allows_non_match` (`:458`),
  `condition_after_ignores_non_matching_events` (`:512`),
  `condition_rejection_is_reported_as_condition_violated` (`:549`),
  `condition_rejection_leaves_store_unchanged` (`:529`).
- **Cases:** E2E-08, E2E-55, E2E-39.
- **Rejects:** an adapter that folds the violation into its own error type, which
  destroys the caller's only means of telling "retry the decision" from "something
  broke" without pattern-matching on strings; and an optimistic adapter that
  inserts, then probes, then reports the violation without rolling back.

`ConditionViolated::conflicting_position` (`error.rs:96-104`) is informational.
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
`AppendCondition::after_opt` consumes it (`store.rs:198-208`, `append.rs:85-88`).
The pairing that does *not* fall out is the checkpoint resume path, where
`ProjectionStore::checkpoint` returns an inclusive-consumed position and
`ReadOptions::from` is inclusive, so the caller must advance by hand through
`SequencePosition::next()` (`projection.rs:84-93`) — a method whose own
documentation says it is meaningful only on densely-allocating adapters
(`event.rs:138-144`). PS-20 settles the off-by-one — resume strictly
after the checkpoint, inclusively from the store's first position when
`NeverRun` — and VT-13 fixes `next()` so the idiom is sound over gaps; it is
named here because it is the same boundary seen from the other side.

- **Rule:** `condition_after_ignores_events_at_the_boundary` (`suite.rs:477`),
  `condition_after_rejects_events_beyond_the_boundary` (`:494`),
  `read_from_is_inclusive` (`:262`).
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

- **Rule:** `condition_matches_on_tags` **(new)** — two events sharing a type and
  differing in tags, a condition tagged for one of them, asserting rejection; and
  `condition_with_an_unheld_tag_does_not_reject` **(new)**, its mirror, asserting
  acceptance for a tag no event carries. CF-7 and CF-8 are the obligations to
  have the pair. The read-side statement of the same semantics is pinned by
  `query_item_types_are_or` (`suite.rs:83`), `query_item_tags_are_and`
  (`:96`) and `query_item_combines_types_and_tags_with_and` (`:167`) — this
  clause's content is that the *condition* path is evaluated by those same rules,
  which no existing rule checks.
- **Cases:** E2E-55, E2E-03; PRESSURE-TEST §3.9.
- **Retires:** `racing_conditional_appends_elect_one_winner` — its one tagged
  condition runs against a store where a type-only probe returns an identical
  verdict (`crates/happenstance-testkit/src/suite.rs:603-605`), so it cannot
  observe the property this clause exists to enforce. It is also sequential and
  single-handle, which is why ES-34 declines to name it either. Phase 3 replaces
  it with this clause's rule and CF-7's.
- **Rejects:** an adapter that drops the tag join from its condition probe. This
  is the natural first cut, because the join is the expensive half and the planned
  SQLite schema puts tags in a separate table
  (`crates/happenstance-sqlite/src/event_store.rs:21-27`). Every append-condition rule in
  the suite builds its condition from `query_of_types` — `suite.rs:390`, `:447`,
  `:465`, `:483`, `:501`, `:519`, `:537`, `:561` — and the one tagged condition
  (`:603-605`) runs against a store where a type-only probe returns identical
  verdicts. **No rule's verdict depends on the condition path matching tags
  today.** The adapter that drops the join passes the whole suite while rejecting
  every command that touches any tagged entity: a total-availability failure
  certified as conformant, on the canonical DCB uniqueness shape.

#### ES-28 — Degenerate condition inputs

A condition evaluated against an empty store MUST NOT reject. A condition whose
`after` names a position at or beyond the store's head MUST NOT reject and MUST
NOT error.

**[FROZEN]**

- **Rule:** `condition_against_an_empty_store_admits_the_append` **(new)** and
  `condition_after_beyond_head_admits_the_append` **(new)**. No rule evaluates a
  condition against an empty store today.
- **Cases:** E2E-56, E2E-08.
- **Rejects:** an `EXISTS`-probe adapter whose SQL returns a NULL the code reads
  as true on an empty table, and an adapter that validates `after` against its own
  head and errors on a position it has not assigned — plausible, defensible, and
  fatal to a peer resuming after a gap.

#### ES-29 — `after` is store-local

A `SequencePosition` is meaningful only within the store that assigned it. A store
MUST NOT evaluate an `AppendCondition` whose `after` was assigned by a different
store.

**[FROZEN]**

`AppendCondition` derives `Serialize` behind the `serde` feature and puts `after`
on the wire as a naked integer (`append.rs:117-123`), which makes the single most
dangerous replication mistake the path of least resistance. At a receiver,
`after: 288455` names an unrelated recent event, `is_violated_by` compares raw
values (`append.rs:95-107`), and the check runs over an arbitrary tail and passes
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
  crate is named at `crates/happenstance-sync/src/lib.rs:9-11` and does not exist.
- **Cases:** E2E-37, E2E-38, E2E-56.
- **Rejects:** a hub that deserialises a peer's condition and evaluates it
  verbatim. `AppendCondition` is `#[non_exhaustive]` with **public** fields
  (`append.rs:51-61`), so the refusal is checkable today: readable but not
  literal-constructible is exactly what makes an ingest-side policy possible.

The position-free form is already the default — `AppendCondition::new` leaves
`after: None` and `after`/`after_opt` are opt-in builders (`append.rs:65-88`) — so
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

Required rather than provided, and the reason is ES-3 rather than taste. The only
provided form that survives the clone into the variant needs `where Self: Sync`
(ES-4). A single-threaded on-device store — a `RefCell` in a Durable Object, an
`Rc`-shared cursor — is `!Sync`, and that adapter is the entire reason the bare
flavour exists. A provided method the edge adapter cannot call is a method the
port does not have. The cost of "required" is two impls today, one of which is
`todo!()` (`memory.rs:147`, `crates/happenstance-sqlite/src/event_store.rs:58`), and the
blanket impl forwards it for free (`variant.rs:194-237`), so generic code pays
nothing.

`head` is deliberately **not** parameterised by a query. A narrow projection's
problem is that it cannot advance past events it examined and did not match; the
global head is what lets it checkpoint past them. A query-scoped head would
reintroduce exactly the cost the method exists to avoid.

- **Rule:** `head_of_an_empty_store_is_none` **(new)**,
  `head_is_the_highest_visible_position` **(new)** (append, then assert `head()`
  equals the position `append` returned), `head_advances_across_two_handles`
  **(new)** (depends on ES-33).
- **Cases:** E2E-13, E2E-02, E2E-25.
- **Rejects:** three implementations that all compile. A cached last-written
  position, which is stale the moment a second handle writes (E2E-08). A `head`
  that reports the highest position matching some default query rather than the
  store's head. And the workaround as a universal answer —
  `read(&Query::all(), ReadOptions::new().backwards().limit(1))` is one cheap
  statement on local SQLite and one full HTTP round trip on the adapter with the
  smallest latency budget in the system; an adapter that implements it by
  materialising all matches before truncating fails E2E-13 outright, while
  `MemoryEventStore` reverses before truncating (`memory.rs:163-179`) and passes.

**`count()` does not ship, and this is a decision rather than an omission.** A
count has no consumer among the fifty-six cases; `SequencePosition` is explicitly
not a count (`event.rs:94-96`), so a count cannot be derived from one and cannot
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
The named measurement is the projection-runner benchmark that `RUNBOOK.md:66`'s
phase owes; the workspace has no benchmark harness and a conformance rule cannot
substitute for one, because complexity is a benchmark and not an assertion.]**

`RUNBOOK.md:66` owns this row and states the stake correctly: it changes the
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

Everything in this subsection is currently unreachable, and for one shared reason:
the conformance macro re-evaluates its factory expression once per test
(`crates/happenstance-testkit/src/lib.rs:83-91`) and every rule calls it exactly once
(`suite.rs:71` and identically throughout). The signature `F: Fn() -> S` already
*permits* two handles and nothing *asks* for it, so durability, reopen and genuine
multi-connection rules are all foreclosed by the fixture shape rather than by any
decision. ES-33 is therefore a precondition on the other three.

#### ES-33 — The fixture yields handles onto one backing store

The conformance fixture MUST distinguish "a fresh backing store" from "a handle
onto it". It MUST create a fresh, empty backing store once per rule, and MUST be
able to yield two or more handles onto that same backing store within one rule.
An adapter MUST supply both.

**[FROZEN]**

Concretely, the current `$factory: Fn() -> S` becomes a two-level fixture — a
value created per rule, with a `connect()` method returning an `S` — because "call
the factory twice" and "get two handles onto one store" cannot both be true of one
closure that also has to produce an empty store. CF-15 owns the fixture contract
and makes it a trait rather than a closure; `connect()` is its spelling and this
clause uses it. `MemoryEventStore` needs a
shared-backing constructor for this (its `RwLock<Vec<SequencedEvent>>` at
`memory.rs:72` moves behind an `Arc` that a handle clones); a file-backed adapter
points both handles at one temporary path; a pooled adapter hands out two pool
members.

- **Rule:** this is a conformance obligation on the testkit rather than on an
  adapter's behaviour; it is the enabling condition for the **(new)** rules
  `two_handles_share_one_consistency_boundary`,
  `head_advances_across_two_handles` and `acknowledged_writes_survive_a_reopen`,
  and it is stated as a clause because the suite cannot grow any of those three
  until it lands.
- **Cases:** E2E-08, E2E-46; PRESSURE-TEST §6 S6.
- **Rejects:** an adapter whose "second handle" is a clone of the first that
  shares one connection and one cache — which is how a pooled adapter would
  accidentally satisfy the letter of the fixture while defeating every rule built
  on it. The fixture's own documentation must require independent handles, and
  `two_handles_share_one_consistency_boundary` is what detects the cheat.

#### ES-34 — Two handles share one consistency boundary

An append made through one handle MUST be visible to a condition evaluated
through another handle onto the same backing store.

**[FROZEN]**

- **Rule:** `two_handles_share_one_consistency_boundary` **(new)** — handle *A*
  appends an event matching a condition, handle *B* appends under that condition
  with `after` set before *A*'s write, asserting `ConditionViolated`.
- **Cases:** E2E-08.
- **Rejects:** three adapters that pass all twenty-seven rules today. A cached
  `max(position)` fast path — a strategy the ledger explicitly defers rather than
  rules out (`RUNBOOK.md:64`). A per-connection repeatable-read snapshot, where
  the probe is correct only within its own session. An advisory lock scoped to a
  single pool member. All three are correct single-handle and wrong for any
  deployment where one store is reached two ways, which is every deployment with a
  connection pool.
  `racing_conditional_appends_elect_one_winner` (`suite.rs:596-638`) does not
  reach them: it is sequential *and* single-handle.

#### ES-35 — Acknowledged writes survive a reopen, where durability is claimed

An adapter that claims durability MUST make every append that returned `Ok`
readable after the backing store is closed and reopened. Durability is a declared
capability: an adapter that does not claim it MUST document that it does not, and
MUST NOT be presented as an event store of record.

**[PROVISIONAL — axis: **durability**, whose far end is unbuilt and, until CF-17 changes the fixture, is not even expressible: `conformance_test!` re-evaluates the factory per test (`crates/happenstance-testkit/src/lib.rs:83-91`), so no rule can hold a store across a reopen. Nothing in the workspace can currently fail a store that returns `Ok` from `append` and loses the write. Falsified by the first file-backed adapter, or sooner by the fixture that lets the question be asked.]**

Opt-in rather than universal, because `MemoryEventStore` must keep passing and is
by construction not durable (`memory.rs:14-33`). A capability that the reference
implementation cannot have is a capability the base suite cannot require.

- **Rule:** `acknowledged_writes_survive_a_reopen` **(new)**, using ES-33's
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
  Nothing in the workspace can currently express the question, let alone fail it:
  this is the axis with nothing at either end.

#### ES-36 — Two `append` futures on one `&self` interleave safely

Two `append` futures created from one store handle and polled alternately to
completion MUST both complete. Under mutually violating conditions exactly one
MUST succeed and the other MUST return `ConditionViolated`. Neither MUST panic.

**[FROZEN]**

- **Rule:** `interleaved_appends_on_one_handle_elect_one_winner` **(new)** — a
  `tokio::join!` of two `append` futures on a `current_thread` runtime, which
  needs no `Send` bound and so runs on both flavours.
- **Cases:** E2E-09.
- **Rejects:** a `RefCell`-backed adapter that holds its borrow across an awaited
  storage call, which panics at runtime; and, more subtly, one that drops the
  borrow around the await and therefore leaves an unspecified window between the
  condition probe and the write, which the winner assertion catches. This is the
  rule a Durable Object adapter would actually fail, and `MemoryEventStore` cannot
  surface it: `memory.rs:184-224` holds no lock across a suspension point because
  it has no suspension point.

---

### 3.7 Completeness — what a store that has been deleted from may look like

A store that holds only a suffix, or a scattered subset, of its own log is
indistinguishable from a complete one at every seam an ingest path or a runner can
see. Four scenarios reached this from different doors: a pruned device slice, a
regulated purge, a compacted peer, and a crypto-shred. The contract does not
assert completeness anywhere — `query_all_matches_every_event` (`suite.rs:70-80`)
is store-relative by wording and therefore accidentally correct — so a pruned
store passes all twenty-seven rules unchanged. That is not the gap. The gap is
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
which is the half the port has made indexable and queryable (`event.rs:209-221`
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
  axis; it should.
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
(`append.rs:95-107`) and has no third outcome. Where history is gone the
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

---

### 3.8 New conformance rules this section requires

The rules specified above do not exist yet. They fall into five groups by what
they need, and the ordering matters because three of the groups are blocked on
fixtures rather than on effort. The compile-level checks that live in
`happenstance`'s own `tests/` rather than in the suite —
`send_flavour_stream_is_send_in_generic_code` (ES-2),
`provided_method_future_is_send_in_generic_code` (ES-3, ES-4) and
`error_bound_is_identical_on_both_flavours` (ES-5) — are not counted here,
because CF-22's enumeration covers the suite and not the contract crate's tests.

**Writable against the suite as it stands (single handle, one store):**
`read_from_a_gap_position`, `read_result_is_stable_under_concurrent_append`,
`limit_applies_across_items_not_per_item`,
`duplicate_items_do_not_duplicate_events`,
`query_item_order_does_not_change_the_result_set`,
`batch_positions_follow_slice_order`,
`empty_batch_is_refused_before_the_condition_is_evaluated`,
`batch_is_not_evaluated_against_its_own_condition`,
`dropped_append_future_leaves_no_partial_batch`, `condition_matches_on_tags`,
`condition_with_an_unheld_tag_does_not_reject`,
`condition_against_an_empty_store_admits_the_append`,
`condition_after_beyond_head_admits_the_append`,
`head_of_an_empty_store_is_none`, `head_is_the_highest_visible_position`,
`interleaved_appends_on_one_handle_elect_one_winner`, `read_to_is_inclusive`,
`read_from_and_to_bound_a_closed_window`,
`read_to_under_backwards_bounds_the_older_end`.

**Blocked on ES-33's fixture:** `two_handles_share_one_consistency_boundary`,
`head_advances_across_two_handles`, `acknowledged_writes_survive_a_reopen`.

**Blocked on the completeness instrument (CF-27):**
`positions_are_not_reused_after_removal` (ES-38) and
`condition_over_removed_history_does_not_reject` (ES-40). Both are written against
a store that holds only a scattered subset of its own log, and CF-27 owns building
it.

**Blocked on a hostile fixture in the testkit's own `tests/`:**
`nothing_below_an_observed_position_appears_later` (the name VT-12 and CF-13 also
use),
`query_items_share_one_snapshot`. Both ship with the store that can fail them or
they do not ship at all — a rule that has never failed anything is what CLAUDE.md
forbids, and both of these would otherwise pass on every adapter in the workspace
on the day they were written.

**Blocked on a decision:** `store_error_crosses_a_join_handle`
(ES-6), `reissued_batch_after_a_dropped_future_lands_once` (ES-24),
`a_store_reports_the_history_it_does_not_hold` (ES-39),
`wire_condition_with_after_is_refused` (ES-29 and SY-6, and it lives in a crate
that does not exist).

Two existing rules are demoted rather than deleted. `positions_are_unique` and
`positions_are_strictly_monotonic` (`suite.rs:336-365`) both read back through
`read`, which every adapter returns in position order, so both are vacuous for any
sorted read path; the rules that actually pin cross-batch ordering are the
append-condition rules and ES-19's new one. And `memory.rs:328-340`'s
`read_stream_is_send` is replaced by ES-2's generic version, because the one on
disk passes by auto-trait leakage on a concrete type and cannot fail.

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
they have. Eighteen clauses below are frozen in that first sense. None of them
commits the published surface of 0.1, and reading them as if they did would be
reading a stronger claim than this section makes.

Four facts frame everything below.

**The port has zero implementers.** `grep -rn "ProjectionStore for" --include=*.rs`
returns nothing across the workspace; `crates/happenstance-sqlite/src/projection_store.rs:28`
is `pub struct SqliteProjectionStore {}` and
`crates/happenstance-ladybug/src/lib.rs:46` is an error enum with one
`Unimplemented` variant. Every claim the module documentation makes about
transactions is a claim about code that has never been written.

**The one attempt to write one hits `error[E0195]`.** Spelling the impl with the
concrete batch type — `async fn commit(&self, _batch: MyBatch<'_>, …)` — fails
against `projection.rs:109-114`; only the literal `Self::Batch<'_>` compiles
(compiled and reproduced, `docs/evaluation/PRESSURE-TEST.md` §3.5). Nothing in
the crate says so and there is no impl to copy. That, not the missing `apply`
seam, is the strongest available explanation for why the port has no adapters.

**The conformance suite cannot observe a read model.** `type Batch<'a>`
(`projection.rs:80-83`) carries no trait bounds, so generic code holding a
`P::Batch<'_>` can only hand it back to `commit` or `rollback`. The port exists
to defend read-model write and checkpoint write in one transaction
(`projection.rs:13-19`); a suite built on the port as written can test only the
second conjunct. By CLAUDE.md's own corollary — *"a rule that no adapter can
fail is decorative"* — such a suite is decorative in the exact place it matters,
because it cannot reject an adapter that commits the checkpoint and silently
drops the read-model write.

**All six scenarios hit the same two walls.** `docs/scenarios/README.md:1873-1880`
records them as agreements 1 and 2 of nine: the port cannot reset, and `Batch`
has no write vocabulary. Six deliberately dissimilar deployments converging is
the signal this section is built on.

### 4.0 Numbering and the shape being specified

Clauses in this section are `PS-n` and are numbered only within it. Conformance
rules named below are **all new** — `crates/happenstance-testkit/src/suite.rs`
contains 27 rules and every one of them binds `S: EventStore` (`suite.rs:70`
onward). There is no projection rule to reuse.

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
(`crates/happenstance-core/src/error.rs:150-165`) is shaped the way it is: a caller
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
fresh handles; both present or both absent, never one.
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

---

### 4.1a What is actually open

Seventeen `PS` clauses are `[PROVISIONAL]`, which reads as a section that could
not make up its mind. It is not: they are **six** questions, and the clauses
within each stand or fall together. A reader deciding whether to build against
this port needs the six, not the seventeen.

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
**Rule:** `compile_fail` doctest on the `Projection` trait, showing
`error[E0271]: type mismatch resolving <NetworkTopology as Projection>::Store == Pg`.
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

"The contract crate" throughout this document means the crate that is
`crates/happenstance` on disk today and that ADR-0006 renames to
`happenstance-core`. The rename is decided and unexecuted, and phase 0 owns it;
nothing here depends on which name it carries, only on the probe living beside the
port rather than in the testkit. File citations in this document use the paths the
tree has **today**, so that CF-38's checker can resolve them before the rename as
well as after it — phase 0 re-anchors them in the same commit that renames.

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

`commit` accepts a batch begun on a *different store of the same type*: the
elided lifetime in `batch: Self::Batch<'_>` (`projection.rs:109-114`) is a fresh
method-level parameter never tied to `&self`, so `b.commit(a.begin().await?, …)`
type-checks and a runner holding a `HashMap<DepotId, SqliteProjectionStore>` can
write one depot's inventory under another depot's checkpoint as a type-correct
program.

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
and what `conformance_test!`'s per-test re-evaluation of `$factory`
(`crates/happenstance-testkit/src/lib.rs:83-91`) already produces.
**Cases:** E2E-19.
**Rejects:** every adapter that can be written today, all of which corrupt
silently. The implementation is one word: `begin` stamps the batch with an
identity minted per store instance, and `commit` compares. With `Batch` owned
(PS-5) the stamp is a field, and the check is an integer comparison on a path
that is already doing I/O.

That the port carries a dedicated error variant rather than folding this into
the adapter's own `Self::Error` follows `AppendError`'s precedent exactly
(`error.rs:150-165`): a port-level outcome the caller must distinguish from an
adapter failure belongs in a port-level enum, because a generic caller — and the
suite is one — cannot name a variant inside an adapter's `#[non_exhaustive]`
error type.

---

### 4.6 Reset

All six scenarios. `checkpoint` returns an `Option` and `commit` takes a bare
`SequencePosition` backed by `NonZeroU64` (`event.rs:118-119`), so "never run"
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
checkpoint, then `reset` with a batch carrying `probe_delete_all`; assert both
gone. Paired with a failure-injecting variant asserting that a `reset` that
errors leaves both halves as they were.
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
`[PROVISIONAL — falsified if no adapter ever implements protection, in which
case the variant is dead weight and refusal belongs solely to the typed layer.
Evaluated at the exit of the projection-port phase by asking whether the SQLite
adapter implemented it.]`
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

**PS-19 — After a successful `reset`, `checkpoint(id)` MUST return
`Checkpoint::NeverRun`, and this MUST be distinguishable from
`commit(empty_batch, id, SequencePosition::FIRST, Live)`.**
`[FROZEN]`
**Rule:** `reset_is_not_commit_at_first` — perform both on two ids and assert
the checkpoints differ; then drive a replay from each and assert the event at
position 1 is applied in the first case and not in the second.
**Cases:** E2E-15, E2E-16.
**Rejects:** a runner that computes its resume point as
`checkpoint.unwrap_or(FIRST)` and reads `ReadOptions::from` that value —
`from` is inclusive (`query.rs:227`, `store.rs:116`) — which double-applies event
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
`ReadOptions::from` is inclusive while `AppendCondition::after` is exclusive
(`append.rs:103-106`), and the only API for advancing a position is
`SequencePosition::next()`, which is `saturating_add` under a doc promising
`None` on overflow (`event.rs:138-144`, defect D3). The runner MUST NOT be
written against `next()` until that is fixed, because at `u64::MAX` the
saturating version resumes at the position it just applied — an infinite reapply
loop in the one place nobody will test. VT-13 is the fix and freezes it, and it
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
"advances `id`'s checkpoint to `position`" (`projection.rs:102-114`), it would be
equally conformant today, and it makes a narrow projection re-scan the same
range forever on every restart —
Norvant's `cold_chain_certificate_expiry` matches about 40 of 37,000 events a
day. Two adapters can disagree and both pass, which is the sharper hazard than a
capability gap: it is a silent interoperability difference between two stores an
application might swap.

The checkpoint is therefore a high-water mark of **consideration**, not of
application. That is what makes it a resume point rather than a progress report,
and it is why PS-24 needs a separate signal for authority.

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
(`projection.rs:85-93`) permits and nothing else. On the Kestrel Cold Chain hub
the mid-rebuild read model is precisely what the next device's work slice is cut
from, so a checkpoint that reports "caught up to *N*" while holding half a graph
**manufactures the conflict the projection exists to prevent.** A rebuild that
has committed nothing reads as `NeverRun` rather than `Rebuilding`, which is
correct: both mean the rows are not authoritative, and the reader's decision is
the same.

**PS-25 — A checkpoint MUST NOT survive a change to the `Query` that produced
it. The `ProjectionId` a runner uses MUST be derived from the projection's name
and a digest of its `Query`.**
`[PROVISIONAL — falsified by a projection whose query is *narrowed*, where no
rebuild is needed and the derived id forces an expensive one anyway. If that
case is common, the digest moves into the checkpoint record as a separate
field and `commit` rejects a mismatch instead. Owned by the typed-layer phase.]`
**Rule:** `changed_query_starts_a_new_checkpoint` — commit under one query's
derived id, then read the checkpoint under a second query's derived id and
assert `NeverRun`. Contract-level only once `Query` has a canonical encoding;
until then it is a typed-layer rule.
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

`RUNBOOK.md:73` is open. The scenarios settle the shape of the answer even
though they disagree on the answer itself, and the disagreement *is* the result:
halting is correct for a revenue ledger — one that skips an event is worse than
one that stops — and wrong for an availability board, where a stale board is
worse than one missing a connector. A deliberate crypto-shred needs a fourth
option that is neither retry, halt nor dead-letter, because the decode failure
is permanent and *intended*.

**PS-26 — The failure policy MUST be declared per projection, not per runner.**
`[FROZEN]`
**Rule:** `failure_policy_is_per_projection` — two projections in one runner
declaring `Halt` and `SkipAndRecord`; feed both a failing event; assert the
first stops at that position and the second advances past it. Integration-level;
it needs a runner, so it belongs in the workspace e2e crate rather than the
adapter suite.
**Cases:** E2E-27.
**Rejects:** a runner-level `on_error: SkipPolicy` configuration. It is the
obvious design, it is what a builder API invites, and it forces one wrong answer
onto one of Wattline's two projections.

**PS-27 — The policy MUST offer skip-and-record, and the record MUST be written
into the same batch that advances the checkpoint past the poisoned position.**
`[PROVISIONAL — falsified if no projection ever writes a skip record, i.e. if
"record" turns out to mean "log a warning". Evaluated at the exit of the
typed-layer phase against the Kestrel Motor shred case.]`
**Rule:** `skip_and_record_is_atomic` — a projection whose `on_error` writes a
probe row; feed it a failing event; assert the probe row and the advanced
checkpoint are both present after a crash injected between them.
**Cases:** E2E-26, E2E-27.
**Rejects:** the implementation that swallows the skip. The skip *primitive*
already exists and nobody has noticed it: `begin()` followed immediately by
`commit(batch, id, poison_position, Live)` applies nothing and advances the
checkpoint atomically, with the port exactly as written. What does not exist is
any way to record that it happened — and in Kestrel Motor the skip is the
Article 17 evidence, so a swallowed skip is a compliance failure rather than a
missing log line. Routing the record through the projection's own batch means no
new port surface and no store-side knowledge of what a skip means.

**PS-28 — A failing `apply` MUST report the position it failed at, and MUST be
able to carry an application error type distinct from the projection store's.**
`[FROZEN]`
**Rule:** `pump_reports_the_failing_position` — integration-level; assert the
returned error names *P* and that `checkpoint` sits at the last good position.
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
**Rule:** `one_poisoned_projection_does_not_stall_the_others` — twenty
projections over one log, one failing; assert the other nineteen advance *and*
that the supervisor reports the failure without being polled for it.
Integration-level.
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
`[PROVISIONAL — falsified if the fan-out runner is not built, which is decided
by whether the poll cost of N independent reads is real. That is a benchmark,
not an assertion, and the workspace has no benchmark harness. Owned by the
typed-layer phase.]`
**Rule:** `panicking_apply_rolls_back` — integration-level; a projection that
panics; assert no partial rows survive and the checkpoint did not move.
**Cases:** E2E-28.
**Rejects:** a fan-out runner that wraps `&mut P::Batch` in `AssertUnwindSafe`
and carries on. The assertion is defensible *only* because `rollback` exists:
`AssertUnwindSafe` is a promise that no observer will see a half-mutated value,
and `rollback` is what discharges it. Without the rollback the promise is a lie,
and the workspace depends on two unwritten facts to make any of this work —
that the release profile does not set `panic = "abort"`, and that the cheap
runner (one read, one decode, twenty applies) and the safe runner (twenty
`tokio::spawn`s) are opposites the port adjudicates neither way.

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
`[FROZEN]`
**Rule:** none — this is a correction to a document, and the artefact that
proves it is the compiled pump recorded in PRESSURE-TEST §3.4.
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
`[DEFERRED — the experiment is the typed layer itself; the question is answered
by counting callers. Owned by the typed-layer phase (RUNBOOK phase 3;
revised-runway phase 6), whose exit criteria currently do not mention it.]`
**Rule:** none; it is a phase gate, not an adapter obligation, and no adapter can
fail it. Recorded as a clause because `0007:119-121` sets the falsifier, no phase
evaluates it, and an unevaluated falsifier is indistinguishable from none.
**Cases:** none directly; it decides where the code E2E-26 through E2E-28 test
lives.
**Rejects:** keeping a seam because it was argued for — which is ADR-0007's own
phrasing and its own risk. The fallback is already written down and is the ADR's
narrowly-rejected alternative: one runner, in `happenstance`, with the
checkpoint invariant living one crate above the port that states it.

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
trait declaration`, pointing at `projection.rs:109`, and nothing in the crate or
the workspace says the literal `Self::Batch<'_>` is required. `MemoryEventStore`
exists partly to be something an adapter author can copy (`memory.rs:16-23`);
the projection port has no equivalent, which is why `MemoryProjectionStore`
belongs in the same phase as the freeze rather than after it.

---

### 4.10 Derivation and the two flavours

`projection.rs:70` carries `#[trait_variant::make(SendProjectionStore: Send)]` —
the identical construction to `store.rs:91` — and appears in **no ADR at all**.
ADR-0001 argues the scheme for `EventStore` and never mentions this port. That
is the gap this subsection closes; the *value* of the bound is section 3's and
is not re-decided here.

**PS-35 — The derivation decision MUST cover both ports in one ADR. `EventStore`
and `ProjectionStore` MUST NOT be given different flavour schemes.**
`[DEFERRED — settled by the derivation ADR, the next free number after 0007;
PRESSURE-TEST §3.6 records that both prior documents allocate 0007 to it and
that 0007 is already accepted on disk for the runner split. The experiment is
one skeleton per adapter shape declaring its real future and stream types with
`todo!()` bodies, which is the only way to learn whether any real adapter
produces a `!Sync` future. Owned by the derivation phase (revised-runway phase
1).]`
**Rule:** none at adapter level; the ADR is the artefact.
**Cases:** E2E-30, E2E-52, E2E-53.
**Rejects:** settling `EventStore`'s bounds and leaving `projection.rs:70`
undiscussed, which is the present state and which produces a workspace where two
ports with identical constructions have one ADR between them. An application
holding a `!Send` projection store and a `Send` event store is not exotic —
Norvant's rusqlite and Ladybug batches sit beside Postgres ones — so the two
decisions compose at every call site and must be taken together.

**PS-36 — The `Send` flavour transitively requires `Batch: Send`, and the port
MUST document it rather than leaving it to be discovered.**
`[FROZEN]`
**Rule:** a `compile_fail` doctest showing the diagnostic, which is documentation
that CI checks.
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
blocking and wasm flavours without a second mechanism. Every one is new.

| Rule | Clause | Rejects |
|---|---|---|
| `fresh_projection_has_no_checkpoint` | PS-19 | a store that reports `Live { through: FIRST }` for an id it has never seen |
| `commit_advances_the_checkpoint` | PS-1 | — (the baseline the rest are differential against) |
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

Nothing in `happenstance-sync` is a trait. The crate is a module doc comment and
a one-variant error enum (`crates/happenstance-sync/src/lib.rs:103-112`), and its
own prose ends by admitting as much: *"None of this is settled"* (`:98-99`). This
section is what settles it.

It is written before the code exists, and the clauses that depend on measurement
rather than on argument are consequently `[DEFERRED]` against the phase that
builds the port against two real peers — five of the thirty-five, with a further
nine `[PROVISIONAL]`. The rest are frozen, because the shape does not wait on the
transport. They are written anyway, and the reason is recorded in the decision
ledger:
`docs/RUNBOOK.md:80` says deferring the sync port **"leaks `EventId` and a tail
seam back into `EventStore`"**. That is a claim about coupling, and the pressure
test confirms it survives — `PRESSURE-TEST.md:288-290` faults the revised runway
for dropping exactly that warning while resting two of its largest deferrals on
the coupling being absent. A deferral you have not written down is not a
deferral; it is a decision the next pass makes by accident, in the crate that can
least afford it. So the port's shape is specified here, and the *experiments* are
deferred — not the shape.

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
  store assigned it — per `docs/RUNBOOK.md:70`. VT-6 fixes what a `StoreId` names
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
`E2E-CASES.md:1516-1534` records them as mutually contradictory. They are not.
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
all-or-nothing by `store.rs:130-133`, conformance-tested by `append_is_atomic`
(`suite.rs:385-403`). That is an **append**. It adds a fact; it refuses nothing;
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

*Rejects:* the implementation `crates/happenstance-sync/src/lib.rs:78-83` names
first — "whether ingest re-checks conditions". Concretely: an ingest that calls
`append(events, Some(&origin_condition))` and routes `AppendError::ConditionViolated`
into a rejection path. It is the natural first cut, because
`AppendCondition` arrives on the wire already (`append.rs:117-123`) and the
receiving store's `append` will happily take it. It passes every event-store
conformance rule, because it is one correct `append` call. It produces a peer set
that never converges, and nothing in the workspace today can observe that.

---

**SY-2. Where the receiving peer's domain determines that an ingested event
conflicts with a fact it already holds, the compensation MUST be appended in the
same `append` call as the losing event.**

The losing event and its compensation are one batch:
`append(&[losing, compensation], Some(&guard))`. A reader MUST NOT be able to
observe a state in which the log holds the losing event with nothing resolving
it.

`[FROZEN]`
Rule: `compensation_is_atomic_with_the_losing_event` (new,
`happenstance-sync-testkit`), riding on the existing `append_is_atomic`
(`suite.rs:385-403`) for the store-side half.
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
(`docs/adr/0007-projection-runner-decodes.md:62-67`): the caller knows the
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
`ReadOptions::from` resumes at (`projection.rs:86-88`), so an event inserted
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

*Rejects:* the receiver that re-evaluates the origin's condition verbatim, which
is what Kestrel Cold Chain's D5 specified and what `crates/happenstance-sync/src/lib.rs:78-83` frames as
one of two options. Two independent defects, either fatal:

- `after` is a `SequencePosition`, meaningful only inside the store that assigned
  it (`crates/happenstance-sync/src/lib.rs:64-69`), and it serialises as a naked integer
  (`append.rs:117-123`). `is_violated_by` compares raw position values
  (`append.rs:95-107`), so `after: 288455` interpreted in the receiver's
  numbering names an unrelated recent event and the check runs over an arbitrary
  tail and passes **vacuously**. That is worse than no check, because it looks
  like enforcement.
- Even with `after: None`, re-evaluation is not idempotent. On first delivery the
  receiver accepts and writes; on re-delivery the same condition now matches the
  receiver's own copy, returns `ConditionViolated`, and the receiver adjudicates
  *against the event it just accepted* — an inversion, not a duplicate. This is
  `E2E-33`, the sharpest single case in the catalogue.

The refusal is mechanically available today: `AppendCondition` is
`#[non_exhaustive]` with **public** fields (`append.rs:51-61`), so a peer can
read `after` and reject on it while being unable to construct the struct
literally. Readable-but-not-literal-constructible is precisely what makes this
checkable, and VT-30 preserves it — `Guard` is `#[non_exhaustive]` with public
fields for this exact reason. WF-4 is the other half: `after` is **always
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

`crates/happenstance-sync/src/lib.rs:28-36` already states this and states it correctly. It is promoted
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
(`docs/adr/0007-projection-runner-decodes.md:44-50`), and for the same reason:
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
push. `crates/happenstance-sync/src/lib.rs:45-51` already says why and the scenarios confirm it: the hub
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

*Rejects:* the sync crate's own current proposal. `crates/happenstance-sync/src/lib.rs:73-75` suggests "a
UUIDv7 or a content hash in the event's **metadata**". Metadata is opaque
`Bytes` (`event.rs:187`, `:239`) and `QueryItem::matches` filters on type and
tags only (`query.rs:113-116`), so the identity a peer is instructed to carry is
structurally unreachable from anything the port exposes. A peer that must parse
opaque bytes to dedupe has broken ADR-0003 at the exact point ADR-0003 claims to
win — and ADR-0003's own lift condition is *"`happenstance-sync` round-trips an
event between two stores without deserialising its payload"*
(`docs/adr/0003-opaque-payloads.md:14-15`). The metadata proposal fails the ADR's
own test.

It also rejects the naïve fix, which is why this clause names `EventId` rather
than "some queryable identity". Promoting identity to a `Tag` makes it queryable
and costs more than it looks: a maximally high-cardinality entry in the column
adapters are told to index; an entry in every `contains_all` merge-scan
(`tag.rs:231-245`); a writer-forgeable identity; and an identity dimension
visible to every tag-only query in the domain. It also has a hole — a peer's own
locally-originated writes would carry no such tag, so the peer could not order
its own events against ingested ones. The mechanism only works if **the store
stamps its own writes at append time**, which is the store-assigned Lamport pair
`docs/RUNBOOK.md:70` already decided.

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
construction — `sort_unstable` then `dedup` (`tag.rs:259-265`) — and the
deserialiser **re-canonicalises on the way in** so a peer cannot smuggle a
non-canonical `Tags` into memory (`tag.rs:318-319`). Both are correct and both
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
  most one culprit (`error.rs:96-104`), so recovery is a serial peel.
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

`crates/happenstance-sync/src/lib.rs:91-96` calls this "the constraint most likely to be discovered
late". It is promoted from a footnote to a normative constraint on the port's
shape, because a port that a Neon peer cannot implement is a port shaped like a
Durable Object — and the workspace has already convicted itself of exactly that
error once, at the level of the projection store.

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
> spells its batch `type Batch<'a> where Self: 'a` (`projection.rs:80-83`);
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
`tokio::spawn` a per-peer task. `crates/happenstance-sync/src/lib.rs:87-90` states the wasm32
requirement, but understates it: in Kestrel Cold Chain the `!Send` peer — SQLite
inside a Cloudflare Durable Object — sits in the **middle** of the chain, not at
a leaf. It is a spoke to the Neon estate store and a hub to 138 tablets. A runner
bound on the `Send` flavour therefore excludes the hub from its own topology, and
the exclusion is discovered when the adapter is written, not when the runner is.
`EventStore` is the weaker requirement and accepts both flavours
(`store.rs:16-19`).

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

*Rejects:* the current situation, which is that there is nowhere to declare a
capability at all — `crates/happenstance-sync/src/lib.rs:103-112` is the entire crate. The contract
bounds the two fields no engine struggles with, `MAX_EVENT_TYPE_LEN` and
`MAX_TAG_LEN`, both 255 (`event.rs:14`, `tag.rs:14`), and leaves `Event::data`
unbounded. So an event that is durable at its origin can be structurally
unrepresentable at a peer, and the incompatibility is discovered at ingest —
after the write has already committed somewhere else, which is the one moment at
which nothing useful can be done about it.

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
(`event.rs:92-97`) extends across a peer set. Kestrel Rotor's failure is exactly
this belief holding: the same fourteen events sit at vessel position 38,102 and
depot position 3,918,442, each store internally correct, each peer's total order
naming the other as the loser. There is no expression relating the two numbers
and none can be constructed, because `SequencePosition` carries no origin
(`crates/happenstance-sync/src/lib.rs:64-69`).

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
`crates/happenstance-sync/src/lib.rs:84-86` states the premise — merging two independently-ordered logs
means accepting that a replicated event's local position differs from its origin
position — and stops there. What it does not say is that the acceptance has a
price, and that the price is paid by every fold downstream.

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
(`docs/adr/0007-projection-runner-decodes.md:76-81`) — a value carrying exactly
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
answering it. `SequencedEvent` is `{position, event}` and nothing else on disk
today (`event.rs:277-282`), so "which side of midnight did this fall on" is
currently unanswerable from the log, and every timestamp in all six scenarios is
a writer's clock in an opaque payload — including one drifted 2.4 seconds and one
six minutes fast after a factory reset. **A tiebreak is not a fact about the
world.** The audit answer is a separate decision and section 2 has taken it:
VT-9 adds a store-assigned `RecordedAt`, stamped at local append and preserved
verbatim under ingest, and it is taken in the same pass as `EventId` because both
change `SequencedEvent::new` — a two-argument `const fn` every adapter and the
testkit call (`event.rs:286`). VT-9 also forbids ordering by it, which is what
keeps that decision and this one from colliding: `RecordedAt` answers the audit
question and MUST NOT be a merge rule, and `EventId` order is the merge rule and
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
tags, metadata (`event.rs:183-188`) — and none of them is a parent. The
`AppendCondition` that encoded what the author had looked at is taken as a
parameter, evaluated, and dropped (`store.rs:141-145`); it is not persisted by
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
`SequencedEvent` (which is `#[non_exhaustive]`, `event.rs:276`) but not on
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
(types and tags only, `query.rs:103-110`) can be shown to admit an unbounded scan
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
(`error.rs:62-67`) — and `QueryItem` and `Tags` re-validate on deserialisation
(`query.rs:297-302`, `tag.rs:318-319`), so non-canonical tags and fully
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
exactly one `Option<&AppendCondition>` for the whole slice (`store.rs:141-145`),
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
type-checks against `projection.rs:109-114` today. What genuinely does not work
is a *generic* sync runner writing rows into a *generic* `Batch`, because
`type Batch<'a>` carries no trait bounds (`projection.rs:80-83`) — which is the
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
demonstration (`docs/adr/0003-opaque-payloads.md:6-15`).

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
> (`tag.rs:39`) with `key()`/`value()` offered as convention over
> `split_once(':')` (`tag.rs:88-98`), and a tag with no colon is legal. The
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
the *stream* `Send` (`store.rs:103-108`, CLAUDE.md constraint 3) — and that is the
right shape for a store, where laziness buys a million-event replay without
buffering. It is the wrong shape here: a stream is a cursor, a cursor is state
held between polls, and SY-15 forbids it. The peer returns a bounded batch and a
token, and the runner loops. That is one round trip per call by construction
rather than by discipline.

**`Push` holds `SequencedEvent`, not `Event`.** The `EventId` must travel (SY-12)
and `Event` has no field for it (`event.rs:183-188`). This is the clause that
makes `EventId`'s placement on `SequencedEvent` load-bearing outside the store —
and it is the concrete form of `RUNBOOK.md:80`'s warning that deferring this port
leaks `EventId` back into `EventStore`. The leak is real, it is here, and naming
it is cheaper than discovering it.

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
- **`Error: Send + Sync` on either flavour.** Ranked first by blast radius in
  `PRESSURE-TEST.md:481-492` and open on evidence. It reaches this port through
  `type Error` in the sketch above and is settled for the whole workspace at once
  or not at all.

---

## 6. The conformance obligation

The suite is the project's claim to exist. `happenstance-testkit`'s own module
documentation says an adapter "is not considered to exist until it invokes
`event_store_conformance!` and passes"
(`crates/happenstance-testkit/src/lib.rs:1-6`), and CLAUDE.md repeats it as the
rule that matters. Everything downstream of that sentence — every adapter, every
freeze, the whole "storage agnostic" claim — is worth exactly what the suite
measures.

Measured, it is worth less than it says. Twenty-seven rules
(`lib.rs:93-129`), and a store that ignores tags entirely when evaluating an
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

CLAUDE.md's corollary — *a rule that no adapter can fail is decorative* — is
currently enforced by intention. The testkit's `tests/` holds two files:
`memory_conformance.rs`, which runs the suite against the reference store, and
`properties.rs`, which proptests the contract's value types. Neither can fail a
rule. There is no wrong implementation anywhere in the workspace, so the standing
rule has never been checked against a single rule it governs.

The fix is to make the mutant a first-class artefact of the suite, registered as
data, and to let a meta-test rather than a reviewer decide whether the obligation
was met.

**CF-1.** Every conformance rule MUST be paired with at least one *mutant store*
in `happenstance-testkit`'s own `tests/`, which fails that rule. A rule
introduced without one is a defect in the suite and MUST NOT be merged.
`[FROZEN]`
Rule: `mutation_coverage::every_rule_has_a_mutant` (new meta-test).
Cases: all contract-level cases; the obligation is stated at E2E-CASES.md:8-15.
Rejects: `positions_are_unique` and `positions_are_strictly_monotonic`
(`suite.rs:336-365`) as they stand. Both read a quiescent store back through
`read`, which every adapter returns in position order, so both are satisfied by
sorting on the way out. No store in the workspace can fail either, and neither
has ever been paired with one that could.

**CF-2.** Mutants MUST be registered as data — a `const` table naming, per
mutant, the exact set of rules it fails — and MUST NOT be exercised only by
hand-written per-mutant tests. `[FROZEN]`
Rule: `mutation_coverage::mutant_registry_is_exhaustive` (new meta-test).
Cases: all contract-level cases.
Rejects: the natural cheap version — a `tests/wrong_stores.rs` that invokes
`event_store_conformance!` against each mutant with `#[should_panic]`. That
records only *that* something failed, so a mutant which fails the right rule for
the wrong reason (a panic in its constructor, an unrelated regression) reads as
proof. The registry exists so the meta-test can assert the *set*, not the count.

**CF-3.** The meta-test MUST assert both directions: every registered mutant
fails every rule it declares, **and** passes every rule it does not. `[FROZEN]`
Rule: `mutation_coverage::mutants_fail_exactly_their_declared_rules` (new
meta-test).
Cases: all contract-level cases.
Rejects: a mutant that is broken in more ways than it claims — the commonest way
a mutant set decays. A store that drops tags from its condition probe *and*
returns events out of order proves the tag rule catches something, but not that
it catches the tag defect. The second assertion is what keeps a mutant a
scalpel.

**CF-4.** Every mutant MUST carry a non-empty provenance string naming the real
adapter shape or scenario that makes it plausible. `[FROZEN]`
Rule: `mutation_coverage::every_mutant_states_its_provenance` (new meta-test).
Cases: E2E-01, E2E-08, E2E-32, E2E-55.
Rejects: the saboteur — `struct AlwaysWrong; impl EventStore for AlwaysWrong { … }`
— which satisfies CF-1 mechanically and proves nothing, because no author would
have written it. The mutant that earns its place is the one someone would ship:
dropping the tag join because tags live in a second table
(`crates/happenstance-sqlite/src/event_store.rs:21-27`), caching `max(position)` per
session, evaluating each `QueryItem` as its own statement.

**CF-5.** The testkit MUST also hold at least one *conformant variant* — a store
that is legally different from `MemoryEventStore` and MUST pass every rule.
`[FROZEN]`
Rule: `mutation_coverage::conformant_variants_pass_everything` (new meta-test).
Cases: E2E-10.
Rejects: an over-specified rule. The specification permits gaps, and a store
assigning positions in steps of seven, or starting at 4,096, is conformant; a
rule that quietly assumes density passes against the reference store and fails
that adapter in the field. Mutants catch under-specification. Conformant variants
catch over-specification, and nothing in the workspace catches it today.

**CF-6.** No rule may assert on a literal sequence-position value. Every position
assertion MUST be anchored on a value the store under test assigned. `[FROZEN]`
Rule: `mutation_coverage::conformant_variants_pass_everything` (CF-5's gapped
variant is the enforcement).
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
Rule: `condition_matches_on_tags` (new; ES-27 specifies it and names it, and this
clause is the obligation to have it).
Cases: E2E-55, E2E-56; scenario S3 in PRESSURE-TEST.md:590-596.
Rejects: an adapter that drops the tag join from its condition probe and keeps it
only in `read`. Every condition call site in the suite builds its query from
`query_of_types` (`fixtures.rs:64-67`), and the single tagged condition —
`racing_conditional_appends_elect_one_winner` at `suite.rs:603-605` — runs
against a store holding one untagged `CourseDefined` (`:600`) and one tagged
`StudentSubscribed` (`:607`), so a type-only probe returns the identical verdict
at `:612` and `:621`. The tag join is the expensive half and lives in a separate
table in the planned SQLite adapter
(`crates/happenstance-sqlite/src/event_store.rs:21-27`), which makes dropping it the
natural first cut. Such an adapter rejects every command touching any course, and
passes all twenty-seven rules while doing it: a total-availability failure
certified as conformant, on the canonical DCB uniqueness shape.

**CF-8.** At least one rule MUST demonstrate the mirror: a condition carrying a
tag no stored event carries MUST NOT reject the append, even when a stored event
matches the condition's types. `[FROZEN]`
Rule: `condition_ignores_events_whose_tags_differ` (new).
Cases: E2E-55, E2E-56.
Rejects: the over-rejecting adapter — one that treats a condition's tags as
advisory and rejects on type alone. CF-7 alone catches only the fail-open
direction; an adapter tuned to be "safe" by ignoring tags in the narrowing
direction is equally non-conformant and equally invisible today. The two clauses
are a pair and neither is sufficient.

**CF-9.** At least one rule MUST read `Query::all()` against a store holding an
event carrying several tags, and assert each event is yielded exactly once.
`[FROZEN]`
Rule: `query_all_yields_each_event_once` (new).
Cases: E2E-32.
Rejects: an adapter whose tag storage is a row-per-tag side table joined without
`DISTINCT`. `query_all_matches_every_event` (`suite.rs:70-80`) appends three
untagged events, so the fan-out cannot occur; `query_item_tags_match_supersets`
(`:123-145`) stores a three-tag event but reads it through a tag-constrained
query, which an adapter may satisfy with a different code path. The duplicate is
invisible in both.

**CF-10.** At least one rule MUST evaluate an append condition against an
**empty** store and assert the append succeeds. `[FROZEN]`
Rule: `condition_against_empty_store_allows_append` (new).
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

**CF-11.** At least one rule MUST exercise the empty batch against a **non-null**
condition, and assert the precedence ES-20 fixes: `AppendError::NoEvents`, with
the emptiness check preceding the condition check. `[FROZEN]`
Rule: `empty_batch_is_refused_before_the_condition_is_evaluated` (new; ES-20
specifies it and names it).
Cases: E2E-06 (adjacent; the batch's own events against its own condition).
Rejects: `MemoryEventStore` as it stands, which is the sharpest possible answer
to "can this rule fail anything". `append_rejects_empty_batch` only ever calls
`append(&[], None)` (`suite.rs:408`), while `MemoryEventStore` evaluates the
condition first and returns `ConditionViolated` for `append(&[], Some(&matching))`
(`memory.rs:197-216`), reporting a concurrency signal for what is unambiguously
the caller's own bug — and putting a correct client into a retry loop that can
never terminate. Two adapters can disagree here and both pass today; that
interoperability hazard is what the clause closes, and ES-20 is where the
direction is argued.

This clause was drafted `[PROVISIONAL]` against a precedence "section 2 settles".
Section 2 does not own it — the empty-batch/condition precedence is an `append`
semantic and §3 owns `append` — and ES-20 settles it `[FROZEN]`. The rule's
assertion is therefore writable now.

**CF-12.** At least one rule MUST compose `ReadOptions::from` with a
**multi-item** query. `[FROZEN]`
Rule: `read_from_composes_with_multi_item_query` (new).
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

**CF-13.** A rule MUST assert the visibility invariant — that once a reader has
observed position *P*, no event at a position at or below *P* becomes visible
afterwards — and it MUST be accompanied by a fixture that fails it.
`[DEFERRED — the experiment is a deterministic hostile fixture in the testkit's
own tests/: a store that withholds one append's row until a second,
later-positioned append has committed, driven from a single-threaded harness.
Owned by the instrument-portfolio pass, the same pass that builds
`happenstance-postgres` as an instrument. If the fixture cannot be made
deterministic without a `Send + Sync` sub-trait to bind parallel rules on, that
sub-trait becomes the deliverable instead, and this clause is re-derived against
it.]`
Rule: `positions_are_visible_in_assignment_order` (new).
Cases: E2E-01, E2E-02.
Rejects: a Postgres adapter allocating positions with `nextval()` outside the
transaction. Both existing position rules read a quiescent store after a single
sequential writer (`suite.rs:336-365`), so they measure assignment and say
nothing about visibility — the property Wattline's nineteen-millisecond window
breaks and the property `AppendCondition::after` is sound only under. Shipping
this rule without the fixture violates CF-1: it would be a rule that has never
failed anything, which is precisely what E2E-CASES.md:1555-1561 warns against.

**CF-14.** A rule MUST assert that an append acknowledged with `Ok` survives a
restart of the store's process-level state. `[DEFERRED — blocked on the fixture
contract CF-17; owned by the same testkit-hardening pass. The experiment is
whether a `restart` capability can be honoured by rusqlite, a Durable Object and
a one-shot HTTP client with one shape, or whether "durable" needs to be graded.]`
Rule: `acknowledged_appends_survive_restart` (new).
Cases: E2E-07.
Rejects: an adapter that acknowledges before `COMMIT` — a pooled rusqlite store
doing its work in `spawn_blocking` and returning on the join, a Durable Object
relying on output-gate semantics it does not actually have, any store with
`PRAGMA synchronous = OFF` in its connection setup. Nothing in the workspace can
fail such a store today, because `conformance_test!` hands each rule one factory
call (`lib.rs:83-91`) and there is no second call whose results would be
meaningful.

---

### 6.3 The fixture contract

`conformance_test!` expands to `$crate::rules::$name(|| $factory).await`
(`lib.rs:87`), re-evaluating `$factory` per test, and each rule calls it exactly
once. So `F: Fn() -> S` is doing two incompatible jobs at once and has been asked
to promise neither: the macro's documentation says the expression must build "a
**fresh, empty** store" (`lib.rs:17-21`), while the *signature* would accept
`|| store.clone()` — two handles onto one backing store — and nothing in the
suite either requires or forbids it. A rule therefore cannot call the factory
twice, because it cannot know whether it gets isolation or sharing. That
ambiguity is what forecloses CF-14, CF-19 and every genuine multi-connection
rule, and it is a type problem before it is a coverage problem.

The remedy is to name the two operations separately, which means a trait rather
than a closure. A closure has one call signature and no place to hang an
associated store type, a capability declaration, or a second constructor; a trait
has all three, and in Rust the capability declaration can be an associated
`const`, which makes it available to the macro at expansion time rather than at
run time.

**CF-15.** The fixture MUST be a trait, not a bare `Fn() -> S`. Each *fixture
instance* is one isolated backing store; each `connect()` on that instance
returns a handle onto it. Two fixture instances MUST share nothing. `[FROZEN]`
Rule: `fixture_isolation` (new meta-test; two fixture instances, an append to
one, an empty read from the other).
Cases: E2E-08, E2E-09.
Rejects: the file-backed adapter that points every fixture at one temp path — the
mistake `lib.rs:19-21` warns about in prose and nothing detects. Under the
current shape it produces cross-test contamination that surfaces as an unrelated
rule failing intermittently; under CF-15 it fails one named meta-test.

**CF-16.** The fixture MUST be able to open a **second handle** onto the same
backing store, and rules MAY require it. `[FROZEN]`
Rule: `two_handles_observe_each_others_appends` (new).
Cases: E2E-08.
Rejects: an adapter whose correctness is per-session — a cached `max(position)`
fast path, a per-connection repeatable-read snapshot, an advisory lock scoped to
one pool member. All three pass all twenty-seven rules today, and all three are
strategies the decision ledger defers rather than rules out.

**CF-17.** The fixture SHOULD be able to **restart**: invalidate every
outstanding handle's process-level state such that a subsequent `connect()`
observes only what was durably committed.
`[PROVISIONAL — falsified by a legitimate adapter that is durable and cannot
express restart through this contract. A Durable Object, whose storage outlives
the isolate but whose isolate cannot be restarted from inside a test, is the
candidate; if it forces a second shape, `restart` splits into "reopen the handle"
and "restart the host" and this clause is rewritten.]`
Rule: `acknowledged_appends_survive_restart` (CF-14).
Cases: E2E-07.
Rejects: nothing on its own — it is an enabling clause, and CF-14 carries the
rejection. It is `SHOULD` rather than `MUST` because `MemoryEventStore` is
legitimately volatile and must stay a first-class fixture; CF-18 is what stops
that from becoming an excuse.

**CF-18.** A fixture MUST declare its capabilities as associated `const`s, and a
rule whose capability requirement is unmet MUST still be emitted as a test that
**reports** the skip with the fixture's stated reason. A rule MUST NOT be
silently omitted. `[FROZEN]`
Rule: `capability_skips_are_reported` (new meta-test: a fixture declaring no
capabilities must produce the full rule count, with the capability-gated ones
reported as skipped).
Cases: E2E-07, E2E-08.
Rejects: `#[cfg]`-ing capability-gated rules out of the expansion. A rule that
does not appear in the test binary is indistinguishable in CI output from a rule
that passed, so an adapter author who declares `RESTART: false` to make a red
build green gets a green build and no record of the trade. Requiring a non-empty
reason string alongside each `false` puts the trade in the log where a reviewer
and a user of the adapter can both see it.

**CF-19.** A rule MUST assert that an append made through one handle is visible
to an append condition evaluated through a second handle onto the same backing
store. `[FROZEN]`
Rule: `two_handles_observe_each_others_appends` (new).
Cases: E2E-08.
Rejects: the same three session-scoped strategies as CF-16, from the append side
rather than the read side.
`racing_conditional_appends_elect_one_winner` (`suite.rs:596-638`) is the rule
this looks like and is not: it is sequential *and* single-handle, so it pins the
semantics of a race without ever running one.

**CF-20.** The fixture trait MUST be defined without a `Send` bound and MUST NOT
be `trait_variant`-derived. `[FROZEN]`
Rule: the wasm32 step of `cargo xtask ci` (`xtask/src/main.rs:60-77`), extended
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
Rejects: the current arrangement, which makes the testkit's own claim false.
`crates/happenstance-testkit/tests/properties.rs:7-10` says the laws live in the testkit "because they are
the same claims an adapter must satisfy — an adapter that pushes query matching
down into SQL ... should be able to reuse the generators", and the generators are
private functions in an integration-test binary (`properties.rs:19-29`),
reachable by nothing. An adapter author writing their own will not reproduce
`properties.rs:17-18`'s five-symbol alphabet, chosen so that collisions and
duplicates actually occur, and their property tests will therefore never generate
the inputs that break a merge-scan at its boundaries.

---

### 6.4 Runtime independence

`conformance_test!` emits `#[tokio::test]` (`lib.rs:85`), and the crate
documentation tells adapter authors to add tokio to their dev-dependencies
(`lib.rs:23-24`). That is a hard exclusion of the target the two-flavour port
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
Rule: `registry::every_rule_is_enumerated` (CF-24's meta-test is the
enforcement).
Cases: E2E-52, E2E-30, E2E-09.
Rejects: the current arrangement, in which the rule bodies live in `suite.rs` and
a hand-maintained list of twenty-seven `conformance_test!` invocations lives in
`lib.rs:93-129`. The two agree today. Nothing makes them agree.

**CF-23.** The testkit MUST NOT emit any runtime-specific attribute from its own
expansion. The per-test wrapper MUST be a parameter supplied by the adapter.
`[FROZEN]`
Rule: the wasm32 build step of `cargo xtask ci`, extended to compile a
`wasm-bindgen-test` harness over the same rule set.
Cases: E2E-52, E2E-30.
Rejects: `#[tokio::test]` at `lib.rs:85`, and equally a "fix" that swaps it for a
`cfg`-selected attribute inside the testkit — that keeps the list of supported
runtimes in the testkit's source, so a runtime nobody anticipated (a `LocalSet`
harness, `futures::executor::block_on`, an embedded executor) needs a testkit
release to become usable. Three harnesses MUST be demonstrated in-tree: tokio,
a blocking single-threaded one, and `wasm-bindgen-test`. Two would let a
one-off accident pass for a design.

**CF-24.** No rule may exist in the suite without appearing in the enumeration.
`[FROZEN]`
Rule: `registry::no_orphan_rules` (new meta-test, comparing the enumeration
against the public items of `rules`).
Cases: all contract-level cases.
Rejects: the silent no-op — a rule written, reviewed, merged, and never run
because its registration line was forgotten. Under the present two-list shape
this failure produces no signal of any kind: the rule compiles, `cargo test`
reports green, and the adapter is certified against twenty-six rules while its
author believes it was twenty-seven.

---

### 6.5 The instrument portfolio

CLAUDE.md's standing rule: *a port is only as well-designed as the spread of what
implements it*, and before freezing a port you must name the axis it is most
likely to be wrong about and check that something in the workspace sits at the
other end. The rule is stated. It has never been executed, because the workspace
holds one storage shape wearing several hats.

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
it. `[FROZEN]`
Rule: `cargo xtask spec-trace` (CF-38), which reads the portfolio table and the
maturity markers and fails when a `[FROZEN]` port clause has an axis with no
far-end row and no named risk acceptance.
Cases: E2E-01, E2E-02, E2E-24, E2E-46, E2E-52.
Rejects: freezing `EventStore` against `MemoryEventStore`, a `RefCell` store,
rusqlite and a Durable Object — four adapters, one storage shape, every one of
them serialising its writers and assigning positions under a lock it holds until
commit (`memory.rs:195-223`). A port frozen against that population is frozen
against SQLite wearing four hats, and the property it will be wrong about is the
one all four share.

**CF-26.** A fixture instrument satisfies CF-1 and CF-25's falsifiability half.
It does not satisfy CF-25's implementability half; that requires an adapter
instrument. `[FROZEN]`
Rule: the portfolio table's `Far end exists` column, checked by
`cargo xtask spec-trace` (CF-38).
Cases: E2E-01, E2E-24.
Rejects: declaring the position-allocation axis covered because the hostile
visibility fixture of CF-13 exists. The fixture proves the rule bites. Whether a
real Postgres adapter can *pass* it — and at what cost among `xid8` +
`pg_snapshot_xmin`, transaction-scoped advisory locks and a serialised sequence
table — is a measurement, and PRESSURE-TEST.md:662-666 is right that it is owed
one.

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
deletes (`store.rs:117-145`) — and afterwards the store passes all twenty-seven
rules unchanged, including `query_all_matches_every_event` (`suite.rs:70-80`),
whose contract is store-relative by wording and therefore accidentally correct. A
holed log and a young log are the same value. Four of the six scenarios reach
this from unrelated doors — a pruned device slice, a regulated scattered purge, a
compacted peer, an epoch count derived from a slice — which is the strongest
available evidence that it belongs in the port rather than in an adapter, and
CLAUDE.md names an instrument for the two axes it had already noticed and nothing
for this one.

**CF-28.** The workspace MUST hold a `!Send` reference store in the testkit's own
`tests/`, and it MUST pass the suite. `[FROZEN]`
Rule: the existing suite, invoked against a `RefCell`-backed store under a
single-threaded harness (CF-23).
Cases: E2E-52, E2E-09, E2E-30.
Rejects: ADR-0001's own provisional status. That ADR states plainly that **no
`!Send` implementation of these ports exists anywhere, not even a reference one**
(`docs/adr/0001-async-port-flavours.md:8-16`), and names this exact store as the
cheapest proof that lifts it. Until it exists, the two-flavour design's entire
evidence base is a `cargo check` for `wasm32` — a compile of the trait, not of an
implementation. The store also carries E2E-09's re-entrancy question, which
`MemoryEventStore` cannot: `memory.rs:184-224` holds no lock across a suspension
point because it contains no `.await` at all.

#### The portfolio

| Axis | Near end — what exists | Far end | Far end exists? | Kind of instrument needed |
|---|---|---|---|---|
| **Position allocation** | Assigned under the lock held until commit — `memory.rs:195-223`, and every planned adapter | Allocated outside the transaction; visibility order ≠ position order (`nextval()`) | **No.** `happenstance-postgres` planned, unbuilt | Both. Fixture (CF-13) to make the rule bite; adapter to prove it passable |
| **Transport** | In-process, a handle held across awaits — `MemoryEventStore`, rusqlite | One-shot HTTP: no connection, no interactive transaction, no cursor | **No.** `happenstance-neon` planned, unbuilt | Adapter |
| **Async flavour** | `Send` — `impl SendEventStore for MemoryEventStore` (`memory.rs:147`) | `!Send`: `Rc`-shared, single-threaded, futures that are not `Send` | **No.** Nothing, not even a reference store (ADR-0001:8-16) | Fixture (CF-28), then the Cloudflare adapter |
| **Batch shape** (`ProjectionStore`) | A live transaction held across awaits — the shape the port was designed against | A deferred write set buffered and replayed in one call at commit | **No — and neither does the near end.** `ProjectionStore for` matches nothing in the workspace | Both |
| **Completeness** | A store holding its whole log — everything, everywhere | A store holding only a suffix, or a log with a scattered hole | **No, and nothing is planned.** New (CF-27) | Fixture first; a device adapter second |
| **Handle multiplicity** | One handle per store — `conformance_test!` calls the factory once (`lib.rs:83-91`) | Two or more handles onto one backing store, concurrent | **No.** Foreclosed by the fixture's type until CF-16 | Fixture |
| **Durability** | Volatile — `MemoryEventStore` is a `Vec` behind an `RwLock` | Survives a restart of the process | **No.** Not expressible until CF-17 | Fixture, then any file-backed adapter |

Seven axes, seven empty far ends. That is the honest state, and it is the reason
this section exists before the freeze rather than after it.

---

### 6.6 Compatibility policy for the testkit

Adding a conformance rule is a semver-minor change to `happenstance-testkit` that
turns every passing adapter's CI red. That is not a bug — a new rule usually
means a defect was found, and the red build is the point — but it is a policy
decision the crate has never made, and the crate is the moat.

**CF-29.** A conformance rule MAY be added in a minor release, and MUST land in
the same release as its mutant (CF-1) and its changelog entry naming the defect
it detects. `[FROZEN]`
Rule: `mutation_coverage::every_rule_has_a_mutant` (CF-1) plus a changelog check
in `cargo xtask ci`.
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
Rule: a `cargo xtask ci` manifest check.
Cases: none — this is a packaging obligation with a machine check and no
behavioural case, which CF-35 permits so long as the clause says so. It does.
Rejects: the current manifest (`crates/happenstance-testkit/Cargo.toml:4`,
inheriting `version = "0.1.0"` from the workspace root at `Cargo.toml:6`). Under a
shared version key the two crates cannot move independently in either direction,
and both directions are wrong. Adding a rule bumps the testkit's minor, which
drags `happenstance` to the same number and republishes an unchanged contract —
after which a semver-checking tool has no earlier version of that contract to
diff against. A patch release of the contract republishes the testkit and forces
every adapter to re-run a bar that did not change. The version key is a statement
about what a number means, and these two crates' numbers mean different things:
the contract's is a promise about types, the testkit's is a promise about the
bar.

---

### 6.7 Benchmarks are not conformance

An adapter that scans where it should seek passes every rule that can be written.
Norvant's `SuspendLane` decision is the worked case: a mixed tagged/untagged
two-item query over a log where one item selects a handful of events and the
other selects millions. `MemoryEventStore` evaluates `query.matches` over every
event before applying `from` (`memory.rs:159-174`) and says outright that it is
not built for scale — and no conformance rule can catch that, because complexity
is a benchmark and not an assertion.

**CF-33.** No conformance rule may read a clock, measure elapsed time, or assert
on an operation count. `[FROZEN]`
Rule: a `cargo xtask ci` lint step over `happenstance-testkit/src`, rejecting
`std::time`, `Instant`, `elapsed` and `sleep`. It is a grep, not a type — there
is no lint that expresses "this crate may not observe time" — and saying so is
better than pretending otherwise.
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
Cases: E2E-CASES.md:1589-1593 records this as one of the two things that are
neither blocked nor cases.
Rejects: a benchmark result gating a merge. A threshold nobody can justify
becomes a threshold everybody raises, and the number stops meaning anything the
second time it is moved. Benchmarks are published per adapter and compared
against that adapter's own history; they decide nothing about conformance.

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
`crates/happenstance-sync/src/lib.rs:9-11` and not yet existing.

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
forever on rules the specification deliberately leaves unclaimed:
`query_all_matches_every_event` and `racing_conditional_appends_elect_one_winner`
are orphaned **because they are being retired**, and §7.4 says so in prose the
checker cannot read. So a clause MAY dispose of a rule it does not claim, in the
form `Retires: <rule> — <reason>`, and a disposed rule satisfies the check. An
orphan with no disposition still fails. The distinction is the whole point: an
unclaimed rule is either a decision someone made or a rule nobody is responsible
for deleting, and only the author knows which.

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
obligation. Section 2 therefore names tests in `crates/happenstance/tests/wire.rs`
where every other section names conformance rules, and **CF-38's traceability
check must resolve those names too**: a `WF-n` clause whose named test does not
exist is the same defect as an `ES-n` clause whose named rule does not, and a
checker that only reads `happenstance-testkit` would report the whole wire format
as untraceable. If the format is ever opened to other DCB implementations, that
decision brings a `happenstance-wire-testkit` with it, and its meta-rules are
CF-1 through CF-6 unchanged.

**The projection suite's shape.** Section 4 owns it, and has decided. The
conformance consequence is the whole of the problem and is worth restating in
this section's terms: three of the six projection rules the roadmap specifies —
rollback leaves both unchanged, a dropped batch leaves both unchanged, a failed
commit leaves the store unchanged — cannot observe the read model at all, because
generic suite code holding a `P::Batch<'_>` can only pass it to `commit` or
`rollback` (`projection.rs:80-83`). By CF-1 those three are decorative until
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

**It was computed by hand.** `cargo xtask spec-trace` does not exist — CF-38
specifies it and nothing has built it — so this table is a snapshot taken by
parsing the six section files at commit `2a65d76`, not a checked invariant. The
first thing that pass builds should be the checker, and the first thing the
checker should reproduce is this table. If it disagrees with this table, the
checker is right and this table has already decayed, which is the whole argument
for having one.

Two reading conventions. A rule name marked **†** does not exist yet: it is
specified in this document and must be written. Where a clause names more than
three rules the cell is truncated with `…` and the clause is authoritative. The
maturity column carries the marker only; the falsifier or experiment that
accompanies a `[PROVISIONAL]` or `[DEFERRED]` marker is in the clause, and a
marker with an empty one is a build failure under CF-38.

### 7.1 Summary

| Section | Prefix | Clauses | `[FROZEN]` | `[PROVISIONAL]` | `[DEFERRED]` | `[NON-NORMATIVE]` |
|---|---|---|---|---|---|---|
| §2.1–§2.6 value types | `VT` | 31 | 22 | 9 | 0 | 0 |
| §2.7 wire format | `WF` | 12 | 10 | 1 | 1 | 0 |
| §3 `EventStore` | `ES` | 40 | 34 | 3 | 3 | 0 |
| §4 `ProjectionStore` | `PS` | 37 | 18 | 17 | 2 | 0 |
| §5 `SyncPeer` | `SY` | 35 | 21 | 9 | 5 | 0 |
| §6 conformance | `CF` | 38 | 32 | 2 | 3 | 1 |
| **Total** | | **193** | **137** | **41** | **14** | **1** |

The shape of that table is the specification's own self-assessment, and it says
what §1.6 says in words. `ES` is 85 % frozen because it has twenty-seven rules, a
reference implementation and six scenarios behind it. `PS` is 46 % frozen because
it has no implementation at all, and its provisional clauses are provisional
against the same missing adapter rather than against seventeen different ones —
PS-2 is the single gate they all wait on. `SY` sits between them because its
*shape* does not wait on a transport but its *measurements* do.

### 7.2 The table

#### `VT` — value types (§2.1–§2.6)

| Clause | Maturity | Conformance rule — † = does not exist yet | Cases |
|---|---|---|---|
| VT-1 | FROZEN | `append_preserves_event_payload`, `append_preserves_event_type_and_tags_byte_for_byte` † | E2E-33, E2E-34, E2E-43 |
| VT-2 | FROZEN | `appending_equal_events_yields_two_events` † | E2E-33, E2E-36 |
| VT-3 | FROZEN | `append_preserves_event_payload` + review obligation | E2E-34, E2E-42 |
| VT-4 | FROZEN | `append_stamps_identity_and_time` † | E2E-34, E2E-41, E2E-43 |
| VT-5 | FROZEN | `event_ids_are_unique_within_a_store` †, `append_stamps_a_local_event_id` †, `ingest_preserves_origin_identity` † | E2E-33, E2E-34, E2E-36, E2E-41, E2E-42 |
| VT-6 | PROVISIONAL | `store_id_is_stable_across_reopen` †, `restored_peer_does_not_reissue_identities` † | E2E-34, E2E-42 |
| VT-7 | FROZEN | `event_id_is_not_matchable_by_query` †, `contains_event_id_reports_membership` † | E2E-32, E2E-34, E2E-36 |
| VT-8 | FROZEN | `event_ids_are_unique_within_a_store` † | E2E-33, E2E-36 |
| VT-9 | PROVISIONAL | `append_stamps_a_recorded_time` †, `recorded_time_survives_a_reopen` †, `convergent_projection_is_interleaving_independent` † | E2E-41, E2E-43 |
| VT-10 | PROVISIONAL | §5's `happenstance-sync-testkit` suite — `IngestStore` is the trait every `SY-… | E2E-33, E2E-35, E2E-36, E2E-39, E2E-42 |
| VT-11 | FROZEN | `positions_are_unique`, `positions_are_strictly_monotonic` | E2E-10, E2E-46 |
| VT-12 | NON-NORMATIVE (pointer to ES-10) | none of its own — ES-10 names the rule | see ES-10 |
| VT-13 | FROZEN | unit test `position_next_signals_overflow`; `read_from_is_inclusive` (`suite.r… | E2E-10, E2E-16 |
| VT-14 | PROVISIONAL | unit tests `rejects_invalid_event_types` (`event.rs:407-418`) and `rejects_inv… | E2E-40 |
| VT-15 | FROZEN | `tags_differing_only_by_unicode_normalisation_are_distinct` †, `append_preserves_event_type_and_tags_byte_for_byte` † | E2E-40, E2E-49 |
| VT-16 | FROZEN | `query_item_tags_are_and`, `query_item_tags_match_supersets`, `query_item_rejects_partial_tag_overlap` | E2E-32, E2E-40 |
| VT-17 | FROZEN | `tags_may_repeat_a_key` † | *(none — see clause)* |
| VT-18 | FROZEN | compile tests `event_new_accepts_a_held_event_type` and `command_handler_compo… | E2E-51 |
| VT-19 | FROZEN | `wire::decode_rejects_a_non_canonical_tag_set` †, `wire::decode_accepts_an_over_capacity_value` †, `append_reports_exceeded_store_limits` † | E2E-40, E2E-42 |
| VT-20 | FROZEN | `rejects_invalid_event_types` †, `rejects_invalid_tags` †, `store_round_trips_a_maximum_length_type_and_tag` † | E2E-40 |
| VT-21 | PROVISIONAL | `store_accepts_the_guaranteed_minimum_payload` †, `append_reports_exceeded_store_limits` † | E2E-42 |
| VT-22 | PROVISIONAL | `store_accepts_the_guaranteed_minimum_tag_count` † | E2E-40 |
| VT-23 | PROVISIONAL | `store_evaluates_a_query_at_the_guaranteed_minimum_item_count` † | E2E-36, E2E-40 |
| VT-24 | PROVISIONAL | `store_accepts_the_guaranteed_minimum_batch_size` † | E2E-35, E2E-36, E2E-39 |
| VT-25 | FROZEN | `append_reports_exceeded_store_limits` † | E2E-35, E2E-42 |
| VT-26 | FROZEN | compile test `query_items_is_not_constructible_downstream`; `condition_without… | E2E-40 |
| VT-27 | FROZEN | compile-level; enforced by the wire tests, which would otherwise have a positi… | E2E-04, E2E-05, E2E-37, E2E-38 |
| VT-28 | FROZEN | `read_limit_zero_yields_nothing` †, `read_limit_truncates` | E2E-11, E2E-13 |
| VT-29 | FROZEN | `read_to_is_inclusive` †, `read_from_and_to_bound_a_closed_window` †, `read_to_under_backwards_bounds_the_older_end` † | E2E-11 |
| VT-30 | PROVISIONAL | `condition_guards_carry_independent_boundaries` †, `condition_with_one_guard_behaves_as_today` † | E2E-04, E2E-05, E2E-06 |
| VT-31 | FROZEN | `query_items_are_or`, `query_item_order_does_not_change_the_result_set` †, `query_union_is_item_concatenation` †, … | E2E-32 |

#### `WF` — wire format (§2.7)

| Clause | Maturity | Conformance rule — † = does not exist yet | Cases |
|---|---|---|---|
| WF-1 | DEFERRED | `wire::query_all_is_unambiguous`, `wire::empty_object_is_not_a_condition` † + review obligation | E2E-33, E2E-42 |
| WF-2 | FROZEN | `wire::round_trips_in_postcard` † | E2E-33, E2E-35, E2E-42 |
| WF-3 | FROZEN | `wire::query_all_is_unambiguous` †, `wire::option_query_round_trips` † | E2E-40 |
| WF-4 | FROZEN | `wire::empty_object_is_not_a_condition` †, `wire::condition_round_trips` †, `wire::condition_after_is_visible_to_an_ingest_policy` † | E2E-35, E2E-37, E2E-40 |
| WF-5 | FROZEN | `wire::sequenced_event_round_trips` † | E2E-33, E2E-34, E2E-41, E2E-43 |
| WF-6 | FROZEN | `wire::store_id_encodes_as_hex_in_json` †, `wire::store_id_encodes_as_bytes_in_postcard` † | E2E-34, E2E-42 |
| WF-7 | FROZEN | `wire::round_trips_in_json` †, `wire::round_trips_in_postcard` † | E2E-33, E2E-42 |
| WF-8 | FROZEN | `wire::rejects_an_unknown_format_version` †, `wire::version_is_the_first_field` † | E2E-35, E2E-42 |
| WF-9 | FROZEN | `wire::decode_accepts_an_over_capacity_value` †, `append_reports_exceeded_store_limits` † | E2E-42 |
| WF-10 | FROZEN | `wire::decode_rejects_a_non_canonical_tag_set` †, `wire::decode_rejects_an_unconstrained_query_item` †, `wire::decode_rejects_a_zero_item_query` †, … | E2E-40 |
| WF-11 | PROVISIONAL | `wire::payload_is_base64_in_json` †, `wire::payload_is_raw_in_postcard` † | E2E-33 |
| WF-12 | FROZEN | compile test `read_options_is_not_serialisable` | *(none — see clause)* |

#### `ES` — the `EventStore` port (§3)

| Clause | Maturity | Conformance rule — † = does not exist yet | Cases |
|---|---|---|---|
| ES-1 | FROZEN | every rule in `suite.rs` is generic over `S: EventStore` (`suite.rs:70`, and i… | E2E-52, E2E-53, E2E-54 |
| ES-2 | FROZEN | `send_flavour_stream_is_send_in_generic_code` † | E2E-52 |
| ES-3 | FROZEN | `provided_method_future_is_send_in_generic_code` † | E2E-13, E2E-53 |
| ES-4 | FROZEN | `provided_method_future_is_send_in_generic_code` † | E2E-13 |
| ES-5 | FROZEN | `error_bound_is_identical_on_both_flavours` †, `happenstance` † | E2E-53 |
| ES-6 | DEFERRED | `store_error_crosses_a_join_handle` †, `const` † | E2E-53, E2E-52 |
| ES-7 | PROVISIONAL | not a new rule but a new *invocation* of the existing suite — the `RefCell`-ba… | E2E-52, E2E-09, E2E-54 |
| ES-8 | FROZEN | `read_defaults_to_ascending_order`, `read_from_is_inclusive`, `read_backwards_reverses_order`, … | E2E-10, E2E-12 |
| ES-9 | FROZEN | `query_matching_nothing_yields_empty`, `read_from_a_gap_position` †, `from` † | E2E-10 |
| ES-10 | PROVISIONAL (axis: position allocation) | `nothing_below_an_observed_position_appears_later` †, `positions_are_unique`, `positions_are_strictly_monotonic`, … | E2E-01, E2E-02 |
| ES-11 | PROVISIONAL (axis: transport) | `read_result_is_stable_under_concurrent_append` † | E2E-02, E2E-01 |
| ES-12 | PROVISIONAL (axis: transport) | `query_items_share_one_snapshot` † | E2E-03, E2E-05 |
| ES-13 | FROZEN | `read_result_is_stable_under_concurrent_append` † | E2E-02, E2E-03 |
| ES-14 | FROZEN | `read_limit_truncates`, `read_backwards_from_with_limit`, `limit_applies_across_items_not_per_item` †, … | E2E-12, E2E-13 |
| ES-15 | FROZEN | `duplicate_items_do_not_duplicate_events` †, `query_item_order_does_not_change_the_result_set` † | E2E-32 |
| ES-16 | FROZEN | `read_to_is_inclusive` †, `read_from_and_to_bound_a_closed_window` †, `read_to_under_backwards_bounds_the_older_end` † | E2E-11 |
| ES-17 | PROVISIONAL | `append_preserves_event_payload` | E2E-36, E2E-39 |
| ES-18 | FROZEN | `append_is_atomic`, `condition_rejection_leaves_store_unchanged` | E2E-39, E2E-48, E2E-07 |
| ES-19 | FROZEN | `append_returns_last_written_position`, `batch_positions_follow_slice_order` † | E2E-13, E2E-23 |
| ES-20 | FROZEN | `append_rejects_empty_batch`, `append` †, `empty_batch_is_refused_before_the_condition_is_evaluated` † | E2E-06 |
| ES-21 | FROZEN | `batch_is_not_evaluated_against_its_own_condition` †, `after` † | E2E-06 |
| ES-22 | FROZEN | `dropped_append_future_leaves_no_partial_batch` † | E2E-07 |
| ES-23 | FROZEN | *(none — see clause)* | E2E-07 |
| ES-24 | FROZEN | `reissued_conditional_batch_lands_once` †, `reissued_unconditional_batch_lands_twice` † | E2E-07, E2E-33, E2E-36 |
| ES-25 | FROZEN | `condition_without_after_rejects_any_match`, `condition_without_after_allows_non_match`, `condition_after_ignores_non_matching_events`, … | E2E-08, E2E-55, E2E-39 |
| ES-26 | FROZEN | `condition_after_ignores_events_at_the_boundary`, `condition_after_rejects_events_beyond_the_boundary`, `read_from_is_inclusive` | E2E-56, E2E-10 |
| ES-27 | FROZEN | `condition_matches_on_tags` †, `condition_with_an_unheld_tag_does_not_reject` †, `query_item_types_are_or`, … | E2E-55, E2E-03 |
| ES-28 | FROZEN | `condition_against_an_empty_store_admits_the_append` †, `condition_after_beyond_head_admits_the_append` † | E2E-56, E2E-08 |
| ES-29 | FROZEN | `wire_condition_with_after_is_refused` † | E2E-37, E2E-38, E2E-56 |
| ES-30 | FROZEN | `head_of_an_empty_store_is_none` †, `head_is_the_highest_visible_position` †, `head` †, … | E2E-13, E2E-02, E2E-25 |
| ES-31 | FROZEN | `checkpoint_lag_is_not_a_position_difference` † | E2E-13, E2E-23, E2E-25 |
| ES-32 | PROVISIONAL | *(none — see clause)* | E2E-32, E2E-28 |
| ES-33 | FROZEN | this is a conformance obligation on the testkit rather than on an adapter's be… | E2E-08, E2E-46 |
| ES-34 | FROZEN | `two_handles_share_one_consistency_boundary` †, `after` † | E2E-08 |
| ES-35 | PROVISIONAL (axis: durability) | `acknowledged_writes_survive_a_reopen` †, `const` † | E2E-46 |
| ES-36 | FROZEN | `interleaved_appends_on_one_handle_elect_one_winner` †, `append` †, `current_thread` † | E2E-09 |
| ES-37 | FROZEN | *(none — see clause)* | E2E-48, E2E-49, E2E-46 |
| ES-38 | FROZEN | `positions_are_not_reused_after_removal` † | E2E-46, E2E-10 |
| ES-39 | DEFERRED | `a_store_reports_the_history_it_does_not_hold` † | E2E-46, E2E-47, E2E-56 |
| ES-40 | PROVISIONAL (axis: completeness) | `condition_over_removed_history_does_not_reject` † | E2E-47, E2E-56, E2E-44 |

#### `PS` — the `ProjectionStore` port (§4)

| Clause | Maturity | Conformance rule — † = does not exist yet | Cases |
|---|---|---|---|
| PS-1 | FROZEN | `commit_is_atomic_with_the_read_model` † | E2E-17, E2E-21, E2E-23 |
| PS-2 | FROZEN | a testkit-internal hostile store, `CheckpointOnlyStore`, in `crates/happenstan… | E2E-17, E2E-24 |
| PS-3 | PROVISIONAL | `cargo hack --feature-powerset` in `cargo xtask ci`, which already runs; the e… | *(none directly; cites E2E-15, E2E-25)* |
| PS-4 | PROVISIONAL | `commit_is_atomic_with_the_read_model` † | E2E-24 |
| PS-5 | PROVISIONAL | `MemoryProjectionStore` and one real adapter compiling without the `where Self… | E2E-19, E2E-24 |
| PS-6 | PROVISIONAL | the signature; no runtime rule | E2E-24 |
| PS-7 | FROZEN | `dropped_batch_leaves_store_usable` † | E2E-24 |
| PS-8 | FROZEN | `rollback_leaves_both_unchanged` † | E2E-24, E2E-28 |
| PS-9 | PROVISIONAL | *(none — see clause)* | E2E-20, E2E-29 |
| PS-10 | FROZEN | `compile_fail` † | E2E-20, E2E-29 |
| PS-11 | PROVISIONAL | `commit_is_atomic_with_the_read_model` † | E2E-20, E2E-21, E2E-22, E2E-17 |
| PS-12 | PROVISIONAL | `batch_reads_reflect_pending_writes` †, `const` †, `false` † | E2E-21, E2E-22 |
| PS-13 | FROZEN | `rebuild_is_chunk_size_invariant` † | E2E-21, E2E-22 |
| PS-14 | FROZEN | `rebuild_is_chunk_size_invariant` † | E2E-22 |
| PS-15 | PROVISIONAL | `commit_rejects_a_foreign_batch` † | E2E-19 |
| PS-16 | PROVISIONAL | `reset_clears_rows_and_checkpoint_together` †, `reset` †, `probe_delete_all` † | E2E-15, E2E-17 |
| PS-17 | FROZEN | `reset_is_scoped_to_one_projection` † | E2E-18 |
| PS-18 | PROVISIONAL | `refused_reset_changes_nothing` † | E2E-18 |
| PS-19 | FROZEN | `reset_is_not_commit_at_first` † | E2E-15, E2E-16 |
| PS-20 | FROZEN | covered by `reset_is_not_commit_at_first`'s second half; no separate rule, bec… | E2E-16, E2E-23 |
| PS-21 | FROZEN | `commit_accepts_a_position_the_batch_did_not_write` † | E2E-23 |
| PS-22 | PROVISIONAL | `commit_rejects_a_regressing_position` † | E2E-23, E2E-25 |
| PS-23 | PROVISIONAL | `distinct_projections_advance_independently` † | E2E-28, E2E-32 |
| PS-24 | PROVISIONAL | `rebuilding_is_distinguishable_from_live` †, `checkpoint` † | E2E-25 |
| PS-25 | PROVISIONAL | `changed_query_starts_a_new_checkpoint` † | E2E-50 |
| PS-26 | FROZEN | `failure_policy_is_per_projection` † | E2E-27 |
| PS-27 | PROVISIONAL | `skip_and_record_is_atomic` †, `on_error` † | E2E-26, E2E-27 |
| PS-28 | FROZEN | `pump_reports_the_failing_position` †, `checkpoint` † | E2E-26 |
| PS-29 | FROZEN | `one_poisoned_projection_does_not_stall_the_others` † | E2E-28 |
| PS-30 | PROVISIONAL | `panicking_apply_rolls_back` † | E2E-28 |
| PS-31 | FROZEN | *(none — see clause)* | E2E-31 |
| PS-32 | FROZEN | *(none — see clause)* | E2E-20 |
| PS-33 | DEFERRED | *(none — see clause)* | *(none directly; cites E2E-26, E2E-28)* |
| PS-34 | PROVISIONAL | a doctest on `ProjectionStore` implementing the port for a toy store, which ca… | E2E-24 |
| PS-35 | DEFERRED | *(none — see clause)* | E2E-30, E2E-52, E2E-53 |
| PS-36 | FROZEN | a `compile_fail` doctest showing the diagnostic, which is documentation that C… | E2E-30 |
| PS-37 | FROZEN | *(none — see clause)* | E2E-52, E2E-53 |

#### `SY` — the `SyncPeer` port (§5)

| Clause | Maturity | Conformance rule — † = does not exist yet | Cases |
|---|---|---|---|
| SY-1 | FROZEN | `ingest_never_rejects` † | E2E-33, E2E-39, E2E-42, E2E-45, E2E-47 |
| SY-2 | FROZEN | `compensation_is_atomic_with_the_losing_event` †, `append_is_atomic` | E2E-39 |
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
| SY-21 | PROVISIONAL | `convergent_projection_cannot_observe_local_position` †, `apply` † | E2E-41 |
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
| CF-1 | FROZEN | `mutation_coverage::every_rule_has_a_mutant` † | *all* |
| CF-2 | FROZEN | `mutation_coverage::mutant_registry_is_exhaustive` † | *all* |
| CF-3 | FROZEN | `mutation_coverage::mutants_fail_exactly_their_declared_rules` † | *all* |
| CF-4 | FROZEN | `mutation_coverage::every_mutant_states_its_provenance` † | E2E-01, E2E-08, E2E-32, E2E-55 |
| CF-5 | FROZEN | `mutation_coverage::conformant_variants_pass_everything` † | E2E-10 |
| CF-6 | FROZEN | `mutation_coverage::conformant_variants_pass_everything` † | E2E-10 |
| CF-7 | FROZEN | `condition_matches_on_tags` † | E2E-55, E2E-56 |
| CF-8 | FROZEN | `condition_with_an_unheld_tag_does_not_reject` † | E2E-55, E2E-56 |
| CF-9 | FROZEN | `duplicate_items_do_not_duplicate_events` † | E2E-32 |
| CF-10 | FROZEN | `condition_against_an_empty_store_admits_the_append` † | E2E-47 |
| CF-11 | FROZEN | `empty_batch_is_refused_before_the_condition_is_evaluated` † | E2E-06 |
| CF-12 | FROZEN | `read_from_composes_with_multi_item_query` † | E2E-10 |
| CF-13 | DEFERRED | `nothing_below_an_observed_position_appears_later` † | E2E-01, E2E-02 |
| CF-14 | DEFERRED | `acknowledged_writes_survive_a_reopen` †, `const` † | E2E-07 |
| CF-15 | FROZEN | `fixture_isolation` † | E2E-08, E2E-09 |
| CF-16 | FROZEN | `two_handles_share_one_consistency_boundary` † | E2E-08 |
| CF-17 | PROVISIONAL | `acknowledged_writes_survive_a_reopen` † | E2E-07 |
| CF-18 | FROZEN | `capability_skips_are_reported` † | E2E-07, E2E-08 |
| CF-19 | FROZEN | `two_handles_share_one_consistency_boundary` † | E2E-08 |
| CF-20 | FROZEN | the wasm32 step of `cargo xtask ci` (`xtask/src/main.rs:60-77`), extended to b… | E2E-52, E2E-30 |
| CF-21 | FROZEN | a doctest in `fixtures` constructing a strategy, which fails to compile if the… | E2E-32 |
| CF-22 | FROZEN | `registry::every_rule_is_enumerated` † | E2E-52, E2E-30, E2E-09 |
| CF-23 | FROZEN | the wasm32 build step of `cargo xtask ci`, extended to compile a `wasm-bindgen… | E2E-52, E2E-30 |
| CF-24 | FROZEN | `registry::no_orphan_rules` † | *all* |
| CF-25 | FROZEN | `cargo xtask spec-trace` (CF-38), which reads the portfolio table and the matu… | E2E-01, E2E-02, E2E-24, E2E-46, E2E-52 |
| CF-26 | FROZEN | the portfolio table's `Far end exists` column, checked by `cargo xtask spec-tr… | E2E-01, E2E-24 |
| CF-27 | DEFERRED | `suffix_store_is_distinguishable_from_a_young_store` †, `positions_are_not_reused_after_removal` †, `condition_over_removed_history_does_not_reject` † | E2E-46, E2E-47, E2E-56, E2E-33 |
| CF-28 | FROZEN | the existing suite, invoked against a `RefCell`-backed store under a single-th… | E2E-52, E2E-09, E2E-30 |
| CF-29 | FROZEN | `mutation_coverage::every_rule_has_a_mutant` † | *all* |
| CF-30 | NON-NORMATIVE | *(none — see clause)* | *(none — see clause)* |
| CF-31 | FROZEN | `cargo xtask spec-trace` (CF-38), which fails when a rule name referenced by a… | *all* |
| CF-32 | FROZEN | a `cargo xtask ci` manifest check | *(none — see clause)* |
| CF-33 | FROZEN | a `cargo xtask ci` lint step over `happenstance-testkit/src`, rejecting `std::… | *(none — see clause)* |
| CF-34 | PROVISIONAL | *(none — see clause)* | *(none — see clause)* |
| CF-35 | FROZEN | `cargo xtask spec-trace` (CF-38) | *all* |
| CF-36 | FROZEN | `cargo xtask spec-trace` (CF-38), cross-referencing each case's level marker (… | E2E-28, E2E-29, E2E-39, E2E-42 |
| CF-37 | FROZEN | `cargo xtask spec-trace` (CF-38) | *all* |
| CF-38 | FROZEN | `cargo xtask spec-trace` (new step in `xtask/src/main.rs`'s `REQUIRED` list, `… | *all* |

### 7.3 Clauses with no conformance rule

Ten normative clauses name no rule, plus one withdrawn to prose. Under CF-35 a
clause naming no rule is either wrong or belongs in prose, and must say which.
Each of the ten says which, and the verdicts are not uniform — three of them are
defects in this document rather than facts about the design.

| Clause | Why no rule | Verdict |
|---|---|---|
| **ES-23** — cancellation outcome is unspecified | The normative content *is* the word "unspecified". A rule asserting either outcome would convert a `MAY` into a `MUST` | **Correct as a clause.** The adapter-visible half is ES-22, which is checkable |
| **ES-32** — no tail or subscription seam at 0.1 | The absence of a method is not observable by a rule | **Correct as a clause.** Its checkable substitutes are ES-30 and ES-15 |
| **ES-37** — `EventStore` is closed over insertion | Same: the absence of a delete method | **Correct as a clause.** Its checkable consequences are ES-38 and ES-40 |
| **PS-9** — `Batch` carries no universal write vocabulary | Nothing checks the absence of a supertrait bound | **Correct as a clause.** The obligation it creates, PS-11, is checkable |
| **PS-31** — outward-writing projections are out of scope | A documented exclusion is not adapter-checkable | **Correct as a clause**, because silence here is what produces the wrong implementation |
| **PS-32** — ADR-0007's Context must be corrected | It is an instruction to edit a document | **Should be prose.** A specification clause whose subject is another document's wording is a work item, not a constraint on any implementation. It is retained here only because deleting it would lose the correction |
| **PS-33** — ADR-0007's falsifier must be evaluated at a named phase exit | A phase gate, not an adapter obligation | **Should be prose,** for the same reason — but its content is load-bearing: `0007:119-121` sets a falsifier that no phase currently evaluates, and an unevaluated falsifier is indistinguishable from none |
| **PS-35** — the derivation decision must cover both ports in one ADR | The artefact is the ADR | **Should be prose.** It constrains the next pass's paperwork, not an adapter |
| **PS-37** — the `Self: Sync` rule applies to `ProjectionStore` | The obligation is on the contract crate; the artefact is a generic helper that compiles | **Correct as a clause,** and the compile *is* the check — a `cargo xtask ci` build failure is as binding as a rule. It is listed here because CF-38's checker will not find a rule name and must not treat that as a dangling reference |
| **CF-34** — performance is measured by a separate harness, which is not the bar | The clause's content is that no conformance rule may be the check | **Correct as a clause,** and self-referentially so: a rule enforcing it would violate CF-33 |
| **CF-30** — the testkit pin recommendation | Withdrawn: no adapter behaviour violates it | Already `[NON-NORMATIVE]`; the ID is retained so citations resolve |

Three clauses — PS-32, PS-33 and PS-35 — are therefore this list's real content.
All three are in §4, all three are instructions to the pass that lands this
specification rather than constraints on an implementation, and all three should
move into that pass's work list and out of the clause space when it exists. They
are left as clauses in this revision because moving them now would lose them: the
work list does not exist yet, and PS-33's falsifier has already survived one
document handover by being written in an ADR that no phase reads.

Two further clauses name a rule that checks only part of them, and are recorded
here so the checker does not report them as clean. **VT-3** — the contract, a
store and a peer MUST NOT parse `data` or `metadata` — is checked mechanically
only in its positive half (`append_preserves_event_payload`); the prohibition is a
review obligation. **WF-1** — the wire format is private — has two tests that fail
if it is abandoned in practice, and its scope half is likewise a review
obligation. Neither is a defect; both are places where a reviewer is load-bearing
and the document says so rather than implying otherwise.

### 7.4 Conformance rules no clause names

Two of the twenty-seven rules in
[`crates/happenstance-testkit/src/suite.rs`](../../crates/happenstance-testkit/src/suite.rs)
are named by no clause's `Rule:` line. Both are mentioned in the surrounding prose
of §3 and §6, which is not the same thing: prose that discusses a rule does not
put it under a clause's protection, and CF-38 is specified to fail on exactly this
gap.

**`query_all_matches_every_event`** (`suite.rs:70-80`). The natural home is VT-31
or ES-15, and both instead name the new `duplicate_items_do_not_duplicate_events`.
That is deliberate and it is also a hole. The existing rule appends three
*untagged* events, so the fan-out a tag join without `DISTINCT` produces cannot
occur (CF-9). The rule is not wrong; it is weaker than the clause it would be
protecting, and a clause that named it would be claiming coverage it does not
have. The resolution is that CF-9's strengthened successor supersedes it — which
means the rule should be *rewritten* rather than kept alongside, and no clause
currently says so. **This is a defect: name the disposition of the existing rule
in ES-15, or the traceability checker will report it forever.**

**`racing_conditional_appends_elect_one_winner`** (`suite.rs:596-638`). ES-34
discusses it and deliberately does not name it: the rule is sequential *and*
single-handle, so it does not reach the three adapters ES-34 exists to reject.
§6 additionally records that its one tagged condition runs against a store where a
type-only probe returns identical verdicts (`suite.rs:603-605`), which is why CF-7
and CF-8 specify replacements. So the rule is orphaned because it is being
retired, and again no clause says so. **Same defect, same fix**: ES-25 or ES-27
should name its disposition.

Both are the same shape of error, and it is worth naming the shape. A
specification that only ever names the rules it wants written leaves the rules
that already exist unowned, and an unowned rule is one nobody is responsible for
deleting when it stops meaning anything.

### 7.5 Clauses that name no case

Five clauses name no E2E case at all, and two more name cases only in prose after
declaring "none directly". CF-35 requires every clause to name the cases it
serves, so this list is a defect list, not a note.

| Clause | What it names instead | Verdict |
|---|---|---|
| **VT-17** — `key:value` is convention, not enforced | Nothing. It comes from Wattline (`docs/scenarios/README.md:628-635`) and no case covers it | **Defect: the case is missing.** A case is writable — construct a `Tag` with no colon and one with two, assert both are accepted and that neither acquires structure |
| **WF-12** — `ReadOptions` is not on the wire | Nothing; it removes a surface | **Defect, minor.** A case is writable and trivial: assert `ReadOptions` does not implement `Serialize`. It is a compile test, which is why the clause reached for one |
| **CF-32** — the testkit carries its own `version` key | Nothing; a packaging obligation with a machine check | **Acceptable.** CF-35 permits a clause whose check is a manifest check rather than a behavioural one, and there is no user-visible behaviour to write a case against |
| **CF-33** — no rule may read a clock | Nothing; it constrains the suite | **Acceptable**, same reason |
| **CF-34** — benchmarks are not conformance | `E2E-CASES.md:1589-1593`, which records the harness as one of the two things that are neither blocked nor cases | **Acceptable**, and the citation is the right one |
| **PS-3** — ship behind `unstable-projection` | "none directly", then E2E-15 through E2E-25 | **Acceptable.** It is a packaging decision that makes a range of cases safe to answer before publication |
| **PS-33** — evaluate ADR-0007's falsifier | "none directly", then E2E-26 through E2E-28 | Already listed in §7.3 as belonging in a work list |

So: two genuine holes (VT-17, WF-12), both cheap to close, and five clauses where
naming no case is the honest answer and the clause says why.

### 7.6 E2E cases no clause claims

**None. All 56 cases are claimed by at least one clause.**

That result is suspicious enough to state how it was reached, because "the defect
list is empty" is the shape of a list that was never computed.

The `Cases:` line of all 193 clauses was parsed, every `E2E-nn` reference
extracted, and the union compared against `E2E-01`…`E2E-56` as enumerated in
[`E2E-CASES.md`](../scenarios/E2E-CASES.md)'s index (`:30-41`) and confirmed
against its 56 `### E2E-nn` headings. The comparison was then run a second time
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
