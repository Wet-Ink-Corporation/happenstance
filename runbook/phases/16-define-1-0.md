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
