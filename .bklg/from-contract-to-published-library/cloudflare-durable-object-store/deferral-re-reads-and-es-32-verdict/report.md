---
item: "HS-S0057"
stage: report
created: "2026-08-19"
updated: "2026-08-19"
---

# Report — CF-14 and CF-27 re-read on this runtime, and the ES-32 verdict on disk

## Findings Ledger

**Outcome: ten of ten ACs satisfied. Three verdicts on disk, no behaviour, no
marker moved. The diff is `RUNBOOK.md` and this story's own directory.** No
`crates/**`, no `spec/**`, no `.kb/**`; no tail, `subscribe` or `notify` method, no
`TailingEventStore`, no alarm plumbing, no Cargo feature.

### AC by AC

| AC | result | what proves it | where it is mounted |
| --- | --- | --- | --- |
| AC-001 — CF-14 re-read, verdict first, readable without the specification | **satisfied** | first clause is the verdict; the finding names `CloudflareFixture`, the contract shape, `acknowledged_writes_survive_a_reopen` by id, and the run it was read from, plus the limit that the run was not `workerd` | `RUNBOOK.md:561` |
| AC-002 — the far end stated open and owned, and `:488` reconciled | **satisfied** | the cell's last sentence hands the fault half to phase 8 / HS-P0012 and names HS-P0014 as the one implementation still unanswered; `:488` points at the reading in the same diff | `RUNBOOK.md:561`, `:488` |
| AC-003 — CF-27 read against what this runtime permits | **satisfied** | storage outlives the isolate, so eviction is the wrong hazard; `state.storage().sql()` sits one method from `delete_all()`, so the purge verb is outside the port entirely. States that the specification's completeness row understates by one adapter | `RUNBOOK.md:562` |
| AC-004 — the completeness half handed on by name | **satisfied** | HS-P0018 / phase 14 named in the row, with the refusal written rather than implied | `RUNBOOK.md:562` |
| AC-005 — the one-paragraph ES-32 verdict | **satisfied** | in phase 9's Session log; engages the asymmetry and the three shapes; reaches a verdict either way; names **which** falsifier it answers so the benchmark-shaped one is visibly still open; the exit box is ticked *because the paragraph is on disk* | `RUNBOOK.md:4449`, ticks at `:4407` and `:4437`, pointers at `:500` and `:611` |
| AC-006 — recorded, not acted on | **satisfied** | `git diff --stat` confined to `RUNBOOK.md` + this story's directory; the paragraph's own last sentence says *recorded, not acted on*; `cargo xtask ci --fast` green | — |
| AC-007 — every verdict names an artefact that did not exist before | **satisfied** | the negative control was performed and recorded in `implementation-report.md`, *The negative control*: each sentence was attempted from the clause alone and each attempt failed at a named point | — |
| AC-008 — no marker moves, the census is untouched | **satisfied, with a finding** | `git diff -- spec/SPECIFICATION.md` empty; `spec-trace` green; thirteen rows, three struck, ten live, none displaced. **The equality statement was stale on arrival** and now states what is true — see below | `RUNBOOK.md:564-577` |
| AC-009 — a fired falsifier is escalated, and silence is not left blank | **satisfied** | no falsifier fired, and both readings say so in terms; two things *are* handed on by name — the deferred-table drift, and the unmeasured alarm cost | ledger evidence + the RUNBOOK rows |
| AC-010 — the verdict is where the reader already is | **satisfied, after a post-review correction** | two existing table cells and three paragraphs in an existing session log; no new section, no new table, no bare pointer, and the long form exists once with the cells pointing at it. The **sentence budget was the one part initially missed** — see *The density budget* below | `RUNBOOK.md:561`, `:562`, `:4492-4525` |

### The finding a reviewer must read

**The deferred table's equality statement was false before this PR and is now
corrected rather than repaired.** AC-008 asks that it be re-read against the
checker. It was. `cargo xtask spec-trace` counts **twelve** `[DEFERRED]` clauses and
names **PS-18, PS-27 and PS-30**, which the table does not list; the table names
**PS-33**, which `spec/SPECIFICATION.md:9157` now carries as `NON-NORMATIVE`. So the
sentence *"the two sets are equal — `spec-trace` counts ten and names the same ten"*
was wrong in both directions.

What was done: the sentence now states the twelve, names the four clauses involved,
and says the drift predates this reading. What was **not** done, deliberately: no
row added, none removed, no marker moved, no census figure touched — reconciling a
maturity table is a clause question and a prose re-read has no licence for it. The
reconciliation is handed to `adr-0023-and-atom-resolutions` and to whichever pass
owns the projection deferrals, in the RUNBOOK text itself so it cannot be lost with
this report.

### The density budget, and the correction it took

**Recorded because a review caught it rather than because the story did.** AC-010
fixes a numeric invariant — *each row cell stays within the one-to-three-sentence
budget its ten neighbours keep* — and the CF-14 and CF-27 cells as first written ran
to **four** sentences and roughly 900 to 1,100 characters each, against neighbouring
cells of about fifty. The conclusion-first, in-place, greppable half of the AC was
served, and the struck CF-13 cell is an in-table precedent for a long one, so this
was a soft miss rather than a betrayal — but it was a miss, and a numeric invariant
the spec chose to state is not satisfied by being *nearly* met.

**The fix moves detail, it does not delete it.** Each cell is now its verdict plus
the one narrow finding that verdict rests on, three sentences each: CF-14 keeps the
fixture, the contract shape, `acknowledged_writes_survive_a_reopen` and the zero-`SKIP`
run, plus the far-end handoff AC-002 requires; CF-27 keeps the `delete_all()` verb, the
completeness-row consequence and the HS-P0018 handoff AC-004 requires. Everything
trimmed — the three rule names in full, the `wasm-bindgen-test-runner`-not-`workerd`
limit, `happenstance-neon` as the one named implementation still unanswered, and
CF-27's *Rejects*-paragraph argument — moved into phase 9's Session log as
*The CF-14 re-read, in full* and *The CF-27 re-read, in full*, beside the ES-32
paragraph. That is the shape AC-010 itself blesses: **long form once, pointers
elsewhere**, and it is the same shape the ES-32 verdict already uses from five sites.
No claim was weakened and no citation dropped; `rg -n "CF-14|CF-27|ES-32" RUNBOOK.md`
still finds one authoritative statement per clause.

### Deferred, and to whom

- **CF-14's far end** — a store that can lose an acknowledged write to a *fault*
  rather than to an instruction — stays phase 8's and `sqlite-durable-store`'s
  (HS-P0012). `happenstance-neon` (HS-P0014) is the one named implementation of
  `REOPEN` still unanswered.
- **CF-27's completeness instrument** — a testkit-adjacent store holding only a
  suffix and reporting that it does — stays `retention-and-incomplete-logs`'
  (HS-P0018) and phase 14's. This story answers the re-read and refuses the
  instrument, in writing.
- **The ES-32 measurement that would sharpen the verdict** — what a Durable Object
  alarm's wake-up costs, in latency and per-object price at N objects — is
  unmeasurable from this gate and is post-0.1 work outside this initiative. It is
  named inside the verdict paragraph, so a reader of the verdict cannot miss it.
- **The deferred-table drift** — to ADR-0023's queue row and the projection passes,
  as above.

### What was not touched

`spec/SPECIFICATION.md` — no clause text, no maturity marker, no census figure.
`.kb/**` — no atom, no ADR, no open-question resolution; `adr-0023-and-atom-resolutions`
owns every KB write in this project. Any port, adapter or testkit code. The
`workerd` execution itself and the measured limits, which are
`every-rule-under-workerd`'s and `measured-store-limits`' and were consumed here
rather than relitigated.
