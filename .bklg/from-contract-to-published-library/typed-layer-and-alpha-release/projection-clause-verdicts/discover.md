---
item: HS-S0027
stage: discover
created: 2026-08-12T13:01:49.949Z
updated: 2026-08-12T13:01:49.949Z
template_sig: 86ce4036
rendered_sig: 5100b5a8
---

# Discover — PS-33, PS-27, PS-30 settled and PS-18 excluded, on the record

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice: write the verdicts the runner's existence makes answerable — PS-33 discharged by naming an independent caller of the checkpoint pump or by the superseding ADR that collapses it upward, PS-27 and PS-30 settled by counting with their integration tests, PS-18 recorded as a **documented exclusion** — as the three-part `spec/SPECIFICATION.md` edit: generated §7.1/§7.2, the hand-written §1.3 census, and IDs retained on exit, with `cargo xtask spec-trace` green | `_storymap.md:58` (M5 row) | Four clauses, three different dispositions, and one edit mechanism that is easy to get half-right |
| AC-007 — PS-33 is evaluated, not inherited: discharged by naming an independent caller *or* by writing the superseding ADR. **An unevaluated falsifier fails this criterion** | `project.md:185-188` | "We built the runner, so PS-33 is answered" is the failure mode named in the criterion itself |
| AC-008 — PS-18's protection variant, PS-27's skip record and PS-30's fan-out runner each name their callers or are promoted to a documented exclusion | `project.md:189-191`; `RUNBOOK.md:4070-4077` (the escape hatch, cited by both briefs) | *Name the callers, or promote to a documented exclusion.* Both branches are legitimate; silence is not |
| `depends_on: projection-trait-and-runner` — supplies the thing being counted. PS-33 asks whether the core pump acquires a caller other than the typed one; PS-27 and PS-30 are properties of a runner that must exist to have them | `_storymap.md:58`, `:124-126` | The verdicts cannot precede the runner, which is why M5 orders them this way |
| PS-33's own status: `[DEFERRED — the experiment is the typed layer itself; the question is answered by counting callers. Owned by the typed-layer phase]`, rule *"none; it is a phase gate, not an adapter obligation"* | `_grounding.md:152-156`, reading `spec/SPECIFICATION.md:5529` | Evaluated at this phase's exit **and nowhere else** (`RUNBOOK.md:408-413`) |
| What the count is actually over: at HEAD `crates/happenstance-core/src/projection.rs` holds the *port*, `ProjectionId` and nothing that runs — **there is no pump function in the contract crate to have callers**. So the verdict is a judgement about where the checkpoint invariant lives, with two admissible outcomes | `_decomposition.md:422` (AC-007 row) | A literal caller count returns zero for a reason that has nothing to do with the question |
| PS-27: skip-and-record atomicity, *"evaluated at the exit of the typed-layer phase against the Kestrel Motor shred case"*, with a named rule and E2E cases E2E-26/E2E-27 | `_grounding.md:147-149`; `_decomposition.md:786` | Explicitly this project's, and **testable** |
| PS-30: fan-out runner panic rollback, *"falsified if the fan-out runner is not built… Owned by the typed-layer phase"*, rule `panicking_apply_rolls_back`, case E2E-28 whose header marks its span *"⚠ e2e crate"* — and **no workspace e2e crate exists** | `_grounding.md:150-152`; `_decomposition.md:786` | Where PS-30's test lands is `_design.md`'s to place; that no home exists is a fact to plan around, not to discover late |
| A structural fact that bears on PS-30: `Batch<'a>` borrows from the store, so it is not `'static` and **cannot be moved into a `tokio::spawn`** — a fan-out runner cannot move a batch into a task | `_decomposition.md:697-700`; `crates/happenstance-core/src/projection.rs:96-99` | This is evidence for the verdict, available today |
| **PS-18 is not evaluable in this tree.** Its clause text assigns evaluation to *"the exit of the projection-port phase"* and asks whether the SQLite adapter implemented it; neither HS-P0010 nor HS-P0012 has shipped an adapter, and **PS-18's subject does not exist** — there is no `reset` method and no `ResetError` anywhere in `crates/happenstance-core/src/projection.rs`, whose port is `checkpoint`, `begin`, `commit`, `rollback`. *"Plan for the exclusion branch and be pleasantly surprised"* | `_decomposition.md:660-673` (Tensions 1); `_grounding.md:140-146`, `:163-176`; `crates/happenstance-core/src/projection.rs:96-130` | A count of zero over an empty set is not a count. The honest disposition is the documented exclusion |
| **The three-part edit mechanism.** §7.1/§7.2 are *generated* (`spec-trace --write`) and the gate compares the committed region against a fresh computation; §1.3's census is written **by a human and only checked** — the total, each maturity count, the "of which N are normative" figure and the document's own subtraction — deliberately never generated, because generating both would let them drift together silently; and an ID that leaves the clause space is **retained**, marked `[NON-NORMATIVE]`, *"its ID is retained so that citations resolve"*, with CF-30 as the worked example | `_decomposition.md:616-641`; `spec/SPECIFICATION.md:209-221` | Today's census reads 200 clause IDs, 198 normative: 139 FROZEN, 49 PROVISIONAL, 10 DEFERRED, two NON-NORMATIVE. Moving PS-32/PS-33/PS-35 out means editing those numbers by hand |
| A `[PROVISIONAL]` or `[DEFERRED]` marker with an **empty** falsifier is a build failure rather than a convention (CF-38); and a provisional marker whose falsifier is *"further thought"* is not a falsifier — *"it must name an artefact, a measurement or a deployment that would produce a different answer"* | `spec/SPECIFICATION.md:200-217` | The first half is mechanised. **The second half is not**, and that gap is this story's central hazard |
| DR-06 — every provisional marker this phase is scheduled to evaluate is evaluated and the verdict written, and PS-32/PS-33/PS-35 leave the clause space **with their IDs retained** | `project.md:147` | Three IDs leaving, and the census arithmetic changes with them |
| `cargo xtask spec-trace` is a `REQUIRED` gate step | `_decomposition.md:618-620`; `CLAUDE.md`, *Commands* | The edit is checked mechanically for consistency — and only for consistency |

## Questions

**Answered here.**

- *What does PS-33's count run over?* Not a function. `crates/happenstance-core/src/projection.rs`
  holds the port and `ProjectionId` and nothing that executes, so there is no pump *function*
  whose callers could be counted (`_decomposition.md:422`). The verdict is a judgement about
  where the checkpoint invariant lives, and the two admissible outcomes are named in AC-007:
  name the independent caller, or write the superseding ADR that collapses the pump upward.
- *Is PS-18 settled by counting here?* No. Its subject does not exist in the tree — no
  `reset`, no `ResetError` (`crates/happenstance-core/src/projection.rs:96-130`) — and its own
  clause text assigns evaluation to the projection-port phase. It is recorded as a documented
  exclusion under the runbook's own escape hatch, with the reason stated
  (`_decomposition.md:660-673`).
- *Does this story amend a frozen clause?* No. PS-18, PS-27, PS-30 are `[PROVISIONAL]` and
  PS-33 is `[DEFERRED]`. The census counts change as a consequence, which is arithmetic about
  frozen clauses rather than a change to one.

**Deferred, with owners.**

- *Where PS-27's and PS-30's integration tests live.* `_design.md`'s call — a `tests/`
  directory under `crates/happenstance/`, or a new workspace e2e crate if one is warranted.
  E2E-28's case header marks its span *"⚠ e2e crate"* and no such crate exists
  (`_decomposition.md:786`). Discovery's contribution is that the absence is known now.
- *Whether PS-33 resolves by naming a caller or by a superseding ADR.* Genuinely open, and it
  is the verdict. Evidence available today points toward the second branch — there is nothing
  in the contract crate to call — but the record must argue it rather than assume it, and an
  ADR is authored through `/redkiln:kb-ingest`, which is a human handoff.
- *PS-3's verdict* — whether the projection port ships behind `unstable-projection` as a
  matter of record — stays HS-P0016's (`project.md:267`). This story settles the four clauses
  named in its title and no others.

**Not blocked** on `trybuild`. **Downstream of** `projection-trait-and-runner`, which is
itself blocked on HS-P0010's `MemoryProjectionStore` for its transactional test — PS-27's and
PS-30's integration tests inherit that dependency, since both are properties of a runner
committing into a `ProjectionStore`.

## Decision

The problem this slice solves is that four clauses in the specification have been waiting for
this phase to exist: three of them ask *"does anyone actually call this?"* and one asks
whether ADR-0007's split survives contact with a real consumer, and each was deliberately
left provisional rather than guessed at. Building the runner makes them answerable; it does
not answer them, and the difference is the whole of AC-007 — an unevaluated falsifier fails
the criterion even if the code it was about now exists. This story writes the four verdicts
and lands them as the three-part specification edit the document's own mechanics require:
regenerating §7.1/§7.2, editing §1.3's hand-written census arithmetic to match, and retaining
the IDs of PS-32/PS-33/PS-35 as they leave the clause space so every citation that names them
— `RUNBOOK.md`'s included — still resolves. The spec for this story covers each clause's
disposition and the evidence for it (PS-33's judgement about where the checkpoint invariant
lives, with the two admissible branches; PS-27 and PS-30 counted with their integration tests
named; PS-18 promoted to a documented exclusion with its reason, since its subject does not
exist in the tree), the replacement text for every marker that changes, the exact census
numbers before and after, and a green `cargo xtask spec-trace`. Where the verdict is the
superseding ADR, that ADR is staged into `.kb/_intake/` for a human-invoked ingest, not
hand-written into `.kb/decisions/`.

## The wrong implementation

**The mutant: discharging a falsifier by rewording it.**

```diff
-**PS-33** … `[DEFERRED — the experiment is the typed layer itself; the question is
- answered by counting callers. Owned by the typed-layer phase]`
+**PS-33** … `[PROVISIONAL — falsified if experience with the typed layer suggests the
+ split should be revisited]`
```

Every mechanical check passes. The marker is non-empty, so CF-38's build failure does not
fire — CF-38 catches an *empty* falsifier (`spec/SPECIFICATION.md:213-217`). §7.1/§7.2
regenerate cleanly. §1.3's census is unchanged, because the clause did not leave the space
and the `DEFERRED`→`PROVISIONAL` move is a two-number edit that is easy to get right.
`cargo xtask spec-trace` is green, `cargo xtask ci` is green, and the diff reads as
diligence: someone clearly *looked* at PS-33.

It is wrong because the document says, four lines above the marker syntax, exactly what this
is: *"a provisional marker whose falsifier is 'further thought' is not a falsifier; it must
name an artefact, a measurement or a deployment that would produce a different answer"*
(`spec/SPECIFICATION.md:200-202`). Nothing enforces that sentence. The parser checks that a
falsifier is *present*, not that it is *falsifiable*, so this mutant converts a question the
runbook scheduled for exactly this phase into one that can never come due — and, as the same
section warns about the general case, by the time anyone notices it has been load-bearing for
a year. AC-007 anticipates this precisely: an unevaluated falsifier fails the criterion, and
"evaluated" means a verdict with evidence, not a marker with new prose.

The discriminator is human and must be written into the spec as a required output: for each
of the four clauses, a verdict paragraph naming *what was examined* and *what the answer
was*, such that a reader who disagrees knows which fact to attack. A marker edit with no such
paragraph is this mutant.

**A second mutant: settling PS-18 by counting to zero.** *"We counted the callers of reset
protection. There are none. PS-18 is falsified."* It is a true sentence, it satisfies AC-008
read literally — PS-18's variant named its callers, and the count was zero — and nothing
disagrees. It is wrong because there is no `reset` method and no `ResetError` anywhere in
`crates/happenstance-core/src/projection.rs` (verified: the port is `checkpoint`, `begin`,
`commit`, `rollback`), so the count ran over an empty set: it measured the absence of the
subject, not the absence of demand for it. HS-P0010 owns introducing the subject, and both
sibling projects that could supply an adapter are still at `stage: storymap`. The honest
disposition is the documented exclusion the runbook's escape hatch permits, stating *why* the
count is unavailable — which is a different claim, and a recoverable one.

**A third mutant: deleting the IDs that leave the clause space.** PS-32, PS-33 and PS-35
exit; removing their headings and their rows makes §7.1/§7.2 regenerate cleanly and makes the
census arithmetic *easier*, because there is nothing left to count. And every citation that
names them stops resolving — `RUNBOOK.md`'s among them — which is why the document's own rule
is that an ID leaving the space is **retained** and marked `[NON-NORMATIVE]`, *"its ID is
retained so that citations resolve"*, with CF-30 as the worked example
(`spec/SPECIFICATION.md:209-212`). `spec-trace` checks the document against itself; it cannot
know that `RUNBOOK.md:408-413` was pointing at something.

**A fourth, and it is the quiet one: regenerating §7.1/§7.2 and leaving §1.3 alone.**
`cargo xtask spec-trace --write` renders the generated sections, so the diff looks complete
and the gate's comparison of the committed region against a fresh computation passes. §1.3's
census is written by a human and only *checked* — deliberately never generated, because
§7.1/§7.2 and the census come from the same parser and would drift together silently
(`_decomposition.md:626-632`). Here the check does its job and fails the gate, which is the
one place in this story where a mutant is caught mechanically; it is named anyway, because
the temptation on seeing that failure is to make the numbers agree rather than to work out
which of the two is wrong.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

The two judgement boxes, and this is the story where the second needed the most care.
**Literal positions:** this story adds no conformance rule; PS-27's and PS-30's rules
(`skip_and_record_is_atomic`, `panicking_apply_rolls_back`) are already named in the
specification and their integration tests assert on atomicity and rollback, not on position
values. Where a checkpoint position is observed, it is compared against the position the
store assigned. Ticked, checked rather than assumed. **Frozen clauses:** none is amended.
All four clauses this story dispositions are `[PROVISIONAL]` (PS-18, PS-27, PS-30) or
`[DEFERRED]` (PS-33). §1.3's census counts *include* the frozen total and change as
arithmetic — 139 `[FROZEN]` is a count of frozen clauses, not a clause — and no `[FROZEN]`
clause's text, marker or rule is touched. If a verdict were to require amending one, this
story stops and writes the ADR first; that is the branch, and it is not the expected one.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
