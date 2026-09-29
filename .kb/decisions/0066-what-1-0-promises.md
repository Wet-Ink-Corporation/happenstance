---
id: kb-decision-0066
title: What 1.0 promises — nine crates, one disposition per unfrozen clause, and CF-39 frozen
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0066
reversibility: low
phase: 16
supersedes: null
superseded_by: null
summary: >-
  The 1.0 charter. It names nine promised crates: happenstance-core, happenstance,
  happenstance-testkit, happenstance-sqlite, happenstance-cloudflare,
  happenstance-postgres, happenstance-neon, happenstance-sync and
  happenstance-sync-testkit. Sync is in the set by D-1 (kb-decision-wi-40b321),
  which this record takes as input and does not re-argue. Promised and publishable
  are different facts and are reconciled at phase 21. The two sync names were
  unclaimed on crates.io on 2026-09-29, so phase 13's claim item is on the critical
  path. happenstance-ladybug is outside 1.0 and may have its own 0.x line. Of the
  three routes weighed for it, waiting on an upstream lbug fix blocks 1.0 on a third
  party. Publishing with docs.rs building no driver (2a) would probably fail on
  intra-doc links to cfg-ed items at lib.rs:139-150. Publishing with driver and
  supplied env! values (2b) is recorded as a route, unmeasured.
  Every non-[FROZEN] clause on a promised surface gets exactly one disposition in
  runbook/ledgers.md's "The 1.0 dispositions" section: freeze-by-N (N in phase 21's
  closure: 13, 14, 17, 18), renew-past-1.0 with a falsifier, or outside-1.0 with a
  reason. cargo xtask lints holds the section. The table lives in the ledger, not in
  this atom. The long form keeps the 53-row snapshot, whose totals are 17/4/14/7
  freeze-by-13/14/17/18, 10 renewed, 0 outside and 1 frozen here.
  Adversarial verification refuted 13 of 14 proposed freezes. Each was an argument
  that a falsifier was decorative, never evidence that it had run and not fired.
  Three falsifiers are restated rather than frozen past: VT-9's clock falsifier no
  longer discriminates, CF-34's rows-examined falsifier is foreclosed by CF-33, and
  CF-17's HS-P0013/14 have answered.
  CF-39 is frozen. rusqlite and Postgres, the named instruments, ran and neither
  driver absorbed the fault, and Cloudflare and Neon arm real in-store triggers
  too. An absorbing driver can only force a declension, which the clause already
  requires. Every armed fault is a SQL trigger, and the freeze says so.
  Versioning: 1.0.0 is released in lockstep, then crates version independently.
  Adapters declare happenstance-core = "1.N", and a minimal-versions job keeps
  those bounds honest. Conformant means passes happenstance-testkit X.Y. A new
  rule for an already-frozen clause is a testkit minor, and a new requirement is
  a major. An adapter's major follows its re-exported driver (ADR-0044), and
  futures-core 0.3 is a named accepted risk. That closes
  kb-open-question-adapter-version-lockstep-001.
  Semver exemptions are enumerated. ES-6 sub-question 4: re-exported driver error
  payloads are part of the promise. Cloudflare's 1.0 claim is conformance on
  workerd, through a sibling CI job before 1.0, which closes
  kb-open-question-workerd-runner-absent-001. The MSRV is ADR-0067's. The owner
  set the rc soak (conditions, no time floor), the supported-versions policy
  (latest minor, previous major's security fixes for six months) and a licence
  promise (the nine crates stay MIT OR Apache-2.0 for all of 1.x).
  cloudflare-worker-feature-gate is closed. Phase 17's breaking list is written.
depends_on:
  - kb-decision-wi-40b321
  - kb-decision-0037
  - kb-decision-0044
  - kb-decision-0057
  - kb-decision-0063
  - kb-decision-0023
  - kb-decision-0052
related:
  - kb-decision-0067
  - kb-decision-0068
  - kb-decision-0069
  - kb-decision-0070
  - kb-decision-0071
  - kb-decision-0012
  - kb-decision-0061
  - kb-decision-0025
  - kb-open-question-adapter-version-lockstep-001
  - kb-open-question-workerd-runner-absent-001
  - kb-open-question-cloudflare-feature-gate-001
  - kb-open-question-es-6-unwritable-rule-001
  - kb-open-question-cf-33-cf-34-scope-001
  - kb-open-question-cf-17-cf-14-markers-001
  - kb-open-question-provisional-falsifiers-001
  - kb-open-question-trademark-search-001
  - kb-decision-wi-2798d5
  - kb-decision-wi-d61f21
  - kb-decision-wi-8e5bd4
  - kb-decision-wi-460397
  - kb-decision-wi-1408e8
  - kb-decision-wi-cbc941
  - kb-decision-wi-7899af
source_paths:
  - references/adr/0066-what-1-0-promises.md
  - runbook/phases/16-define-1-0.md
  - runbook/ledgers.md
  - spec/SPECIFICATION.md
  - xtask/src/lints.rs
  - xtask/src/package.rs
  - Cargo.toml
  - crates/happenstance-testkit/src/suite.rs
  - crates/happenstance-sqlite/tests/append.rs
  - crates/happenstance-postgres/tests/support/mod.rs
  - crates/happenstance-cloudflare/tests/support/mod.rs
  - crates/happenstance-neon/tests/support/mod.rs
last_reviewed: 2026-09-29
---

# What 1.0 promises — nine crates, one disposition per unfrozen clause, and CF-39 frozen

The full record, with the 53-row disposition snapshot, the per-clause
refutations and every citation, is
[`references/adr/0066-what-1-0-promises.md`](../../references/adr/0066-what-1-0-promises.md).
This atom is the summary. Section numbers match the long form.

## The one question

Nothing in the repository said what `1.0.0` is. Phase 21 has to check it one
clause at a time, so the answer has to be something a lint can hold rather than
a paragraph.

## §1 The crate set

The nine crates are `happenstance-core`, `happenstance`, `happenstance-testkit`,
`happenstance-sqlite`, `happenstance-cloudflare`, `happenstance-postgres`,
`happenstance-neon`, `happenstance-sync` and `happenstance-sync-testkit`. Sync is
in the set by D-1. This list is a **promise**, while `PUBLISHABLE`
(`xtask/src/package.rs:86`) records what **can** ship. The two are reconciled at
phase 21, and when they meet the sync crates also join the semver job's package
list and `SECURITY.md`'s scope. Neither sync name was claimed on crates.io on
2026-09-29.

**`happenstance-ladybug` is outside 1.0 (option 3).** The other routes weighed,
numbered as in the long form:

- **1. Wait for an upstream `lbug` fix.** This makes 1.0 depend on a third party
  for an adapter whose CI job has never run.
- **2a. Publish with docs.rs building no driver.** This would probably fail
  `broken_intra_doc_links = "deny"`, because of the links at `lib.rs:139-150`.
- **2b. Publish with docs.rs building the driver and supplied `env!` values.**
  Recorded as the cheapest later route to a separate `0.x` line. It has not been
  measured.

Staying outside is also cheaper for a structural reason. `lbug` is a `0.x`
driver re-exported under ADR-0044, so every `lbug` break would be a ladybug
major.

## §2 The disposition rule

Every non-`[FROZEN]` clause on a promised surface has exactly one disposition,
in one of three shapes:

- **`freeze-by-N`**, where N is 13, 14, 17 or 18 (phase 21's closure);
- **`renew-past-1.0: <falsifier>`**, allowed only where the falsifier firing
  would be additive or would relax an obligation;
- **`outside-1.0: <reason>`**.

The table lives in `runbook/ledgers.md`, in the section *"The 1.0
dispositions"*. `cargo xtask lints` holds it through
`runbook_clause_ledgers_match_the_specification`. It is kept there rather than
here because this atom is immutable and that table shrinks as phases 13, 14, 17
and 18 freeze rows. The long form carries the 53-row snapshot this record
accepted.

| Disposition | Count | Clauses |
|---|---:|---|
| freeze-by-13 | 17 | VT-6, VT-9, VT-21, VT-24, SY-7, SY-10, SY-14, SY-18, SY-20, SY-22, SY-23, SY-27, SY-28, SY-29, SY-30, SY-31, CF-40 |
| freeze-by-14 | 4 | ES-39, ES-40, SY-32, CF-27 |
| freeze-by-17 | 14 | VT-10, VT-14, VT-30, ES-7, ES-11, ES-12, ES-17, ES-41, PS-9, PS-11, PS-15, PS-22, PS-23, PS-24 |
| freeze-by-18 | 7 | PS-16, PS-18, PS-25, PS-27, PS-30, PS-38, SY-21 |
| renew-past-1.0 | 10 | VT-22, VT-23, WF-1, WF-11, ES-32, ES-35, PS-6, CF-14, CF-17, CF-34 |
| frozen here | 1 | CF-39 |

Some rows name fallbacks:

- PS-18 renews past 1.0 if phase 18 does not deliver.
- PS-30 becomes `outside-1.0` if phase 18 builds no fan-out runner. This is the
  only use of that category.
- SY-10, SY-18 and SY-23 renew past 1.0 if phase 13 cannot run their tests.

CF-14 and CF-17 are renewed rather than scheduled, because the verdict that
would freeze them comes from phase 19a, which is outside phase 21's closure.

**Thirteen of the fourteen freezes the survey proposed were refuted.** They were
VT-9, VT-14, VT-30, ES-7, ES-12, PS-15, PS-16, PS-22, PS-38, CF-14, CF-17, CF-34
and CF-40. Each one argued that a marker's falsifier was decorative. None showed
that the falsifier had been run and had not fired. A falsifier that no longer
discriminates is a reason to **restate** it, and three are restated here. Each
restatement is written into that clause's own marker in `spec/SPECIFICATION.md`
in the same change as this record, and none of the three changes maturity:

- **VT-9.** The clock falsifier has already fired, on `wasm32-unknown-unknown`,
  where `MemoryEventStore` stamps `from_millis(0)` (`memory.rs:264-274`), and it
  changed nothing. The live falsifier is now the ingest seam, meaning
  `IngestStore` fails to carry a foreign `RecordedAt` unchanged.
- **CF-34.** The rows-examined falsifier is foreclosed by `[FROZEN]` CF-33, which
  bans operation-count assertions. The live falsifier is
  `kb-open-question-cf-33-cf-34-scope-001`.
- **CF-17.** HS-P0013 and HS-P0014, the two adapters named in the marker that
  waited on them, have answered. What stays open is the medium: Cloudflare's
  reopen ran over an in-process shim, and the browser-storage verdict belongs
  to phase 19a.

## §3 CF-39 is frozen

This record is the act that moves the marker. Both instruments named in it ran,
and neither driver absorbed the fault:

- **rusqlite.** A trigger-armed 16-event append returns `Err` and leaves zero
  rows (`crates/happenstance-sqlite/tests/append.rs:848-886`). The fixture
  declines by scope and names that test.
- **Postgres.** An `AFTER INSERT` trigger fires behind the `sqlx` pool
  (`tests/support/mod.rs:459-473`). It is asserted as supported and not left at
  the default (`postgres_conformance.rs:415-429`), and it runs in CI's
  `live-postgres` job.

Two more adapters arm faults of their own:

- **Cloudflare** arms a `BEFORE INSERT` trigger. A standing control reads the
  trigger's text back out of the error the caller sees
  (`tests/support/mod.rs:193-244`).
- **Neon** arms a transaction-local trigger. Its retry loop keys only on SQLSTATE
  `40001` (`error.rs:40-48`), so it cannot absorb the fixture's `P0001`.

The rule `arming_a_mid_batch_fault_makes_the_append_fail` rejects
`NoopFaultFixture`.

The falsifier cannot force a text change. An adapter whose driver absorbs every
fault it can arm must decline under the clause's own last sentence.

The freeze makes no claim beyond that. Every armed fault in the workspace is a
SQL trigger, so the far end of the axis is unoccupied, and the freeze rests on
the decline branch. Cloudflare's run is on the shim, and §6 moves it. Neon's
live verification rests on a doc comment and on a job that is gated on a
secret.

After the freeze, `Fixture::arm_mid_batch_fault`'s signature is fixed. The
census moves to 142/40/12/7. `runbook/ledgers.md`'s CF-39 row (`:164`), which
read *"no adapter has armed a fault yet"*, is struck, and the sentence that
routed CF-39 to phases 8 and 10 (`:199`) is corrected.

## §4 Versioning

- **`1.0.0` ships in lockstep, then each crate versions independently.** Every
  adapter declares `happenstance-core = "1.N"`. Phase 17 adds a
  minimal-versions CI job, a sibling of `gate`, to keep those bounds honest.
  Release tooling is chosen at phase 21.
- **Conformance is stated against a testkit version.** Each adapter README says
  *"passes `happenstance-testkit` X.Y"*.
- **CF-29 and CF-31 stand.** A new rule that detects a violation of a clause
  already `[FROZEN]` is a testkit minor: it finds a defect and breaks nothing
  that was conformant. A new requirement needs a clause change, which needs a
  record and is a major. The same applies to `happenstance-sync-testkit`.
  `cargo-semver-checks` cannot see rules, so CF-29's changelog lint is the
  signal.
- **CF-32 is unchanged.** ADR-0057 is executed at phase 17.
- **ADR-0044 sets a named limit on the promise.** An adapter's major follows its
  re-exported driver's breaking version. `happenstance-core`'s
  `pub use futures_core` (`lib.rs:176`) makes `futures-core 0.3` a named
  accepted risk.

This closes **`kb-open-question-adapter-version-lockstep-001`**.

## §5 Semver exemptions, and ES-6 sub-question 4

The exemptions at 1.0 are:

- `happenstance`'s `unstable-projection` runner, unless phase 18 lifts it;
- `happenstance-postgres`'s `naive-arm`, if phase 17 declares it rather than
  removing it;
- `#[doc(hidden)]` items, `Display` and `Debug` text, and rustdoc prose.

`happenstance-core`'s empty `unstable-projection` is removed in `0.4.0`, so it
is not on the list. `conformance`/`ProjectionProbe` is **inside** the promise,
by ADR-0063.

**ES-6, sub-question 4.** Driver error payloads re-exported under ADR-0044 are
part of the promise, under the driver-major limit. Phase 17 writes this into
ES-6's prose. The rest of `kb-open-question-es-6-unwritable-rule-001` stays open.

§5's exemption list and the ES-6 sub-question 4 answer are this record's own
call, not a recorded owner decision, and may be revised by a superseding record
before phase 21.

## §6 Cloudflare runs on `workerd` before 1.0

Cloudflare's 1.0 claim is conformance on the real runtime, not on the
`node:sqlite` shim. Phase 17 adds a **`workerd` sibling job**, shaped like
`live-postgres` and `live-neon`: a sibling of `gate` and never a step inside
it, so ADR-0023's reason for keeping `workerd` out of the gate still holds.

ADR-0023's list of what the shim does not prove is: no isolate, no eviction, no
hibernation, no I/O gate, no mid-`await` re-entry, and no platform storage
ceilings. Two things are measured on the new job before they are promised:

- the SQL-text wall;
- ADR-0052's `MAX_QUERY_ARMS_PER_STATEMENT` and
  `MAX_QUERY_PARAMETERS_PER_STATEMENT`.

`runbook/README.md:90`'s *"every rule green under `workerd`"* is false and is
corrected. This closes **`kb-open-question-workerd-runner-absent-001`**.

## §7 MSRV

The MSRV is **ADR-0067**'s. The floor holds at 1.97.1, and the `msrv-premise`
ratification is withdrawn. After 1.0 the floor rises only in a minor, only to a
stable at least six months old at release, and always with a CHANGELOG entry.
The promise rests on resolver 3's MSRV-aware fallback. A 1.x minor reaches
consumers through `cargo update`, so ADR-0037's "a minor is incompatible below
1.0" no longer carries it.

## §8 Release

Each of the three below is the owner's call, taken 2026-09-29.

- **rc soak.** No time floor. `1.0.0` follows `1.0.0-rc.N` as soon as five
  conditions hold on that candidate:
  - all nine crates render on docs.rs from the registry;
  - `examples/outside-projection-adapter` and one other example build against
    the registry rc;
  - the semver baseline is clean;
  - phase 21's outside-reader pass is done against the rc;
  - no defect found in the rc needs an API change.

  Any API change moves to `rc.N+1`, and the conditions are checked again on the
  new candidate. There is no time floor because the registry shows no one
  external to find anything in it: `happenstance-core`'s reverse dependencies are
  this workspace's own crates, and a wait with nobody reading is decorative.
- **Supported versions.** The latest 1.x minor is supported, and the previous
  major gets security fixes for six months after the next major ships. Adapter
  majors follow driver majors (§4), so this window is what lets an adopter move
  on their own schedule.
- **Licence.** The nine crates named in §1 stay `MIT OR Apache-2.0` for all of
  1.x. A crate added later may be licensed otherwise; these nine may not.

## §9 Not 1.0 questions

- **`measured-not-claimed`.** Largely discharged: `benchmarks/` exists, and its
  benchmark-shaped falsifiers live in §2's table.
- **`licensing-and-the-commercial-seam`.** Its only 1.0 consequence is §8's
  licence promise over the nine crates. The commercial seam itself lies outside
  those crates, so it stays a seed.
- **The trademark search.** It stays open on its own triggers.
- **`.kb` link resolution.** Not owed before 1.0, because `.kb` ships in no
  `.crate`. Measured on 2026-09-29: of 1,023 frontmatter references, 10 are
  unresolved, and all 10 are template placeholders.
- **`cloudflare-worker-feature-gate`.** No features table: `worker` reaches
  every construction path, so a feature that excluded it would exclude the
  adapter. This closes **`kb-open-question-cloudflare-feature-gate-001`**.
- **`conflicting_position`.** Settled as a hint by `[FROZEN]` ES-25 and ADR-0012
  §8. The ledger row is struck.

## §10 Phase 17's list: the breaking half

**Open questions that phase 17 settles:**

- `should-codec-be-sealed`
- `projection-batch-sql-seam-statement-type`
- `projection-id-is-unvalidated`, together with SY-31's `sync/` prefix
- `then-empty-emission-idiom-and-the-nothing-to-do-channel`
- `tuple-boundary-heterogeneous-event-type`
- `es-17-two-adapter-measurement-is-unscheduled`
- `no-fixture-tolerance-for-transient-contention`
- `cf-23-emitter-names-mandatory-and-marked-unstable`. The policy is decided
  here: the emitter names CF-23 obliges adapters to write are inside the
  promise. Phase 17 lands the renames.
- ES-6's sub-question 4, as prose.

**Reclassified as additive:**

- `read-page-budget-is-unspecified`
- `trait-variant-caret-resolves-past-the-locked-gate`. ES-7's freeze still rides
  on its record in phase 17.

**Sent to phase 17 by this charter:**

- the `workerd` job and its two measurements;
- the minimal-versions job;
- ADR-0057;
- removing `happenstance-core`'s empty `unstable-projection`;
- `naive-arm`;
- the ES-11 record that supersedes ADR-0061's keep, now that Neon is promised,
  settling ES-11 and ES-12 together;
- every `freeze-by-17` row.

## Rejected

- **Counting the crate set rather than naming it.** CLAUDE.md records a count
  that drifted twice.
- **Carrying the table in this atom.** The atom is immutable and the table
  shrinks.
- **Freezing on argument.** Thirteen proposals did exactly that.
- **Renewing CF-39.** Its falsifier cannot change its text, and the fixture
  surface is cheapest to fix now.
- **Full lockstep, a shared version, or adapters kept at 0.x.** Each of these
  contradicts CF-32, turns every driver break into a core major, or promises
  less than the named set.
- **Every new rule a major, or versioned bars.** Majors would stop carrying
  signal, and exact pinning is already free.
- **A `workerd` runner inside the gate.** It costs the single-command property
  on Windows.
- **Scoping Cloudflare's claim to the shim.** Rejected by the owner.

## Falsifier

Reopened by any of:

- sync needing an `EventStore` seam that phase 17 missed (D-1's own falsifier,
  and a 2.0 after 1.0);
- a clause reaching phase 21 undispositioned;
- a renewed clause whose falsifier turns out to need a signature change;
- a `MID_BATCH_FAULT` fixture whose fault its driver absorbs;
- a consumer resolving two core majors under one adapter's declared bound;
- the `workerd` job proving too costly to keep. In that case Cloudflare's claim
  reverts to the shim's scope, by a superseding record;
- an outside adopter finding a defect in `1.0.0` that a time floor on the rc
  would have exposed. That reopens the soak for the next major's rc, and only
  for it.

A §8 policy changes only by a superseding record.
