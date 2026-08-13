---
item: "HS-S0091"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The public surface is diffed against what is already on the registry

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
  criterion: >-
    The gate proves it without a network, and says that it does. GIVEN a maintainer on a clean
    checkout with no network and no `cargo-semver-checks` installed — the ordinary case for a
    contributor and for every `affected` run, WHEN they run `cargo xtask ci --fast`, or `cargo xtask
    affected --base main` on a diff that touches no workspace package (only `CHANGELOG.md`,
    `Cargo.toml`'s version line or a report), THEN the surface-diff step runs — it is in `REQUIRED`
    with `probe: None`, so it can never be skipped for a missing tool — completes offline from file
    reads and `git` alone, writes nothing to the tree, prints its result without displacing any
    other step's output, and the two places that state in prose what the gate proves
    (`xtask/src/main.rs`'s "What the gate proves" paragraph :8-24, and `print_help` :718-772) name
    it. A step whose existence is not in the gate's own prose is a claim shipped without its
    evidence.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — the REQUIRED array (probe: None, beside package-check at :519-532), the main dispatch (:639-706), print_help (:718-772) and the module doc's \"What the gate proves\" paragraph (:8-24); second mount at xtask/src/affected.rs:114-125"
  verifying_test: "xtask/src/surface_diff.rs::tests::check_mode_touches_no_network_and_writes_nothing; ::tests::required_step_is_mandatory_not_probed; ::tests::gate_prose_names_this_step"

- id: AC-002
  criterion: >-
    The answer is about the registry, not about the branch point. GIVEN a maintainer who wants to
    know whether the crate they are about to publish breaks the version consumers are already
    building against, and GIVEN that `.github/workflows/ci.yml`:295-316 already answers a different
    question on every PR, WHEN they run `cargo xtask surface-diff --run --date <YYYY-MM-DD>`, THEN
    the invocation carries `--baseline-version <exact published version>` and `-p <crate>` and never
    `--baseline-rev`, so a break merged several PRs ago — invisible to the branch-point job by
    construction, as `CONTRIBUTING.md`:295-301 states in its own words — is caught here. The
    maintainer can tell the two apart from the output alone, because each names its own baseline.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:639-706 — the `Some(\"surface-diff\")` dispatch arm and its `--run` flag handling, modelled on spec-trace's at :670-678"
  verifying_test: "xtask/src/surface_diff.rs::tests::baseline_is_a_published_version_never_a_rev; ::tests::break_merged_before_the_branch_point_is_still_a_finding"

- id: AC-003
  criterion: >-
    The evidence is a committed artefact a person can open, not a green badge. GIVEN a reviewer, or
    the maintainer six months later, who wants to know what was actually compared, WHEN they open
    `spec/audits/surface-diff-<YYYY-MM-DD>.md`, THEN they find a header carrying `captured-on`,
    `captured-at`, the `cargo semver-checks` version, and one row per crate with baseline version ·
    feature set · verdict · findings · release-version, followed by the `accepted-breaking` block,
    the verbatim transcripts and the "what this did not check" section — and check mode reads only
    that header, never the prose. Composition, binding: every verdict cell carries words, never a
    bare glyph or colour (`_design.md` AP-2, :646-647); every count and verdict is attached to a date
    and a version in the same block (AP-3, :648-650); the header is plain Markdown with no raw HTML,
    inline style or meaning-carrying image (AP-7, :657-658). Density: the success path prints
    exactly one line per crate — three lines, matching `xtask/src/package.rs`:138-143's
    one-line-per-crate convention, and the report's header is ≤ 12 lines plus one row per crate, so
    it fits one screen before the transcripts begin.
  satisfied: false
  evidence: ""
  mount_point: "spec/audits/surface-diff-<YYYY-MM-DD>.md — written by capture mode, read by the REQUIRED step in xtask/src/main.rs:519-532"
  verifying_test: "xtask/src/surface_diff.rs::tests::header_round_trips_through_writer_and_reader; ::tests::missing_header_field_fails_naming_the_field; ::tests::verdict_cell_carries_words_not_a_glyph; ::tests::check_mode_prints_one_line_per_crate"

- id: AC-004
  criterion: >-
    The version number is a derived fact the maintainer can be argued out of, not a number they
    typed. GIVEN a maintainer at the release gate who must choose what to publish as, WHEN check
    mode runs, THEN it recomputes each crate's implied version from that crate's baseline and
    findings — pre-release-of-the-candidate ⇒ the candidate release, provided every breaking finding
    is enumerated; stable `0.Y.Z` ⇒ `0.(Y+1).0` on any break, else `0.Y.(Z+1)` — and fails when the
    report's declared `release-version` differs, naming which side moved; and it fails when
    `happenstance` and `happenstance-core` imply different numbers, because they share one
    `[workspace.package] version` key (`Cargo.toml`:5-6) and a shared key cannot carry two numbers.
    A report implying anything outside `0.2.0` is a re-plan, not a rounded-down number
    (`_storymap.md`:186-191).
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/surface_diff.rs, invoked from the REQUIRED step in xtask/src/main.rs:519-532; reconciles against Cargo.toml:5-6 and crates/happenstance-testkit/Cargo.toml:14, read-only"
  verifying_test: "xtask/src/surface_diff.rs::tests::prerelease_baseline_implies_the_candidate_release; ::tests::stable_baseline_with_a_break_implies_a_minor_bump; ::tests::stable_baseline_without_a_break_implies_a_patch_bump; ::tests::declared_version_disagreeing_with_derived_fails_naming_both; ::tests::shared_workspace_key_implying_two_numbers_fails"

- id: AC-005
  criterion: >-
    A deliberately breaking change fails it — the instrument can fail. GIVEN project AC-002's own
    text, "Running it against a deliberately breaking change fails it" (`project.md`:229-232), and
    CLAUDE.md's corollary that a rule no adapter can fail is decorative, WHEN a public item is
    removed or a signature changed and capture mode is run, THEN the run reports a breaking verdict
    for that crate, and check mode then fails unless the finding appears in `accepted-breaking` with
    a one-line reason. Both directions fail: an unlisted finding is a break nobody read; a listed
    non-finding is an acceptance carried over from a report that no longer exists. The seeded change
    is reverted — a break the design did not anticipate blocks the release for a re-plan
    (`_design.md`:76-79), it is not repaired inside a gate-instrument PR — and the real `--run`
    transcript for both the clean and the seeded case is captured under this story's folder.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:639-706 — the capture-mode (`--run`) dispatch arm; transcripts committed under .bklg/from-contract-to-published-library/publication-and-positioning/registry-surface-diff/"
  verifying_test: "xtask/src/surface_diff.rs::tests::finding_absent_from_accepted_breaking_fails; ::tests::acceptance_with_no_matching_finding_fails; ::tests::nonzero_exit_with_zero_scraped_lints_is_schema_drift"

- id: AC-006
  criterion: >-
    When it cannot answer, it says so and stops — it never finds nothing and passes. GIVEN a
    maintainer running the gate in a state the instrument cannot evaluate — nothing published yet,
    no report, a malformed report name, a `captured-at` that is not an ancestor of `HEAD`,
    `cargo-semver-checks` absent, the registry unreachable, WHEN the step runs, THEN it exits
    non-zero with a message that names the path, the value it read, and what the reader must do
    next, in that one message — no second command to run to find out why, and no context jump into a
    log (ADR-0010's "a skip is reported, never silent",
    `.kb/decisions/0010-the-suite-must-prove-itself.md`; `_decomposition.md`:701-707). The prohibited
    pass is an empty set compared against an empty set: a locator that finds zero reports and
    reports zero problems is this story's single most likely wrong implementation. Where the cause is
    upstream, the message names the owner (`typed-layer-and-alpha-release`) rather than leaving the
    maintainer to work it out.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/surface_diff.rs check mode, reached from the REQUIRED step in xtask/src/main.rs:519-532 and from xtask/src/affected.rs:114-125's unconditional block"
  verifying_test: "xtask/src/surface_diff.rs::tests::changelog_with_only_unreleased_fails_naming_the_baseline_owner; ::tests::no_report_at_all_fails_rather_than_passing_vacuously; ::tests::report_name_that_is_not_an_iso_date_fails_not_skips; ::tests::captured_at_not_an_ancestor_of_head_fails; ::tests::every_failure_message_names_path_value_and_remedy"

- id: AC-007
  criterion: >-
    The reader is told what was not checked, and no sentence in the tree contradicts the tree. GIVEN
    a consumer or evaluator who will read a claim derived from this report, WHEN they reach the
    report, THEN it names its three blind spots — `cargo-semver-checks` does not detect breaking type
    changes, generic or lifetime changes, or breakage visible only under a feature subset
    (`standards/rust/40-public-surface-and-evolution.md`:69-71, checked 2026-08-09, rustc 1.97.1) —
    carries the tool version and the feature set each invocation used, and points at the
    feature-powerset step as the nearest neighbouring check. No absence is stated without its reason
    next to it (`_design.md` AP-13, :668-669; RS-40-5). AND `CONTRIBUTING.md`:295-301, which today
    says the registry baseline "is not available yet", is corrected in place to name this step and to
    keep the two jobs distinct — leaving it is the documentation form of a sentence that is false on
    the tree that shipped (IQ-7, `_decomposition.md`:305-310), and the same defect class as
    `_design.md`'s AP-9 known-stale strings. A `CHANGELOG.md` entry names the defect the step
    detects, matching the discipline `lint-changelog` already enforces.
  satisfied: false
  evidence: ""
  mount_point: "spec/audits/surface-diff-<YYYY-MM-DD>.md's \"what this did not check\" section; CONTRIBUTING.md:295-301; CHANGELOG.md (read by the existing lint-changelog step in xtask/src/main.rs)"
  verifying_test: "xtask/src/surface_diff.rs::tests::report_carries_tool_version_feature_set_and_limits; ::tests::contributing_no_longer_claims_the_baseline_is_unavailable"

- id: AC-008
  criterion: >-
    What is diffed is the decided crate set, and three crates mean three baselines. GIVEN
    `crate-set-decision`'s recorded answer — three crates, the four-crate alternative rejected
    (`_decomposition.md`:574-608) — and CF-32's [FROZEN] ruling that the testkit carries its own
    `version` key because "the contract's is a promise about types, the testkit's is a promise about
    the bar" (`spec/SPECIFICATION.md`:8200-8220), WHEN the step runs, THEN it diffs per crate against
    that crate's own published version and the report carries three rows, not one verdict; and the
    report's crate set must equal `PUBLISHABLE` (`xtask/src/package.rs`:86), so adding or removing a
    published crate invalidates the report rather than silently narrowing it. That is what makes the
    dependency on `crate-set-decision` mechanical rather than editorial.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/package.rs:86 PUBLISHABLE (visibility widened to pub(crate), read-only), consumed by xtask/src/surface_diff.rs and reached from the REQUIRED step in xtask/src/main.rs:519-532"
  verifying_test: "xtask/src/surface_diff.rs::tests::report_crate_set_must_equal_publishable; ::tests::testkit_baseline_is_independent_of_the_workspace_version"
```
