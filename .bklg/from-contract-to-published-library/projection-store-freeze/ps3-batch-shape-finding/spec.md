---
item: HS-S0014
stage: spec
created: 2026-08-12T13:46:08.816Z
updated: 2026-08-12T13:46:08.816Z
template_sig: 87bbf1d0
rendered_sig: c0b758a0
---

# Spec — The PS-3 evidence written as a finding, not a verdict

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — BR/AC/DoD spine; **DoD 8** ("the freeze verdict is written") is the scenario this evidence feeds |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md:41` — the scope seam: this project supplies the PS-3 evidence, `publication-and-positioning` (HS-P0016) supplies the verdict |
| Project | `.bklg/from-contract-to-published-library/projection-store-freeze/project.md` — AC-015 at `:229-231`; the out-of-scope seam at `:129-131` |
| This spec | `.bklg/from-contract-to-published-library/projection-store-freeze/ps3-batch-shape-finding/spec.md` |
| Key briefs | `../_decomposition.md` — Architecture brief **AC-A04** (`:307-311`), **Note 3** "what this decision does not buy" (`:436-456`), **Note 10 item 2** (`:713-719`); Testing brief **AC-015 row** (`:785`) |
| Design | `../_design.md` — `surfaces: []`; this project renders no user-facing surface, and that determination is signed off (`:96-101`) |
| Story map / merge order | `../_storymap.md:66` (this story's one-line), `:112` (slice 6: `buffering-conformant-variant` → this story) |
| Discover (this story) | `discover.md` — the four questions this spec was told to answer, and the three named wrong implementations (`:35-52`, `:73-107`) |
| Roadmap pointer | `RUNBOOK.md:3924-3928` — phase 6's "decide the 0.1 exposure" checkbox, whose *"if the two batch shapes disagree"* is the literal question this finding answers |

## One-line PR slice

The PS-3 evidence written as a finding — *did the two batch shapes disagree, and where?* — including the "they agreed everywhere" outcome, which is itself the finding and not a silent success, handed to `publication-and-positioning` (HS-P0016) with no verdict on the `unstable-projection` exposure made here.

## Executive summary

This PR lands **one document and its two mounts**: a dated, commit-pinned evidence
file in the repository's evidence tree, indexed where a reader will find it and cited
from the clause it is evidence for.

Everything above this story already establishes *that* a finding is owed
(`../project.md:229-231`), *who receives it* (`../project.md:129-131`), and *that the
null result counts* (`../_decomposition.md:713-719`). The delta this spec adds is the
five things none of them fix, and each is a place the deliverable degrades quietly if
left to the implementer's judgement at writing time:

1. **A path and a form.** `references/evaluation/projection-batch-shape-evidence.md` —
   the evidence tree, whose whole definition is *"evidence kept for citation, binding
   nothing"* (`CLAUDE.md`, repository map) — and explicitly **not** a `.kb/decisions/`
   atom, which would make this project the author of a decision the charter assigns to
   another and would force HS-P0016 to *supersede* rather than decide.
2. **A disagreement vocabulary fixed before the run, not after.** Four operational
   categories, so "did they disagree" has an answer that cannot be selected to fit the
   conclusion (`discover.md:40-45`).
3. **A per-rule ledger covering the whole enumeration**, so no rule is silently absent
   from the evidence the way a rule must not be silently absent from a run
   (`../project.md:180-182`).
4. **Two mandatory sentences.** The PS-2 sentence (its bar is not met by anything this
   project built alone) and the explicit non-verdict. Both are guards against the
   failure `discover.md:75-87` names: a finding every sentence of which is true, that
   functions downstream as a verdict.
5. **Mounting.** The document is cited from PS-3 by `file:line` so `cargo xtask
   spec-trace` — a mandatory gate step — resolves it, and indexed in
   `references/evaluation/README.md` so a human meets it. An evidence document nothing
   indexes and nothing cites is not delivered; it is stored.

No test tier proves this story (Testing brief, `../_decomposition.md:785`). Its
evidence comes from the run its slice-mate produced; its instrument is review plus
`spec-trace`.

## Context pack

The load-bearing decisions this story must honour. Each is a decision already taken
somewhere above this story; none is re-opened here.

**1. Evidence, not a verdict — and the recipients are two, not one.** The charter puts
*"the PS-3 verdict on whether the port ships behind `unstable-projection` at publish"*
outside this project and names `publication-and-positioning` (HS-P0016) as its owner:
*"this project supplies the evidence; that project makes the call"*
(`../project.md:129-131`). Architecture brief AC-A04 restates it as a design obligation
rather than a courtesy — *"the PS-3 evidence hands `publication-and-positioning` a
finding, not a verdict"* (`../_decomposition.md:307-311`). The second recipient is
`ladybug-projection-store` (HS-P0015), which writes the freeze verdict itself and is the
reason the freeze is *"provisional in fact"* until then (`../project.md:301-307`). The
finding therefore addresses two readers making two different calls, and recommends
neither.

**2. The artefact is evidence-shaped, and that rules out the KB.** `.kb/decisions/`
is out for three reasons that compound: an accepted decision atom is **immutable**
(`CLAUDE.md`, *Where the work lives*), so HS-P0016 would have to supersede a decision
instead of making one; atoms are authored by `/redkiln:kb-ingest` from `.kb/_intake/`
and not by hand; and a decision atom naming the alternatives that lost (DR-06,
`../project.md:161-163`) is exactly the artefact this story is forbidden to produce,
because this story settles nothing. The home is `references/`, defined in the
repository map as *"evidence kept for citation, binding nothing"*, and specifically
`references/evaluation/`, whose README already carries the lifecycle this document
needs: **dated, pinned to a commit, immutable, superseded rather than edited**
(`references/evaluation/README.md:10-12`), with a precedent for a byproduct document
that is not one of the original fourteen (`:40-48`).

**3. Mounted means indexed *and* cited.** A library has no render tree, and the
Architecture brief's mount rule is that an item reachable at neither of its two mount
points *"is an item no adapter can name"* (`../_decomposition.md:340-344`). The prose
analogue is exact. So: the document is listed in `references/evaluation/README.md`'s
"Later additions, which are neither" section (the index a human reads), and PS-3's
clause body cites it by `file:line` (the pointer a downstream planner follows). The
second mount is machine-checked: `check_citations` in `xtask/src/spec_trace.rs:297-370`
fails the gate if the cited file does not exist, if the line is past its end, or if the
backticked subject the sentence attributes to it sits more than `ANCHOR_SLACK` = 12
lines away (`xtask/src/spec_trace.rs:374-380`). `references/adapter-shapes.md:220` is
already cited from the specification this way, so this is a used road, not a new one.

**4. The disagreement vocabulary is fixed here, before the run is read.** `discover.md:40-45`
deferred it to this spec *and said why*: **"fixing the vocabulary in advance is what
stops the answer being chosen to fit the conclusion."** Four categories, defined
operationally so two readers classify the same observation the same way:

- **D1 — per-shape handling.** A rule whose body, fixture wiring or assertion had to
  branch on which shape it was running against.
- **D2 — asymmetric declension.** A rule one shape ran and the other reported as
  `RuleOutcome::Skipped`, because a capability constant (fixture-declared, or
  `ProjectionProbe::READS_THROUGH_BATCH`) differs between the two.
- **D3 — assertion loosened.** A rule whose assertion was generalised *during*
  `buffering-conformant-variant` so the second shape could pass it — including any
  change from an exact expectation to a weaker one. This category is retrospective and
  is read off that story's diff, not off the run.
- **D4 — divergent observable behaviour.** Both shapes conformant, both green, but the
  observable sequence differs in a way a clause does not currently describe (for
  instance: when a write becomes visible to a read *through* the batch).

A rule that shows none of the four is **agreed**. A rule that could not be compared —
one shape's fixture never reached it — is **not comparable**, and that is a third
verdict, never silently folded into "agreed".

**5. "They agreed everywhere" is the finding, and it has two candidate explanations.**
The storymap puts this in the story's own one-line (`../_storymap.md:66`), and
Architecture brief Note 10 item 2 gives the two readings that must be weighed: *"either
the rules are shape-blind in a way that hides the axis, or the axis is not where §4.2
says it is"* (`../_decomposition.md:716-719`). *"We cannot tell from this evidence"* is
an admissible answer; silence is not, and neither is "no disagreements observed" as the
whole of it (`discover.md:89-98`).

**6. One sentence about PS-2, in one direction only.** PS-2 is `[FROZEN]`; its bar is
two **adapters** at opposite ends of the batch-shape axis, and its **Rejects** clause
names, verbatim, *"the schedule that freezes this port against `MemoryProjectionStore`
and an in-process rusqlite transaction"* as the monoculture to refuse
(`spec/SPECIFICATION.md:4760-4775`). Two testkit instruments do not clear it either, and
Architecture brief Note 3 already says so in writing (`../_decomposition.md:436-456`).
The finding states that PS-2's bar is not met by anything this project built alone, and
makes no claim about when it will be. That sentence is the guard against the wrong
implementation `discover.md:75-87` names — *"both batch shapes passed the whole suite;
the port is proven against the batch-shape axis"* — every word of which is true and
which functions downstream as a verdict.

**7. The evidence is read off one run, not off memory.** Testing brief AC-015 types this
story **E2E (process, derived from AC-004)**: *"not a new test: the finding is written
from what AC-004's Integration run actually showed"* (`../_decomposition.md:785`). That
run is the slice-mate's — `buffering-conformant-variant` (HS-S0013) — in which both
fixtures drive `projection_store_conformance!` to completion **inside one `cargo xtask
ci` invocation** (`../_decomposition.md:774`). The finding records that run's commit sha
and both fixture names, and every skip claim is taken from a `RuleOutcome` value
(`crates/happenstance-testkit/src/contract.rs:473-537`), never from a recollection of
stdout — the same reason AC-005 asserts on values rather than on printed lines
(`../project.md:194-197`).

**8. Nothing `[FROZEN]` is edited, and the clause disposition is not this story's.**
PS-2 is cited, not amended. PS-3 is `[PROVISIONAL]` (`spec/SPECIFICATION.md:4776-4790`),
and the only change to it here is one additive sentence carrying the `file:line`
citation. **Maturity markers, the `unstable-projection` gate and the rest of PS-1 –
PS-37's disposition belong to `unstable-projection-gate-and-clause-disposition`
(HS-S0016)**, which this story blocks (`story.md` frontmatter, `blocks: [HS-S0016]`) and
which is where AC-014 lands (`../_storymap.md:68`). Adding a maturity marker, deleting
the provisional block, or gating the module here is scope theft from a story that
depends on this one.

**9. The reader this is written for.** Not the adapter author — the UX brief's personas
are about the port and the suite's printed output (`../_decomposition.md:55-59`), and
this document is neither. The reader is **the planner of the next project**, standing at
PS-3 and deciding an exposure, and the bar `discover.md:35-39` sets is that they can do
it *"without reading this project's implementation history"*. That is the humane outcome
the acceptance criteria are framed against: they open one document and can make a call
they own, or see plainly that the evidence does not support making it yet.

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `capability` — a slice through to a reader-observable artefact (`story.md` frontmatter, `archetype: capability`) |
| **Slice / milestone** | `second-batch-shape-and-evidence`. Slice-mate: `buffering-conformant-variant` (HS-S0013), which is also this story's `depends_on` — the slice is implemented in one context and the variant lands first (`../_storymap.md:112`) |
| **Mount point** | `references/evaluation/README.md` — the index of the evidence tree, and the composition-root analogue for a prose deliverable: the "Later additions, which are neither" section (`:40-48`) is where a document that is dated, pinned and not one of the original fourteen is named. A document absent from it is invisible to every reader who does not already know its filename |
| **Second mount (gate-checked)** | `spec/SPECIFICATION.md` §4.1, the **PS-3** clause body (`:4776-4790`) — one additive sentence citing the finding by `file:line`, resolved by `cargo xtask spec-trace` (`xtask/src/spec_trace.rs:297-370`), which is a mandatory step of `cargo xtask ci` |
| **Wires into** | The slice-mate's two fixtures and the single gate run that exercised them (`buffering-conformant-variant`, HS-S0013 — the CF-5 buffering variant in `crates/happenstance-testkit/tests/` and `MemoryProjectionStore`); the projection rule enumeration `for_each_projection_store_rule!` in `crates/happenstance-testkit/src/registry.rs`, which is the authoritative list the per-rule ledger must cover; `RuleOutcome` and `Capability` (`crates/happenstance-testkit/src/contract.rs:372`, `:473-537`) as the source of every skip claim; `references/evaluation/README.md`'s stated lifecycle rules; and the receiving items `.bklg/from-contract-to-published-library/publication-and-positioning/` (HS-P0016) and `.bklg/from-contract-to-published-library/ladybug-projection-store/` (HS-P0015) |
| **Renders surfaces** | **None.** `../_design.md` records `surfaces: []` and the no-surface determination is signed off (`:48-50`, `:96-101`). This story adds no public API item and claims no `## Items` id, because that block is `N/A` for this project |
| **Conformance rule(s)** | **None, and it is not adapter-observable.** The finding is a claim about what one gate run showed, not about what a store must do; no adapter can pass or fail it. Its checks are `cargo xtask spec-trace` (the citation resolves) and the story review (the mandatory sentences are present). Stated explicitly because a story that names no rule is otherwise indistinguishable from one that forgot |
| **Clause(s)** | **PS-3** (`spec/SPECIFICATION.md:4776-4790`, `[PROVISIONAL]`) — gains an evidence citation; its maturity marker is **not** touched. **PS-2** (`:4760-4775`, `[FROZEN]`) — cited in one direction only (its bar is not met); not amended, so no new ADR is owed. Clause disposition and the `unstable-projection` gate remain HS-S0016's (AC-014) |
| **Advances DoD scenario** | Initiative **DoD 8** — *"the freeze verdict is written… with what it was checked against"* (`../initiative.md`, Definition of Done, item 8). This story does not write that verdict; it supplies the "what it was checked against" that HS-P0015 cannot reconstruct later. Secondarily **DoD 12** (the clause ledger audited at publish), because PS-3's disposition at publish is HS-P0016's call and this is its input. **DoD 7** is closed by the slice-mate, not here |

## PR boundary

```
references/evaluation/projection-batch-shape-evidence.md
references/evaluation/README.md
spec/SPECIFICATION.md
.bklg/from-contract-to-published-library/projection-store-freeze/ps3-batch-shape-finding/**
```

**In this PR**

- `references/evaluation/projection-batch-shape-evidence.md` — the finding: dated,
  pinned to the sha of the run it reports, both fixtures named, the four-category
  vocabulary stated before the ledger, the per-rule ledger, the null-result treatment,
  the PS-2 sentence and the explicit non-verdict.
- `references/evaluation/README.md` — one index entry for it, in the "Later additions,
  which are neither" section, following that section's existing shape (what it is, when
  it was written, what it is pinned to, and that the runbook was not derived from it).
- `spec/SPECIFICATION.md` — one additive sentence in PS-3's body carrying the
  `file:line` citation. No maturity marker, no `Rule:` / `Cases:` / `Rejects:` field
  edited, no other clause touched.
- This story's own backlog folder — `_ledger.md` and the implementation report.

**Explicitly not in this PR**

- **Any verdict.** No recommendation on whether `ProjectionStore` ships behind
  `unstable-projection` at 0.1, no statement that the port is or is not proven, no
  schedule for when PS-2's bar will be met (`../project.md:129-131`).
- **Any `.kb/` change** — no decision atom, no `.kb/reference/` atom, and no staging
  into `.kb/_intake/`. Durable-knowledge harvest is the closeout's
  (`closeout-and-durable-audience`, HS-P0019), and an atom written here would be the
  verdict wearing different frontmatter (`discover.md:100-107`).
- **Clause disposition** — maturity markers, the `unstable-projection` feature gate and
  the module's provisional block are HS-S0016's (`../_storymap.md:68`).
- **Any change to a rule, fixture, mutant or adapter.** If the comparison finds a rule
  wrong, that finding is recorded here and the fix belongs to the rule's own story, with
  the reason in the same change (`discover.md:130-133`).
- **Re-running or re-tuning the suite to produce a better story.** The evidence is what
  the slice-mate's run showed.

**Merge DoD (one line).** The finding exists at its fixed path, carries all four
mandatory elements (vocabulary, whole-enumeration ledger, PS-2 sentence, explicit
non-verdict), is indexed in `references/evaluation/README.md` and cited from PS-3, and
`cargo xtask ci` — including `spec-trace` — is green.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The finding exists at one fixed, citable path | `references/evaluation/projection-batch-shape-evidence.md`. Fixed rather than left to taste because two other files cite it by path and one of them is checked by the gate. It opens with the lifecycle statement its directory requires: the date, the commit sha of the run it reports, and that it is superseded rather than edited | `CLAUDE.md` repository map (`references/` = evidence, binding nothing); `references/evaluation/README.md:9-13`, `:40-48` |
| It names its evidence base before it reasons | Both fixture names (the CF-5 buffering replay-at-commit variant and `MemoryProjectionStore`, apply-on-write), the harness invocation, and the single `cargo xtask ci` run — one invocation, not two reconciled by hand — with its sha | `../_decomposition.md:774` (AC-004's Integration + E2E rows); `../_storymap.md:65` |
| The disagreement vocabulary is stated before any verdict is given | D1 per-shape handling · D2 asymmetric declension · D3 assertion loosened · D4 divergent observable behaviour; plus **agreed** and **not comparable**. Each defined operationally, in the document, ahead of the ledger — so a reader can re-classify the raw evidence and get the same answer | `discover.md:40-45` (the question, and the reason it is answered in advance) |
| Every rule in the enumeration appears in the ledger exactly once | The row set is taken from `for_each_projection_store_rule!`, not from the rules the author remembers running. One row per rule: rule name · verdict (agreed / D1–D4 / not comparable) · the observation it rests on. A rule absent from the ledger is the prose form of a rule absent from the run | `crates/happenstance-testkit/src/registry.rs`; `../project.md:180-182` (AC-001's "no rule silently absent" discipline) |
| Skip claims are read off `RuleOutcome`, not off stdout | A D2 classification cites the `RuleOutcome::Skipped` value and the capability constant it names — the fixture's own, or `ProjectionProbe::READS_THROUGH_BATCH` — because the printed line is suppressed by libtest for a passing test and reaches a human only through `--show-output` | `crates/happenstance-testkit/src/contract.rs:473-537`; `../project.md:194-197`; `../_decomposition.md:170-180` (AC-U09) |
| The null result is written as a result, with both explanations weighed | If the ledger is all-**agreed**, the finding says so explicitly and then addresses Note 10 item 2's two candidate readings — rules shape-blind to the axis, or the axis not where §4.2 places it — choosing one with its reasoning, or recording that this evidence cannot distinguish them. "No disagreements observed" as the whole of it does not discharge this | `../_decomposition.md:713-719`; `../_storymap.md:66`; `discover.md:89-98` |
| One sentence disposes of PS-2, in one direction | The finding states that PS-2's bar — two **adapters** at opposite ends of the batch-shape axis — is not met by anything this project built alone, quoting or citing the **Rejects** clause that names the two-instrument monoculture. It makes no prediction about when the bar will be met | `spec/SPECIFICATION.md:4760-4775`; `../_decomposition.md:436-456` (Note 3, "what this decision does not buy") |
| The non-verdict is explicit, and the recipients are named | A closing section states that no exposure verdict is made here, names HS-P0016 as the owner of the `unstable-projection` call and HS-P0015 as the owner of the freeze verdict, and contains no recommendation — not a preference, not a "suggests", not a leaning | `../project.md:129-131`, `:301-307`; `../_decomposition.md:307-311`; `discover.md:75-87` |
| The document is indexed where a human meets it | One entry in `references/evaluation/README.md`'s "Later additions, which are neither" section, matching that section's existing entry shape | `references/evaluation/README.md:40-48` |
| PS-3 cites it, and the gate resolves the citation | One additive sentence in PS-3's body citing `references/evaluation/projection-batch-shape-evidence.md:<line>`. The backticked subject the sentence attributes to the citation must appear within 12 lines of the cited line, or `spec-trace` reports the citation as pointing at the wrong place. `cargo xtask spec-trace` is run and green before the story is called done | `xtask/src/spec_trace.rs:297-380`; precedent at `spec/SPECIFICATION.md:840`, `:1208` (`references/adapter-shapes.md:220` cited the same way) |
| Nothing frozen is edited, and nothing is decided | PS-2 cited only; PS-3's maturity marker untouched; no `.kb/` atom written; no rule, fixture or adapter changed | `spec/SPECIFICATION.md:4776-4790`; `CLAUDE.md` ("Changing a `[FROZEN]` clause requires a new ADR, not an edit"); `discover.md:125-133` |

## Data and migrations

**N/A.** This story ships no code, no schema, no persisted runtime state and no
serialised format: its deliverable is one markdown document plus two references to it.
There is nothing to migrate and nothing whose representation can change under a reader.

One thing behaves enough like a migration to name, because getting it wrong is silent:
**the PS-3 citation is line-anchored.** `spec-trace` resolves
`references/evaluation/projection-batch-shape-evidence.md:<line>` against the file's
current contents and fails if the line is past its end or if the attributed subject has
drifted more than twelve lines from it (`xtask/src/spec_trace.rs:297-380`). So any later
edit to the finding is a two-file change — and `references/evaluation/README.md:75-85`
already states the discipline the evidence tree applies to exactly this case: a document
here is superseded rather than edited, and the one permitted in-place exception is
repointing a citation at the file it already named, *"because it changes no claim, only
whether a reader can follow one."* The finding therefore carries its own supersession
note in its header rather than inviting a future editor to revise it in place.

## Acceptance criteria

Framed from the reader this document exists for. Per the Context pack (§9) that is
**the planner of the next project** — HS-P0016 standing at PS-3 deciding an exposure,
and HS-P0015 writing the freeze verdict. That reader is the initiative's **P4, the
evaluator**, one step inside the boundary: someone who *"decide[s], in a bounded amount
of research time, whether `happenstance`'s claims… are real, without being able to run
the project's own internal test suite themselves"*
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:249-266`),
whose named fear is *"adopting a library on the strength of a claim… that turns out to
be true for only one storage shape"* (`:285-290`). Every criterion below is that
person's goal crossing the full stack — from the gate run that produced the evidence,
through the document, to the mount where they meet it.

Per the Testing brief, AC-015 is **E2E (process, derived from AC-004)**: *"not a new
test"* (`../_decomposition.md:785`). So the *Verification* column names, for each row,
either a mechanical check that exists (`spec-trace`, a git diff, a name-set comparison)
or the story review reading the document against this table. No row is verified by "the
author intended it".

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** a planner who has never read this project's implementation history, **WHEN** they follow a single path from the specification or the evidence index, **THEN** they land on `references/evaluation/projection-batch-shape-evidence.md`, whose first block states the date, the commit sha of the run it reports, and that it is superseded rather than edited — **AND** no `.kb/decisions/` atom, `.kb/reference/` atom or `.kb/_intake/` file was created by this story, so the planner is reading evidence they may still decide against, not a decision they would have to supersede. | Static: the file exists at exactly that path and opens with the three lifecycle facts (`references/evaluation/README.md:9-13` states the lifecycle it must match). `git status --porcelain .kb/` is empty for this PR, and `redkiln validate --kb` is green — the emptiness is the check, not the validation. Reviewed against `discover.md:100-107`. |
| **AC-002** | **GIVEN** a planner who cannot re-run the suite themselves, **WHEN** they ask *"what was this actually observed on?"*, **THEN** the finding names both fixtures by their real identifiers (the CF-5 buffering replay-at-commit variant in `crates/happenstance-testkit/tests/`, and apply-on-write `MemoryProjectionStore`), the harness invocation that drove them, and the **single** `cargo xtask ci` invocation with its sha — one run, not two `cargo test` runs reconciled by hand. | E2E (process, derived from AC-004): the named sha is the slice-mate's gate run (`../_decomposition.md:774` requires both fixtures inside one `cargo xtask ci`), and `git cat-file -e <sha>` resolves it on this branch. Review checks the fixture identifiers against the files `buffering-conformant-variant` (HS-S0013) actually landed. |
| **AC-003** | **GIVEN** two readers of the same evidence — the HS-P0016 planner and the HS-P0015 verdict author — **WHEN** each classifies the same observation, **THEN** they reach the same label, because the finding states the vocabulary operationally **before** the ledger: **D1** per-shape handling · **D2** asymmetric declension · **D3** assertion loosened · **D4** divergent observable behaviour, plus **agreed** and **not comparable** as distinct verdicts — and no fifth category is minted inside the ledger to accommodate a row. | Review against the Context pack §4 definitions, which are themselves fixed here rather than at writing time (`discover.md:40-45`: *"fixing the vocabulary in advance is what stops the answer being chosen to fit the conclusion"*). Mechanical: the set of verdict labels appearing in the ledger column is a subset of the six defined above. |
| **AC-004** | **GIVEN** a planner who must know the evidence is complete and not curated, **WHEN** they compare the finding's ledger against the suite's own rule enumeration, **THEN** every rule named in `for_each_projection_store_rule!` appears exactly once, and no row names a rule that is not in it — so a rule cannot be silently absent from the evidence the way a rule must not be silently absent from a run. | Static (mechanical, both directions): the rule-name set extracted from the ledger's rows equals the arm set of `for_each_projection_store_rule!` in `crates/happenstance-testkit/src/registry.rs` — the same two-direction discipline `no_orphan_rules` (`:412`) applies to the enumeration itself. Recorded in the implementation report as the comparison actually run, not as an assertion that it was. |
| **AC-005** | **GIVEN** a planner deciding how much a "skip" weakens the evidence, **WHEN** they read any **D2** row or any *not comparable* row, **THEN** it cites the `RuleOutcome::Skipped` value and the capability constant that produced it — a fixture-declared `Capability`, or `ProjectionProbe::READS_THROUGH_BATCH` — never a remembered or quoted stdout line, because libtest suppresses that line for a passing test unless `--show-output` is passed. | Review: each D2 / not-comparable row carries a `RuleOutcome`-grounded citation resolving into `crates/happenstance-testkit/src/contract.rs:473-537`. This mirrors AC-005's own discipline at project grain — *"asserted on `RuleOutcome` values rather than on stdout"* (`../project.md:194-197`). |
| **AC-006** | **GIVEN** a planner reading a ledger with no disagreements in it, **WHEN** they look for what that means, **THEN** the finding says so explicitly as a result and weighs Architecture brief Note 10 item 2's two candidate readings — *the rules are shape-blind in a way that hides the axis*, or *the axis is not where §4.2 says it is* — choosing one with its reasoning, or recording that this evidence cannot distinguish them. *"No disagreements observed"* as the whole of it does not discharge this, and neither does omitting the section because there was nothing to report. | Review against `../_decomposition.md:716-719` and `discover.md:89-98`. The check is presence-and-substance: a named section addressing the two readings exists whenever the ledger is all-**agreed**, and its conclusion is one of {reading A, reading B, *this evidence cannot distinguish them*}. |
| **AC-007** | **GIVEN** a planner who could otherwise inherit a conclusion as though it were evidence, **WHEN** they reach the end of the finding, **THEN** they meet two mandatory statements and no third: that **PS-2's bar — two *adapters* at opposite ends of the batch-shape axis — is not met by anything this project built alone**, citing PS-2's **Rejects** clause naming the two-instrument monoculture verbatim; and that **no exposure verdict is made here**, naming HS-P0016 as owner of the `unstable-projection` call and HS-P0015 as owner of the freeze verdict. The document contains no recommendation, preference, leaning or prediction about when PS-2's bar will be met. | Review, and it is the review's primary job: this is the guard against `discover.md:75-87`'s named wrong implementation — *"both batch shapes passed the whole suite; the port is proven against the batch-shape axis"* — every sentence of which is true and which functions downstream as a verdict. Both statements are cited to `spec/SPECIFICATION.md:4760-4775`, `../_decomposition.md:307-311,436-456` and `../project.md:129-131,301-307`. |
| **AC-008** | **GIVEN** a planner who does not already know this file's name, **WHEN** they arrive from either direction a reader actually travels — browsing `references/evaluation/README.md`, or following PS-3's clause body — **THEN** they find it: one index entry in the README's *"Later additions, which are neither"* section, and one additive sentence in PS-3 citing it by `file:line`. **AND** `cargo xtask ci` is green including `spec-trace`; PS-2's body is byte-identical; PS-3's `[PROVISIONAL]` marker, `Rule:`, `Cases:` and `Rejects:` fields are unchanged; no other clause is touched. | Static: `cargo xtask spec-trace` (a mandatory step of `cargo xtask ci`) resolves the citation — `check_citations` fails if the file is missing, the line is past the end, or the backticked subject sits more than `ANCHOR_SLACK` = 12 lines away (`xtask/src/spec_trace.rs:297-370`, `:391`). Static: `git diff spec/SPECIFICATION.md` shows exactly one added sentence inside PS-3's body and nothing inside PS-2's. Review: the README entry follows the shape of the section's existing entry (`references/evaluation/README.md:40-48`). |

**Coverage of the traced project AC.** Project **AC-015** — *"the PS-3 evidence is
recorded as a written finding — did the two batch shapes disagree? — and handed to
`publication-and-positioning`; no verdict on the `unstable-projection` exposure is made
here"* (`../project.md:229-231`) — has three arms, and all three are carried: *recorded
as a written finding* by AC-001 – AC-006, *handed to `publication-and-positioning`* by
AC-008 (the two mounts are the handoff; there is no other channel), and *no verdict
made here* by AC-007. No other project AC is traced, and none is partially covered.

## Interaction quality

This project renders **no user-facing surface**, and that determination is signed off
rather than assumed: `../_design.md` records `surfaces: []`, states *"No user-facing
surface"* with two independent confirmations, and was approved at the `/redkiln:plan`
design gate with *"Conditions: none"* (`:48-50`, `:96-101`). Its `## Items`,
`## Signatures`, `## Anti-patterns` and `## The doctest` blocks are all `N/A`. So there
is no composition, transience or density budget inherited from a signed-off design for
this story to render — and inventing one here would contradict a signed-off decision.

That is not a licence to skip the section. The **state** family of D6 invariants
transfers exactly, because this story does change what a reader can reach and what a
reader can no longer reach, and the deliverable has its own composition invariants in a
prose medium. Every invariant below is carried by an **AC-### row in the table above**,
never by a bullet here — a bullet in this section gets no ledger row and is never gated.

**State invariants, and the AC that carries each.**

| Invariant (D6) | Its form in this medium | Carried by | How it is verified |
| --- | --- | --- | --- |
| **In-place, not a context jump** | The specification edit is one *additive sentence inside PS-3's existing body*. The reader's position in §4.1 is preserved; they are not sent to a new clause, and no clause is renumbered | **AC-008** | `git diff spec/SPECIFICATION.md` — one added sentence, inside PS-3 |
| **Non-occlusion** | Nothing already reachable stops being reachable. PS-2's body is byte-identical, PS-3's maturity marker and its `Rule:` / `Cases:` / `Rejects:` fields are untouched, and the README gains an entry rather than losing or re-scoping one | **AC-008** | Same diff review, plus `cargo xtask spec-trace` proving no existing citation broke |
| **Reachability** (the keyboard-reachability analogue) | An artefact reachable from neither mount is unreachable in fact. The Architecture brief's own mount rule says an item reachable at neither of its mount points *"is an item no adapter can name"* (`../_decomposition.md:340-344`); the prose analogue is exact — indexed **and** cited, not one or the other | **AC-008**, and **AC-001** for the fixed path both mounts point at | `spec-trace` (machine) for the citation; review for the index entry |
| **Reversibility** | The evidence tree's rule is *supersede, never edit* (`references/evaluation/README.md:75-85`). The finding carries its own supersession note in its header, so a later correction is a new dated document, not a silent revision under a reader who already cited it | **AC-001** | Review of the header block against the lifecycle the directory states |
| **Preserved selection / no silent reflow** | The PS-3 citation is line-anchored with 12 lines of slack, so a later edit to the finding can silently point a reader at the wrong line — the exact defect `review-citation-drift.md` §1 records recurring here after phase 2 closed it. The supersede-not-edit header is what removes the hazard rather than mitigating it | **AC-001** (header), **AC-008** (`spec-trace` green) | `xtask/src/spec_trace.rs:297-370`, `:391` |

**Composition invariants.** The signed-off design declares none, so these are taken
from the *deliverable's own* stated shape in the Behavior and interfaces table above —
the prose analogue of composition, and the reason an "all the facts are present"
document can still fail this story. An unstyled render satisfies every data assertion;
a document that carries all eight mandatory elements in the wrong order, or buries the
non-verdict, satisfies every fact check and still misleads its reader.

| Invariant | Its form here | Carried by | How it is verified |
| --- | --- | --- | --- |
| **Presentation exists at all** | The finding is a composed document, not a dumped table: the evidence base is named before it is reasoned from, and the vocabulary is defined before it is used | **AC-002**, **AC-003** | Review — the ordering is the assertion |
| **Hierarchy** | Fixed reading order: lifecycle header → evidence base → vocabulary → per-rule ledger → null-result reading → PS-2 sentence → explicit non-verdict. The non-verdict is the *last* thing read, not a footnote, because it is what stops the rest being read as a recommendation | **AC-003** (vocabulary precedes ledger), **AC-007** (both closing statements present and terminal) | Review against this ordering |
| **Density budget, with its real numbers** | Exactly **six** verdict labels and no seventh (D1, D2, D3, D4, agreed, not comparable); exactly **one** ledger row per enumerated rule, no more and no fewer; exactly **one** additive sentence in PS-3; exactly **one** index entry in the README; exactly **two** mandatory closing statements and **zero** recommendations | **AC-003** (six labels), **AC-004** (one row per rule, both directions), **AC-008** (one sentence, one entry), **AC-007** (two statements, zero recommendations) | Mechanical for AC-004 and AC-008; review for AC-003 and AC-007 |
| **Transience** — what is persistent, what is revealed | The two mounts are *persistent chrome*: the README entry and the PS-3 sentence stay reachable for every future reader. The ledger's per-rule detail is *opened on demand* — a planner reads the null-result section and the non-verdict, and descends into the ledger only when a row disagrees | **AC-006** (the summary a reader can stop at), **AC-004** (the detail beneath it) | Review — a finding whose only content is the ledger fails AC-006 |
| **Named anti-patterns** | The three from `discover.md`, treated as blocking: (a) *"both batch shapes passed the whole suite; the port is proven against the batch-shape axis"* — true sentences functioning as a verdict (`:75-87`); (b) the null result recorded as silence or as *"no disagreements observed"* (`:89-98`); (c) the finding written as a `.kb/decisions/` atom, which passes `redkiln validate --kb` and makes this project the author of another project's decision (`:100-107`) | (a) **AC-007**, (b) **AC-006**, (c) **AC-001** | Each is the failure mode its AC's verification is pointed at; the review reads for the anti-pattern, not only for the criterion |

## Error conditions

| id | Condition | Required handling |
| --- | --- | --- |
| **EC-001** | `cargo xtask spec-trace` reports the PS-3 citation as unresolvable, or as pointing more than 12 lines from its attributed subject (`xtask/src/spec_trace.rs:354-360`, `:391`) | Fix the citation, never the checker and never by loosening the sentence. If the subject genuinely moved, repoint the citation at the file it already named — the one in-place edit `references/evaluation/README.md:75-85` permits, *"because it changes no claim, only whether a reader can follow one."* Do not delete the citation to make the gate green |
| **EC-002** | The slice-mate's gate run is not clean — a projection rule failed, or the two fixtures did not run inside one `cargo xtask ci` invocation | **Stop; do not write the finding.** There is no evidence base yet, and a finding assembled from two reconciled `cargo test` runs is not what `../_decomposition.md:774` describes. This is a blocked dependency on `buffering-conformant-variant` (HS-S0013), reported as such, not worked around |
| **EC-003** | The comparison finds a rule that is actually wrong — an assertion that admits a non-conformant store, or one that rejects a conformant shape | Record it in the finding as the disagreement it is (D1–D4 as classified), and **do not fix it here**. The fix belongs to the rule's own story with the reason in the same change (`CLAUDE.md`, *The rule that matters*; `discover.md:130-133`). Name the owning story so the pointer is not lost |
| **EC-004** | A **D3** classification is claimed but the loosening cannot be located in `buffering-conformant-variant`'s diff | Drop the claim or restate it as D1/D4 with its real evidence. D3 is retrospective and is read off the diff, not off the run (Context pack §4); a D3 row with no diff behind it is the "chosen to fit the conclusion" failure the vocabulary exists to prevent |
| **EC-005** | A rule cannot be compared — one shape's fixture never reached it, or the rule was skipped on both | Record it as **not comparable**, with the reason, and never fold it into **agreed**. If *not comparable* rows are numerous enough to weaken the whole comparison, say so in the null-result section rather than letting the ledger imply coverage it does not have |
| **EC-006** | The rule enumeration changes after the finding is written — a later story adds a projection rule, so the ledger no longer covers it | The finding is **pinned to a sha** and does not silently acquire rows. It remains accurate about the run it reports. If the gap matters to a downstream reader, that is a superseding document, not an edit (`references/evaluation/README.md:75-85`). AC-004's mechanical check is run against the enumeration **as of the pinned sha** |
| **EC-007** | A reviewer reads the finding as endorsing an exposure | AC-007 has failed, whatever the words say. Rewrite until a reader arriving cold cannot extract a recommendation; the test is the reader's inference, not the author's intent |

## Non-functional

| id | Requirement | Why it is here |
| --- | --- | --- |
| **NF-001** | **Self-sufficient in one read.** A planner can make the call they own — or see plainly that the evidence does not support making it yet — without opening this project's implementation history, its story items, or the testkit source | `discover.md:35-39` sets exactly this bar, and it is the P4 evaluator's *"bounded amount of research time"* one step inside the boundary (`…/personas-and-journeys.md:262-266`) |
| **NF-002** | **Immutable and pinned.** Dated, carrying the sha of the run it reports, superseded rather than edited — the lifecycle the evidence tree's first class already has | `references/evaluation/README.md:9-13`, `:75-85`. A mutable evidence document is the *"mutable speculation"* class (`:15-22`), which is explicitly the wrong shelf for this |
| **NF-003** | **Costs the gate nothing new.** No new CI job, no new gate step, no new tool. The citation is checked by `spec-trace`, which already runs in every `cargo xtask ci` | The pattern is deliberately borrowed rather than invented: `references/adapter-shapes.md` is already cited from `spec/SPECIFICATION.md` this way, so this is a used road |
| **NF-004** | **Zero recommendation verbs.** No *should*, *recommend*, *suggests*, *leans*, *is ready*, *is proven* applied to the exposure or the freeze. Statements of fact about one run only | AC-007 is unenforceable by any mechanical check; the only defence is a stated lexical discipline a reviewer can hold the text to |
| **NF-005** | **Two audiences, one document.** It addresses HS-P0016 (the `unstable-projection` call) and HS-P0015 (the freeze verdict) without being written to either's conclusion, and recommends to neither | `../project.md:129-131`, `:301-307`. A document written to one recipient's decision is that recipient's brief, not evidence |

## Implementation notes (non-prescriptive)

These are aids, not instructions. Nothing here is an acceptance criterion.

- **Write it after the slice-mate is green, in the same context.** The slice is
  implemented as one unit (`../_storymap.md:112`), and the observations this document
  reports are cheapest to capture while `buffering-conformant-variant`'s run is still in
  front of you. Capturing them into a scratch note during that story and composing the
  document here is fine; reconstructing them from memory a day later is the failure
  AC-005 exists to catch.
- **Derive the ledger's row set mechanically, not by recall.** The arm list of
  `for_each_projection_store_rule!` in `crates/happenstance-testkit/src/registry.rs` is
  the authority (the event-store enumeration at `:94` and its `no_orphan_rules`
  meta-test at `:412` are the shape to mirror). Extract the names, then fill verdicts —
  the opposite order lets a rule go missing.
- **`--show-output` is how a skip line becomes visible**, but the *claim* still cites
  the `RuleOutcome` (`crates/happenstance-testkit/src/contract.rs:473-537`). Use the
  printed line to find the rule; cite the value.
- **Place the PS-3 citation with `ANCHOR_SLACK` in mind.** `spec-trace` wants the
  backticked subject within 12 lines of the cited line (`xtask/src/spec_trace.rs:391`).
  Citing a stable, early line of the finding — its heading or its lifecycle header —
  is more robust than citing a line inside the ledger, which will be the longest and
  most edit-prone part of the document.
- **The README entry has a template already written**: `review-citation-drift.md`'s
  entry (`references/evaluation/README.md:40-48`) states what it is, when it was
  written, what it is pinned to, and that the runbook was not derived from it. Follow
  that shape; a new shape invites a third lifecycle into a directory that has exactly
  two and says so.
- **If the ledger is all-agreed, the document gets *longer*, not shorter.** That is the
  case AC-006 governs and the one `../_storymap.md:66` anticipates in the story's own
  one-line. A short document here is the signal that AC-006 was skipped.
- **Sequencing against HS-S0016.** This story blocks
  `unstable-projection-gate-and-clause-disposition` (`story.md` frontmatter,
  `blocks: [HS-S0016]`). Resist doing its work while you are already inside
  `SPECIFICATION.md` — a maturity marker changed here is scope theft from a story that
  depends on this one and is where AC-014 lands (`../_storymap.md:68`).

## Tests and CI (merge gate)

Tier vocabulary is the Testing brief's (`../_decomposition.md:760-767`): **Static** is a
check that does not execute the code under test, **E2E** is `cargo xtask ci` run whole
or a claim provable only by inspecting that run's combined output. AC-015 is typed
**E2E (process, derived from AC-004)** — *"not a new test"* (`:785`) — so this story
adds no test file, and its instruments are the gate steps that already exist plus the
story review.

| Tier | Command / path | Proves |
| --- | --- | --- |
| **Static** | `cargo xtask spec-trace` (also a mandatory step of `cargo xtask ci`; `xtask/src/spec_trace.rs:297-370`) | The PS-3 citation resolves: the finding exists at the cited path, the line is in range, and the attributed subject is within `ANCHOR_SLACK` = 12 lines — **AC-008** |
| **Static** | `git diff spec/SPECIFICATION.md`, reviewed line by line | Exactly one added sentence, inside PS-3's body; PS-2 byte-identical; no maturity marker, `Rule:`, `Cases:` or `Rejects:` field changed — **AC-008**, and the non-occlusion invariant |
| **Static** | `git status --porcelain .kb/` (empty) and `redkiln validate --kb` | This story wrote no KB atom and staged nothing into `.kb/_intake/`; the emptiness is the assertion, the validation is the guard that nothing slipped in — **AC-001** |
| **Static (mechanical)** | Name-set comparison: ledger rule names vs the arms of `for_each_projection_store_rule!` (`crates/happenstance-testkit/src/registry.rs`), both directions, at the pinned sha | Every enumerated rule appears exactly once and no row names a non-rule — **AC-004**. Recorded in the implementation report as the comparison run, per the `no_orphan_rules` precedent (`:412`) |
| **E2E (process, derived from AC-004)** | The slice-mate's `cargo xtask ci` run — one invocation, both fixtures — and its sha | The evidence base is a real run rather than a reconstruction; both fixture names and the sha in the finding match it — **AC-002**, and the `RuleOutcome`-grounded skip claims of **AC-005** |
| **E2E** | `cargo xtask ci` on this branch, green | Nothing regressed; the citation, the clause table and the docs all still hold with the edit in place — merge DoD |
| **Review (story)** | This spec's AC table + `_ledger.md`, read against `references/evaluation/projection-batch-shape-evidence.md` | The vocabulary precedes the ledger (**AC-003**), the null result is weighed against Note 10's two readings (**AC-006**), and both mandatory closing statements are present with zero recommendations (**AC-007**). These are the rows no tool can check, which is exactly why they are enumerated rather than left to judgement |

**Merge gate, one line.** `cargo xtask ci` green including `spec-trace`; the four Static
rows above run and recorded; the story review signed against AC-003, AC-006 and AC-007;
`_ledger.md` complete with cited evidence on all eight rows.

## Risks and coupling (PR-scoped)

- **The finding reads as a verdict.** The highest-probability failure and the one with
  the largest blast radius: HS-P0016 inherits a conclusion as evidence and settles PS-3
  on it, which is the decision the charter deliberately moved out of this project
  (`../project.md:129-131`). Nothing mechanical catches it — prose is not compiled and
  `spec-trace` checks citations, not inferences (`discover.md:82-84`). Mitigated by
  AC-007's two mandatory statements and NF-004's lexical discipline, and by the review
  reading for the anti-pattern rather than for the criterion.
- **The null result goes unwritten.** The likelier and cheaper failure: the shapes agree
  everywhere, there is nothing to say, and the section is omitted because an absence of
  news feels like an absence of content. AC-006 makes the all-agreed case the *longer*
  document, and the story's own one-line names the outcome so it cannot be missed
  (`../_storymap.md:66`).
- **Coupling to `buffering-conformant-variant` is total.** This story has no content
  until HS-S0013's run exists, both fixtures inside one gate invocation. Implemented in
  one context per the slice, so the risk is *ordering within the context* rather than
  cross-PR: write the variant, run the gate, then write the finding. EC-002 is the stop
  condition if the run is not clean.
- **Scope leak into HS-S0016.** Being inside `SPECIFICATION.md` with a maturity marker
  visible three lines from the edit is an invitation. The blocking relationship makes
  the leak expensive, not cheap: HS-S0016 owns AC-014 and would have to unpick it
  (`../_storymap.md:68`).
- **Citation drift after merge.** The PS-3 citation is line-anchored with 12 lines of
  slack, and `review-citation-drift.md` §1 records this exact class recurring in this
  repository after phase 2 closed it (`references/evaluation/README.md:53-58`).
  Mitigated structurally rather than by care: the finding is supersede-not-edit, so
  there is no routine reason for its lines to move.
- **The evidence tree grows a third lifecycle.** `references/evaluation/README.md`
  states two — immutable pinned evidence, and mutable speculation (`:9-22`) — and this
  document belongs to the first, in the *"Later additions, which are neither"* class.
  An index entry written in a new shape, or a document without its lifecycle header,
  quietly proposes a third. AC-001 and AC-008 hold it to the existing one.
- **No adapter can fail this.** By construction — the finding is a claim about what one
  run showed, not about what a store must do (Integration contract, *Conformance
  rule(s)*). Stated as a risk because a story that names no rule is otherwise
  indistinguishable from one that forgot to add one, and a reviewer looking for a rule
  should find this sentence instead of a gap.

## Dependencies

| Direction | Story | Why |
| --- | --- | --- |
| **Blocks on** | `buffering-conformant-variant` (HS-S0013) | Supplies the second batch shape and, decisively, *the run* — both fixtures driving `projection_store_conformance!` to completion inside one `cargo xtask ci` invocation (`../_decomposition.md:774`, `../_storymap.md:112`). Until it is green there is no evidence base and EC-002 applies. Same slice, same context, this story second |
| **Unlocks** | `unstable-projection-gate-and-clause-disposition` (HS-S0016) | Slice 8's first story lists `ps3-batch-shape-finding` among its dependencies (`../_storymap.md:112`, merge order item 8) and `story.md` frontmatter records `blocks: [HS-S0016]`. It owns AC-014 — maturity markers, the `unstable-projection` gate — and needs PS-3's evidence to exist and be cited before it disposes of the clause |
| **Unlocks (cross-project)** | `publication-and-positioning` → `projection-port-ship-shape` (HS-P0016) | That story settles PS-3 *"on `projection-store-freeze`'s and `ladybug-projection-store`'s reports rather than re-derivation"* (`.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md:57`). This finding is the first of those two reports; without it, "rather than re-derivation" has nothing to stand on |
| **Unlocks (cross-project)** | `ladybug-projection-store` → `freeze-verdict-document` (HS-P0015) | Writes the dated freeze verdict — *"implementation, rules run, PS clause ids, commit SHA, held or did not hold"* (`.bklg/from-contract-to-published-library/ladybug-projection-store/_storymap.md:48`). This finding supplies the *"what it was checked against"* for the two shapes that ran before Ladybug's, which that story cannot reconstruct later (initiative DoD 8) |
| **Does not block on** | `documented-extension-surface`, `ps-clause-pairing-sweep` | The DT-8 arm and the PS-clause pairing sweep change neither the run nor the vocabulary. Named here so their absence is not read as an omission |

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. The Context pack above is
self-sufficient for starting; open these at the moments named.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `references/evaluation/README.md` | The mount, and the lifecycle contract. Its two-lifecycle split (`:9-22`), the *"Later additions, which are neither"* entry shape (`:40-48`) and the supersede-never-edit rule with its single permitted exception (`:75-85`) are the specification for both the document's header and its index entry | Before writing the finding's header block, and again before writing the README entry | AC-001, AC-008 |
| `spec/SPECIFICATION.md` | Carries both clauses verbatim. **PS-2** at `:4760-4775` — the `[FROZEN]` bar of *two adapters*, and the **Rejects** clause naming the two-instrument monoculture, which AC-007's mandatory sentence must cite accurately. **PS-3** at `:4776-4790` and §4.1a's explanation at `:4796-4803` (*"resolved by PS-2 and by nothing of its own"*) — the clause that receives the citation | Read PS-2 before drafting the AC-007 sentence; read PS-3 before touching `SPECIFICATION.md` at all | AC-007, AC-008 |
| `xtask/src/spec_trace.rs` | The machine half of the second mount. `check_citations` at `:297-370` and `ANCHOR_SLACK = 12` at `:391` define exactly what a resolvable citation is — file exists, line in range, backticked subject within twelve lines. Guessing this costs a red gate | Immediately before writing the citation sentence, to choose which line of the finding to cite | AC-008 |
| `crates/happenstance-testkit/src/registry.rs` | The authoritative rule enumeration the ledger must cover. `for_each_projection_store_rule!` lands here beside `for_each_event_store_rule!` (`:94`), and `no_orphan_rules` (`:412`) is the two-direction discipline AC-004 mirrors in prose | When building the ledger's row set — first, before any verdict is filled in | AC-004 |
| `crates/happenstance-testkit/src/contract.rs` | `RuleOutcome` and its `skip_line` / `report` path at `:473-537`, plus `Capability`. Every D2 or *not comparable* claim cites a value from here rather than a remembered stdout line | When classifying any rule as skipped or not comparable | AC-005 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` | Three passages, and each settles something this story would otherwise re-decide: **AC-A04** at `:307-311` (finding, not verdict — an architectural obligation); **Note 3** at `:436-456` (*what this decision does not buy* — the PS-2 sentence's substance); **Note 10 item 2** at `:713-719` (the two candidate readings of a null result). The Testing brief's AC-015 row at `:785` types this story E2E-process | Note 3 before AC-007's sentence; Note 10 before AC-006's section; the testing row before planning any verification | AC-002, AC-006, AC-007 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/project.md` | The scope seam in the charter's own words — `:129-131` names HS-P0016 as owner of the PS-3 verdict, `:301-307` explains why the freeze is *provisional in fact* until HS-P0015 writes its verdict, `:229-231` is AC-015 itself. These are the two recipients AC-007 must name | When writing the explicit non-verdict, to name recipients from the charter rather than from memory | AC-007 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/ps3-batch-shape-finding/discover.md` | The three named wrong implementations at `:75-107`, in full, with why each survives a facts-only review; and `:35-52`, the four questions this spec answered, including why the vocabulary is fixed in advance | Before the story review — read the wrong implementations and check the draft against them, not only against the AC table | AC-001, AC-003, AC-006, AC-007 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | **P4, the evaluator** at `:249-290` — the bounded research window and the *"true for only one storage shape"* fear that the AC table is framed against; and the standing qualification at `:361-363` that no persona was directly observed | If a criterion's *"who is this for"* becomes unclear while drafting | AC-001, AC-006 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md` | Row `:57`, `projection-port-ship-shape` — the receiving story, which settles PS-3 *"on reports rather than re-derivation"*. Confirms what the downstream reader will actually do with this document | When judging whether the finding is sufficient for its recipient, before calling the story done | AC-007, AC-008 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/_storymap.md` | Row `:48`, `freeze-verdict-document` — the second recipient, and the shape of the verdict this evidence feeds (implementation, rules run, PS clause ids, sha, held or did not hold) | Same moment as the row above; the finding addresses two readers, not one | AC-002, AC-007 |
| `RUNBOOK.md` | `:3924-3928` — phase 6's *"decide the 0.1 exposure"* checkbox, whose *"if the two batch shapes disagree"* is the literal question this finding answers. The roadmap framing, not an instruction | Optional; read once at the start if the question's provenance is unclear | AC-006 |

## Clarifications resolved during spec

1. **The form and the path: a `references/evaluation/` document, not a KB atom, not a
   project artefact.** `discover.md:35-39` deferred this. Resolved to
   `references/evaluation/projection-batch-shape-evidence.md`. The KB is out for three
   compounding reasons (accepted atoms are immutable, so HS-P0016 would supersede rather
   than decide; atoms are authored by `/redkiln:kb-ingest` from `.kb/_intake/`, not by
   hand; and a decision atom naming the alternatives that lost is precisely what a story
   that settles nothing must not produce). A backlog artefact is out because HS-P0016
   must cite it without reading this project's implementation history. The path is fixed
   rather than left to taste because two other files cite it and one citation is
   gate-checked.
2. **The disagreement vocabulary: four categories plus two verdicts, fixed here.**
   `discover.md:40-45` deferred it *and said why*. D1 per-shape handling, D2 asymmetric
   declension, D3 assertion loosened (retrospective, read off the slice-mate's diff), D4
   divergent observable behaviour; **agreed** and **not comparable** are distinct
   verdicts and the second is never folded into the first. Six labels, no seventh —
   which is why it appears as a density number in the Interaction quality table.
3. **The null result is a required section, not a permitted one.** `discover.md:46-48`
   asked which of Note 10's two explanations applies if the shapes agreed everywhere.
   Resolved: the finding must *address* it, and *"this evidence cannot distinguish
   them"* is an admissible conclusion — silence and *"no disagreements observed"* are
   not. AC-006 carries it, and the all-agreed case makes the document longer.
4. **PS-2 gets one sentence, in one direction.** `discover.md:49-52` had already
   answered this; the spec makes it mandatory rather than advisable, per AC-A04
   (`../_decomposition.md:307-311`), and pins its substance to PS-2's own **Rejects**
   clause so the sentence cannot soften into a paraphrase.
5. **Mounting is two-sided, and one side is machine-checked.** Not deferred by
   `discover.md`, and the gap this spec closes: a written finding nobody can find is
   stored, not delivered. Indexed in `references/evaluation/README.md` (the human path)
   **and** cited from PS-3 by `file:line` (the machine path, checked by `spec-trace`).
   AC-008 carries both because either alone leaves the document unreachable from one of
   the two directions a reader actually travels.
6. **Skip claims are `RuleOutcome`-grounded.** Also not deferred, and also a gap: the
   D2 category is defined in terms of a declined capability, and libtest suppresses the
   printed skip line for a passing test. Without AC-005 the most citable evidence for
   the most consequential category would have been a recollection of stdout.
7. **The AC set is exactly the eight the first pass enumerated.** AC-001 – AC-008, none
   added and none dropped. They partition the eight mandatory elements of the deliverable
   (path and form · evidence base · vocabulary · whole-enumeration ledger ·
   `RuleOutcome`-grounded skips · null result weighed · PS-2 sentence and non-verdict ·
   both mounts with nothing frozen edited), and `_ledger.md` carries one row per id.
8. **Interaction quality has no signed-off composition to inherit, and says so.**
   `../_design.md` records `surfaces: []` with sign-off and *"Conditions: none"*
   (`:96-101`). Rather than skipping the section, the state invariants are transferred
   to their real form in this medium (in-place edit, non-occlusion, reachability via two
   mounts, supersede-not-edit reversibility) and the composition invariants are taken
   from the deliverable's own stated shape — every one bound to an AC row, none left as
   a prose bullet.
9. **Not resolved here, deliberately.** Whether `ProjectionStore` ships behind
   `unstable-projection` at 0.1 (HS-P0016's, via `projection-port-ship-shape`); whether
   the freeze held (HS-P0015's, via `freeze-verdict-document`); PS-3's maturity marker
   and the `unstable-projection` gate itself (HS-S0016's, AC-014); and whether any rule
   this comparison finds wrong should change (its own rule story's, with the reason in
   the same change).
