---
id: HS-I0006
uid: 86ec1f
type: initiative
slug: from-contract-to-published-library
title: From Contract to Published Library
parent: null
initiative: from-contract-to-published-library
project: null
status: implementing
process: initiative
stage: implementation
automation: HITL
severity: null
blocked_by: []
blocks: []
tier: standard
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-14T01:25:15.902Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: 62affb3b
---
# From Contract to Published Library

## One-line intent

Take `happenstance` from a contract that only a type checker has ever agreed with
to a library an application author can install and rely on, an adapter author can
be told when they are finished by, and whose remaining silences have been answered
out loud.

## Vision and narrative

Five bodies of work are finished, and they were sequenced by **blast radius** so
that the expensive-to-revise decisions were settled first (`RUNBOOK.md:38-40`).
Three crate names are owned. The two-flavour async port design is proved rather
than argued. Six skeletons compile on their real targets with real associated
types. The conformance suite has been made into an instrument — every rule has a
registered wrong implementation that fails it. The `EventStore` contract, the
values crossing it, and the wire format are frozen.

All of that is the half that can be settled by reasoning against a compiler. What
is left is the half that can only be settled **by contact** — with a database,
with a consumer, with a second implementation that genuinely disagrees, and with a
user who has installed the crate and cannot be asked to accept a correction.

Six things are true today and should not be, and each has the same underlying
shape: something that looks like evidence is not. A `todo!()` body type-checks
against *any* signature, so thirty-seven of them across six crates have told us
nothing. Every implementation that has actually passed the suite serialises its
writers and assigns positions under a lock it holds until commit — four adapters,
**one storage shape wearing four hats**. `ProjectionStore` carries the largest
block of provisional clauses and the invariant it exists to protect is documented
on the trait and enforced by nothing. The crate people will actually install is a
glob re-export (`crates/happenstance/src/lib.rs`), so no consumer has ever
discovered a contract defect. Nothing is published, so the MSRV, the public
surface and the semver commitment are each an opinion nobody is relying on. And
replication and retention are unanswered — a position is a statement about one
store's log, and a reader that quietly builds a wrong answer from a truncated log
is the failure mode nothing currently detects.

Discovery changed how much of this is a private standard and how much is a gap in
the field. Three findings matter for how this initiative is framed:

**"DCB-compliant" is a self-asserted label everywhere.** The specification defines
no conformance suite, no test kit and no certification process; its own conformance
clause asks only for "equivalent functionality", judged by each implementer
(`_discovery/research/01-…`). Two DCB-labelled stores already disagree in public
on whether positions may contain gaps — a portability trap an application author
could walk into by accident. Nothing outside this project can vouch for the phrase,
which means whatever credibility it earns has to come from the proof artefact
itself.

**The category this library is aiming at does not currently exist in Rust.** The
crates with real adoption (`cqrs-es`, `esrs`) are storage-coupled frameworks; the
one prior storage-agnostic attempt died at exactly one non-memory backend in 2020;
the spec's own Rust entry ships one backend and describes itself as a "slightly
different approach". The nearest live comparison, `umadb-dcb`, is a client for a
new standalone server, not a port over databases you already operate — and it
published a release the day before this intake (`_discovery/research/02-…`). The
segment this initiative serves is *unserved*, not underserved, and the space is
sparse but moving.

**Everything the house rules already insist on is converged industry practice.**
"An adapter that compiles but has not run the suite is not an adapter" is the TCK
premise. "A rule no adapter can fail is decorative" is Jepsen's. Capability
declension with a stated reason rather than a silent skip is what OpenDAL and
Kubernetes CSI independently arrived at. And "a port is only as well-designed as
the spread of what implements it" is reproduced as field damage by every production
DCB adopter surveyed (`_discovery/research/03-…`, `04-…`). That is not a reason to
relax the bar. It is a reason to state it as inherited practice and then actually
clear it, because the same research shows storage-agnostic ports carrying
undetected semantic drift for *years* after wide adoption.

So the narrative this initiative closes is simple to state and expensive to fake:
the contract stops being a well-argued document and becomes a thing that has been
used, implemented three ways that disagree with each other, published under
promises that can no longer be withdrawn, and honest about what it still refuses
to decide.

## Goals

- **The contract gets used.** A typed layer and a worked example sit above the
  facade, and the contract defects that use discovers are recorded rather than
  quietly absorbed.
- **The conformance suite gets run by implementations that disagree.** At minimum:
  a real durable store, a store that does not serialise its writers, and a store
  with no connection and no cursor — each green, none of them an instrument with
  `todo!()` bodies.
- **`ProjectionStore` earns its freeze, and the freeze is tested afterwards.** The
  axis is batch shape and transactional seam; the verdict on whether the freeze
  held is written either way, not assumed.
- **Publication converts private opinions into promises.** `0.2.0-alpha.1` after
  the typed layer, `0.2.0` after the adapters, with every provisional clause
  audited against the clause ledger at the moment of publish rather than against
  prose.
- **The two open silences get answers.** Replication identity across a store
  boundary, and what a store may forget and how a reader finds out — each a
  decision or an explicit, reasoned refusal. Silence is not an outcome.
- **The `!Send` / edge case survives contact.** It is carried at real cost so a
  store can live in a constrained single-threaded runtime; work that quietly drops
  it has changed the product.

## Non-goals

- Deciding **how the work decomposes** — whether a RUNBOOK phase is a project,
  whether several collapse into one, and where the release boundaries fall. That
  is `/redkiln:plan`'s, from its own grounding.
- Proposing architecture, tech choices, or public API shape. The API surface
  review is a real gate, but it is the planning design stage's
  (`.redkiln/templates/_design.md` as repurposed), not this charter's.
- Reaching zero open questions. Several are deliberately unresolved and are already
  atoms under `.kb/open-questions/`; settling one in passing is a defect, not
  progress.
- Winning a comparison. Whether to state an explicit position against the nearest
  peers is itself an open design tension below, not a settled goal.

## In scope

- Making the typed layer real above the facade, and a worked example that exercises
  it the way an application would.
- Freezing `ProjectionStore`, and the written verdict on whether that freeze held
  once a structurally unlike batch shape exists.
- Building the adapters that have so far only ever been instruments, to the point
  where each has *passed* the conformance suite rather than compiled against it.
- The publication train: registry-facing completeness, the semver baseline
  comparison, the MSRV commitment becoming a promise, and the provisional-clause
  audit at the moment of publish.
- Written answers — decision or reasoned refusal — on replication identity across a
  boundary and on incomplete-log semantics, together with whatever proof each
  answer needs to not be an assertion.
- The decision records the above requires, taken through the runbook's ADR queue
  rather than as side effects.
- Recording, at closeout, the personas and journeys this work was done for, so the
  next initiative inherits an adjudicated audience instead of re-deriving one.

## Out of scope (explicit non-goals)

Named concretely, so nobody has to guess at the boundary:

- **Amending anything `[FROZEN]`.** The `EventStore` contract, the values crossing
  it, and the wire format are out of scope for amendment. If this work needs one
  changed, that is a new decision atom and a re-plan, not a line edit.
- **A `1.0` release, or any post-1.0 semver commitment.** The publish target is
  `0.2.0-alpha.1` then `0.2.0`. Under 0.x the minor bump *is* the breaking-change
  boundary; that is the promise being made, and no larger one.
- **Publishing `happenstance-sync` or a `happenstance-sync-testkit` to the
  registry** as part of this release train. The sync port stays out of the contract
  crate precisely so that publishing never waits on replication.
- **A query language, or any extension beyond the base specification.** Peers in
  this space are each growing bespoke extensions; staying close to the bare
  specification is a deliberate, legible choice here and is not up for
  reconsideration inside this initiative.
- **A standalone server, a hosted service, or non-Rust clients.** The shape being
  built is a port over databases the user already operates, not a new store you run.
- **A backward-compatibility adapter for existing non-DCB event stores.** DCB's own
  maintainers describe this as unsolved; it is not this initiative's to solve.
- **Performance work beyond the measurements already owed.** The position-visibility
  cost measurement is in scope because a decision depends on it. General throughput
  tuning, benchmarking suites and optimisation passes are not.
- **Hand-authoring `.kb/` atoms outside the ingest and closeout paths.** The first
  attempt at that was reverted (`0269720`) for producing the directory layout of the
  process without the process.
- **Reusing the reverted decomposition.** The ground this initiative covers was once
  hand-split into three initiatives (HS-I0002/3/4) and reverted wholesale. Those
  bodies are prior art for candidate seams only; planning re-derives the
  decomposition from grounding.
- **Incidental bugs found in passing.** They route to the `support` initiative per
  `.redkiln/config.yaml:5`, and are not absorbed as in-scope fixes.
- **Any screen, UI, or documentation site.** `userFacing` is false. The public API
  surface is the only "surface" here, and it is reviewed at the design stage.
- **Running `redkiln adopt --templates`.** It would silently overwrite six
  deliberate customisations and then fail CI on the absence it created.

## Who this is for

**The application author** doing dynamic consistency boundary event sourcing in
Rust, who wants to model a consistency boundary once and choose a database second.
Today every Rust event-sourcing crate with real adoption forces the database first,
and the one prior storage-agnostic attempt stalled at a single backend. Their fear
is discovering a contract defect in production — particularly through a *second,
independent reader* building a wrong answer from a torn or gapped log.

**The adapter author**, including this project's own, who needs an executable
definition of "correct" rather than a prose specification to interpret. Their fear
is that the port quietly assumed something their storage cannot provide — the same
fear that made the nearest domain peer reject storage-agnosticism outright for a
structurally identical invariant.

**The local-first and edge Rust developer** on a constrained, single-threaded
runtime, currently hand-rolling the whole layer and absorbing the platform's own
gaps directly. Their pain is documented and dated, not inferred. Their fear is
being on the less-supported path and being orphaned the way the one prior serious
attempt in this niche was.

**The evaluator**, deciding from a bounded look at public evidence whether
"DCB-compliant", "storage-agnostic" and "edge-capable" are real claims before
adopting. They have no external body to check a compliance claim against, and often
one look at a registry page in which to decide.

## Referenced personas & journeys

The durable `.kb/` product layer is **structurally present and functionally empty**
— `.kb/product/README.md` and `.kb/design/README.md` are template READMEs, and no
persona or journey atom (`authority_tier: product`) exists yet in this tree. There
is therefore nothing adjudicated to cite, and inventing one here would be exactly
the hand-authoring that was reverted at `0269720`.

The four personas above and the moment-by-moment journeys they improve are
therefore carried from this initiative's own distillation:
[`_discovery/distillation/personas-and-journeys.md`](_discovery/distillation/personas-and-journeys.md),
with the archetypes in
[`_discovery/distillation/opportunities.md`](_discovery/distillation/opportunities.md).

The journeys named there and improved by this work:

- *Choose a contract before a database* — the application author's first hour, from
  modelling a boundary to running it, without the database being decided first.
- *Learn when you are finished* — the adapter author's loop, from a signature that
  type-checks to a suite that says pass or fail and names why.
- *Event-source at the edge without hand-rolling it* — the constrained-runtime
  developer's path from platform primitives to a working write model.
- *Decide in one sitting* — the evaluator's bounded look at public evidence,
  ending in adopt or decline for a stated reason.

**Flagged for promotion at closeout:** these four personas and their journeys should
be promoted into `.kb/product/` as `authority_tier: product` atoms as part of this
initiative's closeout, so the next initiative inherits an adjudicated audience.
Whether the evaluator is its own persona or an earlier stage of the application
author's journey is left open below. All four rest on secondary evidence — download
counts, public posts, an adjacent project's postmortem — never direct contact; that
qualification travels with them into the KB.

## Why now

- **Every cheap decision has been taken, and only the expensive ones are left.**
  Sequencing by blast radius has done its work: what remains cannot be settled more
  cheaply by waiting, only more expensively by publishing first and learning after.
- **The instruments exist but have never been fired.** Six skeletons were built
  specifically so that something in the workspace sits at the far end of each axis
  the contract might be wrong about. Held any longer, they are a portfolio of
  intentions.
- **The window on the category is open and not indefinitely.** The nearest live
  comparison shipped a release the day before this intake and already has real
  download volume. The comparison set is thin, but it is not static.
- **Publication is the only thing that makes some mistakes real — and it is also the
  only thing that makes them permanent.** Semver breakage in Rust is common,
  invisible on review, and best caught by diffing against a registry baseline that
  does not exist until a first release does.
- **The audience is currently paying the cost by hand.** The edge developer's pain
  is documented in live issue threads and a public write-up concluding the other
  language is the less painful path. That is a real person absorbing this gap today.

## Business requirements matrix

| ID | Requirement | Rationale | Priority | Source |
| --- | --- | --- | --- | --- |
| BR-01 | The contract must have been exercised by a consumer above the facade, and the defects that use discovered must be recorded | A consumer is what discovers contract defects; today the installable crate is a re-export and nothing has played that role | Must | `_intake-brief.md` Desired Outcome 1; `crates/happenstance/src/lib.rs` |
| BR-02 | The conformance suite must be green against at least three implementations that genuinely disagree on storage shape | Four adapters at the same end of every axis is one shape wearing four hats; disagreement is the proof, not the risk | Must | `_intake-brief.md` Proof artefact; `_discovery/research/04-…` |
| BR-03 | Every implementation counted toward BR-02 must be a passing implementation, not a skeleton | A `todo!()` body type-checks against any signature, so skeleton-based proof is decorative by construction | Must | `RUNBOOK.md:91-100`; `CLAUDE.md` repository map |
| BR-04 | `ProjectionStore` must be frozen only once its own invariant is enforced by something, and the freeze must be re-tested against an unlike batch shape afterwards | It carries the largest provisional block, and its invariant is documented on the trait and enforced by nothing | Must | `_intake-brief.md` Clauses; `RUNBOOK.md:157` |
| BR-05 | A published release must exist at `0.2.0-alpha.1` and at `0.2.0`, with the boundary between them treated as an externally legible claim | Under 0.x the minor bump is the ecosystem's actual breaking-change signal, read by cargo and by humans | Must | `_discovery/research/08-…`; `RUNBOOK.md:157` |
| BR-06 | Each provisional clause must be audited against the clause ledger at the moment of publish and be frozen, demoted, or carrying a non-empty falsifier | A provisional marker with no falsifier is indistinguishable from a decision nobody wanted to make | Must | `spec/SPECIFICATION.md:215-221` |
| BR-07 | The public surface must be diffed against its own last published baseline before each release | Semver breakage is common and invisible on review; code that violates semver does not look wrong | Must | `_discovery/research/08-…` |
| BR-08 | The MSRV must become a stated, justified promise at first publish rather than a build setting | Whether an MSRV bump is breaking is unresolved ecosystem-wide; the transition is a relationship change, not a formality | Must | `.kb/decisions/0029-msrv-raised-to-1-97-1.md`; `CLAUDE.md` |
| BR-09 | Registry-facing completeness — licence, description, README, rendered docs — must be verified rather than assumed | Publication does not stop a crate shipping in a state that looks unfinished on its own landing page; it fails silently | Must | `_discovery/research/08-…`; `RUNBOOK.md` phase 0 |
| BR-10 | Replication identity across a store boundary must receive a written decision or an explicit reasoned refusal | A position is a statement about one store's log; the field has reached this the hard way, via documented duplicate-delivery bugs | Must | `_intake-brief.md` Open Questions; `_discovery/research/06-…` |
| BR-11 | Incomplete-log semantics — what a store may forget, and how a reader finds out — must receive a written decision or an explicit reasoned refusal | A reader that quietly builds a wrong answer from a truncated log is the failure mode nothing currently detects | Must | `_intake-brief.md` Open Questions; `_discovery/research/07-…` |
| BR-12 | The `!Send` / constrained-runtime flavour must remain exercised end to end by everything that ships | It is carried at real cost so a store can live in a single-threaded edge runtime; dropping it quietly changes the product | Must | ADR-0001; `references/seeds/remaining-runway.md:88-91` |
| BR-13 | Where a guarantee does not apply to an implementation, the consumer must be told so with a stated reason rather than by silent absence | Every mature multi-backend suite converged on declared capability plus a reported reason; the alternative is a vanished rule | Must | `_discovery/research/03-…`; `.kb/open-questions/cf-40-fixture-limits-ownership.md` |
| BR-14 | What the library promises about positions and gaps must be findable and readable before a consumer depends on it | Two DCB-labelled stores already disagree on this in public; it is a live portability trap | Should | `_discovery/research/01-…` |
| BR-15 | Each answer this initiative settles must be recorded as a decision atom that states the alternatives that lost and why | Accepted decisions are immutable and are the only durable record of why an option was rejected | Should | `.kb/decisions/README.md`; `CLAUDE.md` |
| BR-16 | The personas and journeys this work serves must be promoted into the durable product layer at closeout | That layer is currently empty; leaving it empty means the next initiative re-derives an audience from scratch | Should | `.kb/product/README.md`; `_discovery/grounding/product-functional-alignment.md` |
| BR-17 | Whether and how this library is positioned against its nearest live peers must be decided rather than defaulted | Silence on a peer that shipped the day before intake reads as an oversight, not as neutrality | Could | `_discovery/distillation/opportunities.md` |

## Acceptance criteria

Framed from what a person is trying to do, and each observable rather than argued.

- **AC-01 — The application author can model a boundary before choosing a database.**
  Starting from the worked example, they can express a consistency boundary, append
  under it and read it back, without the storage choice appearing in the code that
  expresses the domain.
- **AC-02 — The application author is protected by the compiler when their domain
  grows.** Adding an event variant to the worked example's model stops the build
  until it is handled, and that protection is asserted by a case that fails if it
  regresses — not by a convention.
- **AC-03 — The application author can install the library from the registry and get
  something that works.** A project outside this workspace can add the published
  crate and complete a full write-then-read cycle without reaching into internals.
- **AC-04 — The adapter author is told when they are finished.** They can invoke one
  entry point against their own fixture and receive a pass or a named failure for
  every rule, with no rule silently absent from the run.
- **AC-05 — The adapter author is told, with a reason, where a guarantee does not
  apply to them.** A declined capability reports the fixture's stated reason and
  still appears in the output; it never vanishes from the binary.
- **AC-06 — The adapter author's storage shape is not quietly assumed.** The suite
  passes against an implementation that does not serialise its writers and against
  one that cannot hold a transaction open at all — or, where it cannot, the contract
  changed rather than the adapter being excused.
- **AC-07 — The edge developer keeps their runtime.** Every rule runs and passes on
  the constrained single-threaded target, in the same run the rest of the gate uses,
  rather than as a separately maintained subset.
- **AC-08 — The evaluator can check the compliance claim instead of trusting it.**
  The evidence behind "DCB-compliant" is reachable from public artefacts and states
  which implementations it was checked against and when.
- **AC-09 — The evaluator can read what the library promises about positions and
  gaps before depending on it**, in plain language, without reading the source.
- **AC-10 — The evaluator can tell how strong each promise is.** Every clause a
  consumer might rely on is legibly frozen, provisional with a named falsifier, or
  deliberately deferred, and that state is accurate at the moment of publish.
- **AC-11 — A consumer's build is not broken by surprise.** The published surface has
  been diffed against its own prior published baseline, and the version number
  reflects what that diff found.
- **AC-12 — A consumer knows which compiler they need, and why it moved.** The MSRV
  is stated as a promise with a recorded justification, not as an incidental build
  setting.
- **AC-13 — Nobody has to guess what happens at a store boundary.** Replication
  identity has a written answer on file — a decision or a reasoned refusal — and the
  question atom reflects its resolved state.
- **AC-14 — Nobody has to guess what happens to a log with holes.** What a store may
  forget, and how a reader learns it happened, has a written answer on file under the
  same bar.
- **AC-15 — The next initiative inherits this one's audience.** The personas and
  journeys above exist as durable product-layer atoms, carrying the qualification
  that their evidence is secondary.

## Definition of Done

Each of these is run and observed to pass on the assembled library, from a clean
checkout, before this initiative is called shipped. None may be left pending, and a
green gate is a precondition for looking at these, never a substitute for them.

1. **@smoke — the worked example runs end to end.** `cargo run -p course-subscriptions`
   completes the canonical DCB cycle and prints the outcomes the specification's
   observable cases describe, with no `todo!()` reached.
2. **@smoke — the compiler protects the domain.** The compile-fail case for an
   unhandled event variant is present and green: it fails to compile with the
   expected diagnostic, and removing the protection makes the case fail.
3. **The durable store passes the suite for real.** `event_store_conformance!`
   is green against the durable adapter's fixture, including the concurrency case at
   64 contenders and an acknowledged write surviving a process reopen.
4. **The constrained-runtime store passes the suite on its own target.** Every rule
   is green under the edge runtime on `wasm32`, executed in the gate rather than
   asserted in prose, and its error type is shown either to carry what the caller
   needs or demonstrably not to.
5. **A store that does not serialise its writers passes the suite**, with the
   position-visibility cost measured rather than estimated.
6. **A store with no connection, no interactive transaction and no cursor passes the
   suite**, or the contract is amended by decision record and the suite re-run.
7. **The projection suite discriminates.** Two structurally unlike batch shapes pass
   it, and a deliberately wrong implementation that writes a checkpoint without its
   read model *fails* it, by name.
8. **The freeze verdict is written.** After the unlike batch shape exists, a written
   verdict states whether the `ProjectionStore` freeze held, either way, with what it
   was checked against.
9. **@smoke — a stranger can install it.** From a scratch project outside this
   workspace, adding the published crate from the registry and running the smallest
   write-then-read cycle succeeds against the published version, not a path
   dependency.
10. **The published crate looks finished.** The rendered documentation build is green
    under all features, and the registry page carries licence, description and README
    as rendered, checked by looking at them.
11. **The release is diffed, not asserted.** The public-surface comparison runs
    against the prior published baseline and its report is recorded; the version
    chosen matches what it found.
12. **The clause ledger is audited at publish.** A run over the specification reports
    every clause's maturity, and no clause is provisional with an empty falsifier;
    the count in the report matches `spec/SPECIFICATION.md`'s own stated figure.
13. **The gate is green on the assembled whole.** `cargo xtask ci` passes, including
    the specification cross-reference step, on the exact tree that was published.
14. **Replication has an answer on disk.** An accepted decision atom answers whether
    ingest re-checks a writer's asserted conditions — or explicitly refuses, with
    reasons — the corresponding open-question atom reflects that resolution, and
    `redkiln validate --kb` passes.
15. **Incomplete logs have an answer on disk**, under the same bar: a store that holds
    only a suffix of its own log is exercised against a reader, and the reader either
    fails loudly or the refusal to define this is recorded as a decision.
16. **The audience is durable.** Persona and journey atoms exist under `.kb/product/`
    with valid frontmatter and pass validation, and the initiative's closeout links
    them.

## Open design tensions

`userFacing` is false and no `interaction-patterns.md` was produced — deliberately,
at the intake gate. But the public API surface *is* a designed surface here, reviewed
at planning's design stage under the repurposed `_design.md`, and these are the
consumer-facing choices that stage must settle. Owning projects are named
descriptively; `/redkiln:plan` assigns the real ids, and each resolution is recorded
in that project's `_design.md`.

| ID | Tension | Options | Trade-off | Owning project |
| --- | --- | --- | --- | --- |
| DT-1 | Which claim leads on first contact — contract-first proof, or the constrained-runtime audience | (a) lead with storage-agnostic proof; (b) lead with the edge story; (c) two entry points, one per audience | The two pull toward different first impressions; leading with the edge story narrows the perceived audience, leading with the proof asks the reader to care about a claim they cannot yet check | Publication & positioning |
| DT-2 | How much a caller must state before their consistency boundary is checked | (a) minimal ceremony, more caught later; (b) explicit declaration, more caught at build time | Error timing versus first-hour cost; the audience is new to the language's idioms as often as not | Typed layer & worked example |
| DT-3 | How a consumer learns a guarantee does not apply to their implementation | (a) reported in the suite's own output; (b) documented per adapter; (c) both, with one authoritative | A silent skip is the documented failure mode; two sources of truth is the other one. CF-40's ownership is already an open atom | Conformance & projection freeze |
| DT-4 | Where the promise about positions and gaps is stated for a newcomer | (a) on the crate landing page; (b) in the specification only; (c) both, with the landing page pointing | Two DCB stores already disagree publicly; a reader who never finds the statement depends on the wrong one by accident | Publication & positioning |
| DT-5 | Whether the clause maturity vocabulary is published to consumers or kept internal | (a) publish the ledger; (b) publish only frozen guarantees; (c) publish a summary that links the ledger | Publishing it is unusually honest and unusually noisy; not publishing it hides exactly the signal an evaluator wants | Publication & positioning |
| DT-6 | Whether to state an explicit comparison to the nearest live peers | (a) state it; (b) let the proof stand alone | Silence on a peer that shipped the day before intake can read as an oversight; comparison invites maintenance and dates badly | Publication & positioning |
| DT-7 | How a reader is told the log it is reading is incomplete, and why | (a) one undifferentiated signal; (b) distinguish transient, benign-permanent and meaningful-permanent | A shipped peer framework found one undifferentiated signal insufficient in production; distinguishing costs the reader complexity | Replication & retention answers |
| DT-8 | Whether the adapter author addressed by the acceptance criteria is only this project's own, or an outside author too | (a) internal only for now; (b) hold the bar for an external author | Every precedent shows a suite's own permissiveness becomes publicly scrutinised the moment outsiders make claims with it | Conformance & projection freeze |

## Risks

| Risk | Likelihood / Impact | Mitigation |
| --- | --- | --- |
| The port is frozen against implementations that are too alike, and the abstraction turns out to be unearned | Medium / High | The freeze is not called earned until a structurally unlike batch shape has passed, and the verdict on whether it held is written after that, not at the freeze |
| "Passed against N adapters" is treated as durable proof when it is a snapshot | Medium / High | Published confidence is scoped to what was actually checked and when; two long-lived precedents carried undetected drift for years after wide adoption |
| A skeleton is mistaken for an adapter because it compiles | Medium / High | Nothing counts toward an outcome until it has *passed* the suite; a `todo!()` body type-checks against any signature and that is stated in the criteria |
| The constrained-runtime flavour is quietly dropped under schedule pressure | Low / High | It is a binding constraint and a Definition-of-Done scenario on its own target, not a compatibility note |
| A release ships a breaking change that nobody could see in review | Medium / High | The surface is diffed against its own published baseline and the version follows the diff; this is the instrument that did not exist before a baseline did |
| The MSRV promise is made without deciding what it means, and is contested later | Medium / Medium | It is stated with a recorded justification at first publish; the ecosystem norm is genuinely unsettled, so a position is taken rather than inherited |
| An open question gets settled in passing rather than by decision | Medium / High | Every answer lands as a decision atom stating the alternatives that lost; the corresponding open-question atom is resolved, never deleted |
| The nearest live peer moves and the positioning goes stale or reads as an oversight | Medium / Medium | Positioning is an owned design tension (DT-1, DT-6) with a decision attached, not a default |
| The audience most differentiated for is the one the ecosystem's own defaults steer away from | High / Medium | The property is explained actively rather than assumed legible; comparisons will default to the common case |
| The decomposition drifts back toward the reverted three-initiative split | Medium / Medium | Planning re-derives from grounding; the reverted bodies are prior art for seams only |
| The product layer stays empty and the next initiative re-derives its audience | Medium / Low | Promotion of personas and journeys is an acceptance criterion and a Definition-of-Done scenario, owned at closeout |

## Open questions for the planning team

Carried forward deliberately. None is to be settled in passing, and several are
already atoms under `.kb/open-questions/` that must be *consumed* rather than
rediscovered.

- **Which of the two lead opportunities carries the headline at publish** —
  contract-first proof or the constrained-runtime audience? They pull toward
  different first impressions (DT-1).
- **Of the two silent-industry questions, which is answered first** if both cannot
  be fully resolved inside this initiative's boundary — replication identity, or
  incomplete-log semantics?
- **Is the answer to "why not reject storage-agnosticism the way the nearest peer
  did" a documentation question or a decision-record question** owned by the
  projection port freeze itself?
- **Is the evaluator a persona in its own right, or an earlier stage of the
  application author's journey?** This changes what gets promoted at closeout.
- **Does the "honest answers" outcome fully cover the cross-persona gap** that every
  persona shares — what happens when a read is independent of the write that
  produced it — or is that a fifth, unnamed concern?
- **Is the adapter author's bar held for an outside author, or only this project's
  own, within this timeframe?** (DT-8.) It changes what the acceptance criteria have
  to survive.
- **The constrained-runtime persona's fear of orphaned tooling has no proof artefact
  among the outcomes.** Is that an intentional scope cut or a gap in how the outcome
  was framed?
- **How do the runbook's phases map onto projects and stories** — whether a phase is
  a project, whether several collapse, and where the release boundaries fall? This is
  explicitly planning's, and the ordering constraints at `RUNBOOK.md:244-258` are
  binding inputs to it rather than suggestions.
- **Which provisional clauses actually get frozen at publish and which are demoted?**
  The audit is against the ledger, not against prose.
- **On discovery coverage:** all nine approved research angles, all three grounding
  passes and both distillation passes were written and are cited below. The only
  absent discovery artefact is `interaction-patterns.md`, which was deliberately not
  commissioned because `userFacing` is false — its concerns surface here as the
  public-API-surface tensions in *Open design tensions*, resolved at the design stage
  rather than at discovery.

## Context anchors

Discovery corpus:

- [`_discovery/research/01-dcb-implementations-and-competing-event-stores.md`](_discovery/research/01-dcb-implementations-and-competing-event-stores.md)
- [`_discovery/research/02-rust-event-sourcing-crate-landscape-and-sentiment.md`](_discovery/research/02-rust-event-sourcing-crate-landscape-and-sentiment.md)
- [`_discovery/research/03-conformance-testkit-as-a-shipped-product.md`](_discovery/research/03-conformance-testkit-as-a-shipped-product.md)
- [`_discovery/research/04-storage-agnostic-ports-that-survived-a-second-database.md`](_discovery/research/04-storage-agnostic-ports-that-survived-a-second-database.md)
- [`_discovery/research/05-position-visibility-in-postgres-and-distributed-logs.md`](_discovery/research/05-position-visibility-in-postgres-and-distributed-logs.md)
- [`_discovery/research/06-replication-and-position-identity-across-store-boundaries.md`](_discovery/research/06-replication-and-position-identity-across-store-boundaries.md)
- [`_discovery/research/07-retention-deletion-and-incomplete-log-semantics.md`](_discovery/research/07-retention-deletion-and-incomplete-log-semantics.md)
- [`_discovery/research/08-rust-publication-semver-and-msrv-discipline.md`](_discovery/research/08-rust-publication-semver-and-msrv-discipline.md)
- [`_discovery/research/09-wasm32-edge-and-non-send-async-ecosystem.md`](_discovery/research/09-wasm32-edge-and-non-send-async-ecosystem.md)
- [`_discovery/grounding/product-functional-alignment.md`](_discovery/grounding/product-functional-alignment.md)
- [`_discovery/grounding/backlog-adjacency.md`](_discovery/grounding/backlog-adjacency.md)
- [`_discovery/grounding/vocabulary-and-conventions.md`](_discovery/grounding/vocabulary-and-conventions.md)
- [`_discovery/distillation/opportunities.md`](_discovery/distillation/opportunities.md)
- [`_discovery/distillation/personas-and-journeys.md`](_discovery/distillation/personas-and-journeys.md)
- [`_intake-brief.md`](_intake-brief.md) — the approved intent, constraints and clause ledger

Knowledge base:

- `.kb/maps/domain-map.md` (lines 37-142 — the two domains this initiative pushes from specified to proven-by-contact)
- `.kb/maps/decision-map.md`, `.kb/maps/open-questions-index.md`
- `.kb/decisions/0006-bare-name-to-the-typed-layer.md` — why the installable crate is the typed layer
- `.kb/decisions/0029-msrv-raised-to-1-97-1.md` — the MSRV as a provisional promise
- `.kb/concepts/torn-reads-and-the-append-condition-boundary.md`
- `.kb/open-questions/ps-1-states-no-progress-obligation.md`
- `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`
- `.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md`
- `.kb/open-questions/cf-40-fixture-limits-ownership.md`
- `.kb/open-questions/projection-store-batch-has-no-apply-seam.md`
- `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`
- `.kb/product/README.md`, `.kb/design/README.md` — the empty layers this closeout populates

Specification, plan and code:

- `spec/SPECIFICATION.md:215-221` — 200 clause IDs, 198 normative: 139 frozen, 49 provisional, 10 deferred
- `RUNBOOK.md:38-40` — sequencing by blast radius, and the proof-artefact bar
- `RUNBOOK.md:145-163` — the phase status table and its proof artefacts
- `RUNBOOK.md:244-258` — what must not be parallelised
- `RUNBOOK.md:262-284` — the decision-record queue
- `crates/happenstance/src/lib.rs` — the facade this initiative replaces with a used contract
- `examples/course-subscriptions/` — the worked example
- `references/seeds/remaining-runway.md:68-91` — the approved vision and audience
- `references/seeds/remaining-runway.md:125-131` — decomposition explicitly left to planning
- `CLAUDE.md` — binding constraints and the rule that matters
- `.redkiln/config.yaml:5` — support routing for incidental work

## Companions

Board-invisible drill-down for this card:

- [`_discovery/`](_discovery/) — the full discovery corpus: nine research angles, three grounding passes, two distillation passes
- [`_intake-brief.md`](_intake-brief.md) — the approved intake, its constraints and its clause ledger
- [`_decomposition.md`](_decomposition.md) — the project decomposition, authored later by `/redkiln:plan`
- [`_storymap.md`](_storymap.md) — the vertical-slice story map with dependencies and traceability, authored later by `/redkiln:plan`

## Assumptions

- The specification wins wherever this charter and it disagree, and a frozen clause
  changes by a new decision record rather than by edit.
- The clause figures quoted here are the specification's own count of clause **IDs**
  (139 / 49 / 10), not a count of marker occurrences; the two disagree and the
  distinction is recorded below.
- The runbook's ordering constraints remain binding inputs to planning even though
  the phase-to-project mapping is planning's to make.
- The audience evidence is secondary — download counts, public posts, issue threads,
  an adjacent project's postmortem — and no persona here has been directly observed.
  That qualification travels with them into the product layer.
- The comparison landscape is thin but active; anything positioned against a named
  peer will need re-checking at closeout rather than being assumed still true.
- Nothing downstream is currently pinned to this library, so the MSRV and public
  surface are still free to move — up to the moment of first publish, and no further.
- Incidental defects surfaced by this work route to the `support` initiative rather
  than expanding this one's scope.

## Exit criteria for this initiative

This initiative is finished when all of the following are true at once:

1. Every acceptance criterion AC-01 through AC-15 is met, and every Definition-of-Done
   scenario has been run and observed to pass on the assembled library.
2. `0.2.0` is published, and a project outside this workspace can install it from the
   registry and complete a write-then-read cycle against the published version.
3. No clause a consumer can rely on is provisional with an empty falsifier at the
   moment of publish, and the maturity of every other clause is accurate.
4. The `ProjectionStore` freeze has a written verdict against a structurally unlike
   batch shape — held or did not hold, stated either way.
5. Replication identity and incomplete-log semantics each have an accepted decision
   atom or an explicit, reasoned refusal on file, with the corresponding
   open-question atoms resolved rather than deleted.
6. Every design tension DT-1 through DT-8 has a recorded resolution in its owning
   project's design artefact; none is left unowned.
7. The personas and journeys this work served exist as durable product-layer atoms,
   and `redkiln validate --kb` and `redkiln doctor` are clean.
8. Every project is closed out, the whole gate is green on the published tree, and
   what was learned has been harvested rather than left in the backlog.

## Clarifications resolved during intake

- **The seed's `ProjectionStore` probe was stale.** `grep -rn "ProjectionStore for"`
  now matches six sites, all `todo!()`-bodied skeletons. The seed's *argument* stands
  unchanged — a port with no passing implementation is a guess — but the probe does
  not, and nobody should re-derive it.
- **The disputed clause counts are reconciled.** `spec/SPECIFICATION.md:219-221`
  states 200 clause IDs, 198 normative — 139 frozen, 49 provisional, 10 deferred,
  two non-normative. The larger figures from a raw marker grep count marker
  *occurrences*, not clauses. Quote the specification.
- **Decomposition is out of scope here, by decision.** Whether a phase is a project
  and where the release boundaries fall belongs to `/redkiln:plan`.
- **`userFacing` is false and the standing interaction-pattern angle was deliberately
  not appended.** The public API shape is still reviewed — at the planning design
  stage, which is a different gate.
- **Tier is standard**, scored 12/12 against the six-axis rubric with every axis at 2
  and each score carrying cited evidence; nine research angles were approved.
- **The port-freeze axis is named.** `ProjectionStore`'s is batch shape and
  transactional seam, and the far ends already exist in the workspace as skeletons —
  a graph-shaped batch, and a store with no connection, no interactive transaction
  and no cursor. For the already-frozen `EventStore`, the axis is who assigns
  positions and when, and its far end has not been built either.
- **CF-40's ownership is already an accepted open-question atom**
  (`.kb/open-questions/cf-40-fixture-limits-ownership.md`) and is consumed, not
  rediscovered.
- **Reactive work routes to `support`.** Bugs surfaced during this initiative's own
  discovery, planning or implementation go there per `.redkiln/config.yaml:5`.

## Related initiatives

- [`.bklg/support/initiative.md`](../support/initiative.md) — the standing support
  lane; incidental defects found during this work route there rather than expanding
  this initiative's scope.
- **Superseded prior art, not live items.** The ground covered here was once
  hand-split into three initiatives — `alpha-0-2-0` (HS-I0002), `release-0-2-0`
  (HS-I0003) and `adapters-and-replication` (HS-I0004) — and reverted wholesale at
  commit `0269720` for being hand-authored rather than process-produced, not for
  being wrong about the ground. They exist only in history and are useful as
  candidate seams for planning to weigh, never as a decomposition to copy.
