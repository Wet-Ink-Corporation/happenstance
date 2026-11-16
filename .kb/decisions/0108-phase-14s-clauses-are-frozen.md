---
id: kb-decision-0108
title: Phase 14's clauses are frozen, SY-32 is held for a CF-25 acceptance, and five frozen lines are repaired
kind: decision
status: proposed
authority_tier: decision
adr_id: ADR-0108
reversibility: low
phase: 14
supersedes: null
superseded_by: null
summary: >-
  Proposed 2026-11-16 for #163; every freeze in it is the owner's call. Six clauses freeze on rules
  that run on phase-14 and the wrong implementations they fail: ES-39 as ADR-0028 decision 9's
  refusal, on experiments/retention-reader's verdict (#482); ES-40 on
  condition_over_removed_history_does_not_reject (#474), with the completeness axis's verdict moved
  to adapter yes because four real adapters remove rows outside the port and pass it; CF-27 on
  WithholdingStore (#467); SY-7 and SY-10 on the runner's adjudication hook (#469, #450, with #441
  settled by #466); and VT-36 on the runner's Split on TagsPerBatch through the deployed object
  (#475), green on two deployed runs. SY-32 is not frozen: its experiment ran on MemorySyncPeer and
  every real peer declines COMPACTION, so freezing it fails the CF-25 Completeness / SyncPeer row
  unless an accepted record takes the risk by name. It moves to provisional, keeps freeze-by-14,
  and section 3 is that acceptance, effective only if the owner accepts this record. ES-38's,
  SY-3's, SY-21's and SY-34's Rule lines and the Durability / SyncPeer ledger cell are repaired.
depends_on:
  - kb-decision-0013
  - kb-decision-0028
  - kb-decision-0066
  - kb-decision-0100
  - kb-decision-0107
related:
  - kb-decision-0026
  - kb-decision-0027
  - kb-decision-0098
  - kb-decision-0103
source_paths:
  - spec/SPECIFICATION.md
  - runbook/ledgers.md
  - runbook/phases/14-retention.md
  - experiments/retention-reader/README.md
  - crates/happenstance-testkit/src/withholding.rs
  - crates/happenstance-testkit/tests/completeness_instrument.rs
  - crates/happenstance-testkit/tests/mutation_coverage/removed_history.rs
  - crates/happenstance-sync/src/runner.rs
  - crates/happenstance-sync/tests/runner/wrong_runners.rs
  - crates/happenstance-sync/tests/runner/main.rs
  - crates/happenstance-sync-testkit/tests/wrong_retention.rs
  - harness/workerd/src/sync_mount.rs
  - xtask/src/spec_trace/sync_family.rs
last_reviewed: 2026-11-16
---

# Phase 14's clauses are frozen, SY-32 is held for a CF-25 acceptance, and five frozen lines are repaired

**Proposed on 2026-11-16 for #163.** Every freeze in it is the owner's call, and each one is its
own commit on `claude/project-thread-xxl8lr-163`. Declining one means reverting that commit and
re-reconciling the census totals (§1.3, §7.1) and the sync docs, which the last commit writes
assuming the others; the fallback for each is the `freeze-by-14` row it had. No long-form record exists; this
atom is the record.

## Context

ADR-0066 §2 gave phase 14 the `freeze-by-14` rows of `runbook/ledgers.md`: ES-39, ES-40, SY-32 and
CF-27, which ADR-0028 decided at phase 17 and left to this phase to build and freeze. ADR-0107 added
SY-7, SY-10 and VT-36, because phase 14 is the last phase before 1.0 that a `freeze-by-N` may name
(`freeze-by-21` is refused; 21 is the audit). Phase 14's exit criteria require each one `[FROZEN]`
or re-dispositioned by a record, and ES-39, CF-27 and SY-32 no longer `[DEFERRED]`. The work is on
`phase-14` at `363dcdca`: #467 (#144), #469 (#450), #466 (#441), #471 (#147), #474 (#151, #154),
#475, #476 (#452), #477 (#448), #478 (#457), #473 (#158), #482 (#156) and #481 (#458).

## 1. Frozen

Each marker restates what reopens it. From `1.0.0`, loosening any of these is a major.

1. **ES-39, as the refusal.** ADR-0028 decision 9 pre-authorised two outcomes and
   `experiments/retention-reader/` chose between them by criteria committed before any test ran
   (`4eb3532a`): a reader needs a port report only if it meets C1 (harm), C2 (undetectable today),
   C3 (a report defaulting to `Unknown` cures it, generically) and C4 (`EventStore` is the seam).
   Four readers, R1 the typed runner, R2-src and R2-dst the ingest path from and into a holed store,
   and R3 a decision model (recorded only), ran over the instrument and over a raw SQLite `DELETE`;
   23 tests, all passing, assert what was observed. Four harms were found (R1-4, R2-dst-1, R2-dst-2,
   R2-dst-3), every one fails C4 and each fails C3 or C2 as well, so the verdict is **refusal**, and
   no reader met C1, C2 and C4 together, so ADR-0028's falsifier ("the report would have to be
   required") did not fire. The marker becomes: the port does not report completeness, and a caller
   MUST NOT infer it from any port value. Its rules are the two the refusal names, ES-38's and
   ES-40's, against CF-27's instrument and every fixture granting `REMOVAL`.
   `a_store_reports_the_history_it_does_not_hold` is not written and leaves `spec_trace.rs`'s
   table. **The harms are real and their cures belong outside `EventStore`:** #483 (pruned foreign
   events resurrected on a re-pull: `IngestStore::ingest` or a persisted pull token), #484 (a
   pruned own event wedges the edge with a false `UnheldOwnIdentity`: the adapter's ingest and
   SY-38's wording) and #485 (a projection checkpoint is not pinned to its source store: the typed
   checkpoint). Reopened by a reader that meets all four criteria; if it fails only C3 because
   `Unknown` cannot be the default, that is ADR-0028's falsifier and goes to the owner.
2. **ES-40.** `condition_over_removed_history_does_not_reject` (#474) refuses while the matching
   event is held, removes it, then requires the append to be admitted, and still refuses over
   survivors on either side of the hole. It fails `RememberingConditionFixture` (a soft delete),
   `FailClosedFixture` and `ConditionBlindFixture`, and runs against `PrunedMemoryFixture` over
   `WithholdingStore` and against every fixture granting `REMOVAL`: SQLite in the gate,
   `happenstance-postgres` in `live-postgres`, `happenstance-neon` in `live-neon`, and the Durable
   Object under `workerd`, locally, cold and deployed (green on run 43746838635, among others). The
   marker said the one observation that would still move it is an instrument that cannot stage the
   removal; none failed to. **CF-25.** The marker carried the completeness axis for `EventStore`
   (ADR-0013 §7), so freezing it fails that row unless it is discharged. It is discharged as
   `far end`: the axis's far end is "a log with a scattered hole", and since #147 four real
   adapters hold one, made by a raw `DELETE` outside the port as a deployment would make it, and
   pass ES-38's and ES-40's rules over it. The verdict cell moves from *Fixture yes, adapter no* to
   *Fixture yes, adapter yes*. That is a judgment the owner should check: the removal goes through
   a fixture capability, but the store, the medium and the `DELETE` are the adapter's own. §1.3's
   residual-exposure paragraph and the ledger's residual row drop ES-40. Reopened only by
   ADR-0028's own falsifier; E2E-47's third outcome stays rejected for 1.x.
3. **CF-27.** `WithholdingStore` (#467) is the instrument: a decorator over any `EventStore`
   holding a suffix or a scattered retained set, reporting it only through its inherent
   `withheld()`. `instrument_report_is_accurate_and_the_suite_cannot_tell` runs the event-store,
   concurrency and model families against it with twelve withheld seeds carrying the rules' own
   types and tags, records the pass list in
   `crates/happenstance-testkit/tests/completeness_instrument/pass_list.txt` (104 rules, 8
   capability skips at #467; the guard, ES-38 and ES-40 since #474), and fails a range report, a
   last-call-only report and leaky membership. **Recorded limit:** the instrument masks ingest-side
   harms (the experiment's two controls), so claims about ingest need the `REMOVAL` capability, not
   the decorator. Reopened by an instrument whose report misstates what it withholds while the
   suite passes, or by a rule the suite gains that can tell a holed log from a young one, which
   would mean the port reports completeness after all.
4. **SY-7.** The condition ADR-0107 set is met: the hook exists and mounts the rule. `runner::Adjudicate`
   (#469, #450) runs on both ingest sites, `Edge<R, A = Accept>::with_adjudicator` on the pull and
   `StoreSyncPeer<S, I, A = Accept>::with_adjudicator` behind `serve_sync_adjudicated` on the push;
   `Accept`, which writes nothing, is the default everywhere. `only_the_adjudicator_compensates`
   mounts a seat rule on the adjudicator and `Accept` on the other and fails
   `FilesCompensationUnderAPlaceholder`; the runner half, that a non-adjudicator writes none, is
   `tests/runner/wrong_runners.rs`, where the shipped `Edge` passes and `MiniEdge` fails
   `accept_writes_no_compensation` when it adjudicates under `Accept`, each defect failing its own
   rule only. The falsification test ran and recorded its divergence (#136). Reopened by its own
   falsifier: two adjudicators for one family whose compensations can be made convergent.
5. **SY-10.** `directional_merge_rules_compose` mounts its rules through the shipped hook, and a hub
   driving its own edge now adjudicates what it pulls (`with_adjudicator`), so the per-direction
   rule holds whichever side drives. #441 is settled by #466, which removed the refused-push check
   no store could fail; SY-34's refusal is the runner's half. `tests/runner/directions.rs` walks
   both topologies through one `Edge` with no role parameter, and `wrong_runners.rs`'s
   `an_edge_judges_nothing_it_pushes` fails a runner that adjudicates its own push. Reopened by its
   own falsifier: a hub and a spoke shown to need the same merge rule.
6. **VT-36.** The condition was the 262,144-tag branch of `append_reports_exceeded_store_limits`
   green on the deployed object across runs, and the floor meeting a sync ingest that splits on
   `TagsPerBatch` (ADR-0100, ADR-0107 §3). #475 adds `/sync-wide/` to the harness and
   `the_runner_splits_a_push_on_tags_per_batch_through_the_object`: two write groups of 129 events
   of 1,024 tags (264,192) pushed over HTTPS, the first push refused `413 Split` with the ingest's
   own `tags per batch limit: 264192` text, so a body-cap refusal cannot pass, the edge running
   with last failure `(Push, Split)` and nothing held, then one group a push, both landing whole.
   It ran green on the **deployed** leg of the `workerd` job on run **43721323137** (#477, job
   132833872166) and run **43746838635** (#482, job 132911938166), each beside a deployed
   `ok append_reports_exceeded_store_limits`. Reopened by its falsifier: a store whose per-call
   wall is not a function of the tag count.

## 2. Repaired lines in frozen clauses

None changes a MUST.

- **ES-38's Rule bullet** (eleven lines for eleven, from `.kb/_intake/es-38-rule-line-after-151.md`,
  removed): `positions_are_not_reused_after_removal` exists (#151), runs on the instrument and on
  every `REMOVAL` fixture, and fails `RowidReuseFixture`, `SequenceHeadFixture` and
  `TruncatingWhenEmptyFixture`. The phase-4 sentence "`Fixture` declares no such capability" is
  false since #147 and is gone.
- **SY-3's Rule line** is repointed to SY-12's `dedupe_reaches_identity_without_decoding`, which
  delivers a group carrying a compensation twice, requires one compensation held, and fails
  `RecompensatesOnRedelivery`. That is the assertion the never-written
  `compensation_is_idempotent_under_redelivery` stated. Its declaration leaves
  `SYNC_UNRESOLVABLE_RULE_NAMES` (21 entries to 20).
- **SY-34's Rule line** (#465) is repointed to `happenstance-sync`'s
  `a_refused_push_keeps_the_log_and_the_watermark_and_records_the_class`, declared `Elsewhere`.
  **Gap stated:** it holds the shipped `Edge` only; no rollback runner is built to fail it.
- **SY-33's line is left as it is** (#465 stays open). No test anywhere drives a receiver into its
  refusals and varies its log; `every_refusal_states_its_class` drives one refusal per real peer
  and checks its class, not its independence from the log. Its declaration stays `Scheduled`.
- **SY-21's Rule line** no longer says "phase 13 adds": `convergent_projection_cannot_observe_local_position`
  exists (#457, #478) and fails `RecordsItsCheckpoint`. Its reading, D-457-2 (the checkpoint may
  not reach the read model), is the owner's to confirm.
- **§6.5's Durability / SyncPeer cell** (D-448-4, #477): the rule reopens where
  `SyncFixture::REOPEN` is granted, Postgres and Neon, and fails `BuffersAcrossHandles` there; the
  Durable Object's fixture declines and is reconnected, which is the stated cost of a declension.

## 3. Held: SY-32, and the risk acceptance it needs

**SY-32 is not frozen by this record's commits.** #473 built what its marker asked:
`FailureClass::Reseed`, `StopCause::RetentionGap`, `MemorySyncPeer::compact_recorded_before` with a
computed `retention_floor` and `floor_token()`, and `retention_gap_is_reported_not_silent` behind
`SyncFixture::COMPACTION`, which fails seven wrong peers in `tests/wrong_retention.rs`, among them
the 120-days-offline peer against a 90-day window answered with a silent subset. The experiment's
question is answered yes: the scalar floor detects the offline-too-long peer. Two things fall short
of a freeze:

- the stand-in was `MemorySyncPeer`'s own compaction, not CF-27's instrument, and every real peer
  (Neon, the Postgres `StoreSyncPeer`, the Durable Object) declines `COMPACTION`, so the rule skips
  on all of them and each declares `retention_floor: None`;
- SY-32 carries the completeness axis for `SyncPeer` in §6.5, a frozen port. Freezing it fails that
  row unless an adapter sits at the far end for this port, which none does, or an **accepted**
  record names the axis and the port. The verdict section 1.2 moves is the axis's, and reading it as
  this port's far end would be pro forma, which ADR-0103 declined for the same reason.

So the marker moves `[DEFERRED]` to `[PROVISIONAL]`, because its experiment ran, and keeps
`freeze-by-14`. **The acceptance, effective only on this record's acceptance:** the completeness
axis is accepted for `SyncPeer` without an adapter at its far end. SY-32's floor means resumability,
not completeness (ADR-0028), so a peer is never asked to describe a scattered purge, and the harms a
pruned replica causes (#483, #484) belong to the ingest path, not the peer port. Retired when a real
peer grants `COMPACTION` and passes the rule. If the owner accepts, a follow-up commit freezes SY-32
and writes `ADR-0108` in that row; if he declines this section, SY-32 stays `freeze-by-14` and
phase 14 must build a compacting real peer or renew it past 1.0 (`renew-past-1.0` with the
falsifier its marker now states) before it closes.

## Open points recorded at the proposal

- **A. ES-40's verdict move** (section 1.2). Recommended: adapter yes, as argued. Alternative: hold
  ES-40 as SY-32 is held, with an acceptance naming the axis and `EventStore`.
- **B. SY-32** (section 3). Recommended: accept section 3 and freeze in a follow-up. Alternatives:
  renew past 1.0 against a real compacting peer, or build one in phase 14.
- **C. SY-34's repoint** names a test with no wrong runner beside it. Recommended: accept, and file
  a rollback `MiniEdge` defect as a follow-up. Alternative: decline the repoint and write the rule.
- **D. #473's Durable Object skip.** The `workerd` harness tolerates a skip of
  `retention_gap_is_reported_not_silent` with exactly `UNSTATED_COMPACTION`'s reason (D10 there).
  It is consistent with section 3 and is not a condition of anything frozen here.

## Consequences

- §1.3 reads 193 `[FROZEN]`, 10 `[PROVISIONAL]` and 2 `[DEFERRED]` (WF-1, CF-14) with every
  commit applied: 187, 13 and 5 before.
- The ledger's `freeze-by-14` rows for ES-39, ES-40, CF-27, SY-7, SY-10 and VT-36 are struck;
  SY-32's keeps `freeze-by-14` with this record's reason.
- §6.5: Completeness / `EventStore` is `far end`; Completeness / `SyncPeer` stays carried by SY-32.
- The evidence for ES-40 on Postgres, Neon and the Durable Object, and for VT-36, is in CI's live
  jobs, not the gate. A regression there turns `live-postgres`, `live-neon` or `workerd` red.
