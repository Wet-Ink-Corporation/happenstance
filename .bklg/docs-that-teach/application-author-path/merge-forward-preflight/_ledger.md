---
item: "HS-S0183"
stage: implement
created: "2026-08-17T13:16:32.000Z"
updated: "2026-08-17T13:16:32.000Z"
---

# Acceptance ledger — Merge forward and record the baseline before authoring

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

**Three notes specific to this story.** First, no row's `verifying_test` is a Rust `#[test]`: this
story compiles nothing of its own, so the verifying tests are the tiers named in
`.bklg/docs-that-teach/application-author-path/_decomposition.md`, `## Testing brief`
(`:480-486`, `:543-574`) — a captured command outcome against a named sha, or the tier-5 reviewer
walk. Second, evidence must be the **measured** value taken on the merged tree, never a figure
copied out of `spec.md`: every number in the Context pack was measured against the sibling tip
`3f49ec6` before the merge, and is there to be compared against, not transcribed. Third,
`mount_point` is `crates/happenstance/src/lib.rs` on every row because that is where this story's
mount is observable — the merge commit is the mechanism, but the 237-line file with `Tags::empty()`
at `:38` and `:55` is the evidence it happened.

```yaml
- id: AC-001
  criterion: "GIVEN Persona 1 (the application author) will meet this library as its rendered crate root, WHEN the merge lands on `initiative/docs-that-teach`, THEN `initiative/from-contract-to-published-library` is an ancestor of the branch tip, the merge commit records both parent shas in `_baseline.md § Merge`, `crates/happenstance/src/lib.rs` is the sibling's 237-line file carrying `Tags::empty()` at `:38` and `:55`, `cargo doc -p happenstance --no-deps` renders `/happenstance/index.html` with `#main-content details.top-doc > div.docblock` present — so the page the reader will be taught from is composed and addressable at baseline, not merely merged on paper — and nothing this branch authored under `.bklg/**`, `.redkiln/telemetry/events/**` or `references/seeds/user-documentation.md` is occluded by the merge."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "tier 0 preflight — `git merge-base --is-ancestor initiative/from-contract-to-published-library HEAD` and `git rev-list --parents -n 1 HEAD`, transcribed into `.bklg/docs-that-teach/application-author-path/merge-forward-preflight/_baseline.md` § Merge; plus the tier-1 render check `cargo doc -p happenstance --no-deps` with the selector `#main-content details.top-doc > div.docblock` located in `target/doc/happenstance/index.html`, and `git diff <pre-merge-sha> HEAD -- .bklg .redkiln/telemetry references/seeds/user-documentation.md` showing only this story's own additions"

- id: AC-002
  criterion: "GIVEN the next implementer in the `preflight-and-anchor` slice must be able to trust that a red gate is their own page's doing and not an inherited failure, WHEN this story's checkpoint is cut, THEN `cargo xtask ci --fast` has been run **on the merge commit itself** and recorded green against that commit's sha in `_baseline.md § Gate`, `cargo xtask affected --base main` is green at the story grain, and the baseline is reversible in one move — the merge is a single commit whose first parent is this branch's pre-merge tip, so `git revert -m 1 <sha>` restores it without unpicking 223 commits."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "integration grain, non-terminal — `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) run at HEAD = the merge commit, with sha, command and outcome recorded in `.bklg/docs-that-teach/application-author-path/merge-forward-preflight/_baseline.md` § Gate; story grain — `cargo xtask affected --base main` (`.redkiln/config.yaml:41`) at the checkpoint; parent order confirmed by `git rev-list --parents -n 1 HEAD`"

- id: AC-003
  criterion: "GIVEN an author of any of the four downstream page stories needs a clause or a source line **cold**, without re-running the archaeology this story already ran, WHEN they open `_baseline.md § Anchors`, THEN every anchor those stories will open resolves from that table alone — ES-25, VT-30, ES-26, ES-27 and CF-7 stated **by clause id first and merged line second** (ids are stable and never renumbered, `spec/SPECIFICATION.md:280`; lines are not), `const REQUIRED` and the `\"tests\"` step in `xtask/src/main.rs`, the worked example's module-doc span, and `crates/happenstance/Cargo.toml`'s `[dev-dependencies]` table — each row carrying the command that re-derives it, each line verified against the merged tree rather than copied from the pre-merge corpus, and `cargo xtask spec-trace` green over that tree."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "tier 1 structural — `cargo xtask spec-trace` (REQUIRED step, `xtask/src/main.rs`) on the merge commit, and `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml` `reachability_static`); every row of `.bklg/docs-that-teach/application-author-path/merge-forward-preflight/_baseline.md` § Anchors re-derived by the command it names (`rg -n \"^ES-25\" spec/SPECIFICATION.md` and siblings) with the reported line matching the recorded one"

- id: AC-004
  criterion: "GIVEN the four contradictions the merge creates would otherwise be rediscovered one story at a time — each time as a surprise mid-authoring — WHEN a page author reads `_baseline.md § Dispositions`, THEN all four appear with an explicit disposition of `carried`, `routed to <item>` or `fixed here`: the `tokio` dev-dependency already present with `macros`/`rt`/`rt-multi-thread`, the worked example importing `happenstance` rather than `happenstance_core`, its module doc at `:1-28` rather than `:1-19`, and its new `tests/runs.rs` and `tests/ui.rs` under `trybuild` against `project.md:75-77`'s \"no test target\" — each preserving the *reason* `_design.md` gave rather than re-deciding the design (the `tokio` row's justification that a fence must execute and not merely type-check survives even though the row's status does not), each route named per project DoD item 9 (`project.md:300-302`), and `_design.md` byte-identical to its signed-off state."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "tier 5 review sign-off — a reviewer walks `.bklg/docs-that-teach/application-author-path/merge-forward-preflight/_baseline.md` § Dispositions against `.bklg/docs-that-teach/application-author-path/_design.md:284-306`, `:497-502`, `.bklg/docs-that-teach/application-author-path/project.md:73-77` and `.redkiln/config.yaml:5`, recorded as DoD item 8's observation; plus `git diff <pre-merge-sha> HEAD -- .bklg/docs-that-teach/application-author-path/_design.md` returning empty"

- id: AC-005
  criterion: "GIVEN every composition number in the signed-off design was measured on the **pre-merge** render of a file the merge replaces, WHEN the baseline is recorded, THEN `_baseline.md § Composition baseline` carries the re-measured value for each composition invariant the merge could have moved — the count of literal `[bracket]` pairs rendered on the crate root (four before; anti-pattern 2 targets zero), the count of `#`-hidden lines in the crate-root fence and whether any of them carries a `Query`, `Tags`, `Guard`, `AppendCondition`, the append call or an assertion (forbidden by the transience policy, `_design.md:524`), the merged fence's widest line against the **68-column** budget and its rendered height against the **32-line** crate-root ceiling, and every `##` heading over **22 characters** — each read off `target/doc/happenstance/index.html` rather than argued from the source; and the authored diff ships **zero teaching content**: no Rust fence, no answered-need line, no mapping table, and no control outside rustdoc's own chrome (anti-pattern 8)."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "tier 1 structural render plus tier 5 review — `cargo doc -p happenstance --no-deps` (`.bklg/docs-that-teach/application-author-path/_design.md:1025-1027`), then each number measured off `target/doc/happenstance/index.html` and written into `.bklg/docs-that-teach/application-author-path/merge-forward-preflight/_baseline.md` § Composition baseline beside the design's pre-merge figure; `git diff --stat HEAD^1 HEAD` on the merge showing no authored file, and the PR boundary fence holding under `redkiln verify --grain story`"
```
