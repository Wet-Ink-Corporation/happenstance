---
item: HS-S0118
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — Two readers meet the hole: read_decision_model and IngestStore::holds

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Four notes specific to this story, so a row is not flipped on the wrong evidence:

- **`crates/happenstance-testkit/tests/completeness_instrument.rs` is HS-S0114's file, not this
  story's.** This story extends it with a `readers` module. Every `mount_point` below names that
  target. A row whose evidence cites anything under `crates/*/src/**` is citing the wrong thing —
  this story changes no production code, which is itself AC-003.
- **"Control" means the retained-set configuration that hides nothing; "hole" means one that hides
  matching history.** A row is evidence only from a **pair**: an admitted append with no run that
  rejected is not evidence of forgetting, it is evidence that a condition matched nothing (EC-002).
- **AC-005 has two authorised branches** — *reached* (the production `IngestStore::holds` was called
  in a new `crates/happenstance-sync/tests/` target) or *recorded* (the finding citing
  `crates/happenstance-sync/src/ingest.rs:223-225` and naming HS-P0017). The evidence must state
  which branch was taken and why. A hand-written `IngestStore` impl, a stubbed body, or a replaced
  `todo!()` is a third outcome and may never satisfy this row.
- **The values are the evidence.** Every row that names `_observations.md` is satisfied only when the
  cited cells hold values the code returned. A cell describing a value, or a `#[should_panic]`
  standing in for one, fails the row (CF-2, `spec/SPECIFICATION.md:7192-7196`).

```yaml
- id: AC-001
  criterion: |-
    GIVEN the application author, whose stated fear is a second independent reader building a wrong answer from a gapped log (`initiative.md:203-208`), WHEN they look for whether that fear is real in *this* library rather than in general, THEN the DCB command loop exactly as shipped — `read_decision_model` → `AppendCondition::after_opt` → `EventStore::append` — has been run **twice** inside `crates/happenstance-testkit/tests/completeness_instrument.rs` over the same fixture type, the same seeded events, the same `Query` and the same call sequence, differing **only** in the retained set; the forgotten events are ones that `Query` **matches**; the control run **rejects** and the hole run **admits**; and no locally inlined re-implementation of `read_decision_model` or of the guard exists anywhere in the target, so what was observed is the library's code and not this story's.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/completeness_instrument.rs — the `readers` module (inline or completeness_instrument/readers.rs), driving happenstance_core::read_decision_model (crates/happenstance-core/src/store.rs:321-331) → AppendCondition::after_opt (crates/happenstance-core/src/append.rs:201) → EventStore::append (crates/happenstance-core/src/store.rs:213-217) over HS-S0114's instrument"
  verifying_test: "decision_model_loop_admits_under_a_hole_and_rejects_under_the_control in crates/happenstance-testkit/tests/completeness_instrument.rs — run by `cargo test -p happenstance-testkit --test completeness_instrument readers::`; plus a search of crates/happenstance-testkit/tests/ finding no local definition of read_decision_model or is_violated_by"

- id: AC-002
  criterion: |-
    GIVEN the evaluator deciding in one sitting from public evidence (`initiative.md:224-226`), who cannot act on "a test failed", WHEN they open this story's record, THEN every read-half observation is the **value itself**: the `Option<SequencePosition>` `read_decision_model` returned under each configuration (the hole's being the last *retained* match, and provably different from the control's), the length of the `Vec<SequencedEvent>` it returned under each, and the append `Result` — `Ok(position)` under the hole with that position recorded, `Err(AppendError::ConditionViolated(_))` under the control with the violating position recorded — AND no `#[should_panic]`, no "it failed" boolean, and no assertion whose failure message would not name the observed value.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/completeness_instrument.rs — the `readers` module's observation record (the two Option<SequencePosition> values, the two Vec lengths, the two append Results), transcribed into .bklg/from-contract-to-published-library/retention-and-incomplete-logs/decision-model-and-ingest-observed/_observations.md"
  verifying_test: "decision_model_loop_admits_under_a_hole_and_rejects_under_the_control in crates/happenstance-testkit/tests/completeness_instrument.rs, asserting on the four concrete values with messages that print them; negative check that no `should_panic` attribute appears in that target"

- id: AC-003
  criterion: |-
    GIVEN the adapter author, who needs an executable definition of "correct" rather than prose to interpret (`initiative.md:210-215`), and who must not read this observation as "a store is misbehaving", WHEN they open the `readers` module or `_observations.md`, THEN both state in terms that the admitted append is ES-40's **specified** behaviour — `Guard::is_violated_by` has two arms and no third, so a condition over destroyed history passes vacuously — that the finding concerns what the *caller* may then conclude, and that the rule and rustdoc which discharge ES-40 belong to `condition-over-removed-history-does-not-reject`; AND the diff shows **no file under any crate's `src/`** changed by this PR: no rule body, no `for_each_event_store_rule!` line, no `Fixture` item, no signature, no rustdoc sentence.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/completeness_instrument.rs — the `readers` module's own rustdoc; and the PR boundary itself (crates/happenstance-testkit/tests/**, crates/happenstance-sync/tests/**, this story's folder)"
  verifying_test: "content review of the readers module rustdoc and _observations.md against the three named sentences, at the story review gate; plus `git diff --stat main...HEAD` showing nothing under any crate's src/, and `mutants_fail_exactly_their_declared_rules` / `mutant_registry_is_exhaustive` (crates/happenstance-testkit/tests/mutation_coverage.rs) green and unchanged"

- id: AC-004
  criterion: |-
    GIVEN the local-first / edge developer replicating between two stores (`initiative.md:217-222`), whose peer must be able to ask "do you already have this event?", WHEN an event **this store minted and acknowledged** falls inside the hole, THEN `contains_event_id` returns `false` for that exact `EventId` under the hole and `true` for the same id under the control, both recorded as values; the id is resolved through the inner store's `snapshot()` and **never** through `EventId::position()` — that half belongs to the *origin* store for an ingested event, and that reduction is the one DA-8 shows to be wrong; and `_observations.md` states that this `false` is byte-for-byte the answer "never had it" returns, so a peer that re-sends on `false` re-sends forever.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/completeness_instrument.rs — the `readers` module calling EventStore::contains_event_id (crates/happenstance-core/src/store.rs:250-269) through HS-S0114's instrument, resolving the id via the inner MemoryEventStore's snapshot() (crates/happenstance-core/src/memory.rs:186)"
  verifying_test: "contains_event_id_answers_false_for_an_id_this_store_minted in crates/happenstance-testkit/tests/completeness_instrument.rs — paired control/hole, recording both answers and the id; plus the absence of any EventId::position() call in the readers module, and contains_event_id_reports_membership (crates/happenstance-testkit/src/registry.rs) still green in the existing mounts"

- id: AC-005
  criterion: |-
    GIVEN the same developer, for whom `IngestStore::holds` is the ingest-side spelling of that same question, WHEN this story runs, THEN the precondition is checked **first** and recorded — does `impl SendIngestStore for MemoryEventStore` have a real `holds` body, and is `happenstance_sync::EventId` unified with core's — and **exactly one** branch is taken: **reached**, a new `crates/happenstance-sync/tests/` target calling the **production** `holds` over a forgetting store and recording its `false`; or **recorded**, a finding citing `crates/happenstance-sync/src/ingest.rs:223-225` verbatim (whose `todo!()` names `contains_event_id` as its future answer, which *is* DA-8's inheritance), naming HS-P0017 as the owner of the write path, and stating what remains unobserved. In neither branch does a hand-written `IngestStore` impl, a stubbed body, or a replaced `todo!()` appear.
  satisfied: false
  evidence: ""
  mount_point: "Reached branch: a new target under crates/happenstance-sync/tests/ calling happenstance_sync::IngestStore::holds (crates/happenstance-sync/src/ingest.rs:164) via its only production impl (:206-229). Recorded branch: the precondition block in .bklg/from-contract-to-published-library/retention-and-incomplete-logs/decision-model-and-ingest-observed/_observations.md, citing crates/happenstance-sync/src/ingest.rs:223-225"
  verifying_test: "Reached: `cargo test -p happenstance-sync --test ingest_holds_across_a_hole`. Recorded: content review of _observations.md's precondition block against crates/happenstance-sync/src/ingest.rs:223-225 and crates/happenstance-sync/src/identity.rs:26-39 as they stand at merge. Both: no `impl … IngestStore` under crates/happenstance-testkit/tests/ or crates/happenstance-sync/tests/, and `git diff main...HEAD -- crates/happenstance-sync/src` empty"

- id: AC-006
  criterion: |-
    GIVEN the repository owner asking DR-6's question — does a reader fail loudly? (`project.md:160-164`) — who cannot accept "no" as a claim, WHEN they read this story's record, THEN the silence is **enumerated**: for each reader, every channel that could have carried an incompleteness signal is listed with the value it carried under the hole — the `Result` (`Ok`), `read_decision_model`'s return tuple (no incompleteness field), `head()` (the highest **retained** position, the same shape a young store returns), `contains_event_id` (`false`, the same value "never had it" returns), and `AppendError`'s variants (`ConditionViolated`, `NoEvents`, the capacity variant, the store variant — **none** meaning "I cannot judge this condition") — so "fails loudly: no" is supported by where the author looked rather than asserted.
  satisfied: false
  evidence: ""
  mount_point: "The silence-enumeration table in .bklg/from-contract-to-published-library/retention-and-incomplete-logs/decision-model-and-ingest-observed/_observations.md, whose cells are the values recorded by the readers module in crates/happenstance-testkit/tests/completeness_instrument.rs"
  verifying_test: "content review of the silence table against crates/happenstance-core/src/error.rs:214-225 and crates/happenstance-core/src/store.rs:248, :250-269, :321-331 — a channel present in the source and absent from the table fails the row; mechanically fed by decision_model_loop_admits_under_a_hole_and_rejects_under_the_control and contains_event_id_answers_false_for_an_id_this_store_minted"

- id: AC-007
  criterion: |-
    GIVEN the repository owner, who has already pre-computed DA-7's four options and their version consequence and must not have one of them quietly implemented, WHEN this PR is read, THEN each observation **names the DA-7 row it bears on** — the membership observation at row 4 (a tri-state `contains_event_id`, smallest signature delta), the admitted-append observation at rows 2 and 3 (retained ranges; a third outcome on condition evaluation) — records the version consequence (`0.3.0`, outside this initiative's exit criteria) and **stops**: no public signature in `crates/happenstance-core/src/store.rs` or `crates/happenstance-core/src/append.rs` is touched, no `spec/SPECIFICATION.md` edit or marker move is in the diff, and the escalation artefact itself is left to `surface-diff-and-the-ac-012-escalation`.
  satisfied: false
  evidence: ""
  mount_point: "The missing-surface finding block in .bklg/from-contract-to-published-library/retention-and-incomplete-logs/decision-model-and-ingest-observed/_observations.md, mapped onto the DA-7 option table at .bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md:363-379"
  verifying_test: "content review of the finding block against _decomposition.md:363-379 (a surface named with no DA-7 row fails; an option named with no version consequence fails); plus `git diff main...HEAD -- crates/happenstance-core spec` empty and `cargo xtask spec-trace` green and unchanged under `cargo xtask affected --base main`"

- id: AC-008
  criterion: |-
    GIVEN the repository owner writing ADR-0028 two slices later, who needs to cite **cells** rather than paraphrase paragraphs (`RUNBOOK.md:4658-4662`), WHEN they open this story's folder, THEN `_observations.md` exists there — not in a shared project file two stories both write — carries **one row per observation** with **reader**, **configuration** (the retained-set value printed via its `Debug`, which names variant *and* bound), **call**, **control value** and **hole value** in separate columns, followed by the silence enumeration and the missing-surface finding — and **every cell holds a value the code returned**, never a description of one.
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/retention-and-incomplete-logs/decision-model-and-ingest-observed/_observations.md — transcribed from the observation record built in crates/happenstance-testkit/tests/completeness_instrument.rs's readers module"
  verifying_test: "content review at the story gate: each cell cross-checked against the assertion that produced it in crates/happenstance-testkit/tests/completeness_instrument.rs; a cell whose text does not appear as a literal or a printed value in that target's output fails the row. Bar: _decomposition.md:704-706 (AC-T04) and RUNBOOK.md:4658-4662"
```
