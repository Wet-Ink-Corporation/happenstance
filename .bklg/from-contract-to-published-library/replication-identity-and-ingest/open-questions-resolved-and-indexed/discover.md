---
item: HS-S0100
stage: discover
created: 2026-08-12T13:03:20.772Z
updated: 2026-08-12T13:03:20.772Z
template_sig: 86ce4036
rendered_sig: 76ce1282
---

# Discover — Both phase-13 open questions resolved in place, not deleted

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: resolve both phase-13-owned open-question atoms in place — `sync-message-set-and-format-version` and `dcb-reference-publishes-no-wire-format` — annotate rather than delete, update `.kb/maps/open-questions-index.md`, and leave `redkiln validate --kb && redkiln doctor` green | `_storymap.md`, *Slices* table, `decisions-of-record` row 3 | The deliverable is two annotated atoms plus one index edit, and a green validator. No code |
| **Depends on `adr-0026-peer-ingest-and-transport` (HS-S0098)**, which supplies WF-1's interoperability disposition in its envelope section — the resolution the DCB atom is waiting for | `_storymap.md`, *Slices* `depends_on`; `RUNBOOK.md:4593-4595`; `_grounding.md` §6 | The DCB atom cannot be resolved before the atom that resolves it exists. This edge is a real content dependency, not ordering |
| **Depends on `adr-0027-merge-compensation-and-message-set` (HS-S0099)**, which supplies the message set and `format_version`'s per-message-versus-negotiated disposition — the resolution the wire atom is waiting for | `_storymap.md`, *Slices* `depends_on`; `.kb/open-questions/sync-message-set-and-format-version.md:19-23` | Same shape: the answer must land in a decision atom before the question can point at it |
| **AC-013** — both phase-13-owned open-question atoms reflect a resolved state, the index is updated **in place**, and `redkiln validate --kb` passes | `project.md`, *Acceptance criteria*, AC-013 | Three checkable outputs, and "in place" is the operative phrase |
| **DR-10** — each open-question atom this project consumes is **resolved**, never deleted; the record that it was once open is itself worth keeping | `project.md`, *Derived requirements*, DR-10 | Deletion is the failure mode this requirement is written against |
| The index states the same rule in its own summary: *"A withdrawn or superseded question stays listed, annotated, rather than removed — the record that it was once open is itself worth keeping."* | `.kb/maps/open-questions-index.md:12-13` | The map atom is self-describing. Editing it to drop a bullet contradicts its own frontmatter |
| Atom 1: `kb-open-question-sync-message-set-undesigned-001` — *"`FORMAT_VERSION = 1` names a message set that does not exist yet"*, `status: accepted`, `kind: open_question`, explicitly *"Owned by phase 13"* | `.kb/open-questions/sync-message-set-and-format-version.md` | Its own summary names the refutation: a phase-13 design negotiating per connection would make the field dead weight and removing it a format break |
| Atom 2: `kb-open-question-dcb-no-published-format-001` — WF-1's interoperability half, deferred because the DCB reference publishes no wire format to interoperate with at all. Owned by phase 13 | `.kb/open-questions/dcb-reference-publishes-no-wire-format.md`; `spec/SPECIFICATION.md:1892-1921` | The deferral is *stronger*, not weaker. Resolving it means recording the design position, not building a bridge |
| Both atoms indexed under the same section, with the phase-13 ownership visible in the bullets | `.kb/maps/open-questions-index.md:122-134` | The two bullets to annotate are identified. Nothing else in the index is this project's |
| Atoms are authored by `/redkiln:kb-ingest` from `.kb/_intake/`, never by hand; hand-writing produces the directory layout of the process without the process, which is why the first attempt was reverted | `CLAUDE.md`, *Where the work lives*; `0269720` | The resolutions arrive through the same intake path as the ADRs — this is AC-A09's *"Nothing in `.kb/` was hand-authored"* |
| Immutability applies to an **accepted decision atom**, which `validate --kb` checks against `HEAD`. These two are `kind: open_question` | `CLAUDE.md`, *Where the work lives*; both atoms' frontmatter | An open question may be annotated in place; a decision may not. Confusing the two produces either a forbidden edit or a needless new atom |
| A DCB wire-interoperability *bridge* is out of scope for this project; the project records the deferral's **renewal**, it does not build the bridge | `project.md`, *Out of scope* | The resolution is "declined by design position, renewed against a named experiment", never "built" |

## Questions

**Does ingest re-check the writer's asserted append conditions? — Not this
story's, and it must not be re-answered here.** SY-1 and SY-6 answer it,
ADR-0026 reconciles with them, and neither of the two atoms this story touches is
about ingest at all. Recorded so the question is visibly declined rather than
silently absent: settling it in passing, in a KB atom, outside the ADR path is
precisely the failure `project.md`'s risk table names last.

**Hub-and-spoke versus peer-to-peer — not this story's either.** ADR-0027 owns
it (`RUNBOOK.md:306`). Neither atom indexes it, so this story creates no new
bullet for it.

**Is "resolved" a `status` value the atoms already support, or a body
annotation? — Deferred to spec, with the constraint fixed.** Whatever shape the
resolution takes, three things must be true afterwards and they are what spec
must pin: `redkiln validate --kb` passes; the resolution **names the decision
atom that resolved it** so the supersession graph has an edge rather than a
claim; and the atom's own body still contains the question, because the record of
what was once open is the point (`.kb/maps/open-questions-index.md:12-13`, DR-10).
The mechanics belong to `/redkiln:kb-ingest` and its adjudication step, not to a
hand edit.

**Can the wire atom resolve if ADR-0027 chooses "keep the per-message field"? —
Yes, and this is worth stating because it looks like a non-resolution.** The atom
is not *"is the field right"*; it is *"the field names no vocabulary"*. Once
ADR-0027 names the vocabulary, the question is answered whichever way the version
mechanism goes, and the atom records which answer landed.

**Does resolving the DCB atom change WF-1's marker? — No, and the story must not
let it.** WF-1 stays `[DEFERRED]` with its named experiment
(`spec/SPECIFICATION.md:1892-1921`). What resolves is the *question about the
question* — whether to attempt a bridge or decline by design position. If the
resolution text implies WF-1 has moved, the KB and the specification disagree,
and the specification wins (`project.md`, DR-2).

## Decision

Two open-question atoms are stamped "Owned by phase 13" and this project is
phase 13, so at exit they are either resolved or they are a lie about who was
going to answer them. The failure mode is not that they go unanswered — ADR-0026
and ADR-0027 answer both — it is that they go *unlinked*: the answer lands in a
decision atom, nothing points the question at it, and the index keeps advertising
a live question that has in fact been settled for a release. The opposite failure
is deleting them, which loses the record that the question was ever open and with
it the reasoning a future reader needs to understand why the answer has the shape
it has. This story does neither: it annotates both atoms in place with their
resolution and the decision atom that produced it, edits the two bullets in
`.kb/maps/open-questions-index.md` rather than removing them, and drives all of it
through `.kb/_intake/` and `/redkiln:kb-ingest` so nothing in `.kb/` is
hand-authored. The spec stage will cover: the exact resolution text for each atom
and which ADR section it cites; the shape a resolved `open_question` atom takes
(status, body annotation, `related` edges) as `/redkiln:kb-ingest`'s adjudication
defines it; the two index bullets and their annotated wording; the assertion that
WF-1's `[DEFERRED]` marker in `spec/SPECIFICATION.md` is untouched by this diff;
and the evidence command, `redkiln validate --kb && redkiln doctor`.

## The wrong implementation

**The mutant: `git rm` on both atoms, and the index bullets deleted with them.**
It satisfies every existing check. `redkiln validate --kb` passes — validation
checks the atoms that exist, and an atom that does not exist cannot fail
conformance or immutability. `redkiln doctor` passes. `cargo xtask spec-trace`
passes, because it reads `spec/SPECIFICATION.md`'s clause-to-rule graph and knows
nothing about `.kb/open-questions/`. The open-questions index gets *shorter*,
which reads as progress. And AC-013's own words — "resolved, not deleted" — would
be the only thing in the repository standing against it, which is why DR-10 exists
as a separate requirement and why the index atom states the rule inside its own
frontmatter summary. What is lost is not recoverable by re-reading the ADR: the
ADR records the answer, the question atom records *why it was open*, what would
have refuted the answer, and who was waiting on it. `kb-open-question-sync-message-set-undesigned-001`'s
summary carries a refutation condition — a per-connection negotiation design that
would make the field dead weight and removing it a format break — that appears
nowhere else in the repository. Delete the atom and the next person to propose
negotiation has no record that it was considered and priced.

**The quieter mutant: annotate both atoms as resolved with no edge to the atom
that resolved them.** Both files read `resolved`, the index bullets say
`Resolved`, everything is green, and nothing anywhere says *by what*. Six months
later the supersession graph `validate --kb` enforces has two nodes with no
inbound edge, and the only way to find the answer is to read seventeen decision
records looking for it. The guard is that each resolution names the ADR and the
section by id.

**And the mutant that resolves the wrong question.** The DCB atom annotated as
"interoperability settled — we do not interoperate", while
`spec/SPECIFICATION.md:1892-1921` still carries WF-1 as `[DEFERRED]` with a named
experiment. Nothing checks the KB against the specification's maturity markers,
so this is green in both directions and contradictory across them. The atom's
resolution is *the design position and its renewal*, not a marker change; a
marker change would be an amendment to a deferred clause and belongs to
`clause-arithmetic-and-deferral-renewals` (HS-S0113) under ADR-0026's
authorisation.

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

**Box 6.** This story adds no conformance rule and touches no `.rs` file at all;
its whole diff is two `.kb/open-questions/` atoms and one `.kb/maps/` atom. The
box is satisfied vacuously and is recorded as vacuous rather than argued.

**Box 7.** This story changes no `[FROZEN]` clause, and the section above names
the specific way it could accidentally appear to: annotating the DCB atom in
language that implies WF-1 has moved. WF-1 stays `[DEFERRED]` in this diff. The
decisions that authorise the resolutions — ADR-0026 and ADR-0027 — are both
written first and are this story's `depends_on` edges.
