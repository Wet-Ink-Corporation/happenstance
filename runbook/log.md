# Session log

Newest first. One entry per session: the date, the commit it ended on (or
*uncommitted*), what moved, and what was verified — named, so a later reader can
tell a check that ran from one that was assumed. Detail belongs in the phase
file's own session log; this is the index across phases.

Entries before 2026-09-28 are in the per-phase session logs of the frozen
monolith, `RUNBOOK.md`, and in `git log`.

---

## 2026-09-28 — the runbook becomes a directory

*Committed on `lane/runbook-split`, and opened as a PR.* Phase 15.

**Taken stock.** Four read-only audits across the runbook, the backlog, the crates
and specification, and the knowledge base. The findings are the work list in
[phase 15](phases/15-reconcile.md) and the ordering argument in
[`roadmap.md`](roadmap.md). In one line each: every plan-of-record document had
stopped between 2026-09-07 and 09-11; none of the backlog's 190 stories was at
done; nothing defined 1.0; and `happenstance-sqlite` does not build for `wasm32`.

**Changed.** The runbook split into `runbook/`, with `RUNBOOK.md` frozen in place;
the status lint repaired on two defects that let the table drift; phases 15–21
written; the roadmap written; a seed for SQLite on `wasm32`.

**Verified.**
- The xtask unit tests for the changed lints, including three new ones that fail
  on the table as `3916f29` left it.
- `cargo run -p xtask -- lints` green on the new files: 29 status rows against 5
  released versions, both clause ledgers matching §7.2, and V-6's 180 citations
  still resolving.
- The status check watched failing on the real table: phase 12 set back to
  `not started` and the `0.3.2` row removed gave four disagreements. The file
  was restored byte-identical.
- `RUNBOOK.md` and `HANDOVER.md` line counts unchanged: 5,704 and 420.
- `cargo xtask affected --base main` passed: the document checks, fmt, clippy and
  tests for the affected packages.

**Decided, later the same session.** D-1: sync is inside 1.0, overriding the
recommendation. D-2: `.bklg/` is frozen and this runbook tracks the rest. D-3:
delete the merged branches, waiting on a go. The owner then retired redkiln until
`redkiln-rs`, after 1.0, and `CLAUDE.md`, the roadmap, phases 15, 16, 19, 20 and
21, and the status table's phase 21 row were brought into line.

**Executed.** D-3: six squash-merged `lane/*` branches deleted locally and on
origin, and `wip/hs-p0012-benchmark-harness` locally; the tips are in phase 15.
The redkiln plugin disabled for this repository only, and its allowlist entries
removed, in the gitignored `.claude/settings.local.json`.

**Gate.** `cargo xtask ci` green on `lane/runbook-split` ("all checks passed",
exit 0). One step skipped, as its probe reports: LadybugDB projection conformance,
because `ladybug-configured` did not succeed on this machine.

**Not verified.** Nothing further on this change.
