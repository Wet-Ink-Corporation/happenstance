---
item: HS-S0113
stage: discover
created: 2026-08-12T13:03:35.318Z
updated: 2026-08-12T13:03:35.318Z
template_sig: 86ce4036
rendered_sig: a2e758db
---

# Discover — The deferrals name experiments and the clause arithmetic comes out

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing. This is the
project's exit computation, and it is deliberately last: *"a repair can only name
the rules and symbols that exist, and the arithmetic is an exit computation"*
(`_storymap.md`, *Merge order* step 7).

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: settle or renew every `[DEFERRED]` SY clause against a *named* experiment (a renewal with no experiment is a build failure under CF-38), then compute the union of ADR-0026's and ADR-0027's clause ranges against this project's stated range, write the arithmetic down, and leave `cargo xtask spec-trace` green | `_storymap.md`, *Slices* table, `spec-repairs-and-clause-exit` row 2 | Two deliverables: five deferral dispositions, and one computation written down |
| **Depends on `frozen-clause-repairs` (HS-S0112)** for the repaired clause text and on **`gate-mounts-for-the-sync-suite` (HS-S0104)** for `RULE_FILES`, without which `spec-trace` cannot resolve a single sync rule | `_storymap.md`, *Slices* `depends_on` | The arithmetic runs through a tool that must already know the fourth suite exists |
| **AC-010** — no deferral survives without a named experiment; every `[DEFERRED]` SY clause is settled or renewed, and a renewal with no experiment fails the gate under CF-38 | `project.md`, *Acceptance criteria*, AC-010 | Five clauses, each needing a disposition |
| **AC-014** — this project's stated clause range and the union of the two ADRs' ranges are **computed and equal at exit**, every maturity marker is checked against `spec/SPECIFICATION.md` rather than against the charter, and `cargo xtask spec-trace` is green | `project.md`, *Acceptance criteria*, AC-014 | "Computed", not asserted. Three separate obligations in one AC |
| **DR-9** — every clause is checked against the clause ledger, not against prose; the stated range and the union of the ADRs' ranges are computed and compared at exit — *the standing lesson from phase 4* | `project.md`, DR-9; `RUNBOOK.md:334-336` | The lesson exists because it has been got wrong before |
| *"The parenthesised clause ranges are a scope statement, and phase 4 proved they are not a coverage guarantee."* | `RUNBOOK.md:308-312` | A range in the ADR queue is a claim about intent. This story is what turns it into a claim about coverage |
| **CF-38** — a `[PROVISIONAL]` or `[DEFERRED]` marker with an empty falsifier or experiment is **forbidden and a build failure**, because *"a provisional marker with no falsifier is indistinguishable from a decision nobody wanted to make, and by the time anyone notices it has been load-bearing for a year"* | `spec/SPECIFICATION.md:205-217` | The check is on **non-emptiness**. It cannot check that the experiment is still the right one |
| A deferral must also name an **owning phase**: *"A deferral with no owning phase is a decision the next pass makes by accident."* | `spec/SPECIFICATION.md:203-205` | Three fields per renewal: the experiment, what it would settle, and who owns it |
| The five `[DEFERRED]` SY clauses, re-derived at HEAD: **SY-14, SY-18, SY-27, SY-28, SY-32** | `_decomposition.md`, *Tension 3*; verified against `spec/SPECIFICATION.md` | The full population. `[FROZEN]` is 21 and `[PROVISIONAL]` is 9 |
| **SY-14** — bulk ingest bounded independently of batch size; deferred against *"the experiment named in PRESSURE-TEST.md:688-693 and… the Kestrel Rotor bulk-ingest measurement: 1,840 events over one-shot HTTP inside a 34-minute window, against a store-level `EventId` uniqueness guarantee versus a per-event conditional append. Owning phase: the phase that builds the two peer adapters."* | `spec/SPECIFICATION.md:6234-6260` | **This project is that phase.** SY-14's `Rejects:` prices all three available shapes and says why it is deferred rather than decided |
| **SY-18** — a peer declares its limits; deferred against *"building the Turnstile peer D shape: a KV-backed store with a 128 KiB value cap, against an origin store that has already durably committed a 340 KB payload. The experiment is whether a capability declaration prevents the failure or merely relocates it."* The sketch already built `SyncPeer::limits()` non-`async`, and *"`PeerLimits::admits` is explicitly advisory, so the deferral below is untouched by the type existing"* | `spec/SPECIFICATION.md:6362-6392`; `crates/happenstance-sync/src/peer.rs:150-158`, `:319-324` | A clause whose *type* landed and whose *question* did not. The distinction is the whole trap |
| **SY-27** — whole-log versus scoped replication; deferred against *"the experiment at PRESSURE-TEST.md:688-693: build a spoke holding a deliberately filtered subset of a hub's log and attempt a position-based resume against it"*, with the instrument being a store holding only a suffix or filtered subset (E2E-CASES.md:1595-1600). Rule `scoped_replication_resume_is_sound`, *"unwritable until this is settled"* | `spec/SPECIFICATION.md:6629-6650` | The instrument is the **suffix store**, which is `retention-and-incomplete-logs`' (HS-P0018), not this project's |
| **SY-28** — the round-trip rule asserts log equality only over the agreed scope; *"same experiment as SY-27; this clause is its consequence and cannot be written before it."* | `spec/SPECIFICATION.md:6653-6670` | SY-28 cannot move independently of SY-27. Two clauses, one disposition |
| **SY-32** — the retention floor; *"SY-32 depends on ES-39 and cannot be settled ahead of it"*, and ES-39 belongs to ADR-0028 under `retention-and-incomplete-logs` | `spec/SPECIFICATION.md:6775-6791`, `:7000-7003`; `RUNBOOK.md:307`, `:4637` | Recorded as a **named handoff** by ADR-0026 (HS-S0098). This story confirms the record, it does not re-decide it |
| The arithmetic defect to fix **before** drafting, not after: `RUNBOOK.md:4535` says phase 13 discharges SY-1 – SY-35; the intake split gives ADR-0027 SY-1 – SY-7 and SY-19 – SY-31 and ADR-0026 SY-8 – SY-18, *"which leaves SY-33, SY-34 and SY-35 assigned to nothing"* | `_decomposition.md`, *Tension 3* | Proposed assignment: SY-33/SY-34 to ADR-0026 (transport-refusal, peer-shaped), SY-35 to ADR-0027 (reconciliation-shaped) |
| *"Fix the split; do not fix the total by editing the RUNBOOK."* | `_decomposition.md`, *Tension 3* | The one move that makes the arithmetic come out and means nothing |
| The intake brief's ledger is superseded: its counts were right and its ranges wrong, and it calls the wire clauses `VT-*` when they are `WF-*` | `_decomposition.md`, *Tension 3*; `project.md`, *Risks* row 1 | *"every maturity marker is checked against `spec/SPECIFICATION.md` rather than against this charter"* (AC-014) |
| **AC-A12** — the union of ADR-0026's and ADR-0027's stated ranges equals SY-1 – SY-35 minus SY-32's named handoff, *"computed and written down at exit rather than asserted"* | `_decomposition.md`, AC-A12 | The subtraction of SY-32 is part of the statement, not an exception to it |
| `spec-trace` is a gate step precisely so maturity markers and citations *"cannot rot into decoration"*, and `.redkiln/config.yaml:48` runs it per story | `CLAUDE.md`, *Open questions*; `project.md`, DoD 2 | The evidence command for both ACs is one command that already runs |
| `references/evaluation/PRESSURE-TEST.md` is present in the tree, so the citations resolve | `_decomposition.md`, *Tension 3* | Named experiments that cite a missing file would fail differently and more loudly |

## Questions

**Does ingest re-check the writer's asserted append conditions? — Answered, and
SY-14 is where the pressure to re-open it is highest.** SY-14's `Rejects:` prices
three bulk-ingest shapes and one of them is *"one condition of 1,840
single-identity items"* — a conditional append, on the ingest path, at scale. It
is priced and rejected on its own terms (one already-seen event rejects the other
1,839, and `ConditionViolated.conflicting_position` names at most one culprit, so
recovery is a serial peel), and the third shape's race *"has nothing closing it —
the `AppendCondition` is the only mechanism that could, and SY-1 has just
forbidden using it that way"* (`spec/SPECIFICATION.md:6247-6255`). So settling
SY-14 must not be allowed to smuggle the condition back onto the ingest path; the
surviving candidate is the **store-level `EventId` uniqueness guarantee**, and it
is a contract-crate question this project must be careful about.

**Hub-and-spoke versus peer-to-peer — not this story's**, but SY-27's
confidentiality rationale depends on the answer: Kestrel Cold Chain's spokes hold
a 90-day slice chosen for size *and for commercial confidentiality*, so *"the
scoped answer is not merely an optimisation the deployment could decline"*
(`spec/SPECIFICATION.md:6645-6650`). A renewal must not describe scoping as a
performance question.

**Which of the five settle and which renew? — Deferred to spec, with the
constraints already fixed and only two genuinely open.** SY-32 is a **handoff**,
recorded by ADR-0026 and confirmed here (`spec/SPECIFICATION.md:7000-7003`).
SY-27 and SY-28 are one disposition and their instrument — a store holding only a
suffix or a filtered subset — belongs to `retention-and-incomplete-logs`
(HS-P0018), so on present evidence they **renew** against the same named
experiment with the owning phase updated to name that project. SY-14 and SY-18
name *"the phase that builds the two peer adapters"* as their owning phase, and
that is this project (`durable-object-and-neon-peers`, HS-S0111) — so both are
genuinely settleable here and a renewal would need a reason.

**Is renewing with the same experiment text acceptable? — Answered: only if the
experiment is still the thing that would settle the clause.** CF-38 checks
non-emptiness and cannot check truth. SY-18 is the live example: the clause's type
has *landed* — `SyncPeer::limits()` exists, non-`async`, returning `PeerLimits` —
and the deferral is *"untouched by the type existing"* because
`PeerLimits::admits` is advisory. A renewal that cited "build the limits API"
would now be citing something already built, and CF-38 would pass.

**Does the arithmetic use SY-33 – SY-35? — Answered: yes, and assigning them is
part of the computation.** They were assigned to nothing by the intake split.
This story verifies that ADR-0026 and ADR-0027 as merged actually claim them, and
if they do not, the finding is that the two ADRs' union is short by three — which
is a re-plan input, not something to paper over.

**What if the union does not equal the stated range? — Answered, and the
forbidden move is named.** Fix the split — reassign clauses between the two ADRs
by amendment, or record the shortfall as a finding with an owner. Do **not** edit
`RUNBOOK.md:4535`'s range so the totals agree. Editing the total is the
arithmetic equivalent of deleting a failing test.

## Decision

Two things in this project are true only if someone computes them, and neither has
a tool that computes it automatically. The first is the deferral discipline: five
`SY` clauses carry `[DEFERRED]` markers with named experiments, CF-38 makes an
*empty* experiment a build failure, and nothing anywhere checks that a named
experiment is still the thing that would settle its clause — SY-18 is already in
that position, its API landed and its question untouched. Two of the five (SY-14,
SY-18) name *"the phase that builds the two peer adapters"* as their owning phase
and that is this project, so they are settleable here rather than renewable; two
more (SY-27, SY-28) are one disposition whose instrument belongs to
`retention-and-incomplete-logs`; and SY-32 is a handoff ADR-0026 already recorded.
The second is the clause arithmetic, and phase 4 already proved that a
parenthesised range in the ADR queue is a scope statement rather than a coverage
guarantee — the intake split, taken at face value, leaves SY-33, SY-34 and SY-35
assigned to no ADR at all. This story computes both and writes them down: a
disposition per deferral with its experiment, what it would settle, and its
owning phase; and the union of ADR-0026's and ADR-0027's actual claimed ranges
compared against SY-1 – SY-35 minus SY-32's handoff. The spec stage will cover:
the five dispositions and which are settlements; the re-derived maturity ledger
taken from `spec/SPECIFICATION.md` rather than from any brief; the union
computation with its result stated as a set difference in both directions; the
clause-by-clause assignment of SY-33 – SY-35; the escalation path if the union is
short; and `cargo xtask spec-trace` green as the evidence.

## The wrong implementation

**The mutant is a renewal that satisfies CF-38 and settles nothing, and SY-18 is
already halfway to being it.** Renew a `[DEFERRED]` clause by copying its
existing experiment text forward and bumping the owning phase. CF-38 passes,
because the experiment string is non-empty. `cargo xtask spec-trace` passes,
because it checks that a deferral *names* an experiment. The clause reads as
under active investigation. And in SY-18's case the named experiment — *"whether a
capability declaration prevents the failure or merely relocates it"* — now sits
beside an API that has already been built, so a careless renewal would cite
building something that exists, and the actual question, whether `PeerLimits`
prevents or merely relocates the 340 KB-into-128 KiB failure, would go another
phase unasked. CF-38's own justification is the diagnosis: *"a provisional marker
with no falsifier is indistinguishable from a decision nobody wanted to make, and
by the time anyone notices it has been load-bearing for a year"*
(`spec/SPECIFICATION.md:212-217`). A renewal whose falsifier can no longer
falsify anything is the playbook's definition of a **gap**, not a repair — and a
gap is a decision's (`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`).

**The arithmetic mutant: make the totals agree by moving the total.** Phase 13's
range is SY-1 – SY-35 and the two ADRs' union comes to SY-1 – SY-32 minus a
handoff. The cheapest reconciliation is to edit `RUNBOOK.md:4535` to say SY-1 –
SY-32, and every check in the repository stays green, because no tool compares a
RUNBOOK range to an ADR range — that comparison is precisely what AC-014 asks a
human to compute. Three clauses then belong to nobody: SY-33 and SY-34, which are
the transport-refusal pair (*a peer MAY refuse a push for transport-level or
authorisation-level reasons*; *a spoke whose push is refused MUST leave its own
log unchanged*), and SY-35, which is *"Anything replication must reason about
MUST be in the tags"* — the clause DR-5's no-payload-decoding rule rests on. The
architecture brief names the move and forbids it in four words: *"Fix the split;
do not fix the total by editing the RUNBOOK."*

**And the rule mutant SY-28 has already written down, which this story must not
let through as a settlement.** *"The round-trip rule everyone will write first —
push everything, pull everything, assert the two logs are equal. Against a scoped
spoke it fails for a conformant adapter, which makes it worse than useless: it
will be 'fixed' by weakening it until it passes, and what it is weakened to will
be whatever the first scoped adapter happens to do"*
(`spec/SPECIFICATION.md:6665-6670`). If SY-27 is settled here on whole-log
replication, that rule becomes writable and is green against every peer in the
tree — which is not evidence that whole-log is right, only that no scoped peer
exists yet to refute it. The instrument that could refute it is the suffix store,
and it is HS-P0018's. So a settlement of SY-27 taken on the strength of a green
suite would be a settlement taken because the refuting instrument had not been
built — the same failure CF-1's own worked example describes, where two rules went
four phases without a store that could fail them.

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

**Box 6.** This story adds no conformance rule; its diff is clause markers,
experiment text, and a written computation. It does *unblock* rules by settling
the clauses that schedule them — `bulk_ingest_is_idempotent_in_bounded_round_trips`
(SY-14), `peer_declares_its_own_limits` (SY-18), and, if SY-27 moved,
`scoped_replication_resume_is_sound` and `round_trip_preserves_the_agreed_scope` —
and any such rule inherits CF-6 in the form this project needs most: a
round-trip or bulk-ingest rule compares the **sets of `EventId`s** the two logs
hold, never positions and never counts, because a scoped or gapped peer is
conformant and a position-anchored assertion would fail it (SY-19, SY-28,
`crates/happenstance-testkit/tests/mutation_coverage.rs:326-338`).

**Box 7, with a distinction this story turns on.** No `[FROZEN]` clause is edited
here. What this story *does* edit is `[DEFERRED]` markers, and **moving a
maturity marker is not a repair** — it changes the set of implementations the
clause admits, which is the mechanical test's `yes` branch, so every settlement is
authorised by ADR-0026 or ADR-0027 by clause id, both written first in slice 1
(`_storymap.md`, *Merge order* step 1). Where a settlement is needed that neither
ADR authorises, the required output is a recorded finding plus a new decision atom
and a re-plan, per `project.md`'s *Out of scope*. A renewal, by contrast, leaves
the marker where it is and only updates the experiment and owning phase — a repair
under the playbook, provided the new experiment can actually falsify the clause.
And per box 8: where a clause's named rule is found to be unwritable as specified
— SY-27 says `scoped_replication_resume_is_sound` is *"unwritable until this is
settled"* — the clause is fixed with its reason in the same change, never left
citing a rule nobody can write.
