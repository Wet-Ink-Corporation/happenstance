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
  satisfied: false
  evidence: ""
  mount_point: "experiments/polling-cost/Cargo.toml"
  verifying_test: "experiments/polling-cost/tests/one_cell.rs"
- id: AC-002
  criterion: |-
    GIVEN R1 opens the artefact looking for the cost ES-32 imposes, WHEN they read the first number, THEN it is **delivery amplification** — events the store yielded across all views ÷ events the projections applied — a dimensionless count, not a duration; AND it is reported over the declared grid (fan-out × log size × poll interval × selectivity arm, both the *overlapping* and *disjoint* arms present), with wall-clock timings recorded only as secondary figures carrying the machine that produced them.
  satisfied: false
  evidence: ""
  mount_point: "experiments/polling-cost/Cargo.toml"
  verifying_test: "experiments/polling-cost/tests/amplification.rs, experiments/polling-cost/tests/sweep_shape.rs"
- id: AC-003
  criterion: |-
    GIVEN R1 finds a figure they want to judge the relevance of, WHEN they look for what it was taken under, THEN every condition is *in the record itself* — fan-out, log size, poll interval, chunk, arm, repeat index, seed, projection store used, feature set, `rustc -vV`, `cargo -V`, git revision + dirty flag, build profile, OS, CPU model, logical CPUs, RAM, run instant — AND the fixed ones are tabulated in the README's §1 in the shape the sibling already uses, so the judgement can be made without opening a JSON file.
  satisfied: false
  evidence: ""
  mount_point: "experiments/polling-cost/Cargo.toml"
  verifying_test: "experiments/polling-cost/tests/record_conditions.rs"
- id: AC-004
  criterion: |-
    GIVEN R1 reads a staleness figure, WHEN they ask what it counts, THEN it is something that was *observed* — nanoseconds from an event's append instant to the instant an observing view's checkpoint reaches or passes it, and the count of events yielded to a view but not yet applied at a poll boundary — AND `head() - checkpoint` appears nowhere: not in the harness code, not in a README column, not in a derived field of a result file, because positions are an opaque ordering key that may have gaps.
  satisfied: false
  evidence: ""
  mount_point: "experiments/polling-cost/Cargo.toml"
  verifying_test: "experiments/polling-cost/tests/staleness.rs"
- id: AC-005
  criterion: |-
    GIVEN R2 wants to re-analyse the pass rather than re-read the prose, WHEN they open `results/`, THEN they find newline-delimited JSON conforming to a schema committed beside it, every record carrying `schema_version` and the seed that produced it — AND the projection store the run used is one of exactly two named options (HS-P0010's `MemoryProjectionStore` if it has landed, else a harness-local store under `experiments/polling-cost/src/` honouring `begin`/`commit` atomicity and never added under `crates/`), recorded as a field on every record and as a row in the README's conditions table with its dilution rationale stated.
  satisfied: false
  evidence: ""
  mount_point: "experiments/polling-cost/Cargo.toml"
  verifying_test: "experiments/polling-cost/tests/schema.rs"
- id: AC-006
  criterion: |-
    GIVEN R1 is deciding whether this number bears on ES-32's falsifier, WHEN they reach the README's own `## What this does not prove` section, THEN it names, in the falsifier's own words, the terms this measurement does not reach — "on a real deployment" (no durable adapter exists; one process, an in-process `Vec`, no network, no storage latency) and "their staleness budget" (none has been stated, so staleness is reported and not judged) — AND the word **floor** appears, AND the verdict section quotes the amplification figure and stops there, so a floor cannot be mistaken for a verdict.
  satisfied: false
  evidence: ""
  mount_point: "experiments/polling-cost/Cargo.toml"
  verifying_test: "experiments/polling-cost/tests/readme.rs"
- id: AC-007
  criterion: |-
    GIVEN a contributor who has never heard of this experiment runs the repository's gate, WHEN `cargo xtask ci` and `cargo xtask affected --base main` execute over a diff that touches only `experiments/polling-cost/**`, THEN nothing changes: no package is selected, no step is added, the root `Cargo.toml` `members` list and `[workspace.dependencies]` are untouched, no `crates/**` manifest gains a dependency, and the harness's own manifest opens with a bare `[workspace]` table carrying the comment that says why.
  satisfied: false
  evidence: ""
  mount_point: "experiments/polling-cost/Cargo.toml"
  verifying_test: "experiments/polling-cost/tests/gate_inertness.rs"
- id: AC-008
  criterion: |-
    GIVEN the harness produces a number a reader dislikes, WHEN the run completes, THEN nothing fails and nothing is recommended: the harness exits non-zero only when *it* failed to produce a number, never because a number was large; no threshold exists anywhere; the README recommends neither adding nor withholding a tail seam; ES-32 keeps its `[PROVISIONAL]` marker and `spec/SPECIFICATION.md` is not edited — AND if holding `run_projection` from outside the workspace proved awkward, that finding is written as a project AC-012 defect-log entry naming the item, not patched around locally.
  satisfied: false
  evidence: ""
  mount_point: "experiments/polling-cost/Cargo.toml"
  verifying_test: "experiments/polling-cost/tests/no_verdict.rs"
- id: AC-009
  criterion: |-
    GIVEN R1 opens this artefact cold, six months out, in a terminal and in a text editor, WHEN they read it top to bottom, THEN it is *composed*, not dumped: the README carries the sibling's section order with the headline amplification figure and the conditions table above any per-cell detail, timings never appearing before the ratio they qualify; tables overflow with `… and N more` rather than reflowing; AND the harness's own console output is plain — whole newline-terminated lines, no ANSI colour, no spinner, no percentage, no carriage-return rewrite, nothing past 80 columns, no assumption of a TTY, so the run is legible piped to a file.
  satisfied: false
  evidence: ""
  mount_point: "experiments/polling-cost/Cargo.toml"
  verifying_test: "experiments/polling-cost/tests/output_format.rs, experiments/polling-cost/tests/readme.rs"
- id: AC-010
  criterion: |-
    GIVEN R2 re-measures after the runner has changed, WHEN they run the harness again, THEN the committed pass they are comparing against is still there, byte-identical, and still cited by the README — the new pass is written under a new run tag beside it; AND re-running with a tag that already exists is refused with a message naming the existing pass rather than silently overwriting it, so a re-run is undone by deleting one directory and nothing is ever edited in place.
  satisfied: false
  evidence: ""
  mount_point: "experiments/polling-cost/Cargo.toml"
  verifying_test: "experiments/polling-cost/tests/rerun_is_additive.rs"
```
