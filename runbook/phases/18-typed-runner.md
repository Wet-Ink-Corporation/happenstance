# Phase 18 — The typed runner leaves its gate

**Goal.** `happenstance`'s projection runner is a stable, published API: no
`unstable-projection` feature between an application author and it.

**Why here.** The port under it has been `[FROZEN]` since `0.3.0` (ADR-0063), and
`happenstance-core`'s feature of the same name gates nothing. What still holds the
runner back is one layer up, and it is three things: `Projection::apply` is
synchronous (`crates/happenstance/src/domain.rs:249`), so a projection can push into
a buffered batch and cannot issue a statement into a live one; the runner halts on
the first failure and has no failure-policy seam, so PS-27's *skip and record*
has a count of zero; and there is no fan-out runner, so PS-30 binds nothing.
Phase 17 decides `apply`; this phase builds it.

**Decisions it settles.** None new — it implements phase 17's `apply` record.
Discharges PS-18, PS-27 and PS-30 from `[DEFERRED]`, or renews them with the
disposition phase 16 gave.

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
- [ ] `unstable-projection` removed from `happenstance`, and every example that
      enabled it compiled without it.

**Proof artefact.** `examples/rebuilding-read-models` compiled and run with no
`unstable-projection` anywhere in its dependency graph, and PS-18, PS-27 and
PS-30 each with a rule and a mutant that fails it.

**Exit criteria**

- [ ] No `unstable-projection` feature in `happenstance`.
- [ ] PS-18, PS-27 and PS-30 are out of `[DEFERRED]`, or renewed per phase 16.
- [ ] A projection can write into a live batch, demonstrated against a real
      database rather than a buffer.

**Estimate.** 5–8 days.

**Session log**
