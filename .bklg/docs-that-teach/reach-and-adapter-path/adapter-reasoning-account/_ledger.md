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

**`<TREE>` in `mount_point` is the concrete root of HS-P0020's pinned narrative tree, bound at
implementation.** It is written symbolically because the tree does not exist on this branch and
inventing its path is the invented-primitive failure `_design.md` refuses (finding F5). If it has not
landed when implementation starts, the story blocks (spec `EC-001`) — it does not ship a loose file.

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
  satisfied: false
  evidence: ""
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
  satisfied: false
  evidence: ""
  mount_point: "<TREE>/<the reasoning account page>, regions 1–2 (.bklg/docs-that-teach/reach-and-adapter-path/_design.md, ## Composition, adapter-reasoning-account)"
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
  satisfied: false
  evidence: ""
  mount_point: "<TREE>/<the reasoning account page>, regions 2 and 4 — the reading order and the six sequenced sections"
  verifying_test: "rg assertions recorded in .bklg/docs-that-teach/reach-and-adapter-path/adapter-reasoning-account/_verification.md — each of the six paths cited once, in document order, each with a subject string that rg-matches in the cited file; sixth heading asserted as 'Adding a method to a port'"

- id: AC-004
  criterion: >-
    GIVEN a reader who lands mid-sequence — arriving at section 3 from a store.rs fragment or a search
    result, with no memory of having passed a setup — WHEN they read the heading they landed on, THEN
    it carries its own position ("3 of 6" or equivalent) so they learn from that heading alone that two
    sections precede them, without scrolling up. No section heading is unpositioned.
  satisfied: false
  evidence: ""
  mount_point: "<TREE>/<the reasoning account page>, region 4 — the six section headings"
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
  satisfied: false
  evidence: ""
  mount_point: "<TREE>/<the reasoning account page>, region 3 — the caveat in position at entry 1 (state memory-caveat-in-position)"
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
  satisfied: false
  evidence: ""
  mount_point: "<TREE>/<the reasoning account page>, region 4 — the why-sentences of entries 4, 5 and 6, inside HS-P0020's checked surface"
  verifying_test: "cargo xtask spec-trace (xtask/src/spec_trace.rs) plus HS-P0020's AC-007 clause-id check once landed; every cited clause id resolved against spec/SPECIFICATION.md and every cited atom checked against .kb/maps/decision-map.md, recorded in .bklg/docs-that-teach/reach-and-adapter-path/adapter-reasoning-account/_verification.md"

- id: AC-007
  criterion: >-
    GIVEN the one-need review pass a reviewer who did not write the page must be able to run
    (initiative DoD 8), WHEN they read the page's first region, THEN it declares exactly one answered
    need — "in what order do I read what already exists, to build an adapter" — and the page carries no
    second need. HS-P0021's rule is obeyed, not authored: where it has not landed, the page is authored
    to the initiative's stated shape and expects a conformance pass.
  satisfied: false
  evidence: ""
  mount_point: "<TREE>/<the reasoning account page>, region 1 — the H1 and its answered-need declaration"
  verifying_test: "Declaration recorded verbatim and asserted singular in .bklg/docs-that-teach/reach-and-adapter-path/adapter-reasoning-account/_verification.md; re-checked against .bklg/docs-that-teach/page-need-discipline/project.md:224-226 (HS-P0021 AC-006) once that rule lands"

- id: AC-008
  criterion: >-
    GIVEN the project's own finding that "AC-05 is a sequencing job, not an authoring job", WHEN the
    finished page is measured, THEN it is ≤ 900 words (hard cap 1,200) and no single source's
    connective tissue exceeds 5 sentences / ~120 words. WHEN the count would exceed 1,200, THEN the
    story escalates to the initiative rather than trimming the sequence — connective tissue is the only
    thing that ever yields, and neither a citation nor the caveat is ever what yields.
  satisfied: false
  evidence: ""
  mount_point: "<TREE>/<the reasoning account page>, whole page — the density budget in .bklg/docs-that-teach/reach-and-adapter-path/_design.md, ## Density budget, 'adapter-reasoning-account, whole page' row"
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
  satisfied: false
  evidence: ""
  mount_point: "<TREE>/<the reasoning account page>, region 5 — the terminal region (state terminal-section-onward-hop); the hop's form and guard come from the policy landed by pointer-policy-and-inventory"
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
  satisfied: false
  evidence: ""
  mount_point: "<TREE>/<the reasoning account page>, whole page (a); and crates/happenstance-core/src/store.rs's `## Import one flavour, not both` section read with the page absent (b)"
  verifying_test: "(a) rg assertions for <details>, <div, style=, <table and an unskipped heading-ladder check, plus a keyboard-only pass, recorded in .bklg/docs-that-teach/reach-and-adapter-path/adapter-reasoning-account/_verification.md; (b) the B1-independence deletion check on crates/happenstance-core/src/store.rs recorded in the same file"
```
