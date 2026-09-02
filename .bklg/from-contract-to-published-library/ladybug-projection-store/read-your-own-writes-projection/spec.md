---
item: HS-S0079
stage: spec
created: 2026-08-12T13:47:17.621Z
updated: 2026-08-12T13:47:17.621Z
template_sig: 87bbf1d0
rendered_sig: a6769d52
---

# Spec — Read-your-own-writes inside one batch, answered

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — Goals, **DoD 7**, **DoD 8**, BR-04ʳ |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — the DAG and the scope seams |
| Project charter | `.bklg/from-contract-to-published-library/ladybug-projection-store/project.md` — **AC-005** (`:197-201`), DR-4, DR-5, DR-9, and the "verdict chosen to match the result" risk (`:269-271`) |
| This spec | `.bklg/from-contract-to-published-library/ladybug-projection-store/read-your-own-writes-projection/spec.md` |
| Key briefs | `.bklg/…/ladybug-projection-store/_decomposition.md` — architecture **M1** (`:127-142`, the invocation is the whole target), **M2** (`:144-153`, declension with a reason), **M4** (`:165-171`, the proof registry), **§8 item 4** (`:421-428`, what is fixed about this case and what is not); testing brief's **AC-005** row (`:475`) |
| Story map row | `.bklg/…/ladybug-projection-store/_storymap.md:45` (slice `conformance-run`); why it is its own story, `:75-78` |
| This story's discover | `.bklg/…/read-your-own-writes-projection/discover.md` — the signal ledger, the four questions it answered, and the **three** wrong implementations |
| Slice-mate (merges first) | `.bklg/…/ladybug-fixture-and-conformance-run/spec.md` — `LadybugFixture`, the conformance target, the `ARTEFACTS` row this case is named in |
| Signed-off design | `.bklg/…/ladybug-projection-store/_design.md` — **no user-facing surface**, approved Ryan Britton 2026-08-12. This story renders no surface and adds no public API item. |
| Roadmap pointer | `RUNBOOK.md:4393-4444` — phase 11; `RUNBOOK.md:4440` names E2E-19 and E2E-24's third-shape halves |

## One-line PR slice

Execute a projection that `MATCH`es a node it created earlier in the same batch,
and record the result as a supported behaviour or a declared capability limit of
the deferred write set — never as a hypothesis.

## Executive summary

By the time this story opens, `happenstance-ladybug` drives the real `lbug`
engine (HS-S0077) and the projection conformance suite has run against a
`LadybugFixture` inside a target registered in `xtask/src/proof.rs`'s `ARTEFACTS`
(HS-S0078, the slice-mate). What the tree does **not** contain is an answer to
the one question the skeleton wrote down and refused to guess at.

The skeleton settled one of PS-4's two falsifying conditions and stated in the
module docs that it could not settle the other: with a deferred write set, a
traversal in statement *n* runs at commit time against a graph that does not yet
hold statement *n−1*'s effect — *"a run-time capability limit, not a compile
error. Phase 11 closes it by writing a projection that needs read-your-own-writes"*
(`crates/happenstance-ladybug/src/projection_store.rs:49-64`).

**The delta this PR lands is the observation.** One projection built from **two
distinct `GraphStatement`s in one `GraphWriteSet`** — the second traversing a
node only the first creates — executed against the real engine inside the
registered conformance target; a **single-statement control** run beside it,
because statement-local visibility is a property of the query engine and not of
the batch, and a case that cannot tell the two apart is evidence for nothing; and
a recorded outcome written in **PS-12's** vocabulary rather than in the fixture's.
That last part is the sharp end: PS-12 does not merely permit an adapter to lack
read-your-own-writes, it *forbids* a batch read path that answers from committed
state (`spec/SPECIFICATION.md:5052-5057`), so one of the three possible results is
not a Ladybug quirk at all — it is a finding about the port surface.

What it does **not** land: the fixture, the suite invocation and the `ARTEFACTS`
row (HS-S0078); the verdict document itself (`freeze-verdict-document`, which
consumes this record); PS-12's or PS-4's marker, which is
`projection-store-freeze`'s (HS-P0010); and the E2E cases this makes writable
(see D10).

## Context pack

The load-bearing decisions, stated as decisions. Everything deeper is a signposted
anchor in the second half of this spec.

**D1 — The case is *two* statements in one batch, and that is not a stylistic
preference.** `GraphWriteSet` holds an ordered `Vec<GraphStatement>` and nothing
executes until `commit` replays it
(`crates/happenstance-ladybug/src/projection_store.rs:105-138`). The behaviour
under test is therefore **cross-statement visibility inside one batch**: statement
1 creates a node, statement 2 `MATCH`es it. Collapsing them into one statement
(`MERGE (a:…) WITH a MATCH (a)-[…]->(c) RETURN c`) tests the *query engine's*
statement-local visibility, executes cleanly, goes green, and answers a question
nobody asked. That is the mutant most likely to be built by accident, because it
is what a Cypher author writes naturally (`discover.md`, *The wrong
implementation*, first heading).

**D2 — The single-statement variant is a control, not a second test, and both are
run and recorded together.** The two variants must **diverge** for the case to be
evidence: the single-statement one is expected to succeed whatever the batch does;
the two-statement one is the observation. If both come out the same way, the case
cannot distinguish statement-local visibility from cross-statement visibility and
proves nothing — recording that is the control's job. The control is run and its
outcome written **before the real result is interpreted**, one level down from
AC-001's commit-order discipline (`project.md:180-184`; `discover.md`, *The wrong
implementation*).

**D3 — A `Capability::declined` string is the wrong instrument for this answer,
and reaching for it is a named failure.** A declension says the **fixture could
not set the rule up**, and a declined rule *does not run*
(`crates/happenstance-testkit/src/contract.rs:219-232`, `:412-419`). AC-005
requires the projection to be **executed**. So "the traversal saw nothing" is an
observation from a rule that ran, and entering it as
`Capability::declined("deferred write set cannot read its own pending writes")`
launders a specification finding into a fixture note that the next reader takes
for a local quirk. If the only way the case can be expressed in the suite's
vocabulary is as a declension, **that is a finding about the suite** and is raised
with HS-P0010 — it does not close this AC (`discover.md`, question 3).

**D4 — Under PS-12 there are three possible results, not two, and the third is a
freeze finding.** PS-12 requires an adapter to *either* make batch reads reflect
that batch's pending writes *or* expose **no read path on the `Batch` at all**,
and forbids one that answers from committed state
(`spec/SPECIFICATION.md:5052-5057`). So:

1. **The traversal sees the pending node** → read-your-own-writes is a *supported
   behaviour*; PS-4's second condition survives and PS-12 is confirmed on this
   adapter.
2. **There is no batch read path to issue the traversal through** → a *stated
   limit*, and it is PS-12's own compliant shape (`declining by omitting the
   method is enforceable at compile time`, `spec/SPECIFICATION.md:5067-5073`).
3. **The traversal runs and answers from committed state** → the adapter as built
   sits in the shape PS-12 **forbids**. The resolution is either no batch read
   path or a changed port, and it goes to the verdict and to HS-P0010 — never into
   a capability string (`discover.md`, question 2).

Recording result 3 as if it were result 2 is the most consequential error
available in this PR.

**D5 — Where the case lives is decided by what HS-P0010 shipped, and one branch
makes the local case *mandatory* rather than a fallback.** PS-12's rule
`batch_reads_reflect_pending_writes` is **gated on
`ProjectionProbe::READS_THROUGH_BATCH`** (`spec/SPECIFICATION.md:5059-5065`), and
a `false` declaration makes the rule a **reported skip** — which does not execute
the projection and therefore cannot discharge AC-005. Hence:

- **If the probe (or HS-P0010's fixture-level analogue) shipped and this adapter
  declares `READS_THROUGH_BATCH = true`** — the suite rule is the primary
  evidence, and the local two-statement case remains as the control's twin.
- **If the adapter must declare `false`** — the suite emits a skip, and the
  two-statement case **must** run as a test alongside the suite invocation, in the
  same target, or nothing executed at all.

Either way the suite is invoked whole and never subsetted (`_decomposition.md:127-142`,
M1; `:475`, the testing brief's AC-005 row).

**D6 — The `READS_THROUGH_BATCH` declaration is downstream of the run, which
inverts the order a declaration is normally written in.** An honest `const`
requires having *observed* what a batch read does; writing the const from the
module docs and then never running the case is exactly the substitution — mechanism
for observation — AC-005 exists to reject (`discover.md`, *The wrong
implementation*, third heading). So: run the two-statement case locally first,
read the result, then set the declaration to what was seen. If the declaration has
to change during this PR, that change **is** part of the recorded finding.

**D7 — The probe impl, if there is one, is the slice-mate's; only the declared
value is this story's.** Rust coherence decides this, not preference: each file
under `tests/` is its own crate, so `impl ProjectionProbe for
LadybugProjectionStore` cannot live there — neither the trait nor the type would
be local. It must live in `crates/happenstance-ladybug/src/`. And PS-11 states
that *"an adapter that does not implement it cannot invoke the suite"*
(`spec/SPECIFICATION.md:4977-4986`), so the impl is a precondition of HS-S0078's
run and lands with it. This story touches `src/` for **one thing only**: the value
of the visibility declaration, once the run has said what it is.

**D8 — Nothing about this answer moves a marker here.** §4.1a's question 1 gates
**PS-4, PS-5, PS-6, PS-12 and PS-34 together** and names *"The Ladybug skeleton"*
as what answers it, *"2, confirmed at 11"* (`spec/SPECIFICATION.md:4809`). Five
clauses turn on one observation — which is the leverage this story has and the
reason it is not folded into the harness story. It **supplies** the answer: no
clause marker moves, no `[FROZEN]` text is amended, `cargo xtask spec-trace` stays
green, and a result implying the freeze did not hold routes through the verdict to
a decision atom and a re-plan (`project.md:109-111`, `:210-213`).

**D9 — A blocking-bridge failure must be distinguishable from "the traversal saw
nothing".** ADR-0025's Q3 answer decides how `lbug`'s synchronous API meets the
`async` port (`_decomposition.md:293-310`), and it is not this story's. But if the
bridge blocks the executor thread, a case that hangs or times out must not be read
as a negative result — the two failure modes look identical in a bare red run.
This is a requirement on **how the outcome is captured**, and it is why the record
carries the case's own emitted output rather than a pass/fail bit.

**D10 — E2E-19 and E2E-24 are made *writable* here; they are not written here.**
`_decomposition.md:429-431` leaves the choice open and `RUNBOOK.md:4440` only
claims they become writable at this phase. Decision: **not written in this PR.**
`spec/E2E-CASES.md` is not in this project's seams table
(`_decomposition.md:86-88`; only `spec/SPECIFICATION.md`'s six citations are, and
those merged with HS-S0077), and E2E-24's third-shape half is a claim about the
whole adapter passing the suite — which is HS-S0078's evidence plus this story's.
What this PR owes them is the pointer, recorded for `freeze-verdict-document`.

**D11 — Never assert on a literal position value.** The specification permits gaps
and a conformant adapter may leave them (`CLAUDE.md`, *The rule that matters*;
`project.md:293-294`). This case asserts on **graph contents** and, where a
position appears at all, on the position the store was **handed** — never on one it
assigned.

**D12 — The claim must cite its run.** The recorded outcome names the target and
the fully qualified test name out of the `ARTEFACTS` row
(`xtask/src/proof.rs:133`), so that a finding with no run behind it has nothing to
cite. That is the mechanical guard against the third wrong implementation — the
answer written from the module docs, which is *probably true*, cites a real source,
reads as a finding, and was never executed (`discover.md`, *The wrong
implementation*, third heading).

**D13 — The domain is free; the structure is not.** Which projection models the
case is deliberately unprescribed (`_decomposition.md:421-428`). What is fixed:
two distinct statements, ordered, the second depending on a node only the first
creates, in one `GraphWriteSet`, replayed by one `commit`. A domain that mirrors
`examples/course-subscriptions/` makes the case read as a *projection* rather than
as a probe, which matters because the reviewer must recognise it as something a
real projection would do.

**Persona-journey slice.** The actor is the reviewer on *Decide in one sitting*
(`initiative.md:241-250`) — the one who will be handed a *held* / *did not hold*
verdict and asked to believe it. What this story hands them is the single line in
that verdict that could not have been written from the design: not "the deferred
write set defers, therefore…", but *this projection, these two statements, this
target, this commit, this is what happened.* The adapter author on *Learn when you
are finished* is served by the same artefact, from the other side: it tells them
what a `Batch` in this shape can and cannot be asked.

## Integration contract

- **Archetype**: `capability` — a slice through the projection, the batch, the real
  engine, the registered target and the gate that would notice its removal.
- **Slice / milestone**: `conformance-run`. Slice-mate:
  `ladybug-fixture-and-conformance-run` (HS-S0078), which merges **first** and
  supplies the fixture, the target and the registry row this case runs inside
  (`_storymap.md:44-45`; merge order `:134-135`). The two are mounted as one
  integrated surface: one test target, one `ARTEFACTS` row, one run record the
  verdict reads.
- **Mount point**: **`xtask/src/proof.rs`** — the `ARTEFACTS` constant at `:133`,
  whose row for this crate's conformance target gains **this case's fully qualified
  test name** in its `tests` list. That row is the composition root: `cargo test`
  exits 0 on `running 0 tests`, so without a named test an emptied or renamed case
  passes the step a deleted one fails (`xtask/src/main.rs:170-178`; the argument is
  `xtask/src/proof.rs:1-24`). The file the row mounts is
  `crates/happenstance-ladybug/tests/<conformance target>.rs` — the case executes
  **inside** the registered target, never as a bespoke test beside it.
- **Wires into** (real sibling contracts consumed, not invented):
  - `crates/happenstance-ladybug/src/projection_store.rs` — `GraphStatement`
    (`:70-103`), `GraphWriteSet` (`:105-138`), and the module doc that names this
    case as the open half (`:49-64`).
  - `crates/happenstance-ladybug/tests/` — `LadybugFixture` and the conformance
    target, both landed by HS-S0078 and **consumed unchanged**.
  - `crates/happenstance-testkit/src/contract.rs` — `Capability` and `RuleOutcome`
    vocabulary, read to establish what a declension *means* (`:219-232`, `:412-419`)
    and therefore why it is not this answer's instrument (D3). **Read, never
    edited** (`_decomposition.md:86-88`).
  - `crates/happenstance-core/src/projection.rs` — the port and PS-1's single
    transaction (`:13-30`). **Read, never edited.**
  - HS-P0010's projection probe seam — either PS-11's `ProjectionProbe` in the
    contract crate behind `feature = "conformance"`
    (`spec/SPECIFICATION.md:4990-5010`) or the fixture-level `write_probe` /
    `read_probe` its charter committed to
    (`../projection-store-freeze/project.md`, In scope item 4). **Read what
    shipped** (`_decomposition.md:104-114`); do not guess between them.
- **Renders surfaces**: **none**. `_design.md`'s `## Surfaces`, `## Items` and
  `## Signatures` are all `N/A — no user-facing surface` (approved 2026-08-12).
  This story adds **no new `pub` item** to `happenstance-ladybug`: the projection
  and its control live under `tests/`, and the only `src/` change contemplated is
  the *value* of an already-declared visibility constant (D6, D7).
- **Conformance rule(s)**: `batch_reads_reflect_pending_writes` — PS-12's rule,
  gated on `READS_THROUGH_BATCH` (`spec/SPECIFICATION.md:5059-5065`) — is the rule
  that observes this behaviour **if HS-P0010 registered it and the declaration is
  `true`**. This story **writes no conformance rule**; the suite and its rules are
  `projection-store-freeze`'s (`_decomposition.md:86-88`). Where the declaration is
  `false` the rule emits a reported skip and the local two-statement case carries
  the observation instead (D5) — that substitution is itself recorded, because a
  skip is not an answer.
- **Clause(s)**: answers **PS-12** (`spec/SPECIFICATION.md:5052-5075`,
  `[PROVISIONAL]`, Ladybug named as its candidate falsifier) and closes the second
  condition of **PS-4** (`:4849-4862`, `[PROVISIONAL]`). Feeds §4.1a question 1
  (`:4809`), which gates PS-4, PS-5, PS-6, PS-12 and PS-34 together. **No marker
  moves and no clause sentence is edited here** (D8); `cargo xtask spec-trace`
  stays green.
- **Advances DoD scenario**: initiative **DoD 8** — *"The freeze verdict is
  written… with what it was checked against"* (`initiative.md:381-383`) — by
  producing the one finding in that verdict that no amount of type-checking could
  reach. Secondarily **DoD 7** (*"the projection suite discriminates"*,
  `initiative.md:377-380`), because this is the behaviour the unlike batch shape
  can plausibly fail; and project **DoD 2** (`project.md:230-232`), jointly with
  HS-S0078.

## PR boundary

`redkiln verify --grain story` reads the first fenced block below and fails on any
file changed outside it.

```
crates/happenstance-ladybug/tests/**
crates/happenstance-ladybug/src/projection_store.rs
xtask/src/proof.rs
.bklg/from-contract-to-published-library/ladybug-projection-store/read-your-own-writes-projection/**
```

**In this PR**

- The read-your-own-writes projection: **two distinct `GraphStatement`s** in one
  `GraphWriteSet`, the second traversing a node only the first creates, executed
  through one `commit` against the real engine, inside the conformance target
  HS-S0078 registered.
- The **single-statement control** beside it, run and recorded together with the
  real case, with the divergence (or its absence) written down.
- The recorded outcome, in PS-12's vocabulary: supported behaviour, stated limit
  with "no batch read path at all" spelled out, or the PS-12-forbidden shape routed
  as a freeze finding (D4).
- `xtask/src/proof.rs` — the existing `ARTEFACTS` row's `tests` list gains this
  case's fully qualified name, read out of `cargo test --test <target> -- --list`
  (mounting this slice; the mount is named in the Integration contract and touching
  it is not scope drift).
- `crates/happenstance-ladybug/src/projection_store.rs` — **narrowly**: the value of
  the batch-read visibility declaration, if the shipped probe seam carries one, set
  to what the run observed (D6). No body, no signature, no new item.
- This story's folder: the run record — the two variants, their outputs, the
  divergence check, the target and test names, the commit — in the form
  `freeze-verdict-document` consumes.

**Explicitly not in this PR**

- `LadybugFixture`, the conformance target itself, the `RowShapedFixture` negative
  control and the creation of the `ARTEFACTS` row → `ladybug-fixture-and-conformance-run`
  (HS-S0078), merged first.
- Any edit under `crates/happenstance-testkit/**` or to
  `crates/happenstance-core/src/projection.rs` — the suite, its rules and the port
  are `projection-store-freeze`'s (`_decomposition.md:86-88`). A rule that seems
  wrong is raised there with its reason in this same change, never `#[cfg]`-ed out,
  retried, or made to pass by weakening the case.
- The probe impl itself, and any `Cargo.toml` feature that gates it → HS-S0078,
  which cannot invoke the suite without it (D7, PS-11).
- The freeze verdict document and anything under `references/evaluation/` →
  `freeze-verdict-document`; the axes document is `preflight-and-unlike-axes`'.
- Writing **E2E-19** and **E2E-24**'s third-shape halves, or any edit to
  `spec/E2E-CASES.md` (D10).
- Moving PS-4's, PS-12's, PS-5's, PS-6's or PS-34's marker, or any `[FROZEN]` text
  (D8) — that is HS-P0010's, via a decision atom and a re-plan.
- Packaging, licences, `PUBLISHABLE`, the cold-build measurement and CI shape →
  `package-completeness-and-name-claim`, `cold-build-cost-and-ci-shape`.

**Merge DoD**: `cargo xtask ci` — the whole gate, not `--fast`
(`_storymap.md:145-147`) — is green with the case executing inside the registered
conformance target under `--show-output`, `cargo xtask proof-artefact` asserting the
case's name out of `--list`, and the run record committed in this story's folder
carrying both variants' outcomes and the interpretation written after them.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The batch under test holds two ordered statements, not one** | The projection pushes statement 1 (creates a node) and statement 2 (`MATCH`es that node) as **separate** `GraphStatement`s into one `GraphWriteSet`, in that order, through the buffering surface the adapter already exposes. Nothing executes until `commit` replays them inside one `BEGIN TRANSACTION` … `COMMIT`. A single statement doing both is a different experiment and does not discharge this story. | `crates/happenstance-ladybug/src/projection_store.rs:70-103` (`GraphStatement`, parameters carried beside the text), `:105-138` (`GraphWriteSet`), `:49-64` (the open half); `discover.md`, *The wrong implementation* |
| **The case is executed against the real engine, inside the registered target** | It runs in `crates/happenstance-ladybug/tests/<conformance target>.rs` — the target HS-S0078 wrote and registered — against a `LadybugFixture` instance driving real `lbug`. Not a bespoke target, not a doc example, not `#[ignore]`d, and never a substitute for the suite being invoked whole. | `_decomposition.md:127-142` (M1, "not a subset"), `:475` (testing brief AC-005); `xtask/src/proof.rs:133`; `.bklg/…/ladybug-fixture-and-conformance-run/spec.md` (the target and row) |
| **A single-statement control runs beside it and the two must diverge** | The control is the same projection collapsed into one statement, expected to succeed whatever the batch does. Both variants run in the same invocation and both outcomes are recorded. **If they agree, the case cannot distinguish statement-local from cross-statement visibility and is not evidence** — that conclusion is the recorded finding, not a reason to retry. The control's outcome is written before the real result is interpreted. | `discover.md`, *The wrong implementation* (first heading and the falsifier paragraph); `project.md:180-184` (the ordering discipline this mirrors); `standards/rust/60-what-a-test-must-prove.md` |
| **The outcome is one of three, and the third is a port finding** | (1) traversal observes the pending node ⇒ **supported behaviour**; (2) no batch read path exists to issue it through ⇒ **stated limit**, PS-12's own compliant shape, recorded with the "no read path at all" consequence spelled out; (3) traversal answers from **committed state** ⇒ the shape PS-12 forbids — a freeze finding routed to the verdict and HS-P0010, never a capability string. | `spec/SPECIFICATION.md:5052-5057` (the clause), `:5067-5073` (why declining by omission is enforceable and declining by stale data is not); `discover.md`, question 2 |
| **A declension does not close this AC** | `Capability::declined` describes what the **fixture could not set up**, and a declined rule does not run. AC-005 requires execution. If the suite's vocabulary can only express this case as a declension, that is a finding raised with HS-P0010 in this same change — not a discharge. | `crates/happenstance-testkit/src/contract.rs:219-232`, `:412-419`; `discover.md`, question 3; `project.md:197-201` |
| **Where the case lives follows what HS-P0010 shipped, and one branch makes the local case mandatory** | PS-12's rule is gated on `READS_THROUGH_BATCH`; a `false` declaration makes it a **reported skip**, which executes nothing. `true` ⇒ the suite rule is primary evidence. `false` ⇒ the two-statement case must run as a test in the same target or nothing ran at all. The branch taken is recorded. | `spec/SPECIFICATION.md:5059-5065` (the gate and CF-18's reported-skip requirement); `_decomposition.md:104-114` (read what shipped), `:475` |
| **The visibility declaration is written after the run, not before it** | An honest `READS_THROUGH_BATCH` (or its fixture-level analogue) requires an observation. Run the two-statement case, read the result, then set the constant. A declaration that changes during this PR is part of the finding, not a tidy-up. | `discover.md`, *The wrong implementation*, third heading; D6 above |
| **No probe impl and no new public item** | The probe impl, if PS-11's shape shipped, lives in `crates/happenstance-ladybug/src/` because coherence forbids it under `tests/` (each `tests/` file is its own crate; neither trait nor type would be local) — and it is HS-S0078's, because the suite cannot be invoked without it. This story's only `src/` edit is a declared constant's value. | `spec/SPECIFICATION.md:4977-4986` (PS-11), `:4990-5010` (the probe sketch and where it lives); `standards/rust/40-public-surface-and-evolution.md` |
| **A hang is not a negative result** | If ADR-0025's Q3 answer blocks the executor thread, a timing-out case must be distinguishable in the captured output from a traversal that ran and saw nothing. The record carries the case's emitted output, not a pass/fail bit. | `_decomposition.md:293-310` (Q3 and its three candidates); `crates/happenstance-ladybug/src/projection_store.rs:39-47`; `discover.md`, question 5 |
| **No literal position value is asserted** | Assertions are on **graph contents** and on positions the store was **handed** by `commit(batch, id, position)`; never on a position the store assigned. The specification permits gaps. | `CLAUDE.md`, *The rule that matters*; `project.md:293-294`; `crates/happenstance-core/src/projection.rs` |
| **The claim cites its run** | The recorded outcome names the target and the fully qualified test name from the `ARTEFACTS` row, plus the commit. A finding written from the module docs has nothing to cite, which is the mechanical guard against it. | `xtask/src/proof.rs:58-70` (qualified naming), `:133`, `:195-217` (`check()` bails on absent names); `project.md:172-174` (DR-9) |
| **Nothing frozen moves and nothing upstream is edited** | No clause marker, no clause sentence, no testkit file, no `projection.rs`. `cargo xtask spec-trace` green. A result implying the freeze did not hold routes through the verdict to a decision atom and a re-plan. | `project.md:109-111`, `:210-213`; `_decomposition.md:86-88`; `spec/SPECIFICATION.md:4809` (§4.1a question 1) |
| **The record is in the form the verdict consumes** | Both variants, their outputs, the divergence check, the branch taken (suite rule vs local case), the declaration's value and whether it changed, the target/test names and the commit — enough for `freeze-verdict-document` to name *which implementation, which rules, which clauses, at which commit* without re-running anything, and enough for a stranger to re-take the snapshot. | `project.md:151-153` (DR-4), `:172-174` (DR-9); `_storymap.md:48` (what the verdict story carries) |

## Data and migrations

**No user data, no schema migration, no persistent store outside the test run.**
Everything this story creates lives inside a single `cargo test` process and is
gone when it exits.

- **Storage lifecycle is inherited, not re-decided.** The temp-directory-per-fixture-instance
  strategy, its teardown, and the Windows-first `PermissionDenied` hazard (drop the
  `Database` before removing the directory) are `LadybugFixture`'s, landed by
  HS-S0078 and consumed here unchanged
  (`crates/happenstance-testkit/src/contract.rs:1-63`;
  `.bklg/…/ladybug-fixture-and-conformance-run/spec.md`, *Data and migrations*).
  This story adds no new storage mechanism and no new dependency of any kind.
- **The graph the case writes is the case's own.** Two statements, one node, one
  traversal — created inside the fixture instance's isolated database and never
  reused across instances or across the two variants. Each variant gets its own
  fixture instance, because sharing one would let the control's write be the thing
  the real case traverses, which would fabricate a positive result.
- **The Cypher schema is not decided here.** Label names, property names and the
  checkpoint node's placement are ADR-0025's Q1 plus whatever
  `fill-the-bodies-and-ps-34-disposition` implemented
  (`_decomposition.md:261-274`, `:408-419`). This story consumes it as given and
  models a domain over it (D13).
- **The checkpoint write is untouched.** It remains the last statement before
  `COMMIT`, inside the same `BEGIN TRANSACTION` as the read-model write — that
  single transaction is PS-1 and the reason the port has this shape
  (`crates/happenstance-core/src/projection.rs:13-30`). The `INT64` ↔ `NonZeroU64`
  narrowing and its `MalformedCheckpoint` / `PositionOutOfRange` variants are not
  simplified, touched or asserted against here
  (`crates/happenstance-ladybug/src/projection_store.rs:175-202`).
- **The only durable artefact is prose.** The run record in this story's folder —
  committed, and read by `freeze-verdict-document`. It does not live under
  `references/evaluation/`, which is `preflight-and-unlike-axes`' (the axes) and
  `freeze-verdict-document`'s (the verdict): two files in two commits, because
  AC-001 is an ordering claim about commits (`_decomposition.md:210-218`, M9).

## Acceptance criteria

Eight criteria. Each is written from the intent of a named actor on a named journey
— the **reviewer** on *Decide in one sitting* who will be handed a *held* / *did not
hold* verdict, the **adapter author** on *Learn when you are finished* who needs to
know what a `Batch` in this shape can be asked, and the **suite maintainer** at
HS-P0010 who has to trust that a green run here was not bought by editing the thing
under test (`initiative.md:241-250`). Every one crosses the full stack: projection →
`GraphWriteSet` → `commit` → real `lbug` → registered target → gate.

The whole table discharges project **AC-005** (`project.md:197-201`); AC-001 through
AC-003 are its two halves — *executed* and *recorded as an answer, not a hypothesis*
— and AC-004 through AC-008 are what keep those two halves honest.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** the reviewer who has been told the deferred write set is the one axis a projection can plausibly fail against, and who will not accept "the mechanism implies…" as an observation, **WHEN** they run `cargo xtask ci` on this branch and read the `tests` step's captured output for the Ladybug conformance target, **THEN** a projection built from **two distinct, ordered `GraphStatement`s in one `GraphWriteSet`** — the second `MATCH`ing a node only the first creates — has been **executed** through a single `commit` against the real `lbug` engine inside the target HS-S0078 registered, and its result is in the scroll they are already reading. A single statement doing both is not this case and does not discharge it. | The two-statement case in `crates/happenstance-ladybug/tests/<conformance target>.rs` (the target and `LadybugFixture` are HS-S0078's, consumed unchanged), run by `cargo xtask ci`'s `tests` step with `--show-output` (`xtask/src/main.rs:131-151`). Shape asserted against `crates/happenstance-ladybug/src/projection_store.rs:70-103` (`GraphStatement`) and `:105-138` (`GraphWriteSet`); the case's statement count is visible in its own source, and the emitted output is pasted verbatim into the run record in this story's folder. |
| AC-002 | **GIVEN** the same reviewer, who knows the natural Cypher spelling collapses both halves into one statement and would then be reading a fact about the *query engine* rather than about the batch, **WHEN** they read the record, **THEN** a **single-statement control** has run in the same invocation, its outcome is written down **before** the two-statement result is interpreted, and the two are compared: if they **diverge**, the case discriminates statement-local from cross-statement visibility and AC-003's answer stands on it; if they **agree**, the recorded finding is *"this case cannot distinguish the two mechanisms and is not evidence"* — never a reason to retry, reword or delete the control. | The control case beside the observation in the same target, each on **its own** `LadybugFixture` instance (sharing one would let the control's write be the thing the observation traverses); both run by the same `cargo xtask ci` `tests` step. Divergence check written into the run record ahead of the interpretation, mirroring `project.md:180-184`'s commit-order discipline and `standards/rust/60-what-a-test-must-prove.md`'s named-wrong-implementation bar; the mutant is named at `discover.md`, *The wrong implementation*, first heading. |
| AC-003 | **GIVEN** the reviewer opening the freeze verdict expecting the one line that could not have been written from the design, **WHEN** they read this story's recorded outcome, **THEN** it says exactly **one** of PS-12's three shapes and says which: (1) the traversal **observed the pending node** ⇒ *supported behaviour*, PS-4's second condition survives; (2) the `Batch` **exposes no read path at all** ⇒ *stated limit*, PS-12's own compliant shape, with the "no read path at all" consequence spelled out rather than implied; (3) the traversal ran and **answered from committed state** ⇒ the shape PS-12 **forbids**, recorded as a finding about the port surface and routed to the verdict and to HS-P0010. No hedge, no two shapes at once, and no sentence that would read the same had the case never run. | The recorded outcome in this story's folder, checked against `spec/SPECIFICATION.md:5052-5057` (the clause) and `:5067-5073` (why declining by omission is enforceable and declining by stale data is not); review-tier, because "which of three" is a reading and not an assertion. Mechanically anchored by AC-007: the outcome cites the target, the fully qualified test names and the commit, so shape (3) recorded as shape (2) is visible in the diff between the emitted output and the prose. |
| AC-004 | **GIVEN** the adapter author on *Learn when you are finished*, who will read this record next year to learn what a Ladybug batch can be asked, **WHEN** they look for the answer, **THEN** it is **not** hiding inside a `Capability::declined("…")` string: a declension states what the **fixture could not set up** and a declined rule **does not run**, so it cannot express a store behaviour and cannot discharge an AC that requires execution. If the suite's vocabulary can only express this case as a declension, that fact is written down as a **finding about the suite**, raised with `projection-store-freeze` in this same change with its reason — and AC-005's branch (below) carries the observation instead. | `crates/happenstance-testkit/src/contract.rs:219-232` and `:412-419` read to establish what a declension means and that a declined rule does not execute; `rg -n "Capability::declined" crates/happenstance-ladybug/tests/` reviewed so that no declension string carries this story's answer. The raised finding, if any, is prose in the run record naming HS-P0010 — `crates/happenstance-testkit/**` is not edited (`_decomposition.md:86-88`). |
| AC-005 | **GIVEN** the suite maintainer at HS-P0010, who registered `batch_reads_reflect_pending_writes` behind `ProjectionProbe::READS_THROUGH_BATCH` and needs to know whether this adapter's declaration is an observation or a guess, **WHEN** they read the record, **THEN** the branch taken is stated: with the declaration **`true`**, the suite rule is the primary evidence and the two-statement case stands beside it as the control's twin; with the declaration **`false`**, the suite emits a **reported skip** — which executes nothing and therefore discharges nothing — and the two-statement case runs as a test in the **same** target or nothing ran at all. Either way the suite is invoked **whole**, never subsetted, and the declaration's value was written **after** the run said what it is, not copied from the module docs. | `spec/SPECIFICATION.md:5059-5065` (the gate and CF-18's reported-skip requirement); what HS-P0010 actually shipped read out of `.bklg/from-contract-to-published-library/projection-store-freeze/projection-probe-conformance-feature/spec.md` rather than guessed (`_decomposition.md:104-114`). Suite invoked whole per `_decomposition.md:127-142` (M1) and `:475`. If the declaration changes in this PR, the before/after value and the run that caused it are both in the record (D6); the only `src/` edit permitted is that constant's value, in `crates/happenstance-ladybug/src/projection_store.rs`. |
| AC-006 | **GIVEN** the gate reader already burned once by a step that a *deletion* fails and an *emptying* passes, **WHEN** either variant is deleted, emptied, renamed or `#[ignore]`d, **THEN** `cargo xtask ci` fails **before** the run with a message naming the missing test — never exiting 0 on `running 0 tests` — because the **existing** `ARTEFACTS` row for this crate's conformance target now carries **both** variants' fully qualified names, copied out of `cargo test --test <target> -- --list` rather than guessed. | `xtask/src/proof.rs` — the `ARTEFACTS` constant at `:133`; the row this story extends is HS-S0078's, so the constant stays at **4** rows and gains **≥ 2** names in one row's `tests` list. Asserted by `cargo xtask proof-artefact` (`check()` at `:195-217`, the qualification requirement at `:58-70`), which the gate runs as its own step (`xtask/src/main.rs:170-178`). Negative control performed by hand: one variant `#[ignore]`d, the gate re-run, the failure output recorded. |
| AC-007 | **GIVEN** a stranger who did not write this adapter and is asked to re-take the snapshot — DR-9's bar, because *"passed against N adapters"* is a snapshot and a snapshot has to be re-takeable — **WHEN** they read the record alone, **THEN** it names the crate, the target, both fully qualified test names, the exact command, the toolchain and the commit SHA, and carries **both variants' emitted output verbatim** rather than a pass/fail bit — so that a case which **hung or timed out** on the blocking bridge is visibly a different event from a traversal that ran and saw nothing, and so that a claim written from the module docs would have nothing to cite. | The run record in this story's folder, cross-checked against the `ARTEFACTS` row's names (`xtask/src/proof.rs:133`, qualification at `:58-70`) and against `cargo xtask ci`'s captured `tests` output. Blocking-bridge confound per `_decomposition.md:293-310` (ADR-0025 Q3) and `crates/happenstance-ladybug/src/projection_store.rs:39-47`; evidence discipline per `project.md:151-153` (DR-4), `:172-174` (DR-9) and `references/evaluation/README.md`. |
| AC-008 | **GIVEN** the maintainers of the port, the suite and the specification, none of whom accepted a change in this PR, **WHEN** they read the whole diff, **THEN** nothing under `crates/happenstance-testkit/**`, `crates/happenstance-core/src/projection.rs`, `spec/SPECIFICATION.md` or `spec/E2E-CASES.md` was touched; no clause marker moved and `cargo xtask spec-trace` is green; `happenstance-ladybug` gains **no new `pub` item**; no rule was `#[cfg]`-ed out, `#[ignore]`d, retried until green or made to pass by weakening the fixture or the case; and no assertion anywhere names a **literal position value** — the case asserts on graph contents and, where a position appears, on the position the store was *handed*. | `git diff --stat` showing no path under `crates/happenstance-testkit/`, `crates/happenstance-core/`, `spec/`, `.kb/` or `references/`; `cargo xtask ci` (whole gate, not `--fast`) green including `spec-trace`, `clippy -D warnings`, `cargo hack` and `cargo deny`; `rg -n "#\[cfg|#\[ignore\]" crates/happenstance-ladybug/tests/` returning no case-level gate; `cargo public-api`-free review of the diff for new `pub` items per `standards/rust/40-public-surface-and-evolution.md`; position discipline per `CLAUDE.md`, *The rule that matters*, and `project.md:293-294`. |

## Interaction quality

**This story renders no user-facing surface, and that is signed off, not assumed.**
`_design.md`'s `## Surfaces`, `## Items`, `## Signatures` and `## Anti-patterns` are
all `N/A — no user-facing surface`, approved by Ryan Britton on 2026-08-12 with the
`design.capture` perceptual review recorded as a **declared** skip. There is no
screen, no control and no rendered markup here, so the composition family below is
not waived on a technicality — it is **re-seated on the only artefact this story
composes for a human**: the evidence the reviewer meets, which is the `cargo xtask
ci` scroll plus the committed run record. That artefact has exactly the failure mode
the composition family exists to catch: a run that is *technically present* and
*unreadable as evidence* passes every data-shaped assertion and tells the reviewer
nothing.

Every invariant below is carried by an **AC-### row in the table above**. Nothing in
this section is a free-standing obligation; `redkiln verify` extracts ACs from that
table, and a bullet here would be ungated and untested.

**STATE invariants**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not a context-jump** — the answer appears in the gate scroll the reviewer is already reading. No second tool, no second CI job, no machine they do not have, no "run this by hand to see the finding". | **AC-001**, **AC-006** | The case executes inside `cargo xtask ci`'s own `tests` step (`xtask/src/main.rs:143-151`), in the target already registered in `ARTEFACTS`; the record is a committed file in this story's folder, not console-only output. |
| **Non-occlusion** — the control does not hide the observation and the observation does not hide the control. Both variants' outputs are present and adjacent; neither is summarised away into "the case passed". | **AC-002**, **AC-007** | `--show-output` is passed by the gate precisely so a passing case's lines are not written into a void (`xtask/src/main.rs:131-151`); both outputs pasted verbatim into the record. |
| **Preserved selection** — adding this case does not move what the reviewer was already looking at: the suite is still invoked **whole**, no rule is subsetted, excluded or reordered, and HS-S0078's target keeps its body and its row identity. | **AC-005**, **AC-008** | `_decomposition.md:127-142` (M1, "not a subset"); `git diff` on the target showing extension rather than replacement; `ARTEFACTS` stays at 4 rows. |
| **Reversibility** — no result reached here is absorbed silently. A finding that implies the freeze did not hold routes **out** of this PR to the verdict, a decision atom and a re-plan; nothing `[FROZEN]` is edited so that nothing has to be reverted. | **AC-003**, **AC-008** | `project.md:109-111`, `:210-213`; `cargo xtask spec-trace` green; the routed finding named in the record. |
| **Reachability by one command** — the whole of this story's evidence is reachable by `cargo xtask ci` alone. A finding that requires a bespoke incantation to reproduce is a finding the next reader will not reproduce. | **AC-006**, **AC-007** | The `proof-artefact` step (`xtask/src/main.rs:170-178`) plus the `tests` step; the exact command recorded in the run record (DR-9, `project.md:172-174`). |

**COMPOSITION invariants** (re-seated on the evidence artefact, per the framing above)

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — the outcome is *composed prose in PS-12's vocabulary*, not raw output left to speak for itself and not a bare boolean. A pasted test log with no sentence naming which of the three shapes occurred is the "bare markup" failure in this medium. | **AC-003**, **AC-007** | Review of the record against `spec/SPECIFICATION.md:5052-5057`; DR-4's bar — *"a verdict that says only 'held' is not a verdict"* (`project.md:151-153`). |
| **Composition and placement** — the record sits in **this story's folder**, where `freeze-verdict-document` consumes it; it does **not** land under `references/evaluation/`, which is the axes' and the verdict's (two files, two commits, because AC-001 is an ordering claim about commits). | **AC-007**, **AC-008** | `_decomposition.md:210-218` (M9); `git diff --stat` showing no path under `references/`. |
| **Transience — persistent chrome vs revealed on demand** — the two `ARTEFACTS` names are **persistent chrome**: they run on every gate invocation forever, and their absence fails the gate. The interpretation is **revealed on demand**, in the committed record. Neither is a one-off console line that exists only in the run that produced it. | **AC-006**, **AC-007** | `xtask/src/proof.rs:133`, `:195-217`; the committed record. |
| **Density budget, with its real numbers** — **exactly 2** variants, run in the **same** invocation, on **2** fixture instances (never 1). The observation: **exactly 1** `GraphWriteSet`, **≥ 2** ordered `GraphStatement`s, the second naming a node only the first creates. The control: **exactly 1** `GraphStatement`. `ARTEFACTS` stays at **4** rows and one row's `tests` list grows by **≥ 2**. The recorded outcome is **exactly 1** of PS-12's 3 shapes. **0** clause markers moved, **0** `[FROZEN]` sentences edited, **0** new `pub` items, **0** files touched under `crates/happenstance-testkit/`. | **AC-001**, **AC-002**, **AC-006**, **AC-008** | Counted in the diff and in `cargo test --test <target> -- --list`; `xtask/src/proof.rs:133` (4 rows after HS-S0078); `git diff --stat`. |
| **Hierarchy** — the record leads with **what was run** and **what happened**, and only then with **what it means**; the control's outcome is written before the observation is interpreted. The mechanism (`projection_store.rs:49-64`) appears as context, never as the headline, because the headline written from the mechanism is the third named wrong implementation. | **AC-002**, **AC-003** | Ordering visible in the record; `discover.md`, *The wrong implementation*, third heading. |
| **Named anti-patterns** — `_design.md` names none (no surface), so this story adopts `discover.md`'s three as the binding set: the one-statement case that answers a question nobody asked; the result laundered into `Capability::declined`; the answer written from the module docs and never executed. Each has an AC that rejects it. | **AC-002** (first), **AC-004** (second), **AC-001** + **AC-007** (third) | `discover.md`, *The wrong implementation*; `standards/rust/60-what-a-test-must-prove.md` — a case no plausible wrong implementation fails is decorative. |

## Error conditions

| id | Condition | Required handling |
| --- | --- | --- |
| **EC-001** | Both variants come out the same way — the control and the observation agree. | The case cannot distinguish statement-local from cross-statement visibility. Record **that** as the finding and stop; do not retry, do not reword the control, do not delete it and do not report the observation's result as if the control had diverged (AC-002). |
| **EC-002** | The case hangs or times out. | Not a negative result. ADR-0025's Q3 bridge may be blocking the executor thread (`_decomposition.md:293-310`); capture the emitted output and the mode of failure and record it as *inconclusive, blocked on the bridge*, distinct from "the traversal saw nothing" (AC-007). Raising it with HS-S0077's decision is legitimate; inferring a PS-12 shape from it is not. |
| **EC-003** | The merged `Batch` exposes **no read path at all**, so the two-statement traversal cannot even be written against it. | This is PS-12's **compliant** declining shape (`spec/SPECIFICATION.md:5067-5073`) and is result 2, not a blocked story. Record it with the consequence spelled out — a projection that calls a batch read simply does not compile — and cite the compile error or the absent method as the observation. AC-001's "executed" is then discharged by the single-statement case plus the recorded compile-level observation, and the substitution is stated (AC-003, AC-005). |
| **EC-004** | The traversal runs and answers from **committed** state. | The adapter as built sits in the shape PS-12 forbids. Record it as a **freeze finding** about the port surface, route it to `freeze-verdict-document` and `projection-store-freeze`, and do **not** enter it as a capability string or as "a Ladybug limit" (AC-003, AC-004). Do not fix the port here. |
| **EC-005** | `READS_THROUGH_BATCH` (or HS-P0010's fixture-level analogue) is `false`, so `batch_reads_reflect_pending_writes` emits a reported skip. | A skip executes nothing and discharges nothing. The two-statement case must then run as a test in the same target, and the record must state that the suite reported a skip and the local case carried the observation (AC-005). |
| **EC-006** | The commit fails with `WriteTransactionInUse` (LadybugDB permits many readers and exactly one writer, `crates/happenstance-ladybug/src/projection_store.rs:204-213`). | Give each variant its **own** fixture instance and do not run them concurrently. If the error survives serial execution, it is a finding for the verdict — *never* "retry until green" (`project.md`, *Single writer, many readers*). |
| **EC-007** | `cargo xtask proof-artefact` bails naming a test absent from `--list`. | Read the names out of `cargo test --test <target> -- --list` and fix the `ARTEFACTS` entry; never relax the assertion, and never leave the row's list unchanged on the theory that HS-S0078's names already cover it (AC-006). |
| **EC-008** | The visibility declaration's value has to change during this PR because the run contradicted it. | That change **is** the finding. Record the before value, the after value and the run that caused it; do not fold it into a tidy-up commit and do not write the constant first (AC-005, D6). |
| **EC-009** | The case will not compile because the merged `Batch` shape, the probe seam or `lbug`'s real value type differ from what this spec assumed. | The mismatch is the finding, recorded for the verdict. Do not weaken `port_shape.rs`, the fixture or the case to make it compile (`_decomposition.md:445-446`), and do not edit the testkit or the port (AC-008). |
| **EC-010** | The projection's Cypher is rejected by the real engine (label, property or parameter shape). | A schema mismatch is HS-S0077/HS-S0076 territory consumed as given (`_decomposition.md:261-274`, `:408-419`); fix the case's Cypher to the schema that shipped. It is **not** a read-your-own-writes result and must not be recorded as one (AC-003). |

## Non-functional

| id | Requirement | Why, and where it is checked |
| --- | --- | --- |
| **NF-001** | **No new `pub` item** on `happenstance-ladybug`. Both variants live under `tests/`; the only `src/` edit is a declared constant's *value*. | `_design.md` signs off a no-surface project; a case or helper promoted into `src/` as `pub` is a semver promise nobody made (`standards/rust/40-public-surface-and-evolution.md`). Diff review (AC-008). |
| **NF-002** | **Zero warnings.** Each file under `tests/` is its own crate; anything shared with the control is organised so neither includer has dead items — not silenced with a blanket `#![allow(dead_code)]`. | `cargo clippy --workspace --all-targets --all-features -D warnings` (`xtask/src/main.rs:116-129`). |
| **NF-003** | **No new dependency**, dev or otherwise. This story consumes the fixture, the driver and the target that already landed. | `standards/rust/50-dependency-hygiene.md`; `cargo deny` and `cargo hack` feature-powerset inside `cargo xtask ci`. |
| **NF-004** | **Determinism and isolation.** Each variant gets its own `LadybugFixture` instance and its own temp directory; neither depends on the other's ordering, and repeated runs give the same answer. | `crates/happenstance-testkit/src/contract.rs:1-63` — one fixture instance is one isolated backing store; sharing one would fabricate a positive result (AC-002). |
| **NF-005** | **No `#[ignore]`, no `#[cfg]` gate, no `should_panic` standing in for an observation.** A case that is present but not running is the exact hole `ARTEFACTS` exists to close. | `rg -n "#\[cfg\|#\[ignore\]" crates/happenstance-ladybug/tests/`; `cargo xtask proof-artefact` (`xtask/src/proof.rs:195-217`) (AC-006, AC-008). |
| **NF-006** | **Bounded gate cost.** The two variants add two short runs to a target that already exists; they must not add a second cold `lbug` build. The crate's build cost itself is `cold-build-cost-and-ci-shape`'s number, not this story's. | `xtask/src/proof.rs`'s `cargo_args` note on feature-set matching (`:150-170` region) — a step differing only in features refingerprints and rebuilds; `_decomposition.md:380-395`. |
| **NF-007** | **The record is legible to a stranger** — crate, target, both qualified test names, command, toolchain, commit SHA, both outputs verbatim, the divergence check, the branch taken, the declaration's value. Immutable, dated, pinned to a commit, superseded rather than edited. | `references/evaluation/README.md`'s evidence discipline; DR-9 (`project.md:172-174`) (AC-007). |

## Implementation notes (non-prescriptive)

Not instructions — the traps this story's shape has, written down so they are met on
purpose rather than by surprise.

- **Read what HS-P0010 actually shipped before writing anything.** The probe seam has
  two candidate shapes — PS-11's `ProjectionProbe` behind `feature = "conformance"`
  (`spec/SPECIFICATION.md:4990-5010`) and the fixture-level `write_probe` / `read_probe`
  the freeze project's charter committed to. Which one exists decides whether the
  suite rule or the local case is the primary evidence (AC-005). Guessing between them
  is how this story ends up asserting against an API that is not there.
- **Run the case before writing the constant, and before writing a word of the
  record.** The order is: run → read the output → decide which of PS-12's three shapes
  → set the declaration → write the record. Every inversion of that order is one of
  the three named wrong implementations.
- **Write the control first and commit its outcome first.** It is the same discipline
  AC-001 of the project applies to the unlike axes: the thing that decides whether the
  result means anything cannot be dated after the result.
- **Pick a domain a reviewer will recognise as a projection.** `examples/course-subscriptions/`
  is the workspace's canonical DCB worked example; a students/courses/enrolments shape
  makes the case read as something a real projection does rather than as a probe (D13).
  The structure is fixed, the domain is not.
- **Get the qualified names from the tool, not from memory.** `cargo test -p
  happenstance-ladybug --test <target> -- --list` prints the module prefix the
  `ARTEFACTS` entry must carry (`xtask/src/proof.rs:58-70`); the emitter arm HS-S0078
  chose is what fixes that prefix (`crates/happenstance-testkit/src/lib.rs:311-357`).
- **Do the negative control on the registry too.** `#[ignore]` one variant, re-run
  `cargo xtask ci`, confirm it fails naming the absent test, then remove the attribute
  and record the failure output. A row whose list was never falsified is a row that
  might be vacuous.
- **If a rule or the suite's vocabulary looks wrong, raise it in this same change with
  its reason.** Never `#[cfg]` it out, never retry it, never weaken the case
  (`CLAUDE.md`, *The rule that matters*; `_decomposition.md:86-88`).
- **Keep the two E2E cases out.** `spec/E2E-CASES.md` is not in this project's seams
  table; this PR hands `freeze-verdict-document` the pointer that E2E-19 and E2E-24's
  third-shape halves are now writable, and writes neither (D10).

## Tests and CI (merge gate)

Grounded in the project testing brief's AC-005 row (`_decomposition.md:475`) and its
tier summary (`:500-516`): this project's primary tier is **integration**, and the
suite is the oracle it does not write.

| tier | command / path | proves |
| --- | --- | --- |
| **Integration (primary)** | `crates/happenstance-ladybug/tests/<conformance target>.rs` — the two-statement case, run inside HS-S0078's registered target by `cargo xtask ci`'s `tests` step with `--show-output` (`xtask/src/main.rs:131-151`) | **AC-001** — the projection is executed against real `lbug`, two ordered statements in one `GraphWriteSet`, one `commit` |
| **Integration (control)** | The single-statement variant in the same target, on its own fixture instance, same invocation | **AC-002** — the case can discriminate statement-local from cross-statement visibility, or it is recorded as unable to |
| **Integration (suite, whole)** | `projection_store_conformance!(LadybugFixture::new())` invoked whole and never subsetted (`_decomposition.md:127-142`, M1) — `batch_reads_reflect_pending_writes` reports pass, or a reported skip under a `false` declaration (`spec/SPECIFICATION.md:5059-5065`) | **AC-005** — the branch taken is stated; a reported skip is visibly not an answer |
| **Gate registry** | `cargo xtask proof-artefact` — its own gate step (`xtask/src/main.rs:170-178`), asserting the extended `ARTEFACTS` row's names out of `--list` (`xtask/src/proof.rs:195-217`); plus the by-hand `#[ignore]` negative control | **AC-006** — an emptied, renamed or ignored variant fails the gate; `running 0 tests` cannot pass |
| **Static (boundary)** | `git diff --stat`; `rg -n "#\[cfg\|#\[ignore\]\|Capability::declined" crates/happenstance-ladybug/tests/`; `cargo xtask spec-trace`; `cargo clippy --workspace --all-targets --all-features -D warnings` | **AC-004**, **AC-008** — no testkit/core/spec path touched, no clause marker moved, no case gated out, no answer laundered into a declension, no new `pub` item |
| **Process / review** | The run record in this story's folder, read against `spec/SPECIFICATION.md:5052-5075` and DR-4/DR-9 (`project.md:151-153`, `:172-174`) | **AC-003**, **AC-007** — exactly one of three shapes, named; the claim cites its run and is re-takeable by a stranger |
| **Whole gate (merge bar)** | `cargo xtask ci` — **not** `--fast` (`_storymap.md:145-147`; `_decomposition.md:517-525`) | The merge DoD: fmt, clippy, tests, four wasm32 steps, docs, `spec-trace`, `--no-default-features` docs, `cargo package --list`, `cargo hack`, `cargo deny`, `proof-artefact` |

`cargo xtask affected --base main` is the story-grain command `.redkiln/config.yaml`
wires to `verify`; it is a narrowing of the above, not a substitute for the whole gate
at merge.

## Risks and coupling (PR-scoped)

- **The verdict chosen to match the result** — the project's single highest-integrity
  risk (`project.md:269-271`), and this story is where it bites hardest, because this
  story *produces the result*. Mitigation is structural: the control is committed and
  interpreted first (AC-002), the outcome must be one of three named shapes (AC-003),
  and the claim must cite a run that exists (AC-007).
- **The answer that is probably right and never executed.** The mechanism is written
  down in the module docs and is persuasive. Every reviewer who reads
  `projection_store.rs:49-64` will believe result 2 before the case runs. That belief
  is the thing AC-001 and AC-007 exist to make insufficient.
- **Result 3 mistaken for result 2.** Both feel like "no read-your-own-writes". One is
  a stated limit; the other is a violation of a `[PROVISIONAL]` clause that names this
  adapter as its candidate falsifier. Getting this wrong quietly retires a finding
  HS-P0010 needs (AC-003, EC-004).
- **Coupling to HS-S0078 is total and one-directional.** No fixture, no target, no
  `ARTEFACTS` row, no run. If the slice-mate's target name, emitter arm or fixture
  constructor differs from what this spec assumed, this story adapts — it does not
  re-decide them (`_decomposition.md:86-88`; the slice-mate's own clarification that
  the target file name is deliberately unfixed).
- **Coupling to HS-P0010's probe seam is a read, not a contract this story can move.**
  If neither `ProjectionProbe` nor a fixture-level analogue shipped, AC-005's `true`
  branch is unavailable and the local case is the whole evidence — which is legitimate
  and must be *stated*, not silently taken.
- **ADR-0025 Q3's bridge is a confound, not a dependency.** A blocking bridge can make
  this case time out and look like a negative result. EC-002 is the guard; the fix is
  HS-S0076's, not this PR's.
- **Single-writer contention is routine here, not exceptional.** Two fixtures, two
  commits, one engine per instance — but a shared instance or a concurrent run turns
  `WriteTransactionInUse` into noise that a tired implementer resolves with a retry
  loop. EC-006 forbids that.
- **Scope creep toward the verdict.** This story produces a *record*; the verdict is
  `freeze-verdict-document`'s and the axes are `preflight-and-unlike-axes`'. Writing
  even a paragraph of the verdict here collapses two commits into one and breaks the
  ordering claim M9 rests on (`_decomposition.md:210-218`).

## Dependencies

**Blocks on**

- **`ladybug-fixture-and-conformance-run` (HS-S0078)** — the only edge, and it is
  hard. It supplies `LadybugFixture`, the conformance target this case executes
  inside, and the `ARTEFACTS` row this story extends. Same slice (`conformance-run`),
  merged **first** (`_storymap.md:44-45`, merge order `:134-135`); the two are mounted
  as one integrated surface — one target, one row, one run record.

Transitively (already merged by the time this opens, not re-verified here):
`real-lbug-driver-swap` and `fill-the-bodies-and-ps-34-disposition` (a real driver and
no `todo!()` bodies), `adr-0025-three-answers` (the schema and the bridge),
`preflight-and-unlike-axes` (the dated axes this case is the load-bearing test of).

**Unlocks**

- **`freeze-verdict-document`** — which consumes this story's record directly and
  cannot name *which implementation, which rules, which clauses, at which commit*
  without it (`_storymap.md:48`).
- **PS-12's disposition and §4.1a question 1** at HS-P0010 — supplied here, decided
  there; question 1 gates PS-4, PS-5, PS-6, PS-12 and PS-34 together
  (`spec/SPECIFICATION.md:4809`).
- **E2E-19 and E2E-24's third-shape halves** become *writable*; they are not written
  here (D10).

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. Every path verified present in this
worktree.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `crates/happenstance-ladybug/src/projection_store.rs` | `:49-64` is the module doc that names this exact case as the half the skeleton could not settle — the sentence this story converts into an observation. `:70-103` is `GraphStatement` (text plus parameters), `:105-138` is `GraphWriteSet` and its replay-at-commit contract, `:204-213` is the single-writer rule behind EC-006, `:175-202` is the checkpoint narrowing this story must not touch. | **First**, before writing either variant — the statement/parameter shape decides how the two statements are pushed. Again at EC-006. | AC-001 |
| `spec/SPECIFICATION.md` | `:5052-5057` is PS-12's normative sentence — the three-way vocabulary the outcome must be written in; `:5067-5073` explains why declining by omission is enforceable and declining by stale data is not; `:5059-5065` is the rule and CF-18's reported-skip gate; `:4849-4862` is PS-4; `:4809` is §4.1a question 1, which gates five clauses on this one observation. | **Before writing the recorded outcome** — and again before deciding whether a result is shape 2 or shape 3. | AC-003, AC-005 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/read-your-own-writes-projection/discover.md` | The three named wrong implementations in full — the one-statement collapse, the laundered `Capability::declined`, and the answer written from the module docs — each with the falsifier that rejects it. This spec distils them; the reasoning is here. | **Before writing the control**, and again before writing the record's first sentence. | AC-002, AC-004 |
| `crates/happenstance-testkit/src/contract.rs` | `:1-63` is the fixture contract (one instance is one isolated backing store) that NF-004 rests on; `:219-232` and `:412-419` are `Capability` / `RuleOutcome` — what a declension *means* and the fact that a declined rule does not run. **Read, never edited.** | When tempted to express the answer as a declension, and when giving each variant its own fixture instance. | AC-004, AC-002 |
| `xtask/src/proof.rs` | `:9-24` argues why naming a target is not enough; `:58-70` is the fully-qualified-name requirement; `:133` is `ARTEFACTS` (four rows after HS-S0078); `:195-217` is `check()`, which bails naming absent tests. | **Before extending the row**, and again the first time the check fails. | AC-006 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/ladybug-fixture-and-conformance-run/spec.md` | The slice-mate this story mounts inside: the fixture's shape, the target's body, the emitter arm that fixes the module prefix, the `RowShapedFixture` negative control, and its deliberate decision to leave the target's file name unfixed. Consumed unchanged. | **First**, alongside `projection_store.rs` — it tells you what already exists so nothing is rebuilt. | AC-001, AC-006 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/projection-probe-conformance-feature/spec.md` | HS-P0010's probe story — whether `ProjectionProbe` behind `feature = "conformance"` shipped or a fixture-level `write_probe` / `read_probe` did. AC-005's branch is decided by which one is on disk, and this is where to read it rather than guess. | **Before touching the visibility declaration**, and before deciding whether the suite rule or the local case is primary. | AC-005 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/_decomposition.md` | M1 (`:127-142`, the invocation is the whole target, never a subset), M2 (`:144-153`, declension with a reason), M4 (`:165-171`, the proof registry), M9 (`:210-218`, two evidence files in two commits), ADR-0025 Q3 (`:293-310`, the blocking bridge and EC-002), §8 item 4 (`:421-428`, what is fixed about this case and what is free), the testing brief's AC-005 row (`:475`) and merge-gate commands (`:517-535`). | Whenever a scope question arises — especially "may I subset the suite?" and "where does the record live?". | AC-005, AC-007 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/project.md` | AC-005 verbatim (`:197-201`), DR-4 (`:151-153`, a verdict that says only "held" is not a verdict), DR-5 (`:159-165`), DR-9 (`:172-174`, the re-takeable snapshot), the "verdict chosen to match the result" risk (`:269-271`), and the single-writer risk. | **Before writing the record** — DR-4 and DR-9 are the record's acceptance bar. | AC-003, AC-007 |
| `standards/rust/60-what-a-test-must-prove.md` | The house bar this story is the sharpest instance of: a case that no plausible wrong implementation fails is decorative, and the wrong implementation must be named. The control exists because of this atom. | **Before writing the control**, and when justifying why both variants are needed. | AC-002 |
| `standards/rust/24-the-blocking-bridge.md` | How a synchronous driver is allowed to meet a non-blocking port in this workspace — the vocabulary for describing an EC-002 hang without re-deciding ADR-0025 Q3. | Only if the case hangs or times out. | AC-007 |
| `standards/rust/40-public-surface-and-evolution.md` | What counts as a new public item and why a test helper promoted into `src/` is a semver promise nobody made. NF-001's basis. | Before the one permitted `src/` edit (the declaration's value). | AC-008 |
| `crates/happenstance-core/src/projection.rs` | `:13-30` — the port and PS-1's single transaction: why read-model write and checkpoint write are in one `BEGIN`…`COMMIT`, which is the reason a batch has pending writes to read at all. **Read, never edited.** | Before asserting anything about commit boundaries. | AC-008 |
| `crates/happenstance-testkit/src/lib.rs` | `:311-357` — the conformance macro's arms; the general arm `mod_name = …, emit = …, fixture = …` fixes the module prefix every `ARTEFACTS` name must carry. | Before copying qualified names out of `--list`. | AC-006 |
| `references/evaluation/README.md` | The evidence discipline every kept artefact in this repository obeys — immutable, dated, pinned to a commit, superseded rather than edited. NF-007's basis, applied to a record that lives in the story folder rather than under `references/`. | Before writing the record's header. | AC-007 |
| `spec/E2E-CASES.md` | E2E-19 (`:502-520`) and E2E-24 (`:622-640`) — the two cases this run makes *writable*. Read to write the pointer for the verdict; **not edited here** (D10). | Only when writing the record's hand-off paragraph. | AC-008 |

## Clarifications resolved during spec

1. **The AC set is exactly the eight the front half enumerated** — AC-001 … AC-008.
   Nothing added, nothing dropped; the ledger matches one row per id.
2. **HS-S0078 leaves the conformance target's body as the macro invocation and
   nothing else; this story adds the two `#[test]` cases to that same file.** That is
   not a contradiction of the slice-mate's scope statement — it is what "the case
   executes inside the registered target" means, and it is why the mount is the
   **existing** `ARTEFACTS` row rather than a new one. `ARTEFACTS` stays at four rows.
   If HS-P0010's probe shipped and the declaration is `true`, the suite rule's
   qualified name is already in that row and the two cases are added beside it.
3. **The two test function names are this story's to fix; the target's file name is
   not.** The slice-mate deliberately left the file name open until ADR-0025's Q3
   answer selects the emitter arm, so this spec writes `<conformance target>`
   throughout and fixes only the case names — whose exact qualified spelling is still
   copied out of `cargo test --test <target> -- --list`, never typed from memory.
4. **The composition family of Interaction quality is re-seated, not waived.**
   `_design.md` approves a no-surface project, so there is no markup to compose; the
   invariants are applied to the evidence artefact the reviewer actually meets — the
   gate scroll plus the committed run record — because that artefact has exactly the
   failure mode the family exists to catch. The design's own `## Anti-patterns` is
   `N/A`, so `discover.md`'s three named wrong implementations are adopted as the
   binding anti-pattern set, each bound to an AC.
5. **EC-003 is the one branch that changes what "executed" means.** If the merged
   `Batch` exposes no read path at all, the two-statement traversal cannot be written
   and AC-001's execution obligation is discharged by the single-statement case plus a
   recorded compile-level observation. This is PS-12's compliant declining shape, not a
   blocked story — and the substitution must be stated in the record rather than left
   as an absence.
6. **E2E-19 and E2E-24 are not written here**, confirming D10 against the option
   `_decomposition.md:429-431` left open. `spec/E2E-CASES.md` is outside this project's
   seams table; the hand-off is a pointer in the record.
7. **The record lives in this story's folder, not under `references/evaluation/`.**
   That directory carries two files in two commits — the axes
   (`preflight-and-unlike-axes`) and the verdict (`freeze-verdict-document`) — and
   AC-001 of the project is an ordering claim about those commits. A third file here
   would blur it.
8. **No ADR is written in this story.** A result implying the freeze did not hold is
   routed to `freeze-verdict-document` and to HS-P0010, which decide whether a decision
   atom is owed; recording the gap is this story's obligation, deciding it is not
   (`project.md:109-111`, `:210-213`).
