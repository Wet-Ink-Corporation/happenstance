---
item: "HS-S0150"
stage: implement
created: "2026-08-17"
updated: "2026-08-17"
---

# Acceptance ledger — The page-need checker, mounted as an ordinary gate step

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Four standing obligations specific to this story, all stated in `spec.md` and none a licence to
soften a row:

- **Paste output, never a claim about it.** AC-001, AC-007, AC-010, AC-011 and AC-012 all rest on
  captured terminal or `git`/`rg` output. `RUNBOOK.md:920-925` is the incident that makes this
  non-negotiable: a document vouching for a check is not evidence the check runs.
- **AC-010 requires a *measured* number.** Record the longest problem line the real run produced,
  in characters, against the ≤ 100-character budget and `_design.md` `## Mock` finding 1 (112). The
  budget is knowingly breachable; shortening a citation to hit it is the defect, not the fix.
- **If the pages tree is empty at this merge, AC-002's vacuity guard is *expected* to fail the run**
  (`spec.md` EC-004; Testing brief AC-006). Record the failure as the correct behaviour; do not
  soften the guard to a warning.
- **If `xtask/src/narrative.rs` or its `TREE` const is absent, stop and report** (`spec.md` EC-005).
  Do not declare a local `PAGE_DIR`, do not stub the pages half, do not `#[cfg]` it away. That is a
  project-dependency escalation, not an implementation choice.

```yaml
- id: AC-001
  criterion: "**The gate actually runs it, by all three routes.** GIVEN a page author whose `cargo xtask ci` is the last thing between a half-shaped page and `main`, WHEN they run the full gate, the lint family alone, or the story-grain selector, THEN a step named `every page declares one need` runs in all three — as an ordinary `REQUIRED` entry with `probe: None` and `--locked` in its args, printed by `print_help()`, named in `lint_steps()`, and invoked *unconditionally* by `affected::run` before any package selection — so a checker that exists in `xtask/src/` but is unreachable is impossible to ship."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — `REQUIRED` at :105 (one `Step`, `probe: None`), dispatch at :689-700, `print_help()` at :718, `lint_steps()` at :799-808 (guarded by `steps_named`'s panic at :816-826); plus the unconditional file-reading list at xtask/src/affected.rs:118-125"
  verifying_test: "gate-integration, captured verbatim: `cargo run --locked -p xtask -- lints` (step name present), `cargo xtask ci` (step named among the mandatory steps), `cargo xtask affected --base main` (step printed under `=== the file-reading checks ===`)"

- id: AC-002
  criterion: "**An emptied or moved tree fails the run; it never passes it.** GIVEN the application author whose real fear is a check that reports green over nothing, WHEN either pinned tree is absent, THEN the run fails with a `.with_context()` naming the expected path; and WHEN either exists but holds no files, THEN the run `bail!`s in the `lint_constitution.rs:176` spelling — *`{DIR}` holds no …, so every check below is vacuous* — and never prints `0 pages, all consistent`. Both guards, on **both** trees."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lint_pages.rs — the two directory readers inside `pub(crate) fn run(mode: Mode)`, reached from xtask/src/main.rs:105 on every `cargo xtask ci`"
  verifying_test: "static: four `#[cfg(test)]` tests in xtask/src/lint_pages.rs over fabricated roots under `std::env::temp_dir()` (missing and empty x rules tree and pages tree), no `tempfile` dependency; run by `cargo test -p xtask`"

- id: AC-003
  criterion: "**Moving the pages tree is one edit, not two.** GIVEN a maintainer who relocates HS-P0020's narrative tree six months from now, WHEN they change its `const`, THEN this checker follows without a second edit, because it reads `xtask::narrative::TREE` (made `pub(crate)` here) and declares no `PAGE_DIR` of its own — the *three lists that must agree* defect `xtask/src/spec_trace.rs:122-160` records, foreclosed rather than documented."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/narrative.rs — HS-P0020's `TREE` const, made `pub(crate)` (one token) and referenced from xtask/src/lint_pages.rs"
  verifying_test: "static + procedural: a `cargo test -p xtask` test asserting the checker's pages root resolves from `narrative::TREE`; plus captured `rg -n 'narrative::TREE' xtask/src/lint_pages.rs` (one reference) and a search for any second string literal naming the pages tree in `xtask/src/` (no hits)"

- id: AC-004
  criterion: "**A page that quietly answers two needs stops being invisible.** GIVEN a page author who wrote something that reads as complete and is not, WHEN the gate runs, THEN zero declarations is one problem naming the page and the band-00 atom to read; two declarations is one problem **at the line of the second one**, naming both tokens; and an unenumerated token — `reference` included — is one problem at the declaration's line naming the offending token *and* the enumerated set; and prose elsewhere on the page that merely mentions a need word is never counted as a declaration."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lint_pages.rs — the pure line-carrying declaration parser and the zero/two/unenumerated checks, reached from xtask/src/main.rs:105"
  verifying_test: "static: `cargo test -p xtask` parser tests over synthetic `&str` literals — zero, one, two, one-plus-mentioning-prose (must not false-positive), and a malformed `> **Answers:**` line (EC-010); each asserts the returned 1-based line number, mirroring `rules_are_split_at_the_next_heading` at xtask/src/lint_constitution.rs:871-877"

- id: AC-005
  criterion: "**The rule can reject something, and it stays able to.** GIVEN a reviewer asking whether this check is decorative, WHEN they run the test suite, THEN three named wrong pages that could plausibly ship — two declarations, none, one unenumerated — each produce a problem naming the correct line number *within the literal*, and none of the three is a committed file, so the rejection is permanent regression coverage rather than a one-off observation."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lint_pages.rs — `#[cfg(test)] mod tests`, the same synthetic-input convention xtask/src/affected.rs:640-705 already uses"
  verifying_test: "static: three named `#[cfg(test)]` tests in xtask/src/lint_pages.rs run by `cargo test -p xtask`; plus captured `git diff main --stat` showing no committed broken page"

- id: AC-006
  criterion: "**Routing pages cannot breed.** GIVEN the evaluator whose gap is *nowhere to go after the second question*, WHEN a second `orientation` page appears at one directory level, THEN the run reports one problem naming that directory and every offending `path:line` — RP-10-3, the ceiling that keeps DT-3's new category from becoming the sink option (a) was chosen to avoid."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lint_pages.rs — the per-directory `orientation` counter over the parser's output, reached from xtask/src/main.rs:105"
  verifying_test: "static: a `cargo test -p xtask` test over synthetic (path, declaration, line) triples — one directory holding two `orientation` pages fails and names both paths; two directories holding one each pass"

- id: AC-007
  criterion: "**The router's index cannot fall behind the corpus it indexes.** GIVEN the next page author who must load one rule rather than the tree, WHEN a rule atom lands without its router row, THEN `cargo xtask ci` fails with the disagreement and the repair instruction `cargo xtask lint-pages --write` **inside** the problem line; `--write` rewrites only the region between the markers; the **first** `--write` against the router committed by `router-precedence-and-announcement` produces **no diff**; and every `.md` the router links resolves. The complete index stays present alongside the filter — never behind a toggle, never a partial list."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/README.md — the `discipline-router` surface's generated region between `<!-- BEGIN GENERATED -->` and `<!-- END GENERATED -->`, written only by `Mode::Write` in xtask/src/lint_pages.rs and reached by the dispatch `--write` arm at xtask/src/main.rs:689-700"
  verifying_test: "static + gate-integration: `cargo test -p xtask` — a synthetic router-vs-atom-set disagreeing by exactly one atom (fails and names it), `Mode::Write` regenerating to equality, and a synthetic router with one dangling link reporting a problem; plus captured `cargo run --locked -p xtask -- lint-pages --write && git diff --exit-code standards/pages/README.md`"

- id: AC-008
  criterion: "**`NEEDS` and band 10 cannot disagree in silence.** GIVEN a maintainer widening or narrowing the need set, WHEN they change the `const` without `standards/pages/10-the-need-set.md` or the reverse, THEN the run reports a problem naming **which token moved**, citing both paths, and stating in the message that this one is not `--write`-repairable — the atom holds a prose column no `const` can generate."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lint_pages.rs — the containment-and-exclusion check over `standards/pages/10-the-need-set.md`, reached from xtask/src/main.rs:105"
  verifying_test: "static: two `cargo test -p xtask` tests over synthetic const/table pairs — a `NEEDS` member missing from the atom (containment) and a need-shaped backticked token in the atom that is not in `NEEDS` (exclusion) — each asserting the offending token appears in the message (RS-81-5, standards/rust/81-checks-that-cannot-be-types.md:335)"

- id: AC-009
  criterion: "**A rule atom that has drifted is caught at its own line.** GIVEN a reader who must pay for every byte of what they load, WHEN an atom loses one of the five fixed sections, carries no `## RP-` rule at all, exceeds 6 rules or 16,384 bytes, or contains a `rust`-tagged or untagged fence, THEN each is one problem at `path:line`; the byte-ceiling message carries the measured byte count; and the fence message says *why* — nothing in this workspace compiles this tree, so a `rust` fence is a Rust claim no check backs."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lint_pages.rs — the rules-tree shape checks (a `SECTIONS` analogue, the two ceilings, the fence rule, the router link check), reached from xtask/src/main.rs:105"
  verifying_test: "static: `cargo test -p xtask` — one test per rejection over synthetic atom strings, mirroring `check_shape` at xtask/src/lint_constitution.rs:477-508; the byte test asserts the measured number appears in the message; the fence tests assert `text` and `markdown` pass while `rust` and untagged fail (design anti-pattern 13)"

- id: AC-010
  criterion: "**The terminal tells the author everything, in one run, in reading order.** GIVEN the author who just broke the rule and will read nothing but the terminal, WHEN a run finds three problems in three files, THEN the output is one block **sorted by path then line**, each line `  {path}:{line} — {what is wrong}; {why it matters, or what to do}` with the repair *inside* the line — no per-problem heading, no blank lines between problems, no summary section, no `next steps` paragraph, no box drawing, no spinner or per-file progress, nothing truncated and no `and others` — closed by `bail!(\"{n} problem(s) in …\")`; and a zero-problem run prints `  {n} pages, {m} rules, all consistent` rather than nothing."
  satisfied: false
  evidence: ""
  mount_point: "the `lint-terminal-output` surface — stdout/stderr of `cargo run --locked --quiet -p xtask -- lint-pages`, also reached by `cargo xtask ci` (xtask/src/main.rs:105) and `cargo xtask lints` (:799-808); formatted in xtask/src/lint_pages.rs's single problem-printing path"
  verifying_test: "static + procedural: `cargo test -p xtask` — a sort-stability test (same problems, shuffled input order, identical output, ties broken on message text per EC-011) and a line-format test asserting `path:line — ` is each line's prefix; plus the verbatim `green`, `single-problem` and `many-problems` captures read against _design.md `## Composition` S4, `## Transience policy`, `## Hierarchy` and `## Density budget`, and the **measured longest problem line** recorded in characters"

- id: AC-011
  criterion: "**A prose-only pull request reads the prose, and nothing else.** GIVEN a contributor whose diff touches only `standards/pages/`, WHEN they run `cargo xtask affected --base main`, THEN the page-need checker runs unconditionally *and* `standards/pages/` selects no package — the pair landed in one change — while `standards/rust/` still selects `xtask`, so no lazily broadened `standards/` prefix can silently un-compile the constitution."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/affected.rs — the unconditional file-reading list at :118-125 and `is_inert`'s `INERT` at :249-266; the arm at :214-221 is deliberately NOT broadened"
  verifying_test: "static + gate-integration: two tests in xtask/src/affected.rs's own `mod tests`, mirroring `the_relocated_trees_stay_inert` (:662-673) and `a_constitution_atom_selects_xtask` (:688-696); plus a captured `cargo xtask affected --base main` on a rules-tree-only diff showing the checker ran and no package was selected"

- id: AC-012
  criterion: "**The instrument states its own blind spot before it states its result.** GIVEN the adapter author who must tell a limit of the tool from a gap in their understanding, WHEN they open `xtask/src/lint_pages.rs`, THEN its first section says — unhedged and first — that the check proves a need is **declared** and never that the page **answers** it, followed by the other five limits and the three divergences from `lint_constitution` including the stated **absence** of a `check_harness` equivalent; the scaffold `#![allow(dead_code, reason = …)]` naming this story is deleted; and `docs/README.md:25-29`'s `[PROVISIONAL — settles at page-need-checker-mounted-in-the-gate]` marker is removed in the same commit that makes the sentence true."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lint_pages.rs — the module-level `//!` docs (limits first, in the shape of xtask/src/lint_constitution.rs:9-28); docs/README.md:25-29 — the gate-read-trees paragraph in the repository index"
  verifying_test: "procedural + gate-integration: a field-by-field read of the module docs against Architecture brief Note 7's six items and Notes 4/5/6's three divergences, each cited by `xtask/src/lint_pages.rs:<line>`; `cargo xtask ci`'s clippy step (`-D warnings`, xtask/src/main.rs:116-127) green with the `dead_code` allow deleted; and captured `rg -n 'PROVISIONAL' docs/README.md` returning nothing"
```
