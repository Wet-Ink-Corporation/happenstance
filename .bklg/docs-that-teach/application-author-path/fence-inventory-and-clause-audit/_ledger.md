---
item: "HS-S0189"
stage: implement
created: "2026-08-17T13:16:36.424Z"
updated: "2026-08-17T13:16:36.424Z"
---

# Acceptance ledger — Every fence inventoried and every clause citation audited

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
  criterion: "GIVEN Persona 1 has read this project's material across all four surfaces and is deciding whether the code on the page is code that still works, WHEN the reviewer walks .bklg/docs-that-teach/application-author-path/fence-inventory-and-clause-audit/_inventory.md § Fences, THEN every fenced block on crates/happenstance/src/lib.rs, on the three pages in HS-P0020's pinned tree and in examples/course-subscriptions/src/overview.md carries one row with: its info string verbatim, its execution class (executed / compiled-only / prose), the named mechanism that exercises it, whether that mechanism can fail if the fence opts out, and its hidden-line classification read off the rendered page rather than the source — AND the count of rows opted out (ignore, no_run, untagged-treated-as-prose, or named in IGNORE_ALLOWANCES) is zero, with the enumeration taken from _design.md's ## Surfaces block (:45-65) so a designed-but-unauthored page appears as a missing row rather than as an absence nobody counted."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/narrative.rs"
  verifying_test: ".bklg/docs-that-teach/application-author-path/fence-inventory-and-clause-audit/_inventory.md § Fences, re-derived by `cargo xtask ci --fast` (xtask/src/main.rs:143-155 \"tests\" + HS-P0020's two narrative steps) and `cargo doc -p happenstance --no-deps` for the rendered hidden-line read"

- id: AC-002
  criterion: "GIVEN the inventory claims zero fences opted out, WHEN one fence on a page under HS-P0020's tree is retagged `ignore` and the narrative checker is run, THEN it fails with a problem line naming that path:line, and WHEN the edit is reverted the gate returns green; AND WHEN the same retag is applied to the fence in crates/happenstance/src/lib.rs, THEN the gate stays green — the expected asymmetry — and _inventory.md § Drills records that the crate-root fence is protected by review only (_design.md:880-882, anti-pattern 1 at :836-838) with the coverage gap routed to HS-P0020 by name."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/narrative.rs"
  verifying_test: ".bklg/docs-that-teach/application-author-path/fence-inventory-and-clause-audit/_inventory.md § Drills — drills D-1 and D-2, each run via `cargo xtask lints` (.redkiln/config.yaml:48) and `cargo xtask ci --fast`, with the exact edit, the pasted `path:line — message`, the revert and `git status` clean"

- id: AC-003
  criterion: "GIVEN Persona 1 meets a sentence on one of these pages that states a rule they are expected to obey, WHEN they follow its citation, THEN they land on a clause that exists — and _citations.md § Claims carries one row per normative claim with the page and location, the cited clause id, that clause's maturity marker verbatim from spec/SPECIFICATION.md, and the named resolver for that page: HS-P0020's checker via clause_ids(root) for tree pages, and nothing for crates/happenstance/src/lib.rs, recorded as such rather than covered by another page's mechanism. ES-25 and CF-7 are cited as [FROZEN]; VT-30 is cited as [PROVISIONAL] in wording that would not need rewriting if VT-30's shape changed (_design.md:400-404). Every citation is an ordinary inline link in last position — never a tooltip, popover or fold (_design.md:526)."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/narrative.rs"
  verifying_test: ".bklg/docs-that-teach/application-author-path/fence-inventory-and-clause-audit/_citations.md § Claims, re-derived by `cargo xtask spec-trace` (REQUIRED, xtask/src/main.rs:315) plus HS-P0020's `every narrative page is checked` step; line anchors read from merge-forward-preflight/spec.md:261"

- id: AC-004
  criterion: "GIVEN the audit claims every cited clause id resolves, WHEN one cited id on a tree page is retagged to an id spec/SPECIFICATION.md does not define and the checker is run, THEN it fails with `<page>:<line> — cites ES-999, which SPECIFICATION.md does not define`, and reverting returns it to green; AND WHEN the same retag is applied to a citation on the crate root, THEN nothing fails — spec_trace reads only spec/SPECIFICATION.md and a relative markdown link is not an intra-doc link, so broken_intra_doc_links does not see it — and that gap is recorded and routed to HS-P0020 rather than left as an implied coverage."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/narrative.rs"
  verifying_test: ".bklg/docs-that-teach/application-author-path/fence-inventory-and-clause-audit/_citations.md § Drills — drills D-3 and D-4, run via `cargo xtask spec-trace` and `cargo xtask lints`, with the exact edit, the pasted problem line, the revert and `git status` clean"

- id: AC-005
  criterion: "GIVEN Persona 1 could reasonably follow a teaching page's wording instead of the clause it points at, WHEN the restatement audit runs, THEN the mechanical pass finds zero sentences containing MUST or MUST NOT that are not a link to a clause id — anti-pattern 11 (_design.md:867-868) — across all four surfaces, AND a reviewer who did not author the pages applies HS-P0021's written spot check (page-need-discipline/project.md:236-238, non-author-performable per :233-235) and records a per-page verdict that no clause is restated in the page's own words. No second restatement procedure is invented."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/narrative.rs"
  verifying_test: ".bklg/docs-that-teach/application-author-path/fence-inventory-and-clause-audit/_citations.md § Restatement — the pasted MUST/MUST NOT sweep with its command and empty output, plus HS-P0021's spot check walked by a non-author reviewer with a verdict per page (testing brief tier 5, _decomposition.md:486)"

- id: AC-006
  criterion: "GIVEN the four page stories' output is already reviewed content and the design behind it is signed off (_design.md:1039), WHEN this audit finds a defect, THEN it is closed one of exactly two ways and never a third: an in-place one-line repair (a restated MUST becomes a clause-id link; an info string is corrected; a missing #[cfg(doctest)] mod line is added to xtask/src/narrative.rs) after which the page still renders and still satisfies the signed-off composition — #main-content details.top-doc > div.docblock resolves, fences stay within 68 columns and 24 rendered lines (32 on crate-root-encounter alone), ## headings within 22 characters, paragraphs within 435 characters (_design.md:532-620), no fence behind a fold, no control outside rustdoc's own chrome, no `use happenstance_core::` import line, no diagram, every fence's output block non-empty (anti-patterns 1, 2, 3, 8, 13, 14, 15) — OR a routed finding with a named destination and no silent absorption; AND _design.md, project.md, _storymap.md and _decomposition.md are byte-identical at the checkpoint, the tree carries no drill residue, IGNORE_ALLOWANCES names none of this project's fences, and if HS-P0020's two narrative steps are absent from REQUIRED the story halts and routes rather than substituting a hand count (project.md:345-350)."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/narrative.rs"
  verifying_test: "`cargo xtask affected --base main` (.redkiln/config.yaml:40) green plus `cargo xtask ci --fast` (:55); `git diff --stat` empty on the four planning artifacts and `git status` clean; `cargo doc -p happenstance --no-deps` and the REQUIRED `documentation` step's RUSTDOCFLAGS=-D warnings build for the render and bracket checks; .bklg/docs-that-teach/application-author-path/fence-inventory-and-clause-audit/_inventory.md § Routing"
```
