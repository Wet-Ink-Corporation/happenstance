# Seed — measured, not claimed

Raw material for `/redkiln:initiative`. This states a problem and a vision. It
deliberately does **not** decompose the work, name projects, or propose a design:
that is `/redkiln:plan`'s to decide, from its own grounding.

## Where this stands

happenstance has never measured anything about itself.

One real measurement instrument exists — `experiments/position-visibility/`, a
bash and Docker harness driving `pgbench` through four schema arms — and what it
measures is PostgreSQL, not this library. The other experiment says so in its own
words: `experiments/wire-format/README.md:265-269` records that "every number here
is a byte count or a pass/fail outcome, never a timing", and points at
position-visibility as "what a throughput measurement in this repository looks
like when one is actually taken".

There is no Rust benchmark harness at all. No `criterion`, no `divan`, no `iai`;
no `benches/` directory, no `[[bench]]` target, and no `[profile.bench]` —
`Cargo.toml:137-139` is the whole of the profile block. The only executable store
in the workspace is `MemoryEventStore`, a `Vec` behind an `RwLock` that scans
every event on every read (`crates/happenstance-core/src/memory.rs:296-336`) and
says outright that it is not built for scale. Six adapters carry `todo!()` bodies
where their I/O will go, and no SQL executes anywhere in the workspace.

The separation of concerns is already settled, and only one half of it was built.
`CF-33` is `[FROZEN]`: no conformance rule may read a clock, measure elapsed time
or assert on an operation count, and `cargo xtask lint-clock` enforces it on every
gate run. `CF-34` states the other half — performance is measured by a separate
harness, which is not the bar — and that harness has never existed.

One body of work in flight touches this. The runbook's SQLite phase owns
`event_store_benchmarks!(fixture)` in the testkit behind a `bench` feature
(`RUNBOOK.md:4207-4213`), and its exit criterion is "a benchmark *number* in
ADR-0022, not a claim" (`RUNBOOK.md:4227-4228`). That is a decision instrument
built for one decision. The initiative carrying it names the rest as an explicit
non-goal in the same breath — general throughput tuning, benchmarking suites and
optimisation passes are out of its scope. So the instrument is owned and the
discipline around it is not, and the two will collide on the same file unless
somebody decides where the seam falls.

## The problem

"Fast, efficient" are the first two words of the stated goal, and nothing in the
repository can be pointed at to support them.

Six things are true today and should not be:

**Four clauses in the specification cannot be resolved, and each one names the
same blocker.** ES-17 (`SPECIFICATION.md:3291-3295`) is falsified by "a
measurement on a real adapter showing the per-event clone is a material fraction
of append cost". ES-32 (`:4025-4030`) says outright that "the workspace has no
benchmark harness and a conformance rule cannot substitute for one". PS-30
(`:5464-5467`) turns on "whether the poll cost of N independent reads is real —
that is a benchmark, not an assertion". The append-condition guard collapse
(`:1821-1824`) names "the Postgres adapter and the benchmark harness" as its two
instruments and records that both are unbuilt. These are not clauses awaiting an
opinion. They are waiting on a number nobody can produce.

**The repository already cites performance figures it has never reproduced.**
`references/evaluation/ARCHITECTURAL-EVALUATION.md:827-828` carries a 970×
penalty for the obvious query translation and a 650× swing on multi-tag probe
order, and finding P2 is stamped `[PLAUSIBLE — schema claim confirmed, figures
unverified in-repo]`. Those numbers are load-bearing — they are the argument for
`Query::index_arms()` and for the shape of the tag index — and they came from
outside. A library that decides its schema on figures it cannot re-run has
imported a conclusion, not evidence.

**A benchmark that nothing can fail is decorative.** This repository already
applies that test to conformance rules and it transfers exactly. A number with no
characterised noise floor rejects no wrong implementation, and the expensive
version of this lesson has already been paid for once: the position-visibility
experiment's first design *failed*, the baseline drifting 2.7× at one client and
3.0× at 64 — larger than two of the three effects being measured
(`RUNBOOK.md:1614-1620`). The paired design that replaced it is where the
published numbers came from, and the failed pass is kept as evidence. The
standing instruction is in the same paragraph: "any later benchmark in this
repository should assume the same instability." Nothing carries that forward into
a Rust harness, because there is no Rust harness.

**Complexity is invisible to every check that exists.** `SPECIFICATION.md:8226-8234`
works the case: a mixed tagged and untagged two-item query over a log where one
item selects a handful of events and the other selects millions. An adapter that
scans where it should seek passes every rule that can be written. The conformance
suite is an instrument sharp enough that every rule has a registered wrong
implementation which fails it — and this entire class of defect walks past it.

**Nothing measures what the abstraction costs.** The question an adopter actually
asks is not how fast happenstance is; it is what happenstance costs over the
database they already run. That number — the overhead above a raw insert on the
same file or the same instance — is the one nobody can dispute and nobody can be
sold, and it has never been taken.

**There is nowhere for a number to live.** `CF-34` prescribes the model already:
benchmarks "are published per adapter and compared against that adapter's own
history". No such record exists, no format for one exists, and a prospective
`cargo add` user meets an adjective. A measurement that lives in the commit
message of the change it justified is findable only by someone who already knows
it exists, which is exactly the person who does not need it
(`.kb/reference/README.md:42-45`).

## The vision

Performance becomes a thing this library reports rather than asserts: measured
under stated conditions, published where a stranger will find it, and comparable
against its own past well enough that a regression is visible before a user finds
it.

Concretely, that means four things. A **committed corpus** in the shape
`experiments/position-visibility/README.md` already established — environment
table, positive controls declared before the run, an honest note on what the
method got wrong, a section saying what none of it proves. A **reader-facing
figure** standing where the adjective stands today, in the README and in each
adapter's documentation, carrying its date and its conditions with it. A
**published history per adapter**, which is the comparison `CF-34` names and the
only one that detects drift. And underneath all three, the **overhead against the
honest floor** — what the storage engine does on its own, so the number published
is happenstance's cost rather than SQLite's speed.

The harness is worth as much as the numbers. Adapters inherit the conformance
suite; an adapter author who inherits the benchmarks the same way learns their
store is slow before their users do, and no other Rust event-sourcing library
ships that.

## Who it is for

**The prospective adopter**, weighing this against the Postgres event table they
would otherwise hand-roll. They are being asked to put a domain model behind an
abstraction, and the cost of that abstraction is currently unstated.

**Adapter authors**, this project's own included. The conformance suite tells them
when they are correct and nothing tells them when they are slow — and the failure
mode is silent, because a store that scans where it should seek is fully
conformant.

**This project's own decision-makers.** Four provisional clauses, the SQLite
append-condition strategy, and the schema figures inherited from an external
review are each waiting on a measurement. A decision deferred to a measurement
that never happens is not a decision.

**The local-first and edge case**, where the budget is smallest and the cost of
being wrong is highest — `spec/E2E-CASES.md:378-383` describes one cheap local
statement that becomes a full HTTP round trip on "the adapter with the smallest
latency budget in the system".

## What must remain true

These are constraints on any answer, not preferences:

- A benchmark never gates a merge (`CF-34`). A threshold nobody can justify
  becomes a threshold everybody raises, and the number stops meaning anything the
  second time it is moved.
- No conformance rule reads a clock (`CF-33`, `[FROZEN]`, enforced by
  `cargo xtask lint-clock`). Benchmarks stay out of `suite.rs` and out of
  `for_each_event_store_rule!`; the conformance rule count is unchanged by their
  arrival, structurally rather than by assertion.
- A benchmark whose noise floor is unmeasured publishes decoration. Assume the
  instability already found; a drift-resistant design is the default and not an
  improvement on one.
- Commit your evidence, or it is not evidence (`RUNBOOK.md:1649-1650`). Raw
  results are committed, discarded passes included — the failed sequential pass is
  kept under `experiments/position-visibility/results/discarded-sequential/` on
  purpose.
- Every published figure is a statement about a moment, and says which: a date and
  a commit sha in the body (`.kb/reference/README.md:19-26`).
- Measurements stay out of the gate (`CLAUDE.md`, repository map). A Cargo-based
  experiment stays out of the workspace the way `experiments/wire-format/` does.
- No measurement dependency enters `happenstance-testkit`'s `[dependencies]`. The
  emitter is a caller-supplied parameter, and the tooling lands in the adapter's
  dev-dependencies.
- No `#[async_trait]`, and the `!Send` flavour keeps working. A harness that can
  only measure `Send` stores has quietly dropped the constraint the whole design
  pays for.
- Where any document and `spec/SPECIFICATION.md` disagree, the specification wins.
  A frozen clause changes by a new decision record, not an edit.

## Supporting material

Read these rather than trusting the summary above; several are long and all of
them are more specific.

| Path | What it carries |
| --- | --- |
| `experiments/position-visibility/README.md` | The house format for a measurement writeup, and the methods failure that produced it. The closest thing to a template that exists. |
| `spec/SPECIFICATION.md` §6.7 | `CF-33` and `CF-34` — why benchmarks are not conformance, stated as clauses with the wrong implementation each forbids. |
| `spec/SPECIFICATION.md:1821-1824, :3291-3295, :4025-4030, :5464-5467` | The four provisional clauses whose stated blocker is the absent harness. |
| `RUNBOOK.md:4207-4228` | The harness work item the in-flight SQLite phase already owns, and the exit criterion demanding a number rather than a claim. |
| `references/evaluation/ARCHITECTURAL-EVALUATION.md:825-830` | Findings P1–P4: the unreproduced figures, and the case for benchmarks that adapters inherit. |
| `.kb/reference/README.md` and `.kb/reference/position-visibility-experiment-2026-08.md` | How a measurement becomes citable here, and the rule that a number without a date is false rather than weak. |
| `crates/happenstance-testkit/src/contract.rs` | The `Fixture` trait — `connect()`, the `Capability` constants and the three declared ceilings. What a benchmark would have to drive adapters through. |
| `crates/happenstance-core/src/memory.rs:296-336` | The only executable store, and the scan that `CF-34`'s preamble uses as its worked example. |
| `examples/course-subscriptions/src/main.rs:114-125` | The overlapping two-item OR query — the realistic workload shape, in the one program that uses the contract. |
| `RUNBOOK.md:1614-1620, :1649-1650, :1672-1675` | The drift finding, and the two instructions it left behind. |

## Deliberately not decided here

**How this decomposes, and where it sits against the work already in flight.** The
SQLite phase owns `event_store_benchmarks!` for one decision and its initiative
disowns everything around it. Whether this work builds that harness, consumes it,
or arrives first and hands it over is a real seam with real sequencing
consequences. What is stated above is only that the collision exists.

**Whether any of it runs without a human typing it.** `CF-34` forbids a benchmark
that gates a merge, and a published history implies runs nobody asked for. Manual
measurement, taken deliberately for a named question, is the starting position;
whether a recording-only scheduled run earns its place is open and probably owes
its own decision record.

**Which workloads, which tool, which machine.** `criterion`, `divan`, or something
hand-rolled; what a benchmark of a `!Send` store on `wasm32` even means; whether a
developer machine can host a figure worth publishing at all. All downstream of
deciding what is being measured and why.

**Comparison against peer implementations is declined, not deferred.**
`ARCHITECTURAL-EVALUATION.md:830` names a comparison table against a peer library
as "the most persuasive artefact this project could publish", and this seed
disagrees: cross-language, cross-design throughput comparisons are contested by
default and the credibility cost outweighs the positioning. Own history,
cross-adapter within the workspace, and overhead above the bare database are the
comparisons worth standing behind. If that judgement is wrong it should be
overturned deliberately, not drifted into.
