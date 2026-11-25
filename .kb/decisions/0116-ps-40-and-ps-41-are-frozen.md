---
id: kb-decision-0116
title: PS-40 and PS-41 are frozen on the 0.8.0 evidence
kind: decision
status: proposed
authority_tier: decision
adr_id: ADR-0116
reversibility: low
phase: 20
supersedes: null
superseded_by: null
summary: >-
  Proposed 2026-11-25 by the v1 AFK run for the owner. PS-40 (a typed projection's checkpoint is
  pinned to its source store) and PS-41 (Projection::scopes) are the only two clauses whose 1.0
  disposition is freeze-by-20. Each ledger row states its freeze condition: its rules run green
  through a release, with no caller-supplied identity shown to drift (PS-40) and no projection
  shown to need per-scope event types (PS-41). Both rules shipped green in the published 0.8.0
  and nothing in the tree reports either falsifier. Recommended: freeze both now, reading
  "through a release" as "shipped green in a published release". The flip is the owner reading it
  as "a release after the one that minted them".
depends_on:
  - kb-decision-0109
  - kb-decision-0066
related:
  - kb-decision-0102
  - kb-decision-0110
source_paths:
  - runbook/ledgers.md
  - spec/SPECIFICATION.md
  - crates/happenstance/tests/projection_runner.rs
  - crates/happenstance/tests/projection_scopes.rs
  - examples/rebuilding-read-models/tests/rebuild_through_reset.rs
last_reviewed: 2026-11-25
---

# PS-40 and PS-41 are frozen on the 0.8.0 evidence

**Proposed 2026-11-25** by the unattended v1 run, for the owner. Nothing below has been applied.

## Context

PS-40 and PS-41 are the only two of the thirteen non-frozen clauses whose 1.0 disposition is
`freeze-by-20` (`runbook/ledgers.md:346-347`); the other eleven renew past 1.0 with falsifiers.
Both were minted `[PROVISIONAL]` at `0.8.0` (`spec/SPECIFICATION.md`, PS-40 at about `:6388`,
PS-41 at about `:6431`), and both say they are frozen "in the 1.0 freeze pass".

The freeze conditions, verbatim from the ledger:

- **PS-40:** "Freezes when `a_checkpoint_is_not_resumed_against_another_store` and
  `a_view_refuses_another_sqlite_file` have run green through a release with no caller-supplied
  identity shown to drift; if it does not, the row becomes `renew-past-1.0` against that
  falsifier, which firing would answer additively with a store-reported identity (485-Q1 B)."
- **PS-41:** "Freezes when `several_scopes_are_one_query` has run green through a release with no
  projection shown to need per-scope event types; if one does, the row becomes `renew-past-1.0`
  against that falsifier, which firing would answer additively with items of the typed layer's
  own."

## Evidence

- **The rules exist and run in the gate.** PS-40's: `crates/happenstance/tests/projection_runner.rs`
  (`a_checkpoint_is_not_resumed_against_another_store`), the two-real-SQLite-files test in
  `examples/rebuilding-read-models/tests/rebuild_through_reset.rs`, and the unit test in
  `crates/happenstance/src/checkpoint/tests.rs`. PS-41's: `crates/happenstance/tests/projection_scopes.rs`
  (`several_scopes_are_one_query`, two wrong runners, the id check and the empty-scopes refusal).
  `xtask/src/spec_trace.rs` registers each as `Elsewhere`. Examples are workspace members, so
  `cargo test --workspace --all-features` in the gate, `msrv` and `floating-deps` jobs runs them.
- **They shipped green in a published release.** `0.8.0` (2026-11-20) carries both, with the
  `CHANGELOG.md` entries and trace rows H1 and H2.
- **Neither falsifier has fired.** Nothing in `.kb/`, the runbook, the issues or the tree reports
  a caller-supplied `StoreId` that is not the store's, or a projection that needs per-scope event
  types.

## Against

- The release that "ran them through" is the release that minted them, five days old, with no
  release since.
- "Not shown" is absence of evidence. No downstream deployment is on record.
- PS-40's real-adapter evidence is SQLite only. Postgres, Neon and Cloudflare carry an inherent
  `store_id` (ADR-0109), but no runner test pins against them.

## Options

- **A. Freeze both now (recommended).** Read "through a release" as "shipped green in a published
  release". The falsifiers stay as reopening conditions, each answered additively.
- B. Freeze after the next release (`0.9.0`, carrying ES-29), which is the last before
  `1.0.0-rc.1`. Costs nothing if `0.9.0` ships soon, and reads the condition strictly.
- C. Renew both past 1.0 against their falsifiers. The ledger allows it only if a falsifier fires,
  and neither has.

## Decision (proposed)

A. On acceptance: both markers become `[FROZEN]` citing this record, the falsifier sentences
become "Reopened by …", the two ledger rows close with this record's number, and `spec-trace
--write` regenerates §7.1 and §7.2. No code changes.

## Door

One-way: a `[FROZEN]` clause is semver-binding.
