---
item: "HS-S0119"
stage: implement
created: "2026-08-13"
updated: "2026-08-13"
---

# Acceptance ledger — The projection runner resumed across the hole

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes specific to this story, both from `spec.md`:

- **AC-001's satisfying evidence has two legal shapes.** If HS-P0011's runner and HS-P0010's
  `MemoryProjectionStore` are present, the evidence is the harness binding them. If either is absent,
  the story **halts** (EC-001 / EC-002) and AC-001's evidence is the blocker note in this folder plus
  an empty `crates/**` diff — in which case AC-002 – AC-008 are not reachable and the story does not
  advance on a halt; the halt is reported to the orchestrating command, not flipped through.
- **A negative finding is a satisfied AC-008.** The expected outcome is that this reader cannot be
  made loud without a port primitive it does not have; AC-008 is satisfied by that finding being
  recorded with the surface named, not by any change being made.

```yaml
- id: AC-001
  criterion: "GIVEN the application author's dashboard is driven by the projection runner `happenstance` actually ships — HS-P0011's, per ADR-0007 — WHEN this story runs its observation, THEN the harness drives that runner and HS-P0010's `MemoryProjectionStore` and constructs neither itself; and WHEN either is absent from the tree, THEN the story halts, reports the blocker by story id (HS-P0011 `projection-trait-and-runner`; HS-P0010 `projection-store-freeze` AC-012) as a note in this story's own folder, and writes no bespoke runner, no throwaway projection store and no code at all — because a hazard demonstrated against a reader nobody ships tells the application author nothing about the runner they will actually deploy"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/readers_against_the_hole.rs"
  verifying_test: "crates/happenstance-testkit/tests/readers_against_the_hole/projection_runner.rs::dependency_probe_binds_the_shipped_runner"

- id: AC-002
  criterion: "GIVEN the application author ran a projection, then pruned the store, then restarted the projection, WHEN the runner resumes, THEN it resumes from a checkpoint derived from the positions `append` actually returned and from the retained predicate the instrument was built with — never a literal, never the instrument's first retained position — and at least one forgotten position lies strictly between that checkpoint and the head, so the resume genuinely crosses the hole rather than starting above it and reporting a complete-looking model"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/readers_against_the_hole.rs"
  verifying_test: "crates/happenstance-testkit/tests/readers_against_the_hole/projection_runner.rs::resume_checkpoint_is_derived_and_below_the_hole"

- id: AC-003
  criterion: "GIVEN the real-world order is *projection runs, prune runs, projection restarts*, WHEN the harness stages the hole, THEN it narrows the instrument's retained view between the first run and the resume — a second handle over the same inner `MemoryEventStore` with a narrower retained set, or a target-local narrowing seam — and never rebuilds through `MemoryEventStore::restore`, so the positions the runner already checkpointed still mean the same thing after the prune; the seam adds nothing to the `Fixture` trait and the existing conformance mounts stay green and unchanged"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/readers_against_the_hole.rs"
  verifying_test: "crates/happenstance-testkit/tests/readers_against_the_hole/projection_runner.rs::hole_opens_after_the_checkpoint_without_renumbering"

- id: AC-004
  criterion: "GIVEN two journeys the application author actually lives — the device slice that keeps only a suffix (E2E-46) and the regulated purge that leaves survivors below the hole (E2E-47) — WHEN the runner is resumed under each, THEN five values are captured and asserted per configuration: the checkpoint resumed from, the positions of the events the runner actually received, the checkpoint held afterwards, whether it advanced, and whether anything anywhere produced an error or a log line — recorded as values the evaluator can read, never as \"the reader failed\""
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/readers_against_the_hole.rs"
  verifying_test: "crates/happenstance-testkit/tests/readers_against_the_hole/projection_runner.rs::observation_captures_five_values_under_both_configurations"

- id: AC-005
  criterion: "GIVEN the suite already predicts that a projection resuming across a gap \"stalls forever with no error anywhere\" (`crates/happenstance-testkit/src/suite.rs:1523-1531`), WHEN the runner meets that shape, THEN the stall is evidenced deterministically — the runner driven a bounded number of iterations, the read across the gap captured as its actual empty or short result, the checkpoint asserted unchanged, and the absence of any error or log line asserted as an absence — with no `sleep`, no wall-clock timeout and no \"it didn't finish in time\", because a stall proved by a clock is a flake the application author will later be told to ignore"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/readers_against_the_hole.rs"
  verifying_test: "crates/happenstance-testkit/tests/readers_against_the_hole/projection_runner.rs::stall_is_evidenced_by_unchanged_checkpoint_and_no_error"

- id: AC-006
  criterion: "GIVEN the application author whose dashboard looks perfectly healthy after a prune, WHEN the runner does advance across the hole instead of stalling, THEN the recorded artefact is the materialised read-model state itself, placed beside the state the same projection produces over the unforgotten inner log, so the two values exhibit the divergence — internally consistent, monotonic checkpoint, intact transaction discipline, and wrong in exactly E2E-47's way — rather than a sentence claiming the model was wrong"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/readers_against_the_hole.rs"
  verifying_test: "crates/happenstance-testkit/tests/readers_against_the_hole/projection_runner.rs::read_model_across_the_hole_differs_from_the_unforgotten_model"

- id: AC-007
  criterion: "GIVEN the repository owner must resolve DT-7 — one undifferentiated incompleteness signal, or the transient / benign-permanent / meaningful-permanent distinction — WHEN they read this story's record, THEN they find whether the runner's observable state differs at all between the suffix prune and the scattered purge, recorded either way; identical is a result, not a null result, and is handed to `dt-7-signal-shape-and-the-redaction-answer` as the measurement that a three-way distinction cannot be delivered to this reader without a new primitive"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/readers_against_the_hole.rs"
  verifying_test: "crates/happenstance-testkit/tests/readers_against_the_hole/projection_runner.rs::suffix_and_scattered_observations_are_compared"

- id: AC-008
  criterion: "GIVEN the evaluator deciding in one sitting, WHEN they read this story's finding, THEN they learn that this reader cannot be told loudly without a port primitive it does not have — a checkpoint is a bare `SequencePosition` and `EventStore::read` has no channel to say history below it was destroyed — with the primitive named as ES-39's and the option set as DA-7's four rows (a floor, retained ranges, a third condition outcome, a tri-state `contains_event_id`) and its version consequence (a `0.3.0` this initiative's exit criteria do not contemplate); AND no such change is made here — no new error variant, no panic added to make the harness look loud, no `warn!(\"gap detected\")`, no port-surface edit — the finding being recorded against project AC-011 and handed to `surface-diff-and-the-ac-012-escalation` (project AC-012)"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/readers_against_the_hole.rs"
  verifying_test: "crates/happenstance-testkit/tests/readers_against_the_hole/projection_runner.rs::finding_names_the_missing_surface_and_changes_nothing"
```
