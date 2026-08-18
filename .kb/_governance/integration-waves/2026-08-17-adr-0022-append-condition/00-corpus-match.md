# Wave `2026-08-17-adr-0022-append-condition` — corpus match

What already exists in `.kb/`, what each incoming claim could have merged into, and the score
behind every disposition. Read this before `02-placement-and-adjudication.md`: the placements
there are only defensible because of what this file establishes about the corpus and about the
intake.

This is the corpus's **first six-file wave**, and the first in which two intake files, staged a
day apart by different stories, **claim the same ADR number**. Cross-file dedup is therefore not
the usual hunt for duplicated paragraphs — it is an allocation problem, and `02` Adjudication 1
settles it against the backlog rather than against either file's own assertion. The second
hardest job is the reverse of the last three waves': four of the six files arrive with a verdict
already taken and a long-form record behind it, and the fifth — the phase-7 defect log — arrives
asking for **five decision records this wave has no mandate to author**. Those five are routed to
`open-questions/`, three of them merged down to two atoms.

## The corpus, as verified

```
ls .kb/decisions      →  23 atoms (0001–0021, 0029, 0030), every one kind: decision
ls .kb/open-questions →  20 atoms, all authority_tier: note
ls .kb/reference      →   5 · playbooks 6 · maps 3 · concepts 1 · governance 1
ls .kb/product .kb/design → READMEs only; nothing routes there this wave
                         59 atoms + 8 layer READMEs, before this wave
ls references/adr     →  24 records: 0001–0022, 0029, 0030
```

| Layer | Atoms | Bearing on this wave |
| --- | --- | --- |
| `decisions/` | **23**, ADR-0001–0021 + ADR-0029/0030 | **Four are added** (ADR-0022, ADR-0031, ADR-0032, ADR-0033). **One receives a metadata flip and nothing else** (`kb-decision-0021`). **No body is edited anywhere** |
| `open-questions/` | 20 | **Four are created**; **one is resolved** (`ps-32-…`, metadata flip + dated section, body verbatim) |
| `reference/` | 5 | **Three are added**; **one is extended** (`port-traits-compiled-findings`, the corpus's only atom that declares itself a register) |
| `maps/` | 3 | All three inherit work, via `mapsImpact` rather than ops — `decision-map` four rows + a supersession lineage, `domain-map` a new domain and two re-pointed bullets, `open-questions-index` five bullets |
| `playbooks/` | 6 | `repair-frozen-clause` is the method three of the four new open questions describe. **Cited, not merged** — fourth wave running |
| `governance/` | 1 | `kb-governance-referent-not-reasoning-001` is what makes Op 7 a metadata flip and Op 4 *not* one. Cited, not touched |
| `concepts/` | 1 | `kb-concept-torn-read-append-boundary-001` is ADR-0022's nearest concept neighbour and gains a `related` edge. Not merged |

For authority purposes:

- **Twenty-three accepted decision atoms exist and the intake touches seven of them by name** —
  `kb-decision-0003` (the serde boundary, stated backwards by the atom that cites it),
  `kb-decision-0007` (the runner split, whose falsifier has fired), `kb-decision-0009` (the
  unbounded `Error` and its marker), `kb-decision-0012` (`append`'s borrowed batch and ES-17),
  `kb-decision-0019` (apply failure and PS-30's missing harness), `kb-decision-0020` (the fold/query
  derivation, and D-1), and `kb-decision-0021` (the codec tag). **Exactly one body-level defect is
  found, in `kb-decision-0021`, and it is repaired by supersession, not by edit.**
- **The wave performs one full supersession and one partial one, and spells them differently on
  purpose.** The rule that separates them is the corpus's own and is stated in `02` Adjudication 2:
  *flip when the atom's body carries the defect; leave it `accepted` when only the long-form record
  does.* `kb-decision-0021:141-144` carries its defect in the atom. `kb-decision-0007` does not —
  `kb-open-question-ps-32-…` established that in the last wave, in terms.
- **Five claims ask for a decision record and get an open question instead.** C1/C2 (merged),
  C3, C4 from the defect log. This is the ingest rule and the layer README's first bullet, and it
  is the same call wave 4 made on PS-32 — which this wave now gets to close, because the decision
  it was waiting for finally arrived.

## Provenance check

Every path the six files name was tested against this worktree. The check changed three
placements, which is why it is reported before the scores rather than after.

```
references/adr/0022-append-condition-strategy.md              ✓  35,777 bytes — the record exists
references/evaluation/phase-7-macros-verdict.md               ✓  13,869 bytes
references/evaluation/phase-7-contract-defects.md             ✓  19,330 bytes
references/adr/0021-payload-evolution-and-codec-tag.md        ✓  499 lines, appendix at :458
references/adr/0007-projection-runner-decodes.md              ✓  165 lines
experiments/append-condition/{run.sh,src,tests,results}       ✓  outside the workspace, as claimed
crates/happenstance/tests/projection_clauses.rs               ✓  the executed count
crates/happenstance-core/src/projection.rs:152-155            ✓  :152-154 is the prose exactly
xtask/src/spec_trace.rs:699, :1750-1755                       ✓  and it says something the intake does not
.bklg/from-contract-to-published-library/_implementation.md:704  ✓  allocates ADR-0031 by name
.bklg/from-contract-to-published-library/_plan.md:169         ✓  reserves ADR-0023
references/adr/0031-*, 0032-*, 0033-*                         ✗  none exists. Three atoms with no record
```

Four things the check turned up that no intake file says, and each changes a placement:

1. **`.bklg/…/_implementation.md:704` allocates ADR-0031 to the runner collapse by filename** —
   *"ADR-0031 is staged intake only (N-2) — `.kb/_intake/0032-adr-0031-the-runner-collapses-upward.md`."*
   That is the tiebreak for the number collision, and it points the opposite way from staging order.
   `02`, Adjudication 1.
2. **`_plan.md:169` reserves ADR-0023** for the Cloudflare project's `adr-0023-and-atom-resolutions`.
   So 0023 is not free, and the two unnumbered decisions in this wave append above 0031 rather than
   backfilling the 0023–0028 gap.
3. **`has_suite` already admits `PS-`** (`xtask/src/spec_trace.rs:1750-1755`), which reads as a flat
   contradiction of defect C4 — until `:699` is read beside it. The loop skips on
   `c.schedules_new || !has_suite(&c.id)`, and `schedules_new` is set by a bare `†` in the clause's
   `Rule:` line (`:1630-1634`). So C4's *conclusion* is live and its *stated mechanism* is the wrong
   half of a two-term guard. The open question this wave writes carries the corrected mechanism, and
   **extends** `kb-reference-spec-trace-has-suite-001` rather than contradicting it. `02`,
   Adjudication 6.
4. **Three of the four new decision atoms have no `references/adr/` record.** ADR-0022 has one;
   ADR-0031, ADR-0032 and ADR-0033 do not. ADR-0033's evidence is in `references/evaluation/`
   instead. `02`, Adjudication 8 states what follows and what does not.

## Scoring method

Unchanged from all four previous waves — a threshold that moves between waves is not a threshold.
A 0–100 judgement of **subject identity**: would a reader looking for one claim expect to find the
other in the same atom.

- **≥ 80 — same knowledge.** Collapse into one atom. Never two.
- **50–79 — same subject, different method or different settling condition.** Two atoms, linked,
  and the reason written down.
- **< 50 — related.** A `related` link and nothing else.

Four rules carried forward and two added:

- **A README is not a merge target** (wave 1). Exercised twice and **obeyed both times**: the
  extract pass proposed merging `0033`'s measurement-split rationale into `.kb/reference/README.md`
  and three separate "what the wave must not do" claims into `.kb/decisions/README.md`. Those
  READMEs are the *authorities being obeyed*; absorbing a claim into one would rewrite a layer
  contract to record a wave that complied with it.
- **A supersession chain is never a merge** (wave 2). Exercised twice — Ops 4/5 and Ops 6/7.
- **A decision naming a gap is not the same knowledge as the question that owns the gap** (wave 3).
  Exercised on D-1: `kb-decision-0020` logs it as a defect candidate *and names a decision record as
  its route*. That is a decision naming a gap; the question that owns it is Op 8.
- **An accepted decision atom is not a merge target, however high it scores** (wave 4). Exercised
  five times: ADR-0003, ADR-0009, ADR-0012, ADR-0019 and ADR-0020 all score 65–95 against an
  incoming claim and all produce **no operation on themselves**.
- **New this wave: an ADR number is allocated by the artefact that already carries it, not by the
  file that asks for it.** A long-form record fixes the number (ADR-0022). Failing that, the
  backlog's own allocation fixes it (ADR-0031). Failing both, the number is highest-taken + 1
  (ADR-0032, ADR-0033). Two intake files asked for 0031 and only one of them had a repo artefact
  behind the ask.
- **New this wave: a full flip and a partial supersession are told apart by where the defect
  physically is.** Not by how much of the decision changes — that test is unfalsifiable in
  practice, and it is what would have produced two identical spellings for two different
  situations. `02`, Adjudication 2.

## Merge candidates against the existing corpus

| Existing atom | Incoming claim | Score | Disposition |
| --- | --- | --- | --- |
| `kb-decision-0021` | `0031`'s C1/C2/C3 — the ADR-0003 attribution in its body inverts the constraint it cites | **95** | **supersede + flip** — Ops 6 and 7. The defect is in the atom's own body (`:141-144`), which is what makes the flip mandatory rather than optional |
| `kb-decision-0003` | `0031`'s C3 — "state the boundary in the right direction, once" | 60 | **link only** (`related` on Op 6). ADR-0003 is correct as written; what was wrong was a citation *of* it. Merging the correction into ADR-0003 would edit an accepted body to fix someone else's misreading |
| `kb-decision-0007` | `0032`'s C2/C3/C4 — the falsifier fired, the pump was never written, collapse upward | **92** | **partial supersession, and no edit at all** — Op 4. `supersedes: null`, `depends_on: [kb-decision-0007]`, exactly as `kb-decision-0007` itself records its partial reversal of `kb-decision-0006`. The decision-map's *Reading the partial-supersession chain* section states this convention in terms |
| `kb-open-question-ps-32-adr-0007-correction-owed-001` | `0032`'s C7 — the corrected Context, carried by the superseding atom | **100** | **merge_existing (resolve)** — Op 5. This claim *is* the answer the atom's own *What a future wave should carry* section asked for, and it answers both halves, including the pump question in the negative |
| `kb-decision-0012` | `0033`'s C1 (ES-17 not lifted) and C4 (the measurement that would lift it is unscheduled) | **85** on subject, **0 actionable** | **no operation on ADR-0012.** Its falsifier item 1 is quoted, not amended; the intake forbids the edit and so does the layer. The unowned obligation becomes Op 2 |
| `kb-open-question-cf-40-ownership-001` | `0033`'s CF-40 non-verdict | 70 | **link only.** ADR-0022 §14 cites it and leaves it open; the question's own *What forces it* already names phase 8's first adapter, so it is being forced exactly as written and needs no amendment to say so |
| `kb-concept-torn-read-append-boundary-001` | ADR-0022's `max(position)` guard inside `BEGIN IMMEDIATE` | 55 | **link only.** The concept explains why a boundary is a boundary; ADR-0022 picks the SQL that enforces one. Same subject, different job |
| `kb-reference-position-visibility-experiment-2026-08` | `0034` — the whole atom, by shape | 32 | **precedent, not a merge target** — and the intake says so itself. Also barred by the reference README's dating rule: that atom is a snapshot of a different experiment |
| `kb-decision-0020` | defect C1/C2 — D-1, which ADR-0020's own body already logs | **90** on subject, **0 actionable** | **no operation on ADR-0020.** It logs D-1 *and names "a decision record, not a line edit" as its route*; wave 3's rule makes the question a separate atom — Op 8 |
| `kb-decision-0020` | `happenstance-macros-verdict` claim-1 — AC-013 settles "out", against ADR-0020's 2.4:1 prediction of "in" | 72 | **create_new, not supersede** — Op 10. ADR-0020's Consequences says *"the record itself asserts no must about the derive"*: the prediction was published as falsifiable and has been falsified, which is the prediction working. The admitted set of implementations is unchanged, so `README.md:20-24`'s mechanical test says this is not even a repair of ADR-0020 |
| `kb-decision-0015` | defect C2 — `Tags` has no total path | 65 | **link only.** ADR-0015 owns validated identifiers and byte equality; the total-path gap is a *consequence* nobody decided, which is the open-questions layer's subject |
| `kb-decision-0009` | N1 — the marker trait has now been exercised by a real consumer | **90** on subject, **0 actionable** | **no operation on ADR-0009.** The intake's own words: *"at most an amendment … never a new decision"* — and an accepted decision takes no amendment. The finding lands in the register instead, Op 14 |
| `kb-reference-port-traits-compiled-findings-001` | N1 | **82** | **merge_existing** — Op 14. This is the corpus's only atom that declares itself a *register* — *"cite it by id rather than re-deriving the compile"* — and N1 is a `Send`-bound finding about the same two ports, reached the same way |
| `kb-reference-port-traits-compiled-findings-001` | defect C5 — `&mut P` costs N reads for N projections | 45 | **link only.** The register's every finding is about the two *flavours* — coherence, `Send`, erasure. C5 is about the runner's fan-out arity. Adjacent, not the same register |
| `kb-decision-0019` | defect C5 | **80** on subject, **0 actionable** | **no operation.** ADR-0019 already names PS-30's falsifier as *"whether the poll cost of N independent reads is real, a benchmark this workspace has no harness for."* C5 is the API-level half of exactly that, and the atom that states the question cannot absorb its own evidence |
| `kb-open-question-projection-batch-no-apply-001` | defect C5's requested destination | — | **unavailable.** `status: superseded` since 2026-08-13. The intake asked for the evidence to be attached to "whichever open question owns the tail-seam decision"; there is no live one, which is why Op 13 exists and why the gap is also carried in `unresolved` |
| `kb-reference-spec-trace-has-suite-001` | defect C4 — no `PS` rule name is resolved | 68 | **link only, and a correction inherited.** Same instrument, later moment, and the reference README's dating rule forbids appending a 2026-08-17 fact to a 2026-08-15 snapshot — the same refusal wave 4 made in creating this very atom. Op 12 |
| `kb-open-question-es-6-unwritable-rule-001` | defect C3/C4 — a frozen clause naming a check nothing performs | 58 | **link only.** ES-6 names an unwritable *rule*; CF-36 and CF-38 name checks the tool does not perform. Same shape, three different clauses and three different fixes |
| `kb-open-question-provisional-falsifiers-001` | defect C3/C4 | 52 | **link only.** Fourth wave in which this atom is the nearest shape-neighbour to a new finding and third in which it is declined; a resemblance is not shared knowledge |
| `kb-playbook-repair-frozen-clause-001` | C3's *"implement the cross-reference or supersede CF-36"* | 48 | **link only** — fourth wave running. The playbook already carries the method; a fourth instance adds none |
| `.kb/decisions/README.md` | three separate "the wave must not edit X" claims, across three files | 85 lexical | **no operation.** These are the authority being obeyed. `02`, *Standing choices* 6 |
| `.kb/reference/README.md` | `0033` C2 / `0034`'s "why the measurement is its own atom" | 75 lexical | **no operation.** Same reason; the rationale is realised as the Op 1 / Op 3 split, not as a README edit |

## Cross-file clusters

Six files, and **four genuine cross-file clusters** — the most any wave has carried. Two of them
would have produced duplicate atoms if the files had been ingested independently.

| Cluster | Files | Claims | Score | Disposition |
| --- | --- | --- | --- | --- |
| **CL-1** — ADR-0022 and the experiment under it | `0033`, `0034` | C1–C3, C5 / 0034-C1–C3 | **78** | **two atoms, and the split is mandatory.** Ops 1 and 3 |
| **CL-2** — the ADR-0031 number | `0031`, `0032` | 0031-C2 / C3 (filename) | **collision, not identity** | **two atoms, two numbers.** ADR-0031 → the runner, ADR-0032 → the serde correction. `02`, Adjudication 1 |
| **CL-3** — defect C2 and the macros reopen condition | `contract-defect-log-phase-7`, `happenstance-macros-verdict` | C2 / claim-5 | **84** | **one atom for the defect** (Op 8), and a `related` edge from Op 10. The macros file does not get its own copy |
| **CL-4** — D-1's two faces | `contract-defect-log-phase-7` | C1, C2 | **91** | **one atom, not two.** Op 8 |

### CL-1 — two atoms, and this is the one split the corpus mandates

`0033` and `0034` were staged together, on the same day, by the same story, and `0034`'s figures
appear almost verbatim in `0033`'s proposed `summary`. On subject identity that is 78, one point
below the collapse threshold and close enough that the threshold is not what decides it.
`.kb/decisions/README.md:36-39` decides it:

```
What does not belong here — The evidence. The measurement or compilation a decision rests on
is a reference atom that this one cites. Separating them is what lets a decision be superseded
without invalidating the evidence underneath it.
```

Both intake files quote that rule at themselves, unprompted, and both name
`kb-reference-position-visibility-experiment-001` as the precedent shape. The wave agrees and adds
one observation neither file makes: ADR-0022 is `reversibility: medium` with a live re-open trigger
(`postgres` and `neon` independently needing `index_arms()`), so the case for keeping the numbers
separately citable is not hypothetical here — the decision above them is *expected* to move.

### CL-2 — the collision, and why it is not a dedup

Two files, staged a day apart, both want the number 0031. They are **not** the same knowledge —
one corrects a serde attribution in ADR-0021, the other collapses the projection runner upward —
so this is not a merge candidate at all. It is an allocation conflict that would have produced two
atoms at one path, the second silently overwriting the first. Caught only because both files were
read in one pass, which is the entire argument for adjudicating a wave rather than a file.

### CL-3 — the reopen condition that belongs to another file's claim

`happenstance-macros-verdict` names its own reopen trigger as *"defect C2 settled with an
infallible `Tags` path"*, and `contract-defect-log-phase-7` independently asks the wave for a
decision record on the same C2 and notes *"if this is settled … AC-013's measurement should be
re-taken."* Two files, one subject, each pointing at the other. The temptation is to restate C2
inside the macros decision so that atom reads standalone. **Refused**: that is the "never copy
paragraphs into a second atom" rule exactly, and it would leave two descriptions of one open defect
to drift apart. Op 8 owns C2; Op 10 carries a one-sentence trigger and a `related` edge, and Op 9's
measurement atom is what a re-take would supersede.

### CL-4 — D-1 is one defect with two faces, and the intake says so

`C1` (no infallible `QueryItem` constructor) and `C2` (`DomainEvent::tags` is infallible over a
fallible `Tags`) both bear on `VT-18` `[FROZEN]`, both ask for a decision record, and the intake
labels C2 *"D-1's other face"* in its own text. `kb-decision-0020` logs the pair as a single defect
candidate, **D-1**. One atom. Splitting them would file half a defect twice and give the eventual
decision two questions to answer where the corpus recorded one.

## The dedup that was refused, and the ones that were not offered

| Group | Score | Ruling |
| --- | --- | --- |
| C3 (CF-36's level cross-reference) and C4 (no `PS` rule name resolved) as one atom | **72** | **Two atoms, linked.** Squarely in the 50–79 band: one defect shape — a `[FROZEN]` `CF` clause describing a `spec-trace` behaviour `spec-trace` does not perform — but two clauses with two different settling conditions. C3 chooses between writing the check and superseding CF-36; C4 chooses between resolving `PS` rule names through the `†` guard and accepting the skip, and carries a **third**, separable question about the `†` convention itself. `kb-open-question-es-7-and-vt-9-…` is the precedent for bundling, and it bundles two instances of *one* settling condition, which is what these two do not share |
| C4's `†`-convention half as a third atom, or as a `governance` atom | 40 | **Refused, folded into Op 12 as sub-question 3.** The `†` marker and the `PS` short-circuit are the *same guard* — `schedules_new` is set by the `†` (`spec_trace.rs:1630-1634`) — so the convention question and the resolution question are mechanically the same question asked twice. Splitting them would hide the one fact that makes either answerable |
| ADR-0031, ADR-0032 and ADR-0033 as one "phase 7 closeout" atom | 30 | **Not offered, and correctly.** Three subjects, three verdicts, three titles. `kb-playbook-one-decision-per-adr-title-001` is the bar, and it is the same refusal wave 3 made of three phase-6 ADRs |
| Op 9 (the macros measurement) folded into Op 10's Consequences | **74** | **Two atoms.** The nearest call in the wave and the one most open to disagreement. It goes two ways on the same README clause that governs CL-1, plus a fact specific to this measurement: the verdict's own reopen condition says *"AC-013's measurement should be re-taken"*, so the numbers are expected to be superseded on a schedule the decision does not share. A wave that split ADR-0022 from its experiment and inlined this one would be applying the rule by size of table |
| Op 13 (C5's fan-out cost) into Op 14's register | 45 | **Two atoms.** See the candidate table. Op 13 is the wave's only `create_new` where *every* candidate scored below 50 — which is the definition of "no reasonable owner exists" rather than a preference for a new file |
| N2 (`read_through` dead in eight `wasm32` combinations) into any `.kb/` atom | — | **No atom.** The intake routes it to `redkiln new`, and `.kb/open-questions/README.md:37-38` refuses it in terms: *"a task … is a backlog item in `.bklg/`, not a KB atom."* Carried in `unresolved` |
| N3 (a private module shadowing a glob re-export) into any `.kb/` atom | — | **No atom.** The intake names `standards/rust/` as the destination, which is outside this corpus. Carried in `unresolved` |
| `0031`'s C5 (the long-form rule-3 tightening) and C6 (the citation widening that is wrong) | — | **No atoms, and both verified.** `references/adr/0021-…` is 499 lines with *"Which instrument covers rule 3"* at `:458`, so C5's "already applied" is true. `projection.rs:152-154` is the quoted prose exactly and `:155` is `pub fn new`, so C6's refusal is correct. Recorded in `02` so neither is re-raised |
