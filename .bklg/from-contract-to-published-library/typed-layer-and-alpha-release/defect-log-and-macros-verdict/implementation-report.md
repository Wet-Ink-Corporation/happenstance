---
item: "HS-S0032"
stage: implement
created: "2026-08-16"
updated: "2026-08-16"
---

# Implementation Report — The contract defect log, and the happenstance-macros verdict

**All ten ACs are satisfied. Nothing is blocked and nothing is deferred.** Two records
landed, no code changed, and the diff is what proves the second: no path under `crates/**`,
no path under `spec/**`, no `.rs` file and no `Cargo.toml`.

**The macros verdict came back `out`, contradicting the design's own prediction by roughly
a factor of five.** That is a successful outcome of this story rather than a problem with
it — `_design.md:1104-1111` wrote 2.4:1 down as *falsifiable* precisely so a measurement
could falsify it, and §8 of this story's spec says a contradiction is what recording "either
way" means.

## TDD Evidence

**There is no red-then-green here, and pretending otherwise would be the dishonesty this
story is about.** Both criteria are *"Record, not a test"* in the testing brief's own
taxonomy (`_decomposition.md:790-791`): every gate step in this repository stays green on an
empty log and a verdict that reuses the prior. The spec says so in terms, and the merge-gate
table says the honest statement is *"the gate proves the boundary and the baseline, and the
ledger plus review prove the record."*

What replaces red/green is **three re-runnable checks**, each of which a reviewer can
execute against the committed tree and each of which failed at least once during
authoring — which is the closest thing to a red step this story admits.

**1. The partition check (AC-008), and it did fail.** The classification's totals are not
asserted; they are re-derived from the published table by a script that reads the `| a–b | n
| bucket |` rows out of the record itself, asserts contiguity and no overlap, asserts each
row's `n` equals `end - start + 1`, and compares the sum against `wc -l` on the subject:

```console
rows: 29
buckets: {'ceremony': 40, 'domain': 249, 'neither': 158, 'contested': 85}
covered 532 of 532 lines (wc -l)
contested -> ceremony  ceremony=125 domain=249 ratio=0.50 -> OUT
contested -> domain    ceremony= 40 domain=334 ratio=0.12 -> OUT
```

The first draft of the table did **not** sum to 532 — three handler ranges were counted at
the wrong boundary — and the check is what said so. A number nobody can re-derive is the
failure mode AC-008 exists to close, and this is the instrument that closes it.

**2. The entry-shape check (AC-002).**

```console
$ rg -c '^### D-'            references/evaluation/phase-7-contract-defects.md   # 5
$ rg -c '^\*\*Clause:\*\*'   references/evaluation/phase-7-contract-defects.md   # 5
$ rg -c '^\*\*Attempted\.\*\*' …                                                 # 5
$ rg -c '^\*\*Contract\.\*\*'  …                                                 # 5
$ rg -c '^\*\*Why a defect'    …                                                 # 5
$ rg -c '^\*\*Routing\.\*\*'   …                                                 # 5
```

The `Clause:` count equals the entry-heading count, which is the assertion AC-002 names.
This too was red first: the entries were authored as `##` headings and the six labels as
run-in bold, and restructuring them to `###` is what made the check land rather than
approximately land.

**3. The density check (NF-003, and the design's own budget).** Prose wraps at 80 columns,
tables and fences exempt; each entry ≤ 40 lines, and what yields when it is exceeded is the
commentary. It was red on both records — 104 prose lines over budget in the defect log, and
three entries over 40 lines — and the commentary yielded, exactly as
`_design.md:868-871` says it should. One 82-column line survives: an unbreakable repository
path, exempt for the same reason a URL is.

**4. The boundary check (AC-007), which is the one a machine catches without help.**

```console
$ git diff --name-only <this commit> | rg '^(crates|spec)/'
$ git diff --name-only <this commit> | rg '\.rs$|Cargo\.toml$'
```

Both empty. D-1's proposed fix is a four-line `QueryItem::from_validated` that would have
gone green everywhere, and it is recorded **inside the entry** instead of applied. That is
the mutant Context pack §1 writes out, failing by path before anyone has to notice what the
edit meant.

## Commits

- `344f2c0` — `feat(typed-layer-and-alpha-release): Defect log and macros verdict`

## Changes

| File | Shape of the change |
| --- | --- |
| `references/evaluation/phase-7-contract-defects.md` | **New**, 337 lines. Five entries (D-1 … D-5), each with six labelled fields including a clause ID and its maturity marker; two explicitly-not-entries (N-1, N-2); a fourteen-row reconciliation table |
| `references/evaluation/phase-7-macros-verdict.md` | **New**, 265 lines. Prediction → method → count → verdict, with a 29-row line-range classification that partitions the example exactly |
| `references/evaluation/README.md` | Two paragraphs under `## Later additions, which are neither`, one per record, so the directory's own lifecycle taxonomy still covers everything in it |
| `.kb/_intake/contract-defect-log-phase-7.md` | **New**, staged. Five claims plus two explicit non-claims, with **proposed** frontmatter and `source_paths` naming the long form |
| `.kb/_intake/happenstance-macros-verdict.md` | **New**, staged. Opens with *the one thing the wave must not do*: never edit ADR-0020's atom |
| `RUNBOOK.md` | Exactly three hunks: the `:525` decision-table row off `open`; the phase-7 macros exit box to `[x]`; the phase-7 session log, previously empty |
| `.bklg/.../defect-log-and-macros-verdict/` | `_ledger.md` and these two reports |

## Gates

| Gate | Result |
| --- | --- |
| `git diff --name-only \| rg '^(crates\|spec)/'` | empty — AC-007 |
| `git diff --name-only \| rg '\.rs$\|Cargo\.toml$'` | empty — NF-001 |
| `cargo xtask affected --base main` | **affected gate passed** (227 tests, 0 failed) |
| `cargo xtask spec-trace` | `traceability: no problems found; §7.1–§7.2 matches the checker` |
| `cargo xtask lints` | all seven file-reading lints green |
| `redkiln validate --kb` | `validate passed` |
| `redkiln doctor` | exactly the six expected `template-drift` advisories, no new class |
| Partition check (AC-008) | 532 of 532 lines; both extremes **out** |
| Entry-shape check (AC-002) | six labels × five entries; `Clause:` count = entry count |
| Density check (NF-003) | prose ≤ 80 columns, every entry ≤ 40 lines |

`cargo xtask ci --fast` is the project's integration bar and is run for the slice as a
whole; green here is a **baseline** claim and is evidence for no AC, which is what the spec
says it is.

## Notes

**The verdict is the deviation, and it is the point.** The design predicted *in* at 2.4:1
and the example returns 0.50:1 at the extreme most favourable to a derive. The two disagree
because the doctest is a *minimum viable domain*: the `DomainEvent` impl is a fixed cost that
grows barely at all with the domain — 26 lines for two variants, 40 for three — while the
domain grows with every consistency concern, refusal and handler. Both numbers are true and
they answer different questions, and the record says which is which rather than picking one.

**Two judgement calls in the classification are declared in the record, because they move
the number.** The `commit(...)` scaffolding inside each handler is `neither`, which *shrinks*
domain by 30 lines and moves the ratio **toward** "in" — against the verdict, which is the
direction a judgement call should err in. And `CourseId`/`StudentId` (81 lines) are
**contested** rather than assigned, because by the method's letter they are payloads (domain)
while their `Tag` field and serde bridge exist only for the mapping (ceremony). EC-007's
both-extremes report is what makes that honest rather than convenient.

**One EC did not fire and one nearly did.** EC-004 did not: the substrate was checked before
counting and `examples/course-subscriptions/src/main.rs` is the rewritten file. EC-005 was
checked rather than assumed — `rg -c 'macro_rules!'` returns 0 and `impl DomainEvent for
Enrolment` is written out by hand — because a shrunken mapping would have answered AC-013 by
concealment. EC-006's exact-1.0 tie did not arise; the answer is not close at either extreme.

**EC-008 fired and was handled as it says.** Every `RUNBOOK.md` line number in this spec was
taken at planning time. All three anchors were located by **heading and text** rather than by
number — the decision row by its `Is happenstance-macros in scope for 0.1` cell, the exit box
by its sentence, the session log by its `**Session log**` heading under phase 7 — and the
resolved lines are recorded in the ledger.

**The citation-drift sweep, and the part of it that is honest rather than clean.** The
swept set — `.kb spec references standards docs CLAUDE.md` — is unaffected: the highest
`RUNBOOK.md:NNN` any of them names is **3953**, and the first of my three edits is at 4079.
The `:525` row is one line replaced by one line and shifts nothing. **But** citations under
`.bklg/` from *other projects'* planning artefacts (from 4104 upward, in
`cloudflare-durable-object-store` and the initiative's `_discovery/`) do shift by the session
log's insertion. They are outside this story's PR fence, they are work-in-motion rather than
durable knowledge, and re-anchoring them would break the fence this story exists to respect.
Recorded here rather than silently left.

**One finding arrived from the slice-mate mid-story and is in the log rather than fixed.**
`edge-flavour-and-wasm-claim` found that `run_projection` cannot be spawned from generic code
without a caller-side bound. It is **N-1**, not an entry: ES-6 `[FROZEN]` and ADR-0009 already
answer it, and inventing a clause citation for a promise that was kept is exactly what EC-001
forbids. The same story's `read_through` dead-code finding is **N-2**, classified `support` at
the moment of finding because it bears on no clause. Neither was promoted, and both are
written down so their absence from the entries is not read as a gap.

**No `crates/happenstance-macros/` was created**, and none was proposed. `_decomposition.md:428`
routes only an *in* verdict to the runbook as a scope change, and this is *out*.
`publish-0-2-0-alpha-1` depends on the record, never on a crate, and is unblocked.
