---
item: HS-S0142
stage: spec
created: 2026-08-17T13:16:04.504Z
updated: 2026-08-17T13:16:04.504Z
template_sig: 87bbf1d0
rendered_sig: 9f966054
---

# Spec — A clause id a page cites either resolves or fails the gate

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` — BR-09 (`:344`), DoD scenario 12 (`:455-456`) |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` — the DAG and the sibling-branch gate decision |
| Project charter | `.bklg/docs-that-teach/checked-documentation-surface/project.md` — AC-007, DR-07 |
| This spec | `.bklg/docs-that-teach/checked-documentation-surface/narrative-citation-resolution/spec.md` |
| Key brief — architecture | `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` Note 6 (`:250-274`), Note 4 (`:202-219`), Note 8 (`:311-326`) |
| Key brief — testing | `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` AC-007 bullet (`:617-628`) and "Fixtures and seams to mock" (`:704-720`) |
| Key brief — ux | `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` "Reader states" (`:473`) and "Falsifiers" (`:485`) |
| Signed-off design (**binding**) | `.bklg/docs-that-teach/checked-documentation-surface/_design.md` — `## Composition` (the worked failure report, `:327-352`), `## Transience policy` (clause citations are persistent chrome, inline, `:363`), `## The states the API must express` (`:555-569`), `## Hierarchy`, `## Density budget`, `## Anti-patterns` 7/8/9 |
| Story map / merge order | `.bklg/docs-that-teach/checked-documentation-surface/_storymap.md` — `specification-pin` row (`:53`), merge order 3.7 (`:157-164`) |
| Depends on (specs, already authored) | `../spec-trace-clause-id-accessor/spec.md` (the resolver), `../narrative-checker-mounted-with-pinned-path/spec.md` (the module, the step, the mount) |

## One-line PR slice

Parse the clause ids a narrative page cites with the hard-error-on-unreadable posture, resolve
each through `clause_ids`, and report every dangling id as a problem naming the page — a page
citing a real id passes, one citing a nonexistent id fails.

## Executive summary

Two things already exist when this PR opens, and neither of them reads a citation. Milestone 2
landed `xtask/src/lint_narrative.rs`: a bin-crate checker with the pinned `TREE`, the vacuity
guard, the bidirectional registration check, one accumulating `Vec<String>` and one `bail!`
carrying the count, mounted as the `REQUIRED` step `every narrative page is checked`. Milestone
3's foundation landed `pub(crate) fn clause_ids(root: &Path) -> Result<BTreeSet<String>>` beside
`all_rules` in `xtask/src/spec_trace.rs`, carrying an `#[expect(dead_code, reason = …)]` that
names this story.

This PR is the join: one check function inside the existing module, one call to `clause_ids` per
gate run, and the deletion of that `expect` — which is not housekeeping but the foundation's
designed hand-off, because the moment a consumer calls the function the expectation is
unfulfilled and `-D warnings` turns that into a failure that forces the attribute out.

The delta is one fact the compiler cannot state and no existing step checks: **every clause id a
page cites is a clause the specification actually declares.** Today `docs/` contains prose no
step reads at all, and the fixture page's own `(ES-40)` is a claim about
`spec/SPECIFICATION.md` that nothing verifies. After this PR a page citing `ES-40` passes, a
page citing `ES-99` fails naming the page, the line and the id, and a token that *looks* like a
citation and cannot be read is a problem rather than a silence.

## Context pack

Everything in this section is a decision already taken — by an authored sibling spec, by the
signed-off `_design.md`, or by this spec. Honor it; do not re-decide it.

**Resolution goes through the parser, and the alternative is the defect this story exists to
avoid.** Call `crate::spec_trace::clause_ids(&root)`. Do not write a regex over
`spec/SPECIFICATION.md`, and do not write a literal list of the six clause-family prefixes: that
list would be the **fourth**, and `SECTIONS`' own doc comment says why the fourth loses — it is
"the single place the six clause families are enumerated: the parser takes its accepted prefixes
from here, the census takes its rows, and §7.2 takes its subsection headings. Three lists that
must agree, kept as one so that adding a family cannot half-land"
(`xtask/src/spec_trace.rs:107-120`, `:122-153`; architecture brief Note 6). `clause_ids` is
called **once per run**, before the page loop, and the resulting set is passed to this check and
to `frozen-documentation-must-pin`'s — a contract the foundation spec states on its consumers,
because two calls parse a 9,000-line document twice inside one step.

**Which tokens count as citations is decided here, by shape and by two boundaries, and the
boundaries are the load-bearing half.** A candidate citation is two ASCII uppercase letters, a
hyphen, then one or more ASCII digits, with the preceding character not ASCII-alphanumeric and
the following character neither a digit nor a hyphen nor alphanumeric. That is not fussiness: the
trailing-hyphen boundary is what keeps `RS-81-1` — this repository's constitution rule ids, which
a page may legitimately cite — from being reported as a dangling clause, and the leading boundary
is what keeps `ADR-0001` from being read as `DR-0001`. `HS-S0142` fails the shape outright
because no digit follows the hyphen. A checker written without either boundary is a named wrong
implementation, and it fails on prose that is correct.

**The families come from the resolved set, not from a constant.** Derive them from
`clause_ids`'s return value — the prefix of each id up to its digits — so the recognised families
are exactly the families the document declares, and adding a seventh family to `SECTIONS`
needs no second edit here. Eighty-eight `####` clause declarations exist across the six families
today, so the derivation is not theoretical.

**Never skip what looks like a citation.** `spec_trace::citations` silently skips a span it does
not recognise, and that is safe *there* (`xtask/src/spec_trace.rs:2215-2260`).
`lint_constitution` takes the opposite posture and explains it in the module docs the architecture
brief tells this story to copy verbatim: "the span *is* the check, so a citation this parser
declines to read is a citation nothing verifies — the failure mode the step exists to prevent"
(`xtask/src/lint_constitution.rs:30-44`; the problem it pushes is at `:688-693`, and its test is
`a_citation_with_a_nested_backtick_does_not_parse` at `:833-841`). Two consequences, and both are
problems rather than skips: a citation-shaped token whose family the specification declares
nowhere is reported with the declared families named, and a near-miss inside a declared family —
`ES-` with no digits, `ES-4O` with a letter for a zero — is reported as a citation the checker
cannot read.

**The whole page is scanned, fences included.** Excluding fenced blocks would put a hiding place
for unresolvable claims in exactly the region that is the checked artifact, and a clause id in a
comment above an example is still a claim. The cost is stated rather than hidden: a
clause-shaped token that is genuinely *data* inside an example is reported, and the answer is to
spell it outside the fence — not an allowance list. If that case ever recurs it is petitioned in
the shape of `IGNORE_ALLOWANCES`, in its own change, with its own falsification
(`_design.md`, `## Open questions` 3).

**Citations are inline, and the design already settled that they must be.** Clause ids sit
"inline in the sentence that depends on them — not collected in a footer", and they are
persistent chrome deliberately *not* revealed on demand, "because AC-007 exists because
provenance is the thing that rots" (`_design.md`, `## Composition` region 3, `## Transience
policy`). So the check reads every line of prose, in source order, and needs no citation section
to find.

**The output shape is signed off and is not this story's to invent.** One more problem line on
the surface milestone 2 created: two-space indent, `{path}:{line} — {message}`, in source order,
count last, never truncated, and the failure report `_design.md` draws includes this story's own
line verbatim — `` docs/append-conditions.md:12 — cites `ES-99`, which SPECIFICATION.md does not
define `` (`_design.md:327-339`). Accumulate onto the module's existing `Vec<String>`; a page
citing five dangling ids reports five, because "a check that stops at the first problem turns one
review cycle into six" (`_decomposition.md:218-219`). Success stays exactly one line — this story
adds no second summary line and no per-page progress line.

**No new step, no new subcommand, no `main.rs` edit.** The step, the dispatch arm, the help line,
the `lint_steps` membership and the `affected::run` call all exist from
`narrative-checker-mounted-with-pinned-path`. This story adds a `check_citations`-shaped function
and one call site in that module's `run()`, mirroring `lint_constitution::run`'s per-atom
`check_citations(&root, atom, &mut problems)` (`xtask/src/lint_constitution.rs:184-190`). A second
step would split one tree's failures across two banners for no reason and would contradict
`_design.md`'s two-steps-two-banners decision, which allots exactly one banner to the checker.

**The foundation's dead-code marker is deleted in this PR, and that is the designed hand-off.**
`#[expect(dead_code, reason = …)]` on `clause_ids` names `narrative-citation-resolution` and
`frozen-documentation-must-pin`. Being the first consumer, this PR makes the expectation
unfulfilled — itself a warning, which `-D warnings` makes a failure — so the attribute must go
in the same change. `#[allow(dead_code)]` must not appear anywhere in the diff; the whole point of
`expect` here is that it cannot rot into a permanent exemption
(`../spec-trace-clause-id-accessor/spec.md`, AC-006; the in-repo precedent is
`crates/happenstance-neon/src/event_store.rs:232-240`).

**Hard errors belong to the artifact that broke, and a page is never blamed for the
specification.** An unreadable or declares-nothing `spec/SPECIFICATION.md` is `clause_ids`'s
error, propagated unchanged so the message names the document and says the checker is broken
rather than the pages (`../spec-trace-clause-id-accessor/spec.md`, AC-004). An unreadable page is
a hard error naming the page, never a skipped page: "a scanner that silently skips what it cannot
read reports green over exactly the file it failed to inspect"
(`standards/rust/81-checks-that-cannot-be-types.md:95`, RS-81-2).

**The persona slice, and the falsifier this closes.** The reader is the **contributor running the
gate** and the **reviewer reading its output** (`_storymap.md` preamble), acting for the three
personas the initiative carries: the application author who cites a clause on a teaching page, the
adapter author who meets the frozen MUSTs at their discharge sites, and the evaluator who has
"nowhere for that question to go" and follows a citation as a promise that the sentence above it is
anchored in something real (`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:56`,
`:148`, `:225`, `:269-272`). The reader state made true is *"Following a clause citation — land on
the clause, not on a paraphrase of it"*, whose "true today" column reads *"No citation-resolution
check exists for narrative pages"* (`_decomposition.md:473`). The falsifier closed is exact: *"A
page cites a clause id that does not exist, and the gate passes"* (`_decomposition.md:485`).

**What this story must not become.** It resolves citations; it does not judge them. Whether a page
*defers* to a clause rather than restating it is BR-09's reviewer half and HS-P0021's
(`project.md`, "Out of scope"). Nothing here amends, discharges or restates a clause: AC-007 and
AC-008 *read* the document and `cargo xtask spec-trace` remains the only writer of §7.1/§7.2, and
only under `--write` (architecture brief Note 8). No `[FROZEN]` clause is touched, so no ADR is
owed — the grounding pass found no Accepted decision atom governing gate structure, documentation
trees or fence compiling (`_storymap.md:94-99`). And nothing in the module, its output or its docs
may claim the surface proves a page teaches: a resolving citation is provenance, not comprehension
(project DoD 8; `_design.md` anti-pattern 9).

**Two standing constraints, restated because they bound the implementation shape.** No new
dependency — the check is `str` scanning over `std` plus the `anyhow` already in `xtask`
(`xtask/Cargo.toml:16-21`, DR-12), so no regex crate and no markdown parser. And
`xtask/src/lint_constitution.rs`, `xtask/src/constitution.rs` and `spec_trace`'s own `run` stay
unrefactored: copy the shape, do not abstract over it, because a shared abstraction over two trees
makes one error message answer two questions (RS-81-3; architecture brief Note 8).

## Integration contract

- **Archetype**: `capability` — observable through `cargo xtask narrative`, `cargo xtask ci`,
  `cargo xtask ci --fast` and `cargo xtask affected --base main`.
- **Slice / milestone**: `specification-pin`. Slice-mates, implemented in the same context and
  mounted as one surface: `spec-trace-clause-id-accessor` (the foundation, landing first and
  before both consumers) and `frozen-documentation-must-pin` (the second consumer of the same
  resolved set). This story is second of the three in merge order (`_storymap.md:157-164`).
- **Mount point**: **`xtask/src/lint_narrative.rs`** — the checker module's own composition root:
  one `clause_ids` call in `run()` before the page loop, and one `check_citations(page, &ids,
  &mut problems)` call site in the same list the fence walk already occupies, mirroring
  `lint_constitution::run`'s per-atom check list (`xtask/src/lint_constitution.rs:180-190`).
  **`xtask/src/main.rs` is deliberately not touched**: the `Step`, the dispatch arm, the
  `print_help()` line and the `lint_steps()` membership were mounted by
  `narrative-checker-mounted-with-pinned-path`, and adding a second step for the same tree would
  contradict `_design.md`'s one-banner allotment for the checker.
- **Wires into**:
  - `crate::spec_trace::clause_ids` (`xtask/src/spec_trace.rs`, beside `all_rules` at `:1746`) —
    the resolver, called once per run; its `#[expect(dead_code, …)]` is deleted here.
  - `crate::spec_trace::workspace_root` (`xtask/src/spec_trace.rs:2311`) — already how the module
    resolves the repository root; no second root-finding path is written.
  - `xtask/src/lint_narrative.rs`'s `Page` enumeration, its accumulating `Vec<String>`, its
    `bail!("{n} problem(s) in {TREE}")` and its page text read — reused, not duplicated. The page
    text is read once and both the fence walk and this check consume it; a second read can see a
    different file mid-edit and doubles the I/O the `affected::run` argument rests on.
  - `xtask/src/lint_constitution.rs:30-44`, `:674-693`, `:795-826`, `:833-841` — the hard-error
    posture, the problem-line wording, the span/parse shape and the test template. Copied, not
    imported (RS-81-3).
  - `spec/SPECIFICATION.md` — read-only, through `clause_ids`, by clause id rather than by line
    number, which is what makes the check survive the 521-line divergence on the unmerged
    `initiative/from-contract-to-published-library` branch (`spec/SPECIFICATION.md:280`;
    `project.md:274-281`).
  - No workspace crate, no port, no `Send` bound, no feature, no `serde`: ADR-0001 and ADR-0003
    are untouched by construction (`_design.md`, `## What it costs a caller`).
- **Renders surfaces**: `gate-narrative-checker-step` (`_design.md`, `## Surfaces`) — this story
  adds the *unresolvable clause id* and *unreadable citation* problem lines to its `fail-one` and
  `fail-many` states, in the composed form `_design.md:327-352` draws, and leaves its `pass` state
  at exactly one summary line. `narrative-page` and `narrative-scoped-page` are **not** changed:
  the fixture page's `(ES-40)` citation is milestone 1's and already resolves, and no page content
  is authored here.
- **Public items** (`_design.md`, `## Items`): none added. This story is the first *caller* of
  `xtask::spec_trace::clause_ids` and adds one private check function to
  `xtask::lint_narrative`. `IGNORE_ALLOWANCES` and `HIDDEN_MARKERS` belong to milestone 2;
  `TREE` and `HARNESS` are already pinned.
- **Conformance rule(s)**: none, and this is not adapter-observable. Nothing here touches
  `crates/happenstance-testkit/`, a port, a value type or a fixture — the artifact read is
  `spec/SPECIFICATION.md` and the pages under `docs/`. Stated explicitly because a story that
  changes a port and names no rule is a port change nothing can fail; this changes no port.
- **Clause(s)**: none discharged, none amended, none restated. The check reads clause ids as
  names. No `[FROZEN]` clause is edited, so no ADR is owed.
- **Advances DoD scenario**: initiative DoD **12** — *"No page has become a second specification.
  Each normative claim a teaching page makes is a citation that resolves"* (`initiative.md:455-456`).
  This story turns the mechanical half of that scenario from assertable into checked; the spot
  check that a page *defers* rather than restates stays HS-P0021's, and this story must not be
  written up as evidence of it.

## PR boundary

```
xtask/src/lint_narrative.rs
xtask/src/spec_trace.rs
.bklg/docs-that-teach/checked-documentation-surface/narrative-citation-resolution/**
```

**In this PR**

- `xtask/src/lint_narrative.rs` — the citation check: the candidate-token scan with both
  boundaries, the family derivation from the resolved set, the three problem forms (dangling id,
  undeclared family, unreadable citation), the one `clause_ids` call in `run()`, and the limits
  this check adds to the module's existing `# What this does not verify` section.
- `xtask/src/lint_narrative.rs` `#[cfg(test)] mod tests` — new cases in the house shape
  (`xtask/src/lint_constitution.rs:827-841`), every one of them in-memory over `&str` page text
  and a hand-built `BTreeSet<String>`, because `xtask` has no `tempfile` dev-dependency and
  NF-scope forbids adding one.
- `xtask/src/spec_trace.rs` — **deletion only**: the `#[expect(dead_code, reason = …)]` on
  `clause_ids`, whose self-erasure this PR triggers. No other line in that file changes; the
  parser, `run`, and the generated §7.1–§7.2 region are untouched.
- This story's own backlog folder: the `_ledger.md` and the implementation report.

**Explicitly not in this PR**

- Any new `Step`, subcommand, `print_help()` line, `lint_steps()` entry or `affected::run` call —
  all five are `narrative-checker-mounted-with-pinned-path`'s and already landed. `xtask/src/main.rs`
  is out of the boundary on purpose.
- The frozen-documentation-MUST pin, its discharge sites, its re-derivation and its
  derived-versus-hand-written count — `frozen-documentation-must-pin`, the same slice's second
  consumer of the same resolved set.
- The fence walk, `IGNORE_ALLOWANCES`, its reverse sweep and `HIDDEN_MARKERS` — milestone 2's,
  already landed; this check runs beside them, not through them.
- Any page content: no page is authored, no citation is added to a page, and no fixture page is
  edited. Every test in this story is in-memory. The corpus is HS-P0021/22/23's (`project.md`,
  risk table).
- Any judgement about whether a page defers to a clause rather than restating it (BR-09's
  reviewer half, HS-P0021), and any observed-failure transcript
  (`observed-failure-falsification`).
- Any edit to `spec/SPECIFICATION.md`, `standards/rust/`, `.kb/`, `docs/`, or `xtask/Cargo.toml`.
- Any refactor of `xtask/src/lint_constitution.rs`, `xtask/src/constitution.rs`, or `spec_trace`'s
  `run`.

**Merge DoD**: `cargo xtask ci --fast` is green (`.redkiln/config.yaml:55`), `cargo xtask
narrative` passes standalone over the tree as milestone 2 left it and still prints exactly one
summary line, `cargo xtask spec-trace` prints what it printed before this commit, `cargo test -p
xtask` covers the resolving, dangling, undeclared-family, unreadable-citation and
boundary-carve-out cases, and the diff contains no `allow(dead_code)` and no new manifest entry.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| -------------------- | ------- | ------------- |
| Resolution goes through the parser | `crate::spec_trace::clause_ids(&root)?` — no regex over the specification, no literal family list, no second parse. A fourth family list is a family that can half-land. | `xtask/src/spec_trace.rs:107-153`, `:1746`; `_decomposition.md:250-266` |
| One call per gate run | `clause_ids` is called once in `run()`, before the page loop, and the `BTreeSet<String>` is passed by reference to this check (and to the pin check when it lands). Two calls parse a 9,000-line document twice in one step. | `../spec-trace-clause-id-accessor/spec.md` ("One call per gate run in the consumer"); `xtask/src/lint_constitution.rs:180-190` |
| Families are derived, never listed | The recognised prefixes are the prefixes present in the resolved set (`ES-1` → `ES-`). Adding a seventh family to `SECTIONS` needs no edit here; 88 `####` declarations across six families exist today, so the derivation has real input. | `xtask/src/spec_trace.rs:122-153`, `:1391-1430` |
| A citation is recognised by shape, with two boundaries | Two ASCII uppercase letters, `-`, one or more ASCII digits; the preceding character is not ASCII-alphanumeric and the following character is not a digit, `-`, or alphanumeric. `ES-40` and `CF-6` are candidates; `RS-81-1`, `ADR-0001` and `HS-S0142` are not. | this spec, `## Context pack`; `_design.md:327-339` (the drawn failure line) |
| A resolving citation passes silently | An id in the set produces no problem and no output. Provenance is checked, not narrated; the success surface stays one line. | `_design.md`, `## Transience policy` (success summary), `## States` |
| A dangling id in a declared family is a problem | `` {page}:{line} — cites `ES-99`, which SPECIFICATION.md does not define `` — the design's own drawn line, naming page, line and id. | `_design.md:327-339`; `project.md` AC-007 |
| A citation-shaped token in an undeclared family is a problem, not a skip | Reported with the declared families named, so an author who meant a constitution rule spells it in full (`RS-00-1`) rather than guessing. `spec_trace::citations` skips what it does not recognise; that posture is wrong here, where the span *is* the check. | `xtask/src/lint_constitution.rs:30-44`; `xtask/src/spec_trace.rs:2215-2260` |
| A near-miss inside a declared family is a problem | `ES-`, `ES-4O`, `ES -40`: "looks like a clause citation and does not parse; a citation the checker cannot read is one nothing verifies" — the wording and the mechanism copied from the in-house parser that hard-errors on purpose. | `xtask/src/lint_constitution.rs:688-693`, `:795-826`, `:833-841` |
| The whole page is scanned, fences included | A clause id in a comment above an example is still a claim, and excluding fenced regions would put a hiding place inside the checked artifact. Recorded as a limit rather than defended with a parser: a clause-shaped token that is genuinely data inside an example is reported. | this spec, `## Context pack`; `_design.md`, `## Open questions` 3 |
| Every problem, in source order, count last | Problems push onto the module's existing `Vec<String>`; pages in enumeration order, lines ascending. Never fail fast, never truncate, no `… and N more`. | `xtask/src/lint_constitution.rs:184-199`; `_decomposition.md:218-219`; `_design.md` anti-patterns 7, 8 |
| Page text is read once | This check consumes the same page text the fence walk already read. A second read doubles the I/O the `affected::run` inclusion argument rests on and can see a different file mid-edit. | `xtask/src/affected.rs:28-36`; `_decomposition.md:202-219` |
| No new step and no `main.rs` edit | One check function and one call site inside `run()`. The banner stays `=== every narrative page is checked ===`; a second step would split one tree's failures across two banners. | `xtask/src/main.rs:816-826`; `_design.md`, `## Composition` ("Two steps, two banners") |
| The foundation's `expect` is deleted here | This PR is `clause_ids`' first caller, so `#[expect(dead_code, …)]` becomes unfulfilled — a warning, which `-D warnings` makes a failure. The attribute is removed in the same change and no `allow(dead_code)` replaces it. | `crates/happenstance-neon/src/event_store.rs:232-240`; `../spec-trace-clause-id-accessor/spec.md` AC-006 |
| An unreadable or empty specification blames the checker | `clause_ids`' error is propagated unchanged: it names `spec/SPECIFICATION.md` and says the checker is broken rather than the document. Never `Ok` with an empty set, which would report every real citation on every page as dangling. | `../spec-trace-clause-id-accessor/spec.md` AC-004; `xtask/src/spec_trace.rs:2307-2309` |
| An unreadable page is a hard error, not a skipped page | Named by path, with context. A scanner that silently skips what it cannot read reports green over exactly the file it failed to inspect. | `standards/rust/81-checks-that-cannot-be-types.md:95` (RS-81-2); `../narrative-checker-mounted-with-pinned-path/spec.md` EC-004 |
| Limits, in the module's own docs, before the guarantee | Added to the module's existing `# What this does not verify` section: resolution is not correctness — a resolving citation says nothing about whether the sentence above it is true, nor whether the page defers rather than restates (BR-09's reviewer half, HS-P0021's); a clause-shaped token inside a fence is not distinguished from prose; a doubly-declared id collapses upstream in `clause_ids`. | `xtask/src/lint_constitution.rs:9-28`; RS-81-1; project DoD 7; architecture brief Note 10 |
| No claim of teaching, anywhere | No badge, tick, shield or "verified" wording in code, docs or output. A resolving citation is provenance, not comprehension. | project DoD 8; `_design.md` anti-pattern 9 |
| Zero new dependencies | `str` scanning over `std` plus the `anyhow` already present. No regex crate, no markdown parser. | `xtask/Cargo.toml:16-21` (DR-12) |
| Tests are in-memory and cover only the new behaviour | Page text as `&str`, the id set hand-built, in the shape `xtask/src/lint_constitution.rs:827-841` uses. `clause_ids`' own parse is proven by its story and by §1.3's hand count that `run` checks every gate run; re-proving it here would be the parallel fixture corpus the testing brief forbids. | `_decomposition.md:617-628`, `:704-720`; `xtask/src/spec_trace.rs:39-57` |

## Data and migrations

**N/A.** No schema, no store, no migration, no serialised format and no persisted state. This
story reads two artifacts that already exist — the pages under `docs/` and, through
`clause_ids`, `spec/SPECIFICATION.md` — and writes lines to stdout and stderr. `xtask` carries
`publish = false`, so there is no released artifact and no semver promise
(`xtask/Cargo.toml:7`; `_design.md`, `## Visibility and stability`).

Three things behave like a contract-by-value even though none of them is data, and each is
recorded here rather than discovered later:

- **The citation token shape is a contract on page authors.** HS-P0021, HS-P0022 and HS-P0023
  write pages against it. Narrowing the shape later silently stops checking citations already
  written in the old spelling; widening it starts failing prose that was previously correct. It
  belongs in the module's docs beside the check, with the two boundaries and their reasons —
  `RS-81-1` and `ADR-0001` — spelled out, because those are the tokens a reviewer will otherwise
  read as bugs.
- **Clause ids are stable names, never line references.** That is what makes the check survive the
  521-line divergence between this branch and the unmerged
  `initiative/from-contract-to-published-library` (`spec/SPECIFICATION.md:280`;
  `project.md:274-281`). If the sibling branch *adds* clause families or clauses, this check
  simply resolves more ids — nothing here needs re-deriving, which is exactly why the pin story
  and not this one carries the re-derivation obligation.
- **`spec/SPECIFICATION.md` is read-only here.** No `--write`, no edit to the generated §7.1–§7.2
  region, so the committed-versus-computed equality `cargo xtask spec-trace` asserts is unaffected
  (`xtask/src/spec_trace.rs:31-38`).

## Acceptance criteria

The "user" is the **contributor running the gate** and the **reviewer reading its output**
(`_storymap.md:12-16`), acting for the three personas the initiative carries: the *application
author* who cites a clause on a teaching page, the *adapter author* who meets the frozen MUSTs at
their discharge sites, and the *evaluator* who follows a citation as a promise that the sentence
above it is anchored in something real and today has "nowhere for that question to go"
(`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:56`, `:148`, `:225`,
`:266-272`). Every criterion below is one of those people completing something end to end through
the real module and the real gate — not a capability the module happens to have.

The reader state this table makes true is *"Following a clause citation — land on the clause, not on
a paraphrase of it"*, whose "true today" column reads *"No citation-resolution check exists for
narrative pages"* (`_decomposition.md:473`). The falsifier it closes is *"A page cites a clause id
that does not exist, and the gate passes"* (`:485`).

| id | criterion | verification |
| -- | --------- | ------------ |
| AC-001 | **GIVEN** a maintainer who later adds a seventh clause family to `SECTIONS` — the one place the six families are enumerated, whose own doc comment says three lists must agree "so that adding a family cannot half-land" (`xtask/src/spec_trace.rs:107-120`) — **WHEN** they add it there and run the gate, **THEN** the narrative checker recognises citations in the new family with **no second edit**, because it resolves through `crate::spec_trace::clause_ids(&root)` and derives its recognised prefixes from that set's own ids rather than from a literal list that would be the fourth. `clause_ids` is called **once** in `run()`, before the page loop, and the `BTreeSet<String>` is passed by reference to the check. **AND** because this PR is `clause_ids`' first caller, the foundation's `#[expect(dead_code, reason = …)]` becomes unfulfilled — itself a warning, which `-D warnings` makes a failure — so it is deleted in the same change and `#[allow(dead_code)]` appears nowhere in the diff. | *Static.* `xtask/src/lint_narrative.rs` `mod tests`: `the_recognised_families_are_derived_from_the_resolved_set` — hand-build a set containing a fictional `ZZ-1`, scan a page citing `ZZ-9`, and assert the problem is the *dangling id* form and **not** the undeclared-family form; a checker holding a literal six-prefix list fails this test. *Lint:* `cargo xtask ci --fast` (clippy, `-D warnings`) is green, which is only possible once the `expect` is gone. *Reviewer check:* the diff introduces no clause-id regex, no prefix array, and exactly one `clause_ids(` call site. |
| AC-002 | **GIVEN** an application author who has written a teaching page and cited the clause the page's claim rests on — the fixture page's own `(ES-40)`, which is a claim about `spec/SPECIFICATION.md` that nothing verifies today — **WHEN** they run `cargo xtask narrative` or the gate, **THEN** the page passes with **no output about the citation at all**, and the step still prints exactly one summary line. Provenance is checked, not narrated: a check that reports each resolving citation makes the one line that matters harder to find (`_design.md`, `## Transience policy`). | *Static.* `mod tests`: `a_citation_naming_a_declared_clause_is_not_a_problem` — page text citing `ES-40` against a set containing it yields an empty problem list. *Gate-integration:* `cargo xtask narrative` over the tree as milestone 2 left it prints exactly one summary line under exactly one banner, with the fixture page's `(ES-40)` in it. |
| AC-003 | **GIVEN** an evaluator who follows a clause citation on a teaching page and lands on nothing — the provenance failure AC-007 exists because of, and one the reviewer half of BR-09 cannot catch at scale — **WHEN** the gate runs over a page citing an id `spec/SPECIFICATION.md` does not declare, **THEN** the run **fails**, naming the page, the line and the id, in the composed form the signed-off design already drew: `` docs/append-conditions.md:12 — cites `ES-99`, which SPECIFICATION.md does not define `` (`_design.md:327-339`). A message naming only the id, or only the page, does not satisfy this row: the location must be first so soft-wrap in an 80-column log cannot push it off the first visual row. | *Static.* `mod tests`: `a_dangling_id_names_the_page_the_line_and_the_id` — asserts the pushed string matches `  {page}:{line} — ` followed by the design's wording, with the id back-quoted, and that the location precedes the em dash. *Gate-integration:* the same page walked through `cargo xtask narrative` and observed non-zero with the count line last. |
| AC-004 | **GIVEN** a page author who wrote something that *looks* like a clause citation and is not one the checker can resolve — `ES-` with no digits, `ES-4O` with a letter for a zero, or a family (`XX-7`) the specification declares nowhere — **WHEN** the gate runs, **THEN** each is reported as a **problem**, never skipped, and the undeclared-family form names the families the specification does declare so the author can spell what they meant. `spec_trace::citations` silently skips a span it does not recognise, which is safe there (`xtask/src/spec_trace.rs:2215-2260`); the opposite posture is mandatory here, for `lint_constitution`'s stated reason — "the span *is* the check, so a citation this parser declines to read is a citation nothing verifies" (`xtask/src/lint_constitution.rs:30-44`, and the problem it pushes at `:688-693`). A checker that silently non-matches is the named wrong implementation this row rejects. | *Static.* `mod tests`, three cases in the shape of `a_citation_with_a_nested_backtick_does_not_parse` (`xtask/src/lint_constitution.rs:833-841`): `a_citation_shaped_token_in_an_undeclared_family_is_a_problem` — asserts the message lists the declared families; `a_near_miss_inside_a_declared_family_is_a_problem` — `ES-` and `ES-4O`, asserting the "cannot read" wording rather than the "does not define" wording, so the two forms stay distinguishable; and `a_skipped_near_miss_would_fail_this_test` is not a separate test but the assertion that the problem list is non-empty in both. |
| AC-005 | **GIVEN** a page author who legitimately cites this repository's own constitution rule ids and ADR numbers in prose — `RS-81-1`, `ADR-0001`, and a backlog item id such as `HS-S0142` — **WHEN** the gate runs, **THEN** none of them is reported, because the token shape carries **both** boundaries: the preceding character is not ASCII-alphanumeric (so `ADR-0001` is never read as `DR-0001`) and the following character is neither a digit, a hyphen, nor alphanumeric (so `RS-81-1` is not read as a dangling `RS-81`). `HS-S0142` fails the shape outright — no digit follows the hyphen. A checker missing either boundary fails on prose that is **correct**, which is worse than missing a defect: it teaches contributors to remove true sentences. | *Static.* `mod tests`: `a_constitution_rule_id_is_not_a_clause_citation` (`RS-81-1`, `RS-00-1`), `an_adr_reference_is_not_a_clause_citation` (`ADR-0001`, `ADR-0029`), `a_backlog_item_id_is_not_a_clause_citation` (`HS-S0142`, `HS-P0020`), each asserting an empty problem list; and `the_boundaries_are_both_load_bearing` — the same page with `ES-40` present asserts the check is not simply always-empty. |
| AC-006 | **GIVEN** a reviewer reading a failing CI log they cannot re-run, on a branch where five pages each cite a dangling id, **WHEN** the checker fails, **THEN** they get **every** problem in one run — pushed onto the module's existing `Vec<String>`, two-space indent, `{path}:{line} — {message}`, pages in enumeration order and lines ascending, count and directory last, never truncated and never `… and N more` — because "a check that stops at the first problem turns one review cycle into six" (`_decomposition.md:218-219`). **AND** the scan covers the whole page including fenced blocks — a clause id in a comment above an example is still a claim, and excluding fences would put a hiding place inside the checked artifact — consuming the page text the module already read once, not re-reading the file. | *Static.* `mod tests`: `five_dangling_ids_on_one_page_report_five_problems_in_source_order` — asserts length 5, ascending line numbers, and that no message is elided; `a_clause_id_inside_a_fence_is_still_checked` — a page whose only citation sits inside a ` ```rust ` block still reports. *Reviewer check:* the call site takes the already-read page text as an argument; no second `read_to_string` of a page appears in the diff. *Gate-integration:* the standalone output read against `_design.md` `## Composition`'s worked failure report. |
| AC-007 | **GIVEN** a contributor whose checkout has a truncated, moved or unreadable `spec/SPECIFICATION.md`, or a page that is not valid UTF-8, **WHEN** the gate runs, **THEN** the run fails with a message naming **the artifact that actually broke** — the specification, with the checker blamed rather than the pages; or the page, by path — and **never** reports every real citation on every page as dangling, and **never** skips the file it could not inspect. `clause_ids`' error is propagated unchanged (`../spec-trace-clause-id-accessor/spec.md` AC-004, EC-001/EC-002); an unreadable page is a hard error because "a scanner that silently skips what it cannot read reports green over exactly the file it failed to inspect" (`standards/rust/81-checks-that-cannot-be-types.md:95`, RS-81-2). A page blamed for the specification's condition does not satisfy this row. | *Static.* `mod tests`: `an_unreadable_specification_is_propagated_not_swallowed` — the `run`-shaped path against a root with no specification is `Err` and the `{:#}` chain contains `reading spec/SPECIFICATION.md`, with no page path in it; `an_unreadable_page_is_a_hard_error_naming_the_page` — asserts `Err` whose chain contains the page path, and asserts the problem list is *not* silently short by one. |
| AC-008 | **GIVEN** a contributor or a downstream project (HS-P0021/22/23) opening this check for the first time to decide how far to trust it, **WHEN** they read the module, **THEN** the *first* thing they meet is `# What this does not verify`, now carrying this check's own limits — that a resolving citation says nothing about whether the sentence above it is true, nor whether the page *defers* to the clause rather than restating it (BR-09's reviewer half, HS-P0021's); that a clause-shaped token inside a fence is not distinguished from prose; and that a doubly-declared id collapses upstream in `clause_ids` — **AND** nowhere in the module, its output, `docs/README.md` or this story's own artefacts does anything claim the surface proves a page *teaches*. A check whose limits are undocumented is read as a guarantee (`xtask/src/lint_constitution.rs:9-13`, RS-81-1), and a green step read as evidence of teachability is the initiative's top-ranked risk (`project.md:286`). | *Static.* `mod tests`: `the_citation_check_documents_its_limits_before_its_guarantee` — reads `xtask/src/lint_narrative.rs` as text through the module's own `read` helper, exactly as `check_harness` reads a source file (`xtask/src/lint_constitution.rs:424-425`), and asserts the `does not verify` line precedes the first line describing what the citation check *does*. *Gate:* `cargo xtask lint-constitution` and `cargo test --locked -p xtask --doc` still pass, so the docs compile rather than merely exist. *Reviewer check:* a grep of the diff for `verified`, `badge`, `✓`, `proves`, `teaches` returns nothing in a claiming sense (project DoD 8; `_design.md` anti-pattern 9). The six-item *contents* of the limits section remain `documented-blind-spots-and-their-proofs`'. |

**Project AC coverage.** Project **AC-007** — *"A narrative page citing a `SPECIFICATION.md`
clause id that does not exist fails the gate. A page citing one that does exist passes"*
(`project.md:221-222`) — is satisfied by AC-002 (the resolving half) and AC-003 (the dangling
half), with AC-001 making both resolve through the parser rather than through a list that can
half-land, AC-004 closing the silent-skip escape that would let a citation pass unchecked, AC-005
keeping the check from firing on correct prose, AC-006 making a multi-page failure reviewable in one
run, AC-007 keeping the blame on the artifact that broke, and AC-008 carrying project DoD items 7
and 8. DR-07's machine half (`project.md:168-171`) and initiative DoD scenario 12
(`initiative.md:455-456`) are advanced by the same rows; the reviewer half of BR-09 — whether a
page *defers* rather than restates — is HS-P0021's and is not claimed here.

## Interaction quality

This story renders no interactive surface: no screen, no viewport, no pointer, no editor state.
The **state** family is therefore answered N/A — but answered, one line each, because a reviewer
who cannot tell "inapplicable" from "forgotten" has to assume the worse of the two. The
**composition** family is *not* vacuous: this story adds lines to a composed surface a person reads
(`gate-narrative-checker-step`, `_design.md`, `## Surfaces`), and the signed-off design binds their
form, placement, transience, density and hierarchy. **Every invariant below is carried by an
`AC-###` row in the table above; none of them lives only here**, because `redkiln verify` extracts
criteria from table rows and a bullet in this section would never be gated.

**State invariants**

- *In-place vs context-jump* — N/A. There is no navigation. The nearest genuine obligation is that
  this check lands **in place**, inside the step milestone 2 mounted, rather than jumping the
  contributor to a second banner for the same tree: **AC-001** (one call site in the existing
  `run()`), and the PR boundary's exclusion of `xtask/src/main.rs`.
- *Non-occlusion* — the real analogue, and it is not N/A: a new problem line must not displace or
  hide an existing one. **AC-006** carries it — problems accumulate onto the module's existing
  `Vec<String>` and nothing is truncated, so a citation problem never occludes a fence problem.
- *Preserved focus / scroll / selection* — N/A at the surface. Its analogue is source order:
  **AC-006** requires the list to read in the same order as the tree the contributor is about to
  edit, so their place in the file is preserved by the output rather than by a scroll position.
- *Reversibility* — N/A for the reader; the gate mutates nothing. The one temporary state this PR
  introduces — the foundation's dead-code marker — is undone by the compiler rather than by
  anyone's memory: **AC-001**.
- *Keyboard reachability* — N/A; there is no pointer affordance. The equivalent is that the check is
  reachable from every wired invocation path without a flag, which milestone 2 established and
  which **AC-002**'s gate-integration half re-observes for this story's own addition.

**Composition invariants** (from `_design.md`, signed off 2026-08-17; each names the AC row that
carries it)

- *Presentation exists at all* — **AC-003** and **AC-004**. Each problem is a fully composed line
  (`  {path}:{line} — {message}`) built from the verified primitive at
  `xtask/src/lint_constitution.rs:605-660`, not a bare `Err`, not a debug-printed id, not a boolean.
  A check that returns `false` and lets the caller phrase the failure satisfies every type check and
  fails both rows.
- *Composition and placement* — **AC-001** and **AC-006**. The lines sit inside the checker's
  existing banner, in the same accumulating list the fence walk occupies, before the count. Two
  banners for one tree is the placement error `_design.md` `## Composition` ("Two steps, two
  banners") rules out for this surface.
- *Transience* — **AC-002** (success: persistent chrome, exactly one line, no per-citation
  narration) and **AC-003**/**AC-004** (problem lines: *opened on demand — by failing*, and only
  then). There is deliberately nothing opened-on-demand behind a verbose flag: clause citations are
  persistent chrome inline on the page precisely "because AC-007 exists because provenance is the
  thing that rots" (`_design.md:363`), and the check that reads them inherits the posture.
- *Density budget, with the real numbers* — **AC-003** carries the terminal surface's **80 columns**
  with the location first, and the **≤ 48-character location prefix inclusive of the 16-character
  `xtask\src\../../` doctest prefix** (≤ 32 characters repo-relative), which is finding 3's
  disposition (`_design.md:409-418`, `:420-428`). **AC-002** carries the **1-line** success budget.
  **AC-006** carries the **unbounded, never-truncated** problem list. The page-side budgets
  (≤ 250 source lines, ≤ 40-character H1, ≤ 80-column fences) are not this story's — it authors no
  page.
- *Hierarchy* — **AC-003**. Position only: `{path}:{line}` first, em dash, message; the two-space
  indent and the count recessive (`_design.md:453-460`). Colour, weight and size do not exist in
  this medium and the design says so.
- *Named anti-patterns* — **AC-006** carries anti-pattern **7** (a gate failure whose first visual
  row does not begin with `path:line`) and anti-pattern **8** (a truncated problem list — no
  `… and N more`, ever). **AC-008** carries anti-pattern **9** (no badge, tick, shield or
  "verified" mark asserting the documentation is checked for correctness or comprehension).
  Anti-patterns 1–6 and 10–11 bind the markdown tree, which this story does not render; they land on
  its slice-mates and on HS-P0021–23.

## Error conditions

| id | condition | required behaviour |
| -- | --------- | ------------------ |
| EC-001 | `spec/SPECIFICATION.md` is absent, unreadable, or declares no clause ids at all | `clause_ids`' error is propagated **unchanged**, naming the document and attributing the fault to the checker (`../spec-trace-clause-id-accessor/spec.md` EC-001/EC-002; `xtask/src/spec_trace.rs:2307-2309`). Never `Ok` with an empty set — that would report every real citation on every page as dangling, i.e. blame the pages for the specification. No bespoke message that loses the path. (AC-007) |
| EC-002 | a page under `TREE` cannot be read, or is not valid UTF-8 | Hard error naming the page, with context; never a skipped page and never a short problem list. RS-81-2, one medium over (`standards/rust/81-checks-that-cannot-be-types.md:95`). Consistent with the checker story's own EC-004, which set this posture for the module. (AC-007) |
| EC-003 | a citation-shaped token's family is declared nowhere in the specification (`XX-7`) | A **problem**, not a skip, whose message names the families the specification does declare — derived from the resolved set, so the message cannot go stale when a seventh family lands. An author who meant a constitution rule then spells it in full. (AC-001, AC-004) |
| EC-004 | a near-miss inside a declared family: `ES-` with no digits, `ES-4O` with a letter for a zero, `ES -40` with a space | A **problem** worded as *looks like a clause citation and does not parse; a citation the checker cannot read is one nothing verifies* — distinct from EC-005's wording, so the two are never confused in a log. Copied from `xtask/src/lint_constitution.rs:688-693`. (AC-004) |
| EC-005 | a well-formed id in a declared family that the specification does not declare (`ES-99`) | A **problem** in the design's drawn form: `` {page}:{line} — cites `ES-99`, which SPECIFICATION.md does not define `` (`_design.md:333`). (AC-003) |
| EC-006 | the same dangling id appears on several lines of one page, or several pages each carry one | **One problem per occurrence**, per line, all of them, in source order, count last. Deduplicating to one problem per id would hide the other lines a contributor must edit; fail-fast would hide the other pages. (AC-006) |
| EC-007 | a citation resolves but its clause is `[DEFERRED]`, or its family has no conformance suite | **Not a problem, and must not become one.** The resolver answers existence, never eligibility (`../spec-trace-clause-id-accessor/spec.md` AC-003, EC-004); applying a maturity or `has_suite` filter here would silently dangle every `PS-` and `SY-` citation. (AC-002) |
| EC-008 | a clause-shaped token is genuinely *data* inside a fenced example | Reported. The cost is stated rather than hidden, and the answer is to spell it outside the fence — **not** an allowance list. If the case recurs it is petitioned in the shape of `IGNORE_ALLOWANCES`, in its own change, with its own falsification (`_design.md`, `## Open questions` 3). Recorded as a documented limit. (AC-006, AC-008) |
| EC-009 | the tree is missing, or exists and holds no pages | **Not this story's condition** — milestone 2 already answers it (`../narrative-checker-mounted-with-pinned-path/spec.md` EC-001/EC-002: a missing tree names the pinned path, an empty tree `bail!`s before any check runs). Stated so nobody re-implements a vacuity guard beside this check, which would give one tree two guards with two messages. |

## Non-functional

| id | requirement | how it is held |
| -- | ----------- | -------------- |
| NF-001 | **one read and one parse of `spec/SPECIFICATION.md` per gate run.** The document is ~9,070 lines / ~567 KB; `clause_ids` is called once in `run()` and the set is passed by reference to this check and to `frozen-documentation-must-pin`'s | The foundation states it as a contract on its consumers (`../spec-trace-clause-id-accessor/spec.md` EC-006, NF-001); enforced here by review of the single call site, in the shape `lint_constitution::run` already uses (`xtask/src/lint_constitution.rs:180-190`) |
| NF-002 | **zero new dependencies.** `str` scanning over `std` plus the `anyhow` already in `xtask` — no regex crate, no markdown parser, no temp-directory crate | DR-12's standing trade in situ (`xtask/Cargo.toml:16-21`), which keeps `rusqlite` and `sqlx` out of xtask's dev graph because every `cargo xtask ci` would build them. All tests are in-memory over `&str`, which is what makes this free |
| NF-003 | **one extra pass over page text already in memory**, and no second file read per page. The added cost is a linear scan the class `xtask/src/affected.rs:28-36` already argues finishes inside the time cargo takes to decide `xtask` is up to date | AC-006's reviewer check: the check function takes the page text as an argument |
| NF-004 | no `unwrap` / `expect` outside `#[cfg(test)]`; the test module carries the scoped `#![allow(clippy::unwrap_used, reason = "test code, per the house style")]` | `xtask/src/lint_constitution.rs:827-829` is the exact shape to copy; enforced by clippy under `-D warnings` |
| NF-005 | `cargo xtask spec-trace` prints byte-identically to its pre-commit run, and the committed §7.1–§7.2 region is unchanged | The only edit to `xtask/src/spec_trace.rs` in this PR is the **deletion** of an attribute; `run` and the generated-region equality check (`:31-38`) are outside the diff. Run before *and* after the change, outputs compared |
| NF-006 | MSRV, `wasm32` and the feature powerset are untouched | `xtask` is a host-only `publish = false` tooling crate; this diff adds no `cfg`, no feature and no target attribute (`xtask/Cargo.toml:1-10`) |
| NF-007 | the diff contains **no `#[allow(dead_code)]`** and no new manifest entry | AC-001's lint tier; `cargo xtask ci --fast`'s clippy step with `-D warnings` is the instrument, and `crates/happenstance-neon/src/event_store.rs:232-240` is the in-repo precedent for why `expect` beats `allow` |
| NF-008 | no conformance rule, no port, no `Send` bound, no `serde`, no `CHANGELOG.md` obligation | Nothing here touches `crates/happenstance-testkit/`, a port or a value type, so CF-29's changelog check (`xtask/src/lints.rs:504-580`) is not triggered. Stated because a story silent about it is indistinguishable from one that forgot |

## Implementation notes (non-prescriptive)

Shape, not instructions. Every one of these is a *reason* an implementer may overrule with a better
one — except where a row of the acceptance table says otherwise.

- **Read `check_citations` in `xtask/src/lint_constitution.rs:674-693` first**, then
  `parse_citation` (`:795-809`) and the test module (`:827-841`). Three shapes are already solved
  there and copying them is cheaper and more consistent than inventing: the per-artifact check
  signature `(root, artifact, &mut Vec<String>)`, the `let Some(x) = … else { push; continue }`
  posture that turns an unreadable span into a problem instead of a `continue`, and a test that
  asserts the *parser* refuses a near-miss rather than asserting the whole run's exit code.
- **Split the scan from the decision.** A `fn clause_citations(line: &str) -> Vec<&str>`-shaped
  scanner returning candidate tokens, and a caller that classifies each against the resolved set,
  keeps every test in-memory and makes AC-005's boundary cases assertable on the scanner alone. If a
  test needs a filesystem, the split has not been made.
- **The two boundaries are two `char` predicates, not a regex.** `char::is_ascii_alphanumeric` on
  the preceding byte; on the following byte, reject `is_ascii_digit`, `-`, and
  `is_ascii_alphanumeric`. Write both, with `RS-81-1` and `ADR-0001` named in a comment as the
  tokens each one exists to protect — those are the two a reviewer will otherwise read as bugs.
- **Derive the families with one pass over the set.** `id.split_once('-')` on each resolved id,
  collected into a `BTreeSet<&str>`; do the derivation once, beside the `clause_ids` call, not per
  page and not per line.
- **Keep the three problem wordings distinct** (EC-003, EC-004, EC-005). A reviewer scanning a log
  should be able to tell "you meant a different family", "I could not read this" and "this clause
  does not exist" apart without opening the page. Reusing one message for all three collapses the
  distinction the hard-error posture exists to make.
- **Put the limits in the module's existing `# What this does not verify` section**, not in a new
  one. Milestone 2 created that section; a second heading beside it is two lists to keep in
  agreement (`xtask/src/lint_constitution.rs:9-28` is the shape, and it is one section).
- **Delete the `#[expect(dead_code, …)]` in the same commit as the first call**, not in a follow-up.
  Clippy will tell you the moment the call lands; the attribute's whole design is that it cannot rot
  into a permanent exemption.
- **Do not touch `xtask/src/main.rs`.** If it looks like a second step would be tidier, re-read
  `_design.md` `## Composition`: one banner is allotted to this checker, and a second banner makes
  one name answer two questions.
- **Do not re-prove the parser.** `clause_ids`' own story covers the parse, and §1.3's hand count
  checks it against the real document on every gate run (`xtask/src/spec_trace.rs:39-57`). A
  parallel fixture corpus here is exactly what the testing brief forbids
  (`_decomposition.md:704-720`).

## Tests and CI (merge gate)

Grounded in the project's testing brief — its AC-007 bullet (`_decomposition.md:617-628`), its
"Fixtures and seams to mock — there are almost none, deliberately" note (`:704-720`) and its
merge-gate command list (`:689-698`), narrowest first.

| tier | command / path | proves |
| ---- | -------------- | ------ |
| unit — the scanner, in-memory | `cargo test -p xtask` → `xtask/src/lint_narrative.rs` `mod tests`: `a_constitution_rule_id_is_not_a_clause_citation`, `an_adr_reference_is_not_a_clause_citation`, `a_backlog_item_id_is_not_a_clause_citation`, `the_boundaries_are_both_load_bearing` | AC-005. Page text as `&str`; no filesystem, no fixture tree, no temp directory, no new dev-dependency — the testing brief's explicit instruction |
| unit — resolution and the three problem forms | `cargo test -p xtask` → same module: `a_citation_naming_a_declared_clause_is_not_a_problem`, `a_dangling_id_names_the_page_the_line_and_the_id`, `a_citation_shaped_token_in_an_undeclared_family_is_a_problem`, `a_near_miss_inside_a_declared_family_is_a_problem`, `the_recognised_families_are_derived_from_the_resolved_set` | AC-001 (derivation half), AC-002 (static half), AC-003, AC-004. The id set is hand-built, so the specification is never mocked as a file — `spec_trace.rs`'s own suite already proves the parse |
| unit — accumulation, order and fence coverage | `cargo test -p xtask` → `five_dangling_ids_on_one_page_report_five_problems_in_source_order`, `a_clause_id_inside_a_fence_is_still_checked` | AC-006. The named wrong implementation each rejects: a fail-fast check, and a fence-excluding scan that leaves a hiding place inside the checked artifact |
| unit — hard-error posture | `cargo test -p xtask` → `an_unreadable_specification_is_propagated_not_swallowed`, `an_unreadable_page_is_a_hard_error_naming_the_page` | AC-007, EC-001, EC-002. No temp directory needed: a root that does not exist is enough |
| unit — the limits are first | `cargo test -p xtask` → `the_citation_check_documents_its_limits_before_its_guarantee` | AC-008's ordering half, read as text through the module's own `read` helper (`xtask/src/lint_constitution.rs:424-425`), so "limits first" is gated rather than reviewed |
| lint — the dead-code hand-off | `cargo xtask ci --fast` (clippy, `-D warnings`) | AC-001's `expect`-deletion half and NF-007. This is the tier that fails if the attribute is left in place, or replaced by `allow(dead_code)` |
| step — standalone | `cargo xtask narrative` over the tree as milestone 2 left it | AC-002's gate-integration half: exactly one banner, exactly one summary line, no `skipped:` line, the fixture page's `(ES-40)` resolving. Then the same command over a deliberately dangling page for AC-003/AC-006's composed-output check against `_design.md:327-352` |
| docs | `cargo xtask ci` (docs step) and `cargo test --locked -p xtask --doc` | AC-008's rendering half: the module docs build, and `cargo xtask lint-constitution` still passes so copying `lint_constitution.rs`'s shapes did not edit it (`_decomposition.md`, Note 8) |
| regression — the neighbouring checker | `cargo xtask spec-trace`, run before and after the change and the outputs compared | NF-005: `run`, §1.3's hand count and the committed §7.1–§7.2 region are untouched by a deletion-only edit |
| story-grain gate | `cargo xtask affected --base main` | that a change confined to `xtask/src/` selects the `xtask` package and reaches the checker in `affected::run`'s unconditional block — the grain `.redkiln/config.yaml:40` wires as this story's actual check, so a gap here is invisible until a later story compiles nothing |
| merge bar | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | the bar a non-terminal story meets (CLAUDE.md, "Commands"; `project.md` DoD 6). The full `cargo xtask ci` is the project's bar, not this story's |

No conformance rule is added, so no file under `crates/happenstance-testkit/` changes and CF-29's
`CHANGELOG.md` obligation is not triggered (NF-008).

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | how this PR bounds it |
| ---- | ------------------- | --------------------- |
| The check fires on `RS-81-1` or `ADR-0001` and reports correct prose as broken | **High / High** — this is the single most likely defect, because the constitution's own rule ids are two uppercase letters, a hyphen and digits, and this repository's prose is full of them | AC-005 is a first-class row with six named tokens, not a note. Both boundaries are written as explicit predicates with the protected token named in a comment. A checker missing either fails four tests |
| A literal six-prefix list appears anyway, because it was easier to type than to find the resolver | Medium / High, and it fails **quietly**: every id of a newly added family reports as dangling while `spec-trace` counts it happily | AC-001 asserts derivation with a fictional family the constant could not contain; the foundation lands first in the same context (`_storymap.md:157-164`); `SECTIONS`' own doc comment is quoted in the context pack |
| The scan excludes fences "to avoid false positives", creating a hiding place inside the checked artifact | Medium / High — it looks like a kindness and is the fence-shaped version of the hidden-panel defect DT-7 rejected | AC-006's `a_clause_id_inside_a_fence_is_still_checked`, and EC-008 states the cost and names the petition route (`_design.md`, `## Open questions` 3) instead of leaving it to taste |
| The near-miss case is implemented as a silent `continue`, copying `spec_trace::citations` instead of `lint_constitution` | Medium / High — the failure is a citation nothing verifies, which is the exact thing this story exists to prevent | AC-004 with its own tests; both postures are cited side by side in the context pack (`xtask/src/spec_trace.rs:2215-2260` vs `xtask/src/lint_constitution.rs:30-44`) so the choice is visible rather than accidental |
| `#[expect(dead_code, …)]` is left on `clause_ids`, or widened to `allow` | Medium / Low, and self-correcting | The unfulfilled expectation is a warning and `-D warnings` is a failure, so it surfaces on the first clippy run. AC-001 and NF-007 make it a criterion rather than a courtesy |
| A second `Step` is added for the citation check | Low / Medium — one tree's failures split across two banners, contradicting a signed-off design | `xtask/src/main.rs` is outside the PR boundary; the mount point names the single call site |
| A dangling-citation problem is reported without the page path, because the check has only the text | Low / High — it satisfies "fails the gate" and fails project AC-007's *naming* half and anti-pattern 7 | AC-003 asserts the composed form including the location, and the check signature takes the page alongside its text |
| `xtask/src/spec_trace.rs` conflicts on merge with the unmerged `initiative/from-contract-to-published-library` branch (521 lines of divergence in the specification) | Low / Low here — this PR's only edit to that file is a deletion | Resolution is by clause id, never by line number; a merged divergence that *adds* clauses simply resolves more ids (`spec/SPECIFICATION.md:280`; `project.md:274-281`) |
| The green step is read as evidence that citations are *correct*, or that the page teaches | Low / **High** — the initiative's top-ranked risk and the one this project is most able to cause (`project.md:286`) | AC-008: limits first, no badge/tick/"verified" wording anywhere, and the deferral-versus-restatement question explicitly assigned to HS-P0021. `documented-blind-spots-and-their-proofs` audits this at the end of the project |

**Coupling.** Two files, and both are shared with exactly one other story each:
`xtask/src/lint_narrative.rs` is created by `narrative-checker-mounted-with-pinned-path` and also
edited by `frozen-documentation-must-pin` (the second consumer of the same resolved set), and
`xtask/src/spec_trace.rs` is `spec-trace-clause-id-accessor`'s. All three are in the same slice and
implemented in the same context, in the merge order `_storymap.md:157-164` fixes, which is why the
shared `clause_ids` call site is a coordination point rather than a conflict. Nothing in this PR
reads the harness, the fence walk's constants, or any page content.

## Dependencies

- **Blocks on** — both, and for different reasons:
  - `narrative-checker-mounted-with-pinned-path` — the module, the pinned `TREE`, the page
    enumeration, the vacuity guard, the accumulating `Vec<String>`, the `bail!` carrying the count,
    the `REQUIRED` step, its dispatch arm, its help line, its `lint_steps` membership and its place
    in `affected::run`'s unconditional block. Without it there is no `run()` to add a call site to
    and no banner for a problem line to appear under.
  - `spec-trace-clause-id-accessor` — `pub(crate) fn clause_ids(root: &Path) -> Result<BTreeSet<String>>`
    beside `all_rules`, and the `#[expect(dead_code, reason = …)]` this PR deletes. Without it the
    only way to resolve an id is the fourth prefix list AC-001 forbids.
- **Unlocks** — `documented-blind-spots-and-their-proofs`, which lists this story among its own
  `depends_on` because the project's limits list is only complete once every check that has a limit
  exists (`_storymap.md:56`, `:168-171`). This story owes that story the *existence* of its limits
  in the module docs (AC-008); their six-item contents are that story's.
- **Slice-mate, not a dependency** — `frozen-documentation-must-pin` also depends on
  `narrative-checker-mounted-with-pinned-path` and `spec-trace-clause-id-accessor`, and lands
  **after** this story in merge order. It is a sibling consumer of the same resolved set, not a
  prerequisite: an implementer who waits for the pin has misread the order.

## Anchors (progressive disclosure)

Open these when the row says to, not before. The context pack above is self-sufficient for
starting; these carry the depth it distilled. Nothing here is pasted in bulk on purpose.

| anchor | why it is load-bearing | when to open | serves |
| ------ | ---------------------- | ------------ | ------ |
| `xtask/src/lint_constitution.rs` | The three shapes this story copies rather than invents: the limits-first module docs and the "why the citation parser hard-errors instead of skipping" argument (`:9-44`), the per-artifact check list in `run` that the new call site mirrors (`:180-199`), `check_citations`' `let … else { push; continue }` posture and its unreadable-span problem (`:674-693`), `parse_citation` as the near-miss template (`:795-809`), the test module with its scoped `allow` and `a_citation_with_a_nested_backtick_does_not_parse` (`:827-841`), and the read-a-source-file-as-text pattern AC-008's test uses (`:424-425`) | **First, before writing a line.** Read `:30-44` and `:688-693` together — they are the posture and its executed consequence | AC-004 |
| `xtask/src/spec_trace.rs` | The resolver and the two seams: `SECTIONS`' "three lists that must agree" doc comment, which is AC-001's whole argument (`:107-153`), `clause_ids`' placement beside `all_rules` and the `#[expect(dead_code, …)]` to delete (`:1746`), `read` and `workspace_root` (`:2307-2316`), §1.3's hand count that already proves the parse so this story need not (`:39-57`), and `citations`' permissive skip — the posture this story must **not** copy (`:2215-2260`) | Before writing the `clause_ids` call and the family derivation; `:2215-2260` when the near-miss case tempts a `continue` | AC-001 |
| `.bklg/docs-that-teach/checked-documentation-surface/_design.md` | Signed off by a human on 2026-08-17 and **binding**: the worked failure report containing this story's own problem line verbatim (`:327-352`), clause citations as persistent inline chrome (`:363`), the one-line success budget and the never-truncated problem list (`:359-374`), the terminal density budget including finding 3's ≤ 48-inclusive-of-16 disposition (`:409-428`), the hierarchy carried by position alone (`:453-460`), the enumerated states the `Vec<String>` must express — *unresolvable clause id* and *unreadable citation* among them (`:555-569`) — and anti-patterns 7, 8 and 9 (`:571-597`) | Before composing any problem message, and again before writing the module's limits | AC-003 |
| `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` | Architecture Note 6 (`:250-274`) is the decision this story implements in the words that decided it — "do not copy a regex", the `SECTIONS` prefix requirement, and the transfer of the hard-error posture; the testing brief's AC-007 bullet (`:617-628`) names the three tests owed and the one template to copy; the fixtures note (`:704-720`) is why every test is in-memory; Note 8 (`:311-326`) is what must not move; Note 10 (`:351-360`) is the limits list this story adds to | Note 6 before implementing; the testing brief before writing the test module | AC-004 |
| `.bklg/docs-that-teach/checked-documentation-surface/spec-trace-clause-id-accessor/spec.md` | The foundation's own contract on this consumer: existence-not-eligibility (AC-003), blame-the-checker on an unreadable specification (AC-004, EC-001/EC-002), the one-call-per-run rule (EC-006), and the self-erasing marker whose deletion is this PR's (AC-006) | Before the first `clause_ids` call, and when clippy first reports the unfulfilled expectation | AC-007 |
| `.bklg/docs-that-teach/checked-documentation-surface/narrative-checker-mounted-with-pinned-path/spec.md` | The module this story edits, as its own story specified it: the pinned `TREE`, the page enumeration, the accumulate-all/source-order/count-last output shape (AC-006), the one-line success surface (AC-007), the unreadable-page posture (EC-004), and the limits section this story appends to (AC-009) | Before adding the call site, so the existing `run()` shape is reused rather than re-decided | AC-006 |
| `standards/rust/81-checks-that-cannot-be-types.md` | RS-81-1 — prove the blind spot in the tests, *then* state it in the docs — is AC-008's whole structure; RS-81-2 (`:95`) is the hard-error posture as a house rule rather than a local precedent; RS-81-3's directory scoping is why `lint_constitution.rs` is copied and not abstracted over | With AC-008's limits and AC-007's tests open together | AC-008 |
| `standards/rust/60-what-a-test-must-prove.md` | The bar each test row is graded against: a test that no plausible wrong implementation fails is decorative, which is why every AC row above names the implementation it rejects | Before writing the test module, and again if a test starts asserting a tautology | AC-005 |
| `xtask/src/affected.rs` | `:28-36` is the module's own argument for the unconditional file-reading block — the cost model NF-003 rests on and the reason the checker is on that list at all; `:118-125` is the block itself, which milestone 2 joined | Only when running the story-grain gate, or if the added scan's cost is questioned | AC-006 |
| `crates/happenstance-neon/src/event_store.rs` | The in-repo precedent for the self-erasing marker, with the reasoning attached at `:232-240`, including why `expect` beats `allow` — the sentence to cite if anyone proposes widening it | When the first `cargo clippy` run reports the unfulfilled expectation | AC-001 |
| `spec/SPECIFICATION.md` | §1.4's "clause IDs are stable and are never renumbered" at `:280` is the property that makes resolution-by-name meaningful and makes this check survive the 521-line divergence on the unmerged sibling branch; the six clause families' declarations are what `clause_ids` returns | When writing the family derivation, and when explaining to a reviewer why no line number appears in this check | AC-002 |
| `.bklg/docs-that-teach/checked-documentation-surface/project.md` | AC-007 verbatim (`:221-222`) — the criterion this story is measured against — DR-07's machine half (`:168-171`), the out-of-scope boundary that keeps the defers-versus-restates judgement in HS-P0021 (`:333`), and the risk row naming a green gate read as teachability as the project's largest self-inflicted risk (`:286`) | Before wording any problem message or module sentence that could sound like a correctness claim | AC-008 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | The three personas the criteria are framed from — application author (`:56`), adapter author (`:148`), evaluator (`:225`, with "nowhere for that question to go" and the fallback to reading 200 numbered clauses at `:266-272`) — and the standing qualification that none has been directly observed, which is why no criterion here claims comprehension | Before re-wording any acceptance criterion | AC-003 |
| `.bklg/docs-that-teach/initiative.md` | BR-09 (`:344`) and DoD scenario 12 (`:455-456`) — *"No page has become a second specification. Each normative claim a teaching page makes is a citation that resolves"* — the gold-source sentence whose mechanical half this story turns from assertable into checked | When writing the implementation report's DoD claim, so it claims the mechanical half only | AC-002 |
| `xtask/Cargo.toml` | DR-12's standing trade in situ (`:16-21`): why `rusqlite` and `sqlx` are deliberately absent from xtask's dev graph, which is the argument any new dependency — a regex crate, a markdown parser, `tempfile` — would have to beat | Only if a dependency starts to look necessary, which means the scan/decide split has not been made | AC-005 |

## Clarifications resolved during spec

- **The AC set is exactly the eight the front half enumerated** — AC-001 through AC-008. None added,
  none dropped, and the `_ledger.md` carries the same eight ids and no others.
- **The `expect`-deletion lives in AC-001 rather than in an AC of its own.** It is not separate
  work: being `clause_ids`' first caller is *what* makes the expectation unfulfilled, so the
  deletion is the same fact as the call. Splitting it would produce a criterion whose only
  verification is "the previous criterion happened".
- **Interaction quality is not N/A wholesale.** The state family is, and is answered line by line;
  the composition family is not, because the failure report is a composed surface a signed-off
  design binds (`_design.md:323-352`). Two state invariants turned out to have real analogues rather
  than being vacuous — non-occlusion (a new problem line must not displace an existing one) and
  preserved place (source order) — and both are carried by AC-006 as a table row, per the extraction
  rule. Nothing in that section is gated by a bullet.
- **Which token shape counts as a citation was still open, and is decided here.** The front half
  fixed it; this half makes it AC-005 with six named tokens, because the shape is a contract on
  HS-P0021/22/23's page authors and "narrow it later" silently stops checking citations already
  written. The two boundaries are the load-bearing half: without them the check fails on prose that
  is correct, which is a worse outcome than missing a defect.
- **A dangling id is reported per occurrence, not per id** (EC-006). Deduplicating looked tidier and
  is wrong: each occurrence is a line a contributor must edit, and a list that hides four of five is
  the six-review-cycle failure `_decomposition.md:218-219` names.
- **Fenced blocks are in scope, and the cost is recorded rather than parsed around** (EC-008). No
  allowance list is introduced for clause-shaped data inside an example; the petition route is
  `_design.md`, `## Open questions` 3, in its own change, with its own falsification.
- **AC-008's "limits first" is gated, not merely reviewed.** The module can be read as text through
  its own `read` helper, exactly as `check_harness` reads a source file
  (`xtask/src/lint_constitution.rs:424-425`), so the ordering of two lines in a doc comment is a
  real assertion. The *contents* of the six-item limits list stay
  `documented-blind-spots-and-their-proofs`'; this story owes only its own three limits and the
  section's continued existence.
- **`xtask/src/lint_narrative.rs` does not exist on `main` yet**, and is deliberately cited as the
  mount point anyway: it is created by `narrative-checker-mounted-with-pinned-path`, which lands
  before this story in the same slice and the same implementer context (`_storymap.md:150-164`).
  Every anchor in the table above is a path that exists today; the mount point is the one forward
  reference, and it is a dependency edge rather than an invention.
- **No ADR is owed.** The grounding pass found no Accepted decision atom under `.kb/decisions/`
  governing gate structure, documentation trees or citation checking (`_storymap.md:94-99`); no
  `[FROZEN]` clause is edited, amended or restated; no port, `Send` bound, feature or `serde`
  boundary moves. The one divergence from in-repo precedent — hard-erroring where
  `spec_trace::citations` skips — is a convention, discharged by a sentence in the module's own docs,
  which is exactly what architecture Note 6 asks for.
