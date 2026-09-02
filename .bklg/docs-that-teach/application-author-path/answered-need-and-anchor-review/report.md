---
item: "HS-S0190"
stage: report
created: "2026-08-19T00:00:00.000Z"
updated: "2026-08-19T00:00:00.000Z"
---

# Report — Each page's answered need and every anchor reviewed

## Findings Ledger

**Eight of eight criteria satisfied. Four surfaces walked, four `pass`, no row at `fail` or
`indeterminate` — and the procedure that produced those four verdicts was made to return `fail`
twice before any of them was written down.** That ordering is the whole claim: a walk that has
never returned `fail` is decorative, and four rows of `pass` from one would be an impression
wearing a table's clothes.

The finding a reviewer should read first is the one nobody was looking for. AC-001's second
cross-check — meant as a formality — showed that `docs/carry-your-invariant.md` and
`docs/read-the-worked-example.md` are absent from the tree's own narrative index, and the inbound
sweep then showed they link **only to each other**. Two of the four surfaces this project
authored are an isolated component with no reader-facing route in.

| Finding | Evidence | Follow-up |
| --- | --- | --- |
| **Four surfaces, four `pass`.** Exactly one declaration each, in HS-P0021's exact form, from the closed four-token set, above the first fence in reading order on every surface that has one | `_walk.md § 3`; `git grep -c '^> \*\*Answers:\*\*'` → 1 per page; head regions captured showing nothing interposed; declaration line vs first-fence line tabulated | none — this is the expected result, and it is the one the calibration makes meaningful |
| **The procedure was demonstrated able to return `fail`, in both directions.** HS-P0021's inert specimen → `fail — two needs`; a second declaration injected into a live page of this set → the walk **and** the gate step both fail, then both green after a revert shown by hash | `_walk.md § 5`; ``docs/carry-your-invariant.md:4 — declares `explanation` and `how-to`; a page answers one need`` | none. EC-006 did not fire |
| **Two of the four surfaces are unreachable.** `docs/carry-your-invariant.md` and `docs/read-the-worked-example.md` are absent from `docs/README.md`'s narrative index, are linked by neither the crate root nor the opening encounter, and link only to each other. The keyboard traverse `crate root → bridge → #where-your-streams-went` does not complete | `_walk.md § 1` cross-check 1 and `§ 6` traverse; `git grep -n "carry-your-invariant" -- docs crates examples standards xtask` returns one page link, two test constants and the harness line | **W-1 → HS-P0023** `reach-and-adapter-path`. HS-S0188 deferred the same row for the same reason (`surface-course-subscriptions/report.md:50-53`); what the last slice adds is that the deferral, taken twice, isolated a *component* rather than a page |
| **Nothing in the repository reads the crate root's answered-need line.** It is outside `TREE = "docs"`, so `every page declares one need` never opens it; the assertion at `xtask/tests/first_encounter.rs:184` is a story's test, not the discipline's instrument | `_walk.md § 4` coverage table, middle column | **W-4 → HS-P0021.** Agrees with the slice-mate's R-2 and R-5 about the same file, for the same structural reason, and the two tables were coordinated before either was written |
| **HS-P0021's form and walk have no defined behaviour on a rustdoc crate root.** RP-00-2 anchors the declaration to a markdown `# Title` and there is none; RP-40-1 step 3 asks whether every *section* serves the need, and the medium requires sections band 10 excludes from the need set by name | `_walk.md § 3`, the two scopings, stated in full so they can be overturned | **W-2, W-3 → HS-P0021.** No variant notation was invented — inventing one to unblock a criterion is DR-14's defect in its worst form |
| **The prior model is named on exactly one page, in exactly one section.** All four phrases hit only `docs/carry-your-invariant.md:19`, `:20`, `:22`, every one inside the `## Where your streams went` span `:17-29`. Zero on the crate root, zero on any step of the opening encounter | `_walk.md § 6`, full output with line numbers; anti-pattern 6 also run in its reviewer-performable form against the rendered pages | none. EC-009 did not fire |
| **The one recorded location exists exactly once and every relying page cites it.** `## Where your streams went` at `:17`, one occurrence; one relying page, `docs/read-the-worked-example.md:12`, and its link resolves | `_walk.md § 6`; also asserted mechanically by `examples/course-subscriptions/tests/reach.rs:42` | none |
| **No page re-argues the decision.** The single relying page's verdict is `cites`, recorded as a judgement with the sentence quoted verbatim | `_walk.md § 7` | none — no repair was needed, so AC-005's sweep did not have to be re-run |
| **EC-004's 23-character heading was measured and deliberately not renamed.** The 22-character budget derives from rustdoc's 200px sidebar and `tension-resolutions/_resolutions.md:338-343` scoped it to `crate-root-encounter` alone | `_walk.md § 6` | none — recorded, and the disposition was already decided. Renaming would break every inbound link AC-006 exists to prove resolves |
| **The corpus constant is `xtask::lint_narrative::TREE`, not `xtask::narrative::TREE`** as HS-P0021's spec names it. Present, single, and `lint_pages` declares no `PAGE_DIR` | `_walk.md § 4`, pasted `git grep` output; `xtask/src/lint_pages.rs:443-444` | **W-5 → HS-P0021**, a citation correction. EC-003 halts on the constant being *absent*; it is present, so this records |
| **A consumption-map row predicts a crate-root link the tree correctly does not carry** | `tension-resolutions/_resolutions.md:404`; `boundary-refusal-encounter/spec.md:418`'s conditional | **W-6 → HS-P0025**'s reference reconciliation. Not a defect — HS-S0185's obligation is conditional on *relying* on the decision, and the crate root names no prior model at all |

**Mount point.** `xtask/src/narrative.rs`, because membership in the set this story makes a claim
about is decided there. Both halves observed: all three tree pages are named at `:135`, `:141`,
`:150` — no registration line was needed and none was added — and the fourth surface is recorded
in the coverage table **as outside the corpus with its reviewer named**, which is what the
integration contract asks for in place of a registration line it cannot have.

**Nothing deferred, nothing stubbed, nothing absorbed.** Six findings, six named destinations.
EC-001, EC-002, EC-003, EC-006, EC-008 and EC-009 did not fire; EC-004 and EC-005 fired and are
recorded with dispositions; EC-007 held — the one repair the walk found is two index rows carrying
orientation copy, which exceeds one-line grain and is HS-P0023's by this story's own clarification
7, so it stopped and routed rather than being made here.

## Acceptance

| AC | Status | What proves it | Where it lives |
| --- | --- | --- | --- |
| **AC-001** — the set enumerated before it is walked, cross-checked twice | **Met** | Four ids from `_design.md:45-65` resolved to paths, all four existing; both cross-checks pasted; **both** set differences computed and printed, with the one that is a finding recorded as W-1 rather than quietly appended | `_walk.md § 1` |
| **AC-002** — a non-author walks RP-40-1, exactly one declaration, exactly one of four verdicts | **Met** | Six steps answered per page, not summarised; form, position and token checks run as commands first; four `pass`, none at `fail` or `indeterminate`; the crate root's two scopings disclosed so they can be overturned | `_walk.md § 2`, `§ 3` |
| **AC-003** — coverage per page, never averaged; the step run, its corpus named | **Met** | Three-column table with `crate-root-encounter`'s middle column reading **no** and routed as W-4; `lint-pages` output pasted; the corpus shown by `git grep` rather than assumed; no `PAGE_DIR`, and this story's own `xtask/` diff empty | `_walk.md § 4` |
| **AC-004** — the procedure demonstrated able to return `fail`, in both directions | **Met** | (a) the inert specimen → `fail — two needs`, inertness proved by `ls`; (b) four pasted outputs in order — injected diff, the step failing by `path:line`, the revert shown by hash with a clean tree, the step green again | `_walk.md § 5` |
| **AC-005** — the prior model named on exactly one page | **Met** | All four phrases swept over the five files AC-005 names, output pasted with line numbers; every hit inside the `:17-29` span, captured separately; zero elsewhere; anti-pattern 6 also run in its rendered form | `_walk.md § 6` |
| **AC-006** — every relying page links the one recorded location, and it resolves | **Met** | The heading exists exactly once at `:17`; one relying page, and its link resolves both by hand and by `reach.rs:42`; `documentation` step green under `-D warnings`; zero literal bracket pairs on the render. The keyboard traverse **does not complete** and is recorded as W-1 — a defect of reach, not of the anchor | `_walk.md § 6` |
| **AC-007** — cites or re-argues, as a judgement with the sentence quoted | **Met** | One relying page, verdict `cites`, sentence quoted verbatim with the three things it does and the three it does not; the other two surfaces recorded as *does not rely* with the phrase counts that establish it | `_walk.md § 7` |
| **AC-008** — the recorded result, dated, attributed, with every routed finding named | **Met** | An eight-column per-page table in `_ledger.md`'s body; vacuous and not-applicable cells written in those words; six findings each naming its destination; `git status --porcelain` empty; `affected gate passed`; `all required checks passed` | `_ledger.md § Recorded result` |

**Deferred: none.** Every routed finding is a pointer-policy, rule-text or record question owned
by a named item outside this story's PR boundary. No AC is met by a stub, a placeholder notation
or a skipped check, and no verdict was softened to let the story complete — where softening was
the alternative, the scoping was written down instead so a reviewer can reject it.

## Knowledge Harvest

Four candidates for `.kb/` at closeout (HS-P0025's, not authored here — `project.md:144-147`).

1. **Deferring the same small thing twice can isolate a component.** HS-S0188 correctly routed
   *its* index row to the pointer-policy owner; HS-S0187 had no index row to route. Neither
   decision was wrong and the result is two authored pages no reader can reach. The transferable
   rule is that a per-page deferral needs a set-level re-check before the set is called done — and
   that the re-check is a graph question (*is every authored page reachable from the front door?*)
   rather than a per-page one.

2. **A written procedure needs its own conformance fixture, and it needs to be inert.**
   `standards/pages/examples/two-needs.md` costs nothing, can never turn a gate red, and is the
   only reason four `pass` verdicts are worth reading. The generalisation of CLAUDE.md's *name a
   plausible wrong implementation* is that **a procedure ships with the specimen that makes it
   fail**, stored outside the corpus it governs so it can stay permanently broken.

3. **A discipline written for one medium acquires an undefined edge the first time it is applied
   to another.** RP-00-2 anchors a declaration to a markdown `# Title`; RP-40-1 asks whether every
   *section* serves the need. Neither has a defined answer on a rustdoc crate root, where the
   heading is generated and several sections are the medium's. The honest response is to state the
   substitution and route the rule gap — not to answer the question the rule did not ask, and not
   to soften the verdict.

4. **A CRLF working copy silently defeats `$`-anchored patterns in re-derivation commands.**
   `git grep -n '^## Where your streams went$'` returns nothing on a Windows checkout while the
   heading is present. A record whose commands assume LF reads as evidence that the thing is
   missing. Re-derivation commands in a cross-platform repository should avoid trailing anchors,
   or say which line ending they assume.
