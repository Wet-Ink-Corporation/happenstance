# Observed — the story-grain gate on a change confined to the narrative tree

AC-005's evidence. Dated evidence of one observation, not a golden file: nothing in
the test suite compares against it, so it cannot rot into a failing test (spec
clarification 8). RS-81-4 is why the evidence is the *parsed output naming the
package* rather than a zero exit status.

**Date.** 2026-08-17. **Tree.** `initiative/docs-that-teach` at `dad0b86`
(`feat(checked-documentation-surface): A prose-only change selects the package that
compiles it`), with the slice-mate's tree and harness already in it.

**The change under observation.** One sentence appended to a single page under the
narrative tree — `docs/append-conditions.md` — and nothing else. `git status
--porcelain` immediately before the run:

```
 M docs/append-conditions.md
```

**The invocation.** `cargo xtask affected --base HEAD`.

`--base HEAD` rather than `--base main`, and the difference is the branch rather than
the command. `.redkiln/config.yaml:40` runs `cargo xtask affected --base {{base}}`,
and `changed_files` (`xtask/src/affected.rs:547-571`) unions four sources: the diff
from the merge-base to `HEAD`, the staged set, the unstaged set and the untracked
set. On this branch the first of those is the whole initiative — 345 files against
`main`, including every planning artifact — so no invocation with `--base main` can
be tree-confined here, and one that pretended to be would be observing the branch
rather than the change. `--base HEAD` collapses the first source to nothing and
leaves exactly the working-tree edit above, which is the diff shape the story-grain
gate meets mid-story and the one AC-005 is about. Same command, same code path, same
`affected_packages` call.

## Transcript

Verbatim, with the file-reading block's body elided at the marked point only — it is
`spec-trace`'s and `lints`', it is unchanged by this story, and it runs before the
block AC-005 is about.

```
=== the file-reading checks ===
§7.4: 0 disposition(s) against 95 rule(s) in 3 file(s), none naming a rule still live
CF-33: 7 file(s) in crates/happenstance-testkit/src read no clock
CF-6: no position-shaped literals in 3 rule file(s)
CF-29: all 95 rules in 3 file(s) have a changelog entry
CF-32: crates/happenstance-testkit/Cargo.toml carries its own `version = "0.2.0"`
D12: crates/happenstance-core/Cargo.toml's `serde = [...]` names both `serde/alloc` and `base64/alloc`
200 clauses (139 FROZEN, 49 PROVISIONAL, 10 DEFERRED, 2 NON-NORMATIVE), 95 conformance rules, 58 e2e cases, 358 citations checked (69 anchored to their subject, 12 external)
2 rule(s) claimed by no clause and owing a decision:
  [... the two standing ADR-owed rules, elided — unchanged by this story ...]
traceability: no problems found; §7.1–§7.2 matches the checker

=== affected packages ===
1 file(s) changed against `HEAD`
  xtask

=== formatting ===

=== clippy (affected packages) ===
    Checking xtask v0.2.0 (D:\repos\happenstance\.claude\worktrees\docs-that-teach\xtask)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.61s

=== tests (affected packages) ===
   [... unittests src\lib.rs, unittests src\main.rs, and the doctest target ...]

test result: ok. 231 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out; finished in 16.65s


affected gate passed
```

## What the transcript is evidence of

| AC-005 requires | Where it is in the transcript |
| --------------- | ----------------------------- |
| `=== affected packages ===` | Present, and still the block's first line — a reviewer scanning a long log finds the same landmark in the same place. |
| The `{n} file(s) changed against \`{base}\`` line | `1 file(s) changed against \`HEAD\``. |
| `xtask` on its own two-space-indented line | `  xtask`. 7 characters, indented 2 — nowhere near the 80-column budget, and the longest member name in this workspace (`happenstance-cloudflare`, 23) would not reach it either. |
| **Not** `no package affected — nothing to compile` | Absent. That line is a *state*, not chrome, and it must stop appearing for a tree-confined diff; the branch that prints it (`xtask/src/affected.rs:135-145`) is now unreachable for one. |
| The fmt, clippy and test steps actually running | Three banners, then a real `cargo test` over 231 tests. Before this story the same diff ran none of them. |
| `affected gate passed` as the single closing summary line | Last line, one line. |
| No composition added, nothing hidden by the addition | The block is `xtask/src/affected.rs:130-149` unchanged: no colour, no bold, no tick, no badge, no glyph, no spinner, no per-path progress line, and no truncation — `xtask` is one more `println!` in the existing sorted loop. |

## The counter-observation, and its honest limit

Removing the arm's third disjunct (`|| path.starts_with("docs/")`) and re-running the
same command on the same working tree printed **six** package names rather than one:
with `"docs/"` gone from `INERT`, a page path is now unrecognised and widens to the
whole workspace. That is the module's stated posture — an unattributable path errs
toward *more* packages — and it demonstrates that the two halves of this change are
jointly load-bearing rather than one being cosmetic.

Reconstructing the *full* pre-story state (arm absent **and** `"docs/"` back on
`INERT`), which is what printed `no package affected — nothing to compile`, is not
obtainable as a transcript on this tree: doing it requires editing
`xtask/src/affected.rs`, and that edit is itself a change to a member directory, so
the run then selects `xtask` for the wrong reason and reports two changed files
instead of one. The mechanical statement of the pre-story behaviour is therefore the
red run of `a_narrative_page_selects_xtask`, which failed with

```
assertion `left == right` failed
  left: {}
 right: {"xtask"}
```

— an empty selection for `docs/append-conditions.md`, driven through the same
`affected_packages` function with no git repository and no compiler. Recorded here
rather than left implied, because a transcript that could not be produced is worth
more said than quietly omitted.
