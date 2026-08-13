---
item: HS-S0047
stage: spec
created: 2026-08-12T13:46:45.590Z
updated: 2026-08-12T13:46:45.590Z
template_sig: 87bbf1d0
rendered_sig: 9f2ea73d
---

# Spec — The standing reconciliation criterion, discharged

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DoD 12, DoD 13, BR-06 |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/sqlite-durable-store/project.md` — AC-016, DR-07, *Risks* |
| This spec | `.bklg/from-contract-to-published-library/sqlite-durable-store/spec-and-code-reconciliation/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md` — architecture brief §1 (the `spec/SPECIFICATION.md` mount row), testing brief §2 (AC-016's tier row) |
| Signed-off design | `.bklg/from-contract-to-published-library/sqlite-durable-store/_design.md` — **no user-facing surface**, approved as such; this story renders none |
| Story map row | `.bklg/from-contract-to-published-library/sqlite-durable-store/_storymap.md` — slice `publishable-and-reconciled` |
| Roadmap pointer | `RUNBOOK.md:3806-3820` (the standing criterion), `RUNBOOK.md:4166-4237` (phase 8 in full) |

## One-line PR slice

Discharge the standing reconciliation criterion: read every clause ADR-0022 discharges against the
code as it now stands, compare the phase's clause range against the union of its ADRs' ranges, and
show `spec-trace`'s citation count has not fallen.

## Executive summary

Nine of the ten places `spec/SPECIFICATION.md` names `crates/happenstance-sqlite` describe a crate
that no longer exists in that form. Four of them cite
`crates/happenstance-sqlite/src/event_store.rs:47-53` — the schema sketch the runbook itself calls
wrong (`RUNBOOK.md:4178-4187`) and that `schema-migration-and-identity` has since replaced. One
states a *census*: that two of five projection stores are `todo!()` throughout and
`SqliteProjectionStore` carries real bodies "in `begin` and `rollback`"
(`spec/SPECIFICATION.md:372`), which stops being true the moment
`projection-store-passes-the-borrowed-suite` lands. One names `happenstance-sqlite` as the
*instrument* whose arrival would falsify a `[PROVISIONAL]` marker
(`spec/SPECIFICATION.md:4388-4394`) — the instrument has now arrived, so the marker owes a verdict
rather than a wait. And one describes the read path in the conditional
(`spec/SPECIFICATION.md:2944-2947`: "`spawn_blocking` panics with no runtime in scope"), which is a
statement about a `todo!()` body, not about the store ADR-0022's runtime seam produced.

This PR is the pass that reads those back, repairs what is a repair, escalates what is a gap, and
records two numbers that nothing in this repository has ever computed: the phase's clause range and
the union of its ADRs' clause ranges. It writes no Rust. Its delta over the project charter is that
AC-016 stops being a sentence in `RUNBOOK.md` stated three times with zero enforcement
(`.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md:49-54`) and becomes a discharged,
cited, ledgered obligation for exactly one phase — without deciding, in passing, whether it should
have been a gate step all along.

## Context pack

The load-bearing decisions this story must honour. Read this before opening anything.

**1. The criterion is three bullets, and they are not interchangeable.**
`RUNBOOK.md:3810-3820` states them, and the first is the one that cannot be mechanised: *every
clause the phase's ADRs discharge has been read against the code as it now stands, not as it stood
when the clause was written — a clause whose supporting prose describes a superseded implementation
is a defect even when its MUST is untouched.* The second is arithmetic: the phase's clause range and
the union of its ADRs' clause ranges are computed and compared. The third is a floor:
`cargo xtask spec-trace`'s citation count has not fallen, and any clause the phase froze names a rule
that exists or is marked `†`. A pass that runs `spec-trace`, sees green, and ticks all three has
discharged one of them.

**2. A `[FROZEN]` clause may be repaired; it may not be amended.** The discriminator is mechanical
and is not this story's to invent — `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`
settles it: *a correction to a FROZEN clause is a repair if the set of implementations the clause
admits is unchanged, and otherwise it is a gap, which is an ADR's.* The safe form for an obligation
that has been met is to keep the MUST verbatim, name the discharge **as** a discharge, and cite the
code and the test that assert it. The safe form for a gap is to record the defect inside the clause
it is about and change nothing normative. `CLAUDE.md` says the same thing from the other side:
changing a `[FROZEN]` clause requires a new ADR, not an edit. If this pass finds one, it stops and
escalates — it does not write the ADR (`project.md` *Out of scope*: "Amending anything `[FROZEN]`").

**3. `spec-trace` measures two different things and only one of them is a count.** The summary line
is emitted at `xtask/src/spec_trace.rs:995-1007` as `N citations checked (M anchored to their
subject, K external)`. `checked` is coverage — how many `file:line` citations the tool looked at;
`anchored` is the windowed subject match at `xtask/src/spec_trace.rs:374-390`, twelve lines of slack
before a citation is called drift. Deleting a citation to silence a failure lowers `checked` and the
criterion catches it; *re-pointing* a citation at a line whose subject no longer matches lowers
`anchored` and only that second number catches it. Both are recorded, both are compared. The design
reasoning behind the windowed match — and why a content hash and an exact line match were both
rejected — is `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`; the same playbook's
closing rule is the one this story must not break: *a heuristic that cannot tell its own mistakes
from the corpus's must decline rather than guess.*

**4. Two of `spec-trace`'s regions are hand-authored and one is generated, and confusing them
breaks the run.** §1.3's census sentence (`spec/SPECIFICATION.md:219`) is a *hand* count, checked
against the run at `xtask/src/spec_trace.rs:447-471` precisely because it is not generated — if this
pass moves any clause's maturity marker, that sentence is edited by hand in the same commit or the
gate reds. §7.1 and §7.2 are generated: the remedy for a wrong row there "is never *edit the row* —
it is always `--write`" (`xtask/src/spec_trace.rs:1317`), i.e. `cargo xtask spec-trace --write`.
Hand-editing the generated region and hand-computing the census are the two symmetrical ways to get
this wrong.

**5. The phase's clause range is stated three times in the tree and the three disagree.** That is the
finding the second bullet exists to produce, and it is available before any code is read:
`RUNBOOK.md:4175-4176` says phase 8 "Discharges CF-14 and CF-17"; the clause-ownership table at
`RUNBOOK.md:605-606` assigns **CF-17, CF-34** and, in the portfolio row, **ES-35** to phase 8; the
approved intake adds **VT-21 – VT-24** and CF-14
(`.bklg/from-contract-to-published-library/sqlite-durable-store/_intake-brief.md:76-92`). Against
that, ADR-0022's own row in the ADR queue (`RUNBOOK.md:301`) carries **no parenthesised clause range
at all** — alone among the queue's rows. Phase 4 is the precedent for what that costs: its queue rows
named 35 clauses and its body discharged 64, and the 29 missing were found by a hand audit because
"a clause owned by no ADR is invisible in exactly the way a clause that is already finished is
invisible" (`RUNBOOK.md:315-322`). The comparison this story computes is therefore *the union of
what ADR-0022 says it discharges* against *phase 8's stated range*, both enumerated by ID, with a
disposition for every element of the symmetric difference.

**6. The durability verdicts are read in, not decided here.** CF-17's rule shape, ES-35's
frozen-or-restated marker and CF-14's confirm-or-withdraw are AC-010, owned by
`reopen-negative-control-and-durability-verdicts` — a hard dependency of this story. This pass
transcribes those verdicts into the specification and checks that each surviving `[PROVISIONAL]` /
`[DEFERRED]` marker still names a falsifier long enough to satisfy CF-38's build failure
(`xtask/src/spec_trace.rs:659-662`, twelve characters). It does not re-open them, and if a verdict
is missing the story is blocked, not creative.

**7. This story executes the open question; it does not answer it.**
`.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md` is an accepted atom whose five
ordered sub-questions — checkbox versus gate step, where a citation baseline would live, whether
clause ranges become machine-readable, what a disagreement obliges, whether it needs its own ADR —
are deliberately open. Running the criterion once by hand is not an answer to any of them, and
committing a baseline number into the tree would silently answer sub-question 2 against the standing
rule at `.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md` about writing a count
beside the list it counts. Findings that argue for mechanisation are **staged** for the knowledge
base and escalated to the runbook's ADR queue; accepted atoms are immutable and are authored by
`/redkiln:kb-ingest`, never by hand (`CLAUDE.md`, *Where the work lives*).

**8. The persona slice.** The reader here is the maintainer of the *next* phase — phases 9, 10 and
12 all inherit whatever this pass leaves — and, at first publish, a downstream consumer for whom the
specification stops being an internal document and becomes a promise
(`.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md:92-94`). The observable outcome
they need is not "spec-trace is green": it is that a clause they read tells them the truth about the
code they are looking at, and that where it cannot, the clause says so in its own body rather than
leaving them to discover it.

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `capability` — the deliverable is a document a human reads, and it is delivered mounted in the gate that checks it, not as a side note |
| **Slice / milestone** | `publishable-and-reconciled`; slice-mates `instrument-markers-removed-and-gate-green`, `crates-io-name-and-packaging-facts` |
| **Mount point** | **`spec/SPECIFICATION.md`** — the architecture brief's §1 mount table names it explicitly, wired to `cargo xtask spec-trace` as a gate step (`.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md:88`) |
| **Wires into** | `xtask/src/spec_trace.rs` (the nine checks, the summary line, `UNCLAIMED_PENDING_ADR`); `.redkiln/config.yaml:48` (`reachability_static`, which runs `cargo xtask lints && cargo xtask spec-trace` at story grain); `crates/happenstance-testkit/src/suite.rs` (the rule names clauses cite — `spec-trace` reads only this file); `crates/happenstance-sqlite/src/**` (read-only: the code the clauses are read *against*); `spec/E2E-CASES.md` (the case IDs clauses cite); `.kb/decisions/0022-*.md` and its long record under `references/adr/` (authored upstream by `adr-0022-append-condition-strategy`; **does not exist at spec-authoring time**) |
| **Renders surfaces** | **none** — `_design.md` records "no user-facing surface" and that determination is signed off (`_design.md:84-89`) |
| **Conformance rule(s)** | **none, and this is not adapter-observable.** The story adds no rule and changes no rule. Its check 4/check 6 obligation is the inverse: every rule a repaired clause *names* must exist in `suite.rs` or be marked `†`, and no rule may be claimed by a clause while it also sits in `UNCLAIMED_PENDING_ADR` (`xtask/src/spec_trace.rs:780-795`) |
| **Clause(s)** | Discharges the reconciliation obligation over phase 8's clause set — **CF-14, CF-17, CF-34, ES-35, VT-21 – VT-24** — plus every clause whose *prose* cites `crates/happenstance-sqlite` (`spec/SPECIFICATION.md:372, 2552, 2642, 2945, 3326, 3841, 3958, 4393, 4586, 4604, 7218, 7285`). It **amends nothing `[FROZEN]`**: a finding that would requires a new ADR and a re-plan, per `project.md` *Out of scope* |
| **Advances DoD scenario** | **DoD 12 — "the clause ledger is audited at publish"**: this is the phase-local pass that makes that audit read decisions rather than silences (project `DR-07`). Secondarily **DoD 13**, whose gate explicitly includes "the specification cross-reference step" |

This story is delivered mounted: the deliverable is not a report about
`spec/SPECIFICATION.md`, it is the edited `spec/SPECIFICATION.md` plus a green
`cargo xtask spec-trace` run over it in the same commit.

## PR boundary

**In this PR**

- Clause-body repairs in `spec/SPECIFICATION.md` — supporting prose, `Rejects:` lines, `Cases:`
  lines and `file:line` citations read back against the tree as it now stands.
- Citation re-anchoring for every `crates/happenstance-sqlite` reference the earlier stories moved.
- The §1.3 hand census, edited by hand **only if** a marker moved, so it still agrees with the run.
- The §7.1/§7.2 generated region, regenerated with `cargo xtask spec-trace --write` and never
  hand-edited.
- Narrow citation repair in `spec/E2E-CASES.md` where a case's own pointer into the SQLite crate
  drifted.
- A `_reconciliation.md` companion in this story's folder carrying the clause-by-clause verdict
  table and the two computed clause-range sets, and the story's `_ledger.md`.
- A staged intake document under `.kb/_intake/` recording what the manual pass cost and what it
  found, for `/redkiln:kb-ingest` to adjudicate later.
- `xtask/src/spec_trace.rs`, **only** to remove an entry from `UNCLAIMED_PENDING_ADR` that a repaired
  clause now legitimately claims (check 6 fails on a rule that is both claimed and listed). Never to
  relax, widen or disable a check.
- The composition-root/wiring files named in the Integration contract, to the extent mounting this
  slice requires it — that is not scope drift.

**Explicitly not in this PR**

- **Any Rust behaviour change.** No `crates/**/src/**` edit. The last `todo!()` and the scoped
  `#![allow(clippy::todo)]` belong to `instrument-markers-removed-and-gate-green`, and so does the
  crate's known-wrong module-doc sketch at `crates/happenstance-sqlite/src/lib.rs:1-24`.
- **Deciding CF-14, CF-17 or ES-35.** Those verdicts arrive from
  `reopen-negative-control-and-durability-verdicts`; this pass transcribes and checks them.
- **Amending any `[FROZEN]` clause**, adding a clause ID, or retiring one.
- **Answering `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md`** — no committed
  citation baseline, no new gate check, no machine-readable clause-range format.
- **Editing `RUNBOOK.md`** — ticking phase 8's boxes, the session log and the ADR queue's rows are
  the runbook owner's, not a side effect of this story.
- **Hand-writing or hand-editing any `.kb/` atom.** Accepted atoms are immutable and `.kb/` is
  `/redkiln:kb-ingest`'s to write; this story stages input for it.
- **New E2E cases** for the durability half of E2E-07, E2E-46's precondition or E2E-08 against a real
  second handle (`RUNBOOK.md:4231-4233`). Case *authoring* is phase 8 work outside this slice; only
  citation repair to existing cases is in boundary.
- Packaging facts and the crates.io reservation — `crates-io-name-and-packaging-facts`.

```
spec/SPECIFICATION.md
spec/E2E-CASES.md
xtask/src/spec_trace.rs
.kb/_intake/**
.bklg/from-contract-to-published-library/sqlite-durable-store/spec-and-code-reconciliation/**
```

**Merge DoD.** `cargo xtask spec-trace` green on the merged tree, its citation `checked` and
`anchored` figures both at or above the recorded pre-slice baseline, every clause in phase 8's
computed range carrying a verdict in `_reconciliation.md`, and no `[FROZEN]` clause amended.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **Every clause ADR-0022 discharges is read against the code as it now stands** | For each ID in the computed phase-8 range, the clause's MUST **and its supporting prose** are read against the merged tree. The verdict is one of *unchanged*, *repaired* (prose/citation corrected, admitted-implementation set identical), or *gap* (escalated, nothing normative changed). A clause whose MUST is untouched but whose prose describes a superseded implementation is a **defect**, not a pass. | `RUNBOOK.md:3810-3814`; `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` |
| **The repair/gap discriminator is applied, not improvised** | A correction is a repair iff the set of implementations the clause admits is unchanged; otherwise it is a gap and belongs to an ADR. A met obligation keeps its MUST verbatim, is named as a discharge, and cites the code and the test that assert it. | `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`; `CLAUDE.md` *Open questions* |
| **The stale schema-sketch citations are re-anchored or repaired** | Four clauses cite `crates/happenstance-sqlite/src/event_store.rs:47-53` as "the planned SQLite schema" that puts tags in a side table. ADR-0022 amends that schema (`event_type` as a covering column, `tag_cardinality`) and `schema-migration-and-identity` lands it, so both the line range and the word *planned* are read back. | `spec/SPECIFICATION.md:3326, 3841, 7218, 7285`; `RUNBOOK.md:4178-4187` |
| **The projection-store census sentence stops being false** | §5's port table states "**Two** of the five are `todo!()` throughout" and that `SqliteProjectionStore` carries real bodies "in `begin` and `rollback`" only. After `projection-store-passes-the-borrowed-suite`, both halves are wrong, and the sentence exists to justify a stated *count* — the exact failure mode a hand-maintained count beside a list has. | `spec/SPECIFICATION.md:372`; `.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md` |
| **A `[PROVISIONAL]` marker naming this crate as its instrument gets a verdict** | The membership clause's falsifier reads "The instruments are `happenstance-sqlite` at phase 8 and `happenstance-cloudflare` at phase 9, whichever lands first." The named instrument has landed. The marker is either restated with what is *still* missing or the clause is escalated — a falsifier that names an event which has already happened is CF-38's failure mode in slow motion. | `spec/SPECIFICATION.md:4388-4394`; `xtask/src/spec_trace.rs:659-662` |
| **The read-path prose is read against the settled runtime seam** | D7's paragraph explains `happenstance-sqlite`'s conformance by "`spawn_blocking` panics with no runtime in scope" — a claim about a `todo!()` skeleton. ADR-0022 settles the runtime seam (architecture brief AC-A02) and `lazy-read-with-snapshot-ceiling` lands the ceiling; the prose is checked against what the store now does. The obligation itself (ceiling captured no later than the first poll) is ADR-0011's and is untouched. | `spec/SPECIFICATION.md:2942-2951`; `.kb/decisions/0011-read-laziness-and-isolation.md` |
| **The phase's clause range and the union of its ADRs' ranges are computed and compared** | Both are written down as enumerated ID sets in `_reconciliation.md`, not as prose. Phase-8 side: `RUNBOOK.md:4175-4176`, `RUNBOOK.md:605-606`, `_intake-brief.md:76-92`. ADR side: whatever the landed ADR-0022 atom states it discharges — its queue row carries no range at all. Every element of the symmetric difference gets a disposition: *already discharged elsewhere*, *owed by ADR-0022*, or *escalated*. | `RUNBOOK.md:301`, `RUNBOOK.md:3815-3816`, `RUNBOOK.md:315-322` |
| **The citation figures do not fall** | `cargo xtask spec-trace` prints `N citations checked (M anchored to their subject[, K external])`. The pre-slice figures are captured from the merge-base and recorded; the post figures are captured from the merged tree. Both `checked` and `anchored` are compared. A drop in either is a finding, and deleting a citation is not a remedy. | `xtask/src/spec_trace.rs:995-1007`, `:374-390` |
| **A clause this phase froze names a rule that exists or is marked `†`** | Check 4 resolves cited rule names against `suite.rs` plus the `wire::`-qualified tests; check 6 refuses a rule that is claimed by a clause *and* listed in `UNCLAIMED_PENDING_ADR`. Both are the tool's, and the story's obligation is not to create a violation while repairing prose. | `xtask/src/spec_trace.rs:632-646`, `:780-795`; `RUNBOOK.md:3818-3820` |
| **The hand census and the generated region are treated differently** | §1.3's count (`spec/SPECIFICATION.md:219`) is deliberately hand-maintained and is checked against the run; if a marker moved, it is edited by hand in the same commit. §7.1/§7.2 are regenerated with `cargo xtask spec-trace --write` and never hand-edited. | `xtask/src/spec_trace.rs:447-471`, `:1317`; `xtask/src/main.rs:671-673` |
| **Nothing `[FROZEN]` is amended and nothing open is settled** | A finding that would change what a frozen clause admits is recorded and escalated to the runbook's ADR queue; the reconciliation open question is left open, with the pass's cost and findings staged under `.kb/_intake/` for `/redkiln:kb-ingest`. | `project.md` *Out of scope*; `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md:96-120` |
| **Verification command** | `cargo xtask spec-trace` and the story-grain gate `cargo xtask affected --base main`, which runs the five file-reading lints and `spec-trace` unconditionally — the config comments note that a story whose whole deliverable is an edit to `SPECIFICATION.md` maps to no package, which is exactly this one. | `.redkiln/config.yaml:36-48` |

## Data and migrations

**N/A.** This story writes no Rust, opens no connection and touches no schema. The SQLite schema and
its migration 1 are `schema-migration-and-identity`'s, landed several stories upstream and consumed
here only as the *subject* a clause's prose is read against; `crates/**/src/**` is outside this
story's PR boundary in both directions. The only persistent artifacts it produces are markdown: the
repaired `spec/SPECIFICATION.md`, the `_reconciliation.md` verdict table in this story's folder, and
a staged document under `.kb/_intake/` that `/redkiln:kb-ingest` adjudicates on its own schedule.

## Acceptance criteria

The persona throughout is the one the initiative charter and
`.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md:92-94` name: **the maintainer of
the next phase** — phases 9, 10 and 12 all inherit whatever this pass leaves — and, at first
publish, **a downstream consumer** for whom `spec/SPECIFICATION.md` stops being an internal document
and becomes a promise. Their goal is not "the gate is green". It is: *the clause I am reading tells
me the truth about the code I am looking at, and where it cannot, it says so in its own body rather
than leaving me to find out.*

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** the maintainer of phase 9 opens any clause in phase 8's computed range, **WHEN** they read its MUST *and* its supporting prose against the merged tree, **THEN** every one of those clauses carries an explicit verdict — `unchanged`, `repaired`, or `gap` — in `_reconciliation.md`, and a clause whose MUST is untouched but whose prose describes a superseded implementation is recorded as a **defect** and not as a pass. No clause in the range is absent from the table, and none carries a verdict of "looks fine". | Review tier against `.bklg/from-contract-to-published-library/sqlite-durable-store/spec-and-code-reconciliation/_reconciliation.md`: one row per clause ID in the AC-004 range, each row citing the `spec/SPECIFICATION.md` line it read and the `crates/happenstance-sqlite/src/**` line it read it against. Cross-checked by `cargo xtask spec-trace` green on the merged tree (`.redkiln/config.yaml:48`, `reachability_static`). Criterion source `RUNBOOK.md:3810-3814`. |
| **AC-002** | **GIVEN** a reader following a clause's own `file:line` pointer into `crates/happenstance-sqlite`, **WHEN** they land, **THEN** they find the subject the clause said they would: the four schema-sketch citations no longer point at the sketch the runbook itself calls wrong, §5's projection-store census sentence states the count that is now true, the `[PROVISIONAL]` marker naming this crate as its instrument no longer waits for an event that has already happened, and D7's read-path paragraph no longer explains conformance by the behaviour of a `todo!()` body. Every drifted pointer is **re-anchored or repaired** — none is deleted. | Static tier: `cargo xtask spec-trace` green, with the windowed subject match (`xtask/src/spec_trace.rs:374-390`) passing for every repaired citation; `anchored` figure compared per AC-005. Review tier: `_reconciliation.md` names each of `spec/SPECIFICATION.md:372, 2552, 2642, 2945, 3326, 3841, 3958, 4393, 4586, 4604, 7218, 7285` with its disposition, plus any `spec/E2E-CASES.md` pointer repaired alongside. |
| **AC-003** | **GIVEN** a reviewer who must be able to trust that this pass changed no promise, **WHEN** they diff `spec/SPECIFICATION.md`, **THEN** every edit is a **repair** — the set of implementations the clause admits is unchanged — and any finding that would change what a `[FROZEN]` clause admits appears as a recorded, escalated **gap** inside the clause it is about, with nothing normative altered and no ADR written here. A met obligation keeps its MUST verbatim, is named **as** a discharge, and cites the code and the test that assert it. | Review tier against the diff plus `_reconciliation.md`'s discriminator column, applying `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` rather than an improvised rule. Mechanically: no `[FROZEN]` clause's MUST text changes in the diff (a `git diff` assertion a reviewer runs), and every `gap` row names the runbook ADR-queue escalation it produced. Criterion source `project.md` *Out of scope*; `CLAUDE.md` *Open questions*. |
| **AC-004** | **GIVEN** the phase-4 precedent, where queue rows named 35 clauses and the body discharged 64 and the 29 missing were invisible in exactly the way a finished clause is invisible (`RUNBOOK.md:315-322`), **WHEN** phase 8 closes, **THEN** two enumerated ID sets exist side by side — the phase's stated clause range and the union of its ADRs' ranges — and **every element of the symmetric difference carries a disposition**: *already discharged elsewhere*, *owed by ADR-0022*, or *escalated*. The three disagreeing statements of the phase's range (`RUNBOOK.md:4175-4176`, `RUNBOOK.md:605-606`, `_intake-brief.md:76-92`) are reconciled into one set rather than averaged, and ADR-0022's queue row carrying no parenthesised range at all (`RUNBOOK.md:301`) is itself recorded as a finding. | Review tier against `_reconciliation.md` §*Clause ranges*: two literal ID sets (not prose), their symmetric difference computed, one disposition row per element. Criterion source `RUNBOOK.md:3815-3816`. Blocked, not guessed, if the landed ADR-0022 atom under `.kb/decisions/` states no range — the absence is the finding. |
| **AC-005** | **GIVEN** a consumer who needs to know the specification's coverage did not shrink to buy its greenness, **WHEN** the slice merges, **THEN** `cargo xtask spec-trace` is green and **both** figures in its summary line — `N citations checked` and `M anchored to their subject` — are at or above the figures recorded from the pre-slice merge-base. Deleting a citation to silence a failure is not a remedy (it lowers `checked`); re-pointing one at a line whose subject no longer matches is not a remedy either (it lowers `anchored`). §1.3's hand census is edited by hand in the same commit if any maturity marker moved; §7.1/§7.2 are regenerated with `cargo xtask spec-trace --write` and never hand-edited. | Static tier: `cargo xtask spec-trace` (`xtask/src/spec_trace.rs:995-1007` emits the compared line); the census check at `xtask/src/spec_trace.rs:447-471` reds if the hand count and the run disagree. Story grain `cargo xtask affected --base main` (`.redkiln/config.yaml:40`), which runs the five file-reading lints and `spec-trace` unconditionally — the reason a spec-only story is gated at all. Both baselines recorded in `_reconciliation.md` against a named merge-base sha. Criterion source `RUNBOOK.md:3817-3820`. |
| **AC-006** | **GIVEN** `publication-and-positioning`'s clause-ledger audit must read a *decision* rather than a silence (project `DR-07`), **WHEN** it opens CF-14, CF-17 and ES-35, **THEN** each carries the verdict `reopen-negative-control-and-durability-verdicts` produced, transcribed here without being re-opened, and every surviving `[PROVISIONAL]` / `[DEFERRED]` marker names a falsifier that is still *live* and long enough to satisfy CF-38's build failure. The reconciliation open question is left **open**: no committed citation baseline, no new gate check, no machine-readable clause-range format — findings that argue for mechanisation are staged under `.kb/_intake/` for `/redkiln:kb-ingest` to adjudicate, never hand-written into `.kb/`. | Static tier: `cargo xtask spec-trace`'s falsifier-length check (`xtask/src/spec_trace.rs:659-662`, twelve characters) plus the census agreement above. Review tier: each of CF-14/CF-17/ES-35 traced in `_reconciliation.md` to the upstream story's ledger row; `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md` unedited in the diff; the staged intake document present under `.kb/_intake/` and no atom under `.kb/decisions/`, `.kb/concepts/` or `.kb/playbooks/` touched. Backlog check: `redkiln validate --kb && redkiln doctor` clean. |

**Coverage of the traced project AC.** All six rows together discharge **AC-016** — *the standing
criterion at `RUNBOOK.md:3810-3820` is discharged: every clause ADR-0022 discharges is read against
the code as it now stands (AC-001, AC-002, AC-003), the phase's clause range and the union of its
ADRs' ranges are computed and compared (AC-004), and `cargo xtask spec-trace`'s citation count has
not fallen (AC-005)*. AC-006 is AC-016's `DR-07` half: the criterion is discharged *without* the
pass silently settling what it was only asked to run. No other project AC is claimed here.

## Interaction quality

**Composition invariants: N/A, and that is signed off.** This story renders **no user-facing
surface**, and the project's approved design says so explicitly rather than leaving it a silent skip
(`.bklg/from-contract-to-published-library/sqlite-durable-store/_design.md` — *Anti-patterns*, *The
doctest*, *Sign-off* all read "N/A — no user-facing surface", approved 2026-08-12). There is no
composition, transience policy, density budget or hierarchy to honour, and no mock was produced or
owed. Inventing composition ACs here would be inventing a surface the human already declined.

**State invariants do apply, in this story's medium.** The artifact a human reads is
`spec/SPECIFICATION.md`, and the state invariants have exact analogues in it. Each is carried by an
AC row in the table above — none is stated only here, because `redkiln verify` extracts ACs from
table cells and bullets in that section, and an invariant living in this section alone would never
be gated.

| Invariant | Analogue in this story's medium | Carried by | Verified by |
| --- | --- | --- | --- |
| **In-place, not a context jump** | A correction is made *inside the clause it is about*, not in an errata list, a companion document or a commit message a reader will never see. A gap is recorded in the clause's own body. | AC-003 | Diff review: every gap disposition in `_reconciliation.md` has a corresponding in-body note in `spec/SPECIFICATION.md` |
| **Non-occlusion** | A repair may not hide the defect it repairs. Deleting a drifted citation, or softening prose until it is merely un-falsifiable, occludes the finding that the pass existed to surface. | AC-002, AC-005 | `checked`/`anchored` both compared, not just green (`xtask/src/spec_trace.rs:995-1007`) |
| **Preserved selection / identity** | Clause IDs, MUST text and maturity markers are the reader's stable handles across versions. Re-anchoring a citation preserves all three; a marker only moves when a landed verdict moves it, and then the §1.3 census moves with it in the same commit. | AC-003, AC-005 | `xtask/src/spec_trace.rs:447-471` reds on a census/run disagreement |
| **Reversibility** | Nothing this pass does costs a decision that would need an ADR to undo. Every edit is a repair whose admitted-implementation set is unchanged; anything else is escalated rather than applied. | AC-003, AC-006 | `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`'s discriminator, applied per row |
| **Reachability (the keyboard analogue)** | Every claim a reader wants to check is reachable by following a pointer that resolves — a rule name that exists in `crates/happenstance-testkit/src/suite.rs` or is marked `†`, a `file:line` that lands on its subject, a falsifier that names something that has not already happened. | AC-002, AC-006 | `spec-trace` checks 4 and 6 (`xtask/src/spec_trace.rs:632-646`, `:780-795`), falsifier length (`:659-662`) |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | A finding would change the set of implementations a `[FROZEN]` clause admits. | **Stop.** Record the defect inside the clause it is about, change nothing normative, escalate to the runbook's ADR queue, and mark the row `gap` in `_reconciliation.md`. Do **not** write the ADR here (`project.md` *Out of scope*; `CLAUDE.md` *Open questions*). The story may merge with open gaps; it may not merge with a silently amended clause. |
| **EC-002** | A durability verdict this pass must transcribe (CF-14, CF-17 or ES-35) has not arrived from `reopen-negative-control-and-durability-verdicts`. | **Blocked, not creative.** The story does not invent, infer or "provisionally record" the verdict. It halts and reports the missing upstream ledger row — this is a declared hard dependency, not a soft ordering preference. |
| **EC-003** | `cargo xtask spec-trace` reds after an edit because a citation no longer resolves. | Repair the citation to point at its real current subject. **Never** delete it: deletion lowers `checked` and is precisely the remedy the third bullet of the criterion exists to forbid (`RUNBOOK.md:3817-3818`). |
| **EC-004** | `checked` holds at baseline but `anchored` falls. | A citation was re-pointed at a line whose subject no longer matches within the twelve-line window (`xtask/src/spec_trace.rs:374-390`). Treat as an unrepaired drift finding, not as noise, and re-anchor properly. Widening the window is out of boundary. |
| **EC-005** | Check 6 fails: a repaired clause now legitimately claims a rule that is still listed in `UNCLAIMED_PENDING_ADR`. | Remove **that one entry** from `xtask/src/spec_trace.rs` — the single permitted edit to the tool in this PR (`xtask/src/spec_trace.rs:780-795`). Never relax, widen or disable a check to make a clause pass. |
| **EC-006** | A maturity marker moved and §1.3's hand census was not updated. | The gate reds at `xtask/src/spec_trace.rs:447-471`. Edit the census by hand in the same commit; it is deliberately hand-maintained and is checked against the run precisely because it is not generated. |
| **EC-007** | A row in §7.1 or §7.2 is wrong. | The remedy "is never *edit the row* — it is always `--write`" (`xtask/src/spec_trace.rs:1317`). Regenerate with `cargo xtask spec-trace --write`. Hand-editing the generated region and hand-computing the census are the two symmetrical ways to get this wrong. |
| **EC-008** | The landed ADR-0022 atom states no clause range, or its range disagrees with all three of the phase's statements. | Record the absence or the disagreement as the finding — the phase-4 precedent (`RUNBOOK.md:315-322`) is that an unowned clause is invisible in exactly the way a finished one is. Do not synthesise a range to make the arithmetic close. Every element of the symmetric difference still gets a disposition (AC-004). |
| **EC-009** | The pass produces a compelling argument for mechanising the criterion (a committed baseline, a gate step, a machine-readable range). | Stage it under `.kb/_intake/` and escalate to the runbook's ADR queue. Do **not** implement it, and do **not** commit a baseline number beside the list it counts — that would answer sub-question 2 of an open atom in passing, against `.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md`. |
| **EC-010** | A clause in the computed range cannot be given a verdict because the code it describes is itself still in motion. | Record `blocked` with the story slug that owns the motion, and do not close the story on a partial table. AC-001 requires a verdict for every clause in the range; `blocked` is a verdict only when it names its blocker. |

## Non-functional

| id | requirement | why it is here |
| --- | --- | --- |
| **NF-001** | **Zero Rust behaviour change.** The diff contains no `crates/**/src/**` edit; the only `xtask` edit permitted is EC-005's single `UNCLAIMED_PENDING_ADR` removal. | The slice-mate `instrument-markers-removed-and-gate-green` owns the last `todo!()` and the crate's module-doc sketch. A behaviour change smuggled in here would be invisible to both stories' ledgers. |
| **NF-002** | **The baselines are reproducible.** The pre-slice `checked`/`anchored` figures are captured from a **named merge-base sha**, recorded in `_reconciliation.md`, and re-derivable by a reviewer running the same command at that sha. | A comparison whose "before" number cannot be recomputed is an assertion, not a measurement — the same failure the criterion's third bullet exists to catch. |
| **NF-003** | **No number is committed as a standing baseline.** The figures live in this story's `_reconciliation.md`, not in `spec/SPECIFICATION.md`, not in `xtask/`, and not in any gate script. | Committing one silently answers sub-question 2 of `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md`. NF-002 and NF-003 are not in tension: recorded-in-the-story is reproducible without being normative. |
| **NF-004** | **The pass's cost is recorded**: how many clauses were read, how long the manual read took, and which findings only a human read could have produced. Staged under `.kb/_intake/`. | The open question's central trade is checkbox-versus-gate-step, and it cannot be adjudicated later without knowing what running it by hand actually costs. This is input to that decision, not the decision. |
| **NF-005** | **The backlog and knowledge base stay clean**: `redkiln validate --kb && redkiln doctor` passes with exactly the six expected `template-drift` advisories and no seventh. | `CLAUDE.md` *Commands*; a seventh advisory means a template changed without anyone deciding to. |
| **NF-006** | **No new gate step and no widened check.** `spec-trace`'s nine checks, its window and its summary are untouched apart from EC-005. | A story that changes the instrument it is measured by has measured nothing. |

## Implementation notes (non-prescriptive)

Ordering matters more here than technique, and one ordering mistake is unrecoverable without a
rebase.

- **Capture the baseline first, from the merge-base, before touching anything.** Run
  `cargo xtask spec-trace` at the merge-base sha and write both figures down (NF-002). After the
  first edit the "before" number no longer exists in the working tree.
- **Enumerate before you read.** Build AC-004's two ID sets from `RUNBOOK.md:4175-4176`,
  `RUNBOOK.md:605-606`, `_intake-brief.md:76-92` and the landed ADR-0022 atom *before* opening a
  single clause. The symmetric difference tells you which clauses are the interesting ones; reading
  in document order buries them.
- **`rg` finds the drifted pointers faster than the eye does.** A search for
  `crates/happenstance-sqlite` across `spec/` returns the candidate set for AC-002 in one pass; the
  exec summary's twelve line numbers are the expected result, and a *thirteenth* hit is itself a
  finding.
- **Apply the discriminator per row and write the answer down**, even for the boring rows. The
  repair/gap question is cheap to answer at the moment you are looking at the clause and expensive
  to reconstruct afterwards — which is what makes `_reconciliation.md` a working document rather
  than a write-up produced at the end.
- **Regenerate, then census, then re-run.** `--write` first for §7.1/§7.2, then the §1.3 hand count
  if any marker moved, then a clean `cargo xtask spec-trace`. Doing the census first means doing it
  twice.
- **A verdict of `unchanged` is a real result.** The criterion is not satisfied by finding defects;
  it is satisfied by having read. Rows that say `unchanged` with a cited code line are the evidence
  that the range was covered.
- The shape of `_reconciliation.md` is the implementer's. It needs a clause-verdict table (AC-001), a
  citation-disposition list (AC-002), two ID sets with a dispositioned symmetric difference (AC-004),
  and the two baseline figures with their sha (AC-005) — how those are laid out is not prescribed.

## Tests and CI (merge gate)

Grounded in the project testing brief's §1 tier table and §6 merge-gate commands
(`.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md`), which name
AC-016's tier as **Static** and its command as `cargo xtask spec-trace`.

| tier | command / path | proves |
| --- | --- | --- |
| **Static (the story's real proof)** | `cargo xtask spec-trace` | The nine checks green on the merged tree, and the summary line's `N citations checked (M anchored to their subject)` for the post-slice comparison — AC-005. Also carries checks 4 and 6 (rule exists or `†`; no rule both claimed and `UNCLAIMED_PENDING_ADR`) — AC-002, AC-006. |
| **Static (baseline capture)** | `cargo xtask spec-trace` at the recorded merge-base sha | The "before" half of the not-fallen comparison, reproducibly — AC-005, NF-002. |
| **Static (regeneration)** | `cargo xtask spec-trace --write` | §7.1/§7.2 regenerated rather than hand-edited (`xtask/src/spec_trace.rs:1317`) — AC-005, EC-007. |
| **Static (census agreement)** | the same run, via `xtask/src/spec_trace.rs:447-471` | §1.3's hand count agrees with the run after any marker move — AC-005, EC-006. |
| **Story grain (the gate that fires)** | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | That this spec-only story is gated at all: the command runs the five file-reading lints and `spec-trace` unconditionally because "a story whose whole deliverable is an edit to SPECIFICATION.md maps to no package" (`.redkiln/config.yaml:36-40`). This is that story. |
| **Story grain (reachability tripwire)** | `cargo xtask lints && cargo xtask spec-trace` (`reachability_static`, `.redkiln/config.yaml:48`) | A clause citing a rule that no longer exists is decoration — the library analogue of "nothing imports it" — AC-002, AC-006. |
| **Integration grain (project bar)** | `cargo xtask ci --fast` (`integration_scoped`, `.redkiln/config.yaml:55`; `project.md` DoD 5) | Nothing this pass did broke the tree the slice-mates just took green; NF-001's zero-Rust-change claim shows up here as an unchanged compile. |
| **Review tier** | `.bklg/from-contract-to-published-library/sqlite-durable-store/spec-and-code-reconciliation/_reconciliation.md` | AC-001's per-clause verdicts, AC-003's discriminator column, AC-004's two ID sets and dispositioned symmetric difference. Not a runnable test, and named here so it is not silently skipped — the same treatment the testing brief §2 gives AC-010 and AC-013. |
| **Backlog / KB** | `redkiln validate --kb && redkiln doctor` | `.kb/` untouched apart from the staged `_intake/` document; six `template-drift` advisories and no seventh — AC-006, NF-005. |
| **Ledger** | `_ledger.md` with `require_ledger: true` (`.redkiln/config.yaml:62-67`) | Every AC-001 – AC-006 carries cited evidence; a green gate is a precondition for looking at the criteria, never a substitute (`RUNBOOK.md:38-42`; testing brief AC-T05). |

**Not run here:** the whole `cargo xtask ci`, feature powersets, `cargo deny`, docsrs and MSRV —
`closeout-and-durable-audience` (HS-P0019) owns those (`.redkiln/config.yaml:60`). No benchmark and
no conformance macro is added or removed by this story; the rule count is unchanged by construction.

## Risks and coupling (PR-scoped)

- **The baseline is capturable exactly once, and only before the first edit.** Forget it and the
  criterion's third bullet degrades to "spec-trace is green" — the precise degradation the runbook
  warns about. Mitigation: NF-002's named sha, captured as the story's first action.
- **Both slice-mates land in the same slice, and this story reads what they wrote.**
  `instrument-markers-removed-and-gate-green` changes the crate's module-doc sketch and deletes the
  last `todo!()`; `reopen-negative-control-and-durability-verdicts` produces the CF-14/CF-17/ES-35
  verdicts. Reading clauses against a tree where either has not landed produces a table that is wrong
  in a way no gate can see. Mitigation: both are hard `depends_on`, and EC-002 blocks rather than
  improvises.
- **ADR-0022's queue row carries no clause range, alone among the queue's rows** (`RUNBOOK.md:301`).
  AC-004's arithmetic depends on the landed atom supplying one. If it does not, the story's output is
  a *finding about the ADR* rather than a closed comparison, and that must be acceptable to the
  reviewer rather than a reason to invent a range (EC-008).
- **The most likely silent failure is a deleted citation.** It turns a red gate green, looks like a
  cleanup in a diff, and is caught only by comparing `checked` — which is why the comparison, not the
  green run, is the AC. Second most likely: a citation re-pointed to a plausible-looking line, caught
  only by `anchored` (EC-004).
- **Scope pressure toward mechanisation.** Doing this by hand is tedious, and the obvious response —
  add a gate step, commit a baseline — is exactly what the open atom reserves for a later decision.
  The pass's job is to make that decision *better informed* (NF-004), not to pre-empt it (EC-009).
- **The one permitted `xtask` edit is a slippery slope.** Removing an `UNCLAIMED_PENDING_ADR` entry
  is in boundary; widening a window, softening a check or skipping a step to make a clause pass is
  the story defeating its own instrument (NF-006).
- **CF-40's clause home is forced by this phase and is not this story's to settle**
  (`.kb/open-questions/cf-40-fixture-limits-ownership.md`). If a VT-21 – VT-24 repair drifts toward
  deciding where CF-40 lives, that is EC-001 territory: record and escalate.
- **`_design.md` records no surface, so no perceptual review will ever look at this story's output.**
  The only instrument that can catch a clause that is technically green and practically useless is a
  human reading it — which is what AC-001 asks for and why "spec-trace green" is nowhere near
  sufficient on its own.

## Dependencies

**Blocks on** (both hard; both in the same `publishable-and-reconciled` slice):

- **`instrument-markers-removed-and-gate-green`** — this story reads clauses *against the code as it
  now stands*, and "as it now stands" is only true after the last `todo!()`, the scoped
  `#![allow(clippy::todo)]` and the crate's known-wrong module-doc sketch are gone. Reading before it
  lands produces verdicts about a tree that no longer exists. It also takes `cargo xtask ci --fast`
  green, which is the tree this story must not break.
- **`reopen-negative-control-and-durability-verdicts`** — supplies the CF-14 / CF-17 / ES-35 verdicts
  (project AC-010, `DR-07`) that AC-006 transcribes. This story does not re-open them; if a verdict
  is missing it halts (EC-002).

**Unlocks**

- **`publication-and-positioning` (HS-P0016)** — its **clause-ledger audit at publish** (initiative
  DoD 12, project `DR-07`) reads the verdicts and repairs this pass leaves. It is the reason AC-006
  exists as a separate criterion rather than as a footnote to AC-001.
- The project's own closeout: AC-016 is the last of `project.md`'s sixteen criteria, and the
  `publishable-and-reconciled` slice is complete once this and
  `crates-io-name-and-packaging-facts` (its parallel slice-mate, which this story neither blocks nor
  is blocked by) are both in.

Within the slice the order is `instrument-markers-removed-and-gate-green` →
(`crates-io-name-and-packaging-facts` ‖ `spec-and-code-reconciliation`)
(`.bklg/from-contract-to-published-library/sqlite-durable-store/_storymap.md:180-184`).

## Anchors (progressive disclosure)

Linked, not pasted. Each row says why the artifact is load-bearing and the moment to open it.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `RUNBOOK.md` | The criterion itself at `:3810-3820` (three bullets, not interchangeable), phase 8 in full at `:4166-4237`, the clause-ownership rows at `:605-606`, the ADR queue row with no range at `:301`, and the phase-4 precedent at `:315-322` that explains why the arithmetic exists. | Before the first clause is read, and again when building AC-004's two ID sets. | AC-001, AC-004 |
| `spec/SPECIFICATION.md` | The mount point and the deliverable. The twelve `crates/happenstance-sqlite` prose sites, §1.3's hand census at `:219`, the §5 projection-store census at `:372`, the instrument falsifier at `:4388-4394`, D7's read-path paragraph at `:2942-2951`. | Continuously — this is the file being edited. Open `:4388-4394` and `:372` first: both are known-false and are the pass's cheapest wins. | AC-002 |
| `xtask/src/spec_trace.rs` | The instrument. The summary line at `:995-1007` (the two compared figures), the twelve-line window at `:374-390`, the census check at `:447-471`, the falsifier length at `:659-662`, checks 4 and 6 at `:632-646` and `:780-795`, and the "never edit the row — always `--write`" rule at `:1317`. | Before running the baseline, and again the first time a check reds. | AC-005, AC-006 |
| `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` | Settles the repair-versus-gap discriminator so this story does not invent one: a correction is a repair iff the set of implementations the clause admits is unchanged. Also gives the safe form for a met obligation and for a recorded gap. | Before the first `[FROZEN]` clause is edited — i.e. before any edit, since most of the range is frozen. | AC-003 |
| `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` | Why the match is windowed rather than exact or hashed, and the closing rule this story must not break: a heuristic that cannot tell its own mistakes from the corpus's must decline rather than guess. | When a citation almost matches and the temptation is to nudge the window or the line. | AC-002, AC-005 |
| `.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md` | The standing rule about writing a count beside the list it counts — which is both why §5's census sentence is wrong and why committing a citation baseline would be a mistake. | When editing the §1.3 or §5 counts, and again if mechanisation starts to look attractive. | AC-005, EC-009 |
| `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md` | The accepted atom this story *executes without answering*. Its five ordered sub-questions are the exact list of things a well-meaning implementer would settle in passing; `:49-54` is why AC-016 exists, `:92-94` is the persona, `:96-120` is what may be staged. | Before writing anything into `.kb/_intake/`, and any time the pass wants to add a check. | AC-006, NF-003, NF-004 |
| `.kb/decisions/0011-read-laziness-and-isolation.md` | The obligation behind D7's read-path prose (ceiling captured no later than the first poll). It is **untouched** by this story — the anchor exists so the implementer repairs the *explanation* without disturbing the *obligation*. | When rewriting `spec/SPECIFICATION.md:2942-2951`. | AC-002, AC-003 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/_intake-brief.md` | The approved clause list for this project (`:76-92`, adding VT-21 – VT-24 and CF-14) — one of the three disagreeing statements of phase 8's range, and the only one that went through an approval gate. | While building AC-004's phase-8 ID set. | AC-004 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md` | Architecture brief §1's mount table (the `spec/SPECIFICATION.md` row wired to `spec-trace`) and testing brief §2's AC-016 row and §6 merge-gate commands — the tier and command this story is measured by, restated so they are not reconstructed from config mid-story. | Before running any gate command; again when filling the ledger's `verifying_test`. | AC-005, AC-001 |
| `crates/happenstance-testkit/src/suite.rs` | The only file `spec-trace` reads for rule names. A repaired clause that names a rule not in here (and not marked `†`) fails check 4 — the failure mode of a well-intentioned prose repair. | When a repair names or renames a conformance rule. | AC-002, AC-006 |
| `spec/E2E-CASES.md` | The case IDs clauses cite; in boundary for *citation repair only* where a case's own pointer into the SQLite crate drifted. Authoring new cases is explicitly out of boundary. | Only if an AC-002 repair follows a clause's `Cases:` line into this file. | AC-002 |
| `.redkiln/config.yaml` | `affected_gate` at `:40` and `reachability_static` at `:48` — including the comment explaining that a story whose whole deliverable is an edit to `SPECIFICATION.md` maps to no package, which is why the file-reading lints and `spec-trace` run unconditionally. `require_ledger` at `:62-67`. | When deciding what "the gate" means for a story that compiles nothing. | AC-005 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/reopen-negative-control-and-durability-verdicts/spec.md` | Where the CF-14 / CF-17 / ES-35 verdicts are produced (project AC-010). Its ledger rows are the citation AC-006 transcribes from. | Before transcribing any durability verdict; immediately, if one appears to be missing (EC-002). | AC-006 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/instrument-markers-removed-and-gate-green/spec.md` | Defines the tree state "as it now stands" that AC-001 reads against — the last `todo!()`, the scoped allow, and the module-doc sketch correction. Also fixes what this story may not touch. | Before the first clause read, to confirm the dependency has landed and what it changed. | AC-001, NF-001 |
| `.kb/open-questions/cf-40-fixture-limits-ownership.md` | The clause-home contradiction this phase forces. Named so a VT-21 – VT-24 repair does not settle it in passing; it is record-and-escalate, not decide. | If an AC-002 repair touches VT-21 – VT-24 or the fixture-limits clause. | AC-003 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/_design.md` | The signed-off determination that this project renders **no user-facing surface** — the reason the Interaction quality section carries no composition ACs and no mock is owed. | Once, if the question "should this have a surface?" comes up. It is answered and approved. | AC-001 (framing) |

## Clarifications resolved during spec

1. **The AC set is exactly the six the front half decided** — AC-001 – AC-006. None added, none
   dropped. They partition the criterion's three bullets (AC-001/AC-002/AC-003 → bullet 1,
   AC-004 → bullet 2, AC-005 → bullet 3) plus `DR-07`'s "a decision rather than a silence"
   obligation (AC-006), which is the half of project AC-016 that a purely mechanical reading of
   `RUNBOOK.md:3810-3820` would drop.
2. **Bullet 1 is split three ways on purpose.** *Reading* the clauses (AC-001), *repairing the
   pointers* the reading finds broken (AC-002) and *not amending anything frozen while doing so*
   (AC-003) fail independently and in different ways — a single "clauses are reconciled" AC would
   be satisfied by a pass that read nothing and repaired everything, or vice versa.
3. **This story renders no surface, and the composition invariants are therefore N/A** — not
   omitted, and not silently skipped. `_design.md` records the no-surface determination and it is
   signed off (2026-08-12). The state invariants were mapped onto the document medium instead and
   are all carried by AC rows in the table, never as prose bullets in the Interaction quality
   section, so `redkiln verify` can extract every one of them.
4. **AC-005 asserts on *both* figures, not on greenness.** `checked` and `anchored` catch different
   cheats (deletion versus re-pointing) and a criterion phrased as "spec-trace is green" catches
   neither. The runbook's bullet says "citation count"; the tool emits two numbers, and reading it
   as one number would leave the more subtle failure ungated.
5. **NF-002 and NF-003 are deliberately in tension and the tension is resolved by *location*.** The
   baselines must be reproducible (so a reviewer can re-run them) but must not be committed as a
   standing number (so sub-question 2 of the open atom stays open). Recording them in the story's own
   `_reconciliation.md` against a named merge-base sha satisfies both.
6. **AC-004 can close on a finding rather than on a match.** If the landed ADR-0022 atom states no
   clause range, the criterion is satisfied by *recording that*, with a disposition for every element
   of the resulting symmetric difference — not by synthesising a range. That reading comes from the
   phase-4 precedent (`RUNBOOK.md:315-322`) and is written into EC-008 so it cannot be mistaken for
   a licence to skip the comparison.
7. **`_reconciliation.md` is a working document, not a report.** It is authored *during* the clause
   read and is the evidence AC-001, AC-003 and AC-004 are verified against. Its shape is the
   implementer's; its four required contents are named in the Implementation notes.
8. **One `xtask` edit is in boundary and it is bounded to a single line-range.** Removing an
   `UNCLAIMED_PENDING_ADR` entry that a repaired clause now legitimately claims is a *consequence*
   of a correct repair (check 6 fails otherwise). NF-006 and the PR boundary state the inverse
   explicitly so the permission cannot be read as licence to touch the instrument generally.
