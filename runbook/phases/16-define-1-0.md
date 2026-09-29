# Phase 16 — Define 1.0

**Goal.** A written statement of what `1.0.0` promises, precise enough that phase
21 can check it clause by clause rather than by judgement.

**Why here.** Nothing in the repository says what 1.0 is. The closest statements
are `SECURITY.md`'s *"Pre-1.0, and honestly so"*, `CHANGELOG.md:13-33`'s account of
what semver covers and what it exempts, ADR-0037's MSRV promise, and `HS-I0006`'s charter
naming 1.0 an explicit non-goal. Every phase after this one is sequenced against
the target, and a target nobody has written down is one each phase will
re-derive differently.

**Decisions it settles.** A decision record, numbered when it is written (0066 is
free as of the split). D-1 is an input to it, not an output: the owner has
decided sync is inside 1.0 ([roadmap](../roadmap.md#decisions-taken)), and the
charter records that rather than re-arguing it.

**Work**

- [ ] **The crate set.** Which crates 1.0 covers, by name — the grep
      `grep -n '^publish' crates/*/Cargo.toml` answers what can publish, not what
      is promised. Sync and its testkit are in the set, by D-1.
- [ ] **Every non-`[FROZEN]` clause on a promised surface gets a disposition**:
      frozen before 1.0 (with the phase that does it), renewed past 1.0 against a
      named falsifier, or declared outside 1.0's surface. 41 `[PROVISIONAL]` and 12
      `[DEFERRED]` at the split, and the [ledgers](../ledgers.md) are the list.
      A disposition table that `cargo xtask spec-trace` can be held against is the
      shape to aim for; a paragraph is the shape to refuse.
- [ ] **Versioning across crates**: whether an adapter's version implies a core
      version (`.kb/open-questions/adapter-version-lockstep-and-cf-32.md`), and
      what CF-32's independent testkit number means once a conformance rule added
      after 1.0 can turn a passing adapter red.
- [ ] **The MSRV after 1.0**, extending ADR-0037.
- [ ] **Which open questions are breaking.** Classify each open question as
      breaking-if-answered or additive. The breaking ones become phase 17's work
      list. At the split, the candidates are: `should-codec-be-sealed`,
      `no-fixture-tolerance-for-transient-contention` (a `Busy` variant on
      `AppendError`), `projection-batch-sql-seam-statement-type`,
      `projection-id-is-unvalidated`, `trait-variant-caret-resolves-past-the-locked-gate`,
      `cloudflare-worker-feature-gate`, `then-empty-emission-idiom-and-the-nothing-to-do-channel`,
      `tuple-boundary-heterogeneous-event-type`, `read-page-budget-is-unspecified`,
      `adapter-version-lockstep-and-cf-32`, `cf-23-emitter-names-mandatory-and-marked-unstable`
      and `es-17-two-adapter-measurement-is-unscheduled`.
- [ ] **The clause questions the split left open**: `conflicting_position`, and
      whether ADR-0063 answered ES-10's global-versus-per-boundary question (see
      [ledgers](../ledgers.md#open-decisions)).
- [ ] **The open questions whose own deadline passed**, given to this phase by
      phase 15 (`wi-0ed2c1`: a question goes to phase 17 only if answering it
      breaks a published crate). Each is answered in its own record or closed
      with a reason; the classification item above re-reads every assignment.
      Their deadlines named `0.2.0`, phase 12, the `ProjectionStore` freeze or an
      earlier phase, and every one of those has passed. In
      `.kb/open-questions/`:
      - **Decisions**, each with a row in the [ledgers](../ledgers.md#open-decisions):
        `adr-0022-falsifiers-have-fired` (supersede, re-open or ratify §§4–15),
        `msrv-ratification-conflicts-with-the-accepted-floor` (with the MSRV item
        above), `no-workerd-class-runner-in-the-gate`,
        `nothing-owns-the-post-phase-reconciliation`,
        `postgres-fixture-read-fault-declension-is-owed` and
        `d-1-the-validated-type-has-no-total-path` (additive, an ADR owed by
        ADR-0020).
      - **Clause dispositions**, for the clause audit above:
        `cf-25-cf-26-portfolio-check-does-not-exist`,
        `cf-36-names-a-cross-reference-nothing-performs`,
        `es-23-frozen-doc-musts-adapter-half`, `read-fault-rule-has-no-clause`,
        `vt-30-provisional-marker-is-stale-and-unscheduled`,
        `reset-refusal-declension-has-no-clause`, `es-7-and-vt-9-provisional-markers`
        (its PS-4 limb goes with `Projection::apply` in phase 17),
        `cf-18-residuals-after-declension-by-inheritance` and
        `model-only-kind-memberless-dormant-or-withdrawn`.
      - **Already on the classification list above**, with their deadlines now
        passed too: `adapter-version-lockstep-and-cf-32` and
        `cf-23-emitter-names-mandatory-and-marked-unstable` stay here; the other
        eight are phase 17's.
- [ ] **`.kb` link resolution.** `cargo xtask lint-kb` checks accepted decision
      bodies and nothing else; no check resolves `related`, `depends_on` or
      `superseded_by` since redkiln retired. Decide whether one is owed before
      1.0 or recorded as not. Left by phase 15's closure of
      `accepted-atom-immutability-check-is-pre-commit-only`.
- [ ] **`happenstance-ladybug`**: inside 1.0 behind an upstream `lbug` fix, or
      published with docs.rs building it without its `driver` feature (every item
      is behind that `cfg`, so the page would be the crate-root prose only), or
      outside 1.0. Nothing in the repository weighs the second option yet.
- [ ] **The two unplanned seeds** — `measured-not-claimed` and
      `licensing-and-the-commercial-seam` — each confirmed as not a 1.0 question,
      or brought in with a reason.

**Proof artefact.** The decision record, and a per-clause disposition table
covering every clause in the provisional and deferred ledgers on a surface 1.0
promises, with no clause left without a disposition.

**Exit criteria**

- [ ] The record is accepted, and names its crate set.
- [ ] Every clause in both ledgers on a promised surface has a disposition and, where
      it is not frozen, an owning phase or a named falsifier.
- [ ] Phase 17's work list is the breaking half of the open questions, written into
      [`17-breaking-window.md`](17-breaking-window.md).
- [ ] Phase 21's dependency row names 13 and 14, as D-1 requires. (Done at the
      split; confirm the charter did not move it.)

**Estimate.** 2 days.

**Session log**
