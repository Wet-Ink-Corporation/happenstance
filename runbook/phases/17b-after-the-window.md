# Phase 17b — After the window: the additive half of phase 17

**Goal.** Every phase-17 item whose answer is additive, landed against the `0.4.0`
surface. None of it breaks a published crate.

**Why here.** [ADR-0072](../../.kb/decisions/0072-phase-17-is-split-at-the-release.md)
split phase 17 at its release. The breaking window ends at `0.4.0` and does not
wait on these items, because an additive change does not need the window. They
still come before 1.0, because the remaining `freeze-by-17b` rows in
[`ledgers.md`](../ledgers.md) and phase 21's clause audit depend on them. **The rule
that sends an item back:** if the answer an item is about to take would break a
published crate, it belongs to phase 17, and is not landed here.

**Depends on** 17. **Phase 21 waits on it.**

**Decisions it settles.** ES-7's freeze and `trait-variant`'s resolve
(`trait-variant-caret-resolves-past-the-locked-gate`). VT-14. VT-30 (ADR-0054's
alias and builder-state). The closures of `tuple-boundary-heterogeneous-event-type`
and `then-empty-emission-idiom-and-the-nothing-to-do-channel`. `QueryItem::of`'s
spelling, settled by compiling it. PS-39's freeze on a live Neon run (`wi-378078`, #81).

**Work**

- [x] #83 · **ADR-0069's total `QueryItem` constructor.** An infallible constructor
      taking a first `EventType` as its own parameter, any further types and a
      `Tags`, canonicalising exactly as `QueryItem::new` does
      (`crates/happenstance-core/src/query.rs:56-72`) through one shared private
      helper, so the two doors cannot diverge. The spelling (`QueryItem::of` is
      ADR-0069's candidate) is settled by compiling `QueryItem::of(ty, [], tags)`;
      its doc contrasts it with the fallible `of_types`. Check
      `crates/happenstance-core/tests/frozen_signatures.rs` for an enumeration it
      must join.
      **Done:** spelled `QueryItem::of`, canonicalising through the private
      `QueryItem::canonical` that `new` now shares, and pinned against `new` by a
      property test in `crates/happenstance-core/tests/query_item_of.rs`.
      `frozen_signatures.rs` lists no `QueryItem` constructors, so there was
      nothing for it to join.
- [ ] #85 · **VT-30 — ADR-0054's alias and builder-state, decided in one pass.** The
      research recommends: `after_every_guard` and `after_every_guard_opt`, with
      `after` / `after_opt` deprecated since the release that carries them and
      kept through 1.x; the two pin tests and doc sentences ADR-0054 names;
      builder-state (typestate) rejected by record. Every in-tree call site
      migrates in the same change, because the gate's `-D warnings` turns the
      deprecation into an error — so this lands **last** among 17b's code items.
      Limb 2 retired by the record or by a multi-guard `fragmented_boundary`
      bench scenario.
- [x] #86 · **VT-14 — the RTL identifier corpus check.** A standalone
      `experiments/identifier-validation/` with a path dependency on
      `happenstance-core`: the E11 reproduction run against the shipped validator
      (every `char`, alone and embedded), and an Arabic, Hebrew and Persian corpus
      with mixed LTR, its "empty" criterion pre-registered in the README before the
      run. Freeze VT-14 if it comes back empty.
      **Done:** empty (PR #239), and VT-14 is `[FROZEN]` by
      [ADR-0089](../../.kb/decisions/0089-vt-14-is-frozen-and-the-bidi-count-is-nine.md),
      which also corrects the clause's count of bidirectional controls from seven to nine.
- [ ] #88 · **ES-7 — frozen in the record that answers
      `trait-variant-caret-resolves-past-the-locked-gate`.** The research
      recommends keeping the caret, and buying the protection with a derivation
      contract compiled into `happenstance-core`'s library code (a `const _`
      closure asserting the `SendEventStore` → `EventStore` blanket and the `Send`
      append future) and a non-gating `floating-deps` sibling job on the weekly
      schedule. The falsifier is restated to cover a consumer's unlocked resolve.
      An exact `=0.1.3` pin is the alternative, and the owner may prefer to take
      it inside phase 17 (ADR-0072's borderline rule).
- [ ] #90 · **A minimal-versions CI job.** After a lockstep `1.0.0` the crates version
      independently (ADR-0066), and each adapter declares the core it needs as
      `happenstance-core = "1.N"`. A lower bound nothing ever resolves against is
      a guess, so a sibling job builds the workspace at its minimal versions.
      Leg 1 is `cargo minimal-versions check --workspace --direct --ignore-private
      --all-features` plus a `wasm32` pass for `happenstance-cloudflare` and
      `happenstance-neon`, run red once before any floor is raised; leg 2 resolves
      against the published `0.4.0`. A sibling rather than a gate step: it needs a
      nightly resolver.
- [ ] #92 · **CF-40's `MetadataLen`, built under ADR-0043.** `StoreLimit::MetadataLen`
      in `happenstance-core`; a defaulted `MAX_METADATA_LEN` on the testkit's
      `Fixture`; a metadata branch in `append_reports_exceeded_store_limits`; its
      wrong implementations in the testkit's `tests/`. `happenstance-cloudflare`'s
      ceiling is carved from the row budget measured by phase 17's deployed
      Durable Object leg, because local `workerd` does not enforce the production
      row limit. Phase 13 then decides the budget unit.
- [x] #93 · **The two typed-layer closures**, each in its own record:
      - `tuple-boundary-heterogeneous-event-type` — option A taken deliberately:
        one DCB boundary folds one domain enum; a `compile_fail` doctest fences it,
        and `boundary.rs:72-74` is corrected.
      - `then-empty-emission-idiom-and-the-nothing-to-do-channel` — `then(&[])`
        asserts the decision emitted exactly nothing, and `decide` gets no third
        channel. `then_nothing()` is additive at any date and is not added.
      Written as ADR-0090 and ADR-0091; the fence is on the two-arity tuple impl.
- [x] #81 · **PS-39 — frozen once live Neon runs `projection_ids_round_trip_by_bytes`
      green.** ADR-0082 minted it provisional at phase 17, green on memory, SQLite and
      both Postgres stores, and left one condition: a live Neon run. That run is CI's
      `live-neon` job, on its own Neon branch. Scheduled here and frozen in the same
      change by the owner's `wi-378078`.
      **Done:** green on `main` at `8b52528` (run 38271947822, job 114888156072; 20 of 20
      gated projection tests passed, and this rule declined nothing), and PS-39 is `[FROZEN]`
      by [ADR-0094](../../.kb/decisions/0094-ps-39-is-frozen-on-a-live-neon-run.md). That
      run is the proof rather than the pull request's own, because a prose-only diff
      skips `live-neon` on a pull request (`changes` says `code=false`). The merge queue
      runs it again before `main` moves, as a required check.

**Proof artefact.** The freezes of VT-14, VT-30, ES-7 and PS-39 in
`spec/SPECIFICATION.md`, PS-39's with the id of the live Neon run it rests on, and
a red then green run URL for each new sibling job, in this file's session log.

**Exit criteria**

- [ ] `QueryItem`'s total constructor (ADR-0069) is in the tree.
- [ ] VT-14, VT-30 and ES-7 are `[FROZEN]`, or re-dispositioned by a record that
      says why.
- [x] PS-39 is `[FROZEN]` on a green live Neon run of
      `projection_ids_round_trip_by_bytes`, with the run's id recorded.
- [ ] The minimal-versions and floating-dependency jobs exist, and each has been
      watched failing once.
- [ ] CF-40's `MetadataLen` is built, with a named wrong implementation per branch.
- [x] Both typed-layer open questions are closed: ADR-0090 and ADR-0091.
- [ ] #95 · `cargo-semver-checks` against `0.4.0` reports no major finding.
- [ ] The specification is reconciled against this phase's changes (session
      protocol step 6), and `cargo xtask spec-trace` passes.

**Estimate.** 8–10 days.

**Session log**

- 2026-09-29 — Created by ADR-0072, which split it from phase 17.
- 2026-10-11 — #86: the RTL corpus run in `experiments/identifier-validation/` came back
  empty (PR #239). On the owner's call, VT-14 is `[FROZEN]` by ADR-0089, and "seven"
  explicit bidirectional controls is corrected to nine wherever it was live prose. VT-30
  and ES-7 remain, so the exit criterion stays open.
- 2026-10-12 — #90: the minimal-versions job landed (PR #268), and its leg 1 was red
  locally before six floors moved. Ryan kept all six. #77's live-projection trio
  landed (PR #267); it went red in CI on a deliberately broken `reset`, then green on
  the revert. `cargo semver-checks` against `0.4.0` reports nothing for all seven
  crates, and `0.4.1` is cut. #83, #85, #88, #92 and #93 are one-way and wait on the
  owner.
- 2026-10-12 — #81: PS-39 was `freeze-by-17b` in the ledger and missing from this
  file. It is scheduled here and frozen by ADR-0094 on the owner's `wi-378078`. The
  proof is the `live-neon` run on `main` at `8b52528`, run 38271947822, where
  `projection_ids_round_trip_by_bytes` passed on its own Neon branch. The merge queue re-runs
  it as a required check.

- 2026-10-12 — #93: option A on both typed-layer questions, as the owner cleared it
  (`wi-378078`). ADR-0090 records that one boundary folds one domain enum, with a
  `compile_fail` doctest on the two-arity tuple impl and `Boundary::Event`'s sealed-trait
  rationale corrected. ADR-0091 records that `then(&[])` asserts nothing was emitted, that
  `decide` gets no third channel and that no `then_nothing()` is added. Both open questions
  are superseded. The PR is one-way and waits on the owner.
