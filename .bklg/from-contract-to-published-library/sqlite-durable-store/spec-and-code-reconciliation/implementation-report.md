---
item: "HS-S0047"
stage: implement
created: "2026-08-18"
updated: "2026-08-18"
---

# Implementation Report — The standing reconciliation criterion, discharged

> **STATUS: six of six ACs satisfied. No Rust behaviour changed, nothing
> `[FROZEN]` was amended, and no marker moved.**
>
> The pass found **seven citations that resolved while pointing at the wrong
> subject** and **eighteen passages whose prose had stopped being true** —
> including two `[PROVISIONAL]` falsifiers waiting for an event that had already
> happened, and one sentence the pass wrote itself and the review sent back in
> four places. Twenty-six repairs in total: 18 false prose, 3 pointer-only, and 5
> discharge paragraphs that keep their MUST verbatim. None is deleted, and the
> citation count **rose**.
>
> Three gaps are escalated rather than applied, and the largest of them is the one
> the criterion's second bullet exists to expose: **ADR-0022 states no clause
> range at all**, so the arithmetic had nothing to close against.

## TDD Evidence

This story writes no Rust, so its red/green loop runs through the gate that reads
the artefact rather than through a test suite. Each row below names the instrument
that would have gone red, and what it actually did.

| AC | Instrument | Red, and for what reason | Green |
| --- | --- | --- | --- |
| **AC-001** | `_reconciliation.md`'s verdict table, cross-checked by `cargo xtask spec-trace` | Red by construction on the base tree: **no such table existed**, and four of the eight clauses in the range had never been read against the phase-8 code | 43 verdict rows: 17 unchanged, 26 repaired, 0 silently passed |
| **AC-002** | `cargo xtask spec-trace` | **The honest finding is that it was already green and the citations were already wrong.** Six pointers resolved and named the wrong subject; the checker had no derivable subject for any of them and therefore no opinion. Recorded as finding 1 in the staged intake document rather than papered over | Every one re-anchored; `traceability: no problems found` |
| **AC-003** | `git diff spec/SPECIFICATION.md`, read against the playbook's discriminator | Would go red on any deleted MUST. Two deleted lines contain "MUST" and both were read line by line: one is a table row whose PS-2 column is reproduced word-for-word, the other is prose | 202 insertions / 85 deletions, no normative change, census identical |
| **AC-004** | the two ID sets in `_reconciliation.md` | Red on the first attempt to compute set B: ADR-0022's atom states no clause range, and its queue row is the only one in the queue with none. **No range was synthesised to make it close** | Both sets literal, symmetric difference of ten, one disposition per element |
| **AC-005** | `cargo xtask spec-trace`'s summary line, compared against `53a4764` | **Genuinely red mid-pass**: `checked` held and `anchored` fell 76 → 75, which is EC-004 exactly. Diagnosed to `subject_before`, re-anchored properly — the window was **not** widened | 389 → 401 checked, 76 → 80 anchored |
| **AC-005** | `cargo xtask lints` | Red, and caused by this pass: a constitution atom cites a spec line these insertions moved. One digit repaired | `27 atoms, all consistent` |
| **AC-006** | `spec-trace`'s falsifier-length check; `redkiln validate --kb` | Would go red on a marker whose falsifier fell under twelve characters, or on a hand-written `.kb/` atom | falsifier check green; `redkiln: validate passed` |

**The one thing this story could not falsify, stated plainly.** Twenty-three of
the twenty-six repairs changed a *sentence* and not only a pointer — only three
rows read `repaired, citation only` — and eighteen of those sentences were false
about counts, tenses, a spent falsifier and, four times, about which axes of §6.5
are empty. No extension of `spec-trace` catches "seven" when the answer is
twelve. The
instrument for those is a person reading the clause against the code, which is
what the standing criterion is, and it is why the pass is worth owning rather than
mechanising away. That argument is staged, not settled.

## Commits

- `54df29a` — `feat(sqlite-durable-store): The specification and the code, reconciled`

## Changes

| File | Shape of the change |
| --- | --- |
| `spec/SPECIFICATION.md` | Twenty-six repairs across §1.3, §1.6, §1.7, ES-4, ES-6, ES-11/D7, ES-17, ES-30, ES-41, the append-condition clause, §4.5, PS-5, §6.5's portfolio, CF-4, CF-34 and VT-21 – VT-24. Seven drifted `file:line` pointers re-anchored; four false counts corrected; two "planned" → "landed"; one `[PROVISIONAL]` falsifier restated inside its own level; two discharge paragraphs added, each keeping its MUST verbatim. The review fix added §4.5, §6.5's batch-shape row and §6.5's opening claim, and corrected the "batch shape is empty at both ends" sentence this pass had itself written into four places |
| `.bklg/.../spec-and-code-reconciliation/_reconciliation.md` | **New.** The verdict table, the two clause-range sets and their symmetric difference, the citation figures against `53a4764`, the settled routed item, the mechanical re-run of the widened inclusion rule, and the three escalated gaps |
| `.kb/_intake/0034-what-the-phase-8-reconciliation-cost.md` | **New, staged for `/redkiln:kb-ingest`, explicitly not an atom.** What the pass cost and three findings, the sharpest being that only 80 of 401 citations are under any anchoring discipline |
| `standards/rust/01-standard-of-evidence.md` | One digit, `:5926` → `:5984` and then → `:5993`, because this pass's insertions moved the sentence it cites and `cargo xtask lints` is a `REQUIRED` step. The second move was pre-emptive: the anchor had drifted to 9 lines against an `ANCHOR_SLACK` of 10 |

Untouched, and verified untouched: `crates/**/src/**` (no Rust changed),
`xtask/src/spec_trace.rs` (`git diff` empty — the diagnostic instrumentation was
reverted), `spec/E2E-CASES.md` (it cites this crate nowhere), `RUNBOOK.md`, and
every atom under `.kb/decisions/`, `.kb/concepts/`, `.kb/playbooks/` and
`.kb/open-questions/`.

## Gates

| Gate | Grain | Result |
| --- | --- | --- |
| `cargo xtask spec-trace` | story (`reachability_static`) | `401 citations checked (80 anchored to their subject, 12 external)`; `no problems found; §7.1–§7.2 matches the checker` |
| `cargo xtask lints` | story (`reachability_static`) | red once on a citation this pass moved, then `27 atoms, all consistent` |
| `redkiln validate --kb` | backlog | `validate passed` (exit 0) |
| `redkiln doctor` | backlog | exit 1 on standing advisories only — the six template drifts `CLAUDE.md` records as expected forever, and nine pre-existing foundation-story notes. None names a file this story touched |
| `cargo xtask affected --base main` | story (`affected_gate`) | `affected gate passed` (227 tests) |
| `cargo fmt --all -- --check` | formatter | clean |
| `cargo xtask ci --fast` | integration (`integration_scoped`) | `all required checks passed (--fast: 4 optional step(s) not run)` |

## Notes

**Deviation 1 — `standards/rust/01-standard-of-evidence.md` is outside the
declared PR boundary and was edited anyway, by one digit.** The constitution's own
citation lint uses a ten-line window against `spec-trace`'s twelve, so it caught a
shift `spec-trace` tolerated. Leaving it would have left a `REQUIRED` gate step
red through an edit this pass caused. Recorded in `_reconciliation.md` under *One
file outside the PR boundary, and the reason*, and in AC-005's ledger row.

**Deviation 2 — the routed item was settled, and it moved four statements.**
`reopen-negative-control-and-durability-verdicts` handed over §6.5's
handle-multiplicity row rather than moving it in passing. The verdict is that the
axis **does** have an adapter instrument at its far end — `SqliteFixture::connect`
opens a real second `rusqlite::Connection` and `connect_many` races up to 64 of
them — so the portfolio's adapter column went from one tick to two, and §1.3,
§1.6, §1.7 and §6.5 were moved together so that four places agree rather than one
contradicting three. The narrower reading (only a pool counts) is recorded as
considered and rejected, with pooling and cross-process handles named as what is
still missing.

**Deviation 3 — a temporary diagnostic was added to `xtask/src/spec_trace.rs` and
reverted.** Two `eprintln!`s and a script that swapped the base blob in and out
were the only way to find *which* citation stopped anchoring. `git diff` on that
file is empty. No check was relaxed, widened or disabled, and `ANCHOR_SLACK` is
still 12.

**Review fix — CF-40, and why the first pass missed it.** The slice review found
one more marker waiting for an event that had already happened: CF-40's falsifier
still ended *"no adapter has stated a ceiling yet"*, while `SqliteFixture` states
all three (`crates/happenstance-sqlite/tests/support/mod.rs:189-195`) mirrored
from the adapter's own constants (`crates/happenstance-sqlite/src/event_store.rs:245`,
`:252`, `:261`) — the same three this pass discharged VT-21, VT-22 and VT-24
against. It is the identical defect the pass repaired for ES-41 at `:4457`, and
CF-40 was already sitting in the symmetric-difference table, dispositioned on
clause *ownership* without its marker text ever being read. **The cause is a
mechanical one and is fixed at the rule rather than at the row:** the verdict
table's inclusion rule read "every clause whose *prose* cites
`crates/happenstance-sqlite`", so it was run as a path search, and CF-40 names the
adapter in words ("the rusqlite adapter at phase 8") and cites no path. The rule
now reads on the crate's identity — path **or** name, prose, `Rejects:` **or**
maturity marker. The falsifier is restated inside the same level, ES-41's way,
with the marker unmoved, the MUST verbatim and clause ownership still
`kb-open-question-cf-40-ownership-001`'s. The figures moved with it: 396 →
**398** checked (`anchored` unchanged at 78, `external` at 12), and the diff over
`spec/SPECIFICATION.md` 152/72 → **166/75**, still with exactly two deleted lines
containing a lower-case "must" and neither of them CF-40's. Review fix 2 below
moved them once more, to **401/80** and **202/85**, which is what the tables above
now read; each figure is quoted at the fix that produced it rather than
overwritten, because a figure with no fix attached is the drift this story is
about. CF-34's cited range in the verdict table shifted eleven lines with the
insertion and was repaired in place.

**Review fix 2 — §6.5's batch-shape row, and the sentence this pass wrote that
was false when it wrote it.** The second review found the same class again, and
this time on both sides of the ledger. (a) §6.5's **Batch shape** row was
unrepaired and false in three statements at once — `SqliteProjectionStore` "in
`begin` and `rollback`", "still there is no suite to run any of them against",
and an instrument column asking for "the projection conformance suite, which does
not exist" — every one of them contradicted by *this same pass's* repairs at
`:391` and `:4665`, by `crates/happenstance-sqlite/src/projection_store.rs:552`,
`:566`, `:592`, `:673`, and by `crates/happenstance-sqlite/tests/projection.rs`.
It was also the explicit hand-off recorded at
`projection-store-passes-the-borrowed-suite/_ledger.md:98`, so it arrived named.
(b) Worse, the pass **affirmatively wrote** "transport, batch shape and
completeness are empty at both ends" into four places — `:232-249`, `:390`,
`:472` and `:8545` — contradicting its own §7 text. Under AC-003's discriminator
that is not a repair but a newly authored false claim, and it is recorded as the
eighth instance of the defect class in AC-001 rather than quietly corrected. All
five sentences now name transport and completeness as the axes empty at both ends
and state batch shape's *asymmetry*: a passing implementation at the
replay-at-commit far end, nothing at the live-transaction near end — which no
rusqlite adapter can supply on the `Send` flavour
(`crates/happenstance-sqlite/src/projection_store.rs:19-43`) — and PS-2 uncleared
because it wants both. **The method fix is the one the review asked for:** the
widened inclusion rule is now *run* mechanically, as one `grep -nE
"happenstance-sqlite|rusqlite|Sqlite[A-Za-z]*|phase 8"` over the specification,
and all 89 hits are dispositioned — 56 inside changed passages, 33 outside, of
which eleven get their own **unchanged** row with the reason they needed none.
Two of those eleven are the ones the review named (`:3659`, `:4289`) and both are
checked against code rather than waved past. §4.5 was repaired in the same sweep:
it cited a spelling ADR-0017 deleted and said a runner "can write" a cross-store
commit that PS-15's stamp now refuses.

**Not done, deliberately.** No committed citation baseline, no new gate step, no
machine-readable clause-range format, and no edit to
`.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md`. Running the
criterion once by hand answers none of its five sub-questions, and the arguments
this pass produced for mechanising parts of it are staged under `.kb/_intake/`
for `/redkiln:kb-ingest` to adjudicate.
