# Handover

**Rewritten whole at the end of every session, never appended to.** History goes
in [`log.md`](log.md) and in each phase's session log; this file is only *now*. If
its `As of` line does not name `HEAD` or its parent, it is stale — trust
`git log` and the status table over it, and say so in the next session log.

The template is the headings below, in this order. Keep each section short enough
to read in one screen; link out for anything longer.

---

## As of

`0d59926` on `main`, plus `lane/phase-15-kb-lint` open as a PR.
2026-09-28.

## Where things are

Seven crates are published at `0.3.2`. `EventStore` has been frozen since `0.2.0`,
and `ProjectionStore` since `0.3.0`. Phases 0–12 are done. What remains is
sequenced in [`roadmap.md`](roadmap.md):
- reconcile the record (15);
- define 1.0 (16);
- a breaking window, released as `0.4.0` (17);
- then sync (13), retention (14) and the typed runner (18);
- and `1.0.0` (21) last.

**Sync is inside 1.0**, so phases 13 and 14 are on the critical path. SQLite on
`wasm32` (19) and the documentation work (20) run alongside.

**Redkiln is retired here.** This runbook is the only tracker until `redkiln-rs`,
after 1.0. `.bklg/` and `.redkiln/` are frozen records; `CLAUDE.md`'s *Where the
work lives* has the rules.

Phase 15 is being finished unattended, one PR per work item, self-merged on green
(`wi-ab0a5a`).

## In flight

Branch `lane/phase-15-kb-lint`, open as a PR: `cargo xtask lint-kb` (`wi-38373d`),
a gate step that fails when an accepted decision's body changes against the merge
base. CI's `gate` job checks out full history for it.

Committed and waiting behind it: `lane/phase-15-kb-intake`, the KB intake wave,
written by hand.

`log.md` now names PR #18's squash, `0d59926`.

## Next action

Merge this PR. Then rebase and open `lane/phase-15-kb-intake`; then phase 15's
overdue open questions, each given an owner in phase 16 or 17.

## Waiting on the owner

- Review the Weigh-In defaults taken during the unattended session, with
  `/weigh:in digest`.
- **`wi-6c9f77` (blocked):** `CLAUDE.md`'s binding constraint 5 still says, in
  the present tense, that `0.2.0` "has not happened yet" and that nothing is
  yanked. Correcting the tense leaves the MSRV constraint unchanged, but the
  paragraph is a binding constraint, so the edit waits for the owner. Phase 15's
  third exit criterion stays open until it lands.
- Optional: `lane/0.2.0-closeout`, `origin/worktree-kb-intake-2026-09-11`,
  `origin/lane/runbook-split` and `origin/lane/runbook-citations-after-squash` are
  all merged. The first three were outside D-3; the last merged after it.
- `runbook/phase-15-afk-prompt.md` and `assets/brand/happenstance-mark.png` are
  untracked, and committing them is the owner's call.

## Do not re-open

Settled, with the record that settled it. Re-opening one needs a new decision
record, and the owner.

- `happenstance-sync` is inside 1.0 — D-1, `wi-40b321`. The recommendation was
  the opposite and was overridden.
- Redkiln is retired until `redkiln-rs`; `.bklg/` is frozen and not advanced —
  D-2, `wi-016abe`, and the owner's instruction of 2026-09-28.
- Ingest is unconditional, with compensation — SY-1 – SY-7.
- The projection port is frozen — ADR-0063.
- The MSRV is 1.97.1 and a promise since `0.2.0` — ADR-0029, ADR-0037.
- `happenstance-macros` is out of scope — ADR-0033.
- `happenstance-ladybug` is finished and does not publish while `lbug` fails on
  docs.rs — phase 11, `crates/happenstance-ladybug/src/lib.rs`.
- The `0.2.0` release set is seven crates — `e597c34`.
- `[Unreleased]` ships in `0.4.0`; there is no `0.3.3` — `wi-052920`.
- An accepted decision's body is checked by `cargo xtask lint-kb` —
  `wi-38373d`.

## Traps

- **The gate's `tests` step is the memory hog.** Verify in slices with
  `cargo run -p xtask -- <step>`; do not pipe the whole gate into `tail`.
- **`cargo hack --no-dev-deps` rewrites manifests.** A killed gate run can strip
  dev-dependencies from every crate; check `git status` before believing a result.
- **SQLite's concurrency rule flakes on Windows**, worse with more CPU. Do not
  isolate the test to reproduce it; it is owned by two open questions already.
- **The redkiln plugin is disabled for this repository only**, in the gitignored
  `.claude/settings.local.json`; it stays enabled in the user settings for other
  repositories. A fresh clone does not carry that file, so there its hooks can
  still write to `.redkiln/telemetry/` — do not commit new telemetry as work.
- **The merge method varies.** PR #14 was squash-merged and PR #15 rebase-merged,
  and both rewrite commit ids. A citation of a PR-branch commit dies with the
  branch, so cite the commit on `main` after the merge.
- **`cargo xtask lint-kb` needs history.** It compares `.kb/decisions/` against
  the merge base and fails on a shallow clone; CI's `gate` job checks out at
  `fetch-depth: 0` for it. Only a repointed `path:line` citation passes as a
  repair; any other edit to an accepted body is a supersession.
- **A staged file blocks `git rebase`.** The stray `.redkiln/telemetry/` change
  stays staged, so rebase a branch in a throwaway `git worktree` instead.
