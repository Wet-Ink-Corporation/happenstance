---
item: "HS-S0002"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — ADR-0017, ADR-0018 and ADR-0019 accepted before the port changes

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Note on `verifying_test`: this story changes nothing executable, so no `#[test]` can observe it. Per
`_decomposition.md`'s Testing brief (AC-008 row) the instrument is **Static** — "a process gate on
commit sequence as much as a schema check" — so each row names the real command plus the real path
it reads. Inventing a test file to fill this column would be exactly the decorative rule `CLAUDE.md`
forbids.

```yaml
- id: AC-001
  criterion: >-
    GIVEN an adapter author whose store cannot hand out a batch that outlives the transaction, WHEN they ask *why does `type Batch` carry no lifetime, and was my shape considered?*, THEN `.kb/decisions/0017-*.md` answers with the argument that actually holds — `error[E0195]` plus the `DefId::expect_local` ICE proving today's port is implementable only by stores that outlive every batch (`references/adapter-shapes.md:186-194,307-365`) — **not** the `Send` argument, which `crates/happenstance-ladybug/src/live_handle.rs:14-31` already refutes; it records that the owned batch does **not** close the foreign-batch hazard (PS-15 stays `[PROVISIONAL]`, discharged at run time by `CommitError::ForeignBatch`), that PS-9's *no universal write vocabulary* and PS-11's `ProjectionProbe`-in-the-contract-crate are one split-by-consumer decision rather than "the port grows a write method", and it names `LiveHandleProjectionStore`'s disposition explicitly — deleted, moved to `experiments/`, or kept with its transcripts and a note that the port no longer admits it.
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "redkiln validate --kb (green over .kb/decisions/0017-*.md) + reviewed-artefact check of references/adr/0017-*.md against references/adapter-shapes.md:186-194,307-365 and crates/happenstance-ladybug/src/live_handle.rs:14-31"

- id: AC-002
  criterion: >-
    GIVEN an operator with a runbook who wants to rebuild one projection, and an adapter author who must decide what `reset` costs them, WHEN they read `.kb/decisions/0018-*.md`, THEN it separates four halves of different strength rather than levelling them: `reset(batch, id)` as one unit of work carrying the two-statements-on-two-connections failure it models (PS-16, provisional); scope at `(store, ProjectionId)` **cited as ADR-0007's decision applied to removal, not re-derived** (PS-17, frozen); refusal as a port *mechanism* with the policy in the domain, naming "refusal is purely a typed-layer concern" as what lost because a policy with no port-level mechanism is bypassed by anyone holding the store; and `Checkpoint`'s three variants over `(Option<SequencePosition>, bool)`, which can spell `(None, true)`. It states PS-19's pairing defect as **known, scoped out, and whose** — and repairs nothing.
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "redkiln validate --kb + reviewed-artefact check that .kb/decisions/0018-*.md cites .kb/decisions/0007-projection-runner-decodes.md for PS-17 and defers .kb/open-questions/ps-19-scope-narrower-than-its-rule.md; `git diff spec/SPECIFICATION.md` empty"

- id: AC-003
  criterion: >-
    GIVEN an adapter author who fears the port will grow a failure-handling surface they must implement, WHEN they read `.kb/decisions/0019-*.md`, THEN the decision is stated as the decision it is — **the port grows nothing for apply failure** — showing that the skip primitive already exists (`begin()` then `commit(batch, id, poison_position, Live)` applies nothing and advances the checkpoint atomically), that `rollback` must survive the port change because PS-30's `AssertUnwindSafe` promise rests on it, and that policy is per projection not per runner; and it **defers by name** `PumpError`'s three type parameters, the `SkipAndRecord` vocabulary and the supervisor's observability half to HS-P0011 / ADR-0020-0021 instead of designing another project's error type.
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "redkiln validate --kb + reviewed-artefact check of .kb/decisions/0019-*.md against spec/SPECIFICATION.md:5399-5478 and .bklg/from-contract-to-published-library/projection-store-freeze/project.md (Out of scope): no runner error-type design present, HS-P0011 named as owner"

- id: AC-004
  criterion: >-
    GIVEN a reader two years from now asking *what did the compiler actually say*, WHEN they open any of the three records, THEN each quotes `references/adapter-shapes.md` by `file:line` rather than paraphrasing — and where no transcript exists, ADR-0018 and ADR-0019 quote the **stated absence** (§5, *What a skeleton does not prove*, `references/adapter-shapes.md:286-303`: "Nothing. Every body is `todo!()`") instead of a fabricated diagnostic, because a table showing only `error[E….]` would rank the least compatible adapter in the workspace as the most compatible (`:5-36`).
  satisfied: true
  evidence: "All three long-form records exist and each quotes `references/adapter-shapes.md` by line rather than paraphrasing: `references/adr/0017-what-a-projection-batch-owns.md` carries six such citations (`:7-8`, `:61-102`, `:97-102`, `:160-167`, `:169-179`, `:186-194`, `:307-365`, `:346-350`); `references/adr/0018-returning-a-projection-to-never-run.md` and `references/adr/0019-what-happens-when-apply-fails.md` carry two each. ADR-0018 and ADR-0019 quote the STATED ABSENCE from section 5, 'What a skeleton does not prove' (`references/adapter-shapes.md:286-303`: 'Durability | Nothing. Every body is `todo!()` | Phase 8'), because no skeleton exercised reset or an apply failure, and each says in its own text why a fabricated diagnostic would be a defect rather than a stronger record, citing `:29-33`. Zero diagnostics appear in any record that are absent from the referent: every quoted range was re-read against the working tree and eight citations were re-anchored to their subject text before commit (the referent, not the address). Static check red then green: against the baseline commit b5b48a8 the three records did not exist and both stated-absence assertions failed; against this tree all pass."
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "reviewed-artefact check over references/adr/0017-*.md, 0018-*.md, 0019-*.md: every references/adapter-shapes.md:<range> citation verified against its referent per .kb/playbooks/anchoring-citations-in-a-long-lived-document.md; zero diagnostics absent from the referent"

- id: AC-005
  criterion: >-
    GIVEN P2 discovering late that the port assumed something their storage cannot provide, WHEN they look for whether their shape was ever on the table, THEN each of the three atoms names the alternatives that lost and why — the `ProjectionBatch` supertrait with `put`/`get` (obliges every store into a key-value table and reintroduces at the read-model layer the opaque blob ADR-0003 confined to payloads), a testkit-side probe trait (rejected by the orphan rule from an adapter's own `tests/`, forcing a non-dev dependency on `happenstance-testkit`), the two-statement reset, refusal-as-typed-layer-only, and the `(Option<SequencePosition>, bool)` checkpoint — so the answer to "was my shape considered" is a citation, not an archaeology exercise.
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "reviewed-artefact check that each of .kb/decisions/0017-*.md, 0018-*.md, 0019-*.md carries a non-empty rejected-alternatives section with a reason per entry, matched against .kb/decisions/README.md and the shape model .kb/decisions/0010-the-suite-must-prove-itself.md"

- id: AC-006
  criterion: >-
    GIVEN the three runbook questions are each a conjunction (`RUNBOOK.md:296-298`), and the upstream sweep has just reported whether the PS-1/PS-19 pairing defect is systematic or isolated, WHEN the records are written, THEN no title carries a strong decision and a weak one undifferentiated: a half settled *by being made* stands, a half that is a claim about code that does not exist yet is marked provisional **with the observation that would refute it and the phase that would produce it**, or the document is split — and each ADR states whether any clause in its own range carries the pairing shape and defers the repair, reporting a "systematic" verdict at the story boundary rather than widening ten frozen clauses under cover of a rule-writing story.
  satisfied: true
  evidence: "Each record grades its title's halves before deciding them, against `.kb/playbooks/one-decision-per-adr-title.md`. `references/adr/0017-what-a-projection-batch-owns.md` opens with a three-row strength table (owns = settled by being made; vocabulary = provisional with falsifier and phase; dropped = settled by being made) and says why the middle half is marked rather than split. `references/adr/0018-returning-a-projection-to-never-run.md` opens with a four-row table across three strengths and states that levelling them is the playbook's recorded failure; its PS-17 half is cited to `.kb/decisions/0007-projection-runner-decodes.md` rather than re-derived. `references/adr/0019-what-happens-when-apply-fails.md` states its Out of scope paragraph first and defers `PumpError`, the `SkipAndRecord` vocabulary and the supervisor's observability half to HS-P0011 by name. Every provisional half names a falsifier and a phase (PS-4/PS-5, PS-9/PS-11, PS-15, PS-16, PS-18, PS-27, PS-30). Each record consumes the upstream sweep in its scope section: ADR-0017 names PS-8 and PS-13, ADR-0018 names PS-19, ADR-0019 names PS-29 and PS-28 \u2014 each stated as known, scoped out and attributed, with the repair routed to `unstable-projection-gate-and-clause-disposition` under the frozen-clause playbook, and none repaired. The sweep's verdict is ISOLATED, so no record's scope is widened and no re-plan is raised; that is recorded in each record and in the implementation report. Static check red then green: against baseline b5b48a8 every assertion failed (no records); against this tree all pass."
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "reviewed-artefact check against .kb/playbooks/one-decision-per-adr-title.md and .kb/playbooks/repairing-a-frozen-clause-without-amending-it.md: every provisional half names a falsifier and a phase; the finding from .bklg/from-contract-to-published-library/projection-store-freeze/ps-clause-pairing-sweep/ is quoted in each record's scope sentence"

- id: AC-007
  criterion: >-
    GIVEN that hand-written atoms produce the directory layout of the process without the process — which is why the first attempt was reverted at `0269720` — WHEN these three decisions enter `.kb/`, THEN they arrive as the output of one human-invoked `/redkiln:kb-ingest` wave fed from `.kb/_intake/`, with the long-form records left in `references/adr/` (the atom is the ~100-line canonical form; the record holds the transcripts and rejected alternatives a summary cannot), the intake README dropped at the approval gate, a wave id that does not collide with `2026-08-10-intake` / `-2` under `.kb/_governance/integration-waves/`, `.kb/_intake/` empty afterwards, and frontmatter that invents no keys — `status: accepted` with the provisional qualification **and its falsifier** in the first clause of `summary`, `supersedes`/`superseded_by` reserved for full supersession.
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "redkiln validate --kb && redkiln doctor && redkiln validate (all green); `ls .kb/_intake/` clear of this wave's documents; a non-colliding directory present under .kb/_governance/integration-waves/; .kb/open-questions/adr-status-vocabulary-exceeds-the-schema.md unchanged"

- id: AC-008
  criterion: >-
    GIVEN a reader who needs to know *what was not known on the day the choice was made*, WHEN ADR-0017 answers `kb-open-question-projection-batch-no-apply-001`, THEN the question is **resolved, not deleted**: a new atom is the answer, the question's `status` moves to `withdrawn` or `superseded`, the two are linked through `related`, **its body is left verbatim**, sub-question 3 (ADR-0006's encoding-versus-orchestration discriminator) stays open with HS-P0011 — and the six questions this story deliberately does not answer are each named with their owner, so the omission reads as a decision rather than an oversight.
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "`git diff --diff-filter=D -- .kb/open-questions/` empty; `git diff .kb/open-questions/projection-store-batch-has-no-apply-seam.md` touches frontmatter only; the six-owner table of spec Context pack §7 reproduced in the implementation report with those six files untouched"

- id: AC-009
  criterion: >-
    GIVEN an adapter author who has never read this backlog and arrives at the knowledge base cold, WHEN they open `.kb/maps/decision-map.md`, THEN ADR-0017, ADR-0018 and ADR-0019 are each a row there with status, phase 6 and an empty supersession column, and `.kb/maps/open-questions-index.md` still lists the resolved question — annotated with its answer rather than removed — because an atom absent from the map is exactly a `lib.rs` export block's missing `pub use`: it validates, and nobody can find it.
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "three new rows present in .kb/maps/decision-map.md; the resolved question's row still present and annotated in .kb/maps/open-questions-index.md; redkiln validate --kb link check resolves the reciprocal `related` edges in both directions"

- id: AC-010
  criterion: >-
    GIVEN project AC-008's whole point — that a decision accepted *after* the port changed proves nothing about which one was allowed to constrain the other — WHEN this story merges, THEN the three atoms are committed on a tree where `crates/happenstance-core/src/projection.rs` is byte-identical to `main`, `redkiln validate --kb` is green **on that tree** and not on a later one, and the PR contains no diff under `crates/**`, `spec/**` or `RUNBOOK.md`; `owned-batch-port-shape` (HS-S0003) is blocked on this ordering, which is the check.
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "`git diff --stat main...HEAD -- crates/ spec/ RUNBOOK.md` empty; redkiln validate --kb run at the atom commit with the sha recorded in the implementation report; cargo xtask ci --fast green as a non-regression check only"
```
