---
item: "HS-S0172"
stage: implement
created: "2026-08-17T13:16:24.878Z"
updated: "2026-08-17T13:16:24.878Z"
---

# Acceptance ledger — Merge the sibling branch forward and name the baseline tree

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

**Two notes specific to this story.** First, no row's `verifying_test` is a Rust `#[test]`: this
story compiles nothing and `AC-TB-08` forbids implying otherwise, so the verifying tests are the
tiers named in `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md`, `## Testing
brief` — a captured command outcome, or the Tier 2 human read against IQ-1…IQ-8. Second, evidence
must be the *measured* value, never a figure copied out of `spec.md`: every number in the Context
pack was measured on 2026-08-17 against a branch that was still moving (spec.md, D8).

```yaml
- id: AC-001
  criterion: "GIVEN U3 asks \"does this branch actually contain the sibling's contract work, or only a claim that it does?\", WHEN they resolve the branch tip of initiative/docs-that-teach, THEN it is a commit with two parents — this branch's pre-merge tip and the sibling branch's tip at merge time — created by `git merge --no-ff initiative/from-contract-to-published-library` on a worktree that was clean (`git status --porcelain` empty) before the merge ran, so the sha names one tree that reviewed work produced rather than a tree with uncommitted planning mixed into it. Not a rebase, not a squash, not a checkout of the sibling."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/_baseline.md"
  verifying_test: "Tier 1 static — `git cat-file -p HEAD` shows exactly two `parent` lines; `git merge-base --is-ancestor <parent2> HEAD` exits 0; `git log --oneline HEAD^1` shows the six pre-merge commits unrewritten. Outputs captured into _baseline.md."

- id: AC-002
  criterion: "GIVEN U3 opens exactly one file to answer \"which tree was this closeout observed against?\", WHEN they open ../_baseline.md and read nothing else, THEN they get the full 40-hex merged sha, both parent shas, the sibling branch name, the branch the merge sits on, the merge date and the observer — each as a literal copyable string, none of them behind a link, a `git log` invocation, or a phrase like \"see the item\". Downstream records cite this file; none re-derives the sha."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/_baseline.md"
  verifying_test: "Tier 2 content review — _baseline.md read alone against IQ-1's falsifier (.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md, ## UX brief, IQ-1); plus Tier 1 `git cat-file -e <sha>^{commit}` for every sha in the record and a string-compare against `git rev-parse HEAD HEAD^1 HEAD^2`."

- id: AC-003
  criterion: "GIVEN U2 must be able to trust that this project edited none of the sibling's files while claiming only to have merged them, WHEN they diff the merged tip against the sibling tip over the paths ../_decomposition.md §1 marks read-only, THEN the result is empty: `git diff --name-only <parent2> HEAD -- crates spec xtask standards docs examples .kb` prints nothing, and `git diff --name-only <parent2> HEAD -- references` names at most references/seeds/user-documentation.md. Every cited file:line in the planning corpus therefore still means what it meant."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/_baseline.md"
  verifying_test: "Tier 1 static — the two `git diff --name-only` commands above, run immediately after the merge and pasted verbatim (including empty output) into _baseline.md. This is the compensating control for the `**` PR boundary; a non-empty result fails the story (AC-A10)."

- id: AC-004
  criterion: "GIVEN U2 cannot tell a clean merge from an unexamined one by looking at the tree, WHEN they read ../_baseline.md's conflict section, THEN they find either one row per conflicted path (the path, which side's content survived, and the one-sentence reason) or the explicit sentence that no path conflicted — never an absent section, an empty heading, or a summary that replaces the rows. Where a read-only path conflicted, the row states that the sibling's content won and why."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/_baseline.md"
  verifying_test: "Tier 2 content review against IQ-8 (.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md, ## UX brief) — no state expressed by omission; corroborated by Tier 1 `git log -1 --format=%B HEAD` and, per conflicted path, `git diff <parent2> HEAD -- <path>` showing the stated survivor."

- id: AC-005
  criterion: "GIVEN U2 and U3 need the planning corpus's three now-stale premises re-observed rather than silently outgrown, WHEN they read ../_baseline.md's premise section, THEN each of D2's three rows appears as premise-as-written (with its cite) → what the merged tree shows (with the command that showed it, output verbatim) → the story slug that owns the decision — and nothing is adjudicated here: no .kb/_intake/ file is narrowed, deleted or ingested, no persona pair is dispositioned, no decision atom is treated as settled."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/_baseline.md"
  verifying_test: "Tier 2 content review of the three rows, each carrying its command and literal output (`ls .kb/_intake/`; the counterpart item's path plus its `stage:`/`status:` read from the merged tree; `ls .kb/decisions/`); corroborated by Tier 1 `git diff --name-only <parent2> HEAD -- .kb` being empty, which proves no adjudication happened in this diff."

- id: AC-006
  criterion: "GIVEN the baseline must be machine-readable and not only prose, WHEN `redkiln verify --grain story` reads HS-S0172 under require_commit_provenance: true, THEN links.commits carries the merged sha, written by `redkiln record-links HS-S0172 --sha <merged-sha>` and by nothing else — the item's YAML frontmatter is never hand-edited (a PreToolUse hook denies it), and the sha in links.commits is byte-identical to the one in ../_baseline.md."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/merge-forward-baseline/story.md (links.commits, CLI-written)"
  verifying_test: "Tier 1 static — `rg \"commits\" .bklg/docs-that-teach/durable-audience-closeout/merge-forward-baseline/story.md` shows the sha and it string-matches _baseline.md's; `redkiln verify --grain story` for HS-S0172 reports no missing commit provenance (.redkiln/config.yaml:73)."

- id: AC-007
  criterion: "GIVEN a downstream story hits a failure and must know whether the merge caused it, WHEN they read ../_baseline.md's baseline-health line, THEN they find the exact command `cargo xtask affected --base main` (.redkiln/config.yaml:40), the date it ran, and its outcome — green, or the failure captured verbatim with the sentence routing it to the support initiative (.redkiln/config.yaml:5). The line explicitly states it is not AC-016 evidence, and nothing sibling-owned is repaired in this story."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/_baseline.md"
  verifying_test: "Tier 1 for the run — `cargo xtask affected --base main` (xtask/src/main.rs), exit code and output recorded; Tier 2 for the attribution — the reviewer checks the record carries the command string, the date, the outcome, the non-AC-016 disclaimer and, on failure, the routing sentence rather than a fix (EC-004)."
```
