---
item: HS-P0010
stage: implementation
created: 2026-08-14T01:25:19.009Z
updated: 2026-08-14T01:25:19.009Z
template_sig: 4c5f37d6
rendered_sig: e468b480
---

# Slice ledger — Freeze ProjectionStore behind a suite that can fail

The review verdict of every vertical slice of this project, sealed as it closed.

A slice's stories commit BEFORE its adversarial review runs, so "committed" is not "approved".
This ledger is the second axis: it records which slices actually cleared review, so a re-launched
implement run re-reviews a rejected slice instead of walking past it as finished. Each row is sealed
by a commit carrying a `Slice-Verdict: <project-slug>/<slice> <verdict>` trailer — the ledger is the
human-readable record, the trailer is what resume greps.

## Verdicts

| Slice | Verdict | Story checkpoints | Sealed by |
| ----- | ------- | ----------------- | --------- |
| decisions-and-design-record | approved | ps-clause-pairing-sweep f77f183, projection-decision-atoms 9520b28, projection-api-design-record 0df2c1c | (this commit) |
| projection-port-and-probe | approved | owned-batch-port-shape 2eade38, projection-probe-conformance-feature cb495ee, memory-projection-store 5fd62c6 | (this commit) |
| projection-conformance-suite | approved | projection-suite-entry-point 79df6b7, projection-capability-skips 7fcb378 | (this commit) |
| commit-atomicity-and-mutants | approved | projection-mutant-registry c385e40, commit-rollback-and-drop-rules 5d9b4fd | (this commit) |

## Surviving findings

For each slice whose verdict is `changes-requested`, the findings that survived the in-slice fix
pass, with the `file:line` evidence the reviewer cited. These are the prescription a resumed run —
or a human — starts from. They are hypotheses for the next reviewer to verify, not facts to trust.

## Fix passes

Claims for the next reviewer to verify, not facts to trust. The findings above are left exactly as
the reviewer wrote them; what each one was answered with is recorded here.

### projection-port-and-probe — fix pass 2026-08-14

- **The `[dev-dependencies]` snippet.** Rewritten to the shape that compiles and still carries the
  argument, at `crates/happenstance-core/src/projection.rs:496-509`: `[dependencies]
  happenstance-core = "…"`, `[features] conformance = ["happenstance-core/conformance"]`,
  `[dev-dependencies] happenstance-testkit = "…"`. The forwarding entry carries a comment saying why
  the dev-dependency spelling cannot work — the impl lives in the adapter's `src/` and a crate cannot
  `cfg` on a dependency's feature — so the wrong form is refuted on the page rather than merely
  absent from it. The prose above it (`:491-494`) now says the impl goes in `src/`, which is what
  the orphan-rule paragraph three lines above always implied.
- **The unnamed falsifier.** Named, at `:514-517`: "the falsifier is `documented-extension-surface`
  (HS-S0015), which builds an outside author's fixture from this documentation alone". The first arm
  of the reviewer's remedy was taken rather than the second: probe AC-006 asks for the name, the
  slug is the identifier the outside-author story is known by in every other artefact, and a
  placement argument whose falsifier is anonymous is one nobody can go and check.
- **The AC-006 row.** `projection-probe-conformance-feature/_ledger.md` AC-006 records the
  withdrawal and the re-assertion, quotes both false descriptions it previously carried, and states
  the evidence against the corrected text with current line numbers.
- **The stale adapter module docs** — a defect of the same class, found by this pass and not by the
  slice review. Four rendered module docs described the deleted GAT port in the present tense; they
  now read, in order, `crates/happenstance-neon/src/projection_store.rs:3-22` and `:51-53`,
  `crates/happenstance-sqlite/src/projection_store.rs:11-18`, and
  `crates/happenstance-postgres/src/projection_store.rs:7-12`. All four are restated in the past
  tense with the port's current shape and the clause that settled it (PS-4, PS-5, PS-6, ADR-0017).
  This is what `owned-batch-port-shape/spec.md:180` put those files in the boundary to prevent, and
  it had been done for the `//` comments and the crate-level `//!` docs but not for the module `//!`
  docs a docs.rs reader actually meets.
- **The duplicated `error[E0195]` narrative.** Trimmed at the module doc (`projection.rs:25-36`) to
  an intra-doc pointer at `ProjectionStore`'s `# Implementing it` — the item whose unusual shape
  bought it, and the one carrying the compiling doctest — leaving the narrative stated once, per
  RS-70-5 and memory AC-009. The rustc ICE, which is the *other* of the two compiled results and is
  stated nowhere else, stays in the module doc.
- **`stand_in.rs`, the undeclared boundary file.** Settled as a deliberate widening of
  `owned-batch-port-shape/spec.md`, recorded in the paragraph under its PR-boundary fence with the
  reason: the edit was forced by an entry the boundary already admits (ADR-0017's removal of the
  `live_handle` module, which `crates/happenstance-ladybug/src/lib.rs` is in the list for), and an
  intra-doc link into a removed module is a hard error under `broken_intra_doc_links = "deny"`. The
  alternative was not "leave the file alone" but "do not execute ADR-0017".

## Authorised boundary widenings

Widenings taken outside any single story's declared boundary, recorded where a later reader can find
them. A commit message asserting its own authorisation is not an authorisation record.

- **`cc0f158` — `spec/SPECIFICATION.md`, `standards/rust/**`, `CLAUDE.md`, `xtask/src/main.rs`.**
  None of the three story specs in this slice lists those paths, and `owned-batch-port-shape`'s
  EC-004 explicitly requires stop-and-report rather than editing `SPECIFICATION.md` inside the story.
  The stop-and-report *was* filed — `owned-batch-port-shape/_reviewed-diff.md` §7 — and `cc0f158`
  executed its remedy (a) at the slice boundary rather than inside a story, which is where a finding
  larger than one story is supposed to be discharged. The forcing reason: `xtask/src/affected.rs:117-125`
  runs `spec-trace` first and unconditionally and `.redkiln/config.yaml:40` wires
  `cargo xtask affected --base {{base}}` as the story gate, so while the seven stale citations stood,
  **no** story in this project could pass its own declared gate. The re-points changed `file:line`
  targets only — no clause text and no maturity marker moved — and the two retirements (RS-22-4 and
  RS-92-1) were taken at each rule's own written PROVISIONAL settlement trigger, which PS-5 met when
  ADR-0017 landed. RS-21-1 was amended, not retired, because only its spelling died. Ratified here
  on 2026-08-14; the remaining exposure is that it was ratified *after* the fact, which is what this
  section exists to stop happening twice.

### commit-atomicity-and-mutants — human decision, 2026-08-14

**AC-001 `failed_commit_leaves_both_unchanged`: the human chose to AMEND DT-3 and add the
commit-fault capability.** The three escapes the halting implementer enumerated were all
correctly refused; this is the fourth path and it is a deliberate design change, not a
workaround. Add a capability constant that arms a commit fault, in `MID_BATCH_FAULT`'s mould,
so the rule can be written and gated rather than skipped or duplicated.

What that obliges, and none of it may be skipped:

- `_design.md`'s DT-3 resolution (`:276-280`) currently enumerates the capability set as
  exactly three constants. Amend it in place with the date and the reason, and do not silently
  grow the set — the enumeration is the signed-off artefact and its amendment is the record.
- `crates/happenstance-testkit/src/contract.rs:519-596` implements exactly that three-constant
  set and must grow with it.
- The rule needs a **mutant that fails it and passes the others** — a store whose `commit`
  reports failure and leaves one half written. Without that it is the decorative rule ADR-0010
  exists to make unwriteable, which was escape (2)'s defect and is not cured by arriving via a
  capability.
- `cf-40-fixture-limits-ownership.md` and `project.md`'s risk register both forbid minting a
  *second declension policy*. Adding a fourth capability under the existing policy is not that;
  inventing a new way for a fixture to decline would be. Stay on the near side of that line.

PS-1's second conjunct is unenforced by any conformance rule until this lands.

**Executed 2026-08-14, and every obligation above discharged.** `_design.md`'s
DT-3 table carries a fourth row with the date, what forced it and why a fourth
constant under the existing policy is not a second declension policy;
`crates/happenstance-testkit/src/contract.rs` grew with it, `COMMIT_FAULT`
required rather than defaulted so no testkit-written reason had to be minted;
`failed_commit_leaves_both_unchanged` landed gated on it; and
`PartialCommitStore` — rows applied, checkpoint not, error returned honestly —
fails that rule and nothing else, demonstrated by deleting its one line and
watching the exactness meta-test go red by name. Three fixtures answer the new
constant: the mutant harness supports it, the reference fixture and the declining
instrument decline it with their own words, and the reference run therefore
prints exactly one `SKIP` line — which is the first time this port has
demonstrated CF-18's reporting discipline on real values rather than reserved it
for a later story.

The residue, named rather than left to be found: no fixture-level mutant is
registered for an `arm_commit_fault` with an empty body. The rule's first
assertion rejects it and the trait's provided body panics for the commoner
mistake, so the hole is in the demonstration rather than in the suite, and
`projection_mutation_coverage.rs`'s scope note says so.

### projection-conformance-suite — boundary, found after the slice sealed

**Both stories in this slice write outside their declared boundaries, and the slice sealed
`approved` without noticing.** `redkiln verify --grain story` rejects both:

- `projection-suite-entry-point` (`79df6b7`) — eleven `standards/rust/**` atoms, 23 insertions
  and 23 deletions.
- `projection-capability-skips` (`7fcb378`) — eight of the same atoms, 17 and 17.

**Every hunk is a citation line-number re-pointing** (`lib.rs:370` → `lib.rs:384`,
`fixtures.rs:369` → `fixtures.rs:521`). Both stories legitimately own
`crates/happenstance-testkit/src/**`; editing it shifts the lines the constitution cites into
it; `cargo xtask lint-constitution` checks those citations and is a gate step. The work is
**compelled, not discretionary** — the same shape run 3 ratified for `owned-batch-port-shape`'s
`stand_in.rs` edit, where `broken_intra_doc_links` at `deny` forced an edit the boundary already
implied.

This is now the **third** instance, so treat it as structural rather than as three accidents:
this repository has two deny-level cross-reference checkers whose targets sit outside any
story's natural boundary, so any story touching a cited file is forced across its own boundary
or into a red gate. Settle it once — either each such spec's boundary admits the corpora its own
edits compel, with the reason stated as run 3 stated it, or the boundary check learns that a
compelled citation repair is not a scope breach.

**Do not widen a boundary to match what was written without saying why.** That converts the
check into a rubber stamp for whatever the implementer happened to touch, which is the failure
mode the check exists to prevent.

### reset-and-rebuild-rules — fix pass 2026-08-14

The slice review returned four findings against `10ace94` and `5be22ab`. All four
are answered below; the first is a **halt**, not a repair, and it leaves an AC
blocked rather than satisfied.

**BLOCKING — AC-005's ordering gate was unmet, and the recorded evidence
contradicted the repository's own KB.** `reset-rules`' AC-005 makes
`fresh_projection_has_no_checkpoint` legitimate *only* once the PS-19 repair atom
from `projection-decision-atoms` is `status: accepted` under `.kb/decisions/`;
EC-001 requires the story to **halt and report** otherwise, because writing the
rule anyway "widens a `[FROZEN]` clause by test". **No repair atom exists** —
`.kb/decisions/` holds 0001–0019 and 0029, and none widens PS-19.
`reset-rules/_ledger.md` and `report.md` both claimed the precondition was met,
citing ADR-0018's own `status: accepted`; that is a different atom answering a
different question, and ADR-0018 says so about itself
(`.kb/decisions/0018-returning-a-projection-to-never-run.md:113-116`: the pairing
defect "sits inside this clause range and is named here as a gap, not repaired").
`.kb/open-questions/ps-19-scope-narrower-than-its-rule.md:122-129` and
`projection-decision-atoms/spec.md:72` agree, and so does the specification about
itself (`spec/SPECIFICATION.md:5238-5239`).

The consequence was substantive rather than procedural: the suite convicted
adapters of an obligation PS-19's frozen MUST does not state, and
`PresumedLiveCheckpointStore` — registered as a *mutant* — is **conformant** with
PS-19 as written today. CF-5's positive control could not have caught it, because
no conformant variant in the projection registry models the legal
`.unwrap_or(Checkpoint::Live { through: FIRST })` store.

**Path (a) taken — EC-001 as written.** `fresh_projection_has_no_checkpoint` and
`PresumedLiveCheckpointStore` are **held**: removed from the rules module, the
`for_each_projection_store_rule!` enumeration, `mutants.rs`, the projection
`REGISTRY` and `for_each_projection_mutant!`, with the reason stated in full at
each point they would sit. PS-19 was **not** line-edited and the rule was **not**
dropped silently — `CHANGELOG.md` carries the non-delivery as an entry naming
why, and `cargo xtask spec-trace --write` restored `†` ("does not exist yet") on
§7.2's PS-19 row, which is the gate stating the hold out loud. The ledger row is
`satisfied: false` with the citations above; the report's summary reads nine of
ten.

**Path (b) remains open to a human and was not taken.** The reviewer's
alternative was a recorded decision to proceed, in the shape this ledger already
carries for the DT-3 amendment above. No human authorised it, and this run
declined to invent an authorisation. If a human takes path (b), what it obliges,
and none of it may be skipped:

- Restore the rule, its mutant, its registry row, its enumeration entries and its
  `CHANGELOG.md` entry, and re-run `spec-trace --write` to clear the `†`.
- Correct `reset-rules/_ledger.md` AC-005 and `report.md` to state that the
  precondition was **not** met, cite `.kb/decisions/0018-…:113-116` and
  `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md:122-129`, and **name
  who authorised proceeding**. Proceeding on an unmet precondition is a decision;
  it is not a green run.
- Say in the same place what the suite is then asserting that no clause states,
  so `unstable-projection-gate-and-clause-disposition` inherits a debt it can see
  rather than a rule it will assume was always owed.

Either way the repair itself is still owed and is still that story's: a **new
accepted decision atom** widening PS-19 or minting the clause, under
`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` — never a line
edit.

**Stale rustdoc created by this slice** — the class the previous slice review
escalated (`d9eb8de` items 5–6), recurring.
`crates/happenstance-testkit/tests/mutation_coverage/variants.rs:660-673` said of
`RESET_REFUSAL` that "no rule reads that constant yet" (it is read by
`refused_reset_changes_nothing`) and that the reference fixture "reports exactly
one skip"; it reports two, and the assertion that landed in the same commit pins
two. Rewritten to state both declensions, both now read by a rule, and the
two-skip set `assert_reference_projection_declensions` pins.
`tests/mutation_coverage.rs`'s `PROJECTION_MUST_REJECT` doc said "the four
absentees"; holding one rule made it three, and it now says three. **This is the
third occurrence in three slices, and the pattern is worth naming:** a doc
comment that counts something goes stale in the next commit, and nothing in the
gate reads a prose count.

**Misattached doc comment** in `crates/happenstance-testkit/src/projection.rs`.
The new unit test's doc had been appended to the *existing* block above
`two_opens_make_two_isolated_stores`, so
`a_declined_reset_refusal_is_reported_with_the_fixtures_reason` carried a summary
line describing a different test and `two_opens_make_two_isolated_stores` carried
none. The block is split back; while restoring it, "will depend on it directly
one slice later" was corrected to the present tense, that slice having landed at
`5d9b4fd`.

**`read-through-and-rebuild-rules` AC-006 recorded as a boundary finding rather
than as satisfaction.** The criterion asks for "the fixture's **own** non-empty
stated reason"; the reason a skip actually carries is the testkit-written
`NO_BATCH_READ_PATH_REASON`, because the landed probe const is a `bool`
(`crates/happenstance-core/src/projection.rs:536`) and carries no reason. That
follows the `NO_CEILING_REASON` / `NO_STORE_LIMITS` exception the story's own D1
points at, and the PR boundary forbids the alternative — "a rule that cannot be
written against the landed port is a finding, not a licence to edit the port".
The ledger row now says so, and routes the open question — should the probe const
carry a reason? — to `owned-batch-port-shape` / the port disposition story.

**Scope drift, examined and kept.** `spec/SPECIFICATION.md`'s changes across both
commits sit between the `BEGIN/END GENERATED` markers of §7.1–§7.2, are produced
by `cargo xtask spec-trace --write`, and are stale-checked by a gate step
(`xtask/src/spec_trace.rs:747,1034-1051`) — leaving them would fail CI. Every
hunk is a `†` added or removed as a rule name started or stopped existing; no
clause text, maturity marker or rule citation moved. The
`standards/rust/{11,13,40,41}-*.md` hunks are `file:line` citation anchors
re-pointed because `contract.rs` grew inside the boundary and
`cargo xtask lint-constitution` is a gate step. Both are compelled, and both are
the pattern ratified at `d9eb8de` and at run 3 — which makes this the **fourth
and fifth** instance, and is the standing argument for settling it once rather
than ratifying it per slice.
