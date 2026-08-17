---
item: HS-S0168
stage: implement
created: "2026-08-17T13:16:21.305Z"
updated: "2026-08-17T13:16:21.305Z"
---

# Acceptance ledger — Land the small content fixes a "fixed" disposition asserts

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes specific to this story. **The mount point is the `Disposition: fixed: <ref>` field of each
fixed stumble in `_friction-log.md`'s `## Chronological record`** — where `_design.md`'s protocol
section fixes a different path for that file, it wins verbatim (spec `## Error conditions`, EC-001).
And **no `verifying_test` here is a compiled test**: this story adds no function, so its criteria are
verified by the Static, Doctest and Artifact-evidence tiers the project's testing brief defines
(spec `## Tests and CI (merge gate)`), which is the honest shape rather than a shortfall.

```yaml
- id: AC-001
  criterion: "GIVEN U3 — a sibling-project owner, the `support` initiative, or HS-P0025 — opens the log to decide what is still theirs, WHEN they read a stumble whose arm reads `fixed:`, THEN the `<ref>` resolves to a real path in this PR and to the hunk that discharged it, and conversely every content hunk in this PR traces back to exactly one `FL-###` whose arm reads `fixed:` — so the friendliest of the five labels is a fact about the tree rather than an assertion nothing can fail"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md → the `Disposition: fixed: <ref>` field of each fixed stumble in `## Chronological record`"
  verifying_test: "Static — `rg -n \"Disposition: fixed:\" .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` for the work list; `test -f` plus an anchor `rg` on each `<ref>`; `git diff --name-only main...HEAD` restricted to crates/ docs/ examples/ spec/ mapped against the work list in both directions"

- id: AC-002
  criterion: "GIVEN the next reader who walks the same path U1 walked and who will never see the log, WHEN they reach the point where U1 stopped, THEN the page they open is the page the fix landed in — because every fix traces stumble id → the page named in that stumble's \"What happened\" field (the file opened, the link followed, the search typed) → the diff hunk, and no hunk lands in a page no stumble names"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md → the `What happened` field of each fixed stumble, and the implicated page it names under docs/, crates/happenstance-core/src/, crates/happenstance/src/ or examples/course-subscriptions/"
  verifying_test: "Artifact-evidence — a three-column trace per fix (`FL-###` → implicated path → hunk) cited `file:line` into `_friction-log.md` and checked by a reviewer against that entry's own narrative field"

- id: AC-003
  criterion: "GIVEN a reviewer reading this PR as raw text with every rendering step stripped, WHEN they look at any one fix, THEN it is composed from one of exactly three permitted primitives — a `# Errors`/`# Panics` doc-comment section or intra-doc link per RS-70, a row in `docs/README.md`'s existing two-column routing table, or a compiled example per RS-62 — and it carries real composed presentation in that primitive's own shape (conditions named in the `# Errors` section, a routing row with both columns filled, an example that compiles) rather than a sentence appended below the existing prose; and the PR contains zero new heading vocabulary, zero navigation widgets, zero `ignore`-fenced snippets, zero `allow(clippy::doc_markdown)`, zero collapsed or `<details>` containers, and no intra-doc link that resolves in only some feature configurations"
  satisfied: false
  evidence: ""
  mount_point: "The changed pages themselves — docs/README.md's routing table, the rustdoc of crates/happenstance-core/src/ and crates/happenstance/src/, examples/course-subscriptions/ — each reachable from a `fixed:` arm's `<ref>`"
  verifying_test: "Static — an rg for an ignore-fenced code block, for allow(clippy::doc_markdown), and for <details>/<summary>/doc(hidden) over the changed files, each returning nothing; new headings checked against standards/rust/70-rustdoc-obligations.md. Doctest — cargo test --workspace --all-features"

- id: AC-004
  criterion: "GIVEN an adapter author who will rely on a `happenstance-core` doc comment that discharges a `SPECIFICATION.md` clause, WHEN a fix rewrites that comment, THEN the set of implementations the clause admits is unchanged — the edit is a repair — or the edit is not made: the difference is recorded as a gap naming the clause id and routed through `route-and-escalate`, never decided here and never written as an ADR; and every `file:line` citation in `SPECIFICATION.md` whose range this PR's own edits shifted is re-resolved and repaired in the same commit as the edit that moved it"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/ doc comments as cited by spec/SPECIFICATION.md's `file:line` ranges (e.g. spec/SPECIFICATION.md:371 → crates/happenstance-core/src/store.rs:93-268); a recorded gap is mounted at the stumble's disposition arm in _friction-log.md"
  verifying_test: "Artifact-evidence against `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`, `## The test: repair or gap`, ledger-cited per affected comment; Static — `rg -n \"src/[a-z_]+\\.rs:[0-9]+-[0-9]+\" spec/SPECIFICATION.md` with each range re-resolved by hand; `cargo xtask spec-trace` green"

- id: AC-005
  criterion: "GIVEN a sibling-project owner who signed off DT-1, DT-4, DT-5 or DT-6 in HS-P0022's design review, WHEN a fix attempted here would change what they decided, THEN the fix is not landed: the stumble's arm is revised to `escalated: DT-<n>` with a DT id that exists in the ownership table, the change appends a dated line in that entry's `Revisions:` slot carrying the earlier disposition and the reason with the original `fixed:` text still legible, and the entry's id, heading line (label included) and narrative text are byte-identical to what they were"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md → the `Disposition:` and `Revisions:` fields of the affected stumble in `## Chronological record`, and only those two fields"
  verifying_test: "Static — every `DT-<n>` written by this PR appears in `rg \"^\\| DT-\" .bklg/docs-that-teach/_decomposition.md`; `git diff -- .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` shows hunks only inside `Disposition:` and `Revisions:` fields, none touching a `### FL-` heading, the entry order or a `What happened` field"

- id: AC-006
  criterion: "GIVEN the repository owner merging this PR, WHEN they run the bars this repository already defines, THEN `cargo xtask affected --base main` is green at story grain, `cargo xtask ci --fast` is green at integration grain — including every doctest a fix touched and all four wasm32 steps — and `redkiln validate --kb && redkiln doctor` is clean at exactly the six standing `template-drift` advisories, with no gate step, no new tool, no new dependency and no automated \"disposition check\" added by this story"
  satisfied: false
  evidence: ""
  mount_point: "The repository gate itself — `.redkiln/config.yaml:40` (affected_gate), `:48` (reachability_static), `:55` (integration_scoped), defined once in xtask/src/main.rs"
  verifying_test: "`cargo xtask affected --base main`; `cargo xtask lints && cargo xtask spec-trace`; `cargo xtask ci --fast`; `cargo test --workspace --all-features`; `redkiln validate --kb && redkiln doctor`"

- id: AC-007
  criterion: "GIVEN `disposition-every-stumble` gave every stumble an arm and none of them reads `fixed:`, WHEN this story runs, THEN it lands zero changes under `crates/`, `docs/`, `examples/` and `spec/`, and records the fixed-arm count — zero — in the implementation report and in this story's ledger, so an empty diff reads as a complete outcome rather than as work not done, and no fix is manufactured to make the story look delivered"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md → the enumerated `Disposition: fixed:` arms in `## Chronological record`, whose count is the work list; recorded in this story's implementation report"
  verifying_test: "Static — `rg -c \"Disposition: fixed:\" .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` yields the count the report states; when zero, `git diff --name-only main...HEAD` contains no path under crates/, docs/, examples/ or spec/"
```
