# Handover

**Rewritten whole at the end of every session, never appended to.** History goes
in [`log.md`](log.md) and in each phase's session log; this file is only *now*. If
its `As of` line does not name `HEAD` or its parent, it is stale — trust
`git log` and the status table over it, and say so in the next session log.

The template is the headings below, in this order. Keep each section short enough
to read in one screen; link out for anything longer.

---

## As of

`feature/friendly-wright-9lsxua`, on `1fe405a` (`main`, where PR #217 merged). 2026-10-10.

**GitHub Issues is the tracker** ([ADR-0088](../.kb/decisions/0088-github-issues-is-the-tracker.md)).
Per-item progress is in issues #61–#214. This file, the status table and the phase files
are the plan of record. Unattended sessions follow [`afk.md`](afk.md).

## Where things are

Seven crates are published at `0.3.2`. `EventStore` has been frozen since `0.2.0`,
and `ProjectionStore` since `0.3.0`. Phases 0–12, 15 and 16 are done; **phase 17
is in progress** and its code is all on `main`: only the `0.4.0` release is left.
Phases 20 and 22 run alongside it. What remains is sequenced in
[`roadmap.md`](roadmap.md): `0.4.0` (17), its additive half (17b), then the typed
runner (18), sync (13) and retention (14), and `1.0.0` (21) last.

**Phase 17 is split at its release** ([ADR-0072](../.kb/decisions/0072-phase-17-is-split-at-the-release.md)).
Phase 17 keeps what breaks or changes behaviour on a published crate, plus the
`workerd` job, and ends at `0.4.0`. Phase 17b takes the additive items.

**The tracker, counted on 2026-10-10:** 154 open issues. 24 are
`status:ready-for-agent`, 7 `status:ready-for-human`, 26 `status:needs-owner`,
2 `status:needs-triage` and 95 `status:blocked`, most of them behind the
`0.4.0` release or phase 18. The query, not this count, is the answer.

## In flight

Nothing on a branch. PR #216, from a fork, is open: a one-line fix for #91 (the
`Projection::apply` link that breaks `happenstance`'s default-features doc build). It
needs a maintainer's review and a CI run approval.
- **Merged on 2026-10-09:** #60 (the tracker vocabulary and seeded backlog) and #215
  (ADR-0088, held by `cargo xtask lints`).
- **Merged on 2026-10-07 and 2026-10-08:** phase 17's last lanes. See `log.md`.
- **Known, not this phase's:** the deployed `workerd` leg meets Cloudflare
  propagation answers. #217 adds error 1104, "Script not found", as the fourth answer it
  retries (`harness/workerd/platform-miss.mjs`). The HTML 500 that #51 met is still
  unclassified; #94 prints the page `<title>` of the next one.

## Next action

The `0.4.0` release (epic #61) is the owner's. **The owner decided on 2026-10-10
(`wi-f267f3`) that both pending breaks land first**, so the release waits on two PRs:

1. **#89** — ADR-0084's Postgres parameter-count check: refuse surplus parameters on
   a projection batch. It fills the trace table's one pending row.
2. **#78** — landed on `local/78`, for the owner to merge: `happenstance-neon` no longer
   exports `ProbeThenWriteStore`. As the owner chose on 2026-10-11 (`wi-17ec03`), it and
   `probe_request` are `#[cfg(test)]` and crate-private, the root re-export is gone, and
   the intra-doc links to it are code spans. Trace row T8 (`struct_missing`).
3. Then #96 (the release PR: trace table, heading, install lines), #97 (tag and
   publish) and #87.

#89 and #78 are `door:one-way` and `semver:breaking`: each is its own PR, carries
its `CHANGELOG.md` trace row, and is left for the owner to merge. #84 is answered
by the same decision.

#109 (ES-41's held-versus-visible reading) is not on this path. The owner chose **held**
on 2026-10-11 (`wi-33ab08`). That is what Postgres and Neon already answer, so the rule,
its mutant and the ADR the frozen clause needs are additive and stay in phase 13.

Until then, an agent takes the first issue the ready-for-agent query returns
([`afk.md`](afk.md) § *Taking work*):

```
repo:wet-ink-corporation/happenstance is:issue is:open label:status:ready-for-agent no:assignee -label:door:one-way
```

In milestone `0.4.0`, #82 (the 0.3 to 0.4 migration guide) and #91 (or merging
#216) should land before the release PR.

## Waiting on the owner

- **Merge #89 and #78** once each is open and green (above).
- **#73** — `assets/brand/happenstance-mark.png` is in no commit and absent from a
  fresh clone. If it exists only on your machine, commit it or discard it.
- Defaults in force (the owner's, from the previous handover): `trait-variant`
  keeps its caret (17b); PS-25's digest is a hand-written FNV-1a; VT-30 is a
  deprecated alias; new CI jobs are not required checks.

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
