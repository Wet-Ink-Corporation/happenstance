---
item: HS-S0032
stage: spec
created: 2026-08-12T13:46:29.793Z
updated: 2026-08-12T13:46:29.793Z
template_sig: 87bbf1d0
rendered_sig: 191e74d1
---

# Spec — The contract defect log, and the happenstance-macros verdict

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — BR-01 (`:284`), AC-10, DoD 12 |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md` — AC-012 (`:204-206`), AC-013 (`:207-209`), DoD 8/9 (`:241-244`), risk row 6 (`:270`) |
| This spec | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/defect-log-and-macros-verdict/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md` — *Architecture brief*, AC-A02 (`:363-371`) and the AC-012/AC-013 seam rows (`:427-428`); *The ADR route, and who invokes it* (`:643-656`); *UX brief → What would make this brief wrong* (`:307-312`); *Testing brief*, AC-012/AC-013 rows (`:790-791`) |
| Signed-off design | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` — **defect candidate D-1** and the residual it came from (`:652-672`), the `EVENT_TYPES`↔`event_type()` residual that *is* AC-013's measurement (`:641`), and the falsifiable 2.4:1 prediction (`:1104-1111`) |
| This story's discovery | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/defect-log-and-macros-verdict/discover.md` — the signal ledger, and the three named mutants (`:90-153`) |
| Story map row | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_storymap.md:63` (M7 `alpha-release`) |
| Roadmap pointer | `RUNBOOK.md:524` (the `happenstance-macros` criterion and its "open" row); `RUNBOOK.md:4078-4080` (the exit box); `RUNBOOK.md:4104` (phase 7's empty session log); `RUNBOOK.md:256-258` (why 7 precedes 8 at all) |

## One-line PR slice

Land the BR-01 record this project exists to produce — every defect using the
`[FROZEN]` contract revealed, each naming its clause ID and its routing to a decision
record (never a line edit — AC-A02), staged for `/redkiln:kb-ingest`; plus the
`happenstance-macros` verdict measured over the *rewritten* example — boilerplate
versus domain logic (`RUNBOOK.md:524`) — recorded in or out either way.

## Executive summary

**What this PR lands.** Two records and no code. The first is the **contract defect
log**: a dated, pinned document under `references/evaluation/` carrying one entry per
defect that using the frozen `EventStore`/`Query`/`ProjectionStore` contract revealed
across M2–M6, each entry naming a clause ID, what was attempted, what the contract did,
and where the finding goes next — with `_design.md`'s **D-1** (`:667-672`) already
written as entry one and `projection-clause-verdicts`' **CF-36** finding already routed
here by name. The second is the **`happenstance-macros` verdict**: a count of mapping
ceremony against domain logic taken over `examples/course-subscriptions/src/main.rs` as
the slice-mate rewrote it, with the classification published line-range by line-range so
a reader can disagree with the judgement rather than only with the number, and the
verdict recorded *in or out either way* — including against `_design.md`'s own
prediction that it lands "in".

**Pointer, not a restatement.** The project charter already says why both belong here
(`project.md`, AC-012/AC-013 and DoD 8/9) and the runbook already states the macros
criterion and where its answer is written (`RUNBOOK.md:524`, `:4078-4080`). What this
spec adds is the *mechanics and the traps*: where each record lives so it survives the
`_intake` sweep that clears its own staging area, what an entry must contain to clear
DoD 9's *non-speculative* bar, the counting method that makes AC-013 a measurement
rather than an opinion, the one place the runbook's own decision table routes this
question to an atom that will already be immutable by the time this story runs, and the
three ways this story is normally failed rather than done.

**The bar, and it is unusual for this project.** Nothing mechanical will notice if this
story produces nothing: both criteria are *"Record, not a test"* in the testing brief's
taxonomy (`_decomposition.md:790-791`), and every gate step in this repository stays
green on an empty log. What *is* mechanical is the inverse — the **PR boundary fence**
below excludes `crates/**` and `spec/**`, so `redkiln verify --grain story` fails the
primary mutant (absorbing a defect with a four-line convenience edit) by path, on the
diff, without needing to understand it.

## Context pack

Everything below is a decision this story must honor. Read it before opening anything.

### 1. What this story is defending against, stated as the commit that would do it

`discover.md:90-131` names it precisely and it takes one commit. Mid-implementation on
`Boundary::query`, the author meets the residual D-1 already describes — `QueryItem::new`
returns `Result` even for inputs that are already validated
(`crates/happenstance-core/src/query.rs:56`, whose own `# Errors` doc explains the
fallible-conversion bound at `:48-55`) — and adds a four-line
`QueryItem::from_validated`. **Everything goes green.** `cargo xtask affected --base main`
passes, `cargo xtask ci` passes including `spec-trace` because no clause *text* changed,
`redkiln validate --kb` passes because it checks atom frontmatter and accepted-atom
immutability rather than the specification's frozen clauses, and the typed layer is
genuinely nicer.

It is wrong because a `[FROZEN]` clause was amended by a line edit with no ADR. **The
damage is not the constructor — the constructor may well be the right answer.** The
damage is that the decision was taken by whoever was mid-implementation, with no record
of the alternatives, no clause citation, and nothing for the six adapter authors who
will pin `happenstance-core` to read. That is the reason phase 7 sits before phase 8 at
all (`RUNBOOK.md:256-258`; `project.md:270`).

### 2. An entry without a clause ID is not an entry

DoD 9 sets the bar on the *entry*, not on the document: *"The defect log from BR-01
exists and is non-speculative: each entry names a clause ID and its routing"*
(`project.md:243-244`). `discover.md:133-141` names the mutant this forbids — a
`defects.md` containing *"the contract's query construction is a bit awkward"*. It
exists, it is committed, closeout has something to point at, and it is worthless: no
clause ID, so nobody can tell which promise is at issue; no routing, so nothing happens
next; no statement of what was attempted, so a reader cannot judge whether the
awkwardness was real or a misuse.

**The entry shape is therefore fixed, and D-1 is its worked example.** Every entry
carries: an id; the **clause ID** it bears on and that clause's maturity marker; **what
was attempted**, concretely, with the call site; **what the contract did** instead;
**why this is a contract defect and not a misuse** (the discriminator a reader would
otherwise have to guess at); and **the routing**. D-1 in full, from `_design.md:667-672`:
*"`happenstance-core` has no infallible `QueryItem` constructor for pre-validated
inputs; every derived query therefore carries a `Result` that is unreachable for
well-formed models."* Clause: **VT-18** — *"constructors accept values the caller
already holds, and their errors compose"* (`spec/SPECIFICATION.md:1371`, `[FROZEN]` at
`:8544`) — which it partially contradicts. Routed to a decision record, never a line
edit.

### 3. Clause ID present ⇒ defect entry. Clause ID absent ⇒ `support`. The call is made when the finding is made

`.redkiln/config.yaml:5` declares `support_initiative: support` and `.bklg/support/`
exists in the tree. `project.md:270` routes *"incidental bugs"* there rather than
expanding this project. Discovery's rule, adopted here as binding: **if it has a clause
ID it is a defect entry; if it does not, it is a support item — and the distinction is
made when it is found, not later** (`discover.md:59-62`). Filing the support item is a
`redkiln new` invocation and therefore a **handoff**, not a step inside this story; the
log records the finding, its classification and the fact that it was handed off, and
`.bklg/support/**` stays outside this story's PR boundary.

### 4. The log's home is chosen so it survives the ingest that clears its own staging area

Two homes, on purpose, and the pairing is this repository's standing discipline
(`CLAUDE.md`, *Where the work lives* — the atom and the long record).

- **The durable long form:** `references/evaluation/phase-7-contract-defects.md`. That
  directory is *"evidence, not instructions… dated, pinned to a commit, cited by
  `file:line`… must be superseded rather than edited"*
  (`references/evaluation/README.md:3-12`), which is exactly the lifecycle a defect log
  wants. The precedent for adding one is in the same README: `review-citation-drift.md`
  is *"a byproduct: defects surfaced while building `standards/rust/`, each re-verified
  directly afterwards, recorded because the alternative was losing them"*
  (`:40-48`). **Because that README enumerates its own contents by lifecycle, adding a
  file without adding its row makes the README false** — so the README's *"Later
  additions, which are neither"* section gains this document.
- **The staged claims:** `.kb/_intake/`. It is *"the default input to
  `/redkiln:kb-ingest`"* and the only one (`.kb/_intake/README.md:3-5`); a document
  anywhere else is never ingested, never becomes an atom, and never enters the corpus.
  Staged material is raw and deliberately **not** held to `KbFrontmatter`, because
  `redkiln validate --kb` skips `_`-prefixed directories by design (`:21-27`).

The split is load-bearing rather than tidy: **a successful ingest clears `_intake`**
(`.kb/_intake/README.md:13-19`), so a defect log that lived only there would be replaced
by ~100-line atoms and the "what was attempted / what the contract did" detail — the
part that lets a later reader judge whether the defect was real — would exist only in
git history. The staged document cites the long form in its proposed `source_paths`.

### 5. Ingest is a human handoff. This story ends at "staged and ready"

`/redkiln:kb-ingest` is human-invoked and authors the atoms; hand-writing a
`.kb/decisions/` atom produces *"the directory layout of the process without the
process"*, which is why the first attempt at it was reverted at `0269720` (`CLAUDE.md`,
*Where the work lives*; `_decomposition.md:643-651`; `_storymap.md:163-166`). This story
writes nothing under `.kb/decisions/`, `.kb/open-questions/` or `.kb/maps/`, and runs no
ingest. Note also that this is the wave **after** M1's: the ADR-0020/ADR-0021 wave has
already run and cleared `_intake`, so these documents take distinct filenames and do not
overwrite that wave's audit trail.

### 6. The runbook routes the macros question to an atom that will already be immutable

`RUNBOOK.md:524` is the decision-table row — *"Is `happenstance-macros` in scope for 0.1
| 7 | open — the criterion is stated in phase 7 and evaluated in its session log |
0020"* — and its last column says **ADR-0020**. ADR-0020 is M1's, it is accepted before
M2 writes a line, and **an accepted decision atom is immutable**. Worse, ADR-0020's own
spec deliberately carries the 2.4:1 figure as a *consequence with a falsifiable
prediction* and states that AC-013's verdict is *"a later obligation owned by the
project's closeout… not settled here"*
(`adr-0020-fold-query-agreement/spec.md:323`).

**Decision: the verdict is staged as its own intake document and is never an edit to
ADR-0020's atom.** If the ingest wave decides the right shape is a supersession of
ADR-0020, that is the wave's adjudication to make from the staged material — not this
story's, and not a hand-edit either way.

### 7. AC-013 is a measurement with a stated threshold over a named artefact

The criterion is one sentence: *"if the rewritten example carries more mapping
boilerplate than domain logic, the derive is in scope for 0.1"* (`RUNBOOK.md:524`;
`project.md:207-209`). Three things follow, and each closes a way of getting it wrong.

**The artefact is the rewritten example**, `examples/course-subscriptions/src/main.rs`
as `worked-example-on-typed-layer` leaves it — not `_design.md`'s doctest. The design
forbids the substitution in the same paragraph that supplies the number: *"that
prediction is falsifiable and must be checked against the rewritten example, not against
this doctest"* (`_design.md:1110-1111`). The two texts can genuinely disagree — the
doctest is one two-variant enum with an empty `Tags`, the example has more event types,
real tag scopes and multiple decision models — so the ratio may move in either direction
(`discover.md:143-153`). The slice-mate is under a matching obligation from its own side:
its **NF-002** requires the mapping written *plainly*, with no local `macro_rules!`, no
helper trait and no blanket impl that shrinks the `DomainEvent` impl, precisely because
this story reads that file (`worked-example-on-typed-layer/spec.md:435`).

**The threshold is 1:1 and it is mechanical.** *More* ceremony than domain means the
ratio exceeds one. The judgement therefore does not live in the threshold — it lives
entirely in the **classification**, which is why the classification is published line
range by line range and the verdict is reproducible from it. Ceremony is what a derive
would emit: `const EVENT_TYPES`, `event_type()`, `tags()`, and the encode/decode
plumbing — plus `assert_domain_event`, which `_design.md:641` names as the residual that
*"is AC-013's measurement"* and which exists only because a hand-written impl cannot
enforce `EVENT_TYPES`↔`event_type()` agreement. Domain is the event enum's variants and
payloads, the model's state, `apply`'s arms, `scope`, and the decision function's body.
Neither category takes `main`'s I/O, the transcript printing, store construction or
imports — counting the print statements as domain swamps the ratio in the direction of
"out" without anyone deciding anything.

**Both altitudes are reported.** *"AC-U01 and AC-013 are the same measurement read at
two altitudes… answer them together"* (`_decomposition.md:307-312`). The record carries
the doctest's 11:26 as the recorded prior and the example's count as the criterion, and
says which is which.

### 8. "Recorded either way" means the record must be able to say "out"

The design predicts *in*, with a number and on the record (`_design.md:1104-1111`). A
verdict that reuses the prior instead of taking the measurement has answered AC-013 by
citation, and the whole criterion is that it be answered *either way* on evidence
(`discover.md:151-153`). So the record states the prediction, states the count, and
states explicitly whether the count **confirms or contradicts** it. A contradiction is a
successful outcome of this story, not a problem with it.

### 9. If the verdict is "in", this story does not build the derive

*"If the answer is `in`, that is a **new crate** under `crates/` and a new workspace
member — a scope change for the runbook to take, not an implementer"*
(`_decomposition.md:428`; `_storymap.md:155-157`). The deliverable is the verdict plus
the escalation, addressed to the plan of record. `publish-0-2-0-alpha-1` depends on this
story for the *record*, never for a crate, and must not be blocked on one.

### 10. Where the verdict is written, and the one boundary tension it raises

The runbook assigns the location by name: *"The `happenstance-macros` criterion is
evaluated in the session log"* (`RUNBOOK.md:4078-4080`), phase 7's session log is at
`RUNBOOK.md:4104` and is empty, and the `:524` row still reads `open`. Leaving either
untouched after the verdict exists is the *"marker that has quietly become decoration"*
failure this repository has an open question about; leaving them is not a neutral
default.

**The tension, named rather than papered over.** AC-A02 enumerates this project's diff
as `crates/happenstance/`, `crates/happenstance-testkit/`, `examples/course-subscriptions/`,
`xtask/`, `spec/SPECIFICATION.md`, `CHANGELOG.md` and `.kb/`
(`_decomposition.md:363-371`) — and `RUNBOOK.md` is not on it. **Resolution: that list
is about which *source* seams a code change may cross, and its whole purpose is to
forbid `crates/happenstance-core/src/**`; `RUNBOOK.md` is the plan of record** (`CLAUDE.md`,
repository map) whose own exit criterion names its session log as this verdict's home,
whose phases 0–6 session logs are already written this way (`RUNBOOK.md:3223` is the
worked shape), and which is the only artefact that can act on §9's escalation. The
runbook edit is therefore admissible and **minimal**: one session-log entry citing the
long-form record by path, the one exit box this story discharges, and the `:524` row.
Every other phase-7 box belongs to its own story and stays as it is.

### 11. What is already routed here, so the log does not start empty and does not start complete

Two findings exist before implementation begins, and both were written *before* anyone
was tempted:

- **D-1 / VT-18**, from `_design.md:652-672`, restated at `command-loop/spec.md:419` and
  `adr-0020-fold-query-agreement/discover.md:30` as *"log, do not edit"*.
- **CF-36**, from `projection-clause-verdicts`' own AC-010: CF-36 names
  `cargo xtask spec-trace` as *"cross-referencing each case's level marker"*
  (`spec/SPECIFICATION.md:8297-8300`, `[FROZEN]`) and `xtask/src/spec_trace.rs` reads no
  level marker at all — *"recorded with its clause ID and routed… neither
  `xtask/src/spec_trace.rs` nor the frozen clause is edited to make it go away"*
  (`projection-clause-verdicts/spec.md:405`).

Six further stories declare routings into this log by name —
`domain-event-and-decision-model` (EC-008, `spec.md:459`),
`projection-trait-and-runner` (EC-010, `spec.md:526`), `misbehaving-testkit-stores`
(`spec.md:427`), `given-when-then-dsl` (`spec.md:430`), `worked-example-on-typed-layer`
(`spec.md:531`) and `polling-cost-measurement` (AC-008, `spec.md:343`). **Which defects
exist beyond the two named is unknowable at planning time by construction** — the log's
premise is that defects are found by *use*, and the use is M2 through M6
(`discover.md:54-56`). What is knowable, and is this story's obligation, is that every
one of those routings is **accounted for**: an entry, or an explicit "this story found
none". Silence from a story is indistinguishable from a finding that was dropped.

### 12. The persona-journey slice this realizes

Activity **A7** of the story map's backbone — *"Get it from the registry, and know what
it promises… and the record of what using the frozen contract revealed"*
(`_storymap.md:29`). The reader is **P1, the application author**, whose named fear is
*"being the one who discovers a contract defect in production, after they have already
built on it"* (`_decomposition.md:47`) — and this story is the artefact that makes the
alternative true: the defects were discovered by the library's own first consumer, in
the open, before the adapter phase. The second reader is the **adapter author**, six of
whom will pin `happenstance-core`, and for whom this log is the only place a surface
change's reasoning will ever have existed.

### 13. Not this story's to reopen

The four `CLAUDE.md` binding constraints; every `[FROZEN]` `ES-*` / `VT-*` / `CF-*`
clause body — naming a clause in the log is not amending it, and no story in this project
is licensed to amend one (`_storymap.md:152-154`); DT-2's resolution, which is
`_design.md`'s and human-approved; the PS-* verdicts, which are
`projection-clause-verdicts`'; the testkit's version number, the fifth `wasm32` step and
the release itself, which are M7's other two stories'.

## Integration contract

- **Archetype**: `capability`. The observable deliverable is a record a reader meets, not
  in-tree substrate a sibling compiles against — but it is delivered **mounted**, at the
  ingest path that turns it into corpus and in the plan of record that can act on it,
  never as a note in a backlog folder.
- **Slice / milestone**: **M7 `alpha-release`**. Slice-mates implemented in the same
  context and mounted as one surface: `edge-flavour-and-wasm-claim` (independent of this
  story) and `publish-0-2-0-alpha-1` (which lists this story in its own `depends_on` —
  `publish-0-2-0-alpha-1/discover.md:20`). Upstream `depends_on`:
  `worked-example-on-typed-layer` (M6, the substrate AC-013 is measured over) and
  `projection-trait-and-runner` (M5, the second place this project consumes the frozen
  contract in anger).
- **Mount point**: **`.kb/_intake/`** — the KB's real composition root and the *only*
  input path to `/redkiln:kb-ingest` (`.kb/_intake/README.md:3-5`). This story lands two
  staged documents there, `.kb/_intake/contract-defect-log-phase-7.md` and
  `.kb/_intake/happenstance-macros-verdict.md`, and a record staged anywhere else is
  unmounted by construction: never ingested, never an atom, never in the corpus. The
  wave's output — the atoms, their rows in `.kb/maps/decision-map.md` and
  `.kb/maps/open-questions-index.md` — is written by the human-invoked ingest run on its
  own worktree branch, not by this PR. The **second mount**, for the half `_intake` cannot
  hold, is `RUNBOOK.md`: phase 7's session log (`:4104`), the macros exit box
  (`:4078-4080`) and the decision-table row (`:524`).
- **Wires into**:
  - `references/evaluation/` — the durable long form and its README's own lifecycle
    taxonomy (`README.md:3-12`, `:40-48`). Created by this story:
    `phase-7-contract-defects.md` and `phase-7-macros-verdict.md`.
  - `examples/course-subscriptions/src/main.rs` — **read, never written.** AC-013's
    measurement substrate, as the M6 slice-mate leaves it, held plain by that story's
    NF-002.
  - `_design.md:652-672` and `:641` — D-1 and the `assert_domain_event` residual, the two
    findings that pre-date implementation.
  - `spec/SPECIFICATION.md` — **read, never written.** VT-18 (`:1371`, `:8544`) and CF-36
    (`:8297-8300`) are cited as the subjects of entries.
  - `.bklg/support/` — the destination for a finding with no clause ID; the hand-off is
    named in the log, and `redkiln new` is not run here.
  - This story's own backlog folder — `_ledger.md` (required: `.redkiln/config.yaml:67`)
    and the implementation report.
- **Renders surfaces**: **none.** This story renders none of `_design.md`'s six declared
  surface ids (`crate-root-rustdoc`, `crate-readme`, `first-program-doctest`,
  `worked-example-transcript`, `dsl-failure-message`, `compile-fail-diagnostic` —
  `_design.md:40-86`). Its relationship to the signed-off design runs the other way: it
  **measures** the surface `first-program-doctest` and the rewritten example present, and
  it collects the defect `_design.md` itself recorded while composing them. No item in the
  `## Items` block is added or changed.
- **Conformance rule(s)**: **none, and that is the correct answer.** Nothing here is
  adapter-observable: the deliverables are two records, classified *"Record, not a test"*
  in the testing brief (`_decomposition.md:790-791`). A rule asserting "a defect log
  exists" is the decorative rule `CLAUDE.md` forbids — no adapter could fail it.
  `crates/happenstance-testkit/src/suite.rs` is untouched, and no `CHANGELOG.md` entry is
  owed under CF-29 because no rule is added.
- **Clause(s)**: **discharges none, amends none.** VT-18 and CF-36 are named as the
  *subjects* of log entries; naming a clause is not moving its marker, and moving one
  would take a new ADR rather than an edit. `spec/SPECIFICATION.md` is deliberately
  outside this story's PR boundary so that this is true on the diff and not only in prose.
- **Advances DoD scenario**: initiative **DoD 12** — *"The clause ledger is audited at
  publish… and no clause is provisional with an empty falsifier"* (`initiative.md:393-396`)
  — whose *accuracy* half is what this story protects: a clause the first consumer found
  wrong and nobody recorded is a clause whose stated strength is false at the moment of
  publish. It is also a hard **precondition** of DoD 9/10/11, by way of
  `publish-0-2-0-alpha-1`'s `depends_on`. It discharges the project's own **DoD 8** (the
  macros verdict recorded either way) and **DoD 9** (the defect log exists and is
  non-speculative), and satisfies initiative **BR-01**'s second half — *"the defects that
  use discovered must be recorded"* (`initiative.md:284`).

## PR boundary

`redkiln verify --grain story` reads the first fenced block under this heading and fails
on any file changed outside it. **Read what is absent.** `crates/**` and `spec/**` are
both excluded, which means the primary mutant of Context pack §1 — absorbing D-1 with a
four-line `QueryItem::from_validated` — fails this story on the diff, by path, before
anyone has to notice what the edit meant. That is AC-A02 expressed in git rather than in
prose.

```
references/evaluation/**
.kb/_intake/**
RUNBOOK.md
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/defect-log-and-macros-verdict/**
```

**In this PR**

- `references/evaluation/phase-7-contract-defects.md` — the durable defect log: the entry
  shape, D-1, CF-36, every routing collected from M2–M6, and the accounting that shows
  which stories found none.
- `references/evaluation/phase-7-macros-verdict.md` — the measurement: the counting
  method, the line-range classification over the rewritten example, both altitudes'
  numbers, the verdict, and the escalation if it is *in*.
- `references/evaluation/README.md` — one row under *"Later additions, which are neither"*
  (`:40-48`) for each new document, so the directory's own taxonomy stays true.
- `.kb/_intake/contract-defect-log-phase-7.md` and
  `.kb/_intake/happenstance-macros-verdict.md` — the staged claims, with **proposed**
  frontmatter for the ingest run to author from and `source_paths` naming the long forms.
- `RUNBOOK.md` — the phase-7 session-log entry (`:4104`), the macros exit box
  (`:4078-4080`), and the `:524` decision-table row. Nothing else in the file.
- This story's own backlog folder — `_ledger.md` and the implementation report.

The implementer **may** also touch the composition-root/wiring files named in the
Integration contract to mount this slice — that is not scope drift.

**Explicitly not in this PR**

- `crates/**` — every one of them, and `crates/happenstance-core/src/**` most of all
  (AC-A02). A defect is logged and routed; it is not fixed here, in either direction.
- `spec/SPECIFICATION.md` and `spec/E2E-CASES.md` — no clause body, marker or census is
  touched. The PS-* verdicts are `projection-clause-verdicts`'.
- `xtask/**` — including `xtask/src/spec_trace.rs`, whose CF-36 gap is *recorded* here and
  fixed nowhere in this project.
- `.kb/decisions/**`, `.kb/open-questions/**`, `.kb/maps/**` — the ingest wave's, and it
  is human-invoked. This story does not run `/redkiln:kb-ingest`.
- `.bklg/support/**` — filing a support item is a `redkiln new` invocation and a handoff;
  the log names the finding and its destination.
- `examples/**` — read for the measurement, never edited. Shrinking the mapping to improve
  the ratio would answer AC-013 by concealment.
- `crates/happenstance-macros/` — the derive itself, if the verdict is *in*. A new
  workspace member is a scope change for the runbook (`_decomposition.md:428`).
- `CHANGELOG.md` — the alpha's section is `publish-0-2-0-alpha-1`'s.

**Merge DoD one-liner** — the defect log exists with every entry naming a clause ID, what
was attempted, what the contract did and its routing; every M2–M6 story's routing is
accounted for; both records are staged at `.kb/_intake/`; the macros verdict is recorded
in or out against a published counting method over the rewritten example; and the diff
touches no file under `crates/**` or `spec/**`.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| A durable contract-defect log exists, dated and pinned | `references/evaluation/phase-7-contract-defects.md`, carrying the date and the commit it was written against, under that directory's stated lifecycle — evidence rather than instructions, superseded rather than edited. `review-citation-drift.md` is the precedent for a byproduct log added later, *"recorded because the alternative was losing them"* | `references/evaluation/README.md:3-12`, `:40-48`; `project.md:243-244` |
| Every entry carries the same six fields, and one of them is a clause ID | id · clause ID and its maturity marker · what was attempted (with the call site) · what the contract did · why this is a defect and not a misuse · the routing. DoD 9's *non-speculative* bar is on the entry, not the document; an entry missing the clause ID or the routing is not an entry | `project.md:243-244`; `_decomposition.md:427`; `discover.md:40-42`, `:133-141` |
| D-1 is entry one and is written verbatim from the design | *"`happenstance-core` has no infallible `QueryItem` constructor for pre-validated inputs; every derived query therefore carries a `Result` that is unreachable for well-formed models."* Clause **VT-18**, which it partially contradicts. Note the honest form: the log records that the constructor may well be the right answer, and that the objection is to taking that decision without one | `_design.md:652-672`; `spec/SPECIFICATION.md:1371`, `:8544`; `crates/happenstance-core/src/query.rs:48-62` |
| The CF-36 finding routed here by the slice's upstream story is carried | CF-36 (`[FROZEN]`) names `cargo xtask spec-trace` as cross-referencing each case's level marker; `xtask/src/spec_trace.rs` reads no level marker. Recorded with its clause ID and routed. Neither the checker nor the clause is edited | `spec/SPECIFICATION.md:8297-8300`; `projection-clause-verdicts/spec.md:405`, `:452` |
| Every declared routing from M2–M6 is accounted for, including the empty ones | Eight stories declare a routing into this log by name. Each is reconciled: an entry, or an explicit record that the story found none. A story's silence is indistinguishable from a dropped finding, which is why absence is written down rather than inferred | `domain-event-and-decision-model/spec.md:459`; `projection-trait-and-runner/spec.md:526`; `misbehaving-testkit-stores/spec.md:427`; `given-when-then-dsl/spec.md:430`; `worked-example-on-typed-layer/spec.md:531`; `polling-cost-measurement/spec.md:343`; `command-loop/spec.md:419` |
| A finding with no clause ID is classified as a support item at the moment it is found | `support_initiative: support` is declared in config and `.bklg/support/` exists. The log records the finding, its classification and the handoff; `redkiln new` is not invoked by this story and `.bklg/support/**` is outside the boundary | `.redkiln/config.yaml:5`; `project.md:270`; `discover.md:59-62` |
| Nothing is fixed by a line edit, and the diff is what proves it | No file under `crates/**` and no file under `spec/**` changes. The PR-boundary fence excludes both, so `redkiln verify --grain story` rejects the absorption mutant by path rather than by review | `_decomposition.md:363-371`; `.redkiln/config.yaml:40`; `discover.md:90-131` |
| Both records are staged at the one path `/redkiln:kb-ingest` reads | `.kb/_intake/contract-defect-log-phase-7.md` and `.kb/_intake/happenstance-macros-verdict.md`, carrying **proposed** frontmatter for the wave to author from and `source_paths` naming the long-form records. Raw material, not atoms: `redkiln validate --kb` skips `_`-prefixed directories by design, so the staged files are not held to `KbFrontmatter` | `.kb/_intake/README.md:3-5`, `:21-27`; `_decomposition.md:643-651` |
| Ingest is a handoff; the story ends at "staged and ready" | The wave is human-invoked, runs on its own worktree branch, and clears `_intake` — which is why the long-form records exist separately. Filenames differ from M1's wave so that wave's audit trail is not overwritten | `.kb/_intake/README.md:13-19`; `_storymap.md:163-166`; `CLAUDE.md`, *Where the work lives* |
| The macros verdict is never an edit to ADR-0020 | `RUNBOOK.md:524`'s ADR column says 0020, and ADR-0020 is an accepted, immutable atom by the time this story runs; its own spec assigns AC-013's verdict to the project's closeout rather than to itself. The verdict is staged as its own document; whether that becomes a supersession is the ingest wave's adjudication | `RUNBOOK.md:524`; `adr-0020-fold-query-agreement/spec.md:323`; `CLAUDE.md`, *Where the work lives* |
| AC-013 is counted over the rewritten example, with a published classification | Substrate: `examples/course-subscriptions/src/main.rs` as M6 leaves it, held plain by that story's NF-002. Ceremony = what a derive would emit — `EVENT_TYPES`, `event_type()`, `tags()`, encode/decode plumbing, and the `assert_domain_event` residual. Domain = the enum's variants and payloads, model state, `apply`'s arms, `scope`, the decision body. Neither = `main`'s I/O, transcript printing, store setup, imports. Visible source lines, classified by line range so a reader can disagree with the judgement, not only the number | `RUNBOOK.md:524`; `_design.md:641`, `:1104-1111`; `worked-example-on-typed-layer/spec.md:435` |
| The threshold is 1:1 and mechanical; the judgement lives in the classification | *"More mapping boilerplate than domain logic"* means ratio > 1. Publishing the classification is what makes the verdict reproducible rather than asserted; the counting method is stated so a reader can re-run it and reach the same number | `RUNBOOK.md:524`; `discover.md:51-53` |
| Both altitudes are reported, and the design's prediction is confirmed or contradicted by name | The doctest's 11:26 (2.4:1) is recorded as the **prior**; the example's count is the **criterion**. The record says which is which and states explicitly whether the count confirms or contradicts `_design.md`'s prediction that the verdict is *"in"*. A contradiction is a successful outcome | `_design.md:1104-1111`; `_decomposition.md:307-312` |
| The verdict is written where the runbook says it is written | One session-log entry at `RUNBOOK.md:4104` citing the long-form record by path; the exit box at `:4078-4080` ticked because this story is what discharges it; the `:524` row moved off `open` to the verdict. Every other phase-7 box is left alone. Phase 4's log at `:3223` is the worked shape | `RUNBOOK.md:4078-4080`, `:4104`, `:524`, `:3223`; `CLAUDE.md`, repository map |
| An *in* verdict escalates; it does not build | A new workspace member is a scope change for the runbook to take, not an implementer. `crates/happenstance-macros/` is outside the boundary, and `publish-0-2-0-alpha-1` depends on the record, never on a crate | `_decomposition.md:428`; `_storymap.md:155-157`; `publish-0-2-0-alpha-1/discover.md:20` |
| No conformance rule, no clause edit, no changelog entry | Nothing here is adapter-observable; `crates/happenstance-testkit/src/suite.rs` is untouched, so CF-29 owes no changelog line. A rule asserting the log exists is the decorative rule this repository forbids | `_decomposition.md:790-791`; `CLAUDE.md`, *The rule that matters*; `xtask/src/lints.rs:507-575` |
| The gate is the boundary assertion, not a proof of the record | `cargo xtask affected --base main` (which for a no-package diff still runs the five file-reading lints and `spec-trace`) and `cargo xtask ci --fast`, this project's integration bar. Neither can observe whether the log is honest — that is the ledger's and the review's job, and `require_ledger: true` blocks `implement → report` without cited evidence per criterion | `.redkiln/config.yaml:30-40`, `:55`, `:67`; `_decomposition.md:790-791` |

## Data and migrations

**N/A — no schema, no store, no persisted state, no runtime artefact of any kind.** This
story adds no crate, no feature, no table and no serialized format; `MemoryEventStore` is
in-process and every durable adapter carries `publish = false` and is untouched
(`_decomposition.md:994-1001`).

Two *document* migrations are real, and both have stated rules rather than implicit ones.

**The staged documents are consumed, and that is the design.** *"A successful ingest
clears this directory… a file still sitting here after a run is a file that run did not
ingest"* (`.kb/_intake/README.md:13-19`). So the migration path for each staged file is
one-way and terminal: staged here → adjudicated by the human-invoked wave → an atom under
`.kb/`, with the staged source removed in the same commit and preserved in git history.
That is precisely why the long-form records under `references/evaluation/` are not
optional duplicates: a ~100-line atom cannot carry the call site, the attempted code and
the contract's actual response, and losing those loses a later reader's ability to judge
whether the defect was real. The staged files name the long forms in their proposed
`source_paths` so the eventual atoms cite them.

**`references/evaluation/` is append-and-supersede, never edit-in-place.** Its documents
are *"dated, pinned to a commit, cited by `file:line`… must be superseded rather than
edited"* (`README.md:3-12`). A defect log is therefore written once with its date and
commit; a later phase that disagrees with an entry writes a superseding document rather
than correcting this one — the same rule ADR-0006 applied to ADR-0005's body. The one
permitted in-place change is repointing a `file:line` citation at the file it already
named, *"because it changes no claim, only whether a reader can follow one"*
(`README.md:83-85`).

The one migration hazard worth naming: `references/evaluation/README.md` enumerates its
own contents by lifecycle, in three groups. Adding a file without adding its row leaves
the README asserting a taxonomy that no longer covers the directory — a silent, uncheckable
falsehood in the one document a reader opens to find out what the directory is. The row
lands in the same commit as the file.

## Acceptance criteria

Every criterion is framed from the reader who is failed when it is absent. Two
personas carry this story: **P1, the application author**, whose named fear is *"being
the one who discovers a contract defect in production, after they have already built
on it"* (`_decomposition.md:47`), and the **adapter author** — six of whom will pin
`happenstance-core` and for whom this log is the only place a surface change's
reasoning will ever have existed. Both meet activity **A7** of the backbone
(`_storymap.md:29`).

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | GIVEN an adapter author about to pin `happenstance-core`, WHEN they open `references/evaluation/` to find out what the contract's first consumer discovered, THEN `phase-7-contract-defects.md` and `phase-7-macros-verdict.md` are there, each carrying its date and the commit it was written against, and `references/evaluation/README.md` gains a row for each under *"Later additions, which are neither"* — so the directory's own lifecycle taxonomy still covers everything in it rather than silently going false | `test -f references/evaluation/phase-7-contract-defects.md` and `…/phase-7-macros-verdict.md`; `rg -n "phase-7-contract-defects\|phase-7-macros-verdict" references/evaluation/README.md` returns a row for each; both records' first ten lines carry a date and a SHA that `git cat-file -e` resolves. Reviewed at the closeout gate against `references/evaluation/README.md:3-12`, `:40-48` |
| **AC-002** | GIVEN P1 reading one entry to decide whether the defect touches the program they already shipped, WHEN they read **any** entry in the defect log, THEN it presents six labelled fields — id · clause ID with its maturity marker · what was attempted, with the call site as `path:line` · what the contract did instead · why this is a contract defect and not a misuse · the routing — as composed structure under its own headings, never a free-prose sentence; an entry missing the clause ID or the routing is not an entry and does not merge | Structural read of every entry: `rg -n "^### D-\|^\| *Clause\|^\*\*Routing" references/evaluation/phase-7-contract-defects.md` shows the same six field labels under every entry id, and the count of `Clause:` lines equals the count of entry headings. Human review at the closeout gate against DoD 9 (`project.md:243-244`) and the speculative-log mutant (`discover.md:133-141`) |
| **AC-003** | GIVEN a reader who wants to know what a *good* entry looks like before writing their own, WHEN they read entry one, THEN it is **D-1** verbatim from the signed-off design — *"`happenstance-core` has no infallible `QueryItem` constructor for pre-validated inputs; every derived query therefore carries a `Result` that is unreachable for well-formed models"* — naming clause **VT-18** and its `[FROZEN]` marker, citing the call site `crates/happenstance-core/src/query.rs:48-62`, routed to a decision record, and stating plainly that the constructor may well be the right answer and the objection is to taking that decision without a record | Text comparison of entry one against `_design.md:652-672`; `rg -n "VT-18" references/evaluation/phase-7-contract-defects.md`; the cited clause line resolves — `sed -n '1371p;8544p' spec/SPECIFICATION.md` still shows VT-18 and its `[FROZEN]` marker; `sed -n '48,62p' crates/happenstance-core/src/query.rs` still shows the fallible constructor and its `# Errors` doc |
| **AC-004** | GIVEN the upstream slice-mate that routed a finding here by name rather than fixing it, WHEN a reader looks for CF-36 in the log, THEN it is an entry with its clause ID, the contradiction stated concretely (CF-36 says `cargo xtask spec-trace` cross-references each case's level marker; `xtask/src/spec_trace.rs` reads no level marker), and its routing — and neither `xtask/src/spec_trace.rs` nor the frozen clause was edited to make the finding go away | `rg -n "CF-36" references/evaluation/phase-7-contract-defects.md` returns an entry; `git diff --name-only <base>...HEAD` contains neither `xtask/src/spec_trace.rs` nor `spec/SPECIFICATION.md`; cross-checked against `projection-clause-verdicts/spec.md:405`, `:452` |
| **AC-005** | GIVEN a reader who cannot tell a story that found nothing from a story whose finding was dropped, WHEN they read the log's reconciliation table, THEN every M2–M6 story that declared a routing into this log appears as a row with an explicit disposition — an entry id, or *"found none"* stated as a claim by that story — and no declared routing is absent | `rg -n "defect log\|phase-7-contract-defects" .bklg/from-contract-to-published-library/typed-layer-and-alpha-release/*/spec.md` enumerates the declaring stories; every slug it returns must appear as a row in the log's reconciliation table. Reviewed at the closeout gate against `_decomposition.md:427` |
| **AC-006** | GIVEN a bug found mid-implementation that bears on no clause, WHEN the implementer classifies it, THEN the classification is made at the moment of the finding, the log records the finding, its *support* classification and the fact that it was handed to the `support` initiative — and this PR neither invents a clause ID to promote it into an entry nor writes anything under `.bklg/support/**` | The log carries a *support-bound findings* section (possibly empty, and empty is stated rather than omitted); `git diff --name-only <base>...HEAD` contains no path under `.bklg/support/`; classification rule read against `.redkiln/config.yaml:5`, `project.md:270`, `discover.md:59-62` |
| **AC-007** | GIVEN the one commit that would destroy this story's reason to exist — absorbing a defect with a four-line convenience edit and going green — WHEN the story's gate runs, THEN it fails on the **paths in the diff**, before anyone has to notice what the edit meant: no file under `crates/**` and no file under `spec/**` is changed by this PR | `redkiln verify --grain story` reads this spec's PR-boundary fence and fails on any out-of-fence path; independently `git diff --name-only <base>...HEAD \| rg "^(crates\|spec)/"` must return nothing. The mutant is written out at `discover.md:90-131`; the rule is AC-A02 (`_decomposition.md:363-371`) |
| **AC-008** | GIVEN a reader who wants to disagree with the macros verdict rather than take it on trust, WHEN they read `phase-7-macros-verdict.md`, THEN they find the counting method stated, a classification published **line range by line range** over `examples/course-subscriptions/src/main.rs` as M6 leaves it — ceremony (what a derive would emit: `EVENT_TYPES`, `event_type()`, `tags()`, encode/decode plumbing, the `assert_domain_event` residual) versus domain (variants and payloads, model state, `apply`'s arms, `scope`, the decision body) versus neither (`main`'s I/O, transcript printing, store construction, imports) — the ranges partitioning the file's visible source lines with no overlap and no unclassified line, **both altitudes reported** (the doctest's 11:26 as the recorded prior, the example's count as the criterion, each labelled), and the verdict stated as *in* or *out* against the 1:1 threshold with the design's 2.4:1 prediction named as confirmed or contradicted | Re-derivation: the published ranges are summed and compared to `wc -l examples/course-subscriptions/src/main.rs` less the "neither" bucket — the three buckets must partition the file exactly at the pinned commit, and a second reader re-running the stated method reaches the same two integers. Cross-read against `RUNBOOK.md:524`, `_design.md:641`, `:1104-1111`, `worked-example-on-typed-layer/spec.md:435` |
| **AC-009** | GIVEN a maintainer reading the plan of record to find out whether phase 7's macros question was answered, WHEN they open `RUNBOOK.md`, THEN the verdict is where the runbook itself says it is written — one phase-7 session-log entry citing `references/evaluation/phase-7-macros-verdict.md` by path, the macros exit box ticked, and the decision-table row moved off `open` to the verdict — the edit touching nothing else in phase 7, and every `file:line` citation into `RUNBOOK.md` from elsewhere in the repository still resolving to the text it named | `rg -n "phase-7-macros-verdict" RUNBOOK.md` returns the session-log entry; the macros box (near `:4078-4080`) is `[x]` and the decision-table row (near `:524`) no longer reads `open`; `git diff --stat RUNBOOK.md` shows only those three hunks; citation-drift sweep — `rg -no "RUNBOOK\.md:[0-9]+" .kb spec references standards docs CLAUDE.md` and confirm each cited line still carries the text it was cited for (the failure class `references/evaluation/review-citation-drift.md` §1 records) |
| **AC-010** | GIVEN the human who will run `/redkiln:kb-ingest` next, WHEN they look at `.kb/_intake/`, THEN both records are staged there under names distinct from M1's wave — `contract-defect-log-phase-7.md` and `happenstance-macros-verdict.md` — each carrying **proposed** frontmatter for the wave to author from and `source_paths` naming its long-form record, and this PR has authored nothing under `.kb/decisions/**`, `.kb/open-questions/**` or `.kb/maps/**` and has run no ingest | `test -f .kb/_intake/contract-defect-log-phase-7.md` and `…/happenstance-macros-verdict.md`; `rg -n "source_paths" .kb/_intake/*.md` names both `references/evaluation/` records; `git diff --name-only <base>...HEAD \| rg "^\.kb/(decisions\|open-questions\|maps)/"` returns nothing; `redkiln validate --kb && redkiln doctor` stay clean (staged files are outside validation by design — `.kb/_intake/README.md:21-27`) |

**Coverage of the traced project ACs.** **AC-012** (*contract defects found by use are
recorded, with clause ID and routing, none fixed by editing a frozen clause*) is
carried by AC-001 through AC-007 and AC-010 — the record, its entry shape, its two
seeded entries, the accounting, the support split, the no-absorption fence and the
staging. **AC-013** (*the `happenstance-macros` question answered either way*) is
carried by AC-008, AC-009 and AC-010 — the measurement, where the verdict is written,
and the staging. Project DoD 8 is discharged by AC-008 + AC-009; DoD 9 by AC-002 +
AC-005.

## Interaction quality

This story renders **none** of `_design.md`'s six declared surfaces
(`_design.md:40-86`) — see the Integration contract. That does not make this section
vacuous, because the deliverables *are* read surfaces: two markdown documents whose
whole value is that a reader can find the finding, judge it, and follow it onward. The
signed-off design's three transience categories and its density-budget discipline
(`_design.md:810-836`, `:837-880`) are applied here in the medium this story actually
has, and the design's **standing** anti-patterns bind unchanged.

Every invariant below is an **AC row in the table above**. Nothing in this section is
an additional obligation; this is the index of which AC carries which invariant.

**State invariants.**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump.** The record lands beside the fourteen in the directory a reader already opens for evidence, not in a backlog folder they would have to know exists; the "what was attempted / what the contract did" detail is present *in the document*, never recoverable only from git history | AC-001, AC-010 | `test -f` on both long forms; the staged files cite them in `source_paths` rather than carrying the detail alone |
| **Non-occlusion.** Adding two documents must not falsify the one document a reader opens to learn what the directory is: `README.md` enumerates its contents by lifecycle in three groups, so the row lands in the same commit as the file. Symmetrically, the runbook edit occludes no other phase-7 exit box | AC-001, AC-009 | `rg` for both filenames in `references/evaluation/README.md`; `git diff --stat RUNBOOK.md` shows three hunks |
| **Preserved position — the citation analogue of preserved scroll.** Inserting into `RUNBOOK.md` shifts every line after the insertion point, and this repository has already paid for exactly that class once (`references/evaluation/review-citation-drift.md` §1: six citations that resolve, pass `spec-trace`, and point at the wrong line). Every existing `RUNBOOK.md:NNN` citation must still name the text it was cited for | AC-009 | The citation-drift sweep in AC-009's verification column |
| **Reversibility, stated rather than assumed.** `references/evaluation/` is append-and-supersede: a later phase that disagrees with an entry writes a superseding document rather than correcting this one. The staged `_intake` files are *designed* to be consumed and removed by the wave, which is precisely why the long forms are not duplicates | AC-001, AC-010 | Reviewed against `references/evaluation/README.md:3-12`, `:83-85`; `.kb/_intake/README.md:13-19` |
| **Reachability without search — the keyboard-reachability analogue.** Every claim a reader might dispute is followable in one hop: clause IDs cite `spec/SPECIFICATION.md:LINE`, call sites cite `path:line`, the classification cites line ranges in the example, and the staged files cite the long forms by path. No entry says *"see the specification"* | AC-003, AC-004, AC-008, AC-010 | Each cited path/line resolved during review; the same bar `_design.md`'s AC-U10 sets for a doc comment |

**Composition invariants.**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all.** An entry is *composed* — six labelled fields under their own headings — not bare markup and not a sentence of prose. This is the invariant the speculative-log mutant violates while still producing a committed file that DoD 9 reads as satisfied | AC-002 | Structural read: the six field labels appear under every entry id, and `Clause:` count equals entry count |
| **The verdict is a table, not an assertion.** A ratio with no published classification is the same failure one level up: it exists, it is checkable by nobody | AC-008 | The three buckets partition the file's visible source lines exactly at the pinned commit |
| **Placement.** Two homes on purpose — the durable long form under `references/evaluation/`, the staged claims at the one path `/redkiln:kb-ingest` reads — plus the plan of record for the verdict, because it is the only artefact that can act on an *in* escalation | AC-001, AC-009, AC-010 | The three `test -f`/`rg` checks in those rows |
| **Transience, in this medium's three categories.** *Persistent* = the long-form records and their README rows (they outlive the wave). *Revealed* = the reconciliation table and the support-bound section — present, one heading away, not competing with the entries. *Opened on demand* = the off-site citations into `spec/SPECIFICATION.md`, `.kb/` and the ADR long forms, which `_design.md:810-836` already places in that category. *Consumed* = `.kb/_intake/**`, the only category with a scheduled end | AC-001, AC-005, AC-006, AC-010 | Reviewed against `_design.md:810-836`; the staged/durable split is checked by both files existing |
| **Density budget, with the real numbers.** Markdown prose wraps at **80 columns**, matching the doc-comment budget the design set for the same audience (`_design.md:837-846`); tables may exceed it, because wrapping a table cell breaks the row. **Each entry ≤ 40 lines**, and what yields when it is exceeded is the commentary — never the clause ID, the call site or the routing, which are the entry's identity in exactly the way the transcript's position and event type are the log row's (`_design.md:868-871`). **The classification table is one row per contiguous line range**, and it is never summarised into a total that hides which lines were counted: the totals are derived from the rows, not asserted beside them | AC-002, AC-008 | Column check on prose lines; entry length read at review; the partition check re-derives the totals from the rows |
| **Hierarchy.** *Primary* in every entry: the clause ID and the routing — they are what makes it an entry at all, and they are what DoD 9 names. *Secondary:* what was attempted and what the contract did. *Recessive:* the judgement about whether the eventual fix is right, which is deliberately held subordinate — D-1 says the constructor may well be the correct answer, and that sentence must not read as the entry's conclusion | AC-002, AC-003 | Review against `project.md:243-244` and `_design.md:667-672` |
| **The named anti-patterns.** Three, each already written out with the commit that produces it: **(a)** absorbing a defect into `crates/**` instead of logging it (`discover.md:90-131`); **(b)** a speculative log — *"the contract's query construction is a bit awkward"* — with no clause ID and no routing (`discover.md:133-141`); **(c)** measuring AC-013 against `_design.md`'s doctest because it is already counted and conveniently produces a verdict, which the design forbids in the same paragraph that supplies the number (`discover.md:143-153`; `_design.md:1110-1111`) | (a) AC-007 · (b) AC-002 · (c) AC-008 | (a) is checked **by path on the diff**, which is the only one of the three a machine can catch; (b) and (c) are checked structurally and by re-derivation |
| **Standing constraints, not re-litigated.** `_design.md:1020-1024`'s standing list binds unchanged — no `#[async_trait]`, no `serde` in `happenstance-core`'s defaults, `read` returns the stream at the top level, generic code binds `EventStore`. This story writes no Rust, so it satisfies them by construction; NF-001 is what makes that a checked claim rather than an assumption | NF-001 | `git diff --name-only <base>...HEAD` contains no `.rs` file |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | A finding bears on no clause — an incidental bug in `happenstance`, the testkit or the example | Classify it **support** at the moment it is found, not later (`discover.md:59-62`). Record the finding, the classification and the hand-off in the log's support-bound section. **Never invent or stretch a clause ID** to promote it into an entry: a wrong clause citation is worse than no entry, because it routes a decision at a promise that was never at issue. Filing the item is a `redkiln new` handoff outside this PR |
| **EC-002** | A finding's clause is `[FROZEN]` and the fix is four lines away and obviously right | Log and route. The rightness of the fix is *not* the discriminator — AC-A02's rule is about who takes the decision and whether it leaves a record (`_decomposition.md:363-371`). Record the proposed fix inside the entry so the eventual ADR starts from it; do not apply it. AC-007 fails the PR by path if it is applied anyway |
| **EC-003** | A story that declared a routing into this log found nothing | Write *"found none"* as that story's claim in the reconciliation table, with the story slug. Silence is indistinguishable from a dropped finding, which is the whole reason AC-005 exists |
| **EC-004** | `examples/course-subscriptions/src/main.rs` is not yet the rewritten example when AC-008 is attempted — the M6 slice-mate has not landed | **Halt loudly** and report the missing dependency; do not substitute `_design.md`'s doctest, and do not measure the pre-rewrite example. The `depends_on` edge exists for exactly this (`_storymap.md:63`; `_decomposition.md:428`, `:791`). A verdict taken over the wrong text is the third mutant |
| **EC-005** | The rewritten example shrank its mapping with a local `macro_rules!`, a helper trait or a blanket impl, contrary to that story's NF-002 (`worked-example-on-typed-layer/spec.md:435`) | The measurement substrate is invalid and the count is not taken. Record it as a finding, route it to the M6 story, and re-measure once the mapping is plain. Counting the shrunken text answers AC-013 by concealment — the same failure as editing the example to improve the ratio, arrived at from the other side |
| **EC-006** | The count comes out at exactly 1:1 | The criterion is *"**more** mapping boilerplate than domain logic"* (`RUNBOOK.md:524`). Exactly 1.0 is therefore **out**, and the record says so explicitly with the numbers, rather than rounding toward the design's prediction |
| **EC-007** | A line genuinely resists classification — it is both ceremony and domain | Put it in a **contested** bucket, list it, and report the verdict at both extremes (all contested counted as domain; all counted as ceremony). If the two extremes give the same verdict, that verdict stands and the contested set is a footnote. If they disagree, the answer is **indeterminate at this artefact's size** and that is the recorded verdict, escalated to the runbook exactly as an *in* verdict would be — never resolved by picking the assignment that produces the tidier answer |
| **EC-008** | A cited `RUNBOOK.md` line anchor has drifted since planning (`:524`, `:4078-4080`, `:4104`) | Locate by **heading and text**, not by number — the decision-table row by its `Is happenstance-macros in scope for 0.1` cell, the exit box by its sentence, the session log by its `**Session log**` heading under phase 7 — and record the actual line in the spec's own citations when reporting. A citation that resolves to the wrong line is the failure class `review-citation-drift.md` §1 documents |
| **EC-009** | `.kb/_intake/` already holds files from a prior wave (M1's ADR-0020/ADR-0021 staging) | Do not overwrite and do not clear. Add the two documents under the distinct filenames AC-010 names. Clearing `_intake` is the ingest wave's act and only after a successful run (`.kb/_intake/README.md:13-19`) |
| **EC-010** | The macros verdict looks like it should amend ADR-0020, whose decision-table row (`RUNBOOK.md:524`) names it | Stage the claim; **never** edit the accepted atom. ADR-0020 is immutable by the time this story runs, and its own spec assigns AC-013's verdict elsewhere (`adr-0020-fold-query-agreement/spec.md:323`). Whether the wave produces a supersession is the wave's adjudication from the staged material |
| **EC-011** | The verdict is *in* and a `crates/happenstance-macros/` skeleton is tempting | Record the verdict and raise the escalation to the runbook. A new workspace member is a scope change for the plan of record, not an implementer (`_decomposition.md:428`); `crates/**` is outside the fence and `publish-0-2-0-alpha-1` depends on the record, never on a crate (`publish-0-2-0-alpha-1/discover.md:20`) |

## Non-functional

| id | Requirement | Why, and how it is checked |
| --- | --- | --- |
| **NF-001** | **Zero code change.** The diff contains no `.rs` file and no `Cargo.toml` | This is the strongest available statement of AC-012's *"none is fixed by editing a frozen clause"*, and it is mechanical: `git diff --name-only <base>...HEAD \| rg "\.rs$\|Cargo\.toml$"` returns nothing. It also discharges `_design.md`'s standing constraints by construction |
| **NF-002** | **The measurement is reproducible by a stranger.** A reader with the pinned commit, the stated method and the published ranges reaches the same two integers without asking a question | AC-013 is a *"Record, not a test"* (`_decomposition.md:791`); reproducibility is the only property that makes a record with no assertion behind it worth anything. The partition check is the mechanism |
| **NF-003** | **Prose wraps at 80 columns**; tables may exceed | Matches the doc-comment budget the design set for this same audience (`_design.md:837-846`), and keeps a `git diff` of a superseding revision readable line by line — which matters because these documents are superseded rather than edited |
| **NF-004** | **Both records are pinned.** Each carries its date and the commit SHA it was written against, in its first ten lines | The directory's own rule: *"dated, pinned to a commit, cited by `file:line`"* (`references/evaluation/README.md:3-12`). An unpinned defect log cannot be re-checked, because nobody knows which contract it was read against |
| **NF-005** | **The gate's step set is unchanged.** `cargo xtask ci --fast` runs the same steps before and after this PR, and is green | This story adds no conformance rule and no proof artefact (`_decomposition.md:790-791`). A new gate step here would be the decorative rule `CLAUDE.md` forbids — no adapter could fail it |
| **NF-006** | **The verdict record reads in evidence order**: the prediction, then the counting method, then the count, then the verdict | A record that states its conclusion first invites the reader to check the arithmetic against the conclusion rather than the other way round. `_design.md:1104-1111` put the prediction on the record precisely so it could be contradicted; the ordering is what keeps that a real possibility |
| **NF-007** | **No `.kb/` atom is hand-written.** Nothing this PR authors carries `KbFrontmatter` or lands outside `.kb/_intake/` | Hand-authoring atoms produces *"the directory layout of the process without the process"*, which is why the first attempt was reverted at `0269720` (`CLAUDE.md`, *Where the work lives*) |

## Implementation notes (non-prescriptive)

**Order that avoids the traps.** Take the M6 substrate check first: confirm
`examples/course-subscriptions/src/main.rs` is the rewritten file and that its mapping
is plain (EC-004, EC-005). If it is not, stop there — everything else in the story is
still doable, but AC-008 is not, and discovering that after writing the verdict is how
the doctest gets substituted.

**Finding the runbook's anchors.** Line numbers in this spec were taken at planning
time and the file moves. Locate the decision-table row by its `Is happenstance-macros
in scope for 0.1` cell, the exit box by the sentence *"The `happenstance-macros`
criterion is evaluated in the session log"*, and the session log by phase 7's
`**Session log**` heading. Phase 4's log is the worked shape for what a session-log
entry looks like in this file.

**The reconciliation sweep is one command and then judgement.** `rg` the sibling
stories' specs for their declared routings; the slugs it returns are the rows the table
must have. Seven are known at planning time —
`domain-event-and-decision-model`, `projection-trait-and-runner`,
`misbehaving-testkit-stores`, `given-when-then-dsl`, `worked-example-on-typed-layer`,
`polling-cost-measurement`, `command-loop` — plus `projection-clause-verdicts`, whose
CF-36 finding is already an entry. Re-run the sweep rather than copying that list: a
story may have added a routing after this spec was written.

**Counting, concretely.** Work from a `git rev-parse HEAD` pin. Walk the example
top to bottom and assign every line to exactly one of *ceremony*, *domain*, *neither*
or *contested*; blank lines and comment-only lines follow the block they document. Emit
one table row per contiguous range with its bucket and a one-clause reason, then derive
the totals by summing the rows. The reason column is what makes the classification
arguable — a reader who disagrees with one row can recompute without redoing the work.
Record the doctest's 11:26 beside it, labelled as the **prior**, and say which of the
two the criterion is read from.

**The staged files are raw material, deliberately.** `redkiln validate --kb` skips
`_`-prefixed directories, so the staged documents are not held to `KbFrontmatter` and
should not pretend to be atoms. Give each a **proposed** frontmatter block the wave can
author from — a proposed kind, title and `source_paths` naming its long-form record —
and let the wave adjudicate shape, supersession and links. Whether the macros verdict
becomes a supersession of ADR-0020 is the wave's call, not this story's (EC-010).

**What "recorded either way" costs in practice.** If the count contradicts
`_design.md`'s prediction, say so in those words and cite `_design.md:1104-1111`. A
contradiction is a successful outcome of this story: the design wrote the prediction
down as falsifiable so that this measurement could falsify it.

## Tests and CI (merge gate)

Both criteria are *"Record, not a test"* in the testing brief's own taxonomy
(`_decomposition.md:790-791`), so the honest statement of this table is: **the gate
proves the boundary and the baseline, and the ledger plus review prove the record.**
Nothing here pretends a compiled assertion is watching.

| tier | command / path | proves |
| --- | --- | --- |
| Static (boundary) | `redkiln verify --grain story` against this spec's PR-boundary fence | No file outside the fence changed — which is AC-007, and is the only one of the three named mutants a machine can catch. Also enforces `require_ledger: true` and `require_commit_provenance: true` (`.redkiln/config.yaml:67`, `:73`) |
| Static (lints + trace) | `cargo xtask affected --base main` — the story grain wired at `.redkiln/config.yaml:40` | The five file-reading lints and `spec-trace` run **unconditionally**, which is exactly why a diff that maps to no workspace package is still gated rather than trivially green |
| Static (citations) | `cargo xtask spec-trace` | `spec/SPECIFICATION.md`'s cross-references and markers are unmoved — this story cites clauses, it does not touch them |
| Static (no code) | `git diff --name-only <base>...HEAD` filtered for `.rs`/`Cargo.toml` | NF-001, and with it `_design.md`'s standing constraints by construction |
| Static (KB) | `redkiln validate --kb && redkiln doctor` | Frontmatter conformance and accepted-atom immutability are untouched; the staged `_intake` files are outside validation by design (`.kb/_intake/README.md:21-27`); doctor still reports exactly the six expected `template-drift` advisories |
| Integration bar | `cargo xtask ci --fast` — this project's bar (`.redkiln/config.yaml:55`) | The tree is green and the step set is unchanged (NF-005). Green here is a *baseline* claim, not evidence for any AC |
| Record (structural) | The reconciliation sweep and the partition check in AC-005 / AC-008 | Every declared routing has a disposition; the classification's three buckets partition the example exactly. Re-runnable by a reviewer, which is what makes them checks rather than assertions |
| Record (review) | The story's `_ledger.md` (`require_ledger: true`) and the closeout gate against `project.md:241-244` | DoD 8 and DoD 9. The ledger is the mechanism the runbook's *"a phase is done when its proof artefact exists, not when the gate is green"* has needed: each AC flipped only with cited evidence |
| Provenance | `redkiln record-links --sha` (`.redkiln/config.yaml:73`) | The work commit is recorded, so the record's pinned SHA and the PR agree |

**Not added, and deliberately.** No conformance rule
(`crates/happenstance-testkit/src/suite.rs` is untouched), so no `CHANGELOG.md` entry is
owed under CF-29 and the `changelog_names_every_rule` lint stays satisfied. A rule
asserting *"a defect log exists"* is the decorative rule `CLAUDE.md` forbids — name the
adapter that could fail it, and there is none.

## Risks and coupling (PR-scoped)

| Risk / coupling | Why it bites here | Handling |
| --- | --- | --- |
| **Nothing mechanical notices an empty story.** Every gate step stays green on a log with one entry and a verdict that reuses the prior | This is the project's least-defended criterion pair, and its most load-bearing (BR-01) | The fence catches the *absorption* mutant by path (AC-007); the ledger forces cited evidence per AC; AC-005's sweep and AC-008's partition check are re-runnable by a reviewer rather than trusted |
| **The M6 substrate may not be plain when AC-008 runs** — slice-mates land in one context, and a helper that shrinks the mapping is a natural thing to write | The measurement would be taken over a text engineered, however innocently, to produce *out* | EC-005 halts and routes; the obligation is already mirrored in the M6 story's NF-002 (`worked-example-on-typed-layer/spec.md:435`), so both sides know |
| **`RUNBOOK.md` is edited by three stories in this milestone** | Concurrent inserts drift each other's line anchors and produce citations that resolve to the wrong text — the exact failure `review-citation-drift.md` §1 records | Locate by heading text, not number (EC-008); keep the edit to three hunks; run the citation sweep in AC-009 last, after the slice's other runbook edits have landed |
| **`_intake` collides with M1's wave** | Overwriting a staged file destroys the audit trail of a wave that has already run | Distinct filenames, fixed in AC-010; EC-009 forbids clearing |
| **An *in* verdict invites a crate** | It is a one-directory change that would look like finishing the job, and it is a scope change the runbook owns | `crates/**` is outside the fence (AC-007 catches it); EC-011 states the escalation path; `publish-0-2-0-alpha-1` depends on the record, not a crate |
| **The staged files vanish if an ingest wave runs before this PR merges** | A wave on another branch clears `_intake`; a merge could then land two files into a directory the wave has emptied, or lose them to a conflict resolution | The long forms under `references/evaluation/` are the hedge and are never cleared — which is why the two-homes split is load-bearing rather than tidy. Re-stage rather than reconstruct if it happens |
| **AC-A02 does not list `RUNBOOK.md`** | A literal reading makes AC-009's edit out of bounds | Resolved in Context pack §10, on the record: AC-A02 governs which *source* seams a code change may cross; the runbook is the plan of record whose own exit criterion names its session log as this verdict's home. The edit is minimal and the fence declares it |
| **`projection-trait-and-runner` is upstream and may still be finding defects** | The log closes when both consumers have been written, not when the first has (`_storymap.md:63`, `:130-132`) | The `depends_on` edge is declared; AC-005's sweep is re-run at the end so a late routing is not missed |

## Dependencies

**Blocks on**

- **`worked-example-on-typed-layer`** (M6) — supplies the one artefact AC-008 is
  measured over, and is under a matching obligation from its own side (NF-002: the
  mapping written plainly, no local `macro_rules!`, no helper trait, no blanket impl)
  precisely because this story reads that file. Measuring anything else answers a
  different question (`_storymap.md:63`; `_decomposition.md:428`, `:791`).
- **`projection-trait-and-runner`** (M5) — the second place this project consumes the
  frozen contract in anger (streaming reads, inclusive `from`, the batch/checkpoint
  seam), so the defects it reveals belong in the same log. Its EC-010 already routes
  here by name (`projection-trait-and-runner/spec.md:526`).

**Unlocks**

- **`publish-0-2-0-alpha-1`** (M7 slice-mate) — which lists this story in its own
  `depends_on` (`publish-0-2-0-alpha-1/discover.md:20`) and needs the **record**, never
  a crate; it must not be blocked on a derive even if the verdict is *in*.
- **The next `/redkiln:kb-ingest` wave** — human-invoked, on its own worktree branch.
  This story ends at *staged and ready*; the atoms, their `.kb/maps/` rows and any
  supersession of ADR-0020 are the wave's.
- **Project closeout** — DoD 8 and DoD 9 (`project.md:241-244`), and through them
  initiative BR-01's second half and DoD 12's accuracy claim.

**Slice-mate, independent.** `edge-flavour-and-wasm-claim` shares M7 `alpha-release`
and is mounted in the same context, but neither story reads the other's output.

## Anchors (progressive disclosure)

Everything load-bearing is in the Context pack above; this table is what to open, and
when, for the depth a summary cannot carry. Link, do not paste.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `references/evaluation/README.md` | Defines the two lifecycles this directory runs, names the precedent for adding a byproduct log (`review-citation-drift.md`), and — because it enumerates its own contents — is the file that goes false if a document is added without its row | Before creating either record, and again in the same commit that adds them | AC-001 |
| `.kb/_intake/README.md` | States that `_intake` is the only input to `/redkiln:kb-ingest`, that a successful ingest clears it, and that `validate --kb` skips `_`-prefixed directories — the three facts that make the two-homes split necessary rather than tidy | Before writing either staged file | AC-010 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` | Carries defect candidate **D-1** verbatim with its clause and routing (`:652-672`), the `assert_domain_event` residual that *is* AC-013's measurement (`:641`), the falsifiable 2.4:1 prediction with its instruction to re-measure elsewhere (`:1104-1111`), and the transience/density discipline this story's documents inherit (`:810-880`) | Open `:652-672` before writing entry one; open `:1104-1111` before taking the count, not after | AC-003, AC-008 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/defect-log-and-macros-verdict/discover.md` | Writes out all three mutants as the commits that produce them — the absorption commit in full Rust (`:90-131`), the speculative log (`:133-141`), the wrong-text measurement (`:143-153`) — and states the clause-ID/support classification rule (`:59-62`) | At the start, and again whenever a defect looks cheap to fix | AC-002, AC-006, AC-007, AC-008 |
| `RUNBOOK.md` | The plan of record: the macros criterion and its `open` decision-table row (`:524`), the exit box this story discharges (`:4078-4080`), phase 7's empty session log (`:4104`), the worked shape of a session-log entry (`:3223`), and why phase 7 precedes phase 8 at all (`:256-258`) | Read `:256-258` for the why before starting; edit the three anchors last, after the slice's other runbook edits | AC-009 |
| `examples/course-subscriptions/src/main.rs` | The measurement substrate — **read, never written**. Its shape as M6 leaves it is what AC-013 is a verdict about | At AC-008, after confirming it is the rewritten file with a plain mapping | AC-008 |
| `spec/SPECIFICATION.md` | The clause bodies the entries name — VT-18 at `:1371` with its `[FROZEN]` marker at `:8544`, CF-36 at `:8297-8300`. **Read, never written**; naming a clause is not amending it | When writing each entry's clause field, to copy the clause text and its maturity marker accurately | AC-003, AC-004 |
| `crates/happenstance-core/src/query.rs` | D-1's call site: the fallible `QueryItem::new` at `:56` and the `# Errors` doc explaining the fallible-conversion bound at `:48-55` — the concrete *what was attempted / what the contract did* an entry must carry. **Read, never written** | When writing entry one's call-site field | AC-003 |
| `xtask/src/spec_trace.rs` | CF-36's counterparty: the checker that reads no level marker. Confirms the finding is still live and is the file that must **not** appear in the diff | When writing the CF-36 entry | AC-004 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/projection-clause-verdicts/spec.md` | Routes CF-36 here by name and states the no-edit condition on both the checker and the clause (`:405`, `:452`) | Before writing the CF-36 entry | AC-004, AC-005 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/worked-example-on-typed-layer/spec.md` | Carries the matching obligation from the other side — NF-002 holds the mapping plain *because* this story reads it (`:435`) — and its own routing into this log (`:531`) | Before the count, to verify the substrate is valid (EC-005) | AC-005, AC-008 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md` | AC-A02's diff-level rule (`:363-371`), the AC-012/AC-013 seam rows (`:427-428`), the ADR route and who invokes it (`:643-656`), the two-altitudes instruction (`:307-312`), and the testing brief's *"Record, not a test"* classification (`:790-791`) | Whenever a scope question arises about what this PR may touch | AC-005, AC-007, AC-008, AC-010 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/adr-0020-fold-query-agreement/spec.md` | States that AC-013's verdict is *"a later obligation owned by the project's closeout… not settled here"* (`:323`) — the reason the verdict is staged rather than written into an accepted atom | Before deciding where the verdict goes (EC-010) | AC-010 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md` | AC-012/AC-013 as the project states them (`:204-209`), DoD 8/9 (`:241-244`), and risk row 6 — *"absorbing one quietly would defeat the reason phase 7 sits before phase 8"* (`:270`) | At the start, and at the closeout gate | AC-001, AC-005, AC-008 |
| `references/evaluation/review-citation-drift.md` | The precedent both ways: the model for a byproduct log added later, and §1's record of citations that resolve, pass `spec-trace`, and point at the wrong line — the failure the runbook edit can cause | Before editing `RUNBOOK.md`, and when running the citation sweep | AC-001, AC-009 |
| `.redkiln/config.yaml` | The gate this story is actually held to: the story grain (`:40`), the project's integration bar (`:55`), `require_ledger` (`:67`), `require_commit_provenance` (`:73`), and `support_initiative` (`:5`) | Before claiming the gate proves anything, and when classifying a support-bound finding | AC-006, AC-007 |

## Clarifications resolved during spec

1. **The AC set is exactly the ten the front half enumerated.** AC-001 … AC-010, none
   added and none dropped; the ledger mirrors them one for one.
2. **AC-A02 versus the runbook edit** — resolved in Context pack §10 and carried into
   AC-009 and the risk table: AC-A02 governs which *source* seams a code change may
   cross, the runbook is the plan of record whose own exit criterion names its session
   log as this verdict's home, and the fence declares `RUNBOOK.md` explicitly so the
   admissibility is on the diff rather than in prose.
3. **This story renders no surface, and the interaction-quality section is still
   binding.** `_design.md`'s six surface ids are untouched; its transience categories,
   density discipline and standing constraints are applied to the medium this story
   does have — two read documents — and every applicable invariant is carried by an
   AC row, not a prose bullet.
4. **The threshold's edge case is decided.** *More* means strictly greater, so exactly
   1:1 is **out** (EC-006). Leaving it undecided would let the design's prediction
   settle it silently.
5. **Contested lines have a stated policy** (EC-007): report the verdict at both
   extremes; if they disagree, the recorded verdict is *indeterminate at this
   artefact's size*, escalated rather than resolved by choosing the tidier assignment.
   Without this, a single ambiguous block could decide AC-013.
6. **Verification is honest about what it proves.** Only AC-007 is machine-caught (by
   path). AC-005 and AC-008 are given *re-runnable* structural checks — the routing
   sweep and the bucket partition — so a reviewer can falsify them without redoing the
   work; the rest rest on the ledger's cited evidence and the closeout gate. No
   conformance rule is added, because no adapter could fail one.
7. **The eight-story routing list is a starting point, not the answer.** AC-005 is
   satisfied by re-running the sweep, not by reconciling this spec's list — a story may
   have declared a routing after this spec was written.
