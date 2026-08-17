---
item: HS-S0185
stage: discover
created: 2026-08-17T13:07:31.904Z
updated: 2026-08-17T13:07:31.904Z
template_sig: 86ce4036
rendered_sig: 4ee095a2
---

# Discover — The opening encounter: a boundary refuses an append

## Signal Ledger

Every signal below was already held when this story was scaffolded: the story map row a
human approved at the review gate on 2026-08-17, the decomposition rationale behind its
project, and that project's signed-off design. No discovery fan-out was warranted, and none
was run — inventing signals here would compete with the artifacts that already carry them.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice, as the story map states it | `application-author-path/_storymap.md` | Author the opening encounter into HS-P0020's pinned tree in DT-4's resolved shape, so a reader following it from step one runs a program whose own output carries `AppendError::ConditionViolated` (ES-25, `spec/SPECIFICATION.md:3693`) against a real `MemoryEventStore`, with every step anchor answerable cold and no hidden `#`-prefixed line carrying any part of the boundary. |
| Project acceptance criteria this slice traces to | `application-author-path/_storymap.md` | AC-002, AC-004, AC-006 — the spec's AC-### must map onto these, not restate them |
| Milestone / slice | `application-author-path/_storymap.md` | `opening-encounter` — its slice-mates are delivered with it |
| Archetype | `application-author-path/_storymap.md` | `capability` — a user-observable slice through every layer |
| Dependency edges | `application-author-path/_storymap.md` | `merge-forward-preflight`, `tension-resolutions` |
| Why this project owns it | `.bklg/docs-that-teach/_decomposition.md` | The approved cut assigns this responsibility to `application-author-path` and names the sibling owning each excluded part in that project's non-goals |
| The design this spec binds to | `application-author-path/_design.md` | Signed off by the repository owner on 2026-08-17; its resolved tensions and anti-patterns are binding on this story |

## Questions

Carried into `spec`, not answered here:

- **Does anything in the merged tree move this slice's target?** Deferred to `spec`. The
  initiative runs in parallel with `initiative/from-contract-to-published-library` and merges
  forward before the pull request; where that changes a file this story touches, the spec
  states it rather than this ledger guessing.
- **Do the project ACs above decompose cleanly into this story's own AC-###?** Deferred to
  `spec`, which is the artifact that enumerates them and the gate that checks the mapping.
- **Are this story's dependencies satisfied in merge order?** Answered: yes. The edges
  above are recorded as real `blocked_by` links on the item, and the story map was ordered
  so every dependency precedes its consumer.

## Decision

Author the opening encounter into HS-P0020's pinned tree in DT-4's resolved shape, so a reader following it from step one runs a program whose own output carries `AppendError::ConditionViolated` (ES-25, `spec/SPECIFICATION.md:3693`) against a real `MemoryEventStore`, with every step anchor answerable cold and no hidden `#`-prefixed line carrying any part of the boundary.

That is the whole of it, and it is deliberately the story map's own sentence rather than a
paraphrase: the map was reviewed and approved by a human, and restating it here would create
a second description that can drift from the approved one. The spec will carry the observable
behaviour, the AC-### enumeration mapping onto AC-002, AC-004, AC-006, and the integration contract naming
the real mount point — none of which is decided here.

## The wrong implementation

An opening program that constructs a store, appends, and asserts the log is non-empty. Every claim true, the boundary empty — the published alpha's `Tags::empty()` shape with a new domain.

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
