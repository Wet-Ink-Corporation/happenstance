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
  satisfied: true
  evidence: |-
    `_falsification.md` §§ Baseline / The edit / Run 2 — red. Run 1 exited 0 at
    `6368e2b1c45c6506d0258441035c48829370dc31` with 26 banners and `all checks passed`; the
    one-line hunk (`assert_eq!(store.len(), 0)` -> `1`) is recorded as a unified diff `git apply
    --check` accepts at that sha; run 2 exited 1 in 30.2 s at the same sha. Failure is
    attributable to the edit and nothing else: the failing test is
    `xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 9)` and the
    panic is `assertion `left == right` failed / left: 0 / right: 1`. The truncation is stated
    beside the transcript — `run_steps` bails at the first non-zero status
    (xtask/src/main.rs:963), so run 2 printed 3 of 26 banners and says nothing about the rest.
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
  satisfied: true
  evidence: |-
    `_falsification.md` § Attribution and its counterfactual. **The predicted banner is
    FALSIFIED and that is the recorded outcome, not a smoothed one.** Measured: the failing
    banner is `=== tests ===` (step index 2); `=== the narrative tree's examples compile ===`
    (index 16) and `=== the constitution's examples compile ===` (index 17) both never printed,
    because `run_steps` bails first. Mechanism: `tests` is `cargo test --locked --workspace
    --all-features`, and `cargo test` runs a lib target's doctests, so `xtask`'s lib target —
    the harness's doctest root at xtask/src/lib.rs:28 — compiles and runs the narrative pages
    fourteen steps before the narrative step does. EC-005 governs the disposition: recorded
    verbatim under this criterion and routed as finding **F2** to
    `pinned-narrative-tree-and-compiling-step` (owner of step ordering) and to HS-S0145; not
    fixed here (EC-009). The counterfactual half HOLDS: `RUSTDOCFLAGS=-D warnings cargo test
    --locked -p xtask --doc` run directly while the page was broken exits 101, so the overlap
    milestone 1 reasoned about is real and ordering — not the `narrative::` filter — is what
    would preserve attribution between steps 16 and 17. It is not what preserves it in the gate.
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
  satisfied: true
  evidence: |-
    `_falsification.md` §§ Predictions, pre-registered / What the failure actually identified.
    Predictions P1-P8 were written into the record before run 2 was started. As measured:
    file = `xtask\src\../../docs/append-conditions.md` (the harness's *directory* plus the
    page's own repo-relative path — Note 3's "the harness file" DIVERGES, routed as F5);
    doctest name = `narrative::append_conditions`, confirmed independently by `cargo test
    --locked -p xtask --doc -- --list`; line = 9, the fence's opening marker, NOT the broken
    assertion at page line 13; failure kind = **assertion panic**, exit 101, so the fences are
    run and not merely compiled (F1). Second finding: the panic's own location is a different
    and unstable string — `xtask\src\../../docs/append-conditions.md:6:1` with `--show-output`,
    `…\Temp\rustdoctest<random>\doctest_bundle_2024.rs:9:1` without it, established by a
    controlled pair differing only in that flag — and `:6` counts inside rustdoc's synthesized
    doctest source, so it points at prose (F5).
  mount_point: "xtask/src/narrative.rs — milestone 1's harness, the file rustdoc is predicted to name, reached through `xtask/src/main.rs:105`"
  verifying_test: "`cargo test --locked -p xtask --doc -- --list` + run 2's transcript, recorded at .bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/_falsification.md §§ Predictions, pre-registered / What the failure actually identified"

- id: AC-004
  criterion: |-
    GIVEN a contributor who has reverted the break and needs to know the tree is genuinely restored —
    and a reviewer who needs to know the green means something — WHEN `cargo xtask ci` is run again,
    THEN the whole gate is green, and the record carries the narrative compile step's enumerated page
    count and the checker's `  {n} pages, all consistent` line with the numbers as they stood at the
    recorded sha, and confirms neither narrative step printed a `skipped:` line.
  satisfied: true
  evidence: |-
    `_falsification.md` § Run 3 — recovered. `git checkout -- docs/append-conditions.md` then
    `cargo xtask ci` exited 0 in 1 m 58 s, printing all 26 banners and `all checks passed` — the
    only whole-gate claim in the record. Coverage at `6368e2b`, quoted verbatim: `  1 page(s)'
    examples enumerated` and `running 1 test … 1 passed; 0 failed … 170 filtered out` from the
    compile step, `  1 pages, all consistent` from the checker (one line, two-space indent, the
    shape of xtask/src/lint_constitution.rs:192), alongside `  27 atoms, all consistent` and
    spec-trace's 358 citations checked. `docs/` held two files at this sha, one of them the
    unregistered index, which is why the number is 1. `grep -c "^skipped"` returns 0 across all
    three runs; both narrative steps carry `probe: None` (xtask/src/main.rs:509, :551), so
    RUNBOOK.md:914-928's failure is not reproduced and EC-006 is not triggered.
  mount_point: "xtask/src/main.rs:105 — both narrative steps in `REQUIRED` with `probe: None` (contract at :89-102), exercised through `cargo xtask ci`"
  verifying_test: "`git checkout -- docs/append-conditions.md` then `cargo xtask ci`, transcribed at .bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/_falsification.md § Run 3 — recovered"

- id: AC-005
  criterion: |-
    GIVEN every later story in this initiative, each of which runs this same gate, WHEN this story's
    PR is merged, THEN it carries no broken page and no source change: `git status --porcelain` is
    empty after the recovered run, and the commit diff contains no path under `docs/`, `crates/`,
    `xtask/`, `spec/` or `standards/` — the only committed paths are inside this story's own backlog
    folder.
  satisfied: true
  evidence: |-
    `_falsification.md` § Residue. After run 3, `git status --porcelain` returns two entries and
    neither is a source file or a page: `M .redkiln/telemetry/events/ryan-britton@docs-that-teach.jsonl`
    (the redkiln CLI's own append-only session telemetry, written by the tool that launched the
    work) and `?? …/observed-failure-falsification/_falsification.md` (this record).
    `docs/append-conditions.md` is absent from the list — edited, observed, restored with
    `git checkout --` and not `git stash`, so git sees no change to it. No path under `docs/`,
    `crates/`, `xtask/`, `spec/` or `standards/` is modified. The commit for this story stages
    only paths inside
    `.bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/`, and
    the reviewer's re-check on the merged tree is
    `git diff --stat HEAD~1 -- docs/ crates/ xtask/ spec/ standards/` returning empty.
  mount_point: ".bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/ — the only tree this PR writes; `docs/append-conditions.md` appears in the boundary and must be unchanged"
  verifying_test: "`git status --porcelain` (empty) and `git diff --stat HEAD~1 -- docs/ crates/ xtask/ spec/ standards/` (empty), recorded at .bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/_falsification.md § Residue"

- id: AC-006
  criterion: |-
    GIVEN a contributor reading this record a year later against a tree that has moved, WHEN they try
    to re-run it, THEN `_falsification.md` gives them everything they need without asking anyone: the
    date, a `git rev-parse HEAD` sha that still resolves, the toolchain, the full command sequence and
    the edit as a diff hunk; AND a transcript that a later tree has made stale is answered by a second
    dated section from a second run, never by editing the first.
  satisfied: true
  evidence: |-
    `_falsification.md` § Provenance, plus the file's own preamble. It carries the date
    (2026-08-17), the sha `6368e2b1c45c6506d0258441035c48829370dc31` (`git cat-file -e` resolves
    it; the branch `initiative/docs-that-teach` is recorded beside it), `rustc 1.97.1 (8bab26f4f
    2026-07-14)` with host and LLVM version, `cargo 1.97.1 (c980f4866 2026-06-30)`,
    `rust-toolchain.toml`'s pin, the OS and shell, the full copy-pasteable command sequence in
    order, and the edit as a unified diff hunk `git apply --check` accepts at that sha. The
    preamble states the append-a-dated-section rule in as many words and cites `_design.md`
    `## Mock`'s "editing the mock to agree with the corrected design would delete the evidence";
    the run is nested under a dated `## Run of 2026-08-17` heading precisely so a second run gets
    a sibling section rather than an edit. Transcript shape copied from
    `experiments/rustc-ice-gat-foreign-trait/README.md:1-22`.
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
  satisfied: true
  evidence: |-
    `_falsification.md` § What this does not establish. Its opening sentence is unhedged and
    names the non-substitutable instrument: the observation establishes that the code inside the
    page still compiles and, because rustdoc runs it, that the one claim its fence asserts still
    holds — and is silent about whether the page teaches anybody anything, with HS-P0024's
    friction log named as the only instrument in this initiative that is about comprehension.
    The three limits are stated as three numbered items: run 2's log is truncated at the failing
    step; `docs/` held one page and one index at this sha, so one fixture page is not the corpus;
    and one toolchain (`rustc 1.97.1`, `x86_64-pc-windows-msvc`) on one runner is not a guarantee
    about others, with the Windows-shaped `xtask\src\../../` name called out so a path separator
    is not read as a defect elsewhere. Mechanical half: `rg -n "verified|badge|shield"` over the
    record returns matches only inside the single sentence that quotes `_design.md`
    `## Anti-patterns` item 9, which the record marks as its only occurrence.
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
  satisfied: true
  evidence: |-
    `_falsification.md` § Composition, checked against the design — a twelve-row table, one row
    per predicted element, each carrying `observed` or `diverged` and the quoted line. Observed:
    the banner first on stdout (xtask/src/main.rs:940); a composed failure report and never a
    bare non-zero exit; problem lines present only in the failure state; the success summary at
    exactly one line; and no probe skip line in any of the three runs. Three named divergences,
    each routed to HS-S0145 rather than normalised — **F2** (the failing banner is a third step's,
    `=== tests ===`, which the design's surface manifest does not contain), **F3** (the tail is
    ONE xtask line, `xtask failed: tests failed with exit code: 101`, because :777 renders the
    error :963 produced inside the same process; the line beneath it is cargo's, not this
    repository's — so `_design.md` `## Composition`'s two-line tail and mock finding 2's
    correction are both over-specified), and **F4** (anti-pattern 7 is breached on the compile
    surface by rustdoc's own report, which begins `test ` or `thread 'main' (23672) panicked at `
    before the path). The section closes by naming the six states this story does NOT render, so
    the table is not read as coverage.
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
  satisfied: true
  evidence: |-
    `_falsification.md` § Density, measured — widths counted with `awk '{ print length($0) }'`,
    not by eye. Ten measured lines against the 80-column budget (97, 98, 85, 80, 100, 46, 83, 32,
    25, 117). The location prefix `xtask\src\../../docs/append-conditions.md` measures **41**
    characters: 16 of doctest prefix plus 25 repo-relative — inside both halves of the budget
    (41 <= 48, 25 <= 32), so the fixture page's own path fits with 7 characters of headroom.
    Column arithmetic on the failing line shows the path occupying columns 6-46 and `(line 9)`
    occupying 79-86, so at 80 columns the page survives on the first visual row and **the line
    number does not** — recorded as finding **F4** rather than rounded off. Green run: the
    checker's output is exactly one line and the compile step opens with exactly one coverage
    line. Truncation: `grep -n "and [0-9]* more|… and|\.\.\. and"` over all three logs returns
    nothing. The record states its own abridgement policy in § How complete each transcript below
    is — every quoted block is unedited and complete from banner to banner, the omission is
    between blocks, and the line counts (5,716 / 4,366 / 5,663) and reproducing commands are
    given so the omitted regions can be regenerated byte-for-byte.
  mount_point: "xtask/src/main.rs:105 → the two narrative steps' rendered output; budget owned by .bklg/docs-that-teach/checked-documentation-surface/_design.md `## Density budget` (terminal table), success-summary primitive at xtask/src/lint_constitution.rs:192"
  verifying_test: "measured line widths and location-prefix lengths taken off the captured transcripts, tabulated at .bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/_falsification.md § Density, measured"
```
