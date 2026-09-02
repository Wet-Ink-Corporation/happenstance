---
item: HS-S0134
stage: discover
created: 2026-08-12T13:04:01.327Z
updated: 2026-08-12T13:04:01.327Z
template_sig: 86ce4036
rendered_sig: 8283d0f0
---

# Discover — Every finding routed to an owner, none absorbed here

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-liner: "Route every finding the re-observation surfaced — `support`, a new item against the owning sibling, or a decision atom plus a re-plan — and show zero were fixed inside this project." | `_storymap.md:62` | Every finding needs a named destination; the check is the *absence* of a fix commit inside this project's own stories. |
| AC-013 | `project.md:237-240` | "Every defect surfaced by the re-observation appears in the record with its destination... Zero are fixed inside this project." |
| DR-12 — Findings are dispositioned, not absorbed | `project.md:179-183` | Three routing lanes named explicitly: incidental defect → `support` (`.redkiln/config.yaml:5`); genuine cross-project interaction → a new item against the owning sibling, not a fix here; anything touching a `[FROZEN]` clause → a new decision atom and a re-plan. |
| `support_initiative: support` | `.redkiln/config.yaml:5` | The literal config key that names where incidental defects go. |
| Risk table, "Pressure to fix what re-observation finds" | `project.md:299` | "The whole value of this project is that it is the first tree where a cross-project interaction is visible. That makes it the project most tempted to absorb a fix. AC-013 makes routing the deliverable." |
| The evaluator/DT-1 finding, already identified | `persona-and-journey-intake-staging/discover.md` (this project, upstream slice) | A concrete instance of DR-12's second lane: a genuine cross-project interaction (this project's testing brief DR-10 vs. `publication-and-positioning`'s signed-off DT-1 resolution) that `persona-and-journey-intake-staging` resolved for its own purposes but explicitly deferred routing to *this* story, per that story's own Decision section. |
| `dependsOn`, all six prior capability stories | `_storymap.md:62` | This is the collection point — it cannot compile a findings register before every producing story has had the chance to surface something. Matches merge order step 5 (`_storymap.md:152-155`): "Last by construction: it measures the tree every prior slice contributed to and routes what they found." |
| Every finding candidate named in `project.md`'s own risk table | `project.md:293-306` | The risk table itself is a pre-registered list of the shapes a finding might take — DoD 13's phrase, the `!Send` flavour, positioning-date staleness, the kb-ingest glob/README, a seventh drift advisory — this story's spec should check against all of them, not only what happens to surface. |

## Questions

- **Is the evaluator/DT-1 discrepancy the only finding known at discover time, or should this story's spec anticipate others?** Only one is concretely identified this early, because most of this project's own re-observation work (the gate run, the DoD table, the audit tables, the health check) has not executed yet — findings from those stories are, by construction, not knowable before they run. **Deferred to spec/implementation**: the register's spec fixes the *shape* of an entry (finding, source story, destination lane, item/atom reference) now; the actual population happens once every dependency story has run.
- **What destination does the evaluator/DT-1 finding get?** DR-12's second lane fits precisely — "a genuine cross-project interaction → a new item against the owning sibling, not a fix here." The owning sibling is `publication-and-positioning` (HS-P0016), since it is that project's DT-1 resolution the discrepancy concerns, and its own sign-off already flagged this exact reversal risk (`publication-and-positioning/_design.md:764-767`). **Answered**: the register routes it as a new backlog item against HS-P0016 (or its closeout record, if already closed, per `.redkiln/config.yaml:5`'s support-initiative pattern for post-closeout items) asking whether the testing brief's now-superseded DR-10 text needs its own correction — this story does not resolve *that* question, only routes it.
- The DoD 13 caveat is explicitly not this story's finding to make — `published-tree-delta-statement` already states it as an expected, bounded, non-defect delta (per the upstream gate-decision constraint); it appears in this register's scope only in the negative case where that constraint was found broken (see that story's own Questions section).

## Decision

The problem this story solves is structural, not incidental: a project whose entire purpose is being the first tree where cross-project interactions become visible is, by the same logic, the project under the most pressure to just fix what it finds — and doing so would make the finding invisible to the sibling that actually owns the code or the decision, defeating the audit this whole project exists to be. This story is the discipline that keeps re-observation and repair separate. The spec that follows will specify the register's row shape (finding description, which upstream story surfaced it, the routing lane per DR-12, and the concrete destination — a `support` item id, a new item against a named sibling, or a decision-atom-plus-replan pointer), and will pre-populate the one finding already known at discover time (the evaluator/DT-1 discrepancy) as a worked example of the shape.

## The wrong implementation

A register that lists findings but silently resolves the ones this project's own stories are best positioned to fix — for instance, "corrected" the evaluator/DT-1 discrepancy by quietly editing this project's own promoted persona-atom count to match whichever resolution seemed more convenient, with no entry in the register at all, because "it was already handled upstream." This satisfies a shallow reading of "zero were fixed inside this project" (no *code* changed) while violating AC-013's actual intent: a `.kb/product/` atom count is exactly the kind of cross-project consequence DR-12 requires to be *routed*, not silently absorbed by whichever project happens to execute last and therefore gets to decide unilaterally. It is also the specific trap this project's own risk table names — "the project most tempted to absorb a fix" — dressed up as due diligence rather than as a fix, which makes it harder to catch than an outright code change. What catches it: cross-referencing every decision this project's own stories made that could have gone the other way (the evaluator count chief among them) against whether a corresponding register entry exists — a decision made here that resolves a cross-project disagreement, with no register entry, is a fix wearing the discover stage's own reasoning as cover.

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
