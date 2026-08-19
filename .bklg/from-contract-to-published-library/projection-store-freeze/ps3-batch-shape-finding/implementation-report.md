---
item: "HS-S0014"
stage: implement
created: "2026-08-14"
updated: "2026-08-14"
---

# Implementation Report — The PS-3 evidence written as a finding, not a verdict

> **STATUS: eight of eight ACs satisfied.** One document, two mounts, and no
> verdict. `references/evaluation/projection-batch-shape-evidence.md` is dated,
> pinned to `cfd9231`, indexed in the evidence tree's README and cited from PS-3's
> clause body by `file:line` — so `cargo xtask spec-trace`, a mandatory gate step,
> resolves it on every run.
>
> **The finding is not a null result**, which was the outcome the story was
> written to be able to report honestly. Fourteen rules agreed, **two are D2**
> (asymmetric declension between the two fixtures), and the comparison turned up
> something sharper than either: **`MemoryProjectionStore` is already a deferred
> write set**, so the pair of shapes spans a narrower axis than PS-2's. That is
> recorded with its citation and decided on by nobody here.
>
> **Nothing frozen was edited and nothing was decided.** PS-2 is cited only. PS-3
> keeps its `[PROVISIONAL]` marker and its `Rule:` / `Cases:` / `Rejects:` fields.
> No `.kb/` atom was written and `.kb/_intake/` is empty.

## TDD Evidence

This story adds no test file — the Testing brief types AC-015 **E2E (process,
derived from AC-004)**, *"not a new test"* (`../_decomposition.md:785`). Its
instruments are two gate steps that already exist, two mechanical comparisons,
and the story review. The red-then-green below is real and was run in that order.

| AC | Instrument | Red → Green |
| -- | ---------- | ----------- |
| **AC-008** | `cargo xtask spec-trace` | **Red first, deliberately.** The PS-3 citation was written into `spec/SPECIFICATION.md` *before* the finding existed, and the gate said so by name: ``spec/SPECIFICATION.md:4790 — citation `references/evaluation/projection-batch-shape-evidence.md:1` names a file that does not exist`` — 1 traceability problem, exit 1. **Green** once the document landed: *"traceability: no problems found; §7.1–§7.2 matches the checker"*, 359 citations checked where the previous run checked 358. |
| **AC-004** | mechanical name-set comparison, both directions | Ledger rule names extracted from the built document and compared against the arms of `for_each_projection_store_rule!`. Result: enumeration arms **16**, ledger rows **16**, order identical **True**, in-enum-not-in-ledger **[]**, in-ledger-not-in-enum **[]**, duplicates **[]**. Run rather than asserted; the script is in `## Gates`. |
| **AC-003** | mechanical label-set check | Verdict labels appearing in the ledger column: `{agreed, **D2**}` — a subset of the six defined in §2, no seventh minted inside the ledger. |
| **AC-001** | `git status --porcelain .kb/`, `redkiln validate --kb` | Empty, and `validate passed.` The emptiness is the assertion; the validation is the guard that nothing slipped in. |
| **AC-002** | `git cat-file -e cfd92313cc59e059655fe130f3c8c31b19dfcf08` | Resolves on `initiative/from-contract-to-published-library`. The pinned sha is the slice-mate's checkpoint, whose `cargo xtask ci` run is the one §1 reports. |
| **AC-005 / AC-006 / AC-007** | story review against the AC table and `discover.md:75-107` | No tool can check whether prose reads as a verdict. Each is a named, locatable section with cited claims, listed row by row in `report.md`'s Findings Ledger. |

## Commits

`feat(projection-store-freeze): The PS-3 batch-shape finding` — one checkpoint
commit, the second and last of slice `second-batch-shape-and-evidence`, on
`initiative/from-contract-to-published-library`, immediately after
`feat(projection-store-freeze): The second, unlike batch shape` (`cfd9231`).

Named by subject and by predecessor rather than by hash, for the reason the
earlier reports in this project give: this file is committed *inside* the commit
it describes. `git log --grep "Story: projection-store-freeze/ps3-batch-shape-finding"`
resolves it. The **predecessor's** hash is load-bearing and is quoted in full,
because the finding is pinned to it.

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `references/evaluation/projection-batch-shape-evidence.md` | **New, and the whole deliverable.** 222 lines in the fixed reading order the spec sets: lifecycle header (date, pinned sha, supersede-not-edit with its citation) → §1 the evidence base, named before it is reasoned from → §2 the six-label vocabulary, stated before the ledger → §3 the sixteen-row per-rule ledger → §4 the null result weighed against both of Note 10 item 2's readings → §5 the PS-2 sentence → §6 what is not in it → §7 the explicit non-verdict, last. |
| `references/evaluation/README.md` | **One index entry** (`:91-112`), in the *"Later additions, which are neither"* section, following the shape `review-citation-drift.md`'s and `ps-clause-pairing-sweep.md`'s entries already use: what it is, when it was written, what it is pinned to, that the runbook was not derived from it, and the same immutable-supersede lifecycle sentence. No third lifecycle proposed. |
| `spec/SPECIFICATION.md` | **One additive paragraph** in PS-3's body (`:4788-4790`), after the `Rejects:` field and before the section break, carrying the `file:line` citation. `git diff` is `+4` lines in the whole file: one blank line and three lines of prose. No maturity marker, no `Rule:` / `Cases:` / `Rejects:` field, no other clause, and no change to the generated §7.1–§7.2 region. |
| `.bklg/…/ps3-batch-shape-finding/_ledger.md` | Eight rows flipped `false → true` with cited evidence. No criterion re-worded. |

**Not touched, and checked:** `.kb/**` (`git status --porcelain .kb/` empty),
`crates/**` (this story compiles nothing new), PS-2's clause body, every other
clause, and the `unstable-projection` gate.

## Gates

| Gate | Command | Result |
| ---- | ------- | ------ |
| Citation resolves | `cargo xtask spec-trace` | red-then-green, above. Final: `traceability: no problems found`, 359 citations checked, 69 anchored, 12 external |
| Spec diff | `git diff spec/SPECIFICATION.md` | `1 file changed, 4 insertions(+)`, all four inside PS-3 |
| KB untouched | `git status --porcelain .kb/` | empty |
| KB valid | `redkiln validate --kb` | `redkiln: validate passed.` |
| Pinned sha resolves | `git cat-file -e cfd92313cc59e059655fe130f3c8c31b19dfcf08` | resolves |
| Whole gate | `cargo xtask ci` | `all checks passed` |

**The AC-004 comparison, recorded as run rather than as asserted.** Executed
against the working tree at the pinned sha:

```python
arms   = re.findall(r'^\s{12}([a-z_][a-z0-9_]*),\s*$', for_each_projection_store_rule_body, re.M)
ledger = re.findall(r'^\| \d+ \| `([a-z_][a-z0-9_]*)` \|', finding, re.M)
# enumeration arms: 16 · ledger rows: 16 · order identical: True
# in enum not ledger: []   in ledger not enum: []   duplicate rows: []
# verdict labels: ['**D2**', 'agreed']
```

The arm list came from `crates/happenstance-testkit/src/projection.rs:1843-1881`,
not from `registry.rs`: the projection enumeration lives beside the projection
rules, while the event-store family's sits in `registry.rs`. The spec and this
story's `_ledger.md` `verifying_test` both name `registry.rs`; the finding records
the correction at `:29-31` so a reader who goes looking finds the macro.

## Notes

**Three deviations, all reported rather than absorbed.**

**1. The ledger is not all-agreed, so AC-006's "whenever the ledger is
all-agreed" clause is discharged more than required.** Two rows are **D2** —
`failed_commit_leaves_both_unchanged` and `refused_reset_changes_nothing` run
against the buffering fixture and report `RuleOutcome::Skipped` against
`MemoryProjectionFixture`, because the two fixtures declare `COMMIT_FAULT` and
`RESET_REFUSAL` differently. §4 was written anyway, and at length, because the
*batch-shape* question it answers is still a null one: on that axis the two
shapes did not disagree anywhere.

**2. AC-002's criterion calls `MemoryProjectionStore` "apply-on-write". The
finding does not repeat that, because it is false.** Its `begin` returns an owned
batch holding a delta and its `commit` applies it
(`crates/happenstance-core/src/projection_memory.rs:280-330`). The criterion's
operative requirements — both fixtures by their real identifiers, the harness
invocations, one `cargo xtask ci` with its sha — are all met; the characterisation
is corrected in §4 with its citation rather than reproduced. Restating a
falsehood inside an evidence document to match a plan's wording is the one thing
this document cannot afford, and the criterion was not re-worded to hide the
divergence.

**3. `references/evaluation/README.md`'s own line numbers moved when the entry
was inserted, and the finding's citation to them was repointed.** The finding
cites `README.md:140-142` for the supersede-never-edit rule; that block sat at
`:117-119` before the new entry pushed it down. Repointing a citation at the file
it already named is the one in-place edit the evidence tree permits, *"because it
changes no claim, only whether a reader can follow one"* — and it was done before
the document was committed, so no reader has ever followed the old numbers.

**Nothing was fixed that this story found.** §4 describes three rules whose reach
is narrower than a reader might assume (rows 4, 5 and 13 accept two different
mechanisms as the same answer). That is a description of what a port-level rule
asserts, not a defect filed against it; §6 says so, and a rule that turns out to
be wrong is repaired in its own story with the reason in the same change.

**Nothing of HS-S0016's was done.** Being inside `SPECIFICATION.md` with PS-3's
`[PROVISIONAL]` marker three lines above the edit is the invitation the spec
warned about. The marker is byte-identical, the `unstable-projection` feature does
not exist yet, and the clause disposition is still
`unstable-projection-gate-and-clause-disposition`'s (AC-014).
