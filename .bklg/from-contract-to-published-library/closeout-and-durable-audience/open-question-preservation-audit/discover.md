---
item: HS-S0130
stage: discover
created: 2026-08-12T13:03:55.724Z
updated: 2026-08-12T13:03:55.724Z
template_sig: 86ce4036
rendered_sig: c65d4a94
---

# Discover — Zero open-question deletions, every consumed atom resolved and annotated

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-liner: "Prove no open-question atom was deleted — a zero-deletion diff of `.kb/open-questions/` across the initiative — and that each consumed atom carries a resolution-bearing status and an annotated bullet in the index." | `_storymap.md:58` | Two distinct proofs: a diff (zero deletions) and a per-atom status/annotation check, not one or the other. |
| AC-006 | `project.md:211-214` | `git diff --diff-filter=D` across the initiative's start commit to closeout must be empty; every atom whose question this initiative answered needs a resolution-bearing status *and* an annotated bullet in the index. |
| DR-7 — Open questions resolved, never deleted | `project.md:153-161` | Names six atoms by filename this initiative specifically consumes: `cf-40-fixture-limits-ownership.md`, `projection-store-batch-has-no-apply-seam.md`, `es-38-and-gap-read-rules-are-unowned.md`, `global-versus-per-boundary-visibility-invariant.md`, `ps-1-states-no-progress-obligation.md`, `human-readable-payload-encoding-on-a-constrained-peer.md`. |
| Current state on disk (checked directly) | `.kb/maps/open-questions-index.md` | Nineteen atoms total, all currently listed with status **Open** — including all six of DR-7's named atoms. None carries a resolution annotation yet, which is expected: that annotation is this story's own deliverable, not yet produced. |
| The never-delete convention, verbatim | `.kb/maps/open-questions-index.md:136-143` ("Adding an entry") | "A withdrawn or superseded question stays listed, annotated, rather than removed" — the standing rule this story audits, not a project-specific invention. Same wording appears in the atom's own frontmatter summary at `.kb/maps/open-questions-index.md:7-13`. |
| `dependsOn: decision-atom-audit-table` | `_storymap.md:147` (merge order 3) | "the second is meaningless without the first's list" (`_storymap.md:79-80`) — this story needs the enumeration of what the initiative actually settled to know which open questions should now carry a resolution. |

## Questions

- **Is a zero-deletion diff alone sufficient, or does the story also need to prove the six named atoms' *content* actually reflects resolution?** AC-006's text requires both — the diff proves nothing was removed, but a status field and an index annotation prove something was actually settled, not merely left untouched. **Answered**: this story's spec must run both checks, not treat the diff as a proxy for the second.
- **What counts as "the initiative's start commit"** for the diff base? Not yet fixed in any artefact this discover pass has found. **Deferred to spec**: the spec must name the concrete SHA or ref (likely the commit at which `HS-I0006` was created, per its `created: 2026-08-12` frontmatter field in `../initiative.md:19`, or the first commit touching `.bklg/from-contract-to-published-library/`) rather than leave "the initiative's start" informal.
- **Do any of the nineteen atoms belong to this initiative at all, versus predating it?** Not fully determined here — the index's own sectioning (`.kb/maps/open-questions-index.md:33,77`) shows the nineteen split into two intake waves, both dated `2026-08-10`, and DR-7 names only six as "this initiative consumes." **Deferred to spec**: the audit's zero-deletion check applies to all nineteen (deletion of *any* atom is the thing forbidden), but the resolution-annotation check applies only to the ones this initiative actually answered — the spec must state which subset that is, starting from DR-7's six but confirming none of the other thirteen was also silently resolved by this initiative's work without being named.
- The evaluator-persona question and DoD 13's caveat do not touch this story directly, though the evaluator-persona finding (see `persona-and-journey-intake-staging`) is a candidate for whether it should itself become, or reference, an open-question atom — noted as a possibility for spec to consider, not decided here.

## Decision

An open question that gets quietly deleted once it is answered erases the record that it was ever a live fork — and the next initiative that reaches the same fork has no way to know someone already stood there. `.kb/maps/open-questions-index.md`'s own standing rule (never removed, only annotated) exists precisely so a reader can trust that "not in this index" means "never asked," not "asked and swept." This story is the audit that makes that trust checkable rather than assumed: a diff proving nothing was deleted, and a per-atom check that the six (or more, if spec finds others) this initiative actually settled now say so. The spec that follows will specify the diff base, the resolution-status vocabulary each atom's frontmatter should carry, and the exact annotation text the index gets per resolved atom.

## The wrong implementation

A story that runs the zero-deletion diff, finds it empty, and stops there — reporting AC-006 satisfied because "nothing was deleted." It technically clears the diff half of the acceptance criterion but silently skips the half that actually matters for a *resolved* question: an atom that is still sitting in `.kb/open-questions/` with `status: open`... today, six months after the initiative that was supposed to answer it, tells a reader nothing changed. AC-006's own text is explicit that this is not enough — "every atom whose question this initiative answered has a resolution-bearing status and an annotated bullet in the index" (`project.md:213-214`) — and a diff alone cannot distinguish "resolved and correctly left on disk" from "never touched and correctly left on disk." Both pass a diff check identically. What catches it: this story's ledger must cite, for each of DR-7's six named atoms (and any others spec finds), the atom's frontmatter status field before and after, and the specific index bullet text added — a diff with nothing else attached proves absence of deletion, never presence of resolution.

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
