---
item: HS-S0073
stage: spec
created: 2026-08-12T13:47:11.499Z
updated: 2026-08-12T13:47:11.499Z
template_sig: 87bbf1d0
rendered_sig: 1afba512
---

# Spec — The far-end discharge recorded for the publication audit

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DoD 5, DoD 6, DoD 12 (`:354-396`) |
| Project | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/project.md` — AC-013, DR-5, DR-7, DR-9, *Definition of done* item 6 |
| This spec | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/far-end-discharge-record/spec.md` |
| Key brief — architecture | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md:69` (the AC-013 row) and `:145-167` (*Root C, the gate*, and *Root D, the knowledge base*) |
| Key brief — testing | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md:523` (AC-013 is a **static** tier), `:517` (how a skip's reason is read out of `--show-output`) |
| Key brief — deployment | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md:684-696` — the two live jobs whose logs are this record's raw material |
| Signed-off design | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_design.md` — **no user-facing surface**, approved 2026-08-12; `## Items` is `N/A`, so this story claims no surface id |
| Story map row | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_storymap.md:72` and `:204-208`; merge order at `:262-265` |
| This story's discover | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/far-end-discharge-record/discover.md` — the signal ledger, the three named wrong implementations, and the two questions (§5 location/format, §6 the `ARTEFACTS` row) deferred **to this spec** |
| Roadmap | `RUNBOOK.md:4311-4390` — phase 10 in full; `:4384` (*Proof artefact*), `:4389` (the empty **Session log** this story fills); `RUNBOOK.md:606` and `:622-635` (the residual-exposure table phase 12 audits) |

## One-line PR slice

The status of ES-10, ES-11, ES-12, ES-41, ES-42 and VT-21 – VT-24 after this
work — discharged, still exposed, or amended, and **against which store** — is
written where the publication audit reads it instead of re-deriving it.

## Executive summary

This PR lands the project's **memory**. Every story before it produced evidence
that is perishable: a green concurrency family inside a Docker-gated CI job, a
skip list printed once to a log with `--show-output`, an ADR that answers a
different question, and two crates that now say the right thing about
themselves. `publication-and-positioning` (HS-P0016) will need, months later, to
state which clauses were checked against which implementations, because
initiative AC-08 requires the published compliance claim to name them
(`project.md` *Unlocks*). This story writes that down while the runs are fresh.

The delta against what already exists in the tree:

- **Delta on `spec/SPECIFICATION.md`.** §7.1 already carries a maturity marker
  per clause and §7.2 a rule cross-reference — and both are *generated from the
  same `parse_clauses` the check itself runs* (`xtask/src/spec_trace.rs:46`), so
  they agree with the clauses whatever any adapter did. This record adds the one
  axis a marker cannot carry: which store, which fixture backing, which run, and
  which of pass / fail / skipped-with-this-reason / amended / not-exercised.
- **Delta on `references/adapter-shapes.md`.** That document records what six
  *skeletons* told the type checker, and §5 (`:286-303`) is explicitly the table
  of what a skeleton does **not** prove — with `Phase 10` written in the "still
  empty" column for position allocation (`:294`) and transport (`:295`). This
  story is the first one entitled to change those two cells, and DoD 6 asks for
  the entries "including anything that contradicts a prior assumption".
- **Delta on `RUNBOOK.md`.** Phase 10's **Session log** at `:4389` is empty, and
  its residual-exposure table at `:606` still names ES-10, ES-11 and ES-12 as
  phase 10's to discharge. The record supplies the evidence; the *reconciliation*
  of that table is knowingly HS-P0016's (`:622-635`).
- **Delta on the gate.** Nothing in the tree can currently fail because a written
  record is wrong or absent. This story adds the one step that can — a file-reading
  lint in `xtask/src/main.rs`'s `REQUIRED` array — and decides, on the evidence,
  whether phase 10 also earns an `xtask/src/proof.rs` `ARTEFACTS` row.

What it does **not** land: any maturity-marker promotion, any clause edit, any
ADR, any change to store behaviour, and any reconciliation of the runbook's
knowingly-short residual-exposure groups. ES-10 stays `[FROZEN]`; ES-11, ES-12,
ES-41, ES-42 and VT-21 – VT-24 stay `[PROVISIONAL]`.

## Context pack

The decisions below are the ones this story must honour. They are stated here as
decisions, not as reading; the deeper material is behind the anchors.

**1. The unit of the record is a triple — clause × store × outcome — and the
store term is a *fixture and its backing*, not a type name.** AC-013's wording is
"and against which store" (`project.md:276-278`), and the initiative's compliance
claim must name its implementations (`project.md` *Unlocks*). A per-clause status
column cannot serve, because it writes "ES-11 passed against a real Neon
endpoint", "ES-11's rule was skipped because `NeonFixture` declined a capability"
and "ES-11 was never exercised at all" identically. The `store` term must
therefore name what was actually behind the fixture — a Neon branch reached over
`/sql`, a pinned Postgres container image — and the CI job and run the evidence
came from. A row reading "`NeonEventStore` over `NeonFixture`" is satisfied by
the mutant `neon-fixture-and-live-job` exists to forbid (a pooled Postgres
connection standing in, `project.md` DR-4); a row naming the branch, the endpoint
and the job is not.

**2. Outcome is a five-valued term, and "skipped" carries the fixture's own
words.** The permitted values are **passed**, **failed**, **skipped — reason**,
**amended by <record>**, **not exercised**. `RuleOutcome::Skipped` exists
precisely so a declined capability is distinguishable from a pass
(`crates/happenstance-testkit/src/contract.rs:32-38`), and `Capability::declined`
takes a reason because "the reason is not a formality — it is printed on every
run for every rule" (`:378-380`). Flattening a skip to "not applicable" in this
record throws away the only thing that made the decline honest in the first
place, and it is the fact HS-P0016 needs most: a clause whose far end was
*declined* has not been discharged, however green the run was.

**3. No maturity marker moves in this story, and that is a decision, not
timidity.** ADR-0012 names the exact act: "lifting on no new evidence moves a
maturity marker because a phase wanted it moved, which is the one thing a marker
must never do" (`.kb/decisions/0012-append-shape-and-preconditions.md:97-99`).
Here there *is* new evidence, which makes the promotion more tempting rather than
less — and it is still a decision with an owner, and the owner is HS-P0016 with
this record as its input. ES-10 is `[FROZEN]` already; changing it takes a new ADR
and this story writes none (`CLAUDE.md`, *Open questions*). Promoting ES-11 or
ES-12 to `[FROZEN]` is one word per row, `spec-trace --write` regenerates §7.1
cleanly afterwards, and the gate stays green — because `spec-trace` verifies that
a clause cites rules which *exist*, not that a marker was *earned*. That is why
the prohibition has to live in this spec rather than in the gate.

**4. §7.1 and §7.2 are generated, and regenerating them is not evidence of
anything.** The region between `<!-- BEGIN GENERATED: spec-trace §7.1–§7.2 -->`
and `<!-- END GENERATED -->` (`xtask/src/spec_trace.rs:200-201`) is rendered by
`cargo xtask spec-trace --write` and compared for equality by the gate on every
run. Since both the table and the check derive from one `parse_clauses`
(`xtask/src/spec_trace.rs:46`), the table agrees with the clauses no matter what
any adapter did. So: never hand-edit that region, and never cite its regeneration
as proof of a discharge. If it is stale because an earlier story moved something,
the remedy is `--write` and nothing else.

**5. `spec-trace` checks `file:line` citations *and their subjects*, which makes
editing `references/adapter-shapes.md` a line-numbering problem.**
`check_citations` (`xtask/src/spec_trace.rs:291-372`) resolves every citation in
`SPECIFICATION.md`, fails if it points past the end of the file, and fails again
if the cited subject is not within twelve lines of the cited line (`:374-377`).
Three citations reach into this document — `references/adapter-shapes.md:97-102`,
`:220` and `:297` (`SPECIFICATION.md:833,840,1208,8095`) — and `:297` is the
**Batch shape** row of the very §5 table this story updates
(`references/adapter-shapes.md:294-297`). Edit the position-allocation row at
`:294` and the transport row at `:295` **in place**; inserting a line above `:297`
shifts the cited subject and turns a documentation edit into a red gate. If a
shift is genuinely unavoidable, repairing the citation in `SPECIFICATION.md` is a
citation repair — permitted, recorded, and not a clause change.

**6. `references/` and `RUNBOOK.md` are inert to the compiler, which is exactly
why this story needs a gate step.** `xtask/src/affected.rs`'s `INERT` list
(`:248-268`) contains `references/`, `spec/`, `.kb/`, `.bklg/` and `RUNBOOK.md`:
they reach no package. A story whose whole deliverable is a `references/` file
plus a runbook paragraph maps to nothing, compiles nothing, and — but for the
lints and `spec-trace` that `affected` runs unconditionally
(`.redkiln/config.yaml`, `affected_gate`) — would be green before it started.
This is the same argument `xtask/src/lints.rs` makes for existing in the gate at
all: "there is no compiler pass that reads `CHANGELOG.md`" (`:1-13`). There is no
compiler pass that reads a discharge record either.

**7. Therefore the record is *checked*, and the check is a file read.** This spec
settles discover.md's deferred question 5: the record is a new document at
**`references/far-end-discharge.md`** — a sibling of `adapter-shapes.md`, in the
tree CLAUDE.md describes as "evidence kept for citation, binding nothing" — and a
new `REQUIRED` step reads it. The step asserts the structural facts a rot would
break: that every clause id this project owns has exactly one row, that every row
carries a non-empty store term and an outcome drawn from the five permitted
values, that no row's outcome is `skipped` with an empty reason, and that every
clause id named in the record is one `parse_clauses` still recognises. It asserts
nothing about whether an outcome is *true* — that is judgement, exactly as
`lints.rs` says of its own five ("every check below states what it does *not*
verify, because a check whose limits are undocumented is read as a guarantee",
`:11-13`).

**8. The `ARTEFACTS` question is decided here, and the deciding constraint is
Docker.** discover.md deferred it; the answer turns on a mechanism detail.
`xtask/src/proof.rs`'s `check` asserts the named tests out of `--list` and then
**runs the target** (`:230-238`). So a phase-10 row would put
`cargo test -p happenstance-postgres --all-features --test <target>` in the
default gate — which is fine if the live-suite gating that
`postgres-schema-and-live-fixture` chose leaves the target listable and
runnable-to-a-pass with no server (an `#[ignore]` or an env read that returns
early), and a violation of AC-011 / DR-9 if it does not (a `required-features`
gate makes the target unlistable; a panic on a missing DSN makes it red).
**Decision: take the row if and only if the chosen gating admits it with no
Docker and no network, and record the verdict either way in the record itself.**
The verdict is a deliverable; a silent omission is not.

**9. What this record reports on is nine clause rows, and their starting state is
a fact to check rather than to remember.** ES-10 is `[FROZEN]`; ES-11 and ES-12
are `[PROVISIONAL]` (`spec/SPECIFICATION.md:8592-8594`); ES-41 and ES-42 are
`[PROVISIONAL]` (`:8623-8624`); VT-21 – VT-24 are `[PROVISIONAL]` (`:8547-8550`).
Read them out of the tree at implementation time rather than trusting this
sentence — an earlier story in this project may legitimately have moved one by
ADR, and if it did, that ADR is what the row cites.

**10. The three results this record reproduces rather than re-derives, each by
citation.** ADR-0024's chosen position-visibility mechanism and its measured cost
(`adr-0024-position-visibility-mechanism`, HS-S0065 — cite the accepted atom
under `.kb/decisions/`, never a summary that could disagree with it); the
`conflicting_position` verdict against a live endpoint, and specifically whether
ES-25's permission to return `None` was exercised by Neon or not
(`neon-conflicting-position-verdict`, HS-S0070); and the Neon fixture's declared
capabilities and numeric ceilings with their stated reasons
(`neon-fixture-and-live-job`). A restatement that drifts from the atom is a third
copy of a requirement, which is the failure `.kb/`'s two-places-on-purpose split
already guards against (`CLAUDE.md`, *Where the work lives*).

**11. Two axes stay empty and the record must say so.** `RUNBOOK.md:606`'s
residual-exposure row names five clauses carrying CF-25's risk — ES-10, ES-11,
ES-12, **ES-35** and **ES-40** — and this project owns only the first three.
Durability's far end (ES-35) is `sqlite-durable-store`'s and completeness's
(ES-40) is `retention-and-incomplete-logs`' (`project.md` *Out of scope*). A
record that reports three discharges without naming the two it did not touch
invites an audit to read "far ends filled" as "portfolio complete". `CF-26` is
the clause that forbids reading a fixture as a far end
(`RUNBOOK.md:687-688`, `references/adapter-shapes.md:286-303`).

**12. The runbook's residual-exposure groups are knowingly wrong, and repairing
them is not this story's.** They are short by ES-41, ES-42, CF-39 and CF-40, and
the last row still carries ES-10, which is no longer provisional — "five edits and
it is the next pass's, not this one's — but it is phase 12's audit that reads this
table" (`RUNBOOK.md:622-635`). `project.md` assigns that reconciliation to
HS-P0016. The record **points at** the discrepancy by line so the audit meets it
deliberately instead of tripping over it; it does not perform the five edits.

**13. The persona-journey slice.** There is no screen (`_design.md` — "N/A, no
user-facing surface", approved 2026-08-12). The reader this story serves is a
**maintainer at publish time**, months later, holding initiative DoD 12 — "a run
over the specification reports every clause's maturity, and no clause is
provisional with an empty falsifier" (`initiative.md:393-395`) — and initiative
AC-08's requirement that the published compliance claim name the implementations
it was checked against. Their failure mode is not confusion; it is *plausible
reconstruction*: reading a green marker and inferring a passing adapter, which
CF-26 says is exactly the inference nobody may make. This story's whole job is to
make that inference unnecessary.

## Integration contract

- **Archetype**: `capability`. User-observable in this repository's only medium
  for it: a document a future project reads, and a gate step that fails when the
  document has rotted. It is not a `foundation` — nothing in this project consumes
  it; the consumer is HS-P0016.
- **Slice / milestone**: `publish-readiness-and-audit`. Slice-mate:
  **`deskeleton-and-package-readiness`** (HS-S0072), implemented in the same
  context and mounted with this one, and this story's `depends_on`. Order within
  the slice is that story first, this one second — it "reports on what the five
  slices above it actually discharged" (`_storymap.md:262-265`), so it cannot be
  written until they have.
- **Mount point**: **`xtask/src/main.rs`** — the `REQUIRED` step array at `:105`,
  which the architecture brief names as Root C, "the single definition of what CI
  runs" (`_decomposition.md:145-157`). The new discharge-record step is added
  there beside the existing file-reading lints (`:340-420`), implemented in
  **`xtask/src/lints.rs`** and dispatched from the subcommand match at `:682-687`.
  Adding it to `lint_steps()` (`:799-808`) is what makes it reachable from
  `cargo xtask lints`, which is half of `.redkiln/config.yaml`'s
  `reachability_static` grain — so the record is checked at story grain and not
  only at release grain. That step is what makes this story *mounted* rather than
  a document committed beside the code: without it, `references/` is inert
  (`xtask/src/affected.rs:248-268`) and nothing in the tree could ever go red
  because the record was wrong.
- **Wires into**:
  - `references/far-end-discharge.md` — **new**; the record itself, and the file
    the new step reads.
  - `references/adapter-shapes.md:286-303` — §5's "what is still empty" table;
    rows `:294` (position allocation) and `:295` (transport) edited **in place**,
    with `:297` held at `:297` because `SPECIFICATION.md:8095` cites it.
  - `RUNBOOK.md:4389` — phase 10's empty **Session log**; and `:4380-4384`'s exit
    criteria and proof artefact, which the log is written against.
  - `xtask/src/lints.rs` — the new check, modelled on
    `changelog_names_every_rule` (`:525`) and `no_position_literals` (`:628`),
    both of which read a file and match strings.
  - `xtask/src/proof.rs:133` — `ARTEFACTS`, read to decide Context-pack item 8;
    edited only if the verdict is that phase 10 earns a row.
  - `xtask/src/spec_trace.rs:200-201`, `:291-372` — read-only: the generated-region
    markers and the citation checker whose twelve-line subject window constrains
    the `adapter-shapes.md` edit.
  - `CHANGELOG.md` — the new gate step named, in the register the existing steps
    use.
  - `.kb/decisions/` (ADR-0024 and any DoD-6 amendment record) and the two Neon
    stories' ledgers — read-only, as the citations the record reproduces.
- **Renders surfaces**: **none.** `_design.md` records this project as having no
  user-facing surface and its `## Items` block is `N/A`; the 2026-08-12 sign-off
  approved that determination. No surface id exists for this story to claim and no
  perceptual review is owed (`.redkiln/config.yaml` deliberately omits
  `design.capture`).
- **Conformance rule(s)**: **none, and deliberately so.** This story defines no
  store, changes no port and adds no rule, so nothing it touches is observable
  through `happenstance-testkit`'s `suite.rs` — a conformance rule observes a
  store through the `EventStore` trait and cannot see a markdown file. Adding one
  here would be the decorative rule CLAUDE.md's corollary forbids: no adapter
  could fail it. Its falsifier is the new gate step, which is why the step is a
  deliverable and not a nicety. Nothing is owed to
  `crates/happenstance-testkit/tests/` and no `mutation_coverage.rs` `REGISTRY`
  row is added.
- **Clause(s)**: **none discharged by edit, and none amended.** The record
  *reports on* ES-10, ES-11, ES-12, ES-41, ES-42 and VT-21 – VT-24 and changes no
  clause text and no maturity marker; `spec/SPECIFICATION.md` is touched only if
  `cargo xtask spec-trace --write` must regenerate a stale §7.1–§7.2 region left
  by an earlier story, or if the `adapter-shapes.md` edit forces a citation
  repair. Both are recorded. A `[FROZEN]` clause changes by ADR, never by edit
  (`CLAUDE.md`).
- **Advances DoD scenario**: initiative **DoD 12** — "the clause ledger is audited
  at publish… a run over the specification reports every clause's maturity, and no
  clause is provisional with an empty falsifier" (`initiative.md:393-395`). This
  story does not perform that audit; it supplies the one input the audit cannot
  reconstruct from the specification, which is which implementation each clause
  was checked against. It is also the point at which **DoD 5** and **DoD 6** become
  *citable*: both scenarios are observed by earlier stories in this project, and
  this is where the observation is written down in a form that survives the CI
  logs it was observed in.

## PR boundary

`redkiln verify --grain story` reads the fenced block below and fails on any file
changed outside it.

```
references/far-end-discharge.md
references/adapter-shapes.md
RUNBOOK.md
xtask/src/lints.rs
xtask/src/main.rs
xtask/src/proof.rs
CHANGELOG.md
spec/SPECIFICATION.md
.bklg/from-contract-to-published-library/postgres-and-neon-stores/far-end-discharge-record/**
```

Two entries need their presence justified, because both are dangerous and both
are narrower than they look.

`spec/SPECIFICATION.md` is in the boundary **only** for two named repairs: a
`cargo xtask spec-trace --write` regeneration of the §7.1–§7.2 region if an
earlier story left it stale, and a `file:line` citation repair if the
`adapter-shapes.md` edit could not avoid shifting a cited subject (Context pack
items 4 and 5). No clause text, no maturity marker, no §7.3–§7.6 judgement prose.
If a clause needs to change, this story stops and reports it.

`xtask/src/proof.rs` is in the boundary because Context-pack item 8's verdict may
be "yes". If it is "no", the file is untouched and the verdict is still recorded.

**In this PR**

- `references/far-end-discharge.md`: the record — nine clause rows as
  clause × store × outcome triples, each naming the fixture's real backing and
  the CI job and run it was observed in; the two axes this project did not fill;
  the pointer to `RUNBOOK.md:622-635`; the `ARTEFACTS` verdict; and citations to
  ADR-0024, the Neon `conflicting_position` verdict and the fixtures' declared
  capabilities.
- `references/adapter-shapes.md`: §5's position-allocation and transport rows
  updated in place, plus the phase-10 entries DoD 6 asks for, "including anything
  that contradicts a prior assumption".
- `RUNBOOK.md`: phase 10's **Session log** written; its exit-criteria boxes
  reconciled with what was actually observed.
- `xtask/src/lints.rs` + `xtask/src/main.rs`: the new discharge-record check, its
  `REQUIRED` step, its subcommand arm, and its entry in `lint_steps()`.
- `xtask/src/proof.rs`: an `ARTEFACTS` row for phase 10 **iff** the live-suite
  gating admits it with no Docker and no network.
- `CHANGELOG.md`: the new gate step named.

**Explicitly not in this PR**

- Any maturity-marker promotion — ES-11 and ES-12 stay `[PROVISIONAL]` and ES-10
  stays `[FROZEN]`. That decision is HS-P0016's, on this record's evidence.
- Any clause text change, any new ADR, any `.kb/` atom. This story records; it
  does not decide. An amendment record, if one were needed, would be
  `neon-conflicting-position-verdict`'s and would already exist.
- The five edits that reconcile `RUNBOOK.md:606`'s residual-exposure groups —
  knowingly HS-P0016's (`RUNBOOK.md:622-635`).
- Any change to store behaviour, schema, SQL, transport, fixture, manifest or
  crate documentation. If the record cannot be written truthfully because a run
  was not real or an outcome was mis-declared, that is a blocking finding routed
  to the owning story or to the `support` initiative (`.redkiln/config.yaml`),
  never a row written optimistically.
- Any new CI job, any Docker or network dependency, and any change to an existing
  step's `name` string.

**Merge DoD**: `cargo xtask ci --fast` is green on a clean checkout with no
Docker, no network and no credentials — including the new discharge-record step
and `cargo xtask spec-trace` — with `references/far-end-discharge.md` carrying one
row per clause this project owns, each naming a store, a backing and an outcome.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The record exists at one known path, and the audit is told where it is (AC-001) | `references/far-end-discharge.md`, a new sibling of `adapter-shapes.md` in the tree CLAUDE.md describes as "evidence kept for citation, binding nothing". One file, not a status spread across a runbook log, an ADR and a CI artifact. `RUNBOOK.md`'s phase-10 session log and `references/adapter-shapes.md` both link it rather than duplicating it. | `references/`, `CLAUDE.md` *Repository map*, `RUNBOOK.md:4389` |
| Nine clause rows, exactly, and their starting markers are read from the tree (AC-001) | ES-10, ES-11, ES-12, ES-41, ES-42, VT-21, VT-22, VT-23, VT-24 — the set `project.md` AC-013 names. Each row's *prior* marker is read out of the specification at implementation time, not copied from this spec: ES-10 `[FROZEN]`, the rest `[PROVISIONAL]` as of authoring. A clause with no row and a row with no clause are both failures. | `project.md:276-278`, `spec/SPECIFICATION.md:8592-8594`, `:8623-8624`, `:8547-8550` |
| Every row is a **triple**: clause × store × outcome (AC-001) | The store term is the deliverable's whole point. A per-clause status column is satisfied by copying §7.1's maturity column, which is generated from the same parser the check runs and therefore agrees with the clauses whatever any adapter did. | `xtask/src/spec_trace.rs:46`, `discover.md` *The wrong implementation* |
| The store term names the fixture's **backing**, and the run it was observed in (AC-002) | Not `NeonEventStore over NeonFixture` — that row is satisfied by a `NeonFixture` backed by a pooled Postgres connection, which is the mutant DR-4 forbids and which this story is the last place to catch and the first place to make durable. A row states the real endpoint or image (a Neon branch reached over `/sql`; a pinned Postgres minor under `testcontainers`) plus the CI job name and run identifier the evidence came from. | `project.md` DR-4, `crates/happenstance-neon/src/lib.rs:10-29`, `_decomposition.md:684-696` |
| Outcome is one of five values, and `skipped` carries the fixture's own reason verbatim (AC-003) | **passed / failed / skipped — <reason> / amended by <record> / not exercised**. A declined capability still emits a test returning `RuleOutcome::Skipped`, and the reason is printed on every run precisely so the trade is on the record; collapsing it to "n/a" here discards the fact HS-P0016 needs most. "Not exercised" is a distinct, permitted, honest value — a clause nobody ran is not a clause that passed. | `crates/happenstance-testkit/src/contract.rs:26-42`, `:374-380`, `_decomposition.md:517`, `project.md` DR-5 |
| The three upstream results are reproduced by **citation**, never restated (AC-002) | ADR-0024's chosen mechanism and measured cost cite the accepted atom under `.kb/decisions/`; the `conflicting_position` verdict cites `neon-conflicting-position-verdict`'s record, including whether ES-25's permission to return `None` was exercised; the fixtures' capabilities and ceilings cite the fixture source. A restatement that drifts from an atom is a third copy of a requirement. | `.kb/maps/decision-map.md`, `.bklg/from-contract-to-published-library/postgres-and-neon-stores/neon-conflicting-position-verdict/`, `crates/happenstance-neon/src/lib.rs:46-77`, `CLAUDE.md` *Where the work lives* |
| No maturity marker moves and no clause text changes (AC-004) | ES-10 stays `[FROZEN]`; ES-11, ES-12, ES-41, ES-42 and VT-21 – VT-24 stay `[PROVISIONAL]`. Promotion is one word per row and `spec-trace` would stay green, because it checks that a clause cites rules which exist, not that a marker was earned — which is exactly why the prohibition lives in this spec. The promotion is HS-P0016's decision on this record's evidence. | `.kb/decisions/0012-append-shape-and-preconditions.md:97-99`, `CLAUDE.md` *Open questions*, `xtask/src/spec_trace.rs:1-60` |
| §7.1–§7.2 are regenerated only by `spec-trace --write`, and never cited as evidence (AC-004) | The region between the two markers is generated and compared for equality by the gate on every run. Hand-editing it passes review and then drifts; regenerating it proves nothing, since table and check share one `parse_clauses`. Expected diff on `spec/SPECIFICATION.md`: **empty**, unless an earlier story left the region stale or the `adapter-shapes.md` edit forced a citation repair. | `xtask/src/spec_trace.rs:200-201`, `:23-31`, `:46` |
| The record is **checked** by a gate step, not merely written (AC-005) | A new file-reading step in `REQUIRED`, implemented in `lints.rs` beside `changelog_names_every_rule` and `no_position_literals`, and listed in `lint_steps()` so `cargo xtask lints` — half of `reachability_static` — runs it at story grain. It asserts: one row per owned clause id, a non-empty store term, an outcome from the five permitted values, no empty skip reason, and no clause id the parser no longer recognises. It asserts nothing about truth, and says so in its own module docs, per `lints.rs`'s standing rule that a check must state what it does not verify. | `xtask/src/main.rs:105`, `:340-420`, `:682-687`, `:799-808`, `xtask/src/lints.rs:1-13`, `:525`, `:628`, `.redkiln/config.yaml` *verify* |
| The `ARTEFACTS` verdict is decided and written down (AC-005) | `proof.rs`'s `check` lists the named tests **and then runs the target**, so a phase-10 row is admissible only if the live-suite gating leaves that target listable and runnable-to-a-pass with no Docker and no network. Take the row if so; decline it with the gating mechanism named if not. Either way the verdict, and its reason, is a line in the record — a silent omission would leave the question open a second time. | `xtask/src/proof.rs:133`, `:196-238`, `.bklg/from-contract-to-published-library/postgres-and-neon-stores/postgres-schema-and-live-fixture/`, `project.md` AC-011, DR-9 |
| `adapter-shapes.md` §5's two phase-10 cells are updated **in place** (AC-006) | The position-allocation row at `:294` and the transport row at `:295` both read "Phase 10" in the "what is still empty" column; this story is the first entitled to change them. Edit within the existing lines. `SPECIFICATION.md:8095` cites `references/adapter-shapes.md:297` and `check_citations` requires the cited subject within twelve lines, so an inserted line above `:297` is a red gate, not a formatting choice. | `references/adapter-shapes.md:286-303`, `:294-297`, `xtask/src/spec_trace.rs:291-372`, `:374-377`, `spec/SPECIFICATION.md:8095` |
| The phase-10 entries DoD 6 asks for include the contradicted prior assumptions (AC-006) | "What the two adapters told the type checker **and the server** that the skeletons could not, including anything that contradicts a prior assumption" — and two contradictions are structural rather than incidental: the Neon CTE keeping `conflicting_position` against the decision ledger's standing assumption, and the Postgres frontier's structural cost, which the throughput experiment could not price. Both belong in the entry rather than only in the record. | `project.md` *Definition of done* item 6, `references/adapter-shapes.md:286-303`, `crates/happenstance-neon/src/lib.rs:46-77`, `project.md` DR-3 |
| `RUNBOOK.md` phase 10's session log is written against its own proof artefact (AC-007) | The log at `:4389` is empty. It is written against `:4384`'s proof artefact — the concurrency macro green under a multi-thread runtime on a store that does not serialise its writers, a committed number for the visibility strategy including the held-transaction behaviour, and Neon's capability skip list — and against `:4380-4384`'s exit criteria, ticked only where observed. A box that was not met is said so rather than ticked, per the session protocol at `RUNBOOK.md:59-67`. | `RUNBOOK.md:4374-4389`, `:59-67` |
| The record names what it does **not** discharge (AC-008) | Three absences, each stated: ES-35 (durability) and ES-40 (completeness) are far ends this project did not touch and their owners are named; `RUNBOOK.md:606`'s residual-exposure groups are knowingly short by ES-41, ES-42, CF-39 and CF-40 and still carry ES-10, and reconciling them is HS-P0016's five edits; and the marker promotions this evidence would support are HS-P0016's decision, not this record's claim. A record reporting three discharges without its absences invites "far ends filled" to be read as "portfolio complete", which CF-26 forbids. | `RUNBOOK.md:606`, `:622-635`, `:687-688`, `project.md` *Out of scope*, `references/adapter-shapes.md:286-303` |
| The default gate stays Docker-free, network-free and credential-free (AC-009) | The one added step is a file read — no compilation beyond `xtask` itself, no server, no credential — which is the same argument the five existing lints make for their own presence. No CI job is added, no existing step's `name` string changes (`steps_named` panics on a miss and `wasm_steps()` selects by name), and the Neon `wasm32` build stays green. `lint_steps()`'s doc comment says "five checks" over a list of six; adding a seventh entry without correcting the sentence lands a fresh drift in the file whose job is to prevent one. | `xtask/src/main.rs:792-816`, `:771-800`, `project.md` AC-011, DR-8, DR-9, `.redkiln/config.yaml` `integration_scoped` |
| No public API item is added, removed or changed | This story touches documents, a runbook, and `xtask` — which is not a published crate. The semver surface across it is empty by construction, which is what makes it safe to land last in the project and is the precondition HS-P0016's `semver` job inherits. | `_design.md` (`## Items` — N/A), `xtask/Cargo.toml`, `xtask/src/package.rs` `PUBLISHABLE` |

## Data and migrations

**N/A — no schema change, no migration, no backfill, and no runtime data of any
kind.** Three reasons, each verified rather than assumed:

1. **This story ships no code that touches a database.** Its only Rust change is
   `xtask`, which is a build-tool crate, is not in `PUBLISHABLE`
   (`xtask/src/package.rs`), and whose new check reads a markdown file. The
   Postgres schema — migration 1, its phase-4 `EventId` and `recorded_at`
   columns, and whichever column ADR-0024's mechanism added — belongs to
   `postgres-schema-and-live-fixture` and is untouched here
   (`_decomposition.md:713-724`, `RUNBOOK.md:249-253`).
2. **There is nothing to backfill, because there is no prior record to migrate
   from.** `references/far-end-discharge.md` does not exist today; this story
   creates it. The facts it carries are transcribed from CI runs and story
   ledgers that are still live at the time it is written, which is the whole
   reason it is authored now rather than at phase 12 — the alternative is
   re-derivation from job history, which cannot recover *why* a rule was skipped
   or what was behind a fixture during a green run.
3. **The only "data" is one new document and three edited ones.** A new
   `references/` file, plus in-place edits to `references/adapter-shapes.md`,
   `RUNBOOK.md` and `CHANGELOG.md`. All four are inert to the compiler
   (`xtask/src/affected.rs:248-268`), which is precisely why the new gate step
   exists (Context pack items 6 and 7).

One adjacent fact worth stating so it is not rediscovered as a surprise: the
record's *format* is load-bearing in one machine-readable respect and free in
every other. The new check parses it, so the clause-id column, the store column
and the outcome vocabulary are a contract between the document and
`xtask/src/lints.rs`, and changing either half alone breaks the gate. Everything
else in the document — the prose, the ordering, the citations — is authored and
must stay authored, for the reason `spec_trace.rs:56-58` gives about §7.3–§7.6:
generated tables carry facts, and only a person can carry the judgement about why
a gap exists.

## Acceptance criteria

The persona these are written from is **the maintainer at publish time** — the
author of `publication-and-positioning` (HS-P0016), holding initiative DoD 12 and
AC-08, whose journey is *Decide in one sitting* read from the other side: they
must state, in a published compliance claim, which clauses were checked against
which implementations, and they have the CI logs of six months ago and nothing
else (`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:249`,
`initiative.md:393-395`). Their failure mode is *plausible reconstruction* —
reading a green marker and inferring a passing adapter — which is the inference
CF-26 forbids (`RUNBOOK.md:687-688`). Every criterion below is that person's goal
crossing the whole stack: from the live job that produced the fact, through the
record, to the gate step that fails when the record has rotted.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | GIVEN the maintainer at publish time is drafting the compliance claim and knows only that this project happened, WHEN they look for its far-end result, THEN they find one document at `references/far-end-discharge.md` carrying exactly nine rows — ES-10, ES-11, ES-12, ES-41, ES-42, VT-21, VT-22, VT-23, VT-24 — each a clause-times-store-times-outcome triple, and they reach it without prior knowledge because both entry points an auditor already uses link to it: `RUNBOOK.md` phase 10's session log and `references/adapter-shapes.md` §5. A clause with no row, a row with no clause, and a row missing any of the three terms are each a failure. | New gate step `the far-end discharge record is consistent`, `xtask/src/lints.rs`; unit tests `tests::every_owned_clause_has_exactly_one_row` and `tests::a_row_missing_a_term_is_rejected` in the same module. Reachable at story grain via `cargo xtask lints`. |
| AC-002 | GIVEN the maintainer must name the implementation each clause was checked against and cannot re-open a six-month-old fixture to find out what was behind it, WHEN they read any row, THEN the store term names the fixture AND its real backing — a Neon branch reached over the `/sql` endpoint, or a pinned Postgres image under `testcontainers` — plus the CI job name and run identifier the evidence came from; and the three results this project already decided elsewhere (ADR-0024's mechanism and cost, the Neon `conflicting_position` verdict including whether ES-25's permission to return `None` was exercised, and each fixture's declared capabilities and ceilings) are reproduced by citation to the accepted atom or the story ledger, never restated in this record's own words. | `tests::a_row_whose_store_term_names_only_a_type_is_rejected` (a row reading `NeonEventStore over NeonFixture` — the DR-4 mutant — fails); `tests::a_row_without_a_job_and_run_is_rejected`; the citation half is read by the slice review against `.kb/decisions/` and the two Neon stories' ledgers. |
| AC-003 | GIVEN a clause whose rule was skipped because a fixture declined a capability has NOT been discharged however green the run was, WHEN the maintainer reads an outcome, THEN it is one of exactly five values — `passed`, `failed`, `skipped — <reason>`, `amended by <record>`, `not exercised` — and a `skipped` row carries the fixture's own `Capability::declined` reason verbatim as printed by the live job's `--show-output`, so that a decline, a pass and a clause nobody ran are three distinguishable facts rather than one. | `tests::an_outcome_outside_the_five_is_rejected`; `tests::a_skip_with_an_empty_reason_is_rejected`; source of truth for the reason text is the live job log per `_decomposition.md:517` and `crates/happenstance-testkit/src/contract.rs:32-38`, `:374-380`. |
| AC-004 | GIVEN promoting ES-11 or ES-12 out of `[PROVISIONAL]` is one word per row that `spec-trace` would happily stay green through, and GIVEN that decision belongs to the maintainer at publish time and not to this record, WHEN this story merges, THEN no maturity marker has moved and no clause text has changed — ES-10 is still `[FROZEN]`, the other eight still `[PROVISIONAL]` — each row states the marker it read out of the tree, and the record is henceforth red if any row's stated marker disagrees with the clause's marker in `spec/SPECIFICATION.md`. Any diff to that file is either a `cargo xtask spec-trace --write` regeneration of the §7.1–§7.2 region or a `file:line` citation repair, both recorded; a hand edit inside the generated markers is a failure. | `tests::a_row_whose_marker_disagrees_with_the_clause_is_rejected` (the assertion re-reads the marker with the same parser `spec_trace` uses); `cargo xtask spec-trace` as a `REQUIRED` step proves the generated region was not hand-edited; `redkiln verify --grain story` against the PR boundary. |
| AC-005 | GIVEN nothing in this tree can currently fail because a written record is wrong or absent, and GIVEN `references/` reaches no package, WHEN the record rots — a row deleted, an outcome vocabulary drifted, a clause renamed out from under it — THEN the merge gate goes red rather than the audit discovering it months later; and the question of whether phase 10 also earns an `xtask/src/proof.rs` `ARTEFACTS` row is answered in the record itself, taken if and only if the live-suite gating leaves the target listable and runnable-to-a-pass with no Docker and no network, and declined in writing with the gating mechanism named if not. | The step is in `REQUIRED` (`xtask/src/main.rs:105`) and in `lint_steps()`, so `cargo xtask lints` — half of `.redkiln/config.yaml`'s `reachability_static` — runs it; `steps_named` panics on a name that resolves to nothing. `tests::a_rotted_record_is_rejected` drives the check over a fixture record with a row removed. `tests::the_artefacts_verdict_is_recorded` asserts the verdict line is present whichever way it went. |
| AC-006 | GIVEN `references/adapter-shapes.md` §5 is the table of what a skeleton does not prove and still writes `Phase 10` in the still-empty column for position allocation and transport, WHEN the maintainer reads it after this story, THEN both cells state what the two real adapters told the type checker AND the server — including the two prior assumptions this project contradicted, the Neon CTE that keeps `conflicting_position` against the ledger's standing assumption and the Postgres frontier's structural cost the throughput experiment could not price — and they were edited **in place**, with `references/adapter-shapes.md:297` still at `:297` because `spec/SPECIFICATION.md:8095` cites it and `check_citations` requires the cited subject within twelve lines. | `cargo xtask spec-trace` (`check_citations`, `xtask/src/spec_trace.rs:291-372`, `:374-377`) fails if the cited subject shifted; `tests::a_phase_10_cell_left_deferring_is_rejected` fails if either §5 row still defers to phase 10; the DoD-6 content itself is read by the slice review against `project.md` *Definition of done* item 6. |
| AC-007 | GIVEN `RUNBOOK.md` phase 10's **Session log** is empty and its exit criteria are unticked, WHEN the maintainer opens the runbook at the phase this project executed, THEN the log is written against phase 10's own proof artefact — the concurrency family green under a multi-thread runtime on a store that does not serialise its writers, a committed number for the visibility strategy including the held-transaction behaviour, and Neon's capability skip list — it links `references/far-end-discharge.md` rather than duplicating it, and an exit-criteria box that was not met says so rather than being ticked. | `tests::an_empty_phase_10_session_log_is_rejected` (the log is non-empty and cites the record); the session protocol at `RUNBOOK.md:59-67` is the bar the slice review reads the ticks against. |
| AC-008 | GIVEN a record reporting three discharges invites `far ends filled` to be read as `portfolio complete`, WHEN the maintainer reads it, THEN it names its own absences in the same document: ES-35 (durability) and ES-40 (completeness) as far ends this project did not touch with their owning stories named; `RUNBOOK.md:606`'s residual-exposure groups as knowingly short by ES-41, ES-42, CF-39 and CF-40 and still carrying ES-10, cited by line as HS-P0016's five edits and not performed here; and the marker promotions this evidence would support as HS-P0016's decision rather than this record's claim. | `tests::a_record_omitting_the_untouched_far_ends_is_rejected` (ES-35, ES-40 and the `RUNBOOK.md:622-635` pointer are each required to appear); the ownership attributions are read by the slice review against `project.md` *Out of scope*. |
| AC-009 | GIVEN two of this project's gates need a live server and DR-9 forbids either from entering the default path, WHEN any contributor runs the gate on a clean checkout with no Docker, no network and no credentials, THEN `cargo xtask ci --fast` is green **including** the new discharge-record step — which is a file read and nothing more — no CI job is added, no existing step's `name` string changes, the wasm32 steps still resolve by name, and `lint_steps()`'s doc comment is corrected to match the number of checks it now selects. | `cargo xtask ci --fast` offline on a clean checkout (`.redkiln/config.yaml` `integration_scoped`); `cargo xtask affected --base main`; `cargo test -p xtask`; `cargo xtask wasm` (`wasm_steps()` resolves all four names or panics). |

## Interaction quality

**Composition family: N/A, and verified rather than assumed.** This story renders
no surface. `_design.md` records this project's `## Surfaces`, `## Items`,
`## Signatures`, `## Shape decision` and `## Placement and re-export` as
`N/A — no user-facing surface`, and that determination was signed off on
2026-08-12; the initiative sets `userFacing: false` (`initiative.md:411`), no
`interaction-patterns.md` distillation exists, and `.redkiln/config.yaml`
deliberately omits `design.capture` so the perceptual review is a *skip* rather
than a silent pass. There is therefore no composed presentation, no transience
policy, no density budget and no named visual anti-pattern for this story to
honour or violate. Inventing one here would contradict a signed-off design.

**State family: it applies, in this repository's medium.** The artifact a reader
navigates is a document tree that other documents cite by `file:line`, and the
state invariants have exact analogues that are *blocking* rather than decorative.
Each is already carried by a row in the table above — this section only says which
row carries it and how it is verified, because `redkiln verify` extracts ACs from
that table and a bullet here would never be gated.

| Invariant | Analogue in this medium | Carried by | How it is verified |
| --- | --- | --- | --- |
| **In-place, not context-jump** | §5's two phase-10 cells are edited *within their existing lines*. Inserting a line above `references/adapter-shapes.md:297` moves a subject `spec/SPECIFICATION.md:8095` cites, which is this tree's version of a jump that loses the reader's place. | **AC-006** | `cargo xtask spec-trace`'s `check_citations` twelve-line subject window (`xtask/src/spec_trace.rs:374-377`). |
| **Non-occlusion** | The record adds a document; it hides nothing. No existing evidence is overwritten, and the three upstream results are reproduced by citation rather than by a copy that could later disagree with its atom. | **AC-002** | Slice review against `.kb/decisions/` and the two Neon story ledgers. |
| **Preserved focus and selection** | The `file:line` citations already pointing into the edited documents still resolve to the same subjects afterwards — the reader's anchors survive the edit. | **AC-006** | `cargo xtask spec-trace`, `REQUIRED`. |
| **Reversibility** | Nothing this story does is a one-way door: the only permitted `spec/SPECIFICATION.md` diffs are a `--write` regeneration and a citation repair, both mechanically reproducible, and no maturity marker moves. | **AC-004** | `cargo xtask spec-trace`; `redkiln verify --grain story` against the PR boundary. |
| **Reachability without prior knowledge** | The audit must be able to *find* the record. One known path, linked from the two places an auditor already opens — phase 10's session log and `adapter-shapes.md` §5 — rather than a fact spread across a runbook log, an ADR and a CI artifact. | **AC-001**, **AC-007** | `tests::every_owned_clause_has_exactly_one_row` (single path) and `tests::an_empty_phase_10_session_log_is_rejected` (the link from the runbook). |
| **The state is legible, not inferred** | Five outcome values with a verbatim skip reason, so `declined`, `passed` and `never ran` are three readable states instead of one. This is the state invariant this story exists for. | **AC-003** | `tests::an_outcome_outside_the_five_is_rejected`, `tests::a_skip_with_an_empty_reason_is_rejected`. |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | The record names a clause id `parse_clauses` no longer recognises — retired, renamed, or mistyped. | The step fails naming the id and the file, and says which of the two remedies applies: repair the row, or route the rename to the story that performed it. It must not silently drop the row: a record that quietly loses a clause is exactly the reconstruction failure AC-013 exists to prevent. |
| **EC-002** | A row's outcome is outside the five permitted values (`passed`, `failed`, `skipped — <reason>`, `amended by <record>`, `not exercised`). | Fail, printing the offending value and the permitted set. `n/a`, `ok`, `green` and a blank cell are all rejected — the vocabulary is the contract between the document and `xtask/src/lints.rs`, and a synonym admitted once is a synonym forever. |
| **EC-003** | An outcome is `skipped` with an empty or placeholder reason. | Fail. The remedy is to read the fixture's `Capability::declined` reason out of the live job's `--show-output` (`_decomposition.md:517`), not to invent one and not to downgrade the row to `not exercised`. |
| **EC-004** | A row's stated prior marker disagrees with the clause's marker in `spec/SPECIFICATION.md`. | Fail, naming both values. Two causes with different remedies, and the step says so: either a marker moved (which is HS-P0016's decision arriving early — stop and report, do not update the record to match), or the row was mis-transcribed (repair the row). |
| **EC-005** | The `adapter-shapes.md` edit shifts a subject cited from `SPECIFICATION.md`. | `cargo xtask spec-trace` fails. The permitted remedy is a `file:line` citation repair in `SPECIFICATION.md`, recorded in the PR description as such. Editing the clause, weakening the citation, or widening the twelve-line window are all forbidden. |
| **EC-006** | A row cannot be written truthfully — the live job never ran, ran against a mutant fixture (a `NeonFixture` backed by a pooled Postgres connection), or an outcome was mis-declared upstream. | Stop. This is a blocking finding routed to the owning story or to the `support` initiative (`.redkiln/config.yaml`), never a row written optimistically and never a row silently omitted. This story is the last place that lie is recoverable and the first place it becomes durable. |
| **EC-007** | The `ARTEFACTS` row is taken but the live-suite gating leaves the target unlistable (a `required-features` gate) or red without a server (a panic on a missing DSN). | The default gate goes red, which is the correct failure. The remedy is to decline the row and record the verdict with the gating mechanism named — never to relax AC-011/DR-9 by admitting Docker or a credential into `cargo xtask ci`. |
| **EC-008** | The new step's name in `lint_steps()` does not match its `REQUIRED` entry. | `steps_named` panics (`xtask/src/main.rs:800-816`). This is the intended failure mode — the steps are compile-time constants, so a miss is a bug in that file and never a user error — and it is why the step is selected by name rather than by index. |
| **EC-009** | `references/far-end-discharge.md` is absent entirely. | Fail with a message naming the path and this story. An absent record must not read as a vacuous pass: the check reads the file first and errors on the read, in the manner of the existing file-reading lints. |

## Non-functional

| id | Requirement | Why, and where the bar comes from |
| --- | --- | --- |
| **NF-001** | The added gate step is a file read: no server, no network, no credential, no compilation beyond `xtask` itself, and it finishes in the time the existing lint set does. | `lint_steps()`'s own justification — "each is a file read and a string match, so the whole set finishes in the time it takes cargo to decide `xtask` is up to date" (`xtask/src/main.rs:792-800`) — and AC-011/DR-9. |
| **NF-002** | The check documents what it does **not** verify, in its own module or function docs. | `xtask/src/lints.rs:11-13`: "every check below states what it does *not* verify, because a check whose limits are undocumented is read as a guarantee." This check verifies shape, vocabulary and marker agreement; it does not verify that any outcome is true. |
| **NF-003** | Honesty beats completeness in every row. `not exercised` is a permitted, expected value; an optimistic row is a defect even when nothing fails. | `project.md` DR-5 and CF-26 (`RUNBOOK.md:687-688`) — a fixture instrument does not falsify a far-end clause, so a row claiming one on fixture evidence is false in the only way that matters. |
| **NF-004** | The record stays citable for years: one stable path, stable row order (the clause ids in the order AC-013 names them), and citations to accepted atoms and story ledgers rather than to summaries that can drift. | `CLAUDE.md`, *Where the work lives* — the two-places-on-purpose split exists precisely so a third copy of a requirement is never minted. |
| **NF-005** | The public API surface change is empty. No published crate is touched; `xtask` is not in `PUBLISHABLE`. | `xtask/src/package.rs`; it is what makes this story safe to land last and is the precondition HS-P0016's `semver` job inherits. |
| **NF-006** | The record's machine-read columns (clause id, store, outcome) are a contract between the document and `xtask/src/lints.rs`; its prose, ordering and citations stay authored by a person. | `xtask/src/spec_trace.rs:56-58` on §7.3–§7.6: generated tables carry facts, only a person carries the judgement about why a gap exists. Changing either half of the contract alone breaks the gate — change both in one commit. |

## Implementation notes (non-prescriptive)

- **Write the record before the check.** The check's assertions should be
  discovered from a record that is already truthful, not the other way round;
  a check designed first tends to legislate a shape the evidence cannot fill,
  and the first thing to give will be a row's honesty.
- **The parsing seam is a choice, and the cheap one is probably right.** The
  existing lints match strings over `code_lines`-cleaned source
  (`xtask/src/lints.rs:525`, `:628`); a markdown table parsed by splitting on
  `|` is the same order of complexity and needs no dependency. Resist a
  frontmatter/YAML schema unless the string match genuinely cannot express a
  required assertion — a new dependency in `xtask` is a `cargo deny` question.
- **Reuse `spec_trace::parse_clauses` rather than re-deriving markers.** AC-004's
  marker-agreement assertion is only trustworthy if it reads clauses with the
  same parser the specification's own check uses (`xtask/src/spec_trace.rs:46`).
  If that function is not visible from `lints.rs`, making it `pub(crate)` is a
  smaller change than a second parser, and a second parser is the defect.
- **`lint_steps()`'s doc comment already says "five checks" over a list of six.**
  Adding a seventh entry without correcting the sentence lands a fresh drift in
  the file whose job is to prevent one. Fix the count in the same commit.
- **Decide the `ARTEFACTS` question by reading, not by guessing.** Open
  `xtask/src/proof.rs:196-238` and the gating that
  `postgres-schema-and-live-fixture` actually chose, then answer. If the answer
  is no, the record carries one line saying so and naming the mechanism — the
  verdict is a deliverable either way.
- **Where a row's evidence is thin, say so in the row.** A record that admits
  "exercised once, on run N, against a branch that no longer exists" is more
  useful to the audit than one that rounds it to `passed`.
- **Order of work inside the PR:** transcribe the nine rows from the live jobs
  and the story ledgers; write the absences (AC-008); write the `adapter-shapes`
  and runbook entries; then build the check against the finished record; then
  the `REQUIRED`/`lint_steps()`/subcommand wiring; then `CHANGELOG.md`.

## Tests and CI (merge gate)

Tier names follow the testing brief's vocabulary
(`.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md`,
*Testing brief*, where AC-013's tier is **static**).

| Tier | Command / path | Proves |
| --- | --- | --- |
| **Static (new)** | `cargo xtask lints` → the `the far-end discharge record is consistent` step, implemented in `xtask/src/lints.rs` | AC-001, AC-002, AC-003, AC-004, AC-005, AC-006, AC-007, AC-008 — the record exists, has one row per owned clause, every row carries a store term with a real backing plus job and run, every outcome is one of five with a non-empty skip reason, every stated marker still agrees with the clause, §5 no longer defers to phase 10, the runbook log is written and links the record, and the absences are named. |
| **Unit (new)** | `cargo test -p xtask` → `#[cfg(test)] mod tests` in `xtask/src/lints.rs` | That the check above can *fail*: each named test drives it over a deliberately wrong record and asserts an error. Without these the step is decorative — the standing corollary in `CLAUDE.md` about rules no implementation can fail applies to gate steps too. `xtask/src/affected.rs:596`, `xtask/src/package.rs:408` and `xtask/src/lint_constitution.rs:827` are the in-tree pattern; `lints.rs` has no test module today and gains one. |
| **Static (existing)** | `cargo xtask spec-trace` | AC-004 and AC-006 — the §7.1–§7.2 generated region was not hand-edited (equality against a fresh render), and every `file:line` citation still resolves to its subject within twelve lines after the `adapter-shapes.md` edit. |
| **Static (existing)** | `cargo xtask package-check` | NF-005 — the publishable set and its required files are unchanged by this story; `xtask` is not in it. |
| **Process (story grain)** | `cargo xtask affected --base main` (`.redkiln/config.yaml` `affected_gate`) | AC-009 — that a diff of `references/`, `RUNBOOK.md` and `xtask/` still runs the lints and `spec-trace` unconditionally, which is the only reason a `references/`-only change is not vacuously green (`xtask/src/affected.rs:248-268`). |
| **Process (integration grain)** | `cargo xtask ci --fast` on a clean checkout, offline, no Docker, no credentials (`.redkiln/config.yaml` `integration_scoped`) | AC-009 — the project's non-terminal bar, including the four wasm32 steps and the new lint. This is the merge DoD. |
| **Process (release grain, not run here)** | `cargo xtask ci` | Recorded for completeness: the full gate is HS-P0016's terminal bar. If the `ARTEFACTS` verdict is "take the row", `cargo xtask proof-artefact` runs the named target and this story must confirm it is green with no server. |
| **Process (ledger)** | `redkiln verify --grain story` | That every AC-### in this spec has a `_ledger.md` row flipped to `satisfied: true` with cited evidence, and that no file outside the PR boundary changed (`.redkiln/config.yaml` `require_ledger`, `require_commit_provenance`). |
| **Conformance** | **None, deliberately.** | No store, port or rule is defined or changed here, so nothing this story touches is observable through `happenstance-testkit`'s `suite.rs`. A conformance rule observes a store through `EventStore` and cannot see a markdown file; adding one would be the decorative rule `CLAUDE.md`'s corollary forbids. No `mutation_coverage.rs` `REGISTRY` row is added. |
| **Live infrastructure** | **None added.** | This story *consumes* the two live jobs' output (`_decomposition.md:684-696`) and adds neither a job nor a dependency on one. If a row's evidence is missing because a live job never ran, that is EC-006 and a blocking finding, not a reason to add a job here. |

## Risks and coupling (PR-scoped)

| Risk | Why it is live in this PR | Containment |
| --- | --- | --- |
| **The record is written from markers instead of from runs.** | `cargo xtask spec-trace --write` produces a well-formed table in one command, and it passes every check *necessarily* — §7.1 and the check share one `parse_clauses`. It is the cheapest possible way to produce something that looks like this deliverable. | The store term and the job/run identifier (AC-002) cannot be derived from a marker at all, and `tests::a_row_whose_store_term_names_only_a_type_is_rejected` rejects the nearest fake. |
| **A row is accurate about a run that was not real.** | If the DR-4 mutant landed upstream — `NeonFixture` over a pooled Postgres connection — then ES-11 and ES-12 genuinely passed and are genuinely reported as discharged at an axis that is still empty. | AC-002 requires the *backing*, not the type; EC-006 makes the response a blocking finding rather than a row. This is the last place the lie is recoverable. |
| **The marker promotion gets taken because the evidence is good.** | Here there *is* new evidence, which makes it more tempting rather than less, and the gate would stay green either way. | AC-004 forbids it in the spec (the only place it can be forbidden), and the new marker-agreement assertion makes a later silent promotion go red instead of quietly diverging from the record. |
| **The `adapter-shapes.md` edit shifts a cited subject.** | Three citations reach into that file and `:297` sits three lines below the two rows being edited. | AC-006 requires in-place edits; `spec-trace` is the falsifier (EC-005); the permitted escape is a recorded citation repair. |
| **Scope creep into `spec/SPECIFICATION.md`.** | The file is in the PR boundary for two narrow repairs, and it is the single most consequential file in the tree. | The boundary section names both permitted diffs exhaustively; anything else stops the story and reports. `redkiln verify --grain story` reads the boundary. |
| **The story is written before its inputs exist.** | It is terminal by construction and reports on five slices above it; started early, its rows would be aspirational. | `depends_on: deskeleton-and-package-readiness`, and merge order item 6 (`_storymap.md:262-265`). If an upstream story's evidence is missing, that is EC-006. |
| **The new step becomes decorative.** | A check with no failing test passes forever and teaches the tree that the record is guarded when it is not. | The unit tier is a required deliverable, one named failing case per assertion, following `CLAUDE.md`'s "name a plausible wrong implementation it rejects". |
| **`lint_steps()`'s stale count is copied rather than corrected.** | The doc comment already says "five" over six entries; adding a seventh silently makes it worse. | AC-009 and the implementation notes both name it explicitly. |

**Coupling.** Upstream: every story in slices 2–5 supplies rows, and
`deskeleton-and-package-readiness` must have removed the last `todo!()` before a
row can honestly say `passed`. Downstream: `publication-and-positioning`
(HS-P0016) reads this record for initiative AC-08 and DoD 12, and owns both the
marker promotions and the five residual-exposure edits this story deliberately
leaves undone.

## Dependencies

**Blocks on**

- **`deskeleton-and-package-readiness`** (HS-S0072) — the slice-mate implemented
  in the same context, immediately before this one. Two reasons, and the second is
  the load-bearing one: it is the story that removes the last `todo!()` and the
  scoped `#![allow(clippy::todo)]` from both crates, so until it merges no row may
  honestly read `passed` against either store; and it is the story that reconciles
  `xtask/src/package.rs`'s `PUBLISHABLE`, which is the fact NF-005 asserts is
  unchanged by this PR (`_storymap.md:262-265`).

Implicit and not modelled as story edges, because they are slice-level rather
than story-level: the five slices above supply the evidence transcribed here —
`postgres-live-suite`, `neon-transport`, `position-visibility-decision`,
`neon-live-suite` and `postgres-projection-store`. This story is last in merge
order for exactly that reason.

**Unlocks**

- Nothing inside this project — nothing here consumes the record, which is why the
  archetype is `capability` and not `foundation`.
- **`publication-and-positioning`** (HS-P0016), at the project grain: initiative
  AC-08's requirement that the published compliance claim name its implementations,
  DoD 12's clause-ledger audit, the marker promotions this evidence supports, and
  the five edits that reconcile `RUNBOOK.md:606`'s residual-exposure groups
  (`project.md` *Unlocks*).

## Anchors (progressive disclosure)

Open these at the moment named, not before. Everything needed to *start* is in the
Context pack; these carry the depth a summary cannot.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/project.md` | AC-013's exact wording ("and against which store", `:274-278`) and the *Unlocks* block naming HS-P0016 as the reader are what force the triple rather than a status column; DR-4, DR-5, DR-7 and DR-9 are the constraints the rows and the gate step are written against. | First, before drafting a single row. | AC-001, AC-002, AC-003, AC-009 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/far-end-discharge-record/discover.md` | The signal ledger, the three named wrong implementations (marker-derived record, accurate-about-an-unreal-run, marker promotion) and the two questions this spec answers. The mutants are the test cases the unit tier owes. | Before writing the unit tests for the new check. | AC-002, AC-004, AC-005 |
| `references/adapter-shapes.md` | §5 (`:286-303`) is the "what a skeleton does not prove" table; `:294` and `:295` are the two cells this story is the first entitled to fill, and `:297` is a cited subject that must not move. | Immediately before the §5 edit, with `spec-trace` ready to run. | AC-006 |
| `spec/SPECIFICATION.md` | The nine clauses' live maturity markers, read at implementation time rather than trusted from this spec (`:8547-8550`, `:8592-8594`, `:8623-8624`), and `:8095`, the citation that pins `adapter-shapes.md:297`. | When transcribing each row's prior-marker column, and again before the §5 edit. | AC-004, AC-006 |
| `xtask/src/spec_trace.rs` | `parse_clauses` (`:46`) is the parser the marker-agreement assertion must reuse; the generated-region markers (`:200-201`) are the region never to hand-edit; `check_citations` (`:291-372`) and its twelve-line subject window (`:374-377`) are what the `adapter-shapes.md` edit is constrained by. | Before implementing AC-004's assertion, and before the §5 edit. | AC-004, AC-006 |
| `xtask/src/lints.rs` | The module docs (`:1-24`) state the standing rule that every check documents what it does not verify; `changelog_names_every_rule` (`:525`) and `no_position_literals` (`:628`) are the two file-reading checks the new one is modelled on. | Before writing the new check. | AC-005, NF-002 |
| `xtask/src/main.rs` | `REQUIRED` (`:105`) is Root C, the single definition of what CI runs; `lint_steps()` (`:792-808`) is how the step becomes reachable from `cargo xtask lints`, carries the stale "five checks" sentence, and `steps_named` (`:800-816`) is why the step is selected by name. | When wiring the step, after the check compiles. | AC-005, AC-009, EC-008 |
| `xtask/src/proof.rs` | `ARTEFACTS` (`:133`) and `check` (`:196-238`) — `check` lists the named tests **and then runs the target**, which is the mechanism detail that decides whether phase 10 may take a row without admitting Docker into the default gate. | Before recording the `ARTEFACTS` verdict. | AC-005 |
| `RUNBOOK.md` | Phase 10 in full (`:4311-4390`): the proof artefact at `:4384`, the exit criteria at `:4380-4384`, the empty session log at `:4389`, and the session protocol at `:59-67`. Separately `:606` and `:622-635` (the residual-exposure table and its knowingly-short groups) and `:687-688` (CF-26 and the instrument-portfolio rows this project fills). | When writing the session log, and again when writing the absences. | AC-007, AC-008 |
| `crates/happenstance-testkit/src/contract.rs` | `RuleOutcome::Skipped` (`:26-42`) and `Capability::declined`'s reason (`:374-380`) are why the outcome vocabulary has five values and why a skip's reason is carried verbatim — the reason is printed on every run precisely so the trade stays on the record. | Before fixing the outcome vocabulary in the check. | AC-003 |
| `crates/happenstance-neon/src/lib.rs` | `:10-29` states the transport axis the Neon adapter exists to occupy — the axis a pooled Postgres connection destroys — and `:46-77` is the CTE that keeps `conflicting_position` against the decision ledger's standing assumption, one of the two prior-assumption contradictions DoD 6 asks for. | When writing the Neon rows' store terms, and when writing the §5 transport cell. | AC-002, AC-006 |
| `.kb/decisions/0012-append-shape-and-preconditions.md` | `:97-99` names the exact prohibited act — lifting a maturity marker because a phase wanted it lifted — which is the bar AC-004 enforces and the temptation this story's evidence increases. | Before touching any marker, which is to say: before deciding not to. | AC-004 |
| `.kb/maps/decision-map.md` | The index that resolves ADR-0024 (and any DoD-6 amendment record) to its accepted atom, so the record cites the atom rather than a summary that could disagree with it. | When writing the citations for ES-10's row. | AC-002 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md` | The testing brief's AC-013 row (`:523`, tier **static**), how a skip's reason is read out of `--show-output` (`:517`), and the two live jobs whose logs are this record's raw material (`:684-696`). | Before transcribing outcomes; the jobs' names and invocations are row content. | AC-002, AC-003 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | Persona 4, the evaluator (`:249`), and the *Decide in one sitting* journey the published compliance claim serves — the reason "which store" is a requirement rather than a nicety. | If a row's usefulness to the audit is ever in doubt. | AC-001, AC-008 |
| `.redkiln/config.yaml` | `reachability_static` (`cargo xtask lints && cargo xtask spec-trace`) is the story-grain command the new step must be reachable from; `integration_scoped` is this project's merge bar; `require_ledger` is why every AC needs a cited row. | When wiring `lint_steps()`, and before claiming the story done. | AC-005, AC-009 |

## Clarifications resolved during spec

1. **discover.md Q5 — the record's location and format.** Resolved: a **new
   document at `references/far-end-discharge.md`**, a sibling of
   `adapter-shapes.md` in the tree `CLAUDE.md` calls "evidence kept for citation,
   binding nothing", linked from `RUNBOOK.md` phase 10's session log and from
   `adapter-shapes.md` §5 rather than duplicated into either. Rejected: a
   runbook-only section (the audit would have to know which phase to read); an
   `ARTEFACTS`-row-only answer (it holds test names, not outcomes against stores);
   and a `.kb/` atom (this is evidence, not a settled decision — and hand-authoring
   atoms is the failure `CLAUDE.md` records at `0269720`).
2. **discover.md Q6 — whether phase 10 earns an `ARTEFACTS` row.** Resolved as a
   *rule with a recorded verdict* rather than a guess: take the row **iff** the
   live-suite gating chosen by `postgres-schema-and-live-fixture` leaves the target
   listable and runnable-to-a-pass with no Docker and no network, because
   `proof.rs`'s `check` runs the target and not merely lists it
   (`xtask/src/proof.rs:196-238`); decline it otherwise, naming the gating
   mechanism. Either way the verdict is a line in the record — AC-005 asserts its
   presence, so a silent omission cannot leave the question open a second time.
3. **discover.md Q7 — the `adapter-shapes.md` entries and the session-log text.**
   Resolved into AC-006 and AC-007, with the two prior-assumption contradictions
   named explicitly (the Neon CTE keeping `conflicting_position`; the Postgres
   frontier's structural cost) so DoD 6's "including anything that contradicts a
   prior assumption" is a deliverable rather than a hope.
4. **New mechanism decided here: the record's stated markers are checked against
   the tree.** Nothing in discover.md asked for this. It emerged from AC-004's
   problem — a marker promotion is one word and the gate stays green — and it is
   the one assertion that turns "no marker moved" from a promise in a spec into a
   standing tripwire: after this story, promoting ES-11 without updating the record
   goes red. It reuses `spec_trace::parse_clauses` rather than re-deriving markers,
   because a second parser would be free to disagree with the first.
5. **The AC set is unchanged from the front half.** AC-001 … AC-009 as enumerated
   there; none added, none dropped. The `_ledger.md` rows match one-for-one.
6. **Not resolved here, and deliberately.** Whether ES-11 and ES-12 are promoted
   out of `[PROVISIONAL]`; the five edits reconciling `RUNBOOK.md:606`'s
   residual-exposure groups; whether Persona 4 is promoted as its own persona at
   closeout. The first two are HS-P0016's on this record's evidence
   (`RUNBOOK.md:622-635`, `project.md` *Unlocks*); the third is the initiative's
   closeout question (`initiative.md:252-258`).
