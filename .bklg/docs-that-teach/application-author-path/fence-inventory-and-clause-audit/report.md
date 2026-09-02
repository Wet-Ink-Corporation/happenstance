---
item: "HS-S0189"
stage: report
created: "2026-08-19T00:00:00.000Z"
updated: "2026-08-19T00:00:00.000Z"
---

# Report — Every fence inventoried and every clause citation audited

## Findings Ledger

**Six of six criteria satisfied. Zero fences opted out, nine of nine clause citations resolve,
zero clauses restated — and none of those three numbers is a hand count.** Each was made to fail
on purpose where a mechanism exists, and where none exists the absence is named, measured and
routed rather than covered by another page's coverage.

The headline result the reviewer should test this record against is the one it was hardest to
produce: **two of the four drills were expected to fail to fail, and both did.** They are
recorded as *observed green*, with the commands and outputs, because they are the only evidence
that `crates/happenstance/src/lib.rs` — the page a `cargo add happenstance` reader lands on
first — is protected by review rather than by machinery.

| Finding | Evidence | Follow-up |
| --- | --- | --- |
| **Eleven fences across five files; zero opted out.** `ignore` 0, `no_run` 0, untagged 0, `IGNORE_ALLOWANCES` 0 — and the constant itself is the empty slice, so the commitment `_design.md:206-208` made is true of HS-P0020's list and not only of this project's rows in it | `_inventory.md § Fences`; `xtask/src/lint_narrative.rs:304`; `cargo xtask narrative` → `5 pages, all consistent` | none — this is the expected result, and an exception would have been a stop (EC-003), not a row |
| **The three exercise mechanisms do not cover the same ground, and one of them covers nothing.** Tree pages: compiled *and* executed, and checked as files. Crate root: executed, checked by nobody. `overview.md`: **no doctest at all**, because `course-subscriptions` is a bin-only package | `_inventory.md § Fences`; `cargo test --workspace --all-features --doc -- --list` returns no `course`/`overview` entry | **R-4 → HS-P0020**. Vacuous today (zero fences there); prospective, which is when it is cheap |
| **An `ignore` on the crate root is caught by nothing.** Four instruments run against it, all green | `_inventory.md § Drills` D-2 | **R-2 → HS-P0020.** Compensation in place is `_design.md:872-874` anti-pattern 1 — a reviewer reading a diff |
| **A clause citation on the crate root is resolved by nothing.** `spec_trace` reads only the specification; `lint_narrative` sweeps `docs/`; a relative markdown link is not an intra-doc link | `_citations.md § Drills` D-4 | **R-5 → HS-P0020** |
| **`no_run` passes HS-P0020's checker silently, and that silence is load-bearing.** Measured with the fence's central assertion inverted: with `no_run` the page is *false* and green; without it the same page is red | `_inventory.md § Drills`, EC-002 probe; `xtask/src/lint_narrative.rs:823` | **R-1 → HS-P0020.** No `no_run` exists on this project's surfaces, so EC-002's repair half is vacuous and only the routing is owed |
| **The crate-root fence's doctest is named `(line 247)` in a 242-line file** — the `README.md` + module-doc concatenation defect `xtask/src/constitution.rs:11-18` names, present on the one surface outside the harness. The five tree fences are named by their real source lines | `_inventory.md § Drills` D-2 | **R-3 → `support` / `inherited-documentation-defects`**, beside HS-B0001, for the reason that item's own § Notes gives |
| **`spec-trace` does not resolve a page's citation, and the storymap's one-liner is shorthand.** Falsified directly: on a tree carrying `ES-999`, `cargo xtask spec-trace` exits 0 | `_citations.md § Claims` and `§ Drills` D-3 | none — the correction is recorded, and the resolver is now stated per page |
| **Nine normative sentences; nine resolving clause ids; zero restatements.** Two passes, no third procedure invented: the `MUST` sweep returns *no output* over all five files, and RP-40-2 was executed file by file with the sentence counts its own named wrong output demands | `_citations.md § Restatement` | none. The two closest calls are quoted verbatim with their reasoning, for the tier-5 reviewer to disagree with if they wish |
| **The three crate-root density overages are unchanged and already owned** — 35 rendered lines / 32, 70 columns / 68, headings at 39 and 26 / 22 | `_inventory.md § Fences`, composition table | **R-6 → HS-B0001**, re-measured rather than re-owned. Filing them twice is how a routing table stops being read |
| **`docs/append-conditions.md:13` imports `happenstance_core` in a fence** — HS-P0020's own page, outside this project's four surfaces | `git grep -n "use happenstance_core::" -- docs` | **R-7 → HS-P0020.** Recorded because the sweep crossed it, and a hit seen and unwritten is the finding that comes back |

**Mount point.** `xtask/src/narrative.rs`. All three tree pages were already registered — one
`#[cfg(doctest)] mod` per file at `:135`, `:141`, `:150` — so EC-006 did not fire and no line
was added. The mount is *observed* rather than made, and it is observable two ways exactly as the
spec's integration contract requires: every page this project authored is named there, and
`IGNORE_ALLOWANCES` names none of this project's fences.

**Nothing deferred, nothing stubbed, nothing absorbed.** Seven findings, seven named
destinations. EC-001, EC-003, EC-004, EC-005, EC-006, EC-007 and EC-009 did not fire; EC-002 did
and is discharged in both halves; EC-008's cap held on the two citation-position observations,
which are recorded rather than rewritten.

## Acceptance

| AC | Status | What proves it | Where it lives |
| --- | --- | --- | --- |
| **AC-001** — every fence inventoried, zero opted out | **Met** | 11 fence rows + 2 zero-fence rows, each with info string, execution class, exerciser, failability and hidden-line count; enumeration from `_design.md:45-65` rather than a directory listing; hidden lines read off the DOM (35 rendered against 37 source, the two absent being `lib.rs:31` and `:63`) | `_inventory.md § Fences` |
| **AC-002** — the zero made falsifiable | **Met** | D-1 red with the exact `path:line — message` then green after revert; D-2 green across four instruments and recorded as *observed green*; both files byte-identical afterwards | `_inventory.md § Drills` |
| **AC-003** — every claim cites a clause that resolves, with its marker and its resolver | **Met** | C-1…C-9 with markers verbatim from both the clause body and the maturity index; the resolver stated **per page**, `nothing` for the crate root and for `overview.md`; all nine link fragments resolved by slugging the specification's headings | `_citations.md § Claims` |
| **AC-004** — resolution made falsifiable | **Met** | D-3 red with ``cites `ES-999`, which SPECIFICATION.md does not define``; D-4 green across four instruments, recorded and routed | `_citations.md § Drills` |
| **AC-005** — no page restates a clause | **Met** | The `MUST` sweep returns no output and exits 1 across all five files; RP-40-2 executed per file with normative-sentence counts and the two closest calls quoted. Walker authored none of the four surfaces | `_citations.md § Restatement` |
| **AC-006** — findings closed in-boundary or routed, composition preserved | **Met** | Seven routed findings with named destinations; no repair needed and none made; `git diff --stat` on the four planning artifacts empty; `git status --porcelain` empty; the render's selector resolves and its bracket count is zero; `affected gate passed` and `all required checks passed` | `_inventory.md § Routing`; `implementation-report.md § Gates` |

**Deferred: none.** Every routed finding is a substrate or ownership question outside this
story's PR boundary, and each names the item that owns it. No AC is met by a stub, a fixture pin
or a skipped check.

## Knowledge Harvest

Four candidates for `.kb/` at closeout (HS-P0025's, not authored here — `project.md:144-147`).

1. **A coverage table that averages is a reassurance.** The whole value of this story's two
   artifacts is the per-row *failability* column: "every fence is exercised" is true of all five
   files and means three different things across them. The transferable rule is that an audit of
   a set states its instrument per member, and that a member whose instrument is `none` is a row
   rather than a rounding error. This is CLAUDE.md's *a rule that no adapter can fail is
   decorative*, one level up: **a report no member of the set can fail is decorative too.**

2. **The drill that is expected to fail to fail is the one worth running.** D-2 and D-4 produced
   no red line and are the two most informative results in the story. A drill dropped because
   "it wouldn't work anyway" removes the only evidence that a surface is protected by review
   rather than by machinery — and the difference between those two is invisible in a green gate.

3. **`no_run` is a silent opt-out from *execution* that survives every check for opt-outs from
   *compilation*.** The checker's closed token set treats `rust`, `no_run` and `should_panic`
   alike, so the one tag that keeps a fence type-checked and never run is the one an author
   reaches for when a doctest is slow. Measured here with the assertion inverted: the page was
   false and the gate was green. Any tree that compiles narrative fences owes a decision about
   this token, and the decision belongs beside the token set rather than in a page's prose.

4. **Concatenated `include_str!` moves the failure's line number off the map, and the harness
   rule that prevents it stops at the harness.** `xtask/src/narrative.rs` registers one module
   per page precisely so a failure names a line a reader can open. `crates/happenstance/src/lib.rs`
   concatenates its README with its module doc and reports `(line 247)` in a 242-line file. The
   rule is right; its *scope* is the finding.
