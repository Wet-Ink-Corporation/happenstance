---
item: HS-S0146
stage: discover
created: 2026-08-17T06:13:35.891Z
updated: 2026-08-17T06:13:35.891Z
template_sig: 86ce4036
rendered_sig: 53bf6bc7
---

# Discover — The closed need set and the declaration form, landed once

## Signal Ledger

Every signal below was already held when this story was scaffolded: the story map row a
human approved at the review gate on 2026-08-17, the decomposition rationale behind its
project, and that project's signed-off design. No discovery fan-out was warranted, and none
was run — inventing signals here would compete with the artifacts that already carry them.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice, as the story map states it | `page-need-discipline/_storymap.md` | Resolve DT-2, DT-3 and DR-05 in _design.md with the rejected options and the HS-P0020 hosting assumption named, and land the closed need set once — as the NEEDS const in the new xtask/src/ module and as the rule atom that documents it. |
| Project acceptance criteria this slice traces to | `page-need-discipline/_storymap.md` | AC-003, AC-004 — the spec's AC-### must map onto these, not restate them |
| Milestone / slice | `page-need-discipline/_storymap.md` | `discipline-on-disk` — its slice-mates are delivered with it |
| Archetype | `page-need-discipline/_storymap.md` | `foundation` — real in-tree substrate a capability slice consumes; never a double or a fixme |
| Dependency edges | `page-need-discipline/_storymap.md` | none — this slice opens its project |
| Why this project owns it | `.bklg/docs-that-teach/_decomposition.md` | The approved cut assigns this responsibility to `page-need-discipline` and names the sibling owning each excluded part in that project's non-goals |
| The design this spec binds to | `page-need-discipline/_design.md` | Signed off by the repository owner on 2026-08-17; its resolved tensions and anti-patterns are binding on this story |

## Questions

Nothing this slice needs is unresolved at this stage:

- **Does anything in the merged tree move this slice's target?** Deferred to `spec`. The
  initiative runs in parallel with `initiative/from-contract-to-published-library` and merges
  forward before the pull request; where that changes a file this story touches, the spec
  states it rather than this ledger guessing.
- **Do the project ACs above decompose cleanly into this story's own AC-###?** Deferred to
  `spec`, which is the artifact that enumerates them and the gate that checks the mapping.
- **Does this slice depend on anything not yet built?** Answered: no. It opens its project
  and its dependency list is empty, which is why it is first in that project's order.

## Decision

Resolve DT-2, DT-3 and DR-05 in _design.md with the rejected options and the HS-P0020 hosting assumption named, and land the closed need set once — as the NEEDS const in the new xtask/src/ module and as the rule atom that documents it.

That is the whole of it, and it is deliberately the story map's own sentence rather than a
paraphrase: the map was reviewed and approved by a human, and restating it here would create
a second description that can drift from the approved one. The spec will carry the observable
behaviour, the AC-### enumeration mapping onto AC-003, AC-004, and the integration contract naming
the real mount point — none of which is decided here.

## The wrong implementation

An open-ended needs vocabulary, so a lint cannot check membership and every page's declaration is unfalsifiable. The closed set exists so the check can exist.

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
