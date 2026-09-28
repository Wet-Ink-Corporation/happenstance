# Handover

**Rewritten whole at the end of every session, never appended to.** History goes
in [`log.md`](log.md) and in each phase's session log; this file is only *now*. If
its `As of` line does not name `HEAD` or its parent, it is stale — trust
`git log` and the status table over it, and say so in the next session log.

The template is the headings below, in this order. Keep each section short enough
to read in one screen; link out for anything longer.

---

## As of

`5afcca1` on `main`, plus `lane/phase-15-publication-state` open as a PR.
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

## In flight

Branch `lane/phase-15-publication-state`, open as a PR. It does two things.

First, it brings the record current. PR #15 merged by **rebase** as `db99a72` and
`5afcca1`, and `log.md` now names those commits.

Second, it fixes phase 15's publication-state item. `README.md`, `CONTRIBUTING.md`
and `ci.yml`'s two semver comment blocks no longer describe the registry as empty.
The rev baseline in `ci.yml` now lists all seven crates. `happenstance-postgres`
and `happenstance-neon` had been left out only while they lacked a published
predecessor, and this PR's `semver` job is the first run that includes them.

Verified: see the phase 15 session log.

## Next action

Merge this PR. Then take [phase 15](phases/15-reconcile.md)'s next open item:
`CLAUDE.md` and `crates/happenstance-sync/src/lib.rs:116-119` stop calling
"does ingest re-check append conditions" the central open question. SY-1 – SY-7
settled it.

Found in passing and not fixed: the doc comment on
`no_accepted_semver_break_outlives_its_reason` (`xtask/src/lints.rs:943`) still
says the registry-baseline step "is `if: false`". The check itself is correct,
because the exemption it guards is gone.

## Waiting on the owner

- **`0.3.3` or `0.4.0`** for `CHANGELOG.md`'s `[Unreleased]`: SQLite's busy
  timeout (ADR-0065) and `FaultyStore::contend_next`. The first is a behaviour
  change to a published adapter. Phase 15 lists this.
- Optional: `lane/0.2.0-closeout`, `origin/worktree-kb-intake-2026-09-11`,
  `origin/lane/runbook-split` and `origin/lane/runbook-citations-after-squash` are
  all merged. The first three were outside D-3; the last merged after it.

## Do not re-open

Settled, with the record that settled it. Re-opening one needs a new decision
record, and the owner.

- `happenstance-sync` is inside 1.0 — D-1, `wi-40b321`. The recommendation was
  the opposite and was overridden.
- Redkiln is retired until `redkiln-rs`; `.bklg/` is frozen and not advanced —
  D-2, `wi-016abe`, and the owner's instruction of 2026-09-28.
- Ingest is unconditional, with compensation — SY-1 – SY-7. (Two files still call
  it open; phase 15 corrects them.)
- The projection port is frozen — ADR-0063.
- The MSRV is 1.97.1 and a promise since `0.2.0` — ADR-0029, ADR-0037.
- `happenstance-macros` is out of scope — ADR-0033.
- `happenstance-ladybug` is finished and does not publish while `lbug` fails on
  docs.rs — phase 11, `crates/happenstance-ladybug/src/lib.rs`.
- The `0.2.0` release set is seven crates — `e597c34`.

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
