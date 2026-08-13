---
item: HS-S0099
stage: discover
created: 2026-08-12T13:03:19.354Z
updated: 2026-08-12T13:03:19.354Z
template_sig: 86ce4036
rendered_sig: cd9c6620
---

# Discover — ADR-0027: the merge rule, the compensation contract, and the message set

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: author ADR-0027 through the same intake path — the merge rule, the compensation contract, replication scope, whether hub-and-spoke and peer-to-peer are one abstraction or two, and the message set plus the derives on `PushBatch`/`EventGroup`/`ReplicatedEvent` authorised **by name** — taking the complement of ADR-0026's clause range so the split adds up | `_storymap.md`, *Slices* table, `decisions-of-record` row 2 | Two obligations: answer four questions, and be the *complement* of a range this story does not own |
| **Depends on `adr-0026-peer-ingest-and-transport` (HS-S0098)**, which supplies the re-derived clause ledger at HEAD, the claimed range SY-8 – SY-18 + SY-33/SY-34, and the answer to the central question this ADR must not re-answer | `_storymap.md`, *Slices* `depends_on` column; `_decomposition.md`, *Tension 3* | ADR-0027 cannot state its own range until ADR-0026 has stated one. Sequencing is real, not decorative |
| **AC-001** — an accepted atom under `.kb/decisions/`, authored through the ingest path, merged before the code it constrains, stating the alternatives that lost | `project.md`, *Acceptance criteria*, AC-001 | Same two-artefact shape as ADR-0026: long form under `references/adr/0027-*.md`, atom via `/redkiln:kb-ingest` |
| **AC-010** — the ADR-side renewals: every `[DEFERRED]` SY clause in this ADR's range is settled or renewed against a **named** experiment | `project.md`, *Acceptance criteria*, AC-010; `_storymap.md`, *Coverage* AC-010 row | SY-14, SY-27 and SY-28 fall in this range. A renewal with no experiment fails the gate under CF-38 |
| **AC-014** — the union of ADR-0026's and ADR-0027's ranges equals this project's stated range at exit | `project.md`, *Acceptance criteria*, AC-014 | ADR-0027 takes the complement by construction: SY-1 – SY-7, SY-19 – SY-31, SY-35 |
| **SY-2 `[FROZEN]`** — the compensation MUST be appended in the same `append` call as the losing event; a reader MUST NOT observe the log holding the losing event with nothing resolving it | `spec/SPECIFICATION.md:5871-5900` | The compensation contract is already frozen. ADR-0027 records it and the alternative that lost, which SY-2 names as compensate-after-commit |
| **SY-3 `[FROZEN]`** — the port supplies atomicity, identity and idempotence for a compensation and MUST NOT supply its content | `spec/SPECIFICATION.md:5901-5930` | The division of labour: the domain decides what a compensation *means*; the port makes it atomic and idempotent |
| **SY-7 `[PROVISIONAL]`** — compensation authorship MUST be assigned to at most one peer per fact family | `spec/SPECIFICATION.md:6033-6040` | This one is genuinely open at the marker level and is in ADR-0027's range |
| **SY-9 `[FROZEN]`** — hub-ness is a property of an **edge**, not of the port's type or constructor; one adapter type serves both roles simultaneously | `spec/SPECIFICATION.md:6090-6106` | Half the topology question is already settled. What is open is SY-10's |
| **SY-10 `[PROVISIONAL]`** — hub-and-spoke and peer-to-peer are both first-class; the runner MUST … | `spec/SPECIFICATION.md:6110` onward; `_decomposition.md`, *Non-prescriptive implementation notes* | *"SY-10 is where the abstraction question actually lives."* This is the one-abstraction-or-two question in clause form |
| **SY-27 / SY-28 `[DEFERRED]`** — whether replication is whole-log or scoped; a spoke holding a filtered subset cannot distinguish "not yet received" from "filtered out", so a position-based resume watermark against a hub is unsound | `RUNBOOK.md:4567-4572`; `_decomposition.md`, *Tension 3* | Named experiment already on file: the whole-log-versus-scoped experiment at `references/evaluation/PRESSURE-TEST.md:685-693` |
| **SY-30** — the decomposition into groups is explicit on the wire and never re-inferred by the receiver | `crates/happenstance-sync/src/peer.rs:185-196` | *"a receiver that flattened the batch and appended event-by-event would publish a state the origin never had, and no amount of ordering fixes that"* |
| **SY-31** — the watermark is a confirmation, deliberately **not** an event | `crates/happenstance-sync/src/peer.rs:253-272` | Writing "peer X has seen up to Y" into the log makes every confirmation replicable; two peers confirming each other's confirmations is a log that grows unattended |
| **SY-35** — everything replication reasons about is in the tags; the suite never decodes `Event::data` or `Event::metadata` | `RUNBOOK.md:4576-4580`; `project.md`, DR-5 | Assigned to ADR-0027 by `_decomposition.md`, *Tension 3*, as reconciliation-shaped. It is the clause DR-5 rests on |
| A `#[derive]` on a public message type **is** a wire format, and the derives were added once, withdrawn, and not restored: *"what travels is phase 13's"* | `crates/happenstance-sync/src/lib.rs:86-94` | Authorising the derives is a *decision*, and it belongs in ADR-0027 by name — not in a commit |
| `Envelope<T>` is generic in `T` and **is not** an enum of message kinds, precisely so the message set does not end up beside the version check | `crates/happenstance-sync/src/wire.rs:8-16` | The message set instantiates `T`; the envelope's shape does not change. `message-set-on-the-envelope` (HS-S0106) implements this |
| `FORMAT_VERSION` bumps on a **shape** change and never on a capacity bound; the open question is whether the version is per-message or per-connection-negotiated | `crates/happenstance-sync/src/wire.rs:46-65`; `.kb/open-questions/sync-message-set-and-format-version.md` | *"FORMAT_VERSION = 1 is a promise about a vocabulary that has not been chosen."* ADR-0027 chooses the vocabulary |
| ADR-0016 explicitly does **not** settle the replication protocol, message set or `SyncError` extension, and names them ADR-0027's and ADR-0026's | `.kb/decisions/0016-the-wire-format.md`; `.kb/open-questions/sync-message-set-and-format-version.md:19-23` | Build on the envelope, not around it |
| One decision per ADR title; the merge rule and the compensation contract are ADR-0027's and re-deliberating them elsewhere is the side-effect authorship this repository has already reverted once | `.kb/playbooks/one-decision-per-adr-title.md`; `_decomposition.md`, *Notes*; `0269720` | The architecture brief deliberately does not decide these. This story does |

## Questions

**Are hub-and-spoke and peer-to-peer one abstraction or two? — Answered here,
and only half of it is open.** SY-9 is `[FROZEN]` and settles the half that
matters structurally: hub-ness is a property of an **edge**, and one adapter type
must be usable simultaneously as a hub to one set of peers and a symmetric peer to
another (`spec/SPECIFICATION.md:6090-6106`). Kestrel Rotor is the case in the
clause's own text — a vessel that is a hub to nine tablets over ship's wifi and a
symmetric peer to a shore depot over satellite, at the same time. What is open is
SY-10 `[PROVISIONAL]`: whether the *runner* expresses the two as one
configuration or two types. ADR-0027 answers that, states the alternative that
lost, and hands the exercise to `hub-and-spoke-and-peer-to-peer-topologies`
(HS-S0110) rather than asserting it in prose. What ADR-0027 may **not** do is
reach a conclusion that requires `Peer::new(is_hub: bool)` or separate
`HubPeer`/`SpokePeer` traits — SY-9 forbids both and SY-9 is frozen.

**Does ingest re-check the writer's asserted append conditions? — Not this
story's question, and deliberately so.** SY-1 and SY-6 answer it, ADR-0026
reconciles with them, and `.kb/playbooks/one-decision-per-adr-title.md` is why
ADR-0027 must not answer it a second time in its own words. ADR-0027 inherits the
answer and specifies what happens **instead**: SY-2's single-batch compensation.

**Is replication whole-log or scoped? — Renewed as `[DEFERRED]` against a named
experiment.** SY-27 and SY-28 stay deferred and cite
`references/evaluation/PRESSURE-TEST.md:685-693`, the whole-log-versus-scoped
experiment. The reason it cannot be settled on paper is stated in the port
already: an opaque associated `Resume` type defers the question into the adapter
without deferring the port, because a scalar cannot distinguish "not yet
received" from "filtered out" (`crates/happenstance-sync/src/peer.rs:95-118`).
The file exists, so the citation resolves and CF-38 is satisfied.

**Which derives land on `PushBatch`, `EventGroup` and `ReplicatedEvent`? —
Answered in ADR-0027 by name, and nowhere else.** They carried derives once, the
derives were withdrawn, and phase 5 did not restore them
(`crates/happenstance-sync/src/lib.rs:86-94`). `message-set-on-the-envelope`
(HS-S0106) may add only what this atom names.

**Is `format_version` per-message or per-connection-negotiated? — Answered here,
and it is the whole of the open-question atom.** `.kb/open-questions/sync-message-set-and-format-version.md`
records the refutation condition in its own summary: a phase-13 design that
negotiates per connection would make the field dead weight on every message and
removing it a format break. ADR-0027 picks one and says what a bump would then
mean; `open-questions-resolved-and-indexed` (HS-S0100) records the resolution
against the atom.

**Does `SyncError` grow here? — Deferred to ADR-0026.** ADR-0016 named the
extension as ADR-0026's (`crates/happenstance-sync/src/lib.rs:91-94`), and this
story takes the complement rather than a second opinion.

**Does the merge rule need a conformance rule of its own? — Deferred to spec,
under a standing bar.** Before landing any sync rule beyond the two headline
ones, name the plausible wrong peer it rejects and write that peer into
`happenstance-sync-testkit`'s own `tests/` in the same change
(`CLAUDE.md`, *The rule that matters*; `_decomposition.md`, testing brief
*Notes*). A merge-rule clause with no failing peer is decorative.

## Decision

Two logs that each accepted a fact the other has not seen must converge without
either of them deleting anything, and today the repository states what must
*not* happen — no rejection, no re-evaluated condition — without stating what
happens instead in a decision record anyone can cite. The specification has
frozen the shape of the answer (SY-2's single-batch compensation, SY-3's
division of labour between port and domain, SY-9's edge-property hub-ness), the
crate carries the message types with their derives deliberately withheld because
*"a `#[derive]` on a public message type is a wire format"*
(`crates/happenstance-sync/src/lib.rs:86-94`), and `FORMAT_VERSION = 1` names a
vocabulary that has never been chosen. This story is the decision that chooses
it. The spec stage will cover: the merge rule and the alternatives that lost;
the compensation contract as SY-2/SY-3 already freeze it, with the domain/port
seam stated explicitly; the answer to whether hub-and-spoke and peer-to-peer are
one abstraction or two, taken at SY-10 and constrained by frozen SY-9; the
replication-scope deferral renewed against `PRESSURE-TEST.md:685-693` by name;
the message set as **new types instantiating `Envelope<T>`**, never an enum
around it, with each derive on `PushBatch`, `EventGroup` and `ReplicatedEvent`
authorised individually; `format_version`'s per-message-versus-negotiated
disposition and what a bump would then mean; SY-35's no-payload-decoding
obligation restated as this ADR's own constraint on the suite; and ADR-0027's
clause range — SY-1 – SY-7, SY-19 – SY-31 and SY-35 — written down as a range so
`clause-arithmetic-and-deferral-renewals` (HS-S0113) can add it to ADR-0026's and
compare.

## The wrong implementation

**The merge mutant, and it is SY-2's own named target.** `CompensateAfterCommit`,
in `crates/happenstance-sync-testkit/tests/` — a peer that ingests the losing
event, returns `Ack`, and lets a projection or a follow-up command author the
compensation afterwards. It passes `ingest_never_rejects`, because it rejects
nothing. It passes every event-store rule, because each of its two appends is a
correct single `append`. It is green against any test that counts events, because
both events do eventually arrive. The window it opens is small, real, and
survives the window closing: *"a device syncing inside it cuts its next slice
from a hub log that says one physical compressor is held twice, and the projection
that exists to prevent the conflict is what manufactures it"*
(`spec/SPECIFICATION.md:5893-5900`). The rule that must fail it is
`compensation_is_atomic_with_the_losing_event`, and
`headline-rules-and-mutant-registry` (HS-S0105) is where the mutant is compiled.

**The ordering mutant that a counting test cannot see.** `FlatteningIngestPeer` —
a receiver that takes a `PushBatch`, flattens `groups` into one event list, and
appends event-by-event. Every event arrives; the count is right; the payloads are
byte-identical; a test that asserts "receiver holds N events" is green. It is
still wrong, because the group decomposition is *on the wire and explicit* for
exactly this reason: a flattened ingest publishes an intermediate state the
origin never had, and no ordering repair recovers it
(`crates/happenstance-sync/src/peer.rs:185-190`, SY-30). Its sibling is
`DedupingIngestPeer`, which silently drops what it decides is a duplicate on a
content comparison rather than on `EventId`, and reports a plausible
`Ingested { appended, skipped }` while the two logs quietly diverge. **Both are
invisible to any rule whose assertion is a count**, which is why the rules this
ADR's range schedules must assert on the *sequence of groups the receiver
publishes*, anchored on positions the receiving store actually assigned — never
on a literal, and never on a position that crossed the boundary.

**The mutant that is the ADR.** An ADR-0027 that authorises the derives in prose
that says "the message types become serialisable" without naming
`PushBatch`, `EventGroup` and `ReplicatedEvent` and the exact derives. It passes
`redkiln validate --kb`, it merges, and it leaves the next implementer free to
add `Deserialize` to a type where a derive decodes every field before the version
can be examined — the partial decode WF-8's MUST NOT forbids and the reason
`Envelope`'s `Deserialize` is hand-written (`crates/happenstance-sync/src/wire.rs:18-20`,
`crates/happenstance-sync/src/lib.rs:78-84`). AC-A08's *"authorised by ADR-0027
by name"* is the guard, and "by name" is load-bearing.

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

**Box 6.** This story adds no conformance rule. It does bind the ones its clause
range schedules, and the binding is stronger than CF-6's spelling check: a merge
or ordering rule must anchor every position assertion on a value the *receiving*
store assigned, because an origin position that crossed the boundary is not a
position in the receiver's numbering at all
(`crates/happenstance-sync/src/lib.rs:100-104`, ADR-0013). A rule that asserted
`[1, 2, 3]` would be wrong twice over here.

**Box 7.** This story *is* a new ADR and it is written before the code it
constrains — `message-set-on-the-envelope` and
`hub-and-spoke-and-peer-to-peer-topologies` both `depends_on` it. It edits no
`[FROZEN]` clause: SY-2, SY-3, SY-9 and SY-35 are cited and inherited, and SY-10
is `[PROVISIONAL]`, which is a marker this ADR is entitled to move because moving
it is what an ADR is for. If drafting finds SY-2 or SY-9 wrong, the output is a
recorded finding and a re-plan, not a softer sentence.
