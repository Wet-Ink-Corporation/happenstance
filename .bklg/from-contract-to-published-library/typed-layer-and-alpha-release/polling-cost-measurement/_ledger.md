---
item: HS-S0028
stage: implement
created: 2026-08-12T13:46:25.210Z
updated: 2026-08-12T13:46:25.210Z
---

# Acceptance ledger — The polling cost ES-32 imposes, as a number

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Every `verifying_test` below lives inside the experiment crate and is run by `cargo test
--manifest-path experiments/polling-cost/Cargo.toml`. That command is deliberately **not** a step in
`cargo xtask ci` — the crate is not a workspace member (CF-34; `CLAUDE.md`, repository map) — so the
evidence for a flipped row is the recorded output of that command plus the cited `file:line`, not a
gate run.

```yaml
- id: AC-001
  criterion: |-
    GIVEN R2 has a clean checkout and a Rust toolchain — no Docker, no database, no network — WHEN they run `bash experiments/polling-cost/run.sh`, THEN the harness builds `--release`, captures the environment, drives the **real** `happenstance::run_projection` behind `unstable-projection` across the declared sweep, and writes a complete pass into `results/`; AND it re-implements no poll loop of its own, and every generic function in it binds `EventStore`, never `SendEventStore`.
  satisfied: true
  evidence: >-
    `bash experiments/polling-cost/run.sh` runs from a clean checkout with a Rust toolchain and
    nothing else: it prints `rustc -vV`, `cargo -V`, the measured revision, `git status --porcelain
    -- crates Cargo.toml Cargo.lock` and `uname -a`, builds `--release`, runs all 144 cells and
    writes `results/<tag>/records.ndjson`. Transcript in the implementation report. The real runner
    is called at experiments/polling-cost/src/lib.rs (`happenstance::run_projection`), reached
    through the path dependency in experiments/polling-cost/Cargo.toml with `features =
    ["unstable-projection", "memory", "json"]`. Tests:
    experiments/polling-cost/tests/one_cell.rs::every_seeded_event_reaches_every_view_through_the_runner
    (4 views x 256 events overlapping: 1024 delivered, 1024 applied, 256 distinct; disjoint: 256
    delivered) and ::the_harness_calls_the_published_runner_and_binds_the_bare_flavour
    (comment-stripped source: `happenstance::run_projection(` present,
    `SendEventStore`/`SendProjectionStore` absent, no second poll loop outside the counting
    instrument). Both PASS under `cargo test --manifest-path experiments/polling-cost/Cargo.toml`.
  mount_point: "experiments/polling-cost/Cargo.toml"
  verifying_test: "experiments/polling-cost/tests/one_cell.rs"
- id: AC-002
  criterion: |-
    GIVEN R1 opens the artefact looking for the cost ES-32 imposes, WHEN they read the first number, THEN it is **delivery amplification** — events the store yielded across all views ÷ events the projections applied — a dimensionless count, not a duration; AND it is reported over the declared grid (fan-out × log size × poll interval × selectivity arm, both the *overlapping* and *disjoint* arms present), with wall-clock timings recorded only as secondary figures carrying the machine that produced them.
  satisfied: true
  evidence: >-
    The headline field is `delivery_amplification`, a ratio of two observed counts and not a
    duration: **32.00** at 32 overlapping views, **1.00** disjoint, recorded on every one of the 144
    rows in experiments/polling-cost/results/pass-001/records.ndjson. Timings are the separate
    `elapsed_ns` field and the README names them secondary, below the ratio. Tests:
    tests/amplification.rs::the_headline_is_delivery_amplification_and_not_a_duration (hand-built
    8x128 cell with known counts: 1024 delivered, 128 distinct, headline 8.00; asserts no field name
    claims the headline is a duration),
    ::the_two_arms_differ_and_that_is_what_makes_the_ratio_mean_anything,
    ::amplification_grows_with_fan_out_in_the_overlapping_arm; and
    tests/sweep_shape.rs::every_declared_axis_value_appears +
    ::both_selectivity_arms_are_present_in_both_phases +
    ::every_cell_carries_its_seed_and_its_repeat. All 6 PASS. The declared grid is trimmed into two
    phases and the trim is recorded with its reason in the README's §1 — no axis value and neither
    arm is dropped.
  mount_point: "experiments/polling-cost/Cargo.toml"
  verifying_test: "experiments/polling-cost/tests/amplification.rs, experiments/polling-cost/tests/sweep_shape.rs"
- id: AC-003
  criterion: |-
    GIVEN R1 finds a figure they want to judge the relevance of, WHEN they look for what it was taken under, THEN every condition is *in the record itself* — fan-out, log size, poll interval, chunk, arm, repeat index, seed, projection store used, feature set, `rustc -vV`, `cargo -V`, git revision + dirty flag, build profile, OS, CPU model, logical CPUs, RAM, run instant — AND the fixed ones are tabulated in the README's §1 in the shape the sibling already uses, so the judgement can be made without opening a JSON file.
  satisfied: true
  evidence: >-
    Every condition is a field on the record, not prose about it: `fan_out`, `log_size`,
    `poll_interval_ms`, `chunk`, `arm`, `repeat`, `seed`, `projection_store`, plus the once-per-pass
    manifest carrying `rustc`, `cargo`, `git_rev`, `git_dirty`, `profile`, `os`, `cpu`,
    `logical_cpus`, `ram_bytes`, `features` and `started_at`. The fixed ones are tabulated in
    experiments/polling-cost/README.md §1 in the sibling's shape. Tests:
    tests/record_conditions.rs::every_record_carries_every_condition (17 fields on every record, 15
    on the manifest, none null and no empty string), ::the_manifest_appears_exactly_once_per_pass
    (and is the first line), ::a_committed_pass_measured_a_clean_tree (`git_dirty == false`,
    `profile == release`), ::started_at_is_the_run_instant_and_not_the_commit_instant (a `Z`-shaped
    instant, parsed and asserted strictly after the instant `git_rev` was committed),
    ::a_committed_pass_records_the_ram_it_ran_on (non-zero on a recorded pass) and
    ::a_pass_is_committed_at_all. All 6 PASS. The committed pass names revision `3999b5a` with
    `git_dirty: false`. Two conditions were re-taken after review: `started_at` read
    `git log -1 --format=%cI`, the *commit* instant under a field documented as the run's, which
    made two passes at one revision indistinguishable in the only field that separates them; and
    `ram_bytes` was hardcoded to 0 under a field documented as "or 0 where the OS did not say",
    implying a query nothing made. Both are captured now — the system clock, and `/proc/meminfo` /
    `sysctl hw.memsize` / `Get-CimInstance Win32_ComputerSystem` — and the last two tests above were
    red against the earlier pass before it was re-taken.
  mount_point: "experiments/polling-cost/Cargo.toml"
  verifying_test: "experiments/polling-cost/tests/record_conditions.rs"
- id: AC-004
  criterion: |-
    GIVEN R1 reads a staleness figure, WHEN they ask what it counts, THEN it is something that was *observed* — nanoseconds from an event's append instant to the instant an observing view's checkpoint reaches or passes it, and the count of events yielded to a view but not yet applied at a poll boundary — AND `head() - checkpoint` appears nowhere: not in the harness code, not in a README column, not in a derived field of a result file, because positions are an opaque ordering key that may have gaps.
  satisfied: true
  evidence: >-
    Staleness is two observed quantities: nanoseconds from an event's append instant to the instant
    an observing view's checkpoint reached or passed it (`staleness_ns_p50`/`p95`), and the backlog
    at a poll boundary counted by **comparing** positions (`pending_at_poll_max`).
    experiments/polling-cost/src/observer.rs holds both, and `pending` filters on `entry.position >
    through` rather than subtracting. Tests:
    tests/staleness.rs::the_observer_reports_the_same_backlog_on_a_gapped_store (the same
    three-appended/one-observed pattern through `MemoryEventStore` and through
    `happenstance_testkit::GappyMemoryStore` at stride 7 — the observer answers 2 for both),
    ::the_subtracting_mutant_fails_the_same_comparison (the mutant is written into the test file and
    answers 2 and **14**, so the discriminator is demonstrated rather than assumed),
    ::staleness_is_a_duration_between_two_observed_instants, and
    ::no_head_value_participates_in_arithmetic_anywhere_in_the_harness (comment-stripped source
    assertion: no `head()` anywhere in the harness, and no position in a `*_sub`). All 4 PASS.
  mount_point: "experiments/polling-cost/Cargo.toml"
  verifying_test: "experiments/polling-cost/tests/staleness.rs"
- id: AC-005
  criterion: |-
    GIVEN R2 wants to re-analyse the pass rather than re-read the prose, WHEN they open `results/`, THEN they find newline-delimited JSON conforming to a schema committed beside it, every record carrying `schema_version` and the seed that produced it — AND the projection store the run used is one of exactly two named options (HS-P0010's `MemoryProjectionStore` if it has landed, else a harness-local store under `experiments/polling-cost/src/` honouring `begin`/`commit` atomicity and never added under `crates/`), recorded as a field on every record and as a row in the README's conditions table with its dilution rationale stated.
  satisfied: true
  evidence: >-
    experiments/polling-cost/results/pass-001/records.ndjson is newline-delimited JSON — one
    manifest line then 144 records — conforming to
    experiments/polling-cost/schema/result-record.schema.json and schema/manifest.schema.json,
    committed beside it. Every record carries `schema_version` and `seed`. The projection store used
    is HS-P0010's `MemoryProjectionStore` (it had landed), recorded as `projection_store` on every
    line and as a row in README §1 with its dilution rationale stated in §4. Tests:
    tests/schema.rs::every_committed_line_validates_against_the_committed_schema (through the
    hand-written checker at src/validate.rs, which enforces `required`, `properties[].type` and
    `enum`), ::the_schema_version_in_the_corpus_matches_the_harness,
    ::the_projection_store_is_one_of_exactly_two_named_values (never absent, never free text),
    ::the_seed_is_in_the_file_rather_than_in_prose. All 4 PASS.
  mount_point: "experiments/polling-cost/Cargo.toml"
  verifying_test: "experiments/polling-cost/tests/schema.rs"
- id: AC-006
  criterion: |-
    GIVEN R1 is deciding whether this number bears on ES-32's falsifier, WHEN they reach the README's own `## What this does not prove` section, THEN it names, in the falsifier's own words, the terms this measurement does not reach — "on a real deployment" (no durable adapter exists; one process, an in-process `Vec`, no network, no storage latency) and "their staleness budget" (none has been stated, so staleness is reported and not judged) — AND the word **floor** appears, AND the verdict section quotes the amplification figure and stops there, so a floor cannot be mistaken for a verdict.
  satisfied: true
  evidence: >-
    experiments/polling-cost/README.md `## 4. What this does not prove` is its own section and
    carries both of ES-32's falsifier terms verbatim — *"on a real deployment"* (no durable adapter
    exists; one process, an in-process `Vec` behind a `RwLock`, no network, no storage latency) and
    *"their staleness budget"* (none has been stated, so staleness is reported and not judged) —
    plus the word **floor** in its opening sentence. `## 5. The verdict` quotes the figures and
    stops. Tests: tests/readme.rs::the_gap_is_named_in_the_falsifiers_own_words,
    ::the_verdict_quotes_the_figure_and_stops_there (no recommendation for or against a tail seam),
    and the strong one, ::the_headline_figure_matches_the_committed_corpus — the peak overlapping
    amplification is recomputed from results/ and the formatted figure `32.00` must appear in §1, so
    prose and corpus cannot drift apart. All PASS.
  mount_point: "experiments/polling-cost/Cargo.toml"
  verifying_test: "experiments/polling-cost/tests/readme.rs"
- id: AC-007
  criterion: |-
    GIVEN a contributor who has never heard of this experiment runs the repository's gate, WHEN `cargo xtask ci` and `cargo xtask affected --base main` execute over a diff that touches only `experiments/polling-cost/**`, THEN nothing changes: no package is selected, no step is added, the root `Cargo.toml` `members` list and `[workspace.dependencies]` are untouched, no `crates/**` manifest gains a dependency, and the harness's own manifest opens with a bare `[workspace]` table carrying the comment that says why.
  satisfied: true
  evidence: >-
    Nothing in the workspace changed. experiments/polling-cost/Cargo.toml opens with a bare
    `[workspace]` table under a comment saying why; the root `Cargo.toml` `members` list,
    `[workspace.dependencies]` and every `crates/**` manifest are untouched (`git status --porcelain
    -- crates Cargo.toml Cargo.lock xtask` empty). `cargo xtask affected --base main` over this diff
    reports **affected gate passed** and selects no package, and `cargo xtask ci --fast` reports
    **all required checks passed** with an unchanged step list. Tests:
    tests/gate_inertness.rs::the_harness_manifest_opens_with_a_bare_workspace_table (and that the
    note explaining it is there), ::the_root_members_list_names_no_experiment,
    ::no_gate_step_names_this_harness, ::the_affected_gate_still_treats_experiments_as_inert
    (`is_inert`'s `experiments/` prefix still in xtask/src/affected.rs),
    ::no_crate_manifest_depends_on_this_harness. All 5 PASS.
  mount_point: "experiments/polling-cost/Cargo.toml"
  verifying_test: "experiments/polling-cost/tests/gate_inertness.rs"
- id: AC-008
  criterion: |-
    GIVEN the harness produces a number a reader dislikes, WHEN the run completes, THEN nothing fails and nothing is recommended: the harness exits non-zero only when *it* failed to produce a number, never because a number was large; no threshold exists anywhere; the README recommends neither adding nor withholding a tail seam; ES-32 keeps its `[PROVISIONAL]` marker and `spec/SPECIFICATION.md` is not edited — AND if holding `run_projection` from outside the workspace proved awkward, that finding is written as a project AC-012 defect-log entry naming the item, not patched around locally.
  satisfied: true
  evidence: >-
    The harness has no pass/fail path and no threshold. Tests:
    tests/no_verdict.rs::an_absurd_amplification_still_returns_ok_and_writes_a_record (32 views over
    one log, amplification > 30, `Ok` and a serialised record), ::the_harness_holds_no_threshold
    (comment-stripped sweep of every measured source for `THRESHOLD`, `BUDGET_`,
    `MAX_AMPLIFICATION`, `assert!`, `assert_eq!`, `panic!` — none),
    ::the_error_type_has_no_arm_for_a_number_being_large (`HarnessError` has `Seed`, `Run`, `Write`
    and no `TooSlow`/`Exceeded`/`OverBudget`/`Regression`). All 3 PASS. ES-32 keeps its
    `[PROVISIONAL]` marker and `git status --porcelain -- spec` is empty for this story. The one
    signature-friction finding — that `&mut P` forces one `run_projection` call per view per poll,
    which is what makes the read count 2N — is written into the implementation report as a project
    AC-012 defect-log entry rather than patched around.
  mount_point: "experiments/polling-cost/Cargo.toml"
  verifying_test: "experiments/polling-cost/tests/no_verdict.rs"
- id: AC-009
  criterion: |-
    GIVEN R1 opens this artefact cold, six months out, in a terminal and in a text editor, WHEN they read it top to bottom, THEN it is *composed*, not dumped: the README carries the sibling's section order with the headline amplification figure and the conditions table above any per-cell detail, timings never appearing before the ratio they qualify; tables overflow with `… and N more` rather than reflowing; AND the harness's own console output is plain — whole newline-terminated lines, no ANSI colour, no spinner, no percentage, no carriage-return rewrite, nothing past 80 columns, no assumption of a TTY, so the run is legible piped to a file.
  satisfied: true
  evidence: >-
    README section order is fixed and asserted; the headline amplification and the conditions table
    are in §1, above any per-cell detail, and the first timing figure appears after the ratio it
    qualifies. Both long tables carry an explicit `… and 30 more` overflow marker rather than
    reflowing, and §1's conditions table is split into *the measurement* and *the machine* so
    neither region exceeds the eight-row budget. Console output is plain:
    experiments/polling-cost/src/progress.rs emits whole
    `\n`-terminated lines, flattens embedded control characters, and **truncates** at 80 columns
    with an ellipsis rather than wrapping. Tests:
    tests/readme.rs::the_sections_are_present_and_in_the_declared_order,
    ::the_ratio_precedes_the_first_timing_figure,
    ::no_table_runs_past_eight_rows_without_saying_it_was_trimmed — scoped after review to the prose
    following the table that overflowed, because asking the whole document meant one marker
    satisfied every future overflow — and
    ::a_marker_under_a_different_table_does_not_excuse_an_overflow, the document that check used to
    accept, written down so the rule is not decorative;
    tests/output_format.rs::a_progress_line_carries_no_control_bytes (no ANSI, no carriage return,
    no newline, no `%`), ::a_long_line_is_truncated_rather_than_wrapped,
    ::every_line_the_sweep_would_emit_fits_the_budget (all 144 real sweep lines),
    ::embedded_control_characters_are_flattened_rather_than_forwarded. All 8 PASS.
  mount_point: "experiments/polling-cost/Cargo.toml"
  verifying_test: "experiments/polling-cost/tests/output_format.rs, experiments/polling-cost/tests/readme.rs"
- id: AC-010
  criterion: |-
    GIVEN R2 re-measures after the runner has changed, WHEN they run the harness again, THEN the committed pass they are comparing against is still there, byte-identical, and still cited by the README — the new pass is written under a new run tag beside it; AND re-running with a tag that already exists is refused with a message naming the existing pass rather than silently overwriting it, so a re-run is undone by deleting one directory and nothing is ever edited in place.
  satisfied: true
  evidence: >-
    A pass is one directory under `results/<tag>/` and `write_pass` refuses an existing tag, naming
    it. Demonstrated for real as well as in a test: `HS_TAG=pass-001 bash
    experiments/polling-cost/run.sh` against the committed pass exits non-zero with *the pass
    `pass-001` already exists at …*, leaving the committed bytes untouched. Tests:
    tests/rerun_is_additive.rs::a_second_run_under_the_same_tag_is_refused_and_changes_nothing
    (writes a pass, re-runs the same tag, asserts the refusal names it, asserts the bytes are
    byte-identical before and after, then writes a second tag and asserts both survive
    independently) and ::a_pass_is_one_directory_so_undoing_a_rerun_is_one_delete. Both PASS. The
    README's §6 documents `HS_TAG` and cites `results/pass-001/records.ndjson` by path.
  mount_point: "experiments/polling-cost/Cargo.toml"
  verifying_test: "experiments/polling-cost/tests/rerun_is_additive.rs"
```
