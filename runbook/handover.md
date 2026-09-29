# Handover

**Rewritten whole at the end of every session, never appended to.** History goes
in [`log.md`](log.md) and in each phase's session log; this file is only *now*. If
its `As of` line does not name `HEAD` or its parent, it is stale — trust
`git log` and the status table over it, and say so in the next session log.

The template is the headings below, in this order. Keep each section short enough
to read in one screen; link out for anything longer.

---

## As of

`3dcba41` on `main`, plus `lane/phase-16-define-1-0` open as one PR for the
owner to merge. 2026-09-29.

## Where things are

Seven crates are published at `0.3.2`. `EventStore` has been frozen since `0.2.0`,
and `ProjectionStore` since `0.3.0`. Phases 0–12 and 15 are done, and **phase 16
is done in this PR**. What remains is sequenced in [`roadmap.md`](roadmap.md):
- a breaking window, released as `0.4.0` (17);
- then sync (13), retention (14) and the typed runner (18);
- and `1.0.0` (21) last.

**1.0 is now defined.** [ADR-0066](../.kb/decisions/0066-what-1-0-promises.md)
names nine crates (the seven published plus `happenstance-sync` and
`happenstance-sync-testkit`; `happenstance-ladybug` is outside). Every non-frozen
clause has one row in [`ledgers.md`](ledgers.md)'s *The 1.0 dispositions*, and
`cargo xtask lints` holds it: `freeze-by-N` must name a phase that is not done
and is inside phase 21's prerequisites. CF-39 is the one clause frozen here.

**Redkiln is retired here.** This runbook is the only tracker until `redkiln-rs`,
after 1.0. `.bklg/` and `.redkiln/` are frozen records; `CLAUDE.md`'s *Where the
work lives* has the rules.

## In flight

Branch `lane/phase-16-define-1-0`, one PR, left open for the owner:
- six records, ADR-0066 – ADR-0071, and seven Weigh-In atoms;
- the disposition table and its lint;
- the open questions closed or routed;
- phase 17's work list;
- the phase files' new items.

## Next action

The owner reviews and merges the phase 16 PR, and fills in its squash commit in
`log.md`. Then start [phase 17](phases/17-breaking-window.md), the breaking window.
Its heaviest item is the foreign-identity spike (VT-10), so start there. The
`workerd` sibling job and the minimal-versions job are the other new
infrastructure it owes.

## Waiting on the owner

- Merge the phase 16 PR. Your seven calls are recorded as `wi-2798d5`,
  `wi-d61f21`, `wi-8e5bd4`, `wi-460397`, `wi-1408e8`, `wi-cbc941` and
  `wi-7899af`.
- ADR-0066 §5 is the record's own call rather than yours: the semver exemption
  list, and re-exported driver error payloads being inside the promise. Revise
  it before merge if you disagree.
- Still open from phase 15: the Weigh-In digest (`/weigh:in digest`); the merged
  `lane/*` branches, none deleted; and the untracked `runbook/phase-15-afk-prompt.md`
  and `assets/brand/happenstance-mark.png`.

## Do not re-open

Settled, with the record that settled it. Re-opening one needs a new decision
record, and the owner.

- What 1.0 promises: nine crates, ladybug outside, versioning, `workerd` before
  1.0, soak, support window and licence — ADR-0066 and its seven `wi-*` atoms.
- The MSRV holds at 1.97.1, and after 1.0 a rise is bounded — ADR-0067.
- ES-10 stays global — ADR-0071.
- `happenstance-sync` is inside 1.0 — D-1, `wi-40b321`.
- Redkiln is retired until `redkiln-rs`; `.bklg/` is frozen and not advanced —
  D-2, `wi-016abe`.
- Ingest is unconditional, with compensation — SY-1 – SY-7.
- The projection port is frozen — ADR-0063.
- `happenstance-macros` is out of scope — ADR-0033.
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
- **Thirteen of fourteen proposed freezes were refuted.** A clause's marker
  moves on evidence in the tree, not on an argument that its falsifier is
  decorative. ADR-0066 §2 records each refutation. Read it before proposing to
  freeze anything early.
