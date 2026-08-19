---
item: HS-S0089
stage: spec
created: 2026-08-12T13:47:27.268Z
updated: 2026-08-12T13:47:27.268Z
template_sig: 87bbf1d0
rendered_sig: 1b31f734
---

# Spec — A publish-time clause audit that reconciles and cannot be talked out of a frozen clause

## Scope lock

| | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DoD 12, *"the clause ledger is audited at publish"* (`:392-394`) |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/publication-and-positioning/project.md` — DR-3, DR-4, DR-15; AC-003, AC-004, AC-015 |
| This spec | `.bklg/from-contract-to-published-library/publication-and-positioning/clause-maturity-audit/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` — **testing brief** (`### Test mix` AC-003/AC-004/AC-015 rows, AC-TEST-002's `mod tests` obligation) and **deployment brief** (`### CI implication`, AC-DEP-005: Mandatory, never behind a probe). No `architecture` brief exists for this project, deliberately. |
| Signed-off design | `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` — binding on DT-5's census sentence, which **takes its four numbers from this audit's report at the publish commit** (`### DT-5`). This story renders none of its surfaces. |
| Grounding | `.bklg/from-contract-to-published-library/publication-and-positioning/_grounding.md` — *Code patterns to follow*: compose with `spec-trace`'s existing split, do not duplicate the parser |
| Story map / merge order | `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md` — milestone `publish-time-gate-instruments`, position 2.2, strictly after `falsifier-ledger-repair` |

## One-line PR slice

Add a mandatory `cargo xtask` clause-audit step that composes with `xtask/src/spec_trace.rs`'s
existing parser: it reports every clause's maturity at the commit, reconciles its totals against
§1.3's hand-computed 200/198/139/49/10/2, asserts the repaired ledger's clause-ID set equals the
parsed `[PROVISIONAL]` list, fingerprints every `[FROZEN]` clause against the tree the project
received, and carries `#[cfg(test)] mod tests` in the `xtask/src/package.rs`:408-457 shape whose
fixtures include a seeded §1.3 disagreement and a seeded frozen-clause edit that it must fail on.

## Executive summary

**What lands.** A new `xtask` module, `clause_audit`, wired into `xtask/src/main.rs`'s `REQUIRED`
step list and into `xtask/src/affected.rs`'s unconditional file-reading block; a dated, committed
audit report under `spec/audits/`; and a provenanced fingerprint baseline for the 139 `[FROZEN]`
clauses. Three checks, one report, one baseline.

**The delta against what the tree already does.** `spec-trace` already parses every clause's
maturity marker, already forbids an empty falsifier on a `[PROVISIONAL]`/`[DEFERRED]` marker
(CF-38), and already reconciles §1.3's hand-written census against what it counted
(`xtask/src/spec_trace.rs:459-506`, `check_stated_census`). So **the reconciliation in AC-003 is not
new enforcement — it is an existing check this story must reuse rather than re-implement**, and the
report is what makes its result an artefact rather than a console line that scrolls past. What is
genuinely absent from the tree today, and is what this PR is for:

1. **Nothing checks `RUNBOOK.md`'s falsifier ledger against the specification.** The ledger was
   reconciled by hand once and agreed *"at that commit"* (`RUNBOOK.md:618-620`); it has been wrong
   ever since, by its own admission (`RUNBOOK.md:622-635`). `falsifier-ledger-repair` fixes the five
   rows; this story is what stops them going wrong again.
2. **Nothing can observe that a `[FROZEN]` clause changed.** `spec-trace` checks the document's
   internal consistency; a frozen clause edited to agree with a wrong implementation is *more*
   internally consistent afterwards. AC-015 is unobservable in this tree until a baseline exists.
3. **There is no committed clause-maturity artefact.** Project DoD 3 requires one with a date, and
   `_design.md`'s DT-5 census sentence is specified to take its four numbers from it rather than
   from memory.

**What this PR is not.** It changes no clause, repairs no ledger row, and states no deferral reason.
Those are `falsifier-ledger-repair` (landed before this one) and `deferred-clause-reread` (extends
this module afterwards).

## Context pack

Everything an implementer needs to start. Deeper material is behind the anchors, never pasted here.

**The audit composes with one parser; it does not become a second one.** `spec_trace::parse_clauses`
is the only thing in this repository that reads clause IDs, maturity markers and falsifiers out of
`spec/SPECIFICATION.md`, and `Census`/`check_stated_census` are the only things that count them. A
second parser would double the surface on which the document could be misread and would break the
property §1.3 exists to prove — that a human's count and the machine's count are *independent*
(`xtask/src/spec_trace.rs:39-57`). The mechanical consequence: `parse_clauses`, `Clause`, `Census`
and `check_stated_census` are private today and this story widens exactly those to `pub(crate)`.
Widening visibility is the mount; copying the code is the failure.

**§1.3 is checked and never generated, and this story must not quietly reverse that.** The module
doc at `xtask/src/spec_trace.rs:39-57` predicts the next contributor's first instinct — that §1.3
"looks like the only hand-maintained number left" — and forbids it. The audit's report *prints* both
figures side by side so the artefact is self-contained; the report is not an input to any check.

**The falsifier ledger's failure mode is arithmetic that agrees with a heading and not with the
rows.** `RUNBOOK.md:588-635` groups the 49 `[PROVISIONAL]` clauses by what falsifies them; the
heading was corrected to 49 while the rows were left short by ES-41, ES-42, CF-39 and CF-40 and
still carrying ES-10. So the check is **three-way**: the heading's own number, the union of the
`Clauses` column, and `spec-trace`'s parsed `[PROVISIONAL]` set must all agree. A two-way check
between the rows and the specification would have passed the exact defect the runbook documents in
its own words.

**Ledger parsing has two traps the fixtures must carry.** The `Clauses` column uses en-dash ranges
(`PS-4 – PS-6`, `VT-21 – VT-24`) that must be expanded, and the last row's `Falsified by` prose
mentions clause IDs (`ES-10`, `ES-11`, …) that are **not** ledger membership. A parser that sweeps
the whole row rather than the `Clauses` cell reports a set that is wrong in both directions.

**Fail closed, and say why — never find nothing and pass.** ADR-0010's discipline is that a skip is
reported, never silent (`.kb/decisions/0010-the-suite-must-prove-itself.md`), and the deployment
brief applies it to this instrument by name. Concretely: a renamed ledger heading, an absent
baseline, an unreadable report target or a clause carrying no marker this table knows must each fail
with the path, the line and what the reader is expected to do — a ledger locator that finds zero
rows and reports two equal empty sets is the single most likely wrong implementation of this story.

**The frozen fingerprint is a promise about *provenance*, not about bytes.** AC-015 says no
`[FROZEN]` clause differs between the tree this project received and the tree it published, so the
baseline is captured from the merge-base with `main` — the tree received — and its header records
that rev and the date. Two decisions follow, and both were made here rather than left to the
implementer:

- **The fingerprint is over whitespace-normalised clause body text, not raw bytes.** A markdown
  re-wrap is cosmetic; a word change is not. Byte-exact fingerprints fire on re-wraps, and a check
  that cries wolf trains its owner to re-baseline, which is how the check dies.
- **The hash must be value-stable across toolchains.** `std::collections::hash_map::DefaultHasher`
  is explicitly not stable across Rust releases, so a baseline hashed with it silently "expires" on
  the next toolchain bump and re-baselining becomes routine. Write a small, documented FNV-1a/64 in
  the module with a pinned test vector instead, so the value is defined by code in this repository.

**Re-baselining is a recorded act, not an escape hatch.** `--rebaseline` exists (a baseline nobody
can regenerate is a baseline that gets deleted), but it is never run by the gate, its diff is
reviewable, and DR-15 makes running it to unblock a release a **stop**: a frozen clause found wrong
becomes a new decision atom and a re-plan, not an edit and a fresh baseline
(`project.md`, DR-15 / AC-015).

**Mandatory, never behind a probe.** `xtask/src/main.rs:33-38` draws the Mandatory/Optional line on
one basis only: optional means *skipped when the probed tool is absent*, never *skipped when the
check would fail*. This step probes for nothing — its inputs are two files in the tree — so gating it
behind a probe would invent the escape hatch CLAUDE.md's decorative-gate rule exists to forbid
(deployment brief, AC-DEP-005). It must also join `xtask/src/affected.rs:116-125`'s unconditional
block, because the two stories that most need it — the ledger repair and the deferred-clause re-read
— touch no Rust package at all, and the story-grain gate would otherwise skip the only check that
could fail them.

**The persona slice.** Backbone activity **A2**, *"prove the promises before making them"*
(`_storymap.md`): the maintainer at the release gate, with no user intent of its own, which is
exactly why this story is a gate instrument consumed downstream. It reaches Persona 4, the evaluator,
through one hop only: `_design.md`'s DT-5 resolution sites a dated census sentence on the packaged
READMEs whose four numbers come from **this report at the publish commit**, so that a reader learns
what "provisional" means in the same sentence that uses it. Getting the report's shape wrong makes
that sentence unwritable without typing numbers from memory, which is the failure mode DT-5's
resolution names.

**What this story must not settle in passing.**
`.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md` records that no phase obliges
anyone to read the specification back against the tree, and names first publish as its secondary
forcing event. This audit discharges one instance of that; whether it *resolves* the atom is
`crate-set-decision`-adjacent decision work under AC-013, not a side effect of landing a gate step
(`project.md`, *Coupling notes*).

## Integration contract

| | |
| --- | --- |
| **Archetype** | `capability` — the maintainer runs one command and gets an answer that can block a release. |
| **Slice / milestone** | `publish-time-gate-instruments`. Slice-mates: `falsifier-ledger-repair` (lands first), `deferred-clause-reread` (extends this module), `registry-surface-diff` (the sibling instrument mounting into the same list). |
| **Mount point** | **`xtask/src/main.rs`** — the gate's composition root. Four edits in the same change: the `REQUIRED` array (`:106` onward, a `Step` with `probe: None`, following the `spec-trace` entry at `:303-328`); the `main` dispatch (`:639-700`, a `Some("clause-audit")` arm with the flag handling `spec-trace`'s arm models at `:671-679`); `print_help` (`:724-753`); and the module doc's *"What the gate proves"* paragraph (`:8-24`) — the one place in this repository that states in prose what the gate proves, which the deployment brief requires be updated in the same change. |
| **Second mount** | **`xtask/src/affected.rs:116-125`** — the unconditional file-reading block, so the story-grain gate (`cargo xtask affected --base main`, wired at `.redkiln/config.yaml`) runs it for a diff that touches no package. |
| **Wires into** | `xtask/src/spec_trace.rs` — `parse_clauses`, `Clause`, `Census`, `check_stated_census`, `SPEC`, `workspace_root` (visibility widened to `pub(crate)`, no behaviour change); `spec/SPECIFICATION.md` (read-only); `RUNBOOK.md:588-635`, the ledger `falsifier-ledger-repair` has already repaired (read-only); `git` via `Command`, as `affected.rs` already does, for the rev the baseline is captured from. |
| **Renders surfaces** | **None.** `_design.md`'s seven surfaces are rendered pages owned by `published-surface-copy` and `rendered-page-preflight`. This story is upstream of one of them by data only: DT-5's census sentence consumes this report. |
| **Public items** | None. `_design.md`'s `## Items` block carries two entries and this story implements neither — `xtask` is `publish = false` (`xtask/Cargo.toml:7`) and adds no public API. That is the correct reading of the Items block, not a gap in it. |
| **Conformance rule(s)** | **None, and it is not adapter-observable.** This story constrains a *document* and the gate that reads it, not a port; no adapter can pass or fail it. The equivalent obligation is discharged in the module's own `#[cfg(test)] mod tests` (testing brief AC-TEST-002), where each fixture names the wrong implementation it rejects. |
| **Clause(s)** | Reads all 200; **amends none**. It is the machinery behind CF-38's neighbourhood rather than a change to it. Nothing `[FROZEN]` is edited — DR-15 and AC-015 make that a release blocker plus a decision atom, not an edit. |
| **Advances DoD scenario** | Initiative **DoD 12** — *"The clause ledger is audited at publish. A run over the specification reports every clause's maturity, and no clause is provisional with an empty falsifier; the count in the report matches `spec/SPECIFICATION.md`'s own stated figure"* (`initiative.md:392-394`). It also supplies the artefact project DoD 3 requires and makes initiative DoD 13's *"green on the exact tree that was published"* mean one more thing. |

## PR boundary

The paths this story may touch. `redkiln verify --grain story` reads the first fenced block under
this heading and fails on any file changed outside it.

```
xtask/src/clause_audit.rs
xtask/src/main.rs
xtask/src/affected.rs
xtask/src/spec_trace.rs
spec/audits/**
.bklg/from-contract-to-published-library/publication-and-positioning/clause-maturity-audit/**
CHANGELOG.md
```

**In this PR**

- `xtask/src/clause_audit.rs` — the module: the three checks, the report writer, the baseline
  reader/writer, the FNV-1a/64 with its pinned test vector, and `#[cfg(test)] mod tests`.
- `xtask/src/main.rs` — the four mount edits named in the Integration contract.
- `xtask/src/affected.rs` — one call appended to the unconditional block, and the module doc at
  `:30` extended to name it.
- `xtask/src/spec_trace.rs` — **visibility only**. `pub(crate)` on the items the audit consumes,
  plus doc comments saying who consumes them. No behaviour change: `cargo xtask spec-trace`'s output
  is identical before and after.
- `spec/audits/` — the first dated report and the frozen baseline, both committed.
- `CHANGELOG.md` — one entry naming the defect the step detects, matching the discipline
  `lint-changelog` already enforces for conformance rules.

**Explicitly not in this PR**

- **Any edit to `spec/SPECIFICATION.md`.** Not a clause, not §1.3, not §7.1/§7.2. If the audit's
  first real run disagrees with §1.3, that is a finding to report — the reconciliation is green in
  the tree today, so a disagreement means the ledger repair or this parser is wrong, and both are
  adjudicated, not silenced.
- **Any edit to `RUNBOOK.md`.** The five ledger rows are `falsifier-ledger-repair`'s and land first.
  This includes the stale `#the-46-provisional-clauses` anchor at `RUNBOOK.md:4495`: it is a real
  defect (`_grounding.md`, *Tensions* 2) and it belongs to whoever owns that section, not to a gate
  step that happens to be reading nearby.
- **The `[DEFERRED]` clauses' dated reasons and the check that requires them** — `deferred-clause-reread`,
  which extends this module. Leave the seam obvious; do not build it.
- **The registry surface diff** — `registry-surface-diff`, the slice-mate mounting into the same list.
- **The census sentence on any README** — `landing-copy-and-status-truth`, which reads this report.
- **Running the audit at the publish commit and committing that report** — `publish-0-2-0`.

**Merge DoD.** `cargo xtask ci --fast` is green with the new step in the Mandatory list; the audit
fails, with a named clause and a path, on each of the seeded fixtures; and the first dated report and
the provenanced baseline are in the tree.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| `cargo xtask clause-audit` — the gate mode | No flags. Runs the three checks against the working tree, prints a summary line naming how many clauses it read, exits non-zero on any problem. Writes nothing. This is the form `REQUIRED` and `affected` invoke. | `xtask/src/main.rs:303-328` (the `spec-trace` step to copy), `:639-700` (dispatch) |
| `cargo xtask clause-audit --write --date <YYYY-MM-DD>` — the artefact mode | Runs the same checks, then writes `spec/audits/clause-maturity-<date>.md`. The date is an **argument, not a clock read**: the release date is a decision, and an explicit date keeps the writer deterministic and its test hermetic. Refuses to overwrite an existing report for that date unless the content is identical. | `xtask/src/spec_trace.rs:209-216` (`Mode::Check`/`Mode::Write` precedent) |
| `cargo xtask clause-audit --rebaseline --from <rev>` | Rewrites `spec/audits/frozen-baseline.tsv` from `spec/SPECIFICATION.md` **as of `<rev>`** (`git show <rev>:spec/SPECIFICATION.md`), stamping the rev and date into the header. Never run by the gate. Fails if the frozen set at `<rev>` differs from the working tree's — that difference is the very thing AC-015 is about, and baselining it would launder it. | `xtask/src/affected.rs` (shelling to `git` is established); `project.md` DR-15 |
| Check 1 — census reconciliation | Calls `spec_trace::check_stated_census` against the same `Census` the parse produced. One implementation, two callers. The report prints both figures — §1.3's stated 200/198/139/49/10/2 and the parsed counts — so the artefact stands alone. A seeded §1.3 disagreement in a fixture document must fail, which is what proves the call is wired rather than described. | `xtask/src/spec_trace.rs:459-506`; `spec/SPECIFICATION.md:219-222` |
| Check 2 — ledger set equality, three-way | Parses `RUNBOOK.md`'s `### The 49 [PROVISIONAL] clauses` table (`:580-606`): the number in the heading, the union of the `Clauses` column with en-dash ranges expanded, and `spec-trace`'s parsed `[PROVISIONAL]` set must all be equal. Failures name the direction and the IDs — *in the ledger, not provisional* vs *provisional, not in the ledger* — because the remedy differs and only a reader can choose it. Only the `Clauses` cell counts; IDs appearing in the `Falsified by` prose are not membership. | `RUNBOOK.md:580-606`, `:618-635`; `xtask/src/spec_trace.rs:437-506` (the "name both numbers" failure style) |
| Check 3 — frozen fingerprints | For every clause the baseline lists: it still parses, it is still `[FROZEN]`, and its normalised body fingerprint is unchanged. A clause that left the frozen set, a changed fingerprint, or a **new** `[FROZEN]` clause absent from the baseline each fail, naming the clause ID and its line. This project promised to change nothing frozen; a new one is a spec change belonging to another project. | `project.md` AC-015 / DR-15; `spec/SPECIFICATION.md:224-235` (CF-25's standing qualification, which the baseline must not be read as discharging) |
| Normalisation and hash | Body text is taken as `parse_clauses` sees it; each line trimmed, internal whitespace runs collapsed, blank lines dropped, lines joined with a single `\n`; then FNV-1a/64, written inline and documented, with a pinned test vector. `DefaultHasher` is the named wrong choice: its values are not stable across Rust releases, so the baseline would expire on a toolchain bump and re-baselining would become routine. | `xtask/Cargo.toml:9-10` (dependencies are `anyhow` only — a hash crate is not on the table) |
| Report contents | Header: the date, the commit (`git rev-parse HEAD`), the baseline's captured-at rev. Body: one row per clause — ID, maturity, and for `[PROVISIONAL]`/`[DEFERRED]` the falsifier's first line — plus the reconciliation block carrying both censuses, the ledger equality result, and the frozen-fingerprint result. The four counts DT-5's census sentence quotes must be readable **as four numbers**, not inferred from 200 rows. | `_design.md`, `### DT-5` (the sentence, its four numbers, and why they are not typed from memory) |
| Fail-closed catalogue | Ledger heading absent or renamed; ledger table empty; baseline file absent, unparseable or with an empty header; report target unwritable; a clause carrying no maturity marker. Each is a failure naming path, line and remedy — never an empty set compared against an empty set. | `.kb/decisions/0010-the-suite-must-prove-itself.md`; `_decomposition.md`, deployment brief *CI implication* |
| `spec-trace` is unchanged in behaviour | The only edit is visibility plus doc comments. `cargo xtask spec-trace` and `cargo xtask spec-trace --write` produce identical output before and after; §7.1/§7.2's generated region is byte-identical. | `xtask/src/spec_trace.rs:23-57` |
| Unit tests, in the established shape | `#[cfg(test)] mod tests` with fixture strings and a named wrong implementation per test, following `xtask/src/package.rs:408-457` and `xtask/src/spec_trace.rs`'s own test blocks. Fixtures required: a seeded §1.3 disagreement; a ledger short by one clause; a ledger carrying a clause that is no longer provisional; a heading whose number disagrees with its rows; an en-dash range; a `Falsified by` cell mentioning a non-member ID; a seeded frozen-clause word change; a frozen clause re-wrapped only (must **not** fire); a frozen clause demoted to `[PROVISIONAL]`; the FNV-1a/64 test vector. Fixtures are synthetic documents, never the real 200-clause file. | `_decomposition.md`, testing brief AC-TEST-002 and *Fixtures and seams to mock* |
| Runtime posture | Two file reads plus at most two `git` invocations. No network, no registry, no clock. It runs in `--fast` and in `affected`, on every story in this project. | `_decomposition.md`, deployment brief *CI implication* |

## Data and migrations

**No database, no schema, no runtime data.** This story adds two **committed text artefacts**, and
they are the closest thing it has to persisted state, so their format and lifecycle are specified
here rather than discovered during implementation.

| Artefact | Path | Lifecycle |
| --- | --- | --- |
| Frozen-clause baseline | `spec/audits/frozen-baseline.tsv` | Written once by `--rebaseline --from <merge-base with main>`, then read-only for the life of the project. Header lines carry `# captured-at: <rev>` and `# captured-on: <YYYY-MM-DD>`; body is one `CLAUSE-ID\t<16 hex>` row per `[FROZEN]` clause, sorted by ID so a diff is readable. Tab-separated and hand-parsed because `xtask` depends on `anyhow` alone (`xtask/Cargo.toml:9-10`) and this file is not worth a serde dependency in the gate's own build graph. |
| Dated audit report | `spec/audits/clause-maturity-<YYYY-MM-DD>.md` | Written by `--write`, one per release, never rewritten. It is a **snapshot, not a mirror**: no check reads it back, so a stale report cannot make the gate green — what the gate checks is the document itself. `publish-0-2-0` writes the next one at the publish commit; that one, not this one, is the release's artefact of record. |

**Migration: N/A, and the reason is not "first release".** There is no prior baseline or report to
migrate *because the check does not exist yet*; the first baseline is a capture, not a conversion.
The one lifecycle event that looks like a migration — regenerating the baseline — is deliberately
made expensive and visible rather than smooth: see `--rebaseline` above and DR-15.

**Rollback.** Removing the step is a revert of the mount edits; nothing else in the tree depends on
the two artefacts at merge time. That changes at `publish-0-2-0`, which is why this story lands
early in the slice rather than beside it.

## Acceptance criteria

Nine criteria. Each is framed from the intent of the person on the other side — the
maintainer at the release gate (backbone **A2**, `_storymap.md`), and behind them Persona 4,
the evaluator who gets one sitting and cannot run this repository's suite
(`_discovery/distillation/personas-and-journeys.md`:249-313). Every criterion is a
GIVEN/WHEN/THEN that crosses the whole stack this story owns: the document, the parser it
composes with, the mount in the gate, and the artefact a downstream story reads.

Tests named `xtask/src/clause_audit.rs::tests::…` are created by this story. `xtask` is a
binary target with a doctest-only `lib.rs` (`xtask/src/lib.rs`:1-16), so these run under
`cargo test -p xtask`, exactly as `xtask/src/package.rs`'s block already does.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** a maintainer who must not have to *remember* to audit the specification, **WHEN** they run `cargo xtask ci --fast` — or `cargo xtask affected --base main` on a commit that touches only `RUNBOOK.md` and no Rust package — **THEN** the clause audit runs anyway, prints how many clauses it read, and exits non-zero on any problem: it is in the **Mandatory** list with `probe: None`, it is in `affected.rs`'s unconditional file-reading block, it is named in `print_help`, and `main.rs`'s *"what the gate proves"* module doc says it is there. *(IQ-1 applied to the gate: the failure is answerable where the reader is standing.)* | `xtask/src/main.rs::tests::clause_audit_is_mandatory_and_probes_for_nothing` (asserts the `REQUIRED` entry exists and carries `probe: None`); observed by running `cargo xtask affected --base HEAD~1` on a `RUNBOOK.md`-only commit and seeing the audit in the file-reading block's output |
| AC-002 | **GIVEN** §1.3's census is the one count in the document a human produced by reading it, and the whole point of it is that the human's count and the machine's count are *independent* (`xtask/src/spec_trace.rs`:39-57), **WHEN** the audit runs against a document whose §1.3 sentence has been seeded to disagree with its own clauses, **THEN** the run fails naming both figures and `spec/SPECIFICATION.md`'s line — and it does so by **calling** `spec_trace::check_stated_census`, not by re-counting, so there is exactly one implementation of the reconciliation and two callers. | `xtask/src/clause_audit.rs::tests::seeded_census_disagreement_fails`; `…::tests::census_check_is_the_spec_trace_one` (a fixture that is green in `spec-trace` is green here and vice versa) |
| AC-003 | **GIVEN** Persona 4 must learn what *provisional* means in the same sentence that uses it, and DT-5's resolution fixes that sentence's four numbers as *"taken from the AC-003 clause audit at the publish commit, not typed from memory"* (`_design.md`, `### DT-5`), **WHEN** the maintainer runs `cargo xtask clause-audit --write --date <YYYY-MM-DD>`, **THEN** `spec/audits/clause-maturity-<date>.md` is written carrying the date, the commit, the baseline's captured-at rev, one row per clause, and a reconciliation block in which **139 / 49 / 10 / 2 are four readable numbers** — not a total a writer must derive by counting 200 rows — and the file is committed. *(IQ-2: the report annotates and never subtracts; the 49 provisional are never suppressed while the 139 frozen are listed. IQ-5: the artefact is reachable without running anything.)* | `xtask/src/clause_audit.rs::tests::report_states_all_four_counts_verbatim`; `…::tests::report_lists_provisional_alongside_frozen`; `…::tests::write_is_deterministic_for_a_given_date`; the committed `spec/audits/clause-maturity-<date>.md` in the diff |
| AC-004 | **GIVEN** the falsifier ledger was reconciled by hand once and agreed only *"at that commit"* (`RUNBOOK.md`:618-620) and has been wrong ever since by its own admission (`RUNBOOK.md`:622-635), **WHEN** the audit reads the repaired ledger, **THEN** it asserts a **three-way** equality — the number in the `### The 49 [PROVISIONAL] clauses` heading, the union of the `Clauses` column, and `spec-trace`'s parsed `[PROVISIONAL]` set — and a ledger short by one clause, a ledger carrying a clause that is no longer provisional, or a heading whose number disagrees with its own rows each fail, naming the direction (*in the ledger, not provisional* vs *provisional, not in the ledger*) and the IDs, because the remedy differs and only a reader can choose it. | `xtask/src/clause_audit.rs::tests::ledger_short_by_one_fails_naming_the_missing_id`; `…::tests::ledger_carrying_a_demoted_clause_fails_naming_the_direction`; `…::tests::heading_number_disagreeing_with_its_rows_fails` |
| AC-005 | **GIVEN** a locator that sweeps a whole ledger row rather than its `Clauses` cell reports a set that is wrong in both directions, and the real table contains both en-dash ranges (`PS-4 – PS-6`) and `Falsified by` prose that mentions clause IDs which are *not* membership, **WHEN** the audit parses the ledger, **THEN** ranges are expanded to their members and IDs appearing outside the `Clauses` cell are not counted — so the check cannot pass by accident on a table it is misreading. | `xtask/src/clause_audit.rs::tests::expands_en_dash_ranges`; `…::tests::falsified_by_prose_is_not_membership` |
| AC-006 | **GIVEN** this project promised to amend nothing `[FROZEN]` (DR-15) and nothing in the tree can currently observe that a frozen clause changed — an edited frozen clause makes the document *more* internally consistent, so `spec-trace` will not see it — **WHEN** the audit runs against `spec/audits/frozen-baseline.tsv`, captured from the merge-base with `main` and stamped with that rev and date, **THEN** a seeded word change inside a frozen clause fails naming the clause ID and its line, a purely cosmetic re-wrap of the same clause does **not** fail, and the fingerprint is value-stable across toolchains because the hash is a documented FNV-1a/64 written in this module with a pinned test vector rather than `DefaultHasher`. *(IQ-4: re-baselining is the reversible act, made expensive and reviewable — `--rebaseline` is never run by the gate, and using it to unblock a release is the stop DR-15 describes.)* | `xtask/src/clause_audit.rs::tests::seeded_frozen_word_change_fails`; `…::tests::rewrapping_a_frozen_clause_does_not_fire`; `…::tests::fnv1a64_pinned_vector`; `…::tests::rebaseline_refuses_when_the_rev_and_the_tree_disagree` |
| AC-007 | **GIVEN** AC-015 is about the frozen *set*, not only about frozen text, **WHEN** a clause the baseline lists has left the frozen set (demoted to `[PROVISIONAL]`), or a clause that is `[FROZEN]` in the tree is absent from the baseline, **THEN** each fails by name — a new frozen clause is a specification change belonging to another project, and a demotion is exactly the laundering DR-15 forbids. | `xtask/src/clause_audit.rs::tests::a_demoted_frozen_clause_fails`; `…::tests::a_new_frozen_clause_absent_from_the_baseline_fails` |
| AC-008 | **GIVEN** ADR-0010's discipline that a skip is reported and never silent (`.kb/decisions/0010-the-suite-must-prove-itself.md`) and that the single most likely wrong implementation of this story is a ledger locator that finds zero rows and reports two equal empty sets, **WHEN** the ledger heading is renamed or absent, the ledger table is empty, the baseline is missing / unparseable / has an empty header, a clause carries no maturity marker this table knows, or the report target cannot be written, **THEN** the run **fails** — never passes — with the path, the line and what the reader is expected to do, and never with a comparison of one empty set against another. *(IQ-2 applied to the instrument itself: an absence is never stated without a reason.)* | `xtask/src/clause_audit.rs::tests::renamed_ledger_heading_fails_not_passes`; `…::tests::empty_ledger_table_is_never_an_equal_empty_set`; `…::tests::absent_baseline_fails_with_the_path`; `…::tests::a_clause_with_no_marker_fails` |
| AC-009 | **GIVEN** a second parser would double the surface on which the document could be misread and would destroy the independence §1.3 exists to prove, **WHEN** this story lands, **THEN** `xtask/src/spec_trace.rs`'s diff is **visibility and doc comments only** — `parse_clauses`, `Clause`, `Census`, `check_stated_census`, `SPEC` and `workspace_root` widened to `pub(crate)` — `cargo xtask spec-trace` and `cargo xtask spec-trace --write` produce identical output before and after, and §7.1/§7.2's generated regions are byte-identical. *(IQ-3: no anchor, heading or generated region a citation resolves to is moved by this change.)* | `cargo xtask spec-trace` output diffed across the change; `git diff` on `xtask/src/spec_trace.rs` shows no expression changed; `cargo xtask ci --fast` green (its `specification traceability` step, `xtask/src/main.rs`:303-328, is the standing check) |

Coverage of the traced project ACs: **AC-003** ← AC-001, AC-002, AC-003, AC-008, AC-009;
**AC-004** ← AC-004, AC-005, AC-008; **AC-015** ← AC-006, AC-007, AC-008.

## Interaction quality

**This story renders none of `_design.md`'s seven surfaces** (Integration contract, *Renders
surfaces*). The signed-off design binds it in one direction only, and it is a real one: DT-5's
census sentence — persistent chrome on the packaged `happenstance` and `happenstance-core`
READMEs, immediately after the status callout — **takes its four numbers from this audit's
report** (`_design.md`, `### DT-5`). So the composition family applies to the *report*, as
the data source that makes that sentence writable or unwritable, and the state family applies
to the gate step's own behaviour. Both are carried as AC rows above; this section says which
row carries which invariant, and how each is checked.

**State invariants.**

| Invariant | AC that carries it | How it is verified |
| --- | --- | --- |
| In place, not a context jump (IQ-1) — a failure is actionable where the maintainer is standing: path, line, and the remedy in the message, never *"see the specification"* | AC-004, AC-008 | The failure-message assertions in `…::tests::ledger_short_by_one_fails_naming_the_missing_id` and `…::tests::absent_baseline_fails_with_the_path` assert on the substring, not merely on `is_err()` |
| Preserved position (IQ-3) — no heading, anchor or generated region a citation resolves to is moved | AC-009 | `cargo xtask spec-trace` output byte-identical across the change |
| Reversibility (IQ-4) — the one reversible act (`--rebaseline`) is deliberately expensive, reviewable as a diff, and never reachable from the gate | AC-006 | `…::tests::rebaseline_refuses_when_the_rev_and_the_tree_disagree`; the absence of any `--rebaseline` invocation in `REQUIRED` (AC-001's mount test) |
| Reachable without running anything (IQ-5) — the maintainer's evidence is a committed file, not console scrollback | AC-003 | The committed `spec/audits/clause-maturity-<date>.md` in the diff |
| Keyboard/command reachability — one documented command, discoverable without reading the source | AC-001 | `print_help` names `clause-audit` and its two flags |

**Composition invariants** (from `_design.md`, applied to the report as DT-5's data source).

| Invariant | AC that carries it | How it is verified |
| --- | --- | --- |
| Presentation exists at all — the report is a composed artefact with a header (date, commit, baseline rev) and named blocks, not a raw dump of parser state | AC-003 | `…::tests::report_states_all_four_counts_verbatim` asserts the header fields and the reconciliation block are present and labelled |
| Density budget, with its real number — DT-5's sentence needs **four** numbers, and the report must yield them as four readable numbers rather than as an inference over 200 rows | AC-003 | `…::tests::report_states_all_four_counts_verbatim` |
| Non-occlusion (IQ-2) — the 49 provisional are never suppressed while the 139 frozen are listed; `_design.md` calls frozen-only *"forbidden, not merely worse"* | AC-003 | `…::tests::report_lists_provisional_alongside_frozen` |
| Non-occlusion applied to the instrument — an absence is never stated without a reason, and never rendered as a silent pass | AC-008 | `…::tests::empty_ledger_table_is_never_an_equal_empty_set` |
| Hierarchy — the per-clause rows are the body and the reconciliation block is the head; a reader looking for the verdict never scrolls 200 rows to find it | AC-003 | Ordering assertion in `…::tests::report_states_all_four_counts_verbatim` |
| Transience — the report is a **snapshot, not a mirror**: no check reads it back, so a stale report can never make the gate green | AC-003 | `…::tests::write_is_deterministic_for_a_given_date`; and the absence of any read of `spec/audits/clause-maturity-*.md` in the check path |
| Named anti-pattern, AP-5 (*no feature matrix, ever* — a matrix implies completeness and a row goes stale invisibly) | AC-003 | The report is a per-clause list plus a census, not a capability matrix; asserted by the shape test above |

**Not applicable, and why.** Non-occlusion of an overlay, preserved focus/scroll/selection, and
opened-on-demand chrome have no referent: there is no viewport, no focus and nothing revealed.
Recording that is the point — `_design.md`'s composition rules bind this story through DT-5's
data dependency, and pretending they bind a console step would be decorative.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | `RUNBOOK.md`'s `### The 49 [PROVISIONAL] clauses` heading is absent, renamed, or its number cannot be parsed | Fail with `RUNBOOK.md` and the search text; state that the locator, not the ledger, may be what is wrong. Never fall through to an empty set (AC-008) |
| EC-002 | The heading is found but the table under it has no parseable rows | Fail. This is the single most dangerous silent success in the story: zero ledger IDs compared against zero parsed IDs is equality (AC-008) |
| EC-003 | `spec/audits/frozen-baseline.tsv` is absent, has an unparseable row, or has an empty/absent `# captured-at:` header | Fail naming the path and the remedy (`--rebaseline --from <rev>`), and state that a baseline with no provenance proves nothing about the tree the project received |
| EC-004 | A clause carries a maturity marker `MATURITY` does not list, or carries none | Fail naming the clause ID and its line. A marker the table has never seen is a document change, not a parser bug to work around |
| EC-005 | `--write` targets a date whose report already exists with **different** content | Fail rather than overwrite; a rewritten dated artefact is a lost audit trail. Identical content is a no-op success |
| EC-006 | `--write`'s target directory does not exist or is unwritable | Fail with the path. Do not create the report elsewhere and do not degrade to printing it |
| EC-007 | `git rev-parse HEAD` or `git show <rev>:spec/SPECIFICATION.md` fails or returns nothing | Fail with the command and its stderr. A report whose header cannot name the commit it describes is worse than no report |
| EC-008 | `--rebaseline --from <rev>` is given a rev whose frozen set differs from the working tree's | Refuse. Baselining a difference launders exactly what AC-015 exists to observe |
| EC-009 | Both `--write` and `--rebaseline` are passed, or a flag is given without its value | Fail with usage, matching `main.rs`'s existing arm-with-flags shape |

## Non-functional

| id | requirement | why it is here |
| --- | --- | --- |
| NF-001 | **No new dependency.** `xtask` depends on `anyhow` alone (`xtask/Cargo.toml`:9-10); the FNV-1a/64 and the TSV parsing are written inline for that reason, and the manifest's own comment records that widening `cargo deny`'s scope for a convenience is a rejected trade | The gate builds its own tool on every run |
| NF-002 | **Deterministic and hermetic.** The date is an argument, never a clock read; unit tests run against synthetic fixture strings, never the real 200-clause document, and require no network. `xtask/src/lints.rs`'s `no_clock` lint is the standing discipline | A gate whose output depends on when it ran cannot be diffed |
| NF-003 | **Hash value-stability is a published property of this repository, not of the toolchain.** The pinned FNV-1a/64 test vector is what makes the baseline outlive a compiler bump; `DefaultHasher` is named in the module doc as the wrong choice and why | A baseline that silently expires makes re-baselining routine, which kills the check |
| NF-004 | **Runtime: two file reads and at most two `git` invocations**, well under a second. It runs in `--fast` and in `affected`, on every story in this project | AC-001 puts it on every gate run; an expensive step gets argued out of the Mandatory list |
| NF-005 | **`spec-trace` has no behaviour delta.** Visibility and doc comments only | AC-009; §7.1/§7.2 are generated and cited by line elsewhere |
| NF-006 | **Failure output is a collected list, not the first problem.** Follow `check_stated_census`'s `problems: &mut Vec<String>` accumulation (`xtask/src/spec_trace.rs`:459-508) so one run tells the maintainer everything that is wrong | Iterating a gate one error at a time is how a gate becomes something people run last |

## Implementation notes (non-prescriptive)

Direction, not instruction. Where the tree already answers a question, the answer is cited rather
than restated.

- **Widen, then call.** The first commit that should exist is the `pub(crate)` widening in
  `spec_trace.rs` plus doc comments naming `clause_audit` as the consumer, with `cargo xtask
  spec-trace` output diffed to prove nothing moved. Everything after that is additive.
- **Accumulate problems; report once.** `check_stated_census` takes `problems: &mut Vec<String>`
  and pushes rather than returning early (`xtask/src/spec_trace.rs`:459-508). Three checks that
  each push into one vector give the maintainer one list, and match the failure prose the gate
  already speaks — *"one of the two is wrong and only a reader can say which"* is the register to
  write in.
- **Fixtures are synthetic documents.** `xtask/src/package.rs`:417-423's `METADATA` constant is
  the shape: one fixture carrying every trap at once, with a doc comment naming each. Note that
  the testing brief's second citation, `spec_trace.rs:1809-1825`, is a **fixture constant**
  (`WIRE_PROBE`, `:1802-1828`) and not a `mod tests` — see *Clarifications*. Both are worth
  copying, for different halves of the job.
- **Parse the ledger cell, not the row.** The `Clauses` column is the membership set; the
  `Falsified by` column is prose. Splitting the row on `|` and indexing the cell is enough, and
  is why AC-005 exists as a separate criterion rather than as a note.
- **Normalisation before hashing** is line-trim → collapse internal whitespace runs → drop blank
  lines → join with `\n`. Write it as a named function with its own test so the re-wrap case
  (AC-006) is provable at the function, not only at the check.
- **The report writer takes `&Census`, `&[Clause]` and the three header values and returns a
  `String`.** Keeping it pure is what makes `…::tests::report_states_all_four_counts_verbatim`
  possible without touching the filesystem.
- **Leave `deferred-clause-reread`'s seam obvious and empty.** That story extends this module
  with a fourth check over the ten `[DEFERRED]` clauses' dated reasons. Do not build it; do not
  build a plugin mechanism for it either.
- **Say what the gate now proves.** The module doc at `xtask/src/main.rs`:8-24 is the one place
  in this repository that states the gate's claims in prose, and the deployment brief requires it
  change in the same commit (AC-DEP-005). One sentence, in the register the surrounding ones use.
- **`CHANGELOG.md` names the defect, not the feature.** `xtask/src/lints.rs`'s
  `changelog_names_every_rule` is the standing discipline for conformance rules; the same form —
  what could have gone wrong and now cannot — is what this entry should carry.

## Tests and CI (merge gate)

Grounded in the testing brief's *Test mix* rows for AC-003, AC-004 and AC-015, its
AC-TEST-002 obligation, and its *Fixtures and seams to mock* section
(`_decomposition.md`, `## Testing brief`).

| Tier | Command / path | Proves |
| --- | --- | --- |
| unit | `cargo test -p xtask` → `xtask/src/clause_audit.rs::tests` | Every AC-002…AC-008 assertion above. Each test carries a doc comment naming the wrong implementation it rejects, per `xtask/src/package.rs`:408-457 and AC-TEST-002 |
| unit (mount) | `cargo test -p xtask` → `xtask/src/main.rs::tests::clause_audit_is_mandatory_and_probes_for_nothing` | AC-001's first half: the step is in `REQUIRED` with `probe: None`, so it can never be skipped for a missing tool |
| static (gate step) | `cargo xtask clause-audit` | The three checks against the real tree — the assertion that the repository is, today, in the state the audit describes |
| story grain | `cargo xtask affected --base main` (wired at `.redkiln/config.yaml`) | AC-001's second half: a diff touching only `RUNBOOK.md` or `spec/SPECIFICATION.md` still runs the audit, because it joins the unconditional block at `xtask/src/affected.rs`:116-125 |
| integration grain | `cargo xtask ci --fast` | The whole non-terminal bar with the new step inside it, plus AC-009's `specification traceability` step proving `spec-trace` is unchanged |
| regression (manual, once) | `cargo xtask spec-trace` before and after, diffed; `cargo xtask spec-trace --write` producing no change | AC-009: visibility-only edit, §7.1/§7.2 byte-identical |
| artefact | `cargo xtask clause-audit --write --date <YYYY-MM-DD>`; the committed `spec/audits/` files | AC-003 and project DoD 3 — a dated committed artefact rather than a remembered observation |

**Not tested here, deliberately.** No `EventStore` fixture and no `event_store_conformance!`
invocation: this project ships no adapter, and the testing brief cites the rule only to say it
does not apply (`_decomposition.md`, *Fixtures and seams to mock*). No network and no registry
round-trip — that is `registry-surface-diff`'s problem, and its baseline is the thing that can be
unreachable, not this story's two local files.

## Risks and coupling (PR-scoped)

| Risk | Likelihood / Impact | Mitigation, in this PR |
| --- | --- | --- |
| The ledger locator finds nothing and the check passes on two empty sets | **High** / High — it is the default behaviour of the obvious implementation | EC-001, EC-002 and AC-008 make it a fixture that must fail; the fixture is a renamed heading, which is the realistic way it happens |
| A second clause parser is written because widening visibility feels invasive | Medium / High — it silently destroys §1.3's independence | AC-009 makes it observable in the diff; the Context pack states the mount is the widening and the failure is the copy |
| The frozen fingerprint fires on a cosmetic re-wrap, the owner re-baselines, and the check dies | Medium / High | Normalisation is specified, and AC-006 carries a re-wrap fixture that must **not** fire, alongside the word change that must |
| The baseline is captured from `HEAD` instead of the merge-base, so it certifies the tree this project already changed | Medium / High | The baseline header records the rev; EC-008 refuses a rev whose frozen set disagrees with the tree; the reviewer can read the provenance in the file |
| The audit's first real run disagrees with §1.3, and §1.3 is edited to make it green | Low / **High** | `spec/SPECIFICATION.md` is outside the PR boundary. The reconciliation is green in the tree today, so a disagreement means the ledger repair or this parser is wrong — both adjudicated, neither silenced |
| A `[FROZEN]` clause is found wrong and quietly amended to keep the release date | Low / High | DR-15 and project AC-015: it is a stop, a new decision atom and a re-plan. `spec/SPECIFICATION.md` being outside the PR boundary makes the attempt visible in `redkiln verify --grain story` |
| `falsifier-ledger-repair` has not landed, and the ledger check is written against the short table | Medium / Medium | The dependency is hard (`_storymap.md`, *Two ordering constraints that must not be reordered*). If the repair is in flight the fixtures are still authorable — they are synthetic — but the real-tree run must wait |
| The step is added to the Optional list because "it might be flaky" | Low / High | AC-001, AC-DEP-005, and `xtask/src/main.rs`:33-38's own definition: optional means *the probed tool is absent*, never *the check would fail* |

**Coupling.** Outbound: `deferred-clause-reread` extends this module; `publish-0-2-0` runs it at
the publish commit and commits that report; `landing-copy-and-status-truth` reads DT-5's four
numbers out of a report of this shape. Inbound: `falsifier-ledger-repair`'s five edits are what
the ledger check reads. Sideways: `registry-surface-diff` mounts into the same `REQUIRED` array
and the same `main.rs` module doc — which is precisely why the story map put both in one slice
(`_storymap.md`, *Why the milestones are cut where they are*).

## Dependencies

**Blocks on**

- `falsifier-ledger-repair` — **hard, and not merely an ordering preference.** DR-4 is a
  separate and earlier obligation than DR-3 so that a falsifier is never chosen to make an audit
  pass (`RUNBOOK.md`:625-628, *"not a thing to do in passing"*; risk register row 1 in
  `project.md`). The ledger check in AC-004 is meaningless against a table known to be short by
  five rows.

**Unlocks**

- `deferred-clause-reread` — extends this module with the fourth check over the ten `[DEFERRED]`
  clauses' dated reasons; the story map orders it strictly after this one so the reasons and the
  check that requires them land together and the gate is never red between them.
- `publish-0-2-0` — lists `clause-maturity-audit` among its blockers; it runs the whole gate,
  including this step, on the literal publish commit.
- `landing-copy-and-status-truth` — a **data** dependency rather than a merge-order one: DT-5's
  census sentence takes its four numbers from a report of this shape.

## Anchors (progressive disclosure)

Open these when the row says to, not before. Every path was confirmed to exist.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `xtask/src/spec_trace.rs`:39-57 | The module doc that forbids generating §1.3 and explains why the human count and the machine count must stay independent. It predicts and rejects the first instinct this story will produce | Before writing a line of `clause_audit.rs` | AC-002, AC-009 |
| `xtask/src/spec_trace.rs`:459-508 | `check_stated_census` itself — the function AC-002 must call rather than re-implement, and the model for accumulating problems and for the register the failure prose is written in | While wiring check 1 | AC-002 |
| `xtask/src/main.rs`:33-38, :105-120, :303-328 | The Mandatory/Optional definition, the head of `REQUIRED`, and the `spec-trace` step to copy — a `Step` with `probe: None` and a comment naming the defect it exists against | While adding the mount | AC-001 |
| `xtask/src/affected.rs`:105-126 | The unconditional file-reading block and the comment explaining why it runs regardless of which packages the diff touched — the second mount, without which a `RUNBOOK.md`-only story is ungated | While adding the second mount | AC-001 |
| `xtask/src/package.rs`:408-457 | The `#[cfg(test)] mod tests` shape AC-TEST-002 requires: one fixture carrying every trap, a doc comment per test naming the wrong implementation, and an assertion that the wrong implementation is rejected | Before writing the test module | AC-008 |
| `xtask/src/package.rs`:4-18 | Why containment is not presentation. The argument this story inherits when it decides the report must state four numbers rather than let a reader derive them | While designing the report's reconciliation block | AC-003 |
| `RUNBOOK.md`:580-635 | The ledger itself: the heading, the `Clauses` and `Falsified by` columns, the en-dash ranges, the methodology note that it agreed only *"at that commit"*, and the five known-short rows in the runbook's own words | Before implementing check 2 | AC-005 |
| `spec/SPECIFICATION.md`:213-235 | §1.3's census sentence — the literal 200 / 198 / 139 / 49 / 10 / 2 — the empty-falsifier prohibition, and CF-25's standing qualification on every frozen port clause, which a fingerprint baseline must not be read as discharging | While implementing checks 1 and 3 | AC-006 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` | The signed-off `### DT-5` resolution (`:284-318`): the census sentence, its four numbers, and the decision that those numbers come from this report at the publish commit rather than from memory — the reason the report's shape is an acceptance criterion and not a preference | Before writing the report writer | AC-003 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` | The testing brief's `### Test mix` rows (the wrong implementation each instrument must reject), `### Fixtures and seams to mock` (synthetic strings, never the real 200-clause file), and the deployment brief's `### CI implication` with AC-DEP-005's Mandatory-never-behind-a-probe rule and the same-change module-doc update | While planning the test module and the mount | AC-001 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | The accepted decision behind *fail closed and say why*: a declined capability still runs the rule and reports its stated reason rather than vanishing. It is the authority for EC-001…EC-004 being failures rather than skips | When tempted to let a missing input pass quietly | AC-008 |
| `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md` | The open question this audit discharges one instance of. Read it to be sure this story does **not** resolve it in passing — that is AC-013 decision work in a sibling story | If the temptation arises to declare the question closed in the report or the changelog | AC-003 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/project.md` | DR-15, project AC-015 and the risk register row that makes a wrong frozen clause a **stop**: a new decision atom and a re-plan, never an edit inside this project | The moment check 3 fails on the real tree | AC-007 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md` | *Merge order* and *Two ordering constraints that must not be reordered*: why `falsifier-ledger-repair` strictly precedes this story, stated as a constraint a later re-plan may not undo | Before starting, to confirm the dependency has landed | AC-004 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | Persona 4, the evaluator (`:249-313`): one sitting, cannot run this suite, judges the crate by what it publishes. The reason a gate step with no user of its own still carries an acceptance criterion about a report's readability | When deciding how much the report may assume of its reader | AC-003 |

## Clarifications resolved during spec

1. **The testing brief's second `mod tests` precedent does not exist.**
   `_decomposition.md`'s AC-TEST-002 cites `spec_trace.rs:1809-1825` as a second example of the
   `#[cfg(test)] mod tests` shape. `xtask/src/spec_trace.rs` contains **no** `mod tests` and no
   `#[test]` function; those lines are inside `WIRE_PROBE` (`:1802-1828`), a fixture *string
   constant* whose doc comment names the traps it carries. Resolved as: `xtask/src/package.rs`:408-457
   is the sole precedent for the test-module shape, and `WIRE_PROBE` is the precedent for the
   fixture-string discipline. Both are cited above for their real halves. The brief's obligation
   is unchanged; only its citation was loose.
2. **The reconciliation in project AC-003 is not new enforcement.** `check_stated_census`
   already exists and already fails a §1.3 disagreement. This story's contribution is to *call*
   it from the audit and to put its result in a committed artefact. Recorded here so no one
   re-implements it believing the tree lacks it — and so AC-002 is written as *"calls it, and a
   seeded disagreement fails"* rather than as *"reconciles"*, which the tree already satisfies.
3. **AC ids are exactly the nine the first pass enumerated** — AC-001…AC-009 — none added, none
   dropped. The two frozen-clause criteria stayed split (AC-006 text, AC-007 set membership)
   because their fixtures and their failure messages differ; the fail-closed catalogue stayed one
   criterion (AC-008) because it is one property with several inputs.
4. **The composition family binds this story through data, not through a render.** The default
   assumption is a story that renders a surface; this one does not. Rather than declare the
   composition invariants inapplicable, they are applied to the report as DT-5's data source —
   density (four numbers), non-occlusion (provisional never suppressed), hierarchy (verdict
   before the 200 rows) — and each is an AC row, not a prose bullet.
5. **The hash is written here rather than depended on.** Given `xtask/Cargo.toml`'s deliberate
   single dependency and its comment on why the driver crates are absent, adding a hash crate to
   the gate's own build graph was not a live option. `DefaultHasher` was rejected on
   value-stability, which is a correctness property of the baseline and not a preference — hence
   NF-003 and a pinned test vector rather than a note.
6. **The date is an argument, not a clock read.** `xtask/src/lints.rs` carries a `no_clock` lint
   for the library; the same discipline is applied here for a different reason — a release date
   is a decision, and an argument keeps the writer's test hermetic (NF-002).
