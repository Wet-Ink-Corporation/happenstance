# Wave `2026-08-10-intake-2` — corpus match

What already exists in `.kb/`, what each incoming claim could have merged into, and the score
behind every disposition. Read this before `02-placement-and-adjudication.md`: the placements
there are only defensible because of what this file establishes about the corpus and about the
intake.

This is the wave the previous one named. `2026-08-10-intake` closed with the sentence
*"Importing the ADR corpus is a separate wave with a human in it"*, and left `.kb/decisions/`
empty on purpose. The seventeen files of this wave **are** that corpus.

## The corpus, as verified

```
Glob .kb/**/*.md  →  17 atoms + 6 layer READMEs + .kb/README.md + _templates/atom.md
                     + .kb/_intake/README.md + the seventeen files of this wave
```

| Layer | Atoms | Ids |
| --- | --- | --- |
| `decisions/` | **0** | — (README only) |
| `reference/` | 1 | `kb-reference-phase-4-5-spec-reconciliation-001` |
| `playbooks/` | 4 | `kb-playbook-verify-referent-report-coverage-001`, `kb-playbook-anchoring-citations-001`, `kb-playbook-ratchet-gate-landing-001`, `kb-playbook-repair-frozen-clause-001` |
| `open-questions/` | 7 | `…-disjoint-boundaries-no-clause-001`, `…-model-family-rule-no-clause-001`, `…-ps-1-no-progress-obligation-001`, `…-ps-19-scope-narrower-001`, `…-es-6-unwritable-rule-001`, `…-provisional-falsifiers-001`, `…-post-phase-reconciliation-001` |
| `maps/` | 2 | `kb-map-domain-001`, `kb-map-open-questions-index-001` |
| `product/`, `design/` | 0 | READMEs only |
| `concepts/`, `governance/`, `narratives/`, `roadmaps/` | — | **do not exist** (`ls`, 2026-08-10) |

So, for authority purposes:

- **There is still no accepted `decision` atom.** Nothing in `.kb/` can be conflicted with,
  superseded, or drifted by this wave. Authority rule 1 is satisfied vacuously *against the
  corpus* — but not against the intake, which arrives carrying its own supersession chain
  (ADR-0002 → 0005 → 0006 → 0007) and its own amendment (0004 ← 0029). Adjudicating that chain
  without collapsing it is this wave's authority work, and it is in `02`.
- **Two merge targets do exist**, and both are `open_question` atoms rather than decisions, so
  amending them is permitted. Both were left explicitly waiting for this wave — see the table
  below.
- **No `map` atom needs inventing this time.** The domain map and the open-questions index were
  created by the last wave's Maps phase and both carry an "Adding a …" section that says how to
  extend them. A **decision map** is named by `maps/README.md` as belonging in that layer and
  does not exist; twenty-one decision atoms land this wave, so the Maps phase must create it.

## Provenance check

Every one of the seventeen intake files is a **byte-identical copy of a file that still exists
in `.kb/decisions/`** (`diff -q`, spot-checked on 0001, 0016 and 0029, 2026-08-10; all silent). That
single fact governs more of this plan than any score in it:

- `.kb/decisions/` is the canonical, human-signed ADR corpus and is **not** consumed by this ingest.
  A successful ingest deletes `.kb/_intake/`; `.kb/decisions/` is untouched.
- The intake files run **302 to 15,305 words** (`wc -w`). Eleven of the seventeen exceed the
  900-word atomicity ceiling, four by an order of magnitude. A `.kb` decision atom therefore
  **cannot** be the ADR; it is a summary, its commitments, its rejected alternatives, and a
  pointer to `.kb/decisions/NNNN-*.md`. That is the reference README's pointer rule applied to the
  decisions layer, and it is why no ADR is split into three atoms to make the words fit — see
  `02`, "One atom per ADR".
- Every `source_paths` entry proposed in `02` was tested against this worktree. All resolve:
  `.kb/decisions/` (17 files), `spec/SPECIFICATION.md`, `spec/E2E-CASES.md`, `RUNBOOK.md`,
  `CLAUDE.md`, `references/evaluation/{phase-4-reconciliation,phase-4-5-reconciliation}.md`,
  `references/adapter-shapes.md`, `experiments/{position-visibility,wire-format}/`,
  `crates/happenstance-core/src/{event,identity,limits,store,append,projection,memory}.rs`,
  `crates/happenstance-testkit/src/{fixtures,registry,concurrency,model}.rs`,
  `crates/happenstance-sync/src/{identity,wire,ingest}.rs`.
  Note that `CLAUDE.md` cites the specification at `docs/architecture/SPECIFICATION.md`; the
  file is at `spec/SPECIFICATION.md` in this worktree, and the atoms use the path that resolves.

### One intake claim was resolved by reading the tree rather than filed

The ADR-0013 extract raised a flag (`0013-3`) that `SequencePosition::next`'s move from
`saturating_add` to `checked_add` might already be in the tree, which would make the ADR's
"owed to the code" item already discharged, and proposed an `open_question` if the wave could
not tell. It can tell:

```
grep -n "checked_add\|saturating_add" crates/happenstance-core/src/event.rs
  273:  // `checked_add`, not `saturating_add`. Saturating made the one method
  278:  match self.0.get().checked_add(1) {
  835:  /// VT-13. `saturating_add` clamped, `u64::MAX` is non-zero, and `new`
```

**Discharged, verified 2026-08-10 at `event.rs:278`**, with the `match` spelling ADR-0013 asked
for and the reasoning preserved in the comment above it. No open question is filed; the
verification is recorded in the ADR-0013 atom and the existing census atom's phrasing is left
alone, because it was describing a repair that had already landed rather than one that was owed.

`crates/happenstance-core/src/identity.rs`, `limits.rs` and `crates/happenstance-sync/src/wire.rs`
all exist, so ADR-0014, ADR-0015 and ADR-0016 describe shipped code and not intentions.

## Scoring method

Unchanged from `2026-08-10-intake`, deliberately — a threshold that moves between waves is not a
threshold. No deterministic engine exists in this repo, so the score is stated rather than
computed: a 0–100 judgement of **subject identity** — would a reader looking for one claim expect
to find the other in the same atom — cross-checked against theme-keyword overlap from the extract
digests.

- **≥ 80 — same knowledge.** Collapse into one atom. Never two atoms.
- **50–79 — same subject, different method or different settling condition.** Two atoms, linked.
  The reason must be written down.
- **< 50 — related.** A `related` link and nothing else.

One rule is added this wave, because the intake is a decision corpus and the last one was not:

- **A supersession chain is never a merge.** Two ADRs about one subject, where the later reverses
  part of the earlier, score 90+ on subject identity and must still be two atoms. Collapsing them
  is the exact act the corpus forbids — `.kb/decisions/README.md` ("its body is never reworded"),
  ADR-0005 §*On the historical record*, and CONTRIBUTING.md all say supersede rather than rewrite.
  Subject identity is the wrong instrument here; the right one is whether a human signed the two
  documents separately.

## Merge candidates against the existing corpus

Two of the seven existing open questions were left pointing at ADRs that had not been imported.
Both are now merge targets, and both are the wave's clearest `merge_existing` dispositions.

| Existing atom | Incoming claims | Score | Disposition |
| --- | --- | --- | --- |
| `kb-open-question-es-6-unwritable-rule-001` | `0009-C2` (ADR-0009 corrects ES-6's premise and makes the rule writable), `0008-C6` (the rule must assert on the future's `Output`, not the future) | **88** | **merge_existing.** The atom's own body already says *"ADR-0009 is accepted and makes the rule writable … ADR-0009 is named here by id only; it is not imported as a decision atom in this wave."* That sentence is now false, and the fix is a link, not a new atom. |
| `kb-open-question-provisional-falsifiers-001` | `0001-claim-2` (the ES-7 falsifier's own instrument compiles clean), `0008-C3` (the same, compiled, plus the coherence claim), `0014-provisional-exposure` (VT-9's falsifier is owned by phase 9's Workers skeleton) | **82** | **merge_existing.** The atom already narrates ES-7's `LocalMemoryEventStore` evidence with no decision to cite; three ADRs supply the citation and one supplies VT-9's owner. |
| `kb-reference-phase-4-5-spec-reconciliation-001` | every ADR's "amendments this decision owes the specification" ledger | 30 | **link only.** The census is a snapshot of one pass at one commit; seven amendment ledgers are not the same measurement and the reference README bars a mirror somebody has to keep current. |
| `kb-playbook-repair-frozen-clause-001` | `0015-C11`, `0008-C8`, `0012-C10`, `0013-1` amendment sections | 45–55 | **link only.** These are *instances* of the playbook's method, and the playbook already carries worked examples. Adding four more would grow it past atomicity for no new method. |
| `kb-open-question-ps-1-no-progress-obligation-001`, `…-ps-19-scope-narrower-001` | `0007-c7` (the projection-store apply seam) | 20 | **link only.** Same port, different defect: those two are about clause scope, this is about a missing trait bound. |
| `kb-open-question-disjoint-boundaries-no-clause-001` | `0012-C6` (ES-25's `conflicting_position`, guards) | 32 | **link only.** ADR-0012 generalises ES-25 across guards and does not state the independence proposition the question is about. |
| `.kb/decisions/README.md`, `.kb/open-questions/README.md` | `0005-c3`, `0006-C8`, `0007-c8` | 55–85 lexical | **not a merge target.** A README is a layer contract, not an atom; the previous wave established this and it holds. Where a claim generalises a README's rule, the atom grounds it and cites it — see the governance atom in `02`. |

## Cross-file clusters

Seventy-plus claims arrived across seventeen files. The clusters below are where two or more
**different files** carry the same knowledge; everything not listed is single-file.

| Cluster | Claims absorbed | Files | Score | Disposition |
| --- | --- | --- | --- | --- |
| **CL-A** — what the compiler said about the port traits | `0001-claim-2`, `0008-C3`, `0009-C1`'s four findings, `0010-C3`'s two corrections, `0011`'s E11 | **5** | **85** | one `reference` atom |
| **CL-B** — rewrite the referent, never the reasoning | `0005-c3`, `0006-C8`, `0007-c8` | 3 | **86** | one `governance` atom |
| **CL-C** — an "and" in an ADR title hides a second decision | `0007-c6`, with `0005` and `0006` as its two prior instances | 3 | **80** | one `playbook` atom |
| **CL-D** — the status vocabulary the schema cannot express | `0003-c2`, `0001`, `0004-C0`, `0005-c1/c2`, `0006`, `0011`, `0012-C0`, `0014`, `0015-C8` | **9** | **81** | one `open_question` atom |
| **CL-E** — ES-6 is writable now but unwritten | `0008-C6`, `0009-C2` | 2 | **88** | merge into an existing atom |
| **CL-F** — the two provisional markers' falsifiers | `0001-claim-2`, `0008-C3`, `0014-provisional-exposure` | 3 | **82** | merge into an existing atom |
| **CL-G** — a rule named by two ADRs and scheduled by neither | `0013-9`, `0011`'s ES-9 disposition | 2 | **84** | one `open_question` atom |
| **CL-H** — who owns the fixture's numeric limits | `0015-C9`/`0015-C10`, `0012`'s CF-39 precedent | 2 | **76** | one `open_question` atom (a real conflict, deferred) |
| **CL-I** — crates.io reservation still outstanding | `0005-c5`, `0006-C5` | 2 | **90** | folded into both decision atoms; **no** question atom — see below |

### CL-A is the wave's largest merge, and it is a five-way one

Five files independently record findings from compiling the same two traits. The anchor is a
single fact stated three times in three different documents:

| Pair | Score | Evidence |
| --- | --- | --- |
| `0008-C3` vs `0010-C3` | **95** | Both state *"`RefCell` is `Send` (it surrenders `Sync`); `Rc` is what does the work"* and both conclude CF-28's wording must name it. Identical finding, identical remedy, two ADRs one day apart. Two atoms here would be two copies of one measurement — the failure the reference README names by name. |
| `0009-C1` vs the above | **80** | ES-6's premise is half wrong for the same reason: `JsValue` is `Send + Sync` on non-atomics `wasm32`; `Rc` in `happenstance-cloudflare`'s error is the real hazard, found by a workspace-wide `cargo check`. Same fact, third instrument. |
| `0001-claim-2` vs `0008-C3` | **82** | Both record `LocalMemoryEventStore` passing all twenty-seven rules natively and under `wasm-bindgen-test`, and both record the blanket impl sitting beside a downstream direct impl with no `error[E0119]`. ADR-0008 *is* the document that lifts ADR-0001's marker on that evidence. |
| `0011`'s E11 vs `0001` | **70** | Below the merge bar on its own — a different experiment, run three days later — but it *corrects* ADR-0001's stated consequence that `dynosaur` erases the port (`error[E0277]`; a hand-written wrapper works with no `unsafe`). A correction to a fact this atom already owns cannot live anywhere else without splitting the fact from its correction. |
| `0010-C3`'s `#[tokio::test]` finding | 78 | *"`#[tokio::test]` already drives a `!Send` store"* (it expands to `Runtime::block_on`, not `tokio::spawn`) is recorded identically in `0008-C3`. Same reason as row 1. |

`0011`'s E10 (the escaping-stream compile matrix) and E12 (`Unpin` forbids a generator-backed
stream) are **not** in CL-A: they establish nothing any other document states, they are the
grounding for exactly one decision, and they are two sentences each. They stay named in the
ADR-0011 atom with a pointer to `references/evaluation/phase-4-reconciliation.md`.

### CL-D — nine files, one representational gap

`KbFrontmatter.status` is `draft | proposed | accepted | superseded | withdrawn`. The imported
corpus uses two values it does not have:

- **"accepted — provisional"**, on ADR-0001 (lifted), ADR-0003, ADR-0004, and in named parts on
  ADR-0011 (two clauses), ADR-0012 (ES-17, CF-39), ADR-0014 (four parts) and ADR-0015 (five
  clauses). Each carries a falsifier and a lift condition, and both are load-bearing: the
  provisional preamble on ADR-0003 says in terms that work contradicting it *"does not owe
  deference to a decision the code has not yet voted on"*. That is a different authority than
  `accepted`, and mapping it to `accepted` loses it.
- **"partly superseded by"**, on ADR-0005 and ADR-0006.

The four extract agents that met this each proposed a local fix and one (`0003-c2`) explicitly
asked the adjudicator to dedupe rather than file four near-duplicates. Scored against each other
the nine instances run 78–88: one root cause, two symptoms. **One atom.** It is deliberately
distinct from `kb-open-question-provisional-falsifiers-001`, which is about `[PROVISIONAL]`
markers on *specification clauses* and their falsifiers — this one is about the *frontmatter of
an imported ADR* and how the KB should carry it. The two are linked and the distinction is stated
on the new atom, because a reader who finds one will otherwise assume it covers the other.

### CL-I — the merge that produced no atom

`0005-c5` ("Reserve the name") and `0006-C5`'s neutral consequence ("crates.io reservation for
`happenstance` and `happenstance-core` both still outstanding") are the same follow-up, scored 90.
The ADR-0005 extract proposed an `open_question`. **Refused**, on the open-questions README's own
exclusion: *"Nor does a task belong here. Work that someone is expected to do is a backlog item in
`.bklg/`, not a KB atom."* Reserving a crate name is work, not an unknown. It is recorded in both
decision atoms as the ADRs' own recorded follow-up, and carried in `unresolved` so that the
absence of a backlog item is visible to a human rather than silently absorbed.

## The dedup that was refused: seventeen ADRs stay seventeen atoms

The single largest available "merge" in this wave is the one that must not happen. Scored on
subject identity alone:

| Group | Score | Why not merged |
| --- | --- | --- |
| ADR-0002 + ADR-0005 + ADR-0006 — crate naming | **92** | One subject, three human sign-offs, two supersessions. The corpus's rule is explicit and self-applied: ADR-0005 kept ADR-0002's body verbatim *because rewriting `eventum` → `happenstance` inside it would invert a factual claim into a falsehood*. A merge is a rewrite with extra steps. |
| ADR-0004 + ADR-0029 — the MSRV | **90** | ADR-0029 says in its own header that it **amends** rather than supersedes, and that ADR-0004's reasoning is *what it acted on*. Merging would delete the invitation that produced the amendment and leave only the number. |
| ADR-0006 + ADR-0007 — where the projection runner lives | **85** | ADR-0007 partly supersedes ADR-0006 and explicitly preserves the other half in place. Merging orphans a standing commitment. |
| ADR-0008 + ADR-0009 — the derivation and the error bound | **72** | Below the bar anyway, and ADR-0009's header records that ADR-0008 *"recorded two facts for this decision and deliberately did not take it"*. Two decisions, separately taken, separately reversible. |
| ADR-0011 + ADR-0012 + ADR-0013 + ADR-0014 + ADR-0015 — the phase-4 contract freeze | 55–65 | One phase, five questions, five sign-offs. The RUNBOOK's ADR ledger lists them as five rows with five distinct clause ranges. |

The converse refusal is in `02`: several extract digests proposed **splitting** one ADR across
two or three atoms (ADR-0011 into three, ADR-0015 into three). That is refused too, and for a
reason that is not symmetry — `adr_id` and the supersede pair have to identify one atom, and a
15,000-word ADR is not made atomic by being summarised in three files instead of one.
