---
item: HS-S0028
stage: spec
created: 2026-08-12T13:46:25.210Z
updated: 2026-08-12T13:46:25.210Z
template_sig: 87bbf1d0
rendered_sig: f1a21e5c
---

# Spec — The polling cost ES-32 imposes, as a number

## Scope lock

| Layer | Path | What it fixes for this story |
| --- | --- | --- |
| Initiative | [`.bklg/from-contract-to-published-library/initiative.md`](../../initiative.md) | DoD 12 (the clause ledger audited at publish: *"no clause is provisional with an empty falsifier"*), `:354-403` |
| Project | [`.bklg/.../typed-layer-and-alpha-release/project.md`](../project.md) | AC-010 (`:197-199`), DR-08 (`:149`), DoD 8 (`:241-242`), *Out of scope* (`:118-126`) |
| This spec | `.bklg/.../typed-layer-and-alpha-release/polling-cost-measurement/spec.md` | the harness's layout, axes, units, result contract and negative requirements |
| Story map | [`_storymap.md`](../_storymap.md) | the M5 row (`:59`), merge order (`:124-126`), *not a story here* (`:141-146`) |
| Architecture brief | [`_decomposition.md`](../_decomposition.md) | AC-010 lands in `experiments/` (`:425`); `head()` compares, never subtracts (`:515-519`) |
| Testing brief | [`_decomposition.md`](../_decomposition.md) | AC-010 is a **Measurement, not a test** — no pass/fail assertion (`:788`) |
| Design | [`_design.md`](../_design.md) | the runner's signature (`:538-570`), its feature gate (`:647`), *polling renders nothing* (`:919`), the `MemoryProjectionStore` input (`:1260-1263`) |
| Discovery | [`discover.md`](discover.md) | the three named mutants (`:91-143`) and the two questions deferred to here (`:47-62`) |
| Roadmap | [`RUNBOOK.md`](../../../../RUNBOOK.md) | `:4028-4030` — *"A number here is what a post-0.1 tail-seam ADR would be argued from"*; the phase that owes the benchmark, `:3953-3955` |
| Specification | [`spec/SPECIFICATION.md`](../../../../spec/SPECIFICATION.md) | ES-32 and its falsifier (`:4021`); CF-34 (`:8263`) |
| Cases | [`spec/E2E-CASES.md`](../../../../spec/E2E-CASES.md) | E2E-32, the fan-out runner the falsifier names (`:826`); *a benchmark harness is not a case* (`:1671-1677`) |

## One-line PR slice

Land a reproducible harness under `experiments/` that records the N views × N reads
cost ES-32 imposes on the polling runner as a **number with its conditions** — outside
the gate by construction (`CLAUDE.md`, repository map; CF-34) — so a post-0.1 tail-seam
decision argues from a measurement rather than an estimate.

## Executive summary

**What this PR lands.** A new directory, `experiments/polling-cost/`, holding a Rust
harness that drives the *real* `happenstance::run_projection` across a swept fan-out of
N projections over one `MemoryEventStore`, and writes one machine-readable record per
swept cell into `results/`, alongside the schema those records conform to, the script
that captures the environment and reproduces them, and a `README.md` that states what
was measured, on what, and — in its own section — what it does not prove.

**The delta this story creates.** ES-32 (`spec/SPECIFICATION.md:4021`) forbids
`EventStore` from growing a tail, subscribe or notify method at 0.1: *consumers poll*.
It carries a `[PROVISIONAL]` marker whose falsifier names a measurement that has never
been taken, and says so in terms — *"the workspace has no benchmark harness and a
conformance rule cannot substitute for one, because complexity is a benchmark and not
an assertion."* Every argument for or against a tail seam in this repository is
therefore currently an estimate. After this PR there is one number, with its conditions,
and a written statement of the distance between that number and the falsifier's own
terms. `spec/E2E-CASES.md:1671-1677` already records the benchmark harness as one of the
two things that are *"not blocked and are not cases"* — this is the story that stops it
being neither.

**What this PR is not.** It is not a gate step, not a workspace member, not a
conformance rule, not an edit to any clause, and not a recommendation about the tail
seam. It produces the evidence a later decision argues from; the decision is
`projection-clause-verdicts`' and HS-P0016's, not this story's.

## Context pack

The load-bearing decisions, stated inline. Everything deeper is a signposted anchor.

**1 — What is actually being measured, and why it is a number rather than an opinion.**
ES-32 makes polling the only mechanism, so a deployment with N views performs N
independent reads of the log to deliver the same events N times. That is the cost. The
falsifier is phrased as *"whether the fan-out runner of E2E-32 can hold N views within
their staleness budget at a measured poll interval on a real deployment"*
(`spec/SPECIFICATION.md:4021`), and E2E-32 (`spec/E2E-CASES.md:826`) is the case that
describes the runner in question. The measurement must speak in those terms — fan-out,
poll interval, staleness — or it does not bear on the clause it was commissioned for.

**2 — The headline number is a COUNT, not a timing.** The primary reported quantity is
**delivery amplification**: events the store yielded across all views, divided by events
the projections actually applied. It is dimensionless, machine-independent, and
reproducible on any laptop, and it is the quantity ES-32 is really about — a tail seam
buys back exactly the redundant deliveries. Wall-clock timings are recorded as
*secondary* figures with the machine that produced them attached, never as the headline.
This is not fastidiousness: CF-34's own falsifier (`spec/SPECIFICATION.md:8263-8270`)
gestures at precisely this substitution — *"an instrumented fixture that counts rows
examined rather than seconds elapsed"* — and discovery's first mutant is a harness that
reports `N=10 views: 3.2ms total`, which is the cost of scanning a `Vec` ten times in
one process and can be waved at from either side of the argument (`discover.md:91-125`).

**3 — Conditions are part of the artefact, not metadata about it.** AC-010 says the cost
is recorded *"with the conditions it was taken under, not as an estimate"*
(`project.md:197-199`). A number without conditions **is** an estimate wearing a decimal
point. Every swept axis and every fixed parameter — fan-out, log size, poll interval,
chunk size, query selectivity, the projection store used, the RNG seed, the toolchain,
the git revision, the build profile, the machine — is written into the result record and
tabulated in the README, in the shape `experiments/position-visibility/README.md:27-55`
already established for this repository.

**4 — Outside the gate BY CONSTRUCTION, not by omission.** `experiments/` is
*"measurements. reproducible, and not in the gate"* (`CLAUDE.md`, repository map), and
CF-34 states performance MUST be measured by a separate harness that MUST NOT be part of
the conformance bar. Mechanically: the harness's own `Cargo.toml` opens with a bare
`[workspace]` table so cargo does not walk up and adopt it as a member — the trick
`experiments/wire-format/Cargo.toml:1-8` documents and `xtask/src/affected.rs:238-242`
already relies on in prose — the root `members` list is unchanged, and no `Step` is added
to `xtask/src/main.rs`'s `REQUIRED` (`:105`) or `OPTIONAL`. Discovery's second mutant is a
gate step with a timing threshold: it passes on the machine it was written on, and its
first red build is resolved by raising the threshold, after which it measures nothing and
blocks everything (`discover.md:127-134`).

**5 — It must drive the REAL runner, not a model of it.** The harness takes a path
dependency on `crates/happenstance` with `unstable-projection` enabled and calls
`happenstance::run_projection` as published (`_design.md:538-570`, `:647`). A harness
that re-implements the poll loop measures the harness. This has a second consequence
worth taking deliberately: this story is the **first consumer of `run_projection` from
outside the workspace**, so if the signature is awkward to hold — the `&mut P`, the
`NonZeroUsize` chunk, the two-parameter error type — this is where that is discovered.
Report it as an AC-012 defect-log entry rather than working around it locally.

**6 — Staleness is measured in time and in events observed, never as `head() -
checkpoint`.** `EventStore::head`'s own documentation forbids the subtraction
(`crates/happenstance-core/src/store.rs:235-245`): positions are an opaque ordering key
and the specification permits gaps, so the difference is not a count of anything. Against
`MemoryEventStore` the subtraction happens to be right, which is what makes it survive
review; against any store that leaves gaps it reports a backlog that does not exist
(`_decomposition.md:515-519`). This story reports staleness as (a) nanoseconds between an
event's append instant and the instant a view's checkpoint reaches or passes it, and
(b) the count of events a view had yielded to it but not yet applied at a poll boundary —
both observed, neither inferred.

**7 — The projection store question, decided.** `MemoryProjectionStore` is HS-P0010's
AC-012 and does not exist in this tree (`_design.md:1260-1263`; `_storymap.md:141-146`).
The decision: **use it if it has landed by implementation time; otherwise define a
harness-local `ProjectionStore` inside `experiments/polling-cost/src/`, and never under
`crates/`.** The storymap's prohibition is against *shipping* a fixture whose shape this
project does not own — an experiment crate that is not a workspace member, not published
and not in the gate ships nothing. Which store was used is a recorded condition of the
run, not an implementation detail. The harness-local store is deliberately near-zero-cost
(an in-memory counter per view), because the quantity under study is the redundant *read*
fan-out and read-model work would dilute it — which is a design choice **and** a stated
limitation, and belongs in both places in the README.

**8 — The honest artefact is a floor with a named gap.** ES-32's falsifier says *"on a
real deployment"*. There is none: all six adapter crates carry `publish = false` and none
has run the conformance suite (`CLAUDE.md`, repository map), and `MemoryEventStore` is an
in-process `Vec` (`project.md:121-124`). An in-process number bounds the cost from below
and is the first number that exists, which is worth having — but the README must name,
explicitly, the terms of the falsifier this measurement does not reach: *a real
deployment* (no network, no storage latency, no second process) and *their staleness
budget* (nobody has stated one, so the harness reports staleness and judges nothing). The
sibling's `## 6. What none of this proves`
(`experiments/position-visibility/README.md:345-367`) is the section shape to follow. A
floor with a stated gap is evidence; a floor presented as a verdict is the first mutant
wearing better clothes.

**9 — No marker moves and no clause is edited here.** ES-32 stays `[PROVISIONAL]`. This
story supplies evidence toward its falsifier and leaves the disposition to
`projection-clause-verdicts` and HS-P0016. Nothing under `crates/happenstance-core/src/**`
and nothing in `spec/SPECIFICATION.md` is touched (`discover.md:157-161`).

**10 — The persona-journey slice.** Beat 4 of *"Choose a contract before a database"*
(`_storymap.md:27`) is reading events back into a read model. The design has already
refused to make the polling cost visible as motion — no spinner, no progress indicator,
no in-place rewrite, because *"its cost is a number in `experiments/`, not a
performance"* (`_design.md:181-183`, `:919`). The audience for this artefact is therefore
not the application author at all: it is whoever opens `RUNBOOK.md`'s post-0.1 tail-seam
question six months from now and needs something to argue from (`project.md:149`,
DR-08; `RUNBOOK.md:4028-4030`). Write the README for that reader.

## Integration contract

- **Archetype**: `capability`. The deliverable is user-observable in the only sense
  available to it — a maintainer runs one command and gets a number — but it adds no
  public item to any published crate.
- **Slice / milestone**: **M5 `projection-runner`**. Slice-mates:
  [`projection-trait-and-runner`](../projection-trait-and-runner/spec.md) (hard
  dependency — there is nothing to measure until a polling runner exists) and
  [`projection-clause-verdicts`](../projection-clause-verdicts/spec.md) (independent;
  either order, `_storymap.md:124-126`).
- **Mount point**: **`experiments/polling-cost/Cargo.toml`** — the manifest is this
  story's composition root. It is where the harness binds to the real
  `crates/happenstance` by path with `features = ["unstable-projection", …]`, and it is
  simultaneously where the harness is held *out* of the workspace by its bare
  `[workspace]` table. Both facts live in one file, which is why that file and not
  `src/main.rs` is the mount. The binary entry point is
  `experiments/polling-cost/src/main.rs`; the human entry point is
  `experiments/polling-cost/run.sh`.
- **Wires into** (real siblings, by path):
  - `crates/happenstance/` — `happenstance::run_projection` and `happenstance::Projection`
    behind `unstable-projection` (`_design.md:538-570`), and `happenstance::Codec` for
    decode (`_design.md:647`).
  - `crates/happenstance-core/src/store.rs` — `EventStore` (**not** `SendEventStore`;
    `CLAUDE.md` binding constraint 4) and `MemoryEventStore` behind `memory`.
  - `crates/happenstance-core/src/projection.rs` — the `ProjectionStore` port whose
    `begin` (`:117`) / `commit` (`:126`) pair the harness's store must honour, and whose
    inclusive-`from` resume rule (`:104-106`) the runner already obeys.
  - `experiments/position-visibility/` — the layout, environment-capture and
    `README.md` section shape this harness copies (`run.sh`, `container/env.sh`,
    `schema/`, `results/`).
  - `experiments/wire-format/Cargo.toml` — the precedent for a Rust experiment crate
    outside the workspace, including the bare `[workspace]` table and its rationale.
- **Renders surfaces**: **none.** `_design.md`'s three surfaces are
  `crate-root-rustdoc`, `worked-example-transcript` and `dsl-failure-message`, and this
  story renders none of them. That is the design's own decision, not an omission: the
  runner's polling has no loading state because *"polling renders nothing… its cost is a
  number in `experiments/`, not a performance"* (`_design.md:919`). Adding a progress
  indicator here would contradict a signed-off rejection (`_design.md:179-182`).
- **Public items**: **none added or changed.** No row of `_design.md`'s `## Items` block
  is claimed by this story. It is a *consumer* of `happenstance::Projection` (`:320-324`)
  and `happenstance::run_projection` (`:326-330`), which `projection-trait-and-runner`
  implements — and the first one from outside the workspace, which makes it an unplanned
  usability check on those two signatures.
- **Conformance rule(s)**: **none, and none is possible.** CF-34 states performance MUST
  be measured by a separate harness that MUST NOT be part of the conformance bar, and its
  own audit row calls it *"correct as a clause, and self-referentially so: a rule
  enforcing it would violate CF-33"* (`spec/SPECIFICATION.md:8774`). `spec/E2E-CASES.md:1671-1677`
  records the benchmark harness as neither blocked nor a case for the same reason. This
  story changes no port, so it owes no rule.
- **Clause(s)**: **ES-32** (`spec/SPECIFICATION.md:4021`) — evidence is supplied toward
  its falsifier; the `[PROVISIONAL]` marker is **not** moved and the clause text is not
  edited. **CF-34** (`:8263`) — obeyed, by keeping the harness out of the bar. No
  `[FROZEN]` clause is touched, so no new ADR is required.
- **Advances DoD scenario**: initiative **DoD 12** — *"The clause ledger is audited at
  publish… no clause is provisional with an empty falsifier"* (`initiative.md:393-395`).
  ES-32's falsifier currently names a benchmark that does not exist and says so inline;
  this story is what makes that parenthetical false. It does not close DoD 12 by itself —
  the audit run and the count reconciliation are HS-P0016's — and it is the only DoD
  scenario this story moves.

## PR boundary

`redkiln verify --grain story` reads the first fenced block under this heading and fails
on any file changed outside it. Note what is *not* here: the root `Cargo.toml`,
`xtask/**`, `crates/**` and `spec/**` are all excluded, and the mount point named above
sits **inside** the boundary. That is what "outside the gate by construction" looks like
expressed in git — a correct implementation of this story cannot touch a file the gate
compiles.

```
experiments/polling-cost/**
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/polling-cost-measurement/**
```

**In this PR**

- `experiments/polling-cost/Cargo.toml` — bare `[workspace]`, `publish = false`, path
  dependency on `crates/happenstance` with `unstable-projection`, plus the async runtime
  and the serialisation the result writer needs.
- `experiments/polling-cost/src/` — the sweep driver, the N `Projection` implementations
  under measurement, the instrumented counters, and (only if HS-P0010's
  `MemoryProjectionStore` has not landed) the harness-local `ProjectionStore`.
- `experiments/polling-cost/run.sh` — the reproducer: capture the environment, build
  `--release`, run the sweep, write `results/`.
- `experiments/polling-cost/schema/` — the result-record schema (see *Data and
  migrations*).
- `experiments/polling-cost/results/` — the committed raw output of the recorded run.
- `experiments/polling-cost/README.md` — what was measured on what, the numbers, what it
  does not prove, the verdict, how to re-run, and the layout.
- This story's own backlog folder (`_ledger.md`, the implementation report).

**Explicitly not in this PR**

- Any change to the root `Cargo.toml` `members` list, to `[workspace.dependencies]`, or
  to any `crates/**/Cargo.toml`.
- Any `Step` in `xtask/src/main.rs` (`REQUIRED` or `OPTIONAL`), any threshold, any
  assertion about the cost.
- Any edit to `spec/SPECIFICATION.md` — including moving ES-32's marker or amending its
  falsifier. That is `projection-clause-verdicts`' and HS-P0016's.
- The `Projection` trait and `run_projection` themselves — `projection-trait-and-runner`'s.
- A second run against a durable adapter. ES-32's *"real deployment"* term is named as
  unreached here and belongs to HS-P0012 (`discover.md:52-59`).
- The project closeout paragraph that carries the number (DoD 8, `project.md:241-242`).
  This story produces the artefact and its quotable headline; the closeout cites it.

**Merge DoD.** `bash experiments/polling-cost/run.sh` completes from a clean checkout and
regenerates `results/` against the committed schema; the README's headline amplification
figure and its *what this does not prove* section are both present; `cargo xtask ci` and
`cargo xtask affected --base main` are unchanged in behaviour by this diff, the latter
still selecting no package.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path | AC |
| --- | --- | --- | --- |
| One command reproduces the run | `bash experiments/polling-cost/run.sh` from a clean checkout builds `--release`, captures the environment, runs the full sweep and writes `results/`. Optional single-cell and single-arm invocations, and a suffix knob so a re-measurement does not overwrite the pass it is being compared with (`HS_TAG`'s role in the sibling). Requires a Rust toolchain and nothing else — no Docker, no database. | `experiments/position-visibility/run.sh:1-12`, `README.md:424-461` | AC-001 |
| The harness drives the real runner | `experiments/polling-cost/Cargo.toml` takes `happenstance = { path = "../../crates/happenstance", features = ["unstable-projection", …] }` and the sweep calls `happenstance::run_projection(events, models, projection, codec, chunk)` as signed off. No local re-implementation of the poll loop, and generic code binds `EventStore`, never `SendEventStore`. | `_design.md:538-570`, `:647`; `CLAUDE.md` binding constraint 4 | AC-001 |
| The headline number is delivery amplification | Per swept cell: `events_delivered` (summed across views) ÷ `events_applied`. Dimensionless and machine-independent. Timings are recorded per repeat as p50/p95 nanoseconds **with the machine attached** and are explicitly secondary in the README. | `spec/SPECIFICATION.md:8263-8270` (CF-34's counting falsifier); `discover.md:108-118` | AC-002 |
| The swept axes, fixed here | **Fan-out** N ∈ {1, 2, 4, 8, 16, 32} views. **Log size** ∈ {1 000, 10 000, 100 000} events. **Poll interval** ∈ {10 ms, 100 ms, 1 000 ms}. **Selectivity arm**: *overlapping* (every view's query selects the whole log — ES-32's worst case, and the arm the tail-seam argument turns on) and *disjoint* (each view selects ≈1/N). **Fixed and recorded**: chunk size, repeats per cell, RNG seed, codec. Ranges may be trimmed for runtime if the trim is recorded in the README with its reason; the *shape* of the sweep may not be. | discovery deferred the ranges to spec (`discover.md:49-51`) | AC-002, AC-003 |
| Every condition is recorded per record | Each result record carries: fan-out, log size, poll interval, chunk, selectivity arm, repeat index, seed, the projection store used, feature set, `rustc -vV`, `cargo -V`, git revision + dirty flag, build profile, OS/kernel, CPU model, logical CPU count, RAM, and the run's start instant. The README's §1 tabulates the fixed ones in the sibling's format. | `project.md:197-199`; `experiments/position-visibility/README.md:27-55` | AC-003 |
| Staleness is observed, never subtracted | Reported as (a) nanoseconds from an event's append instant to the instant the observing view's checkpoint reaches or passes that event, and (b) the count of events yielded to a view but not yet applied at a poll boundary. `head() - checkpoint` appears nowhere in the harness — not in the code, not in the README, not in a derived column of a result file. | `crates/happenstance-core/src/store.rs:235-245`; `_decomposition.md:515-519`; `discover.md:136-143` | AC-004 |
| The projection store is chosen, not improvised | Prefer HS-P0010's `MemoryProjectionStore` if present; otherwise a harness-local `ProjectionStore` under `experiments/polling-cost/src/`, honouring `begin`/`commit` atomicity, deliberately near-zero-cost, and **never** added under `crates/`. Whichever was used is a field in every record and a row in the README's conditions table, with the dilution rationale stated. | `_design.md:1260-1263`; `_storymap.md:141-146`; `crates/happenstance-core/src/projection.rs:117,126` | AC-003, AC-005 |
| Results are machine-readable and schema-bound | One JSON object per cell-repeat, newline-delimited, under `results/`, conforming to the schema committed in `schema/`. A run manifest header record carries the environment block once. See *Data and migrations*. | `experiments/position-visibility/schema/`, `results/` | AC-005 |
| The README states the gap in its own section | A `## What this does not prove` section names, by the falsifier's own words, the terms not reached: *"on a real deployment"* (no durable adapter exists; in-process `Vec`, one process, no network, no storage latency) and *"their staleness budget"* (none has been stated, so staleness is reported and not judged). The word **floor** appears. The verdict section quotes the amplification figure and stops there. | `spec/SPECIFICATION.md:4021`; `experiments/position-visibility/README.md:345-367`; `project.md:121-124` | AC-006 |
| Outside the gate, mechanically | Bare `[workspace]` in the harness manifest; root `Cargo.toml` `members` unchanged; no `Step` added to `xtask/src/main.rs`; no dependency added to any `crates/**` manifest or to `[workspace.dependencies]`. `cargo xtask affected --base main` over a diff touching only `experiments/` still selects no package, which `is_inert`'s `"experiments/"` prefix already guarantees and this story must not disturb. | `CLAUDE.md` repository map; `spec/SPECIFICATION.md:8263`; `xtask/src/affected.rs:238-258`, `:658-667`; `experiments/wire-format/Cargo.toml:1-8` | AC-007 |
| No assertion, no threshold, no verdict on the seam | The harness has no pass/fail path: it exits non-zero only if the harness itself fails, never because a number was large. The README recommends neither adding nor withholding a tail seam. ES-32's marker is not moved and `spec/SPECIFICATION.md` is not edited. | `_decomposition.md:788`; `discover.md:127-134`, `:157-161`; `project.md:149` | AC-008 |
| Signature friction is reported, not absorbed | If holding `run_projection` from outside the workspace is awkward, the finding is written as an AC-012 defect-log entry naming the item, not patched around locally and not fixed by editing `crates/happenstance`. | `project.md` AC-012; `_storymap.md:152-154` | AC-008 |

**Non-goals of the interface.** The harness exposes no library API — it is a binary and a
script. Nothing in `experiments/polling-cost/` is `pub` for a consumer's benefit, nothing
is documented as stable, and no item here is re-exported from any crate.

## Data and migrations

**No database, no schema migration, no persistent store.** The only durable data this
story creates is its own result corpus, and it has a contract:

| Concern | Decision |
| --- | --- |
| Format | Newline-delimited JSON under `experiments/polling-cost/results/`. One object per cell-repeat, plus a single leading manifest object carrying the environment block so it is not repeated on every row. |
| Schema | A JSON Schema committed at `experiments/polling-cost/schema/`, in the role `experiments/position-visibility/schema/` plays for its SQL arms: the results are only reproducible if the thing they must conform to is in the tree beside them. |
| Versioning | Every record carries `schema_version`. Changing the record shape bumps it; it never silently re-interprets an existing file. |
| Re-runs | Additive. A new pass writes new files under a run tag rather than overwriting a committed pass — the sibling's `HS_TAG` exists precisely so *"a re-measurement does not overwrite the pass it is being compared with"* (`experiments/position-visibility/README.md:454-456`). Committed results are the record of a run that happened and are never edited in place. |
| Determinism | Any randomisation (tag assignment, event ordering within an arm) is seeded, and the seed is **written into the file**, not described in prose — `experiments/wire-format/README.md:13-16` names this as the failure the whole `experiments/` discipline exists to prevent. |
| Size | Raw per-repeat rows are committed. If the sweep's full output would be unreasonably large, the *sweep* is trimmed and the trim recorded — the output is never post-processed down to summaries, because a summary cannot be re-analysed and this artefact's audience is a reader six months out. |

No workspace `Cargo.lock` is touched: the harness resolves its own lockfile inside
`experiments/polling-cost/`, which is a consequence of the bare `[workspace]` table and
is the same property `experiments/wire-format/` already has
(`experiments/wire-format/README.md:27-28`).

## Acceptance criteria

Three readers matter here, and only two of them are people who run anything. **R1, the
future arguer** — whoever opens `RUNBOOK.md`'s post-0.1 tail-seam question six months
from now and needs something other than an estimate (`RUNBOOK.md:4028-4030`;
`project.md:149`, DR-08). **R2, the reproducer** — the maintainer who wants to know
whether the number still holds, on their machine, today. **R3, the application author** —
explicitly *not* served by this artefact, because the design already decided the polling
cost is not rendered to them (`_design.md:919`); an AC that made the cost visible in the
library's own output would contradict a signed-off rejection.

Every criterion below is a behaviour of the shipped artefact, and every verification
column names a real path. Note the tiering: these instruments live *inside*
`experiments/polling-cost/` and are run by `cargo test --manifest-path
experiments/polling-cost/Cargo.toml`. They are tests **of the harness**, never of a cost,
and they are not in the gate — see *Tests and CI*.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** R2 has a clean checkout and a Rust toolchain — no Docker, no database, no network — **WHEN** they run `bash experiments/polling-cost/run.sh`, **THEN** the harness builds `--release`, captures the environment, drives the **real** `happenstance::run_projection` behind `unstable-projection` across the declared sweep, and writes a complete pass into `results/`; **AND** it re-implements no poll loop of its own, and every generic function in it binds `EventStore`, never `SendEventStore`. | `experiments/polling-cost/tests/one_cell.rs` — drives one cell through the *same* entry point the sweep uses and asserts every seeded event reached every view *through* `run_projection` (a local loop makes the assertion fail to compile against the feature-gated import). Plus a clean-checkout `run.sh` transcript in the implementation report. |
| **AC-002** | **GIVEN** R1 opens the artefact looking for the cost ES-32 imposes, **WHEN** they read the first number, **THEN** it is **delivery amplification** — events the store yielded across all views ÷ events the projections applied — a dimensionless count, not a duration; **AND** it is reported over the declared grid (fan-out × log size × poll interval × selectivity arm, both the *overlapping* and *disjoint* arms present), with wall-clock timings recorded only as secondary figures carrying the machine that produced them. | `experiments/polling-cost/tests/amplification.rs` (hand-built cell with known delivered/applied counts; asserts the record's headline field is that ratio and is not a duration type) and `experiments/polling-cost/tests/sweep_shape.rs` (asserts the enumerated grid contains every declared axis value and both selectivity arms). |
| **AC-003** | **GIVEN** R1 finds a figure they want to judge the relevance of, **WHEN** they look for what it was taken under, **THEN** every condition is *in the record itself* — fan-out, log size, poll interval, chunk, arm, repeat index, seed, projection store used, feature set, `rustc -vV`, `cargo -V`, git revision + dirty flag, build profile, OS, CPU model, logical CPUs, RAM, run instant — **AND** the fixed ones are tabulated in the README's §1 in the shape the sibling already uses, so the judgement can be made without opening a JSON file. | `experiments/polling-cost/tests/record_conditions.rs` — asserts every required field is present and non-empty on every emitted record, that the manifest record appears exactly once per pass, and that committed passes carry `git_dirty: false`. Format precedent: `experiments/position-visibility/README.md:27-55`. |
| **AC-004** | **GIVEN** R1 reads a staleness figure, **WHEN** they ask what it counts, **THEN** it is something that was *observed* — nanoseconds from an event's append instant to the instant an observing view's checkpoint reaches or passes it, and the count of events yielded to a view but not yet applied at a poll boundary — **AND** `head() - checkpoint` appears nowhere: not in the harness code, not in a README column, not in a derived field of a result file, because positions are an opaque ordering key that may have gaps. | `experiments/polling-cost/tests/staleness.rs` — the observer is fed a **gapped** position sequence (`happenstance_testkit::GappyMemoryStore` if M4 has merged, otherwise the harness-local gapped wrapper) and must report the same figures it reports for a dense sequence with the same delivery pattern; a subtraction-based implementation reports a different, larger number and fails. Backed by a source-text assertion in the same file that no `head()` value participates in arithmetic. |
| **AC-005** | **GIVEN** R2 wants to re-analyse the pass rather than re-read the prose, **WHEN** they open `results/`, **THEN** they find newline-delimited JSON conforming to a schema committed beside it, every record carrying `schema_version` and the seed that produced it — **AND** the projection store the run used is one of exactly two named options (HS-P0010's `MemoryProjectionStore` if it has landed, else a harness-local store under `experiments/polling-cost/src/` honouring `begin`/`commit` atomicity and **never** added under `crates/`), recorded as a field on every record and as a row in the README's conditions table with its dilution rationale stated. | `experiments/polling-cost/tests/schema.rs` — every line of every committed file under `results/` validates against `experiments/polling-cost/schema/result-record.schema.json` and the `schema_version` matches the harness's own constant; the store-identity field is asserted to be one of the two named values, never absent or free text. |
| **AC-006** | **GIVEN** R1 is deciding whether this number bears on ES-32's falsifier, **WHEN** they reach the README's own `## What this does not prove` section, **THEN** it names, in the falsifier's own words, the terms this measurement does **not** reach — *"on a real deployment"* (no durable adapter exists; one process, an in-process `Vec`, no network, no storage latency) and *"their staleness budget"* (none has been stated, so staleness is reported and not judged) — **AND** the word **floor** appears, **AND** the verdict section quotes the amplification figure and stops there, so a floor cannot be mistaken for a verdict. | `experiments/polling-cost/tests/readme.rs` — asserts the section exists as its own `##` heading, contains both quoted falsifier terms and the word *floor*, and — the strong check — that the headline amplification figure printed in §1 equals the figure recomputed from the committed `results/`, so prose and corpus cannot drift apart. Shape precedent: `experiments/position-visibility/README.md:345-367`. |
| **AC-007** | **GIVEN** a contributor who has never heard of this experiment runs the repository's gate, **WHEN** `cargo xtask ci` and `cargo xtask affected --base main` execute over a diff that touches only `experiments/polling-cost/**`, **THEN** nothing changes: no package is selected, no step is added, the root `Cargo.toml` `members` list and `[workspace.dependencies]` are untouched, no `crates/**` manifest gains a dependency, and the harness's own manifest opens with a bare `[workspace]` table carrying the comment that says why. | `experiments/polling-cost/tests/gate_inertness.rs` — reads the two manifests as files and asserts the bare `[workspace]` table is present in the harness's and that the root `members` list mentions no `experiments/` path. Plus `cargo xtask affected --base main` on the story branch selecting no package (`xtask/src/affected.rs:238-258`), and `cargo xtask ci --fast` green with an unchanged step list. |
| **AC-008** | **GIVEN** the harness produces a number a reader dislikes, **WHEN** the run completes, **THEN** nothing fails and nothing is recommended: the harness exits non-zero only when *it* failed to produce a number, never because a number was large; no threshold exists anywhere; the README recommends neither adding nor withholding a tail seam; ES-32 keeps its `[PROVISIONAL]` marker and `spec/SPECIFICATION.md` is not edited — **AND** if holding `run_projection` from outside the workspace proved awkward, that finding is written as a **project** AC-012 defect-log entry naming the item, not patched around locally. | `experiments/polling-cost/tests/no_verdict.rs` — drives a cell whose amplification is absurdly high and asserts the run still returns `Ok` and writes a record; asserts no threshold constant exists in the harness. Plus `git diff --name-only main` showing no `spec/**`, `xtask/**` or `crates/**` path (the same check `redkiln verify --grain story` makes against the PR boundary). |
| **AC-009** | **GIVEN** R1 opens this artefact cold, six months out, in a terminal and in a text editor, **WHEN** they read it top to bottom, **THEN** it is *composed*, not dumped: the README carries the sibling's section order with the headline amplification figure and the conditions table above any per-cell detail, timings never appearing before the ratio they qualify; tables overflow with `… and N more` rather than reflowing; **AND** the harness's own console output is plain — whole `\n`-terminated lines, no ANSI colour, no spinner, no percentage, no carriage-return rewrite, nothing past 80 columns, no assumption of a TTY, so the run is legible piped to a file. | `experiments/polling-cost/tests/readme.rs` (section headings present and in the declared order; the amplification figure precedes the first timing figure in the file; no table region exceeds 8 rows without the `… and N more` marker) and `experiments/polling-cost/tests/output_format.rs` (progress writer output contains no `\x1b`, no `\r`, no `%`-progress token, and no line longer than 80 columns). Design authority: `_design.md:810-836` (transience), `:911-928` (overflow/narrow-viewport), `:969-1013` anti-patterns 9 and 10. |
| **AC-010** | **GIVEN** R2 re-measures after the runner has changed, **WHEN** they run the harness again, **THEN** the committed pass they are comparing against is still there, byte-identical, and still cited by the README — the new pass is written under a new run tag beside it; **AND** re-running with a tag that already exists is refused with a message naming the existing pass rather than silently overwriting it, so a re-run is undone by deleting one directory and nothing is ever edited in place. | `experiments/polling-cost/tests/rerun_is_additive.rs` — writes a pass, re-runs with the same tag and asserts the refusal (and that the original bytes are unchanged), then re-runs with a new tag and asserts both passes exist independently. Precedent: `HS_TAG` in `experiments/position-visibility/README.md:454-456`. |

**Coverage of the traced project AC.** Project **AC-010** — *"the N views × N reads cost
ES-32 imposes is recorded as a number with the conditions it was taken under, not as an
estimate"* (`project.md:197-199`) — decomposes exactly here: **AC-002** is *the number*,
**AC-003** is *the conditions*, **AC-005** is the machine-readable form that keeps them
attached, **AC-006** is what stops it being read as more than it is, and **AC-001** /
**AC-007** are the two halves of *reproducible and not in the gate*. AC-004, AC-008,
AC-009 and AC-010 are the three named mutants and the artefact's own legibility. No other
project AC is claimed by this story. (Where this document says **AC-012**, it means the
*project's* AC-012 defect log, not a story criterion — this story enumerates AC-001
through AC-010 only.)

## Interaction quality

This story renders **none** of `_design.md`'s three surfaces (`crate-root-rustdoc`,
`worked-example-transcript`, `dsl-failure-message`) and adds no public item, so the
design's surface-level composition rules do not bind it by mount. Two things about it are
nevertheless *presentation*, and both are load-bearing: the README a future reader
actually reads, and the console output of `run.sh`. The design's terminal conventions
govern the second directly — they are conventions about this repository's terminal
output, and a harness is not exempt from them because it lives under `experiments/`.

Every invariant below is carried by a **row in the acceptance-criteria table above**.
This section says only which row carries which, and how it fails.

**State invariants.**

| Invariant | Carried by | How it is verified, and what failing looks like |
| --- | --- | --- |
| In place, not a context jump | **AC-001** | One shell command, one toolchain, no container and no second tool — unlike the sibling, which needs Docker (`experiments/position-visibility/run.sh:1-12`). Failure: a reproducer must install a database before they can see a number. |
| Non-occlusion | **AC-010**, **AC-006** | A new pass never hides the pass it is being compared with; the *what this does not prove* section is its own `##` heading rather than a footnote under the verdict. Failure: `results/` has one directory that means "whatever was run last". |
| Preserved selection — the reader's citation survives | **AC-010** | `tests/rerun_is_additive.rs` asserts prior files are byte-identical after a re-run, so a `file:line` citation into a committed pass still resolves. Failure: a cited row moved and the citation now points at a different number. |
| Reversibility | **AC-010**, **AC-007** | A re-run is undone by deleting one tagged directory; the whole story is undone by deleting `experiments/polling-cost/`, because it touches nothing the gate compiles. Failure: reverting the experiment requires reverting a `members` list or an `xtask` step. |
| Reachability without a TTY | **AC-009**, **AC-001** | Plain `\n`-terminated lines, no cursor control, so the run is legible piped to a file or read from CI logs. Failure: the transcript is unreadable unless watched live. |

**Composition invariants** — sourced from the signed-off design, applied to this
artefact's two rendered things.

| Invariant | Design authority | Carried by |
| --- | --- | --- |
| Presentation exists at all — the artefact is composed, not a dump of numbers | `_design.md:761-809` (composition) | **AC-009**, **AC-003** (conditions as a table in the sibling's format, not prose) |
| Placement — the gap section is a peer of the verdict, not a footnote to it | `experiments/position-visibility/README.md:345-367` | **AC-006** |
| Transience — persistent: the headline figure and the conditions table; revealed: the per-cell tables further down the README; opened on demand: the raw `results/` corpus and its `schema/` | `_design.md:810-836` | **AC-009**, **AC-005** |
| Density budget, with its real numbers — 80 columns for every console line and every fence; ≤ 8 rows per table region before `… and N more`; the README's section count matches the sibling's eight | `_design.md:837-872`, `:911-928` (overflow, narrow viewport) | **AC-009** |
| Hierarchy — the dimensionless ratio outranks every millisecond; no timing figure appears above the amplification figure it qualifies | `_design.md:873-910` | **AC-009**, **AC-002** |
| Anti-pattern 9 — *no colour, spinner, percentage, or line that rewrites in place* | `_design.md:969-1013` (item 9) | **AC-009** |
| Anti-pattern 10 — *no line past 80 columns, nothing truncated to fit* | `_design.md:969-1013` (item 10) | **AC-009** |
| The design's own refusal — polling renders **nothing** to the application author; its cost is a number in `experiments/`, not a performance | `_design.md:919`, `:179-183` | **AC-009** (negatively: no progress affordance is added anywhere, here or in the library) |

Anti-patterns 1–8 and 11–15 govern the crate-root page, the README of the published
crate, the worked-example transcript, the DSL failure message and the compile-fail
diagnostic. This story renders none of those and must not touch them.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | `run.sh` invoked without a working `cargo`/`rustc` | Fail before writing anything, naming the missing tool. No partial `results/` directory is left behind — a half-written pass that looks complete is worse than no pass. |
| **EC-002** | A swept cell fails (panic, allocation failure at the largest fan-out × log size) | Record the cell as failed **with its reason** in the manifest, continue the remaining cells, and exit **non-zero at the end**. Non-zero here means *the harness did not produce a number*, which is the only failure mode AC-008 permits — it is never a reaction to the size of a number. |
| **EC-003** | A run tag that already exists under `results/` | Refuse, naming the existing pass and the tag to use instead. Never overwrite (AC-010). |
| **EC-004** | The working tree is dirty when the environment is captured | Record `git_dirty: true` and warn loudly; do not refuse — the record must describe the run that happened. A *committed* pass with `git_dirty: true` fails `tests/record_conditions.rs`, so an exploratory run cannot be mistaken for the recorded one. |
| **EC-005** | `crates/happenstance` does not expose `run_projection`, or `unstable-projection` has been renamed | Fail at build time from the manifest and the import. There is no fallback path to a locally written poll loop — a harness that silently measures itself is mutant 1 (`discover.md:91-125`). |
| **EC-006** | A produced record does not validate against `schema/` | Fail the run and say which record and which field. An unvalidated corpus is not re-analysable, which is the whole reason the schema is committed beside the results. |
| **EC-007** | HS-P0010's `MemoryProjectionStore` is present but its shape differs from what the harness expects | Fall back to the harness-local store and record the substitution as the store-identity field — never vendor a copy under `crates/`, and never edit HS-P0010's fixture from this story (`_storymap.md:141-146`). |

## Non-functional

| id | requirement | why, and how it is judged |
| --- | --- | --- |
| **NF-001** | The committed sweep completes in **≤ 20 minutes** single-threaded on a developer laptop. | A harness nobody re-runs is prose. If the declared grid exceeds the budget, the *ranges* are trimmed and the trim is recorded in the README with its reason; the sweep's *shape* — both selectivity arms, all four axes — is not trimmed (AC-002). |
| **NF-002** | Prerequisites are a Rust toolchain and nothing else. No Docker, no database, no network. | Deliberately weaker than `experiments/position-visibility/`, whose Postgres arms need a container. It is also exactly why this number is a floor and not a deployment figure (AC-006). |
| **NF-003** | The committed `results/` corpus stays under a few megabytes. | Raw per-repeat rows are committed and are never post-processed down to summaries; if the corpus would be unreasonably large the sweep is trimmed instead (*Data and migrations*). A summary cannot be re-analysed by a reader six months out. |
| **NF-004** | Re-running the same tag'd sweep on the same machine reproduces the amplification figure **exactly**; timings reproduce within a band the README states. | The headline being a count is what makes exact reproduction possible, and is the point of choosing a count over a timing (`spec/SPECIFICATION.md:8263-8270`). |
| **NF-005** | This story adds **zero seconds** to `cargo xtask ci` and `cargo xtask ci --fast`. | AC-007's mechanical consequence. Measured by the step list being unchanged, not by a stopwatch. |
| **NF-006** | The harness builds on the toolchain pinned in `rust-toolchain.toml` (1.97.1) and makes **no MSRV claim**. | It is not a workspace member and is not published, so `cargo hack --rust-version` never sees it and it constrains nothing downstream (`.kb/decisions/0029-msrv-raised-to-1-97-1.md`). It may use any stable feature the pin allows — including let-chains — without an ADR. |
| **NF-007** | The harness's own dependencies are its own. Nothing it needs is added to the root `[workspace.dependencies]` or to any `crates/**` manifest. | Dependency hygiene for a crate outside the gate: an experiment must not be able to grow the published graph (`standards/rust/50-dependency-hygiene.md`; AC-007). |

## Implementation notes (non-prescriptive)

Shape suggestions, not requirements. The ACs are the contract.

- **Copy the sibling's skeleton, drop what does not apply.**
  `experiments/position-visibility/` gives the layout for free: `README.md`, `run.sh`,
  `schema/`, `results/`. Its `container/` has no analogue here (NF-002) — capture the
  environment inline in `run.sh` and write it to `results/<tag>/environment.txt` *and*
  into the manifest record, so the corpus is self-describing even when detached from
  the directory.
- **One call site for `run_projection`.** Put the single invocation in one small module
  (`src/cell.rs` is the obvious name). It is the file that churns if
  `projection-trait-and-runner`'s signature moves, and it is the file to read when
  answering *"was this actually measured against the real runner?"* — which is AC-001's
  whole content and the first thing a sceptical reader will check.
- **Count at the boundary, not inside the projection.** `events_delivered` is what the
  store yielded to each view's stream; `events_applied` is what each projection's
  `apply` accepted. Counting inside a projection makes amplification 1.0 by
  construction, which is a wrong implementation that looks like a working one.
- **The overlapping arm is the one the argument turns on.** Every view selecting the
  whole log is ES-32's worst case; the disjoint arm exists so the ratio has a control
  to be read against. If time forces a trim, trim log sizes or repeats, never an arm.
- **Two clocks, and say which.** Append instant and apply instant should come from a
  monotonic clock (`std::time::Instant`); the run's start instant, which is a condition
  rather than a measurement, is wall-clock. Mixing them silently is how a negative
  staleness appears in a results file.
- **Poll interval is a real wait.** At 1 000 ms × several cells the sweep's runtime is
  dominated by sleeping, which is what NF-001's budget is mostly spent on. Consider
  capping the observation window per cell rather than the event count, and record the
  window as a condition.
- **The harness-local `ProjectionStore` should be boring.** A counter per view behind
  the `begin`/`commit` pair (`crates/happenstance-core/src/projection.rs:117,126`).
  Near-zero cost is deliberate — read-model work would dilute the quantity under study —
  and that sentence belongs in the README's limitations as well as in the code comment.
- **Bind `EventStore`, import one flavour name per module.** Constraint 4 in `CLAUDE.md`;
  `standards/rust/20-two-flavour-ports.md` is the atom to open if the harness ends up
  generic over the store at all. It may not need to be — a concrete `MemoryEventStore`
  is legitimate here, and honest, provided the *conditions* say so.
- **Write the README last but not late.** Its headline figure is asserted equal to the
  corpus (AC-006), so it cannot be drafted before the run and cannot be edited after
  without re-checking.

## Tests and CI (merge gate)

The testing brief classifies project AC-010 as a **Measurement, not a test** — *"no
pass/fail assertion; the number and its conditions are the artefact"*
(`_decomposition.md:788`). That classification is about the *cost*. It does not exempt
the harness's own behaviour from being checked, and the instruments below check only
that: schema conformance, condition completeness, staleness semantics, output format,
re-run behaviour. None of them asserts anything about how large a number is, which is
what CF-34 forbids the bar from doing.

| tier | command / path | proves |
| --- | --- | --- |
| Harness tests (**outside the gate**) | `cargo test --manifest-path experiments/polling-cost/Cargo.toml` | AC-001 (`tests/one_cell.rs`), AC-002 (`tests/amplification.rs`, `tests/sweep_shape.rs`), AC-003 (`tests/record_conditions.rs`), AC-004 (`tests/staleness.rs`), AC-005 (`tests/schema.rs`), AC-006 + AC-009 (`tests/readme.rs`), AC-007 (`tests/gate_inertness.rs`), AC-008 (`tests/no_verdict.rs`), AC-009 (`tests/output_format.rs`), AC-010 (`tests/rerun_is_additive.rs`). Run by the implementer and by anyone re-running the experiment; **never** added to `xtask/src/main.rs`. |
| Measurement | `bash experiments/polling-cost/run.sh` from a clean checkout | AC-001 end to end: the pass in `results/` is regenerated, validates against `schema/`, and matches the README's headline. The transcript goes in the implementation report — this is the evidence that the artefact is a record of a run that happened. |
| Gate inertness | `cargo xtask affected --base main` | AC-007 / NF-005: over this story's diff it selects **no package**, which `is_inert`'s `"experiments/"` prefix already guarantees (`xtask/src/affected.rs:238-258`) and this story must not disturb. A non-empty selection means something leaked out of `experiments/`. |
| Project bar | `cargo xtask ci --fast` | The project's own integration bar (`.redkiln/config.yaml`, `verify:`). It must be green **and unchanged in step composition** — a green run with an extra step is a failure of this story, not a pass. |
| Full gate (unchanged) | `cargo xtask ci` | That fmt, clippy, the four `wasm32` steps, docs and `spec-trace` are all indifferent to this diff. `spec-trace` in particular proves ES-32's marker and citations are untouched (AC-008). |
| Story gate | `redkiln verify --grain story` | The PR boundary block above and the `_ledger.md` rows: every AC present, satisfied, and carrying non-placeholder evidence. |

**What is deliberately absent.** No step in `xtask/src/main.rs`'s `REQUIRED`
(`:105`) or `OPTIONAL`; no threshold; no benchmark job in CI; no `cargo bench`. Mutant 2
is precisely the version of this story that adds one, and its failure mode is that the
first red build is fixed by raising the threshold (`discover.md:127-134`).

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | mitigation inside this PR |
| --- | --- | --- |
| **A floor is cited as a verdict.** The number is quoted in the tail-seam argument without its conditions, and an in-process `Vec` figure decides a networked question. | likely / high — it is mutant 1's exact failure and the reason discovery named it | AC-006 puts the unreached falsifier terms in their own section with the word *floor*; AC-008 forbids a recommendation; AC-003 attaches the conditions to the record itself so a quoted figure can always be traced back to them. |
| **`run_projection`'s signature moves** after `projection-trait-and-runner` merges or during review. | moderate / low | One call site (*Implementation notes*); the harness is outside the gate, so a lagging harness never blocks anyone. Friction found while holding it is reported as a project AC-012 entry, not absorbed (AC-008). |
| **`MemoryProjectionStore` (HS-P0010) lands mid-flight**, changing the store identity between the recorded pass and the tree. | moderate / low | The store is a recorded condition, not an assumption (AC-005); EC-007 fixes the fallback; a later pass under a new tag re-measures without disturbing the committed one (AC-010). |
| **`GappyMemoryStore` is not available** when AC-004's test is written (M4 merges before M5 in the computed order, but this story does not depend on it). | low / low | The harness-local gapped wrapper is the stated fallback — an experiment crate ships nothing, so a local fixture freezes no shape (`_storymap.md:141-146`). |
| **Someone later wires the harness into the gate**, sincerely, as rigour. | moderate / high (a flaky gate blocks everything) | The bare `[workspace]` table carries the comment saying why, in `experiments/wire-format/Cargo.toml:1-8`'s voice; the README states it; AC-007 has a test that reads both manifests. |
| **Sweep runtime blows the budget** and the harness stops being re-run. | moderate / moderate | NF-001's explicit budget, and a trim discipline that trims ranges and records the reason rather than dropping an arm (AC-002). |
| **Timings become the headline** because they are the number people expect from a benchmark. | moderate / high | AC-002 makes the ratio the primary field and the machine mandatory on any timing; AC-009 forbids a timing figure appearing above the ratio it qualifies. |
| **The story drifts into deciding the seam** — a README paragraph that reads as a recommendation. | low / high (it would pre-empt `projection-clause-verdicts` and HS-P0016) | AC-008; the PR boundary excludes `spec/**` entirely, so the drift cannot land as a clause edit even if it lands as a sentence, and review catches the sentence. |

**Coupling ledger.** Hard, in-repo: `crates/happenstance` (`run_projection`, `Projection`,
`Codec` behind `unstable-projection`), `crates/happenstance-core` (`EventStore`,
`MemoryEventStore`, the `ProjectionStore` port). Soft: `happenstance-testkit`'s
`GappyMemoryStore` (dev-only, with a stated fallback), HS-P0010's `MemoryProjectionStore`
(preferred if present). None of these is *modified* by this story — a diff that edits
`crates/**` has left the PR boundary.

## Dependencies

**Blocks on**

- **`projection-trait-and-runner`** (M5, same slice) — hard. There is nothing to measure
  until a polling runner exists, and AC-001 requires the *real* one
  (`_storymap.md:59`, `:124-126`). This is the story's only `depends_on`.

**Soft ordering, not a dependency**

- **`misbehaving-testkit-stores`** (M4) supplies `GappyMemoryStore`, which is AC-004's
  preferred instrument. M4 precedes M5 in the computed order, so it will normally be
  present; if it is not, the harness-local gapped wrapper discharges AC-004 and this
  story does not wait.
- **HS-P0010 `projection-store-freeze`** may land `MemoryProjectionStore`. Preferred if
  present, never required (AC-005, EC-007).

**Unlocks**

- **DoD 8** (`project.md:241-242`) — the closeout paragraph that carries the number. This
  story produces the artefact and its quotable headline; the closeout cites it. The
  citation is not in this PR.
- **The post-0.1 tail-seam decision** (`RUNBOOK.md:4028-4030`; `project.md:149`, DR-08) —
  a future ADR pass, not a story on this map, and not settled here.
- **Initiative DoD 12** (`initiative.md:393-395`) — ES-32's falsifier stops naming a
  benchmark that does not exist. The audit and the count reconciliation remain HS-P0016's.

**Independent of** `projection-clause-verdicts` — same slice, either order
(`_storymap.md:124-126`). Nothing in this story reads or writes a clause, and nothing in
that story reads this harness.

## Anchors (progressive disclosure)

Every path below exists in the tree today. Open them at the moment named, not before —
the context pack above is the part that must be read first.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `experiments/position-visibility/run.sh` | The house shape for an experiment's entry point: one script, documented sub-commands on lines 8–12, `set -euo pipefail`, and an explicit statement that nothing it does touches the workspace. Copy the shape; drop the Docker. | Before writing `run.sh` — first file to open in implementation. | AC-001 |
| `experiments/position-visibility/README.md` | The eight-section layout this README must follow, and specifically §1 *What was measured, on what* (`:27-55`), §6 *What none of this proves* (`:345-367`) and §8 *How to re-run it* (`:424-461`), including `HS_TAG`'s role (`:454-456`). This is the only in-tree example of the artefact this story is trying to be. | Before drafting the README, and again before writing the gap section. | AC-003, AC-006, AC-009, AC-010 |
| `experiments/wire-format/Cargo.toml` | Lines 1–8 are the bare `[workspace]` trick *with the comment explaining it*. The comment is as load-bearing as the table — it is what stops a future contributor "fixing" the missing workspace membership. | Before writing `experiments/polling-cost/Cargo.toml`. | AC-007 |
| `experiments/wire-format/README.md` | Why the seed goes **into the result file** rather than into prose, and why the crate resolves its own lockfile. The failure the whole `experiments/` discipline exists to prevent, stated by a sibling that already hit it. | While designing the result record. | AC-005 |
| `xtask/src/affected.rs` | `is_inert`'s `"experiments/"` prefix (`:238-258`) and the test that pins it (`:658-667`). This is the mechanism that makes AC-007 true rather than aspirational; read it before assuming the gate ignores you. | When verifying gate inertness, and if `affected` ever selects a package over this diff. | AC-007 |
| `xtask/src/main.rs` | The `REQUIRED` list (`:105`) — the file this story must **not** modify. Open it to confirm the absence, not to add to it. | At review, as a negative check. | AC-007, AC-008 |
| `crates/happenstance-core/src/store.rs` | `EventStore::head`'s own documentation (`:235-245`) forbidding the subtraction, and the `read`-returns-the-stream shape. The reason AC-004 is phrased the way it is, in the compiler-adjacent words rather than the spec's. | Before implementing the staleness observer. | AC-004 |
| `crates/happenstance-core/src/projection.rs` | The `ProjectionStore` port: `begin` (`:117`), `commit` (`:126`) and the inclusive-`from` resume rule (`:104-106`) that the harness-local store must honour and the runner already obeys. | Only if the harness-local store is needed (i.e. `MemoryProjectionStore` has not landed). | AC-005 |
| `crates/happenstance-core/src/memory.rs` | `MemoryEventStore`, the substrate; and `spawns_from_generic` / `send_flavour_stream_is_send_in_generic_code` as the pattern for writing a `Send` bound at the definition if the harness ends up generic. | When seeding the log, and if the sweep is written generically over the store. | AC-001 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` | The signed-off design. Read `:538-570` (the runner's signature), `:647` (its feature gate), `:810-836` (transience), `:911-928` (overflow and narrow viewport), `:919` (polling renders nothing), `:969-1013` (anti-patterns 9 and 10) and `:1260-1263` (`MemoryProjectionStore`). It binds the console output and the README's composition even though this story renders no surface. | Before calling `run_projection`, and again before writing any line the harness prints. | AC-001, AC-005, AC-009 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md` | The architecture brief on `head()` comparing rather than subtracting (`:515-519`) and where AC-010 lands (`:425`); the testing brief's AC-010 row (`:788`) — *Measurement, not a test* — which is why no threshold exists. | Before writing the staleness metric; before adding any assertion about a number. | AC-004, AC-008 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_storymap.md` | *What is deliberately not a story here* (`:141-146`) — the `MemoryProjectionStore` prohibition and its exact scope, which is what licenses a harness-local store outside `crates/`. | When deciding the projection store. | AC-005 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md` | AC-010 verbatim (`:197-199`), DR-08 (`:149`), DoD 8 (`:241-242`), and the note that `MemoryEventStore` is an in-process `Vec` (`:121-124`) — the substrate limitation the README must state. | When drafting the conditions table and the gap section. | AC-003, AC-006 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/polling-cost-measurement/discover.md` | The three named mutants (`:91-143`) written out with their discriminators, and the two questions deferred to this spec (`:47-62`). The fastest way to check an implementation is honest. | At self-review, before opening the PR. | AC-002, AC-004, AC-007, AC-008 |
| `spec/SPECIFICATION.md` | ES-32 and its falsifier (`:4021`) — the words AC-006 must quote; CF-34 (`:8263-8270`) and its audit row (`:8765`, `:8774`) — the counting-not-seconds falsifier that chose the headline unit. | Before writing the README's verdict and gap sections. Do not edit this file. | AC-002, AC-006, AC-008 |
| `spec/E2E-CASES.md` | E2E-32, the fan-out runner ES-32's falsifier names (`:826`), and the record that a benchmark harness is *not blocked and not a case* (`:1671-1677`). It fixes what "fan-out" must mean in the sweep. | When fixing the sweep's shape. | AC-002 |
| `RUNBOOK.md` | `:4028-4030` — *"a number here is what a post-0.1 tail-seam ADR would be argued from"*, and `:3953-3955`, the phase that owes the benchmark. This is who the README is written for. | Before writing the README's first paragraph. | AC-006 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/projection-trait-and-runner/spec.md` | The blocking story's own contract: what `run_projection` will actually look like when this harness calls it. | Immediately, at the start of implementation. | AC-001 |
| `standards/rust/20-two-flavour-ports.md` | Why `EventStore` and not `SendEventStore`, and one flavour name per module — only relevant if the harness is generic over the store. | Only if the sweep is written generically. | AC-001 |
| `standards/rust/50-dependency-hygiene.md` | The bar a new manifest's dependency list must clear, and why an experiment's dependencies must not reach the workspace's. | When writing the harness manifest's dependency list. | AC-007, NF-007 |
| `.kb/decisions/0029-msrv-raised-to-1-97-1.md` | The MSRV is a promise the *published* crates make; an experiment makes none. Read it before assuming this harness is constrained by the floor. | If a stable-since-1.88 construct (let-chains) looks disallowed. | NF-006 |

## Clarifications resolved during spec

1. **The sweep's axes and ranges**, deferred by discovery (`discover.md:49-51`), are fixed
   in *Behavior and interfaces*: fan-out {1, 2, 4, 8, 16, 32}, log size {1 000, 10 000,
   100 000}, poll interval {10 ms, 100 ms, 1 000 ms}, and two selectivity arms. Ranges may
   be trimmed for runtime with the trim recorded; the shape may not (AC-002, NF-001).
2. **How staleness is expressed**, deferred by discovery (`discover.md:60-62`): observed
   append-to-apply nanoseconds, and events yielded-but-not-applied at a poll boundary.
   Never `head() - checkpoint` (AC-004).
3. **Whether the harness runs against anything but `MemoryEventStore`**, the live tension
   (`discover.md:52-59`): **no** — and the artefact says so in the falsifier's own words.
   A second run against a durable adapter belongs to HS-P0012 (AC-006).
4. **The projection store**: HS-P0010's `MemoryProjectionStore` if present, else a
   harness-local store inside `experiments/polling-cost/src/`, never under `crates/`.
   Recorded as a condition either way (AC-005, EC-007). This resolves discovery's *"a
   harness that measures only the read side could proceed without it; whether that is
   worth doing is spec's call"* — the read side is the quantity under study, so the
   projection store is deliberately near-zero-cost, and that is a stated limitation
   rather than an omission.
5. **The headline unit** was not posed as a question by discovery, but was the load-bearing
   choice available to this spec: a dimensionless **count** (delivery amplification), not a
   duration. CF-34's own falsifier gestures at the substitution
   (`spec/SPECIFICATION.md:8263-8270`) and mutant 1 is what a timing headline produces.
6. **Two acceptance criteria were added beyond the eight the front half enumerated.**
   **AC-009** (the artefact's composition: README section order and hierarchy, and plain
   80-column console output with no colour, spinner, percentage or in-place rewrite) and
   **AC-010** (a re-run is additive — a committed pass is never overwritten and stays
   byte-identical). Both were present in the front half as prose — in *Data and
   migrations*' `Re-runs` row and in the design's terminal conventions — with **no AC
   carrying them**, which means `redkiln verify` would never have gated them. RFC §6.7
   requires the interaction-quality invariants to be table rows rather than bullets, so
   they were promoted rather than restated. Nothing in the front half was re-decided.
7. **Numbering collision, stated so it cannot mislead.** This story's **AC-010** is a
   re-run invariant; the **project's** AC-010 is the polling-cost measurement this whole
   story traces to, and the **project's** AC-012 is the defect log. Every reference to a
   project criterion in this document is written as *"project AC-0xx"*.
8. **The harness has its own tests, and that does not contradict *Measurement, not a
   test*.** The testing brief's classification (`_decomposition.md:788`) is about the
   *cost*: no assertion may exist about how large a number is. The instruments in *Tests
   and CI* assert schema conformance, condition completeness, staleness semantics, output
   format and re-run behaviour — the harness's own correctness — and none of them runs in
   the gate, because the crate is not a workspace member. A harness whose own behaviour is
   unchecked produces a corpus nobody can trust, which fails project AC-010 by a different
   route.
