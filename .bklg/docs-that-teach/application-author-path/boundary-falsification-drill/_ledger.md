---
item: HS-S0186
stage: implement
created: 2026-08-17
updated: 2026-08-17
---

# Acceptance ledger — Removing the boundary makes the repository fail

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two mount points appear below and both are real. `xtask/src/narrative.rs` is HS-P0020's `HARNESS`
— the lib-crate registration file whose `include_str!` line makes the step-3 page a doctest the
`"tests"` REQUIRED step executes. `crates/happenstance/src/lib.rs` is the crate-root twin, executed
today by `cargo test -p happenstance --doc`, which is why AC-002…AC-004 have a proof that does not
wait on the pinned tree. Evidence for a page-side row must cite the page mount; evidence citing only
the twin leaves the page half unproven.

```yaml
- id: AC-001
  criterion: "GIVEN an application author who has just run step 3 and watched `AppendError::ConditionViolated` appear in their own output, and who now wants evidence the boundary is load-bearing rather than decorative (UI-2, UX-002), WHEN they read on to the bottom of step 3, THEN they meet a `###` falsification drill — after the clause-citation sentence, at the bottom of that step, on the same page rather than a page of its own — carrying three separately labelled, composed parts in this order: the exact edit, the exact failure to expect, and the exact revert, each stated in operable terms rather than described."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/narrative.rs — the step-3 page registered in HS-P0020's HARNESS, route {tree}/first-encounter/ with {tree} = docs"
  verifying_test: "Tier 5 reviewer walk against .bklg/docs-that-teach/application-author-path/_design.md `## Composition` and `## Hierarchy`, recorded in .bklg/docs-that-teach/application-author-path/boundary-falsification-drill/_drill-observation.md"

- id: AC-002
  criterion: "GIVEN the same reader will only believe a drill whose named check the repository genuinely runs, WHEN the tree is clean and unmodified, THEN the refusal assertion is executed at both mounts — no `no_run`, `ignore` or `compile_fail` attribute on either fence, and zero entries added to HS-P0020's `IGNORE_ALLOWANCES` — and each doctest target reports the refusal test as run and passing."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/narrative.rs (page) and crates/happenstance/src/lib.rs (crate-root twin)"
  verifying_test: "`cargo test -p happenstance --doc -- --show-output` and `cargo test -p xtask --doc -- --show-output`, both swept by the \"tests\" REQUIRED step at xtask/src/main.rs:143-155"

- id: AC-003
  criterion: "GIVEN the reader performs the page's exact edit — passing `None` where the program passes `Some(&AppendCondition::new(seats).after_opt(upto))` — WHEN they run the command the drill names, THEN the run fails on the refusal assertion itself at both mounts, not on a build error and not on a query-constructor error, and the failing command is one `cargo xtask ci` already sweeps: no step is added to `REQUIRED`, no harness is introduced."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/narrative.rs (page) and crates/happenstance/src/lib.rs (crate-root twin)"
  verifying_test: "The drill's failing direction executed for real — `cargo test -p happenstance --doc` and `cargo test -p xtask --doc` under the edit, both failures captured verbatim in .bklg/docs-that-teach/application-author-path/boundary-falsification-drill/_drill-observation.md; plus a diff of xtask/src/main.rs showing the REQUIRED array unchanged"

- id: AC-004
  criterion: "GIVEN the reader has watched the check fail and now wants their tree back, and reversibility is a first-class state rather than a footnote (_design.md `## States`, Boundary restored; UX brief States table, Restored), WHEN they apply the page's exact revert and nothing else, THEN both commands pass again and `git status --porcelain` is empty — nothing else in their tree needs repairing, and no second instruction is required to get there."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/narrative.rs (page) and crates/happenstance/src/lib.rs (crate-root twin)"
  verifying_test: "The drill's recovery direction executed for real — revert, re-run both doctest commands green, `git status --porcelain` empty, all recorded in .bklg/docs-that-teach/application-author-path/boundary-falsification-drill/_drill-observation.md; followed by `cargo xtask affected --base main` green at the story checkpoint"

- id: AC-005
  criterion: "GIVEN initiative DoD-4 is a human-observed obligation (\"run and observed\", .bklg/docs-that-teach/initiative.md:425-428) that no per-commit check can retire, WHEN the implementer performs the drill, THEN a dated transcript in this story's own folder records, in order: the starting tree state, the commands, the edit applied, the verbatim failing output at both mounts, the revert, the verbatim passing output at both mounts, and the closing clean-tree check — because this is the only record DoD-4 and project DoD item 3 will ever have."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/narrative.rs (page) and crates/happenstance/src/lib.rs (crate-root twin) — the two mounts the transcript must cover"
  verifying_test: "Tier 4 human-observed record: .bklg/docs-that-teach/application-author-path/boundary-falsification-drill/_drill-observation.md, walked by the reviewer against project.md DoD item 3 and the testing brief's tier 4"

- id: AC-006
  criterion: "GIVEN Persona 1's fear is silent wrongness, and a drill only answers it if the reader can recognise the failure rather than a build error, WHEN they compare what they see to what the page quoted, THEN the quoted failure is verbatim from the recorded run — including whichever file and line the runner names, which under `include_str!` may be the including item rather than the page — never paraphrased, never re-wrapped, never elided with an ellipsis; AND neither the drill nor the surrounding prose uses or suggests emptying the query, because `Query::from_items([])` returns `Err(InvalidQuery::NoItems)` and would go red for a constructor reason the reader cannot tell from a build error."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/narrative.rs — the quoted failure block on the step-3 page"
  verifying_test: "Byte comparison of the page's quoted failure block against the corresponding block in .bklg/docs-that-teach/application-author-path/boundary-falsification-drill/_drill-observation.md, plus a grep of the page returning nothing for `from_items([])` and for any elision inside the quoted output block (crates/happenstance-core/src/query.rs:180-190 is the ground truth)"

- id: AC-007
  criterion: "GIVEN the drill is content every reader meets rather than an aside for the curious, and the page must not acquire a second subject while acquiring it, WHEN the rendered step-3 page is inspected without running anything, THEN the drill is persistent — not inside `<details>`/`<summary>` or any fold or collapsed region — introduces no interactive affordance, CSS or JS; is the step's one declared additional element and adds no eighth competing one; every fence it references imports `use happenstance::{…}` and never `happenstance_core`; the words \"aggregate\", \"one stream per entity\" and \"which stream\" do not appear; the page still carries exactly one answered-need, above the first fence; and no sentence the drill adds states a MUST/MUST NOT in the page's own words — the only normative reference is the ES-25 citation."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/narrative.rs — the rendered step-3 page under HS-P0020's TREE (docs)"
  verifying_test: "Tier 5 reviewer walk against .bklg/docs-that-teach/application-author-path/_design.md `## Transience policy`, `## Density budget` and `## Anti-patterns` 1/3/6/8/11/12/13, recorded in _drill-observation.md; mechanically assisted by greps for `<details`, `<summary`, `happenstance_core`, `aggregate`, `MUST`, and by `cargo xtask spec-trace` (xtask/src/main.rs:315-327) resolving the ES-25 citation at spec/SPECIFICATION.md:3693"
```
