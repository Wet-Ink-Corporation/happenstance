---
item: HS-S0086
stage: discover
created: 2026-08-12T13:02:55.338Z
updated: 2026-08-12T13:02:55.338Z
template_sig: 86ce4036
rendered_sig: de822d23
---

# Discover — The MSRV stops being a preference and becomes a promise

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Storymap slice | `_storymap.md`:58 | "Author, through `.kb/_intake/` and `/redkiln:kb-ingest`, a new decision atom at 0030-or-above stating the 1.97.1 floor as a promise, why it is where it is, and what an MSRV bump costs a consumer under 0.x — leaving `.kb/decisions/0004-edition-and-msrv.md` and `0029-msrv-raised-to-1-97-1.md` byte-identical and `redkiln validate --kb` clean." |
| AC-006 | `project.md`:246-250 | New accepted atom states the floor, why it is where it is, and what a bump means to a consumer now that one exists; the two prior atoms unmodified; `validate --kb` passes. |
| AC-013 | `project.md`:273-276 | Atom numbered 0030+, authored via ingest. |
| ADR-0004's Policy | `.kb/decisions/0004-edition-and-msrv.md`:83-86 | "An MSRV bump is a minor version bump... This is a preference until first publish (phase 12) and a promise to downstream consumers afterward." This project *is* phase 12 (`project.md`:61). |
| ADR-0029's closing line | `.kb/decisions/0029-msrv-raised-to-1-97-1.md`:94-95 | "Phase 12 must revisit the floor at first publish, when it stops being a preference and starts being a promise to a consumer who may not even build `happenstance-sqlite`." Direct charge for this story. |
| ADR-0004's Status section | `.kb/decisions/0004-edition-and-msrv.md`:88-94 | "The body stays verbatim" — amended, not superseded, by ADR-0029; both `status: accepted`. |
| Immutability rule | `.kb/maps/decision-map.md`:30-34, 76-79 | Accepted atoms are never edited; a correction is a repair (unchanged in place) or a new atom carrying `supersedes`/`amends`. ADR-0004/ADR-0029 is the amendment shape already in use — this story's atom is a **sibling**, not an amendment of either. |
| CLAUDE.md constraint 5 | `CLAUDE.md`:174-192 | Restates the same: MSRV is 1.97.1, raised at phase 2; "weigh the floor, do not obey it" — the floor moved because a *dependency's build script* forced it, not our code; the `msrv` job "proves nothing until the two [MSRV and toolchain pin] diverge." This nuance belongs in the new atom's justification. |
| Hand-authoring prohibition | `CLAUDE.md`:103-105 | Atom must come from `/redkiln:kb-ingest`, never hand-written under `.kb/decisions/`. |
| The release-path consequence, already stated | `_decomposition.md`, Deployment brief, "Release path if a published version changes" (lines 722-747) | Under 0.x the minor bump *is* the breaking-change boundary; the new atom "states the release-path consequence: an MSRV bump is a minor-version event under the same 0.x rule" — the brief states the consequence, the atom is the artefact of record. |
| U6 / AC-UX-010, the consumer-facing sentence | `_decomposition.md`, UX brief (lines 361-366) | The Guarantees slot states the MSRV as a promise and links the new atom — that's a downstream story (`guarantees-and-docs-rs-presentation`), not this one; this story's job stops at the atom. |

## Questions

- **What does "a promise" add beyond restating ADR-0004/0029's existing text?** DR-6 (`project.md`:176-183) is explicit: the atom "must also state whether an MSRV bump remains a minor bump now that the policy... binds a real consumer" — i.e. it is not enough to copy the floor and the phase-2 justification forward; the atom has to say something ADR-0004 could not, because ADR-0004 was written when the promise/preference line was still hypothetical. Answered here: the atom's distinct content is the consumer-facing consequence (a real downstream build now breaks on an MSRV bump) and the confirmation that the minor-bump convention holds under that new stake, not a re-justification of 1.97.1 itself.
- **Does raising the MSRV again after this promise land differently than before?** Not this story's to answer in full — deferred to spec, which can state it as a consequence (any future bump is a `0.(2+n).0` event, per the deployment brief's Release path section) without re-opening ADR-0004/0029's own reasoning.

## Decision

The MSRV stops being ADR-0004's "preference until first publish" and becomes a promise: a new, sibling decision atom (0030+) states the 1.97.1 floor, cites ADR-0004's and ADR-0029's reasoning without repeating or amending it, and states explicitly — because DR-6 requires it and because a real downstream consumer now exists — that an MSRV bump remains a minor-version bump under the 0.x convention, called out in the changelog. Spec will author this atom through `.kb/_intake/` and `/redkiln:kb-ingest`, verify `.kb/decisions/0004-edition-and-msrv.md` and `0029-msrv-raised-to-1-97-1.md` are byte-identical afterward, and run `redkiln validate --kb` as the check.

## The wrong implementation

A new atom, correctly numbered 0030+, correctly authored through the ingest path, that never touches `.kb/decisions/0004-edition-and-msrv.md` or `0029-msrv-raised-to-1-97-1.md` and passes `redkiln validate --kb` clean — but whose body only restates "the MSRV is 1.97.1, promoted from preference to promise at phase 12" without ever answering DR-6's actual question: whether an MSRV bump still counts as a minor bump now that a real consumer is bound by it. This satisfies AC-006's letter (a new atom exists, the old ones are untouched, validation passes) while missing DR-6's substance — a claim that passes its own check. What catches it: the atom's own content must state the minor-bump answer explicitly, and a review pass reading the atom for that sentence (not just for frontmatter validity) is the instrument, since `validate --kb` checks structure, not whether the promised content is actually present.

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
