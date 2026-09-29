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

- [x] **The crate set.** Which crates 1.0 covers, by name — the grep
      `grep -n '^publish' crates/*/Cargo.toml` answers what can publish, not what
      is promised. Sync and its testkit are in the set, by D-1.
- [x] **Every non-`[FROZEN]` clause on a promised surface gets a disposition**:
      frozen before 1.0 (with the phase that does it), renewed past 1.0 against a
      named falsifier, or declared outside 1.0's surface. 41 `[PROVISIONAL]` and 12
      `[DEFERRED]` at the split, and the [ledgers](../ledgers.md) are the list.
      A disposition table that `cargo xtask spec-trace` can be held against is the
      shape to aim for; a paragraph is the shape to refuse.
- [x] **Versioning across crates**: whether an adapter's version implies a core
      version (`.kb/open-questions/adapter-version-lockstep-and-cf-32.md`), and
      what CF-32's independent testkit number means once a conformance rule added
      after 1.0 can turn a passing adapter red.
- [x] **The MSRV after 1.0**, extending ADR-0037.
- [x] **Which open questions are breaking.** Classify each open question as
      breaking-if-answered or additive. The breaking ones become phase 17's work
      list. At the split, the candidates are: `should-codec-be-sealed`,
      `no-fixture-tolerance-for-transient-contention` (a `Busy` variant on
      `AppendError`), `projection-batch-sql-seam-statement-type`,
      `projection-id-is-unvalidated`, `trait-variant-caret-resolves-past-the-locked-gate`,
      `cloudflare-worker-feature-gate`, `then-empty-emission-idiom-and-the-nothing-to-do-channel`,
      `tuple-boundary-heterogeneous-event-type`, `read-page-budget-is-unspecified`,
      `adapter-version-lockstep-and-cf-32`, `cf-23-emitter-names-mandatory-and-marked-unstable`
      and `es-17-two-adapter-measurement-is-unscheduled`.
- [x] **The clause questions the split left open**: `conflicting_position`, and
      whether ADR-0063 answered ES-10's global-versus-per-boundary question (see
      [ledgers](../ledgers.md#open-decisions)).
- [x] **The open questions whose own deadline passed**, given to this phase by
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
        ADR-0020), `remint-identity-precondition-is-trust-only` (whether
        `remint_identity`'s documented procedure owes an in-process check; its
        own first deadline was phase 5), and
        `projection-runner-chunk-type-and-observation-seam` — a named `Chunk`
        type and an observation seam for `run_projection`. Neither is a break
        while `happenstance`'s `unstable-projection` still gates the runner, so
        both are decided here and phase 18 builds them before it lifts the gate.
      - **Clause dispositions**, for the clause audit above:
        `cf-25-cf-26-portfolio-check-does-not-exist`,
        `cf-36-names-a-cross-reference-nothing-performs`,
        `es-23-frozen-doc-musts-adapter-half`, `read-fault-rule-has-no-clause`,
        `vt-30-provisional-marker-is-stale-and-unscheduled`,
        `reset-refusal-declension-has-no-clause`, `es-7-and-vt-9-provisional-markers`
        (its PS-4 limb goes with `Projection::apply` in phase 17),
        `cf-18-residuals-after-declension-by-inheritance`,
        `model-only-kind-memberless-dormant-or-withdrawn` and
        `read-to-backwards-limit-composition-gap` (owed before the first SQL
        adapter shipped a windowed backwards read, which `0.2.0` did).
      - **Already on the classification list above**, with their deadlines now
        passed too: `adapter-version-lockstep-and-cf-32` and
        `cf-23-emitter-names-mandatory-and-marked-unstable` stay here; the other
        eight are phase 17's.
- [x] **`.kb` link resolution.** `cargo xtask lint-kb` checks accepted decision
      bodies and nothing else; no check resolves `related`, `depends_on` or
      `superseded_by` since redkiln retired. Decide whether one is owed before
      1.0 or recorded as not. Left by phase 15's closure of
      `accepted-atom-immutability-check-is-pre-commit-only`.
- [x] **`happenstance-ladybug`**: inside 1.0 behind an upstream `lbug` fix, or
      published with docs.rs building it without its `driver` feature (every item
      is behind that `cfg`, so the page would be the crate-root prose only), or
      outside 1.0. Nothing in the repository weighs the second option yet.
- [x] **The two unplanned seeds** — `measured-not-claimed` and
      `licensing-and-the-commercial-seam` — each confirmed as not a 1.0 question,
      or brought in with a reason.

**Proof artefact.** The decision record, and a per-clause disposition table
covering every clause in the provisional and deferred ledgers on a surface 1.0
promises, with no clause left without a disposition.

**Exit criteria**

- [x] The record is accepted, and names its crate set.
- [x] Every clause in both ledgers on a promised surface has a disposition and, where
      it is not frozen, an owning phase or a named falsifier.
- [x] Phase 17's work list is the breaking half of the open questions, written into
      [`17-breaking-window.md`](17-breaking-window.md).
- [x] Phase 21's dependency row names 13 and 14, as D-1 requires. (Done at the
      split; confirm the charter did not move it.)
- [x] The specification is reconciled against this phase's changes (session
      protocol step 6), and `cargo xtask spec-trace` passes. Added at this
      phase's close; every open phase now carries the same line.

**Estimate.** 2 days.

**Session log**

- 2026-09-29 — PR #24 squash-merged by the owner as `230065f`. One session, one
  branch (`lane/phase-16-define-1-0`). The work ran in three stages:
  - **Survey.** Nine read-only agents inventoried the 53 non-frozen clauses, all
    78 open questions, and the decisions owed.
  - **Adversarial verification.** It ran before anything was written. It
    refuted **13 of the 14** freeze-now candidates, which lacked evidence in the
    tree (ADR-0066 §2 records each one). It also found that 18 of the 20
    "already answered" questions were already closed. Only CF-39 is frozen.
  - **Drafting, review and fix.** Drafting was split by file ownership. A review
    pass with a completeness critic found 72 findings (1 blocker, 45 fixes, 26
    minor), and all were applied or skipped with a reason.

  **What landed:**
  - **Records.** ADR-0066 – ADR-0071. ADR-0068 settles §8 and §16 of ADR-0022
    only; §9 goes to phase 17, and its open-decision row stays live.
  - **Owner decisions.** Seven Weigh-In atoms record the owner's calls, all taken
    in the planning session: `wi-2798d5` (nine crates, ladybug out), `wi-d61f21`
    (`workerd` before 1.0), `wi-8e5bd4` (lockstep, then independent), `wi-460397`
    (MSRV held, rises bounded), `wi-1408e8` (soak with conditions and no floor;
    the recommended 14 days was declined), `wi-cbc941` (support window) and
    `wi-7899af` (licence promise).
  - **Dispositions.** `ledgers.md` gains *The 1.0 dispositions*, and the lint now
    holds it.
  - **Spec.** In `spec/SPECIFICATION.md`, line-neutral: CF-39 frozen; the
    VT-9, CF-17 and CF-34 falsifiers restated; ES-27's `Rejects:` tail replaced
    as ADR-0068 authorises; two stale `E2E-CASES.md` citations repointed.
  - **Open questions.** Ten closed, one amended, one new
    (`postgres-neon-store-id-has-no-restore-detection`, owned by phase 13), and
    twenty-five annotated with their phase-16 classification.
  - **Sequencing.** Phase 18 now runs before phase 13, because SY-20's rule
    consumes a declaration phase 18 builds. The roadmap and the status table
    record the move.

  **Not done here:**
  - ADR-0066 §5 (the semver exemptions, and ES-6 sub-question 4) is the
    record's own call, not an owner decision, and says so.
  - Phase 17's estimate does not yet count the work phase 16 added to it.

  **Verified:** `cargo xtask ci --fast` passed with every required step green, and its six
  optional steps did not run. Separately: `spec-trace`, `lints`, `lint-kb`,
  `lint-constitution`, `cargo test -p xtask`, `cargo fmt --check` and clippy on
  `xtask`. The spec's line count is unchanged at 9939. The full `cargo xtask
  ci` was not run: no crate source changed, only `xtask`, the spec and
  documents.
