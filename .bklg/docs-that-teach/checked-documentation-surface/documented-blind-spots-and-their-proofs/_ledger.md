---
item: HS-S0145
stage: implement
created: "2026-08-17T13:16:06.300Z"
updated: "2026-08-17T13:16:06.300Z"
---

# Acceptance ledger — What the check does not verify is stated first, and executed where it can be

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Four notes for whoever flips these rows.

**Two mount points, in two Cargo targets, and mixing them up is this project's named most-likely
error.** The primary mount is `xtask/src/narrative.rs`, the lib-target harness declared from
`xtask/src/lib.rs:28` — the only file this story touches whose module docs a gate step actually
renders. The second mount is the bin-crate checker module declared from `xtask/src/main.rs:64-70`;
its sibling ledgers record that HS-S0138 landed it as `xtask/src/lint_narrative.rs`. If either file
landed under another name, correct the paths below to the file that actually exists and cite it —
that is a path correction, not a scope change.

**Three rows are not satisfied by a `#[test]` and must not be flipped as if they were.** AC-005,
AC-006 and AC-010 are discharged by recorded evidence in
`.bklg/docs-that-teach/checked-documentation-surface/documented-blind-spots-and-their-proofs/_limits-evidence.md`
— verbatim transcripts naming their exact command and the toolchain pin. A unit test asserting that
the docs *mention* a probe is not the probe. AC-011 is likewise a prose review over the whole diff,
by construction (`project.md` DoD item 8).

**AC-004 is satisfied by an absence.** Its evidence is the documented inherently-untestable clause
plus the fact that no test, fixture, badge or metric anywhere in the PR gestures at covering limit 1.
Adding one to make the row feel better is the exact failure RS-81-1 names.

**If EC-003 fires — the `text` fixture is already flagged, so limit 5 is closed — AC-006 and AC-007
lose their subject.** Record the disposition in `_limits-evidence.md` and cite it as the evidence
here, annotating both rows explicitly. Do not silently satisfy them against a fixture that no longer
proves anything, and do not weaken the fence walk to make the documented limit true.

```yaml
- id: AC-001
  criterion: "GIVEN a contributor opens `xtask/src/narrative.rs` to find out what the compiling step actually buys them, WHEN they read from the top of the file, THEN the very first `#`-level heading in the module docs is `# What this does not verify` — persistent prose composed as a real rustdoc heading with a bulleted body, never a `//` comment, never a bullet nested under a \"how it works\" preamble, and never behind a `<details>` — AND the `documentation` step stays green under `RUSTDOCFLAGS=-D warnings`, which the section earns by carrying no intra-doc link to a `#[cfg(doctest)]` page module (those modules do not exist under `cargo doc`, so such a link is a hard error)."
  satisfied: true
  evidence: |-
    `xtask/src/narrative.rs:1-6` — `//! # What this does not verify` is the first `#`-level
    heading in the module docs, a real rustdoc heading with a bulleted body (five `//! * **…**`
    bullets), not a `//` comment, not nested under a preamble and not behind a fold. Asserted by
    `xtask::lint_narrative::tests::both_modules_state_every_limit_they_own_and_none_of_the_others`,
    which reads the file as text through `workspace_root()` and reports a distinct problem when
    the first heading is anything else — the technique `lint_constitution.rs:423-425` uses one
    file over, because `cargo doc -p xtask` documents the lib target only. The negative arm is
    `::tests::a_limits_section_moved_below_another_heading_is_rejected`. Gate half: the
    `documentation` step is green under `RUSTDOCFLAGS=-D warnings` in `cargo xtask ci --fast`
    and in the full `cargo xtask ci`; the section carries no intra-doc link across the target
    boundary, which `::tests::the_cross_target_reference_is_a_plain_path_and_not_an_intra_doc_link`
    asserts directly.
  mount_point: "`xtask/src/narrative.rs` — the lib-target harness declared from `xtask/src/lib.rs:28` beside `mod constitution;`; rendered and lint-denied by the `documentation` step (`xtask/src/main.rs:284-302`) inside `cargo xtask ci`"
  verifying_test: "the section-shape test in the bin-crate checker module (`xtask/src/lint_narrative.rs` → `mod tests`), reading `HARNESS` as text through `workspace_root()` (`cargo test -p xtask`), plus `cargo xtask ci --fast` → `documentation` green"
- id: AC-002
  criterion: "GIVEN the same contributor opens the bin-crate narrative checker module instead — the half that reads pages rather than compiling them — WHEN they read from the top, THEN they meet `# What this does not verify` first there too, and its reference to the harness's four limits is a plain path in prose, not an intra-doc link, because the bin crate has no path to the lib target's modules (`xtask/src/lint_constitution.rs:5` is the standing proof that such a link fails nothing and therefore protects nothing)."
  satisfied: true
  evidence: |-
    `xtask/src/lint_narrative.rs:10-22` — `# What this does not verify` is first there too, and
    its reference to the harness's four limits is the plain path `xtask/src/narrative.rs` in
    prose ("are properties of the compile mechanism and are stated where they hold, in
    `xtask/src/narrative.rs`"), never an intra-doc link. Asserted by the second arm of
    `::tests::both_modules_state_every_limit_they_own_and_none_of_the_others` and by
    `::tests::the_cross_target_reference_is_a_plain_path_and_not_an_intra_doc_link`, which
    checks both directions and rejects `[`crate::lint_narrative`]` and `[`crate::narrative`]`
    by name. The reason it must be a path is the standing proof at `lint_constitution.rs:5`:
    a bin-crate `[`crate::constitution`]` has never failed a gate that denies every rustdoc
    warning, because nothing renders the bin crate's docs.
  mount_point: "the bin-crate narrative checker module (`xtask/src/lint_narrative.rs`), declared from `xtask/src/main.rs:64-70` beside `mod lint_constitution;`"
  verifying_test: "the section-shape test's second arm, reading the checker module's own file as text (`xtask/src/lint_narrative.rs` → `mod tests`, `cargo test -p xtask`)"
- id: AC-003
  criterion: "GIVEN a downstream author who must decide whether a green run licenses them to stop checking a claim by hand, WHEN they read both sections, THEN they find all six of Note 10's limits — each stated where it actually holds (1–4 in the harness, 5 in the checker, 6 unhedged in both) and each written in the precedent's two-part bullet shape: what is not verified, and what the real instrument is — so no limit is discoverable only by reading the other target's file."
  satisfied: true
  evidence: |-
    All six are present, each where it holds, each in the two-part bullet shape. Harness
    (`xtask/src/narrative.rs`): limit 1 at `:17-27`, limit 2 at `:28-34`, limit 3 at `:35-49`,
    limit 4 at `:50-66`, limit 6 at `:67-71`. Checker (`xtask/src/lint_narrative.rs`): limit 5
    at `:31-45`, limit 6 at `:114-118`. The enumeration and the two-part rule are the `NOTE_TEN`
    table in the checker's test module, whose `Limit` carries `claim` and `instrument` and whose
    `limits_problems` requires **both halves in the same bullet** — a limit whose compensating
    instrument is named three paragraphs away is one a reader meets as an apology. Both
    directions are asserted by
    `::tests::both_modules_state_every_limit_they_own_and_none_of_the_others`: each module
    carries every limit it owns and **restates none of the other's**, which is the half that
    keeps this from being two copies and one that goes stale. The negative arms are
    `::tests::a_module_missing_one_of_its_limits_is_rejected` (deletes limit 2's whole bullet
    from the real harness text and requires `limit 2 is missing`) and
    `::tests::a_module_restating_the_other_modules_limit_is_rejected` (splices limit 4's claim
    into the checker and requires `restates limit 4`).
  mount_point: "both mounts: `xtask/src/narrative.rs` (limits 1–4, 6) and `xtask/src/lint_narrative.rs` (limits 5, 6)"
  verifying_test: "the per-limit presence test asserting each module carries its assigned limits and does not restate the others (`xtask/src/lint_narrative.rs` → `mod tests`, `cargo test -p xtask`)"
- id: AC-004
  criterion: "GIVEN a reader who wants to know whether the headline gap — code that still compiles while no longer demonstrating the surrounding claim — is covered by anything, WHEN they read limit 1, THEN it says the gap exists, says no mechanical test can close it and why (it is semantic, not mechanical), and names HS-P0024's friction log as the non-substitutable instrument — AND no test, fixture, metric or count anywhere in this PR gestures at covering it, because a decorative instrument is what makes a reader delete the real one."
  satisfied: true
  evidence: |-
    `xtask/src/narrative.rs:17-27`. The bullet states the gap, says **"no mechanical test can
    close it"** and says why — "the gap is semantic rather than mechanical, so there is nothing
    for a checker to compare" — adds "None is written, and writing one that gestured at it
    would be worse than none — a decorative instrument is what gets the real one deleted", and
    names the instrument: "the real instrument is HS-P0024's friction log, and nothing in this
    repository substitutes for it". Asserted by
    `::tests::limit_one_says_no_mechanical_test_can_close_it`, which requires both the
    inherently-untestable clause and the word `semantic` inside limit 1's own bullet. The
    absence half is diff-observable and is the point of the row: **no test, fixture, metric or
    count in this PR is named for limit 1.** Precedent followed:
    `xtask/src/constitution.rs:20-25` documents exactly this class of limit with nothing behind
    it because nothing is possible. One correction the record forced: limit 1 is worded
    *narrower* than Note 10's phrasing invites, because `observed-failure-falsification` F1
    measured that a false-but-compiling assertion **does** fail the gate — what the machine
    cannot see is a claim the prose makes that the fence never asserts.
  mount_point: "`xtask/src/narrative.rs` — limit 1 lives in the harness, following `xtask/src/constitution.rs:20-25`'s worked precedent for a limit documented with no test behind it"
  verifying_test: "the per-limit presence test asserts limit 1 carries an explicit inherently-untestable clause (`cargo test -p xtask`); the absence half is verified by diff review — no test is named for limit 1"
- id: AC-005
  criterion: "GIVEN a contributor who has read two mutually contradicting claims about `RUSTDOCFLAGS` in this repository and upstream, WHEN they read limit 3, THEN it states what the probe did, re-run against the narrative compile step as `REQUIRED` declares it on the pinned 1.97.1 toolchain, cites `_limits-evidence.md` for the three verbatim transcripts (default-on lint under the step's `RUSTDOCFLAGS`; the same fence with it removed; a clippy-only lint), and names the upstream reports as context rather than as the finding — AND every probe fence is reverted in the same PR, so the transcripts persist and the fences do not."
  satisfied: true
  evidence: |-
    `xtask/src/narrative.rs:35-49`, and `_limits-evidence.md` § *Limit 3 — the `RUSTDOCFLAGS`
    probe, re-run against the step as wired*. The probe was run against the step as `REQUIRED`
    declares it (`RUSTDOCFLAGS=-D warnings cargo run --locked --quiet -p xtask --
    narrative-doctests`, `xtask/src/main.rs:496-509`) on `rust-toolchain.toml`'s pinned 1.97.1,
    not a hand-typed approximation. **Measured: `-D warnings` reaches nothing inside a narrative
    fence** — a `non_snake_case` violation compiled and ran with no diagnostic and exit 0 (a),
    identically with the variable removed (b), identically with the extra `cargo run -p xtask`
    hop taken out (b′, which exonerates the hop the module previously blamed), and
    `unwrap_used` was likewise unenforced (c). A fourth transcript is the control that makes
    the probe non-vacuous: plain `rustc --edition 2024` emits
    `warning: variable `notSnakeCase` should have a snake case name` on the same snippet, so
    the stimulus fires. This is EC-001's case — the re-run agrees with neither
    `constitution.rs:31-36` nor the upstream reports as stated — and it is recorded rather than
    reconciled; limit 3 is not softened into "may not", the upstream issues are named as
    context, and the merged-doctest mechanism is explicitly **not** claimed as the cause.
    Presence half: `::tests::limit_three_states_the_measurement_and_cites_the_record` requires
    `_limits-evidence.md`, `1.97.1` and `non_snake_case` inside limit 3's own bullet. Every
    probe fence was reverted in the same change: `git status --porcelain` carries no `docs/`
    entry and `cargo xtask ci` is green on the committed tree.
  mount_point: "`xtask/src/narrative.rs` (limit 3's prose) measured against the narrative compile step as wired in `REQUIRED` (`xtask/src/main.rs:105`, shape copied from `:488-492`)"
  verifying_test: "recorded evidence, not a `#[test]`: three verbatim transcripts in `.bklg/docs-that-teach/checked-documentation-surface/documented-blind-spots-and-their-proofs/_limits-evidence.md`, each naming its exact command and `rust-toolchain.toml`'s pin; plus the presence test asserting limit 3 cites the record path, and `cargo xtask ci --fast` green with no probe fence remaining"
- id: AC-006
  criterion: "GIVEN a reader who is told a Rust example tagged `text` is invisible to the check, WHEN they want to know whether that is a real hole or a cautious sentence, THEN a retained fixture page under `TREE` whose ` ```text ` fence is deliberately false about the library has been walked through the whole gate and observed to pass — the compile step never sees it, the fence walk permits the tag, no problem line is emitted — with the run recorded verbatim in `_limits-evidence.md`, reachable by one non-interactive command with no manual step."
  satisfied: true
  evidence: |-
    `docs/text-fences.md`, retained, registered at `xtask/src/narrative.rs:76-86`, and
    `_limits-evidence.md` § *Limit 5 — the `text` fence, walked rather than asserted*. Its only
    fence is tagged ` ```text ` and its body — `let store = MemoryEventStore::new();
    assert_eq!(store.len(), 7);` — is deliberately false about the library: exactly the break
    that failed the gate with an assertion panic when `observed-failure-falsification` put it
    inside a ` ```rust ` fence. Walked through the **whole** gate and observed to pass:
    `cargo xtask ci --fast` exits 0 with `=== every narrative page is checked ===` printing
    `  2 pages, all consistent` and `=== the narrative tree's examples compile ===` printing
    `  1 page(s)' examples enumerated` — the compile step never sees the fence, the walk
    permits the tag, and no problem line is emitted about it. Both transcripts are quoted
    verbatim in the record with their exact commands. EC-003 did **not** fire: the fixture is
    not flagged, so limit 5 is real and this row keeps its subject.
  mount_point: "the retained fixture page under `TREE` (`docs/`), registered by `include_str!` in `xtask/src/narrative.rs`, walked by both narrative steps inside `cargo xtask ci`"
  verifying_test: "end-to-end: `cargo xtask ci --fast` over the tree containing the fixture, output recorded verbatim in `_limits-evidence.md`; narrower: `cargo test --locked -p xtask --doc` and the narrative checker subcommand, both green"
- id: AC-007
  criterion: "GIVEN a future contributor who teaches the fence walk to inspect `text` fences and closes limit 5, WHEN they run the gate, THEN a `#[cfg(test)]` test that runs the real checker over the fixture and asserts no problem is reported for the `text` fence fails, with a message saying the limits section is now wrong and limit 5 must be deleted in the same change — so the section rots loudly rather than silently."
  satisfied: true
  evidence: |-
    `xtask::lint_narrative::tests::the_text_fixture_is_not_flagged_by_the_real_fence_walk`. It
    reads the real `docs/text-fences.md`, drives it through the real `check_page` by way of the
    module's own `walk` helper with the production `IGNORE_ALLOWANCES`, and asserts the walk
    reports **nothing**. Two things stop it being decorative: it first asserts the fixture still
    carries a ` ```text ` fence, so it cannot pass vacuously against a page somebody edited; and
    its failure message is an instruction rather than a complaint — "limit 5 is closed: the
    fence walk now reports a `text` fence. Delete limit 5 from `xtask/src/lint_narrative.rs`'s
    `# What this does not verify` section in this same change, and delete this test with it —
    the limits section is now wrong." Run by the `tests` step (`xtask/src/main.rs:145-155`) on
    every `cargo xtask ci`, so the day limit 5 closes the build says so and says which sentence
    to delete. That is RS-81-1's second half made mechanical
    (`standards/rust/81-checks-that-cannot-be-types.md:11`).
  mount_point: "the `#[cfg(test)]` block of the bin-crate checker module (`xtask/src/lint_narrative.rs`), run by the `tests` step (`xtask/src/main.rs:143`)"
  verifying_test: "the positive pinning test in `xtask/src/lint_narrative.rs` → `mod tests` running the real checker over the fixture and asserting no problem for the `text` fence (`cargo test -p xtask`); its failure message reviewed against `standards/rust/81-checks-that-cannot-be-types.md:11`"
- id: AC-008
  criterion: "GIVEN a contributor who has just watched the gate fail and is trying to map the filename in the output onto the page they broke, WHEN they read limit 4, THEN its wording is quoted from the run `observed-failure-falsification` recorded — which file the failure names, that the page is resolved by module name and the line inside the page — and where that record disagrees with the architecture brief's forecast (Note 3), the limit says what the record says."
  satisfied: true
  evidence: |-
    `xtask/src/narrative.rs:50-66`, worded from the run and not from the forecast, with the
    forecast-vs-record reconciliation tabulated in `_limits-evidence.md` § *Limit 4 — the
    forecast, the record, and which one won*. **The record won, and Note 3 was wrong about its
    subject** (EC-004): the report does not name the harness file at all — it names
    ``xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 9)``, the
    page's own repo-relative path reached through the harness's *directory*, with `(line 9)`
    the fence's opening line rather than the failing statement at page line 13. Confirmed by
    `observed-failure-falsification`'s `_falsification.md` §§ *What the failure actually
    identified* / findings F4 and F5, and independently by
    `cargo test --locked -p xtask --doc -- --list`. Note 3's other two halves — module resolves
    the page, line resolves inside the page — are confirmed in kind and corrected in grain, and
    limit 4 says so. It also carries what the record found that the forecast never considered:
    the panic's own `file:line` is unstable and unusable, a temp bundle file without
    `--show-output` and the page's path with a rustdoc-internal line number with it. The
    consequential sentence is the one a reader acts on: the only stable identifier is the
    doctest's module name.
  mount_point: "`xtask/src/narrative.rs` — limit 4 in the harness, since the degraded pointer is a property of the `include_str!` compile mechanism"
  verifying_test: "traceability review: limit 4's wording matched against the verbatim output recorded by `.bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/`, with the forecast-vs-record reconciliation written into `_limits-evidence.md`"
- id: AC-009
  criterion: "GIVEN a later change that quietly weakens the honesty section — moves it below another heading, drops a limit, or hedges limit 6 with a \"but the gate does check…\" clause — WHEN the gate runs, THEN it fails, because the section tests were written against those three named wrong implementations rather than against the mere existence of a heading."
  satisfied: true
  evidence: |-
    Four negative tests in `xtask/src/lint_narrative.rs`'s `#[cfg(test)]` block, each feeding
    a mutation of the **real** module text through `limits_problems` and requiring a
    distinguishable message — the three the criterion names, plus AC-003's negative direction:
    (1) `::tests::a_limits_section_moved_below_another_heading_is_rejected` prepends a `# How
    the harness works` section to the real harness source and requires `must come first`;
    (2) `::tests::a_module_missing_one_of_its_limits_is_rejected` deletes limit 2's whole bullet
    with `without_bullet` — asserting first that the bullet was there to delete and then that
    it is gone, so the mutation cannot be a no-op — and requires `limit 2 is missing`;
    (3) `::tests::a_hedged_teaching_sentence_is_rejected` appends "but the gate does check every
    fence in the tree" to limit 6 and requires `hedged with`;
    (4) `::tests::a_module_restating_the_other_modules_limit_is_rejected` splices limit 4's
    claim into the checker and requires `restates limit 4`. Each message is distinct and each
    names the limit by number. Run by the `tests` step on every `cargo xtask ci`. The plausible
    wrong implementation they reject as a set is the one `discover.md` names for this story: a
    limits section that lists blind spots in prose with nothing able to fail.
  mount_point: "the `#[cfg(test)]` block of `xtask/src/lint_narrative.rs`, run by the `tests` step (`xtask/src/main.rs:143`)"
  verifying_test: "three negative tests (or three assertions with distinguishable messages) in `xtask/src/lint_narrative.rs` → `mod tests`, one per named wrong implementation (`cargo test -p xtask`), in the house test shape of `xtask/src/lint_constitution.rs:828-878`"
- id: AC-010
  criterion: "GIVEN the project is being called done, WHEN a reviewer asks whether every delivered check has its limit on the record, THEN `_limits-evidence.md` carries the reconciliation — pinned tree, empty-tree guard, bidirectional registration, fence discipline, allowance sweep, hidden markers, citation resolution, frozen-MUST pin, count agreement — each with its limit in the section or a written reason it has none, plus a written disposition for the `compile_fail` candidate seventh (added, or not-applicable because the delivered fence walk does not permit `compile_fail`)."
  satisfied: true
  evidence: |-
    `_limits-evidence.md` § *The completeness reconciliation* — a twelve-row table, one row per
    delivered check, each carrying either the limit's location in a module's section or a
    written reason it has none. The nine the criterion enumerates are all there: pinned tree
    (none of its own, with the reason), empty-tree guard (none, with the reason), bidirectional
    registration (stated, and its registration-guarantee half pointing at limits 1-4), fence
    discipline (stated, limit 5), allowance sweep (stated), hidden markers (stated three ways),
    citation resolution (stated four ways), frozen-MUST pin (stated three ways), count
    agreement (stated as new finding **L2**), plus fence compiling itself and — added on review,
    the twelfth row — the compile step's **attribution**. That row is **EC-005** taken as
    addition: `observed-failure-falsification`'s `_falsification.md` **F2** measured that a
    broken narrative fence fails under `=== tests ===` at index 2 and under neither of the
    tree's banners, and that residual had no limit anywhere. It is now the seventh bullet of
    `xtask/src/narrative.rs`'s `# What this does not verify`, a seventh `NOTE_TEN` entry, and
    two tests (`limit_seven_names_the_step_that_fails_first_and_cites_the_run` and the mutation
    arm `a_dropped_seventh_limit_is_rejected`), with
    `narrative_doctests::tests::the_steps_that_compile_this_tree_are_pinned_in_gate_order`
    pinning the three `REQUIRED` steps that compile the tree so the bullet and the array cannot
    drift. The ordering *defect* is routed to `FU-1` of `HS-P0020` rather than repaired here
    (EC-009), and recorded as finding **L6**. **The `compile_fail`
    candidate seventh is disposed of in writing: not applicable, and not one of the bullets** —
    the delivered walk permits `compile_fail` (`is_rustdoc_tag`, `xtask/src/lint_narrative.rs`)
    *and* goes further than the research's concern by making a `compile_fail` fence name its
    error code in prose, which is the mitigation the limit asks for; no page in `docs/` uses it
    today, and the paragraph is written so a future page that does can re-open it. Three
    findings the reconciliation itself turned up are recorded as **L1** (two sentences in
    `xtask/src/narrative_doctests.rs` the runs measured to be false, corrected in this change),
    **L2** (the compile step's coverage number counts doctests, not registrations — `2
    pages, all consistent` beside `1 page(s)' examples enumerated`) and **L6** (F2's residual
    unstated, and a third false sentence in the same module — the ordering claim on
    `the_narrative_step_precedes_the_constitution_step`, which L1's sweep missed because it read
    the module's limits bullets and not its test doc comments). EC-009 did not fire: no
    delivered check has a limit nobody could state.
  mount_point: "`.bklg/docs-that-teach/checked-documentation-surface/documented-blind-spots-and-their-proofs/_limits-evidence.md`, reconciled against the limits sections at both mounts"
  verifying_test: "review artefact: the reconciliation table in `_limits-evidence.md`, one row per delivered check, checked against `_decomposition.md` Note 10 and `_design.md` `## The states the API must express`"
- id: AC-011
  criterion: "GIVEN any reader of anything this project produced — module docs, the fixture page, `docs/README.md`, a step name, a commit message — WHEN they look for a claim that the surface proves a page teaches, THEN there is none: limit 6 is one unhedged sentence in both modules, and no badge, tick, shield or \"verified\" mark appears anywhere, nor any `book.toml`, `book/`, `site/` or `.css` that would create a second rendered surface able to carry one."
  satisfied: true
  evidence: |-
    Limit 6 is one unhedged sentence in both modules — `xtask/src/narrative.rs:67-71` ("This
    step says nothing about whether any page teaches anybody anything… comprehension is
    HS-P0024's friction log, and no run of this step substitutes for it") and
    `xtask/src/lint_narrative.rs:114-118`, the same sentence for the same reason. Mechanical
    halves: `::tests::neither_module_carries_a_mark_claiming_the_documentation_is_checked`
    scans the shipping half of both modules for the four forbidden marks and fails on any of
    them; `limits_problems`'s hedge arm rejects ` but `, ` however`, ` although`, ` except` and
    ` unless ` inside limit 6's own bullet, driven by
    `::tests::a_hedged_teaching_sentence_is_rejected`; and the pre-existing
    `::tests::nothing_in_the_module_claims_a_page_teaches` still holds. Second-surface half,
    checked by listing rather than by test: `git ls-files` matching `.css`, `book.toml`,
    `book/`, `site/` or `_site/` returns **0** rows, recorded in `_limits-evidence.md` §
    *What this record does not establish*. Prose scan of the whole diff — both module docs, the
    fixture page, `docs/README.md`, the new step-free wiring and the commit message — finds no
    sentence claiming the surface proves a page teaches; `docs/text-fences.md` says the
    opposite in as many words, that nothing in the gate reports the false fence it carries.
  mount_point: "the whole PR surface: both module docs, the fixture page and `docs/README.md` under `TREE`"
  verifying_test: "prose scan of the PR diff against `project.md` DoD item 8 and `_design.md` `## Anti-patterns` 9 and 10 (review, by construction not a `#[test]`), plus the presence test asserting limit 6 present and unhedged in both modules (`cargo test -p xtask`)"
- id: AC-012
  criterion: "GIVEN a reader who lands on `docs/README.md` and follows a row to the new fixture page, WHEN the page renders at the narrow width, THEN it is a conforming `narrative-page` in the signed-off design's terms: registered in the harness in both directions, no `HIDDEN_MARKERS` token anywhere, its repo-relative path ≤ 32 characters with no third directory level, H1 ≤ 40 characters, fences ≤ 80 columns, page ≤ 250 source lines, an index row in the two-column table above the pointer-out table — and it adds one row without reordering the rows already there."
  satisfied: true
  evidence: |-
    `docs/text-fences.md`, measured against `_design.md` `## Density budget` and
    `## Composition`. Mechanical half, run over the fixture by the checker on every
    `cargo xtask ci`: registered in both directions (`check_registration` — `include_str!` and
    `mod text_fences {` at `xtask/src/narrative.rs:76-86`), no `HIDDEN_MARKERS` token
    (`check_hidden_markers`), repo-relative path **19** characters against the budget of 32 and
    no directory level under `docs/` (`check_paths`) — the whole tree reports
    `2 pages, all consistent`. Asserted independently by
    `::tests::the_text_fixture_is_registered_in_both_directions_and_inside_the_budget`, which
    checks both registration directions, the path budget and the directory level. Reviewed
    half, counted rather than eyeballed: H1 `Fences the compiler never sees` is **30**
    characters against a budget of 40; the widest line on the page is **79** columns against a
    fence budget of 80; the page is **27** source lines against a budget of 250. Composition
    follows `## Composition`'s region order — H1, the reserved answered-need slot left empty for
    HS-P0021, the claim band, the fence band with its claim immediately above it, no scope band,
    no closing pointer. Index: one appended row in the existing two-column table, above the
    pointer-out table, with the row already there left where it was —
    `::tests::the_text_fixture_has_an_index_row_and_reorders_nothing` asserts the ordering
    directly, and milestone 1's `the_narrative_table_precedes_the_pointer_out_table` still holds.
  mount_point: "the fixture page under `TREE` (`docs/`), its `include_str!` registration in `xtask/src/narrative.rs`, and its row in `docs/README.md`'s two-column narrative table"
  verifying_test: "the checker's own registration, hidden-marker and path-budget checks run over the fixture via `cargo xtask ci --fast` (mechanical half); review against `_design.md` `## Density budget`, `## Composition` and `## Anti-patterns` 1 and 11 (the H1, fence-width and page-length caps, which are review rules by `_design.md` `## Open questions` 2)"
```
