---
item: "HS-S0057"
stage: implement
created: "2026-08-19"
updated: "2026-08-19"
---

# Implementation Report — CF-14 and CF-27 re-read on this runtime, and the ES-32 verdict on disk

**Three verdicts and no behaviour.** The diff is `RUNBOOK.md` and this story's own
directory, and nothing else: no `crates/**`, no `spec/**`, no `.kb/**`, no port
method, no sub-trait, no alarm plumbing. What changed is that three clauses that
had a promise in this file now have a reading in it.

## The three verdicts, in one place

| clause | verdict, first clause first | where it lives |
| --- | --- | --- |
| **CF-14** `[DEFERRED]` | *The deferral still holds, and one more of the three named implementations has answered with one shape.* | `RUNBOOK.md:561`, reconciled with `:488` |
| **CF-27** `[DEFERRED]` | *More real, not less, and this runtime is why* — the hazard is sharper here than the specification's completeness row records, and the instrument is refused and handed to HS-P0018. | `RUNBOOK.md:562` |
| **ES-32** `[PROVISIONAL]` | *A Durable Object does make a tail seam cheap to reopen post-0.1, and it makes it cheap in exactly the additive shape ES-32 already names* — a confirmation of the clause rather than a case against it. | `RUNBOOK.md:4449`, pointed at from `:500`, `:611`, `:4407`, `:4437` |

No marker moved. CF-14 and CF-27 are still `[DEFERRED]`, ES-32 is still
`[PROVISIONAL]`, and `git diff -- spec/` is empty.

## TDD Evidence

**No test asserts prose, and adding one would be decorative by `CLAUDE.md`'s own
test** — a conformance rule that observed a ledger paragraph could not be failed by
any adapter. The spec says so twice (Integration contract, *Conformance rule(s)*;
Tests and CI, closing paragraph). So the red/green here is over the file-reading
instruments and the `rg` walk that AC-010 makes a procedure rather than an opinion.

| AC | instrument | red (before) | green (after) |
| --- | --- | --- | --- |
| AC-001, AC-002 | `rg -n "CF-14" RUNBOOK.md` | the deferred-table row read *"8 — and the fixture half landed early at phase 3 … What stays deferred is the far end"* and carried **no phase-9 reading**; `:488` stopped at phase 3's | the row's first clause is the phase-9 verdict, and `:488` points at it |
| AC-003, AC-004 | `rg -n "CF-27" RUNBOOK.md` | the row was **`14`** and nothing else — three characters, no reading, no named recipient | a reading in its first clause and HS-P0018 named as the owner of the instrument this story refuses to build |
| AC-005 | `rg -n "A one-paragraph verdict on the tail seam" RUNBOOK.md`; phase 9's Session log | the exit box was **`- [ ]`** and the Session log was **empty** — the ledger promised a verdict at `:500` and `:611` that did not exist | the paragraph is on disk; both boxes are ticked with the reason *because the paragraph is on disk*, and both pointers state the conclusion |
| AC-008 | `cargo xtask spec-trace` | green before | green after — *"traceability: no problems found; §7.1–§7.2 matches the checker"* |
| AC-006, AC-008 | `git diff --stat` | — | `RUNBOOK.md` and this story's directory only |
| all | `cargo xtask affected --base main` | — | **`affected gate passed`** |

## The negative control

AC-007's anti-vacuity procedure, performed rather than asserted: each verdict was
attempted from the clause text alone, and each attempt failed at a specific
sentence.

**CF-14.** From `spec/SPECIFICATION.md:7876-7889` alone, the most that can be
written is the clause restated — *"whether a `reopen` capability can be honoured by
rusqlite, a Durable Object and a one-shot HTTP client with one shape, or whether
'durable' needs to be graded"* — plus CF-17's prediction that the DO can discard
handle state and read the store again. **What cannot be derived is whether it
did.** The sentence that survives — *expressed `REOPEN` through the same contract
shape, nothing added to the trait, with zero `SKIP` lines on the row* — rests on
`../measured-store-limits/implementation-report.md:94` and `:133`, and on
`../every-rule-under-workerd/implementation-report.md:162`, which records the
*opposite* state four weeks earlier: `SKIP acknowledged_writes_survive_a_reopen:
fixture declines REOPEN`. A prediction and a run are different artefacts and only
one of them was on disk before this milestone.

**CF-27.** From `spec/SPECIFICATION.md:8482-8508` alone, the hazard is stated in the
abstract — a 90-day prune happens entirely outside the port and the store passes
every rule afterwards. **What cannot be derived is the direction for *this*
runtime.** Both halves of the sentence written are facts about the bindings this
project put in the tree: that a DO's storage **outlives** its isolate, so the
intuitive eviction hazard is the wrong one, is `CloudflareFixture::reopen`'s
observed behaviour; and that the purge verb exists one method away from the one the
adapter uses is `storage_from_durable_object_state` reaching `sql` off a `Storage`
that also carries `delete_all()` (`worker-0.8.5/src/durable.rs:449`). Before
`worker-binding-layer` replaced the stand-in, there was no `Storage` in this crate's
graph to look at.

**ES-32.** From `spec/SPECIFICATION.md:4087-4118` alone, a paragraph can be written
that repeats the asymmetry and names a DO alarm — the clause already does both.
**What cannot be derived is the decisive property.** That a Workers cursor *is not a
snapshot and cannot be held across an `await`* is not in the clause; it is
`crates/happenstance-cloudflare/src/sql_storage.rs:1-24`'s property 2, checked
against the real bindings rather than asserted about a model, and it is the reason
ADR-0011's ceiling-and-page mechanism exists in this adapter's read path at all. It
is what turns *"a DO could push"* into *"a tail here can only ever be a wake-up plus
a bounded re-read"*, which is the whole content of the verdict.

## Changes

| file / region | shape of the change |
| --- | --- |
| `RUNBOOK.md:561` (CF-14 row) | phase-9 reading appended to the existing cell, conclusion first, naming the fixture, the three rules, the run, and the limit that it was not `workerd`. The far end is stated as still open and owned by HS-P0012. |
| `RUNBOOK.md:562` (CF-27 row) | the row was `14`; it is now a reading with its direction in the first clause and HS-P0018 named as the owner of the instrument this story refuses to build. |
| `RUNBOOK.md:488` (durability Decision-ledger row) | one sentence appended pointing at the CF-14 reading and saying it does not move the marker. Reconciled in the same commit, not as a follow-up. |
| `RUNBOOK.md:500` (tail-seam Decision-ledger row) | *"phase 9 records whether…"* becomes *"phase 9 has now recorded its half"*, with the conclusion in the same sentence and a pointer at the paragraph. |
| `RUNBOOK.md:611` (ES-32 falsifier group row) | marked **answered**, pointing at the paragraph, and stating that the clause's *other*, benchmark-shaped falsifier is not discharged by it. |
| `RUNBOOK.md:4407` (phase 9 Work item) | ticked, with *recorded only — no port method, no sub-trait, no alarm plumbing*. |
| `RUNBOOK.md:4437` (phase 9 exit criterion) | ticked **because the paragraph is on disk**, and it says so, so a reviewer reaches the paragraph from the tick in one hop. |
| `RUNBOOK.md:4449` (phase 9 Session log) | the one-paragraph ES-32 verdict — the only long form, cited by the four pointers rather than copied into them. |
| `RUNBOOK.md:564-577` (the deferred table's equality statement) | **corrected**, and this is the one change the plan did not anticipate — see *Notes*. |
| this story's folder | `_ledger.md` (ten rows flipped with cited evidence), this report, `report.md`. |

## Gates

| command | result |
| --- | --- |
| `cargo xtask spec-trace` | green — *"201 clauses (137 FROZEN, 47 PROVISIONAL, 12 DEFERRED, 5 NON-NORMATIVE) … traceability: no problems found; §7.1–§7.2 matches the checker"* |
| `cargo xtask affected --base main` | **`affected gate passed`** — the arm that runs the five file lints and `spec-trace` unconditionally, which is the whole automated half of a prose story's gate |
| `cargo xtask ci --fast` | green at slice close |
| `git diff --stat` | `RUNBOOK.md` and this story's directory only |
| `git diff -- spec/SPECIFICATION.md` | empty |
| `rg -n "CF-14\|CF-27\|ES-32" RUNBOOK.md` | one authoritative statement per clause; `:488`, `:500`, `:611`, `:4407`, `:4437` are pointers that carry their conclusion |

## Notes

**One deviation, and it is a finding rather than a licence.** AC-008 asks that the
deferred table's equality statement be re-read against the checker's output. It was,
and **it was stale on arrival**: the checker counts **twelve** `[DEFERRED]` clauses
and names PS-18, PS-27 and PS-30, none of which this table lists, while the table
names PS-33, which `spec/SPECIFICATION.md:9157` now carries as `NON-NORMATIVE`. The
sentence *"the two sets are equal — `spec-trace` counts ten `[DEFERRED]` clauses and
names the same ten"* was therefore false before this PR and would have stayed false
after it.

Leaving it would have been the *two tables that drift* anti-pattern this story is
partly about, in the very table being edited. Fixing it properly — adding three rows
and retiring one — is a clause question and outside a prose re-read's boundary. So
the sentence now states what is actually true, names the four clauses involved, and
hands the reconciliation to ADR-0023's queue row and to whichever pass owns the
projection deferrals. **No marker, row or census figure was moved to make it true**,
no row was displaced or renumbered, and the three struck rows stay where a reader
arriving from an older commit will find them.

**Line numbers drifted, as EC-005 said they would.** Every offset this spec quotes
was captured at planning time and the slice-mate merged first; phase 9's section
moved from `:4241` to `:4363`. Every mount point was re-located by heading and
clause id, which is what EC-005 fixes as the identity.

**What did not happen.** No falsifier fired. `CloudflareFixture` expressed `REOPEN`
in one shape and needed no grading, so EC-002's declined-capability path was not
taken; and the ES-32 reading concludes the port should **not** move before 0.1, so
EC-003's escalation was not needed on that edge either. The ledger says so
explicitly rather than leaving the question blank, which AC-009 requires.

**The ES-32 verdict is bounded and the bound is named** (EC-006). Nobody has
measured what a DO alarm's wake-up actually costs — latency, and per-object price at
N objects — and it cannot be measured from this gate, whose runner is
`wasm-bindgen-test-runner` over a `node:sqlite`-backed shim with no alarms and no
isolate. That measurement is what would sharpen the paragraph from *cheap in shape*
to *cheap in cost*, and it is post-0.1 work outside this initiative. A bounded
verdict with a named gap is a verdict; an unbounded claim would not be.

## Commits

One checkpoint on `initiative/from-contract-to-published-library`.

| SHA | subject |
| --- | --- |
| `a20a864` | `feat(cloudflare-durable-object-store): CF-14 and CF-27 re-read on this runtime, and the ES-32 verdict on disk` |
