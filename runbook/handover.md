# Handover

**Rewritten whole at the end of every session, never appended to.** History goes
in [`log.md`](log.md) and in each phase's session log; this file is only *now*. If
its `As of` line does not name `HEAD` or its parent, it is stale — trust
`git log` and the status table over it, and say so in the next session log.

The template is the headings below, in this order. Keep each section short enough
to read in one screen; link out for anything longer.

---

## As of

`claude/v1-main-green`, cut from `origin/main` at `a58384c1` (`v0.8.0`). 2026-11-24.

**GitHub Issues is the tracker** ([ADR-0088](../.kb/decisions/0088-github-issues-is-the-tracker.md)).
Per-item progress is in the issues. This file, the status table and the phase files
are the plan of record. Unattended sessions follow [`afk.md`](afk.md).

## Where things are

**`0.8.0` is published** (all nine crates) and tagged on `main`. Phases 13–18 and 17b are
done; phase 20 (#66) and phase 22 (#67) are in progress; phase 21 (`1.0.0`, #68) waits on 20.
Phase 19 is off the 1.0 path.

The audit this record is drawn from is `runbook/v1-readiness-2026-11-24.md` (owner's working
copy, not committed). What it found that changes the plan:

- Three accepted ADRs were never built: ADR-0111 (ES-44, CF-44, CF-45;
  `UNCLAIMED_PENDING_ADR` still holds three), ADR-0112 (PS-15's pointer, the #115 box) and
  ADR-0113 (the SKIP line still goes to stdout; the Cloudflare README says it declines nothing).
- ADR-0114's three rules: ES-6 and ES-31 are test-only; **ES-29 / SY-6, the refusal of a wire
  condition carrying `after`, is breaking and ships as `0.9.0`**, the last breaking release.
- Seven `[FROZEN]` SY clauses — SY-6, SY-9, SY-13, SY-16, SY-19, SY-26, SY-35 — name rules
  that exist nowhere: `Scheduled(OWED_BY_458)` in `xtask/src/spec_trace/sync_family.rs`, and
  #458 is closed.
- Decisions phase 21 was handed belong in phase 20's window: CF-36's last three (#100),
  release tooling (#195), and PS-40/41, ES-18 and #209 need records.

## In flight

The v1 AFK run (2026-11-24), one PR per leaf, in this order: `main` green (#528); ADR-0113;
ADR-0111; ES-31 and ES-6; ADR-0112; #458's seven SY rules; #527; ES-29 / SY-6 as `0.9.0`;
phase 22's broken-link probe; `proposed` ADRs for the owner; `SECURITY.md` for 1.x (#207).

## Next action

1. Work the queue above. Each leaf's issue carries `source:audit` and milestone `1.0.0-rc.1`.
2. Then phase 20's documentation (#164, #165, #180–#184, #190), whose artefacts land in
   `references/evaluation/phase-20/`, never `.bklg/`.

```
repo:wet-ink-corporation/happenstance is:issue is:open label:status:ready-for-agent no:assignee -label:door:one-way
```

## Waiting on the owner

- **The outside reader (#166) is the critical path to 1.0**, not code. Phase 20 cannot
  close until a non-insider has read the docs, and the soak needs that reader again.
- Tagging and publishing `0.9.0` once the ES-29 / SY-6 PR lands on `main`.
- The `proposed` ADRs: PS-40/41 freeze, CF-36 scope (#100), ES-18 narrowing, release
  tooling (#195), phase 22 gating (#209).
- #71: `happenstance-ladybug` 0.0.0 still 404s on crates.io.
- The deployed `workerd` leg runs only on a manual dispatch (#515). Never dispatch it
  without asking.
- `.kb/decision/` (untracked) holds three weigh-in atoms from 2026-11-21 that belong in
  `.kb/decisions/` with map rows.

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
- Phase 13's nineteen clauses are frozen — ADR-0107, ADR-0105, ADR-0106.
- Phase 14's seven clauses are frozen, ES-39 as the refusal, and SY-32 with no real peer
  compacting yet — ADR-0108, ADR-0028.
- The projection port is frozen — ADR-0063.
- Phase 18's eight clauses are frozen, and the fan-out runner catches nothing —
  ADR-0102, ADR-0101.
- `happenstance-macros` is out of scope — ADR-0033.
- `[Unreleased]` ships in `0.4.0`; there is no `0.3.3` — `wi-052920`.
- An accepted decision's body is checked by `cargo xtask lint-kb` —
  `wi-38373d`.
- ADR-0111 – ADR-0115 are accepted; their follow-ups are builds, not decisions.
- Phase 19 (SQLite on `wasm32`) is off the 1.0 path — phase 21's dependency row.

## Traps

- **Bump to `0.9.0` in the ES-29 / SY-6 PR, and nowhere else.** It is the one breaking change left before `1.0.0-rc.1` (ADR-0114 (i)); the non-breaking leaves land before it so none is caught behind the bump.
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
