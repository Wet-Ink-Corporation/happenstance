---
item: "HS-S0187"
stage: implement
created: "2026-08-17"
updated: "2026-08-17"
---

# Acceptance ledger — The reader's invariant carried to an AppendCondition

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two rows are worth reading before implementing rather than after. **AC-002** is unmet by a fence
that type-checks without executing: if HS-P0020's narrative step turns out not to run doctests, the
answer is EC-002's `xtask/tests/bridge_guard.rs` wired into the `"tests"` REQUIRED step — a scope
change recorded through `redkiln advance` — never a weaker criterion flipped green. **AC-004** is
the row that fails loudly if the wrong wrong-side ships: a type-only guard over-refuses (CF-7), so
its `is_ok()` assertion goes red rather than passing quietly.

```yaml
- id: AC-001
  criterion: "GIVEN a reader holding a cross-entity invariant stated in ordinary event-sourcing vocabulary and no familiarity with this library's types, WHEN they read the page from its answered-need line to `## Back to the working version` with every off-page link struck out, THEN they can write their own `Query` of one `QueryItem` per entity's tag set, the fold over the events it returns, and the `AppendCondition` built from that same query — and at no step were they required to open `spec/SPECIFICATION.md` or any file under `crates/`. Clause citations are provenance, never required reading."
  satisfied: false
  evidence: ""
  mount_point: "docs/carry-your-invariant.md, registered by an include_str! line in xtask/src/narrative.rs under the pinned TREE = \"docs\""
  verifying_test: "Tier 5 — IQ-1's strike-every-off-page-link falsification walked against the rendered page and recorded in .bklg/docs-that-teach/application-author-path/invariant-to-appendcondition-bridge/implementation-report.md; mechanical half is the §5 fence compiling under HS-P0020's narrative REQUIRED step"

- id: AC-002
  criterion: "GIVEN a reader who trusts the page because the repository's own gate compiles it, WHEN `cargo xtask ci --fast` runs from a clean checkout, THEN `docs/carry-your-invariant.md` is registered by exactly one `include_str!` line in `xtask/src/narrative.rs` under the pinned `TREE = \"docs\"` (neither unregistered nor dangling), both Rust fences carry a recognised info string, and both are executed — neither is `ignore`, neither is `no_run`, and zero entries are added to `IGNORE_ALLOWANCES`. A fence that type-checks but never runs does not satisfy this criterion."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/narrative.rs — the include_str! registration line; render path docs/carry-your-invariant.md"
  verifying_test: "HS-P0020's narrative REQUIRED step (registration, info string, allowance list, hidden markers) plus `cargo test --locked --workspace --all-features -- --show-output` (xtask/src/main.rs:143-155) executing both doctests; EC-002 fallback xtask/tests/bridge_guard.rs"

- id: AC-003
  criterion: "GIVEN a reader who has just stated their rule in their own words in §2, WHEN they reach `## The guard you would write` — the first code on the page — THEN the fence shows, unhidden, `Tags::from_pairs([...])?` → `QueryItem::new(types, tags)?` → `Query::from_items([...])?` with one item per entity's tag set → `read_decision_model(&store, &query).await?` → a visible fold over the returned events → `AppendCondition::new(query).after_opt(last)`, against a real `happenstance::MemoryEventStore`, importing from `happenstance` and binding `EventStore` (that name only, never `SendEventStore`) — and the guard is tagged to the invariant it must hold, not to the row the command writes."
  satisfied: false
  evidence: ""
  mount_point: "docs/carry-your-invariant.md §5 `## The guard you would write`, compiled through xtask/src/narrative.rs"
  verifying_test: "`cargo test --locked --workspace --all-features -- --show-output` (xtask/src/main.rs:143-155) — the §5 doctest compiles and runs green; tier 5 confirms it is the first code in reading order"

- id: AC-004
  criterion: "GIVEN a reader whose stated fear is a model that \"looks right, compiles, runs, and is quietly wrong\", WHEN they read `## What a type-only guard misses` — strictly after §5, marked as wrong at its start and at its end, and never the last code on the page — THEN they meet one fence differing from §5's by one expression: the guard's query tagged to what the command writes rather than to the invariant it holds, so the conflicting event never enters the query, the append is accepted, and the fence asserts exactly that (`is_ok()`) rather than asserting that the code is bad. The page's normative sentence there is a citation to CF-8 ([FROZEN]: a condition carrying a tag no stored event carries MUST NOT reject the append) with CF-7 named as the mirror over-refusal this fence is deliberately not, and ES-27 for why the boundary is drawn on tags at all. No `compile_fail`: the wrong guard compiles, and teaching that the compiler catches this is the false comfort the criterion exists to refuse."
  satisfied: false
  evidence: ""
  mount_point: "docs/carry-your-invariant.md §6 (the wrong-side contrast), compiled through xtask/src/narrative.rs"
  verifying_test: "`cargo test --locked --workspace --all-features -- --show-output` (xtask/src/main.rs:143-155) — the §6 doctest's is_ok() assertion passes; `cargo xtask spec-trace` (xtask/src/main.rs:315-327) resolves CF-8, CF-7 and ES-27; tier 5 checks the two markers and the §5→§6→§7 ordering"

- id: AC-005
  criterion: "GIVEN a reader arriving with a one-stream-per-entity model and the reflex question which stream does this go in?, WHEN they reach `## Where your streams went`, THEN the prior model is named and retired there in three or four sentences at the stable slug `#where-your-streams-went` — and the words \"aggregate\", \"your aggregates\", \"one stream per entity\" and \"which stream\" appear on this page and nowhere else in this project's output. Every other page links to that heading rather than re-arguing the decision, and the heading ships at 23 characters because the 22-character budget's mechanism (rustdoc's 200px sidebar TOC) does not exist on this markdown surface."
  satisfied: false
  evidence: ""
  mount_point: "docs/carry-your-invariant.md §3 `## Where your streams went`, slug #where-your-streams-went"
  verifying_test: "Tier 5 — the DT-1 consistency walk owned set-wide by .bklg/docs-that-teach/application-author-path/answered-need-and-anchor-review, to which this story supplies the single recorded location; `cargo doc --locked --workspace --all-features --no-deps --document-private-items` with RUSTDOCFLAGS=-D warnings (xtask/src/main.rs:290-301) fails an unresolved inbound intra-doc link"

- id: AC-006
  criterion: "GIVEN a reader who needs to see the shift rather than be told it happened, WHEN they read `## Tag, query, fold, guard`, THEN the four steps are named tag, query, fold, guard and those same four words are used identically everywhere they appear on the page, and the mapping table sits directly beneath the narration (never beside it) with exactly three columns in this order — Your words · This library's words · Where you saw it. No diagram, chart or image ships from this story, and the implementation report records that AC-013 was discharged by narration rather than leaving the disposition to silence."
  satisfied: false
  evidence: ""
  mount_point: "docs/carry-your-invariant.md §4 `## Tag, query, fold, guard` and the three-column mapping table beneath it"
  verifying_test: "Tier 5 reviewer check against .bklg/docs-that-teach/application-author-path/_design.md:174-204,481-485,592-595, recorded in the story's implementation-report.md as the AC-013 disposition (project.md:272-273, _storymap.md:120)"

- id: AC-007
  criterion: "GIVEN a reader who bounces off the first fence and needs to know what the page is for before they invest, WHEN they look at the top of the page, THEN the first blockquote line, immediately after the H1 and before any other block element, reads `> **Answers:** `explanation` — <the reader's question>?` in HS-P0021's exact form; exactly one such declaration exists on the page and it is above the first fence in reading order; the token is `explanation` from the closed `NEEDS` set (`how-to` is the alternative and it lost, because under it the DT-1 anchor and the wrong-side contrast read as a second need). AND every fence on the page imports from `happenstance`, never `happenstance_core` — the reader is taught one vocabulary and meets it everywhere this story controls."
  satisfied: false
  evidence: ""
  mount_point: "docs/carry-your-invariant.md — the answered-need blockquote immediately after the H1; both fences' use lines"
  verifying_test: "HS-P0021's declaration check where it is mounted, plus the tier-5 answered-need walk (project DoD item 7) owned by .bklg/docs-that-teach/application-author-path/answered-need-and-anchor-review; the import half is compiled by the narrative REQUIRED step and grep-checkable against anti-pattern 13 (_design.md:871-874)"

- id: AC-008
  criterion: "GIVEN a reader on a 1024×768 window who scans before they read, WHEN they scroll the rendered page, THEN no fence scrolls horizontally (fence lines ≤ 68 columns, inside HS-P0020's ≤ 80), the H1 is ≤ 40 characters, prose source wraps at ≤ 90, no paragraph exceeds 435 characters, the mapping table is three columns, and nothing this story authored is behind a control: no `details`, `summary`, tabs, accordion, banner, badge, button, CSS or JS, and no `HIDDEN_MARKERS` token anywhere under the tree. No `#`-prefixed doctest line carries a `Query`, `QueryItem`, `Tags`, `Guard`, `AppendCondition`, the append call, the read call or any assertion — the reader rebuilds both guards from the rendered page alone. One `h1`, no skipped heading levels, link text meaningful in isolation. The enumeration of interactive affordances this story introduced is empty."
  satisfied: false
  evidence: ""
  mount_point: "docs/carry-your-invariant.md as rendered under the pinned TREE = \"docs\", registered from xtask/src/narrative.rs"
  verifying_test: "HS-P0020's narrative REQUIRED step rejects any HIDDEN_MARKERS token and any unrecognised info string (checked-documentation-surface/_design.md:92-96,550-568); widths, heading length, paragraph length, the hidden-line rule and the empty affordance enumeration are the tier-5 walk against _design.md:524-528,552-617 and anti-patterns 3, 4, 8, recorded in the story's implementation-report.md"
```
