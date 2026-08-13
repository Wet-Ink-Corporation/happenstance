---
item: HS-S0071
stage: implement
created: 2026-08-12
updated: 2026-08-12
---

# Acceptance ledger — PostgresProjectionStore against the frozen Batch

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Three notes the implementer needs before flipping anything:

1. **Nothing here is flippable until HS-P0010's artefacts are in the tree.** The frozen
   `ProjectionStore` port, the projection suite and its macro, and the capability-declension policy
   all land in `projection-store-freeze`. If any is absent, EC-008 applies: halt and report. A row
   satisfied against today's provisional `crates/happenstance-core/src/projection.rs` is satisfied
   against the wrong port.
2. **AC-002, AC-004, AC-005 and AC-009 are the four rows no conformance rule can satisfy on its
   own.** They are about *this adapter's construction* — the batch shape and the record it corrects,
   the `Clone` identity, the restart-survival of `Rebuilding`, and the wording of a declined
   capability — so each carries a story-local gated test in the same file, and AC-002 additionally
   needs the two corrected records cited by `file:line`.
3. **AC-010's evidence is a transcript and a workflow run, not a test id.** Cite the run and the
   Docker-stopped/network-unplugged transcript, including the deliberate negative exercise with the
   gate flag off shown failing.

```yaml
- id: AC-001
  criterion: "GIVEN an adapter author who has just written a `ProjectionStore` body and wants an executable answer to \"am I done\", WHEN they run the live-Postgres job's projection command, THEN `crates/happenstance-postgres/tests/postgres_projection_conformance.rs` invokes `projection_store_conformance!` over its **full expansion** — all seventeen rules present in the binary, each reporting pass, fail or a declared skip — and not one hand-picked subset."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/postgres_projection_conformance.rs"
  verifying_test: "the live-Postgres job's `cargo test -p happenstance-postgres --all-features -- --ignored --list` assertion in .github/workflows/ci.yml, modelled on xtask/src/proof.rs:199-217, plus the default gate's clippy step compiling the macro expansion"
- id: AC-002
  criterion: "GIVEN the recorded shape `type Batch<'a> = sqlx::Transaction<'static, Postgres>` was written by a `todo!()` body and the frozen `begin` is neither `async` nor fallible, WHEN the adapter author implements `begin`, THEN it returns an owned `Self::Batch` without a round trip, a runtime block or a lazily-deferred checkout, its module doc names the alternative that lost and why, and where the outcome contradicts `references/adapter-shapes.md:160-168` that record and `RUNBOOK.md`'s phase-10 session log are corrected in place with what actually happened."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/src/projection_store.rs (the impl and its module doc); references/adapter-shapes.md and RUNBOOK.md's phase-10 session log"
  verifying_test: "crates/happenstance-postgres/tests/postgres_projection_conformance.rs::begin_does_not_touch_the_server, plus the corrected records cited by file:line"
- id: AC-003
  criterion: "GIVEN the adapter author's whole reason to reach for a projection port is that a read model and its checkpoint never disagree, WHEN a `commit` succeeds against a live Postgres, THEN the read-model rows and the checkpoint are both visible **through a fresh handle**, and when it fails neither is — one transaction, one commit, the batch consumed either way."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/postgres_projection_conformance.rs"
  verifying_test: "crates/happenstance-postgres/tests/postgres_projection_conformance.rs::commit_is_atomic_with_the_read_model, ::commit_advances_the_checkpoint, ::failed_commit_leaves_both_unchanged (live job)"
- id: AC-004
  criterion: "GIVEN a pooled adapter hands out several cloneable handles onto one backing store and a batch is an owned value a caller can carry anywhere, WHEN a batch begun on one store instance is passed to `commit`, `reset` or `rollback` on a different instance, THEN it is refused as `CommitError::ForeignBatch` / `ResetError::ForeignBatch` with both stores unchanged — and whether a **clone** counts as the same instance is a decided, documented answer rather than an accident of `#[derive(Clone)]`."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/postgres_projection_conformance.rs"
  verifying_test: "crates/happenstance-postgres/tests/postgres_projection_conformance.rs::commit_rejects_a_foreign_batch (live job) and ::a_clone_shares_its_parents_batch_identity (story-local, gated)"
- id: AC-005
  criterion: "GIVEN the reader who most needs to know a projection is mid-rebuild is the one who arrives after the rebuilding process died, WHEN `checkpoint` is called from a **new process** against the same database, THEN it still answers `Rebuilding { through }` for a projection last committed under `Authority::Rebuilding`, `NeverRun` for an id with no row at all, and `Live { through }` otherwise — the discriminator being a stored column, never a flag on the handle, and absence of a row never a sentinel position."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/postgres_projection_conformance.rs"
  verifying_test: "crates/happenstance-postgres/tests/postgres_projection_conformance.rs::rebuilding_is_distinguishable_from_live, ::fresh_projection_has_no_checkpoint, ::reset_is_not_commit_at_first (live job) and ::rebuilding_survives_a_new_handle (story-local, gated)"
- id: AC-006
  criterion: "GIVEN an adapter author rebuilding one projection must not damage another, WHEN `reset` runs for one `ProjectionId`, THEN that projection's caller-supplied deletes and its checkpoint row disappear together in one unit and every other projection's checkpoint is untouched — and whether this store ever answers `ResetError::Refused` is stated with its reason where a caller reads it rather than defaulted to \"never\"."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/postgres_projection_conformance.rs"
  verifying_test: "crates/happenstance-postgres/tests/postgres_projection_conformance.rs::reset_clears_rows_and_checkpoint_together, ::reset_is_scoped_to_one_projection, ::refused_reset_changes_nothing (live job), plus the refusal-policy prose on the impl cited by file:line"
- id: AC-007
  criterion: "GIVEN a projection that consumed a thousand events and wrote nothing must still be able to advance, and a checkpoint that goes backwards silently is how a read model loses events forever, WHEN `commit` is handed a position the batch never wrote, THEN it is accepted; and WHEN it is handed a position at or below the stored one, THEN it is refused as `CommitError::CheckpointRegression { current, attempted }` carrying the value read **inside the same transaction**."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/postgres_projection_conformance.rs"
  verifying_test: "crates/happenstance-postgres/tests/postgres_projection_conformance.rs::commit_accepts_a_position_the_batch_did_not_write and ::commit_rejects_a_regressing_position (live job)"
- id: AC-008
  criterion: "GIVEN the suite cannot look at a read model whose shape it does not know, and an adapter author should pay for that with one feature flag rather than a new edge in their dependency graph, WHEN they enable `conformance` on `happenstance-postgres`, THEN `ProjectionProbe` is implemented for `PostgresProjectionStore` forwarding to `happenstance-core/conformance`, `READS_THROUGH_BATCH` states a **fact** about the chosen batch shape (a `true` that answers from the committed table is a defect, a `false` is honest and reported), and the feature composes in every direction the powerset takes."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/Cargo.toml [features]; crates/happenstance-postgres/tests/postgres_projection_conformance.rs"
  verifying_test: "crates/happenstance-postgres/tests/postgres_projection_conformance.rs::batch_reads_reflect_pending_writes and ::rebuild_is_chunk_size_invariant (live job), plus `cargo hack --feature-powerset check -p happenstance-postgres` in the default gate"
- id: AC-009
  criterion: "GIVEN \"a rule absent from the binary is indistinguishable in CI output from a rule that passed\", WHEN this adapter cannot honour something the suite asks for, THEN the run prints ``SKIP {rule}: fixture declines `{capability}` — {reason}`` in the fixture's own Postgres-specific words under `--show-output`, using a capability name that already exists upstream — no invented constant, no `#[cfg]`-ed-away rule, no empty reason."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/postgres_projection_conformance.rs (the fixture's Capability constants)"
  verifying_test: "crates/happenstance-postgres/tests/postgres_projection_conformance.rs::declined_capabilities_carry_a_postgres_specific_reason — assertions on RuleOutcome values against crates/happenstance-testkit/src/contract.rs:495-537 — plus the live job's --show-output log"
- id: AC-010
  criterion: "GIVEN every other contributor to this initiative runs the gate on a laptop, WHEN they run `cargo xtask ci --fast` with the Docker daemon stopped and the network unplugged, THEN it is green with the new target **compiled, linted and reported as ignored** rather than cfg'd out of existence — while in CI the projection command runs inside the **existing** live-Postgres job (no third job) and that job cannot be green having executed zero tests."
  satisfied: false
  evidence: ""
  mount_point: ".github/workflows/ci.yml (the existing live-Postgres job, extended); xtask/src/main.rs REQUIRED steps via `cargo xtask ci --fast`"
  verifying_test: "`cargo xtask ci --fast` transcript with the Docker daemon stopped and networking disabled showing the target as `ignored` not `0 tests`; the job's nonzero executed-test-count assertion modelled on xtask/src/proof.rs:199-231; and a deliberate negative exercise with the gate flag off shown failing — all cited in implementation-report.md"
```
