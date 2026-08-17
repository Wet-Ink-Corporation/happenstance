---
item: HS-S0188
stage: implement
created: 2026-08-17T13:16:35.851Z
updated: 2026-08-17T13:16:35.851Z
---

# Acceptance ledger — The worked example surfaced in its own words

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: "GIVEN the application author has finished the bridge and opens the worked example's rendered documentation, WHEN they read its opening explanation, THEN they meet the example author's own bytes and not a summary: examples/course-subscriptions/src/overview.md exists and is the only copy of that explanation in the tree, examples/course-subscriptions/src/main.rs opens with #![doc = include_str!(\"overview.md\")] and carries no //! module-doc block, cargo doc -p course-subscriptions renders the doc in the same position and with the same content as before the move, and no assertion, import, handler or Cargo.toml line of the example changed."
  satisfied: false
  evidence: ""
  mount_point: "examples/course-subscriptions/src/main.rs — the second render path, #![doc = include_str!(\"overview.md\")]"
  verifying_test: "examples/course-subscriptions/tests/reach.rs::overview_is_the_only_copy; plus the \"documentation\" REQUIRED step (cargo doc … RUSTDOCFLAGS=-D warnings, xtask/src/main.rs:290-301)"
- id: AC-002
  criterion: "GIVEN a reader following the pinned narrative tree, WHEN they open docs/read-the-worked-example.md, THEN they are on a checked page and not an orphan file: the page exists at that route, xtask/src/narrative.rs carries exactly one #[cfg(doctest)] mod read_the_worked_example { #![doc = include_str!(\"../../docs/read-the-worked-example.md\")] } — one module, one included file — and HS-P0020's checker reports zero unregistered pages for the tree."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/narrative.rs — the harness that makes a markdown file a checked page, reached through the REQUIRED narrative steps mounted in xtask/src/main.rs"
  verifying_test: "cargo test -p xtask --doc collecting xtask::narrative::read_the_worked_example; examples/course-subscriptions/tests/reach.rs::page_is_registered"
- id: AC-003
  criterion: "GIVEN the author lands on this page cold from a search result rather than from the bridge, WHEN the first screen renders, THEN they can tell in one glance what question this page answers and that it will send them elsewhere: a single blockquote sits directly under the H1 and above every other element, in HS-P0021's notation and no other — > **Answers:** `orientation` — <question>? — carrying a token from the closed NEEDS set, a question in the reader's own voice ending in ?, and there is exactly one such declaration on the page."
  satisfied: false
  evidence: ""
  mount_point: "docs/read-the-worked-example.md, registered in xtask/src/narrative.rs"
  verifying_test: "examples/course-subscriptions/tests/reach.rs::exactly_one_answered_need"
- id: AC-004
  criterion: "GIVEN the author is about to spend their attention on someone else's file, WHEN they read this page top to bottom, THEN they know before they leave what they are about to read and why — two sentences of orientation naming three invariants that do not fit an aggregate — and they meet the DT-1 anchor before the outbound link, as one link to the bridge's ## Where your streams went and not as a second account of the prior model, because a reader who follows the link does not come back. This is the Handed off state (_decomposition.md:106-108)."
  satisfied: false
  evidence: ""
  mount_point: "docs/read-the-worked-example.md, registered in xtask/src/narrative.rs; anchor target docs/carry-your-invariant.md"
  verifying_test: "examples/course-subscriptions/tests/reach.rs::orientation_precedes_departure"
- id: AC-005
  criterion: "GIVEN the author has just been taught to write a Query and an AppendCondition by hand on the bridge, WHEN they follow the link and find neither in the example, THEN they were warned: exactly one sentence, immediately before the link and nowhere else, names the seam as it exists post-merge — the example is the same boundary one layer up, where happenstance::commit derives both from a DecisionModel — states it as a claim about what the reader will look for, names no crate the merged example does not import, and restates no sentence of the example's own '# What is not in this file, and used to be' section."
  satisfied: false
  evidence: ""
  mount_point: "docs/read-the-worked-example.md, registered in xtask/src/narrative.rs"
  verifying_test: "examples/course-subscriptions/tests/reach.rs::seam_named_once_before_the_link"
- id: AC-006
  criterion: "GIVEN the author is mid-bridge, WHEN they act on A5, THEN the good example is one hop away and stays that way: docs/carry-your-invariant.md carries one link to this page, this page links to overview.md first and to main.rs last, every link's text is meaningful standing alone (never 'here'/'this'), and a repository check fails by name if any link in that chain is removed, if a link's target file does not exist, or if a pasted copy of the explanation's opening sentence appears on the page."
  satisfied: false
  evidence: ""
  mount_point: "docs/carry-your-invariant.md (the slice-mate's bridge page) → docs/read-the-worked-example.md → examples/course-subscriptions/src/overview.md"
  verifying_test: "examples/course-subscriptions/tests/reach.rs::the_chain_holds, swept by the \"tests\" REQUIRED step (xtask/src/main.rs:143-155)"
- id: AC-007
  criterion: "GIVEN the author reads on a 1024x768 laptop, on a phone, in print, or with JavaScript off, WHEN the page renders, THEN nothing is hidden from them and nothing overflows: the page introduces zero interactive affordances, zero Rust fences, zero images or diagrams, and no <details>/tab/admonition marker; it holds the substrate's budgets — repo-relative path <= 32 characters (docs/read-the-worked-example.md is 31), H1 <= 40 characters, page <= 250 source lines, prose source wrapped <= 90 columns, every ## heading <= 22 characters so the sidebar TOC does not clip it with an ellipsis; and its hierarchy is the design's — the destination is primary, the orientation secondary above it, the seam sentence recessive as one sentence and never promoted to a ###."
  satisfied: false
  evidence: ""
  mount_point: "docs/read-the-worked-example.md, registered in xtask/src/narrative.rs; HS-P0020's HIDDEN_MARKERS rejection at the gate"
  verifying_test: "examples/course-subscriptions/tests/reach.rs::page_holds_its_budgets"
```
