---
item: HS-S0032
stage: discover
created: 2026-08-12T13:01:54.715Z
updated: 2026-08-12T13:01:54.715Z
template_sig: 86ce4036
rendered_sig: 55e87cbc
---

# Discover — The contract defect log, and the happenstance-macros verdict

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice: land the BR-01 record this project exists to produce — every defect using the `[FROZEN]` contract revealed, each naming its **clause ID** and its **routing** to a decision record (never a line edit), staged for `/redkiln:kb-ingest` — plus the `happenstance-macros` verdict measured over the **rewritten example**, boilerplate versus domain logic, recorded in or out either way | `_storymap.md:63` (M7 row) | Two records in one story, and both are outputs no test can produce |
| AC-012 — every defect this consumer discovers in the `[FROZEN]` contract is written down with its clause ID and routed to a decision record; **none is fixed by editing a frozen clause** | `project.md:204-206`; DoD 9 at `project.md:243-244` (*"non-speculative: each entry names a clause ID and its routing"*) | An entry without a clause ID is not an entry |
| AC-013 — *"if the rewritten example carries more mapping boilerplate than domain logic, the derive is in scope for 0.1"* — evaluated and recorded, in or out | `project.md:207-209`; `RUNBOOK.md:524` | The criterion is a **measurement**, with a stated threshold, over a named artefact |
| `depends_on: worked-example-on-typed-layer` — supplies the substrate AC-013 is measured over. The criterion *"cannot be evaluated before AC-003 lands"* | `_storymap.md:63`; `_decomposition.md:428`, `:791` | Measuring anything else answers a different question |
| `depends_on: projection-trait-and-runner` — the runner is the second place this project consumes the frozen contract in anger (streaming reads, inclusive `from`, the batch/checkpoint seam), so its defects belong in the same log | `_storymap.md:63`, `:130-132` | The log closes when both consumers have been written, not when the first has |
| **This project is the first thing to use the frozen `EventStore`, so it is where defects appear.** ES-\* is `[FROZEN]`; a defect is a re-plan, not a patch. *"Absorbing one quietly would defeat the reason phase 7 sits before phase 8"* | `project.md:270` (risk row 6); `RUNBOOK.md:256-258` | The 7-before-8 ordering exists for this story's output |
| AC-A02 — a change under `crates/happenstance-core/src/**` is admissible **only** as the recorded outcome of AC-012's route, never as a convenience edit discovered mid-implementation. Incidental bugs route to the `support` initiative | `_decomposition.md:363-371`; `project.md:270` | The rule is stated at the diff level: which paths this project's changes may touch |
| **One defect candidate already exists, discovered at design time: D-1** — *`happenstance-core` has no infallible `QueryItem` constructor for pre-validated inputs; every derived query therefore carries a `Result` that is unreachable for well-formed models.* Clause: **VT-18**, *"constructors accept values the caller already holds, and their errors compose"*, which it partially contradicts. Routed to a decision record, never a line edit | `_design.md:652-672`; VT-18 at `spec/SPECIFICATION.md:1371`, `[FROZEN]` at `:8544`; `crates/happenstance-core/src/query.rs:50-62` | The log is not empty on day one, and D-1 is the shape every other entry must take |
| **The design's falsifiable prediction on AC-013:** counted over the design's own doctest, domain logic is 11 lines and mapping ceremony 26 — **2.4:1 against the domain for a two-variant enum, worsening with a third**. *"The prediction this design records: AC-013's verdict is '`happenstance-macros` is in scope for 0.1'… That prediction is falsifiable and must be checked against the rewritten example, not against this doctest"* | `_design.md:1104-1111` | A prediction with a number, an artefact and an instruction to re-measure elsewhere. This story does the re-measurement |
| The two criteria are the same measurement read at two altitudes: *"if the resolved DT-2 answer produces a first program whose ceremony exceeds its domain logic, AC-U01 has been satisfied in form and failed in substance — and that is precisely the condition AC-013 tests for. Answer them together"* | `_decomposition.md:307-312` | AC-U01 and AC-013 are one instrument; a verdict that ignores the doctest's own ratio is half a reading |
| If the answer is *in*, that is a **new crate** under `crates/` and a new workspace member — *"a scope change for the runbook to take, not an implementer"*, and not a story on this map | `_decomposition.md:428`; `_storymap.md:155-157` | The verdict is the deliverable. Building the derive is not |
| There is no `happenstance-macros` crate in `crates/` today; nine crates exist and that is not one of them, so AC-013 is a live unresolved question | `_grounding.md:22-24` | Nothing has pre-empted the answer |
| Atoms are authored by `/redkiln:kb-ingest` from `.kb/_intake/`, human-invoked; the defect entries stage there like ADR-0020 and ADR-0021 did | `_decomposition.md:643-651`, `:427`; `_storymap.md:163-165` | Plan the ingest as a handoff at the point the material is ready |
| Verification is at closeout, not by a compiled test — both AC-012 and AC-013 are "Record, not a test" in the testing brief's taxonomy | `_decomposition.md:790-791`; `project.md:241-244` (DoD 8, DoD 9) | Nothing mechanical will notice if this story produces nothing |

## Questions

**Answered here.**

- *Is the defect log empty?* No. D-1 exists already, discovered while resolving DT-2 in
  `_design.md`, and it names its clause (VT-18) and its routing
  (`_design.md:667-672`). It is the template for every subsequent entry.
- *What does an entry contain?* At minimum a clause ID, what was attempted, what the contract
  did, and the routing. DoD 9 requires the log to be non-speculative
  (`project.md:243-244`), which is a bar on each entry rather than on the document.
- *Which artefact is AC-013 measured over?* The **rewritten example**, not the design's
  doctest (`RUNBOOK.md:524`; `_design.md:1110-1111`). The doctest's 2.4:1 is a prior, and it
  is recorded as falsifiable precisely so the re-measurement can contradict it.
- *If the verdict is "in", does this story build the derive?* No. That is a new workspace
  member and a scope change for the runbook (`_decomposition.md:428`).

**Deferred, with owners.**

- *The verdict itself.* Genuinely open. The design predicts *in* with a number; this story
  must count over the rewritten example and record whichever answer the count gives, with the
  counting method stated so a reader can disagree with it.
- *Which defects exist beyond D-1.* Unknowable at discovery by construction — the log's whole
  premise is that defects are found by *use*, and the use is M2 through M6. Discovery's
  contribution is the entry shape and the routing rule, not a list.
- *The ADRs the routed defects become.* `/redkiln:kb-ingest` is human-invoked and authors the
  atoms; this story stages material and hands off (`_storymap.md:163-165`).
- *Where an incidental (non-clause) bug goes.* The `support` initiative, per
  `.redkiln/config.yaml`, cited at `project.md:270`. Discovery's rule: if it has a clause ID
  it is a defect entry; if it does not, it is a support item — and the distinction is made
  when it is found, not later.

**Not blocked** on `trybuild`. **Inherits** `projection-trait-and-runner`'s dependency on
HS-P0010's `MemoryProjectionStore`, since defects the runner reveals cannot all be found
until the runner can be exercised.

## Decision

The problem this slice solves is that a frozen contract that has never been used is a
hypothesis, and this project is the first consumer that can falsify it — which means it is
the one place in the plan where contract defects will surface, and the one place where the
cheapest response is also the most destructive: quietly adding the convenience the contract
lacks and moving on. This story is the discipline that stops that: every defect using the
`[FROZEN]` contract reveals is written down with its clause ID and its routing to a decision
record, and none is fixed by a line edit. Alongside it sits the second question this project
was told to answer either way — whether the mapping boilerplate a `DomainEvent` impl costs
is large enough that `happenstance-macros` belongs in 0.1 — measured over the rewritten
example rather than over any convenient proxy, with the design's own 2.4:1 prediction on
record to be confirmed or contradicted. The spec for this story covers the defect log's
location and entry format (clause ID, what was attempted, what the contract did, the
routing), D-1 as its first entry, the rule distinguishing a clause-bearing defect from an
incidental bug bound for the `support` initiative, the staging of routed entries into
`.kb/_intake/` and the human handoff to `/redkiln:kb-ingest`, the counting method for
AC-013 and the artefact it is applied to, and the verdict recorded in the project's closeout
in whichever direction the count goes — with the consequence, if *in*, being a scope change
raised to the runbook rather than a crate built here. No `[FROZEN]` clause is amended; this
story is the mechanism by which one *could* be, correctly, later.

## The wrong implementation

**The mutant: absorbing the defect instead of logging it.**

This is the one the 7-before-8 ordering exists to prevent, and it takes one commit. While
implementing `Boundary::query`, the author hits the residual D-1 already names — every
derived query carries a `Result` that cannot fire for a well-formed model, because
`QueryItem::new` is fallible even for pre-validated inputs
(`crates/happenstance-core/src/query.rs:50-62`). The obvious fix is four lines:

```rust
// crates/happenstance-core/src/query.rs
impl QueryItem {
    /// Infallible: both inputs are already validated.
    #[must_use]
    pub fn from_validated(types: Vec<EventType>, tags: Tags) -> Self { Self { types, tags } }
}
```

Everything goes green. `cargo xtask affected --base main` runs fmt, clippy `-D warnings` and
the tests for the touched packages and their dependents, and they all pass — the new
constructor breaks nothing. `cargo xtask ci` passes, including `spec-trace`, because no
clause text changed. `redkiln validate --kb` passes, because it checks KB frontmatter and
accepted-atom immutability, **not** the specification's frozen clauses. The typed layer is
genuinely nicer: the `Result` is gone from the derivation and the first program is shorter.
There is no mechanical check anywhere in this repository that fails.

It is wrong because a `[FROZEN]` clause has been amended by a line edit with no ADR, which
is exactly what AC-A02 forbids and what `project.md`'s risk table calls out by name —
*"absorbing one quietly would defeat the reason phase 7 sits before phase 8"*. The damage is
not the constructor; the constructor may well be the right answer. The damage is that the
decision was taken by whoever was mid-implementation on a Tuesday, with no record of the
alternatives, no clause citation, and nothing for the adapter authors who pin
`happenstance-core` to read. Six adapters will be written against that contract, and the one
artefact that would have told them why the surface changed does not exist.

The discriminator is the diff's *paths*: this project's changes touch
`crates/happenstance/`, `crates/happenstance-testkit/`, `examples/course-subscriptions/`,
`xtask/`, `spec/SPECIFICATION.md`, `CHANGELOG.md` and `.kb/` (`_decomposition.md:363-371`).
A change under `crates/happenstance-core/src/**` is admissible only with a log entry naming
its clause and its routing — and D-1 shows what that looks like, written *before* anyone was
tempted.

**A second mutant: a speculative log.** A `defects.md` containing *"the contract's query
construction is a bit awkward for derived queries"* and *"the projection port may want a
batch-size hint"*. It exists, it is committed, closeout has a document to point at, and DoD 9
reads as satisfied because a defect log exists. It is worthless: no clause ID, so nobody can
tell which promise is at issue; no routing, so nothing happens next; no statement of what was
attempted, so a reader cannot judge whether the awkwardness was real or a misuse. DoD 9 says
*non-speculative* and *"each entry names a clause ID and its routing"*
(`project.md:243-244`) for exactly this reason, and D-1 is the counter-example: it names
VT-18, states the contradiction precisely, and says where it goes.

**A third mutant, specific to AC-013: measuring the wrong text.** Counting boilerplate
against `_design.md`'s doctest — which is right there, already counted at 2.4:1, and
conveniently produces a verdict — rather than against the rewritten example. The design
anticipates this and forbids it in the same paragraph that supplies the number:
*"that prediction is falsifiable and must be checked against the rewritten example, not
against this doctest"* (`_design.md:1110-1111`), and `RUNBOOK.md:524` names the example. The
two texts can disagree: a doctest is one two-variant enum with an empty `Tags`, while the
example has three event types, real tag scopes and multiple decision models, so the ratio
could move in either direction. A verdict that reuses the prior instead of taking the
measurement has answered AC-013 by citation, and the whole criterion is that it be answered
*either way* on evidence.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

The two judgement boxes, and the second is the substance of this story rather than a
formality. **Literal positions:** this story adds no conformance rule and no test at all —
its outputs are two records, verified at closeout (`_decomposition.md:790-791`). Vacuously
true, ticked on that basis. **Frozen clauses:** this story *is* the box's mechanism. It
amends no frozen clause; it collects the cases where one arguably should be amended, names
each by ID, and routes each to a decision record so that any future amendment is preceded by
an ADR rather than performed by a line edit. D-1 against VT-18 is the first such entry and
is already written. The mutant above is precisely the violation this box exists to catch,
and it is named so that it is recognisable when it is tempting.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
