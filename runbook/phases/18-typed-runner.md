# Phase 18 — The typed runner leaves its gate

**Goal.** `happenstance`'s projection runner is a stable, published API: no
`unstable-projection` feature between an application author and it.

**Why here.** The port under it has been `[FROZEN]` since `0.3.0` (ADR-0063), and
`happenstance-core`'s feature of the same name gates nothing. What still holds the
runner back is one layer up, and it is three things: `Projection::apply` is
synchronous (`crates/happenstance/src/runner.rs:95-99`), so a projection can push into
a buffered batch and cannot issue a statement into a live one; the runner halts on
the first failure and has no failure-policy seam, so PS-27's *skip and record*
has a count of zero; and there is no fan-out runner, so PS-30 binds nothing.
Phase 17 decides `apply`; this phase builds it.

**Decisions it settles.** None new — it implements phase 17's `apply` record and
ADR-0070's `Chunk`. Discharges PS-18, PS-27 and PS-30 from `[DEFERRED]`, or
renews them with the disposition phase 16 gave.

**Work**

- [ ] `Projection::apply` in the shape phase 17 decided, against both batch shapes
      the port was frozen on: a buffered batch, and `LivePostgresProjectionStore`'s
      live `sqlx` transaction.
- [ ] A failure-policy seam through which *skip and record* can be written
      atomically with the checkpoint (PS-27). The Kestrel Motor shred case in
      `references/scenarios/README.md` §4 is the workload that needs it.
- [ ] A fan-out runner holding N views over one log (PS-30, E2E-32), measured with
      `experiments/polling-cost` — a benchmark, not a conformance rule (CF-34).
- [ ] A refusable reset implemented by at least one adapter (PS-18,
      `refused_reset_changes_nothing`).
- [ ] **ADR-0070's `Chunk`.** `run_projection`'s `chunk: NonZeroUsize`
      (`crates/happenstance/src/runner.rs:519`) becomes a named `Chunk`:
      `#[non_exhaustive]`, private representation, `Chunk::of(NonZeroUsize)` its
      first constructor — the shape `Retry` already has
      (`crates/happenstance/src/command.rs:43-62`). The fan-out runner above takes
      the same type. No `Default` and no observation seam: both are additive
      later, and ADR-0070 says what reopens each. Whether the parameter is
      `Chunk` or `impl Into<Chunk>` is this phase's compile to settle, and the
      session log records which. If `Chunk` cannot serve both runners without a
      second type, say so before the gate lifts — that is ADR-0070's falsifier.
- [ ] **The convergence declaration SY-20 needs.** The way a projection declares
      itself convergent, in the shape phase 17's `apply` record gives it, with
      SY-22's placement for it. Phase 13's
      `convergent_projection_is_interleaving_independent` consumes it, and phase
      13 runs after this phase for that reason — it is in 13's dependency row.
- [ ] **The clauses phase 16 gave this phase to freeze.** Each `freeze-by-18` row
      in [the 1.0 dispositions](../ledgers.md), and what freezes it:
      - **PS-16** — a typed-runner rebuild through `reset` against a multi-table
        or graph read model, with the `RESET_REFUSAL` clause composed in.
      - **PS-18** — the refusable reset above, a CF-39-shaped clause, and a
        `NoopProtectFixture` mutant. If this phase cannot deliver it, PS-18 is
        renewed past 1.0 as additive, by a record rather than by default.
      - **PS-25** — the remedy phase 17 chose (the derived id, or the digest in
        the checkpoint record), built before the gate lifts.
      - **PS-27** — the failure-policy seam above and `skip_and_record_is_atomic`,
        with a mutant.
      - **PS-30** — the fan-out runner above and `panicking_apply_rolls_back`,
        with a mutant. **If this phase does not build fan-out, PS-30 becomes
        outside 1.0's surface**: a conditional MUST on a runner 1.0 does not ship.
        ADR-0066 states that fallback, so taking it is a session-log line and a
        ledger edit, not a new decision.
      - **PS-38** — settled with PS-23 against the fan-out runner, carrying the
        no-lagging-replica obligation phase 17 documented.
      - **SY-21** — `Projection::apply` built in the shape phase 17's record gave
        it, frozen here on that signature — a convergent projection is handed an
        `EventId` and has no parameter through which a `SequencePosition` could
        reach it — and a typed-layer test that holds it. Phase 13 runs after this
        phase, and its sync rule exercises the same surface through replication.
- [ ] `unstable-projection` removed from `happenstance`, and every example that
      enabled it compiled without it. Removing a Cargo feature is itself a break
      (`references/adr/0063-the-projection-port-is-frozen.md:48-56`), and this
      phase runs after `0.4.0`'s window has closed, so the removal ships in the
      next breaking release — `1.0.0`, if nothing comes between — or the name
      stays declared and empty, as `happenstance-core`'s did until phase 17. The
      session log says which. If the gate cannot lift at all, ADR-0066 declares
      the runner exempt from semver at 1.0.

**Proof artefact.** `examples/rebuilding-read-models` compiled and run with no
`unstable-projection` anywhere in its dependency graph, and PS-18, PS-27 and
PS-30 each with a rule and a mutant that fails it.

**Exit criteria**

- [ ] No `unstable-projection` feature in `happenstance`.
- [ ] PS-18, PS-27 and PS-30 are out of `[DEFERRED]`, or renewed per phase 16.
- [ ] A projection can write into a live batch, demonstrated against a real
      database rather than a buffer.
- [ ] `run_projection` takes ADR-0070's `Chunk`, and so does the fan-out runner.
- [ ] Every `freeze-by-18` clause in [`ledgers.md`](../ledgers.md)'s 1.0
      dispositions — PS-16, PS-18, PS-25, PS-27, PS-30, PS-38 and SY-21 — is
      `[FROZEN]`, or re-dispositioned by a record that says why. PS-30's
      outside-1.0 fallback needs only the session-log line and the ledger edit,
      since ADR-0066 already records it.
- [ ] The specification is reconciled against this phase's changes (session
      protocol step 6), and `cargo xtask spec-trace` passes.

**Estimate.** 5–8 days.

**Session log**
