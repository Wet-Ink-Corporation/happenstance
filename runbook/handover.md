# Handover

**Rewritten whole at the end of every session, never appended to.** History goes
in [`log.md`](log.md) and in each phase's session log; this file is only *now*. If
its `As of` line does not name `HEAD` or its parent, it is stale — trust
`git log` and the status table over it, and say so in the next session log.

The template is the headings below, in this order. Keep each section short enough
to read in one screen; link out for anything longer.

---

## As of

`ae501c5` on `main`, plus `lane/phase-15-small-corrections` open as a PR.
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

Branch `lane/phase-15-small-corrections`, open as a PR. It carries four of phase
15's items: `happenstance-sqlite` says *host only*; `REMEDIATION-HANDOVER.md` and
`SESSION-DECISIONS-0.2.0.md` say in their titles that they are records;
`[Unreleased]` rides `0.4.0`, and phase 17 says so; and two things found in
passing, a stale lint doc comment and a phase 13 work item. Every edit to a file
cited by line kept its line count.

`log.md` now names PR #17's squash, `ae501c5`.

## Next action

Merge this PR. Then take [phase 15](phases/15-reconcile.md)'s `.kb` lint
(`wi-38373d`): an xtask check that fails when an accepted atom's body differs from
the merge base, run in the gate.

## Waiting on the owner

- Review the Weigh-In defaults taken during the unattended session, with
  `/weigh:in digest`.
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
- **Nothing validates `.kb/` frontmatter now.** Read any diff under
  `.kb/decisions/` for an edited accepted body before it merges.
