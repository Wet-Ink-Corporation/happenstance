---
item: HS-S0082
stage: discover
created: 2026-08-12T13:02:51.713Z
updated: 2026-08-12T13:02:51.713Z
template_sig: 86ce4036
rendered_sig: "06e50532"
---

# Discover — The freeze verdict, dated and either way

## Signal Ledger

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line — write the dated verdict (implementation, rules run, PS clause ids, commit SHA, *held* or *did not hold*), carrying the T1, GAT/ICE and `WriteTransactionInUse` findings, routing any "did not hold" to a decision atom and a re-plan while amending nothing `[FROZEN]` | `_storymap.md:48` | The verdict is a composite of four kinds of evidence, only one of which is a green suite. |
| **AC-007** — the verdict exists **and it exists either way**; its presence is not conditional on the result | `project.md:206-209` | The asymmetry is the point: an artefact that only appears when the news is good is not evidence. |
| **DR-4** — required whichever way the run goes, and must name which implementation, which rules, which PS clause ids, at which commit, on which date; *"a verdict that says only 'held' is not a verdict"* | `project.md:149-151` | Five named fields. A verdict missing any of them fails the AC regardless of how well it reads. |
| **AC-008** — a "did not hold" verdict routes to a decision atom and a re-plan and changes nothing frozen; `cargo xtask spec-trace` green and no `[FROZEN]` marker or text differs from its state at the project's start | `project.md:210-213`; `project.md:109-111` | The response to bad news is a new atom, never an edit — which is also `CLAUDE.md`'s standing rule that an accepted decision atom is immutable. |
| Backbone activity **E is unconditional**, and that asymmetry *"is what makes it evidence"* | `_storymap.md:24`, `:26-30` | Stated twice in the planning artefacts because it is the single claim most likely to erode under a green run. |
| PS-2 is `[FROZEN]` and states the bar: the suite must have failed `CheckpointOnlyStore` **and** two adapters at opposite ends of the batch-shape axis must have passed — rejecting *"the schedule that freezes this port against `MemoryProjectionStore` and an in-process rusqlite transaction… the same storage shape wearing two hats"* | `spec/SPECIFICATION.md:4760-4774` | The verdict is judged against this clause, and this clause is read-only here. Its "Rejects" paragraph is also the best statement of what the Ladybug run is *for*. |
| PS clause ids in scope — PS-4 – PS-6, PS-9, PS-11, PS-12, PS-15 (the batch shape and write seam row, *"6, re-tested 11"*), PS-34 (*"6, re-tested 11"*), PS-2 as the bar and PS-3 as a data point (*"6, decided at 12"*) | `project.md:48-56`; `_decomposition.md:477`; `RUNBOOK.md:598`, `:601`, `:602` | Nine clause ids, and the verdict must name them rather than gesture at "the projection clauses". |
| **AC-A04** — T1, T2 and T3 are honoured as decided, **or the deviation is written into the verdict** | `_decomposition.md:51-54` | The verdict is the designated place for every deviation the project accumulated, which is why it depends on all three preceding capability slices. |
| T1's finding — *the freeze deletes the counter-example that was evidence for freezing*; `LiveHandleProjectionStore` cannot exist under a frozen `type Batch;`, and its retirement costs nothing evidentially because both transcripts are preserved outside the crate | `_decomposition.md:314-331`; `references/adapter-shapes.md:169-195`, `:363-367` | A finding that no test produces and no green run reveals. It reaches the verdict only if someone carries it. |
| The GAT and the ICE — today's port is implementable *"only by stores that outlive every batch lifetime"* and the failure mode is a compiler panic rather than a diagnostic; if phase 6 kept the GAT, that is *"a finding about the port and belongs in the verdict, not a workaround to absorb quietly"* | `project.md:272-277`; `crates/happenstance-ladybug/src/live_handle.rs:38-66` | Named in the project's risk register as a verdict input, not an implementation detail. |
| `WriteTransactionInUse` under a concurrency-shaped rule — *"whether that is a declared capability limit or a rule defect is a question for the verdict, and the answer must not be 'retry until green'"* | `project.md:278-283`; `crates/happenstance-ladybug/src/projection_store.rs:204-213` | An open question this story is expected to *answer*, not merely report. |
| **M9** — `references/evaluation/` holds dated, commit-pinned evidence that is *"immutable evidence… dated, pinned to a commit… must be superseded rather than edited"*; the genre precedent is `phase-4-reconciliation.md` and `phase-4-5-reconciliation.md` | `_decomposition.md:211-218`; `references/evaluation/README.md:1-13` | The verdict cannot be quietly improved after publication has read it. |
| The runbook's own framing of the proof artefact — *"a written answer to whether phase 6's freeze needed amending. If it did, that is a superseding ADR and a data point that the two-implementation freeze rule should have been three"* | `RUNBOOK.md:4427-4430` | The "did not hold" branch already has its shape: a superseding ADR plus a meta-finding about the freeze rule itself. |
| `depends_on: read-your-own-writes-projection`, `cold-build-cost-and-ci-shape`, `package-completeness-and-name-claim` | `_storymap.md:48`, `:83-84` | RYOW supplies AC-005's answer and the rules-run list; the build story supplies the measured number and the CI shape; packaging supplies the crate state and the held name. The verdict must name the commit it was run at, so all three merge first. |

## Questions

**What does a "did not hold" verdict trigger? — answered, and this is the
question the story exists to have already answered before the result is known.** A
new decision atom and a re-plan (`project.md:109-111`), plus — per the runbook —
a superseding ADR and a recorded data point that the two-implementation freeze
rule should have been three (`RUNBOOK.md:4427-4430`). What it does **not** trigger
is an edit: no `[FROZEN]` clause marker or clause sentence in
`spec/SPECIFICATION.md` moves in this project, `cargo xtask spec-trace` stays
green, and the check is a reviewed `git diff` against the project-start commit
(`_decomposition.md:478`). An accepted decision atom is likewise superseded, never
edited, and `redkiln validate --kb` enforces that against `HEAD`.

**Does the verdict move PS-34's or PS-3's marker? — answered: neither.** PS-34 is
`projection-store-freeze`'s (`RUNBOOK.md:602`, "6, re-tested 11") and PS-3 is
`publication-and-positioning`'s (`RUNBOOK.md:601`, "6, decided at 12"). The
verdict supplies both with evidence and names the owner beside each finding, so a
reader can tell a data point from a decision.

**Is `WriteTransactionInUse` under a concurrency rule a capability limit or a rule
defect? — genuinely open, and it is this story's to answer.** LadybugDB permits
many readers and exactly one writer, so two commits racing is routine rather than
exceptional. The two candidate answers have different consequences: a stated
capability limit means the suite is right and the adapter's concurrency envelope
is narrower than SQL adapters'; a rule defect means the rule assumed a
concurrency model the port never promised, and it goes to HS-P0010. What is fixed
in advance is that the answer is **not** "retry until green", because a retry loop
converts a design finding into a flake.

**How does `lbug`'s blocking API meet a non-blocking port (ADR-0025)? — answered
in HS-S0075 and *reported on* here.** The verdict records whether the chosen
bridge survived contact with the suite, and in particular whether any conformance
emitter (tokio, blocking, wasm) could not run the adapter as built. A bridge that
narrowed the harness is a finding about the port's async posture, not a footnote
about this adapter.

**What counts as "structurally unlike"? — fixed on disk before any of this, and
the verdict is judged against those axes and nothing else.** The verdict cites the
axes document's commit SHA alongside its own, which is what makes AC-001's
ordering claim visible in the artefact rather than only in `git log`. If the run
showed an axis was wrong, the axes document is **superseded** under M9 and the
verdict says so; it is never edited to match what happened.

**Deferred to `spec`:** the verdict document's filename and whether the
build-cost measurement is a section of it or a separate evidence file, and
whether the T1 retirement finding is stated in the verdict alone or also raised as
its own note to HS-P0010.

## Decision

Everything this project produced — a real adapter, a suite run, an answered
read-your-own-writes question, a measured build, a set of findings no test emits —
is only worth what someone can read off it later, and the specific danger at this
point is that a green run reads as an answer when the question was whether the
port had been frozen against a monoculture. This story writes the verdict: dated,
pinned to a commit, naming the implementation, the rules run with one row per
registered rule, the nine PS clause ids in scope, and the word *held* or *did not
hold* — produced whichever way the run went, and carrying the findings a green
suite cannot carry, namely T1's retirement of the counter-example the freeze
deleted, the GAT/ICE constraint if the lifetime survived, the
`WriteTransactionInUse` question answered rather than deferred, and the
read-your-own-writes result in PS-12's terms. The spec will cover the document's
required fields, its rule table with *declined-with-reason* as an outcome distinct
from *passed* and a row count reconciled against `projection-store-freeze`'s
registration list, its citation of the axes commit alongside its own, and the
"did not hold" routing. That routing is explicit and it is not an edit: a verdict
of *did not hold* produces a new decision atom and a re-plan — and, per the
runbook, a superseding ADR and a recorded finding that the two-implementation
freeze rule should have been three — while every `[FROZEN]` clause marker and
sentence in `spec/SPECIFICATION.md` stays exactly as it was at this project's
start, with `cargo xtask spec-trace` green and a reviewed diff as the proof.

## The wrong implementation

**The verdict written only in the case where the freeze held.** The literal form
is rare and easy to spot; the forms that pass every check are not. A verdict
template committed with a "Held" section filled in and a "Did not hold" heading
left empty satisfies "the document exists". A verdict drafted *after* the run and
shaped by it — dated correctly, commit-pinned, listing rules, reading as a
finding — satisfies every field AC-007 enumerates. Both are the same failure:
the artefact's existence became contingent on the result, which is exactly the
asymmetry `_storymap.md:26-30` says makes it evidence. The structural guard is
that the verdict's skeleton — fields, rule table, clause list — is committed with
the axes at HS-S0074 or at latest before the run, and only its *values* are
written afterwards, so the shape of the document cannot be chosen to fit the news.

**The verdict that lists what passed and not what declined.** The most likely
mutant, and the most damaging. "Held. The projection suite ran green against
`LadybugProjectionStore` at `abc1234` on 2026-08-12; PS-4 – PS-6, PS-9, PS-11,
PS-12, PS-15 and PS-34 are confirmed." Every field DR-4 names is present, the
commit is real, the date is real, the rules did pass — and the six rules that
*declined* are absent, including the concurrency-shaped one that declined because
LadybugDB has exactly one writer. A freeze does not fail loudly; it fails as a
rule reported as a fixture limitation. The guard is structural rather than
diligent: the verdict carries **one row per registered rule**, with an outcome
column in which *declined, with the fixture's stated reason* is a distinct value
from *passed*, and the row count equals `projection-store-freeze`'s registration
count — the same reconciliation AC-004 already requires
(`_decomposition.md:474`). That makes the omission impossible to make rather than
easy to notice.

**The verdict that treats a green suite as the whole answer.** "Held — suite
green, all rules pass." It is true, it is checkable, it cites a run, and it drops
every finding the suite is structurally incapable of producing: that the freeze
**deleted the counter-example that was evidence for freezing** (T1); that if the
GAT survived, the port is implementable only by `'static` stores and the failure
mode for anything else is a compiler panic rather than a diagnostic; that
`WriteTransactionInUse` met a rule and something had to be decided about it; and
what the read-your-own-writes case actually did, in PS-12's terms rather than as a
capability note. DR-4's *"a verdict that says only 'held' is not a verdict"* names
this exactly. The guard is AC-A04: T1, T2 and T3 are honoured as decided **or the
deviation is written here**, so each of the three has a named home in the document
whether or not it produced news.

**The verdict improved after the fact.** The document lands, `publication-and-positioning`
reads it, and a later commit sharpens a sentence, softens a declension, or adds
the rule table that was missing. Nothing in the gate objects — it is a markdown
file — and the artefact that a downstream decision was taken against no longer
exists. `references/evaluation/README.md:1-13` is the standing rule and it is not
advisory here: this document is superseded, never edited, and the supersession
carries its own date and commit.

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
