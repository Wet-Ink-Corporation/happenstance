---
item: HS-S0161
stage: spec
created: 2026-08-17T13:16:16.594Z
updated: 2026-08-17T13:16:16.594Z
template_sig: 87bbf1d0
rendered_sig: 59f02ebd
---

# Spec — The two second questions, walked and recorded

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` — BR-16, AC-06, AC-09; DoD scenario 9 |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` — the DAG, the warranted briefs, the merge-forward rule |
| Project (gold source) | `.bklg/docs-that-teach/reach-and-adapter-path/project.md` — AC-005, DR-4, DR-10; DoD item 4 |
| **This spec** | `.bklg/docs-that-teach/reach-and-adapter-path/second-question-walk-records/spec.md` |
| Key brief — architecture + UX (one file) | `.bklg/docs-that-teach/reach-and-adapter-path/_decomposition.md` — UX brief Journey A (A0→A3), interaction invariants 3, 4, 5, the accessibility floor, UX-AC-04/08/09/11 |
| Key brief — grounding (verified dependency state) | `.bklg/docs-that-teach/reach-and-adapter-path/_grounding.md` |
| Signed-off design (**binding**) | `.bklg/docs-that-teach/reach-and-adapter-path/_design.md` — surface `evaluator-onward-links` and its three declared states; `## States` (**Empty**, **Refused**); anti-patterns 3, 5, 6, 9, 10 |
| Story map (slice, one-line, dependencies) | `.bklg/docs-that-teach/reach-and-adapter-path/_storymap.md` — backbone **B6**, slice `reach-walks`, Coverage AC-005 row, standing constraints |
| This story's discover artifact | `.bklg/docs-that-teach/reach-and-adapter-path/second-question-walk-records/discover.md` |
| The story that named the questions and built the hops | `.bklg/docs-that-teach/reach-and-adapter-path/evaluator-onward-links/spec.md` — Q-A, Q-B, register rows P4/P5, the origin page and the two destination headings |
| Roadmap pointer | none. `RUNBOOK.md` is the library's phase plan and carries no documentation-reach phase; the plan of record for this work is the initiative above. |

**No testing brief exists for this project, deliberately** — `.bklg/docs-that-teach/_decomposition.md`
("Warranted briefs") withholds one because AC-004, AC-005 and AC-009 are *observed walks*, not
automated checks. This story is one of the three instruments that omission is defensible because of.

## One-line PR slice

Each of the two named second questions is walked keyboard-only from the first-question page to the
passage that answers it, one dated record per question, and any dead end is recorded as a failed
walk rather than quietly re-walked.

## Executive summary

This PR lands **two dated walk records and one mount** — `walk-record-q-a.md` and
`walk-record-q-b.md` in this story's folder, plus their registration in
`.bklg/docs-that-teach/reach-and-adapter-path/project.md`'s `## Companions`, so the project's
Definition-of-done item 4 resolves for its AC-005 walk against files rather than a memory.

The delta over what already exists: slice `front-door-reach` will have *installed* the reach —
`front-door-pointer` puts the guide on both front-door surfaces, and `evaluator-onward-links` names
Q-A and Q-B and installs one in-passage link per question from the origin page to the answering
passage, filing register rows P4 and P5. Nothing yet **observes** that a reader with the question in
their head actually gets carried. `_storymap.md`'s backbone row **B6** puts every walk in its own
slice for exactly that reason: *"Putting a walk in the same slice as the surface it walks would make
the author their own witness, which is the one thing those criteria forbid."*

This story is that separate pair of hands. It changes no code, no public item, no register row and no
rendered surface. It produces evidence, and the evidence is falsifiable in the one direction that
matters: if either hop does not carry the walker, the record says so, the finding is routed by story
slug, and slice `front-door-reach` is not done.

## Context pack

Everything below is a decision already taken elsewhere and binding here. Read this section and you
can start; the anchors are for depth, not for orientation.

**1. This story walks; it does not choose.** The two questions are already named, and naming them is
somebody else's finished work: `_storymap.md`'s Coverage table splits project AC-005 in two — *"The
first names the two questions and builds the hops (DR-4 requires naming before answering); the second
walks them, and must be a different pair of hands."* `evaluator-onward-links` fixes them as **Q-A**
(*"Is 'dynamic' just a polite word for unstructured — where is the structure?"*, dynamism mistaken
for chaos) and **Q-B** (*"Is DCB a modelling technique or a routing technique?"*), each traced to an
independently observed stall point in research 03, with the third stall point recorded as
deliberately not walked. Re-deriving, re-wording or substituting a question here is the wrong
implementation `discover.md` names in one line: *"Two questions chosen because their answers are
known to exist, rather than because a reader would actually ask them."* If a question as named turns
out to be unaskable at the origin page, that is a **finding against `evaluator-onward-links`**, not a
licence to pick a third.

**2. One record per question, and merging them destroys the instrument.** The story map's own
sentence is *"one dated record per question"*. Two questions, two files, two dated walks. A single
combined record makes a failure on Q-B invisible behind a success on Q-A, and project AC-005 asks for
a *path per question* (`project.md`, Acceptance criteria: *"Recorded as a path per question"*).

**3. The session is single and unforgiving, and that shapes where the walk starts.** Persona 3's
*"entire journey happens inside one reading session, with no second attempt if the first one fails
silently"* (`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md`, Cross-persona
tensions). The failure state being tested is therefore **not a blank page — it is a good page with no
exit** (`_decomposition.md`, UX brief, Journey A). So the walker reaches the origin page the way a
reader does, continuing from the front door, and the record states how they got there; the
*observation under test* begins the moment the origin page has answered question one and the walker
has formed Q-A or Q-B. Being handed a URL is not a walk of Journey A's A1→A2 transition, it is a
walk of A2 alone.

**4. This story's walk and the front-door walk are one reader's session and two records.** Its
slice-mate `front-door-walk-record` observes project AC-004 (front door → narrative material);
this story observes AC-005 (first-question page → the answering passage). They are contiguous in a
reader's experience and separate as evidence, because the criteria are separate and one failing must
not be masked by the other passing. If the same person walks both — permitted, and likely, since the
slice is implemented in one context — each of this story's records **cites the front-door record as
its prefix and discloses what that prefix taught the walker**. An undisclosed prefix is the
contamination the cold-start protocol exists to surface.

**5. The claim is bounded, in the record's own words.** `project.md`'s risk table and
`_storymap.md`'s standing constraints agree verbatim: *"these walkers are non-authors, not
non-insiders… Evidence that the path exists is not evidence that a stranger finds it — that claim is
HS-P0024's alone (BR-14)."* And the questions themselves are bounded the same way — the honest claim
is *"two plausible second questions were walked"*, never *"the evaluator's real second question is
answered"*, because no evaluator has been observed. One enthusiastic sentence here spends a sibling
project's only instrument in advance and cannot un-spend it.

**6. Landing on the *passage* is the criterion, not landing on the page.** UX-AC-08 requires each
record to name *"the destination passage, not just the page"*, and UX invariant 3 is the rule behind
it: *land on the answer, not the top of the page*. `evaluator-onward-links` binds each hop to a
heading fragment — Q-A to `## Tag, query, fold, guard` and Q-B to `## Where your streams went` on
HS-P0022's bridge page. Arrival is proved by naming the heading landed on **and** quoting the
sentence in that passage that answers the question. "I ended up on the bridge page" is not arrival.

**7. Reversibility and no dead ends are checked at the landing, not assumed.** Invariant 4 —
*"the destination of each hop states what it assumes the reader has already read"* — is falsified by
*"arriving at any destination in an AC-005 or AC-009 walk and being unable to name the page that
should have preceded it."* Invariant 5 — *"every walked path terminates in a page that either answers
the question or names the next hop"* — is falsified by *"any AC-005 path whose terminal page does
neither — recorded as a failed walk, not quietly re-walked."* Both are literally about this story;
the record answers them per walk, in words, on arrival.

**8. A failed walk is a successful execution of this story; a blocked walk is not a walk at all.**
`_design.md`'s `## States` distinguishes them. **Empty** — the destination does not exist yet, in
which case *"the pointer is not installed"* and there is nothing to walk: halt, record the block
against the dependency, do not simulate. **Refused** — *"a walk that found a dead end. Expressed as a
recorded failed walk, never a quiet re-walk (AC-005, DR-10)."* A later attempt after somebody else's
fix is a **new dated entry citing the failed one**, never an edit of it. This is the single behaviour
that makes these records capable of failing, and a record that cannot fail is decorative.

**9. Keyboard-only, and the record names the keys.** The accessibility floor is explicit that *"each
of the three observed walks (AC-004, AC-005, AC-009) is performed keyboard-only, and the dated record
says so"*, and UX-AC-08 repeats it for this walk specifically. Note a narrowness worth not inheriting:
UX-AC-09's parenthetical lists only the AC-004 and AC-009 records; the floor and UX-AC-08 both cover
AC-005, and **this story honours the broader statement** — walker named, relationship stated,
keyboard-only declared, keys named per hop. A record that says "keyboard-only" and names no key has
asserted the floor rather than observed it.

**10. Without leaving the surfaces a Rust developer already reads.** Project AC-005 carries this
clause and it is a real constraint on the walk, not scenery: the permitted surfaces are the rendered
markdown pages under HS-P0020's pinned tree `docs/`, the crate's rustdoc and README, and rustdoc's
own built-in affordances (search `S` / `/`, plain links, the destination's own heading structure).
Reaching the answer via the backlog, the `_design.md`, a `git grep` of the repository, or a
third-party site is **not** a successful walk — it is evidence the installed path did not carry the
reader, and it is recorded as such.

**11. This story installs nothing and repairs nothing.** It renders no surface from `_design.md`'s
`## Surfaces` block; it *observes* `evaluator-onward-links` in its three declared states —
`link-in-passage`, `destination-fragment-landing`, `dead-end-baseline`. It appends no register row
(P4 and P5 are `evaluator-onward-links`'s), authors no answering content (HS-P0022's), and fixes
nothing it finds. *A witness who edits the thing they are witnessing has destroyed the observation.*
`_storymap.md`'s standing constraints still apply to the artefacts it does write: no widget, no raw
HTML, no folded content, self-describing link text, no bare URL into this repository's own tree.

**12. Nothing in the gate checks any of this, and that is stated rather than papered over.**
`_design.md`'s `## Density budget` names the gap in its own words: *"Nothing in `cargo xtask ci`
verifies keyboard-only reachability or self-describing link text; AC-004, AC-005 and AC-009 are dated
observed walks by design, and the absence of a linter is stated rather than papered over."* The
verification here is a human reading each record against its own stated protocol, plus
`cargo xtask ci --fast` proving the tree is unchanged in the ways this story promises it is unchanged.

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `capability` — a user-observable slice; the user is Persona 3 and what they observe is whether the hops built by slice `front-door-reach` actually carry a reader from a formed question to its answer. |
| **Slice / milestone** | `reach-walks` (`_storymap.md`, Slices). |
| **Slice-mate** (implemented in one context, mounted together) | `front-door-walk-record` (HS-S0160), which observes project AC-004. Merge order within the slice: `front-door-walk-record`, then this story (`_storymap.md`, Merge order, step 5) — which is also the reader's own order, and is why context-pack item 4 requires the prefix disclosure. |
| **Mount point** | `.bklg/docs-that-teach/reach-and-adapter-path/project.md` — the project item's body, where the two records are linked under `## Companions` and where Definition-of-done item 4 (*"The three walks (AC-004, AC-005, AC-009) each have a dated record in this project's artefacts, each naming its walker and their relationship to the work"*) stops being a promise and starts resolving for AC-005. **Body prose only**; the `redkiln` CLI owns the frontmatter and a `PreToolUse` hook denies edits to it. |
| **Wires into** | The surfaces its dependencies land, consumed **as a reader at their rendered paths, never as source**: the origin page that answered question one (`evaluator-onward-links`'s mount point — HS-P0022's `opening-encounter`, derived as `docs/first-encounter.md`, **bound at implementation to the path actually registered in the narrative harness**), the two in-passage links it installs, and their destination headings `## Tag, query, fold, guard` and `## Where your streams went` on HS-P0022's bridge page (derived as `docs/carry-your-invariant.md`). Also the front-door pointer on `crates/happenstance/src/lib.rs` and `crates/happenstance/README.md` (rows P1/P2), which is how the walker arrives at the origin page at all. |
| **Design-system primitives consumed** | None installed; the primitives are *exercised*. Plain in-tree markdown links with heading fragments, the destination page's own heading structure, rustdoc search (`S` / `/`), and `#[doc(alias)]` keys where present — all from `_decomposition.md`, UX brief, *Design-system primitives*. The records themselves are composed prose plus one hop table each. |
| **Renders surfaces** | **none.** This story renders no surface id from `_design.md`'s `## Surfaces`. It **observes** `evaluator-onward-links` in its three declared states (`link-in-passage`, `destination-fragment-landing`, `dead-end-baseline`) and passes through `crate-root-front-door` / `readme-front-door` on the way in. Claiming to render either would double-count work its two dependencies own. |
| **Public items** | **none.** `_design.md`'s `## Items` block declares `path: ""` for the whole project; this story adds no code, no attribute and no manifest entry. |
| **Conformance rule(s)** | **none, and it is not adapter-observable.** No port, bound, future or value type changes, so nothing in `crates/happenstance-testkit/` can observe this story. Per `CLAUDE.md`, a rule added here would be decorative — no adapter could fail it. |
| **Clause(s)** | **none discharged, none amended.** No `spec/SPECIFICATION.md` clause is edited and no `[FROZEN]` clause is touched. `cargo xtask spec-trace` is run to prove this story disturbed nothing, not to discharge anything; the clause-pin obligation is `store-error-site-rewrite`'s (project AC-010). |
| **Advances DoD scenario** | **Initiative DoD scenario 9** — *"The evaluator's second question is walked. Starting from the page answering the first question, at least two plausible second questions are followed to a page that answers them, without dead ends and without leaving the surfaces a Rust developer already reads."* This story is the **entire observation half** of that scenario; the installation half is slice `front-door-reach`. It also closes project **DoD item 4** for one of its three walks, and is the human guard `evaluator-onward-links`'s own integration contract names for its hops. |
| **External gates** (hard) | Slice `front-door-reach` must have merged, which itself sits behind HS-P0022's pages existing and being registered and HS-P0020's tree being pinned (`_storymap.md`, Merge order, step 3). If the links are not installed — including the legitimate case where `evaluator-onward-links` correctly declined to install one into a missing passage (`_design.md` `## States`, **Empty**) — this story is **blocked, not failed**. |

**Delivered mounted, not as an isolated component.** Two walk records sitting unlinked in a story
folder are the documentation-medium equivalent of a component that compiles and is never rendered:
nothing reads them, nothing depends on them, and the project's DoD item 4 goes on failing while two
green files sit on disk. The `## Companions` link in `project.md` is the mount, and it is part of
this PR.

## PR boundary

`redkiln verify --grain story` reads the first fenced block under this heading and fails on any file
changed outside it.

```
.bklg/docs-that-teach/reach-and-adapter-path/second-question-walk-records/**
.bklg/docs-that-teach/reach-and-adapter-path/project.md
```

**In this PR**

- `walk-record-q-a.md` — the dated, first-person record of the Q-A walk: cold-start protocol, how the
  origin page was reached and what the front-door prefix taught the walker, the formed question in
  the walker's own words, the hop table, the arrival check at the named passage, the reversibility
  and dead-end checks, the keyboard-only statement with keys named, the walker and their relationship
  to the work, and the bounded-claim line.
- `walk-record-q-b.md` — the same, for Q-B, as a **separate** dated record.
- Any *failed* walk, as its own dated entry appended inside that question's file, with the finding
  routed by owning story slug.
- The link from `project.md`'s `## Companions` — body prose only, one bullet per record, with
  self-describing link text.
- This story's own `_ledger.md` and stage artifacts.

**Explicitly not in this PR**

- **Any file under `crates/`, `docs/`, `xtask/`, `standards/`, `spec/` or `.kb/`.** This story reads
  those surfaces and changes none of them. The absence of those globs above is what makes that
  checkable rather than promised.
- **Any repair of what the walks find.** A missing fragment, a two-hop path, a link that resolves
  nowhere, a passage that answers a different question — each is recorded and routed by slug
  (`evaluator-onward-links` for the hops and their register rows, `front-door-pointer` for the front
  door, HS-P0022 for answering content, HS-P0020 for registration and the tree). None is fixed here.
- **Choosing, re-wording or adding a question.** Q-A and Q-B are named by `evaluator-onward-links`;
  the third stall point is on record as deliberately not walked, and this story does not re-open that.
- **Register rows.** P4 and P5 belong to `evaluator-onward-links`; this story appends none and edits
  none, and it does not render the register — it is build-time data by design (`_design.md`,
  `## Transience policy`; anti-pattern 14).
- **The other two walks.** AC-004 is `front-door-walk-record`'s (slice-mate) and AC-009 is
  `error-site-walk-record`'s, in slice `adapter-error-walk`. Their evidence is not merged into these
  records.
- **The non-insider claim.** HS-P0024 `comprehension-evidence` owns the friction log and the only
  method that supports it (BR-14).

**Permitted wiring.** The implementer may also touch the composition-root file named in the
Integration contract — `project.md`'s `## Companions` body prose — to mount this slice. That is not
scope drift; it is what "mounted" means here.

**Merge DoD one-liner.** Merged when two dated, keyboard-only, first-person records exist — one per
named question — each naming a walker who authored neither dependency, each reaching its answering
*passage* in one hop from the origin page (or closing as a routed failed walk), each carrying its
arrival, reversibility and bounded-claim statements, both linked from `project.md`'s `## Companions`
so project DoD item 4 resolves for AC-005 — with `cargo xtask ci --fast` green and no file changed
outside the boundary.

## Behavior and interfaces

The "interface" in this medium is each record's own shape: the fields a reader needs in order to
re-walk it and disagree with it. The table below is the contract.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **Two records exist, one per question, at known paths, each dated** | `walk-record-q-a.md` and `walk-record-q-b.md` in this story's folder, each carrying the ISO date of the *walk*, not of the commit. Never one merged file: a failure on one question must be visible independently of the other. | `_storymap.md` slice row (*"one dated record per question"*); `project.md` AC-005 (*"Recorded as a path per question"*) and DoD item 4 |
| **The questions are consumed, not chosen** | Each record opens by quoting its question as `evaluator-onward-links` named it (Q-A dynamism-as-chaos; Q-B modelling-versus-routing) and cites that story's named-questions record. A question the walker could not plausibly have formed at the origin page is a **finding against that story**, recorded, not swapped for another. | `_storymap.md` Coverage, AC-005 row; `.bklg/docs-that-teach/reach-and-adapter-path/evaluator-onward-links/spec.md` (Q-A / Q-B table); `discover.md`, *The wrong implementation* |
| **The cold-start protocol is stated before the first hop** | What the walker had at t=0 (what `cargo add happenstance` shows, and the guide reached from it), how they arrived at the origin page, and what was forbidden: this spec, the sibling story specs, `_design.md`, the pointer register, the backlog, and asking either author. Contamination is disclosed, never omitted. | `initiative.md` DoD 9 (*"Starting from the page answering the first question"*); `project.md` AC-005; `discover.md`, *The wrong implementation* |
| **The front-door prefix is disclosed, not hidden** | If the walker also performed the `front-door-walk-record` walk, or otherwise saw the guide first, the record cites that record and states in one sentence what it taught them about where things are. The two records stay separate artefacts. | `_storymap.md` backbone **B6** and slice `reach-walks`; `personas-and-journeys.md`, Cross-persona tensions (*"Persona 3's entire journey happens inside one reading session"*) |
| **The question is formed at the origin page, in the walker's words** | Before any hop, the record states the question the walker actually had after reading the origin passage, and whether it matches the named one. This is the A1 transition (*question formed*) and it is the part a handed URL skips entirely. | `_decomposition.md`, UX brief, Journey A states A0→A3 |
| **One hop, per question, counted** | The hop table reaches the answering passage in exactly **one** link traversal from the origin page. Two hops is a finding, not a rounding error; an intermediate index page or a "start here" detour is a failed walk. | `project.md` AC-005; `_design.md` `## What it costs a caller` (*"One hop, never two"*); `evaluator-onward-links` behaviour row *One hop, per question, from the origin page* |
| **Every hop is a row: what was on screen, what was pressed, where it landed** | A hop table per record — ordinal · surface · affordance and keys used (plain link / rustdoc search `S` or `/` / heading fragment) · destination **passage**. No empty cells. | `_decomposition.md`, accessibility floor; UX-AC-08 (*passage, not just the page*) |
| **Keyboard-only, and it says which keys** | One explicit statement that no pointing device was used, plus the keys named per hop. Named narrowly: the floor covers all three walks even though UX-AC-09's parenthetical lists only two. | `_decomposition.md`, accessibility floor (*"each of the three observed walks (AC-004, AC-005, AC-009) is performed keyboard-only, and the dated record says so"*); UX-AC-08 |
| **Arrival is proved at the passage, not the page** | On landing, the record names the heading fragment it landed on — `## Tag, query, fold, guard` for Q-A, `## Where your streams went` for Q-B — and quotes the sentence in that passage that answers the question. An arrival the record can only describe as "the bridge page" fails this row. | UX invariant 3 and UX-AC-04, `_decomposition.md`; `_design.md` `## Composition`, `evaluator-onward-links`; `.bklg/docs-that-teach/application-author-path/_design.md` (the two headings) |
| **Reversibility is checked on arrival** | The record answers, in words, whether the destination states what it assumes the reader has already read — i.e. whether the walker could retrace without browser history and could tell if they arrived out of order. A "no" is a finding against the destination's owner. | `_decomposition.md`, invariant 4 (*falsified by "arriving at any destination… and being unable to name the page that should have preceded it"*) |
| **No dead end** | The terminal passage either answers the named question or names the next hop. Neither, and the entry closes as a failed walk. | `_decomposition.md`, invariant 5; `_design.md` `## States`, **Refused** |
| **Only surfaces a Rust developer already reads were used** | The record lists every surface touched. A hop through the backlog, `_design.md`, the register, a repository-wide `grep`, or an external site is recorded as such and **fails the walk** rather than completing it. | `project.md` AC-005 (*"without leaving surfaces a Rust developer already reads"*); `initiative.md` DoD 9 |
| **A failed walk is recorded as a failure and routed by slug** | The entry closes as a failure, the finding names the owning story slug, nothing is repaired in this PR, and a later attempt is a **new dated entry citing the failed one** — never an edit of a closed entry. | `_decomposition.md`, invariant 5; `_design.md` `## States`, **Refused**; `project.md` DR-10 |
| **A blocked walk is distinguished from a failed one** | If the links are not installed — including the legitimate not-installed case of `_design.md`'s **Empty** state — the story records the block against the dependency slug and stops. Simulating a destination produces a record that measures scheduling. | `_design.md` `## States`, **Empty**; `_grounding.md`, *Sibling-project dependency state*; `_storymap.md` Merge order, step 3 |
| **The walker is named, with their relationship to the work** | Name, and one sentence: that they authored neither `evaluator-onward-links` nor `front-door-pointer`, and what they knew of the surface before starting. | `project.md` DR-10 and DoD item 4; `_decomposition.md` UX-AC-09 |
| **The claim is bounded in the record's own words** | An explicit line per record: this is evidence that the path exists for a non-author, not that a stranger finds it; and these are two *plausible* second questions, not the evaluator's observed one. No sentence anywhere may imply otherwise. | `project.md` risk table (the two AC-005 rows); `_storymap.md` standing constraints; `initiative.md` BR-14 |
| **The records are mounted** | `project.md`'s `## Companions` links both, with link text naming each record and the question it walked — no "here", no bare URL, no *See also* block. | `project.md` `## Companions`; `_decomposition.md` accessibility floor; `_design.md` anti-patterns 3 and 5 |
| **The records are composed, not a wall of prose** | Prose plus one hop table per record, heading ladder unskipped, nothing folded or tabbed, no raw HTML or inline `style=`, no table of fewer than three rows used as a navigation device. Every mechanical assertion in this spec passes on one unpunctuated paragraph containing the right strings; this row is what fails that record. | `_design.md` anti-patterns 3, 5, 6, 9, 10; `_storymap.md` standing constraints; UX-AC-11 |
| **Nothing else in the tree moved** | `cargo xtask ci --fast` green (project DoD item 1) and `cargo xtask spec-trace` clean, run to prove this story disturbed neither — not to prove anything about the records. | `CLAUDE.md`, Commands; `project.md` DoD items 1 and 2 |

**Acceptance-criteria ids this story enumerates** (the table is authored in the second pass, against
exactly this set): `AC-001` two dated records, one per named question, consumed not chosen ·
`AC-002` cold start and origin arrival, with the front-door prefix disclosed · `AC-003` one hop per
question, arriving at the named passage and quoting the answering sentence · `AC-004` keyboard-only
with the affordance and keys named per hop, and only surfaces a Rust developer already reads ·
`AC-005` reversibility and no dead end checked on arrival, in words · `AC-006` a failed walk recorded
as a failure, routed by slug, append-only, and distinguished from a blocked walk · `AC-007` walker
named with their relationship, and the claim bounded to non-author and to *plausible* questions ·
`AC-008` both records mounted from `project.md`'s `## Companions` so project DoD item 4 resolves for
AC-005 · `AC-009` the records are composed artefacts — hop tables that render as tables, fixed entry
order, nothing folded, no forbidden device.

## Data and migrations

**N/A — no schema, no store, no persisted state, no manifest change, and no migration.**

Stated precisely rather than left as two letters, because there is exactly one data artefact in this
project and this story's relationship to it is *read-only by design*:

- **The pointer register** — `_design.md`'s `## Transience policy` fixes it as build-time data in the
  shape of `SUMMARIES` at `xtask/src/lint_constitution.rs:64` and `check_summaries` at `:463-473`,
  never a rendered page (anti-pattern 14). Its schema is `pointer-policy-and-inventory`'s and rows P4
  and P5 are `evaluator-onward-links`'s. **This story adds no row, edits no row, and does not consult
  it during a walk** — consulting it would tell the walker where the links are, which is precisely the
  knowledge the cold-start protocol excludes. It may be read *afterwards*, when routing a finding.
- **The records themselves are deliberately not a schema.** No frontmatter beyond what the CLI owns on
  the items, no YAML block, no id namespace: prose plus one hop table each, so a reviewer who cannot
  run `cargo` can still read them and disagree. What is fixed is the *order* of the sections inside a
  dated entry, because the order is the instrument (see `## Behavior and interfaces`).
- **No store adapter, no event, no projection, no `SequencePosition`** is read or written; no crate
  manifest, feature or public item changes; both port flavours and the `wasm32` target are untouched.

## Acceptance criteria

Framed from Persona 3's intent — the evaluator, reading on a bounded budget, inside a single session
(`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md`, Persona 3 and
Cross-persona tensions). Every row is a goal crossing the whole stack: a question already in a
reader's head reaching the passage that answers it, and the *evidence* that it did. `redkiln verify`
extracts these by the leading `| AC-001 |` cell, so **every** blocking invariant is a row here and
none is a prose bullet elsewhere in this spec.

Throughout, "the two records" means
`.bklg/docs-that-teach/reach-and-adapter-path/second-question-walk-records/walk-record-q-a.md` and
`walk-record-q-b.md`; "the origin page" is `evaluator-onward-links`'s mount point (derived
`docs/first-encounter.md`, **bound at implementation to the path the narrative harness actually
registers**); "the destinations" are the headings `## Tag, query, fold, guard` (Q-A) and
`## Where your streams went` (Q-B), bound the same way.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an evaluator whose two plausible second questions were named by somebody else before anything was authored (DR-4), and no evaluator has ever been observed, **WHEN** this story walks, **THEN** exactly two files exist — one per question, each carrying the ISO date of the *walk* and never of the commit — and each opens by quoting its question **as `evaluator-onward-links` named it** (Q-A: *"Is 'dynamic' just a polite word for unstructured — where is the structure?"*; Q-B: *"Is DCB a modelling technique or a routing technique?"*) and citing that story's named-questions record; no question is re-derived, re-worded, substituted, merged with the other, or replaced by the third stall point. | Both files present in the PR and named in `_ledger.md` evidence; `rg -n "Is 'dynamic' just a polite word|modelling technique or a routing technique"` returns one hit per record; `rg -n "_named-questions.md"` returns a citation in each; read against `_storymap.md` Coverage AC-005 row (*naming* is the other story's, *walking* is this one's) and `discover.md`'s named wrong implementation; **T3** proves no third record and no edit to the naming record |
| AC-002 | **GIVEN** Persona 3's whole journey happens inside one reading session with no second attempt, and the failure under test is *a good page with no exit* rather than a blank page, **WHEN** the walker begins, **THEN** each record states — before its first hop — what they had at t=0 (what `cargo add happenstance` shows, and the guide reached from it), how they arrived at the origin page **as a reader continuing from the front door rather than by being handed a URL**, what was forbidden to them (this spec, the sibling story specs, `_design.md`, the pointer register, the backlog, and asking either author), and the question they actually formed at the origin page in their own words; and if the walker also performed the `front-door-walk-record` walk or otherwise saw the guide first, the record **cites that record and states in one sentence what the prefix taught them**. | `rg -n "front-door-walk-record/walk-record.md"` returns a prefix-disclosure citation in each record, or an explicit statement that no prefix existed; the cold-start and forbidden-sources block present in each; **T6** recorded read against `_decomposition.md` UX brief Journey A states A0→A1 and `personas-and-journeys.md` Cross-persona tensions; a record whose arrival at the origin page is "I opened the URL" fails this row |
| AC-003 | **GIVEN** an evaluator who has just had question one answered well and has formed a sharper one, **WHEN** they follow the in-passage link `evaluator-onward-links` installed, **THEN** each record's hop table shows **exactly one** link traversal from the origin page to the answering passage — no index page, no "start here" detour, no second hop — the record names the **heading fragment it landed on**, and it **quotes the sentence in that passage that answers the question**. An arrival the record can only describe as "the bridge page" is not arrival and closes as a failed walk under AC-006. | Each record's hop table has exactly one traversal row per dated entry; `rg -n "Tag, query, fold, guard"` / `rg -n "Where your streams went"` return the landed heading in the respective record **and** in the registered destination file; the quoted sentence located verbatim in that file; `cargo xtask narrative` (HS-P0020's step) green over the registered tree once landed; **T6** read against UX invariant 3, UX-AC-04 and UX-AC-08 |
| AC-004 | **GIVEN** the accessibility floor converts keyboard-only reachability from a claim into an artefact the project already owes, and project AC-005 bounds the walk to *surfaces a Rust developer already reads*, **WHEN** either walk is performed, **THEN** each record carries one explicit statement that no pointing device was used **and names the affordance and keys per hop** (Tab/Shift-Tab, Enter, rustdoc search `S` or `/`, the destination's own heading fragment), with no empty cell in the hop table; and each record lists **every surface touched**, all of which are the rendered pages of HS-P0020's pinned tree, the crate's rustdoc, or its README — a hop through the backlog, `_design.md`, the pointer register, a repository-wide `grep`, or a third-party site is recorded as such and **fails the walk** rather than completing it. | `rg -n "keyboard-only"` returns the explicit statement in each record; no hop-table cell empty (**T7**); the surfaces-touched list read against project AC-005's clause and initiative DoD scenario 9; **T6** recorded read against `_decomposition.md` Accessibility floor (*"each of the three observed walks (AC-004, AC-005, AC-009) is performed keyboard-only, and the dated record says so"*) — a record that says "keyboard-only" and names no key fails this row |
| AC-005 | **GIVEN** UX invariant 4 is falsified by *"arriving at any destination in an AC-005 or AC-009 walk and being unable to name the page that should have preceded it"* and invariant 5 by a terminal page that neither answers nor names a next hop, **WHEN** the walker lands, **THEN** each record answers **both, in words, at the landing**: whether the destination states what it assumes the reader has already read — i.e. whether the walker could retrace without browser history and could tell they had arrived out of order — and whether the terminal passage answers the named question or names the next hop. A "no" to either is written as a finding against the destination's owner, never left as silence. | `rg -n` locates a reversibility answer and a dead-end answer in each record, each as prose rather than a tick; **T6** read against `_decomposition.md` interaction-quality invariants 4 and 5 and `_design.md` `## States`; a record carrying only "arrived successfully" fails this row because it has asserted the invariants rather than checked them |
| AC-006 | **GIVEN** a record that cannot fail is decorative, and `_design.md` separates a **Refused** walk (a dead end that was found) from an **Empty** one (a destination that does not exist, so there is nothing to walk), **WHEN** a walk does not reach its answer, **THEN** the entry **closes as a failed walk**, names the owning story slug it is routed to (`evaluator-onward-links`, `front-door-pointer`, HS-P0022 or HS-P0020), repairs nothing in this PR, and any later attempt after somebody else's fix is a **new dated entry citing the failed one — never an edit of a closed entry**; and where the link was legitimately not installed, the story records a **block against the dependency slug and stops**, rather than simulating a destination. | `git log -p` over each record shows no commit that rewrites the body of a previously closed dated entry (append-only); each failure entry names a slug; **T3** proves nothing under `crates/`, `docs/`, `xtask/`, `standards/`, `spec/` or `.kb/` changed, so no finding was quietly repaired; **T6** read against `_design.md` `## States` (**Empty**, **Refused**) and project DR-10 |
| AC-007 | **GIVEN** everyone available to this project is an insider and BR-14 reserves the non-insider claim to HS-P0024 alone, **WHEN** each record is written, **THEN** it names the walker, states in one sentence that they authored neither `evaluator-onward-links` nor `front-door-pointer` and what they knew of the surface before starting, and carries an explicit bounded-claim line: this is evidence the path exists **for a non-author**, not that a stranger finds it; and these are **two plausible** second questions, not the evaluator's observed one. No sentence anywhere in this PR implies otherwise. | `rg -n "authored neither"` and the bounded-claim sentence present in each record; `rg -n "the evaluator's real second question\|a stranger\|newcomer\|proves that anyone"` over the whole diff returns nothing that overclaims; **T6** read against `project.md`'s two AC-005 risk rows, `_storymap.md` standing constraints and initiative BR-14; project DoD item 4's *"naming its walker and their relationship to the work"* satisfied for the AC-005 walk |
| AC-008 | **GIVEN** two records sitting unlinked in a story folder are this medium's equivalent of a component that compiles and is never rendered, **WHEN** the PR lands, **THEN** `project.md`'s `## Companions` body prose carries **one bullet per record**, each with self-describing link text naming the record *and* the question it walked (never "here", "this", "see this page", "docs", "read more", or a bare URL into this repository's tree), both relative links resolving to files that exist, project **Definition-of-done item 4** thereby resolving for its AC-005 walk — and the item's YAML frontmatter untouched, because the `redkiln` CLI is its only writer. | `git diff` over `project.md` shows changes **only** beneath the closing `---`, none inside the frontmatter block; each bullet's link target exists (**T7**); **T5** deny-list over the added link text returns nothing; `redkiln validate --kb && redkiln doctor` green; **T6** read against `_design.md` anti-patterns 3 and 5 and the `_decomposition.md` accessibility floor's self-describing-link-text rule |
| AC-009 | **GIVEN** every mechanical assertion above passes perfectly on one unpunctuated paragraph containing the right strings, **WHEN** a reviewer who cannot run `cargo` opens either record, **THEN** it is a **composed artefact**: prose plus one hop table per dated entry that renders as a real markdown table with no empty cells, the fixed section order of a dated entry held (cold start · formed question · hop table · arrival · reversibility · dead end · keyboard statement · walker and relationship · bounded claim), heading ladder unskipped, nothing folded or tabbed, no raw HTML and no inline `style=`, no bare URL into this repository's own tree, and no table of fewer than three rows used as a navigation device. | **T4** `rg -n "<details>\|<summary>\|<div\|<br\|style="` over both records returns nothing; **T7** structural check — each dated entry's required sections present in order, every hop-table row full; heading levels never skip; **T6** read against `_design.md` anti-patterns 3, 5, 6, 9, 10, `_storymap.md` standing constraints and UX-AC-11 |

**Traceability.** Project **AC-005** (*"Two named second questions are walked to an answer… no dead
ends, and without leaving surfaces a Rust developer already reads. Recorded as a path per
question."*) → **every row above**: AC-001 and AC-002 establish that the questions and the starting
point are the ones the criterion means; AC-003 is the *walked to an answer* half; AC-004 the
*without leaving surfaces* half; AC-005 and AC-006 the *no dead ends* half; AC-007 and AC-009 make
the record readable and honest; AC-008 makes it count. Project **DoD item 4** → AC-007 and AC-008.
Initiative **DoD scenario 9** → AC-003, AC-004, AC-005. No other project AC is claimed here, and in
particular **project AC-003 is not**: the register rows are `evaluator-onward-links`'s and this
story files none.

## Interaction quality

RFC §6.7/D6. The blocking invariants, in two families. **Each is carried by a row in the acceptance
table above** — this section says which row and how that row is actually falsified. Nothing here is
an additional requirement: a bullet in this section would get no ledger row, no gate and no test.

**Why this section is load-bearing here in particular.** This story renders no surface, so the
instinct is to skip it — and that instinct is exactly backwards. The invariants are not decorations
on something this story builds; **they are the thing this story measures**. `evaluator-onward-links`
could only *check* invariants 4 and 5 by refusing to install into a bad landing; this story is where
they are finally *observed*, on the assembled surface, by someone who did not build it. And the
record itself is a surface: `_design.md`'s `## Density budget` states plainly that nothing in
`cargo xtask ci` verifies keyboard-only reachability or self-describing link text, so an unstyled,
unpunctuated wall of prose containing every required string would satisfy every mechanical assertion
in this spec. AC-009 is what fails that record.

### STATE invariants

Two columns, because each invariant lands twice: on the **observed** surface (what the walk is a
measurement of) and on the **record** (what this story actually writes).

| Invariant | Carried by | How it is falsified |
| --- | --- | --- |
| **In-place before context-jump** — the reader's first question is answered without leaving the origin page, and the hop is an *offer* taken afterwards, never a step required to get unstuck | **AC-002**, **AC-003** | A record whose formed-question section shows the walker still needed question one answered when they left, or whose hop table shows a jump taken before the origin passage had finished answering. This is UX invariant 1 measured rather than asserted |
| **Non-occlusion** — nothing this story adds displaces anything | **AC-006**, **AC-008** | A later dated entry that rewrites or shortens a closed one (AC-006's append-only rule), or a `## Companions` edit that reorders, rewords or removes an existing bullet instead of adding two (`git diff` over `project.md`) |
| **Preserved focus, scroll and selection** — the landing puts the reader *at the answer*, and the answering sentence is selectable contiguous text a walker can quote | **AC-003** | An arrival at the top of a long page that the walker had to re-scan, or a record that names a page rather than a heading fragment, or an "answering sentence" the walker had to paraphrase because it was not selectable text (UX invariant 3, UX-AC-04) |
| **Reversibility** — every hop is walk-backable without browser history | **AC-005** | Arriving at a destination and being unable to name the page that should have preceded it, and the record failing to say so. This story checks and routes; it may not author the fix, because that prose is HS-P0022's (UX invariant 4) |
| **No dead ends** | **AC-005**, **AC-006** | A terminal passage that neither answers the named question nor names the next hop, recorded as anything other than a **failed walk** — a quiet re-walk is the named prohibition (UX invariant 5; `_design.md` `## States`, **Refused**; project DR-10) |
| **Keyboard reachability** — observed, not asserted | **AC-004** | A record asserting "keyboard-only" while naming no key, or a hop whose affordance cell is empty, or a hop that required a pointing device — which is a finding against the surface, not a footnote (accessibility floor, final bullet) |
| **Reversibility of the evidence itself** — a reader can disagree with the record without re-running anything | **AC-007**, **AC-009** | A record whose walker, relationship, forbidden-sources list or bounded claim is missing, so a reviewer cannot tell what the observation was worth |

### COMPOSITION invariants

Taken from the signed-off `_design.md`, which is binding. This story does not re-decide it; it
observes the surface that design produced, and it obeys the same rules in the artefacts it writes.

| Invariant | The design's own rule | Carried by |
| --- | --- | --- |
| **Presentation exists at all** — the hop the walker took was real composed prose, an inline link inside the answering passage, not a bare URL or a page-bottom list | `## Composition`, `evaluator-onward-links`: *one inline link, sentence-final, at the end of the passage that answered the first question* | **AC-003** |
| **Composition / placement** — the walk starts where a reader is, and the link is met in the passage rather than hunted for | `## Pattern decision`, `evaluator-onward-links` row: a "See also" block at page bottom is *"reached only after the reader has decided to leave"* | **AC-003**, **AC-002** |
| **Transience** — persistent chrome only; the walker opened nothing to reach a constraint, and never consulted the register | `## Transience policy`: zero revealed-on-hover-or-focus affordances installed; the register is **not rendered to a reader at all** | **AC-004**, **AC-009** |
| **Density budget, with its real numbers** — **one hop, never two**; the visible link text is ≥ 3 words and a noun phrase; the answer is on the landed-on passage, not somewhere below it | `## Density budget`, "Minimum legible size for the primary label"; `## What it costs a caller`, *"One hop, never two"* | **AC-003** |
| **Hierarchy** — the answering passage is primary and the link recessive; a link that outweighs its passage is a "next steps" block wearing a sentence | `## Hierarchy`, `evaluator-onward-links` row | **AC-003** |
| **Anti-pattern 3** — link text "here", "this", "see this page", "docs", "read more", or a raw URL | `## Anti-patterns`, 3 | **AC-008** (the `## Companions` bullets), **AC-009** (inside the records) |
| **Anti-pattern 5** — a "See also" / "Next steps" / "Further reading" block appended to any touched page | `## Anti-patterns`, 5 | **AC-008**, **AC-009** |
| **Anti-pattern 6** — a table of fewer than three rows used as a navigation device | `## Anti-patterns`, 6 | **AC-009** |
| **Anti-pattern 9** — any collapsed or folded element in added content | `## Anti-patterns`, 9 | **AC-009** |
| **Anti-pattern 10** — raw HTML or an inline `style=` in added markdown | `## Anti-patterns`, 10 | **AC-009** |
| **Anti-pattern 14** — the pointer register rendered as a page a reader can navigate to | `## Anti-patterns`, 14 | **AC-004** (the register is not a permitted walk surface), **AC-006** (this story renders none) |

**What the mock does and does not approve for the surface this story walks.**
`design/mock.html` renders `evaluator-onward-links` for **composition and density only** — HS-P0020's
tree had no renderer at design time, so those frames approve hop placement, hierarchy and word budget
and approve *nothing* about type, colour or measure (`_design.md`, "What the mock cannot be honest
about"; finding F5, dispositioned **tracked, not closed**). A walker must therefore **not** consult
the mock at all before walking — it is a source that would tell them where the link is, which is
precisely what the cold-start protocol excludes. It is a review aid for AC-009 afterwards, and never
a pixel target.

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| EC-001 | Slice `front-door-reach` has not merged, or the in-passage links are otherwise absent, so there is nothing to walk | **Blocked, not failed.** Record the block against the dependency slug and stop. A walk of a destination that does not exist measures scheduling, not reach (`_design.md` `## States`, **Empty**; `_storymap.md` Merge order, step 3) |
| EC-002 | `evaluator-onward-links` **correctly declined** to install a link, because the answering passage or its heading did not exist (its own AC-007 refusal) | Same as EC-001 — this is the legitimate not-installed case and it is a *block*, not a dead end. Record it as such, cite that story's recorded refusal, and do not walk a substitute path |
| EC-003 | A named question turns out not to be one the walker could plausibly have formed at the origin page | **A finding against `evaluator-onward-links`**, recorded in that question's record and routed by slug. **Not** a licence to pick a third question, re-word this one, or fall back to the excluded stall point (`discover.md`, *The wrong implementation*) |
| EC-004 | The answer is reached, but in **two** hops, or via an index page or a "start here" detour | **A failed walk**, closed as such and routed to `evaluator-onward-links`. Two hops is a finding, not a rounding error (`_design.md` `## What it costs a caller`, *"One hop, never two"*) |
| EC-005 | The link resolves to the top of the destination page, or to a fragment that resolves nowhere (anchor-slug drift) | **A failed walk**, routed to `evaluator-onward-links` (fragment binding is its AC-002 and its EC-008). Record the fragment as written and where it actually landed; do not hand-repair the anchor |
| EC-006 | The landing passage exists and is reached, but answers a *different* question, or terminates without answering and without naming a next hop | **A failed walk**, routed to **HS-P0022** as a content gap. Do not author the missing answer here (UX invariant 5; `_design.md` `## States`, **Refused**) |
| EC-007 | The destination states nothing about what it assumes the reader has already read, so the walker cannot tell whether they arrived out of order | The walk may still *complete*, but AC-005's reversibility answer is **"no"** and is written as a finding routed to HS-P0022. Recording "yes" because arrival succeeded is the failure this row exists to prevent (UX invariant 4) |
| EC-008 | The answer is only reachable by leaving the permitted surfaces — the backlog, `_design.md`, the pointer register, a repository-wide `grep`, or an external site | **A failed walk**, recorded with the surface that actually carried the walker named. This is the AC-005 clause *"without leaving surfaces a Rust developer already reads"* doing its job, and it is a finding, not a completion |
| EC-009 | The only person available to walk authored `evaluator-onward-links` or `front-door-pointer` | **Halt and escalate.** Do not walk. The whole instrument is the separation of hands (`_storymap.md` backbone **B6**; project DR-10); a self-witnessed walk is worse than no walk because it looks like evidence |
| EC-010 | The walker unavoidably knew the destination in advance — from the front-door walk, from a review, from writing a sibling spec | **Disclose, do not omit.** AC-002's prefix disclosure is the mechanism; a contaminated walk that says so is usable evidence with a stated bound, and an undisclosed one is not evidence at all |
| EC-011 | Some hop cannot be completed keyboard-only | **A failed walk**, routed to the surface's owning slug. The accessibility floor is a bar the surface must clear, not a condition on the walker; completing it with a mouse and mentioning it in passing converts a finding into a footnote |
| EC-012 | A previously failed walk is re-attempted after somebody else's fix lands | **A new dated entry citing the failed one**, appended below it. Never an edit, never a deletion, never a re-word of the closed entry. This is the single behaviour that makes these records capable of failing (project DR-10; `_design.md` **Refused**) |
| EC-013 | The merge-forward rule bites — the origin page, destination page, headings or front-door surfaces differ on the merged tree from what this worktree shows | Re-bind every path and heading against the **merged** tree before walking, and say in the record which tree was walked. Every path in this spec is *derived* and **not verifiable from this worktree** (`_storymap.md` Merge order, step 3; `_grounding.md`) |
| EC-014 | Editing `project.md` to mount the records trips the `PreToolUse` hook | The edit landed in the YAML frontmatter. Only the body beneath the closing `---` is yours; the `redkiln` CLI is the single writer of `id`, `stage`, `status`, `updated` and `links` (`CLAUDE.md`, "Where the work lives") |

## Non-functional

| id | Requirement | Why it is stated |
| --- | --- | --- |
| NF-001 | **No runtime cost and no build cost.** This story adds no crate, no module, no test binary and no gate step. `cargo xtask ci --fast` gains nothing from it and is run only to prove the tree is unchanged in the ways this story promises | A documentation-evidence story that slows the gate has spent a budget it never asked for |
| NF-002 | **No dependency, feature, manifest or MSRV movement.** Nothing here touches `Cargo.toml`, `rust-toolchain.toml` or any feature flag, so ADR-0003's `serde` constraint and ADR-0029's floor are untouched by construction rather than merely satisfied | The workspace's binding constraints are easiest to break in a change nobody thinks touches them (`CLAUDE.md`, Binding constraints) |
| NF-003 | **Both port flavours and `wasm32` are unaffected.** No type, bound, future, signature or public item changes, so `EventStore` / `SendEventStore` and every `wasm32` step are untouched | Stated once so a reviewer does not have to check (ADR-0001; `CLAUDE.md`, constraints 1 and 3) |
| NF-004 | **Accessibility floor, applied to the records themselves.** Nothing added carries meaning in colour, position or ASCII art; nothing animates; no contrast is overridden (no raw HTML, no inline style); link text is self-describing; the heading ladder never skips | `_decomposition.md`, UX brief, Accessibility floor — in a text medium the floor lands in specific, checkable places, and the artefacts this story writes are in that medium |
| NF-005 | **Evidence durability: append-only, forever.** A dated entry, once closed, is never edited. Corrections and re-walks are new dated entries that cite the old one, so the audit trail of what was true when is recoverable from the file alone | A record that can be silently improved after the fact measures the improver, not the surface (project DR-10) |
| NF-006 | **Reader budget: one hop, and one session.** The observation is bounded to what Persona 3 would spend — no path may need a second hop, and no walk may span sessions with the walker reading up in between | `_design.md` `## What it costs a caller`; `personas-and-journeys.md`, Cross-persona tensions |
| NF-007 | **Claim budget: two sentences, no more.** Each record's bounded-claim line is fixed in scope — non-author, not non-insider; *plausible* questions, not the evaluator's observed one — and nothing in the PR, including the commit message and the `## Companions` link text, may widen it | One enthusiastic sentence here spends HS-P0024's only instrument in advance and cannot un-spend it (initiative BR-14; `project.md` risk table) |

## Implementation notes (non-prescriptive)

Shape suggestions, not requirements. The acceptance criteria above are the contract.

- **Decide who walks before anything else, and write it down first.** The walker must not be the
  author of `evaluator-onward-links` or `front-door-pointer` (EC-009). Establishing that after the
  walk is how a slice quietly witnesses itself: the slice is implemented in one context, so the
  temptation is structural rather than careless.
- **Write the cold-start block before opening a single page.** What you have at t=0 and what is
  forbidden to you are only honest if they are recorded *before* the walk. Reconstructing them
  afterwards produces a protocol shaped like whatever happened.
- **Walk `front-door-walk-record` first if one person does both.** `_storymap.md`'s merge order runs
  it first and it is also the reader's own order. Then disclose the prefix here in one sentence
  rather than pretending to a cold start you no longer have.
- **Keep a live hop log while walking, not a reconstruction afterwards.** Ordinal, surface, what was
  on screen, keys pressed, where it landed. The keys are the part memory loses first, and AC-004 is
  the row that fails without them.
- **Quote the answering sentence by selecting it, not by paraphrasing it.** If it cannot be selected
  as contiguous text, that is itself a finding worth recording.
- **Answer reversibility and dead-end in words at the moment of arrival**, before reading further.
  Both questions are about the state you are in on landing, and both get easier to answer "yes" the
  longer you stay on the page.
- **Resist repairing anything.** A missing fragment, an off-by-one heading, a link that resolves
  nowhere — each is a one-line fix and each fix destroys the observation. Route it by slug and move
  on; that is what `## Explicitly not in this PR` is for.
- **Two files from the start.** Drafting one combined record and splitting it later reliably produces
  a Q-B section that leans on Q-A's success, which is the exact masking project AC-005's *path per
  question* forbids.
- **Mount before you finish.** Add the two `## Companions` bullets in the same change as the records,
  beneath `project.md`'s closing `---` and nowhere near its frontmatter. An unmounted record is the
  documentation-medium equivalent of a component that compiles and is never rendered.

## Tests and CI (merge gate)

This project takes **no `testing` brief, deliberately** — `.bklg/docs-that-teach/_decomposition.md`
records `architecture` and `ux` as the warranted pair and withholds `testing` because AC-004, AC-005
and AC-009 are *observed walks*, and `project.md` states plainly that automated checking of them is
owned by nobody, deliberately. This table is therefore grounded in the project's **Definition of
done** (items 1, 2 and 4), `_design.md`'s named gap 2, and `CLAUDE.md`'s Commands block. Where a
tier is a human read rather than a command, it says so rather than borrowing a green tick.

| Tier | Command / path | Proves |
| --- | --- | --- |
| **T1** Gate (whole bar) | `cargo xtask ci --fast` | The non-terminal project's merge bar (project DoD item 1). Here it proves a **negative**: fmt, clippy, tests, docs and the wasm32 steps are all exactly as they were, because this story changed nothing they read — **AC-006** |
| **T2** Gate (specification) | `cargo xtask spec-trace` | No `spec/SPECIFICATION.md` citation anywhere in the tree was disturbed. Run to prove this story discharged and amended nothing, not to discharge anything — **AC-006** |
| **T3** PR boundary | `git diff --name-only main...HEAD`, read against this spec's `## PR boundary` fence (`redkiln verify --grain story`) | Every changed file is under this story's folder or is `project.md`; nothing under `crates/`, `docs/`, `xtask/`, `standards/`, `spec/` or `.kb/` moved — which is what makes "recorded, not repaired" checkable rather than promised — **AC-001, AC-006, AC-008** |
| **T4** Diff assertion (no device) | `rg -n "<details>\|<summary>\|<div\|<br\|style=\|^\s*<" ` over the two records and the `project.md` diff | Zero folds, zero raw HTML, zero inline styles in anything this story authored — anti-patterns 9 and 10 — **AC-009** |
| **T5** Diff assertion (link text) | `rg -n "\[here\]\|\[this\]\|\[docs\]\|\[read more\]\|\[see this page\]\|https\?://" ` over the two records and the added `## Companions` bullets | No deny-listed link text and no bare URL into this repository's own tree — anti-pattern 3 and the accessibility floor's self-describing-link-text rule — **AC-008, AC-009** |
| **T6** Human read (recorded, not automated) | Each record read end-to-end against this spec's `## Behavior and interfaces` table and `_design.md`'s `## States` and `## Anti-patterns` | Composition fidelity and evidential honesty. **Nothing in `cargo xtask ci` verifies keyboard-only reachability or self-describing link text** — `_design.md` `## Density budget`, named gap 2, says so, and the honest gate is a recorded read rather than a fabricated linter — **AC-002, AC-003, AC-004, AC-005, AC-006, AC-007, AC-009** |
| **T7** Structural check (recorded) | Per dated entry: required sections present in the fixed order; every hop-table row full; exactly one traversal row per entry; each record's `## Companions` bullet target exists on disk | The instrument's shape survived — the order of a dated entry is the instrument, and a hop table with an empty affordance cell is a walk that was not observed — **AC-003, AC-004, AC-008, AC-009** |
| **T8** Append-only check | `git log -p -- .bklg/docs-that-teach/reach-and-adapter-path/second-question-walk-records/walk-record-q-*.md` | No commit rewrites the body of a dated entry closed in an earlier commit; re-walks appear as new entries citing the old — **AC-006** |
| **T9** Backlog integrity | `redkiln validate --kb && redkiln doctor` | `project.md`'s frontmatter is untouched by the mount, and the six expected `template-drift` advisories are still exactly six — **AC-008** |
| Deferred, by design | The non-insider friction log | **Not this story's gate and not this story's claim.** HS-P0024 `comprehension-evidence` owns the only method that supports it (BR-14); nothing here may stand in for it — **AC-007** bounds the claim instead |

**Merge gate, stated as one line:** `cargo xtask ci --fast` and `cargo xtask spec-trace` green, two
dated keyboard-only records existing — one per named question, each reaching its answering passage in
one hop or closing as a routed failed walk — both linked from `project.md`'s `## Companions`, and no
file changed outside the PR boundary.

## Risks and coupling (PR-scoped)

| Risk / coupling | Exposure in this PR | Mitigation |
| --- | --- | --- |
| **The slice witnesses itself** | `reach-walks` is implemented in one context alongside `front-door-walk-record`, and the surfaces walked were built one slice earlier by the same team | EC-009 halts rather than proceeds; **B6** exists precisely to keep the installer and the witness in different slices; AC-007 makes the relationship a written field a reviewer can check |
| **Prefix contamination** | The same person very likely walked the front door minutes earlier and now knows where the guide lives | AC-002's prefix disclosure. A disclosed prefix is a bounded observation; an undisclosed one is not evidence at all (EC-010) |
| **Blocked is mistaken for failed, or worse, for done** | If slice `front-door-reach` has not merged, or a link was legitimately declined, there is nothing to walk | EC-001 and EC-002 separate the two and require the block be recorded against the dependency slug. Simulating a destination produces a record that measures scheduling |
| **The temptation to repair** | Every failure this walk can find is a one-line fix sitting right there | AC-006 plus **T3**: the PR boundary excludes every path a repair would touch, so a repair is a gate failure rather than a judgement call. *A witness who edits the thing they are witnessing has destroyed the observation* |
| **Merge-forward** | Every path and heading this walk depends on — the origin page, both destination fragments, both front-door surfaces — is **derived and not verifiable from this worktree**; `crates/happenstance/src/lib.rs` alone differs by 162 lines between branches | EC-013: re-bind against the merged tree and say which tree was walked. The story map records this as a hard external gate on slice 3 |
| **Overclaiming** | It is tempting to report initiative DoD 9 green because two walks succeeded | AC-007's bounded-claim line and NF-007's claim budget. The honest sentence is fixed by `project.md`'s risk table and `_storymap.md`'s standing constraints, and it is narrower than the result feels |
| **A decorative record** | Nine mechanical assertions all pass on an unpunctuated paragraph containing the right strings | AC-009 plus **T6** and **T7**. This is stated in `_design.md`'s own words as named gap 2 rather than papered over with a linter that does not exist |
| **Slice-mate coupling on one file** | `front-door-walk-record` also appends a bullet to `project.md`'s `## Companions` in the same slice | Append in merge order (that story first, per `_storymap.md` step 5); the conflict is textual adjacency in body prose, never two stories claiming one bullet |
| **The third stall point looks like an omission** | A reviewer may read two-of-three questions as incompleteness | It is on record as deliberately not walked, with its reason, in `evaluator-onward-links`'s named-questions record; two is the floor project AC-005 sets, and the third is a direct probe for HS-P0024's friction log. Re-opening it here is out of boundary |
| **`project.md` frontmatter** | The mount touches a live CLI-owned item | EC-014 and **T9**: body prose only, beneath the closing `---`; a `PreToolUse` hook denies the frontmatter edit and `redkiln doctor` catches drift |

## Dependencies

**Blocks on** (hard, in-project — both edges are recorded as real `blocked_by` links on the item):

- **`front-door-pointer`** — installs the DT-10 pointer on `crates/happenstance/src/lib.rs`'s crate
  root and `crates/happenstance/README.md` (register rows P1, P2). Without it there is no way for the
  walker to reach the origin page *as a reader continuing from the front door*, which is what AC-002
  requires; being handed a URL walks Journey A's A2 alone and skips the A1 transition entirely.
- **`evaluator-onward-links`** — names Q-A and Q-B, records why the third stall point is not walked,
  and installs one in-passage link per question from the origin page to the answering passage (rows
  P4, P5). It is the surface this story observes; without it AC-001 has no questions to consume and
  AC-003 has no hop to count. Its own spec is explicit that the walks are **not** its gate and that
  AC-005 requires a different pair of hands.

**Slice-mate, not a dependency:** `front-door-walk-record` (HS-S0160) — the same slice `reach-walks`,
implemented in one context and mounted as one integrated surface, observing project AC-004. It merges
first (`_storymap.md`, Merge order, step 5), which is also the reader's own order and the reason
AC-002 requires the prefix disclosure. Its evidence stays a separate artefact and is not merged into
these records.

**Unlocks:**

- **Project `reach-and-adapter-path`'s Definition-of-done item 4**, for its AC-005 walk — the last of
  its three walks to land in the `reach-walks` slice, alongside `front-door-walk-record`'s AC-004.
  Item 4 is not fully resolved until `error-site-walk-record` lands AC-009 in slice
  `adapter-error-walk`.
- **Initiative DoD scenario 9** — this story is the entire observation half; slice `front-door-reach`
  was the installation half.
- **HS-P0024 `comprehension-evidence`** — a friction log run against a half-assembled surface measures
  the assembly, not the teaching (`project.md`, Dependencies). These records are the statement of what
  was assembled, and the bounded claim is what leaves HS-P0024's own claim available to be made.

**External gates** (outside this project, hard, and **not verifiable from this worktree**): slice
`front-door-reach` merged; HS-P0022's answering pages existing, registered and carrying the two named
headings; HS-P0020's pinned tree and its registration check landed and its hosting shape resolved; the
merge-forward rule applied to the slice.

## Anchors (progressive disclosure)

Load-bearing depth is deferred, not optional. The `## Context pack` above is sufficient to start; open
these at the moment named. Every path was confirmed present in this worktree.

**One anchor is deliberately withheld from the walker.** The rows below are for the *implementer* —
the person who plans the walk, reviews the record and mounts it. If the implementer is also the
walker, the cold-start protocol (AC-002) forbids opening the surface-revealing ones — rows 2, 3, 5
and 9 — until **after** the last hop is logged. Reading them first does not invalidate the walk; it
converts it into an undisclosed contaminated one, which is worse.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/reach-and-adapter-path/_design.md` | The **binding** signed-off design. `## States` carries the whole **Empty** vs **Refused** distinction this story turns on; `## Composition` and `## Hierarchy` fix what a correct landing looks like; `## Density budget` states in its own words that nothing in `cargo xtask ci` checks keyboard reachability or link text, which is why the gate here is a recorded read; anti-patterns 3, 5, 6, 9, 10 and 14 are the diff-checkable rules the records themselves obey | Before writing either record's structure, and again when judging whether a walk failed or was blocked | AC-004, AC-005, AC-006, AC-009 |
| `.bklg/docs-that-teach/reach-and-adapter-path/evaluator-onward-links/spec.md` | The story that named Q-A and Q-B and built both hops: the exact question wording, the origin page and both destination headings, the register rows P4/P5, and its own AC-007 refusal behaviour — which is the difference between EC-002 (a legitimate block) and EC-006 (a real dead end) | For the question wording and the routing target of any finding — **after the walk** if the implementer is also the walker (AC-002) | AC-001, AC-003, AC-006 |
| `.bklg/docs-that-teach/reach-and-adapter-path/front-door-walk-record/spec.md` | The slice-mate's record shape and its mount into the same `## Companions` section, including its own `walk-record.md` path — which is the artefact AC-002's prefix disclosure must cite by name | When writing the prefix-disclosure sentence, and before appending the `## Companions` bullets | AC-002, AC-008 |
| `.bklg/docs-that-teach/reach-and-adapter-path/_decomposition.md` | The UX brief in full: Journey A's states A0→A3 (the *question formed* transition a handed URL skips), the nine interaction-quality invariants with their falsifications — 3, 4 and 5 are literally about this story — and the accessibility floor's keyboard-only clause covering all three walks | Before the first hop, for the protocol; and at the landing, for invariants 4 and 5 | AC-002, AC-003, AC-004, AC-005 |
| `.bklg/docs-that-teach/reach-and-adapter-path/_storymap.md` | Backbone **B6** (why a walk is a separate slice from the surface it walks), the Coverage AC-005 row splitting *naming* from *walking*, Merge order step 5, and the standing constraints — including the verbatim bounded-claim sentence AC-007 must not exceed | When staffing the walk (EC-009) and when writing the bounded-claim line | AC-001, AC-007 |
| `.bklg/docs-that-teach/reach-and-adapter-path/project.md` | The mount point itself, plus AC-005's full text, DR-10, DoD item 4, and the two risk rows whose wording AC-007's bounded claim reproduces | When mounting the records, and when checking the claim has not widened | AC-007, AC-008 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | Persona 3 and the Cross-persona tension that the whole journey happens in one session with no second attempt — the reason a silent wrong landing is a lost reader rather than a deferred one, and the reason the walk is time-boxed to one session | When designing the protocol, and when judging whether an arrival was actually an arrival | AC-002, AC-003 |
| `.bklg/docs-that-teach/_discovery/research/03-teaching-the-aggregate-to-boundary-shift-how-dcb-events-disi.md` | The three independently observed stall points Q-A and Q-B are drawn from, and why the third is a friction-log probe rather than this story's walk | Only if a reviewer challenges the question set — never as a source for substituting one (EC-003) | AC-001 |
| `.bklg/docs-that-teach/application-author-path/_design.md` | Owns the origin and destination pages, their routes and their headings — including DT-1's ruling that the prior-model anchor's reader-facing home is the `conceptual-bridge` section "Where your streams went", which is Q-B's landing, and the assumption statements UX invariant 4 is checked against | When routing a finding to HS-P0022, and when checking whether a landing passage states its assumptions — **after the walk** if the implementer is also the walker | AC-005, AC-006 |
| `.bklg/docs-that-teach/reach-and-adapter-path/_grounding.md` | The verified sibling-project dependency state at planning time — what was actually true in this worktree, so the implementer knows what to re-verify rather than trust before deciding whether this story is blocked | At the very start, before staffing a walk that may have nothing to walk | AC-006 |
| `.bklg/docs-that-teach/initiative.md` | DoD scenario 9 in its own words (the scenario this story is the observation half of), BR-16, and BR-14 — the requirement that reserves the non-insider claim to HS-P0024 and bounds every sentence in these records | When writing the bounded-claim line, and when reporting the result upward | AC-007 |
| `.bklg/docs-that-teach/reach-and-adapter-path/design/mock.html` | The sign-off frames for the walked surface, with their own honesty note that they approve composition and density **only** because HS-P0020's tree had no renderer at design time | **Never before a walk** — it reveals where the link is. Afterwards, when reviewing a record for AC-009's composition rules | AC-009 |
| `.bklg/docs-that-teach/reach-and-adapter-path/second-question-walk-records/discover.md` | This story's own signal ledger and, in one line, the wrong implementation it rejects: *"Two questions chosen because their answers are known to exist, rather than because a reader would actually ask them"* | Before accepting any argument for adjusting the question set | AC-001 |
| `.redkiln/templates/_ledger.md` | The ledger contract: planning writes every row `satisfied: false`, the implementer may only flip a row with cited evidence and may never re-word a criterion | Before flipping the first ledger row | AC-001 |

## Clarifications resolved during spec

1. **The AC set is exactly the nine the front half enumerated** — AC-001 through AC-009. None added,
   none dropped, none re-scoped; `_ledger.md` carries the same nine ids and the same wording.
2. **Both walks are one story and two files, and that is not a contradiction.** The story map's *one
   dated record per question* is the binding phrase. One PR, one mount, two artefacts — because a
   failure on Q-B must be visible independently of a success on Q-A, and project AC-005 asks for a
   path *per question*.
3. **"Two hops" is a failure, not a partial success.** Resolved in AC-003 and EC-004 rather than left
   to judgement, because the pressure at the moment of walking is entirely towards recording a
   near-miss as an arrival. `_design.md`'s `## What it costs a caller` states *"One hop, never two"*
   and this story is where that is measured.
4. **Blocked, failed and completed are three outcomes, not two.** `_design.md`'s **Empty** and
   **Refused** states are what separate them (EC-001, EC-002, EC-006). A block is recorded against
   the dependency and the story stops; a failure is recorded against the surface and the story
   *completes*, because a recorded failed walk is a successful execution of this instrument.
5. **The keyboard-only requirement covers this walk, despite UX-AC-09's narrower parenthetical.**
   UX-AC-09 lists only the AC-004 and AC-009 records; the accessibility floor and UX-AC-08 both cover
   AC-005 explicitly. AC-004 honours the broader statement, and the narrowness is recorded here as a
   drafting artefact of the brief rather than a licence.
6. **Prefix contamination is disclosed rather than engineered away.** Recruiting a second person to
   preserve a pure cold start was considered and is out of this project's reach — everyone available
   is an insider, and HS-P0024 owns the only method that reaches further (BR-14). AC-002 makes the
   contamination a stated bound; EC-010 makes concealing it the failure.
7. **This story files no register row and renders no surface.** P4 and P5 are `evaluator-onward-links`'s
   and the register is build-time data by design (anti-pattern 14), so project AC-003 is not claimed
   here. Consulting the register *during* a walk would tell the walker where the links are, which is
   exactly the knowledge the cold-start protocol excludes; reading it afterwards to route a finding is
   fine.
8. **Every page path, heading and fragment in this spec is derived, not confirmed.**
   `docs/first-encounter.md`, `docs/carry-your-invariant.md`, `## Tag, query, fold, guard` and
   `## Where your streams went` come from crossing HS-P0022's routes with HS-P0020's `docs/<page>.md`
   shape, and none is verifiable from this worktree. EC-013 makes re-binding against the merged tree
   an obligation rather than a courtesy — and a walker who cannot find the origin page has produced a
   finding, not a mistake.
9. **The verifying instruments are honest about what is a command and what is a read.** Six of the
   nine ACs are carried in part by **T6**, a recorded human read. `_design.md` already measured that
   gap in its own words ("Two named gaps, not assumed away", item 2), and inventing a linter here to
   produce a green tick would be the decorative-rule failure `CLAUDE.md` names for conformance rules,
   transposed into this medium.
