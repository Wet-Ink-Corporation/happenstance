---
item: HS-S0173
stage: implement
created: "2026-08-17T13:16:25.568Z"
updated: "2026-08-17T13:16:25.568Z"
---

# Acceptance ledger — Re-run spec-trace post-merge and state clause-id completeness

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Five notes for the implementer, every one drawn from `spec.md` rather than added here:

- **`mount_point` is identical on every row, and that is the point.** This story ships no `pub`
  item and renders no surface (`_design.md:45-47`); its mount is *this file* citing the companion
  record with a real `file:line` per row. A completeness statement written into the story folder and
  referenced by nothing is this project's analogue of a component rendered into no tree
  (`_decomposition.md:429-436`), so an `evidence` value that names only a filename is not evidence.
- **`verifying_test` values are real commands and named reviewer reads, not Rust tests.** This story
  compiles nothing, and AC-TB-08 (`_decomposition.md:846-851`) forbids implying new Rust coverage.
  Every command below resolves to `.redkiln/config.yaml` or `xtask/src/main.rs` (AC-TB-02); a
  reviewer read cites the checklist it was run against — IQ-1…IQ-8 and AC-UX-01…AC-UX-12, never a
  second checklist invented at implement time (AC-TB-06).
- **`_clause-completeness.md` is a working name.** The file name is yours; the directory is not, and
  neither is the obligation that every row's evidence lands *inside* it at a cited line.
- **AC-005 and AC-006 are the composition and mount criteria.** A record that satisfies AC-001
  through AC-004 while being a pasted terminal transcript with tick-marks for status, or one that no
  row of this ledger cites by line, satisfies every functional assertion and fails this story.
- **Evidence for AC-001 must include the sha and the transcript.** "Green" is the summary the record
  exists to replace; cite the line the transcript starts on and the line the sha appears on.

```yaml
- id: AC-001
  criterion: "**GIVEN** a closeout reviewer (U2) handed a merge commit whose sibling diverges from `spec/SPECIFICATION.md` by 521 lines (`_decomposition.md:89-103`), who must decide whether the specification's cross-references still resolve *on that tree* rather than on the one planning ran in, **WHEN** they open `_clause-completeness.md`, **THEN** its first screen names the merge commit by **sha** and names the checkout it was observed in, and carries the outcome of `cargo xtask spec-trace` on that tree **verbatim** — the command line, the tree, and what it printed — never the word \"green\" standing in for a transcript. **AND** the record states in its own words what a passing run does *not* prove — that it is a document-internal cross-reference check, not a judgement that a doc comment discharges its obligation well — read off `xtask/src/spec_trace.rs`'s module documentation rather than asserted."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/post-merge-clause-completeness/_ledger.md — this row, citing `_clause-completeness.md` at the line carrying the sha and the line the verbatim transcript starts on"
  verifying_test: "`cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:48`) on the merged tree, exit zero, transcript pasted into `.bklg/docs-that-teach/durable-audience-closeout/post-merge-clause-completeness/_clause-completeness.md`; plus `rg -n \"<sha>\"` over that directory and `git cat-file -t <sha>` resolving to a commit; plus a reviewer read of the limits paragraph against `xtask/src/spec_trace.rs`'s module docs"

- id: AC-002
  criterion: "**GIVEN** a reviewer who must trust that \"HS-P0020's pinned set\" is a real object in the tree and not a phrase inherited from a plan — the planned landing site `xtask/src/lint_narrative.rs` does not exist in this worktree today — **WHEN** they follow the record's citation, **THEN** it lands on the **one** enumeration (`checked-documentation-surface/project.md:223-226`) at the path and line it actually occupies on the merged tree, and the record states the search that found it so a later reader can repeat the location step without knowing HS-P0020's internals; **and if it is absent**, that is recorded as a named finding, with the candidate set derived against the stated derivation rule **for the record only**. **AND** the diff introduces no second enumeration of the set — no regex, no prefix list, no hand-copied id table presented as authoritative — because a fourth list of the clause families is the one that can half-land (`frozen-documentation-must-pin/spec.md` AC-002)."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/post-merge-clause-completeness/_ledger.md — this row, citing `_clause-completeness.md` at the line carrying the located enumeration's path:line and the recorded search"
  verifying_test: "`test -f` on the enumeration path the record cites (merged tree) and `rg -n \"lint_narrative\" xtask/` reproducing the location step; plus a reviewer read confirming every clause id in the record's table names its source, and that the diff contains no clause-id regex or prefix array"

- id: AC-003
  criterion: "**GIVEN** U2, who must be able to tell *the document grew* from *the enumeration went stale* — two different bugs with two different owners — **WHEN** they read the set difference, **THEN** they meet **two separate sentences**, each naming the clause ids on its side: candidates the enumeration does not classify, and enumerated entries that are no longer candidates. Never \"the sets disagree\", never a bare count. **AND** when nothing was added — the expected outcome — the record still shows **both** directions computed and empty, with the enumeration's entries listed as still present and still discharged, because an absent section reads identically to a check nobody ran."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/post-merge-clause-completeness/_ledger.md — this row, citing the two direction headings in `_clause-completeness.md` by line"
  verifying_test: "Reviewer content review against `.bklg/docs-that-teach/checked-documentation-surface/frozen-documentation-must-pin/spec.md` (AC-005, EC-007, EC-008) and `standards/rust/81-checks-that-cannot-be-types.md:335-341`; plus `rg -ni \"sets disagree|counts disagree\"` over the story directory returning nothing and `rg -n \"^#+ \"` showing both direction headings present whether or not either is populated"

- id: AC-004
  criterion: "**GIVEN** the residual risk handed forward to this project **by name** at HS-P0020's own boundary (`checked-documentation-surface/project.md:273-279`), **WHEN** the reviewer reads any difference the statement found, **THEN** every one carries a disposition that is exactly one of two words with an owner and a destination attached: **pinned** — handed back to the single enumeration as HS-P0020's act, with the classifying change identified — or **routed** — an item under the `support` initiative (`.redkiln/config.yaml:5`), named together with the reason it could not be pinned inside this closeout. \"Noted for a future pass\" is not a disposition and appears nowhere. **AND** the record states the gate consequence a later reader would otherwise meet as a red build: while the pin's derived candidate scan is live an unclassified candidate is a **gate failure** (`frozen-documentation-must-pin/spec.md` EC-007), so an addition must be classified before `terminal-gate-run` takes `cargo xtask ci` (`.redkiln/config.yaml:60`; `_storymap.md:156-160`) — the routed arm covers the follow-up work, never the classification itself."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/post-merge-clause-completeness/_ledger.md — this row, citing the disposition column (or, in the empty case, the stated gate-consequence paragraph) in `_clause-completeness.md` by line"
  verifying_test: "Reviewer read of every difference row's disposition cell for a named owner and destination, against `.bklg/docs-that-teach/durable-audience-closeout/project.md:102-105` and `:202`; plus `rg -ni \"noted for|future pass|TBD|to be decided\"` over the story directory returning nothing in a disposition cell"

- id: AC-005
  criterion: "**GIVEN** U2 reading one row at a time inside a pull-request diff, in a corpus whose only automated reader is `redkiln validate --kb`'s frontmatter check, **WHEN** they read any single row of the record out of context, **THEN** it names its clause id, its side of the difference, its disposition, its owner or destination and its evidence, and loses nothing (AC-UX-10, `_decomposition.md:270-273`); **AND** every state anywhere in this story's diff is a literal **word** — no ✅/❌, no colour, no glyph, no strikethrough, no ordering and no empty cell carries meaning (AC-UX-09, `:266-269`, `:120-130`), the shape `.kb/maps/open-questions-index.md:136-143` already fixes; **AND** the load-bearing verdict is stated as a **sentence** outside the table as well as inside it (`_decomposition.md:142-145`); **AND** the record is *composed* from the corpus's own primitives rather than dumped — one `#`, sections at `##` with no skipped level, every link naming its destination (WCAG 2.4.4, `:132-136`), one table of at most five columns with no nested table, and prose wrapped to the ≤ 97-column width this spec's own non-table body already uses."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/post-merge-clause-completeness/_ledger.md — this row, citing the verdict sentence's line and the difference table's header line in `_clause-completeness.md`"
  verifying_test: "One reviewer checklist pass against `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md:115-145` (accessibility floor) and AC-UX-09 / AC-UX-10 at `:266-273`; plus `rg -n \"✅|❌|🟢|~~\"` and `rg -ni \"\\[here\\]|\\[this\\]|see above\"` over the story directory returning nothing, and `rg -n \"^#+ \"` showing one `#` and no skipped level"

- id: AC-006
  criterion: "**GIVEN** the downstream reader this milestone exists to serve — `dod-scenario-ledger`, held `blocked_by` this story precisely so its **DoD-11** row can cite scenario 11's fresh observation (`_storymap.md:56`; `initiative.md:452-454`) — **WHEN** that ledger's single row is written, **THEN** it can cite this statement by path and `file:line` **without paraphrasing it**, because the statement is reachable from this story's `_ledger.md` (mandatory under `require_ledger: true`, `.redkiln/config.yaml:67`) with a real `file:line` per criterion rather than a bare filename. **AND** nothing this story writes moves an anchor someone else already cited: the diff adds files under `.bklg/docs-that-teach/durable-audience-closeout/post-merge-clause-completeness/**` and reflows no pre-existing section (IQ-3, `_decomposition.md:180-187`)."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/post-merge-clause-completeness/_ledger.md — every row above, each resolving to a line inside `_clause-completeness.md`; this row records the walk itself"
  verifying_test: "Mount-point walk: open this ledger, follow each row's `evidence` `file:line`, land on the claimed sentence in `_clause-completeness.md`; plus `git diff --name-only` listing no path outside `.bklg/docs-that-teach/durable-audience-closeout/post-merge-clause-completeness/**`, `git diff` showing no `-` line in any pre-existing file, and `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) green"
```
