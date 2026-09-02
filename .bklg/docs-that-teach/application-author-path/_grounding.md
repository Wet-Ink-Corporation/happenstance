---
Companion file — not a Redkiln item. No frontmatter fields are system-owned here;
edit freely. Written for HS-P0022 `application-author-path`.
---

# Grounding — The Application Author's Path

## 1. What this project actually owns (from the decomposition gate)

Per `.bklg/docs-that-teach/_decomposition.md`, HS-P0022 is the persona-1 vertical
slice: DT-1, DT-4, DT-5, DT-6 (all four owned tensions are one coupled decision —
DT-1 must be settled before DT-4 stages anything against it, and DT-5/DT-6 are
"how is the shift drawn, how honest is the old side"); BR-03, BR-07, BR-18; AC-01,
AC-02, AC-08; DoD-3, DoD-4. It depends on HS-P0020 (`checked-documentation-surface`)
and HS-P0021 (`page-need-discipline`), and unlocks HS-P0023/HS-P0024. Only `ux` and
`testing` briefs are warranted (`_decomposition.md:319`) — no `architecture` brief
(the pinned tree and gate wiring belong to HS-P0020) and no `deployment` brief
(HS-P0020 is the only project that earns one). The `testing` tag is earned
specifically by DoD-4 (removing the boundary must make a repository check fail)
and, if DT-6 resolves to a real compiled wrong side, by a `publish = false`
artefact the gate must check forever (`_decomposition.md:327-328`).

**Load-bearing sequencing constraint, easy to violate by accident:** the
decomposition gate recorded that `crates/happenstance/src/lib.rs` on this branch
(75 lines, confirmed by `wc -l`) is a materially different, shorter file than the
sibling branch `initiative/from-contract-to-published-library`'s copy (237 lines),
which is where the `Tags::empty()`-twice defect that motivates this whole
initiative actually lives (`_decomposition.md:83-103`). AC-014 codifies this as a
gate: the merge-forward from that branch must complete and be recorded *before*
the first page is authored, and the opening encounter must be written against the
merged `lib.rs`, not this worktree's stale copy. This is not optional groundwork —
authoring against the current 75-line file would mean redoing the work at merge.

## 2. Accepted decision atoms this project must not contradict

- **`.kb/decisions/0006-bare-name-to-the-typed-layer.md`** (ADR-0006, accepted,
  not provisional). `happenstance` (bare name) is the typed layer an application
  `cargo add`s; `happenstance-core` is the contract crate an adapter author pins.
  Any opening-encounter code the reader is meant to write should import from
  `happenstance`, not `happenstance-core` — the crate's own module docs
  (`crates/happenstance/src/lib.rs:44-49`) already state this discriminator
  (encoding, not orchestration) and note `happenstance-core`'s doc must say so "in
  its first paragraph" (ADR-0006 Consequences, "Bad"). AC-010's worked example
  (`examples/course-subscriptions/src/main.rs`) currently imports from
  `happenstance_core` directly (`main.rs:24`), which is correct for that example's
  own purpose (it is a store/testkit-level demonstration, not an application
  author's onboarding surface) — the bridge material this project writes should
  not silently copy that import path if the persona it addresses is meant to enter
  through `happenstance`.
- **`.kb/decisions/0007-projection-runner-decodes.md`** (ADR-0007, accepted).
  Relevant only if any opening-encounter material touches projections; the opening
  encounter as scoped (a boundary refusing an append) does not need the
  `Projection` trait, so this project is unlikely to touch it. Flagged for
  completeness, not because it constrains anything here.
- **`.kb/governance/rewrite-the-referent-never-the-reasoning.md`.** Governs how
  *this repository's own* accepted records may be touched — not directly binding
  on narrative prose this project authors, but the discrimination it teaches
  (rewrite the referent, never the reasoning) is a useful frame if the bridge
  material ever needs to describe why a name changed (e.g. explaining
  `happenstance` vs `happenstance-core` to a reader coming from a single-crate
  mental model).

No Accepted decision atom addresses documentation structure, prior-model framing,
diagrams, or disclosure shape — `CLAUDE.md`'s own summary states this plainly
("none of the seventeen decisions concerns documentation"). DT-1, DT-4, DT-5, DT-6
are therefore this project's genuinely free decisions to make in `_design.md`, not
resolutions to look up. There is nothing in `.kb/` to contradict on those four axes
— the risk is inventing a decision and dressing it as grounded, not disagreeing
with one that exists.

## 3. The specification clauses this project's teaching must cite, not restate

Per AC-011/AC-012 and BR-09, every normative claim on a page this project authors
must be a citation to a resolving `spec/SPECIFICATION.md` clause, never a
paraphrase. The clause that *is* the opening encounter's entire teaching content:

- **ES-25 — Condition semantics** (`spec/SPECIFICATION.md:3693`, `[FROZEN]`): "The
  store MUST reject the append if and only if it holds at least one event matching
  a guard's query at a position strictly greater than that guard's `after`. With
  `after: None` any match at all MUST reject. A rejection MUST be reported as
  `AppendError::ConditionViolated`... A `ConditionViolated` MUST be returned only
  when nothing was written." This is the single clause the boundary-refusal demo
  (AC-004/DoD-3) exists to make concrete. `[FROZEN]` — any page describing this
  behaviour cites ES-25 rather than re-deriving it in prose.
- **VT-30 — An `AppendCondition` is one or more guards** (`:1813`, `[PROVISIONAL]`):
  the shape (`AppendCondition { guards: Box<[Guard]> }`, `Guard { query, after }`)
  the bridge material (AC-008) must land the reader on. Provisional, so a page
  citing it should not imply the shape is permanently frozen.
- **ES-26 — The AC3 boundary: `after` is exclusive, `from` is inclusive**
  (`:3755` area) — relevant if the bridge material's worked invariant needs to
  explain why the boundary position behaves the way it does.
- **ES-27 — A condition matches on tags, not only on types** — relevant if the
  bridge's cross-entity invariant (AC-008 asks for a *cross-entity* invariant
  specifically) is built with `Tags` rather than `Query::of_types` alone; a
  cross-entity boundary is exactly the shape CF-7 says a type-only probe can
  silently fail to enforce (`:7266-7281`), which is good source material for why
  the bridge should reach for tags, not just types.
- **CF-7** (`:7266`, `[FROZEN]`, discharged at phase 3 stage 4) is the
  conformance obligation behind ES-27 and is good grounding for *why* the
  cross-entity example in AC-008 should be tag-based: an adapter that drops the
  tag join "rejects every command touching any course, and passes all
  twenty-seven rules while doing it" — a vivid, already-written cautionary
  example of what an empty/wrong boundary costs, useful raw material (cite the
  clause, do not restate its prose) if the opening encounter or bridge wants to
  motivate *why* the boundary must be real (ties to AC-006, "no page constructs an
  empty consistency boundary... where prose claims a real one").

`spec/SPECIFICATION.md:280` ("Clause IDs are stable and are never renumbered")
is itself already cited at the decomposition gate as the reason a 521-line
specification divergence on the sibling branch does not invalidate citations by
clause id — safe to cite `ES-25` etc. now even though the merge-forward (AC-014)
has not yet happened.

## 4. Existing code this project's material is built on or must not duplicate

- **`examples/course-subscriptions/src/main.rs`** — AC-010 requires this example
  be reachable from this project's material with its module doc
  (`main.rs:1-19`) *surfaced*, not paraphrased. Confirmed present and matches the
  citation: lines 1-19 are exactly the module doc block (three invariants, why
  none fits a single aggregate, DCB's per-decision boundary, "Run with `cargo run
  -p course-subscriptions`"). It imports `happenstance_core` directly (`:24-27`),
  uses `MemoryEventStore`, `Query`/`QueryItem`, `Tags`, `AppendCondition`,
  `read_decision_model` — the exact vocabulary AC-008's bridge must land the
  reader on. This example already demonstrates all three of the invariants named
  in scope; it is candidate material to *link to* rather than a template the
  opening encounter needs to re-derive from scratch, per the "findability, not
  volume" framing that runs through the whole initiative charter.
- **`crates/happenstance/src/lib.rs`** (this worktree's copy, 75 lines,
  superseded per §1 above once AC-014's merge lands) — its module docs already
  demonstrate the `happenstance`-crate-first vocabulary
  (`use happenstance::{EventStore, MemoryEventStore, Query, ReadOptions, collect};`
  at `:60`) and are compiled as a doctest (D10, cited in the file's own leading
  comment) — i.e. this file is itself already inside the checked-documentation
  mechanism this project's own fenced blocks must join (AC-007).
- **`crates/happenstance-core/src/store.rs`** — the trait-resolution error
  explanation lives here (owned by HS-P0023, not this project) but its doc
  comments on `EventStore`/`SendEventStore` are the reference-level account of
  the two-flavour design; not this project's job to write, but useful
  vocabulary-consistency check for anything the bridge says about `EventStore`.
- **`.kb/concepts/torn-reads-and-the-append-condition-boundary.md`** — an
  existing, accepted, correct account of a *related but distinct* failure mode
  (a torn read producing a silently-accepted append rather than a refused one).
  Directly relevant to AC-006 ("no page constructs an empty consistency boundary
  ... where prose claims a real one") as a worked cautionary example already in
  the corpus, and useful background for why the opening encounter's refusal must
  be real rather than illustrative — but it is pitched at an implementer/adapter
  reader (cites `store.rs:205-215`, ADR-0011/12/13) and is not itself
  application-author-facing prose this project can repurpose directly.

## 5. Tensions and gaps flagged for the design stage (not resolved here)

- **DT-1 has zero grounding in `.kb/`.** A repo-wide `rg -n "aggregate" .kb`
  returns no hits at all. The charter itself states the reason: `happenstance` is
  DCB-native and "never had an aggregate to kill" — every comparable teacher in
  the field's prior art relieves an existing pain this project does not have. This
  is confirmed, not merely asserted at intake; `_design.md` will be authoring
  fresh reasoning here, not citing a settled position, and AC-001 correctly
  requires that resolution be recorded with its evidence rather than assumed.
- **The intake brief (`_intake-brief.md`) is an unfilled template stub** — every
  section still holds its placeholder prose ("The problem or opportunity in one
  paragraph.", etc.) even though its Gate: Intake checkboxes are ticked. It
  carries no citable content of its own; all real grounding for this project
  lives in the initiative charter and the decomposition, both read above. Do not
  cite `_intake-brief.md` for substance in the briefs — its sections are empty.
- **This branch does not yet contain the `Tags::empty()` defect** that is the
  initiative's headline evidence — confirmed by reading this worktree's
  `crates/happenstance/src/lib.rs` in full (75 lines, no `Tags::empty()` call
  anywhere, no course-subscriptions-style example inline). The defect and its
  fix belong to the merged-forward file per AC-014; do not write briefs that
  describe fixing a defect visible in *this* file, since it is not here yet.
- **AC-008's "cross-entity invariant... in ordinary event-sourcing vocabulary"
  starting point has no single existing worked statement in this repo** in plain
  prose (only in code, in `course-subscriptions`). The bridge's first step (the
  "ordinary vocabulary" statement of the invariant, before any library terms
  appear) is original prose this project must write, grounded in the *shape* of
  `course-subscriptions`'s three invariants but not copied from existing text.
- **DT-6's exemption mechanism, if a "wrong model" contrast ships as illustrative
  (not compiled) code**, needs an "exact, narrow basis" per AC-003. No existing
  precedent for an intentionally-uncompiled fenced block exists in this repo to
  cite — HS-P0020 owns the compiled-prose mechanism and its documented failure
  mode (a page of opted-out fences "type-checks against anything," per
  `initiative.md` BR-01 rationale) is the cautionary precedent to weigh against,
  not a template to follow.

## 6. Anchors for the briefs to cite

- `.bklg/docs-that-teach/initiative.md` — vision, BR-03/07/18, AC-01/02/08,
  DoD-3/4, DT-1/4/5/6, the `Tags::empty()` defect narrative (lines 50-59), open
  question on cross-branch verification (lines 544-552)
- `.bklg/docs-that-teach/_decomposition.md` — this project's exact scope cut,
  dependency edges, warranted briefs, DT ownership table, merge-order rationale
- `.kb/decisions/0006-bare-name-to-the-typed-layer.md` — which crate the bridge
  material must import from
- `.kb/decisions/0007-projection-runner-decodes.md` — out of scope but checked
- `.kb/governance/rewrite-the-referent-never-the-reasoning.md` — background frame
- `.kb/concepts/torn-reads-and-the-append-condition-boundary.md` — related
  cautionary mechanism, adapter-pitched, not directly reusable prose
- `spec/SPECIFICATION.md:3693` (ES-25, `[FROZEN]`) — the boundary-refusal clause
- `spec/SPECIFICATION.md:1813` (VT-30, `[PROVISIONAL]`) — the `AppendCondition`
  shape
- `spec/SPECIFICATION.md:7266` (CF-7, `[FROZEN]`) — cross-entity/tag-matching
  cautionary example
- `spec/SPECIFICATION.md:280` — clause-id stability across the pending merge
- `examples/course-subscriptions/src/main.rs:1-19` — the module doc AC-010
  requires be surfaced, not paraphrased
- `crates/happenstance/src/lib.rs` — this worktree's stale copy; superseded by
  AC-014's merge before authoring
- `docs/README.md` — states this tree is for user documentation and is
  deliberately near-empty; the front-door signpost table this project's material
  must eventually be reachable through (AC-11 territory, owned by HS-P0023, but
  the table itself lives here)
