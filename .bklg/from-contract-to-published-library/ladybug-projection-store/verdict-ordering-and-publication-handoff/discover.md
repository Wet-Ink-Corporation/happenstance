---
item: HS-S0083
stage: discover
created: 2026-08-12T13:02:52.581Z
updated: 2026-08-12T13:02:52.581Z
template_sig: 86ce4036
rendered_sig: b684f776
---

# Discover — Merge the verdict ahead of publication, and hand over what it owes

## Signal Ledger

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line — merge the verdict to the initiative branch ahead of `publication-and-positioning`'s gate, state that ordering explicitly rather than leaving it to the DAG, and hand HS-P0016 the build number, the docs.rs failure and the `[package.metadata.docs.rs]` question | `_storymap.md:49` | Two deliverables: an ordering claim that is checkable, and a handoff whose contents are enumerated. |
| **AC-011** — the verdict document is merged to the initiative branch **before** `publication-and-positioning` opens its gate, and the design brief states that ordering explicitly rather than leaving it to the DAG | `project.md:222-224` | "Explicitly rather than by inference" is the AC's own wording, and it is aimed at the failure of relying on a graph nobody re-reads under pressure. |
| **DR-8** — the verdict must land before publication opens so that a "did not hold" is heard *before* the surface is published rather than after; this project `blocks` publication and the timing must be stated rather than inferred | `project.md:166-170` | The whole reason the edge exists is asymmetry of consequence: a late verdict is not late, it is useless. |
| The edge is one-directional and timed — *"if schedule pressure inverts that edge, the initiative's own exit criterion 4 is what breaks"* | `project.md:296-298` | The named risk is not that someone forgets, but that someone reorders deliberately under pressure. A record is the only defence. |
| **Unlocks** — by decomposition gate decision 1, `0.2.0` waits for all four adapter projects, because AC-08 requires the published compliance claim to name **which** implementations it was checked against | `project.md:250-255` | The handoff is what makes that claim writable for this adapter. |
| Proof is a `git log` ordering check, **not CI-enforced** — no such gate exists in `.redkiln/config.yaml`; it is a human-checked ordering claim recorded in closeout, *"the same discipline AC-001 uses for its own commit-order proof"* | `_decomposition.md:481` | The one AC in this project with no automated enforcement anywhere. Naming that is the mitigation. |
| Merge order 5 — `freeze-verdict` merges before HS-P0016 opens its gate; *"that edge is the whole of DR-8 and AC-011"* | `_storymap.md:139-143` | The story map states the constraint at the slice level; this story is where it becomes an artefact. |
| The docs.rs handoff — docs.rs itself fails to build `lbug` 0.19.1, removing `publish = false` makes it a real consequence, and *"this project's obligation is to hand that project the number and the `[package.metadata.docs.rs]` question"*, modelled on `crates/happenstance-core/Cargo.toml:54-56` | `_decomposition.md:401-406`; `crates/happenstance-ladybug/src/lib.rs:31-32` | Phase 12's proof artefact is docs.rs green under `--all-features` (`RUNBOOK.md:162`), so this is a proof obligation HS-P0016 cannot meet without knowing. |
| PS-3 — whether the port ships behind `unstable-projection` — is marked *"6, decided at 12"*; this project supplies **a data point, not the decision** | `project.md:112-114`; `RUNBOOK.md:601` | Evidence and opinion must be separable in the handoff, and any opinion labelled as one. |
| `depends_on: freeze-verdict-document` — supplies the artefact being ordered and handed over | `_storymap.md:49` | Without it there is nothing to merge ahead of anything, and the ordering claim has no subject. |
| The other handoff contents come from siblings — the cold/warm build numbers and the CI shape from `cold-build-cost-and-ci-shape`, the package-complete state and held name from `package-completeness-and-name-claim` | `_storymap.md:46-47`; `project.md:214-221` | Both are transitive dependencies through the verdict, so every fact handed over is already merged when this story writes it down. |
| Incidental defects found in passing route to the `support` initiative | `project.md:132`; `.redkiln/config.yaml:5` | Anything discovered while assembling the handoff that is not this project's does not get carried into HS-P0016 as an informal to-do. |

## Questions

**Is there a mechanical gate enforcing the ordering? — answered: no, and saying
so is the point.** `.redkiln/config.yaml` wires no such check
(`_decomposition.md:481`), so AC-011 is discharged by a human-checked commit-order
claim recorded in closeout. The mitigation carried into `spec` is to make the
claim *checkable* rather than merely asserted: it names two commit SHAs — the
verdict document's merge commit, and the commit at which HS-P0016's gate opened —
so a reviewer can verify it in one command instead of trusting the sentence.

**What exactly is handed over? — answered, and enumerated rather than gestured
at.** Four items, each traceable to where it was produced: the cold and warm
`lbug` build figures with their toolchain and runner stamps, plus the CI shape
that was implemented and why (from `cold-build-cost-and-ci-shape`); the fact that
docs.rs fails to build `lbug` 0.19.1, together with the
`[package.metadata.docs.rs]` question modelled on
`crates/happenstance-core/Cargo.toml:54-56`; the crate's package-complete state
and the held crates.io name (from `package-completeness-and-name-claim`); and the
verdict itself, with PS-3's data point called out as a data point.

**Does the handoff carry a recommendation on `unstable-projection`? — deferred to
HS-P0016 as a decision, permitted here only as a labelled opinion.** PS-3 is
*"decided at 12"* (`RUNBOOK.md:601`) and this project supplies evidence
(`project.md:112-114`). A recommendation is not forbidden, but it must be visibly
separated from the evidence, because an opinion formatted like a finding is how a
data point becomes a decision without anyone taking one.

**How does `lbug`'s blocking API meet a non-blocking port (ADR-0025)? — settled
upstream, and carried here as a positioning fact rather than re-opened.** Whatever
Q3 chose, it is something a stranger meets on the crate page and something
HS-P0016's landing copy has to be truthful about, so the handoff states it
alongside the docs.rs failure.

**What counts as "structurally unlike"? — settled at HS-S0074 and carried as
context for the compliance claim.** The published claim must name which
implementations the port was checked against (`project.md:250-255`), and the axes
are what make "and one of them was structurally unlike the others" a statement
with content rather than a flourish.

**Deferred to `spec`:** where the handoff lives — a section of the verdict
document, a note in HS-P0016's own intake, or both — and whether the ordering
claim is recorded in this project's closeout, the verdict, or `RUNBOOK.md`'s phase
11 body.

## Decision

The verdict is only protective if it arrives before the decision it is meant to
inform, and nothing in this repository enforces that: `.redkiln/config.yaml` has
no gate for it, the dependency DAG is a planning artefact rather than a check, and
the pressure at this point in an initiative runs entirely toward publishing
first. So this story turns the ordering into an artefact — a claim naming the
verdict's merge commit and the commit at which `publication-and-positioning`
opened its gate, stated explicitly rather than left to be inferred from the graph
— and enumerates the handoff HS-P0016 inherits: the cold and warm build figures
with the CI shape that was implemented and why, the docs.rs failure against `lbug`
0.19.1 with the `[package.metadata.docs.rs]` question, the crate's
package-complete state and held name, and the verdict with PS-3's contribution
marked as a data point rather than a recommendation. The spec will cover the
ordering claim's two SHAs and where it is recorded, the enumerated handoff with
each item traceable to the story that produced it, and the separation of evidence
from opinion. Nothing `[FROZEN]` is touched here: PS-3's marker is HS-P0016's to
move (`RUNBOOK.md:601`), and if the verdict this story is ordering says *did not
hold*, the response remains a decision atom and a re-plan — handed over as such,
never as an edit to a clause or a suggestion that publication proceed around it.

## The wrong implementation

**The verdict merged after publication's gate opened, with the ordering claim
written as if it had not been.** Every artefact exists and every content check
passes: the verdict is dated, commit-pinned, complete under DR-4; AC-007 is
satisfied; the handoff is written; the closeout says "the verdict preceded
publication." The only thing that disagrees is `git log`, and nobody reads `git
log` at closeout unless the record asks them to. This is the exact shape of
AC-001's risk, one story later and with more at stake, because by then the
surface is published and a "did not hold" has nowhere to go. The guard is that the
claim cites **two commit SHAs** rather than asserting an order, which converts it
from a sentence a reviewer must believe into one they can check in a single
command — and which makes the false version require fabricating a SHA rather than
phrasing a sentence loosely.

**The handoff written as links rather than facts.** "See the freeze verdict for
the run, `RUNBOOK.md` phase 11 for the build cost, and this project's packaging
story for the crate state." It satisfies "hand over", it is accurate on the day it
is written, and it pushes the entire reading cost onto the project least able to
pay it — the one deciding what to publish under time pressure. It also decays:
this initiative's folder is reconciled and archived at closeout, so
`.bklg/from-contract-to-published-library/…` paths move, and a link into a
planning artefact is the first thing to break. The rule for `spec` is that the
number, the failure and the question appear **inline** in the handoff, with the
citation as provenance rather than as the content.

**The handoff that omits the docs.rs failure.** The most consequential omission
available here, and the most natural: the crate is package-complete, the name is
held, the suite is green, the verdict is written — and `lbug` 0.19.1 does not
build on docs.rs, which happens to be the exact form of `publication-and-positioning`'s
own proof artefact, docs.rs green under `--all-features` (`RUNBOOK.md:162`). Every
check in this project passes; the next project inherits a proof obligation it
cannot meet and discovers this after publishing or after burning its schedule on
it. Nothing automated catches an omission from a handoff, so the guard is that
`spec` enumerates the handoff's contents as a checklist with each item traceable
to the story that produced it, and that the closeout reads the list rather than
the prose.

**PS-3 handed over as a recommendation dressed as a finding.** A handoff section
reading "the projection port need not ship behind `unstable-projection`" — stated
in the same register as the measured facts around it, with no marker separating
the two. PS-3 is *"decided at 12"* and this project supplies a data point
(`project.md:112-114`; `RUNBOOK.md:601`); a decision taken here by presentation is
a project boundary crossed without anyone noticing they crossed it, and it is
harder to undo than an explicit overreach because there is nothing to point at.
The guard is a labelled separation between what was observed and what this project
thinks about it.

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
