---
id: kb-decision-0072
title: Phase 17 is split at the release — what must ship in 0.4.0, and 17b after it
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0072
reversibility: high
phase: 17
supersedes: null
superseded_by: null
summary: >-
  Phase 17's work list was written at 5–8 days, and phase 16 then added the workerd and
  minimal-versions jobs, ADR-0022 §9's reproduction, ADR-0069's constructor and the VT-6 call to
  it. A read-only research pass over every item (2026-09-29, one reader per cluster plus a
  sequencing synthesis) re-estimated it at about 275 hours, 35–45 working days. The owner chose to
  split it rather than cut it. Phase 17 keeps everything whose answer changes a published
  signature or published behaviour, every record that decides such a surface, and the workerd job,
  because its measurement sets happenstance-cloudflare's published constants; it ends at 0.4.0.
  A new phase, 17b, takes the items whose recommended answer is additive: QueryItem's total
  constructor (ADR-0069), VT-14's corpus check, VT-30's alias (ADR-0054), ES-7's freeze with the
  floating-dependency job, the minimal-versions job, CF-40's MetadataLen build, and the closures of
  tuple-boundary-heterogeneous-event-type and then-empty-emission-idiom. 17b depends on 17, and
  phase 21 waits on 17b, so its freezes stay inside 1.0. VT-14, VT-30 and ES-7 move from
  freeze-by-17 to freeze-by-17b. The rule that decides a borderline item is the roadmap's own
  ordering rule: if the answer an item is about to take would break a published crate, it returns
  to phase 17, whatever this record assigned it.
depends_on:
  - kb-decision-0066
related:
  - kb-decision-0069
  - kb-decision-0054
  - kb-decision-0043
  - kb-decision-0055
source_paths:
  - runbook/phases/17-breaking-window.md
  - runbook/phases/17b-after-the-window.md
  - runbook/README.md
  - runbook/roadmap.md
  - runbook/ledgers.md
last_reviewed: 2026-09-29
---

# Phase 17 is split at the release — what must ship in 0.4.0, and 17b after it

## The question

Phase 17 was scoped as "every change to a published crate that 1.0 needs and that breaks a
signature, decided and landed in one release". Phase 16 then gave it five more items, and its own
handover asked for a re-estimate at the start of phase 17. The re-estimate came back at about 275
hours: 35–45 working days against the phase file's 5–8. Most of the difference is three items the
original estimate did not carry. The `workerd` harness is about 26 hours. `AppendError::Busy`
across three adapters and the testkit is about 20. The ES-11 fence spike on Neon is about 16.

The phase could be cut, deferring heavy items by re-disposition, or kept whole, or split. Cutting
weakens claims ADR-0066 has already made: `happenstance-cloudflare`'s conformance on the real
runtime, and ES-11 for Neon. Keeping it whole holds every additive item hostage to the slowest
breaking one, and additive work does not need the window at all.

## Decision

**Split at the release.** Phase 17 keeps its number, its file and its proof artefact, and ends at
`0.4.0`. A new phase, **17b**, follows it.

**Phase 17 keeps** every item whose answer changes a published signature or published behaviour,
and every record deciding such a surface:

- VT-10's spike;
- ADR-0028;
- the `apply` record and the port-clauses record (PS-9, PS-11, PS-15, PS-22 – PS-25, PS-38);
- CF-23's renames;
- ADR-0057's execution;
- the removal of `unstable-projection` from `happenstance-core`;
- `naive-arm`;
- `Busy` (with ES-6's prose);
- the `workerd` job and the Cloudflare partition it forces;
- ES-17;
- ES-11 and ES-12;
- the SQL-seam statement type;
- VT-6;
- ADR-0022 §9 with the guard-plan assertion, which touches the same file;
- `ProjectionId`;
- `should-codec-be-sealed`;
- the release.

**17b takes**:

- `QueryItem`'s total constructor (ADR-0069);
- VT-14's corpus check and E11 reproduction;
- VT-30's alias and builder-state pass (ADR-0054);
- ES-7's freeze, with `trait-variant-caret-resolves-past-the-locked-gate` and its floating-dependency job;
- the minimal-versions job;
- CF-40's `MetadataLen` build under ADR-0043;
- the closures of `tuple-boundary-heterogeneous-event-type` and `then-empty-emission-idiom-and-the-nothing-to-do-channel`.

Each is additive under the answer its research recommended. `QueryItem::of`, the `MetadataLen`
variant and the deprecated alias are new items. ES-7 keeps the caret. The tuple keeps its
homogeneity bound, and `then(&[])` keeps its meaning.

**17b depends on 17, and phase 21 waits on 17b**, so every freeze 17b owes stays inside 1.0 and
under `cargo xtask lints`' `freeze-by-N` closure. VT-14, VT-30 and ES-7 move from `freeze-by-17` to
`freeze-by-17b` in `runbook/ledgers.md` in the same change as this record.

## The rule for a borderline item

The roadmap's ordering rule decides it. **If the answer an item is about to take would break a
published crate, the item returns to phase 17**, whatever this record assigned it. The tuple
question is the clearest case: option A is additive, but a heterogeneous design that changes
`<(X, X) as Boundary>::Event` is not. So is ES-7: an exact `=0.1.3` pin is not a signature change,
but it is a resolver promise to consumers, and the owner may prefer to make it before `0.4.0`.

## Consequences

- Phase 17's exit criteria lose `QueryItem`'s constructor, the minimal-versions job and the
  moved freezes. Its estimate becomes 25–30 days, and 17b's is 8–10.
- The 1.0 path in `runbook/roadmap.md` grows from 34–43 working days to about 62–75.
- 17b is not a breaking window. A change it lands that `cargo-semver-checks` reports as major is a
  defect in this record's classification, and is answered by moving the item back, not by
  releasing a `0.5.0`.

## Falsifier

This record is wrong if 17b ships a break, or if phase 17 is held open by an item this record
assigned it that turns out to be additive. The first is caught by 17b's release running
`cargo-semver-checks` against `0.4.0`. The second is caught by the ES-17 measurement and the
`ProjectionId` round-trip rule: both are items whose outcome decides whether they break anything.
