---
item: HS-S0057
stage: spec
created: 2026-08-12T13:46:55.942Z
updated: 2026-08-12T13:46:55.942Z
template_sig: 87bbf1d0
rendered_sig: e97df7d5
---

# Spec — CF-14 and CF-27 re-read on this runtime, and the ES-32 verdict on disk

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project item | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/project.md` |
| This spec | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/deferral-re-reads-and-es-32-verdict/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_decomposition.md` (architecture Notes §6 at `:460-461`, §7.5 at `:478-480`; testing brief's AC-009/AC-010 rows at `:524-525`) |
| Intake of record | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_intake-brief.md` (**Clauses**, `:79-89`) |
| Story map row | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_storymap.md:56` |
| Signed-off design | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_design.md` — **no user-facing surface**, approved as such on 2026-08-12 (`:106-111`). This story renders none and adds none. |
| Roadmap pointer | `RUNBOOK.md` Phase 9 (`:4241-4307`); the clause ledgers at `:532-600`; the Decision ledger at `:417-512` |

Project ACs this story traces to: **AC-009** and **AC-010** — sole owner of both
(`.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_storymap.md:120-121`).

## One-line PR slice

Write the per-clause statement of whether CF-14's and CF-27's deferrals still hold
on this runtime (handing CF-27's completeness half onward rather than answering it),
and the one-paragraph ES-32 tail-seam verdict in `RUNBOOK.md`'s ledger — recorded,
not acted on — with `cargo xtask spec-trace` still green.

## Executive summary

This PR lands **three verdicts and no behaviour**. It is the story where the
Cloudflare project stops producing evidence and starts *disposing* of it: two
`[DEFERRED]` clauses get re-read against a runtime that has, for the first time in
this workspace, actually executed the conformance suite, and one `[PROVISIONAL]`
clause gets the verdict its own falsifier names.

The delta against what is already on disk:

- `RUNBOOK.md:559` (CF-14) and `RUNBOOK.md:560` (CF-27) each carry an owning phase
  and no phase-9 reading. After this PR each carries one, written from this
  runtime's observed behaviour rather than from the clause text.
- `RUNBOOK.md:597` names ES-32's falsifier as "a Durable Object making one cheap
  enough to reopen" and its owner as "9 (verdict), post-0.1". The verdict does not
  exist. After this PR it does, as one paragraph, and `RUNBOOK.md:498`'s Decision
  ledger row — which already promises that "phase 9 records whether a Durable
  Object's storage API makes a tail seam cheap enough to reopen post-0.1" — points
  at it.
- `RUNBOOK.md:4300`'s exit criterion "*A one-paragraph verdict on the tail seam, in
  this file's ledger*" is unticked. After this PR it is ticked because the paragraph
  is there, not because someone decided it was true.

What this PR deliberately does **not** land: any change to `spec/SPECIFICATION.md`,
any `.kb/` atom, any port method, any adapter code. Three of the four are other
stories' (named below); the fourth — a tail or subscription seam — is out of this
initiative entirely (`project.md`, **Out of scope**).

## Context pack

The decisions this story must honour. Internalise these before writing a word of
the ledger; everything deeper is a signposted anchor.

**1. A verdict is a disposal, not an opinion. It is written from an artefact that
exists.** The whole reason this project is worth doing is stated in the intake
brief's proof artefact (`_intake-brief.md:69-76`): a build of the crate proves
nothing, because a `todo!()` body type-checks against any signature. The same
standard binds this story one level up. A CF-14 re-read that could have been
written the day the clause was authored is not a re-read — it is the clause,
restated. Every sentence this PR adds must be traceable to something the
`durable-object-conformance-run` milestone produced: the `workerd` run's per-rule
output, the fixture's declared `Capability` constants and their reasons, or a
documented property of the real `SqlStorage` bindings that `worker-binding-layer`
put in the tree. If a verdict cannot name its artefact, it is not ready to be
written.

**2. CF-14's re-read asks one narrow question, and it is not "is this store
durable".** The clause defers *the experiment*, and the experiment is stated
verbatim: whether a `reopen` capability "can be honoured by rusqlite, a Durable
Object and a one-shot HTTP client with one shape, or whether 'durable' needs to be
graded" (`spec/SPECIFICATION.md:7471-7480`). The rule itself already landed early
as a named exception, against `DurableFixture` in the testkit's own `tests/`. So
this story reads exactly one thing: **did the real Durable Object fixture honour
`REOPEN` through the same contract shape the memory and durable fixtures use, or
did it need a graded notion of durable?** CF-17 already predicted the answer's
shape — "the Durable Object is why the weaker half is the one named: its storage
outlives the isolate, so it can discard handle state and read the store again, and
its isolate cannot be restarted from inside a test at all"
(`spec/SPECIFICATION.md:7565-7576`). Confirming a prediction *with a run* is the
deliverable; the prediction on its own is what is already on disk.

**3. CF-14's far end stays phase 8's, and saying so is part of the verdict.** What
remains deferred after this re-read is "a store that can lose an acknowledged write
to a *fault* rather than to an instruction" (`RUNBOOK.md:486`, `:559`). A Durable
Object that honours `REOPEN` does not supply that far end; `sqlite-durable-store`
(HS-P0012) does. The re-read therefore ends in *deferral confirmed, with the
reading that narrows it*, unless the run says otherwise — and if it does say
otherwise, see decision 7.

**4. CF-27 is handed on, by name, and not answered.** The completeness instrument
is "a testkit-adjacent store that deliberately holds only a suffix of its own log,
and reports that it does" (`spec/SPECIFICATION.md:8034-8045`), owned by phase 14 and
by `retention-and-incomplete-logs` (HS-P0018). This project's own boundary says so
twice — `project.md`, **Out of scope** ("CF-27's completeness half and what a store
may forget → `retention-and-incomplete-logs`") and `_intake-brief.md:83-85`. The
re-read that *is* owed here is the narrower one: does a Durable Object — whose
storage a caller can delete wholesale and whose object can be evicted — make the
suffix-store hazard more or less real than the portfolio's "**No, and nothing is
planned**" row claims (`spec/SPECIFICATION.md:8096`)? Answer that, then name HS-P0018
as the owner of the instrument. Naming the recipient is what makes this a handoff
rather than a shrug; an unowned observation is the failure mode the deferred-clause
table exists to prevent ("No deferred clause is unowned", `RUNBOOK.md:562-563`).

**5. ES-32 is recorded and not acted on, and "not acted on" is enforced by the
diff.** The clause forbids `EventStore` growing a tail, subscribe or notify method
at 0.1 and gives the argument in full, including the asymmetry that decides it:
"adding the capability later is additive, while removing it later is not"
(`spec/SPECIFICATION.md:4021-4052`). A Durable Object is one of the three named
shapes that cannot hold a subscriber open, *and* it is one of the two named stores
that genuinely can push — via an alarm. That is precisely why the falsifier row
(`RUNBOOK.md:597`) points at this runtime. The verdict is one paragraph answering
whether a DO's storage API makes a tail seam cheap enough to reopen post-0.1, and
it lands in the ledger. Acting on it — a method, a `TailingEventStore` sub-trait, a
feature flag — is post-0.1 and outside this initiative (`project.md`, **Out of
scope**; `RUNBOOK.md:498`'s ADR column reads "— (post-0.1)").

**6. This story writes prose into `RUNBOOK.md` and nothing else. It does not touch
`spec/SPECIFICATION.md`.** Two independent reasons, and both bite. First,
`spec/SPECIFICATION.md` §1.3's census (`:170-200`) is the one count in the document
a human computed by reading it, and `xtask/src/spec_trace.rs:39-56` explains at
length why it is *checked* and must never be generated — so a maturity marker moved
here shifts §7.1/§7.2 (regenerated) and §1.3 (hand-authored) together and turns a
prose verdict into a spec-surgery PR. Second, and prior to that: changing a clause
is a decision record's job, not a line edit (`CLAUDE.md`, **Open questions**;
`_intake-brief.md:88-89`). This PR's boundary excludes `spec/` for exactly that
reason, and `cargo xtask spec-trace` is run as *proof that nothing rotted*, not as
a thing to satisfy by editing.

**7. If a re-read fires a falsifier, escalate — do not settle it here.** CF-14's
marker says "Falsified by the first adapter that cannot express it". If the Durable
Object fixture could not express `REOPEN` in one shape, or if the ES-32 reading
concludes the port should move before 0.1, that is a finding, and it belongs to
`adr-0023-and-atom-resolutions` (the project's sole owner of every `.kb/` write,
`_storymap.md:94-96`) and to the runbook's ADR queue — never to a side-effect atom
written from this story. This is the repository's standing rule that an ADR is
never authored as a side effect: record the gap, hand it to the pass that owns it.

**8. Who observes this, and where.** There is no screen; the observer is the
**gate reader** of the story map's backbone activity D — *"know what this runtime
cannot do, and why"* (`_storymap.md:29`). Their journey ends at a table row they can
read without running anything: they open `RUNBOOK.md`, find CF-14 and CF-27 in the
ten-row deferred table, and learn what phase 9 found rather than what phase 3
predicted. The verdicts must be legible to that reader in place, without opening the
specification — which means each one states its own conclusion in its first clause,
then its evidence.

## Integration contract

- **Archetype**: `capability` — a slice observable end to end by the gate reader
  persona, in the artefact they actually read.
- **Slice / milestone**: `evidence-and-verdicts`. Slice-mate:
  `wf-11-memory-ceiling-falsifier` (unordered with respect to this story);
  `adr-0023-and-atom-resolutions` closes the milestone after both
  (`_storymap.md:55-57`, **Merge order** §4 at `:158-161`).
- **Mount point**: **`RUNBOOK.md`** — the plan of record, and the composition root
  for every verdict this repository holds. Four real regions, all of which exist
  today and none of which is created by this story:
  - `RUNBOOK.md:559` — the CF-14 row of `### The 10 [DEFERRED] clauses`.
  - `RUNBOOK.md:560` — the CF-27 row of the same table.
  - `RUNBOOK.md:597` — the ES-32 row of `### The 49 [PROVISIONAL] clauses`, whose
    Falsified-by cell already names this runtime.
  - `RUNBOOK.md:4270-4307` — Phase 9's **Work** item for the tail seam (`:4273-4275`),
    its **Exit criteria** tick (`:4300`) and its **Session log** (`:4307`), which is
    where the one-paragraph verdict is written out in full.

  Secondary, and consistent rather than duplicative: `RUNBOOK.md:486` (the
  durability Decision-ledger row) and `RUNBOOK.md:498` (the tail-seam Decision-ledger
  row) are updated to point at the readings above. A verdict in one table and a
  stale promise in another is the rot this repository already burned a whole
  revision on (`RUNBOOK.md:16`).
- **Wires into**: the artefacts of the milestone that merged before this one — the
  `workerd` conformance run's per-rule output from `every-rule-under-workerd`
  (`.bklg/from-contract-to-published-library/cloudflare-durable-object-store/every-rule-under-workerd/spec.md`
  and its `_ledger.md`), the `REOPEN` / `MID_BATCH_FAULT` decisions and measured
  limits from `measured-store-limits`
  (`.bklg/from-contract-to-published-library/cloudflare-durable-object-store/measured-store-limits/spec.md`),
  the fixture's `Capability` constants and the `SKIP <rule>: <reason>` emission at
  `crates/happenstance-testkit/src/contract.rs:473-483`, and the four documented
  `SqlStorage` properties at `crates/happenstance-cloudflare/src/sql_storage.rs:1-24`
  as the real bindings left them.
- **Renders surfaces**: **none.** `_design.md` records no user-facing surface for
  this project and that determination is itself what was signed off
  (`_design.md:12-18`, `:106-111`). This story adds no public item, no signature and
  no doctest, so the design's `## Items` and `## Signatures` blocks stay `N/A` and
  this story claims nothing from them.
- **Conformance rule(s)**: **none, and that is the correct answer here.** No rule in
  `crates/happenstance-testkit/src/suite.rs` observes a ledger paragraph, and adding
  one would be decorative by `CLAUDE.md`'s own test — no adapter could fail it. The
  rules that produced this story's *evidence* (`acknowledged_writes_survive_a_reopen`
  for CF-14; the whole event-store family's skip lines for the capability readings)
  are `every-rule-under-workerd`'s and `measured-store-limits`' to run, and this
  story reads their output rather than adding to it.
- **Clause(s)**: **CF-14**, **CF-27** (`[DEFERRED]`, re-read and disposed of in the
  ledger, markers untouched) and **ES-32** (`[PROVISIONAL]`, verdict written against
  its own named falsifier, marker untouched). No clause is amended; no `[FROZEN]`
  clause is approached. If a reading demands an amendment, decision 7 applies.
- **Advances DoD scenario**: **DoD 12** — *"The clause ledger is audited at publish…
  no clause is provisional with an empty falsifier"*
  (`.bklg/from-contract-to-published-library/initiative.md:393-396`). ES-32's
  falsifier is not empty; it is aimed squarely at this runtime, and until phase 9
  fires it the publish-time audit has a row it can only re-describe. This story is
  also the point at which **DoD 15** (*"Incomplete logs have an answer on disk"*,
  `initiative.md:400-402`) acquires a named owner rather than an assumed one — it
  hands the completeness half to HS-P0018 in writing.

This story is delivered **mounted**: the verdicts are in the file the phase's exit
criteria are read against, not in a story-local note. A verdict nobody reads on
their way to the next phase has not been recorded.

## PR boundary

```
RUNBOOK.md
.bklg/from-contract-to-published-library/cloudflare-durable-object-store/deferral-re-reads-and-es-32-verdict/**
```

**In this PR**

- The CF-14 re-read, written into `RUNBOOK.md:559` and reconciled with `:486`.
- The CF-27 re-read plus the named handoff of the completeness half to
  `retention-and-incomplete-logs` (HS-P0018), written into `RUNBOOK.md:560`.
- The one-paragraph ES-32 tail-seam verdict, in Phase 9's ledger
  (`RUNBOOK.md:4270-4307`), with `:597` and `:498` pointing at it and `:4300`'s exit
  box ticked.
- This story's own `_ledger.md` and, if the implementer wants working notes, files
  under this story's directory.

**Explicitly not in this PR**

- `spec/SPECIFICATION.md` — no clause text, no maturity marker, no census figure.
  Decision 6 above says why, twice.
- `.kb/**` — no atom, no ADR, no open-question resolution.
  `adr-0023-and-atom-resolutions` owns every KB write in this project
  (`_storymap.md:94-96`), and ADR authorship is never a side effect.
- Any port, adapter or testkit code. In particular: no tail, `subscribe` or `notify`
  method, no `TailingEventStore`, no alarm plumbing — ES-32's verdict is recorded,
  not acted on.
- CF-14's far end (a store losing an acknowledged write to a fault) →
  `sqlite-durable-store` (HS-P0012). CF-27's completeness instrument →
  `retention-and-incomplete-logs` (HS-P0018). Both are handed on here, by name.
- Re-running or repairing the `workerd` execution itself → `every-rule-under-workerd`;
  the measured limits and the `REOPEN` decision → `measured-store-limits`. This story
  consumes their output and does not relitigate it.

**Merge DoD (one line)**: the two deferred-clause rows and the ES-32 paragraph are on
disk, each naming the artefact it was read from; `cargo xtask spec-trace` and
`cargo xtask affected --base main` are green; and `git diff --stat` shows nothing
outside the boundary block above.

## Behavior and interfaces

There is no interface here in the type-system sense — the "interface" is what a gate
reader can read, and where. Each row is a contract on the recorded text.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **CF-14's deferral is re-read on this runtime** | A statement whose first clause is the verdict — *the deferral still holds here* or *it does not* — followed by the one narrow finding it rests on: whether the real Durable Object fixture honoured `REOPEN` through the same contract shape as the memory and durable fixtures, or whether "durable" needed grading. It names the fixture, the rule and the run it was read from. | `spec/SPECIFICATION.md:7471-7487` (the clause and its deferred experiment); `:7565-7590` (CF-17's prediction about this exact runtime); `RUNBOOK.md:559`; `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/measured-store-limits/spec.md` |
| **CF-14's far end is stated as still open, and owned** | The re-read explicitly does *not* close the deferral: nothing in this project loses an acknowledged write to a fault rather than to an instruction, so the far end stays phase 8's / `sqlite-durable-store`'s. Written as a sentence, not left as an absence. | `RUNBOOK.md:486`, `:559`; `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/project.md` (**Out of scope**, durability far end); `.bklg/from-contract-to-published-library/sqlite-durable-store/project.md` |
| **CF-27's deferral is re-read on this runtime** | A statement of whether a Durable Object makes the suffix-store hazard more or less real than the portfolio's "**No, and nothing is planned**" row records — read against what this runtime's storage actually permits (wholesale deletion, eviction) as the real bindings landed it, not against the clause text. | `spec/SPECIFICATION.md:8034-8060`; `:8096` (the completeness row of the portfolio table); `RUNBOOK.md:560`; `crates/happenstance-cloudflare/src/sql_storage.rs:1-24` |
| **CF-27's completeness half is handed on by name** | The instrument — a testkit-adjacent suffix store that reports what it holds — is named as `retention-and-incomplete-logs` (HS-P0018) / phase 14's, in the row itself. This story answers the re-read and refuses the instrument; the refusal is written, so the clause stays owned. | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md`; `_intake-brief.md:83-85`; `RUNBOOK.md:560-563` |
| **The ES-32 tail-seam verdict exists as one paragraph** | Answers the falsifier as posed: does a Durable Object's storage API — and its alarm — make a tail or subscription seam cheap enough to reopen post-0.1? It engages the clause's own asymmetry argument (additive to add, breaking to remove) and the three shapes that cannot hold a subscriber open, and it reaches a verdict either way. Length is a budget, not a target: one paragraph, in the ledger. | `spec/SPECIFICATION.md:4021-4052`; `RUNBOOK.md:597` (the falsifier row naming this runtime), `:498` (the Decision-ledger row promising phase 9 records it), `:4273-4275` (the Work item), `:4307` (Session log) |
| **Recorded, not acted on** | The verdict changes no port, adds no method, adds no feature flag and re-opens no clause, whatever it concludes. Enforced by the PR boundary block above rather than by intent: no path under `crates/` or `spec/` is writable in this story. | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/project.md` (**Out of scope**: "Reopening the tail / subscription seam (ES-32) → post-0.1"); `RUNBOOK.md:498` (ADR column "— (post-0.1)") |
| **Every verdict names the artefact it was read from** | The anti-vacuity contract. A sentence that could have been written before the `workerd` run existed fails this story. Each of the three verdicts cites at least one thing this project produced: a run output line, a fixture constant with its stated reason, or a documented property of the real bindings. | `crates/happenstance-testkit/src/contract.rs:473-483` (the `SKIP <rule>: <reason>` emission); `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/every-rule-under-workerd/_ledger.md`; `_intake-brief.md:69-76` (the project's own standard for what proof means) |
| **No clause marker moves, and the census is untouched** | `[DEFERRED]` stays on CF-14 and CF-27; `[PROVISIONAL]` stays on ES-32. The deferred table's own invariant — ten live rows, equal to what `spec-trace` counts — is preserved, so §1.3's hand-authored census and §7.1/§7.2's generated tables all still agree after the diff. | `spec/SPECIFICATION.md:170-200` (§1.3, and why it is hand-computed); `xtask/src/spec_trace.rs:39-56`; `RUNBOOK.md:562-563` |
| **A fired falsifier is escalated, not settled** | If the re-read concludes a deferral has been falsified, or that ES-32 should move before 0.1, the finding is recorded in this story's ledger and handed to `adr-0023-and-atom-resolutions` and the runbook's ADR queue. No atom, no ADR and no clause edit originates here. | `_storymap.md:94-96` (that story owns every KB write); `RUNBOOK.md:302` (ADR-0023's queue row); `CLAUDE.md` (**Where the work lives** — atoms are authored by `/redkiln:kb-ingest`, never by hand) |
| **The gate that observes this story** | `cargo xtask spec-trace` proves the citations still resolve and no marker rotted; `cargo xtask affected --base main` is the story grain. Neither compiles anything this story wrote, because this story wrote no code — which is exactly why the reviewed reading of the diff is a first-class criterion rather than a footnote. | `.redkiln/config.yaml:40`, `:48`; `_decomposition.md:524-525` (the testing brief's own tiering of AC-009 as spec-traced documentation and AC-010 as review-gate) |

## Data and migrations

**N/A.** This story writes prose into two Markdown ledgers and reads artefacts that
already exist. There is no schema, no stored data, no wire format and no persisted
state in its diff.

Two adjacent things are deliberately *not* this story's, and are named so the
implementer does not reach for them: the Durable Object's SQL schema and its
migration 1 — the identity columns `origin_store` / `origin_position` and the time
column every store's migration needs — belong to `durable-object-write-path`
(`_storymap.md:49`), and any wire-format change is
`replication-identity-and-ingest`'s (`project.md`, **Out of scope**). A verdict that
finds itself proposing a column or a byte has left its own story.

## Acceptance criteria

Ten criteria, each framed from the goal of a person who actually reads this
repository's ledgers. The personas are this initiative's own — the **gate reader**
of backbone activity D (`_storymap.md:24-30`), the **adapter author** on *Learn when
you are finished*, the **constrained-runtime developer** on *Event-source at the edge
without hand-rolling it*, and the **evaluator** on *Decide in one sitting*
(`.bklg/from-contract-to-published-library/initiative.md:241-250`) — plus the two
downstream implementers this story hands work to. There is no screen to cross; the
stack this crosses is *clause → run → ledger row → the next person's input*.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** a gate reader on backbone activity D — *know what this runtime cannot do, and why* (`_storymap.md:29`) — who knows the workspace bought a `REOPEN` capability and a `DurableFixture` at phase 3 before any real store could reopen anything, **WHEN** they open `RUNBOOK.md`'s ten-row `[DEFERRED]` table and read the CF-14 row **without opening `spec/SPECIFICATION.md`**, **THEN** the cell's *first clause is the verdict for this runtime* — the deferral still holds here, or it does not — and what follows is the one narrow finding it rests on: whether `CloudflareFixture` honoured `REOPEN` through the same contract shape `MemoryFixture` and `DurableFixture` use, or whether "durable" needed grading — naming the fixture, the rule `acknowledged_writes_survive_a_reopen`, and the `workerd` run it was read from | review of `RUNBOOK.md:559` against the deferred experiment as the clause states it (`spec/SPECIFICATION.md:7471-7480`) and CF-17's prediction about this exact runtime (`:7565-7576`); `cargo xtask spec-trace` green; the cited run line traceable to `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/every-rule-under-workerd/_ledger.md`'s evidence and to `measured-store-limits`' recorded `REOPEN` decision |
| AC-002 | **GIVEN** the implementer of `sqlite-durable-store` (HS-P0012), who inherits CF-14's far end and must not discover from an absence that it was left to them, **WHEN** they read the same row and the durability Decision-ledger row above it, **THEN** the re-read says *in a sentence* that this project supplied no store that can lose an acknowledged write to a **fault** rather than to an instruction, names phase 8 / HS-P0012 as the owner of that far end, and `RUNBOOK.md:486` is reconciled to point at the phase-9 reading instead of stopping at phase 3's — so the two rows agree rather than one going stale | review of the `:559` and `:486` diff together; `git diff -- spec/` empty, so CF-14's `[DEFERRED]` marker is untouched; the owner named matches `.bklg/from-contract-to-published-library/sqlite-durable-store/project.md` and `RUNBOOK.md:605` (CF-17's falsifier, phase 8) |
| AC-003 | **GIVEN** an evaluator on *Decide in one sitting* who wants to know what this library's stores may quietly forget before adopting it, **WHEN** they read the CF-27 row, **THEN** it states whether a Durable Object makes the suffix-store hazard **more or less real** than the portfolio's `**No, and nothing is planned**` completeness row records (`spec/SPECIFICATION.md:8096`), read against what this runtime actually permits — an object's storage deletable wholesale, an object evictable — as the real bindings and their four modelled properties leave it, and **not** against the clause text | review of `RUNBOOK.md:560` against `spec/SPECIFICATION.md:8034-8045` and `:8096`, and against `crates/happenstance-cloudflare/src/sql_storage.rs:1-24` as the real bindings landed it; `cargo xtask spec-trace` green |
| AC-004 | **GIVEN** the implementer of `retention-and-incomplete-logs` (HS-P0018), who owns the completeness instrument and whose project is the only place initiative DoD 15 can close (`initiative.md:400-402`), **WHEN** they open the CF-27 row looking for their input, **THEN** the row names HS-P0018 / phase 14 as the owner of *a testkit-adjacent store that holds only a suffix of its own log and reports that it does*, and states in the same breath that this story deliberately refuses to build it — so the clause stays owned and `RUNBOOK.md:562-563`'s standing claim "No deferred clause is unowned" survives the diff rather than being weakened by an ownerless observation | review of `RUNBOOK.md:560` against `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md` and `_intake-brief.md:83-85`; the ten-row deferred table still lists CF-27 with a named owning phase |
| AC-005 | **GIVEN** a gate reader arriving at Phase 9's exit criteria to decide whether the phase is finished, **WHEN** they read `RUNBOOK.md:4300` and follow it into the phase's Session log, **THEN** one paragraph answers the falsifier exactly as `RUNBOOK.md:597` poses it — does a Durable Object's storage API, including its alarm, make a tail or subscription seam cheap enough to reopen post-0.1 — engaging ES-32's own asymmetry (*adding the capability later is additive, while removing it later is not*) and the three shapes that cannot hold a subscriber open, reaching a verdict either way, and stating which falsifier it answers so the clause's **other**, benchmark-shaped falsifier (the projection-runner measurement, `spec/SPECIFICATION.md:4026-4030`, owned with phase 7 at `RUNBOOK.md:498`) is visibly *not* claimed as discharged | review of the paragraph at `RUNBOOK.md:4307` against `spec/SPECIFICATION.md:4021-4052`; `RUNBOOK.md:597` and `:498` point at it; `:4300`'s box is ticked and the box is ticked *because the paragraph is on disk* — a reviewer can find the paragraph from the tick in one hop |
| AC-006 | **GIVEN** a reviewer whose job is to confirm that a *recorded* verdict did not quietly become an *acted-on* one — the exact failure ES-32 forbids at 0.1 — **WHEN** they read `git diff --stat`, **THEN** the diff touches only `RUNBOOK.md` and this story's own directory: no `crates/**`, no `spec/**`, no `.kb/**`; no tail, `subscribe` or `notify` method, no `TailingEventStore` sub-trait, no alarm plumbing, no Cargo feature; and the PR is revertible on its own, because nothing outside prose depends on anything it wrote | `git diff --stat` against the **PR boundary** block above; `cargo xtask ci --fast` green (the project grain, `.redkiln/config.yaml`'s `integration_scoped`); `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/project.md`, **Out of scope** ("Reopening the tail / subscription seam (ES-32) → post-0.1") read against the diff |
| AC-007 | **GIVEN** an adapter author on *Learn when you are finished*, who has been told by this project's own intake that a build proves nothing because a `todo!()` body type-checks against any signature (`_intake-brief.md:69-76`), **WHEN** they read all three verdicts, **THEN** each one cites at least one artefact this project produced and **could not have been written before that artefact existed** — a per-rule line from the `workerd` run, a `CloudflareFixture` `Capability` constant with its stated reason as the run emitted it, or a documented property of the real `SqlStorage` bindings — and none of the three is the clause restated in other words | for each of the three verdicts, the cited artefact resolved in `every-rule-under-workerd/_ledger.md` or `measured-store-limits/spec.md`; `crates/happenstance-testkit/src/contract.rs:473-483` (`RuleOutcome::Skipped` carries `capability` and `reason`) as the shape of the skip evidence; a **negative control** recorded in the implementation report — the reviewer tries to write each sentence from the clause text alone and records that it cannot be done |
| AC-008 | **GIVEN** a gate reader who knows §1.3's census is the one count in the specification a human computed by reading it and that `spec-trace` therefore *checks* it rather than generating it, **WHEN** this PR merges, **THEN** CF-14 and CF-27 are still `[DEFERRED]`, ES-32 is still `[PROVISIONAL]`, the deferred table still carries thirteen rows of which three are struck and **ten live** — struck rows preserved for a reader arriving from an older commit — and the checker still counts and names the same ten | `cargo xtask spec-trace` green; `cargo xtask affected --base main` green, which runs `retired_rules`, the five file-reading lints and `spec-trace` unconditionally even though `RUNBOOK.md` selects no package (`xtask/src/affected.rs:119-125`, `:248-266`); `git diff -- spec/SPECIFICATION.md` empty; `RUNBOOK.md:562-563`'s equality statement re-read against the checker's output |
| AC-009 | **GIVEN** the implementer of `adr-0023-and-atom-resolutions` (HS-S0058), who merges last in this milestone and owns **every** `.kb/` write in this project, **WHEN** a re-read fires a falsifier — `CloudflareFixture` could not express `REOPEN` in one shape, or the ES-32 reading concludes the port should move before 0.1 — **THEN** the finding is written as a named open item in this story's `_ledger.md` evidence *and* in the RUNBOOK row that carries the verdict, handed by name to HS-S0058 and to the runbook's ADR queue (`RUNBOOK.md:302`), and **no** atom, ADR or clause edit originates in this PR; and if no falsifier fires, the ledger says so explicitly rather than leaving the question blank | `git diff --stat` shows no path under `.kb/`; `redkiln validate --kb && redkiln doctor` clean; the handoff sentence reviewed against `_storymap.md:94-102` (that story owns every KB write) and `CLAUDE.md`, **Where the work lives** (atoms are authored by `/redkiln:kb-ingest`, never by hand) |
| AC-010 | **GIVEN** the gate reader of backbone activity D again, now mid-scroll and unwilling to open a second file, **WHEN** they reach the deferred table, **THEN** each verdict is where they already are — a cell in an existing table row and a paragraph inside Phase 9's existing Session log, with **no new top-level section**, no new table and no bare "see phase 9" pointer that costs a jump without carrying a conclusion; each row cell stays within the one-to-three-sentence budget its ten neighbours keep (`RUNBOOK.md:548-560`); the long form exists **once**, in the session log, and the other four sites point at it rather than copying it; and `CF-14`, `CF-27` and `ES-32` are each greppable in plain text from the row that carries their verdict | review of the rendered `RUNBOOK.md` diff against its neighbouring rows; `rg -n "CF-14\|CF-27\|ES-32" RUNBOOK.md` walked to show exactly one authoritative statement per clause and pointers (not copies) at `:486`, `:498`, `:597`; sentence count per new cell compared to `:549-557`; the testing brief's own tiering — AC-009 static/spec-traced, AC-010 review-gate — applied (`_decomposition.md:524-525`) |

## Interaction quality

RFC §6.7/D6. Every invariant below is carried by an `AC-###` **row in the table
above**; this section says only which id carries which, and how it is checked. No
invariant lives here as a bullet, because a bullet gets no ledger row and is never
gated.

**A note on the composition family, and why it is not skipped.** The project's
signed-off design records **no user-facing surface**, and that determination is
itself what a human approved (`_design.md:12-18`, `:106-111`). So the composition
invariants do not attach to a rendered control — but they are not vacuous either,
because this story *does* have a presentation medium with a signed-off shape: the
existing ledger tables and phase sections of `RUNBOOK.md`. The composition family is
therefore read against **that** document's established composition, and the
anti-patterns are the ones this repository has already paid for.

**State invariants**

| Invariant | Carried by | How it is checked |
| --- | --- | --- |
| **In place, not a context jump** — the verdict is legible where the reader already is; no row defers its conclusion to another file | AC-001, AC-003, AC-010 | AC-001 requires the CF-14 verdict readable "without opening `spec/SPECIFICATION.md`"; AC-010 forbids a bare pointer that carries no conclusion |
| **Non-occlusion** — nothing existing is displaced, hidden or renumbered to make room | AC-008, AC-010 | AC-008 preserves the thirteen-row/ten-live table including the struck rows a reader from an older commit needs; AC-010 forbids a new section or table |
| **Preserved landmarks** (the document analogue of focus/scroll/selection) — row order, clause ids, markers and the census all still resolve after the diff, so a reader's existing bookmarks and the checker's citations both survive | AC-008 | `cargo xtask spec-trace` and `cargo xtask affected --base main`, both of which read files rather than compiling packages |
| **Reversibility** — the change is a self-contained prose commit that can be reverted whole, because nothing outside prose depends on it | AC-006 | `git diff --stat` confined to `RUNBOOK.md` + this story's directory; no port method, sub-trait, feature or alarm plumbing |
| **Reachability without special tooling** (the keyboard-reachability analogue) — every verdict is findable by `rg` on its clause id, in plain text, by someone with no build toolchain | AC-010 | the `rg -n "CF-14\|CF-27\|ES-32" RUNBOOK.md` walk |

**Composition invariants** (against `RUNBOOK.md`'s existing composition)

| Invariant | Carried by | How it is checked |
| --- | --- | --- |
| **Presentation exists at all** — a verdict is composed *into* the ledger's own row-and-phase structure, not dropped in as a free-floating note or a story-local file | AC-005, AC-010 | AC-005 puts the paragraph in Phase 9's Session log under its own exit criterion; AC-010 forbids a new top-level section |
| **Placement** — the deferred re-reads land in the `[DEFERRED]` table rows that already exist for them; the ES-32 verdict lands in the phase that owns it; the two Decision-ledger rows are reconciled rather than duplicated | AC-002, AC-004, AC-005, AC-010 | diff review across `:486`, `:498`, `:559`, `:560`, `:597`, `:4300`, `:4307` |
| **Transience** — persistent chrome is the one-to-three-sentence table cell a scrolling reader always sees; the full argument is *opened on demand* in the session log; nothing is duplicated between the two | AC-005, AC-010 | AC-010's "the long form exists once, and the other four sites point at it" |
| **Density budget, with its real numbers** — the ES-32 verdict is **one paragraph** (`RUNBOOK.md:4300`'s own words); each new table cell is **one to three sentences**, matching the ten neighbours at `:548-560`; **one** authoritative statement per clause across five sites | AC-005, AC-010 | sentence count against neighbouring rows; the `rg` walk showing one statement and four pointers |
| **Hierarchy** — conclusion first, evidence second, in every one of the three verdicts; a reader who stops after the first clause has still learned the answer | AC-001, AC-003, AC-005 | AC-001 makes "first clause is the verdict" the criterion, not a preference; AC-003 and AC-005 require a stated verdict either way |
| **Anti-pattern: two tables that drift** — a verdict in one ledger and a stale promise in another is the rot this file already burned a revision on (`RUNBOOK.md:16`) | AC-002, AC-005, AC-010 | `:486` and `:498` reconciled in the same diff that writes `:559` and `:4307` |
| **Anti-pattern: the unowned observation** — a re-read that notices something and hands it to nobody | AC-002, AC-004, AC-009 | every open residue names a project (HS-P0012, HS-P0018) or a story (HS-S0058) by id |
| **Anti-pattern: the clause restated as a verdict** — prose that could have been written the day the clause was authored | AC-007 | the negative control: try to write each sentence from the clause alone, and record that you cannot |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | The `workerd` run's per-rule output is unavailable, thin, or `every-rule-under-workerd` merged without capturing it | **Halt and escalate; do not write from the clause text.** The dependency exists precisely so these verdicts have artefacts (`_intake-brief.md:69-76`). Writing three plausible paragraphs from the specification is the failure mode this story is defined against, and it would pass every automated check in the gate |
| EC-002 | `CloudflareFixture` **declined** `REOPEN`, so the run reports a skip rather than a pass | This is still a verdict, not an absence. Record the fixture's stated reason **verbatim** as `RuleOutcome::Skipped` carried it (`crates/happenstance-testkit/src/contract.rs:473-483`) and read CF-14 against it: a runtime that declines the capability is evidence about grading, which is exactly what the deferred experiment asks. AC-001 is satisfied by a well-evidenced "the deferral still holds, and here is the sharper reason" |
| EC-003 | A re-read fires a falsifier — the capability could not be expressed in one shape, or ES-32 looks like it should move before 0.1 | AC-009's path: record it in this story's `_ledger.md` and in the row, hand it to `adr-0023-and-atom-resolutions` (HS-S0058) and to `RUNBOOK.md:302`'s ADR queue, and stop. Do not amend a clause, do not write an atom, and do not open a port question inside a prose PR |
| EC-004 | `cargo xtask spec-trace` fails after the edit | The failure is in a citation this PR did not intend to move. **Fix the citation, never the marker.** If the only way to make the checker green is to move a maturity marker or a census figure, the change has left this story's boundary and EC-003 applies |
| EC-005 | `RUNBOOK.md`'s line numbers have drifted because the unordered slice-mate `wf-11-memory-ceiling-falsifier` merged first | Re-locate every mount point by **heading and clause id**, never by the line numbers quoted in this spec. The line numbers here are navigational aids captured at planning time; the clause id is the identity |
| EC-006 | The ES-32 reading turns out to need a measurement nobody took (for example the real cost of an alarm-driven wake-up) | Write the verdict **bounded by what was observed**, naming the measurement that would sharpen it and whose it is. A bounded verdict with a named gap is a verdict; silence, or an unbounded claim, is not. Do not take the measurement here — that is post-0.1 work outside this initiative (`project.md`, **Out of scope**) |
| EC-007 | The CF-27 re-read starts answering the completeness question rather than handing it on | Stop at the boundary the project states twice (`project.md`, **Out of scope**; `_intake-brief.md:83-85`). The re-read says whether the hazard is more or less real *here*; the instrument that would settle it is HS-P0018's, and AC-004 is what checks that the line was held |

## Non-functional

| id | requirement | why, and how it is observed |
| --- | --- | --- |
| NF-001 | **Legibility budget.** One paragraph for ES-32; one to three sentences per new table cell; one authoritative statement per clause across the five sites that mention it | The exit criterion says "a one-paragraph verdict" (`RUNBOOK.md:4300`) and the table's ten neighbours set the cell budget (`:548-560`). Observed by AC-005 and AC-010 |
| NF-002 | **Zero build-time and runtime cost.** No package is added to the compile set, because `RUNBOOK.md` is inert to package selection (`xtask/src/affected.rs:262`), and no crate's public surface, feature set or dependency graph changes | Keeps this story off the critical path of the wasm32 gate its slice-mates depend on. Observed by AC-006 and by `cargo xtask ci --fast` |
| NF-003 | **Durability of the record.** Each verdict names the artefact it was read from, so it stays checkable after the artefact moves and does not decay into an assertion when `RUNBOOK.md` is next revised | `RUNBOOK.md:16` is this file's own record of what happened last time a ledger held a promise nobody could re-derive. Observed by AC-007 |
| NF-004 | **Reviewable in one sitting.** The whole diff is prose a reviewer can read end to end without a build, in roughly one screen per verdict | AC-010 is a review-gate criterion by the testing brief's own tiering (`_decomposition.md:525`), and a review gate only works if the artefact is small enough to actually review |

## Implementation notes (non-prescriptive)

Not a prescription — the AC table is the contract. These are the traps this story's
grounding actually surfaced.

- **Read the evidence before opening `RUNBOOK.md`.** The natural order is: the
  `workerd` run's per-rule output and `every-rule-under-workerd`'s `_ledger.md`
  evidence, then `measured-store-limits`' `REOPEN` / `MID_BATCH_FAULT` decisions and
  its measured limits, then the four modelled `SqlStorage` properties as the real
  bindings left them — and only then the two clauses. Reading the clause first is how
  a verdict ends up restating it (AC-007's negative control exists to catch that).
- **Write the two table cells first and the ES-32 paragraph last.** The paragraph is
  the only piece with argumentative shape, and it is easier to keep to one paragraph
  when the two narrower readings have already forced you to be specific about what
  this runtime does.
- **Reconcile `:486` and `:498` in the same commit that writes `:559` and `:4307`.**
  Not as a follow-up. Two tables in one file that disagree is the failure this file
  already names at `:16`.
- **The tick at `:4300` goes last**, after the paragraph is on disk. Ticking a box
  because the work is "done" rather than because the artefact is there is the exact
  inversion the phase's proof-artefact section was rewritten to prevent
  (`RUNBOOK.md:4277-4292`).
- **Expect the CF-14 verdict to be short and the reasoning behind it to be long.**
  CF-17 already predicted this runtime's shape (`spec/SPECIFICATION.md:7565-7576`);
  the deliverable is confirming or refuting it *with a run*, so the sentence that
  survives is small and the reading behind it is not.
- **Line numbers in this spec are aids, not identities.** EC-005 applies from the
  moment a slice-mate merges.
- If the implementer wants working notes — the raw run lines, the reasoning that did
  not fit in a paragraph — they belong in this story's own directory, which is inside
  the PR boundary. They are not a substitute for the mounted verdict (see
  **Integration contract**: a verdict nobody reads on their way to the next phase has
  not been recorded).

## Tests and CI (merge gate)

Grounded in the testing brief's tiering of these two ACs — AC-009 is *static
(documentation, spec-traced)* and AC-010 is *reviewed, no automated tier*
(`_decomposition.md:524-525`) — and in `.redkiln/config.yaml`'s `verify:` wiring.

| tier | command / path | proves |
| --- | --- | --- |
| Static, file-reading | `cargo xtask spec-trace` | AC-008: every citation still resolves, no marker rotted, the checker counts and names the same ten `[DEFERRED]` clauses |
| Story grain (the redkiln seam) | `cargo xtask affected --base main` (`.redkiln/config.yaml`, `affected_gate`) | AC-008, AC-010: runs `retired_rules`, the five file lints and `spec-trace` **unconditionally** — the arm that exists because a prose-only story selects no package (`xtask/src/affected.rs:119-125`, `:248-266`). It is the reason this story is gated at all |
| Project grain (non-terminal) | `cargo xtask ci --fast` (`.redkiln/config.yaml`, `integration_scoped`) | AC-006: nothing this story wrote broke the gate, including the wasm32 steps its slice-mates depend on |
| Boundary check | `git diff --stat` read against the **PR boundary** block | AC-006, AC-009: no `crates/**`, no `spec/**`, no `.kb/**` |
| Process gate | `redkiln validate --kb && redkiln doctor` | AC-009: no atom was minted here, and the KB is clean and unchanged |
| Ledger gate | `redkiln verify --grain story` over `_ledger.md` | every AC: no row may be flipped without cited, non-placeholder evidence |
| Review gate (no command; the tier the brief names) | a human reading the `RUNBOOK.md` diff against this spec | AC-001 – AC-005, AC-007, AC-010: no test asserts prose. This is the tier this story mostly lives in, which is why AC-007's negative control and AC-010's `rg` walk are written as *procedures a reviewer performs*, not as opinions |
| Negative control (recorded in the implementation report) | attempt to write each of the three verdicts from the clause text alone | AC-007: if the sentence can be written without the run, it is not a verdict — it is the clause |
| Evidence trace | each verdict's cited artefact resolved in `every-rule-under-workerd/_ledger.md` or `measured-store-limits/spec.md` | AC-007: the anti-vacuity contract, checked rather than asserted |

Not run here, and deliberately: the full `cargo xtask ci` is the terminal project's
`verify.e2e` grain (`closeout-and-durable-audience`), and no conformance rule is added
— a rule that observes a ledger paragraph could not be failed by any adapter, which is
`CLAUDE.md`'s own test for a decorative rule.

## Risks and coupling (PR-scoped)

- **This story's gate compiles nothing.** `RUNBOOK.md` is inert to package selection
  (`xtask/src/affected.rs:262`), so the only automated instruments are `spec-trace`,
  the five file lints and a human. Three plausible, well-written, entirely vacuous
  paragraphs would pass everything. *Mitigation:* AC-007's negative control and the
  evidence-trace tier are written as blocking criteria, and `_ledger.md` requires a
  cited artefact per row.
- **Upstream evidence quality is a hard dependency, not a soft one.** If
  `every-rule-under-workerd` merges with a thin report, this story has nothing to read
  and EC-001 fires. *Mitigation:* that story's own AC-001 and AC-005 already require
  the per-rule output and the `SKIP` lines pasted into its implementation report; read
  them before starting, and escalate rather than improvise.
- **Line-number drift from the unordered slice-mate.** `wf-11-memory-ceiling-falsifier`
  may merge first and shift every `RUNBOOK.md` offset quoted here. *Mitigation:*
  EC-005 — relocate by heading and clause id.
- **The pull toward fixing the clause.** Both re-reads sit one edit away from a
  maturity marker, and `spec/SPECIFICATION.md` §1.3's census is hand-computed while
  §7.1/§7.2 are generated (`xtask/src/spec_trace.rs:39-56`), so a single marker move
  turns a prose PR into spec surgery — and a clause change is a decision record's job,
  not a line edit. *Mitigation:* `spec/` is outside the PR boundary; EC-003/EC-004.
- **The pull toward answering CF-27.** The re-read and the answer are adjacent
  sentences. *Mitigation:* EC-007 and AC-004's named handoff.
- **Two-ledger drift.** Five sites mention these clauses; writing three of them and
  forgetting `:486`/`:498` leaves the file self-contradicting. *Mitigation:* AC-002,
  AC-005 and AC-010's one-statement-four-pointers rule, all in one commit.
- **Coupling out.** `adr-0023-and-atom-resolutions` (HS-S0058) merges last in this
  milestone and reads this story's escalations as input; if AC-009's finding is left
  in a working note instead of the ledger and the row, that story cannot see it.
- **What this story cannot break.** No public API, no adapter, no conformance rule, no
  wire format, no schema. The blast radius of a wrong verdict is a wrong sentence a
  later pass must supersede — which is why the sentence must name its evidence
  (NF-003).

## Dependencies

**Blocks on**

- `every-rule-under-workerd` — the only source of the artefacts all three verdicts are
  read from: the per-rule `workerd` output, the `SKIP <rule>: fixture declines
  <CAPABILITY> — <reason>` lines, and the honest scoping of what the run did and did
  not cover. Without it, EC-001 fires by construction.

*(Matches this story's `depends_on`: `["every-rule-under-workerd"]`.)*

**Unordered slice-mate** (same `evidence-and-verdicts` milestone, no edge either way)

- `wf-11-memory-ceiling-falsifier` — gathers a different kind of evidence from the same
  run. Either may merge first; see EC-005.

**Unlocks**

- `adr-0023-and-atom-resolutions` — closes the milestone after both evidence stories
  and owns every `.kb/` write in this project (`_storymap.md:57`, `:94-102`). It is the
  recipient of anything AC-009 escalates.
- Downstream of this project, by name rather than by edge: `sqlite-durable-store`
  (HS-P0012) receives CF-14's far end, and `retention-and-incomplete-logs` (HS-P0018)
  receives CF-27's completeness half — the handoffs AC-002 and AC-004 require in
  writing.

## Anchors (progressive disclosure)

Link, do not paste. Everything load-bearing that is not in the **Context pack** is
here, signposted and bound to the criterion it serves.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `spec/SPECIFICATION.md` (CF-14 at `:7471-7487`; CF-17 at `:7565-7590`) | States the deferred *experiment* verbatim — one shape or a graded "durable" — and CF-17's prediction about this exact runtime. The re-read answers this question and no other | Before writing the CF-14 cell, and again when checking the verdict answers the stated experiment rather than a broader one | AC-001 |
| `spec/SPECIFICATION.md` (CF-27 at `:8034-8060`; the completeness row at `:8094-8098`) | Defines the completeness instrument this story refuses to build, and gives the portfolio row (`**No, and nothing is planned**`) the re-read is measured against | Before writing the CF-27 cell | AC-003 |
| `spec/SPECIFICATION.md` (ES-32 at `:4021-4052`) | The full argument the verdict must engage: the asymmetry, the three shapes that cannot hold a subscriber, "a DO alarm" named explicitly, the `TailingEventStore` escape hatch, and a *second* falsifier that is benchmark-shaped and is not this story's | Immediately before drafting the paragraph — this is the one anchor that must be read in full | AC-005 |
| `spec/SPECIFICATION.md` §1.3 (`:170-200`) with `xtask/src/spec_trace.rs:39-56` | Why the census is hand-computed and checked rather than generated, and therefore why moving a marker here is spec surgery rather than a line edit | The moment editing `spec/` starts to look tempting | AC-008 |
| `RUNBOOK.md` `:486`, `:498`, `:548-563`, `:588-597`, `:4241-4307` | The mount point, in full: both Decision-ledger rows, the ten-row deferred table with its neighbours' density, the falsifier group row naming this runtime, and Phase 9's Work item, Exit criteria and Session log | Open at the start and keep open — every write lands in one of these regions | AC-001 – AC-010 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/every-rule-under-workerd/spec.md` and `_ledger.md` | The dependency's contract and evidence rows: the per-rule output, the `SKIP` lines with their reasons, and its AC-008's honest scoping of what the run did not cover. This is the primary artefact all three verdicts are read from | First, before `RUNBOOK.md` is opened at all | AC-001, AC-007 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/measured-store-limits/spec.md` | Where `REOPEN` and `MID_BATCH_FAULT` are decided honestly against the real runtime, and the measured limits are recorded. CF-14's verdict turns on what that story concluded | Immediately after the run output, and before the CF-14 cell | AC-001, AC-007 |
| `crates/happenstance-testkit/src/contract.rs` (`:468-483`, `:485-503`) | `RuleOutcome::Skipped` carries `capability` and `reason`, and `skip_line` is the emitted form — the exact shape of the evidence a declined-capability verdict must quote | When EC-002 fires, or when quoting a skip reason verbatim | AC-002, AC-007 |
| `crates/happenstance-cloudflare/src/sql_storage.rs:1-24` | The four load-bearing properties of this storage as modelled and as the real bindings left them — synchronous `exec`, a cursor that is not a snapshot, `!Send`/`!Sync` throughout, single-threaded and re-entrant. CF-27's and ES-32's readings both rest on what this storage actually is | Before the CF-27 cell and before the ES-32 paragraph | AC-003, AC-005 |
| `xtask/src/affected.rs:119-125`, `:248-266` | Why a prose-only story is gated at all: `RUNBOOK.md` is inert to package selection, yet `affected` runs the file lints and `spec-trace` unconditionally. This is the whole automated half of this story's merge gate | When deciding what "green" means for this PR | AC-008 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/project.md` (**Out of scope**) | The four exclusions this story is held to, each naming its recipient: the tail seam post-0.1, CF-14's far end, CF-27's completeness half, and every `[FROZEN]` amendment | Before writing, and again before opening the PR | AC-006 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md` | The named recipient of CF-27's completeness half, so the handoff is written to a real owner rather than to a phase number | While writing the CF-27 cell's second half | AC-004 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/project.md` | The named owner of CF-14's far end — a store that can lose an acknowledged write to a fault | While writing the CF-14 cell's second half | AC-002 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_storymap.md` (`:24-30`, `:55-57`, `:94-102`) | The gate-reader persona this story serves (backbone D), the milestone's ordering, and the rule that `adr-0023-and-atom-resolutions` owns **every** `.kb/` write | Before starting, and whenever an escalation looks like it wants to be an atom | AC-009, AC-010 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_intake-brief.md` (`:69-89`) | The project's own standard for what counts as proof, and the clause boundary — including the two-line statement that CF-27's completeness half is not answered here | Before writing any verdict sentence | AC-004, AC-007 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_decomposition.md` (`:514-533`, and the merge-gate notes at `:649-670`) | The testing brief's own tiering of AC-009 and AC-010, and the merge-gate commands for a non-terminal project | When assembling the verification column and the CI table | AC-008, AC-010 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_design.md` (`:12-18`, `:106-111`) | The signed-off no-surface determination — what makes the composition family a document-composition question rather than a skipped section | When reading **Interaction quality** | AC-010 |
| `.bklg/from-contract-to-published-library/initiative.md` (`:241-250`, `:391-402`) | The four personas and journeys the ACs are framed from, and DoD 12 and DoD 15 — the two Definition-of-Done scenarios this story advances | When checking an AC is a user goal rather than a capability | AC-004, AC-005 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | The accepted atom behind the fixture contract and the rule that *a capability an adapter declines is skipped with its stated reason rather than vanishing* — the mechanism EC-002's evidence comes out of | When EC-002 fires | AC-002, AC-007 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/adr-0023-and-atom-resolutions/spec.md` | The escalation recipient's contract — what it expects to inherit from the evidence stories | When AC-009's path is taken | AC-009 |

## Clarifications resolved during spec

1. **The AC set is exactly the front half's ten.** AC-001 through AC-010 as decided in
   the scope-lock pass; none added, none dropped. The ten map onto the front half's
   **Behavior and interfaces** rows one for one, with that table's "gate that observes
   this story" row absorbed into AC-008's and AC-010's verification columns rather than
   becoming an eleventh criterion — a gate is how a criterion is checked, not a
   criterion.
2. **ES-32 has two falsifiers, and this story answers one of them.** The clause's own
   `[PROVISIONAL]` marker names a *benchmark-shaped* falsifier — the projection-runner
   measurement for E2E-32's fan-out (`spec/SPECIFICATION.md:4026-4030`), which
   `RUNBOOK.md:498` places with phase 7's runner — while `RUNBOOK.md:597`'s group row
   names *"a Durable Object making one cheap enough to reopen"* and owns it at phase 9.
   This story answers the second and must say so, so that the first is visibly still
   open. AC-005 carries that obligation. Resolving the discrepancy between the two
   falsifier statements is **not** this story's; it is a clause question, and EC-003
   routes it.
3. **The composition family is not "N/A".** `_design.md` records no user-facing surface
   (approved as such), which would ordinarily retire the composition invariants. It does
   not here, because this story has a real presentation medium with an established
   composition — `RUNBOOK.md`'s ledger tables and phase sections — and the failure modes
   are concrete: a new section nobody scrolls to, a cell three times its neighbours'
   length, a verdict duplicated into two tables that then drift. They are carried by
   AC-005 and AC-010 rather than dropped.
4. **A declined `REOPEN` is a verdict, not a blocker.** Resolved in EC-002: if the
   fixture declines the capability, the re-read reads *that*, with the stated reason
   quoted verbatim. The story fails only if there is no artefact at all (EC-001).
5. **Line numbers versus identities.** Every `RUNBOOK.md` offset in this spec was
   captured at planning time and the unordered slice-mate may move them. EC-005 makes
   the clause id the identity and the line number an aid; no AC is verified by a line
   number alone.
6. **No conformance rule is added, and that is a decision rather than an omission.**
   Recorded in the **Integration contract** and re-stated in **Tests and CI**: a rule
   observing a ledger paragraph could not be failed by any adapter, which is
   `CLAUDE.md`'s own definition of a decorative rule.
7. **`_ledger.md` is the escalation channel of record.** AC-009 requires a fired
   falsifier to appear in the ledger evidence *and* in the RUNBOOK row, because
   `adr-0023-and-atom-resolutions` reads this story's ledger as input and a working note
   in this directory is not something it is obliged to open.
