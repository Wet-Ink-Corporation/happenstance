---
item: "HS-S0184"
stage: implement
created: "2026-08-17T13:16:32.581Z"
updated: "2026-08-17T13:16:32.581Z"
---

# Acceptance ledger — DT-1, DT-4, DT-5 and DT-6 resolved and recorded

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

**Three notes specific to this story.** First, no row's `verifying_test` is a Rust `#[test]` that
lands in the tree: this story authors no Rust, and the DT-6 probe compiles and runs **outside** the
repository with only its transcript committed (`spec.md` § PR boundary, EC-008). The verifying
tests are therefore the tiers named in
`.bklg/docs-that-teach/application-author-path/_decomposition.md`, `## Testing brief` (`:480-486`,
`:543-574`) — a captured command outcome, a transcribed probe run, or the tier-5 reviewer walk.
Second, evidence must be **transcribed**, never asserted: these three project ACs score at tier 5
(`:522-524`), which names the mechanism, not permission to claim. A row whose evidence is "the
design says so" is the decorative check CLAUDE.md names, and this story exists because the design
already said so. Third, `mount_point` is
`.bklg/docs-that-teach/application-author-path/_design.md` on every row because that is the
composition root this story mounts onto — it is read-only here, and the mount is observable
downstream: it has happened when a page story can write
`[Where your streams went](…#where-your-streams-went)` without opening `_design.md` to guess the
slug.

```yaml
- id: AC-001
  criterion: "GIVEN Persona 1 arrives holding a cross-entity invariant they cannot place, and the initiative's bar is that the prior model is named \"once, consistently, rather than differently on each page\" (`project.md:194-197`), WHEN the four page stories are authored against this record, THEN `§ DT-1` certifies the resolution as `_design.md:81-120` records it — option (c), invariant-first, bounded by a **single located use** of the stream-per-entity prior at the seam where the reflex question *which stream does this go in?* arises, decided on the EventStoreDB frequency evidence (`interaction-patterns.md:547-566`) and treating \"explicitly none\" as a real answer (DR-06, `project.md:191-193`) — names the one reader-facing home as a **heading on `conceptual-bridge`**, not this file, and records the baseline for anti-pattern 6: the words \"aggregate\", \"your aggregates\", \"one stream per entity\" and \"which stream\" appear on no other surface."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/application-author-path/_design.md"
  verifying_test: "tier 5 review sign-off — a reviewer walks `.bklg/docs-that-teach/application-author-path/tension-resolutions/_resolutions.md` § DT-1 against `.bklg/docs-that-teach/application-author-path/_design.md:81-120`, `:108-114` and `:852-854`, recorded as project DoD item 1's observation (`.bklg/docs-that-teach/application-author-path/project.md:281-284`); plus the anti-pattern-6 baseline probe `rg -n -i \"aggregate|your aggregates|one stream per entity|which stream\" crates/happenstance/src/lib.rs docs/ examples/course-subscriptions/src/main.rs` run on the merged tree with its output transcribed into § DT-1, and a cold read of § Anchor table by the `boundary-refusal-encounter` implementer with no fallback to `_design.md`"

- id: AC-002
  criterion: "GIVEN a reader who lands on step 3 cold from a search result — the literature's named failure mode for staged disclosure (`interaction-patterns.md:161-166`) — WHEN `boundary-refusal-encounter` is authored against this record, THEN `§ DT-4` certifies the disclosure shape *with its three structural mitigations intact and not collapsed to a warning sentence*: no step is a fragment (cumulative in teaching, never in execution), a **two-line in-body step header block** carries the mid-sequence signal because rustdoc's sidebar TOC is removed entirely below 700px (`_design.md:394`, `:521`), and the payoff sits in step 3 so cold arrival is the best available landing — and the certification carries a **measured** verdict on whether step 3's program is simultaneously \"a complete runnable program\" and inside the 24-line / 68-column step budget, with the seven-element per-step composition (`_design.md:454-471`, `:591`) stated as the order the page must render."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/application-author-path/_design.md"
  verifying_test: "tier 1 structural render plus tier 5 review — step 3's program written against the merged `crates/happenstance/src/lib.rs` API and rendered via `cargo doc -p happenstance --no-deps` (`.bklg/docs-that-teach/application-author-path/_design.md:1025-1027`), its rendered line count and widest line read off `target/doc/happenstance/index.html` against the 24-line and 68-column budgets (`:552-574`) and transcribed into `.bklg/docs-that-teach/application-author-path/tension-resolutions/_resolutions.md` § DT-4 with a verdict of holds / reconciled / raised as reopen condition; reviewer walk recorded as DoD item 1 (tier 5, `.bklg/docs-that-teach/application-author-path/_decomposition.md:523`)"

- id: AC-003
  criterion: "GIVEN the hazard worth teaching is the **quiet** one — a guard that compiles, runs, and accepts an append it should have refused — and Persona 1's fear is exactly that shape, WHEN `invariant-to-appendcondition-bridge` is authored against this record, THEN `§ DT-5+DT-6` certifies DT-5 and DT-6 as **one** resolution: no diagram ships and AC-013 is discharged **explicitly, by the reviewer recording that it went that way**, not vacuously by silence (`_storymap.md:120`); the narration is four fixed names — *tag, query, fold, guard* — with a three-column mapping table in fixed column order and never four columns (`_design.md:592-595`); and the wrong side is **one compiled, executed fence** differing from the correct one by a single expression, a guard **tagged to what the command writes rather than to the invariant it holds**, whose `assert!(accepted.is_ok())` must **pass** — the mirror of CF-7 (`spec/SPECIFICATION.md:7266`, `[FROZEN]`), which names the *broadening* failure and would make that assertion fail. Zero entries go on HS-P0020's allowance list, and UX-015 is recorded as **vacuously satisfied with its reason** rather than omitted."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/application-author-path/_design.md"
  verifying_test: "tier 3 executed (out-of-tree) plus tier 5 review — the wrong-side fence from `.bklg/docs-that-teach/application-author-path/_design.md:373-384` compiled and run against the merged public API in a scratch program outside the repository, binding `EventStore` not `SendEventStore` and importing from `happenstance` (`.kb/decisions/0006-bare-name-to-the-typed-layer.md`), with stdout and the `assert!(accepted.is_ok())` result pasted verbatim into `.bklg/docs-that-teach/application-author-path/tension-resolutions/_resolutions.md` § DT-5+DT-6; a refusal is a falsification handled by EC-001, not a fix. Reviewer walk against `_design.md:170-262` recorded as DoD item 1 (tier 5, `.bklg/docs-that-teach/application-author-path/_decomposition.md:524`)"

- id: AC-004
  criterion: "GIVEN a page author mid-authoring needs the exact string `#where-your-streams-went` and must not decide it a second time — the defect BR-07 and DR-07 exist to prevent (`project.md:191-197`) — WHEN any of the six downstream stories opens this record, THEN `§ Anchor table` carries one row per `##` heading `_design.md`'s `## Composition` names across all four surfaces (`:408-504`) with **final heading text, character count, the surface's applicable budget, and the fragment id the renderer actually emits** — ids **read off a render**, never predicted, and marked provisional where the renderer is HS-P0020's markdown one; the retitles `_design.md:575-585` mandates but does not supply are supplied here for every heading whose surface the 22-character budget binds (`## Watch a boundary refuse` at 23, `## Where your streams went` at 23); the budget's scope is **reconciled** against sign-off condition 2 (`_design.md:1039`) per surface with the consequence for anti-pattern 5 stated; and `§ Consumption map` names, per downstream story, the `_design.md` sections binding it, the anchor rows it renders or links, and the one citation string it uses for DT-1."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/application-author-path/_design.md"
  verifying_test: "tier 1 structural — `cargo doc -p happenstance --no-deps`, then the emitted `id=\"…\"` attributes read out of `target/doc/happenstance/index.html` and pasted into `.bklg/docs-that-teach/application-author-path/tension-resolutions/_resolutions.md` § Anchor table (the method `.bklg/docs-that-teach/application-author-path/_design.md:110-112` uses by example); `cargo xtask spec-trace` and `cargo xtask lints` green (`.redkiln/config.yaml:48`); plus the tier-5 reviewer check that there is one row per named `##` heading and one § Consumption map row per remaining story (`.bklg/docs-that-teach/application-author-path/_storymap.md:56-62`), which is the mechanism `.bklg/docs-that-teach/application-author-path/_decomposition.md:530` assigns to AC-009"

- id: AC-005
  criterion: "GIVEN the design gate rejected this project once for absorbing findings its binding sections still contradicted (`_design.md:1041-1048`), and project DoD item 9 requires that nothing found is absorbed silently (`project.md:300-302`), WHEN this story's checkpoint is cut, THEN `§ Conditions and dispositions` carries a per-resolution status of `holds`, `reconciled` (with the reconciliation stated) or `raised as reopen condition` for **both** sign-off conditions (`_design.md:1039`) and **all four** merge dispositions from the slice-mate's `§ Dispositions` — each as three fields (what the design says, what the tree says, the status), each preserving the design's original *reason* even where the row's status changes, each escalation routed to a real item (HS-P0020 / HS-P0023 / HS-P0024 / the `support` initiative, `.redkiln/config.yaml:5`) — while `_design.md` is **byte-identical** to its signed-off state, the authored diff is confined to this story's folder, and **zero teaching content** ships: no fence on any surface, no answered-need line, no mapping table, no step header block, no control outside the medium's own chrome."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/application-author-path/_design.md"
  verifying_test: "ledger and boundary gate plus tier 5 review — `git diff <base> HEAD -- .bklg/docs-that-teach/application-author-path/_design.md` returning empty; `redkiln verify --grain story` holding the PR boundary fence over `.bklg/docs-that-teach/application-author-path/tension-resolutions/**`; `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) green at the checkpoint; and a reviewer walk of `.bklg/docs-that-teach/application-author-path/tension-resolutions/_resolutions.md` § Conditions and dispositions against `.bklg/docs-that-teach/application-author-path/_design.md:1039`, the slice-mate's `.bklg/docs-that-teach/application-author-path/merge-forward-preflight/_baseline.md` § Dispositions and `.bklg/docs-that-teach/application-author-path/project.md:300-302`, with every `raised as reopen condition` row cross-checked to a named condition on the story that would have consumed it"
```
