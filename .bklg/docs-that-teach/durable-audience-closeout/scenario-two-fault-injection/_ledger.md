---
item: HS-S0181
stage: implement
created: 2026-08-17T13:16:30.754Z
updated: 2026-08-17T13:16:30.754Z
---

# Acceptance ledger — Re-observe scenario 2 in both halves: fail by name, then recover

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes specific to this story, both from its spec:

- The `mount_point` is the fifteen-row DoD re-observation ledger **table**, not the filename. If
  `dod-scenario-ledger` (HS-S0180) created that table under a different name in this project's
  folder, that name wins (spec, EC-008) — correct the `mount_point` here and in the spec's
  Integration contract rather than creating a parallel file.
- AC-015 and AC-014 are Tier 4 with no secondary tier
  (`.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md:874-875`). Several rows below
  therefore name a human review or a `git`/`rg` observation as the verifying test — there is no Rust
  test to point at, and citing one would be the fabrication this story exists to prevent.

```yaml
- id: AC-001
  criterion: "GIVEN the closeout reviewer needs to re-run this observation on the same tree the closeout was taken on, WHEN scenario 2's re-observation begins, THEN it runs inside the fresh checkout `dod-scenario-ledger` created off the merged branch — never `.claude/worktrees/docs-that-teach` — and that checkout's path and the merged tree sha are recorded before the first command is issued and repeated in both ledger rows."
  satisfied: false
  evidence: "" # file:line and/or verifying test id — required once satisfied
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/_dod-ledger.md — the scenario-2 rows' checkout-path and tree-sha cells"
  verifying_test: "Tier 1 static: `rg -n \"<merged-sha>\" .bklg/docs-that-teach/durable-audience-closeout/_dod-ledger.md` returns both scenario-2 rows and `git rev-parse <merged-sha>` resolves; the same path and sha head .bklg/docs-that-teach/durable-audience-closeout/scenario-two-fault-injection/_evidence-failing.md and _evidence-recovered.md"

- id: AC-002
  criterion: "GIVEN a reviewer who must be able to tell a real falsification from a typo, WHEN the page under the pinned narrative tree is broken, THEN exactly one claim on exactly one page is made untrue of the library — a fence that no longer compiles against it, or a cited clause id that no longer resolves — with the break, the page and the reason it is a claim break rather than a formatting break written down in `_break-note.md`."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/scenario-two-fault-injection/_break-note.md — cited from the `failed by name` row's evidence cell in _dod-ledger.md"
  verifying_test: "Tier 2 content review of .bklg/docs-that-teach/durable-audience-closeout/scenario-two-fault-injection/_break-note.md against spec B2 and the pinned constants at .bklg/docs-that-teach/checked-documentation-surface/_design.md:93 (`const TREE = \"docs\"`) and :96 (`const HARNESS`)"

- id: AC-003
  criterion: "GIVEN a reviewer who was burned by a step that was vouched for by two documents and printed `skipped` on all three runners (`RUNBOOK.md:918-925`), WHEN the gate is run against the broken tree, THEN the failing half's evidence is the run's output captured byte-for-byte into `_evidence-failing.md` — banner, problem line(s) and terminal failure — and `_break-note.md` records the file the gate actually named, even where that is the harness rather than the page, with no word of the transcript reworded to agree with the expectation."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/_dod-ledger.md — the `failed by name` row's evidence cell, linking .../scenario-two-fault-injection/_evidence-failing.md"
  verifying_test: "Tier 4 e2e: the failing `cargo xtask ci` run (.redkiln/config.yaml:60), read as Tier 2 content review — .bklg/docs-that-teach/durable-audience-closeout/scenario-two-fault-injection/_evidence-failing.md carries one fenced verbatim block whose problem line matches `  {path}:{line} — {message}` under `=== every narrative page is checked ===` (.bklg/docs-that-teach/checked-documentation-surface/_design.md:37, :761), and the file named there equals the file named in _break-note.md"

- id: AC-004
  criterion: "GIVEN the terminal `cargo xtask ci` in `terminal-gate-run` must be taken on a tree that does not contain the break, WHEN the failing transcript has been captured, THEN the page is restored to its committed content, the working tree is observed clean of the break before anything is committed, and no commit in this story's history contains the broken page."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/scenario-two-fault-injection/_break-note.md — the recorded post-revert cleanliness observation, cited from both scenario-2 rows"
  verifying_test: "Tier 1 static: `git status --porcelain` recorded in _break-note.md showing the page absent from the change set, plus `git log -p -- <broken page>` and `git log -S \"<the broken claim string>\"` over this story's commit range returning nothing"

- id: AC-005
  criterion: "GIVEN a reviewer who must see the same instrument recover, not a cheaper one pass, WHEN the revert is in place, THEN the same command run in AC-003 is re-run on the reverted tree and its green output is captured verbatim into `_evidence-recovered.md` as the recovery half's evidence — not `--fast`, not a narrower grain, not a different command."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/_dod-ledger.md — the `recovered` row's evidence cell, linking .../scenario-two-fault-injection/_evidence-recovered.md"
  verifying_test: "Tier 4 e2e: the green `cargo xtask ci` re-run (.redkiln/config.yaml:60), plus a Tier 2 diff of the command line at the head of .../scenario-two-fault-injection/_evidence-recovered.md against the one in _evidence-failing.md — they must be character-identical and resolve to a real command (xtask/src/main.rs), per AC-TB-02 (.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md:827-829)"

- id: AC-006
  criterion: "GIVEN a reviewer reading the DoD ledger one row at a time, WHEN they reach scenario 2, THEN they find two named rows — outcome `failed by name` and outcome `recovered` — mounted in the project's fifteen-row re-observation ledger in place of its single placeholder row, each carrying scenario id, owning project (HS-P0020), observer, checkout path, tree sha, what was seen and a link to its own transcript, each understandable without reading the other, and neither row's outcome column containing the string \"inherited from a sibling\"."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/_dod-ledger.md — scenario 2's position in the fifteen-row table"
  verifying_test: "Tier 3 mount-point walk of .bklg/docs-that-teach/durable-audience-closeout/_dod-ledger.md (two rows at scenario 2, no placeholder left, both evidence links resolve in one hop) with `rg -n \"scenario 2\" .bklg/docs-that-teach/durable-audience-closeout` showing no second record; Tier 1 `rg -n \"inherited from a sibling\" .bklg/docs-that-teach/durable-audience-closeout` returns nothing; Tier 2 read of each row in isolation against AC-UX-10 (_decomposition.md:270-273) and AC-UX-11 (:274-277)"

- id: AC-007
  criterion: "GIVEN a reader consuming the ledger as plain text — in a diff, a quote, a terminal or a screen reader — WHEN they read either scenario-2 row, THEN every state it expresses is a written word: no outcome carried by colour, emoji, glyph, strikethrough, row ordering or an empty cell, no \"the failing row is the one above the green one\", every not-applicable written out, and each row's load-bearing verdict also stated in a sentence outside the table."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/_dod-ledger.md — both scenario-2 rows and the prose sentence beside the table"
  verifying_test: "Tier 2 content review against AC-UX-09 (.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md:266-269), IQ-8 (:221-224) and the plain-text-equivalence rule (:142-145); Tier 1 `rg -n \"[✅❌✓✗~]\" .bklg/docs-that-teach/durable-audience-closeout/_dod-ledger.md` returns nothing for the scenario-2 rows and no cell in either row is empty"
```
