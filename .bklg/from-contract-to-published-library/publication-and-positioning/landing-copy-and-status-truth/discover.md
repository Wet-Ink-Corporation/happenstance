---
item: HS-S0092
stage: discover
created: 2026-08-12T13:03:00.637Z
updated: 2026-08-12T13:03:00.637Z
template_sig: 86ce4036
rendered_sig: 7206ff7f
---

# Discover — The first screen carries the lead claim, and no sentence on it is false

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Storymap slice | `_storymap.md`:64 | Carry DT-1's lead claim onto the packaged `happenstance` README's first screen, DT-5's maturity treatment and DT-6's peer positioning into existing slots, correct the four stale strings, spell "passes the conformance suite" differently from "compiles," never suppress the 49 provisional clauses while listing the frozen ones, check every link and anchor mechanically. |
| AC-007 (this story's share) | `project.md`:251-253 | The published crate "looks finished" — copy half of the split the coverage table names (`_storymap.md`:130): "*the copy*" vs. `guarantees-and-docs-rs-presentation`'s "*the docs.rs configuration*" vs. `rendered-page-preflight`'s "*the dated read*". |
| AC-011 (this story's share) | `project.md`:266-269, `_storymap.md`:134 | "*the resolution naming what lost*" (owned by `first-contact-design-resolutions`) vs. "*the surface carrying it*" (this story). |
| `dependsOn` | manifest | `first-contact-design-resolutions` (the DT-1/4/5/6 winners), `crate-set-decision` (what the status table/disambiguation triad names), `projection-port-ship-shape` (whether a `doc(cfg)`-gated item needs a status line). |
| The four stale strings, named exactly | `_decomposition.md`, UX brief "Live defects in state copy" (lines 118-129) | `README.md:11-16` ("Nothing is published yet"), `README.md:104-109` (`happenstance = "0.1"` / "does not resolve yet"), `README.md:85-90` (🔲 rows this initiative's upstream projects made real), `crates/happenstance/README.md:6-11` ("this crate is currently a facade… adds nothing yet"). Verified current text at `README.md`:11-16, 79-98, 100-137 and `crates/happenstance/README.md`:6-20 (read directly). |
| DT-1's resolution to carry | `_design.md`:196-224 | Proof-led identity block: H1 + one-line identity → status callout → disambiguation triad; the edge story and two-entry-points options both lost, and why. |
| DT-5's resolution to carry | `_design.md`:265-297 | The exact census sentence form (dated, four counts, four inline definitions, one link), sited immediately after the status callout, never dropping the 49-provisional count while listing frozen guarantees (IQ-2/AP-4). |
| DT-6's resolution to carry | `_design.md`:301-324 | Full form in the existing Prior art slot (`README.md`:218-225); one-sentence reduced form on the packaged README; no feature matrix ever (AP-5). |
| Composition, region-by-region | `_design.md`:332-357 (`crates-io-happenstance` table) | R1 Identity, R2 State, R3 Audience, R4 Maturity census, R5 Badges (moved below R4), R6 Compliance claim, etc. — the exact order this story implements. |
| Anti-patterns this story is checked against | `_design.md`:620-654 (AP-1 through AP-15) | AP-1 (audience block must be visible without scrolling at 1024×768), AP-2 (no glyph-only status cell), AP-3 (two visible maturity counts / an undated count), AP-4 (frozen list with no provisional mention), AP-9 (the four stale strings), AP-11 (8-row table on a packaged surface), AP-14 (badges above the callout). |
| Density budget, binding | `_design.md`:418-478 | 14-line first-screen budget at 1024×768, accepted as **full at 0.2.0** per Sign-off (`_design.md`:751-762) — no sixth region, no callout regrowing to three lines. |
| UX brief's status vocabulary requirement | `_decomposition.md`, UX brief "States this surface must render" (97-116) | Must distinguish: published-and-passes-suite; published-no-suite-applies; in-tree-compiles-not-suite-passed; skeleton; deliberately-out-of-train; clause maturity — six genuinely different states, never spelled the same. |
| AC-UX-011, mechanical link check | `_decomposition.md`, UX brief (367-370) | No dead link, no dead anchor; a renamed heading either keeps its anchor or every inbound reference updates in the same change. Known live anchors: `README.md`:9 → `#licence`, `README.md`:16 → `#status`. |

## Questions

- **Does the four-crate-vs-three-crate status table change shape once `crate-set-decision` lands?** No — `crate-set-decision` confirms three; the status table (`README.md`:79-98) already lists eight rows including adapter stubs that stay unpublished, and nothing in the crate-set decision removes those rows from the *table* (only from the *publishable set*). Answered: the table's row count is unaffected; only the disambiguation triad on the packaged surfaces is capped at three entries per `_design.md`:446.
- **Where exactly does the status vocabulary distinction (six states) get spelled out — is it new prose, or does the existing table's glyph+words already suffice?** The existing table already carries words per cell (`README.md`:83-90, verified compliant), but the *number* of distinguishable states it currently encodes (✅/🔲) is fewer than six. Deferred to spec: whether the GitHub-only table needs new state words, or whether the crates.io status callout (which is prose, not a table) carries the finer distinctions instead — `_design.md`'s Composition table assigns the callout, not the table, to that job on packaged surfaces.
- **What is the exact wording for the corrected `crates/happenstance/README.md`:6-11 callout, now that the typed layer may have shipped?** Depends on `typed-layer-and-alpha-release`'s actual state at spec time, which is a sibling project's output this story consumes but does not control. Deferred to spec/implementation, to be written against whatever is actually true of the tree at that point (IQ-7).

## Decision

The packaged `happenstance` README's first screen carries DT-1's proof-led identity block, correcting all four known-stale strings and replacing the glyph-only reading of "done" with a status vocabulary that distinguishes passes-the-suite from merely-compiles. DT-5's dated census sentence and DT-6's peer-positioning sentence land in the slots `_design.md` already specifies (immediately after the status callout, and the existing Prior art slot respectively), never suppressing the 49 provisional clauses while the frozen ones are visible. Every link and anchor is checked mechanically against the two known live inbound anchors. Spec will write the exact corrected prose for each of the four stale strings and confirm against whatever `typed-layer-and-alpha-release` and `crate-set-decision` actually produced by then, rather than assuming their outcomes.

## The wrong implementation

A landing-copy edit that replaces all four named stale strings with fresh, dated, accurate text — passing a literal "do these four exact strings still appear" check — and adds the disambiguation triad and status callout exactly as `_design.md`'s Composition table specifies, but that carries the existing status-table row logic forward unexamined: a status cell whose only signal is a glyph (✅/🔲/🔩) with no words distinguishing, say, "passes the conformance suite" from "compiles but has not run it" — the exact two states `CLAUDE.md`'s rule ("an adapter that compiles but has not run the suite is not an adapter") says must never be spelled the same. Such an edit would pass a naive review that only checks the four named strings are gone, while still committing AP-2 (a status cell whose only signal is a glyph) and the deeper defect the UX brief's "States this surface must render" section exists to prevent — a reader cannot tell, from the table alone, whether a ✅ row actually ran the suite or merely built. What catches it: `_design.md`'s AP-2 is checked directly on the rendered page (not the source), and the UX brief's six-state list is used as a literal checklist against every status cell on every published surface, not only the four cells that happened to contain one of the four named stale phrases.

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
