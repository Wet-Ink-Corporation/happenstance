---
item: HS-P0010
stage: implementation
created: 2026-08-14T01:25:19.009Z
updated: 2026-08-15T07:21:56.000Z
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
| reset-and-rebuild-rules | changes-requested | reset-rules 10ace94, read-through-and-rebuild-rules 5be22ab | (this commit) |
| second-batch-shape-and-evidence | approved | buffering-conformant-variant cfd9231, ps3-batch-shape-finding d9cfb0e | (this commit) |
| outside-author-extension-surface | approved | documented-extension-surface d6496cd | (this commit) |

## Surviving findings

For each slice whose verdict is `changes-requested`, the findings that survived the in-slice fix
pass, with the `file:line` evidence the reviewer cited. These are the prescription a resumed run —
or a human — starts from. They are hypotheses for the next reviewer to verify, not facts to trust.

### reset-and-rebuild-rules

1. **Issue:** BLOCKING — reset-rules AC-005 is not satisfied, and the slice cannot be sealed with it
   open. `fresh_projection_has_no_checkpoint` is absent from
   `crates/happenstance-testkit/src/projection.rs` (the HELD block occupies :1422-1448), from the
   enumeration (:1868-1872), and `PresumedLiveCheckpointStore` is out of
   `tests/projection_mutation_coverage/mutants.rs` and the REGISTRY
   (`projection_mutation_coverage.rs:688-697`). The hold is CORRECT — EC-001 fired, no atom under
   `.kb/decisions/` widens PS-19, and writing the rule would convict adapters of an obligation the
   frozen MUST does not state — but EC-001's handling is "halt and report at the story boundary",
   which is a story that has not finished, not a story that has passed. The projection family lands
   at eleven of §4.11's seventeen adapter rules where the spec's PR boundary said twelve.
   **Fix:** Do NOT restore the rule to clear this review — that is the frozen-clause widening EC-001
   forbids and the reason the last review bounced. Escalate to the human with the two paths already
   written up at `_slices.md:200-234`, and take whichever they authorise: (a) route the repair
   upstream — a new accepted decision atom widening PS-19 or minting the never-seen-id clause, under
   `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`, owned by
   `projection-decision-atoms` / `unstable-projection-gate-and-clause-disposition` — and re-plan this
   story to land after it, so AC-005 has a clause behind it; or (b) record a named human decision to
   proceed on the unmet precondition, discharging every obligation `_slices.md:214-228` already
   enumerates, including naming who authorised it and stating in the same place what the suite then
   asserts that no clause states.

2. **Issue:** Stale docs created by this slice, contradicted by an assertion the same slice landed —
   the fourth occurrence of the class the fix pass itself escalated as "the third occurrence in three
   slices". `assert_reference_projection_declensions`
   (`crates/happenstance-testkit/tests/mutation_coverage.rs:3560-3576`) pins the reference run's skip
   set to exactly two rules, yet: `crates/happenstance-testkit/tests/projection_conformance.rs:24`
   still says "One rule in this run is answered by a `SKIP`" and names only
   `failed_commit_leaves_both_unchanged`; `crates/happenstance-testkit/src/fixtures.rs:460-462` still
   says the suite "prints one `SKIP` line for `failed_commit_leaves_both_unchanged`";
   `crates/happenstance-testkit/src/projection.rs:2029-2032` says
   "`refused_reset_changes_nothing` is the ONLY thing it can answer with a skip"; and
   `fixtures.rs:441-442` keeps the future tense "would fail `refused_reset_changes_nothing` the day
   that rule lands" for a rule that landed at `10ace94`. All four files are inside this slice's
   declared boundary.
   **Fix:** Correct all four to the two-skip reality and name both declined capabilities
   (`COMMIT_FAULT` and `RESET_REFUSAL`) wherever the count is stated; put `fixtures.rs:441` into the
   present tense citing the landed rule. Since this is now the fourth recurrence, also stop counting
   in prose where a test already counts: have each of these docs point at
   `assert_reference_projection_declensions` as the authority for the set instead of restating a
   number nothing in the gate reads.

3. **Issue:** read-through AC-006's ledger row is marked `satisfied: true` while its own evidence
   field opens "PARTIAL" and states "NOT SATISFIED, literally"
   (`read-through-and-rebuild-rules/_ledger.md:52-53`). The implementation is right — `_design.md`'s
   capability table named the TESTKIT as the reason-writer for `READS_THROUGH_BATCH` at sign-off
   (`0df2c1c:241`, note 1 at :245-248) — so the defect is in the criterion's wording, not the code.
   But a true row whose evidence says false is a record a later reader cannot act on.
   **Fix:** Resolve the conflict in the record rather than in the flag: state in the ledger row (and
   in `_slices.md`'s fix-pass entry) that the AC text contradicts the binding design record, cite
   `_design.md`'s capability table row for `READS_THROUGH_BATCH` as the authority the implementation
   followed, and keep the routed question — should the probe const carry a reason at all — attached
   to the port disposition story. Do not change the code to chase the AC's wording; that would mint
   the projection-local reason DT-3's one stated exception exists to avoid.

4. **Issue:** reset-rules AC-010 requires `git diff` over `spec/SPECIFICATION.md` to be EMPTY, and it
   is not: 18 lines changed across the slice, all `†` markers inside §7.1–§7.2's BEGIN/END GENERATED
   block. Every hunk is generated by `cargo xtask spec-trace --write` and stale-checked by a gate
   step, so the criterion as written cannot be met by any story that adds or removes a rule name.
   This is the same unsettled structural issue as the `standards/rust/**` citation re-pointing, now
   at its fifth instance across three slices (`_slices.md:288-301`).
   **Fix:** Settle it once instead of ratifying it per slice, as `_slices.md` itself argues. Either
   amend the criterion's premise to "no clause text, maturity marker or rule citation outside the
   BEGIN/END GENERATED block changed" — which is what AC-010 actually means and what is verifiably
   true here — or teach the boundary check that a generated-region regeneration and a compelled
   citation repair are not scope breaches. Record whichever is chosen where the next story reads it,
   not in a commit message.

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

### reset-and-rebuild-rules — human decisions, 2026-08-14

**AC-005 / PS-19: PATH (a). The human declined to authorise proceeding on the unmet
precondition.** The hold stands exactly as `064687a` left it — `fresh_projection_has_no_checkpoint`
and `PresumedLiveCheckpointStore` stay out of the rules module, the enumeration, `mutants.rs`, the
`REGISTRY` and `for_each_projection_mutant!`; PS-19 stays un-line-edited; `CHANGELOG.md` keeps the
non-delivery entry; §7.2's PS-19 row keeps its `†`. **Do not restore the rule.** Path (b) was
offered and refused, so there is no authorisation to cite and inventing one is the failure this
whole halt exists to prevent.

The repair is routed upstream, and it is `projection-decision-atoms`' / `unstable-projection-gate-
and-clause-disposition`'s, not this story's:

- A **new accepted decision atom** widening PS-19, or minting the never-seen-id clause, under
  `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`. **Never a line edit** to the
  frozen clause.
- It must reach `status: accepted` under `.kb/decisions/` through **one human-invoked
  `/redkiln:kb-ingest` wave**, exactly as ADR-0017/0018/0019 did in `2026-08-13-projection-adrs`.
  Staging it in `.kb/_intake/` is this project's work; accepting it is not, and no run may
  hand-write the atom.
- `reset-rules` then re-plans to land **after** that atom, so AC-005 has a clause behind it.

Until it lands, `reset-rules` is nine of ten and the projection family is **eleven of §4.11's
seventeen** adapter rules where the PR boundary said twelve. Say that number plainly wherever the
freeze is reported; it is the second named hole, beside PS-1's second conjunct, and
`unstable-projection-gate-and-clause-disposition` inherits both.

**Why this is worth the delay.** The finding was not procedural. `PresumedLiveCheckpointStore` was
registered as a *mutant* and is **conformant** with PS-19 as written — a store that answers
`.unwrap_or(Checkpoint::Live { through: FIRST })` breaks no frozen MUST. The suite was convicting
legal adapters, and CF-5's positive control could not catch it because no conformant variant in the
projection registry models that store. In a project whose product is a suite that can fail, a rule
that fails conformant stores is the more expensive error of the two. **Consider adding that
conformant variant to the registry** when the repair lands, so the positive control can catch the
next one.

**AC-010's instrument amended, premise unchanged.** Recorded in
`reset-rules/spec.md` beneath the AC table: the `git diff … empty` form was unmeetable by any story
that adds or removes a rule name, because `spec-trace --write` regenerates §7.1–§7.2 and a separate
gate step fails on a stale region — the criterion demanded a story leave stale what the gate demands
it regenerate. It now scopes to hunks **outside** the `BEGIN/END GENERATED` region, which is what
its own next clause always said it meant. This is a tightening: the old form was going to be
ratified away per slice, five times so far, each ratification eroding what the criterion forbids.

Findings 2 and 3 stay with run 5: correct the four stale docs to the two-skip reality **and point
them at `assert_reference_projection_declensions` as the authority** rather than restating a count
nothing in the gate reads; and resolve read-through AC-006's `satisfied: true`-over-`NOT SATISFIED`
row **in the record, not the code** — `_design.md`'s capability table named the testkit as
`READS_THROUGH_BATCH`'s reason-writer at sign-off, so the AC's wording is what is wrong.

### Run 5 scope — read this before starting

**`reset-and-rebuild-rules` is deliberately SKIPPED this run** (`resumeSlice:
second-batch-shape-and-evidence`). It is sealed `changes-requested` and cannot be sealed
`approved` by any amount of work inside this project: AC-005 waits on an accepted decision atom
that only a human-invoked `/redkiln:kb-ingest` wave can produce. Re-entering it at Review would
halt on the same finding a third time. **Do not restore the held rule to make it seal.**

**One addition to `unstable-projection-gate-and-clause-disposition` (HS-S0016), which already owns
the PS-19 repair jointly with `projection-decision-atoms`:** stage the repair document into
`.kb/_intake/` this run. Write it, do not accept it — hand-writing an atom under `.kb/decisions/`
is what `0269720` was reverted for, and the wave is the human's to invoke. The document should
specify a new `decision` atom that widens PS-19 to cover the never-seen-id case, or mints the
clause it lacks, under `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` — never a
line edit to the frozen clause — and should carry, as the `2026-08-13-projection-adrs` intake
documents did, the frontmatter conventions, the claim, the rejected alternatives, and the
`.kb/maps/decision-map.md` row. Give the wave id a suffix that collides with neither
`2026-08-10-intake`, `-2` nor `2026-08-13-projection-adrs`.

**Two findings from this slice stay open and are NOT this run's** — they live in the skipped
slice's files and belong with the run that closes it after the wave: the four stale docs
(point them at `assert_reference_projection_declensions` rather than restating a count) and
read-through AC-006's `satisfied: true`-over-`NOT SATISFIED` row (fix the record, not the code).
Do not fix them opportunistically from another slice; that would put slice-5 repairs in a slice-6
commit and leave the boundary check unable to tell them apart.

**What HS-S0016's disposition must state honestly**, whatever else it says: the projection family
lands at **eleven of §4.11's seventeen** adapter rules, and the freeze carries **two named holes**
— PS-19's never-seen-id case, unenforced pending the atom above, and the residue noted for
`arm_commit_fault` with an empty body, which has no fixture-level mutant. PS-1's second conjunct is
**no longer** a hole; the DT-3 amendment closed it.
