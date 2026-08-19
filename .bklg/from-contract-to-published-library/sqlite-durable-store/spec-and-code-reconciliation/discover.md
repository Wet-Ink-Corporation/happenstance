---
item: HS-S0047
stage: discover
created: 2026-08-12T13:02:09.436Z
updated: 2026-08-12T13:02:09.436Z
template_sig: 86ce4036
rendered_sig: d00d3ad8
---

# Discover — The standing reconciliation criterion, discharged

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: discharge the standing reconciliation criterion — read every clause ADR-0022 discharges against the code as it now stands, compare the phase's clause range against the union of its ADRs' ranges, and show `spec-trace`'s citation count has not fallen. | `_storymap.md`, *Slices* table, row `publishable-and-reconciled` / `spec-and-code-reconciliation` | Three sub-criteria. Only the third has a command behind it. |
| **AC-016** — the specification and the code still agree: the standing criterion at `RUNBOOK.md:3810-3820` is discharged. | `project.md`, AC-016 | The criterion is "standing" — every phase from 6 onward carries it — so this story is the phase-8 instance of a recurring obligation, not a one-off cleanup. |
| `dependsOn: instrument-markers-removed-and-gate-green` (HS-S0045) — the code must be in its final state before "read against the code as it now stands" means anything — and `reopen-negative-control-and-durability-verdicts` (HS-S0043), which is where CF-17, ES-35 and CF-14's markers actually move. | `_storymap.md`, *Merge order* item 5 | Reconciling before the verdicts land would reconcile against markers about to change. |
| The criterion, in full: (a) "Every clause the phase's ADRs discharge has been read against the code as it now stands, not as it stood when the clause was written. A clause whose supporting prose describes a superseded implementation is a defect even when its MUST is untouched." (b) "The phase's clause range and the union of its ADRs' clause ranges are computed and compared." (c) "`cargo xtask spec-trace`'s citation count has not fallen, and any clause the phase froze names a rule that exists or is marked `†`." | `RUNBOOK.md:3810-3820` | (a) is a reading task, (b) is an arithmetic task **nothing implements**, (c) is a command. |
| Why it became a standing criterion at all: "The rule that would have caught it is already written down, at the end of the ADR queue: *a phase's clause range and the union of its ADRs' clause ranges are two numbers, and nothing checks that they are equal.* **Nothing implements it either.**" | `RUNBOOK.md:3805-3808` | The repository states plainly that sub-criterion (b) has no machine. That is the fact this story's mutant turns on. |
| `cargo xtask spec-trace` is wired as part of `reachability_static` and runs on the story grain whether or not anyone types it. | `.redkiln/config.yaml:44-48`; `_decomposition.md`, *Testing brief*, §6 | So (c) is continuously enforced and (a) and (b) are not. The story's risk concentrates exactly where the enforcement is absent. |
| What `spec-trace` actually checks: that a clause's cited rule name exists in `RULE_FILES` (check 4), and that every conformance rule is claimed by a clause, retired by one, or on a stated list (check 6). Its `RULE_FILES` are `suite.rs`, `model.rs` and `concurrency.rs`. | `xtask/src/spec_trace.rs:72-90`, `:756`, `:1956-1969` | It answers "does a name a clause cites exist" and "is every rule owned". It does **not** answer "is the clause's supporting prose still true". |
| The clause census the audit reads: 200 clause IDs, 198 normative — 139 `[FROZEN]`, 49 `[PROVISIONAL]`, 10 `[DEFERRED]`, two `[NON-NORMATIVE]`. | `spec/SPECIFICATION.md:219-222`; `project.md`, *Context anchors* | The census is a number in prose. If this project moves a marker, the census moves with it or becomes wrong — and nothing counts it. |
| CF-38 makes an empty falsifier on a `[PROVISIONAL]` or `[DEFERRED]` marker a **build failure** rather than a convention: "a provisional marker with no falsifier is indistinguishable from a decision nobody wanted to make, and by the time anyone notices it has been load-bearing for a year." | `spec/SPECIFICATION.md:213-218` | Constrains HS-S0043's ES-35 restatement: the narrowed falsifier is mechanically required to be non-empty, though its *truth* is not checked. |
| The clauses this project's ADR discharges: CF-14, CF-17, ES-35, plus CF-34, CF-40 and VT-21 – VT-24 exercised. `RUNBOOK.md:4175` states "**Decisions it settles.** ADR-0022. Discharges CF-14 and CF-17." | `project.md`, *In scope*; `_grounding.md`, *Anchors*; `RUNBOOK.md:4175` | This is the clause range side of sub-criterion (b). The ADR range side is whatever ADR-0022's own clause list ends up being — which is why the two are compared rather than assumed equal. |
| The code that clauses now describe differently: `crates/happenstance-sqlite/src/lib.rs:1-73` (status banner, *Open decisions*), `event_store.rs:36-54` (the known-wrong schema sketch), `event_store.rs:5` ("Every operation is `todo!()`"). | `_decomposition.md`, *Architecture brief*, §7; `_storymap.md`, row `instrument-markers-removed-and-gate-green` | Sub-criterion (a) is not abstract in this project: three specific pieces of published prose describe a superseded implementation. |
| `cargo xtask ci --fast` is this project's bar; the whole gate belongs to `closeout-and-durable-audience`. | `.redkiln/config.yaml:50-60`; `project.md`, DoD 5 | So the citation-count check runs here, and the clause-ledger *audit* — the thing that would notice a census gone stale — is `publication-and-positioning`'s (`project.md`, *Out of scope*). |
| DR-07: ES-35 and CF-14 leave this project with a recorded verdict "so that `publication-and-positioning`'s clause-ledger audit reads a decision rather than a silence." | `project.md`, *Derived requirements*, DR-07 | This story is the last point at which a silence can be caught before it is inherited. |

## Questions

Open questions to resolve before specifying.

1. **How is sub-criterion (b) — the clause-range comparison — actually
   performed?** *Deferred to `spec` as a method, and flagged as the one with no
   machine.* `RUNBOOK.md:3805-3808` says outright that nothing implements it. The
   spec must choose: perform it by hand and record both sets explicitly in
   `_ledger.md` (the phase's clause list, ADR-0022's clause list, and the
   symmetric difference), or build the check. Building it is not in this
   project's scope and would be new `xtask` surface; recording both sets is, and
   is sufficient for AC-016. What is not acceptable is asserting the two agree
   without writing them down.
2. **Which clauses count as "the phase's"?** Answered from the grounding: CF-14,
   CF-17 and ES-35 as discharged; CF-34, CF-40 and VT-21 – VT-24 as exercised.
   The spec should reconcile that list against ADR-0022's own, because the two
   being different is precisely what sub-criterion (b) exists to surface.
3. **Does `spec-trace`'s citation count suffice as evidence for all three
   sub-criteria?** Answered: **no**, and the story must say so in the ledger. It
   discharges (c) only. (a) is a reading with a named list of files, (b) is two
   sets written down.
4. **Does this story move any marker itself?** Answered: **no.** The verdicts are
   `reopen-negative-control-and-durability-verdicts`' (HS-S0043). This story
   *verifies* that the markers, the falsifiers, the rule citations and the census
   agree with each other and with the code — a distinct job from making the
   decision, and deliberately downstream of it.
5. **Does the 200/198/139/49/10/2 census need updating?** Deferred to `spec`, and
   conditional: if HS-S0043's verdicts move a clause between marker classes, the
   census at `spec/SPECIFICATION.md:219-222` is stale and nothing counts it. The
   spec must check the arithmetic rather than assume the prose followed the edit.
6. **Does anything here amend a `[FROZEN]` clause?** Answered: **no.** This story
   reads and reconciles; it changes no MUST. If reading turns up a frozen clause
   whose supporting prose is now false, the *prose* correction is in scope and the
   MUST is not — and if the MUST itself is wrong, that is a new ADR written first
   and a re-plan (`project.md`, *Out of scope*).
7. **The append-condition SQL strategy.** Settled by ADR-0022; this story checks
   that the clauses describing it now match the code, and settles nothing.

## Decision

Every phase from 6 onward carries the same standing exit criterion because the
repository caught itself with a specification whose MUSTs were intact and whose
supporting prose described an implementation that no longer existed — and this
project changed more code, and more clause markers, than any phase since the
freeze. This slice discharges the criterion for phase 8. The spec will cover: (a)
reading every clause ADR-0022 discharges or exercises — CF-14, CF-17, ES-35,
CF-34, CF-40, VT-21 – VT-24 — against the code as it now stands, with the three
known-stale sites named in advance (`crates/happenstance-sqlite/src/lib.rs:1-73`'s
status banner and *Open decisions*, `event_store.rs:5`'s "Every operation is
`todo!()`", and `event_store.rs:36-54`'s schema sketch), verifying rather than
assuming that the earlier stories corrected them; (b) computing the phase's clause
range and the union of ADR-0022's clause ranges and **writing both sets into
`_ledger.md`**, since `RUNBOOK.md:3805-3808` records that nothing implements this
comparison; and (c) running `cargo xtask spec-trace` and recording that the
citation count has not fallen and that every marker this project moved still names
a rule that exists or is marked `†`, with CF-38's non-empty-falsifier requirement
satisfied by HS-S0043's restated ES-35. This story adds no conformance rule, so
the literal-position bar is vacuous, and it amends no `[FROZEN]` clause — a frozen
clause found to have false supporting prose gets its prose corrected; a frozen
clause found to be *wrong* is a new ADR and a re-plan.

## The wrong implementation

**The mutant: `cargo xtask spec-trace` green, the citation count unchanged, and
the criterion declared discharged.** It is the natural reading — sub-criterion (c)
is the only one with a command, the command is already wired into
`reachability_static` (`.redkiln/config.yaml:44-48`) so it has *already run*
before anyone looks, and a green step is the most quotable evidence available.

It satisfies every check that exists and discharges one criterion of three.
`spec-trace` answers two questions: does a name a clause cites exist somewhere in
`RULE_FILES`, and is every conformance rule claimed or retired by some clause
(`xtask/src/spec_trace.rs:756`, `:1956-1969`). It does not read a clause's
supporting prose, and it cannot: "a clause whose supporting prose describes a
superseded implementation is a defect even when its MUST is untouched"
(`RUNBOOK.md:3812-3815`) is a semantic claim about English. So in this project the
green run is compatible with — for instance — CF-14 still describing `PRAGMA
synchronous = OFF` as a wrong implementation no adapter can yet exhibit, ES-35's
falsifier still reading "falsified by the first file-backed adapter" after that
adapter shipped, and `crates/happenstance-sqlite/src/lib.rs` still announcing on
docs.rs that "every operation that touches SQL is `todo!()`". Every citation
resolves. Every count holds. The specification and the code have diverged in
exactly the way the standing criterion was written, after a real incident, to
catch.

**The second mutant, and it is the one the runbook pre-emptively named: asserting
sub-criterion (b) without computing it.** "The phase's clause range and the union
of its ADRs' clause ranges are computed and compared" — the ledger entry says
"computed and compared, they agree", and nobody can tell whether the sets were
ever written down, because **nothing implements the check**
(`RUNBOOK.md:3805-3808`). There is no artefact and no failure mode. This is the
defect that created the standing criterion in the first place: a rule that was
"already written down, at the end of the ADR queue" and that nothing enforced. The
control is the only one available without building new `xtask` surface — the two
sets are enumerated **literally** in `_ledger.md`, so a reviewer can recompute
them, and their symmetric difference is stated even when it is empty. An empty
difference written out is evidence; "they agree" is a claim.

**The third mutant, specific to this project and cheap to miss: the clause census
left at 200 / 198 / 139 / 49 / 10 / 2.** If HS-S0043's verdicts move CF-14 out of
`[DEFERRED]` or restate ES-35, the counts at `spec/SPECIFICATION.md:219-222` are
prose facts that nothing counts — CF-38 makes an *empty falsifier* a build failure
(`spec/SPECIFICATION.md:213-218`), but nothing checks that the tally at the top of
the document still matches the markers below it. `spec-trace` is green either way,
and `publication-and-positioning`'s clause-ledger audit inherits a document that
counts itself wrongly. The control is arithmetic performed and recorded in this
story rather than trusted to have followed the edit two stories earlier — and it
is worth stating that this is a **reading** control with no machine behind it,
because pretending otherwise is the same failure one level up.

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
