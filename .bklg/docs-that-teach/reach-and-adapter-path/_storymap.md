---
item: HS-P0023
stage: storymap
created: 2026-08-17T03:21:39.043Z
updated: 2026-08-17T03:21:39.043Z
template_sig: 1c63534a
rendered_sig: d8e30914
---

# Story Map — Reach and the Adapter Path

Eight stories in five slices. The spine is `project.md`'s AC-001..AC-012; the slice
cut follows the architecture brief's N-10 split (`_decomposition.md` companion,
"Sequencing the implementer inherits") because AC-A08 makes expressing that split a
condition of the project being checkable at all: *"The adapter-half stories carry no
dependency on HS-P0022, and the reach-half stories carry both of theirs… collapsing
it costs the project its only available parallelism."*

## Backbone

The activities a reader performs, left to right. Every story below sits under one of
them. Two are the project's own (the first and the last); four are a persona's.

| # | Activity | Whose | What "done" looks like from outside |
| --- | --- | --- | --- |
| **B1** | **Decide, once, how this surface points outward** | the maintainer | DT-10 is resolved and the resolution is *data in the tree*, not a preference in a review comment |
| **B2** | **Arrive at the front door and learn that guide material exists** | evaluator (A0) | after `cargo add happenstance`, both surfaces a developer meets say guide-level material exists and where |
| **B3** | **Ask the second question and reach its answer** | evaluator (A1→A3) | two named stall points each land on an answering *passage*, no dead end, no bespoke widget |
| **B4** | **Hit `error[E0034]` and find out what happened, in place** | adapter author (B0→B1) | the string rustc emitted is in the file already open, and the fix is stated without a hop |
| **B5** | **Follow the reasoning, not just the recipe** | adapter author (B2→B3) | one hop from `store.rs` reaches a sequenced account that cites rather than restates, and says what `MemoryEventStore` is *not* |
| **B6** | **Observe that each path is actually walkable, and write it down** | the project | three dated, keyboard-only records, each naming a walker who is not the author of what they walked |

**B6 is a separate activity on purpose**, not a checkbox inside B2–B5. AC-004,
AC-005 and AC-009 each require an observer who did not install the thing being
walked (`project.md`, DR-10; DoD item 4), and the implement stage hands a whole slice
to *one* context. Putting a walk in the same slice as the surface it walks would make
the author their own witness, which is the one thing those criteria forbid.

## Slices

Grouped by milestone. `capability` = a user-observable slice through every layer;
`foundation` = real in-tree substrate consumed and demonstrated by a capability slice
in this same project (`pointer-policy` is consumed by all four of them).

| Milestone | Story | Archetype | One-line slice | depends_on | traces_to |
| --------- | ----- | --------- | -------------- | ---------- | --------- |
| `pointer-policy` | `pointer-policy-and-inventory` | foundation | Land `_design.md`'s DT-10 resolution as in-tree substrate: the permitted pointer forms, the enumerated pointer inventory with a named rot-guard per row, and the widget-rejection record — so no later story chooses a pointer shape case by case. | — | AC-001, AC-003, AC-011 |
| `adapter-error-site` | `adapter-reasoning-account` | capability | Author the sequenced adapter reasoning account as a registered page in HS-P0020's pinned tree: the six existing sources in the order an adapter author needs them, each cited in anchored named-subject+path form, with the "`MemoryEventStore` is the conformance oracle, not an adapter" caveat where it is first sequenced. | `pointer-policy-and-inventory` | AC-008, AC-012 |
| `adapter-error-site` | `store-error-site-rewrite` | capability | Rewrite `crates/happenstance-core/src/store.rs`'s module doc behind HS-P0020's clause-id pin: restore rustc's real `error[E0034]` block including its `= note:` candidate lines and `TraitVariantBlanketType`, state the *narrow* unchecked limit, keep the in-place fix ahead of any hop, add the one guarded pointer to the reasoning account, and run `cargo xtask spec-trace` over the result. | `pointer-policy-and-inventory`, `adapter-reasoning-account` | AC-003, AC-006, AC-007, AC-010, AC-012 |
| `adapter-error-walk` | `error-site-walk-record` | capability | A person who did not write the rewrite reproduces the E0034 collision, reaches the reasoning account keyboard-only from the file and the message alone, and leaves a dated record naming each hop and their relationship to the work. | `adapter-reasoning-account`, `store-error-site-rewrite` | AC-009 |
| `front-door-reach` | `front-door-pointer` | capability | Install the DT-10 pointer on both front-door surfaces — `crates/happenstance/src/lib.rs`'s crate root and `crates/happenstance/README.md` — above the fold, displacing nothing, with copy that is true whichever hosting shape HS-P0020 resolves to, and register both rows in the inventory. | `pointer-policy-and-inventory` | AC-002, AC-003 |
| `front-door-reach` | `evaluator-onward-links` | capability | Name the two second questions from DR-4's stall points and make each one hop from the page that answered the first, targeting the answering *passage* — using only intra-doc links, `#[doc(alias)]`, rustdoc search and the destination's own TOC, and adding no navigation component. | `pointer-policy-and-inventory` | AC-003, AC-005, AC-012 |
| `reach-walks` | `front-door-walk-record` | capability | Someone who did not install the pointer starts from nothing but what `cargo add happenstance` shows and reaches the narrative material keyboard-only; dated record of what they opened, in order. | `front-door-pointer` | AC-004 |
| `reach-walks` | `second-question-walk-records` | capability | Each of the two named second questions is walked keyboard-only from the first-question page to the passage that answers it, one dated record per question, and any dead end is recorded as a failed walk rather than quietly re-walked. | `front-door-pointer`, `evaluator-onward-links` | AC-005 |

### Why these slices and not others

- **`adapter-reasoning-account` and `store-error-site-rewrite` are one slice.** The
  rewrite's whole point is a pointer into the account; shipping the account with
  nothing pointing at it, or the pointer with nothing behind it, is exactly the
  "build it" / "wire it in" split a slice is supposed to prevent. They are also the
  two edits that must both sit inside HS-P0020's checked surface (DoD item 5).
- **`front-door-pointer` and `evaluator-onward-links` are one slice.** Both are
  applications of the same freshly-decided policy to the same persona's single
  reading session (`personas-and-journeys.md`, Cross-persona tensions: *"Persona 3's
  entire journey happens inside one reading session"*). A front door that points at
  material with no onward link reproduces the axum ordering failure the UX brief
  names as the recorded cost.
- **The two walk slices are separate from each other, not merged into one.**
  `adapter-error-walk` depends on nothing HS-P0022 owns; `reach-walks` depends on
  HS-P0022's pages existing. Merging them would put the adapter walk behind a sibling
  project it has no relationship with and forfeit the parallelism AC-A08 requires the
  map to express.

## Coverage

Every project AC is owned by at least one story, and no responsibility is owned
twice. Where an AC appears against more than one story the responsibilities differ,
and the difference is stated.

| Project AC | Story / stories | Note on split ownership |
| --- | --- | --- |
| **AC-001** DT-10 resolved in writing | `pointer-policy-and-inventory` | Sole owner. The story consumes `_design.md`'s resolution and is what makes it binding rather than recorded. |
| **AC-002** front door points outward | `front-door-pointer` | Sole owner. |
| **AC-003** every pointer resolves and something checks it | `pointer-policy-and-inventory`; `store-error-site-rewrite`; `front-door-pointer`; `evaluator-onward-links` | The foundation story owns the *register* and the guard rule; each installing story owns its own *rows*. A story that installs a pointer and files no row has not finished. |
| **AC-004** front-door walk observed | `front-door-walk-record` | Sole owner. |
| **AC-005** two second questions walked | `evaluator-onward-links`; `second-question-walk-records` | The first *names* the two questions and builds the hops (DR-4 requires naming before answering); the second *walks* them, and must be a different pair of hands. |
| **AC-006** `store.rs` carries rustc's own output | `store-error-site-rewrite` | Sole owner. |
| **AC-007** the error site points at the reasoning | `store-error-site-rewrite` | Sole owner of the pointer; the destination is `adapter-reasoning-account`'s (AC-008). |
| **AC-008** the reasoning account is sequencing, not volume | `adapter-reasoning-account` | Sole owner. |
| **AC-009** error walk observed | `error-site-walk-record` | Sole owner. |
| **AC-010** frozen documentation MUSTs still discharged | `store-error-site-rewrite` | Sole owner — it is the only story that edits a `happenstance-core` doc comment, which is why it and nothing else sits behind HS-P0020's pin. |
| **AC-011** no navigation widget, rejection on record | `pointer-policy-and-inventory` | Sole owner. The rejection is a policy artefact, not a per-pointer one; the installing stories inherit it as a constraint. |
| **AC-012** pages obey the discipline and the specification | `adapter-reasoning-account`; `store-error-site-rewrite`; `evaluator-onward-links` | One per surface authored: the narrative page, the module doc comment, and any linking prose added to a sibling-owned page. `front-door-pointer` is excluded deliberately — it adds one sentence and authors no page (DoD item 7 makes that checkable by diff). |

**No AC is orphaned**: AC-001 through AC-012 each appear above. **No story is
untraced**: all eight carry at least one AC. **Project DoD coverage falls out of the
same table** — DoD 1 and 5 are conditions on every story's merge, DoD 2 lands with
`store-error-site-rewrite`, DoD 3 is the design gate AC-001 consumes, DoD 4 is the
three stories of `adapter-error-walk` and `reach-walks`, DoD 6 is AC-003's split
ownership above, and DoD 7 is `front-door-pointer`'s diff.

## Merge order

Foundation first, then the two independent halves, each followed by its own witness.

1. **`pointer-policy`** — `pointer-policy-and-inventory`. Blocks everything; nothing
   blocks it inside this project. Its own precondition is this project's design gate
   (AC-001), which `_decomposition.md` records as runnable while HS-P0022 is still
   authoring ("Available parallelism").
2. **`adapter-error-site`** — `adapter-reasoning-account`, then
   `store-error-site-rewrite`. **Hard external gate**: HS-P0020's clause-id pin must
   be committed and readable before the *first* edit to
   `crates/happenstance-core/src/store.rs`; copy may be drafted earlier,
   implementation may not start (N-6, step 1). HS-P0020's pinned tree must also exist
   for the account to be a registered page rather than a loose file (AC-A03). Carries
   no HS-P0022 edge.
3. **`front-door-reach`** — `front-door-pointer` and `evaluator-onward-links`, in
   either order within the slice. **Hard external gates**: HS-P0022's pages must exist
   (there is nothing to point at otherwise), HS-P0020's hosting shape must be resolved
   before the destination path is bound, and the **merge-forward rule applies
   verbatim** — `crates/happenstance/src/lib.rs` is 75 lines on this branch against
   237 on `initiative/from-contract-to-published-library`, and placement is an
   invariant-2 judgement, not a mechanical rebase. **Not verifiable from this
   worktree**; re-verify against the merged tree before binding sequencing to it.
4. **`adapter-error-walk`** — `error-site-walk-record`. Runs as soon as slice 2 has
   merged; it does **not** wait on slice 3.
5. **`reach-walks`** — `front-door-walk-record`, then `second-question-walk-records`.

Slices 2 and 3 are concurrent, and so are 4 and 3. The only total order in the graph
is `pointer-policy` → everything, which is what a foundation is for.

### Standing constraints every story inherits

Stated once here rather than repeated in eight specs.

- **No public item, signature, feature or manifest changes.** A candidate solution
  that needs one — the `prelude` module at N-9 is the live example — is outside this
  project and owes an ADR, not a documentation story.
- **No widget, no raw HTML, no inline styles, no folded or tabbed content**, and no
  skipped heading levels. DT-7 is HS-P0020's.
- **Self-describing link text**; no "here", no bare URL into this repository's own
  tree.
- **Every walk is keyboard-only and its record says so** (UX-AC-09).
- **The honest claim is bounded**: these walkers are non-authors, not non-insiders.
  Evidence that the path exists is not evidence that a stranger finds it — that claim
  is HS-P0024's alone (BR-14).
