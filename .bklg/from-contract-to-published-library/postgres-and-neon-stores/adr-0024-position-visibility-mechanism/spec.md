---
item: HS-S0065
stage: spec
created: 2026-08-12T13:47:03.977Z
updated: 2026-08-12T13:47:03.977Z
template_sig: 87bbf1d0
rendered_sig: 5dc0e76e
---

# Spec — ADR-0024 — the position-visibility mechanism, decided on numbers

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DoD 5 at `:373-374`; BR-02, BR-03, BR-15 |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — traceability matrix, DAG |
| Project (charter) | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/project.md` — AC-001, DR-3, DR-7 |
| This spec | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/adr-0024-position-visibility-mechanism/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md` — *Architecture* AC-001 row (`:57`), Notes §2 Root D (`:159-164`), §3 (`:166-190`), §8 point 2 (`:372-380`), §9.3 (`:444-451`), §10 (`:453-476`); *Testing* AC-001 row (`:511`), Notes §1 (`:527-537`), Notes §2 (`:539-555`) |
| Signed-off design | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_design.md` — **no user-facing surface**, approved 2026-08-12. Renders nothing; this story is bound by its anti-pattern list only |
| Story map | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_storymap.md` — this story's row (`:64`), its prose entry (`:131-140`), merge order step 3 (`:251-254`) |
| Roadmap pointer | `RUNBOOK.md:4311-4390` (phase 10 in full — *the one decision that is not a storage preference*, `:4322-4344`); `RUNBOOK.md:303` (the ADR queue row); `RUNBOOK.md:262-268` (the queue's numbering rule) |

## One-line PR slice

ADR-0024 lands accepted, choosing the mechanism on a number re-measured against
the real adapter — steady state and with a long-running transaction held open on
the same database — and naming the two arms that lost.

## Executive summary

**What this PR lands.** The decision record that phase 10 exists to produce, and
the measurement that entitles it to exist: a re-run of the position-visibility
cost against `PostgresEventStore` as actually built by
`postgres-append-and-frontier-head`, in two scenarios (steady state, and with a
write transaction deliberately held open on the same cluster); a long-form record
at `references/adr/0024-…`; the intake document that carries both into
`.kb/_intake/`; and — through `/redkiln:kb-ingest`, which is a **human-invoked
handoff**, never a hand-written atom — the accepted decision atom under
`.kb/decisions/` with its row in `.kb/maps/decision-map.md`.

**Pointer + delta, not a restatement.** The mechanism analysis already exists in
prose (`crates/happenstance-postgres/src/event_store.rs:26-77`) and the four SQL
arms are already measured (`experiments/position-visibility/README.md:229-246`,
`:300-341`). ADR-0013 spent that measurement on one narrow claim — *one
affordable mechanism exists*, which is what lifted ES-10 to `[FROZEN]` — and said
in terms that it was not choosing the adapter's
(`.kb/open-questions/postgres-arm-c-structural-cost.md:74-79`). The delta this PR
adds is exactly three things the existing corpus does not have:

1. **A number against an implementation rather than against a SQL script.** The
   phase-2 harness had no connection pool, no transaction lifetime tied to a
   trait method's async boundary, no cursor and no error mapping
   (`experiments/position-visibility/README.md:364-366`). All four now exist.
2. **A choice, with the two losers named and priced**, including the finding that
   the cheap arm buys a *different invariant* from the one ES-10 states.
3. **The structural bill, discovered rather than predicted** — what `sqlx`'s
   pooling and transaction lifetime actually cost the append path, and what
   staleness figure a caller should be told to plan for out of a measured range
   that spans four orders of magnitude (0.688 ms → 4010.719 ms).

The code path is **not** changed here. It was wired by
`postgres-append-and-frontier-head`; this story measures it, decides it, and
records it. The one code edit in scope is retiring the now-false sentence
`crates/happenstance-postgres/src/event_store.rs:77` — *"Nothing above is a
measurement, and the choice is owed one"* — in favour of a citation of the
accepted atom.

## Context pack

The load-bearing decisions this story must honour. Everything deeper is a
signposted anchor; nothing below is optional reading.

**1. The choice is this story's, and it does not exist yet.** No `0024-*` file
exists under `.kb/decisions/` (verified: the directory holds 0001–0016 and 0029)
or under `references/adr/`. The architecture brief states this as a standing
trap: *"ADR-0024 is a deliverable, not prior art. Any brief, spec or story citing
it is citing something AC-001 must write"*
(`postgres-and-neon-stores/_decomposition.md:179-181`). The number 0024 is
allocated and free (`RUNBOOK.md:303`, `RUNBOOK.md:264-266`).

**2. ADR-0013 settled affordability, not the adapter's mechanism — and this
story inherits its premise without owning it.** ADR-0013 lifted ES-10 to
`[FROZEN]` on the strength of arm C (`xid8` + `pg_snapshot_xmin`) being the only
arm that passes the inversion detector on both writer pairs *and* leaves writers
unserialised, at ratios 0.987 / 0.993 / 1.015 / 1.026 at 1 / 8 / 32 / 64 clients
(`spec/SPECIFICATION.md:2816-2842`). Two consequences bind here. First, ADR-0013
is explicit that it "needs one affordable mechanism to exist; it does not choose
the adapter's". Second, ES-10's *global* framing rests on the projection
checkpoint being global, which is **unsettled and owned by phase 6**
(`.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`;
`postgres-and-neon-stores/_decomposition.md:460-463`). **The record must say
plainly that it inherits that premise rather than presenting the global invariant
as permanently settled** — if phase 6's checkpoint turns out boundary-scoped,
ADR-0013 reopens and this measurement reopens with it.

**3. The number owed is a re-measurement, and it is two numbers, not one.** DR-3
(`project.md:180-187`) and `RUNBOOK.md:4341-4342` both require the steady-state
cost *and* the behaviour with a long-running transaction deliberately held open
on the same database. The second is not a refinement of the first: phase 2
measured 0.688 ms with no holder and **4010.719 ms** behind an unrelated
five-second write in an *unrelated database*
(`experiments/position-visibility/README.md:320-341`), and
`.kb/open-questions/postgres-arm-c-structural-cost.md:66-70` says the open
question is precisely *which end of that range a caller should plan for*. A
record carrying only a steady-state ratio discharges half of DR-3.

**4. The measurement is not a test, and must never enter the gate.**
`experiments/` is documented as "measurements. reproducible, and not in the
gate." (`CLAUDE.md`, repository map), and the existing harness "is **not** a
crate, it is not in the workspace members, it is not a `cargo xtask ci` step, and
it adds no dependency to any `Cargo.toml`"
(`experiments/position-visibility/README.md:21-23`). The workspace's
`members = ["crates/*", "examples/*", "xtask"]` (`Cargo.toml:3`) excludes
`experiments/` by construction; a harness added there that joins the workspace
would be compiled by the default gate and powerset-checked by `cargo hack`, which
is the failure this constraint exists to prevent. AC-001's number "is proven by
the record existing and validating, not by a pass/fail rule"
(`postgres-and-neon-stores/_decomposition.md:511`).

**5. The mechanism is already wired; this story does not re-wire it.**
`postgres-append-and-frontier-head` (HS-S0062) landed `append`/`head` and the
frontier read predicate composed into the cursor's `DECLARE`
(`_storymap.md:96-108`). This story measures **what was built**. If the
measurement *refutes* the wired mechanism — arm C structurally expensive through
`sqlx`, or the staleness bound unacceptable — the deliverable is a recorded
reversal naming what would have to change (the brief anticipates reconsidering
arm B-tag in light of phase 6:
`.kb/open-questions/postgres-arm-c-structural-cost.md:93-96`) **plus a re-plan
input**, not a quiet swap of the append path inside this story's diff.

**6. "Hard-won" is evidence produced by the two slice-predecessors, and this
record is where it is spent.** `postgres-rule-controls` (HS-S0064) runs
`nothing_below_an_observed_position_appears_later`
(`crates/happenstance-testkit/src/suite.rs:5880`) against a naive `nextval()`
arm and records that it **fails**; `postgres-concurrency-family` (HS-S0063)
proves the concurrency family green at `CONTENDERS = 8` under a multi-thread
runtime. Both are *inputs* to this record — phase 10's exit criterion is that
"`PreCommitPositionStore`'s rule is one this adapter had to *work* to pass, and
ADR-0024 says what that work cost" (`RUNBOOK.md:4377-4378`). A record that
asserts difficulty without citing the control is undischarged
(`postgres-and-neon-stores/_decomposition.md:512`).

**7. The specification owes this ADR an instrument, and this is where it is
discharged or excused.** `spec/SPECIFICATION.md:2844-2850` records that
`nothing_below_an_observed_position_appears_later`'s strength varies with the
adapter's poll shape, names the bounding instrument — *a poll-padding decorator
over `PreCommitPositionStore`* — and says it is **owed by phase 10 (ADR-0024)**.
The matching open question is
`.kb/open-questions/poll-count-bounds-the-visibility-rule.md`, whose §*What
forces it* says the real blocker was "an adapter with real I/O", which now
exists. Either the decorator was built (by `postgres-rule-controls`, per
`_storymap.md:126-129`) and its result is reported here, or **this record states
why a real multi-poll `append` did not need it**. Silence is not one of the two
options. Note the escape hatch the clause itself grants: "if it fires, the rule
changes and this clause does not."

**8. Root D is the mount, and the atom is never hand-authored.** The seam is
`.kb/_intake/` → `/redkiln:kb-ingest` → an atom under `.kb/decisions/` plus its
row in `.kb/maps/decision-map.md`
(`postgres-and-neon-stores/_decomposition.md:57`, `:159-164`). Three facts an
implementer trips over: (a) hand-writing atoms "produces the directory layout of
the process without the process, which is why the first attempt at this was
reverted (`0269720`)" (`CLAUDE.md`); (b) `/redkiln:kb-ingest` carries
`disable-model-invocation` — it is a **human handoff**, so this story's own diff
ends at a staged intake document and the long-form record, and the ingest wave
commits the atom on its own worktree branch; (c) the long form goes to
`references/adr/` and the atom links it — deleting the long form because the atom
exists discards the transcripts and measurement tables `spec-trace` cites by
line (`CLAUDE.md`, *Where the work lives*).

**9. Nothing `[FROZEN]` is edited.** ES-10 is `[FROZEN]`
(`spec/SPECIFICATION.md:2823`) and a frozen clause changes by ADR, never by edit.
This record *reports on* ES-10; it does not restate or amend it. Any marker
movement is regenerated by `cargo xtask spec-trace --write`, never hand-edited
(`postgres-and-neon-stores/_decomposition.md:69`).

**10. The slice-mate boundary is one sentence wide.** `postgres-structural-bill`
(HS-S0066) owns the *consumer-facing* half — the frontier `head`, the absence of
read-your-own-writes and the staleness bound written into the crate's rustdoc
where a caller meets them, and the recorded choice between doc prose and a clause
of its own (`_storymap.md:65`, `:142-146`). This story owns the **decision and
its numbers**, and touches the Postgres crate's docs only to retire
`event_store.rs:77`'s claim that the choice is still owed. The two are
implemented in one context as one slice; the split exists so neither AC is
reported as satisfied by the other's artefact.

**11. Who the "user" is here.** Not a screen — `_design.md` records **no
user-facing surface**, approved. The story map's own framing: the user is "an
adapter author and a consuming application", and a capability story is
user-observable as "an accepted decision atom a future reader is bound by"
(`_storymap.md:18-23`). The humane outcome this story is judged on is that a
reader arriving in a year can re-derive the choice — the arms, the numbers, the
scenario each number was taken under, and what the choice costs them — without
re-running an experiment or reading a diff.

## Integration contract

- **Archetype**: `capability` (`story.md` frontmatter, `archetype: capability`) —
  user-observable in this repository's only medium for a decision: an atom that
  validates, is indexed, and binds a future reader.
- **Slice / milestone**: `position-visibility-decision` (`story.md` frontmatter,
  `slice: position-visibility-decision`). Slice-mate: **`postgres-structural-bill`**
  (HS-S0066), implemented in the same context and mounted as one surface.
- **Mount point**: **Root D — the knowledge base.** The new decision atom
  `.kb/decisions/0024-<slug>.md`, reached through `.kb/_intake/` and
  `/redkiln:kb-ingest`, carrying its row in `.kb/maps/decision-map.md` (which
  exists and is the corpus's supersession index). Not a helper file, not a
  planning note: an ADR that lives only in `references/adr/` has not mounted,
  because `redkiln validate --kb` never sees it and `.kb/maps/decision-map.md`
  does not index it.
- **Wires into** (real sibling contracts, by path):
  - `references/adr/0024-<slug>.md` — the long form the atom links; the corpus
    convention is set by the seventeen files already in `references/adr/`.
  - `.kb/decisions/0013-position-assignment-and-visibility.md` — the atom this
    one builds on and must **not** edit (accepted decisions are immutable).
  - `.kb/open-questions/postgres-arm-c-structural-cost.md` — the question this
    record is named as the owner of (`:74-79`).
  - `.kb/open-questions/poll-count-bounds-the-visibility-rule.md` — owned by
    phase 10 per its own *What forces it*.
  - `.kb/reference/position-visibility-experiment-2026-08.md` and
    `experiments/position-visibility/` — the phase-2 measurement this one is
    paired against, including `results/staleness_pinned.txt`.
  - `crates/happenstance-postgres/src/event_store.rs` — the implemented
    `append`/`head` the number is taken against, and the module note at `:26-77`
    whose closing sentence this story retires.
  - `crates/happenstance-testkit/src/suite.rs:5880` — the rule the "hard-won"
    claim is made about.
- **Renders surfaces**: **none.** `_design.md` records no user-facing surface for
  this project and was approved on that determination; its `## Items` and
  `## Signatures` blocks are `N/A`. This story renders no surface id and adds no
  public API item.
- **Conformance rule(s)**: none added, none changed. This story is **not
  adapter-observable**: it is a record about behaviour that `postgres-append-and-frontier-head`
  and `postgres-rule-controls` already made observable through
  `event_store_conformance!` and `happenstance_testkit::rules::nothing_below_an_observed_position_appears_later`.
  It cites those runs; it does not re-run them as its own deliverable, and it adds
  nothing to `crates/happenstance-testkit/` (a must-not-change seam here:
  `postgres-and-neon-stores/_decomposition.md:102-103`).
- **Clause(s)**: **ES-10** (`spec/SPECIFICATION.md:2816-2850`) — reported on, not
  amended; its `[FROZEN]` marker is not this story's to move. **The obligation at
  `:2844-2850`** (the poll-padding decorator, "owed by phase 10 (ADR-0024)") is
  discharged or explicitly excused here. If the measurement forces a contract
  amendment, that is a *second* decision record with the suite re-run, per DR-7
  (`project.md:203-207`) — never an edit.
- **Advances DoD scenario**: initiative **DoD 5** — "A store that does not
  serialise its writers passes the suite, **with the position-visibility cost
  measured rather than estimated**" (`initiative.md:373-374`). The slice-mates
  supply the "passes"; this story is the sole owner of "measured rather than
  estimated", and DoD 5 cannot go green without it.

## PR boundary

`redkiln verify --grain story` reads the first fenced block below and fails on
any file changed outside it.

```
.bklg/from-contract-to-published-library/postgres-and-neon-stores/adr-0024-position-visibility-mechanism/**
.kb/_intake/**
references/adr/**
experiments/position-visibility/**
crates/happenstance-postgres/src/event_store.rs
.kb/decisions/**
.kb/maps/decision-map.md
.kb/open-questions/**
```

The last three globs are **deliberately wide, and here is why**: the atom, the
map row and the open-question updates are written by `/redkiln:kb-ingest` on its
own worktree branch, not by this story's implementer. They land in this story's
merge, so excluding them makes the boundary fail on the wave it was written to
produce; including them is honest about what the merge carries. Everything under
them is still governed by the harder rule above it — an accepted decision atom is
immutable, so `.kb/decisions/**` here means *one new file*, never an edit to an
existing one.

**In this PR**

- The adapter-level re-measurement harness and its committed results, under
  `experiments/position-visibility/`, tagged so it cannot overwrite the phase-2
  pass (`HS_TAG`, `experiments/position-visibility/README.md:447-456`).
- The long-form record `references/adr/0024-<slug>.md`.
- The intake document under `.kb/_intake/`, staged for the human's
  `/redkiln:kb-ingest` run, plus the resulting atom, decision-map row and
  open-question updates that wave produces.
- One rustdoc edit: `crates/happenstance-postgres/src/event_store.rs:77`'s
  closing sentence, replaced by a citation of the accepted atom.
- This story's own `_ledger.md`.

**Explicitly not in this PR**

- Any change to `append`, `head`, `read` or the schema — that is
  `postgres-append-and-frontier-head`'s landed work (constraint 5 above).
- The consumer-facing structural bill in the crate's rustdoc and the
  doc-prose-versus-clause decision — `postgres-structural-bill` (project AC-009).
- The naive-`nextval()` control and the mutant-control seam themselves —
  `postgres-rule-controls` (project AC-002/AC-004); this record *cites* their
  results.
- Any edit to `spec/SPECIFICATION.md` clause text or markers, any edit to
  `crates/happenstance-testkit/**`, any `Cargo.toml` change, and any change to
  `crates/happenstance-core/**`.
- Reopening global-versus-per-boundary visibility — owned by phase 6
  (`project.md:147-151`).

**Merge DoD**: DoD 5's second half is observable — an accepted, indexed decision
atom names the chosen mechanism and the two that lost, carries a steady-state
number and a held-transaction number both taken against the real adapter, and
`redkiln validate --kb` plus `cargo xtask affected --base main` are green.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The steady-state cost is re-measured against the **implementation**, not a SQL script | A harness drives the real `PostgresEventStore` (pool, `append` with its transaction lifetime tied to the trait method, `read` through the cursor, error mapping) under the same paired design the phase-2 pass used — baseline re-measured between arms, ratios taken against the mean of the brackets either side, residual drift reported. Absolute figures are not portable and are not claimed; the ratio and its bracketing drift are. | `experiments/position-visibility/README.md:209-259`; the four gaps at `:364-366`; `.kb/open-questions/postgres-arm-c-structural-cost.md:83-85` |
| The held-transaction scenario is measured separately and reported as its own number | Append → commit → poll a fresh snapshot until admitted, with a write transaction deliberately held open on the same cluster (phase 2 held five seconds *in a different database* and measured 4010.719 ms against a 0.688 ms control). The record states which magnitude a caller of `happenstance-postgres` should plan for, and says explicitly that the bound is a property of the cluster rather than of this store's workload. | `experiments/position-visibility/README.md:300-341`; `results/staleness_pinned.txt`; `RUNBOOK.md:4341-4342`; sub-question 3 at `.kb/open-questions/postgres-arm-c-structural-cost.md:88-92` |
| The harness stays outside the gate and outside the workspace | Added under `experiments/`, which `Cargo.toml:3`'s member globs exclude; a Rust harness there must stand alone (its own `[workspace]` table) so `cargo xtask ci`, `cargo hack`'s powersets and `cargo deny` do not acquire it, and it must add no dependency to any crate manifest. Results are committed with an `HS_TAG` suffix so the phase-2 pass is not overwritten. | `experiments/position-visibility/README.md:21-23`, `:447-456`; `CLAUDE.md` repository map (`experiments/` is "not in the gate") |
| The structural cost is answered, not asserted | The record answers sub-question 1 directly: whether `sqlx`'s connection pooling and a transaction whose lifetime is tied to `append`'s async boundary expressed the mechanism cleanly, or forced a design compromise — naming each of the four gaps (pooling, transaction lifetime, cursor, error mapping) and what it actually cost. The read-side half — the visibility predicate composed *into* the cursor's `DECLARE` inside one `REPEATABLE READ` transaction rather than evaluated per chunk — is reported as part of that cost. | `.kb/open-questions/postgres-arm-c-structural-cost.md:64-70`; `crates/happenstance-postgres/src/read_stream.rs:37-77`; `postgres-and-neon-stores/_decomposition.md:209-216` |
| The two losing arms are named and priced, including the one that loses for a reason throughput cannot show | A serialised sequence table and a constant advisory lock lose on cost (16× and 30× at 64 writers, throughput flat from 8 clients up) — they buy ES-10 by deleting the reason for the adapter. The tag-keyed advisory lock loses on **correctness of the wrong invariant**: it is nearly free and reproduces the baseline inversion on disjoint keys, because it implements a per-boundary property while ES-10 states a global one. The record must state that distinction rather than rank arms by tps. | `experiments/position-visibility/README.md:128-184`, `:263-296`, `:398-412`; `crates/happenstance-postgres/src/event_store.rs:43-75` |
| The global premise is declared inherited, not owned | The record says in its own words that ES-10's global framing rests on the projection checkpoint being global, that this is unsettled and owned by phase 6, and that if phase 6 lands a boundary-scoped checkpoint then ADR-0013 reopens and this measurement reopens with it. | `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`; `postgres-and-neon-stores/_decomposition.md:170`, `:460-463` |
| "Hard-won" is evidenced by citation, not by adjective | The record cites the naive-`nextval()` control failing `nothing_below_an_observed_position_appears_later` and the concurrency family green at `CONTENDERS = 8` under a multi-thread runtime, each by the run that produced it, and states what the passing mechanism cost. | `crates/happenstance-testkit/src/suite.rs:5880`; `RUNBOOK.md:4377-4378`; `postgres-and-neon-stores/_decomposition.md:512`, `:539-555` |
| The poll-padding obligation is discharged or excused, in writing | Either the decorator over `PreCommitPositionStore` exists and its result is reported (does the rule still reject an *n*-poll implementation?), or the record states why the real `append`'s poll shape made it unnecessary. The clause's own terms apply: if it fires, the rule changes and ES-10 does not. | `spec/SPECIFICATION.md:2844-2850`; `.kb/open-questions/poll-count-bounds-the-visibility-rule.md:74-99` |
| The record reaches `.kb/` through the process, and only through it | Intake document staged under `.kb/_intake/`; long form at `references/adr/0024-<slug>.md`; atom authored by `/redkiln:kb-ingest` (human-invoked) with valid `KbFrontmatter`, `kind: decision`, `status: accepted`, linking the long form; row added to `.kb/maps/decision-map.md`. `redkiln validate --kb` and `redkiln doctor` clean, `doctor` still reporting exactly six `template-drift` advisories. | `postgres-and-neon-stores/_decomposition.md:57`, `:159-164`; `CLAUDE.md` (*Where the work lives*, the `0269720` reversion); `.kb/_intake/README.md:13-19` |
| A refutation is a recorded outcome, not a silent re-wire | If the measurement refutes the wired mechanism, this story delivers the record saying so — what would have to change, and whether arm B-tag becomes reconsiderable in light of phase 6 — plus a re-plan input. It does not change `append` inside its own diff. | `.kb/open-questions/postgres-arm-c-structural-cost.md:93-96`; `project.md` risk table (`:335-336`); `postgres-and-neon-stores/_decomposition.md:455-459` |
| Nothing frozen is edited, and the stale in-tree claim is retired | `spec/SPECIFICATION.md` clause text and markers are untouched (`cargo xtask spec-trace` green). The one code-side change is `crates/happenstance-postgres/src/event_store.rs:77` — "Nothing above is a measurement, and the choice is owed one" — replaced by a pointer to the accepted atom, so the crate's own docs stop asserting an open question that closed. | `spec/SPECIFICATION.md:2823`; `CLAUDE.md` binding-constraints preamble; commit `ce933d8` (*Point every ADR reference at the atom*) |

## Data and migrations

**N/A for schema.** This story adds no table, column, index or migration. The
mechanism's column and migration 1 belong to `postgres-schema-and-live-fixture`
(HS-S0061) and were landed there
(`postgres-and-neon-stores/_decomposition.md:235`, `_storymap.md:60`); this story
measures what that schema and `postgres-append-and-frontier-head`'s append path
already do, and changes neither.

**Data this story does write** — measurement artefacts, not application data:

| Artefact | Where | Constraint |
| --- | --- | --- |
| Adapter-level throughput/ratio series | `experiments/position-visibility/results/` | Suffixed via `HS_TAG` so it sits beside, never on top of, the phase-2 pass; raw per-run reports committed as produced (`README.md:447-456`, `:463-491`) |
| Held-transaction staleness transcript | `experiments/position-visibility/results/` | Same tagging rule; the control (no holder) and the held case are both committed, because the pair is the finding (`README.md:326-332`) |
| The long-form record | `references/adr/0024-<slug>.md` | Carries the transcripts and tables the atom cannot hold; cited by `file:line`, never deleted in favour of the atom (`CLAUDE.md`) |
| The intake document | `.kb/_intake/` | Consumed and cleared by the ingest wave — "a file still sitting here after a run is a file that run did not ingest" (`.kb/_intake/README.md:13-19`) |
| The decision atom and its index row | `.kb/decisions/0024-<slug>.md`, `.kb/maps/decision-map.md` | Written by `/redkiln:kb-ingest` only; immutable once accepted |

## Acceptance criteria

The "user" here is fixed by the context pack (§11) and by
`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`:
**the adapter author** (persona 2, journey *Learn when you are finished*), **the
evaluator** (persona 4, journey *Decide in one sitting*) and **the application
author** (persona 1, journey *Choose a contract before a database*). Each
criterion below is a goal one of them holds, crossing the whole stack from the
running cluster to the atom they read. The testing brief is explicit that
AC-001's number "is proven by the record existing and validating, not by a
pass/fail rule"
(`.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md:511`);
the verification column therefore names process gates and committed artefacts
where no rule can exist, and never invents one.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | GIVEN an evaluator deciding in one sitting whether `happenstance-postgres` is affordable, WHEN they open the accepted decision atom, THEN they find the steady-state cost of the chosen mechanism taken against the **built** `PostgresEventStore` — connection pool, `append`'s transaction lifetime tied to the trait method, cursor read, error mapping all present — stated as a ratio against a baseline re-measured between arms with its residual drift reported, under `fsync=on`, and explicitly **not** the phase-2 SQL-script figure and not a preference. | Static / process: `redkiln validate --kb` over `.kb/decisions/0024-<slug>.md`; the committed run under `experiments/position-visibility/results/` carrying an `HS_TAG` suffix (`experiments/position-visibility/README.md:447-456`); a human read of the atom against the paired-design method at `experiments/position-visibility/README.md:209-259`. No pass/fail rule exists or is invented (`postgres-and-neon-stores/_decomposition.md:511`). |
| AC-002 | GIVEN an application author who must size a read-after-write expectation before choosing a database, WHEN they read the record, THEN the held-transaction scenario is reported as its **own** number — append → commit → poll a fresh snapshot until admitted, with a write transaction deliberately held open on the same cluster — placed against its no-holder control, with a stated magnitude the caller should plan for out of the phase-2 range (0.688 ms → 4010.719 ms), and with the bound named as a property of the *cluster*, not of this store's workload. | Static / process: both transcripts (control and held) committed side by side under `experiments/position-visibility/results/` under the same `HS_TAG` rule (`experiments/position-visibility/README.md:326-332`); `redkiln validate --kb`; human read against DR-3 (`postgres-and-neon-stores/project.md:180-187`), `RUNBOOK.md:4341-4342` and sub-question 3 (`.kb/open-questions/postgres-arm-c-structural-cost.md:88-92`). A record carrying only a steady-state ratio fails this AC. |
| AC-003 | GIVEN an adapter author about to build the seventh store and wanting to know what this mechanism costs to *express*, WHEN they read the record's structural-cost section, THEN each of the four gaps the phase-2 harness never had — connection pooling, a transaction whose lifetime is tied to `append`'s async boundary, the cursor, error mapping — is named with what it actually cost, and the read-side half (the visibility predicate composed into the cursor's `DECLARE` inside one `REPEATABLE READ` transaction rather than evaluated per chunk) is priced as part of that bill rather than omitted. | Static / process: `redkiln validate --kb`; human read of the record's structural-cost section against sub-question 1 (`.kb/open-questions/postgres-arm-c-structural-cost.md:64-70`), `experiments/position-visibility/README.md:364-366` and the shipped read path `crates/happenstance-postgres/src/read_stream.rs:37-77`. An assertion with no gap named is undischarged. |
| AC-004 | GIVEN a future reader who must be able to re-derive the choice without re-running an experiment, WHEN they read the record, THEN both losing arms are named **and priced** — the serialised sequence table and the constant advisory lock losing on cost (16× and 30× at 64 writers, throughput flat from 8 clients up: they buy ES-10 by deleting the reason to reach for Postgres) and the tag-keyed advisory lock losing on **the correctness of a different invariant** (nearly free, reproduces the baseline inversion on disjoint keys, because it implements a per-boundary property where ES-10 states a global one) — so that the arms are not ranked by tps. | Static / process: `redkiln validate --kb`; human read against `experiments/position-visibility/README.md:128-184`, `:263-296`, `:398-412` and the in-tree prose at `crates/happenstance-postgres/src/event_store.rs:43-75`. DR-7 requires the losers be named and why (`postgres-and-neon-stores/project.md:203-207`). |
| AC-005 | GIVEN an adapter author who will inherit this decision after phase 6 has shaped the projection checkpoint, WHEN they read the record, THEN it says in its own words that ES-10's **global** framing rests on the checkpoint being global, that this is unsettled and owned by phase 6, and that a boundary-scoped checkpoint reopens ADR-0013 and this measurement with it — stated as an inherited premise this record does not own, never as a permanently settled invariant. | Static / process: `redkiln validate --kb`; the atom's outbound link to `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md` present and reciprocal after the ingest wave; human read against `postgres-and-neon-stores/_decomposition.md:170`, `:460-463`. No edit to `.kb/decisions/0013-position-assignment-and-visibility.md` — accepted atoms are immutable and `redkiln validate --kb` fails the build if one is touched. |
| AC-006 | GIVEN an adapter author asking "did this rule actually cost the implementer anything, or is it decorative?", WHEN they read the record's evidence, THEN the claim that `nothing_below_an_observed_position_appears_later` was hard-won is carried by **citation of the two predecessor runs** — the naive-`nextval()` control observed *failing* that rule, and the concurrency family green at `CONTENDERS = 8` under a multi-thread runtime with writers unserialised — each attributed to the run that produced it, with what the passing mechanism cost stated. | Static / process: `redkiln validate --kb`; human read confirming each citation resolves to a real run from `postgres-rule-controls` (HS-S0064) and `postgres-concurrency-family` (HS-S0063) and to `crates/happenstance-testkit/src/suite.rs:5880`. Phase 10's exit criterion is exactly this (`RUNBOOK.md:4377-4378`; `postgres-and-neon-stores/_decomposition.md:512`, `:539-555`). The two conformance runs are **cited, not re-run** by this story. |
| AC-007 | GIVEN the specification's standing debt at `spec/SPECIFICATION.md:2844-2850` — a poll-padding decorator over `PreCommitPositionStore`, owed by phase 10 — WHEN a reader arrives at ES-10 wanting to know whether the rule's strength was ever bounded, THEN this record either reports the decorator's result (does the rule still reject an *n*-poll implementation?) or states in writing why a real multi-poll `append` made it unnecessary. Silence is a failure of this AC, not a neutral outcome. | Static / process: `redkiln validate --kb`; `cargo xtask spec-trace` green with ES-10's clause text and markers **unchanged**; human read against `.kb/open-questions/poll-count-bounds-the-visibility-rule.md:74-99`. If the decorator fires, the clause's own escape hatch applies — the rule changes and ES-10 does not — and that outcome is recorded here and handed to a rule change outside this PR. |
| AC-008 | GIVEN a reader who meets the question in the crate before they meet the answer in the knowledge base, WHEN the merge lands, THEN the decision has **mounted at Root D** — an accepted atom at `.kb/decisions/0024-<slug>.md` with valid `KbFrontmatter` (`kind: decision`, `status: accepted`), authored by `/redkiln:kb-ingest` and never by hand, linking its long form at `references/adr/0024-<slug>.md` which is retained rather than folded into the atom, carrying its row in `.kb/maps/decision-map.md`, with `.kb/_intake/` emptied by the wave — **and** `crates/happenstance-postgres/src/event_store.rs:77`'s now-false sentence "Nothing above is a measurement, and the choice is owed one" replaced by a pointer to that atom, so the reader is answered where they asked. | Static / process: `redkiln validate --kb` (KbFrontmatter conformance plus accepted-decision immutability) and `redkiln doctor` clean, still reporting exactly six `template-drift` advisories; `.kb/_intake/` empty after the wave (`.kb/_intake/README.md:13-19`); `cargo xtask affected --base main` and `cargo xtask ci --fast` green over the rustdoc edit; `test -f references/adr/0024-<slug>.md`. Hand-authored atoms are the failure `0269720` reverted (`CLAUDE.md`). |

Coverage of the traced project AC: **AC-001** of
`postgres-and-neon-stores/project.md` ("ADR-0024 accepted, numbers included") is
satisfied by AC-001 + AC-002 (the two numbers), AC-003 + AC-004 (the choice and
its bill), AC-005 + AC-006 + AC-007 (the premises, the evidence and the owed
instrument) and AC-008 (the mount). No other project AC is claimed here; the
project's AC-002, AC-003, AC-004 and AC-009 belong to the slice-mates and
predecessors named in the PR boundary.

## Interaction quality

RFC §6.7/D6. This story renders **no surface**: the project's signed-off design
records `N/A — no user-facing surface` under every heading including
`## Anti-patterns`, approved 2026-08-12 with no conditions
(`postgres-and-neon-stores/_design.md:78-95`). What follows therefore splits
honestly rather than pretending a screen exists.

**COMPOSITION family — not applicable, by an approved determination.** There is
no presentation, placement, transience policy, density budget or hierarchy to
hold, and the design records **no named anti-patterns** to bind against
(`_design.md:78-80`). No composition invariant is written as an AC row here,
because inventing one would contradict a design a human signed off — and a
fabricated density budget for a markdown atom is exactly the decorative check
CLAUDE.md's "a rule that no adapter can fail is decorative" warns against. The
one composition-adjacent constraint that *is* real is structural rather than
visual, and it is carried by **AC-008**: the atom is the composed artefact, and
an ADR that lives only in `references/adr/` has not mounted, because
`redkiln validate --kb` never sees it and `.kb/maps/decision-map.md` does not
index it.

**STATE family — applicable, in the knowledge base's medium, and every invariant
below is carried by a numbered AC row above, not by this prose.**

| Invariant (state family) | Its meaning in this medium | Carried by | How it is verified |
| --- | --- | --- | --- |
| **In place, not a context jump** | The reader who meets the open question in `crates/happenstance-postgres/src/event_store.rs:26-77` is answered *there*, by a pointer to the accepted atom, rather than left with a crate whose own docs assert a question that closed. | **AC-008** (second clause) | Rustdoc build inside `cargo xtask ci --fast`; human read of the retired sentence at `:77`. |
| **Non-occlusion** | The atom does not hide what it summarises: `references/adr/0024-<slug>.md` is retained with the transcripts and tables the atom cannot hold, and the new results are added *beside* the phase-2 pass, never on top of it (`HS_TAG`). | **AC-008**, **AC-001**, **AC-002** | `test -f references/adr/0024-<slug>.md`; the tagged result files present alongside an untouched phase-2 pass in the diff. |
| **Preserved state — nothing already settled is disturbed** | `.kb/decisions/0013-…` is cited, never edited; `spec/SPECIFICATION.md`'s `[FROZEN]` ES-10 text and markers are untouched; `crates/happenstance-testkit/**` gains nothing. | **AC-005**, **AC-007** | `redkiln validate --kb`'s accepted-decision immutability check against `HEAD`; `cargo xtask spec-trace` green; `cargo xtask affected --base main`. |
| **Reversibility** | A wrong decision here is reversed by **supersession**, never by an edit — and the record itself names the condition under which it reopens (a boundary-scoped checkpoint from phase 6). | **AC-005** | Supersession graph enforced by `redkiln validate --kb`; the reopening condition read out of the atom's own text. |
| **Reachability without inside knowledge** | The keyboard analogue: a reader arriving cold reaches the decision through the corpus's own index — `.kb/maps/decision-map.md` — rather than by knowing the filename. | **AC-008** | The map row present and resolving; `redkiln doctor` clean. |
| **Re-derivability** | The humane outcome this story is judged on (context pack §11): a reader in a year re-derives the choice — arms, numbers, the scenario each number was taken under, and what it costs them — without re-running an experiment or reading a diff. | **AC-001** – **AC-004** together | Human read of the atom plus its long form; the committed results are the receipt, and NF-005 requires the reproduction command line beside them. |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| EC-001 | The re-measurement **refutes** the wired mechanism — arm C is structurally expensive through `sqlx`, or the staleness bound is unacceptable under a real workload. | Deliver the record saying so: what would have to change, and whether arm B-tag becomes reconsiderable in light of phase 6 (`.kb/open-questions/postgres-arm-c-structural-cost.md:93-96`), **plus a re-plan input**. Do **not** swap the append path inside this story's diff — that is `postgres-append-and-frontier-head`'s landed work (`postgres-and-neon-stores/project.md:335-336`; `postgres-and-neon-stores/_decomposition.md:455-459`). A refutation is a satisfied AC-004, not a failed story. |
| EC-002 | The implementer reaches the ledger with only an intake document staged, because `/redkiln:kb-ingest` carries `disable-model-invocation` and is a **human** handoff. | AC-008 stays `satisfied: false`. The intake file is not evidence for the mount; the ledger row is flipped only with a `file:line` under `.kb/decisions/0024-<slug>.md` and a `.kb/maps/decision-map.md` row that exist after the wave. Report the handoff and stop; never hand-author the atom (`CLAUDE.md`; the `0269720` reversion). |
| EC-003 | The live Postgres cluster is unreachable when the harness is run — no Docker, no credential, container pull fails. | Halt and report. Do **not** substitute the phase-2 SQL-script figures, do not estimate, and do not mark AC-001/AC-002 satisfied from the existing `experiments/position-visibility/results/ratios.csv` — the whole delta this story adds is "a number against an implementation rather than against a SQL script". DR-9 keeps this out of the default gate, so an unreachable cluster blocks *this story*, never `cargo xtask ci`. |
| EC-004 | The new harness is added in a way that joins the Cargo workspace — a crate under a member glob, or a `[dependencies]` edge into a workspace manifest. | Fail the story's own boundary check. `Cargo.toml:3`'s `members = ["crates/*", "examples/*", "xtask"]` excludes `experiments/` by construction; a Rust harness there must carry its own `[workspace]` table so `cargo xtask ci`, `cargo hack`'s powersets and `cargo deny` never acquire it (`experiments/position-visibility/README.md:21-23`). Detected by `cargo xtask ci --fast`'s step list being unchanged in kind and by no manifest appearing in the diff. |
| EC-005 | `HS_TAG` is unset and the run overwrites the phase-2 pass it is being compared against. | Treated as data loss. The phase-2 files under `experiments/position-visibility/results/` are the comparand for AC-001; restore them from `HEAD` and re-run tagged (`experiments/position-visibility/README.md:447-456`). A diff that *modifies* rather than *adds* result files fails review. |
| EC-006 | An accepted atom is edited — most likely `.kb/decisions/0013-position-assignment-and-visibility.md`, in the temptation to "update" it with the new number. | `redkiln validate --kb` fails on immutability against `HEAD`, by design. The correction path is a new atom that supersedes, never an edit (`CLAUDE.md`, *Where the work lives*). ADR-0013 is cited and built on here; it is not amended. |
| EC-007 | `.kb/_intake/` still holds this story's document after the ingest wave ran. | The wave did not ingest it — "a file still sitting here after a run is a file that run did not ingest" (`.kb/_intake/README.md:13-19`). AC-008 is unsatisfied; investigate the wave rather than deleting the file. Two known traps: the intake glob includes the intake `README.md`, which is dropped at the wave's approval gate, and a second wave's id must be suffixed so it does not overwrite the first's audit trail. |
| EC-008 | The poll-padding decorator was never built by `postgres-rule-controls` and the implementer cannot say why it was unnecessary. | AC-007 is unsatisfied and the story halts — the obligation at `spec/SPECIFICATION.md:2844-2850` names phase 10 as owner and admits exactly two outcomes. Building the decorator sits in `postgres-rule-controls`' boundary, not this one; a genuine absence is a dependency failure to report upward, not a silent excuse written into the record. |

## Non-functional

| id | Requirement | Why, and where it is checked |
| --- | --- | --- |
| NF-001 | The default gate stays **Docker-free and network-free**. Nothing this story adds is reachable from `cargo xtask ci` or `cargo xtask ci --fast`. | DR-9 (`postgres-and-neon-stores/project.md`), project AC-011, `.redkiln/config.yaml:55`. Checked by running `cargo xtask ci --fast` on a machine with no cluster and observing zero new steps and zero new failures. |
| NF-002 | The harness adds **no dependency to any `Cargo.toml`** in the workspace and is not a workspace member. | `experiments/position-visibility/README.md:21-23`; `Cargo.toml:3`. Checked by the absence of any manifest in this story's diff — no `Cargo.toml` glob appears in the PR boundary block. |
| NF-003 | The measurement keeps phase 2's method: `fsync=on` (never `-c fsync=off`), the **paired** design with the baseline re-measured between arms, ratios taken against the mean of the brackets either side, residual drift reported, and absolute figures explicitly not claimed as portable. | `experiments/position-visibility/README.md:209-259` and its two standing warnings at `:453-457`; `.kb/open-questions/postgres-arm-c-structural-cost.md:83-85`. Checked by human read of the committed run and of the record's methods note. |
| NF-004 | `redkiln doctor` still reports **exactly six** `template-drift` advisories after the merge — no more, no fewer. | `CLAUDE.md` (the six deliberately customised templates; the `backlog` CI job asserts the set). A seventh means a template changed without a decision; a missing one means `adopt --templates` was run, which is forbidden here. |
| NF-005 | The record is **re-derivable**: the exact reproduction command line — the `run.sh` invocation and every `HS_*` knob used — is committed beside the results, and the environment is captured the way `results/environment.txt` captures it for the phase-2 pass. | `experiments/position-visibility/README.md:440-456` (the driver and its knobs); `experiments/position-visibility/results/environment.txt`. Checked by a reader re-running from the committed line alone. |
| NF-006 | No MSRV change, no manifest change, no new dependency anywhere in `crates/**`, and no change to `crates/happenstance-core/**` or `crates/happenstance-testkit/**`. | The PR boundary block above; `postgres-and-neon-stores/_decomposition.md:102-103` names the testkit as a must-not-change seam. Checked by `cargo xtask affected --base main` and by the boundary check `redkiln verify --grain story` runs. |
| NF-007 | The atom stays **atom-sized** — roughly the ~100-line shape the other seventeen hold — with transcripts, tables and rejected-alternative detail pushed to the long form and cited by `file:line`. | `CLAUDE.md`, *Where the work lives*: deleting the long form would discard about 78% of the corpus, and `spec-trace` cites line ranges that exist only there. Checked by human read against the existing files in `.kb/decisions/`. |

## Implementation notes (non-prescriptive)

Not instructions — the traps this story's shape actually sets. Every one is the
implementer's call to make differently, with a reason.

- **Order the work so the record is written last.** Run the harness, read the
  numbers, *then* write the record. The failure mode this story is most exposed
  to is a record drafted from the phase-2 figures with the re-measurement bolted
  on: it produces prose arguing for a conclusion the new numbers were never
  allowed to disturb, and EC-001 becomes unreachable by construction.
- **The four ordered sub-questions at
  `.kb/open-questions/postgres-arm-c-structural-cost.md:83-96` are the shape of
  the record's measured-cost section**, in that order, per the story map's own
  framing (`_storymap.md:131-140`). Answering them out of order is fine;
  answering three of the four is not.
- **Extending the existing harness is likely cheaper than writing a second one.**
  `run.sh`'s driver, `container/env.sh`'s knobs and the `results/` layout already
  exist and already carry the `HS_TAG` discipline
  (`experiments/position-visibility/README.md:440-491`). A new arm that drives the
  real `PostgresEventStore` instead of a `pgbench` script is a fifth entry in an
  existing structure, not a new experiment — but it is a **Rust** program where
  the others are SQL, so it needs its own standalone `[workspace]` table (EC-004).
- **The held-transaction scenario is a different script from the throughput
  scenario.** Phase 2 kept them apart on purpose — `container/staleness.sh` versus
  `container/staleness-pinned.sh`. Merging them loses the control pair that *is*
  the finding (`experiments/position-visibility/README.md:326-332`).
- **The two predecessor runs are cited, not re-executed.** `postgres-rule-controls`
  owns the naive-`nextval()` control and `postgres-concurrency-family` owns the
  `CONTENDERS = 8` run. Re-running them here to "have the evidence locally" widens
  the diff into two other stories' boundaries and reports their ACs as satisfied by
  this story's artefact — the exact confusion the slice split exists to prevent
  (context pack §10).
- **Draft the intake document to be ingested, not to be pretty.** `/redkiln:kb-ingest`
  extracts claims and adjudicates them; it prefers merge/amend over new atoms and it
  will *not* edit an accepted decision body. Writing the intake so that ADR-0013
  reads as needing amendment sets the wave up to fail rather than to supersede.
- **The rustdoc edit is one sentence.** `crates/happenstance-postgres/src/event_store.rs:77`.
  The temptation is to expand the module note with the new numbers while the file is
  open; that is `postgres-structural-bill`'s project AC-009 and belongs to the
  slice-mate (`_storymap.md:142-146`).
- **If the poll-padding decorator did run and *did* find an `n` the rule fails to
  reject**, the clause's own terms apply — "the rule changes and this clause does
  not" (`spec/SPECIFICATION.md:2844-2850`). Record the finding and hand the rule
  change onward; changing `crates/happenstance-testkit/**` is outside this boundary
  in either direction.

## Tests and CI (merge gate)

Grounded in the project testing brief
(`postgres-and-neon-stores/_decomposition.md` — the AC-001 row at `:511`, Notes §1
at `:527-537`, Notes §2 at `:539-555`, and Notes §6's merge-gate command list).
This story is the project's one AC that is **not** proven by a conformance rule,
and the brief says so; the table is honest about that rather than manufacturing a
tier.

| Tier | Command / path | Proves |
| --- | --- | --- |
| Static / process — KB | `redkiln validate --kb` | The atom exists at `.kb/decisions/0024-<slug>.md` with valid `KbFrontmatter`, `status: accepted`, resolving outbound links, and **no accepted atom edited** (immutability checked against `HEAD`). Gates AC-001 – AC-008; it is the only automatic gate AC-005's "0013 untouched" has. |
| Static / process — backlog | `redkiln doctor` | The item tree is clean and exactly six `template-drift` advisories remain (NF-004). Gates AC-008, NF-004. |
| Static / process — spec | `cargo xtask spec-trace` | ES-10's `[FROZEN]` marker and every cited line range still resolve — this story reported on the clause without editing it and did not break the citations its long form is about to join. Gates AC-007 and the "nothing frozen is edited" row of *Behavior and interfaces*. |
| Gate — story grain | `cargo xtask affected --base main` | The one rustdoc edit at `crates/happenstance-postgres/src/event_store.rs:77` compiles, documents and lints clean, and nothing else in `crates/**` moved. `.redkiln/config.yaml` wires this as the story grain. Gates AC-008, NF-006. |
| Gate — project bar | `cargo xtask ci --fast` | The non-terminal project bar (`.redkiln/config.yaml:55`): fmt, clippy `-D warnings`, `cargo test --workspace --all-features`, wasm32, docs, `spec-trace`, `package-check` — all still green with **no Docker and no network**, which is simultaneously the check that this story added nothing to the gate. Gates NF-001, NF-002, EC-004. |
| Measurement — deliberately outside the gate | `bash experiments/position-visibility/run.sh …` with `HS_TAG` set, extended to drive the real `PostgresEventStore` | The steady-state ratio (AC-001) and the held-transaction pair (AC-002), committed as produced under `experiments/position-visibility/results/`. **Not a `cargo xtask ci` step, not a workspace member, adds no dependency** (`experiments/position-visibility/README.md:21-23`; `CLAUDE.md` repository map). A failure here blocks the story, never the gate. |
| Conformance — **cited, not re-run** | `crates/happenstance-testkit/src/suite.rs:5880` (`nothing_below_an_observed_position_appears_later`) and `event_store_concurrency_conformance!` at `CONTENDERS = 8`, as run by HS-S0064 and HS-S0063 in the live Postgres CI job | The "hard-won" claim of AC-006. This story adds no rule, changes no rule and re-runs neither family as its own deliverable; it links the runs that produced the outcomes (`postgres-and-neon-stores/_decomposition.md:102-103`, `:539-555`). |
| Human read — the only instrument for a record | The atom, plus `references/adr/0024-<slug>.md` | AC-003, AC-004, AC-005, AC-007 in full, and NF-003/NF-005/NF-007. No command can tell you whether both losers were priced, whether the global premise was declared inherited, or whether a reader in a year can re-derive the choice. The testing brief states the position plainly: this AC "is proven by the record existing and validating, not by a pass/fail rule" (`:511`). |

**Merge gate, as one list.** `redkiln validate --kb` ∧ `redkiln doctor` ∧
`cargo xtask spec-trace` ∧ `cargo xtask affected --base main` ∧
`cargo xtask ci --fast`, plus the committed measurement artefacts and the human
read. No live-infrastructure CI job is added or changed by this story — the
Postgres job is `postgres-schema-and-live-fixture`'s deliverable and this story
consumes its output.

## Risks and coupling (PR-scoped)

| Risk | Why it is live in *this* PR | Mitigation inside this PR |
| --- | --- | --- |
| The number is taken against one machine and read as a portable fact | Phase 2 already warns absolute figures are not portable; a record stating "0.9 ms" without its environment invites a consumer to plan against a laptop | NF-003 and NF-005: ratios plus bracketing drift, `environment.txt` committed, the reproduction line beside the results, and the record saying which claims are portable (the ratio) and which are not (the absolutes) |
| The record is written from the phase-2 numbers with the re-measurement as garnish | It is the cheapest way to satisfy the shape of this story while adding none of its value; the whole delta is "an implementation, not a SQL script" | Implementation-notes bullet 1 (measure, then write); AC-001 and AC-002 both require artefacts under an `HS_TAG` suffix that cannot exist without a new run |
| The mount straddles two branches, so the merge looks incomplete at review time | `/redkiln:kb-ingest` is a human handoff that commits on its own worktree branch, so this story's implementer legitimately cannot produce the atom | The PR boundary already admits `.kb/decisions/**`, `.kb/maps/decision-map.md` and `.kb/open-questions/**` and explains why; EC-002 fixes the ledger discipline — intake is not evidence |
| The slice-mate consumes a staleness figure that then moves | `postgres-structural-bill` (HS-S0066) writes the cluster-wide staleness bound into the crate's public rustdoc, sourced from AC-002 | Sequencing is explicit: HS-S0066 is `blocked_by` HS-S0065 (`story.md` `blocks: [HS-S0066]`; `_storymap.md:251-254`). If AC-002's figure changes after HS-S0066 starts, that is a re-plan input, not a quiet doc edit |
| Phase 6 lands a boundary-scoped checkpoint and this measurement is stale on arrival | The global-versus-per-boundary question is real, unsettled and owned elsewhere (`postgres-and-neon-stores/project.md`'s risk table; `_decomposition.md:460-463`) | AC-005 makes the inherited premise and its reopening condition a criterion, so a future reader is never misled about the record's standing |
| The measurement refutes the mechanism and someone "just fixes" `append` | The file is already open for a one-line rustdoc edit, and the fix looks small | EC-001 plus the PR boundary: `crates/happenstance-postgres/src/event_store.rs` is admitted for **one** sentence; any change to `append`, `head`, `read` or the schema fails the boundary check |
| A second, deliberately-broken Postgres fixture is kept alive to "have the control handy" | The testkit already keeps one canonical wrong implementation per axis, and duplicating the pattern without a decision is an undecided addition | The control is `postgres-rule-controls`' throwaway, cited here; this story adds nothing to `crates/happenstance-testkit/**` (Testing brief Notes §2, `:539-555`) |

## Dependencies

**Blocks on** — all three must be merged; `story.md` frontmatter carries
`blocked_by: [HS-S0062, HS-S0063, HS-S0064]`.

| Story slug | Item | What this story cannot do without it |
| --- | --- | --- |
| `postgres-append-and-frontier-head` | HS-S0062 | The implementation the number is taken *against*. Without it there is no pool, no `append` transaction lifetime, no cursor and no error mapping — nothing distinguishing AC-001 from the phase-2 SQL-script measurement (`_storymap.md:96-108`) |
| `postgres-concurrency-family` | HS-S0063 | The green `CONTENDERS = 8` run under a multi-thread runtime that AC-006 cites as half the "hard-won" evidence, and that proves the chosen mechanism did not re-serialise writers (DR-2) |
| `postgres-rule-controls` | HS-S0064 | The naive-`nextval()` control observed *failing* `nothing_below_an_observed_position_appears_later` — the other half of AC-006 — and the poll-padding decorator, or its recorded absence, that AC-007 discharges or excuses (`_storymap.md:126-129`) |

**Unlocks:**

| Story slug | Item | What it takes from here |
| --- | --- | --- |
| `postgres-structural-bill` | HS-S0066 | Slice-mate, implemented in the same context. Takes AC-002's staleness bound and AC-003's structural findings and writes the consumer-facing half into the Postgres crate's rustdoc, plus the recorded doc-prose-versus-clause choice (project AC-009) |
| `deskeleton-and-package-readiness` | HS-S0067 | Transitively, through HS-S0066: the crate cannot stop being a skeleton while its own module docs assert an open question this story closes |
| `far-end-discharge-record` | HS-S0068 | Reads ES-10's post-work status — discharged, still exposed, or amended, and against which store — which is exactly what AC-005 and AC-007 record |

Merge order within the slice: `adr-0024-position-visibility-mechanism`, then
`postgres-structural-bill` — "the ADR lands *after* a working append path because
the number it owes is a re-measurement against the adapter, not against four SQL
strategies" (`_storymap.md:251-254`).

## Anchors (progressive disclosure)

Everything below is deferred, not optional. Open each at the moment named; link,
never bulk-paste. All paths verified present in the worktree.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.kb/open-questions/postgres-arm-c-structural-cost.md` | The question this record is named as the owner of, and whose four ordered sub-questions **are** the shape of the measured-cost section (`:83-96`). `:64-70` states the four gaps; `:88-92` states that which end of the 0.688 ms → 4010.719 ms range a caller should plan for is the open part | Before running the harness — it decides what you must measure, not merely what you must write | AC-001, AC-002, AC-003 |
| `experiments/position-visibility/README.md` | The method this re-measurement must keep: the paired design and drift reporting (`:209-259`), the four missing gaps (`:364-366`), the staleness pair (`:300-341`), the arm prices (`:128-184`, `:263-296`, `:398-412`), the `HS_TAG` rule and knobs (`:440-456`), and the standing "do not add `fsync=off`" warning | Before writing a line of harness code; again when committing results | AC-001 |
| `experiments/position-visibility/results/staleness_pinned.txt` | The phase-2 held-transaction transcript AC-002's new number is placed against — 4010.719 ms behind an unrelated five-second write, versus a 0.688 ms control | While designing the held-transaction scenario, and again when stating the magnitude a caller should plan for | AC-002 |
| `.kb/decisions/0013-position-assignment-and-visibility.md` | The atom this one builds on: it lifted ES-10 to `[FROZEN]` on arm C's ratios and says in terms that it does **not** choose the adapter's mechanism. **Immutable** — cite it, never edit it | Before drafting the record's premise section; before touching anything under `.kb/decisions/` | AC-005 |
| `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md` | The premise AC-005 requires be declared *inherited*: ES-10's global framing rests on a global projection checkpoint, which phase 6 owns and has not settled | When writing the premise/standing section of the record | AC-005 |
| `.kb/open-questions/poll-count-bounds-the-visibility-rule.md` | Carries the exact instrument the specification owes (`:74-99`) — a poll-padding decorator over `PreCommitPositionStore` — and why phase 10 rather than phase 4 was named its owner (an adapter with real I/O). AC-007 is discharged or excused against this text | After the predecessor stories are read, before deciding whether AC-007 is a report or an excuse | AC-007 |
| `spec/SPECIFICATION.md` | ES-10 at `:2816-2842` with its `[FROZEN]` marker at `:2823`, and the phase-10 obligation at `:2844-2850`. This record **reports on** the clause; the marker is not this story's to move | When writing the ES-10 section of the record; before running `cargo xtask spec-trace` | AC-007 |
| `crates/happenstance-postgres/src/event_store.rs` | The implementation the number is taken against, the in-tree arm analysis at `:26-77`, the "`MemoryEventStore` with network latency" argument against the serialised arms at `:43-75`, and the one sentence this story retires at `:77` | Before AC-004's pricing of the losing arms; last, for the single rustdoc edit | AC-004 |
| `crates/happenstance-postgres/src/read_stream.rs` | The read-side half of the structural bill: the visibility predicate composed into the cursor's `DECLARE` inside one `REPEATABLE READ` transaction (`:37-77`) rather than evaluated per chunk | When answering sub-question 1 — the cursor is one of the four gaps and its cost is the easiest to omit | AC-003 |
| `crates/happenstance-testkit/src/suite.rs` | Line 5880 is `nothing_below_an_observed_position_appears_later` — the rule the entire "hard-won" claim is made about. Read it to state accurately what the naive arm failed | When writing the evidence section; never to change it (must-not-change seam) | AC-006 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md` | Architecture's Root D mount (`:57`, `:159-164`), the standing trap that ADR-0024 is a deliverable and not prior art (`:179-181`), the testkit must-not-change seam (`:102-103`), Testing's "no pass/fail rule" (`:511`), the control at `:539-555`, and phase 6 ownership (`:460-463`) | First, before any drafting — it is the brief this spec distils and the tie-breaker on scope | AC-006 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_storymap.md` | This story's row (`:64`), its longer entry (`:131-140`), the slice-mate boundary (`:65`, `:142-146`) and the merge order (`:251-254`) — the authority on what belongs to HS-S0066 rather than here | Before starting, and again before touching the Postgres crate's docs | AC-008 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/project.md` | DR-3 (`:180-187`) — the two numbers, stated as a delivery requirement — and DR-7 (`:203-207`) — the ADR queue, the named losers, and "a `[FROZEN]` clause changes by ADR, never by edit" | Before writing the record's decision section; when scoping AC-002 | AC-002 |
| `RUNBOOK.md` | Phase 10 in full at `:4311-4390`: the framing at `:4322-4344` (*the one decision that is not a storage preference*), the two scenarios at `:4341-4342`, the exit criterion at `:4377-4378`, and the ADR queue row and numbering rule at `:303` and `:262-268` that make 0024 the right free number | Before allocating the ADR number; when writing the exit-criterion evidence | AC-006 |
| `.kb/reference/position-visibility-experiment-2026-08.md` | The phase-2 measurement as the KB itself holds it — the summary a future reader meets before the raw harness, and the shape this record's KB-side summary must not contradict | While drafting the intake document, so the wave adjudicates a consistent pair | AC-001 |
| `.kb/_intake/README.md` | The mount's operating rules (`:13-19`): what staging means, and that a file still present after a run is a file that run did not ingest | When staging the intake document, and when checking the wave afterwards | AC-008 |
| `.kb/maps/decision-map.md` | The corpus's supersession index and the row this decision must acquire; also the model for how the seventeen existing decisions are indexed | After the ingest wave, to verify the mount actually landed | AC-008 |
| `references/adr/0013-position-assignment-and-visibility.md` | The long-form record whose structure this story's long form should follow — the transcripts, rejected alternatives and measurement tables an atom cannot hold | When drafting `references/adr/0024-<slug>.md` | AC-004 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | Personas 2 (adapter author, `:114`) and 4 (evaluator, `:249`) and their journeys — the audience every AC above is framed from, since no `.kb/product/` persona atom exists yet to cite | When judging whether the record is re-derivable by its actual reader rather than by its author | AC-001 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_design.md` | The signed-off determination that this project renders **no user-facing surface**, with `## Anti-patterns` recorded `N/A` and the sign-off at `:90-95`. It is why *Interaction quality*'s composition family is empty rather than invented | Before writing or reviewing anything that looks like a presentation requirement | AC-008 |

## Clarifications resolved during spec

1. **AC count.** The front half enumerated AC-001 … AC-008 and the back half
   enumerates exactly those eight. None added, none dropped; the ledger matches.
2. **Where the ten *Behavior and interfaces* rows went.** Eight map one-to-one
   onto AC-001 – AC-008. The two that do not are deliberately not ACs: "a
   refutation is a recorded outcome, not a silent re-wire" is **EC-001**, because
   it is a conditional path rather than something every merge must show; and
   "nothing frozen is edited" is split between **AC-007**'s verification
   (`spec-trace` green) and **NF-006** (the boundary), because it is an *absence*,
   and an absence is checked by a gate rather than asserted by a record.
3. **Interaction quality with no surface.** The composition family is written as
   *not applicable by an approved determination* rather than skipped silently,
   because `_design.md:78-80` records `N/A` for anti-patterns and `:90-95` is the
   human sign-off on that determination. Fabricating a density budget or a
   transience policy for a markdown atom would contradict a signed-off design and
   would be the decorative check CLAUDE.md warns against. The state family *does*
   apply — reversibility, non-occlusion, preserved state, in-place answering and
   index reachability all have real meanings in the knowledge base's medium — and
   every one is carried by a numbered AC row, none by prose.
4. **The mount cannot be completed by this story's implementer alone, and that is
   correct.** `/redkiln:kb-ingest` carries `disable-model-invocation`; it is a human
   handoff. The spec therefore treats the atom as this story's mount (AC-008) while
   making the intake document explicitly *not* evidence for it (EC-002), and the PR
   boundary admits the `.kb/decisions/**` and map paths the wave writes, with the
   reason stated inline.
5. **AC-001 has no automated pass/fail rule, and none was invented.** The testing
   brief states the position (`postgres-and-neon-stores/_decomposition.md:511`);
   the verification column names `redkiln validate --kb`, the committed artefacts
   and a human read. A synthetic assertion over a measured number would be a rule
   no adapter can fail.
6. **The slice-mate boundary is one sentence wide, and it stayed one sentence
   wide.** This story edits `crates/happenstance-postgres/src/event_store.rs:77`
   only. Everything else consumer-facing — the frontier `head`, the absent
   read-your-own-writes, the staleness bound in the crate's rustdoc, and the
   doc-prose-versus-clause decision — is `postgres-structural-bill` (project
   AC-009), even though the two are implemented in one context.
7. **The `<slug>` in `0024-<slug>.md` is left unfixed on purpose.** The number is
   allocated and free (`RUNBOOK.md:303`, `:262-268`) but the slug should name the
   mechanism the measurement actually selects; fixing it here would presume the
   outcome and make EC-001's refutation path read as a failure. The implementer
   names it once the number is in hand, and the ledger's `mount_point` carries the
   same placeholder until then.
8. **No conformance rule is added or changed, and neither predecessor run is
   re-executed.** This story cites `crates/happenstance-testkit/src/suite.rs:5880`
   and the `CONTENDERS = 8` family as evidence produced by HS-S0064 and HS-S0063;
   `crates/happenstance-testkit/**` is a must-not-change seam here
   (`postgres-and-neon-stores/_decomposition.md:102-103`).
9. **`redkiln verify --grain story`'s AC extraction.** Every acceptance criterion
   above is a table row whose first cell is a bare `AC-###`, which is one of the two
   shapes the extractor matches. No criterion is stated only as a prose bullet
   elsewhere in this document — in particular, the interaction-quality invariants
   point *back* at AC rows rather than introducing new ids.
