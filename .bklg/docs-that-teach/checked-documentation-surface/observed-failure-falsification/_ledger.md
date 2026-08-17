---
item: HS-S0144
stage: implement
created: "2026-08-17T13:16:05.747Z"
updated: "2026-08-17T13:16:05.747Z"
---

# Acceptance ledger — The gate is watched failing on a page broken on purpose, then recovering

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

**Two things about this story's rows are unusual and deliberate.** First, no row names a `#[test]`:
the testing brief settles that AC-003's proof "is a recorded procedure rather than a `#[test]`
function" and is "not substitutable by a unit test that calls the checker function directly"
(`.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md:568-570,581-584`), so each
`verifying_test` names the real command plus the section of `_falsification.md` that holds its
transcript — the artifact a reviewer re-runs. Second, the primary mount point is
`xtask/src/main.rs:105` (the `REQUIRED` array) exercised **as a whole** through `cargo xtask ci`:
this story adds nothing to that array and its entire claim is about what the array does when a page
under `docs/` is wrong. The artifact-side mount is `_falsification.md` itself, mounted into the
project's evidence chain by being cited from this ledger and from HS-S0145's spec.

```yaml
- id: AC-001
  criterion: |-
    GIVEN a contributor who has just edited a narrative page so one of its claims is no longer true
    of the library, and a tree whose gate was observed green immediately beforehand at a recorded
    sha, WHEN they run `cargo xtask ci`, THEN the gate exits non-zero, and all three runs — green
    baseline, red, recovered green — are transcribed verbatim into `_falsification.md` together with
    the exact edit as a unified diff hunk, so the failure is attributable to that edit and to
    nothing else in the tree.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:105 — the `REQUIRED` array, exercised as a whole through `cargo xtask ci`"
  verifying_test: "`cargo xtask ci` × 3 (green → red → green), transcribed at .bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/_falsification.md §§ Baseline, The edit, Run 2 — red"

- id: AC-002
  criterion: |-
    GIVEN the same broken page and a contributor who must work out which half of the machine is
    complaining, WHEN they read the failing run, THEN the failing banner is
    `=== the narrative tree's examples compile ===` and `=== the constitution's examples compile ===`
    never printed; AND WHEN the constitution step's own argv is run directly while the page is still
    broken, THEN it fails too — recorded as the counterfactual that shows step ordering, not the
    `narrative::` filter, is what preserves attribution.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:480-492 — `the constitution's examples compile`, unfiltered, ordered after the narrative step in `REQUIRED`"
  verifying_test: "run 2's banner sequence + `RUSTDOCFLAGS=-D warnings cargo test --locked -p xtask --doc` run directly, both at .bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/_falsification.md § Attribution and its counterfactual"

- id: AC-003
  criterion: |-
    GIVEN a reviewer who must decide whether the recorded failure is actionable — can they open a
    file and fix it? — WHEN they read the record, THEN it states as measured: the file rustdoc names,
    the doctest name including its module, the line number, whether that line locates the broken
    assertion inside `docs/append-conditions.md`, and whether the failure was a compile error or an
    assertion panic; AND both dispositions were pre-registered before the red run, so every
    divergence from the architecture brief's Note 3 prediction is written up as a finding routed to
    HS-S0145 rather than smoothed over.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/narrative.rs — milestone 1's harness, the file rustdoc is predicted to name, reached through `xtask/src/main.rs:105`"
  verifying_test: "`cargo test --locked -p xtask --doc -- --list` + run 2's transcript, recorded at .bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/_falsification.md §§ Predictions, pre-registered / What the failure actually identified"

- id: AC-004
  criterion: |-
    GIVEN a contributor who has reverted the break and needs to know the tree is genuinely restored —
    and a reviewer who needs to know the green means something — WHEN `cargo xtask ci` is run again,
    THEN the whole gate is green, and the record carries the narrative compile step's enumerated page
    count and the checker's `  {n} pages, all consistent` line with the numbers as they stood at the
    recorded sha, and confirms neither narrative step printed a `skipped:` line.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:105 — both narrative steps in `REQUIRED` with `probe: None` (contract at :89-102), exercised through `cargo xtask ci`"
  verifying_test: "`git checkout -- docs/append-conditions.md` then `cargo xtask ci`, transcribed at .bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/_falsification.md § Run 3 — recovered"

- id: AC-005
  criterion: |-
    GIVEN every later story in this initiative, each of which runs this same gate, WHEN this story's
    PR is merged, THEN it carries no broken page and no source change: `git status --porcelain` is
    empty after the recovered run, and the commit diff contains no path under `docs/`, `crates/`,
    `xtask/`, `spec/` or `standards/` — the only committed paths are inside this story's own backlog
    folder.
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/ — the only tree this PR writes; `docs/append-conditions.md` appears in the boundary and must be unchanged"
  verifying_test: "`git status --porcelain` (empty) and `git diff --stat HEAD~1 -- docs/ crates/ xtask/ spec/ standards/` (empty), recorded at .bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/_falsification.md § Residue"

- id: AC-006
  criterion: |-
    GIVEN a contributor reading this record a year later against a tree that has moved, WHEN they try
    to re-run it, THEN `_falsification.md` gives them everything they need without asking anyone: the
    date, a `git rev-parse HEAD` sha that still resolves, the toolchain, the full command sequence and
    the edit as a diff hunk; AND a transcript that a later tree has made stale is answered by a second
    dated section from a second run, never by editing the first.
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/_falsification.md — the artifact-side mount, cited from this ledger and from HS-S0145's spec"
  verifying_test: "`git cat-file -e <recorded sha>` and `git apply --check <recorded hunk>` against .bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/_falsification.md § Provenance"

- id: AC-007
  criterion: |-
    GIVEN the initiative's top-ranked risk — a green gate read as evidence that the documentation
    teaches — WHEN anyone reads `_falsification.md` end to end, THEN it carries one unhedged sentence
    saying the observation is silent about whether the page teaches and names HS-P0024's friction log
    as the non-substitutable instrument; AND it states its own three limits (the red log is truncated
    at the failing step; one fixture page is not the corpus; one toolchain on one runner is not a
    guarantee about others); AND no badge, tick, shield or "verified" mark appears anywhere in it.
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/_falsification.md § What this does not establish — the closing section of the artifact-side mount"
  verifying_test: "document review against project.md DoD item 8 and _design.md `## Anti-patterns` item 9, plus `rg -n \"verified|badge|shield\"` over .bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/_falsification.md"

- id: AC-008
  criterion: |-
    GIVEN a contributor meeting the gate's failure for the first time, who should meet a composed
    report — a banner naming the step, a body, and a tail that says which step failed — rather than a
    bare non-zero exit, WHEN the red and recovered runs are recorded, THEN the record checks the real
    output element by element against `_design.md` `## Composition` and `## Hierarchy`: banner first
    on stdout (`main.rs:864`), the report body, the two-line stderr tail (`xtask failed: {err:#}` at
    :709-715, then `{step} failed with {status}` at :887), the recovered run's success summary as
    exactly one line, and problem lines present only in a failure state; AND every element the design
    predicts that the real output does not produce — in particular anything rustdoc's own doctest
    report owns rather than this repository — is recorded as a named divergence routed to HS-S0145,
    never quietly normalised.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:862-892 (banner :864, `bail!` :887) and :709-715 — the composition root whose rendered output this story is the only story to observe live, in surface `gate-narrative-compile-step` state `fail-broken-fence`"
  verifying_test: "element-by-element check of runs 2 and 3 against .bklg/docs-that-teach/checked-documentation-surface/_design.md `## Composition` / `## Hierarchy`, tabulated at .bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/_falsification.md § Composition, checked against the design"

- id: AC-009
  criterion: |-
    GIVEN the same contributor reading that failure in an 80-column CI log, where a location pushed
    off the first visual row is a location they will not act on, WHEN the transcripts are measured,
    THEN the record reports: each line's width against the 80-column budget; the location prefix
    against ≤ 48 characters inclusive of the 16-character `xtask\src\../../` doctest prefix (i.e.
    ≤ 32 characters repo-relative); the green run's narrative output as exactly one summary line; and
    confirms nothing was truncated or elided — no `… and N more` anywhere in the gate's output, and
    the transcripts themselves complete rather than abridged.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:105 → the two narrative steps' rendered output; budget owned by .bklg/docs-that-teach/checked-documentation-surface/_design.md `## Density budget` (terminal table), success-summary primitive at xtask/src/lint_constitution.rs:192"
  verifying_test: "measured line widths and location-prefix lengths taken off the captured transcripts, tabulated at .bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/_falsification.md § Density, measured"
```
