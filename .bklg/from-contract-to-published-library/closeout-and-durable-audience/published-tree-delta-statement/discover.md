---
item: HS-S0128
stage: discover
created: 2026-08-12T13:03:53.301Z
updated: 2026-08-12T13:03:53.301Z
template_sig: 86ce4036
rendered_sig: d6b05480
---

# Discover — The DoD 13 delta between the published tag and the closeout tree, stated

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-liner: "State the DoD 13 delta rather than glossing it: the published `0.2.0` tag, the commits between it and the closeout tree, the two projects that own them, and the cited gate decision that keeps the published surface unchanged across the delta." | `_storymap.md:56` | Four named things must appear: the tag, the commit range, the owning projects, and the gate-decision citation — not a paraphrase. |
| AC-004 | `project.md:204-206` | "The record names the published `0.2.0` tag, the commits between it and the closeout tree, the projects responsible, and the reason the published surface is unchanged across that delta." |
| DoD 13, verbatim | `../initiative.md:396-397` | "The gate is green on the assembled whole. `cargo xtask ci` passes, including the specification cross-reference step, **on the exact tree that was published.**" The literal phrase this story exists to qualify. |
| DR-4 — the DoD 13 delta is stated | `project.md:139-143` | Names the tag, `replication-identity-and-ingest` (HS-P0017) and `retention-and-incomplete-logs` (HS-P0018) as the two post-publish projects, and cites the gate decision constraining retention's answer to need no published-surface change. |
| The gate decision, item 4, verbatim | `../_decomposition.md:239-249` | "Retention's answer is constrained to what needs no published-surface change... If `retention-and-incomplete-logs` answered with a decision rather than a reasoned refusal, that would be a change to a published port — under 0.x a minor bump, meaning a `0.3.0` this initiative's exit criteria do not contemplate." This is *why* the delta does not change the published surface, not an assertion. |
| Risk table, row 2 | `project.md:298` | "'the exact tree that was published' is false by construction once HS-P0017 and HS-P0018 land after HS-P0016... If that constraint was broken upstream, this project reports it — it does not absorb a `0.3.0` the exit criteria do not contemplate." |
| No `0.2.0` git tag exists yet in this tree | Checked directly: `git tag -l` returns empty in this worktree | At discover time the publish event this story will diff against has not yet happened in this checkout's history; the tag, and the commit range against it, are facts this story's *spec/implementation* observes once `publication-and-positioning` has actually merged and tagged — not something discover can state today. |
| `dependsOn: dod-set-re-observation-record` | `_storymap.md:55` (same slice) | The two stories are "one artefact with two halves... read in the same sitting by the same reader; the delta exists precisely to qualify a phrase in the DoD preamble the table is answering" (`_storymap.md:75-77`). |

## Questions

- **Is the `0.2.0` tag and the HS-P0017/HS-P0018 commit range observable yet?** Not in this worktree as of discover time — no tag exists. **Deferred to spec/implementation**: by the time this story actually executes (rank 6, after all nine siblings including HS-P0016 have merged per `project.md` *Dependencies*), the tag and range will exist; this discover pass records the citation path (`git log <tag>..HEAD`, filtered to HS-P0017/HS-P0018 commits) rather than the literal commits, which cannot be known before those projects land.
- **What if retention's actual, merged answer *did* require a published-surface change** — i.e., the upstream gate-decision constraint (item 4) was violated? **Answered by DR-4/AC-004's own design, not by this story inventing a new rule**: `project.md`'s risk table (row 2, `project.md:298`) is explicit that this project "reports it — it does not absorb a `0.3.0`." This story's spec must therefore include the negative case: if the constraint was broken, the delta statement says so plainly and routes the finding to `findings-disposition-register`, rather than silently writing a `0.3.0`-shaped statement as if it were the plan.
- The evaluator-persona question does not touch this story. Noted only for completeness.

## Decision

DoD 13's own sentence — "on the exact tree that was published" — is false by construction the moment a tenth project (this one) exists at all, because `replication-identity-and-ingest` and `retention-and-incomplete-logs` are sequenced *after* `publication-and-positioning` in merge order (`project.md` *Dependencies*), so the closeout tree this project's own gate runs on necessarily contains commits the published `0.2.0` artifact does not. The honest response is not to satisfy the sentence literally (impossible) nor to silently reinterpret "exact tree" to mean "the tree at closeout" (which would let a real published-surface regression slip through unnoticed). It is to state the delta and bound it: name the tag, name the commits, name the two owning projects, and cite the upstream constraint (gate decision item 4) that is the actual reason those commits do not change what a consumer of `0.2.0` sees. The spec that follows will specify the concrete `git log`-based procedure for computing the commit range once the tag exists, and the exact sentence structure the delta statement uses to distinguish "the published surface is unchanged, here is why" from "it was not, here is the finding."

## The wrong implementation

A delta statement that says only "DoD 13 is satisfied; the published tree and the closeout tree are functionally equivalent" without naming the tag, without listing the actual commits or the two owning projects, and without citing the gate-decision constraint that is the *reason* equivalence holds. It would read as reassurance and satisfy a skim of AC-004's first clause ("the reason the published surface is unchanged"), but it fails the clause's actual demand — "the record names the published `0.2.0` tag, the commits between it and the closeout tree, the projects responsible" (`project.md:204-206`) — and it is unfalsifiable: a reader cannot check "functionally equivalent" against anything. Worse, it is the shape a genuine violation would also produce if someone wanted to paper over it — a bare assertion of equivalence is indistinguishable from a bare assertion covering up the exact failure mode DR-4 exists to catch (retention answering with a real decision rather than a refusal, which the gate-decision item 4 forbids). What catches it: the spec must require the four named elements as separate, checkable fields, not prose; a reviewer can then independently run `git log <tag>..HEAD` and confirm the commit list matches what's claimed, and confirm the citation to `../_decomposition.md:239-249` actually supports "unchanged" rather than merely gesturing at it.

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
