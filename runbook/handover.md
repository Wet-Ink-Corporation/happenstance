# Handover

**Rewritten whole at the end of every session, never appended to.** History goes
in [`log.md`](log.md) and in each phase's session log; this file is only *now*. If
its `As of` line does not name `HEAD` or its parent, it is stale — trust
`git log` and the status table over it, and say so in the next session log.

The template is the headings below, in this order. Keep each section short enough
to read in one screen; link out for anything longer.

---

## As of

`9d99339` on `main` (PR #33: `happenstance-ladybug` retired, ADR-0078), plus
`lane/p17-workerd`, which opens lane L6a with this file. 2026-10-01.

## Where things are

Seven crates are published at `0.3.2`. `EventStore` has been frozen since `0.2.0`,
and `ProjectionStore` since `0.3.0`. Phases 0–12, 15 and 16 are done; **phase 17
is in progress**. What remains is sequenced in [`roadmap.md`](roadmap.md): the
breaking window released as `0.4.0` (17), its additive half (17b), then the typed
runner (18), sync (13) and retention (14), and `1.0.0` (21) last.

**Phase 17 is split at its release** ([ADR-0072](../.kb/decisions/0072-phase-17-is-split-at-the-release.md)).
A read-only research pass over every item re-estimated the unsplit phase at about
275 hours. Phase 17 now keeps what breaks or changes behaviour on a published
crate, plus the `workerd` job, and ends at `0.4.0` (25–30 days). Phase 17b takes
the additive items (8–10 days), and phase 21 waits on it. VT-14, VT-30 and ES-7
are `freeze-by-17b`.

**The owner's calls at kickoff:** spike the ES-11 fence on Neon (ask again if it
fails); promise `AppendError::Busy`, with the typed commit loop retrying it inside
`Retry`; a `NEON_CONNECTION` secret, a Cloudflare API token for a deployed Durable
Object leg, and Docker for testcontainers Postgres are available.

## In flight

Nothing merged is pending. Phase 17 lanes are merged through L5: L0 to L5 are PRs #26–#28 and #30–#32. `happenstance-ladybug` was retired by PR #33 (ADR-0078). Phase 22's documentation site merged as #29.

## Next action

**Lane L6a: the `workerd` CI job, landed red on purpose**, on `lane/p17-workerd`. Read the plan's L6a section first (`C:/Users/ryanm/.claude/plans/ultracode-runbook-phases-17-breaking-win-sequential-puppy.md`), then the research brief's `ci-jobs` item. The brief is in the kickoff workflow's journal; phase 17's session log says where. The lane:
- adds a `harness/workerd` crate (publish = false, a workspace member) holding a real `#[durable_object]` class and a `dispatch` emitter under CF-23's public names (`emit_*`, ADR-0076);
- runs the conformance rules through `@cloudflare/vitest-pool-workers`, with a committed lockfile, as a sibling job shaped like `live-neon`;
- adds a **deployed-Durable-Object leg** using the `CLOUDFLARE_API_TOKEN` and `CLOUDFLARE_ACCOUNT_ID` repository secrets. Both are set; the token is account-owned and verified active, but its Workers permissions are unverified, and the first deploy will show whether they are enough;
- must **fail on today's code**, on VT-23's 128-arm UNION (`COMPOUND_SELECT`), and records that run's URL;
- measures the SQL-length, compound-term, parameter and depth walls, locally and deployed, into `experiments/durable-object-limits`.

Two-instance rules need a namespaced constructor on `CloudflareEventStore`, which becomes a 1.0 promise; flag it in the PR. L6b, a separate PR, then lowers Cloudflare's partition constants from the measurement.

## Waiting on the owner

- **A follow-up PR is owed for PR #33's review** (`wi-13bd3b`):
  - `declared_excludes` fails open and matches its key by prefix (`xtask/src/affected.rs:639`);
  - a stale `xtask/src/main.rs:89-103` citation in `lint_narrative.rs` and `narrative_doctests.rs`, which should be `:131-144`;
  - no test for a multi-line `exclude` array;
  - one doc paragraph is stale.
- Defaults the lanes will take unless overridden before they start:
  - `trait-variant` keeps its caret (17b);
  - PS-25's digest is a hand-written FNV-1a;
  - `ProjectionId` refuses the full ADR-0015 set, with a generic reserved prefix;
  - Neon's `push` narrowing rides `0.4.0`;
  - VT-30 is a deprecated alias;
  - the new CI jobs are not required checks.
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
- **Spec and source edits shift `path:N` citations tree-wide.**
  - Snapshot the file first, then map old line numbers to new with difflib and repoint once. The scratchpad scripts `specmap2.py` and its siblings did this.
  - Never repoint inside historical quotes: `SESSION-DECISIONS-*`, `RUNBOOK.md`, `references/`, `.kb/_governance`, experiment results, and released CHANGELOG sections.
  - An accepted atom's frontmatter `summary` must not change, even for a citation.
- **Rebasing over a lane that repointed the same citations:** take `main`'s version of each file whose diff is citation-only, recompute the shift from `main`'s copies, and apply it once. Hand-fix ranges whose endpoints fell in deleted text.
- **Git Bash mangles `rev:path`.** Set `MSYS_NO_PATHCONV=1` for `git show REV:path`.
- **A hook refuses shell edits whose command text mentions a `.rs` path,** even in markdown. Use the Edit tool.
- **Live Neon flakes on the ES-11/ES-12 race** (`query_items_share_one_snapshot`, `read_result_is_stable_under_concurrent_append`) until L8 lands its fence. Re-run it; don't chase it.
- **The CI base-commit semver step is advisory** while `0.4.0` is unpublished (a crates.io probe). The release trace comes from `cargo semver-checks check-release --workspace --baseline-version 0.3.2 --release-type minor`.

- **At `0.4.0` the semver tool skips every lint.** The trace table must come from `cargo semver-checks check-release --workspace --baseline-version 0.3.2 --release-type minor`, plus hand rows for core's feature removal and the hidden emitter renames.

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
