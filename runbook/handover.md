# Handover

**Rewritten whole at the end of every session, never appended to.** History goes
in [`log.md`](log.md) and in each phase's session log; this file is only *now*. If
its `As of` line does not name `HEAD` or its parent, it is stale — trust
`git log` and the status table over it, and say so in the next session log.

The template is the headings below, in this order. Keep each section short enough
to read in one screen; link out for anything longer.

---

## As of

`claude/project-thread-u1dtdj-050`, cut from `phase-18` at `0f0e3c3`. 2026-11-07.

**GitHub Issues is the tracker** ([ADR-0088](../.kb/decisions/0088-github-issues-is-the-tracker.md)).
Per-item progress is in the issues. This file, the status table and the phase files
are the plan of record. Unattended sessions follow [`afk.md`](afk.md).

## Where things are

**`0.5.0` is prepared, not yet tagged** (D9, the owner, AFK batch 9: `0.5.0` at phase
18's exit, and no `0.4.7`). Its release PR, for `phase-18`, moves `[Unreleased]` to
`[0.5.0] — 2026-11-07`; all nine crates were already at `0.5.0` (#375). It is breaking
from `0.4.x`: the typed runner leaves `unstable-projection`, which is removed (#132),
with an async `apply` handed a `Delivered<E>` (ADR-0074), a projection's own
`type Error` (PS-28), a `Chunk` (ADR-0070) and a derived checkpoint id (PS-25). The
CHANGELOG's `0.5.0` trace has ten hand rows and no tool rows, and
[`docs/upgrading-to-0-5.md`](../docs/upgrading-to-0-5.md) walks each row.

**Phase 18 is done** on `phase-18`. [ADR-0102](../.kb/decisions/0102-phase-18s-clauses-are-frozen.md)
froze PS-16, PS-18, PS-25, PS-27, PS-30, PS-38, SY-21 and CF-43. The specification
counts 168 `[FROZEN]`, 29 `[PROVISIONAL]`, 7 `[DEFERRED]` and 7 `[NON-NORMATIVE]`
(`cargo xtask spec-trace` is the authority).

**`0.4.6` is released** (2026-11-02; all nine crates on crates.io). Phases 0–12, 15–17b
and 18 are done. What remains is sequenced in [`roadmap.md`](roadmap.md): sync (13)
and retention (14), then `1.0.0` (21).

**This release is tagged on `main`, at #403's merge commit**, not at its release PR's
merge commit as earlier releases were: that PR merges into `phase-18`, which is not on
`main` until #403 lands, and `release.yml` publishes only a tag on `main`. Tag that
merge commit, not a later `main`. Cloud sessions cannot push tags; the owner pushes them.

## In flight

The `0.5.0` release PR, from `claude/project-thread-u1dtdj-050` into `phase-18`, and
then PR #403, `phase-18` into `main`. No release PR is open yet.

## Next action

1. The owner merges the `0.5.0` release PR into `phase-18`, then merges #403
   (`phase-18` → `main`). Before tagging, run `cargo semver-checks check-release
   --workspace --baseline-version 0.4.6 --release-type minor` on that merge commit;
   any lint it reports that no hand row covers becomes a **T** row in the `0.5.0`
   trace. The CI semver job does not stand in for it: its registry step passes no
   `--release-type`, so `0.4.6` → `0.5.0` reads as major and requires nothing. Then
   tag #403's merge commit (`git tag -a v0.5.0 <merge-sha> -m "0.5.0"`, then push the
   tag); `release.yml` publishes from the tag.
2. The two follow-ups #127 named are filed: #400, the `experiments/polling-cost` fan-out
   passes 002/003, and #401, optional, a live-Postgres reset through `TRUNCATE`.
3. Phase 13 (#64). It inherits SY-21's `convergent_projection_cannot_observe_local_position`,
   declared `Scheduled` in `UNRESOLVABLE_RULE_NAMES`; write it beside #141, and the entry
   goes stale on purpose.

```
repo:wet-ink-corporation/happenstance is:issue is:open label:status:ready-for-agent no:assignee -label:door:one-way
```

## Waiting on the owner

- The `0.5.0` release PR, #403, and the `v0.5.0` tag.
- Two protected files still describe `unstable-projection` or `0.4.x` as current, and
  agent sessions may not edit them: `CLAUDE.md` and
  `.github/ISSUE_TEMPLATE/bug_report.yml`. The release PR's body lists the lines.
- Calls taken *as recommended* while the owner was asked, still unconfirmed: #101 O1–O3,
  #103 O1(a), #118's schema call, and #123 O1(B)/O2. No freeze depends on them (ADR-0102).
- #123 left phase 13 a crate-boundary call: `happenstance-sync-testkit` depending on
  `happenstance` for #141.
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
