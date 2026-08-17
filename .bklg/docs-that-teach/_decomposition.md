---
item: HS-I0007
stage: decomposition
created: 2026-08-17T03:02:21.696Z
updated: 2026-08-17T03:02:21.696Z
template_sig: 0ddf7230
rendered_sig: 3c06be17
---

# Decomposition — From Accurate to Teachable

Six projects, approved at the decomposition gate on 2026-08-17. This file is the
durable record of that gate: what the projects are, why the cut falls where it
does, which requirement each one owns, and the order they merge in.

## Children

| Child | Slug | Rationale |
| ----- | ---- | --------- |
| HS-P0020 | `checked-documentation-surface` | The substrate every other project consumes. Pins the narrative tree by path in `xtask/src/`, wires its build and render into `cargo xtask ci` as an ordinary step, compiles every fence against the real crates, and proves the check by watching it fail. Carries BR-10's clause-id pin because it is first in merge order and depends on nothing, which is what makes the pin structurally precede every doc-comment rewrite. |
| HS-P0021 | `page-need-discipline` | Makes "which need does this page answer" a labelled, reviewer-checkable property, and writes the discipline down where it durably binds. Separated from the surface because one owns the rule and the other owns the machine. |
| HS-P0022 | `application-author-path` | The persona-1 vertical slice: settle which prior mental model the teaching argues against, build the bridge from it, and stage an opening encounter in which a consistency boundary actually refuses an append. |
| HS-P0023 | `reach-and-adapter-path` | Reachability and the persona-2 slice together, because both are pointer problems governed by one policy (DT-10): the front door into the narrative material, the evaluator's second question, and the adapter author's error meeting its explanation at the site where it fires. |
| HS-P0024 | `comprehension-evidence` | The non-substitutable second instrument. A genuine non-author, non-insider reader walks the assembled material; what stopped them is recorded in an auditable shape and every stumble is dispositioned. |
| HS-P0025 | `durable-audience-closeout` | Terminal DoD owner. Reconciles the audience with the separately staged persona set, promotes it into `.kb/product/` through the ingest path, and re-observes all fifteen Definition-of-Done scenarios on the assembled tree from a clean checkout. |

## Why this cut, and not the charter's

The charter's `## Open design tensions` table names six owning projects
descriptively — Conceptual bridge, Page-need discipline, First encounter, Checked
prose, Comprehension evidence, Reach and placement — and says `/redkiln:plan`
assigns the real ids. That cut was adopted as the starting point and then changed
in three ways, each deliberate.

**Two responsibilities the charter's six did not cover were given owners.**
AC-05 ("the adapter author can follow the reasoning, not just the recipe") traced
to no project in the charter's own naming, and no BR covers it — BR-15 covers only
the pointer. BR-13, BR-17, AC-14 and DoD-15 (persona reconciliation and promotion
into the durable product layer) likewise had no home, and could not become a story
inside any of the six without one of them acquiring a second, unrelated job — the
exact defect BR-04 exists to make visible.

**The four content projects were collapsed into two.** The eight-project shape
that resulted from the first pass had a dependency graph that was a total order:
every project depended on all of its predecessors, which is the signature of
over-fragmentation rather than of real sequencing. `conceptual-bridge` and
`first-encounter` were merged into `application-author-path`, and
`reach-and-placement` and `adapter-path` into `reach-and-adapter-path`. Both merges
join work that one person does in one sitting against one persona, and both restore
genuine parallelism to the graph.

`application-author-path` therefore carries four tensions (DT-1, DT-4, DT-5, DT-6).
That is more than any other project and it is the right call anyway, because they
are one coupled decision rather than four independent ones: DT-5 and DT-6 are both
"how is the aggregate-to-boundary shift drawn, and how honest is the old side", the
interaction dossier states they are coupled, and DT-1's anchor model must be settled
*before* DT-4 stages anything against it. Splitting them across two design sign-offs
would put half a decision in each.

**BR-10 was placed inside `checked-documentation-surface` rather than given a
project of its own.** BR-10 is a sequencing constraint whose only real requirement
is that it precede every `happenstance-core` doc-comment rewrite. Putting it in the
project that is first in merge order and depends on nothing satisfies that
structurally, and it is the same subject matter — what the repository mechanically
knows and asserts about its own documentation. A standalone "pin the clause ids"
project would be a task wearing a project's clothes, and would still have to be
sequenced first, buying nothing. `reach-and-adapter-path` is the project that
rewrites `happenstance-core` doc comments (`store.rs`), and it sits behind the pin
in the DAG, so the constraint is visible in the graph rather than implicit in a note.

**Hosting shape** — docs.rs-only versus a separate rendered narrative surface — is
owned by `checked-documentation-surface`, because it is inseparable from which tree
is pinned by path and built by the gate. `reach-and-adapter-path`'s DT-10 owns only
*pointing at* whatever that resolves to.

## Decisions taken at the gate

Four questions were put to the approver; two more were resolved from the charter
rather than asked. All six are recorded here because each one changes the work.

**The sibling branch, and why this initiative does not block on it.**
`initiative/from-contract-to-published-library` (HS-I0006) is unmerged and actively
being implemented: 210 commits, 78 files, roughly 24,000 insertions. The
`Tags::empty()` defect that is this initiative's central piece of evidence
(`crates/happenstance/src/lib.rs:38,55`) exists *only* on that branch — this branch's
copy of that file is a different, shorter version. `spec/SPECIFICATION.md` diverges
there by 521 lines.

The decision is to **run in parallel and merge forward before the pull request**.
The clause-id concern that argued for blocking does not survive contact with the
specification's own text: *"Clause IDs are stable and are never renumbered"*
(`spec/SPECIFICATION.md:280`). Ids are stable names (`VT-n`, `ES-n`, `CF-n`), not
line references, so a 521-line divergence cannot invalidate a pinned set. The
residual risk is narrower — the sibling may *add* documentation MUSTs, leaving the
pinned set incomplete — and DoD-11 already re-runs the specification cross-reference
at closeout, which `durable-audience-closeout` owns.

One operational rule falls out and must not be lost: **`application-author-path`
merges forward before it implements.** `crates/happenstance/src/lib.rs` is the one
file where the sibling has not merely diverged but replaced the thing this
initiative is fixing (75 lines here, 237 there, with the defect intact in their
version). Authoring the new opening encounter against this branch's stale copy would
mean redoing the work at merge rather than merging it.

**Where the discipline lives (BR-12).** It lands immediately in a new tree pinned by
path in `xtask/src/`, sibling to `standards/rust/` and inside its precedence chain
without extending it, with a `.kb/playbooks/` atom staged through ingest at closeout.
This is the only option in which the rule binds while the projects that must obey it
are being built: `.kb/` atoms may only be authored through the ingest path (the first
attempt at hand-authoring was reverted at `0269720`), and ingest runs at closeout,
but `page-need-discipline` is second in merge order and its rules must bind from that
point on. `docs/README.md:25-29` and `xtask/src/lint_constitution.rs` are the
precedent: a tree the gate reads is pinned by path, and moving it means editing
`xtask/src/` in the same change.

**Persona reconciliation (BR-13).** Run in parallel and reconcile at closeout.
HS-S0131 (`persona-and-journey-intake-staging`, HS-P0019/HS-I0006) was verified
directly and is still `stage: plan`, `status: ready` — unrun, on the unmerged branch.
`.kb/product/` and `.kb/design/` hold READMEs only. There is nothing to consume
today. `durable-audience-closeout` promotes against whatever tree exists when it
runs, stating supersession explicitly either way. Blocking would put an
unschedulable cross-branch dependency on this initiative's terminal project;
superseding is the charter's explicitly named failure mode.

**AC-05's scope — resolved from the charter, not asked.** The adapter author's
narrative account surfaces and sequences the existing `MemoryEventStore`
walk-through with connective tissue, rather than authoring a new chapter-length
account. The charter's non-goal is explicit that volume is not the measure and that
this is a findability-and-sequencing problem; the walk-through is named as one of
the three strongest explanations already written. If the friction log shows the
surfaced version does not carry an adapter author, a new account is owed and is
dispositioned through `comprehension-evidence`.

**The evaluator's status — resolved from the charter, not asked.** Planned against
three personas, because `## Who this is for` names three and `reach-and-adapter-path`
needs its own DoD scenarios (7 and 9) to be discharged by someone. Whether the
evaluator is a persona in its own right or an early stage of the application
author's journey is adjudicated once, at promotion, by `durable-audience-closeout`
— jointly with HS-S0131's set rather than twice.

## Design tension ownership

All ten tensions from the charter's table, each owned exactly once. An unowned
tension is how a documented failure mode ships; `design.capture` is deliberately
absent from `.redkiln/config.yaml`, so the perceptual review is a skip and each
project's written `_design.md` is the *only* record of its resolutions.

| DT | Tension | Owning project |
| --- | --- | --- |
| DT-1 | Which prior mental model the teaching anchors against | HS-P0022 `application-author-path` |
| DT-2 | Named external taxonomy vs. the one-need-per-page rule only | HS-P0021 `page-need-discipline` |
| DT-3 | Where findability and navigation live in that discipline | HS-P0021 `page-need-discipline` |
| DT-4 | How the opening encounter discloses complexity | HS-P0022 `application-author-path` |
| DT-5 | Whether the aggregate-to-boundary shift is taught with a diagram | HS-P0022 `application-author-path` |
| DT-6 | Whether a "wrong model" contrast is shown as real compiled code | HS-P0022 `application-author-path` |
| DT-7 | Whether adapter- or feature-scoped content uses hidden panels | HS-P0020 `checked-documentation-surface` |
| DT-8 | Where the safe-aside / load-bearing line falls | HS-P0021 `page-need-discipline` |
| DT-9 | Which persona the comprehension session walks first | HS-P0024 `comprehension-evidence` |
| DT-10 | Whether the reference surface points outward once, repeatedly, or contextually | HS-P0023 `reach-and-adapter-path` |

`comprehension-evidence` owns one tension and `durable-audience-closeout` owns none.
That is deliberate: they carry owned requirements rather than unresolved interaction
choices, which is exactly why the charter's tension table did not name the latter.

## Traceability matrix

Every initiative business requirement and every acceptance criterion maps to at
least one project. A requirement with no project is a gap; a project tracing to
nothing is scope creep.

### Business requirements

| ID | Requirement (abbreviated) | Owning project(s) |
| --- | --- | --- |
| BR-01 | Narrative material compiled against the real crates by the gate | HS-P0020 |
| BR-02 | The gate obligation observed *failing* on a deliberately broken page | HS-P0020 |
| BR-03 | The opening encounter demonstrates a boundary doing its job | HS-P0022 |
| BR-04 | Every page carries a named, checkable answered-need | HS-P0021 |
| BR-05 | A dated friction log from a non-author, non-insider reader | HS-P0024 |
| BR-06 | The log follows an auditable shape and is routed to someone who can act | HS-P0024 |
| BR-07 | The teaching states which prior mental model it argues against | HS-P0022 |
| BR-08 | The result is reachable where a Rust developer already looks | HS-P0023 |
| BR-09 | The specification wins; a page cites clauses and never restates them | HS-P0021 (the authoring rule and reviewer spot check) + HS-P0020 (the mechanical citation-resolution check) |
| BR-10 | The frozen documentation MUSTs pinned by clause id before any rewrite | HS-P0020 |
| BR-11 | Hidden content proven to sit inside the same checked surface | HS-P0020 (the demonstration) + HS-P0021 (the rule for what may never be folded, DT-8) |
| BR-12 | Where the discipline lands is decided; a gate-read tree is pinned by path | HS-P0021 (where it lands) + HS-P0020 (pinning by path) |
| BR-13 | The audience model reconciled with the staged persona work | HS-P0025 |
| BR-14 | The comprehension claim scoped to what the method supports | HS-P0024 |
| BR-15 | The adapter author meets the trait-resolution explanation where it fires | HS-P0023 |
| BR-16 | The evaluator's second question has somewhere to go | HS-P0023 |
| BR-17 | Personas promoted into the durable product layer at closeout | HS-P0025 |
| BR-18 | Whether the shift is taught with a picture is decided | HS-P0022 |

The three split rows are split by **responsibility, not shared ownership**: in each
case one project owns the rule and the other owns the machine, and the seam is
stated in both projects' non-goals. No responsibility is owned twice.

### Acceptance criteria

| ID | Criterion (abbreviated) | Owning project(s) |
| --- | --- | --- |
| AC-01 | The first program teaches what the library is for | HS-P0022 |
| AC-02 | The application author can carry their own invariant across | HS-P0022 |
| AC-03 | The reader is not taught something the library no longer does | HS-P0020 |
| AC-04 | The adapter author meets the explanation where the problem finds them | HS-P0023 |
| AC-05 | The adapter author can follow the reasoning, not just the recipe | HS-P0023 |
| AC-06 | The evaluator's second question has somewhere to go | HS-P0023 |
| AC-07 | Any reader can tell what a page is for before reading it | HS-P0021 |
| AC-08 | A reader arriving with the wrong prior model is met | HS-P0022 |
| AC-09 | The team can see where teaching failed a real person | HS-P0024 |
| AC-10 | Someone acted on what that reader found | HS-P0024 |
| AC-11 | A reader finds the teaching from where they already are | HS-P0023 |
| AC-12 | The specification stays the single normative voice | HS-P0021 (the page defers to the clause) + HS-P0020 (the citation resolves) |
| AC-13 | The next author is not starting from scratch | HS-P0021 |
| AC-14 | The next initiative inherits this one's audience | HS-P0025 |

### Definition of Done

| # | Scenario (abbreviated) | Owning project |
| --- | --- | --- |
| 1 | Clean checkout; gate green; narrative builds and renders as part of it | HS-P0020 |
| 2 | A deliberately broken page fails the gate by name, then recovers | HS-P0020 |
| 3 | The opening encounter runs and demonstrates a boundary | HS-P0022 |
| 4 | The boundary claim is checked, not narrated | HS-P0022 |
| 5 | A non-author, non-insider completes a stated scenario; auditable log | HS-P0024 |
| 6 | Every stumble in that log has a disposition | HS-P0024 |
| 7 | The reader reaches the teaching from the front door | HS-P0023 |
| 8 | Every page's answered need is stated and singular | HS-P0021 |
| 9 | The evaluator's second question is walked | HS-P0023 |
| 10 | The adapter author's error meets its explanation | HS-P0023 |
| 11 | The frozen documentation MUSTs are still discharged | HS-P0020 |
| 12 | No page has become a second specification | HS-P0021 |
| 13 | Nothing load-bearing is hidden from the check | HS-P0020 |
| 14 | The discipline is on disk and cited | HS-P0021 |
| 15 | The audience is durable and reconciled | HS-P0025 |

All fifteen are additionally **re-observed by HS-P0025** as terminal DoD owner, on
the assembled result from a clean checkout. That is a verification obligation, not a
second ownership: the project named above is the one that makes the scenario true.

### MECE confirmation

- **No uncovered ids.** All eighteen business requirements, all fourteen acceptance
  criteria, all fifteen Definition-of-Done scenarios and all ten design tensions
  carry a named owner. There are no gaps.
- **No project without coverage.** HS-P0020 owns 6 BR / 2 AC / 4 DoD / 1 DT;
  HS-P0021 4 BR / 3 AC / 3 DoD / 3 DT; HS-P0022 3 BR / 3 AC / 2 DoD / 4 DT;
  HS-P0023 3 BR / 4 AC / 3 DoD / 1 DT; HS-P0024 3 BR / 2 AC / 2 DoD / 1 DT;
  HS-P0025 2 BR / 1 AC / 1 DoD plus the terminal Definition of Done.
- **Three requirements are split by responsibility** (BR-09, BR-11, BR-12), each
  with the seam stated in both owners' non-goals. No other id has two owners.

## Sequencing

### Dependency graph

| Project | Depends on | Unlocks |
| --- | --- | --- |
| HS-P0020 `checked-documentation-surface` | — | HS-P0021, HS-P0022, HS-P0023 |
| HS-P0021 `page-need-discipline` | HS-P0020 | HS-P0022, HS-P0023 |
| HS-P0022 `application-author-path` | HS-P0020, HS-P0021 | HS-P0023, HS-P0024 |
| HS-P0023 `reach-and-adapter-path` | HS-P0020, HS-P0021, HS-P0022 | HS-P0024 |
| HS-P0024 `comprehension-evidence` | HS-P0022, HS-P0023 | HS-P0025 |
| HS-P0025 `durable-audience-closeout` | HS-P0024 | — |

Acyclic. Every edge points strictly forward in the merge order below, so the graph
is a subset of that total order's edges and admits no cycle.

### Why the load-bearing edges exist

- **HS-P0020 first, depending on nothing.** It is the substrate — the pinned tree
  and the gate step — that every page consumes, and it carries BR-10's clause-id
  pin, which puts every project that rewrites a `happenstance-core` doc comment
  transitively behind the pin.
- **HS-P0021 before the content projects.** The discipline binds the pages; a page
  authored before the rule exists is a page the rule is then retro-fitted to, which
  is how the rule becomes decorative.
- **HS-P0022 before HS-P0023.** DoD-9 walks the evaluator's second question *to
  pages that answer it*, and AC-11 needs narrative material to reach. Reversing this
  edge is the only cycle available in the graph.
- **HS-P0024 after all content.** A friction log run against a half-assembled
  surface measures the assembly, not the teaching.
- **HS-P0025 last.** BR-17 says promotion happens at closeout, and the promotion must
  record precisely which persona the friction log *did* directly observe rather than
  shipping the blanket "none has been directly observed" qualification unchanged.

### Merge order

1. HS-P0020 `checked-documentation-surface`
2. HS-P0021 `page-need-discipline`
3. HS-P0022 `application-author-path`
4. HS-P0023 `reach-and-adapter-path`
5. HS-P0024 `comprehension-evidence`
6. HS-P0025 `durable-audience-closeout` — **terminal, DoD owner**

### Available parallelism

The merge order is a linearization, not a schedule. HS-P0021's design stage runs
alongside HS-P0020's implementation; HS-P0023's DT-10 pointer policy can be resolved
in design while HS-P0022 is still authoring, and only its *implementation* waits on
HS-P0022's pages existing. Within HS-P0023, the adapter half (BR-15, AC-05 — the
`store.rs` error site and the surfaced reasoning account) does not depend on
HS-P0022's content at all and can proceed as soon as the pointer policy is settled;
that ordering is the story map's to express.

HS-P0025 is the terminal DoD owner and the only project created with `--terminal`.
Every other project is `--no-terminal`, so each is held to a project-scoped
integration bar rather than to a whole-initiative end-to-end it could not pass.

## Warranted briefs per project

A charter is always produced; the tags below are the briefs each project actually
warrants. No stubs.

| Project | architecture | ux | testing | deployment |
| --- | --- | --- | --- | --- |
| HS-P0020 `checked-documentation-surface` | yes | yes | yes | yes |
| HS-P0021 `page-need-discipline` | yes | yes | yes | — |
| HS-P0022 `application-author-path` | — | yes | yes | — |
| HS-P0023 `reach-and-adapter-path` | yes | yes | — | — |
| HS-P0024 `comprehension-evidence` | — | yes | yes | — |
| HS-P0025 `durable-audience-closeout` | yes | yes | yes | — |

HS-P0020 earns all four: `architecture` for the pinned tree and gate wiring, `ux`
for DT-7's hidden-panel decision, `testing` for the two falsifications, and
`deployment` for where the rendered surface actually lives and how it is published.
It is the only project that earns `deployment`. HS-P0022's `testing` tag is earned
by DoD-4 (removing the boundary must make the example fail) and, if DT-6 resolves to
a real compiled wrong side, by a `publish = false` artifact the gate must check
forever. HS-P0023 takes no `testing` brief because DoD-7, 9 and 10 are all observed
walks rather than automated checks.

## Story map

Not used on a standard project. Each project's story map is its own `_storymap.md`,
authored by the briefs workflow and approved at the story-map review gate.
