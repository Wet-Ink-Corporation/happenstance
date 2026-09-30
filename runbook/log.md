# Session log

Newest first. One entry per session: the date, the commit it ended on (or
*uncommitted*), what moved, and what was verified — named, so a later reader can
tell a check that ran from one that was assumed. Detail belongs in the phase
file's own session log; this is the index across phases.

Entries before 2026-09-28 are in the per-phase session logs of the frozen
monolith, `RUNBOOK.md`, and in `git log`.

---

## 2026-09-30 — the first breaking PR: 0.4.0 manifests, and the emitters made public

*Uncommitted at writing; `lane/p17-surface-renames`.*
Phase 17, lane L4.

The workspace and testkit now read `0.4.0`. The lane:
- executes ADR-0057;
- removes `happenstance-core`'s empty `unstable-projection`;
- moves `naive-arm` to a rustc cfg;
- renames CF-23's ten emitters to `emit_*` as public API (ADR-0076, CF-41 frozen).

Breaks the semver tool cannot see are recorded for the release's hand rows.

**Verified.** See phase 17's session log.

---

## 2026-09-30 — the apply record and the projection port's 1.0 clauses

*Uncommitted at writing; `lane/p17-apply-record`.*
Phase 17, lane L3.

ADR-0074 decides the typed layer's `apply`: async, handed a position-free
`Delivered<E>` and a batch to issue statements through, on `experiments/apply-shape`.
ADR-0075 settles the port's remaining 1.0 clauses. PS-9, PS-11, PS-15, PS-23 and
PS-24 are frozen. PS-15 was narrowed to `commit` and `reset` by the owner's
ruling (`wi-ff17f4`), after a walkthrough of the options.

**Verified.** See phase 17's session log.

---

## 2026-09-29 — ADR-0028: what a store may forget, decided as a refusal

*Uncommitted at writing; `lane/p17-provided-method-spike`.*
Phase 17, lane L2.

A compile spike showed a default-bodied method is additive on a `trait_variant`
port: `cargo-semver-checks` is clean against `0.3.2`, and a required-method
control is flagged major. On that evidence ADR-0028 keeps deletion outside the
port through 1.x, and reserves a report defaulting to `Unknown` as a later
additive method. ES-41 and PS-22 are frozen. `experiments/apply-shape` shows an
async `apply` with a batch handle working against a live Postgres, and it feeds
L3's record.

**Verified.** See phase 17's session log.

---

## 2026-09-29 — VT-10 frozen: the foreign-identity write path is the adapter's

*Uncommitted at writing; `lane/p17-foreign-identity`.*
Phase 17, lane L1.

ADR-0073 answers ADR-0026's published half. `happenstance-core` needs no write
path that preserves a foreign `EventId`: SQLite's own `write_batch` takes one,
and VT-10's falsifier did not fire on its named instrument. The spike is
`#[cfg(test)]` because `happenstance-sync` is unpublished. Neon's single-statement
ingest is structural evidence for SY-14. `happenstance-sync`'s placeholder
identity types are gone. Spec citations the edits shifted were repointed across
the tree.

**Verified.** See phase 17's session log.

---

## 2026-09-29 — phase 17 opens, split at its release

*Uncommitted at writing; `lane/p17-kickoff`.*
Phase 17.

The handover asked for a re-estimate at the start of phase 17. A read-only
research pass, one reader per cluster of work items and a sequencing synthesis,
put the phase at about 275 hours against its 5–8 days. The owner split it
(ADR-0072): phase 17 keeps what breaks or changes behaviour on a published crate,
plus the `workerd` job, and ends at `0.4.0`. A new phase, 17b, takes the additive
items, and phase 21 waits on it. VT-14, VT-30 and ES-7 moved to `freeze-by-17b`.
The status table, roadmap and ledgers carry the split. The owner also chose to
spike ES-11's fence on Neon, and to promise `AppendError::Busy`.

**Verified.** See phase 17's session log.

---

## 2026-09-29 — phase 16 is done: what 1.0 promises

*Merged as PR #24, squash `230065f`.*
Phase 16.

ADR-0066 is the 1.0 charter: nine crates by name, `happenstance-ladybug`
outside; versions released in lockstep at `1.0.0` and independent after; a
`workerd` sibling job before 1.0; the rc soak, supported versions and a licence
promise. Five sibling records settle what it needed first: ADR-0067 (MSRV after
1.0), ADR-0068 (ADR-0022 §§8, 16), ADR-0069 (a total `QueryItem` constructor),
ADR-0070 (the runner's `Chunk`) and ADR-0071 (ES-10 stays global). Seven owner
decisions are Weigh-In atoms, `wi-2798d5`, `wi-d61f21`, `wi-8e5bd4`,
`wi-460397`, `wi-1408e8`, `wi-cbc941` and `wi-7899af`.

Every non-frozen clause has one row in `ledgers.md`'s *The 1.0 dispositions*,
and `cargo xtask lints` now refuses a missing row, a bad disposition, or a
`freeze-by-N` naming a phase that is done or outside phase 21's prerequisites.
CF-39 is frozen. The other thirteen freezes the survey proposed were refuted
by adversarial verification and given later owners. Phase 17's work list is
the breaking half of the open questions, plus ADR-0022 §9's reproduction and
the guard-plan assertion (ADR-0068) and `QueryItem`'s total constructor
(ADR-0069). Phase 18 now runs before phase 13: SY-20's rule at 13 consumes the
convergence declaration 18 builds, and the two had no order between them.

**Verified.** See the phase 16 session log for the gate slices that ran.

---

## 2026-09-29 — phase 15 is done

*Merged as PR #23, squash `3dcba41`.*
Phase 15.

The owner decided `wi-6c9f77`: `CLAUDE.md`'s binding constraint 5 no longer
describes the registry as it stood before `0.2.0`. It states the registry as
read on 2026-09-29, and the MSRV rule it carries is unchanged. That was the last
open exit criterion, so phase 15 is `done`. Its decision atom joins the other
six Weigh-In atoms in `.kb/decisions/`.

**Verified.** See the phase 15 session log.

---

## 2026-09-28 — phase 15 at its exit, with one criterion waiting on the owner

*Merged as PR #22, squash `4c538e7`.*
Phase 15.

The last of the unattended session's work. The adapter pages stop hedging on a
release the registry records. Phase 15's exit criteria are ticked where true:
five of six. The third — no top-level document contradicting the registry in
the present tense — waits on `wi-6c9f77`, `CLAUDE.md`'s binding constraint 5,
which is the owner's to edit. Phase 15 stays `in progress` until it lands.

Merged this session: #18 (`0d59926`), #19 (`13e4dd8`), #20 (`6d35bc6`) and #21
(`5e60b3d`). The Weigh-In defaults it took are listed in phase 15's session log
and in the ledger, for `/weigh:in digest`.

**Verified.** The gate, in the slices named in the phase 15 session log.

---

## 2026-09-28 — every overdue open question has an owner

*Merged as PR #21, squash `5e60b3d`.* Phase 15.

Twenty-eight open questions had passed a deadline of their own — `0.2.0`, phase
12, the `ProjectionStore` freeze or an earlier phase. Each now has an owner in
phase 16, as a decision or as a clause disposition, and eight join the ledgers'
*Open decisions* table. The rule is `wi-0ed2c1`: phase 17 only when answering
breaks a published crate. None did once checked, because the runner is still
behind `happenstance`'s `unstable-projection`, and phase 16 re-reads every
assignment in its own classification item.

**Verified.** See the phase 15 session log.

---

## 2026-09-28 — the KB intake wave, by hand

*Merged as PR #20, squash `6d35bc6`.* Phase 15.

The first intake wave since redkiln retired, written by hand. The six Weigh-In
decision atoms move into `.kb/decisions/` in the neighbouring non-ADR shape,
bodies verbatim. Four open questions close as superseded — the query-union
rule, the testkit's exemption scope, the `0.0.0` reservations and the
pre-commit-only immutability check — and `es-38-and-gap-read-rules-are-unowned`
loses its answered half. `global-versus-per-boundary-visibility-invariant` was
verified and stays open, with phase 16. The orphan `cf-17-cf-14` question is
indexed, and the stale intake README is gone.

**Verified.** See the phase 15 session log.

---

## 2026-09-28 — an accepted decision's body cannot change

*Merged as PR #19, squash `13e4dd8`.* Phase 15.

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
