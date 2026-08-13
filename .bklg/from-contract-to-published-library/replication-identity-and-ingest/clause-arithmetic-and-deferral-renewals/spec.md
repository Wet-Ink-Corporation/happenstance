---
item: HS-S0113
stage: spec
created: 2026-08-12T13:47:54.167Z
updated: 2026-08-12T13:47:54.167Z
template_sig: 87bbf1d0
rendered_sig: b9c9e111
---

# Spec — The deferrals name experiments and the clause arithmetic comes out

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project (charter) | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/project.md` — AC-010, AC-014, DR-9, DoD 7 |
| This spec | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/clause-arithmetic-and-deferral-renewals/spec.md` |
| Key briefs | `…/replication-identity-and-ingest/_decomposition.md` — *architecture: Tension 3* (the re-derived ledger, the split defect, the five named experiments), *Tension 4* (repair versus gap), **AC-A11**, **AC-A12**; *testing: The test mix* (`spec-trace` is the evidence command for AC-010, AC-012 and AC-014) |
| Story map row | `…/replication-identity-and-ingest/_storymap.md`, *Slices*, `spec-repairs-and-clause-exit` row 2; *Merge order* §7 — last on purpose |
| Signed-off design | `…/replication-identity-and-ingest/_design.md` — **no user-facing surface**, approved 2026-08-12. This story adds no public Rust item at all; there is no surface id to render |
| Discover stage (this story) | `…/clause-arithmetic-and-deferral-renewals/discover.md` — the signal ledger, the five clauses at HEAD, and the two named mutants this spec is written against |
| Roadmap pointer | `RUNBOOK.md:4516-4620` (phase 13, whose stated range is SY-1 – SY-35 at `:4535`); `RUNBOOK.md:305-307` (the ADR queue rows and their parenthesised ranges); `RUNBOOK.md:308-312` and `:334-336` (why a range is a scope statement and not a coverage guarantee); `RUNBOOK.md:4626-4670` (phase 14, which owns the suffix store) |

## One-line PR slice

Settle or renew every `[DEFERRED]` `SY` clause against a *named* experiment (a renewal with no
experiment is a build failure under CF-38), then compute the union of ADR-0026's and ADR-0027's
clause ranges against this project's stated range, write the arithmetic down, and leave
`cargo xtask spec-trace` green.

## Executive summary

**This PR lands no Rust. It lands the two things in this project that are true only if someone
computes them, and neither of which any tool computes.**

The first is the deferral discipline. Five `SY` clauses carry `[DEFERRED]` markers — SY-14, SY-18,
SY-27, SY-28, SY-32 — and CF-38 makes an *empty* experiment a build failure
(`spec/SPECIFICATION.md:210-217`). Nothing anywhere checks that a named experiment is still the
thing that would settle its clause, and nothing checks that its owning phase has not already run.
Four of the five name **"the phase that builds the two peer adapters"** as their owner
(`spec/SPECIFICATION.md:6240-6241`, `:6368-6369`, `:6635-6636`, `:6787-6788`) — and that is *this*
project, arriving at its own exit. A clause whose owner is a phase that has finished is the
deferral CF-38's own justification describes: *"a decision nobody wanted to make, and by the time
anyone notices it has been load-bearing for a year"* (`:212-217`). This story is the pass that
notices.

The second is the arithmetic. `RUNBOOK.md:4535` says phase 13 discharges SY-1 – SY-35; the ADR
queue's parenthesised ranges (`RUNBOOK.md:305-307`) give ADR-0026 SY-8 – SY-18, ADR-0027
SY-1 – SY-7 and SY-19 – SY-31, and ADR-0028 SY-32 — which leaves SY-33, SY-34 and SY-35 assigned
to nothing. Phase 4 already proved a parenthesised range is a scope statement and not a coverage
guarantee: its body claimed 64 clause IDs while its five queue rows named 35, and the missing 29
were found by a hand audit and by no gate (`RUNBOOK.md:308-312`). The standing rule that came out
of it is one sentence: *"a phase's clause range and the union of its ADRs' clause ranges are two
numbers, and nothing checks that they are equal. Compute both at the phase's exit"*
(`RUNBOOK.md:334-336`). This is that exit.

**The delta over slice 1 is verification rather than authorship.** Both ADR stories have already
written their halves down in computable form — ADR-0026 states **SY-8 – SY-18 + SY-33 + SY-34**
(13) and ADR-0027 states **SY-1 – SY-7 + SY-19 – SY-31 + SY-35** (21), with SY-32 subtracted as a
named handoff (`…/adr-0026-peer-ingest-and-transport/spec.md`, *ADR-0026's stated clause range*;
`…/adr-0027-merge-compensation-and-message-set/spec.md`, §8). This story does not re-derive those
ranges from intent; it reads what the **merged atoms actually claim**, adds them, and compares in
both directions. If the merged text differs from the plan — and the plan's own EC-003 says it
might — the union is recomputed from what landed, and a shortfall becomes a recorded finding with
a named owner. **The one forbidden reconciliation is editing `RUNBOOK.md:4535` so the totals
agree**, which every check in this repository would survive, because nothing compares a RUNBOOK
range to an ADR range. That comparison is the deliverable.

**The delta over `frozen-clause-repairs` (HS-S0114) is which part of a clause is touched.** That
story repairs `Rule:` lines and `Rejects:` fields — prose whose admitted-implementation set is
unchanged. This story touches the **interior of maturity markers**, and one of the two moves it
makes is *not* a repair: a settlement moves the marker, which changes the set of implementations
the clause admits, and is therefore an amendment that an ADR must authorise by clause id.

## Context pack

The load-bearing decisions, stated inline. Everything deeper is a signposted anchor.

**1. Two moves exist, and they are not the same move.** A **renewal** leaves the maturity marker
where it is and rewrites the interior of the marker — the experiment, what it would settle, the
owning phase. Under `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`'s mechanical
test the admitted-implementation set is unchanged, so a renewal is a **repair** and needs no ADR.
A **settlement** moves the marker (`[DEFERRED]` → `[PROVISIONAL]` or `[FROZEN]`), which changes
what the clause admits, so it is an **amendment**: it is authorised by ADR-0026 or ADR-0027 **by
clause id**, or it does not happen here. Both ADRs are merged before this story by construction
(`_storymap.md`, *Merge order* §1 and §7).

**2. The default is renewal, and the burden of proof sits on settlement.** ADR-0026's own spec
already schedules SY-14 and SY-18 as **renewals** supplying its range's half of the CF-38 sweep
(`…/adr-0026-peer-ingest-and-transport/spec.md`, AC-009). Read the merged record rather than that
plan — but where the record states no marker move for a clause id, the disposition is a renewal.
A settlement taken on this story's own judgement is the *Out of scope* violation the project
forbids: the output would be a recorded finding, a new decision atom through `.kb/_intake/` and a
re-plan, never a quieter marker (`project.md`, *Out of scope*, fifth bullet).

**3. A renewal whose owning phase has already run settles nothing, and this is the story's
sharpest rule.** §1.3 states the marker's own required form —
`[DEFERRED — <the experiment that settles it, and its owning phase>]` — and gives the reason: *"A
deferral with no owning phase is a decision the next pass makes by accident"*
(`spec/SPECIFICATION.md:204-208`). Four of the five markers name the peer-adapter phase, which is
this one. Every renewal therefore re-points its owning phase at a phase that has **not** run.
Phase 14 (`retention-and-incomplete-logs`, HS-P0018, ADR-0028) is the last one in the plan
(`RUNBOOK.md:4626-4670`) and it explicitly owns the suffix store — *"a testkit-adjacent store that
deliberately holds only a suffix of its own log"* (`RUNBOOK.md:4645-4649`), CF-27's instrument and
the far end of the completeness axis. Where no unrun phase can honestly own an experiment, that is
a **blocker to raise**, not a phase number to invent.

**4. SY-27 and SY-28 are one disposition across two clauses.** SY-28's marker says so in its own
words — *"same experiment as SY-27; this clause is its consequence and cannot be written before
it"* (`spec/SPECIFICATION.md:6657-6658`). They move together or neither moves. And SY-27's
settlement has a trap written into SY-28's own `Rejects:` field: the round-trip rule everyone
writes first — push everything, pull everything, assert the two logs equal — *"against a scoped
spoke it fails for a conformant adapter… it will be 'fixed' by weakening it until it passes"*
(`:6665-6670`). If SY-27 were settled on whole-log replication because the suite is green, the
suite is green only because **no scoped peer exists in the tree to refute it**. The instrument
that could refute it is phase 14's. A settlement taken on that evidence is CF-1's own worked
failure, one family over.

**5. SY-32 is a handoff already recorded, and this story confirms the record rather than
re-deciding it.** §5.13 states flatly that *"SY-32 depends on ES-39 and cannot be settled ahead of
it"* (`spec/SPECIFICATION.md:7002-7003`), and ES-39 is ADR-0028's under phase 14
(`RUNBOOK.md:307`, `:4637`). ADR-0026 records the handoff by name (project AC-011). But SY-32's
*own marker* still names **"the phase that builds the two peer adapters"** as its owning phase
(`spec/SPECIFICATION.md:6787-6789`) — the document disagrees with itself, in the exact direction
this story exists to catch. The renewal re-points the marker at phase 14 and ADR-0028, which makes
the marker agree with §5.13 without touching either clause's normative sentence.

**6. A named experiment is only named if its citation resolves.** Two of the five markers cite
`E2E-CASES.md:1595-1600` for the completeness instrument (SY-27 at
`spec/SPECIFICATION.md:6635-6636`, SY-32 at `:6783-6784`). At HEAD, `spec/E2E-CASES.md:1595-1600`
is the *ingest re-check* discussion; the suffix-store instrument is at **`:1682`**. The anchors
have drifted, and CF-38 cannot see it — the check is `falsifier.trim().len() < 12`
(`xtask/src/spec_trace.rs:659`), a non-emptiness test that a drifted citation passes forever.
Every citation inside a marker this story rewrites is verified against its referent and re-anchored
under `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` — *verify the referent, not
just the address*.

**7. The arithmetic is a set computation stated in both directions.** Not "the totals agree" but:
`(ADR-0026's claimed range ∪ ADR-0027's claimed range ∪ {SY-32 → ADR-0028})` against
`{SY-1 … SY-35}`, with the difference printed **each way** — clauses claimed by no record, and
clauses claimed by both. A clause counted twice is as wrong as a clause counted zero times, and
ADR-0026 *cites* SY-1, SY-2 and SY-6 without *claiming* them
(`…/adr-0026-peer-ingest-and-transport/spec.md`, AC-002), so the union must distinguish citation
from claim or it will double-count by construction.

**8. The forbidden reconciliation, named so it cannot arrive by drift.** Editing `RUNBOOK.md:4535`
from SY-1 – SY-35 to SY-1 – SY-32 makes the totals agree, leaves every gate green, and orphans the
transport-refusal pair (SY-33, SY-34) and SY-35 — *"Anything replication must reason about MUST be
in the tags"* (`spec/SPECIFICATION.md:6868`), the clause DR-5's no-payload-decoding rule rests on.
The architecture brief forbids it in six words: *"Fix the split; do not fix the total by editing
the RUNBOOK"* (`…/_decomposition.md`, *Tension 3*). `RUNBOOK.md` is outside this story's PR
boundary for that reason and for no other.

**9. §1.3's census is prose a person wrote, and the tool holds it to account.** The document states
its own figures — 200 clause IDs, 198 normative, **139 `[FROZEN]`, 49 `[PROVISIONAL]`, 10
`[DEFERRED]`**, two `[NON-NORMATIVE]` (`spec/SPECIFICATION.md:219-222`) — and
`check_stated_census` compares that sentence against the computed census, per maturity column
(`xtask/src/spec_trace.rs:459-508`). **Any settlement moves two of those numbers**, and §1.3 is
hand-edited while §7.1–§7.2 are *generated and held equal* to the computation. So the order is
fixed: edit the markers, hand-correct §1.3, then `cargo xtask spec-trace --write`, then re-run in
`Mode::Check`. Hand-editing §7.1 or §7.2 is how the equality stops meaning anything
(`xtask/src/spec_trace.rs:735-744`).

**10. This story is downstream of two mounts and inherits both.** `gate-mounts-for-the-sync-suite`
(HS-S0104) put the sync rule file(s) into `RULE_FILES` (`xtask/src/spec_trace.rs:85-89`), without
which no `SY` clause's `Rule:` name can resolve and §7.2 renders every one of them `†`.
`frozen-clause-repairs` (HS-S0114) dropped the `(new)` markers from the `Rule:` lines whose rules
now exist (`xtask/src/spec_trace.rs:1626-1634` is what makes `(new)` mean *unresolved*). If either
has not landed, `spec-trace` cannot produce the evidence this story's ACs cite, and the correct
behaviour is to raise the blocker rather than to compute against a tool that cannot see the suite.

**11. A settlement may unblock a rule; it does not write one.** SY-27's `Rule:` field says
`scoped_replication_resume_is_sound` is *"unwritable until this is settled"*
(`spec/SPECIFICATION.md:6638-6640`). If a clause settles here, its scheduled rule becomes writable
and **keeps its `(new, happenstance-sync-testkit)` marker until the rule exists** — dropping the
marker before the rule lands makes `spec-trace` check 4 resolve a name against nothing. The
unblocked rule is recorded as a named handoff, never written here (`discover.md`, *Gate: Discover*,
box 6).

**The persona-journey slice.** The reader is the evaluator and the adapter author of
`.kb/product/` — the one who opens a clause to find out what the bar is, and what is still open.
Today they open SY-18, see a live-looking deferral, and read an experiment — *"whether a capability
declaration prevents the failure or merely relocates it"* — that sits beside an API which has
already been built: `SyncPeer::limits()` exists, is non-`async`, and returns `PeerLimits`
(`crates/happenstance-sync/src/peer.rs:150-158`), while `PeerLimits::admits` is explicitly advisory
(`:319-324`), so the question is untouched by the type existing. After this story, every open `SY`
clause tells that reader an experiment someone could actually run and a phase that has not yet run
to own it — or it is no longer open.

## Integration contract

- **Archetype**: `capability` — the observable slice is what a reader of `spec/SPECIFICATION.md`
  meets: a deferral that names a live experiment and a live owner, and a clause ledger whose stated
  figures equal the computed ones. It ships mounted in the document the gate reads on every run,
  not as a note beside it.
- **Slice / milestone**: `spec-repairs-and-clause-exit`. Slice-mate: **`frozen-clause-repairs`**
  (HS-S0114, predecessor and hard dependency). Both are implemented in one context and mounted as
  one integrated surface — one `spec/SPECIFICATION.md` diff, reviewed once against project DoD 7.
  Merge order is fixed at `…/_storymap.md`, *Merge order* §7: repairs first, arithmetic last,
  *"because a repair can only name the rules and symbols that exist, and the arithmetic is an exit
  computation"*.
- **Mount point**: **`spec/SPECIFICATION.md`** — §5's five `[DEFERRED]` `SY` markers, §1.3's
  hand-written census sentence (`:219-222`), and the generated §7.1–§7.2 region. This file is the
  composition root for every claim this project makes about its own clauses, and it is reached by
  the gate through the **`specification traceability`** `REQUIRED` step in `xtask/src/main.rs:303-328`
  and by every story through `reachability_static` (`.redkiln/config.yaml:48`). A marker that is
  not in this file is not in the ledger.
- **Wires into**:
  - `xtask/src/spec_trace.rs:651-670` — check 2, the CF-38 falsifier test, and the bar it does
    *not* clear (`len() < 12`, non-emptiness only).
  - `xtask/src/spec_trace.rs:459-508`, `:517-556`, `:570-588` — `check_stated_census` and the
    §1.3 parser, which refuse a census sentence that names a marker more than once.
  - `xtask/src/spec_trace.rs:735-744`, `:1057-1183` — the `Mode::Check` equality against the
    generated §7.1–§7.2 region and `rule_cell`'s `†`.
  - `xtask/src/spec_trace.rs:85-89` — `RULE_FILES` as widened by HS-S0104; the reason an `SY`
    clause's rule name can resolve at all.
  - `.kb/decisions/` — the **merged** ADR-0026 and ADR-0027 atoms, whose stated ranges are the
    left-hand side of the arithmetic; read as landed, never as planned.
  - `references/adr/0026-*.md`, `references/adr/0027-*.md` — the long-form records, where a range
    that a ~100-line atom compresses is cited by `file:line`.
  - `references/evaluation/PRESSURE-TEST.md:685-693` — the whole-log-versus-scoped experiment that
    SY-14, SY-27 and SY-28 cite; present in the tree, so the citations resolve.
  - `spec/E2E-CASES.md:1682` — the completeness instrument SY-27 and SY-32 *mean*, at the line it
    is actually on.
- **Renders surfaces**: **none.** `…/replication-identity-and-ingest/_design.md` records no
  user-facing surface for this project (approved 2026-08-12), and this story adds no public Rust
  item of any kind.
- **Public items**: none. Nothing in this diff is compiled.
- **Conformance rule(s)**: **none added, and none may be.** `discover.md`'s box 6 states it: this
  story's diff is clause markers, experiment text and a written computation. It is not
  adapter-observable — no adapter passes or fails differently because of it. The substitute
  obligation, per `CLAUDE.md`'s *a rule no adapter can fail is decorative* corollary applied one
  level up, is that **every disposition names what would falsify it and who would run it**, which
  AC-003 and AC-004 make checkable. Where a settlement makes a scheduled rule writable
  (`scoped_replication_resume_is_sound`, `round_trip_preserves_the_agreed_scope`,
  `bulk_ingest_is_idempotent_in_bounded_round_trips`, `peer_declares_its_own_limits`), the rule is
  recorded as a handoff and its `(new)` marker stays.
- **Clause(s)**:
  - **Renewed or settled** (marker interior; marker itself only under ADR authorisation):
    **SY-14** (`spec/SPECIFICATION.md:6234-6260`), **SY-18** (`:6362-6392`), **SY-27**
    (`:6629-6650`), **SY-28** (`:6653-6670`), **SY-32** (`:6775-6800`).
  - **Confirmed, not changed**: **SY-33** (`:6808`), **SY-34** (`:6833`), **SY-35** (`:6868`) —
    the three the intake split assigned to nothing; this story verifies the merged ADRs claim
    them and records the finding if they do not.
  - **Amended**: none by this story's own authority. Every marker move cites the ADR that
    authorises it by clause id; where none does, the output is a finding and a re-plan
    (`project.md`, AC-002 and *Out of scope*).
  - **Untouched**: every `[FROZEN]` normative sentence, every `Rejects:` field, every `Rule:`
    line. Those belong to HS-S0114 and to slice 1.
- **Advances DoD scenario**: initiative **DoD 12** — *"The clause ledger is audited at publish. A
  run over the specification reports every clause's maturity, and no clause is provisional with an
  empty falsifier; the count in the report matches `spec/SPECIFICATION.md`'s own stated figure"*
  (`.bklg/from-contract-to-published-library/initiative.md:393-395`). This story is the `SY`
  family's half of it, and the census correction is literally the second sentence. Secondarily
  initiative **DoD 13** (the gate green on the assembled whole, including the cross-reference step)
  and project **DoD 2** and **DoD 7**.

## PR boundary

**In this PR**

- `spec/SPECIFICATION.md` — the interiors of five `[DEFERRED]` `SY` markers (experiment, what it
  would settle, owning phase, and the citations inside them); any marker **move** authorised by
  ADR-0026 or ADR-0027 by clause id; §1.3's hand-written census sentence if a marker moved
  (`:219-222`); and §7.1–§7.2 regenerated by `cargo xtask spec-trace --write`, never by hand.
- `.bklg/…/clause-arithmetic-and-deferral-renewals/_clause-arithmetic.md` — **the written
  computation**, and the artifact AC-014's *"computed and written down at exit"* actually names:
  the ledger re-derived from `spec/SPECIFICATION.md` at HEAD, the five dispositions with their
  experiments and owners, the two ADRs' ranges **as merged**, the union, and the set difference in
  both directions. Cited from `_ledger.md`; harvested at project closeout.
- This story's own backlog folder (`_ledger.md`, `implementation-report.md`).

**Explicitly not in this PR**

- **`RUNBOOK.md`, in any form.** The forbidden reconciliation is a one-line range edit that every
  gate survives (*Context pack* 8). If phase 13's range is genuinely wrong, that is a re-plan with
  a named owner, recorded in `_clause-arithmetic.md` and raised — not a diff hunk here.
- Any `Rule:` line, `Rejects:` field or normative MUST — `frozen-clause-repairs` (HS-S0114) owns
  the first two and nothing owns the third without a new ADR.
- Any `.kb/` atom. Atoms arrive through `.kb/_intake/` and `/redkiln:kb-ingest`, never by hand
  (`CLAUDE.md`, *Where the work lives*); slice 1 owns ADR-0026 and ADR-0027, and a settlement this
  story cannot authorise is a **blocker**, not an atom written here.
- Any conformance rule, mutant or fixture, including the four this story may unblock. A rule
  written to make a freshly settled clause look discharged would be green against every peer in the
  tree and refutable by none of them.
- `xtask/**`. CF-38's checker is not strengthened here; its weakness (non-emptiness, not
  namedness) is *stated* and *worked around by hand*, and any proposal to make namedness
  machine-checkable is recorded as a finding for a later story.
- `CHANGELOG.md`. The per-rule entry lint (`xtask/src/lints.rs:525`) is owed one entry per **rule**
  and this story lands none.

**Merge DoD**: `cargo xtask spec-trace` green in `Mode::Check` with no `SY` deferral naming a
finished phase, and `_clause-arithmetic.md` showing the union of the two merged ADRs' ranges equal
to SY-1 – SY-35 with SY-32 subtracted as a handoff — differenced in both directions, not asserted.

```
spec/SPECIFICATION.md
.bklg/from-contract-to-published-library/replication-identity-and-ingest/clause-arithmetic-and-deferral-renewals/**
```

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The ledger is re-derived at HEAD, not read from a brief** | The `SY` population and its maturity split are recomputed from `spec/SPECIFICATION.md` as it stands after HS-S0114 — expected 21 `[FROZEN]`, 9 `[PROVISIONAL]`, 5 `[DEFERRED]` (SY-14, SY-18, SY-27, SY-28, SY-32) — and any disagreement with a brief is resolved *for* the specification. The intake brief's ledger is already known wrong twice over: its ranges, and calling the wire clauses `VT-*` when they are `WF-*`. | `spec/SPECIFICATION.md` §5; `…/_decomposition.md`, *Tension 3*; `project.md`, *Risks* row 1, AC-014 |
| **Each deferral gets a written disposition, and settlement carries the burden** | Five dispositions, one per clause: **renew** (marker stays; interior rewritten) or **settle** (marker moves). A settlement happens only where the merged ADR-0026 or ADR-0027 states the move **by clause id**, and the record's stated target marker is the one used. Where the ADR says "settled" without naming a marker, the clause takes `[PROVISIONAL]` with a falsifier and not `[FROZEN]` — CF-25's portfolio qualification on freezing a port clause is not this story's to discharge. | `.kb/decisions/` (merged atoms) + `references/adr/0026-*.md`, `0027-*.md`; `spec/SPECIFICATION.md:224-227` (CF-25's standing qualification); `discover.md`, *Gate: Discover*, box 7 |
| **No renewal names a phase that has already run** | Four markers name *"the phase that builds the two peer adapters"* — this project. Each renewal re-points at a phase that has not run, with phase 14 / `retention-and-incomplete-logs` / ADR-0028 the only one left in the plan and the owner of the suffix store the SY-27/SY-28/SY-32 experiments need. No unrun owner ⇒ blocker raised, never a phase invented. | `spec/SPECIFICATION.md:204-208`, `:6240-6241`, `:6368-6369`, `:6635-6636`, `:6787-6789`; `RUNBOOK.md:4626-4670` |
| **Every citation inside a rewritten marker resolves to its referent** | `E2E-CASES.md:1595-1600` (cited by SY-27 and SY-32 for the completeness instrument) points at the ingest-re-check discussion at HEAD; the instrument is at `spec/E2E-CASES.md:1682`. Re-anchored, with the subject string checked rather than the line number trusted. `references/evaluation/PRESSURE-TEST.md:685-693` is verified to still carry *"Is replication whole-log or scoped?"*. | `spec/E2E-CASES.md:1682`; `references/evaluation/PRESSURE-TEST.md:685-693`; `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`; `.kb/playbooks/verify-the-referent-and-report-coverage.md` |
| **SY-27 and SY-28 move as one disposition, or not at all** | SY-28's marker binds it to SY-27's experiment and forbids it moving first. A settlement of SY-27 on whole-log replication justified by a green suite is **refused** here: no scoped peer exists in the tree, so green is the absence of the refuting instrument, not evidence. That instrument is phase 14's. | `spec/SPECIFICATION.md:6653-6670`, `:6629-6650`; `spec/SPECIFICATION.md:7161-7207` (CF-1's worked failure); `discover.md`, *The wrong implementation*, third paragraph |
| **SY-32's marker is made to agree with §5.13** | §5.13 says SY-32 cannot be settled ahead of ES-39; SY-32's own marker still names the peer-adapter phase as its owner. The renewal re-points it at phase 14 and ADR-0028, matching the handoff ADR-0026 already records for project AC-011. Neither clause's normative sentence is touched. | `spec/SPECIFICATION.md:7002-7003`, `:6775-6800`; `RUNBOOK.md:307`, `:4637`; `project.md`, AC-011 |
| **A settled clause's scheduled rule keeps its `(new)` marker** | `(new)` is what makes `spec-trace` treat a `Rule:` name as *scheduled* rather than dangling. A clause that settles here unblocks its rule for a later story; dropping the marker before the rule exists turns check 4 into a resolution against nothing. The unblocked rule is recorded as a named handoff instead. | `xtask/src/spec_trace.rs:1626-1634`, `:684-720`; `spec/SPECIFICATION.md:6638-6640` |
| **The arithmetic is a set computation, differenced both ways** | `(ADR-0026 claims ∪ ADR-0027 claims ∪ {SY-32 → ADR-0028})` versus `{SY-1 … SY-35}`, printed as *claimed by nothing* and *claimed twice*. Planned shape: 13 + 21 + 1 = 35. **Claims** are counted, **citations** are not — ADR-0026 cites SY-1, SY-2 and SY-6 without claiming them, and a union that conflates the two double-counts by construction. | `RUNBOOK.md:305-307`, `:4535`, `:334-336`; `…/adr-0026-peer-ingest-and-transport/spec.md`, AC-002; `…/adr-0027-merge-compensation-and-message-set/spec.md`, §8 and AC-008 |
| **SY-33, SY-34 and SY-35 are verified as claimed, not assumed** | The intake split assigned all three to nothing. The plan assigns SY-33/SY-34 to ADR-0026 (transport refusal, peer-shaped) and SY-35 to ADR-0027 (reconciliation-shaped). This story checks the **merged** records for those ids; absence is a shortfall of exactly three, recorded with an owner and raised. | `spec/SPECIFICATION.md:6808`, `:6833`, `:6868`; `…/_decomposition.md`, *Tension 3* and AC-A12 |
| **The stated census and the generated region are made true in that order** | Markers first, then §1.3's hand-written sentence if any marker moved, then `cargo xtask spec-trace --write`, then `Mode::Check`. §1.3 is authored prose held to the computation per column; §7.1–§7.2 are generated and held **equal** to it. Hand-editing either generated table is how the equality stops meaning anything. | `spec/SPECIFICATION.md:219-222`; `xtask/src/spec_trace.rs:459-508`, `:735-744`, `:1057-1145` |
| **The exit diff is reviewable against DoD 7 by reading it** | `git diff spec/SPECIFICATION.md` for this story shows marker interiors, at most the census sentence, and the generated region — no MUST, no `Rejects:` line, no `Rule:` line, and no maturity marker moved without a named authorising ADR beside it in `_clause-arithmetic.md`. | `project.md`, DoD 7; `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`; `.kb/governance/rewrite-the-referent-never-the-reasoning.md` |

## Data and migrations

**N/A — no data, no schema, no persisted format.** This story compiles nothing, stores nothing and
ships no public item; `_design.md` records no surface and the PR boundary contains one markdown
document and one backlog folder.

The one thing in the diff that resembles a migration is **§7.1–§7.2**, and it is deliberately not
one: the region is *generated* and held equal to what `spec-trace` computes, so it is regenerated
with `cargo xtask spec-trace --write` rather than migrated by hand
(`xtask/src/spec_trace.rs:735-744`). The adjacent §1.3 census sentence is the opposite — authored
prose, hand-corrected, and then held to the same computation per maturity column by
`check_stated_census` (`:459-508`). Treating either one as the other is the failure mode: a
hand-edited generated table disagrees with the tool silently, and a generated census sentence would
be a number that can never be wrong and therefore never checks anything.

## Acceptance criteria

The persona is the **evaluator** — *"deciding from a bounded look at public evidence whether
'DCB-compliant', 'storage-agnostic' and 'edge-capable' are real claims"*, whose journey is *Decide
in one sitting* (`.bklg/from-contract-to-published-library/initiative.md:222-225`, `:249-251`) — and,
where a rule or a handoff is at stake, the **adapter author**, whose journey is *Learn when you are
finished* (`initiative.md:210-214`, `:245-247`). Both are carried from
`_discovery/distillation/personas-and-journeys.md`; no `authority_tier: product` atom exists yet
(`initiative.md:229-240`). The initiative AC this story serves them through is **AC-10** — *"The
evaluator can tell how strong each promise is… legibly frozen, provisional with a named falsifier,
or deliberately deferred, and that state is accurate at the moment of publish"*
(`initiative.md:335-338`).

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** an evaluator who has been handed the intake brief's clause ledger, **WHEN** they open `spec/SPECIFICATION.md` §5 at this story's merge commit, **THEN** the SY population and maturity split written into `_clause-arithmetic.md` — 21 `[FROZEN]`, 9 `[PROVISIONAL]`, 5 `[DEFERRED]` (SY-14, SY-18, SY-27, SY-28, SY-32) — is the split the document itself yields, and every place a brief disagrees is resolved **for the document** and the disagreement recorded rather than quietly dropped. | `_clause-arithmetic.md` §*Re-derived ledger* reproduces its own derivation command over `spec/SPECIFICATION.md`, re-run by the reviewer and compared; `cargo xtask spec-trace` census check (`xtask/src/spec_trace.rs:459-508`) green on the same tree |
| **AC-002** | **GIVEN** an evaluator reading a `[DEFERRED]` `SY` clause to decide whether the promise is being worked on, **WHEN** they read any of the five, **THEN** each carries exactly one written disposition — *renew* (marker stays, interior rewritten **in place at the clause**, never as a sidecar note or a companion file) or *settle* (marker moves) — and every settlement names, beside it in `_clause-arithmetic.md`, the merged ADR-0026 or ADR-0027 statement that authorises it **by clause id**; where a settlement makes a scheduled rule writable, the rule is recorded as a named handoff and keeps its `(new, …)` marker until the rule exists. | `_clause-arithmetic.md` §*Dispositions* — five rows, each with authorising-ADR cell (empty ⇒ renewal); `git diff spec/SPECIFICATION.md` shows no `Rule:` line lost a `(new` marker; `cargo xtask spec-trace` check 4 green (`xtask/src/spec_trace.rs:684-720`, `:1626-1634`) |
| **AC-003** | **GIVEN** an evaluator who wants to know what would end a deferral, **WHEN** they read the marker's experiment, **THEN** it names something that could still falsify the clause **today** — verified by hand against its referent, not merely by being non-empty — and in particular SY-18's experiment no longer cites building an API that `crates/happenstance-sync/src/peer.rs:150-158` already provides while `PeerLimits::admits` stays advisory (`:319-324`). | `cargo xtask spec-trace` check 2 green (`xtask/src/spec_trace.rs:651-670`) as the floor; above it, `_clause-arithmetic.md` §*Dispositions* carries, per clause, the sentence *what would falsify this* and the reviewer's confirmation that the named thing is not already built |
| **AC-004** | **GIVEN** the next phase's implementer picking up an open clause, **WHEN** they read its owning phase, **THEN** no `[DEFERRED]` `SY` marker names a phase that has already run: each of the five names a phase still ahead (phase 14 `retention-and-incomplete-logs` / ADR-0028 being the last one left in the plan, and the owner of the suffix-store instrument), and where no unrun phase can honestly own an experiment a blocker is raised rather than a phase number invented. | `rg -n 'the phase that builds the two peer adapters' spec/SPECIFICATION.md` returns **zero** hits inside any `SY` maturity marker after the change; `_clause-arithmetic.md` §*Dispositions* names an owner per clause, cross-checked against `RUNBOOK.md:4626-4670` |
| **AC-005** | **GIVEN** an evaluator who follows a citation out of a marker to see the evidence, **WHEN** they open it, **THEN** it lands on the referent rather than on whatever moved into that address: `spec/E2E-CASES.md`'s completeness instrument is cited at `:1682` and not the stale `:1595-1600`, and `references/evaluation/PRESSURE-TEST.md:685-693` is confirmed to still carry the whole-log-versus-scoped experiment. | Every citation inside a rewritten marker opened and matched by **subject string**, per `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`; the check and its result recorded in `_clause-arithmetic.md` §*Citations re-anchored*, one row per citation with the matched string quoted |
| **AC-006** | **GIVEN** an adapter author who would write the round-trip rule SY-28 warns about, **WHEN** they read SY-27 and SY-28 after this story, **THEN** the two carry **one** disposition and neither has moved without the other; and any settlement of SY-27 on whole-log replication justified by a green suite is refused in writing, because no scoped peer exists in the tree to refute it and green is the absence of the instrument rather than evidence. | `_clause-arithmetic.md` §*Dispositions* shows SY-27 and SY-28 with the identical disposition and identical experiment; the refusal is written out with its reason and cites `spec/SPECIFICATION.md:6653-6670` and CF-1's worked failure (`:7161-7207`) |
| **AC-007** | **GIVEN** an evaluator who reads §5.13 and then SY-32 and finds the document disagreeing with itself, **WHEN** they read them after this story, **THEN** SY-32's marker names phase 14 / `retention-and-incomplete-logs` / ADR-0028 as its owner — agreeing with *"SY-32 depends on ES-39 and cannot be settled ahead of it"* — and neither clause's normative sentence has been touched. | `spec/SPECIFICATION.md:6775-6800` read against `:7002-7003`; `git diff` shows the change confined to the marker interior; the handoff matches the one ADR-0026 already records for project AC-011 (`project.md`, AC-011) |
| **AC-008** | **GIVEN** a reviewer asking whether this project covered the clause range it claimed, **WHEN** they open `_clause-arithmetic.md`, **THEN** they find a set computation, not an assertion: `(ADR-0026 claims ∪ ADR-0027 claims ∪ {SY-32 → ADR-0028})` against `{SY-1 … SY-35}`, differenced **in both directions** (claimed by nothing; claimed twice), with **claims** counted and **citations** excluded — ADR-0026 cites SY-1, SY-2 and SY-6 without claiming them — and the two ADRs read **as merged**, never as planned. | `_clause-arithmetic.md` §*The arithmetic*: two explicit id lists, two difference lists, and the per-ADR claim lists each cited to the merged atom under `.kb/decisions/` (long form under `references/adr/`) by `file:line` |
| **AC-009** | **GIVEN** SY-33, SY-34 and SY-35 were assigned to no ADR by the intake split, **WHEN** the arithmetic runs, **THEN** the merged records are checked for those three ids specifically and the result stated either way — claimed, or a shortfall of exactly three recorded with a named owner and raised — and in neither case is `RUNBOOK.md:4535`'s range edited to make the totals agree. | `_clause-arithmetic.md` §*The arithmetic*, the three ids named individually with their claiming record or their shortfall row; `git diff --name-only` contains no `RUNBOOK.md` |
| **AC-010** | **GIVEN** the repository owner reviewing this project's exit against DoD 7, **WHEN** they read `git diff spec/SPECIFICATION.md` for this story, **THEN** it shows marker interiors, at most §1.3's hand-written census sentence, and the `spec-trace`-regenerated §7.1–§7.2 — no normative MUST, no `Rejects:` field, no `Rule:` line, and no maturity marker moved without a named authorising ADR — with the order honoured (markers → §1.3 by hand → `cargo xtask spec-trace --write` → `Mode::Check`) and `cargo xtask spec-trace` green. | `cargo xtask spec-trace` (`Mode::Check`, `xtask/src/main.rs:303-328`, `.redkiln/config.yaml:48`) green; `cargo xtask ci --fast` green (`.redkiln/config.yaml:55`); the diff read line by line against `project.md` DoD 7 and `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`'s mechanical test |

**Coverage of the traced project ACs.** Project **AC-010** (*no deferral survives without a named
experiment*) is AC-002 + AC-003 + AC-004 + AC-005 + AC-006 + AC-007 — the disposition, the live
experiment, the unrun owner, the resolving citation, and the two clauses whose dispositions are
constrained by other clauses. Project **AC-014** (*the clause arithmetic is done… every maturity
marker checked against `spec/SPECIFICATION.md` rather than against this charter, and
`cargo xtask spec-trace` green*) is AC-001 + AC-008 + AC-009 + AC-010. No AC here is a restatement
of another; the split is *disposition* versus *arithmetic* versus *the gate that carries both*.

## Interaction quality

This story **renders no surface**: `…/replication-identity-and-ingest/_design.md` records
*"N/A — no user-facing surface"* under every section, and its sign-off (2026-08-12) approved that
determination itself. So there is no DOM, no route and no design-system primitive to compose. What
there *is* — and what the RFC §6.7/D6 families still bind to — is a **rendered document a human
reads to make a decision**: `spec/SPECIFICATION.md`. Both families are therefore mapped onto the
document's own composition rules, which the repository already enforces mechanically. Every
invariant below is carried by an **AC row above**; this section only says which row carries which,
and how it is checked.

**STATE invariants.**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not a context jump** — a deferral is repaired at the clause the reader is already looking at, never by a companion note, an appendix or a "see also" that moves the reader out of §5 | **AC-002** | `git diff spec/SPECIFICATION.md` shows the change inside the marker; `_clause-arithmetic.md` is a *record of the computation*, not the place a disposition lives |
| **Non-occlusion** — the clause's normative sentence, its `Rejects:` field and its `Rule:` line stay exactly where and what they were; the repaired marker does not push, wrap or displace them | **AC-010**, **AC-007** | the diff carries no line outside the marker interior, §1.3 and the generated region |
| **Preserved position** — clause ids are stable and none is renumbered, so every external `file:line` and by-id citation into this document still lands (`spec-trace` itself resolves by id) | **AC-010** | `rg -n '^\*\*SY-' spec/SPECIFICATION.md` yields the same id sequence before and after; `cargo xtask spec-trace` green |
| **Reversibility** — the one generated region is reproducible rather than authored: `--write` regenerates §7.1–§7.2 from the computation, so an incorrect edit is undone by re-running the tool, never by re-typing a table | **AC-010** | `cargo xtask spec-trace --write` then `Mode::Check` clean (`xtask/src/spec_trace.rs:735-744`) |
| **Reachability without special tooling** — the keyboard-reachability analogue: every claim this story makes is reachable by opening one markdown file and one backlog file, with no build step, no generated site and no tool the reader must install | **AC-008**, **AC-005** | the reviewer follows every citation in `_clause-arithmetic.md` by hand and lands on real content |

**COMPOSITION invariants.** `_design.md` declares no surface and its *Anti-patterns* section is
`N/A`, so there is no approved visual composition to inherit; what binds instead is the document's
own composition, which `spec-trace` enforces, plus the three named mutants this story is written
against (`discover.md`, *The wrong implementation*).

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — a `[DEFERRED]` marker is never bare markup. §1.3's required form is `[DEFERRED — <the experiment that settles it, and its owning phase>]` (`spec/SPECIFICATION.md:204-208`): three composed parts — the experiment, what it would settle, the owning phase — and a marker missing any of them is unfinished even where CF-38 passes | **AC-003**, **AC-004** | CF-38 as the floor (`xtask/src/spec_trace.rs:659`, `len() < 12`); above it, the per-clause *what would falsify this / who owns it* rows in `_clause-arithmetic.md` |
| **Placement and hierarchy** — authored prose and generated tables occupy different levels and are never confused: §1.3's census sentence is hand-written and held to the computation per maturity column; §7.1–§7.2 are generated and held **equal** to it | **AC-010**, **AC-001** | `check_stated_census` (`xtask/src/spec_trace.rs:459-508`) and the `Mode::Check` region equality (`:735-744`) both green in the same run |
| **Transience** — nothing this story writes is revealed on demand or hidden behind a link: the disposition is persistent chrome inside the clause, and the arithmetic is a persistent artifact under the story folder that closeout harvests. A disposition that exists only in a PR description or a report body fails this | **AC-002**, **AC-008** | both files present in the merge diff; `_ledger.md` cites `_clause-arithmetic.md` by path |
| **Density budget, with the real numbers** — five markers, three composed parts each; 35 `SY` ids differenced two ways; a 13 + 21 + 1 = 35 union; a document-wide census of 200 clause ids / 198 normative / 139 `[FROZEN]` / 49 `[PROVISIONAL]` / 10 `[DEFERRED]` / 2 `[NON-NORMATIVE]` (`spec/SPECIFICATION.md:219-222`) that **any settlement moves by exactly the count settled**, in two columns at once | **AC-001**, **AC-008**, **AC-010** | the numbers appear as numbers in `_clause-arithmetic.md` and in §1.3, and `spec-trace` recomputes both |
| **Named anti-patterns, refused** — (1) a renewal that copies its experiment forward and bumps the phase, passing CF-38 and settling nothing; (2) making the totals agree by editing `RUNBOOK.md:4535`; (3) settling SY-27 on a green suite that is green only because the refuting instrument has not been built | **AC-003**, **AC-009**, **AC-006** | each is a stated refusal with a written reason in `_clause-arithmetic.md`, and each has a mechanical trace: a changed experiment string, an absent `RUNBOOK.md` from the diff, an identical SY-27/SY-28 disposition |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | A clause reads as settleable, but neither merged ADR-0026 nor ADR-0027 authorises the marker move **by clause id**. | The marker does **not** move. The disposition is recorded as a renewal, and the fact that a settlement was warranted but unauthorised is recorded as a finding with a named owner. A new decision atom goes through `.kb/_intake/` and `/redkiln:kb-ingest` in a later story, never by hand here (`project.md`, *Out of scope*, fifth bullet; `CLAUDE.md`, *Where the work lives*). |
| **EC-002** | No unrun phase can honestly own a renewal's experiment — phase 14 does not fit and nothing later exists in the plan. | Raise a blocker. Do not invent a phase number, do not write *"a future phase"*, and do not leave the finished phase named: a deferral whose owner has run is the exact defect this story exists to remove (`spec/SPECIFICATION.md:204-208`). |
| **EC-003** | The union of the two merged ADRs' claimed ranges does not equal `{SY-1 … SY-35}` minus SY-32's handoff. | State the difference in both directions and stop there. Fix the **split** by proposing a reassignment with a named owner, or record the shortfall as a finding. **Never** edit `RUNBOOK.md:4535`; `RUNBOOK.md` is outside the PR boundary for this reason (`…/_decomposition.md`, *Tension 3*). |
| **EC-004** | A clause id appears in *both* ADRs' claimed ranges. | Recorded as a defect in the split, with the id named and a proposed single owner. A double-count is as wrong as a zero-count and must not be silently de-duplicated on the way into the union (`RUNBOOK.md:334-336`). |
| **EC-005** | A citation inside a marker no longer resolves to its referent (the known case: `E2E-CASES.md:1595-1600`). | Re-anchor by **subject string**, not by nudging the line number, per `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`. If the referent has left the tree entirely, the experiment is restated against something that exists, or the clause's disposition escalates to EC-001's path. |
| **EC-006** | `check_stated_census` fails after a marker moved. | Hand-correct §1.3's sentence (`spec/SPECIFICATION.md:219-222`), then regenerate §7.1–§7.2 with `--write`. Never the reverse, and never a hand edit to a generated table — a hand-edited §7 disagrees with the tool silently and the equality stops meaning anything (`xtask/src/spec_trace.rs:735-744`). |
| **EC-007** | `frozen-clause-repairs` (HS-S0114) or `gate-mounts-for-the-sync-suite` (HS-S0104) has not landed on the branch this story runs against. | Halt and raise. Without HS-S0104's `RULE_FILES` widening (`xtask/src/spec_trace.rs:85-89`) every `SY` rule name renders `†` and the tool cannot see the suite; computing an exit against a blind tool produces a green run that proves nothing (`discover.md`, signal row 2). |
| **EC-008** | `cargo xtask spec-trace` is green but a marker's experiment is stale. | Not an error the tool can raise — this is the story's own mutant. The required output is the per-clause *what would falsify this* line in `_clause-arithmetic.md`, and a proposal (as a recorded finding, not an `xtask` change here) for making namedness machine-checkable in a later story. |

## Non-functional

| id | requirement | how it is met |
| --- | --- | --- |
| **NF-001** | **The gate's cost does not move.** This story compiles nothing and adds no test target; `cargo xtask ci --fast` should take the same wall-clock time before and after. | The PR boundary contains one markdown document and one backlog folder; no `Cargo.toml`, no `.rs`, no `xtask/**`. |
| **NF-002** | **The computation is reproducible by a third party.** A reviewer who has only the merge commit can re-derive the same ledger and the same union, without asking the author what they did. | `_clause-arithmetic.md` records the derivation commands it used (the `rg` patterns over `spec/SPECIFICATION.md`, and the `file:line` of every claimed range in the merged atoms) beside their results, per `.kb/playbooks/verify-the-referent-and-report-coverage.md`. |
| **NF-003** | **The exit is reviewable in one sitting.** DoD 7 is a *read* of `git diff spec/SPECIFICATION.md`, so the diff must stay small enough to read that way. | Two paths in the boundary; the generated §7 region is the only bulk, and it is regenerated rather than authored, so a reviewer skips it and checks it with `--write` instead. |
| **NF-004** | **The artifact survives the story.** `_clause-arithmetic.md` is harvest material for project closeout, not scaffolding. | Cited from `_ledger.md` and from `implementation-report.md`; the closeout lane harvests durable knowledge from stage reports into `.kb/` atoms, and this is the record of *how* an exit computation is done, which is transferable practice. |
| **NF-005** | **No accepted decision atom is edited.** An accepted decision atom is immutable and `redkiln validate --kb` checks each against `HEAD`. | This story writes no `.kb/` file at all; it *reads* the merged ADR-0026 and ADR-0027 atoms. Any correction to a merged ADR is a superseding atom in a later story, never an edit here (`CLAUDE.md`, *Where the work lives*). |

## Implementation notes (non-prescriptive)

**The order is not negotiable, and it is the only prescriptive thing here.** Markers first; then
§1.3's census sentence by hand *if and only if* a marker moved; then
`cargo xtask spec-trace --write`; then `cargo xtask spec-trace` in `Mode::Check`. Doing `--write`
first hides a census error inside a regenerated table.

**Derive the ledger before reading any brief.** The `SY` population and its maturity split come out
of `spec/SPECIFICATION.md` with a pattern match over the clause headings and their markers; record
the exact pattern used in `_clause-arithmetic.md` so the next reader can re-run it (NF-002). Two
briefs are already known wrong about this — the intake brief's ranges, and its `VT-*` for `WF-*`
(`project.md`, *Risks* row 1) — so a derivation that agrees with a brief and disagrees with the
document has found a brief defect, not a document defect.

**Distinguish *claims* from *citations* mechanically, not by eye.** A merged ADR mentions clause ids
in at least two registers: the range it is responsible for, and the frozen clauses it reconciles
against. ADR-0026 cites SY-1, SY-2 and SY-6 without claiming them
(`…/adr-0026-peer-ingest-and-transport/spec.md`, AC-002). Read the atom's **stated range** section
as the claim set and treat every other mention as a citation; if the merged atom has no such
section, that absence is itself an EC-003 finding.

**A renewal's three parts are worth writing as three clauses of one sentence**, because CF-38 reads
the whole marker as one string and a reader reads it as three answers: *what experiment*, *what it
would settle*, *whose phase*. The existing markers already do this well — SY-14's is the model
(`spec/SPECIFICATION.md:6234-6260`) — so renew by rewriting the parts that went stale, not by
replacing the form.

**Check the experiment against the tree, not against memory.** SY-18 is the worked case: its
question is untouched by `SyncPeer::limits()` existing, because `PeerLimits::admits` is advisory
(`crates/happenstance-sync/src/peer.rs:150-158`, `:319-324`). The test to apply per clause is: *if
someone ran this experiment tomorrow, would the answer change the clause?* If not, the experiment is
stale even where the string is long.

**`_clause-arithmetic.md` wants five sections and no more**: the re-derived ledger; the five
dispositions (clause, disposition, experiment, what it would settle, owner, authorising ADR or
blank); the citations re-anchored; the arithmetic with both differences; and the findings with
owners. Anything else belongs in the implementation report.

## Tests and CI (merge gate)

Grounded in `…/_decomposition.md`, *Testing brief* — which assigns project AC-010 and AC-014 to the
**Static** tier and names `cargo xtask spec-trace` as the mechanism for both, and confirms there is
no new E2E tier because this project is not terminal (`.redkiln/config.yaml:57-60`; `project.md`,
DoD 1). This story adds **no** unit or integration test, because it compiles nothing — the substitute
obligation is that every disposition names what would falsify it and who would run it, which the
static tier plus the written artifact carry.

| tier | command / path | proves |
| --- | --- | --- |
| Static (story grain) | `cargo xtask affected --base main` | that the diff's blast radius is what the PR boundary claims — no crate is affected, because nothing in the boundary is compiled (`.redkiln/config.yaml`, `affected_gate`) |
| Static (unconditional) | `cargo xtask lints && cargo xtask spec-trace` — the `reachability_static` grain (`.redkiln/config.yaml:48`) | **AC-003** (CF-38 check 2 non-emptiness, `xtask/src/spec_trace.rs:651-670`), **AC-002** (check 4 rule resolution, `:684-720`), **AC-010** (`Mode::Check` region equality, `:735-744`), **AC-001** (`check_stated_census`, `:459-508`) |
| Static (regeneration) | `cargo xtask spec-trace --write` followed by a clean `cargo xtask spec-trace` | **AC-010** — §7.1–§7.2 are generated, not hand-authored; a dirty re-check after `--write` means someone typed into a generated table (`xtask/src/main.rs:671-676`) |
| Static (derivation) | `rg -n '\[DEFERRED' spec/SPECIFICATION.md` and `rg -n 'the phase that builds the two peer adapters' spec/SPECIFICATION.md`, results recorded in `_clause-arithmetic.md` | **AC-001** (the population is five, and which five), **AC-004** (zero surviving hits naming this phase inside an `SY` marker) |
| Static (referent check) | every citation inside a rewritten marker opened and matched by subject string; `spec/E2E-CASES.md:1682`, `references/evaluation/PRESSURE-TEST.md:685-693` | **AC-005** — per `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`; no tool does this, so the evidence is the recorded table |
| Static (the written computation) | `.bklg/…/clause-arithmetic-and-deferral-renewals/_clause-arithmetic.md` §*The arithmetic*, read against the merged atoms under `.kb/decisions/` | **AC-008**, **AC-009** — the union, the two differences, the three contested ids, claims separated from citations |
| Static (diff review) | `git diff spec/SPECIFICATION.md` read against `project.md` DoD 7 and `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`'s mechanical test; `git diff --name-only` | **AC-002**, **AC-006**, **AC-007**, **AC-010** — no MUST, no `Rejects:`, no `Rule:` line, no unauthorised marker move, and **no `RUNBOOK.md`** |
| Integration (project ceiling) | `cargo xtask ci --fast` — `integration_scoped` (`.redkiln/config.yaml:55`), which already contains `spec-trace`, fmt, clippy, docs and the `cargo package --list` assertion | **AC-010**, and project DoD 1 + DoD 2 on the assembled slice |
| KB | `redkiln validate --kb && redkiln doctor` | **NF-005** — no accepted decision atom was edited by this story, and the six expected `template-drift` advisories are still exactly six |
| Ledger | `redkiln verify --grain story` against `_ledger.md` (`.redkiln/config.yaml:62-67`, `require_ledger: true`) | project DoD 3 — every AC-### above carries cited evidence, not a green command standing in for it |

**Merge-gate order**, per the testing brief's *Merge-gate commands*: `affected` → `lints &&
spec-trace` → (`--write` if a marker moved, then `spec-trace` again) → `ci --fast` → `validate --kb
&& doctor`.

## Risks and coupling (PR-scoped)

| risk | why it bites here | mitigation inside this PR |
| --- | --- | --- |
| **The forbidden reconciliation arrives by drift.** Editing `RUNBOOK.md:4535` makes the totals agree and leaves every gate green, because nothing compares a RUNBOOK range to an ADR range. | It is the *cheapest* fix at the moment the arithmetic fails, and the failure arrives late, at the project's exit, under time pressure. | `RUNBOOK.md` is outside the PR boundary; EC-003 names the required output instead; AC-009's verification is a `git diff --name-only` check. |
| **One file, two stories, one slice.** `frozen-clause-repairs` and this story both edit `spec/SPECIFICATION.md` and are mounted as one integrated surface. | Line numbers move under both. A citation this story verifies against HEAD can be stale by the time the slice merges, and the two diffs can silently overlap on the same clause. | Merge order is fixed (repairs first, `…/_storymap.md` §7); this story touches only marker interiors and HS-S0114 touches only `Rule:`/`Rejects:` lines, so the two never share a line; AC-005's checks are re-run against the slice tip, not against HEAD. |
| **A green `spec-trace` reads as a discharged AC.** CF-38 checks `len() < 12` and nothing more. | The project's own testing brief names `spec-trace` as the mechanism for AC-010 and AC-014, which makes a green run look sufficient. | Every AC above pairs the command with a *written* artifact; EC-008 says explicitly that the tool cannot raise the stale-experiment error, and the ledger cites `_clause-arithmetic.md`, not just the command. |
| **The merged ADRs say something other than the plan.** This story reads what landed; the two ADR stories' specs state what was intended. | If ADR-0026 or ADR-0027 merged with a different range, the union is wrong in a way that looks like this story's arithmetic error. | The arithmetic is computed from the merged atoms by `file:line` (AC-008) and every discrepancy with the plan is recorded as a finding with an owner rather than reconciled in either direction. |
| **A settlement taken because the evidence is missing.** SY-27's suite is green because no scoped peer exists. | The temptation peaks exactly here, at the last story of the project, where settling looks like finishing. | AC-006 makes the refusal an acceptance criterion with a written reason; SY-27 and SY-28 are bound to one disposition; phase 14 owns the instrument (`RUNBOOK.md:4645-4649`). |
| **Unblocking a rule and then writing it.** Four rules become writable if their clauses settle. | The rule would be green against every peer in the tree and refutable by none, which is CF-1's worked failure. | The PR boundary excludes every conformance rule, mutant and fixture; the unblocked rule keeps its `(new, …)` marker and is recorded as a handoff (AC-002). |

## Dependencies

**Blocks on** (both hard, both in-project; this story is last in the merge order by design —
`…/_storymap.md`, *Merge order* §7):

- **`frozen-clause-repairs`** (HS-S0114) — the same slice's predecessor. It drops the `(new)`
  markers from `Rule:` lines whose rules now exist and repairs the `Rejects:` fields, so the clause
  text this story computes over is the repaired text. Computing the exit before the repairs means
  re-deriving the ledger against a document that is about to change.
- **`gate-mounts-for-the-sync-suite`** (HS-S0104) — taught `RULE_FILES` about the fourth suite
  (`xtask/src/spec_trace.rs:85-89`). Without it no `SY` clause's `Rule:` name resolves, §7.2 renders
  every one of them `†`, and `spec-trace` cannot produce the evidence this story's ACs cite. EC-007
  is the halt condition.

Transitively, through `frozen-clause-repairs`: `adr-0026-peer-ingest-and-transport` and
`adr-0027-merge-compensation-and-message-set` must be **merged**, not merely written — their stated
ranges are the left-hand side of the arithmetic and their clause-id authorisations are the only
thing that permits a marker to move.

**Unlocks**

- **This project's exit.** Project AC-010 and AC-014 are complete only when this story lands, and
  with them DoD 2 (`lints && spec-trace` green) and DoD 7 (the diff reviewable against the frozen
  clauses). No other story in `…/_storymap.md`'s *Coverage* table owns the CF-38 sweep or the
  arithmetic.
- **`retention-and-incomplete-logs` (HS-P0018).** The renewals re-point SY-27, SY-28 and SY-32 at
  phase 14 and ADR-0028, which is what makes those clauses' experiments actionable by the project
  that owns the suffix store (`RUNBOOK.md:4626-4670`).
- **The initiative's DoD 12 audit** at `closeout-and-durable-audience` — *"no clause is provisional
  with an empty falsifier; the count in the report matches `spec/SPECIFICATION.md`'s own stated
  figure"* (`initiative.md:393-395`). This story is the `SY` family's half of it and corrects the
  stated figure if a marker moved.

## Anchors (progressive disclosure)

Open these at the moment named, not before. Everything load-bearing that this spec compressed is
below; nothing here is required to *start*, and each row says what it is required *for*.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `spec/SPECIFICATION.md` §1.3 (`:204-227`) | The marker's required three-part form, CF-38's justification, the stated census sentence, and CF-25's standing qualification on freezing a port clause. This is the spec of the thing this story edits. | First, before touching any marker. | AC-001, AC-003, AC-004 |
| `spec/SPECIFICATION.md` SY-14 `:6234-6260`, SY-18 `:6362-6392`, SY-27 `:6629-6650`, SY-28 `:6653-6670`, SY-32 `:6775-6800` | The five markers themselves, with their existing experiments, owners and `Rejects:` fields — the exact text being renewed, and SY-28's written-down mutant. | Per clause, immediately before writing its disposition. | AC-002, AC-003, AC-006, AC-007 |
| `spec/SPECIFICATION.md:7002-7003` (§5.13) | States that SY-32 depends on ES-39 and cannot be settled ahead of it — the sentence SY-32's own marker currently contradicts. | When writing SY-32's disposition. | AC-007 |
| `xtask/src/spec_trace.rs:459-508`, `:651-670`, `:684-720`, `:735-744`, `:1626-1634` | What the gate actually checks: the census parser per maturity column, CF-38's `len() < 12` floor, rule resolution, the generated-region equality, and why `(new)` means *scheduled* rather than *dangling*. Read it before assuming a green run means more than it does. | Before the first `spec-trace` run, and again before touching §1.3 or §7. | AC-001, AC-002, AC-003, AC-010 |
| `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` | The mechanical test that separates a repair from an amendment — the admitted-implementation set. It is what makes a renewal ADR-free and a settlement ADR-bound. | Before deciding renewal versus settlement for any clause. | AC-002, AC-010 |
| `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` | *Verify the referent, not just the address* — the discipline for the known-drifted `E2E-CASES.md` citation and for every other citation inside a rewritten marker. | When rewriting any marker that carries a citation. | AC-005 |
| `.kb/playbooks/verify-the-referent-and-report-coverage.md` | How coverage is reported so a third party can re-run it — the shape `_clause-arithmetic.md`'s derivation records should take. | While authoring `_clause-arithmetic.md`. | AC-008, NF-002 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The binding constraint behind the forbidden reconciliation: when a record and reality disagree, the record's *reasoning* is not what gets edited. | The moment the arithmetic fails to come out. | AC-009 |
| `.bklg/…/replication-identity-and-ingest/_decomposition.md`, *Tension 3* (`:302-345`) | The re-derived ledger, the split defect (SY-33/SY-34/SY-35 assigned to nothing), the proposed reassignment, AC-A11/AC-A12, and *"Fix the split; do not fix the total by editing the RUNBOOK."* | Before computing the union. | AC-008, AC-009 |
| `.bklg/…/replication-identity-and-ingest/_decomposition.md`, *Testing brief* (`:603-846`) | Which tier proves which project AC, why there is no new E2E tier, and the merge-gate command order. | When assembling the evidence for the ledger. | AC-010 |
| `.bklg/…/adr-0026-peer-ingest-and-transport/spec.md` | ADR-0026's *stated clause range* section (SY-8 – SY-18 + SY-33 + SY-34) and its AC-002 distinction between cited and claimed clauses — the left half of the union, and the reason a naive union double-counts. | When computing ADR-0026's claim set — then verify against the **merged** atom. | AC-008, AC-009 |
| `.bklg/…/adr-0027-merge-compensation-and-message-set/spec.md` §8 | ADR-0027's stated range (SY-1 – SY-7 + SY-19 – SY-31 + SY-35) with SY-32 subtracted as a named handoff — the right half of the union. | When computing ADR-0027's claim set — then verify against the **merged** atom. | AC-008, AC-009 |
| `.bklg/…/frozen-clause-repairs/spec.md` | The slice-mate's PR boundary, so the two diffs into one file provably do not overlap on a line. | Before writing any marker, and again at slice review. | AC-010 |
| `.bklg/…/gate-mounts-for-the-sync-suite/spec.md` | What `RULE_FILES` was widened to include — the precondition EC-007 halts on. | At preflight, before the first `spec-trace` run. | AC-010 |
| `RUNBOOK.md:305-312`, `:334-336`, `:4516-4620`, `:4626-4670` | The ADR queue's parenthesised ranges, phase 4's proof that a range is a scope statement, the standing rule to compute both numbers at exit, phase 13's stated range, and phase 14's ownership of the suffix store. | Before the union, and when choosing an unrun owning phase. | AC-004, AC-008, AC-009 |
| `crates/happenstance-sync/src/peer.rs:150-158`, `:319-324` | `SyncPeer::limits()` exists and `PeerLimits::admits` is advisory — the proof that SY-18's *type* landed and its *question* did not. | When writing SY-18's experiment. | AC-003 |
| `references/evaluation/PRESSURE-TEST.md:685-693` | The whole-log-versus-scoped experiment that SY-14, SY-27 and SY-28 all cite; the referent that must still say what the marker claims. | When re-anchoring citations. | AC-005 |
| `spec/E2E-CASES.md:1682` | The completeness instrument SY-27 and SY-32 *mean*, at the line it is actually on — the known drift. | When re-anchoring SY-27's and SY-32's citations. | AC-005 |
| `.bklg/…/clause-arithmetic-and-deferral-renewals/discover.md` | The signal ledger, the five clauses quoted at HEAD, and the three named mutants this spec is written against, in more detail than the *Context pack* carries. | If any disposition feels underdetermined. | AC-002, AC-003, AC-006 |
| `.bklg/from-contract-to-published-library/initiative.md:335-338`, `:393-395` | Initiative AC-10 (the evaluator can tell how strong each promise is) and DoD 12 (the clause ledger audited at publish) — the reason a stale deferral is a product defect and not a documentation nit. | When framing the report, or when a disposition looks like busywork. | AC-001, AC-003 |
| `.redkiln/config.yaml:40-67` | Which command runs at which grain, and `require_ledger: true` — why the ledger is the gate rather than the green run. | When filling `_ledger.md`. | AC-010 |

## Clarifications resolved during spec

1. **The AC set is exactly the ten the front half decided** — AC-001 … AC-010 — and none was added
   or dropped. Two behaviour-table rows fold into one AC rather than getting their own: *"a settled
   clause's scheduled rule keeps its `(new)` marker"* is part of **AC-002**, because the handoff is
   part of the disposition and not a separate obligation; and *"the exit diff is reviewable against
   DoD 7 by reading it"* is part of **AC-010**, because the gate command and the human read are the
   same claim about the same diff.
2. **Which of the five settle is deliberately not fixed here.** `discover.md` reasoned that SY-14
   and SY-18 are *"genuinely settleable here"* because this project is their named owning phase. The
   spec does not turn that into an obligation, because the authority is the **merged** ADR text and
   not the plan: AC-002 makes the disposition follow what the record states by clause id, and where
   the record states nothing the disposition is a renewal (Context pack 2). A settlement taken on
   this story's own judgement is `project.md`'s *Out of scope* violation.
3. **The `[DEFERRED]` census figure is 10 document-wide, and 5 in the `SY` family.** §1.3 states 10
   (`spec/SPECIFICATION.md:219-222`); the five this story owns are the `SY` ones. A settlement moves
   the document-wide figure, which is why §1.3 is inside the PR boundary at all — the other five
   deferrals belong to other families and are untouched.
4. **"Renders surfaces: none" is a finding, not an omission.** `_design.md` was checked with
   `test -f` and read: every section reads `N/A — no user-facing surface`, and the sign-off approved
   that determination explicitly. The *Interaction quality* section above therefore maps both RFC
   §6.7/D6 families onto `spec/SPECIFICATION.md`'s own composition — which is a real, tool-enforced
   composition — rather than declaring the section inapplicable.
5. **`_clause-arithmetic.md` is named as a deliverable here but is not yet on disk.** It is created
   by the implementer inside this story's own folder; the front half's PR boundary already lists it,
   and the *Implementation notes* fix its five sections. It is deliberately not an anchor above,
   because an anchor must exist to be cited.
6. **No conformance rule, and no `xtask` change.** CF-38's weakness — non-emptiness, not namedness —
   is *stated* (EC-008) and worked around by hand rather than fixed here. Strengthening the checker
   is a real proposal and is recorded as a finding for a later story; doing it in this PR would put
   a compiled change inside a boundary whose whole claim is that it compiles nothing.
