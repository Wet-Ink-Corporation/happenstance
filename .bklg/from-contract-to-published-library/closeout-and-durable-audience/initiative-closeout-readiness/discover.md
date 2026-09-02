---
item: HS-S0135
stage: discover
created: 2026-08-12T13:04:02.524Z
updated: 2026-08-12T13:04:02.524Z
template_sig: 86ce4036
rendered_sig: 68878bed
---

# Discover — No open child of HS-I0006, and the closeout links the promoted atoms

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-liner: "Show the initiative can close: every sibling HS-P0010…HS-P0018 closed out with no open child of HS-I0006 under `redkiln status`, and the initiative's own closeout linking the promoted product atoms." | `_storymap.md:63` | Two proofs: a `redkiln status` read (no open child) and a link check (closeout references the new atoms). |
| AC-014 | `project.md:241-243` | "All nine sibling projects are closed out and the initiative's closeout links the promoted product atoms, evidenced by `redkiln status` showing no open child of HS-I0006." |
| DR-13 — The initiative can close | `project.md:184-186` | "Every child HS-P0010…HS-P0018 is closed out, the initiative's own closeout links the promoted product atoms, and what was learned is harvested rather than left in the backlog (exit criteria 7–8)." |
| Exit criteria 7 and 8, verbatim | `../initiative.md:578-581` | "7. The personas and journeys this work served exist as durable product-layer atoms... 8. Every project is closed out, the whole gate is green on the published tree, and..." (this project owns 7 and 8 per `project.md:61`). |
| Current initiative-level status (checked directly) | `../initiative.md:10-12` | `status: planning`, `stage: storymap` at the initiative frontmatter level as of this discover pass — this is the state *before* this project's own stories (including this one) have executed; this story's own job is to move that state to closed, not to observe it closed already. |
| `dependsOn: product-atom-promotion-via-kb-ingest, findings-disposition-register` | `_storymap.md:63` | Cannot claim initiative-closeout readiness before the durable atoms exist (exit criterion 7) or before every finding this project surfaced has a recorded destination (nothing left ambiguous for a reader of the closeout to trip over). |
| Merge order, step 5 (this story last) | `_storymap.md:152-155` | "Last by construction: it measures the tree every prior slice contributed to and routes what they found" — same reasoning applies here as the terminal story of the terminal project. |

## Questions

- **Are all nine sibling projects (HS-P0010…HS-P0018) actually closed out yet, as of discover time?** Not fully confirmed by this discover pass — a scan of sibling `project.md` frontmatter was inconclusive at the level of detail this discover stage warrants, and `redkiln status`'s own live read is the authoritative source, not a manual grep of files that may lag the tool's index. **Deferred to spec/implementation**: this story's spec runs `redkiln status` itself against the tree at execution time and records the actual output, rather than this discover pass asserting a conclusion from static files that could be stale by the time the story runs (this project sits at rank 6, after all nine, by design — so the expectation is yes, but the check must be live).
- **Does "the initiative's own closeout links the promoted product atoms" mean this *project's* closeout, or the initiative-level `/redkiln:closeout`'s?** Read literally against `../initiative.md`'s own exit criteria and `project.md`'s DR-13, it is the **initiative-level** closeout artifact (produced by `/redkiln:closeout` against `HS-I0006`, downstream of this project closing) that must carry the links — this project's own closeout is one input to that, not the artifact AC-014 is checking. **Deferred to spec**: the spec should state this distinction explicitly so a reader does not conflate this project's own closeout report with the initiative's.
- The evaluator-persona finding and DoD 13's caveat are not this story's to resolve — they are inputs this story checks were *routed* (via `findings-disposition-register`) and *promoted* (via `product-atom-promotion-via-kb-ingest`), respectively, not re-litigated here.

## Decision

This story is the final gate before the initiative itself can be declared closeable: it is not more re-observation, it is the check that every prior piece of re-observation actually reached a terminal state — no sibling project left open, no promoted atom missing, no finding left unrouted (which `findings-disposition-register`, its co-dependency, already guarantees by construction). The spec that follows will specify the literal `redkiln status` invocation and the exact criterion for "no open child of HS-I0006" it checks against, and the check that the initiative's own closeout artifact (once `/redkiln:closeout` runs against HS-I0006) actually cites the three product atoms this project promoted rather than leaving them orphaned facts nobody links to.

## The wrong implementation

Declaring this story's AC-014 satisfied because *this project's own* nine dependency edges are all "capability" stories that have completed, without ever running `redkiln status` against the live backlog to check the other nine *sibling projects'* actual close state. This confuses "everything this project's own DAG required has executed" with "everything the initiative's DAG required has executed" — the two are different claims, and AC-014's text is explicit that it is the latter: "All nine sibling projects are closed out... evidenced by `redkiln status` showing no open child of HS-I0006" (`project.md:241-243`), not "every story inside this project's own board is done." A sibling project that is functionally finished but never formally advanced past `review` in redkiln's own state machine would still show as an open child, and a story that never actually invoked `redkiln status` would have no way to notice. What catches it: this story's ledger must cite the literal `redkiln status` output (or `redkiln next` showing nothing actionable at initiative grain) as its evidence, not an inference from this project's own dependency graph having resolved.

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
