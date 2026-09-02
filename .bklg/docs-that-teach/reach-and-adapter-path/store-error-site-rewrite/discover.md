---
item: HS-S0156
stage: discover
created: 2026-08-17T06:27:41.451Z
updated: 2026-08-17T06:27:41.451Z
template_sig: 86ce4036
rendered_sig: 412ef199
---

# Discover — rustc's own E0034 output at the site where it fires

## Signal Ledger

Every signal below was already held when this story was scaffolded: the story map row a
human approved at the review gate on 2026-08-17, the decomposition rationale behind its
project, and that project's signed-off design. No discovery fan-out was warranted, and none
was run — inventing signals here would compete with the artifacts that already carry them.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice, as the story map states it | `reach-and-adapter-path/_storymap.md` | Rewrite crates/happenstance-core/src/store.rs's module doc behind HS-P0020's clause-id pin: restore rustc's real error[E0034] block including its = note: candidate lines and TraitVariantBlanketType, state the narrow unchecked limit, keep the in-place fix ahead of any hop, add the one guarded pointer to the reasoning account, and run cargo xtask spec-trace over the result. |
| Project acceptance criteria this slice traces to | `reach-and-adapter-path/_storymap.md` | AC-003, AC-006, AC-007, AC-010, AC-012 — the spec's AC-### must map onto these, not restate them |
| Milestone / slice | `reach-and-adapter-path/_storymap.md` | `adapter-error-site` — its slice-mates are delivered with it |
| Archetype | `reach-and-adapter-path/_storymap.md` | `capability` — a user-observable slice through every layer |
| Dependency edges | `reach-and-adapter-path/_storymap.md` | `pointer-policy-and-inventory`, `adapter-reasoning-account` |
| Why this project owns it | `.bklg/docs-that-teach/_decomposition.md` | The approved cut assigns this responsibility to `reach-and-adapter-path` and names the sibling owning each excluded part in that project's non-goals |
| The design this spec binds to | `reach-and-adapter-path/_design.md` | Signed off by the repository owner on 2026-08-17; its resolved tensions and anti-patterns are binding on this story |

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

Rewrite crates/happenstance-core/src/store.rs's module doc behind HS-P0020's clause-id pin: restore rustc's real error[E0034] block including its = note: candidate lines and TraitVariantBlanketType, state the narrow unchecked limit, keep the in-place fix ahead of any hop, add the one guarded pointer to the reasoning account, and run cargo xtask spec-trace over the result.

That is the whole of it, and it is deliberately the story map's own sentence rather than a
paraphrase: the map was reviewed and approved by a human, and restating it here would create
a second description that can drift from the approved one. The spec will carry the observable
behaviour, the AC-### enumeration mapping onto AC-003, AC-006, AC-007, AC-010, AC-012, and the integration contract naming
the real mount point — none of which is decided here.

## The wrong implementation

An explanation added to `store.rs` that a reader reaches only by already knowing to look there — the current state, which is the defect.

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
