---
item: HS-S0169
stage: discover
created: 2026-08-17T06:45:07.508Z
updated: 2026-08-17T06:45:07.508Z
template_sig: 86ce4036
rendered_sig: 70c060f1
---

# Discover — Scope the claim to what one session supports

## Signal Ledger

Every signal below was already held when this story was scaffolded: the story map row a
human approved at the review gate on 2026-08-17, the decomposition rationale behind its
project, and that project's signed-off design. No discovery fan-out was warranted, and none
was run — inventing signals here would compete with the artifacts that already carry them.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice, as the story map states it | `comprehension-evidence/_storymap.md` | State the narrow claim — real stumbles were captured and are traceable — wherever this evidence is summarised, and assert nothing about exhaustiveness anywhere in the log or its summary. |
| Project acceptance criteria this slice traces to | `comprehension-evidence/_storymap.md` | AC-008 — the spec's AC-### must map onto these, not restate them |
| Milestone / slice | `comprehension-evidence/_storymap.md` | `scoped-claim-and-handoff` — its slice-mates are delivered with it |
| Archetype | `comprehension-evidence/_storymap.md` | `capability` — a user-observable slice through every layer |
| Dependency edges | `comprehension-evidence/_storymap.md` | `session-run-against-pinned-tree` |
| Why this project owns it | `.bklg/docs-that-teach/_decomposition.md` | The approved cut assigns this responsibility to `comprehension-evidence` and names the sibling owning each excluded part in that project's non-goals |
| The design this spec binds to | `comprehension-evidence/_design.md` | Signed off by the repository owner on 2026-08-17; its resolved tensions and anti-patterns are binding on this story |

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

State the narrow claim — real stumbles were captured and are traceable — wherever this evidence is summarised, and assert nothing about exhaustiveness anywhere in the log or its summary.

That is the whole of it, and it is deliberately the story map's own sentence rather than a
paraphrase: the map was reviewed and approved by a human, and restating it here would create
a second description that can drift from the approved one. The spec will carry the observable
behaviour, the AC-### enumeration mapping onto AC-008, and the integration contract naming
the real mount point — none of which is decided here.

## The wrong implementation

A claim of exhaustiveness from one session. The small-sample basis that makes one session defensible supports only the narrower claim.

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
