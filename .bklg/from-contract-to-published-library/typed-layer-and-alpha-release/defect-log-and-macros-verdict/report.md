---
item: "HS-S0032"
stage: report
created: "2026-08-16"
updated: "2026-08-16"
---

# Report — The contract defect log, and the happenstance-macros verdict

## Findings Ledger

**Outcome: ten of ten ACs satisfied. Nothing deferred, nothing blocked.** Two records
landed and no code changed — `git diff --name-only` for this checkpoint returns no path
under `crates/**` or `spec/**` and no `.rs` or `Cargo.toml` at all, which is AC-A02
expressed in git rather than in prose.

**The headline: the macros verdict is `out`, and it contradicts the signed-off design's own
prediction by about a factor of five.** That is what "recorded either way" was for.

| AC | Result | Proven by | Notes |
| --- | --- | --- | --- |
| **AC-001** | satisfied | Both records exist under `references/evaluation/`, each carrying its date and pin at `:3-4`; `references/evaluation/README.md:165-201` gains a paragraph for each | The pin `78a2170` resolves. The README's three-way lifecycle taxonomy still covers the whole directory |
| **AC-002** | satisfied | `rg -c '^### D-'` = 5; `rg -c '^\*\*Clause:\*\*'` = 5; four more labels also = 5 | Red first: entries were `##` with run-in labels. The `Clause:` count equalling the entry count is the assertion the AC names, and it now holds exactly |
| **AC-003** | satisfied | Entry one at `phase-7-contract-defects.md:50-92` | D-1 from `_design.md:652-672`, naming **VT-18** and its `[FROZEN]` marker, citing `crates/happenstance-core/src/query.rs:48-62`. Both cited spec lines re-resolved at the pin. States plainly that the constructor may well be right and the objection is to taking that decision without a record — and it is **stronger** than recorded, because `Boundary`'s seal makes the error arm untestable from outside the crate |
| **AC-004** | satisfied | Entry D-3 at `:137-176` | CF-36 `[FROZEN]`; the contradiction stated concretely (`grep -c "Level" xtask/src/spec_trace.rs` = 0). Neither `spec_trace.rs` nor the clause body appears in the diff |
| **AC-005** | satisfied | The reconciliation table at `:283-303` | The sweep was **re-run** at the pin rather than copied from planning: fifteen files, fourteen rows, every one with an explicit disposition. Five say *found none*, each citing the line where that story said so |
| **AC-006** | satisfied | The `## Findings that are **not** entries` section at `:247-281` | Present and non-empty. **N-2** is the support finding (`read_through`, no clause ID); **N-1** is the inverse discipline — a finding that looks like a defect and is not. No clause ID was invented; nothing was written under `.bklg/support/` |
| **AC-007** | satisfied | `git diff --name-only \| rg '^(crates\|spec)/'` → empty | The mutant was real and named: D-1's proposed fix is four lines and would have gone green. It is recorded inside the entry rather than applied |
| **AC-008** | satisfied | `phase-7-macros-verdict.md`, and the partition check re-derived from its own table | See *The verdict* below |
| **AC-009** | satisfied | Three hunks in `RUNBOOK.md`: `:525`, `:4079-4083`, `:4109` | All three anchors located by heading and text, not by number (EC-008). Citation-drift sweep run — see *Two things stated rather than hidden* |
| **AC-010** | satisfied | `.kb/_intake/contract-defect-log-phase-7.md`, `.kb/_intake/happenstance-macros-verdict.md` | Distinct filenames; M1's wave untouched (EC-009). Proposed frontmatter with `source_paths` naming the long forms. Nothing under `.kb/decisions|open-questions|maps`; no ingest run. `redkiln validate --kb` passes and `doctor` reports exactly the six expected advisories |

### The verdict, since it is the story's substance

| Contested 85 lines assigned to | ceremony | domain | ratio | verdict |
| --- | ---: | ---: | ---: | --- |
| ceremony | 125 | 249 | **0.50 : 1** | out |
| domain | 40 | 334 | **0.12 : 1** | out |

Both extremes agree, so the contested block is a footnote rather than the decision (EC-007).
The 29 published ranges partition `examples/course-subscriptions/src/main.rs` exactly — 532
of 532 lines, contiguous, no overlap — and that is checked **mechanically from the published
table**, not asserted. The check failed on the first draft (three handler boundaries were
wrong), which is what makes it a check.

**The design predicted `in` at 2.4:1 and is contradicted by name.** The two disagree because
the doctest is a minimum viable domain: `impl DomainEvent` is a fixed cost that barely grows
(26 lines for two variants, 40 for three) while the domain grows with every consistency
concern, refusal and handler. Both altitudes are reported side by side and labelled, because
the doctest's 2.4:1 is a true fact about a reader's first program — it is simply not what
AC-013 asked.

**A derive would have deleted 40 lines of 532 — 7.5%.** The one condition that could reopen
it is written into the record: if defect D-2 (`DomainEvent::tags` infallible over a fallible
`Tags`) is settled with an infallible path, the contested identity newtypes shrink and the
measurement should be re-taken. Even then an 85-line swing does not reach 1:1 from 0.50:1.

### Mount point

**`.kb/_intake/`** — the only input path `/redkiln:kb-ingest` reads, so a record staged
anywhere else is unmounted by construction. Both documents are there under names distinct
from M1's wave, which is still present and untouched. The **second mount** is `RUNBOOK.md`,
for the half `_intake` cannot hold: the plan of record is the only artefact that could act on
an escalation, and it is where the runbook itself says the verdict is written.

The **durable** home is `references/evaluation/`, and the split is load-bearing rather than
tidy: a successful ingest **clears** `_intake`, and a ~100-line atom cannot carry the call
site, the attempted code and the contract's actual response — the part that lets a later
reader judge whether a defect was real.

### Two things stated rather than hidden

**1. The citation-drift sweep is clean where it was specified and not clean everywhere.**
AC-009's swept set — `.kb spec references standards docs CLAUDE.md` — names no `RUNBOOK.md`
line above **3953**, and the first of the three edits is at 4079, so nothing in it drifted.
But citations under `.bklg/` from *other projects'* planning artefacts (4104 upward) do shift
by the session log's insertion. They are outside this story's PR fence and are work-in-motion
rather than durable knowledge; re-anchoring them would break the fence this story exists to
respect. Recorded so the next reader does not mistake it for an oversight.

**2. One 82-column line survives the prose budget.** It is an unbreakable repository path,
exempt for the same reason a URL is. Everything else in both records wraps at 80, and every
entry is within the 40-line budget — the commentary yielded three times to get there, which
is what `_design.md:868-871` says should yield.

### Nothing deferred

No conformance rule was added, so no `CHANGELOG.md` entry is owed under CF-29 and
`changelog_names_every_rule` stays satisfied (NF-005 — the gate's step set is unchanged). No
`.kb/` atom was hand-written (NF-007). No `crates/happenstance-macros/` was created and none
was proposed: an *out* verdict escalates nothing, and `publish-0-2-0-alpha-1` depends on the
record rather than on a crate.

The one thing this story deliberately does **not** finish is the ingest itself.
`/redkiln:kb-ingest` is a human handoff; this story ends at *staged and ready*, and the
long-form records under `references/evaluation/` are the hedge that survives the wave
clearing `_intake`.
