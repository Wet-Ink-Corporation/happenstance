---
item: HS-S0112
stage: discover
created: 2026-08-12T13:03:34.156Z
updated: 2026-08-12T13:03:34.156Z
template_sig: 86ce4036
rendered_sig: e3e3e525
---

# Discover — Every clause still rejects something that exists

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing. **This is the
story that owns the specification repair** the project charter calls for — the
clauses whose words are now wrong, and the clauses whose named wrong
implementation is `happenstance-sync`'s own superseded proposal.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: re-run the `re-check\|recheck\|re-evaluat` sweep and record the finding either way, drop the `(new)` markers from the `Rule:` lines whose rules now exist, and repair — in the playbook's three-part form, MUST verbatim — every clause whose `Rejects:` names a symbol this project changed, each repair authorised by ADR-0026 and justified by the mechanical test that the admitted implementation set is unchanged | `_storymap.md`, *Slices* table, `spec-repairs-and-clause-exit` row 1 | Three deliverables: a recorded sweep, the marker drops, and the `Rejects:` repairs |
| **Depends on `headline-rules-and-mutant-registry` (HS-S0105)**, because a `(new)` marker may only come off once the rule it schedules exists, and on **`adr-0026-peer-ingest-and-transport` (HS-S0098)**, which authorises each repair | `_storymap.md`, *Slices* `depends_on`; `_storymap.md`, *Merge order* step 7 | *"Last on purpose: a repair can only name the rules and symbols that exist"* |
| **AC-012** — the corrections go through decision records, not edits. Each clause *"either keeps a wrong implementation that still exists, or is corrected by decision record"*, audited **by id** | `project.md`, *Acceptance criteria*, AC-012 | The audit is by id and pre-committed, not a hunt (`_decomposition.md`, *Tension 4*) |
| **AC-002**, this story's share — the proof that no frozen clause was edited in this project's diff | `project.md`, *Acceptance criteria*, AC-002; `_storymap.md`, *Coverage* AC-002 row | DoD 7: `git diff` over `spec/SPECIFICATION.md` shows only additions a decision record authorises |
| **The sweep, re-run at HEAD for this discover pass.** `re-check\|recheck\|re-evaluat` returns ten hits, of which five are about ingest: `:4373` (ES-38's `Rejects:`), `:5825` (the Cold Chain paragraph), `:5862` (SY-1's `Rejects:`), `:5998` and `:6014` (SY-6's `Rejects:` and its second defect). The other five are unrelated — a falsifier at `:3086`, fixture re-evaluation at `:4063`, `:7508`, `:7538`, and an ES-42 marker note at `:9028` | Verified against `spec/SPECIFICATION.md` at HEAD, 2026-08-12; matches `_decomposition.md`, *Tension 4* | **Every one of the five reads as consistent with SY-1/SY-6 as frozen.** No clause still describes ingest as re-checking conditions |
| `:4373` — ES-38's `Rejects:` *presupposes* unconditional ingest and calls the vacuous pass *"the **normal** path"*; `:5825` distinguishes Cold Chain's *"re-checking"* as *"a domain decision about facts already accepted, running after the ingest, not a gate in front of it"* | `spec/SPECIFICATION.md:4373`, `:5798-5836` | The convergence argument and its three apparent counter-examples are already written. Nothing to correct there |
| **The live half is the inverse:** SY-1 and SY-6 both name the **public** `guard: Option<AppendCondition>` field on `EventGroup` as the thing that makes re-evaluation reachable — *"prose is not a type constraint… a receiver is handed exactly the value it would need to re-evaluate"* | `spec/SPECIFICATION.md:5856-5867`, `:5998-6006`; `crates/happenstance-sync/src/peer.rs:236-243` | If ADR-0026 or ADR-0027 makes `guard` private, renames or removes it, both clauses lose their named wrong implementation **in the same commit** |
| **SY-12 has already been through this once and records it in its own text:** *"The exemplar was the sync crate's own proposal and it was withdrawn: the crate now mints `(StoreId, SequencePosition)`… The rejection keeps a live target anyway, and phase 2 sharpened rather than removed it"* — the current target being that `impl IngestStore for MemoryEventStore` cannot be written truthfully | `spec/SPECIFICATION.md:6173-6183`; `crates/happenstance-sync/src/identity.rs:87-124`; `crates/happenstance-sync/src/ingest.rs:38-50` | **`memory-store-ingest-seam` and `ingest-store-and-memory-peer-round-trip` remove that target.** SY-12's `Rejects:` needs a new live one or a recorded discharge |
| The `(new)` mechanism: `spec-trace` treats a `Rule:` line containing `(new)`, `†`, a leading `new `, `" new \`"`, or the words *unit test / compile test / meta-test* as **scheduled**, so the rule is never resolved | `xtask/src/spec_trace.rs:1626-1634` | *"The moment `ingest_never_rejects` and `compensation_is_atomic_with_the_losing_event` exist, SY-1's and SY-2's `Rule:` lines must lose `(new)` or `spec-trace` keeps reporting them unwritten"* |
| The `(new)` drop is a **repair**, not an amendment, by the playbook's mechanical test: the set of implementations the clause admits is unchanged. *"Say so in ADR-0026 so that DoD 7's… is satisfied by an authorisation rather than by an argument after the fact"* | `_decomposition.md`, *Tension 4* | The authorisation is ADR-0026's, written first in slice 1 |
| The playbook's test, verbatim: *"a correction to a `[FROZEN]` clause is a repair if the set of implementations the clause admits is unchanged. Otherwise it is a gap, and a gap is a decision's."* And: *"is there an implementation that was conformant before the edit and is not after, or vice versa? If no, edit freely. If yes — or if you cannot tell — you have found a gap, and the correct output is a recorded finding, not a smaller edit."* | `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`, *The test: repair or gap* | Applicable by someone not authorised to decide anything. "Cannot tell" counts as a gap |
| The playbook's taxonomy — **repairs:** a citation pointing at the wrong line, prose describing a superseded implementation, a named rule that exists under a different name or file, a self-referential count, tense describing a shipped plan as future. **Gaps:** the MUST admits an implementation the named rule rejects, the MUST is silent about something a rule enforces, a maturity marker whose falsifier can no longer falsify anything | `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`, same section | Most of this story's work is squarely in the first list |
| The safe form for a discharged MUST: *"The most tempting edit is deleting an obligation that has been met — almost always wrong… the obligation's bare absence reads as an oversight."* Keep the MUST verbatim, name the discharge as a discharge, cite the code and the test that assert it | `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`, *The safe form for a discharged MUST* | Three-part form. Deletion is the failure mode it is written against |
| **AC-A11** — every clause whose `Rejects:` names a symbol this project changed has a repair **in the same commit**, and each repair's justification names the mechanical test | `_decomposition.md`, AC-A11 | Same commit, not a follow-up |
| Citation anchoring is its own discipline in this repository | `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`; `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | A repair that re-points a citation must verify the new referent says what the clause claims |
| The code prose these clauses cite by line, and which this project rewrites: `peer.rs:230-243` (guard as evidence), `lib.rs:100-104` (positions are within one store), `ingest.rs:38-50` (the untruthful impl), `lib.rs:116-119` (the central question), `tests/real_peer_shapes.rs` (the two stand-ins) | `_decomposition.md`, *Notes*; the earlier stories' discovers | Each is a citation that may go stale in this project's own diff. The audit is over this list |
| Out of scope for the whole initiative: **amending** any `[FROZEN]` clause. *"If this work needs one changed, that is a new decision atom and a re-plan, not a line edit."* | `project.md`, *Out of scope* | The stop condition, stated at project grain |

## Questions

**Does ingest re-check the writer's asserted append conditions? — Answered, and
this story records the audit that proves the specification says so consistently.**
The sweep was re-run at HEAD for this pass and the result is a **null finding**:
ten hits, five about ingest, and all five consistent with SY-1 and SY-6 as
frozen. The implementer re-runs it at implementation time and records the result
either way — *"a null finding recorded is worth more than a null finding
assumed"* (`_decomposition.md`, *Tension 4*). The charter's phrase "the clauses
framing ingest as re-checking conditions" therefore resolves, on present evidence,
to **no clause**; what is live is the opposite defect, below.

**Hub-and-spoke versus peer-to-peer — not this story's**, except as a source of
stale citations if ADR-0027's answer changed the module documentation SY-9 or
SY-10 cite (`spec/SPECIFICATION.md:6120-6121` cites
`crates/happenstance-sync/src/lib.rs:57-63`). That citation is in the audit list.

**Which clauses are in the audit, by id? — Answered, and the list is a
pre-commitment rather than a hunt.** SY-1 and SY-6 (public `EventGroup::guard`);
SY-12 (the `impl IngestStore for MemoryEventStore` target that
`memory-store-ingest-seam` and `ingest-store-and-memory-peer-round-trip`
remove); SY-1 and SY-2's `(new)` markers, plus every other `SY`/`WF` clause whose
scheduled rule this project landed; SY-9 and SY-10's citations into
`lib.rs`'s module documentation; SY-15 and SY-17's references to the stand-ins in
`tests/real_peer_shapes.rs`; and SY-6's citation of `lib.rs:100-104`. Each is
checked, and each either keeps a target that still exists or gets a repair.

**Is dropping a `(new)` marker a repair or an amendment? — Answered: a repair,
by the mechanical test.** The set of implementations the clause admits is
identical before and after; what changes is whether `spec-trace` can resolve the
rule name. ADR-0026 says so explicitly, so DoD 7's *"only additions a decision
record authorises"* is satisfied by an authorisation rather than by an argument
made afterwards.

**What happens if a clause's `Rejects:` target genuinely no longer exists and no
replacement is available? — Deferred to spec, with the two admissible outcomes
named.** Either the clause records the **discharge** in the playbook's three-part
form — MUST verbatim, the discharge named as a discharge, the code and test that
assert it cited — or, if removing the target means the MUST now admits something
it used to forbid, that is a **gap**, and the output is a recorded finding plus a
new decision atom and a re-plan. Not a smaller edit.

**Does this story touch `[PROVISIONAL]` or `[DEFERRED]` markers? — No.**
Maturity-marker movement and the deferral renewals are
`clause-arithmetic-and-deferral-renewals`' (HS-S0113). This story touches
`Rule:` and `Rejects:` fields and citations only.

## Decision

A `[FROZEN]` clause is protected in its normative sentence and nowhere else, and
what this repository has learned is that the unprotected parts are where clauses
die: a `Rejects:` field naming an implementation that no longer exists rejects
nothing, and a `Rule:` field carrying `(new)` is reported as unwritten forever
even after the rule ships. This project is unusually good at producing both. It
rewrites the exact prose SY-1, SY-6, SY-9, SY-10, SY-12, SY-15 and SY-17 cite by
line; it may make `EventGroup::guard` private, which would delete the public field
SY-1 and SY-6 name as the thing that makes re-evaluation reachable; it makes
`impl IngestStore for MemoryEventStore` truthful, which removes the live target
SY-12 was sharpened onto after its **first** exemplar — the sync crate's own
superseded proposal — was withdrawn; and it lands the rules that three frozen
clauses schedule with `(new)`. So this story is the audit and the repair: re-run
the ingest sweep and record the finding either way, drop every `(new)` marker
whose rule now exists, and repair every clause whose `Rejects:` names a symbol
this project changed — MUST verbatim, discharge named as a discharge, code and
test cited — with each repair authorised by ADR-0026 and justified by the
playbook's mechanical test. Where the test says *gap*, the output is a recorded
finding, a new decision atom and a re-plan. The spec stage will cover: the audit
list by clause id with the current state of each named target; the sweep's
recorded result; the `(new)` drops enumerated; each repair in the three-part
form with its mechanical-test justification; the citation re-anchoring checked
against the new referents; and the `git diff` assertion DoD 7 names.

## The wrong implementation

**The mutant is a clause that passes every check and rejects nothing, and there
are two ways to build it.** The first is doing nothing: leave SY-12's `Rejects:`
pointing at *"`impl IngestStore for MemoryEventStore` cannot be written
truthfully because `SequencedEvent` has nowhere to hold an accepted `EventId`"*
after this project has made exactly that impl truthful. `cargo xtask spec-trace`
is green, because it resolves rule names and citation anchors, not the *existence
of a wrong implementation*. `redkiln validate --kb` is green. The clause reads
authoritatively and forbids a thing nobody can do any more, which means the
metadata-borne identity it exists to prevent is now guarded by a sentence about a
compile error that no longer occurs. SY-12's own text is the warning, because it
has already survived one withdrawal: its **original** exemplar was the sync
crate's own proposal, it was withdrawn, and the clause was re-pointed rather than
retired (`spec/SPECIFICATION.md:6173-6183`). Doing that a second time is this
story's job; not noticing is the mutant.

**The second way is the cheap repair, and it is worse because it looks like
work.** Replace the withdrawn target with a generic restatement — *"rejects a
peer that carries identity somewhere unreachable"* — or delete the `Rejects:`
field entirely on the grounds that the obligation has been met. Both leave a
clause that is syntactically complete, passes `spec-trace`, and names no
implementation anyone would have written, which is CF-4's saboteur problem one
level up: *"the mutant that earns its place is the one someone would ship"*
(`spec/SPECIFICATION.md:7208-7215`). The playbook's answer is the three-part
form: keep the MUST verbatim, **name the discharge as a discharge**, and cite the
code and the test that now assert it — *"the record of what was once required, and
what discharged it, is what a future reader needs; the obligation's bare absence
reads as an oversight"*.

**The third mutant is a repair that is quietly an amendment.** Softening SY-6's
*"MUST be refused as ingest input"* to *"SHOULD be refused"* because the landed
implementation only warns; or narrowing SY-1's *"any reason that is a function of
the receiving store's state"* to *"any `AppendCondition`"* because that is what
the rule checks. Both read as tidying. Both change the set of implementations the
clause admits, which is precisely the mechanical test's `yes` branch — and the
correct output there is a recorded finding and a re-plan, not a smaller edit. The
tell is that the edit makes the clause agree with the code; a repair makes the
clause's *description* agree with the code and leaves its *demand* alone.

**And the citation mutant: re-anchor without reading the new referent.** SY-6
cites `crates/happenstance-sync/src/lib.rs:100-104` for "a position is meaningful
only inside the store that assigned it", and this project rewrites that module
documentation wholesale. Re-pointing the citation at whatever now occupies those
lines produces a clause that cites a real file at a real line saying something
else — undetectable by any tool, since the anchor resolves. This repository has a
playbook for exactly that failure
(`.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`), and the
governance rule is to rewrite the referent, never the reasoning
(`.kb/governance/rewrite-the-referent-never-the-reasoning.md`).

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

**Box 6.** This story adds no conformance rule; its diff is `spec/SPECIFICATION.md`
prose fields plus the ADR that authorises them. Ticked as vacuously true, with
one carried obligation: where a repair *cites* a rule's assertion as the discharge
of a MUST, the citation must name an assertion that itself honoured CF-6 — a
repair that pointed at a literal-position assertion would launder a defect into
the specification as evidence.

**Box 7, and this is the box this story exists to make honest.** Every
`[FROZEN]` clause this story touches is touched **only** in its unprotected parts
— `Rule:`, `Rejects:`, citations — and every such change is a **repair** under
`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`'s mechanical
test: the set of implementations each clause admits is identical before and
after. Each repair is authorised by name in **ADR-0026**, which is written first
in slice 1 and is this story's direct `depends_on` edge, so DoD 7's *"only
additions a decision record authorises"* is satisfied by an authorisation and not
by an argument made afterwards. Where the mechanical test returns *yes* — or
where the implementer cannot tell — the clause is a **gap**, and the required
output is a recorded finding plus a new decision atom and a re-plan, which is
`project.md`'s *Out of scope* stop condition and not something this story may
absorb. And per box 8: if a rule this story is re-pointing a clause at turns out
to be the wrong rule, the rule is fixed and the reason given in the same change,
never worked around in the clause.
