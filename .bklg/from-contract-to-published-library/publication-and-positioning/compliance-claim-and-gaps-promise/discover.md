---
item: HS-S0093
stage: discover
created: 2026-08-12T13:03:01.640Z
updated: 2026-08-12T13:03:01.640Z
template_sig: 86ce4036
rendered_sig: 1990f9c3
---

# Discover — The compliance claim ships with its evidence, and the gaps promise is findable

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Storymap slice | `_storymap.md`:65 | Claim-with-evidence form for "DCB-compliant" — implementations, date, reachable link — plus the positions-and-gaps statement at DT-4's chosen site, within one hop, without implying ownership of `read_from_a_gap_position`. |
| AC-009 | `project.md`:258-260 | "DCB-compliant" evidence reachable from a published artefact, names the implementations the suite ran against, and carries the date it ran. |
| AC-010 | `project.md`:262-264 | The positions-and-gaps promise, in plain language, without reading source, at DT-4's chosen site. |
| `dependsOn: first-contact-design-resolutions` | manifest | DT-4's resolution (site, form, register) is a hard input this story cannot start ahead of. |
| DT-4's resolution, exact form | `_design.md`:228-261 | Stated in full in the Guarantees block of `crates/happenstance/README.md`:41-49 and `crates/happenstance-core/README.md`:43-54, ≤3 rendered lines + 1 nested line, exactly one link into a clause ID; must not imply ownership of `read_from_a_gap_position`; the nested line states the absence with its reason. |
| The claim-with-evidence precedent | `README.md`:227-234 | The former-name section — states the fact plainly, cites `ADR-0005` by path rather than asserting authority. `_grounding.md`:144-147 names this as the exact form AC-009's claim and AC-006's MSRV promise both take. |
| No existing claim to redirect | `_grounding.md`:139-143 | "no match for that phrase anywhere in `README.md` or any crate README" — this is new copy, not an edit to something that already exists. |
| The unowned rule, precisely | `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`:44-58 | `read_from_a_gap_position` is ES-9's owed rule, named by both ADR-0011 and ADR-0013, claimed by neither. "Two accepted decisions have now looked directly at this rule and neither took it." The copy must not say or imply the library owns this rule. |
| ADR-0010's provenance discipline | `.kb/decisions/0010-the-suite-must-prove-itself.md` | Every conformance rule carries a mutant with stated provenance; a skip is always reported, never silent. This is what makes a passing rule mean something to cite — the evidentiary backbone the compliance claim points to (`_grounding.md`:27-31). |
| Testing brief's named wrong implementations | `_decomposition.md`, Testing brief AC table, AC-009 and AC-010 rows | AC-009: "a claim with no date, or one naming implementations broader than what actually ran the suite — `ADR-0010`'s provenance discipline is what makes 'passed' mean something to check." AC-010: "a statement implying ownership of `read_from_a_gap_position`, which no accepted decision actually claims." |
| Density budget for the compliance block | `_design.md`:448 | ≤6 rendered lines: claim 1, implementations 1–2, date 1, link 1 — "Longer and it stops reading as a single checkable claim." |
| Which implementations can actually be named | `project.md`'s Dependencies section (315-329) | `sqlite-durable-store`, `cloudflare-durable-object-store`, `postgres-and-neon-stores` are blockers this project accepted specifically so AC-009 can name more than one adapter, not a single-implementation claim. |

## Questions

- **Which implementations does the compliance claim actually name?** Depends on which of `sqlite-durable-store`, `cloudflare-durable-object-store`, `postgres-and-neon-stores` and `ladybug-projection-store` have actually run the conformance suite and passed by the time this story reaches spec/implementation — genuinely not knowable at discovery time, since all are upstream sibling projects still in flight. Deferred: the claim names exactly the implementations that passed by the date it is written, per the risk register's own framing (`project.md`:434, *"passed against N adapters" is a snapshot*) — never a forward-looking or aspirational list.
- **Exact wording for the non-ownership nested line.** `_design.md`:255-259 specifies the shape (a subordinate nested bullet, links the open question) but not the sentence. Deferred to spec, to be drafted against `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`'s actual current text rather than paraphrased from memory.

## Decision

The "DCB-compliant" claim ships on the packaged READMEs in the existing claim-with-evidence form (`README.md`:227-234's shape): the implementations the suite actually ran against, the date it ran, and one reachable link — never a broader or aspirational list. The positions-and-gaps promise lands in the Guarantees block per DT-4's resolution, stating what is true today (positions unique and increasing, gaps permitted, do not assume `n+1`) and explicitly not claiming ownership of `read_from_a_gap_position`, with a nested line naming that absence and its reason. Spec will fix the exact implementation list against whichever sibling adapter projects have actually closed by then, and draft the non-ownership sentence against the open question's current text.

## The wrong implementation

A compliance block that names implementations and includes a link — satisfying a naive "does it cite evidence" check — but lists every adapter this initiative *intends* to build rather than only the ones that had actually run the conformance suite and passed as of the stated date, or omits the date entirely so the claim reads as timeless rather than as a snapshot. This is exactly the testing brief's named wrong implementation for AC-009. It is a real risk here specifically because the implementation list is naturally written from `project.md`'s Dependencies section (which names four sibling projects as blockers) rather than from each sibling's actual, closed pass/fail state — conflating "blocks this release" with "already passed the suite" would produce a claim broader than what was checked. A second, equally real form: a positions-and-gaps sentence that says the library "guarantees reads work correctly at a gap position" without the nested non-ownership line — which reads as claiming `read_from_a_gap_position`, a rule `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md` records as owned by nobody. What catches it: cross-checking the compliance block's named implementations against each sibling project's actual closed report (not its Dependencies-section listing), and reading the positions-and-gaps sentence specifically for any phrase that could be read as claiming the unowned rule.

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
