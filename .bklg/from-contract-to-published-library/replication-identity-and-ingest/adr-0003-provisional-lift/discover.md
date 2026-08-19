---
item: HS-S0108
stage: discover
created: 2026-08-12T13:03:29.656Z
updated: 2026-08-12T13:03:29.656Z
template_sig: 86ce4036
rendered_sig: b1ad1aac
---

# Discover — ADR-0003 loses provisional by a new atom, never by an edit

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: lift ADR-0003's `provisional` marker with a **new** atom through kb-ingest that `depends_on` it and cites the round trip as the evidence its own Status section asks for — never an edit to the accepted atom — or, if the round trip cannot be made byte-identical, record why it cannot lift | `_storymap.md`, *Slices* table, `wire-message-set-and-round-trip` row 3 | Two admissible outcomes, both of them a new atom. Neither is an edit and neither is silence |
| **Depends on `byte-identical-round-trip-and-idempotent-replay` (HS-S0107)**, which supplies the evidence this atom cites | `_storymap.md`, *Slices* `depends_on`; `_storymap.md`, *Merge order* step 4 | *"The lift is last because it cites evidence that does not exist until the story before it merges"* |
| **AC-006** — ADR-0003's `provisional` marker is lifted against **the ADR's own stated lift condition** — or the reason it cannot be lifted is recorded | `project.md`, *Acceptance criteria*, AC-006 | The condition is the ADR's, not this project's. Cite it; do not restate it |
| ADR-0003's Status, verbatim: *"Accepted and provisional… The payoff it claims — that a peer forwards events without deserialising them — has not yet been exercised, because no replication code exists yet. It lifts at phase 13, when `happenstance-sync` round-trips an event between two stores without deserialising its payload."* | `.kb/decisions/0003-opaque-payloads.md:87-95` | Two conjuncts: a round trip **between two stores**, and **without deserialising the payload**. Both must be evidenced |
| **An accepted decision atom is immutable** — `redkiln validate --kb` checks each one against `HEAD`, so correcting one means writing a new atom, never editing the body | `CLAUDE.md`, *Where the work lives* | Editing ADR-0003's Status section fails the very validator DoD 5 relies on |
| The in-workspace precedent: ADR-0029 amends ADR-0004 **without touching it** — `supersedes: null`, `depends_on: [kb-decision-0004]`, and a summary stating in its own words *"This amends ADR-0004 rather than superseding it - that decision's body stays verbatim, because its reasoning is what this one acted on"* | `.kb/decisions/0029-msrv-raised-to-1-97-1.md:10-26` | The shape is already proven in this repository. Copy it rather than inventing an amendment mechanism |
| Recommended shape: the lift rides inside **ADR-0026**, declaring `depends_on: kb-decision-0003`; *"A standalone atom is equally legitimate and costs an ADR number the RUNBOOK queue does not carry"* | `_decomposition.md`, *Tension 5* | Two legitimate answers with a stated cost each. This story picks one and says why |
| **AC-A09** — nothing in `.kb/` was hand-authored: the ADR-0003 amendment arrives through `.kb/_intake/` and `/redkiln:kb-ingest`, and `.kb/decisions/0003-opaque-payloads.md` **is unchanged in the diff** | `_decomposition.md`, AC-A09 | "Unchanged in the diff" is a mechanically checkable statement about `git diff` |
| The three claims ADR-0003 makes that a lift must be honest about: `Event::data` is `bytes::Bytes`; `happenstance-core` carries no `serde` in default features; the `serde` feature covers envelope types only | `.kb/decisions/0003-opaque-payloads.md`; `CLAUDE.md`, binding constraint 2 | Lifting `provisional` must not read as retiring the constraint. It is the same decision, now with evidence |
| The alternatives ADR-0003 already recorded as losing: a payload type parameter (*"would force a replication adapter to compile against the sender's own domain types"*) and `serde_json::Value` in the contract crate | `.kb/decisions/0003-opaque-payloads.md:76-85` | A lift atom does not re-litigate these; it records that the winner's claim was exercised |
| **SY-12 `[FROZEN]`** cites ADR-0003's lift condition by line from the long-form record and observes that a metadata-borne identity *"fails the ADR's own test"* | `spec/SPECIFICATION.md:6165-6172`; `references/adr/0003-opaque-payloads.md:14-15` | The lift condition is load-bearing in the specification, not only in the KB. Changing its wording would break a citation |
| The long form is cited by `spec/SPECIFICATION.md` by line range, which is why `references/adr/` is not deleted in favour of the atoms | `CLAUDE.md`, *Where the work lives*; `_decomposition.md`, *The seam, in package terms* | Whatever this story writes, both artefacts stay and `spec-trace` must still resolve |
| **DoD 5** — DoD 14 is observable: the corresponding open-question atoms reflect the resolution and `redkiln validate --kb` passes | `project.md`, *Definition of done* 5 | The validator is the gate for this story, and it is the same one an illegal edit trips |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` and `.kb/playbooks/one-decision-per-adr-title.md` both apply directly | `_decomposition.md`, *The Accepted atoms that constrain this* | The lift is one decision with one title; it does not also settle what `serde` may be used for |

## Questions

**Does ingest re-check the writer's asserted append conditions? — Not this
story's, and it is worth saying why it is nearby.** ADR-0003's payoff and SY-1's
answer share a mechanism: a peer that never parses a payload also never has a
domain reason to refuse one, which is part of why refusal is not on the table
(`crates/happenstance-sync/src/lib.rs:68-76`). But the question is ADR-0026's and
this atom must not restate it.

**Hub-and-spoke versus peer-to-peer — not this story's.** ADR-0027 owns it, and
ADR-0003 is topology-blind.

**Does the marker lift by a new atom or by an edit? — Answered: a new atom,
always.** `.kb/decisions/0003-opaque-payloads.md` is `status: accepted` and
`redkiln validate --kb` checks accepted decision atoms against `HEAD`. An edit to
its Status section fails validation, which is DoD 5's own gate — so the illegal
route is at least loud. The legal route already has a worked precedent in the
tree: ADR-0029, which amends ADR-0004 with `supersedes: null` and
`depends_on: [kb-decision-0004]`.

**Does the lift ride inside ADR-0026 or land as a standalone atom? — Deferred to
spec, with the trade stated.** Inside ADR-0026 it costs no ADR number and couples
the lift to an atom whose subject is peers rather than payloads, which strains
one-decision-per-title. Standalone it is cleaner to cite and costs a number the
`RUNBOOK.md:305-307` queue does not currently carry. Note the sequencing
consequence, because it is not obvious: ADR-0026 merges in **slice 1** and the
evidence does not exist until **slice 4**, so an ADR-0026 that already asserts
the lift would be citing evidence that had not been produced when it was
accepted. That is an argument for the standalone atom that `_decomposition.md`'s
recommendation does not make, and spec must weigh it.

**What if the round trip cannot be made byte-identical? — Answered: that is a
legitimate outcome and it is still a new atom.** AC-006's second arm requires the
reason recorded. A recorded refusal is an answer; silence is not
(`project.md`, DR-3).

**Does lifting `provisional` change anything a compiler can see? — Answered: no,
and the atom must say so.** ADR-0003's three constraints are unchanged by the
lift; what changes is that the claim behind them has been exercised. In
particular `happenstance-core` still carries no `serde` in default features, and
`happenstance`, the typed layer, still may (`CLAUDE.md`, binding constraint 2).

**Is any code or gate step in this story's diff? — Answered: none.** The diff is
`.kb/_intake/` plus what `/redkiln:kb-ingest` writes, and — if the standalone
route is chosen — a long-form record under `references/adr/`. The evidence
command is `redkiln validate --kb && redkiln doctor`.

## Decision

ADR-0003 is the decision the whole crate structure rests on — opaque `Bytes`
payloads, no `serde` in `happenstance-core`'s default features — and it has
carried `provisional` since the day it was written, on its own honest admission
that *"the payoff it claims… has not yet been exercised, because no replication
code exists yet"*. It also named exactly what would exercise it and when: a
`happenstance-sync` round trip between two stores that does not deserialise the
payload, at phase 13. The story before this one produces precisely that, measured
as a byte-identity assertion across a real store boundary. So the only thing left
is to record the discharge — and the one way it must **not** be recorded is by
editing ADR-0003, because an accepted decision atom is immutable and
`redkiln validate --kb` checks each one against `HEAD`. This story writes a new
atom, through `.kb/_intake/` and `/redkiln:kb-ingest`, that `depends_on`
`kb-decision-0003`, states that ADR-0003's body stays verbatim, cites the round
trip and the replay as the evidence ADR-0003's own Status section asked for, and
leaves all three of ADR-0003's constraints in force. The spec stage will cover:
whether the lift rides inside ADR-0026 or lands standalone, weighed against the
slice-1-versus-slice-4 sequencing; the atom's frontmatter — `supersedes: null`,
`depends_on: [kb-decision-0003]`, on the ADR-0029 pattern; the exact evidence
citation, naming the test and its assertion rather than the story; the explicit
statement that constraint 2 is unchanged; the second-arm text if the round trip
did not come out byte-identical; and the assertion that
`.kb/decisions/0003-opaque-payloads.md` is byte-identical in this diff.

## The wrong implementation

**The loud mutant, named so it is not mistaken for the interesting one:** open
`.kb/decisions/0003-opaque-payloads.md`, delete the words "and provisional", and
commit. `redkiln validate --kb` fails on accepted-decision immutability, which is
exactly what it is for. This one is a tripwire, not a hazard.

**The interesting mutant: `supersedes: kb-decision-0003` instead of
`depends_on`.** It is a new atom. It arrives through `.kb/_intake/` and
`/redkiln:kb-ingest`. It never touches ADR-0003's body. `redkiln validate --kb`
passes — supersession is a first-class relationship the validator *supports*,
and ADR-0003 dutifully acquires `superseded_by`. `redkiln doctor` passes. The
diff looks exemplary. And ADR-0003 is now **retired**: the atom that says
`Event::data` is opaque `Bytes`, that `happenstance-core` carries no `serde` in
its default features, and that the `serde` feature covers envelope types only,
reads as no longer in force — replaced by an atom whose actual subject is a
marker. `CLAUDE.md`'s binding constraint 2 then cites a superseded decision, and
the next person to want `serde` in the contract crate finds nothing standing
against it. ADR-0029 is the counter-example written into the tree for exactly
this reason: `supersedes: null`, `depends_on: [kb-decision-0004]`, and a summary
that says out loud *"This amends ADR-0004 rather than superseding it - that
decision's body stays verbatim"* (`.kb/decisions/0029-msrv-raised-to-1-97-1.md:10-26`).
The distinction between amending and superseding is invisible to every automated
check in the repository and is the whole content of this story.

**The third mutant lifts on the wrong evidence, and it is the one this project's
own structure invites.** A lift atom citing "the round trip is green" without
naming which assertion ran. If HS-S0107 had shipped `DecodedValueRoundTrip` — the
comparison on decoded values rather than on bytes — the suite would be green, the
lift would cite it, and ADR-0003's guarantee would be certified by a test that
never looked at a byte, against a codec that WF-11's measured inverted branch
shows can be wrong while round-tripping perfectly
(`spec/SPECIFICATION.md:2315-2322`). The guard is that the citation names the
assertion and the negative control, not the story: *"the payload `Bytes` compared
for equality at the receiver after commit, with `PayloadTouchingSuite` passing"*
is a claim someone can check; *"AC-006 green"* is not. This is also why DoD 3
requires a `_ledger.md` citing which command produced the evidence per AC — *"a
green suite proves something works, never that the criteria this project was
written to satisfy are the things that work"* (`project.md`, DoD 3).

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

**Box 6.** This story adds no conformance rule and contains no test code; its
whole diff is `.kb/` and possibly `references/adr/`. Ticked as vacuously true
rather than argued — but with one live obligation carried over: the evidence this
atom cites must be an assertion that itself honoured CF-6, which is HS-S0107's
box 6 and is why this story `depends_on` it rather than running in parallel.

**Box 7.** This story edits no `[FROZEN]` clause and no `spec/SPECIFICATION.md`
line at all. The nearest thing to a clause interaction is SY-12, which cites
ADR-0003's lift condition by line from the long-form record
(`spec/SPECIFICATION.md:6165-6172`, `references/adr/0003-opaque-payloads.md:14-15`)
— so the long form's wording must not move, or `spec-trace`'s citation anchoring
breaks. The decision that authorises this work, ADR-0026, is written first in
slice 1; the lift atom itself is the new decision record that AC-006 requires, and
it is written before anything downstream reads it.
