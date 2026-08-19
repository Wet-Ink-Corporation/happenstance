---
item: HS-S0121
stage: spec
created: 2026-08-12T13:48:02.170Z
updated: 2026-08-12T13:48:02.170Z
template_sig: 87bbf1d0
rendered_sig: 8144c780
---

# Spec — ADR-0028, and the open question resolved rather than deleted

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — BR-11, AC-14, **DoD 15**, DT-7, exit criterion 5 |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — the DAG, the scope seams, gate decision 4 (no surface change) |
| Project | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md` — AC-008, AC-010, AC-015, AC-016; DR-8, DR-9, DR-10 |
| This spec | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/adr-0028-and-the-open-question-wave/spec.md` |
| Key briefs | `.bklg/.../retention-and-incomplete-logs/_decomposition.md` — architecture brief **DA-1** (widening), **DA-2** (falsifiability only), **DA-3** (which outcome held), **DA-6** (the three-artifact wave), **DA-7** (the escalation option table), **DA-9** (redaction against the code); testing brief rows AC-008 / AC-010 / AC-015 / AC-016 (`:610`, `:612`, `:617`, `:618`) |
| Signed-off design | `.bklg/.../retention-and-incomplete-logs/_design.md` — **N/A by sign-off**: this project records *no* user-facing surface; the approval is of the no-surface determination itself |
| Story map row | `.bklg/.../retention-and-incomplete-logs/_storymap.md` — slice `the-retention-decision`, second of two |
| Roadmap pointer | `RUNBOOK.md:307` (ADR-0028's queue row) and `RUNBOOK.md:4626-4674` (phase 14 in full: work, proof artefact, exit criteria) |
| Open question being resolved | `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md` — sub-question 3 |

## One-line PR slice

Author ADR-0028 as **one question** — what is a store permitted to forget, and how does it say so —
long form under `references/adr/` and the atom staged in `.kb/_intake/` and landed by
`/redkiln:kb-ingest`, never hand-written into `.kb/decisions/`; and in the **same** ingest wave
resolve `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md` per DA-6: sub-question 3
answered on the record, a **narrowed successor** open-question atom carrying only the
`read_from_a_gap_position` thread, the original moved to `superseded` with `related` naming both,
`.kb/maps/open-questions-index.md`'s bullet updated, and `redkiln validate --kb` clean.

## Executive summary

This PR writes the project's answer down. Everything before it in this project produced *evidence*:
the instrument, the recorded pass list per configuration, two rules with their registered mutants,
and three readers run against a hole with their actual output captured. This story converts that
evidence into the two artefacts the initiative's exit criterion 5 and DoD 15 are read against — an
accepted decision atom on disk, and an open-question atom **resolved rather than deleted**.

Delta against the project brief, not a restatement of it:

- **Two files, one decision.** `CLAUDE.md` records that a decision lives in two places on purpose:
  the long-form record under `references/adr/` carries the transcripts, the rejected alternatives
  and the tables a summary cannot hold; the atom under `.kb/decisions/` carries the frontmatter,
  the status and the supersession graph `redkiln validate --kb` enforces. This story authors the
  first directly and the second **only** as intake — `.kb/_intake/0028-<slug>.md`.
- **The wave is three artefacts, not one.** DA-6 is binding: the atom this project resolves bundles
  two threads and this project owns one. A single flip to `withdrawn` would close
  `read_from_a_gap_position`'s ownership in passing, which is the exact defect `project.md`
  *Out of scope* fences against.
- **Both branches are in scope, and one is not cheaper.** DR-9 keeps the instrument and the
  illustration under the refusal branch; the only thing refusing saves is DA-7's escalation. The
  branch this story lands is fixed by evidence already in hand, not chosen here on schedule grounds.
- **This story decides nothing about the port.** If the honest answer needs a surface, ADR-0028
  states the surface, the version consequence and DA-7's options and **stops**; the escalation
  artefact itself is `surface-diff-and-the-ac-012-escalation`'s (project AC-012).

## Context pack

Read this section and you can start. Everything deeper is an anchor below.

**1. ADR-0028 is one question, and the question is fixed.** *What is a store permitted to forget,
and how does it say so?* That is `RUNBOOK.md:307`'s row verbatim (scope: ES-39, CF-27, SY-32). An
ADR that cannot be stated as one question is two ADRs (`RUNBOOK.md:264-268`, project DR-8). Do not
widen it to cover replication identity, the merge rule, or gap-read ownership.

**2. The answer may be a refusal, and a refusal has terms.** RUNBOOK phase 14's goal names both
outcomes: *"decide what a store is permitted to forget and how it says so — or refuse the whole area
in writing, which is a legitimate answer that silence is not."* Under refusal, DR-9 still owes the
second half of the sentence: *"deletion is out of scope for `EventStore`, and here is what a
deleted-from store looks like to a reader"* — the instrument and the three recorded reader
observations **are** that illustration and are cited as such. A refusal by omission is not a
refusal.

**3. The alternatives that lost are named, and one of them is named for a specific reason.**
`earliest_position()` is the only primitive anyone has proposed and ES-39
(`spec/SPECIFICATION.md:4336-4341`) argues against it by name: a regulated purge is **scattered,
not a prefix** — a 2019 catastrophic-injury file with a periodical payment order sits at a low
position and must survive while its neighbours are destroyed — so *"a floor is the shape a prefix
truncation has and precisely the shape this purge does not, and it ships looking correct until a
claim runs long."* ES-39's `Rejects:` names shipping it on the strength of the prune case as the
path of least resistance. ADR-0028 must reject it **in those terms**, not merely omit it. The other
losers are DA-7's remaining rows: a set of retained ranges, a third outcome on condition evaluation,
and the tri-state `contains_event_id` DA-8 found on a surface that already exists.

**4. The evidence base is data this story consumes, not prose it writes.** The enumerated pass list
per configuration (suffix **and** scattered, DA-1) from `cf-27-experiment-and-recorded-pass-list`;
the DA-3 outcome that actually held, with the other two named; and the three reader observations —
`read_decision_model`'s last-*retained* match feeding `AppendCondition::after_opt` into an admitted
append, `IngestStore::holds` answering `false` for an event this store minted, and the projection
runner's actual state across the hole. ADR-0028 cites them; it does not re-derive them and it does
not soften a null result. If the pass list is "all of them", that is CF-27's predicted outcome and
the **strongest** evidence for ES-39 — record it as the finding it is.

**5. DA-1's widening is recorded, with its reason.** CF-27's text says *"holds only a suffix"*
(`spec/SPECIFICATION.md:8034-8036`); ES-38 and ES-39 say *"scattered subset"*. The instrument was
built to the wider reading because a suffix-only instrument **structurally cannot falsify the
floor**. That widening is available to take only because CF-27 is `[DEFERRED — owned by the pass
that settles retention and deletion]` and this project is that pass — so ADR-0028 records it as a
deliberate widening with its reason, and the clause text and the instrument agree afterwards.

**6. Falsifiability, not implementability — and the residual exposure stays open.** The instrument
forgets by hiding at the port (DA-2), and at that seam a store that hides and a store that destroyed
are the same value. CF-26 (`spec/SPECIFICATION.md:8020-8032`) says a fixture instrument satisfies
CF-25's falsifiability half and **not** its implementability half; the completeness axis's *"device
adapter second"* (`:8090-8098`) is out of this project's scope. ADR-0028 records that exposure as
**open**. Reporting the axis as covered is the failure this bullet exists to prevent.

**7. SY-32 is consumed by id, never re-decided.** `spec/SPECIFICATION.md:6771-6799` — a peer reports
the floor below which it no longer retains history, and a runner detects a peer offline longer than
another's retention window. Its answer arrives from `replication-identity-and-ingest` (HS-P0017).
ADR-0028 cites SY-32 **by id**; where this project's answer changes what a peer must report, that is
recorded against SY-32 rather than by reopening ADR-0026 or ADR-0027, and this story's diff touches
neither of those atoms (project AC-016).

**8. The redaction answer (project AC-010) is carried, not re-decided here.**
`dt-7-signal-shape-and-the-redaction-answer` settles E2E-49 against the code DA-9 reads: `Tag` is
`Tag(Cow<'static, str>)` with hand-written `PartialEq`/`Eq`/`Ord`/`Hash` all delegating to
`as_str()`, no digest field to swap in, and no store-side update path. ADR-0028 is where AC-010 asks
for it to be **recorded** — either the decision, or an explicit deferral naming a **real**
`experiments/` path. CF-38 makes an unnamed deferral a gate failure, so "deferred" with no path is
not an option this story can land.

**9. The open-question wave has exactly three artefacts (DA-6), and the mechanism is written down.**
`.kb/open-questions/README.md` *"Resolving one"*: the answer is a **new atom**, the question's record
**stays**, the two are linked via `related`, the question's `status` moves to `withdrawn` or
`superseded`, and *"do not rewrite a question into its own answer."* So: (1) ADR-0028; (2) a
**narrowed successor** `open_question` atom, still `accepted`, carrying only the
`read_from_a_gap_position` ownership thread with its own `related` edge back to the original; (3)
the original moved to `superseded` with `related` naming both. Plus the bullet update at
`.kb/maps/open-questions-index.md:109-113`, which that map's own summary requires — a superseded
question stays listed and annotated rather than removed.

**10. Sub-question 3 is the one this story answers on the record.** *"When CF-27 lands, does
`positions_are_not_reused_after_removal` get written against whatever removal-capable fixture arrives
with it, or does phase 14 need its own instrument decision first?"*
(`.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md:87-89`). The project has now run that
experiment, so the answer is a fact about what happened, not a preference. Sub-questions 1 and 2 —
`read_from_a_gap_position`'s owner, and the general `[FROZEN]`-clause-needs-an-owning-phase policy —
are **not** answered; they are what the narrowed successor carries.

**11. Nothing under `.kb/` is hand-written.** Atoms are authored by `/redkiln:kb-ingest` from
`.kb/_intake/`; hand-writing them produces the directory layout of the process without the process,
which is why the first attempt was reverted (`0269720`, cited in `CLAUDE.md`). An **accepted decision
atom is immutable** and `validate --kb` checks each against `HEAD`. If the ingest wave cannot express
the supersession of an existing `open_question` atom as a mutation, that is a **finding to report** —
not grounds for hand-editing `.kb/` (DA-6's closing paragraph).

**12. Two operational facts about the wave, from DA-6.** The intake glob picks up
`.kb/_intake/README.md`, so drop it at the approval gate; and suffix the wave id so it does not
overwrite an earlier wave's audit trail under `.kb/_governance/integration-waves/` (two waves are
already recorded there). A successful ingest **clears** `.kb/_intake/` — a file still sitting there
after a run is a file that run did not ingest (`.kb/_intake/README.md:13-19`).

**13. The persona-journey slice.** The reader this artefact serves is the adapter author and the
maintainer who arrives at `spec/SPECIFICATION.md`'s completeness row and finds *"No, and nothing is
planned"*. What they get from this PR is a written answer with its losers named and its residual
exposure stated — and, if they follow the gap-read thread, an open question that is still open
rather than one that was closed while they were not looking.

## Integration contract

- **Archetype**: `foundation` — real in-tree substrate (a decision atom and a resolved open question)
  that `cf-27-rule-or-recorded-refusal` and `marker-moves-and-spec-trace-green` consume directly in
  this same project. Not a double, not a flag, not a `todo!()`.
- **Slice / milestone**: `the-retention-decision`. Slice-mate:
  `dt-7-signal-shape-and-the-redaction-answer` (merged first — it produces the AC-010 answer this
  story records). The two are implemented in one context and land as one integrated result: a
  decision with an answered redaction question in it.
- **Mount point**: **`.kb/_intake/`**, consumed by `/redkiln:kb-ingest` — composition root 8 of the
  architecture brief (`_decomposition.md:146-148`). This is the *only* write path into `.kb/`. The
  wave lands at `.kb/decisions/0028-<slug>.md`, `.kb/open-questions/` (the narrowed successor plus
  the superseded original) and `.kb/maps/open-questions-index.md`; the Maps phase of the same wave
  updates `.kb/maps/decision-map.md`, and the wave's audit trail lands under
  `.kb/_governance/integration-waves/<wave-id>/`. An atom written straight into `.kb/decisions/` is
  not mounted — it is the reverted shape.
- **Wires into**:
  - `references/adr/` — the long-form record, the sibling half of the two-file convention
    (`CLAUDE.md`, *Where the work lives*); `references/adr/0029-msrv-raised-to-1-97-1.md` is the
    nearest precedent for shape and length.
  - `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md` — the atom being resolved, and the
    source of the narrowed successor's body.
  - `.kb/open-questions/README.md` *"Resolving one"* and `.kb/README.md:21`'s status enum
    (`draft · proposed · accepted · superseded · withdrawn`) — the mechanism and the vocabulary.
  - `.kb/decisions/0029-msrv-raised-to-1-97-1.md` — the `KbFrontmatter` shape an intake atom must be
    able to become (`kind`, `status`, `authority_tier`, `adr_id`, `reversibility`, `phase`,
    `supersedes`/`superseded_by`, `summary`, `depends_on`, `related`, `source_paths`,
    `last_reviewed`).
  - `.kb/decisions/0013-position-assignment-and-visibility.md` — the ADR that *settled* ES-38 and
    created the "Owner: phase 14" obligation this story discharges. Cited, never renegotiated.
  - `.kb/decisions/0007-projection-runner-decodes.md` and
    `.kb/decisions/0001-async-port-flavours.md` — cited where the reader observations depend on them.
  - The five upstream stories' recorded outputs (pass list, two rules + mutants, three reader
    observations, the DT-7/redaction answer) — the evidence ADR-0028 cites by artefact path.
- **Renders surfaces**: **none.** `_design.md` records `N/A — no user-facing surface` for this
  project and that determination is what was signed off (2026-08-12). This story renders no Rust
  public item either: it adds no `pub` item, changes no signature, and touches no file under
  `crates/`.
- **Conformance rule(s)**: none — this story is **not adapter-observable**, and that is deliberate.
  It writes the decision that *fixes the assertion* of `suffix_store_is_distinguishable_from_a_young_store`;
  writing (or not writing) that rule is `cf-27-rule-or-recorded-refusal`'s, and moving the markers is
  `marker-moves-and-spec-trace-green`'s. This story changes no port, so it owes no rule.
- **Clause(s)**: **discharges none by itself; it is what makes three dischargeable.** ADR-0028 is the
  decision ES-39 (`spec/SPECIFICATION.md:4325-4349`, `[DEFERRED]`), CF-27 (`:8034-8060`,
  `[DEFERRED]`) and SY-32 (`:6771-6799`, `[DEFERRED]`) are each deferred *on*. No clause text or
  marker is edited in this PR — every edit to `spec/SPECIFICATION.md` belongs to
  `marker-moves-and-spec-trace-green`. Nothing `[FROZEN]` is amended: ES-37 and ES-38 are cited as
  binding, and ADR-0026/ADR-0027 are cited by id and not touched.
- **Advances DoD scenario**: initiative **DoD 15** — *"Incomplete logs have an answer on disk … the
  reader either fails loudly or the refusal to define this is recorded as a decision"*
  (`.bklg/from-contract-to-published-library/initiative.md:402-405`), and with it exit criterion 5's
  second half (*the matching open-question atom resolved rather than deleted*). This story is the
  *"on disk"* half; HS-P0019 re-observes it as part of the assembled set.

## PR boundary

**In this PR**

- `references/adr/0028-<slug>.md` — the long-form record: the one question, the context, the decision
  (or the reasoned refusal in terms), the alternatives that lost with `earliest_position()` argued
  against on ES-39's own grounds, the pass-list evidence and the DA-3 outcome, DA-1's widening with
  its reason, the CF-25/CF-26 residual recorded open, SY-32 cited by id, and AC-010's redaction
  answer or its named-experiment deferral.
- `.kb/_intake/0028-<slug>.md` — the staged decision atom.
- `.kb/_intake/<narrowed-successor>.md` — the staged narrowed `open_question` atom carrying only the
  `read_from_a_gap_position` thread.
- `.kb/_intake/<supersession-note>.md` — the staged instruction that moves
  `es-38-and-gap-read-rules-are-unowned.md` to `superseded` with `related` naming both.
- The results of running `/redkiln:kb-ingest` on that wave: `.kb/decisions/0028-<slug>.md`, the
  narrowed successor and superseded original under `.kb/open-questions/`, the bullet update at
  `.kb/maps/open-questions-index.md`, the Maps-phase row on `.kb/maps/decision-map.md`, the wave
  record under `.kb/_governance/integration-waves/`, and a cleared `.kb/_intake/` (README aside).
- This story's own backlog folder: the `_ledger.md` and, at report stage, its implementation report.

**Explicitly not in this PR**

- Any edit to `spec/SPECIFICATION.md` — no marker moves, no clause text, no census
  (`marker-moves-and-spec-trace-green`).
- Writing or declining `suffix_store_is_distinguishable_from_a_young_store`
  (`cf-27-rule-or-recorded-refusal`).
- The AC-012 escalation artefact and the `0.2.0` surface diff
  (`surface-diff-and-the-ac-012-escalation`).
- Any file under `crates/` — no signature, no rustdoc, no rule body, no fixture item. ES-40's
  documentation sentence is `condition-over-removed-history-does-not-reject`'s and has already
  landed.
- Answering sub-questions 1 or 2 of the open-question atom, or settling
  `read_from_a_gap_position`'s ownership in any form.
- Re-deciding SY-32, or editing `.kb/decisions/0026-*` / `0027-*` in any way.
- Hand-editing anything under `.kb/decisions/` or `.kb/open-questions/` outside a `kb-ingest` wave.

**The implementer may also touch the mount named above** — `.kb/_intake/` and the map atoms the
ingest wave updates — because that is how this slice is mounted, not scope drift.

```
references/adr/0028-*.md
.kb/_intake/**
.kb/decisions/**
.kb/open-questions/**
.kb/maps/**
.kb/_governance/**
.bklg/from-contract-to-published-library/retention-and-incomplete-logs/adr-0028-and-the-open-question-wave/**
```

**Merge DoD.** ADR-0028 exists as an accepted `.kb/decisions/` atom reached through the ingest path
and answers one question with its losers named; the open-question atom is `superseded` rather than
deleted with a narrowed successor beside it and the index bullet updated; `redkiln validate --kb &&
redkiln doctor` are clean; and `cargo xtask affected --base main` is green (this story maps to no
package, so its value is the five file-reading lints and `spec-trace` — which must stay green
*without* this story having edited the specification).

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| ADR-0028 exists in **both** places, and only one of them is hand-written | The long form is authored directly under `references/adr/`; the atom is authored **only** as `.kb/_intake/0028-<slug>.md` and reaches `.kb/decisions/` through `/redkiln:kb-ingest`. Link the atom; cite the record by `file:line` | `CLAUDE.md` *Where the work lives*; `.kb/_intake/README.md`; the revert at `0269720`; precedent pair `references/adr/0029-msrv-raised-to-1-97-1.md` + `.kb/decisions/0029-msrv-raised-to-1-97-1.md` |
| It is **one** question | *What is a store permitted to forget, and how does it say so?* — scope ES-39, CF-27, SY-32, and nothing else | `RUNBOOK.md:307`; `RUNBOOK.md:264-268` (an ADR that cannot be stated as one question is two ADRs); project DR-8 |
| The answer is a decision **or** a refusal in terms | Refusal branch owes the illustration: the instrument plus the three recorded reader observations, cited, not summarised | `RUNBOOK.md:4626-4634`; project DR-9; `_decomposition.md:561-564` (*"the refusal branch is not cheaper"*) |
| `earliest_position()` is rejected on ES-39's own grounds | Scattered purge is not a prefix; the low-position survivor; *"ships looking correct until a claim runs long"*. Named as a loser, not omitted | `spec/SPECIFICATION.md:4336-4349` |
| The remaining losers are DA-7's rows | Retained ranges (new method + new public type); a third outcome on condition evaluation (`crates/happenstance-core/src/error.rs:214-248`, reaches every caller's `match`); tri-state `contains_event_id` (`crates/happenstance-core/src/store.rs:268`) — all four are `0.3.0`-class under 0.x | `_decomposition.md:363-379` (DA-7); `_decomposition.md:381-397` (DA-8) |
| The pass list is the evidence, per configuration | Suffix **and** scattered, enumerated as rule names, stated as *the set of rules that cannot tell a pruned store from a young one*; the DA-3 outcome that held is recorded with the other two named | `spec/SPECIFICATION.md:8034-8060` (CF-27's stated experimental outcome); `_decomposition.md:234-274` (DA-3); the upstream story's recorded artefact |
| The three reader observations are cited as observed output | `read_decision_model` → last *retained* match → `after_opt` → an admitted append; `IngestStore::holds` → `false` for an event this store minted; the projection runner's actual state across the hole | `crates/happenstance-core/src/store.rs:321-331`; `crates/happenstance-sync/src/ingest.rs:164`; `crates/happenstance-testkit/src/suite.rs:1490-1531`; `RUNBOOK.md:4658-4668` |
| DA-1's widening is recorded with its reason | CF-27 says *"suffix"*; the instrument holds an arbitrary retained set, because a suffix-only instrument cannot falsify the floor. Available only because CF-27 is owned by this pass | `spec/SPECIFICATION.md:8034-8036` vs `:4309-4311`, `:4326-4328`; `_decomposition.md:150-171` (DA-1) |
| The CF-25/CF-26 residual is recorded **open** | Falsifiability discharged, implementability not; the *"device adapter second"* stays outstanding on the completeness axis | `spec/SPECIFICATION.md:8003-8032`, `:8090-8098`; `_decomposition.md:205-213` (DA-2) |
| SY-32 is cited by id | Consumed as answered by `replication-identity-and-ingest`; any change to what a peer must report is recorded *against SY-32*, not by reopening ADR-0026/ADR-0027 | `spec/SPECIFICATION.md:6771-6799`; project AC-016; `_decomposition.md:618` (the check is that ADR-0026/0027 are untouched by the diff) |
| The redaction answer (AC-010) is recorded here | Decision, or explicit deferral naming a **real** path under `experiments/` — an unnamed deferral is a CF-38 gate failure | `spec/E2E-CASES.md:1288-1311` (E2E-49); `crates/happenstance-core/src/tag.rs:79`, `:170-193`; `_decomposition.md:446-466` (DA-9); existing `experiments/` siblings: `position-visibility`, `wire-format` |
| Sub-question 3 is answered on the record | *Does the rule get written against whatever removal-capable fixture arrives with CF-27, or does the phase need its own instrument decision first?* — answered as what happened, in ADR-0028's body | `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md:87-89` |
| Sub-questions 1 and 2 survive in a narrowed successor | New `open_question` atom, `status: accepted`, `authority_tier: note`, carrying only the `read_from_a_gap_position` ownership thread and the `[FROZEN]`-clause-owner policy question, `related` back to the original | `.kb/open-questions/README.md:40-45`; `_decomposition.md:328-361` (DA-6) |
| The original is `superseded`, never deleted | `status: superseded`, `related` naming both ADR-0028 and the narrowed successor; body left describing what was not known at the time (*"do not rewrite a question into its own answer"*) | `.kb/open-questions/README.md:40-45`; `.kb/README.md:21` (the status enum) |
| The index bullet is updated, not removed | `.kb/maps/open-questions-index.md`'s existing `**Open** — es-38-and-gap-read-rules-are-unowned.md` bullet is re-annotated; a superseded question stays listed | `.kb/maps/open-questions-index.md:109-113` |
| The whole wave lands through one `kb-ingest` run | Wave id suffixed so it does not overwrite an earlier wave's audit trail (`.kb/_governance/integration-waves/2026-08-10-intake`, `-2` already exist); `.kb/_intake/README.md` dropped at the approval gate; `_intake` cleared on success | `_decomposition.md:356-361`; `.kb/_intake/README.md:13-19`, `:21-31` |
| An inexpressible mutation is a **finding**, not a hand edit | If the wave cannot express the supersession of an existing `open_question` atom, report it; do not hand-edit `.kb/` | `_decomposition.md:356-358`; `CLAUDE.md` (the `0269720` revert) |
| No surface changes | No file under `crates/` is touched; no `pub` item, no signature, no rustdoc. AC-011's diff is proved by a sibling story, but this story contributes a zero | project AC-011; `_decomposition.md:54-65` (the seam map: this story's row is `.kb/_intake/ → .kb/…`, surface class *authored through kb-ingest*) |
| Validation is the gate | `redkiln validate --kb` (`KbFrontmatter` conformance + accepted-decision immutability against `HEAD`) and `redkiln doctor`; the six expected `template-drift` advisories are the known-good set | `CLAUDE.md` *Commands*; `_decomposition.md:610`, `:617`, `:641` |

## Data and migrations

**N/A for application data** — this story writes no schema, no store, no migration, and touches no
crate. The only persisted state it changes is the knowledge base, and that is a governed mutation
rather than a migration:

- **`.kb/` is append-and-supersede, not update-in-place.** An accepted decision atom's body is
  immutable and `redkiln validate --kb` checks each against `HEAD`; a correction is a new atom
  carrying `supersedes`, with the old atom's frontmatter flipped to `status: superseded` +
  `superseded_by`. ADR-0028 is a **new** decision (`supersedes: null`) — it discharges an obligation
  ADR-0013 created and renegotiates nothing (`.kb/decisions/README.md`; `.kb/maps/decision-map.md`).
- **The one status transition this story makes** is on an `open_question` atom, not a decision:
  `kb-open-question-es-38-and-gap-read-unowned-001` moves `accepted → superseded`. Its body is left
  as-is by design.
- **Reversal.** The whole wave is a single commit on its own worktree branch, so reverting it
  restores the prior `.kb/` state and the staged `_intake` sources in one move — which is why the
  wave must land as one ingest run rather than as several partial ones.

## Acceptance criteria

Every criterion is framed from a persona's goal crossing the full stack, from the initiative's
carried personas and journeys (`.bklg/from-contract-to-published-library/initiative.md:227-250`;
`_discovery/distillation/personas-and-journeys.md`). The four that matter here: the **adapter
author** (*Learn when you are finished*), the **evaluator** (*Decide in one sitting*), the
**application author** (*Choose a contract before a database*), and the **maintainer** who inherits
this tree and has to know what is still open. "The reader" below always means one of these, never a
screen.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an evaluator doing a bounded look at public evidence, **WHEN** they open `references/adr/0028-<slug>.md`, **THEN** it is a single long-form record that states *one* question verbatim — *what is a store permitted to forget, and how does it say so* — scoped to ES-39, CF-27 and SY-32 and to nothing else, and it carries the depth (the transcripts, the tables, the losers argued out) that a summary cannot hold. *Hierarchy invariant:* the long form is where the depth lives; the atom links to it and never duplicates it. | Content review against `RUNBOOK.md:307` (the question verbatim) and `RUNBOOK.md:264-268` (an ADR that cannot be stated as one question is two ADRs); shape and length compared to the nearest precedent `references/adr/0029-msrv-raised-to-1-97-1.md`. Recorded in the story's implementation report with the question quoted and the three clause ids listed. |
| AC-002 | **GIVEN** a maintainer who trusts `redkiln validate --kb` to tell them the knowledge base is well-formed, **WHEN** ADR-0028 lands, **THEN** it reached `.kb/decisions/0028-<slug>.md` **only** by being staged as `.kb/_intake/0028-<slug>.md` and consumed by `/redkiln:kb-ingest` — never hand-written into `.kb/decisions/` — and the resulting atom carries real composed `KbFrontmatter` (`kind`, `status`, `authority_tier`, `adr_id`, `reversibility`, `phase`, `supersedes`/`superseded_by`, `summary`, `depends_on`, `related`, `source_paths`, `last_reviewed`) and a real body, not a stub that only points at the long form. *Presentation invariant:* the atom is a composed artifact in its own right. | `redkiln validate --kb` clean (`KbFrontmatter` conformance + accepted-decision immutability against `HEAD`) and `redkiln doctor` clean with exactly the six known `template-drift` advisories; plus `git log --diff-filter=A -- .kb/decisions/0028-*.md` showing the file first appears in the `kb-ingest` wave commit, not an earlier hand edit. Frontmatter field set compared against `.kb/decisions/0029-msrv-raised-to-1-97-1.md`. |
| AC-003 | **GIVEN** an adapter author who needs to know whether they are finished, **WHEN** they read ADR-0028's decision section, **THEN** they find either a decision or a refusal **stated in terms** — and if it refuses, the second half of the sentence is present too: *deletion is out of scope for `EventStore`, and here is what a deleted-from store looks like to a reader*, with the completeness instrument and the three recorded reader observations cited as that illustration. A refusal by omission fails this criterion. | Content review against `RUNBOOK.md:4626-4634` (phase 14's goal names both outcomes) and project DR-9; the refusal branch is checked for a citation to the instrument's committed path and to all three reader observations from `decision-model-and-ingest-observed` and `projection-runner-across-the-hole`. Recorded in the implementation report as *decided* or *refused* with the citing lines quoted. |
| AC-004 | **GIVEN** an application author choosing a contract before a database, **WHEN** they ask why the library does not simply expose a floor, **THEN** ADR-0028 names `earliest_position()` as a rejected alternative **on ES-39's own grounds** — a regulated purge is scattered, not a prefix; the low-position survivor that must outlive its neighbours; *"it ships looking correct until a claim runs long"* — rather than omitting it. | Content review against `spec/SPECIFICATION.md:4336-4349`, checking the rejection is argued in those terms and that ES-39's `Rejects:` line (shipping it on the strength of the prune case) is answered. Grep the ADR body for `earliest_position` and read the surrounding paragraph. |
| AC-005 | **GIVEN** an evaluator who wants to know what the alternatives cost, **WHEN** they read the alternatives section, **THEN** the remaining DA-7/DA-8 losers are each named with their surface and version class: a set of retained ranges (new method + new public type), a third outcome on condition evaluation (`crates/happenstance-core/src/error.rs`, reaching every caller's `match`), and the tri-state `contains_event_id` on a surface that already exists (`crates/happenstance-core/src/store.rs:268`) — all four `0.3.0`-class under 0.x. | Content review against `_decomposition.md:363-379` (DA-7's option table) and `:381-397` (DA-8); each of the four rows must appear with its surface and its version consequence. Cross-checked so that no row *recommends* a change — recommending is `surface-diff-and-the-ac-012-escalation`'s. |
| AC-006 | **GIVEN** an adapter author who wants to know which conformance rules would have caught a forgetting store, **WHEN** they read ADR-0028's evidence section, **THEN** the enumerated pass list is cited **per configuration** — suffix *and* scattered, per DA-1 — stated as *the set of rules that cannot tell a pruned store from a young one*, with the DA-3 outcome that actually held recorded and the other two named; and a null result ("all of them passed") is recorded as CF-27's predicted outcome and the strongest evidence for ES-39, never softened. | Content review against `spec/SPECIFICATION.md:8034-8060` (CF-27's stated experimental outcome) and `_decomposition.md:234-274` (DA-3); the ADR's citation must resolve to the committed artefact produced by `cf-27-experiment-and-recorded-pass-list`, and the rule names in the ADR must match that artefact exactly (no re-derivation, no rounding). |
| AC-007 | **GIVEN** a maintainer who arrives at the claim *"a reader fails loudly"* and wants to know whether anyone checked, **WHEN** they read ADR-0028, **THEN** all three reader observations appear as **observed output**, not predicted prose: `read_decision_model`'s last-*retained* match feeding `AppendCondition::after_opt` into an admitted append; `IngestStore::holds` answering `false` for an event this store minted; and the projection runner's actual state across the hole. | Content review against `crates/happenstance-core/src/store.rs:321-331`, `crates/happenstance-sync/src/ingest.rs:164` and `RUNBOOK.md:4658-4668` (*"observed … not asserted in prose"*); each of the three citations must point at the recorded output artefact of `decision-model-and-ingest-observed` / `projection-runner-across-the-hole`, with the actual value quoted. |
| AC-008 | **GIVEN** a maintainer comparing CF-27's clause text (*"holds only a suffix"*) to the instrument that was actually built, **WHEN** they read ADR-0028, **THEN** DA-1's widening to an arbitrary retained set is recorded **with its reason** — a suffix-only instrument structurally cannot falsify the floor — and with the ground that made the widening available: CF-27 is `[DEFERRED — owned by the pass that settles retention and deletion]` and this project is that pass. | Content review against `spec/SPECIFICATION.md:8034-8036` (CF-27's wording) versus `:4309-4311` / `:4326-4328` (ES-38/ES-39's *"scattered subset"*), and `_decomposition.md:150-171` (DA-1). The ADR must state the widening as deliberate, not present the two readings as if they agreed all along. |
| AC-009 | **GIVEN** an evaluator deciding in one sitting whether the completeness axis is covered, **WHEN** they read ADR-0028, **THEN** the CF-25/CF-26 residual is recorded **open**: falsifiability is discharged by a fixture instrument, implementability is not, and the *"device adapter second"* half of the completeness axis stays outstanding and out of this project's scope. Reporting the axis as covered fails this criterion. | Content review against `spec/SPECIFICATION.md:8003-8032` (CF-26) and `:8090-8098` (the completeness axis), and `_decomposition.md:205-213` (DA-2). Checked by reading the ADR for an explicit "still open" sentence naming the adapter half; absence, or a sentence that implies closure, is a fail. |
| AC-010 | **GIVEN** a maintainer who owns replication and must not have it re-decided under them, **WHEN** they read ADR-0028 and the PR diff, **THEN** SY-32 is cited **by id** as answered by `replication-identity-and-ingest` and never re-derived, any change to what a peer must report is recorded *against SY-32*, and no file under `.kb/decisions/0026-*` or `0027-*` is touched by this diff. *(Project AC-016.)* | Citation check per the testing brief's AC-016 row: read ADR-0028 for an SY-32 citation by id against `spec/SPECIFICATION.md:6771-6799`; then `git diff --name-only main...HEAD` must list no path matching `.kb/decisions/0026-*` or `.kb/decisions/0027-*` (`_decomposition.md:618`). |
| AC-011 | **GIVEN** an application author who asked whether a `Tag` can be redacted, **WHEN** they read ADR-0028, **THEN** they get an answer: either the decision itself — grounded in the code DA-9 read, where `Tag` is `Tag(Cow<'static, str>)` with hand-written `PartialEq`/`Eq`/`Ord`/`Hash` all delegating to `as_str()`, no digest field and no store-side update path — or an explicit deferral naming a **real** path under `experiments/`. A deferral with no named experiment is a gate failure under CF-38 and fails this criterion. *(Project AC-010.)* | Content review against `spec/E2E-CASES.md:1288-1311` (E2E-49), `crates/happenstance-core/src/tag.rs:79` and `:170-193`, and `_decomposition.md:446-466` (DA-9). If deferred, the named path is checked to exist alongside the existing `experiments/position-visibility` and `experiments/wire-format` siblings; CF-38's empty-falsifier prohibition is already a `spec-trace` check (`xtask/src/spec_trace.rs`). |
| AC-012 | **GIVEN** a maintainer following the ES-38 thread, **WHEN** they read ADR-0028, **THEN** sub-question 3 — *does `positions_are_not_reused_after_removal` get written against whatever removal-capable fixture arrives with CF-27, or does phase 14 need its own instrument decision first?* — is answered on the record as **what actually happened** in this project, and sub-questions 1 and 2 are visibly **not** answered anywhere in the diff. *(Project AC-015, first half.)* | Content review against `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md:87-89` for the answer, and a negative check that neither `read_from_a_gap_position`'s ownership nor the general `[FROZEN]`-clause-owner policy is settled in ADR-0028's text or in any atom this wave lands. |
| AC-013 | **GIVEN** a maintainer who follows the gap-read thread six months from now, **WHEN** they open `.kb/maps/open-questions-index.md` and click through, **THEN** they find the question **still open** rather than closed while they were not looking: a narrowed successor `open_question` atom, `status: accepted`, carrying only the `read_from_a_gap_position` ownership thread with `related` back to the original; the original at `status: superseded`, **not deleted**, its body left describing what was not known at the time, with `related` naming both ADR-0028 and the successor; and the index bullet at `.kb/maps/open-questions-index.md:109-113` re-annotated rather than removed. *State invariants:* resolution happens **in place** (the original record is preserved and still reachable, not occluded), the unanswered thread is **preserved** across the transition (nothing this project does not own is closed in passing), and the whole wave is **reversible** as one commit. *(Project AC-015, second half.)* | `redkiln validate --kb` clean after the wave; then read the three atoms directly: successor `status: accepted` + `related` → original; original `status: superseded` + `related` → {ADR-0028, successor} + body unchanged in substance; index bullet present and re-annotated. Mechanism checked against `.kb/open-questions/README.md:40-45` (*"do not rewrite a question into its own answer"*) and the status enum at `.kb/README.md:21`. The wrong implementation this rejects is named in the testing brief's AC-015 row: flipping the whole atom to `withdrawn`/`superseded` with no narrowed successor. |
| AC-014 | **GIVEN** an adapter author already building against `0.2.0`, **WHEN** this PR merges, **THEN** nothing they compile against moved: no file under `crates/` is touched, no `pub` item, signature or rustdoc changes, and `spec/SPECIFICATION.md` is untouched (every marker move is `marker-moves-and-spec-trace-green`'s) — and the wave itself is hygienic: it lands as **one** `kb-ingest` run under a suffixed wave id that does not overwrite `.kb/_governance/integration-waves/2026-08-10-intake` or `-2`, with `.kb/_intake/` cleared on success and `.kb/_intake/README.md` dropped at the approval gate. *Non-occlusion invariant:* resolving this question disturbs no unrelated part of the tree. | `git diff --name-only main...HEAD` lists no path under `crates/` and not `spec/SPECIFICATION.md`; `cargo xtask affected --base main` green (the five file-reading lints and `spec-trace` run unconditionally, so `spec-trace` staying green *without* this story having edited the specification is the real signal). Wave hygiene: a new directory under `.kb/_governance/integration-waves/` whose name collides with neither existing wave, and `.kb/_intake/` containing only `README.md` afterwards (`.kb/_intake/README.md:13-19`). |

## Interaction quality

**This story renders no user-facing surface, and that is a signed-off determination, not an
omission.** `.bklg/.../retention-and-incomplete-logs/_design.md` records `N/A — no user-facing
surface` for every surface section (Surfaces, Items, Signatures, Shape decision, Placement), on the
ground that this project ships no screen and its "readers" are `read_decision_model`, a projection
runner and `IngestStore::holds` — Rust code paths, not a screen a human looks at. There is no route,
no DOM selector and no component to enumerate, so the visual-composition family (density budget in
px/rem, persistent chrome versus revealed, hierarchy of visual weight) has **no applicable
invariants** here, and inventing numbers for it would contradict a design a human already approved.

What *does* apply is the state family, translated onto the artifact surface this story actually
renders — the knowledge base as a thing a maintainer navigates. Each applicable invariant is carried
by an **AC row above**, never by a bullet here:

| Invariant family | Invariant as it applies here | Carried by | How it is verified |
| --- | --- | --- | --- |
| In-place vs context-jump | The open question is resolved **in place**: its record stays at its existing path with a status transition, rather than being deleted and re-created somewhere else, which would break every inbound `related` edge and every reference in the maps. | AC-013 | Read the original atom at its unchanged path with `status: superseded`; `redkiln validate --kb` resolves its `related` edges. |
| Non-occlusion | Resolving this question hides nothing else: the original's body is left describing what was not known at the time, and no unrelated atom, clause or crate file is disturbed. | AC-013, AC-014 | Original body substantively unchanged; `git diff --name-only` shows no `crates/` path and no `spec/SPECIFICATION.md`. |
| Preserved selection / state | The half this project does **not** own survives the transition intact — `read_from_a_gap_position`'s ownership thread and the `[FROZEN]`-clause-owner policy question are carried forward into a narrowed successor that is still `accepted`, not silently closed. This is the "rename in place, preserving selection" of this story: *resolve the answered thread while preserving the unanswered one*. | AC-012, AC-013 | Successor atom exists, `status: accepted`, contains only those threads; negative check that neither is answered in the diff. |
| Reversibility | The entire wave — ADR-0028, the successor, the supersession, the map updates and the audit trail — lands as **one** commit on the worktree branch, so a revert restores the prior `.kb/` state and the staged `_intake` sources in one move. | AC-014 | One `kb-ingest` wave commit; `git revert` of it is a clean single-step restore. |
| Reachability (the keyboard-reachability analogue) | Every artifact this wave lands is reachable by following links from the maps a maintainer actually starts at: the index bullet is re-annotated rather than removed, and the Maps phase adds ADR-0028's row to `.kb/maps/decision-map.md`. Nothing is reachable only by knowing its filename. | AC-013, AC-002 | Index bullet present and updated at `.kb/maps/open-questions-index.md:109-113`; ADR-0028's row present in `.kb/maps/decision-map.md`. |
| Presentation exists at all | The atom is a **composed** artifact — real `KbFrontmatter`, a real summary, a real body — not a stub redirecting to the long form; and the two-file split is respected in the other direction too, with the depth in `references/adr/` rather than duplicated. | AC-001, AC-002 | Frontmatter field set compared to `.kb/decisions/0029-msrv-raised-to-1-97-1.md`; body read for substance. |
| Named anti-patterns | The design and briefs name three: (a) hand-writing atoms into `.kb/` — *the directory layout of the process without the process*, reverted at `0269720`; (b) flipping the bundled question to `withdrawn` and closing a thread this project does not own; (c) recording a *predicted* outcome in prose where an *observed* one was required. | AC-002, AC-013, AC-007 | Each anti-pattern is the named wrong implementation of its AC's verification row. |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | `/redkiln:kb-ingest` cannot express artifact (3) — the supersession of an existing `open_question` atom — as a mutation of that atom. | **Report it as a finding** in the implementation report and stop; do **not** hand-edit `.kb/`. `_decomposition.md:356-358` states this explicitly and `CLAUDE.md` cites the revert at `0269720` as the recorded cost of the alternative. The story is blocked, not worked around. |
| EC-002 | `redkiln validate --kb` fails on the accepted-decision immutability check against `HEAD`. | Treat as a signal that the wave mutated an accepted decision body. ADR-0028 is a **new** decision (`supersedes: null`) discharging an obligation ADR-0013 created; if the wave is proposing to edit an accepted atom's body instead, that is the defect — correct the wave, never the check. |
| EC-003 | The wave id collides with `.kb/_governance/integration-waves/2026-08-10-intake` or `-2`. | Suffix the wave id before running. A collision overwrites an earlier wave's audit trail, which is unrecoverable from the working tree (`_decomposition.md:356-361`). |
| EC-004 | `.kb/_intake/` still contains files other than `README.md` after the run. | Those files were **not** ingested (`.kb/_intake/README.md:13-19`). The run is incomplete; do not report the wave as landed. Re-run or report the failure — a partially-landed wave also breaks the single-commit reversibility NF-004 depends on. |
| EC-005 | The intake glob picks up `.kb/_intake/README.md` and proposes an atom for it. | Drop it at the approval gate (`_decomposition.md:356-361`). An atom minted from the intake directory's own README is noise in `.kb/` that a later wave has to supersede. |
| EC-006 | AC-011's redaction answer cannot be decided, and no experiment exists to defer against. | An unnamed deferral is a CF-38 gate failure by construction — not a bar this spec invented. Either the experiment path is created (alongside `experiments/position-visibility`, `experiments/wire-format`) and named, or the answer is decided. "Deferred, TBD" is not a landable state. |
| EC-007 | The evidence this story cites has changed since the upstream story recorded it (pass list, DA-3 outcome, or a reader observation). | Cite the committed artefact, not a remembered value. If they disagree, the upstream artefact wins and the divergence is reported — ADR-0028 does not re-derive evidence (Context pack 4). |
| EC-008 | The honest answer to ADR-0028 requires a port-surface change. | ADR-0028 states the surface, the version consequence and DA-7's option rows, and **stops**. Making the change here, or writing the escalation artefact here, is out of boundary — the escalation belongs to `surface-diff-and-the-ac-012-escalation` (project AC-012). |

## Non-functional

| id | requirement | rationale and check |
| --- | --- | --- |
| NF-001 | **Self-sufficiency at the atom grain.** The `.kb/decisions/` atom is readable on its own — question, decision, losers, residual — at roughly the ~100-line scale `CLAUDE.md` describes for atoms, with the long-form record carrying the transcripts and tables. The density split is the point: the atom is what `validate --kb` governs, the record is what a reader cites by `file:line`. | Compare against the precedent pair `.kb/decisions/0029-msrv-raised-to-1-97-1.md` + `references/adr/0029-msrv-raised-to-1-97-1.md`. |
| NF-002 | **Decide-in-one-sitting legibility.** An evaluator reading only ADR-0028 can tell, without opening another file, whether the library decided or refused, what lost, and what is still open. | Content review; the three questions must be answerable from the atom plus its `summary` field. |
| NF-003 | **Zero build impact.** This story maps to no workspace package. `cargo xtask affected --base main` must still be green, and its value here is the five file-reading lints plus `spec-trace` — which must pass *without* this story having edited `spec/SPECIFICATION.md`. | `cargo xtask affected --base main`; `.redkiln/config.yaml:40`. |
| NF-004 | **Single-commit atomicity.** The wave lands as one commit so it reverts as one; partial ingest runs are not an acceptable end state. | One wave commit on the worktree branch; EC-004's check. |
| NF-005 | **Citation integrity.** Every `file:line` citation in ADR-0028 resolves in the tree at merge time. The citations into `spec/SPECIFICATION.md` are the ones most at risk, because `marker-moves-and-spec-trace-green` edits that file downstream — prefer clause **ids** (ES-39, CF-27, SY-32) alongside line numbers where both would serve. | Spot-check each citation; prefer id-plus-line over line-only. |

## Implementation notes (non-prescriptive)

Not prescriptive — these are the traps, not a recipe.

- **Write the long form first, then distil the atom from it.** The reverse order produces an atom
  that is the whole decision and a record that is padding, which inverts the two-file convention
  `CLAUDE.md` describes and makes the record too thin to hold the line-range citations that will be
  made into it.
- **The branch is fixed by evidence, not chosen here.** The five upstream stories have already run.
  Read their recorded artefacts before drafting; if the pass list is "all rules passed", that is
  CF-27's predicted outcome and the strongest argument for ES-39, and the ADR should say so plainly
  rather than hedging.
- **Draft all three intake files before running the wave.** `/redkiln:kb-ingest` adjudicates across
  files and prefers merge/amend over new atoms; staging the successor and the supersession note
  alongside ADR-0028 is what lets it see the three as one resolution rather than three unrelated
  additions.
- **The narrowed successor is a new question, not a copy.** It carries sub-questions 1 and 2 and the
  context they need to stand alone — a successor that says "see the original" is a redirect, not an
  open question, and the maintainer following the thread lands nowhere.
- **Do not rewrite the original into its own answer.** `.kb/open-questions/README.md:40-45` says this
  in as many words. The original's body stays as the record of what was not known; only its
  frontmatter moves.
- **Slice ordering.** `dt-7-signal-shape-and-the-redaction-answer` merges first within
  `the-retention-decision`; its output is AC-011's input. Implemented in one context, the two land
  as one integrated result — a decision with an answered redaction question in it.
- **Where the ADR needs a surface, write the sentence and stop.** "This would require X on
  `EventStore`, which is a `0.3.0` under 0.x" is in scope. "Therefore we should do X" is the
  escalation story's.

## Tests and CI (merge gate)

Grounded in the project testing brief (`_decomposition.md` *Test mix* rows AC-008 `:610`, AC-010
`:612`, AC-015 `:617`, AC-016 `:618`) and its *Merge-gate commands* section. This story ships no
compiled test — its tiers are **static (process)**, **static (content review)** and **static
(citation check)**, which is what the brief assigns to exactly these four project ACs.

| tier | command / path | proves |
| --- | --- | --- |
| static (process) | `redkiln validate --kb` | `KbFrontmatter` conformance across the wave and accepted-decision immutability against `HEAD`. Rejects the named wrong implementation for project AC-008: ADR-0028 hand-written straight into `.kb/decisions/` (the `0269720` revert). Gates AC-002, AC-013. |
| static (process) | `redkiln doctor` | The backlog and KB are structurally sound, with exactly the six expected `template-drift` advisories — a seventh or a missing one is its own failure. Gates AC-002. |
| static (process) | `git log --diff-filter=A -- .kb/decisions/0028-*.md`; `ls .kb/_intake/`; `ls .kb/_governance/integration-waves/` | The atom's first appearance is the ingest wave commit; `_intake` is cleared but for its README; the wave id collides with neither existing wave. Gates AC-002, AC-014; rejects EC-003 and EC-004. |
| static (content review) | `references/adr/0028-<slug>.md` and `.kb/decisions/0028-<slug>.md`, read against `RUNBOOK.md:307`, `:264-268`, `:4626-4634` | One question, correctly scoped; a decision or a refusal **in terms** with its illustration. Rejects a refusal by omission. Gates AC-001, AC-003. |
| static (content review) | ADR-0028's alternatives section, read against `spec/SPECIFICATION.md:4336-4349` and `_decomposition.md:363-397` | `earliest_position()` rejected on ES-39's own grounds, and DA-7/DA-8's remaining rows named with surface and version class. Rejects the ADR that omits the only primitive anyone proposed. Gates AC-004, AC-005. |
| static (content review) | ADR-0028's evidence section, read against the upstream stories' committed artefacts and `RUNBOOK.md:4658-4668` | The pass list is cited per configuration and the three reader observations are **observed output**. Rejects the brief's named wrong implementation for project AC-006: a *predicted* outcome recorded in prose. Gates AC-006, AC-007. |
| static (content review) | ADR-0028 read against `spec/SPECIFICATION.md:8034-8036`, `:4309-4311`, `:8003-8032`, `:8090-8098` | DA-1's widening recorded with its reason; the CF-25/CF-26 residual recorded **open**. Rejects an ADR that reports the completeness axis as covered. Gates AC-008, AC-009. |
| static (content review) + `spec-trace`'s CF-38 check | ADR-0028's redaction paragraph; `experiments/<named-path>` if deferred; `xtask/src/spec_trace.rs` | AC-011 has an answer or a deferral against a **real** experiment. Rejects an unnamed deferral, which CF-38 makes a gate failure by construction. Gates AC-011. |
| static (citation check) | ADR-0028 read for an SY-32 citation by id (`spec/SPECIFICATION.md:6771-6799`); `git diff --name-only main...HEAD` | SY-32 consumed rather than re-derived, and `.kb/decisions/0026-*` / `0027-*` untouched (`_decomposition.md:618`). Gates AC-010. |
| static (atom read) | `.kb/open-questions/` (successor + original) and `.kb/maps/open-questions-index.md:109-113`, against `.kb/open-questions/README.md:40-45` and `.kb/README.md:21` | The three-artifact resolution: successor `accepted`, original `superseded` and not deleted, `related` both ways, index bullet re-annotated. Rejects the brief's named wrong implementation for project AC-015: a bare flip to `withdrawn` with no successor. Gates AC-012, AC-013. |
| story grain (automatic) | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | The repository still builds and lints. This story maps to no package, so the live signal is the five file-reading lints and `spec-trace` — green **without** this story having touched `spec/SPECIFICATION.md`. Gates AC-014, NF-003. |
| integration grain (project, not this story) | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | The non-terminal project bar. Named for completeness: it is the project's DoD, not this story's merge gate. The whole gate (`cargo xtask ci`) is `closeout-and-durable-audience`'s. |

## Risks and coupling (PR-scoped)

- **The ingest wave may not be able to express the supersession as a mutation** (EC-001). This is the
  single highest-probability blocker and it has a prescribed response — report the finding, do not
  hand-edit. Mitigation: draft all three intake files and read `.kb/_intake/README.md` before running
  the wave, so the shape is known before the approval gate rather than discovered at it.
- **Evidence drift between the upstream stories and this one.** Five stories feed this one; if any of
  their recorded artefacts moved after this story's context was assembled, ADR-0028 could cite a
  value that no longer holds. Mitigation: cite artefact paths and quote from them at authoring time
  (EC-007), and prefer clause ids over bare line numbers (NF-005).
- **Line-number citations into `spec/SPECIFICATION.md` will rot.**
  `marker-moves-and-spec-trace-green` edits that file downstream in this same project, and ES-39 and
  CF-27 are exactly the clauses it edits. Mitigation: id-plus-line, never line-only.
- **Scope pressure toward the escalation.** If ADR-0028 decides rather than refuses, everything ES-39
  can be answered *with* is a port-surface change, and the pull to "just write the escalation while
  we are here" is strong. It is `surface-diff-and-the-ac-012-escalation`'s (project AC-012), and
  writing it here would make AC-014's zero-surface claim untestable at the story grain.
- **Scope pressure toward the markers.** Similarly, ADR-0028 is what makes ES-39 and CF-27
  dischargeable, and moving their markers in the same PR would feel natural. It belongs to
  `marker-moves-and-spec-trace-green`; `spec-trace` staying green here *without* a specification edit
  is a signal this story would destroy by editing it.
- **Coupling to the slice-mate.** `dt-7-signal-shape-and-the-redaction-answer` produces AC-011's
  input. If it lands "deferred", this story inherits the obligation to carry a real `experiments/`
  path — which means the two must be sequenced within the slice, not merged blind.
- **Downstream coupling.** `cf-27-rule-or-recorded-refusal` reads ADR-0028's branch to decide whether
  `suffix_store_is_distinguishable_from_a_young_store` gets written at all, and
  `marker-moves-and-spec-trace-green` reads it to move three markers. An ambiguous ADR — one that
  neither decides nor refuses in terms — blocks both. AC-003 exists to make that ambiguity a story
  failure rather than a downstream surprise.

## Dependencies

**Blocks on** (all six must have merged; this story consumes their recorded artefacts, it does not
re-derive them):

| story slug | what arrives from it | consumed by |
| --- | --- | --- |
| `cf-27-experiment-and-recorded-pass-list` | The enumerated pass list per configuration (suffix and scattered, DA-1) and the committed instrument that is the refusal branch's illustration | AC-003, AC-006, AC-008 |
| `positions-are-not-reused-after-removal` | ES-38's rule written, registered and non-decorative with its mutant — the fact sub-question 3 is answered *with* | AC-012 |
| `condition-over-removed-history-does-not-reject` | ES-40's rule and its mutant; the vacuous-pass outcome the decision rests on | AC-006 |
| `decision-model-and-ingest-observed` | Two of the three reader observations as actual output: `read_decision_model` → `after_opt` → admitted append, and `IngestStore::holds` → `false` | AC-003, AC-007 |
| `projection-runner-across-the-hole` | The third reader observation: the projection runner's actual state across the hole | AC-003, AC-007 |
| `dt-7-signal-shape-and-the-redaction-answer` | The redaction answer (or the named experiment to defer against) — slice-mate, merged first | AC-011 |

**Unlocks:**

| story slug | what it takes from here |
| --- | --- |
| `cf-27-rule-or-recorded-refusal` | ADR-0028's branch — decided or refused — which fixes whether `suffix_store_is_distinguishable_from_a_young_store` is written at all and what it asserts (project AC-007) |
| `marker-moves-and-spec-trace-green` | The decision ES-39, CF-27 and SY-32 are each `[DEFERRED]` *on*, without which no marker can honestly move (project AC-014) |
| `surface-diff-and-the-ac-012-escalation` | The surface, version consequence and DA-7 option rows ADR-0028 states and stops at, which the escalation turns into a finding raised to the initiative (project AC-012) |

Project-level: this project depends on `replication-identity-and-ingest` (HS-P0017) for SY-32's
answer and the `IngestStore` seam, and transitively on `publication-and-positioning` (HS-P0016) for
the `0.2.0` baseline. It unlocks `closeout-and-durable-audience` (HS-P0019), where DoD 15 is
re-observed as part of the assembled set.

## Anchors (progressive disclosure)

Load-bearing depth, deferred not optional. Open each at the moment named — do not preload the corpus.

| anchor (real path) | why it is load-bearing | when to open | serves AC-### |
| --- | --- | --- | --- |
| `RUNBOOK.md` | `:307` carries ADR-0028's queue row with the one question verbatim and its scope (ES-39, CF-27, SY-32); `:264-268` is the "one question or it is two ADRs" rule; `:4626-4674` is phase 14 in full — goal, both outcomes, proof artefact, exit criteria; `:4658-4668` is the *"observed … not asserted in prose"* bar | Before writing the first sentence of the long form, and again before finalising the evidence section | AC-001, AC-003, AC-007 |
| `spec/SPECIFICATION.md` | ES-39 at `:4325-4349` argues against `earliest_position()` by name and supplies the scattered-purge argument and the *"ships looking correct until a claim runs long"* line; CF-27 at `:8034-8060` supplies the "suffix" wording DA-1 widened and the stated experimental outcome; CF-26 at `:8003-8032` and the completeness axis at `:8090-8098` supply the residual; SY-32 at `:6771-6799` is what is cited by id | When drafting the alternatives section (AC-004), the widening note (AC-008), the residual note (AC-009) and the SY-32 citation (AC-010) | AC-004, AC-006, AC-008, AC-009, AC-010 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md` | The architecture brief: DA-1 (`:150-171`, the widening and its reason), DA-2 (`:205-213`, falsifiability only), DA-3 (`:234-274`, the three outcomes and the dishonest resolutions), DA-6 (`:328-361`, the three-artifact wave and its operational notes), DA-7 (`:363-379`, the option table), DA-8 (`:381-397`, the tri-state `contains_event_id`), DA-9 (`:446-466`, redaction against the code); the testing brief rows at `:610`, `:612`, `:617`, `:618` name each AC's wrong implementation | DA-6 and DA-7 before staging the intake wave; DA-1/DA-2/DA-3 while drafting the evidence and widening sections; DA-9 before writing AC-011's paragraph; the testing rows before claiming any AC satisfied | AC-004, AC-005, AC-006, AC-008, AC-009, AC-011, AC-013 |
| `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md` | The atom being resolved. `:87-89` is sub-question 3 verbatim — the one this story answers; sub-questions 1 and 2 above it are the successor's body and must **not** be answered | Immediately before drafting the answer paragraph, and again when writing the narrowed successor | AC-012, AC-013 |
| `.kb/open-questions/README.md` | `:40-45`, *"Resolving one"* — the answer is a new atom, the record stays, the two link via `related`, status moves to `withdrawn`/`superseded`, and *"do not rewrite a question into its own answer."* This is the mechanism AC-013 is scored against | Before staging artifacts (2) and (3) of the wave | AC-013 |
| `.kb/maps/open-questions-index.md` | `:109-113` is the existing `**Open** — es-38-and-gap-read-rules-are-unowned.md` bullet that must be re-annotated rather than removed; the surrounding rows show the house annotation style for owned and superseded questions | When updating the index as part of the wave | AC-013 |
| `.kb/_intake/README.md` | `:13-19` (a successful ingest clears `_intake`; a file left behind was not ingested) and `:21-31` (what the intake glob picks up, hence dropping the README at the approval gate) | Before running `/redkiln:kb-ingest`, and again when verifying the run landed | AC-002, AC-014 |
| `.kb/decisions/0029-msrv-raised-to-1-97-1.md` | The nearest precedent atom: the exact `KbFrontmatter` field set an intake atom must be able to become, and the atom-grain density the ~100-line convention describes | When drafting the atom's frontmatter and judging how much belongs in it | AC-002 |
| `references/adr/0029-msrv-raised-to-1-97-1.md` | The other half of the same precedent pair — the long-form record's shape and length, and what depth belongs there rather than in the atom | When drafting the long form and deciding the split | AC-001 |
| `.kb/decisions/0013-position-assignment-and-visibility.md` | The accepted decision that settled ES-38 and created the "Owner: phase 14" obligation this story discharges. It is cited, never renegotiated — and being **accepted**, its body is checked against `HEAD` by `validate --kb` | When writing ADR-0028's context section, to state what obligation is being discharged | AC-001, AC-012 |
| `CLAUDE.md` | *Where the work lives* — the two-file convention (link the atom, cite the record by `file:line`), the immutability of accepted decision atoms, and the `0269720` revert that is the recorded cost of hand-writing into `.kb/` | Before deciding where any sentence goes, and before any temptation to edit `.kb/` directly | AC-001, AC-002 |
| `crates/happenstance-core/src/tag.rs` | `:79` and `:170-193` — `Tag(Cow<'static, str>)` with hand-written `PartialEq`/`Eq`/`Ord`/`Hash` delegating to `as_str()`, no digest field, no store-side update path. This is the code DA-9 read and the ground AC-011's answer stands on | When writing the redaction paragraph — read the code, do not paraphrase the brief | AC-011 |
| `spec/E2E-CASES.md` | `:1288-1311` — E2E-49, the redaction case AC-011's answer resolves | With `tag.rs`, when writing the redaction paragraph | AC-011 |
| `crates/happenstance-core/src/store.rs` | `:268` is `contains_event_id`, DA-8's tri-state candidate and one of DA-7's four option rows; `:321-331` is `read_decision_model`, the first reader observation's subject | When naming the losers (AC-005) and when citing the first reader observation (AC-007) | AC-005, AC-007 |
| `crates/happenstance-sync/src/ingest.rs` | `:164` is `IngestStore::holds`, the second reader observation's subject and the surface where `false` means *"never had it"* to every caller | When citing the second reader observation | AC-007 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md` | `:223-227` (AC-008), `:231-234` (AC-010), `:252-258` (AC-015 and AC-016) — the project ACs this story traces to; `:260-281` the boundary DoD they roll into; `:308-330` the risk notes on the surface constraint and the CF-25/CF-26 exposure | Before claiming any AC satisfied, to check this story's criterion still says what the project's says | AC-002, AC-010, AC-011, AC-013 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md` | The signed-off determination that this project renders **no** user-facing surface, with its grounds. It is why the Interaction quality section above carries no visual-composition invariants — a decision, not an omission to be corrected | Before writing anything that would render a surface, and if a reviewer asks where the composition ACs are | AC-014 |
| `.bklg/from-contract-to-published-library/initiative.md` | `:227-250` carries the four personas and the journeys this work improves — the framing every AC above is written from; `:402-405` is DoD 15, the scenario this story's *"on disk"* half advances | When framing or re-framing an AC, and when writing the implementation report's DoD claim | AC-001, AC-009 |

## Clarifications resolved during spec

1. **The AC set is exactly the fourteen the front half enumerated.** AC-001 – AC-014 are carried
   unchanged; none was added or dropped. Their distribution against the four traced project ACs:
   AC-001 – AC-009 serve project **AC-008** (the decision atom, through the ingest path, answering
   one question with its losers named); AC-010 serves project **AC-016** (SY-32 by id, ADR-0026/0027
   untouched); AC-011 serves project **AC-010** (the redaction answer or a named deferral); AC-012
   and AC-013 serve project **AC-015** (resolved rather than deleted); AC-014 is the boundary
   criterion that keeps the other thirteen honest — no `crates/` file, no specification edit, one
   hygienic wave.
2. **Nine story ACs against one project AC is deliberate, not padding.** Project AC-008 bundles five
   separable failure modes — wrong authoring path, wrong question scope, refusal by omission, a
   loser omitted, evidence softened — and a single row would let four of them pass. Each story AC
   above names a distinct wrong implementation drawn from the testing brief or the architecture
   brief.
3. **The refusal branch and the decision branch share one AC set.** AC-003 is written to accept
   either, and to fail only *ambiguity* or a refusal by omission. Splitting the ACs by branch would
   have forced this spec to pre-empt an outcome that the upstream evidence, not this story, fixes.
4. **Interaction quality is answered, not skipped.** `_design.md` records `N/A — no user-facing
   surface` as a signed-off determination, so the visual-composition family has no applicable
   invariants and none is invented. The state family does apply, translated onto the knowledge base
   as the artifact surface a maintainer navigates, and every applicable invariant is carried by an
   AC row (AC-002, AC-007, AC-012, AC-013, AC-014) rather than by a prose bullet — which is what
   makes them gated.
5. **AC-014 folds three separate zeros into one criterion** — no `crates/` file, no
   `spec/SPECIFICATION.md` edit, one clean wave — because they share a single verification
   (`git diff --name-only` plus the wave's own artefacts) and because splitting them would imply
   this story owns project AC-011's surface diff, which it does not: it contributes a zero to a
   comparison a sibling story performs.
6. **No compiled test is specified, and that is the brief's assignment, not a gap.** The testing
   brief maps project AC-008, AC-010, AC-015 and AC-016 to *static (process)*, *static (content
   review)* and *static (citation check)* tiers. The gate that fires automatically at this story's
   grain is `cargo xtask affected --base main`, whose live signal here is the file-reading lints and
   `spec-trace` — green *without* a specification edit.
7. **EC-001 is a blocker with a prescribed response, not a fallback path.** If the ingest wave cannot
   express the supersession, the story stops and reports. This spec deliberately offers no
   hand-editing escape hatch, because `_decomposition.md:356-358` and the `0269720` revert already
   ruled one out.
8. **Citation style is settled toward clause ids.** NF-005 asks for id-plus-line rather than
   line-only into `spec/SPECIFICATION.md`, because `marker-moves-and-spec-trace-green` edits exactly
   the clauses ADR-0028 cites most. This is a spec-level preference, not a repository convention
   change — `CLAUDE.md`'s *cite the record by `file:line`* still governs citations into
   `references/adr/`, which no downstream sibling edits.
