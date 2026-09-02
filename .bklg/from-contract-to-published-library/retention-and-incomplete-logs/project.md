---
id: HS-P0018
uid: a03e31
type: project
slug: retention-and-incomplete-logs
title: What a store may forget, and how a reader finds out
parent: HS-I0006
initiative: from-contract-to-published-library
project: retention-and-incomplete-logs
status: in-review
process: project
stage: design
automation: HITL
severity: null
blocked_by: []
blocks: []
terminal: false
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-12T12:57:19.207Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: 7289a0c4
---
# What a store may forget, and how a reader finds out

## One-line objective

Build the completeness axis's missing far end — a store holding only a suffix of
its own log — run a reader against it until something fails loudly, and land the
result as **ADR-0028**: either a written answer to what a store may forget and how
it says so, or an explicit reasoned refusal — without changing any surface
published at `0.2.0`.

## How this advances the initiative

This is the second of the charter's two silences (BR-11, AC-14, DoD 15, DT-7 in
[`../initiative.md`](../initiative.md)), and the only project that owns them.

The completeness axis has **nothing at either end**. `spec/SPECIFICATION.md:8096`
records it as the one axis in the portfolio table whose far end is *"No, and
nothing is planned"*, and §3.7 (`spec/SPECIFICATION.md:4263-4272`) states why that
is worse than it looks: a store that has been pruned passes every rule in the suite
unchanged, because `query_all_matches_every_event` is store-relative by wording and
therefore accidentally correct. **A holed log and a young log are the same value at
every seam the port exposes.** Four of the six deployment scenarios reach that
missing primitive from unrelated doors — a pruned device slice, a regulated
scattered purge, a compacted peer, an epoch count derived from a slice.

Three clauses are stalled on the same absent instrument. ES-38
(`spec/SPECIFICATION.md:4299-4323`) is `[FROZEN]` and names a rule,
`positions_are_not_reused_after_removal`, that **was examined at phase 4 and
deliberately not written** — the reason is recorded as an accepted open question at
[`.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`](../../../.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md),
which assigns it to this phase by name. ES-39 (`:4325-4349`) is `[DEFERRED]` on
choosing a primitive. ES-40 (`:4351-4379`) is `[PROVISIONAL]` with **this project's
instrument as its named falsifier**. CF-27 (`:8034-8060`) is the instrument itself,
`[DEFERRED]`, owned by *"the pass that settles retention and deletion"* — this one.

So the initiative's exit criterion 5 — replication identity and incomplete-log
semantics each with an accepted decision atom or an explicit reasoned refusal, and
the matching open-question atoms resolved rather than deleted — cannot close
without this project, and neither can DoD 15.

## In scope (this project)

- **The completeness instrument (CF-27).** A testkit-adjacent store that
  deliberately holds only a suffix of its own log, built as a decorator over any
  `EventStore` — the experiment CF-27 itself names — and run through the full
  conformance suite. The outcome that matters is the recorded *list of rules that
  pass*, because that list **is** the set of rules that cannot tell a pruned store
  from a young one.
- **ADR-0028**, from the runbook's ADR queue (`RUNBOOK.md:307`): one question —
  what is a store permitted to forget, and how does it say so? — answered, or the
  area refused in writing. Authored through `.kb/_intake/` and
  `/redkiln:kb-ingest`, never hand-written into `.kb/decisions/` (`CLAUDE.md`, and
  the revert at `0269720`).
- **The two rules that already have assertions and are waiting only on the
  instrument**: `positions_are_not_reused_after_removal` (ES-38) and
  `condition_over_removed_history_does_not_reject` (ES-40), each with a registered
  wrong implementation that fails it.
- **A reader failing loudly.** A projection runner and an ingest path each run
  against the instrument, with the outcome observed rather than asserted
  (`RUNBOOK.md:4658-4668`).
- **DT-7's resolution** in this project's `_design.md`: one undifferentiated
  incompleteness signal, or a distinction between transient, benign-permanent and
  meaningful-permanent.
- **The redaction question (E2E-49)** — whether a `Tag` can be redacted at all,
  given that `Tag` equality is byte equality and every index is keyed on it —
  answered or explicitly deferred against a named experiment. Never silent.
- **Marker movement and its gate.** ES-39 and CF-27 leave `[DEFERRED]`; ES-40's
  `[PROVISIONAL]` falsifier is discharged or renewed against a named experiment;
  `cargo xtask spec-trace` green on the result.
- **Consuming SY-32** (`spec/SPECIFICATION.md:6771-6799`) as answered by
  `replication-identity-and-ingest`, not re-deciding it.

## Out of scope (this project)

Each exclusion names the sibling that owns it, per
[`../_decomposition.md`](../_decomposition.md) *"Scope seams"*.

- **Replication identity, the merge rule, and what ingest promises** →
  `replication-identity-and-ingest` (HS-P0017). SY-32 arrives here answered.
- **Crypto-shredding, lawful-deletion tooling, or a retention policy engine** →
  not in the charter's scope at any grain. The research corpus
  (`_discovery/research/07-retention-deletion-and-incomplete-log-semantics.md`)
  is evidence for the *decision*, not a work list.
- **Amending anything `[FROZEN]`** — ES-37 (`EventStore` is closed over insertion,
  `spec/SPECIFICATION.md:4274-4297`) and ES-38 both are → a new decision atom and a
  re-plan, per the charter's *Out of scope*.
- **Any change to a surface published at `0.2.0`.** Version consequences belong to
  `publication-and-positioning` (HS-P0016), which shipped that release; a `0.3.0`
  is outside this initiative's exit criteria. See AC-011 and AC-012.
- **The adapter instrument at the completeness axis's far end** — the "device
  adapter second" of `spec/SPECIFICATION.md:8096`. CF-26 (`:8020-8032`) says a
  fixture instrument satisfies falsifiability and **not** implementability; this
  project discharges the first half only, and records the residual exposure rather
  than implying it is closed. Post-initiative.
- **`read_from_a_gap_position`'s ownership.** The sibling half of the open-question
  atom above — ES-9's owed rule, named by two accepted ADRs and owned by neither.
  It sits in the same neighbourhood and is *not* adopted by default; adopting it is
  a deliberate call in this project's architecture brief, and if it is not taken,
  that half of the atom stays open on purpose.
- **Publishing `happenstance-sync` or `happenstance-sync-testkit`** → out of this
  release train per the charter.
- **Re-observing DoD 15 as part of the assembled set, and auditing that this
  project's decision atom names the alternatives that lost** →
  `closeout-and-durable-audience` (HS-P0019).
- **Incidental defects found in passing** → the `support` initiative
  (`.redkiln/config.yaml:5`).

## Derived requirements

Expanded from BR-11, BR-15, AC-14, DoD 15 and DT-7 in
[`../initiative.md`](../initiative.md), and from the intake brief's *Clauses*
section.

- **DR-1 — The instrument exists and is an instrument.** A store holding only a
  suffix of its own log, as a decorator over any `EventStore`, testkit-adjacent and
  incapable of becoming a published adapter. `CLAUDE.md`'s skeleton discipline
  applies in its strongest form: *an instrument first and a target never*.
- **DR-2 — The suite is run against it and the result is recorded as data.** Not
  "it passed" — the enumerated list of rules that pass, which is CF-27's stated
  experimental outcome and the evidence base for ADR-0028.
- **DR-3 — At least two rules fail against it**, by name (`RUNBOOK.md:4667`). A
  store that lies about its own completeness is the one wrong implementation
  nothing in the workspace can currently detect; if nothing fails, the instrument
  has not earned its keep.
- **DR-4 — ES-38's owed rule is written.** `positions_are_not_reused_after_removal`,
  registered in `for_each_event_store_rule!` — the single rule registry
  (`crates/happenstance-testkit/src/lib.rs:84-89`) — with a mutant that fails it:
  an adapter that renumbers on compaction, which ES-38's own `Rejects:` names.
- **DR-5 — ES-40's owed rule is written**, and it asserts the **vacuous pass** as
  specified behaviour rather than as a desired one, so an adapter author reads it
  as the contract and an ingest author reads it as a hazard.
- **DR-6 — A reader fails loudly.** A projection runner and an ingest path are each
  run against the instrument and each produce a named, non-silent outcome rather
  than a confidently wrong answer. This is where the failure mode the charter
  names — *a reader that quietly builds a wrong answer from a truncated log* —
  either becomes detectable or is written down as undetectable.
- **DR-7 — DT-7 is resolved with its loser named.** One signal, or three; the
  production evidence for the three-way distinction is in the research corpus and
  the cost of it is reader complexity.
- **DR-8 — ADR-0028 is one question.** An ADR that cannot be stated as one question
  is two ADRs (`RUNBOOK.md:264-268`). It must name the alternatives that lost,
  including `earliest_position()` and **why a floor is the wrong shape**: a
  regulated purge is scattered, not a prefix, and the floor ships looking correct
  until a claim runs long (ES-39).
- **DR-9 — The refusal, if that is the answer, still owes the illustration.**
  *"Deletion is out of scope for `EventStore`, and here is what a deleted-from
  store looks like to a reader"* — the instrument is the second half of that
  sentence and is not optional under the refusal branch.
- **DR-10 — The open-question atom is resolved, not deleted.** Sub-question 3 of
  `es-38-and-gap-read-rules-are-unowned.md` — *when CF-27 lands, does the rule get
  written against whatever removal-capable fixture arrives with it, or does this
  phase need its own instrument decision first?* — is answered on the record.
- **DR-11 — No published-surface change, and the constraint is checked rather than
  intended.** Gate decision 4 in [`../_decomposition.md`](../_decomposition.md).
- **DR-12 — A conclusion that a surface change is required is a finding, not a
  licence.** It escalates to the initiative with the surface, the version
  consequence and the option set stated.

## Acceptance criteria

Project-grain and testable. This is the spine the story map must cover.

- **AC-001 — The completeness instrument exists and is reachable by the suite.** A
  store holding only a suffix of its own log, implemented as a decorator over any
  `EventStore`, lives testkit-adjacent (the pattern
  `crates/happenstance-testkit/tests/` already uses for `LocalMemoryEventStore`,
  `CachedHeadFixture` and the mutant registry) and can be handed to
  `event_store_conformance!` through a `Fixture`
  (`crates/happenstance-testkit/src/contract.rs`).
- **AC-002 — CF-27's experiment has been run and its output recorded.** The full
  suite is run against the instrument and the **enumerated list of rules that
  pass** is written into this project's artifacts, stated as what it is: the list
  of rules that cannot tell a pruned store from a young one.
- **AC-003 — At least two conformance rules fail against the instrument, by name**,
  and the failure is reported as a rule name rather than as "conformance failed"
  (`RUNBOOK.md:4667`).
- **AC-004 — ES-38's rule is written, registered and non-decorative.**
  `positions_are_not_reused_after_removal` appears in `for_each_event_store_rule!`,
  is green against `MemoryFixture`, and a registered wrong implementation — a store
  that renumbers on compaction — fails **exactly** it, with
  `mutants_fail_exactly_their_declared_rules` green in both directions.
- **AC-005 — ES-40's rule is written and asserts the specified outcome.**
  `condition_over_removed_history_does_not_reject` is green against the instrument,
  asserting that a condition over destroyed history passes vacuously, with a
  registered wrong implementation that rejects (or errors) instead.
- **AC-006 — A reader fails loudly, observed.** A projection runner and an ingest
  path are each run against the instrument, and each terminates with a named,
  non-silent outcome. Where either cannot be made to fail loudly without a surface
  the port does not have, that fact is recorded against AC-011 rather than papered
  over.
- **AC-007 — CF-27's own rule is settled either way.**
  `suffix_store_is_distinguishable_from_a_young_store` either exists with its
  assertion fixed by the retention decision, or ADR-0028's refusal records in
  writing why it cannot exist and what a reader is therefore on its own against.
- **AC-008 — ADR-0028 is accepted on disk and answers one question.** A decision
  atom under `.kb/decisions/` with valid `KbFrontmatter`, authored via `.kb/_intake/`
  and `/redkiln:kb-ingest`, naming the alternatives that lost — including
  `earliest_position()` and the scattered-purge argument against a floor — and, if
  it refuses, refusing in terms rather than by omission.
- **AC-009 — DT-7 has a recorded resolution** in this project's `_design.md`: one
  undifferentiated incompleteness signal, or the transient / benign-permanent /
  meaningful-permanent distinction, with the option that lost and the reason.
- **AC-010 — The redaction question has an answer.** Whether a `Tag` can be
  redacted at all, given byte equality and index keying, is answered in ADR-0028 or
  explicitly deferred against a **named** experiment. A deferral with no experiment
  is a gate failure under CF-38.
- **AC-011 — No breaking change to any surface published at `0.2.0`.** A
  public-surface comparison against the `0.2.0` registry baseline reports no
  breaking change to `happenstance-core`, `happenstance` or `happenstance-testkit`;
  anything additive (a new `pub` rule function, a defaulted associated const)
  ships as `0.2.x`. No `0.3.0` is implied or required.
- **AC-012 — If the honest answer needs a surface change, it is escalated, not
  made.** The finding names the surface, the version consequence, and the options,
  and is raised to the initiative. This is the acceptance criterion that makes
  AC-011 a decision rather than a suppression.
- **AC-013 — The instrument cannot become a published adapter.** It declares no
  crate of its own, or if it does, that crate carries `publish = false`; nothing in
  the three publishable crates' public API exposes it; and it is documented as an
  instrument that is never a target.
- **AC-014 — Markers move and the gate proves it.** ES-39 and CF-27 no longer carry
  `[DEFERRED]`; ES-40's `[PROVISIONAL]` marker is discharged or renewed against a
  named experiment; `cargo xtask spec-trace` is green on the resulting
  `spec/SPECIFICATION.md`.
- **AC-015 — The open-question atom is resolved rather than deleted**, its
  sub-question 3 answered, its `status` reflecting the resolution, and
  `redkiln validate --kb` clean.
- **AC-016 — SY-32 is consumed, not re-decided.** The retention-gap answer inherited
  from `replication-identity-and-ingest` is cited by id; where this project's answer
  changes what a peer must report, the change is recorded against SY-32 rather than
  by reopening ADR-0026 or ADR-0027.

## Definition of done (boundary-level)

- ADR-0028 exists as an accepted `.kb/decisions/` atom, reached through the ingest
  path, and the matching open-question atom is resolved rather than deleted.
  `redkiln validate --kb` is clean. *(AC-008, AC-015)*
- The suffix store is committed, the suite has been run against it, the pass list is
  recorded, and at least two named rules fail against it. *(AC-001 – AC-003)*
- A projection runner and an ingest path have each been run against it and observed
  — not asserted in prose — to fail loudly, or the inability to do so is written
  down. *(AC-006)*
- ES-38's and ES-40's rules are written, registered, and each has a registered wrong
  implementation that fails exactly it. *(AC-004, AC-005)*
- `cargo xtask spec-trace` is green with ES-39 and CF-27 no longer `[DEFERRED]`.
  *(AC-014)*
- `cargo xtask ci --fast` is green — the bar `.redkiln/config.yaml:55` sets for a
  non-terminal project. The whole gate on the assembled tree is
  `closeout-and-durable-audience`'s.
- The public-surface comparison against the `0.2.0` baseline reports no breaking
  change, and any additive item is recorded. *(AC-011)*
- DT-7 and the redaction question each have a recorded resolution. *(AC-009,
  AC-010)*

## Dependencies

From the DAG in [`../_decomposition.md`](../_decomposition.md) *"Sequencing"* —
this project is at rank 5 on the serial trunk. The item's `blocked_by` /`blocks`
frontmatter is the CLI's to write; this section is the plan of record for what the
edges mean.

**Depends on — `replication-identity-and-ingest` (HS-P0017).** Three things arrive
from it and none is re-derived here: SY-32's answer (a peer reporting the floor
below which it no longer retains history, and a runner detecting a peer offline
longer than another's retention window); the `IngestStore` seam this project runs a
reader through (`crates/happenstance-sync/src/ingest.rs`); and the settled position
on whether ingest re-checks a writer's asserted conditions, which is what makes
ES-40's *"an ingest path that re-evaluates an origin condition against a pruned
slice"* the **normal** path rather than a hazard. Transitively this also depends on
`publication-and-positioning` (HS-P0016), because the `0.2.0` baseline AC-011 diffs
against does not exist until it ships.

**Unlocks — `closeout-and-durable-audience` (HS-P0019).** DoD 15 is re-observed
there as part of the assembled set, and BR-15's audit — that this project's decision
atom exists, names the alternatives that lost, and left its open question resolved
rather than deleted — is that project's.

**Runbook provenance.** Phase 13 → 14 (`RUNBOOK.md:165`, `:4626-4674`). This is the
last phase in the plan of record and the last empty far end in the portfolio.

## Risks and coupling notes

- **The no-surface-change constraint may collide with "fails loudly", and that
  collision is the highest-value thing to discover early.** The intake brief names
  it as the open question most likely to hit the constraint. Whether loud failure
  is expressible in the existing error vocabulary is a question to answer in the
  architecture brief, before the instrument is built — not after a runner has been
  written against a signal that does not exist.
  *Mitigating fact worth carrying:* ES-40 is a **documentation** clause — *"the
  port's documentation MUST state…"* — so falsifying or amending it is a doc change,
  not a surface change. The collision, if it comes, comes from ES-39's primitive.
- **`happenstance-testkit` is one of the three publishable crates.** "Testkit-adjacent"
  is therefore not automatically off-surface. `tests/` is not public API and a new
  `pub` rule function is additive, but the trait shape matters: a new **required**
  method on `Fixture` is breaking, while a **defaulted** associated const is not —
  which is precisely why CF-39 was defaulted, and why `contract.rs` separates
  capabilities (trades, which owe a reason) from limits (facts, which do not). Any
  fixture-side seam this project needs must take the defaulted shape.
- **CF-25 / CF-26 exposure is not discharged here, and must not be reported as if it
  were.** A fixture instrument satisfies falsifiability; implementability needs an
  adapter at the far end, and the portfolio table asks for *"a device adapter
  second"*. That second half is out of scope, so the residual exposure on the
  completeness axis is recorded rather than implied closed.
- **Schedule position is itself the risk.** Rank 5, after publication, at the end of
  the initiative — which is exactly where "refuse it" stops being a decision and
  becomes the default. The counterweight is DR-9: the refusal branch still owes the
  instrument and the illustration, so it is not cheaper than the decision branch by
  much, and choosing it for cost rather than for reason will show.
- **The instrument is one commit away from looking like an adapter.** A decorator
  over any `EventStore` that passes most of the suite reads as a working store.
  AC-013 exists because `CLAUDE.md`'s six skeletons already demonstrate how easily
  an instrument is mistaken for a target.
- **A rule written against the instrument alone may be unfalsifiable in practice.**
  `CLAUDE.md`'s corollary — *a rule that no adapter can fail is decorative* — binds
  both new rules; each owes a named wrong implementation in the testkit's own
  `tests/`, which is what AC-004 and AC-005 spell out.
- **Adjacent-question drift.** The gap-read neighbourhood (ES-9,
  `read_from_a_gap_position`) is one clause away and owned by nobody. Settling it in
  passing would be exactly the defect the charter's non-goals name; it is listed
  under *Out of scope* so that adopting it has to be a decision.

## Context anchors

Backlog:

- [`../initiative.md`](../initiative.md) — BR-11, AC-14, DoD 15, DT-7, exit
  criterion 5
- [`../_decomposition.md`](../_decomposition.md) — the DAG, the scope seams, and
  gate decision 4 (the no-surface-change constraint)
- [`_intake-brief.md`](_intake-brief.md) — the approved problem, constraints and
  clause ledger
- `_discovery/research/07-retention-deletion-and-incomplete-log-semantics.md` —
  three distinct mechanisms, no shared vocabulary; the evidence behind DT-7

Specification and plan:

- `spec/SPECIFICATION.md:4263-4272` — §3.7, why a holed log and a young log are the
  same value
- `spec/SPECIFICATION.md:4274-4297` — ES-37, `[FROZEN]`: `EventStore` is closed over
  insertion
- `spec/SPECIFICATION.md:4299-4323` — ES-38 and its unwritten rule, **Owner: phase 14**
- `spec/SPECIFICATION.md:4325-4349` — ES-39, and why `earliest_position()` is the
  wrong shape
- `spec/SPECIFICATION.md:4351-4379` — ES-40, the vacuous pass, and the ingest hazard
- `spec/SPECIFICATION.md:6771-6799` — SY-32, retention across a peer set and refusal
- `spec/SPECIFICATION.md:8003-8032` — CF-25 / CF-26, fixture versus adapter instrument
- `spec/SPECIFICATION.md:8034-8060` — CF-27, the completeness instrument and its
  experiment
- `spec/SPECIFICATION.md:8090-8098` — the portfolio table; the completeness row is
  the empty one
- `spec/E2E-CASES.md` — E2E-44, E2E-46, E2E-47, E2E-48, E2E-49, E2E-56
- `RUNBOOK.md:165` — phase 14's status row and proof artefact
- `RUNBOOK.md:307` — ADR-0028's row in the ADR queue
- `RUNBOOK.md:4626-4674` — phase 14 in full: work, proof artefact, exit criteria

Knowledge base:

- [`../../../.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`](../../../.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md)
  — the atom this project resolves, and the sibling half it must not settle in
  passing
- [`../../../.kb/concepts/torn-reads-and-the-append-condition-boundary.md`](../../../.kb/concepts/torn-reads-and-the-append-condition-boundary.md)
  — why a condition derived from a read cannot catch what the read did not see; the
  same mechanism, one cause along
- `.kb/maps/open-questions-index.md`, `.kb/maps/decision-map.md`

Code:

- `crates/happenstance-testkit/src/contract.rs` — `Fixture`, `Capability`,
  `RuleOutcome`; capabilities are trades and limits are facts (`:44-54`)
- `crates/happenstance-testkit/src/lib.rs:84-89` — `for_each_event_store_rule!` is
  the one place the rule set lives
- `crates/happenstance-testkit/src/fixtures` — `MemoryFixture`, the reference
  implementation
- `crates/happenstance-core/src/store.rs` — `read` and `append`: two methods, and
  neither deletes
- `crates/happenstance-core/src/append.rs` — `is_violated_by`, a pure predicate over
  events that still exist, with no third outcome
- `crates/happenstance-sync/src/ingest.rs` — the `IngestStore` seam a foreign
  identity arrives through
- `CLAUDE.md` — the rule that matters, the two corollaries, and the instrument /
  target discipline
- `.redkiln/config.yaml:55` — `cargo xtask ci --fast`, the non-terminal project bar

## Companions

Board-invisible drill-down for this card:

- [`_decomposition.md`](_decomposition.md) — this project's warranted briefs
  (architecture, testing), authored by `plan-briefs`
- [`_grounding.md`](_grounding.md) — the grounding pass behind those briefs
- [`_storymap.md`](_storymap.md) — the vertical-slice story map covering AC-001 –
  AC-016
- [`_intake-brief.md`](_intake-brief.md) — the approved intake, its constraints and
  its clause ledger
- [`../initiative.md`](../initiative.md) — the parent charter
- [`../_plan.md`](../_plan.md) — the initiative-wide plan rollup
