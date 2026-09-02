---
item: HS-S0082
stage: spec
created: 2026-08-12T13:47:20.534Z
updated: 2026-08-12T13:47:20.534Z
template_sig: 87bbf1d0
rendered_sig: d6f81989
---

# Spec — The freeze verdict, dated and either way

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project charter | `.bklg/from-contract-to-published-library/ladybug-projection-store/project.md` |
| This spec | `.bklg/from-contract-to-published-library/ladybug-projection-store/freeze-verdict-document/spec.md` |
| Key brief — architecture (§ *Mount points* M8/M9, § *The three tensions* T1–T3, § *ADR-0025's three questions* Q3) | `.bklg/from-contract-to-published-library/ladybug-projection-store/_decomposition.md:121-218`, `:261-311`, `:312-373` |
| Key brief — testing (AC-007 row: "reviewed, not scripted"; AC-008 row: the reviewed `git diff`) | `.bklg/from-contract-to-published-library/ladybug-projection-store/_decomposition.md:477`, `:478` |
| Signed-off design | `.bklg/from-contract-to-published-library/ladybug-projection-store/_design.md` — **no user-facing surface**, approved 2026-08-12 (`:40-98`). This story renders none. |
| Story map row (slice, one-line, "last and unconditional") | `.bklg/from-contract-to-published-library/ladybug-projection-store/_storymap.md:48`, `:24`, `:26-30`, `:83-84`, `:112` |
| Discovery (signal ledger, the three named wrong implementations, the four deferrals) | `.bklg/from-contract-to-published-library/ladybug-projection-store/freeze-verdict-document/discover.md` |
| Upstream story this one cites by SHA | `.bklg/from-contract-to-published-library/ladybug-projection-store/preflight-and-unlike-axes/spec.md:199` (the axes document's agreed path) |
| Roadmap pointer | `RUNBOOK.md:162` (phase 11's row), `RUNBOOK.md:4427-4430` (proof artefact), `RUNBOOK.md:4432-4438` (exit criteria) |

Traces to project **AC-007** (sole owner) and **AC-008** (the "nothing `[FROZEN]` moved" review and
the "did not hold" routing half; the six citation repairs are
`fill-the-bodies-and-ps-34-disposition`'s — `_storymap.md:112`). `depends_on`:
`read-your-own-writes-projection` (HS-S0079), `cold-build-cost-and-ci-shape` (HS-S0081),
`package-completeness-and-name-claim` (HS-S0080).

## One-line PR slice

Write the dated verdict — implementation, rules run, PS clause ids, commit SHA, *held* or *did not
hold* — carrying the T1, GAT/ICE and `WriteTransactionInUse` findings, routing any "did not hold" to
a decision atom and a re-plan while amending nothing `[FROZEN]`.

## Executive summary

This PR lands **one new document**, `references/evaluation/phase-11-freeze-verdict.md`, plus the two
edits that mount it: **one classification line in `references/evaluation/README.md`** and **the phase
11 exit-criteria boxes ticked in place in `RUNBOOK.md`**. On a *did not hold* verdict it additionally
stages **one note under `.kb/_intake/`**. It touches no Rust, no `spec/`, no `crates/`, no `xtask/`.

The charter already says the verdict must exist either way and must name five things
(`project.md:149-151`, `:206-209`). The delta this spec adds is seven decisions, four of them
explicitly deferred to this stage at discovery (`discover.md:74-77`):

1. **One file, named `phase-11-freeze-verdict.md`**, on the `phase-4-reconciliation.md` genre
   precedent and matching the name HS-S0074 already took for the axes
   (`preflight-and-unlike-axes/spec.md:199`).
2. **The build-cost number is cited, not restated.** It lives where AC-009 put it — `RUNBOOK.md`'s
   phase 11 body and HS-S0081's own measurement record. The verdict names the **CI shape the run
   happened under**, because that is part of "what it was checked against", and points at the number.
   A number copied into two documents is a number that will one day disagree with itself.
3. **T1 is stated in the verdict alone.** The upstream escalation already happened at HS-S0074's
   preflight finding; the verdict carries the finding text and cites that document's SHA. No second
   note is authored — a finding kept in two places is a finding that drifts, and this one has to read
   identically wherever it is read.
4. **The skeleton guard is discharged by this spec, not by a stub document.** Discovery wanted the
   fields, the rule table and the clause list fixed before the run (`discover.md:112-115`); HS-S0074's
   PR boundary has already shipped without a verdict stub in it. So the document's *shape* — its
   heading text, its five fields, its table columns, its nine clause ids — is fixed **here**, in a
   file `git log` shows predating the first conformance run, and only its *values* are written
   afterwards.
5. **The rule table's outcome column is a closed vocabulary of three values** — `passed`,
   `declined — <the fixture's stated reason>`, `failed` — not two, and its row count reconciles
   against `projection-store-freeze`'s registration count.
6. **"Did not hold" routes through `.kb/_intake/`, never `.kb/decisions/`.** The atom is authored by
   `/redkiln:kb-ingest` in its own run (`_decomposition.md:204-209`); this PR stages the note. The
   re-plan is a backlog action, not a file here.
7. **Two mounts, both required.** The README classification is what makes the document *evidence*
   rather than a file; the RUNBOOK ticks are what make phase 11 closable and give HS-S0083 something
   to hand over.

Everything that produces the verdict's content — the adapter, the fixture, the run, the
read-your-own-writes answer, the measurement, the packaging — is upstream and explicitly not here.

## Context pack

Read this section and you can start. The deeper artefacts stay behind the anchors table.

**The verdict exists whichever way the run went, and that asymmetry is the whole evidentiary value.**
`project.md:206-209` states it — "its presence is not conditional on the result" — and
`_storymap.md:26-30` states it a second time because it is the single claim most likely to erode
under a green run. An artefact that only appears when the news is good is not evidence. Everything
else below is downstream of protecting that one property.

**Five named fields, and a verdict missing any of them fails regardless of how well it reads.**
DR-4 (`project.md:149-151`): *which implementation, which rules, which PS clause ids, at which
commit, on which date* — and *"a verdict that says only 'held' is not a verdict"*. `held` or
`did not hold` is written as one of exactly those two literal strings under a fixed heading, not
inferred from a paragraph's tone.

**The most likely and most damaging mutant is the verdict that lists what passed and not what
declined** (`discover.md:117-130`). "Held. The suite ran green against `LadybugProjectionStore` at
`abc1234`; PS-4 – PS-6, PS-9, PS-11, PS-12, PS-15 and PS-34 are confirmed" satisfies every field
DR-4 names, cites a real commit, and silently omits the rules that *declined* — including the
concurrency-shaped one that declined because LadybugDB has exactly one writer. A freeze does not fail
loudly; it fails as a rule reported as a fixture limitation. The guard is structural, not diligent:
**one row per registered rule**, an outcome column in which *declined, with the fixture's stated
reason* is a distinct value from *passed*, and a row count reconciled against
`projection-store-freeze`'s registration list — the same reconciliation AC-004 already requires of
the run itself (`_decomposition.md:474`).

**A green suite is not the whole answer, and four findings exist that no suite can emit.** They reach
the verdict only because someone carries them (`discover.md:132-143`; AC-A04 at
`_decomposition.md:51-54` designates the verdict as the home for every deviation the project
accumulated):

- **T1 — the freeze deleted the counter-example that was evidence for freezing.**
  `LiveHandleProjectionStore` binds `type Batch<'a> = GraphWriteHandle<'a>`, a genuinely borrowed
  handle with no lifetime-free spelling for a `'static` store, so under a frozen `type Batch;` it
  cannot exist (`_decomposition.md:314-331`). And it was *evidence*: `references/adapter-shapes.md:169-195`
  records that the borrowed GAT compiled, ran across a real `tokio::spawn`, and **did not** fail on
  the `Send` flavour — the result that "cuts against §4.2" and that phase 6 was told to weigh. Its
  retirement costs nothing evidentially, because both transcripts survive outside the crate — but the
  verdict has to say that the freeze consumed its own counter-example, because nothing else will.
- **The GAT and the ICE, if the lifetime survived.** Today's port is implementable only by stores that
  outlive every batch lifetime, and the failure mode for one that does not is a **compiler panic, not
  a diagnostic** (`project.md:272-277`; transcript at `crates/happenstance-ladybug/src/live_handle.rs:38-66`).
  That is a finding about the port and belongs in the verdict, never a workaround absorbed quietly.
- **`WriteTransactionInUse` under a concurrency-shaped rule.** LadybugDB permits many readers and
  exactly one writer, so two commits racing is routine rather than exceptional
  (`crates/happenstance-ladybug/src/projection_store.rs:204-213`). This story **answers** it rather
  than reporting it: a *declared capability limit* means the suite is right and this adapter's
  concurrency envelope is narrower than a SQL adapter's; a *rule defect* means the rule assumed a
  concurrency model the port never promised, and it goes to HS-P0010 by name. What is fixed in
  advance is that the answer is **never "retry until green"** (`project.md:278-283`;
  `discover.md:50-58`) — a retry loop converts a design finding into a flake.
- **Read-your-own-writes, stated in PS-12's terms.** PS-12 requires an adapter to make batch reads
  reflect the batch's own pending writes *or* expose no read path on `Batch` at all, and it names
  Ladybug as its candidate falsifier (`spec/SPECIFICATION.md:5052-5075`). HS-S0079 produces the
  answer; this document records it as a supported behaviour or a stated capability limit, in the
  clause's vocabulary — not as a capability note that a reader has to translate.

**Nine clause ids are judged, and each carries its owner beside it.** PS-2 as the bar; the
batch-shape-and-write-seam row PS-4, PS-5, PS-6, PS-9, PS-11, PS-12, PS-15; and PS-34. All nine are
`projection-store-freeze`'s (`RUNBOOK.md:598`, `:602`, both marked "6, re-tested 11"). PS-3 is named
as a **tenth, non-judged data point** for `publication-and-positioning` (`RUNBOOK.md:601`, "6, decided
at 12"). **This story moves no marker** (`discover.md:44-48`): naming the owner beside each finding is
what lets a reader tell a data point from a decision.

**PS-2 is the bar, it is `[FROZEN]`, and it is read-only here.** It forbids freezing the port until
`CheckpointOnlyStore` has failed the suite *and* two adapters at opposite ends of the batch-shape axis
have passed, and its *Rejects* paragraph names the failure this whole project exists to prevent —
freezing against `MemoryProjectionStore` plus an in-process rusqlite transaction, *"the same storage
shape wearing two hats"* (`spec/SPECIFICATION.md:4760-4774`). The verdict is judged against that
clause and against the axes document, and against nothing else.

**The axes were fixed on disk first, and the verdict makes that visible in the artefact.** The verdict
cites `references/evaluation/phase-11-preflight-and-unlike-axes.md` **by commit SHA alongside its
own** (`discover.md:67-72`), so an inverted order is visible to a reader who never runs `git log`. If
the run showed an axis was wrong, the axes document is **superseded** under M9 and the verdict says so
— it is never edited to match what happened.

**A "did not hold" verdict already has its shape, decided before the result is known.** A new decision
atom and a re-plan (`project.md:109-111`), plus — per the runbook's own framing of the proof artefact
— a **superseding ADR** and a recorded **meta-finding that the two-implementation freeze rule should
have been three** (`RUNBOOK.md:4427-4430`). What it does **not** trigger is an edit: no `[FROZEN]`
clause marker or sentence moves, `cargo xtask spec-trace` stays green, and the proof is a reviewed
`git diff` against the project-start commit (`_decomposition.md:478`). Accepted decision atoms are
likewise superseded, never edited, and `redkiln validate --kb` enforces that against `HEAD`.

**`references/evaluation/` documents are superseded, never edited** (`references/evaluation/README.md:77-85`).
This is not advisory here. The named failure mode is precise (`discover.md:145-151`): the verdict
lands, `publication-and-positioning` reads it, and a later commit sharpens a sentence or softens a
declension — nothing in the gate objects, it is a markdown file, and the artefact a downstream
decision was taken against no longer exists.

**This story writes no design decision and settles no open question that is not its own.** ADR-0025's
Q3 (how `lbug`'s blocking API meets a non-blocking port) was answered in HS-S0075; the verdict
*reports* whether the chosen bridge survived contact with the suite — in particular whether any
conformance emitter (tokio, blocking, wasm) could not run the adapter as built, which is a finding
about the port's async posture rather than a footnote about this adapter (`discover.md:60-65`).

## Integration contract

- **Archetype**: `capability`. It is the project's terminal deliverable and the initiative's **DoD 8**
  artefact — the one thing this project cannot substitute (`project.md:226-239`, item 4). Not a
  double, not a placeholder.
- **Slice / milestone**: `freeze-verdict`. Slice-mate: **`verdict-ordering-and-publication-handoff`**
  (HS-S0083), which depends on this story, merges it ahead of `publication-and-positioning`'s gate,
  and hands HS-P0016 the build number, the docs.rs failure and the `[package.metadata.docs.rs]`
  question (`_storymap.md:49`). The two are implemented in one context and mounted as one surface.
- **Mount point**: **`references/evaluation/README.md`**. It is the file that assigns every document
  in that directory to a lifecycle — *immutable evidence, dated, pinned to a commit, superseded rather
  than edited* (`:1-13`, `:77-85`) versus *mutable speculation* versus the "later additions, which are
  neither" case it already carries (`:40-55`). The verdict is classified there **by name**, in the
  first lifecycle. This is a real mount and not a gesture: an unclassified evidence file is exactly
  the one a later contributor edits in place, which is the fourth named wrong implementation
  (`discover.md:145-151`) and the single move that destroys the artefact `publication-and-positioning`
  will have decided against.
- **Second, co-required mount**: **`RUNBOOK.md`'s phase 11 exit criteria (`:4432-4438`)**, ticked
  **in place** (`_decomposition.md:82`) — the plan of record is where the repository reads how far it
  has got. This story ticks every box not already ticked by an upstream story, and ticks a box
  discharged elsewhere only against that story's merged commit: *projection conformance green with
  capability skips reported* (HS-S0078/HS-S0079), *the verdict written down either way* (this story),
  *build cost measured and the CI decision recorded here* (HS-S0081, `:4437`), *`publish = false`
  removed* (HS-S0080). Untickable boxes stay unticked with the reason written beside them; a box
  ticked against work that did not merge is worse than an unticked one.
- **Wires into** (all read-only from this story; none is edited here):
  - `references/evaluation/phase-11-preflight-and-unlike-axes.md` — the axes the verdict is judged
    against, cited by commit SHA alongside the verdict's own
    (`preflight-and-unlike-axes/spec.md:199`; `discover.md:67-72`).
  - `crates/happenstance-ladybug/tests/` — the conformance target HS-S0078 mounted and HS-S0079
    extended; its rule list and its `--show-output` `SKIP` lines are the verdict's rule table
    (`_decomposition.md:474`; `xtask/src/main.rs:131-151`).
  - `xtask/src/proof.rs:133` — the `ARTEFACTS` registry row HS-S0078 added. The verdict names the
    target and test names that row holds the gate to, so "the suite ran" is checkable rather than
    claimed.
  - `spec/SPECIFICATION.md` — PS-2 (`:4760-4774`), PS-5 (`:4868-4883`), PS-11 (`:4977-5005`),
    PS-12 (`:5052-5075`), PS-34 (`:5546-5560`). **Cited, never amended.**
  - `crates/happenstance-ladybug/src/live_handle.rs:38-66` and
    `references/adapter-shapes.md:169-195`, `:186-191` — the ICE and E0195 transcripts T1's finding
    rests on, preserved outside the crate.
  - `crates/happenstance-ladybug/src/projection_store.rs:204-213` — the `WriteTransactionInUse`
    variant and LadybugDB's single-writer rule.
  - `.kb/open-questions/projection-store-batch-has-no-apply-seam.md` and
    `.kb/open-questions/cf-40-fixture-limits-ownership.md` — **consumed as evidence, not resolved
    here**; their resolution is `projection-store-freeze`'s (DR-5, DR-7).
  - `.kb/decisions/0008-one-derivation-for-both-ports.md` — the GAT-across-a-suspension-point finding
    the T1 and GAT/ICE paragraphs are written against.
- **Renders surfaces**: **none.** `_design.md` records `N/A — no user-facing surface` for every block
  and was approved on that determination (`_design.md:40-98`). This story ships no Rust public item,
  so no `## Items` row is claimed and none is owed.
- **Conformance rule(s)**: **none, and deliberately.** This story adds no rule, changes no port and
  edits nothing under `crates/happenstance-testkit/**` — which is explicitly not this project's
  (`_decomposition.md:86-88`). It *reports on* rules others registered. Its own falsifiable claims are
  checked by fixed-heading greps, by the row-count reconciliation, and by review, which is what the
  testing brief's AC-007 row already specifies: *"reviewed, not scripted"* (`_decomposition.md:477`).
- **Clause(s)**: **discharges none, amends none, moves no marker.** It judges nine (PS-2 as the bar;
  PS-4, PS-5, PS-6, PS-9, PS-11, PS-12, PS-15; PS-34) and hands PS-3 a data point. PS-34's marker is
  `projection-store-freeze`'s (`RUNBOOK.md:602`) and PS-3's is `publication-and-positioning`'s
  (`RUNBOOK.md:601`). If the verdict implies a marker should move, that routes through a decision atom
  and a re-plan, not an edit (`project.md:109-111`). `[FROZEN]` clause text and markers are untouched
  by construction: this PR's boundary contains no `spec/` path.
- **Advances DoD scenario**: initiative **DoD 8** — *"The freeze verdict is written. After the unlike
  batch shape exists, a written verdict states whether the `ProjectionStore` freeze held, either way,
  with what it was checked against"* (`initiative.md:380-382`). This story **is** that scenario, and
  turning it green is the whole of the PR. It also makes **DoD 7** judgeable rather than asserted —
  *"two structurally unlike batch shapes pass it"* (`initiative.md:377-379`) — by naming, against the
  axes, which two and on what axis they are unlike; DoD 7 itself is owned upstream.

## PR boundary

`redkiln verify --grain story` reads the first fenced block below and fails on any file changed
outside it.

```
references/evaluation/phase-11-freeze-verdict.md
references/evaluation/README.md
RUNBOOK.md
.kb/_intake/**
.bklg/from-contract-to-published-library/ladybug-projection-store/freeze-verdict-document/**
```

**In this PR**

- The verdict at `references/evaluation/phase-11-freeze-verdict.md`: dated, pinned to the commit the
  suite was run at, carrying the five DR-4 fields, the one-row-per-registered-rule table, the nine
  clause ids with their owners, the four findings no suite emits, and the literal word *held* or
  *did not hold*.
- The mount: `references/evaluation/README.md` gains the document **by name** under the
  immutable-evidence lifecycle, with one line saying what it is.
- The mount: `RUNBOOK.md`'s phase 11 exit-criteria boxes (`:4432-4438`) ticked in place, each against
  the story and commit that discharged it; any box that cannot be ticked left unticked with its
  reason.
- **Only on a *did not hold* verdict**: one staged note under `.kb/_intake/` carrying the routing —
  what did not hold, which clause, which alternative the superseding ADR must weigh, and the
  meta-finding that the two-implementation freeze rule should have been three
  (`RUNBOOK.md:4427-4430`).
- This story's own backlog folder (its ledger and implementation report).

**Explicitly not in this PR**

- **Any file under `crates/**` or `xtask/**`.** The adapter, the fixture, the conformance target and
  the two registry rows are HS-S0076 – HS-S0081's, all merged before this story starts. A verdict that
  edits the thing it is judging is not a verdict.
- **`spec/SPECIFICATION.md`.** No clause body, marker or citation moves anywhere in this project; the
  six citation repairs rode with the bodies at HS-S0077 (`_storymap.md:112`). This is the half of
  AC-008 this story owns, and it owns it by having no `spec/` path in the boundary at all.
- **`.kb/decisions/**`.** Atoms are authored by `/redkiln:kb-ingest` from `.kb/_intake/`, never by
  hand (`_decomposition.md:204-209`; `CLAUDE.md` names the reverted commit). The did-not-hold routing
  therefore stages an intake note and stops.
- **The re-plan itself.** A "did not hold" produces backlog work; creating it is `redkiln`'s and the
  human's, not a file in this PR.
- **The build-cost number's home.** It stays in `RUNBOOK.md`'s phase 11 body and HS-S0081's record;
  this document cites it (decision 2 above).
- **The merge-ordering claim against `publication-and-positioning`.** That is HS-S0083's
  (`_storymap.md:49`; project AC-011 at `_decomposition.md:481`). This story writes the artefact; the
  slice-mate states and proves the ordering.
- **Moving PS-34's or PS-3's marker**, editing an accepted atom, or amending the axes document in
  place. All three are supersessions if they are anything.

The implementer **may** additionally touch the mount files named in the Integration contract —
`references/evaluation/README.md` and `RUNBOOK.md` — because mounting this slice is the delivery, not
scope drift; both are already inside the boundary block above.

**Merge DoD**: the verdict exists, is dated, is pinned to the run commit, carries all five DR-4 fields
and the literal held/did-not-hold word, reconciles its rule-row count against
`projection-store-freeze`'s registration count, names all nine clause ids with their owners, carries
the four suite-invisible findings, cites the axes document's SHA beside its own, is classified in
`references/evaluation/README.md`, has ticked the phase 11 boxes it is entitled to tick — with
`cargo xtask ci` green (the whole gate, not `--fast`), which for a docs-only diff proves no regression
rather than proving anything this story claims.

## Behavior and interfaces

The document's **normative shape** is fixed here so the acceptance criteria can be commands rather
than opinions, and so the shape cannot be chosen after the news (Executive summary, decision 4).
Heading text is fixed; row content is the implementer's:

```
# Phase 11 — the ProjectionStore freeze verdict
   (front matter: Date · Implementation · Run commit · Axes document + its commit · CI shape)
## Verdict            -> exactly one line, exactly one of: `held` / `did not hold`
## Rules run          -> table: Rule | Outcome | Reason (required when Outcome is `declined`)
## Clause ledger      -> table: Clause | What the run showed | Owner
## Findings the suite cannot emit
                      -> T1 · GAT/ICE · WriteTransactionInUse · read-your-own-writes (PS-12)
## What this was checked against
## If it did not hold -> the routing; present and answered on both branches
```

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The document exists on both branches, and its shape is the same on both** | The artefact is unconditional (`project.md:206-209`). Every heading above is present whichever way the run went; the `## If it did not hold` section is *answered* on a green run ("not triggered; the routing that would have applied is …"), never deleted. A template with a Held section filled in and a Did-not-hold heading left empty satisfies "the document exists" and is the first named wrong implementation. | `_storymap.md:24`, `:26-30`; `discover.md:105-115` |
| **Five fields, named, in the front matter** | Which implementation (`LadybugProjectionStore`, crate version, `lbug` version), which rules (the `## Rules run` table), which PS clause ids (the `## Clause ledger`), at which commit (a full SHA of the tree the suite was run at, not a branch name), on which date. A verdict missing one fails regardless of how it reads. | `project.md:149-151` (DR-4); `_decomposition.md:477` |
| **The verdict word is a literal, on its own line, under a fixed heading** | `held` or `did not hold`. Not "largely held", not "held with caveats" — a caveat is a row in `## Findings the suite cannot emit`, and a verdict that needs a qualifier to be true is a `did not hold` with an explanation. | `project.md:206-209`; `RUNBOOK.md:4435-4436` (*"it held" is a result and must be recorded as one*) |
| **One row per registered rule, and the count reconciles** | The `## Rules run` table has exactly as many body rows as `projection-store-freeze` registered rules, diffed against that project's registration list rather than counted by eye — the same reconciliation AC-004 requires of the run. A rule absent from the table is indistinguishable from a rule that passed, which is the exact defect CF-18 forbids of the run and this table inherits. | `_decomposition.md:474`; `spec/SPECIFICATION.md:5052-5075` (PS-12's reported-skip requirement); `xtask/src/proof.rs:133` |
| **Three outcomes, and `declined` is not `passed`** | The outcome vocabulary is closed: `passed`, `declined — <the fixture's stated reason, quoted>`, `failed`. A `declined` row with an empty or generic reason fails the criterion; the reason is the fixture's own text as printed under `--show-output`, not a paraphrase. This is the structural guard against the most damaging mutant — the verdict that lists what passed and not what declined. | `discover.md:117-130`; `_decomposition.md:148-154` (M2); `xtask/src/main.rs:131-151` |
| **Nine clause ids, each with what the run showed and who owns it** | PS-2 (the bar), PS-4, PS-5, PS-6, PS-9, PS-11, PS-12, PS-15, PS-34 — all nine owned by `projection-store-freeze` and all marked "6, re-tested 11". PS-3 appears as a tenth row explicitly labelled *data point*, owner `publication-and-positioning`. The owner column is what lets a reader tell a data point from a decision; without it the verdict reads as if it moved markers it has no right to move. | `project.md:48-56`; `RUNBOOK.md:598`, `:601`, `:602`; `discover.md:44-48` |
| **PS-2 is the yardstick and is quoted, not paraphrased** | The `## What this was checked against` section quotes PS-2's requirement — `CheckpointOnlyStore` failed, plus two adapters at opposite ends of the batch-shape axis passed — and states, against the axes document, which two adapters and on which axis. A verdict that asserts "unlike" without naming the axis is unfalsifiable. | `spec/SPECIFICATION.md:4760-4774`; `references/evaluation/phase-11-preflight-and-unlike-axes.md` |
| **T1 is carried as a finding: the freeze deleted its own counter-example** | `LiveHandleProjectionStore` cannot exist under a frozen `type Batch;`, and it was evidence — the borrowed GAT compiled and did not fail the `Send` flavour across a real `tokio::spawn`, a result that cuts against §4.2's argument. The verdict states that the freeze consumed it, that the transcripts survive outside the crate, and that the escalation already went upstream at HS-S0074 rather than repeating it as a second note. | `_decomposition.md:314-331`; `references/adapter-shapes.md:169-195`; `discover.md:24` |
| **The GAT/ICE finding, conditional on what actually merged** | If the lifetime survived phase 6, the verdict records that the port is implementable only by stores that outlive every batch lifetime and that the failure mode for anything else is a **compiler panic rather than a diagnostic**, citing the transcript. If `type Batch;` landed, it records the relief as *observed* and notes that binding an owned type to the GAT bought none of it — so "the trap is retired" is not a vacuous claim. | `project.md:272-277`; `crates/happenstance-ladybug/src/live_handle.rs:38-66`; `references/adapter-shapes.md:186-191`; `_decomposition.md:332-359` (T2) |
| **`WriteTransactionInUse` is answered here, not deferred** | One of two answers, each with its consequence stated: a **declared capability limit** (the suite is right; this adapter's concurrency envelope is narrower than a SQL adapter's) or a **rule defect** (the rule assumed a concurrency model the port never promised; routed to HS-P0010 by name). "Retry until green" is excluded in advance — it converts a design finding into a flake. If no concurrency-shaped rule met it, the verdict says *that*, which is itself a finding about the suite's reach. | `project.md:278-283`; `crates/happenstance-ladybug/src/projection_store.rs:204-213`; `discover.md:50-58` |
| **Read-your-own-writes is recorded in PS-12's vocabulary** | HS-S0079's result is written as *reads through an open `Batch` reflect that batch's pending writes*, or *no read path is exposed on `Batch` at all* — PS-12's two permitted shapes — and never as *"a read path answering from committed state"*, which the clause forbids and calls the natural, silently-losing shape. Ladybug is PS-12's named candidate falsifier, so this row is the clause's own test result. | `spec/SPECIFICATION.md:5052-5075`; `project.md:198-201` (AC-005); `crates/happenstance-ladybug/src/projection_store.rs:49-64` |
| **The blocking bridge's contact with the harness is reported** | Whether ADR-0025 Q3's chosen bridge let every conformance emitter (tokio, blocking, wasm) run the adapter as built, or narrowed the harness. A bridge that narrowed it is a finding about the port's async posture, not a footnote about this adapter. ADR-0025 is HS-S0075's; this row reports on it and re-decides nothing. | `discover.md:60-65`; `_decomposition.md:293-311`; `.kb/decisions/0001-async-port-flavours.md` |
| **The axes SHA is cited beside the verdict's own** | Both SHAs appear in the front matter, so a reader who never runs `git log` can see the ordering AC-001 claims. If the run showed an axis was wrong, the axes document is **superseded** by a new dated document and the verdict says so by name — the axes file is never edited to match what happened. | `discover.md:67-72`; `references/evaluation/README.md:77-85`; `preflight-and-unlike-axes/spec.md:199` |
| **"Did not hold" routes; it does not edit** | The routing is: a note staged under `.kb/_intake/` for `/redkiln:kb-ingest` to make an atom of, a re-plan, a **superseding** ADR, and the recorded meta-finding that the two-implementation freeze rule should have been three. Nothing `[FROZEN]` moves, no accepted atom is edited, no marker is touched. | `project.md:109-111`, `:210-213`; `RUNBOOK.md:4427-4430`; `_decomposition.md:204-209`; `discover.md:33-42` |
| **Nothing `[FROZEN]` moved, and the proof is a diff** | `cargo xtask spec-trace` green, plus a reviewed `git diff <project-start-commit>..HEAD -- spec/SPECIFICATION.md` confirming no clause marker or sentence differs. This story's own contribution to that proof is a PR boundary containing no `spec/` path, which makes the claim structural rather than promised. | `project.md:210-213`; `_decomposition.md:478`; `xtask/src/spec_trace.rs:291-372` |
| **Both mounts, or it is a file rather than evidence** | `references/evaluation/README.md` classifies the document by name under the immutable-evidence lifecycle; `RUNBOOK.md:4432-4438`'s boxes are ticked in place, each against the merged story that discharged it, with untickable boxes left unticked and reasoned. | `_decomposition.md:211-218` (M9), `:82`; `references/evaluation/README.md:1-13`, `:40-55` |
| **Superseded, never edited** | A correction to this document is a **new** dated, commit-pinned document naming the one it supersedes. Amending in place would destroy the artefact `publication-and-positioning` decided against — the fourth named wrong implementation, and the one the gate cannot object to because it is markdown. | `references/evaluation/README.md:77-85`; `discover.md:145-151` |
| **The build cost is cited, not restated** | The verdict names the CI shape the run happened under — shared three-OS gate, or `--exclude` plus a dedicated job — because that is part of what it was checked against, and points at HS-S0081's number in `RUNBOOK.md`'s phase 11 body rather than copying it. | `RUNBOOK.md:4437`; `_decomposition.md:375-406`, `:479` |
| **E2E-19 and E2E-24's third-shape halves get a stated status** | Writable now, or still not, with the reason — the runbook lists them as the cases phase 11 makes writable, and leaving it unstated hands HS-P0016 an unanswered question it will discover late. Writing the cases is not this story's. | `RUNBOOK.md:4440`; `spec/E2E-CASES.md`; `_decomposition.md:429-431` |
| **Interfaces changed** | **None.** No Rust item is added, removed, renamed or re-typed; no feature, no dependency, no public surface. `cargo xtask ci` on this diff is a no-regression check, and every claim this story makes lives outside anything the compiler can see. | `_design.md:40-98`; PR boundary above |

## Data and migrations

**N/A.** This story adds no schema, no persisted format, no serialized envelope, no stored state and
no migration. It ships one markdown document plus two mount edits, and `references/` binds nothing:
its README says so directly — the material there is evidence kept for citation whose contents lose to
`spec/SPECIFICATION.md` wherever they disagree (`references/evaluation/README.md:87-92`; `CLAUDE.md`,
repository map).

Two adjacent things that *are* data are deliberately not this story's. The LadybugDB on-disk graph
schema — label and property names, indexes, and whether the checkpoint node is per-`ProjectionId` or
one node with a property per id — is ADR-0025 Q1's, with PS-23 owned by `projection-store-freeze`
(`_decomposition.md:265-274`; `spec/SPECIFICATION.md:5317`). And the checkpoint's `INT64` ↔
`NonZeroU64` narrowing, with `MalformedCheckpoint` and `PositionOutOfRange`, was implemented and
unit-tested at HS-S0077 (`crates/happenstance-ladybug/src/projection_store.rs:175-202`); this document
reports its behaviour under the suite and changes none of it.

## Acceptance criteria

Each criterion is written from the intent of a named actor. Two of the initiative's four personas
reach this artefact and neither is served by a green CI badge (`initiative.md:227-252`): the
**adapter author** on the *learn when you are finished* journey, who has just run the suite against
the third shape and needs to know what the run actually settled; and the **evaluator** on the
*decide in one sitting* journey, who has minutes, no checkout, and must be able to tell a frozen
port from a port frozen against a monoculture. The reviewer holding the freeze
(`_storymap.md:14-16`) is the third reader and the strictest.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** the projection suite has been run against `LadybugFixture` and the result is known, **WHEN** the evaluator opens `references/evaluation/` at the tip of the initiative branch, **THEN** `phase-11-freeze-verdict.md` is present with all seven fixed headings from *Behavior and interfaces* — `## Verdict`, `## Rules run`, `## Clause ledger`, `## Findings the suite cannot emit`, `## What this was checked against`, `## If it did not hold` — and `## If it did not hold` is **answered** on a green run ("not triggered; the routing that would have applied is …") rather than deleted or left empty, **SO THAT** the artefact's existence and shape carry no information about which way the run went, which is the whole of its evidentiary value. | Static: every heading above is present in the file, and the `## If it did not hold` section is non-empty, in the reviewed `git diff` of this PR. Process: `git log --follow` shows the shape was fixed by this spec (a file predating the first conformance run) and only its values written after. `_storymap.md:26-30`; `discover.md:105-115` |
| AC-002 | **GIVEN** the evaluator will not clone the repository, **WHEN** they read the verdict's front matter and its `## Verdict` line, **THEN** they find all five DR-4 fields — which implementation (`LadybugProjectionStore`, crate version, `lbug` version), which rules (the `## Rules run` table), which PS clause ids (the `## Clause ledger`), at which commit (a **full SHA**, never a branch name or an abbreviation), on which date — plus the axes document's own SHA and the CI shape the run happened under, with the cold-build **number cited to `RUNBOOK.md`'s phase 11 body rather than copied**; and the `## Verdict` heading is followed by exactly one line reading exactly `held` or exactly `did not hold`, **SO THAT** the verdict can be judged, and its ordering against the axes seen, without running `git log` or trusting a paraphrase. | Static: the five fields are greppable by name; `git cat-file -e <run-sha>` and `git cat-file -e <axes-sha>` both resolve; the `## Verdict` body matches one of the two literals and nothing else. Review: no qualifier ("largely held", "held with caveats") appears — a caveat is a `## Findings` row. `project.md:149-151`; `RUNBOOK.md:4435-4436`; `_decomposition.md:479` |
| AC-003 | **GIVEN** the adapter author knows a freeze fails quietly — as a rule reported as a fixture limitation, not as a red test — **WHEN** they read `## Rules run`, **THEN** the table carries **one body row per rule `projection-store-freeze` registered**, with its row count reconciled against that project's registration list at the run commit rather than counted by eye, and each row's outcome drawn from a closed vocabulary of three — `passed`, `declined — <the fixture's stated reason, quoted from the `--show-output` `SKIP` line>`, `failed` — with a `declined` row carrying a paraphrased, generic or empty reason failing the criterion, **SO THAT** a rule that declined is impossible to confuse with a rule that passed and impossible to omit. | Reconciliation: table body-row count diffed against `projection-store-freeze`'s registration list; each `declined` reason string matched against the run's captured `--show-output` transcript. Reviewed, not scripted (`_decomposition.md:477`). `_decomposition.md:474`; `xtask/src/main.rs:131-151`; `xtask/src/proof.rs:133`; `discover.md:117-130` |
| AC-004 | **GIVEN** the reviewer must tell a data point from a decision, **WHEN** they read `## Clause ledger` and `## What this was checked against`, **THEN** nine clause ids appear — PS-2 as the bar, plus PS-4, PS-5, PS-6, PS-9, PS-11, PS-12, PS-15 and PS-34 — each with what the run showed **and its owner beside it** (`projection-store-freeze` for all nine), PS-3 appears as a tenth row explicitly labelled *data point* owned by `publication-and-positioning`, and PS-2's requirement is **quoted** — `CheckpointOnlyStore` failed, plus two adapters at opposite ends of the batch-shape axis passed — and answered by naming, against the axes document, **which two adapters and on which axis they are unlike**, **SO THAT** "the freeze held" is falsifiable against the clause that set the bar rather than against a green run. | Static: all ten ids present; PS-2's quoted text matched against `spec/SPECIFICATION.md:4760-4774`. Review: each row names an owner, and the unlike-axis claim resolves to a named axis in the axes document rather than the word "unlike". `project.md:48-56`; `RUNBOOK.md:598`, `:601`, `:602`; `discover.md:44-48` |
| AC-005 | **GIVEN** a green suite is structurally incapable of emitting a finding about the port, **WHEN** the adapter author reads `## Findings the suite cannot emit`, **THEN** T1 is stated — the freeze **deleted the counter-example that was evidence for freezing**: `LiveHandleProjectionStore` binds a genuinely borrowed handle with no lifetime-free spelling and cannot exist under a frozen `type Batch;`, and its borrowed GAT had compiled and **not** failed the `Send` flavour across a real `tokio::spawn`, a result that cuts against §4.2 — together with the fact that both transcripts survive outside the crate and that the escalation already went upstream at HS-S0074 rather than being re-raised here; and the GAT/ICE finding is recorded on whichever branch merged (survived: the port is implementable only by stores outliving every batch lifetime and the failure mode is a **compiler panic, not a diagnostic**; retired: recorded as *observed*, noting that binding an owned type to the GAT bought none of that relief), **SO THAT** the freeze's own cost is on the record where nothing else would put it. | Review against the transcripts: `references/adapter-shapes.md:169-195` and `:186-191`, `crates/happenstance-ladybug/src/live_handle.rs:38-66`. AC-A04's bar — each of T1/T2/T3 honoured as decided or its deviation written here (`_decomposition.md:51-54`, `:314-331`, `:332-359`). Reviewed, not scripted (`_decomposition.md:477`). |
| AC-006 | **GIVEN** the reviewer will treat any unanswered question as an answer of "we did not look", **WHEN** they read the remaining `## Findings` rows, **THEN** four questions are **answered rather than reported**: `WriteTransactionInUse` under a concurrency-shaped rule is either a **declared capability limit** (the suite is right; this adapter's concurrency envelope is narrower than a SQL adapter's) or a **rule defect** routed to HS-P0010 by name — never "retry until green", and if no concurrency-shaped rule met it, that absence is itself stated as a finding about the suite's reach; read-your-own-writes is written in **PS-12's own vocabulary** (reads through an open `Batch` reflect that batch's pending writes, *or* no read path is exposed on `Batch` at all) and never as "a read path answering from committed state"; ADR-0025 Q3's blocking bridge is reported as having let every conformance emitter (tokio, blocking, wasm) run the adapter as built, or as having narrowed the harness; and E2E-19's and E2E-24's third-shape halves carry a stated status — writable now, or not, with the reason, **SO THAT** the next project inherits answers instead of discovering the questions late. | Review, per the AC-007 row's "reviewed, not scripted" (`_decomposition.md:477`). Cross-checked against `crates/happenstance-ladybug/src/projection_store.rs:204-213` and `:49-64`, `spec/SPECIFICATION.md:5052-5075` (PS-12's two permitted shapes and its forbidden one), `RUNBOOK.md:4440`, `spec/E2E-CASES.md`. `project.md:278-283`; `discover.md:50-58`, `:60-65` |
| AC-007 | **GIVEN** an unclassified markdown file in `references/evaluation/` is exactly the file a later contributor edits in place, and **GIVEN** the repository reads how far it has got from `RUNBOOK.md`, **WHEN** this PR merges, **THEN** `references/evaluation/README.md` classifies `phase-11-freeze-verdict.md` **by name** under the immutable-evidence lifecycle with one line saying what it is — making explicit that a correction is a **new dated, commit-pinned document naming the one it supersedes**, never an in-place amendment — and `RUNBOOK.md:4432-4438`'s phase 11 exit-criteria boxes are ticked **in place**, each against the merged story and commit that discharged it, with any box that cannot be ticked left unticked and its reason written beside it, **SO THAT** the document is evidence rather than a file, and phase 11 is closable by someone who did not do the work. | Static: the filename appears in `references/evaluation/README.md` under the lifecycle described at `:1-13`, `:77-85`; the `RUNBOOK.md:4432-4438` block's boxes are ticked or reasoned in the reviewed diff. Review: every ticked box resolves to a commit that is an ancestor of `HEAD`. `_decomposition.md:211-218`, `:82`; `project.md` Definition of done items 4 and 6 |
| AC-008 | **GIVEN** the response to bad news must be a new atom and never an edit, **WHEN** the reviewer diffs this PR and, on a *did not hold* verdict, follows the routing, **THEN** no path under `spec/` appears in the diff at all, `cargo xtask spec-trace` is green, and `git diff <project-start-commit>..HEAD -- spec/SPECIFICATION.md` shows no `[FROZEN]` clause marker or sentence differing from its state at the project's start; and a *did not hold* stages exactly **one note under `.kb/_intake/`** carrying what did not hold, which clause, which alternative the superseding ADR must weigh, and the meta-finding that the two-implementation freeze rule should have been three — with **nothing written under `.kb/decisions/` by hand**, no accepted atom edited, no clause marker moved and the re-plan left as backlog work, **SO THAT** a freeze that did not hold is heard before publication opens without the record it disagrees with being quietly rewritten. | Gate: `cargo xtask spec-trace` (part of `cargo xtask ci`; `xtask/src/spec_trace.rs:291-372`). Static: `git diff --name-only` on this PR contains no `spec/` path and no `.kb/decisions/` path. Review: the reviewed `git diff <project-start-commit>..HEAD -- spec/SPECIFICATION.md` (`_decomposition.md:478`). `redkiln validate --kb && redkiln doctor` clean with the intake note staged. `project.md:109-111`, `:210-213`; `RUNBOOK.md:4427-4430` |

**Coverage of the traced project ACs.** Project **AC-007** (`project.md:206-209` — the verdict exists,
either way, naming implementation, rules, clause ids and commit) is carried by AC-001 (unconditional
existence and identical shape), AC-002 (the five fields and the literal word), AC-003 (the rules),
AC-004 (the clause ids) and AC-005 + AC-006 (the content that makes it a verdict rather than a
receipt), and mounted by AC-007. Project **AC-008** (`project.md:210-213`) is carried by AC-008 in
both halves it owns here — nothing `[FROZEN]` moved, and the "did not hold" routing. The six citation
repairs, AC-008's remaining half, are `fill-the-bodies-and-ps-34-disposition`'s (`_storymap.md:112`).

## Interaction quality

This story **renders no user-facing surface**. `_design.md` records `N/A — no user-facing surface`
for every block and was approved on that determination (`_design.md:40-98`), so there is no signed-off
composition, transience policy or density budget to inherit and none is invented here. What this story
does ship is a *read* artefact with a real reader and a real navigation path, so the invariants below
are the ones that genuinely apply, taken from the genre's own standing rules
(`references/evaluation/README.md:1-13`, `:77-85`) and from the mount points at `_decomposition.md:211-218`.

Every invariant that applies is already a row in the acceptance-criteria table above. Nothing is
introduced here as a prose bullet, because a bullet in this section gets no ledger row and is never
gated.

**State family**

| Invariant | Applies? | Carried by | How verified |
| --- | --- | --- | --- |
| In-place vs context-jump | Yes | **AC-007** | The reader reaches the verdict from `references/evaluation/README.md`'s classification list by name, not by directory listing or search; and `RUNBOOK.md`'s phase 11 boxes are ticked **in place** rather than answered in a new section elsewhere (`_decomposition.md:82`). |
| Non-occlusion | Yes | **AC-007** | Ticking a box never overwrites, re-words or unticks a box an upstream story discharged; untickable boxes stay visible and unticked with the reason beside them rather than being removed. |
| Reversibility | Yes | **AC-007**, **AC-008** | The only permitted correction to this document — and to the axes document, and to any accepted decision atom — is **supersession**: a new dated, commit-pinned artefact naming the one it replaces. In-place amendment is the fourth named wrong implementation and the one the gate cannot object to (`discover.md:145-151`). |
| Preserved focus / scroll / selection | N/A | — | No interactive surface; there is no selection or scroll state to preserve. |
| Keyboard reachability | N/A | — | No control surface; the artefact is a static markdown document in a git tree. |

**Composition family** — inherited from the evidence-document genre rather than from `_design.md`,
which declares none.

| Invariant | Applies? | Carried by | How verified |
| --- | --- | --- | --- |
| Presentation exists at all | Yes | **AC-001**, **AC-002** | The verdict is a *composed* document with a fixed heading skeleton and named front-matter fields — not a paragraph of prose that happens to contain the facts. A reader must be able to find the verdict word, the run commit and the declined rules by position, not by reading. |
| Composition / placement | Yes | **AC-001** | The seven headings appear in the fixed order given in *Behavior and interfaces*: verdict first, then the machine evidence (rules, clauses), then the human evidence (findings), then what it was judged against, then the routing. A verdict whose conclusion is buried after its evidence is a report, not a verdict. |
| Transience | Yes | **AC-001** | `## If it did not hold` is **persistent chrome, not revealed content**: it is present and answered on both branches. A heading that appears only when triggered makes the document's shape carry the result, which AC-001 exists to forbid. |
| Density budget | Yes | **AC-003**, **AC-004** | The real numbers: exactly **one row per registered rule** (count reconciled, not eyeballed), exactly **three** permitted outcome values, exactly **nine** judged clause ids plus **one** labelled data point, exactly **five** DR-4 front-matter fields, exactly **two** commit SHAs, exactly **one** verdict line. Each is a hard count; a table shorter than its registration list fails AC-003 regardless of how it reads. |
| Hierarchy | Yes | **AC-002**, **AC-004** | The verdict word outranks everything: one line, its own heading, a closed two-value vocabulary. Beneath it, ownership is explicit — every clause row names its owner, so a data point can never be read as a decision this story had no right to take. |
| Named anti-patterns | Yes | **AC-001**, **AC-003**, **AC-005**, **AC-008** | The four from `discover.md:103-151`, each with a structural guard rather than a diligence request: the verdict written only when the freeze held (AC-001); the verdict listing what passed and not what declined (AC-003); the verdict treating a green suite as the whole answer (AC-005, AC-006); the verdict improved after the fact (AC-007's lifecycle classification, AC-008's supersede-never-edit). |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | The run commit is recorded as a branch name, a tag, an abbreviated SHA, or a SHA that does not resolve in this repository. | Fails AC-002. The field is a full SHA that `git cat-file -e` resolves; a branch name moves and a moved pin destroys the artefact's meaning (`project.md:149-151`). |
| **EC-002** | A `## Rules run` row has outcome `declined` with an empty, generic ("fixture limitation") or paraphrased reason. | Fails AC-003. The reason is the fixture's own text as printed under `--show-output` (`xtask/src/main.rs:131-151`); an empty reason on a `Capability` is already a codegen-tier failure upstream (`_decomposition.md:474`), and a paraphrase here re-introduces exactly what that catches. |
| **EC-003** | The `## Rules run` body-row count does not equal `projection-store-freeze`'s registered rule count at the run commit. | Fails AC-003. Neither number is adjusted to match the other: the discrepancy is itself the finding and is written into `## Findings the suite cannot emit`, then reconciled — a rule absent from the table is indistinguishable from a rule that passed. |
| **EC-004** | The `## Verdict` line reads anything other than the two literals — "held with caveats", "largely held", "held (see findings)". | Fails AC-002. A verdict needing a qualifier to be true is a `did not hold` with an explanation; the caveat becomes a `## Findings` row and the line takes one of the two literals. |
| **EC-005** | A `RUNBOOK.md` phase 11 box is ticked against a story whose commit is not an ancestor of `HEAD`, or against work merged to a different branch. | Fails AC-007. Leave it unticked with the reason beside it — a box ticked against work that did not merge is worse than an unticked one (`_decomposition.md:82`). |
| **EC-006** | The "did not hold" routing is written directly into `.kb/decisions/`, or an existing accepted atom is edited to reflect it. | Fails AC-008. Atoms are authored by `/redkiln:kb-ingest` from `.kb/_intake/` and accepted atoms are superseded, never edited; `redkiln validate --kb` enforces the second against `HEAD` (`_decomposition.md:204-209`; `CLAUDE.md`, *Where the work lives*). |
| **EC-007** | The run shows one of the axes was wrong, and the axes document is edited to match. | Fails AC-004 and AC-008's reversibility half. The axes document is **superseded** by a new dated document under M9 and the verdict says so by name; the original stays exactly as merged (`discover.md:67-72`; `references/evaluation/README.md:77-85`). |
| **EC-008** | Any path under `spec/` appears in this PR's diff, for any reason including a citation line-number repair. | Fails AC-008. Citation repairs were `fill-the-bodies-and-ps-34-disposition`'s and rode with the bodies (`_storymap.md:112`); this story's contribution to "nothing `[FROZEN]` moved" is structural — no `spec/` path in the boundary. |
| **EC-009** | A concurrency-shaped rule fails intermittently and a retry, a sleep or a re-run makes it green. | Fails AC-006 outright. "Retry until green" is excluded in advance — it converts a design finding into a flake. The finding is written as a declared capability limit or a rule defect routed to HS-P0010 (`project.md:278-283`). |
| **EC-010** | One of the three upstream stories has not merged when the verdict is written, so the run commit predates part of what the verdict describes. | Do not write a provisional verdict. This story is *last and unconditional*; the run commit must be a tree in which all three dependencies are present (`_storymap.md:83-84`). A verdict pinned to a commit that does not contain the work it describes is unfalsifiable. |

## Non-functional

| id | Requirement | Why, and how it is met |
| --- | --- | --- |
| **NF-001** | **No interface change, and the whole gate green.** `cargo xtask ci` (not `--fast`) passes on this PR's tree. | The project's bar is the whole gate (`_decomposition.md:517-539`). For a docs-only diff the gate proves *no regression* rather than proving anything this story claims — say so in the implementation report rather than presenting green CI as evidence for the verdict, which is the failure DR-4 names. |
| **NF-002** | **Every fact has exactly one home.** The cold-build number lives in `RUNBOOK.md`'s phase 11 body and HS-S0081's record and is cited here; T1's finding text is stated in the verdict alone and cited from HS-S0074's document by SHA. | A number copied into two documents is a number that will one day disagree with itself, and a finding kept in two places is a finding that drifts — this one has to read identically wherever it is read (Executive summary, decisions 2 and 3). |
| **NF-003** | **Judgeable without a checkout.** Every claim in the verdict is either cited to a `path:line` or verifiable by a single `git show <sha>`; no claim rests on output that exists only in a CI log. | The evaluator's journey is a bounded look at public evidence ending in adopt or decline (`initiative.md:246-250`). A verdict whose evidence is unreachable is a verdict the evaluator must take on trust. |
| **NF-004** | **Re-takeable.** The verdict records the exact command that produced the run and the shape of its expected output, so a third party who did not write the adapter can re-take the snapshot. | DR-9 (`project.md:171-174`): *"passed against N adapters" is a snapshot and the snapshot has to be re-takeable*. |
| **NF-005** | **Immutable after merge.** Once merged, the document is amended only by supersession, and its date and commit pin travel with it. | `references/evaluation/README.md:77-85`. This is what makes it safe for `publication-and-positioning` to decide against, and it is not protected by any gate — which is precisely why AC-007 mounts the classification. |
| **NF-006** | **Non-binding by construction.** Nothing in this document constrains behaviour: where it and `spec/SPECIFICATION.md` disagree, the clause wins. | `references/evaluation/README.md:87-92`; `CLAUDE.md`, repository map — `references/` keeps evidence for citation, binding nothing. A verdict that reads as normative invites the marker moves AC-008 forbids. |

## Implementation notes (non-prescriptive)

Nothing here is binding; the acceptance criteria are.

- **Write the skeleton first, from this spec, before reading the run output.** The heading list, the
  table columns, the nine clause ids and the five front-matter field names are all fixed above. Filling
  a fixed shape is what keeps the document from being shaped by the news; drafting freehand after the
  run satisfies every field and still fails AC-001's intent (`discover.md:105-115`).
- **Get the rule table from the transcript, not from memory.** Capture the conformance run's
  `--show-output` text once, keep it beside you, and transcribe `SKIP` reasons verbatim. The three
  outcome values are a closed vocabulary precisely so this is mechanical.
- **Reconcile the row count against `projection-store-freeze`'s registration list at the run commit,
  not at `HEAD`.** The list lives in another project and can move; a reconciliation against a moving
  target is not a reconciliation. Record which commit of that list you diffed against.
- **Write the `## If it did not hold` section on the green branch too.** The easiest way to satisfy
  AC-001 honestly is to write that section *before* you know the result and then, if the run was
  green, add one sentence recording that it was not triggered.
- **Order the RUNBOOK ticks last.** Each box needs a merged commit beside it; do that after the three
  dependencies are merged and their SHAs are stable, and leave anything you cannot evidence unticked
  with a reason. Two of the four boxes are not yours (`RUNBOOK.md:4437` is HS-S0081's, `publish = false`
  is HS-S0080's) — tick them only against their merged commits.
- **The `.kb/_intake/` note, if it is needed, is a note and not an atom.** Prose, no `KbFrontmatter`
  guesswork, saying what did not hold, which clause, which alternative the superseding ADR must weigh,
  and the meta-finding. `/redkiln:kb-ingest` turns it into an atom in its own run
  (`_decomposition.md:204-209`).
- **Slice-mate coordination.** HS-S0083 states and proves the merge ordering against
  `publication-and-positioning` and carries the handover of the build number, the docs.rs failure and
  the `[package.metadata.docs.rs]` question. Leave all four out of this document beyond the citation
  AC-002 requires (`_storymap.md:49`).
- **If the verdict implies a marker should move**, stop and route it: a decision atom and a re-plan
  (`project.md:109-111`). Do not open `spec/SPECIFICATION.md`; the PR boundary contains no `spec/` path
  on purpose, and that structural fact is half of what AC-008 proves.

## Tests and CI (merge gate)

The testing brief classifies this story's evidence as **static / process**, *"reviewed, not
scripted"* (`_decomposition.md:477`), and puts the whole-gate run behind it as a no-regression check
(`_decomposition.md:517-539`). Nothing below is a new test-framework assertion, because this story
adds no code.

| Tier | Command / path | Proves |
| --- | --- | --- |
| **Static — structure** | Reviewed `git diff`: `references/evaluation/phase-11-freeze-verdict.md` carries all seven fixed headings, `## If it did not hold` is non-empty, `## Verdict`'s body is one of two literals | AC-001, AC-002, EC-004 |
| **Static — pins** | `git cat-file -e <run-sha>` and `git cat-file -e <axes-sha>`; both SHAs full, both resolving; the cold-build number appears nowhere in the document body | AC-002, EC-001, NF-002 |
| **Reconciliation** | `## Rules run` body-row count diffed against `projection-store-freeze`'s registration list **at the run commit**; each `declined` reason matched verbatim against the captured `--show-output` transcript (`xtask/src/main.rs:131-151`, the flag that makes `SKIP` lines visible; `xtask/src/proof.rs:133`, the registry row holding the target to the gate) | AC-003, EC-002, EC-003 |
| **Static — clauses** | Ten ids present (`PS-2`, `PS-4`, `PS-5`, `PS-6`, `PS-9`, `PS-11`, `PS-12`, `PS-15`, `PS-34`, plus `PS-3` labelled *data point*), each with an owner column; PS-2's quoted text matched against `spec/SPECIFICATION.md:4760-4774` | AC-004 |
| **Review — findings** | Read against `references/adapter-shapes.md:169-195`, `:186-191`, `crates/happenstance-ladybug/src/live_handle.rs:38-66`, `crates/happenstance-ladybug/src/projection_store.rs:204-213`, `:49-64`, `spec/SPECIFICATION.md:5052-5075`, `RUNBOOK.md:4440` — each of T1, GAT/ICE, `WriteTransactionInUse`, read-your-own-writes, the blocking bridge and the E2E third-shape status has a named home and an answer | AC-005, AC-006, EC-009 |
| **Static — mounts** | `references/evaluation/README.md` names the file under the immutable-evidence lifecycle (`:1-13`, `:77-85`); `RUNBOOK.md:4432-4438`'s boxes ticked or reasoned, every ticked box's commit an ancestor of `HEAD` | AC-007, EC-005 |
| **Gate — nothing frozen moved** | `cargo xtask spec-trace` green (also inside `cargo xtask ci`; `xtask/src/spec_trace.rs:291-372`); `git diff --name-only` on this PR contains no `spec/` and no `.kb/decisions/` path; reviewed `git diff <project-start-commit>..HEAD -- spec/SPECIFICATION.md` shows no clause marker or sentence changed (`_decomposition.md:478`) | AC-008, EC-006, EC-007, EC-008 |
| **Gate — knowledge base** | `redkiln validate --kb && redkiln doctor` clean with any `.kb/_intake/` note staged | AC-008 |
| **Gate — whole** | `cargo xtask ci` (**not** `--fast`; `_decomposition.md:517-539` explains why this project has no `--fast` bar) | NF-001 — no regression from a docs-only diff |
| **Gate — backlog** | `redkiln verify --grain story` against this story's `_ledger.md` and the PR boundary block above | Every AC row satisfied with cited evidence; no file changed outside the boundary |

## Risks and coupling (PR-scoped)

- **A green run erodes the "either way" property, silently.** The most likely way this story fails is
  that the freeze holds, the routing section feels hypothetical, and it is dropped or left as an empty
  heading — which is the first named wrong implementation and the one no gate can see
  (`discover.md:105-115`). Mitigation is AC-001's requirement that the section be *answered*, plus
  writing it before the result is known.
- **The document is markdown and the gate cannot protect it.** After merge, a later commit can sharpen
  a sentence or soften a declension and nothing objects. AC-007's classification is the only guard, and
  it is a social one; the residual risk is real and is worth naming in the implementation report
  (`discover.md:145-151`).
- **Three upstream dependencies, one commit pin.** If any of HS-S0079, HS-S0080 or HS-S0081 slips or is
  re-worked after the run, the pinned commit no longer contains what the verdict describes (EC-010).
  Write the verdict against a tree in which all three are merged, and re-pin rather than re-word if one
  moves afterwards.
- **The registration list is another project's.** The AC-003 reconciliation targets
  `projection-store-freeze`'s rule registrations, which this project does not own and cannot freeze.
  Record the commit of that list you diffed against so the reconciliation stays checkable later.
- **A "did not hold" verdict's blast radius exceeds this PR.** It produces a superseding ADR, a re-plan
  and a meta-finding about the freeze rule itself (`RUNBOOK.md:4427-4430`). This story can only stage
  the note and state the routing; the human and `redkiln` own the rest, and the story must not stall
  waiting for them.
- **Slice-mate coupling.** `verdict-ordering-and-publication-handoff` (HS-S0083) depends on this story
  and merges it ahead of `publication-and-positioning`'s gate. The ordering *claim* is HS-S0083's;
  making the artefact exist in time is this story's. The two are implemented in one context, so the
  temptation is to write the ordering statement here — that would put project AC-011's evidence in the
  wrong PR (`_storymap.md:49`; `_decomposition.md:481`).
- **T1's finding is the one most likely to be quietly dropped**, because by the time this story runs
  `live_handle.rs` has been retired and the counter-example is no longer in the tree to remind anyone.
  The transcripts in `references/adapter-shapes.md` are the only surviving prompt.

## Dependencies

**Blocks on** (all three must be merged before the run commit this verdict pins):

- **`read-your-own-writes-projection`** (HS-S0079) — supplies AC-006's read-your-own-writes answer in
  PS-12's terms and completes the rules-run list AC-003 reconciles.
- **`cold-build-cost-and-ci-shape`** (HS-S0081) — supplies the CI shape AC-002 names and the cold-build
  number AC-002 cites rather than restates; also owns `RUNBOOK.md:4437`'s box.
- **`package-completeness-and-name-claim`** (HS-S0080) — supplies the crate state and the held name, and
  owns the `publish = false` box among AC-007's ticks.

Transitively, through those three: `ladybug-fixture-and-conformance-run` (the run itself),
`fill-the-bodies-and-ps-34-disposition`, `real-lbug-driver-swap`, `adr-0025-three-answers` and
`preflight-and-unlike-axes` (the axes document AC-002 pins and AC-004 judges against).

**Unlocks**

- **`verdict-ordering-and-publication-handoff`** (HS-S0083) — the slice-mate; it states and proves the
  ordering against `publication-and-positioning` and carries the handover.
- **`publication-and-positioning`** (HS-P0016), at the project grain — DR-8 requires this verdict merged
  before that project's gate opens, so a "did not hold" is heard before the surface is published rather
  than after (`project.md:166-170`).

## Anchors (progressive disclosure)

Link, do not paste. Each row says why the artefact is load-bearing and the moment to open it.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/freeze-verdict-document/discover.md` | Carries the four named wrong implementations in full, including the two that pass every field check, and the four questions deferred to this spec. The document's shape exists to defeat these specifically. | Before writing the skeleton — first, and once. | AC-001, AC-003, AC-005 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/project.md` | DR-4's five fields (`:149-151`), the "did not hold" routing (`:109-111`, `:210-213`), the GAT/ICE and `WriteTransactionInUse` risk entries (`:272-283`) and DR-9's re-takeability (`:174-177`). | Before drafting the front matter and the findings section. | AC-002, AC-005, AC-006, AC-008 |
| `spec/SPECIFICATION.md` | PS-2 at `:4760-4774` is the bar the verdict is judged against and its *Rejects* paragraph states what the whole project is for; PS-12 at `:5052-5075` supplies the exact vocabulary AC-006 requires and names Ladybug as its candidate falsifier. `[FROZEN]` and read-only. | Open PS-2 when writing `## What this was checked against`; open PS-12 when writing the read-your-own-writes row. Never open it to edit. | AC-004, AC-006 |
| `references/evaluation/README.md` | Defines the lifecycle the verdict is classified into — immutable, dated, commit-pinned, superseded rather than edited (`:1-13`, `:77-85`) — and records at `:87-92` that this material binds nothing. It is the mount point and the reason supersession is not optional. | When mounting the document, and again before any later correction. | AC-007, NF-005, NF-006 |
| `RUNBOOK.md` | `:4427-4430` frames the proof artefact and gives the "did not hold" branch its shape (superseding ADR plus the meta-finding); `:4432-4438` is the exit-criteria block ticked in place; `:598`, `:601`, `:602` assign the clause owners; `:4440` names the E2E cases phase 11 makes writable. | When writing the routing section, when ticking the boxes, and when filling the owner column. | AC-004, AC-006, AC-007, AC-008 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/_decomposition.md` | T1 in full at `:314-331` and T2 at `:332-359`; AC-A04 at `:51-54` designating the verdict as the home for every deviation; M9's evidence lifecycle at `:211-218`; the AC-004 reconciliation at `:474`; the AC-007/AC-008 testing rows at `:477-478`; the merge-gate commands at `:518-534`. | Open the T1/T2 ranges before writing `## Findings the suite cannot emit`; open `:474` before reconciling the rule table. | AC-003, AC-005, AC-007, AC-008 |
| `references/adapter-shapes.md` | `:169-195` is the transcript proving the borrowed GAT compiled and did **not** fail the `Send` flavour across a real `tokio::spawn` — the evidence T1 says the freeze consumed. `:186-191` records that binding an owned type to the GAT bought none of the E0195 relief, which is what stops "the trap is retired" from being vacuous. | Before writing the T1 and GAT/ICE rows — the counter-example is no longer in the tree by this point, and this is the only surviving prompt. | AC-005 |
| `crates/happenstance-ladybug/src/live_handle.rs` | `:38-66` holds the ICE transcript: the failure mode for a store that does not outlive every batch lifetime is a compiler panic, not a diagnostic. | When recording the GAT/ICE finding, on either branch of T2. | AC-005 |
| `crates/happenstance-ladybug/src/projection_store.rs` | `:204-213` is the `WriteTransactionInUse` variant and LadybugDB's many-readers-one-writer rule; `:49-64` is the read-your-own-writes seam the skeleton could not close. | When answering AC-006's first two questions — before deciding capability limit versus rule defect. | AC-006 |
| `xtask/src/main.rs` | `:131-151` is the `--show-output` wiring that makes `SKIP` lines visible; without it a declension is invisible and AC-003's reason column cannot be filled from the transcript. | When capturing the run output the rule table is transcribed from. | AC-003 |
| `xtask/src/proof.rs` | `:133` is the `ARTEFACTS` registry row HS-S0078 added, holding the conformance target to the gate. Naming that target makes "the suite ran" checkable rather than claimed. | When writing the `## What this was checked against` section. | AC-003, AC-004 |
| `xtask/src/spec_trace.rs` | `:291-372` is the citation resolver and its twelve-line tolerance — the mechanism behind AC-008's `cargo xtask spec-trace` half. | Only if `spec-trace` goes red; it should not, since no `spec/` path is in the boundary. | AC-008 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/preflight-and-unlike-axes/spec.md` | `:193-205` fixes the axes document's agreed path and PR boundary — the file AC-002 pins by SHA and AC-004 judges the unlike-axis claim against. That document does not exist until HS-S0074 merges. | Before writing the front matter, to get the axes document's path and name exactly right. | AC-002, AC-004 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/_storymap.md` | `:24`, `:26-30` state the unconditional-artefact property twice and say why; `:48-49` are this story's and HS-S0083's rows; `:83-84` places this story last; `:112` assigns the six citation repairs elsewhere. | When you are tempted to make the routing section conditional, or to state the merge ordering here. | AC-001, AC-008 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/_design.md` | `:40-98` records the approved no-surface determination and the sign-off. It is why this story owes no `## Items` row, no doctest and no perceptual review. | If anyone asks for a rendered surface or a design review of this story. | AC-001 (Interaction quality) |
| `.kb/decisions/0008-one-derivation-for-both-ports.md` | The accepted atom on GATs across a suspension point — the decision T1's and the GAT/ICE finding's language must be written against, so the verdict's framing matches the record rather than re-deriving it. | Before writing the GAT/ICE paragraph. | AC-005 |
| `.kb/decisions/0001-async-port-flavours.md` | The two-flavour derivation the conformance emitters (tokio, blocking, wasm) come from — the frame for reporting whether ADR-0025 Q3's bridge narrowed the harness. | When writing the blocking-bridge row. | AC-006 |
| `.kb/open-questions/cf-40-fixture-limits-ownership.md` | Where fixture capability limits live and who owns declaring them. Consumed as evidence for the `declined` outcome column; **not resolved here** — it is `projection-store-freeze`'s (DR-5). | When writing the rule table's reason column, if a declension's ownership is unclear. | AC-003 |
| `.kb/open-questions/projection-store-batch-has-no-apply-seam.md` | The open question this project reports on rather than rediscovers (DR-7): whether the phase-6 write seam survived a third implementer. | When writing the findings section, to keep the report separate from a resolution this story may not take. | AC-005 |
| `spec/E2E-CASES.md` | Holds E2E-19 and E2E-24, whose third-shape halves AC-006 requires a stated status for. Writing the cases is not this story's. | When writing the last findings row. | AC-006 |

## Clarifications resolved during spec

1. **The AC set is exactly the eight the front half decided** — AC-001 through AC-008 — with no
   additions or drops. AC-005 and AC-006 split the four suite-invisible findings along a real seam:
   AC-005 is what the freeze *cost* (T1, GAT/ICE — reported), AC-006 is what this story must *answer*
   (`WriteTransactionInUse`, read-your-own-writes, the blocking bridge, the E2E third-shape status).
   Keeping them as one row would have let a diligent T1 paragraph carry an unanswered
   `WriteTransactionInUse`, which `discover.md:50-58` explicitly forbids.
2. **The build-cost number and the E2E status found their homes.** The number is cited, never copied
   (AC-002, NF-002); the E2E-19/E2E-24 third-shape status is an AC-006 row rather than a section of its
   own, because it is a *stated answer* handed to HS-P0016, not a finding about the run
   (`RUNBOOK.md:4440`).
3. **"Reviewed, not scripted" is honoured rather than worked around.** The testing brief says AC-007's
   evidence is a presence-and-content review the merge gate cannot fabricate (`_decomposition.md:477`).
   Two things were made mechanical anyway because they can be — the heading/literal checks and the
   rule-row reconciliation — and the rest (AC-005, AC-006) is left to review with the exact artefacts
   to read against named in the Anchors table, so "reviewed" does not mean "unspecified".
4. **The interaction-quality section is answered, not skipped.** `_design.md` declares no surface
   (`:40-98`), so no composition is inherited from it; the composition invariants are taken instead from
   the evidence-document genre's own rules and are all carried by existing AC rows. The state-family
   invariants that do not apply are named as N/A with the reason, rather than omitted.
5. **`references/evaluation/phase-11-preflight-and-unlike-axes.md` is cited in prose but is not an
   anchor row**, because it does not exist in the tree until HS-S0074 merges. The anchor is that story's
   spec, which fixes the path. Every path in the Anchors table exists at the time of writing.
6. **The AC-003 reconciliation is pinned to the run commit**, not to `HEAD`. The registration list is
   `projection-store-freeze`'s and can move; the spec did not previously say which version to diff
   against, and a reconciliation against a moving target is not one (EC-003).
7. **Not resolved here, and deliberately:** whether `WriteTransactionInUse` is a capability limit or a
   rule defect. It is genuinely open and it is this story's to answer *from the run*
   (`discover.md:50-58`) — fixing the answer in the spec would be the same error as drafting the
   verdict before the result. What is fixed in advance is the shape of the two permitted answers, each
   with its consequence, and the exclusion of "retry until green".
