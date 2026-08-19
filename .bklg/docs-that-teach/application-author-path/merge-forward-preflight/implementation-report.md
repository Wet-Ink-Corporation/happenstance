---
item: "HS-S0183"
stage: implement
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Implementation Report — Merge forward and record the baseline before authoring

## TDD Evidence

This story compiles nothing of its own, so its "tests" are the tiers
`_decomposition.md` § Testing brief names — a captured command outcome against a named sha,
and a re-derivation of every recorded number. To make Red/Green real rather than rhetorical,
those re-derivations were written **first**, as a single runnable check
(`check_baseline.py`, built and run outside the tree per the same discipline the slice-mate's
spec sets for probes). It parses `_baseline.md`, re-derives every claim from the merged tree,
and fails on any disagreement — so transcribing a number wrongly is a red test, not a
reviewer's job.

First run, before `_baseline.md` existed:

```console
$ python check_baseline.py
== HS-S0183 baseline check ==
  [FAIL] AC-001..AC-005 the baseline record exists — …/_baseline.md is missing
RED: 5 acceptance criteria unverifiable; the record this story authors does not exist.
EXIT: 1
```

Final run, after the record was authored: **61/61 checks passed, GREEN**.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `check_baseline.py` AC-001 block (11 assertions) — parents from `git rev-list --parents -n 1`, `git merge-base --is-ancestor` exit 0, `lib.rs` line count and both `Tags::empty()` sites, the rendered selector located in `target/doc/happenstance/index.html`, and the scoped non-occlusion diff | **Red**: record missing, all five ACs unverifiable. **Green**: 11/11 after `_baseline.md § Merge` was written from the measurements |
| AC-002 | `cargo xtask ci --fast` (exit 0), `cargo xtask affected --base main` (exit 0), plus `check_baseline.py` AC-002 block asserting `§ Gate` names both commands, ties them to `a5c0f30`, records an outcome, and that `git diff a5c0f30 HEAD` over non-`.bklg` paths is empty | **Red**: record missing. **Green**: 6/6; both gate commands run in this session and transcribed |
| AC-003 | `check_baseline.py` AC-003 block — it **executes** each of the 17 anchor rows' own re-derivation command and asserts the printed line equals the recorded one; plus `cargo xtask spec-trace` exit 0 | **Red**: 17/17 rows failed on the first authored draft, because every row named `rg -n …` and `rg` is not on this machine's `PATH` — the command was unrunnable, which is the same defect as a wrong line number. **Green**: rewritten as `git grep -nF`, 18/18 |
| AC-004 | `check_baseline.py` AC-004 block — ≥4 disposition rows, each status matching `carried \| routed to <item> \| fixed here`, and `git diff --stat 3fd3866 HEAD -- …/_design.md` empty | **Red**: record missing. **Green**: 11/11 over nine rows |
| AC-005 | `check_baseline.py` AC-005 block — it rebuilds the render with `cargo doc -p happenstance --no-deps` and re-derives all five composition numbers, asserts no Rust fence in the record, and asserts every file `git diff-tree --cc --name-only a5c0f30` reports is named in the PR-boundary fence | **Red twice, and both reds were real findings.** (1) recorded brackets `0`, measured `2` — because the gate's own doc build had overwritten the render, which is how the invocation-dependence was found. (2) `7 resolved, undeclared: ['xtask/src/main.rs']` — the fence named six of seven. **Green**: 8/8 after both were recorded and the seventh path was added |

The two AC-005 reds are the reason this story was worth writing a check for: neither was
visible to a reviewer reading the record, and both changed what the record says.

## Commits

This story's own checkpoint sha cannot appear in this file — the file is inside the commit,
which is the paradox the template names. It is recorded on the item by
`redkiln record-links HS-S0183 --sha <sha>`, which the orchestrating command runs after the
checkpoint exists; the CLI is the only writer of an item's `links`. The slice-mate's
implementation report carries it, because by then it exists.

The **work commit** this story delivers, which does exist and is citable:

| SHA | Subject |
| --- | ------- |
| `a5c0f300f78b9f0289bc7d1575ce35b47dcd44f5` (`a5c0f30`) | `Merge initiative/from-contract-to-published-library into docs-that-teach` — parents `3fd3866` (ours) and `4327ce3` (theirs); the mount this story's whole record is about |

The checkpoint commit is `feat(application-author-path): Merge-forward preflight`, carrying
`_baseline.md`, the five flipped ledger rows, the fence correction in `spec.md`, and both
stage-artifact bodies.

## Changes

| File | Shape of the change |
| --- | --- |
| `.bklg/docs-that-teach/application-author-path/merge-forward-preflight/_baseline.md` | **New, 254 lines.** The deliverable: the five sections `spec.md` fixes — `§ Merge`, `§ Gate`, `§ Anchors` (17 rows, each with a runnable `git grep -nF`), `§ Dispositions` (nine rows), `§ Composition baseline` (six invariants) — plus a re-derivation appendix |
| `.bklg/docs-that-teach/application-author-path/merge-forward-preflight/_ledger.md` | Five rows flipped `false → true`, each with cited evidence. No criterion re-worded, none removed |
| `.bklg/docs-that-teach/application-author-path/merge-forward-preflight/spec.md` | Body only. One path added to the PR-boundary fence (`xtask/src/main.rs`) and a dated amendment note explaining why the list said six when `git diff-tree --cc` says seven |
| `.bklg/docs-that-teach/application-author-path/merge-forward-preflight/implementation-report.md`, `report.md` | New stage artifacts, authored bodies |

No file outside this story's folder was authored, and no frontmatter field was touched. The
merge commit `a5c0f30` was already on the branch when this story began — it was performed in
`/redkiln:implement` preflight under human supervision, which `spec.md § PR boundary` records.

## Gates

| Command | Result |
| --- | --- |
| `python check_baseline.py` (the story's own tests, outside the tree) | **61/61 GREEN** |
| `cargo xtask ci --fast` (`.redkiln/config.yaml:55`, the non-terminal project bar) | **exit 0** — "all required checks passed (--fast: 4 optional step(s) not run)" |
| `cargo xtask affected --base main` (`.redkiln/config.yaml:41`, the story grain) | **exit 0** — "affected gate passed"; 228 doctests passed, 0 failed |
| `cargo xtask spec-trace` | **exit 0** — 201 clauses, 112 rules, 401 citations, "no problems found" |
| `cargo xtask lints` (with `spec-trace`, `reachability_static`) | **green** — 27 constitution atoms, 2 narrative pages, 2 page-need checks |
| `cargo doc -p happenstance --no-deps` | **exit 0**; the render every composition number is read off |
| `redkiln verify --grain story --item HS-S0183` | `[ok] affected-gate`, `[ok] ledger`, `[FAIL] boundary`, `[FAIL] provenance` — both failures forecast by EC-007 and by CLI sequencing; see Notes |

The formatter is inside `cargo xtask ci --fast` as its first REQUIRED step
(`cargo fmt --all --check`, `xtask/src/main.rs:124`) and was green. No Rust source was
touched by this story, so there was nothing for it to reformat.

## Notes

**Four things went differently from the plan, and each is recorded rather than smoothed.**

1. **EC-004 fired.** `spec.md` states every pre-merge measurement against sibling tip
   `3f49ec6`; the tip actually merged is `4327ce3`. Every predicted post-merge line number in
   the corpus is therefore wrong — ES-25 is at `:3756`, not `:3698`; `const REQUIRED` at
   `:120`, not `:117`. This is the failure the story exists to prevent downstream, arriving
   inside the story itself, and it is why `§ Anchors` carries commands rather than numbers.
2. **`rg` is not on this machine's `PATH`.** The spec's verification cells name `rg -n`, which
   is the repository's convention in prose. Every anchor row was rewritten as `git grep -nF`
   after all 17 failed to run. NF-002 says the record must be self-sufficient cold; a command
   that only runs in one sandbox is not.
3. **The bracket count is a property of the `cargo doc` invocation, not of the page.** Zero
   under `cargo doc -p happenstance --no-deps`; two under the gate's own `documentation` step,
   which exits 0 with `RUSTDOCFLAGS=-D warnings` and two unresolved intra-doc links. Found
   because the check went red between two runs. Routed: page half to
   `boundary-refusal-encounter`, gate half to the `support` initiative.
4. **The PR-boundary fence named six of seven resolved files.** `git diff-tree --cc
   --name-only a5c0f30` reports `xtask/src/main.rs` as well — a genuine union resolution the
   amendment's own prose already described while the fence omitted the path. Added, with the
   correction dated and explained in the same block.

**Five findings beyond the four the spec measured** are in `§ Dispositions` rows 5–9. The
largest is row 6: four of `_design.md`'s six substrate gaps have **closed** on the sibling
branch. The narrative tree is pinned (`docs/`, `xtask/src/narrative.rs:124`), the render shape
is markdown, the answered-need notation exists with its own standard
(`standards/pages/00-one-need.md`), and both the compiled-fence and file-reading narrative
steps are in `REQUIRED` and green. `{tree}` in `_design.md § Surfaces` now resolves. The
consequence for the 22-character heading budget's *scope* is deliberately left to the
slice-mate — `tension-resolutions` AC-004 owns that reconciliation against sign-off
condition 2, and taking it here would have been this story re-deciding a signed-off design.

**Nothing was absorbed.** `_design.md` is byte-identical; every contradiction carries a
disposition and every route names a real item id.
