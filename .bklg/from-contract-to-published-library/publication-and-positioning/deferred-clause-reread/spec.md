---
item: HS-S0090
stage: spec
created: 2026-08-12T13:47:28.320Z
updated: 2026-08-12T13:47:28.320Z
template_sig: 87bbf1d0
rendered_sig: b59e8874
---

# Spec — Every deferral is re-read at publish and says why, dated

## Scope lock

| | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DoD 12; BR-06 |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/publication-and-positioning/project.md` — AC-005, DR-5, DR-15 |
| This spec | `.bklg/from-contract-to-published-library/publication-and-positioning/deferred-clause-reread/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` — *Testing brief* (the AC-005 row of the per-AC instrument table, and AC-TEST-002) and *Deployment brief* (AC-DEP-005: mandatory step, no probe, module doc updated in the same change) |
| Signed-off design | `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` — **renders no surface this story touches**; binding only in that nothing here may add a public item (`## Items` carries exactly two entries and neither is this story's) |
| Story map / slice | `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md`:62, 79-86, 161-165, 179-184 |
| Roadmap pointer | `RUNBOOK.md`:163 — phase 12's row and its proof artefact |

## One-line PR slice

Re-read all ten `[DEFERRED]` clauses for whether the deferral is still honest at publish, give each a
dated, consumer-readable reason recorded in the clause itself, and extend the clause audit in the same
change so an inherited (undated, unchanged) reason fails the gate — the *"a skip is reported, never
silent"* discipline of `.kb/decisions/0010-the-suite-must-prove-itself.md`:20-21 applied to prose.

## Executive summary

**Pointer.** `spec/SPECIFICATION.md`:204-207 defines `[DEFERRED — <the experiment that settles it, and
its owning phase>]`, and `xtask/src/spec_trace.rs`:650-679 (check 2) already fails the gate when that
marker's text is shorter than twelve characters — CF-38. Ten clauses carry the marker: **WF-1, ES-39,
PS-33, SY-14, SY-18, SY-27, SY-28, SY-32, CF-14, CF-27** (`spec/SPECIFICATION.md`:8511-8519's census
row, and the `DEFERRED` rows of §7.2 at `:8566`, `:8621`, `:8662`, `:8685`, `:8689`, `:8698`, `:8699`,
`:8703`, `:8725`, `:8738`).

**Delta.** CF-38 checks that a deferral *named an experiment once*. It cannot see whether that deferral
is still honest at the moment the version becomes a promise, and today nothing can: a marker written at
phase 2 reads identically at phase 12, and there is no field for the answer to a question nobody has
asked yet. This PR adds that field and the check behind it.

Three things land together, and they are one change because none of them is safe alone:

1. **Ten `Re-read:` records**, one per deferred clause, each carrying the version being published, an
   ISO date, and a reason *a consumer* can read for why the deferral is still the right answer — not
   the experiment, which the marker already carries.
2. **One new field the parser knows.** `field_head`'s allowlist at `xtask/src/spec_trace.rs`:1541-1555
   is closed (`["Rule", "Cases", "Rejects", "Retires"]`). `Re-read` joins it, which is what makes the
   field visible *and* what makes a `Re-read:` line terminate the field above it instead of being
   swallowed into it.
3. **The deferral half of the publish-time clause audit**, mounted in the mandatory step
   `clause-maturity-audit` registers, that fails on a record that is missing, mis-stamped, or a restatement
   of the marker it sits under.

The instrument's teeth are in the version stamp: at `0.3.0` all ten records go red until someone reads
them again. That is the only mechanical form "re-read rather than inherited" can take, and it is stated
here with the axis it cannot check (whether the sentence is *true*) named rather than implied.

## Context pack

**This story does not decide any deferral's outcome.** It records, for each of the ten, why the
deferral is still honest *at this publish*, and it makes the absence of that record a build failure.
Resolving a deferred clause here would be settling a question in passing, which `project.md`'s risk
table forbids and `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md` exists to prevent.
Any clause whose re-read concludes the deferral is **no longer** honest is a stop: it becomes a decision
atom and a re-plan (`project.md` DR-15 / AC-015), not an edit made here.

**The reason is a different field from the marker, and folding them is the failure mode.**
`spec/SPECIFICATION.md`:204-207 says the marker carries *the experiment that settles it, and its owning
phase*. The re-read answers a different question — *is deferring still right for a consumer who is about
to depend on this version*. If the reason is appended into the marker, CF-38's ≥12-character check is
satisfied by the experiment text that was already there, the new obligation becomes invisible, and the
gate reports ten green clauses it never read. WF-1 is the live warning: its marker already ends *"(ADR-0016
§1, reading of 2026-08-05)"* (`spec/SPECIFICATION.md`:1899-1908), a dated reading that predates this
release by two phases and would pass any naive "does it mention a date" check.

**The version stamp is the mechanism; the date is for the reader.** The record's version is compared
against `Cargo.toml`:5-6's `workspace.package.version` (`0.2.0` today). That comparison is the whole of
"re-read at this publish rather than inherited": it fails automatically at the next release, for all ten
at once, and no rewording can talk it out of that. The date is checked for *shape* only. The audit does
**not** read the clock — a gate that consults `now()` is non-deterministic against a tree, and CF-33's
prohibition on a rule reading a clock (`xtask/src/main.rs`:361) is the house instinct even though this
is `xtask` rather than a conformance rule.

**The parser edit is ordered before the prose edit, and getting it backwards produces a confusing
failure.** `field_line` (`xtask/src/spec_trace.rs`:1499-1537) reads a field's continuation lines until it
meets a blank line, a `#`, a `**`, or *another known field*. Before `Re-read` is in the allowlist, a
`Re-read:` line placed under a `Rule:` line is continuation: its text joins the rule value, its
backticked identifiers are parsed as rule names by `rules_of`, check 4 reports rules that do not exist,
and §7.2's generated cell changes so check 9's equality fails. Every one of those is a true failure
pointing at the wrong cause. Add the field to `field_head` first.

**Nothing in the generated region may move.** `spec-trace` regenerates §7.1 and §7.2 between the markers
at `spec/SPECIFICATION.md`:8507 and compares them against what is committed
(`xtask/src/spec_trace.rs`:23-38 — *"the load-bearing half is the equality check the gate runs without
the flag"*). A `Re-read:` field is not a column in either. `cargo run -p xtask -- spec-trace --write`
must produce **no diff** after this change; a diff means the field is leaking into `Rule:` or `Cases:`.

**§1.3 stays hand-computed.** `xtask/src/spec_trace.rs`:39-57 is explicit that generating §1.3 would
destroy the property it is used to prove. This story reconciles its ten records against §1.3's stated
**10** (`spec/SPECIFICATION.md`:219-222) and against the parser's own `DEFERRED` count; it does not
rewrite the sentence.

**A check that cannot fail is decorative, and the tree already shows both ways of avoiding that.**
`xtask/src/package.rs`:408-457 is the `#[cfg(test)] mod tests` precedent the testing brief names — a
fixture string, a named wrong shape, an assertion that it is rejected. The stronger one is
`spec_trace`'s own **run-time probe**: `RETIRES_PROBE` (`:954-967`, checked at `:849-868`) and
`WIRE_PROBE` (`:1802-1848`) are fixtures parsed on *every gate run*, so a parser that quietly stops
recognising a form fails the gate instead of reporting silence. This story owes both, and the probe is
the one that matters — a unit test proves the parser worked when `cargo test` ran, the probe proves it
worked on the tree being published. (Note for the implementer: the testing brief cites
`spec_trace.rs:1809-1825` as a `#[cfg(test)]` precedent. It is not one — `spec_trace.rs` has no test
module; those lines are `WIRE_PROBE`. The precedent is real, its kind is different, and this spec uses
both kinds deliberately.)

**The slice is implemented in one context and mounts once.** `_storymap.md`:79-86 is explicit that the
two instrument stories share `xtask/src/main.rs`'s `REQUIRED` list and its module doc at `:8-24`, and
that two contexts editing that list is a merge conflict by construction. `clause-maturity-audit`
(HS-S0089) registers the mandatory step; this story adds the deferral checks *inside* it and extends the
same doc paragraph in the same change. AC-DEP-005 forbids putting it behind a `probe` — there is no
external tool to probe for, and `Some(probe)` on a step with no tool converts "the check found a
problem" into "skipped" (`xtask/src/main.rs`:89-102).

**Ordering that must not be reordered.** `falsifier-ledger-repair` → `clause-maturity-audit` → this
story (`_storymap.md`:161-165, 179-184). The reason this one comes last in the slice is that the ten
reasons and the check requiring them land in a single commit — a tree where the check exists and the
records do not is a red gate on `main`.

**Who is on the other side.** Backbone activity A2, *prove the promises before making them*
(`_storymap.md`:41). There is no rendered surface here: the reader served is the maintainer at the
release gate, and — one hop behind DT-5's census sentence — the evaluator who is told 10 clauses are
deferred and can reach what each deferral now says. The census sentence is
`landing-copy-and-status-truth`'s to write; the truth of its fourth number is this story's to keep.

## Integration contract

- **Archetype**: `capability` — the observable outcome is a gate that fails a dishonest tree, and a
  specification a consumer can read a dated reason out of. It is not substrate for a later story.
- **Slice / milestone**: `publish-time-gate-instruments`. Slice-mates: `falsifier-ledger-repair`,
  `clause-maturity-audit`, `registry-surface-diff` — implemented together in one context and mounted as
  one integrated gate surface.
- **Mount point**: **`xtask/src/main.rs`** — the `REQUIRED` step list (`:105`ff) and the module doc at
  `:8-24` that states in prose what the gate proves. This story's checks run inside the mandatory
  clause-audit step `clause-maturity-audit` adds to that list; the doc paragraph gains the sentence that
  the gate now proves every deferral was re-read *at the published version*. No new `Step` entry is
  added by this story, and none is added behind a `probe` by the slice (AC-DEP-005).
- **Wires into**:
  - `xtask/src/spec_trace.rs` — `parse_clauses` (`:1357-1389`), the `Clause` struct (`:217-239`),
    `maturity_of`/`falsifier_of` (`:1432-1489`), `field_line`/`field_head` (`:1499-1555`). The audit
    *composes with* this parser and does not fork a second one: two parsers of one document is how the
    census and the table came to disagree, which is the defect `spec_trace`'s module doc was written
    against.
  - `spec/SPECIFICATION.md` — the ten deferred clauses' bodies (field added, normative text untouched),
    §1.3's census at `:219-222` (read, never written), and the generated region opened at `:8507`
    (must not move).
  - `Cargo.toml`:5-6 — `workspace.package.version`, the value the stamp is checked against.
  - The clause-audit module `clause-maturity-audit` adds under `xtask/src/` — its report is where the
    ten records are printed, and its `#[cfg(test)] mod tests` is where this story's fixtures join.
- **Renders surfaces**: **none.** `_design.md`'s seven surface ids are the three crates.io pages, the
  three docs.rs pages and the GitHub landing page; this story renders none of them and adds no public
  item — `_design.md`'s `## Items` carries two entries and neither belongs here. Its one indirect
  obligation is to keep DT-5's census sentence true (`_design.md`:289-296, the *10 deferred* count and
  *"each names the experiment and the phase that owns it"*), which this story strengthens rather than
  changes.
- **Public items**: none. No `pub` item is added, changed or removed in any published crate; `xtask` is
  not published (`xtask/src/package.rs`'s `PUBLISHABLE` set is the three library crates).
- **Conformance rule(s)**: **none, and this is not adapter-observable.** Nothing here changes a port, a
  value type or an adapter obligation — the subject is the specification's own prose and the gate that
  reads it. The equivalent of a conformance rule for this story is the run-time probe in `xtask`, which
  is why AC-008 exists rather than a `suite.rs` rule. `happenstance_testkit::fixtures::MemoryFixture`
  and `event_store_conformance!` are cited only to say they do not apply (testing brief, *Fixtures and
  seams to mock*).
- **Clause(s)**: **discharges none and amends none.** The ten deferred clauses gain a field beside
  `Rule:`/`Cases:`/`Rejects:`; their normative sentences, their markers and their maturity are byte-stable.
  No `[FROZEN]` clause is touched — AC-015 is an observable property of this diff. CF-38's existing check
  (`spec/SPECIFICATION.md`:213-217) keeps passing unchanged, and this work is the standing obligation
  that section describes rather than a new clause.
- **Advances DoD scenario**: **initiative DoD 12** — *"The clause ledger is audited at publish"*
  (`initiative.md`:395-397). `clause-maturity-audit` moves the maturity-and-falsifier half; this story
  moves the half DoD 12's sentence implies and no instrument covered — that a deferral's stated reason
  was read at this publish rather than carried over.

## PR boundary

```
spec/SPECIFICATION.md
xtask/src/spec_trace.rs
xtask/src/main.rs
xtask/src/*.rs
.bklg/from-contract-to-published-library/publication-and-positioning/deferred-clause-reread/**
```

`xtask/src/*.rs` is deliberately one level wider than this story alone needs: the clause-audit module
this story extends is created by its slice-mate in the same context, and its filename is that story's
call. Nothing outside `xtask/src/` and `spec/SPECIFICATION.md` is in bounds.

**In this PR**

- A `Re-read:` record on each of the ten `[DEFERRED]` clauses: the version, an ISO date, and the reason
  the deferral is still honest, written in the caller's register.
- `Re-read` added to `field_head`'s allowlist (`xtask/src/spec_trace.rs`:1541-1555), with the doc
  comment stating why the list is closed and what adding an entry does to continuation parsing.
- The deferral checks, inside the mandatory clause-audit step: presence, set equality against the parsed
  `DEFERRED` set and §1.3's stated 10, version stamp equals the workspace version, ISO date shape, and
  the reason is not a restatement of the marker.
- A run-time `DEFERRAL_PROBE` in the `RETIRES_PROBE`/`WIRE_PROBE` shape, plus `#[cfg(test)] mod tests`
  cases each naming the wrong tree it rejects.
- The module-doc sentence at `xtask/src/main.rs`:8-24, and the audit's own module doc stating the axis
  it does **not** check.
- The ten records echoed into the dated clause-audit report the slice commits.

**Explicitly not in this PR**

- Resolving, narrowing or re-scoping any deferred clause; changing any marker text; touching any
  `[FROZEN]` or `[PROVISIONAL]` clause. A re-read that concludes "no longer honest" halts and re-plans.
- `RUNBOOK.md`'s falsifier ledger and its clause-ID set equality — `falsifier-ledger-repair` and
  `clause-maturity-audit` own those; the ledger is about `[PROVISIONAL]` clauses, not deferred ones.
- The §1.3 census sentence, the §7.1/§7.2 generated region, and the maturity/frozen-fingerprint checks —
  `clause-maturity-audit`'s.
- Any published README or rustdoc copy, including DT-5's census sentence — `published-surface-copy`'s.
- Any decision atom or `.kb/_intake/` staging. This story settles nothing that needs one; AC-013's atoms
  belong to the `release-decisions` milestone.
- The release event itself, and any version-number choice — `registry-surface-diff` and `publish-0-2-0`.

**Merge DoD (one line):** `cargo xtask affected --base main` is green, `cargo run -p xtask -- spec-trace
--write` leaves the tree unchanged, and the mandatory clause-audit step passes on a tree where all ten
records are present and stamped `0.2.0` — and fails, by name, on each of the four seeded wrong trees.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The record's shape** | Each deferred clause carries one field line: `Re-read: <version>, <YYYY-MM-DD> — <reason>`. It sits with that clause's other fields, in that clause's own local spelling (`Rule:` bare, `- **Rule:**`, or `` `Rule:` `` — all three are in use and `field_head` strips the punctuation). The reason is prose for a caller: what depending on this version costs while the question is open, not what would settle it | `spec/SPECIFICATION.md`:204-207 (what the marker carries), `:1899-1908` (WF-1's dated reading, inside the marker, which is the shape *not* to reuse); `xtask/src/spec_trace.rs`:1499-1554 |
| **The parser learns one field** | `"Re-read"` joins `field_head`'s allowlist. Two consequences, both wanted: `field_line(body, "Re-read")` starts returning a value, and a `Re-read:` line now terminates the preceding field's continuation instead of being absorbed into it. The `Clause` struct gains the parsed record (or the audit calls `field_line` directly — either is in bounds; forking a second parser is not) | `xtask/src/spec_trace.rs`:1541-1555, `:1499-1537`, `:217-239` |
| **The ten are exactly the ten** | The set of clauses carrying `Re-read:` equals the set the parser marks `DEFERRED`, and both equal §1.3's stated **10**. A record on a non-deferred clause fails; a deferred clause without one fails; an eleventh deferral added later fails until it is re-read | `spec/SPECIFICATION.md`:219-222, `:8511-8519`; clauses WF-1, ES-39, PS-33, SY-14, SY-18, SY-27, SY-28, SY-32, CF-14, CF-27 |
| **Inherited fails, mechanically** | The record's version must equal `workspace.package.version`. Nothing else in the check depends on human honesty: at the next release every record is stale by construction and all ten fail until re-read | `Cargo.toml`:5-6; `project.md` AC-005 (*"re-read at this publish rather than inherited"*) |
| **The date is shape-checked, never clocked** | `YYYY-MM-DD`, parsed for well-formedness and rejected otherwise. The audit reads no clock: the gate's verdict is a function of the tree alone, so two runs on one commit cannot disagree | `xtask/src/main.rs`:361 (CF-33's clock prohibition, the house instinct); `xtask/src/spec_trace.rs`:23-38 (equality-against-the-tree as the load-bearing property) |
| **The reason is not the marker wearing a date** | The record's prose, normalised (whitespace collapsed, punctuation and backticks stripped, case-folded), must not equal or be contained in the marker text `falsifier_of` returns for the same clause, and must clear a stated minimum length. This is the check that rejects the cheapest fake re-read: pasting the experiment across | `xtask/src/spec_trace.rs`:1475-1489 (`falsifier_of`), `:650-679` (CF-38's own ≥12 test, the check this one sits beside and does not duplicate) |
| **The gate reports, it does not merely pass** | The audit prints all ten — clause id, date, and the reason's first line — into the dated report the slice commits, so a re-read is *reported, never silent*. A green run that printed nothing would be indistinguishable from a check that found no deferrals | `.kb/decisions/0010-the-suite-must-prove-itself.md`:20-21, `:71-72`; `project.md` DoD 3 (the report is a committed dated artefact) |
| **Mandatory, never probed** | The checks run inside the clause-audit step in `REQUIRED`. No `probe:` — there is no external tool, and a probe would turn "found a problem" into "skipped" | `xtask/src/main.rs`:89-102, `:105`ff; `_decomposition.md` AC-DEP-005 |
| **The gate's own prose is updated in the same change** | `xtask/src/main.rs`:8-24 states what the gate proves; it gains the deferral sentence in this PR, not a later one. The audit's module doc states what it does **not** verify — that a reason is *true* — following the five lints' standing discipline | `xtask/src/main.rs`:26-42 (*"a check whose limits are undocumented is read as a guarantee"*); `standards/rust/70-rustdoc-obligations.md` |
| **The check can fail, twice over** | A `DEFERRAL_PROBE` fixture clause parsed on every run (so a parser that stops recognising the field fails the gate, not the tree), and `#[cfg(test)] mod tests` cases over synthetic clause strings — never the real 200-clause document | `xtask/src/spec_trace.rs`:849-868, `:954-967`, `:1802-1848`; `xtask/src/package.rs`:408-457; `_decomposition.md` AC-TEST-002 |
| **The generated region is byte-stable** | After the change, `cargo run -p xtask -- spec-trace --write` produces no diff and check 9's equality passes. A moved cell means the field leaked into `Rule:` or `Cases:` | `xtask/src/spec_trace.rs`:741 (check 9), `spec/SPECIFICATION.md`:8507 |
| **Nothing normative moves** | The ten clauses' sentences, markers, `Rule:`, `Cases:` and `Rejects:` fields are unchanged; no `[FROZEN]` clause is in the diff. `git diff` over `spec/SPECIFICATION.md` shows added `Re-read:` lines and nothing else | `project.md` DR-15 / AC-015; `_storymap.md`:186-191 (where the release blocks) |
| **A dishonest deferral halts the story** | If a re-read concludes a deferral is no longer honest — the experiment has been run, or the question has been answered elsewhere — the implementer stops and reports. It becomes a decision atom and a re-plan; it is not resolved inside this PR | `project.md` *Out of scope* and the risk table; `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md` |

## Data and migrations

**N/A — no schema, no store, no persisted state.** This story adds no table, column, index or encoded
value; it ships no library code at all. `xtask` is a build-time binary, and the only artifacts it reads
are `spec/SPECIFICATION.md` and `Cargo.toml`.

The one thing with migration-shaped risk is the **document format**, and it is stated here so it is not
discovered later: adding `Re-read` to `field_head`'s closed allowlist changes how *every* clause body in
`spec/SPECIFICATION.md` is parsed, not only the ten. Any pre-existing line anywhere in the document that
begins `Re-read:` (after the leading `-`, `*`, `` ` `` and whitespace `field_head` strips) would become a
field terminator where it used to be continuation text. There are none today — `Re-read` appears nowhere
in the tree — so the change is inert outside the ten clauses, and `spec-trace`'s §7.1/§7.2 equality check
is the standing instrument that would report it if that ever stopped being true.

`_decomposition.md`'s deployment brief records the project-level migration posture as N/A with its
reason (AC-DEP-004: first publish, no prior compatible version); nothing in this story changes it.

## Acceptance criteria

Every criterion is written from the intent of a real reader crossing the whole stack — the **evaluator**
deciding in one sitting (`_discovery/distillation/personas-and-journeys.md`, Persona 4) and the
**maintainer standing at the release gate**, who is the reader `xtask`'s output exists for. "Deferred"
is a word an evaluator meets on the published page and follows into the specification; the gate is what
keeps the word honest between those two places.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an evaluator who has read the published census sentence and been told ten clauses are deferred, **WHEN** they open any one of WF-1, ES-39, PS-33, SY-14, SY-18, SY-27, SY-28, SY-32, CF-14, CF-27 in `spec/SPECIFICATION.md`, **THEN** they read — at that clause, without following a link or opening a second file — a single `Re-read: <version>, <YYYY-MM-DD> — <reason>` line whose reason is written in the caller's register (what depending on `0.2.0` costs them while the question is open), distinct from the marker's experiment-and-phase, and **all ten** clauses carry one | Unit: `xtask/src/clause_audit.rs::tests::rejects_deferred_clause_with_no_reread` over a synthetic two-clause fixture in which one `[DEFERRED]` clause has no record. Real tree: the mandatory clause-audit step in `cargo xtask ci` prints ten records and exits 0; a grep for `Re-read:` over `spec/SPECIFICATION.md` returns exactly ten lines |
| AC-002 | **GIVEN** a maintainer who adds the record to a clause that already has `Rule:` and `Cases:` lines, **WHEN** `spec-trace` parses the document, **THEN** `field_line(body, "Re-read")` returns the record *and* the `Re-read:` line terminates the field above it instead of being absorbed as continuation — so `rules_of` never sees the reason's backticked identifiers, check 4 reports no non-existent rule, and no `Rule:`/`Cases:` value gains a character | Unit: `xtask/src/clause_audit.rs::tests::reread_line_terminates_the_field_above_it` — a fixture clause with `Rule: …` immediately followed by `Re-read: …`, asserting the parsed `Rule` value is byte-identical to the same fixture with the `Re-read` line absent. Run-time: `DEFERRAL_PROBE` in `xtask/src/spec_trace.rs`, parsed on every gate run |
| AC-003 | **GIVEN** the evaluator was told *"10 deferred"* by the published census, **WHEN** the gate runs on the tree that publishes that sentence, **THEN** three sets are proven equal — the clauses carrying `Re-read:`, the clauses `spec-trace` parses as `DEFERRED`, and §1.3's hand-computed **10** (`spec/SPECIFICATION.md`:219-222) — so a record on a non-deferred clause, a deferred clause without one, and an eleventh deferral added later each fail by name | Unit: `xtask/src/clause_audit.rs::tests::rejects_reread_on_a_non_deferred_clause` and `::rejects_an_eleventh_deferral_that_was_never_reread`, plus a fixture whose §1.3 figure disagrees with the parsed count. Real tree: the mandatory step's printed totals |
| AC-004 | **GIVEN** the maintainer at the *next* release, who would otherwise inherit ten reasons written for this one, **WHEN** `workspace.package.version` moves past `0.2.0` (`Cargo.toml`:5-6), **THEN** all ten records are stale by construction and the gate fails until each is re-read and re-stamped — the version comparison is exact, the date is checked for `YYYY-MM-DD` shape only, and the audit reads **no clock**, so two runs on one commit can never disagree | Unit: `xtask/src/clause_audit.rs::tests::rejects_a_record_stamped_at_a_previous_version` (fixture stamped `0.1.0` against a workspace at `0.2.0`) and `::rejects_a_malformed_date` (`2026-8-5`, `05-08-2026`, `soon`). Determinism: `::verdict_is_a_function_of_the_tree` asserts two evaluations of one fixture produce identical output |
| AC-005 | **GIVEN** a maintainer under release pressure, for whom the cheapest fake re-read is pasting the marker's experiment across into the new field, **WHEN** the record's prose — normalised (whitespace collapsed, backticks and punctuation stripped, case-folded) — equals, or is contained in, the marker text `falsifier_of` returns for that same clause, or falls below the stated minimum length, **THEN** the gate fails that clause by name rather than counting it re-read | Unit: `xtask/src/clause_audit.rs::tests::rejects_a_reason_that_restates_the_marker` — a fixture whose `Re-read` reason is the marker's text with the date prepended, which is precisely WF-1's existing *"(ADR-0016 §1, reading of 2026-08-05)"* shape (`spec/SPECIFICATION.md`:1899-1908) — and `::rejects_a_reason_below_the_minimum_length` |
| AC-006 | **GIVEN** the maintainer running the gate, **WHEN** the clause-audit step runs, **THEN** it runs **mandatorily** — inside `REQUIRED` in `xtask/src/main.rs`:105ff, never behind a `probe:` that would convert *found a problem* into *skipped* (`:89-102`) — it **reports rather than merely passes**, printing one line per clause (id, date, the reason's first line: ten lines, no more) into the dated report the slice commits, every failure names the clause id, the file and the field so the reader reaches the offending spot from the message alone, and `xtask/src/main.rs`:8-24 gains, in this same change, the sentence stating the gate now proves every deferral was re-read at the published version, alongside the audit's own module doc stating the axis it does not check | Real tree: the committed dated clause-audit report contains ten rows; the new step's registration carries no `probe:` argument; the module-doc diff is in the same commit. Unit: `xtask/src/clause_audit.rs::tests::failure_message_names_the_clause_and_the_field` asserts a rejected fixture's message contains the clause id |
| AC-007 | **GIVEN** anyone who cites this specification — `spec-trace`'s own line-range cross-references, `CLAUDE.md`, the ADR corpus — **WHEN** this change lands, **THEN** nothing normative has moved: the ten clauses' sentences, `[DEFERRED]` markers, `Rule:`, `Cases:` and `Rejects:` fields are byte-stable, no `[FROZEN]` or `[PROVISIONAL]` clause appears in the diff, no clause is renumbered or reordered, `cargo run -p xtask -- spec-trace --write` leaves the tree unchanged (the generated §7.1/§7.2 region opened at `:8507` does not move a cell), and `cargo xtask spec-trace` stays green so every existing citation still resolves | Static: `git diff spec/SPECIFICATION.md` shows added `Re-read:` lines and nothing else. Gate: `cargo run -p xtask -- spec-trace --write && git diff --exit-code spec/SPECIFICATION.md`; `cargo xtask affected --base main` (which runs `spec-trace` unconditionally, `_decomposition.md`:492) |
| AC-008 | **GIVEN** the maintainer must be able to believe a green gate, **WHEN** the parser silently stops recognising the field — the failure mode a compiled-once unit test cannot see, because it proves the parser worked when `cargo test` ran and not on the tree being published — **THEN** a `DEFERRAL_PROBE` fixture clause, parsed on every gate run in the `RETIRES_PROBE`/`WIRE_PROBE` shape (`xtask/src/spec_trace.rs`:849-868, `:954-967`, `:1802-1848`), fails the gate, and the audit additionally carries `#[cfg(test)] mod tests` in the `xtask/src/package.rs`:408-457 shape where **every** test names in a doc comment the wrong tree it rejects | Run-time: `DEFERRAL_PROBE` evaluated inside `cargo xtask spec-trace`, asserted to parse into a well-formed record. Unit: `cargo test -p xtask` — the module's tests, each with its named wrong implementation, satisfying `_decomposition.md`'s AC-TEST-002 |

Coverage of the traced project AC: **AC-005** (*"each of the ten `[DEFERRED]` clauses carries a stated
reason, readable by a consumer, that was re-read at this publish rather than inherited"*,
`project.md`:243-245) is discharged by AC-001 (stated, readable, consumer-facing), AC-003 (each of the
ten, exactly), AC-004 (re-read at *this* publish, mechanically), AC-005 (not the marker wearing a date)
and AC-006 (reported, never silent). AC-002, AC-007 and AC-008 are the obligations that make the other
five trustworthy rather than decorative.

## Interaction quality

This story renders **no surface**. `_design.md`'s seven surface ids are three crates.io pages, three
docs.rs pages and the GitHub landing page, and its `## Items` block carries exactly two entries, neither
of which is this story's. The composition family is therefore *not* discharged by pointing at a
stylesheet — it is discharged by naming the two media this story does render into, and the one
design-derived constraint that survives into a story with no page.

The two media are **the specification as a document a human reads at a clause**, and **the gate's
terminal output plus its committed dated report**. Both have state and composition invariants, and both
are places an unstyled, structurally-perfect render (a check that returns the right exit code while
printing nothing legible) would satisfy every assertion and still fail the reader.

**State invariants — which AC carries each, and how it is verified**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not a context jump** — the reason is read *at* the clause, 0 hops. It is deliberately not a second ledger file the reader must jump to (which is what `RUNBOOK.md`'s provisional ledger is, and why keeping it in sync is a separate story's problem) | **AC-001** | The record is a field line inside the clause body; the fixture asserts it parses out of the clause it belongs to, not a side table |
| **Non-occlusion** — the record sits beside `Rule:`/`Cases:`/`Rejects:` and never displaces, rewraps or rewords the normative sentence or the `[DEFERRED]` marker above it | **AC-007** | `git diff` over `spec/SPECIFICATION.md` shows added lines only |
| **Preserved anchors** (the document analogue of preserved focus/scroll/selection) — clause ids, ordering and every existing `file:line` citation into the specification still resolve after ten lines are added; the generated §7.1/§7.2 region does not move a cell | **AC-007** | `cargo xtask spec-trace` green, and `spec-trace --write` leaving no diff |
| **Reversibility** — the whole change is additive prose plus one allowlist entry; reverting is deleting ten lines and one string, with no published artifact and no persisted state depending on it. The irreversible move — resolving a deferral — is explicitly *not* available here | **AC-007**, and **EC-001** for the halt path | The PR boundary excludes clause resolution; EC-001 makes "no longer honest" a stop, not an edit |
| **Reachability of the failure** (the CLI analogue of keyboard reachability) — a maintainer reaches the offending clause from the failure message alone, without grepping: every message names the clause id, the file and the field | **AC-006** | `::failure_message_names_the_clause_and_the_field` |

**Composition invariants — which AC carries each, and how it is verified**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — a gate that returns the right exit code and prints nothing is the "unstyled render" of this medium. The step must *report*: ten rows, each legible without opening the specification | **AC-006** | The committed dated report contains ten rows (id, date, reason's first line) |
| **Density budget, with its real number** — **one line per clause in the step's summary output, ten lines total**; full reasons live in the clause and in the dated report file, never expanded inline into the gate log. This is the same instinct `_design.md` applied when it rejected DT-5 option (a) | **AC-006** | The report's row count and the summary's line count are asserted in the real-tree run |
| **Transience** — the reason is *persistent chrome* in the specification (it is part of the clause, and stays), and the gate summary is transient; the dated report is the persistent record of a transient run | **AC-001** (persistent), **AC-006** (transient + persisted report) | The record is committed in `SPECIFICATION.md`; the report is a committed dated artefact (`project.md` DoD 3) |
| **Hierarchy** — the record is subordinate to the clause's normative sentence and to the marker; it never reads as a new normative statement and carries no `Rule:`-style obligation language | **AC-001**, **AC-007** | The field's position and the byte-stability of everything above it |
| **The design's named anti-pattern that reaches this story** — `_design.md` rejected *(a) publish the ledger* because 200 rows bury every other claim on the one screen an evaluator reads. The corollary here: these ten reasons belong in the specification and the gate report, and **must not** be lifted onto any published README. What the published surface gets is DT-5's single census sentence, whose fourth number this story keeps true | **AC-003** (the count stays 10 and stays checked), **AC-007** (no published copy in this diff) | The PR boundary excludes README/rustdoc copy — that is `published-surface-copy`'s milestone |

**Not applicable, stated rather than skipped:** occlusion by overlay, focus restoration, and every
viewport/theme obligation in `_design.md` — there is no rendered surface here to occlude or to theme.
`design.capture` is absent from `.redkiln/config.yaml` (`CLAUDE.md`), so the perceptual review is a
recorded skip for this project, not a silent pass.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | A re-read concludes a deferral is **no longer honest** — the experiment has been run, the question was answered elsewhere, or the clause should now be provisional or frozen | **Stop.** The implementer halts and reports; it becomes a decision atom and a re-plan (`project.md` DR-15 / AC-015), never an edit inside this PR. Do not narrow the clause, do not change the marker, do not write a reason that describes a resolution as if it were a deferral |
| **EC-002** | A deferred clause carries no `Re-read:` record, or a non-deferred clause carries one | Gate fails, naming the clause id and which of the two it is. Set equality is against the parser's `DEFERRED` set, not a hard-coded list of ten |
| **EC-003** | The record's version does not equal `workspace.package.version` | Gate fails, printing both values and the clause id. This is the expected, wanted failure at every future release: the message must read as *"re-read me"*, not as a broken build |
| **EC-004** | The date is not `YYYY-MM-DD` | Gate fails on shape. The audit does not attempt to judge whether the date is recent — that would require a clock, and the verdict must be a function of the tree alone |
| **EC-005** | The reason is empty, below the minimum length, or a normalised restatement of / containment in the marker | Gate fails, quoting the overlap it found, so the maintainer can see *why* it read as a paste rather than being told only that it did |
| **EC-006** | §1.3's stated deferred count and the parsed `DEFERRED` count disagree | Gate fails and says which is which. §1.3 stays **hand-computed** (`xtask/src/spec_trace.rs`:39-57) — the audit reports the disagreement; it never rewrites the sentence to make itself pass |
| **EC-007** | `spec-trace --write` produces a diff after this change, or check 4 reports a rule that does not exist | Treat as the parser-ordering defect, not as a specification defect: `Re-read` is missing from `field_head` and the reason is being absorbed as continuation into `Rule:`. Fix the allowlist first, then re-run |
| **EC-008** | `DEFERRAL_PROBE` fails to parse | The parser has regressed on the tree being published. Fail the gate loudly; never downgrade the probe to a unit test to make the gate green |

## Non-functional

| id | requirement | rationale / evidence |
| --- | --- | --- |
| **NF-001** | **Determinism.** The audit's verdict is a pure function of the tree: no clock, no network, no environment lookup. Two runs on one commit produce identical output | `xtask/src/spec_trace.rs`:23-38 (equality-against-the-tree is the load-bearing property); `xtask/src/main.rs`:361 (CF-33's clock prohibition as the house instinct) |
| **NF-002** | **One parser, not two.** The audit composes with `parse_clauses`/`field_line`/`falsifier_of` and does not fork a second reader of `SPECIFICATION.md`. Two parsers of one document is exactly how the census and the table came to disagree | `xtask/src/spec_trace.rs`:1357-1389, `:1432-1489`, module doc `:23-57` |
| **NF-003** | **Gate cost.** The document is already parsed once per `spec-trace` run; the deferral checks add set operations and ten string comparisons over the existing parse. No measurable addition to gate wall-clock, and no second read of the 566 KB file | `spec/SPECIFICATION.md` is read by `spec-trace` today |
| **NF-004** | **Fixtures are synthetic.** Unit tests run against short `SPECIFICATION.md`-shaped strings, never the real 200-clause document, so a test cannot be made to pass by editing the specification and cannot be made to fail by a legitimate clause edit | `_decomposition.md`, *Fixtures and seams to mock* (`package.rs`:417-423's `METADATA` constant is the established seam) |
| **NF-005** | **Documented limits.** The audit's module doc states the axis it does **not** check — whether a reason is *true* — because a check whose limits are undocumented is read as a guarantee | `xtask/src/main.rs`:26-42; `standards/rust/70-rustdoc-obligations.md` |
| **NF-006** | **No published-surface cost.** Nothing here enters a published crate: `xtask` is not published, and the specification is not packaged. `cargo package --list` output for the three publishable crates is unchanged | `xtask/src/package.rs`'s `PUBLISHABLE` set |

## Implementation notes (non-prescriptive)

*Guidance, not instruction — the implementer's judgement governs, but the ordering in the first bullet
is load-bearing and reversing it wastes a debugging session.*

- **Do the allowlist first.** Add `"Re-read"` to `field_head` (`xtask/src/spec_trace.rs`:1541-1555)
  and run the gate green *before* writing a single record. Written the other way round, the first
  `Re-read:` line is parsed as continuation of the field above it, and the gate reports non-existent
  rules and a moved §7.2 cell — three true failures all pointing away from the cause.
- **Then write one record, on one clause, and run `spec-trace --write`.** A no-diff result on a single
  record is the cheap proof that the field is inert in the generated region. Only then write the other
  nine.
- **The record's local spelling follows its clause.** Three field spellings are in use in the document
  (`Rule:` bare, `- **Rule:**`, `` `Rule:` ``) and `field_head` strips the punctuation. Match the
  clause's neighbours rather than imposing one spelling across all ten — this diff should look like ten
  clauses each gaining a line, not like a formatting pass.
- **Where the checks live is the slice's call, not this story's.** `clause-maturity-audit` creates the
  audit module and registers the mandatory step; these checks are functions *in that module*. If the
  slice-mate names the module something other than `clause_audit.rs`, the test paths in this spec and in
  `_ledger.md` follow that name — the ledger's `verifying_test` is a real path to a real test, and the
  file it sits in is the slice's naming decision recorded at implementation.
- **Two kinds of test, on purpose.** The `#[cfg(test)] mod tests` cases prove the logic; the
  `DEFERRAL_PROBE` proves the parser on the tree being published. Note for the record: the testing brief
  cites `spec_trace.rs:1809-1825` as a `#[cfg(test)]` precedent, and it is not one — `spec_trace.rs` has
  no test module and those lines are `WIRE_PROBE`. The precedent is real; its kind is different; this
  story uses both kinds deliberately.
- **Reading the ten is the actual work.** The mechanism is a day's typing; the value is in ten honest
  reads. Read the clause, read what the marker says settles it, then answer one question in the caller's
  register: *what does depending on `0.2.0` cost me while this is open?* If the answer is "nothing you
  would notice", say that — a deferral with no consumer-visible cost is a fine deferral, honestly stated.
- **Normalisation for the restatement check.** Collapse whitespace, strip backticks and trailing
  punctuation, case-fold, then test both equality and containment in either direction against
  `falsifier_of`'s text. Containment in *either* direction matters: a reason that is the marker plus a
  date, and a reason the marker swallows, are the same fake.
- **Watch WF-1 specifically.** Its marker already ends *"(ADR-0016 §1, reading of 2026-08-05)"* — a dated
  reading two phases old, sitting inside the marker. It will pass any naive "mentions a date" check and
  it is the clause most likely to receive a reason that is just that string moved.

## Tests and CI (merge gate)

Grounded in `_decomposition.md`'s testing brief — the AC-005 row of the per-AC instrument table
(*static*; wrong implementation: *"a deferred clause whose reason string is unchanged from a prior
phase's audit"*), AC-TEST-002 (`#[cfg(test)] mod tests` in the `package.rs`:408-457 shape) and the
merge-gate commands at `_decomposition.md`:485-495.

| tier | command / path | proves |
| --- | --- | --- |
| **unit** | `cargo test -p xtask` → `xtask/src/clause_audit.rs::tests::*` (synthetic clause-string fixtures; each test's doc comment names the wrong tree it rejects) | AC-001 (missing record), AC-002 (field termination), AC-003 (three set-equality failures), AC-004 (stale version, malformed date, determinism), AC-005 (marker restatement, minimum length), AC-006 (failure message names the clause) |
| **run-time probe** | `cargo run -p xtask -- spec-trace` → `DEFERRAL_PROBE` in `xtask/src/spec_trace.rs`, in the `RETIRES_PROBE` (`:954-967`, checked `:849-868`) / `WIRE_PROBE` (`:1802-1848`) shape | AC-002, AC-008 — the parser recognises the field **on the tree being published**, not merely when `cargo test` last ran |
| **static (mandatory gate step)** | the clause-audit step in `xtask/src/main.rs`'s `REQUIRED` list (`:105`ff), no `probe:` | AC-001, AC-003, AC-004, AC-005, AC-006 against the real `spec/SPECIFICATION.md` and the real `Cargo.toml` version |
| **static (idempotence)** | `cargo run -p xtask -- spec-trace --write` then `git diff --exit-code spec/SPECIFICATION.md` | AC-007 — the new field is inert in the generated §7.1/§7.2 region; a diff means it is leaking into `Rule:` or `Cases:` |
| **static (diff shape)** | `git diff spec/SPECIFICATION.md` reviewed at the checkpoint | AC-007 — added `Re-read:` lines and nothing else; no `[FROZEN]`/`[PROVISIONAL]` clause touched, nothing renumbered |
| **story grain (automatic)** | `cargo xtask affected --base main` — wired to redkiln's story grain in `.redkiln/config.yaml` | The whole story. It runs `spec-trace` and the five file-reading lints **unconditionally**, which is why a story that only edits `SPECIFICATION.md` and `xtask/` is still gated (`_decomposition.md`:492) |
| **integration grain (slice, automatic)** | `cargo xtask ci --fast` — this project is not terminal (`closeout-and-durable-audience` holds `verify.e2e`) | The mandatory step is in `REQUIRED` and the gate is green with the four slice stories mounted together |
| **backlog / KB** | `redkiln validate --kb && redkiln doctor` | AC-TEST-003 — clean at this story's checkpoint, with exactly the six expected `template-drift` advisories (`CLAUDE.md`) |
| **deferred to the release event (not this PR)** | full `cargo xtask ci` on the literal publish commit, output committed and dated | Project AC-016 / DoD 5 — owned by `publish-0-2-0`, which depends on this story. Explicitly **not** run here, and `--fast` is not treated as a substitute (`_decomposition.md`:498-505) |

**Merge DoD (one line):** `cargo xtask affected --base main` green, `spec-trace --write` leaves no diff,
the mandatory clause-audit step passes with ten records stamped `0.2.0` — and fails, by name, on each of
the seeded wrong trees in EC-002 through EC-006.

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | mitigation, in this PR |
| --- | --- | --- |
| **A reason is written to pass the check rather than to inform a caller.** The exact shape of `project.md`'s first risk row — a falsifier chosen during an audit to make the audit pass — one level over | Medium / High | The restatement and minimum-length checks (AC-005) raise the price of the cheapest fake, and the axis they cannot check is named in the module doc (NF-005) rather than implied. The residual is a human review obligation at the slice review, stated here so it is not mistaken for something the gate caught |
| **The parser edit is made after the prose edit** and produces three true failures pointing at the wrong cause | Medium / Medium | EC-007 names the symptom and the cause; the implementation notes make the ordering the first bullet |
| **The field leaks into the generated region** and §7.2 shifts, so check 9's equality fails at the worst moment | Low / High | AC-007's `--write`-then-`git diff --exit-code` is a gate step, not a habit, and the single-record dry run in the implementation notes catches it after one line rather than ten |
| **A re-read finds a deferral that is no longer honest**, and the pressure is to quietly write a reason anyway | Medium / High | EC-001 makes it a halt with a named destination (a decision atom and a re-plan, `project.md` DR-15/AC-015). This story cannot resolve a clause: doing so is outside the PR boundary |
| **Merge conflict on `xtask/src/main.rs`'s `REQUIRED` list and its module doc** — the reason the slice exists | Low, by construction / High if it happens | `_storymap.md`:79-86: the four slice stories are implemented in **one context**, sequentially. This story adds no `Step` entry of its own; it extends the step `clause-maturity-audit` registered, and extends the same doc paragraph in the same change |
| **`clause-maturity-audit` lands with a different module name or report format** than this spec assumes | Medium / Low | Nothing here depends on the name — only on there being one mandatory step and one dated report. The test paths follow the slice's naming decision (implementation notes); the ledger's `verifying_test` is filled with the real path at implementation |
| **The version stamp turns into noise at the next release** — ten failures at once, read as a broken build rather than an obligation | Medium / Low | EC-003 requires the failure message to read as *"re-read me"*, printing both versions and the clause id. That the failure is loud, total and annual is the feature, not the defect |
| **Downstream copy asserts a number this story keeps true.** DT-5's census sentence publishes *10 deferred* | Low / Medium | AC-003 makes the 10 a checked equality across three sources rather than a remembered figure. The sentence itself is `landing-copy-and-status-truth`'s to write, and `publish-0-2-0` depends on this story so the check exists before the claim ships |

## Dependencies

**Blocks on**

- **`clause-maturity-audit`** (HS-S0089) — strictly. It creates the audit module, registers the single
  mandatory step in `xtask/src/main.rs`'s `REQUIRED` list, and establishes the dated report format. This
  story adds checks *inside* that step and rows to that report; it registers nothing of its own
  (`_storymap.md`:161-165, 179-184). Its own blocker, `falsifier-ledger-repair`, is transitively this
  story's — the ordering `falsifier-ledger-repair` → `clause-maturity-audit` → `deferred-clause-reread`
  must not be reordered by a re-plan.

Nothing else blocks: this story reads `Cargo.toml`'s version but does not choose it, and it reads the
`[DEFERRED]` markers as they stand.

**Unlocks**

- **`publish-0-2-0`** — names this story in its own `dependsOn`. The release commit must run a gate that
  proves every deferral was re-read at the version being published; without this story that half of DoD
  12 is unchecked and the census sentence is a claim nobody verified.
- **`landing-copy-and-status-truth`** — indirectly. It writes DT-5's census sentence, whose *10 deferred
  (each names the experiment and the phase that owns it)* is exactly the set this story now proves and
  keeps honest.

**Same slice, implemented in one context, mounted once:** `falsifier-ledger-repair`,
`clause-maturity-audit`, `registry-surface-diff`.

## Anchors (progressive disclosure)

Read the **Context pack** first — it is complete enough to start. Open these only at the moment named.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `xtask/src/spec_trace.rs`:1499-1555 | `field_line` and `field_head` — the closed allowlist and the continuation rules. The single edit this story makes to the parser, and the one place its ordering hazard is visible | **First, before writing any prose.** Add `"Re-read"` here and run the gate green before touching `SPECIFICATION.md` | AC-002 |
| `xtask/src/spec_trace.rs`:23-57 | The module doc: the §7.1/§7.2 equality check is the load-bearing property, and §1.3 must stay hand-computed for the reconciliation to mean anything | Before the first `spec-trace --write`, and again if §1.3's count is tempting to "fix" | AC-003, AC-007 |
| `xtask/src/spec_trace.rs`:650-679 | CF-38's existing empty-falsifier check (≥12 characters) — the check this one sits beside without duplicating, and the reason folding the reason into the marker makes the new obligation invisible | While designing the restatement check, to see what is already proven and what is not | AC-005 |
| `xtask/src/spec_trace.rs`:1432-1489 | `maturity_of` / `falsifier_of` — the parsed marker text the reason is compared against, and the source of the `DEFERRED` set | When implementing the set equality and the restatement comparison | AC-003, AC-005 |
| `xtask/src/spec_trace.rs`:849-868, `:954-967`, `:1802-1848` | `RETIRES_PROBE` and `WIRE_PROBE` — the run-time probe pattern to copy verbatim for `DEFERRAL_PROBE` | When writing the probe, after the checks pass on the real tree | AC-008 |
| `xtask/src/package.rs`:408-457 | The testing brief's named precedent: a fixture string, a named wrong implementation in a doc comment, an assertion it is rejected. `:417-423`'s `METADATA` constant is the string-fixture seam to reuse | When writing `mod tests` — copy its shape, not a happy path | AC-008, and every unit row |
| `xtask/src/main.rs`:8-42, `:89-102`, `:105`ff | What the gate proves, in prose (`:8-24`); why undocumented limits read as guarantees (`:26-42`); what `Some(probe)` does to a failing step (`:89-102`); the `REQUIRED` list itself | When mounting — this is the mount point, and the doc sentence lands in the same change | AC-006 |
| `spec/SPECIFICATION.md`:204-207, `:219-222` | The `[DEFERRED]` marker's definition (the experiment and its owning phase) and §1.3's hand-computed census with its **10** | Before writing the first record, to keep the reason a different field from the marker | AC-001, AC-003 |
| `spec/SPECIFICATION.md`:1899-1908 | WF-1's marker, already ending in a dated reading two phases old — the live example of the shape *not* to reuse, and the clause most at risk of a copied reason | When re-reading WF-1, which should be first | AC-005 |
| `spec/SPECIFICATION.md`:8507-8519 | The generated region's opening marker and §7.2's census row — what must not move, and the DEFERRED rows enumerating the ten | After the parser edit, to confirm the field is inert; and to locate the ten clauses | AC-003, AC-007 |
| `Cargo.toml`:5-6 | `workspace.package.version` — the value the stamp is compared against, and the whole of "re-read at *this* publish" | When implementing the version check | AC-004 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` | The testing brief's AC-005 row (tier, and the wrong implementation it must reject), AC-TEST-002's `mod tests` bar, the merge-gate commands, and the deployment brief's AC-DEP-005 (mandatory, no probe, module doc in the same change) | Before writing tests, and before mounting | AC-006, AC-008 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/project.md` | AC-005's exact wording, DR-5 and DR-15, the risk register's first row (a falsifier chosen to make the audit pass), and the coupling note that `spec-trace` is both input and subject | At the start, and again if a re-read looks like it wants to resolve a clause | AC-001, AC-004, EC-001 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md`:79-86, 161-165, 179-184 | Why the slice is one context, why this story is third in it, and where the release blocks | Before starting, to confirm `clause-maturity-audit` has landed | AC-006 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` | DT-5's resolution: the census sentence this story keeps true, and why publishing the ledger lost on density. Confirms no surface here and no public item | When tempted to put the ten reasons on a README | AC-003, AC-007 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | *A skip is reported, never silent* — the discipline this story applies to prose, and the reason a silent green run is a defect | When deciding what the step prints | AC-006 |
| `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md` | Records that no phase obliges anyone to read the specification back against the tree, and names first publish as the forcing event. This story is one instance of that read — **not** its resolution | Before concluding this story resolves the atom; that call is AC-013's decision pass | AC-001, EC-001 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | Persona 4, the evaluator deciding in one sitting — the reader whose *"10 deferred"* this story keeps honest, with the secondary-evidence qualification that travels with the persona | When writing the ten reasons, to pitch the register | AC-001 |
| `standards/rust/70-rustdoc-obligations.md` | The doc-comment bar: state the limit, name the alternative once. The audit's module doc must say it does not check whether a reason is true | When writing the module doc | AC-006, NF-005 |
| `RUNBOOK.md`:163 | Phase 12's row and its proof artefact — where this work sits in the plan of record | For orientation only | — |

## Clarifications resolved during spec

1. **The AC set is exactly the eight the front half enumerated** — AC-001 through AC-008. None added,
   none dropped. AC-001/003/004/005/006 discharge project AC-005 between them; AC-002/007/008 are the
   obligations that keep the other five from being decorative.
2. **The reason is a new field, not an extension of the marker.** Folding it into the `[DEFERRED]`
   marker would let CF-38's existing ≥12-character check be satisfied by text that was already there,
   making the new obligation invisible. Resolved in favour of a separate `Re-read:` field, which is also
   what forces the `field_head` edit.
3. **"Re-read rather than inherited" is mechanised as a version stamp, not a date comparison.** A clock
   would make the gate non-deterministic against a tree. The date is checked for shape and is there for
   the human reader; the version is what fails, totally and automatically, at the next release.
4. **This story renders no surface, and the composition family is discharged rather than waived.** The
   media are the specification at a clause and the gate's output; both carry composition invariants, and
   both are mapped onto existing AC rows in *Interaction quality* — including a real density number (ten
   summary lines, one per clause) and the design's transferable anti-pattern (do not publish the ledger).
5. **A deferral found to be no longer honest is a halt, not an edit** (EC-001). The temptation to resolve
   a clause while re-reading it is the specific failure `project.md` DR-15 and
   `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md` exist to catch, and this spec
   forecloses it in the PR boundary rather than trusting restraint.
6. **The test module's filename is the slice's decision, not this spec's.** `clause-maturity-audit`
   creates the audit module; this spec and `_ledger.md` name `xtask/src/clause_audit.rs` as the expected
   path and state that the ledger's `verifying_test` follows the real name recorded at implementation.
7. **The testing brief's `spec_trace.rs:1809-1825` citation is a mis-citation** — those lines are
   `WIRE_PROBE`, a run-time probe, and `spec_trace.rs` has no `#[cfg(test)]` module. Both precedents are
   real and this story uses both kinds deliberately; the brief is not corrected here (it is another
   story's artefact), the discrepancy is recorded so the implementer does not go looking for a test
   module that does not exist.
8. **`RUNBOOK.md`'s falsifier ledger is out of scope.** It is about `[PROVISIONAL]` clauses;
   `falsifier-ledger-repair` and `clause-maturity-audit` own it. Nothing in this story reads or writes it.
