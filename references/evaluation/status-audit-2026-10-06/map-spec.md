# Audit map — THE SPECIFICATION

Worktree: `audit-wt` at `1f92d08` (== origin/main). Read-only. Date 2026-10-06.
Sources: `spec/SPECIFICATION.md` (10,368 lines), `spec/E2E-CASES.md` (1,684 lines), `runbook/ledgers.md` (341 lines), `xtask/src/spec_trace.rs`, `crates/happenstance-testkit/src/*`.
Last commits touching the files: SPECIFICATION.md `1f92d08` (2026-10-05); E2E-CASES.md `9d99339` (2026-10-01).

Method note: clause markers in the body are written in inconsistent shapes (`#### VT-1 —` headings for VT/WF/ES; `**PS-1 — …` / `**SY-7.` / `**CF-14.**` bold paragraphs for PS/SY/CF; markers as `**[FROZEN]**`, `` `[FROZEN]` `` etc.), so a naive `grep -c '\[FROZEN\]'` is meaningless (`grep -c '^\*\*\[FROZEN\]\*\*'` = 34; `grep -E '^#{2,6} [A-Z]{2,3}-[0-9]+'` finds only 89 headings — VT/WF/ES only). I therefore counted from the generated §7.2 table (`SPECIFICATION.md:9787-10037`, between `BEGIN/END GENERATED` markers, regenerated and diff-checked by `cargo xtask spec-trace` per `SPECIFICATION.md:9716-9726`): `grep -E '^\| [A-Z]{2}-[0-9]+ \|' spec/SPECIFICATION.md` = **203 rows**, then grouped by prefix × maturity myself.

## 1. Clause counts — family × marker (my count from §7.2 rows)

| Family | Section | Total | FROZEN | PROVISIONAL | DEFERRED | NON-NORMATIVE | % frozen |
|---|---|---|---|---|---|---|---|
| VT | §2.1–2.6 value types | 34 | 25 | 8 | 0 | 1 | 74% |
| WF | §2.7 wire format | 12 | 10 | 1 | 1 | 0 | 83% |
| ES | §3 EventStore | 43 | 35 | 7 | 1 | 0 | 81% |
| PS | §4 ProjectionStore | 38 | 26 | 4 | 3 | 5 | 68% |
| SY | §5 SyncPeer | 35 | 21 | 9 | 5 | 0 | 60% |
| CF | §6 conformance | 41 | 35 | 3 | 2 | 1 | 85% |
| **Total** | | **203** | **152** | **32** | **12** | **7** | 75% |

- My count **matches** the generated §7.1 summary (`SPECIFICATION.md:9791-9800`) and the §1.3 prose (`SPECIFICATION.md:228-236`: "203 clause IDs … 152 FROZEN, 32 PROVISIONAL, 12 DEFERRED and seven NON-NORMATIVE").
- It also matches the ledger headings: "The 12 `[DEFERRED]` clauses" / "The 32 `[PROVISIONAL]` clauses" (`runbook/ledgers.md`), whose set-equality with §7.2 is gate-checked by `cargo xtask lints` (`runbook_clause_ledgers_match_the_specification`, per `ledgers.md:9-12`).
- **Stale prose:** `SPECIFICATION.md:9775` says "`ES` is 76 % frozen" (actual 35/43 = 81%) and `:9782` says "`PS` is 51 % frozen because it has no implementation at all" (actual 26/38 = 68%; PS has passing adapters — memory, sqlite, postgres buffered + live, neon, cloudflare per CLAUDE.md and §1.3 `:262-271`). This authored paragraph sits outside the generated block and has not been updated.
- **Stale rule count prose:** §1.6 / §3 say "eighty-nine rules" (`SPECIFICATION.md:409, 2512, 3362`); `grep -c 'pub async fn' crates/happenstance-testkit/src/suite.rs` = 95 and `…/projection.rs` = 21. Counting basis may differ (unverified which).

## 2. Every [DEFERRED] clause (12)

| Clause | Gist (clause heading) | Line | Owner per ledger (old) | 1.0 disposition (`ledgers.md` "The 1.0 dispositions") |
|---|---|---|---|---|
| WF-1 | Wire format private; DCB interoperability deferred | 1990 | 13 | renew-past-1.0 (a DCB impl publishes a wire encoding) |
| ES-39 | Whether a store can declare what it does not hold | 4785 | 14 | freeze-by-14 (decided at 17 by ADR-0028: refusal + additive reservation) |
| PS-18 | An adapter MUST be able to refuse a reset | 5920 | "6" (HS-P0010) | freeze-by-18 |
| PS-27 | Failure policy MUST offer skip-and-record, written atomically | 6256 | "6" | freeze-by-18 (seam shape fixed by ADR-0074) |
| PS-30 | Fan-out runner catching a panic in `apply` MUST rollback | 6366 | "6" | freeze-by-18, else outside-1.0 |
| SY-14 | Bulk ingest with already-seen events in bounded round trips | 7253 | 13 | freeze-by-13 |
| SY-18 | A peer declares the limits it imposes | 7381 | 13 | freeze-by-13 (else renew) |
| SY-27 | Whole-log vs scoped replication | 7662 | 13 | freeze-by-13 — **sequencing hazard**: needs CF-27's suffix store, built by phase 14 which runs after 13 (`ledgers.md` "One sequencing hazard") |
| SY-28 | Round-trip rule asserts log equality only over agreed scope | 7686 | 13 | freeze-by-13 (same hazard) |
| SY-32 | Peer reports floor below which a resume token is invalid | 7808 | 14 | freeze-by-14 |
| CF-14 | Rule asserting an `Ok` append survives a reopen (durability) | 8541 | 8 | renew-past-1.0 |
| CF-27 | Workspace MUST hold a completeness (suffix-store) instrument | 9219 | 14 | freeze-by-14 |

Note: the "Owning phase" column in the old deferred table still names phase **6** for PS-18/27/30 (`ledgers.md`, deferred table), a phase long done; the ledger itself says "This is the schedule; the owner columns above are history" and the dispositions table (phase 18) is the live copy.

## 3. Every [PROVISIONAL] clause (32)

| Clause | Gist | Line | 1.0 disposition |
|---|---|---|---|
| VT-6 | `StoreId` names a store incarnation, not a device/peer | 848 | freeze-by-13 |
| VT-9 | Store records accept-time; not an ordering key | 968 | freeze-by-13 |
| VT-14 | Validation rejects control chars and bidi overrides | 1203 | freeze-by-17b (ADR-0072) |
| VT-21 | `MIN_SUPPORTED_EVENT_DATA_LEN` floor 65,536 | 1563 | freeze-by-13 |
| VT-22 | Tags-per-event floor 64 | 1593 | renew-past-1.0 |
| VT-23 | Query-items floor 128 | 1615 | renew-past-1.0 |
| VT-24 | Events-per-batch floor 128 | 1636 | freeze-by-13 |
| VT-30 | `AppendCondition` = one or more guards, each with own boundary | 1911 | freeze-by-17b |
| WF-11 | Payloads base64 in human-readable formats | 2377 | renew-past-1.0 |
| ES-7 | Downstream crate may implement the bare flavour | 2839 | freeze-by-17b |
| ES-11 | A read is a snapshot | 3080 | freeze-by-17 — **falsifier has fired** on `happenstance-neon` (marker at `:3087`; ADR-0061 states neon does not satisfy ES-11). Phase 17 item unchecked (`runbook/phases/17-breaking-window.md:162`) |
| ES-12 | All items of one Query share one snapshot | 3209 | freeze-by-17 (same record as ES-11) |
| ES-17 | The batch is borrowed, not owned | 3563 | freeze-by-17 (ADR-0055 two-build measurement owed) |
| ES-32 | No tail/subscription seam at 0.1 | 4459 | renew-past-1.0 |
| ES-35 | Acknowledged writes survive a reopen where durability claimed | 4580 | renew-past-1.0 (fault end unbuilt) |
| ES-40 | Conditional append sound only over a complete store | 4823 | freeze-by-14 |
| PS-6 | Adapter with nothing to reserve MUST NOT round-trip at `begin` | 5504 | renew-past-1.0 |
| PS-16 | `reset` applies batch and returns checkpoint to never-run | 5890 | freeze-by-18 |
| PS-25 | Checkpoint MUST NOT survive a change to its Query | 6190 | freeze-by-18 |
| PS-38 | Successful `commit` MUST advance visible checkpoint | 6146 | freeze-by-18 |
| SY-7 | Compensation authorship to at most one peer per fact | 7052 | freeze-by-13 |
| SY-10 | Hub-and-spoke and peer-to-peer both first-class | 7129 | freeze-by-13 |
| SY-20 | Convergent projection → byte-identical read models | 7448 | freeze-by-13 |
| SY-21 | Convergent projection handed EventId, not local position | 7482 | freeze-by-18 |
| SY-22 | Projection may declare itself non-convergent | 7517 | freeze-by-13 |
| SY-23 | Peer-independent deterministic order where needed | 7538 | freeze-by-13 |
| SY-29 | Peer-supplied Query validated against cost policy | 7704 | freeze-by-13 |
| SY-30 | Push envelope preserves group boundaries | 7733 | freeze-by-13 |
| SY-31 | Per-peer watermark not an event, transactional | 7764 | freeze-by-13 |
| CF-17 | Fixture SHOULD be able to reopen | 8657 | renew-past-1.0 |
| CF-34 | Performance measured by a separate harness | 9533 | renew-past-1.0 |
| CF-40 | Fixture states capacity ceilings | 8830 | freeze-by-13 |

### Where the 44 non-frozen clauses land (dispositions table, my tally)

| Target | Count | Clauses |
|---|---|---|
| freeze-by-13 (sync; **not started**, `runbook/README.md:104`) | 17 | VT-6, VT-9, VT-21, VT-24, SY-7, SY-10, SY-14, SY-18, SY-20, SY-22, SY-23, SY-27, SY-28, SY-29, SY-30, SY-31, CF-40 (+ SY-27/28 hazard) |
| freeze-by-14 (retention; not started, `README.md:105`) | 4 | ES-39, ES-40, SY-32, CF-27 |
| freeze-by-17 (in progress, `README.md:101`) | 3 | ES-11, ES-12, ES-17 |
| freeze-by-17b (not started, `README.md:102`) | 3 | VT-14, VT-30, ES-7 |
| freeze-by-18 (not started, `README.md:103`) | 7 | PS-16, PS-18, PS-25, PS-27, PS-30, PS-38, SY-21 |
| renew-past-1.0 | 10 | VT-22, VT-23, WF-1, WF-11, ES-32, ES-35, PS-6, CF-14, CF-17, CF-34 |

Check: 17+4+3+3+7+10 = 44 = 32 provisional + 12 deferred.

Phase 21 (1.0) depends on 13, 14, 16, 17, 17b, 18, 20 (`runbook/README.md:110`); **34 of 44 non-frozen clauses must be frozen by phases that are not done**, 27 of them by phases not yet started (13, 14, 17b, 18).

## 4. How clauses cite conformance rules

Each clause has `Rule:`, `Cases:`, `Rejects:` bullets (e.g. ES-8 `SPECIFICATION.md:2886-2900`). `xtask spec-trace` resolves names against `crates/happenstance-testkit/src/suite.rs` (+ `wire::` names against `crates/happenstance-core/tests/wire.rs`, `crates/happenstance-sync/tests/wire.rs`) and marks misses with † (`SPECIFICATION.md:9752-9762`). Names that live elsewhere are declared in an allowlist in `xtask/src/spec_trace.rs` as `Unresolvable::Elsewhere(path)` (34 entries), `Scheduled(reason)` (16), `NotARuleName` (5) (`grep -c` over `spec_trace.rs`), and the checker verifies the `Elsewhere` file still contains the identifier (`spec_trace.rs:859, 999`).

### Sample verification (21 names across families; `grep -rn "fn <name>\b" crates xtask`)

| Clause | Rule name | Found at |
|---|---|---|
| ES-8 | `read_from_is_inclusive` | `crates/happenstance-testkit/src/suite.rs:905` |
| ES-8 | `read_backwards_from_with_limit` | `suite.rs:978` |
| ES-10/CF-13 | `nothing_below_an_observed_position_appears_later` | `suite.rs:6425` |
| ES-11/13 | `read_result_is_stable_under_concurrent_append` | `suite.rs:6104` |
| ES-12 | `query_items_share_one_snapshot` | `suite.rs:6208` |
| ES-21 | `batch_is_not_evaluated_against_its_own_condition` | `suite.rs:3456` |
| ES-22 | `dropped_append_future_leaves_no_partial_batch` | `suite.rs:3556` |
| ES-41/VT-7 | `contains_event_id_reports_membership` | `suite.rs:2866` |
| VT-4 | `append_stamps_identity_and_time` | `suite.rs:2263` |
| VT-17 | `tags_may_repeat_a_key` | `suite.rs:4511` |
| PS-1/2/4/11 | `commit_is_atomic_with_the_read_model` | `crates/happenstance-testkit/src/projection.rs:562` |
| PS-13/14 | `rebuild_is_chunk_size_invariant` | `projection.rs:1767` |
| PS-18 | `refused_reset_changes_nothing` | `projection.rs:1411` |
| PS-17 | `reset_is_scoped_to_one_projection` | `projection.rs:1298` |
| PS-12 | `batch_reads_reflect_pending_writes` | `projection.rs:1696` |
| WF-3 | `wire::query_all_is_unambiguous` | `crates/happenstance-core/tests/wire.rs:680` |
| WF-11 | `wire::payload_is_base64_in_json` | `wire.rs:1060` |
| CF-1 | `mutation_coverage::every_rule_has_a_mutant` | `crates/happenstance-testkit/tests/mutation_coverage.rs:3275` |
| CF-22/24 | `registry::no_orphan_rules` | `crates/happenstance-testkit/src/registry.rs:439` |
| CF-39 | `arming_a_mid_batch_fault_makes_the_append_fail` | `suite.rs:3109` |
| CF-41 | `the_promised_emitters_are_exactly_the_pinned_list` | `crates/happenstance-testkit/tests/emitter_surface.rs:268` |

**All 21 non-daggered names sampled resolve.** No false citation found.

### Daggered (†) names — "does not exist yet" (49 rows)

By family: **SY 32, ES 9, PS 7, CF 1** (`grep '†' trace | cut by prefix`).
Spot-check of daggered names (`grep -rln <name> crates examples xtask harness`):

| Name | Result |
|---|---|
| `ingest_never_rejects` (SY-1) | no hit anywhere |
| `the_sync_suite_never_decodes` (SY-35) | no hit anywhere |
| `failure_policy_is_per_projection` (PS-26) | only in `xtask/src/spec_trace.rs` (Scheduled: "the projection runner's own rules") |
| `one_poisoned_projection_does_not_stall_the_others` (PS-29) | only spec_trace.rs |
| `changed_query_starts_a_new_checkpoint` (PS-25) | only spec_trace.rs |
| `positions_are_not_reused_after_removal` (ES-38) | only spec_trace.rs (Scheduled: "the phase that lands history removal") |
| `wire_condition_with_after_is_refused` (ES-29/SY-6) | only spec_trace.rs (Scheduled) |
| `store_error_crosses_a_join_handle` (ES-6) | prose only in `crates/happenstance-cloudflare/src/send_shape.rs:14,95`, which argues it is **unwritable against today's port** (spec_trace.rs:1167-1173) |
| `provided_method_future_is_send_in_generic_code` (ES-3, ES-4) | **exists** as a unit test, `crates/happenstance-core/src/memory.rs:791` (dagger is a "not in suite.rs" artefact; declared Elsewhere at spec_trace.rs:1152-1161) |
| `error_bound_is_identical_on_both_flavours` (ES-5) | **exists**, `crates/happenstance-core/src/store.rs:588` |
| `begin_makes_no_round_trip` (PS-6) | **exists**, `crates/happenstance-neon/src/projection_store.rs:798`; companion `begin_resolves_at_its_first_poll_without_a_runtime` at `crates/happenstance-core/tests/projection_memory.rs:61` |

So the † reads "not in the suite file", not strictly "unwritten". Real absence ≈ 45 rows (49 minus ES-3, ES-4, ES-5, PS-6).

### FROZEN clauses with no existing rule (weak backing)

| Clause(s) | Rule status |
|---|---|
| ES-6 | † and argued unwritable on today's port |
| ES-29, ES-31, ES-38 | † Scheduled (wire condition; checkpoint lag; history removal) |
| PS-26, PS-28, PS-29 | † Scheduled "the projection runner's own rules" (phase 18 territory) |
| SY-1–6, 9, 11–13, 16, 19, 24–26, 33–35 (19 frozen SY) | † — `happenstance-sync-testkit` **does not exist** (`ls crates | grep sync` → only `happenstance-sync`); spec_trace exempts SY via `spec_trace.rs:1424-1460, 3549-3550` |
| SY-8, SY-15, SY-17 | cite the nonexistent sync-testkit / `peer_conformance!` as their rule (no †, prose cell) |
| ES-23, ES-37, PS-9, PS-31, PS-36, PS-37 | `*(none — see clause)*` (defended in authored §7.3–7.6 per `SPECIFICATION.md:9740-9746`; not reviewed individually here) |

Frozen-but-unchecked: 7 ES (3 of which exist elsewhere) + 3 PS + 22 SY = the SY family is frozen **on design only**; §1.3 says so explicitly: "`SY` clauses bind only the design" (`SPECIFICATION.md:198-199`).

## 5. E2E-CASES coverage

- **58 cases** (`grep -c '^### E2E-' spec/E2E-CASES.md`), groups A (01–14 event store), B (15–32 projections/runner), C (33–45 sync), D (46–51 lifecycle), E (52–58 edge/!Send) (`E2E-CASES.md:37-46`).
- **There is no implemented/pending marker per case.** Each case carries Level + Spans; status is only expressed via "blocked" in the Spans line.
- Levels: **49 contract, 9 integration** (`grep '^- \*\*Level:\*\*'`). Integration cases: E2E-28, 29, 31, 39, 42, 44, 45, 54, 56. Their home, "a workspace e2e test crate, `tests/e2e/`, not yet created" (`E2E-CASES.md:22`) — **confirmed absent** (`ls tests` → No such file). Scenario level has no cases.
- Spans lines marked "blocked": **11** (E2E-04, 05, 11, 20, 25, 33, 34, 38, 43, 48, 49 — `E2E-CASES.md:138,163,317,530,658,864,889,995,1132,1261,1292`); text at `:1569` says "Eleven cases above are marked blocked". The ledger says **twelve** (`ledgers.md` "The blocked cases", adds E2E-46), but E2E-46's Spans now reads "answered by the written refusal (ADR-0028)" (`E2E-CASES.md:1207`) — counts disagree (11 vs 12).
- **Many "blocked" markers are stale.** E2E-11 is "blocked on adding `ReadOptions::to`" (`:317`) but `ReadOptions::to` exists (`crates/happenstance-core/src/query.rs:327`) and `read_to_is_inclusive` exists (`suite.rs:1303`; VT-29/ES-16 FROZEN). E2E-33/34/43 blocked on `EventId`/store time — `EventId` (`crates/happenstance-core/src/identity.rs:97`) and `RecordedAt` (`identity.rs:171`) exist, VT-4/5/7/8 FROZEN. E2E-04/05 blocked on per-item boundaries — `condition_guards_carry_independent_boundaries` exists (`suite.rs:5535`), VT-30 provisional. E2E-20 blocked on apply seam "ADR pending" — ADR-0074 now accepted. The ledger's unblock column (phase 4/6) confirms these were meant to unblock long ago. Also E2E-01 still says `happenstance-postgres` "(does not exist)" (`E2E-CASES.md:56`). The "What cannot be written yet" section (`:1567+`) still cites `RUNBOOK.md:70` "with no ADR" for EventId. **E2E-CASES.md is a frozen-in-time catalogue, not a live status board.**
- Code references to case IDs: only 5 distinct IDs (E2E-05, 09, 13, 32, 42) appear in any `.rs` under crates/examples (`grep -rhoE 'E2E-[0-9]{2}'`). Traceability runs clause→case, not test→case.

### Derived coverage (via §7.2: case backed if ≥1 citing clause has a non-†, non-"none" rule cell)

- **51/58** cases are cited by at least one clause with a resolved rule (coarse: a case like E2E-39 counts as backed via ES-18's `append_is_atomic` though its sync half is untested).
- **7 cases have only †/none backing:** E2E-14 (SY-6), E2E-26 (PS-27, PS-28, PS-33), E2E-27 (PS-26, PS-27), E2E-31 (PS-31 none), E2E-44 (ES-40, SY-10/16/27/28/31/32), E2E-45 (SY-1/7/9/10/33/34), E2E-50 (PS-25).
- 0 cases are cited by no clause. 9 CF clauses claim `*all*`.

## 6. Families with weak test backing (ranked)

1. **SY (sync)** — 32 of 35 rule cells †; the sync testkit crate does not exist; 21 FROZEN clauses rest on design only; phase 13 "not started" and depends on 17 and 18 (`runbook/README.md:104`). Largest open block (17 freeze-by-13 + SY-32 at 14).
2. **PS runner-level clauses (PS-25–PS-30)** — runner rules all † "Scheduled: the projection runner's own rules"; phase 18 not started. Port-level PS-1–PS-24 well backed (17+ projection rules).
3. **ES retention/history (ES-38, ES-39, ES-40) and CF-27** — no instrument; phase 14 not started, ordered after 13.
4. **ES-11/ES-12 transport** — rules exist and *fail* (intermittently) on `happenstance-neon`, a published adapter (ADR-0061; marker `:3087`). Settling record owed by phase 17, unchecked.
5. **ES-35/CF-14/CF-17 durability fault end** — reopen half tested by four adapters; fault half unbuilt, renewed past 1.0.

VT, WF, CF and ES core read/append/condition semantics are strongly backed (sample resolved 21/21; mutation-coverage meta-tests per CF-1–CF-3).

## 7. Staleness / consistency findings

| Finding | Evidence |
|---|---|
| §7 prose percentages stale (ES 76%, PS 51% "no implementation") | `SPECIFICATION.md:9775,9782` vs §7.1 `:9791-9800` |
| "eighty-nine rules" vs 95 `pub async fn` in suite.rs | `SPECIFICATION.md:409,2512`; `grep -c` |
| E2E blocked markers stale (E2E-11, 33, 34, 43, 04/05, 20); E2E-01 says postgres does not exist | see §5 |
| Blocked-case count 11 (catalogue) vs 12 (ledger) | `E2E-CASES.md:1569`; `ledgers.md` |
| Ledger ES-11 row still says "Owed an ADR, staged at `.kb/_intake/2026-09-08-…`"; ADR-0061 exists and intake file is gone | `ls .kb/_intake` → README.md, decisions; `.kb/decisions/0061-es-11s-sufficiency-condition-assumed-a-queue.md` |
| Deferred table names phase 6 as owner for PS-18/27/30 | `ledgers.md` deferred table (declared historical by the ledger itself) |
| Counts all agree: §1.3, §7.1, §7.2 rows, ledger headings | §1 above |
