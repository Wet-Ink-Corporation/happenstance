---
item: HS-S0160
stage: spec
created: 2026-08-17T13:16:15.962Z
updated: 2026-08-17T13:16:15.962Z
template_sig: 87bbf1d0
rendered_sig: a26397ba
---

# Spec — The observed front-door walk

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` — BR-08, BR-14; **DoD scenario 7** |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` — the DAG, the warranted-brief table, the merge-forward rule |
| Project (gold source) | `.bklg/docs-that-teach/reach-and-adapter-path/project.md` — **AC-004**, DR-10, DoD items 1 and 4 |
| This spec | `.bklg/docs-that-teach/reach-and-adapter-path/front-door-walk-record/spec.md` |
| Key brief — architecture + UX (one file) | `.bklg/docs-that-teach/reach-and-adapter-path/_decomposition.md` — interaction invariants 1–5, the accessibility floor, UX-AC-01, UX-AC-02, UX-AC-09 |
| Key brief — grounding (dependency state) | `.bklg/docs-that-teach/reach-and-adapter-path/_grounding.md` — "Sibling-project dependency state", verified rather than assumed |
| Signed-off design (**binding**) | `.bklg/docs-that-teach/reach-and-adapter-path/_design.md` — approved 2026-08-17; surfaces `crate-root-front-door`, `readme-front-door` |
| Story map (slice, one-line, dependencies) | `.bklg/docs-that-teach/reach-and-adapter-path/_storymap.md` — backbone **B2** and **B6**, Merge order step 5 |
| Dependency's spec | `.bklg/docs-that-teach/reach-and-adapter-path/front-door-pointer/spec.md` — the surface this story walks |
| This story's discover artifact | `.bklg/docs-that-teach/reach-and-adapter-path/front-door-walk-record/discover.md` |
| Roadmap pointer | none. `RUNBOOK.md` is the library's phase plan and carries no documentation-reach phase; the plan of record is the initiative above. |

**No testing brief exists for this project, deliberately.** `.bklg/docs-that-teach/_decomposition.md`
("Warranted briefs") withholds it because AC-004, AC-005 and AC-009 are *observed walks*, not
automated checks, and `project.md`'s out-of-scope table names the owner of automated checking for
the three walks as "Nobody, deliberately". This story is one of the three instruments that omission
is defensible because of.

## One-line PR slice

Someone who did not install the pointer starts from nothing but what `cargo add happenstance` shows
and reaches the narrative material keyboard-only; dated record of what they opened, in order.

## Executive summary

This PR lands **one artefact and one link**: a dated, first-person walk record at
`.bklg/docs-that-teach/reach-and-adapter-path/front-door-walk-record/walk-record.md`, and its
registration in the project item so project Definition-of-done item 4 resolves against a file rather
than a memory.

The delta over what already exists: slice `front-door-reach` will have *installed* the DT-10 pointer
on both front-door surfaces — one authoritative 22-word sentence, mirrored byte-identically onto
`crates/happenstance/src/lib.rs`'s crate root and `crates/happenstance/README.md`
(`front-door-pointer`, HS-S0158). Nothing yet **observes** that a person who did not install it can
start from what `cargo add happenstance` shows and arrive at the narrative material, and the story
that installed it cannot supply that observation: `_storymap.md`'s backbone row **B6** puts every
walk in its own slice precisely so the author is never their own witness. This story is that
separate pair of hands.

It also carries a second job the sibling walk does not, and it is the reason this record is more
than a formality. `_design.md` records that **P2 has no live guard**: the README-as-doctest mount at
`crates/happenstance/src/lib.rs:10` compiles the README's ```` ```rust ```` fences, not its prose, and
the mirror assertion that would check the two sentences against each other *does not exist and is
not this project's to write* (`_design.md`, `## Density budget`, "Two named gaps"). Until HS-P0020
lands that step, **this walk is the only thing in the repository that reads both front doors and
notices if they disagree.** That is why the walk enters through both surfaces rather than picking
one.

It changes no code, no public item and no rendered surface. It produces evidence, and the evidence
is falsifiable in the one direction that matters: if the walk fails, the record says so and the
slice it walked is not done.

## Context pack

Everything below is a decision already taken elsewhere and binding here. Read this section and you
can start; the anchors are for depth, not for orientation.

**1. The walk is the deliverable, and an asserted walk is a failed story.** `discover.md` names the
wrong implementation in one line: *"A walk performed by an insider, who routes around the rough
spots without noticing them."* Project **DR-10** states the reason — *"A walk that happened and was
not written down is indistinguishable from one that did not"*. The unit of work is therefore a
**transcript of an event that occurred**, written during or immediately after it, not a summary
composed afterwards by someone who knows where the pointer was installed.

**2. The cold start is fixed by the criterion, and it is narrower than "read the docs".** Initiative
**DoD scenario 7** is verbatim: *"@smoke — the reader reaches the teaching from the front door.
Starting only from what a developer sees after installing the crate, a person with no prior
knowledge of this repository's layout reaches the narrative material, observed rather than
asserted."* Project **AC-004** says the same thing with the walker constraint attached: *"Starting
only from what a developer sees after `cargo add happenstance`, a person who did not install the
pointer reaches the narrative material. Dated record of what they opened, in order."* "Starting
only from" is a constraint on the walker, not a flourish: **no repository checkout, no `docs/`
directory, no `spec/SPECIFICATION.md` opened directly, no backlog, no sibling story, no asking the
person who installed the pointer.** What `cargo add happenstance` actually puts in front of a
developer is two things and no more — the crates.io page (which renders
`crates/happenstance/README.md`) and the docs.rs page (which renders the crate root at
`crates/happenstance/src/lib.rs:11-71`). Those two are the permitted entry points, and they are the
whole starting state.

**3. Both front doors are entered, because the mirror has no other guard.** `_design.md` resolved
DT-10 to option (c) — *"Exactly one authoritative text. One sentence, authored once, mirrored
byte-identically onto both front-door surfaces. Two renderers, two readers, one authority — not two
pointers"* — and its `## Anti-patterns` item 2 forbids two different sentences serving as the
pointer. The check that would catch a drift is the **mirror assertion**, and `_design.md`'s "Two
named gaps" (1) records that it does not exist, that the doctest mount guards code and not prose,
and that the step is *asked of* HS-P0020 rather than forked into `xtask/src/`. Consequence for this
story, and it is a scope decision rather than an enthusiasm: the record covers **both** entry
points — the README as crates.io/GitHub render it, and the crate root as rustdoc renders it — and
states whether the sentence a reader meets is the same one. A walk that enters through one surface
observes half of what was installed and leaves the unguarded half unobserved.

**4. Observation A is "found without scrolling", and it comes before any hop.** The front-door
analogue of the sibling walk's in-place unblock is **UX-AC-01** — *"The front-door pointer is
visible before the fold and costs the reader nothing else"* — plus interaction invariant 2,
non-occlusion. `_design.md`'s `## Density budget` fixes the numbers the observation is made against:
first screen ≈ **27 rendered lines** at 1024x768 (≈ 32 at 1440x900), the pointer within the **first
5 rendered lines** of the crate root, and on the README the pointer sits *above* the status
blockquote for a stated reading reason — *"a reader who bounces off 'early, and a facade' must
already have been told the guide exists"* (`_design.md`, `## Composition`). So the record answers,
in order and before the first hop: at 1024x768, without scrolling, was the offer of guide-level
material on screen, and did it precede the disclaimer? Collapsing that into "I found the link"
destroys the one thing the whole placement decision was made for.

**5. Persona 3, one reading session, no second attempt.** The walker stands in for the evaluator:
*"reads for twenty minutes and decides whether to depend on this"*, whose journey *"happens inside
one reading session, with no second attempt if the first one fails silently"*
(`_discovery/distillation/personas-and-journeys.md`, Persona 3 and Cross-persona tensions). The
measured today-state this walk is compared against is the seed's own verdict — the README answers
the first question well and *"there is nowhere for that question to go"*. The walk is therefore
timed and bounded rather than exhaustive: a path found after fifteen minutes of determined searching
is a finding, not a success.

**6. The walker is a non-author, and the claim stops exactly there.** Project AC-004 requires a
person *"who did not install the pointer"*. `project.md`'s risk table and `_storymap.md`'s standing
constraints bound the claim in the same words: *"these walkers are non-authors, not non-insiders…
Evidence that the path exists is not evidence that a stranger finds it — that claim is HS-P0024's
alone (BR-14)."* The record makes **two** statements and the second is the one that is easy to omit:
who walked it and their relationship to the work, **and** an explicit line saying this is not
non-insider evidence. Overclaiming here spends a sibling project's only instrument in advance and
cannot be un-spent. Note the sharper local risk: initiative DoD 7 says *"a person with no prior
knowledge of this repository's layout"*, which is a stronger condition than AC-004's non-authorship
— where the available walker fails it, the record says so plainly rather than quietly reading DoD 7
down to AC-004.

**7. The medium substitution is named, not hidden.** `cargo add happenstance` resolves to the
*published* crate, and this branch is not published. The walk is therefore performed against the
pre-publication renders — a local `cargo doc -p happenstance --no-deps` standing in for docs.rs, and
the rendered Markdown of `crates/happenstance/README.md` standing in for crates.io — because those
are the same generator's output for the same source. That substitution is a real limit on the claim
and the record states it in the protocol stanza. Nothing else about the checkout may leak into the
walk: having the tree on disk to build rustdoc is not permission to read the tree.

**8. Keyboard-only, and the record says which keys.** The accessibility floor is not advisory:
*"Each of the three observed walks (AC-004, AC-005, AC-009) is performed keyboard-only, and the
dated record says so"* (`_decomposition.md`, Accessibility floor), and **UX-AC-09** adds the
walker's name and relationship. The affordances available are the medium's own and are enumerated
there — plain links, rustdoc search (`S` or `/`), rustdoc collapse (`+` / `-`) — plus Tab/Enter and
in-page find on the rendered README. Naming the affordance used at each hop is what distinguishes a
keyboard walk from a mouse walk retold in the past tense.

**9. Land on the answer, and be able to walk back.** Interaction invariant 3 requires a pointer
whose answer is not on the destination's first screen to target a *fragment*, not a bare page, and
**UX-AC-04** records that per row in the AC-003 inventory. Invariant 4 requires every hop to be
walk-backable — *"the destination of each hop states what it assumes the reader has already read"* —
which is the axum ordering failure stated as a per-hop requirement. So arrival is proved against the
destination: the record names the **passage** landed on and that page's own stated answered-need,
not "the guide".

**10. A dead end is a finding, and it is recorded as one.** Invariant 5 — *"No dead ends… recorded
as a failed walk, not quietly re-walked"* — and `_design.md`'s **Refused** state apply verbatim. If
the sentence is below the fold, if the link resolves nowhere, if the two surfaces carry different
text, or if the destination is a page rather than a passage, the failed walk is the artefact this PR
lands. A second attempt after a fix is a **second dated entry**, never an edit of the first. This is
the one behaviour that makes the record capable of failing, and a record that cannot fail is
decorative.

**11. The witness does not repair what they find.** Findings are routed by **story slug** —
`front-door-pointer` owns the pointer and its placement, `evaluator-onward-links` owns the onward
hop, `pointer-policy-and-inventory` owns the register row and its guard — and nothing is fixed in
this PR. The PR boundary below carries no `crates/**` glob, so the temptation is at least visible as
a boundary violation rather than as an invisible edit to the thing being observed.

**12. This story renders no surface and adds no navigation.** `_design.md`'s `## Surfaces` declares
five; this story renders none. It *observes* `crate-root-front-door` and `readme-front-door`, which
its dependency builds. `_storymap.md`'s standing constraints still apply to the artefact it writes:
no widget, no raw HTML, no inline styles, no folded or tabbed content, no skipped heading levels,
self-describing link text, no bare URL into this repository's own tree. The record is prose and one
table.

**13. Nothing in the gate checks any of this, and that is stated rather than papered over.**
`_design.md`'s `## Density budget` names the gap in its own words: *"Nothing in `cargo xtask ci`
verifies keyboard-only reachability or self-describing link text; AC-004, AC-005 and AC-009 are
dated observed walks by design, and the absence of a linter is stated rather than papered over."*
Verification for this story is therefore a **human reading the record against its own stated
protocol**, plus `cargo xtask ci --fast` proving the tree is unchanged in the ways this story
promises it is unchanged.

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `capability` — a user-observable slice; the user is Persona 3, the evaluator, and what they observe is whether the front door built by slice `front-door-reach` actually carries them to the teaching. |
| **Slice / milestone** | `reach-walks`. **Slice-mate**: `second-question-walk-records` (HS-S0161), implemented in the same context and mounted as one integrated surface — the two walk records are one body of evidence, and `_storymap.md`'s Merge order step 5 runs this one first. Both are separated from the surfaces they walk by backbone row **B6**, so the slice contains no story that installs anything. |
| **Mount point** | **`.bklg/docs-that-teach/reach-and-adapter-path/project.md`** — the project item's body, where the record is linked under `## Companions` and where Definition-of-done item 4 (*"The three walks (AC-004, AC-005, AC-009) each have a dated record in this project's artefacts, each naming its walker and their relationship to the work"*) stops being a promise and starts resolving. **Body prose only**; the `redkiln` CLI is the single writer of the item's frontmatter and a `PreToolUse` hook denies the edit (`CLAUDE.md`, "Where the work lives"). |
| **Wires into** | The two front-door surfaces its dependency lands, consumed **as a reader at their rendered paths, never as source**: `target/doc/happenstance/index.html` at `#main-content > .docblock` (the selector `_design.md`'s `## Surfaces` pins for `crate-root-front-door`, verified against this worktree's own build) and the rendered Markdown of `crates/happenstance/README.md` (`readme-front-door`). Also: the authoritative sentence pinned in `_design.md`'s `## Signatures` P1/P2 block and its mirror substring *"Guide-level documentation"*, which is what the record compares the two surfaces against; the href ladder rung and register row `P1`/`P2` filed by `front-door-pointer`; and the destination page in HS-P0020's pinned narrative tree, whose stated answered-need is what proves arrival. |
| **Renders surfaces** | **none.** This story renders no surface id from `_design.md`'s `## Surfaces`. It *observes* `crate-root-front-door` and `readme-front-door` in the declared states `first-screen-1024x768`, `first-screen-1440x900`, `first-screen-crates-io` and `first-screen-github`. Naming it as a renderer of either would double-count work `front-door-pointer` owns. |
| **Public items** | **none.** `_design.md`'s `## Items` block declares `path: ""` for the whole project; this story adds no code, no attribute and no manifest entry at all. |
| **Conformance rule(s)** | **none, and it is not adapter-observable.** No port, bound, future or value type changes, so nothing in `crates/happenstance-testkit/src/suite.rs` could observe this story. Per `CLAUDE.md`, a rule here would be decorative — name a plausible wrong implementation it rejects and there is none, because the subject is prose a reader meets rather than behaviour an adapter exhibits. |
| **Clause(s)** | **none discharged, none amended.** `spec/SPECIFICATION.md` is not edited and no `[FROZEN]` clause is touched. The clause-pin obligation (project AC-010, DoD item 2) belongs to `store-error-site-rewrite`, the only story editing a `happenstance-core` doc comment. `cargo xtask spec-trace` runs here to prove non-disturbance, not to discharge anything. |
| **Advances DoD scenario** | **Initiative DoD scenario 7** — *"@smoke — the reader reaches the teaching from the front door. Starting only from what a developer sees after installing the crate, a person with no prior knowledge of this repository's layout reaches the narrative material, observed rather than asserted."* This story is the *entire* observation half of that scenario; the installation half is `front-door-pointer`. It also closes project **DoD item 4** for one of its three walks, and supplies the only live check on the mirror gap `_design.md` records against P2. |

**Delivered mounted, not as an isolated component.** A walk record sitting unlinked in a story
folder is this medium's equivalent of a component that compiles and is never rendered: nothing reads
it, nothing depends on it, and the project's DoD item 4 goes on failing while a green file sits on
disk. The link from `project.md`'s `## Companions` is the mount, and it is part of this PR.

## PR boundary

`redkiln verify --grain story` reads the first fenced block under this heading and fails on any file
changed outside it.

```
.bklg/docs-that-teach/reach-and-adapter-path/front-door-walk-record/**
.bklg/docs-that-teach/reach-and-adapter-path/project.md
```

**In this PR**

- `walk-record.md` in this story's folder: the dated first-person record — the cold-start protocol
  and the medium substitution, Observation A (found without scrolling, on each of the two entry
  points), the mirror comparison, the hop table, the arrival check, the keyboard-only statement with
  keys named, the walker's name and relationship, findings routed by slug, and the bounded-claim
  line.
- Any *failed* walk, as its own dated entry in the same file, if a front door did not carry the
  walker.
- The link from `project.md`'s `## Companions` — **body prose only**, never the YAML frontmatter,
  which the CLI owns and a `PreToolUse` hook denies anyway.
- This story's own `_ledger.md` and stage artifacts.

**Explicitly not in this PR**

- **Any file under `crates/`.** This story reads two rendered surfaces; it edits neither their
  source nor anything else in the workspace. The absence of a `crates/**` glob above is what makes
  that checkable rather than promised. Building rustdoc locally produces `target/`, which is
  ignored and is not a change.
- **Any fix to what the walk finds.** A pointer below the fold, a drifted mirror, a link that
  resolves nowhere, a destination that is a page rather than a passage — each is recorded and routed
  to the owning story (`front-door-pointer` for the pointer and its placement,
  `evaluator-onward-links` for the onward hop, `pointer-policy-and-inventory` for the register row
  and its guard). A witness who edits the thing they are witnessing has destroyed the observation.
- **The register.** This story files no row and changes none. It may *read* one to check that P1 and
  P2 exist with non-empty guards, and a discrepancy is a finding routed to the owning story.
- **The other two walks.** AC-009 is `error-site-walk-record`'s, in a different slice with no
  HS-P0022 edge; AC-005 is the slice-mate `second-question-walk-records`'. This story observes the
  front door only, and must not absorb the second-question hops — that would put one record's
  failure in front of two acceptance criteria.
- **The non-insider claim.** HS-P0024 `comprehension-evidence` owns the friction log and the only
  method that supports it (BR-14).
- **Any change to `spec/SPECIFICATION.md`, `standards/`, `xtask/`, `docs/` or `.kb/`.**

**Merge DoD one-liner.** Merged when `walk-record.md` carries a dated, keyboard-only, first-person
record whose walker is named and did not install the pointer, which states its cold-start protocol
and its medium substitution before its first observation, reports the pointer found without
scrolling on **both** front doors with the two sentences compared, ends at a named passage in the
narrative material with no dead end, and is linked from `project.md` so DoD item 4 resolves — with
`cargo xtask ci --fast` green and no file under `crates/` touched.

## Behavior and interfaces

The "interface" in this medium is the record's own shape: the fields a reader needs in order to
re-run the walk and disagree with it. The table below is the contract.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The record exists at one known path, dated** | `walk-record.md` in this story's folder, carrying an ISO date for the walk itself, not the commit date. One file; a failed walk and its later re-walk are two dated entries inside it, newest last. | `project.md` DoD item 4; `.bklg/docs-that-teach/reach-and-adapter-path/front-door-walk-record/` |
| **The cold start is stated as a protocol, before the walk** | What the walker was permitted at t=0 — the crates.io-equivalent README render and the docs.rs-equivalent crate-root render, and nothing else — and what was forbidden: the repository tree, `docs/`, `spec/SPECIFICATION.md` opened directly, the backlog, this spec, the sibling stories, and the person who installed the pointer. Any contamination is disclosed, not omitted. | `initiative.md` DoD scenario 7; `project.md` AC-004; `discover.md`, "The wrong implementation" |
| **The medium substitution is declared** | `cargo add happenstance` resolves to the published crate and this branch is not published. The record names the stand-ins used — `cargo doc -p happenstance --no-deps` for docs.rs, the rendered Markdown of `crates/happenstance/README.md` for crates.io — and states that having the tree on disk to build rustdoc was not permission to read it. | `crates/happenstance/src/lib.rs:11-71`; `crates/happenstance/README.md`; `_design.md` `## Surfaces` (`route`, `selector`) |
| **Observation A: found without scrolling, before any hop** | At **1024x768**, on each entry point, the record answers first: was a sentence offering guide-level material on screen without scrolling, and on the README did it precede the status blockquote (`crates/happenstance/README.md:6-11`)? Quote the sentence as it rendered. Only then does the walk continue. | `_decomposition.md` UX-AC-01 and invariant 2; `_design.md` `## Density budget` (first screen ≈ 27 rendered lines; pointer within the first 5) and `## Composition` |
| **Both front doors are entered, and the mirror is compared** | The walker enters through the rendered README *and* the rendered crate root, and records whether the sentence is the same on both — this is the only live check on the mirror while the assertion `_design.md` asks of HS-P0020 does not exist. Two different sentences is anti-pattern 2 and a finding against `front-door-pointer`. | `_design.md` `## Pattern decision` rule 1, `## Anti-patterns` item 2, `## Density budget` "Two named gaps" (1); `crates/happenstance/src/lib.rs:10` (the doctest mount that guards fences, not prose) |
| **Every hop is a row: what was on screen, what was pressed, where it landed** | A hop table — ordinal, entry point, surface, the affordance used (plain link and `Tab`/`Enter`, rustdoc search `S` or `/`, in-page find, a fragment), and the destination **passage**, not just the page. | `_decomposition.md` Accessibility floor and invariant 3; `_design.md` `## Density budget` (link text ≥ 3 words, a noun phrase) |
| **Keyboard-only, and it says which keys** | One explicit statement that no pointing device was used, plus the keys named per hop. A record that says "keyboard-only" and names no key has asserted the floor rather than observed it. | `_decomposition.md` Accessibility floor ("performed keyboard-only, and the dated record says so"); UX-AC-09 |
| **Arrival is verified against the destination, not assumed** | On landing, the record names the passage reached, that page's own stated answered-need under HS-P0021's discipline, and what the page assumes was read before it — which is invariant 4's walk-backability made observable. "I reached the guide" is not an arrival. | `_decomposition.md` invariants 3 and 4; `project.md` AC-012; `_design.md` `## Composition`, `adapter-reasoning-account` region 1 (the answered-need form) |
| **The reading budget is recorded** | Elapsed time from t=0 to arrival, and the number of surfaces opened. Persona 3 decides inside one bounded session; a path found after prolonged searching is a finding about reach, not a success. | `_discovery/distillation/personas-and-journeys.md`, Persona 3 (*"reads for twenty minutes"*) and Cross-persona tensions (*"no second attempt if the first one fails silently"*) |
| **The walker is named, with their relationship to the work** | Name, and one sentence: that they did not install the pointer (`front-door-pointer`) and did not author the destination material, plus what they knew of the surface before starting. Where the stronger DoD-7 condition — no prior knowledge of this repository's layout — is not met, the record says so plainly. | `project.md` AC-004 and DR-10; `_decomposition.md` UX-AC-09; `initiative.md` DoD scenario 7 |
| **The claim is bounded in the record's own words** | An explicit line: this is evidence that the path exists for a non-author, not that a stranger finds it; the walker is an insider. No sentence in the record may imply otherwise. | `project.md` risk table, *"AC-005 and AC-004 walkers are non-authors, not non-insiders"*; `_storymap.md` standing constraints; `initiative.md` BR-14 |
| **A dead end is recorded as a failed walk** | If the pointer is below the fold, missing from one surface, drifted, unresolving, or lands on a page rather than a passage, the entry is closed as a failure with the finding routed by story slug and nothing repaired here. A later attempt is a new dated entry citing the failed one; closed entries are never edited. | `_decomposition.md` invariant 5; `_design.md` `## The states the API must express`, "Refused" |
| **Findings are routed, never fixed** | By slug: `front-door-pointer` (placement, wording, mirror, ladder rung), `evaluator-onward-links` (the onward hop from the answering page), `pointer-policy-and-inventory` (a register row with an empty or absent guard). No person is named in a finding. | `_storymap.md` Slices table; `_design.md` `## Anti-patterns` item 18 |
| **The record is mounted** | `project.md`'s `## Companions` links it; the link text names the record and the walk it carries — no "here", no bare URL, at least three words. Nothing else in `project.md` changes, and no frontmatter line is touched. | `project.md` `## Companions`; `_decomposition.md` Accessibility floor (self-describing link text); `CLAUDE.md`, "Where the work lives" |
| **Composition of the artefact itself** | Prose plus **one** hop table. No raw HTML, no inline `style=`, no `<details>` or tab strip, no "See also" / "Next steps" / "Further reading" block, no fewer-than-three-row table used as navigation, heading ladder unskipped. | `_design.md` `## Anti-patterns` items 3, 5, 6, 9, 10; `_storymap.md` standing constraints |
| **Nothing else in the tree moved** | `cargo xtask ci --fast` green (project DoD item 1) and `cargo xtask spec-trace` clean, run to prove this story disturbed neither — not to prove anything about the record. | `CLAUDE.md`, Commands; `project.md` DoD items 1 and 2 |

**Acceptance-criteria ids this story enumerates** (the table is authored in the second pass, against
exactly this set): `AC-001` cold-start protocol and declared medium substitution · `AC-002` walker
identity, relationship and the bounded claim · `AC-003` Observation A — the offer found without
scrolling, before any hop · `AC-004` both front doors entered and the mirror compared · `AC-005` the
hop table reaching a named passage in the narrative material, with the reading budget recorded ·
`AC-006` keyboard-only with the affordance and keys named per hop · `AC-007` the dated record
mounted from `project.md` so DoD item 4 resolves, with nothing else in that file changed · `AC-008` a
dead end recorded as a failed walk with findings routed by slug and nothing repaired · `AC-009` the
record's own composition — prose plus one table, none of the named anti-patterns.

## Data and migrations

**N/A — no schema, no store, no persisted state, no migration.** This story adds no code, no type,
no event, no projection and no manifest entry; `_design.md`'s `## Items` block already declares that
the whole project changes no public item, and this story is one of the two members of it that touch
no `crates/` file at all.

The nearest thing to "data" here is the record's own shape, and it is deliberately **not** a schema:
no frontmatter beyond what the CLI owns on the items, no YAML block, no id namespace. It is prose
plus one hop table, so that a reviewer who cannot run `cargo` can still read it and disagree.

Two adjacent data structures exist and this story's relationship to each is read-only, stated so
they are not mistaken for deliverables:

1. **The pointer register** (`_design.md`, `## Transience policy`, final row) is build-time data
   owned by `pointer-policy-and-inventory`, with rows **P1** and **P2** filed by `front-door-pointer`.
   This story may read it to confirm both rows exist with non-empty guards; it adds no row, changes
   no row, and never renders it as a page (anti-pattern 14). A missing or unguarded row is a finding
   routed by slug.
2. **The destination's href**, bound late off the href ladder by `front-door-pointer`. If it is
   rebound later (typically rung 2 → rung 1 once HS-P0020 resolves hosting), **this walk is not
   re-run automatically and its record is not edited**. The record is a transcript of one event at
   one tree state, which is why it names the commit sha it was walked against; a rebinding that
   invalidates the path earns a new dated entry, not a rewritten old one.

## Acceptance criteria

Nine criteria. Each is framed from the intent of the person the walk stands in for — **Persona 3,
the evaluator**, who *"reads for twenty minutes and decides whether to depend on this"* and whose
*"entire journey happens inside one reading session, with no second attempt if the first one fails
silently"* (`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:225-229` and
Cross-persona tensions, `:304-308`) — crossing the whole stack: source doc comment → rustdoc or
markdown render → the screen a person actually meets → the dated artefact that says what happened
there. The second reader is the maintainer who has to believe the record a year from now without
having been in the room.

Project **AC-004** is carried by all nine: AC-001, AC-002 and AC-009 make the record *credible*;
AC-003, AC-004, AC-005 and AC-006 make it an *observation*; AC-007 makes it *resolve* project DoD
item 4; AC-008 makes it *capable of failing*. Nothing here restates a project AC, and nothing here
absorbs AC-005 or AC-009 — those are the slice-mate's and `error-site-walk-record`'s.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | GIVEN a maintainer reading this record a year later who must decide whether to believe it, WHEN they read `walk-record.md` from the top, THEN **before any observation** they meet a protocol stanza that states the walk's ISO date, the commit sha of the tree walked, exactly what the walker was permitted at t=0 (the crates.io-equivalent README render and the docs.rs-equivalent crate-root render, and nothing else), what was forbidden by name (the repository tree, `docs/`, `spec/SPECIFICATION.md` opened directly, the backlog, this spec, the sibling stories, and the person who installed the pointer), and the **medium substitution** — that `cargo add happenstance` resolves to a published crate this branch is not, so `cargo doc -p happenstance --no-deps` stood in for docs.rs and the rendered Markdown of `crates/happenstance/README.md` stood in for crates.io — together with the statement that having the tree on disk to build rustdoc was **not** permission to read it; and any contamination that occurred is disclosed here rather than omitted. | Reviewed read of `.bklg/docs-that-teach/reach-and-adapter-path/front-door-walk-record/walk-record.md`: the protocol stanza precedes the first observation heading in document order. Static half: `rg -n "^## " .bklg/docs-that-teach/reach-and-adapter-path/front-door-walk-record/walk-record.md` shows the protocol heading before any observation heading, and `rg -n "[0-9]{4}-[0-9]{2}-[0-9]{2}" ` returns the walk date; `git rev-parse HEAD` at walk time matches the sha named in the stanza. |
| AC-002 | GIVEN the sibling project HS-P0024, whose only instrument is the non-insider friction log, WHEN it later reads this record, THEN the record has **not spent its claim in advance**: it names the walker and states in one sentence that they did not install the pointer (`front-door-pointer`, HS-S0158) and did not author the destination material, says what they knew of the surface before starting, and carries an explicit bounded-claim line — *this is evidence that the path exists for a non-author, not that a stranger finds it* — with **no** sentence anywhere in the record implying otherwise; and where initiative DoD scenario 7's stronger condition (*"a person with no prior knowledge of this repository's layout"*) is not met by the available walker, the record says so plainly instead of quietly reading DoD 7 down to project AC-004. | Reviewed read against `project.md`'s risk table (*"AC-005 and AC-004 walkers are non-authors, not non-insiders"*), `_storymap.md`'s standing constraints and `initiative.md` BR-14. Cross-check: the named walker does not appear as author of the `front-door-pointer` commits (`git log --format='%an' -- crates/happenstance/src/lib.rs crates/happenstance/README.md` over the pointer's commit range). `rg -n "no prior knowledge\|non-insider\|stranger" walk-record.md` returns the bounded-claim line and no overclaim. |
| AC-003 | GIVEN the evaluator arriving at a front door with a twenty-minute budget and no map of this repository, WHEN they look at the **first screen at 1024x768 without scrolling and before any hop**, THEN the record answers in that order and quotes the sentence as it rendered: was an offer of guide-level material on screen at all; was it inside the first **5** rendered lines of the crate root against a first screen of ≈ **27** rendered lines (≈ **32** at 1440x900); on the README did it **precede** the status blockquote (`crates/happenstance/README.md:6-11`) rather than follow it; was it persistent chrome the reader had to expand nothing to see; and did meeting it cost the reader nothing else — no pre-existing content displaced, folded or pushed below the fold. | Reviewed read of the record's Observation A stanza against `_design.md`'s `## Density budget` (first screen ≈ 27 / ≈ 32; pointer within the first 5 rendered lines), `## Composition` (region 2 above region 3, and its stated reading reason) and `## Transience policy` (first row), plus `_decomposition.md` UX-AC-01 and invariant 2. Render half: `cargo doc -p happenstance --no-deps`, then `target/doc/happenstance/index.html` at `#main-content > .docblock` at a 1024x768 viewport, compared against the `crate-root-front-door` / `first-screen-1024x768` and `readme-front-door` / `first-screen-crates-io` frames in `.bklg/docs-that-teach/reach-and-adapter-path/design/mock.html`. |
| AC-004 | GIVEN that **nothing in the repository compares the two front doors** — the README-as-doctest mount at `crates/happenstance/src/lib.rs:10` compiles the README's ```` ```rust ```` fences and not its prose, and the mirror assertion is asked of HS-P0020 and does not exist — WHEN the walk is performed, THEN it enters through **both** entry points, quotes the pointer sentence as rendered on each, and states whether the two are the same authoritative text carrying the pinned substring `Guide-level documentation` exactly once per surface; two different sentences is `_design.md` anti-pattern 2 and is recorded as a finding routed to `front-door-pointer`, never repaired here. | Reviewed read of the mirror stanza. Independent static check run **after** the walk and reported alongside it, never used to author the observation: `rg -c "Guide-level documentation" crates/happenstance/src/lib.rs crates/happenstance/README.md` returns `1` for each, and the sentence extracted from each file (with `//! ` stripped) diffs clean. Design authority: `_design.md` `## Pattern decision` rule 1, `## Signatures` P1/P2, `## Anti-patterns` item 2, `## Density budget` "Two named gaps" (1). |
| AC-005 | GIVEN the evaluator who has read the offer and decides to follow it inside one bounded reading session, WHEN they hop, THEN the record carries **one hop table** whose every row names the ordinal, the entry point, the surface, the affordance used and the destination, and the walk terminates at a **named passage** in the narrative material — not "the guide" and not a page top — with that page's own stated answered-need recorded and with what the page assumes was read before it (invariant 4's walk-backability made observable); and the reading budget is recorded as elapsed time from t=0 to arrival and the number of surfaces opened, so a path found only after prolonged determined searching is reported as a finding about reach rather than as a success. | Reviewed read of the hop table and the arrival stanza against `_decomposition.md` invariants 3 and 4, UX-AC-04, `project.md` AC-012 and `_design.md` `## Composition`, `adapter-reasoning-account` region 1 (the answered-need form). Independent confirmation that the named passage exists at the cited path and fragment in HS-P0020's pinned tree. Budget read against `personas-and-journeys.md:229` (*"reads for twenty minutes"*). |
| AC-006 | GIVEN a keyboard-only or screen-reader reader, for whom the accessibility floor is not advisory, WHEN the walk is performed, THEN it is performed **keyboard-only** and the record says so once explicitly *and* names the affordance and the keys used at each hop from the medium's own set — plain links with `Tab`/`Enter`, rustdoc search (`S` or `/`), rustdoc collapse (`+` / `-`), in-page find on the rendered README, a fragment target — with a statement of where focus landed on each arrival; a record that says "keyboard-only" and names no key has asserted the floor rather than observed it and does not satisfy this criterion. | Reviewed read of the hop table's affordance column and the keyboard statement against `_decomposition.md`'s Accessibility floor (*"Each of the three observed walks (AC-004, AC-005, AC-009) is performed keyboard-only, and the dated record says so"*) and UX-AC-09. **No linter exists for this** — `_design.md`'s `## Density budget`, "Two named gaps" (2) states that plainly — so the verification is the reviewed read plus the named keys being drawn from the enumerated affordance set and not invented. |
| AC-007 | GIVEN the project's Definition-of-done item 4, which today is a promise with no file behind it, WHEN this PR merges, THEN `walk-record.md` is linked from `.bklg/docs-that-teach/reach-and-adapter-path/project.md`'s `## Companions` with self-describing link text of at least three words naming the record and the walk it carries — never "here", never a bare URL into this repository's tree — **body prose only**, with no YAML frontmatter line touched and nothing else in that file changed, so DoD item 4 resolves against an artefact rather than a memory for one of its three walks. | `git diff -U0 <merge-base> -- .bklg/docs-that-teach/reach-and-adapter-path/project.md` shows exactly one added link inside `## Companions`, zero deleted lines, and no line above the closing `---` of the frontmatter. `rg -n "front-door-walk-record" .bklg/docs-that-teach/reach-and-adapter-path/project.md` returns the link; `rg -n "\[here\]\|https?://" ` over the added line returns nothing. `redkiln validate --kb && redkiln doctor` stay green (the CLI's single-writer rule, `CLAUDE.md`, "Where the work lives"). |
| AC-008 | GIVEN that a record which cannot fail is decorative, WHEN the walk hits a dead end — the sentence below the fold, absent from one surface, drifted between the two, a link that resolves nowhere, or a destination that is a page rather than a passage — THEN **that failure is the artefact this PR lands**: the entry is closed as a failed walk with its stop point named, every finding is routed by **story slug** (`front-door-pointer` for placement, wording, mirror and ladder rung; `evaluator-onward-links` for the onward hop; `pointer-policy-and-inventory` for a register row with an absent or empty guard), **nothing is repaired in this PR**, no person is named in a finding, and a later attempt after a fix is a **new dated entry** citing the failed one rather than an edit to it — closed entries are never edited. | Reviewed read against `_decomposition.md` invariant 5, `_design.md` `## The states the API must express` ("Refused") and `## Anti-patterns` item 18. Structural half: the PR-boundary fence carries no `crates/**` glob, so `git diff --name-only` proving zero files changed under `crates/` is what makes "nothing repaired" checkable rather than promised; `redkiln verify --grain story` fails the PR if it is not. Where a second entry exists, `git log --follow` shows the first entry's lines unmodified since the commit that added them. |
| AC-009 | GIVEN a reviewer who cannot run `cargo` and reads only the rendered record, WHEN they open it, THEN it is **prose plus exactly one hop table** composed from markdown's own primitives — no raw HTML, no inline `style=`, no `<details>`, tab strip or accordion, no "See also" / "Next steps" / "Further reading" block, no fewer-than-three-row table used as a navigation device, an unskipped heading ladder, and self-describing link text throughout — so that nothing in the artefact this story ships trips a `_design.md` anti-pattern or a `_storymap.md` standing constraint. | Static: `rg -n "<[a-z]+\|style=\|<details" walk-record.md` returns nothing and `rg -c "^\|" walk-record.md` accounts for exactly one table (its header, separator and rows, contiguous); `rg -n "^#+ " walk-record.md` shows no skipped level; `rg -n "See also\|Next steps\|Further reading\|\[here\]\|\[this\]" ` returns nothing. Reviewed against `_design.md` `## Anti-patterns` items 3, 5, 6, 9, 10 and `_storymap.md`'s standing constraints. |

## Interaction quality

RFC §6.7/D6. Every invariant below is **already an AC row above** — this section names which row
carries it and how it is checked, and adds nothing that is not gated. The medium is prose: rendered
by rustdoc and by two Markdown renderers for the surfaces walked, and by a git host for the record
itself. "An unstyled render satisfies every assertion perfectly" has a precise analogue here — a
walk record that is textually complete and evidentially worthless: every field present, the walk
asserted rather than performed, the pointer "found" without a viewport, the mirror "checked" by
reading two source files instead of two rendered surfaces. The composition rows below are what fail
that record.

Note the unusual shape, and it is deliberate: this story **renders no surface** from `_design.md`'s
`## Surfaces` (see the Integration contract). So the composition invariants apply in two directions
at once — to the two surfaces the walk **observes**, where the design's numbers are the yardstick
the observation is made against, and to the artefact this story **writes**, which is itself
composed and can itself trip an anti-pattern.

### STATE invariants

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In-place before context-jump.** Meeting the offer costs the reader nothing else: no pre-existing answer is displaced, and a reader who never hops is left exactly as informed as before. Observed, not assumed. | **AC-003** | The Observation A stanza answers "did it cost the reader anything else" explicitly, at 1024x768, before the first hop (`_decomposition.md` UX-AC-01, invariant 2) |
| **Non-occlusion.** Nothing pre-existing is pushed below the fold, folded or reworded to make room for the pointer; on the README the offer precedes the disclaimer rather than being buried under it. | **AC-003**, and **AC-004** for the second surface | Read against `_design.md` `## Composition` (region 2 above region 3) and `## Density budget`; render half at 1024x768 against `design/mock.html`'s first-screen frames |
| **Preserved focus, scroll and position.** Each hop's landing is recorded — the passage, not the page — and where focus went; a fragment target that lands the reader at the top of a page is a finding, not an arrival. | **AC-005**, **AC-006** | The hop table's destination and affordance columns; `_decomposition.md` invariant 3 and UX-AC-04 |
| **Reversibility, twice over.** Every hop is walk-backable — the destination states what it assumes was read (invariant 4). And the record itself is append-only: a failed walk is never edited into a successful one; a re-walk is a new dated entry citing the old. | **AC-005** (hops), **AC-008** (the record) | The arrival stanza's "assumes you have read" line; `git log --follow` showing closed entries unmodified since their commit |
| **Keyboard reachability.** No pointing device at any hop, and the keys are named rather than claimed. | **AC-006** | Reviewed read; **no linter exists** and `_design.md`'s "Two named gaps" (2) says so rather than papering over it |
| **Reversibility of the claim.** The record cannot be read as more than it is: the bounded-claim line is what stops a later reader spending HS-P0024's instrument retroactively. | **AC-002** | Reviewed read against `project.md`'s risk table and `initiative.md` BR-14 |
| **Failure is reachable.** A dead end terminates the entry as a failure with findings routed by slug and nothing repaired — the state that makes every row above capable of being false. | **AC-008** | `git diff --name-only` shows zero files under `crates/`; `redkiln verify --grain story` enforces the PR-boundary fence |

### COMPOSITION invariants

Taken from `.bklg/docs-that-teach/reach-and-adapter-path/_design.md`, which is **binding** and is
not re-decided here.

| Invariant | Carried by | The real number or rule |
| --- | --- | --- |
| **Presentation exists at all** — on the observed surfaces, the pointer is composed from a repository primitive (rustdoc's pre-heading top blurb; a plain Markdown paragraph), not bare markup bolted on; in the artefact, the record is composed from Markdown's own primitives and nothing is hand-rolled | **AC-003**, **AC-004** (observed); **AC-009** (authored) | `_decomposition.md`, UX brief "Design-system primitives"; `_design.md` `## Mock` finding **F6** — every element composes from a primitive already live in this tree at a cited line |
| **Composition and placement** | **AC-003**, **AC-004** | Crate root: region **2** of `_design.md`'s composition table — after `crates/happenstance/src/lib.rs:11`, before `# Status` at `:13`. README: between `:1-4` and the status blockquote at `:6-11`, **outside** it. The record observes the rendered result of both, not the source |
| **Transience — persistent chrome, never revealed, never opened-on-demand** | **AC-003** (the offer required no expansion), **AC-009** (the record installs no fold) | `_design.md` `## Transience policy`, first row: *"A pointer that must be revealed is the gap restated."* The rustdoc-collapse row is the live risk: if `#main-content > .docblock` renders inside a closed `details.toggle`, the pointer is behind a `[+]` nobody chose — EC-007 |
| **Density budget, with its real numbers** | **AC-003**, **AC-004**, **AC-005** | First screen ≈ **27** rendered lines at 1024x768, ≈ **32** at 1440x900. Pointer within the first **5** rendered lines. Sentence **22** words as pinned (bounds **8–30**), pinned mirror substring `Guide-level documentation`, link text **≥ 3** words, named need **≥ 4** words. Reading budget: Persona 3's **twenty minutes**, one session, no second attempt |
| **Hierarchy** | **AC-003** | Position relative to the first `#` heading, and nothing else — no bold, no callout, no emoji, no second blockquote. The record states where the offer sat relative to the disclaimer, because *"a reader who bounces off 'early, and a facade' must already have been told the guide exists"* (`_design.md`, `## Composition`) |
| **Overflow / yield order, as observed** | **AC-003** | If the budget was exceeded on the day, the correct yield was the **pointer's own sentence** shortening (30 → 20 words) and nothing pre-existing moving. A walk that finds pre-existing content displaced records that as a finding against `front-door-pointer` |
| **Named anti-patterns this story can trip or observe** (`_design.md`, `## Anti-patterns`) | 1 → **AC-003**; 2 → **AC-004**; 3 → **AC-005**; 4 → **AC-003**; 5, 6, 9, 10 → **AC-009**; 16 → **AC-003**; 18 → **AC-008** | Each is checkable from a screenshot or a diff by someone who cannot read Rust — which is why they are numbers rather than taste. Items 7, 8, 12, 13, 15 and 17 belong to surfaces this story does not walk (`store-error-site-rewrite`, `adapter-reasoning-account`) and are deliberately not claimed here |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | No walker is available who did not install the pointer — the only hands on the project are the author's. | The story **blocks and escalates**; it does not proceed with the author as their own witness. That is the single failure `_storymap.md`'s backbone **B6** exists to prevent (*"Putting a walk in the same slice as the surface it walks would make the author their own witness"*), and a record produced that way is the wrong implementation `discover.md` names. |
| **EC-002** | The dependency's external gates never opened: HS-P0020 resolved no hosting shape or HS-P0022 landed no pages, so `front-door-pointer` correctly installed **nothing** (`_design.md`'s `Empty` state — the state this branch is in today). | There is nothing to walk. The story **blocks on its dependency**; it does not walk a placeholder, and it does not manufacture a record saying the walk was "not applicable". A record of a walk that could not start is not this story's deliverable — the deliverable is a walk. |
| **EC-003** | The pointer is present but **below the fold** at 1024x768, or absent from one of the two surfaces. | Recorded as a **failed walk** with the stop point named, routed to `front-door-pointer` (placement / installation). Nothing is repaired here; the PR-boundary fence carries no `crates/**` glob precisely so the temptation is visible as a boundary violation. |
| **EC-004** | The two front doors carry **different sentences**. | `_design.md` anti-pattern 2. Recorded with both sentences quoted verbatim as rendered, routed to `front-door-pointer`. This is the finding the whole two-surface entry exists to be able to make, because the mirror assertion does not exist and is not this project's to write. |
| **EC-005** | The link resolves nowhere, or lands on a **page** rather than the passage that answers the need. | Recorded as a failed walk (invariant 5) or as an arrival finding (invariant 3 / UX-AC-04) respectively, routed to `evaluator-onward-links` for the onward hop or `front-door-pointer` for the front-door href. Not repaired, not re-walked quietly. |
| **EC-006** | The walk is **contaminated** — the walker consulted the tree, the backlog, this spec, or the person who installed the pointer, deliberately or not. | Disclosed in the protocol stanza in the same breath as the protocol itself (AC-001), and the affected observation is annotated as contaminated rather than deleted. An undisclosed contamination is the one defect that cannot be detected later, so the record's honesty about it *is* its evidential value. |
| **EC-007** | rustdoc renders the crate-root module doc inside a collapsed `details.toggle`, putting the pointer behind a `[+]` the reader never chose. | Recorded as a **transience finding** against `_design.md`'s `## Transience policy` (rustdoc-collapse row) and routed to `front-door-pointer` for escalation. The walk continues and reports it; the walker does not configure rustdoc to make it go away. |
| **EC-008** | `cargo doc -p happenstance --no-deps` fails, so the docs.rs stand-in cannot be produced. | The walk **does not substitute a source read of `crates/happenstance/src/lib.rs`**. A source read is not the surface a developer meets and would make the crate-root half of AC-003 and AC-004 an assertion. Fix the build (it is a gate failure in its own right, project DoD item 1) and walk afterwards. |
| **EC-009** | The available walker fails initiative DoD scenario 7's stronger condition — they *do* have prior knowledge of this repository's layout. | Say so plainly (AC-002) and record that DoD 7's observation half is **partially** met at this strength, with the residual owed to HS-P0024. Do not read DoD 7 down to project AC-004's weaker non-authorship condition and call the scenario closed. |
| **EC-010** | After this story merges, the destination href is rebound (typically ladder rung 2 → rung 1 once HS-P0020 resolves hosting). | **The record is not edited and the walk is not re-run automatically.** It is a transcript of one event at one tree state, which is why it names the commit sha. A rebinding that invalidates the path earns a new dated entry, decided by whoever rebinds it. |

## Non-functional

| id | Requirement | Number or bound |
| --- | --- | --- |
| **NF-001** | The walk fits the persona's session, and the budget is data rather than an impression. | Elapsed t=0 → arrival recorded to the minute against Persona 3's **twenty minutes**; number of surfaces opened recorded. Over budget is a **finding about reach**, recorded as such, not a failure of the walker |
| **NF-002** | Zero footprint on the library. | No file under `crates/` changed; no public item, signature, feature or manifest entry; no conformance rule; no `spec/SPECIFICATION.md` edit; no new gate step in `xtask/src/main.rs`. The gate gains no measurable time |
| **NF-003** | Readable without a toolchain. | Prose plus **one** hop table. A reviewer who cannot run `cargo` can read the record end to end and disagree with it — which is the only reason its evidential claim survives the absence of a linter |
| **NF-004** | Durable and append-only. | Every entry carries an ISO date **and** the commit sha of the tree walked. Closed entries are immutable; a later attempt is a new entry citing the earlier one. Newest last |
| **NF-005** | WCAG AA as it lands in a text medium, in the artefact this story ships. | Self-describing link text (≥ 3 words, noun phrase); colour and position never the only carrier; nothing animates; heading ladder unskipped; no fixed width, no raw HTML (`_decomposition.md`, UX brief "Accessibility floor") |
| **NF-006** | Falsifiable in the direction that matters. | The record must be able to say "this walk failed" and have that be the merged artefact (AC-008). A record with no reachable failure state is decorative — the same bar `CLAUDE.md` sets for a conformance rule that no adapter can fail |
| **NF-007** | The claim is bounded and stays bounded. | Exactly one claim strength is asserted: the path exists for a non-author. Non-insider reach is HS-P0024's alone (BR-14), and an overclaim here cannot be un-spent |

## Implementation notes (non-prescriptive)

Sequencing, not prescription. The shape of the surfaces is `_design.md`'s and the shape of the
record is the Behavior table's; neither is re-decided here.

- **Recruit the walker before anything else.** They must not have installed the pointer and must not
  have authored the destination material. If nobody qualifies, stop — EC-001. Everything downstream
  is worthless if this step is fudged, and it is the step that is cheapest to fudge.
- **Brief the walker on the protocol, not on the answer.** They are told what they may open (the two
  rendered front doors) and what they may not (the tree, `docs/`, `spec/SPECIFICATION.md`, the
  backlog, this spec, the sibling stories, the pointer's author). They are **not** told where the
  pointer is, what it says, or that a mirror exists. Someone other than the walker prepares the
  stand-ins — `cargo doc -p happenstance --no-deps` and the rendered README — so the walker never
  needs to touch the checkout to start.
- **Fix the viewport before opening anything.** 1024x768 is the budget's viewport and Observation A
  is meaningless without it. Record 1440x900 as a second reading only if it is cheap; the design
  designs to the smaller and treats the larger as free headroom.
- **Write during, not after.** The hop table is filled in as the hops happen, with the key pressed
  written down at the moment it is pressed. A hop table reconstructed afterwards is a summary, and
  DR-10's whole point is that a summary and a memory are indistinguishable.
- **Enter both doors, in either order, and quote what rendered.** Quote the sentence from the screen,
  not from the source. The static `rg` mirror check is run **afterwards** and reported alongside the
  observation as a separate line — it is corroboration, never the observation itself.
- **Stop at the first dead end and write it up.** Do not fix, do not retry, do not ask. Route the
  finding by slug and close the entry. A witness who repairs what they are witnessing has destroyed
  the observation, and this is the moment that happens.
- **Mount it last.** Add the link to `project.md`'s `## Companions` — body prose only, never the
  frontmatter, which the CLI owns and a `PreToolUse` hook denies anyway — in the same PR. An unlinked
  record in a story folder is a component that compiles and is never rendered.
- **Leave the second-question hops to the slice-mate.** `second-question-walk-records` (HS-S0162) owns
  AC-005; absorbing its hops here would put one record's failure in front of two project criteria.
- **Do not touch `crates/`.** Building rustdoc produces `target/`, which is ignored and is not a
  change. That is the only contact this story has with the workspace.

## Tests and CI (merge gate)

This project takes **no testing brief**, deliberately: `.bklg/docs-that-teach/_decomposition.md`'s
warranted-brief table gives it `architecture` and `ux` only, because AC-004, AC-005 and AC-009 are
*observed walks*, and `project.md`'s out-of-scope table names the owner of automated checking for
them as **"Nobody, deliberately"**. This story is one of the three instruments that omission is
defensible because of. The bar below is therefore the repository's standing gate — run to prove this
story disturbed nothing — plus the diff checks the mount requires, plus the reviewed read that is
the actual verification. It is honest about which is which.

| Tier | Command / path | Proves |
| --- | --- | --- |
| **Preflight — dependency** | `.bklg/docs-that-teach/reach-and-adapter-path/front-door-pointer/_ledger.md`: its AC-001, AC-002 and AC-005 rows are `satisfied: true` with cited evidence | There is a pointer to walk and rows P1/P2 exist. If the pointer was correctly **not** installed (`_design.md`'s `Empty` state), this story blocks — EC-002 |
| **Preflight — surface** | `cargo doc -p happenstance --no-deps`, then `target/doc/happenstance/index.html` exists and `#main-content > .docblock` resolves | The docs.rs stand-in can be produced at all. A failure here is EC-008 and is a gate failure in its own right, not a licence to read source |
| **Observation — render, 1024x768** | The built rustdoc root and the rendered Markdown of `crates/happenstance/README.md`, both at 1024x768, walked keyboard-only | **AC-003**, **AC-004**, **AC-006** — composition, placement, transience and hierarchy, which no textual assertion reaches. This is the test; everything else corroborates it |
| **Observation — arrival** | The destination passage opened at its fragment; its stated answered-need and its "assumes you have read" line read off the page | **AC-005** — arrival proved against the destination rather than asserted (invariants 3 and 4) |
| **Corroboration — mirror** | `rg -c "Guide-level documentation" crates/happenstance/src/lib.rs crates/happenstance/README.md` returns `1` for each; the two sentences extracted and diffed with `//! ` stripped | **AC-004** — the only live check on the mirror while the assertion `_design.md` asks of HS-P0020 does not exist. Run **after** the walk, reported as a separate line |
| **Corroboration — walker** | `git log --format='%an %s' -- crates/happenstance/src/lib.rs crates/happenstance/README.md` over `front-door-pointer`'s commits does not name the walker | **AC-002** — non-authorship is checkable, not merely stated |
| **Static — the record's composition** | `rg -n "<[a-z]+\|style=\|<details\|See also\|Next steps\|Further reading" walk-record.md` returns nothing; `rg -n "^#+ " walk-record.md` shows no skipped level; exactly one table | **AC-009** — `_design.md` anti-patterns 3, 5, 6, 9, 10 and `_storymap.md`'s standing constraints |
| **Static — the record's shape** | `rg -n "[0-9]{4}-[0-9]{2}-[0-9]{2}" walk-record.md` returns the walk date; the protocol heading precedes the first observation heading in `rg -n "^## "` order; the named commit sha resolves under `git cat-file -e` | **AC-001** — dated, sha-pinned, protocol-first |
| **Diff gate — the mount** | `git diff -U0 <merge-base> -- .bklg/docs-that-teach/reach-and-adapter-path/project.md` shows one added `## Companions` line, zero deletions, nothing above the frontmatter's closing `---` | **AC-007** — project **DoD item 4** resolves, and the CLI's single-writer rule is respected (`CLAUDE.md`, "Where the work lives") |
| **Diff gate — nothing repaired** | `git diff --name-only <merge-base>` shows **zero** files under `crates/`, `spec/`, `standards/`, `xtask/`, `docs/` or `.kb/` | **AC-008** — "the witness does not repair what they find" made checkable by the PR-boundary fence's deliberate absence of a `crates/**` glob |
| **Build gate** | `cargo xtask ci --fast` (`xtask/src/main.rs`), the bar `.redkiln/config.yaml` wires for a non-terminal project | Project **DoD item 1** — the tree is unchanged in the ways this story promises it is unchanged. It proves nothing *about* the record |
| **Build gate — story grain** | `cargo xtask affected --base main` | The grain `.redkiln/config.yaml`'s `verify:` block wires to stories: only what this diff could break, which here is nothing |
| **Spec gate** | `cargo xtask spec-trace` | No clause citation disturbed. This story discharges none and must break none |
| **Backlog gate** | `redkiln validate --kb && redkiln doctor` | The `project.md` body edit did not touch a system field, and the six expected `template-drift` advisories are still exactly six |
| **Story gate** | `redkiln verify --grain story` over `_ledger.md` and the PR-boundary fence | Every AC above carries a row, `satisfied: true`, with cited evidence — and no file changed outside the fence |

**What nothing checks, stated rather than papered over.** `_design.md`'s `## Density budget`, "Two
named gaps" (2): *"Nothing in `cargo xtask ci` verifies keyboard-only reachability or
self-describing link text; AC-004, AC-005 and AC-009 are dated observed walks by design, and the
absence of a linter is stated rather than papered over."* AC-003, AC-005 and AC-006 are verified by
a human reading the record against its own stated protocol. That is not a weakness of this spec; it
is the reason this story exists.

## Risks and coupling (PR-scoped)

| Risk | Exposure in this PR | Handling |
| --- | --- | --- |
| **The author becomes their own witness** | The single highest-value failure available here, and the cheapest to commit by accident — the implementing context has just been reading the pointer's spec. | EC-001 and the Implementation notes' first bullet. `_storymap.md` backbone **B6** put this walk in its own slice for exactly this reason; the walker is recruited before anything is opened, and non-authorship is corroborated by `git log`, not by assertion |
| **The walk is asserted rather than performed** | A complete, plausible, fictional record is indistinguishable from a real one on a `rg` check. | The protocol stanza (AC-001) is what a fabrication would have to lie about explicitly: a named date, a resolvable commit sha, a named walker, a stated contamination policy. The hop table filled in *during* the walk is the other half — DR-10's *"a walk that happened and was not written down"* runs in this direction too |
| **Contamination by the checkout** | The walker needs `cargo doc` output, and the tree is right there. | Someone other than the walker prepares the stand-ins; the record states that having the tree on disk was not permission to read it (AC-001); an accidental read is disclosed, not omitted (EC-006) |
| **Overclaiming into HS-P0024's territory** | One careless sentence spends a sibling project's only instrument and cannot un-spend it. | AC-002's bounded-claim line, plus EC-009 for the DoD-7/AC-004 strength gap. `project.md`'s risk table and `_storymap.md`'s standing constraints say it in the same words, and the spec repeats them verbatim rather than paraphrasing |
| **The mirror gap is this walk's alone to catch** | `_design.md` records that P2's mirror assertion **does not exist** and that the doctest mount at `crates/happenstance/src/lib.rs:10` compiles fences, not prose. | AC-004 enters through **both** doors. A single-door walk leaves the unguarded half unobserved, which is why the two-surface entry is a criterion rather than thoroughness |
| **Both external gates are still open** | HS-P0020's hosting shape and HS-P0022's pages gate the dependency, so the pointer may correctly not exist yet. | EC-002 — block, do not walk a placeholder. `_storymap.md`'s Merge order step 3 names both gates; `_grounding.md`'s "Sibling-project dependency state" records both siblings at `stage: storymap` with no `_design.md` and is **verified, not assumed** |
| **The merged tree moves the surface under the walk** | `crates/happenstance/src/lib.rs` is 75 lines here and reported at 237 on `initiative/from-contract-to-published-library`; the README is edited by HS-P0016. **Not verifiable from this worktree.** | Walk the tree that will merge, and name its commit sha in the record (AC-001). A walk against a stale copy records a surface nobody will meet |
| **Slice-mate coupling** | `second-question-walk-records` (HS-S0162) is implemented in the same context and mounted as one body of evidence; `_storymap.md`'s Merge order step 5 runs this one first. | Separate files, separate criteria. This story observes the front door only and must not absorb the second-question hops — that would put one record's failure in front of both AC-004 and AC-005 |
| **The temptation to fix** | The walker will find something. Fixing it is one line away and destroys the observation. | The PR-boundary fence carries no `crates/**` glob, so the temptation surfaces as a `redkiln verify --grain story` failure rather than as an invisible edit to the thing being observed (AC-008) |
| **A record that cannot fail** | A walk written to close a DoD item rather than to test one. | NF-006 and AC-008. The failed-walk path is a first-class merged artefact, not an exception branch; if no entry in this record could have said "failed", the record is decorative |

## Dependencies

**Blocks on**

- `front-door-pointer` (HS-S0158) — installs the DT-10 pointer on both front-door surfaces, the one
  authoritative 22-word sentence mirrored onto `crates/happenstance/src/lib.rs`'s crate root and
  `crates/happenstance/README.md`, and files register rows P1 and P2. There is nothing to walk until
  it has merged, and its **author may not be this story's walker**. Matches `_storymap.md`'s
  `depends_on` for this row exactly.

**Slice-mate, not a dependency**

- `second-question-walk-records` (HS-S0162) — same slice `reach-walks`, implemented in the same
  context and mounted as one integrated body of evidence. `_storymap.md`'s Merge order step 5 runs
  this story first. It carries project AC-005 and additionally depends on `evaluator-onward-links`;
  this story carries AC-004 and does not.

**Unlocks**

- Project **DoD item 4**, for one of its three walks. The other two are
  `second-question-walk-records` (AC-005) and `error-site-walk-record` (AC-009, a different slice
  with no dependency on this one).
- Initiative **DoD scenario 7**'s observation half. The installation half is `front-door-pointer`'s.
- **HS-P0024 `comprehension-evidence`**, downstream of the whole project — it needs the assembled
  surface, and it is the only instrument that may make the non-insider claim this record explicitly
  does not (BR-14).

**External gates, outside this project** — inherited through the dependency, not held directly

- **HS-P0020 `checked-documentation-surface`** — the hosting shape the href resolves against and the
  pinned tree the destination passage lives in. Also the owner of the mirror assertion this walk
  stands in for.
- **HS-P0022 `application-author-path`** — the narrative material the walk must arrive at.

## Anchors (progressive disclosure)

Load-bearing depth, deferred not optional. The `## Context pack` above is the must-read core and is
sufficient to start; each row below is opened at the moment named, never in bulk.

| Anchor | Why it is load-bearing | When to open it | Serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/reach-and-adapter-path/_design.md` | **Binding.** The yardstick every observation is made against: `## Density budget` (first screen ≈ 27 / ≈ 32 rendered lines, pointer within the first 5, sentence 8–30 words, link text ≥ 3 words) and its "Two named gaps"; `## Composition` (why region 2 sits above region 3); `## Signatures` (the pinned 22-word sentence and the mirror substring); `## Transience policy`; `## Anti-patterns`; `## The states the API must express` ("Refused") | Before the walk, to fix what Observation A is measured against — read `## Density budget` and `## Composition` first; again at the mirror comparison for `## Signatures` and anti-pattern 2 | AC-003, AC-004, AC-008, AC-009 |
| `.bklg/docs-that-teach/reach-and-adapter-path/design/mock.html` | The **approved** frames — 38 labelled, including `crate-root-front-door` / `first-screen-1024x768` and `readme-front-door` / `first-screen-crates-io`. It is what "found without scrolling" was signed off to look like, and its six findings amended three numbers written elsewhere in the design | At the render-observation tier, after `cargo doc`; open at `#all` and wait for "all 38 frames built" | AC-003, AC-004 |
| `.bklg/docs-that-teach/reach-and-adapter-path/_decomposition.md` | The UX brief's **interaction invariants 1–5** with their falsifications, the **Accessibility floor** (the keyboard-only clause that makes AC-006 a criterion rather than a courtesy, and the enumerated affordance set: search `S` or `/`, collapse `+` / `-`), and **UX-AC-01, UX-AC-02, UX-AC-04, UX-AC-09** | Before writing the hop table and the keyboard statement; again when judging whether an arrival was an arrival | AC-003, AC-005, AC-006 |
| `.bklg/docs-that-teach/reach-and-adapter-path/project.md` | The **mount point** (`## Companions`), **AC-004** verbatim, **DR-10** (*"a walk that happened and was not written down is indistinguishable from one that did not"*), **DoD item 4**, and the risk table's non-authors-not-non-insiders row that AC-002's bounded claim is copied from | At the start for AC-004 and DR-10; at the end, to add the `## Companions` link — body prose only | AC-002, AC-007, AC-008 |
| `.bklg/docs-that-teach/reach-and-adapter-path/front-door-pointer/spec.md` | The dependency's own spec: what was installed, where, at which ladder rung, and its EC-001 (*the pointer is not installed if only rung 4 is available*). Reading it tells you whether there is anything to walk — and it is the one anchor the **walker** must not open | Before recruiting, by whoever runs the story; never by the walker | AC-002, AC-004 |
| `.bklg/docs-that-teach/reach-and-adapter-path/_storymap.md` | Backbone **B6** and the reason this walk is its own slice (*"the author is never their own witness"*), the Merge order step 5 ordering against the slice-mate, and the standing constraints the record's own composition inherits | At the start, and again before declaring done | AC-002, AC-008, AC-009 |
| `.bklg/docs-that-teach/reach-and-adapter-path/_grounding.md` | "Sibling-project dependency state", **verified not assumed**: HS-P0020 and HS-P0021 both at `stage: storymap` with no `_design.md`. Also the line-level confirmation of both front-door files and the **NOT VERIFIABLE HERE** marker on the merged sibling branch | Before assuming an external gate has opened, and before trusting any line number in this spec | AC-004, EC-002 |
| `.bklg/docs-that-teach/initiative.md` | **DoD scenario 7** verbatim — *"a person with no prior knowledge of this repository's layout"* — which is the stronger condition EC-009 exists for, plus **BR-08** and **BR-14** (the claim-scoping requirement AC-002's bounded line implements) | When writing the walker-identity and bounded-claim stanzas, and when reporting what DoD 7 this story actually advanced | AC-002 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | **Persona 3** at `:225-229` (*"reads for twenty minutes and decides whether to depend on this"*) and Cross-persona tensions at `:304-308` (*"one reading session, with no second attempt if the first one fails silently"*) — the source of the reading budget and of the framing every criterion is written from | When setting the budget before t=0, and when judging whether a slow success is a finding | AC-005 |
| `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` | The two-surface split (tokio / serde / diesel convergence) the pointer imitates, and the anti-pattern that the evaluator's gap *"is evidence of a missing link, not a missing widget"* — the standard against which a "the docs are hard to navigate" finding must be re-stated as a missing link | Only if the walk produces a finding that sounds like it wants a widget | AC-005 |
| `crates/happenstance/README.md` | One of the two entry points, **read as rendered, never as source during the walk**. Lines 1-4 and the status blockquote at `:6-11` are the placement the offer must precede | Afterwards, for the corroborating mirror check only | AC-003, AC-004 |
| `crates/happenstance/src/lib.rs` | The other entry point's source. `:10` is the README-as-doctest mount that compiles fences and **not prose** — the reason the mirror has no live guard; `:11-71` is the crate root the rustdoc render comes from | Afterwards, for the corroborating mirror check; and when writing why AC-004 needed both doors | AC-004 |
| `.bklg/docs-that-teach/reach-and-adapter-path/second-question-walk-records/spec.md` | The slice-mate's boundary. Reading it is how you keep this record from absorbing the second-question hops and putting one record's failure in front of two project criteria | Before writing the hop table, if there is any doubt about where this walk stops | AC-005 |
| `.bklg/docs-that-teach/reach-and-adapter-path/front-door-walk-record/discover.md` | The named wrong implementation in one line — *"A walk performed by an insider, who routes around the rough spots without noticing them"* — and the signal ledger showing no discovery fan-out was warranted | At the start, when briefing the walker | AC-002, AC-008 |
| `CLAUDE.md` | "Where the work lives": the CLI is the single writer of an item's system frontmatter and a `PreToolUse` hook denies the edit — which is what makes AC-007's mount *body prose only*. Also the Commands block for the gate this story runs to prove non-disturbance | Before editing `project.md`, and before running the gate | AC-007 |
| `xtask/src/main.rs` | The gate defined once — `cargo xtask ci --fast` and `spec-trace`. This story adds nothing to it; the anchor is here so the implementer can see that, rather than assume a new step is owed | Before claiming the build gate is green | AC-007 |
| `.redkiln/config.yaml` | Lines around 15 and 75-81: `design.capture` is deliberately absent, so the perceptual review is a standing skip and no downstream instrument will look at these surfaces. This walk is the closest thing that exists | If anyone asks why a human read is the verification for AC-003 and AC-006 | AC-003, AC-006 |
| `crates/happenstance-core/src/store.rs` | Line 77 is the live instance of the **named-but-unlinked** cross-reference form — ladder rung 3. If the front-door pointer landed on that rung, this is the shape the walker met and the record should recognise it as a pointer rather than as prose | Only if the walk finds a named-but-unlinked destination instead of a link | AC-005 |

## Clarifications resolved during spec

1. **The AC ids are exactly the nine the front half decided** — AC-001 through AC-009. None added,
   none dropped, none renumbered. The ledger matches this table row for row.
2. **AC-004 is a story criterion, not the project's AC-004.** The collision is unfortunate and is
   called out here once so no reader conflates them: this spec's `AC-004` is *"both front doors
   entered and the mirror compared"*; the project's `AC-004` is the whole front-door walk and is
   carried by all nine rows. `tracesTo` in the digest is the project's.
3. **Both front doors, not one, and it is a scope decision rather than thoroughness.**
   `_design.md`'s "Two named gaps" (1) records that P2's mirror assertion does not exist, that the
   doctest mount at `crates/happenstance/src/lib.rs:10` compiles the README's ```` ```rust ````
   fences and not its prose, and that the step is *asked of* HS-P0020 rather than forked into
   `xtask/src/`. Until it lands, this walk is the only thing in the repository that reads both front
   doors and notices if they disagree. A one-door walk would leave the unguarded half unobserved.
4. **The mirror `rg` check corroborates the walk; it does not constitute it.** It reads *source*, and
   the criterion is about what two *renderers* put in front of a reader. It is therefore run after
   the walk and reported as its own line — the design's anti-pattern 2 is a screenshot check, not a
   grep check.
5. **DoD scenario 7 is stronger than project AC-004, and the gap is recorded rather than closed by
   reading.** DoD 7 says *"no prior knowledge of this repository's layout"*; AC-004 says *"did not
   install the pointer"*. Where the available walker meets the second and not the first, EC-009
   requires the record to say so and the DoD-7 claim to be reported as partially met, with the
   residual owed to HS-P0024 (BR-14). Quietly reading DoD 7 down to AC-004 would close a scenario
   that is not closed.
6. **This story renders no surface, and the composition invariants still bind — in two directions.**
   `_design.md`'s `## Surfaces` declares five and this story renders none; naming it as a renderer of
   `crate-root-front-door` or `readme-front-door` would double-count work `front-door-pointer` owns.
   The design's numbers bind here as the **yardstick** the observation is measured against (AC-003,
   AC-004), and its anti-patterns bind the artefact this story **writes** (AC-009). Both are AC rows,
   not prose bullets.
7. **No conformance rule, no clause, no public item, no data.** Nothing here is adapter-observable;
   `crates/happenstance-testkit/src/suite.rs` could not observe a walk record, and a rule that no
   adapter can fail is decorative (`CLAUDE.md`). `spec/SPECIFICATION.md` is not edited;
   `cargo xtask spec-trace` runs to prove non-disturbance, not to discharge anything.
8. **There is no testing brief and this spec does not invent one.** `_decomposition.md`'s
   warranted-brief table withholds it and `project.md` names the owner of automated checking for the
   three walks as *"Nobody, deliberately"*. The Tests and CI table above therefore separates the
   *observation* tiers (the actual verification) from the *corroboration*, *diff* and *gate* tiers
   (which prove the tree is unchanged), and states plainly which criteria no linter reaches.
9. **The record is append-only and sha-pinned, which is why a later href rebinding does not
   invalidate it.** EC-010 and NF-004 make this explicit because the alternative — silently
   re-walking and editing the entry — is exactly the *"quietly re-walked"* failure invariant 5
   forbids, and because a transcript that is edited is no longer a transcript.
10. **The register is read, never written.** `pointer-policy-and-inventory` owns it and
    `front-door-pointer` files rows P1 and P2. This story may confirm both rows exist with non-empty
    guards and route a discrepancy as a finding; it files no row, and it never renders the register
    as a page (`_design.md` anti-pattern 14).
