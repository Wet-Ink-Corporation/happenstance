---
item: HS-S0189
stage: spec
created: 2026-08-17T13:16:36.424Z
updated: 2026-08-17T13:16:36.424Z
template_sig: 87bbf1d0
rendered_sig: 154650f0
---

# Spec — Every fence inventoried and every clause citation audited

## Scope lock

| | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` |
| Project | `.bklg/docs-that-teach/application-author-path/project.md` |
| This spec | `.bklg/docs-that-teach/application-author-path/fence-inventory-and-clause-audit/spec.md` |
| Key briefs | `.bklg/docs-that-teach/application-author-path/_decomposition.md` (UX brief `:11-446`, testing brief `:448-651`) |
| Signed-off design (binding) | `.bklg/docs-that-teach/application-author-path/_design.md` (approved `:1039`) |
| Grounding | `.bklg/docs-that-teach/application-author-path/_grounding.md` |
| Story map / merge order | `.bklg/docs-that-teach/application-author-path/_storymap.md` (this story `:61`; slice rationale `:79-84`) |
| Substrate owner (consumed, not built) | `.bklg/docs-that-teach/checked-documentation-surface/_design.md` (`## Signatures` `:83-110`) |
| Discipline owner (procedure borrowed, not invented) | `.bklg/docs-that-teach/page-need-discipline/project.md` (AC-010 `:236-238`) |
| Baseline this story reads anchors from | `.bklg/docs-that-teach/application-author-path/merge-forward-preflight/spec.md` (`§ Anchors`, `:299-305`) |
| Roadmap pointer | `RUNBOOK.md` — documentation work, no phase added; the roadmap is not amended here |

## One-line PR slice

Produce the inventory of every fenced block this project authored showing zero opted out —
or each exception named against `_design.md`'s stated exemption and present on HS-P0020's
enumerated allowance list — and audit every normative claim as a clause citation that
resolves and that no page restates in its own words.

## Executive summary

This PR lands **two audit artifacts and four falsification drills**, and no teaching content.

The pointer is project AC-007 (`project.md:254-256`) and AC-011 (`project.md:266-268`), and
the testing brief already says what shape they take: *"AC-007's inventory is a review
artifact, not a script. Nothing in this repository today enumerates 'every fenced block a
given project authored'"* (`_decomposition.md:638-643`).

The delta this spec adds is that an inventory nobody can fail is exactly the artifact this
initiative exists to refuse. CLAUDE.md's own discipline — *before adding a check, name a
plausible wrong implementation it rejects* — applies to a table as much as to a conformance
rule, and a table cannot fail. So the deliverable is not a count. It is:

1. **An enumeration that unions three exercise mechanisms**, because this project's pages do
   not all live in one. HS-P0020's checker sweeps `TREE = "docs"` and nothing else
   (`checked-documentation-surface/_design.md:93`); the crate-root fence lives in
   `crates/happenstance/src/lib.rs` and rides `cargo test --doc` under the `"tests"` REQUIRED
   step (`xtask/src/main.rs:143`); `examples/course-subscriptions/src/overview.md` is included
   into a `publish = false` crate and rides the same `--workspace` sweep
   (`_design.md:728-735`). A fence belonging to none of the three is exercised by nothing, and
   **no single command in this repository detects that today**.
2. **A per-fence column nobody has written before: can the mechanism that exercises this
   fence actually fail if the fence opts out?** For a page under `docs/` the answer is yes —
   the checker reports *opted out* by `path:line`
   (`checked-documentation-surface/_design.md:565`). For the crate-root fence the answer is
   **no**: an `ignore` there makes rustdoc skip it and the gate stays green. That fence is
   protected by `_design.md:880-882` and anti-pattern 1 (`_design.md:836-838`) — by review, not
   by the gate — and saying so in the inventory is the difference between an audit and a
   reassurance.
3. **Four drills that make both headline claims falsifiable**, in the shape of AC-005's
   DoD-4 drill but aimed at this story's own claims: opt one fence out and watch the gate name
   it; retag one cited clause id to one the specification does not define and watch the gate
   name it; and — the two that are expected to *fail to fail* — repeat both against the crate
   root, where no mechanism is in place to catch either, and route the gap.

Everything the drills prove, the artifacts record. Everything the artifacts claim, a named
command re-derives. Nothing here substitutes a hand count for a mechanism: where the
mechanism is missing, the inventory says so and routes it, because a hand count is precisely
the artifact whose silent decay this project exists to prevent.

## Context pack

**The design already spent this story's discretion, and it spent it on zero.** Project AC-007
permits exceptions — *"shows zero opted out, or names each exception against AC-003's stated
exemption"*. `_design.md` declined the second half: DT-6 resolved to real, compiled, executed
code, and *"this project ships zero uncompiled fences and puts nothing on HS-P0020's
enumerated allowance list"* (`_design.md:206-208`). The sign-off records the consequence the
approver accepted: that commitment *"removes a dependency; it also means that if a fence turns
out to be genuinely unwritable as compiled code, this design must be reopened rather than an
exemption written"* (`_design.md:1055-1058`). **So the expected result of this audit is
exactly zero, and an exception is not a row to justify — it is a stop.** This story may not
close a finding by writing an exemption, adding an `IGNORE_ALLOWANCES` entry, or relaxing a
fence to `ignore`. Discovering that a fence cannot be compiled reopens `_design.md`, which is
a human's gate, not this story's.

**The three mechanisms, and why the union is the whole job.** `_design.md`'s `## Surfaces`
block names four surfaces (`:45-65`) and `## Placement and re-export` puts them in three
different places (`:707-747`):

- `opening-encounter`, `conceptual-bridge`, `worked-example-handoff` land in HS-P0020's pinned
  tree, `TREE = "docs"` (`checked-documentation-surface/_design.md:93`), registered one
  `#[cfg(doctest)] mod` per page in the harness `xtask/src/narrative.rs`
  (`:96`) — one module per file, because concatenated includes report a failure at a line
  number that maps to no file a reader can open (`xtask/src/constitution.rs:11-18`).
- `crate-root-encounter` is `crates/happenstance/src/lib.rs`. It is deliberately **not** in the
  tree: `include_str!` resolves against the file tree at compile time and a path escaping the
  package would not resolve once published (`crates/happenstance/src/lib.rs:7-9`), which is
  why step 3's program exists twice on purpose (`_design.md:707-726`).
- `examples/course-subscriptions/src/overview.md` is a new file included by that example's
  crate root (`_design.md:728-735`).

Three homes, three exercisers, and the inventory is the only artifact in the repository that
sees all three at once. A page authored under `docs/` but absent from `xtask/src/narrative.rs`
is *unregistered* and the checker catches it
(`checked-documentation-surface/_design.md:564-566`). A fence added to `crates/happenstance/src/lib.rs`
with `no_run` is caught by nobody.

**Compiled is not executed, and this project has already been warned in writing.** The testing
brief names `no_run` as the concrete failure mode: it *"passes tier 2 (the compiled-fence step)
while silently never reaching tier 3 — the boundary would type-check forever without ever being
asked to refuse anything"* (`_decomposition.md:616-625`). `_design.md` forbids `no_run` and
`ignore` on the boundary-refusal fence specifically (`:880-882`). HS-P0020's planning corpus
never mentions `no_run` anywhere — verified by a repo-wide search over
`.bklg/docs-that-teach/checked-documentation-surface/`, which returns nothing. Its checker
enumerates *untagged fence*, *unrecognised info string* (exhaustively matched, so an unknown
part is a hard error), *opted out* and *stale allowance*
(`checked-documentation-surface/_design.md:562-566`); whether `no_run` lands under
"unrecognised" is a fact to establish by running the step, not to assume. **The inventory
therefore carries an execution class per fence, not a compiled/not-compiled bit**, and records
which of the two possible answers the checker actually gave.

**`cargo xtask spec-trace` is not what resolves a citation on a page, and the storymap's
one-liner is shorthand.** `spec_trace` checks `spec/SPECIFICATION.md`'s *internal*
traceability — every normative clause against the conformance rule that discharges it
(`xtask/src/spec_trace.rs:5-28`) — and never opens a narrative page. What resolves a clause id
*cited by a page* is HS-P0020's narrative checker calling the new accessor
`clause_ids(root) -> BTreeSet<String>` (`checked-documentation-surface/_design.md:88-90`),
whose failure line is ``docs/append-conditions.md:12 — cites `ES-99`, which SPECIFICATION.md
does not define`` (`:333`). Both run: `spec-trace` is a REQUIRED step
(`xtask/src/main.rs:315`) and is also wired unconditionally into the story grain
(`.redkiln/config.yaml:40`, `:48`). **The audit names the resolver per page rather than per
project**, because the crate root sits outside `TREE` and is therefore outside the only
mechanism that resolves a cited clause id at all — a relative markdown link to
`spec/SPECIFICATION.md` is not an intra-doc link and `broken_intra_doc_links` does not see it.

**Restatement is judged by a borrowed procedure, never a new one.** HS-P0020 routes *"whether a
page defers to a clause rather than restating it"* to HS-P0021 explicitly
(`checked-documentation-surface/project.md:128`), and HS-P0021's AC-010 owns the rule and the
spot check (`page-need-discipline/project.md:236-238`). This story applies **that** procedure to
this project's four surfaces and records the verdict. Inventing a second restatement test is
DR-14's defect wearing a different hat (`project.md:224-226`). The cheap mechanical first pass
is already written down as anti-pattern 11 — *"a sentence containing MUST or MUST NOT that is
not a link to a clause id"* (`_design.md:867-868`) — which is greppable; the spot check is the
second pass, because a clause can be restated without the word MUST.

**Maturity markers survive the citation, or the citation is wrong even though it resolves.**
`_design.md:400-404` binds this: ES-25 (`spec/SPECIFICATION.md`, `[FROZEN]`) and CF-7
(`[FROZEN]`) are cited as such, and VT-30 (`[PROVISIONAL]`) may not be cited in language that
implies its shape is frozen or that would need rewriting if VT-30 changes. Post-merge line
anchors come from the preflight's `§ Anchors` record — ES-25 `:3698`, VT-30 `:1816`, CF-7 in the
`:7266` region (`merge-forward-preflight/spec.md:261`) — and are read from there rather than
re-derived, because clause **ids** are stable and never renumbered
(`spec/SPECIFICATION.md:280`) while lines are not.

**The persona-journey slice.** Backbone activity A6 — *"I want to trust the whole set, not one
page"* (`_storymap.md:46`). Persona 1 never opens either artifact. What reaches them is that no
page they read is quietly unchecked, and no page has become a second, weaker specification
whose wording they might follow instead of the clause. This is the set-level answer to the
persona's stated fear, *"a mental model that looks right, compiles, runs, and is quietly
wrong"* (`_decomposition.md:28-30`).

**Why this story is last and set-wide.** AC-007 and AC-011 are properties of a *set*; assigning
them to any single page story leaves them unprovable until the set exists anyway
(`_storymap.md:79-84`). The slice splits on **mechanism, not subject**: this story is what the
gate steps and the allowance list can decide; `answered-need-and-anchor-review` (HS-S0190) is
what only a reviewer can (`_storymap.md:82-84`). Both are in the same slice and mount together.

**Substrate absent is a halt, not a hand count.** If HS-P0020's two steps —
`the narrative tree's examples compile` and `every narrative page is checked`
(`checked-documentation-surface/_design.md:327-331`) — are not in `REQUIRED` when this story
runs, both headline claims have no enforcer. The response is to record the gap and route it to
HS-P0020 per project DoD item 9 (`project.md:300-302`, `_storymap.md:158-163`), and to halt this
story's completion — **not** to substitute an unfalsifiable hand count, which is the weaker
criterion the project's own coupling note forbids (`project.md:345-350`).

**What this story may not do.** It may not author a page, amend a clause (the initiative is
additive and discharges none, `project.md:155-157`), edit `_design.md` (signed off,
`:1039`), re-decide DT-1/DT-4/DT-5/DT-6, or re-implement any part of HS-P0020's checker. Where
a page it audits is defective, it repairs at one-line grain — a restated `MUST` becomes a
clause-id link; a missing harness registration line is added; a fence's info string is
corrected — and anything larger is a stop-and-route.

## Integration contract

- **Archetype**: `capability` — a user-observable slice, though the user observed is the reader
  of the whole set rather than of one page. What they observe is the absence of a defect, which
  is why the drills are load-bearing: they are the only way an absence is demonstrated rather
  than asserted.
- **Slice / milestone**: `page-set-assurance`. Slice-mate: `answered-need-and-anchor-review`
  (HS-S0190), independent of this story and delivered in the same slice
  (`_storymap.md:149-151`). Both are properties of the assembled page set and both mount last.
- **Mount point**: **`xtask/src/narrative.rs`** — HS-P0020's registration harness, declared from
  `xtask/src/lib.rs` (`checked-documentation-surface/_design.md:96`, `:491-493`). It is the real
  composition root because it is the single file in which *whether a page's fences are exercised
  at all* is decided: a page absent from it is compiled by nothing, and `IGNORE_ALLOWANCES`
  (`:100`) is the in-tree constant this story's headline claim is a claim **about**. The mount is
  observable two ways — every page this project authored has a `#[cfg(doctest)] mod` line there
  (one module per file, `xtask/src/constitution.rs:11-18`), and `IGNORE_ALLOWANCES` names none of
  this project's fences. Where the audit finds a page missing from the harness, this story adds
  the line; that is the mount, not scope drift. **If HS-P0020 pinned a different filename, use
  the file it pinned and record the deviation — never stand up a parallel harness**
  (`_decomposition.md:118-130`).
- **Wires into**:
  - `xtask/src/main.rs` — the `REQUIRED` array and the two narrative steps that enforce both
    headline claims, plus the `"tests"` step (`:143`) that executes the crate-root and
    `overview.md` fences and the `"specification traceability"` step (`:315`). HS-P0020 adds its
    checker to `lint_steps` (`xtask/src/main.rs:799-808`), which is what puts it inside the
    story-grain gate as well as the project one.
  - `xtask/src/spec_trace.rs` — `clause_ids(root)`, `pub(crate)` beside `all_rules`
    (`checked-documentation-surface/_design.md:88-90`, `:500`). Read as a contract; not
    called from this story's own code, because this story adds no code.
  - `spec/SPECIFICATION.md` — the clause bodies and maturity markers every citation is audited
    against; `:280` for id stability. Read only.
  - `crates/happenstance/src/lib.rs`, `docs/**` and
    `examples/course-subscriptions/src/overview.md` — the authored page set, produced by the four
    stories this one blocks on (HS-S0185, HS-S0186, HS-S0187, HS-S0188).
  - `.redkiln/config.yaml` — `affected_gate` (`:40`), `reachability_static` (`:48`),
    `integration_scoped` (`:55`) and `require_ledger` (`:67`), the commands redkiln runs at this
    story's and this project's grains whether or not anyone types them.
- **Renders surfaces**: **none newly.** This story audits all four ids from `_design.md`'s
  `## Surfaces` block (`:45-65`) — `crate-root-encounter`, `opening-encounter`,
  `conceptual-bridge`, `worked-example-handoff` — and **changes** one only where the audit finds
  a defect on it. It re-composes nothing: composition, hierarchy, density and states are the
  signed-off design's and are not reopened here.
- **Public items**: **none.** `_design.md`'s `## Items` block records that this project adds,
  changes and removes no public Rust API item (`:268`), and this story adds no code at all —
  its only possible in-tree edit is a harness registration line and a one-line page repair.
- **Conformance rule(s)**: **none, and this is not adapter-observable.** No port, store, fixture
  or `suite.rs` rule is touched. Naming one would be decorative by CLAUDE.md's own test. What
  observes this story is the gate: the two narrative steps, `"tests"`, `"specification
  traceability"`, and `cargo xtask affected --base main` at the checkpoint.
- **Clause(s)**: **none discharged, none amended.** ES-25, VT-30 and CF-7 are *cited* by the
  pages this story audits (`_decomposition.md:168-173`) and their bodies are untouched; changing
  a `[FROZEN]` clause would take a new ADR and nothing here comes near one.
- **Advances DoD scenario**: initiative DoD **12** — *"No page has become a second
  specification. Each normative claim a teaching page makes is a citation that resolves, and a
  spot check confirms the page defers to the clause rather than restating it"*
  (`initiative.md:455-457`) — for this project's four surfaces, which is the whole of AC-011.
  Secondarily DoD **1** (`:413-415`), because "every fence exercised" is the per-project half of
  the narrative material building as part of the gate rather than as a manual step; and the
  second half of DoD **11** (`:452-454`), *"the specification cross-reference step passes over
  the tree as it stands after every doc comment this work touched"*, which is project DoD item 6
  (`project.md:294-296`). DoD **13**'s second disjunct — *"or no such content carries a
  load-bearing claim"* (`:458-461`) — is evidenced for this project's pages by the inventory's
  hidden-content column; the initiative-wide observation stays HS-P0020's and HS-P0025's.

## PR boundary

```
.bklg/docs-that-teach/application-author-path/fence-inventory-and-clause-audit/**
docs/**
crates/happenstance/src/lib.rs
examples/course-subscriptions/src/overview.md
xtask/src/narrative.rs
```

**Read the fence with this paragraph.** `redkiln verify --grain story` fails on any file changed
outside the first fenced block above (`.redkiln/config.yaml`, `verify:`), and four of these five
entries are expected to show **no net change at all**. They are in the boundary for two honest
reasons and not for room to manoeuvre. First, the four drills each edit a page and revert it
inside this PR; a drill that could not touch the file it falsifies would be a drill against a
copy. Second, a one-line repair — a restated `MUST` becoming a clause-id link, a corrected info
string, a missing `#[cfg(doctest)] mod` line — is what makes the audit load-bearing rather than a
complaint filed against a slice-mate.

`docs/**` is deliberately a glob rather than a list: the page filenames belong to the page
stories under HS-P0020's pinned tree, and enumerating them here would pin something that is not
this story's to pin. It is the same tree the checker itself sweeps. What the glob does **not**
license is adding a page — see below.

`spec/SPECIFICATION.md` is **absent by design**. The initiative discharges and amends no clause
(`project.md:155-157`), so a citation that does not resolve is repaired on the *page*, never in
the specification.

**In this PR**

- The fence inventory: one row per fenced block on every page this project authored, across all
  three exercise mechanisms, carrying its info string, its execution class, the mechanism that
  exercises it, and whether that mechanism can fail if the fence opts out.
- The clause-citation audit: one row per normative claim, carrying the cited id, its maturity
  marker, the resolver that proves it resolves, and the restatement verdict from HS-P0021's
  procedure.
- Four drills, run and recorded in both directions where both directions exist: opt-out on a
  `docs/` page, opt-out on the crate root, an undefined clause id on a `docs/` page, an undefined
  clause id on the crate root.
- Any one-line repair the audit's findings require on this project's own pages, and any missing
  harness registration line.
- Every finding this story does not itself close, routed to a named item.

**Explicitly not in this PR**

- **Any new page, fence, output block, mapping table or answered-need line.** Authoring belongs to
  HS-S0185 through HS-S0188 and is finished before this story starts (`_storymap.md:149-151`).
- **Any exemption.** No `IGNORE_ALLOWANCES` entry, no `ignore`, no `no_run`, no second
  exemption marker. `_design.md:206-208` and `:1055-1058` make a genuinely uncompilable fence a
  reopening of the design, not a row on a list.
- **Any edit to `_design.md`, `project.md`, `_storymap.md` or `_decomposition.md`.** Contradictions
  are recorded with a disposition and routed, exactly as the preflight did
  (`merge-forward-preflight/spec.md:230-231`).
- **Any change to HS-P0020's checker, its constants beyond a registration line, or its step
  wiring.** A gap in what the checker detects — including `no_run` — is routed to HS-P0020, not
  patched here (`_storymap.md:158-163`).
- **The answered-need and DT-1-anchor walk.** That is HS-S0190's, in this same slice, split from
  this story on mechanism (`_storymap.md:82-84`).
- **Repairing stale line citations inside the planning corpus.** The closeout's reference
  reconciliation owns those (`_design.md:736-741`, HS-P0025).

**Merge DoD one-liner** — both artifacts exist at stable paths with every row re-derivable by a
named command, the inventory shows zero opted out with each fence's exerciser and failability
stated, every cited clause id resolves with its maturity marker carried and no page restates a
clause, all four drills are recorded, and `cargo xtask ci --fast` is green on the result.

The implementer MAY also touch the composition-root / wiring files named in the Integration
contract to mount this slice; that is not scope drift. Here that means `xtask/src/narrative.rs`
specifically, and only to add a registration line the audit proves is missing.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The page set is enumerated before anything is audited** | Four surfaces from the signed-off design, in three different homes: `crate-root-encounter` → `crates/happenstance/src/lib.rs`; `opening-encounter`, `conceptual-bridge`, `worked-example-handoff` → HS-P0020's tree; plus `examples/course-subscriptions/src/overview.md`. The enumeration is taken from `_design.md`, not from a directory listing, so a page that was designed and never authored shows up as a missing row rather than as an absence nobody counted. | `_design.md:39-65`, `:707-747` |
| **The inventory unions three exercise mechanisms** | Column per fence: the mechanism that exercises it — the narrative tree's compile step, `cargo test --doc` under `"tests"`, or the same sweep via the example crate's `include_str!`. A fence mapping to none of the three is the defect the inventory exists to find, and no single command in this repository finds it. | `checked-documentation-surface/_design.md:93`, `:327-331`; `xtask/src/main.rs:143`; `_design.md:728-735` |
| **Every fence carries an execution class, not a compiled bit** | `rust` / `rust,should_panic` / `text` / `ignore` / `no_run` recorded verbatim from the info string, plus whether the fence is *executed* or only *type-checked*. `no_run` is the named concrete failure mode and is absent from HS-P0020's entire planning corpus, so whether their checker rejects it is established by running the step and recorded either way. | `_decomposition.md:616-625`; `_design.md:880-882`; `checked-documentation-surface/_design.md:562-566` |
| **Every fence carries whether its mechanism can fail on an opt-out** | Yes for `docs/` pages — the checker reports *opted out* by `path:line`. No for the crate root — an `ignore` there makes rustdoc skip the doctest and the gate stays green; that fence is protected by review (`_design.md:880-882`, anti-pattern 1) and the inventory says so rather than implying gate coverage it does not have. | `checked-documentation-surface/_design.md:565`, `:333`; `_design.md:836-838` |
| **Zero opted out is the expected result; an exception is a stop** | `_design.md` committed this project to zero uncompiled fences and zero allowance entries. The audit may not discharge a finding by writing an exemption; a fence that genuinely cannot be compiled reopens the design at the human gate. | `_design.md:206-208`, `:1055-1058` |
| **Zero opted out is falsified, not asserted** | Mark one fence on a `docs/` page `ignore`, run `cargo xtask lints` (or the narrative step directly), observe a problem line naming that page and line, revert, observe green. Repeat against the crate-root fence and record that it **fails to fail** — the expected asymmetry, and the finding that gets routed. | `checked-documentation-surface/_design.md:565`; `.redkiln/config.yaml:40,48`; `project.md:300-302` |
| **Every normative claim maps to a cited clause id with its marker** | One row per claim: the page and location, the id, its maturity marker, and the sentence's own wording. ES-25 and CF-7 are `[FROZEN]` and cited as such; VT-30 is `[PROVISIONAL]` and a page may not imply its shape is frozen or be written so it would need rewriting if VT-30 changes. Line anchors are read from the preflight's `§ Anchors` record, never re-derived, because ids are stable and lines are not. | `_design.md:400-404`; `_decomposition.md:168-173`; `merge-forward-preflight/spec.md:261`; `spec/SPECIFICATION.md:280` |
| **The resolver is named per page, because coverage is not uniform** | For `docs/` pages, HS-P0020's checker via `clause_ids(root)`. For the crate root, **nothing**: `spec_trace` reads only `spec/SPECIFICATION.md`, and a relative markdown link is not an intra-doc link, so `broken_intra_doc_links` does not see it. The audit records the resolver per row and routes the crate-root gap; it does not silently let one page's coverage stand in for another's. | `checked-documentation-surface/_design.md:88-90`; `xtask/src/spec_trace.rs:5-28`; `standards/rust/70-rustdoc-obligations.md` |
| **Citation resolution is falsified, not asserted** | Retag one cited id on a `docs/` page to an id the specification does not define, run the step, observe ``<page>:<line> — cites `ES-999`, which SPECIFICATION.md does not define``, revert, observe green. Repeat against the crate root and record that it fails to fail. | `checked-documentation-surface/_design.md:333`, `:566` |
| **Restatement is judged by HS-P0021's procedure, not a new one** | Mechanical first pass: anti-pattern 11 — every sentence containing `MUST`/`MUST NOT` that is not a link to a clause id — which is greppable across the page set. Second pass: HS-P0021's written, non-author-performable spot check, applied to this project's four surfaces and recorded with a verdict per page. Inventing a second procedure is DR-14's defect in another medium. | `_design.md:867-868`; `page-need-discipline/project.md:236-238`, `:233-235`; `checked-documentation-surface/project.md:128` |
| **Findings are closed in-boundary or routed; never absorbed** | One-line repairs on this project's own pages are made here. Everything else carries a named destination: substrate and checker gaps (including `no_run` and the crate-root coverage hole) to HS-P0020; pointer and reach gaps to HS-P0023; comprehension doubts to HS-P0024; incidental bugs to the `support` initiative. Nothing is absorbed silently. | `project.md:300-302`; `_storymap.md:158-163`; `.redkiln/config.yaml:5` |
| **Substrate absent halts the story** | If the two narrative steps are not in `REQUIRED`, both headline claims have no enforcer. Record, route to HS-P0020, and halt — do not substitute a hand count, which is the weaker criterion the project's coupling note already refuses. | `project.md:345-350`; `checked-documentation-surface/_design.md:327-331` |
| **The gate the story is held to** | `cargo xtask affected --base main` at the checkpoint (which runs the file-reading lints and `spec-trace` unconditionally, so both narrative steps ride it once HS-P0020 lands them in `lint_steps`), and `cargo xtask ci --fast` as the project bar — this project is `terminal: false`. | `.redkiln/config.yaml:40,48,55`; `xtask/src/main.rs:799-808`; `project.md:17`, `:291-293` |

**Interfaces, explicitly.** This story defines, changes and consumes **no Rust interface**, and
writes no Rust code. Its interfaces are three: two markdown artifacts in its own folder whose
section headings the ledger points at; the harness contract at `xtask/src/narrative.rs`
(one `#[cfg(doctest)] mod` per page, `IGNORE_ALLOWANCES` empty), which it reads and may append
one line to; and the checker's problem-line shape `path:line — message`
(`checked-documentation-surface/_design.md:327-338`), which is what the drills assert against
rather than a bare non-zero exit. Anything that looks like an API decision surfacing during the
audit is, by construction, someone else's already taken — record it and route it.

## Data and migrations

**N/A — no schema, no store, no data.** Nothing here reads or writes an event store, a
projection store or a checkpoint. `MemoryEventStore` is not constructed, no `Cargo.toml` is
touched, no feature is added or removed, and no serialized shape crosses a boundary. The only
persistent structures this story produces are two markdown tables in its own backlog folder,
and they are records of what the gate decided — not a source of truth anything reads back.

Two migration-shaped things are named so they are not mistaken for absent work:

- **Citation migration is not this story's.** Clause ids are stable and never renumbered
  (`spec/SPECIFICATION.md:280`); only line offsets moved at the merge, and the preflight already
  re-resolved them in its `§ Anchors` record (`merge-forward-preflight/spec.md:261`,
  `:299-305`). This story reads that record. Repairing the stale line numbers still sitting in
  `project.md`, `_grounding.md`, `_storymap.md` and `_decomposition.md` is the closeout's
  reference reconciliation (`_design.md:736-741`).
- **The drills mutate the tree and must leave it clean.** Each of the four edits a real file and
  reverts it inside this PR. The obligation that matters is the one AC-005's drill already
  states in the reader-facing case: reversibility is a state, not a footnote
  (`_design.md:824-825`). The tree at the checkpoint carries no drill residue —
  `git status` clean, and the reverted files byte-identical to their pre-drill state.

## Acceptance criteria

Six criteria. Each is framed from Persona 1's own goal — the application author whose
backbone activity here is A6, *"I want to trust the whole set, not one page"*
(`_storymap.md:46`), and whose stated fear is *"a mental model that looks right, compiles,
runs, and is quietly wrong"* (`_decomposition.md:28-30`). None of them is a bare capability:
each names the reader-visible property that fails if the criterion is unmet, and each is
verified by something that can return red.

Artifact paths are fixed here so a ledger row can point at one. **`_inventory.md`** and
**`_citations.md`** live in this story's own folder — companion files, leading underscore,
matching `_ledger.md` beside them; the names are this spec's choice and not a redkiln
convention (see `§ Clarifications resolved during spec`).

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** Persona 1 has read this project's material across all four surfaces and is deciding whether the code on the page is code that still works, **WHEN** the reviewer walks `.bklg/docs-that-teach/application-author-path/fence-inventory-and-clause-audit/_inventory.md` `§ Fences`, **THEN** every fenced block on `crates/happenstance/src/lib.rs`, on the three pages in HS-P0020's pinned tree and in `examples/course-subscriptions/src/overview.md` carries one row with: its info string verbatim, its execution class (`executed` / `compiled-only` / `prose`), the named mechanism that exercises it, whether that mechanism can *fail* if the fence opts out, and its hidden-line classification read off the **rendered** page rather than the source — **AND** the count of rows opted out (`ignore`, `no_run`, untagged-treated-as-prose, or named in `IGNORE_ALLOWANCES`) is **zero**, with the enumeration taken from `_design.md`'s `## Surfaces` block (`:45-65`) so a designed-but-unauthored page appears as a missing row rather than as an absence nobody counted. | `cargo xtask ci --fast` green, with the `"tests"` step (`xtask/src/main.rs:143-155`) and HS-P0020's two narrative steps (`checked-documentation-surface/_design.md:327-331`) named per row as the exerciser; `cargo doc -p happenstance --no-deps` for the rendered hidden-line read (`_design.md:524`); `_inventory.md § Fences` walked at tier 5. |
| AC-002 | **GIVEN** the inventory claims zero fences opted out, **WHEN** one fence on a page under HS-P0020's tree is retagged `ignore` and the narrative checker is run, **THEN** it fails with a problem line naming that `path:line` (`checked-documentation-surface/_design.md:565`), and **WHEN** the edit is reverted the gate returns green; **AND WHEN** the same retag is applied to the fence in `crates/happenstance/src/lib.rs`, **THEN** the gate stays **green** — the expected asymmetry — and `_inventory.md § Drills` records that the crate-root fence is protected by review only (`_design.md:880-882`, anti-pattern 1 at `:836-838`) with the coverage gap routed to HS-P0020 by name. Persona 1 never sees this drill; what reaches them is that "every fence is checked" is a claim someone made fail on purpose rather than a sentence someone wrote. | Drills D-1 and D-2 recorded in `_inventory.md § Drills`, each with the exact edit, the exact observed output and the revert, run via `cargo xtask lints` (`.redkiln/config.yaml:48`) and `cargo xtask ci --fast`; `git status` clean afterwards. |
| AC-003 | **GIVEN** Persona 1 meets a sentence on one of these pages that states a rule they are expected to obey, **WHEN** they follow its citation, **THEN** they land on a clause that exists — and `_citations.md § Claims` carries one row per normative claim with the page and location, the cited clause id, that clause's maturity marker verbatim from `spec/SPECIFICATION.md`, and the **named resolver for that page**: HS-P0020's checker via `clause_ids(root)` for tree pages (`checked-documentation-surface/_design.md:88-90`), and **nothing** for `crates/happenstance/src/lib.rs`, recorded as such rather than covered by another page's mechanism. ES-25 and CF-7 are cited as `[FROZEN]`; VT-30 is cited as `[PROVISIONAL]` in wording that would not need rewriting if VT-30's shape changed (`_design.md:400-404`). Every citation is an ordinary inline link in last position — never a tooltip, popover or fold (`_design.md:526`). | `cargo xtask spec-trace` (REQUIRED, `xtask/src/main.rs:315`; also `.redkiln/config.yaml:48`) plus HS-P0020's checker; line anchors read from `merge-forward-preflight/spec.md:261` and never re-derived, because ids are stable and lines are not (`spec/SPECIFICATION.md:280`); `_citations.md § Claims` walked at tier 5. |
| AC-004 | **GIVEN** the audit claims every cited clause id resolves, **WHEN** one cited id on a tree page is retagged to an id `spec/SPECIFICATION.md` does not define and the checker is run, **THEN** it fails with ``<page>:<line> — cites `ES-999`, which SPECIFICATION.md does not define`` (`checked-documentation-surface/_design.md:333`), and reverting returns it to green; **AND WHEN** the same retag is applied to a citation on the crate root, **THEN** nothing fails — `spec_trace` reads only `spec/SPECIFICATION.md` (`xtask/src/spec_trace.rs`) and a relative markdown link is not an intra-doc link, so `broken_intra_doc_links` does not see it — and that gap is recorded and routed to HS-P0020 rather than left as an implied coverage. | Drills D-3 and D-4 recorded in `_citations.md § Drills` with the exact edit, the exact observed line and the revert; `cargo xtask spec-trace` and `cargo xtask lints`; `git status` clean afterwards. |
| AC-005 | **GIVEN** Persona 1 could reasonably follow a teaching page's wording instead of the clause it points at, **WHEN** the restatement audit runs, **THEN** the mechanical pass finds **zero** sentences containing `MUST` or `MUST NOT` that are not a link to a clause id — anti-pattern 11 (`_design.md:867-868`) — across all four surfaces, **AND** a reviewer who did not author the pages applies HS-P0021's written spot check (`page-need-discipline/project.md:236-238`, non-author-performable per `:233-235`) and records a per-page verdict that no clause is restated in the page's own words. No second restatement procedure is invented; inventing one is DR-14's defect in another medium (`project.md:224-226`). | Greppable sweep for `MUST`/`MUST NOT` over the four surfaces, output pasted into `_citations.md § Restatement`; HS-P0021's spot check walked by a non-author reviewer, verdict per page recorded there; tier 5 sign-off, discharging initiative DoD-12 (`initiative.md:455-457`) for this project. |
| AC-006 | **GIVEN** the four page stories' output is already reviewed content and the design behind it is signed off (`_design.md:1039`), **WHEN** this audit finds a defect, **THEN** it is closed one of exactly two ways and never a third: an **in-place one-line repair** (a restated `MUST` becomes a clause-id link; an info string is corrected; a missing `#[cfg(doctest)] mod` line is added to `xtask/src/narrative.rs`) after which the page still renders and still satisfies the signed-off composition — `#main-content details.top-doc > div.docblock` resolves, fences stay within 68 columns and 24 rendered lines (32 on `crate-root-encounter` alone), `##` headings within 22 characters, paragraphs within 435 characters (`_design.md:532-620`), no fence behind a fold, no control outside rustdoc's own chrome, no `use happenstance_core::` import line, no diagram, every fence's output block non-empty (anti-patterns 1, 2, 3, 8, 13, 14, 15) — **OR** a **routed finding** with a named destination and no silent absorption; **AND** `_design.md`, `project.md`, `_storymap.md` and `_decomposition.md` are byte-identical at the checkpoint, the tree carries no drill residue, `IGNORE_ALLOWANCES` names none of this project's fences, and if HS-P0020's two narrative steps are absent from `REQUIRED` the story **halts and routes** rather than substituting a hand count (`project.md:345-350`). | `git diff --stat` empty on the four planning artifacts and `git status` clean at the checkpoint; `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) green; `cargo doc -p happenstance --no-deps` plus the REQUIRED `documentation` step's `RUSTDOCFLAGS=-D warnings` build for the render and bracket checks; `_inventory.md § Routing` carries one row per finding with a named destination; tier 5 reviewer sign-off against `_design.md`'s anti-pattern list. |

**Coverage of the traced project ACs.** Project **AC-007** (`project.md:253-255`) — every fence
exercised, inventory shows zero opted out — is AC-001 (the artifact and the zero) and AC-002
(the zero made falsifiable), with AC-006 holding the "no exemption is written to close a
finding" half that `_design.md:206-208` and `:1055-1058` bind. Project **AC-011**
(`project.md:266-268`) — every normative claim is a citation that resolves and a spot check
confirms no clause is restated — is AC-003 (resolves, with the marker carried), AC-004
(resolution made falsifiable) and AC-005 (the spot check). Nothing in this story traces to a
project AC it does not own, and neither owned AC is left to a single unfalsifiable table.

## Interaction quality

RFC §6.7/D6. **Every invariant below is carried by an AC-### row in the table above** — this
section names *which* id carries each and how it is verified, because `redkiln verify` extracts
ACs from table cells and bullets in `## Acceptance criteria`, and an invariant that lives only
as prose here is never gated and never tested.

The medium is rendered markdown and rustdoc; the "user" for the state family is both readers of
this project's pages (Persona 1) and the contributor who meets the gate's output. This story
authors no new surface, so its composition obligation is **preservation under repair**, which is
the harder of the two — a repair that quietly breaks a density budget looks exactly like a
repair that does not.

**STATE invariants.**

- **In-place vs context jump — AC-003.** A clause citation is provenance, not a reading step.
  It stays an ordinary inline link in last position within its sentence, never a tooltip or
  popover (`_design.md:526`; a hovered citation fails on touch and on print), and IQ-1's test
  holds: strike every off-page link and the teaching still completes (`_design.md:804-805`).
  Verified by the per-row resolver and link-form check in `_citations.md § Claims`.
- **Non-occlusion — AC-006.** A one-line repair may not occlude what it repairs. Replacing a
  restated `MUST` with a clause link must leave the sentence's claim visible in the same place;
  correcting an info string must not drop a fence's output block (anti-pattern 15). Verified by
  the render build and the reviewer walk in AC-006's cell.
- **Preserved selection / prior state — AC-006.** The "selection" this story must not clear is
  the human sign-off already recorded in `_design.md:1039` and the four page stories' reviewed
  content. Both are preserved; contradictions are recorded beside them and routed, never folded
  in. Verified by the empty `git diff` on the four planning artifacts.
- **Reversibility — AC-002, AC-004.** Four drills each mutate a real file and revert it inside
  this PR. Reversibility is a state, not a footnote (`_design.md:824-825`): the end state of
  every drill is a green gate and a clean tree, and each drill records both directions. Verified
  by `git status` clean at the checkpoint and by each drill's recorded revert.
- **Keyboard reachability — n/a, recorded rather than dropped, under AC-006.** This story adds
  no control. Anti-pattern 8 (`_design.md:858-860`) forbids this project from adding any control
  rustdoc does not already ship, and AC-006's "no control outside rustdoc's own chrome" clause is
  what keeps that true here rather than assumed. Dropping this line silently would leave a reader
  unable to tell whether it was considered.

**COMPOSITION invariants**, from the signed-off `_design.md` (binding; sign-off row `:1039`).

- **Presentation exists at all — AC-001, AC-006.** The strongest failure available to this story
  is an audit that passes every text-level assertion over pages that no longer render. AC-001
  therefore reads hidden-line classification off the **rendered** page, not the source — the
  design's hardest transience finding is that `#`-prefixed lines are *absent from the DOM
  entirely*, with no hover, focus or toggle that recovers them (`_design.md:524`), so a source
  read cannot see what a reader meets. AC-006 requires `#main-content details.top-doc >
  div.docblock` (`_design.md:46-49`) to resolve on the built docs after any repair.
- **Composition and placement — AC-006.** Anti-pattern 2 — a literal `[bracket]` pair where an
  intra-doc link failed to resolve — was measured at four on the pre-merge crate root and `cargo
  doc` did not warn (`_design.md:886-921`). The target on every page this project touches is
  zero, and a repair must not add one. Verified by the `RUSTDOCFLAGS=-D warnings` documentation
  step plus a visual bracket count on the render.
- **Transience — AC-001, AC-003, AC-006.** The policy table (`_design.md:507-529`) classifies
  every control. Three rows bind here: every fence is **persistent** and none is behind a control
  (AC-001's failability column is meaningless if a fence can be hidden); clause citations are
  **persistent, inline, recessive** (AC-003); hidden doctest lines are **binary** and forbidden
  for any `Query`, `QueryItem`, `Tags`, `Guard`, `AppendCondition`, the append call, the read
  call and every assertion (AC-001's hidden-line column is exactly this check applied per fence).
- **Density budget, with its real numbers — AC-006.** Fence width **68 columns** hard (72 is
  where `overflow-x` engages on the 696px fence at 1024×768); fence height **24 rendered lines**
  on a step page and **32 on `crate-root-encounter` alone** (finding F-4); `##` heading length
  **22 characters** before the sidebar TOC truncates with an ellipsis; paragraph length **435
  characters**; per-step budget **seven elements**; mapping table **three columns, never four**
  (`_design.md:532-620`). A one-line repair is precisely the size of edit that pushes a fence
  from 68 to 69 columns without anyone looking. AC-006 re-checks the budgets a repair can
  violate; it does not re-decide one, which would reopen a signed-off design.
- **Hierarchy — AC-005, AC-006.** The only three channels available are reading-order position,
  heading level and form (`_design.md:621-631`). Two hierarchy facts are load-bearing for this
  story: the answered-need line is above everything and the last code block on the bridge must
  be the correct guard, with the wrong-model block marked at top *and* bottom (anti-patterns 7
  and 12). A citation repair that reorders a sentence must not move either. Verified by the
  reviewer walk.
- **Named anti-patterns — AC-001 (1), AC-005 (11), AC-006 (2, 3, 8, 13, 14, 15), AC-002 (1
  again, as the drill's subject).** Anti-pattern 1 — *a code block visibly labelled as not
  compiled, not checked, `ignore`, or exempt* — is the one this whole story is an instrument for:
  *"This project ships zero. If one appears on any of these four surfaces, DT-6 was re-litigated
  without a new decision"* (`_design.md:836-838`). Anti-patterns 4, 5, 6, 9 and 10 concern page
  content this story does not author; they are the page stories' and are re-checked here only
  where a repair could disturb one, which the AC-006 render check covers.

## Error conditions

| id | condition | required response |
| --- | --- | --- |
| EC-001 | HS-P0020's two narrative steps (`the narrative tree's examples compile`, `every narrative page is checked`) are not in `REQUIRED` when this story runs. | **Halt.** Both headline claims have no enforcer and the drills have nothing to run against. Record the gap in `_inventory.md § Routing`, route it to HS-P0020 per project DoD item 9 (`project.md:300-302`), and stop — do **not** substitute a hand count, which is the weaker criterion `project.md:345-350` already refuses. |
| EC-002 | The narrative checker does **not** reject `no_run` — it is absent from HS-P0020's entire planning corpus, so its treatment is unknown until the step is run. | Record which of the two answers the checker actually gave, per fence, in `_inventory.md § Fences`. If `no_run` passes silently, that is a substrate gap routed to HS-P0020, **and** every `no_run` on this project's surfaces is still a defect to repair here — `no_run` type-checks forever without executing (`_decomposition.md:616-625`). The routing does not defer the repair. |
| EC-003 | The audit finds a fence that genuinely cannot be written as compiled, executed code. | **Stop and escalate to the sign-off owner.** This is a reopening of `_design.md`, not a row to justify: the design committed to zero uncompiled fences and zero allowance entries, and the approver accepted that consequence in writing (`_design.md:206-208`, `:1055-1058`). Do not write an exemption, do not add an `IGNORE_ALLOWANCES` entry, do not relax the fence to `ignore`. |
| EC-004 | A cited clause id does not resolve on `spec/SPECIFICATION.md`. | Repair on the **page**, never in the specification — the initiative discharges and amends no clause (`project.md:155-157`). Ids are stable and never renumbered (`spec/SPECIFICATION.md:280`), so a missing id is a semantic change, not line drift: name the id, name the page, and if no correct clause exists route it to HS-P0023 as a pointer gap rather than substituting a nearby clause nobody chose. |
| EC-005 | `xtask/src/narrative.rs` does not exist, or HS-P0020 pinned a different harness filename. | Use the file HS-P0020 actually pinned and record the deviation in `_inventory.md` (`_decomposition.md:118-130`). **Never stand up a parallel harness** — a second registration mechanism is two things to keep in sync and one that goes stale, and it would make the "every page is registered" claim unfalsifiable. If no harness exists at all, this is EC-001. |
| EC-006 | A page under the tree is authored but absent from the harness, so its fences are exercised by nothing. | Add the one `#[cfg(doctest)] mod` line — one module per file, because concatenated includes report failures at line numbers that map to no file a reader can open (`xtask/src/constitution.rs:11-18`). That is the mount, not scope drift. If the page's fences then fail to compile, that is the page story's defect: record it, route it to the owning story, and do not repair a fence's *content* here. |
| EC-007 | A drill's revert leaves the tree dirty, or a drill is recorded in one direction only. | The drill is not complete. `_design.md:824-825` makes the restored state a state, not a footnote: re-run, record both directions, and confirm `git status` clean and the touched file byte-identical to its pre-drill state before the checkpoint commit. |
| EC-008 | A one-line repair would grow into a rewrite — the restated `MUST` cannot become a link without restructuring the paragraph, or the info string correction breaks the fence. | Stop at one line. Record the finding with its measured size and route it to the owning page story (`boundary-refusal-encounter`, `boundary-falsification-drill`, `invariant-to-appendcondition-bridge` or `surface-course-subscriptions`). This story audits a page set; it does not re-author one, and a rewrite here would bypass the review the page already had. |
| EC-009 | `redkiln verify --grain story` reports files changed outside the PR boundary because a drill was mid-flight at the check. | Not a licence to widen the fence. Complete or revert the drill, then re-run. The fence is the narrowest honestly-true statement of what this story touches; four of its five entries are expected to show no net change at all. |

## Non-functional

| id | requirement | why |
| --- | --- | --- |
| NF-001 | Every row in both artifacts names the **command that re-derives it**, and no row is a hand count without one. | An inventory nobody can re-derive is the artifact whose silent decay this initiative exists to prevent. CLAUDE.md's own discipline — before adding a check, name a plausible wrong implementation it rejects — applies to a table as much as to a conformance rule. |
| NF-002 | Both artifacts are self-sufficient cold: a reviewer at closeout resolves every claim from them without re-running the audit. | Progressive disclosure only pays if the disclosed layer is complete at the point of disclosure. HS-P0025's closeout reads these; a record that replaces work is worth writing, a record that adds a second thing to read is not. |
| NF-003 | Every clause reference is stated **id-first, line-second**, with the anchor read from `merge-forward-preflight/spec.md`'s `§ Anchors` record. | Ids are stable and never renumbered (`spec/SPECIFICATION.md:280`); lines are not, and they already rotted once at the merge. Stating the id makes later drift a five-second repair rather than a second archaeology. |
| NF-004 | The drills add **no permanent cost to the gate**: no new step, no new test target, no new constant. | This story consumes HS-P0020's mechanism; it does not extend it. A drill is a one-time human-observed falsification (testing brief tier 4, `_decomposition.md:485`), recorded once, not a per-commit check. |
| NF-005 | Nothing in either artifact restates a decision from `_design.md`, `project.md` or HS-P0020's design — each is cited by path and line. | Two copies of a decision is two things to update and one that goes stale. This is the same rule the story enforces on the pages it audits, applied to its own output. |
| NF-006 | The artifacts' own vocabulary is the taught vocabulary: `happenstance`, never `happenstance_core`, wherever they name the crate a reader installs (ADR-0006). | An audit record that seeds the wrong crate name into the closeout undermines the exact consistency it is auditing for (anti-pattern 13, `_design.md:871-874`). |
| NF-007 | The project bar is `cargo xtask ci --fast`, not `cargo xtask ci`. | This project is `terminal: false` (`project.md:17`), so `integration_scoped` applies (`.redkiln/config.yaml:55`); the whole-initiative re-observation is HS-P0025's at closeout, and running it twice on an unchanged tree buys nothing. |

## Implementation notes (non-prescriptive)

- **Order that keeps every failure attributable.** Confirm the substrate first (EC-001) — the two
  narrative steps in `REQUIRED`, the harness file, `IGNORE_ALLOWANCES` empty. Then enumerate from
  `_design.md`'s `## Surfaces` block, not from a directory listing, so a designed-but-unauthored
  page shows as a missing row. Then fill the inventory from a green tree. Then run the four
  drills. Then write the routing. A drill run before the tree is green cannot distinguish its own
  failure from an inherited one.
- **Enumerate mechanically, classify by hand.** Finding every fence is a `rg` over three roots;
  deciding whether its mechanism can *fail* on an opt-out is a judgement that needs the step run
  once in each direction. Do the first with a command and paste the command into the artifact; do
  the second with the drills. Do not blur them into one column.
- **A drill is four fields, not a sentence**: the exact edit (as a diff), the exact command, the
  exact output (pasted, including the `path:line — message` shape), and the revert with `git
  status` clean afterwards. Assert on the *problem line*, not on a bare non-zero exit — a checker
  that fails for an unrelated reason would otherwise read as a passing drill.
- **The two drills that are expected to fail to fail are the valuable ones.** D-2 and D-4 exist to
  produce a green gate against a broken page, which is the finding. Record them as *observed
  green*, not as skipped, and route the gap. A drill that is quietly dropped because "it wouldn't
  work anyway" removes the only evidence that the crate-root fence is protected by review rather
  than by machinery.
- **Read the render, not only the source, for the hidden-line column.** `cargo doc -p happenstance
  --no-deps` and diff the fence in `crates/happenstance/src/lib.rs` against the fence as it
  appears in `target/doc/happenstance/index.html`: lines present in the source and absent from the
  DOM are the hidden ones, and that diff is the only way to see them (`_design.md:524`,
  `merge-forward-preflight/spec.md:425-429`).
- **The mechanical restatement sweep is a grep and should be pasted as one.** Anti-pattern 11 is
  greppable — every sentence containing `MUST` or `MUST NOT` that is not a link to a clause id.
  Paste the command and its (expected empty) output. The spot check is the second pass precisely
  because a clause can be restated without the word `MUST`; a grep alone would be the decorative
  check CLAUDE.md names.
- **Get the spot check performed by someone who did not author the pages.** HS-P0021's procedure
  is written to be non-author-performable (`page-need-discipline/project.md:233-235`); running it
  on your own prose measures your memory of the clause, not the page.
- **Keep both artifacts tables.** They will be opened under time pressure at closeout and by
  HS-P0025's reference reconciliation. Prose that has to be read to be searched is a context jump
  wearing a different hat.

## Tests and CI (merge gate)

Grounded in the project testing brief's five tiers (`_decomposition.md:480-486`), which this
story rides rather than extends. Tiers stated `n/a` are stated, not omitted, per that brief's own
rule (`:517-518`).

| tier | command / path | proves |
| --- | --- | --- |
| 1 — structural, citations | `cargo xtask spec-trace` — REQUIRED step `"specification traceability"` (`xtask/src/main.rs:315`) | AC-003 — `spec/SPECIFICATION.md`'s internal traceability still holds over the tree this story leaves, which is project DoD item 6 (`project.md:294-296`). It does **not** open a narrative page; that distinction is why AC-003 names a resolver per page. |
| 1 — structural, page citations | HS-P0020's `every narrative page is checked` step via `clause_ids(root)` (`checked-documentation-surface/_design.md:88-90`, `:331-333`) | AC-003, AC-004 — a clause id cited *by a page* resolves, and fails by `path:line` when it does not. The only mechanism in the repository that does this. |
| 1 — structural, render | `cargo doc -p happenstance --no-deps`, plus the REQUIRED `documentation` step's `RUSTDOCFLAGS=-D warnings` build inside `--fast` (`_decomposition.md:557-559`) | AC-001, AC-006 — the pages actually render, the hidden-line column is read off the DOM rather than the source, and anti-pattern 2's bracket count stays at zero. |
| 2 — compiled fence | HS-P0020's `the narrative tree's examples compile` step (`checked-documentation-surface/_design.md:327-331`) | AC-001 — every fence under the tree type-checks against the real workspace crates. Its **absence** is EC-001 and halts the story. |
| 3 — executed | `cargo test --locked --workspace --all-features -- --show-output` — REQUIRED step `"tests"` (`xtask/src/main.rs:143-155`) | AC-001 — the crate-root fence and `examples/course-subscriptions/src/overview.md` are *executed*, not merely compiled; this is the tier that distinguishes the execution class column from a compiled/not bit. |
| 4 — falsification drill | D-1 and D-2 in `_inventory.md § Drills`; D-3 and D-4 in `_citations.md § Drills` — each an edit, a command, a pasted `path:line — message`, a revert, and `git status` clean | AC-002, AC-004 — both headline claims are made to fail where a mechanism exists, and recorded as failing-to-fail where none does. Tier 4 is a signed-off observation, not a per-commit gate (`_decomposition.md:485`). |
| 5 — review sign-off | A non-author reviewer walks `_inventory.md` and `_citations.md` against `_design.md`'s anti-pattern list and applies HS-P0021's spot check (`page-need-discipline/project.md:236-238`) | AC-001, AC-005, AC-006 — the "or names each exception" half, the no-restatement verdict per page, and the composition-preservation walk. `design.capture` is deliberately absent from `.redkiln/config.yaml`, so this written record is the only record these observations will ever have. |
| story grain (redkiln `affected_gate`) | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | AC-006 — the checkpoint commit does not break what its diff could reach; it runs the file-reading lints and `spec-trace` unconditionally, which is what covers a story whose deliverable maps to no package. Runs whether or not anyone types it. |
| static reachability (redkiln `reachability_static`) | `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:48`) | AC-002, AC-004 — the drills' harness: once HS-P0020 lands its checker in `lint_steps` (`xtask/src/main.rs:799-808`), `cargo xtask lints` is the cheap command that returns red on a drilled page. |
| integration grain, non-terminal (redkiln `integration_scoped`) | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | AC-001, AC-006 — the project bar. This project is `terminal: false` (`project.md:17`); HS-P0025 owns the whole-initiative `cargo xtask ci` re-observation at closeout. |
| 0 — preflight | **n/a.** The merge-forward preflight is `merge-forward-preflight`'s tier 0 (`_decomposition.md:535`) and is a precondition on this story, not a check it re-runs. | — |
| conformance suite | **n/a.** No port, store, fixture or `suite.rs` rule is touched; sweeping `event_store_conformance!` here would prove the wrong thing and cost real CI time doing it (`_decomposition.md:589-594`). | — |
| ledger gate | `redkiln verify --grain story` over `_ledger.md` (`.redkiln/config.yaml:67`) | all six — every AC present, satisfied, and carrying non-placeholder evidence before `implement → report`. |

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | mitigation in this PR |
| --- | --- | --- |
| The inventory becomes a table nobody can fail — a count asserted rather than a property derived. | High / High | The named defect this spec was written against. NF-001 requires a re-deriving command per row; AC-002 and AC-004 require the two headline claims to be *observed failing*; AC-001's failability column states, per fence, whether any mechanism could have caught it. |
| HS-P0020's substrate is not in `REQUIRED` when this story runs, and the implementer produces a hand count to keep the slice moving. | Medium / High | EC-001 halts explicitly and `project.md:345-350` already refuses the weaker criterion. The halt is loud: a routed finding to HS-P0020, not a green ledger row. |
| A finding is closed by writing an exemption, because an allowance list exists and adding to it is one line. | Medium / High | EC-003 and AC-006. `_design.md:206-208` committed this project to zero, and the sign-off (`:1055-1058`) recorded that an uncompilable fence reopens the design. `IGNORE_ALLOWANCES` naming none of this project's fences is an AC-001 check, not a convention. |
| A one-line repair grows into a rewrite of a page another story authored and a reviewer already accepted. | Medium / Medium | EC-008 caps the repair at one line and routes anything larger to the owning page story. The PR boundary paragraph already states that four of five entries are expected to show no net change. |
| The crate-root coverage gap is discovered, felt embarrassing, and quietly fixed by moving the fence into the tree. | Low / High | Out of scope by construction: `include_str!` resolves against the file tree at compile time and a path escaping the package would not resolve once published (`crates/happenstance/src/lib.rs:7-9`), which is why the program exists twice on purpose (`_design.md:707-726`). The gap is recorded and routed, not designed around. |
| A drill leaves residue — a reverted file that is not byte-identical, or a stale `target/` artifact read as evidence. | Medium / Medium | EC-007; AC-002 and AC-004 both require `git status` clean; the drill's four fields include the revert. |
| `no_run` turns out to pass HS-P0020's checker silently, and the routing is treated as a substitute for the repair. | Medium / High | EC-002 states both obligations: route the substrate gap **and** repair the fence here. `_decomposition.md:616-625` names `no_run` as the concrete failure mode; the risk is the whole reason the inventory carries an execution class. |
| The restatement verdict is produced by whoever wrote the pages. | Medium / Medium | AC-005 names a non-author reviewer and cites HS-P0021's non-author-performable clause (`page-need-discipline/project.md:233-235`). |
| This story and its slice-mate `answered-need-and-anchor-review` overlap and one absorbs the other's findings. | Medium / Low | The slice splits on **mechanism, not subject** (`_storymap.md:82-84`): this story is what the gate steps and the allowance list can decide; the other is what only a reviewer can. A finding about answered-needs or the DT-1 anchor is routed there, not audited here. |
| Stale line citations inside the planning corpus send the implementer to the wrong place mid-audit. | Medium / Medium | `merge-forward-preflight`'s `§ Anchors` record is the superseding table (`:299-305`) and NF-003 requires id-first reading; repairing the stale pointers themselves is the closeout's reference reconciliation (`_design.md:736-741`, HS-P0025). |

## Dependencies

**Blocks on** — four stories, matching `_storymap.md:61`'s `depends_on` cell exactly. All four
author the page set this story audits; an audit run before they land measures an empty set.

- `boundary-refusal-encounter` (HS-S0185) — authors the `opening-encounter` surface and rewrites
  `crates/happenstance/src/lib.rs`. Both the crate-root fence and step three's fence are inventory
  rows, and its clause citations (ES-25 above all) are audit rows.
- `boundary-falsification-drill` (HS-S0186) — makes the boundary load-bearing to the repository.
  Its drill is the shape this story's four drills borrow, and the check it lands is one of the
  mechanisms the inventory's failability column reports on.
- `invariant-to-appendcondition-bridge` (HS-S0187) — authors the `conceptual-bridge` surface,
  which carries the mapping table, the VT-30 and CF-7 citations, and (if DT-6 shipped one) the
  wrong-model contrast — the single block most likely to attract an `ignore`.
- `surface-course-subscriptions` (HS-S0188) — authors the `worked-example-handoff` surface and
  `examples/course-subscriptions/src/overview.md`, the third exercise mechanism the inventory
  unions and the one whose fence rides `cargo test --doc` rather than the tree's step.

Transitively, both `preflight-and-anchor` stories: `merge-forward-preflight` (whose `§ Anchors`
record this story reads rather than re-deriving) and `tension-resolutions` (which authored the
`_design.md` this story audits against and may not edit).

**Substrate, not a story edge:** HS-P0020 `checked-documentation-surface` owns the pinned tree,
the two narrative steps, `clause_ids`, the harness and `IGNORE_ALLOWANCES`; HS-P0021
`page-need-discipline` owns the restatement procedure AC-005 borrows. Neither is re-implemented
here, and a gap in either is routed rather than patched (`project.md:308-314`).

**Unlocks:**

- `answered-need-and-anchor-review` (HS-S0190) — its slice-mate, **independent of this story**
  (`_storymap.md:149-151`); the two mount together as one `page-set-assurance` slice and neither
  waits on the other.
- The project's Definition of done items 4, 6 and 9 (`project.md:289-302`), and through them
  HS-P0025's closeout, which reads both artifacts as the evidence for initiative DoD-12 and the
  per-project half of DoD-1 and DoD-11.
- `comprehension-evidence` (HS-P0024) — a friction log against a half-assembled surface measures
  the assembly, not the teaching (`project.md:323-324`); this story is the last thing that says
  the surface is assembled.

## Anchors (progressive disclosure)

Each anchor is deeper material this spec deliberately did **not** paste. Open the one whose AC you
are working on, at the moment named — not the corpus.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/application-author-path/_design.md` | The signed-off, **binding** design. It spent this story's discretion on zero (`:206-208`, sign-off consequence `:1055-1058`), enumerates the four surfaces and their three homes (`:45-65`, `:707-747`), classifies every control (`:507-529`), carries the density budget's real numbers (`:532-620`) and the fifteen anti-patterns (`:831-885`). Every AC-006 number and every "may not" in this spec traces here. | Before writing a single inventory row, and again before accepting any repair. Never with an editor open on it — it is signed off. | AC-001, AC-005, AC-006 |
| `.bklg/docs-that-teach/application-author-path/tension-resolutions/_resolutions.md` | The commitments this audit checks the tree against, decided once. `§ DT-5+DT-6` records **zero** entries on HS-P0020's allowance list and **zero** uncompiled fences as a commitment with a consequence — an unwritable fence reopens the design rather than buying an exemption — and records **UX-015 as vacuously satisfied, with its reason**, which is the shape an inventory row for an obligation that never fired should take: dropped-because-it-did-not-fire and forgotten are indistinguishable otherwise. `§ Anchor table` is the heading / fragment-id table every rendered page is audited against, with the 22-character budget scoped to `crate-root-encounter` and anti-pattern 5 scoped with it. | Before writing the failability column, and before recording any row for an obligation that did not fire. | AC-001, AC-005, AC-006 |
| `.bklg/docs-that-teach/checked-documentation-surface/_design.md` | HS-P0020's design: `TREE = "docs"`, `HARNESS`, `IGNORE_ALLOWANCES` and `clause_ids` in `## Signatures` (`:83-110`); the checker's exact problem-line shape and the four problem classes (`:327-338`, `:562-566`); the two REQUIRED step banners. The drills assert against **these strings**, not against a bare non-zero exit. | Before running any drill, and before writing the failability column. | AC-001, AC-002, AC-003, AC-004 |
| `.bklg/docs-that-teach/application-author-path/_decomposition.md` | The UX brief (`:11-446`) and testing brief (`:448-650`). The five-tier table (`:480-486`), the AC-to-tier mapping (`:520-535`), the merge-gate command list (`:543-574`), and the two Notes that define this story: `no_run` as the concrete failure mode (`:616-625`) and "AC-007's inventory is a review artifact, not a script" (`:638-643`). | Before filling the Tests table's tier column and before classifying any fence's execution class. | AC-001, AC-002 |
| `.bklg/docs-that-teach/application-author-path/project.md` | AC-007 verbatim (`:253-255`) and AC-011 verbatim (`:266-268`) — the two this story traces to; the nine DoD items including item 6's spec-trace obligation and item 9's routing rule (`:279-302`); and the coupling note that refuses a weaker criterion (`:345-350`), which is EC-001's authority. | First, before anything else, to fix what is actually owed. | AC-001, AC-003, AC-006 |
| `.bklg/docs-that-teach/page-need-discipline/project.md` | HS-P0021's AC-010 (`:236-238`) — the citation rule and the spot check this story **borrows rather than invents** — and AC-009's non-author-performable clause (`:233-235`), which is who may run it. | Immediately before the restatement pass; do not design a procedure first. | AC-005 |
| `.bklg/docs-that-teach/application-author-path/merge-forward-preflight/spec.md` | The baseline this story reads anchors from: `§ Anchors` (`:299-305`, `:261`) carries the post-merge clause line numbers for ES-25, VT-30 and CF-7, and `:425-429` describes the render-diff technique for reading hidden lines off the DOM. Re-deriving these is the archaeology that story already did. | While filling the maturity-marker column, and before the hidden-line column. | AC-001, AC-003 |
| `.bklg/docs-that-teach/application-author-path/_storymap.md` | Backbone activity A6 (`:46`) — the persona intent every AC above is framed from; this story's row (`:61`); the slice rationale that splits it from its slice-mate on mechanism (`:79-84`); the merge order (`:149-151`); and the routing paragraph naming every destination (`:158-163`). | When routing a finding, and before touching anything the slice-mate owns. | AC-005, AC-006 |
| `spec/SPECIFICATION.md` | The clause corpus every citation is audited against. `:280` states ids are stable and never renumbered — the single sentence NF-003 and EC-004 rest on. Maturity markers (`[FROZEN]`, `[PROVISIONAL]`) are read from the clause itself, not from a summary. | While filling `_citations.md § Claims`; open by id search, never by scrolling to a remembered line. | AC-003, AC-004 |
| `xtask/src/main.rs` | The `REQUIRED` array — the `"tests"` step at `:143-155` that executes the crate-root and `overview.md` fences, the `"specification traceability"` step at `:315`, and `lint_steps` at `:799-808` where HS-P0020's checker lands, which is what puts it inside `cargo xtask lints` and therefore inside the story grain. EC-001's check is a read of this file. | First, at substrate confirmation, before any inventory work begins. | AC-001, AC-002, AC-006 |
| `xtask/src/spec_trace.rs` | What `spec-trace` actually does — it checks `spec/SPECIFICATION.md`'s *internal* traceability and never opens a narrative page. Reading it is how the implementer avoids the storymap one-liner's shorthand and states the right resolver per page. `clause_ids` will be added here as a sibling of `all_rules`. | Before writing the resolver column; the moment anyone says "spec-trace checks the pages". | AC-003, AC-004 |
| `xtask/src/constitution.rs` | `:11-18` and `:39-50` — the one-module-per-file rule and `include_str!`'s identical-bytes guarantee. It is the existing, working precedent for the harness registration line EC-006 may add, and the reason concatenation is forbidden. | Only if a page is missing from the harness and a registration line must be written. | AC-001, AC-006 |
| `crates/happenstance/src/lib.rs` | The one surface outside the tree. `:7-9` records why `include_str!` cannot reach into the tree from here, which is the whole reason for the coverage asymmetry D-2 and D-4 exist to demonstrate; ADR-0006's reasoning at `:20-25` is preserved, never rewritten. | Before running D-2 and D-4, and before any repair on this file. | AC-002, AC-004, AC-006 |
| `.redkiln/config.yaml` | `affected_gate` (`:40`), `reachability_static` (`:48`), `integration_scoped` (`:55`) and `require_ledger` (`:67`) — the commands redkiln runs at this story's and this project's grains whether or not anyone types them; `:5` names the `support` initiative incidental bugs route to. | Before the checkpoint, and again when routing anything found. | AC-002, AC-004, AC-006 |
| `.bklg/docs-that-teach/initiative.md` | The gold source. DoD-12 (`:455-457`) is what AC-005 discharges for this project; DoD-1 (`:413-415`), DoD-11 (`:452-454`) and DoD-13 (`:458-461`) are the ones AC-001 and AC-003 feed. Reading the vision framing is what stops both artifacts drifting into a changelog. | Once, before writing either artifact's opening paragraph. | AC-001, AC-005 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | Persona 1's measured journey and stated fear (`:99-106`) — "a mental model that looks right, compiles, runs, and is quietly wrong" — the user intent every AC above is framed from and the reason a silently-unchecked fence is a harm rather than untidiness. | If any AC's user-intent framing is ever questioned, or when writing what the artifacts are *for*. | AC-001, AC-003 |
| `standards/rust/70-rustdoc-obligations.md` | The only constitution atom on documentation, and the source of the counter-example that is green under the whole workspace — i.e. the precedent for a doc that passes every check while teaching the wrong thing. RS-70-2 is why the `RUSTDOCFLAGS=-D warnings` build catches anti-pattern 2. | Before the render check in AC-006, and when judging whether a fence's info string is defensible. | AC-001, AC-006 |
| `.kb/decisions/0006-bare-name-to-the-typed-layer.md` | The Accepted decision atom behind the taught vocabulary — `happenstance` is the typed layer and the name a reader installs. NF-006 and anti-pattern 13 both derive from it. | Before writing any crate name into either artifact, and when auditing an import line. | AC-006 |
| `docs/README.md` | `:25-29` — why a tree the gate reads is pinned by path. It is the reader-facing statement of the constraint that makes "a page absent from the harness is exercised by nothing" true. | When explaining the three-mechanism union in `_inventory.md`'s preamble. | AC-001 |

## Clarifications resolved during spec

1. **The AC set is exactly the six ids the first pass decided** — AC-001 through AC-006. None
   added, none dropped. The ledger matches one row per id.
2. **Two artifacts, two fixed paths.** The front half said "two audit artifacts"; a ledger row
   needs a path, so they are named here: `_inventory.md` (`§ Fences`, `§ Drills`, `§ Routing`)
   and `_citations.md` (`§ Claims`, `§ Restatement`, `§ Drills`). Both live in this story's own
   folder — the only place it may author — with a leading underscore marking them companions
   rather than stage artifacts, matching `_ledger.md` beside them. The names are this spec's
   choice, not a redkiln convention.
3. **The four drills are numbered and split across the two artifacts.** D-1 (opt-out on a tree
   page) and D-2 (opt-out on the crate root) live in `_inventory.md § Drills` and are AC-002's;
   D-3 (undefined clause id on a tree page) and D-4 (undefined clause id on the crate root) live
   in `_citations.md § Drills` and are AC-004's. Splitting them by artifact keeps each headline
   claim beside its own falsification instead of in a separate appendix.
4. **A drill asserts on the problem line, not on the exit code.** `checked-documentation-surface/_design.md:327-338`
   pins the `path:line — message` shape; a checker failing for an unrelated reason would otherwise
   read as a passing drill. This is stated because the front half described the drills without
   fixing what they assert.
5. **D-2 and D-4 are recorded as *observed green*, not skipped.** The two drills expected to fail
   to fail are the ones that produce the finding; dropping them because "it wouldn't work anyway"
   would remove the only evidence that the crate-root fence is protected by review rather than by
   machinery.
6. **`no_run` carries two obligations, not one** (EC-002). If HS-P0020's checker turns out not to
   reject it, the substrate gap routes to HS-P0020 **and** every `no_run` on this project's
   surfaces is still repaired here. The routing does not defer the repair — that conflation is how
   an audit becomes a complaint filed against a sibling.
7. **The interaction-quality invariants are AC rows, not bullets.** Every applicable STATE and
   COMPOSITION invariant is carried by an id in the acceptance table; `§ Interaction quality`
   only names which id carries which. Keyboard reachability is recorded as `n/a` with its reason
   (this story adds no control; anti-pattern 8 forbids one) rather than dropped, so a later reader
   can tell it was considered.
8. **Composition is *preserved under repair*, not re-decided.** The temptation is to read "authors
   no page" as "has no composition obligation". The opposite holds: a one-line repair is exactly
   the size of edit that pushes a fence from 68 to 69 columns with nobody looking, which is why
   AC-006 re-checks the budgets a repair can violate and why the hidden-line column is read off the
   render rather than the source.
9. **Nothing here reopens DT-1, DT-4, DT-5 or DT-6, and no exemption may be written** (EC-003).
   `_design.md` is signed off; a fence that genuinely cannot be compiled is a human's gate to
   reopen, not a row on an allowance list. This is the one place where the honest response to a
   finding is to stop.
