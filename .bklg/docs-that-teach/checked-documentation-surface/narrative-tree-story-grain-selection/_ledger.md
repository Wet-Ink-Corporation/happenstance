---
item: HS-S0137
stage: implement
created: 2026-08-17
updated: 2026-08-17
---

# Acceptance ledger — A prose-only change selects the package that compiles it

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
  criterion: "GIVEN a contributor whose whole change is one page under the narrative tree (docs/…, _design.md D1), WHEN they run the story-grain gate — cargo xtask affected --base main, the command .redkiln/config.yaml:40 runs at every advance seam — THEN the mapping names xtask, the package whose lib target compiles that page's fences, so the gate builds and tests the one package the change could have broken. No new subcommand, flag or prompt is introduced: the contributor reaches this through the invocation already in print_help (xtask/src/main.rs:729), and the same result is reachable in CI where there is no terminal at all."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/affected.rs — the outside-every-member arm of affected_packages (:209-225), reached in production by affected::run (:112) via the Some(\"affected\") dispatch at xtask/src/main.rs:653-658 and invoked by .redkiln/config.yaml:40"
  verifying_test: "xtask/src/affected.rs :: tests::a_narrative_page_selects_xtask (cargo test -p xtask)"

- id: AC-002
  criterion: "GIVEN a contributor editing prose that is not in the narrative tree — references/, spec/, experiments/, .bklg/, .kb/, or a top-level file such as RUNBOOK.md — WHEN they run the same command, THEN no package is selected, because the new arm is a rule about one directory and not about every tree of markdown; and GIVEN a path this module has never heard of, the selection still widens to every member rather than narrowing to nothing, so the module's stated posture (:12-27, \"naming too few reports green over an untested regression\") survives the change. The existing test that used to stand for docs/ is renamed to what it actually asserts, not deleted — it is one of the two anchors of that posture."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/affected.rs — the outside-every-member arm of affected_packages (:209-225) and its fall-through to is_inert (:222-224)"
  verifying_test: "xtask/src/affected.rs :: tests::the_narrative_arm_does_not_widen_to_all_prose and tests::a_top_level_prose_file_selects_nothing (renamed from a_docs_only_change_selects_nothing, :650-654), with tests::the_relocated_trees_stay_inert (:660-675), tests::an_unrecognised_path_widens_rather_than_narrows (:645-649), tests::the_readme_selects_xtask (:676-680), tests::a_constitution_atom_selects_xtask (:688-696) and tests::the_lockfile_selects_everything (:636-640) still green (cargo test -p xtask)"

- id: AC-003
  criterion: "GIVEN a reviewer reading is_inert six months from now to decide whether a tree is checked, WHEN they look for the narrative tree, THEN they find \"docs/\" absent from INERT — removed, not left shadowed behind an earlier else if — with the doc comment stating that it is deliberately absent and why, in the same shape as the standards/rust/ paragraph at :245-247 (\"Adding it here would silently un-compile the corpus\"). A prefix left on the list but unreachable is a prefix the next person to reorder that chain re-arms; that is this AC's named wrong implementation."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/affected.rs — is_inert's INERT list (:248-260) and its doc comment (:232-247)"
  verifying_test: "xtask/src/affected.rs :: tests::the_narrative_tree_is_no_longer_inert (cargo test -p xtask), plus rg -n '\"docs/\"' xtask/src/affected.rs returning the arm's literal only"

- id: AC-004
  criterion: "GIVEN a reviewer who cannot run the gate and must judge the mapping by reading it — the only instrument available, since a wrong mapping returns too few names and that is invisible from a green run (:183-190) — WHEN they read the module, THEN every comment the change falsified says something true: the tree list at :210-213 no longer claims docs/ reaches no package; is_inert's doc comment (:232-247) covers the new case; the empty-selection branch's justification at :136-138 no longer rests on \"a docs-only story genuinely has no package to compile\"; and the new arm states, in the \"what this does not verify\" register of xtask/src/lint_constitution.rs:9-28, that selection is not compilation — naming xtask means the package is built and tested, not that any fence compiled. Nothing added anywhere claims the surface proves a page teaches (project.md DoD items 7 and 8)."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/affected.rs — the arm's comment (:209-221), is_inert's doc comment (:232-247) and the empty-selection branch's justification in affected::run (:135-142)"
  verifying_test: "cargo xtask lint-constitution and cargo test -p xtask --doc green (project.md DoD-7), with rg -n '\"docs/\"' xtask/src/affected.rs as the mechanical half; the statement itself is reviewed against the four sites per RS-81-1 (standards/rust/81-checks-that-cannot-be-types.md:11)"

- id: AC-005
  criterion: "GIVEN a reviewer reading a CI log for a prose-only pull request, WHEN cargo xtask affected --base main runs against a change confined to the narrative tree, THEN the recorded output shows === affected packages ===, the {n} file(s) changed against `main` line, and xtask on its own two-space-indented line — and does not show no package affected — nothing to compile — followed by the fmt, clippy and test steps actually running and affected gate passed as the single closing summary line. The block's composition is the one that already exists at :130-146: nothing is added to it and nothing is hidden by the addition. Per _design.md ## Transience policy, the banner is persistent chrome and the summary is one line; per ## Density budget, the terminal surface is 80 columns and no list is ever truncated; per ## Hierarchy, distinction is carried by position and indent only — no colour, weight, tick, badge or \"verified\" mark, and no progress ticker or per-path line (## States, Loading). The evidence is the parsed output naming the package, because a zero exit status is evidence of nothing (RS-81-4)."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/affected.rs — the === affected packages === banner and package list in affected::run (:130-146), reached via the Some(\"affected\") dispatch at xtask/src/main.rs:653-658 and invoked by .redkiln/config.yaml:40"
  verifying_test: "cargo xtask affected --base main run on a tree-confined working tree, transcript recorded verbatim at .bklg/docs-that-teach/checked-documentation-surface/narrative-tree-story-grain-selection/_observed-affected-run.md; plus cargo xtask ci --fast green (project.md DoD-6)"
```
