---
item: HS-S0128
stage: implement
created: 2026-08-12
updated: 2026-08-12
---

# Acceptance ledger — The DoD 13 delta between the published tag and the closeout tree, stated

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

This story is **static / process** tier throughout — `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md:47`
is explicit that AC-004 is proven by "a checked, cited artefact", not a test-framework check. Every
`verifying_test` below is therefore a re-runnable command plus the transcript path it is checked
against, not a `#[test]` id. That is the honest form of the field here; a fabricated test id would be
worse evidence than none.

```yaml
- id: AC-001
  criterion: "GIVEN a reader who has just been told the initiative shipped `0.2.0`, WHEN they read the near end of the delta, THEN they find a tag they can check out for themselves: the tag string exactly as it exists in this repository and the commit SHA it points at, both read from the tree rather than copied from a spec — and if no tag resolves, or several plausibly do, the section says so, states what was searched, and raises a finding naming HS-P0016 as owner instead of rendering a plausible SHA."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md — section `## DoD 13 — the published-tree delta`"
  verifying_test: "static/process: `git tag --list` and `git rev-parse <tag>^{commit}` (plus `git show <tag>`) re-run on the closeout tree and compared against the transcript at .bklg/from-contract-to-published-library/closeout-and-durable-audience/published-tree-delta-statement/_delta-log.md"

- id: AC-002
  criterion: "GIVEN a reader who wants to know which tree the gate was green on, WHEN they read the far end of the delta, THEN it is the commit SHA `whole-gate-green-on-the-assembled-tree` recorded as the tree `cargo xtask ci` exited zero on — never `HEAD` at authoring time — and the section states that SHA, whether the tag is an ancestor of it, and the count of commits in the range, so the reader can tell whether the table below it is complete rather than curated."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md — section `## DoD 13 — the published-tree delta`"
  verifying_test: "static/process: `git merge-base --is-ancestor <tag> <gate-sha>` (exit code recorded) and `git rev-list --count <tag>..<gate-sha>` compared against the count stated in the section and the SHA recorded at .bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md:192-195; transcripts in published-tree-delta-statement/_delta-log.md"

- id: AC-003
  criterion: "GIVEN a reader who was told 'two projects landed after publication', WHEN they read the commit table, THEN every commit in the range carries an owner — `replication-identity-and-ingest` (HS-P0017), `retention-and-incomplete-logs` (HS-P0018), or something else, named — with zero unattributed rows and zero commits omitted; residue is classified as backlog/planning-only, docs-only, or code; and a code commit owned by neither expected project is escalated with its owning sibling named. DR-4's two owners are the hypothesis being tested, never the filter applied."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md — section `## DoD 13 — the published-tree delta`"
  verifying_test: "static/process: `git log --oneline --no-merges <tag>..<gate-sha>` matched row-for-row against the section's table (row count equals AC-002's stated count) and `git show --stat <sha>` per residue commit; transcripts in published-tree-delta-statement/_delta-log.md"

- id: AC-004
  criterion: "GIVEN a reader deciding whether the API they may already have built against moved under them, WHEN they read the delta's published-surface claim, THEN it is carried by two citations to work done elsewhere and is checkable in one hop each: the gate decision constraining retention's answer to what needs no published-surface change (.bklg/from-contract-to-published-library/_decomposition.md:239-249), and the surface-diff record showing how that came out (.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_storymap.md:78), with the byte-identical `EventStore` signature commitment beside it (.bklg/from-contract-to-published-library/replication-identity-and-ingest/_storymap.md:58). No surface diff, `cargo-semver-checks` run or version class is re-derived here."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md — section `## DoD 13 — the published-tree delta`"
  verifying_test: "static: citation resolution — a reviewer opens all three cited paths on the closeout tree and confirms the cited lines say what the section claims; plus the negative check that no `cargo-semver-checks` invocation and no `crates/` diff appears in this story's commits (`git diff --stat` over the story's SHAs)"

- id: AC-005
  criterion: "GIVEN a reader who has read DoD 13's actual words, 'on the exact tree that was published' (.bklg/from-contract-to-published-library/initiative.md:396-397), WHEN they reach DoD 13 in the closeout record, THEN they find that phrase explicitly qualified in the record's own words: that byte-identity is not satisfiable on this tree and why (merge positions 7 → 8 → 9 → 10, _decomposition.md:178-193), and what was observed in its place — `cargo xtask ci` green on the assembled whole from a clean checkout, with the delta from the tag bounded and attributed. Nowhere in the artefact is DoD 13 marked green on an unqualified byte-identity claim."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md — section `## DoD 13 — the published-tree delta`"
  verifying_test: "static: read of .bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md — the qualifying statement is present in the DoD 13 section and a search of the whole artefact finds no DoD 13 pass mark that is not adjacent to it"

- id: AC-006
  criterion: "GIVEN the same reader working down the fourteen-row DoD table, WHEN they reach the DoD 13 position, THEN one pointer takes them to `## DoD 13 — the published-tree delta` in the same `_closeout-record.md` — appended, with no earlier section re-worded, re-ordered, truncated or displaced — and every escalation the delta raised is written where `findings-disposition-register` (HS-S0134) picks it up, with zero defects fixed inside this story."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md — section `## DoD 13 — the published-tree delta`, reachable in one hop from the slice-mate's DoD 13 table position"
  verifying_test: "static: read of _closeout-record.md for the exact heading; `git diff` of this story's commits over that file showing additions only (no deletions or modifications above the new heading); the slice-mate's DoD 13 position naming that heading; and `git diff --stat` for this story's commits confined to the two PR-boundary globs"
```
