# Session log

Newest first. One entry per session: the date, the commit it ended on (or
*uncommitted*), what moved, and what was verified — named, so a later reader can
tell a check that ran from one that was assumed. Detail belongs in the phase
file's own session log; this is the index across phases.

Entries before 2026-09-28 are in the per-phase session logs of the frozen
monolith, `RUNBOOK.md`, and in `git log`.

---

## 2026-09-28 — an accepted decision's body cannot change

*`lane/phase-15-kb-lint`. Its commit on `main` is recorded here when it
merges.* Phase 15.

The owner's answer to `wi-38373d`, built. `cargo xtask lint-kb` fails the gate
when an atom under `.kb/decisions/` that was accepted at the merge base has a
changed body or has gone. Its frontmatter may change only in the supersession
keys and two pointer keys, and `accepted` may only become `superseded` with a
successor named — so a supersession passes, and so does a repointed citation — the one repair the governance atom
allows that a diff can see. It runs in `cargo xtask ci` and in `affected`, not
in `lints`, because it starts `git`. CI's `gate` job now checks out at
`fetch-depth: 0` and names the base in `HS_KB_BASE`; a shallow clone with no
base is an error, never a skip.

**Verified.** See the phase 15 session log.

---

## 2026-09-28 — four small corrections, and the record catches up with PR #17

*Merged as PR #18, squash `0d59926`.* Phase 15.

The handover said PR #17 was still open; `git log` shows it merged as `ae501c5`,
and the entry below now says so. Four of phase 15's items, on one branch because
their work files are disjoint (`wi-faa4be`):
- `happenstance-sqlite`'s crate root and README say *host only*: it does not build
  for `wasm32-unknown-unknown`, and no gate step checks that target.
- `REMEDIATION-HANDOVER.md` and `SESSION-DECISIONS-0.2.0.md` say in their titles
  that they are records, as `HANDOVER.md` already did. Bodies verbatim.
- `[Unreleased]` rides `0.4.0` (`wi-052920`), and phase 17 says so.
- Found in passing: a stale `if: false` in a lint's doc comment, corrected; and a
  phase 13 work item to delete `happenstance-sync`'s placeholder identity types.

Every edited file that is cited by line kept its line count.

**Verified.** See the phase 15 session log.

---

## 2026-09-28 — ingest re-checking is settled, and the documents say so

*Merged as PR #17, squash `ae501c5`.* Phase 15.

`CLAUDE.md` and `happenstance-sync`'s crate root called "does ingest re-check
append conditions" the central open question, and `peer.rs`'s `EventGroup` doc
said the same. The specification settled it in SY-1 – SY-7: six clauses frozen,
and SY-7, compensation authorship, provisional. All three now say so, and so do
two sentences of prose under SY-1 and SY-6 that had quoted the crate's claim. The
crate root's ledger bullets on identity (VT-5) and ordering (SY-19) were as stale
and are corrected with it. What `CLAUDE.md` now names as open is what the
specification defers: SY-14, SY-18, SY-27 and SY-32.

Every edit to a file cited by line — `SPECIFICATION.md`, and the crate's `lib.rs`
and `peer.rs` — kept its line count, so no citation moved. The frozen clauses'
MUSTs, markers and rules are unchanged; only non-normative prose under them was.

**Verified.** `cargo xtask spec-trace`, `cargo run -p xtask -- lints`,
`cargo xtask lint-constitution` and `cargo xtask affected --base main`, all green.
`cargo doc -p happenstance-sync` built with `-D warnings`, which covers the edited
intra-doc links.

---

## 2026-09-28 — the record current, and the registry is not empty

*Merged as PR #16, squash `c0df525`.* Phase 15.

The handover and this log brought current as of `5afcca1`: PR #15's entry names its
rebased commits. Then the top-level documents that still described the registry as
empty — `README.md`'s status legend and its postgres paragraph, `CONTRIBUTING.md`'s
provisional-ADR paragraph and its semver section, and both semver comment blocks in
`ci.yml` — say what is true: seven crates at `0.3.2` on crates.io, and the registry
baseline running since `86a410c`. The rev baseline now watches all seven crates;
`happenstance-postgres` and `happenstance-neon` were left out only because they had
no published predecessor, and the comment that excluded them said they rejoined at
`0.2.0`.

**Verified.** `max_stable_version` is `0.3.2` for all seven crates, measured against
the crates.io API before the README was edited. The gate checks are listed in the
phase 15 session log.

---

## 2026-09-28 — citations moved off a squashed commit

*Merged as PR #15 by **rebase**, not squash: `db99a72` and `5afcca1` on `main`.
The PR-branch commits `6d49501` and `029dfba` are not on `main`.* Phase 15.

PR #14 merged as a squash (`65253fc`), leaving `3916f29` off `main`. Eleven
citations named it; ten now name `f89e184`, where `RUNBOOK.md` and `HANDOVER.md`
are byte-identical to it, and the handover is rewritten. The squash was checked
byte-identical to the PR head `b38d966` before local `main` was moved to it.

**Verified.** `cargo run -p xtask -- lints` green; `RUNBOOK.md` line count
unchanged at 5,704.

---

## 2026-09-28 — the runbook becomes a directory

*Merged as PR #14, squash `65253fc`.* Phase 15.

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
  on the table as `f89e184` left it.
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
