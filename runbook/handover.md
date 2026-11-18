# Handover

**Rewritten whole at the end of every session, never appended to.** History goes
in [`log.md`](log.md) and in each phase's session log; this file is only *now*. If
its `As of` line does not name `HEAD` or its parent, it is stale — trust
`git log` and the status table over it, and say so in the next session log.

The template is the headings below, in this order. Keep each section short enough
to read in one screen; link out for anything longer.

---

## As of

`claude/project-thread-xxl8lr-070`, cut from `origin/phase-14` at `2c19afd1`, with
`origin/main` (#468, #470) merged in. 2026-11-16.

**GitHub Issues is the tracker** ([ADR-0088](../.kb/decisions/0088-github-issues-is-the-tracker.md)).
Per-item progress is in the issues. This file, the status table and the phase files
are the plan of record. Unattended sessions follow [`afk.md`](afk.md).

## Where things are

**Phase 14 is done** on `phase-14`, and this branch is its `0.7.0` release commit:
`[Unreleased]` becomes `[0.7.0] — 2026-11-16` under a summary that marks it breaking from
`0.6.x`, with a `0.7.0` trace of six hand rows: `BelowRetentionFloor`'s class moving from
`Stop` to `Reseed`, `MemorySyncPeer::with_limits`' floor, `RemoteSyncPeer`'s refusal before
sending, and the new rules in both testkits, with the sync-testkit's `REOPEN` declension
check. Nothing on the wire or in a schema moves. All nine crates, the workspace
requirements, the sync-testkit's pin, the testkit's recipe pins and the seven examples are
at `0.7.0`; `Cargo.lock` and seven off-workspace lockfiles move only the path crates. Status
and install lines (`"0.7"`), SECURITY's table, the README's table and roadmap, the site's
cards and reference table, two how-to pages that said adjudication was not built, the
`0.7.0` status row and phase 14's file follow.

**`0.6.0` is the latest on crates.io** (2026-11-09). Phases 0–18 are done, 17b included.
What remains is sequenced in [`roadmap.md`](roadmap.md): `1.0.0` (21), beside phases 19,
20 and 22.

**`0.7.0` is tagged on `main`, at `phase-14` → `main`'s merge commit**, as `0.6.0` was at
#462's: the release PR merges into `phase-14`, and `release.yml` publishes only a tag on
`main`, then creates the GitHub release from the tag's `## [0.7.0]` section
(`scripts/release-notes.sh`). Cloud sessions cannot push tags; the owner pushes them.

## In flight

The `0.7.0` release PR, `claude/project-thread-xxl8lr-070` → `phase-14`, is pushed and not
yet opened.

## Next action

1. The owner merges the release PR into `phase-14`.
2. `phase-14` → `main`. On its merge commit run `cargo semver-checks check-release
   --workspace --baseline-version 0.6.0 --release-type minor`; any lint no trace row
   covers becomes a **T** row in the `0.7.0` trace before the tag.
3. Tag `v0.7.0` on that merge commit. Check that the GitHub release job read the
   `## [0.7.0]` section.
4. On the merge, close #65, #144, #147, #151, #154, #156, #158 and #163, whose PRs merged
   into `phase-14` and so closed nothing.
5. Phase 21 (`1.0.0`, #68), with the retention harms phase 14 filed (#483, #484, #485) and
   #480 ahead of it.

```
repo:wet-ink-corporation/happenstance is:issue is:open label:status:ready-for-agent no:assignee -label:door:one-way
```

## Waiting on the owner

- The `0.7.0` release PR into `phase-14`, `phase-14` → `main`, the semver-checks run and
  the `v0.7.0` tag.
- No `docs/upgrading-to-0-7.md` was written. Every break is behaviour, none needs a code
  change to compile, and the trace states each; write a guide on the `0.6` precedent if
  you want one.
- #480: `sync_declensions_are_stated` still accepts an inherited `COMPACTION` declension,
  because every real peer fixture inherits it.
- Seven `[FROZEN]` `SY` clauses name a † rule that exists nowhere in the tree (SY-6, SY-9,
  SY-13, SY-16, SY-19, SY-26, SY-35), each declared in `SYNC_UNRESOLVABLE_RULE_NAMES`
  with its owner (#458). SY-33's, the eighth, was written by #465:
  `refusal_is_never_condition_derived`, in the sync peer family.
- Calls taken *as recommended* while the owner was asked, still unconfirmed: #101 O1–O3,
  #103 O1(a), #118's schema call, and #123 O1(B)/O2.
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
