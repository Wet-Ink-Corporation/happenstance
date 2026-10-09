# Backlog gap report: runbook to GitHub issues

Generated from `07215eb` on 2026-10-09. This is a draft. Nothing was written to GitHub and the repo was not edited.

## Headline

GitHub has **0 issues**. The manifest proposes **154**.

| by type | n | | by milestone | n | | by status | n | | by source | n |
|---|---|---|---|---|---|---|---|---|---|---|
| epic | 10 | | 0.4.0 | 9 | | blocked | 95 | | runbook | 135 |
| feature | 96 | | 1.0.0-rc.1 | 113 | | needs-owner | 26 | | audit | 14 |
| task | 44 | | 1.0.0 | 12 | | ready-for-agent | 24 | | open-question | 4 |
| bug | 4 | | none | 20 | | ready-for-human | 7 | | review | 1 |
| | | | | | | needs-triage | 2 | | | |

There are 58 `door:one-way` items, 16 `semver:breaking` and 16 `semver:additive`. The 9 drafts held 158 items. After 4 merges, 1 split (+2 tasks) and 2 folds, 154 remain.

## Epics in run order

| key | title | features/bugs | tasks | milestone | blocked_by |
|---|---|---|---|---|---|
| epic.17 | Phase 17: the breaking window, released as 0.4.0 | 7 + 1 bug | 2 | 0.4.0 | none |
| epic.17b | Phase 17b: the additive half of phase 17 | 10 + 1 bug | 0 | 1.0.0-rc.1 | 17 |
| epic.18 | Phase 18: the typed runner leaves its gate | 14 | 3 | 1.0.0-rc.1 | 17 |
| epic.13 | Phase 13: happenstance-sync and its testkit | 19 | 10 | 1.0.0-rc.1 | 17, 18 |
| epic.14 | Phase 14: retention, deletion and completeness | 7 | 0 | 1.0.0-rc.1 | 13, 17 |
| epic.20 | Phase 20: documentation that teaches | 7 + 1 bug | 29 | 1.0.0-rc.1 | none |
| epic.22 | Phase 22: the documentation site | 6 | 0 | 1.0.0-rc.1 | none |
| epic.21 | Phase 21: publish 1.0.0 | 11 | 0 | 1.0.0 | 13, 14, 17, 17b, 18, 20 |
| epic.19a | Phase 19a: SQLite on wasm32, skeleton | 6 | 0 | none | none |
| epic.19b | Phase 19b: SQLite on wasm32, the adapter | 4 | 0 | none | 17, 19a |

Six items have no parent: `owner.neon-required-check`, `owner.weigh-in-digest`, `owner.merged-lane-branches`, `owner.happenstance-mark-png`, `audit.R8` and `doc.p13-ledger-kv-cite`.

**0.4.0 release-blocking (9):**
- epic.17
- f01 and its tasks t01 and t02 (release PR, then tag/publish)
- f02 (ADR-0084 arity), blocked on owner.adr-0084-before-0-4-0
- f03 bug (doc build)
- audit.IMPL-5
- audit.docs-dx-F4 (migration guide)

## Doc inconsistencies found (all are issues in the manifest)

- **epic.17.f03 (bug):** the default-features doc build of `happenstance` fails at `crates/happenstance/src/lib.rs:111`. `[Projection::apply]` does not resolve when `unstable-projection` is off. The drafter verified this with `cargo doc -D warnings`.
- **doc.ps-39-unscheduled-in-17b:** `runbook/ledgers.md:312` marks PS-39 freeze-by-17b, but `runbook/phases/17b-after-the-window.md` never mentions it. Confirmed by grep.
- **doc.p13-ledger-kv-cite:** `runbook/ledgers.md:316` cites `phases/13-sync.md:118-127`, which is the MemorySyncPeer and identity text. The KV-capped peer is at `:139`. Confirmed.
- **doc.phase-20-progress-unbacked:** `runbook/README.md:108` says "in progress", but the session log at `phases/20-docs-that-teach.md:54` is empty.
- **doc.stale-records-2026-10-06 (t01 to t05):** five stale records.
  - CLAUDE.md: clause count, facade size, 101-of-101.
  - SPECIFICATION.md: rule counts and frozen percentage.
  - Crate READMEs: seven published, Ladybug retired.
  - `runbook/handover.md:68`: says `phase-15-afk-prompt.md` is untracked, but it has been tracked since 1f92d08 (#34). It also names a PNG that does not exist.
  - `experiments/es-11-fence/README.md:12`: says "proposed", but ADR-0087 is accepted.
- **epic.21.f09** (now holds two merged findings): the phase-21 prose at `:10-11` omits 17b, which the status row `README.md:110` includes. Phase 22 appears in neither.
- **audit.R8:** ADR-0078:158 and `xtask/src/reserve.rs:110-112` claim a `happenstance-ladybug` 0.0.0 reservation that crates.io does not have (404). The ADR body is immutable, so the fix is in `reserve.rs` or a superseding record.
- **New, found while reconciling:** HS-P0025's `merge-forward-baseline` story merges `initiative/from-contract-to-published-library`. That branch is not on origin (48 heads listed). The story was folded into epic.20.f05.t02 and marked UNSURE.

## Already closed: drafter drops with evidence

These items were dropped as done or fixed at HEAD:
- **Phase 17:**
  - Every ticked Work box (`17-breaking-window.md:31-238`).
  - ADR-0084 accepted and merged (#44, #49).
  - Racing-mutant flake fixed (#58, `:627-638`).
  - Workerd "Worker not found" and DO-reset 500 already classified (`harness/workerd/platform-miss.mjs:21-27`).
  - The Neon fence landed (#57); only the ruleset edit remains, as `owner.neon-required-check`.
  - `phase-15-afk-prompt.md` "untracked" is not an owner item (it is tracked).
- **Phase 17b items moved by ADR-0072:** the QueryItem constructor, minimal-versions, and VT-14/VT-30/ES-7 (`17-breaking-window.md:130,187,194`). These live in epic.17b.
- **Phase 18:** core's `unstable-projection` is already removed (`crates/happenstance/Cargo.toml:140`). Only happenstance's own feature remains (epic.18.f14).
- **Phase 13:**
  - Placeholder `EventId`/`StoreId`/`RecordedAt` deleted (ticked, ADR-0073, `13-sync.md:124-128`).
  - "Phase 17 decides mint-per-open" is settled by ADR-0086.
- **Phase 14:**
  - ADR-0028 written (`14-retention.md:36-40`).
  - Redaction E2E-49 answered (`:75-76`).
  - Struck suffix-store exit criterion (`:110-116`).
  - Phase-15 dependency done (`README.md:99`).
- **Phase 20:** three HS-P0023 stories landed even though their story.md files still say "ready":
  - pointer policy (`xtask/src/pointers.rs`)
  - adapter reasoning account (`docs/adapter-reading-order.md:1-20`)
  - E0034 store-error site (`crates/happenstance-core/src/store.rs:37`)
- **Phase 22:** four ticked boxes: the Zola shell, `cargo xtask site`, doc-scrape-examples, and `pages.yml` (`22-docs-site.md:40,44,53,65`).
- **Audit:**
  - F6 fixed by ADR-0079 / #35 (`happenstance-cloudflare/src/event_store.rs:405,428`).
  - T-07: ES-23 met (all four adapters carry `# Cancellation`), PgCursor has `Drop` (`read_stream.rs:241`), Ladybug retired.
  - The handover "PR #34 open" and "write ADR-0083" lines are fixed.
  - The `log.md` date is fresh.
  - V6's count is stale (156 FROZEN of 205); the residue is audit.A7.

These were dropped as already covered by a phase Work box, so they are not duplicates:
- **Audit:** F2 (`22-docs-site.md:75`), R8 sync half (`13-sync.md:60`).
- **Open questions:** 10 atoms:
  - dcb-reference-wire-format, cf-25/26, es-38 and es-7/vt-9: phases 13 and 14.
  - es-23-adapter-half: phase 20.
  - reset-refusal: phase 18.
  - trait-variant-caret, tuple-boundary, then-empty and vt-30: phase 17b.

## Open questions excluded (26 accepted atoms)

None of these has a phase forcing event before 1.0. Most are "no phase owner": cf-38, cf-5, es-18, gate-step-first-check, sole-evidence-pin, what-the-exact-anchor-rule, adr-status-vocabulary, dagger-convention and docs-citation-anchor.

The rest are excluded for a stated reason:
- **Owned after 1.0:** cf-33/34, model-only-kind, read-page-budget and read-to-backwards-limit.
- **Renew-past-1.0 already recorded:** cf-17/cf-14.
- **Forced only by a future event:** event-type-positional-mapping, happenstance-facade-vs-ADR-0006, references-adr-in-place-correction and rustdoc-citations-form.
- **Decided or not blocking:** es-11-ceiling (ADR-0061), es-6 (prose landed; its open sub-questions are not blocking), off-poll-visibility (phase 10 passed) and experiment-raw-output.
- **Waits on the 17b closures:** scope-coverage-helper.
- **Outside the 1.0 path:** trademark-search.
- **Folded into `oq.read-fault-rule-has-no-clause` as one joint ADR:** disjoint-boundaries and model-family.

Tally: 4 kept, 10 dropped and 26 excluded, 40 in all.

## Validation fixes applied

**Merges.** Each kept the runbook key and unioned the references.
- `oq.sync-message-set-and-format-version` merged into **epic.13.f09**, which was retitled "Sync message set, envelope types and FORMAT_VERSION semantics".
- `doc.handover-afk-prompt-tracked` merged into **doc.stale-records-2026-10-06.t04**.
- `doc.phase21-prose-omits-17b` and `doc.phase22-absent-from-phase21-deps` merged into **epic.21.f09**, whose scope was those two findings.

**Status.** Every item with open `blocked_by` now carries `status:blocked`. The owner call stays in its body and in `door:one-way`.
- **To blocked:**
  - From needs-owner: epic.14.f07, epic.17.f01.t02, epic.18.f13, epic.18.f14.
  - From ready-for-agent: epic.19a.f03, f04, f05.
  - From needs-triage: epic.19b.f04.
  - Epic: epic.17b, which is blocked by epic.17.
- **epic.18.f10 (Chunk):** it was `ready-for-agent` while also carrying `door:one-way` and `semver:breaking`. It is now `blocked_by epic.17` and `status:blocked`, like its phase-18 siblings.

**Labels and cites.**
- epic.17.f03: removed `semver:additive`. A rustdoc link fix changes no API.
- epic.20.f01: source_cite range `:23-34` changed to `:23`.

**Granularity.**
- epic.13.f11 was split into t01 (Durable Object peer and SQLite round trip) and t02 (Postgres/Neon peer and SQLite round trip). The drafter had said it was 2+ PRs.
- epic.20.f05.t01 (record a sha) was folded into t02.
- epic.20.f05.t11 (run the gate last) was folded into f05's acceptance criteria.

**Flagged, not changed:**
- epic.19b.f01 covers both stores and may need tasks once scoped.
- epic.20.f04 and f05 still keep one task per backlog story (10 and 9). Several are small record-only PRs; review whether to collapse them.

**Final check.** All keys are unique and every parent and blocker resolves. There are no cycles. Each item has exactly one type, status and source label, and every label is in the allowed set. Milestones are valid. Each body is 220 words or fewer and has all six headings.

## Spot-check: 15 random source_cites (seed 7)

All 15 pass. I opened every one at HEAD.
- **Runbook phase-file cites (match their Work box):**
  - 18-typed-runner :23, :27
  - 17b :24, :42, :73
  - 19 :29
  - 21 :47
  - 13 :139
- **Ledger cites:**
  - ledgers :312 shows PS-39 as freeze-by-17b.
  - ledgers :316 shows the stale `118-127` cite, which confirms the bug.
- **CHANGELOG :526** is the "0.4.0 trace (draft)" heading.
- **The four `.bklg` story cites are weak passes.** They land on the frontmatter (project.md:1) or on the "## One-line PR slice" heading, one line above the slice text. That is acceptable but imprecise.

## UNSURE: needs owner eyes

- **epic.17.f03:** the doc-build failure is release-relevant for 0.4.0. Not verified against docs.rs.
- **owner.weigh-in-digest:** no digest file was found; the wi-* intake atoms remain in `.kb/_intake/decisions/`.
- **owner.happenstance-mark-png:** the file is absent from the checkout and from all git history.
- **owner.merged-lane-branches:** about 38 `lane/*` branches are on origin. Which are merged was not verified. Keep `lane/p17-vacuity-control` and `lane/p17-es11-fence`.
- **audit.IMPL-7:** whether a weaker model rule is feasible on Postgres and Neon. This is needs-triage.
- **audit.A7:** whether phase 14 picks up ES-29 and ES-31. ES-6 is argued unwritable.
- **audit.PG-4:** asks the owner to reconsider wi-052920's "no 0.3.3".
- **oq.cf-36:** may already be discharged for the nine SY entries by `13-sync.md:114`.
- **Phase 13:**
  - epic.13.f01: the crates.io names were not re-checked.
  - epic.13.f02: `ready-for-human` but `door:one-way`. ADR-0086 pre-authorises it; switch to needs-owner if a sign-off is wanted.
  - epic.13.f06: ES-41 may be semver:breaking.
  - epic.13.f15.t08: `MetadataLen` is not in crates/, so this waits on epic.17b.f06.
  - epic.13.f18: recount the DEFERRED SY clauses. The grep was truncated.
- **epic.17b.f07:** re-read `boundary.rs:72-74` before the doc correction. At HEAD those lines are the `type Event` doc comment.
- **epic.18.f09:** the phase file does not say which adapter implements the refusable reset.
- **Phase 20:**
  - epic.20.f02: per-story confirmation of HS-P0020 to 22.
  - epic.20.f05: its specs call the retired redkiln `kb-ingest` and `record-links`.
  - epic.20.f05.t02: the initiative branch is absent.
- **Phase 21:**
  - epic.21.f01: which phases still own freeze-by rows.
  - epic.21.f03: the semver baseline against 0.4.x.
- **Phase 19:**
  - epic.19a.f01: where tracking lives.
  - epic.19b.f04: phase-13 timing.
- **epic.22.f03:** whether the `rebuild-a-read-model` how-to is enough to close audit docs-dx F2.
