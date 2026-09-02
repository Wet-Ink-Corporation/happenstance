# Grounding — Reach and the Adapter Path (HS-P0023)

Companion to `project.md`, `_intake-brief.md` and `_storymap.md`. Written for the
briefs workflow (architecture, ux — no `testing`, no `deployment` per
`.bklg/docs-that-teach/_decomposition.md`, Warranted briefs). All paths below were
verified against the worktree at
`D:\repos\happenstance\.claude\worktrees\docs-that-teach` on 2026-08-17; every
anchor cited exists unless marked **NOT VERIFIABLE HERE**.

Note: `project.md` (authored at the charter stage, `stage: storymap`) already
carries an unusually complete anchor set — DR-1..DR-10, AC-001..AC-012, a risk
table, and a "Context anchors" section. This note does not repeat that content; it
verifies it, adds what the charter did not need (line-level confirmation, sibling-
project dependency state, an ADR reading), and flags what a briefs author would
otherwise discover the hard way.

## Accepted decision atoms that constrain this project

- **`.kb/decisions/0001-async-port-flavours.md`** (ADR-0001, accepted). Defines the
  two-flavour port scheme `EventStore`/`SendEventStore` via
  `#[trait_variant::make(SendEventStore: Send)]`, with `SendEventStore` implying
  `EventStore` through a blanket impl — the exact asymmetry that produces the
  E0034 ambiguous-method collision this project's AC-006/AC-007 puts in
  `store.rs`. This is the decision the error the project documents is a
  *consequence of*, not the decision under revision. The reasoning account
  (AC-008) cites this ADR for *why* the split exists; it does not restate it
  (DR-8, precedence chain below).
- **`.kb/decisions/0008-one-derivation-for-both-ports.md`** (ADR-0008, accepted).
  Extends ADR-0001's scheme to `ProjectionStore` and states the three rules for a
  provided method (hand-desugar, `Self: Sync` at point of use, one body compiles
  under both flavours) — the same rules `CONTRIBUTING.md:97-140` explains for a
  contributor audience. The reasoning account sequences both without duplicating
  either.
- **`.kb/decisions/0006-bare-name-to-the-typed-layer.md`** (ADR-0006, accepted).
  Governs why `happenstance` (bare name) is the front door this project installs
  the pointer on, and why `happenstance-core` is what adapter authors pin —
  directly relevant to AC-002 and to distinguishing which crate's front door the
  pointer belongs on. `crates/happenstance/src/lib.rs:20-25` already states this
  ADR's conclusion in prose; the pointer this project adds must not contradict it.
- **`.kb/decisions/0007-projection-runner-decodes.md`** (ADR-0007, accepted). Not
  load-bearing for this project's own pages, but cited by `lib.rs`'s existing
  crate-root doc for where the projection runner lives — relevant only if the
  front-door pointer text is edited in a way that touches the surrounding
  paragraph; do not disturb it (DoD-7 of `project.md`: "nothing in
  `crates/happenstance/README.md` changed except the addition of the pointer").
- **`.kb/decisions/0029-msrv-raised-to-1-97-1.md`** (ADR-0029, accepted). Not
  directly cited by this project's ACs, but binds any code fence this project's
  pages compile (HS-P0020's gate step runs at the pinned toolchain); no fence
  this project writes may assume a newer MSRV.

No Accepted decision atom governs documentation structure, narrative reach, or
pointer policy itself — confirmed by the initiative charter's own audit
(`.bklg/docs-that-teach/initiative.md`, "Across the constitution there is no rule
about narrative structure... none of the seventeen decisions concerns
documentation"). DT-10 is genuinely open design space, not a tension with
anything Accepted.

## Existing code patterns and modules this project must follow

- **`crates/happenstance-core/src/store.rs`** — the file AC-006/AC-007 rewrite.
  Verified line-for-line: the "Import one flavour, not both" section is at lines
  31–45; the trimmed ```` ```text ```` E0034 excerpt the project restores is at
  lines 36–41 (four lines: the `error[E0034]` line, a blank gutter, the
  `store.read(...)` line, and the `^^^^ multiple` caret line). It currently has
  **no** `= note:` candidate lines and **no** occurrence of
  `TraitVariantBlanketType` anywhere in the file — confirmed by direct read, not
  inferred — which is exactly the gap AC-006 names. The excerpt sits inside the
  module doc (`//!`), which participates in the crate's rustdoc build but, being
  a `text` fence rather than a Rust one, is not compiled by
  `standards/rust/62-doctests-and-harnesses.md`'s obligations — the risk table in
  `project.md` already states this limit; the architecture brief should carry it
  forward rather than imply the excerpt is checked.
- **`crates/happenstance-core/src/memory.rs:16-71`** (`MemoryEventStore`'s doc
  comment) — the runnable full-DCB-loop walk-through AC-008's reasoning account
  sequences. Verified: lines 16-35 state the three reasons the reference store
  exists (conformance oracle, runnable examples, pre-adapter application
  development) and its scale limits; the compiled example (lines 40-71) is the
  read-decide-append-retry loop end to end, already a doctest. `project.md`'s own
  risk table flags the load-bearing caveat correctly: `MemoryEventStore` is the
  suite's oracle, not an adapter, and the account must say so explicitly rather
  than let a reader infer a `RwLock<Vec<_>>` is what an adapter looks like
  (`CLAUDE.md`, "A port is only as well-designed as the *spread* of what
  implements it").
- **`crates/happenstance/src/lib.rs`** (75 lines on this branch) — AC-002's
  target. Verified: the crate-root doc (lines 11-71) currently points outward to
  crates.io links and to `spec/SPECIFICATION.md` only; it contains no pointer to
  any narrative/guide material, confirming the gap DR-2 names. Line 1-10 carries
  the `#![cfg_attr(doctest, doc = include_str!("../README.md"))]` mechanism
  AC-002 depends on for its compiled-doctest guarantee — the README's fences are
  literally re-parsed as this crate's own doctest, so a malformed pointer written
  into the README is caught here, not only by eye.
- **`crates/happenstance/README.md`** — AC-002's other target, and the file
  shared with the unmerged `publication-and-positioning` project (HS-P0016). The
  seam stated in `project.md` ("this project adds a pointer... and changes no
  claim on it") is real and checkable by diff; no other line in this file should
  be touched.
- **`CONTRIBUTING.md:69-95`** (four-step adapter recipe) and **`:97-140`** (why a
  provided method is never `async fn`, the `Self: Sync` placement rule, and the
  trait/impl asymmetry) — both verified present at those line ranges. Both are
  contributor-facing prose the reasoning account sequences and cites rather than
  copies (AC-008's own bar).
- **`standards/rust/91-adapter-authoring-recipe.md`** — verified: RS-91-1 through
  RS-91-4 give the six-step recipe with a compiled `PgStore` example implementing
  `SendEventStore` — the atom's own header names atoms 20, 21, 22, 23, 24, 40, 90,
  92 as related reading, which is useful context for what the reasoning account
  is *not* required to re-sequence (only 20 and 25 are named in `project.md`'s
  DR-7; 91 is a fourth, already-sequenced source in its own right).
- **`standards/rust/20-two-flavour-ports.md`** and
  **`standards/rust/25-what-removes-send-and-sync.md`** — both verified present;
  not read in full here (out of this grounding pass's budget) but confirmed as
  real, citable paths per DR-7.
- **`docs/README.md`** — verified: an 8-row table pointing to `spec/`,
  `standards/rust/`, `.kb/decisions/`, etc., stating plainly that the tree is
  "deliberately near-empty" and that "nothing has yet been written that belongs
  here instead." This is the directory `docs.rs`-adjacent narrative material
  would live under *if* HS-P0020 resolves hosting that way — but as of this
  grounding pass HS-P0020 has not authored `_design.md` yet (see Tensions below),
  so this project's front-door pointer cannot yet cite a concrete destination
  path with certainty; it can only cite the policy (DT-10) and the mechanism.
- **`.redkiln/config.yaml`** — verified line 15: "Deterministic gate commands, and
  design capture. BOTH OPTIONAL, both commented" — confirms `design.capture` is
  absent, so the perceptual design review is a skip and `_design.md`'s written
  resolution is the only record of DT-10, exactly as `CLAUDE.md` and `project.md`
  state.

## Sibling-project dependency state (verified, not assumed)

Both projects this project depends on for content and hosting shape are, as of
this grounding pass, still at `stage: storymap` — **neither has an `_design.md`
yet**:

- `.bklg/docs-that-teach/checked-documentation-surface/` (HS-P0020) — its
  `project.md` names the hosting/render shape (docs.rs-only vs. a separate
  rendered narrative surface) as a decision still owed to *its own* `_design.md`
  (verified at `project.md:115-121,229,250`). Until that lands, this project's
  front-door pointer (AC-002) has a policy (DT-10) but not a resolved destination
  URL/path to point at with certainty.
- `.bklg/docs-that-teach/page-need-discipline/` (HS-P0021) — its `project.md`
  likewise defers the discipline's decided home to its own `_design.md`
  (verified at `project.md:145-147,206-208`). AC-012 ("obey HS-P0021's
  discipline") cannot be checked against a concrete rule set until that design
  stage runs.

This is expected sequencing per the decomposition's dependency graph (HS-P0020,
then HS-P0021, before HS-P0023) and not a defect — but the architecture/ux briefs
for HS-P0023 should be written so DT-10's *policy* (once / per-item / both-with-
one-authoritative) and its rule can be decided and recorded independently of the
still-open hosting-shape decision, per `project.md`'s own "Available parallelism"
citation (`_decomposition.md`, "HS-P0023's DT-10 pointer policy can be resolved in
design while HS-P0022 is still authoring").

## Tensions and risks to flag

- **The referent/reasoning governance atom's worked examples are all ADR edits,
  not doc-comment edits.** `.kb/governance/rewrite-the-referent-never-the-
  reasoning.md` (verified, three worked instances, all `.kb/decisions/*`
  supersessions) states a general test — "ask whether the edit changes what the
  document asserts" — but every worked instance is a decision-record rename.
  Applying it to `store.rs`'s doc comment (a frozen-clause discharge, not a
  decision record) is the same test on a different artefact type; `CLAUDE.md`'s
  non-goal already sanctions this application by name ("Un-discharging the frozen
  documentation MUSTs... the referent may be rewritten; the reasoning may not"),
  so it is not a novel extension, but the architecture brief should say
  explicitly that it is analogy, not a fourth worked instance in the atom itself.
- **AC-010's clause-id set does not exist yet.** BR-10's pin is HS-P0020's
  responsibility, sequenced first in the DAG for exactly this reason. This
  project's own DoD item 2 and AC-010 are gated on that pin existing before the
  first edit to `store.rs` — a real, structural dependency, not a soft one. The
  architecture brief should state that the `store.rs` rewrite story cannot start
  implementation until HS-P0020's pinned set is committed, even though its
  *design* (what text goes where) can be drafted earlier.
- **Two files are shared with work outside this initiative.**
  `crates/happenstance/README.md` is edited by both this project and the
  unmerged `publication-and-positioning` (HS-P0016) on
  `initiative/from-contract-to-published-library`; `crates/happenstance/src/lib.rs`
  diverges hard between branches (75 lines here vs. 237 there, with the
  `Tags::empty()` defect intact in the sibling's version, per the decomposition's
  own risk note). The decomposition's operational rule — merge the sibling branch
  forward before implementing content that touches these files — is stated for
  HS-P0022 but applies verbatim to this project's AC-002 story, since it edits
  the same crate root and README. **NOT VERIFIABLE HERE**: the sibling branch's
  actual current state, since it is not present in this worktree; `redkiln
  status` here sees only this initiative.
- **No Accepted ADR or `.kb/` atom addresses pointer/navigation policy.**
  Confirmed by direct search — DT-10 is real, unconstrained design space, so the
  architecture/ux briefs are free to resolve it without reconciling against a
  prior decision. The only constraint is process (the `_design.md` template and
  the fact that the perceptual review is a permanent skip, so the written record
  is authoritative and final in a way it might not otherwise be).
- **`standards/rust/91-adapter-authoring-recipe.md` is a fourth reasoning source**
  beyond the three DR-7 names (`memory.rs`, `CONTRIBUTING.md` twice, 20, 25).
  Verified it exists and is directly on-topic (it is the atom that gives the
  compiled `SendEventStore` example). The briefs should decide explicitly whether
  AC-008's sequenced account cites it too, since leaving a directly relevant,
  already-written atom out of a "sequencing, not volume" account would be an odd
  omission for a page whose entire premise is findability over invention.

## Anchors verified present (spot-checked, not exhaustive — `project.md` carries the full list)

- `.kb/decisions/0001-async-port-flavours.md`, `.kb/decisions/0008-one-derivation-for-both-ports.md`
- `.kb/governance/rewrite-the-referent-never-the-reasoning.md`
- `standards/rust/20-two-flavour-ports.md`, `standards/rust/25-what-removes-send-and-sync.md`, `standards/rust/91-adapter-authoring-recipe.md`
- `crates/happenstance-core/src/store.rs` (lines 1-100 read; E0034 excerpt at 36-41 confirmed trimmed)
- `crates/happenstance-core/src/memory.rs` (lines 1-90 read)
- `crates/happenstance/src/lib.rs` (full file read, 75 lines, no narrative pointer present)
- `crates/happenstance/README.md` (full file read)
- `docs/README.md` (full file read)
- `CONTRIBUTING.md` lines 60-149 (covers the cited 69-95 and 97-140 ranges)
- `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md`, `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md`
- `.bklg/docs-that-teach/checked-documentation-surface/project.md`, `.bklg/docs-that-teach/page-need-discipline/project.md` (both confirmed `stage: storymap`, no `_design.md` authored yet)
- `.redkiln/config.yaml` (line 15 confirms `design.capture` absent/commented)
