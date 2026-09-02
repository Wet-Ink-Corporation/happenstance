# Wave `2026-08-17-adr-0022-append-condition` — placement and adjudication

The ordered action plan. Fourteen operations: four decision atoms created, three reference atoms
created, four open questions created, one open question resolved, one accepted decision flipped in
frontmatter only, one reference register extended. **No body of any accepted decision is edited.**

Order is load-bearing in four places, and the rule is the one every wave uses: an atom a later op
links to is created first, and mutating ops run strictly serially.

```
Op  1  create_new     .kb/reference/append-condition-experiment-2026-08.md
Op  2  create_new     .kb/open-questions/es-17-two-adapter-measurement-is-unscheduled.md
Op  3  create_new     .kb/decisions/0022-append-condition-strategy.md
Op  4  create_new     .kb/decisions/0031-the-runner-collapses-upward.md
Op  5  merge_existing .kb/open-questions/ps-32-adr-0007-context-correction-is-owed.md   (resolve)
Op  6  create_new     .kb/decisions/0032-adr-0021-serde-attribution-correction.md
Op  7  merge_existing .kb/decisions/0021-payload-evolution-and-codec-tag.md   (frontmatter only)
Op  8  create_new     .kb/open-questions/d-1-the-validated-type-has-no-total-path.md
Op  9  create_new     .kb/reference/phase-7-macros-ceremony-measurement.md
Op 10  create_new     .kb/decisions/0033-happenstance-macros-out-of-scope-for-0-1.md
Op 11  create_new     .kb/open-questions/cf-36-names-a-cross-reference-nothing-performs.md
Op 12  create_new     .kb/open-questions/no-ps-rule-name-is-resolved.md
Op 13  create_new     .kb/reference/projection-fan-out-costs-n-reads.md
Op 14  merge_existing .kb/reference/port-traits-compiled-findings.md
```

- **Ops 1 and 2 before Op 3** — ADR-0022's `related` names both by id.
- **Op 4 before Op 5** — the resolution annotates PS-32's question with `kb-decision-0031`.
- **Op 6 before Op 7** — the flip writes `superseded_by: kb-decision-0032`.
- **Ops 8 and 9 before Op 10** — ADR-0033's `related` names the D-1 question and its `depends_on`
  names the measurement.

Ops 5, 7 and 14 are the only mutating operations and run strictly serially, in that order.

---

## Standing choices, applied to every atom in the wave

### 1. One atom per ADR, and the long-form record keeps the evidence

Unchanged from waves 2, 3 and 4, and it is what `CLAUDE.md` requires: *"link the atom; cite the
record by `file:line`."* Applies cleanly to ADR-0022, whose 35 KB record exists. It applies
*aspirationally* to ADR-0031, ADR-0032 and ADR-0033, which have no `references/adr/` record —
Adjudication 8.

### 2. `status`, and the value the schema still does not have

ADR-0022 carries a live re-open trigger and two named non-verdicts; ADR-0031's collapse rests on a
falsifier that fired. `KbFrontmatter`'s `status` enum still has no "accepted, provisional", and
`kb-open-question-adr-status-vocabulary-001` records exactly that gap. The convention that question
observes is followed rather than answered: `status: accepted`, with every qualification and its
falsifier folded into `summary` and into a body heading. **No edit to the status-vocabulary atom.**
Fourth wave running.

### 3. `authority_tier`, `phase`, `reversibility`

`authority_tier: decision` for Ops 3, 4, 6 and 10, mirroring all twenty-three siblings.
`authority_tier: note` for every `reference` and `open_question` atom, mirroring every atom in
those two layers. `phase` is taken from the source where it states one — ADR-0022 `phase: 8`,
ADR-0032 `phase: 7` (matching the atom it supersedes), ADR-0031 and ADR-0033 `phase: 7` (the typed
layer, which is the phase whose exit produced both). `reversibility` is taken from the source where
stated; where it is not, Adjudication 9 states what the wave proposes and why.

### 4. `source_paths` keeps the intake path

Every atom this wave writes or amends carries its originating `.kb/_intake/…` path, even though the
ingest clears `_intake` afterwards. The path documents provenance; the git history holds the file.
Wave 1's rule, unchanged, and it is why an atom minted from two intake files lists both.

### 5. `last_reviewed: 2026-08-17` on everything this wave touches

Including the three mutated atoms. On Op 7 that is the *only* field besides `status` and
`superseded_by` that moves, and it is not a body edit.

### 6. What is *not* extracted

No `crates/**` change. **No `spec/SPECIFICATION.md` edit** — `0033` says its record mints no clause
and amends none; `0032` says PS-32 and PS-33 stay as `[NON-NORMATIVE]` and must not be deleted; the
defect log's four clause findings become questions, not clause edits. No maturity marker moves. No
`[FROZEN]` line is touched. **No layer README is edited**, though five claims across three files
score 65–85 against one: those READMEs are the authority being obeyed, and absorbing a compliance
record into a contract is how a contract becomes a changelog.

Two long-form edits are named by the intake and are **not** this wave's `.kb/` ops:
`references/adr/0021-…`'s rule-3 appendix is **already applied** (verified: 499 lines, appendix at
`:458`), and `references/adr/0007-…` **should be amended** with PS-32's correction in the same wave
— that file is mutable, it is not an atom, and the amendment is recorded here as owed to the
integration step rather than to the atom author. Adjudication 3.

### 7. Reciprocal links are the Maps phase's, not the atom author's

Every new atom is authored with **outbound-only** links. Where a mutual edge is wanted —
`kb-decision-0022` ↔ `kb-reference-append-condition-experiment-001`, `kb-decision-0033` ↔ the D-1
question — the second half is wired by the Maps phase, as in wave 4.

---

## Adjudication 1 — the ADR number collision, and how it is broken

**Two intake files claim ADR-0031.** `0031-adr-0021-serde-attribution-correction.md` names
`.kb/decisions/0031-adr-0021-serde-attribution-correction.md` as its mint path and reasons that
*"0031 is the next free slot … 0030 is the highest taken."*
`0032-adr-0031-the-runner-collapses-upward.md` carries the claim in its own filename. Neither knows
about the other; they were staged a day apart by different stories. Ingested independently, the
second would have overwritten the first at one path, and the corpus would carry one atom where two
decisions were taken.

**The tiebreak is not staging order, and not either file's assertion.** It is the repository's own
allocation, which exists for one of them and not the other:

```
.bklg/from-contract-to-published-library/_implementation.md:704
  4. **ADR-0031 is staged intake only** (N-2) — `.kb/_intake/0032-adr-0031-the-runner-collapses-upward.md`;
     `.kb/decisions/` stops at 0030 and `kb-decision-0007` still reads `superseded_by: null`.
```

That names the number, the file and the atom. Nothing anywhere names a number for the serde
correction; its own document explicitly defers — *"the wave assigns the final id."* So:

| Decision | Number | Fixed by |
| --- | --- | --- |
| The append-condition strategy | **ADR-0022** | `references/adr/0022-append-condition-strategy.md`, which already exists |
| The runner collapses upward | **ADR-0031** | `.bklg/…/_implementation.md:704`, by filename |
| The ADR-0021 serde attribution correction | **ADR-0032** | highest-taken + 1 |
| `happenstance-macros` out of scope for 0.1 | **ADR-0033** | highest-taken + 1 |

Serde before macros on staging order: `0031` was staged 2026-08-15, the macros verdict 2026-08-16.

**Why 0023–0028 are not backfilled.** `.bklg/…/_plan.md:169` reserves **ADR-0023** for the
Cloudflare project's `adr-0023-and-atom-resolutions` story, whose AC-006 is *"ADR-0023 accepted."*
Taking 0023 here would collide with a planned initiative. 0024–0028 are genuinely free, but the
corpus's own convention — stated by both intake files and by `decision-map.md`'s tolerance of
non-monotonic numbering — is highest-taken + 1, with the gap left as the record of when 0029 and
0030 were minted out of band. Filling a gap nothing explains is how a number stops meaning a date.

**What the integrator must not do:** do not name the runner atom
`0032-…` because its intake file is prefixed `0032`. The intake prefix is a staging sequence, not
an ADR number — `0033` mints ADR-0022 and `0034` mints no ADR at all.

---

## Adjudication 2 — a full flip and a partial supersession, told apart by where the defect is

This wave performs both, and spells them differently. The distinction is not "how much of the
decision changed" — that test is unfalsifiable in argument and would have produced one spelling for
two different situations. It is **physical, and the corpus already made it**:

> `kb-open-question-ps-32-adr-0007-correction-owed-001`, body:
> *"the KB atom `kb-decision-0007` itself does not repeat the defective sentence … only the
> long-form record and the phase-2 work item in `RUNBOOK.md:204-208` derived from it carry the
> error."*

So the rule this wave writes down and applies twice:

**Flip the old atom when the atom's own body carries the defect. Leave it `accepted` when only the
long-form record does, and carry the correction in the new atom's Context.**

| | ADR-0032 supersedes ADR-0021 | ADR-0031 supersedes ADR-0007's pump allocation |
| --- | --- | --- |
| Where the defect is | **In the atom.** `kb-decision-0021:141-144` states the ADR-0003 attribution backwards, verbatim | **In the record only.** `references/adr/0007-…:37`. The atom's summary and body are silent on the generic-code claim |
| Is a half left standing under the old atom's own authority? | **No.** The new atom restates the rejection on its two sound grounds and is where a reader must land | **Yes.** The discriminator and all three shape decisions stay ADR-0007's, uncorrected and implemented as written |
| `supersedes` on the new atom | `[kb-decision-0021]` | **`null`** |
| Old atom's `status` | `accepted` → **`superseded`** | stays **`accepted`** |
| Old atom's `superseded_by` | **`kb-decision-0032`** | stays **`null`** |
| Edge carried by | `supersedes` / `superseded_by` | **`depends_on: [kb-decision-0007]`** |
| Precedent | `kb-decision-0002` → `kb-decision-0005`, the only outright supersession in the corpus | `kb-decision-0005` → `0006` → `0007`, and `decision-map.md`'s *Reading the partial-supersession chain* |

The partial spelling is the corpus's own, in terms:

```
.kb/maps/decision-map.md, "Reading the partial-supersession chain"
  Each later atom's `depends_on` names the one it corrects; none of the three is
  `status: superseded` in full, because each still has a half standing.
```

`kb-decision-0007` itself is the worked example: it partly supersedes ADR-0006, and it carries
`supersedes: null` with `depends_on: [kb-decision-0006]`, while `kb-decision-0006` carries
`superseded_by: null` and opens its own summary *"Partly superseded by ADR-0007."*

**This diverges from what `0032`'s intake instructs**, and the divergence is deliberate and
flagged. That file says *"write a new atom whose `supersedes` names it, and set the old atom's
`superseded_by` through the CLI"*, and `.bklg/…/_implementation.md:704` reads
`kb-decision-0007`'s `superseded_by: null` as owed rather than as correct. The wave declines both
on the corpus precedent above, and on the stronger ground that flipping ADR-0007 to `superseded`
would retire three shape decisions that are **implemented and in force today** — `Query`
nominating events, `Projection::Store` as an associated type, checkpoints per
`(store, ProjectionId)` — one of which `kb-decision-0030` `depends_on` directly. A reader landing
on a `superseded` ADR-0007 would have no atom asserting them.

**If the human prefers the full spelling**, the metadata flip is the only permitted edit and can be
applied without touching a body: `status: superseded`, `superseded_by: kb-decision-0031`. It is
reversible in one edit and costs nothing to defer, which is the last reason to take the narrower
option now. Carried in `unresolved` so the choice stays visible rather than being settled by this
file's confidence.

---

## Adjudication 3 — PS-32 is resolved, on its fourth wave

`kb-open-question-ps-32-adr-0007-correction-owed-001` asked two things, and ADR-0031 answers both:

1. *"Who performs the correction, and in which atom"* — **`kb-decision-0031`**, which is exactly the
   *"superseding decision atom's act"* the question named as the only legal route, and exactly the
   shape it predicted: *"whether the eventual superseding atom is a full supersession or a partial
   one that corrects the record and the runbook item while leaving the rest of ADR-0007's
   split-at-the-decode-boundary decision untouched."* **Partial**, per Adjudication 2.
2. *"Whether the same atom, or a different one, finally writes ADR-0007's own long-deferred
   callback-driven pump"* — **neither. No pump is written**, and the reason is the falsifier: it was
   allocated by an ADR, three phases passed, and it was never written because the typed runner can
   drive the port directly. The question is answered in the negative, which is still an answer.

So Op 5 is a **resolution**, not an amendment — `status: superseded`, body verbatim, a dated section
appended, the shape wave 4 established for `kb-open-question-ps-1-…` and `kb-open-question-ps-19-…`.
`superseded_by` is **not** added: it is a decision-only key, no precedent atom carries it on an
`open_question`, and inventing it here would put a key in the corpus that nothing validates and
every later wave reads as fact.

**One obligation the resolution does not discharge.**
`references/adr/0007-projection-runner-decodes.md:37` still says the wrong thing, and `0032`'s
intake says it *"should be amended directly in the same wave."* That file is mutable and is not an
atom, so it is not a `.kb/` op — but it is the substance of PS-32, and closing the question without
it would resolve the record of an obligation while leaving the obligation. Recorded here as owed to
the integration step, and named in Op 5's dated section so a reader who finds it unperformed can see
that it was not forgotten.

---

## Adjudication 4 — five decision records asked for, none authored

The phase-7 defect log asks the wave for a decision record on C1, C2, C3, C4, and evidence
attachment for C5. Four of those five requests are declined into `open_question` atoms, and the
fifth into a `reference` atom. The reason is the ingest rule and the layer's own first bullet, and
it is the same call wave 4 made on PS-32 — a call this wave now gets to close, because the decision
finally arrived from a story that owned it.

The distinction that matters, and it is not subtle: ADR-0022, ADR-0031, ADR-0032 and ADR-0033 each
arrive with **the decision already taken** — a 35 KB record, a fired falsifier plus an executed
test, a verified defect with corrected text supplied, a published 29-row classification. The wave
transcribes them. C1–C4 arrive as *"wanted from the wave: a decision record"*, with the alternatives
unnamed and nothing signed off. Authoring those would be the wave deciding four architecture
questions in the same pass it was handed four, with no record, no rejected alternatives and no
human at the gate — which is what `.kb/decisions/README.md:31-33` forbids in terms: *"A decision
recorded without its rejected options is indistinguishable from an accident."*

---

## Adjudication 5 — the experiment's three lessons stay in the reference atom

`0034`-C2's three findings — measure arms round-robin in one process on a shared host; subtract a
baseline and check the subtraction resolves; measure the path where the arms structurally differ —
read like playbook material, and the file even titles them *"the transferable findings, beyond this
one decision."* They stay in the `reference` atom.

A playbook atom is a **procedure someone follows**, and the six in `.kb/playbooks/` are all written
that way — *repairing a frozen clause*, *landing a stricter gate*, *anchoring citations*. These
three are findings about what *this* experiment learned, each stated with the conditions that
produced it: the 45% inter-run disagreement, the ~18 ms commit that swallowed the guard at 50,000
events, the rejection path where the arms differ. Lifted out of those conditions they become
proverbs. Left in, they are citable — and if a second experiment reaches the same three
independently, *that* is the wave that has earned a playbook, with two instances behind it.

---

## Adjudication 6 — defect C4's conclusion is live and its mechanism is corrected

C4 states: *"`spec_trace.rs`'s check 4 short-circuits on `!has_suite(&c.id)` (`:695-697`), so no
`PS` rule name is resolved at all."* Verified against this worktree, and the first half is wrong:

```rust
// xtask/src/spec_trace.rs:1750
fn has_suite(clause_id: &str) -> bool {
    clause_id.starts_with("ES-") || clause_id.starts_with("VT-")
        || clause_id.starts_with("WF-") || clause_id.starts_with("PS-")
}
```

`PS-` is admitted, and `kb-reference-spec-trace-has-suite-001` records the flip and the test holding
it. But the loop has **two** terms (`:699`):

```rust
if c.schedules_new || !has_suite(&c.id) { continue; }
```

and `schedules_new` is set by a bare `†` in the clause's `Rule:` line (`:1630-1634`). So C4's
*conclusion* stands — a `PS` clause carrying `†` is skipped before its rule name is looked up — and
its *cited mechanism* is the wrong term of the guard. That also joins C4's two halves into one
question rather than two, because the `†` is simultaneously the redundant marker C4 complains about
and the switch that disables the check: the redundancy and the skip are the same character.

The existing reference atom **anticipates this without stating it** — *"the seventeen `†` marks §7.2
printed against the family were an accurate statement rather than a checked one"* — so Op 12
`extends` it and does not contradict it. Op 12 is a new atom rather than an append, on the reference
README's dating rule: that atom is a statement about 2026-08-15, and this is a fact about
2026-08-17. Wave 4 made the identical refusal when it created that atom rather than appending to
`kb-reference-phase-4-5-spec-reconciliation-001`.

---

## Adjudication 7 — the macros verdict is a new decision, not a supersession of ADR-0020

The intake leaves this to the wave. It is `create_new`, on `.kb/decisions/README.md:20-24`'s
mechanical test — *"a correction is a repair if the set of implementations the decision admits is
unchanged"* — applied one step further: this correction does not even reach ADR-0020's admitted set,
because ADR-0020 committed nothing about the derive. Its own Consequences says so:

> DT-2 resolves toward **explicit declaration**, and its price is measured in the signed-off first
> program: … 2.4:1, carried forward as the falsifiable prediction that AC-013's verdict lands
> "`happenstance-macros` is in scope for 0.1" — **a consequence stated here, not a second decision;
> the record itself asserts no *must* about the derive.**

A prediction published as falsifiable, and falsified, is the prediction working. Superseding
ADR-0020 for it would retire the sealed-`Boundary::query` derivation — the whole content of that
decision — because a number attached to it as a consequence came out the other way. `RUNBOOK.md:525`
naming 0020 in its ADR column is what makes the edit tempting and is not authority to make it; the
macros verdict is its own decision with its own title, which is what
`kb-playbook-one-decision-per-adr-title-001` asks for.

---

## Adjudication 8 — three atoms with no `references/adr/` record

ADR-0022 has a 35 KB record. ADR-0031, ADR-0032 and ADR-0033 have none —
`references/adr/` stops at 0030 plus 0022, verified. ADR-0033's evidence lives at
`references/evaluation/phase-7-macros-verdict.md` and ADR-0031's at
`references/evaluation/PRESSURE-TEST.md` plus an executed test; ADR-0032's is the intake document
itself plus the two accepted atoms it cites.

**This does not block the wave, and it is not silently normalised.** `CLAUDE.md`'s *"link the atom;
cite the record by `file:line`"* describes where the seventeen imported ADRs put their evidence, not
a precondition for minting an atom — and the three atoms each cite a real, immutable file by path.
What it does mean is that these three atoms are the **canonical** form rather than a summary of one,
so their bodies carry the rejected alternatives in full instead of pointing at a section number.
Each `source_paths` names the evidence file it actually has.

Recorded because the alternative — waiting for three records to be written before the atoms exist —
would leave four decisions taken and none findable, and because a later wave meeting a `.kb/`
decision with no `references/adr/` sibling should be able to see that it was noticed.

---

## Adjudication 9 — `reversibility` where the source states none

| Atom | Value | Why |
| --- | --- | --- |
| `kb-decision-0022` | **`medium`** | The intake states it. Confirmed as coherent: migration 1 has shipped shape, so changing the tag storage means a migration on stored data, but nothing is published and no adapter is frozen against it |
| `kb-decision-0031` | **`medium`** | Matches `kb-decision-0007`, the decision it partly reverses. Moving the runner back would re-split a working API across a seam, but nothing is published |
| `kb-decision-0032` | **`low`** | The intake states it, matching the atom it supersedes. Correct for the substance: ADR-0021's low reversibility is about re-siting a codec tag on events already written, and ADR-0032 carries that decision forward unchanged |
| `kb-decision-0033` | **`high`** — **the wave's proposal, not the source's** | The source states none. An *out* verdict for 0.1 that names its own reopen condition is the most reversible kind of decision there is: reversing it means adding a crate later, and `_decomposition.md:428` confirms an *out* verdict escalates nothing. Nothing is published that depends on the absence. Flagged here because it is the one `reversibility` in the wave with no source behind it |

---

## Op 1 — the append-condition experiment

**op:** `create_new` · **kind:** `reference` · **classification:** `extends`
**destPath:** `.kb/reference/append-condition-experiment-2026-08.md`
**sourceFiles:** `.kb/_intake/0034-append-condition-experiment-2026-08.md`,
`.kb/_intake/0033-adr-0022-append-condition-strategy.md`

**Why `create_new`.** No reference atom owns this measurement.
`kb-reference-position-visibility-experiment-001` scores 32 — the same *shape*, a different
experiment, and the reference README's dating rule forbids appending one snapshot to another. The
intake names it as the precedent shape, which is exactly what a precedent is: something to copy, not
somewhere to write.

**Why it is not folded into Op 3.** `.kb/decisions/README.md:36-39` — evidence is a reference atom
the decision cites, so a decision can be superseded without invalidating the numbers. ADR-0022 is
`reversibility: medium` with a named re-open trigger, so that is not hypothetical here. `00`, CL-1.

### Proposed frontmatter

```yaml
id: kb-reference-append-condition-experiment-001
title: The append-condition experiment — three strategies and three tag storages against real SQLite
kind: reference
status: accepted
authority_tier: note
summary: >-
  The phase-8 measurement ADR-0022 rests on, run 2026-08-16 against real SQLite 3.53.2 under WAL,
  synchronous NORMAL and a finite 5,000 ms busy timeout, kept in experiments/append-condition/
  outside the workspace and outside the gate. All five measured arms cleared
  event_store_conformance! first — 445 tests, 89 rules each — because a wrong arm is always the
  fastest. Three append-condition strategies separate only on the rejection path, measured
  round-robin in one process over 400 rounds: the monotonic-position guard costs 23 us against the
  BEGIN IMMEDIATE plus EXISTS probe's 32 and the conditional INSERT's 45 over a 5,000-event log,
  213 against 311 and 306 over 50,000, 972 against 1,513 and 1,532 on a two-tag boundary at 5,000,
  and 42,399 against 65,637 and 65,383 at 50,000. On the accepted-append path (guard cost 67, 79,
  74 us) and under contention at 8 and 64 connections the three are a tie within noise and are
  reported as one rather than given a manufactured margin. Three tag storages, 50,000 events, 25
  rounds round-robin: the join table's selective read of 516 of 50,050 costs 10,744 us against a
  canonical blob's 33,992 and JSON1's 49,766 — 3.16x and 4.63x against a plus-or-minus 7 per cent
  noise floor measured as an unfiltered read that touches no tag storage — while its unconditional
  single-event append costs 862 us against 414 and 590. Dropping the GROUP BY aggregate for a
  single-tag item, measured against its own negative control, cut the probe from 1,093 us to 556.
  A two-tag boundary costs roughly 200x a single-tag one at 50,000 events on every strategy.
  Sixty-four rusqlite connections opened on one file on every one of thirty races with busy 0 and
  failed 0 and exactly one winner each; a race costs about 130 ms at 8 contenders and 1.4 to 2.7 s
  at 64. Two positive controls fired: the runner refuses to emit a number under synchronous OFF,
  and the journal mode is read back rather than trusted from the PRAGMA that was issued. Two
  harness figures are noise-dominated on a shared developer host — two runs an hour apart
  disagreed by up to 45 per cent — and nothing here measured the tokio runtime seam.
depends_on: []
related:
  - kb-decision-0022
  - kb-decision-0012
  - kb-reference-position-visibility-experiment-001
source_paths:
  - .kb/_intake/0034-append-condition-experiment-2026-08.md
  - .kb/_intake/0033-adr-0022-append-condition-strategy.md
  - references/adr/0022-append-condition-strategy.md
  - experiments/append-condition/
last_reviewed: 2026-08-17
```

**Body sections:** `## What this is a pointer to` (the instrument, and that it is outside the
workspace and the gate), `## The question the experiment answers`, `## The conditions` (the table
verbatim — the reference README's dating rule is satisfied by the machine/toolchain/pragma table
plus the 2026-08-16 run date **in the body**, not only by `last_reviewed`), `## The controls that
fired`, `## What the instrument cannot do`, `## The transferable findings, beyond this one
decision` (Adjudication 5).

**mapsImpact:** `decisionMap: false`, `domainMap: true` (a bullet mirroring
`kb-reference-position-visibility-experiment-001`'s at `domain-map.md:125-127`),
`openQuestionIndex: false`.

---

## Op 2 — the two-adapter measurement ES-17 needs is unscheduled

**op:** `create_new` · **kind:** `open_question` · **classification:** `extends`
**destPath:** `.kb/open-questions/es-17-two-adapter-measurement-is-unscheduled.md`
**sourceFiles:** `.kb/_intake/0033-adr-0022-append-condition-strategy.md`

**Why an open question and not a line in ADR-0022.** ADR-0012 owns ES-17 and its falsifier, is
accepted, and its body may not absorb the finding that the falsifier's own measurement has no
owner. ADR-0022 records the non-verdict but cannot carry a live unowned obligation inside a
decision — wave 3's rule, and `.kb/decisions/README.md:41-43` in terms: *"A decision not yet taken
… is an `open_question`. Filing a live question here gives it an authority nobody granted it."*

**Why not merged into `kb-open-question-cf-40-ownership-001`** (score 25): different clause,
different subject. CF-40 is about which document owns a fixture-constant clause; this is about a
measurement nobody is scheduled to take.

### Proposed frontmatter

```yaml
id: kb-open-question-es-17-two-adapter-measurement-001
title: The two-build measurement ES-17's falsifier requires is scheduled by nobody
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ADR-0012 kept append's events &[Event] and declined to lift the clause to frozen, naming phase 8
  as the phase that would produce the evidence; its falsifier item 1 requires two builds of the
  same SQLite adapter differing only in append's ownership, measured on the same harness. Phase 8
  has now produced its append-condition measurement — three candidate stores in an experiment
  crate, recorded as kb-reference-append-condition-experiment-001 — and that is not it: the
  candidates differ in strategy, not in ownership. What is true today is that ADR-0022 records the
  marker as not lifted and quotes ADR-0012's falsifier verbatim rather than discharging it, and
  that the four implementation stories after ADR-0022 in this project's map build one adapter
  rather than two builds of one. What is not decided is who takes the measurement, or whether the
  obligation is deferred to a later phase with a named owner instead; either is a decision, and
  the silence is neither. Forced by whoever proposes to lift ES-17 to frozen, or by phase 12,
  where first publish turns append's signature into a promise. The KB half is this record; the
  queue row it also needs is .bklg/'s and is not a KB atom.
depends_on: []
related:
  - kb-decision-0012
  - kb-decision-0022
  - kb-reference-append-condition-experiment-001
source_paths:
  - .kb/_intake/0033-adr-0022-append-condition-strategy.md
  - references/adr/0012-append-shape-and-preconditions.md
  - references/adr/0022-append-condition-strategy.md
  - RUNBOOK.md
last_reviewed: 2026-08-17
```

**Body:** the layer README's four headings — *What is true today* (ADR-0012's falsifier item 1
quoted; ADR-0022 §13; the experiment's three arms and why they are not two builds), *What is not
decided*, *What forces it*, *Ordered sub-questions*.

**mapsImpact:** `openQuestionIndex: true` (a bullet under the contract-ports domain),
`decisionMap: false`, `domainMap: false`.

---

## Op 3 — ADR-0022, the append-condition strategy

**op:** `create_new` · **kind:** `decision` · **classification:** `extends`
**destPath:** `.kb/decisions/0022-append-condition-strategy.md`
**sourceFiles:** `.kb/_intake/0033-adr-0022-append-condition-strategy.md`

**Why `create_new`.** No accepted atom owns the proposition. `kb-decision-0012` scores 35 and is the
nearest — it owns `append`'s *signature and preconditions*, not how an adapter *evaluates* a
condition — and it is immutable. ADR-0022 has its own `adr_id`, its own 35 KB record and its own
phase.

**Why `supersedes: null`.** The record discharges no clause, amends none and moves no marker; it
ratifies the already-settled driver choice rather than re-deciding it.

### Proposed frontmatter

The intake's staged block is adopted essentially as proposed — it is the most complete staged
frontmatter any wave has received — with four changes, each stated:

```yaml
id: kb-decision-0022
title: The append condition is a max(position) guard inside BEGIN IMMEDIATE, and tags live in a join table keyed (tag, position)
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0022
reversibility: medium
phase: 8
supersedes: null
superseded_by: null
summary: >-
  [as staged at .kb/_intake/0033-…:12-38, verbatim]
depends_on:
  - kb-decision-0012
  - kb-decision-0010
related:
  - kb-reference-append-condition-experiment-001
  - kb-open-question-cf-40-ownership-001            # CHANGED — id corrected
  - kb-open-question-es-17-two-adapter-measurement-001   # ADDED — Op 2
  - kb-concept-torn-read-append-boundary-001        # ADDED
source_paths:
  - .kb/_intake/0033-adr-0022-append-condition-strategy.md
  - references/adr/0022-append-condition-strategy.md
  - experiments/append-condition/
  - RUNBOOK.md
last_reviewed: 2026-08-17                            # CHANGED — was 2026-08-16
```

1. **`kb-open-question-cf-40-fixture-limits-ownership-001` → `kb-open-question-cf-40-ownership-001`.**
   The staged id does not exist; the atom's real id is `kb-open-question-cf-40-ownership-001`
   (`.kb/open-questions/cf-40-fixture-limits-ownership.md:2`). A `related` naming a non-existent id
   fails resolution and cascades.
2. **`kb-open-question-es-17-two-adapter-measurement-001` added** — Op 2 is this decision's own
   escalation, and an atom that names an obligation should link to the atom that owns it.
3. **`kb-concept-torn-read-append-boundary-001` added** — `related`, score 55. The concept explains
   why a boundary is a boundary; this decision picks the SQL that enforces one.
4. **`last_reviewed: 2026-08-17`** — standing choice 5.

**Body sections:** `## Decision` (the guard, the SQL it is, and the rejection-path figures with
their conditions), `## The tag storage` (the join table, the covering column, the write cost paid),
`## The single-tag fast path` (`tag_cardinality` and most-selective-first as **requirements**),
`## The pragmas` (three values, each with the alternative that lost), `## The runtime seam`
(`NoRuntime` keeps a meaning), `## Alternatives rejected` (the two strategies, the two tag storages,
`index_arms()` with its named re-open trigger), `## Consequences`, `## The two non-verdicts`
(ES-17 → `kb-open-question-es-17-two-adapter-measurement-001`; CF-40 → the existing question,
cited and left open), `## What this decision does not do` (mints no clause; ratifies rather than
decides `rusqlite` without a pool).

**mapsImpact:** `decisionMap: true` (a new `## 2026-08-17 …` wave section and one row),
`domainMap: true` (phase 8 / adapter storage has no domain section yet — the Maps phase decides
whether ADR-0022 opens one or joins the contract-ports section), `openQuestionIndex: false`
(Op 2 carries it).

---

## Op 4 — ADR-0031, the runner collapses upward

**op:** `create_new` · **kind:** `decision` · **classification:** `extends` (partial supersession)
**destPath:** `.kb/decisions/0031-the-runner-collapses-upward.md`
**sourceFiles:** `.kb/_intake/0032-adr-0031-the-runner-collapses-upward.md`

**Why `create_new` and not an edit.** `kb-decision-0007` is accepted and immutable. **Why
`supersedes: null` and no flip on ADR-0007** — Adjudication 2, and it is the wave's most
consequential frontmatter choice.

### Proposed frontmatter

```yaml
id: kb-decision-0031
title: One runner, in happenstance — the checkpoint pump collapses upward
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0031
reversibility: medium
phase: 7
supersedes: null
superseded_by: null
summary: >-
  Partly supersedes ADR-0007's runner allocation while leaving its discriminator and all three of
  its shape decisions untouched and standing. ADR-0007 set its own falsifier — if the core pump
  has acquired no caller but the typed one when phase 7 exits, collapse it upward and supersede
  this decision — and it fired, in a stronger form than it anticipated: phase 7 exited with
  happenstance-core publishing exactly two module-level free functions, collect and
  read_decision_model at store.rs:285 and :321, neither a pump, and no pump function in the
  contract crate at all. The pump was allocated by an ADR, three phases passed, and it was never
  written, because at every point what an application needed was the typed runner and the typed
  runner drives the port directly. The decision: one runner, in happenstance. happenstance-core
  keeps the ProjectionStore port and the transactional invariant stated on its module doc and
  nothing that runs; happenstance::run_projection reads the checkpoint, derives the query, streams
  the replay, decodes through Codec, applies in chunks, and hands each chunk's write set and last
  applied position to the port's single commit. The cost ADR-0007 named for this alternative is
  still the true one — it puts the checkpoint invariant in a crate above the port that states it —
  and it is mitigated by something that did not exist when ADR-0007 was written rather than
  waved past: the projection conformance suite drives the port through ProjectionProbe and can
  fail a store that commits the two halves apart, which is a stronger guard than a pump no adapter
  calls. A second consequence is a saving: the old pump typed its callback's error as the
  projection store's, leaving a decode failure no representable home (E2E-26), where the collapsed
  runner has CodecError concrete and a ProjectionError decode arm carrying the failing position.
  The same record corrects ADR-0007's Context per PS-32: a callback-driven pump can be written
  against the port as it stands, compiled during the pressure test; what could not be written was
  the conformance suite. What is not superseded is everything else — the discriminator is
  encoding, not orchestration; a projection nominates its events with Query; Projection::Store is
  an associated type; checkpoints stay per store and projection id. All three are implemented as
  written, which is why kb-decision-0007 stays accepted and superseded_by stays null.
depends_on:
  - kb-decision-0007
related:
  - kb-decision-0006
  - kb-decision-0010
  - kb-decision-0017
  - kb-decision-0019
  - kb-decision-0030
  - kb-open-question-ps-32-adr-0007-correction-owed-001
  - kb-governance-referent-not-reasoning-001
source_paths:
  - .kb/_intake/0032-adr-0031-the-runner-collapses-upward.md
  - references/adr/0007-projection-runner-decodes.md
  - references/evaluation/PRESSURE-TEST.md
  - crates/happenstance/src/runner.rs
  - crates/happenstance/tests/projection_clauses.rs
  - crates/happenstance-core/src/store.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-17
```

**Edge justification.** `depends_on: kb-decision-0007` — the corpus's spelling for a partial
reversal, and the atom this one corrects. `related: kb-decision-0006` — the other half of the same
lineage, whose naming decision is untouched by both. `kb-decision-0017` and `kb-decision-0010` —
the owned-`Batch`/`ProjectionProbe` shape and the suite's proof obligation, which together are the
mitigation C5 rests on. `kb-decision-0019` — the failure-policy decision the collapsed runner
implements. `kb-decision-0030` — it `depends_on` ADR-0007's checkpoint-per-`(store, ProjectionId)`
shape, which this decision explicitly leaves standing; the edge is what shows a reader that
ADR-0030 does not move. `kb-open-question-ps-32-…` — the question Op 5 resolves with this atom.
`kb-governance-referent-not-reasoning-001` — the test that makes correcting a Context a superseding
atom's act rather than an edit.

**Body sections:** `## Context` (the falsifier and the re-derived count, **and PS-32's correction
stated in the corrected direction**: what cannot be written against the port as it stands is the
conformance suite, not the runner), `## Decision`, `## Alternatives rejected` (the cost ADR-0007
named, and the mitigation that did not exist then), `## Consequences` (E2E-26's decode-failure
saving), `## What this decision is not` (the discriminator and the three shape decisions, named
individually, and one sentence stating why `kb-decision-0007` keeps `status: accepted` — mirroring
`kb-decision-0006`'s own closing section).

**mapsImpact:** `decisionMap: true` (a row, plus the partial-supersession-chain section gains
`kb-decision-0007` → `kb-decision-0031` as the fourth link in the 0005 → 0006 → 0007 lineage),
`domainMap: true` (the projection-store domain's ADR-0007 bullet gains the collapse),
`openQuestionIndex: false` (Op 5 carries it).

---

## Op 5 — PS-32's question is answered

**op:** `merge_existing` · **kind:** `open_question` · **classification:** `extends`
**destPath / mergeTargetPath:** `.kb/open-questions/ps-32-adr-0007-context-correction-is-owed.md`
**sourceFiles:** `.kb/_intake/0032-adr-0031-the-runner-collapses-upward.md` · **score: 100**

Adjudication 3. Both halves of the question are answered, one of them in the negative.

**Frontmatter delta only. The body above the appended section is never touched.**

```yaml
status: accepted  →  superseded
related:  + kb-decision-0031            # appended; existing entries kept in order
source_paths:
  + .kb/_intake/0032-adr-0031-the-runner-collapses-upward.md
last_reviewed: 2026-08-15  →  2026-08-17
summary: + a closing clause, in the voice of the atom's own last sentence:
  "Resolved 2026-08-17 by ADR-0031 (kb-decision-0031), which collapses the runner upward and
  carries the corrected Context: what cannot be written against the port as it stands is the
  conformance suite, not the runner. The supersession is partial, as this atom anticipated —
  kb-decision-0007 stays accepted because its three shape decisions are implemented as written —
  and the pump sub-question is answered in the negative: no pump is written, because the typed
  runner drives the port directly. The amendment to references/adr/0007-projection-runner-decodes.md
  is owed with the same wave and is not an atom's."
```

**Body: append one `## Resolved 2026-08-17 — ADR-0031 collapses the runner upward` section.** It
must carry, and must not carry more than: which atom performs the correction; that the supersession
is partial and why `kb-decision-0007` keeps `status: accepted`; the corrected Context sentence in
the direction §4.9 asks for; that the pump sub-question is answered by no pump existing; and one
sentence recording that `references/adr/0007-…:37` is a long-form amendment owed alongside, so a
reader who finds it unamended can see it was not overlooked.

**`superseded` over `withdrawn`**, as in wave 4: `withdrawn` would say the question stopped
mattering, and it was answered. **`superseded_by` is not added** — decision-only key, no precedent
on an `open_question`.

**mapsImpact:** `openQuestionIndex: true` (the bullet moves Open → **Superseded**, naming
ADR-0031), `decisionMap: false`, `domainMap: false`.

---

## Op 6 — ADR-0032, the ADR-0021 serde attribution correction

**op:** `supersede` · **kind:** `decision` · **classification:** `conflicts` → resolved
**destPath:** `.kb/decisions/0032-adr-0021-serde-attribution-correction.md`
**supersedesPath:** `.kb/decisions/0021-payload-evolution-and-codec-tag.md`
**sourceFiles:** `.kb/_intake/0031-adr-0021-serde-attribution-correction.md`

**Why supersession and not an edit.** `.kb/decisions/README.md:9-13`, and `redkiln validate --kb`
checks each accepted body against `HEAD`, so a reword fails the gate by design. **Why a full
supersession** — Adjudication 2: the defect is in the atom's own body at `:141-144`, so a reader
must be routed onward.

**Why the ADR-0003 attribution is dropped rather than corrected**, per C3: ADR-0003 has nothing to
say about this alternative in either direction, and a corrected-but-present citation keeps inviting
the same misreading. The relationship is carried by `related` instead.

### Proposed frontmatter

```yaml
id: kb-decision-0032
title: The serde-encoded framing region is rejected on two grounds, and ADR-0003 was never one of them
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0032
reversibility: low
phase: 7
supersedes:
  - kb-decision-0021
superseded_by: null
summary: >-
  A repair of ADR-0021, not an amendment: the set of implementations admitted is unchanged, and
  all three of ADR-0021's decisions carry over intact — the codec tag lives in Event::metadata
  inside a versioned framing region the typed layer owns, EventType carries no version suffix and
  matching is exact equality, and upcasting happens at decode with no read-path hook added to the
  frozen EventStore, with an untagged event decoding under the codec already in hand rather than
  as UnknownTag. What is withdrawn is one justification for one rejected alternative. ADR-0021
  rejected a serde-encoded framing region on three grounds and the third stated a binding
  constraint backwards: also barred by ADR-0003, which forbids serde in happenstance-core's
  default features. ADR-0003 constrains happenstance-core only and positively assigns encoding and
  decoding to happenstance, the layer above; the framing region is written by happenstance, one
  crate above the port, so ADR-0003 does not bar serde there and CLAUDE.md's binding constraint 2
  warns about this exact reading after ADR-0006's rename. Read as written, an implementer would
  conclude serde is barred from the crate whose entire job is encoding — forbidding the thing the
  ADR-0006 split exists to allow. The rejection stands on its two sound grounds, which never
  depended on ADR-0003: reading the framing region is how the payload codec is discovered, so the
  region cannot be encoded by a Codec and must not depend on any codec feature, since a build with
  only postcard enabled must still read a tag written by a build with only json; and the region
  must be readable with no default features and on wasm32, so it is not serde-encoded and is not
  JSON. The boundary is stated here once in the right direction so the atom that supersedes this
  one cannot re-acquire the inversion: ADR-0003 constrains happenstance-core, whose serde feature
  covers envelope types only; happenstance is the typed layer whose job is encoding, and serde
  there is the split working; payloads stay Bytes at the port. Provenance: the story's staged
  deliverable gave only the two sound reasons and stated the boundary correctly; the ADR-0003
  clause was introduced during distillation in the 2026-08-15 wave, which makes this an ingest
  defect rather than a story defect.
depends_on:
  - kb-decision-0021
related:
  - kb-decision-0003
  - kb-decision-0006
  - kb-decision-0016
  - kb-decision-0020
source_paths:
  - .kb/_intake/0031-adr-0021-serde-attribution-correction.md
  - references/adr/0021-payload-evolution-and-codec-tag.md
  - .kb/decisions/0003-opaque-payloads.md
  - crates/happenstance-core/src/event.rs
  - xtask/src/main.rs
last_reviewed: 2026-08-17
```

`depends_on: [kb-decision-0021]` **and** `supersedes: [kb-decision-0021]` are both carried, as C3
instructs: the dependency is what makes the atom readable (it does not restate ADR-0021's three
decisions in full), and the supersession is what routes a reader off the defective body.

**Body sections:** `## What is repaired` (the inverted attribution, quoted, and the direction it
inverts, citing `0003-opaque-payloads.md:17-18`), `## The rejection, restated on its two sound
grounds`, `## The boundary, in the right direction`, `## What carries over unchanged` (ADR-0021's
three decisions, its falsifier, its reversibility and its untagged-event rule, by name),
`## Provenance` (an ingest defect, EC-004 / NF-004, introduced at distillation).

**The atom must carry no citation change.** C6's proposed widening of `projection.rs:152-154` is
verified wrong; `:155` is `pub fn new`. Recorded here, applied nowhere.

**mapsImpact:** `decisionMap: true`, `domainMap: true`, `openQuestionIndex: false`.

---

## Op 7 — `kb-decision-0021`'s metadata flip

**op:** `merge_existing` · **kind:** `decision` · **classification:** `conflicts` → resolved
**destPath / mergeTargetPath:** `.kb/decisions/0021-payload-evolution-and-codec-tag.md`
**sourceFiles:** `.kb/_intake/0031-adr-0021-serde-attribution-correction.md` · **score: 95**

**The only edit an accepted decision ever receives. Three fields, no prose.**

```yaml
status: accepted  →  superseded
superseded_by: null  →  kb-decision-0032
last_reviewed: 2026-08-15  →  2026-08-17
```

**The body stays verbatim, including the defective sentence at `:141-144`.** That is the point of
the rule and not an oversight: `.kb/decisions/README.md:15-18` — the old atoms *"are kept verbatim
precisely because the crate names and constraints in older commits only make sense with them."*
A reader who follows a 2026-08-15 citation into `:141-144` must find what was cited, and
`superseded_by` is what tells them where the correction is.

`source_paths` is **not** extended. Nothing about this atom's provenance changed; the flip is a
consequence of another atom's provenance, and that atom carries it.

**mapsImpact:** `decisionMap: true` (`:138`'s row → `superseded`, *"superseded by ADR-0032"* in the
last column, and the partial-supersession-chain section gains this lineage as the corpus's **second**
outright supersession), `domainMap: true` (`:189-194`'s typed-layer bullet re-points at
`kb-decision-0032`), `openQuestionIndex: false`.

---

## Op 8 — D-1: the validated type has no total path

**op:** `create_new` · **kind:** `open_question` · **classification:** `requires-new-decision`
**destPath:** `.kb/open-questions/d-1-the-validated-type-has-no-total-path.md`
**sourceFiles:** `.kb/_intake/contract-defect-log-phase-7.md`,
`.kb/_intake/happenstance-macros-verdict.md`

**One atom for C1 and C2** — `00`, CL-4. The intake calls C2 *"D-1's other face"*, both bear on
VT-18 `[FROZEN]`, and `kb-decision-0020` logs the pair as one defect candidate, D-1.

**Why an open question and not a decision** — Adjudication 4. **Why not merged into
`kb-decision-0020`** (score 90): accepted, immutable, and it already names *"a decision record, not
a line edit"* as D-1's route, which is a decision naming a gap rather than the question that owns it
— wave 3's rule.

### Proposed frontmatter

```yaml
id: kb-open-question-d-1-no-total-path-001
title: D-1 — a validated identifier has no infallible route, in either direction
kind: open_question
status: accepted
authority_tier: note
summary: >-
  Two faces of one defect, both bearing on VT-18 which is FROZEN, both recorded by phase 7's use
  of the frozen contract and both named by ADR-0020 as defect candidate D-1 with a decision record
  rather than a line edit as their route. Inward: QueryItem::new is fallible even when the caller
  already holds validated EventTypes and Tags, so every derived query carries a Result that is
  unreachable for a well-formed model; and because Boundary is sealed, that error arm is
  untestable from outside happenstance-core, since no downstream type can be a failing Boundary.
  Outward: DomainEvent::tags returns Tags totally while every route into Tags is fallible, so an
  implementor whose tag values are runtime strings has no total path except to invent a newtype
  holding the validated Tag — which the worked example does, at a cost of 81 lines. What is not
  decided is whether happenstance-core grows an infallible constructor for pre-validated inputs,
  what it is called, and whether the same answer serves both faces or only the first; ADR-0020
  routed it to a decision record and named VT-18 as its nearest clause subject, and no record has
  been written. What forces it is the first API change after 0.1, and the macros verdict, which
  names this defect's settlement with an infallible Tags path as the single condition that would
  reopen AC-013 and require its 2026-08-16 measurement to be re-taken.
depends_on: []
related:
  - kb-decision-0020
  - kb-decision-0015
  - kb-decision-0033
  - kb-reference-macros-ceremony-measurement-001
source_paths:
  - .kb/_intake/contract-defect-log-phase-7.md
  - .kb/_intake/happenstance-macros-verdict.md
  - references/evaluation/phase-7-contract-defects.md
  - crates/happenstance-core/src/query.rs
  - crates/happenstance-core/src/tag.rs
  - examples/course-subscriptions/src/main.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-17
```

`related: kb-decision-0033` and `kb-reference-macros-ceremony-measurement-001` are the CL-3 edge and
are why this atom is authored **before** Op 10 in the order — though as outbound links from Op 8 to
atoms Ops 9 and 10 create, the Maps phase wires them rather than the author. The integrator should
author Op 8 with `related: [kb-decision-0020, kb-decision-0015]` only, and let Maps add the other
two once they exist.

**mapsImpact:** `openQuestionIndex: true`, `domainMap: true` (typed-layer domain),
`decisionMap: false`.

---

## Op 9 — the phase-7 ceremony measurement

**op:** `create_new` · **kind:** `reference` · **classification:** `extends`
**destPath:** `.kb/reference/phase-7-macros-ceremony-measurement.md`
**sourceFiles:** `.kb/_intake/happenstance-macros-verdict.md`

**Why a separate atom** — `00`, *the dedup that was refused*, and the wave's nearest call at 74.
Two grounds: `.kb/decisions/README.md:36-39`, applied identically to CL-1 rather than by the size of
the table; and the verdict's own reopen condition, which says *"AC-013's measurement should be
re-taken"* — so these numbers are expected to be superseded on a schedule the decision does not
share, which is precisely what the separation exists for.

### Proposed frontmatter

```yaml
id: kb-reference-macros-ceremony-measurement-001
title: The ceremony-to-domain ratio of the worked example, measured line range by line range
kind: reference
status: accepted
authority_tier: note
summary: >-
  The classification AC-013's verdict rests on, taken 2026-08-16 against
  examples/course-subscriptions/src/main.rs at commit 78a2170, 532 lines, as
  worked-example-on-typed-layer left it. Checked plain before counting — no macro_rules!, and impl
  DomainEvent for Enrolment written out by hand — rather than against the design's doctest, which
  _design.md forbids substituting. The judgement is not in the threshold, which is mechanical, but
  entirely in the classification, which is why the classification is published as 29 contiguous
  line ranges that exhaust the file, 532 of 532, with the totals derived by summing rows.
  Ceremony, the impl DomainEvent block at :209-247, is 40 lines; domain is 249; neither — main,
  the transcript, imports and call plumbing — is 158; contested, the CourseId and StudentId
  newtypes at :98-182, is 85. Assigning the contested block to ceremony gives 125:249, a ratio of
  0.50:1; assigning it to domain gives 40:334, or 0.12:1. Both extremes are below 1.0, so the
  contested block is a footnote rather than the decision. One further figure, which is what a
  derive would have bought: 40 lines of 532, or 7.5 per cent. The reconciliation with _design.md's
  2.4:1 prediction over its own doctest is that the DomainEvent impl is a near-fixed cost, 26
  lines for two variants and 40 for three, while domain logic grows with consistency concerns,
  refusals and handlers — so the ceremony ratio is a function of how much domain an artefact
  contains, and a minimal doctest measures it where there is almost none. Both numbers are true
  and they answer different questions.
depends_on: []
related:
  - kb-decision-0020
  - kb-decision-0033
source_paths:
  - .kb/_intake/happenstance-macros-verdict.md
  - references/evaluation/phase-7-macros-verdict.md
  - examples/course-subscriptions/src/main.rs
  - RUNBOOK.md
last_reviewed: 2026-08-17
```

The dating rule is satisfied by the commit sha **and** the date in the body, not only by
`last_reviewed`. `related: kb-decision-0033` is outbound to an atom Op 10 creates; the integrator
authors it with `related: [kb-decision-0020]` and Maps adds the second.

**Body sections:** `## What was measured, and against what` (the substrate, the pin, and the
plain-before-counting check), `## The classification` (the four buckets and the exhaustion claim),
`## The two readings` (the verdict table), `## What a derive would have bought`,
`## Why this differs from the design's own number` (the fixed-cost reconciliation).

**mapsImpact:** `domainMap: true`, `decisionMap: false`, `openQuestionIndex: false`.

---

## Op 10 — ADR-0033, `happenstance-macros` is out of scope for 0.1

**op:** `create_new` · **kind:** `decision` · **classification:** `extends`
**destPath:** `.kb/decisions/0033-happenstance-macros-out-of-scope-for-0-1.md`
**sourceFiles:** `.kb/_intake/happenstance-macros-verdict.md`,
`.kb/_intake/contract-defect-log-phase-7.md`

**Why `create_new` and not a supersession of ADR-0020** — Adjudication 7. **`sourceFiles` lists two
files** because the reopen condition is C2's, staged in the defect log — `00`, CL-3.

### Proposed frontmatter

```yaml
id: kb-decision-0033
title: happenstance-macros is out of scope for 0.1
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0033
reversibility: high
phase: 7
supersedes: null
superseded_by: null
summary: >-
  AC-013's criterion is mechanical — if the rewritten worked example carries more mapping
  boilerplate than domain logic the derive is in scope, more meaning a ratio above one, and
  exactly 1.0 is out — so the whole judgement is in the classification, which is published line
  range by line range and summed rather than asserted. Measured against
  examples/course-subscriptions/src/main.rs at 78a2170, the ratio is 0.50:1 with every contested
  line charged to ceremony and 0.12:1 with them charged to domain. Both extremes say out, so the
  contested block is a footnote and the verdict stands: no crates/happenstance-macros/ ships in
  0.1. ADR-0020 predicted the opposite, in at 2.4:1, over its own doctest, and the contradiction
  is the prediction working rather than a conflict to reconcile away: ADR-0020 published the
  number as falsifiable and asserts no must about the derive, so nothing accepted is superseded
  here. The two numbers differ for a durable reason worth keeping: the DomainEvent impl is a
  near-fixed cost, 26 lines for two variants and 40 for three, while domain logic grows with
  consistency concerns, refusals and handlers — so the ceremony ratio is a function of how much
  domain an artefact contains, and a minimal doctest measures it at the point of almost no domain.
  A first-program page is still held to 2.4:1, which is AC-U01; a scope decision is not taken on
  it, which is AC-013. What a derive would have bought is 40 lines of 532, 7.5 per cent, against a
  fourth published crate and a proc-macro in every consumer's build graph. Reopen if and only if
  D-1 is settled with an infallible Tags path: a derive that also handled tags from runtime values
  would reach into the contested 85 lines rather than only the 40, but even an 85-line swing does
  not cross 1.0 from 0.50:1, so it stays a post-0.1 question and the measurement would be re-taken
  rather than re-argued. Consequences already carried out: no crate was created, since an out
  verdict escalates nothing; RUNBOOK.md's decision-table row moves off open; and
  publish-0-2-0-alpha-1 is unblocked, because it depends on the record and never on a crate.
depends_on:
  - kb-reference-macros-ceremony-measurement-001
related:
  - kb-decision-0020
  - kb-open-question-d-1-no-total-path-001
source_paths:
  - .kb/_intake/happenstance-macros-verdict.md
  - .kb/_intake/contract-defect-log-phase-7.md
  - references/evaluation/phase-7-macros-verdict.md
  - examples/course-subscriptions/src/main.rs
  - RUNBOOK.md
last_reviewed: 2026-08-17
```

`depends_on` the measurement rather than `related`: this decision is unreadable without it — the
verdict *is* the ratio, and a reader who cannot reach the classification cannot check the one thing
that decides the question.

**Body sections:** `## Decision`, `## Why, in the form that makes it checkable` (the criterion, and
that the judgement is in the classification), `## The prediction it contradicts` (ADR-0020's 2.4:1,
and why a falsified prediction is not a supersession), `## What a derive would have bought`,
`## The reopen condition` (D-1, and why even the contested 85 does not cross the threshold),
`## Consequences already carried out`.

**mapsImpact:** `decisionMap: true`, `domainMap: true` (typed-layer domain),
`openQuestionIndex: false` (Op 8 carries the D-1 bullet).

---

## Op 11 — CF-36 names a cross-reference nothing performs

**op:** `create_new` · **kind:** `open_question` · **classification:** `requires-new-decision`
**destPath:** `.kb/open-questions/cf-36-names-a-cross-reference-nothing-performs.md`
**sourceFiles:** `.kb/_intake/contract-defect-log-phase-7.md`

**Why an open question** — Adjudication 4; the intake itself says *"both are decisions; neither is
a patch"* and names neither as preferred. **Why separate from Op 12** (score 72) — `00`, *the dedup
that was refused*: one defect shape, two clauses, two different settling conditions.

### Proposed frontmatter

```yaml
id: kb-open-question-cf-36-unperformed-cross-reference-001
title: CF-36 names a cross-reference cargo xtask spec-trace does not perform
kind: open_question
status: accepted
authority_tier: note
summary: >-
  CF-36 is FROZEN and its Rule line claims cargo xtask spec-trace cross-references each case's
  level marker. It does not: grep -c "Level" over xtask/src/spec_trace.rs returns zero, and no
  other check performs the comparison under another name. The consequence is worse than an
  unwritten check, because the gate is green and reads as evidence: a passing spec-trace is
  currently taken as confirmation of a clause whose stated mechanism does not exist, which is the
  same defect shape a stale exemption carries — nothing fails when the check is absent, because
  the absent thing's job is to make things fail. What is not decided is which of two routes CF-36
  takes: implementing the cross-reference in spec_trace.rs so the clause becomes true, or
  superseding CF-36 so it stops claiming an instrument nobody built. Both are decisions with
  alternatives and a record; neither is a patch, and the repository's own method for the second is
  kb-playbook-repair-frozen-clause-001. Forced by the next reader who cites a green spec-trace as
  evidence for a level marker, and by phase 12, where first publish makes the specification a
  promise rather than a working note.
depends_on: []
related:
  - kb-reference-spec-trace-has-suite-001
  - kb-playbook-repair-frozen-clause-001
  - kb-open-question-es-6-unwritable-rule-001
source_paths:
  - .kb/_intake/contract-defect-log-phase-7.md
  - references/evaluation/phase-7-contract-defects.md
  - xtask/src/spec_trace.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-17
```

**mapsImpact:** `openQuestionIndex: true` (specification-governance section), `decisionMap: false`,
`domainMap: false`.

---

## Op 12 — no `PS` rule name is resolved, and the `†` is the reason

**op:** `create_new` · **kind:** `open_question` · **classification:** `requires-new-decision`
**destPath:** `.kb/open-questions/no-ps-rule-name-is-resolved.md`
**sourceFiles:** `.kb/_intake/contract-defect-log-phase-7.md`

**Adjudication 6 governs the content**: the conclusion is live, the cited mechanism is corrected,
and the correction is what merges C4's two halves into one question instead of two.

### Proposed frontmatter

```yaml
id: kb-open-question-no-ps-rule-name-resolved-001
title: No PS rule name is resolved, because the dagger that marks it unwritten is also what disables the check
kind: open_question
status: accepted
authority_tier: note
summary: >-
  CF-38 is FROZEN, and PS-27 and PS-30 are the visible symptoms of a check that does not run
  against the projection family. The intake recorded the cause as has_suite excluding PS, and that
  half is now wrong: has_suite admits PS- since the 2026-08-15 wave, held by a test, and
  kb-reference-spec-trace-has-suite-001 records the flip. The guard has two terms. Check 4's loop
  continues on c.schedules_new || !has_suite(&c.id), and schedules_new is set by a bare dagger in
  the clause's Rule line, so every PS clause marked with a dagger — the seventeen §7.2 prints — is
  skipped before its rule name is looked up. The conclusion the intake reached therefore stands
  and its stated mechanism does not, and the correction is what joins the two halves of the
  finding: the dagger is simultaneously the marker that duplicates the clause's own maturity
  marker while naming no owner where the clause does, and the switch that disables the check on
  the clauses carrying it. What is not decided is whether a dagger should suppress rule
  resolution at all, whether the dagger convention is superseded by maturity markers that carry
  owners, and whether removing it from the guard would report a wall of true positives that some
  other decision must absorb first. Forced by the next PS clause that cites a rule name nobody has
  written, which no gate in this repository would report.
depends_on: []
related:
  - kb-reference-spec-trace-has-suite-001
  - kb-open-question-cf-36-unperformed-cross-reference-001
  - kb-open-question-provisional-falsifiers-001
  - kb-playbook-repair-frozen-clause-001
source_paths:
  - .kb/_intake/contract-defect-log-phase-7.md
  - references/evaluation/phase-7-contract-defects.md
  - xtask/src/spec_trace.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-17
```

**Body:** four headings, with *What is true today* carrying both terms of the guard quoted from
`spec_trace.rs:699` and `:1630-1634` and the `has_suite` body quoted so the correction is checkable,
and *Ordered sub-questions* carrying the `†`-convention question as sub-question 3 rather than as a
separate atom.

**mapsImpact:** `openQuestionIndex: true` (specification-governance section), `decisionMap: false`,
`domainMap: false`.

---

## Op 13 — N projections cost N reads, at the API level

**op:** `create_new` · **kind:** `reference` · **classification:** `extends`
**destPath:** `.kb/reference/projection-fan-out-costs-n-reads.md`
**sourceFiles:** `.kb/_intake/contract-defect-log-phase-7.md`

**Why `create_new` when the bias is merge.** This is the wave's only new atom where *every*
candidate scored below 50 — the definition of "no reasonable owner exists":

- `kb-decision-0019` (80 on subject) names PS-30's falsifier as *"whether the poll cost of N
  independent reads is real, a benchmark this workspace has no harness for"*. It is accepted and
  immutable, and an atom that states a question cannot absorb its own evidence.
- `kb-open-question-projection-batch-no-apply-001`, the destination the intake asks for, is
  `status: superseded` since 2026-08-13. There is no live open question owning the tail seam.
- `kb-reference-port-traits-compiled-findings-001` (45) is a register of findings about the two
  *flavours* — coherence, `Send`, erasure. This is about the runner's fan-out arity.

**Why `reference` and not `open_question`.** The intake is explicit: *"recorded as a shape, not a
wrong answer"*, and the runner's defence is *"sound for the alpha."* Filing a settled-for-now shape
as an open question would misrepresent it as undecided; filing it as a reference makes it citable by
whichever decision eventually meets the tail seam, which is what the intake asked for.

### Proposed frontmatter

```yaml
id: kb-reference-projection-fan-out-cost-001
title: What &mut P fixes about fan-out — N projections cost N reads
kind: reference
status: accepted
authority_tier: note
summary: >-
  Recorded 2026-08-16 against commit 78a2170, from phase 7's use of the frozen contract. Because
  apply takes &mut P, a runner cannot drive more than one projection from a single replay: N
  projections cost N reads of the same events, and the cost is fixed at the API level rather than
  by any adapter's implementation. This is a shape and not a wrong answer. The runner's own
  defence is on the record and is sound for the alpha — an owned write set cannot be shared
  between projections, and one failure policy for every projection would be wrong, which is
  ADR-0019's per-projection policy decision. The projection port family is PROVISIONAL and has no
  conformance suite, and ADR-0019 already names the open half: PS-30 falls if the fan-out runner is
  never built, decided by whether the poll cost of N independent reads is real, which is a
  benchmark this workspace has no harness for. This atom supplies the half that needs no harness —
  the arity is derivable from the signature — so that whichever decision eventually meets the tail
  seam meets a figure already on the record rather than starting from an argument. It supplies no
  measurement of what a read costs, and none should be inferred from it.
depends_on: []
related:
  - kb-decision-0019
  - kb-decision-0017
  - kb-decision-0031
source_paths:
  - .kb/_intake/contract-defect-log-phase-7.md
  - references/evaluation/phase-7-contract-defects.md
  - crates/happenstance/src/runner.rs
  - crates/happenstance-core/src/projection.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-17
```

**Body:** `## What is true today` (the signature, the arity it fixes, the pin), `## Why this is a
shape rather than a defect` (the runner's defence, and ADR-0019's per-projection policy),
`## What this does not establish` (no cost measurement; the harness ADR-0019 names still does not
exist).

**mapsImpact:** `domainMap: true` (projection-store domain), `decisionMap: false`,
`openQuestionIndex: false`.

---

## Op 14 — the compiled-findings register gains ADR-0009's marker, exercised

**op:** `merge_existing` · **kind:** `reference` · **classification:** `aligns`
**destPath / mergeTargetPath:** `.kb/reference/port-traits-compiled-findings.md`
**sourceFiles:** `.kb/_intake/contract-defect-log-phase-7.md` · **score: 82**

**Why this is the merge and ADR-0009 is not.** The intake's own words: *"at most an amendment to
ADR-0009's atom … never a new decision."* An accepted decision takes no amendment either, so the
finding needs a home that is designed to receive one — and this atom is the corpus's only atom that
declares itself a register: *"the register those facts live in — cite it by id rather than
re-deriving the compile."* N1 is a `Send`-bound finding about the same two ports, reached the same
way, and it is the register's first entry that reports a promise **kept** rather than a premise
half-wrong. That is worth having in it.

**The delta.**

```yaml
related:  + kb-decision-0009  (already present — no change)
          + kb-decision-0031        # the runner the finding is about
source_paths:
  + .kb/_intake/contract-defect-log-phase-7.md
  + references/evaluation/phase-7-contract-defects.md
  + crates/happenstance/src/runner.rs
last_reviewed: 2026-08-13  →  2026-08-17
summary: + one clause, in the register's own voice:
  "And ADR-0009's marker-trait shape has now been exercised by a real consumer: run_projection
  cannot be spawned without a caller-side bound, which is ES-6 working as frozen rather than a
  defect, with the consequence written where a caller meets it."
```

**Body: one new finding paragraph** under `## The findings`, in the shape of the six already there
— bold claim sentence, mechanism, the ADR it belongs to — plus one clause in `## Provenance`
attributing it to the phase-7 defect log. It must state that this is a promise kept and **not** a
defect, because the register's other six entries all correct something, and an entry that reads as
a seventh correction would misreport ADR-0009.

**Not extended with C5.** Score 45 — `00`, and Op 13.

**mapsImpact:** `domainMap: false` (the atom is already indexed), `decisionMap: false`,
`openQuestionIndex: false`.

---

## What the Maps phase inherits

Three map atoms, none of them an op of its own — wave 3's convention, kept.

| Map | Work |
| --- | --- |
| `kb-map-decision-001` | A new `## 2026-08-17 append-condition and phase-7 closeout ADRs (ADR-0022, ADR-0031, ADR-0032, ADR-0033)` section with **four rows** in ADR-number order; `:138`'s ADR-0021 row → `superseded`, *"superseded by ADR-0032"*; the *Reading the partial-supersession chain* section gains **two** lineages — `kb-decision-0007` → `kb-decision-0031` as the fourth link in the 0005 → 0006 → 0007 chain (partial, `superseded_by` deliberately null), and `kb-decision-0021` → `kb-decision-0032` as the corpus's **second outright supersession**, the first since `kb-decision-0002`. The section should say which spelling each is and why, since this is the first wave to perform both |
| `kb-map-domain-001` | Ops 1, 3 and 13 want a home phase 8 / adapter storage does not have yet — the Maps phase decides whether ADR-0022 opens a new domain section or joins *Contract ports, conformance, and the ADR corpus*; Ops 8, 9 and 10 join *The typed layer*; `:189-194`'s ADR-0021 bullet re-points at `kb-decision-0032`; the projection-store paragraph gains ADR-0031's collapse; a bullet for each of the three new reference atoms |
| `kb-map-open-questions-index-001` | **Four new bullets** — Ops 2, 8, 11, 12, each under its domain section, status `Open` — and **one flip**, `ps-32-adr-0007-context-correction-is-owed` → `Superseded`, naming ADR-0031. Label per the index's own *Adding an entry* rule, matching the atom's `status` |

## Reciprocal edges the Maps phase wires

| From | To | Why the author cannot write it |
| --- | --- | --- |
| `kb-reference-append-condition-experiment-001` | `kb-decision-0022` | Op 1 precedes Op 3 |
| `kb-open-question-d-1-no-total-path-001` | `kb-decision-0033`, `kb-reference-macros-ceremony-measurement-001` | Op 8 precedes Ops 9 and 10 |
| `kb-reference-macros-ceremony-measurement-001` | `kb-decision-0033` | Op 9 precedes Op 10 |
| `kb-open-question-cf-36-unperformed-cross-reference-001` | `kb-open-question-no-ps-rule-name-resolved-001` | Op 11 precedes Op 12 |
| `kb-reference-spec-trace-has-suite-001` | `kb-open-question-no-ps-rule-name-resolved-001` | The extension edge for Adjudication 6. **The atom's body is not edited** — a `related` entry and `last_reviewed` only, if the Maps phase judges the edge worth the touch; a reference atom is not immutable, but its snapshot property means the body must not gain a 2026-08-17 sentence |

## Owed outside `.kb/`, and not this wave's ops

1. **`references/adr/0007-projection-runner-decodes.md:37`** — amend the Context per PS-32.
   Mutable, not an atom, and named in Op 5's dated section as owed. Adjudication 3.
2. **A `.bklg/` queue row** for ES-17's two-build measurement, or an explicit deferral with a named
   owner. Op 2 records the gap; the row is `redkiln new`'s and a task is not a KB atom.
3. **A `redkiln new` invocation** for N2, the `read_through` dead code in eight `wasm32` feature
   combinations. `unresolved`.
4. **A candidate rule in `standards/rust/`** for N3, the private module shadowing a glob re-export.
   `unresolved`.
