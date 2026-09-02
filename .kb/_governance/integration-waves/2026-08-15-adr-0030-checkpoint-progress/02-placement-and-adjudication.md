# Wave `2026-08-15-adr-0030-checkpoint-progress` — placement and adjudication

The ordered action plan. Five operations: one decision atom created, two open questions resolved,
one open question created, one reference atom created. **No accepted decision atom is edited, and
none is superseded.**

Order matters in one place only, and it is Op 1 → Ops 2 and 3: the two resolutions link to
`kb-decision-0030` by id, so the atom has to exist before the annotations that cite it. Ops 4 and 5
link outbound to atoms that already exist plus (Op 5) `kb-decision-0030`, so they follow Op 1 as
well. Ops 2 and 3 are the only mutating operations and run strictly serially.

```
Op 1  create_new     .kb/decisions/0030-the-checkpoint-reports-the-commits-that-happened.md
Op 2  merge_existing .kb/open-questions/ps-1-states-no-progress-obligation.md          (resolve)
Op 3  merge_existing .kb/open-questions/ps-19-scope-narrower-than-its-rule.md          (resolve)
Op 4  create_new     .kb/open-questions/ps-32-adr-0007-context-correction-is-owed.md
Op 5  create_new     .kb/reference/spec-trace-has-suite-family-switch.md
```

## Standing choices, applied to every atom in the wave

### 1. One atom per ADR, and the long-form record keeps the evidence

`references/adr/0030-…md` is 158 lines and carries the finding table, the compiled evidence, the
exposing implementation and four rejected alternatives. The atom is the ~100-line canonical form
and cites the record by `file:line`. Unchanged from waves 2 and 3, and it is what
`CLAUDE.md`'s *"link the atom; cite the record by `file:line`"* requires.

### 2. `status`, and the value the schema still does not have

PS-38 is `[PROVISIONAL]`. `KbFrontmatter`'s `status` enum has no "accepted, provisional", and
`kb-open-question-adr-status-vocabulary-001` records exactly that gap. The convention that question
observes is followed rather than answered: `status: accepted`, with the provisional qualification
**and its falsifier** folded into the first clause of `summary`, and at length under the atom's
`## Provisional` heading. **No edit to the status-vocabulary atom.** Third wave running.

### 3. `authority_tier`, `phase`, `reversibility`

`authority_tier: decision` for Op 1 (mirroring all twenty siblings); `authority_tier: note` for
Ops 4 and 5, mirroring every atom in `open-questions/` and `reference/`. `phase: 6` — the record
says *"phase 6 (unscheduled — the queue had no number for it, on ADR-0029's precedent:
`RUNBOOK.md:287`)"*, and the integer field cannot hold "unscheduled", so the qualification goes in
the body where ADR-0029's atom put its own. `reversibility: low` as the intake states: PS-38 is a
normative clause four rules already rest on, and unminting it would re-orphan them.

### 4. `source_paths` keeps the intake path

Every atom this wave writes or amends carries
`.kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md` in `source_paths`, even though the ingest
clears `_intake` afterwards. The path documents provenance; the git history holds the file. Wave 1's
rule, unchanged.

### 5. What is *not* extracted

No `crates/**` change. No `spec/SPECIFICATION.md` edit — the PS-38 clause, the four recorded
findings and §1.3's recount landed with the ADR before this wave ran, and the wave records them as
consequences rather than proposing them. No maturity marker moved. No `[FROZEN]` line edited. No
supersession of ADR-0007, ADR-0017, ADR-0018 or ADR-0019. No new conformance rule written —
`fresh_projection_has_no_checkpoint` still does not exist, and the atom says so.

### 6. Two open questions are **resolved**, not amended — and the bodies stay verbatim

This is the wave's one novel operation shape, and the layer README governs it in terms
(`.kb/open-questions/README.md:40-45`): the answer is a new atom, the record of the question stays,
`status` moves to `withdrawn` or `superseded`, the two are linked by `related`, and the body is left
describing what was not known at the time. `superseded` is chosen over `withdrawn` in both cases —
`withdrawn` would say the question stopped mattering, and both questions were answered. The exact
precedent is `kb-open-question-projection-batch-no-apply-001`, flipped to `superseded` by the
2026-08-13 wave with an "Answered 2026-08-13 by ADR-0017" clause appended to its `summary` and its
body untouched. **`superseded_by` is not added**: it is a decision-only key, the precedent atom does
not carry it, and inventing it on an `open_question` would put a key in the corpus that nothing
validates and every later wave reads as fact.

---

## Op 1 — ADR-0030, the decision atom

**op:** `create_new` · **kind:** `decision` · **classification:** `extends`
**destPath:** `.kb/decisions/0030-the-checkpoint-reports-the-commits-that-happened.md`
**sourceFiles:** `.kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md`

**Why `create_new` and not a merge.** No accepted atom owns the proposition. The closest three —
`kb-decision-0017`, `-0018`, `-0019` — are the phase-6 projection decisions, and each was verified
against the claim: none of them says a `commit` must advance anything, because the whole finding is
that **nothing said either way** (`references/adr/0030-…:28-31`). ADR-0030 has its own `adr_id`, its
own 158-line record and its own human sign-off; `adr_id` and the supersede pair have to identify one
atom.

**Why nothing is superseded.** `supersedes: null`, on the record's own first bullet: *"Amends
nothing, edits nothing. PS-1, PS-19, PS-21 and PS-22 are byte-identical before and after."* A
decision that mints a clause beside four others does not supersede them.

### Proposed frontmatter

```yaml
id: kb-decision-0030
title: The checkpoint reports the commits that happened
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0030
reversibility: low
phase: 6
supersedes: null
superseded_by: null
summary: >-
  Accepted with its single clause provisional and the falsifier named: PS-38 falls to a store that
  answers checkpoint from a replica which may lag its own commit, at which point the obligation
  narrows to a subsequent read through the same handle and every rule downstream gains a handle
  constraint; owned by the first projection adapter over storage this workspace does not control.
  Nothing in section 4 obliged a commit to advance anything — PS-1's MUST is a coupling, and a
  store that makes neither write durable satisfies it through the "or not at all" arm while three
  rules and a fourth that does not exist yet all assert progress. The decision mints PS-38 in
  section 4.7: a successful commit MUST advance id's checkpoint to position, and a ProjectionId no
  successful commit has named MUST read as Checkpoint::NeverRun. Both sentences are one
  proposition — the checkpoint reports the commits that happened; the first says a commit is
  visible in it, the second says nothing else is — which is what keeps this one decision rather
  than two. The exposing implementation is a store whose backing state lives per handle rather
  than per store, the fixture bug CLAUDE.md exists to forbid, and it is independent: no other PS
  MUST rejects it. Rejected: adding a sentence to FROZEN PS-1, which conflates coupling with
  progress and widens the admitted set; splitting PS-23's "exactly one", the only sentence that
  entails progress today and does so incidentally on a PROVISIONAL clause about fan-out scope;
  routing it to phase 7, which PS-1's own paragraph forbids; and folding all seven of the sweep's
  defective rows into one record, which is four questions too many for one ADR title. PS-1, PS-19,
  PS-21 and PS-22 are byte-identical across it and each gains a recorded finding; PS-8, PS-13,
  PS-28 and PS-29 are recorded and routed, not repaired here. No adapter gains or loses
  conformance today.
depends_on:
  - kb-decision-0007
  - kb-decision-0017
  - kb-decision-0018
related:
  - kb-decision-0019
  - kb-open-question-ps-1-no-progress-obligation-001
  - kb-open-question-ps-19-scope-narrower-001
  - kb-playbook-repair-frozen-clause-001
  - kb-playbook-one-decision-per-adr-title-001
source_paths:
  - .kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md
  - references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md
  - references/evaluation/ps-clause-pairing-sweep.md
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-15
```

**Edge justification.** `depends_on: kb-decision-0007` — the checkpoint is per `(store,
ProjectionId)` by ADR-0007, which is what makes "`id`'s checkpoint" a well-formed phrase in PS-38;
cited, not re-derived. `kb-decision-0017` — the batch shape and `ProjectionProbe` PS-38's rules run
against. `kb-decision-0018` — `Checkpoint::NeverRun` is ADR-0018's three-variant enum, and PS-38's
second sentence is a claim *about that variant*; without ADR-0018 the sentence has no referent.
`related: kb-decision-0019` — same phase, adjacent clause range, and the owner of PS-28/PS-29's
disposition, which this atom names and declines. The two playbooks are cited *by the record* as the
bars two rejected alternatives were judged against (`:134-139`, `:154-158`) — `related`, not
`depends_on`, because the decision applies their method rather than resting on their conclusions.

**Body sections the integrator should write:** `## Decision` (PS-38, both sentences, the one
proposition argument), `## Provisional` (the lagging-replica falsifier and its owner),
`## Alternatives rejected` (the four, each with its reason), `## Consequences` (§1.3's recount;
three pairings become sound and `fresh_projection_has_no_checkpoint` stays daggered; no adapter
changes conformance), `## What this decision does not repair` (PS-8, PS-13, PS-28, PS-29 with their
owners, and one sentence recording that PS-32's correction is still owed and is
`kb-open-question-ps-32-adr-0007-correction-owed-001`'s, not this decision's).

**mapsImpact:** `decisionMap: true` (a new `## 2026-08-15 …` wave section and one row),
`domainMap: true` (one sentence in the projection-store domain paragraph),
`openQuestionIndex: false` (Ops 2 and 3 carry that).

---

## Op 2 — PS-1's question is answered

**op:** `merge_existing` · **kind:** `open_question` · **classification:** `extends`
**destPath / mergeTargetPath:** `.kb/open-questions/ps-1-states-no-progress-obligation.md`
**sourceFiles:** `.kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md` · **score: 100**

The atom's own sub-question 1 asked: *"Is the progress obligation PS-1's to carry, or does it belong
on a new clause?"* ADR-0030 answers it — **a clause of its own** — and the record names why the
other two candidates lost. This is a resolution, not an amendment.

**Frontmatter delta only. The body above the annotation is never touched.**

```yaml
status: accepted  →  superseded
related:  + kb-decision-0030            # appended; existing entries kept in order
source_paths:
  + .kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md
  + references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md
last_reviewed: 2026-08-13  →  2026-08-15
summary: + a closing clause, in the same voice as the 2026-08-13 amendment already there:
  "Resolved 2026-08-15 by ADR-0030 (kb-decision-0030), which mints PS-38 rather than widening
  PS-1: a successful commit MUST advance id's checkpoint to position. Sub-question 1 is answered
  a clause of its own, sub-question 2 by reattributing commit_advances_the_checkpoint to PS-38,
  and sub-question 4 by the decision itself; PS-1's own text is byte-identical across it."
```

**Body: append one `## Resolved 2026-08-15 — …` section**, in the shape of the `## Amended
2026-08-13` section already present. It must carry, and must not carry more than: the resolution
(a clause of its own); PS-38's text; why the other two candidates lost — adding to `[FROZEN]` PS-1
conflates two independently falsifiable propositions and widens the admitted set, and splitting
PS-23 puts an obligation three rules rest on onto a `[PROVISIONAL]` clause scheduled to be
rewritten by an unrelated falsifier; that the misfiled-onto-PS-23 amendment from 2026-08-13
**stands**, and is in fact what turned *"add a sentence to PS-1"* into *"put it where the rules can
cite it"*; and that PS-1's sentence did not move.

**What the annotation must not do:** rewrite the question into its own answer. The refuted sentence
at `:48-51` stays refuted-in-place with its 2026-08-13 note; the 2026-08-10 paragraphs stay as
written. That is the layer's contract and it is also the only reason this atom is worth keeping
after the answer exists.

**mapsImpact:** `openQuestionIndex: true` — the bullet flips **Open → Resolved**. `decisionMap:
false`, `domainMap: true` (the domain map's open-questions sentence names this atom's status).

---

## Op 3 — PS-19's question is answered by the same record

**op:** `merge_existing` · **kind:** `open_question` · **classification:** `extends`
**destPath / mergeTargetPath:** `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md`
**sourceFiles:** `.kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md` · **score: 100**

Same shape as Op 2, and the two run separately rather than as one op because they are two atoms: the
wave has no cross-file cluster to collapse and collapsing two *destinations* is not dedup.

```yaml
status: accepted  →  superseded
related:  + kb-decision-0030
source_paths:
  + .kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md
  + references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md
last_reviewed: 2026-08-13  →  2026-08-15
summary: + "Resolved 2026-08-15 by ADR-0030 (kb-decision-0030): PS-19 keeps its post-reset scope
  and the never-seen-id obligation becomes PS-38's second sentence, a ProjectionId no successful
  commit has named MUST read as Checkpoint::NeverRun. Sub-question 1 is answered a new clause,
  sub-question 3 by section 4.7 rather than adjacency to PS-19; fresh_projection_has_no_checkpoint
  is now listed against both clauses and still does not exist, so it renders †."
```

**Body: append one `## Resolved 2026-08-15 — …` section** carrying: the answer to sub-question 1
(a new clause, not a widening) and to sub-question 3 (§4.7, grouped with the progress obligation
from the PS-1 question — which is the outcome that section speculated about and is worth saying so);
that the rule is listed against both clauses and remains unwritten; and that sub-question 2's
`isolated` verdict from 2026-08-13 stands untouched — **nothing here reopens it.**

**mapsImpact:** `openQuestionIndex: true` (Open → Resolved), `domainMap: true`, `decisionMap:
false`.

---

## Op 4 — PS-32: the correction that is owed, and now has an atom

**op:** `create_new` · **kind:** `open_question` · **classification:** `conflicts` /
`requires-new-decision`
**destPath:** `.kb/open-questions/ps-32-adr-0007-context-correction-is-owed.md`
**sourceFiles:** `.kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md`

### Proposed frontmatter

```yaml
id: kb-open-question-ps-32-adr-0007-correction-owed-001
title: ADR-0007's record overstates what cannot be written, and only a superseding atom may fix it
kind: open_question
status: accepted
authority_tier: note
summary: >-
  PS-32 is FROZEN and states that ADR-0007's Context MUST be corrected: a callback-driven pump can
  be written against the ProjectionStore port as it stands, and what cannot be written is the
  conformance suite. The sentence it rejects is at references/adr/0007-projection-runner-decodes.md
  line 37 — "the runner ADR-0006 relocated therefore cannot be written against the port as it
  stands, in either crate" — and it was falsified by compilation rather than by argument:
  PRESSURE-TEST section 3.4 builds the ADR's own indicative pump against projection.rs unchanged,
  because the callback's caller knows the concrete Batch. What is not decided is who performs the
  correction and in what atom. ADR-0007 is accepted and immutable, and the governance test asks
  whether an edit changes what a document asserts rather than whether it changes the document, so
  this is a superseding decision's act and never an edit; the KB atom kb-decision-0007 does not
  repeat the wrong sentence, so what is defective is the long-form record and the phase-2 work item
  in RUNBOOK.md that derives from it. ADR-0017 records the correction as owed and states its shape
  without performing it, ADR-0030's phase-6 clause disposition changes nothing about that, and this
  is the third wave to carry it forward unperformed. Forced by whoever writes the runner, since the
  sentence being corrected is about work RUNBOOK.md still schedules on its strength.
depends_on: []
related:
  - kb-decision-0007
  - kb-decision-0017
  - kb-decision-0030
  - kb-governance-referent-not-reasoning-001
  - kb-playbook-repair-frozen-clause-001
source_paths:
  - .kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md
  - references/adr/0007-projection-runner-decodes.md
  - references/adr/0017-what-a-projection-batch-owns.md
  - references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md
  - references/evaluation/PRESSURE-TEST.md
  - spec/SPECIFICATION.md
  - RUNBOOK.md
last_reviewed: 2026-08-15
```

**Body:** the layer's four-part shape. *What is true today* — the sentence, its location, the
compiled refutation (`PRESSURE-TEST.md:203-232`, the pump signature and the `checkpoint` → `begin`
→ per-event `apply` → `commit`/`rollback` body), what genuinely cannot be written (generic suite
code holding a `P::Batch<'_>` can only pass it to `commit` or `rollback`, so three of the six rules
cannot observe the read model), and the finding that `kb-decision-0007`'s **atom** does not carry
the defective sentence — only the record does. *What is not decided* — whether the correction rides
a decision that supersedes ADR-0007 outright, or a partial supersession of the ADR-0006 → ADR-0007
lineage's shape, and whether `RUNBOOK.md:204-208`'s phase-2 item goes with it. *What forces it* —
whoever writes the runner; and, secondarily, that the specification carries a `[FROZEN]` `MUST`
which no atom has owned for three waves. *Ordered sub-questions* — (1) supersede in full or in
part; (2) does the correction also move the RUNBOOK item, or is that a separate act; (3) does
correcting the Context disturb ADR-0007's own falsifier at PS-33, which is a different and still
`[DEFERRED]` question.

**mapsImpact:** `openQuestionIndex: true` — a new bullet under *Specification governance &
conformance*, per that index's *"Adding an entry"* section. `domainMap: true`. `decisionMap: false`
— nothing was decided.

---

## Op 5 — `has_suite` is a per-family switch

**op:** `create_new` · **kind:** `reference` · **classification:** `extends`
**destPath:** `.kb/reference/spec-trace-has-suite-family-switch.md`
**sourceFiles:** `.kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md`

### Proposed frontmatter

```yaml
id: kb-reference-spec-trace-has-suite-001
title: spec-trace's has_suite is a per-family switch, and PS sat outside it for two slices
kind: reference
status: accepted
authority_tier: note
summary: >-
  As of 2026-08-15, cargo xtask spec-trace decides per clause family whether to check the
  conformance-rule names a clause cites, through has_suite in xtask/src/spec_trace.rs; a family
  absent from that switch is a family whose rule citations nothing checks. PS was excluded when the
  projection suite did not exist, the exclusion outlived the suite by two slices, and until it was
  flipped a PS clause could cite a rule that had never existed with every gate in the repository
  green. The flip is now held by spec_trace::tests::the_projection_family_is_checked_against_its_suite
  and the one family that still abstains, SY, is held by the test beside it, so widening the switch
  to everything is a build failure rather than a judgement call. The transferable shape is an
  exclusion written for a true reason, kept after the reason expired, and structurally invisible
  because the thing it disables is itself a check — the same shape the ES-7 and VT-9 markers carry
  one level up, and the property a ratchet's fail-on-discharge exemption list exists to supply and
  this switch did not have.
depends_on: []
related:
  - kb-decision-0030
  - kb-open-question-provisional-falsifiers-001
  - kb-playbook-ratchet-gate-landing-001
  - kb-reference-phase-4-5-spec-reconciliation-001
source_paths:
  - .kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md
  - references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md
  - xtask/src/spec_trace.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-15
```

**Body:** what the switch is and where (`xtask/src/spec_trace.rs:1750`, and check 4's abstention at
`:699`); the dated fact, with the commit or date stated **in the body** as the reference README's
dating rule requires; the two tests (`:2363`, `:2387`) and what each holds; and the transferable
shape in one paragraph, stated as an observation rather than as a rule — a `reference` atom binds
nothing, and the moment this atom starts telling someone what to do it belongs in `playbooks/`.

**mapsImpact:** `domainMap: true` (a reference-atom entry in the specification-governance domain).
`decisionMap: false`, `openQuestionIndex: false`.

---

## Adjudications

### 1. Why the two resolutions are `merge_existing` and not `supersede`

The extract proposed `supersede` for Ops 2 and 3, and the word is right in English and wrong in this
plan's vocabulary. `supersede` here means *a new **decision** atom carrying `supersedes: [<old
id>]`, with the old decision's frontmatter flipped*. Ops 2 and 3 touch no decision: they flip an
`open_question`'s `status` to `superseded` and append a dated section, which is an amendment to a
`note`-tier atom — the operation wave 3 ran on `kb-open-question-projection-batch-no-apply-001` and
recorded as `merge_existing`. Filing them as `supersede` would put two supersessions in the wave
manifest that no decision performed, and the next wave reading the manifest would look for a
superseding decision atom that does not exist.

The substantive rule underneath: **`kb-decision-0030` does not carry `supersedes`.** ADR-0030
supersedes nothing. It answers two questions, which is a different relation and is carried by
`related` in both directions.

### 2. Claim 5 produces no operation, and that is a verified finding rather than an omission

The intake's four-row table (PS-8, PS-13, PS-28, PS-29) scored 80 and 85 against `kb-decision-0017`
and `kb-decision-0019`, which under the default bias reads as *merge into the owner atom*. It cannot
be, twice over:

1. **Both are accepted decision atoms.** `redkiln validate --kb` checks each `status: accepted`
   decision body against `HEAD`. There is no merge that is not a gate failure.
2. **They already say it.** Verified in the atoms, not inferred from titles.
   `kb-decision-0017` records PS-8's and PS-13's clause/rule pairing defects as gaps it names and
   repairs not; `kb-decision-0019` covers PS-28 as `undetermined` and PS-29 as `defective` and
   deferred to `unstable-projection-gate-and-clause-disposition`. The intake itself says the four
   findings now live *in the specification clauses they concern*, which is `spec/`, not `.kb/`.

So the claim is **confirmatory**. What it earns is one sentence inside Op 1's atom — *these four are
not this decision's* — which is a scope boundary a decision is required to state, and a line in this
plan so that the future wave the intake warns about ("a future wave should not fold these into
ADR-0030") meets the warning where it will be looking.

### 3. PS-32 becomes an atom rather than a third `unresolved` line — the wave's one judgement call

The 2026-08-13 wave carried PS-32 in `unresolved`. This wave could have done the same, and the
argument for doing so is real: the intake does not ask for an atom, and inventing destinations the
source did not ask for is how a corpus grows noise. Four things decide it the other way:

1. **It is a conflict, and the ingest rule routes conflicts here.** Not a preference between two
   readings — a sentence falsified by a compiler, with the specification's own `Rejects` field
   naming it (`spec/SPECIFICATION.md:5699`).
2. **The specification asks for it, in a `[FROZEN]` `MUST`, and names this file.**
   `spec/SPECIFICATION.md:5679` states the correction as owed; `:5714` says *"The staging note for
   the next wave is `.kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md`."* This is that wave.
3. **`unresolved` is a wave-local channel and this obligation has outlived two waves.** A finding
   that recurs in three consecutive manifests and lands in no atom is a finding the corpus is
   losing; `open-questions/README.md` exists precisely so *"we looked, and this is genuinely open"*
   has somewhere to land instead of being rediscovered.
4. **Opening it settles nothing.** The atom records the gap; it does not correct ADR-0007, does not
   flip its status, does not author the superseding decision. Every immutability rule is untouched.

The distinction from claim 7, which stays in `unresolved`: claim 7 is an *observation with an
instrument that already exists* and its source withholds mandate in terms. There is no conflict
there, and no `MUST` anywhere asking for it.

### 4. Claim 7 (PS-3 / PS-31 / PS-36) is declined, visibly

*"Worth an atom; not opened by this story, which had no mandate to."* The source's restraint is
honoured: no atom, no op. It goes in `unresolved` so a human can overrule in one line rather than
having to rediscover the candidate — the same handling wave 3 gave the two candidates the sweep
declined, one of which was **this same candidate**, raised at `ps-clause-pairing-sweep.md:433-454`
and declined then too. Twice declined by its own sources is not the same as forgotten, and the
second `unresolved` entry is what keeps the difference visible.

### 5. The reference atom is created rather than merged, and the dating rule is why

`kb-reference-phase-4-5-spec-reconciliation-001` scored 55 and is the only plausible host. Wave 3
already declined it once as *"precedent, not a merge target"*. The second, stronger reason is the
reference README's dating rule: that atom is a census **at 2026-08-10**, pinned to six commits, and
its value is that it is a snapshot someone can compare a later count against. Appending a
2026-08-15 fact about a switch that changed after the census would convert a snapshot into a mirror
— exactly what the README says does not belong in the layer, because *"nobody will [maintain it],
and a mirror nobody maintains is worse than no mirror because it looks maintained."*

The `kb-playbook-ratchet-gate-landing-001` edge is worth stating rather than leaving to a reader:
`has_suite` is the same instrument that playbook describes — a named exemption list inside
`spec_trace.rs` — **minus the property the playbook says makes one safe**, that it fails when an
entry becomes discharged so the list can only shrink. `PS`'s entry became discharged and nothing
failed. That is the sharpest available worked counter-example to the playbook's own method, and it
is why the edge is `related` on the reference atom rather than a new paragraph inside the playbook:
the playbook is `authority_tier: guideline` and already carries its method; what this wave has is a
dated instance, and instances belong in `reference/`.

### 6. Nothing is routed to `product/`, `design/`, `concepts/` or `playbooks/`

Checked rather than assumed. `product/` and `design/` hold READMEs only and this wave carries no
persona, journey or interaction pattern. `concepts/` was considered for the *"the checkpoint reports
the commits that happened"* proposition and refused: it is a `MUST`, and
`.kb/decisions/README.md:27-29` says a commitment in a `concept` or `playbook` atom is not a
commitment, because those layers carry no immutability. `playbooks/` was considered for claim 8's
transferable shape and refused under Adjudication 5.

### 7. The index bullets read `Superseded`, not `Resolved` — one word, and it is the atom's own field

The intake says both rows *"move from open to resolved"*. Taken literally that introduces a fourth
label into a map whose own *"Adding an entry"* section names three — `Open`, `Withdrawn`,
`Superseded` — and none of the four is `Resolved`. Worse, the index's job is to print the atom's
`status`, and `KbFrontmatter`'s enum has no `resolved` either: Ops 2 and 3 write `superseded`,
because that is one of the two values `open-questions/README.md` permits.

So the bullet label is **`Superseded`**, matching the atom, the schema, the index's instruction and
the `kb-open-question-projection-batch-no-apply-001` precedent; the *word* "resolved" belongs in the
sentence after it — *"Resolved 2026-08-15 by ADR-0030 (`kb-decision-0030`) …"* — which is exactly
how the existing bullet reads ("Answered 2026-08-13 by ADR-0017"). This is the same discipline
standing choice 2 applies to `status`: where the intake's English and the schema's vocabulary
diverge, the schema takes the field and the English takes the prose.

---

## What the Maps phase inherits

| Map atom | Change | From |
| --- | --- | --- |
| `kb-map-decision-001` | A new `## 2026-08-15 the checkpoint's progress obligation (ADR-0030)` section and **one row**: `ADR-0030` · `kb-decision-0030` · *The checkpoint reports the commits that happened* · `accepted` · `6` · `—`. The section prose should say what the table cannot: that ADR-0030 supersedes nothing, that PS-1/PS-19/PS-21/PS-22 are byte-identical across it, and that its `Status` reads `accepted` for the same reason the 2026-08-13 section gives — no enum value spells "accepted, provisional", and printing one would answer `kb-open-question-adr-status-vocabulary-001` by acting on it | Op 1 |
| `kb-map-open-questions-index-001` | **Two bullets flip `Open` → `Superseded`** (not "Resolved" — Adjudication 7), each keeping its 2026-08-13 annotation and gaining one sentence naming ADR-0030 and what it answered. Neither bullet is removed — the index's own summary already states that a resolved question stays listed. **One new bullet**, `Open`, for `kb-open-question-ps-32-adr-0007-correction-owed-001`, under *Specification governance & conformance*. The atom's `summary` should record that this is the first wave to resolve a question it did not also file | Ops 2, 3, 4 |
| `kb-map-domain-001` | One sentence in the projection-store paragraph — ADR-0030 joins ADR-0017–0019 in that domain, phase 6, minting PS-38 — and the domain's open-question and reference listings pick up the two resolutions, the new PS-32 question and `kb-reference-spec-trace-has-suite-001` | Ops 1–5 |

Reciprocal links the Maps/backlink phase must wire, none of which any op writes twice:

- `kb-decision-0030` ↔ `kb-open-question-ps-1-no-progress-obligation-001` (both `related`)
- `kb-decision-0030` ↔ `kb-open-question-ps-19-scope-narrower-001` (both `related`)
- `kb-decision-0030` → `kb-decision-0007`, `-0017`, `-0018` (`depends_on`; **the three accepted
  atoms gain nothing** — an accepted decision's body and frontmatter are not edited to record an
  inbound edge, and the map is where the relation becomes navigable)
- `kb-open-question-ps-32-adr-0007-correction-owed-001` → `kb-decision-0007`, `-0017`, `-0030`,
  `kb-governance-referent-not-reasoning-001`, `kb-playbook-repair-frozen-clause-001` (outbound only,
  same reason)
- `kb-reference-spec-trace-has-suite-001` → `kb-decision-0030`,
  `kb-open-question-provisional-falsifiers-001`, `kb-playbook-ratchet-gate-landing-001`,
  `kb-reference-phase-4-5-spec-reconciliation-001` (outbound only; the two `note`-tier atoms *may*
  take a reciprocal edge, and this plan does not ask for one — neither claims the reference)

## Validation the integrate pass must clear

```
redkiln validate --kb    # KbFrontmatter on 3 new atoms; accepted-decision immutability on 20
redkiln doctor           # expects exactly six template-drift advisories, per CLAUDE.md
```

`validate --kb` is the check that would catch this wave's one plausible failure mode: an edit to an
accepted decision body absorbing claim 5 or claim 6. Neither op touches one, and the plan's count of
**zero** accepted-decision edits is the assertion to re-check after integration rather than before.
