---
item: HS-S0152
stage: spec
created: 2026-08-17T13:16:10.990Z
updated: 2026-08-17T13:16:10.990Z
template_sig: 87bbf1d0
rendered_sig: 10b96ce2
---

# Spec — A governed page names the discipline as the reason it is shaped as it is

## Scope lock

| Level | Path and the part that binds this story |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` — **DoD-14** (`:462-464`, *"the discipline is on disk and cited"* — a written account exists in its decided home, **is reachable from the material it governs**, and **at least one page cites it as the reason it is shaped as it is**); AC-13 |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` — the BR-12 gate decision (a new tree, sibling to `standards/rust/`, pinned by path) and the HS-P0020 seam this story reaches across |
| Project | `.bklg/docs-that-teach/page-need-discipline/project.md` — **AC-011** (this story owns the *cited in place* half); **DR-10**; the non-goal "authoring any teaching content" |
| Briefs (key) | `.bklg/docs-that-teach/page-need-discipline/_decomposition.md` — Architecture brief **AC-011** (`:130-137`, two mount points, and "the link target must be a path the checker already pins, so a tree move breaks the build rather than the link") and **Note 2** (`:221-246`, one path constant for the pages tree, not two); UX brief **UX-006** (`:504-510`) and **UX-010**; Testing brief **AC-011** (`:937-956`, and the explicit instruction that the cross-project half stays a one-time procedural confirmation, never a shared scanner over two trees) |
| Signed-off design (**BINDING**) | `.bklg/docs-that-teach/page-need-discipline/_design.md` — `## Surfaces` (`page-need-declaration`, `discipline-router`, and the pinned-constant table), `### S1 vocabulary — DT-3` (RP-10-2 / RP-10-3, the `orientation` ceilings), `## Composition` S1 (nothing between the H1 and the declaration) and S2 (the router's binding region order), `## Density budget` (router ≤ 8,192 bytes), `## Transience policy`, `## Anti-patterns` 1, 5, 6, 9, 12 |
| Sibling project (the tree this story writes *into*) | `.bklg/docs-that-teach/checked-documentation-surface/_design.md` — `## Signatures`, `const TREE: &str = "docs";` — HS-P0020 pins the governed tree at **`docs/`**; `.bklg/docs-that-teach/checked-documentation-surface/_storymap.md` — the fixture page that exists there at merge |
| Dependency specs | `.bklg/docs-that-teach/page-need-discipline/router-precedence-and-announcement/spec.md` — the router's authored regions and its 8,192-byte budget; `.bklg/docs-that-teach/page-need-discipline/page-need-checker-mounted-in-the-gate/spec.md` — the pin and the copied link check this story leans on |
| Story map | `.bklg/docs-that-teach/page-need-discipline/_storymap.md` — slice `binding-beyond-this-project`; this story's row; "Why the slices fall here", last bullet (*both its stories reach outside this project's own diff*) |
| Grounding | `.bklg/docs-that-teach/page-need-discipline/_grounding.md` — no Accepted decision atom governs documentation trees; the binding authority here is sub-ADR |
| Roadmap pointer | **None.** `RUNBOOK.md` carries no phase for this initiative. The roadmap of record is the story map's merge order, which puts this story after both its dependencies and before `playbook-atom-staged-for-ingest`. |
| Design mock (built, awaiting sign-off) | `.bklg/docs-that-teach/page-need-discipline/design/mock.html` — the `page-need-declaration` frames, including `declared-valid` and `plain-text-pager` |

## One-line PR slice

At least one page in HS-P0020's pinned tree names the discipline as the reason it is
shaped as it is, as a link the checker's own pin keeps resolving, and the discipline
links back to the material it governs.

## Executive summary

**What this PR lands.** Two edits, one in each tree, and together they close the last
third of DoD-14. In the **governed** tree: a one-sentence, in-place citation on
`docs/README.md` naming the rule that dictates that page's shape, with the rule id as
the visible link text. In the **rules** tree: a back-link in `standards/pages/README.md`'s
scope paragraph naming `docs/` as the material these rules govern. After this PR the
relationship is legible from both ends — a reader standing on a governed page can reach
the rule that shaped it in one hop, and a reader standing on the rule can reach what it
governs in one hop.

**Pointer, not restatement.** The discipline itself is already on disk and already
announced: `router-precedence-and-announcement` landed the tree, the rank and the
`docs/README.md` table row, and its own spec is explicit that the remaining third of
DoD-14 — *at least one page cites it as the reason it is shaped as it is* — is this
story's. This story writes **no teaching content** (`_storymap.md`, "Deliberately not
stories here": *adds a link and a reason to a page HS-P0020/HS-P0022 own; it writes no
teaching*) and **no new `xtask` code**.

**The delta worth reading twice.** Three things in this PR are easy to get subtly wrong,
and each is settled as a decision in the Context pack rather than left to the implementer:

1. **An announcement is not a citation.** `docs/README.md` already links the rules tree
   from its "Looking for / It is at" table. That row says *the tree exists and is over
   there*. It does not say *this page looks like this because of that rule*, which is
   what DoD-14 and UX-006 ask for. A second table row would discharge nothing.
2. **The citing page must be a page the discipline actually governs**, i.e. one the
   checker's own walk reports on — otherwise "cited by what it governs" is satisfied by
   a page nothing governs, which is the decorative outcome one level up from the
   decorative rule this project exists to refuse.
3. **The citation must survive the discipline it cites.** `docs/README.md` is an
   `orientation` page, and RP-10-2 caps an orientation page at links plus at most one
   sentence per destination, teaching nothing. A citation that explains itself for a
   paragraph makes the page answer a second need — the citing page failing the rule it
   cites, in the same commit.

## Context pack

The decisions this story must honor, stated inline. Everything deeper is a signposted
anchor in the second half of this spec — link, do not paste.

**Both trees are pinned, and this story adds no third pin.** The rules tree is
`standards/pages/` with its router at `standards/pages/README.md` (`_design.md`
pinned-constant table: `RULE_DIR`, `ROUTER`). The governed tree is **`docs/`** —
HS-P0020's `const TREE: &str = "docs";`
(`.bklg/docs-that-teach/checked-documentation-surface/_design.md`, `## Signatures`),
consumed here by value and never re-declared: *one path constant for the pages tree, not
two* (architecture brief, Note 2, item 1 — a second const "drifts silently the day the
tree moves"). This story writes no const at all; it writes prose whose links assume the
two spellings that already exist.

**The citing page is `docs/README.md`, and the reason it is the right one is that its
shape is the discipline's doing.** It is the governed tree's index — HS-P0020's
`narrative-tree-index` surface — and under this project's own vocabulary it is an
`orientation` page: RP-10-2 says an orientation page contains links and at most one
sentence per destination and *teaches nothing*, and RP-10-3 says there is at most one
such page per directory level (`_design.md`, `### S1 vocabulary — DT-3`). So the sentence
"this page routes and does not explain, because RP-10-2 says an orientation page must
not" is **true**, not decorative — which is the bar the whole story exists to clear.
Two alternatives and why they lost: **HS-P0020's fixture page** — it exists at merge, but
its shape is dictated by the *compiling* check, not by this discipline, so the citation
would be an assertion about a rule that did not shape it; **an HS-P0022 teaching page** —
the best long-term citer, but this story may not author teaching content and must not
block on a project that merges later. Neither is forbidden as an *addition*: the bar is
"at least one", and a later story adding a second citer does not replace this one.

**A conditional this story must resolve rather than assume, with the tie-break decided
in advance.** Whether `README.md` is inside the checker's governed set is
`page-need-checker-mounted-in-the-gate`'s decision, not this story's — the precedent it
copies excludes `README.md` from its corpus (`xtask/src/lint_constitution.rs:212-219`),
and if the page checker inherits that exclusion then `docs/README.md` is not a governed
page and citing it discharges nothing. **The rule is: the citing page must be one the
checker's walk reports on.** If `docs/README.md` is excluded, the citation additionally
lands on the governed page nearest the reader that the walk *does* report — HS-P0020's
fixture page — and the ledger records which. This is checked, not assumed: the membership
probe is an AC.

**The citation's form is fixed by the design, and three of its four constraints are
positional.** One sentence, in place, on the page whose shape it explains:

- it sits **below the first prose paragraph** and **above** the page's routing table —
  never in a footer, never at the end (the design's standing rule that a repair or a
  reason belongs where the reader is, `_design.md` `## Transience policy`, S4 repair row,
  and `## Anti-patterns` 12);
- it may **never** be inserted between the H1 and the `> **Answers:**` declaration —
  `_design.md` `## Composition`, S1: *"Nothing may be inserted between the H1 and the
  declaration"*, and anti-pattern 1 makes that a screenshot-checkable failure;
- it is **persistent chrome** — not behind a `<details>`, a tab or a panel. It states a
  constraint on the page, which is never-fold class 3, and `PERMITTED_FOLD_MECHANISMS`
  ships empty, so anti-pattern 6 forbids the mechanism outright;
- the **rule id is the visible link text** (`RP-10-2`), so the reader can see they are
  being handed to the rule rather than to a paraphrase of it. This is UX-010's shape,
  applied to a rule id instead of a clause id (`_decomposition.md`, UX brief, `:533-541`).

**Cite, never restate — and here the rule bites its own citation.** The discipline's own
authoring rule is that a normative claim is a citation and never a restatement of the
thing cited (DR-09; the constitution's "Clause or atom?" test at
`standards/rust/README.md`). The citation sentence therefore names the rule and links it;
it does not reproduce what RP-10-2 says. The failure this forbids is concrete and cheap
to commit: a paragraph on `docs/README.md` explaining the discipline is a second copy of
the rules tree, and *"one of the two will be stale"*
(`xtask/src/lint_constitution.rs:466-470`).

**The back-link is one link, and deliberately not an index.** The router gains a resolving
link to `docs/` in its **scope paragraph** — region 2 of the binding region order, so no
region moves and no heading is added (`_design.md` `## Composition`, S2). It must not
become a generated index of governed pages and their declared needs: `_design.md`
declined exactly that, because its source is HS-P0020's tree and adopting it would make
the rules tree change on every page addition, coupling two projects' commit cadence
(`### S2 — discipline-router`, "Rejected"). One link is reachability; an index is
coupling.

**Two mechanical consequences of putting a `../../docs/README.md` link in the router, both
load-bearing.** The router's link check — copied verbatim from `check_router` by the
checker story — resolves each markdown link as `root.join(RULE_DIR).join(target)` and
asserts it exists (`xtask/src/lint_constitution.rs:343-356`, the join at `:349`). So:
(a) the back-link **is** mechanically checked the moment the checker lands, and a
back-link that rots fails the gate; (b) that check must **not** be narrowed to "targets
inside `RULE_DIR`" — the inherited semantics resolve relative to the router's own
directory, and narrowing them would report this story's correct back-link as a dangling
link. That is a forward obligation on `page-need-checker-mounted-in-the-gate`, recorded
here because this is the story whose diff would break.

**What protects the outbound link is the pin, not a link check — and the difference is
stated, not blurred.** Nothing checks that `docs/README.md`'s link into
`standards/pages/` resolves: this project owns no scanner over the governed tree, and it
may not build one. Architecture brief Note 6 and RS-81-3 forbid one scanner ranging over
two trees (`standards/rust/81-checks-that-cannot-be-types.md:209`), and the testing brief
is explicit that this half *"stays a one-time procedural confirmation, not a permanent
check"* (`_decomposition.md:948-956`). What the repository does guarantee is that the
target cannot move quietly: `RULE_DIR` and `ROUTER` are `const`s the checker reads, and a
missing tree is `.with_context()` plus a vacuity `bail!`, never a skip
(`xtask/src/lint_constitution.rs:212`, `:176`). Moving the rules tree therefore breaks the
**build**, in the same change, rather than breaking the link silently — which is
precisely the property architecture brief AC-011 asks the link target to have. Saying
"the link is checked" would be the false-guarantee failure RS-81-1 names
(`81-checks-that-cannot-be-types.md:11`); the blind spot is stated instead, and stating it
is an AC.

**The persona-journey slice this realizes — and it is the initiative's one *measured*
defect.** The adapter author's `E0034` explanation exists, in `RUNBOOK.md`, an ADR and
evaluation documents — *"all contributor-facing, none of them
`crates/happenstance-core/src/store.rs`, the file the reader is looking at at the moment
they need it"*
(`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:174-181`). A
discipline that exists in `standards/pages/` and is invisible from `docs/` is that defect
wearing this project's name. UX-006's test is the antidote and is stated as a traverse,
not a property: *from a governed page, reach the governing rule in one navigation step,
keyboard only* — and back again.

**Governance test for touching text that already discharges something.** Both files this
PR edits already do a job: `docs/README.md` routes readers to eight trees and carries the
`[PROVISIONAL — settles at …]` marker the router story left, and
`standards/pages/README.md` is a signed-off composition. The rule is to rewrite the
referent and never the reasoning
(`.kb/governance/rewrite-the-referent-never-the-reasoning.md`): nothing existing in either
file is reworded, no region is reordered, no marker is removed. This PR **adds** one
sentence to each.

**What this story is forbidden from doing, stated as decisions.** No `xtask/src/**` change
of any kind — not the checker, not `affected.rs`, not a link scanner over `docs/`
(architecture brief Note 6; `git diff main -- xtask` is empty at merge). No teaching
content on any page. No second table row in `docs/README.md` standing in for the citation.
No generated page-need index in the router. No `.kb/` write — the playbook atom is the
slice-mate's, staged under `.kb/_intake/`. No edit to `standards/rust/README.md`, whose
untouched precedence block is still AC-002's evidence one slice back.

## Integration contract

**Slice / milestone.** `binding-beyond-this-project`. Slice-mate, implemented in the same
context and mounted as one surface: `playbook-atom-staged-for-ingest`. The two are grouped
because both reach outside this project's own diff — one into HS-P0020's pages tree, one
into the ingest path HS-P0025 drains — and neither can be observed while the discipline is
still moving (`_storymap.md`, "Why the slices fall here", last bullet).

**Archetype.** `capability` — a user-observable slice. The observable user is a reader
standing on a governed page who wants to know why it is shaped the way it is, and a reader
standing on the rule who wants to see what it governs.

**Mount point.** **`docs/README.md`** — HS-P0020's pinned narrative tree
(`const TREE: &str = "docs";`), its index page, and the composition root a documentation
citation has here. The citation mounts into the page body between the opening paragraph
and the "Looking for / It is at" table (`docs/README.md:12-24`). This is the same file the
router story mounted its *announcement* into; the two edits are different regions
answering different questions, and this spec's Context pack states why one does not
substitute for the other.

**Wires into** (real siblings, by path):

| Consumed | Path | What this story takes from it |
| --- | --- | --- |
| the governed tree's pin | `.bklg/docs-that-teach/checked-documentation-surface/_design.md` (`const TREE: &str = "docs";`) | which tree a "governed page" is in, by value, without declaring a second const |
| the rules tree and its router (**dependency**) | `standards/pages/README.md`, `standards/pages/10-*.md` | the link target of the citation, and the file the back-link is added to |
| the rule that shapes the citing page | `_design.md` `### S1 vocabulary — DT-3` → **RP-10-2**, **RP-10-3** | the reason the citation sentence is true of `docs/README.md` specifically, and the one-sentence ceiling it must itself obey |
| the declaration form | `_design.md` `### S1` (`> **Answers:** \`token\` — question?`) and `## Composition` S1 | where the citation may **not** go, and the declaration the citing page must still carry |
| the pin and the copied link check (**dependency**) | `xtask/src/lint_constitution.rs:212`, `:176`, `:343-356` | why moving the rules tree breaks the build; why the back-link is mechanically checked once the checker lands |
| the announcement already landed | `docs/README.md:12-24`, `:25-29` | the row and the `[PROVISIONAL — settles at …]` marker this PR preserves verbatim and does not duplicate |
| the story-grain selector | `xtask/src/affected.rs:116-125`, `:249-266` | what `cargo xtask affected --base main` does on a two-file prose diff — read, not edited |

**Design-system primitives consumed.** Textual, all from `_design.md`: the
`> **Answers:**` blockquote declaration (present on the citing page, not authored here);
the `docs/README.md` two-column routing table's own voice and relative-link form; the
router's scope paragraph as persistent chrome; inline code for path and rule-id tokens;
markdown link with the rule id as visible text. No new primitive is introduced, and no
bespoke navigation widget is added — the evaluator's gap is *a missing link, not a missing
widget* (`_design.md` anti-pattern 9).

**Renders surfaces.** `page-need-declaration` — **changed, not created**: this story adds
the citation to a page that already carries a declaration, and is accountable for that
surface's `declared-valid` and `plain-text-pager` states remaining true after the edit
(`_design.md` `## Surfaces`). `discipline-router` — **changed**: one sentence in the scope
paragraph, region order and generated region untouched. `rule-atom`,
`lint-terminal-output`, `reviewer-procedure` — not rendered or changed here.

**Public items.** **None.** This project changes no public API (`_design.md`, "Template
sections that do not apply"), and this story changes no Rust at all: `git diff main --
xtask` is empty at merge.

**Conformance rule(s).** None, and this is not adapter-observable. No port, no crate and
no feature is in this diff; `happenstance-testkit`'s suite observes stores. The instrument
that *does* observe half of this story is the checker's copied link half
(`xtask/src/lint_constitution.rs:343-356`), which covers the back-link; the outbound
citation is observed by a one-time procedural confirmation, by the testing brief's
explicit instruction (`_decomposition.md:948-956`).

**Clause(s).** None discharged, none amended. Nothing here writes, restates or renumbers a
`spec/SPECIFICATION.md` clause — the discipline requires pages to *cite*, and
`cargo xtask spec-trace` remains the only writer of that file's generated sections.
`cargo xtask spec-trace` must still be green at merge.

**Advances DoD scenario.** **DoD-14** — *"The discipline is on disk and cited"*
(`.bklg/docs-that-teach/initiative.md:462-464`). `router-precedence-and-announcement`
moved the first two thirds to green (it exists in its decided home; it is reachable from
the repository's index). **This story closes the third and last: at least one page cites
it as the reason it is shaped as it is**, and the discipline is reachable from — and links
back to — the material it governs. With this story merged, DoD-14 is green for
re-observation by HS-P0025 on the assembled tree rather than for first proof.

## PR boundary

**In this PR**

- `docs/README.md` — one added sentence: the citation, below the opening paragraph and
  above the routing table, naming the rule that shapes the page with the rule id as
  visible link text. Nothing existing is reworded or reordered.
- `standards/pages/README.md` — one added link in the scope paragraph naming `docs/` as
  the material these rules govern. Region order unchanged; the `<!-- BEGIN GENERATED -->`
  region untouched; the router still under its 8,192-byte budget.
- If, and only if, the checker's walk excludes `README.md` from the governed set, the
  citation additionally lands on the governed page the walk *does* report — HS-P0020's
  fixture page under `docs/` — per the tie-break decided in the Context pack.
- `.bklg/docs-that-teach/page-need-discipline/governed-page-cites-the-discipline/**` —
  this spec, the `_ledger.md` the second pass defines, and the story's own stage artifacts.

The implementer **may** also touch the composition-root files named in the Integration
contract to mount this slice; that is not scope drift. Here the composition root *is*
`docs/README.md`, and it is already in the list above.

**Explicitly not in this PR**

- **`xtask/src/**` — nothing.** No checker change, no link scanner over `docs/`, no
  `affected.rs` edit. A scanner ranging over both trees is what RS-81-3 forbids
  (`standards/rust/81-checks-that-cannot-be-types.md:209`; architecture brief Note 6), and
  the testing brief already ruled this half procedural.
- **`standards/rust/**` — nothing.** The precedence block stays untouched, as it has since
  `router-precedence-and-announcement`.
- **Teaching content.** No explanation, no tutorial, no how-to, no diagram, no worked
  example on any page in `docs/`. Those are HS-P0022's and HS-P0023's.
- **A generated page-need index in the router** — declined by `_design.md` and not
  reopened here.
- **A second `docs/README.md` table row** for the rules tree — the row already exists, and
  a row is an announcement rather than a citation.
- **`.kb/**` — nothing.** The playbook atom is the slice-mate's, staged under
  `.kb/_intake/`.
- **`.redkiln/templates/**` — nothing**, so `redkiln doctor` still reports exactly six
  `template-drift` advisories, and `redkiln adopt --templates` is never run.
- **The narrative tree's own hosting, build or render**, and any fold/tab demonstration —
  HS-P0020's (DT-7).

```
docs/**
standards/pages/README.md
.bklg/docs-that-teach/page-need-discipline/governed-page-cites-the-discipline/**
```

**Merge DoD, one line.** A governed page under `docs/` names the discipline as the reason
it is shaped as it is, in one sentence, in place, with the rule id as visible link text;
the router names `docs/` back; each direction is one keyboard hop; the citing page still
declares exactly one need and is still inside the checker's walk; `git diff main -- xtask`
and `git diff main -- standards/rust/README.md` are both empty; and `cargo xtask ci
--fast`, `cargo xtask lints`, `cargo xtask spec-trace` and `cargo xtask affected --base
main` are green.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **A governed page names the discipline as the reason for its shape** | One sentence on `docs/README.md`, below the opening paragraph and above the `:12-24` routing table, stating that the page routes and does not teach **because** the discipline says an orientation page must not — with `RP-10-2` as the visible link text. It is a *reason*, not a "see also": the sentence names what about the page the rule determines. | `.bklg/docs-that-teach/page-need-discipline/project.md` AC-011, DR-10; `_decomposition.md` UX-006 (`:504-510`); `_design.md` `### S1 vocabulary — DT-3` |
| **The citer is a page the discipline actually governs** | The citing page lies under HS-P0020's pinned `TREE` (`docs/`) **and** is reported on by the page-need checker's walk. Membership is observed, not assumed: with the citing page's declaration temporarily removed, `cargo run --locked --quiet -p xtask -- lint-pages` names that file and line. That probe is a one-off *membership* observation and is distinct from `declaration-check-seen-to-fail`'s AC-007, which owns the two-needs and unenumerated-need failure demonstrations against `cargo xtask ci`. | `.bklg/docs-that-teach/checked-documentation-surface/_design.md` (`const TREE`); `_decomposition.md` architecture brief AC-006, AC-011; `xtask/src/lint_constitution.rs:212-219` (the `README.md` exclusion in the precedent, which is why this is checked) |
| **The citation obeys the rule it cites** | `docs/README.md` is an `orientation` page: RP-10-2 caps it at links plus at most one sentence per destination and forbids it teaching; RP-10-3 keeps it the only orientation page at its level. The citation is therefore **one sentence**, and the page still declares exactly one need after the edit. A citation that grows into an explanation makes the page answer a second need — the citing page failing the rule it cites, in the same commit. | `_design.md` `### S1 vocabulary — DT-3` (RP-10-2, RP-10-3); `_design.md` `## Density budget`, S1 yield order ("the overflow is the diagnosis") |
| **Cite, never restate** | The sentence names the rule and links it; it reproduces no rule text and no need-set enumeration. A page that explains the discipline is a second copy of the rules tree, and *"one of the two will be stale"*. | `xtask/src/lint_constitution.rs:466-470`; project `project.md` DR-09; `standards/rust/README.md` ("Clause or atom?") |
| **Position is binding, and one position is forbidden** | The citation sits below the first prose paragraph and above the routing table. It may **never** sit between the H1 and the `> **Answers:**` declaration — that region's contents *are* the declaration's meaning — and it is never a footer or an end-of-page note. | `_design.md` `## Composition` S1 ("Nothing may be inserted between the H1 and the declaration"), `## Anti-patterns` 1 and 12 |
| **Persistent chrome, no mechanism** | The citation is visible on first load, in every state, in a plain-text pager as in a rendered page. No `<details>`, no tab, no accordion, no admonition: it states a constraint on the page (never-fold class 3) and `PERMITTED_FOLD_MECHANISMS` ships empty, so the mechanism is forbidden outright. | `_design.md` `## Transience policy`, `### S1/S3 — DT-8` Parts 2 and 3, `## Anti-patterns` 5 and 6 |
| **The rule id is the visible link text** | `RP-10-2` is what the reader sees and clicks — so it is visible that they are being handed to the rule rather than to a paraphrase, and the visible token stays true because rule ids are stable names in a tree whose index is generated from the corpus. | `_decomposition.md` UX brief UX-010 (`:533-541`); `xtask/src/lint_constitution.rs:400-420` (the generated index the ids come from) |
| **The discipline links back to what it governs** | `standards/pages/README.md`'s scope paragraph gains one resolving relative link to `docs/` (`../../docs/README.md`). One link, not an index: a generated index of governed pages was declined by `_design.md` because its source is HS-P0020's tree and it would couple two projects' commit cadence. | `_design.md` `### S2 — discipline-router` ("Rejected", third bullet); `_decomposition.md` UX-006; router spec's Integration contract |
| **The router's composition survives the edit** | The added link lands **inside region 2** (the scope paragraph). No heading is added, no region is reordered, and the `<!-- BEGIN GENERATED -->` / `<!-- END GENERATED -->` region is not touched — so the checker's `--write` still produces no diff. The router stays ≤ **8,192 bytes** and every prose line outside a table stays ≤ **96 columns**. | `_design.md` `## Composition` S2, `## Density budget`; `.bklg/docs-that-teach/page-need-discipline/router-precedence-and-announcement/spec.md` (AC-004, AC-009) |
| **The back-link is mechanically checked; the outbound link is not** | The checker's copied link half resolves each markdown link as `root.join(RULE_DIR).join(target)` and asserts existence, so `../../docs/README.md` is checked from the moment the checker lands — and that check must not be narrowed to targets inside `RULE_DIR`, or this story's correct back-link is reported as dangling. Nothing checks the outbound link at this merge, and the spec says so rather than implying a guarantee. | `xtask/src/lint_constitution.rs:343-356` (the loop), `:349` (the join and existence test); `standards/rust/81-checks-that-cannot-be-types.md:11` (RS-81-1) |
| **A tree move breaks the build, not the link** | `RULE_DIR` and `ROUTER` are `const`s the checker reads; a missing tree is `.with_context()` naming the pinned path, and an empty one is a `bail!`, never a skip. So the citation's target cannot be relocated silently: the gate fails in the same change. This is the property architecture brief AC-011 requires of the link target, and it is observed once (rename the tree, run the gate, capture, revert, re-run green, `git status` clean). | `xtask/src/lint_constitution.rs:212`, `:176`; `_decomposition.md` architecture brief AC-011 (`:130-137`); `docs/README.md:25-29` (why a gate-read tree is pinned by path) |
| **One hop each way, keyboard only** | From the governed page to the governing rule in one navigation step, and from the rule back to the governed material in one. No bespoke navigation widget, no breadcrumb, no sidebar rail is added on top of what the renderer already gives. | `_decomposition.md` UX-006 (`:504-510`); `_design.md` `## Anti-patterns` 9 |
| **No shared scanner over two trees** | This story adds no code. The cross-project half stays a one-time procedural confirmation recorded in `_ledger.md` — a permanent checker ranging over both `standards/pages/` and `docs/` is exactly what RS-81-3 forbids. | `_decomposition.md` testing brief AC-011 (`:948-956`); `standards/rust/81-checks-that-cannot-be-types.md:209`; architecture brief Note 6 |
| **Existing text is preserved, referent-only** | The `docs/README.md` routing table, its gate-read paragraph and its `[PROVISIONAL — settles at …]` marker are unchanged; the router's signed-off regions are unchanged. This PR adds two sentences and rewords nothing. | `.kb/governance/rewrite-the-referent-never-the-reasoning.md`; `docs/README.md:12-24`, `:25-29` |
| **The affected gate is read, not edited** | `docs/` and `standards/pages/` selection is settled by other stories — HS-P0020's `narrative-tree-story-grain-selection` for the first, `page-need-checker-mounted-in-the-gate`'s `INERT` pair for the second — and the page checker runs unconditionally in `affected::run`'s file-reading block regardless. This PR records what `cargo xtask affected --base main` actually does on a two-file prose diff and changes nothing about it. | `xtask/src/affected.rs:116-125`, `:249-266`; `.bklg/docs-that-teach/checked-documentation-surface/_storymap.md` (`narrative-tree-story-grain-selection`) |
| **Nothing else moves** | No `xtask` source, no `standards/rust/` file, no `.kb/` file, no `.redkiln/templates/` file, no teaching content. `redkiln doctor` still reports exactly six `template-drift` advisories. | `CLAUDE.md`; project `project.md` Definition of done |

## Data and migrations

**N/A — no data and no migration.** This story adds two sentences to two markdown files. It
touches no schema, no serialised format, no wire envelope, no stored state, and no Rust:
`git diff main -- xtask` is empty at merge, so nothing in this diff is compiled or run.

Three adjacent obligations that are *not* migrations but are the closest this story has to
one, each already stated above and each owned by a named place rather than by a script:

- **The `TREE` / `RULE_DIR` spellings are consumed by value, never re-declared.** If either
  tree is later renamed, the rename is a `const` edit in `xtask/src/` plus these two
  sentences, in one change — which is the whole point of pinning by path
  (`docs/README.md:25-29`).
- **The back-link's resolution semantics are a forward contract on the checker story**, not
  a data format: the copied link half must resolve relative to the router's own directory.
  Narrowing it to `RULE_DIR`-internal targets turns this story's correct link into a
  reported problem.
- **The citation is a claim with a named retirement condition**, not a TODO. If HS-P0022's
  teaching pages later carry their own citations, this one is not removed — the bar is "at
  least one", and removing the orientation page's citation would silently reduce the
  discharge to whatever the newest page happens to say.

## Acceptance criteria

Each criterion is a reader's goal crossing the whole stack — the page, the rule, the gate
and the pin — not a capability. The three readers are the initiative's own personas
(`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md`): the
**application author** (persona 1), the **adapter author** (persona 2, whose `E0034`
journey is the measured defect this story answers), and the **evaluator** (persona 3, whose
gap is *"good until the second question, and then nowhere to go"*).

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** the adapter author, who has landed on `docs/README.md` because it is the repository's documentation index and who is asking why the page hands them onward instead of explaining anything, **WHEN** they read the page top to bottom with nothing clicked, **THEN** one sentence between the opening prose paragraph and the "Looking for / It is at" table states that the page routes and does **not** teach *because* the page discipline says an `orientation` page must not — naming what about the page the rule determines, not merely that a rule exists — with **`RP-10-2` as the visible link text**, **AND** following that link reaches the rule in **one** navigation step, keyboard only. | *Mechanical, outputs pasted into `_ledger.md`:* `rg -n 'RP-10-2' docs/README.md` → ≥ 1; the visible-text form asserted by `rg -n '\[`RP-10-2`\]\(' docs/README.md` → 1 hit (the id inside the link text, not only in the target); the link target resolves — `test -f standards/pages/10-the-need-set.md` (or whichever band-10 atom the router's index names for RP-10-2, captured by path). *Procedural (ledger):* the UX-006 traverse — from `docs/README.md`, Tab to the citation link, Enter, land on the rule, **one** step, no mouse; recorded as `page → link → file`. *Procedural (ledger), non-author read-back:* a person who did not write the sentence states, from the sentence alone, which property of the page the rule dictates. A "see also" that names no property fails. |
| AC-002 | **GIVEN** the non-author reviewer, who must be able to say the citation is not decorative, **WHEN** they ask whether the citing page is one the discipline actually governs, **THEN** the citing page is shown to lie under HS-P0020's pinned `TREE` (`docs/`) **and** to be reported on by the page-need checker's own walk — observed, never assumed — **AND** if the walk excludes `README.md`, the pre-decided tie-break has fired and the citation additionally sits on the governed page the walk *does* report, with `_ledger.md` recording which page carries it and why. | *Procedural, three captures verbatim in `_ledger.md`, in this order.* (1) With the citing page's `> **Answers:**` declaration temporarily deleted, `cargo run --locked --quiet -p xtask -- lint-pages` prints a problem **naming that file** — this is the membership observation and nothing else. (2) `git checkout -- docs/README.md` and re-run → green, with the `{n} pages, {m} rules, all consistent` line. (3) `git status` → clean. Distinct from `declaration-check-seen-to-fail`'s AC-007, which owns the two-needs and unenumerated-token demonstrations against `cargo xtask ci`; this is a one-off *membership* probe. Precedent that makes it necessary: `xtask/src/lint_constitution.rs:212-219` (`name != "README.md"`). |
| AC-003 | **GIVEN** the same reviewer one step later, asking the only question that can retire the whole story — *does the citing page still obey the rule it cites?* — **WHEN** they run the gate and read the page, **THEN** the page still declares **exactly one** need (`orientation`), the citation is **one sentence**, it teaches nothing and restates no rule text, `docs/` still holds at most one `orientation` page (RP-10-3), and the added source line is ≤ **96 columns** — so the page does not fail, in the same commit, the rule it has just named. | *Mechanical:* `cargo xtask ci` green, the `every page declares one need` step included — that step is what asserts the single declaration and the per-directory `orientation` count. `rg -c '^> \*\*Answers:\*\*' docs/README.md` → `1`. `awk '!/^\|/ && length > 96 {print FILENAME":"FNR": "length}' docs/README.md` → no output. Sentence count: the added block is one sentence — captured by pasting the added lines into `_ledger.md` verbatim, not by claiming a count. *Procedural (ledger):* the paraphrase spot check of `reviewer-and-citation-procedures` run over this one sentence — it names the rule and links it, and reproduces none of RP-10-2's wording (`xtask/src/lint_constitution.rs:466-470`, "one of the two will be stale"). |
| AC-004 | **GIVEN** the evaluator standing on `standards/pages/README.md`, who has read the rules and is asking the reciprocal question — *what material do these rules actually govern?* — **WHEN** they read the scope paragraph, **THEN** it carries **one** resolving relative link naming `docs/` as the governed material, reachable in one keyboard hop; **AND** the router's composition survives: the binding region order is unchanged, no heading is added, the `<!-- BEGIN GENERATED -->` region is byte-identical, the file is still ≤ **8,192 bytes**, and no index of governed pages has appeared. | *Mechanical:* `rg -n 'docs/README.md' standards/pages/README.md` → exactly 1, inside the scope paragraph. `wc -c < standards/pages/README.md` → ≤ `8192`. `rg -n '^#{1,2} ' standards/pages/README.md` prints the same heading list, in the same order, as on `main` (diff the two captures). `git diff main -- standards/pages/README.md` shows one hunk, inside region 2, and no change between the generated markers. `cargo run --locked -p xtask -- lint-pages` green — its copied link half resolves every markdown link as `root.join(RULE_DIR).join(target)` and asserts existence (`xtask/src/lint_constitution.rs:343-356`, join at `:349`), so the back-link is mechanically checked here. `cargo run --locked -p xtask -- lint-pages --write` → **no diff**. *Procedural (ledger):* the reverse UX-006 traverse, keyboard only, one hop. |
| AC-005 | **GIVEN** any of the three readers opening the page in a plain-text pager with no renderer, **WHEN** the page first loads, **THEN** the citation is **composed presentation in the page's own textual grammar** — a body sentence carrying the rule id as inline code inside a markdown link, in the voice of the surrounding prose — and it sits **below the first prose paragraph and above the routing table**; it is **not** a bare URL, a badge, an admonition block, an HTML element or an appended "See also:" stub, it is **not** in a footer or an end-of-page note, and the region between the H1 and the `> **Answers:**` declaration is **exactly one blank line and nothing else**. | *Mechanical:* capture the file's first 20 lines into `_ledger.md`; assert line 2 is blank and the next non-blank line is the `> **Answers:**` declaration (`_design.md` `## Composition` S1). Assert ordering by line number: `citation_line > first_prose_line` and `citation_line < first_table_line`, all three captured from `rg -n`. `rg -n '<[a-z]|^\s*!\[|^> \[!' docs/README.md` → no HTML element, no badge image, no admonition. `rg -n 'https?://' docs/README.md` → no bare URL added. *Procedural (ledger):* read the file through `less` and confirm the sentence reads as prose, not as chrome. Anti-patterns refused: 1 and 12 (`_design.md` `## Anti-patterns`). |
| AC-006 | **GIVEN** the evaluator reading in a pager, printing the page, and reading it in greyscale, **WHEN** nothing has been clicked, **THEN** both added sentences are visible in **every** state — persistent chrome, never behind a `<details>`, a tab, an accordion or an inactive panel — nothing added is a bespoke navigation widget, no meaning is carried by colour, an icon or size, and nothing added can move focus, scroll or selection, because the only interactive thing either PR touches is a plain markdown link. | *Mechanical:* `rg -n '<details>\|<summary>\|role="tab"\|\{\{#tab' docs/README.md standards/pages/README.md` → no matches. `rg -n '<small>\|<sub>\|<sup>\|<nav>\|<img' docs/README.md standards/pages/README.md` → no matches. `rg -n '[🟢🔴⚠️✅]' docs/README.md standards/pages/README.md` → no matches (no icon standing in for a word). *Procedural (ledger):* the plain-pager read and a greyscale print/read of both pages, recorded. Basis: the citation states a constraint on the page — never-fold class 3 — and `PERMITTED_FOLD_MECHANISMS` ships empty, so the mechanism is forbidden outright (`_design.md` `### S1/S3 — DT-8` Parts 2 and 3; `## Anti-patterns` 2, 3, 5, 6, 9). |
| AC-007 | **GIVEN** the maintainer who relocates `standards/pages/` in some future refactor and does **not** touch `xtask/src/`, **WHEN** they run the gate, **THEN** the build fails in the same change — `.with_context()` naming the pinned path, or the vacuity `bail!` — rather than the citation rotting quietly into a dead link; **AND** moving the tree back returns the gate to green with no residue. | *Procedural, both directions captured verbatim in `_ledger.md`.* (1) `git mv standards/pages standards/pages-moved`; `cargo xtask ci` → fails, the message naming the pinned path, pasted verbatim. (2) `git mv standards/pages-moved standards/pages`; `cargo xtask ci` → green. (3) `git status` → clean. Evidence for why this is the property architecture brief AC-011 asks the link target to have: `xtask/src/lint_constitution.rs:212` (the `.with_context()`), `:176` (the vacuity `bail!`), and `docs/README.md:25-29` (why a gate-read tree is pinned by path). |
| AC-008 | **GIVEN** the contributor who assumes that because this repository has a gate, the gate is watching this citation, **WHEN** they read `_ledger.md` and the router's `## What checks this tree, and what does not` section, **THEN** they are told plainly that at this merge the **back-link is mechanically checked** by the router's copied link half while the **outbound link from `docs/README.md` is not checked by anything**, and that the cross-project confirmation is a one-time procedural record — **AND** no shared scanner ranging over both trees was built to close that gap. | *Gate-state:* `git diff main -- xtask` → **empty**, output pasted into `_ledger.md`; nothing inside this diff can prove another tree was untouched except the diff itself. *Mechanical:* `rg -n 'What checks this tree, and what does not' standards/pages/README.md` → 1 hit (landed by `router-precedence-and-announcement`, read here, not edited). *Procedural (ledger):* a blind-spot row stating the asymmetry in one sentence, plus a non-author read-back confirming they can state which half is checked. Bar and prohibition: RS-81-1 (`standards/rust/81-checks-that-cannot-be-types.md:11`, a check whose limits are undocumented reads as a guarantee) and RS-81-3 (`:209`, no scanner over two trees); testing brief AC-011 (`_decomposition.md:948-956`). |

**Project-AC coverage.** Project **AC-011** — *the discipline is cited by what it governs* —
is discharged by this story's half in full: **AC-001** (named in place, as a reason, with the
rule id visible), **AC-002** (by a page it actually governs), **AC-004** (and reachable back
from the discipline), with **AC-003**, **AC-005**, **AC-006** keeping the citation honest and
legible, **AC-007** keeping the link's target immovable-in-silence, and **AC-008** keeping the
guarantee's edge stated rather than implied. The other half of project AC-011 — *announced in
the repository's index* — is `router-precedence-and-announcement`'s and is already merged.
Initiative **DoD-14**'s third and last clause closes here.

## Interaction quality

Every invariant below is carried by an **AC row in the table above**, never by a bullet here:
this section says which id carries what, and how it is verified. That matters more than usual
in this project, because there is no perceptual review to catch what the assertions miss —
`design.capture` is deliberately absent from `.redkiln/config.yaml`, so `_design.md` plus
these rows are the only instrument.

**State invariants** (RFC §6.7/D6).

| invariant | carried by | how it is verified |
| --- | --- | --- |
| **In place, not a context jump** — the reason is on the page the reader is already standing on | **AC-001**, reinforced by **AC-005** | the ordering assertions in AC-005 and the non-author read-back in AC-001. The forbidden shape is a citation that only *points at* the discipline from a footer or a second file, which is the measured defect (`personas-and-journeys.md:174-181`) reproduced |
| **Non-occlusion** — nothing the reader needs is hidden behind an interaction | **AC-006** | the `<details>` / tab / accordion greps over **both** edited files. Anti-patterns 5 and 6; the citation is never-fold class 3 |
| **Preserved focus, scroll and selection** | **AC-006** | discharged by **removing the mechanism**: neither added sentence introduces an authored disclosure or any script, so nothing exists that *can* move focus or scroll. AC-006's greps are what keep it that way |
| **Reversibility** | **AC-002** (declaration removed → probe → `git checkout --` → green → clean `git status`), **AC-007** (tree moved → gate fails → moved back → green → clean `git status`) | both walks are run in both directions and both are recorded. Nothing this PR adds leaves a cache, a generated artifact or a dirty file |
| **Keyboard reachability** | **AC-001** (governed page → rule, one hop), **AC-004** (rule → governed material, one hop) | the two recorded traverses, run keyboard only, which is UX-006's own test wording (`_decomposition.md:504-510`) |

**Composition invariants**, taken from the signed-off
`.bklg/docs-that-teach/page-need-discipline/_design.md`. This story **changes** two surfaces it
did not create — `page-need-declaration` on `docs/README.md` and `discipline-router` — and is
accountable for both remaining true after the edit.

| invariant | carried by | how it is verified |
| --- | --- | --- |
| **Presentation exists at all** — the citation is real composed presentation in the repo's own textual grammar (a body sentence, inline-code rule id inside a markdown link, in the surrounding prose's voice), not bare markup, a bare URL or an appended stub | **AC-005** | the HTML/badge/bare-URL greps plus the plain-pager read. An unstyled render passes every ordering assertion; this row is what fails a `See also: standards/pages/` stub that satisfies them |
| **Composition and placement** — below the first prose paragraph, above the routing table; **never** between the H1 and the declaration; the router's link inside region 2 with no region moved | **AC-005** (governed page), **AC-004** (router) | the three line-number comparisons and the first-20-lines capture in AC-005; the heading-order capture and single-hunk `git diff` in AC-004. `_design.md` `## Composition` S1 and S2 |
| **Transience** — both additions are **persistent chrome**; the only opened-on-demand control either page has is a link to another file, which is the one intended context switch | **AC-006** | the fold-mechanism greps. `_design.md` `## Transience policy`: `PERMITTED_FOLD_MECHANISMS` is empty, so "eligible to fold" and "permitted to fold" are different sets and the second is empty |
| **Density budget, with its real numbers** — the citation is **one sentence** on a page whose rule caps it at one sentence per destination; the added source line is ≤ **96 columns**; the router stays ≤ **8,192 bytes** | **AC-003** (the sentence and the column ceiling), **AC-004** (the router's bytes) | `awk` for the column count, `wc -c` for the router, and the verbatim paste of the added lines. `_design.md` `## Density budget` — and its yield order: if the sentence will not fit, the diagnosis is that **the page is answering a second need**, not that the sentence needs trimming |
| **Hierarchy** — the rule id is the primary token of the citation, carried by inline-code monospace and by being the link text; never by colour, an icon or size | **AC-001** (the visible-link-text assertion), **AC-006** (the `<small>`/`<sub>`/icon greps) | `_design.md` `## Hierarchy` ("Colour is never a carrier") and UX-010 (`_decomposition.md:533-541`) |
| **Named anti-patterns refused** — **1** (anything between the H1 and the declaration), **2**/**3** (a need or a citation shown as colour, icon or shrunken text), **5** (a constraint invisible until clicked), **6** (any `<details>`/tab/accordion at all), **9** (a bespoke navigation widget), **12** (a reason in a footer rather than where the reader is) | 1, 12 → **AC-005**; 2, 3, 5, 6, 9 → **AC-006** | the greps and captures named in each AC. Anti-pattern 12 is the one with a *positional* test, which is why AC-005 compares line numbers rather than asserting presence |

## Error conditions

| id | condition | required handling |
| --- | --- | --- |
| **EC-001** | The page-need checker's walk **excludes** `README.md` from the governed set, inheriting the precedent's `name != "README.md"` (`xtask/src/lint_constitution.rs:212-219`), so `docs/README.md` is not a page the discipline governs. | **The pre-decided tie-break fires, and it is not re-litigated at implementation time.** The citation additionally lands on the governed page nearest the reader that the walk *does* report — HS-P0020's fixture page under `docs/` — and `_ledger.md` records which page carries it, with AC-002's probe re-run against that page. Do **not** "fix" this by editing the checker's walk: `xtask/src/**` is out of scope, and widening the governed set is `page-need-checker-mounted-in-the-gate`'s decision, not this story's. |
| **EC-002** | The citation is written as a **second row** in `docs/README.md`'s "Looking for / It is at" table. | **It discharges nothing and must be reverted.** A row says *the tree exists and is over there*; DoD-14 and UX-006 ask for *this page looks like this because of that rule*. The row for the rules tree already exists (`router-precedence-and-announcement`, AC-007); duplicating it also breaks that story's mechanical `rg` count. |
| **EC-003** | The citation grows past one sentence — a paragraph explaining what the discipline is, what the need set contains, or why the rule exists. | **AC-003 fails, and so does the page.** `docs/README.md` is an `orientation` page: RP-10-2 caps it at links plus one sentence per destination and forbids it teaching, so an explanatory citation makes the page answer a second need and the gate's own `every page declares one need` step becomes the thing rejecting it — the citing page failing the rule it cites, in the same commit. Cite and link; never restate (`xtask/src/lint_constitution.rs:466-470`). |
| **EC-004** | The checker story's copied link half is narrowed to "targets inside `RULE_DIR`", or is given a `..`-rejecting guard. | **This story's correct back-link is then reported as dangling and the gate fails on a true link.** The inherited semantics resolve `root.join(RULE_DIR).join(target)` relative to the router's own directory (`:343-356`, join at `:349`), and `../../docs/README.md` resolves correctly under them. This is a **forward obligation on `page-need-checker-mounted-in-the-gate`**, recorded here because this is the story whose diff breaks; if that story has already narrowed it, the narrowing is reverted there, not worked around here. |
| **EC-005** | The back-link grows into a **generated index** of governed pages and their declared needs. | **Declined at design time and not reopened.** Its source is HS-P0020's tree, so the rules tree would change on every page addition, coupling two projects' commit cadence (`_design.md` `### S2 — discipline-router`, "Rejected"). One link is reachability; an index is coupling. If a reviewer asks for it, the answer is a link to that decision, not a compromise. |
| **EC-006** | The back-link is added as a **new heading or new region** ("## What this governs"), or pushes the router past 8,192 bytes. | **AC-004 fails.** The router's region order is signed off (`_design.md` `## Composition` S2) and a seventh region reorders nothing but adds a heading the design does not have. The link belongs in region 2, the scope paragraph. If the budget bites, apply the stated yield order — merge `Start here` rows, then shorten the scope paragraph — never trim the index. |
| **EC-007** | The citation is inserted **between the H1 and the `> **Answers:**` declaration**, because that is where a "this page is like this because…" sentence intuitively wants to go. | **Anti-pattern 1, and the highest-value failure in this PR to catch.** That region's contents *are* the declaration's meaning — UX-001's test is "read nothing but the region above the first prose paragraph and name the need", and any interposed element makes it ambiguous. AC-005's first-20-lines capture is the guard; run it before opening the PR. |
| **EC-008** | A link scanner over `docs/` is written to "make the outbound half checked too". | **Out of scope and forbidden.** RS-81-3 scopes a scanner to the directory whose behaviour it constrains (`standards/rust/81-checks-that-cannot-be-types.md:209`; architecture brief Note 6), and the testing brief already ruled this half a one-time procedural confirmation (`_decomposition.md:948-956`). `git diff main -- xtask` empty is AC-008's guard. The honest response to the gap is AC-008's stated blind spot, not a second scanner. |

## Non-functional

| id | requirement | why, and how it is observed |
| --- | --- | --- |
| **NF-001** | **No Rust, no dependency change, no manifest change.** | `git diff main -- xtask '**/Cargo.toml' Cargo.lock` → empty. This PR is two sentences in two markdown files; nothing in it is compiled or run. |
| **NF-002** | **LF line endings, UTF-8, no BOM**, on both edited files. | This is a Windows checkout, and the router's generated region is compared for string equality by `check_router`'s analogue — a CRLF interior line is not equal to an LF one, which would make the checker's `--write` produce a whole-region diff and fail AC-004. Observe with `git diff --check` and by inspecting the added lines. |
| **NF-003** | **Non-ASCII characters match the corpus's existing set** — the em dash `—` only. No smart quotes, no non-breaking spaces, no ellipsis character. | Two files rendering the same idiom two ways is a second grammar. `rg -n '[“”‘’… ]' docs/README.md standards/pages/README.md` → nothing beyond what `main` already carries. |
| **NF-004** | **Every claim in `_ledger.md` is a captured command output or a captured walk, never a paraphrase of one.** | The project's testing brief makes procedural evidence first-class (`.redkiln/config.yaml`, `require_ledger: true`), which only holds if what is recorded is the output. A row reading "checked, resolves" without the command's output is not evidence. |
| **NF-005** | **No gate step is added, and gate wall-clock does not regress from this PR.** | Nothing here is a new `Step`; `cargo xtask ci`'s step list is byte-identical to `main`'s. Record `cargo xtask affected --base main`'s behaviour on this two-file prose diff as observed, and change nothing about it (`xtask/src/affected.rs:116-125`, `:249-266`). |
| **NF-006** | **`redkiln doctor` still reports exactly six `template-drift` advisories.** | `.redkiln/templates/**` is out of scope, and the `backlog` CI job asserts the set is exactly those six (`CLAUDE.md`). A seventh or a fifth means a template moved in this diff. |
| **NF-007** | **`cargo xtask spec-trace` green and `spec/` unmodified.** | Nothing here writes, restates or renumbers a clause — the discipline requires pages to *cite*. `git diff main -- spec/` → empty. |
| **NF-008** | **No `.kb/` write of any kind**, including `.kb/_intake/`. | The playbook atom is the slice-mate's (`playbook-atom-staged-for-ingest`, AC-012). `git diff main -- .kb` → empty **for this story's own commits**; the slice-mate's staged file is its own evidence and is not conflated with this one's. |
| **NF-009** | **`standards/rust/README.md` stays untouched.** | Its unedited precedence block is still `router-precedence-and-announcement`'s AC-002 evidence one slice back; a friendly reciprocal link added here would silently invalidate a satisfied ledger row in another story. `git diff main -- standards/rust/README.md` → empty. |

## Implementation notes (non-prescriptive)

Not instructions — the shape the Context pack's decisions already imply, offered so the
implementer spends their judgement on the two sentences rather than on rediscovering the
constraints.

- **Run AC-002's membership probe first, before writing a word.** Everything downstream
  depends on which page is the governed one. If the walk does not report `docs/README.md`,
  EC-001's tie-break changes where the citation lands, and discovering that after the
  sentence is written means writing it twice.
- **Write the citation as a sentence about *this page*, not about the discipline.** The test
  the non-author read-back applies is whether a reader can name the property of the page the
  rule determines. "This page routes and does not explain, because `RP-10-2` says an
  orientation page must not" passes. "These pages follow the page standards" names no
  property and fails, while satisfying every grep.
- **Two spellings are pins typed by hand exactly once and then copied:** the band-10 atom's
  filename (the citation's target) and `../../docs/README.md` (the back-link). A typo in
  either is not a typo — it is a link the gate will report, or worse, one nothing reports.
- **Derive the back-link's relative path from the router's own directory, not from the repo
  root.** The checker's copied link half joins `RULE_DIR` to the target; `standards/pages/` is
  two levels down, so `../../docs/README.md` is what resolves. Verify it with `test -f` from
  inside `standards/pages/` before committing, not after the gate says so.
- **A useful order of work:** membership probe → the citation sentence → AC-005's first-20-
  lines capture (catch EC-007 immediately) → the back-link → `lint-pages` and `--write` →
  the two traverses → AC-007's move/restore walk → the boundary `git diff`s → paste
  everything into `_ledger.md` as you go. Collecting captures at the end is how they get
  paraphrased.
- **Do not soften AC-008's blind-spot row.** The temptation, having just watched the back-link
  get checked, is to write "the links are checked". Half of them are. RS-81-1's whole subject
  is that a check whose limits are undocumented reads as a guarantee
  (`standards/rust/81-checks-that-cannot-be-types.md:11`), and this project's thesis dies on
  that sentence.
- **Resist the reciprocal-link instinct in `standards/rust/README.md`.** It is small, friendly,
  and it breaks a satisfied ledger row one slice back (NF-009, and the router story's EC-006).
- **Rewrite the referent, never the reasoning.** Both files already discharge something. Add;
  do not reword, reorder, or "tidy while I am here"
  (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`).

## Tests and CI (merge gate)

Tiers as the project's testing brief defines them
(`.bklg/docs-that-teach/page-need-discipline/_decomposition.md`, Testing brief): **static**
(`#[cfg(test)]`), **gate-integration**, **end-to-end/fixture**, **procedural
(ledger-recorded)**. This story is weighted to the last two by the brief's own instruction
that the cross-project half stays procedural (`:948-956`).

| tier | command / path | proves |
| --- | --- | --- |
| **static** | *(none in this PR — by construction)* | This PR touches no `xtask/src/**`, so it can carry no `#[test]`. The one permanent assertion its ACs imply — that the router's link half resolves targets **outside** `RULE_DIR` correctly — is inherited by `page-need-checker-mounted-in-the-gate` and recorded as EC-004 rather than dropped. |
| **gate-integration** | `cargo run --locked --quiet -p xtask -- lint-pages` | **AC-002** (the membership probe, run with the declaration removed and again after revert), **AC-003** (one declaration, one `orientation` page in `docs/`), **AC-004** (the back-link resolves under the copied link half, `xtask/src/lint_constitution.rs:343-356`). |
| **gate-integration** | `cargo run --locked -p xtask -- lint-pages --write` | **AC-004.** No diff — the router's generated region is untouched by this PR, so `--write` must still be a no-op. |
| **gate-integration** | `cargo xtask lints` | The file-reading lint family is green and unchanged; `lint-constitution` still passes over `standards/rust/`, proving this story did not perturb the tree it does not touch (`verify.reachability_static`). |
| **gate-integration** | `cargo xtask spec-trace` | **NF-007.** The specification's markers and citations still resolve — load-bearing precisely because this project's deliverable is files the compiler never reads. |
| **gate-integration** | `cargo xtask ci --fast` | The bar a non-terminal project is held to (`.redkiln/config.yaml`, `verify.integration_scoped`; `project.md` Definition of done). Green at merge. |
| **gate-integration** | `cargo xtask ci` | The merge gate of record (`CLAUDE.md`, "Commands"), and the command **AC-003** is observed under and **AC-007** is observed failing and recovering under. |
| **gate-integration** | `cargo xtask affected --base main` | The story grain (`verify.affected_gate`). Green; its behaviour on a two-file prose diff is recorded as observed and nothing about it is edited (NF-005). |
| **gate-state** | `git diff main -- xtask` | **AC-008**, and the PR boundary's largest promise: no checker change, no scanner, no `affected.rs` edit. Empty. |
| **gate-state** | `git diff main -- standards/rust/README.md spec .kb .redkiln/templates '**/Cargo.toml' Cargo.lock` | **NF-001, NF-006, NF-007, NF-008, NF-009** in one command. Empty. |
| **mechanical (procedural, captured)** | the `rg` / `awk` / `wc` / `test -f` / `sed` invocations named in the acceptance table | **AC-001, AC-003, AC-004, AC-005, AC-006, AC-008.** Each command's *output* — never a claim about it — enters `_ledger.md` (NF-004). |
| **procedural (ledger-recorded)** | the two one-hop keyboard traverses (AC-001, AC-004); the non-author read-back of the citation (AC-001); the paraphrase spot check (AC-003); the plain-pager and greyscale reads (AC-005, AC-006); the blind-spot read-back (AC-008) | The judgements no command carries. `.redkiln/config.yaml`'s `require_ledger: true` treats a recorded manual verification as first-class proof, which is why the testing brief made this a named tier rather than an excuse. |
| **end-to-end/fixture (reversibility walks)** | AC-002's declaration removal → probe → `git checkout --` → green → `git status`; AC-007's `git mv` → `cargo xtask ci` fails → `git mv` back → green → `git status` | **AC-002, AC-007.** Both are run in both directions and all captures are verbatim. These are the only two places in this story where something is deliberately broken, and neither overlaps `declaration-check-seen-to-fail`'s AC-007 demonstrations. |

**Merge gate, one line.** `cargo xtask ci` green; `cargo xtask affected --base main` green;
both boundary `git diff`s empty; every mechanical measurement captured; both reversibility
walks recorded in both directions; every procedural walk in `_ledger.md` with cited evidence.

## Risks and coupling (PR-scoped)

| Risk | Coupling it runs through | Mitigation in this PR |
| --- | --- | --- |
| **The citing page turns out not to be governed.** If the checker's walk inherits the precedent's `README.md` exclusion, the whole story discharges nothing while looking complete. | `page-need-checker-mounted-in-the-gate` → this story, by the walk's membership semantics (`xtask/src/lint_constitution.rs:212-219`) | AC-002 makes membership an **observation**, run first (Implementation notes), and EC-001 pre-decides the tie-break so the implementer does not negotiate scope mid-PR. The ledger records which page carries the citation either way. |
| **The checker story narrows its copied link half** to targets inside `RULE_DIR`, and this story's correct back-link is reported as dangling. | `xtask/src/lint_constitution.rs:343-356` → the checker story → this story's `standards/pages/README.md` diff | EC-004 states it as a **forward obligation** on that story, names the join at `:349` as the semantics to preserve, and puts the fix there rather than here. AC-004 runs `lint-pages` in this PR, so the breakage surfaces at this merge rather than later. |
| **The citation grows into an explanation** — the most natural failure in the whole project, because explaining is what a documentation initiative rewards. | RP-10-2 → `docs/README.md`'s own declaration → the gate's `every page declares one need` step | EC-003 names it; AC-003 makes the page's single declaration a gate assertion rather than a reviewer's impression; the design's yield order supplies the diagnosis — the overflow means the page is answering a second need, not that the sentence is too long (`_design.md` `## Density budget`, S1). |
| **The citation lands between the H1 and the declaration.** Small, intuitive, and it breaks the surface this project's foundation story exists to establish. | `_design.md` `## Composition` S1 → the `page-need-declaration` surface on a page this story only *changes* | EC-007 plus AC-005's first-20-lines capture, which is cheap enough to run before the sentence is even final. Anti-pattern 1 is screenshot-checkable by someone who cannot read the code. |
| **A reviewer asks for a generated index of governed pages** in the router, because it is obviously more useful. | `standards/pages/README.md` ← HS-P0020's tree; two projects' commit cadence | EC-005 answers with the design's own recorded rejection rather than a compromise. One link is reachability; an index makes the rules tree change on every page addition. |
| **"The links are checked" gets written into the ledger.** Half of them are, and the half that is not is the one the story is named after. | AC-008 ← RS-81-1 (`81-checks-that-cannot-be-types.md:11`) | AC-008 makes the blind-spot statement a **criterion**, with a non-author read-back, so the asymmetry has to be stated in someone else's words before the row can be flipped. EC-008 forbids closing the gap with a second scanner. |
| **This PR is prose, so the affected gate widens or narrows unexpectedly** and someone "fixes" `affected.rs`. | `xtask/src/affected.rs:116-125`, `:249-266`; `docs/` is already on `INERT` | NF-005 records the behaviour as observed; the PR boundary puts `xtask/src/**` out of scope; AC-008's `git diff main -- xtask` makes a stray edit a merge blocker. |
| **The retirement condition is misread as a TODO.** A later story adds a teaching-page citation and removes this one as redundant. | HS-P0022 → `docs/README.md` | The Data and migrations section states it: the bar is "at least one", and removing the orientation page's citation silently reduces the discharge. A second citer is an **addition**, never a replacement. |

## Dependencies

**Blocks on** (both must be merged before this story can be implemented):

- **`router-precedence-and-announcement`** — supplies the link's *target*: the rules tree at
  `standards/pages/` with a router at a stable path, its band-10 atom carrying RP-10-2, and
  the region-2 scope paragraph this story's back-link is added to. It also landed the
  `docs/README.md` table row and the `[PROVISIONAL — settles at …]` marker this PR preserves
  verbatim, and its own spec states plainly that the remaining third of DoD-14 is this
  story's. Without it, the citation has nothing that resolves to link to.
- **`page-need-checker-mounted-in-the-gate`** — supplies the *instrument*: the walk whose
  report defines what "a page the discipline governs" means (AC-002), the copied link half
  that mechanically checks the back-link (AC-004), and the `.with_context()`-plus-`bail!`
  pinning that makes AC-007's move/restore walk observable. Without it, AC-002 has nothing to
  probe and AC-004's back-link is unchecked by anything.

**Unlocks.** No story in this project declares `depends_on` this one — the slice-mate
`playbook-atom-staged-for-ingest` depends on `fold-line-rule`,
`reviewer-and-citation-procedures` and `declaration-check-seen-to-fail`, not on this story
(`.bklg/docs-that-teach/page-need-discipline/_storymap.md`). What this story unlocks is
downstream of the project: **HS-P0025 `durable-audience-closeout`**, which re-observes DoD-14
on the assembled tree rather than proving it for the first time, and **HS-P0022**'s teaching
pages, which inherit a citation form already demonstrated to survive its own rule.

**Slice-mate** (implemented in one context, mounted as one surface):
`playbook-atom-staged-for-ingest` — the `binding-beyond-this-project` milestone. The two are
grouped because both reach outside this project's own diff, and neither can be observed while
the discipline is still moving.

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. Each row says **why** it is load-bearing and
**when** to open it, and is bound to the AC it serves. Every path was confirmed to exist in
this worktree.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/page-need-discipline/_design.md` | The **binding** signed-off design. `### S1 vocabulary — DT-3` is where RP-10-2/RP-10-3 are actually stated — the rule the citation names and the ceiling it must itself obey; `## Composition` S1/S2 fix the two positions; `## Transience policy` and `## Anti-patterns` 1, 5, 6, 9, 12 are the failure list. It is not re-decided here. | Before writing either sentence, and again before running AC-005's capture. | AC-001, AC-003, AC-004, AC-005, AC-006 |
| `.bklg/docs-that-teach/page-need-discipline/_decomposition.md` | Carries the three briefs' own words: architecture AC-011 (`:130-137`, the two mounts and "a tree move breaks the build rather than the link") and Note 2 (one path constant, not two); UX-006 (`:504-510`) and UX-010 (`:533-541`); testing AC-011 (`:937-956`) with the explicit instruction that the cross-project half stays procedural. | Before AC-004's back-link (Note 2), and before writing any ledger row that describes what is checked (testing AC-011). | AC-001, AC-004, AC-007, AC-008 |
| `xtask/src/lint_constitution.rs` | The precedent every mechanical claim in this spec rests on: `:212` `.with_context()` and `:176` the vacuity `bail!` (why a tree move breaks the build); `:212-219` the `name != "README.md"` exclusion (why membership is probed, not assumed); `:343-356` with the join at `:349` (why the back-link is checked and must not be narrowed); `:466-470` (why a restatement goes stale). | Open `:212-219` before AC-002's probe; `:343-356` before writing the back-link's relative path. | AC-002, AC-004, AC-007, AC-008 |
| `.bklg/docs-that-teach/page-need-discipline/page-need-checker-mounted-in-the-gate/spec.md` | The dependency that defines the walk, the problem-line shape AC-002's probe will print, and the `--write` no-diff obligation AC-004 re-runs. EC-004's forward obligation is owed **by** this spec. | Immediately before AC-002's probe and AC-004's `lint-pages` runs, to know what green and what a problem line look like. | AC-002, AC-004 |
| `.bklg/docs-that-teach/page-need-discipline/router-precedence-and-announcement/spec.md` | The other dependency: the router's authored regions and binding order, the 8,192-byte budget, the `docs/README.md` row and marker this PR must preserve, and its own statement that this story owns DoD-14's last third. Its AC-002 is the reason NF-009 exists. | Before touching `standards/pages/README.md` at all, and before any edit near `docs/README.md:12-29`. | AC-004, AC-008 |
| `.bklg/docs-that-teach/checked-documentation-surface/_design.md` | HS-P0020's `## Signatures`, where `const TREE: &str = "docs";` is pinned. This story consumes that spelling by value and declares nothing; it is also what makes "a governed page" a decidable question. | Before AC-002, to confirm the tree's spelling has not moved since this spec was written. | AC-002 |
| `docs/README.md` | The mount point itself, and its current state: the opening paragraph, the `:12-24` routing table, and the `:25-29` gate-read paragraph whose reasoning ("moving either tree means editing `xtask/src/` in the same change") is the argument AC-007 observes. | First — read the whole file before adding a line to it. | AC-001, AC-003, AC-005, AC-007 |
| `standards/rust/81-checks-that-cannot-be-types.md` | RS-81-1 at `:11` (a check whose limits are undocumented reads as a guarantee) and RS-81-3 at `:209` (no scanner ranging over two trees). Together they are why AC-008 is a criterion rather than a caveat and why EC-008 forbids the obvious fix. | Before writing the ledger's blind-spot row, and any time a second scanner starts to look reasonable. | AC-008 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The governance atom for editing text that already discharges something. Both files this PR touches already do a job; this is the rule that keeps the diff additive. | Before the first edit to either file. | AC-004, AC-005 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | `:174-181` is the initiative's one **measured** defect — the `E0034` explanation that exists in three contributor-facing files and not in the one the reader is looking at. It is the failure this story's in-place citation refuses, and it is what makes AC-001's "in place" wording non-negotiable. | Before writing the citation sentence, to keep it a reason rather than a pointer. | AC-001 |
| `.bklg/docs-that-teach/initiative.md` | The gold source. **DoD-14** at `:462-464` is the scenario this story closes the last third of, in the initiative's own words. | Once, at the start, and again when writing the ledger's DoD note. | AC-001, AC-004 |
| `.bklg/docs-that-teach/page-need-discipline/design/mock.html` | The built (sign-off-pending) mock of the `page-need-declaration` frames, including `declared-valid` and `plain-text-pager` — the two states this story must leave true on a page it only changes. | Before AC-005's and AC-006's plain-pager reads, to compare against what was drawn rather than what is imagined. | AC-005, AC-006 |
| `xtask/src/affected.rs` | `:116-125` (the unconditional file-reading block) and `:249-266` (`INERT`, which already lists `docs/`) — read-only here. They are why NF-005 records the affected gate's behaviour instead of adjusting it. | Only if `cargo xtask affected --base main` behaves unexpectedly and the instinct to "fix" it appears. | AC-008 |
| `.bklg/docs-that-teach/page-need-discipline/project.md` | AC-011 verbatim (`:239-241`) and DR-10 (`:191-193`) — the project-level obligation this story traces to, and the non-goal that forbids authoring teaching content. | When a scope question arises about what the citation may say. | AC-001, AC-003 |

## Clarifications resolved during spec

- **The AC set is exactly the eight the front half enumerated** — AC-001 through AC-008 —
  and none was added or dropped in this pass. The back half assigns each an
  interaction-quality role and a ledger row; `_ledger.md` carries the same eight ids.
- **Which page carries the citation is *decided*, with a *conditional* fallback, and the
  conditional is itself an AC.** `docs/README.md` is the choice, because its shape is the
  discipline's doing (RP-10-2) rather than merely being in the governed tree. Whether the
  checker's walk reports it is `page-need-checker-mounted-in-the-gate`'s decision, so AC-002
  probes it and EC-001 pre-decides the tie-break. This is deliberately not left as
  "implementer's judgement": the tie-break is a scope question, and scope questions decided
  mid-PR are how a story ends up discharging nothing.
- **"Cited" and "announced" are different obligations and neither substitutes for the
  other.** The `docs/README.md` table row landed by `router-precedence-and-announcement`
  discharges the *announced* half of project AC-011; this story's sentence discharges the
  *cited in place* half. EC-002 forbids the plausible shortcut of a second table row.
- **The back-link is mechanically checked and the outbound citation is not**, and the spec
  states the asymmetry rather than blurring it. Making it symmetric would require a scanner
  over two trees, which RS-81-3 forbids and the testing brief already ruled procedural — so
  the honest response is AC-008's stated blind spot, not a second instrument.
- **The static test tier is empty here by construction, not by omission.** No `xtask/src/**`
  change means no `#[test]` can live in this PR. The one permanent assertion the ACs imply —
  that the router's link half resolves targets outside `RULE_DIR` — is recorded as EC-004, a
  forward obligation on the checker story, so the obligation is inherited rather than lost.
- **AC-002's and AC-007's deliberate breakages do not overlap
  `declaration-check-seen-to-fail`.** That story owns the two-needs and unenumerated-token
  demonstrations against `cargo xtask ci`; this story's are a one-off *membership* probe and
  a *pin* observation. Both are recorded in this story's own ledger and neither is cited as
  evidence for the other story's AC-007.
- **Nothing in this story is `.kb/`-authored.** The playbook atom is the slice-mate's, staged
  under `.kb/_intake/`, and NF-008 keeps this story's own diff clear of `.kb/` entirely — the
  hand-authoring failure that was reverted at `0269720` is a repository-level lesson, not a
  per-story preference.
