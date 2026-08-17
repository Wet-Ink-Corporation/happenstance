---
item: HS-S0187
stage: spec
created: 2026-08-17T13:16:35.289Z
updated: 2026-08-17T13:16:35.289Z
template_sig: 87bbf1d0
rendered_sig: c0af24b2
---

# Spec — The reader's invariant carried to an AppendCondition

## Scope lock

| | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` |
| Project | `.bklg/docs-that-teach/application-author-path/project.md` |
| This spec | `.bklg/docs-that-teach/application-author-path/invariant-to-appendcondition-bridge/spec.md` |
| Key briefs | `.bklg/docs-that-teach/application-author-path/_decomposition.md` (UX brief `:11-446`, testing brief `:448-651`) |
| Signed-off design (binding) | `.bklg/docs-that-teach/application-author-path/_design.md` — approved 2026-08-17 (`:1039`) |
| Grounding | `.bklg/docs-that-teach/application-author-path/_grounding.md` |
| Story map / merge order | `.bklg/docs-that-teach/application-author-path/_storymap.md` (this story's row, `:59`) |
| Substrate consumed (not built) | `.bklg/docs-that-teach/checked-documentation-surface/_design.md` (HS-P0020), `.bklg/docs-that-teach/page-need-discipline/_design.md` (HS-P0021) |
| Baseline the anchors resolve against | `.bklg/docs-that-teach/application-author-path/merge-forward-preflight/spec.md:290-297` — the `_baseline.md` that story authors |
| Roadmap pointer | `RUNBOOK.md` — documentation work, no phase added; the roadmap is not amended here |

**Standing note on every line number below.** Each was read on the **pre-merge** tree in this
worktree. Clause *ids* are stable and never renumbered (`spec/SPECIFICATION.md:280`); lines are
not, and the merge that `merge-forward-preflight` lands moves them (ES-25 `:3695`→`:3698`,
VT-30 `:1813`→`:1816`). Cite the id; take the line from `_baseline.md § Anchors` before quoting
one.

## One-line PR slice

Author the bridge material — a cross-entity invariant stated in ordinary event-sourcing
vocabulary carried to `Query`, `QueryItem`, `Tags`, a fold and an `AppendCondition` with no step
requiring `spec/SPECIFICATION.md` or the crate source — carrying DT-5/DT-6's resolved shift
material: a mapping-table narration of the query/append-condition cycle, and the "wrong model"
contrast that ships as compiled, executed code.

## Executive summary

This PR lands **one page** — `docs/carry-your-invariant.md` — registered in HS-P0020's harness so
the gate compiles and runs every fence on it, and **no other content**.

The pointer is project AC-008 and AC-013 (`project.md:256-259`, `:272-273`), realising UX-006 /
IQ-1 (`_decomposition.md:340-343`, `:234-239`) on the `conceptual-bridge` surface whose seven
sections `_design.md` already fixes in order (`:476-491`). The design is signed off and binding.
This spec does not re-open it.

The delta this spec adds is that four things a page author would otherwise discover mid-authoring
were **checked against the tree** rather than inherited from the design's prose:

1. **The design contradicts itself about DT-6's wrong side, and the contradiction is resolvable
   from the record rather than by a new decision.** `## Pattern decision` (`_design.md:206-232`),
   finding F-6's disposition (`:1017`) and the approver's own row (`:1039`) all say the wrong side
   is a guard **tagged too narrowly**, which under-refuses. The stale `## Signatures` snippet
   (`:373-384`) and the §6 heading in `## Composition` (`:487`) still say *type-only*, which is
   the opposite failure and is what the design was rejected once for. Three recorded statements
   beat two residual strings; the too-narrow guard ships.
2. **CF-8 — not CF-7 — is the clause that makes that fence honest.** CF-8
   (`spec/SPECIFICATION.md:7300`, `[FROZEN]`) *requires* a store to accept an append whose
   condition carries a tag no stored event carries. The wrong-side fence's `is_ok()` assertion is
   therefore pinned by a frozen clause rather than by incidental behaviour, and the page's sharpest
   sentence is available without restating anything: the library is doing exactly what it MUST;
   the model is what is wrong. CF-7 (`:7266`) stays cited as the *mirror* — the over-refusing
   failure this fence is deliberately not.
3. **The 22-character heading budget's mechanism does not exist on this surface, so the pinned
   DT-1 anchor survives intact.** That budget is rustdoc's 200px sidebar TOC (`_design.md:575-584`);
   the approver's condition (2) records that the step and bridge surfaces render as markdown under
   HS-P0020's "the markdown is the render" (`_design.md:1018`, `:1039`). `## Where your streams
   went` — 23 characters, and the one recorded reader-facing location every other page links to
   (`_design.md:108-114`) — ships as written. Renaming it to fit a sidebar that is not there would
   break the only anchor the DT-1 decision has.
4. **Two call spellings in the design's own fences do not compile.** `Tags::from_iter([("course",
   "c1")])` (`_design.md:332-335`) cannot: `Tags` implements `FromIterator<Tag>`
   (`crates/happenstance-core/src/tag.rs:479`), not `FromIterator<(&str, &str)>`. The working
   spelling is the fallible `Tags::from_pairs([...])?` (`tag.rs:304`), which is what the worked
   example already uses (`examples/course-subscriptions/src/main.rs:118`, `:123`). The design
   itself delegates residual spellings to the merged tree (`_design.md:365-371`); this is that
   delegation made concrete before an implementer spends a compile cycle on it.

So the deliverable is not "write a page about tags". It is: land the seven-section composition in
the fixed order, with a correct guard and a wrong guard that differ by one expression, both
compiled and **executed** by the gate, the prior-model anchor stated once at its pinned slug, every
normative sentence a resolving clause id, and the whole carry survivable when every off-page link
is struck out.

## Context pack

**The reader, and the moment this page is for.** Backbone activity A4 — *"Take the rule I already
have and show me how to say it here"* (`_storymap.md:44`), which is UI-3 (`_decomposition.md:64-69`).
Persona 1 arrives holding a cross-entity invariant they cannot place, and the reflex question they
are about to ask is *which stream does this go in?* The page answers that question once, where it is
asked, and then carries their rule across. Their stated fear is **silent wrongness** — "a mental
model that looks right, compiles, runs, and is quietly wrong"
(`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:99-106`) — which is why
this page ends with a fence that compiles, runs, and is quietly wrong on purpose.

**The composition is decided; implement it, do not re-derive it.** `_design.md` is binding
(`:10-12`) and the `conceptual-bridge` surface's reading order is load-bearing (`:476-491`):
(1) the answered-need line; (2) `## Your rule, in your words` — the invariant in ordinary
event-sourcing vocabulary, prose only, no code, no library terms; (3) `## Where your streams went`
— the DT-1 anchor, three or four sentences, the only place in the entire page set where a prior
model is named; (4) `## Tag, query, fold, guard` — the four-step narration with the three-column
mapping table directly *beneath* it, never beside it; (5) `## The guard you would write` — the
correct fence, and the first code on the page; (6) the wrong-side contrast, strictly after §5;
(7) `## Back to the working version` — one paragraph and a link up to §5, which exists solely so
the wrong fence is never the last code on the page (IQ-2.3, `_decomposition.md:256-259`).
Re-ordering any of these is re-deciding a signed-off design.

**DT-5 resolved to narration, and AC-013 is discharged explicitly rather than by silence.** No
diagram ships from this project (`_design.md:174-204`). The mapping table has exactly three columns
in this order — *Your words* · *This library's words* · *Where you saw it* — and the four-step
narration uses the same four names, **tag, query, fold, guard**, everywhere it appears
(`_design.md:190-200`). Synonym drift across pages is how a narration loses to a diagram; the fixed
vocabulary is the cheap fix, and this page is where it is set. The reviewer records that AC-013 went
this way (`_storymap.md:120,123-126`); "no diagram shipped" is a recorded outcome, not an omission.

**DT-6's wrong side, stated once so it cannot be got backwards a third time.** It is **one fence
that differs from the correct fence by one expression**: a guard whose query is tagged to what the
command *writes* rather than to the invariant it must *hold*, so the conflicting event never enters
the guard's query, the append is **accepted**, and the invariant is violated with no error
(`_design.md:206-232`, F-6 at `:1017`, approver's row at `:1039`). It asserts the outcome the reader
should fear, not that the code is bad. It is real, compiled, executed code: this project ships
**zero** uncompiled fences and puts **nothing** on HS-P0020's enumerated allowance list
(`_design.md:206-208`) — a stronger commitment than AC-003 requires, and one that removes a
dependency. A type-only guard is the *opposite* failure and must not be what ships: CF-7
(`spec/SPECIFICATION.md:7266`, quoted at `:7286-7288`) says dropping the tag join makes the query
broader and "rejects every command touching any course", so it over-refuses, and an `is_ok()`
assertion against it would simply fail. Where `_design.md:373-384` and `:487` still say "type-only",
they are pre-correction residue; the disposition table and the sign-off row are the record
(`_design.md:1003-1022`).

**Why the store is right and the reader is wrong — and the clause that says so.** The accepting
behaviour the wrong fence asserts is not a library defect the page is exposing; it is
**required**. CF-8 (`spec/SPECIFICATION.md:7300`, `[FROZEN]`): a condition carrying a tag no stored
event carries MUST NOT reject the append, even when a stored event matches the condition's types.
The page therefore cites CF-8 for what the store does, CF-7 for the mirror failure it is not, and
ES-27 (`:3794`) for why the boundary is drawn on tags rather than types at all. That is the whole
normative content of §6, and none of it is restated (AC-011, DR-09, UX-007).

**No `compile_fail`, deliberately.** The obvious move — pin the bad guard with `compile_fail` and
let the compiler enforce the lesson — is rejected in the design (`_design.md:929-935`) because a
too-narrowly-tagged guard **compiles perfectly well**; teaching that the compiler catches this is
exactly the false comfort Persona 1 fears. The fence compiles, runs, and asserts the wrong outcome.
That is the only shape that tells the truth.

**The DT-1 anchor is this page's, and it is the page set's only copy.** `## Where your streams
went`, slug `#where-your-streams-went`, is the single reader-facing statement of the prior-model
decision (`_design.md:108-114`). Every other page **links to that heading** and does not re-argue
it; `_design.md` is the decision's provenance, that heading is its statement. The crate root and
every step of the opening encounter name no prior model at all (`_design.md:116-118`, anti-pattern 6
at `:852-854`) — so the words "aggregate", "your aggregates", "one stream per entity" and "which
stream" appear on this page and nowhere else in the project's output. Changing the heading changes
the slug and breaks that contract; see the executive summary for why the heading budget that would
have forced a rename does not bind here.

**The substrate this page mounts into, consumed and never re-implemented.** HS-P0020 pins
`const TREE: &str = "docs"` and `const HARNESS: &str = "xtask/src/narrative.rs"`, whose
`include_str!` lines register every page; a page in the tree the harness does not register is the
*unregistered* state and fails the gate
(`.bklg/docs-that-teach/checked-documentation-surface/_design.md:92-96`, `:550-568`; their AC-005 at
`project.md:215-217`). Their checker also rejects an untagged fence, an unrecognised info string, an
`ignore`-class fence not on `IGNORE_ALLOWANCES`, any of the `HIDDEN_MARKERS` tokens (`<details`,
`<summary`, `{{#tabs`, `` ```admonish ``, …) anywhere under the tree, and — the row that changes this
story's proof burden — an **unresolvable clause id** (their AC-007,
`checked-documentation-surface/project.md:221-223`). AC-011's machine half is therefore a gate
failure on this page, not a reviewer's note.

**The page's own budgets, and which document owns each.** Fence lines ≤ **80 columns** and prose
source wrap ≤ **90**, page H1 ≤ **40 characters**, narrow content width **70 columns** — HS-P0020's,
for the markdown surface that renders this page
(`checked-documentation-surface/_design.md:387-394`). This project's own 68-column figure
(`_design.md:552-562`) was derived from rustdoc's 696px fence interior and is strictly inside
HS-P0020's 80, so authoring to **68** satisfies both and nothing has to be relaxed to make it true.
The mapping table is three columns, never four (`_design.md:592-595`); paragraphs stay at or under
435 characters (`:586-587`).

**The answered-need declaration, in HS-P0021's form and no other.** First blockquote line of the
file, matching `> **Answers:** \`<token>\` — <reader's question>?`, immediately after the H1 and
before any other block element (`page-need-discipline/_design.md:57`, `:104-117`, `:496`). The token
is one member of the closed `NEEDS` set. **This page declares `explanation`** — "build the mental
model behind a behaviour", whose test is that the reader "can predict what the API does before
running it" (`page-need-discipline/_design.md:177`), which is precisely what §6 asks of them.
`how-to` was the alternative and it lost: under `how-to` the DT-1 anchor and the wrong-side contrast
both read as a second need on the page, and two visible need words is the defect HS-P0021's own
anti-pattern 4 names (`:587-589`). Inventing a second notation is a defect (DR-14); if HS-P0021's
reviewer procedure disputes the token, that routes to HS-P0021 rather than being re-argued here
(project DoD item 9, `project.md:300-302`).

**The API this page teaches, spelled as it actually compiles.** `Query::from_items([...])?` with
one `QueryItem` per entity's tag set, `QueryItem::new(types, tags)?`
(`crates/happenstance-core/src/query.rs:56`, `:185`), `Tags::from_pairs([("course", c)])?`
(`tag.rs:304`) — **not** `Tags::from_iter`, which takes `Tag` values (`tag.rs:479`) — then
`read_decision_model(&store, &query).await?` returning `(events, last)`
(`crates/happenstance-core/src/store.rs:321-331`), a hand-written fold over `events`, and
`AppendCondition::new(query).after_opt(last)` (`append.rs:146`, `:201`). `Guard` is
`#[non_exhaustive]` with no constructor and `AppendCondition::new` takes a single `Query`, not a
list of guards (`_design.md:354-364`) — the multi-guard shape is VT-30's and VT-30 is
`[PROVISIONAL]` (`spec/SPECIFICATION.md:1813`), so the page may describe the guard the reader wrote
and may cite the id, but must not imply the shape is frozen and must not be written so that it needs
rewriting if VT-30 changes (`_design.md:400-405`, UX-007). The shape of the invariant to carry comes
from the worked example's three real ones — capacity spans every subscription for a course, and "the
same course twice" spans one student *and* one course
(`examples/course-subscriptions/src/main.rs:1-19`, the multi-entity query at `:113-125`) — but the
prose statement of it is original to this page (`_grounding.md:171-176`).

**One vocabulary, everywhere.** Every fence imports from `happenstance`, never
`happenstance_core` (ADR-0006, `.kb/decisions/0006-bare-name-to-the-typed-layer.md`; UX-011,
IQ-9; anti-pattern 13 at `_design.md:871-874`). The one sanctioned appearance of the other crate's
name in this project is a single prose sentence on the handoff page, which is
`surface-course-subscriptions`'s, not this story's — and after the merge that sentence's premise may
itself be gone (`merge-forward-preflight/spec.md:117-119`).

**Nothing hidden, nothing folded, nothing invented.** No `#`-prefixed doctest line may carry any
`Query`, `QueryItem`, `Tags`, `Guard`, `AppendCondition`, the append call, the read call, or any
assertion — the rendered page must let a reader rebuild the boundary from what they can see
(`_design.md:524`, UX-003, IQ-2.1, DR-02). No `details`, tabs, accordion, badge, banner, CSS or JS:
the correct answer to "enumerate the affordances this project introduced" is *none* (UX-012, IQ-6,
`_design.md:528`). And nothing is added to `docs/README.md`'s signpost table — this project makes
material reachable and does not decide where the pointer lives, which is HS-P0023's DT-10
(`_design.md:749-751`).

**The bar this is held to.** `cargo xtask affected --base main` at the story checkpoint
(`.redkiln/config.yaml:40`) and `cargo xtask ci --fast` as the project bar — this project is
`terminal: false` (`project.md:17`, `.redkiln/config.yaml:55`). Tier 2 (compiled fence) and tier 3
(executed) are what a fence on this page rides; tier 5 is the reviewer walk for the strike-every-link
falsification and the no-restated-clause spot check (`_decomposition.md:480-486`, `:529`). If
HS-P0020's step turns out to type-check without *executing* doctests, the wrong-side fence's
`is_ok()` assertion proves nothing, and the response is a `#[tokio::test]` of this project's own
wired into the `"tests"` REQUIRED step — with the file named in this spec's PR boundary fence first
— never a weaker criterion (`_decomposition.md:627-636`, `project.md:345-350`).

## Integration contract

- **Archetype**: `capability` — a user-observable slice. The user is Persona 1 and the observable
  is a rendered page they read and copy code out of.
- **Slice / milestone**: `conceptual-bridge`. Slice-mate: `surface-course-subscriptions`
  (implemented in the same context, immediately after this story; the handoff *is* the bridge's last
  step, which is why the two are one slice — `_storymap.md:75-78`). Depends on
  `tension-resolutions` (the signed-off `_design.md` this page implements) and
  `boundary-refusal-encounter` (whose vocabulary this page reuses and whose reader arrives here),
  both transitively behind `merge-forward-preflight`.
- **Mount point**: **`xtask/src/narrative.rs`** — HS-P0020's `HARNESS`, whose `include_str!` line
  registers `docs/carry-your-invariant.md` into the compiled surface
  (`checked-documentation-surface/_design.md:92-96`). The render path is the page file itself,
  **`docs/carry-your-invariant.md`**, under the pinned `TREE = "docs"`. Both halves are the mount:
  a page in the tree that the harness does not name is the *unregistered* state and fails the gate
  (`:550-568`), and a registration naming a page that is not there is the *dangling* state. Neither
  file may be stubbed: if `xtask/src/narrative.rs` is not in the tree when this story starts,
  HS-P0020 has not landed and the correct action is to **halt and report**, never to author a
  parallel tree or a second renderer (`_decomposition.md:118-131`).
- **Wires into**:
  - `happenstance` (`crates/happenstance/src/lib.rs`) — the crate every fence imports from, per
    ADR-0006. The merged crate root, not this worktree's copy (AC-014, DR-13).
  - `crates/happenstance-core/src/query.rs`, `tag.rs`, `append.rs`, `store.rs` — `Query`,
    `QueryItem`, `Tags`, `AppendCondition`, `read_decision_model`, re-exported through the facade
    (`crates/happenstance/src/lib.rs:75`). Bind `EventStore`, never `SendEventStore`, and import
    only one of the two names per fence (CLAUDE.md binding constraint 4; `_design.md:776-780`).
  - `happenstance::MemoryEventStore` — the only store this project's checked material may construct.
    No mock, no stub, no hand-rolled fake, and not `happenstance_testkit`'s fixture or conformance
    macro (`_decomposition.md:578-610`).
  - `spec/SPECIFICATION.md` — cited by clause id, resolved by HS-P0020's checker.
  - HS-P0021's declaration form and its `NEEDS` token set
    (`page-need-discipline/_design.md:57`, `:174-177`).
  - `.redkiln/config.yaml:40,55` — the two commands redkiln runs at this story's and this project's
    grains whether or not anyone types them.
- **Renders surfaces**: **`conceptual-bridge`** (`_design.md:56-59`), route `{tree}/carry-your-invariant/`
  → `docs/carry-your-invariant.md`, states `default`, `mapping-table`, `wrong-side-contrast`,
  `anchor-note`, `narrow-700`, `overflow-fence`. Its `selector` is `null` on the markdown render
  (`checked-documentation-surface/_design.md:115-133`): this repository controls no DOM there, and
  `design.capture` is absent from `.redkiln/config.yaml`, so the perceptual review is a **skip** and
  the reviewer walk is the only instrument. No other surface is rendered or changed here — the
  crate root is `boundary-refusal-encounter`'s and the handoff page is
  `surface-course-subscriptions`'s.
- **Public items**: **none.** This project adds, changes and removes no public Rust API item
  (`_design.md:266-268`); this story adds no `## Items` row of its own and touches none of the three.
  The one artifact it creates is a markdown page and the one line that registers it.
- **Conformance rule(s)**: **none, and it is not adapter-observable.** No port, no store, no fixture
  and no rule in `crates/happenstance-testkit/src/suite.rs` changes. Naming one would be decorative
  by CLAUDE.md's own test. What observes this story is HS-P0020's narrative step (registration,
  fence compilation, allowance list, hidden markers, clause-id resolution), `cargo test` executing
  the fences, `cargo xtask spec-trace`, and the tier-5 reviewer walk. The clause the wrong-side fence
  *depends on* — CF-8 — is already discharged by `condition_with_an_unheld_tag_does_not_reject`
  (`spec/SPECIFICATION.md:7303-7304`); this page cites that clause, it does not add a rule.
- **Clause(s)**: **none discharged, none amended.** The initiative is additive
  (`project.md:155-157`). This page *cites* ES-25 (`[FROZEN]`), ES-27 (`[FROZEN]`), CF-7 and CF-8
  (both `[FROZEN]`) and VT-30 (`[PROVISIONAL]`, and must not be implied frozen). Changing any of
  them takes a new ADR; nothing here comes near one.
- **Advances DoD scenario**: initiative DoD **1** — the narrative material builds and renders as
  part of the gate from a clean checkout (`initiative.md:413-415`) — this page being registered and
  compiled is one of the pages that claim has to be true of. Also DoD **12**, no page has become a
  second specification (`:455-457`), which this page is the hardest case for because it is the one
  that argues about tags; and DoD **13**, nothing load-bearing hidden from the check (`:458-461`),
  which it advances by carrying the one contrast that would most tempt a fold and refusing to fold
  it. It does **not** advance DoD 3 or 4 — those are the opening encounter's and the drill's, and
  this page inherits the vocabulary they taught.

## PR boundary

```
docs/carry-your-invariant.md
xtask/src/narrative.rs
.bklg/docs-that-teach/application-author-path/invariant-to-appendcondition-bridge/**
```

`redkiln verify --grain story` reads the first fenced block above and fails on any file changed
outside it. Two of the three entries are single files on purpose: this story writes one page and
one registration line. `xtask/src/narrative.rs` is in the fence because the mount is not optional —
an unregistered page fails HS-P0020's own check — and the edit to it is exactly one `include_str!`
line. **If the fallback in the Context pack is taken** (HS-P0020's step type-checks without
executing, so this project owes a `#[tokio::test]` of its own), the test file is named in this fence,
in this spec, *before* it is written — the same rule `merge-forward-preflight/spec.md:206-214`
sets for a conflict resolution.

**In this PR**

- `docs/carry-your-invariant.md`: H1 (≤ 40 characters), the `> **Answers:** \`explanation\` — …`
  declaration, and the seven sections in `_design.md:476-491`'s fixed order.
- Two Rust fences: the correct guard (§5) and the wrong-side contrast (§6), differing by one
  expression, both compiled and executed against a real `MemoryEventStore`, both importing from
  `happenstance`.
- The three-column mapping table and the four-step *tag, query, fold, guard* narration, table
  beneath narration.
- `## Where your streams went` — the DT-1 anchor, three or four sentences, at slug
  `#where-your-streams-went`.
- One `include_str!` registration line in `xtask/src/narrative.rs`.
- The story's own ledger and any companion under its backlog folder.

**Explicitly not in this PR**

- Any edit to `crates/happenstance/src/lib.rs`. The crate-root rewrite is
  `boundary-refusal-encounter`'s, and this page must not restate its refusal program.
- Any edit to `examples/course-subscriptions/`, including the `overview.md` extraction — that is
  `surface-course-subscriptions`'s (`_design.md:728-741`).
- The closing outward pointer to the worked example, and the vocabulary-seam sentence that goes with
  it. HS-P0020 fixes that there is exactly one such slot at the bottom of a page
  (`checked-documentation-surface/_design.md:303-305`); the slice-mate fills it, and its *policy* is
  HS-P0023's DT-10.
- Any row in `docs/README.md` — neither the existing signpost table (`:12-23`) nor HS-P0020's
  narrative index region. Pointer policy is HS-P0023's (`_design.md:749-751`).
- Any diagram, image or chart (DT-5, anti-pattern 14) and any entry on `IGNORE_ALLOWANCES`
  (DT-6 ships zero).
- Any edit to `_design.md`, `spec/SPECIFICATION.md`, or any `.kb/` atom. A contradiction found in the
  design is **recorded and routed**, not amended (`project.md:300-302`); no `.kb/` atom is
  hand-authored in this initiative (`project.md:144-147`).
- The set-wide inventories and reviews: the fence inventory and clause audit, and the answered-need
  and anchor walk, are `page-set-assurance`'s two stories (`_storymap.md:61-62`). This page supplies
  its own rows to them; it does not perform them.

**Merge DoD one-liner** — the page exists in the pinned tree, the harness registers it, both fences
compile *and run* green under `cargo test`, the reviewer strikes every off-page link and the carry
still completes, and `cargo xtask affected --base main` is green at the checkpoint.

The implementer MAY also touch the composition-root / wiring file named in the Integration contract
(`xtask/src/narrative.rs`) to mount this slice; that is not scope drift — it is the mount.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The carry completes in place** | The reader starts from a cross-entity invariant in ordinary event-sourcing vocabulary (§2, prose only, no library terms) and ends holding a `Query` of one `QueryItem` per entity's tag set, a fold, and an `AppendCondition` built from that same query. No step requires opening `spec/SPECIFICATION.md` or the crate source. The falsification is IQ-1's: strike every off-page link and the argument still completes. | `project.md:256-259` (AC-008); `_decomposition.md:234-239` (IQ-1), `:340-343` (UX-006); `_design.md:476-491` |
| **The page is mounted, not merely written** | `docs/carry-your-invariant.md` under the pinned `TREE`, named by an `include_str!` line in `xtask/src/narrative.rs`. Unregistered is a checker state that fails the gate; so is a dangling registration. | `checked-documentation-surface/_design.md:92-96`, `:550-568`; `checked-documentation-surface/project.md:215-217` |
| **Every fence is compiled and executed** | Two fences, both tagged with a recognised info string, neither `ignore` nor `no_run`. `no_run` passes the compiled-fence tier and silently never reaches the executed tier — the exact "compiles forever while quietly ceasing to demonstrate its own claim" shape this initiative exists to refuse. Zero entries added to `IGNORE_ALLOWANCES`. | `_decomposition.md:616-625`, `:483-484`; `_design.md:206-208`; `project.md:335` (risks row 1) |
| **The correct guard is tagged to the invariant, not to the row** | §5's fence: `Query::from_items([QueryItem::new(types, Tags::from_pairs([...])?)?])?`, `read_decision_model(&store, &query).await?`, a visible fold, then `AppendCondition::new(query).after_opt(last)`. Binds `EventStore`, imports only that name, imports from `happenstance`. First code on the page. | `crates/happenstance-core/src/query.rs:56,185`; `tag.rs:304`; `store.rs:321-331`; `append.rs:146,201`; `_design.md:317-371`, `:776-780` |
| **The wrong side differs by one expression and asserts the feared acceptance** | §6's fence tags the guard's query to what the command writes rather than to the invariant it holds; the conflicting event never enters the query; the append is **accepted** and the invariant is violated with no error. It asserts `is_ok()` — the outcome the reader should fear — never a prose claim that the code is bad. It is *not* a type-only guard, which over-refuses and would make that assertion fail. | `_design.md:206-232`, `:1017`, `:1039`; `spec/SPECIFICATION.md:7266`, `:7286-7288` (CF-7, the mirror) |
| **The accepting behaviour is cited as required, not reported as a bug** | CF-8 `[FROZEN]`: a condition carrying a tag no stored event carries MUST NOT reject the append, even when a stored event matches the condition's types. §6's normative sentence is a citation to CF-8; ES-27 carries why the boundary is drawn on tags at all. The store is conformant; the model is wrong, and that is the lesson. | `spec/SPECIFICATION.md:7300-7304` (CF-8), `:3794` (ES-27) |
| **No `compile_fail`, and the reason is on the record** | A too-narrowly-tagged guard compiles. Pinning it with `compile_fail` would teach that the compiler catches this — the precise false comfort Persona 1's fear names. | `_design.md:929-935`; `personas-and-journeys.md:99-106` |
| **The wrong fence never occludes the right one** | Marked wrong at its start *and* at its end; strictly after §5 in reading order; never the last code on the page (§7 exists for that); never inside a fold or any `HIDDEN_MARKERS` token. | `_decomposition.md:256-259` (IQ-2.3); `_design.md:485-491`, `:525`, `:855-857` |
| **The prior model is named once, here, at a pinned slug** | `## Where your streams went`, three or four sentences, slug `#where-your-streams-went`, linked to from every page that relies on the decision and re-argued nowhere. The heading is 23 characters and ships as written: the 22-character budget is rustdoc's sidebar TOC, which this markdown surface does not have. | `_design.md:108-118`, `:575-584`, `:1018`, `:1039`; `_decomposition.md:350-352` (UX-009) |
| **The shift is narrated, and the narration's vocabulary is fixed** | Four steps named *tag, query, fold, guard*, used identically wherever they appear; the mapping table has exactly three columns — *Your words* · *This library's words* · *Where you saw it* — placed directly beneath the narration, never beside it. No diagram ships, and AC-013 is recorded as discharged that way. | `_design.md:174-204`, `:481-485`, `:592-595`; `project.md:272-273`; `_storymap.md:120` |
| **Exactly one answered-need, in HS-P0021's form, above the first fence** | `> **Answers:** \`explanation\` — <the reader's question>?` as the first blockquote line, immediately after the H1, before any other block element. Token `explanation`; `how-to` is the alternative that lost, named once. | `page-need-discipline/_design.md:57`, `:104-117`, `:174-177`, `:496`, `:580-589`; `_decomposition.md:299-303` (IQ-8), `:348-349` (UX-008) |
| **Every normative claim is a resolving citation** | ES-25, ES-27, CF-7, CF-8 by id; VT-30 by id with no implication that the shape is frozen. HS-P0020's checker fails the gate on an unresolvable id, and `cargo xtask spec-trace` runs over the tree as this story leaves it. No sentence containing MUST or MUST NOT appears that is not a link to a clause id. | `checked-documentation-surface/project.md:221-223`; `spec/SPECIFICATION.md:280`, `:1813` (VT-30 `[PROVISIONAL]`); `_design.md:400-405`, `:867-868` |
| **Nothing load-bearing is hidden** | No `#`-prefixed doctest line carries a `Query`, `QueryItem`, `Tags`, `Guard`, `AppendCondition`, the append call, the read call or any assertion. A reader rebuilds both guards from the rendered page alone. | `_design.md:524`; `_decomposition.md:243-251` (IQ-2.1), `:329-332` (UX-003) |
| **Zero introduced affordances, zero new dependencies** | No tabs, accordions, nav widgets, badges, CSS or JS; the enumeration of affordances this story introduces is empty. No crate dependency is added — the fences use `happenstance` and, for `#[tokio::main]`/`#[tokio::test]`, the dev-dependency the merge already brought. | `_decomposition.md:287-291` (IQ-6), `:362-365` (UX-012); `merge-forward-preflight/spec.md:106-113` |
| **The page fits its medium** | Fence lines ≤ 68 columns (inside HS-P0020's ≤ 80), prose source ≤ 90, H1 ≤ 40 characters, three-column table, no horizontal scroll at 70 columns, paragraphs ≤ 435 characters. Where a fence exceeds budget the yield order is comments, then setup sentences, then a `use` line already shown visibly — never a line that constructs the boundary. | `checked-documentation-surface/_design.md:387-394`; `_design.md:552-595`, `:601-617` |
| **What is found and not fixed is routed** | A substrate gap — the compiled-fence step not executing doctests, an info string the checker will not accept, a hidden-marker token that collides with legitimate prose — goes to HS-P0020. Pointer and reach gaps go to HS-P0023, comprehension doubts to HS-P0024, incidental bugs to the `support` initiative. The design's own residual type-only wording is recorded as a design-record defect, not silently patched. | `project.md:300-302`; `.redkiln/config.yaml:5`; `_decomposition.md:433-438` |

**Interfaces, explicitly.** This story defines and changes **no Rust interface**. It *consumes*
three: the facade's re-exports (`crates/happenstance/src/lib.rs:75`), HS-P0020's registration
constant, and HS-P0021's declaration grammar. Its own interface to the rest of the page set is two
stable anchors — `#where-your-streams-went`, which the DT-1 decision pins by name, and §5's heading,
which §7 links back up to — plus the four fixed narration words. Anything that looks like an API
decision surfacing while authoring is the merged tree's decision already taken: record it, do not
re-take it.

**The criteria this spec will enumerate** are eight, `AC-001` … `AC-008`, in the order the rows
above establish: the in-place carry; the mounted and executed page; the correct guard; the wrong-side
contrast and its CF-8 citation; the DT-1 anchor at its pinned slug; the narration, table and the
explicit AC-013 disposition; the single answered-need and the `happenstance` vocabulary; and the
rendered page's budgets and zero-affordance surface.

## Data and migrations

**N/A — no schema, no migration, no persisted state.** The only store any fence on this page
constructs is an in-process `happenstance::MemoryEventStore`, built fresh inside the fence and
dropped when it ends. No projection store, no checkpoint, no file written at runtime, no fixture and
no seeded data — the testing brief's instruction here is the inverse of the usual one: *do not
introduce a seam* (`_decomposition.md:578-610`).

Two migration-shaped things exist and neither is one:

- **Citation migration.** Clause ids are stable and never renumbered
  (`spec/SPECIFICATION.md:280`), so nothing this page cites is renamed by the merge; only line
  offsets move, and they are re-resolved from `merge-forward-preflight`'s baseline record rather
  than by editing any stage artifact that carries a stale number
  (`merge-forward-preflight/spec.md:282-288`).
- **Registration, not data.** Mounting the page is one `include_str!` line in
  `xtask/src/narrative.rs`. It is a compile-time inclusion of bytes already in the tree — the same
  mechanism `crates/happenstance/src/lib.rs:10` already uses for the crate README — and it carries
  no state, no ordering requirement and no rollback beyond deleting the line.

## Acceptance criteria

Eight criteria, `AC-001` … `AC-008`, in the order the Behavior rows establish. Each is written
from **Persona 1's intent** — the application author holding a cross-entity invariant they cannot
place (`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:99-106`) — and each
crosses the full stack this story has: the authored page, the harness registration that compiles
it, and the gate command that executes it. "The reader" below is always that persona; "the page"
is always `docs/carry-your-invariant.md`.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** a reader holding a cross-entity invariant stated in ordinary event-sourcing vocabulary and no familiarity with this library's types, **WHEN** they read the page from its answered-need line to `## Back to the working version` **with every off-page link struck out**, **THEN** they can write their own `Query` of one `QueryItem` per entity's tag set, the fold over the events it returns, and the `AppendCondition` built from that same query — and at no step were they required to open `spec/SPECIFICATION.md` or any file under `crates/`. Clause citations are provenance, never required reading. | Tier 5 reviewer walk, run as IQ-1's own falsification (`_decomposition.md:234-239`) against the rendered page and recorded in the story's `implementation-report.md`; tier 2 is the mechanical half — the §5 fence compiles, so what the reader copies is real. Project AC-008 (`project.md:256-259`), UX-006 (`_decomposition.md:340-343`). |
| AC-002 | **GIVEN** a reader who trusts the page because the repository's own gate compiles it, **WHEN** `cargo xtask ci --fast` runs from a clean checkout, **THEN** `docs/carry-your-invariant.md` is registered by exactly one `include_str!` line in `xtask/src/narrative.rs` under the pinned `TREE = "docs"` (neither *unregistered* nor *dangling*), **both** Rust fences carry a recognised info string, and **both are executed** — neither is `ignore`, neither is `no_run`, and **zero** entries are added to `IGNORE_ALLOWANCES`. A fence that type-checks but never runs does not satisfy this criterion. | HS-P0020's narrative REQUIRED step (registration, info string, allowance list, hidden markers) plus `cargo test --locked --workspace --all-features -- --show-output` (`xtask/src/main.rs:143-155`) executing both doctests. Tiers 2 and 3 (`_decomposition.md:483-484`, `:616-625`). If the step proves not to execute doctests, EC-002 applies — never a weaker criterion. |
| AC-003 | **GIVEN** a reader who has just stated their rule in their own words in §2, **WHEN** they reach `## The guard you would write` — the **first code on the page** — **THEN** the fence shows, unhidden, `Tags::from_pairs([...])?` → `QueryItem::new(types, tags)?` → `Query::from_items([...])?` with one item per entity's tag set → `read_decision_model(&store, &query).await?` → a visible fold over the returned events → `AppendCondition::new(query).after_opt(last)`, against a real `happenstance::MemoryEventStore`, importing from `happenstance` and binding `EventStore` (that name only, never `SendEventStore`) — and the guard is tagged to the **invariant it must hold**, not to the row the command writes. | Tier 2 + tier 3: the fence compiles and runs green under the `"tests"` step. Spellings pinned at `crates/happenstance-core/src/query.rs:56,185`, `tag.rs:304`, `store.rs:321-331`, `append.rs:146,201`; vocabulary at `.kb/decisions/0006-bare-name-to-the-typed-layer.md`; binding rule at CLAUDE.md constraint 4 and `_design.md:776-780`. Tier 5 confirms it is the first code in reading order. |
| AC-004 | **GIVEN** a reader whose stated fear is a model that "looks right, compiles, runs, and is quietly wrong", **WHEN** they read `## What a type-only guard misses` — **strictly after** §5, marked as wrong at its start *and* at its end, and never the last code on the page — **THEN** they meet one fence differing from §5's by **one expression**: the guard's query tagged to what the command writes rather than to the invariant it holds, so the conflicting event never enters the query, the append is **accepted**, and the fence asserts exactly that (`is_ok()`) rather than asserting that the code is bad. The page's normative sentence there is a citation to **CF-8** (`[FROZEN]`: a condition carrying a tag no stored event carries MUST NOT reject the append) with **CF-7** named as the mirror over-refusal this fence is deliberately not, and ES-27 for why the boundary is drawn on tags at all. No `compile_fail`: the wrong guard compiles, and teaching that the compiler catches this is the false comfort the criterion exists to refuse. | Tier 3 — the fence runs and its `is_ok()` assertion passes under the `"tests"` step; if it fails, the shipped fence is the type-only guard (the opposite failure) and AC-004 is unmet. Tier 5 checks the two markers, the ordering and that §7 follows. Design record: `_design.md:206-232`, `:1017`, `:1039`, `:929-935`; clauses `spec/SPECIFICATION.md:7300-7304` (CF-8), `:7266`, `:7286-7288` (CF-7), `:3794` (ES-27). |
| AC-005 | **GIVEN** a reader arriving with a one-stream-per-entity model and the reflex question *which stream does this go in?*, **WHEN** they reach `## Where your streams went`, **THEN** the prior model is named and retired there in three or four sentences at the stable slug `#where-your-streams-went` — and the words "aggregate", "your aggregates", "one stream per entity" and "which stream" appear **on this page and nowhere else** in this project's output. Every other page links to that heading rather than re-arguing the decision, and the heading ships at 23 characters because the 22-character budget's mechanism (rustdoc's 200px sidebar TOC) does not exist on this markdown surface. | Tier 5 — the DT-1 consistency walk, owned set-wide by `answered-need-and-anchor-review` (`_storymap.md:62`), to which this story supplies the single recorded location. `cargo doc` with `RUSTDOCFLAGS=-D warnings` (`xtask/src/main.rs:290-301`) fails any inbound intra-doc link that does not resolve to it. UX-009 (`_decomposition.md:350-352`), `_design.md:108-118`, `:575-584`, `:1018`, `:1039`. |
| AC-006 | **GIVEN** a reader who needs to *see* the shift rather than be told it happened, **WHEN** they read `## Tag, query, fold, guard`, **THEN** the four steps are named **tag, query, fold, guard** and those same four words are used identically everywhere they appear on the page, and the mapping table sits **directly beneath** the narration (never beside it) with exactly **three** columns in this order — *Your words* · *This library's words* · *Where you saw it*. No diagram, chart or image ships from this story, and the implementation report **records** that AC-013 was discharged by narration rather than leaving the disposition to silence. | Tier 5 reviewer check against `_design.md:174-204`, `:481-485`, `:592-595`; the recorded disposition is project AC-013 (`project.md:272-273`) and the storymap's coverage row (`_storymap.md:120`). Anti-pattern 14 (`_design.md:867`) is screenshot-checkable: any image on the page fails this. |
| AC-007 | **GIVEN** a reader who bounces off the first fence and needs to know what the page is for before they invest, **WHEN** they look at the top of the page, **THEN** the **first blockquote line**, immediately after the H1 and before any other block element, reads `> **Answers:** \`explanation\` — <the reader's question>?` in HS-P0021's exact form; **exactly one** such declaration exists on the page and it is above the first fence in reading order; the token is `explanation` from the closed `NEEDS` set (`how-to` is the alternative and it lost, because under it the DT-1 anchor and the wrong-side contrast read as a second need). **AND** every fence on the page imports from `happenstance`, never `happenstance_core` — the reader is taught one vocabulary and meets it everywhere this story controls. | HS-P0021's declaration check where it is mounted, plus the tier-5 answered-need walk (project DoD item 7) owned by `answered-need-and-anchor-review`. Form and token set at `page-need-discipline/_design.md:57`, `:104-117`, `:174-177`, `:496`, `:580-589`. Vocabulary: tier 2 compiles the imports; anti-pattern 13 (`_design.md:871-874`) is a grep on the rendered page for `use happenstance_core::`. IQ-8/IQ-9, UX-008/UX-011. |
| AC-008 | **GIVEN** a reader on a 1024×768 window who scans before they read, **WHEN** they scroll the rendered page, **THEN** no fence scrolls horizontally (fence lines ≤ **68** columns, inside HS-P0020's ≤ 80), the H1 is ≤ **40** characters, prose source wraps at ≤ **90**, no paragraph exceeds **435** characters, the mapping table is three columns, and **nothing this story authored is behind a control**: no `details`, `summary`, tabs, accordion, banner, badge, button, CSS or JS, and no `HIDDEN_MARKERS` token anywhere under the tree. No `#`-prefixed doctest line carries a `Query`, `QueryItem`, `Tags`, `Guard`, `AppendCondition`, the append call, the read call or any assertion — the reader rebuilds both guards from the rendered page alone. One `h1`, no skipped heading levels, link text meaningful in isolation. The enumeration of interactive affordances this story introduced is **empty**. | HS-P0020's checker rejects any `HIDDEN_MARKERS` token and any unrecognised info string mechanically (`checked-documentation-surface/_design.md:92-96`, `:550-568`). Widths, heading length, paragraph length and the hidden-line rule are the tier-5 walk against `_design.md:524-528`, `:552-595`, `:601-617` and anti-patterns 3, 4, 8 (`:838-841`, `:855-862`); `checked-documentation-surface/_design.md:387-394` carries the ≤ 80 / ≤ 90 / ≤ 40 outer budgets. UX-003/UX-012/UX-013, IQ-2.1/IQ-6. |

**Coverage of the traced project ACs.** Project **AC-008** (`project.md:256-259`) is carried by
AC-001 end to end, with AC-003 supplying the destination the carry lands on and AC-006 supplying
the narration that gets it there. Project **AC-013** (`project.md:272-273`) is carried by AC-006 —
DT-5 resolved to narration, so the criterion is discharged by a **recorded** disposition rather
than vacuously; the reviewer must record which way it went (`_storymap.md:120,123-126`). Nothing
else in `project.md`'s fourteen is owned here; AC-007, AC-009, AC-011 and AC-012 are set-wide and
belong to `page-set-assurance`'s two stories, to which this page supplies rows.

## Interaction quality

The surface is `conceptual-bridge`, rendered as markdown under HS-P0020's "the markdown is the
render" (`_design.md:1018`). `selector` is `null` and `design.capture` is absent from
`.redkiln/config.yaml`, so **there is no perceptual gate** — the tier-5 reviewer walk is the only
instrument, and every invariant below therefore had to become a table row above rather than a
bullet here. This section says **which AC carries which invariant and how it is checked**; it
adds no criterion of its own.

**State invariants.**

| Invariant | Carried by | How it is checked |
| --- | --- | --- |
| **In place, not a context jump** (IQ-1) — the carry completes without leaving for the specification or the crate source; citations are provenance | AC-001 | Strike every off-page link; the argument still completes (tier 5) |
| **Non-occlusion — nothing load-bearing hidden** (IQ-2.1) — no `#`-prefixed line carries any part of the boundary or an assertion | AC-008 | Read only the rendered page and rebuild both guards (tier 5); `_design.md:524` |
| **Non-occlusion — no fold holds a constraint** (IQ-2.2) — no `details`, no `HIDDEN_MARKERS` token | AC-008 | HS-P0020's checker fails the gate on the token; anti-pattern 3 is screenshot-visible |
| **Non-occlusion — the wrong model never occludes the right one** (IQ-2.3) — marked at both ends, strictly after §5, never the last code on the page | AC-004 | Scroll to the bottom: the last code is the correct guard (anti-pattern 7, `_design.md:857-859`) |
| **Place is preserved** (IQ-4) — `#where-your-streams-went` is stable, human-readable and unnumbered, so inserting a section breaks no inbound link | AC-005 | `cargo doc` with `-D warnings` fails an unresolved inbound intra-doc link; tier-5 anchor walk |
| **Keyboard reachability by construction** (IQ-6) — every affordance is one the base medium already renders | AC-008 | Enumerate the affordances this story introduced; the correct answer is **none** |
| **One need, stated before the first fence** (IQ-8) | AC-007 | HS-P0021's own check, applied by a reviewer |
| **Vocabulary stable across the reader's path** (IQ-9) | AC-007 | No fence imports `happenstance_core`; tier 2 compiles what does |
| **Reversibility** (IQ-5) — **not this story's.** The remove → observe failure → revert drill is `boundary-falsification-drill`'s (project AC-005, DoD item 3) and appears on step 3 of the opening encounter, not on the bridge | — | Stated so its absence here is a recorded scope line, not an omission |
| **Mid-sequence arrival** (IQ-3) — **not this story's.** The step header block is the opening encounter's mitigation; the bridge is one page, not a staged sequence | — | `_design.md:476-491` gives the bridge no step header block |

**Composition invariants** — taken from the signed-off `_design.md`, which is binding and is not
re-decided here.

| Invariant | The design's number | Carried by |
| --- | --- | --- |
| **Presentation exists at all** — the page is a *rendered, compiled* surface reached through the harness, not a markdown file sitting in a tree. An unregistered page is a checker state that fails, and so is a dangling registration | `checked-documentation-surface/_design.md:92-96`, `:550-568` | AC-002 |
| **Composition and placement** — seven sections in the fixed reading order; §5 is the first code; §6 strictly after §5; §7 exists solely so the wrong fence is never last | `_design.md:476-491` | **AC-003** (§5 first code), **AC-004** (§6 placement and §7), **AC-006** (§4 narration then table) |
| **Placement of the mapping table** — directly *beneath* the narration, never beside it: at 764px there is no room for two columns and a side-by-side layout would be custom CSS, which UX-012 forbids | `_design.md:481-485` | AC-006 |
| **Transience — persistent chrome vs revealed vs opened-on-demand** — the answered-need line, every fence, the clause citations and the wrong-side contrast are all **persistent**; the contrast is explicitly *never* opened-on-demand; hidden doctest lines are binary in this medium (absent from the DOM, no hover recovers them) and are forbidden for every boundary-constructing expression | `_design.md:522-528` (transience table) | **AC-007** (the need line), **AC-004** (the contrast), **AC-008** (hidden lines, zero controls) |
| **Density budget, with its real numbers** — fence ≤ **68** columns (72 is where `overflow-x` engages at the 696px fence interior); paragraphs ≤ **435** characters; mapping table **three** columns, never four (four give ~20 characters per cell at 764px); H1 ≤ **40**; prose source ≤ **90** | `_design.md:552-595`; `checked-documentation-surface/_design.md:387-394` | **AC-008**, with the three-column rule also in **AC-006** |
| **What yields first when a fence exceeds budget** — comments, then setup sentences down to one, then a `use` line already shown visibly; **never** a line that constructs a `Query`, `Tags`, `Guard` or `AppendCondition`, never the append call, never an assertion | `_design.md:601-617` | **AC-003**, **AC-008** |
| **Hierarchy** — one `h1`, `##` for the seven sections, no skipped levels, link text meaningful in isolation | UX-013 (`_decomposition.md`), `_design.md:476-491` | AC-008 |
| **The 22-character `##` budget does not bind on this surface** — its mechanism is rustdoc's sidebar TOC ellipsis, and this surface has no sidebar (approver's condition 2). `## Where your streams went` ships at 23 characters because renaming it would break the only anchor the DT-1 decision has | `_design.md:575-584`, `:1018`, `:1039` | **AC-005** (stated as a recorded exception, not a silent overrun) |

**The design's named anti-patterns, and where each is gated.** 1 (a block labelled not-compiled
or exempt) → AC-002, which ships zero allowance entries. 3 (a collapsed disclosure triangle) →
AC-008. 4 (a fence with a horizontal scrollbar at 1024×768) → AC-008. 6 (the words "aggregate",
"your aggregates", "one stream per entity", "which stream" outside this page's DT-1 section) →
AC-005. 7 (the wrong-model block last on its page or above the correct one) → AC-004. 8 (any tab
strip, accordion, breadcrumb, rail, banner, badge or button that is not rustdoc's own) → AC-008.
11 (a MUST sentence that is not a link to a clause id) → AC-004 for §6, and the set-wide audit for
the rest. 12 (two answered-needs, or one below the first fence) → AC-007. 13 (`use
happenstance_core::` in a fence) → AC-007. 14 (a diagram, chart or image) → AC-006. Anti-patterns
2, 5, 9, 10 and 15 belong to other surfaces (the crate root and the step pages) and are named here
so their absence is a recorded scope line rather than an oversight.

## Error conditions

| id | Condition | Required response |
| --- | --- | --- |
| **EC-001** | `xtask/src/narrative.rs` is not in the tree when this story starts — HS-P0020's mount has not landed. (It is **not** in the tree as this spec is written; `xtask/src/` holds ten files and that is not one of them.) | **Halt and report.** Do not author a parallel tree, a second renderer, or a stub `narrative.rs`. The dependency is a real blocking edge and a missing dependency halts loudly (`_decomposition.md:118-131`). |
| **EC-002** | HS-P0020's narrative step **type-checks without executing** doctests. The §6 fence's `is_ok()` assertion then proves nothing, and AC-002 is unmet by construction. | This project owes a `#[tokio::test]` of its own wired into the `"tests"` REQUIRED step (`_decomposition.md:627-636`, `project.md:345-350`), in the manner of `examples/course-subscriptions/src/main.rs:195-224`'s `commit` helper — **never** a weaker criterion, never `no_run` accepted as sufficient. The file's name is fixed here so the amendment is mechanical: **`xtask/tests/bridge_guard.rs`**. Because it is outside the front half's PR-boundary fence, adding it is a scope change and goes through `redkiln advance` as a recorded human decision, not a quiet fence edit. Route the substrate gap to HS-P0020. |
| **EC-003** | A clause id cited on the page does not resolve — HS-P0020's checker (their AC-007, `checked-documentation-surface/project.md:221-223`) or `cargo xtask spec-trace` fails. | Re-resolve the id against `merge-forward-preflight`'s baseline record rather than editing a stage artifact that carries a stale line number (`merge-forward-preflight/spec.md:282-288`). Ids are stable and never renumbered (`spec/SPECIFICATION.md:280`); if an id genuinely does not exist, the citation was invented and must be removed, not renumbered to something nearby. |
| **EC-004** | The checker rejects the info string the fences carry, or a `HIDDEN_MARKERS` token collides with legitimate prose on this page. | Route to HS-P0020 as a substrate gap (`project.md:300-302`). Do **not** work around it by marking a fence `ignore` or `no_run`, and do not add an `IGNORE_ALLOWANCES` entry — this project ships zero (`_design.md:206-208`), and taking one would re-litigate DT-6 without a decision. |
| **EC-005** | HS-P0021's reviewer procedure disputes `explanation` as this page's need token. | Route to HS-P0021 (project DoD item 9, `project.md:300-302`). Do **not** invent a second notation, and do not carry two visible need words — that is HS-P0021's own anti-pattern 4 (`page-need-discipline/_design.md:587-589`) and DR-14's defect. |
| **EC-006** | A fence exceeds 68 columns or the §5 fence exceeds its rendered-line budget. | Yield in the recorded order — comments, then setup sentences down to one, then a `use` line **only if already shown visibly earlier on the same page** — and stop as soon as it fits (`_design.md:601-617`). If it still does not fit, the *scenario* is too large and the scenario shrinks. Never hide, shorten or delete a line that constructs the boundary or asserts on it. |
| **EC-007** | The implementer reaches `_design.md:373-384` or `:487` and finds "type-only" as the wrong side, contradicting `## Pattern decision`, F-6's disposition and the approver's row. | Ship the **too-narrowly-tagged** guard. The three recorded statements are the record; the two residual strings are pre-correction residue. Record the contradiction as a design-record defect in the implementation report and route it; **do not edit `_design.md`** — a signed-off design is amended by decision, not in passing (`project.md:300-302`). A type-only guard over-refuses (CF-7) and its `is_ok()` assertion would simply fail, so EC-007 taken wrongly surfaces as a red test. |
| **EC-008** | `Tags::from_iter([("course", "c1")])` (the spelling in `_design.md:332-335`) fails to compile. | Expected. `Tags` implements `FromIterator<Tag>` (`crates/happenstance-core/src/tag.rs:479`), not `FromIterator<(&str, &str)>`. Use the fallible `Tags::from_pairs([...])?` (`tag.rs:304`), which is what `examples/course-subscriptions/src/main.rs:118,123` already uses. The design delegates residual spellings to the merged tree (`_design.md:365-371`); this is that delegation, and it costs no decision. |
| **EC-009** | A fence needs to assert on a stored event's position. | Do not assert on literal position values — the specification permits gaps and a conformant store may leave them (CLAUDE.md, "The rule that matters"). Compare against positions the store actually assigned, or restructure the fence so it does not need one. Both guards on this page are built from `after_opt(last)`, where `last` came from the read, so no literal is needed. |

## Non-functional

| id | Requirement | Basis |
| --- | --- | --- |
| **NF-001** | **No new crate dependency, and no new workspace member.** The fences use `happenstance` and, for `#[tokio::main]`/`#[tokio::test]`, the dev-dependency the merge already brought (`merge-forward-preflight/spec.md:106-113`). Adding a dependency to teach a page is a cost the page has not earned. | `_decomposition.md:287-291` (IQ-6); the project's own zero-affordance line |
| **NF-002** | **Gate cost stays negligible.** Two doctests against an in-process `MemoryEventStore`, no I/O, no network, no sleep, no fixture. The `"tests"` step's wall clock must not move measurably; if it does, the scenario is too large. | Testing brief's fixtures section (`_decomposition.md:578-610`) |
| **NF-003** | **Determinism.** Neither fence may depend on wall-clock time, iteration order of a map, or any position literal. Given the same tree the two doctests produce the same result on every run and every platform, including a `wasm32` docs build of `happenstance-core` that does not compile them at all. | CLAUDE.md, positions-may-have-gaps rule; `xtask/src/main.rs`'s wasm32 steps |
| **NF-004** | **The compiled wrong-model contrast is a standing maintenance liability, accepted knowingly.** It is gate-checked forever, and any change to `Query`/`Tags`/`AppendCondition` that breaks it breaks the build rather than rotting the page. That is the trade DT-6 made; it is recorded here so a future maintainer meets the reason before the red test. | `project.md:335` (risks row 4); `_design.md:206-232` |
| **NF-005** | **Content-level accessibility floor.** One `h1`; no skipped heading levels; link text meaningful in isolation ("the worked example", never "here"); every fence copy-faithful, so a reader who copies it gets a program that runs. No colour, weight or size carries a distinction on its own — this project authors no CSS and therefore cannot. | UX-013 (`_decomposition.md`); `_design.md:528` |
| **NF-006** | **Reading time is budgeted, not open.** The page is one screen of prose per section at 1024×768 and two fences; the initiative's non-goal is "Volume. Pages written is not the measure." A ninth element in a section means the section is doing two things. | `_design.md:783-786`; `initiative.md` (non-goals) |
| **NF-007** | **The page survives a renderer change.** No chrome assumption is load-bearing: the surface is markdown under HS-P0020's "the markdown is the render", so if HS-P0020 ever pins a different renderer only composition survives, and composition is all this page depends on. This is why the 22-character sidebar budget is recorded as non-binding rather than obeyed defensively. | F-7's disposition (`_design.md:1018`); `_design.md:1039` |

## Implementation notes (non-prescriptive)

Not instructions — the places a competent implementer would otherwise spend a cycle discovering,
and the order that costs least.

- **Read `_baseline.md` before quoting any line number.** Every `spec/SPECIFICATION.md` line in
  this spec was read pre-merge. Ids are stable; lines move (ES-25 `:3695`→`:3698`, VT-30
  `:1813`→`:1816`). Take the line from `merge-forward-preflight`'s baseline record, cite the id in
  the page, and never propagate a stale number into the rendered material.
- **Write §5 first, then derive §6 from it by changing one expression.** The two fences differing
  by exactly one expression is the teaching, and it is much easier to preserve by construction
  than to recover by editing two independently written programs into alignment. Diff them before
  committing: if more than one expression differs, the contrast has drifted.
- **Get §6 red before you get it green.** A too-narrowly-tagged guard makes the append *succeed*;
  the fence asserts `is_ok()`. Write the assertion first against the *correct* guard, watch it
  fail (the correct guard refuses), then narrow the tag and watch it pass. That sequence is the
  cheapest available proof that the fence is actually demonstrating what §6 claims, and it costs
  one compile.
- **The invariant's prose statement is original to this page; its *shape* is not.** The worked
  example already carries three real cross-entity invariants — capacity spans every subscription
  for a course, and "the same course twice" spans one student *and* one course
  (`examples/course-subscriptions/src/main.rs:1-19`, the multi-entity query at `:113-125`).
  Borrow the shape, write the sentence fresh (`_grounding.md:171-176`).
- **§2 is prose only.** No code, no library terms — not `Query`, not `Tags`, not `tag`. The whole
  point of the mapping table is that the reader's words and the library's words are two columns;
  if §2 already uses the library's words the table has nothing to map.
- **Keep the four narration words literal.** *tag, query, fold, guard.* Not "tagging", not
  "the query step", not "the decision model". Synonym drift is how a narration loses to a diagram,
  and this page is where the vocabulary is set for the pages that come after it.
- **VT-30 is `[PROVISIONAL]`.** The page may describe the guard the reader wrote and may cite the
  id; it must not imply the multi-guard shape is frozen, and it must not be written so that it
  needs rewriting if VT-30 changes (`_design.md:400-405`, UX-007). `AppendCondition::new` takes a
  single `Query`, and `Guard` is `#[non_exhaustive]` with no constructor — write to that.
- **Budget the fence at 68 columns while typing, not afterwards.** Doctest fences are not
  rustfmt'd and `rustfmt.toml` declares only the edition, so the default `max_width = 100`
  overflows this column by ~13 characters. Breaking `use` blocks and long call chains is manual
  and is much cheaper before the fence is written than after.
- **§7 is one paragraph and a link up to §5's anchor.** It is cheap on purpose. Its whole job is
  that the last code a reader sees is the correct guard.
- **Anything that looks like an API decision surfacing while authoring is a decision the merged
  tree already took.** Record it; do not re-take it, and do not amend `_design.md`,
  `spec/SPECIFICATION.md` or any `.kb/` atom from inside this story.

## Tests and CI (merge gate)

Tiers are the testing brief's five (`_decomposition.md:448-651`); this story adds none of its own.

| Tier | Command / path | Proves |
| --- | --- | --- |
| **0 — preflight** | `merge-forward-preflight`'s recorded merge + its `_baseline.md` | Project AC-014: the page is authored against the merged `crates/happenstance/src/lib.rs`, and every clause line this spec quotes is re-resolvable. Nothing here starts until it is recorded. |
| **1 — structural** | `cargo xtask spec-trace` (`xtask/src/main.rs:315-327`); `cargo doc --locked --workspace --all-features --no-deps --document-private-items` with `RUSTDOCFLAGS=-D warnings` (`:290-301`) | Every clause id this page cites resolves; no intra-doc link into `#where-your-streams-went` is silently broken (AC-003 citations, AC-004, AC-005). |
| **2 — compiled fence** | HS-P0020's narrative REQUIRED step over `TREE = "docs"`, registered from `xtask/src/narrative.rs` | Both fences type-check against the real workspace crates; the page is registered (not *unregistered*, not *dangling*); the info strings are recognised; no `HIDDEN_MARKERS` token is present; no `IGNORE_ALLOWANCES` entry was added. **AC-002**, and the mechanical half of AC-003, AC-007 and AC-008. |
| **3 — executed** | `cargo test --locked --workspace --all-features -- --show-output` (`xtask/src/main.rs:143-155`); fallback per EC-002 is `xtask/tests/bridge_guard.rs` | The §5 guard actually refuses and the §6 guard actually **accepts** — the `is_ok()` assertion is the criterion, not a comment. **AC-003**, **AC-004**. This is the tier that rejects a `no_run` fence, the named failure mode. |
| **4 — human-observed falsification** | Not this story's. The remove → fail → revert drill is `boundary-falsification-drill`'s (project AC-005, DoD item 3) | Recorded here so its absence is scope, not a gap. |
| **5 — review sign-off** | The reviewer walk recorded in this story's `implementation-report.md`, and the two set-wide walks in `page-set-assurance` | IQ-1's strike-every-off-page-link falsification (**AC-001**); §5-before-§6 ordering and the two wrong-side markers (**AC-004**); the DT-1 anchor's single location (**AC-005**); the narration vocabulary, the three-column table and the recorded AC-013 disposition (**AC-006**); one answered-need above the first fence (**AC-007**); widths, hidden lines and the empty affordance enumeration (**AC-008**). |
| **story checkpoint** | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | The story grain — fmt, clippy `-D warnings` and tests for the affected packages, plus the five file-reading lints and `spec-trace` unconditionally. Runs whether or not anyone types it. |
| **project bar** | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | The project-scoped integration bar; this project is `terminal: false` (`project.md:17`), so the whole-initiative `cargo xtask ci` re-observation is HS-P0025's at closeout. |
| **ledger gate** | `redkiln verify --grain story` over `_ledger.md` | Every AC-001…AC-008 row present, `satisfied: true`, with non-placeholder evidence, before `implement → report`. |

**No perceptual capture.** `design.capture` is deliberately absent from `.redkiln/config.yaml`, so
the design review is a **skip** and tier 5 is the only instrument for every composition invariant
above. That is the reason those invariants are AC rows: an unstyled render satisfies every
mechanical check on this page perfectly.

## Risks and coupling (PR-scoped)

| Risk | Likelihood / Impact | Note, and what absorbs it |
| --- | --- | --- |
| **The §6 fence ships as a type-only guard** — the design's residual wording wins over its own correction, for the third time | Medium / High | This project was rejected once at its design gate for exactly this. Absorbed mechanically: a type-only guard over-refuses (CF-7), so `is_ok()` fails and the build is red. EC-007 says which record wins and forbids editing `_design.md` in passing. |
| **HS-P0020's step compiles without executing** — the whole `is_ok()` proof evaporates silently | Medium / High | The initiative's named central risk in this story's shape. EC-002 is the response and it is a *stronger* obligation, not a weaker criterion; the fallback file is named before it is written. |
| **`xtask/src/narrative.rs` is not in the tree** — it is not, as this spec is written | High / High | Not a risk to mitigate but a precondition to honour: EC-001, halt and report. Authoring the page against a stub renderer would produce a page nobody's gate compiles, which is the *unregistered* state wearing a disguise. |
| **The two fences drift apart** so the contrast is no longer one expression | Medium / Medium | Derive §6 from §5 rather than writing it; diff before committing. Once they differ by two expressions the reader can no longer see what the mistake *was*. |
| **The DT-1 heading gets renamed to fit the 22-character budget** | Medium / High | It would change the slug and break the only anchor the DT-1 decision has, on every page that links to it. AC-005 records the exception and its mechanism-based reason so a later reviewer meets the argument, not just the overrun. |
| **The page becomes a second specification** — it argues about tags, so it is the hardest case in the whole initiative for DoD 12 | Medium / High | Anti-pattern 11 and AC-004: every MUST sentence is a link to a clause id. The set-wide no-restated-clause spot check is `fence-inventory-and-clause-audit`'s, and this page supplies its rows. |
| **Coupling to the slice-mate** — `surface-course-subscriptions` fills the single outward-pointer slot at the bottom of the page set and carries the vocabulary-seam sentence | Certain / Low | By design: the handoff *is* the bridge's last step, which is why the two are one slice. This story must leave that slot empty rather than filling it, or the slice-mate has nowhere to land (`checked-documentation-surface/_design.md:303-305`). |
| **Coupling to `page-set-assurance`** — four project ACs are properties of the *set*, not of this page | Certain / Low | This story produces its own inventory rows and its own answered-need row and hands them on; it does not perform either walk (`_storymap.md:61-62`). |
| **Persona 1 is inferred, not observed** | High / Medium | An initiative-level assumption. HS-P0024 is the first contact with a real reader and its dispositions may return work here; that is a recorded route, not a defect in this page. |

## Dependencies

**Blocks on** — both are `depends_on` in the story map (`_storymap.md:59`) and both must be
complete before a line of this page is written:

- **`tension-resolutions`** — lands `_design.md` signed off with DT-1, DT-4 and the joint DT-5/DT-6
  resolution. This story *implements* that design's `conceptual-bridge` composition; without it
  there is no fixed section order, no wrong-side decision, and no recorded DT-1 anchor. Project
  AC-001/AC-002/AC-003 are that story's.
- **`boundary-refusal-encounter`** — authors the opening encounter this reader arrives from. Its
  vocabulary (`Query`, `Tags`, `AppendCondition`, `ConditionViolated`) is what this page assumes
  the reader has already met, and its pages are the ones that link *into*
  `#where-your-streams-went` rather than re-arguing the prior model.

Both sit transitively behind **`merge-forward-preflight`**, which is a precondition on the whole
project (AC-014, DR-13) rather than a direct edge here.

**Unlocks** — three stories name this one in their own `depends_on` (`story.md` `blocks:`
HS-S0188, HS-S0189, HS-S0190):

- **`surface-course-subscriptions`** — the slice-mate, implemented in the same context immediately
  after this story. It fills the outward-pointer slot this page leaves empty and carries the
  `happenstance` / `happenstance_core` vocabulary-seam sentence at the link.
- **`fence-inventory-and-clause-audit`** — needs this page's two fences and its clause citations to
  exist before the set-wide inventory and the no-restated-clause audit can be produced
  (project AC-007, AC-011).
- **`answered-need-and-anchor-review`** — needs this page's single answered-need declaration and
  its DT-1 anchor before the set-wide walk can find them (project AC-009, AC-012, DoD item 7).

## Anchors (progressive disclosure)

Every path below was confirmed present in this worktree. Open them **when the row says**, not
before — the Context pack above is what this story needs on first load, and these carry the depth
that would flatten it. Link them from the page only where the page's own argument survives their
removal (IQ-1).

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/application-author-path/_design.md` | The signed-off, **binding** design. `:476-491` is the seven-section order; `:206-232` + `:1017` + `:1039` are DT-6's wrong side; `:552-617` is the density budget and the yield order; `:838-874` are the anti-patterns; `:1039` is the approver's row and its two conditions. | Before writing the first heading, and again before every section boundary. Re-open `:1003-1022` the moment a "type-only" string appears anywhere. | AC-003, AC-004, AC-005, AC-006, AC-008 |
| `.bklg/docs-that-teach/application-author-path/_decomposition.md` | The UX brief (`:11-446`) carries IQ-1…IQ-9 and UX-001…UX-014 with each invariant's own falsification; the testing brief (`:448-651`) carries the five tiers, the AC-to-tier map, the deliberate absence of fixtures, and the `no_run` note. | The UX half before authoring; the testing half before writing the first fence and again before claiming AC-002. | AC-001, AC-002, AC-007, AC-008 |
| `.bklg/docs-that-teach/application-author-path/project.md` | The fourteen project ACs (`:250-279`), the nine DoD items, the risks table and the routing rule (`:300-302`) that says a found-and-unfixed thing is recorded, never absorbed. | When resolving what this story owes versus what routes elsewhere — i.e. the moment anything surprising is found. | AC-001, AC-006 |
| `.bklg/docs-that-teach/application-author-path/_storymap.md` | This story's row at `:59` (the one-line slice, `depends_on`, `traces_to`), the slice rationale, and the coverage table that records AC-013's disposition at `:120`. | Before writing the AC-013 disposition sentence in the implementation report, and when checking a scope line against the slice-mate. | AC-006 |
| `.bklg/docs-that-teach/checked-documentation-surface/_design.md` | The substrate this page mounts into: `:92-96` the `TREE`/`HARNESS` pins and the `include_str!` registration, `:115-133` why `selector` is `null` on a markdown render, `:303-305` the single outward-pointer slot, `:387-394` the ≤ 80 / ≤ 90 / ≤ 40 budgets, `:550-568` the *unregistered* and *dangling* states. | Before writing the registration line, and before setting any width budget. | AC-002, AC-008 |
| `.bklg/docs-that-teach/checked-documentation-surface/project.md` | Their AC-005 (`:215-217`) and AC-007 (`:221-223`) — registration and clause-id resolution as *their* gate rows, which is why an unresolvable id on this page fails the build rather than a review. | When EC-003 or EC-004 fires, to see whose check is failing before routing. | AC-002 |
| `.bklg/docs-that-teach/page-need-discipline/_design.md` | The answered-need grammar (`:57`, `:104-117`, `:496`), the closed `NEEDS` set and each token's test (`:174-177`), and anti-pattern 4 on two visible need words (`:587-589`). | Before writing the H1's following line — the declaration is the first block element after it. | AC-007 |
| `spec/SPECIFICATION.md` | The clauses this page cites: CF-8 `:7300-7304` (the store MUST accept a condition carrying an unheld tag — the sentence §6 rests on), CF-7 `:7266` + `:7286-7288` (the mirror over-refusal), ES-27 `:3794` (tags not types), ES-25, VT-30 `:1813` (`[PROVISIONAL]`), and `:280` (ids are stable, lines are not). | Before writing §6's normative sentence; re-resolve the lines from `_baseline.md` first. Never link it *into* the reading path — citations are provenance (IQ-1). | AC-004, AC-001 |
| `crates/happenstance-core/src/query.rs` | `QueryItem::new` at `:185` and `Query::from_items` at `:56` — the fallible constructors both fences call, and the one-item-per-entity shape the carry lands on. | Before writing the §5 fence. | AC-003 |
| `crates/happenstance-core/src/tag.rs` | `Tags::from_pairs` at `:304` is the working spelling; `FromIterator<Tag>` at `:479` is why the design's `Tags::from_iter([("course","c1")])` cannot compile (EC-008). | The moment a `Tags` construction is typed. | AC-003, AC-004 |
| `crates/happenstance-core/src/append.rs` | `AppendCondition::new` at `:146` takes a single `Query`; `after_opt` at `:201`. `Guard` is `#[non_exhaustive]` with no constructor — the reason the page describes a guard rather than constructing one. | Before writing the condition, and before any sentence that could imply VT-30's multi-guard shape is frozen. | AC-003, AC-004 |
| `crates/happenstance-core/src/store.rs` | `read_decision_model` at `:321-331` returns `(events, last)` — the pair the visible fold and `after_opt(last)` both consume. | Before writing the fold. | AC-003 |
| `crates/happenstance/src/lib.rs` | The facade every fence imports from (ADR-0006), its re-exports at `:75`, and at `:10` the `include_str!` pattern the harness registration mirrors. Use the **merged** copy, not this worktree's. | Before the first `use` line. | AC-003, AC-007 |
| `examples/course-subscriptions/src/main.rs` | Three real cross-entity invariants at `:1-19`, the multi-entity query at `:113-125`, the working `Tags::from_pairs` spellings at `:118,123`, and the `commit` helper at `:195-224` that EC-002's fallback test would be written in the manner of. | When choosing the invariant's *shape* — then write the sentence fresh. Again if EC-002 fires. | AC-003, AC-004 |
| `.kb/decisions/0006-bare-name-to-the-typed-layer.md` | The Accepted decision that makes `use happenstance::{…}` the taught vocabulary and `happenstance_core` the wrong one on this page. | If any impulse arises to import from the contract crate "because that is where the type lives". | AC-007 |
| `.bklg/docs-that-teach/application-author-path/merge-forward-preflight/spec.md` | `:282-297` — the `_baseline.md` record and the rule that a moved line is re-resolved from it rather than patched into a stage artifact; `:106-113` the dev-dependency the merge brings; `:206-214` the name-it-before-you-write-it rule EC-002 reuses. | Before quoting any `spec/SPECIFICATION.md` line into the page. | AC-004 |
| `.bklg/docs-that-teach/application-author-path/tension-resolutions/spec.md` | The upstream story's own account of what `_design.md` was required to settle — useful when the design's text is ambiguous and the question is what the resolution was *for*. | Only if `_design.md` reads ambiguously; it is the second reading, not the first. | AC-004, AC-005 |
| `.bklg/docs-that-teach/application-author-path/boundary-refusal-encounter/spec.md` | What the reader has already met before arriving here — the vocabulary, the four named types, and the pages that link into `#where-your-streams-went` rather than re-arguing DT-1. | Before §2, so the page assumes exactly what the reader has and no more. | AC-001, AC-005 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | `:99-106` — Persona 1's stated fear, "a mental model that looks right, compiles, runs, and is quietly wrong", which is why §6 exists in the shape it does and why `compile_fail` was rejected. | Before writing §6's framing sentence, and any time the wrong-side fence feels like an indulgence. | AC-004 |
| `.bklg/docs-that-teach/_decomposition.md` | The initiative DAG, the merge-forward rule, and `:118-131`'s halt-loudly-on-a-missing-dependency instruction that EC-001 invokes. | If `xtask/src/narrative.rs` is absent. | AC-002 |
| `.bklg/docs-that-teach/initiative.md` | DoD scenarios 1, 12 and 13 (`:413-415`, `:455-457`, `:458-461`) — the three this page advances, including "no page has become a second specification", for which this page is the hardest case. | When writing the implementation report's DoD advancement claims. | AC-001, AC-004 |
| `xtask/src/main.rs` | The `REQUIRED` array: `"tests"` at `:143-155`, `"documentation"` at `:290-301`, `"specification traceability"` at `:315-327` — the three steps that actually execute this story's proofs. | Before claiming AC-002, and when EC-002's fallback needs wiring into a step that already sweeps it. | AC-002 |
| `xtask/src/constitution.rs` | `:11-18` and `:39-50` — the `#[cfg(doctest)] mod` + `include_str!` shape, one module per included file, that this repository already uses to make a file's bytes load-bearing. | Only if the registration line's shape is unclear from HS-P0020's design. | AC-002 |
| `.redkiln/config.yaml` | `:40` the story-grain `affected_gate` and `:55` the non-terminal project bar — the two commands that run at this story's checkpoint whether or not anyone types them; `:5` the routing target for incidental bugs. | Before the checkpoint commit. | AC-002 |

## Clarifications resolved during spec

1. **The AC set is exactly the eight the first pass named**, `AC-001` … `AC-008`, in the order the
   Behavior rows establish (`spec.md`, "The criteria this spec will enumerate"). None was added,
   none dropped, and the ledger matches. The eight cover both traced project ACs: AC-008 by
   AC-001/AC-003/AC-006, AC-013 by AC-006's recorded disposition.
2. **DT-6's wrong side is the too-narrowly-tagged guard**, resolved against the design's own
   residual "type-only" strings. Three recorded statements (`_design.md` `## Pattern decision`,
   F-6's disposition at `:1017`, the approver's row at `:1039`) beat two stale ones (`:373-384`,
   `:487`). The residue is **recorded and routed** as a design-record defect (EC-007); this story
   does not amend `_design.md`.
3. **CF-8, not CF-7, is §6's load-bearing clause.** CF-8 (`spec/SPECIFICATION.md:7300`, `[FROZEN]`)
   *requires* the store to accept an append whose condition carries a tag no stored event carries,
   which is what makes the `is_ok()` assertion honest rather than incidental. CF-7 stays cited as
   the mirror failure the fence is deliberately not.
4. **The 22-character `##` budget does not bind on this surface**, so `## Where your streams went`
   ships at 23 characters. The budget's mechanism is rustdoc's 200px sidebar TOC ellipsis
   (`_design.md:575-584`); the approver's condition (2) records that the step and bridge surfaces
   render as markdown (`:1039`). Renaming would change the slug and break the only anchor the DT-1
   decision has. Recorded as an exception with its reason, not taken silently.
5. **`Tags::from_pairs`, not `Tags::from_iter`.** The design's snippet cannot compile;
   `tag.rs:304` is the working spelling and the worked example already uses it. Resolved here so no
   compile cycle is spent on it (EC-008).
6. **The need token is `explanation`, and `how-to` is the alternative that lost.** Under `how-to`
   the DT-1 anchor and the wrong-side contrast both read as a second need on the page, which is
   HS-P0021's own anti-pattern 4. If HS-P0021's reviewer procedure disputes it, that routes to
   HS-P0021 (EC-005) rather than being re-argued here.
7. **EC-002's fallback test file is named now**: `xtask/tests/bridge_guard.rs`. It is deliberately
   **outside** the front half's PR-boundary fence, so taking the fallback is a scope change
   recorded through `redkiln advance` — a human decision — rather than a quiet fence edit. Naming
   it before it is written is the same discipline `merge-forward-preflight/spec.md:206-214` sets
   for a conflict resolution.
8. **`xtask/src/narrative.rs` does not exist in this worktree today.** `xtask/src/` holds ten
   files and that is not one of them. This is not a defect in the spec — it is HS-P0020's mount,
   and its absence at implementation time means EC-001: **halt and report**, never stub.
9. **Tier 4 and IQ-3/IQ-5 are explicitly not this story's**, and are recorded as scope lines in
   the Interaction quality and Tests tables rather than omitted, so a reviewer reading this spec
   alone does not read their absence as a gap.
10. **No conformance rule, no clause, no public item, no `.kb/` atom.** Naming a conformance rule
    here would be decorative by CLAUDE.md's own test, and the initiative is additive: this page
    cites frozen clauses and amends none.
