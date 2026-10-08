# Handover

**Rewritten whole at the end of every session, never appended to.** History goes
in [`log.md`](log.md) and in each phase's session log; this file is only *now*. If
its `As of` line does not name `HEAD` or its parent, it is stale — trust
`git log` and the status table over it, and say so in the next session log.

The template is the headings below, in this order. Keep each section short enough
to read in one screen; link out for anything longer.

---

## As of

`lane/p17-exit-pass`, on `527dc08` (`main`, where PR #58 merged). 2026-10-08.

## Where things are

Seven crates are published at `0.3.2`. `EventStore` has been frozen since `0.2.0`,
and `ProjectionStore` since `0.3.0`. Phases 0–12, 15 and 16 are done; **phase 17
is in progress**. What remains is sequenced in [`roadmap.md`](roadmap.md): the
breaking window released as `0.4.0` (17), its additive half (17b), then the typed
runner (18), sync (13) and retention (14), and `1.0.0` (21) last.

**Phase 17 is split at its release** ([ADR-0072](../.kb/decisions/0072-phase-17-is-split-at-the-release.md)).
Phase 17 keeps what breaks or changes behaviour on a published crate, plus the
`workerd` job, and ends at `0.4.0`. Phase 17b takes the additive items.

**The owner's calls at kickoff:** spike the ES-11 fence on Neon (ask again if it
fails); promise `AppendError::Busy`, with the typed commit loop retrying it inside
`Retry`. **L8 has landed** (#57, the ES-11 fence), so the live Neon job's known
race is fixed on `main`; it becomes a required check again when the owner re-adds
it to the ruleset (`wi-0f1291`).

## In flight

Nothing. Phase 17's work is on `main`; only the `0.4.0` release is left.
- **Merged on 2026-10-07:** #41, #42, #46, #47, #49, #52.
- **Merged on 2026-10-08:**
  - #53 (L10, ADR-0082);
  - #43 (ADR-0083), #44 (ADR-0084), #45 (ADR-0086);
  - #55 (the owner's decision intake);
  - #48 (L9, ADR-0081);
  - #51 (ADR-0087; ADR-0061 superseded);
  - #57 (the ES-11 fence; ES-11 and ES-12 frozen);
  - #58 (deterministic racing mutants).
- **Closed unmerged, as planned:** spike #50.
- **The full gate** passes on `527dc08`.
- **Known, not this phase's:** the default-features doc build of `happenstance`
  fails on `lib.rs:111`'s `Projection::apply` link. The deployed `workerd` leg met
  one unclassified Cloudflare HTML 500 on #51; it passed on re-run. Printing the
  page `<title>` on a failure is the proposed follow-up.

## Next action

1. Release `0.4.0`. The trace table in `CHANGELOG.md`'s `[Unreleased]` carries every
   break; its one pending row, ADR-0084's Postgres parameter-count check, is accepted
   and owed, and either lands first or is recorded as left for after the release.

## Waiting on the owner

- Re-add `conformance against a live Neon endpoint` to the `Protect main` ruleset
  (id 22926481) as a required check (ADR-0087 D8).
- Decide whether ADR-0084's Postgres parameter-count check lands before `0.4.0`.
- Defaults in force (the owner's, from the previous handover): `trait-variant`
  keeps its caret (17b); PS-25's digest is a hand-written FNV-1a; VT-30 is a
  deprecated alias; new CI jobs are not required checks.
- Still open from phase 15: the Weigh-In digest; the merged `lane/*` branches; the untracked `runbook/phase-15-afk-prompt.md` and `assets/brand/happenstance-mark.png`.

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
  D-2, `wi-016abe`.
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
- **Live Neon flakes on the ES-11/ES-12 race** (`query_items_share_one_snapshot`, `read_result_is_stable_under_concurrent_append`) until L8 lands its fence. Re-run it; don't chase it. **It is not a required check until then** (`wi-0f1291`, 2026-10-06: dropped from the `Protect main` ruleset, id 22926481). When L8 lands, the owner re-adds it to that ruleset and confirms it is required; a Neon red on a PR still needs a look before merging.
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
