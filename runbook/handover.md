# Handover

**Rewritten whole at the end of every session, never appended to.** History goes
in [`log.md`](log.md) and in each phase's session log; this file is only *now*. If
its `As of` line does not name `HEAD` or its parent, it is stale — trust
`git log` and the status table over it, and say so in the next session log.

The template is the headings below, in this order. Keep each section short enough
to read in one screen; link out for anything longer.

---

## As of

`claude/p13-exit`, cut from `phase-13` at `a4a25ab4`. 2026-11-09.

**GitHub Issues is the tracker** ([ADR-0088](../.kb/decisions/0088-github-issues-is-the-tracker.md)).
Per-item progress is in the issues. This file, the status table and the phase files
are the plan of record. Unattended sessions follow [`afk.md`](afk.md).

## Where things are

**Phase 13 is done** on `phase-13`, pending only `phase-13` → `main` and the `0.6.0` tag
(phase 18's precedent: its row read `done` on its integration branch before #403 merged).
Every work item and exit criterion in [the phase file](phases/13-sync.md) is ticked on
evidence in the tree. The runner (#244), write groups (#420, ADR-0105), `SendPeer` (#421),
the Durable Object peer over HTTP (#130), `NeonSyncPeer` and `harness/sync-round-trip`
(#131) and the freeze rules (#133, #134, #136, #137, #139, #140, #141, #143) are merged.
CF-25's portfolio check is built (#125), ADR-0003 is no longer provisional (ADR-0106, #126),
and [ADR-0107](../.kb/decisions/0107-phase-13s-clauses-are-frozen.md) froze nineteen
clauses (#124, #128). `cargo xtask spec-trace` reads 187 `[FROZEN]`, 13 `[PROVISIONAL]`,
5 `[DEFERRED]` and 7 `[NON-NORMATIVE]`; in §5, 35 of 39 `SY` clauses are frozen, with
SY-7 and SY-10 `freeze-by-14` on the adjudication hook (#450).

**`0.5.1` is released** (2026-11-08; all nine crates on crates.io and docs.rs). Phases
0–13, 15–17b and 18 are done. What remains is sequenced in [`roadmap.md`](roadmap.md):
retention (14), then `1.0.0` (21).

**`0.6.0` is tagged on `main`, at `phase-13` → `main`'s merge commit**, as `0.5.0` was at
#403's: its release PR merges into `phase-13`, and `release.yml` publishes only a tag on
`main`. Cloud sessions cannot push tags; the owner pushes them.

## In flight

The phase-13 exit reconciliation, `claude/p13-exit` into `phase-13`, docs only. No release
PR is open yet.

## Next action

1. The owner merges this branch into `phase-13`.
2. The `0.6.0` release PR into `phase-13`: `[Unreleased]` becomes `[0.6.0]`, all nine
   crates and the pins move, a `0.6.0` trace (ADR-0104's wire v3 and the migrations are the
   breaking rows), an upgrading guide if the `0.5` precedent holds, and the `0.6.0` status
   row. Then `phase-13` → `main`, `cargo semver-checks check-release --workspace
   --baseline-version 0.5.1 --release-type minor` on its merge commit, and the tag.
3. On the merge, close #64, #108, #117, #130, #131 and #244, whose PRs merged into
   `phase-13` and so closed nothing.
4. Phase 14 (#65). It owns SY-7 and SY-10 (#450 first, then #441), SY-32 and VT-36.

```
repo:wet-ink-corporation/happenstance is:issue is:open label:status:ready-for-agent no:assignee -label:door:one-way
```

## Waiting on the owner

- This branch, the `0.6.0` release PR, `phase-13` → `main`, and the `v0.6.0` tag.
- `CLAUDE.md` is protected: the proposed replacement text for its stale sync lines is at
  `/mnt/project-files/notes/0.6.0-claude-md-proposal.md`. The earlier items on this list
  (the #412 doc build, `unstable-projection` at `CLAUDE.md:440-443` and in
  `bug_report.yml`) are already corrected in the tree.
- SY-21's `convergent_projection_cannot_observe_local_position` was not written at phase
  13: its Rule line says "phase 13 adds" it and `spec_trace.rs` still declares it
  `Scheduled` for phase 13. Write it, or reschedule it with a follow-up issue.
- Ten `[FROZEN]` `SY` clauses name a † rule that exists nowhere in the tree (SY-3, SY-6,
  SY-9, SY-13, SY-16, SY-19, SY-26, SY-33, SY-34, SY-35), and `spec-trace`'s check 4 reads
  no sync suite, so nothing fails on it. A follow-up decides whether check 4 reads
  `happenstance-sync-testkit` now that it exists.
- §1.6 now reads `SyncPeer` as **Frozen at phase 13**, which is what `spec-trace`'s CF-25
  walk already treated it as. Confirm or reword.
- Calls taken *as recommended* while the owner was asked, still unconfirmed: #101 O1–O3,
  #103 O1(a), #118's schema call, and #123 O1(B)/O2.
- Follow-ups filed overnight and not yet scheduled: #441, #448, #450–#455.
- Defaults in force: `trait-variant` keeps its caret (ADR-0092); new CI jobs are not
  required checks.

## Do not re-open

Settled, with the record that settled it. Re-opening one needs a new decision
record, and the owner.

- Phase 17 is split at the release; 17b holds the additive half — ADR-0072.
- `happenstance-ladybug` is retired: excluded from the workspace, kept as a frozen
  record — ADR-0078, `wi-630032`.
- What 1.0 promises: nine crates, ladybug outside, versioning, `workerd` before
  1.0, soak, support window and licence — ADR-0066 and its seven `wi-*` atoms.
- The MSRV holds at 1.97.1, and after 1.0 a rise is bounded — ADR-0067.
- ES-10 stays global — ADR-0071.
- `happenstance-sync` is inside 1.0 — D-1, `wi-40b321`.
- Redkiln is retired until `redkiln-rs`; `.bklg/` is frozen and not advanced —
  D-2, `wi-016abe`, carried forward by ADR-0088.
- GitHub Issues is the tracker; exit criteria stay in the phase files — ADR-0088, D-5.
- Unattended sessions self-merge on green unless `door:one-way` — `wi-1fde8c`.
- Ingest is unconditional, with compensation — SY-1 – SY-7.
- Phase 13's nineteen clauses are frozen; SY-7 and SY-10 wait on the adjudication hook —
  ADR-0107, ADR-0105, ADR-0106.
- The projection port is frozen — ADR-0063.
- Phase 18's eight clauses are frozen, and the fan-out runner catches nothing —
  ADR-0102, ADR-0101.
- `happenstance-macros` is out of scope — ADR-0033.
- `[Unreleased]` ships in `0.4.0`; there is no `0.3.3` — `wi-052920`.
- An accepted decision's body is checked by `cargo xtask lint-kb` —
  `wi-38373d`.

## Traps

- **Memory is tight on this machine.** Run cargo with `CARGO_BUILD_JOBS=2`. Background gate runs get stopped when the session idles, so run the temper gate in the foreground. Never stop a gate mid-run: `cargo hack --no-dev-deps` strips dev-dependencies, so restore with `git checkout -- '*Cargo.toml' Cargo.lock`.
- **`happenstance-cloudflare`'s wasm32 tests need Node 24.** The shim opens `node:sqlite` with `limits`, which Node 22 lacks, and refuses to open without it (ADR-0079).
- **The `workerd` harness runs whatever is in `harness/workerd/build/`.** Re-run
  `npm run build` (that is, `worker-build --release`) after any Rust change, or vitest
  tests the old wasm. `worker-build` is `cargo install worker-build --version 0.8.5`.
- **Spec and source edits shift `path:N` citations tree-wide.**
  - Snapshot the file first, then map old line numbers to new with difflib and repoint once. The scratchpad scripts `specmap2.py` and its siblings did this.
  - Never repoint inside historical quotes: `SESSION-DECISIONS-*`, `RUNBOOK.md`, `references/`, `.kb/_governance`, experiment results, and released CHANGELOG sections.
  - An accepted atom's frontmatter `summary` must not change, even for a citation.
- **Rebasing over a lane that repointed the same citations:** take `main`'s version of each file whose diff is citation-only, recompute the shift from `main`'s copies, and apply it once. Hand-fix ranges whose endpoints fell in deleted text.
- **Git Bash mangles `rev:path`.** Set `MSYS_NO_PATHCONV=1` for `git show REV:path`.
- **A hook refuses shell edits whose command text mentions a `.rs` path,** even in markdown. Use the Edit tool.
- **Live Neon is a required check again** (#75, 2026-10-10, ADR-0087 D8). A Neon outage or an expired `NEON_*` secret now blocks merges; fix the endpoint or secret rather than dropping the check, which would need the owner.
- **The CI base-commit semver step is advisory** while `0.4.0` is unpublished (a crates.io probe). The release trace comes from `cargo semver-checks check-release --workspace --baseline-version 0.3.2 --release-type minor`.

- **At `0.4.0` the semver tool skips every lint.** The trace table must come from `cargo semver-checks check-release --workspace --baseline-version 0.3.2 --release-type minor`, plus hand rows for core's feature removal, the hidden emitter renames, and `happenstance-cloudflare`'s `planned_statement_count` values (ADR-0079).

- **17b is not a breaking window.** An item whose answer turns out to break a
  published crate goes back to phase 17 (ADR-0072's rule), not into a `0.5.0`.
- **Bump to `0.4.0` at the first breaking PR** (lane L4), or the semver job goes
  red against `0.3.2` on every intermediate PR. Leave the CHANGELOG heading at
  `[Unreleased]` until the release PR.
- **Never touch `Fixture::arm_mid_batch_fault` or its *k*.** CF-39 is frozen
  (ADR-0066), and `Busy`, CF-23 and the `workerd` fixture all edit the testkit.
- **The gate's `tests` step is the memory hog.** Verify in slices with
  `cargo run -p xtask -- <step>`; do not pipe the whole gate into `tail`.
- **`cargo hack --no-dev-deps` rewrites manifests.** A killed gate run can strip
  dev-dependencies from every crate; check `git status` before believing a result.
- **SQLite's concurrency rule flakes on Windows**, worse with more CPU. Do not
  isolate the test to reproduce it; it is owned by two open questions already.
- **The redkiln plugin is disabled for this repository only**, in the gitignored
  `.claude/settings.local.json`. A fresh clone does not carry that file, so its
  hooks can still write to `.redkiln/telemetry/` — do not commit new telemetry.
- **The merge method varies**, and both squash and rebase rewrite commit ids. Cite
  the commit on `main` after the merge.
- **`cargo xtask lint-kb` needs history.** It compares `.kb/decisions/` against
  the merge base and fails on a shallow clone. Only a repointed `path:line`
  citation passes as a repair on an accepted body.
- **A staged file blocks `git rebase`.** The stray `.redkiln/telemetry/` change
  stays staged, so rebase a branch in a throwaway `git worktree` instead.
- **Thirteen of fourteen proposed freezes were refuted.** A clause's marker
  moves on evidence in the tree, not on an argument. ADR-0066 §2 records each.
