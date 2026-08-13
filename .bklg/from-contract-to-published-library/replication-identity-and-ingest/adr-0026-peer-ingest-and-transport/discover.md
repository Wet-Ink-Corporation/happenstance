---
item: HS-S0098
stage: discover
created: 2026-08-12T13:03:17.691Z
updated: 2026-08-12T13:03:17.691Z
template_sig: 86ce4036
rendered_sig: 8b0f4c1b
---

# Discover — ADR-0026: what a peer is, what the port may assume, what ingest promises

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing. This story is
the root of the project DAG — it has **no `depends_on` edges**, and every other
story in the project depends on it transitively, because nothing in a `.rs` file
a clause constrains may merge before this slice does
(`_storymap.md`, *Merge order*, step 1).

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: re-derive the SY/WF ledger at HEAD, then author ADR-0026 (long form under `references/adr/`, atom via `.kb/_intake/` + `/redkiln:kb-ingest`) answering what a peer is, what the port may assume about an unseen transport, and what ingest promises | `_storymap.md`, *Slices* table, `decisions-of-record` row 1 | The deliverable is a decision record and its long-form transcript, not code. The ledger re-derivation happens **before** drafting, not after |
| **AC-001** — the decisions exist before the code, are accepted atoms under `.kb/decisions/`, state the alternatives that lost, and pass `redkiln validate --kb` | `project.md`, *Acceptance criteria*, AC-001 | The atom must arrive through `.kb/_intake/` and `/redkiln:kb-ingest`; hand-authoring was reverted once already (`0269720`) |
| **AC-002** — the central question is answered in ADR-0026 and *reconciled against* SY-1 and SY-6, both `[FROZEN]` | `project.md`, *Acceptance criteria*, AC-002 | Reconciliation and citation, not fresh deliberation. Re-opening either clause is out of scope for the whole initiative |
| **AC-010** — WF-1's interoperability half is recorded in ADR-0026's envelope section, settled or renewed **by name**, never by silence | `project.md`, *Acceptance criteria*, AC-010; `RUNBOOK.md:4593-4595` | A `[DEFERRED]` marker with no named experiment is a build failure under CF-38 |
| **AC-011** — SY-32's disposition is written down: evidence it is settleable without ES-39, or an explicit handoff to ADR-0028 with the constraint stated | `project.md`, *Acceptance criteria*, AC-011 | Structurally a handoff. The `replication → retention` edge is confirmed, not flipped |
| **AC-014** — this project's stated clause range and the union of the two ADRs' ranges are computed and equal at exit | `project.md`, *Acceptance criteria*, AC-014 | ADR-0026 must state its clause range explicitly so the arithmetic has two operands |
| The central question, stated as a position rather than an answer, in the crate's own words: *"Append conditions across a boundary. Still* the *central design question…"* | `crates/happenstance-sync/src/lib.rs:116-119` | This is the prose ADR-0026 consumes and then deletes; the *finding* is not the same as the `todo!()` |
| **SY-1 `[FROZEN]`** — ingest MUST NOT refuse for any reason that is a function of the receiving store's state, and MUST NOT evaluate an `AppendCondition` — its own or the origin's — as a precondition | `spec/SPECIFICATION.md:5841-5867` | The central question is already answered normatively. Rule: `ingest_never_rejects` |
| **SY-6 `[FROZEN]`** — a wire-carried condition is evidence, not an instruction; a guard whose `after` is `Some(_)` MUST be refused as ingest input, with the two independent fatal defects spelled out | `spec/SPECIFICATION.md:5973-6029` | Supplies the *reasons* the ADR must carry: the vacuous pass over an arbitrary tail, and the non-idempotent inversion at `after: None` (E2E-33) |
| **SY-8 `[FROZEN]`** — the port describes exactly one peer relationship; fan-out, ordering and reconciliation live in a runner above it | `spec/SPECIFICATION.md:6070-6086` | Bounds what "what a peer is" may mean. A `merge_policy` associated type is already forbidden |
| **SY-9 `[FROZEN]`** — hub-ness MUST NOT be a property of the port's type or constructor; one adapter type serves both roles at once | `spec/SPECIFICATION.md:6090-6106` | The topology *shape* question is SY-10's and ADR-0027's; SY-9 is settled and ADR-0026 cites it |
| **SY-32 `[DEFERRED]`**, and the section that says why it cannot move: *"ES-39 defers the primitive that lets a store say what it does not hold. SY-32 depends on ES-39 and cannot be settled ahead of it."* | `spec/SPECIFICATION.md:6775-6791`, `:7000-7003` | AC-011's answer is a named handoff to ADR-0028, whose owner is `retention-and-incomplete-logs` (`RUNBOOK.md:307`) |
| **WF-1 `[DEFERRED]`** — the format is private and DCB interoperability is deferred, on the ground that the DCB reference publishes **no** wire format at all | `spec/SPECIFICATION.md:1892-1921` | The deferral is stronger, not weaker. The named experiment is a specific external implementation to interoperate with |
| **CF-38** — a `[PROVISIONAL]` or `[DEFERRED]` marker with an empty falsifier or experiment is a build failure, not a convention | `spec/SPECIFICATION.md:205-217` | Whatever renewal ADR-0026 records must name its experiment or `spec-trace` rejects it |
| ADR queue rows: 0026 = *"What is a sync peer — what may the port assume about a transport it cannot see, and what does ingest promise? (SY-8 – SY-18)"*; 0027 takes SY-1 – SY-7 and SY-19 – SY-31 | `RUNBOOK.md:305-306` | The stated split leaves SY-33, SY-34 and SY-35 assigned to nothing. Fix the split, not the total |
| Re-derived ledger at HEAD: `[FROZEN]` (21) SY-1–6, 8, 9, 11–13, 15–17, 19, 24–26, 33–35; `[PROVISIONAL]` (9) SY-7, 10, 20–23, 29–31; `[DEFERRED]` (5) SY-14, 18, 27, 28, 32. The wire clauses are `WF-*`, not `VT-*` | `_decomposition.md`, *Tension 3* | The intake brief's ranges are superseded by this. Re-verify at spec against `spec/SPECIFICATION.md`, which wins |
| Phase-13 work list: ADR-0026 must be written against **two** unlike peers — a Durable Object over a socket and a Postgres over one-shot HTTP — because *"a `SyncPeer` that cannot be implemented by the second is a `SyncPeer` shaped like the first"* | `RUNBOOK.md:4562-4566`, `RUNBOOK.md:4593-4595` | Spread, not count, is the evidence standard. Phase 2's sketch is the transcript |
| The transport assumption already paid for: `pull` returns a bounded batch and an owned resume token because a one-shot-HTTP peer has no connection, session or transaction to hold a cursor in — and **the type checker did not force that choice** | `crates/happenstance-sync/src/peer.rs:30-50`; `crates/happenstance-sync/src/lib.rs:126-132`; `crates/happenstance-sync/tests/cursor_shape_probe.rs` | ADR-0026 cites the probe for why `pull` returns a batch. The probe must keep compiling |
| `Resume` is an opaque associated type, owned and `Clone`, *"never a handle the peer holds"* — because the normal termination path at the edge is the handle going away | `crates/happenstance-sync/src/peer.rs:95-118` | An answer already embodied in the sketch that ADR-0026 records rather than re-derives |
| ADR-0013: the visibility invariant is global rather than per-boundary, and a `SequencePosition` is a within-one-store visibility predicate, **not an identity** | `.kb/decisions/0013-position-assignment-and-visibility.md:24-30` | The textual source for this project's title. ADR-0026 cites it; it does not re-derive it |
| ADR-0016 lands the envelope and explicitly **not** the replication protocol, message set or `SyncError` extension — those are named ADR-0027's and ADR-0026's | `.kb/decisions/0016-the-wire-format.md`; `.kb/open-questions/sync-message-set-and-format-version.md:19-23` | `SyncError`'s extension is ADR-0026's to size. `WireError` stays a decoding failure and is not folded in |
| ADR-0001 and ADR-0009 already bind both new ports: no `#[async_trait]`, two flavours via `trait_variant`, `Error: core::error::Error + 'static` and nothing stronger | `.kb/decisions/0001-async-port-flavours.md`; `.kb/decisions/0009-error-send-sync.md`; `spec/SPECIFICATION.md:7006-7019` | ADR-0026 *"neither adds to it nor may diverge from it."* The risk is a runner that adds `+ Send + Sync` on the way past |
| Atoms are authored by `/redkiln:kb-ingest` from `.kb/_intake/`, never by hand; a decision lives in two places on purpose — the atom in `.kb/decisions/`, the long form in `references/adr/` | `CLAUDE.md`, *Where the work lives*; `.kb/playbooks/one-decision-per-adr-title.md` | Two artefacts per ADR, and the atom is the one `validate --kb` enforces |
| The residual risk this ADR is a scheduled mitigation for: a store-side seam discovered late is a breaking change to a *published* port | `RUNBOOK.md:462-466` | If a *real* peer needs a seam on the **port**, that is the risk landing hard: stop and raise a new atom plus a re-plan |

## Questions

**Does ingest re-check the writer's asserted append conditions? — Answered: no,
and the answer is not this story's to make.** SY-1 and SY-6 are both `[FROZEN]`
and both say so already (`spec/SPECIFICATION.md:5841-5867`, `:5973-6029`).
ADR-0026's job is reconciliation and citation. What the ADR owes, because an ADR
that only cites is not an ADR, is **the alternative that lost and why**:
re-evaluation of the origin's condition at the receiver, which is the natural
first cut because the condition arrives on the wire already and the receiving
store's `append` will happily take it. It loses on two independent and
individually fatal defects, both recorded verbatim in SY-6 and neither invented
here — `after` is a `SequencePosition` meaningful only inside the store that
assigned it and serialising as a naked integer, so a foreign `after` names an
unrelated local event and the check passes **vacuously**, which is worse than no
check because it looks like enforcement; and even at `after: None`, re-evaluation
is not idempotent, because on re-delivery the condition matches the receiver's
own already-accepted copy and the receiver adjudicates *against the event it just
accepted* — an inversion, not a duplicate (E2E-33). SY-2 is the frozen answer to
what happens instead: losing event and compensation in one atomic `append`.

**Is hub-and-spoke one abstraction with peer-to-peer, or two? — Deferred to
ADR-0027** (`adr-0027-merge-compensation-and-message-set`, HS-S0099), which is
where `RUNBOOK.md:306` puts it and where SY-10 `[PROVISIONAL]` lives. ADR-0026
records only the half that is already frozen — SY-9, hub-ness is a property of an
**edge**, not of a node — and states the handoff so the two ADRs do not both
answer it, which is what `.kb/playbooks/one-decision-per-adr-title.md` forbids.

**Which clauses does ADR-0026 own? — Answered provisionally, verified at spec.**
SY-8 – SY-18 per `RUNBOOK.md:305`, **plus SY-33 and SY-34**, which the intake
brief's split assigned to nothing and which are transport-refusal clauses and so
peer-shaped (`_decomposition.md`, *Tension 3*). SY-35 goes to ADR-0027. SY-32
leaves the range as a named handoff rather than as a settled clause. The union is
recomputed at exit by `clause-arithmetic-and-deferral-renewals` (HS-S0113); this
story's obligation is to state a range that can be added up.

**WF-1's interoperability half — answered as a renewed deferral with a named
experiment.** The experiment is a *specific external implementation to
interoperate with*, and there is none: the DCB specification and its reference
TypeScript library publish no wire format at all
(`spec/SPECIFICATION.md:1892-1921`). Recorded in ADR-0026's envelope section per
`RUNBOOK.md:4593-4595`, which is also where
`.kb/open-questions/dcb-reference-publishes-no-wire-format.md` resolves
(`open-questions-resolved-and-indexed`, HS-S0100, does the atom half).

**SY-32 — answered as a handoff, not a settlement.** ADR-0026 records the
dependency on ES-39 and ADR-0028 **by name**, and confirms the
`replication → retention` DAG edge is already ordered correctly rather than
flipping it. `PeerLimits::retention_floor` already exists on the port
(`crates/happenstance-sync/src/peer.rs:296-301`); what is missing is the
store-side primitive, which is not this project's.

**How much does `SyncError` grow? — Deferred to spec.** ADR-0016 deliberately
left it untouched and named the extension as ADR-0026's
(`crates/happenstance-sync/src/lib.rs:86-96`). The constraint carried forward is
that `WireError` stays a decoding failure and is not folded in.

**Does ADR-0026 make `EventGroup::guard` private, rename it, or remove it? —
Deferred to spec, with a consequence that must be decided in the same breath.**
SY-1 and SY-6 both name that **public** field as the live wrong implementation
they reject (`crates/happenstance-sync/src/peer.rs:236-243`). If ADR-0026 closes
the field, both clauses lose their named wrong implementation *in the same
commit* and become decorative. The repair belongs to `frozen-clause-repairs`
(HS-S0112) and it is **ADR-0026 that must authorise it**, in the form
`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` prescribes.
Whichever way the field goes, the ADR says so out loud.

## Decision

`crates/happenstance-sync` is a phase-2 instrument whose most honest sentence is
that its central question is unanswered: it carries the origin's condition across
the boundary as `EventGroup::guard` and documents it as evidence rather than as
an instruction, *"which is a position rather than an answer"*
(`crates/happenstance-sync/src/lib.rs:116-119`). Meanwhile the specification has
already frozen the answer in SY-1 and SY-6, and nothing in the knowledge base
records it — so the repository holds a normative answer with no decision atom
behind it and a port crate whose prose still calls the question open. This story
closes that gap in the direction the constitution requires: a decision record
that **reconciles with** the frozen clauses, states the alternative that lost
(receiver-side re-evaluation) with the two defects that killed it, and settles
what the port may assume about a transport it cannot see, using the phase-2
sketch's own transcripts as evidence rather than argument. The spec stage will
cover: ADR-0026's clause range and its complement with ADR-0027, computed rather
than asserted; the answer to the central question with SY-1 and SY-6 cited by
line; the transport floor, sourced from `peer.rs:30-50` and
`tests/cursor_shape_probe.rs` — bounded batch, owned `Clone` resume token, one
round trip per call by construction; the envelope section carrying WF-1's
renewed deferral against its named experiment; SY-32's handoff to ADR-0028 and
the confirmation that the `replication → retention` edge already points the right
way; the disposition on `EventGroup::guard` together with the repair authorisation
`frozen-clause-repairs` will need; the `SyncError` extension's size; and the
two-artefact shape — the long form under `references/adr/0026-*.md` carrying the
transcripts, the atom authored into `.kb/decisions/` by `/redkiln:kb-ingest` and
never by hand.

## The wrong implementation

**The mutant this decision exists to forbid, and where it must live.**
`PortablePositionPeer`, in `crates/happenstance-sync-testkit/tests/` — a peer
whose `Resume` is a `SequencePosition` (or a `Watermark` collapsed to a scalar)
and whose `pull` means "send me everything after N", interpreted in the *caller's*
numbering. It implements `SyncPeer` completely, it is `Send`, it compiles, it
round-trips a small fixture correctly whenever both stores happen to have been
appended to in lockstep, and it passes every event-store conformance rule because
it never touches `EventStore`. It is wrong by construction: two instances that
each append independently assign the same positions to different events
(`crates/happenstance-sync/src/lib.rs:100-104`), so `after: 288455` in the
receiver's numbering names an unrelated recent event and the pull silently skips
or re-delivers an arbitrary tail. This is the field's documented
duplicate-delivery bug and it is exactly what a position-typed resume token buys.
The rule it must fail is the one this ADR's clause range names, and the mutant's
`provenance` string is not hypothetical: "send everything after position N" is
what every log-shipping protocol in the world does, and it is the first thing an
implementer reaches for.

**The second mutant is the ADR itself, and it is the likelier one.** An ADR-0026
that answers the central question *afresh* — reaching the same conclusion, in its
own words, without citing SY-1 or SY-6 — passes `redkiln validate --kb`, passes
`redkiln doctor`, passes `cargo xtask spec-trace` (which checks clause-to-rule
resolution, not clause-to-atom agreement), and merges. It is still wrong, because
a restatement in weaker words is an amendment to a `[FROZEN]` clause performed
without the ADR that a frozen clause requires, and the next reader now has two
normative texts that will drift. The guard is mechanical and cheap: every
normative sentence in ADR-0026 about ingest carries a `spec/SPECIFICATION.md`
line citation, and the clause range the ADR claims is written down so
`clause-arithmetic-and-deferral-renewals` can add it up.

**The third is a clause that survives its own target.** SY-1 and SY-6 currently
reject a receiver that re-evaluates the origin's condition, and both name the
**public** `EventGroup::guard` field as the thing that makes the mistake reachable
(`spec/SPECIFICATION.md:5856-5867`, `:5998-6014`;
`crates/happenstance-sync/src/peer.rs:236-243`). SY-12 has already been through
this once — its original exemplar *was* `happenstance-sync`'s own superseded
proposal, and the clause records the withdrawal in its own text
(`spec/SPECIFICATION.md:6173-6183`). A named wrong implementation that no longer
exists rejects nothing. So if ADR-0026 closes the field, it must authorise the
repair in the same decision, and `frozen-clause-repairs` (HS-S0112) is where the
repair lands.

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

**Box 6, and it is the load-bearing one for this project.** This story adds no
conformance rule — it *names* the rules SY-8 – SY-18, SY-33 and SY-34 already
schedule. It also makes the constraint stronger than the lint: a position is not
merely a bad literal to assert on, it is **not portable across a store boundary
at all** (`.kb/decisions/0013-position-assignment-and-visibility.md:24-30`), so
ADR-0026 forbids any rule whose assertion is anchored on a position value that
crossed the boundary, not just on a written-out integer. CF-6 and
`lint-position-literals` catch the spelling; this clause range catches the idea.

**Box 7.** This story *is* the new ADR, and it is written first — it is the root
of the merge order and no `.rs` file a clause constrains may merge before it
(`_storymap.md`, *Merge order*, step 1). It edits no `[FROZEN]` clause: SY-1,
SY-6, SY-8, SY-9 and SY-32 are cited, and the one correction it foresees — the
`Rejects:` field of a clause whose named target it may close — is authorised here
and executed by `frozen-clause-repairs` under
`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`, whose mechanical
test (is the admitted implementation set unchanged?) decides repair versus
amendment. If the test says amendment, the output is a recorded finding and a
re-plan, never a quieter clause.
