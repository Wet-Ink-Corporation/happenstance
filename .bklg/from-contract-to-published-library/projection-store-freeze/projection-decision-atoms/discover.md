---
item: HS-S0002
stage: discover
created: 2026-08-12T13:01:10.374Z
updated: 2026-08-12T13:01:10.374Z
template_sig: 86ce4036
rendered_sig: 616e182b
---

# Discover — ADR-0017, ADR-0018 and ADR-0019 accepted before the port changes

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: write and accept ADR-0017 (batch ownership and write vocabulary, PS-4 – PS-15), ADR-0018 (reset and checkpoint scope, PS-16 – PS-20) and ADR-0019 (apply-failure policy, PS-26 – PS-30) as `.kb/decisions/` atoms, each quoting a compiler transcript, each naming `LiveHandleProjectionStore`'s disposition, open questions resolved rather than deleted, `redkiln validate --kb` green | `../_storymap.md:54` | Three atoms, three clause ranges, and four named obligations per atom. The clause ranges are the scoping input, and they may widen — see the dependency row below |
| **AC-008** — the three ADRs accepted **before** the port change lands, each quoting a transcript from `references/adapter-shapes.md`, each naming the alternatives that lost, open-question atoms marked resolved not deleted | `../project.md:204-208` | "Before" is load-bearing and is checked by commit order, not by content. The storymap says so: "the ordering against `owned-batch-port-shape` *is* the check" (`../_storymap.md:86`) |
| `dependsOn: ps-clause-pairing-sweep` — supplies the finding that decides how wide the frozen-clause repair has to be, and whether the answer is a re-plan rather than an ADR | `../_storymap.md:53,107` | This story cannot start with its scope known. If the sweep returns "systematic", the ADR scope written here is wrong and the escalation is a re-plan (`../_storymap.md:126`) |
| Three ADRs do not exist yet: `.kb/decisions/` holds 0001–0016 plus 0029, and no 0017/0018/0019 | `.kb/decisions/` (verified listing); `../_grounding.md:179-185` | AC-008 asks this project to **write** three ADRs, not to cite existing ones. The intake brief's "Settled by ADR-0017/18/19" names decisions that do not exist until this story produces them |
| The transcripts the ADRs must quote are already captured: two independent `SendProjectionStore` rejections of `type Batch<'a> = rusqlite::Transaction<'a>`, the `error[E0195]` trap, and the minimised `DefId::expect_local` ICE | `references/adapter-shapes.md:61,186-191,311-347`; `../_grounding.md:70-77` | Nothing needs re-deriving. The ADR pass is quotation and adjudication, not fresh experiment, and `RUNBOOK.md:3952-3953` requires the quotation rather than an assertion that the port "survives" |
| `LiveHandleProjectionStore` binds `type Batch<'a> = GraphWriteHandle<'a>` with **real bodies**, exists to be the counter-example to §4.2's argument, and cannot survive `type Batch;` as written | `crates/happenstance-ladybug/src/live_handle.rs:16-31,154-219`; `../_decomposition.md:606-627` | Its disposition — deleted, moved to `experiments/`, or kept with a note — is a named decision. "Deleting the only compiled evidence against the decision you are making is how a port gets frozen against its own hypothesis" |
| The clause PS-5 should rest on is the *other* argument, not the `Send` one: today's port is implementable only by stores that outlive every batch lifetime, and a non-`'static` store ICEs rustc in the error-reporting path | `../_decomposition.md:617-623`; `crates/happenstance-ladybug/src/live_handle.rs:38-66` | ADR-0017's reasoning has a stronger and a weaker form already identified. The weaker one ("the `Send` objections are properties of rusqlite") is the one the instrument refutes |
| ADR-0008's finding that a provided body cannot hold the `Batch` GAT across a suspension point is an input ADR-0017 must weigh, not just the borrowed-versus-owned question | `.kb/decisions/0008-one-derivation-for-both-ports.md`; `.kb/open-questions/projection-store-batch-has-no-apply-seam.md`; `../_grounding.md:263-267` | The apply-seam atom names ADR-0017 by number as its own resolution, so that atom is one of the four this story must resolve rather than delete |
| Testing brief tiers AC-008 as **Static**: `redkiln validate --kb` run before the port-shape commit, plus `git diff --diff-filter=D` over `.kb/open-questions/` for the atoms this project touches, which must be empty | `../_decomposition.md:778` | The delete-versus-resolve distinction has an executable check. The ordering does not — it is a process gate on commit sequence |
| An accepted decision atom is immutable; correcting one means a new atom that supersedes it | `CLAUDE.md`, *Where the work lives*; `redkiln validate --kb` | Getting the clause ranges wrong is expensive after acceptance. This is the second reason the sweep runs first |
| Architecture brief AC-A08: nothing `[FROZEN]` is line-edited; PS-1 / PS-19 scope gaps and any sibling the sweep finds are carried as **ADR scope** | `../_decomposition.md:326-330` | The frozen-clause repair is this story's payload, and it is the reason the story sits in slice 1 rather than beside the rule work |

## Questions

Open questions to resolve before specifying.

1. **Does the sweep's result change the three-ADR split?** Deferred to `spec`,
   and it must be — the sweep runs first precisely so this is answerable rather
   than assumed. If the finding is "isolated", the repair is a clause of its own
   inside ADR-0017 and ADR-0018. If it is "systematic", the split is wrong and
   the escalation is a re-plan, reported at the story boundary
   (`../_storymap.md:126`).
2. **Which atom carries the frozen-clause repair — a fourth, or a clause inside
   the three?** Deferred to `spec`. Both are legal under the
   repair-frozen-clause discipline the two open-question atoms cite; the choice
   turns on question 1 and should be made once, in writing.
3. **What is `LiveHandleProjectionStore`'s disposition?** Deferred to `spec`,
   with the constraint answered here: **not silent deletion**
   (`../_decomposition.md:314-316,624-627`). Whichever of the three routes is
   taken, ADR-0017 names it and says what evidence survives the move.
4. **Is `ProjectionId::new`'s infallibility in scope?** Answered: no. Architecture
   brief AC-A09 and the constructor's own doc comment both say it is an open
   question owned elsewhere (`crates/happenstance-core/src/projection.rs:44-64`;
   `.kb/open-questions/projection-id-is-unvalidated.md`). It must not be
   hardened as a side effect of writing three ADRs about the module it lives in.
5. **Does a boundary-scoped projection checkpoint come up?** If it does, stop:
   it reopens ADR-0013's globally frozen visibility invariant and is a separate
   decision (`.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`;
   `../project.md:326-329`). Flagged, not settled.

## Decision

The port's shape is about to change in five places at once, and the reasoning for
each change currently lives in compiler transcripts, a RUNBOOK phase and four
open-question atoms rather than in anything a future reader can cite. This slice
converts that into three accepted decision atoms — ADR-0017 (what a projection
batch owns and what vocabulary writes into it, PS-4 – PS-15), ADR-0018 (how a
projection is returned to "never run" and what may refuse it, PS-16 – PS-20) and
ADR-0019 (what happens when `apply` fails, PS-26 – PS-30) — written *before*
`owned-batch-port-shape` touches a line of `projection.rs`, because ordering is
what AC-008 actually checks. The spec will fix: the three atoms' clause ranges
and where the frozen-clause repair lands (contingent on the sweep's finding);
which transcript each atom quotes, by `file:line`, from
`references/adapter-shapes.md`; the alternatives each atom records as losing —
including the `Send`-based argument for PS-5 that `LiveHandleProjectionStore`
refutes, and the stronger non-`'static`-store argument that survives;
`LiveHandleProjectionStore`'s stated disposition; the four open-question atoms
to be marked **resolved** with a pointer to the atom that resolves them; and the
commit sequence, which must place `redkiln validate --kb` green on the three new
atoms strictly before the port-shape commit. Two `[FROZEN]` clauses are repaired
here by new atoms and never by edit — PS-1 (`spec/SPECIFICATION.md:4733-4759`)
and PS-19 (`:5218-5249`) — plus any sibling the sweep found.

## The wrong implementation

**Three atoms that pass `redkiln validate --kb` and assert the port survives.**
Frontmatter conformant, `kind: decision`, `status: accepted`, supersession graph
clean, links reciprocal — every check this repository runs against the knowledge
base is green, and not one of them reads the body. The atoms say the owned batch
"was validated against the skeletons" instead of quoting
`references/adapter-shapes.md:61`'s two independent `SendProjectionStore`
rejections, and `RUNBOOK.md:3952-3953` asked for exactly the opposite. This is the
default failure mode for an ADR pass and nothing mechanical convicts it; the spec
must require a quoted transcript with a `file:line` per atom, so the review has
something to check rather than an impression to form.

**The one that is actively destructive:** `git rm`-ing
`.kb/open-questions/projection-store-batch-has-no-apply-seam.md`,
`ps-1-states-no-progress-obligation.md` and `ps-19-scope-narrower-than-its-rule.md`
once the ADRs answer them. `redkiln validate --kb` goes green — greener, in fact,
since an unresolved question is one fewer — `redkiln doctor` is quiet, and the
audit trail linking the defect to its repair is gone. The check that catches it
is already named by the testing brief: `git diff --diff-filter=D` over
`.kb/open-questions/` must be empty (`../_decomposition.md:778`), mirroring the
closeout project's own AC-006 check. DR-06 states the rule in words —
**resolved rather than deleted** (`../project.md:161-163`).

**The one that costs the most later:** deleting
`crates/happenstance-ladybug/src/live_handle.rs` as part of "restating the
skeletons", because under `type Batch;` it no longer compiles. `cargo xtask ci`
goes green, AC-013's bar is arguably met, and the workspace has just discarded
the only compiled counter-example to the argument ADR-0017 is making. The module's
own doc says what it is for (`live_handle.rs:16-31`); the architecture brief calls
the deletion out by name (`../_decomposition.md:624-627`). If it goes, it goes to
`experiments/` with its transcripts and a named decision, and ADR-0017 records
that the port no longer admits it.

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

The two judgement boxes. **Literal positions**: this story adds no conformance
rule — it writes three knowledge-base atoms — so the box is vacuously true.
**`[FROZEN]` clauses**: this story is the one that legitimately touches them, and
it touches them the only permitted way. PS-1 and PS-19 (plus any sibling the
sweep found) are repaired by a **new accepted atom**, never by editing the clause
text, and the atom is accepted before `owned-batch-port-shape` lands
(`../_storymap.md:107-108`). The eighth box is vacuous at the rule grain: §4.11's
seventeen rules are not in the tree yet.
