---
item: "HS-S0152"
stage: implement
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Implementation Report — A governed page names the discipline as the reason it is shaped as it is

## TDD Evidence

This story ships **zero lines of Rust**, by construction rather than by omission: its PR boundary
forbids any `xtask/src/**` change, so it can carry no `#[test]` (spec, *Tests and CI*, the `static`
row). Its Red/Green is therefore the **mechanical assertion set the acceptance table names**, each
one run before the sentence existed and again after, plus two deliberate breakages watched failing.

**Red, captured before a word was written.**

| AC | assertion | Red result |
| --- | --- | --- |
| AC-001 | `rg -n 'RP-10-2' docs/README.md` | no output, exit 1 — nothing cited the rule |
| AC-001 | `rg -n '\[`RP-10-2`\]\(' docs/README.md` | no output, exit 1 — no visible-link-text form |
| AC-004 | `rg -n 'docs/README.md' standards/pages/README.md` | no output, exit 1 — the rules tree named nothing it governs |
| AC-002 | `cargo run --locked --quiet -p xtask -- lint-pages` | `2 pages, 16 rules, all consistent` — green, and silent about the page that was about to be cited |

Each Red fails for the *right* reason: the strings are absent, not misspelled, and the checker is
green rather than broken.

**The membership probe was run first, before the citation, exactly as the spec's Implementation
notes instruct** — because everything downstream depends on which page is governed.

*Probe A.* An unenumerated token was inserted into `docs/README.md` under its H1 —
`> **Answers:** \`probe\` — Is this page inside the checker's walk?` — a declaration that would fail
any governed page. `cargo run --locked --quiet -p xtask -- lint-pages` printed
`2 pages, 16 rules, all consistent`: **total silence**. `docs/README.md` is outside the walk
(`xtask/src/lint_pages.rs:496`). Reverted; `git status --short docs` clean.

*Probe B.* `docs/append-conditions.md`'s declaration was deleted. The same command printed
`docs/append-conditions.md — no \`> **Answers:**\` line; see standards/pages/00-one-need.md` and
`xtask failed: 1 problem(s) in standards/pages + docs`. That page **is** in the walk. Reverted;
re-run green; `git status` clean.

So **EC-001's pre-decided tie-break fired** and the citation lands twice, which `_ledger.md`'s
`## EC-001 fired` section records in full.

**Green, after the two sentences and the back-link.**

| AC | assertion | Green result |
| --- | --- | --- |
| AC-001 | `rg -n 'RP-10-2' docs/README.md` | `14:not here, because [`RP-10-2`](../standards/pages/10-the-need-set.md) is the rule` |
| AC-001 | `test -f standards/pages/10-the-need-set.md` | `PRESENT standards/pages/10-the-need-set.md` |
| AC-003 | `rg -c '^> \*\*Answers:\*\*' docs/append-conditions.md docs/text-fences.md` | `docs/append-conditions.md:1`, `docs/text-fences.md:1` |
| AC-003 | `awk '!/^\|/ && length > 96 …'` over all three files | no output |
| AC-004 | `rg -n 'docs/README.md' standards/pages/README.md` | `8:rules govern is the narrative tree at [`docs/`](../../docs/README.md).` — exactly one |
| AC-004 | `wc -c < standards/pages/README.md` | `5429` (ceiling 8192) |
| AC-004 | `cargo run --locked -p xtask -- lint-pages --write` | `2 pages, 16 rules, all consistent`, and no diff |
| AC-005 | `rg -n '<[a-z]\|^\s*!\[\|^> \[!'` over the two pages | exit 1, no match |
| AC-006 | fold / size / icon greps over all three files | exit 1, no match on each |
| AC-008 | `git diff HEAD -- xtask` | empty |

**Two breakages watched failing, which is what stops the Green being decorative.**

*The back-link's check, seen to fail.* With the target mistyped as `../../docs/READMEX.md`:
`standards/pages/README.md:8 — link \`../../docs/READMEX.md\` resolves to no file` and
`xtask failed: 1 problem(s) in standards/pages + docs`. Restored, green. This also *observes*
EC-004's forward obligation rather than assuming it: the copied link half resolves
`root.join(RULE_DIR).join(target)` (`xtask/src/lint_pages.rs:817`) and was not narrowed to targets
inside `RULE_DIR`, so a `../../` target outside the rules tree resolves correctly.

*The pin, seen to fail (AC-007).* `git mv standards/pages standards/pages-moved` →
`xtask failed: reading standards/pages: The system cannot find the path specified. (os error 3)`,
the `.with_context()` at `xtask/src/lint_pages.rs:395` naming the pinned path; and
`cargo xtask ci --fast` → `test result: FAILED. 217 passed; 44 failed` →
`xtask failed: tests failed with exit code: 101`. Moved back → `all required checks passed` →
`git status` clean.

## Commits

| SHA | subject |
| --- | --- |
| `263dc7b` | `feat(page-need-discipline): Governed page cites the discipline` |

One checkpoint commit, carrying the three edited files plus this story's `_ledger.md`,
`implementation-report.md` and `report.md`. Trailer: `Story: page-need-discipline/governed-page-cites-the-discipline`.

## Changes

| file | shape of the change |
| --- | --- |
| `docs/README.md` (`:13-15`) | **one added sentence**, below the second prose paragraph and above the `\| Page \| Read it at \|` table: `It routes rather than teaches: the library is explained on the pages below and not here, because [\`RP-10-2\`](../standards/pages/10-the-need-set.md) is the rule that shapes a page whose job is orientation.` Nothing existing is reworded, reordered or removed |
| `docs/append-conditions.md` (`:9-10`) | **one added sentence**, below the first prose paragraph and above the `rust` fence, four lines *below* the declaration: `It answers the one question its declaration names and no second one, because [\`RP-00-1\`](../standards/pages/00-one-need.md) is the rule that shaped it.` EC-001's tie-break citation, on the governed page the walk reports |
| `standards/pages/README.md` (`:8`) | **one added link**, inside region 2 (the scope paragraph): `The material these rules govern is the narrative tree at [\`docs/\`](../../docs/README.md).` One hunk, one line replaced by two. No heading added, no region moved, the generated region untouched |
| `.bklg/…/governed-page-cites-the-discipline/_ledger.md` | eight rows flipped to `satisfied: true` with captured evidence; a new `## EC-001 fired` section recording which page carries which citation and why; a new `## Blind spot` section stating the checked/unchecked asymmetry |
| `.bklg/…/governed-page-cites-the-discipline/implementation-report.md`, `report.md` | this file and the review-facing report |

Not changed, and each is a promise the diff itself has to keep: `xtask/src/**`,
`standards/rust/README.md`, `spec/`, `.kb/`, `.redkiln/templates/`, every `Cargo.toml`, `Cargo.lock`.
`git diff HEAD --stat -- xtask standards/rust spec .kb .redkiln/templates Cargo.lock` is empty.

## Gates

| command | result |
| --- | --- |
| `cargo run --locked --quiet -p xtask -- lint-pages` | `2 pages, 16 rules, all consistent` |
| `cargo run --locked -p xtask -- lint-pages --write` | same line, **no diff** |
| `cargo xtask lints` | green; `27 atoms, all consistent`, `2 pages, all consistent`, `2 pages, 16 rules, all consistent` |
| `cargo xtask spec-trace` | `traceability: no problems found; §7.1–§7.2 matches the checker` — the two rules owing a decision are the pre-existing pair, unchanged by this diff |
| `cargo xtask affected --base main` | `affected gate passed`; `396 file(s) changed against \`main\``, selecting `xtask` |
| `cargo xtask ci --fast` | `all required checks passed (--fast: 4 optional step(s) not run)` |
| `cargo xtask ci` | **`all checks passed`** |
| `redkiln doctor` | `doctor found no problems` plus **exactly six** `template-drift` warnings |

Formatting is part of `cargo xtask ci`'s first step (`=== formatting ===`) and is green; this diff
contains no Rust, so `cargo fmt` has nothing to reformat.

## Notes

**Deviation 1 — EC-001 fired, so the citation lands on two pages rather than one.** The spec decided
`docs/README.md` as the citer and made membership an *observation* rather than an assumption. The
observation says `README.md` is excluded from the checker's walk (`xtask/src/lint_pages.rs:496`,
inheriting `lint_constitution.rs:212-219`'s `name != "README.md"`), so the tie-break fired exactly as
pre-decided: `docs/append-conditions.md` — the governed page nearest the reader that the walk *does*
report, first in the index's own table and HS-P0020's fixture page — additionally carries a citation.
The checker's walk was **not** edited to widen the governed set; that is
`page-need-checker-mounted-in-the-gate`'s decision and `xtask/src/**` is out of scope.

**Deviation 2 — the rule cited differs per page, because a citation must be true of the page it sits
on.** RP-10-2 governs an `orientation` page. `docs/append-conditions.md` declares `explanation`, so
citing RP-10-2 there would assert a rule that did not shape it — the decorative outcome the spec's
Executive summary point 2 refuses. RP-00-1 is what shapes that page, and the sentence names it.

**Deviation 3 — AC-003's and AC-005's declaration assertions are observed on the governed citing
page.** Both rows were written expecting `docs/README.md` to carry a `> **Answers:**` declaration.
It does not, and it was deliberately **not** given one: a declaration on a page no checker walks
claims a governance nothing extends, which is the decoration this initiative exists to refuse (the
same argument the slice-mate's spec makes for `.kb` atoms). The rows are therefore observed on
`docs/append-conditions.md`, which declares exactly one need and whose H1/declaration region is
exactly one blank line, and the `docs/README.md` measurements are recorded beside them rather than
dropped. `_ledger.md` records this as a divergence under
`.kb/governance/rewrite-the-referent-never-the-reasoning.md`: the referent moved, the reasoning did
not.

**Deviation 4 — AC-007's walk was driven with `cargo xtask ci --fast`.** The `.with_context()`
message itself was captured from the step's own program (`cargo run -p xtask -- lint-pages`), and
`--fast` runs the same `tests` and `every page declares one need` steps that carry the observation.
The full `cargo xtask ci` was run separately, twice, to `all checks passed`.

**One transient to record rather than hide.** The first full `cargo xtask ci` run after the
`git mv` walk failed at `the constitution's examples compile` with `error: doctest failed`. It did
not reproduce: `RUSTDOCFLAGS="-D warnings" cargo test --locked -p xtask --doc` passes standalone
(`63 passed; 0 failed`), and the two subsequent full `cargo xtask ci` runs both reached
`all checks passed`. The cause was a stale doctest artifact left by moving the pinned tree out and
back underneath an incremental build, not anything in this diff — which contains no Rust at all.

**Nothing was softened in the blind-spot statement.** Having just watched the back-link get checked,
the temptation is to write "the links are checked". Half of them are. `_ledger.md` says which half,
in three bullets, and names the pin as what protects the other half instead of a check that does not
exist (RS-81-1, `standards/rust/81-checks-that-cannot-be-types.md:11`).
