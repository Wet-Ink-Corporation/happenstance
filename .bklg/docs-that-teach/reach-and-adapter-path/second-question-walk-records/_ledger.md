---
item: HS-S0161
stage: implement
created: "2026-08-17"
updated: "2026-08-17"
---

# Acceptance ledger — The two second questions, walked and recorded

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes specific to this story, because its evidence is not a test run.

**Six of these nine rows are carried in part by a recorded human read (`T6` in the spec's
`## Tests and CI`).** That is not a shortcut: `_design.md`'s `## Density budget`, named gap 2, states
that nothing in `cargo xtask ci` verifies keyboard-only reachability or self-describing link text.
Evidence for those rows is a `file:line` into the walk record plus the named read, never a green tick
borrowed from a command that did not check the thing.

**A recorded *failed* walk satisfies its rows.** This story's deliverable is an honest observation,
not a successful one. AC-003 and AC-005 are satisfied by a dated entry that closes as a routed
failure just as much as by one that closes as an arrival. What does **not** satisfy them is a
*blocked* walk (`_design.md` `## States`, **Empty**) — if the links were never installed, the story
halts and no row is flipped.

```yaml
- id: AC-001
  criterion: "GIVEN an evaluator whose two plausible second questions were named by somebody else before anything was authored (DR-4), and no evaluator has ever been observed, WHEN this story walks, THEN exactly two files exist — one per question, each carrying the ISO date of the *walk* and never of the commit — and each opens by quoting its question as `evaluator-onward-links` named it (Q-A: \"Is 'dynamic' just a polite word for unstructured — where is the structure?\"; Q-B: \"Is DCB a modelling technique or a routing technique?\") and citing that story's named-questions record; no question is re-derived, re-worded, substituted, merged with the other, or replaced by the third stall point."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/reach-and-adapter-path/project.md — `## Companions` body prose, linking walk-record-q-a.md and walk-record-q-b.md"
  verifying_test: "T3 `git diff --name-only main...HEAD` against this spec's `## PR boundary` fence (via `redkiln verify --grain story`), plus `rg -n \"Is 'dynamic' just a polite word|modelling technique or a routing technique\" .bklg/docs-that-teach/reach-and-adapter-path/second-question-walk-records/` returning one hit per record and `rg -n \"_named-questions.md\"` returning a citation in each"
- id: AC-002
  criterion: "GIVEN Persona 3's whole journey happens inside one reading session with no second attempt, and the failure under test is *a good page with no exit* rather than a blank page, WHEN the walker begins, THEN each record states — before its first hop — what they had at t=0 (what `cargo add happenstance` shows, and the guide reached from it), how they arrived at the origin page as a reader continuing from the front door rather than by being handed a URL, what was forbidden to them (this spec, the sibling story specs, `_design.md`, the pointer register, the backlog, and asking either author), and the question they actually formed at the origin page in their own words; and if the walker also performed the `front-door-walk-record` walk or otherwise saw the guide first, the record cites that record and states in one sentence what the prefix taught them."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/reach-and-adapter-path/project.md — `## Companions` body prose, linking walk-record-q-a.md and walk-record-q-b.md"
  verifying_test: "T6 recorded human read of both records against `.bklg/docs-that-teach/reach-and-adapter-path/_decomposition.md` (UX brief, Journey A states A0→A1) and `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` (Cross-persona tensions), plus `rg -n \"front-door-walk-record/walk-record.md\"` returning a prefix disclosure — or an explicit no-prefix statement — in each record"
- id: AC-003
  criterion: "GIVEN an evaluator who has just had question one answered well and has formed a sharper one, WHEN they follow the in-passage link `evaluator-onward-links` installed, THEN each record's hop table shows exactly one link traversal from the origin page to the answering passage — no index page, no \"start here\" detour, no second hop — the record names the heading fragment it landed on, and it quotes the sentence in that passage that answers the question. An arrival the record can only describe as \"the bridge page\" is not arrival and closes as a failed walk under AC-006."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/reach-and-adapter-path/project.md — `## Companions` body prose, linking walk-record-q-a.md and walk-record-q-b.md"
  verifying_test: "T7 structural check — exactly one traversal row per dated entry — plus `rg -n \"Tag, query, fold, guard\"` and `rg -n \"Where your streams went\"` locating the landed heading in the respective record and in the registered destination file, the quoted answering sentence located verbatim in that file, and `cargo xtask narrative` green over the registered tree once HS-P0020's step has landed"
- id: AC-004
  criterion: "GIVEN the accessibility floor converts keyboard-only reachability from a claim into an artefact the project already owes, and project AC-005 bounds the walk to *surfaces a Rust developer already reads*, WHEN either walk is performed, THEN each record carries one explicit statement that no pointing device was used and names the affordance and keys per hop (Tab/Shift-Tab, Enter, rustdoc search `S` or `/`, the destination's own heading fragment), with no empty cell in the hop table; and each record lists every surface touched, all of which are the rendered pages of HS-P0020's pinned tree, the crate's rustdoc, or its README — a hop through the backlog, `_design.md`, the pointer register, a repository-wide `grep`, or a third-party site is recorded as such and fails the walk rather than completing it."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/reach-and-adapter-path/project.md — `## Companions` body prose, linking walk-record-q-a.md and walk-record-q-b.md"
  verifying_test: "T7 structural check (no empty hop-table affordance cell) plus `rg -n \"keyboard-only\"` returning the explicit statement in each record, and T6 recorded read of the surfaces-touched list against `.bklg/docs-that-teach/reach-and-adapter-path/_decomposition.md` (Accessibility floor, keyboard-only clause) and project AC-005's surfaces clause"
- id: AC-005
  criterion: "GIVEN UX invariant 4 is falsified by \"arriving at any destination in an AC-005 or AC-009 walk and being unable to name the page that should have preceded it\" and invariant 5 by a terminal page that neither answers nor names a next hop, WHEN the walker lands, THEN each record answers both, in words, at the landing: whether the destination states what it assumes the reader has already read — i.e. whether the walker could retrace without browser history and could tell they had arrived out of order — and whether the terminal passage answers the named question or names the next hop. A \"no\" to either is written as a finding against the destination's owner, never left as silence."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/reach-and-adapter-path/project.md — `## Companions` body prose, linking walk-record-q-a.md and walk-record-q-b.md"
  verifying_test: "T6 recorded human read of each record's landing section against `.bklg/docs-that-teach/reach-and-adapter-path/_decomposition.md` interaction-quality invariants 4 and 5 and `.bklg/docs-that-teach/reach-and-adapter-path/_design.md` `## States`, confirming a prose answer to each — a record carrying only \"arrived successfully\" fails"
- id: AC-006
  criterion: "GIVEN a record that cannot fail is decorative, and `_design.md` separates a Refused walk (a dead end that was found) from an Empty one (a destination that does not exist, so there is nothing to walk), WHEN a walk does not reach its answer, THEN the entry closes as a failed walk, names the owning story slug it is routed to (`evaluator-onward-links`, `front-door-pointer`, HS-P0022 or HS-P0020), repairs nothing in this PR, and any later attempt after somebody else's fix is a new dated entry citing the failed one — never an edit of a closed entry; and where the link was legitimately not installed, the story records a block against the dependency slug and stops, rather than simulating a destination."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/reach-and-adapter-path/project.md — `## Companions` body prose, linking walk-record-q-a.md and walk-record-q-b.md"
  verifying_test: "T8 `git log -p -- .bklg/docs-that-teach/reach-and-adapter-path/second-question-walk-records/walk-record-q-*.md` showing no commit rewrites a previously closed dated entry, plus T3 boundary check proving nothing under `crates/`, `docs/`, `xtask/`, `standards/`, `spec/` or `.kb/` changed, and T1 `cargo xtask ci --fast` with T2 `cargo xtask spec-trace` green"
- id: AC-007
  criterion: "GIVEN everyone available to this project is an insider and BR-14 reserves the non-insider claim to HS-P0024 alone, WHEN each record is written, THEN it names the walker, states in one sentence that they authored neither `evaluator-onward-links` nor `front-door-pointer` and what they knew of the surface before starting, and carries an explicit bounded-claim line: this is evidence the path exists for a non-author, not that a stranger finds it; and these are two plausible second questions, not the evaluator's observed one. No sentence anywhere in this PR implies otherwise."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/reach-and-adapter-path/project.md — `## Companions` body prose, linking walk-record-q-a.md and walk-record-q-b.md; project Definition-of-done item 4 resolves for its AC-005 walk"
  verifying_test: "`rg -n \"authored neither\"` and the bounded-claim sentence present in each record; `rg -n \"the evaluator's real second question|a stranger|newcomer|proves that anyone\"` over the whole diff returning nothing that overclaims; T6 recorded read against `.bklg/docs-that-teach/reach-and-adapter-path/project.md` (the two AC-005 risk rows, DR-10, DoD item 4), `_storymap.md` standing constraints and `.bklg/docs-that-teach/initiative.md` BR-14"
- id: AC-008
  criterion: "GIVEN two records sitting unlinked in a story folder are this medium's equivalent of a component that compiles and is never rendered, WHEN the PR lands, THEN `project.md`'s `## Companions` body prose carries one bullet per record, each with self-describing link text naming the record and the question it walked (never \"here\", \"this\", \"see this page\", \"docs\", \"read more\", or a bare URL into this repository's tree), both relative links resolving to files that exist, project Definition-of-done item 4 thereby resolving for its AC-005 walk — and the item's YAML frontmatter untouched, because the `redkiln` CLI is its only writer."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/reach-and-adapter-path/project.md — `## Companions` body prose, linking walk-record-q-a.md and walk-record-q-b.md"
  verifying_test: "T9 `redkiln validate --kb && redkiln doctor` green with `git diff` over `project.md` showing changes only beneath the closing `---`, plus T5 `rg -n \"\\[here\\]|\\[this\\]|\\[docs\\]|\\[read more\\]|\\[see this page\\]|https?://\"` over the added bullets returning nothing and T7 confirming each bullet's link target exists on disk"
- id: AC-009
  criterion: "GIVEN every mechanical assertion above passes perfectly on one unpunctuated paragraph containing the right strings, WHEN a reviewer who cannot run `cargo` opens either record, THEN it is a composed artefact: prose plus one hop table per dated entry that renders as a real markdown table with no empty cells, the fixed section order of a dated entry held (cold start · formed question · hop table · arrival · reversibility · dead end · keyboard statement · walker and relationship · bounded claim), heading ladder unskipped, nothing folded or tabbed, no raw HTML and no inline `style=`, no bare URL into this repository's own tree, and no table of fewer than three rows used as a navigation device."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/reach-and-adapter-path/project.md — `## Companions` body prose, linking walk-record-q-a.md and walk-record-q-b.md"
  verifying_test: "T4 `rg -n \"<details>|<summary>|<div|<br|style=\"` over both records and the `project.md` diff returning nothing, plus T7 structural check (required sections present in the fixed order, every hop-table row full, heading levels never skipped) and T6 recorded read against `.bklg/docs-that-teach/reach-and-adapter-path/_design.md` anti-patterns 3, 5, 6, 9, 10 and `_storymap.md` standing constraints"
```
