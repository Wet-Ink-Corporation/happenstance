---
item: HS-S0075
stage: spec
created: 2026-08-12T13:47:13.251Z
updated: 2026-08-12T13:47:13.251Z
template_sig: 87bbf1d0
rendered_sig: 2baf20fb
---

# Spec — ADR-0025: checkpoint placement, mutation vocabulary, blocking bridge

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/from-contract-to-published-library/initiative.md`](../../initiative.md) — Goals, BR-03, BR-04, **BR-15** (the requirement this story carries), DoD 7/8 |
| Initiative decomposition | [`.bklg/from-contract-to-published-library/_decomposition.md`](../../_decomposition.md) — traceability matrix, dependency DAG |
| Project charter | [`.bklg/from-contract-to-published-library/ladybug-projection-store/project.md`](../project.md) — AC-002, **DR-6**, DR-7, project DoD 3 |
| This spec | `.bklg/from-contract-to-published-library/ladybug-projection-store/adr-0025-three-answers/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — architecture brief **§5 "ADR-0025's three questions, and what already constrains each"** (the binding source for this story), **§6 T1/T2** (which port shape the answers are written against), **M8** (how a KB atom is authored), **§9** (standing prohibitions); testing brief's **AC-002 row** (what "accepted" is checked by) |
| Design sign-off | [`../_design.md`](../_design.md) — **no user-facing surface**, approved 2026-08-12; this story renders none and adds none |
| Grounding | [`../_grounding.md`](../_grounding.md), [`../_intake-brief.md`](../_intake-brief.md) — the three open questions as intake stated them |
| Story map row | [`../_storymap.md`](../_storymap.md) — milestone `preflight-and-decisions`, row `adr-0025-three-answers` |
| Roadmap pointers | `RUNBOOK.md:304` (the ADR queue row), `RUNBOOK.md:497` (the projections open-question row, status `open`), `RUNBOOK.md:4407` ("Decisions it settles. ADR-0025"), `RUNBOOK.md:4415-4419` (the phase-11 work checkbox stating the three questions verbatim) |

## One-line PR slice

Author ADR-0025 through `.kb/_intake/` — checkpoint placement, graph-mutation vocabulary,
blocking-API bridge — each question with its losers named, plus the long-form record and the
decision-map entry.

## Executive summary

This PR turns three questions that today live as **prose bullets in a skeleton's module
documentation** into one immutable, cited decision.

The delta, precisely. Today the three questions exist in three places and bind nothing:
`crates/happenstance-ladybug/src/lib.rs:51-66` lists them under `# Open decisions`;
`crates/happenstance-ladybug/src/projection_store.rs:39-47` records what the store's layout leaves
open for the blocking one; and `RUNBOOK.md:497` carries the single row `| Ladybug checkpoint
placement; how a projection expresses graph mutations; \`lbug\`'s blocking API | 11 | open | 0025 |`.
No `.kb/decisions/0025-*.md` exists — the corpus holds seventeen atoms, ADR-0001 – ADR-0016 and
ADR-0029, and 0017–0028 are reserved numbers with no files.

After this PR: a staged intake document and a long-form record under `references/adr/`, ingested by
`/redkiln:kb-ingest` into an accepted decision atom with valid `KbFrontmatter`; a row in
`.kb/maps/decision-map.md` that makes it reachable; and the two `RUNBOOK.md` queue rows moved from
*open* to *written*. The next story in the merge order (`real-lbug-driver-swap`) deletes
`lib.rs:51-66` and points at the atom instead — which is why this story is foundation and merges
before it.

What this PR does **not** land: any Rust. No body is filled in, no dependency is added, no test
runs. The ADR is the constraint the bodies are written against, authored first exactly as
`RUNBOOK.md:4407` schedules it and as every prior phase in this repository has done (`RUNBOOK.md:4059`,
`:3952`, `:4606`: "written before the code they constrain").

## Context pack

Read this section before opening anything. Everything deeper is a signposted anchor.

**1. The unit of work is three answers, not one record.** DR-6 ([`../project.md`](../project.md),
Derived requirements) is explicit: ADR-0025 states the alternatives that lost and why *for each of
its three questions, not once for the whole record*. A record with one `## Alternatives rejected`
section covering all three fails this story even if every sentence in it is true. The reason is
mechanical rather than stylistic — an accepted decision atom is immutable
(`.kb/decisions/README.md`, *The immutability rule*), so it is the **only durable record of a
rejected option**, and a loser that was never named cannot be recovered later without a second ADR.

**2. Q1 — checkpoint placement is nearly settled, and the ADR's job is to say *why* it is settled,
not to re-open it.** The checkpoint lives **in the graph as a node property**, because that is what
keeps its write inside the same `BEGIN TRANSACTION` as the read-model write, which is PS-1 — the
one invariant the port has this shape for (`crates/happenstance-core/src/projection.rs:13-30`;
argued at `crates/happenstance-ladybug/src/lib.rs:53-58`). **The loser is the sidecar** — a file or
a table outside the graph — and it loses on PS-1, not on preference. The statement is already
written in the instrument:
`MERGE (c:ProjectionCheckpoint {id: $id}) SET c.position = $position`
(`crates/happenstance-ladybug/src/live_handle.rs:203-217`), carrying the `i64::try_from` narrowing
that `PositionOutOfRange` exists for. Two residues are genuinely open and are the ADR's real
content: the `INT64` ↔ `NonZeroU64` narrowing in **both** directions (`MalformedCheckpoint` and
`PositionOutOfRange`, `projection_store.rs:175-202` — collapsing a corrupt checkpoint into
`Ok(None)` "would silently replay a projection from the beginning", so neither variant may be
simplified away), and whether the checkpoint is one node per `ProjectionId` or one node with a
property per id. **That second residue must not be decided here as a port question.** PS-23 asks
whether one `commit` may advance two ids (`spec/SPECIFICATION.md:5317`); that clause is
`projection-store-freeze`'s, and this adapter is a data point for it
(`spec/SPECIFICATION.md:4814`). Decide the adapter's schema; report to PS-23.

**3. Q2 — the vocabulary is raw parameterised Cypher, and the ADR is one of two data points the
port is waiting on.** `GraphStatement` is the raw-Cypher answer and it is deliberately *the port's*
answer too: PS-9 and PS-11 ask whether generic code needs a write vocabulary on `Batch`, "and this
crate is one of the two data points" (`lib.rs:63-66`; clauses at `spec/SPECIFICATION.md:4947`,
`:4977`). Parameters are carried **beside** the text rather than interpolated into it, because
replay goes through `prepare`/`execute` and a stringified statement cannot be prepared once and
executed many times (`projection_store.rs:70-80`). Two constraints bind the answer before any
weighing starts. **ADR-0008** (`.kb/decisions/0008-one-derivation-for-both-ports.md`): a provided
body cannot hold the `Batch` GAT across a suspension point *under any remedy tried* — that rules
shapes out rather than ranking them. And **ADR-0007**
(`.kb/decisions/0007-projection-runner-decodes.md`) carries an indicative `Projection::apply`
signature taking `&mut …::Batch<'_>`; the runner is `typed-layer-and-alpha-release`'s, and a
vocabulary chosen here that the runner cannot call is a bill that arrives there. **Losers to name:**
a typed builder (state what it buys, and what it costs a projection needing Cypher the builder does
not model), and the three apply-seam shapes — a bound on `Batch`, a method on `ProjectionStore`, a
second associated type — which are sub-question 1 of
`.kb/open-questions/projection-store-batch-has-no-apply-seam.md`. **That open question is consumed
as evidence, not resolved** (DR-7): its resolution is phase 6's, and this ADR reports whether the
seam that shipped survived a third implementer.

**4. Q3 — how a blocking driver meets a non-blocking port is the crux, and it cannot be hidden.**
Every `lbug` method blocks; every port method is `async`. Because the store is `'static` and owns
its `Database`, `tokio::task::spawn_blocking` is *available* — a connection can be built inside the
closure — "but it is not used, because reaching for it would put a tokio dependency in a
runtime-agnostic adapter" (`projection_store.rs:39-47`). Three candidates, each owed a real cost:
a runtime-gated `spawn_blocking` feature; blocking the executor thread and documenting it; offering
a blocking-only adapter. Two hard constraints frame the choice and one non-argument must be
retired. **`#[async_trait]` is never an option** — it injects `+ Send` and makes the wasm32 target
impossible (ADR-0001, `.kb/decisions/0001-async-port-flavours.md`; CLAUDE.md binding constraint 1).
The testkit already ships `__emit_blocking`, a `#[test]` + `block_on` emitter needing **nothing**
from the adapter (`crates/happenstance-testkit/src/lib.rs:60-67`), so *"the suite needs tokio"* is
not an argument for any of the three. And `Cargo.toml`'s existing tokio **dev**-dependency is not a
precedent for a normal one. The intake brief states the stake plainly: whatever bridges the two
"cannot be hidden in an adapter's private helper — if the port cannot express it, that is the
freeze not holding" ([`../_intake-brief.md`](../_intake-brief.md), Constraints).

**5. The `Send` flavour is not one of the three questions.** It is already decided, on evidence:
`lbug` writes `impl Send` and `impl Sync` for both `Database` and `Connection`
(`projection_store.rs:252-257`). BR-12's `!Send` proof belongs to
`cloudflare-durable-object-store`. Do not re-argue it, and do not let Q3's answer quietly reopen it.

**6. The atom is authored by `/redkiln:kb-ingest`, never by hand.** This story writes a staged
document under `.kb/_intake/` and a long-form record under `references/adr/`, and hands off. Hand-writing
`.kb/decisions/0025-*.md` "produces the directory layout of the process without the process, which
is why the first attempt at this was reverted (`0269720`)" (CLAUDE.md, *Where the work lives*).
Three mechanics that have already cost time here: the ingest glob **includes
`.kb/_intake/README.md`** — drop it from the wave at the approval gate, because it is a README and
not raw material; give the wave an id that does not collide with `2026-08-10-intake` or
`2026-08-10-intake-2` under `.kb/_governance/integration-waves/`, or the new wave overwrites the
first's audit trail; and a successful ingest **clears `_intake/`** — a file still sitting there
after a run is a file that run did not ingest (`.kb/_intake/README.md`).

**7. A decision lives in two places on purpose.** The atom is canonical, ~100 lines, carrying the
frontmatter, status and supersession graph `redkiln validate --kb` enforces. The long-form record
under `references/adr/` carries what a summary cannot hold — the per-question alternatives, the
compiler evidence, the citations `spec/SPECIFICATION.md` will resolve by `file:line`. **Link the
atom; cite the record by `file:line`** (CLAUDE.md). `references/adr/0016-the-wire-format.md` is the
shape to copy — `## The question…`, `## Context`, `## Decision`, `## Consequences`,
`## Alternatives rejected`, `## What this ADR leaves open`, `## Amendments this decision owes the
specification`.

**8. The answers are written against what actually merged, not against the sketch.** This story
depends on `preflight-and-unlike-axes`, whose whole job is to assert the merged port really shipped
`type Batch;` (no lifetime), the write seam, and `projection_store_conformance!`. **If that
preflight came back red, this project halted and this story does not start** — the answer is
surfaced to `projection-store-freeze` (HS-P0010), not worked around locally
([`../_decomposition.md`](../_decomposition.md), AC-A01; [`../_storymap.md`](../_storymap.md), Merge
order, gate 1). Which shape merged changes Q2's answer materially: with the GAT gone, ADR-0008's
"cannot hold the batch across a suspension point" reads differently, and
`port_shape.rs`'s `for<'a> S::Batch<'a>: Send` collapses to `S::Batch: Send`. The ADR must **name
the port shape it was written against and the commit it read it at**, so a later reader can tell
whether the reasoning still applies.

**9. The mount is a `RUNBOOK.md` edit with a line-number hazard attached.** `spec/SPECIFICATION.md`
carries thirteen `RUNBOOK.md:<line>` citations naming ten distinct anchors (68, 204-208, 438-439,
476, 494, 1786-1790, 2870-2876, 3126-3137, 3841-3846, 3953-3955), and `cargo xtask spec-trace`
resolves every one. Both rows this story edits are **single-line markdown table rows**, and both are
*above* five of those anchors — an insertion at `RUNBOOK.md:304` drifts eight of the ten, one at
`:497` drifts five. So the rule is **edit in place, line-count-neutral**: replace the row text,
never add a line. `spec-trace` alone will not protect you, because it allows `ANCHOR_SLACK` of 12
lines (`xtask/src/spec_trace.rs:391`) — a small drift passes green while being wrong. The settled
rows show the in-place idiom: `RUNBOOK.md:288` and `:485` are long, and each is still one line.

**10. What this story is forbidden to settle.** It amends nothing `[FROZEN]` — that takes a new ADR,
not an edit (CLAUDE.md). It does not move PS-34's marker; this project *reports* on PS-34 and moving
the marker is `projection-store-freeze`'s (`RUNBOOK.md:602`). It does not resolve PS-9, PS-11, PS-12
or PS-23, all of which are phase 6's clauses awaiting this adapter as a data point. It does not
decide the CI shape (`cold-build-cost-and-ci-shape`), whether the crate publishes at `0.2.0`
(`publication-and-positioning`), or PS-3. And it settles no incidental defect found in passing —
those route to the `support` initiative (`.redkiln/config.yaml:5`).

**11. Who this is for.** The persona is the adapter author meeting `ProjectionStore` for the third
time (`_storymap.md`, Backbone: "an adapter author meeting `ProjectionStore` for the third time, and
the reviewer who will be asked to believe the answer"). They read this atom *before* filling a body,
to know which of the three forks was already taken and why. The second reader is the reviewer of the
freeze verdict, who uses it to tell a decided choice from a defaulted one — a verdict resting on
choices nobody recorded is a verdict about an accident.

## Integration contract

- **Archetype**: `foundation`. It lands a real in-tree artefact — an accepted decision atom under
  `.kb/decisions/` — that the `real-adapter` and `freeze-verdict` slices consume by name. It is not
  a double, not a placeholder, and not terminal ([`../_storymap.md`](../_storymap.md), *Why the
  slices are cut here*).
- **Slice / milestone**: `preflight-and-decisions`. Slice-mate: **`preflight-and-unlike-axes`**,
  which merges **first** — it is this story's `depends_on`, and the axes wording depends on what
  actually merged upstream.
- **Mount point**: **`.kb/maps/decision-map.md`** — the knowledge base's composition root for
  decisions. An atom not on this map is an atom nothing indexes; AC-002 names reachability from it
  explicitly ([`../project.md`](../project.md), AC-002). The row is written by the **Maps phase of
  the `/redkiln:kb-ingest` wave**, not by hand, and it must carry the same columns every other row
  does — ADR number, atom link, title, status, phase, supersedes/superseded-by.
- **Secondary mounts** (each a real file with an existing shape, all edited in place):
  - `RUNBOOK.md:304` — the ADR queue row for **0025**, rewritten to the "written" idiom the settled
    rows use (`RUNBOOK.md:288`, `:280`), linking the atom.
  - `RUNBOOK.md:497` — the Projections open-questions row, status `open` → the answer, ADR column
    already `0025`.
  - `RUNBOOK.md:4415-4419` — phase 11's ADR-0025 work checkbox, ticked `- [x]` in place. **No other
    phase-11 box is this story's**; the remaining ones belong to `cold-build-cost-and-ci-shape` and
    `freeze-verdict-document` ([`../_storymap.md`](../_storymap.md), Coverage).
- **Wires into** (read, cited, never edited by this story):
  - `.kb/decisions/0001-async-port-flavours.md` — why `#[async_trait]` is excluded from Q3's field.
  - `.kb/decisions/0007-projection-runner-decodes.md` — the indicative `Projection::apply` signature
    Q2's answer must stay callable from.
  - `.kb/decisions/0008-one-derivation-for-both-ports.md` — the GAT-across-a-suspension-point
    finding that rules Q2 shapes out before they are weighed.
  - `.kb/open-questions/projection-store-batch-has-no-apply-seam.md` — consumed as evidence (DR-7),
    with its ordered sub-question 1 quoted as Q2's loser set.
  - `crates/happenstance-core/src/projection.rs` — the port and the PS-1 invariant on the trait.
  - `crates/happenstance-ladybug/src/lib.rs:51-66`, `src/projection_store.rs:39-64`, `:70-103`,
    `:175-213`, `:252-268`, `src/live_handle.rs:203-217` — the skeleton's own record of each
    question and the constraints already discovered.
  - `crates/happenstance-testkit/src/lib.rs:60-67` — the three emitters, which retire "the suite
    needs tokio" as a Q3 argument.
  - `spec/SPECIFICATION.md` — PS-1, PS-4, PS-9, PS-11, PS-12, PS-23, PS-34, read only.
- **Renders surfaces**: **none.** [`../_design.md`](../_design.md) records `N/A — no user-facing
  surface` for every block including `## Items` and `## Signatures`, approved 2026-08-12. This story
  adds no `pub` item, changes no signature, and writes no doctest. A story that renders a surface
  here would be contradicting a signed-off determination.
- **Conformance rule(s)**: **none, and this is not adapter-observable.** The story ships no Rust, so
  no rule in the projection suite can pass or fail because of it. That is honest rather than
  convenient: the ADR *constrains* the bodies `fill-the-bodies-and-ps-34-disposition` writes, and the
  rules that observe those bodies are `projection-store-freeze`'s and are named in that story. The
  machine checks that do bind here are `redkiln validate --kb`, `redkiln doctor` and
  `cargo xtask spec-trace`.
- **Clause(s)**: discharges none, amends none, moves no marker. It contributes **data points** to
  PS-9 and PS-11 (write vocabulary), PS-12 (read-your-own-writes, answered later by
  `read-your-own-writes-projection`), PS-23 (two ids in one commit) and PS-34 (the E0195 trap,
  dispositioned by `fill-the-bodies-and-ps-34-disposition`). Nothing `[FROZEN]` is touched
  (AC-008).
- **Advances DoD scenario**: project **DoD 3** directly — *"ADR-0025 is accepted on disk;
  `redkiln validate --kb` and `redkiln doctor` are clean"* ([`../project.md`](../project.md),
  Definition of done). Toward the initiative, it moves **DoD 8** ("The freeze verdict is written")
  toward green, because the verdict cites this atom for what the adapter decided rather than
  defaulted; and it unblocks the adapter work that DoD 7 depends on.

## PR boundary

**In this PR**

- The staged intake document for ADR-0025 under `.kb/_intake/`, answering all three questions with
  per-question losers.
- The long-form record `references/adr/0025-<slug>.md`, in the house shape
  (`references/adr/0016-the-wire-format.md` is the model).
- The `/redkiln:kb-ingest` handoff and its wave outputs: the accepted atom
  `.kb/decisions/0025-<slug>.md`, the `.kb/maps/decision-map.md` row, the
  `.kb/_governance/integration-waves/<wave-id>` record, and the clearing of the staged file. These
  are produced **by the ingest run** on its own worktree branch and are listed in the boundary only
  so that `redkiln verify --grain story` does not fail on a legitimate file once that branch is
  merged — **never hand-write them**.
- The three `RUNBOOK.md` in-place edits named in the Integration contract.
- This story's own backlog folder (its ledger and report).

**Explicitly not in this PR**

- Any change under `crates/**`. No `todo!()` is filled, no `stand_in` type is re-pointed, no
  dependency is added, `lib.rs:51-66` is *not* rewritten — that is `real-lbug-driver-swap`'s first
  act and it is deliberately downstream of this atom.
- Any change to `spec/SPECIFICATION.md`. The six citation repairs are
  `fill-the-bodies-and-ps-34-disposition`'s and are required in the same commit as the bodies
  ([`../_decomposition.md`](../_decomposition.md), M7); this story must leave the file byte-identical
  and keep `spec-trace` green by not drifting `RUNBOOK.md`.
- Any other `RUNBOOK.md` phase-11 checkbox, the build-cost number, and the CI-shape decision.
- Resolving `.kb/open-questions/projection-store-batch-has-no-apply-seam.md` or
  `.kb/open-questions/cf-40-fixture-limits-ownership.md`, or editing any existing accepted atom's
  body.

**Merge DoD**: ADR-0025 is accepted on disk with three answers and three loser sets, reachable from
`.kb/maps/decision-map.md`; `redkiln validate --kb`, `redkiln doctor` and `cargo xtask ci`
(including `spec-trace`) are green; and `RUNBOOK.md`'s two queue rows say *written* without moving a
single cited line number.

```
.kb/_intake/**
.kb/decisions/0025-*.md
.kb/maps/decision-map.md
.kb/_governance/integration-waves/**
references/adr/0025-*.md
RUNBOOK.md
.bklg/from-contract-to-published-library/ladybug-projection-store/adr-0025-three-answers/**
```

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **Q1 answered: the checkpoint is a node property, not a sidecar** | The decision states that placement inside the graph is what keeps the checkpoint write in the same `BEGIN TRANSACTION` as the read-model write, satisfying PS-1; the sidecar (a file or table outside the graph) is named as the loser and loses **on PS-1, not on preference**. The concrete statement and its `i64::try_from` narrowing are quoted from the instrument. | `crates/happenstance-ladybug/src/lib.rs:53-58`; `crates/happenstance-ladybug/src/live_handle.rs:203-217`; `crates/happenstance-core/src/projection.rs:13-30` |
| **Q1 residue recorded, not silently closed** | Both narrowing directions survive in the decision as required behaviour — `MalformedCheckpoint` (a corrupt stored `INT64`) and `PositionOutOfRange` (a `NonZeroU64` that will not fit `INT64`) — with the stated reason that collapsing a corrupt checkpoint into `Ok(None)` would silently replay a projection from the beginning. Neither may be simplified away by the story that fills the bodies. | `crates/happenstance-ladybug/src/projection_store.rs:175-202` |
| **Q1 stays inside the adapter's schema** | Whether the checkpoint is one node per `ProjectionId` or one node with a property per id is decided **for this adapter** and reported to PS-23 as a data point; the clause itself is not amended and no marker moves. | `spec/SPECIFICATION.md:5317` (PS-23); `spec/SPECIFICATION.md:4814` (this adapter named as the input) |
| **Q2 answered: raw parameterised Cypher, parameters beside the text** | `GraphStatement` is the answer, and the decision states why parameters are not interpolated: replay goes through `prepare`/`execute`, and a stringified statement cannot be prepared once and executed many times. | `crates/happenstance-ladybug/src/projection_store.rs:70-103` |
| **Q2 names its losers, plural** | The typed builder is named with what it buys **and** what it costs a projection needing Cypher the builder does not model; the three apply-seam shapes (bound on `Batch` / method on the port / second associated type) are named from the open question's sub-question 1; and ADR-0008's finding — a provided body cannot hold the `Batch` GAT across a suspension point under any remedy tried — is cited as ruling shapes out before weighing. | `.kb/open-questions/projection-store-batch-has-no-apply-seam.md` (Ordered sub-questions, 1); `.kb/decisions/0008-one-derivation-for-both-ports.md` |
| **Q2 stays callable from the runner that does not exist yet** | The chosen vocabulary is checked against ADR-0007's indicative `Projection::apply` signature and the decision states whether it survives as written; an incompatible vocabulary is a bill that arrives at `typed-layer-and-alpha-release`. | `.kb/decisions/0007-projection-runner-decodes.md` |
| **Q2 reports on the seam, it does not resolve it** | `.kb/open-questions/projection-store-batch-has-no-apply-seam.md` is consumed as evidence about whether the seam phase 6 chose survived a third implementer (DR-7). Its `status` is untouched and its resolution stays `projection-store-freeze`'s. | [`../project.md`](../project.md), DR-7 |
| **Q3 answered: one bridge chosen from three, each costed** | Runtime-gated `spawn_blocking` feature / block the executor thread and document it / blocking-only adapter. The decision names the chosen one and what the other two would have cost, and states plainly that the bridge is a public, documented property of the adapter — not a private helper. | `crates/happenstance-ladybug/src/projection_store.rs:39-47`; [`../_intake-brief.md`](../_intake-brief.md), Constraints |
| **Q3's two hard constraints are stated as constraints, not options** | `#[async_trait]` is excluded outright (it injects `+ Send`, killing wasm32); and "the suite needs tokio" is retired as an argument because `__emit_blocking` is a `#[test]` + `block_on` emitter needing nothing. The existing tokio **dev**-dependency is explicitly not a precedent for a normal one. | `.kb/decisions/0001-async-port-flavours.md`; `crates/happenstance-testkit/src/lib.rs:60-67`; `crates/happenstance-ladybug/Cargo.toml` |
| **The `Send` flavour is not reopened** | The decision records that the flavour is already settled on evidence (`lbug`'s `Database` and `Connection` are both `Send + Sync`) and that BR-12's `!Send` proof belongs to `cloudflare-durable-object-store`. | `crates/happenstance-ladybug/src/projection_store.rs:252-257` |
| **The atom is produced by the ingest process, not by hand** | The implementer stages one document under `.kb/_intake/`; `/redkiln:kb-ingest` (human-invoked) extracts, adjudicates and writes `.kb/decisions/0025-<slug>.md` with valid `KbFrontmatter` — `kind: decision`, `authority_tier: decision`, `adr_id: ADR-0025`, `status: accepted`, `phase: 11`, `supersedes: null`, `source_paths` citing both the intake file and the long-form record. Hand-authoring the atom is the failure `0269720` reverted. | `CLAUDE.md`, *Where the work lives*; `.kb/decisions/0008-one-derivation-for-both-ports.md` (frontmatter shape to copy); `.kb/decisions/README.md` |
| **The wave is run cleanly** | `.kb/_intake/README.md` is dropped from the wave at the approval gate (the glob includes it and it is not raw material); the wave id does not collide with `2026-08-10-intake` or `2026-08-10-intake-2`; and the run clears `_intake/`, so a file left behind means it was not ingested. | `.kb/_intake/README.md`; `.kb/_governance/integration-waves/` |
| **The long-form record carries what the atom cannot** | `references/adr/0025-<slug>.md` holds the per-question argument, the rejected alternatives in full, and the citations, in the house shape. The atom links it; the record is cited by `file:line`. Deleting the record because the atom exists discards the corpus. | `references/adr/0016-the-wire-format.md` (`## Context` / `## Decision` / `## Consequences` / `## Alternatives rejected` / `## What this ADR leaves open` / `## Amendments this decision owes the specification`); `CLAUDE.md` |
| **The atom is reachable** | `.kb/maps/decision-map.md` gains an ADR-0025 row in the existing column shape, written by the wave's Maps phase. | `.kb/maps/decision-map.md` |
| **`RUNBOOK.md` says *written*, without drifting a cited line** | `:304` and `:497` are rewritten in place as single-line table rows; `:4415-4419`'s box is ticked in place. Net line-count change to `RUNBOOK.md` is **zero**, because `spec/SPECIFICATION.md` carries thirteen `RUNBOOK.md:<line>` citations over ten anchors and `ANCHOR_SLACK` of 12 lines would let a small drift pass green while being wrong. | `RUNBOOK.md:288`, `:485` (the in-place idiom); `xtask/src/spec_trace.rs:391` |
| **The decision names the port shape it was written against** | The record states whether `type Batch;` or the GAT merged, and the commit it was read at, so a later reader can tell whether the reasoning still applies. If the preflight was red, this story does not start — the finding goes upstream to HS-P0010. | [`../_decomposition.md`](../_decomposition.md), AC-A01 and T1; [`../_storymap.md`](../_storymap.md), Merge order gate 1 |
| **Nothing frozen moves and no sibling's question is answered** | No `[FROZEN]` clause marker or text changes; PS-34's marker is not moved (this project reports, `projection-store-freeze` moves); PS-3, the CI shape and the publish decision are left to their owners. | `spec/SPECIFICATION.md` (unchanged in this PR); `RUNBOOK.md:602`; [`../project.md`](../project.md), Out of scope |

## Data and migrations

**N/A for schema and runtime data** — this story ships no Rust, no database schema, no persisted
runtime state, and no Cypher is executed. The graph schema Q1 discusses (`:ProjectionCheckpoint`
label, `id` and `position` properties) is *decided* here and *created* by
`fill-the-bodies-and-ps-34-disposition`; there is no existing store to migrate, because
`happenstance-ladybug` has never run.

One persistent-state change is worth naming because it is one-way. **The knowledge base gains an
immutable record.** An accepted decision atom is never edited — `redkiln validate --kb` checks each
one against `HEAD` and fails the gate on a changed body, and a correction is a *new* atom carrying
`supersedes` with the old one's frontmatter flipped to `status: superseded`
(`.kb/decisions/README.md`, *The immutability rule*). So the cost of a wrong or under-argued answer
here is ADR-0030, not an edit — which is the reason DR-6 insists the losers are named per question
while the argument is still in hand.

The staged input is deliberately transient in the opposite direction: a successful
`/redkiln:kb-ingest` run **clears `.kb/_intake/`**, and the atom keeps the `.kb/_intake/…` path in
`source_paths` so the source document remains recoverable from git history
(`.kb/_intake/README.md`).

## Acceptance criteria

The reader every criterion below is framed from is fixed by the story map and by
`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`:
**the adapter author** (Persona 2, `:114-179` — *"an executable definition of 'correct' they can run
against their own storage system, rather than a prose specification they have to interpret"*), here
meeting `ProjectionStore` for the third time; and **the evaluator / reviewer** (Persona 4, `:249-315`),
who will be asked to believe the freeze verdict and needs to tell a *decided* choice from a
*defaulted* one. Each criterion is a goal one of them holds, crossing the whole stack from the
skeleton's module docs to the atom they read.

The testing brief is explicit about the instrument: AC-002 of the project is proven by
`redkiln validate --kb` plus reachability from `.kb/maps/decision-map.md`, and *"content check (each of
Q1/Q2/Q3 names a rejected alternative and why) is a **review-tier** check, not automatable"*
([`../_decomposition.md`](../_decomposition.md), Testing brief, AC-002 row). The verification column
therefore names the real gate where one exists and a human read where none can — it never invents a
pass/fail rule that would be decorative (CLAUDE.md: *a rule that no adapter can fail is decorative*).

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | GIVEN an adapter author about to write `commit`, wondering whether the checkpoint may live in a sidecar file next to the graph, WHEN they open ADR-0025's Q1 section, THEN they find the checkpoint placed **in the graph as a node property** with the reason stated as PS-1 — it is what keeps the checkpoint write inside the same `BEGIN TRANSACTION` as the read-model write — and the **sidecar named as the loser, losing on that invariant rather than on preference**, with the concrete `MERGE (c:ProjectionCheckpoint {id: $id}) SET c.position = $position` statement quoted from the instrument that already wrote it. | Static: `redkiln validate --kb` over `.kb/decisions/0025-<slug>.md`. Review-tier: a human read of the Q1 section against `crates/happenstance-ladybug/src/lib.rs:53-58`, `crates/happenstance-ladybug/src/live_handle.rs:203-217` and the PS-1 invariant at `crates/happenstance-core/src/projection.rs:13-30`. A Q1 section that argues from convenience, or that names no loser, fails this AC even if its conclusion is right (DR-6, [`../project.md`](../project.md)). |
| AC-002 | GIVEN the same author filling in `checkpoint` and tempted to collapse a corrupt stored `INT64` into `Ok(None)` because it is one line shorter, WHEN they read Q1's residue, THEN both narrowing directions survive as **required** behaviour — `MalformedCheckpoint` and `PositionOutOfRange` — with the stated consequence that collapsing the corrupt case *"would silently replay a projection from the beginning"*; AND the checkpoint's schema shape (one node per `ProjectionId`, or one node with a property per id) is decided **for this adapter** and **reported to PS-23 as a data point**, with PS-23's clause text and marker untouched. | Static: `redkiln validate --kb`; `cargo xtask spec-trace` green with `spec/SPECIFICATION.md` byte-identical in this story's diff. Review-tier: human read against `crates/happenstance-ladybug/src/projection_store.rs:175-202` (both variants) and `spec/SPECIFICATION.md:5317` (PS-23) / `:4814` (this adapter named as the input). Downstream: the unit tests `fill-the-bodies-and-ps-34-disposition` writes for the two narrowing paths are the code-tier consequence, not this story's evidence. |
| AC-003 | GIVEN an adapter author who must express a graph mutation and cannot tell whether the port wants a builder or a string, WHEN they read Q2, THEN the vocabulary is **raw parameterised Cypher** carried as `GraphStatement` with **parameters beside the text, never interpolated into it**, and the reason is stated mechanically — replay goes through `prepare`/`execute` and a stringified statement cannot be prepared once and executed many times; AND the record states whether that vocabulary is still callable from ADR-0007's indicative `Projection::apply(&mut …::Batch<'_>)` signature, so that an incompatibility becomes a named bill for `typed-layer-and-alpha-release` rather than a surprise. | Static: `redkiln validate --kb`. Review-tier: human read against `crates/happenstance-ladybug/src/projection_store.rs:70-103` and `.kb/decisions/0007-projection-runner-decodes.md`. A Q2 section that states the vocabulary without the `prepare`/`execute` reason, or that never mentions the runner, is undischarged. |
| AC-004 | GIVEN a future reader who must re-derive why the port has no typed write vocabulary without re-running the exploration, WHEN they read Q2's alternatives, THEN **more than one loser is named and priced** — the typed builder with what it buys *and* what it costs a projection needing Cypher the builder does not model, and the three apply-seam shapes (a bound on `Batch`, a method on `ProjectionStore`, a second associated type) taken from sub-question 1 — with ADR-0008's finding cited as **ruling shapes out before they are weighed**; AND `.kb/open-questions/projection-store-batch-has-no-apply-seam.md` is **consumed as evidence and left unresolved**, its `status` untouched and its resolution still `projection-store-freeze`'s (DR-7). | Static: `redkiln validate --kb` — including the accepted-decision immutability check against `HEAD`, which is what catches an edit to `.kb/decisions/0008-…` or to the open-question atom's frontmatter. Review-tier: human read against `.kb/open-questions/projection-store-batch-has-no-apply-seam.md` (Ordered sub-questions, 1) and `.kb/decisions/0008-one-derivation-for-both-ports.md`. A single unpriced "we chose raw Cypher because it is simpler" fails DR-6. |
| AC-005 | GIVEN an application author who must know, before choosing this adapter, what a synchronous graph engine costs them inside an `async` runtime, WHEN they read Q3, THEN **one** bridge is chosen from the three real candidates — a runtime-gated `spawn_blocking` feature, blocking the executor thread and documenting it, or a blocking-only adapter — **each of the other two carrying what it would have cost**; AND the record states that whatever bridges the two is a public, documented property of the adapter, not a private helper, because *"if the port cannot express it, that is the freeze not holding"*. | Static: `redkiln validate --kb`. Review-tier: human read of Q3 against `crates/happenstance-ladybug/src/projection_store.rs:39-47` and [`../_intake-brief.md`](../_intake-brief.md), *Constraints*. A Q3 section naming three options and pricing one is a preference, not a decision. |
| AC-006 | GIVEN a reviewer checking that Q3's field was framed honestly rather than narrowed to reach a conclusion, WHEN they read the record, THEN `#[async_trait]` is excluded **outright as a constraint** (it injects `+ Send` and makes the wasm32 target impossible, ADR-0001), *"the suite needs tokio"* is **retired as an argument** because `__emit_blocking` is a `#[test]` + `block_on` emitter needing nothing from the adapter, the existing tokio **dev**-dependency is stated **not** to be a precedent for a normal one, and the `Send` flavour is recorded as **already settled on evidence** (`lbug`'s `Database` and `Connection` are `Send + Sync`) with BR-12's `!Send` proof left to `cloudflare-durable-object-store`. | Static: `redkiln validate --kb`; `cargo xtask affected --base main` green (no `crates/**` change, so nothing in the workspace can quietly acquire tokio as a normal dependency in this PR). Review-tier: human read against `.kb/decisions/0001-async-port-flavours.md`, `crates/happenstance-testkit/src/lib.rs:60-67`, `crates/happenstance-ladybug/Cargo.toml` and `crates/happenstance-ladybug/src/projection_store.rs:252-257`. |
| AC-007 | GIVEN a reader in a year who cannot tell whether this record still applies to the port they are holding, WHEN they open it, THEN it **names the `ProjectionStore` shape it was written against** — whether `type Batch;` (no lifetime), the write seam and `projection_store_conformance!` had actually merged — **and the commit it read them at**, so the reasoning can be re-checked rather than assumed; AND if the upstream preflight came back red, this story did not start and the finding was surfaced to `projection-store-freeze` (HS-P0010) instead of being worked around locally. | Process: the predecessor story `preflight-and-unlike-axes` (HS-S0074) merged green — `story.md` carries `blocked_by: [HS-S0074]` and the gate is [`../_storymap.md`](../_storymap.md), *Merge order* gate 1. Review-tier: the record names a real SHA resolvable with `git show`, and the shape it names matches `crates/happenstance-core/src/projection.rs` at that SHA ([`../_decomposition.md`](../_decomposition.md), AC-A01, T1). A record with no port-shape statement fails; a record naming a shape that never merged fails harder. |
| AC-008 | GIVEN an adapter author arriving cold who does not know the filename, WHEN the merge lands, THEN the decision has **mounted at the knowledge base's composition root** — an accepted atom at `.kb/decisions/0025-<slug>.md` with valid `KbFrontmatter` (`kind: decision`, `status: accepted`, `adr_id`, `phase`, `source_paths` citing both inputs), **authored by `/redkiln:kb-ingest` and never by hand**, linking a **retained** long-form record at `references/adr/0025-<slug>.md` in the house shape, carrying its row in `.kb/maps/decision-map.md` in the same column shape every other row uses, with `.kb/_intake/` emptied by the wave, the intake `README.md` dropped at the wave's approval gate, and a wave id that does not overwrite `2026-08-10-intake` or `2026-08-10-intake-2`. | Static: `redkiln validate --kb` (KbFrontmatter conformance **and** accepted-decision immutability against `HEAD`) and `redkiln doctor` clean, still reporting exactly six `template-drift` advisories (NF-003); `.kb/_intake/` empty after the wave (`.kb/_intake/README.md`); `test -f references/adr/0025-<slug>.md`; the `.kb/maps/decision-map.md` row present and resolving; a new record under `.kb/_governance/integration-waves/` that did not overwrite an existing one. Hand-authored atoms are the failure `0269720` reverted (CLAUDE.md). |
| AC-009 | GIVEN the reviewer who must confirm this record changed the plan of record without breaking the specification's citations, WHEN they read the diff, THEN `RUNBOOK.md:304` and `RUNBOOK.md:497` say *written* in the idiom the settled rows already use and `RUNBOOK.md:4415-4419`'s ADR-0025 box is ticked `- [x]`, **all three edited in place with a net line-count change of zero**; AND `spec/SPECIFICATION.md` is byte-identical, no `[FROZEN]` clause marker or text moved, and no phase-11 checkbox belonging to a sibling story was ticked. | Static: `cargo xtask spec-trace` green **and** `git diff --numstat -- RUNBOOK.md` showing equal insertions and deletions — `spec-trace` alone is insufficient because `ANCHOR_SLACK` is 12 lines (`xtask/src/spec_trace.rs:391`), so a small drift passes green while being wrong. `git diff -- spec/SPECIFICATION.md` empty. Review-tier: the two rewritten rows read against the settled idiom at `RUNBOOK.md:288` and `:485`, and the ticked box read against [`../_storymap.md`](../_storymap.md), *Coverage*, to confirm no sibling's box moved. |

**Coverage of the traced project AC.** [`../project.md`](../project.md)'s **AC-002** — *"ADR-0025 is
accepted and answers three questions, each with its losers named … `redkiln validate --kb` passes and
the atom is reachable from `.kb/maps/decision-map.md`"* — is satisfied by **AC-001 + AC-002** (Q1 and
its residue), **AC-003 + AC-004** (Q2, its losers and the seam consumed rather than resolved),
**AC-005 + AC-006** (Q3 and its honest field), **AC-007** (the record's own standing), **AC-008** (the
mount and the validation) and **AC-009** (the plan of record, without collateral drift). No other
project AC is claimed here; AC-001 belongs to the slice-mate `preflight-and-unlike-axes` and AC-003 –
AC-011 to the stories named in the PR boundary. Project **DoD 3** is discharged by AC-008.

## Interaction quality

RFC §6.7/D6. This story renders **no surface**, and that is not an omission — the project's
signed-off design records `N/A — no user-facing surface` under *every* heading including `## Items`,
`## Signatures`, `## The states the API must express` and `## Anti-patterns`, approved by the
repository owner on 2026-08-12 with **no conditions** ([`../_design.md`](../_design.md), `:40-98`).
The section therefore splits honestly rather than pretending a screen exists.

**COMPOSITION family — not applicable, by an approved determination.** There is no presentation,
placement, transience policy, density budget or hierarchy to hold, and the design records **no named
anti-patterns** to bind against. No composition invariant is written as an AC row, because inventing
one would contradict a design a human signed off, and a fabricated density budget for a markdown atom
is exactly the decorative check CLAUDE.md's *"a rule that no adapter can fail is decorative"* warns
against. The one composition-adjacent constraint that is genuinely real here is **structural rather
than visual**, and it is carried by **AC-008**: the atom *is* the composed artefact, and a decision
that lives only in `references/adr/` has not mounted, because `redkiln validate --kb` never sees it
and `.kb/maps/decision-map.md` does not index it. The corollary constraint — that the atom must not
swallow the long form — is carried by **AC-008** and **NF-002**.

**STATE family — applicable, in the knowledge base's medium. Every invariant below is carried by a
numbered AC row in the table above, never by this prose**, because `redkiln verify` extracts ACs from
`| AC-### |` table cells and a bullet here would be gated by nothing.

| Invariant (state family) | Its meaning in this medium | Carried by | How it is verified |
| --- | --- | --- | --- |
| **In place, not a context jump** | The plan of record answers where it asked: `RUNBOOK.md:304` and `:497` are *rewritten*, so a reader meeting the queue row is told the question closed rather than sent hunting. The crate's own `# Open decisions` list is deliberately **not** this story's to rewrite — that is `real-lbug-driver-swap`'s first act, which is why the atom must exist first. | **AC-009** (rows and box), **AC-008** (the atom the rewrite will point at) | `git diff -- RUNBOOK.md` read against the settled idiom at `:288`, `:485`; `redkiln validate --kb`. |
| **Non-occlusion** | The atom does not hide what it summarises. `references/adr/0025-<slug>.md` is retained beside it with the per-question argument, the losers in full and the `file:line` citations an atom cannot hold; the open question it consumes keeps its own text and status. | **AC-008**, **AC-004** | `test -f references/adr/0025-<slug>.md`; the open-question atom unchanged in the diff; `redkiln validate --kb`. |
| **Preserved state — nothing already settled is disturbed** | `spec/SPECIFICATION.md` is byte-identical, no `[FROZEN]` marker moves, PS-23's clause is reported to and not amended, no accepted atom body is edited, and no sibling's phase-11 checkbox is ticked. | **AC-009**, **AC-002**, **AC-004** | `cargo xtask spec-trace`; `git diff -- spec/SPECIFICATION.md` empty; `redkiln validate --kb`'s immutability check against `HEAD`. |
| **Preserved focus / no silent scroll** | The medium's literal analogue of preserved scroll position: thirteen `RUNBOOK.md:<line>` citations over ten anchors in `spec/SPECIFICATION.md` must still land on the sentence they were written for. A **net-zero** line-count edit is what preserves them; `ANCHOR_SLACK`'s twelve lines would otherwise hide the drift. | **AC-009** | `git diff --numstat -- RUNBOOK.md` (insertions = deletions) **plus** `cargo xtask spec-trace`; neither alone is sufficient. |
| **Reversibility** | A wrong answer here is reversed by **supersession**, never by an edit — the cost of an under-argued answer is ADR-0030, which is precisely why the losers are named per question while the argument is still in hand. | **AC-001**, **AC-003**, **AC-005** (the three per-question loser sets), **AC-004** | The supersession graph enforced by `redkiln validate --kb`; human read confirming each question carries its own alternatives section. |
| **Reachable without inside knowledge** | The keyboard-reachability analogue: an adapter author arriving cold reaches the decision through the corpus's own index — `.kb/maps/decision-map.md` — rather than by knowing the filename or the ADR number. | **AC-008** | The map row present and resolving; `redkiln doctor` clean. |
| **Re-derivability** | The humane outcome the story is judged on (context pack §11): a reader can tell a *decided* choice from a *defaulted* one, and can check whether the reasoning still applies to the port they are holding — which requires the record to name the shape and the commit it was written against. | **AC-007**, and **AC-001** – **AC-006** together | Human read of the atom plus its long form; `git show <SHA>` resolving to a tree whose `projection.rs` matches the shape the record names. |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| EC-001 | The upstream preflight (`preflight-and-unlike-axes`, HS-S0074) came back **red** — `crates/happenstance-core/src/projection.rs` has not shipped `type Batch;`, or the write seam or `projection_store_conformance!` is absent. | **This story does not start.** The project halts and the finding is surfaced to `projection-store-freeze` (HS-P0010); it is not worked around locally, and an ADR written against a port that did not ship is worse than no ADR ([`../_decomposition.md`](../_decomposition.md), AC-A01; [`../_storymap.md`](../_storymap.md), Merge order gate 1). Report upward and stop. |
| EC-002 | The implementer reaches the ledger with only a staged document under `.kb/_intake/`, because `/redkiln:kb-ingest` carries `disable-model-invocation` and is a **human** handoff. | AC-008 stays `satisfied: false`. An intake file is **not** evidence for the mount; the row flips only with a `file:line` under `.kb/decisions/0025-<slug>.md` and a `.kb/maps/decision-map.md` row that exist after the wave. Report the handoff and stop — never hand-author the atom (CLAUDE.md; the `0269720` reversion). |
| EC-003 | `.kb/_intake/` still holds this story's document after the wave ran. | The wave did not ingest it — *"a file still sitting there after a run is a file that run did not ingest"* (`.kb/_intake/README.md`). AC-008 is unsatisfied; investigate the wave rather than deleting the file. Two known traps: the glob includes the intake `README.md`, which is dropped at the approval gate, and a second wave's id must be suffixed so it does not overwrite `2026-08-10-intake` / `2026-08-10-intake-2`. |
| EC-004 | An accepted atom is edited — most likely `.kb/decisions/0007-…` or `0008-…`, in the temptation to "update" one with what ADR-0025 found. | `redkiln validate --kb` fails on immutability against `HEAD`, by design. The correction path is a **new** atom that supersedes, never an edit (CLAUDE.md, *Where the work lives*). ADR-0007 and ADR-0008 are cited and built on here; they are not amended. |
| EC-005 | A `RUNBOOK.md` edit **adds or removes a line** — a wrapped table row, an inserted note, a re-flowed checkbox. | Fails AC-009 even if `cargo xtask spec-trace` is green, because `ANCHOR_SLACK` tolerates twelve lines (`xtask/src/spec_trace.rs:391`). Restore the line count: rewrite the row's text in place, on one line, in the idiom of `RUNBOOK.md:288` and `:485`. `git diff --numstat -- RUNBOOK.md` must show equal insertions and deletions. |
| EC-006 | The merged port shape makes one of the three questions **unanswerable** — most sharply, neither the shipped write seam nor `ProjectionProbe` can express a replayable parameterised Cypher statement. | That is **not** an adapter problem to route around. It is the freeze not holding: record it as such, hand it to `freeze-verdict-document` and surface it to HS-P0010 ([`../_intake-brief.md`](../_intake-brief.md), *Constraints*; [`../_decomposition.md`](../_decomposition.md), §2). Inventing a private adapter helper that hides the gap satisfies Q2's shape and destroys its value. |
| EC-007 | Q2's chosen vocabulary cannot be called from ADR-0007's indicative `Projection::apply(&mut …::Batch<'_>)` signature. | Record the incompatibility **in the ADR**, naming `typed-layer-and-alpha-release` as the project that inherits the bill (AC-003). Do not silently choose a vocabulary that the runner cannot call, and do not amend ADR-0007 — it is accepted and immutable. |
| EC-008 | The number `0025` turns out to be taken, or the intended wave id collides with an existing directory under `.kb/_governance/integration-waves/`. | Halt before staging. `0025` is reserved for exactly these three questions by `RUNBOOK.md:304`; a collision means something else claimed it, which is a planning fact to surface, not to route around by picking `0026`. For the wave id, suffix it — an overwritten wave destroys the first run's audit trail. |
| EC-009 | The three questions are answered, but with **one** shared `## Alternatives rejected` section covering all three. | Fails **DR-6** and this story, regardless of whether every sentence is true. An accepted atom is immutable and therefore the only durable record of a rejected option; a loser folded into a shared section is a loser a later reader cannot attribute to its question ([`../project.md`](../project.md), DR-6). |

## Non-functional

| id | Requirement | Why, and where it is checked |
| --- | --- | --- |
| NF-001 | **No change under `crates/**`.** No `todo!()` filled, no dependency added, no `stand_in` re-pointed, no `lib.rs:51-66` rewrite. | The PR boundary above; `real-lbug-driver-swap` owns the crate-side rewrite and is deliberately downstream of this atom. Checked by `cargo xtask affected --base main` reporting no affected package, and by no `crates/` path in the diff. |
| NF-002 | The atom stays **atom-sized** — roughly the ~100-line shape the other seventeen hold — with the per-question argument, the transcripts and the citations pushed to `references/adr/0025-<slug>.md` and cited by `file:line`. | CLAUDE.md, *Where the work lives*: deleting the long form would discard about 78% of the corpus, and `spec-trace` cites line ranges that exist only there. Checked by human read against the existing files in `.kb/decisions/`. |
| NF-003 | `redkiln doctor` still reports **exactly six** `template-drift` advisories after the merge — no more, no fewer. | CLAUDE.md (the six deliberately customised templates; the `backlog` CI job asserts the set). A seventh is a template changed without a decision; a missing one means `adopt --templates` was run, which is forbidden here. |
| NF-004 | `RUNBOOK.md`'s net line-count delta is **zero**, and `spec/SPECIFICATION.md`'s diff is **empty**. | Thirteen `RUNBOOK.md:<line>` citations over ten anchors; the six citation repairs belong to `fill-the-bodies-and-ps-34-disposition` in the same commit as the bodies ([`../_decomposition.md`](../_decomposition.md), M7). Checked by `git diff --numstat` and `cargo xtask spec-trace`. |
| NF-005 | The record is **re-derivable**: it carries a date, the port shape it was written against, and the commit SHA it read that shape at. | The genre discipline `references/evaluation/README.md` states for kept evidence — *immutable, dated, pinned to a commit, superseded rather than edited* — applied to the long-form record. Checked by AC-007's review, and by `git show <SHA>` resolving. |
| NF-006 | **No manifest change, no dependency change, no MSRV change** anywhere in the workspace. | The MSRV is 1.97.1 and moving it requires an ADR of its own (ADR-0029; CLAUDE.md binding constraint 5). This story ships no Rust, so any `Cargo.toml` in the diff is a boundary violation. Checked by the PR boundary block and `cargo xtask affected --base main`. |
| NF-007 | **No accepted atom body is edited and no open question is resolved.** Corrections happen by supersession; `.kb/open-questions/projection-store-batch-has-no-apply-seam.md` and `.kb/open-questions/cf-40-fixture-limits-ownership.md` keep their `status`. | `redkiln validate --kb`'s immutability check against `HEAD`; DR-7 ([`../project.md`](../project.md)); the PR boundary's *Explicitly not in this PR*. |
| NF-008 | The full gate is the bar, not `--fast`. | [`../_storymap.md`](../_storymap.md), *Merge order*: *"`cargo xtask ci` — the whole gate, not `--fast` — is the bar for every slice"*, because this project is where wasm32, `cargo-hack`, package-completeness and `spec-trace` meet a new dependency graph. Cheap here (no crate changed), and it is the run that proves so. |

## Implementation notes (non-prescriptive)

Not instructions — the traps this story's shape actually sets. Every one is the implementer's call to
make differently, with a reason.

- **Read the merged port before writing a sentence.** The whole reason this story sits behind
  `preflight-and-unlike-axes` is that Q2's answer reads differently depending on what shipped: with the
  GAT gone, ADR-0008's *"cannot hold the batch across a suspension point"* constrains less than it did,
  and `port_shape.rs`'s `for<'a> S::Batch<'a>: Send` collapses to `S::Batch: Send`. Writing Q2 from the
  sketch produces an argument against a port that no longer exists.
- **Draft the long-form record first, then distil the intake document from it.** The atom is a summary
  of an argument, and summarising an argument that was never written produces a decision with a
  conclusion and no reasoning. `references/adr/0016-the-wire-format.md` is the shape to copy —
  `## The question…`, `## Context`, `## Decision`, `## Consequences`, `## Alternatives rejected`,
  `## What this ADR leaves open`, `## Amendments this decision owes the specification`.
- **Write the intake document to be *ingested*, not to be pretty.** `/redkiln:kb-ingest` extracts claims
  and adjudicates them; it prefers merge/amend over new atoms and it will **not** edit an accepted
  decision body. Phrasing the intake so that ADR-0007 or ADR-0008 reads as needing amendment sets the
  wave up to fail rather than to add.
- **Three sections, three alternative sets.** The cheapest way to satisfy this story's *shape* while
  adding none of its value is one alternatives section at the end. DR-6 and EC-009 exist because that is
  the failure mode; structure the record so the three cannot be merged by a later editor without
  visibly deleting something.
- **Q1 is nearly settled, so spend the effort on the residue.** Re-arguing "node property beats sidecar"
  at length is a defect in the ADR (AC-A03, [`../_decomposition.md`](../_decomposition.md)) — the
  skeleton already recorded the argument. The genuine content is the bidirectional narrowing and the
  per-id-versus-per-property schema, and the discipline is to decide the *adapter's* schema and report
  to PS-23 rather than answering PS-23.
- **Q3 is where a preference will try to pass as a decision.** All three candidates are real and none
  is obviously right; the temptation is to pick the one the skeleton half-implies and price the others
  in a clause each. Price them properly: what a runtime-gated feature does to a runtime-agnostic
  crate's feature powerset under `cargo hack`; what blocking the executor does to a caller who cannot
  see it; what a blocking-only adapter does to the port's claim to be one contract.
- **Edit `RUNBOOK.md` last, and with the line count in hand.** Open the two rows, rewrite the text on
  the same line, tick the box, then run `git diff --numstat -- RUNBOOK.md` before anything else. Doing
  it first invites a re-flow while the file is open for something else.
- **The ingest is a handoff, and the handoff is where the story ends for the implementer.**
  `/redkiln:kb-ingest` is human-invoked and commits on its own worktree branch. Plan for the ledger row
  to be flipped *after* that branch merges (EC-002), and say so in the implementation report rather
  than treating a staged file as done.

## Tests and CI (merge gate)

Grounded in the project testing brief ([`../_decomposition.md`](../_decomposition.md), *Testing brief*
— the **AC-002 row**, the *Test mix by tier* Static and E2E/process bullets, and the merge-gate command
list). This story is one of the project's two ACs proven by **no conformance rule at all**, and the
brief says so plainly rather than manufacturing a tier: the content check *"is a review-tier check, not
automatable"*.

| Tier | Command / path | Proves |
| --- | --- | --- |
| Static / process — KB | `redkiln validate --kb` | The atom exists at `.kb/decisions/0025-<slug>.md` with valid `KbFrontmatter`, `status: accepted`, resolving outbound links — **and no accepted atom was edited** (immutability against `HEAD`). Gates AC-001 – AC-008; it is the only automatic gate NF-007 has. |
| Static / process — backlog | `redkiln doctor` | The item tree is clean and exactly six `template-drift` advisories remain. Gates AC-008, NF-003. |
| Static — specification citations | `cargo xtask spec-trace` | Every `RUNBOOK.md:<line>` and `references/adr/…` citation in `spec/SPECIFICATION.md` still resolves after the `RUNBOOK.md` edits, and no `[FROZEN]` marker moved. Gates AC-009, NF-004 — **necessary but not sufficient**, because `ANCHOR_SLACK` is 12 lines (`xtask/src/spec_trace.rs:391`). |
| Static — line-count neutrality | `git diff --numstat -- RUNBOOK.md` (insertions = deletions); `git diff -- spec/SPECIFICATION.md` (empty) | The half `spec-trace` cannot see: a drift small enough to pass the slack while being wrong. Gates AC-009, NF-004. This is the check the sibling ADR stories learned to run explicitly. |
| Gate — story grain | `cargo xtask affected --base main` | No package is affected, because no `crates/**` path changed — which is simultaneously the proof of NF-001 and NF-006. `.redkiln/config.yaml` wires this as the story grain. Gates AC-006 (no tokio acquired), NF-001, NF-006. |
| Gate — project bar | `cargo xtask ci` (the **whole** gate, not `--fast`) | fmt, clippy `-D warnings`, `cargo test --workspace --all-features`, four wasm32 steps, docs, `spec-trace`, the `--no-default-features` doc build, `cargo package --list`, plus `cargo-hack` / `cargo-deny` where present — all still green. [`../_storymap.md`](../_storymap.md) makes this every slice's bar in this project. Gates NF-008. |
| Artefact presence | `test -f references/adr/0025-<slug>.md`; the `.kb/maps/decision-map.md` row; `.kb/_intake/` empty; a new, non-colliding directory under `.kb/_governance/integration-waves/` | The mount actually landed and the long form was retained rather than folded into the atom. Gates AC-008, NF-002. |
| Review tier — the only instrument a record has | Human read of `.kb/decisions/0025-<slug>.md` **and** `references/adr/0025-<slug>.md` against architecture brief §5 | AC-001 – AC-007 in substance: whether each of Q1/Q2/Q3 names a rejected alternative **and why**, whether Q3's three candidates were priced or merely listed, and whether the port shape and commit are named. The testing brief states this is not automatable; no command substitutes for it. |
| Downstream, **not** run here | `cargo test -p happenstance-ladybug` narrowing unit tests; `projection_store_conformance!` | Deliberately absent. This story ships no Rust, so no rule can pass or fail because of it; the bodies this ADR constrains are `fill-the-bodies-and-ps-34-disposition`'s and the suite run is `ladybug-fixture-and-conformance-run`'s. Claiming either as evidence here would be false. |

**Merge gate, as one list.** `redkiln validate --kb` ∧ `redkiln doctor` ∧ `cargo xtask spec-trace` ∧
the two `git diff` checks ∧ `cargo xtask affected --base main` ∧ `cargo xtask ci`, plus the artefact
presence checks and the human read. No CI job is added or changed by this story.

## Risks and coupling (PR-scoped)

| Risk | Why it is live in *this* PR | Mitigation inside this PR |
| --- | --- | --- |
| The ADR is written against the **sketch** rather than the merged port | The skeleton's prose, this spec's context pack and the architecture brief all describe a port that phase 6 may have shipped differently; all three are available and the merged tree takes an extra step to read | AC-007 makes the shape-and-commit statement a criterion, and EC-001 halts the story on a red preflight. The `depends_on` edge to HS-S0074 exists for exactly this |
| One shared alternatives section instead of three | It is the cheapest way to look finished, and every sentence in it can be true | DR-6 is restated as **AC-001 / AC-003 / AC-005** (one loser set per question) and as **EC-009**; the reviewer reads the record against architecture brief §5's three-question structure |
| Q3 answered by preference, dressed as a decision | The skeleton half-implies an answer, `#[async_trait]` is already excluded, and "the suite needs tokio" is a plausible-sounding shortcut that is simply false here | **AC-005** requires all three candidates costed; **AC-006** requires the two constraints stated as constraints and the tokio argument retired by name against `crates/happenstance-testkit/src/lib.rs:60-67` |
| A `RUNBOOK.md` re-flow drifts ten anchors and passes green | Both edited rows sit *above* five cited anchors, and `ANCHOR_SLACK` tolerates twelve lines | **AC-009** plus the explicit `git diff --numstat` gate row; the in-place idiom is demonstrated at `RUNBOOK.md:288`, `:485` |
| The mount straddles two branches, so the merge looks incomplete at review time | `/redkiln:kb-ingest` is a human handoff that commits on its own worktree branch, so this story's implementer legitimately cannot produce the atom | The PR boundary already admits `.kb/decisions/**`, `.kb/maps/decision-map.md` and `.kb/_governance/**` and explains why; **EC-002** fixes the ledger discipline — intake is never evidence |
| An open question gets "helpfully" resolved in passing | The apply-seam question is *right there* and this ADR reports on it; closing it feels like tidiness | **DR-7 → AC-004** requires it consumed and left open; **NF-007** and `redkiln validate --kb`'s immutability check are the gate |
| The next story rewrites `lib.rs:51-66` in this PR because "it is one deletion" | `real-lbug-driver-swap` is the very next merge and the edit is small and tempting | The PR boundary names it explicitly as **not** in this PR; `cargo xtask affected --base main` reporting any affected package is the tell |
| A wrong answer is expensive to fix and the cost is invisible at review time | Accepted atoms are immutable; the correction is ADR-0030, not an edit | *Data and migrations* states the one-way cost, and the per-question loser sets are the mitigation — the argument is captured while it is still in hand |

## Dependencies

**Blocks on** — `story.md` frontmatter carries `blocked_by: [HS-S0074]`.

| Story slug | Item | What this story cannot do without it |
| --- | --- | --- |
| `preflight-and-unlike-axes` | HS-S0074 | The assertion that the merged port really shipped `type Batch;` (no lifetime), the write seam and `projection_store_conformance!` — the shape all three answers are written against (AC-007). It also lands the dated "structurally unlike" axes under `references/evaluation/`, which is the vocabulary Q2 and Q3 are argued in. A red preflight halts the project rather than this story (EC-001) |

**Unlocks** — `story.md` frontmatter carries `blocks: [HS-S0076]`.

| Story slug | Item | What it takes from here |
| --- | --- | --- |
| `real-lbug-driver-swap` | HS-S0076 | Directly blocked. Its first act is rewriting `crates/happenstance-ladybug/src/lib.rs:51-66`'s `# Open decisions` list into a pointer at this atom — which cannot be written before the atom exists ([`../_storymap.md`](../_storymap.md), `real-adapter` row) |
| `fill-the-bodies-and-ps-34-disposition` | — | Transitively. The four `projection_store.rs` bodies are written **against** this ADR: Q1 fixes the checkpoint statement and forbids simplifying either narrowing variant away; Q2 fixes the vocabulary the batch replays; Q3 fixes the bridge the `async` methods use |
| `freeze-verdict-document` | — | Transitively. The verdict cites this atom to distinguish what the adapter *decided* from what it *defaulted to* — a verdict resting on unrecorded choices is a verdict about an accident (context pack §11) |

**Unordered with respect to this story.** Everything in `real-adapter`, `conformance-run`,
`packaging-and-ci-shape` and `freeze-verdict` merges strictly after this slice; nothing merges
concurrently with it inside the project ([`../_storymap.md`](../_storymap.md), *Merge order*).

## Anchors (progressive disclosure)

Everything below is deferred, not optional. Open each at the moment named; link, never bulk-paste. All
paths verified present in the worktree.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/_decomposition.md` | **§5, "ADR-0025's three questions, and what already constrains each"**, is the binding source for this story — it names, per question, what is already settled and which losers must appear. §6 (T1/T2) says which port shape the answers are written against, M8 says how a KB atom is authored, §9 lists the standing prohibitions, and the Testing brief's AC-002 row defines what "accepted" is checked by | **First, before drafting anything** — it is the brief this spec distils and the tie-breaker on scope | AC-001, AC-003, AC-005, AC-008 |
| `crates/happenstance-ladybug/src/lib.rs` | The three questions as the skeleton states them (`:51-66`), Q1's argument already made (`:53-58`), Q2 declared one of the port's two data points (`:63-66`), and the driver-absent note with the docs.rs failure (`:24-32`). Re-deriving what is here is a defect in the ADR (AC-A03) | Before writing any question section; **read, never edit** — `lib.rs` belongs to `real-lbug-driver-swap` | AC-001, AC-003 |
| `crates/happenstance-ladybug/src/projection_store.rs` | The deferred write set and why parameters sit beside the text (`:70-103`), the bidirectional narrowing and its "silently replay from the beginning" reason (`:175-202`), the blocking question with `spawn_blocking` available-but-unused (`:39-47`), read-your-own-writes as a run-time limit (`:49-64`), and the `Send + Sync` evidence (`:252-257`) | Before Q1's residue, before Q2's mechanism, and before Q3's whole section | AC-002, AC-003, AC-005, AC-006 |
| `crates/happenstance-ladybug/src/live_handle.rs` | The checkpoint statement already written out, with the `i64::try_from` narrowing `PositionOutOfRange` exists for (`:203-217`), plus the ICE and E0195 transcripts on rustc 1.97.1 | When quoting Q1's concrete statement; **do not re-run the instrument** — it is retired by a later story | AC-001 |
| `crates/happenstance-core/src/projection.rs` | The port itself and the PS-1 invariant stated on the trait (`:13-30`) — the reason Q1's answer is forced rather than preferred. Also the file whose *merged* shape AC-007 requires the record to name | Before Q1; again when writing the port-shape-and-commit statement | AC-001, AC-007 |
| `crates/happenstance-testkit/src/lib.rs` | The three emitters at `:60-67`, including `__emit_blocking` — a `#[test]` + `block_on` emitter needing nothing from the adapter. This is the citation that retires *"the suite needs tokio"* as a Q3 argument | While drafting Q3's constraints section, before pricing any candidate | AC-006 |
| `.kb/decisions/0001-async-port-flavours.md` | Why `#[async_trait]` is excluded outright rather than weighed: it injects `+ Send` and makes the wasm32 target impossible. Accepted and **immutable** — cite it, never edit it | Before Q3's candidate list is written, so the field is framed correctly from the start | AC-006 |
| `.kb/decisions/0007-projection-runner-decodes.md` | Carries the indicative `Projection::apply(&mut …::Batch<'_>)` signature Q2's vocabulary must stay callable from. The runner is `typed-layer-and-alpha-release`'s, so an incompatibility here is a bill that arrives there (EC-007) | After Q2's vocabulary is chosen, before it is written up as final | AC-003 |
| `.kb/decisions/0008-one-derivation-for-both-ports.md` | The finding that a provided body cannot hold the `Batch` GAT across a suspension point *under any remedy tried* — it rules Q2 shapes out before they are ranked. Also the `KbFrontmatter` shape to copy for ADR-0025's atom | Before weighing Q2's alternatives; again when drafting the intake document's frontmatter | AC-004, AC-008 |
| `.kb/open-questions/projection-store-batch-has-no-apply-seam.md` | Its **ordered sub-question 1** is Q2's loser set verbatim — a bound on `Batch` versus a method on the port versus a second associated type. Consumed as evidence (DR-7), **status untouched**; resolving it belongs to `projection-store-freeze` | While writing Q2's alternatives; never to change its frontmatter | AC-004 |
| `spec/SPECIFICATION.md` | PS-1 (the invariant), PS-9 at `:4947` and PS-11 at `:4977` (the write-vocabulary questions this adapter is a data point for), PS-23 at `:5317` (two ids in one commit) and `:4814` (this adapter named as the input). This record **reports to** these clauses; no marker here is this story's to move | When writing Q1's residue and Q2's framing; before running `spec-trace`. **Byte-identical in this PR** | AC-002, AC-009 |
| `RUNBOOK.md` | The two queue rows this story rewrites (`:304`, `:497`), the phase-11 ADR-0025 checkbox (`:4415-4419`), the settled in-place idiom (`:288`, `:485`), and PS-34's ownership row (`:602`) that keeps this project a reporter rather than a mover | **Last**, after the record is written, with `git diff --numstat` in hand | AC-009 |
| `xtask/src/spec_trace.rs` | `ANCHOR_SLACK` at `:391` — twelve lines of tolerance, which is exactly why a green `spec-trace` is necessary but not sufficient for AC-009 | Before deciding that a `RUNBOOK.md` edit is safe | AC-009 |
| `references/adr/0016-the-wire-format.md` | The house shape for the long-form record — `## The question…`, `## Context`, `## Decision`, `## Consequences`, `## Alternatives rejected`, `## What this ADR leaves open`, `## Amendments this decision owes the specification` | When drafting `references/adr/0025-<slug>.md`, before the intake document | AC-008 |
| `.kb/decisions/README.md` | The immutability rule in its own words, and why an accepted atom is the only durable record of a rejected option — the mechanical reason DR-6 insists on losers per question | Before deciding how much argument to capture; when tempted to "fix it later" | AC-004, AC-008 |
| `.kb/_intake/README.md` | The staging mount's operating rules: what a wave consumes, that the glob includes this README, and that a file still present after a run is a file that run did not ingest | When staging the document, and again when checking the wave afterwards | AC-008 |
| `.kb/maps/decision-map.md` | The corpus's decision index and the row ADR-0025 must acquire — the column shape to match, and the mount AC-002 of the project names explicitly | After the ingest wave, to verify the mount actually landed | AC-008 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/_storymap.md` | This story's row and its one-line slice, *Why the slices are cut here* (why this is foundation and merges second), *Coverage* (which phase-11 boxes are **not** this story's), and Merge order gate 1 (the red-preflight halt) | Before starting, and again before ticking any `RUNBOOK.md` checkbox | AC-007, AC-009 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/project.md` | **DR-6** (losers per question, not once for the record), **DR-7** (the seam consumed, not resolved), project **AC-002** (what "accepted" means) and **DoD 3** | Before drafting; when scoping what the record must contain | AC-004, AC-008 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/_design.md` | The signed-off determination that this project renders **no user-facing surface**, with `## Anti-patterns` recorded `N/A` and the sign-off at `:88-98`. It is why *Interaction quality*'s composition family is empty rather than invented | Before writing or reviewing anything that looks like a presentation requirement | AC-008 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | Persona 2, the adapter author (`:114-179`) — the reader every AC is framed from — and Persona 4, the evaluator (`:249-315`), who reads the verdict this atom will be cited in. No `.kb/product/` persona atom exists yet to cite instead | When judging whether the record answers its actual reader rather than its author | AC-007 |
| `references/evaluation/README.md` | The lifecycle this repository already requires of kept evidence — *immutable, dated, pinned to a commit, superseded rather than edited* — which is the same discipline NF-005 applies to the long-form record | When deciding what metadata the record must carry | AC-007 |

## Clarifications resolved during spec

1. **AC count.** The front half decided AC-001 … AC-009 and the back half enumerates exactly those
   nine. None added, none dropped; the ledger matches one row per id.
2. **How the seventeen *Behavior and interfaces* rows map onto nine ACs.** Q1's three rows collapse to
   **AC-001** (the answer and its loser) and **AC-002** (the residue plus the PS-23 reporting boundary),
   because the residue is a separate obligation that can be satisfied while the headline answer is
   right. Q2's four rows become **AC-003** (the vocabulary, its mechanical reason, and runner
   compatibility) and **AC-004** (the plural losers plus the open question consumed rather than
   resolved). Q3's three rows become **AC-005** (one bridge from three, each costed) and **AC-006** (the
   two hard constraints, the retired tokio argument, and the `Send` flavour not reopened) — split
   because the second is about the *field* being framed honestly, which can fail while a defensible
   choice is made. The four mount rows are one criterion, **AC-008**, because a partially-mounted atom
   is not partially reachable. The port-shape row is **AC-007** and the two `RUNBOOK.md`/frozen rows are
   **AC-009**.
3. **Interaction quality with no surface.** The composition family is written as *not applicable by an
   approved determination* rather than skipped silently, because [`../_design.md`](../_design.md)
   records `N/A` for every heading and `:88-98` is the human sign-off on that determination.
   Fabricating a density budget or a transience policy for a markdown atom would contradict a
   signed-off design. The **state family does** apply in the knowledge base's medium — in-place
   answering, non-occlusion, preserved settled state, preserved citation anchors, reversibility by
   supersession, index reachability and re-derivability — and every one is carried by a numbered AC row
   in the table, never by a prose bullet, so `redkiln verify` can extract them.
4. **Why the mount cannot be completed by this story's implementer alone.** `/redkiln:kb-ingest`
   carries `disable-model-invocation` and is human-invoked, and it commits on its own worktree branch.
   The PR boundary therefore lists the wave's outputs so a legitimate file does not fail
   `redkiln verify --grain story` once that branch merges — but **EC-002** makes an intake file
   explicitly *not* evidence for AC-008. The ledger row flips after the wave, with a `file:line` under
   `.kb/decisions/`.
5. **`<slug>` is deliberately unresolved.** The atom and record filenames are written as
   `0025-<slug>.md` throughout, because the slug should name the answer and the answer is Q1+Q2+Q3
   taken together — a name chosen before the record is written tends to name only the question that
   was easiest. The implementer fixes it when citing evidence; the ledger's `mount_point` carries the
   same placeholder.
6. **Why no conformance rule appears in the merge gate.** This story ships no Rust, so no rule in the
   projection suite can pass or fail because of it. The testing brief already classifies project AC-002
   as **Static + review tier** and states the content check is not automatable. Manufacturing a tier
   here would be the decorative-check failure CLAUDE.md names, and would let a green run stand in for a
   record nobody read.
7. **PS-23 is reported to, not answered.** Q1's residue — one checkpoint node per `ProjectionId`, or
   one node with a property per id — is decided **as this adapter's schema** and handed to PS-23 as a
   data point (`spec/SPECIFICATION.md:4814` names this adapter as an input). Deciding it as a *port*
   question would settle a clause owned by `projection-store-freeze`, which CLAUDE.md's open-questions
   discipline forbids doing in passing.
