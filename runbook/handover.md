# Handover

**Rewritten whole at the end of every session, never appended to.** History goes
in [`log.md`](log.md) and in each phase's session log; this file is only *now*. If
its `As of` line does not name `HEAD` or its parent, it is stale — trust
`git log` and the status table over it, and say so in the next session log.

The template is the headings below, in this order. Keep each section short enough
to read in one screen; link out for anything longer.

---

## As of

`lane/p17-workerd-green`'s working tree, uncommitted, on `1f92d088` (`main`, where
PR #34 merged L6a red). 2026-10-05.

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

**Lane L6b merged green (#35, `6a3adf6a`, 2026-10-06).** A `happenstance-cloudflare`
query item binds 0–3 parameters through `json_each(?)`
([ADR-0079](../.kb/decisions/0079-a-query-item-binds-a-constant-number-of-parameters.md));
`MAX_QUERY_ARMS_PER_STATEMENT` is 5 and `MAX_QUERY_PARAMETERS_PER_STATEMENT` is 90;
the shim enforces `workerd`'s four statement limits, so the gate runs Node 24.
- CI run 37419423991: local `workerd` 97 of 97, the deployed object 96 of 96.
- The deployed per-item wall is **32,514** max-length tags (8,527 locally);
  `experiments/durable-object-limits/results/run-workerd-deployed-2026-10-06.txt`.
- The `workerd` exit criterion in `phases/17-breaking-window.md` is ticked.

Phase 17 lanes L0 to L6b are merged (PRs #26–#28, #30–#32, #34, #35), ladybug
retired (#33), and the docs site landed (#29).

## Next action

1. **The vacuity control is still owed:** drop one name from `emit_dispatch` on a
   throwaway branch and watch the `workerd` job go red with `no such rule`.
2. The `0.4.0` trace table needs a hand row for `planned_statement_count`'s new
   values (ADR-0079), beside core's removed feature and the emitter renames.

## Waiting on the owner

- **A follow-up PR is owed for PR #33's review** (`wi-13bd3b`):
  - `declared_excludes` fails open and matches its key by prefix (`xtask/src/affected.rs:640`);
  - a stale `xtask/src/main.rs:89-103` citation in `lint_narrative.rs` and `narrative_doctests.rs`, which should be `:131-144`;
  - no test for a multi-line `exclude` array;
  - one doc paragraph is stale.
- Defaults the lanes will take unless overridden before they start:
  - `trait-variant` keeps its caret (17b);
  - PS-25's digest is a hand-written FNV-1a;
  - `ProjectionId` refuses the full ADR-0015 set, with a generic reserved prefix;
  - Neon's `push` narrowing rides `0.4.0`;
  - VT-30 is a deprecated alias;
  - the new CI jobs are not required checks (the `workerd` job is not one).
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
- **Live Neon flakes on the ES-11/ES-12 race** (`query_items_share_one_snapshot`, `read_result_is_stable_under_concurrent_append`) until L8 lands its fence. Re-run it; don't chase it.
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
