---
item: HS-S0132
stage: discover
created: 2026-08-12T13:03:58.398Z
updated: 2026-08-12T13:03:58.398Z
template_sig: 86ce4036
rendered_sig: 0c3aba72
---

# Discover — The audience promoted into .kb/product/ through the ingest path

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-liner: "Promote that staging into `.kb/product/` **through `/redkiln:kb-ingest`**, leaving the intake consumed and cleared, a `concept`/`playbook` atom per persona/journey at `authority_tier: product`, and `redkiln validate --kb` green on them." | `_storymap.md:60` | The mechanism (ingest path) is as load-bearing as the outcome (atoms exist) — bolded in the storymap itself. |
| AC-007 / AC-008 / AC-009 | `project.md:215-224` | AC-007: atoms exist, `concept`/`playbook`, `authority_tier: product`, `validate --kb` passes. AC-008: commit range shows arrival via `.kb/_intake/` → `/redkiln:kb-ingest`, intake consumed and cleared, no hand-authored commit. AC-009: each atom's `summary` states secondary evidence directly, `source_paths` names discovery artefacts. |
| DR-8 — promotion runs through the ingest path | `project.md:162-165` | "This is the constraint the reverted commit `0269720` exists to remember." |
| DR-9 — the qualification travels in frontmatter | `project.md:166-170` | The exact sentence quoted: "'All four personas rest on secondary evidence... and none was directly observed' appears where a reader who reads only the frontmatter cannot miss it: in `summary`, with `source_paths`." (Count corrected to three per the resolution in `persona-and-journey-intake-staging`'s discover.md — see below.) |
| The reverted commit, as cited | `_grounding.md:41-50` (quoting the commit message) | Hand-authoring `.kb/` atoms, even correctly-shaped ones, "is not an accelerated version of that process, it is a different process wearing its directory layout." This is the citation for why this story must never write a file directly under `.kb/product/`. |
| `.kb/product/README.md`'s own promotion mechanism | `.kb/product/README.md:15-21` | "`/redkiln:closeout` performs that promotion, so the next initiative inherits them instead of inventing a fresh set from the same evidence." The README already names the mechanism this story must use. |
| `.kb/product/README.md`'s exclusion | `.kb/product/README.md:26-31` | "An unevidenced sketch... Promoting it early gives a guess the standing of a finding." Reinforces why the secondary-evidence qualification (AC-009) cannot be optional. |
| Current state on disk (checked directly) | `.kb/product/` and `.kb/_intake/` listings | Both hold only their own `README.md` — no atom, no staged draft yet. Confirms this story's work has not been anticipated or partially done. |
| The evaluator-count correction | `persona-and-journey-intake-staging/discover.md` (this project, same slice), as updated 2026-08-12 by the repository owner | The DT-1/DR-10 discrepancy was escalated and settled as a synthesis: **three** persona concept atoms and **four** journey playbook atoms — the fourth being the evaluation path, linked to the Application-author persona rather than folded into its journey as a stage. This story promotes 3 concepts + 4 playbooks and carries the finding forward rather than re-litigating it. |
| `dependsOn: persona-and-journey-intake-staging` | `_storymap.md:60` | Cannot promote what was not first staged and approved at the intake gate; the ingest run's input is that story's output. |

## Questions

- **How many personas/journeys does this story actually promote?** **Answered by inheritance**, not re-decided here — and the inherited answer changed on 2026-08-12. The DR-10/DT-1 discrepancy was escalated to the repository owner at the `/redkiln:plan` spec stage and settled as a synthesis rather than a win for either artefact: **three persona concept atoms** (Application author, Adapter author, Local-first/edge developer) and **four journey playbook atoms** — the fourth being the evaluation path, a first-class journey linked to the Application-author persona, *not* a stage buried inside that persona's own journey and *not* a fourth persona. Both source artefacts were amended in place to agree (`../_decomposition.md` DR-10; `../../publication-and-positioning/_design.md` DT-1 rider), each preserving its rejected reasoning. This story's spec must promote exactly what `persona-and-journey-intake-staging` stages, and must not re-open the count independently — doing so twice, inconsistently, would be worse than either project's single decision.
- **What does "the intake staging consumed and cleared" mean precisely, given `.kb/_intake/`'s own `README.md`?** The kb-ingest glob risk (`project.md` risk table, "The kb-ingest glob and a second wave") notes the README is picked up with the wave's content. **Deferred to spec**: the spec must state explicitly that the README is dropped at the approval gate before ingest runs (per the memory note "kb-ingest's glob includes the intake README — drop it at the approval gate"), and that this wave carries a distinct wave id so it does not collide with the two prior 2026-08-10 waves already reflected in `.kb/maps/open-questions-index.md:33,77`.
- The DoD 13 caveat does not touch this story.

## Decision

The problem this slice half solves is making the audience durable without inventing a shortcut around the process that exists to keep durable knowledge honest: `.kb/product/README.md` already states that closeout performs this promotion and that an early or hand-written promotion "gives a guess the standing of a finding" — this story is where that promise is kept or broken. The spec that follows will specify the exact `/redkiln:kb-ingest` invocation against the wave this project's staging story produced, the expected atom shapes (`concept`/`authority_tier: product` per persona, `playbook`/`authority_tier: product` per journey — **three concepts and four playbooks**, per the resolved count, the extra playbook being the evaluation journey), and the commit-provenance check AC-008 requires (atoms arriving from `.kb/_intake/`, intake consumed and cleared, no hand-authored commit in the range).

## The wrong implementation

Writing the three (or four) concept and playbook files directly into `.kb/product/` by hand — correctly shaped, correct frontmatter, `authority_tier: product` set, `redkiln validate --kb` even passing — because it is faster than staging a wave and running `/redkiln:kb-ingest`, and because the *content* would be identical either way. This satisfies AC-007 and even AC-009 (the qualifying sentence can be hand-typed too) while failing AC-008 outright, and it is exactly what commit `0269720` was reverted for: "producing the directory layout of the process without the process" (`project.md` risk table, "Hand-authoring the product atoms"; `_grounding.md:41-50`). The failure is invisible to a reviewer who only opens the resulting atom files — they look correct. What catches it: AC-008's own check is on the *commit range*, not the files — `git log --diff-filter=A` on the `.kb/product/` paths must show them arriving in a commit whose diff also touches `.kb/_intake/` (removing the staged drafts) and whose message or PR trail shows `/redkiln:kb-ingest` ran, per `.redkiln/config.yaml:73`'s `require_commit_provenance`. A hand-authored commit adding the same files with no corresponding intake-directory change is the mutant this check exists to reject.

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
