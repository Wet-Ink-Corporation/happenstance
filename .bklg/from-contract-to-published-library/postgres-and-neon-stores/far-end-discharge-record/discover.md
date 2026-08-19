---
item: HS-S0073
stage: discover
created: 2026-08-12T13:02:42.505Z
updated: 2026-08-12T13:02:42.505Z
template_sig: 86ce4036
rendered_sig: d8bfd12f
---

# Discover — The far-end discharge recorded for the publication audit

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one line: the status of ES-10, ES-11, ES-12, ES-41, ES-42 and VT-21 – VT-24 after this work — discharged, still exposed, or amended, and **against which store** — written where the publication audit reads it instead of re-deriving it | `_storymap.md`, *Slices* table, `far-end-discharge-record` row | The unit of the record is a clause plus a store plus an outcome. A marker on its own is not the deliverable |
| **AC-013** — written "in a form `publication-and-positioning` can read at audit time rather than re-derive" | `project.md`, *Acceptance criteria*, AC-013 | Sole owner. The audience is a future project, not this one |
| `depends_on: deskeleton-and-package-readiness` (HS-S0072) | manifest; `_storymap.md`, *Merge order* item 6 | Terminal by construction: it "reports on what the five slices above it actually discharged", so it cannot run until they have |
| The residual-exposure row names ES-10, ES-11, ES-12, ES-35 and ES-40 as the five clauses §1.3 identifies as carrying CF-25's risk in their own markers, and phase 10 as owner of the first three | `RUNBOOK.md:606` | The row also states what a far end *is*: "a **fixture** instrument does not falsify any of them — CF-26 says so in terms" |
| Current markers: ES-10 **FROZEN**; ES-11 and ES-12 **PROVISIONAL**; ES-41 and ES-42 **PROVISIONAL**; VT-21 – VT-24 **PROVISIONAL** | `spec/SPECIFICATION.md:8592-8594`, `:8623-8624`, `:8547-8550` | The nine rows this record covers, and their starting state |
| The instrument-portfolio table's two rows this project fills: position allocation ("**fixture, phase 3** … Adapter at phase 10") and transport ("**empty at both ends.** Adapter at phase 10") | `RUNBOOK.md:687-688` | What "discharged" would mean, per axis, and what a fixture-only answer would still leave open |
| `references/adapter-shapes.md` records what each skeleton told the type checker, and DoD 6 asks for this project's entries "including anything that contradicts a prior assumption" | `project.md`, *Definition of done* item 6; `RUNBOOK.md:682` | The Neon `conflicting_position` result and the Postgres frontier cost are both prior-assumption contradictions by construction |
| `cargo xtask spec-trace --write` renders §7.1–§7.2 between explicit markers, and the region is generated rather than hand-maintained | `xtask/src/spec_trace.rs:23-31`, `:200`, `:204` | "never hand-edited" (`_storymap.md`, *What each story is*). A hand edit passes review and then drifts against `parse_clauses` |
| **§7.1 and §7.2 come from the same `parse_clauses`** that the check itself uses | `xtask/src/spec_trace.rs:46` | Which is why regenerating them is *not* evidence: the table agrees with the clauses because it is derived from them, whatever any adapter did |
| "lifting on no new evidence moves a maturity marker because a phase wanted it moved, **which is the one thing a marker must never do**" | `.kb/decisions/0012-append-shape-and-preconditions.md:97-99` | The bar for promoting ES-11 or ES-12 out of `[PROVISIONAL]`. This story records status; it does not move markers |
| The runbook's residual-exposure groups are **knowingly wrong**: short by ES-41, ES-42, CF-39 and CF-40, and the last row still carries ES-10, which is no longer provisional — "five edits and it is the next pass's, not this one's — but it is phase 12's audit that reads this table" | `RUNBOOK.md:622-635` | Explicitly **not** this story's: `project.md` assigns the reconciliation to `publication-and-positioning`. Recorded so the record can point at it rather than silently inherit it |
| The published compliance claim must name which implementations it was checked against | `project.md`, *Unlocks*; `_decomposition.md`, *Decisions taken at the gate*, item 1 | This is why "against which store" is part of AC-013's wording, and why a per-clause marker alone cannot serve |
| Every rule the Neon run did not pass is either a declared capability with a written reason or an amended clause | AC-007 (`project.md`); DR-5 | Those reasons are inputs to this record: a clause "exercised" against a skip is a different fact from one exercised against a pass |
| `xtask/src/proof.rs`'s `ARTEFACTS` asserts each phase's proof artefact still holds the tests its clauses name | `xtask/src/proof.rs:133` | The nearest existing mechanism to what this record is, and the natural place to ask whether phase 10 earns a row |

## Questions

**Answered.**

1. *What is the unit of the record?* A **triple**: clause, store, outcome — where
   outcome is one of passed / failed / skipped-with-this-reason / amended-by-this-record
   / not exercised. AC-013's "and against which store" and the initiative's
   requirement that the compliance claim name its implementations both force this.
   A per-clause status column alone is not readable by the audit.
2. *Does this story move any maturity marker?* No. It records status; promoting
   ES-11 or ES-12 out of `[PROVISIONAL]` is a decision with its own bar
   (`.kb/decisions/0012-append-shape-and-preconditions.md:97-99`) and belongs to
   `publication-and-positioning` (HS-P0016) with the evidence this record supplies.
3. *Does it fix the runbook's short residual-exposure groups?* No —
   `project.md` assigns that reconciliation to HS-P0016
   (`RUNBOOK.md:622-635`). The record points at the discrepancy so the audit is not
   surprised by it.
4. *How are §7.1–§7.2 updated if a marker moved?* Only by `cargo xtask spec-trace
   --write`, between its generated markers (`xtask/src/spec_trace.rs:200`, `:204`).
   Never by hand.

**Deferred to `spec`.**

5. *The record's location and format* — a file under `references/`, a section of
   `RUNBOOK.md`'s phase-10 session log, an `ARTEFACTS` row in `xtask/src/proof.rs`,
   or some combination. The constraint is that HS-P0016 can read it without
   re-deriving it, and that it does not rot silently.
6. *Whether phase 10 earns an `ARTEFACTS` row.* It is the only mechanism in the tree
   that makes a proof artefact's contents checkable rather than assertable, and
   this project's artefact — the concurrency family green on an unserialised store,
   plus the Neon skip list — is exactly the kind it holds.
7. *The `references/adapter-shapes.md` entries and the RUNBOOK session-log text*
   DoD 6 asks for, including the contradicted prior assumptions.

**Deferred to the owning stories.**

8. *How the adapter buys ES-10's visibility invariant.*
   `adr-0024-position-visibility-mechanism` (HS-S0065). This record reports the
   result and cites the ADR; it does not restate the decision, and a summary that
   disagreed with the atom would be a third copy of a requirement.
9. *Whether `conflicting_position` is a promise every adapter owes or a hint one may
   omit.* `neon-conflicting-position-verdict` (HS-S0070). Its verdict is an input
   here — specifically, whether ES-25's `None` permission was exercised by Neon or
   not — and this record must reproduce it rather than re-derive it.

## Decision

The problem this slice solves is that the evidence this project produces is
perishable. It will exist as CI job logs, as a green concurrency family, as a skip
list printed with `--show-output`, and as an ADR — and `publication-and-positioning`
will need, months later, to state which clauses were checked against which
implementations, because its published compliance claim names them. Re-deriving that
from job history is the failure AC-013 exists to prevent, and the failure is worse
than tedium: a re-derivation cannot recover *why* a rule was skipped, or which
fixture was behind a green run. This story writes the record while the runs are
fresh: nine clause rows — ES-10, ES-11, ES-12, ES-41, ES-42 and VT-21 – VT-24 — each
carrying the store it was exercised against and the outcome that was observed, plus
the `references/adapter-shapes.md` entries and the RUNBOOK phase-10 session-log text
DoD 6 asks for. The spec will cover: the record's location and shape; the triple that
is its unit; how a skip's stated reason is carried through; the pointer to the
runbook's knowingly-short residual-exposure groups as HS-P0016's to reconcile; and
whether phase 10 earns an `xtask/src/proof.rs` `ARTEFACTS` row so the record is
checked rather than merely written. **No marker is moved and no clause is changed**
— ES-10 stays `[FROZEN]`, ES-11 and ES-12 stay `[PROVISIONAL]` — so no ADR is owed
by this story; moving any of them is HS-P0016's act, on this record's evidence, with
its own decision behind it.

## The wrong implementation

**A discharge record generated from the specification.** Run `cargo xtask spec-trace
--write`. Read §7.1's rows for ES-10, ES-11, ES-12, ES-41, ES-42 and VT-21 – VT-24.
Copy the maturity column into a table, add a sentence saying phase 10 has completed,
commit. Every check passes, and passes *necessarily*: `spec-trace` is green because
§7.1 and §7.2 are rendered from the same `parse_clauses` the check itself runs
(`xtask/src/spec_trace.rs:46`), so the table agrees with the clauses no matter what
any adapter did or did not do. The record is internally consistent, well formatted,
and says nothing. What the audit needs — which store, which run, which outcome — is
exactly the information a marker cannot carry, and CF-26 is the clause that says so:
a fixture instrument proves a rule *can fail*; only an adapter instrument proves a
real implementation *can pass* (`RUNBOOK.md:606`, `:687-688`). A record built from
markers cannot distinguish "ES-11 passed against a real Neon endpoint" from "ES-11's
rule was skipped because `NeonFixture` declined a capability" from "ES-11 was never
exercised at all", and all three would be written down identically.

**Its inherited sibling: a record that is accurate about a run that was not real.**
If HS-S0068's named mutant landed — `NeonFixture` backed by a pooled Postgres
connection — then ES-11 and ES-12 were genuinely exercised, genuinely passed, and are
genuinely reported here as discharged at the transport axis's far end, which is
still empty (`RUNBOOK.md:688`). This story is the last place that lie is
recoverable and the first place it becomes durable, because HS-P0016 reads this
record rather than the fixture. `spec` therefore requires the record to name the
*fixture and its backing*, not only the store type — a row that says
"`NeonEventStore` over `NeonFixture`" is satisfied by the mutant; one that says
"against Neon branch X via the `/sql` endpoint, job Y" is not.

**And the marker mutant: promoting ES-11 and ES-12 to `[FROZEN]` because the run was
green.** It is one word per row, `spec-trace --write` regenerates §7.1 cleanly
afterwards, and the gate stays green — `spec-trace` verifies that a clause cites
rules which exist, not that a marker was earned. ADR-0012 names this exact act as the
one thing a maturity marker must never do: "lifting on no new evidence moves a
maturity marker because a phase wanted it moved"
(`.kb/decisions/0012-append-shape-and-preconditions.md:97-99`). Here there *is* new
evidence, which makes it more tempting rather than less — and the promotion is still
a decision with an owner, and the owner is HS-P0016.

This story adds no conformance rule and defines no store, so nothing is owed to
`crates/happenstance-testkit/tests/`. The instrument against all three mutants is the
record's own shape, which is why `spec` specifies the triple rather than the table.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
