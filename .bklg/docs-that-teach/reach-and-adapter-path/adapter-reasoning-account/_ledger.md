---
item: HS-S0155
stage: implement
created: "2026-08-17"
updated: "2026-08-17"
---

# Acceptance ledger — The sequenced adapter reasoning account

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

**`<TREE>` in `mount_point` was the concrete root of HS-P0020's pinned narrative tree, bound at
implementation — and it is now bound.** It was written symbolically because the tree did not exist when
this ledger was authored, and inventing its path is the invented-primitive failure `_design.md` refuses
(finding F5). HS-P0020 has since landed: the tree is `docs/` (`xtask/src/lint_narrative.rs:239`), its
no-orphan mechanism is `check_registration` (`:536`) reading `xtask/src/narrative.rs`, and this page's
route is `docs/adapter-reading-order.md`. Every `mount_point` below carries that path rather than the
placeholder. `EC-001` therefore did not fire and no row is blocked.

```yaml
- id: AC-001
  criterion: >-
    GIVEN an adapter author at B2 who has just been unblocked in
    crates/happenstance-core/src/store.rs and now wants the reasoning, WHEN they take the single hop
    the slice installs, THEN they land on a page that is *registered* in HS-P0020's pinned narrative
    tree — reachable by that tree's own no-orphan mechanism and compiled by its REQUIRED step — and
    not a loose file under docs/, a new //! module doc, or a section appended to CONTRIBUTING.md. If
    the tree or its registration path has not landed, the story blocks; an unregistered page is not a
    mounted page.
  satisfied: true
  evidence: >-
    docs/adapter-reading-order.md is registered at xtask/src/narrative.rs:160-161 (`mod adapter_reading_order` + `include_str!`). RED: `cargo xtask narrative` reported "xtask/src/narrative.rs — does not include adapter-reading-order.md" and "no `mod adapter_reading_order`"; GREEN after registration: "6 pages, all consistent". The no-orphan mechanism is `check_registration` (xtask/src/lint_narrative.rs:536-563), bidirectional, under the mandatory step named at :351. Also indexed at docs/README.md:21. Transcripts in adapter-reasoning-account/_verification.md, section AC-001.
  mount_point: "HS-P0020's pinned narrative tree — the page registered where the tree's no-orphan mechanism reads it (.bklg/docs-that-teach/checked-documentation-surface/project.md:215-216); composition roots CR-3 (tree path constant in xtask/src/) and CR-4 (the REQUIRED step list, xtask/src/main.rs:85-102)"
  verifying_test: "cargo xtask ci --fast (xtask/src/main.rs) — HS-P0020's no-orphan step; registration file:line and gate transcript recorded in .bklg/docs-that-teach/reach-and-adapter-path/adapter-reasoning-account/_verification.md"

- id: AC-002
  criterion: >-
    GIVEN a reader meeting the page for the first time at 1024x768, WHEN it renders, THEN the five
    regions appear in the bound order (H1 + answered-need · the numbered reading order · the caveat at
    entry 1 · six positioned sections · the terminal region), and the complete six-entry reading order
    is visible before the first section, inside the first screen — H1 + need (3) + list (≤ 12) = ≤ 15
    rendered lines against a first screen of ≈ 27 — so the reader learns the sequence from an
    enumeration rather than from scroll position. The list is the page's only enumerated element, it
    precedes everything including the first section, and it never loses an entry to fit.
  satisfied: true
  evidence: >-
    Five regions in the bound order at docs/adapter-reading-order.md:1, :3, :7-19, :28-35, :21/:37/:44/:50/:58/:65 and :71-77. Measured first screen: H1 42 + declaration 90 + lead-in 83 + six entries of 176/173/162/159/148/143 rendered characters = 15 rendered lines at both a 96- and a 105-column content box, against the design's <= 15 of an ~27-line first screen. The list is the page's only enumerated element (rg for list markers returns six matches, all in region 2). Measurement table in adapter-reasoning-account/_verification.md, section AC-002.
  mount_point: "docs/adapter-reading-order.md, regions 1–2 (.bklg/docs-that-teach/reach-and-adapter-path/_design.md, ## Composition, adapter-reasoning-account)"
  verifying_test: "Rendered-line count at 1024x768 plus region-order check recorded in .bklg/docs-that-teach/reach-and-adapter-path/adapter-reasoning-account/_verification.md, against .bklg/docs-that-teach/reach-and-adapter-path/_design.md ## Density budget; visual check against .bklg/docs-that-teach/reach-and-adapter-path/design/mock.html#all"

- id: AC-003
  criterion: >-
    GIVEN the adapter author at B3 trying to build a model of what an adapter is shaped like, WHEN they
    read the page end to end, THEN they meet exactly the six decided sources in the decided order — (1)
    crates/happenstance-core/src/memory.rs (the reference-implementation doc and its full-DCB-loop
    doctest), (2) standards/rust/91-adapter-authoring-recipe.md (RS-91-1..4, the compiled PgStore), (3)
    CONTRIBUTING.md `## Writing an adapter` (:69), (4) standards/rust/20-two-flavour-ports.md, (5)
    standards/rust/25-what-removes-send-and-sync.md, (6) CONTRIBUTING.md `## Adding a method to a port`
    (:97, rule at :104) — each reached by an anchored named-subject + path citation with any line range
    as a convenience rather than the only handle, and none of the six copied onto the page (the
    memory.rs doctest is cited, never pasted).
  satisfied: true
  evidence: >-
    The six sources in the decided order, in the reading order at :7, :9, :11, :13, :16, :18 and again in the six sections at :24, :39, :46, :52, :61, :67; the extra hit at :34 is the pairing AC-005 mandates. Every citation is named subject + path and every subject resolves today: "The reference implementation" (crates/happenstance-core/src/memory.rs:16), PgStore (standards/rust/91-adapter-authoring-recipe.md:55), "Writing an adapter" (CONTRIBUTING.md:69), RS-20-1, RS-25-1, "Adding a method to a port" (CONTRIBUTING.md:97). EC-006 checked: rg for "Provided methods" returns nothing. Nothing copied: the page carries zero fenced blocks, so memory.rs's doctest is cited and never pasted. adapter-reasoning-account/_verification.md, section AC-003.
  mount_point: "docs/adapter-reading-order.md, regions 2 and 4 — the reading order and the six sequenced sections"
  verifying_test: "rg assertions recorded in .bklg/docs-that-teach/reach-and-adapter-path/adapter-reasoning-account/_verification.md — each of the six paths cited once, in document order, each with a subject string that rg-matches in the cited file; sixth heading asserted as 'Adding a method to a port'"

- id: AC-004
  criterion: >-
    GIVEN a reader who lands mid-sequence — arriving at section 3 from a store.rs fragment or a search
    result, with no memory of having passed a setup — WHEN they read the heading they landed on, THEN
    it carries its own position ("3 of 6" or equivalent) so they learn from that heading alone that two
    sections precede them, without scrolling up. No section heading is unpositioned.
  satisfied: true
  evidence: >-
    rg over docs/adapter-reading-order.md returns one H1 and seven level-2 headings; six read "## N of 6 — ..." at :21, :37, :44, :50, :58 and :65, and the numbering agrees entry-for-entry with the reading order at :7-19. The seventh (:71) is the terminal region and names no source. Anti-pattern 17's bare source-name heading is absent. adapter-reasoning-account/_verification.md, section AC-004.
  mount_point: "docs/adapter-reading-order.md, region 4 — the six section headings"
  verifying_test: "rg over the page recorded in .bklg/docs-that-teach/reach-and-adapter-path/adapter-reasoning-account/_verification.md — six section headings, six position markers, numbering diffed against the reading order; named wrong implementation is anti-pattern 17 (.bklg/docs-that-teach/reach-and-adapter-path/_design.md, ## Anti-patterns)"

- id: AC-005
  criterion: >-
    GIVEN a reader who will generalise the first implementation they are shown into "the shape an
    adapter takes", WHEN MemoryEventStore is first sequenced at entry 1, THEN the page states in that
    same place that it is the conformance suite's oracle and the reference implementation and not an
    adapter — deferring to memory.rs's own three-reasons list at
    crates/happenstance-core/src/memory.rs:16-31 rather than paraphrasing it — and names in the same
    breath at least one adapter at the other end of the storage-shape axis
    (standards/rust/91-adapter-authoring-recipe.md's PgStore; references/adapter-shapes.md). Not a
    closing caveat, not a footnote.
  satisfied: true
  evidence: >-
    The caveat opens "**And it is not an adapter.**" at docs/adapter-reading-order.md:28 — inside entry 1's block and before section 2's heading at :37, so neither a closing note nor a footnote. It defers to memory.rs's own three-reasons list (crates/happenstance-core/src/memory.rs:16-25) rather than paraphrasing it, and names the other end of the storage-shape axis in the same paragraph: the PgStore in standards/rust/91-adapter-authoring-recipe.md (:33-34) and references/adapter-shapes.md (:35). adapter-reasoning-account/_verification.md, section AC-005.
  mount_point: "docs/adapter-reading-order.md, region 3 — the caveat in position at entry 1 (state memory-caveat-in-position)"
  verifying_test: "Position assertion in .bklg/docs-that-teach/reach-and-adapter-path/adapter-reasoning-account/_verification.md — the caveat's first sentence occurs inside entry 1's block and before section 2's heading, and the pairing names a second implementation in the same paragraph (UX-AC-07, .bklg/docs-that-teach/reach-and-adapter-path/_decomposition.md)"

- id: AC-006
  criterion: >-
    GIVEN a maintainer performing DoD 12's spot check ("no page has become a second specification"),
    WHEN they read every normative claim on the page, THEN each is a citation that resolves: ADR-0001
    (.kb/decisions/0001-async-port-flavours.md) and ADR-0008
    (.kb/decisions/0008-one-derivation-for-both-ports.md) are cited for why two flavours exist and are
    reached through the supersession graph in .kb/maps/decision-map.md so no superseded atom is cited;
    every spec/SPECIFICATION.md claim is a clause id cited, never a clause restated; and the page is
    not a third statement of the two-flavour rule beside standards/rust/20-two-flavour-ports.md and
    standards/rust/25-what-removes-send-and-sync.md. The precedence chain at
    standards/rust/README.md:25-29 is obeyed and not extended.
  satisfied: true
  evidence: >-
    Two clause ids, both resolving and both checked mechanically by check_citations (xtask/src/lint_narrative.rs:1197): VT-11 at :32 (spec/SPECIFICATION.md:1050) and CF-15 at :42 (spec/SPECIFICATION.md:7949) — `cargo xtask narrative` green is that check passing. ADR-0001 is cited at :54 and ADR-0008 at :69, both reached through .kb/maps/decision-map.md:68 and :75, where each row reads status accepted with an empty supersession column. Not a third statement: :55-56 says in the page's own voice that it states neither the rule nor the decision. adapter-reasoning-account/_verification.md, section AC-006.
  mount_point: "docs/adapter-reading-order.md, region 4 — the why-sentences of entries 4, 5 and 6, inside HS-P0020's checked surface"
  verifying_test: "cargo xtask spec-trace (xtask/src/spec_trace.rs) plus HS-P0020's AC-007 clause-id check once landed; every cited clause id resolved against spec/SPECIFICATION.md and every cited atom checked against .kb/maps/decision-map.md, recorded in .bklg/docs-that-teach/reach-and-adapter-path/adapter-reasoning-account/_verification.md"

- id: AC-007
  criterion: >-
    GIVEN the one-need review pass a reviewer who did not write the page must be able to run
    (initiative DoD 8), WHEN they read the page's first region, THEN it declares exactly one answered
    need — "in what order do I read what already exists, to build an adapter" — and the page carries no
    second need. HS-P0021's rule is obeyed, not authored: where it has not landed, the page is authored
    to the initiative's stated shape and expects a conformance pass.
  satisfied: true
  evidence: >-
    docs/adapter-reading-order.md:3 carries one declaration, immediately after the H1 with nothing interposed, 96 characters against RP-00-2's cap: `> **Answers:** `explanation` — In what order do I read what already exists, to build an adapter?`. RED: `cargo xtask lint-pages` reported "docs/adapter-reading-order.md — no `> **Answers:**` line"; GREEN: "6 pages, 16 rules, all consistent". check_declarations (xtask/src/lint_pages.rs:630) fails on a second need, so singularity is mechanical on this branch. The token is `explanation` rather than the design's routing-shaped wording because RP-10-3's one-orientation-page-per-directory ceiling is already held by docs/read-the-worked-example.md:3 and RP-10-2's ceiling forbids this page's own signed-off density budget; the need sentence itself is unchanged. Recorded in full in adapter-reasoning-account/_verification.md, section AC-007.
  mount_point: "docs/adapter-reading-order.md, region 1 — the H1 and its answered-need declaration"
  verifying_test: "Declaration recorded verbatim and asserted singular in .bklg/docs-that-teach/reach-and-adapter-path/adapter-reasoning-account/_verification.md; re-checked against .bklg/docs-that-teach/page-need-discipline/project.md:224-226 (HS-P0021 AC-006) once that rule lands"

- id: AC-008
  criterion: >-
    GIVEN the project's own finding that "AC-05 is a sequencing job, not an authoring job", WHEN the
    finished page is measured, THEN it is ≤ 900 words (hard cap 1,200) and no single source's
    connective tissue exceeds 5 sentences / ~120 words. WHEN the count would exceed 1,200, THEN the
    story escalates to the initiative rather than trimming the sequence — connective tissue is the only
    thing that ever yields, and neither a citation nor the caveat is ever what yields.
  satisfied: true
  evidence: >-
    wc -w over docs/adapter-reading-order.md returns 704, against a <= 900 target, a 1,200 hard cap and a ~350 floor. Per-region: head plus reading order 187; section 1's connective tissue 43 and its caveat 111; then 69, 54, 59, 67, 52; terminal region 70. No source's connective tissue exceeds five sentences or ~120 words. EC-003 did not fire and nothing was trimmed from the sequence. adapter-reasoning-account/_verification.md, section AC-008.
  mount_point: "docs/adapter-reading-order.md, whole page — the density budget in .bklg/docs-that-teach/reach-and-adapter-path/_design.md, ## Density budget, 'adapter-reasoning-account, whole page' row"
  verifying_test: "wc -w over the page file plus a per-section count, recorded in .bklg/docs-that-teach/reach-and-adapter-path/adapter-reasoning-account/_verification.md against .bklg/docs-that-teach/reach-and-adapter-path/_design.md ## Density budget; overflow path is spec EC-003"

- id: AC-009
  criterion: >-
    GIVEN a reader who has finished the page and must not be left at a dead end (UX invariant 5), WHEN
    they reach the terminal region, THEN it states what the page does not cover and offers exactly one
    onward hop — never a list, and never a "See also" / "Next steps" / "Further reading" block — whose
    form is a guarded rung of the href ladder (an in-tree markdown link inside the pinned tree, guarded
    by HS-P0020's registration check; or the named-but-unlinked cross-reference form live at
    crates/happenstance-core/src/store.rs:77), whose visible link text is a self-describing noun phrase
    of ≥ 3 words — never "here", "this", "docs", "read more" — and which is not a bare URL into this
    repository's own tree.
  satisfied: true
  evidence: >-
    rg for markdown links over the page returns exactly one match, at :77, inside the terminal region: [why an append re-reads its condition](append-conditions.md) — six words, a self-describing noun phrase, none of "here", "this", "docs" or "read more". Limits are stated first at :73-74. rg for "See also|Next steps|Further reading" and for "https?://" both return nothing. The rung taken is 2 of the href ladder — an in-tree markdown link inside the pinned tree (xtask/src/pointers.rs:157-163, PointerForm::PinnedTreeMarkdown) — guarded by check_registration at xtask/src/lint_narrative.rs:536. No register row is filed and the reason is written down. adapter-reasoning-account/_verification.md, section AC-009.
  mount_point: "docs/adapter-reading-order.md, region 5 — the terminal region (state terminal-section-onward-hop); the hop's form and guard come from the policy landed by pointer-policy-and-inventory"
  verifying_test: "rg assertions in .bklg/docs-that-teach/reach-and-adapter-path/adapter-reasoning-account/_verification.md — exactly one outbound link in the terminal region, no in-tree https:// URL, no See also/Next steps/Further reading heading, link text ≥ 3 words; named wrong implementations are anti-patterns 3 and 5 (.bklg/docs-that-teach/reach-and-adapter-path/_design.md)"

- id: AC-010
  criterion: >-
    GIVEN a keyboard-only reader on a screen reader, and GIVEN a second reader who never follows the
    hop out of store.rs at all, WHEN each reads what is in front of them, THEN (a) the page is composed
    only from the enumerated design-system primitives — a markdown link, an unskipped heading ladder,
    compiled/uncompiled fences — with no raw HTML, no inline style=, no <details>, tab, accordion or
    fold, all six sources visible rather than revealed, every hop reachable by keyboard, nothing
    animating, and nothing carrying meaning in colour or position alone; and (b) deleting this page
    entirely leaves crates/happenstance-core/src/store.rs still able to resolve error[E0034] in place —
    nothing on this page is load-bearing for getting unstuck (UX invariant 1).
  satisfied: true
  evidence: >-
    (a) rg for `<details`, `<summary`, `<div`, `<table`, `style=`, tab markers and admonitions over docs/adapter-reading-order.md returns nothing, and so does rg for fenced blocks; the heading ladder is level 1 then level 2 with no skip and no level 3; all six sources are visible rather than revealed, this project installing zero folded, tabbed or opened-on-demand content; the single hop is a plain markdown link, so no bespoke control exists to be unreachable by keyboard; no colour and no spatial encoding is authored. (b) rg -n 'adapter-reading-order' crates/ returns nothing at this commit, and crates/happenstance-core/src/store.rs:31-45 still carries the cause, the error[E0034] block, the import rule and the fully-qualified escape hatch SendEventStore::read(&store, &query, options) — a reader who never hops is correctly unstuck. The re-run with the pointer installed is recorded in store-error-site-rewrite/_verification.md. adapter-reasoning-account/_verification.md, section AC-010.
  mount_point: "docs/adapter-reading-order.md, whole page (a); and crates/happenstance-core/src/store.rs's `## Import one flavour, not both` section read with the page absent (b)"
  verifying_test: "(a) rg assertions for <details>, <div, style=, <table and an unskipped heading-ladder check, plus a keyboard-only pass, recorded in .bklg/docs-that-teach/reach-and-adapter-path/adapter-reasoning-account/_verification.md; (b) the B1-independence deletion check on crates/happenstance-core/src/store.rs recorded in the same file"
```
