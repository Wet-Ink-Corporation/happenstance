---
item: HS-S0143
stage: spec
created: 2026-08-17T13:16:05.099Z
updated: 2026-08-17T13:16:05.099Z
template_sig: 87bbf1d0
rendered_sig: aa8f1ee5
---

# Spec — The frozen documentation MUSTs are enumerated by clause id in one place

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` — BR-10, DoD scenario 11 (`:452-454`), the closing criterion (`:662-663`), and the non-goal "un-discharging the frozen documentation MUSTs" (`:220-223`) |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` — the DAG, and the unmerged-sibling gate decision |
| Project | `.bklg/docs-that-teach/checked-documentation-surface/project.md` — AC-008 (`:223-226`), DR-09 (`:175-180`), the cross-branch note (`:274-281`) |
| This spec | `.bklg/docs-that-teach/checked-documentation-surface/frozen-documentation-must-pin/spec.md` |
| Key brief — architecture | `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` Note 7 (`:275-300`) — the pin as an instrument, and "the set is re-derived, not copied"; Note 6 (`:250-274`) for the resolver it calls |
| Key brief — testing | `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` — the AC-008 bullet (`:617-643`) and "Fixtures and seams to mock" (`:704-720`) |
| Signed-off design (BINDING) | `.bklg/docs-that-teach/checked-documentation-surface/_design.md` — `## Items` (`:45-79`), `## Signatures` (`:83-104`), `gate-narrative-checker-step` (`:147-152`), `## Transience policy`, `## Density budget` terminal table (`:419-425`), `## States the API must express` (`:555-569`), `## Anti-patterns` (`:571-597`) |
| Roadmap pointer | `.bklg/docs-that-teach/checked-documentation-surface/_storymap.md` — this story's row (`:54`), the coverage row (`:120`), merge order 3.8 (`:157-164`) |

## One-line PR slice

Re-derive the frozen documentation MUSTs against `spec/SPECIFICATION.md` as it stands, enumerate
them by clause id in exactly one commented `const` naming each discharge site, and check three
things — the ids resolve, each site still contains its anchor, and the derived count matches the
hand-written one, naming which moved.

## Executive summary

This PR lands the pin BR-10 asks for: one enumerated, heavily commented `const` in the narrative
checker, and three checks over it that run inside the checker's existing mandatory step. It is the
story `HS-P0023` is sequenced behind, because the pin must exist *before* any project rewrites a
`happenstance-core` doc comment (`_storymap.md:157-164`).

The delta over what exists today is that the set stops being a claim in two documents that do not
agree, and becomes an instrument. `RUNBOOK.md:3830-3845` records "discharged nine documentation
MUSTs"; the evidence table behind it
(`references/evaluation/phase-4-5-reconciliation.md:119-142`) lists eight rows, two of which are
explicitly *not* clause discharges but comment repairs, and adds ES-19 as a ninth recorded by the
pass rather than re-verified. **The arithmetic does not close from the evidence alone**, which is
Note 7's stated reason for re-deriving rather than copying (`_decomposition.md:275-300`).

Two corrections to the brief's wording are carried by this spec, both grounded in what the tree
actually contains rather than in preference, and both are the reason this story is not a
twenty-line `const`:

1. **No discharge site names its clause id.** Verified on `main`: `ES-23`, `ES-24`, `VT-15`,
   `VT-3`, `ES-17` and `ES-40` each occur **zero** times in the file the evidence table names as
   their discharge site. The obligations are discharged by doc sections that state the substance
   without citing the clause — `# Cancellation` (`crates/happenstance-core/src/store.rs:146-165`),
   `# Equality is byte equality, and nothing is normalised`
   (`crates/happenstance-core/src/tag.rs:29-47`). So assertion 2 anchors on a **verbatim phrase per
   entry**, not on the id, and this PR does not add ids to those comments — that would be a touch of
   a discharging comment, which is HS-P0023's and is governed by
   `.kb/governance/rewrite-the-referent-never-the-reasoning.md`.
2. **"Derived count" needs a derivable fact.** The `const`'s own length is not derived, and a count
   written into its comment is the defect `UNCLAIMED_PENDING_ADR` already refuses — its count "is
   computed and printed rather than written here, so this comment cannot come to disagree with the
   array beneath it" (`xtask/src/spec_trace.rs:1975-1978`). The derived side is therefore a
   **candidate scan** of `spec/SPECIFICATION.md`, and the hand-written side is the classified
   enumeration; the failure names which of the two moved (RS-81-5).

What this PR deliberately does not do: add a step, a subcommand or a `REQUIRED` entry; touch
`spec/SPECIFICATION.md`; touch any `happenstance-core` doc comment; or change what a green
`cargo xtask spec-trace` prints.

## Context pack

Everything here is a decision already taken. Honor it; do not re-decide it.

**The pin is the only instrument that can observe these clauses, and that is why it exists.** A
documentation MUST is the one clause shape no conformance rule can reach. ES-23 says so about
itself: "**Rule:** none. … this clause's remaining content constrains callers and adapter
documentation, which no conformance rule can observe. It is stated as a clause rather than as prose
only because 'unspecified' is itself the normative content"
(`spec/SPECIFICATION.md:3592-3596`). Ten clauses carry `**Rule:** none`. For everything else in the
specification, `cargo xtask spec-trace` proves the clause names a rule and the rule exists; for
these, nothing in the repository proves anything. The pin is not paperwork about a decision already
enforced — it is the *first* enforcement, and its absence is why a doc-comment rewrite could
silently un-discharge a `[FROZEN]` clause.

**Re-derive, do not copy — and write the reconciliation down.** The two existing statements of the
set disagree (nine vs. eight vs. two rows that are not clause discharges), and the brief is explicit
that "a `const` whose comment says 'nine, per RUNBOOK' while holding eight entries is the defect
BR-10 exists to prevent, one level up" (`_decomposition.md:275-300`). The re-derivation is
evidence-gathering, not a test: it belongs in this story's own artefacts and in the `const`'s
comment, not in `#[test]` code (testing brief, `:617-643`).

**The derivation rule is a classification, and the classification is the deliverable.** A naive scan
of `spec/SPECIFICATION.md` for documentation-obligation wording finds far more than eight, and the
extras are not noise — they are a real distinction the pin must make and record. Verified candidates
beyond the evidence table's set: VT-13 (`:1075-1080`), VT-32 (`:1191-1197`), VT-33 (`:1242-1246`),
VT-21 (`:1483-1488`), VT-22 (`:1513-1518`), VT-24 (`:1556-1561`), ES-35 (`:4142-4147`), PS-36
(`:5601-5603`), CF-39 (`:7631-7636`), CF-40 (`:7661-7665`). Several impose their obligation on an
**adapter** or on a **fixture** ("MUST document its actual limit", "a fixture MUST state its store's
capacity ceilings"), not on the contract's own documentation, and one sits in a family that is not
`[FROZEN]`. Each candidate is therefore either **pinned** with a site and an anchor, or **excluded**
with a one-line reason, and the set of the two together is what the derived scan is compared
against. An unclassified candidate is a problem — that is the whole mechanism.

**Clause ids nest, so match whole identifiers.** `ES-1` is a substring of `ES-17`, `VT-3` of `VT-33`,
`ES-4` of `ES-40`. RS-81-3 exists because exactly this defect shipped once already: a longer name's
changelog entry "silently discharged the shorter rule's obligation and inflated any per-rule
arithmetic built on the same match" (`standards/rust/81-checks-that-cannot-be-types.md:209-217`).
`fn names_rule` at `xtask/src/lints.rs:492-500` is the shape to copy — copy it, do not refactor to
share it (Note 8, and RS-81-3's own evidence line). A `contains("ES-1")` anywhere in this diff is the
named wrong implementation.

**Resolve through `clause_ids`, once per run.** `pub(crate) fn clause_ids(root: &Path) ->
Result<BTreeSet<String>>` is `spec-trace-clause-id-accessor`'s deliverable, beside `all_rules`
(`xtask/src/spec_trace.rs:1746`). Do not add a second parser, a regex or a prefix list: a literal
prefix list would be the fourth list of the six clause families and `SECTIONS`' own doc comment
already says why the fourth loses (`:107-120`). The foundation spec pins the calling contract —
**the checker calls `clause_ids` once and passes the set to both the citation check and the pin
check**; two calls parse a 9,070-line document twice in one step for no new information
(`spec-trace-clause-id-accessor/spec.md`, EC-006). If the self-erasing
`#[expect(dead_code, reason = …)]` on `clause_ids` has not already been deleted by
`narrative-citation-resolution`, deleting it here is in-boundary and required: an unfulfilled
expectation is a warning and the gate is `-D warnings`.

**Existence, not eligibility — inherited, not re-decided.** `clause_ids` applies no maturity or
family filter, so a `[DEFERRED]` id resolves. The pin's *own* filter is the classification above,
applied by a human and recorded in the `const`; it is not applied inside the resolver.

**This is a check inside an existing step, not a new step.** `narrative-checker-mounted-with-pinned-path`
lands the module `lint_narrative` (`xtask/src/lint_narrative.rs`), its five mount sites in
`xtask/src/main.rs`, and the step named **`every narrative page is checked`** with `probe: None`.
This story adds a check *into* that module: problems accumulate in the same `Vec<String>`, print in
source order with a two-space indent, and the count comes last —
`bail!("{} problem(s) in {TREE}", problems.len())`, the shape of
`xtask/src/lint_constitution.rs:190-199`. Never fail fast; never truncate (`_design.md`
anti-pattern 8 forbids any "… and N more"). A second step, a second banner or a second subcommand
would split one surface in two and is out of boundary.

**Where the pin lives, and the alternative that lost.** The `const` and its three checks go in
`xtask/src/lint_narrative.rs`. The alternative — beside `RULE_FILES` / `WIRE_TESTS` in
`xtask/src/spec_trace.rs:85-105`, where the shape comes from — loses on two counts: it would put a
documentation check inside the specification's cross-reference step and change the sentence a green
`cargo xtask spec-trace` prints, which project AC-008 requires to keep passing over the tree as it
stands; and it would make a second caller of `clause_ids`, breaking the one-call-per-run contract.
RS-81-3's directory-scoping half does not argue the other way: it scopes `read_dir`-shaped
*scanners* to the directory they constrain, and the pin is a fixed enumeration of named files, not a
scanner.

**A false positive here is worse than in any other check in this project.** The remedy a contributor
reaches for when a check fires on an innocuous edit is to edit the check — and the check *is* the
pin, so a jumpy anchor teaches people to delete entries. Two consequences, both binding. First, the
anchor match is **reflow-insensitive**: strip doc-comment prefixes and collapse whitespace before
matching, so re-wrapping a paragraph is not a gate failure. Choosing anchors that happen to sit on
one source line is the alternative and it lost — every anchor available today does fit one line,
which is exactly what makes the trap easy to walk into. Second, an anchor is chosen to be the
**load-bearing sentence** of the discharge, not a nearby convenience: `# Cancellation` is the section
the clause demands; "at-most-once under verbatim reissue" is the guarantee ES-24 demands.

**What the check cannot see, and must say so first.** The module's docs open with what it does not
verify — `lint_constitution.rs:9-28`'s reason applies verbatim, a check whose limits are undocumented
is read as a guarantee. This story's own limits are sharp: an anchor can survive while the
*reasoning* around it is rewritten, which is a re-discharge the check cannot detect
(`.kb/governance/rewrite-the-referent-never-the-reasoning.md` is the human rule that covers it); a
clause wording its documentation obligation outside the enumerated candidate phrases is invisible to
the derived scan; and the pin proves the discharge is *present*, never that it is *adequate*. No
badge, no tick, no "verified" (`_design.md` anti-pattern 9), and nothing anywhere claiming the
surface proves a page teaches (project DoD item 8).

**Ids are stable names, which is why this survives the sibling branch.** Clause ids "are stable and
are never renumbered" (`spec/SPECIFICATION.md:280`). `initiative/from-contract-to-published-library`
is unmerged and diverges from that document by 521 lines; the pin resolves by id, not by line, so the
divergence cannot invalidate it (`project.md:274-281`). The residual risk the project records — that
the sibling *adds* documentation MUSTs — was deferred to HS-P0025 at closeout. The candidate scan
turns it into a gate failure on the merge-forward commit instead, and that is the single largest
thing this design buys over a hand list.

**The persona-journey slice.** No runtime surface. The reader is a **contributor running the gate**
and a **reviewer reading its output** (`_storymap.md:12-16`), acting for the **adapter author** who
meets these MUSTs at their discharge sites. The reader state this story makes true: a contributor
about to rewrite a `happenstance-core` doc comment learns from the gate, not from a memory of a
reconciliation pass, that the paragraph in front of them discharges a `[FROZEN]` clause.

## Integration contract

- **Archetype**: `capability` — observable through `cargo xtask ci`, `cargo xtask ci --fast`,
  `cargo xtask narrative` and `cargo xtask affected --base main`.
- **Slice / milestone**: `specification-pin`. Slice-mates, implemented in one context:
  `spec-trace-clause-id-accessor` (the foundation, first) and `narrative-citation-resolution`. This
  story is **last** of the three in merge order (`_storymap.md:157-164`) and is the one HS-P0023 is
  sequenced behind.
- **Mount point**: **`xtask/src/lint_narrative.rs`** — the narrative checker's `run`, which
  `narrative-checker-mounted-with-pinned-path` has already mounted as the `REQUIRED`, `probe: None`
  step `every narrative page is checked` from the bin crate's composition root
  (`xtask/src/main.rs`, five sites: the `mod` list at `:65`, `REQUIRED` at `:105`, the dispatch arm at
  `:689`, `print_help()` at `:718`, `lint_steps()` at `:799`). **This story adds no mount site**: the
  pin is a check inside that `run`, reported through the same problem `Vec` and the same `bail!`. A
  pin that compiles and is called by nothing but a test is the decorative-check defect this project
  exists to refuse.
- **Wires into**:
  - `crate::spec_trace::clause_ids` (`xtask/src/spec_trace.rs`, beside `all_rules` at `:1746`) — the
    resolver, called **once per run** by the checker and shared with the citation check.
  - `crate::spec_trace::workspace_root` (`:2311-2316`) — how every file-reading check in this crate
    resolves the repository root; and `read` (`:2307-2309`) for its `reading {rel}` context.
  - `xtask/src/lint_narrative.rs`'s existing problem accumulation, success line and `bail!` — this
    story appends to them and changes neither shape.
  - `xtask/src/lints.rs:492-500` (`fn names_rule`) — the whole-identifier matching shape, copied not
    shared (Note 8).
  - `xtask/src/spec_trace.rs:85-105` (`RULE_FILES` / `WIRE_TESTS`) and `:1975-2010`
    (`UNCLAIMED_PENDING_ADR`) — the two in-repo shapes for an enumerated, heavily commented `const`
    that carries a decision per entry.
  - The seven discharge-site files, **read-only**: `crates/happenstance-core/src/store.rs`,
    `tag.rs`, `event.rs`, `append.rs`.
  - No workspace crate is depended on, no port, no `Send` bound, no feature: ADR-0001 and ADR-0003
    are untouched by construction.
- **Renders surfaces**: `gate-narrative-checker-step` (`_design.md:147-152`), in its `pass`,
  `fail-one` and `fail-many` states. **No new state id** is added to that surface's list: the pin's
  three failures are instances of the problem-line form the design already fixed, and the design
  already designs their messages in — *pinned MUST no longer at its discharge site* (AC-008) and
  *count disagreement (derived vs. hand-written, RS-81-5)* are two of the fourteen states
  `## States the API must express` enumerates (`_design.md:555-569`). `pass` is unchanged: the
  checker's success output stays exactly one line and its wording belongs to the story that owns it.
  No markdown surface is rendered or changed.
- **Public items** (`_design.md:45-79`): none of the five rows in the design's `## Items` block is
  implemented here — `clause_ids` is the foundation story's, `TREE` / `HARNESS` are the checker
  story's, `IGNORE_ALLOWANCES` / `HIDDEN_MARKERS` are the other slice's. This story adds one private
  `const` the `## Items` block does not enumerate, and says so rather than quietly extending a
  signed-off list: the design nonetheless *designs it* — both of its failure states are in
  `## States the API must express` (`:555-569`), the brief fixes its shape
  (`_decomposition.md:275-300`), and `_design.md`'s visibility table gives every private `const` in
  this module the same answer (private, no feature, no semver promise, `xtask` is `publish = false`).
  Its name is the implementer's; the story map's working name is `FROZEN_DOC_MUSTS`.
- **Conformance rule(s)**: none, and this is not adapter-observable. Nothing here touches
  `crates/happenstance-testkit/`, a port, a value type or a fixture — it reads a specification and
  four source files as text. CF-29's `CHANGELOG.md` obligation reads `RULE_FILES`
  (`xtask/src/lints.rs`) and is therefore not triggered. Stated because a story that changes a port
  and names no rule is a port change nothing can fail; this changes no port. The instruments are the
  module's own `#[cfg(test)] mod tests` and the gate's own output.
- **Clause(s)**: **none discharged, none amended, and this is the story where that matters most.**
  The pin *observes* the discharge of ES-19, ES-23, ES-24, VT-3, VT-15, VT-17, ES-17 and ES-40 (the
  set as re-derivation confirms it); it does not restate, extend or reinterpret any of them, and it
  does not edit `spec/SPECIFICATION.md`. No `[FROZEN]` clause is changed, so no ADR is owed — and the
  grounding pass found no Accepted decision atom under `.kb/decisions/` governing gate structure or
  documentation checks (`_storymap.md:93-99`, `_grounding.md` "Precedence and non-goals").
- **Advances DoD scenario**: initiative DoD **11** — *"The frozen documentation MUSTs are still
  discharged. The pinned set of clause ids is enumerated in one place, and the specification
  cross-reference step passes over the tree as it stands after every doc comment this work touched"*
  (`initiative.md:452-454`). This story is the one that turns scenario 11 from an assertion into a
  check, and it is the gate every later doc-comment rewrite in the initiative passes through. It also
  discharges the initiative's closing criterion at `:662-663`.

## PR boundary

The paths this story is allowed to touch, as globs. `redkiln verify --grain story` reads the first
fenced block under this heading and fails on any file changed outside it.

```
xtask/src/lint_narrative.rs
xtask/src/spec_trace.rs
.bklg/docs-that-teach/checked-documentation-surface/frozen-documentation-must-pin/**
```

**In this PR**

- The enumerated, heavily commented pin `const` in `xtask/src/lint_narrative.rs`: one entry per
  candidate, each carrying the clause id, its disposition (pinned or excluded, with the reason), the
  discharge-site path where pinned, and the verbatim anchor.
- The candidate-phrase `const` the derived scan uses, with its own limit stated next to it.
- The three checks, wired into the existing `run`: resolution through `clause_ids`, site-and-anchor
  presence, and the derived-vs-hand comparison that names which side moved.
- The whole-identifier matcher copied from `xtask/src/lints.rs:492-500`, and the doc-comment
  normalisation the anchor match runs over.
- This module's "What this does not verify" section gaining the pin's own three limits, first rather
  than last.
- `#[cfg(test)] mod tests` additions in the house shape (`xtask/src/lint_constitution.rs:827-841`,
  including the scoped `#![allow(clippy::unwrap_used, reason = "test code, per the house style")]`).
- `xtask/src/spec_trace.rs`: **one permitted edit only** — deleting the self-erasing
  `#[expect(dead_code, reason = …)]` on `clause_ids` if `narrative-citation-resolution` has not
  already removed it. Nothing else in that file is touched.
- This story's own `_ledger.md`, the re-derivation record, and the implementation report.

**Explicitly not in this PR**

- **Any edit to `spec/SPECIFICATION.md`.** No clause is amended, restated or renumbered; the
  initiative is additive and says so (`project.md:134`).
- **Any edit to a `happenstance-core` doc comment**, including adding a clause id to one. The
  `crates/happenstance-core/src/store.rs` rewrite is HS-P0023's (`project.md:130`), and every such
  touch is governed by `.kb/governance/rewrite-the-referent-never-the-reasoning.md`. This story
  reads those files and changes none of them.
- A new step, banner, subcommand, `print_help` line, `REQUIRED` entry or `lint_steps` member — the
  checker's step is the mount, and `xtask/src/main.rs` is outside the boundary for that reason.
- Any change to `run` in `xtask/src/spec_trace.rs`, to the generated §7.1–§7.2 region, or to what
  `cargo xtask spec-trace` prints.
- `clause_ids` itself, and citation parsing or dangling-citation reporting — the two slice-mates'.
- The fence walk, `IGNORE_ALLOWANCES`, `HIDDEN_MARKERS`, and any narrative page: fixture or corpus.
- Any refactor of `xtask/src/lint_constitution.rs`, `xtask/src/constitution.rs` or `xtask/src/lints.rs`
  to share code with the new checks (RS-81-3's evidence and Note 8: copy the shape).
- Any new dependency in `xtask/Cargo.toml`. DR-12's standing trade stands (`:16-21`); every check
  here is `read_to_string` plus string matching over `std` and `anyhow`.
- The six-item contents of the limits section (`documented-blind-spots-and-their-proofs`) and any
  observed-failure transcript (`observed-failure-falsification`).

The implementer MAY touch the wiring named in the Integration contract to mount this slice; here that
wiring is `run` in `xtask/src/lint_narrative.rs`, and it is the point of the story rather than scope
drift.

**Merge DoD**: `cargo xtask ci --fast` is green (`.redkiln/config.yaml:55`); `cargo xtask narrative`
passes standalone, printing exactly one summary line; `cargo xtask spec-trace` prints what it printed
before this commit; `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) runs the checker on
a prose-only diff; `cargo test -p xtask` covers all three assertions plus the nesting defect; and the
re-derivation record exists with the nine-versus-eight arithmetic closed.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| -------------------- | ------- | ------------- |
| The set is re-derived against the document as it stands | Not copied from either existing statement. `RUNBOOK.md:3830-3845` says nine; the evidence table lists eight rows of which two are comment repairs rather than clause discharges, plus ES-19 recorded but not re-verified. The reconciliation is written down — including, if that is the answer, that the count was nine and is now eight and why. | `_decomposition.md:275-300`; `references/evaluation/phase-4-5-reconciliation.md:119-142`; `RUNBOOK.md:3830-3845` |
| The derivation rule is stated, not implied | A pinned entry is a clause that (a) is `[FROZEN]` and (b) places its obligation on **the contract's own documentation**, as opposed to an adapter's or a fixture's. That second half is what separates ES-23 from VT-21's "MUST document its actual limit" and CF-40's "a fixture MUST state its store's capacity ceilings". The rule goes in the `const`'s comment where the next reader meets it. | `spec/SPECIFICATION.md:3574-3603`, `:1483-1488`, `:7661-7665`; `_decomposition.md:275-300` |
| Every candidate is disposed, pinned or excluded | The verified candidate set beyond the evidence table's — VT-13, VT-32, VT-33, VT-21, VT-22, VT-24, ES-35, PS-36, CF-39, CF-40 — is classified with a one-line reason each. An unclassified candidate is a problem, which is what makes a clause added on the sibling branch fail the gate rather than wait for closeout. | `spec/SPECIFICATION.md:1075-1080`, `:1191-1197`, `:1242-1246`, `:1483-1488`, `:1513-1518`, `:1556-1561`, `:4142-4147`, `:5601-5603`, `:7631-7636`; `project.md:274-281` |
| One place, one `const`, one comment per entry | An enumerated array in `xtask/src/lint_narrative.rs`, shaped on `RULE_FILES` / `WIRE_TESTS` and `UNCLAIMED_PENDING_ADR`: the decision travels with the entry, not in a paragraph above the array. No second enumeration of the set exists anywhere in the repository. | `xtask/src/spec_trace.rs:85-105`, `:1960-2010`; `_decomposition.md:275-300`; `project.md:223-226` |
| Assertion 1 — every pinned id resolves | Through `crate::spec_trace::clause_ids(root)`, called **once** per run by the checker and shared with the citation check. No second parser, no regex, no prefix list — a fourth family list is one that can half-land. An id that does not resolve is a problem naming the id and the pin, never a page. | `xtask/src/spec_trace.rs:1746`, `:107-120`; `_decomposition.md:250-274`; `spec-trace-clause-id-accessor/spec.md` EC-006 |
| Assertion 2 — the site exists and still carries its anchor | Per entry: the discharge-site file is read (`read`'s `reading {rel}` context on failure) and the verbatim anchor must be present. A missing **file** and a missing **anchor** are two distinct problems with two distinct messages — the first says the path moved, the second says the discharging text did. | `xtask/src/spec_trace.rs:2307-2309`; `xtask/src/lint_constitution.rs:423-425`; `_design.md:555-569` |
| The anchor is a phrase, not the clause id — because no site names its id | Verified on `main`: zero occurrences of `ES-23`/`ES-24` in `store.rs`, `VT-15` in `tag.rs`, `VT-3`/`ES-17` in `event.rs`, `ES-40` in `append.rs`. The obligations are discharged in prose. Anchors are the load-bearing sentence of each discharge — e.g. `# Cancellation` (`store.rs:146`), `at-most-once under verbatim reissue` (`store.rs:169`), `# Equality is byte equality, and nothing is normalised` (`tag.rs:29`), `may legally appear more than once` (`tag.rs:255-256`), `Not a clone-avoidance route for adapters` (`event.rs:406`), `# A claim about one store's log, not about the world` (`append.rs:29`), `It is not a sound \`after\` for a follow-up condition` (`store.rs:131`). | `crates/happenstance-core/src/store.rs:131-193`; `tag.rs:29-47`, `:253-260`; `event.rs:404-418`; `append.rs:29-47`; `spec/SPECIFICATION.md:3465-3479` |
| The anchor match is reflow-insensitive | Doc-comment prefixes are stripped and whitespace collapsed before matching, so re-wrapping a paragraph is not a gate failure. The alternative — anchors constrained to one source line — lost: every anchor available today happens to fit one line, so the trap is invisible until someone reflows, and the remedy a false positive teaches is deleting the entry. | `_design.md:571-597` (anti-pattern 8's spirit); RS-81-1, `standards/rust/81-checks-that-cannot-be-types.md:11` |
| Whole-identifier matching, everywhere an id is matched | Clause ids nest: `ES-1` ⊂ `ES-17`, `VT-3` ⊂ `VT-33`, `ES-4` ⊂ `ES-40`. Copy `fn names_rule`'s boundary test; `contains("ES-1")` is the named wrong implementation and it shipped once already for rule names. | `xtask/src/lints.rs:486-500`; `standards/rust/81-checks-that-cannot-be-types.md:209-262` |
| Assertion 3 — derived vs. hand-written, naming which moved | The derived side is a candidate scan of `spec/SPECIFICATION.md` over an enumerated phrase `const`; the hand side is the classified enumeration. The failure distinguishes **a candidate nobody classified** (the document grew) from **a pinned or excluded entry that is no longer a candidate** (the enumeration is stale), and names the ids on each side. Never "the counts disagree". | `standards/rust/81-checks-that-cannot-be-types.md:335-341`; `xtask/src/spec_trace.rs:39-57` |
| No count is written into a comment | The `const`'s length is the hand-written intention; nothing restates it in prose. `UNCLAIMED_PENDING_ADR` already refuses that trade — its count "is computed and printed rather than written here, so this comment cannot come to disagree with the array beneath it". | `xtask/src/spec_trace.rs:1975-1978` |
| Reported through the existing step, in the existing shape | Problems push onto the checker's one `Vec<String>`, print to stderr with a two-space indent, and the count comes last via `bail!("{} problem(s) in {TREE}")`. Never fail fast — "a check that stops at the first problem turns one review cycle into six". Never truncate: no "… and N more". | `xtask/src/lint_constitution.rs:190-199`; `_decomposition.md:218-219`; `_design.md:571-597` anti-pattern 8 |
| A problem line names its artifact first | Location-first within the 80-column terminal budget, so soft-wrap cannot push the artifact off the first visual row. Where no line number is meaningful the `{path} — {message}` form is used, which is the form the checker already carries for harness-registration problems. | `_design.md:419-425`, `:571-597` anti-pattern 7 |
| Green output is unchanged | The checker's success line stays exactly one line, as its owning story wrote it. A second summary line is the "green check that says ten lines" the transience policy forbids, and extending the existing string is a slice-mate's decision, not this story's — permitted at implementation only if it stays one line within the 80-column budget. | `_design.md` `## Transience policy`; `narrative-checker-mounted-with-pinned-path/spec.md` |
| Read-only over `happenstance-core` and over the specification | Four source files and one specification are read; none is written. No clause id is added to a doc comment, because that is a touch of a discharging comment and belongs to HS-P0023 under the referent/reasoning rule. | `.kb/governance/rewrite-the-referent-never-the-reasoning.md`; `project.md:130`; `initiative.md:220-223` |
| `cargo xtask spec-trace` still passes and still prints what it printed | Project AC-008 requires it explicitly. `run` is outside the diff and the generated §7.1–§7.2 equality check is unaffected because nothing writes. | `project.md:223-226`; `xtask/src/spec_trace.rs:31-38` |
| Limits first, and proven where a proof is possible | The module's "What this does not verify" section gains three entries: an anchor can survive a rewritten *reasoning* (a re-discharge this check cannot see); a documentation obligation worded outside the candidate phrases is invisible to the derived scan; and presence is not adequacy. RS-81-1's order — prove the blind spot in the tests, then state it in the docs. | `standards/rust/81-checks-that-cannot-be-types.md:11-94`; `xtask/src/lint_constitution.rs:9-28`; `_design.md` `## What a user meets first` |
| No claim of teachability, anywhere | No badge, tick, shield or "verified" wording; nothing asserts the pin proves a doc comment teaches or that a discharge is correct. Project DoD item 8 and `_design.md` anti-pattern 9. | `_design.md:571-597`; `project.md:260` |
| Tests are in-memory except the real-path reads | Assertions run against `&str` slices — a specification slice, a source-file slice, a pin array — with no temp directory and no new dev-dependency; only the whole-pin test touches the real tree, through `workspace_root` and `read`. Do not build a parallel fixture corpus of `SPECIFICATION.md`. | `_decomposition.md:617-643`, `:704-720`; `xtask/src/lint_constitution.rs:827-841` |
| Every check can fail, and the wrong implementation is named | Three named wrong implementations exist to reject: `contains` instead of whole-identifier matching (passes `ES-1` on `ES-17`); an anchor equal to the clause id (fails on every site today, which is how the check would be "fixed" by widening); and a count comparison that reports disagreement without naming the side. A check no edit can fail is decorative. | CLAUDE.md, "The rule that matters"; `standards/rust/81-checks-that-cannot-be-types.md:209-262`, `:335-341` |

## Data and migrations

**N/A.** No schema, no store, no persisted state, no serialised envelope and no released artifact.
`xtask` is `publish = false`, so nothing here is in anyone's dependency graph and there is no semver
promise to migrate (`_design.md` `## Visibility and stability`).

Five artifacts are read and none is written: `spec/SPECIFICATION.md` through the existing `SPEC`
constant and `read` helper, and the four `crates/happenstance-core/src/` files the pin names. This
story runs no `spec-trace --write` and does not touch the generated §7.1–§7.2 region, so the
committed-versus-computed equality check the gate already runs is unaffected
(`xtask/src/spec_trace.rs:31-38`).

The one adjacent hazard worth naming is not a migration either. `initiative/from-contract-to-published-library`
is unmerged and diverges from `spec/SPECIFICATION.md` by 521 lines. The pin resolves by clause id, and
ids are stable and never renumbered (`spec/SPECIFICATION.md:280`), so the divergence cannot invalidate
an entry. What it *can* do is add a documentation MUST — and under the candidate scan that surfaces as
an unclassified candidate on the merge-forward commit, which is a gate failure a human disposes of,
rather than the closeout re-check `project.md:274-281` had to settle for.

## Acceptance criteria

The "user" is a **contributor running the gate** and a **reviewer reading its output**
(`_storymap.md:12-16`), acting for the **adapter author** whose journey is *"walk the adapter path,
not just the recipe"* (`initiative.md:297-300`) — the persona who meets these MUSTs at their
discharge sites and is the reason a silently un-discharged `[FROZEN]` clause costs something. Every
criterion below is one of those readers completing something, and every one of the eight traces to
project **AC-008** (`project.md:223-226`).

| id | criterion | verification |
| -- | --------- | ------------ |
| AC-001 | **GIVEN** a reviewer who inherits two disagreeing statements of the frozen documentation MUST set — `RUNBOOK.md:3830-3845` records "nine", the evidence table behind it (`references/evaluation/phase-4-5-reconciliation.md:119-142`) lists eight rows of which two are comment repairs rather than clause discharges, and ES-19 was recorded by the pass rather than re-verified — **WHEN** they open this PR to decide whether the set is now trustworthy, **THEN** they meet **exactly one** enumerated `const` in `xtask/src/lint_narrative.rs` in which *every* candidate the derivation rule produces is disposed of: **pinned** with its clause id, its discharge-site path and its verbatim anchor, or **excluded** with a one-line reason. The derivation rule — (a) the clause is `[FROZEN]` and (b) its obligation falls on **the contract's own documentation**, not on an adapter's or a fixture's — is stated in the comment where the next reader meets it, not in a paragraph elsewhere. **AND** a written re-derivation lives in this story's own artefacts and closes the nine-versus-eight arithmetic by name, including the answer that the count *was* nine and is now eight if that is the answer. No count is written into any comment. | `xtask/src/lint_narrative.rs::tests::every_pin_entry_is_disposed_with_a_site_and_an_anchor` — iterates the pin array, asserts every entry carries a non-empty clause id, a disposition, a site where pinned and a reason where excluded, and that no id appears twice; `…::tests::the_pin_holds_no_written_count` — reads the module as text through `read(&workspace_root()?, …)` and asserts the pin's comment block contains no numeral-or-number-word count of its own entries, the trade `xtask/src/spec_trace.rs:1975-1978` already made. Reviewer check: the re-derivation record exists in this story's directory and reconciles `RUNBOOK.md:3830-3845` against `references/evaluation/phase-4-5-reconciliation.md:119-142` row by row; `rg -n "ES-23" xtask/ crates/` shows one enumeration of the set and no second. Run by `cargo test -p xtask`. |
| AC-002 | **GIVEN** a contributor who renumbers, deletes or typos a clause id the pin names, **WHEN** they run the gate, **THEN** the run fails with a problem naming **the id and the pin** — never a narrative page — because every pinned id is resolved through `crate::spec_trace::clause_ids(root)`, the resolver `spec-trace-clause-id-accessor` landed beside `all_rules` (`xtask/src/spec_trace.rs:1746`). The checker calls it **once per run** and passes the resulting set to both the citation check and the pin check: no second parser, no regex, no literal prefix list — a fourth list of the six clause families is the one that can half-land, which is what `SECTIONS`' own doc comment says (`:107-120`). Existence, not eligibility: a `[DEFERRED]` id resolves, because the pin's filter is the human classification in AC-001 and not a maturity test inside the resolver. | `xtask/src/lint_narrative.rs::tests::a_pinned_id_absent_from_the_specification_is_a_problem` — an in-memory specification slice declaring every pinned id but one; asserts one problem, that it names the missing id, that it names the pin rather than a page, and that the surviving ids produce no problem. `…::tests::the_specification_is_resolved_once_per_run` — the run threads one `BTreeSet<String>` through both checks; asserted structurally by the checks taking `&BTreeSet<String>` rather than a `&Path`, and checked at review against `spec-trace-clause-id-accessor/spec.md` EC-006. Reviewer check: the diff contains no clause-id regex and no prefix array. Run by `cargo test -p xtask`. |
| AC-003 | **GIVEN** an adapter author's discharge site that has been moved, renamed or rewritten out from under the pin — `crates/happenstance-core/src/store.rs`'s `# Cancellation` section deleted, or `tag.rs` split in two — **WHEN** the gate runs, **THEN** two *distinct* problems are possible and the message says which happened: **the path moved** (the discharge-site file could not be read, reported with `read`'s `reading {rel}` context, `xtask/src/spec_trace.rs:2307-2309`) or **the discharging text moved** (the file is there and the verbatim anchor is not). A contributor who reads "the anchor is gone" and goes looking for a missing file has been sent to the wrong place, and the anchor is the load-bearing sentence of the discharge rather than a nearby convenience — `# Cancellation` (`store.rs:146`), `at-most-once under verbatim reissue` (`store.rs:169`), `# Equality is byte equality, and nothing is normalised` (`tag.rs:29`). | `xtask/src/lint_narrative.rs::tests::a_missing_anchor_and_a_missing_site_are_two_different_problems` — one in-memory source slice with the anchor stripped and one entry naming a path that does not exist; asserts two problems, two different messages, and that neither message is reachable from the other's condition. `…::tests::the_whole_pin_holds_against_the_real_tree` — the only test that touches the real checkout, through `workspace_root()` and `read`, asserting zero problems over `crates/happenstance-core/src/{store,tag,event,append}.rs` as they stand today. Run by `cargo test -p xtask`. |
| AC-004 | **GIVEN** a contributor who re-wraps a paragraph in `crates/happenstance-core/src/store.rs` — a formatting edit that changes no meaning — **WHEN** they run the gate, **THEN** it stays green, because the anchor match strips doc-comment prefixes and collapses whitespace before comparing. This is the criterion that protects the pin from its own users: the remedy a contributor reaches for when a check fires on an innocuous edit is to edit the check, and the check *is* the pin, so a jumpy anchor teaches people to delete entries. **AND** every place this story matches a clause id matches a **whole identifier**: `ES-1` is a substring of `ES-17`, `VT-3` of `VT-33`, `ES-4` of `ES-40`, and the shorter-name-swallowed-by-the-longer defect shipped once already in this repository for conformance rule names (`standards/rust/81-checks-that-cannot-be-types.md:209-217`). | `xtask/src/lint_narrative.rs::tests::the_anchor_match_survives_a_reflowed_doc_comment` — the same anchor across a one-line form, a two-line reflowed form and a form with `///` prefixes and extra indentation; all three match. `…::tests::a_short_clause_id_does_not_match_a_longer_one` — asserts `ES-1` does not resolve against a document declaring only `ES-17`, and that a pin entry for `VT-3` is not satisfied by `VT-33`; the boundary shape is copied from `fn names_rule` (`xtask/src/lints.rs:492-500`), not shared with it (Note 8). Reviewer check: no bare `contains` on a clause id anywhere in the diff. Run by `cargo test -p xtask`. |
| AC-005 | **GIVEN** the unmerged sibling `initiative/from-contract-to-published-library`, which diverges from `spec/SPECIFICATION.md` by 521 lines and may **add** a documentation MUST, **WHEN** that branch is merged forward and the gate runs on the merge commit, **THEN** the new clause surfaces as **an unclassified candidate** — a gate failure a human disposes of in the pin — rather than waiting for the HS-P0025 closeout re-check `project.md:274-281` had to settle for. The comparison is derived-versus-hand-written: the derived side is a candidate scan of `spec/SPECIFICATION.md` over an enumerated phrase `const`, the hand side is AC-001's classified enumeration, and the failure **names which side moved** — *a candidate nobody classified* (the document grew) is a different sentence from *a pinned or excluded entry that is no longer a candidate* (the enumeration is stale), and both name the ids. Never "the counts disagree". | `xtask/src/lint_narrative.rs::tests::an_unclassified_candidate_says_the_document_grew` — an in-memory specification slice carrying one candidate phrase beyond the enumeration; asserts the message names the new id and attributes the movement to the document. `…::tests::an_entry_that_is_no_longer_a_candidate_says_the_enumeration_is_stale` — the mirror, and asserts the two messages are not the same sentence. RS-81-5's explicit requirement (`standards/rust/81-checks-that-cannot-be-types.md:335-341`) is checked at review against both messages. Run by `cargo test -p xtask`. |
| AC-006 | **GIVEN** a contributor who has broken three pinned entries in one edit, **WHEN** they run `cargo xtask narrative` or the same step inside `cargo xtask ci`, **THEN** they see **all three** problems in one run, in source order, with the count last — never the first one and a stop, because "a check that stops at the first problem turns one review cycle into six" (`_decomposition.md:218-219`) — and never a truncated list: no "… and N more" anywhere, which is `_design.md` anti-pattern 8. The pin reports **in place**, through the existing step's own `Vec<String>`, its two-space-indented stderr lines and its `bail!("{} problem(s) in {TREE}", problems.len())` — the shape at `xtask/src/lint_constitution.rs:190-199`. **No new step, banner, subcommand, `print_help` line, `REQUIRED` entry or `lint_steps` member is added**: a second banner would split one surface in two and send the reader to a second place for the same fact. On success the checker still prints **exactly one line**, unchanged, because a green check that says ten lines trains people to skip it (`_design.md`, `## Transience policy`). | `xtask/src/lint_narrative.rs::tests::every_pin_problem_is_reported_not_just_the_first` — three simultaneous defects (an unresolvable id, a missing anchor, an unclassified candidate) yield three problems in source order. `…::tests::the_pin_never_truncates_its_problem_list` — twenty simultaneous defects yield twenty lines and no elision marker. Gate observation: `cargo xtask narrative` fails with all problems and one count line, then passes printing one summary line; `cargo xtask ci --fast` shows exactly one narrative banner. Reviewer check: `xtask/src/main.rs` is outside the PR boundary and unchanged in the diff. Run by `cargo test -p xtask` plus the recorded gate output. |
| AC-007 | **GIVEN** a reviewer reading a failure in an 80-column CI log, **WHEN** a pin problem soft-wraps, **THEN** the artifact is still on the first visual row, because every problem line the pin emits is **location-first** — `{path}:{line} — {message}` where a line is meaningful and `{path} — {message}` where it is not, the form the checker already carries — and the location prefix stays inside the **≤ 48-character** budget the design derives from the 80-column line (`_design.md:419-425`). Anti-pattern 7 is *"a gate failure whose first visual row does not begin with `path:line`"* (`_design.md:571-597`). Hierarchy is carried by position and adjacency alone: no colour, no weight, no size, no box-drawing, no second indent level — the terminal primitive inventory `_design.md:31-40` verified is the whole vocabulary, and a primitive outside it is an invented one. | `xtask/src/lint_narrative.rs::tests::every_pin_problem_line_begins_with_its_artifact` — collects every problem the pin can emit (one per condition, driven from the pin array and the candidate `const` so a future condition cannot escape the test), asserts each starts with a repo-relative path, that the segment before the first `—` is ≤ 48 characters, and that no line contains an ANSI escape or a box-drawing character. Reviewer check against `_design.md:419-425` and anti-pattern 7. Run by `cargo test -p xtask`. |
| AC-008 | **GIVEN** a contributor about to trust the pin — the failure mode being that they read a green gate as proof the documentation is *correct* — **WHEN** they open `xtask/src/lint_narrative.rs`, **THEN** the **first** thing they meet is what it does not verify, before the checks and not after, because "a check whose limits are undocumented is read as a guarantee" (`xtask/src/lint_constitution.rs:9-28`; `_design.md`, `## What a user meets first`). The pin contributes three limits: an anchor can survive while the *reasoning* around it is rewritten, which is a re-discharge this check cannot detect and which `.kb/governance/rewrite-the-referent-never-the-reasoning.md` covers as a human rule; a clause wording its documentation obligation outside the enumerated candidate phrases is invisible to the derived scan; and the pin proves a discharge is **present**, never that it is **adequate**. **AND** nothing in the module, its messages or its output carries a badge, tick, shield or "verified" wording, and nothing anywhere claims the surface proves a page teaches — `_design.md` anti-pattern 9 and project DoD item 8 (`project.md:260`). | `xtask/src/lint_narrative.rs::tests::the_module_states_the_pins_limits_before_the_pin` — reads the module as text through `read(&workspace_root()?, …)` exactly as `check_harness` reads a source file (`xtask/src/lint_constitution.rs:424-425`), asserts a `does not verify` heading precedes the pin `const`'s first line and that all three limit sentences are present. `…::tests::no_pin_message_claims_a_discharge_is_correct` — asserts no emitted message or module doc line contains `verified`, `correct`, `proves`, `✓` or a badge token. Also covered by `cargo xtask lint-constitution` and `cargo test --locked -p xtask --doc` still passing (project DoD item 7), and by a prose review pass. Run by `cargo test -p xtask`. |

## Interaction quality

RFC §6.7/D6. This story renders one surface and it is a terminal one:
**`gate-narrative-checker-step`** (`_design.md:147-152`), in its `pass`, `fail-one` and `fail-many`
states. `_design.md` is signed off and binding; nothing below re-decides it. Every invariant that
applies is an **AC row above** — this section only says which row carries it and how it is verified,
because `redkiln verify` extracts ACs from the table and a bullet here would be gated by nothing.

**A note on what "presentation" means on this surface.** There is no DOM, no stylesheet and no theme:
`_design.md:23-40` verified that no `book.toml`, `*.css` or `*.scss` exists anywhere in the tree, and
the *only* place a real in-repo primitive inventory exists is the gate's terminal output. So the
composition invariants below are the terminal primitives — banner, two-space-indented problem line,
one-line success summary, terminal count, vacuity guard — and "bare markup" here means a `println!`
or a `bail!` that does not compose from that table. A check that emitted `Err(anyhow!("pin failed"))`
would satisfy every assertion about problem *content* and compose nothing.

**STATE invariants**

| Invariant | Carried by | How it is verified |
| --------- | ---------- | ------------------ |
| **In-place, not a context jump** — the pin reports inside the existing narrative step; a contributor does not learn about a broken pin from a second banner, a second subcommand or a second summary. | **AC-006** | `xtask/src/main.rs` is outside the PR boundary and unchanged in the diff; `cargo xtask ci --fast` shows one narrative banner. |
| **Non-occlusion** — no problem hides another. All problems accumulate and print; the list is unbounded and never elided. | **AC-006** | `…::tests::every_pin_problem_is_reported_not_just_the_first` and `…::tests::the_pin_never_truncates_its_problem_list`. |
| **Preserved position / no lost place** — the terminal analogue of preserved focus and scroll: the artifact is on the first visual row after soft-wrap, so a reader never has to scroll back to find out *what* the message was about. | **AC-007** | `…::tests::every_pin_problem_line_begins_with_its_artifact`, prefix ≤ 48 characters. |
| **Reversibility** — the run mutates nothing. Five artifacts are read and none is written; no `spec-trace --write`, no generated region touched, so a failed pin leaves the tree byte-identical and the fix is the contributor's own edit. | **AC-001** (read-only enumeration), **AC-003** (site reads through `read`) | `cargo xtask spec-trace` prints what it printed before the commit; `git status` clean after a failing run, recorded in the implementation report. |
| **Reachability** — the keyboard-reachability analogue: the surface is reachable from every wired invocation path, not only the one the author typed. `cargo xtask narrative` standalone, the same step inside `cargo xtask ci` and `cargo xtask ci --fast` (`probe: None`, so `--fast` cannot filter it), and `cargo xtask affected --base main` on a prose-only diff. | **AC-006** | Gate observation on all four paths, recorded verbatim; `.redkiln/config.yaml:40`, `:55`. |

**COMPOSITION invariants**, from the signed-off `_design.md`

| Invariant | Carried by | How it is verified |
| --------- | ---------- | ------------------ |
| **Presentation exists at all** — every problem is a composed line from the verified primitive table (`_design.md:31-40`): two-space-indented `{path} — {message}` on stderr, the terminal count last via `bail!`. Not a bare `anyhow!` string, and not a `Debug` dump of the pin entry. | **AC-006**, **AC-007** | The two tests above, plus the recorded failing-run output. |
| **Composition / placement** — the pin's problems occupy the same region as the checker's other problems, in source order, under the one banner the step already prints. No new region. | **AC-006** | `…::tests::every_pin_problem_is_reported_not_just_the_first` asserts source order. |
| **Transience** — problem lines are **opened on demand, by failing**; they exist only in the failure state. The success summary is **persistent chrome, exactly one line**, and this story does not add a second (`_design.md`, `## Transience policy`). The step banner is persistent and inherited. | **AC-006** | Green run prints exactly one summary line; asserted by gate observation and reviewer diff check. |
| **Density budget, with its real numbers** — 80-column line width; location prefix **≤ 48 characters** including `:{line}`; problem list **unbounded, never truncated**; success output **1 line** (`_design.md:419-425`). | **AC-006** (list, success line), **AC-007** (line width, prefix) | `…::tests::every_pin_problem_line_begins_with_its_artifact` measures the prefix; the truncation test measures the list. |
| **Hierarchy** — carried by position, adjacency and nothing else. No colour, weight, size, ANSI escape, box-drawing or second indent level; the artifact is primary by being first, the count is terminal by being last. | **AC-007** | The same test asserts no ANSI escape and no box-drawing character. |
| **Limits first** — `_design.md`'s `## What a user meets first`: the contributor's front door is the "What this does not verify" section, and it is *first*. | **AC-008** | `…::tests::the_module_states_the_pins_limits_before_the_pin`. |
| **Anti-pattern 7** — no gate failure whose first visual row does not begin with `path:line`. | **AC-007** | As above. |
| **Anti-pattern 8** — no truncated problem list, no "… and N more". | **AC-006** | The truncation test. |
| **Anti-pattern 9** — no badge, tick, shield or "verified" mark asserting the documentation is checked for correctness or comprehension. | **AC-008** | `…::tests::no_pin_message_claims_a_discharge_is_correct`, plus the prose review pass. |
| **No new state id on the surface** — the pin's failures are instances of the problem-line form `_design.md:555-569` already enumerates (*pinned MUST no longer at its discharge site*; *count disagreement, derived vs. hand-written, RS-81-5*). Extending the signed-off state list would be re-deciding the design. | **AC-003**, **AC-005** | Reviewer check against `_design.md:147-152` and `:555-569`. |

**Not applicable, and why.** No markdown surface is rendered or changed by this story, so
`narrative-tree-index`, `narrative-page` and `narrative-scoped-page` and their budgets (70/100
columns, ≤ 40-character H1, ≤ 250-line page, the inline scope band) are untouched — they belong to
`pinned-narrative-tree-and-compiling-step` and to HS-P0021–23. `rustdoc-reference-surface` is
`unchanged` by design. Anti-patterns 1–6 and 10–11 govern pages and directories this story does not
write; they are not silently dropped, they are simply not this story's to violate.

## Error conditions

| id | condition | required behaviour |
| -- | --------- | ------------------ |
| EC-001 | `spec/SPECIFICATION.md` is missing, truncated or unreadable | Inherited from the foundation, not re-implemented: `clause_ids` returns `Err` naming the artifact it could not read, and the run fails. **Never** `Ok` with an empty set — that would report every pinned id as unresolvable and every real MUST as defective (`spec-trace-clause-id-accessor/spec.md` AC-004). The pin adds no fallback, no `unwrap_or_default` and no "skip if absent" arm. |
| EC-002 | a discharge-site file is missing, moved or unreadable | A **problem**, not a panic, carrying `read`'s `reading {rel}` context (`xtask/src/spec_trace.rs:2307-2309`) and worded so the reader knows the *path* moved. Distinct from EC-003 (AC-003). |
| EC-003 | the file is present and the verbatim anchor is not | A **problem** worded so the reader knows the *discharging text* moved. This is the condition project AC-008 exists for, and the one that fires when a doc-comment rewrite un-discharges a `[FROZEN]` clause. |
| EC-004 | the pin `const` is empty | `bail!` **before any check runs**, in the vacuity-guard shape at `xtask/src/lint_constitution.rs:175-177`. A pin that enumerates nothing must not pass; an empty enumeration is the single most plausible way this whole story becomes decorative. |
| EC-005 | the pin names the same clause id twice | Hard error. A duplicate would let a dedup silently absorb one entry and make AC-005's comparison lie about which side moved — the same class of arithmetic inflation RS-81-3 records (`standards/rust/81-checks-that-cannot-be-types.md:209-217`). |
| EC-006 | an anchor is set equal to its entry's clause id | Rejected. Verified on `main`: `ES-23`, `ES-24`, `VT-15`, `VT-3`, `ES-17` and `ES-40` occur **zero** times in the files the evidence table names as their discharge sites, so an id-as-anchor pin fails on every entry today — and the "fix" a contributor would reach for is widening the match until it passes, which empties the check. A test asserts no anchor equals its id. |
| EC-007 | a candidate the derived scan finds is on neither the pinned nor the excluded list | A **problem** naming the id and attributing the movement to the document (AC-005). This is the arm that turns the sibling branch's residual risk into a gate failure on the merge-forward commit. |
| EC-008 | a pinned or excluded entry is no longer a candidate at all | A **problem** naming the id and attributing the movement to the enumeration. Same check, opposite direction, different sentence — never one message for two unrelated bugs (RS-81-5). |
| EC-009 | `clause_ids` still carries its self-erasing `#[expect(dead_code, reason = …)]` when this story calls it | Not a runtime error — a **build** failure, and deliberately so: an unfulfilled expectation is a warning and the gate is `-D warnings`. Deleting the attribute here is in-boundary and required if `narrative-citation-resolution` has not already removed it (`spec-trace-clause-id-accessor/spec.md` AC-006). `#[allow(dead_code)]` must not appear as the remedy. |
| EC-010 | a clause states its documentation obligation in wording no candidate phrase matches | **Undetectable, and documented rather than silently absorbed** — AC-008's second limit. Stated in the module's "What this does not verify" section next to the candidate-phrase `const`, so the next reader meets the limit where they meet the mechanism. |

## Non-functional

| id | requirement | basis |
| -- | ----------- | ----- |
| NF-001 | **One read and one parse of `spec/SPECIFICATION.md` per gate run.** The document is 9,070 lines / ~567 KB; the checker calls `clause_ids` once and shares the set with the citation check. Two calls inside one step buy no new information. | `spec-trace-clause-id-accessor/spec.md` EC-006, NF-001 |
| NF-002 | **Four additional `read_to_string` calls**, one per discharge-site file, plus one line scan of the specification for candidates. The cost sits inside the time cargo already takes to decide `xtask` is up to date — the class `_design.md`'s `## What it costs a caller` already argued for the file-reading checks. | `_design.md:516-537` |
| NF-003 | **Zero new dependencies in `xtask/Cargo.toml`.** Every check is `read_to_string` plus string matching over `std` and `anyhow`. DR-12's standing trade keeps `rusqlite` and `sqlx` out of the dev graph precisely because every `cargo xtask ci` would build them. | `_design.md:520-525`; `xtask/Cargo.toml:16-21` |
| NF-004 | **No `unwrap` or `expect` outside `#[cfg(test)]`**, and the scoped `#![allow(clippy::unwrap_used, reason = "test code, per the house style")]` inside the test module in the house shape. Clippy runs with `-D warnings`. | `xtask/src/lint_constitution.rs:827-830`; CLAUDE.md, "Commands" |
| NF-005 | **MSRV, wasm32 and features unaffected.** No `cfg`, no feature, no target, no port, no `Send` bound; `xtask` is `publish = false`, so nothing here enters anyone's dependency graph and ADR-0001/ADR-0003 are untouched by construction. | `_design.md:506-515`, `:534-537` |
| NF-006 | **False-positive budget: zero, and treated at the same severity as a miss.** A check that fires on a reflow or an unrelated edit teaches contributors to delete pin entries, which removes the only instrument these clauses have. AC-004 is the mechanism; this is the standard it is held to. | Context pack, "A false positive here is worse than in any other check in this project" |
| NF-007 | **Terminal budget honoured**: 80-column line width, location prefix ≤ 48 characters including `:{line}`, success output one line, problem list unbounded. | `_design.md:419-425` |
| NF-008 | **`cargo xtask spec-trace` prints exactly what it printed before this commit.** Project AC-008 requires it explicitly, and `run` in `xtask/src/spec_trace.rs` is outside the diff. | `project.md:223-226`; `xtask/src/spec_trace.rs:31-38` |

## Implementation notes (non-prescriptive)

Shape suggestions, not requirements. The decisions above are binding; these are not.

- **The `const`'s row shape is the first thing to settle**, because every check reads it. Something
  in the family of `(&str /* clause id */, Disposition, &str /* site or reason */, &str /* anchor
  */)` works, and so does a small private struct with named fields, which reads better at seven
  entries plus ten exclusions and costs nothing in a `publish = false` crate. Both in-repo shapes
  worth copying are already cited: `RULE_FILES` / `WIRE_TESTS` (`xtask/src/spec_trace.rs:85-105`) for
  the tuple form and `UNCLAIMED_PENDING_ADR` (`:1960-2010`) for the *comment-per-entry* discipline —
  the decision travels with the row.
- **Do the re-derivation before writing a line of code.** It is evidence-gathering, not a test
  (`_decomposition.md:617-643`). Read the ten `**Rule:** none` clauses, apply the AC-001 rule, and
  write the disposition down as you go; the `const` is a transcription of that record, and if the two
  disagree the record wins.
- **Normalisation for the anchor match** is one small private helper: strip a leading `///`, `//!` or
  `*`, trim, and collapse runs of ASCII whitespace to a single space, on both haystack and needle.
  Doing it on both sides is what makes the anchor in the `const` writable as a single readable
  string.
- **The candidate scan** is a line scan of the specification for an enumerated phrase `const` —
  `MUST document`, `MUST state`, `MUST say`, and whatever the re-derivation shows is actually used.
  Keep the list short and keep its limit sentence next to it (EC-010); a long list of near-synonyms
  buys recall you cannot verify and hides the limit.
- **`fn names_rule` is copied, not shared** (`xtask/src/lints.rs:492-500`; Note 8, and RS-81-3's own
  evidence line). Two twelve-line boundary tests in two modules is cheaper than a shared helper that
  makes two checkers move together.
- **Test data is `&str` slices**, not files: a specification slice, a source-file slice, a pin array.
  Only `…::tests::the_whole_pin_holds_against_the_real_tree` touches the real checkout. Do not build a
  parallel fixture `SPECIFICATION.md` — `spec_trace.rs`'s own suite already proves the parser against
  the real document (`_decomposition.md:704-720`).
- **Write the three limits into the module docs before the checks compile**, not after. RS-81-1's
  order is prove the blind spot in the tests, then state it in the docs
  (`standards/rust/81-checks-that-cannot-be-types.md:11-94`); the section that goes last is the one
  that goes missing.
- **If a pinned entry will not hold** — if the re-derivation shows a clause whose discharge cannot be
  anchored without editing a `happenstance-core` doc comment — **do not edit the comment**. Record it
  as an excluded entry with that reason and hand it to HS-P0023 under
  `.kb/governance/rewrite-the-referent-never-the-reasoning.md`. Editing the referent is HS-P0023's
  boundary, not this story's.

## Tests and CI (merge gate)

Grounded in the project testing brief's AC-008 bullet (`_decomposition.md:617-643`), its "Fixtures
and seams to mock" note (`:704-720`) and its merge-gate command list (`:679-700`), narrowest to
widest.

| tier | command / path | proves |
| ---- | -------------- | ------ |
| unit — pin integrity | `cargo test -p xtask` → `xtask/src/lint_narrative.rs::tests::every_pin_entry_is_disposed_with_a_site_and_an_anchor`, `::the_pin_holds_no_written_count` | AC-001: every candidate is disposed, no duplicate id, no count in a comment. |
| unit — assertion 1 | `cargo test -p xtask` → `::a_pinned_id_absent_from_the_specification_is_a_problem`, `::the_specification_is_resolved_once_per_run` | AC-002: ids resolve through `clause_ids`, once per run, naming the pin not a page. |
| unit — assertion 2 | `cargo test -p xtask` → `::a_missing_anchor_and_a_missing_site_are_two_different_problems`, `::the_whole_pin_holds_against_the_real_tree` | AC-003, EC-002, EC-003: two distinct problems; the pin is green over the tree as it stands. |
| unit — the nesting defect and the reflow trap | `cargo test -p xtask` → `::a_short_clause_id_does_not_match_a_longer_one`, `::the_anchor_match_survives_a_reflowed_doc_comment` | AC-004, NF-006: the named wrong implementation (`contains("ES-1")`) is rejected, and a formatting edit is not a gate failure. |
| unit — assertion 3 | `cargo test -p xtask` → `::an_unclassified_candidate_says_the_document_grew`, `::an_entry_that_is_no_longer_a_candidate_says_the_enumeration_is_stale` | AC-005, EC-007, EC-008: the failure names which side moved, in two different sentences. |
| unit — reporting shape | `cargo test -p xtask` → `::every_pin_problem_is_reported_not_just_the_first`, `::the_pin_never_truncates_its_problem_list`, `::every_pin_problem_line_begins_with_its_artifact` | AC-006, AC-007: accumulate-all, source order, no elision, location-first inside the 48-character prefix, no ANSI or box-drawing. |
| unit — vacuity and honesty | `cargo test -p xtask` → `::an_empty_pin_is_a_hard_error`, `::the_module_states_the_pins_limits_before_the_pin`, `::no_pin_message_claims_a_discharge_is_correct` | EC-004, AC-008: an empty pin cannot pass; limits come first; nothing claims correctness or teachability. |
| doc | `cargo test --locked -p xtask --doc` | Project DoD item 7: the new module docs render and any example in them compiles; the narrative harness is unaffected. |
| lint — no regression in the neighbour | `cargo xtask lint-constitution` | Note 8: this story copied `names_rule`'s shape rather than refactoring the existing checkers to share it, and the existing corpus check is untouched. |
| gate — the step itself | `cargo xtask narrative` | AC-006: fails with every problem and one count line; passes printing exactly one summary line. |
| gate — the specification's own step | `cargo xtask spec-trace` | NF-008 and project AC-008: prints what it printed before this commit; nothing was written. |
| gate — story grain | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | AC-006 reachability: a prose-only diff still runs the checker, so the pin is not invisible to the grain every downstream story in this initiative is gated by. |
| gate — integration, non-terminal | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | The merge bar for this project: the step is `probe: None`, so `--fast` runs it unfiltered, and one banner appears. |
| gate — full | `cargo xtask ci` | The gate of record before the project is called done (`project.md` DoD 6). |
| evidence — not a test | this story's re-derivation record | AC-001: the nine-versus-eight arithmetic closed against `RUNBOOK.md:3830-3845` and `references/evaluation/phase-4-5-reconciliation.md:119-142`. Evidence-gathering, deliberately not `#[test]` code. |
| review — prose | the diff | AC-008 / project DoD item 8: no badge, tick or "verified" wording; nothing asserts the surface proves a page teaches. |

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | mitigation in this PR |
| ---- | ------------------- | --------------------- |
| **The pin becomes decorative** — an empty or near-empty `const`, or entries whose anchors are so short they match anything. | Medium / **High**. This is the failure that leaves the clauses exactly as unobserved as before while looking solved. | EC-004's vacuity `bail!`; AC-001's per-entry disposition test; EC-006's no-anchor-equals-its-id rule; three named wrong implementations the tests reject. |
| **A false positive teaches people to delete entries.** | Medium / **High**, and worse here than anywhere else in the project, because the remedy is to edit the check and the check *is* the pin. | AC-004's reflow-insensitive match, NF-006's zero-false-positive standard, and anchors chosen as the load-bearing sentence rather than a nearby convenience. |
| **The nesting defect** — `contains("ES-1")` passing on `ES-17`. | Medium / High. It shipped once already in this repository for rule names. | AC-004's boundary test in both directions; `fn names_rule` copied from `xtask/src/lints.rs:492-500`; a reviewer check that no bare `contains` on a clause id survives the diff. |
| **Coupling to `spec-trace-clause-id-accessor`.** If `clause_ids` lands with a different signature, or is routed through `has_suite`, this story's assertion 1 changes meaning: a `[DEFERRED]` id would stop resolving and the pin would report a real MUST as missing. | Low / High | The blocks-on edge below, and the foundation's own AC-003 which asserts `has_suite` is not on the path. Slice-mates land in one context, foundation first (`_storymap.md:157-164`). |
| **Coupling to `narrative-checker-mounted-with-pinned-path`.** The mount point, the problem `Vec`, the success line and the `bail!` are that story's. If its success line or its `TREE` constant changes shape, AC-006's assertions move with it. | Low / Medium | This story appends to that shape and changes neither; the success line stays one line and extending its wording is explicitly the other story's decision. Both are in the same slice. |
| **The unmerged sibling branch adds a documentation MUST.** `initiative/from-contract-to-published-library` diverges from `spec/SPECIFICATION.md` by 521 lines. | Medium / Medium | Turned from a closeout re-check into a gate failure by AC-005's candidate scan. Ids are stable names and are never renumbered (`spec/SPECIFICATION.md:280`), so the divergence cannot invalidate an entry — only add to it. |
| **Scope drift into a `happenstance-core` doc comment.** The most natural "fix" for an unanchored entry is to add a clause id to the comment. | Medium / High — it would be a touch of a discharging comment outside this story's boundary. | The PR boundary excludes `crates/**` entirely; the implementation note says to record the entry as excluded and hand it to HS-P0023 under `.kb/governance/rewrite-the-referent-never-the-reasoning.md`. |
| **Scope drift into a second step or a second `clause_ids` call.** | Low / Medium | `xtask/src/main.rs` is outside the PR boundary; `xtask/src/spec_trace.rs` is in-boundary for exactly one permitted edit (the `#[expect(dead_code)]` deletion, EC-009). |
| **Reading a green pin as evidence the documentation teaches.** | Medium / Medium — it is the initiative's top-ranked risk in miniature. | AC-008: limits first, three of them, and no badge, tick or "verified" wording anywhere. |

## Dependencies

**Blocks on** — both must land before this story, and both are slice-mates implemented in the same
context:

- `spec-trace-clause-id-accessor` — the resolver. `pub(crate) fn clause_ids(root: &Path) ->
  Result<BTreeSet<String>>` beside `all_rules` (`xtask/src/spec_trace.rs:1746`). Assertion 1 has
  nothing to call without it, and the alternative — a local prefix list or regex — is the fourth
  family list `SECTIONS`' own doc comment argues against (`:107-120`). **Foundation, first in the
  milestone** (`_storymap.md:157-164`).
- `narrative-checker-mounted-with-pinned-path` — the mount. It lands `xtask/src/lint_narrative.rs`,
  its five sites in `xtask/src/main.rs`, and the `REQUIRED`, `probe: None` step *every narrative page
  is checked*. Without it there is no `run` to add a check to, no problem `Vec`, no `bail!` and no
  step for `--fast` and `affected` to reach — a pin called by nothing but a test is the decorative
  check this project exists to refuse.

**Unlocks:**

- `documented-blind-spots-and-their-proofs` — names this story explicitly in its `depends_on`
  (`_storymap.md:59`). It owns the *contents* of the six-item limits section, and cannot be complete
  until every check that has a limit exists; this story contributes three of them (AC-008).
- **HS-P0023 `reach-and-adapter-path`** — the project sequenced behind this story at the initiative
  level. The pin must be in place *before* any project rewrites a `happenstance-core` doc comment,
  which is why this lands mid-project rather than at closeout (`_storymap.md:157-164`;
  `project.md:175-180`, DR-09).
- Initiative DoD scenario **11** and the closing criterion at `initiative.md:662-663` move from
  assertion to check.

**Independent of:** `narrative-citation-resolution` (the other consumer of the same resolver —
same slice, no edge between them), and the whole `falsification-and-limits` milestone except the
edge named above.

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than dropped. Every path below exists; open it at the stated
moment, and do not paste it in bulk.

| anchor | why it is load-bearing | when to open | serves |
| ------ | ---------------------- | ------------ | ------ |
| `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` (Note 7, `:275-300`; Note 6, `:250-274`; Note 8) | The architecture brief that fixes the pin's shape, its "re-derived, not copied" requirement, and the copy-don't-share rule for `names_rule`. The single sentence *"a `const` whose comment says 'nine, per RUNBOOK' while holding eight entries is the defect BR-10 exists to prevent"* is the story's design in one line. | **Before writing the `const`** — first thing, before the re-derivation. | AC-001, AC-004 |
| `references/evaluation/phase-4-5-reconciliation.md:119-142` | The eight-row evidence table the set is re-derived *against*. Two of its rows are comment repairs rather than clause discharges, which is where the nine-versus-eight arithmetic breaks. Cannot be summarised — the disposition is per row. | **During the re-derivation, before AC-001's `const` exists.** | AC-001 |
| `RUNBOOK.md:3830-3845` | The competing claim: "discharged nine documentation MUSTs". Read beside the evidence table, not instead of it; the record must name which of the two moved. | **During the re-derivation**, immediately after the evidence table. | AC-001 |
| `spec/SPECIFICATION.md` (the ten `**Rule:** none` clauses; ES-23 at `:3574-3603`; VT-21 at `:1483-1488`; CF-40 at `:7661-7665`; the id-stability sentence at `:280`) | The document the pin is derived from and resolved against. ES-23 states in its own words why no conformance rule can observe it; VT-21 and CF-40 are the adapter- and fixture-facing obligations the derivation rule *excludes*, and reading them is how the rule's second half stops being a preference. 566 KB — never paste, always cite by clause. | **While classifying candidates** (AC-001), and again when writing the candidate-phrase `const` (AC-005). | AC-001, AC-002, AC-005 |
| `crates/happenstance-core/src/store.rs`, `tag.rs`, `event.rs`, `append.rs` | The discharge sites, read-only. The anchors are chosen from these files and nowhere else, and *no site names its clause id* — that verified fact is why the anchor is a phrase. Open them to pick the load-bearing sentence, not a nearby convenience. | **When writing each pinned entry's anchor** (AC-001), and before EC-006's no-id-as-anchor rule can be honoured. | AC-001, AC-003 |
| `xtask/src/spec_trace.rs` (`RULE_FILES` / `WIRE_TESTS` at `:85-105`; `UNCLAIMED_PENDING_ADR` at `:1960-2010`; `clause_ids` beside `all_rules` at `:1746`; `read` / `workspace_root` at `:2307-2316`; `SECTIONS` at `:107-120`) | Four things at once: the two in-repo shapes for a heavily commented enumerated `const` that carries a decision per row, the resolver this story calls, the file-reading helpers every check in this crate uses, and the doc comment explaining why a fourth family list loses. | **Before writing the `const`** (shapes), and **before assertion 1** (the resolver and helpers). | AC-001, AC-002, AC-003 |
| `xtask/src/lints.rs:486-500` (`fn names_rule` and its doc comment) | The whole-identifier matcher to copy, and the transcript of the defect that made it necessary — a longer rule name silently discharging a shorter one's obligation and inflating the arithmetic built on the same match. | **Before implementing any clause-id match** (AC-004), which is before assertion 1 compiles. | AC-004 |
| `xtask/src/lint_constitution.rs` (`:9-28` limits-first; `:175-177` vacuity guard; `:190-199` accumulate-and-count; `:423-425` reading a source file as text; `:827-841` the test-module shape) | The house shape for every structural decision this story does not get to re-decide: where the limits go, how an empty corpus fails, how problems accumulate and the count comes last, how a check reads a source file, and how the test module opens. | **Before wiring the checks into `run`** (AC-006), and again when writing the test module. | AC-006, AC-007, AC-008, EC-004 |
| `standards/rust/81-checks-that-cannot-be-types.md` (RS-81-1 at `:11-94`; RS-81-3 at `:209-262`; RS-81-5 at `:325-341`) | The three constitution rules this story is built on: prove the blind spot then state it; the nesting defect with its evidence; and *"make the failure say which one moved"* — RS-81-5 is assertion 3's whole specification. | **RS-81-3 before AC-004; RS-81-5 before AC-005; RS-81-1 before AC-008's limits section.** | AC-004, AC-005, AC-008 |
| `.bklg/docs-that-teach/checked-documentation-surface/_design.md` (`:147-152`; `:354-375`; `:419-425`; `:538-569`; `:571-597`) | The signed-off, binding surface: the checker step and its states, the transience policy (problem lines opened by failing; success one line), the terminal density budget with its real numbers, what a contributor meets first, and the eleven anti-patterns. This story renders that surface and does not re-decide it. | **Before emitting a single problem message** (AC-006, AC-007) and **before writing the module docs** (AC-008). | AC-006, AC-007, AC-008 |
| `.bklg/docs-that-teach/checked-documentation-surface/spec-trace-clause-id-accessor/spec.md` (AC-001, AC-003, AC-004, AC-006; EC-006) | The foundation's contract as its own spec states it: the signature, existence-not-eligibility, the hard-error posture on an unreadable document, the one-call-per-run rule, and the self-erasing `#[expect(dead_code)]` this story may have to delete. | **Before calling `clause_ids`** (AC-002) and when EC-009 fires at the first build. | AC-002, EC-009 |
| `.bklg/docs-that-teach/checked-documentation-surface/narrative-checker-mounted-with-pinned-path/spec.md` | The mount's contract: the module, the five mount sites, the `REQUIRED` `probe: None` step, the problem `Vec` and the one-line success summary this story appends to and must not reshape. | **Before adding the check to `run`** (AC-006) — it defines what "the existing shape" is. | AC-006 |
| `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` (testing brief AC-008 bullet, `:617-643`; "Fixtures and seams to mock", `:704-720`) | The three assertions stated as three tests, and the explicit instruction *not* to build a parallel fixture `SPECIFICATION.md` — plus the statement that the re-derivation is evidence, not `#[test]` code. | **Before writing the test module** (AC-002, AC-003, AC-005). | AC-002, AC-003, AC-005 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The governance atom that decides what happens when an anchor will not hold: rewriting the referent is permitted, rewriting the reasoning is not — and either way the edit is HS-P0023's, not this story's. | **The moment a pinned entry cannot be anchored without editing a doc comment.** | AC-001, AC-008 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | The audience the ACs are framed from — the adapter author on *"walk the adapter path, not just the recipe"*, carried here because the durable product layer holds no persona atom yet (`initiative.md:275-283`). Read it if an AC's user intent needs re-grounding, not to re-derive it. | **Only if an AC's framing is challenged at review.** | AC-001, AC-008 |
| `.redkiln/config.yaml:35-58` | The `verify:` block that wires `cargo xtask affected --base main` as the story grain and `cargo xtask ci --fast` as the non-terminal integration bar — the two gate paths AC-006's reachability invariant is measured on. | **Before recording gate evidence** for AC-006. | AC-006 |

## Clarifications resolved during spec

1. **The AC set is exactly the eight the front half enumerated.** AC-001 through AC-008, none added
   and none dropped. The mapping is: AC-001 the re-derivation and the enumeration, AC-002 assertion
   1, AC-003 assertion 2, AC-004 the two matching disciplines that keep assertion 2 honest, AC-005
   assertion 3, AC-006 and AC-007 the composition invariants of the surface this story renders, and
   AC-008 the limits-first and no-teachability-claim obligations. All eight trace to project AC-008;
   the ledger carries the same eight ids.

2. **The story map's row says "each site still contains its id"; this spec says "its anchor", and
   that is a correction, not a re-scoping.** Verified on `main`: `ES-23`, `ES-24`, `VT-15`, `VT-3`,
   `ES-17` and `ES-40` occur **zero** times in the files the evidence table names as their discharge
   sites. A check written to the story map's literal wording would fail on every entry on the day it
   landed, and the only way to make it pass would be to add clause ids to `happenstance-core` doc
   comments — a touch of a discharging comment, which is HS-P0023's boundary and is governed by
   `.kb/governance/rewrite-the-referent-never-the-reasoning.md`. The anchor is a verbatim phrase per
   entry, and EC-006 makes an id-as-anchor a rejected implementation rather than a tempting one.

3. **"Derived count" is resolved as a derived *set*, compared to the hand-written classification.**
   The `const`'s own length is not a derived fact, and a count written into its comment is precisely
   the trade `UNCLAIMED_PENDING_ADR` already refused (`xtask/src/spec_trace.rs:1975-1978`). So the
   derived side is a candidate scan of the specification and the failure names which side moved
   (AC-005, RS-81-5) — which is also what converts the sibling branch's residual risk from a
   closeout re-check into a gate failure on the merge-forward commit.

4. **The pinned set is stated as eight ids and is *not* frozen by this spec.** ES-19, ES-23, ES-24,
   VT-3, VT-15, VT-17, ES-17 and ES-40 is what the re-derivation is expected to confirm, and the ten
   further candidates (VT-13, VT-32, VT-33, VT-21, VT-22, VT-24, ES-35, PS-36, CF-39, CF-40) are
   expected to be excluded with reasons. If the re-derivation disagrees, **the re-derivation wins** and
   the record says so — that is the difference between deriving and copying, and AC-001 is written so
   the artifact is the disposition rather than the number.

5. **No new state id is added to `gate-narrative-checker-step`.** Both of the pin's failure shapes
   are already in `_design.md`'s `## The states the API must express` (`:555-569`) — *pinned MUST no
   longer at its discharge site* and *count disagreement (derived vs. hand-written, RS-81-5)*. The
   design designed them in; this story implements them and does not extend the signed-off list.

6. **The pin `const` is not in `_design.md`'s `## Items` block, and it is not added to it.** None of
   the five rows there belongs to this story. The design nonetheless governs the new `const`: both of
   its failure states are enumerated, its visibility answer is the same one the visibility table gives
   every private `const` in the module (private, default feature, no semver promise, `xtask` is
   `publish = false`), and its shape is fixed by the architecture brief. Saying so here is deliberate —
   quietly extending a signed-off list is how a design record stops being one.

7. **`xtask/src/spec_trace.rs` is inside the PR boundary for exactly one edit**: deleting the
   self-erasing `#[expect(dead_code, reason = …)]` on `clause_ids` if `narrative-citation-resolution`
   has not already removed it (EC-009). The attribute is designed to erase itself on the first
   consumer, an unfulfilled expectation is a warning, and the gate is `-D warnings` — so this is a
   build failure the implementer *must* fix, and `#[allow(dead_code)]` is not the fix.

8. **No ADR is owed and none is written here.** No `[FROZEN]` clause is changed, no port moves, and
   the grounding pass found no Accepted decision atom under `.kb/decisions/` governing gate structure
   or documentation checks (`_grounding.md`, "Precedence and non-goals"; `_storymap.md:93-99`). The
   binding authority for this story is the project's signed-off `_design.md`, the architecture and
   testing briefs, `standards/rust/81-checks-that-cannot-be-types.md`, and
   `.kb/governance/rewrite-the-referent-never-the-reasoning.md`.
