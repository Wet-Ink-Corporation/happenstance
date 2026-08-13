---
item: HS-S0079
stage: discover
created: 2026-08-12T13:02:48.806Z
updated: 2026-08-12T13:02:48.806Z
template_sig: 86ce4036
rendered_sig: 392c8e4c
---

# Discover — Read-your-own-writes inside one batch, answered

## Signal Ledger

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line — execute a projection that `MATCH`es a node it created earlier in the same batch, and record the result as a supported behaviour or a declared capability limit of the deferred write set, **never as a hypothesis** | `_storymap.md:45` | The deliverable is an executed case plus a recorded outcome. Neither half substitutes for the other. |
| **AC-005** — the projection exists **and is executed**; the result is recorded as either a supported behaviour or a stated capability limit, closing the half of PS-4 the skeleton explicitly could not | `project.md:197-201` | "Has an answer, not a hypothesis" is the AC's own phrasing, and it is aimed at exactly the shortcut of predicting the result from the design. |
| The skeleton names this as the half it cannot settle: with a deferred set the traversal runs at commit time against a graph that **does not yet contain the write** — *"a run-time capability limit, not a compile error. Phase 11 closes it by writing a projection that needs read-your-own-writes"* | `crates/happenstance-ladybug/src/projection_store.rs:57-64` | The mechanism is understood in advance. That is precisely why the temptation to record the expected answer without running it is strong, and why AC-005 forbids it. |
| **PS-12** — an adapter MUST either make reads issued through an open `Batch` reflect that batch's own pending writes, **or expose no read path on the `Batch` at all**; it MUST NOT expose a read path that answers from committed state | `spec/SPECIFICATION.md:5052-5057` | This reframes the answer. "Not visible" is not automatically a capability limit — under PS-12 an adapter that cannot answer over its pending set must expose **no** batch read path, so the honest outcome may be a statement about the *port surface*, not just about Ladybug. |
| PS-12 is `[PROVISIONAL]` and names **Ladybug as its candidate falsifier** — *"a graph traversal over buffered Cypher is not obviously cheaper than the alternative"* | `spec/SPECIFICATION.md:5058-5061` | This story is the falsification attempt PS-12 asks for by name. The result feeds the clause whether it confirms or falsifies it. |
| §4.1a question 1 — *"does any adapter need a live handle acquired before the first write?"* gates **PS-4, PS-5, PS-6, PS-12 and PS-34 together** and names *"The Ladybug skeleton"* as what answers it, *"confirmed at 11"* | `spec/SPECIFICATION.md:4809` | One behaviour, five clauses. That is the leverage this story has and the reason it is not merged into the harness story. |
| **M1's "not a subset"** — the suite invocation is the primary evidence; if HS-P0010 registered no rule generic enough, a fixture-level test **alongside** it is the fallback, not a replacement | `_decomposition.md:475`; `_decomposition.md:127-142` | Where the case lives depends on what shipped; that it runs inside the same target as the suite does not. |
| Deliberately unprescribed — *which* projection models the case is the implementer's; what is fixed is the **shape** of the question and that the answer is recorded as a behaviour or a stated limit, never a hypothesis | `_decomposition.md:421-428` | A concrete domain (students, courses, enrolments) is free; the two-writes-then-traverse structure is not. |
| Slice rationale — this is its own story because it is *"the one behaviour the deferred write set can plausibly fail"*; merging it with the harness would let a green harness stand in for an answer | `_storymap.md:75-78` | A green conformance run and an answered read-your-own-writes question are different artefacts, and only one of them is at risk. |
| `depends_on: ladybug-fixture-and-conformance-run` — supplies the fixture, the running suite target and the `ARTEFACTS` row this case is executed inside | `_storymap.md:45`, `:71-74` | There is no harness to run the case in before it, and running it outside the harness would make it a bespoke test rather than suite evidence. |
| E2E-19 and E2E-24's third-shape halves become writable at this phase; E2E-24 is framed around exactly this adapter's shape — *"a buffer of pending statements replayed in one call at commit"* | `_grounding.md:162-168`; `spec/E2E-CASES.md:502-520`, `:622-640`; `RUNBOOK.md:4440` | Whether this project *writes* them or merely makes them writable is left open (`_decomposition.md:429-431`). |
| Never assert on literal position values; the specification permits gaps and a conformant adapter may leave them | `CLAUDE.md`, *The rule that matters*; `project.md:293-294` | This case asserts on graph contents and on positions the store was handed, never on a position the store assigned. |

## Questions

**Is the answer a supported behaviour or a capability limit? — genuinely open, and
deliberately not predicted here.** The mechanism argues one way — a deferred write
set replays at commit, so a traversal in statement *n* runs against a graph
without statement *n−1*'s effect
(`crates/happenstance-ladybug/src/projection_store.rs:59-63`) — but a mechanism is
not an observation, and the whole of AC-005 is that this project stops accepting
the first as the second. What is decided now is the *procedure*: the projection is
written, executed against the real driver, and whatever it does is what gets
written down. Recording "limit" without running it is the named failure.

**Does PS-12 change what a "limit" answer means? — yes, and this is the sharpest
thing to carry into `spec`.** PS-12 does not merely permit an adapter to lack
read-your-own-writes; it requires that such an adapter expose **no read path on
the `Batch` at all**, and forbids one that answers from committed state
(`spec/SPECIFICATION.md:5052-5057`). So if the traversal turns out to observe
committed state rather than pending writes, the finding is not "Ladybug declines a
capability" — it is that this adapter, as built, sits in the shape PS-12 forbids,
and the resolution is either no batch read path or a changed port. That is a
freeze finding, and it goes to the verdict and to `projection-store-freeze`, not
into a `Capability::declined` string.

**Is a declension an acceptable discharge of AC-005? — answered: no, and the
distinction matters.** `Capability::declined` describes what the **fixture cannot
set up** (`crates/happenstance-testkit/src/contract.rs:25-63`); it is not a
vocabulary for what the **store does not do**. A declined rule does not run, and
AC-005 requires the projection to be executed. If the only way the case can be
expressed is as a declension, that is itself a finding about the suite's
vocabulary and is raised with HS-P0010 rather than used to close the AC.

**Where does the case live — a suite rule or a fixture-level test? — deferred to
`spec`, gated on what HS-P0010 registered.** The suite invocation is primary; a
local test alongside it is the fallback and never a substitute for the suite
running as a whole (`_decomposition.md:475`). Either way it runs inside the target
registered in `xtask/src/proof.rs`'s `ARTEFACTS`, so it cannot silently stop
running.

**How does `lbug`'s blocking API meet a non-blocking port (ADR-0025)? — not this
story's, and only relevant as a confound to avoid.** If the bridge blocks the
executor, a case that times out must not be read as "the traversal failed"; the
two failure modes have to be distinguishable in the recorded output. That is a
requirement on how the result is captured, not a decision this story takes.

**What counts as "structurally unlike"? — on disk since HS-S0074, and this is the
axis with teeth.** The deferred, owned write set is the axis that a projection can
actually fail against; the other three (no transaction handle type, a synchronous
driver, single-writer concurrency) are structural facts the suite may pass over
without noticing. This story is the axes document's only load-bearing test.

**Deferred to `spec`:** the domain the projection models, and whether E2E-19 and
E2E-24's third-shape halves are written here or merely made writable
(`_decomposition.md:429-431`).

## Decision

The Ladybug skeleton settled one of PS-4's two conditions and stated plainly that
it could not settle the other: whether a projection whose own logic depends on
traversing a node it created earlier in the same batch gets a usable answer from a
deferred write set is a run-time question, and no amount of type-checking reaches
it. This story asks it — one projection, two buffered statements, the second
traversing what only the first creates, executed against the real engine inside
the conformance target — and writes down what happened, in the form PS-12 demands
rather than as a hypothesis dressed as a limit. The spec will cover the
projection's shape (two distinct `GraphStatement`s in one `GraphWriteSet`, not one
statement doing both), its execution inside the registered conformance target, the
recorded outcome as either a supported behaviour or a stated limit with PS-12's
"no read path at all" consequence spelled out, and the negative control that
proves the case can distinguish statement-internal visibility from cross-statement
visibility. The answer feeds PS-12 and §4.1a's question 1, which gates PS-4, PS-5,
PS-6, PS-12 and PS-34 together — but this story only *supplies* it: no clause
marker moves here, nothing `[FROZEN]` is amended, and a result that implies the
freeze did not hold routes through the verdict to a decision atom and a re-plan
(`project.md:109-111`).

## The wrong implementation

**The case that does its write and its read in one Cypher statement.**
`MERGE (a:Student {id: $id}) WITH a MATCH (a)-[:ENROLLED_IN]->(c) RETURN c` —
one `GraphStatement`, so of course the traversal sees the `MERGE`: statement-local
visibility is a property of the query engine, not of the batch. The case is
written, it executes, it returns rows, it goes green, and the record says
"read-your-own-writes is supported." Every check passes — it is a real projection,
really executed, in the registered target — and it has answered a question nobody
asked, because a single statement is not two writes in one batch. This is the
mutant most likely to be built by accident, because collapsing two statements into
one is what a Cypher author does naturally.

The falsifier is a negative control, and it lives in
`crates/happenstance-ladybug/tests/` beside the conformance target rather than in
the testkit, because `crates/happenstance-testkit/**` is not this project's to
edit (`_decomposition.md:86-88`) and the control is about this adapter's batch, not
about the port. Two cases, run together and recorded together: the **single-statement
variant**, which is expected to succeed whatever the batch does, and the
**two-statement variant**, whose second statement refers to a node only the first
creates. If both come out the same way, the case cannot distinguish the two
mechanisms and is not evidence for anything — that is the assertion the control
makes, and it must be run before the real result is interpreted, for the same
reason the axes were committed before the suite.

**The result recorded as a declined capability.** The case is written and run, the
traversal sees nothing, and the outcome is entered as
`Capability::declined("deferred write set cannot read its own pending writes")`.
It looks disciplined — a real reason, no `#[cfg]`, the rule visible in the run
under `--show-output` — and it is the wrong instrument: a declension says the
*fixture* could not set the rule up, so the rule does not run, whereas here the
rule ran and produced an observation. Worse, it launders a specification finding
into a fixture note. Under PS-12, an adapter that cannot answer over its pending
set must expose no batch read path at all
(`spec/SPECIFICATION.md:5052-5057`), so the honest record is a statement about the
port surface that belongs in the verdict and goes to HS-P0010 — not a string in a
capability constant that the next reader will take for a local quirk.

**The answer written from the module docs.** No projection, or a projection
written and not executed, with the record reading "read-your-own-writes is not
supported; the deferred write set replays at commit," citing
`projection_store.rs:59-63`. It is *probably true*, it cites a real source, it
reads as a finding, and it is the exact substitution — mechanism for observation —
that AC-005's "not a hypothesis" clause exists to reject, and that the whole
project's verdict discipline is built to prevent. The guard is that the recorded
outcome must cite the run, by target and test name, from the `ARTEFACTS` row
(`xtask/src/proof.rs:133`) — a claim with no run behind it has nothing to cite.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
