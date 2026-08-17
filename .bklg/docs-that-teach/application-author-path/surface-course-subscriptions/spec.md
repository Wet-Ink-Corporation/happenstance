---
item: HS-S0188
stage: spec
created: 2026-08-17T13:16:35.851Z
updated: 2026-08-17T13:16:35.851Z
template_sig: 87bbf1d0
rendered_sig: 02a3f0ff
---

# Spec — The worked example surfaced in its own words

## Scope lock

| | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` — AC-02 `:363-366`, DoD 8 `:441-444`, DoD 9 `:445-448` |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` |
| Project | `.bklg/docs-that-teach/application-author-path/project.md` — AC-010 `:263-265`, in-scope `:100-105`, routing `:300-302` |
| This spec | `.bklg/docs-that-teach/application-author-path/surface-course-subscriptions/spec.md` |
| Key briefs | `.bklg/docs-that-teach/application-author-path/_decomposition.md` — UX brief `:11-446` (UI-5 `:79-84`, UX-010 `:353-357`, UX-011 `:358-361`, the vocabulary-seam Note `:382-392`), testing brief `:448-651` (tier table `:480-486`, AC-010 row `:531`) |
| Signed-off design — **binding** | `.bklg/docs-that-teach/application-author-path/_design.md` — surface `worked-example-handoff` `:61-64`, composition `:494-504`, hierarchy `:667-674`, `## Items` `:271-289`, `## Placement and re-export` `:707-751`, `## Anti-patterns` `:831-882`, `## Gaps in the substrate` `:886-919`, sign-off conditions `:1039` |
| Substrate design — binding, HS-P0020's | `.bklg/docs-that-teach/checked-documentation-surface/_design.md` — D1 "the markdown is the render" `:165-176`, `TREE`/`HARNESS` `:92-106`, harness placement `:491-499`, page budgets `:388-397`, path budget `:425-426`, checker states `:560-562` |
| Page-need design — binding, HS-P0021's | `.bklg/docs-that-teach/page-need-discipline/_design.md` — declaration form `:104-118`, the closed `NEEDS` set `:174-176`, `RP-10-2`/`RP-10-3` `:225-229` |
| Baseline this story is authored against | `.bklg/docs-that-teach/application-author-path/merge-forward-preflight/spec.md` — the four measured merge contradictions `:106-129`, and the `_baseline.md` that story authors `:292-294` |
| Story map / merge order | `.bklg/docs-that-teach/application-author-path/_storymap.md` — this row `:60`, why the slice holds bridge + handoff `:75-78`, routing `:158-163` |
| Roadmap pointer | `RUNBOOK.md` — this initiative is documentation work and adds no phase; the roadmap is not amended here |

## One-line PR slice

Bring the worked example within the reader's reach: surface
`examples/course-subscriptions/src/main.rs`'s module doc verbatim **by construction** —
extracted once into `overview.md` and included back — and land the
`worked-example-handoff` page in the pinned tree with the orientation prose, the DT-1
anchor citation and the one vocabulary-seam sentence that tell the reader what they are
about to read before they leave for it.

## Executive summary

This PR lands **one file extraction, one new page, one harness registration, one link on
the slice-mate's page, and one test that makes the reach fail loudly when it breaks.** No
fence, no program, no clause.

The pointer is project AC-010 (`project.md:263-265`) and the design's
`worked-example-handoff` surface (`_design.md:61-64`, `:494-504`). The delta this spec adds
is three things the design could not have decided when it was written:

1. **The vocabulary seam moved, and this story owns where it lands.** The signed-off
   design spends one composition slot on a sentence naming that the example imports
   `happenstance_core` directly (`_design.md:497-502`). The merge forward deletes that
   premise — the merged example imports `happenstance` at `:33-37`, which is what ADR-0006
   asks for — and `merge-forward-preflight` dispositions the sentence's fate explicitly to
   this story (`merge-forward-preflight/spec.md:114-119`). The seam is **not gone**; it
   moved one layer up, and it is sharper: the merged example contains no `Query` and no
   `AppendCondition` at all, because `happenstance::commit` derives both. A reader who has
   just been taught to write those by hand will look for them, fail to find them, and
   conclude the bridge lied. That is the friction this story names at the link.
2. **"An include of `overview.md`" is not available on the page.** The design composes the
   handoff page with the surfaced module doc as *an include* (`_design.md:502`), written
   before HS-P0020 settled D1 — **the markdown is the render**, no build step, no
   preprocessor (`checked-documentation-surface/_design.md:165-176`). Markdown has no
   include. This spec resolves the mechanism (below) rather than either faking it with a
   pasted copy or quietly dropping AC-010's verbatim bar.
3. **The module doc grew.** It is `:1-28` post-merge, not `:1-19`, gaining a
   `# What is not in this file, and used to be` section that says in the example's own
   voice most of what a paraphrase would have wanted to say
   (`merge-forward-preflight/spec.md:120-123`). The extraction takes the whole block; the
   page must not restate any of it.

## Context pack

**The decision this story is built on: surfaced means the reader meets the author's own
bytes, and paraphrase is made impossible rather than forbidden.** AC-010's bar is
"surfaced rather than paraphrased" (`project.md:263-265`), and the design buys it *by
construction*: one file of bytes rendered by a mechanism that cannot reword it, rather
than by a reviewer diffing prose (`_design.md:728-733`). The initiative's own risk row —
"the project is measured in pages written" (`project.md:343`) — is the reason: the module
doc is already one of the three strongest explanations this project has written
(`project.md:100-105`), so the work is reach, not authorship.

**The extraction, and its exact shape.** `examples/course-subscriptions/src/overview.md`
is new and holds the module doc's text; `main.rs`'s `//!` block becomes
`#![doc = include_str!("overview.md")]`. This is the shape already wired one crate over at
`crates/happenstance/src/lib.rs:10`, with its reasoning at `:1-9`. The governance test
applies and is the acceptance bar for the move: does the edit change what the document
*asserts*? (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`.) It must not —
this is a move of bytes, not a rewrite, and the example's imports are not touched
(`_decomposition.md:382-392`).

**How the reader meets those bytes, since markdown cannot include them.** The page links
to `overview.md` and never restates it. That is a decision with a cost and it is taken
here rather than deferred:

- **Chosen — one copy, one hop.** `overview.md` is the single copy in the tree. The
  example's rendered documentation is byte-identical to it because `include_str!` says so;
  the handoff page reaches it in one hop and quotes none of it. Paraphrase is then not a
  discipline anyone has to keep — there is no second copy to drift.
- **Rejected — paste the module doc onto the page and assert byte-containment in a test.**
  It satisfies the design's "bulk of the page" hierarchy (`_design.md:667-671`) and fails
  everything else: it puts two copies of one explanation in the tree, and it makes the page
  an `explanation` page whose explanation is someone else's, which collides with
  `RP-10-2`'s ceiling for the need this page actually answers
  (`page-need-discipline/_design.md:225-229`).
- **Rejected — move the single file into `docs/` and have `main.rs` include it from
  there.** It would put this page's answered-need declaration and its outbound links
  inside the example crate's own rendered documentation. AC-012 requires the declaration
  on this page; the example's docs are not this page.
- **Routed, not swallowed.** The residual — that a narrative page cannot include a source
  file's module doc — is a substrate question, and `_design.md:736-741` pre-authorises
  exactly this routing: if HS-P0020's mechanism ever *can* include it directly, the
  extraction becomes unnecessary and should be dropped. It goes to HS-P0020 as a recorded
  gap under project DoD item 9 (`project.md:300-302`), not into this page as a workaround.

**The page's need is `orientation`, and that is a real choice.** HS-P0021's closed set is
`orientation` / `tutorial` / `how-to` / `explanation`, and `orientation`'s success
condition is that the reader "leave, correctly, within one screen"
(`page-need-discipline/_design.md:174-176`) — which is UI-5 and the design's `Handed off`
state word for word (`_decomposition.md:79-84`; `_design.md:696`). The declaration is a
single blockquote immediately under the H1, in HS-P0021's notation and no other:
``> **Answers:** `orientation` — <the reader's question>?``
(`page-need-discipline/_design.md:104-118`). Inventing a second notation is a defect even
if it renders identically (DR-14, `_decomposition.md:132-136`).

**The DT-1 anchor is cited here, and this page is the one place the reason is not
obvious.** The prior model is named exactly once in the whole page set — the bridge's
`## Where your streams went` (`_design.md:108-114`) — and anti-pattern 6 forbids the words
"aggregate" or "which stream" on the crate root and on every step of the encounter
(`_design.md:852-854`). The text this page hands the reader forward into *does* use that
vocabulary, in another voice and correctly: "none of which fits inside a single aggregate"
and "make `Course` the aggregate and invariant 3 needs a read model plus a saga"
(`examples/course-subscriptions/src/main.rs:3-12`). So this page cites the anchor rather
than re-arguing it (UX-009, AC-009), and it does so *before* the link, because a reader who
follows the link does not come back.

**The seam sentence, re-aimed.** UX-011 makes `use happenstance::{…}` the taught
vocabulary per ADR-0006 (`.kb/decisions/0006-bare-name-to-the-typed-layer.md`), and
anti-pattern 13 permits the other crate's name in exactly one place: the seam sentence on
this page (`_design.md:871-874`). **Post-merge this story declines that permission** —
naming `happenstance_core` would now be false, because the merged example imports
`happenstance`. The sentence keeps its slot, its length (one) and its position
(immediately before the link, `_design.md:497-502`), and takes the seam that actually
exists: the example is the same boundary one layer up, where the `Query` and the
`AppendCondition` the bridge taught are derived by `happenstance::commit` from a
`DecisionModel` rather than written out. It must say that in its own words *about the
reader's expectation*, and must not restate the example's own
`# What is not in this file, and used to be` section — that section is the surfaced text,
and restating it is the paraphrase AC-010 forbids.

**The persona-journey slice.** Backbone activity A5 — *"I found the good example. Let me
actually read it."* (`_storymap.md:45`). It is the last step of the `conceptual-bridge`
slice and deliberately not a slice of its own: the UX state table runs `Mid-bridge`
straight into `Handed off`, and the seam sentence lives at the link
(`_storymap.md:75-78`; `_decomposition.md:106-108`). The reader arrives having just
carried their own invariant across; they leave to three invariants that do not fit an
aggregate.

**The substrate this story consumes and must not build.** The tree is `docs/`, pinned as
`const TREE: &str = "docs"`, and the harness is `xtask/src/narrative.rs`
(`checked-documentation-surface/_design.md:92-106`, `:491-499`). A page under `docs/` that
the harness does not `include_str!` is the checker's *unregistered* problem, not a page
(`:560-562`). Three budgets bind this page and all three are met by the design's own route:
filename ≤ 32 characters (`docs/read-the-worked-example.md` is 31 all-in), H1 ≤ 40
characters, page ≤ 250 source lines, prose wrapped at ≤ 90 columns
(`checked-documentation-surface/_design.md:388-397`, `:425-426`). **If
`xtask/src/narrative.rs` does not exist when this story is implemented, stop and report
it** — that is HS-P0020's substrate and the halt is the correct behaviour, not a reason to
invent a second harness (`_decomposition.md:118-131`).

**What this story may not decide.** `_design.md` is signed off and binding
(`_design.md:1039`); this story implements the `worked-example-handoff` surface and does
not re-open DT-1, DT-4, DT-5 or DT-6. It adds nothing to `docs/README.md:12-24` — the
signpost table is tempting and every pointer policy is HS-P0023's (`_design.md:749-751`).
It does not edit the example's imports, its `Cargo.toml`, or any assertion it makes
(`_decomposition.md:382-392`). Reach gaps found while authoring go to HS-P0023, substrate
gaps to HS-P0020, comprehension doubts to HS-P0024, incidental bugs to the `support`
initiative (`.redkiln/config.yaml:5`), per project DoD item 9 (`project.md:300-302`).

## Integration contract

- **Archetype**: `capability` — a user-observable slice: a reader on the bridge page
  reaches a page in the pinned tree and leaves it holding the example's own explanation.
  Nothing here is a double, a fixture or a `todo!()`.
- **Slice / milestone**: `conceptual-bridge`. Slice-mate:
  `invariant-to-appendcondition-bridge` (HS-S0187), which this story blocks on and whose
  page carries the outbound link and the DT-1 anchor this page cites. The two are
  implemented in one context and mounted as one surface (`_storymap.md:59-60`, `:75-78`).
- **Mount point**: **`xtask/src/narrative.rs`** — the harness that makes a markdown file a
  *checked page*. The mount is one `#[cfg(doctest)] mod read_the_worked_example { #![doc =
  include_str!("../../docs/read-the-worked-example.md")] }`, one module for one included
  file, exactly the shape and exactly the reason `xtask/src/constitution.rs:11-18`,
  `:39-50` already pay for. It is reached from the `REQUIRED` narrative steps mounted in
  `xtask/src/main.rs`; an unregistered page is a checker problem, so "constructed but not
  mounted" is a state this substrate does not permit
  (`checked-documentation-surface/_design.md:491-499`, `:560-562`).
- **Wires into**:
  - `examples/course-subscriptions/src/main.rs` — the second render path:
    `#![doc = include_str!("overview.md")]`, which is what makes "verbatim" a property of
    the build rather than of a reviewer (`_design.md:728-733`; precedent
    `crates/happenstance/src/lib.rs:1-10`).
  - `examples/course-subscriptions/src/overview.md` — new; the single copy of the
    explanation, and the page's link target.
  - `docs/carry-your-invariant.md` — the slice-mate's bridge page: the one outbound link
    to this page, and the `#where-your-streams-went` anchor this page cites
    (`_design.md:108-114`, `:475-491`).
  - `examples/course-subscriptions/tests/` — post-merge the example carries a test target
    (`merge-forward-preflight/spec.md:124-129`); the reach check lands there, swept by the
    `"tests"` `REQUIRED` step (`_decomposition.md:552-556`).
  - `.redkiln/config.yaml:40,55` — `affected_gate` and `integration_scoped`, the two
    commands redkiln runs at this story's and this project's grains whether or not anyone
    types them.
- **Renders surfaces**: **`worked-example-handoff`** (`_design.md:61-64`) — new, at
  `docs/read-the-worked-example.md`, in the composition fixed at `_design.md:494-504` and
  the hierarchy at `:667-674`. It also adds **one link line** to `conceptual-bridge`
  (`_design.md:57-59`), which is wiring, not a re-composition of the slice-mate's page.
- **Public items**: **none.** The design's `## Items` block declares that this project adds,
  changes and removes no public Rust API item (`_design.md:266-289`); the one row this
  story realises is `examples/course-subscriptions/src/overview.md`, `kind: doc`,
  `change: added`, in a `publish = false` crate (`examples/course-subscriptions/Cargo.toml:8`).
- **Conformance rule(s)**: **none, and it is not adapter-observable.** No port, no store,
  no fixture, no `suite.rs` rule. Naming one would be decorative by CLAUDE.md's own test.
  What observes this story is the gate: the narrative registration, the `"tests"` step over
  the reach check, and `cargo xtask affected --base main` at the checkpoint.
- **Clause(s)**: **none discharged, none amended.** The initiative is additive and
  discharges no clause (`project.md:155-157`). This page makes no normative claim, so it
  cites none — and a page that stated one in its own words would be the second
  specification anti-pattern 11 forbids (`_design.md:867-868`).
- **Advances DoD scenario**: initiative **DoD 9** — *"the evaluator's second question is
  walked … followed to a page that answers them, without dead ends"*
  (`initiative.md:445-448`): this story supplies one such destination and makes it a
  non-dead-end by naming what is behind the link before the reader takes it. It also moves
  **DoD 8** (`:441-444`) by adding one page carrying exactly one named answered-need, and
  it is a precondition for **DoD 7** (`:437-440`), whose front-door walk is HS-P0023's.

## PR boundary

```
examples/course-subscriptions/src/overview.md
examples/course-subscriptions/src/main.rs
examples/course-subscriptions/tests/**
docs/read-the-worked-example.md
docs/carry-your-invariant.md
xtask/src/narrative.rs
.bklg/docs-that-teach/application-author-path/surface-course-subscriptions/**
```

**In this PR**

- The extraction: `overview.md` created from the module doc's text, `main.rs`'s `//!`
  block replaced by `#![doc = include_str!("overview.md")]`, no assertion changed.
- The page: `docs/read-the-worked-example.md` — H1, the single `orientation` answered-need
  declaration, two sentences of orientation, the one-sentence DT-1 anchor citation, the
  one-sentence vocabulary seam, the link to `overview.md`, and the link to the example's
  source last (`_design.md:494-504`).
- The mount: one `#[cfg(doctest)] mod` in `xtask/src/narrative.rs`, one `include_str!` in
  it.
- The reach: one outbound link on the slice-mate's bridge page, and one test in the
  example crate that fails when the chain breaks.

**Explicitly not in this PR**

- Any row in `docs/README.md` — the signpost table and every pointer policy are HS-P0023's
  (`_design.md:749-751`). This story makes material reachable *from the bridge*; it does
  not decide where the front-door pointer lives.
- Any edit to the example's imports, its `Cargo.toml`, its handlers or its assertions
  (`_decomposition.md:382-392`).
- Any edit to `_design.md`, to another project's checker (`xtask/src/lint_narrative.rs`),
  or to HS-P0020's `TREE`/`HARNESS`/`IGNORE_ALLOWANCES`/`HIDDEN_MARKERS` constants.
- Any fence, program, output block, mapping table or diagram. This page has none:
  anti-pattern 14 forbids a diagram, and the teaching on this surface is someone else's
  prose (`_design.md:875`).
- Repairing the `:1-19` citations that this move makes stale across the planning corpus —
  the closeout's reference reconciliation owns them (`_design.md:736-741`).

**Merge DoD one-liner** — `overview.md` is the only copy of the explanation in the tree
and the example renders it via `include_str!`; `docs/read-the-worked-example.md` exists,
is registered in `xtask/src/narrative.rs`, carries exactly one `orientation` declaration,
cites the DT-1 anchor and names the seam before its link; a reader on the bridge page
reaches it in one hop; and the reach check fails if any link in that chain is removed,
with `cargo xtask affected --base main` green at the checkpoint.

The implementer MAY also touch the composition-root / wiring files named in the
Integration contract — `xtask/src/narrative.rs` and the slice-mate's
`docs/carry-your-invariant.md` — to mount this slice. That is not scope drift; it is the
difference between a page that exists and a page that is reachable and checked.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The extraction moves bytes and changes no assertion** | The module doc's text (post-merge `main.rs:1-28`, the `//!` markers stripped) becomes `examples/course-subscriptions/src/overview.md`; `main.rs` opens with `#![doc = include_str!("overview.md")]` above `#![allow(clippy::print_stdout)]`. The governance test is the bar: the edit must not change what the document asserts. Nothing is reworded to "read better on a page". | `_design.md:728-733`; `.kb/governance/rewrite-the-referent-never-the-reasoning.md`; `crates/happenstance/src/lib.rs:1-10`; `examples/course-subscriptions/src/main.rs:1-19` (pre-merge span) |
| **Verbatim is a property of the build, not of a reviewer** | `include_str!` renders identical bytes; there is exactly **one** copy of the explanation in the tree after this PR. No page in this project restates a sentence of it — the anti-paraphrase rule is enforced by there being nothing to paraphrase *from* on the page itself. | `_decomposition.md:482` (tier 1: the identical-bytes guarantee, not a diff check), `:531`; `_design.md:728-733` |
| **One module per included file** | Every `#[cfg(doctest)] mod` this story adds carries exactly one `include_str!`. Concatenated includes report a failure at a line counted from the first file, "which maps to no file a reader can open". | `xtask/src/constitution.rs:11-18`, `:39-50`; `_design.md:743-747`; `_decomposition.md:140-146` |
| **The page lands at the design's route, inside the tree's budgets** | `docs/read-the-worked-example.md` — the design's `{tree}/read-the-worked-example/` under D1's `TREE = "docs"`. 31 characters all-in against a 32-character filename budget the checker enforces; H1 ≤ 40 characters; ≤ 250 source lines; prose wrapped at ≤ 90 columns. | `_design.md:61-64`; `checked-documentation-surface/_design.md:165-176`, `:92-106`, `:388-397`, `:425-426`, `:596` |
| **Exactly one answered-need, in HS-P0021's notation, above everything** | A single blockquote directly under the H1: ``> **Answers:** `orientation` — …?``, token from the closed `NEEDS` set, question in the reader's voice ending in `?`. `orientation` because success here is the reader leaving correctly within one screen — and because `RP-10-2` caps such a page at links plus at most one sentence per destination, which is the shape this page has. | `page-need-discipline/_design.md:104-118`, `:174-176`, `:225-229`; `_decomposition.md:348-349` (UX-008); `_design.md:520` |
| **Orientation before departure** | Two sentences, above the link: what the reader is about to read and why it is worth the hop — three invariants that do not fit an aggregate, in the voice of the project that wrote them. Never more than two: the page's bulk is the destination, not this page. | `_design.md:494-504`, `:667-674`; `_decomposition.md:79-84` (UI-5), `:353-357` (UX-010) |
| **The DT-1 anchor is cited, never re-argued** | One link to the bridge's `## Where your streams went`, placed before the outbound link. It is here because the surfaced text names the prior model in another voice — `main.rs:3-12` says "aggregate" three times — and a reader who meets that without the anchor meets it unanswered. This page adds no second account of the prior model. | `_design.md:108-114`, `:852-854`; `examples/course-subscriptions/src/main.rs:3-12`; `_decomposition.md:350-352` (UX-009) |
| **The vocabulary seam is named once, at the link, and is re-aimed post-merge** | One sentence, immediately before the link. The merged example imports `happenstance` (`:33-37`), so the design's `happenstance_core` premise is void and naming that crate would be false; the seam this story names instead is the layer: the example writes no `Query` and no `AppendCondition` because `happenstance::commit` derives them from a `DecisionModel`. It says that as a claim about what the reader will look for, and does not restate the example's own `# What is not in this file, and used to be` section. | `merge-forward-preflight/spec.md:114-119`; `_design.md:497-502`, `:871-874`; `.kb/decisions/0006-bare-name-to-the-typed-layer.md`; `_decomposition.md:358-361` (UX-011), `:382-392` |
| **The reader leaves to one file, then to the source** | The link to `overview.md` comes first and the link to `main.rs` last, per the composition's order. Link text is meaningful standing alone — never "here" or "this" (accessibility floor). | `_design.md:494-504`; `_decomposition.md:198-203` |
| **Reach is mounted and load-bearing, not narrated** | The bridge page carries one link to this page; this page carries the two outbound links. A `#[test]` in `examples/course-subscriptions/tests/` reads the four files as text and fails if: `main.rs` no longer carries the `include_str!` line, `docs/read-the-worked-example.md` no longer links to `overview.md`, the bridge page no longer links to this page, or the page has acquired a pasted copy of the explanation's opening sentence. Reading a sibling artifact as text is the repository's own precedent for this class of check. | `_decomposition.md:552-556` (the `"tests"` `REQUIRED` step sweeps `--workspace`), `:568-574`; `checked-documentation-surface/_design.md:498-499`; `merge-forward-preflight/spec.md:124-129` |
| **The named wrong implementation this story rejects** | A summary of the worked example in someone else's words — and its two quieter cousins: a page that links to the example but never says what is behind the link (the reader does not take the hop), and a page that pastes the explanation inline (two copies, drift, and the reviewer-diff AC-010 exists to avoid). | `discover.md:53-55`; `project.md:343`; `_design.md:728-733` |
| **No affordance, no fence, no diagram, no second notation** | This page introduces zero interactive affordances, zero Rust fences, zero images, and no `<details>`/tab/admonition marker — the last being rejected by HS-P0020's `HIDDEN_MARKERS` at the gate, not by taste. | `_design.md:858-860`, `:875`; `_decomposition.md:362-365` (UX-012); `checked-documentation-surface/_design.md:102-106` |
| **The gate this story is held to** | `cargo xtask affected --base main` at the story checkpoint; `cargo xtask ci --fast` as the project bar (`terminal: false`). Within them: the `"tests"` step (the reach check), the narrative compile + checker steps (the page's registration), and `cargo doc … RUSTDOCFLAGS=-D warnings` (the example's rendered doc after the extraction — a broken intra-doc link introduced by the move fails here, RS-70-2). | `.redkiln/config.yaml:40,55`; `_decomposition.md:545-565`; `standards/rust/70-rustdoc-obligations.md` |

**Interfaces, explicitly.** This story defines and changes **no Rust interface**. Its
interfaces are four artifacts and one guarantee: `overview.md` (bytes), the `include_str!`
line in `main.rs` (the guarantee), the page in `docs/`, the harness module in
`xtask/src/narrative.rs`, and the link chain that makes the page reachable. The one type
name that appears anywhere in this story's prose — `happenstance::commit` in the seam
sentence — is named, not called, and no fence on this page imports anything.

## Data and migrations

**N/A for schema, store or event data.** Nothing here reads or writes an event store, a
projection store or a checkpoint; `MemoryEventStore` is not constructed; no `Cargo.toml`
gains a dependency, a feature or a target (`examples/course-subscriptions/Cargo.toml`
already declares `[dev-dependencies] trybuild` and a `tests/` target post-merge, and the
reach check needs neither).

**One prose migration, and it is a move rather than a rewrite.** The example's module
documentation leaves `main.rs`'s `//!` block for `src/overview.md`, and is included back
at compile time. Two consequences are recorded rather than discovered:

- **Line citations to `examples/course-subscriptions/src/main.rs:1-19` (and post-merge
  `:1-28`) go stale across the planning corpus** — `project.md`, `_grounding.md`,
  `_storymap.md` and `_design.md` all carry them. `cargo xtask spec-trace` does not read
  the example, so no gate breaks; the design costs this explicitly and routes the repair to
  the closeout's reference reconciliation, which is HS-P0025's
  (`_design.md:736-741`). This story does not edit stage artifacts belonging to signed-off
  stages to chase them.
- **The rendered position of the doc does not change.** Rustdoc renders an included file
  exactly where the `//!` block rendered, so `cargo doc -p course-subscriptions` shows the
  same page before and after — which is the check that the migration was a move. If it
  renders differently, the extraction changed something and the governance test has failed.

**No migration is reversible-by-accident.** Reverting the extraction means deleting
`overview.md` and restoring the `//!` block; the reach check fails first, by name, because
the `include_str!` line is one of the four things it reads.

## Acceptance criteria

Every criterion is framed from Persona 1's intent — the application author who arrives
with an event-sourcing vocabulary and is afraid of "a mental model that looks right,
compiles, runs, and is quietly wrong"
(`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:99-106`) — and
whose journey today dead-ends at step 4: *"goes looking for a fuller worked example; the
one that exists is unshippable, unlinked"* (`:117-121`). Backbone activity A5, *"I found
the good example. Let me actually read it."* (`_storymap.md:45`).

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** the application author has finished the bridge and opens the worked example's rendered documentation, **WHEN** they read its opening explanation, **THEN** they meet the example author's own bytes and not a summary: `examples/course-subscriptions/src/overview.md` exists and is the **only** copy of that explanation in the tree, `examples/course-subscriptions/src/main.rs` opens with `#![doc = include_str!("overview.md")]` and carries no `//!` module-doc block, `cargo doc -p course-subscriptions` renders the doc in the same position and with the same content as before the move, and no assertion, import, handler or `Cargo.toml` line of the example changed. | `examples/course-subscriptions/tests/reach.rs::overview_is_the_only_copy` — reads `main.rs` as text and fails if the `include_str!("overview.md")` line is absent or a `//!` line has returned; plus the `"documentation"` REQUIRED step (`cargo doc … RUSTDOCFLAGS=-D warnings`, `xtask/src/main.rs:290-301`), which fails on any intra-doc link the move broke (RS-70-2), and the `"tests"` step's rebuild of the example. |
| AC-002 | **GIVEN** a reader following the pinned narrative tree, **WHEN** they open `docs/read-the-worked-example.md`, **THEN** they are on a *checked* page and not an orphan file: the page exists at that route, `xtask/src/narrative.rs` carries exactly one `#[cfg(doctest)] mod read_the_worked_example { #![doc = include_str!("../../docs/read-the-worked-example.md")] }` — one module, one included file — and HS-P0020's checker reports zero `unregistered` pages for the tree. | `cargo test -p xtask --doc` collects the doctest named `xtask::narrative::read_the_worked_example`; the narrative checker's REQUIRED step reports no `unregistered` state for `docs/read-the-worked-example.md` (`checked-documentation-surface/_design.md:491-499`, `:560-562`); `examples/course-subscriptions/tests/reach.rs::page_is_registered` reads `xtask/src/narrative.rs` and fails if the page's `include_str!` line is missing or shares a module with a second one. |
| AC-003 | **GIVEN** the author lands on this page cold from a search result rather than from the bridge, **WHEN** the first screen renders, **THEN** they can tell in one glance what question this page answers and that it will send them elsewhere: a single blockquote sits directly under the H1 and above every other element, in HS-P0021's notation and no other — ``> **Answers:** `orientation` — <question>?`` — carrying a token from the closed `NEEDS` set, a question in the reader's own voice ending in `?`, and there is exactly **one** such declaration on the page. | `examples/course-subscriptions/tests/reach.rs::exactly_one_answered_need` — asserts one `> **Answers:**` line, that it is the first non-blank line after the H1, and that its token is `orientation`; HS-P0021's own declaration check once mounted; tier 5 reviewer walk under `answered-need-and-anchor-review` (project DoD item 7). |
| AC-004 | **GIVEN** the author is about to spend their attention on someone else's file, **WHEN** they read this page top to bottom, **THEN** they know before they leave what they are about to read and why — two sentences of orientation naming three invariants that do not fit an aggregate — and they meet the DT-1 anchor **before** the outbound link, as one link to the bridge's `## Where your streams went` and not as a second account of the prior model, because a reader who follows the link does not come back. This is the `Handed off` state (`_decomposition.md:106-108`). | `examples/course-subscriptions/tests/reach.rs::orientation_precedes_departure` — asserts the DT-1 anchor link and both orientation sentences appear at a byte offset **before** the first `overview.md` link, that the orientation is at most two sentences, and that the page contains no occurrence of the word `aggregate` outside the anchor sentence; tier 5 reviewer walk against `_design.md:494-504` under `answered-need-and-anchor-review` (AC-009). |
| AC-005 | **GIVEN** the author has just been taught to write a `Query` and an `AppendCondition` by hand on the bridge, **WHEN** they follow the link and find neither in the example, **THEN** they were warned: exactly one sentence, immediately before the link and nowhere else, names the seam as it exists **post-merge** — the example is the same boundary one layer up, where `happenstance::commit` derives both from a `DecisionModel` — states it as a claim about what the reader will look for, names no crate the merged example does not import, and restates no sentence of the example's own `# What is not in this file, and used to be` section. | `examples/course-subscriptions/tests/reach.rs::seam_named_once_before_the_link` — asserts exactly one seam sentence, positioned after the DT-1 anchor and before the first outbound link, that the page contains no occurrence of `happenstance_core`, and that no sentence of the page appears verbatim in `overview.md`; tier 5 reviewer walk against `merge-forward-preflight/spec.md:114-119` and the recorded `_baseline.md`. |
| AC-006 | **GIVEN** the author is mid-bridge, **WHEN** they act on A5, **THEN** the good example is one hop away and stays that way: `docs/carry-your-invariant.md` carries one link to this page, this page links to `overview.md` first and to `main.rs` last, every link's text is meaningful standing alone (never "here"/"this"), and a repository check **fails by name** if any link in that chain is removed, if a link's target file does not exist, or if a pasted copy of the explanation's opening sentence appears on the page. | `examples/course-subscriptions/tests/reach.rs::the_chain_holds` — reads the four files as text, resolves each link target on disk, and fails on a broken link in either direction, on link text matching `here`/`this`/`link`, and on the explanation's opening sentence appearing in `docs/read-the-worked-example.md`; swept by the `"tests"` REQUIRED step (`xtask/src/main.rs:143-155`, `--workspace --all-features` covers `publish = false` crates). |
| AC-007 | **GIVEN** the author reads on a 1024×768 laptop, on a phone, in print, or with JavaScript off, **WHEN** the page renders, **THEN** nothing is hidden from them and nothing overflows: the page introduces **zero** interactive affordances, zero Rust fences, zero images or diagrams, and no `<details>`/tab/admonition marker; it holds the substrate's budgets — repo-relative path ≤ 32 characters (`docs/read-the-worked-example.md` is 31), H1 ≤ 40 characters, page ≤ 250 source lines, prose source wrapped ≤ 90 columns, every `##` heading ≤ 22 characters so the sidebar TOC does not clip it with an ellipsis; and its hierarchy is the design's — the destination is primary, the orientation secondary above it, the seam sentence recessive as one sentence and never promoted to a `###`. | `examples/course-subscriptions/tests/reach.rs::page_holds_its_budgets` — asserts the path, H1, line-count, wrap-width and heading-length numbers and that the page contains no fence delimiter, no `<details`, no `![`, and no admonition marker; HS-P0020's `HIDDEN_MARKERS` rejection at the gate (`checked-documentation-surface/_design.md:102-106`); tier 5 reviewer walk against `_design.md:667-674` and `## Anti-patterns` items 1, 3, 8, 12, 13, 14. |

**Coverage of the traced project AC.** Project **AC-010** — *"course-subscriptions
surfaced verbatim, reachable"* (`project.md:263-265`) — is covered in both halves, by the
tier the testing brief assigns it (`_decomposition.md:531`, tier 1): *surfaced verbatim*
by **AC-001** (the `include_str!` identical-bytes guarantee) and **AC-005** / **AC-007**
(nothing on the page paraphrases it, because nothing on the page restates it);
*reachable* by **AC-002** (registered, therefore a page) and **AC-006** (the chain, and a
check that fails when it breaks). **AC-003** and **AC-004** carry the AC-012 and AC-009
obligations this page owes as a member of the set, whose set-wide proof is
`answered-need-and-anchor-review`'s.

## Interaction quality

RFC §6.7/D6. This story renders a surface, so `_design.md`'s signed-off composition is
**binding** (`_design.md:1039`) and its invariants are gated as AC **rows** above, not as
prose bullets here. This section says which row carries which invariant and how each is
verified. The medium is rustdoc-rendered markdown with **no CSS and no JavaScript of this
project's own** (UX-012, `_decomposition.md:362-365`), so several classical invariants are
satisfied structurally — and where they are, that is recorded rather than assumed.

**State invariants.**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Context-jump, declared** | **AC-004**, **AC-005** | This surface's whole purpose *is* a context jump — the reader leaves for another file and, per the design, does not come back. The invariant is therefore not "stay in place" but **"never jump uninformed"**: orientation, then the DT-1 anchor, then the seam, then the link. Byte-offset assertions in `reach.rs::orientation_precedes_departure` and `::seam_named_once_before_the_link`. |
| **Non-occlusion** | **AC-003**, **AC-007** | Nothing this page authors overlays anything. The answered-need line is above everything in reading order — never a badge, never a tooltip; and rustdoc's own `rustdoc-topbar` sets `scroll-margin-top: 45px`, so anchors still land under it at ≤700px (`_design.md:690-700`). Verified by AC-003's first-element assertion and AC-007's zero-affordance assertion. |
| **Preserved focus / scroll / selection** | **AC-007** | Vacuous **by construction, and that is the point**: there is no script, no toggle and no client-side state on this page, so nothing can discard a scroll position or a selection. AC-007's zero-affordance assertion is what keeps it vacuous — the moment a `<details>` or a tab strip appears, the invariant stops being free. |
| **Reversibility** | **AC-006**, **AC-001** | The jump is reversible by the medium (browser back) because it is an ordinary link, not a script-driven navigation — which AC-007 guarantees. The *repository-level* reversibility is AC-001's: reverting the extraction means deleting `overview.md` and restoring the `//!` block, and `reach.rs::overview_is_the_only_copy` fails first, by name, rather than leaving a half-reverted tree. |
| **Keyboard reachability** | **AC-006**, **AC-007** | Every control on this page is an anchor element rustdoc renders from markdown, so it is in the tab order without this project doing anything — provided this project introduces no custom control (AC-007) and every link's text is meaningful standing alone for a screen-reader link list (AC-006, UX-013, `_decomposition.md:198-203`). |

**Composition invariants**, taken from `_design.md`'s `worked-example-handoff` surface.

| Invariant | Design source | Carried by | How it is verified |
| --- | --- | --- | --- |
| **Presentation exists at all** | `_design.md:494-504` | **AC-003**, **AC-004**, **AC-005** | The failure this guards is a page that is a bare link — correct in every structural assertion and useless to a reader. The three composed elements (declaration, orientation, seam) are each their own AC row with their own position assertion, which is why an unstyled render cannot satisfy them: a page carrying only the link passes AC-002 and AC-006 and fails AC-003, AC-004 and AC-005. |
| **Composition and placement** | `_design.md:494-504` | **AC-003**, **AC-004**, **AC-005**, **AC-006** | The five slots in order: answered-need line, two orientation sentences, the seam sentence immediately before the link, the link to `overview.md`, the link to the source **last**. Ordering is asserted by byte offset in `reach.rs`, not by a reviewer's reading. |
| **Transience** | `_design.md:518-520` | **AC-003**, **AC-007** | Everything this page authors is **persistent, in-body**. Nothing is revealed, nothing is opened-on-demand; the answered-need line in particular is "never inside a fold, never a tooltip, never a badge in a corner" (`_design.md:520`). Rustdoc's own chrome is the medium's and is untouched. AC-007's no-`<details>`, no-marker assertion is the mechanical half. |
| **Density budget, with its numbers** | `checked-documentation-surface/_design.md:388-397`, `:425-426`; `_design.md:532-600` | **AC-007** | Repo-relative path ≤ 32 characters (31 used); H1 ≤ 40; page ≤ 250 source lines; prose source ≤ 90 columns; `##` headings ≤ 22 characters, because at 200px with 24px padding the sidebar TOC clips longer entries with an ellipsis (`white-space:nowrap`) rather than wrapping them; paragraphs ≤ 435 characters, which is five rendered lines at 764px. The fence budgets (68 columns, 24 lines) do not bind: this page has no fence, which is itself AC-007. |
| **Hierarchy** | `_design.md:667-674` | **AC-004**, **AC-005**, **AC-007** | Primary is the surfaced module doc — carried by length and by being someone else's voice; secondary is the orientation, carried by position above it; **recessive is the seam sentence**, a caveat at the moment of leaving and not a topic. Promoting it to a `###` would turn a friction into a subject and is a defect even though it renders. AC-005 asserts one sentence; AC-007 asserts no heading was added for it. |
| **Named anti-patterns** | `_design.md:831-882` | **AC-005**, **AC-007** | Item 3 (collapsed disclosure), 8 (any control rustdoc does not render), 12 (two answered-needs, or one below the first block), 13 (`happenstance_core` outside the seam sentence — and post-merge, *including* it), 14 (a diagram or image). Item 2 (a literal bracketed code word where an intra-doc link failed) is caught by AC-001's `cargo doc … -D warnings` step on the example's side and by AC-007's reviewer walk on the page's. |

**The falsification that makes this section non-decorative.** An unstyled, unreviewed
render satisfies AC-001, AC-002 and AC-006 completely — the bytes are identical, the
module compiles, the links resolve — and is still the exact page the initiative exists to
prevent: a bare link with nothing telling the reader what is behind it. AC-003, AC-004,
AC-005 and AC-007 are what fail on it.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | `xtask/src/narrative.rs` does not exist when this story is implemented (HS-P0020's substrate has not landed). | **Halt and report.** Do not invent a second harness, do not register the page in `xtask/src/constitution.rs`, do not pin a tree constant of your own. This is the routed dependency, and stopping is correct behaviour (`_decomposition.md:118-131`; `_design.md:886-895`). |
| **EC-002** | `docs/carry-your-invariant.md` does not yet carry a `## Where your streams went` anchor (the slice-mate is incomplete). | **Halt and report.** The DT-1 anchor is named in exactly one place in the whole page set (`_design.md:108-114`, `:852-854`); authoring a second account here to unblock is the AC-009 defect. This story blocks on `invariant-to-appendcondition-bridge` for exactly this reason. |
| **EC-003** | The page exists under `docs/` but is not registered in the harness. | The checker's `unregistered` state fires and the gate fails (`checked-documentation-surface/_design.md:560-562`). "Constructed but not mounted" is not a state this substrate permits; AC-002 is unsatisfiable until the registration lands. |
| **EC-004** | `cargo doc -p course-subscriptions` renders the module doc differently after the extraction — different position, different content, a new warning. | The move was not a move. Revert and redo it as a pure byte transfer; the governance test has failed (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`). Prose reworded to "read better on a page" is the specific failure to look for. |
| **EC-005** | The extraction breaks an intra-doc link, or a relative link resolves differently from `overview.md` than it did from `main.rs`. | The `"documentation"` REQUIRED step fails under `RUSTDOCFLAGS=-D warnings` (`xtask/src/main.rs:290-301`; RS-70-2, `standards/rust/70-rustdoc-obligations.md`). Fix the link; do not relax the flag and do not delete the link to make the step pass. |
| **EC-006** | The merged example turns out **not** to import `happenstance` (the recorded baseline has drifted). | The seam sentence's premise is void again. **Stop and re-read** `merge-forward-preflight`'s recorded `_baseline.md` (`merge-forward-preflight/spec.md:292-294`) before writing the sentence — a seam sentence naming the wrong crate is worse than none, because it is a confident falsehood delivered at the moment the reader is leaving. |
| **EC-007** | A `#[cfg(doctest)] mod` in `xtask/src/narrative.rs` accumulates more than one `include_str!`. | Reject at review. Concatenated includes report a failure at a line counted from the *first* file, "which maps to no file a reader can open" (`xtask/src/constitution.rs:11-18`). AC-002 asserts one module per file. |
| **EC-008** | A `reach.rs` assertion passes because a link's *text* is present while its target file does not exist. | Not a permitted outcome. Every link assertion resolves the target on disk (AC-006). A check no wrong implementation can fail is decorative by CLAUDE.md's own test, and every assertion in `reach.rs` must have been seen to fail once by deleting the thing it guards. |

## Non-functional

| id | requirement | basis |
| --- | --- | --- |
| **NF-001** | **Exactly one copy of the explanation exists in the repository** after this PR, and its byte-identity across two renders is structural rather than reviewed. Two copies is the defect, not a tolerance. | `_design.md:707-733`; `_decomposition.md:482` (tier 1 is the identical-bytes guarantee, not a diff check) |
| **NF-002** | **Zero new dependencies, zero features, zero targets, zero public API items, no MSRV movement.** `examples/course-subscriptions/Cargo.toml` is not edited; the reach check uses only `std`. | `_design.md:266-289`; `examples/course-subscriptions/Cargo.toml:8` (`publish = false`) |
| **NF-003** | **No new REQUIRED gate step and no measurable gate-time cost.** The reach check is four file reads and a handful of substring assertions inside a step that already runs; the page's registration adds one doctest module that is only expanded under `cargo test`. | `_decomposition.md:545-565`; `xtask/src/constitution.rs:20-25` (`cfg(doctest)` means the includes never expand under `check` / `clippy` / `build`) |
| **NF-004** | **Print, no-JS, reduced-motion and ≤700px parity are free and must stay free.** Everything authored is static prose and links; nothing animates, nothing requires script. This is a consequence of introducing no affordance, not a separate design — and it stops being free the first time one is introduced. | `_design.md:690-700` (the `Print / no-JS`, `Reduced motion` and `Narrow viewport` states); UX-012 |
| **NF-005** | **The stale-citation cost is bounded and recorded, not silently absorbed.** The move invalidates `examples/course-subscriptions/src/main.rs:1-19` / `:1-28` citations across the planning corpus; `cargo xtask spec-trace` does not read the example, so no gate breaks, and the repair belongs to the closeout's reference reconciliation. | `_design.md:736-741`; this spec's `## Data and migrations` |
| **NF-006** | **The page is legible cold.** A reader arriving from search with no prior context can, within one screen at 1024×768, name what this page answers and what is behind the link. `orientation`'s own success condition is that the reader "leave, correctly, within one screen". | `page-need-discipline/_design.md:174-176`; `_decomposition.md:79-84` (UI-5) |

## Implementation notes (non-prescriptive)

Observations that save time, not instructions. Where one conflicts with `_design.md`, the
design wins.

- **Do the extraction first, and prove it before writing a word of the page.** The
  sequence that fails fastest: create `overview.md` by moving the `//!` block's text
  (stripping only the `//! ` prefixes), replace the block with
  `#![doc = include_str!("overview.md")]`, run `cargo doc -p course-subscriptions` and
  `cargo test -p course-subscriptions`, and *look at the rendered page*. If it renders
  identically the migration is a move and everything downstream is prose; if it does not,
  nothing downstream is worth writing yet.
- **The `#![doc = …]` attribute must be the first item in `main.rs`,** above
  `#![allow(clippy::print_stdout)]` — inner attributes precede items, the same ordering
  `crates/happenstance/src/lib.rs:10` already has.
- **Write the seam sentence against the merged file, not against `_design.md`'s premise.**
  Open `examples/course-subscriptions/src/main.rs` in the merged tree and confirm what it
  imports and what it does *not* contain before drafting. The design's sentence is void;
  its *slot*, *length* and *position* are not.
- **Two sentences of orientation is a ceiling, not a target.** The page's bulk is someone
  else's file. If the orientation is doing real explanatory work, that work belongs on the
  bridge page, which is the slice-mate's.
- **The reach check reads files as text; it does not parse markdown.** Substring and
  byte-offset assertions plus a `Path::exists` on each target are sufficient, and reading
  a sibling artifact as text is this repository's own precedent for the class
  (`xtask/src/lint_constitution.rs` reads the corpus rather than trusting the compiler).
  Do not add a markdown-parser dependency for this.
- **Name each assertion after the reader-visible failure it rejects,** so a CI log line
  reads `seam_named_once_before_the_link` rather than `test_page_3`, and let the failure
  message name the file and what was expected — the checker's own location-prefix
  discipline applied to a test (`checked-documentation-surface/_design.md:425-426`).
- **The H1 and the answered-need question are the same decision made twice.** Draft the
  question first, in the reader's own words, then compress it to ≤ 40 characters for the
  H1. The other order produces an H1 that fits and a question that restates it.
- **When unsure whether a sentence belongs on this page, ask whether it would survive
  being deleted after the reader has read `overview.md`.** If yes, it is restating the
  destination, and AC-005's no-verbatim-overlap assertion will find it.

## Tests and CI (merge gate)

Grounded in the project testing brief's five tiers (`_decomposition.md:480-486`) and its
merge-gate command list (`:545-565`). This story adds **no new tier and no new REQUIRED
step**; it rides existing ones.

| tier | command / path | proves |
| --- | --- | --- |
| **1 — Structural (proof by construction)** | `cargo test --locked --workspace --all-features` collecting `xtask::narrative::read_the_worked_example`; `cargo doc --locked --workspace --all-features --no-deps --document-private-items` with `RUSTDOCFLAGS=-D warnings` (`xtask/src/main.rs:290-301`) | AC-001, AC-002. Verbatim is the `include_str!` identical-bytes guarantee, not a diff check (`_decomposition.md:482`, `:531`). The doc step is what catches an intra-doc link the extraction broke (RS-70-2). |
| **1 — Structural (registration)** | HS-P0020's narrative checker REQUIRED step over `TREE = "docs"` | AC-002. A page under the tree that the harness does not include is the checker's `unregistered` problem, not a page (`checked-documentation-surface/_design.md:560-562`). |
| **3 — Executed (`#[test]`, swept by `"tests"`)** | `examples/course-subscriptions/tests/reach.rs` — `overview_is_the_only_copy`, `page_is_registered`, `exactly_one_answered_need`, `orientation_precedes_departure`, `seam_named_once_before_the_link`, `the_chain_holds`, `page_holds_its_budgets`; run by `cargo test --locked --workspace --all-features -- --show-output` (`xtask/src/main.rs:143-155`) | The mechanical half of AC-001 … AC-007. `--workspace --all-features` sweeps `publish = false` crates, which is why the check lands in the example crate rather than needing a new target (`_decomposition.md:568-574`). |
| **2 — Compiled-fence** | HS-P0020's compiled-fence REQUIRED step | **Nothing here, and that is stated rather than omitted.** This page authors zero fences (AC-007), so the step passes vacuously over it. The day it has something to say about this page, a fence has appeared and AC-007 has been violated. |
| **5 — Review sign-off** | The reviewer walk under `answered-need-and-anchor-review` (project DoD item 7) and `fence-inventory-and-clause-audit` | The prose halves no assertion can reach: that the orientation actually orients, that the seam sentence is true of the merged example, that the DT-1 anchor is applied identically across the set (AC-009), and that this page's fence count of zero is on the inventory (AC-007 of the project). |
| **4 — Falsification** | Not applicable here, stated rather than omitted | The project's boundary drill is `boundary-falsification-drill`'s. This story's analogue is EC-008's rule: every `reach.rs` assertion is seen to fail once — delete a link, watch the named test fail, restore it, watch it pass. |
| **Story checkpoint** | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | The story grain: only what this diff could break. Green at the checkpoint commit. |
| **Project bar** | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | The bar a non-terminal project meets. This project is `terminal: false`; the whole-initiative `cargo xtask ci` re-observation is HS-P0025's, at closeout. |

**Named wrong implementations these checks reject**, per CLAUDE.md's discipline: a page
that summarises the example in someone else's words (AC-001 + AC-005); a page that links
to it without saying what is behind the link (AC-004); a page that pastes the explanation
inline, creating a second copy that drifts (AC-001's single-copy assertion plus AC-005's
no-verbatim-overlap assertion); a page that exists under `docs/` but is never compiled
(AC-002); and a bridge page whose link to it is quietly deleted in a later refactor
(AC-006).

## Risks and coupling (PR-scoped)

| risk | why it bites | mitigation, in this PR |
| --- | --- | --- |
| **The harness does not exist yet.** `xtask/src/narrative.rs` is HS-P0020's and is not in the tree today. | Without it the page is an orphan file and AC-002 is unsatisfiable; the tempting workaround — registering the page in `xtask/src/constitution.rs` alongside the Rust constitution — would put a narrative page in the corpus `lint-constitution` walks and break a different gate. | EC-001: halt and report. The dependency is named in the Integration contract, not discovered at implementation time. |
| **The slice-mate's anchor.** This page cites `## Where your streams went` on a page written in the same context by the same slice. | If that heading is retitled to fit the 22-character budget after this page cites it, the citation dangles silently — markdown link checking is not in the gate today. | Implement the slice-mate's heading and this citation in one context (the slice *is* one context, `_storymap.md:75-78`), and let `reach.rs::the_chain_holds` assert the anchor's own text exists in `docs/carry-your-invariant.md`, not merely that a link was written. |
| **The seam sentence is the one place this story can author a confident falsehood.** It is prose, unchecked by any compiler, read at the moment the reader is leaving. | `_design.md`'s own version is already false post-merge — which is proof the failure mode is real rather than hypothetical. | AC-005 asserts the page contains no `happenstance_core`; the tier 5 walk asserts the sentence is true of the merged file; EC-006 halts if the baseline drifted again. |
| **Paraphrase creep.** The orientation sits directly above someone else's explanation, and the natural writing instinct is to preview it. | AC-010's whole bar is "surfaced rather than paraphrased"; a two-sentence preview that restates the destination is paraphrase with a link attached. | AC-005's assertion that no sentence of the page appears verbatim in `overview.md`, the two-sentence ceiling in AC-004, and the deletion test in the implementation notes. |
| **Stale `:1-19` citations across the planning corpus.** `project.md`, `_grounding.md`, `_storymap.md` and `_design.md` all cite the module doc by line. | No gate breaks (`spec-trace` does not read the example), so this rots quietly. | Costed and routed at `_design.md:736-741` to the closeout's reference reconciliation. This story does not edit signed-off stage artifacts to chase it, and the PR boundary says so. |
| **The extraction may become unnecessary.** If HS-P0020's mechanism can include a source file's module doc directly, `overview.md` is a workaround with a standing maintenance cost. | Carrying a dead workaround forever is worse than the hop it saved. | Recorded as a substrate gap to HS-P0020 under project DoD item 9 (`project.md:300-302`), with `_design.md:736-741` already pre-authorising the drop. |
| **Coupling to `docs/README.md`.** The front-door signpost table is the repository's only routing primitive, and this page is exactly the sort of thing someone will want to add to it. | A row added here pre-empts HS-P0023's pointer policy and creates a second, unreviewed front door. | Named in the PR boundary as explicitly out of scope (`_design.md:749-751`). Reach gaps route to HS-P0023. |

## Dependencies

**Blocks on** — `invariant-to-appendcondition-bridge` (HS-S0187). Two hard edges, not one:
it authors `docs/carry-your-invariant.md`, which carries the single outbound link that
makes this page reachable (AC-006), and it authors the `## Where your streams went`
section that is the DT-1 anchor this page cites rather than re-argues (AC-004,
`_design.md:108-114`, `:852-854`). Both stories are in the `conceptual-bridge` slice and
are implemented in one context and mounted as one surface (`_storymap.md:59-60`,
`:75-78`).

**Transitively blocks on** — `merge-forward-preflight` (the recorded baseline AC-005's
seam sentence is written against, `merge-forward-preflight/spec.md:114-119`, `:292-294`)
and `tension-resolutions` (the signed-off `_design.md` this story implements). Both are
already upstream of the slice-mate, so this story does not restate them as direct edges.

**Consumes, does not build** — HS-P0020's pinned tree and narrative harness
(`checked-documentation-surface/_design.md:92-106`, `:491-499`) and HS-P0021's
answered-need notation (`page-need-discipline/_design.md:104-118`). Neither is a backlog
edge inside this project; both are named halts (EC-001, EC-003) rather than assumptions.

**Unlocks** — `fence-inventory-and-clause-audit` (this page contributes a fence count of
zero and a clause count of zero to the set-wide inventory) and
`answered-need-and-anchor-review` (this page contributes one `orientation` declaration and
one DT-1 anchor citation to the set-wide walk). Both name this story in their
`depends_on` (`_storymap.md:63-65`).

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. Open each at the moment named, not
before — the Context pack above is the must-read core and is sufficient to start.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/application-author-path/_design.md` | The signed-off, **binding** composition: the five slots of `worked-example-handoff` (`:494-504`), the hierarchy that makes the seam sentence recessive (`:667-674`), the transience table (`:518-520`), the density numbers (`:532-600`), the fifteen anti-patterns (`:831-882`), and the extraction's own justification (`:707-751`). | Before writing a single line of the page — and again before review, to walk the anti-pattern list item by item. | AC-003, AC-004, AC-005, AC-007 |
| `.bklg/docs-that-teach/checked-documentation-surface/_design.md` | The substrate this story consumes: D1 "the markdown is the render" and why there is no include mechanism (`:165-176`), `TREE` / `HARNESS` (`:92-106`), harness placement (`:491-499`), the page and path budgets (`:388-397`, `:425-426`), and the `unregistered` checker state (`:560-562`). | First of all, to confirm the harness exists at all (EC-001); then before implementing AC-002. | AC-002, AC-007 |
| `.bklg/docs-that-teach/page-need-discipline/_design.md` | The exact declaration notation and the closed `NEEDS` set (`:104-118`, `:174-176`), and `RP-10-2`'s ceiling for an `orientation` page (`:225-229`) — which is why this page is links plus at most one sentence per destination. Inventing a second notation is a defect even if it renders identically. | Before implementing AC-003, at the moment of writing the blockquote. | AC-003 |
| `xtask/src/constitution.rs` | The exact mount shape this story copies and the reason it is one module per included file, in the repository's own words (`:11-18`, `:39-50`) — plus `:20-25`, which explains why `cfg(doctest)` makes a deleted page invisible to every step except `cargo test`. | Before implementing AC-002, when writing the module in `xtask/src/narrative.rs`. | AC-002 |
| `crates/happenstance/src/lib.rs` | The `#![doc = include_str!(…)]` precedent already wired in this workspace (`:10`), with `:1-9` recording why the included file must live inside the package and why a path escaping it "would not resolve once published". | Before implementing AC-001, when placing the attribute in `main.rs`. | AC-001 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The bar the extraction is held to: does the edit change what the document *asserts*? This is the whole difference between a move and a rewrite, and the only thing standing between "surfaced" and a quiet paraphrase. | Immediately after the extraction, before running `cargo doc` — it is the question to hold while reading the diff. | AC-001 |
| `.kb/decisions/0006-bare-name-to-the-typed-layer.md` | The Accepted decision that makes `use happenstance::{…}` the taught vocabulary and the reason the merged example now imports it — which is exactly what voids `_design.md`'s seam premise and re-aims the sentence one layer up. | Before writing the seam sentence (AC-005). | AC-005 |
| `.bklg/docs-that-teach/application-author-path/merge-forward-preflight/spec.md` | The four measured merge contradictions this story is authored against (`:106-129`): the crate change at `:114-119`, the module doc's growth to `:1-28` at `:120-123`, the example's new test target at `:124-129`, and the `_baseline.md` that records them (`:292-294`). | Before AC-001 (which span to extract) and before AC-005 (which seam is true); EC-006's halt reads it. | AC-001, AC-005 |
| `.bklg/docs-that-teach/application-author-path/_decomposition.md` | The UX brief's UI-5 and the `Handed off` / `Vocabulary seam` states (`:79-84`, `:106-108`), UX-010/011/012/013 (`:353-365`), and the testing brief's five tiers, AC-010 row and merge-gate commands (`:480-486`, `:531`, `:545-574`). | Before writing the tests (AC-006), and when choosing which existing tier each assertion rides. | AC-006, AC-007 |
| `.bklg/docs-that-teach/application-author-path/invariant-to-appendcondition-bridge/spec.md` | The slice-mate's spec: what its page is called, where the `## Where your streams went` heading lands, and what vocabulary it teaches the reader immediately before they arrive here. The seam sentence is only correct relative to what that page just taught. | Before AC-004 and AC-005 — the two ACs that are claims about the reader's state on arrival. | AC-004, AC-005 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | Persona 1's stated fear (`:99-106`) and journey step 4 (`:117-121`) — the recorded dead end this story closes. The orientation sentences are written for that reader or they are written for nobody. | Before writing the orientation prose (AC-004), when deciding what "why it is worth the hop" actually means. | AC-004 |
| `standards/rust/70-rustdoc-obligations.md` | RS-70-2 and the `-D warnings` doc obligation — the rule that turns a link the extraction broke into a failed gate step rather than a silently wrong page. | When AC-001's `cargo doc` step fails, and before assuming the failure is unrelated to the move. | AC-001 |
| `xtask/src/main.rs` | The `REQUIRED` array itself: the `"tests"` step (`:143-155`) that sweeps `reach.rs`, and the `"documentation"` step (`:290-301`). This is what "no new gate step" means concretely. | When wiring the tests, to confirm an existing step already covers the path rather than adding one. | AC-002, AC-006 |
| `examples/course-subscriptions/src/main.rs` | The bytes themselves — the module doc being moved, its `# What is not in this file, and used to be` section, and the imports that must not be touched. | First, before anything else in this story. | AC-001, AC-005 |
| `.redkiln/config.yaml` | The two commands redkiln runs at this story's and this project's grains (`:40`, `:55`), and the `support` initiative that incidental bugs route to (`:5`). | At the checkpoint commit, and whenever something found here is not this story's to fix. | AC-001, AC-002, AC-003, AC-004, AC-005, AC-006, AC-007 |

## Clarifications resolved during spec

1. **The AC set is exactly the seven the front half enumerated** — AC-001 through AC-007,
   nothing added and nothing dropped. The partition is two structural (the extraction, the
   registration), three compositional (the declaration, the orientation-plus-anchor, the
   seam), one reach and one budget-and-anti-pattern. Each is one reader-observable failure.
2. **"An include of `overview.md`" (`_design.md:502`) resolves to a link, not a paste.**
   The design was written before HS-P0020 settled D1 — the markdown is the render, no
   build step — and markdown has no include. The front half took the decision and costed
   both rejected alternatives; this half gates it: AC-001 asserts one copy, AC-005 asserts
   no sentence of the page appears verbatim in `overview.md`. The residual is routed to
   HS-P0020, not worked around on the page.
3. **The composition invariants are AC rows, not prose bullets.** `redkiln verify`
   extracts ACs by matching a leading `| AC-001 |` table cell; an invariant written as a
   bullet under `## Interaction quality` would get no ledger row and would never be gated.
   So that section is a *mapping* from invariant to the AC that carries it, and every
   composition invariant that applies lands in AC-003, AC-004, AC-005 or AC-007.
4. **The classical state invariants are re-aimed rather than declared vacuous.** This
   surface's purpose is a context jump, so "in place, no context jump" is the wrong
   invariant and "never jump uninformed" is the right one — AC-004 and AC-005.
   Focus/scroll/selection preservation is genuinely vacuous here, and AC-007's
   zero-affordance assertion is what keeps it vacuous rather than merely true today.
5. **The seam sentence declines `_design.md`'s explicit permission to name
   `happenstance_core`** (anti-pattern 13's single exception). Post-merge that name would
   be false. The slot, the length and the position are kept; the content is re-aimed at
   the layer seam — no `Query` and no `AppendCondition` in the example, because
   `happenstance::commit` derives both from a `DecisionModel`. This is the disposition
   `merge-forward-preflight/spec.md:114-119` routed here, executed.
6. **The reach check lands in `examples/course-subscriptions/tests/`, not in `xtask/`.**
   The merged example carries a test target already
   (`merge-forward-preflight/spec.md:124-129`) and the `"tests"` REQUIRED step sweeps
   `--workspace --all-features`, which covers `publish = false` crates
   (`_decomposition.md:568-574`). Putting it in `xtask` would make it a substrate check,
   which it is not — it is this project's own reach obligation.
7. **No conformance rule and no clause, named rather than left blank.** A conformance rule
   here would be decorative by CLAUDE.md's own test (no adapter could fail it), and a page
   stating a normative rule in its own words is the second specification anti-pattern 11
   forbids (`_design.md:867-868`).
8. **When the substrate is missing this story halts; it does not improvise.** EC-001 and
   EC-002 are written as required behaviour rather than as risks, because each available
   improvisation — a second harness, a second DT-1 account — is a defect that would pass
   every mechanical check in this spec.
