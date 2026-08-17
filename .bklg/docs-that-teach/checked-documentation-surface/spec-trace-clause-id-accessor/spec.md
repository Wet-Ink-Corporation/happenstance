---
item: HS-S0141
stage: spec
created: 2026-08-17T13:16:03.971Z
updated: 2026-08-17T13:16:03.971Z
template_sig: 87bbf1d0
rendered_sig: 60b95f94
---

# Spec — One resolver for specification clause ids, next to the parser that owns them

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` — BR-09/BR-10, DoD 11 and DoD 12 |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` — the DAG, and the sibling-branch gate decision |
| Project | `.bklg/docs-that-teach/checked-documentation-surface/project.md` — AC-007, AC-008, DR-07, DR-09 |
| This spec | `.bklg/docs-that-teach/checked-documentation-surface/spec-trace-clause-id-accessor/spec.md` |
| Key brief — architecture | `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` Note 6 (`:250-274`) and Note 7 (`:275-300`) |
| Key brief — testing | `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md`, AC-007 and AC-008 bullets (`:617-643`) and "Fixtures and seams to mock" (`:704-720`) |
| Signed-off design (BINDING) | `.bklg/docs-that-teach/checked-documentation-surface/_design.md` — `## Items` (`:45-57`), `## Signatures` (`:83-90`), `## Placement and re-export` (`:485-503`), `## Visibility and stability` (`:506-514`) |
| Roadmap pointer | `.bklg/docs-that-teach/checked-documentation-surface/_storymap.md` — `specification-pin` row (`:52`), merge order item 3.6 (`:157-164`) |

## One-line PR slice

Add `pub(crate) fn clause_ids(root: &Path) -> Result<BTreeSet<String>>` beside `all_rules` in
`xtask/src/spec_trace.rs`, built from the existing `parse_clauses` and the existing `SECTIONS`
family list, so the two consumers in this slice resolve clause ids through the parser instead of a
fourth prefix list that will not agree.

## Executive summary

This PR lands one crate-internal function and the evidence that it reads the specification the way
`spec-trace` already does. It is a **foundation** story: it adds no gate step, no subcommand and no
`REQUIRED` entry, and it is proven by the two capability stories that consume it in the same slice.

The delta against what exists today is small and precisely placed. `spec_trace` already owns the
only parser of `spec/SPECIFICATION.md` in the repository, and it already has one sibling accessor —
`all_rules` (`xtask/src/spec_trace.rs:1739-1752`) — whose stated reason for existing is that "a list
of files kept next to the function that parses them cannot drift from it" (`:83-84`). Every fact
this story needs is already parsed: `parse_clauses` (`:1357-1388`) yields a `Clause` per declaration
and `clause_id` (`:1391-1430`) takes its accepted prefixes from `SECTIONS` (`:122-153`). Nothing
outside that module can see any of it, because `parse_clauses`, `clause_id` and `struct Clause`
(`:217-238`) are all private.

So the work is: expose the one narrow fact — *the set of clause ids this document declares* — in the
shape the house already uses for narrow facts, with the two hard-error postures the module already
takes, and with the limits stated in the item's own documentation. What this PR deliberately does
**not** do is change `run`, touch the generated §7.1–§7.2 region, or add a dependency.

## Context pack

**The decision this story exists to make impossible.** Both consumers in this slice need to answer
"does clause id `X` exist in `spec/SPECIFICATION.md`". The cheap answer is a regex and a literal
prefix list in the new checker. That would be the **fourth** list of the six clause families, and
`SECTIONS`' own doc comment already says why the fourth loses: it is "the single place the six clause
families are enumerated: the parser takes its accepted prefixes from here, the census takes its rows,
and §7.2 takes its subsection headings. Three lists that must agree, kept as one so that adding a
family cannot half-land" (`xtask/src/spec_trace.rs:107-120`). A fourth list adds a family that
half-lands: the narrative checker would report every id of the new family as dangling while
`spec-trace` counted it happily. **Resolve through the parser. Do not copy a regex**
(`_decomposition.md:250-258`).

**The shape is already chosen, twice over.** `all_rules(root) -> Result<BTreeSet<String>>` reads the
files and `collect_rules(suite: &str) -> BTreeSet<String>` decides from the text (`:1746-1768`). That
split is not decoration — it is what lets the decision be asserted against an in-memory string with
no filesystem and no fixture tree. `clause_ids` takes the same split, and the design has already
signed off its outer half verbatim, `pub(crate)`, in this file (`_design.md:85-90`, `:499-501`,
`:510`). This story implements that signature; it does not re-decide it.

**Hard-error, never a quiet empty — and the message must blame the right artifact.** Two postures
apply and they come from different places. First, an unreadable document is a failure: `read`
attaches `reading {rel}` context (`:2307-2309`) and `all_rules`' own `# Errors` section says a
missing file "is a hard failure rather than an empty contribution: silently skipping one is how a
check comes to scan two files while reporting on three" (`:1741-1745`). Second, and less obvious: a
document that *parses to nothing* must also fail, **in this function**, not in its callers. If
`clause_ids` returned an empty set, both consumers would still fail loudly — but they would fail by
naming the pages and the pinned ids, reporting the *document* as broken when the *checker* is broken.
`wire_rules` already refuses exactly that trade in the same file: it bails because otherwise "every
`wire::` name a clause cites would report as missing, which is the checker being broken rather than
the document" (`:1784-1791`), and `run` bails the same way at `:616-620`. Take that posture. The
architecture brief transfers the same reasoning from `lint_constitution.rs:29-44` — "a citation this
parser declines to read is a citation nothing verifies" — and tells this story to take the
hard-error posture rather than `spec_trace::citations`' permissive skip (`_decomposition.md:267-271`).

**Ids only, and the private struct stays private.** The temptation is to expose `parse_clauses`
itself, or a `Vec<Clause>`, since a later check might want maturity. Refuse it. `Clause` carries
thirteen fields describing how the checker's own §7.2 renderer works (`:217-238`); making it visible
crate-wide would couple both consumers to the parser's internals and turn every future field
addition into a change to code that only wanted to know whether an id exists. `all_rules` is the
precedent and it exposes names, not rules. A consumer that later needs maturity earns its own narrow
accessor next to this one — that is the pattern, and it is cheap precisely because the accessors sit
beside the parser.

**Existence, not eligibility.** `clause_ids` must not filter by maturity or by family. A
`[DEFERRED]` clause is declared, so its id resolves; `has_suite` (`:1735-1737`) exists to gate a
different question — whether a rule name could be checked yet — and applying it here would silently
make `PS-` and `SY-` citations dangle. Clause ids "are stable and are never renumbered"
(`spec/SPECIFICATION.md:280`, §1.4), which is the property that makes resolution-by-name meaningful
at all, and is why this check survives the 521-line `spec/SPECIFICATION.md` divergence on the
unmerged `initiative/from-contract-to-published-library` branch (`project.md:274-281`).

**The persona-journey slice.** There is no runtime surface here. The reader of this story's output is
a **contributor running the gate**, and the design states what they meet first: the module's "What
this does not verify" section, placed *first* in the docs rather than last, because
`lint_constitution.rs:11-13`'s reason applies verbatim — a check whose limits are undocumented is
read as a guarantee (`_design.md:538-552`). This story's own limits are narrow and must be written
down anyway: `clause_ids` collapses a doubly-declared id (a `BTreeSet` cannot see the duplicate, and
nothing checks for one today), it answers existence rather than whether citing the clause is
*appropriate*, and it parses no citations — finding the ids a page cites belongs to
`narrative-citation-resolution`.

**A foundation with no caller is dead code, and the gate is `-D warnings`.** `dead_code` is warn-by-
default and `cargo xtask ci` runs clippy with `-D warnings` (CLAUDE.md, "Commands"), so a
`pub(crate) fn` with no call site fails the gate at this story's own checkpoint — and a `#[cfg(test)]`
use does not silence it. The repository has already solved this exact shape, in the honest direction:
`crates/happenstance-neon/src/event_store.rs:232-240` carries `#[expect(dead_code, reason = …)]` with
the reasoning attached — "`dead_code` fires today and stops firing on the day the decoder is written
— which is what `expect` rather than `allow` is for". Use `#[expect]`, name this slice's consumer
stories in the `reason`, and rely on the property that makes it better than `allow`: the moment
`narrative-citation-resolution` adds its call, the expectation is unfulfilled, that is itself a
warning, and `-D warnings` forces the attribute's deletion. The marker removes itself; it cannot rot
into a permanent allow.

**What must not move.** `run` (`:610-620`) stays as it is. It needs whole `Clause` values, it already
guards its own empty parse, and rewiring it through `clause_ids` would read and parse the document
twice inside one gate step while changing the sentence a green `cargo xtask spec-trace` prints. The
generated §7.1–§7.2 region is not touched, `--write` is not run, and no `SPECIFICATION.md` clause is
amended or restated — the initiative is additive and says so (`project.md:134`).

**No new dependency.** DR-12 stands: `xtask/Cargo.toml:16-21` records the standing trade that keeps
`rusqlite` and `sqlx` out of xtask's dev graph because every `cargo xtask ci` would then build them.
Nothing here needs a temp-directory crate, which is the whole point of splitting the read from the
decision — every assertion this story owes is an assertion about a `&str`.

**No conformance rule, and therefore no changelog entry.** CF-29's lint reads
`RULE_FILES` (`:85-89`) and demands a `CHANGELOG.md` sentence per *conformance rule*
(`xtask/src/lints.rs:497-536`). This story adds none, changes no port, and is not adapter-observable:
it reads a document. Saying so is the requirement — a story that touches a port and names no rule is
a port change nothing can fail, and this story's answer to that bar is that it is not that kind of
story.

## Integration contract

- **Archetype**: `foundation` — real in-tree substrate, consumed inside this same slice by two
  capability stories. Not a double, not a `todo!()`.
- **Slice / milestone**: `specification-pin`. Slice-mates, implemented in one context:
  `narrative-citation-resolution` (AC-007) and `frozen-documentation-must-pin` (AC-008). This story
  is first inside the milestone and lands before both consumers
  (`_storymap.md:157-164`).
- **Mount point**: `xtask/src/spec_trace.rs` — the module the bin crate already declares with `mod
  spec_trace;` at `xtask/src/main.rs:70`. There is no new composition-root edit to make: the module
  is mounted, and `clause_ids` becomes reachable from the new checker module the moment that module
  is declared alongside `mod lint_constitution;` in the same list. The `xtask` **lib** target
  (`xtask/src/lib.rs`) declares only `mod constitution;`, so `spec_trace` is bin-crate-only and
  `pub(crate)` is exactly the visibility that reaches the checker and nothing else
  (`_design.md:485-503`).
- **Wires into**: `parse_clauses` and `struct Clause` (`xtask/src/spec_trace.rs:1357-1388`,
  `:217-238`) for the parse; `SECTIONS` (`:122-153`) via `clause_id` (`:1391-1430`) for the accepted
  prefixes; `read` (`:2307-2309`) and `workspace_root` (`:2311-2316`) for the file access; `SPEC`
  (`:67`) for the pinned document path. Consumed by the narrative checker module that
  `narrative-checker-mounted-with-pinned-path` creates, which calls it **once per run** and passes
  the resulting set to both the citation check and the pin check — two calls would read and parse the
  same document twice in one step.
- **Renders surfaces**: none. `_design.md`'s six surfaces are markdown a reader meets, gate output a
  contributor meets, and the index that connects them; this story renders none of them. It
  implements one row of `## Items` — `xtask::spec_trace::clause_ids` (`_design.md:53-57`) — whose
  `clause` field reads "n/a — reads `SPECIFICATION.md`, discharges no clause". The two gate-output
  surfaces it *enables* are rendered by its consumers: *unresolvable clause id* and *pinned MUST no
  longer at its discharge site* in `_design.md`'s state list (`:555-569`).
- **Conformance rule(s)**: none, and deliberately. This story adds no rule to `suite.rs`,
  `model.rs` or `concurrency.rs`, changes no port and is not adapter-observable — it reads
  `spec/SPECIFICATION.md`. Its observation instruments are unit tests in `xtask` plus the two
  consuming checks in the same slice.
- **Clause(s)**: none discharged and none amended. The story reads clause ids; it does not restate,
  extend or reinterpret a clause, and no `[FROZEN]` clause is edited, so no ADR is owed
  (`_storymap.md:94-99` records that the grounding pass found no Accepted decision atom governing
  gate structure).
- **Advances DoD scenario**: initiative DoD **11** ("The frozen documentation MUSTs are still
  discharged. The pinned set of clause ids is enumerated in one place, and the specification
  cross-reference step passes over the tree as it stands") and DoD **12** ("No page has become a
  second specification. Each normative claim a teaching page makes is a citation that resolves").
  This story is the resolver both scenarios' checks run through; neither turns green here, and both
  become checkable rather than assertable.

## PR boundary

The paths this story is allowed to touch, as globs. `redkiln verify --grain story` reads the first
fenced block under this heading and fails on any file changed outside it.

```
xtask/src/spec_trace.rs
.bklg/docs-that-teach/checked-documentation-surface/spec-trace-clause-id-accessor/**
```

**In this PR**

- `clause_ids` and its private `&str`-taking sibling, with the doc comment the design signed off,
  an `# Errors` section naming conditions rather than types, and the "What this does not verify"
  paragraph first.
- The self-erasing `#[expect(dead_code, reason = …)]`, naming the two consumer stories.
- A `#[cfg(test)] mod tests` in `xtask/src/spec_trace.rs` — the module has none today, and the
  shape to copy is `xtask/src/lint_constitution.rs:827-841`, including the scoped
  `#![allow(clippy::unwrap_used, reason = "test code, per the house style")]`.
- This story's own ledger and notes under its backlog folder.

**Explicitly not in this PR**

- The narrative checker module, its `REQUIRED` step, its subcommand, its help line or its
  `lint_steps` membership — `narrative-checker-mounted-with-pinned-path` owns all five, and
  `steps_named` panics on a name absent from `REQUIRED`, which makes a half-mount a build-time bug.
- Parsing the ids a page cites, and reporting a dangling one — `narrative-citation-resolution`.
- The `FROZEN_DOC_MUSTS`-shaped pin, its discharge sites and its derived-versus-hand-written count —
  `frozen-documentation-must-pin`, including the re-derivation the brief requires
  (`_decomposition.md:275-300`).
- Any change to `run`, to the generated §7.1–§7.2 region, or to `spec/SPECIFICATION.md`.
- Any change to `xtask/Cargo.toml`. No dependency is added; if one seems necessary, the read/decide
  split has not been made.
- `xtask/src/main.rs`. The mount already exists, which is why it is out of the boundary above — an
  implementer editing it is landing a consumer story's step early.

**Merge DoD**: `cargo xtask ci --fast` is green with `clause_ids` present, documented, unit-tested
and carrying no `allow`; `cargo xtask spec-trace` prints what it printed before this commit.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| -------------------- | ------- | ------------- |
| The accessor exists as signed off | `pub(crate) fn clause_ids(root: &Path) -> Result<BTreeSet<String>>`, in `xtask/src/spec_trace.rs`, beside `all_rules`. Signature, visibility and file are the design's, item for item; no re-decision. | `_design.md:85-90`, `:499-501`, `:510`; `xtask/src/spec_trace.rs:1746` |
| Read and decide are separate | `clause_ids` reads `SPEC` through `read(root, SPEC)`; a private `&str`-taking sibling turns the text into the set and carries the vacuity failure. Mirrors `all_rules` / `collect_rules` exactly, and is what makes every assertion an in-memory one — no temp directory, no new dev-dependency (DR-12). | `xtask/src/spec_trace.rs:1746-1768`, `:2307-2309`, `:67`; `xtask/Cargo.toml:16-21` |
| Ids come from the parser, prefixes from `SECTIONS` | Built on `parse_clauses`; the accepted families are whatever `clause_id` accepts, which is whatever `SECTIONS` lists. No regex, no literal prefix list — a fourth list is a family that can half-land. | `xtask/src/spec_trace.rs:1357-1388`, `:1391-1430`, `:107-153`; `_decomposition.md:250-266` |
| Both declaration forms, all six families | `#### ES-1 — …` headings and `**PS-1 — …**` bold runs both count, as does `**CF-30 is [NON-NORMATIVE] …**`; `**ES-40**` alone is a cross-reference and does not. This is inherited from `clause_id`, not re-implemented. | `xtask/src/spec_trace.rs:1350-1356`, `:1418-1429` |
| Existence, not eligibility | No maturity filter and no family filter. A `[DEFERRED]` `SY-` clause is declared, so its id resolves. `has_suite` answers a different question and is not applied. | `xtask/src/spec_trace.rs:1729-1737`; `spec/SPECIFICATION.md:280` |
| Ids only — `Clause` stays private | Returns `BTreeSet<String>`. Exposing `Vec<Clause>` would make a thirteen-field parser-internal struct crate-visible and couple both consumers to it; a later consumer needing maturity earns its own narrow accessor beside this one. | `xtask/src/spec_trace.rs:217-238`, `:1746-1752` |
| A parse that yields nothing is a hard error | The `&str`-taking sibling `bail!`s when the set is empty, in the shape `wire_rules` and `run` already use, and the message must say the checker is broken rather than the document — otherwise both consumers report real pages and real pinned ids as defective. | `xtask/src/spec_trace.rs:1784-1791`, `:616-620`; `_decomposition.md:267-271` |
| An unreadable document is a hard error | Propagates `read`'s `reading {rel}` context; never degrades to an empty set. The `# Errors` section names the two conditions — unreadable, and parses-to-nothing — not the error type. | `xtask/src/spec_trace.rs:1741-1745`, `:2307-2309`; `standards/rust/70-rustdoc-obligations.md` |
| Duplicates collapse, and that is stated | A `BTreeSet` cannot see a doubly-declared id, and nothing in the repository checks for one today. Recorded as a limit in the item's own docs rather than absorbed silently or fixed here. | `xtask/src/spec_trace.rs:1746-1752`; `_design.md:538-552` |
| Limits first, not last | The doc comment opens with what this does not verify: duplicates collapse; existence is not appropriateness; it parses no citations. `lint_constitution.rs`' reason applies verbatim — an undocumented limit is read as a guarantee. | `xtask/src/lint_constitution.rs:9-28`; `_design.md:538-552`; `project.md` DoD 7 |
| The dead-code marker erases itself | `#[expect(dead_code, reason = …)]` naming `narrative-citation-resolution` and `frozen-documentation-must-pin`. When the first call lands, the expectation is unfulfilled, which is itself a warning under `-D warnings`, which forces the attribute's deletion. `allow` would not. | `crates/happenstance-neon/src/event_store.rs:232-240` |
| `run` and the generated region are untouched | `run` keeps its own parse and its own empty-guard; `cargo xtask spec-trace` prints exactly what it printed before. No `--write`, no §7.1–§7.2 edit, no clause amended. | `xtask/src/spec_trace.rs:604-646`, `:23-57`; `project.md:134` |
| One call per gate run in the consumer | The narrative checker calls `clause_ids` once and passes the set to both the citation check and the pin check. Stated here because it is a contract on the consumers, and two calls would parse one document twice in one step. | `_design.md:555-569`; `_decomposition.md:617-643` |
| Tests are in-memory and cover only the new call site | A new `#[cfg(test)] mod tests` in this file, shaped on `lint_constitution.rs`'s. The parser is already proven against the real document by §1.3's hand count, which `run` checks on every gate run; duplicating that here would be the parallel-fixture-corpus the testing brief forbids. | `xtask/src/lint_constitution.rs:827-841`; `xtask/src/spec_trace.rs:39-57`; `_decomposition.md:704-720` |
| Family coverage is asserted by iterating `SECTIONS` | The test that proves every family resolves iterates `SECTIONS` rather than a literal list of six prefixes, so adding a seventh family cannot leave the test green by omission. `lint_constitution.rs:567` is the in-repo shape. | `xtask/src/lint_constitution.rs:567`; `xtask/src/spec_trace.rs:107-153` |

## Data and migrations

**N/A.** There is no schema, no store, no persisted state and no serialised envelope anywhere in this
story. `xtask` carries `publish = false`, so there is no released artifact and no semver promise to
migrate (`_design.md:510`; `xtask/Cargo.toml:7`).

The only artifact read is `spec/SPECIFICATION.md`, read-only, through the existing `SPEC` constant
and the existing `read` helper — this story does not edit it, does not run `spec-trace --write`, and
does not touch the generated §7.1–§7.2 region, so the committed-versus-computed equality check the
gate runs is unaffected (`xtask/src/spec_trace.rs:31-38`, `:194-200`). The one adjacent data hazard
worth naming is not a migration either: the unmerged
`initiative/from-contract-to-published-library` branch diverges from this document by 521 lines, and
resolution by clause id survives it because ids are stable names rather than line references
(`spec/SPECIFICATION.md:280`; `project.md:274-281`).

## Acceptance criteria

The persona whose goal these criteria serve is not a runtime user — this story has no runtime.
It is the **contributor** the design names as the second of its two readers
(`_design.md`, "What a user meets first"), acting on behalf of the three the initiative
carries: the *application author* who will cite a clause in a teaching page, the *adapter
author* who will meet the frozen MUSTs at their discharge sites, and the *evaluator* who reads
a citation as a promise that the claim above it is anchored in something real
(`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:56`, `:148`, `:225`).
Each criterion is that contributor completing something, end to end, through the real module.

| id | criterion | verification |
| -- | --------- | ------------ |
| AC-001 | **GIVEN** a contributor in the same slice writing the narrative checker as a second bin-crate module, **WHEN** they need to answer "does clause id `X` exist in `spec/SPECIFICATION.md`", **THEN** `pub(crate) fn clause_ids(root: &Path) -> Result<BTreeSet<String>>` is callable from that module without any change to `xtask/src/main.rs` or `xtask/src/lib.rs`, it reads the pinned `SPEC` path through the existing `read` helper, and against the real workspace root it returns a non-empty set containing the ids the document declares — **AND** the decision half is a private `&str`-taking sibling, so every other assertion in this story is in-memory and adds no dev-dependency. | `xtask/src/spec_trace.rs`, new `#[cfg(test)] mod tests`: `clause_ids_reads_the_pinned_specification` — calls `clause_ids(&workspace_root()?)` and asserts the set is non-empty and contains `"ES-1"` and `"VT-1"`, the two ids declared at `spec/SPECIFICATION.md:2460` and `:563`. Signature, visibility and file are checked against `_design.md:85-90`, `:499-501`, `:510` at review. Run by `cargo test -p xtask`. |
| AC-002 | **GIVEN** a maintainer who later adds a seventh clause family to `SECTIONS`, **WHEN** they add it in that one place and run the gate, **THEN** the narrative checker's resolution accepts the new family with no second edit — because `clause_ids` takes its accepted prefixes from `SECTIONS` via `clause_id` rather than from any literal prefix list, and it recognises exactly the declaration forms the parser recognises: the `####`-heading form, the `**PS-1 — …**` bold form and the `**CF-30 is [NON-NORMATIVE] …**` form, while a bare `**ES-40**` cross-reference is not a declaration. | `xtask/src/spec_trace.rs` `mod tests`: `every_section_family_resolves` — builds an in-memory slice declaring one clause per family by **iterating `SECTIONS`** (the shape at `xtask/src/lint_constitution.rs:567`), so a seventh family cannot leave the test green by omission; and `both_declaration_forms_resolve_and_a_cross_reference_does_not`. Reviewer check: the diff introduces no clause-id regex and no prefix array. |
| AC-003 | **GIVEN** an application author whose teaching page cites a `PS-` or `SY-` clause that the specification declares as `[DEFERRED]`, **WHEN** the gate runs, **THEN** the citation resolves and the page passes — the resolver answers *existence*, never *eligibility*: no maturity filter, no family filter, and `has_suite` is not applied. Ids are stable names that are never renumbered, which is what makes resolution-by-name survive the 521-line divergence on the unmerged sibling branch. | `xtask/src/spec_trace.rs` `mod tests`: `a_deferred_clause_without_a_suite_still_resolves` — an in-memory slice declaring a `[DEFERRED]` `SY-` clause; asserts its id is in the set **and** asserts `has_suite` on that same id is `false`, so the test fails if a future edit routes the accessor through it. Grounded in `spec/SPECIFICATION.md:278-281` and `xtask/src/spec_trace.rs:1729-1737`. |
| AC-004 | **GIVEN** a contributor whose checkout has a truncated, moved or unreadable `spec/SPECIFICATION.md`, **WHEN** the gate runs, **THEN** the run fails with a message that names **the artifact it could not read or could not parse** and says the checker is broken rather than the document — never `Ok` with an empty set, which would make both consumers report real pages and real pinned ids as defective. The failure is one composed sentence on the terminal surface with the artifact name first, so soft-wrap at 80 columns cannot push the location off the first visual row, and it never elides. | `xtask/src/spec_trace.rs` `mod tests`: `an_unreadable_specification_names_the_file_it_could_not_read` — `clause_ids(Path::new("no/such/root"))` is `Err` and the formatted chain contains `reading spec/SPECIFICATION.md`; and `a_specification_that_declares_nothing_blames_the_checker` — the `&str`-taking sibling on a document with no declarations is `Err`, and the message contains `spec/SPECIFICATION.md` and wording that attributes the fault to the checker, in the shape `xtask/src/spec_trace.rs:1784-1791` already uses. |
| AC-005 | **GIVEN** a contributor who opens the accessor to decide whether they may trust it, **WHEN** they read its documentation, **THEN** the *first* thing they meet is what it does not verify — a doubly-declared id collapses and nothing checks for one; existence is not appropriateness; it parses no citations — followed by an `# Errors` section naming the two failure *conditions* rather than a type, and one sentence naming the alternative that lost (a literal prefix list; a `Vec<Clause>` return). No badge, tick or "verified" wording claims a resolving citation is a *correct* citation. Prose wraps at ≤ 90 source columns and the hierarchy is carried by position and heading level alone. **AND** the headline limit is proven rather than asserted. | `xtask/src/spec_trace.rs` `mod tests`: `a_doubly_declared_id_collapses_to_one` — an in-memory slice declaring `ES-1` twice yields one entry (RS-81-1's "prove the blind spot in its own tests, then state it in its own documentation", `standards/rust/81-checks-that-cannot-be-types.md:11`); and `the_accessor_documents_its_limits_before_its_errors` — reads `xtask/src/spec_trace.rs` as text through `read(&workspace_root()?, …)`, exactly as `check_harness` reads a source file (`xtask/src/lint_constitution.rs:424-425`), and asserts the `does not verify` line precedes the `# Errors` line inside the `clause_ids` doc block. Rendering proven by `cargo xtask ci`'s docs step. |
| AC-006 | **GIVEN** this foundation lands one commit before its two consumers, **WHEN** the contributor runs the gate at this story's own checkpoint, **THEN** `cargo xtask ci --fast` is green with a `pub(crate)` function that nothing yet calls — carried by `#[expect(dead_code, reason = …)]` whose reason names `narrative-citation-resolution` and `frozen-documentation-must-pin`, **AND** the marker is self-erasing: the moment the first consumer calls `clause_ids`, the expectation is unfulfilled, which is itself a warning, which `-D warnings` turns into a failure that forces the attribute's deletion. `#[allow(dead_code)]` appears nowhere in the diff, because `allow` would rot into a permanent exemption. | `cargo xtask ci --fast` — the clippy step with `-D warnings` (CLAUDE.md, "Commands") is green at this story's checkpoint commit. Reviewer check against the in-repo precedent `crates/happenstance-neon/src/event_store.rs:228-240`: the attribute is `expect`, its `reason` names both consumer stories, and the diff contains no `allow(dead_code)`. The self-erasure fires in the *consumer's* PR by design and is recorded as a hand-off in this story's implementation report. |

Project AC coverage: **AC-007** is served by AC-001 (the resolver exists and reads the real
document), AC-002 (families come from the parser, so no citation family half-lands), AC-003
(deferred ids resolve, so a real citation does not dangle) and AC-004 (a checker that cannot
read is a hard error, not a silent pass). **AC-008** is served by AC-001 and AC-003 — the pin
resolves every id it enumerates, including the deferred ones — and by AC-004, which is what
stops a broken read from reporting the pin as the thing that moved. AC-005 and AC-006 carry
project DoD 7's honesty obligation and the gate discipline that lets a foundation land alone
(`project.md`, AC-007/AC-008; `_storymap.md:52`).

## Interaction quality

This story renders no interactive surface: no screen, no viewport, no pointer, no editor
state. The **state** family is therefore answered N/A — but answered, one line each, because
a reviewer who cannot tell "inapplicable" from "forgotten" has to assume the worse of the two.
The **composition** family is *not* vacuous. This story renders two composed things a person
reads — the item's own rustdoc, and the sentence a contributor meets when the accessor fails —
and `_design.md` binds both. Every invariant below is carried by an `AC-###` row in the table
above; none of them lives only here.

**State invariants**

- *In-place vs context-jump* — N/A. There is no navigation; the only "place" is a call site.
- *Non-occlusion* — N/A. Nothing overlays anything; the module adds no output on success, and
  `_design.md`'s one-line success budget (`:419-425`) is unchanged by this PR.
- *Preserved focus / scroll / selection* — N/A. No viewport and no selection exist.
- *Reversibility* — N/A at the surface, and its real analogue is **AC-006**: the single
  temporary state this PR introduces (a `pub(crate)` item with no caller) is undone by the
  compiler rather than by anyone's memory.
- *Keyboard reachability* — N/A; there is no pointer affordance to be unreachable from. The
  nearest genuine obligation is that the item is reachable from a sibling module at all,
  which is **AC-001**'s `pub(crate)` visibility against the bin-crate target boundary.

**Composition invariants** (from `_design.md`; each names the AC row that carries it)

- *Presentation exists at all* — **AC-005** for the rustdoc (limits first, `# Errors` naming
  conditions, the alternative that lost named once per RS-70-5,
  `standards/rust/70-rustdoc-obligations.md:243`) and **AC-004** for the failure, which is a
  composed sentence naming an artifact rather than a bare error type. A one-line summary
  standing in for documentation, or a `?`-propagated error with no context, satisfies every
  type check and fails both rows.
- *Composition and placement* — **AC-001**. In this medium placement *is* the composition
  decision: beside `all_rules`, in the file that owns the parser, `pub(crate)`
  (`_design.md:485-503`, `:506-514`).
- *Transience* — **AC-005** and **AC-004**. The docs are persistent chrome: always present,
  rendered by `cargo doc` on every gate run. The failure sentence is revealed, on failure
  only, exactly once, and there is deliberately nothing opened-on-demand — no verbose flag, no
  second message behind a switch.
- *Density budget, with the real numbers* — **AC-005** carries the markdown-side prose wrap of
  **≤ 90 source columns** (p90 of the existing corpus is 81, `_design.md:376-395`);
  **AC-004** carries the terminal surface's **80 columns** with the artifact name first, so
  the location is never pushed off the first visual row by soft-wrap (`_design.md:419-425`).
- *Hierarchy* — **AC-005**. Position and heading level only; colour, weight and size are not
  available in either medium and `_design.md` says so (`:429-433`).
- *Named anti-patterns* — **AC-004** carries anti-pattern 8 (**no truncated problem list**:
  this function's error is one sentence and never elides with "… and N more"). **AC-005**
  carries anti-pattern 9 (**no badge, tick, shield or "verified" wording**): the doc comment
  must not imply that a resolving citation is a correct citation, which is the exact claim
  DoD item 8 forbids the whole surface from making.

Anti-patterns 1–7 and 10–11 bind the markdown tree and the checker's own output, which this
story does not render; they land on its slice-mates.

## Error conditions

| id | condition | required behaviour |
| -- | --------- | ------------------ |
| EC-001 | `spec/SPECIFICATION.md` is absent, unreadable, or the `root` handed in is not the workspace root | Propagate `read`'s `reading {rel}` context unchanged (`xtask/src/spec_trace.rs:2307-2309`). Never `Ok(BTreeSet::new())`, and never a bespoke message that loses the path. Callers obtain `root` from `workspace_root()` (`:2311-2316`), not from `current_dir`. |
| EC-002 | the document is readable but declares no clause ids at all | `bail!` from the `&str`-taking sibling, naming the document and attributing the fault to the checker rather than the document — the shape `wire_rules` uses (`:1784-1791`) and `run` uses (`:616-620`). This is the failure that must not be deferred to the consumers, because they would name real pages and real pinned ids instead. |
| EC-003 | the same clause id is declared twice in the document | **Not an error.** A `BTreeSet` cannot see it, and nothing in the repository checks for one today. Recorded as the headline limit in the item's own docs (AC-005) and proven by test rather than silently absorbed. Fixing it is out of this story's boundary; if it is ever wanted, it is a separate check next to §1.3's census, not a change to this return type. |
| EC-004 | a page cites an id that exists but whose family has no conformance suite, or whose maturity is `[DEFERRED]` | **Not an error, and must not become one.** AC-003. Applying `has_suite` or a maturity filter here would silently dangle every `PS-` and `SY-` citation. |
| EC-005 | a caller wants a clause's maturity, section or discharge site | **Not served, deliberately.** `clause_ids` returns ids. The answer is a second narrow accessor beside this one, not a widened return type or a `pub(crate) struct Clause` (`xtask/src/spec_trace.rs:217-238`). |
| EC-006 | two consumers each call `clause_ids` in one gate step | Not an error this function can detect, and stated as a contract on the consumers instead: the checker calls it **once** and passes the set to both checks. Two calls read and parse a 9,070-line document twice inside one step for no new information. |

## Non-functional

| id | requirement | how it is held |
| -- | ----------- | -------------- |
| NF-001 | one read and one parse of `spec/SPECIFICATION.md` per gate run — the document is 9,070 lines / ~567 KB, and the consumer's single call is the contract that keeps it at one | EC-006; reviewed at the consuming story, not enforced by a type here |
| NF-002 | **zero new dependencies.** No temp-directory crate, no regex crate, no walker | DR-12's standing trade at `xtask/Cargo.toml:9-21`, which keeps `rusqlite` and `sqlx` out of xtask's dev graph because every `cargo xtask ci` would build them. The read/decide split is what makes this free; if a dependency looks necessary, the split has not been made. |
| NF-003 | no `unwrap` / `expect` outside `#[cfg(test)]`; the test module carries the scoped `#![allow(clippy::unwrap_used, reason = "test code, per the house style")]` | `xtask/src/lint_constitution.rs:827-829` is the exact shape to copy; enforced by clippy under `-D warnings` |
| NF-004 | `cargo xtask spec-trace` prints byte-identically to its pre-commit run, and the committed §7.1–§7.2 region is unchanged | `run` is outside the diff (`xtask/src/spec_trace.rs:604-646`); the generated-region equality check (`:31-38`, `:194-200`) is unaffected because nothing writes |
| NF-005 | MSRV, `wasm32` and the feature powerset are untouched | `xtask` is a host-only `publish = false` tooling crate with no `cfg`, no feature and no target attribute in this diff (`xtask/Cargo.toml:1-10`) |
| NF-006 | the accessor's cost stays inside the noise floor of the step it joins — a single file read plus one line scan | the class `xtask/src/affected.rs` already argues finishes inside the time cargo takes to decide `xtask` is up to date (`_design.md`, "What it costs a caller") |

## Implementation notes (non-prescriptive)

Shape, not instructions. Every one of these is a *reason* an implementer may overrule with a
better one — except where a row of the acceptance table says otherwise.

- **Read `all_rules` and `collect_rules` first** (`xtask/src/spec_trace.rs:1739-1768`) and
  mirror the split rather than inventing one. `clause_ids` reads and calls; the sibling takes
  `&str`, calls `parse_clauses`, maps each `Clause` to its id, collects, and carries EC-002's
  `bail!`. The sibling's name is the implementer's; `collect_clause_ids` matches the
  neighbour's convention.
- **`parse_clauses` already returns `Vec<Clause>`** (`:1357-1388`), and each `Clause` already
  carries its id — so the decision half is a `map` and a `collect`, not a parser. If a line of
  parsing appears in the new code, the wrong seam has been taken.
- **Put the test module at the end of the file.** `xtask/src/spec_trace.rs` has none today
  (2,316 lines, no `cfg(test)`), so this PR creates the first one; `xtask/src/lint_constitution.rs:827-841`
  is the shape, including the scoped `allow` and `use super::*;`.
- **The in-memory slices should look like the document, not like a minimal parse input.** A
  three-line slice that happens to parse is weaker evidence than a slice that carries a
  heading, a maturity marker and a following paragraph, because the second one is the shape
  that will change when the document changes.
- **Write the `# Errors` section as conditions, not types** — "returns an error if the document
  cannot be read, or if it declares no clauses at all, which would report every citation as
  dangling when the defect is here". `all_rules`' own section (`:1741-1745`) is the sentence
  to model.
- **Name the alternative once** (RS-70-5, `standards/rust/70-rustdoc-obligations.md:243-270`):
  a literal prefix list in the checker lost because it would be the fourth list of the six
  families and `SECTIONS`' doc comment already says why (`:107-120`).
- **Do not reach for a temp directory** to test EC-001. A root that does not exist is enough:
  `read` fails with the context, which is exactly what the test asserts.
- **Leave `run` alone.** If it looks like it should share the accessor, re-read the "What must
  not move" paragraph in the context pack: it needs whole `Clause` values, it already guards
  its own empty parse, and routing it here would parse one document twice per gate step and
  change the sentence a green `spec-trace` prints.

## Tests and CI (merge gate)

Grounded in the project's testing brief — its AC-007 and AC-008 bullets
(`_decomposition.md:617-643`), its "Fixtures and seams to mock — there are almost none,
deliberately" note (`:704-720`) and its merge-gate command list (`:689-698`), narrowest first.

| tier | command / path | proves |
| ---- | -------------- | ------ |
| unit — in-memory decision | `cargo test -p xtask` → `xtask/src/spec_trace.rs`, `mod tests`: `every_section_family_resolves`, `both_declaration_forms_resolve_and_a_cross_reference_does_not`, `a_deferred_clause_without_a_suite_still_resolves`, `a_specification_that_declares_nothing_blames_the_checker`, `a_doubly_declared_id_collapses_to_one` | AC-002, AC-003, AC-004 (vacuity half), AC-005 (blind spot half). All against `&str`: no filesystem, no fixture tree, no temp directory, no new dev-dependency — which is the testing brief's explicit instruction not to mock `spec/SPECIFICATION.md` beyond small in-memory slices. |
| unit — real-path read | `cargo test -p xtask` → `clause_ids_reads_the_pinned_specification`, `an_unreadable_specification_names_the_file_it_could_not_read`, `the_accessor_documents_its_limits_before_its_errors` | AC-001, AC-004 (unreadable half), AC-005 (documentation half). These are the only tests that touch the real tree, and each touches it through the existing `read` / `workspace_root` helpers rather than around them. |
| lint — the dead-code contract | `cargo xtask ci --fast` (clippy, `-D warnings`) | AC-006: an uncalled `pub(crate) fn` is green under `#[expect(dead_code, reason = …)]` and no `allow` appears. This is the tier that fails if the marker is spelled `allow`, or if the `reason` is absent. |
| docs render | `cargo xtask ci` (docs step) | AC-005's rendering half: the doc comment builds and its intra-doc links to `all_rules` and `SECTIONS` resolve in every feature configuration (RS-70-2). |
| regression — the existing checker | `cargo xtask spec-trace` | NF-004: `run`, §1.3's hand count and the committed §7.1–§7.2 region are untouched, and the output is what it was before this commit. Run before *and* after the change, with the two outputs compared. |
| regression — the neighbouring checker | `cargo xtask lint-constitution` | that copying `lint_constitution.rs`'s shapes did not edit it (`_decomposition.md`, Note 8). |
| story-grain gate | `cargo xtask affected --base main` | that a change confined to `xtask/src/` selects the `xtask` package — the grain `.redkiln/config.yaml`'s `verify:` block wires as this story's actual check, so a gap here is invisible until a later story compiles nothing. |
| merge bar | `cargo xtask ci --fast` | the bar a non-terminal story meets (CLAUDE.md, "Commands"; `project.md` DoD 6). The full `cargo xtask ci` is the project's bar, not this story's. |

No conformance rule is added, so no `crates/happenstance-testkit/` file changes and CF-29's
`CHANGELOG.md` obligation (`xtask/src/lints.rs:504-580`) is not triggered — stated because a
story that says nothing about it is indistinguishable from one that forgot.

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | how this PR bounds it |
| ---- | ------------------- | --------------------- |
| The fourth prefix list appears anyway, in the consumer, because the resolver was easier to re-implement than to find | Medium / High — it is the exact defect this story exists to prevent, and it fails *quietly*: every id of a newly added family reports as dangling while `spec-trace` counts it happily | The foundation lands **first** in the slice and both consumers are implemented in the same context (`_storymap.md:157-164`). AC-002 asserts the family list is `SECTIONS`, by iterating it. |
| `#[expect(dead_code)]` is left in place after the first consumer lands | Medium / Low, and self-correcting | The unfulfilled expectation is a warning and `-D warnings` is a failure, so it surfaces in the consumer's own gate run. The `reason` names both consumer stories so the next implementer knows the attribute is theirs to delete, not to widen. |
| An implementer rewires `run` through the new accessor as a tidy-up | Medium / Medium — two parses per gate step and a changed `spec-trace` sentence | Named in the PR boundary's "explicitly not in this PR" list and in NF-004, which requires the before/after outputs to match. |
| `Clause` is made `pub(crate)` under pressure from `frozen-documentation-must-pin` wanting maturity | Low / Medium — a thirteen-field parser-internal struct becomes crate surface, and every future field addition touches code that only wanted an id | EC-005 answers it in advance: a second narrow accessor beside this one, which is cheap precisely because the accessors sit next to the parser. |
| A test is added that re-proves the parser against the real document | Medium / Low | The testing brief forbids it explicitly (`_decomposition.md:704-720`): §1.3's hand count already proves the parse on every gate run, and a parallel fixture corpus is the shape RS-81-3's directory-scoping argument warns against. |
| `xtask/src/spec_trace.rs` conflicts on merge with the unmerged `initiative/from-contract-to-published-library` branch | Low / Medium — that branch diverges from `spec/SPECIFICATION.md` by 521 lines and may touch its parser too | This PR appends beside `all_rules` and creates a new module at the file's end; both are low-conflict placements, and resolution is by clause id rather than by line, so a merged divergence cannot invalidate the accessor (`project.md:274-281`). |
| The gate is read as evidence that citations are *correct* | Low / High — it is the initiative's named honesty failure | AC-005's anti-pattern 9 row: no badge, tick or "verified" wording, and the limits stated first. `documented-blind-spots-and-their-proofs` audits this at the end of the project. |

Coupling out of the boundary is deliberately one-directional: nothing in this PR reads the
narrative tree, the harness, or any constant a slice-mate owns. The only shared file is
`xtask/src/spec_trace.rs`, and no other story in this project touches it.

## Dependencies

- **Blocks on**: nothing. `depends_on: []`. Every construct this story needs —
  `parse_clauses`, `clause_id`, `SECTIONS`, `read`, `workspace_root`, `SPEC` — exists on
  `main` today, and the module is already mounted at `xtask/src/main.rs:70`.
- **Unlocks**: `narrative-citation-resolution` (project AC-007) and
  `frozen-documentation-must-pin` (project AC-008). Both call `clause_ids`; the second is also
  the story `HS-P0023` is sequenced behind, because the pin must exist before any project
  rewrites a `happenstance-core` doc comment (`_storymap.md:157-164`).
- **Not a dependency of this story, and worth saying so**: both consumers also depend on
  `narrative-checker-mounted-with-pinned-path`, which is in the *previous* milestone. This
  story does not — it adds no step, no subcommand and no module, so it can land without the
  checker existing. An implementer who waits for the checker has misread the archetype.

## Anchors (progressive disclosure)

Open these when the row says to, not before. The context pack above is self-sufficient for
starting; these carry the depth it distilled.

| anchor | why it is load-bearing | when to open | serves |
| ------ | ---------------------- | ------------ | ------ |
| `xtask/src/spec_trace.rs` | The mount and every seam: `SPEC` (`:67`), `SECTIONS` and its "three lists that must agree" reasoning (`:107-153`), `struct Clause` (`:217-238`), `parse_clauses` (`:1357-1388`), `clause_id`'s declaration-vs-cross-reference logic (`:1391-1430`), `has_suite` (`:1729-1737`), the `all_rules`/`collect_rules` split to mirror (`:1739-1768`), `wire_rules`' blame-the-checker `bail!` (`:1784-1791`), `read` and `workspace_root` (`:2307-2316`), and `run`, which must not move (`:604-646`) | **First, before writing a line.** Read `:1739-1768` and `:1784-1791` together — they are the two halves of this story's shape | AC-001 |
| `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` | Architecture Note 6 (`:250-274`) is the decision this story implements, in the words that decided it, including "do not copy a regex" and the transfer of `lint_constitution.rs`'s hard-error posture; Note 7 (`:275-300`) is what the consumer will do with the set; the testing brief's AC-007/AC-008 bullets (`:617-643`) and the fixtures note (`:704-720`) are why this story's tests are in-memory and few | Note 6 before implementing; the testing brief before writing the test module | AC-002 |
| `.bklg/docs-that-teach/checked-documentation-surface/_design.md` | Signed off by a human and **binding**: the verbatim signature and doc-comment opening (`:83-90`), the `## Items` row for `clause_ids` (`:45-57`), the placement and `pub(crate)` reasoning (`:485-503`), the visibility/semver table (`:506-514`), the limits-first rule (`## What a user meets first`), the density budget's real numbers (`:376-395`, `:419-425`) and the anti-patterns list | Before writing the signature, and again before writing the doc comment | AC-005 |
| `xtask/src/lint_constitution.rs` | Three shapes to copy rather than invent: the "why the citation parser hard-errors instead of skipping" argument and its limits-first module docs (`:1-45`), the iterate-`SECTIONS` assertion shape (`:567`), and the test module including its scoped `allow` (`:827-841`). Also the read-a-source-file-as-text pattern (`:424-425`) that AC-005's documentation test uses | Before writing the vacuity `bail!` message, and before creating the test module | AC-004 |
| `standards/rust/81-checks-that-cannot-be-types.md` | RS-81-1 (`:11`) is AC-005's whole structure — prove the blind spot in the tests, *then* state it in the docs; RS-81-2 (`:95`) is the hard-error posture as a house rule rather than a local precedent | With the doc comment and the duplicate-collapse test open together | AC-005 |
| `standards/rust/70-rustdoc-obligations.md` | The obligations the doc comment is graded against: `# Errors` phrased as conditions (`:12-92`), intra-doc links that resolve in every configuration (`:93-151`), and RS-70-5 — name the alternative that lost, once (`:243-270`) | While writing the doc comment | AC-005 |
| `crates/happenstance-neon/src/event_store.rs` | The in-repo precedent for the self-erasing marker: `#[expect(dead_code, reason = …)]` with the reasoning attached at `:228-240`, including the sentence explaining why `expect` beats `allow` | When the first `cargo clippy` run reports the unused function | AC-006 |
| `spec/SPECIFICATION.md` | §1.4 at `:278-281` — "Clause IDs are stable and are never renumbered" — is the property that makes resolution-by-name meaningful and makes AC-003 correct rather than merely convenient; `:563` and `:2460` are the two declared ids AC-001's real-path test asserts | When writing AC-001's and AC-003's tests | AC-003 |
| `xtask/Cargo.toml` | DR-12's standing trade in situ (`:9-21`): why `rusqlite` and `sqlx` are deliberately absent from xtask's dev graph, which is the argument any new dependency would have to beat | Only if a dependency starts to look necessary — which means re-reading the read/decide split instead | AC-001 |
| `.bklg/docs-that-teach/checked-documentation-surface/_storymap.md` | The slice's merge order (`:157-164`) and this story's row (`:52`), which name the two consumer stories the `#[expect]` reason must cite and confirm this story lands before both | When writing the `reason` string, and when planning the checkpoint commit | AC-006 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | The three personas the acceptance criteria are framed from — application author (`:56`), adapter author (`:148`), evaluator (`:225`) — and the qualification that none has been directly observed, which is why no criterion here claims comprehension | Before re-wording any acceptance criterion | AC-003 |
| `xtask/src/lints.rs` | CF-29's `CHANGELOG.md` obligation and how it is detected (`:504-580`), which is the check this story must be able to say it does not trigger | Only if a reviewer asks why no changelog entry accompanies a change to gate code | AC-006 |

## Clarifications resolved during spec

- **The AC set is exactly the six the front half enumerated** — AC-001 through AC-006. None
  added, none dropped. The ledger carries the same six ids and no others.
- **Where the tests live.** `xtask/src/spec_trace.rs` has no `#[cfg(test)]` module today
  (verified: no `cfg(test)` in 2,316 lines), so this PR creates the file's first one rather
  than adding to an existing suite. That is a small, deliberate widening of the diff and it is
  named in the PR boundary.
- **How AC-005 is actually gated, rather than asserted.** "Limits first" looked like a
  review-only criterion. It is not: the file can be read as text through the module's own
  `read` helper, exactly as `check_harness` reads a source file, so the ordering of the two
  doc-comment headings is a real test. The `cargo doc` step covers rendering; the
  duplicate-collapse test covers the limit's *truth*. Three instruments, one criterion, no
  prose-only claim.
- **AC-006 cannot be tested in this story, and that is stated rather than hidden.** The
  self-erasure fires when the *consumer* adds the first call. The verification here is the
  green clippy step plus a review of the attribute's spelling and `reason`; the erasure itself
  is recorded as a hand-off. Claiming a test for it would be the decorative-check defect
  CLAUDE.md names.
- **Whether the vacuity guard belongs in the outer or the inner half.** The inner,
  `&str`-taking half, so the failure is assertable without a filesystem. This is the only
  place the split's shape was actually load-bearing rather than merely conventional.
- **Interaction quality is not N/A wholesale.** The state family is; the composition family is
  not, because rustdoc and a terminal sentence are composed surfaces the signed-off design
  binds. Both are carried by AC-004 and AC-005 as table rows, per the extraction rule — a
  bullet in that section would never have been gated.
- **No ADR is owed.** The grounding pass found no Accepted decision atom under
  `.kb/decisions/` governing gate structure or documentation checks (`_storymap.md:93-99`), no
  `[FROZEN]` clause is edited, and no port changes. The one divergence from precedent —
  hard-erroring where `spec_trace::citations` skips — is a convention discharged by a sentence
  in the item's own docs, which is what Note 6 asks for.
