---
item: HS-S0127
stage: spec
created: 2026-08-12T13:48:06.721Z
updated: 2026-08-12T13:48:06.721Z
template_sig: 87bbf1d0
rendered_sig: 857a053e
---

# Spec — DoD 1–12 and 14–15 re-observed as a set on this tree

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative charter (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — the sixteen DoD scenarios verbatim at `:360-407`, under the preamble at `:356-358` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — the traceability matrix; DoD→owning-project rows at `:90-105` |
| Project item | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md` — AC-003 at `:200-203`, DR-2 at `:131-134`, DR-3 at `:135-138`, DR-12 at `:179-183` |
| This spec | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/dod-set-re-observation-record/spec.md` |
| Key brief (the project's only warranted brief) | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md` — the **testing brief**; the AC-003 row at `:46`, the test-mix-by-tier note at `:61-91`, the merge-gate command block at `:93-107` |
| Signed-off design | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_design.md` — **no public API surface**; `## Items` is `N/A`. This story renders no surface and must not introduce one |
| Grounding | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_grounding.md` — the `xtask` gate anatomy at `:54-66`, the `verify:` block reading at `:107-113` |
| Story map | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_storymap.md` — this story's row at `:55`, the `_closeout-record.md` convergence at `:24-30`, merge order at `:144-146` |
| This story's discover stage | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/dod-set-re-observation-record/discover.md` — two questions deferred to *this* spec, decided below; the named wrong implementation at `:38` |
| Roadmap pointer | `RUNBOOK.md` — the plan of record; the ADR queue at `:262-284` is `answers-audit`'s, not this story's |

## One-line PR slice

Re-run or re-inspect DoD 1–12 and 14–15 on this tree and record all fourteen in one table,
each with the command and the artefact path — a sibling's `_ledger.md` may point at what to
re-run, never stand in as the evidence.

## Executive summary

This PR appends **one section — the fourteen-row re-observation table — to the project's single
closeout artefact**, `_closeout-record.md`, and performs the re-runs that populate it. Nothing
else. No production code, no `.kb/` atom, no fix.

The delta this story adds over what `project.md` and the testing brief already say is the part
neither of them could fix: **which command re-proves which scenario, and which scenarios are
honestly re-inspected rather than re-run.** `discover.md` deferred exactly two questions to this
spec — whether the gate run itself re-proves some of the fourteen, and what "re-observed" means
for the four registry-facing scenarios that cannot be re-executed without re-publishing. Both are
**decided here**, per scenario, in the fourteen-row map under *Behavior and interfaces*. The
decision is not a blanket one: DoD 9 and DoD 12 turn out to be genuinely re-runnable on this tree,
DoD 11 turns out to be re-runnable only into the wrong question, and DoD 5/6 depend on an
environment that may not be present — so the record grows a third mode, **conditional re-run**,
which names the absent environment exactly the way `DR-2` makes the gate name the absent tool
behind every `skipped` step.

It also fixes the record's **row shape** — the eight columns every later reader and every later
appending story sees — because that shape is what makes AC-004 mechanically checkable rather than
a matter of taste: pointers and evidence live in *different columns*, so a sibling ledger cannot
be quietly promoted into the evidence slot.

## Context pack

The load-bearing decisions, stated as decisions. Everything deeper is a signposted anchor.

**1. Fourteen, not sixteen — and the record says why.** DoD 13 belongs to the slice-mate
`published-tree-delta-statement` (project AC-004) and DoD 16 to
`product-atom-promotion-via-kb-ingest` (project AC-007…AC-009), per `_storymap.md:131-134`. A
fourteen-row table with no explanation reads as two forgotten scenarios. The record names both
absences and the story that owns each, in the same section.

**2. The ledger is the pointer; it is never the evidence.** This is the whole reason the story
exists. `project.md:202-203`: *"No entry cites only the producing project's ledger; a ledger may
be the pointer, never the evidence."* A sibling's `_ledger.md` proves the scenario passed on the
tree that sibling had **at the time it merged** — which is precisely the assumption this project
exists to stop making (`initiative.md:356-358`: each DoD scenario is *"run and observed to pass on
the assembled library, from a clean checkout"*, and *"a green gate is a precondition for looking at
these, never a substitute for them"*). `discover.md:38` names the wrong implementation in full:
fourteen rows whose artefact column links the producing sibling's ledger entry. It passes a shallow
read of AC-003 and fails DR-3 outright. **The row shape below makes this structurally hard rather
than merely forbidden** — the ledger goes in a *Pointer* column and the evidence goes in an
*Artefact produced here* column, and a reviewer's check is that no cell appears in both.

**3. Re-run is the default. Two named exceptions, each with a stated reason.** DR-3
(`project.md:135-138`) permits *"re-run or re-inspected"* but requires the record to say which. This
spec decides the mode **per scenario**, not by policy:

- **re-run** — a command exists that re-derives the claim on this tree. The default, and the
  majority.
- **conditional re-run** — a command exists but needs an environment this machine may not have (a
  live Postgres for DoD 5, a reachable Neon endpoint for DoD 6). If the environment is present it is
  a re-run; if absent, the row states the **absent environment by name**, mirroring DR-2's rule that
  every gate step printing `skipped` is accounted for by a missing tool probe rather than by a tool
  that ran and was ignored (`project.md:131-134`; the tool-gated step list is at
  `_grounding.md:54-66`).
- **re-inspect** — no command re-derives the claim on this tree, because the claim is documentary
  (a written verdict) or is a property of the *published artefact* rather than of this tree. The row
  states which, and confirms the claim still holds at this SHA.

**4. One SHA, or it is not a set.** The record's header carries the commit SHA the slice-1 gate run
recorded, and every one of the fourteen rows is observed against **that** SHA. Two SHAs means two
trees, and fourteen rows spread across two trees is fourteen separately-remembered ticks in a table
— the failure mode `_storymap.md:24-30` names.

**5. This story does not run the gate; it consumes slice 1's run.** `cargo xtask ci` — the whole
gate, `.redkiln/config.yaml:60`, never the `--fast` bar at `:55` (DR-2) — is
`whole-gate-green-on-the-assembled-tree`'s deliverable, and it is this story's `depends_on`. Several
of the fourteen are re-proved *inside* that run (the tests step, `spec-trace`, `proof-artefact`, the
doc builds). Those rows cite **that** run's output and SHA. Re-running the gate a second time here
would produce a second SHA and violate decision 4.

**6. Nothing is fixed here — a red row is a routed finding.** `project.md:237-240` (AC-013) makes
routing the deliverable and `:179-183` (DR-12) gives the three destinations: `support`
(`.redkiln/config.yaml:5`), a new item against the owning sibling, or a decision atom plus a re-plan
where a `[FROZEN]` clause is touched. This story's job when a scenario does not come back green is
to record the row honestly and hand it to `findings-disposition-register`, which owns the
disposition. A story that repairs what it found has failed its own acceptance criterion
(`_storymap.md:92-95`).

**7. Mount, do not create a rival.** The fourteen rows are a **section inside**
`_closeout-record.md` — the one companion artefact the whole project converges on, stood up by
`clean-checkout-harness` in slice 1. A separate `dod-re-observation.md` beside it would satisfy every
literal word of AC-003 and defeat the word "set".

**8. The reader this is written for.** This project's `_design.md` declares *no public API surface*,
so the surface here is the record's legibility, not a signature. The persona slice is the one
`_storymap.md:16-22` names: the reader who must be able to believe the initiative closed honestly
**without re-deriving the evidence** — the evaluation path that `_decomposition.md:148-181` promotes
to a first-class journey atom of its own. Concretely: a row that a reader cannot follow from command
to artefact without asking someone is a failed row, whatever its outcome column says.

**9. No production code, and no `.kb/` atom.** `project.md:117-120` and `_design.md:10-39`: this
project *"verifies and records; it designs nothing."* This story adds, changes and removes zero
public items, writes no conformance rule, and amends no `SPECIFICATION.md` clause. It also
hand-authors no `.kb/` atom — the constraint the reverted commit `0269720` exists to remember
(`_grounding.md:41-50`).

**10. Who owns what, so the pointers resolve.** From `_decomposition.md:90-105`: DoD 1–2 →
`typed-layer-and-alpha-release` (HS-P0011); DoD 3 → `sqlite-durable-store` (HS-P0012); DoD 4 →
`cloudflare-durable-object-store` (HS-P0013); DoD 5–6 → `postgres-and-neon-stores` (HS-P0014); DoD 7
→ `projection-store-freeze` (HS-P0010); DoD 8 → `ladybug-projection-store` (HS-P0015); DoD 9–12 →
`publication-and-positioning` (HS-P0016); DoD 14 → `replication-identity-and-ingest` (HS-P0017);
DoD 15 → `retention-and-incomplete-logs` (HS-P0018).

## Integration contract

- **Archetype**: `capability` — a slice through the whole closeout record, observable by the reader
  it is written for.
- **Slice / milestone**: `dod-re-observation`. Slice-mate: **`published-tree-delta-statement`**
  (DoD 13 / project AC-004). The two are one artefact with two halves, read in the same sitting by
  the same reader (`_storymap.md:75-77`); the delta statement exists to qualify a phrase in the DoD
  preamble that this story's table is answering.
- **Mount point**:
  `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md` — the
  project's single convergence artefact (`_storymap.md:24-30`), stood up by `clean-checkout-harness`
  in slice 1. **It does not exist at spec time**; it exists by the time this story runs, because
  `whole-gate-green-on-the-assembled-tree` (this story's `depends_on`) has already appended its own
  section to it. This story appends a new `## DoD re-observation (1–12, 14–15)` section to that same
  file. Not a new file, not a per-scenario file.
- **Wires into** (real sibling contracts, by path):
  - `.bklg/from-contract-to-published-library/closeout-and-durable-audience/whole-gate-green-on-the-assembled-tree/` —
    the slice-1 run: its SHA, its exit code, and its per-step outcomes are this story's state
    contract. Rows re-proved inside the gate cite it and must not re-run it.
  - `xtask/src/main.rs` — the gate defined once; its module doc is the authoritative list of which
    steps run and which four are tool-gated (`_grounding.md:54-66`). The commands in the map below
    are read off it, not invented.
  - `.redkiln/config.yaml:60` (`e2e: cargo xtask ci`, the terminal grain), `:55` (the `--fast` bar
    this project must not substitute), `:67` (`require_ledger`, which is what makes every sibling's
    pointer exist at all), `:5` (`support_initiative`, where findings route).
  - `examples/course-subscriptions/`, `crates/happenstance-testkit/src/suite.rs`,
    `spec/SPECIFICATION.md`, `spec/E2E-CASES.md` — the real artefacts the re-runs execute against.
  - Each producing sibling's project directory under
    `.bklg/from-contract-to-published-library/<sibling>/` — followed for its `_ledger.md` pointers,
    which tell this story *what* to re-run and never *whether* it passed.
- **Renders surfaces**: **none.** `_design.md` `## Items` is `N/A — no public surface`; the project
  adds, changes or removes zero items in any crate's public API (`_design.md:41-45`). This story
  claims no `path` id, and an implementer who finds themselves adding one has left the story.
- **Conformance rule(s)**: **none — and this is not adapter-observable.** No rule in
  `crates/happenstance-testkit/src/suite.rs` is added, changed or removed. The suite is *re-run* as
  evidence for DoD 3–7, not edited. The story changes no port, so there is no port change for a rule
  to fail.
- **Clause(s)**: **none discharged or amended.** `spec/SPECIFICATION.md` is not edited.
  `cargo xtask spec-trace` runs as evidence for DoD 12; a clause found wrong by that run is a routed
  finding, and any change to a `[FROZEN]` clause would take a new ADR and a re-plan (`project.md`
  DR-12), never an edit inside this story.
- **Advances DoD scenario**: this story *observes* rather than produces, and what it moves toward
  green is the initiative's DoD **preamble obligation itself** (`initiative.md:356-358`) across
  scenarios **1–12 and 14–15**. At project grain it lands the first half of `project.md` *Definition
  of done* item 2 (*"the re-observation record exists, covers DoD 1–12 and 14–15 with cited
  artefacts"*); the second half — *"and states the published-tree delta"* — is the slice-mate's.
  Explicitly **not** DoD 13 and **not** DoD 16.

## PR boundary

**In this PR**

- The `## DoD re-observation (1–12, 14–15)` section appended to `_closeout-record.md`: the header
  block (SHA, date, the slice-1 gate run it is anchored to), the fourteen-row table in the fixed
  column shape, the mode legend, and the explicit DoD 13 / DoD 16 exclusion note.
- The re-runs, conditional re-runs and re-inspections themselves, executed on the clean-checkout tree
  at the slice-1 SHA.
- Any finding the re-observation surfaces, recorded **as a row plus a findings entry with a proposed
  destination**, handed to `findings-disposition-register`.
- This story's own `_ledger.md` and stage artefacts under its backlog folder.
- Mounting the section into `_closeout-record.md` — the composition root named in the *Integration
  contract*. Touching that file is the mount, not scope drift.

**Explicitly not in this PR**

- **The DoD 13 delta.** The published `0.2.0` tag, the commits between it and this tree, and the
  reason the published surface is unchanged are `published-tree-delta-statement`'s (project AC-004).
- **DoD 16.** Persona and journey promotion is `product-atom-promotion-via-kb-ingest`'s.
- **Running `cargo xtask ci`.** Slice 1 owns the run; this story cites it. And never `--fast` (DR-2).
- **Any fix.** Not a code fix, not a spec-clause fix, not a KB fix, not a "while I was in there".
  Every defect leaves as a routed finding (AC-013).
- **Any disposition decision.** This story *proposes* a destination; `findings-disposition-register`
  owns the routing.
- **Any production code, public item, conformance rule, `SPECIFICATION.md` clause or `.kb/` atom.**
- **Re-publishing anything** to satisfy a registry-facing scenario.

**Merge DoD (one line)**: `_closeout-record.md` carries a fourteen-row re-observation section, all
rows observed at one recorded SHA, every row naming its mode, its command and an artefact produced
here rather than a sibling's ledger — and every non-green row already has a proposed destination.

```
.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md
.bklg/from-contract-to-published-library/closeout-and-durable-audience/dod-set-re-observation-record/**
```

## Behavior and interfaces

### The contract this story delivers

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **One section, one file** | A `## DoD re-observation (1–12, 14–15)` section is appended to the project's existing closeout record. No sibling file is created; no per-scenario file is created. | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md` (mount point, stood up in slice 1); `_storymap.md:24-30` |
| **Fixed row shape, eight columns** | `DoD` \| `Scenario` \| `Owning project` \| `Mode` \| `Command run here` \| `Artefact produced here` \| `Pointer (sibling ledger)` \| `Outcome`. Pointer and evidence are **different columns** by design: this is what makes "a ledger may be the pointer, never the evidence" mechanically checkable instead of a matter of reading care. | `project.md:200-203`; `discover.md:38` (the wrong implementation this shape rejects) |
| **Header block anchors the set** | The section opens with the commit SHA, the observation date, and a citation of the slice-1 gate run whose SHA it is. Every row is observed at that SHA. | `whole-gate-green-on-the-assembled-tree/` (slice-1 story dir); `.redkiln/config.yaml:60` |
| **Three modes, legend included** | `re-run` (a command re-derives the claim here), `conditional re-run` (a command exists but needs an environment — the row names the absent environment if it was absent), `re-inspect` (no command re-derives it here — the row says whether the claim is documentary or a property of the published artefact, and confirms it still holds at this SHA). | `project.md:135-138` (DR-3 permits either, requires the record to say which); `project.md:131-134` (DR-2's absent-tool accounting, which the conditional mode mirrors) |
| **Exclusion note** | The section states that DoD 13 is `published-tree-delta-statement`'s and DoD 16 is `product-atom-promotion-via-kb-ingest`'s, with the story slug for each — so fourteen reads as scope, not omission. | `_storymap.md:131-134` |
| **Findings, not fixes** | Any row whose outcome is not green also appears in a short findings list with a **proposed** destination (`support`, a new item against the named owning sibling, or a decision atom + re-plan). No repair is attempted. | `project.md:179-183` (DR-12); `project.md:237-240` (AC-013); `.redkiln/config.yaml:5` |
| **Commands are read off the tree, not invented** | Where a scenario's command is fixed by this tree, the map below fixes it. Where it depends on what the producing sibling actually landed (DoD 2's compile-fail harness, whose `trybuild` dependency ADR-0015 reserved to phase 6), the row follows that sibling's ledger **to learn the command**, then runs it here and cites its own output. | `xtask/src/main.rs` module doc; `_grounding.md:54-66`; `crates/happenstance-core/src/event.rs:102-106` |
| **No surface, no rule, no clause** | Zero public items (`_design.md` `## Items` = N/A), zero changes to `crates/happenstance-testkit/src/suite.rs`, zero edits to `spec/SPECIFICATION.md`. | `_design.md:41-45`; `project.md:117-120` |

### The fourteen scenarios and how each is re-proved on this tree

This table resolves both questions `discover.md:28-29` deferred to the spec. It is the decided
content of the record's *Mode* and *Command* columns; the *Artefact produced here* column is filled
at run time.

| DoD | Scenario (`initiative.md:360-407`) | Owner | Mode | How it is re-proved here |
| --- | --- | --- | --- | --- |
| 1 | @smoke — the worked example runs end to end | HS-P0011 | **re-run** | `cargo run -p course-subscriptions` on the clean checkout; capture the printed outcomes and confirm no `todo!()` was reached. Evidence is the captured transcript. (`examples/course-subscriptions/`, `CLAUDE.md` *Commands*) |
| 2 | @smoke — the compiler protects the domain | HS-P0011 | **re-run** | The compile-fail case as HS-P0011 landed it, executed by the command its ledger names — a `compile_fail` doctest runs inside the gate's `tests`/docs steps; a `trybuild` snapshot runs as its own test target. ADR-0015 reserved that dependency choice to phase 6, so the ledger is the pointer to *which*, and the run here is the evidence. Confirm removing the protection makes the case fail is recorded by HS-P0011; re-observation confirms the case is present and green at this SHA. (`crates/happenstance-core/src/event.rs:95-106`) |
| 3 | The durable store passes the suite for real (64 contenders, reopen) | HS-P0012 | **re-run** | The adapter's `event_store_conformance!` invocation, executed here (`cargo test -p happenstance-sqlite --all-features`), including the concurrency case and the reopen case. Evidence is this run's test output, not HS-P0012's. (`crates/happenstance-testkit/src/suite.rs`, `crates/happenstance-testkit/src/concurrency.rs`) |
| 4 | The constrained-runtime store passes on its own target | HS-P0013 | **re-run** | Two halves, both cited: the gate's `wasm32-unknown-unknown` conformance-harness check inside the slice-1 run, and the adapter's own suite executed on the edge runtime per HS-P0013's recorded procedure. This is DR-5 / project AC-002's `!Send` evidence arriving at closeout rather than at HS-P0013's own merge. (`xtask/src/main.rs` wasm32 steps; `CLAUDE.md` binding constraint 1) |
| 5 | A store that does not serialise its writers passes, cost measured | HS-P0014 | **conditional re-run** | `cargo test -p happenstance-postgres --all-features` against a live Postgres, plus confirmation that the position-visibility measurement artefact exists and its numbers are the ones cited. If no live Postgres is reachable, the row records **`conditional re-run — absent: live Postgres server`** and re-inspects the measurement instead, in the same shape DR-2 requires of a `skipped` gate step. |
| 6 | A store with no connection, no interactive transaction and no cursor passes | HS-P0014 | **conditional re-run** | `cargo test -p happenstance-neon --all-features` against a reachable Neon endpoint; if unreachable, **`conditional re-run — absent: Neon HTTP endpoint`** and re-inspection of the recorded run, plus confirmation of whether the contract was amended by decision record (the scenario's own alternative branch) and, if so, that the suite was re-run after. |
| 7 | The projection suite discriminates | HS-P0010 | **re-run** | The projection conformance invocation plus `cargo xtask proof-artefact`, which is the step that holds the suite to its own mutant-detection proof — i.e. it re-proves *by name* that the deliberately wrong implementation (checkpoint written without its read model) fails. Both run inside the slice-1 gate; cite that run's step output. (`xtask/src/main.rs:174-190`, `_grounding.md:54-66`) |
| 8 | The freeze verdict is written | HS-P0015 | **re-inspect** | The claim is **documentary** — no command re-derives a written verdict. Confirm the verdict document exists on this tree, states a verdict either way, and names what it was checked against; and confirm the thing it was checked against (the unlike batch shape) is the same thing DoD 7's row just re-ran green. Reason recorded in the row. |
| 9 | @smoke — a stranger can install it | HS-P0016 | **re-run** | Genuinely re-runnable and re-run: in a scratch project **outside this workspace**, `cargo add` the published crate from the registry at its published version and run the smallest write-then-read cycle. This needs no re-publish. Evidence is the transcript pasted into the record, since the scratch project is deliberately outside the repo (a path dependency would defeat the scenario). |
| 10 | The published crate looks finished | HS-P0016 | **mixed — stated per half** | *Re-run*: the rendered documentation build under all features, inside the slice-1 gate, plus the gate's `cargo xtask package-check` licence/README-in-the-packaged-artifact assertion. *Re-inspect*: the registry page and the docs.rs render, which are properties of the **published version**, not of this tree — looked at, with the inspection date recorded. Both halves appear in the one row. (`xtask/src/main.rs:528` `package-check`) |
| 11 | The release is diffed, not asserted | HS-P0016 | **re-inspect** | Re-running the surface diff here would answer a **different question** — it would diff the closeout tree against the baseline, which is DoD 13's question and the slice-mate's (`published-tree-delta-statement`, project AC-004). So: confirm HS-P0016's diff report exists, that the version chosen matches what it found, and that nothing on this tree has since changed the compared surface. The reason for not re-running is stated in the row, because "we could have run it" is exactly the objection this mode invites. |
| 12 | The clause ledger is audited at publish | HS-P0016 | **re-run** | `cargo xtask spec-trace` inside the slice-1 gate re-derives the maturity report on this tree, and the count is reconciled against `spec/SPECIFICATION.md:219-222`'s own stated figure (200 clause IDs, 198 normative: 139 `[FROZEN]`, 49 `[PROVISIONAL]`, 10 `[DEFERRED]`, 2 `[NON-NORMATIVE]`). Also confirm no clause is provisional with an empty falsifier. Cite the corrected `219-222` range, not the intake brief's `215-221` (`_grounding.md:79-84`). |
| 14 | Replication has an answer on disk | HS-P0017 | **re-run** | A static re-run: confirm the accepted decision atom under `.kb/decisions/` answers whether ingest re-checks a writer's asserted conditions (or explicitly refuses, with reasons), that the corresponding open-question atom under `.kb/open-questions/` reflects that resolution with an annotated bullet in `.kb/maps/open-questions-index.md`, and that `redkiln validate --kb` exits zero **on this tree**. The atom filenames come from HS-P0017's ledger as the pointer; the `validate --kb` run here is the evidence. |
| 15 | Incomplete logs have an answer on disk | HS-P0018 | **re-run** | Same bar, same shape, for the incomplete-log answer: the decision atom, the resolved open-question atom (`.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md` is among the six `project.md` DR-7 names by filename), the index annotation, and `redkiln validate --kb` green here. Also confirm the scenario's substantive half — a store holding only a suffix of its own log exercised against a reader — was exercised, or that the refusal to define it is the recorded decision. |

Two consequences worth stating plainly, because both are easy to get wrong:

- **The gate is cited, not re-run.** DoD 4 (in part), 7, 10 (in part) and 12 are re-proved *inside*
  the slice-1 `cargo xtask ci` run. Their rows cite that run's step output and its SHA. Re-running
  the gate for them would produce a second SHA and break decision 4 of the *Context pack*.
- **"Re-inspect" is a decision, not a fallback.** Only DoD 8, DoD 10's registry half and DoD 11 are
  re-inspection, and each row carries its reason. A row that quietly becomes re-inspection because
  re-running was inconvenient is the failure `discover.md:29` warned this spec to prevent: letting
  "re-observed" silently mean "re-read".

## Data and migrations

**N/A — no schema, no database, no code.** This story writes no production code and touches no
persistent store; the only thing it persists is markdown in the backlog, and `_design.md` records
that this project adds, changes or removes zero public items.

One near-neighbour of a migration is real enough to name, so it is not discovered later as a
surprise: **`_closeout-record.md` is append-only across five slices and eleven stories.** It is
stood up by `clean-checkout-harness`, extended by `whole-gate-green-on-the-assembled-tree` before
this story, and extended again afterwards by `published-tree-delta-statement`,
`decision-atom-audit-table`, `open-question-preservation-audit`, `backlog-and-kb-health-at-closeout`,
`findings-disposition-register` and `initiative-closeout-readiness`. This story therefore **appends a
new section and rewrites none** — it does not reflow, renumber or restructure an earlier story's
section, and it does not change the header block slice 1 wrote. The eight-column row shape fixed
above is this story's own section's shape; later stories are not bound to it, and no later story is
entitled to widen it here.

## Acceptance criteria

Seven criteria, each written from the intent of the reader this project exists for — the
evaluator/closeout reader `_storymap.md:16-22` names, who *"must be able to believe the initiative
closed honestly without re-deriving the evidence"*, and whose time-boxed, one-shot reading mechanism
`../_discovery/distillation/personas-and-journeys.md` documents as Persona 4. Together they cover
project **AC-003** (`project.md:200-203`) whole: AC-002 covers *"all fourteen"*, AC-003 covers
*"re-run or re-inspected"*, AC-004 covers *"the command run on this tree and the artefact path it
produced"* together with *"a ledger may be the pointer, never the evidence"*, AC-005 covers the word
*"set"*, AC-001 covers *"one record"*, AC-006 keeps the observation honest under the pressure to
repair, and AC-007 makes the whole thing legible to the reader it is addressed to.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** a reader who opens the project's single closeout artefact expecting one place where the closeout evidence lives, **WHEN** they scroll past the **Gate run** section `whole-gate-green-on-the-assembled-tree` wrote, **THEN** they meet exactly one `## DoD re-observation (1–12, 14–15)` section *inside that same file*, no rival `dod-re-observation.md` or per-scenario file exists beside it in the project directory, and every line written before it is byte-identical — the section is appended, and nothing earlier is reflowed, renumbered or restructured. | `rg -c '^## DoD re-observation' .bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md` returns `1`; `git diff <slice-1-sha>..HEAD -- <record>` shows zero deleted lines; a listing of the project directory shows no new `*.md` beside the record. Captured in `.bklg/from-contract-to-published-library/closeout-and-durable-audience/dod-set-re-observation-record/evidence/record-shape-checks.md`. |
| **AC-002** | **GIVEN** the same reader counting what was promised against what was observed, **WHEN** they read the section, **THEN** it presents a **markdown table** — not a bullet list and not prose — of **exactly fourteen data rows** under an **eight-column** header, whose `DoD` column is exactly the set {1,2,3,4,5,6,7,8,9,10,11,12,14,15}; and immediately beneath it a note names DoD 13 as `published-tree-delta-statement`'s and DoD 16 as `product-atom-promotion-via-kb-ingest`'s, so fourteen reads as scope rather than as two forgotten scenarios. | Row extraction over the section: fourteen `\|`-delimited data rows, header cell count equals 8, DoD id-set equality against the set above; `rg` for both story slugs in the exclusion note, each confirmed to name a real directory under `.bklg/from-contract-to-published-library/closeout-and-durable-audience/`. Captured in `evidence/record-shape-checks.md`. |
| **AC-003** | **GIVEN** a reader who has been told these were "re-observed" and wants to know what that word bought, **WHEN** they read any row, **THEN** the row's `Mode` cell carries a token from the three-mode legend printed in the section itself (`re-run`, `conditional re-run`, `re-inspect`) — DoD 10 alone carrying two tokens for its two halves; **every** `re-inspect` row states in the row why no command re-derives the claim here (a documentary verdict, or a property of the published artefact rather than of this tree) and confirms the claim still holds at this SHA; and **every** `conditional re-run` row that could not execute names the absent environment in DR-2's absent-tool shape (`conditional re-run — absent: live Postgres server`), so no row degrades silently into a re-read. | Per-row mode-token membership check; assertion that the re-inspection set is exactly {DoD 8, DoD 10's registry half, DoD 11} and that each carries a reason clause; assertion that DoD 5 and DoD 6 each show either captured run output or a named absent environment. Read against `project.md:131-138` (DR-2, DR-3). Captured in `evidence/mode-audit.md`. |
| **AC-004** | **GIVEN** a reviewer applying `discover.md:38`'s own test — diffing the fourteen artefact citations against the sibling ledgers they resemble — **WHEN** they check each row, **THEN** pointer and evidence sit in **different columns** (`Pointer (sibling ledger)` versus `Artefact produced here`), no path appears in both, no cell in `Artefact produced here` is any `_ledger.md`, every one of the fourteen artefact paths resolves on this tree, and each was produced or captured by *this* story's observation run rather than pre-dating it — so a row whose only citation is the producing project's memory cannot exist. | Per row: the `Artefact produced here` path exists; it does not match `*_ledger.md`; it is either under `.bklg/.../dod-set-re-observation-record/evidence/` or appears in this story's own commit range (`git log --diff-filter=AM --format=%H -- <path>`); the set intersection of the two columns is empty. Captured in `evidence/pointer-vs-evidence.md`. |
| **AC-005** | **GIVEN** a reader asking the question the word "set" exists to answer — *were these fourteen observed on one tree?* — **WHEN** they read the section header, **THEN** it carries one commit SHA, the observation date and a citation of the slice-1 `cargo xtask ci` run; that SHA equals the one `whole-gate-green-on-the-assembled-tree` recorded; every row is stated as observed at that SHA; and no row's `Command run here` cell contains a `cargo xtask ci` invocation at all — the rows the gate re-proves (DoD 4 in part, 7, 10 in part, 12) cite that run's step output instead of spawning a second run at a second SHA, and `--fast` appears nowhere in the section. | Header SHA compared against the SHA recorded by the slice-1 story; `git rev-parse HEAD` at observation time recorded beside it; `rg 'cargo xtask ci'` over the section matches only citation prose, never a `Command run here` cell; a search for `--fast` over the section returns nothing. Captured in `evidence/sha-anchor.md`. |
| **AC-006** | **GIVEN** the pressure `project.md`'s risk table names ("Pressure to fix what re-observation finds"), **WHEN** a scenario does not come back green, **THEN** that row's `Outcome` names the failure plainly, a findings list beneath the table gives that finding a **proposed** destination drawn from DR-12's three (`support` per `.redkiln/config.yaml:5`, a new item against the named owning sibling, or a decision atom plus a re-plan where a `[FROZEN]` clause is touched), the disposition itself is left to `findings-disposition-register`, and this story's commits touch **no** file outside `_closeout-record.md` and its own story directory — zero repairs, and an all-green table states "no findings" explicitly rather than omitting the list. | `git diff --name-only <base>..HEAD` is a subset of the two paths in the *PR boundary* fence — in particular an empty intersection with `crates/`, `xtask/`, `spec/`, `examples/` and `.kb/`; for each non-green `Outcome` cell a findings entry exists whose destination is one of the three enumerated; if there are none, the literal no-findings statement is present. Captured in `evidence/findings-and-boundary.md`. |
| **AC-007** | **GIVEN** the one-shot evaluator/closeout reader who cannot ask the author what a row meant, **WHEN** they pick any of the fourteen rows and try to follow it end to end, **THEN** the `Command run here` cell is a runnable invocation naming a real target on this tree, the `Artefact produced here` cell resolves to a file they can open, the `Owning project` cell resolves to a real sibling project directory, no cell exceeds roughly 200 characters (longer material moves to a numbered note beneath the table), and any captured transcript of 20 lines or more lives in a linked file under this story's directory rather than inline — so the table is readable in one sitting and no row requires asking its author what it meant. | A recorded follow-through pass over all fourteen rows: each command's package name checked against `cargo metadata --no-deps --format-version 1`, each `xtask` subcommand against the dispatch match at `xtask/src/main.rs:671-681`; every artefact path opened; every owning-project directory confirmed under `.bklg/from-contract-to-published-library/`; a cell-length and transcript-inlining scan over the section. Captured in `evidence/follow-through.md`. |

## Interaction quality

This story renders **no screen and no public API item**: `_design.md` `## Items` is `N/A`, and its
sign-off records `design.capture` as a *declared* skip because there is no app to screenshot
(`_design.md:41-45`, `:89-100`). That does not make this section vacuous. The artefact **is** the
surface, and its medium is the record a reader opens. The invariants below are therefore stated over
`_closeout-record.md`'s composition, and **each already lives as an AC-### row in the table above** —
this section only says which id carries it and how it is checked. Nothing here adds a criterion, and
nothing here is gated by being written here.

**STATE invariants**

| invariant | carried by | how it is verified |
| --- | --- | --- |
| **In place, not a context jump** — the reader stays inside the one record; the evidence is not relocated to a file they must be told about separately | **AC-001** | single-heading count in the record; no new `*.md` beside it in the project directory |
| **Non-occlusion** — the appended section hides or overwrites nothing an earlier story wrote | **AC-001** | zero deleted lines in the record's diff since the slice-1 SHA |
| **Preserved reading position** — earlier sections are not reflowed, renumbered or restructured, so an existing citation into them by line still lands | **AC-001** | byte-identity of every line preceding the new section |
| **Reversibility** — the contribution is a pure append inside this story's own commits, revertable without disturbing slice 1's section | **AC-001**, **AC-006** | append-only diff plus the changed-file boundary check |
| **Reachability without scrolling** (the keyboard analogue in this medium) — any scenario is reachable by a plain-text search for its DoD number, not only by eye | **AC-007** | the follow-through pass locates each of the fourteen rows by its `DoD` cell |

**COMPOSITION invariants** — from what `_design.md` actually approved (the no-surface determination
and its anti-patterns) plus the row shape fixed under *Behavior and interfaces*:

| invariant | real numbers | carried by | how it is verified |
| --- | --- | --- | --- |
| **Presentation exists at all** — a composed table with a printed legend and a header block, never a bare fourteen-line bullet list or a paragraph of prose | 8-column header; 3-mode legend; 1 header block | **AC-002**, **AC-003** | header cell count equals 8; legend present inside the section itself |
| **Composition and placement** — inside `_closeout-record.md`, after slice 1's **Gate run** section, before whatever later slices append | 1 section, 1 file | **AC-001** | heading order within the record |
| **Transience** — the legend and the SHA/date header are **persistent chrome** printed in the section (a reader never holds them in memory or opens this spec); transcripts of 20 lines or more are **opened on demand** as linked files under the story directory, never inlined into a cell | ≥ 20 lines ⇒ linked | **AC-003**, **AC-007** | legend/header presence check; transcript-inlining scan |
| **Density budget** — fourteen rows by eight columns, one line per row; no cell over roughly 200 characters, overflow moved to a numbered note beneath the table; section body under roughly 120 lines excluding linked transcripts | 14 rows; 8 columns; ~200 chars/cell; ~120 lines | **AC-002**, **AC-007** | row and column counts; cell-length scan; section line count |
| **Hierarchy** — H2 section → header block (SHA, date, gate-run citation) → legend → table → exclusion note → findings list, in that order | 6 blocks, fixed order | **AC-002**, **AC-005**, **AC-006** | block-order check over the section |
| **Anti-pattern: the ledger in the evidence column** (`discover.md:38`) | 0 permitted | **AC-004** | no `_ledger.md` in `Artefact produced here`; empty intersection of the two columns |
| **Anti-pattern: a rival file** — `dod-re-observation.md` beside the record, satisfying every literal word of AC-003 and defeating the word "set" | 0 permitted | **AC-001** | project-directory listing |
| **Anti-pattern: silent re-inspection** — a row that becomes re-inspection because re-running was inconvenient, with no reason stated | 0 permitted | **AC-003** | reason clause required on every `re-inspect` row |
| **Anti-pattern: two SHAs** — rows spread across two trees | 0 permitted | **AC-005** | one header SHA; no second gate invocation |
| **Anti-pattern: inventing a surface** — adding a public item, a conformance rule or a clause "while I was in there" | 0 permitted | **AC-006** | changed-file boundary check |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | `_closeout-record.md` does not exist when this story runs — `clean-checkout-harness` / `whole-gate-green-on-the-assembled-tree` did not land | **Halt loudly and report the missing dependency.** Do not create the record from scratch here: the header block and the **Gate run** section are slice 1's, and a record this story invents has no SHA to anchor to (AC-005). |
| **EC-002** | The slice-1 gate run is **red**, or a step printed `skipped` with no named absent tool | Proceed with the re-observation and anchor to that SHA. The fourteen rows are recorded honestly against a red gate; the gate failure is *slice 1's* finding, already routed. This story neither waits for it nor fixes it (AC-006). |
| **EC-003** | The environment a conditional re-run needs is absent (no reachable Postgres for DoD 5, no reachable Neon endpoint for DoD 6) | Record `conditional re-run — absent: <environment>` in the `Mode` cell, re-inspect the recorded measurement or run instead, and say so in the row. Never relabel the row `re-inspect` without the absent-environment name, and never relabel it `re-run` on the strength of the sibling's output (AC-003). |
| **EC-004** | A producing sibling's `_ledger.md` does not name the command to re-run — notably DoD 2, where ADR-0015 reserved the `trybuild`-versus-`compile_fail` choice to phase 6 and the ledger may record only the outcome | Do **not** guess a command. Read the sibling's landed test target directly and cite what was actually run here. If neither the ledger nor the tree yields a runnable command, the row's outcome is `pointer unavailable` and it becomes a routed finding (AC-004, AC-006). |
| **EC-005** | A re-run fails for an environmental reason unrelated to the claim (absent toolchain, absent optional tool, no network) | Distinguish it from a red claim in the shape DR-2 requires of the gate: the row names the absent tool and the outcome is **not** recorded as a passing observation. An absent tool is never a green row (AC-003). |
| **EC-006** | A commit lands on the tree part-way through the observation, so rows would straddle two SHAs | Re-anchor: re-observe every row already taken, at the new SHA, or restart the set. Fourteen rows across two trees is not a set and fails AC-005 by construction. |
| **EC-007** | A finding touches a `[FROZEN]` clause in `spec/SPECIFICATION.md`, or a port surface | The proposed destination is a **decision atom plus a re-plan** (`project.md:179-183`) — never an edit, and never a `support` ticket. `spec/SPECIFICATION.md` is not touched by this story under any outcome. |
| **EC-008** | Re-observation finds two sibling claims that contradict each other — one project's DoD claim quietly invalidated by another's later merge, the interaction `discover.md:34` says only this story can see | Record **both** rows honestly with the contradiction named in their `Outcome` cells, and raise one finding against the owning sibling. This is the story's highest-value output, not an exception to it. |

## Non-functional

| id | requirement | why, and how it is judged |
| --- | --- | --- |
| **NF-001** | **Repeatability.** A second reader who runs a `Command run here` cell verbatim at the recorded SHA gets the same outcome. Commands are copy-pasteable, workspace-relative invocations — no shell aliases, no machine-specific paths. | The record's whole claim is that the observation is checkable rather than remembered (DR-3). Judged during AC-007's follow-through pass. |
| **NF-002** | **No machine-local leakage.** Captured transcripts — notably DoD 9's, taken in a scratch project deliberately outside this workspace — have absolute host paths normalised, and connection details for DoD 5/6 appear as environment-variable **names**, never values. | A pasted transcript is the one place a closeout record can leak a credential or a home directory. Scanned with the transcript check under AC-007. |
| **NF-003** | **One sitting.** The section body, excluding linked transcripts, stays within roughly 120 lines: fourteen one-line rows plus header, legend, exclusion note and findings list. | The reader is time-boxed and one-shot (`../_discovery/distillation/personas-and-journeys.md`, Persona 4). Enforced through AC-007's density rules; a longer section means material belongs in a linked file. |
| **NF-004** | **Append-only durability.** The contribution survives six further stories appending to the same file, because it neither depends on nor alters their content. | Stated under *Data and migrations*; verified by AC-001's zero-deletion diff. |
| **NF-005** | **Wall-clock honesty.** The re-runs are budgeted rather than rushed — the sqlite conformance run at 64 contenders and the workspace test re-runs are the expensive ones — and no row is marked green on a run that was cancelled. | A cancelled run recorded as green is the same failure as a remembered tick, one recursion down. Judged by the presence of captured output behind every `re-run` row (AC-004). |

## Implementation notes (non-prescriptive)

Shape only; the implementer chooses the mechanics.

- **Read the SHA first, then observe.** Take the commit SHA and the per-step outcomes from
  `whole-gate-green-on-the-assembled-tree`'s record *before* running anything, write the header
  block, and only then work the rows. Anchoring last is how EC-006 happens.
- **Split the fourteen into three passes, in this order.** First, the rows slice 1's gate already
  re-proved (DoD 4 in part, 7, 10 in part, 12) — pure citation, no execution. Second, the rows with
  a local command (DoD 1, 2, 3, 14, 15, and DoD 9's out-of-workspace scratch project). Third, the
  rows needing a judgement call (DoD 5 and 6 conditional; DoD 8, DoD 10's registry half and DoD 11
  re-inspection). The third pass is where the story is actually decided, and doing it last means the
  first two have already taught you what this tree contains.
- **Capture as you go, into the story directory.** An `evidence/` folder under
  `.bklg/.../dod-set-re-observation-record/` holding one file per check named in the verification
  column keeps the `Artefact produced here` column short and satisfies AC-007's transcript rule
  without inventing a rival record. The *PR boundary* fence already admits the whole story directory.
- **Write each row when its command exits, not at the end.** Fourteen rows written from memory after
  fourteen runs is precisely the failure this story exists to reject.
- **DoD 9 needs somewhere outside this workspace.** A scratch `cargo new` in a temporary directory,
  `cargo add` of the published crate from the registry at its published version, one write-then-read
  cycle. A path dependency there defeats the scenario entirely; the transcript comes back into the
  story directory as a file.
- **`redkiln validate --kb` is one run covering two rows.** DoD 14 and DoD 15 both need it green on
  this tree: run it once, cite it twice, and keep the per-row difference in the atom filenames each
  row confirms.
- **Do not open a fixing branch.** If a re-run goes red, write the row, write the finding with a
  proposed destination, and move to the next row. `findings-disposition-register` owns what happens
  next (`_storymap.md:62`).

## Tests and CI (merge gate)

The tiers come from the project's one warranted brief — `_decomposition.md` *Testing brief*: the
AC-003 row at `:46` (**E2E / process**) and the tier-by-tier mix at `:61-91`. The merge-gate block at
`:93-107` is the **project's** bar; this story's own grain is the `affected` gate, and the whole gate
is *cited from slice 1*, never re-run here (DR-2, AC-005).

| tier | command / path | proves |
| --- | --- | --- |
| **Story grain (deterministic gate)** | `cargo xtask affected --base main` (`.redkiln/config.yaml:39`) | This story's diff breaks no package. A markdown-only diff maps to no package and the gate is still not vacuous: it runs the five file-reading lints and `spec-trace` unconditionally, which is the exact case `.redkiln/config.yaml:33-37` documents. |
| **Integration (static reachability)** | `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:48`) | No citation this story adds rots the tree's cross-references, and every clause/rule citation `spec-trace` walks still resolves. |
| **Ledger (story DoD)** | `redkiln verify --grain story` over `.bklg/.../dod-set-re-observation-record/_ledger.md` (`.redkiln/config.yaml:67`) | AC-001…AC-007 are each present, `satisfied: true`, and carry non-placeholder cited evidence. This is the gate that makes the seven criteria binding rather than decorative. |
| **Provenance** | `redkiln record-links --sha <commit>` (`.redkiln/config.yaml:73`) | The contribution is tied to a real commit — which is also what AC-004's "produced here, not pre-dating this story" check reads. |
| **E2E (cited, not re-run)** | `cargo xtask ci` as executed by `whole-gate-green-on-the-assembled-tree` (`.redkiln/config.yaml:60`) | DoD 4's wasm32 conformance-harness half, DoD 7 (`proof-artefact`, `xtask/src/main.rs:174-190`), DoD 10's docs and `package-check` half (`xtask/src/main.rs:528`) and DoD 12 (`spec-trace`). Cited by SHA and step output; re-running it here would break AC-005. |
| **Integration (re-run here)** | `cargo test -p happenstance-sqlite --all-features` — the adapter's `event_store_conformance!` invocation over `crates/happenstance-testkit/src/suite.rs` | DoD 3, including the 64-contender concurrency case and the reopen case, on *this* tree rather than on HS-P0012's. |
| **Integration (conditional)** | `cargo test -p happenstance-postgres --all-features`; `cargo test -p happenstance-neon --all-features` | DoD 5 and DoD 6 where the environment is present; where it is absent, the absent environment is named and the measurement re-inspected (EC-003). |
| **E2E (smoke, re-run here)** | `cargo run -p course-subscriptions` over `examples/course-subscriptions/` | DoD 1 — the canonical DCB cycle completes and prints the observable outcomes, with no `todo!()` reached. |
| **Unit / compile-fail (re-run here)** | The compile-fail case as HS-P0011 landed it; `crates/happenstance-core/src/event.rs:95-106` is the surface it protects | DoD 2 — the case is present and green at this SHA. The sibling ledger is the pointer to *which* harness; the run here is the evidence (EC-004). |
| **Static (KB)** | `redkiln validate --kb` over `.kb/decisions/`, `.kb/open-questions/` and `.kb/maps/open-questions-index.md` | DoD 14 and DoD 15 — the answers are on disk, the consumed open-question atoms carry a resolution-bearing status, and the index bullets are annotated. |
| **Process (record checks)** | The shape, count, mode, pointer/evidence, SHA, boundary and follow-through checks named in the *Acceptance criteria* verification column, captured under `.bklg/.../dod-set-re-observation-record/evidence/` | AC-001…AC-007 themselves. |

Deliberately **not** in this story's gate: `redkiln validate` and `redkiln doctor --json`
(`backlog-and-kb-health-at-closeout`, project AC-011/AC-012), and `cargo xtask ci --fast`, which DR-2
forbids as a substitute anywhere in this project.

## Risks and coupling (PR-scoped)

| risk | why it bites here | containment |
| --- | --- | --- |
| **The record is a contended file.** Eleven stories across five slices append to `_closeout-record.md`; a story that rewrites rather than appends silently destroys a sibling's section. | This story is the third writer and the first with a large table, so it is the most likely to be tempted into tidying what came before. | AC-001's zero-deletion diff, and the append-only statement under *Data and migrations*. |
| **The slice-mate follows immediately.** `published-tree-delta-statement` appends next and is read in the same sitting. | A column shape presented as project-wide would over-constrain it; a section that reads as complete would make the delta look optional. | The eight-column shape binds *this* section only (*Data and migrations*); AC-002's exclusion note names DoD 13's owner explicitly. |
| **Conditional environments quietly collapse the record.** If neither Postgres nor Neon is reachable, two of fourteen rows lose their re-run and the record's honesty rests entirely on naming discipline. | This is the exact drift `discover.md:29` asked the spec to prevent — "re-observed" silently meaning "re-read". | AC-003 with EC-003: the absent environment is named in the row, in DR-2's shape. |
| **Wall-clock.** The sqlite conformance run at 64 contenders, the workspace tests and a scratch-project `cargo add` build are each minutes; fourteen rows is a sitting. | A rushed pass produces exactly the remembered ticks the story exists to reject. | NF-005, and the three-pass split under *Implementation notes*. |
| **Pressure to fix.** Every red row will look like a two-line fix, especially in a project that touches no code and therefore feels safe to touch code from. | `project.md`'s risk table names this as the headline risk; project AC-013 makes routing the deliverable. | AC-006's changed-file boundary check, which fails the story mechanically rather than on reviewer vigilance. |
| **A pointer that leads nowhere.** DoD 2's harness choice was reserved to phase 6; if the sibling ledger records only an outcome, the command is unknown. | Guessing a command produces a row that looks re-run and is not. | EC-004: read the landed test target, or record `pointer unavailable` as a finding. |
| **Coupling to slice 1's SHA.** If slice 1 is re-run or amended after this story starts, the header SHA and every row go stale together. | Fourteen rows at a superseded SHA is not a set. | EC-006, and AC-005's comparison of the header SHA against slice 1's recorded SHA. |

## Dependencies

**Blocks on**

- **`whole-gate-green-on-the-assembled-tree`** — supplies the tree this story re-observes: the commit
  SHA every row is anchored to (AC-005), the `cargo xtask ci` exit code and per-step outcomes that
  the four gate-proved rows cite instead of re-running (AC-003, AC-005), and — transitively, through
  its own dependency on `clean-checkout-harness` — both the residue-free checkout (DR-1) and the
  existence of `_closeout-record.md` this story appends to (EC-001).

**Unlocks**

- **`published-tree-delta-statement`** — the slice-mate, whose `depends_on` names this story
  (`_storymap.md:56`). It appends the DoD 13 delta immediately beneath this section, qualifying the
  same DoD preamble phrase this table answers.
- **`findings-disposition-register`** — whose `depends_on` names this story (`_storymap.md:62`).
  Every finding this re-observation surfaces, with its proposed destination, is that story's input;
  it owns the disposition, this story owns the honest row.

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. Everything needed to *start* is in the *Context
pack*; open these at the moments named.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/initiative.md` | The sixteen DoD scenarios **verbatim** at `:360-407`, under the preamble at `:356-358` ("run and observed to pass on the assembled library… a green gate is a precondition for looking at these, never a substitute"). The `Scenario` column must paraphrase these, not the story map's shorthand. | Before writing the table's `Scenario` column — that is, before the fourteen rows exist. | AC-002, AC-003 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md` | AC-003's exact wording at `:200-203` (the ledger/evidence rule), DR-2 at `:131-134` (absent-tool accounting, which the conditional mode copies), DR-3 at `:135-138` (re-run *or* re-inspect, and the record must say which), DR-12 at `:179-183` (the three destinations). | Before fixing the column shape, and again before writing any finding. | AC-004, AC-003, AC-006 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md` | The project's one warranted brief: the AC-003 row at `:46`, the tier-by-tier test mix at `:61-91` naming the exact command per tier, and the merge-gate block at `:93-107`. It half-answers "which command proves which tier" before you start. | While building the three-pass plan, before running anything. | AC-003, AC-007 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_storymap.md` | `:24-30` states why the fourteen rows must converge on one artefact ("fourteen ledger entries in fourteen places is the failure mode"); `:131-134` is the DoD→AC mapping the exclusion note is built from; `:16-22` names the reader the record is written for. | Before deciding where the section lives, and before writing the exclusion note. | AC-001, AC-002 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/dod-set-re-observation-record/discover.md` | `:38` states the wrong implementation in full — fourteen rows citing sibling ledgers — together with the reviewer check that catches it (artefact timestamps predating the clean-checkout run). AC-004 is that check, mechanised. | Immediately before filling the `Artefact produced here` column, and again at self-review. | AC-004 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/whole-gate-green-on-the-assembled-tree/spec.md` | The slice-1 contract this story consumes: the four `wasm32` steps named individually, what a `skipped` line means on this machine, and the requirement that the run's SHA and exit code are recorded rather than summarised. Rows 4, 7, 10 and 12 cite that record. | **First** — before any observation, to take the SHA. Doing this late is what causes EC-006. | AC-005 |
| `xtask/src/main.rs` | The gate defined once: the module doc's accounting of which steps are tool-gated, `proof-artefact` at `:174-190` (DoD 7's mutant-detection proof), `package-check` at `:528` (DoD 10's licence/README assertion), and the subcommand dispatch at `:671-681` that AC-007's command check reads. Commands are read off this file, never invented. | When writing the `Command run here` cells for the gate-proved rows, and during the follow-through pass. | AC-003, AC-007 |
| `.redkiln/config.yaml` | `:60` `e2e: cargo xtask ci` (the terminal grain), `:55` the `--fast` bar DR-2 forbids substituting, `:67` `require_ledger` (which is why every sibling's pointer exists at all), `:73` provenance, `:5` `support_initiative` as a finding destination. | When deciding what this story runs versus cites, and when naming a finding's destination. | AC-005, AC-006 |
| `spec/SPECIFICATION.md` | `:219-222` carries the document's own stated figures — 200 clause ids, 198 normative: 139 `[FROZEN]`, 49 `[PROVISIONAL]`, 10 `[DEFERRED]`, 2 `[NON-NORMATIVE]` — which DoD 12's row reconciles against `spec-trace`'s report. Cite this range, not the intake brief's superseded `:215-221`. | While writing the DoD 12 row, and only then. | AC-003 |
| `crates/happenstance-testkit/src/suite.rs` | The conformance suite the DoD 3, 4 and 7 rows re-run. It is *re-run as evidence*, never edited — this story adds, changes and removes no rule. | Before the DoD 3, 4 and 7 rows, to state what the invocation actually covers. | AC-003 |
| `.kb/maps/open-questions-index.md` | The "a resolved question stays listed and annotated, never removed" convention that the DoD 14 and DoD 15 rows confirm on this tree alongside `redkiln validate --kb`. | While writing the DoD 14 and DoD 15 rows. | AC-003 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | Persona 4 (the evaluator, pre-adoption) and the *Cross-persona tensions* section — the time-boxed, one-shot trust-building mechanism the density budget and the follow-through rule are written for. It is the *reason* AC-007 exists rather than a nicety. | Before the final legibility pass, when judging whether a row can be followed without asking anyone. | AC-007 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_design.md` | The no-surface determination signed off 2026-08-12, with `## Items` as `N/A` and `design.capture` recorded as a *declared* skip. It is what makes "invent no surface" an inherited design decision rather than an omission. | If any part of the work starts to look like adding a public item, a conformance rule or a clause. | AC-006 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_grounding.md` | `:54-66` is the gate's anatomy — which steps are tool-gated and what each proves; `:41-50` is the `0269720` lesson (the process, not its directory layout). The first makes a `skipped` line accountable; the second is why this story hand-authors no `.kb/` atom. | Alongside `xtask/src/main.rs`, when writing the gate-proved rows. | AC-003, AC-006 |

## Clarifications resolved during spec

1. **The two questions `discover.md:28-29` deferred are decided per scenario, not by policy.** The
   fourteen-row map under *Behavior and interfaces* fixes the mode for each. Worth noting where it
   contradicts the guess: `discover.md` expected DoD 9–12 to be re-inspection wholesale, and three of
   the four are not. DoD 9 is genuinely re-runnable without re-publishing, DoD 12 is re-run inside
   the gate, DoD 10 is half and half — and only DoD 11 is re-inspection, because re-running the
   surface diff here would answer DoD 13's question, which belongs to the slice-mate.
2. **The legend stays at three modes; DoD 10 carries two tokens rather than a fourth mode.** A row
   may name two modes where the scenario genuinely has two halves — a docs/`package-check` half that
   re-runs and a registry-page half that is a property of the published version. DoD 10 is the only
   such row, and AC-003 is written to permit exactly that and nothing looser.
3. **`conditional re-run` was added as a third mode rather than folding DoD 5 and 6 into
   `re-inspect`.** DR-3's text offers two modes; recording "re-inspect" for those two would be true
   and would lose the reason. The conditional mode names the absent environment in the same shape
   DR-2 requires of a `skipped` gate step, keeping a missing environment visible as a gap rather than
   as a choice.
4. **Evidence lives in an `evidence/` folder under this story's directory, not inside the record.**
   The `Artefact produced here` column cites files there. That keeps the density budget (NF-003,
   AC-007) achievable without weakening AC-004, since those files are produced by this story's own
   run — which is exactly what the column asserts.
5. **The AC set is the seven the first pass enumerated; none added, none dropped.** Every
   interaction-quality invariant is carried by one of AC-001…AC-007 rather than by a new id, because
   each is a property of the same fourteen-row artefact; splitting them would produce ledger rows no
   separate check could distinguish.
6. **Interaction quality is stated over the record, not over a screen.** `_design.md` declares no
   public surface and a declared `design.capture` skip, so there is no mock to bind to and no
   component to compose. The composition invariants are therefore the record's own — table rather
   than prose, a persistent legend, transcripts opened on demand, a stated density budget — and they
   are real: an unformatted fourteen-line list would satisfy every path-and-count assertion in this
   spec and fail every one of them.
7. **A red slice-1 gate does not block this story** (EC-002). The rows anchor to that SHA and are
   recorded honestly. Waiting would either produce a second SHA later or invite exactly the fix
   AC-006 forbids.
