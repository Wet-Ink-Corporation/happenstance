# Wave `2026-08-13-projection-adrs` — placement and adjudication

The ordered action plan. **Six operations, one per destination atom**: three `decision` atoms
created and three existing `open_question` atoms amended. Every link is **outbound to an atom
that already exists or was created by an earlier operation** — there is no forward reference in
this wave — so the Integration phase can run the three creates in parallel and the three
amendments serially after them. Reciprocal backlinks are the Maps phase's.

**No operation edits an accepted decision atom's body, and no operation flips one's status.**
Seventeen accepted decision atoms exist and all seventeen are untouched. The one edge that looked
like a supersession is adjudicated below (Adjudication 1) and is a `depends_on`.

---

## Standing choices, applied to every atom in the wave

### 1. One atom per ADR — and the long-form record keeps the evidence

Three ADRs, three atoms, ~300–900 words each. The long-form records run 500+ lines and carry the
compiler transcripts, the five-ingredient ICE minimisation, the incident transcript and the full
rejected-alternatives argument. Those are **cited by `file:line`, never copied** — the intake
says so in terms and the reference layer's README says the same thing generally. Collapsing the
two would discard evidence a summary cannot hold; inflating the atom to carry it breaks the other
way and `spec-trace` would eventually catch the drift.

### 2. `status`, and the value the schema does not have

All three ADRs are *accepted with named provisional halves*. `KbFrontmatter.status` is a closed
enum with no such value. The convention wave 2 established is applied unchanged:

- `status: accepted`;
- the provisional qualification **and its falsifier** folded into the **first clause of
  `summary`**, so a reader who reads only the frontmatter cannot mistake a provisional half for a
  frozen one;
- `supersedes` / `superseded_by` reserved for *full* supersession, and both `null` on all three.

`kb-open-question-adr-status-vocabulary-001` records the residual gap and **is not edited**. The
ADR-0017 intake asks for that explicitly: *"Do not answer `kb-open-question-adr-status-vocabulary-001`
by acting on it."* Applying a convention for the third wave running is not answering the
question; deleting the question because a convention exists would be.

### 3. Strength is carried inside the atom, not by splitting it

ADR-0018 grades four claims at three strengths and warns that an atom presenting them as one flat
list *"has lost the decision's shape"*. ADR-0017 does the same with three halves. The answer is a
strength paragraph inside one atom — **settled by being made**, **settled elsewhere and cited**,
**provisional with falsifier and phase** — not one atom per strength. `adr_id` and the supersede
pair have to identify one atom, and a decision split three ways cannot be superseded as a unit.

### 4. `authority_tier`, `phase`, `reversibility`

`authority_tier: decision` and `phase: 6` on all three, mirroring the corpus. `reversibility` is
judged against the corpus's own usage, where every phase-4 port-freezing decision (0011, 0012,
0013, 0015) is `low`:

- **`kb-decision-0017` — `low`.** It fixes the shape of an associated type the whole adapter
  population implements; reversing it after the freeze re-breaks every adapter.
- **`kb-decision-0018` — `low`.** Same surface: a port method, its error type, and the shape of
  the `Checkpoint` enum every adapter returns.
- **`kb-decision-0019` — `medium`.** The decision is that the port grows *nothing*, and the
  reversal — adding a method or an error variant later — is additive rather than breaking, though
  PS-26's frozen per-projection policy shapes the typed layer above it. `medium` is the honest
  cell and `02` states the reasoning rather than leaving the value to look measured.

### 5. `source_paths` keeps the intake path

Every new atom carries its originating `.kb/_intake/…` path plus the long-form record and the
grounding evidence. Every path in this plan was tested against the worktree (`00`, Provenance)
with one deliberate exception, `experiments/live-handle-projection-batch/`, which does not exist
and is therefore **not** put in `source_paths` — it is stated in the atom's body as the
disposition ADR-0017 decided, and carried in `unresolved`.

### 6. What is *not* extracted

- **The wave-id and one-wave-not-three guidance** (`0017-c0`). Orchestration for the runner;
  obeyed, not stored.
- **The "Not proposed" sections** (all four files). A negative scope statement is a guardrail on
  the integrate pass. Enforced here; no atom.
- **Anything under `crates/**` or `spec/SPECIFICATION.md`.** All four documents forbid it and
  this plan touches neither.

---

## Ops 1–3 — the three phase-6 decisions

Three `create_new` operations in ADR-number order, so that a `depends_on` target is always
created before the atom naming it (0018 and 0019 both depend on 0017). All three share
`kind: decision`, `status: accepted`, `authority_tier: decision`, `phase: 6`,
`classification: extends`, and `mapsImpact: decisionMap + domainMap`.

### Op 1 — ADR-0017

| | |
| --- | --- |
| **op** | `create_new` |
| **kind** | `decision` |
| **destPath** | `.kb/decisions/0017-what-a-projection-batch-owns.md` |
| **sourceFiles** | `.kb/_intake/2026-08-13-adr-0017-projection-batch.md` |
| **classification** | `extends` |
| **mapsImpact** | decisionMap, domainMap |

```yaml
id: kb-decision-0017
title: What a projection batch owns, and the seam that is not a write vocabulary
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0017
reversibility: low
phase: 6
supersedes: null
superseded_by: null
summary: >-
  Accepted with two halves provisional and both falsifiers named - PS-9/PS-11 falls if a second
  generic consumer appears, a dead-letter recorder or counter projection happenstance itself
  ships, counted at the typed-layer phase exit; PS-15 falls to a zero-cost type-level
  construction that names an instance, composes with async fn and still permits a batch in a
  collection, at which point the clause is replaced by a compile error. ProjectionStore::Batch
  becomes an owned type Batch with no lifetime parameter, on two compiler transcripts rather than
  on the Send argument - error[E0195] on every impl, and a DefId::expect_local ICE proving
  today's GAT port is implementable only by stores that outlive every batch. The Send argument is
  refuted in this workspace, since LadybugDB's Connection is Send and Sync and a genuinely
  borrowed handle would have bound to a GAT under the Send flavour. An owned batch does not close
  the foreign-batch hazard, because a lifetime names a region and not an instance, so PS-15 stays
  provisional and is discharged at run time by a per-store-instance stamp surfacing as
  CommitError::ForeignBatch. The write seam is split by consumer rather than universalised - no
  write vocabulary on Batch, and a ProjectionProbe: ProjectionStore in the contract crate behind
  feature = "conformance", bare flavour only, because an adapter's tests/ is a third crate where
  neither a testkit trait nor the adapter's type is local and the orphan rule rejects the impl.
  Dropping a batch rolls back and leaves the store usable, and rollback stays on the port because
  Rust has no async Drop. LiveHandleProjectionStore moves to experiments/ rather than being
  deleted - it is the only compiled evidence against this decision's own first claim. Rejected -
  a ProjectionBatch supertrait carrying put/get, which obliges every store into a key-value table
  and reintroduces at the read-model layer the opaque blob ADR-0003 confined to payloads; the
  probe trait in happenstance-testkit; keeping the GAT; a generative brand, which works and
  forbids the batch escaping the closure the hazard is about; and tying the batch to the
  receiver's lifetime, compiled and refuted. Answers the projection-batch apply-seam question's
  sub-questions 1, 2 and 4, leaving sub-question 3 with the typed layer. Names PS-8's and PS-13's
  clause/rule pairing defects as gaps rather than repairs, and repairs neither.
depends_on:
  - kb-decision-0007
  - kb-decision-0008
related:
  - kb-decision-0003
  - kb-decision-0010
  - kb-reference-port-traits-compiled-findings-001
  - kb-open-question-projection-batch-no-apply-001
  - kb-open-question-ps-1-no-progress-obligation-001
  - kb-playbook-repair-frozen-clause-001
  - kb-playbook-one-decision-per-adr-title-001
  - kb-open-question-adr-status-vocabulary-001
source_paths:
  - .kb/_intake/2026-08-13-adr-0017-projection-batch.md
  - references/adr/0017-what-a-projection-batch-owns.md
  - references/adapter-shapes.md
  - references/evaluation/ps-clause-pairing-sweep.md
  - crates/happenstance-ladybug/src/live_handle.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-13
```

**Why `depends_on` names 0007 and 0008 and not more.** 0008 delegated the GAT's survival to
phase 6 in its own accepted body (`:80-81`); 0007 is the decision that put a `Batch` in a runner's
hands at all. ADR-0003 is `related` rather than `depends_on` because it is cited by a *rejected
alternative* — the `put`/`get` supertrait — and a decision does not depend on the reasoning it
declined to repeat. ADR-0010 is `related` because `ProjectionProbe` is a conformance instrument
and 0010 owns what the suite may contain.

**Body must carry, and must not.** Must: the three graded halves with the strength table; the
`E0195` and ICE citations by `file:line` into `references/adapter-shapes.md`; the orphan-rule
citation (`spec/SPECIFICATION.md:5015-5031`); all seven rejected alternatives with their reasons;
the `LiveHandleProjectionStore` disposition; and the PS-8/PS-13 gap-naming with the explicit
sentence that it repairs nothing. Must not: reproduce the transcripts, the ICE minimisation, or
the sweep's census — all three are one `file:line` away.

### Op 2 — ADR-0018

| | |
| --- | --- |
| **op** | `create_new` |
| **kind** | `decision` |
| **destPath** | `.kb/decisions/0018-returning-a-projection-to-never-run.md` |
| **sourceFiles** | `.kb/_intake/2026-08-13-adr-0018-reset.md` |
| **classification** | `extends` |
| **mapsImpact** | decisionMap, domainMap |

```yaml
id: kb-decision-0018
title: Returning a projection to never run — scope, atomicity, and refusal
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0018
reversibility: low
phase: 6
supersedes: null
superseded_by: null
summary: >-
  Accepted with two of its four claims provisional and both falsifiers named - reset's atomicity
  falls to an adapter whose read-model clearing cannot be expressed through the same batch that
  carries ordinary writes, a store whose TRUNCATE cannot join the checkpoint transaction being
  the shape to watch; and the refusal mechanism falls if no adapter ever implements protection,
  evaluated at the projection-port phase exit by asking whether the SQLite adapter did. The four
  claims are of three strengths and are deliberately not levelled. reset(batch, id) applies the
  caller's batch and returns that id's checkpoint to NeverRun as one unit of work; the caller
  fills the batch with its own deletes because the port has no idea what the read model is, and
  ProjectionProbe::probe_delete_all exists so the suite can exercise it without knowing either.
  What the clause models is an incident rather than an abstraction - a two-statement runbook
  procedure on two connections where the truncate committed, the pod died two seconds later, and
  the projection then applied sixty-one events into an empty table and reported healthy. Scope is
  one (store, ProjectionId) pair, settled already by ADR-0007 and cited rather than re-derived,
  because a second independent derivation of one commitment is two things to keep in agreement.
  Refusal is a port mechanism, ResetError::Refused, with the policy left in the domain - a
  refusal leaves both halves unchanged and is never reported as success - because a policy with
  no port-level mechanism is bypassed by anyone holding the store, which is every operator with a
  runbook. Checkpoint is a three-variant enum, NeverRun, Live and Rebuilding, settled by being
  made - the pair of an optional position and a bool loses because it can spell authoritative and
  never run at once, which means nothing. Rejected - the runbook procedure itself; a reset that
  clears the rows, which would require the adapter to know which tables belong to a ProjectionId;
  refusal as purely a typed-layer concern; and re-deriving ADR-0007's scope. Whether
  ResetError::Refused carries the store's stated reason is left to the design record. PS-19's
  pairing defect sits inside this clause range and is named, attributed and not repaired.
depends_on:
  - kb-decision-0007
  - kb-decision-0017
related:
  - kb-decision-0013
  - kb-open-question-ps-19-scope-narrower-001
  - kb-open-question-global-vs-boundary-visibility-001
  - kb-playbook-repair-frozen-clause-001
  - kb-playbook-one-decision-per-adr-title-001
  - kb-open-question-adr-status-vocabulary-001
source_paths:
  - .kb/_intake/2026-08-13-adr-0018-reset.md
  - references/adr/0018-returning-a-projection-to-never-run.md
  - references/adapter-shapes.md
  - references/evaluation/ps-clause-pairing-sweep.md
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-13
```

**The `related` edge to `kb-decision-0013` is not a conflict marker.** A boundary-scoped
projection checkpoint surfaced while §2 was being drafted and was **filed, not absorbed**,
because absorbing it would reopen a globally frozen invariant.
`kb-open-question-global-vs-boundary-visibility-001` already owns it and **is not edited by this
wave** — the outbound `related` edge from this atom is the whole of the wiring, and its
reciprocal is the Maps phase's.

**Body must carry, and must not.** Must: the four-claim strength table; the incident, quoted
rather than paraphrased, because the decision rests on it; the explicit statement that PS-17 is
ADR-0007's and is cited, not re-derived; the five rejected alternatives; the honest form of the
evidence's **absence** — no skeleton exercised `reset`, every body is `todo!()`
(`references/adapter-shapes.md:286-303`), quoted rather than replaced with a fabricated
diagnostic; and the PS-19 gap-naming with its attribution to
`unstable-projection-gate-and-clause-disposition`. Must not: decide whether `ResetError::Refused`
carries a reason (that is `projection-api-design-record`, AC-007), and must not restate PS-19's
question — it has an atom, amended in Op 5.

### Op 3 — ADR-0019

| | |
| --- | --- |
| **op** | `create_new` |
| **kind** | `decision` |
| **destPath** | `.kb/decisions/0019-what-happens-when-apply-fails.md` |
| **sourceFiles** | `.kb/_intake/2026-08-13-adr-0019-apply-failure.md` |
| **classification** | `extends` |
| **mapsImpact** | decisionMap, domainMap |

```yaml
id: kb-decision-0019
title: What happens when apply fails — the port grows nothing
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0019
reversibility: medium
phase: 6
supersedes: null
superseded_by: null
summary: >-
  Accepted with two halves provisional and both falsifiers named - PS-27 falls if no projection
  ever writes a skip record, that is, if record turns out to mean log a warning, evaluated at the
  typed-layer phase exit against the Kestrel Motor shred case; and PS-30 falls if the fan-out
  runner is never built, which is decided by whether the poll cost of N independent reads is
  real, a benchmark this workspace has no harness for. The decision itself is stated as a
  decision rather than as an omission: ProjectionStore grows no method, no associated type, no
  error variant and no feature on the failure path. Three facts make that true rather than
  convenient. The skip primitive already exists and nobody had noticed it - begin followed
  immediately by commit with the poisoned position and Live applies nothing and advances the
  checkpoint atomically, with the port exactly as written; what does not exist is any way to
  record that it happened, and routing that record through the projection's own batch adds no
  port surface and gives the store no opinion about what a skip means. rollback must survive the
  port change, because a fan-out runner wrapping a batch in AssertUnwindSafe is defensible only
  while rollback exists to discharge the promise - without it the promise is a lie, and Rust has
  no async Drop. Isolation already works structurally, since Projection::Store is an associated
  type and checkpoints are per store and projection id under ADR-0007, so a failure cannot span
  two. The failure policy is declared per projection and not per runner, rejecting the runner-level
  on_error configuration a builder API invites, because the scenarios disagree by design - halting
  is right for a revenue ledger and wrong for an availability board. Three things are deferred by
  name rather than designed, all to the typed-layer phase: the pump error type with its three
  parameters, the skip-and-record vocabulary and what record means, and the supervisor's
  observability half. Rejected - a port-level skip method, which is commit with a different name;
  a port-level CommitError::ApplyFailed, since apply is the projection's and the failure never
  reaches commit; designing the pump error here, which would make this the decision the typed
  layer has to supersede on its first day; and deleting rollback. PS-29's pairing defect and
  PS-28's undetermined verdict are named and left to their owner.
depends_on:
  - kb-decision-0007
  - kb-decision-0017
related:
  - kb-open-question-ps-1-no-progress-obligation-001
  - kb-playbook-repair-frozen-clause-001
  - kb-open-question-adr-status-vocabulary-001
source_paths:
  - .kb/_intake/2026-08-13-adr-0019-apply-failure.md
  - references/adr/0019-what-happens-when-apply-fails.md
  - references/adapter-shapes.md
  - references/evaluation/ps-clause-pairing-sweep.md
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-13
```

**Body must carry, and must not.** Must: the deferral table with all three rows, their stated
locations and their owner, because *"an atom that designs any of the three has taken another
project's scope"*; the stated absence as evidence (`references/adapter-shapes.md:286-303`) with
the reason a fabricated diagnostic would be worse than none; the six rejected alternatives;
PS-29's gap-naming and PS-28's `undetermined` verdict with the two readings of *"the last good
position"*. Must not: restate the cross-cutting finding that these rules were written to the
clause's intent rather than its sentence — that is CL-2 and lands **once**, on
`kb-open-question-ps-1-no-progress-obligation-001` in Op 6, which this atom links.

---

## Ops 4–6 — the three existing atoms this wave amends

All three are `open_question` atoms carrying `authority_tier: note`, so amending them is
permitted and no immutability rule is in play. All three keep their bodies verbatim: per
`.kb/open-questions/README.md`, *"Do not rewrite a question into its own answer: the value of the
record is that it shows the state of knowledge on the day the choice was made."* Every addition
is an **appended, dated annotation** plus frontmatter edges — **no existing sentence is deleted
or rewritten in any of the three**, including the one sentence the sweep refutes.

These three run **after** Ops 1–3 and **serially**, because two of them link to atoms Ops 1–2
create.

### Op 4 — the apply seam has an answer

| | |
| --- | --- |
| **op** | `merge_existing` |
| **kind** | `open_question` |
| **destPath / mergeTargetPath** | `.kb/open-questions/projection-store-batch-has-no-apply-seam.md` |
| **sourceFiles** | `.kb/_intake/2026-08-13-adr-0017-projection-batch.md` |
| **classification** | `extends` |
| **mapsImpact** | openQuestionIndex, domainMap |

```yaml
id: kb-open-question-projection-batch-no-apply-001
title: ProjectionStore::Batch carries no bounds, so nothing can write to it
kind: open_question
status: superseded
authority_tier: note
summary: >-
  [unchanged, plus one appended sentence] … Answered 2026-08-13 by ADR-0017 (kb-decision-0017),
  which makes Batch an owned type with no lifetime parameter and declines a universal write
  vocabulary in favour of a ProjectionProbe in the contract crate behind feature = "conformance":
  sub-questions 1, 2 and 4 are settled, and sub-question 3 - whether closing this retroactively
  validates ADR-0006's encoding-versus-orchestration discriminator - is not, and stays with the
  typed layer.
depends_on: []
related:
  - kb-decision-0007
  - kb-decision-0008
  - kb-decision-0017
  - kb-open-question-ps-1-no-progress-obligation-001
  - kb-open-question-ps-19-scope-narrower-001
  - kb-open-question-global-vs-boundary-visibility-001
  - kb-open-question-projection-id-unvalidated-001
source_paths:
  - .kb/_intake/0007-projection-runner-decodes.md
  - .kb/_intake/2026-08-13-adr-0017-projection-batch.md
  - references/adr/0007-projection-runner-decodes.md
  - references/adr/0017-what-a-projection-batch-owns.md
  - crates/happenstance-core/src/projection.rs
  - RUNBOOK.md
last_reviewed: 2026-08-13
```

**`superseded`, not `withdrawn`, and the choice is not cosmetic.** The layer README offers both.
`withdrawn` says the question stopped being worth asking; `superseded` says a later atom now
holds the answer. A decision atom answers this one, and the reader who arrives at the question
needs to be sent to it — so `superseded`, with `kb-decision-0017` on `related`. `superseded_by`
is **not** added: the `KbFrontmatter` authoring set reserves `supersedes`/`superseded_by` for
`decision` atoms, and inventing a key on an `open_question` would validate silently and then be
read as corpus fact by every later wave.

**The residual is stated, not hidden.** Sub-question 3 is unanswered and the annotation must say
so in the same breath as the answer, per the intake: *"Say so explicitly in the annotation, so
the omission reads as a decision."* A question closed with a live sub-question inside it and no
note is indistinguishable from one closed carelessly.

**Must not:** delete the atom, rewrite any of its four sub-questions, or edit the paragraphs that
describe what was true in 2026-08-10. The bullet on the open-questions index is annotated and
**stays listed**, with its status changed from **Open** to **Superseded** — the index's own
summary already promises that a superseded question stays listed rather than removed.

### Op 5 — PS-19: two documents, one atom (CL-1)

| | |
| --- | --- |
| **op** | `merge_existing` |
| **kind** | `open_question` |
| **destPath / mergeTargetPath** | `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md` |
| **sourceFiles** | `.kb/_intake/2026-08-13-adr-0018-reset.md`, `.kb/_intake/2026-08-13-ps-clause-pairing-sweep.md` |
| **classification** | `extends` |
| **mapsImpact** | openQuestionIndex |

```yaml
id: kb-open-question-ps-19-scope-narrower-001
title: PS-19's MUST is scoped after a reset; its second rule asks about an unseen id
kind: open_question
status: accepted
authority_tier: note
summary: >-
  [unchanged, plus one appended sentence] … Amended 2026-08-13: the 37-clause pairing sweep
  answers sub-question 2 - the systematic §4.11-table hypothesis is not supported, there being
  two defects inside the table and both already known, so three point repairs and one recorded
  lesson is the shape - and the finding itself reproduces against a sharper exposing store, so
  the atom is confirmed rather than corrected. ADR-0018 (kb-decision-0018) scoped this defect out
  of its own range by name and repaired nothing; sub-questions 1 and 3 stay open, owner
  unchanged.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-repair-frozen-clause-001
  - kb-open-question-ps-1-no-progress-obligation-001
  - kb-open-question-post-phase-reconciliation-001
  - kb-open-question-projection-batch-no-apply-001
  - kb-decision-0018
source_paths:
  - .kb/_intake/gaps-owed-a-decision.md
  - .kb/_intake/2026-08-13-adr-0018-reset.md
  - .kb/_intake/2026-08-13-ps-clause-pairing-sweep.md
  - references/evaluation/ps-clause-pairing-sweep.md
  - references/adr/0018-returning-a-projection-to-never-run.md
  - spec/SPECIFICATION.md
  - RUNBOOK.md
last_reviewed: 2026-08-13
```

**This is the wave's clearest cross-file merge and the reason the four files were staged as one
wave.** Two independent documents reached this atom on the same day by different routes: ADR-0018
met PS-19 inside its own clause range and scoped it out; the sweep met it as sub-question 2's
subject and answered it. Ingesting them as two waves would have produced two annotations, two
dates and two accounts of one defect. **One annotation, both citations.**

**One appended annotation carries both.** The sweep's proposed sentence and ADR-0018's optional
one are merged rather than stacked: the sweep's is the answer to a sub-question and ADR-0018's is
an attribution, and a reader needs them in one place. The annotation must cite
`references/evaluation/ps-clause-pairing-sweep.md` by path and name
`unstable-projection-gate-and-clause-disposition` as where the repair lands.

**`status` stays `accepted`.** Both source documents say so independently — the sweep
(*"both questions stay accepted and open"*) and ADR-0018 (*"None is resolved by this atom"*). One
of four sub-questions answered is not a resolution.

**The apparent contradiction in the intake, resolved.** The ADR-0017 document's "Not proposed"
section says there is *"no resolution of `kb-open-question-ps-19-scope-narrower-001`"* while the
sweep instructs an amendment to it. These do not conflict: an amendment that answers one
sub-question and leaves `status: accepted` is not a resolution. Both instructions are satisfied
by this operation, and the wave records the reconciliation rather than choosing a side.

### Op 6 — PS-1: an answer, a refutation, and a habit (CL-2)

| | |
| --- | --- |
| **op** | `merge_existing` |
| **kind** | `open_question` |
| **destPath / mergeTargetPath** | `.kb/open-questions/ps-1-states-no-progress-obligation.md` |
| **sourceFiles** | `.kb/_intake/2026-08-13-ps-clause-pairing-sweep.md`, `.kb/_intake/2026-08-13-adr-0019-apply-failure.md` |
| **classification** | `extends` |
| **mapsImpact** | openQuestionIndex |

```yaml
id: kb-open-question-ps-1-no-progress-obligation-001
title: PS-1's MUST is a coupling, not a progress obligation
kind: open_question
status: accepted
authority_tier: note
summary: >-
  [unchanged, plus one appended sentence] … Amended 2026-08-13: the 37-clause pairing sweep
  answers sub-question 3 - isolated, at 29 sound, 7 defective and 1 undetermined, with only PS-1
  and PS-19 involving §4.11's table - and refutes this body's claim that no clause's MUST states
  progress, since PS-23's "one commit advances exactly one ProjectionId" excludes zero, on a
  PROVISIONAL clause about fan-out scope. The PS-1 pairing defect itself reproduces
  independently, so what changes is the candidate repair, now scoped as which clause states
  progress and which rules rest on it; three rules (PS-8, PS-21, PS-22) rest on the missing
  obligation, and the same intent-not-sentence habit recurs outside the table on PS-29, which
  ADR-0019 (kb-decision-0019) names in its own range. Sub-questions 1, 2 and 4 stay open, owner
  unchanged.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-repair-frozen-clause-001
  - kb-open-question-ps-19-scope-narrower-001
  - kb-open-question-post-phase-reconciliation-001
  - kb-open-question-projection-batch-no-apply-001
  - kb-decision-0019
source_paths:
  - .kb/_intake/gaps-owed-a-decision.md
  - .kb/_intake/2026-08-13-ps-clause-pairing-sweep.md
  - .kb/_intake/2026-08-13-adr-0019-apply-failure.md
  - references/evaluation/ps-clause-pairing-sweep.md
  - references/adr/0019-what-happens-when-apply-fails.md
  - spec/SPECIFICATION.md
  - RUNBOOK.md
last_reviewed: 2026-08-13
```

**The refutation is an append, and this is the operation most likely to be got wrong.** The
sentence *"That a successful commit advances anything … is stated by no clause's `MUST` anywhere
in the document"* is false as of today's clause text. The tempting fix is to correct the
paragraph. **It must not be corrected.** The layer README bars it, the sweep bars it in terms
(*"an appended, dated amendment note naming the refuted sentence … not an edit to the original
paragraph"*), and the corpus has a governance atom for exactly this discrimination
(`kb-governance-referent-not-reasoning-001`). The annotation quotes the refuted sentence, states
what refutes it, cites `spec/SPECIFICATION.md:5317-5329`, and says what does **not** change: the
PS-1 pairing defect reproduces independently from `:4733-4759`.

**CL-2 lands here and nowhere else.** ADR-0019 independently observed that PS-29's defect comes
from *"a rule written to the clause's intent rather than to its sentence — a habit, distributed
across the family, not one table's artefact"*. That is the same finding as the sweep's first
qualification, and it belongs on the atom whose sub-question 3 asked whether the defect was
systematic. `kb-decision-0019` links to this atom; it does not restate the finding.

**`status` stays `accepted`**, for the same reason as Op 5, and the sweep says so directly.

---

## Adjudications

**1. The wave's one candidate for `supersede` is a `depends_on`.** ADR-0017 removes the `Batch`
GAT that `kb-decision-0008` describes. `supersede` was considered and rejected on 0008's own
accepted text: *"phase 6 must decide whether the GAT survives"* (`:80-81`). A decision that asks
a question and a later decision that answers it are not in conflict; nothing in 0008's body
becomes false, so there is nothing to correct and no status to flip. Flipping an accepted atom on
the strength of a question it asked would be the immutability rule used backwards. **Zero
accepted decision atoms are edited or flipped this wave.**

**2. `supersedes: null` on all three new atoms, and one supersession is *owed* and not performed.**
ADR-0017's record states that it *"corrects one sentence of ADR-0007's Context, per PS-32, and
that correction is a superseding atom's rather than an edit's"*. Both halves are honoured: the
sentence is **not** edited, and the superseding atom is **not** written here. Writing it would be
this wave authoring a decision nobody signed. It is carried in `unresolved` so the debt is
visible rather than absorbed.

**3. No `open_question` atom is created, and that is a finding rather than a gap.** Five pairing
defects are named across the four files — PS-8, PS-13, PS-19, PS-28, PS-29 — and PS-19 is the
only one with an atom, because it had one already. The other four are **not** filed as questions,
for a reason the corpus states in its own README: *"Nor does a task belong here. Work that
someone is expected to do is a backlog item in `.bklg/`, not a KB atom."* All four repairs have
an owner, verified by `grep`: `unstable-projection-gate-and-clause-disposition` (HS-P0010), whose
AC-014 carries the clause disposition, and HS-P0011 for PS-28's definition. Each defect is named
in the decision atom whose clause range contains it, which is where a reader meets it. The
asymmetry with PS-1 and PS-19 — questions with atoms, in the same family, of the same shape — is
real, and is carried in `unresolved` so a human can overrule rather than discover it later.

**4. No `reference` atom is minted for the sweep, and the option was live.** Sweep claim `1d`
offers one conditionally and names the precedent —
`kb-reference-phase-4-5-spec-reconciliation-001` was minted for a comparable evaluation document.
Declined this wave on three grounds: the sweep's own adjudication bias says *"No new atom is
proposed"* and warns the adjudicator who finds itself minting one that it has misread the
document; the sweep is registered evidence pinned to a commit (`2136dde`) and citable by path
today, which is exactly the state `phase-4-5-reconciliation.md` was in before its atom was minted
in a *later, separate* act; and authority rule 3's default bias is merge and link over creating.
Five destinations in this wave cite it by path. Carried in `unresolved`, because a later wave
minting it should know this one considered it.

**5. The two candidate questions the sweep named and declined are recorded, not lost.** (a) PS-3,
PS-31 and PS-36 carry a *documentation* obligation with no instrument — routed to the rustdoc
obligations in `standards/rust/70-rustdoc-obligations.md`, not to a KB atom. (b)
`rebuild_is_chunk_size_invariant` is ungated while `batch_reads_reflect_pending_writes` carries
`READS_THROUGH_BATCH`, so a write-behind adapter blessed by PS-4 and PS-12's second arm cannot
pass it — routed to ADR-0017 and the suite stories. Neither is minted; both are named here and in
`unresolved` so that declining them stays a decision rather than becoming an oversight.

**6. Six open questions are named "do not touch" and are not touched.**
`kb-open-question-ps-1-no-progress-obligation-001` and `…-ps-19-scope-narrower-001` are *amended*
rather than resolved (Op 5's reconciliation note), and `…-cf-40-ownership-001`,
`…-projection-id-unvalidated-001`, `…-global-vs-boundary-visibility-001`,
`…-post-phase-reconciliation-001` and `…-adr-status-vocabulary-001` receive **no edit at all**.
The last of the five is the interesting one: this wave applies a status convention for the third
time and still does not close the question the convention exists to work around.

**7. Nothing was routed to `product/`, `design/`, `narratives/`, `roadmaps/`, `concepts/`,
`governance/` or `reference/`.** Same as both previous waves for the first four, and for the last
three: no new transferable *method* appeared — the repair-or-gap test was applied five times and
`kb-playbook-repair-frozen-clause-001` already carries it, and applying a playbook is not a new
playbook.

## What the Maps phase inherits

- **Three decision-map rows**, in ADR-number order, under a **new `##` section** per the map's own
  "Adding a row" instruction. Proposed heading: `## 2026-08-13 phase-6 projection port
  (ADR-0017–0019)`. All three cells for supersession are `—`; all three are `accepted (provisional
  in named parts)` in the Status column, matching how `kb-decision-0003` is already rendered
  there.
- **Domain-map placement for three decision atoms.** They belong with *Contract ports,
  conformance, and the ADR corpus*, whose orientation paragraph currently says the ADR corpus runs
  "async port flavours through the wire format" — that sentence is now short by three and needs
  extending, or a new `## The projection store port (phase 6)` section opened. The grouping is the
  Maps phase's call, per that map's own instruction, but **an atom absent from the map is a `pub
  use` missing from `lib.rs`**: it validates and nobody can find it. The domain map's
  "Open questions this domain owns" list must also change `kb-open-question-projection-batch-no-apply-001`
  from an open question to a superseded one, or annotate it.
- **Three open-questions-index bullets, none removed.**
  - `projection-store-batch-has-no-apply-seam.md`: status **Open → Superseded**, plus
    *"Answered 2026-08-13 by ADR-0017 (`kb-decision-0017`); sub-question 3 — whether this
    retroactively validates ADR-0006's discriminator — stays open with the typed layer."*
  - `ps-1-states-no-progress-obligation.md` stays **Open** and gains, in the existing
    *"Amended 2026-08-10: …"* style: *"Amended 2026-08-13: the 37-clause sweep answers
    sub-question 3 — isolated — and refutes the body's claim that no clause's `MUST` states
    progress; PS-23's 'exactly one' does, on a `[PROVISIONAL]` clause about fan-out scope. See
    `references/evaluation/ps-clause-pairing-sweep.md`."*
  - `ps-19-scope-narrower-than-its-rule.md` stays **Open** and gains: *"Amended 2026-08-13: the
    sweep answers sub-question 2 — the systematic §4.11-table hypothesis is not supported; the
    finding itself reproduces. ADR-0018 (`kb-decision-0018`) scoped it out by name and repaired
    nothing. See `references/evaluation/ps-clause-pairing-sweep.md`."*
- **Reciprocal backlinks.** Every link in this plan is outbound to an atom that already exists or
  that an earlier op created. The inbound halves are the Maps phase's: `kb-decision-0017` ←
  `kb-decision-0018`, `kb-decision-0019`; `kb-decision-0007` and `kb-decision-0008` ← all three
  new atoms; `kb-open-question-global-vs-boundary-visibility-001` ← `kb-decision-0018` (an edge
  only — **the question's own frontmatter and body stay untouched**, so if the reciprocal cannot
  be added without editing it, it is not added).
- **The flags, stated once.** `decisionMap: true` on **Ops 1–3 only** — the decision map is one
  row per decision atom, and amending an open question adds no row, however many decision ids its
  `related` list gains. `openQuestionIndex: true` on **Ops 4–6**. `domainMap: true` on **Ops 1–3**
  (three atoms to place) **and Op 4** (the domain's "open questions this domain owns" list names
  an atom that is no longer open).
