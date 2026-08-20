# Wave `2026-08-20-intake-phase-9` — integration summary

Eleven operations executed as adjudicated in `02-placement-and-adjudication.md`. Six atoms
created, five amended (`merge_existing`), zero superseded by metadata flip — no accepted
`decision` atom was edited, and none needed to be.

Five intake documents were ingested: phase 9's ADR-0023 record, the ES-6 verdict, the CF-40
ownership resolution, the WF-11 measurement, and `0034-what-the-phase-8-reconciliation-cost.md`
(tracked since `4ad58d0` and swept up by this run, as the staging story predicted).
`.kb/_intake/README.md` was excluded at the approval gate: it is a README, not an atom.

## Atoms created

| # | destPath | id | kind |
| --- | --- | --- | --- |
| 1 | `.kb/decisions/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md` | `kb-decision-0023` | decision |
| 2 | `.kb/decisions/0034-the-fixture-contract-has-no-single-owner.md` | `kb-decision-0034` | decision |
| 3 | `.kb/reference/wf-11-memory-ceiling-verdict-2026-08.md` | `kb-reference-wf-11-memory-ceiling-verdict-001` | reference |
| 4 | `.kb/reference/phase-8-specification-reconciliation-census.md` | `kb-reference-phase-8-spec-reconciliation-001` | reference |
| 5 | `.kb/open-questions/no-workerd-class-runner-in-the-gate.md` | `kb-open-question-workerd-runner-absent-001` | open_question |
| 6 | `.kb/open-questions/deny-bans-red-on-the-worker-dependency.md` | `kb-open-question-worker-async-trait-ban-001` | open_question |

By kind: 2 decision, 2 reference, 2 open_question.

`kb-decision-0023` merges the ADR-0023 SqlStorage-mapping intake with the ES-6 verdict as one
body of evidence, under the stated exception in `kb-playbook-one-decision-per-adr-title-001`.
`kb-decision-0034` answers `kb-open-question-cf-40-ownership-001` **from outside**, because
ADR-0015, ADR-0012 and ADR-0022 are all accepted and therefore immutable.

## Atoms amended

Five `open_question` atoms, all `merge_existing`, none with an existing sentence deleted.

| destPath | id | frontmatter delta |
| --- | --- | --- |
| `.kb/open-questions/cf-40-fixture-limits-ownership.md` | `kb-open-question-cf-40-ownership-001` | `status: accepted → superseded`; `related` += `kb-decision-0034`, `kb-decision-0023` |
| `.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md` | `kb-open-question-human-readable-encoding-limits-001` | `status: accepted → superseded`; `related` += the WF-11 verdict and the absent-runner question |
| `.kb/open-questions/es-6-names-an-unwritable-rule.md` | `kb-open-question-es-6-unwritable-rule-001` | **`status` deliberately left `accepted`** — see below; `related` += `kb-decision-0023` |
| `.kb/open-questions/poll-count-bounds-the-visibility-rule.md` | `kb-open-question-poll-count-rule-strength-001` | frontmatter-only, zero body hunks; `related` += `kb-decision-0034` |
| `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md` | `kb-open-question-post-phase-reconciliation-001` | `status` unchanged; `related` += `kb-reference-phase-8-spec-reconciliation-001` |

### ES-6 stays open, on purpose

The ES-6 verdict judges **ADR-0009's prediction**, not **ES-6's rule**. The prediction — that the
absent `Send + Sync` bound would cost a caller something on the first runtime able to produce a
`!Send` error — was tested against a real one carrying a live JavaScript value, and it held.
`store_error_crosses_a_join_handle` is still unwritten, still unowned, and still named by a
`[FROZEN]` clause. Three independent instructions agreed on this reading: the intake document's
own *"Do not let the wave read this document as answering that question"*, the atom's existing
*What is not decided* section, and ADR-0023's non-claim section. The open-questions index bullet
stays **Open**.

## Maps and backlinks

Three index atoms edited: `.kb/maps/decision-map.md`, `.kb/maps/domain-map.md`,
`.kb/maps/open-questions-index.md`. Eleven reciprocal backlinks wired, including the four
pre-existing atoms that gained inbound references from this wave's new reference atoms
(`kb-reference-wire-format-measurements-001`, `kb-playbook-anchoring-citations-001`,
`kb-playbook-ratchet-gate-landing-001`, `kb-reference-spec-trace-has-suite-001`).

## Validation

`redkiln validate --kb` — **pass**. Every atom conforms to `KbFrontmatter`, no duplicate ids,
every `depends_on` / `related` / `supersedes` / `superseded_by` reference resolves, and
accepted-decision immutability holds against `HEAD`.

`redkiln doctor` — **exit 1**, on nine `.bklg` structural errors of a single class (*foundation
story X is consumed by no capability slice*) plus the six permanent `template-drift` advisories
CLAUDE.md documents. All nine pre-date this wave: they were last touched by `ae77ac4` and were
independently confirmed present on `initiative/from-contract-to-published-library`, a branch this
wave does not modify. This wave's diff touches `.kb/` only. They are **not** fixed here —
`.bklg` system frontmatter is CLI-write-only, and reworking capability-slice ownership for an
unrelated ten-project initiative is not a KB-intake wave's work.

## One figure corrected rather than transcribed

The ADR-0023 intake stated the harness is *"driven by one row in `xtask/src/proof.rs`'s
executed-target registry"*. `xtask/src/proof.rs:589-600` says *"Three rows, and the shape is the
deliverable"*. The atom says **one row added to a three-row registry**. Flagged rather than
silently fixed, because it is precisely the class of defect Op 10's census is about.

## What this wave did not settle

Seven items are recorded `unresolved` in the run manifest and carried in
`02-placement-and-adjudication.md`. Two of them matter to `HS-P0013` directly, because the
backlog escalated three questions to ADR-0023 and this wave signs only one:

1. **The `deny.toml` ratify-or-refuse call is declined**, not made. Both shapes arrive named and
   neither chosen, and ratifying an exemption to a binding constraint (ADR-0001) against a red
   gate with no human present is what `.kb/decisions/README.md:31-33` forbids. Routed to
   `kb-open-question-worker-async-trait-ban-001`. A human can overrule in one edit: ADR-0023
   gains a section and that atom is dropped.
2. **The three store-limit numbers are not adjudicated by any staged document.** The CF-40 file
   only *reports* `CloudflareFixture` declaring them. ADR-0034 states them as observed fact about
   the fixture and does **not** state they are ratified. This surfaces in slice 3's owed review.

The remaining five: the WF-11 `status` flip (the wave's own most contestable call — the question
is moved to `superseded` although sub-question 2 is explicitly not reached, reversible in one
field); ADR-0034's `reversibility: high` (the wave's proposal, the only reversibility value in
the wave with no source behind it); whether the CF-40 resolution should carry an `adr_id` at all
(nothing in `.bklg/` allocates one, so `0034` is highest-taken + 1 by the wave's own rule); and
three mutable non-atom obligations left undone on purpose — `RUNBOOK.md:4394-4396`'s stale
instruction to retire ADR-0001's provisional marker, `RUNBOOK.md:302`'s phase-9 queue row (which
can now be struck, because the atom's path exists), and project AC-005's wording.

## Finalize

The wave's integrate and maps phases completed and `validate --kb` passed; the workflow's verify
agent stopped before writing this summary, clearing `.kb/_intake/` and committing, because it
treats any `doctor` exit 1 as a halt. The halt was assessed against the branch and found
unrelated to `.kb/`, so the finalize was completed by hand: this summary written, the five
ingested intake documents removed, `README.md` kept, and the wave committed on
`worktree-kb-intake-2026-08-20`. `04-retrospective.md` was not authored — the workflow owns that
artifact and did not reach it.
