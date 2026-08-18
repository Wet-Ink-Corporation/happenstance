---
title: "Review — The Checked Documentation Surface"
initiative_slug: docs-that-teach
project_slug: checked-documentation-surface
terminal: false
overall: 3
dod_green: true
rubric:
  ac-coverage: 3
  integration-reachability: 3
  test-integrity: 3
  gate-greenness: 3
  brief-fidelity: 3
  intent-fidelity: 2
  presentation-fidelity: 0
---

# Review — The Checked Documentation Surface

`HS-P0020`, the **non-terminal** first project of `docs-that-teach`. This is the
project-grain review gate, run adversarially against the cumulative diff
`6d56d0d...HEAD`. Nothing below is taken from a story's report on trust: every
claim marked *re-falsified here* was broken on purpose in this worktree during
this review, observed failing, and restored — the working tree is clean
afterwards.

## Verdict

approved

## Rubric

Each dimension scored 0 (absent) to 3 (excellent). `approved` requires every
dimension >= 2, with `gate-greenness` = 3, `integration-reachability` = 3,
`intent-fidelity` >= 2, and the applicable Definition-of-Done bar green.

`presentation-fidelity` is the single exception to "every dimension >= 2", and
the exemption is narrow. Here it **does not apply**: `.redkiln/config.yaml`
declares no `design:` block at all, deliberately and per
`.redkiln/config.yaml:75-81` and `CLAUDE.md` — this is a Rust library with no
app to screenshot, so no capture was taken and no `_design-review.md` exists. It
scores **0 and is exempt from the bar, never from the record**: presentation was
**NEVER OBSERVED**. It is not 3 on the grounds that nothing was found.

| Dimension | Score | Rationale |
| --------- | ----- | --------- |
| ac-coverage | 3 | All ten project ACs (`project.md:196-233`) are met by real, reachable behaviour, and five were re-falsified live in this review rather than read off an evidence file. AC-002/AC-003: breaking the fixture fence made `cargo run -p xtask -- narrative-doctests` fail as `error[E0599]: no method named length` naming `docs/append-conditions.md - narrative::append_conditions (line 9)`; flipping its assertion produced an assertion panic (`left: 0 / right: 1`), which is what proves fences are **run**, not merely compiled. AC-004/005/006/007: one probe page produced six distinct problems by file and line — the `IGNORE_ALLOWANCES` message, both registration directions, `<details`/`<summary` under DT-7, and a dangling `ES-9999`. AC-008: perturbing the `VT-15` anchor in `crates/happenstance-core/src/tag.rs` produced "the discharging text moved"; restoring returned `2 pages, all consistent`. AC-001 is structural (`lint_narrative.rs:231` `TREE`, `?`-propagated at `:417`, plus `guard_not_vacuous`) and was falsified by `git mv` in `_integration.md:64`. |
| integration-reachability | 3 | Nothing constructed-but-unmounted. Both capabilities are `REQUIRED` steps with `probe: None` (`xtask/src/main.rs:482-510` and `:525-553`); the checker is also in `lint_steps()` (`:882`) and on `affected.rs`'s unconditional file-reading list (`affected.rs:131`); both are dispatched as subcommands (`main.rs:742`, `:746`) and listed in `print_help` (`:829-837`). The harness is declared from the **lib** target (`xtask/src/lib.rs:36`), the only mount where `cargo test --doc` reaches it. Verified running, not just wired: `cargo test -p xtask --doc` lists and passes `narrative::append_conditions (line 9)`; `cargo run -q -p xtask -- narrative` prints `2 pages, all consistent`; `cargo xtask affected --base main` selects `xtask` for a prose-only change and exits 0. `spec_trace::clause_ids` is consumed in production at `lint_narrative.rs:1821`, not only by tests. |
| test-integrity | 3 | No test deleted, gutted, skipped or flag-gated; no `#[ignore]`, `todo!()` or `fixme` anywhere in the diff (the three `ignored` doctests in the run are pre-existing constitution atoms). The single rename — `a_docs_only_change_selects_nothing` to `a_top_level_prose_file_selects_nothing` (`affected.rs:684-690`) — keeps the assertion verbatim over `RUNBOOK.md` and is accompanied by a strictly stronger addition, `the_narrative_tree_is_no_longer_inert` (`:771-775`), which asserts on the `is_inert` predicate precisely because the two directional tests both pass while `"docs/"` sits shadowed on `INERT`. 178 unit + 168 lib doctests + 63 constitution doctests pass. The limits pin (`NOTE_TEN`, `lint_narrative.rs:4301`) is mutation-tested (`a_dropped_seventh_limit_is_rejected`, `a_module_missing_one_of_its_limits_is_rejected`, `a_hedged_teaching_sentence_is_rejected`). `IGNORE_ALLOWANCES` ships **empty** (`:296`) — the strictest posture, not a fixture that pre-answers the check. |
| gate-greenness | 3 | Affected-package scope only, all executed in this worktree at `3539d83` on a clean tree: `cargo test -p xtask --all-features` (178 + 168 + 63 passed, 0 failed); `cargo clippy -p xtask --all-targets --all-features -- -D warnings` exit 0; `cargo fmt -p xtask -- --check` exit 0 (formatter proven green); `cargo xtask affected --base main` exit 0, `affected gate passed`, one package selected (`xtask`), with `=== formatting ===`, `=== clippy (affected packages) ===`, `=== tests (affected packages) ===` and `=== the file-reading checks ===` all green; `cargo run -q -p xtask -- lint-constitution` gave `27 atoms, all consistent`, which is what proves the four `standards/rust/**` evidence-line updates in this diff are correct. No Playwright project exists in this repository, so the collection-only pass does not apply. |
| brief-fidelity | 3 | Every architecture-brief obligation lands at the named shape: pinned `const` plus vacuity guard (AC-001); `REQUIRED` + `probe: None` + `--locked` + step-local `RUSTDOCFLAGS` (AC-002, RS-80-1/2/3/4); one module per page (AC-003); the `check_fences`-shaped walk (AC-004); the bidirectional `check_harness` copy (AC-005); ids resolved *through* `spec_trace` rather than re-parsed (AC-007); a re-derived enumerated `const` (AC-008). The testing brief's four tiers are all present, including the one AC whose proof is a recorded procedure rather than a `#[test]`. Deployment brief Option A is what `_design.md` D1 chose. No Accepted KB decision is deviated from: `spec/SPECIFICATION.md` untouched, `standards/rust/README.md`'s precedence chain not extended, `.kb/` untouched, and **no dependency added** — `xtask/Cargo.toml` and `Cargo.lock` are absent from the diff entirely, honouring `xtask/Cargo.toml:16-21` (DR-12). |
| intent-fidelity | 2 | Design intent is honoured mechanically rather than asserted: `_design.md` D2 (signed off at `:786` with no conditions) forbids hidden panels and `HIDDEN_MARKERS` (`lint_narrative.rs:327`) rejects all seven tokens by file and line with deliberately **no** allowance list; D1's "the markdown is the render" is realised as zero build artefacts, confirmed by breaking and restoring a page with no residue to clean up. The interaction lens passes: non-occlusion (nothing folded, re-falsified here); in-place provenance (`ES-40` inline in the sentence that depends on it, `docs/append-conditions.md:7`); preserved position (`docs/README.md`'s pointer-out table survives intact and in order, the narrative table added *above* it); reversibility (a checkout alone restores green); plain-text reachability (every load-bearing string is greppable, every page has an index row). **Held at 2 by one measured residual:** finding **F2** of `_falsification.md` disproved the interaction claim story AC-004 was written for — under `cargo xtask ci` a broken narrative fence fails at `=== tests ===` (`REQUIRED` index 2), so neither of the tree's own banners prints. The response is honest and enforced (limit 7 in `xtask/src/narrative.rs:67-82`, pinned by `NOTE_TEN`) and the reader still gets the page and module by name, but the repair is routed to `FU-1` with **owner: unassigned**, and `xtask/src/main.rs:486-487` and `:536-538` still state the disproved claim. |
| presentation-fidelity | 0 | **Presentation was NEVER OBSERVED.** The design review **does not apply**: `_design.md` declares five surfaces, but `.redkiln/config.yaml` declares no `design:` block and therefore no `design.capture` command, deliberately (`.redkiln/config.yaml:75-81`, `CLAUDE.md`) — there is no app to screenshot. No `_design-review.md` exists and no capture was taken, so there is no perceptual evidence to score. Scored 0 and **exempt from the >= 2 bar** — exempt from the bar, never from the record. The written design record is therefore the only record, which `_design.md:775-777` states in its own sign-off. |

## Evidence

### Project AC coverage map

| AC | Met by reachable behaviour? | Evidence |
| --- | --- | --- |
| AC-001 — the tree is pinned, not conventional | **yes** | `xtask/src/lint_narrative.rs:231` `TREE = "docs"`, read by `collect` (`:417`) with a `.with_context` naming the path and `?`-propagated; `guard_not_vacuous` (`:487`) makes an emptied tree a hard error; `tests::a_missing_tree_is_an_error_naming_the_pinned_path` (`:1954`), `::an_empty_tree_is_a_hard_error` (`:1968`), `::a_tree_holding_only_its_index_is_still_vacuous` (`:1984`). Falsified by moving the tree in `_integration.md:64` |
| AC-002 — every fence compiled against the real crates, mandatorily | **yes** | `xtask/src/narrative.rs:122-135` (one `#[cfg(doctest)] mod` per page) declared from `xtask/src/lib.rs:36`; `REQUIRED` step at `xtask/src/main.rs:482-510`, `probe: None` (`:509`), step-local `RUSTDOCFLAGS` (`:508`); `xtask/src/narrative_doctests.rs:96-113` asserts the doctests out of `--list` before running them. **Re-falsified here:** `store.len()` to `store.length()` gave `E0599` naming the page and module; restored, green |
| AC-003 — the check has been seen to fail | **yes** | `observed-failure-falsification/_falsification.md` runs 1/2/3 at `6368e2b` — `cargo xtask ci` exit 1 with an **assertion panic**, then exit 0 after revert, both halves written down. **Re-falsified here:** the assertion flipped to `1` produced `assertion left == right failed / left: 0 / right: 1`, confirming the fences are run and not merely compiled |
| AC-004 — no silent opt-out | **yes** | `check_fences` (`lint_narrative.rs:762`), `check_allowances` (`:936`), `IGNORE_ALLOWANCES` empty (`:296`); `tests::every_ignore_spelling_rustdoc_accepts_needs_an_allowance` (`:2456`), `::a_comment_above_an_ignore_fence_does_not_permit_it` (`:2615`), `::a_stale_allowance_is_a_problem` (`:2683`), `::an_untagged_fence_is_rejected` (`:2398`). **Re-falsified here** on a probe page: `docs/zz-review-probe.md:8 — an ignore fence needs an IGNORE_ALLOWANCES entry naming it` |
| AC-005 — no orphan pages | **yes** | `check_registration` (`lint_narrative.rs:528`) reading `HARNESS` (`:249`) in both directions, plus `narrative_doctests::enumerated_pages` (`:160`) for the lib/bin mis-mount. **Re-falsified here:** the probe page produced **two** distinct problems — `does not include zz-review-probe.md; its examples are never compiled` and `no mod zz_review_probe` |
| AC-006 — hidden content inside the check, or absent | **yes**, on the *absent* branch, mechanically enforced | `_design.md:204-228` D2 resolves DT-7 (signed off `:786`, no conditions); `HIDDEN_MARKERS` (`lint_narrative.rs:327`), `check_hidden_markers` (`:997`), reached from `check_page` (`:1248`), with no allowance list by decision (`:307-315`). **Re-falsified here:** `docs/zz-review-probe.md:5 — <details is a hidden panel; DT-7 forbids it in docs`, and the same for `<summary` |
| AC-007 — citations resolve | **yes** | `clause_citations` (`lint_narrative.rs:1084`), `check_citations` (`:1189`), resolving through `spec_trace::clause_ids` (`xtask/src/spec_trace.rs:1803`) consumed at `lint_narrative.rs:1821`. **Re-falsified here:** `cites ES-9999, which SPECIFICATION.md does not define`; the positive half is `docs/append-conditions.md:7`'s `ES-40`, green |
| AC-008 — the frozen documentation MUSTs are pinned | **yes** | `FROZEN_DOC_MUSTS` (`lint_narrative.rs:1346`) — one enumeration, 21 entries, 9 `Pinned` with site and verbatim anchor, 12 `Excluded` with a reason — guarded by `guard_pin` (`:1630`, refusing an empty pin, a duplicate, and an id-as-anchor) and checked three ways: resolution (`:1665`), anchor presence (`:1682`), and a derived-versus-hand-written **census** (`:1697`) that names which side moved. `cargo xtask narrative` green; `cargo xtask spec-trace` green (`_integration.md:49`). **Re-falsified here** on `VT-15`. `ES-26`, `PS-31`, `PS-36` are excluded as undischarged and routed to HS-P0023 (`frozen-documentation-must-pin/_rederivation.md:117-122`) |
| AC-009 — clean checkout, no manual step | **yes** at this project's grain | `_integration.md:45-46`: `cargo xtask ci --fast` exit 0 and full `cargo xtask ci` exit 0, 26 banners, **0** `skipped` lines, both narrative banners inside it. No dependency and no build artefact was added — `xtask/Cargo.toml`, `Cargo.lock`, `book.toml`, any `.css` and `book/` are all absent from the cumulative diff, which is D1's "the markdown is the render" made structural (`_design.md:169-179`). The **fresh-clone** half is DoD-1 and is correctly deferred to the terminal project |
| AC-010 — the limits are on the record | **yes** | `xtask/src/narrative.rs:3-88` opens with `# What this does not verify` and states seven limits, including limit 1 (compiles-but-no-longer-demonstrates, with "no mechanical test can close it" stated unhedged) and limit 3 (`RUSTDOCFLAGS`, **measured on 1.97.1** — three transcripts in `documented-blind-spots-and-their-proofs/_limits-evidence.md`, not cited from upstream); `lint_narrative.rs:10-120` states the walk's limits. Held by `NOTE_TEN` (`:4301`) with owner/claim/instrument per limit, additive-only, plus `tests::nothing_in_the_module_claims_a_page_teaches` (`:2377`) and the hedge scan (`:4346`) — all run by `cargo test -p xtask` inside the gate |

### Gate and DoD results

- **Affected-package gate (scoped to `xtask`, never the whole repo).**
  `cargo test -p xtask --all-features` — 178 unit + 168 lib doctests + 63
  constitution doctests, 0 failed. `cargo clippy -p xtask --all-targets
  --all-features -- -D warnings` — exit 0. `cargo fmt -p xtask -- --check` —
  exit 0. `cargo xtask affected --base main` — exit 0, `affected gate passed`,
  one package selected. `cargo run -q -p xtask -- lint-constitution` — `27
  atoms, all consistent`. Playwright collection check: not applicable, no
  Playwright project in this repository.
- **This project's integration bar.** `dod_green: true` and
  `reachability_ok: true` in [`_integration.md`](_integration.md), whose
  reachability map traces every delivered capability to `REQUIRED`
  (`main.rs:107`), the dispatch arms (`:742`, `:746`), `lint_steps()`
  (`:874-884`) and `affected.rs:131`. All ten project ACs and all eight boundary
  Definition-of-Done items executed and passed; none skipped, fixme'd or
  flag-gated.
- **Whole-initiative Definition of Done.** The fifteen journeys are owned by the
  terminal project `HS-P0025` and appear under `deferred_scenarios`
  (`_integration.md:8-23`). They are deferred, not failed, and are not held
  against this project: `project.md:132` assigns re-observing all fifteen on the
  assembled tree to HS-P0025, and eight of them require teaching content this
  project is forbidden to write (`project.md:294`).

### Escape-hatch, unmounted and scope-drift findings

- **No escape hatch found.** No `#[ignore]`, `todo!()`, `unimplemented!()`,
  production `#[allow(...)]`, skipped scenario or flag-gated-off path anywhere in
  the cumulative diff. No double, fixture or no-op stands in for a real-runtime
  AC: the compile step is the mechanism itself, not a test about it, and the
  checker's reject paths were re-falsified live against the real tree rather
  than only against fixture strings.
- **No unmounted deliverable.** Every capability is on the gate's real path and
  was observed running there. `spec_trace::clause_ids` has exactly one
  production consumer, and the gate's `-D warnings` makes a re-added
  `#[expect(dead_code)]` a build failure rather than a silent orphan.
- **No scope drift.** The cumulative non-backlog diff is 14 files: `docs/` (3),
  `standards/rust/` (4 — evidence line numbers only, proven correct by
  `lint-constitution`), and `xtask/src/` (7). `spec/SPECIFICATION.md`, `.kb/`,
  `.github/`, `Cargo.toml` and `Cargo.lock` are untouched, so every out-of-scope
  boundary in `project.md:123-140` holds.
- **Carried forward, non-blocking — the finding that keeps intent-fidelity at
  2.** `xtask/src/main.rs:486-487` and `:536-538` still state that the two
  compile steps' ordering is "the whole of what keeps a broken narrative fence"
  attributed to the narrative corpus / under the narrative banner — a claim this
  project's own finding **F2** disproved, and which `431c8b0` corrected in
  `xtask/src/narrative_doctests.rs` while deliberately not touching `main.rs`.
  The limit itself is discharged where it matters (`xtask/src/narrative.rs:67-82`,
  held by `NOTE_TEN`), no behaviour depends on either comment, and
  `_integration.md:154-172` records both. It does not block, because the
  load-bearing statement a reader is sent to is correct and enforced. But
  `FU-1`'s owner line reads `unassigned` (`project.md:356`), which is the exact
  shape `project.md:346-348` warns about.
- **Named and routed, not dropped.** `ES-26`, `PS-31` and `PS-36` are `[FROZEN]`
  documentation MUSTs on the contract's own documentation that nothing
  discharges today. The pin excludes them on condition (c) rather than pinning
  an obligation nobody can satisfy, names them on the array itself
  (`lint_narrative.rs:1326-1331`), and routes each to HS-P0023 by name. That is
  a finding with an addressee, and it is the correct disposition at this grain.

### Per-story checkpoint SHAs

Derived from git, not from the seed: `git log 6d56d0d..HEAD --grep "Story:
checked-documentation-surface/"` yields the ten feature checkpoints, and the
four slice seals complete the workflow's fourteen.

| SHA | Subject |
| --- | --- |
| `7020c4c` | feat: The narrative tree exists and every fence in it compiles, mandatorily |
| `2a388ec` | feat: A prose-only change selects the package that compiles it |
| `8cb0c75` | chore: seal slice compiled-narrative-tree approved |
| `b62de17` | feat: Narrative checker mounted with pinned path |
| `0a67e50` | feat: Fence discipline and allowance list |
| `13494c7` | feat: Hidden content resolution |
| `f8b1d57` | chore: seal slice narrative-checker-discipline approved |
| `d69253c` | feat: Spec-trace clause id accessor |
| `6c2a412` | feat: Narrative citation resolution |
| `e313b46` | feat: Frozen documentation MUST pin |
| `6368e2b` | chore: seal slice specification-pin approved |
| `b0bb9bd` | feat: Observed failure falsification |
| `5ecce36` | feat: Documented blind spots and their proofs |
| `99bbdf2` | chore: seal slice falsification-and-limits approved |

Three further commits in the range are in-slice review repairs rather than
out-of-band work, and each is additive: `12d9493` (read a fence the way rustdoc
reads it), `676e64d` (assert the two declaration heuristics agree), `431c8b0`
(land F2 as the seventh limit). `3539d83` is the integration audit. There are no
`Baseline-Repair: checked-documentation-surface` or `Slice-Repair:
checked-documentation-surface` commits — no out-of-band baseline repair occurred
this run.

## Required Changes

None blocking; the verdict is `approved`. Two items are recorded for the
initiative rather than for re-review:

1. **Give `FU-1` an addressee.** `project.md:356` reads `Owner: unassigned`,
   which by the project's own standard (`project.md:346-348`) is a finding
   nobody has. It carries the gate's step-ordering question and the two stale
   comments at `xtask/src/main.rs:486-487` and `:536-538`.
2. **Carry `ES-26`, `PS-31` and `PS-36` into HS-P0023's scope explicitly.** They
   are `[FROZEN]` documentation MUSTs with no discharge today, correctly excluded
   from the pin on condition (c) and named at
   `frozen-documentation-must-pin/_rederivation.md:117-122`; DoD-11 re-checks
   them on the assembled tree at closeout.
