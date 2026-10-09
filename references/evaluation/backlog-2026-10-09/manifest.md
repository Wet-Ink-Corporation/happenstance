# Proposed issues, by epic

Generated from `manifest.json` (HEAD `07215eb`). One row per issue; bodies are in the JSON. Indentation is the sub-issue tree.


## Phase 17: the breaking window, released as 0.4.0  ·  milestone 0.4.0

| key | type | title | status | flags | blocked by | source |
|---|---|---|---|---|---|---|
| `epic.17` | epic | Phase 17: the breaking window, released as 0.4.0 | ready-for-human | door:one-way semver:breaking |  | `runbook/phases/17-breaking-window.md:1` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.17.f01` | feature | Release 0.4.0 with a fully traced semver table | blocked | door:one-way semver:breaking | epic.17.f02 | `runbook/phases/17-breaking-window.md:239` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.17.f01.t01` | task | Release PR: finalise 0.4.0 trace table, heading and install lines | blocked | semver:breaking | epic.17.f02 | `CHANGELOG.md:526` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.17.f01.t02` | task | Tag v0.4.0 and publish the crates | blocked | door:one-way semver:breaking | epic.17.f01.t01 | `runbook/phases/17-breaking-window.md:900` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.17.f02` | feature | ADR-0084: refuse surplus parameters on Postgres projection batches | blocked | door:one-way semver:breaking | owner.adr-0084-before-0-4-0 | `CHANGELOG.md:565` |
| &nbsp;&nbsp;&nbsp;&nbsp;`audit.IMPL-5` | feature | IMPL-5: stop exporting ProbeThenWriteStore from happenstance-neon | needs-owner | door:one-way semver:breaking |  | `references/evaluation/status-audit-2026-10-06.md:169` |
| &nbsp;&nbsp;&nbsp;&nbsp;`audit.PG-4` | feature | PG-4: reconsider a 0.3.3 carrying SQLite's 15 s busy timeout | needs-owner | door:one-way semver:additive |  | `references/evaluation/status-audit-2026-10-06.md:174` |
| &nbsp;&nbsp;&nbsp;&nbsp;`audit.docs-dx-F4` | feature | docs-dx F4: write a 0.3 to 0.4 migration guide | ready-for-agent |  |  | `references/evaluation/status-audit-2026-10-06.md:175` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.17.f03` | bug | Fix default-features doc build of happenstance (lib.rs:111 broken link) | ready-for-agent |  |  | `runbook/handover.md:49` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.17.f04` | feature | workerd deployed leg: print the HTML <title> of an unclassified 500 | ready-for-agent |  |  | `runbook/handover.md:50` |
| &nbsp;&nbsp;&nbsp;&nbsp;`owner.adr-0084-before-0-4-0` | feature | Decide: does ADR-0084's Postgres parameter-count check land before 0.4.0? | needs-owner | door:one-way semver:breaking |  | `runbook/handover.md:64` |

## Phase 17b: the additive half of phase 17  ·  milestone 1.0.0-rc.1

| key | type | title | status | flags | blocked by | source |
|---|---|---|---|---|---|---|
| `epic.17b` | epic | Phase 17b: the additive half of phase 17 | blocked |  | epic.17 | `runbook/phases/17b-after-the-window.md:1` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.17b.f01` | feature | ADR-0069: add total QueryItem constructor (QueryItem::of) | blocked | semver:additive door:one-way | epic.17 | `runbook/phases/17b-after-the-window.md:24` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.17b.f02` | feature | VT-30: add after_every_guard and deprecate after (ADR-0054) | blocked | semver:additive door:one-way | epic.17b.f01, epic.17b.f03, epic.17b.f04, epic.17b.f05, epic.17b.f06, epic.17b.f07 | `runbook/phases/17b-after-the-window.md:33` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.17b.f03` | feature | VT-14: RTL identifier corpus check in experiments/identifier-validation | ready-for-agent |  |  | `runbook/phases/17b-after-the-window.md:42` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.17b.f04` | feature | ES-7: freeze via record on trait-variant caret; add derivation contract | needs-owner | semver:additive door:one-way |  | `runbook/phases/17b-after-the-window.md:48` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.17b.f05` | feature | Add minimal-versions CI job (two legs) | ready-for-agent |  |  | `runbook/phases/17b-after-the-window.md:57` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.17b.f06` | feature | CF-40: StoreLimit::MetadataLen and testkit MAX_METADATA_LEN (ADR-0043) | blocked | semver:additive door:one-way | epic.17 | `runbook/phases/17b-after-the-window.md:66` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.17b.f07` | feature | Close the two typed-layer open questions with records | needs-owner | door:one-way |  | `runbook/phases/17b-after-the-window.md:73` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.17b.f08` | feature | Verify 17b exit: semver-checks vs 0.4.0 and spec reconciliation | blocked |  | epic.17b.f01, epic.17b.f02, epic.17b.f03, epic.17b.f04, epic.17b.f05, epic.17b.f06, epic.17b.f07 | `runbook/phases/17b-after-the-window.md:94` |
| &nbsp;&nbsp;&nbsp;&nbsp;`audit.V5` | feature | V5: run live_projection.rs in CI against live Postgres | ready-for-agent |  |  | `references/evaluation/status-audit-2026-10-06.md:171` |
| &nbsp;&nbsp;&nbsp;&nbsp;`audit.IMPL-7` | feature | IMPL-7: give the non-serialising axis generative model coverage | needs-triage |  |  | `references/evaluation/status-audit-2026-10-06.md:182` |
| &nbsp;&nbsp;&nbsp;&nbsp;`doc.ps-39-unscheduled-in-17b` | bug | PS-39 is freeze-by-17b but phase 17b never schedules it | needs-owner |  |  | `runbook/ledgers.md:312` |

## Phase 18: the typed runner leaves its gate  ·  milestone 1.0.0-rc.1

| key | type | title | status | flags | blocked by | source |
|---|---|---|---|---|---|---|
| `epic.18` | epic | Phase 18: the typed runner leaves its gate | blocked | door:one-way semver:breaking | epic.17 | `runbook/phases/18-typed-runner.md:1` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.18.f01` | feature | PS-9: make Projection::apply async per ADR-0074 | blocked | semver:breaking door:one-way | epic.17 | `runbook/phases/18-typed-runner.md:23` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.18.f01.t01` | task | Add Delivered<E> and the SendProjection async trait shape | blocked | semver:breaking door:one-way | epic.17 | `runbook/phases/18-typed-runner.md:27` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.18.f01.t02` | task | Rewrite every impl Projection in the tree to async apply | blocked |  | epic.18.f01.t01 | `runbook/phases/18-typed-runner.md:47` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.18.f01.t03` | task | Drive LivePostgresProjectionStore with a row-writing projection | blocked |  | epic.18.f01.t02 | `runbook/phases/18-typed-runner.md:52` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.18.f02` | feature | PS-28: Projection::Error and ProjectionError with failing position | blocked | semver:breaking door:one-way | epic.18.f01 | `runbook/phases/18-typed-runner.md:55` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.18.f03` | feature | PS-27: add the on_error failure-policy seam | blocked | semver:breaking door:one-way | epic.18.f01, epic.18.f02 | `runbook/phases/18-typed-runner.md:64` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.18.f04` | feature | ADR-0074 F5: savepoint caveat on a live batch | blocked |  | epic.18.f03, epic.18.f01.t03 | `runbook/phases/18-typed-runner.md:77` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.18.f05` | feature | PS-25: derive checkpoint id from name and Query digest | blocked | semver:breaking door:one-way | epic.18.f01 | `runbook/phases/18-typed-runner.md:88` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.18.f06` | feature | ADR-0074 F1: document Send bounds for generic spawners | blocked |  | epic.18.f01 | `runbook/phases/18-typed-runner.md:109` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.18.f07` | feature | PS-30/PS-38: fan-out runner for N views over one log | blocked | semver:additive door:one-way | epic.18.f01, epic.18.f10 | `runbook/phases/18-typed-runner.md:116` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.18.f08` | feature | Decide: make foreign rollback normative (optional clause) | needs-owner | door:one-way |  | `runbook/phases/18-typed-runner.md:121` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.18.f09` | feature | PS-18: implement a refusable reset in one adapter | needs-owner | door:one-way |  | `runbook/phases/18-typed-runner.md:126` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.18.f10` | feature | ADR-0070: introduce named Chunk for run_projection | blocked | semver:breaking door:one-way | epic.17 | `runbook/phases/18-typed-runner.md:128` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.18.f11` | feature | SY-20/SY-21: convergence declaration on Projection | blocked | semver:additive door:one-way | epic.18.f01 | `runbook/phases/18-typed-runner.md:138` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.18.f13` | feature | Freeze PS-16/18/25/27/30/38 and SY-21, and reconcile the specification | blocked | door:one-way | epic.18.f03, epic.18.f05, epic.18.f07, epic.18.f09, epic.18.f11, epic.18.f12 | `runbook/phases/18-typed-runner.md:143` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.18.f12` | feature | PS-16: typed-runner rebuild through reset on a multi-table read model | blocked | door:one-way | epic.18.f01 | `runbook/phases/18-typed-runner.md:144` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.18.f14` | feature | Remove unstable-projection from happenstance | blocked | semver:breaking door:one-way | epic.18.f13 | `runbook/phases/18-typed-runner.md:168` |

## Phase 13: happenstance-sync and its testkit  ·  milestone 1.0.0-rc.1

| key | type | title | status | flags | blocked by | source |
|---|---|---|---|---|---|---|
| `epic.13` | epic | Phase 13: happenstance-sync and its testkit | blocked |  | epic.17, epic.18 | `runbook/phases/13-sync.md:1` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f01` | feature | Claim happenstance-sync and happenstance-sync-testkit on crates.io | needs-owner | door:one-way |  | `runbook/phases/13-sync.md:60` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f02` | feature | Close the Postgres/Neon StoreId restore gap (VT-6, ADR-0086) | ready-for-human | semver:additive door:one-way |  | `runbook/phases/13-sync.md:66` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f03` | feature | ADR-0026: what a peer is, re-delivery, and transport assumptions | needs-owner | door:one-way |  | `runbook/phases/13-sync.md:85` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f04` | feature | ADR-0027: merge rule, scope, bulk ingest, topologies | needs-owner | door:one-way |  | `runbook/phases/13-sync.md:90` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f05` | feature | VT-10: IngestStore sync feature, real-peer ingest, oracle, error type | blocked | semver:additive door:one-way | epic.13.f03, epic.13.f04 | `runbook/phases/13-sync.md:97` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f06` | feature | ES-41: settle held-versus-visible for contains_event_id | needs-owner | semver:breaking door:one-way |  | `runbook/phases/13-sync.md:107` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f07` | feature | Create happenstance-sync-testkit and sync_peer_conformance! | blocked | semver:additive | epic.13.f03, epic.13.f04 | `runbook/phases/13-sync.md:114` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f08` | feature | MemorySyncPeer behind a memory feature, as oracle and doctest target | blocked |  | epic.13.f03 | `runbook/phases/13-sync.md:119` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f09` | feature | Sync message set, envelope types and FORMAT_VERSION semantics | blocked |  | epic.13.f03 | `runbook/phases/13-sync.md:129` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f10` | feature | Bind ingest and the runner on EventStore, not SendEventStore | blocked |  | epic.13.f03 | `runbook/phases/13-sync.md:131` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f11` | feature | Two real peers (Durable Object, Postgres/Neon) and SQLite round trips | blocked |  | epic.13.f05, epic.13.f07, epic.13.f09, epic.13.f10 | `runbook/phases/13-sync.md:134` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f11.t01` | task | Durable Object sync peer plus native SQLite round trip | blocked |  | epic.13.f05, epic.13.f07, epic.13.f09, epic.13.f10 | `runbook/phases/13-sync.md:134` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f11.t02` | task | Postgres/Neon sync peer plus native SQLite round trip | blocked |  | epic.13.f05, epic.13.f07, epic.13.f09, epic.13.f10 | `runbook/phases/13-sync.md:134` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f12` | feature | Record WF-1 DCB wire interop deferral in ADR-0026 | blocked | door:one-way | epic.13.f03 | `runbook/phases/13-sync.md:136` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f13` | feature | SY-18: build a KV-capped fixture peer (128 KiB per value) | blocked |  | epic.13.f07 | `runbook/phases/13-sync.md:139` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f14` | feature | Build the filtered-subset store for SY-27/SY-28 | blocked |  | epic.13.f04, epic.13.f07 | `runbook/phases/13-sync.md:149` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f15` | feature | Freeze the freeze-by-13 clauses (or re-disposition each) | blocked | door:one-way | epic.13.f04, epic.13.f07 | `runbook/phases/13-sync.md:158` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f15.t01` | task | VT-6: restored_peer_does_not_reissue_identities with negative control | blocked | door:one-way | epic.13.f02, epic.13.f07 | `runbook/phases/13-sync.md:160` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f15.t02` | task | VT-9: ingest preserves RecordedAt, with mutant | blocked | door:one-way | epic.13.f07 | `runbook/phases/13-sync.md:162` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f15.t03` | task | SY-7 and SY-23: adjudicator falsification and merge rule | blocked | door:one-way | epic.13.f04, epic.13.f08 | `runbook/phases/13-sync.md:168` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f15.t04` | task | SY-10: both topologies expressible, directional_merge_rules_compose | blocked | door:one-way | epic.13.f04 | `runbook/phases/13-sync.md:170` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f15.t05` | task | SY-14, VT-24, VT-21: bulk ingest measured against the Neon peer | blocked | door:one-way | epic.13.f11, epic.13.f13 | `runbook/phases/13-sync.md:164` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f15.t06` | task | SY-27, SY-28, SY-29: freeze on the filtered-subset store | blocked | door:one-way | epic.13.f14 | `runbook/phases/13-sync.md:181` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f15.t07` | task | SY-20 and SY-22: convergence and cost-layers rules | blocked | door:one-way | epic.18, epic.13.f07 | `runbook/phases/13-sync.md:174` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f15.t08` | task | SY-30, SY-31, CF-40: real envelopes, runner half, budget unit | blocked | door:one-way | epic.13.f11 | `runbook/phases/13-sync.md:183` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f16` | feature | CF-25: peer-port freeze bar and the missing portfolio check | needs-owner | door:one-way |  | `runbook/phases/13-sync.md:192` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f17` | feature | Lift ADR-0003 provisional via the byte-identical round trip | blocked | door:one-way | epic.13.f11 | `runbook/phases/13-sync.md:221` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.13.f18` | feature | Settle or renew every [DEFERRED] SY clause against a named experiment | blocked | door:one-way | epic.13.f04 | `runbook/phases/13-sync.md:222` |
| &nbsp;&nbsp;&nbsp;&nbsp;`oq.cf-36-names-a-cross-reference-nothing-performs` | feature | Decide: discharge route for the CF-36 undischarged cross-references | needs-owner |  |  | `.kb/open-questions/cf-36-names-a-cross-reference-nothing-performs.md:138` |

## Phase 14: retention, deletion and completeness  ·  milestone 1.0.0-rc.1

| key | type | title | status | flags | blocked by | source |
|---|---|---|---|---|---|---|
| `epic.14` | epic | Phase 14: retention, deletion and completeness | blocked | door:one-way | epic.13, epic.17 | `runbook/phases/14-retention.md:1` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.14.f01` | feature | CF-27: build the completeness instrument in the testkit | blocked |  | epic.17, epic.13 | `runbook/phases/14-retention.md:41` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.14.f02` | feature | Add a removal capability to Fixture, declined by default | blocked | door:one-way semver:additive | epic.17 | `runbook/phases/14-retention.md:52` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.14.f03` | feature | ES-38: positions_are_not_reused_after_removal | blocked |  | epic.14.f01, epic.14.f02 | `runbook/phases/14-retention.md:57` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.14.f04` | feature | ES-40: condition_over_removed_history_does_not_reject | blocked |  | epic.14.f01 | `runbook/phases/14-retention.md:61` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.14.f05` | feature | ES-39: run the reader experiment and record the outcome | blocked | semver:additive | epic.14.f01 | `runbook/phases/14-retention.md:65` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.14.f06` | feature | SY-32: show a 120-day-offline peer's retention gap is reported | blocked |  | epic.14.f01, epic.13 | `runbook/phases/14-retention.md:72` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.14.f07` | feature | Freeze ES-39, ES-40, SY-32, CF-27 (freeze-by-14 clauses) | blocked | door:one-way | epic.14.f03, epic.14.f04, epic.14.f05, epic.14.f06 | `runbook/phases/14-retention.md:77` |

## Phase 20: documentation that teaches (HS-I0007 Definition of Done)  ·  milestone 1.0.0-rc.1

| key | type | title | status | flags | blocked by | source |
|---|---|---|---|---|---|---|
| `epic.20` | epic | Phase 20: documentation that teaches (HS-I0007 Definition of Done) | ready-for-human |  |  | `runbook/phases/20-docs-that-teach.md:1` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f01` | feature | ES-23 adapter half: point new adapter authors at the Cancellation obligation | ready-for-agent | semver:additive |  | `runbook/phases/20-docs-that-teach.md:23` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f02` | feature | Confirm HS-P0020..22 shipped or reopen them | ready-for-agent |  |  | `runbook/phases/20-docs-that-teach.md:44` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f04` | feature | HS-P0024 comprehension evidence | blocked |  | epic.20.f03, epic.20.f02 | `.bklg/docs-that-teach/comprehension-evidence/project.md:1` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f04.t01` | task | HS-P0024: resolve DT-9 persona and fix the session protocol | blocked |  | epic.20.f03, epic.20.f02 | `.bklg/docs-that-teach/comprehension-evidence/dt9-and-fixed-protocol/spec.md:25` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f04.t02` | task | HS-P0024: land the friction-log scaffold | blocked |  | epic.20.f04.t01 | `.bklg/docs-that-teach/comprehension-evidence/friction-log-skeleton/spec.md:25` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f04.t03` | task | HS-P0024: recruit and record a qualifying non-insider reader | blocked |  | epic.20.f04.t02 | `.bklg/docs-that-teach/comprehension-evidence/non-insider-recruitment/spec.md:25` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f04.t04` | task | HS-P0024: run the comprehension session and land the filled log | blocked |  | epic.20.f04.t03 | `.bklg/docs-that-teach/comprehension-evidence/session-run-against-pinned-tree/spec.md:36` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f04.t05` | task | HS-P0024: disposition every stumble (fixed, accepted or routed) | blocked |  | epic.20.f04.t04 | `.bklg/docs-that-teach/comprehension-evidence/disposition-every-stumble/spec.md:25` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f04.t06` | task | HS-P0024: land the content fixes dispositioned as fixed | blocked |  | epic.20.f04.t05 | `.bklg/docs-that-teach/comprehension-evidence/content-fixes-from-dispositions/spec.md:25` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f04.t07` | task | HS-P0024: submit the log to an owner and resolve routed ids | blocked |  | epic.20.f04.t06 | `.bklg/docs-that-teach/comprehension-evidence/route-and-escalate/spec.md:26` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f04.t08` | task | HS-P0024: state the narrow evidence claim wherever summarised | blocked |  | epic.20.f04.t07 | `.bklg/docs-that-teach/comprehension-evidence/scope-the-claim/spec.md:37` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f04.t09` | task | HS-P0024: record the second-session verdict | blocked |  | epic.20.f04.t08 | `.bklg/docs-that-teach/comprehension-evidence/second-session-decision/spec.md:26` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f04.t10` | task | HS-P0024: write the hand-off note for HS-P0025 | blocked |  | epic.20.f04.t09 | `.bklg/docs-that-teach/comprehension-evidence/handoff-note-to-closeout/spec.md:28` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f05` | feature | HS-P0025 durable, reconciled audience (terminal closeout) | blocked |  | epic.20.f04 | `.bklg/docs-that-teach/durable-audience-closeout/project.md:1` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f05.t02` | task | HS-P0025: record baseline sha, re-run spec-trace, state clause diff | blocked |  | epic.20.f04 | `.bklg/docs-that-teach/durable-audience-closeout/post-merge-clause-completeness/spec.md:26` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f05.t03` | task | HS-P0025: dispose of the charter open questions | blocked |  | epic.20.f05.t02 | `.bklg/docs-that-teach/durable-audience-closeout/charter-open-question-disposition/spec.md:25` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f05.t04` | task | HS-P0025: audit DT-1..DT-10 over the merged tree | blocked |  | epic.20.f05.t03 | `.bklg/docs-that-teach/durable-audience-closeout/design-tension-audit/spec.md:26` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f05.t05` | task | HS-P0025: author the staged persona and journey documents | blocked |  | epic.20.f05.t04 | `.bklg/docs-that-teach/durable-audience-closeout/staged-audience-payload/spec.md:25` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f05.t06` | task | HS-P0025: author the pair-by-pair audience reconciliation record | blocked |  | epic.20.f05.t05 | `.bklg/docs-that-teach/durable-audience-closeout/reconciliation-ledger/spec.md:26` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f05.t07` | task | HS-P0025: land persona and journey atoms in one ingest wave | blocked |  | epic.20.f05.t06 | `.bklg/docs-that-teach/durable-audience-closeout/audience-ingest-wave/spec.md:25` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f05.t08` | task | HS-P0025: mount every atom at all four points | blocked |  | epic.20.f05.t07 | `.bklg/docs-that-teach/durable-audience-closeout/product-layer-mounting/spec.md:25` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f05.t09` | task | HS-P0025: re-observe all fifteen DoD scenarios on a fresh checkout | blocked |  | epic.20.f05.t08 | `.bklg/docs-that-teach/durable-audience-closeout/dod-scenario-ledger/spec.md:26` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f05.t10` | task | HS-P0025: fault-inject DoD scenario 2 and capture both halves | blocked |  | epic.20.f05.t09 | `.bklg/docs-that-teach/durable-audience-closeout/scenario-two-fault-injection/spec.md:26` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f03` | feature | HS-P0023 reach and the adapter path | ready-for-agent |  |  | `.bklg/docs-that-teach/reach-and-adapter-path/_slices.md:26` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f03.t01` | task | HS-P0023: record a non-author E0034 error-site walk to the adapter account | ready-for-human |  |  | `.bklg/docs-that-teach/reach-and-adapter-path/error-site-walk-record/spec.md:31` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f03.t02` | task | HS-P0023: install the DT-10 pointer on the happenstance crate root and README | ready-for-agent |  |  | `.bklg/docs-that-teach/reach-and-adapter-path/front-door-pointer/spec.md:25` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f03.t03` | task | HS-P0023: record a keyboard-only walk from `cargo add happenstance` | blocked |  | epic.20.f03.t02 | `.bklg/docs-that-teach/reach-and-adapter-path/front-door-walk-record/spec.md:34` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f03.t04` | task | HS-P0023: add the two evaluator onward links from DR-4 stall points | ready-for-agent |  |  | `.bklg/docs-that-teach/reach-and-adapter-path/evaluator-onward-links/spec.md:29` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`epic.20.f03.t05` | task | HS-P0023: record keyboard-only walks for the two second questions | blocked |  | epic.20.f03.t04 | `.bklg/docs-that-teach/reach-and-adapter-path/second-question-walk-records/spec.md:32` |
| &nbsp;&nbsp;&nbsp;&nbsp;`oq.cf-18-residuals-after-declension-by-inheritance` | feature | Decide: CF-18 SKIP visibility and adapter README disclosure of declines | needs-owner |  |  | `.kb/open-questions/cf-18-residuals-after-declension-by-inheritance.md:96` |
| &nbsp;&nbsp;&nbsp;&nbsp;`doc.stale-records-2026-10-06` | feature | Reconcile stale records found by the 2026-10-06 audit | ready-for-agent |  |  | `references/evaluation/status-audit-2026-10-06.md:531` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`doc.stale-records-2026-10-06.t01` | task | CLAUDE.md: fix clause count, facade size and 101-of-101 | ready-for-agent |  |  | `references/evaluation/status-audit-2026-10-06.md:537` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`doc.stale-records-2026-10-06.t02` | task | SPECIFICATION.md: refresh rule counts and frozen percentages | ready-for-agent |  |  | `references/evaluation/status-audit-2026-10-06.md:541` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`doc.stale-records-2026-10-06.t03` | task | Crate READMEs: seven published, Ladybug retired | ready-for-agent |  |  | `references/evaluation/status-audit-2026-10-06.md:545` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`doc.stale-records-2026-10-06.t04` | task | runbook/handover.md: phase-15-afk-prompt.md is tracked | ready-for-agent |  |  | `references/evaluation/status-audit-2026-10-06.md:540` |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`doc.stale-records-2026-10-06.t05` | task | es-11-fence README: status says proposed, no counted run | ready-for-agent |  |  | `experiments/es-11-fence/README.md:12` |
| &nbsp;&nbsp;&nbsp;&nbsp;`doc.phase-20-progress-unbacked` | bug | Phase 20 marked in progress but its session log is empty | ready-for-human |  |  | `runbook/README.md:108` |

## Phase 22: the documentation site on GitHub Pages  ·  milestone 1.0.0-rc.1

| key | type | title | status | flags | blocked by | source |
|---|---|---|---|---|---|---|
| `epic.22` | epic | Phase 22: the documentation site on GitHub Pages | ready-for-human |  |  | `runbook/phases/22-docs-site.md:1` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.22.f01` | feature | Add rustdoc logo and favicon to the seven published crates | ready-for-agent |  |  | `runbook/phases/22-docs-site.md:57` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.22.f02` | feature | Repoint the two crate-doc links into docs/ at the site | blocked | semver:additive | owner.pages-source | `runbook/phases/22-docs-site.md:68` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.22.f03` | feature | Write the first five how-to pages for docs/ | ready-for-agent |  |  | `runbook/phases/22-docs-site.md:75` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.22.f04` | feature | Write docs/boundaries-not-aggregates.md, the prior-model bridge | ready-for-agent |  |  | `runbook/phases/22-docs-site.md:80` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.22.f05` | feature | Add site search: vendor the client library over Zola's index | ready-for-agent |  |  | `runbook/phases/22-docs-site.md:83` |
| &nbsp;&nbsp;&nbsp;&nbsp;`owner.pages-source` | feature | Set the repository's Pages source to GitHub Actions | needs-owner | door:one-way |  | `runbook/phases/22-docs-site.md:84` |

## Phase 21: publish 1.0.0  ·  milestone 1.0.0

| key | type | title | status | flags | blocked by | source |
|---|---|---|---|---|---|---|
| `epic.21` | epic | Phase 21: publish 1.0.0 | blocked | door:one-way | epic.13, epic.14, epic.17, epic.17b, epic.18, epic.20 | `runbook/phases/21-one-point-oh.md:1` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.21.f01` | feature | Watch the clause audit pass with no freeze-by-* row remaining | blocked |  | epic.13, epic.14, epic.17b | `runbook/phases/21-one-point-oh.md:19` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.21.f02` | feature | Choose and configure release tooling for post-1.0 versioning | needs-owner | door:one-way |  | `runbook/phases/21-one-point-oh.md:31` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.21.f03` | feature | Run the semver baseline against 0.4.x and trace every break | blocked |  | epic.17, epic.18 | `runbook/phases/21-one-point-oh.md:38` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.21.f04` | feature | Reconcile PUBLISHABLE with the nine promised crates | blocked | door:one-way semver:additive | epic.13 | `runbook/phases/21-one-point-oh.md:42` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.21.f05` | feature | Align each crate's MSRV statement with ADR-0067 | ready-for-agent |  |  | `runbook/phases/21-one-point-oh.md:47` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.21.f06` | feature | Run an outside-reader pass of the opening path against the rc | blocked |  | epic.20 | `runbook/phases/21-one-point-oh.md:52` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.21.f07` | feature | Cut 1.0.0-rc.1, soak, then publish 1.0.0 | blocked | door:one-way | epic.21.f01, epic.21.f03, epic.21.f04, epic.21.f08 | `runbook/phases/21-one-point-oh.md:55` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.21.f08` | feature | Rewrite SECURITY.md for a stable 1.x line | blocked | semver:additive | epic.21.f04 | `runbook/phases/21-one-point-oh.md:63` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.21.f09` | feature | Re-read phase 21 dependency row; fix 17b prose gap, decide on phase 22 | needs-owner |  |  | `runbook/phases/21-one-point-oh.md:70` |
| &nbsp;&nbsp;&nbsp;&nbsp;`oq.read-fault-rule-has-no-clause` | feature | Decide: claim the read-fault rule (and its two UNCLAIMED_PENDING_ADR siblings) | needs-owner |  |  | `.kb/open-questions/read-fault-rule-has-no-clause.md:70` |
| &nbsp;&nbsp;&nbsp;&nbsp;`audit.A7` | feature | ES-6, ES-29, ES-31: frozen clauses name rules nothing owns | needs-owner | door:one-way |  | `references/evaluation/status-audit-2026-10-06.md:173` |

## Phase 19a: SQLite on wasm32 - skeleton  ·  milestone None

| key | type | title | status | flags | blocked by | source |
|---|---|---|---|---|---|---|
| `epic.19a` | epic | Phase 19a: SQLite on wasm32 - skeleton | ready-for-human |  |  | `runbook/phases/19-sqlite-on-wasm.md:25` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.19a.f01` | feature | Turn the SQLite-on-wasm seed into phase 19's plan | needs-triage |  |  | `runbook/phases/19-sqlite-on-wasm.md:29` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.19a.f02` | feature | Add a happenstance SQLite-on-wasm skeleton crate | ready-for-agent |  |  | `runbook/phases/19-sqlite-on-wasm.md:31` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.19a.f03` | feature | Decide the driver: rusqlite on wasm or sqlite-wasm-rs | blocked |  | epic.19a.f02 | `runbook/phases/19-sqlite-on-wasm.md:34` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.19a.f04` | feature | Decide storage backends and REOPEN/SECOND_HANDLE answers | blocked |  | epic.19a.f02 | `runbook/phases/19-sqlite-on-wasm.md:37` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.19a.f05` | feature | Decide the wasm gate: headless browser or wasm-bindgen-test | blocked |  | epic.19a.f02 | `runbook/phases/19-sqlite-on-wasm.md:41` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.19a.f06` | feature | Decide whether SQLite SQL gets a home that is not an adapter | needs-owner | door:one-way |  | `runbook/ledgers.md:58` |

## Phase 19b: SQLite on wasm32 - the adapter  ·  milestone None

| key | type | title | status | flags | blocked by | source |
|---|---|---|---|---|---|---|
| `epic.19b` | epic | Phase 19b: SQLite on wasm32 - the adapter | blocked |  | epic.17, epic.19a | `runbook/phases/19-sqlite-on-wasm.md:52` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.19b.f01` | feature | Implement the wasm32 SQLite event store and projection store | blocked |  | epic.17, epic.19a | `runbook/phases/19-sqlite-on-wasm.md:57` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.19b.f02` | feature | Provide append atomicity inside one synchronous poll | blocked |  | epic.19b.f01 | `runbook/phases/19-sqlite-on-wasm.md:60` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.19b.f03` | feature | Run both conformance suites on wasm32 in the gate | blocked |  | epic.19b.f01, epic.19b.f02 | `runbook/phases/19-sqlite-on-wasm.md:64` |
| &nbsp;&nbsp;&nbsp;&nbsp;`epic.19b.f04` | feature | Offer the wasm32 SQLite store as a phase 13 peer if ready | blocked |  | epic.19b.f03 | `runbook/phases/19-sqlite-on-wasm.md:65` |

## Not under an epic

| key | type | title | status | flags | blocked by | source |
|---|---|---|---|---|---|---|
| `audit.R8` | feature | R8: happenstance-ladybug 0.0.0 reservation does not exist on crates.io | needs-owner | door:one-way |  | `references/evaluation/status-audit-2026-10-06.md:177` |
| `doc.p13-ledger-kv-cite` | bug | Fix runbook/ledgers.md:316 stale line cite into phase 13 | ready-for-agent |  |  | `runbook/ledgers.md:316` |
| `owner.happenstance-mark-png` | feature | Owner: decide fate of assets/brand/happenstance-mark.png | needs-owner |  |  | `runbook/handover.md:68` |
| `owner.merged-lane-branches` | feature | Delete merged lane/* branches on the remote | needs-owner | door:one-way |  | `runbook/handover.md:68` |
| `owner.neon-required-check` | feature | Re-add 'conformance against a live Neon endpoint' to Protect main ruleset | needs-owner | door:one-way |  | `runbook/handover.md:62` |
| `owner.weigh-in-digest` | feature | Owner: process the outstanding Weigh-In digest from phase 15 | needs-owner |  |  | `runbook/handover.md:68` |
