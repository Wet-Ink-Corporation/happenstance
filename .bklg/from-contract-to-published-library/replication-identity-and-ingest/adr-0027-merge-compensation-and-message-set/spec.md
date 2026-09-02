---
item: HS-S0099
stage: spec
created: 2026-08-12T13:47:38.177Z
updated: 2026-08-12T13:47:38.177Z
template_sig: 87bbf1d0
rendered_sig: 6929c530
---

# Spec — ADR-0027: the merge rule, the compensation contract, and the message set

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — BR-10 (`:293`), AC-13 (`:344-346`), DoD 14 (`:398-401`) |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — traceability matrix, the `publication → replication → retention` DAG edge, warranted briefs |
| Project | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/project.md` — AC-001, AC-010, AC-014; DR-1, DR-3, DR-5, DR-9; risk rows *The intake brief's clause ledger disagrees with the specification* and *Settling an open question in passing* |
| This spec | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/adr-0027-merge-compensation-and-message-set/spec.md` |
| Key briefs | `.bklg/.../replication-identity-and-ingest/_decomposition.md` — architecture brief *Tension 1* (the ADRs gate the code), *Tension 3* (the re-derived clause ledger and the split that must add up), *The wire mounts inside `Envelope<T>`*, and *Notes* (this ADR's questions are deliberately left undecided there); `_grounding.md` — what is actually on disk; `_design.md` — **no user-facing surface**, signed off 2026-08-12 |
| This story's discovery | `.bklg/.../adr-0027-merge-compensation-and-message-set/discover.md` — the signal ledger, the five answered questions, and the three named wrong implementations. It is the input this spec sharpens, not a duplicate of it |
| Roadmap pointer | `.bklg/.../replication-identity-and-ingest/_storymap.md` — slice `decisions-of-record`, merge order slice 1 of 7, position **2 of 3** inside it |

Everything below is scoped to the `decisions-of-record` slice. This story writes
**no Rust**, changes **no clause text** in `spec/SPECIFICATION.md`, and edits **no
accepted atom**.

## One-line PR slice

Author **ADR-0027** — the merge rule, the compensation contract, replication scope,
whether hub-and-spoke and peer-to-peer are one abstraction or two, and the message
set with the derives on `PushBatch` / `EventGroup` / `ReplicatedEvent` authorised
**by name** — as a long-form record under `references/adr/` plus a staged
`.kb/_intake/` draft promoted by `/redkiln:kb-ingest` into an accepted decision atom
under `.kb/decisions/`, taking the **complement** of ADR-0026's clause range so the
split adds up.

## Executive summary

`adr-0026-peer-ingest-and-transport` (HS-S0098) lands the peer, the transport floor
and the reconciliation of the central question, and states a clause range. Everything
it does not claim is this PR, and the two together are what unblocks the code: no
`.rs` file that a `SY` clause constrains may merge before this slice does
(`_storymap.md`, *Merge order* 1; `RUNBOOK.md:4606`).

The delta this PR lands, measured against the tree today:

- **`.kb/decisions/` holds 0001–0016 and 0029 only.** ADR-0027's number is reserved
  in the queue (`RUNBOOK.md:306`) and nothing occupies it. This PR adds one atom —
  and the long-form record beside it, per `CLAUDE.md`'s two-places-on-purpose rule.
- **`PushBatch`, `EventGroup` and `ReplicatedEvent` carry no derives**, deliberately:
  they carried them once, the derives were withdrawn, and phase 5 did not restore
  them because *"a `#[derive]` on a public message type **is** a wire format, and
  what travels is phase 13's"* (`crates/happenstance-sync/src/lib.rs:86-94`). This PR
  is the authorisation those derives have been waiting for, and it authorises them
  **by name** or `message-set-on-the-envelope` (HS-S0106) may not add them.
- **`FORMAT_VERSION = 1` names a vocabulary nobody has chosen**
  (`.kb/open-questions/sync-message-set-and-format-version.md`). This PR chooses it
  and says what a bump would then measure. The open-question *atom* is updated by
  `open-questions-resolved-and-indexed` (HS-S0100), not here.
- **SY-10 is `[PROVISIONAL]`** (`spec/SPECIFICATION.md:6113`) and is the only place
  the one-abstraction-or-two question is actually open; SY-9 (`:6095`) has already
  frozen the half that constrains the port's type.

What this PR deliberately does **not** land: the answer to whether ingest re-checks a
writer's asserted conditions (ADR-0026's, and re-answering it is the exact failure
`.kb/playbooks/one-decision-per-adr-title.md` exists to prevent); any message type,
derive or runner (HS-S0106, HS-S0110); any clause repair (`frozen-clause-repairs`,
HS-S0112); and the open-question atoms' own resolution (HS-S0100).

## Context pack

The load-bearing decisions, stated as decisions. Read this before opening an editor;
the deeper artefacts stay behind the anchors table the second pass appends.

**1. The atom is authored by the ingest path, and the long form is not optional
either.** `CLAUDE.md` (*Where the work lives*) is explicit that atoms are authored by
`/redkiln:kb-ingest` from `.kb/_intake/`, never by hand — the one attempt to
hand-write a correctly shaped `.kb/` tree was reverted at `0269720` because "the
directory layout of the process without the process" is a different process. So the
deliverable is **two artefacts and a wave**: the long-form record at
`references/adr/0027-*.md` (which carries the transcripts, the rejected alternatives
and the reasoning a ~100-line atom cannot hold), a staged draft under `.kb/_intake/`,
and the wave commit that turns the second into `.kb/decisions/0027-*.md`. Deleting or
skipping the long form is not a shortcut: `spec/SPECIFICATION.md` cites
`references/adr/` by line range and `cargo xtask spec-trace` reads those citations.

**2. This ADR is the complement, and it must not re-answer ADR-0026's question.**
*Does ingest re-check the writer's asserted append conditions* is answered by SY-1
(`spec/SPECIFICATION.md:5841-5867`) and SY-6 (`:5973-6029`), **both `[FROZEN]`**, and
ADR-0026 reconciles and cites them (project AC-002). ADR-0027 inherits that answer
and specifies **what happens instead** — SY-2's single-batch compensation. One
decision per ADR title (`.kb/playbooks/one-decision-per-adr-title.md`) is the rule,
and the failure mode it names is precisely a second ADR restating a settled answer in
its own slightly different words, after which two documents disagree and neither is
wrong.

**3. The compensation contract is frozen in shape; what this ADR owes is the record
and the alternatives that lost.** Three clauses, and the ADR must reproduce their
division of labour rather than a paraphrase of it:

- **SY-2 `[FROZEN]`** (`:5871-5900`) — the compensation is appended in the *same*
  `append` call as the losing event: `append(&[losing, compensation], Some(&guard))`.
  A reader MUST NOT be able to observe the log holding the losing event with nothing
  resolving it. The alternative that lost is named in the clause itself —
  **compensate-after-commit** — and it lost because the window it opens survives the
  window closing: a device syncing inside it cuts its next slice from a hub log that
  says one physical compressor is held twice.
- **SY-3 `[FROZEN]`** (`:5901-5930`) — the port supplies atomicity, identity and
  idempotence; it MUST NOT supply the compensation's *content*. The seam is a
  caller-supplied closure, for the same reason ADR-0007's `pump` takes one: a port
  method `fn compensate(&self, losing: &Event) -> Event` forces the port to name a
  domain vocabulary, which is ADR-0003's prohibition arriving through a different
  door.
- **SY-7 `[PROVISIONAL]`** (`:6033-6052`) — compensation authorship is assigned to at
  most one peer per fact family, a family is identified by tag key (SY-35), and the
  assignment is **runner configuration, not a property of the port**. The alternative
  that lost is *every-peer-adjudicates*, and the clause records why a conditional
  append does not rescue it: a conditional append elects one winner only among the
  peers currently connected, and the premise of the crate is that they are not.

**4. Half the topology question is frozen and the ADR must stay inside it.** SY-9
`[FROZEN]` (`:6090-6106`): hub-ness is a property of an **edge**, not of the port's
type or constructor, and one adapter type must be usable simultaneously as a hub to
one set of peers and a symmetric peer to another. That forbids `Peer::new(is_hub:
bool)` and separate `HubPeer` / `SpokePeer` traits outright. What is open is **SY-10
`[PROVISIONAL]`** (`:6110-6133`): whether the *runner* expresses the two topologies
as one configuration or two, given that it must permit a different merge rule in each
direction of one edge. ADR-0027 answers that, names the alternative that lost, and
hands the *exercise* to `hub-and-spoke-and-peer-to-peer-topologies` (HS-S0110). An
answer that would require a shape SY-9 forbids is not an answer; it is an amendment
to a frozen clause, which takes a new ADR and a re-plan.

**5. The message set instantiates `Envelope<T>`; it never wraps it.**
`crates/happenstance-sync/src/wire.rs:8-16` states the reason and it is structural:
an `enum Message { Push(..), Pull(..) }` would put the message set *beside* the
version check, and the version check must run before any part of the message is
decoded (WF-8). Two consequences the ADR must carry explicitly:

- **The derives are authorised by name or not at all.** Each of `PushBatch`
  (`crates/happenstance-sync/src/peer.rs:191-196`), `EventGroup` (`:236-243`) and
  `ReplicatedEvent` (`crates/happenstance-sync/src/identity.rs:171-180`) is named
  individually with the exact derive list. The ADR that says "the message types
  become serialisable" passes `redkiln validate --kb`, merges, and leaves the next
  implementer free to put a derived `Deserialize` on a type that decodes every field
  before the version can be examined.
- **`Envelope`'s `Deserialize` stays hand-written.** The derive is not an option for
  the decoding half (`crates/happenstance-sync/src/lib.rs:78-84`), and nothing this
  ADR authorises may make it look like one.

**6. `FORMAT_VERSION`'s disposition is this ADR's, and it is a fork with a cost on
each side.** Per-message versioning is what the field's placement implies today;
per-connection negotiation makes the field dead weight on every message after the
first, and removing it later is a format break
(`.kb/open-questions/sync-message-set-and-format-version.md`, *What is not decided*).
Pick one, say what a bump then measures, and keep WF-9's line intact: the version
bumps on a **shape** change and never on a capacity bound
(`crates/happenstance-sync/src/wire.rs:46-65`). The open-question atom's three ordered
sub-questions are the checklist; answering sub-question 2 without sub-questions 1 and
3 leaves the atom half-resolved and HS-S0100 with nothing to record.

**7. Deferrals are renewed against a *named* experiment, and one range boundary is
genuinely in dispute.** SY-27 and SY-28 stay `[DEFERRED]` against the whole-log-versus-scoped
experiment at `references/evaluation/PRESSURE-TEST.md:688-693` — the file exists, so
the citation resolves and CF-38 is satisfied. A renewal with no experiment is a build
failure, not a lapse (`spec/SPECIFICATION.md:213-217`). Separately: **`RUNBOOK.md:4567-4572`
names SY-14 (idempotent bulk ingest in bounded round trips) inside ADR-0027's work
item, while the re-derived split gives SY-8 – SY-18 to ADR-0026**
(`_decomposition.md`, *Tension 3*). That is a real disagreement between two
documents, not a typo. Settle it **with** ADR-0026 and write the settlement into both
records; what AC-014 forbids is a clause assigned twice or to nothing, not a boundary
drawn one way rather than the other.

**8. The arithmetic, computed rather than asserted.** ADR-0027's range is SY-1 – SY-7
(7), SY-19 – SY-31 (13) and SY-35 (1) = **21 clauses**; ADR-0026's is SY-8 – SY-18
(11) plus SY-33 and SY-34 (2) = **13**; SY-32 is a named handoff to ADR-0028
(`spec/SPECIFICATION.md:7000-7003`, `RUNBOOK.md:307`). 21 + 13 + 1 = **35**, which is
the project's stated range. Two things follow. This range contains **every one** of
the nine `[PROVISIONAL]` SY clauses (SY-7, 10, 20, 21, 22, 23, 29, 30, 31), so a
marker moved here is a marker moved by decision — which is what an ADR is for —
whereas a marker left alone is a falsifier this ADR judged still standing, and both
must be said out loud. And **citing a clause is not discharging it**: ADR-0026 cites
SY-1 and SY-6 while ADR-0027 carries them in its range, and the exit computation in
`clause-arithmetic-and-deferral-renewals` (HS-S0113) reads the ranges, not the
citations.

**9. SY-35 is the clause the whole suite rests on, and it is in this range.**
*Anything replication must reason about MUST be in the tags* (`:6868-6900`,
`[FROZEN]`): a value an ingest path, an adjudicator, a compensation or a convergence
check needs MUST NOT be reachable only from `Event::data` or `Event::metadata`. ADR-0027
restates it as its **own** constraint on the suite it schedules, because DR-5 rests on
it and because a suite that parsed a payload would certify a peer that does — the exact
guarantee ADR-0003 exists to buy (`RUNBOOK.md:4576-4580`).

**10. The phase-2 sketch's findings must move into this ADR before the prose that
holds them is deleted.** `crates/happenstance-sync/` is currently the most honestly
self-documenting crate in the workspace; implementing it will delete most of that prose
because it will stop being true, and **the findings are not the same as the
`todo!()`s** (`_decomposition.md`, architecture brief *Notes*). The ones in this ADR's
territory: `src/peer.rs:43-50` (the port cannot express "one round trip with no held
state"; a fixture must count), `src/peer.rs:95-118` (an opaque `Resume` defers the
scope question into the adapter without deferring the port, because a scalar cannot
distinguish "not yet received" from "filtered out"), and
`tests/cursor_shape_probe.rs` (the compiled proof that `impl Stream` admits the
one-shot-HTTP peer only *by buffering and replaying* — legal, `Send`, and a lie).

**11. The reader this is written for.** Not a screen. It is the adapter/peer author
who has to implement `SyncPeer` for a transport nobody in this repository has seen,
and the three implementers immediately downstream — HS-S0105, HS-S0106, HS-S0110 —
who may add **only** what this atom names. The journey the initiative charter cares
about is *learn when you are finished*: an adapter author reads the atom, learns which
merge shape is theirs to supply and which the port supplies, and does not have to
reconstruct it from three clause bodies and a module doc comment.

## Integration contract

- **Archetype**: `foundation`. It lands no runtime behaviour and is consumed inside
  this same project by four capability stories (`blocks: HS-S0100, HS-S0105,
  HS-S0106, HS-S0110`) — never a double, a flag or a `todo!()`.
- **Slice / milestone**: `decisions-of-record`. Slice-mates:
  `adr-0026-peer-ingest-and-transport` (HS-S0098, hard predecessor — it states the
  range this ADR complements) and `open-questions-resolved-and-indexed` (HS-S0100,
  which records this ADR's answers against the two open-question atoms). The three are
  implemented in one context and land as one integrated decision surface; the merge
  order inside the slice is 0098 → 0099 → 0100 (`_storymap.md`, *Merge order* 1).
- **Mount point**: **`.kb/maps/decision-map.md`** — the corpus's decision index and
  the only place the whole supersession graph is visible at a glance. Its own
  `summary` says it is "updated by the Maps phase of every kb-ingest wave that lands a
  new or superseded decision atom" (`:7-11`), and its table today runs ADR-0001 –
  ADR-0016 plus ADR-0029. An atom in `.kb/decisions/` that this map does not list is a
  decision built and never wired in: a reader who does not already know the filename
  never reaches it. Mounted means a row in the ADR table with atom id, title, status,
  phase and supersession column, and reciprocal `related` links that
  `redkiln validate --kb` resolves in both directions.
- **Wires into**:
  - `references/adr/0027-*.md` (new) — the long-form record the atom summarises and
    that `spec/SPECIFICATION.md` will cite by line range; the shape to follow is
    `references/adr/0016-the-wire-format.md:1-25` (Status / Date / Settles / Corrects /
    Rests on and does not settle).
  - `.kb/_intake/` + `/redkiln:kb-ingest` — the only authoring mechanism
    (`.kb/_intake/README.md`; `CLAUDE.md`, *Where the work lives*).
  - `.kb/decisions/0016-the-wire-format.md` — the envelope this ADR builds **on**, and
    the atom that explicitly hands the message set here: *"SY-30 stays ADR-0027's;
    `SyncError` stays ADR-0026's"* (`references/adr/0016-the-wire-format.md:6-11`).
  - `.kb/decisions/0003-opaque-payloads.md` — the guarantee SY-35 and DR-5 protect;
    cited, not amended (its lift is `adr-0003-provisional-lift`, HS-S0107).
  - `.kb/open-questions/sync-message-set-and-format-version.md` — the referent for
    decision 6; its three ordered sub-questions are this ADR's checklist and HS-S0100
    records the resolution against it.
  - `spec/SPECIFICATION.md` §5.1 – §5.9 — the clause bodies this ADR's range covers,
    read (and cited by id and line) but not edited.
  - `crates/happenstance-sync/src/{peer,wire,identity,lib}.rs` and
    `tests/cursor_shape_probe.rs` — the evidence base; read for the sketch's findings,
    unchanged by this PR.
- **Public items**: **none**. The project's signed-off `_design.md` records *no
  user-facing surface* and `N/A` for every items / signatures / visibility / doctest
  section (approved 2026-08-12). This story adds, changes and removes zero public Rust
  items; the items it *authorises* are HS-S0106's to write.
- **Renders surfaces**: **none**, and declared rather than skipped —
  `design.capture` is deliberately absent from `.redkiln/config.yaml`, which makes the
  perceptual review a declared skip (`CLAUDE.md`, *Where the work lives*).
- **Conformance rule(s)**: **none added here, and that is not the same as
  none named.** This ADR's range schedules eight rules that do not exist yet, every
  one carrying `(new, happenstance-sync-testkit)` on its `Rule:` line today:
  `compensation_is_atomic_with_the_losing_event` (SY-2),
  `compensation_is_idempotent_under_redelivery` (SY-3),
  `only_the_adjudicator_compensates` (SY-7), `one_adapter_serves_both_roles` (SY-9),
  `directional_merge_rules_compose` (SY-10),
  `push_envelope_preserves_group_boundaries` (SY-30),
  `watermark_advances_transactionally_with_a_read_model` (SY-31) and
  `the_sync_suite_never_decodes` (SY-35). The ADR binds them — before any of them is
  landed, the plausible wrong peer it rejects is named and compiled into the testkit's
  own `tests/` in the same change (`CLAUDE.md`, *The rule that matters*) — and
  `headline-rules-and-mutant-registry` (HS-S0105) writes the first three.
- **Clause(s)**: this ADR's **range** is `SY-1 – SY-7`, `SY-19 – SY-31` and `SY-35`;
  ADR-0026's is `SY-8 – SY-18` + `SY-33`/`SY-34`; SY-32 is a named handoff. No clause
  text is edited and no `[FROZEN]` clause is approached — SY-2, SY-3, SY-9 and SY-35
  are cited and inherited, and the only marker this ADR may move is a
  `[PROVISIONAL]` one inside its own range, moved explicitly and with the falsifier
  addressed. Clause **repairs** (the `(new)` markers, the `Rejects:` targets) are
  `frozen-clause-repairs`' (HS-S0112).
- **Advances DoD scenario**: initiative **DoD 14** — *"Replication has an answer on
  disk. An accepted decision atom answers whether ingest re-checks a writer's asserted
  conditions — or explicitly refuses, with reasons — the corresponding open-question
  atom reflects that resolution, and `redkiln validate --kb` passes"*
  (`initiative.md:398-401`), whose *second half* — what happens instead of a re-check,
  and what the messages carrying it are — is this atom's. Also project **DoD 5**, and
  project **AC-001** jointly with HS-S0098.

## PR boundary

```
references/adr/0027-*.md
.kb/_intake/**
.kb/decisions/**
.kb/maps/**
.kb/_governance/integration-waves/**
.bklg/from-contract-to-published-library/replication-identity-and-ingest/adr-0027-merge-compensation-and-message-set/**
```

**In this PR**

- The long-form record `references/adr/0027-*.md`, in the house shape, carrying the
  compiler/scenario evidence and every rejected alternative at full length.
- The `.kb/_intake/` draft, the `/redkiln:kb-ingest` wave under a **distinct** wave id
  (two exist: `.kb/_governance/integration-waves/2026-08-10-intake` and
  `-intake-2`), the resulting atom under `.kb/decisions/`, and `.kb/_intake/` cleared
  back to its README.
- The decision-map mount and whatever reciprocal `related` backlinks the Maps phase
  wires — including onto accepted decision atoms, which is legal because `related` sits
  **outside** `validate --kb`'s frozen signature and nothing else about an accepted
  atom may move (`.kb/decisions/README.md`).
- The `_ledger.md` this story's second pass specifies, in this story's own folder.

**Explicitly not in this PR**

- Any `.rs` file, any `Cargo.toml`, any file under `crates/`, `xtask/`, `examples/`.
  The derives are *authorised* here and *added* in HS-S0106.
- Any edit to `spec/SPECIFICATION.md` — including a clause repair, including dropping
  a `(new)` marker (HS-S0112), and categorically including a `[FROZEN]` clause.
- Re-answering whether ingest re-checks asserted conditions (ADR-0026 / HS-S0098).
- Editing `.kb/decisions/0003-opaque-payloads.md`, `0016-the-wire-format.md` or any
  other accepted atom body (HS-S0107 lifts ADR-0003's marker with a *new* atom).
- Editing the two open-question atoms or `.kb/maps/open-questions-index.md`
  (HS-S0100).
- Claiming `happenstance-sync` / `happenstance-sync-testkit` on crates.io — the
  recorded disposition is *not in this release train* (`_decomposition.md`, *AC-015*).
- `redkiln adopt --templates`, ever (`CLAUDE.md`).

**Merge DoD**: an accepted `kb-decision-0027` atom is on the initiative branch,
introduced by a single kb-ingest wave commit, summarising a long-form record under
`references/adr/`, listed in `.kb/maps/decision-map.md`, stating a clause range that
is exactly the complement of ADR-0026's, authorising the three message-type derive
sets by name, and `redkiln validate --kb && redkiln doctor` green with
`git diff -- spec/SPECIFICATION.md crates/` empty.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The decision arrives as an atom through the ingest path** | Draft staged in `.kb/_intake/`, promoted by `/redkiln:kb-ingest` under a wave id distinct from `2026-08-10-intake` and `-intake-2`, `.kb/_intake/` cleared to its README afterwards. A byte-perfect hand-written atom fails this story in a way `validate --kb` cannot detect — only the commit graph can. | `CLAUDE.md`, *Where the work lives*; `.kb/_intake/README.md`; commit `0269720`; `.kb/_governance/integration-waves/2026-08-10-intake-2/` |
| **The atom and the long form are both authored, and they carry different loads** | `.kb/decisions/0027-*.md` is the canonical ~100-line atom with the frontmatter, status and supersession graph `validate --kb` enforces; `references/adr/0027-*.md` carries the transcripts, the rejected alternatives and the measurement tables a summary cannot hold. Cite the record by `file:line`, link the atom. | `CLAUDE.md`, *Where the work lives*; `references/adr/0016-the-wire-format.md:1-25` as the shape |
| **The merge rule is stated as SY-2 freezes it, with compensate-after-commit named as the alternative that lost** | The losing event and its compensation are one `append` call under one guard; no reader may observe the losing event with nothing resolving it. The ADR reproduces *why* the loser lost — the window survives its own closing — rather than asserting the MUST. | `spec/SPECIFICATION.md:5871-5900`; `spec/E2E-CASES.md:1018` (E2E-39) |
| **The port/domain seam is stated as a closure, not a trait method** | The port supplies atomicity, identity and idempotence; the domain supplies content. `fn compensate(&self, losing: &Event) -> Event` is refused by name, because it makes the port name a domain vocabulary — ADR-0003's prohibition through a different door. The precedent for the closure is ADR-0007's `pump`. | `spec/SPECIFICATION.md:5901-5930`; `references/adr/0007-projection-runner-decodes.md:62-67` |
| **Adjudication is a role, and the role belongs to the runner** | At most one peer per fact family; a family is a tag key (SY-35); assignment is runner configuration and not a property of the port. `every-peer-adjudicates` is named as the loser, with the reason a conditional append does not rescue it. SY-7 is `[PROVISIONAL]` and its falsifier is restated or moved deliberately. | `spec/SPECIFICATION.md:6033-6052` |
| **The topology answer is taken at SY-10 and stays inside frozen SY-9** | The ADR says whether the runner expresses hub-and-spoke and peer-to-peer as one configuration or two, and names the loser. It may not reach a conclusion requiring `Peer::new(is_hub: bool)` or `HubPeer`/`SpokePeer` — SY-9 forbids both and is frozen. The exercise (one adapter in both roles on different edges) is HS-S0110's. | `spec/SPECIFICATION.md:6090-6106` (SY-9), `:6110-6133` (SY-10); `crates/happenstance-sync/src/lib.rs:52-66` |
| **The message set instantiates `Envelope<T>` and never wraps it** | New types instantiate `T`; the envelope's shape does not change and does not become an enum of message kinds, because that would put the message set beside the version check WF-8 requires to run first. | `crates/happenstance-sync/src/wire.rs:8-16`; `_decomposition.md`, *The wire mounts inside `Envelope<T>`* |
| **Each derive is authorised by name, per type** | `PushBatch`, `EventGroup` and `ReplicatedEvent` are named individually with their exact derive lists, and the ADR states that `Envelope`'s `Deserialize` remains hand-written. "The message types become serialisable" is a failing formulation: it validates, merges, and licenses a derived `Deserialize` that decodes every field before the version is examined. | `crates/happenstance-sync/src/lib.rs:78-94`; `crates/happenstance-sync/src/peer.rs:191-196`, `:236-243`; `crates/happenstance-sync/src/identity.rs:171-180`; `_decomposition.md` AC-A08 |
| **`FORMAT_VERSION`'s disposition is chosen, with the cost of the other side stated** | Per-message versus per-connection-negotiated, plus what a bump then measures — shape, never a capacity bound (WF-9). All three of the open question's ordered sub-questions are answered, including whether removing the field later would be its own ADR. | `crates/happenstance-sync/src/wire.rs:46-65`; `.kb/open-questions/sync-message-set-and-format-version.md`, *Ordered sub-questions* |
| **SY-35 is restated as this ADR's own constraint on the suite** | Everything replication reasons about lives in `EventType`/`Tags`; no rule in the suite this ADR schedules may decode `Event::data` or `Event::metadata`, on any path. This is DR-5's basis and ADR-0003's payoff, not a second copy of the clause. | `spec/SPECIFICATION.md:6868-6900`; `project.md` DR-5; `RUNBOOK.md:4576-4580` |
| **Deferrals are renewed against a named experiment** | SY-27 and SY-28 stay `[DEFERRED]` citing `references/evaluation/PRESSURE-TEST.md:688-693` by name; the port-side reason they cannot be settled on paper (a scalar resume token cannot distinguish "not yet received" from "filtered out") is recorded from `peer.rs`. A renewal without a named experiment is a build failure under CF-38. | `spec/SPECIFICATION.md:6629-6660`, `:213-217`; `references/evaluation/PRESSURE-TEST.md:687-693`; `crates/happenstance-sync/src/peer.rs:95-118` |
| **The SY-14 boundary is settled by name, with ADR-0026** | `RUNBOOK.md:4567-4572` puts SY-14 in ADR-0027's work item; `_decomposition.md` *Tension 3* puts SY-8 – SY-18 in ADR-0026's range. Either boundary is defensible; silence is not. Whichever lands is written into **both** records so no clause is assigned twice or to nothing. | `RUNBOOK.md:4567-4572`; `_decomposition.md`, *Tension 3*; `spec/SPECIFICATION.md:6234-6260` (SY-14) |
| **The clause range is written down as a range, and the arithmetic is shown** | SY-1 – SY-7 + SY-19 – SY-31 + SY-35 = 21; ADR-0026's 13; SY-32 handed off = 35, this project's stated range. Stated in the atom so HS-S0113 can add the two ranges and compare rather than re-derive them. | `project.md` AC-014, DR-9; `_decomposition.md`, *Tension 3*; `spec/SPECIFICATION.md:7000-7003` |
| **Every provisional marker in range is judged out loud** | The range holds all nine `[PROVISIONAL]` SY clauses. Each is either moved by this decision (with the falsifier addressed) or left standing (with the falsifier restated as still open). A marker that changes silently is the drift `spec-trace` exists to catch. | `spec/SPECIFICATION.md:6040`, `:6113`, `:6433`, `:6466`, `:6487`, `:6508`, `:6674`, `:6706`, `:6734` |
| **The sketch's findings are moved before the prose that holds them is deleted** | The round-trip-counting limit (`peer.rs:43-50`), the opaque-`Resume` argument (`peer.rs:95-118`) and the buffering-`impl Stream` probe (`tests/cursor_shape_probe.rs`) are carried into the long-form record, so implementing the crate does not discard the evidence the phase-2 sketch landed nine phases early to produce. | `_decomposition.md`, architecture brief *Notes*; `RUNBOOK.md:462-466` |
| **No frozen clause, no accepted atom, no line of Rust moves** | `git diff` over `spec/SPECIFICATION.md` and `crates/` is empty for this PR; the only `.kb/decisions/` edits are additive `related:` backlinks the Maps phase wires. A correction to an accepted atom is a new atom, never an edit — the pattern is ADR-0029 amending ADR-0004 with `depends_on` and no `supersedes`. | `.kb/decisions/README.md`; `.kb/decisions/0029-msrv-raised-to-1-97-1.md:12-26`; `project.md` DoD 7 |
| **`redkiln validate --kb` and `redkiln doctor` are the authority, and are relayed verbatim** | Both run on the merged tree; their verdicts are transcribed, not paraphrased, and a hand check never substitutes for them — nor reports a problem they did not report. `doctor` is expected to show exactly six `template-drift` advisories and no more. | `CLAUDE.md`, *Commands* and *Where the work lives*; `project.md` AC-001, AC-013 |

## Data and migrations

**No database, no schema migration, no code.** There *is* a data contract, and it is
half the deliverable: one markdown atom whose frontmatter must satisfy
`KbFrontmatter` **and** the `.kb/decisions/` layer convention at the same time — the
first is machine-checked, the second is not.

**Destination shape**, taken from the existing decision atoms rather than invented
(`.kb/decisions/0016-the-wire-format.md:1-11` is the closest sibling):

| Field | Value for this atom |
| --- | --- |
| `id` | `kb-decision-0027`, matching the corpus's `kb-decision-NNNN` convention and the number reserved at `RUNBOOK.md:306` |
| `title` | one decision, not three — the "and" test in `.kb/playbooks/one-decision-per-adr-title.md` applies to this title before it applies to anyone else's |
| `kind` / `status` / `authority_tier` | `decision` / `accepted` / `decision` |
| `adr_id` | `ADR-0027` |
| `phase` | `13` |
| `reversibility` | stated deliberately: the message-set shape is cheap to reverse **only** while both sync crates read `publish = false` (`crates/happenstance-sync/Cargo.toml:12`), and a derive on a public message type is a wire format the moment either is published |
| `supersedes` / `superseded_by` | `null` / `null` — this ADR supersedes nothing; it takes territory ADR-0016 explicitly declined (`references/adr/0016-the-wire-format.md:6-11`) |
| `depends_on` | `kb-decision-0016` (the envelope it builds on) and ADR-0026's atom (the range it complements); `kb-decision-0003` if the atom leans on opaque payloads for SY-35 — every id must resolve or `validate --kb` fails |
| `summary` | carries the merge rule, the compensation contract, the topology answer, the derive authorisation and the clause range in its own words — a reader who reads only frontmatter must not miss which derives were authorised |
| `source_paths` | the `.kb/_intake/…` draft the wave consumed, `references/adr/0027-*.md`, and the `spec/SPECIFICATION.md` / `crates/happenstance-sync/src/` paths the reasoning rests on |
| `last_reviewed` | the wave date |

**Only the fields the corpus already uses.** `KbFrontmatter` is a passthrough object,
so an invented key validates silently and is read as corpus fact by every later wave.
Author against `.kb/_templates/atom.md` and the existing decision atoms, and nothing
beyond them.

**The one destructive operation** is the intake clear: the staged draft is removed as
part of the wave commit. It is reversible by construction — the draft stays in git
history and the atom cites its `.kb/_intake/…` path in `source_paths`
(`.kb/_intake/README.md`, *A successful ingest clears this directory*). The one file
that must survive it is `.kb/_intake/README.md`, which is scaffolding rather than
staged content, so narrow the wave's glob or pass an explicit file list.

**No backfill and no migration.** Nothing in the corpus occupies `kb-decision-0027`
today, no existing atom's frozen content changes, and the only edits outside
`.kb/decisions/0027-*.md` are the decision-map row and additive `related` backlinks —
both of which are mounts, not migrations.

## Acceptance criteria

The persona throughout is the one the initiative charter names for this work: the
**adapter/peer author**, on the journey *learn when you are finished* — from a
signature that type-checks to a written answer that says which merge shape is theirs
to supply (`initiative.md:241-250`). Two others appear where the criterion is
genuinely theirs: the **constrained-runtime developer** (AC-004, because a topology
answer that assumes a socket excludes them) and the **evaluator** deciding in one
sitting (AC-007, AC-009, because an unrenewed deferral and an unlisted atom both read
as an unfinished project from outside). Each criterion is a goal crossing the whole
stack this story has — question → long form → atom → map → gate — not a file that
exists.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **The decision arrives the way this repository makes decisions.** GIVEN an adapter author who wants to know how the merge rule was settled and by whom, WHEN they look under `.kb/decisions/`, THEN an accepted atom `kb-decision-0027` is there, introduced by a **single** `/redkiln:kb-ingest` wave commit under a wave id distinct from `2026-08-10-intake` and `-intake-2`, with the draft it consumed deleted from `.kb/_intake/` in that same commit and `.kb/_intake/README.md` surviving it. A byte-perfect hand-written atom does not satisfy this criterion. | `redkiln validate --kb` green (`CLAUDE.md`, *Commands*); `git log --oneline --diff-filter=A -- .kb/decisions/0027-*.md` names exactly one commit, and `git show --stat` on it lists both the added atom and the deleted `.kb/_intake/…` draft; `git ls-files .kb/_intake/` returns `README.md` only; a wave directory exists under `.kb/_governance/integration-waves/` whose name matches neither existing wave |
| **AC-002** | **The reasoning survives the summary.** GIVEN the same author, now needing the transcripts and the alternatives a ~100-line atom cannot hold, WHEN they follow the atom's `source_paths`, THEN `references/adr/0027-*.md` exists in the house shape — Status / Date / Settles / Corrects / Rests on and does not settle — carries every rejected alternative at full length, and is reachable from the atom rather than inferred from the number. | `references/adr/0027-*.md` exists and its header block carries the same five fields as `references/adr/0016-the-wire-format.md:1-25`; the atom's `source_paths` lists that path and `redkiln validate --kb` resolves it; every `file:line` citation in the record opens to the text it claims |
| **AC-003** | **The author learns which half of the merge is theirs.** GIVEN a peer author whose `ingest` has just found an incoming event conflicting with a fact the receiver already holds, WHEN they read ADR-0027, THEN they learn that the compensation is appended in the **same** `append` call as the losing event (SY-2), that the port supplies atomicity, identity and idempotence while the **domain** supplies the compensation's content through a caller-supplied closure (SY-3), and that authorship of a compensation belongs to at most one peer per fact family as **runner configuration** (SY-7) — each with the alternative that lost named and the reason it lost: compensate-after-commit, a `fn compensate(&self, losing: &Event) -> Event` port method, and every-peer-adjudicates. | Reviewer check of `references/adr/0027-*.md` against `spec/SPECIFICATION.md:5871-5900`, `:5901-5930`, `:6033-6052` — all three losers present **by name** with a reason, not a MUST restated; `cargo xtask spec-trace` green; `git diff --exit-code -- spec/SPECIFICATION.md` empty |
| **AC-004** | **The topology answer does not exclude the edge.** GIVEN a constrained-runtime developer whose Worker must be a hub to nine tablets and a symmetric peer to a shore depot **at the same time**, WHEN they read ADR-0027, THEN it answers SY-10 — whether the runner expresses the two topologies as one configuration or two, given a different merge rule in each direction of one edge — names the alternative that lost, stays inside frozen SY-9 (hub-ness is a property of an edge), and hands the *exercise* to `hub-and-spoke-and-peer-to-peer-topologies` rather than asserting it. | Reviewer check against `spec/SPECIFICATION.md:6090-6106` (SY-9) and `:6110-6133` (SY-10); `Peer::new(is_hub: bool)`, `HubPeer` and `SpokePeer` appear in the record **only** as refused shapes; `cargo xtask spec-trace` green; the SY-9 clause body is unchanged in the diff |
| **AC-005** | **The next implementer knows exactly what they may add.** GIVEN HS-S0106's implementer opening the atom to learn which derives are permitted on the message set, WHEN they read it, THEN `PushBatch`, `EventGroup` and `ReplicatedEvent` are each named **individually** with an exact derive list, the record states that the message set instantiates `Envelope<T>` and never becomes an enum around it, and it states that `Envelope`'s `Deserialize` stays hand-written. A formulation such as "the message types become serialisable" fails this criterion even though it validates and merges. | Grep `.kb/decisions/0027-*.md` and `references/adr/0027-*.md`: each of the three type names appears with a derive list adjacent to it; cross-check the type paths still exist at `crates/happenstance-sync/src/peer.rs:191-196`, `:236-243` and `crates/happenstance-sync/src/identity.rs:171-180`; the hand-written-`Deserialize` sentence is present and consistent with `crates/happenstance-sync/src/lib.rs:78-84` |
| **AC-006** | **`FORMAT_VERSION` stops naming a vocabulary nobody chose.** GIVEN a peer author writing the first message their transport will ever send, WHEN they ask whether the version travels on every message or is negotiated once per connection, THEN ADR-0027 answers it, states the cost of the side that lost (dead weight on every message versus a format break to remove the field later), says what a bump then **measures**, and keeps WF-9's line intact — the version bumps on a shape change and never on a capacity bound. All three ordered sub-questions of the open-question atom are answered, so HS-S0100 has a resolution to record rather than a further question. | Reviewer check against `.kb/open-questions/sync-message-set-and-format-version.md` (*Ordered sub-questions*) — each sub-question has an answer in the record — and against `crates/happenstance-sync/src/wire.rs:46-65`; `redkiln validate --kb` |
| **AC-007** | **Nothing in this range is deferred by silence.** GIVEN an evaluator deciding in one sitting whether replication is finished enough to adopt, WHEN they read the `[DEFERRED]` clauses in ADR-0027's range, THEN SY-27 and SY-28 are renewed against the whole-log-versus-scoped experiment at `references/evaluation/PRESSURE-TEST.md:687-693` **by name**, with the port-side reason they cannot be settled on paper recorded; and the SY-14 boundary disagreement — `RUNBOOK.md:4567-4572` places it in ADR-0027's work item, `_decomposition.md` *Tension 3* places SY-8 – SY-18 in ADR-0026's range — is settled by name and written into **both** records so no clause is assigned twice or to nothing. | `cargo xtask spec-trace` green — CF-38 makes a renewal with no named experiment a build failure (`spec/SPECIFICATION.md:213-217`, `xtask/src/spec_trace.rs`); both ADR records state the same SY-14 assignment; the cited `PRESSURE-TEST.md` line range resolves to the whole-log-versus-scoped question |
| **AC-008** | **The split adds up, and it can be added up by someone else.** GIVEN HS-S0113 at project exit, whose job is to add ADR-0026's range to ADR-0027's and compare the total against this project's stated range, WHEN they read this atom, THEN the range is written **as a range** — SY-1 – SY-7, SY-19 – SY-31, SY-35 — with the arithmetic shown (21 + 13 + 1 = 35), it is exactly the complement of ADR-0026's, and every one of the nine `[PROVISIONAL]` clauses inside it is judged out loud: moved by this decision with its falsifier addressed, or left standing with its falsifier restated as still open. | Recompute: no `SY-` id appears in both ranges and none appears in neither once SY-32's handoff is counted (`spec/SPECIFICATION.md:7000-7003`); each of SY-7, 10, 20, 21, 22, 23, 29, 30, 31 is named in the record with a disposition; `cargo xtask spec-trace` green (`project.md` AC-014, DoD 2) |
| **AC-009** | **A reader who does not know the filename still finds it, and nothing else moved.** GIVEN an evaluator who arrives at `.kb/maps/decision-map.md` because it is the only place the whole decision corpus is visible at once, WHEN they scan the ADR table, THEN ADR-0027 has a row carrying atom id, title, status, phase and supersession, with reciprocal `related` links that resolve in both directions; and the rest of the tree is untouched — no Rust, no clause text, no accepted atom body. | `redkiln validate --kb && redkiln doctor` green, `doctor` reporting exactly six `template-drift` advisories and no more (`CLAUDE.md`, *Where the work lives*); `git diff --exit-code -- spec/SPECIFICATION.md crates/ xtask/ examples/ Cargo.toml` empty; `git diff -- .kb/decisions/` shows the new atom plus additive `related:` backlinks only |

## Interaction quality

**This story renders no surface, and that is a signed-off determination rather than a
skip.** The project's `_design.md` records `N/A — no user-facing surface` in every
one of its items / signatures / visibility / states / anti-patterns / doctest
sections and was approved on 2026-08-12 with *"no mock was produced or owed"* and
*"conditions: none"* (`_design.md:116-126`). `design.capture` is deliberately absent
from `.redkiln/config.yaml`, which makes the perceptual review a declared skip
(`CLAUDE.md`, *Where the work lives*). So the STATE family — in-place versus
context-jump, non-occlusion, preserved focus/scroll/selection, reversibility,
keyboard reachability — and the COMPOSITION family — presentation exists at all,
placement, transience, density budget, hierarchy, named anti-patterns — have **no
rendered referent here**. There is no control to style, no chrome to keep persistent,
and no density number to hold, because the design that would set them records none.

What *is* load-bearing, and what the invariants above translate into for a decision
record, is stated here only as a pointer to the AC rows that carry it — per RFC
§6.7/D6, an invariant with no table row is never gated:

| Invariant, in the medium this story actually has | Carried by | How it is verified |
| --- | --- | --- |
| **Reachability — the analogue of "presentation exists at all".** An atom nobody can navigate to is a decision built and never wired in. The mount is the decision map's ADR table row plus reciprocal `related` links. | **AC-009** | `redkiln validate --kb` resolves the links in both directions; the row is visible in `.kb/maps/decision-map.md` |
| **Two-layer disclosure — the analogue of transience.** The atom is the persistent, always-loaded layer; the long-form record is opened on demand and carries what the atom cannot. Collapsing them either way is the failure: a 1,500-line atom, or a summary with the alternatives deleted. | **AC-002** | The record exists in house shape and the atom reaches it through `source_paths` |
| **Density budget, expressed as the corpus expresses it.** ~100 lines for the atom, matching the sixteen decision atoms already in `.kb/decisions/`; the long form takes whatever length the evidence needs. Only fields the corpus already uses — `KbFrontmatter` is a passthrough object, so an invented key validates silently. | **AC-001**, **NF-002** | `redkiln validate --kb`; frontmatter diffed field-for-field against `.kb/decisions/0016-the-wire-format.md:1-11` |
| **Hierarchy — one decision per title.** The "and" test in `.kb/playbooks/one-decision-per-adr-title.md` applies to this title before it applies to anyone else's, and the record must not restate ADR-0026's answer in its own slightly different words. | **AC-003**, **AC-008** | Reviewer check; the range is the complement, and the central question appears only as an inherited citation |
| **Reversibility, stated rather than assumed.** The message-set shape is cheap to reverse only while both sync crates read `publish = false`; the atom's `reversibility` field says so. | **NF-003** | `crates/happenstance-sync/Cargo.toml` still carries `publish = false`; the field is present and specific |
| **The named anti-pattern this story does have.** Not from `_design.md`, which records none, but from this story's own discovery: *the mutant that is the ADR* — prose that authorises the derives without naming the three types, which validates, merges, and licenses a derived `Deserialize` that decodes every field before the version is examined. | **AC-005** | The three type names each appear with an exact derive list; the generic formulation fails the row |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | `redkiln validate --kb` fails on the new atom — an unresolvable `depends_on`, a missing required field, or a malformed `KbFrontmatter`. | Fix the atom's frontmatter and re-run the wave; never hand-patch the atom in place after the wave has landed, and never relax the check. `depends_on` ids must resolve — `kb-decision-0016`, ADR-0026's atom id, and `kb-decision-0003` if SY-35's argument leans on opaque payloads. |
| **EC-002** | The ingest wave clears `.kb/_intake/README.md` along with the draft. | The README is scaffolding, not staged content. Restore it in the same commit, and narrow the wave's glob or pass an explicit file list so the next wave cannot repeat it (`.kb/_intake/README.md`). |
| **EC-003** | ADR-0026 lands a range different from `SY-8 – SY-18` + `SY-33`/`SY-34` — the predecessor moved its own boundary. | ADR-0027's range is defined as the **complement**, not as a literal. Recompute from what ADR-0026 actually states, re-show the arithmetic, and only then write the range down. The story is blocked on 0026 for exactly this reason. |
| **EC-004** | The SY-14 boundary is settled one way in ADR-0027 and the other way in ADR-0026 — or in neither. | Both are failures of AC-007 and AC-014, and the second is worse because it is silent. Settle it once, with 0026, and write the identical assignment into both records before either merges. |
| **EC-005** | Drafting finds SY-2, SY-3, SY-9 or SY-35 — all `[FROZEN]` — actually wrong. | Stop. The output is a recorded finding and a re-plan, never a softer sentence in ADR-0027 and never an edit to the clause. Changing a `[FROZEN]` clause takes a new ADR (`CLAUDE.md`, *Open questions*; `project.md` AC-002). |
| **EC-006** | The wave id collides with `2026-08-10-intake` or `-intake-2`. | Choose a distinct id before running the wave. A collision makes the provenance of two waves indistinguishable, and provenance is the only thing that separates AC-001 from a hand-written file. |
| **EC-007** | The derive list for one of the three types cannot be settled without seeing the encoder chosen in phase 13. | Say so explicitly and scope the authorisation to what *is* decidable — for example authorising `Debug`/`Clone`/`PartialEq` and naming the serialisation derives as still owed, with the owner. An unqualified silence is what lets HS-S0106 choose for itself; a qualified deferral does not. |
| **EC-008** | `cargo xtask spec-trace` fails after the ADR lands, citing a `[DEFERRED]` clause with no named experiment. | CF-38 is doing its job. Add the named experiment or settle the clause; do not renew by prose. A missing experiment file is a build failure, not an advisory (`spec/SPECIFICATION.md:213-217`). |
| **EC-009** | A `related:` backlink the Maps phase wires would change something in an accepted atom other than `related`. | `related` sits outside `validate --kb`'s frozen signature; nothing else about an accepted atom may move. If more than `related` changes, revert and write a new atom instead (`.kb/decisions/README.md`; `.kb/decisions/0029-msrv-raised-to-1-97-1.md:12-26` is the pattern). |

## Non-functional

| id | requirement | why |
| --- | --- | --- |
| **NF-001** | Every `file:line` citation in the atom and the long-form record resolves to the text it claims, at the commit that merges. | `spec/SPECIFICATION.md` cites `references/adr/` by line range and `cargo xtask spec-trace` reads those citations; a record whose citations rot takes the specification's traceability with it (`CLAUDE.md`, *Where the work lives*). |
| **NF-002** | The atom uses **only** fields the existing decision corpus already uses. | `KbFrontmatter` is a passthrough object, so an invented key validates silently and is read as corpus fact by every later wave. Author against `.kb/_templates/atom.md` and `.kb/decisions/0016-the-wire-format.md:1-11`. |
| **NF-003** | `reversibility` is specific rather than a word: the message-set shape is cheap to reverse only while both sync crates read `publish = false`, and a derive on a public message type is a wire format the moment either is published. | It is the field a future reader uses to decide whether reopening this is cheap, and the honest answer here has a condition attached (`crates/happenstance-sync/Cargo.toml`). |
| **NF-004** | The atom is self-sufficient for its stated reader: an adapter author who has never opened `spec/SPECIFICATION.md` learns which merge shape is theirs to supply and which the port supplies, without reconstructing it from three clause bodies and a module doc comment. | That is the *learn when you are finished* journey the initiative names (`initiative.md:243-245`), and the whole reason this ADR is written before the code. |
| **NF-005** | `redkiln doctor` reports exactly six `template-drift` advisories — no more, no fewer. | A seventh is a template changed without deciding to; a missing one is a customisation reverted by `adopt --templates`, which is forbidden here (`CLAUDE.md`, *Where the work lives*). |
| **NF-006** | The eight conformance rules this ADR's range schedules are named in the record with their clause ids, and each is bound to the standing bar: before any of them lands, the plausible wrong peer it rejects is named and compiled into the testkit's own `tests/` in the same change. | A rule no adapter can fail is decorative, and this is the last document that can bind the eight before HS-S0105 writes the first three (`CLAUDE.md`, *The rule that matters*). |
| **NF-007** | The phase-2 sketch's three findings — the round-trip-counting limit, the opaque-`Resume` argument, and the buffering-`impl Stream` probe — are carried into the long-form record before the prose holding them is deleted by implementation. | The findings are not the same as the `todo!()`s; implementing the crate will delete most of that prose because it stops being true, and the evidence was landed nine phases early on purpose (`_decomposition.md`, architecture brief *Notes*; `RUNBOOK.md:462-466`). |

## Implementation notes (non-prescriptive)

- **Order of operations.** Read ADR-0026's landed range first; the complement cannot be
  computed before it exists. Then draft the long form, then reduce it to the atom —
  not the other way round. An atom written first tends to become the record, and the
  alternatives that lost are the first thing a summary drops.
- **Draft the long form against a sibling, not against a blank file.**
  `references/adr/0016-the-wire-format.md:1-25` is the closest shape and the closest
  subject; it is also the record that explicitly hands this territory here
  (`:6-11`).
- **Write the losers before the winners.** Compensate-after-commit, the
  `fn compensate` port method, and every-peer-adjudicates are each named in a clause
  body already; lifting the clause's own reasoning is not plagiarism, it is what
  reproducing the division of labour means. What the ADR adds is the trade at full
  length and the evidence.
- **The clause range is a computation, not a claim.** Write the arithmetic in the
  atom in the form HS-S0113 will re-run: two ranges, one handoff, one total. Someone
  will add it up; make that cheap.
- **Nine provisional markers is a checklist, not a paragraph.** Enumerate SY-7, 10,
  20, 21, 22, 23, 29, 30 and 31 explicitly. A marker judged in passing reads
  identically to a marker forgotten.
- **Stage the draft, run the wave, then stop.** The temptation at the end is to fix
  the open-question atom or drop a `(new)` marker while the tree is open. Both belong
  to other stories (HS-S0100, HS-S0112) and both would put this PR outside its
  boundary.
- **Take the whole slice in one context.** `decisions-of-record` is implemented as one
  integrated decision surface: 0026 → 0027 → the open-question resolutions
  (`_storymap.md`, *Merge order* 1). The SY-14 boundary in particular cannot be
  settled by two agents who never see each other's text.

## Tests and CI (merge gate)

Grounded in the project testing brief's *Merge-gate commands* and its per-AC tier
mapping (`_decomposition.md`, testing brief). This story adds no Rust, so the unit and
integration tiers run **unchanged** — their role here is to prove the tree did not
move, which is AC-009's claim.

| tier | command / path | proves |
| --- | --- | --- |
| Static — KB | `redkiln validate --kb` | AC-001, AC-002, AC-006, AC-009. `KbFrontmatter` conformance, `depends_on` resolution, reciprocal `related` links, and accepted-decision immutability against `HEAD` — the check that catches an edit to `0003-opaque-payloads.md`'s or `0016-the-wire-format.md`'s body before it becomes a merge conflict with the discipline itself (testing brief, *Static*, fifth bullet). |
| Static — backlog | `redkiln doctor` | AC-009, NF-005. Exactly six `template-drift` advisories; a seventh or a missing one fails. |
| Static — provenance | `git log --oneline --diff-filter=A -- .kb/decisions/0027-*.md`; `git show --stat <wave-commit>`; `git ls-files .kb/_intake/` | AC-001. The one thing `validate --kb` structurally cannot see: whether the atom came through the ingest path. A hand-written atom is byte-identical and green; only the commit graph separates them (commit `0269720`). |
| Static — specification | `cargo xtask spec-trace` (`xtask/src/spec_trace.rs`) | AC-003, AC-004, AC-007, AC-008. Every `SY`/`WF` clause's `Rule:` resolves, no rule is orphaned, and CF-38 makes a `[DEFERRED]` clause with no named experiment a build failure. This is the mechanism DoD 2 and project AC-014 name by name. |
| Static — no-drift | `git diff --exit-code -- spec/SPECIFICATION.md crates/ xtask/ examples/ Cargo.toml` | AC-009. The PR boundary made mechanical: no clause text, no Rust, no manifest. |
| Static — lints | `cargo xtask lints` | The five-lint story-grain gate `reachability_static` wires unconditionally (`.redkiln/config.yaml`), so a story whose whole deliverable is a decision record still gets checked. |
| Story grain | `cargo xtask affected --base main` | That a documentation-and-KB diff touches no compiled package. Expected to resolve to nothing to rebuild; a non-empty result means something left the PR boundary. |
| Integration (project ceiling) | `cargo xtask ci --fast` | AC-009, and the standing bar `.redkiln/config.yaml` wires to a non-terminal project's integration grain. Contains fmt, clippy `-D warnings`, the test run, the wasm32 steps, docs, `spec-trace`, the `--no-default-features` doc build and the `cargo package --list` assertion (`CLAUDE.md`, *Commands*). Green here proves the tree is unmoved; it never proves an ADR says the right thing. |
| Review — human | `references/adr/0027-*.md`, `.kb/decisions/0027-*.md` read against `spec/SPECIFICATION.md:5871-5900`, `:5901-5930`, `:6033-6052`, `:6090-6133`, `:6868-6900` | AC-003, AC-004, AC-005, AC-006, AC-007, AC-008. The criteria that are claims about *reasoning* — losers named, derives named individually, sub-questions answered, markers judged — have no command that can pass or fail them. The ledger's evidence for these rows cites the record by `file:line`, not a green build. |

**The gap, stated rather than hidden.** Six of the nine ACs bottom out in a reviewer
reading two documents. That is the honest bar for a decision record and it is why the
criteria above are written as *findable facts* — a type name with a derive list beside
it, a loser named in a sentence with a reason, a sub-question with an answer —
rather than as qualities. A criterion a reviewer cannot check by grep-then-read has
been written badly.

## Risks and coupling (PR-scoped)

| risk | why it bites here | mitigation in this PR |
| --- | --- | --- |
| **The predecessor's range moves.** ADR-0027 is defined as a complement; if ADR-0026 states a different range than the re-derived ledger predicted, this ADR's range is wrong the moment it is written. | AC-014 fails at project exit, not here, which is the expensive way to find out. | EC-003: compute from what 0026 actually landed. The slice is implemented in one context precisely so this is a conversation, not a discovery. |
| **Re-answering the central question.** The pull to restate *does ingest re-check asserted conditions* in ADR-0027's own words is strong, because SY-2's compensation only makes sense once you have said it. | Two documents then disagree and neither is wrong — the exact failure `.kb/playbooks/one-decision-per-adr-title.md` exists to prevent, and the side-effect authorship reverted at `0269720`. | Context pack decision 2 states the boundary; AC-003 requires SY-1/SY-6 to appear as **inherited citations** only. |
| **The derives authorised in the generic.** "The message types become serialisable" is fluent, passes validation, and merges. | It licenses a derived `Deserialize` that decodes every field before the version is examined — the partial decode WF-8 forbids and the reason `Envelope`'s `Deserialize` is hand-written. | AC-005 makes "by name" the criterion; the anti-pattern is written into *Interaction quality* so it is visible at review, not only at implementation. |
| **The SY-14 boundary settled by whichever document is read last.** `RUNBOOK.md:4567-4572` and `_decomposition.md` *Tension 3* genuinely disagree. | A clause assigned twice or to nothing is exactly what AC-014 forbids, and the arithmetic still adds up in the double-assigned case — so the check that would catch it is the one nobody runs. | AC-007 requires the same assignment sentence in **both** records; EC-004 names the silent variant as the worse failure. |
| **A provisional marker moved in passing.** The range contains all nine `[PROVISIONAL]` SY clauses, and moving one is legitimate — which is what makes an accidental move hard to see. | The drift `spec-trace` exists to catch, arriving through a document `spec-trace` does not read for tone. | AC-008 requires each of the nine named with a disposition; the atom is where the judgement is recorded, so a later reader can tell decision from omission. |
| **Scope creep into the adjacent stories.** The open-question atom, the `(new)` markers and the derives themselves are all one keystroke away while the tree is open. | Each would put this PR outside its stated boundary and take another story's evidence with it. | The *Explicitly not in this PR* list is enumerated; AC-009's `git diff --exit-code` makes the Rust half mechanical. |
| **Coupling into HS-S0105 / HS-S0106 / HS-S0110.** Three capability stories may add **only** what this atom names. | An under-specified atom does not block them — it silently lets them decide, and the decision then lives in a commit message. | NF-006 binds the eight scheduled rules; AC-005 binds the derives; AC-004 hands the topology exercise onward with SY-9 stated as the constraint on it. |

## Dependencies

**Blocks on**

- `adr-0026-peer-ingest-and-transport` (HS-S0098) — hard, not advisory. It re-derives
  the SY/WF clause ledger from `spec/SPECIFICATION.md` at HEAD, states its own range,
  and answers the central question this ADR inherits rather than re-answers. ADR-0027
  cannot state a complement of a range that does not exist yet
  (`_storymap.md`, *Slices*, `depends_on`; *Merge order* 1).

**Unlocks**

- `open-questions-resolved-and-indexed` (HS-S0100) — records this ADR's
  `FORMAT_VERSION` answer against
  `.kb/open-questions/sync-message-set-and-format-version.md` and updates
  `.kb/maps/open-questions-index.md`. Third in this slice's merge order.
- `headline-rules-and-mutant-registry` (HS-S0105) — writes the first three of the
  rules this ADR's range schedules, including
  `compensation_is_atomic_with_the_losing_event`.
- `message-set-on-the-envelope` (HS-S0106) — adds the message types and **only** the
  derives this atom authorises by name, and settles `FORMAT_VERSION` in code the way
  this atom settles it on paper.
- `hub-and-spoke-and-peer-to-peer-topologies` (HS-S0110) — exercises whichever
  topology shape this ADR chose, with one adapter type in both roles on different
  edges.
- `clause-arithmetic-and-deferral-renewals` (HS-S0113) — adds this ADR's range to
  ADR-0026's at project exit and compares against the project's stated range. It reads
  the ranges, not the citations.

## Anchors (progressive disclosure)

Layer two. Each is linked, never pasted; open it at the moment named, not before.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/adr-0026-peer-ingest-and-transport/spec.md` | The predecessor's spec states the range ADR-0027 must complement and the central question it must not re-answer. Its landed range, not its planned one, is the input. | First — before computing this ADR's range at all. | AC-008, AC-007 |
| `spec/SPECIFICATION.md` (SY-2 `:5871-5900`, SY-3 `:5901-5930`, SY-7 `:6033-6052`) | The compensation contract's division of labour and the two alternatives the clauses themselves name as losers. The ADR reproduces the division, not a paraphrase. | Before drafting the compensation section of the long form. | AC-003 |
| `spec/SPECIFICATION.md` (SY-9 `:6090-6106`, SY-10 `:6110-6133`) | SY-9 is frozen and forbids `Peer::new(is_hub: bool)` and `HubPeer`/`SpokePeer` outright; SY-10 carries the falsification test for the only half that is open. | Before answering the one-abstraction-or-two question. | AC-004 |
| `spec/SPECIFICATION.md` (SY-35 `:6868-6900`) | The clause DR-5 rests on: everything replication reasons about is in `EventType`/`Tags`, never reachable only from `Event::data`/`metadata`. Restated as this ADR's own constraint on the suite it schedules. | While writing the constraint on the eight scheduled rules. | AC-008, NF-006 |
| `crates/happenstance-sync/src/lib.rs` (`:78-94`, `:100-104`) | States, in the crate's own words, why the derives were withdrawn — *a `#[derive]` on a public message type **is** a wire format* — and why `Envelope`'s `Deserialize` is hand-written. This is the authorisation's premise. | Before writing the derive-authorisation paragraph. | AC-005 |
| `crates/happenstance-sync/src/peer.rs` (`:43-50`, `:95-118`, `:185-196`, `:236-243`) | The phase-2 sketch's findings: the port cannot express "one round trip with no held state", an opaque `Resume` cannot distinguish *not yet received* from *filtered out*, and the group decomposition is explicit on the wire for a reason. These get deleted when the crate is implemented. | While drafting the long form's evidence section — before implementation deletes the prose. | AC-007, NF-007 |
| `crates/happenstance-sync/src/wire.rs` (`:8-16`, `:46-65`) | Why `Envelope<T>` is generic rather than an enum of message kinds (the version check must run before any decode, WF-8) and what `FORMAT_VERSION` bumps on (shape, never a capacity bound, WF-9). | Before choosing the `FORMAT_VERSION` disposition. | AC-006, AC-005 |
| `crates/happenstance-sync/src/identity.rs` (`:171-180`) | `ReplicatedEvent`'s definition — the third of the three types whose derives are authorised by name. | With the other two, in the same pass. | AC-005 |
| `crates/happenstance-sync/tests/cursor_shape_probe.rs` | The compiled proof that `impl Stream` admits the one-shot-HTTP peer only by buffering and replaying — legal, `Send`, and a lie. Evidence a prose ADR cannot reconstruct. | While drafting the replication-scope section. | NF-007, AC-007 |
| `.kb/open-questions/sync-message-set-and-format-version.md` | Carries the three ordered sub-questions that are this ADR's checklist, and the refutation condition in its own summary. Answering sub-question 2 alone leaves HS-S0100 with nothing to record. | Immediately before and immediately after writing the `FORMAT_VERSION` section. | AC-006 |
| `references/adr/0016-the-wire-format.md` (`:1-25` shape, `:6-11` handoff) | The house shape for a long-form record, and the record that explicitly hands the message set and SY-30 to ADR-0027. Build on the envelope, not around it. | Before opening a blank file for the long form. | AC-002, AC-005 |
| `.kb/decisions/0016-the-wire-format.md` (`:1-11`) | The frontmatter shape to author against — the closest sibling atom, and the only safe source for which fields the corpus actually uses. | While writing the atom's frontmatter. | AC-001, NF-002 |
| `.kb/_templates/atom.md` | The template the ingest path renders against; `KbFrontmatter` is a passthrough, so this plus a sibling is the only guard against an invented key. | With the sibling atom, same pass. | NF-002 |
| `.kb/_intake/README.md` | States that a successful ingest clears the directory — and is itself the one file that must survive it. | Before staging the draft and before running the wave. | AC-001, EC-002 |
| `.kb/decisions/README.md`; `.kb/decisions/0029-msrv-raised-to-1-97-1.md` (`:12-26`) | The immutability discipline, and the worked pattern for correcting an accepted decision by a *new* atom with `depends_on` and no `supersedes`. | If anything in an accepted atom other than `related` looks like it needs to change. | AC-009, EC-009 |
| `.kb/playbooks/one-decision-per-adr-title.md` | The "and" test, applied to this ADR's own title before anyone else's, and the named failure mode: a second ADR restating a settled answer in slightly different words. | While choosing the title, and again when the compensation section starts explaining ingest. | AC-003, AC-008 |
| `.kb/maps/decision-map.md` | The mount point. Its ADR table is the only place the corpus is visible at once; an atom missing from it is unreachable to anyone who does not know the filename. | At the end, when the wave has landed the atom. | AC-009 |
| `references/evaluation/PRESSURE-TEST.md` (`:687-693`) | The named whole-log-versus-scoped experiment the SY-27/SY-28 renewal cites. The file exists, so the citation resolves and CF-38 is satisfied. | While renewing the deferrals — verify the range still frames that question. | AC-007 |
| `references/adr/0007-projection-runner-decodes.md` (`:62-67`) | The precedent for a caller-supplied closure at a port/domain seam: why `pump` takes one rather than the port naming a domain vocabulary. | While writing the SY-3 seam paragraph. | AC-003 |
| `.kb/decisions/0003-opaque-payloads.md` | The guarantee SY-35 and DR-5 protect. Cited, never amended — its `provisional` lift is HS-S0107's, by a new atom. | If the SY-35 argument leans on opaque payloads (and it should). | AC-008, EC-009 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md` (*Tension 3*, *Notes*, testing brief) | The re-derived clause ledger, the SY-14 disagreement, the instruction that this ADR's questions are deliberately left undecided there, and the tier-by-tier mapping the merge gate above is grounded in. | Before the range section, and again before running the gate. | AC-007, AC-008 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/adr-0027-merge-compensation-and-message-set/discover.md` | This story's own signal ledger, five answered questions and three named wrong implementations — including `CompensateAfterCommit`, `FlatteningIngestPeer` and *the mutant that is the ADR*. The spec sharpens it; it does not replace it. | At the start, once, and again when writing the anti-pattern into the record. | AC-003, AC-005 |
| `xtask/src/spec_trace.rs` | What `spec-trace` actually checks — `Rule:` resolution, orphaned rules, and CF-38's named-experiment requirement. Read it before assuming a renewal will pass. | If the deferral renewal is anything other than a bare file-and-line citation. | AC-007, EC-008 |
| `RUNBOOK.md` (`:305-307`, `:462-466`, `:4567-4572`, `:4576-4580`, `:4606`) | The reserved ADR-0027 number, the residual store-side-seam risk the sketch exists to mitigate, the SY-14 work item that disagrees with the ledger, SY-35's rationale, and the rule that no `SY`-constrained `.rs` file merges before this slice. | Before the range section and before claiming the story unblocks the code. | AC-007, AC-008 |

## Clarifications resolved during spec

1. **The AC set is exactly the nine the first pass enumerated.** AC-001 – AC-009, none
   added and none dropped. They partition as: the artefact and its provenance
   (AC-001, AC-002), the four questions the story exists to answer (AC-003 merge and
   compensation, AC-004 topology, AC-005 message set and derives, AC-006
   `FORMAT_VERSION`), and the three obligations the project levies on any ADR in this
   phase (AC-007 deferrals, AC-008 arithmetic, AC-009 mount and no-drift). Project
   AC-001 is carried by AC-001 – AC-006 and AC-009; project AC-010 by AC-007; project
   AC-014 by AC-008.
2. **Six of nine criteria bottom out in a human reading two documents, and that is
   stated in the gate table rather than papered over.** No command can decide whether
   an alternative was named with a reason. The response was to write those criteria as
   *findable facts* — a type name with a derive list beside it, a sub-question with an
   answer, nine clause ids each with a disposition — so a reviewer can check them by
   grep-then-read rather than by judgement.
3. **The interaction-quality section carries no rendered invariant, by sign-off rather
   than by omission.** `_design.md` records `N/A — no user-facing surface` throughout
   and was approved on 2026-08-12 with no conditions and no mock owed. Rather than
   leave the section empty, the STATE and COMPOSITION families are translated into the
   medium this story does have — reachability, two-layer disclosure, density, hierarchy,
   reversibility, one named anti-pattern — and **every one is bound to an existing AC or
   NF row**, because an invariant with no table row is never gated.
4. **The SY-14 boundary is not settled by this spec, and deliberately.** Two repository
   documents disagree (`RUNBOOK.md:4567-4572` versus `_decomposition.md` *Tension 3*),
   and either boundary is defensible. What AC-007 requires is that it is settled **once,
   with ADR-0026, and written into both records** — a spec that picked one here would be
   settling an ADR's question in a planning artefact, which is the failure mode
   `.kb/playbooks/one-decision-per-adr-title.md` names.
5. **The deferral citation is `references/evaluation/PRESSURE-TEST.md:687-693`.** The
   story's discovery cites `:685-693` and the front half cites `:688-693` in one place
   and `:687-693` in another; the whole-log-versus-scoped question begins at `:687`.
   The line range in the ADR should be verified against the file at the merging commit
   rather than copied from any of the three.
6. **`cargo xtask affected --base main` is expected to resolve to nothing.** This is
   recorded as a positive signal rather than a skipped step: a documentation-and-KB diff
   that makes the affected gate rebuild a package has left its PR boundary, and that is
   the cheapest possible detector for it.
7. **EC-007 exists because "authorised by name" can outrun what is knowable.** If a
   serialisation derive genuinely cannot be chosen before the encoder is, the ADR says
   so and scopes the authorisation, naming who owes the rest. A qualified deferral still
   satisfies AC-005's intent; an unqualified silence does not, because silence is what
   lets HS-S0106 decide in a commit.
