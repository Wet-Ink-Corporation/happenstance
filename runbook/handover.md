# Handover

**Rewritten whole at the end of every session, never appended to.** History goes
in [`log.md`](log.md) and in each phase's session log; this file is only *now*. If
its `As of` line does not name `HEAD` or its parent, it is stale — trust
`git log` and the status table over it, and say so in the next session log.

The template is the headings below, in this order. Keep each section short enough
to read in one screen; link out for anything longer.

---

## As of

`65253fc` on `main`, plus `lane/runbook-citations-after-squash` open as a PR.
2026-09-28.

## Where things are

Seven crates are published at `0.3.2`; `EventStore` has been frozen since `0.2.0`
and `ProjectionStore` since `0.3.0`. Phases 0–12 are done. What remains is
sequenced in [`roadmap.md`](roadmap.md): reconcile the record (15), define 1.0
(16), a breaking window released as `0.4.0` (17), then sync (13), retention (14)
and the typed runner (18), and `1.0.0` (21) last. **Sync is inside 1.0**, so 13
and 14 are on the critical path. SQLite on `wasm32` (19) and the documentation
work (20) run alongside.

**Redkiln is retired here.** This runbook is the only tracker until `redkiln-rs`,
after 1.0. `.bklg/` and `.redkiln/` are frozen records; `CLAUDE.md`'s *Where the
work lives* has the rules.

## In flight

Branch `lane/runbook-citations-after-squash`, open as a PR. PR #14 (the split)
merged as a **squash**, `65253fc`, so `3916f29` — the commit the runbook cited as
the pre-split state — is not on `main`. It is still fetchable through
`refs/pull/14/head`, but citations belong on `main`: they now name `f89e184`,
where `RUNBOOK.md` and `HANDOVER.md` are byte-identical to `3916f29`. The frozen
banner at `RUNBOOK.md:3` names `65253fc`: its predecessor `f89e184` with four
lines edited in place — the banner and two status rows — so line numbers agree
and those four lines' text does not.

Verified: `cargo run -p xtask -- lints` green; `RUNBOOK.md` still 5,704 lines.

## Next action

Merge this PR, then [phase 15](phases/15-reconcile.md)'s next item: the top-level
documents that still describe `0.2.0` as unpublished — `README.md:139-141` and
`:175-176`, `CONTRIBUTING.md:23` and `:323-352`, and the comment at
`.github/workflows/ci.yml:1107-1116`.

## Waiting on the owner

- Nothing blocking. Optional: `lane/0.2.0-closeout`,
  `origin/worktree-kb-intake-2026-09-11` and `origin/lane/runbook-split` are all
  merged, and were outside D-3.

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
- **This repository's PRs have been squash-merged.** Any citation of a commit on a
  PR branch dies with the branch; cite a commit on `main`, or wait for the merge.
- **Nothing validates `.kb/` frontmatter now.** Read any diff under
  `.kb/decisions/` for an edited accepted body before it merges.
