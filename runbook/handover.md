# Handover

**Rewritten whole at the end of every session, never appended to.** History goes
in [`log.md`](log.md) and in each phase's session log; this file is only *now*. If
its `As of` line does not name `HEAD` or its parent, it is stale — trust
`git log` and the status table over it, and say so in the next session log.

The template is the headings below, in this order. Keep each section short enough
to read in one screen; link out for anything longer.

---

## As of

`claude/project-thread-xp895l-release-043`, on `ab1066c` (`main`, where PR #313
merged). 2026-10-14.

**GitHub Issues is the tracker** ([ADR-0088](../.kb/decisions/0088-github-issues-is-the-tracker.md)).
Per-item progress is in the issues. This file, the status table and the phase files
are the plan of record. Unattended sessions follow [`afk.md`](afk.md).

## Where things are

**`0.4.3` is prepared, not yet tagged.** Its release PR moves `[Unreleased]` to
`[0.4.3] — 2026-10-14` and all seven crates to `0.4.3`. It ships all three items of
#262 (Letterpress's G-01): `workerd` conformance across a real Durable Object restart
(#303), idle eviction (#304) and `happenstance-cloudflare`'s `remint_identity`
(#297). `0.4.2` is on crates.io (tag `v0.4.2` at `09d112e`, published by
`release.yml` through trusted publishing), and phase 17b is done.

**Phase 13's owner calls are answered** (2026-10-14). `happenstance-sync` and
`happenstance-sync-testkit` are claimed on crates.io at `0.0.0` (#102).
`SyncPeer::Error` is classified (`wi-cbaec7`, for ADR-0026), replication is
whole-log at 1.0 (`wi-53c544`, for ADR-0027), and SY-7, SY-10, SY-14 and SY-23 are
ratified with three edits (`wi-0fbcac`). Phases 0–12 and 15–17b are done. What
remains is sequenced in [`roadmap.md`](roadmap.md): the typed runner (18), sync (13)
and retention (14), and `1.0.0` (21) last.

**A tag is cut from its release PR's merge commit, not from a later `main`.** Cloud
sessions cannot push tags; the owner pushes them.

## In flight

- **The `0.4.3` release PR.** `door:one-way`: the owner merges it and tags the merge
  commit. The minimal-versions job's published-release leg may stay red until the tag
  publishes.
- **ADR-0026 and ADR-0027 are merged** (PRs #306 and #307), written from the decisions
  above. Both record decisions; no clause marker moves to `[FROZEN]` until the rule
  that checks it exists.

## Next action

1. Land and tag `0.4.3`. ADR-0027 took #122 (the filtered-subset store) and SY-29's
   `Query` off the 1.0 path.
2. Phase 18 (#63), the typed runner. Its breaking chains open as `door:one-way` PRs,
   and it needs the owner present. Phase 13's code waits on it (SY-20).
3. #298, the deployed restore leg of #262.

```
repo:wet-ink-corporation/happenstance is:issue is:open label:status:ready-for-agent no:assignee -label:door:one-way
```

## Waiting on the owner

- **Merge the `0.4.3` release PR, then tag its merge commit** (`git tag -a v0.4.3
  <merge-sha> -m "0.4.3"` and push the tag); `release.yml` publishes from the tag.
- **Trusted publishing for the two sync crates** on crates.io (repository
  `Wet-Ink-Corporation/happenstance`, workflow `release.yml`, environment
  `crates-io`), before `0.6.0`. `release.yml`'s publish set gains them in that
  release.
- **Phase 18** needs the owner present. **#118** gates phase 18's checkpoint-id chain.
- Defaults in force (the owner's, from the previous handover): `trait-variant`
  keeps its caret (ADR-0092); PS-25's digest is a hand-written FNV-1a; new CI jobs
  are not required checks.

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
