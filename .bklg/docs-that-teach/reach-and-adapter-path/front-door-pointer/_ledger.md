---
item: HS-S0158
stage: implement
created: "2026-08-17"
updated: "2026-08-17"
---

# Acceptance ledger — The front door points outward, on both surfaces

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
  criterion: "GIVEN an evaluator who has just run `cargo add happenstance` and opens the crate's rustdoc root inside a bounded reading budget, WHEN they read the first screen at 1024x768 without scrolling, THEN one sentence of plain pre-heading prose — composed from rustdoc's own top-blurb primitive, with no bold, no callout, no emoji and no new heading — names guide-level material and where it is, rendered within the first 5 rendered lines and above `# Status: a facade over [happenstance_core]`, as persistent chrome no reader has to expand."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs — composition root CR-1, the crate-root `//!` doc rendered by rustdoc (region 2 of _design.md's composition table: after :11, before :13)"
  verifying_test: "cargo doc -p happenstance --no-deps, then target/doc/happenstance/index.html at `#main-content > .docblock` compared against the crate-root-front-door / first-screen-1024x768 frame in .bklg/docs-that-teach/reach-and-adapter-path/design/mock.html; static half `rg -n \"Guide-level documentation\" crates/happenstance/src/lib.rs` returns exactly one hit between :11 and :13"
- id: AC-002
  criterion: "GIVEN a crates.io reader who never sees the rustdoc, WHEN they open `crates/happenstance/README.md` as crates.io or GitHub renders it, THEN the same authoritative sentence — byte-identical rendered text, carrying the pinned substring `Guide-level documentation` exactly once in each of the two files — appears as its own paragraph between the description and the status blockquote, outside that blockquote and not as a second one."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md, mounted into this crate through `#![cfg_attr(doctest, doc = include_str!(\"../README.md\"))]` at crates/happenstance/src/lib.rs:10 (CR-1)"
  verifying_test: "`rg -c \"Guide-level documentation\" crates/happenstance/README.md crates/happenstance/src/lib.rs` returns 1 for each; the sentence extracted from each file (with `//! ` stripped) diffs clean; `rg -n \"^> \" crates/happenstance/README.md` shows the blockquote still one contiguous block with the pointer outside it"
- id: AC-003
  criterion: "GIVEN a reader who came to the README for the \"Which crate do I want?\" answer or the compiled example, WHEN the pointer is installed on both surfaces, THEN nothing pre-existing is reworded, folded, collapsed or reflowed; no line moves down by more than 3 source lines; no line other than the two additions changes; the README's compiled Rust fence opens no later than source line 35 (33 measured); and if the budget is threatened, the pointer's own sentence shortens toward 20 words rather than anything pre-existing moving."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs and crates/happenstance/README.md — the diff itself is the surface; project DoD item 7 makes the HS-P0016 seam checkable by diff rather than by assertion"
  verifying_test: "`git diff -U0 <merge-base> -- crates/happenstance/README.md crates/happenstance/src/lib.rs` shows added lines only (zero modified, zero deleted); rg for the opening Rust fence in crates/happenstance/README.md reports a line number <= 35; reviewed against the readme-front-door / diff-against-main frame in .bklg/docs-that-teach/reach-and-adapter-path/design/mock.html"
- id: AC-004
  criterion: "GIVEN the evaluator who has read the sentence and decides to follow it, WHEN they activate the link, THEN it lands on real guide-level material through the highest available rung of the href ladder (intra-doc link, else an in-tree markdown link inside HS-P0020's pinned tree, else the named-but-unlinked form live at `crates/happenstance-core/src/store.rs:77`); the visible link text is a self-describing noun phrase of at least three words and never \"here\", \"this\", \"docs\" or a bare URL; and if only the fourth rung (nothing) is available the pointer is not installed — no placeholder, no \"coming soon\" — and the story blocks and escalates."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs crate-root doc and crates/happenstance/README.md — the href, resolved through the gate's three rustdoc builds under `rustdoc::broken_intra_doc_links = \"deny\"` (Cargo.toml:134)"
  verifying_test: "cargo xtask ci --fast — the `documentation`, `documentation (no default features)` (xtask/src/main.rs:502) and `docs.rs configuration (nightly)` steps; plus `rg -n \"https?://\"` over the two added lines returning nothing, and the link text read against _design.md's `## Density budget`, \"Minimum legible size\""
- id: AC-005
  criterion: "GIVEN a maintainer who must know, a year from now, that this pointer still resolves, WHEN the change merges, THEN rows P1 and P2 are present in the pointer register landed by `pointer-policy-and-inventory` — build-time data, never a page a reader can navigate to — each naming its surface, its destination, its N-3 form, the ladder rung chosen, and a non-empty guard, with P2's mirror-assertion guard recorded honestly as not yet landed if that is the truth at merge."
  satisfied: false
  evidence: ""
  mount_point: "the pointer register landed by pointer-policy-and-inventory (HS-S0154) — build-time data in the shape of `SUMMARIES` at xtask/src/lint_constitution.rs:64; its path is bound by that story and is deliberately absent from this spec's PR-boundary fence until it is known"
  verifying_test: "read the register file HS-S0154 landed and assert two rows whose surface fields are `crates/happenstance/src/lib.rs` and `crates/happenstance/README.md`, each with a non-empty guard cell; if HS-S0154 landed the checker it runs (precedent: `check_summaries` at xtask/src/lint_constitution.rs:463-473); `git diff --name-only` shows no new rendered index page (anti-pattern 14)"
- id: AC-006
  criterion: "GIVEN a keyboard-only or screen-reader reader at 1024x768, WHEN they reach either front-door surface, THEN the pointer is a plain link reachable by Tab with no widget, no \"See also\" block, no table, no raw HTML and no inline `style=`; nothing added carries meaning in colour or position; the existing heading ladder (`crates/happenstance/src/lib.rs:13,27,53`) is unskipped and unchanged; everything added reflows without clipping; and a reader who never follows the link is left exactly as unblocked as before, because the pointer is an offer and displaces no in-place answer."
  satisfied: false
  evidence: ""
  mount_point: "the rendered surfaces of crates/happenstance/src/lib.rs (target/doc/happenstance/index.html) and crates/happenstance/README.md, at 1024x768"
  verifying_test: "recorded keyboard-only pass at 1024x768 over the built cargo doc output and the rendered README (no linter exists — _design.md's `## Density budget`, \"Two named gaps\" (2), states this); static half `rg -n \"<[a-z]+|style=\"` over the two added lines returns nothing and `rg -n \"^//! #\" crates/happenstance/src/lib.rs` shows :13, :27, :53 unchanged in level and order; reviewed against _design.md's `## States` narrow-viewport and screen-reader rows"
- id: AC-007
  criterion: "GIVEN the repository's merge gate, WHEN `cargo xtask ci --fast` runs on the merged result, THEN it is green with the pointer in place — every rustdoc step passes under `broken_intra_doc_links = \"deny\"`, `cargo test` still compiles the README's fences through the doctest mount at `crates/happenstance/src/lib.rs:10`, `cargo xtask spec-trace` stays green, and the diff shows no change to any public item, signature, feature, manifest entry or `#[doc(alias)]` attribute."
  satisfied: false
  evidence: ""
  mount_point: "the repository merge gate defined once in xtask/src/main.rs, run over the merged tree containing crates/happenstance/src/lib.rs and crates/happenstance/README.md"
  verifying_test: "cargo xtask ci --fast; cargo xtask affected --base main; cargo test -p happenstance --doc; cargo xtask spec-trace; and `git diff` confirming no `pub ` item, no `[features]` entry, no Cargo.toml change and no `doc(alias)` line added"
```
