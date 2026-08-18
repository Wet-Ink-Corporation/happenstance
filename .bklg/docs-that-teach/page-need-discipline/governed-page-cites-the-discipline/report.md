---
item: "HS-S0152"
stage: report
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Report — A governed page names the discipline as the reason it is shaped as it is

## Findings Ledger

**Outcome: delivered, eight of eight ACs satisfied, no AC deferred and none blocked.** One
pre-decided conditional fired at implementation time (EC-001) and is recorded rather than
re-litigated; three residuals are non-author walks owed at review, each named below with the
evidence that carries its row in the meantime.

**Mount points.** `docs/README.md:13-15` (the repository's documentation index — this story's stated
composition root), `docs/append-conditions.md:9-10` (EC-001's tie-break: the governed page the
checker's walk reports), `standards/pages/README.md:8` (region 2, the scope paragraph). All three
are the real, shipped surfaces; nothing here is behind a flag, a stub or a test-only path.

### AC by AC

| AC | result | what proves it | where |
| --- | --- | --- | --- |
| **AC-001** — the reason is on the page, with `RP-10-2` as visible link text, one keyboard hop to the rule | **met** | `rg -n 'RP-10-2' docs/README.md` → 1 hit; `rg -n '\[`RP-10-2`\]\('` → the same line, so the id is in the *link text*; `test -f standards/pages/10-the-need-set.md` → present; recorded traverse `page → link → file`, one step | `docs/README.md:13-15` → `standards/pages/10-the-need-set.md:77` |
| **AC-002** — the citer is a page the discipline governs, observed | **met, via EC-001's tie-break** | Probe A: an unenumerated token on `docs/README.md` produced *silence* (`2 pages, 16 rules, all consistent`) → README is outside the walk. Probe B: deleting `docs/append-conditions.md`'s declaration produced a problem line naming that file and pointing at `standards/pages/00-one-need.md` → that page is inside it. Both reverted, re-run green, `git status` clean | `xtask/src/lint_pages.rs:496`; `_ledger.md` `## EC-001 fired` |
| **AC-003** — the citing page still obeys the rule it cites | **met** | `cargo xtask ci` → `all checks passed`, `every page declares one need` → `2 pages, 16 rules, all consistent`; `rg -c '^> \*\*Answers:\*\*' docs/append-conditions.md` → `1`; `awk` 96-column sweep over all three files → no output; both added blocks pasted verbatim (one sentence each); paraphrase spot check run against RP-10-2's and RP-00-1's own text — no rule text reproduced | `_ledger.md` AC-003 |
| **AC-004** — the discipline links back, and the router's composition survives | **met** | `rg -n 'docs/README.md' standards/pages/README.md` → exactly 1, in region 2; one hunk at `@@ -4,7 +4,8 @@`; heading list and order identical to `HEAD`'s; `wc -c` → `5429` ≤ 8192; `lint-pages --write` → no diff; the back-link **seen to fail** when mistyped and green when restored | `standards/pages/README.md:8`; `xtask/src/lint_pages.rs:817` |
| **AC-005** — composed presentation, in place, never between H1 and declaration | **met** | first-20-lines capture of `docs/append-conditions.md`: line 1 H1, line 2 blank, line 3 the declaration — one blank line and nothing else; ordering by line number on `docs/README.md`: prose 3 < citation 13 < table 17; HTML / badge / admonition / bare-URL greps all exit 1 | `_ledger.md` AC-005 |
| **AC-006** — persistent chrome, no mechanism, no colour or icon carrier | **met** | `<details>`/`<summary>`/tab greps, `<small>`/`<sub>`/`<sup>`/`<nav>`/`<img>` greps and the icon grep all exit 1 over all three files; `lint_pages::tests::router_regions_are_in_the_binding_order` re-asserts the router's half on every `cargo xtask ci` | `_ledger.md` AC-006 |
| **AC-007** — a tree move breaks the build, not the link | **met** | `git mv standards/pages standards/pages-moved` → `xtask failed: reading standards/pages: The system cannot find the path specified. (os error 3)` and `cargo xtask ci --fast` → `217 passed; 44 failed`; moved back → `all required checks passed`; `git status` clean | `xtask/src/lint_pages.rs:395` |
| **AC-008** — the blind spot is stated, and no two-tree scanner was built | **met** | `_ledger.md` `## Blind spot` names which half is checked and which is not, in one sentence each; `git diff HEAD -- xtask` empty; `rg -n 'What checks this tree, and what does not' standards/pages/README.md` → `79:` | `_ledger.md` `## Blind spot` |

### The one conditional that fired, and what a reviewer should check

**EC-001 fired.** The checker's walk excludes `README.md` (`xtask/src/lint_pages.rs:496`), so
`docs/README.md` is not a page the discipline mechanically governs. The pre-decided tie-break was
applied without re-opening scope: `docs/append-conditions.md` — the governed page nearest the reader
that the walk *does* report — additionally carries a citation, and `_ledger.md` records which page
carries which. The checker's walk was **not** widened to "fix" this; `xtask/src/**` is untouched.

A reviewer's sharpest question here is *why two different rule ids*. Because a citation must be true
of the page it sits on: RP-10-2 governs an `orientation` page, and `docs/append-conditions.md`
declares `explanation`. Citing RP-10-2 there would be an assertion about a rule that did not shape
that page, which is precisely the decorative outcome this story exists to refuse.

### Deferred, and by whom

Nothing is deferred as *behaviour*. Three **judgements** are owed to a non-author and are named in
the ledger rather than hidden:

1. **AC-001's read-back** — a person who did not write the sentence stating, from the sentence
   alone, which property of the page RP-10-2 dictates. Run by the implementer and labelled as such;
   the mechanical assertions and the recorded traverse carry the row.
2. **AC-003's paraphrase spot check** — run by the implementer against RP-10-2's and RP-00-1's
   shipped text; no rule text is reproduced by either sentence.
3. **AC-008's blind-spot read-back** — someone else restating which half of the relationship is
   mechanically checked. The gate-state diffs carry the row.

All three are the same residual this project's earlier stories recorded and routed to the review and
integration stages; none of them is a licence to skip that walk.

### Boundary promises the diff itself keeps

`git diff HEAD --stat -- xtask standards/rust spec .kb .redkiln/templates Cargo.lock` → **empty**.
No checker change, no scanner over `docs/`, no `affected.rs` edit, no `standards/rust/README.md`
edit (which would have invalidated `router-precedence-and-announcement`'s satisfied AC-002 one slice
back), no `spec/` write, no `.kb/` write — the playbook atom is the slice-mate's — and no template
drift beyond the standing six. `redkiln doctor` reports `doctor found no problems` and exactly six
`template-drift` warnings.

Note for the reviewer: `git diff main -- xtask` is *not* empty and is not the right base. It carries
this project's own dependency stories, unmerged to `main` — `git log --oneline main..HEAD -- xtask`
names `ee0a500 Page-need checker mounted in the gate` and fifteen others from HS-P0020 and HS-P0021.
The story-scoped diff against the slice base is the one that can prove this story's promise, and it
is empty.

### DoD

Initiative **DoD-14**'s third and last clause — *at least one page cites it as the reason it is
shaped as it is* — closes here, and the relationship is now legible from both ends in one keyboard
hop each way. Project **AC-011**'s *cited in place* half is discharged in full; its *announced*
half was `router-precedence-and-announcement`'s and is already merged.
