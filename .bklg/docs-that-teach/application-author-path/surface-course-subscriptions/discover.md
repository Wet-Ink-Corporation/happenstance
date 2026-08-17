---
item: HS-S0188
stage: discover
created: 2026-08-17T13:12:05.776Z
updated: 2026-08-17T13:12:05.776Z
template_sig: 86ce4036
rendered_sig: 3644cac7
---

# Discover — Surface the worked example in its own words

## Signal Ledger

Every signal below was already held when this story was scaffolded: the story map row a
human approved at the review gate on 2026-08-17, the decomposition rationale behind its
project, and that project's signed-off design. No discovery fan-out was warranted, and none
was run — inventing signals here would compete with the artifacts that already carry them.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice, as the story map states it | `application-author-path/_storymap.md` | Bring the worked example within the reader's reach: surface `examples/course-subscriptions/src/main.rs:1-19` verbatim by construction, one module per included file (`xtask/src/constitution.rs:11-18`, `:39-50`), with orientation prose that tells the reader what they are about to read and names the `happenstance` / `happenstance_core` vocabulary seam in one sentence at the link. |
| Project acceptance criteria this slice traces to | `application-author-path/_storymap.md` | AC-010 — the spec's AC-### must map onto these, not restate them |
| Milestone / slice | `application-author-path/_storymap.md` | `conceptual-bridge` — its slice-mates are delivered with it |
| Archetype | `application-author-path/_storymap.md` | `capability` — a user-observable slice through every layer |
| Dependency edges | `application-author-path/_storymap.md` | `invariant-to-appendcondition-bridge` |
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

Bring the worked example within the reader's reach: surface `examples/course-subscriptions/src/main.rs:1-19` verbatim by construction, one module per included file (`xtask/src/constitution.rs:11-18`, `:39-50`), with orientation prose that tells the reader what they are about to read and names the `happenstance` / `happenstance_core` vocabulary seam in one sentence at the link.

That is the whole of it, and it is deliberately the story map's own sentence rather than a
paraphrase: the map was reviewed and approved by a human, and restating it here would create
a second description that can drift from the approved one. The spec will carry the observable
behaviour, the AC-### enumeration mapping onto AC-010, and the integration contract naming
the real mount point — none of which is decided here.

## The wrong implementation

A summary of the worked example in someone else's words, which is the one thing the initiative's own evidence says not to do: the module doc is already one of the three strongest explanations written.

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
