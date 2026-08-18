---
item: "HS-S0047"
stage: report
created: "2026-08-18"
updated: "2026-08-18"
---

# Report — The standing reconciliation criterion, discharged

## Findings Ledger

**Six of six ACs satisfied. Nothing blocked.** No Rust changed, nothing
`[FROZEN]` was amended, no maturity marker moved, and the clause census is
byte-identical before and after: 201 clauses, 137 FROZEN / 47 PROVISIONAL / 12
DEFERRED / 5 NON-NORMATIVE.

| AC | Result | What proves it | Where it landed |
| --- | --- | --- | --- |
| **AC-001** — a verdict for every clause in the computed range | **satisfied** | 8 clauses in set A + 15 further passages whose prose cites this crate; 3 unchanged, 20 repaired, none passed silently. Four sentences whose MUST was untouched and whose prose described a superseded implementation are recorded as **defects**, not passes | `_reconciliation.md`, *Clause-by-clause verdicts* |
| **AC-002** — every pointer lands on its stated subject | **satisfied** | Four schema-sketch citations re-anchored `:47-53` → `:62-67`; the projection census corrected in two places; ES-41's spent falsifier restated; D7's read path read against ADR-0022's settled runtime seam; five further drifted pointers found that no spec named in advance. **None deleted** — `checked` rose 389 → 396 | `spec/SPECIFICATION.md` |
| **AC-003** — every edit is a repair | **satisfied** | 152 insertions / 72 deletions; exactly two deleted lines contain "MUST" and both were read (one table row whose PS-2 column is reproduced verbatim, one prose sentence). Two met obligations named **as** discharges with code and test cited | `spec/SPECIFICATION.md` |
| **AC-004** — the two clause-range sets, compared | **satisfied**, and the comparison is itself the finding | A = {CF-14, CF-17, CF-34, ES-35, VT-21…VT-24}; B = **{ }**; B′ = {ES-17, CF-40}. Symmetric difference of ten, one disposition each | `_reconciliation.md`, *Clause ranges* |
| **AC-005** — neither citation figure falls | **satisfied**, after a real red | 389 → 396 checked, 76 → 78 anchored. `anchored` fell to 75 mid-pass (EC-004) and was diagnosed and re-anchored rather than absorbed; `ANCHOR_SLACK` untouched | `spec/SPECIFICATION.md`, `_reconciliation.md` |
| **AC-006** — verdicts transcribed, open question left open | **satisfied** | CF-14, CF-17 and ES-35 each traced to the upstream ledger row and read back at their current lines; falsifier-length check green; `.kb/open-questions/…` unedited; findings staged under `.kb/_intake/`, no atom written | `.kb/_intake/0034-…md` |

## The finding worth carrying forward

**`spec-trace` green is not evidence that a citation is right.** Six citations
into `crates/happenstance-sqlite` *resolved* — file present, line in range — while
naming an entirely different subject than the sentence claimed. `:151-193` was
cited as an error enum and holds a struct; `:202` as an impl and holds a `const`;
`:195` as a `head` impl and holds `SCHEMA_VERSION`; and four separate clauses cited
`:47-53` as "the tag side table" when that range is the `event` table.

None was reported, and the tool is not at fault. Of 396 citations checked, **78**
are *anchored*, because the subject is only derivable when the code span
immediately before the citation is a lowercase identifier on the citation's own
line or the one above. The other 318 are counted and unexamined. A reader who
takes a green `spec-trace` as "the citations are right" is reading a claim about
one fifth of them.

That is staged as finding 1 of
`.kb/_intake/0034-what-the-phase-8-reconciliation-cost.md`, with two candidate
mechanisations and neither implemented.

## The routed item, settled

The predecessor story moved four §6.5 statements into agreement and deliberately
**routed one here** rather than moving it in passing: the handle-multiplicity row
still read *"no connection has ever been opened twice"* while
`SqliteFixture::connect` had already falsified it.

**Verdict: the axis has an adapter instrument at its far end.** The second
connection is opened through `SqliteEventStore::open`, configured and migrated —
not a refcount bump — and `connect_many` opens up to 64 of them for a green
concurrency family. The portfolio's adapter column therefore goes from one tick to
two, and §1.3, §1.6, §1.7 and §6.5 moved together so that four places agree.

The narrower reading — that only a *pool-backed* adapter counts — is recorded as
considered and rejected: it renames the far end after the fact. Pooling, and
cross-process handles, are named as what is still missing. CF-16 and CF-19 are
`[FROZEN]` and **unchanged**; this is an edit to the evidence table, not to either
clause.

## Gaps escalated, not applied

1. **ADR-0022 states no clause range** — no frontmatter field, no clause ID in its
   decision, and `RUNBOOK.md:301`'s queue row is the only one in the queue with no
   parenthesised range. Escalated to the runbook's ADR queue. Not fixed here: an
   accepted decision atom is immutable, and hand-writing a range into it is exactly
   the edit `redkiln validate --kb` exists to refuse.
2. **ES-17's two-build measurement is owed by nobody** — already escalated by
   ADR-0022 §13 (`RUNBOOK.md:302`). This pass confirms it survived phase 8's
   implementation stories and adds nothing to it.

## Deferred

Nothing in this story's own scope. Two things are deliberately left open, and both
are the point rather than an omission:

- **The reconciliation open question stays open.** No committed citation baseline,
  no new gate check, no machine-readable clause-range format, and
  `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md` is unedited
  in this diff. Running the criterion once by hand answers none of its five
  ordered sub-questions.
- **Case authoring** for the durability half of E2E-07, E2E-46's precondition and
  E2E-08 against a real second handle (`RUNBOOK.md:4231-4233`) is phase-8 work
  outside this slice. Only citation repair to existing cases was in boundary, and
  `spec/E2E-CASES.md` needed none — it cites this crate nowhere.

One item for a reviewer to challenge rather than accept: the handle-multiplicity
verdict is a *judgement*, not a measurement. If a reviewer holds that only a
connection pool puts an adapter at that axis's far end, the row goes back to
"Fixture yes, adapter no" and four statements move with it — the argument for the
reading taken is written out in `_reconciliation.md` rather than assumed.
