---
item: HS-S0028
stage: discover
created: 2026-08-12T13:01:51.544Z
updated: 2026-08-12T13:01:51.544Z
template_sig: 86ce4036
rendered_sig: 0cfcd8e0
---

# Discover — The polling cost ES-32 imposes, as a number

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice: land a reproducible harness under `experiments/` that records the N views × N reads cost ES-32 imposes on the polling runner as a **number with its conditions** — outside the gate by construction — so a post-0.1 tail-seam decision argues from a measurement rather than an estimate | `_storymap.md:59` (M5 row) | "With its conditions" is doing as much work as "a number" |
| AC-010 — the N views × N reads cost is recorded as a number **with the conditions it was taken under, not as an estimate** | `project.md:197-199` | The conditions are part of the artefact, not metadata about it |
| `depends_on: projection-trait-and-runner` — supplies the runner whose cost is being measured; independent of `projection-clause-verdicts`, so the two can run in either order | `_storymap.md:59`, `:124-126` | There is nothing to measure until a polling runner exists |
| ES-32's text, read directly: *"`EventStore` MUST NOT grow a tail, subscribe or notify method at 0.1. Consumers poll,"* `[PROVISIONAL — falsified if the fan-out runner of E2E-32 cannot hold N views within their staleness budget at a measured poll interval on a real deployment]` | `_grounding.md:135-139`, reading `spec/SPECIFICATION.md:4021` | The falsifier names *a real deployment* and *a staleness budget*. The measurement should speak to those terms or it does not bear on the clause |
| **The workspace has no benchmark harness, so the number this project records is the first one to exist** | `_grounding.md:137-139` | There is no house format to copy for *this* measurement, but there is a house *shape* — see the next row |
| The in-tree precedent: `experiments/` holds three studies, and `experiments/position-visibility/` is laid out as `README.md`, `run.sh`, `container/`, `bench/`, `results/`, `schema/` — a runner, a described environment, raw result files and a schema for them | `experiments/position-visibility/` (listed directly) | The reproducibility bar is already set by a sibling and is high: the environment is scripted, not described in prose |
| `experiments/` is *"measurements. reproducible, and not in the gate"*, and CF-34 states performance is measured by a separate harness and is **not the bar** | `CLAUDE.md`, repository map; `_decomposition.md:425`, `:788` | Wiring this into `cargo xtask ci` would convert a measurement into a flaky test — the harness is outside the gate **by construction**, not by omission |
| This is a **Measurement**, not a test: no pass/fail assertion; the number and its conditions are the artefact | `_decomposition.md:788` (testing brief, AC-010 row) | The story's Definition of Done is a recorded artefact, checked at closeout (`project.md:241-242`, DoD 8) |
| The runner polls, and polling renders **nothing** — no spinner, no progress, no in-place rewrite. *"Its cost is a number in `experiments/`, not a performance"* | `_design.md:919`, `:181-183` | The design already refused the alternative of making the cost visible as motion |
| What the number is *for*: a post-0.1 tail-seam decision. The clause forbids a tail seam at 0.1, so N views is N independent reads, and DR-08 says the cost is recorded *"so a post-0.1 tail-seam decision has something to argue from"* | `project.md:149` (DR-08); `_decomposition.md:425` | The audience for this artefact is a future decision, not this project's gate |
| `MemoryEventStore` is all this project needs for its *functional* work — but it is an in-process `Vec`, and no durable adapter exists yet (all six carry `publish = false` and none has run the conformance suite) | `project.md:121-124`; `_decomposition.md:891-896`; `CLAUDE.md`, repository map | The substrate available to this measurement is materially weaker than ES-32's falsifier asks for, and that gap must be *stated in the artefact* rather than papered over |
| `head()` answers "am I caught up?" by comparison, never by subtraction, because positions may have gaps | `_decomposition.md:515-519` | A staleness metric computed as `head - checkpoint` is meaningless; staleness must be measured in time or in events actually observed |

## Questions

**Answered here.**

- *Is this in the gate?* No, and it must not be. `experiments/` is outside the gate by
  construction and CF-34 says performance is not the bar (`CLAUDE.md`, repository map;
  `_decomposition.md:788`).
- *Is it a pass/fail test?* No. It is a Measurement in the testing brief's own taxonomy —
  the number and its conditions are the artefact, and there is nothing to fail
  (`_decomposition.md:788`).
- *What shape does it take?* The one `experiments/position-visibility/` already establishes:
  a scripted runner, a described-and-scripted environment, raw results and a schema. Prose
  describing an environment is not reproducibility.
- *Where is the verdict recorded?* In the project's closeout, either way — DoD 8 pairs it
  with AC-013's macros verdict (`project.md:241-242`).

**Deferred to `spec`.**

- *The exact axes swept.* Discovery fixes what must be recorded — N views, poll interval,
  log size, read latency and observed staleness, each with its value rather than its
  description — and leaves the sweep's ranges to spec.
- *Whether the harness runs against anything other than `MemoryEventStore`.* This is the
  live tension, and it is deferred rather than resolved: ES-32's falsifier names *"a real
  deployment"*, and no durable adapter exists in this project's scope
  (`project.md:121-124`). Discovery's position is that the measurement is still worth taking
  — an in-process floor bounds the cost from below and is the first number that exists — but
  the artefact **must state which term of the falsifier it does not reach**, so a later
  reader does not mistake a floor for a verdict. Whether a second run happens against
  `happenstance-sqlite` belongs to HS-P0012, not here.
- *How staleness is expressed.* Not as `head - checkpoint`
  (`_decomposition.md:515-519`); in time, or in events actually observed. The precise metric
  is spec's.

**Not blocked** on `trybuild`. **Indirectly downstream of HS-P0010's `MemoryProjectionStore`**,
because the runner this measures commits into a `ProjectionStore` and there is no in-memory
one yet (`_design.md:1260-1263`). A harness that measures only the *read* side could proceed
without it; whether that is worth doing is spec's call, and it is a real question rather than
a formality.

## Decision

The problem this slice solves is that ES-32 forbids a tail seam at 0.1 — consumers poll —
and that prohibition is `[PROVISIONAL]` against a falsifier phrased in numbers nobody has
ever taken: whether a fan-out runner can hold N views within their staleness budget at a
measured poll interval. Every argument for or against a tail seam in this repository is
currently an estimate, and the workspace has no benchmark harness at all, so the number this
story records is the first one to exist. This story lands that harness in `experiments/`,
following the layout `experiments/position-visibility/` already established — a scripted
runner, a scripted environment, raw results and a schema — and records the N views × N reads
cost as a number carrying every condition it was taken under. The spec for this story covers
the harness's layout and entry point, the axes it sweeps and the units it reports, the
environment capture that makes a rerun meaningful, the results format and its schema, the
`README.md` that states what was measured and — explicitly — which term of ES-32's falsifier
this measurement does *not* reach, since no durable adapter exists to measure against, and
the closeout record that carries the number. It also covers the negative requirement: nothing
here is wired into `cargo xtask ci`. No `[FROZEN]` clause is amended; ES-32 is
`[PROVISIONAL]` and this story supplies evidence toward its falsifier without moving its
marker, which is `projection-clause-verdicts`' and HS-P0016's territory rather than this
story's.

## The wrong implementation

**The mutant: a number without its conditions.**

A harness that spins up `MemoryEventStore`, seeds a hundred events, runs the projection
runner across N views in a loop, and writes

```
experiments/polling-cost/results/run.txt
    N=10 views: 3.2ms total
```

It satisfies AC-010 read as a sentence — the cost is recorded as a number — it is
reproducible in the weak sense that running it again gives roughly the same figure, it is
outside the gate, and nothing in `cargo xtask ci` disagrees with it because nothing in
`cargo xtask ci` ever looks at it. It will sit in the tree indefinitely and be cited.

It is wrong in two ways that compound. First, it measures the wrong thing: 3.2ms is the cost
of scanning a `Vec` ten times in one process. ES-32's falsifier is about *"N views within
their staleness budget at a measured poll interval on a real deployment"*
(`spec/SPECIFICATION.md:4021`), and an in-process number has no poll interval, no network,
no staleness and no deployment — so it cannot argue for a tail seam and, more dangerously,
it can be waved at to argue *against* one. Second, it records no conditions, so nobody can
tell that it measured the wrong thing: the log size, the poll interval, the number of views,
the machine, the build profile and the store are all absent, and a reader six months from now
has a figure with no way to judge its relevance. AC-010 says *"with the conditions it was
taken under, not as an estimate"* for exactly this reason — a number without conditions **is**
an estimate wearing a decimal point.

The discriminator is the sibling already in the tree:
`experiments/position-visibility/` ships `container/env.sh`, `container/setup.sh`, a `schema/`
for its results and a `run.sh` that reproduces them, alongside per-arm raw output files. This
story's spec must require the same shape, and must require the `README.md` to name the term
of ES-32's falsifier the measurement does not reach — because the honest artefact here is a
floor with a stated gap, and the dishonest one is a floor presented as a verdict.

**A second mutant: wiring it into the gate.** Adding a `Step` to `xtask/src/main.rs`'s
`REQUIRED` that runs the harness and asserts the cost is under some threshold looks like
rigour and passes on the machine it was written on. `experiments/` is outside the gate **by
construction** — *"measurements. reproducible, and not in the gate"* (`CLAUDE.md`, repository
map) — and CF-34 says performance is measured by a separate harness and is not the bar. A
timing assertion in a gate that must be green before every commit is a flaky test with a
threshold nobody can justify, and its first red build will be resolved by raising the
threshold, at which point it measures nothing and blocks everything.

**A third mutant: reporting staleness as `head() - checkpoint`.** It is one subtraction, it
produces a plausible "events behind" figure, and it is exactly what `head()`'s own doc
forbids, because positions may have gaps (`_decomposition.md:515-519`). Against
`MemoryEventStore` it happens to be correct, which is what makes it survive; against any
store that leaves gaps — and `GappyMemoryStore` exists in this same project to make that
concrete — the reported staleness is a fiction, and the fiction is systematically *worse*
than reality, which would argue for a tail seam the measurement was supposed to evaluate
neutrally.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

The two judgement boxes. **Literal positions:** this story adds no conformance rule and no
test of any kind — its output is a measurement, not an assertion. The one place positions
could enter is a staleness metric, and discovery has ruled out the `head() - checkpoint`
form above for exactly the reason the no-literal-positions rule exists: positions may have
gaps. Ticked as vacuously true of the suite, and on the stronger basis that the harness
observes positions only as values the store assigned. **Frozen clauses:** ES-32 is
`[PROVISIONAL]`, not `[FROZEN]`, and this story does not move its marker — it supplies
evidence toward the falsifier and leaves the disposition to the stories and projects that own
it. Nothing under `crates/happenstance-core/src/**` and nothing in `spec/SPECIFICATION.md` is
edited here.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
