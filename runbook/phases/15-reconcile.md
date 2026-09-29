# Phase 15 — Reconcile the record

**Goal.** Every document and tool that says where happenstance has got to says
the same thing, and that thing is true.

**Why here.** The runbook, the handover, the backlog and three top-level documents
all stopped between 2026-09-07 and 09-11, and `0.2.0`, `0.3.0`, `0.3.1` and
`0.3.2` shipped after. The status table read `not started` over phase 12 while
`v0.2.0` was tagged, and `runbook_status_matches_the_registry` passed, because it
skipped every release without a milestone row. This repository has already held a
release on a stale row of that table once (phase 12's preamble, in the archive at
`RUNBOOK.md:5099-5112`). Every later phase is sequenced by reading these documents,
so they are repaired first.

**Decisions it settles.** None of its own. It executes D-2 and D-3 from the
[roadmap](../roadmap.md#decisions-taken).

**Work**

- [x] Split the runbook into `runbook/`: an index carrying the status table, this
      roadmap, the ledgers, a handover and a session log, and one file per open
      phase. `RUNBOOK.md` stays as the frozen monolith, line numbers intact —
      about 2,250 `RUNBOOK.md:N` citations resolve against it, one of them in the
      accepted atom for ADR-0033, which cannot be edited to repoint.
- [x] `runbook_status_matches_the_registry` repaired: a released version with no
      milestone row is now a failure rather than a skip, and a linked phase cell
      compares as its text. Both defects are pinned by tests that fail on the table
      as `f89e184` left it. The ledger check reads `runbook/ledgers.md`.
- [ ] `HANDOVER.md`, `REMEDIATION-HANDOVER.md` and `SESSION-DECISIONS-0.2.0.md`
      read as records, not status. `HANDOVER.md`'s title now says so; its body
      still describes `0.2.0` as unpublished, in the present tense, and is left
      verbatim because it is cited by line.
- [x] `README.md:139-141` and `:175-176` stop saying nothing is published at
      `0.3.2`. `CONTRIBUTING.md:23` and `:323-352` stop describing the registry
      semver baseline as future work, and `.github/workflows/ci.yml:1107-1116`'s
      comment stops telling a reader to delete an `if: false` that is gone. Check
      whether the rev-baseline job at `ci.yml:1092` still lists five crates of
      seven, and whether it should.
      Done: all seven are at `0.3.2` on crates.io, measured on 2026-09-28. It
      listed five and should not have. Its own comment said `happenstance-postgres`
      and `happenstance-neon` rejoined at `0.2.0`, so both are now in the list. The
      line numbers above are as `5afcca1` had them.
- [x] `CLAUDE.md` stops calling "does ingest re-check append conditions" the
      central unanswered question; SY-1 – SY-7 settled it. Same correction in
      `crates/happenstance-sync/src/lib.rs:116-119`.
      Done: SY-1 – SY-6 are frozen and SY-7 is provisional, and `CLAUDE.md` now
      says so. The same claim was also in `peer.rs`'s `EventGroup` doc and in two
      sentences of the specification's SY-1 and SY-6 prose that quoted it; all of
      them are corrected. The neighbouring bullets in the crate's ledger, on
      identity (VT-5) and ordering (SY-19), were just as stale and are corrected
      as well. Every edit to a cited file kept its line count.
- [ ] `happenstance-sqlite`'s crate root and README say *host only*. It does not
      build for `wasm32-unknown-unknown` (`rusqlite` fails at `libsqlite3_sys`, and
      reads hop through `spawn_blocking`), nothing in the gate checks it, and
      nothing says so. See `references/seeds/sqlite-on-wasm.md`.
- [x] The backlog, by D-2: `.bklg/` and `.redkiln/` are frozen records and
      redkiln is retired until `redkiln-rs`, after 1.0. `CLAUDE.md`'s *Where the
      work lives* says so, and this runbook is the only tracker. No item in
      `.bklg/` is advanced or closed — there is no CLI to do it with, and a hand
      edit to item frontmatter would forge a transition nobody made.
- [ ] Decide what replaces redkiln's `.kb` checks until `redkiln-rs`, now that
      nothing validates atom frontmatter and accepted-atom immutability was only
      ever enforced pre-commit. The candidate is an `xtask` lint that diffs every
      `status: accepted` atom body against the merge base — the instrument
      `CLAUDE.md` already named as missing. *None* is a legitimate answer if it is
      written down.
- [ ] A KB intake wave that closes what is already answered —
      `query-union-rule-is-owed-and-unowned`,
      `testkit-projection-module-unstable-projection-exemption-scope`,
      `stale-0-0-0-name-reservations`, the gap-read half of
      `es-38-and-gap-read-rules-are-unowned`, and (verify first)
      `global-versus-per-boundary-visibility-invariant` — indexes the orphan
      `cf-17-cf-14-maturity-markers-and-the-reopen-must`, and removes
      `.kb/_intake/remediation-2026-09-04-briefs/README.md`, which describes forty
      briefs ingested on 2026-09-07. By hand now, since redkiln is retired; the
      three Weigh-In decisions staged at `.kb/_intake/decisions/` go in the same
      pass.
- [ ] The open questions whose own deadline has passed — "at `0.2.0`", "at phase
      12", "at the `ProjectionStore` freeze" — each get a new owner in phase 16 or
      17, recorded in that phase's file.
- [x] The branches, by D-3: six squash-merged `lane/*` branches deleted locally
      and on origin, and `wip/hs-p0012-benchmark-harness` locally, on 2026-09-28.
      Tips at deletion: `998a545`, `f60d7e8`, `7d56bce`, `905e6ee`, `380c8ae`,
      `3b1ffb7`, `d8fd819`. `lane/0.2.0-closeout` and
      `origin/worktree-kb-intake-2026-09-11` are also merged and were outside the
      decision.
- [ ] `CHANGELOG.md`'s `[Unreleased]` holds two merged entries — SQLite's busy
      timeout at fifteen seconds (ADR-0065) and `FaultyStore::contend_next`.
      Decide whether they ship as `0.3.3` or ride `0.4.0`; the first is a
      behaviour change to a published adapter, which argues for not waiting.

**Proof artefact.** The status lint failing on the table as `f89e184` left it and
passing on this one — `a_released_version_with_no_row_is_refused` and
`a_milestone_row_holds_its_phase_to_done` in `xtask/src/lints.rs`.

**Exit criteria**

- [ ] `cargo xtask ci` green, with the status and ledger checks reading `runbook/`.
- [ ] Every released version has a milestone row, and every phase it waits on
      reads `done`.
- [ ] No top-level document states, in the present tense, a publication state
      the registry contradicts.
- [ ] What checks `.kb/` until `redkiln-rs` is decided, even if the answer is
      nothing.
- [x] D-1, D-2 and D-3 decided, reflected, and — for D-3 — executed.
- [ ] [`handover.md`](../handover.md) names a next action the status table agrees
      with.

**Estimate.** 2–3 days.

**Session log**

- 2026-09-28 — Runbook split; status lint repaired; roadmap written; seed for
  SQLite on `wasm32` added. D-1 (sync inside 1.0), D-2 and D-3 decided; redkiln
  retired and `CLAUDE.md` rewritten for manual tracking. D-3 executed.
  `cargo xtask ci` green. Committed on `lane/runbook-split`.
- 2026-09-28 — PR #14 merged as squash `65253fc`; citations of `3916f29` moved to
  `f89e184`, where the monolith is byte-identical.
- 2026-09-28 — PR #15 merged by rebase as `db99a72` and `5afcca1`. The handover
  and log are brought current. `README.md`, `CONTRIBUTING.md` and `ci.yml`'s
  semver comments no longer call the registry empty, and the rev baseline now
  covers all seven crates. The two `standards/rust` citations into `ci.yml` were
  repointed by anchor. So were the constitution's `CONTRIBUTING.md:196`, and a
  seed's `CONTRIBUTING.md:356-365`, which had been stale before this change.
  Verified: `cargo run -p xtask -- lints`, `cargo xtask lint-constitution`,
  `cargo xtask spec-trace` and `cargo xtask affected --base main` all green.
- 2026-09-28 — PR #16 squash-merged as `c0df525`. The ingest re-check question is
  recorded as settled in `CLAUDE.md`, `happenstance-sync`'s crate root and
  `peer.rs`, and the spec prose quoting them. SY-1's and SY-6's MUSTs and markers
  are untouched. Verified: see `log.md`.
