---
item: HS-S0098
stage: spec
created: 2026-08-12T13:47:36.518Z
updated: 2026-08-12T13:47:36.518Z
template_sig: 87bbf1d0
rendered_sig: 52bfb7a8
---

# Spec — ADR-0026: what a peer is, what the port may assume, what ingest promises

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project charter | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/project.md` |
| This spec | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/adr-0026-peer-ingest-and-transport/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md` (architecture brief §*The seam, in package terms*, §*Composition root*, Tensions 1–5; testing brief §*The test mix, tier by tier* — Static tier) |
| Signed-off design | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_design.md` — **no surfaces**, approved 2026-08-12 as a no-surface determination. This story renders nothing and re-decides nothing there. |
| Story map row | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_storymap.md`, slice `decisions-of-record`, first row |
| Roadmap pointer | `RUNBOOK.md:305` (ADR queue row), `RUNBOOK.md:4550-4620` (phase 13: work list, proof artefact, exit criteria) |

## One-line PR slice

Re-derive the SY/WF clause ledger from `spec/SPECIFICATION.md` at HEAD, then author ADR-0026 — long
form under `references/adr/`, atom via `.kb/_intake/` + `/redkiln:kb-ingest` — answering what a peer
is, what the port may assume about a transport it cannot see, and what ingest promises; reconciling
the central question against frozen SY-1/SY-6, recording WF-1's interoperability half in its envelope
section, and naming SY-32's handoff to ADR-0028.

## Executive summary

**What this PR lands:** one accepted decision atom, `kb-decision-0026`, reachable from
`.kb/maps/decision-map.md`, backed by a long-form record under `references/adr/`, and preceded by a
re-derived clause ledger that fixes an arithmetic defect the planning artifacts already found.

**Pointer, not restatement.** The project charter states *what* must be true
(`project.md`, AC-001, AC-002, AC-010, AC-011, AC-014); the architecture brief states *where each
piece lands* (`_decomposition.md`, Tensions 1–5). This spec adds the delta neither of them carries:

1. **The ledger is re-derived before the ADR is drafted, not after.** The intake brief's clause
   ranges are wrong and its counts are right; the brief already computed the correction
   (`_decomposition.md`, Tension 3) and this story is the one that has to *use* it — because the
   split it fixes is ADR-0026's own stated range, and AC-014's arithmetic is computed against that
   range at project exit by a story six slices away.
2. **ADR-0026 does not deliberate the central question.** SY-1 and SY-6 are `[FROZEN]` and already
   answer it (`spec/SPECIFICATION.md:5848`, `:5981`). The ADR's work on AC-002 is reconciliation,
   citation, and naming the alternatives that lost — reopening it would be amending a frozen clause,
   which the whole initiative excludes.
3. **This PR authorises repairs it does not perform.** `frozen-clause-repairs` (slice 7) requires
   "each repair authorised by ADR-0026". That authorisation has to be *in this atom* or that story
   has nothing to cite. It is written here as a pre-commitment with the playbook's mechanical test
   named, not as a licence to edit.
4. **This PR moves evidence before a later PR deletes it.** `crates/happenstance-sync/` documents
   what its phase-2 sketch proved and where the type checker declined to choose
   (`crates/happenstance-sync/src/lib.rs:106-133`, `src/peer.rs:43-50`, `src/ingest.rs:38-71`).
   Implementing the crate deletes most of that prose. The findings must land in the ADR that
   consumed them first.

**Delta against `HEAD`:** `.kb/decisions/` holds 0001–0016 and 0029 only; 0026 does not exist and
nothing collides. `.kb/_intake/` holds only `README.md`, so the ingest wave starts clean.

## Context pack

Read this section before opening anything. Everything below is a decision already taken that this
story must honor; the anchors table carries the depth.

### The deliverable is a decision record, and the path it arrives by is not negotiable

`CLAUDE.md` (*Where the work lives*) binds two things at once. **A decision lives in two places on
purpose:** the ~100-line atom under `.kb/decisions/` carrying `KbFrontmatter`, the status and the
supersession graph that `redkiln validate --kb` enforces; and the full record under
`references/adr/` carrying the compiler transcripts, the rejected alternatives and the measurement
tables a summary cannot hold. Both are owed here. **And atoms are authored by `/redkiln:kb-ingest`
from `.kb/_intake/`, never by hand** — hand-writing them "produces the directory layout of the
process without the process", which is why the first attempt was reverted (`0269720`).

So the sequence this story executes is: long form → `.kb/_intake/` document → `/redkiln:kb-ingest`
→ atom + map rows → `redkiln validate --kb && redkiln doctor`. There is no shorter path, and a
green suite is not a substitute for the ingest wave having run.

### The decisions gate the code, and this is the gate

AC-001 and `RUNBOOK.md:4606` require ADR-0026 and ADR-0027 merged **before** the code they
constrain. This story is the first item in the project's merge order; nothing in a `.rs` file that
an SY or WF clause constrains may merge before it (`_storymap.md`, *Merge order* 1). That makes
this story's own diff unusually narrow and its downstream blast radius unusually wide: eleven of
the project's sixteen stories cite this atom.

### The central question is answered, frozen, and this ADR cites rather than decides it

*Does ingest re-check the writer's asserted append conditions?* Reads as open in the intake brief.
It is not.

- **SY-1** `[FROZEN]` — ingest MUST NOT refuse for any reason that is a function of the receiving
  store's state, and MUST NOT evaluate an `AppendCondition`, its own or the origin's, as a
  precondition (`spec/SPECIFICATION.md:5841-5867`).
- **SY-6** `[FROZEN]` — a wire-carried condition is **evidence, not an instruction**, and one
  carrying a position-relative boundary (`after: Some(_)`) MUST be refused as ingest input
  (`spec/SPECIFICATION.md:5973-6029`).
- **SY-2** `[FROZEN]` — what happens instead: the losing event and its compensation go in one
  `append` call (`spec/SPECIFICATION.md:5871-5900`).

ADR-0026's job on project AC-002 is **reconciliation and citation**, and naming the alternatives
that lost — not deliberation (`_decomposition.md`, Tension 2). If drafting genuinely finds SY-1 or
SY-6 wrong, the output is a recorded finding and a re-plan, never a quieter version of the clause.
Note the ownership subtlety that makes the arithmetic work: ADR-0026 **cites** SY-1/SY-2/SY-6
without **claiming** them — SY-1 – SY-7 belong to ADR-0027's stated range (`RUNBOOK.md:306`), and
the union at exit counts each clause exactly once.

### ADR-0026's stated clause range, fixed here

The intake brief's split leaves SY-33, SY-34 and SY-35 assigned to nothing while `RUNBOOK.md:4535`
says phase 13 discharges SY-1 – SY-35. The architecture brief computed the fix and this story
writes it down (`_decomposition.md`, Tension 3):

- **ADR-0026 owns: SY-8 – SY-18, SY-33, SY-34**, plus **WF-1's interoperability half**.
  SY-33 (`spec/SPECIFICATION.md:6808`) and SY-34 (`:6833`) are transport-refusal clauses — peer-shaped,
  therefore ADR-0026's.
- **ADR-0027 takes the complement: SY-1 – SY-7, SY-19 – SY-31, SY-35.**
- **SY-32 is a named handoff to ADR-0028**, subtracted from the union rather than assigned.

Inside ADR-0026's own range, two clauses are `[DEFERRED]` and must be **renewed against a named
experiment** — a renewal with no experiment is a build failure under CF-38: **SY-14**
(`spec/SPECIFICATION.md:6234`, cites `references/evaluation/PRESSURE-TEST.md:685-693`) and **SY-18**
(`:6362`, the Turnstile KV-backed peer shape). One clause in range is `[PROVISIONAL]` — **SY-10**
(`:6110`), where the one-abstraction-or-two question actually lives, and which is ADR-0027's to
settle, not this one's. The exit-wide CF-38 sweep and the final union computation belong to
`clause-arithmetic-and-deferral-renewals`; ADR-0026 supplies its own range's half.

### What "what the port may assume about a transport it cannot see" already means

The phase-2 sketch settled the shape and recorded that **the type checker did not force it** — and
that transcript is the most useful thing the sketch produced (`crates/happenstance-sync/src/lib.rs:125-132`):

- `SyncPeer::pull` returns a **bounded batch and an owned resume token, not a stream**, because a
  stream is a cursor and a peer reached over one-shot HTTP has nothing to hold a cursor in
  (`crates/happenstance-sync/src/peer.rs:30-42`).
- The cursor shape was *attempted* and compiled against both peers: the one-shot HTTP peer satisfies
  `impl Stream` by buffering a whole response into a `Vec` and replaying it — "legal, `Send`, and a
  lie" (`crates/happenstance-sync/src/peer.rs:43-50`), kept compiling as
  `crates/happenstance-sync/tests/cursor_shape_probe.rs`. **ADR-0026 cites that probe for why `pull`
  returns a batch.**
- The consequence the port *cannot* express — one round trip with no held state — has to be checked
  by a fixture peer that counts its own round trips. ADR-0026 records that as a conformance
  obligation it hands to `sync-testkit-crate-and-rule-registry`, not as something the port grows.

### What ingest promises, and the seam it needs

`IngestStore` is separate from `EventStore` on purpose: an append is a decision taken now against a
condition checked now by the store that assigns identity; an ingest is the recording of a decision
somebody else already took (`crates/happenstance-sync/src/ingest.rs:87-93`). The trait seam is
discharged — `impl IngestStore for MemoryEventStore` compiles with `happenstance-core` untouched —
and the **write-path** seam is not: `MemoryEventStore` mints `EventId::new(self.store_id, position)`
for everything it writes, and `restore` builds a *new* store from an owned snapshot while `ingest`
holds `&self`, so a foreign identity "has a place to sit and no door to come in through"
(`crates/happenstance-sync/src/ingest.rs:50-71`).

ADR-0026 is the atom that **authorises the door and states its semver class**: one additive
**inherent** `&self` operation on `MemoryEventStore` — option (a) in the architecture brief's table
— semver-minor on a concrete type in a published crate, no trait change, no adapter obliged to have
an opinion. Growing `EventStore` is refused (`RUNBOOK.md:450-455`, SY-8 at
`spec/SPECIFICATION.md:6070-6086`). `memory-store-ingest-seam` implements it and cites this atom for
the authorisation; without the authorisation written here, that story is a core change nobody
decided.

### WF-1's interoperability half lands in the envelope section

`RUNBOOK.md:4593-4595` is specific about *where*: "Record the DCB wire interop decision (WF-1) in
ADR-0026's envelope section: named, deferred, with the experiment being a specific external
implementation to interoperate with. **Not silence.**" WF-1 is `[DEFERRED]` on the strongest
available ground — the DCB specification and its reference TypeScript library publish **no** wire
format at all; `EventStore.ts` contains no serialisation code and the spec's JSON snippets are a
labelled "potential JSON representation" (`spec/SPECIFICATION.md:1892-1922`). That makes the
deferral stronger rather than weaker, and it is the ground
`.kb/open-questions/dcb-reference-publishes-no-wire-format.md` is resolved on by the slice-mate
`open-questions-resolved-and-indexed` — which cites this section rather than re-arguing it.

### SY-32 is a handoff, and the DAG edge is confirmed rather than flipped

`spec/SPECIFICATION.md:7000-7003` states plainly that SY-32 depends on ES-39 and cannot be settled
ahead of it; `RUNBOOK.md:307` assigns ES-39 to ADR-0028 under `retention-and-incomplete-logs`.
`PeerLimits::retention_floor` already exists on the port
(`crates/happenstance-sync/src/peer.rs`) — what is missing is the store-side primitive that lets a
store say what it does not hold. So ADR-0026 **records the dependency by name and cannot discharge
it**, and the `replication → retention` edge in
`.bklg/from-contract-to-published-library/_decomposition.md` (ranks 4 → 5) is already ordered
correctly. **AC-011 is satisfied by writing the confirmation down, not by re-deciding the edge.**
The project's own "three documents, two answers" risk resolves the same way: `RUNBOOK.md:4535`'s
"SY-1 – SY-35" is the phase's *range*, and SY-32 leaves it as an explicit named handoff.

### The repairs this atom must authorise in advance

Two live, and both are *repairs* under the mechanical test in
`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` — **a correction to a `[FROZEN]`
clause is a repair if the set of implementations the clause admits is unchanged; otherwise it is a
gap, and a gap is a decision's**:

1. **The `(new)` markers.** `xtask/src/spec_trace.rs:1626` treats a `Rule:` line containing `(new)`
   as *scheduled*, so the rule is never resolved. SY-1's and SY-2's `Rule:` lines carry it
   (`spec/SPECIFICATION.md:5849`, `:5982`). The moment those rules exist, the markers must go or
   `spec-trace` keeps reporting them unwritten. Implementation set unchanged → repair.
2. **The `Rejects:` symbols.** SY-1, SY-6 and SY-12 all name the **public**
   `guard: Option<AppendCondition>` field on `EventGroup` as a live wrong implementation
   (`spec/SPECIFICATION.md:5860-5862`, `:5998-6001`; `crates/happenstance-sync/src/peer.rs`). If a
   later story makes `guard` private, renames it, or removes it, those clauses lose their named
   wrong implementation in the same commit and a clause whose `Rejects:` names nothing rejects
   nothing.

ADR-0026 states both as authorised repairs, in the playbook's three-part form (**the MUST stays
verbatim**, the discharge is named as a discharge, the code and test that assert it are cited), so
that project DoD 7 — "`git diff` over `spec/SPECIFICATION.md` shows only additions a decision record
authorises" — is satisfied by an authorisation rather than by an argument after the fact.
**This story writes the authorisation; it edits no clause.**

### The persona-journey slice

`_design.md` records no user-facing surface, and the initiative carries `userFacing: false`
(`.bklg/from-contract-to-published-library/initiative.md:411`). The reader this story serves is the
next maintainer opening `.kb/decisions/` to find out what a `SequencePosition` means once it has
crossed a store boundary — and finding an answer with the alternatives that lost named, rather than
a crate whose own module doc calls the question "a position rather than an answer"
(`crates/happenstance-sync/src/lib.rs:116-119`). Initiative **DoD 14** is the observable form of
that: *"Replication has an answer on disk."*

### One title trap

The ADR queue's row is *"What is a sync peer — what may the port assume about a transport it cannot
see, and what does ingest promise?"* — three claims. `.kb/playbooks/one-decision-per-adr-title.md`
records that an "and" in a decision title has three times concealed a second, weaker decision, and
gives the discriminator: **the halves that stood were settled by being made; the halves that fell
were claims about code that did not exist yet.** Two of these three halves are settled by evidence
already in the tree (the cursor probe, the two unlike peer shapes); the ingest promise is settled by
frozen clauses. So the conjunction here is defensible as *one decision with two consequences* — but
the ADR must say so in its own words and mark any half that rests on code not yet written as
provisional with the observation that would refute it. It must not simply inherit the queue's
phrasing unexamined.

## Integration contract

- **Archetype**: `foundation` — real in-tree substrate (an accepted decision atom and its long-form
  record) consumed by eleven capability and foundation stories in this same project. Not a double,
  not a flag, not a `todo!()`.
- **Slice / milestone**: `decisions-of-record`. Slice-mates, implemented in one context and mounted
  as one integrated surface: `adr-0027-merge-compensation-and-message-set`,
  `open-questions-resolved-and-indexed`. Both depend on this story; neither may merge ahead of it.
- **Mount point**: **`.kb/maps/decision-map.md`** — the corpus's decision index and the composition
  root for a decision atom. An atom with no row here is not reachable from the knowledge base's own
  navigation, and the map's own summary names the obligation: *"Updated by the Maps phase of every
  kb-ingest wave that lands a new or superseded decision atom."* The story is delivered mounted when
  `kb-decision-0026` has a row in the ADR table with its status, phase and supersession column
  filled, alongside `.kb/maps/domain-map.md`'s subject grouping.
- **Wires into**:
  - `.kb/_intake/` → `/redkiln:kb-ingest` — the only authoring path for a `.kb/` atom
    (`CLAUDE.md`, *Where the work lives*); the wave also writes
    `.kb/_governance/integration-waves/<wave-id>/`.
  - `references/adr/` — the long-form corpus this atom's `source_paths` and the specification's
    line-range citations resolve into (seventeen records at HEAD, `0001`–`0016`, `0029`).
  - `.kb/decisions/0003-opaque-payloads.md`, `0001-async-port-flavours.md`,
    `0009-error-send-sync.md`, `0013-position-assignment-and-visibility.md`,
    `0016-the-wire-format.md` — cited by `depends_on` / `related`, **never edited**: an accepted
    decision atom is immutable and `redkiln validate --kb` checks each against `HEAD`.
  - `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`,
    `.kb/playbooks/one-decision-per-adr-title.md`,
    `.kb/governance/rewrite-the-referent-never-the-reasoning.md` — the governance primitives this
    atom's form must satisfy.
  - `spec/SPECIFICATION.md` — **read-only in this story.** The clause ledger is re-derived from it;
    no clause is edited here.
- **Public items**: none. `_design.md` declares no surfaces and records the no-surface determination
  as the thing that was approved. This story adds no Rust item, `pub` or otherwise.
- **Renders surfaces**: **none** — `_design.md` `## Surfaces` is `N/A — no user-facing surface`.
- **Conformance rule(s)**: **none, and the reason is structural.** This story changes no port and
  ships no `.rs` file, so it is not adapter-observable. It *names* the rules that will observe the
  decisions it records — `ingest_never_rejects` (SY-1), `wire_condition_with_after_is_refused`
  (SY-6), `compensation_is_atomic_with_the_losing_event` (SY-2) — and hands them to
  `headline-rules-and-mutant-registry`, plus the round-trip-counting fixture obligation
  (`crates/happenstance-sync/src/peer.rs:43-50`) to `sync-testkit-crate-and-rule-registry`. A story
  that changed a port and named no rule would be a port change nothing can fail; this one changes no
  port.
- **Clause(s)**: **discharges none by edit; states ADR-0026's stated range** — SY-8 – SY-18, SY-33,
  SY-34, plus WF-1's interoperability half — records SY-32's handoff to ADR-0028, renews SY-14 and
  SY-18 against named experiments, and **pre-authorises** the SY-1/SY-2 `(new)`-marker repair and the
  SY-1/SY-6/SY-12 `Rejects:`-symbol repairs for `frozen-clause-repairs`. **No `[FROZEN]` clause is
  edited in this story's diff**; changing one would take a new ADR and a re-plan, which the whole
  initiative excludes.
- **Advances DoD scenario**: initiative **DoD 14** — *"Replication has an answer on disk. An accepted
  decision atom answers whether ingest re-checks a writer's asserted conditions — or explicitly
  refuses, with reasons — the corresponding open-question atom reflects that resolution, and
  `redkiln validate --kb` passes"* (`initiative.md:398-401`). This story lands the atom and the
  answer; the open-question half is `open-questions-resolved-and-indexed`'s, in the same slice.

## PR boundary

```
references/adr/0026-*.md
.kb/_intake/**
.kb/decisions/0026-*.md
.kb/maps/decision-map.md
.kb/maps/domain-map.md
.kb/_governance/integration-waves/**
.bklg/from-contract-to-published-library/replication-identity-and-ingest/adr-0026-peer-ingest-and-transport/**
```

**In this PR**

- The clause ledger re-derived from `spec/SPECIFICATION.md` at HEAD — every SY and WF clause's
  maturity marker read from the file, the counts stated, and ADR-0026's range and ADR-0027's
  complement written down so the union at exit is computable rather than asserted.
- The long-form record `references/adr/0026-*.md`, carrying the transcripts, the alternatives that
  lost, and the phase-2 findings migrated out of the sketch's module prose.
- The `.kb/_intake/` document, the `/redkiln:kb-ingest` wave that turns it into
  `.kb/decisions/0026-*.md` with valid `KbFrontmatter`, and the map rows the wave writes.
- The atom's answers: what a peer is; what the port may assume about an unseen transport; what
  ingest promises; the write-path seam authorised with its semver class stated; WF-1's
  interoperability half in the envelope section; SY-14 and SY-18 renewed by name; SY-32's handoff to
  ADR-0028 with the DAG edge confirmed; the repair pre-authorisations.
- This story's own `_ledger.md` and implementation report (second pass and implementation).

**Explicitly not in this PR**

- **Any `.rs` file.** The decisions gate the code; no `.rs` file that an SY or WF clause constrains
  may merge in this story (`_storymap.md`, *Merge order* 1).
- **Any edit to `spec/SPECIFICATION.md`.** The repairs are *authorised* here and *performed* by
  `frozen-clause-repairs` in slice 7.
- **ADR-0027's content** — the merge rule, the compensation contract, replication scope, the
  topology question, the message set and the derives on `PushBatch`/`EventGroup`/`ReplicatedEvent`.
  Slice-mate `adr-0027-merge-compensation-and-message-set` takes the complement of this ADR's clause
  range so the split adds up. Re-deliberating them here is the side-effect authorship this
  repository has already reverted once.
- **Any edit to an accepted decision atom**, including `.kb/decisions/0003-opaque-payloads.md`.
  ADR-0003's `provisional` lift is `adr-0003-provisional-lift`'s, as a **new** atom, and only after
  the round-trip evidence exists.
- **The open-question atom resolutions and `.kb/maps/open-questions-index.md`** — slice-mate
  `open-questions-resolved-and-indexed`'s (AC-013).
- **Claiming `happenstance-sync` / `happenstance-sync-testkit` on crates.io.** The disposition is
  recorded in the architecture brief (AC-015): this project does not claim either name.
- **`cargo xtask ci`** — the whole gate is the terminal project's; `cargo xtask ci --fast` is this
  project's ceiling (`project.md`, DoD 1).

The implementer may also touch the mount-point files named in the Integration contract
(`.kb/maps/decision-map.md`, `.kb/maps/domain-map.md`) to mount this atom — that is the mount, not
scope drift, and in practice the kb-ingest wave writes them.

**Merge DoD (one line):** `redkiln validate --kb && redkiln doctor` are green,
`.kb/decisions/0026-*.md` exists with valid `KbFrontmatter` and a row in `.kb/maps/decision-map.md`,
`references/adr/0026-*.md` carries the long form, `.kb/_intake/` is back to `README.md` only, and
`git diff` shows no `.rs` file and no `spec/SPECIFICATION.md` change.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The ledger is re-derived from HEAD before drafting** | Every `SY-*` and `WF-*` clause's maturity marker is read from the file, not from the intake brief. Expected at HEAD: 21 `[FROZEN]`, 9 `[PROVISIONAL]`, 5 `[DEFERRED]` across SY; the wire clauses are `WF-*`, not `VT-*`. If HEAD disagrees with that count, **HEAD wins** and the ADR records the difference. | `spec/SPECIFICATION.md` (clause headers at `:5841`–`:6868`, `:1892`); `_decomposition.md`, Tension 3 |
| **ADR-0026's stated clause range is written down** | SY-8 – SY-18, SY-33, SY-34, plus WF-1's interoperability half. Stated as a range the exit story can compute against, with the complement named as ADR-0027's (SY-1 – SY-7, SY-19 – SY-31, SY-35) and SY-32 subtracted as a handoff. | `RUNBOOK.md:305-306`, `:4535`; `spec/SPECIFICATION.md:6808` (SY-33), `:6833` (SY-34), `:6868` (SY-35) |
| **Clauses are cited, never claimed twice** | SY-1, SY-2, SY-6 are cited as the frozen answer to the central question and stay in ADR-0027's ownership column. Citation ≠ ownership; the union at exit counts each clause once. | `spec/SPECIFICATION.md:5841-5867`, `:5871-5900`, `:5973-6029`; `project.md` AC-014 |
| **The central question is answered by reconciliation** | *Ingest does not re-check the writer's asserted conditions.* Stated as a reconciliation with SY-1/SY-6 and with the three apparent counter-examples the specification already dismisses (`:5798-5836`), plus the alternatives that lost — chiefly "receiver re-evaluates `origin_condition`", with **both** of SY-6's independent defects reproduced: a position-relative `after` checks an arbitrary tail and passes vacuously; even `after: None` inverts on re-delivery. | `spec/SPECIFICATION.md:5798-5836`, `:5998-6018`; `project.md` AC-002 |
| **What a peer is** | Exactly one peer relationship per `SyncPeer`; fan-out, ordering and merge policy live in a runner above the port; hub-ness is an **edge** property, not a property of the port's type or constructor (SY-9 `[FROZEN]`). "Add a second peer" stays a runner configuration, never a breaking change to the port. | `crates/happenstance-sync/src/peer.rs:59-80`; `spec/SPECIFICATION.md:6070-6086` (SY-8), `:6090-6106` (SY-9) |
| **What the port may assume about an unseen transport** | `pull` returns a bounded batch and an owned resume token, not a stream — one round trip per call by construction. The ADR cites the cursor probe as the reason: `impl Stream` admits the one-shot-HTTP peer *by buffering and replaying*, "legal, `Send`, and a lie". The type checker did not force the choice, and the ADR says so. | `crates/happenstance-sync/src/peer.rs:30-50`; `crates/happenstance-sync/tests/cursor_shape_probe.rs`; `crates/happenstance-sync/src/lib.rs:125-132` |
| **The property the port cannot express is handed onward** | "One round trip with no held state" must be checked by a fixture peer that counts its own round trips. Recorded as a conformance obligation for `sync-testkit-crate-and-rule-registry`, not as a port change. | `crates/happenstance-sync/src/peer.rs:43-50`; `_decomposition.md`, testing brief, *Fixtures and seams* |
| **What ingest promises** | Atomicity per group, identity preserved (the foreign `EventId` survives), idempotence on re-delivery, arrival-order local position at the tail. Ingest is the recording of a decision already taken and already made durable — not an append. | `crates/happenstance-sync/src/ingest.rs:87-93`; `spec/SPECIFICATION.md:5954` (SY-5), `:6135` (SY-11), `:6409` (SY-19) |
| **The write-path seam is authorised, with its semver class stated** | One additive **inherent** `&self` operation on `MemoryEventStore` accepting an already-identified `SequencedEvent`. Semver-minor on a concrete type in a published crate; no trait change; `EventStore`'s signature byte-identical before and after. Growing `EventStore` is refused by name. | `crates/happenstance-sync/src/ingest.rs:50-71`; `RUNBOOK.md:450-455`; `spec/SPECIFICATION.md:6070-6086`; `_decomposition.md`, architecture brief §5 |
| **WF-1's interoperability half, in the envelope section** | Named, deferred, with the experiment being a *specific external implementation to interoperate with*. The ground is that the DCB specification and its reference TypeScript library publish no wire format at all — which makes the deferral stronger, not weaker. Not silence. | `RUNBOOK.md:4593-4595`; `spec/SPECIFICATION.md:1892-1922`; `.kb/open-questions/dcb-reference-publishes-no-wire-format.md` |
| **In-range deferrals renewed by name** | SY-14 against the whole-log-versus-scoped experiment; SY-18 against the Turnstile KV-backed peer shape. A renewal with no named experiment is a build failure under CF-38. | `spec/SPECIFICATION.md:6234` (SY-14), `:6362` (SY-18), `:213-217` (CF-38); `references/evaluation/PRESSURE-TEST.md:685-693` |
| **SY-32 is a handoff, and the DAG edge is confirmed** | The ADR records the ES-39 dependency by name, states it cannot discharge it, names ADR-0028 and `retention-and-incomplete-logs` as the owner, and confirms the `replication → retention` edge (ranks 4 → 5) is already ordered correctly. No edge flip. | `spec/SPECIFICATION.md:7000-7003`; `RUNBOOK.md:307`; `.bklg/from-contract-to-published-library/_decomposition.md`, *Dependency DAG* |
| **Two repairs are pre-authorised, none performed** | (i) dropping `(new)` from SY-1's and SY-2's `Rule:` lines once the rules exist; (ii) repairing any `Rejects:` whose named symbol a later story changes — chiefly `EventGroup::guard`. Each carries the playbook's mechanical justification (implementation set unchanged → repair) and the three-part form: MUST verbatim, discharge named as a discharge, code and test cited. | `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`; `xtask/src/spec_trace.rs:1626`; `spec/SPECIFICATION.md:5849`, `:5982`, `:5860-5862`, `:5998-6001`, `:6153-6183` |
| **The phase-2 findings are carried, not discarded** | Each finding the sketch's module prose holds — the position-is-not-an-identity statement, the cursor-shape transcript, the trait-seam-discharged / write-path-not result, and `real_peer_shapes.rs`'s honest limit on stand-in evidence — is moved into the ADR that consumed it **before** any later story deletes the paragraph holding it. | `crates/happenstance-sync/src/lib.rs:98-133`; `src/peer.rs:43-50`; `src/ingest.rs:22-81`; `crates/happenstance-sync/tests/real_peer_shapes.rs`; `_decomposition.md`, architecture brief *Notes* |
| **The atom's form** | Valid `KbFrontmatter`: `id: kb-decision-0026`, `kind: decision`, `status: accepted`, `authority_tier: decision`, `adr_id: ADR-0026`, `phase: 13`, `supersedes: null`, a prose `summary`, `depends_on` naming the atoms it rests on, `source_paths` naming the long form and the intake document. Authored by the ingest wave, not by hand. | `.kb/decisions/0016-the-wire-format.md:1-40` (shape exemplar); `CLAUDE.md`, *Where the work lives* |
| **The title states one decision** | The queue row joins three claims. The ADR either defends the conjunction as one decision with two consequences, or splits the weaker half out and marks it provisional with the observation that would refute it and the phase that would produce it. It does not inherit the queue's phrasing unexamined. | `.kb/playbooks/one-decision-per-adr-title.md:34-55`; `RUNBOOK.md:305` |
| **Nothing under `.kb/` is hand-authored** | `.kb/_intake/` → `/redkiln:kb-ingest` → atom. `.kb/_intake/` holds only `README.md` at HEAD and is back to that at exit. `redkiln validate --kb && redkiln doctor` green. | `CLAUDE.md`, *Where the work lives*; commit `0269720` (the reverted hand-authoring attempt); `_decomposition.md`, AC-A09 |

## Data and migrations

**N/A — no schema, no store, no migration.** This story's entire diff is markdown: a long-form
decision record under `references/adr/`, a staging document under `.kb/_intake/`, the atom and map
rows the ingest wave writes under `.kb/`, and this story's own backlog folder. No Rust item is
added or changed, no table or column exists to migrate, and no serialised format is touched — the
wire format's own change (the message set on `Envelope<T>`) is ADR-0027's and lands in
`message-set-on-the-envelope`, two slices later.

The one thing that *behaves* like a migration, and is called out so it is not mistaken for one: the
`/redkiln:kb-ingest` wave mutates the knowledge base's index atoms (`.kb/maps/decision-map.md`,
`.kb/maps/domain-map.md`) and appends a wave directory under
`.kb/_governance/integration-waves/`. Those writes belong to the wave, are reversible by reverting
the branch, and are validated by `redkiln validate --kb` rather than by any data check. **An
accepted decision atom is immutable**, so the wave must add `kb-decision-0026` rather than edit any
of the seventeen atoms already at HEAD; an edit to one of those bodies is the failure mode this
story must not produce, and `validate --kb` checks each against `HEAD`.

## Acceptance criteria

The persona is the one `_design.md` and the initiative record: **the next maintainer**, and the eleven
downstream stories that cite this atom rather than re-deriving it. There is no screen; the observable
outcome is what that reader finds on disk when they go looking (`initiative.md:398-401`, DoD 14).
Each criterion is a goal crossing the whole stack of this story — specification at HEAD → long form →
intake document → ingest wave → atom → map row.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | GIVEN the intake brief's clause ledger disagrees with `spec/SPECIFICATION.md` — wrong ranges, and `VT-*` where the file says `WF-*` — WHEN a maintainer opens ADR-0026 to find out which SY and WF clauses phase 13 actually discharges, THEN they find a ledger **re-derived from `spec/SPECIFICATION.md` at HEAD**: every SY and WF clause listed with its maturity marker and its line, the per-marker counts stated, and every disagreement with the brief resolved in the specification's favour and written down as a recorded difference rather than silently absorbed — and the re-derivation is stated as having preceded the drafting, not reconciled after it. | Recount at HEAD: `rg -n "^\*\*(SY\|WF)-[0-9]+" spec/SPECIFICATION.md` and the marker sweep `rg -n "\[FROZEN\]\|\[PROVISIONAL\]\|\[DEFERRED\]" spec/SPECIFICATION.md` reproduce the record's table clause-for-clause and count-for-count; `cargo xtask spec-trace` green (`xtask/src/spec_trace.rs`). |
| AC-002 | GIVEN AC-014's arithmetic is computed six slices later by `clause-arithmetic-and-deferral-renewals`, WHEN that story computes the union of the two ADRs' stated ranges against the project's stated range, THEN ADR-0026 has already written its own range down in a form that can be computed against rather than argued with — **SY-8 – SY-18, SY-33, SY-34 plus WF-1's interoperability half**, with ADR-0027's complement named (SY-1 – SY-7, SY-19 – SY-31, SY-35) and SY-32 subtracted as a named handoff — and clauses it merely cites (SY-1, SY-2, SY-6) are marked as cited, not claimed, so the union counts each clause exactly once. | Arithmetic re-run by hand against the record: stated range ∪ named complement ∪ {SY-32 handoff} = SY-1 – SY-35 with no clause counted twice; `rg -n "SY-33\|SY-34\|SY-32" references/adr/0026-*.md` shows all three dispositions present; cross-check against `RUNBOOK.md:305-306`, `:4535`. |
| AC-003 | GIVEN a maintainer asks the question the whole phase is named for — does ingest re-check the writer's asserted append conditions — WHEN they read ADR-0026, THEN they get the answer (**it does not**) **reconciled against the frozen clauses rather than re-deliberated**: SY-1 and SY-6 cited by line as the binding statements, the three apparent counter-examples the specification already dismisses accounted for, and the alternative that lost — receiver re-evaluates `origin_condition` — refuted with **both** of SY-6's independent defects reproduced, that a position-relative `after` checks an arbitrary tail and passes vacuously, and that even `after: None` inverts on re-delivery. | The record cites `spec/SPECIFICATION.md:5841-5867`, `:5973-6029`, `:5798-5836`; the `re-check\|recheck\|re-evaluat` sweep over `spec/SPECIFICATION.md` is recorded with its result either way; `git diff --name-only main -- spec/SPECIFICATION.md` is empty, proving reconciliation rather than amendment. |
| AC-004 | GIVEN a maintainer wiring a second peer wants to know whether that is a configuration change or a breaking one, WHEN they read the record's answer to *what a peer is*, THEN it states that one `SyncPeer` is exactly one peer relationship, that fan-out, ordering and merge policy live in a runner above the port, and that hub-ness is a property of the **edge** and never of the port's type or constructor (SY-9, `[FROZEN]`) — so adding a peer stays runner configuration — with SY-8's refusal to grow `EventStore` cited as the same decision seen from the other side. | Record cites `crates/happenstance-sync/src/peer.rs:59-80`, `spec/SPECIFICATION.md:6070-6086` (SY-8) and `:6090-6106` (SY-9); `hub-and-spoke-and-peer-to-peer-topologies` can quote it without re-deriving it; `git diff --stat main -- crates/` empty. |
| AC-005 | GIVEN a maintainer asks why `pull` hands back a bounded batch and an owned resume token instead of a stream, WHEN they read the record's answer to *what the port may assume about a transport it cannot see*, THEN they find the reason stated as evidence rather than taste — the cursor shape was attempted, compiled against both peer shapes, and admitted the one-shot-HTTP peer only by buffering a whole response into a `Vec` and replaying it — **the type checker did not force the choice and the record says so** — the still-compiling probe is cited by path, and the property the port cannot express (one round trip holding no state) is handed to `sync-testkit-crate-and-rule-registry` as a fixture-owned round-trip-counter obligation rather than pushed back into the port. | Record cites `crates/happenstance-sync/src/peer.rs:30-50`, `crates/happenstance-sync/tests/cursor_shape_probe.rs` and `crates/happenstance-sync/src/lib.rs:125-132`; the probe still compiles — `cargo test -p happenstance-sync --test cursor_shape_probe`; the handoff names the recipient story slug and `crates/happenstance-sync/tests/real_peer_shapes.rs`'s own stated limit on stand-in evidence. |
| AC-006 | GIVEN a maintainer needs to know what arriving at a receiver guarantees, WHEN they read *what ingest promises*, THEN they find four promises stated and each tied to a clause — atomicity per group, the foreign `EventId` preserved, idempotence on re-delivery, and a local position assigned at the arrival-order tail — together with the reason `IngestStore` is a separate trait at all: an append is a decision taken now against a condition checked now by the store that assigns identity, while an ingest records a decision somebody else already took and already made durable, so a `SequencePosition` is meaningful only inside one store. | Record cites `crates/happenstance-sync/src/ingest.rs:87-93` and `spec/SPECIFICATION.md:5954` (SY-5), `:6135` (SY-11), `:6409` (SY-19); the position-is-not-an-identity finding at `crates/happenstance-sync/src/lib.rs:98-133` appears in the record before any later story can delete the paragraph holding it (`rg -n "position" references/adr/0026-*.md`). |
| AC-007 | GIVEN `memory-store-ingest-seam` cannot land a change to a published crate that nobody decided, WHEN its implementer opens ADR-0026 for the authorisation it is told to cite, THEN the atom authorises exactly one additive **inherent** `&self` operation on `MemoryEventStore` accepting an already-identified `SequencedEvent`, **states its semver class** (minor, on a concrete type in a published crate; no trait change; `EventStore`'s signature byte-identical before and after), and refuses growing `EventStore` by name — with the sketch's own finding carried across, that the trait seam is discharged and the write path is not, so a foreign identity today has a place to sit and no door to come in through. | Record cites `crates/happenstance-sync/src/ingest.rs:50-71`, `RUNBOOK.md:450-455`, `spec/SPECIFICATION.md:6070-6086`; the authorisation is quotable by `memory-store-ingest-seam`'s ledger as a `file:line`; `git diff --name-only main -- crates/happenstance-core` empty in this story's PR. |
| AC-008 | GIVEN `RUNBOOK.md:4593-4595` forbids silence on DCB wire interoperability, WHEN a maintainer reads ADR-0026's **envelope section**, THEN WF-1's interoperability half is there by name — deferred, with the experiment stated as a *specific external implementation to interoperate with*, and with the ground recorded that the DCB specification and its reference TypeScript library publish no wire format at all, which makes the deferral stronger rather than weaker — so `open-questions-resolved-and-indexed` can resolve `dcb-reference-publishes-no-wire-format` by citation instead of re-arguing it. | Record's envelope section cites `spec/SPECIFICATION.md:1892-1922` and names the external implementation; `rg -n "WF-1" references/adr/0026-*.md` non-empty; `cargo xtask spec-trace` green over WF-1's deferral (CF-38, `spec/SPECIFICATION.md:213-217`). |
| AC-009 | GIVEN a `[DEFERRED]` clause with no named experiment is a build failure under CF-38, WHEN `clause-arithmetic-and-deferral-renewals` sweeps at project exit, THEN ADR-0026 has already supplied its own range's half: **SY-14** renewed against the whole-log-versus-scoped experiment and **SY-18** against the Turnstile KV-backed peer shape, each experiment named as an observation someone could actually make, and the exit-wide sweep left to the story that owns it. | `cargo xtask spec-trace` green with CF-38 satisfied for SY-14 (`spec/SPECIFICATION.md:6234`) and SY-18 (`:6362`); the record cites `references/evaluation/PRESSURE-TEST.md:685-693` for SY-14's ground. |
| AC-010 | GIVEN HS-P0018 is unblocked only when SY-32's disposition is on disk, WHEN a maintainer asks whether replication settles retention or waits on it, THEN ADR-0026 records the ES-39 dependency **by name**, states plainly that it cannot discharge it, names ADR-0028 and `retention-and-incomplete-logs` as the owner, notes that `PeerLimits::retention_floor` already exists on the port while the store-side primitive does not, and confirms — rather than flips — the `replication → retention` edge at ranks 4 → 5 of the initiative DAG. | Record cites `spec/SPECIFICATION.md:7000-7003` and `RUNBOOK.md:307`; the confirmation is checkable against `.bklg/from-contract-to-published-library/_decomposition.md`'s *Dependency DAG* with no edit to that file; project DoD 6 quotable from the atom. |
| AC-011 | GIVEN project DoD 7 requires that any change to `spec/SPECIFICATION.md` be one a decision record authorised, WHEN `frozen-clause-repairs` runs in slice 7, THEN it finds both live repairs already authorised in this atom in the playbook's three-part form — the MUST kept verbatim, the discharge named as a discharge, the code and test that assert it cited — namely dropping `(new)` from SY-1's and SY-2's `Rule:` lines once those rules exist, and repairing any `Rejects:` whose named symbol a later story changes, chiefly `EventGroup::guard`; each carries the mechanical justification that the admitted implementation set is unchanged, and **this story performs neither repair**. | The record states both authorisations with `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`'s test applied; `git diff --name-only main -- spec/SPECIFICATION.md '*.rs'` is empty; `cargo xtask spec-trace` still reports SY-1/SY-2's rules as scheduled, proving the `(new)` markers were left in place (`xtask/src/spec_trace.rs:1626`). |
| AC-012 | GIVEN a maintainer opening `.kb/` to find what a `SequencePosition` means once it has crossed a store boundary, WHEN they navigate from `.kb/maps/decision-map.md`, THEN they reach `kb-decision-0026` — an atom **authored by the ingest wave, never by hand** (`.kb/_intake/` → `/redkiln:kb-ingest`), carrying valid `KbFrontmatter` with `adr_id: ADR-0026`, `status: accepted`, `phase: 13`, a prose `summary`, `depends_on` and `source_paths` resolving into `references/adr/0026-*.md` — whose title states **one** decision, with any half resting on code not yet written marked provisional and carrying the observation that would refute it; `.kb/_intake/` is back to `README.md` only and no accepted atom's body was edited. | `redkiln validate --kb && redkiln doctor` green; `rg -n "kb-decision-0026" .kb/maps/decision-map.md .kb/maps/domain-map.md` non-empty; `test -f references/adr/0026-*.md`; `ls .kb/_intake` is `README.md` alone; `git diff --name-only main -- .kb/decisions` lists only the new atom; title checked against `.kb/playbooks/one-decision-per-adr-title.md:34-55`. |

Coverage of the traced project ACs: **AC-001** → AC-012 (and AC-004, AC-005 supply the alternatives
that lost); **AC-002** → AC-003, AC-006, AC-007, AC-011; **AC-010** → AC-008, AC-009; **AC-011** →
AC-010; **AC-014** → AC-001, AC-002.

## Interaction quality

**Composition family: N/A, and the determination is the approved artifact.**
`.bklg/from-contract-to-published-library/replication-identity-and-ingest/_design.md` records
`## Surfaces`, `## Items`, `## Signatures`, `## States` and `## Anti-patterns` all as
*N/A — no user-facing surface*, and the sign-off section states that no mock was produced or owed.
This story renders nothing, adds no Rust item `pub` or otherwise, and therefore has no presentation,
placement, transience, density or hierarchy invariant to carry. It also does not re-decide that:
a story that started rendering something would be contradicting a signed-off design.

**State family: applies in the knowledge base's own medium**, which is the only surface this story
touches. Three of the five state invariants have a real referent here, and each is carried by an
AC row in the table above rather than by a bullet in this section:

| Invariant | Its form here | Carried by | Verified by |
| --- | --- | --- | --- |
| **Reachability** (the keyboard-reachability analogue) | The atom is reachable by *navigation* — a row in `.kb/maps/decision-map.md` and a subject grouping in `.kb/maps/domain-map.md` — not only by guessing a path. An atom with no map row is present and unreachable, which is the KB's version of a control no one can tab to. | **AC-012** | `rg -n "kb-decision-0026" .kb/maps/decision-map.md .kb/maps/domain-map.md`; `redkiln doctor` |
| **In place, not a context jump** | The atom answers the decision *in the atom*: a reader gets what was decided and what lost without having to open `references/adr/0026-*.md`, which carries the evidence a summary cannot hold. The long form is depth, never the answer's only location. | **AC-003**, **AC-004**, **AC-005**, **AC-006** | The atom read standalone answers each of the four questions; the long form is cited for evidence only |
| **Reversibility, and non-destruction** | Nothing in this diff destroys prior state: an accepted atom is immutable and none is edited; the two spec repairs are *authorised and not performed*; the ingest wave's map writes are revertable with the branch. The `reversibility` frontmatter field states the class explicitly. | **AC-011**, **AC-012** | `git diff --name-only main -- .kb/decisions spec/SPECIFICATION.md '*.rs'`; `redkiln validate --kb` (immutability against `HEAD`) |

**Preserved focus/scroll/selection** and **non-occlusion** have no referent: there is no viewport, no
focus, and nothing that could overlay anything. Stating that is the point — an invariant with no
referent is not silently dropped here, it is named as inapplicable so a reviewer can disagree.

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| EC-001 | Drafting finds SY-1 or SY-6 **genuinely wrong** — the reconciliation in AC-003 cannot be made honestly. | **Stop.** Record the finding, raise it as a new decision atom and a re-plan (`project.md`, AC-002). Never write a quieter version of a `[FROZEN]` clause, and never let the ADR's prose diverge from the clause it cites. |
| EC-002 | HEAD's maturity markers or clause numbering disagree with the counts this spec states (21 `[FROZEN]`, 9 `[PROVISIONAL]`, 5 `[DEFERRED]` across SY). | **HEAD wins.** Record the difference in the ledger section with the lines that disagree, and carry the corrected numbers forward into AC-002's range statement. The spec's numbers are orientation; the file is the authority. |
| EC-003 | A `[DEFERRED]` clause in ADR-0026's own range cannot be renewed against an experiment anyone could actually run. | Do **not** renew it with a placeholder — CF-38 exists to make that a build failure. Record the clause, the reason no experiment is nameable, and hand it to `clause-arithmetic-and-deferral-renewals` as a stated blocker rather than a quiet renewal. |
| EC-004 | The `/redkiln:kb-ingest` wave proposes MERGE or AMEND against an accepted decision atom (most plausibly `0003-opaque-payloads.md` or `0016-the-wire-format.md`). | Refuse the op. An accepted decision atom is immutable and `redkiln validate --kb` checks each against `HEAD`; the correct output is a **new** atom that `depends_on` the old one. If the wave has already written such an edit, revert the file and re-run. |
| EC-005 | The wave finishes with `.kb/_intake/` holding more than `README.md`, or `redkiln validate --kb` / `redkiln doctor` red. | The story is not done. `.kb/_intake/` is staging that the wave consumes and clears; a leftover document means the wave did not complete, and a hand-fix under `.kb/` is the exact failure `0269720` was reverted for. |
| EC-006 | The queue row's three-way conjunction cannot be defended as one decision — one half turns out to rest wholly on code that does not exist. | Split it: keep the halves settled by evidence already in the tree, mark the remaining half provisional **with the observation that would refute it and the phase that would produce it**, per `.kb/playbooks/one-decision-per-adr-title.md`. Inheriting the queue's phrasing unexamined is the failure mode. |
| EC-007 | A slice-mate's ingest wave has already written `.kb/maps/decision-map.md` and the rows conflict. | Re-run the map sync rather than hand-merging rows: the map is the wave's output, and two hand-merged rows is two atoms one of which is unreachable. Merge order (`_storymap.md`) puts this story first precisely so the conflict is avoidable. |
| EC-008 | A later story deletes a `crates/happenstance-sync/` module paragraph this ADR cites, before or after this merge. | The citation must have been *carried*, not merely referenced (AC-005, AC-006, AC-007). If a cited paragraph is gone, the record is repaired by quoting the migrated text, per `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` — never by dropping the citation. |

## Non-functional

| id | Requirement | Why, and how it is checked |
| --- | --- | --- |
| NF-001 | **Two documents, two jobs.** The atom stays at roughly 100 lines and carries the decision; the long form under `references/adr/` carries the transcripts, the alternatives that lost and the tables. Neither is a copy of the other. | `CLAUDE.md`, *Where the work lives*: deleting the long form because the atom exists would discard about 78% of the corpus, and `spec-trace` catches it because the specification cites line ranges that only exist in the long form. |
| NF-002 | **Citations are anchored, not floating.** Every `file:line` citation quotes enough surrounding text that drift is detectable when the referent moves. | `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`; this story cites eleven files that later slices actively rewrite. |
| NF-003 | **No accepted atom's body changes.** The corpus grows by addition only. | `redkiln validate --kb` checks each accepted decision atom against `HEAD`. |
| NF-004 | **Diff discipline: markdown only.** No `.rs` file and no `spec/SPECIFICATION.md` line in this PR. | The decisions gate the code (`_storymap.md`, *Merge order* 1); `git diff --name-only main` inspected at review; `cargo xtask affected --base main` shows no affected Rust package. |
| NF-005 | **Blast radius stated, not discovered.** Eleven of the project's sixteen stories cite this atom; the record names which claims are load-bearing for which downstream story. | Prevents the failure where a downstream story cites a paragraph the atom never intended as normative. |
| NF-006 | **Gate cost stays at this project's ceiling.** `cargo xtask ci --fast`, never `cargo xtask ci`. | `project.md`, DoD 1; `.redkiln/config.yaml` reserves the whole gate for the terminal project. |
| NF-007 | **Rewrite the referent, never the reasoning.** Where this record restates an older decision's ground, it re-points the reference and leaves the reasoning intact. | `.kb/governance/rewrite-the-referent-never-the-reasoning.md`. |

## Implementation notes (non-prescriptive)

- **Order that actually works:** re-derive the ledger (AC-001) → settle the range and the complement
  with `adr-0027`'s author in the same context, since they are slice-mates (AC-002) → draft the long
  form, migrating the phase-2 findings as you go → write the `.kb/_intake/` document → run
  `/redkiln:kb-ingest` → `redkiln validate --kb && redkiln doctor`. Drafting before the ledger is the
  one ordering this spec forbids, because the range being drafted *is* what the ledger corrects.
- **The findings migration is distributed on purpose.** The sketch's four findings land in three
  different ACs — the cursor transcript and `real_peer_shapes.rs`'s honest limit in AC-005, the
  position-is-not-an-identity statement in AC-006, the trait-seam-discharged / write-path-not result
  in AC-007. A single "move the prose" step would produce a long form that reads like a copied module
  doc; each finding belongs next to the decision it justifies.
- **Use `.kb/decisions/0016-the-wire-format.md` as the frontmatter exemplar**, not as a length target:
  its `summary` is unusually long because that decision had six measured parts. Read `:1-40` for the
  field set and the prose-summary convention.
- **Two sentences the ADR should not write.** "This is provisional pending implementation" without
  naming the observation that would settle it, and "the type checker forced this shape" where it did
  not — `crates/happenstance-sync/src/lib.rs:125-132` is explicit that it did not, and that admission
  is the most useful thing the sketch produced.
- **The repair authorisations are pre-commitments, not licences.** Write them in the playbook's
  three-part form and say which story performs them. An authorisation that reads like permission to
  edit will be used as one.

## Tests and CI (merge gate)

Grounded in the testing brief's *Static* tier and *Merge-gate commands*
(`_decomposition.md:625-663`, `:775-800`). This story ships no `.rs`, so the Unit and Integration
tiers have nothing to run for it — stated rather than omitted, so the absence is a finding a reviewer
can check instead of an oversight.

| tier | command / path | proves |
| --- | --- | --- |
| Static — KB | `redkiln validate --kb && redkiln doctor` | AC-012: `KbFrontmatter` valid on `.kb/decisions/0026-*.md`, every accepted atom's body unchanged against `HEAD` (NF-003, EC-004), the maps consistent. The merge gate the testing brief names for AC-001. |
| Static — spec | `cargo xtask spec-trace` (`xtask/src/spec_trace.rs`) | AC-001, AC-002, AC-009, AC-011: every `SY`/`WF` `Rule:` still resolves, no clause orphaned, CF-38 satisfied for SY-14 and SY-18, and SY-1/SY-2's `(new)` markers still read as *scheduled* — the positive check that this story authorised the repairs without performing them (`xtask/src/spec_trace.rs:1626`). |
| Static — lints | `cargo xtask lints` | The unconditional story-grain gate `.redkiln/config.yaml` wires to `reachability_static`; runs even for a story whose whole deliverable is markdown. |
| Static — grain | `cargo xtask affected --base main` | NF-004: the diff touches no Rust package, so the affected set is empty. A non-empty set here means a `.rs` file leaked into a decision-only PR. |
| Static — ledger recount | `rg -n "^\*\*(SY\|WF)-[0-9]+" spec/SPECIFICATION.md`, `rg -n "\[FROZEN\]\|\[PROVISIONAL\]\|\[DEFERRED\]" spec/SPECIFICATION.md` | AC-001: the record's re-derived table is reproducible from the file at HEAD, clause-for-clause. Any disagreement resolves to the file (EC-002). |
| Static — diff assertion | `git diff --name-only main -- spec/SPECIFICATION.md '*.rs'` is empty; `git diff --name-only main -- .kb/decisions` lists only the new atom | AC-011, AC-012, NF-003, NF-004; project DoD 7. |
| Static — mount | `rg -n "kb-decision-0026" .kb/maps/decision-map.md .kb/maps/domain-map.md`; `test -f references/adr/0026-*.md`; `ls .kb/_intake` = `README.md` | AC-012 and the reachability invariant: the atom is navigable, the long form exists, staging is clear (EC-005). |
| Static — cited-probe liveness | `cargo test -p happenstance-sync --test cursor_shape_probe` | AC-005: the probe the record cites as its reason for the batch-not-stream shape still compiles at merge, so the citation is live rather than historical. |
| Gate | `cargo xtask ci --fast` | NF-006, project DoD 1: this project's ceiling. For this story it proves the markdown-only diff broke nothing, including the `--no-default-features` doc build and the `cargo package --list` assertion. |
| Not run | `cargo xtask ci` | Reserved for the terminal project (`project.md`, DoD 1). Running it here would not be extra rigour; it would be this project claiming a bar it was explicitly not given. |

## Risks and coupling (PR-scoped)

| Risk | Coupling | Mitigation inside this PR |
| --- | --- | --- |
| **The range is wrong and nothing notices until slice 7.** AC-014's arithmetic is computed by `clause-arithmetic-and-deferral-renewals`, six slices away. | Every later SY-clause claim. | AC-001 re-derives from HEAD and AC-002 writes the range down in computable form, with the complement and the SY-32 subtraction stated — so the exit story computes rather than reconstructs. |
| **The complement drifts.** ADR-0027 is drafted in the same context and takes the complement; a change to either range that is not mirrored breaks the union. | `adr-0027-merge-compensation-and-message-set` (slice-mate). | The complement is stated *inside* ADR-0026 (AC-002), so a drift is a contradiction between two adjacent documents rather than a silent gap. Merge order puts this story first. |
| **The ingest wave touches the shared maps.** Three slice-mates each land an atom and each wave writes `.kb/maps/`. | `open-questions-resolved-and-indexed`, `adr-0027-*`. | EC-007: re-run the map sync rather than hand-merge. Sequential merge order is the primary control. |
| **A cited paragraph is deleted by a later story.** `ingest-store-and-memory-peer-round-trip` replaces the `todo!()` bodies and most of the module prose this record cites. | `crates/happenstance-sync/src/{lib,peer,ingest}.rs`. | AC-005 – AC-007 require the findings *carried into* the record, not merely cited; EC-008 and `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` govern repair. |
| **An authorisation is read as a licence.** The repair pre-authorisations could be taken by a later implementer as permission to edit a frozen clause now. | `frozen-clause-repairs` (slice 7). | The atom names the performing story and applies the playbook's mechanical test to each repair; this story's own diff shows no specification change, which is the strongest available demonstration of the distinction. |
| **The conjunction hides a second decision.** Three claims in one title has concealed a weaker second decision three times in this corpus. | The atom's own durability. | AC-012 requires the title checked against `.kb/playbooks/one-decision-per-adr-title.md:34-55`, with EC-006's split as the recorded escape hatch. |
| **Hand-authoring under `.kb/`.** Faster, and already reverted once (`0269720`). | The whole KB discipline. | AC-012 verifies the wave ran (`.kb/_governance/integration-waves/`, cleared `_intake/`), not merely that an atom exists. |

## Dependencies

**Blocks on:** none. `depends_on: []` — this is the first story in the project's merge order
(`_storymap.md`, *Merge order* 1) and the first item in the `decisions-of-record` slice. It reads
`spec/SPECIFICATION.md`, `RUNBOOK.md` and `crates/happenstance-sync/` as they stand at HEAD and needs
nothing built.

**Unlocks** (by story slug, each edge from `_storymap.md`):

| Story | What it takes from this atom |
| --- | --- |
| `adr-0027-merge-compensation-and-message-set` | The complement of the clause range (AC-002), and the reconciliation it must not repeat (AC-003). |
| `open-questions-resolved-and-indexed` | WF-1's envelope-section ground, so `dcb-reference-publishes-no-wire-format` resolves by citation (AC-008). |
| `memory-store-ingest-seam` | The authorisation for the inherent `&self` operation and its stated semver class (AC-007). |
| `sync-testkit-crate-and-rule-registry` | The round-trip-counter conformance obligation the port cannot express (AC-005). |
| `headline-rules-and-mutant-registry` | The clauses the three headline rules answer to, cited rather than re-read (AC-003, AC-006). |
| `frozen-clause-repairs` | Both repair authorisations in the playbook's three-part form (AC-011). |
| `clause-arithmetic-and-deferral-renewals` | ADR-0026's stated range and its half of the CF-38 renewals (AC-002, AC-009). |
| `retention-and-incomplete-logs` (HS-P0018, next project) | SY-32's named handoff and the confirmed DAG edge (AC-010). |

## Anchors (progressive disclosure)

Deferred depth. Open each at the moment named; do not preload the set.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `spec/SPECIFICATION.md` | The authority for every clause number, marker and citation in this story. The ledger is re-derived *from this file*, and where it disagrees with any brief, it wins. | First, before drafting anything — clause headers `:5798-5836`, `:5841-5867`, `:5871-5900`, `:5973-6029`, `:6070-6106`, `:6234`, `:6362`, `:6808`, `:6833`, `:6868`, `:7000-7003`, `:1892-1922`, CF-38 at `:213-217`. | AC-001, AC-002, AC-003, AC-008, AC-009, AC-010 |
| `RUNBOOK.md` | Phase 13's work list, its stated clause range and the ADR queue rows that assign SY-1 – SY-35 across three ADRs; also the explicit instruction on *where* WF-1 is recorded. | With the specification, at ledger time — `:305-307`, `:450-455`, `:4526-4620` (esp. `:4535`, `:4593-4595`, `:4606`). | AC-001, AC-002, AC-007, AC-008, AC-010 |
| `crates/happenstance-sync/src/lib.rs` | Holds the phase-2 sketch's own account of what it proved and where the type checker declined to choose — including the admission that the port shape was not compiler-forced. Most of this prose is deleted by a later story. | While drafting the transport and ingest sections — `:98-133`, esp. `:116-119`, `:125-132`. | AC-005, AC-006 |
| `crates/happenstance-sync/src/peer.rs` | The port as it stands: `pull`'s batch-and-token signature, the cursor-shape transcript, the one-peer-per-`SyncPeer` shape and `PeerLimits::retention_floor`. Also carries the still-public `guard` field the `Rejects:` lines name. | When writing *what a peer is* and *what the port may assume* — `:30-50`, `:59-80`. | AC-004, AC-005, AC-010, AC-011 |
| `crates/happenstance-sync/tests/cursor_shape_probe.rs` | The compiled evidence behind the batch-not-stream decision: the shape that admits a one-shot-HTTP peer only by buffering and replaying. Cite it by path; run it to prove the citation is live. | When AC-005's reason is written, and again at merge as a liveness check. | AC-005 |
| `crates/happenstance-sync/src/ingest.rs` | States why `IngestStore` is a separate trait, and the exact write-path finding: trait seam discharged, write path not — `restore` builds a new store while `ingest` holds `&self`. | When writing *what ingest promises* and the seam authorisation — `:22-81`, esp. `:50-71`, `:87-93`. | AC-006, AC-007 |
| `crates/happenstance-sync/tests/real_peer_shapes.rs` | Records its own limit on how far stand-in peer shapes can be trusted as evidence — the honesty this record must preserve rather than round up. | Alongside the cursor probe, when deciding how strongly to state the transport claim. | AC-005 |
| `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` | The mechanical test that separates a repair from a gap, and the three-part form each authorisation must take. Without it, AC-011's pre-authorisations are opinions. | Immediately before writing the two repair authorisations. | AC-011 |
| `.kb/playbooks/one-decision-per-adr-title.md` | The discriminator for a conjunctive ADR title — the halves that stood were settled by being made; the halves that fell were claims about code that did not exist. | When naming the record, before the long form is finalised — `:34-55`. | AC-012 |
| `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` | How to cite a moving file so drift is detectable, in a record that cites eleven files later slices rewrite. | While writing citations, not after. | AC-005, AC-006, AC-007 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The binding constraint on restating an older decision's ground: re-point the reference, leave the reasoning. | Whenever this record restates ADR-0003's, ADR-0013's or ADR-0016's ground. | AC-006, AC-008 |
| `.kb/decisions/0016-the-wire-format.md` | The frontmatter and prose-summary exemplar for an atom of this weight, and the accepted decision WF-1's ground sits next to. Immutable — read, never edit. | When writing the `.kb/_intake/` document — `:1-40` for the field set. | AC-008, AC-012 |
| `.kb/decisions/0013-position-assignment-and-visibility.md` | What a `SequencePosition` already means inside one store — the decision this record extends across a boundary rather than contradicts. | While writing AC-006's local-tail promise. | AC-006 |
| `.kb/decisions/0003-opaque-payloads.md` | The payload-opacity constraint this record must not weaken; its `provisional` lift belongs to a different story and must be a new atom. | Before any sentence about payload bytes. | AC-006, AC-012 |
| `.kb/open-questions/dcb-reference-publishes-no-wire-format.md` | The open question WF-1's envelope section supplies the ground for; the slice-mate resolves it by citing this record. | When writing the envelope section. | AC-008 |
| `.kb/maps/decision-map.md` | The mount point. An atom without a row here is unreachable by navigation. | At mount time, and again at verification. | AC-012 |
| `xtask/src/spec_trace.rs` | Shows exactly what makes a `Rule:` line read as *scheduled*, which is why the `(new)` markers must stay until the rules exist — the mechanism AC-011 is written against. | When authorising the `(new)`-marker repair — `:1626`. | AC-011 |
| `references/evaluation/PRESSURE-TEST.md` | The measurement SY-14's deferral cites; the experiment named in the renewal has to be consistent with it. | When renewing SY-14 — `:685-693`. | AC-009 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md` | The architecture brief's Tensions 1–5 (esp. Tension 3's computed ledger correction and Tension 2's already-answered central question) and the testing brief's Static tier. | Before drafting; Tension 3 first. | AC-001, AC-002, AC-003, AC-007 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/project.md` | The project ACs this story traces to, and DoD 7's requirement that every specification change be one a decision record authorised. | At the start, and again when checking AC coverage — `:182-254`, `:255-279`. | AC-002, AC-003, AC-010, AC-011 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_storymap.md` | The merge order this story heads, and the eleven downstream stories whose citations constrain what this record must state explicitly. | When deciding what to hand onward versus what to decide here — `:53-70`, `:114-141`. | AC-005, AC-007, AC-011 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_design.md` | The signed-off design, and the reason the composition family of interaction-quality invariants is inapplicable: it records no user-facing surface, deliberately. | Once, if any doubt arises about whether this story renders anything. | AC-012 |

## Clarifications resolved during spec

1. **The twelve AC ids are exactly the front half's twelve.** None added, none dropped. Two of the
   behaviour-table rows that had no obvious home — the phase-2 findings migration, and the
   one-decision title discipline — were folded into existing criteria rather than given new ids: the
   findings distribute across AC-005 (cursor transcript, stand-in limits), AC-006 (position is not an
   identity) and AC-007 (trait seam discharged, write path not), and the title discipline sits in
   AC-012 with the record's form, where its verification already lives.
2. **The composition family of interaction-quality invariants is inapplicable, and that is recorded
   rather than skipped.** `_design.md` declares no surfaces at every heading and states that no mock
   was owed. The state family is carried in the knowledge base's own medium — reachability by map
   row, the answer available in place, and non-destruction — each bound to an AC row, never to a
   prose bullet, so `redkiln verify` can extract it.
3. **Verification for a markdown-only story is deterministic commands over real paths, not a Rust
   test.** `redkiln validate --kb`, `redkiln doctor`, `cargo xtask spec-trace`, `cargo xtask lints`,
   `cargo xtask affected --base main`, `git diff --name-only`, and named `rg` recounts. The one Rust
   command in the table (`cargo test -p happenstance-sync --test cursor_shape_probe`) is there for a
   different reason: to prove that a citation this record leans on is still live at merge, not to
   test anything this story wrote.
4. **The `(new)` markers staying in place is a positive assertion, not an omission.**
   `cargo xtask spec-trace` continuing to report SY-1's and SY-2's rules as *scheduled*
   (`xtask/src/spec_trace.rs:1626`) is the evidence that this story authorised the repair without
   performing it. A green `spec-trace` that no longer schedules them would mean the repair happened
   here, in the wrong story.
5. **Clause counts are stated as an expectation, not a fact.** The 21/9/5 split appears in the
   behaviour table as what HEAD is expected to say. EC-002 makes the file the authority: if HEAD
   disagrees, HEAD wins, the difference is recorded, and AC-002's range statement carries the
   corrected numbers forward. Nothing downstream depends on the spec's numbers being right — it
   depends on the *record's* numbers being re-derived.
6. **`retention-and-incomplete-logs` appears in the unlocks table although it is a project, not a
   story in this project.** It is listed because AC-010's whole purpose is to unblock it (project
   DoD 6) and a reader of the dependencies section would otherwise not see the one edge that leaves
   this project. It is not a `depends_on` edge in either direction for this story.
