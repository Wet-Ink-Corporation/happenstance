# Wave `2026-08-10-intake` — placement and adjudication

The ordered action plan. Twelve operations, one per destination atom. Every link is **outbound
to an atom created by an earlier operation**, so the Integration phase can run creates in
parallel without a dangling reference; reciprocal backlinks are the Maps phase's.

No operation edits an accepted decision, because none exists. No operation authors a decision
atom — see "Adjudications" below.

## Standing choices, applied to every atom in the wave

**`status: accepted`.** Not `draft`. These claims are the output of a completed, six-commit
pass, each commit green under `cargo xtask ci`; nothing about them is pending. For the seven
`open_question` atoms it reads correctly too: the open-questions README's resolution path is
`accepted → withdrawn | superseded` when the answer arrives, so `accepted` is the live state of
a question, not a claim that it has been answered. The immutability check applies to
`kind: decision` only, so nothing here becomes unamendable by this choice.

**`authority_tier`** follows the layer READMEs exactly: `note` in `reference/` and
`open-questions/`, `guideline` in `playbooks/`. Nothing invented.

**`source_paths`** keeps the originating `.kb/_intake/…` path on every atom — the intake files
are deleted by a successful ingest, and the atom must still say where it came from — plus the
repo paths that ground it, all of which were verified to resolve in this worktree (`00`).

**The owning phase is written in the body, not the frontmatter.** `gaps-owed-a-decision.md` asks
that "where a question names a phase, that phase should appear on the atom". `phase` is a
decision-only key in `KbFrontmatter`; putting it on an `open_question` would be inventing a key
on an atom I am authoring, which nothing downstream would catch. The obligation is met in the
summary and the body instead, where a reader and a grep both find it.

**No `file:line` citations in summaries.** Bodies carry them because the evidence demands it,
but nothing checks citations inside `.kb/` — see `unresolved`.

---

## Op 1 — the census

| | |
| --- | --- |
| **op** | `create_new` |
| **kind** | `reference` |
| **destPath** | `.kb/reference/phase-4-5-specification-reconciliation-census.md` |
| **sourceFiles** | `2026-08-10-phase-4-5-pressure-test.md`, `lesson-repairing-a-frozen-clause-without-amending-it.md`, `lesson-anchoring-citations-in-a-long-lived-document.md` |
| **classification** | `extends` |
| **mapsImpact** | domainMap |

```yaml
id: kb-reference-phase-4-5-spec-reconciliation-001
title: The phase 4/5 specification reconciliation — census and pointer
kind: reference
status: accepted
authority_tier: note
summary: >-
  What the unscheduled phase 4/5 specification reconciliation found, counted by class of defect:
  six commits (3c704d3 through 84dcc67 on redkiln-adoption, each green under cargo xtask ci)
  against spec/SPECIFICATION.md, and what cargo xtask spec-trace reports for the document on
  2026-08-10 — 200 clauses (139 FROZEN), 95 conformance rules, 358 citations checked of which 69
  are anchored to their subject. The evidence stays in references/evaluation/phase-4-5-reconciliation.md;
  this atom is the citable pointer, the counts, and the residual defects the pass left recorded.
depends_on: []
related: []
source_paths:
  - .kb/_intake/2026-08-10-phase-4-5-pressure-test.md
  - .kb/_intake/lesson-repairing-a-frozen-clause-without-amending-it.md
  - .kb/_intake/lesson-anchoring-citations-in-a-long-lived-document.md
  - references/evaluation/phase-4-5-reconciliation.md
  - references/evaluation/review-citation-drift.md
  - RUNBOOK.md
  - spec/SPECIFICATION.md
  - xtask/src/spec_trace.rs
  - xtask/src/lint_constitution.rs
last_reviewed: 2026-08-10
```

**Rationale.** The wave's one three-way merge (CL-1, scored 95 / 88 / 62 in `00`). Three intake
files state the same dated measurement of the same document; the extract pass proposed a second
census atom at `.kb/reference/specification-frozen-clause-census.md` for the 200/139 figure, and
that is refused — the reference README's own words, "two copies of a measurement is two things to
update and one that quietly goes stale". This atom owns every number in the wave. The playbooks
cite it; they do not restate it.

Created first because five later atoms link to it.

**Must contain, and must not.** Must: what the pass was; the six defect classes with counts and
owning commits; the `spec-trace` summary line verbatim with its date; the independent
corroboration by `review-citation-drift.md` and why convergence is itself the finding; the seven
unscheduled findings as links to Ops 6–12; and the residual defect —
`const ANCHOR_SLACK: usize = 12` at `xtask/src/spec_trace.rs:391` against `= 10` at
`xtask/src/lint_constitution.rs:111`, with the doc comment that claims they match, **verified by
grep on 2026-08-10 at line 391 and not the `:381` the intake cites.** Must not: any
finding-by-finding detail from the 50KB evaluation document. That prohibition is the first
instruction in the intake file and the reason this atom exists as a pointer.

---

## Op 2 — the checker's two obligations

| | |
| --- | --- |
| **op** | `create_new` |
| **kind** | `playbook` |
| **destPath** | `.kb/playbooks/verify-the-referent-and-report-coverage.md` |
| **sourceFiles** | `lesson-a-check-that-verifies-the-address-not-the-referent.md` |
| **classification** | `extends` |
| **mapsImpact** | domainMap |

```yaml
id: kb-playbook-verify-referent-report-coverage-001
title: A cross-reference checker verifies the referent, and reports its own coverage
kind: playbook
status: accepted
authority_tier: guideline
summary: >-
  Two independent obligations for any checker over cross-references — citations, links, ids,
  schema $refs, test-to-requirement traceability. Verify the referent and not merely the address:
  that a file:line resolves says nothing about whether the attributed content is there. And report
  coverage: a check that does not state what fraction of the corpus it parsed is indistinguishable
  from one that sees all of it. Grounded in a parser that checked 84 of 338 citations while
  printing "no problems found", and in the one-line fix that made the claim falsifiable.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
source_paths:
  - .kb/_intake/lesson-a-check-that-verifies-the-address-not-the-referent.md
  - xtask/src/spec_trace.rs
  - xtask/src/lint_constitution.rs
  - spec/SPECIFICATION.md
  - references/evaluation/phase-4-5-reconciliation.md
last_reviewed: 2026-08-10
```

**Rationale.** `playbook`, not `decision`, despite two "must"s: the claim prescribes how to build
a checker in general and commits no interface, clause or crate — the playbooks README's own test
is whether reversing it would need a decision, and it would not (`01`). Kept separate from Op 3
at a score of 58: same subject, different method, different failure conditions.

Must carry the counter-consideration verbatim in force — coverage reporting makes a check's scope
legible, not correct; 81% of the corpus is still verified for addressing only. A playbook whose
own honest ceiling is edited out is a summary.

---

## Op 3 — the anchoring mechanism

| | |
| --- | --- |
| **op** | `create_new` |
| **kind** | `playbook` |
| **destPath** | `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` |
| **sourceFiles** | `lesson-anchoring-citations-in-a-long-lived-document.md` |
| **classification** | `extends` |
| **mapsImpact** | domainMap |

```yaml
id: kb-playbook-anchoring-citations-001
title: Anchoring citations in a document whose targets move under it
kind: playbook
status: accepted
authority_tier: guideline
summary: >-
  How to check file:line citations against a source tree that moves, without a checker that
  either passes forever or fires on every ordinary edit. Two properties decide the design — it
  must detect a moved target, and inserting lines above a cited item must not red the gate — which
  rules out a content hash and an exact line match and leaves a windowed search for a short subject
  string. Compares the explicit anchor spelling against the derived one with the cost of each,
  records four measured attempts that took the false-report rate from 118/316 to 2/262, and states
  the discriminator that made it work: a heuristic that cannot tell its own mistakes from the
  corpus's must decline rather than guess.
depends_on:
  - kb-playbook-verify-referent-report-coverage-001
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
source_paths:
  - .kb/_intake/lesson-anchoring-citations-in-a-long-lived-document.md
  - xtask/src/spec_trace.rs
  - xtask/src/lint_constitution.rs
  - spec/SPECIFICATION.md
  - standards/rust/README.md
  - references/evaluation/phase-4-5-reconciliation.md
  - references/evaluation/review-citation-drift.md
last_reviewed: 2026-08-10
```

**Rationale.** `depends_on` Op 2 because this mechanism is how Op 2's first obligation is
discharged in this repository; the dependency is real and directional, and it is why Op 2 is
ordered first.

Claim `C3` (the `ANCHOR_SLACK` disagreement) is **not** in this atom. The playbooks README bars a
one-off — "something true of exactly one file, one clause, or one afternoon" — and the intake
itself calls it "a one-line repair and not an ADR". It is a dated fact at a commit, so it lands in
Op 1 and this atom links to it. Claim `C2` (line-numbered citations are a standing tax, three
breakages in one pass) stays here as the required "conditions under which it stops holding".

---

## Op 4 — the ratchet

| | |
| --- | --- |
| **op** | `create_new` |
| **kind** | `playbook` |
| **destPath** | `.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md` |
| **sourceFiles** | `lesson-landing-a-stricter-gate-without-a-red-baseline.md` |
| **classification** | `extends` (flagged — see Adjudications) |
| **mapsImpact** | domainMap |

```yaml
id: kb-playbook-ratchet-gate-landing-001
title: Landing a stricter gate check when the corpus cannot pass it yet
kind: playbook
status: accepted
authority_tier: guideline
summary: >-
  What to do when tightening a gate check surfaces violations you are not authorised to fix.
  Waiting for a clean corpus lands the check after the decision it should have pressured; landing
  it report-only removes exactly that pressure. The third option is a ratchet: fatal from day one,
  with a named exemption list that prints every entry on every green run, fails when an entry
  becomes discharged so the list can only shrink, and carries the owed decision as an argument
  rather than a name. Includes why the tool change and the corpus change had to be one commit,
  why a list never carries a count beside it, and the corpus shape where a ratchet is the wrong
  instrument.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
source_paths:
  - .kb/_intake/lesson-landing-a-stricter-gate-without-a-red-baseline.md
  - xtask/src/spec_trace.rs
  - spec/SPECIFICATION.md
  - RUNBOOK.md
  - references/evaluation/phase-4-5-reconciliation.md
last_reviewed: 2026-08-10
```

**Rationale.** All five claims are one method with one applicability boundary; splitting `C3`
(never write a count beside a list) into its own atom was considered and refused — it is a
property of the mechanism described here, it is grounded in the same commit history, and alone it
is two sentences. `C5` is the "stops holding" clause the playbooks README requires.

The two exemption entries this atom describes are the two open questions in Ops 6 and 7. The
mechanism belongs here; the owed decisions belong in `open-questions/`, and neither absorbs the
other.

---

## Op 5 — repair versus amendment

| | |
| --- | --- |
| **op** | `create_new` |
| **kind** | `playbook` |
| **destPath** | `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` |
| **sourceFiles** | `lesson-repairing-a-frozen-clause-without-amending-it.md` |
| **classification** | `aligns` |
| **mapsImpact** | domainMap |

```yaml
id: kb-playbook-repair-frozen-clause-001
title: Repairing a frozen clause without amending it
kind: playbook
status: accepted
authority_tier: guideline
summary: >-
  How a pass that is not authorised to decide anything corrects a normative document whose clauses
  are frozen. The test is mechanical: a correction to a FROZEN clause is a repair if the set of
  implementations the clause admits is unchanged, and otherwise it is a gap, which is an ADR's.
  Gives the taxonomy of each, the safe form for an obligation that has been met — keep the MUST
  verbatim, name the discharge as a discharge, cite the code and the test that assert it — and the
  form for a gap, which is to record the defect inside the clause it is about and change nothing
  normative. Closes with why discovery and decision were kept in separate passes.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-ratchet-gate-landing-001
source_paths:
  - .kb/_intake/lesson-repairing-a-frozen-clause-without-amending-it.md
  - spec/SPECIFICATION.md
  - CLAUDE.md
  - RUNBOOK.md
  - xtask/src/spec_trace.rs
  - references/evaluation/phase-4-5-reconciliation.md
last_reviewed: 2026-08-10
```

**Rationale.** The wave's one substantive `aligns`: `.kb/decisions/README.md` already states the
repair-versus-amendment test in the corpus's own voice, and `CLAUDE.md` states the freeze rule it
operates under. The atom supplies the procedure, the two taxonomies and the PS-20 worked example
that neither README has room for, and cites both rather than restating them as new.

Claim `C4` (200 clauses, 139 frozen) is routed to Op 1. Claim `C5` is a pointer and produces no
atom: its content is the `related` edges to Ops 8, 9, 11 and 12, which the Maps phase wires as
backlinks since those atoms do not yet exist when this one is created.

---

## Ops 6–11 — the six gaps

All six: `op: defer_open_question`, `kind: open_question`, `status: accepted`,
`authority_tier: note`, `classification: requires-new-decision`,
`mapsImpact: domainMap + openQuestionIndex`, `sourceFiles: gaps-owed-a-decision.md`.

The intake's own instruction governs: **six atoms, not one.** Scored 15–40 against each other in
`00` — shared provenance and nothing more. The closest pair (gaps 3 and 4, at 40) is cross-linked
rather than merged, with the systematic-§4.11 suspicion recorded on both.

Each body follows the open-questions README's four-part shape — what is true today, what is not
decided, what forces it, ordered sub-questions — and each carries its owning phase in the body
because `phase` is a decision-only frontmatter key.

### Op 6 — `.kb/open-questions/disjoint-boundaries-have-no-clause.md`

```yaml
id: kb-open-question-disjoint-boundaries-no-clause-001
title: The DCB independence proposition is enforced by a rule and stated by no clause
kind: open_question
status: accepted
authority_tier: note
summary: >-
  k_disjoint_boundaries_admit_exactly_k_commits is a live conformance rule enforcing the
  independence proposition Dynamic Consistency Boundary exists for — commands sharing no
  consistency boundary do not conflict — and no clause in spec/SPECIFICATION.md states it; the
  word "disjoint" occurs zero times, verified by grep on 2026-08-10. ES-25 is the wrong attachment
  point: its only-if half forbids the false-positive direction and does not assert the positive
  proposition, so attaching there would claim a FROZEN clause contains something it does not.
  Settled by an ADR that widens ES-25 or mints a clause. Owner unassigned; held meanwhile in
  UNCLAIMED_PENDING_ADR, which prints it on every green gate run.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-ratchet-gate-landing-001
  - kb-playbook-repair-frozen-clause-001
source_paths:
  - .kb/_intake/gaps-owed-a-decision.md
  - crates/happenstance-testkit/src/concurrency.rs
  - spec/SPECIFICATION.md
  - xtask/src/spec_trace.rs
last_reviewed: 2026-08-10
```

The intake calls this "the sharpest of the six" — the library's central claim, tested and
unstated. That belongs in the body, not lost in a ranking.

### Op 7 — `.kb/open-questions/model-family-rule-has-no-clause.md`

```yaml
id: kb-open-question-model-family-rule-no-clause-001
title: The model family's rule checks the composition of seven clauses and belongs to none
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ops_agree_with_the_model is the model family's single conformance rule; it replays generated
  operation sequences against a model and therefore checks the composition of ES-8, ES-9, ES-11,
  ES-14, ES-15, ES-18 and ES-25 over inputs no clause enumerates. §6.4 names it only as CF-22's
  illustration, and CF-22's MUST is where the rule list lives rather than what any rule asserts.
  Distinct from the disjoint-boundaries gap: that is one proposition no clause states, this is
  several belonging to no single clause. Settled by an ADR minting a cross-clause property clause,
  or by deciding how cross-clause rules are disposed of — which becomes the precedent for every
  future property-based rule. Owner unassigned; held in UNCLAIMED_PENDING_ADR.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-ratchet-gate-landing-001
  - kb-open-question-disjoint-boundaries-no-clause-001
source_paths:
  - .kb/_intake/gaps-owed-a-decision.md
  - crates/happenstance-testkit/src/model.rs
  - spec/SPECIFICATION.md
  - xtask/src/spec_trace.rs
last_reviewed: 2026-08-10
```

Linked to Op 6 because the intake spends a paragraph distinguishing them, and a reader who finds
one will otherwise assume it covers the other.

### Op 8 — `.kb/open-questions/ps-1-states-no-progress-obligation.md`

```yaml
id: kb-open-question-ps-1-no-progress-obligation-001
title: PS-1's MUST is a coupling, not a progress obligation
kind: open_question
status: accepted
authority_tier: note
summary: >-
  PS-1 is FROZEN and says the read-model write and the checkpoint write MUST become durable
  together or not at all. §4.11 assigns it three rules, and the third does not follow from the
  sentence: a commit returning Ok that makes neither durable satisfies the "or not at all" arm,
  passes commit_is_atomic_with_the_read_model, and fails commit_advances_the_checkpoint. That a
  successful commit advances anything is stated by no clause's MUST — PS-22 presupposes it and
  §4.1a asserts it non-normatively. Adding the obligation changes the set of implementations the
  clause admits, so it is an ADR's and not an edit's. Owned by phase 6, which discharges
  PS-1 through PS-37 and settles ADR-0017, ADR-0018 and ADR-0019.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-repair-frozen-clause-001
source_paths:
  - .kb/_intake/gaps-owed-a-decision.md
  - spec/SPECIFICATION.md
  - RUNBOOK.md
last_reviewed: 2026-08-10
```

### Op 9 — `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md`

```yaml
id: kb-open-question-ps-19-scope-narrower-001
title: PS-19's MUST is scoped after a reset; its second rule asks about an unseen id
kind: open_question
status: accepted
authority_tier: note
summary: >-
  PS-19 is FROZEN and scoped to what checkpoint(id) returns after a successful reset, while §4.11
  additionally assigns it fresh_projection_has_no_checkpoint, which asks about an id never seen.
  The implementation that exposes the gap is the natural one: reset writes an explicit NeverRun
  sentinel and checkpoint(id) resolves a missing row with unwrap_or(Live { through: FIRST }) —
  satisfying the MUST verbatim and failing the rule. No clause obliges an unseen id to read
  NeverRun. Settled by an ADR widening PS-19 or minting a clause. Owned by phase 6. Interacts with
  the PS-1 gap: both are PS-layer clauses whose MUST is narrower than the rule table assigns them,
  so whoever takes either should first check the other 35 PS clauses for the same shape.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-repair-frozen-clause-001
  - kb-open-question-ps-1-no-progress-obligation-001
source_paths:
  - .kb/_intake/gaps-owed-a-decision.md
  - spec/SPECIFICATION.md
  - RUNBOOK.md
last_reviewed: 2026-08-10
```

The "check the other 35 PS clauses" instruction is the most actionable sentence in the file and
must survive into the body of both Op 8 and Op 9 — a reader arriving at either one is the reader
who can act on it.

### Op 10 — `.kb/open-questions/es-6-names-an-unwritable-rule.md`

```yaml
id: kb-open-question-es-6-unwritable-rule-001
title: ES-6 is frozen and names a rule that cannot be written
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ES-6 is FROZEN and names store_error_crosses_a_join_handle as its rule, marked (new) and
  rendered with † where the legend reads "does not exist yet". The identifier occurs as no fn
  anywhere in the workspace — only in prose and in comments, one of which states the rule is
  unwritable against today's port for every adapter, with SendStoreWithLocalError as the probe.
  spec-trace's check 4 deliberately skips clauses whose rule is (new) or †, and that escape hatch
  has no expiry, so a rule scheduled forever is indistinguishable from one scheduled for next
  week. ADR-0009 is accepted and makes the rule writable, so this may be a scheduling gap rather
  than a design gap — but the clause is frozen and names an unwritten rule. Settled by writing the
  rule against ADR-0009's marker, or by deciding that a † with no owning phase is a hard failure.
  Found independently the same day by references/evaluation/review-citation-drift.md §2.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-repair-frozen-clause-001
source_paths:
  - .kb/_intake/gaps-owed-a-decision.md
  - spec/SPECIFICATION.md
  - xtask/src/spec_trace.rs
  - crates/happenstance-cloudflare/src/lib.rs
  - crates/happenstance-cloudflare/src/send_shape.rs
  - references/evaluation/review-citation-drift.md
last_reviewed: 2026-08-10
```

ADR-0009 is named as accepted **in `.kb/decisions/`, not in `.kb/decisions/`**. No decision atom is
minted from it here and none is linked, because importing the ADR corpus is a separate wave; the
body names it by id so the link can be wired when it lands.

### Op 11 — `.kb/open-questions/es-7-and-vt-9-provisional-markers.md`

```yaml
id: kb-open-question-provisional-falsifiers-001
title: Two provisional markers whose falsifiers can no longer falsify
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ES-7 and VT-9 are PROVISIONAL and each names a falsifier that no longer discriminates. ES-7's
  falsifier is error[E0119] on a downstream direct impl, and its own named instrument —
  LocalMemoryEventStore in happenstance-testkit — compiles natively and on wasm32 with no such
  error. VT-9's falsifier is a target that cannot supply a wall clock at append time, and one is
  already in the build: on wasm32-unknown-unknown there is no clock and every event is stamped
  from_millis(0), yet VT-9's MUST is satisfied there because its rules assert presence and
  stability of a recorded time and never recency. Moving a maturity marker is an ADR's act, so
  both are recorded rather than moved. The transferable observation, worth keeping however these
  are settled: a falsifier that has already occurred without changing anything is a marker that
  has quietly become decoration, and spec-trace cannot detect it — it sees that a marker exists,
  not whether its condition has been met.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-repair-frozen-clause-001
source_paths:
  - .kb/_intake/gaps-owed-a-decision.md
  - spec/SPECIFICATION.md
  - crates/happenstance-testkit/tests/local_conformance.rs
  - crates/happenstance-core/src/memory.rs
last_reviewed: 2026-08-10
```

**One atom for 6a and 6b, at a score of 74.** The intake permits the split and asks that the
shared falsifier-hygiene observation be kept on both halves if it happens; one atom keeps it in
one place. Two clauses, two entanglements (ADR-0001's lift condition; the Workers schedule), but
one question shape and one settling move. If a later wave splits it, the observation is what must
be duplicated, and that is recorded here.

---

## Op 12 — nothing owns the reconciliation

| | |
| --- | --- |
| **op** | `defer_open_question` |
| **kind** | `open_question` |
| **destPath** | `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md` |
| **sourceFiles** | `open-question-nothing-owns-the-post-phase-reconciliation.md` |
| **classification** | `requires-new-decision` |
| **mapsImpact** | domainMap + openQuestionIndex |

```yaml
id: kb-open-question-post-phase-reconciliation-001
title: Nothing owns the specification reconciliation at a phase's exit
kind: open_question
status: accepted
authority_tier: note
summary: >-
  Phases 4 and 5 were the two largest changes to the contract in the plan, and neither carried an
  item obliging anyone to read spec/SPECIFICATION.md back against the tree it had just changed;
  the cost was measured in August 2026 by an unscheduled reconciliation pass. The rule that would
  have caught it was already written in RUNBOOK.md three times and is implemented nowhere. The
  pass added a standing exit criterion, and it is attached to no phase and to no tool: what is
  open is whether it becomes a per-phase checkbox, a gate step, or both; who computes a phase's
  clause range against the union of its ADRs' ranges when neither is machine-readable; and what a
  disagreement between the two numbers obliges. Forced by phase 6's exit, which freezes
  ProjectionStore and discharges PS-1 through PS-37, and secondarily by first publish at phase 12.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-ratchet-gate-landing-001
  - kb-open-question-ps-1-no-progress-obligation-001
  - kb-open-question-ps-19-scope-narrower-001
  - kb-open-question-provisional-falsifiers-001
source_paths:
  - .kb/_intake/open-question-nothing-owns-the-post-phase-reconciliation.md
  - RUNBOOK.md
  - spec/SPECIFICATION.md
  - xtask/src/spec_trace.rs
  - references/evaluation/phase-4-5-reconciliation.md
last_reviewed: 2026-08-10
```

**Rationale.** Ordered last because it is the only atom that links to four others. All eight
claims in the file are one question with one forcing event; none stands alone. The figures in
`claim-2` are cited from Op 1 rather than restated — this atom is a question, and a question that
carries its own copy of a census is a second census.

Sub-question 2 (where a gate-step baseline lives) is bound by Op 4's never-write-a-count rule,
which is why the link is `related` and not decorative.

---

## Adjudications

**No decision atom was authored, and that is the finding, not an omission.** Thirteen claims are
labelled `requires-new-decision`, and every one of them names an ADR as its settling move. This
wave writes none, for the three reasons the intake corpus itself argues (`lesson-repairing-a-frozen-clause`,
`C3`): a pass that both discovers and decides cannot be audited; a decision taken to unblock a
checker — or to close an ingest — is taken for the wrong reason; and ADRs here have a described
process with a human sign-off that an ingest cannot supply. The `.kb/decisions/README.md`
requirements agree: an `adr_id`, and the alternatives that lost. Deferring is not the weak
option; it is what the open-questions layer exists for.

**`.kb/decisions/` stays empty this wave.** The real ADR corpus is `.kb/decisions/` and is unmirrored.
Gaps 5 and 11 name ADR-0009 and ADR-0001 as accepted; they are referenced by id in bodies and
linked to nothing, so no dangling id enters the corpus and no ADR is half-imported.

**The ratchet's "must land together" (`C4`) is placed as a playbook.** Reasoned in `01`; carried
in `unresolved` so the promotion, if it happens, happens deliberately.

**Placement advice in the intake is superseded by fact where the corpus moved.** Four files say
"KB root, or a `playbooks`/`reference` layer if one is created". Both layers exist and are
scaffolded with READMEs, so nothing lands at the root.

**Nothing was routed to `product/` or `design/`.** Every intake file says so in its own words,
and they are right: an event-sourcing library has no personas and no interaction surface. Forcing
a fit would give a stock answer the standing of a finding.

## What the Maps phase inherits

- **No `map` atom and no `maps/` directory exist.** Two intake files ask for a bullet on
  "whichever `map` atom indexes the specification/governance area once one exists", and the
  open-questions README requires one for every question filed. Twelve atoms now need indexing and
  seven of them are open questions. The Maps phase must create the domain map and the
  open-question index; this wave deliberately did not invent them, because a map authored by an
  adjudicator mid-wave is a map authored differently every wave. Carried in `unresolved`.
- **Reciprocal backlinks.** Every link in this plan is outbound to an earlier operation. The
  inbound halves — Op 1 → Ops 6–12, Op 5 → Ops 8, 9, 11, Op 4 → Ops 6, 7, 12, Op 2 → Op 3 — are
  the Maps phase's to wire.
- **`decisionMap`: no impact.** Zero decision atoms created, amended or superseded.
