# Wave `2026-08-15-intake` — corpus match

What already exists in `.kb/`, what each incoming claim could have merged into, and the score
behind every disposition. Read this before `02-placement-and-adjudication.md`: the placements there
are only defensible because of what this file establishes about the corpus and about the intake.

This is a **two-file wave**, and it is the second wave to run on 2026-08-15 — the suffixed id is
what keeps it from overwriting `2026-08-15-adr-0030-checkpoint-progress`'s audit trail, exactly as
both intake files instruct. Two files means cross-file dedup is back on the table for the first
time since 2026-08-13, and the answer this wave reaches is the interesting one: **the two ADRs do
not collapse, and the two ADRs' map claims do.** Everything else in the intake is a non-action that
has to be adjudicated in terms rather than by omission — two declined reference atoms, two declined
open-question edits, and a citation repair whose subject is not in `.kb/` at all.

**Fourteen claims in, two operations out.** No accepted decision atom is edited, none is superseded,
and no `status` flips anywhere in the corpus.

## The corpus, as verified

```
ls .kb/decisions      →  21 atoms (0001–0019, 0029, 0030), every one kind: decision
ls .kb/open-questions →  20 atoms, all authority_tier: note
ls .kb/reference      →   5 · playbooks 6 · maps 3 · concepts 1 · governance 1 · design 0 · product 0
                         57 atoms + 8 layer READMEs, before this wave
```

| Layer | Atoms | Bearing on this wave |
| --- | --- | --- |
| `decisions/` | **21**, ADR-0001–0019 + ADR-0029 + ADR-0030 | **Two are added** (ADR-0020, ADR-0021). **None is edited, none is superseded, no `status` flips.** Six are cited by the two new atoms: 0003, 0006, 0007, 0015, 0016, and each other |
| `open-questions/` | 20 | **None is created, resolved or amended.** Two are cited by id (`projection-id-unvalidated`, `human-readable-encoding-limits`) and both intake files say in terms that neither is resolved, edited or deleted |
| `maps/` | 3 | `decision-map` inherits **one** wave section carrying **two** rows; `domain-map` inherits **one new domain section**. `open-questions-index` inherits **nothing** — the first wave in this corpus's history for which that is true |
| `reference/` | 5 | **None is added.** Both intake files raise the question and both decline it; the wave declines it too, with a criterion rather than a precedent — see below |
| `playbooks/` | 6 | `one-decision-per-adr-title` is the bar ADR-0021's four-answers-in-one-title shape has to clear. Checked, cleared, **not** cited as an edge — Adjudication 6 in `02` |
| `concepts/`, `governance/`, `product/`, `design/` | 1 · 1 · 0 · 0 | Untouched. Checked rather than assumed: no persona, no journey, no interaction pattern, and no new rule about how this corpus is run |

For authority purposes:

- **Twenty-one accepted decision atoms exist and the intake touches six of them by name** —
  `kb-decision-0003` (opaque payloads, both records' premise), `kb-decision-0006` (the typed layer
  exists at all), `kb-decision-0007` (decoding is strictly above the port), `kb-decision-0015`
  (`Tags::from_pairs` is fallible and is the only way in), `kb-decision-0016` (the wire format that
  moves the bytes ADR-0021 reads), and each new atom by the other. **All six are cited; none is
  contradicted, none is edited, and none is superseded.**
- **There is no `supersede` candidate in this wave at all.** That is not the same as last wave's
  situation, where a supersession was available and declined: here both records state
  `supersedes: null` on their own first page, and the verification below found nothing that
  contradicts them. `02` states the four near-conflicts that were checked and cleared, because a
  wave that reports zero conflicts without naming what it looked at has not looked.
- **Both new atoms are `phase: 7` and both are transcriptions, not decisions.** ADR-0020 was taken
  by a human at the `/redkiln:plan` design sign-off gate on 2026-08-12; ADR-0021 takes its three
  answers in its own record because the public surface is invariant under all three
  (`_design.md:674-679`). Either way the wave authors no decision it did not receive.

## Provenance check

Both intake files are **staging notes describing ops**, not the knowledge itself — the same shape as
the four files of `2026-08-13-projection-adrs` and the single file of the earlier 2026-08-15 wave.
Every path either one names was tested against this worktree:

```
references/adr/0020-fold-query-agreement.md                    ✓  411 lines
references/adr/0021-payload-evolution-and-codec-tag.md          ✓  454 lines
crates/happenstance-core/src/projection.rs:152-154              ✓  the two-constructor sentence, verbatim
crates/happenstance-core/src/query.rs:112-115                   ✓  matches() reads event_type and tags only
crates/happenstance-core/src/event.rs:378-380, :399-401         ✓  with_metadata / metadata
RUNBOOK.md:525                                                  ✓  "Is happenstance-macros in scope for 0.1"
spec/SPECIFICATION.md:631-633                                   ✓  VT-3, [FROZEN], both halves
spec/SPECIFICATION.md:643-646                                   ✓  VT-3's Rejects — origin identity in metadata
.bklg/…/typed-layer-and-alpha-release/_design.md                ✓
```

**Both repaired citations are correct as repaired, checked line by line rather than taken on
trust.** `projection.rs:152-154` is the two-constructor sentence; `:47-61` is `ProjectionId`'s
docstring and its `new`, which is a different passage about the same subject. `RUNBOOK.md:525` is
the `happenstance-macros` row; `:524` is *"A store holding only a suffix of its own log"*, an
unrelated line. The repairs are real, and the wave can rely on them.

Three things the check turned up that neither intake file says, and each changes something:

1. **`.kb/` cites neither stale range, and `.bklg/` cites both, sixteen times and counting.**
   `projection.rs:47-61` is live in `_design.md:638`, `_decomposition.md:544`, four story `spec.md`
   files and two `_ledger.md` files; `RUNBOOK.md:524` is live in `_design.md` three times,
   `_decomposition.md` three times and four story documents. The story's own `report.md:24` states
   why: *"The stale ranges remain in `_decomposition.md` and `_design.md`, which are signed-off
   artefacts this story may not edit."* So the repair is **recorded, not performed**, and it is
   performed nowhere this wave is allowed to write. It goes in `unresolved` — `02`, Adjudication 9.
2. **ADR-0020 and ADR-0021 interact, and neither record says so.** ADR-0021's Decision 2 charges a
   price — *a genuinely new fact takes a new type name, and every query that must see it names it
   deliberately* — and ADR-0020 is the decision that makes that price payable in exactly one place,
   because the query is **derived** from `EVENT_TYPES` rather than hand-written. That is the wave's
   one genuine cross-file finding, and it is a `related` edge in both directions rather than a
   merge. `02`, Adjudication 2.
3. **The domain map has no typed-layer domain.** Both files ask for *"an entry under the typed
   layer"*; the map's two sections are *Specification governance & conformance* and *Contract ports,
   conformance, and the ADR corpus*, and neither is it. The map's own **Adding a domain** section
   decides it: *"Append a new `##` section rather than editing this one — a domain is a subject
   area, not a wave."* `02`, Adjudication 4.

## Scoring method

Unchanged from all four previous waves — a threshold that moves between waves is not a threshold. A
0–100 judgement of **subject identity**: would a reader looking for one claim expect to find the
other in the same atom.

- **≥ 80 — same knowledge.** Collapse into one atom. Never two.
- **50–79 — same subject, different method or different settling condition.** Two atoms, linked, and
  the reason written down.
- **< 50 — related.** A `related` link and nothing else.

Four rules carried forward and one added:

- **A supersession chain is never a merge** (wave 2). Not exercised — nothing is superseded.
- **A README is not a merge target** (wave 1). Exercised three times: `decisions/README.md`'s
  evidence rule governs the two declined reference atoms, `reference/README.md`'s dating rule
  supplies the criterion, and `domain-map.md`'s *Adding a domain* section governs the new section.
  All three are **obeyed**, not absorbed.
- **A decision naming a gap is not the same knowledge as the question that owns the gap** (wave 3).
  Exercised on defect candidate D-1: ADR-0020 names a shortfall in `happenstance-core` and routes it
  to AC-012's log. `02`, Adjudication 8.
- **An accepted decision atom is not a merge target, however high it scores** (wave 4). Not
  exercised — nothing this wave carries scores above 55 against an accepted atom, which is itself
  the finding that makes both `create_new`s easy.
- **New this wave: a lexical collision is not subject identity, and "derivation" is the word that
  proves it.** `kb-decision-0008` is titled *One derivation scheme, both ports* and ADR-0020's whole
  content is a derivation on a sealed trait. The two senses share no referent: ADR-0008 derives the
  `Send` flavour of a port with `trait_variant` in `happenstance-core`, ADR-0020 derives a `Query`
  from a domain enum in `happenstance`. Scored on the word it is 40; scored on subject identity it
  is 25, and the edge is refused rather than added as harmless — a `related` edge that a reader
  follows and finds nothing is worse than no edge, because it spends the reader's trust in the map.

## Merge candidates against the existing corpus

| Existing atom | Incoming claim | Score | Disposition |
| --- | --- | --- | --- |
| `kb-decision-0008` | `0020-C1` — a derivation the caller cannot override | 40 lexical, **25 subject** | **no edge.** The false friend above. Neither `depends_on` nor `related` |
| `kb-decision-0007` | `0020-C1` — `Boundary::query` returns the same `Query` type a projection nominates with | 35 | **`related` on Op 1.** ADR-0007's third shape decision — *"there is no second filtering vocabulary"* — is the thing ADR-0020 could have broken and does not. A near-conflict, checked and cleared (`02`, Adjudication 1) |
| `kb-decision-0015` | `0020-C1` — `Tags::from_pairs` is fallible and is the only way in | 45 | **`related` on Op 1, an edge the wave adds.** ADR-0015 is the decision that made the constructor fallible; ADR-0020's claim 3 is built on it. `depends_on` was considered and lost — `02`, Adjudication 3 |
| `kb-decision-0003` | `0020-C1`, `0021-C1` — payloads are opaque, so the typed layer is where decode and encode live | 45 / 45 | **`depends_on` on both ops**, as both intake files propose. Below the collapse threshold and above nothing: a premise is not a merge |
| `kb-decision-0006` | `0020-C1`, `0021-C1` — there is a typed layer to put this in | 30 / 30 | **`depends_on` on both ops.** Same reasoning |
| `kb-decision-0016` | `0021-C1`, `0021-C7` — the wire format that moves `Event::metadata`'s bytes | **55** | **two atoms, linked.** The highest score in the wave, and the one merge a careless pass would make. `02`, Adjudication 1 |
| `kb-decision-0007` | `0021-C1`, `0021-C3` — *"decoding is strictly above the port"*, the premise VT-3's answer turns on | 35 lexical, **premise** | **`depends_on` on Op 2** — promoted from the intake's `related`. `02`, Adjudication 3 |
| `kb-open-question-projection-id-unvalidated-001` | `0020-C6` — a reciprocal backlink to the new atom | 100 as a citation, **0 actionable** | **no operation.** `02`, Adjudication 7. The atom is cited outbound by Op 1 and is not edited |
| `kb-open-question-human-readable-encoding-limits-001` | `0021-C6` — a seam marked, not resolved | 100 as a citation, **0 actionable** | **no operation**, same reasoning. Cited outbound by Op 2 |
| `kb-reference-wire-format-measurements-001` | ADR-0021's falsifier table | 30 | **no edge.** ADR-0016's measurements are byte sizes on a wire; ADR-0021's table is an argument about adapter obligations with no number in it. Not evidence of the same kind, and not evidence at all — `02`, Adjudication 5 |
| `kb-playbook-one-decision-per-adr-title-001` | ADR-0021's title carrying three answers | 45 | **no edge, and a bar cleared rather than a link earned.** `02`, Adjudication 6 |
| `kb-map-decision-001` | `0020-C2`, `0021-C5` | 100 | **`mapsImpact.decisionMap` on Ops 1 and 2**, not ops of their own — the convention every previous wave used |
| `kb-map-domain-001` | `0020-C3`, `0021-C5` | 100 | **`mapsImpact.domainMap` on Ops 1 and 2.** The *placement* inside it is the adjudicator's call and is made in `02`, Adjudication 4 |
| `kb-map-open-questions-index-001` | — | — | **nothing.** No question is opened, closed or amended this wave |

## Cross-file clusters

Two files, and exactly one cluster that collapses. The table states both the collapse and the two
refusals, because a refusal recorded only by absence is indistinguishable from an oversight.

| Cluster | Claims | Files | Score | Disposition |
| --- | --- | --- | --- | --- |
| **CL-1** — the two ADRs themselves | `0020-C1` · `0021-C1..C4`, `C7` | both | **38** | **two atoms.** Below the threshold and structurally barred besides — see below |
| **CL-2** — the map rows | `0020-C2` + `0021-C5` (decision map); `0020-C3` + `0021-C5` (domain map) | both | **100** | **collapsed, into `mapsImpact` rather than into an op.** One wave section on `decision-map`, two rows; one new domain section on `domain-map`, two entries. This is the wave's real dedup |
| **CL-3** — the two declined reference atoms | `0020-C4` + `0021-C8` | both | **100 as a question** | **collapsed into one adjudication, producing no atom in either direction.** Both files raise the identical question and both defer it here; answering it twice would be the wave disagreeing with itself |
| **CL-4** — the two untouched open questions | `0020-C6` + `0021-C6` | both | 100 as a shape | **collapsed into one adjudication, no op.** Different atoms, identical disposition |

### CL-1 is the wave's one real temptation, and it must stay two atoms

The case for collapsing is stronger than it looks. Both records are phase 7, both were signed off in
the same design gate on the same `_design.md`, both `depends_on` the same pair, both quote
`projection.rs:152-154`'s two-constructor sentence in their rejected alternatives, and both land in
the same new domain section. A reader who has just met one is very likely reading the other next.

It stays two atoms on three grounds, in increasing order of strength:

1. **Subject identity is 38.** *How a query is derived from a domain enum* and *where a codec tag
   lives* share a crate and a slice, not a subject. The shared quotation is a shared *method* — both
   records invoke the same defect class, "two admissible constructions of one value" — and a shared
   method is what a `playbook` atom is for, not a merge.
2. **`adr_id` and the supersede pair have to identify one atom.** This is the same structural bar
   wave 3 applied to three phase-6 ADRs and wave 4 applied to ADR-0030 against ADR-0017–0019. Two
   `adr_id`s cannot live in one atom, and a merged atom could never be half-superseded.
3. **They have different reversibilities, and the difference is load-bearing.** ADR-0020 is
   `medium`; ADR-0021 is `low`, and its own record explains that reversing it after
   `0.2.0-alpha.1` means re-siting a tag on events already written into users' stores, which VT-3
   forbids any adapter from doing on their behalf. A merged atom would have to print one number, and
   whichever it printed would be false about half its content.

What the collapse would have bought — an adjacency a reader can follow — is bought instead by the
mutual `related` edge (Adjudication 2) and by the domain section (Adjudication 4), at no cost to
either atom's atomicity.

## The dedup that was refused, and the one that was not offered

| Group | Score | Ruling |
| --- | --- | --- |
| ADR-0021's four numbered claims as four atoms | — | **Not offered, and correctly.** The record's own framing is *"one decision atom, three answers"* plus the rule they imply, and its title carries all three. Claim 4 — *an unframed event decodes with the codec in hand, not `UnknownTag`* — is not a fourth decision but the compatibility consequence of the first three, which is why the record calls it *"the rule this implies, so M3 does not invent one"*. Splitting it out would file a consequence as a commitment and leave the decision that entails it unable to state its own back-compatibility story |
| ADR-0020's five numbered claims as five atoms | — | **Not offered.** The intake states it in terms: *"five, and they are one shape"* — the shape being that there is nowhere to put a hand-maintained query. Each claim alone admits the hazard the decision exists to close |
| ADR-0020 or ADR-0021 merged into `kb-decision-0006`'s typed-layer allocation | 30 | **Refused, and not close.** ADR-0006 allocated a crate name; these two put things in it. Also barred: it is accepted and immutable |
| Defect candidate D-1 as an `open_question` atom | 55 against the layer's contract | **Refused this wave**, and the refusal is visible rather than silent — `unresolved`, plus `02` Adjudication 8. It matches last wave's declined claim 7 profile, not its opened PS-32 profile: no conflict, no `MUST` anywhere asking for it, a live owner outside `.kb/` (AC-012's log), and a first appearance rather than a third |
