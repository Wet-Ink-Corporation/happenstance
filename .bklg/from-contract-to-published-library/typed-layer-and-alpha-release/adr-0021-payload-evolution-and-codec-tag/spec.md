---
item: HS-S0019
stage: spec
created: 2026-08-12T13:46:14.461Z
updated: 2026-08-12T13:46:14.461Z
template_sig: 87bbf1d0
rendered_sig: 61f1a37f
---

# Spec — ADR-0021 — payload evolution and the codec tag's home

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md` |
| This spec | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/adr-0021-payload-evolution-and-codec-tag/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md` — *The codec tag has two possible homes, and they cost different things* (`:548-570`); *Dependencies, features, and what the MSRV lesson means here* (`:588-600`); *The ADR route, and who invokes it* (`:643-656`) |
| Signed-off design (**binding**) | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` — `## Signatures` (`:439-462`, `Codec::TAG`, `DomainEvent::encode`/`decode`), `## Shape decision` (`:642-643`), *Open, and deliberately not settled here* (`:674-679`), `## Sign-off` (Approved, 2026-08-12) |
| Discovery (this story) | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/adr-0021-payload-evolution-and-codec-tag/discover.md` — signal ledger, the three deferred questions, and the three named mutants |
| Story map (this row) | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_storymap.md:50` |
| Roadmap pointer | `RUNBOOK.md:300` — ADR-0021, phase 7: *"How does a payload's shape evolve — codec tag, versioned event types, upcasting, and does the read path need a hook it does not have?"* |
| KB corpus rules | `.kb/decisions/README.md` (*The immutability rule*, *What belongs here*), `.kb/_intake/README.md` (the ingest contract) |

## One-line PR slice

Stage into `.kb/_intake/` the decision on payload evolution — whether `EventType` carries a version suffix, whether an upcaster needs a read-path hook `EventStore` does not have, and which of the two admissible homes the codec tag takes (`Event::metadata`, `crates/happenstance-core/src/event.rs:379`, vs `Tags`) under the constraint that **no adapter may need to understand it** — ingested in the same wave as ADR-0020.

## Executive summary

This PR lands **one staged document and one long-form record, and no Rust.**

The delta is entirely in the knowledge base. `.kb/decisions/` today holds seventeen
atoms — `0001`…`0016` and `0029` — and there is no `0021`; `RUNBOOK.md:300` has held
the slot since the plan was written.

**The one structural difference from the slice-mate, and it changes what this story
is.** `adr-0020-fold-query-agreement` *records* a decision a human already took at the
design gate. This one **takes** three. `_design.md:674-679` says so in terms — *"Open,
and deliberately not settled here: ADR-0021's codec-tag home… the design does not wait
on it, and the M1 story decides it"* — because the public surface is invariant under
the choice (`Codec::TAG` is a `&'static str` either way, `_design.md:441-445`). So
there is no signed-off row to copy, and no later stage that will take these answers if
this one does not: M3's `codec-and-feature-forwarding` *consumes* the siting
(`_storymap.md:53`), it does not choose it.

That is also why this story is a decision record and not a design note. Two of the
three answers are commitments — *must*, *must not* — and a commitment that can be
edited is not one (`.kb/decisions/README.md`, *What belongs here*). The record is
staged, the wave mints the atom, and from that point a correction is a **second,
superseding atom**.

Pointer, not restatement: the project's scope, ACs and risks are at `project.md`; the
codec surface this decision governs is at `_design.md:439-462`. Neither is
re-litigated here.

## Context pack

**Read this section and you can start. Everything deeper is an anchor below.**

### The problem, stated against this tree

A contract crate that carries opaque `Bytes` has no evolution problem — bytes never
need reinterpreting. A **typed** layer has one from its first commit: a decoded
`Enrolment::Defined { capacity: u32 }` breaks the moment a field is added, and a store
that holds two encodings at once needs a way to tell them apart **that no adapter is
allowed to understand**. AC-005 already requires the tag to exist
(`project.md:177-180`); this record decides where it lives and what evolution strategy
it belongs to.

### Question 1 — the codec tag's home. Two admissible answers, and one falsifier

The frozen contract offers exactly two places (`_decomposition.md:548-570`):

- **`Event::metadata`** — `with_metadata` at `crates/happenstance-core/src/event.rs:379`,
  read as `Option<&Bytes>` at `:400`. Opaque to every store *by clause*: **VT-3**
  states that *"the contract layer, a store adapter, and a peer MUST NOT parse `data`
  or `metadata`"* (`spec/SPECIFICATION.md:629-637`, `[FROZEN]`). **The cost to state
  and accept:** every event carries it, and `metadata` is the same field an
  application wants for causation/correlation — so the typed layer would be defining a
  private structure inside a public opaque field, and must say how an application's own
  metadata coexists with it.
- **`Tags`** — queryable, validated (`crates/happenstance-core/src/tag.rs:304-312`),
  no adapter change needed. **The cost:** a tag is part of the **DCB matching
  surface**, so it participates in query semantics and in every adapter's tag index,
  and it consumes tag budget (`MIN_SUPPORTED_TAGS_PER_EVENT`,
  `crates/happenstance-core/src/limits.rs:27`).

**The constraint that binds whichever wins — and the record's own falsifier:** *no
adapter may need to understand the tag* (`_decomposition.md:565-570`; `_design.md:678-679`).
Stated as a check rather than a preference: **name the adapter change this choice
forces.** That is ADR-0003's own reasoning and the ground on which ADR-0007 rejected a
decoding projection store.

Applied to the tree as it stands today, that check is not symmetric, and the record
must show the working rather than assert the conclusion. `Tags` forces a change in
**every** adapter — not by making one parse the tag, but by obliging all of them to
store, index and match on it — and it makes a re-encoding change which `QueryItem`s an
event satisfies, so a consistency boundary silently changes shape as a consequence of a
storage-format migration that was supposed to be invisible (`discover.md`, *The
mutant*). `Event::metadata` forces none, because VT-3 already forbids a store from
parsing it. The record states which cost it accepted; it does not get to skip the
comparison.

### Question 2 — does `EventType` carry a version suffix?

`EventType::from_static` is `const` and would accept `"CourseDefined.v2"` happily
(`crates/happenstance-core/src/event.rs:108-119`), and `EVENT_TYPES` is a `const` list
of exact strings. So the naive "yes" compiles, round-trips, and is indistinguishable
from a correct decision at review time — and it is the second named mutant. Every
`Query` already written names `CourseDefined` **exactly**, including the one at
`examples/course-subscriptions/src/main.rs:117`, so the first upcast makes existing
decision models read an empty log: capacity resolves to `None`, the handler bails with
*"course does not exist"*, and the append condition that was supposed to protect the
boundary matches nothing at all.

The obligation this puts on the record is precise: **a version suffix is admissible
only together with a rule for how a query names versions.** A suffix without that rule
is not an evolution strategy; it is a way to make old events unreachable in one commit.

### Question 3 — does an upcaster need a read-path hook `EventStore` does not have?

It does not have one. `EventStore` is `[FROZEN]` and offers `read`, `append`, `head`
and `read_decision_model` (`crates/happenstance-core/src/store.rs:110-159`) — no hook
of any kind. So the answer has a **routing** consequence, not a design one:

- **"Yes" is not a licence to add one.** It is a defect entry naming the clause and
  routed to a decision record under AC-012, never a line edit of a frozen crate
  (`project.md:132-134`, AC-A02).
- **"No" must be earned, not defaulted.** It is the answer that requires no work and
  leaves no trace, and if it is wrong it is discovered by an application in production
  — which is P1's stated fear (`_decomposition.md:45-49`). The honest form of "no"
  **names the upcasting strategy that makes it true**, and the signed-off surface
  already permits one: `DomainEvent::decode(codec, event_type, data)` takes the
  `EventType` and the raw `&Bytes` and returns `Result<Self, CodecError>`
  (`_design.md:396-408`), so decode-time tolerance is expressible inside the typed
  layer with no port change at all.

### `serde` here is the split working, not a violation

ADR-0003's prohibition attaches to **`happenstance-core`**, whose `serde` feature
covers envelope types only; after ADR-0006 this crate is the *typed* layer whose entire
job is encoding (`_decomposition.md:588-595`; `CLAUDE.md`, binding constraint 2). The
record must read the crate name carefully or it will state the constraint backwards.
The line that stays true either way: **payloads remain `Bytes` at the port**, and
`Codec` operates strictly above it — encoding never routes through
`happenstance-core/serde`.

### This is not the wire format, and the record must say so

ADR-0016 already settled the replication wire format — private to happenstance,
`Event::data`/`metadata` as base64 in human-readable formats and raw bytes in binary
ones (`.kb/decisions/0016-the-wire-format.md`). That is **how an event crosses a peer
boundary**; the codec tag is **how a payload's own encoding is identified inside a
store**. Conflating them would put a second, competing answer into a corpus that
already has one, and the immutability rule makes that expensive to unwind. The record
cites `kb-decision-0016` as `related` and states the seam.

### What "written first" actually obliges

`/redkiln:kb-ingest` is **human-invoked** and is a handoff, not a step inside this
story's implementation (`_decomposition.md:643-651`; `CLAUDE.md`, *Where the work
lives*). Atoms are authored by the ingest path, **never by hand** — hand-writing them
produces the directory layout of the process without the process, which is why the
first attempt was reverted at `0269720`. So the unit of work is the **staged
document**; the atom is the wave's output. Two consequences, both acceptance criteria:

- **A successful ingest clears `_intake`** (`.kb/_intake/README.md`, *A successful
  ingest clears this directory*), so the staged file is not a durable home for the
  reasoning. Hence the long-form record at `references/adr/`, which is the corpus's
  two-places-on-purpose rule and exactly what `ADR-0029`'s atom points its own
  `source_paths` at.
- **Once accepted, the atom is immutable** — a correction after M3 is written is a
  second, superseding atom (`.kb/decisions/README.md`, *The immutability rule*,
  checked against `HEAD` by `redkiln validate --kb`).

### The persona-journey slice

Activity **A1**: *"Decide the shape before writing it — the two answers this layer is
built on exist as records naming the alternatives that lost"* (`_storymap.md:23`). The
readers served are the **M3 implementer**, who needs to know where to write the tag
before writing `Codec`, and the **maintainer six months out** who meets the same fork.
P1 — the application author — is served transitively and is the one who pays if this is
wrong: their fear is *"being the one who discovers a contract defect in production,
after they have already built on it"* (`_decomposition.md:47`), and a payload-evolution
answer that only works until the first schema change is precisely that defect.

### What this story must not do

Write Rust. Hand-author `.kb/decisions/0021-*.md`. Amend VT-3 or any `[FROZEN]`
`ES-*` clause. Add a read-path hook to `EventStore`. Re-open ADR-0020's subject
(fold/query agreement — the slice-mate's). Change any signature in `_design.md`
`## Signatures`, which is invariant under every answer this record takes.

## Integration contract

- **Archetype**: `foundation` — real in-tree substrate (a decision record) consumed by
  the capability slice `codec-and-feature-forwarding` (M3) in this same project. Not a
  double, not a fixme.
- **Slice / milestone**: **M1 `decision-records`** (`_storymap.md:49-50`, merge order
  step 1). **Slice-mate**: `adr-0020-fold-query-agreement` — implemented in the same
  context and staged into the **same** `/redkiln:kb-ingest` wave, which is the
  integrated surface this milestone mounts. `depends_on: []` — M1 has no in-project
  input.
- **Mount point**: **`.kb/_intake/0021-payload-evolution-and-codec-tag.md`** — the
  intake directory is the KB's real composition root and the *only* input path to
  `/redkiln:kb-ingest` (`.kb/_intake/README.md`: *"the default input to
  `/redkiln:kb-ingest`"*). A decision document that lands anywhere else is unmounted by
  construction: never ingested, never an atom, never in the corpus. The wave's output —
  `.kb/decisions/0021-payload-evolution-and-codec-tag.md`, its row in
  `.kb/maps/decision-map.md` and its entry in `.kb/maps/domain-map.md` — is written by
  the ingest run on its own worktree branch, not by this PR.
- **Wires into**:
  - `.kb/decisions/0003-opaque-payloads.md` (`kb-decision-0003`) — payloads stay opaque
    `Bytes` at the port; the reasoning the tag-home falsifier is borrowed from.
  - `.kb/decisions/0016-the-wire-format.md` (`kb-decision-0016`) — the *replication*
    encoding answer this record must be distinguished from, not merged into.
  - `.kb/decisions/0006-bare-name-to-the-typed-layer.md` (`kb-decision-0006`) — why
    `serde` and encoding belong in `happenstance`, not in the contract crate.
  - `.kb/decisions/0007-projection-runner-decodes.md` (`kb-decision-0007`) — the
    decode boundary; the ground on which a decoding *store* was rejected.
  - `spec/SPECIFICATION.md:629-637` (**VT-3**, `[FROZEN]`) — obeyed, never amended.
  - `crates/happenstance-core/src/event.rs:379` / `:400` (`with_metadata`,
    `metadata()`), `:108-119` (`EventType::from_static`, `const`),
    `crates/happenstance-core/src/tag.rs:304-312` (`Tags::from_pairs`),
    `crates/happenstance-core/src/limits.rs:27` (tag budget),
    `crates/happenstance-core/src/store.rs:110-159` (the frozen read surface with no
    hook) — the constructors and surfaces the decision is constrained by and does not
    change.
  - `_design.md:439-462` (`Codec::TAG`, `CodecError`) and `:396-408`
    (`DomainEvent::encode`/`decode`) — the signed-off surface this record governs and
    must leave signature-identical.
- **Renders surfaces**: **none.** This story renders no id from `_design.md`'s
  `## Surfaces` manifest (`crate-root-rustdoc`, `crate-readme`,
  `first-program-doctest`, `worked-example-transcript`, `dsl-failure-message`,
  `compile-fail-diagnostic`) — all are routed at `crates/` or `examples/` paths this PR
  may not touch. **Public items it governs but does not implement**:
  `happenstance::Codec` (`_design.md:260`), `happenstance::CodecError` (`:284`) and
  `happenstance::DomainEvent`'s `encode`/`decode`, all built by M3's
  `codec-and-feature-forwarding`. The record must stay consistent with those
  signatures; it must not restate them.
- **Conformance rule(s)**: **not adapter-observable, and deliberately.** That is the
  decision's *content*, not an omission: the whole point of siting the tag where VT-3
  keeps it opaque is that no store can see it, so there is no rule in
  `crates/happenstance-testkit/src/suite.rs` any adapter could fail — adding one would
  be decorative (`CLAUDE.md`, *A rule that no adapter can fail is decorative*), and if
  a rule *could* observe the tag, that would itself falsify the decision. The
  instruments are `redkiln validate --kb` and M3's per-codec round-trip tests
  (`_storymap.md:53`).
- **Clause(s)**: **none discharged, none amended.** **VT-3** is *obeyed* and cited as
  the constraint the tag-home choice is checked against; `EventStore`'s `[FROZEN]` read
  surface is the wall the upcaster question runs into. If the upcaster answer is "a
  hook is needed", that produces a **defect entry with a clause ID routed to a decision
  record** under AC-012 — which is the rule, not an exception to it. No edit to
  `spec/SPECIFICATION.md` is in this PR in any form.
- **Advances DoD scenario**: initiative **DoD 1** — *"@smoke — the worked example runs
  end to end"* (`initiative.md:360-362`). This record licenses the encoding shape M3
  builds and M6's rewritten example runs on; a payload path decided after the code is a
  payload path recorded rather than chosen. It also directly discharges the project's
  own **DoD 3** (*ADR-0020 and ADR-0021 are accepted atoms under `.kb/decisions/`,
  written before the code they govern, and `redkiln validate --kb` is clean*) jointly
  with its slice-mate, and its answers are inherited unchanged by initiative **DoD 9**
  (*a stranger can install it*), because the alpha ships whatever tag home this
  chooses.

## PR boundary

**In this PR**

- The staged decision document at `.kb/_intake/0021-payload-evolution-and-codec-tag.md`.
- The long-form record at `references/adr/0021-payload-evolution-and-codec-tag.md`,
  which the staged document cites in the `source_paths` it proposes and which survives
  `_intake` being cleared by the wave.
- This story's own backlog folder — its `_ledger.md` and implementation report.

**Explicitly not in this PR**

- **Any file under `crates/`, `examples/`, `xtask/` or `spec/`.** M1 exists precisely
  because no code yet exists for the record to describe (AC-016, "written first"), and
  a `spec/SPECIFICATION.md` edit here would be the amendment AC-A02 forbids.
- **`.kb/decisions/0021-*.md` and the map atoms.** Written by `/redkiln:kb-ingest`, on
  its own worktree branch, from the staged input. Hand-authoring an atom is the
  reverted anti-pattern (`0269720`).
- **`.kb/open-questions/`.** No open-question atom has this record's subject — the
  nearest neighbour, `human-readable-payload-encoding-on-a-constrained-peer.md`, is
  about **wire-format** encoding on a memory-limited peer and is owned by phase 9
  (`kb-open-question-human-readable-encoding-limits-001`). It is not resolved, edited or
  deleted here; the record may cite it as `related` to mark the seam.
- **ADR-0020's subject matter** — fold/query agreement, `Boundary::query`,
  `DecisionModel::scope`. Slice-mate's, staged in the same wave.
- **The codec trait, its features, and the tag's on-the-wire encoding.** M3's
  (`_storymap.md:53`; `codec-and-feature-forwarding/discover.md:50`). This record sites
  the tag; it does not implement or format it.

**Allowed globs** — `redkiln verify --grain story` reads the first fenced block under
this heading and fails on any file changed outside it.

```
.kb/_intake/0021-payload-evolution-and-codec-tag.md
references/adr/0021-payload-evolution-and-codec-tag.md
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/adr-0021-payload-evolution-and-codec-tag/**
```

**Merge DoD one-liner** — the staged document and its long-form record take **all
three** answers (tag home with its accepted cost, version suffix, read-path hook), each
argued against the falsifier *name the adapter change this choice forces* and against
real code in this tree, contradict no signature in the signed-off `_design.md`, amend no
`[FROZEN]` clause, and carry everything `/redkiln:kb-ingest` needs to mint a conformant
atom; `cargo xtask ci --fast` and `redkiln doctor` stay green (nothing compiled
changed).

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| A staged decision document exists at the ingest mount point | `.kb/_intake/0021-payload-evolution-and-codec-tag.md`, titled for ADR-0021, answering `RUNBOOK.md:300`'s question in its own words. Raw material, **not** held to `KbFrontmatter` — `redkiln validate --kb` skips `_`-prefixed directories by design | `.kb/_intake/README.md` (*the default input*; *`_`-prefixed directories are reserved*) |
| All **three** questions are answered; none is deferred | The tag's home, the version-suffix answer, the read-path-hook answer. A record that settles one and leaves two open does not unblock M3, which consumes the siting, and does not satisfy AC-005's "and ADR-0021 states…" half | `project.md:177-180`; `_storymap.md:50`, `:53` |
| The tag home is **one** home, with the cost it accepted stated out loud | `Event::metadata` or `Tags` — not both, and not "either is fine". Two admissible constructions of the same value is the defect the contract already names in prose about `ProjectionId`: *"two constructors enforcing different rules is the defect that makes an invalid value reachable through the weaker one"* | `_decomposition.md:548-563`; `crates/happenstance-core/src/projection.rs:47-61` |
| The choice is argued against the record's own falsifier, mechanically | *Name the adapter change this choice forces.* `Tags` forces one in every adapter — store, index and match on it, plus tag budget — and re-encoding changes which `QueryItem`s the event satisfies, so a boundary changes shape from a storage migration. `Event::metadata` forces none, because VT-3 forbids a store parsing it | `_decomposition.md:565-570`; `spec/SPECIFICATION.md:629-637`; `crates/happenstance-core/src/limits.rs:27` |
| If `Event::metadata` wins, its accepted cost is discharged, not waved through | `metadata` is the field an application wants for causation/correlation. The record states how an application's own metadata coexists with a typed-layer-private structure inside a public opaque field, and that the structure stays **unparsed by every store** | `crates/happenstance-core/src/event.rs:379`, `:400`; `_decomposition.md:552-559` |
| The public surface is stated as **invariant** under the choice | `Codec::TAG` is a `&'static str` either way and no signature moves — so the record shows the decision was taken on cost, not on ergonomics, and M3's shape did not wait on it | `_design.md:441-445`, `:674-679` |
| The version-suffix answer carries the query-naming consequence | `EVENT_TYPES` is a `const` list of exact strings and `from_static` accepts `"CourseDefined.v2"`, so a suffix is invisible to every query already written — including `examples/course-subscriptions/src/main.rs:117`. A suffix is admissible **only** with a rule for how a query names versions | `crates/happenstance-core/src/event.rs:108-119`; `examples/course-subscriptions/src/main.rs:114-125` |
| The read-path-hook answer is earned, whichever way it goes | "No" **names the upcasting strategy that makes it true** — decode-time tolerance, which `DomainEvent::decode(codec, event_type, data) -> Result<Self, CodecError>` already permits without a port change. "Yes" is a defect entry with a clause ID routed under AC-012, never an amendment | `_design.md:396-408`; `crates/happenstance-core/src/store.rs:110-159`; `project.md:132-134` |
| Every alternative that lost is named **with the wrong implementation it admits** | `Tags` (silent boundary shape-change on re-encode; an adapter obligation in every adapter); a bare version suffix (old events unreachable in one commit); "no hook needed" asserted without naming a strategy (fails in production, P1's stated fear); a hook added to `EventStore` (amends a `[FROZEN]` surface) | `.kb/decisions/README.md`, *What belongs here*; `discover.md`, *The wrong implementation* |
| The record distinguishes itself from ADR-0016 | ADR-0016 owns the **replication wire format** (base64 in human-readable formats, raw bytes in binary); this record owns **which encoding a stored payload is in**. Cited as `related: kb-decision-0016`, with the seam stated so the corpus does not acquire two competing answers | `.kb/decisions/0016-the-wire-format.md` |
| The `serde` boundary is stated in the right direction | ADR-0003 constrains `happenstance-core`, whose `serde` feature covers envelope types only; `happenstance` is the typed layer whose job is encoding. Payloads stay `Bytes` at the port; encoding never routes through `happenstance-core/serde` | `.kb/decisions/0003-opaque-payloads.md`; `_decomposition.md:588-595`; `CLAUDE.md`, binding constraint 2 |
| The long-form record outlives the wave | `references/adr/0021-payload-evolution-and-codec-tag.md` carries the full reasoning, the rejected alternatives and the cost comparison a ~100-line atom cannot hold; the atom will cite it in `source_paths`, exactly as `ADR-0029`'s does | `CLAUDE.md`, *Where the work lives*; `.kb/decisions/0029-msrv-raised-to-1-97-1.md` (`source_paths`) |
| The staged document carries what the wave needs | Proposed `adr_id: ADR-0021`, `kind: decision`, `authority_tier: decision`, `status: accepted`, `phase: 7`, a stated `reversibility`, `depends_on`/`related` naming real atom ids (`kb-decision-0003`, `kb-decision-0006`, `kb-decision-0007`, `kb-decision-0016`), and `source_paths` that all resolve — proposed *for* the ingest run, which authors the frontmatter | `.kb/decisions/0016-the-wire-format.md:1-50` and `.kb/decisions/0029-msrv-raised-to-1-97-1.md` (frontmatter shape) |
| Ingest is a human handoff, not an implementation step | The story ends at "staged and ready"; `/redkiln:kb-ingest` is invoked by a human, in **one** wave with the slice-mate, on its own worktree branch, and it clears `_intake` | `_decomposition.md:643-651`; `.kb/_intake/README.md` |
| Post-wave, the atom is real and reachable | `.kb/decisions/0021-payload-evolution-and-codec-tag.md` exists, `.kb/maps/decision-map.md` carries its row, `redkiln validate --kb` and `redkiln doctor` are clean — an explicit acceptance step, not an automated assertion inside this PR | `.kb/maps/decision-map.md`; `project.md`, *Definition of done*, item 3 |
| Nothing compiled changes | No file under `crates/`, `examples/`, `xtask/` or `spec/` is touched; the affected gate and `cargo xtask ci --fast` are unchanged by construction | `.redkiln/config.yaml` (`affected_gate`, `integration_scoped`) |

## Data and migrations

**N/A — no schema, no store, no persisted data.** This story writes two markdown files
and changes no runtime artefact.

Two migration-shaped things are named here so neither is mistaken for work this story
performs.

**The `_intake` → `.kb/decisions/` transition is a process handoff**, executed by
`/redkiln:kb-ingest`, not a data migration. It is one-way in the sense that matters:
once the atom is accepted its body is immutable, and a correction is a new superseding
atom carrying `supersedes:`, with the old atom's frontmatter flipped to
`status: superseded` + `superseded_by:`. That metadata flip is the only edit an
accepted decision ever receives (`.kb/decisions/README.md`, *The immutability rule*).
Which is exactly why the wording is worth getting right **before** the wave runs rather
than after M3 has been written against it.

**The event-data migration this record governs is not performed here either — and
naming its shape is part of the deliverable.** The codec tag exists so one store can
hold more than one encoding, which is a migration story by construction: events already
written carry no tag, and the record must say what a reader does when the tag is absent
(a default, a refusal, or a rule) rather than leaving M3 to invent one. Two constraints
bound that answer and neither is negotiable in this PR: **no adapter may rewrite or
reinterpret stored events** — VT-3 keeps `data` and `metadata` unparsed by every store
(`spec/SPECIFICATION.md:629-637`) — and **no backfill may run through a hook
`EventStore` does not have** (`crates/happenstance-core/src/store.rs:110-159`). An
answer that needs either is the AC-012 defect entry, not an amendment.

## Acceptance criteria

The readers served by every row below are the two named in the *Context pack*, and
neither is the `cargo add` application author directly: **the M3 implementer**, who must
know where the codec tag is written before writing `Codec`
(`_storymap.md:53`), and **the maintainer six months out** who meets the same fork and
must be able to tell it *was* a fork (`.kb/decisions/README.md`, *What belongs here*).
**P1**, the application author, is served transitively and is the one who pays if this is
wrong — their stated fear is *"being the one who discovers a contract defect in
production, after they have already built on it"* (`_decomposition.md:47`), and a
payload-evolution answer that holds only until the first schema change is exactly that
defect.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** the M3 implementer is about to write `Codec` and needs to know where the tag goes, **WHEN** they look for ADR-0021, **THEN** a staged decision document exists at `.kb/_intake/0021-payload-evolution-and-codec-tag.md`, composed as a decision record — hazard/context, the three decisions, the alternatives that lost, the consequences, the residual — and carrying a **proposed** frontmatter block for the ingest run to author from (`adr_id: ADR-0021`, `kind: decision`, `authority_tier: decision`, `status: accepted`, `phase: 7`, a stated `reversibility`, `depends_on`/`related` naming the real atom ids `kb-decision-0003`, `kb-decision-0006`, `kb-decision-0007`, `kb-decision-0016`, and `source_paths` that all resolve); it lands at no other path, because `_intake` is `/redkiln:kb-ingest`'s only input. | Static. `test -f .kb/_intake/0021-payload-evolution-and-codec-tag.md`; `cargo xtask affected --base main` (the story grain — for a no-package diff it still runs the five file-reading lints and `spec-trace`); allowed-globs check by `redkiln verify --grain story` against this spec's *PR boundary* fence. Frontmatter shape reviewed against the precedent at `.kb/decisions/0016-the-wire-format.md:1-45` and `.kb/decisions/0029-msrv-raised-to-1-97-1.md:1-30`. `redkiln validate --kb` deliberately does **not** cover it — `_`-prefixed directories are skipped by design (`.kb/_intake/README.md`). |
| AC-002 | **GIVEN** an application will one day hold two encodings in one store and the M3 implementer must write the tag *somewhere*, **WHEN** they read this record, **THEN** it names **exactly one** home — `Event::metadata` (`crates/happenstance-core/src/event.rs:379`, read at `:400`) **or** `Tags` — never "either is fine", states the cost it accepted out loud, and shows the choice was decided **on cost and not on ergonomics** by recording that the public surface is invariant under it (`Codec::TAG` is a `&'static str` either way, `_design.md:441-445`). If `Event::metadata` wins, the record additionally discharges its accepted cost: how an application's own causation/correlation metadata coexists with a typed-layer-private structure inside a public opaque field. | Human review at the report gate against `_decomposition.md:548-570` and `_design.md:674-679`; the one-home claim is checked against the contract crate's own words on the same defect class — *"two constructors enforcing different rules is the defect that makes an invalid value reachable through the weaker one"* (`crates/happenstance-core/src/projection.rs:47-61`). A record that names two admissible homes fails this row even if both are argued well. |
| AC-003 | **GIVEN** the record's own falsifier is *name the adapter change this choice forces*, **WHEN** the maintainer asks why the losing home lost, **THEN** the record has applied that falsifier **mechanically and to both candidates**, and has read VT-3 in the direction that actually decides it: `metadata` is admissible **only because nothing below the port needs to see the tag** — VT-3 requires that any value a store, a peer, a conformance rule or a query *must* see be carried in `EventType` or `Tags` (`spec/SPECIFICATION.md:629-637`, `[FROZEN]`) — while `Tags` forces a change in **every** adapter by obliging all of them to store, index and match on it, consumes tag budget (`crates/happenstance-core/src/limits.rs:27`), and makes a re-encoding change which `QueryItem`s the event satisfies, so a consistency boundary silently changes shape as a consequence of a storage-format migration that was supposed to be invisible. | Human review; the reviewer re-opens `spec/SPECIFICATION.md:629-637` and confirms the record uses **both** halves of VT-3, not only the "MUST NOT parse" half. The named mutant (`discover.md`, *The mutant*) is checked to appear in the record as a rejected alternative with its failure mode, not merely as a road not taken. Mechanical negative: `git diff --name-only` shows no path under `spec/` (project AC-A02). |
| AC-004 | **GIVEN** `EventType::from_static` is `const` and accepts `"CourseDefined.v2"` happily (`crates/happenstance-core/src/event.rs:108-119`) so the naive "yes" compiles, round-trips and is indistinguishable from a correct decision at review time, **WHEN** the record answers whether `EventType` carries a version suffix, **THEN** it answers **and carries the query-naming consequence**: every `Query` already written names its types exactly, including `examples/course-subscriptions/src/main.rs:114-125`, so a suffix is invisible to them and the first upcast makes existing decision models read an empty log — capacity resolves to `None`, the handler bails with *"course does not exist"*, and the append condition that was to protect the boundary matches nothing. A suffix is therefore admissible **only** together with a stated rule for how a query names versions; a bare "yes" fails this row. | Human review with the example open at `:114-125`; the reviewer confirms the record either (a) answers "no suffix" and names what carries evolution instead, or (b) answers "yes" **and** states the query-naming rule. Consequence-side proof is deferred to M3/M6 by design: the round-trip and example-execution tests that could observe a wrong answer do not exist until `codec-and-feature-forwarding` and `worked-example-on-typed-layer` land (`_decomposition.md:783`). |
| AC-005 | **GIVEN** `EventStore` is `[FROZEN]` and offers `read`, `append`, `head` and `read_decision_model` and **no hook of any kind** (`crates/happenstance-core/src/store.rs:110-159`), so "no hook is needed" is the answer that requires no work and leaves no trace, **WHEN** the record answers the upcaster question, **THEN** the answer is **earned**: "no" names the upcasting strategy that makes it true — decode-time tolerance, which `DomainEvent::decode(codec, event_type, data) -> Result<Self, CodecError>` already permits with no port change (`_design.md:396-408`) — and "yes" is a **defect entry naming the clause ID and routed to a decision record** under project AC-012 (`project.md:204-206`), never a line edit of the frozen crate and never a proposed amendment. | Human review against `crates/happenstance-core/src/store.rs:110-159` and `_design.md:396-408`. Mechanical negative, and it is the whole enforcement available here: `git diff --name-only` for this PR shows **no** path under `crates/`, `examples/`, `xtask/` or `spec/` (project AC-A02, AC-012; NF-001). A "no" with no named strategy fails the row — that is the third mutant at `discover.md:121-128`. |
| AC-006 | **GIVEN** the codec tag exists so that one store can hold more than one encoding — which makes this a migration decision by construction — and events already written carry no tag at all, **WHEN** the M3 implementer meets an untagged event, **THEN** the record has already said what a reader does (a stated default, a refusal, or a rule), rather than leaving M3 to invent one, and has bounded that answer by the two things this PR may not change: **no adapter rewrites or reinterprets stored events** (VT-3, `spec/SPECIFICATION.md:629-637`) and **no backfill runs through a hook `EventStore` does not have** (`crates/happenstance-core/src/store.rs:110-159`). An answer that needs either is the AC-012 defect entry, not an amendment. | Human review; the reviewer checks the untagged-event rule is stated as a rule a decoder can follow, and that `CodecError::UnknownTag` (`_design.md:461-462`) is either the stated outcome or explicitly not the stated outcome. Cross-checked against `codec-and-feature-forwarding/discover.md` so M3 inherits an answer rather than a gap. |
| AC-007 | **GIVEN** a successful ingest **clears** `_intake` (`.kb/_intake/README.md`), so the staged document is not a durable home for the reasoning, **WHEN** the wave runs, **THEN** the long-form record already exists at `references/adr/0021-payload-evolution-and-codec-tag.md` carrying the cost comparison, the rejected homes with their failure modes and the compiler-facing detail a ~100-line atom cannot hold; the staged document names it in the `source_paths` it proposes; and the record states the seam against **ADR-0016** — which owns the *replication wire format* (`.kb/decisions/0016-the-wire-format.md`) — citing it as `related` so the corpus does not acquire two competing answers to "how is a payload encoded". | Static + review. `test -f references/adr/0021-payload-evolution-and-codec-tag.md`; the proposed `source_paths` lists both files, matching `.kb/decisions/0029-msrv-raised-to-1-97-1.md`'s own; reviewed against `CLAUDE.md`, *Where the work lives* (atom ≈ 100 lines, long record up to ~1,508 — deleting the second would discard about 78% of the corpus). The ADR-0016 seam is checked by reading `0016`'s `summary` and confirming the new record does not restate or contradict its base64/raw-bytes rule. |
| AC-008 | **GIVEN** AC-016 is a *sequencing* obligation — the record must exist before the code it governs — and atoms are authored by the ingest path and never by hand (`0269720` is the reverted attempt), **WHEN** a human runs `/redkiln:kb-ingest` over this document and its slice-mate in **one wave** on its own worktree branch with a suffixed wave id, **THEN** `.kb/decisions/0021-payload-evolution-and-codec-tag.md` exists as an accepted atom, `.kb/maps/decision-map.md` carries its row, `redkiln validate --kb` and `redkiln doctor` are clean, and `git log --diff-filter=A` shows that atom added by the **wave's** commit and not by this story's PR. | Handoff-verified, and deliberately outside this PR's diff. `redkiln validate --kb && redkiln doctor` on the ingest branch; `git log --diff-filter=A -- .kb/decisions/0021-payload-evolution-and-codec-tag.md` names the wave commit. Ledger evidence is that sha plus the validate output. This is the project's DoD 3, discharged jointly with `adr-0020-fold-query-agreement` (`project.md:228-229`). |

**Traceability.** Project **AC-005** (*"…and ADR-0021 states whether `EventType` carries
a version suffix and whether an upcaster needs a read-path hook `EventStore` does not
have"*, `project.md:177-180`) ← AC-002, AC-003, AC-004, AC-005, AC-006. Project
**AC-016** (`project.md:218-220`) ← AC-001, AC-007, AC-008. Both traced project ACs are
covered; no row here reaches outside those two. The *other* half of project AC-005 — the
codec itself, its features and the tag's on-the-wire encoding — is
`codec-and-feature-forwarding`'s (`_storymap.md:53`) and is deliberately absent.

## Interaction quality

**This story renders no surface id from `_design.md`'s `## Surfaces` manifest** (`:40`).
All six — `crate-root-rustdoc`, `crate-readme`, `first-program-doctest`,
`worked-example-transcript`, `dsl-failure-message`, `compile-fail-diagnostic` — are
routed at `crates/` or `examples/` paths this PR may not touch, and are mounted by M2–M6.
So the design's *rendered-surface* budgets (72-column doc fences, ≤ 12 prose lines to the
first fence, ≤ 35-line first program, 80-column transcript, `_design.md:837-869`) do not
bind this deliverable; they bind the stories that render those surfaces, and this record
must not restate them.

What binds here is two things: (a) `_design.md`'s `## Signatures`, `## Shape decision`
and `## Anti-patterns` as **content this record may not contradict**, and (b) the KB
corpus's own composition rules, which are this artefact's real presentation layer.
Every invariant below is carried by an **AC row in the table above** — none is a
prose-only bullet, because `redkiln verify` extracts ACs from table cells and a bullet
here would never be gated.

**STATE invariants** — the "surface" is the record a reader opens; "state" is what
survives the ingest wave.

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** — the record answers all three of its own questions where the reader is, and defers none to another document (`AC-U10`'s rule, *"a doc comment that says 'see the specification' for the policy fails this"*, `_decomposition.md:155-163`). Citing `_design.md` as evidence is correct; citing it *instead of deciding* fails | **AC-002**, **AC-004**, **AC-005** | Review: each question has an answer in the record's own words. A record that settles the tag home and leaves two open does not unblock M3 |
| **Non-occlusion — a record must not hide what it filtered.** The direct analogue of `AC-U11`: a decision showing only the winner has occluded its own alternatives and is indistinguishable from an accident | **AC-003**, **AC-004**, **AC-005** | Review: `Tags`, the bare version suffix, and unearned "no hook" each appear as named alternatives **with the wrong implementation each admits** (`discover.md:80-128`), not as omissions |
| **Preserved state across the handoff** — the wave preserves the audit trail: both M1 documents ingested in the **same** wave, a **suffixed** wave id so a second wave cannot overwrite the first's trail, and every `source_paths` entry still resolving after `_intake` is cleared | **AC-007**, **AC-008** | `source_paths` resolution checked before the wave; after it, `git log --diff-filter=A` and `redkiln validate --kb` |
| **Reversibility, and its real cost — this invariant inverts here.** An accepted atom is immutable, so the record is one-way: reversal is a **new superseding atom** carrying `supersedes:`, never an edit; and the hook answer's reversal route is a decision record, never a line edit of a frozen crate | **AC-005**, **AC-008** | Review: the proposed frontmatter states an honest `reversibility` and the record names the supersession route. `redkiln validate --kb`'s accepted-decision-immutability check against `HEAD` is the enforcement |
| **Reachability without a search engine** (`AC-U14`'s analogue) — the atom is reachable from `.kb/maps/decision-map.md`, the long record from the atom's `source_paths`, and the ADR-0016 seam from a `related` edge; nothing load-bearing is findable only by grep | **AC-008**, **AC-007** | The decision-map row and the resolving `source_paths` are explicit acceptance steps; the `related` edge is checked at AC-001's frontmatter review |
| **The decision survives being wrong** — an untagged event, met by a decoder that has no rule, is the state this record exists to prevent M3 from inventing its way out of | **AC-006** | Review: the untagged-event rule is stated as a rule, and it needs neither a store rewrite nor a hook |

**COMPOSITION invariants** — from `_design.md` where it governs content, and from the
corpus rules where they govern this artefact's own shape.

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — the deliverable is a *composed decision record* (hazard, the three decisions, rejected alternatives with failure modes, consequences, residual, proposed frontmatter), not a bullet dump, not a pasted brief excerpt, not three disconnected notes | **AC-001** | Review against `.kb/decisions/README.md`, *What belongs here*, and the precedent pair `.kb/decisions/0016-…` / `references/adr/0016-…` |
| **Placement / composition root** — the staged document lands at `.kb/_intake/0021-payload-evolution-and-codec-tag.md` and nowhere else; a decision document at any other path is unmounted by construction (never ingested, never an atom) | **AC-001** | `test -f` plus the allowed-globs fence in *PR boundary* |
| **Transience** — the same three-way disposition `_design.md`'s `## Transience policy` gives ADR links (*"opened on demand — off-site… the long record is a click away, and only for readers who want it"*, `:831`): the **atom** is the persistent summary a reader meets in the map; the **long record** is opened on demand; the **staged document** is transient by contract and is erased by the wave | **AC-007** | The pairing exists before the wave; the atom's `source_paths` is the click |
| **Density budget, with the corpus's real numbers** — atom ≈ **100 lines** with a `summary` of `0016`/`0029` density; long-form records run **up to ~1,508 lines**; the two exist on purpose and collapsing them would discard about **78%** of the corpus (`CLAUDE.md`, *Where the work lives*). The staged document is sized to **distil to** an atom of that order, not to be one | **AC-007**, **AC-001** | `wc -l` at review against the corpus's own distribution |
| **Hierarchy** — the three decisions are primary and stated first; the rejected homes and the cost comparison are secondary; the untagged-event rule and the ADR-0016 seam are recessive but present. The record never opens with the residual, which would read as a hedge on decisions that were actually taken | **AC-002**, **AC-006** | Review |
| **One home, stated once** — the composition failure specific to this record is a document that argues both homes so evenly that a reader cannot tell which was chosen. That is the same defect the contract crate names about `ProjectionId` and it is a *presentation* failure as much as a decision one | **AC-002** | Review against `crates/happenstance-core/src/projection.rs:47-61`; a reader is asked, cold, to name the chosen home from the record alone |
| **The design's named anti-patterns, as content constraints** — the record must license nothing `_design.md` `## Anti-patterns` forbids, and must not restate the rendered-surface ones (items 1–15 check a *rendered page*, which this story does not render). The **standing** set at `:1008-1010` binds here: no `#[async_trait]`; no `serde` in `happenstance-core`'s defaults; `read` returns the stream at the top level; generic code binds `EventStore`, not `SendEventStore`; no `unwrap`/`expect` in library code | **AC-002**, **AC-005** | Review: the shape the record licenses is checked against that standing list. The second item is the one this record is most likely to state backwards — see NF-004 |

## Error conditions

| id | condition | required handling |
| --- | --- | --- |
| **EC-001** | The wave runs and `.kb/_intake/0021-payload-evolution-and-codec-tag.md` is **still there** afterwards | The contract's own signal: *"a file still sitting here after a run is a file that run did not ingest"* (`.kb/_intake/README.md`). Do not re-run blind and do not hand-author the atom; read the wave's manifest, fix the staged document, re-run. AC-008 stays unsatisfied |
| **EC-002** | The wave's default glob (`.kb/_intake/*.md`, run with no argument) sweeps in `.kb/_intake/README.md` | Drop it at the wave's approval gate. It is the directory's documentation, not raw material; ingesting it mints an atom about the staging area |
| **EC-003** | A second ingest wave reuses the first wave's id | Suffix the id so the second wave cannot overwrite the first's audit trail. That trail is what makes `source_paths` and the wave commit legible after `_intake` is cleared |
| **EC-004** | The record's `serde` paragraph is written against the wrong crate — "no `serde` in `happenstance`" | Backwards, and it forbids the thing the ADR-0006 split exists to allow. ADR-0003 constrains **`happenstance-core`** (`.kb/decisions/0003-opaque-payloads.md`; `CLAUDE.md`, binding constraint 2); `happenstance` is the typed layer whose job is encoding. Rewrite before the wave — after it the atom is immutable |
| **EC-005** | The record answers the codec-tag question in ADR-0016's terms (base64, wire encoding, peer messages) | Two competing answers in one corpus. ADR-0016 owns how an event crosses a **peer boundary**; this record owns how a **stored payload's own encoding** is identified. State the seam and cite `kb-decision-0016` as `related` (AC-007) |
| **EC-006** | `redkiln validate --kb` fails on the new atom's frontmatter after the wave | The defect is in the *staged* document's proposed frontmatter — a dangling `source_paths` entry, an atom id in `depends_on`/`related` that does not exist, a missing `reversibility`. Fix the staged document and re-run the wave; never hand-edit the minted atom into conformance |
| **EC-007** | The upcaster answer comes out "a read-path hook is needed" | Not a licence to add one, and not an amendment. Write the **defect entry** with the clause ID and route it to a decision record (project AC-012, `project.md:204-206`; AC-A02's route, `_decomposition.md:368`). The frozen `EventStore` surface is untouched by this PR either way |
| **EC-008** | ADR-0020's subject matter (fold/query agreement, `Boundary::query`, `DecisionModel::scope`) starts being decided inside this document | Scope leak into the slice-mate. Cut it and hand it across; both are staged in the same wave, which is exactly why a stray decision here is cheap to move and expensive to unwind after ingest |
| **EC-009** | The record resolves or edits `.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md` in passing | Out of scope. Its subject is **wire-format** encoding on a memory-limited peer and it is phase 9's (`kb-open-question-human-readable-encoding-limits-001`). Cite it as `related` to mark the seam; settling it silently is exactly what `CLAUDE.md`, *Open questions*, forbids |
| **EC-010** | A `file:line` citation in the record does not say what the record claims it says | Blocks sign-off. Re-open the range, repair the citation and record the repair — the precedent is `_design.md` `## Sign-off`, which repaired a citation before the human approved it. `spec-trace` catches specification citations; nothing catches a wrong `crates/**` range except a reader (NF-003) |

## Non-functional

| id | requirement | why |
| --- | --- | --- |
| **NF-001** | **Nothing compiled changes.** No path under `crates/`, `examples/`, `xtask/` or `spec/` is touched, so `cargo xtask affected --base main` maps this diff to no package and `cargo xtask ci --fast` is unchanged by construction | The point of M1 is that no code yet exists for the record to describe (project AC-016, *"written first"*); a `spec/` edit here would be the amendment AC-A02 forbids |
| **NF-002** | The staged document distils into an atom of roughly **100 lines**, with a `summary` of the density `0016`'s carries (it names its rejected options *inside* the summary); anything longer belongs in the long-form record | `CLAUDE.md`, *Where the work lives*; `.kb/decisions/0016-the-wire-format.md:12-38` |
| **NF-003** | Every `file:line` citation resolves and is re-read before sign-off | `.kb` and `spec/` citations are load-bearing across the corpus; `cargo xtask spec-trace` catches specification citations, but nothing catches a wrong `crates/**` line range except a reader |
| **NF-004** | The record states the `serde` boundary **in the right direction** and states it once: ADR-0003 constrains `happenstance-core`, whose `serde` feature covers envelope types only; payloads stay `Bytes` at the port and encoding never routes through `happenstance-core/serde` | `CLAUDE.md`, binding constraint 2, warns in terms that after ADR-0006 this constraint *"says the opposite of what it used to"*. Getting it backwards is EC-004 and is unfixable after the wave |
| **NF-005** | The record is **history, not current truth**: it states what was decided and when, does not describe code that does not exist yet as though it did, and will never be updated to match what M3 builds | `.kb/decisions/README.md`, *What does not belong here* — *"where the two disagree, the specification wins. An ADR is never updated to match the code."* |
| **NF-006** | The proposed frontmatter states an honest `reversibility`. The tag-home decision is **not** freely reversible once events are written — reversing it after 0.2.0-alpha.1 means re-siting a tag on data already in stores — and the record says so rather than defaulting to `high` | It is the field a later reader uses to judge how expensive supersession is; `0016` states `high` **because** its format is private with no compatibility obligation, which is precisely the reasoning that does *not* transfer here |
| **NF-007** | The record is legible to a reader **new to idiomatic Rust**: `Bytes` cheap-clone semantics, opaque payloads, and why a `&'static str` tag constant is invariant under the siting choice are each explained by what the alternative was and why it lost, not by naming the construct | `CLAUDE.md`, *Who you are working with*; `standards/rust/70-rustdoc-obligations.md` RS-70-5 is the same rule one level down |

## Implementation notes (non-prescriptive)

Nothing below is binding; the ACs are. This is the order that makes the ACs cheap.

- **Write the long record first, then distil.**
  `references/adr/0021-payload-evolution-and-codec-tag.md` is where the cost comparison,
  the two homes' failure modes and the three mutants actually fit; the staged document is
  then a distillation of it plus the proposed frontmatter. The other order tends to
  produce a staged document that is already atom-shaped and a long record that is a copy
  of it — which is how the 78% gets lost quietly.
- **Answer the three questions in dependency order, not in the order they are listed.**
  The hook question (AC-005) constrains the suffix question (AC-004): if decode-time
  tolerance is the upcasting strategy, a version suffix is doing work `decode` already
  does, and the suffix answer follows. Deciding the suffix first tends to produce a
  suffix rule that the hook answer then makes redundant.
- **Use the `0016` pair as the working template**, not as prose to imitate:
  `.kb/decisions/0016-the-wire-format.md` for the frontmatter fields and a `summary` that
  names its rejected options inline, and `references/adr/0016-the-wire-format.md` for
  what the long form carries that an atom cannot. `0016` is the better precedent than
  `0029` here because it is the corpus's other encoding decision and the one this record
  must be distinguished from.
- **Apply the falsifier as a two-column table before writing prose.** *Name the adapter
  change this choice forces* — one column per home, one row per adapter obligation
  (store it, index it, match on it, budget it, migrate it). AC-003 is most of that table
  written out, and the table is what makes the conclusion look derived rather than
  asserted.
- **State the hazard with the code open.** `examples/course-subscriptions/src/main.rs:114-125`
  is the whole of AC-004's evidence — quote the query's exact event-type names rather
  than describing them, because the argument *is* that they are exact.
- **Write the untagged-event rule as a sentence M3 can implement.** AC-006 is satisfied
  by something of the form *"an event with no tag decodes as ⟨X⟩, because ⟨reason⟩"* —
  not by a paragraph about migration in general. If the honest answer is `UnknownTag`,
  say `UnknownTag` (`_design.md:461-462`).
- **Then stop, and hand off.** The story is done at *staged and ready*. A human invokes
  `/redkiln:kb-ingest` over both M1 documents in one wave, on its own worktree branch,
  with a suffixed wave id (`_decomposition.md:643-656`; `.kb/_intake/README.md`). AC-008
  is verified against that wave, not inside this PR.

## Tests and CI (merge gate)

Grounded in `_decomposition.md`'s testing brief, whose **AC-016** row is explicit that
this criterion's tier is *Static (`redkiln validate --kb`)* and that atoms are authored
by neither the brief, the story nor any test (`:794`), and whose **AC-005** row assigns
the compiled instruments — per-codec round-trip tests and the feature powerset — to M3
(`:783`), not here.

| tier | command / path | proves |
| --- | --- | --- |
| Static, story grain | `cargo xtask affected --base main` (`.redkiln/config.yaml`, `affected_gate`) | The diff maps to **no** workspace package, so the package half compiles nothing — and the unconditional half still runs the five file-reading lints and `spec-trace`. That is why the grain is wired this way: a story whose whole deliverable is a document maps to no package, and a purely package-shaped gate would compile nothing, read nothing and call it green |
| Static, reachability | `cargo xtask lints && cargo xtask spec-trace` | No specification citation rotted and no clause marker moved. This story amends no clause, so the value here is the **negative** proof — VT-3 and the `EventStore` clauses are exactly as they were |
| Static, KB + backlog | `redkiln validate --kb && redkiln doctor` | Before the wave: the corpus is clean and this PR added no atom (`doctor`'s six expected `template-drift` advisories unchanged). After the wave (AC-008): `KbFrontmatter` conformance on the new atom and accepted-decision immutability against `HEAD`. `_intake` is skipped by design, so this says nothing about the staged document — AC-001's instrument is `test -f` plus review |
| Static, existence | `test -f .kb/_intake/0021-payload-evolution-and-codec-tag.md`; `test -f references/adr/0021-payload-evolution-and-codec-tag.md` | AC-001's mount point and AC-007's long record exist at the paths the wave and the atom's `source_paths` will look for |
| Static, boundary | `git diff --name-only` against the *PR boundary* fence, via `redkiln verify --grain story` | NF-001 and the negative halves of AC-003 and AC-005: no `crates/`, `examples/`, `xtask/` or `spec/` path is in this diff, so no frozen clause and no frozen trait was touched |
| Integration, project bar | `cargo xtask ci --fast` (`.redkiln/config.yaml`, `integration_scoped`) | This project's declared integration bar (`project.md` DoD 6) — green **and unchanged**, because nothing compiled was touched. Its four `wasm32` steps and its packaging assertion are unaffected by construction |
| Human review, report gate | This spec's AC table vs `_design.md` `## Signatures` / `## Shape decision` / `:674-679`, and vs `spec/SPECIFICATION.md:629-637` | AC-002 … AC-007 — the substance of the record. There is no compiled assertion that a decision was recorded correctly, and the testing brief says so for this whole class: *"Record, not a test… verified at closeout, not by a compiled test"* (`:790`) |
| Handoff, post-wave | `/redkiln:kb-ingest` (human-invoked), then `git log --diff-filter=A -- .kb/decisions/0021-payload-evolution-and-codec-tag.md` | AC-008: the atom exists, and was added by the wave's commit rather than by hand — the reverted anti-pattern at `0269720` is exactly what this discriminates |
| Deferred to M3 | Per-codec round-trip tests and the `cargo-hack` feature powerset, beside the codec code (`_decomposition.md:783`, `:1038`) | The record's *consequences* become testable only once `Codec` exists: a tag written where this record sited it, round-tripping through JSON/CBOR/postcard. Named here so the absence is not mistaken for a gap in this story |

**Not applicable, and deliberately.** No conformance rule
(`crates/happenstance-testkit/src/suite.rs`): the whole point of siting the tag where
VT-3 keeps it opaque is that **no store can see it**, so no adapter could fail such a
rule and it would be decorative (`CLAUDE.md`, *A rule that no adapter can fail is
decorative*) — and a rule that *could* observe the tag would itself falsify the decision.
No `trybuild` case: nothing here is a compile-time claim.

## Risks and coupling (PR-scoped)

| Risk | Why it bites here | Mitigation in this PR |
| --- | --- | --- |
| **This story *takes* three decisions rather than recording one already taken.** Its slice-mate copies a signed-off design row; this one has no such row (`_design.md:674-679`) | There is no later stage that will take these answers if this one does not — M3 *consumes* the siting (`_storymap.md:53`), it does not choose it. A record that defers any of the three ships a gap disguised as a decision | AC-002, AC-004 and AC-005 are three separate rows, each failing independently; the *Interaction quality* "in place, not a context jump" invariant is bound to all three |
| **Immutability makes wording expensive.** Phrasing that admits two homes cannot be edited out later | `.kb/decisions/README.md`'s repair/amendment test is mechanical: if the admitted-implementation set changes, it is a new decision, not a fix | AC-002 requires **one** home stated in the contract's own words about the two-constructor defect; the composition invariant "one home, stated once" is a reader test, cold |
| **The tag-home decision is the least reversible thing in this project, and looks like the most.** No signature changes either way, so it reads as free | Reversing it after `0.2.0-alpha.1` means re-siting a tag on events already written into real stores, with no adapter permitted to rewrite them (VT-3) | NF-006 forces an honest `reversibility`; AC-006 forces the untagged-event rule now, which is the mechanism any future re-siting would have to reuse |
| **`Tags` is the attractive wrong answer and passes everything.** It validates, it is queryable, no adapter changes, the whole gate is green (`discover.md:82-107`) | Nothing mechanical rejects it. The instrument is the record's own falsifier | AC-003 makes the falsifier a criterion applied to **both** candidates, with VT-3's *positive* half — put it in `Tags` **only if** something below the port must see it — as the discriminator |
| **Slice coupling with `adr-0020-fold-query-agreement`** — same milestone, same wave, no `depends_on` edge | A wave containing one of the two mints one atom and satisfies half of project DoD 3; a document that strays into the other's subject mints a conflicting one | EC-008 draws the line; the *Integration contract*'s slice-mate row names the shared wave as the integrated surface |
| **The story can be "done" while the atom does not exist.** Ingest is a human handoff on its own branch | AC-016 is a sequencing obligation on M3, and `codec-and-feature-forwarding` starts from the atom. Merging with the wave un-run leaves M3 reading a document about to be deleted from `_intake` | AC-008 is an explicit, ledger-carried row whose evidence is the wave commit sha — not an implicit DoD sentence |
| **Nothing in the compiled gate can fail on this PR** — a green gate proves almost nothing | The temptation is to read green as done | The merge-gate table says what each green step actually proves (mostly a negative), and every substantive row is explicitly human-reviewed |

## Dependencies

**Blocks on** — none. This story's `depends_on` is `[]` (`_storymap.md:50`), and that is
structural rather than incidental: M1 exists precisely so that nothing it needs has been
written yet. The signed-off design confirms it directly — the public surface is invariant
under this record's central choice, so the record does not wait on the design and the
design did not wait on it (`_design.md:674-679`).

**Unlocks**

- `codec-and-feature-forwarding` (M3) — its `depends_on` names this story directly
  (`_storymap.md:53`). It writes the tag where this record sites it and inherits the
  untagged-event rule (AC-006) rather than inventing one.
- `command-loop` (M3), transitively through the above.
- `worked-example-on-typed-layer` (M6), transitively — the example's payloads are encoded
  through the `Codec` this record's siting governs, and its query naming is what AC-004's
  suffix answer must not break.
- Project **DoD 3** (*ADR-0020 and ADR-0021 are accepted atoms under `.kb/decisions/`,
  written before the code they govern, and `redkiln validate --kb` is clean*,
  `project.md:228-229`), discharged **jointly** with `adr-0020-fold-query-agreement`.
  Not a dependency edge in either direction — a shared wave.

## Anchors (progressive disclosure)

Everything above is enough to start. Open these at the moment named, and link them rather
than pasting them.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` | The **binding** signed-off design. `## Signatures` (`:439-452`) fixes `Codec::TAG` as a `&'static str` and `CodecError::UnknownTag`; `:396-408` is `DomainEvent::decode`, the surface that makes decode-time tolerance expressible without a port change; `:674-679` is the sentence that hands this decision to this story and names the binding constraint | Before writing a word of the long record — it is the input, and AC-002's invariance claim is read off it | AC-002, AC-005, AC-006 |
| `spec/SPECIFICATION.md` | **VT-3** at `:629-637`, `[FROZEN]`, in both halves: stores/peers MUST NOT parse `data` or `metadata`, **and** anything a store, peer, rule or query must see MUST live in `EventType` or `Tags`. The second half is what actually decides the siting and is the half most easily skipped | When applying the falsifier, before writing the comparison — and again before claiming `metadata` is admissible | AC-003, AC-006 |
| `crates/happenstance-core/src/event.rs` | `with_metadata` at `:379` and `metadata()` at `:400` (`Option<&Bytes>`) are the exact affordance one home is built on; `EventType::from_static` at `:108-119` is `const` and accepts `"CourseDefined.v2"`, which is why the suffix mutant compiles | `:379`/`:400` when siting the tag; `:108-119` when answering the suffix question | AC-002, AC-004 |
| `crates/happenstance-core/src/store.rs` | `:110-159` is the `[FROZEN]` read surface — `read`, `append`, `head`, `read_decision_model`, and **no hook**. The wall the upcaster question runs into, and the reason "yes" is a routing answer rather than a design one | Before answering the hook question, and again before writing any sentence about backfill | AC-005, AC-006 |
| `crates/happenstance-core/src/tag.rs` | `Tags::from_pairs` at `:304-312` is the only (fallible) way in — the losing home is genuinely constructible, which is what makes it a real alternative rather than a straw man | While writing the `Tags` column of the falsifier table | AC-003 |
| `crates/happenstance-core/src/limits.rs` | `MIN_SUPPORTED_TAGS_PER_EVENT` at `:27` — the tag budget every store must accept, spent on every event forever if the tag lives in `Tags` | Same moment as `tag.rs`; it is one row of the cost table | AC-003 |
| `crates/happenstance-core/src/projection.rs` | `:47-61` states the two-constructor defect in the contract crate's own words. AC-002's one-home claim is strongest quoted from here, because it is the repository already condemning the failure mode | When writing the one-home-not-two paragraph | AC-002 |
| `examples/course-subscriptions/src/main.rs` | The suffix hazard in real code: the two-item `Query` at `:114-125` names its event types **exactly**, so `CourseDefined.v2` is a different type to every query already written | While writing the version-suffix section — quote it, do not describe it | AC-004 |
| `.kb/decisions/0016-the-wire-format.md` | The corpus's *other* encoding decision — replication wire format, base64 in human-readable formats, raw bytes in binary. The record must be distinguished from it, cite it as `related`, and borrow its frontmatter/`summary` shape | Before drafting the proposed frontmatter, and again when writing the seam paragraph | AC-007, AC-001 |
| `.kb/decisions/0003-opaque-payloads.md` | Payloads stay opaque `Bytes` at the port, and the reasoning the tag-home falsifier is borrowed from. Also the constraint most easily stated backwards after ADR-0006 | When writing the `serde`-boundary paragraph (NF-004) and when justifying the falsifier | AC-003, AC-002 |
| `.kb/decisions/0007-projection-runner-decodes.md` | The decode boundary, and the ground on which a *decoding store* was rejected — the precedent that a component below the port must not need domain knowledge | When arguing why "no adapter may understand the tag" is a real constraint rather than a preference | AC-003, AC-005 |
| `.kb/decisions/README.md` | The corpus's rules: what belongs (*state the alternatives that lost*), what does not (evidence; current truth), the immutability rule, and the mechanical repair-vs-amendment test | Before writing, and again before sign-off as a checklist against the finished record | AC-002, AC-008 |
| `.kb/_intake/README.md` | The ingest contract: `_intake` is the default input and the **only** mount point; a successful run clears it; `_`-prefixed directories are skipped by `redkiln validate --kb`, which is why the staged document is not held to `KbFrontmatter` | When siting the staged document, and when interpreting EC-001/EC-002 after a wave | AC-001, AC-008 |
| `.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md` | The nearest neighbouring open question — **wire-format** encoding on a memory-limited peer, owned by phase 9. Marks the seam this record must not cross or silently settle | Before citing anything as `related`, to confirm the seam is stated and not stepped over | AC-007 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md` | *The codec tag has two possible homes* (`:548-570`) frames the question and its costs; *Dependencies… and the MSRV lesson* (`:588-600`) fixes the `serde` direction; *The ADR route, and who invokes it* (`:643-656`) fixes the handoff; the Testing brief rows `:783`, `:790`, `:794` fix the tiers this gate table inherits | `:548` before drafting; `:643` before assuming ingest is a step you run; `:783-794` when reading the gate table | AC-002, AC-005, AC-008 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md` | The two traced project ACs verbatim — AC-005 at `:177-180`, AC-016 at `:218-220` — plus AC-012's defect route at `:204-206` and DoD 3 at `:228-229`, the joint obligation AC-008 discharges | When checking traceability before the report gate, and the moment the hook answer looks like "yes" | AC-005, AC-008 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/adr-0021-payload-evolution-and-codec-tag/discover.md` | The three named mutants in full (`:80-128`) — `Tags` siting, the bare suffix, the unearned "no hook" — each with the reason nothing mechanical rejects it | When writing the rejected-alternatives section; each mutant should be recognisable in it | AC-003, AC-004, AC-005 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/codec-and-feature-forwarding/discover.md` | The consuming story's own reading of what it inherits from this record (`:50`) — the check that the answers this record gives are the answers M3 is waiting for | Before declaring the record complete, as a downstream read-back | AC-006, AC-002 |
| `.bklg/from-contract-to-published-library/initiative.md` | DoD 1 (*@smoke — the worked example runs end to end*, `:360-362`) and DoD 9 (*a stranger can install it*) are the outcomes this decision is inherited by; the personas and journeys are the audience the reasoning is written for | When writing the *consequences* section, to state the outcome the siting buys rather than the siting itself | AC-004, AC-006 |
| `RUNBOOK.md` | `:300` is ADR-0021's slot and its question in the plan's own words — *"How does a payload's shape evolve — codec tag, versioned event types, upcasting, and does the read path need a hook it does not have?"*. Four sub-questions; the record answers all of them or it has not filled the slot | When titling the record, and as the final completeness check before staging | AC-001, AC-004 |
| `.redkiln/config.yaml` | The `verify:` block wires the four grains regardless of who types them — `affected_gate`, `reachability_static`, `integration_scoped`. Explains why a no-package diff is still gated, and why a green gate is mostly a negative proof here | When interpreting a green gate, so it is not mistaken for proof of the record's substance | AC-001 |

## Clarifications resolved during spec

1. **The AC set is exactly the front half's eight** — AC-001 … AC-008. None added, none
   dropped. AC-001 and AC-007 carry the two artefacts; AC-002, AC-003, AC-004, AC-005 and
   AC-006 carry the record's substance (the three questions, the falsifier, and the
   untagged-event rule); AC-008 carries the post-wave state. The ledger matches this set
   row for row.
2. **The three questions are three ACs, not one.** RUNBOOK's slot phrases them as one
   sentence, and a single "the record answers ADR-0021's question" row would let two of
   the three be answered thinly while the row still passed. Splitting them is what makes
   *"a record that settles one and leaves two open"* a failure with a name.
3. **AC-006 is the front half's *Data and migrations* obligation promoted to a criterion.**
   That section states — correctly — that no data migration happens in this PR, and then
   names the thing that **is** deliverable: the rule for an event written before the tag
   existed. Without a row, that obligation would have been prose no gate reads, and M3
   would invent an answer under time pressure. It is bounded by VT-3 and the absent hook,
   so it cannot smuggle in either.
4. **AC-008 straddles the PR boundary on purpose.** The atom is minted by
   `/redkiln:kb-ingest` on its own branch, so it cannot be an assertion inside this PR's
   diff — and it cannot be dropped either, because project DoD 3 is written against the
   atom's existence, not the staged document's. An implementer who cannot produce the wave
   commit sha leaves it `satisfied: false`; that is the correct outcome, not a blocker to
   work around by hand-authoring the atom (`0269720`).
5. **This story renders no design surface, and that is a finding rather than an omission.**
   All six ids in `_design.md`'s `## Surfaces` manifest (`:40`) route at `crates/` or
   `examples/` paths this PR may not touch. The composition invariants that *do* bind were
   re-derived from the corpus's own rules (`.kb/decisions/README.md`, `.kb/_intake/README.md`,
   `CLAUDE.md`'s two-places-on-purpose paragraph) and from `_design.md`'s content-governing
   sections, and every one is carried as an AC row above rather than as prose.
6. **VT-3 is used in both directions, and that is the spec's sharpest correction to the
   front half's framing.** The obvious reading — *stores must not parse `metadata`,
   therefore `metadata` is safe* — is only half the clause. The other half says anything a
   store, peer, conformance rule or query **must** see belongs in `EventType` or `Tags`.
   So the siting turns on a prior question: *does anything below the port need to see the
   codec tag?* If the honest answer were yes, VT-3 would **require** `Tags` and the
   constraint *no adapter may need to understand it* would already be violated. AC-003
   makes that the discriminator.
7. **`reversibility` is not inherited from `0016`.** ADR-0016 states `high` because its
   wire format is private with no compatibility obligation and nothing has been published.
   That reasoning does not transfer: this decision is inherited by `0.2.0-alpha.1` and by
   every event a user writes with it, and re-siting a tag on stored events is exactly what
   VT-3 forbids an adapter from doing on their behalf. NF-006 makes stating an honest
   value a requirement rather than a formality.
8. **No conformance rule is in scope, and the absence is the decision's own content.**
   A testkit rule that could observe the codec tag would prove the tag is visible below
   the port — which falsifies the decision rather than verifying it. That is why the
   *Integration contract* records "not adapter-observable" as content and this section
   records it as reasoned, not deferred.
