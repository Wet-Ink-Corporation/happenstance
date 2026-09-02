---
item: HS-S0173
stage: spec
created: 2026-08-17T13:16:25.568Z
updated: 2026-08-17T13:16:25.568Z
template_sig: 87bbf1d0
rendered_sig: f9342bf1
---

# Spec — Re-run spec-trace post-merge and state clause-id completeness

## Scope lock

| Layer | Path | What it fixes for this story |
| --- | --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` | DoD scenario **11** (`:452-454`) and the closing criterion (`:662-663`) — the frozen documentation MUSTs are enumerated in one place by clause id and every one is still discharged, on the tree as it stands |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` | The merge-forward decision and the residual risk it accepted (`:89-103`): clause ids are stable, so the sibling can only *add* a documentation MUST — and DoD-11's re-run at closeout is what finds it |
| Project (this project's mandate) | `.bklg/docs-that-teach/durable-audience-closeout/project.md` | AC-013 and AC-012 (`:202`, `:201`), DR-12 (`:172-175`), and the out-of-scope line that keeps *pinning* with HS-P0020 (`:102-105`) |
| This spec | `.bklg/docs-that-teach/durable-audience-closeout/post-merge-clause-completeness/spec.md` | The story an implementer loads |
| Key briefs | `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md` | Architecture brief §1 (the read-only diff surface, `:410-427`), §3 (what actually binds, and the ADR that does not exist, `:494-518`), §5 (DR-12's re-check is a read, `:649-655`); UX brief IQ-3 (`:180-187`), AC-UX-09/10/12 (`:266-281`); Testing brief tier row for AC-013 (`:873`) |
| Signed-off design | `.bklg/docs-that-teach/durable-audience-closeout/_design.md` | `## Items` is empty by decision — this project ships no `pub` item and no rendered surface (`:10-20`, `:45-47`); sign-off recorded `:91-93` |
| Grounding | `.bklg/docs-that-teach/durable-audience-closeout/_grounding.md` | Verified anchors, and the confirmation that no Accepted decision atom governs this work (`:18-31`) |
| Story map / roadmap pointer | `.bklg/docs-that-teach/durable-audience-closeout/_storymap.md` | This story's row (`:49`), milestone `merged-tree-baseline` (`:62-65`), and merge order 1 (`:144-147`) |
| Upstream owner of the pin | `.bklg/docs-that-teach/checked-documentation-surface/project.md` | AC-008 — the set is enumerated by clause id in **exactly one file** (`:223-226`) — and the explicit hand-off of the residual risk to HS-P0025 (`:273-279`) |

## One-line PR slice

Re-run `cargo xtask spec-trace` on the merged tree and write the set-difference statement
saying whether the sibling added a documentation MUST absent from HS-P0020's pinned
clause-id set, pinning or routing any addition.

## Executive summary

`merge-forward-baseline` produces one named commit. This story is the first thing observed
against it, and it observes exactly one thing: whether the merge moved the frozen
documentation MUSTs out from under the pin HS-P0020 already landed.

The delta over what already exists is small and deliberate. `cargo xtask spec-trace` is
already a `REQUIRED` gate step (`xtask/src/main.rs:315`, args at `:317-326`), so *running*
it is not the deliverable — it runs again inside `terminal-gate-run`'s `cargo xtask ci`
regardless. What this story adds on top is the thing no command produces: **a written set
difference over clause ids**, in both directions, naming the merged sha it was taken
against, and disposing of every difference it finds rather than noting it
(`_decomposition.md:649-655`).

It also closes a risk the initiative's decomposition accepted on the record. The gate
decision to run in parallel and merge forward rather than block was taken knowing the
sibling branch diverges from `spec/SPECIFICATION.md` by 521 lines; the argument that made
that safe was that clause ids are stable names, never line references
(`spec/SPECIFICATION.md:280`), so a divergence cannot invalidate a pinned entry. The
residual — the sibling *adds* a documentation MUST and leaves the pin incomplete — was
handed forward by name to this project (`checked-documentation-surface/project.md:273-279`).
This story is where that hand-off is discharged or is proven not to have fired.

## Context pack

Ten decisions. They are decisions, not references; the deeper artefacts sit behind the
anchors the second pass enumerates.

**1. This is a read. Nothing under `crates/`, `spec/`, `xtask/`, `standards/` or `docs/`
is written.** The architecture brief fixes the diff surface as a table and puts all five of
those trees on the read-only side (`_decomposition.md:410-427`). `cargo xtask spec-trace`
*runs over* `spec/SPECIFICATION.md`; it does not edit it. No `--write` invocation, no
regenerated §7.1–§7.2 region, no new gate step, no new subcommand, no new `REQUIRED` entry.
If a read-only path turns out to need a change, that is a routed gap, not a scope
extension.

**2. The comparison is a set difference over clause ids — never a diff over line numbers.**
`spec/SPECIFICATION.md:280` states that clause IDs are stable and are never renumbered.
That is precisely why the failure mode this story hunts is an **incomplete** pin rather
than a stale one (`_decomposition.md:516-518`; `project.md:248`). An implementer who reaches
for `git diff spec/SPECIFICATION.md` between the pre- and post-merge trees has answered a
different question — 521 lines of divergence will drown the one clause that matters.

**3. There is exactly one enumeration of the set in the tree, and it is not this story's.**
HS-P0020's AC-008 requires the set enumerated by clause id in exactly one file
(`checked-documentation-surface/project.md:223-226`), and its `frozen-documentation-must-pin`
story lands that enumeration as a commented `const` inside the narrative checker, together
with a derived candidate scan and the derivation rule — (a) the clause is `[FROZEN]` and
(b) its obligation falls on **the contract's own documentation**, not on an adapter's or a
fixture's (`checked-documentation-surface/frozen-documentation-must-pin/spec.md`, its
`## Behavior and interfaces` rows). **This story reads that enumeration as landed and
builds no second one.** A regex over clause ids, a prefix list, a hand-copied set in a
backlog markdown table that later drifts — each is a second enumeration, and the sibling
spec already names the class of defect: a fourth list of the six clause families is the one
that can half-land.

**4. Verify the pin's landed shape before citing it; do not assume the planned path.**
`xtask/src/lint_narrative.rs` and `crate::spec_trace::clause_ids` are HS-P0020's planned
landing sites and **do not exist in this worktree today** — `xtask/src/` currently holds
`affected.rs`, `constitution.rs`, `lib.rs`, `lint_constitution.rs`, `lints.rs`, `main.rs`,
`package.rs`, `proof.rs`, `reserve.rs`, `spec_trace.rs`. On the merged tree they should
exist, because HS-P0020 is upstream of this project through HS-P0024
(`project.md:224-229`). Locate the enumeration by search on the merged tree and cite it at
the path and line it actually occupies. If it is absent — HS-P0020 having landed a
different shape, or not yet — that is a finding the statement records by name, and the
fallback is to derive the candidate set against the stated derivation rule **for the record
only**, never by adding code.

**5. Pinning is HS-P0020's; routing is this story's; silence is neither.** The project's
out-of-scope list is explicit: HS-P0020 owns the pin, and this project "only re-runs the
cross-reference over the merged tree and reports whether the pinned set is still complete
and still discharged" (`project.md:102-105`). So an addition has exactly two admissible
dispositions, and each names an owner and a destination: **pinned** — handed back to the
one enumeration, classified there, with the classifying change identified; or **routed** —
an item under the `support` initiative (`.redkiln/config.yaml:5`), named in the statement
with the reason it could not be pinned inside this closeout. "Noted for a future pass" is
not a disposition.

**6. The routing arm has a hard consequence, and the statement must say so.** If the pin
landed with its derived candidate scan, an unclassified candidate is already a **gate
failure**, not an advisory (`frozen-documentation-must-pin/spec.md` EC-007). AC-016 requires
`cargo xtask ci` green on the exact final tree (`.redkiln/config.yaml:60`), and
`terminal-gate-run` is the last story in this project by construction
(`_storymap.md:156-160`). Therefore, while the pin is live, "routed" can never mean
"deferred past the terminal gate": an addition must be disposed of in the enumeration
before `terminal-gate-run` can be green, and the routed arm covers the disposition's
*follow-up work*, not the classification itself. State that relationship in the record
rather than leaving a later reader to discover it when the gate goes red.

**7. Everything is observed against the merged sha, and the record names it.** Ordering 1
of the architecture brief's data flow — merge before everything — is the reason this
milestone exists (`_decomposition.md:630-632`; AC-A06 at `:383-387`). AC-012 requires both
this re-check and the reconciliation record to name the merged tree by ref or sha
(`project.md:201`), and AC-UX-12 requires it so a later reader can re-run them
(`_decomposition.md:278-281`). Nothing here is observed against this planning worktree's
pre-merge copy, and the sha comes from `merge-forward-baseline`, not from a second merge.

**8. The default shape of the record is "no addition", stated positively and in both
directions.** The most likely outcome is that the merge adds nothing and the pin is still
complete. That case is not an empty section: it is the full set difference computed and
shown to be empty on both sides — no candidate the enumeration does not classify, and no
enumerated entry that is no longer a candidate. Those two directions are different
sentences about different bugs (the document grew; the enumeration went stale), and the
sibling's checker already refuses to merge them into "the counts disagree"
(`frozen-documentation-must-pin/spec.md` EC-007, EC-008). The written record inherits that
discipline.

**9. The reader is the closeout reviewer, and the record is written for one-row-at-a-time
reading.** The audience for this project's artefacts is U2, the closeout reviewer, and U1,
the next initiative's charter author — not the three documented personas, who are its
subject (`_storymap.md:37-39`). Three obligations fall out and are not stylistic: every row
is self-contained, naming its clause id, its disposition and its evidence with no
dependence on the row above (AC-UX-10, `_decomposition.md:270-273`); every state is a
**word**, never a tick, an emoji, a colour, a strikethrough or an empty cell (AC-UX-09,
`:266-269`, `:120-130`); and the load-bearing verdict is also stated in a **sentence**,
because a verdict that exists only as a table cell is lost in a quote or a diff
(`:142-145`).

**10. No decision atom is authored or touched, and no ADR is written as a by-product.**
No Accepted decision atom governs documentation or gate structure — sixteen govern the Rust
contract and the charter says so outright (`_grounding.md:18-31`;
`initiative.md:524-525`). Citing one here would be manufacturing authority. What binds
instead is `.kb/governance/rewrite-the-referent-never-the-reasoning.md`: a correction to a
standing decision is a new atom that supersedes, never an edit — and the same atom forbids
inventing a decision as closeout exhaust (AC-A09, `_decomposition.md:396-400`). If this
re-check surfaces something that wants a decision, it is a routed gap written in prose.

## Integration contract

- **Archetype**: `capability` (`story.md` frontmatter, `archetype: capability`;
  `_storymap.md:49`). Its observable output is a written record a human reads, not a `pub`
  item — which is what `_design.md` means by declaring no items at all.
- **Slice / milestone**: **`merged-tree-baseline`**. Slice-mate, implemented in the same
  context: **`merge-forward-baseline`** (foundation, first in merge order). This story is
  second and last in the milestone (`_storymap.md:144-147`). The two are one surface because
  "the sha the merge produces is the sha the clause-completeness statement must name"
  (`:62-65`) — splitting them yields a merge with nothing observed on it.
- **Mount point**:
  **`.bklg/docs-that-teach/durable-audience-closeout/post-merge-clause-completeness/_ledger.md`**
  — this story's own evidence ledger, which `require_ledger: true`
  (`.redkiln/config.yaml:67`) makes mandatory for any story declaring `AC-###` and which
  `redkiln verify` reads. The completeness statement itself is authored as a companion in
  this same directory (working name `_clause-completeness.md`; the file name is the
  implementer's, the directory is not), and the ledger is the file that cites it per
  criterion with a real `file:line`. That citation is the mount: a statement written into
  the story folder and referenced by nothing is this project's analogue of a component
  rendered into no tree (`_decomposition.md:429-436`).
  **Downstream mount, load-bearing and named here so it cannot be missed**:
  `dod-scenario-ledger` is `blocked_by` this story precisely so its **DoD-11 row** can cite
  this statement as scenario 11's fresh observation (`_storymap.md:56`;
  `initiative.md:452-454`). Write the statement so a single row of that ledger can cite it
  by path and line without paraphrasing it.
- **Wires into** (all read-only unless stated):
  - **The merged commit** produced by `merge-forward-baseline` — the sha this record names.
    Consumed as a value, not re-derived: this story runs no `git merge`.
  - **`cargo xtask spec-trace`** as already wired: `xtask/src/main.rs:315` (`name:
    "specification traceability"`), args at `:317-326`, inside the `REQUIRED` step list at
    `:105`. Invoked, never modified; the dispatch arm and `print_help` line already exist
    (`xtask/src/main.rs:671-675`, `:738`).
  - **HS-P0020's single enumeration of the frozen documentation MUSTs** — the `const` its
    `frozen-documentation-must-pin` story lands, together with its derivation rule and its
    derived candidate scan. Located on the merged tree by search (planned path
    `xtask/src/lint_narrative.rs`; see context decision 4), read as text, never copied into
    a second list.
  - **`spec/SPECIFICATION.md`** — read by the tooling above and by the implementer for the
    clause text behind any candidate; `:280` is the clause that governs the method.
  - **`xtask/src/spec_trace.rs`** — the module whose docs state what `spec-trace` does and
    does not check; read to say honestly what the green run proves.
  - **The reachability grain** `cargo xtask lints && cargo xtask spec-trace`
    (`.redkiln/config.yaml:48`) — the Tier 3 instrument the testing brief assigns to AC-013
    (`_decomposition.md:873`), run corpus-wide rather than diff-scoped.
  - No workspace crate, no port, no `Send` bound, no feature, no dependency: ADR-0001 and
    ADR-0003 are untouched by construction, and no crate is compiled that was not compiled
    before.
- **Renders surfaces**: **none.** `_design.md` records `# no items — no public API surface,
  no rendered UI surface` (`:45-47`) and states the project's user-facing surface is four
  markdown artefacts (`:26-35`). This story produces one instance of artefact (3)-class —
  a written record — and changes no signed-off surface. The design's accessibility floor and
  interaction-quality invariants still bind it as markdown (context decision 9); "no surface"
  is not "no obligations".
- **Public items**: none. No `pub` item is added, changed or removed, and no row of
  `_design.md`'s `## Items` block is claimed, because that block is empty by decision.
- **Conformance rule(s)**: **none, and this is not adapter-observable.** Nothing here
  touches `crates/happenstance-testkit/`, a port, a value type or a fixture. Stated
  explicitly because a story that changes a port and names no rule is a port change nothing
  can fail — this changes no port and compiles no Rust. The instruments are the already-wired
  gate step, the pin's own tests (HS-P0020's), and a human reading the statement.
- **Clause(s)**: **none discharged, none amended.** This story *observes* the frozen
  documentation MUSTs; it does not restate, extend, reinterpret or renumber any clause, and
  `spec/SPECIFICATION.md` is read-only here (`_decomposition.md:422-427`). No `[FROZEN]`
  clause is changed, so no ADR is owed. The one clause that governs the method rather than
  being touched by it is `spec/SPECIFICATION.md:280` — clause ids are stable and are never
  renumbered.
- **Advances DoD scenario**: initiative DoD **11** — *"The frozen documentation MUSTs are
  still discharged. The pinned set of clause ids is enumerated in one place, and the
  specification cross-reference step passes over the tree as it stands after every doc
  comment this work touched"* (`initiative.md:452-454`). HS-P0020 turned scenario 11 from an
  assertion into a check; this story is the re-observation of that check on the assembled
  tree, and it is the evidence `dod-scenario-ledger`'s DoD-11 row cites. It also feeds the
  initiative's closing criterion at `:662-663`.

## PR boundary

The paths this story is allowed to touch, as globs. `redkiln verify --grain story` reads the
first fenced block under this heading and fails on any file changed outside it.

```
.bklg/docs-that-teach/durable-audience-closeout/post-merge-clause-completeness/**
```

One glob, and it is honestly narrow rather than narrowed to look good: this story writes a
record and runs two commands. The merge itself belongs to `merge-forward-baseline` and its
boundary; the pin belongs to HS-P0020's.

**In this PR**

- The completeness statement, a companion under this story's directory: the merged sha it
  was taken against, the located enumeration cited at its real path and line, the set
  difference in both directions, one self-contained row per clause id, the disposition of
  every difference, and the verdict repeated as a sentence.
- The verbatim outcome of `cargo xtask spec-trace` on the merged tree — the command line,
  the tree it ran in, and what it printed — recorded as evidence rather than summarised as
  "green".
- Where the enumeration was found, and by what search, so a later reader can repeat the
  location step without knowing HS-P0020's internals.
- The honest statement of what a green `spec-trace` does **not** prove — that it is a
  document-internal consistency check, not a judgement that a doc comment teaches — read off
  `xtask/src/spec_trace.rs`'s own module documentation rather than asserted.
- This story's `_ledger.md` (second pass) and its implementation report.

**Explicitly not in this PR**

- **Any edit to `spec/SPECIFICATION.md`.** No clause is added, amended, restated or
  renumbered.
- **Any edit to the pin, to `xtask/`, to `crates/`, to `standards/` or to `docs/`.**
  Classifying a newly discovered candidate inside the enumeration is HS-P0020's act, not
  this project's (`project.md:102-105`); this story hands it back and records the hand-off.
- **A second enumeration of the frozen documentation MUSTs**, in code or in markdown, in
  any shape that could later drift from the one in the tree (context decision 3).
- **`cargo xtask spec-trace --write`**, or any regeneration of the §7.1–§7.2 region.
- **The merge itself**, the sha's derivation, and any second merge — `merge-forward-baseline`
  owns them and this story consumes the result.
- **The fifteen-scenario DoD ledger**, scenario 2's fault injection and the terminal
  `cargo xtask ci` run — `dod-scenario-ledger`, `scenario-two-fault-injection` and
  `terminal-gate-run` (`_storymap.md:56-58`). This story supplies scenario 11's evidence; it
  does not open the ledger.
- **The reconciliation record and the DT audit** — `reconciliation-ledger` and
  `design-tension-audit`, in the next milestone.
- **Any `.kb/` write.** No atom, no map append, no `_intake/` staging: those belong to the
  `product-layer-promotion` milestone, and `.kb/decisions/` is untouched by anyone
  (AC-A09).
- **An ADR**, in any form, as a by-product of a finding.

The implementer MAY touch the wiring named in the Integration contract to mount this slice.
Here that wiring is this story's own `_ledger.md` and the companion record it cites; both
are inside the single glob above, and citing the statement from the ledger is the point of
the story rather than scope drift.

**Merge DoD**: `cargo xtask spec-trace` has been run on the merged tree and its output is
recorded verbatim; the completeness statement exists, names the merged sha, cites the located
enumeration at a real path and line, and shows the set difference in both directions; every
difference carries a disposition with a named owner and destination; `cargo xtask affected
--base main` (`.redkiln/config.yaml:40`) is green on the prose-only diff; and no file outside
`.bklg/docs-that-teach/durable-audience-closeout/post-merge-clause-completeness/**` is
changed.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The tree observed is the merged one, named by sha | The statement opens by naming the merge commit `merge-forward-baseline` produced, and the checkout it was observed in. Nothing is observed against this planning worktree's pre-merge copy; every downstream record in this project names the same sha. | `_decomposition.md:383-387` (AC-A06), `:630-632` (ordering 1); `project.md:201` (AC-012); `_decomposition.md:278-281` (AC-UX-12) |
| `cargo xtask spec-trace` is invoked as already wired, and passes | The step exists in the `REQUIRED` list — `name: "specification traceability"` at `xtask/src/main.rs:315`, args `:317-326`, list head `:105`. It is run, not modified; no new step, subcommand, `print_help` line or `REQUIRED` entry is added. Its output is recorded verbatim, not summarised as "green". | `xtask/src/main.rs:105`, `:315`, `:317-326`, `:671-675`; `_decomposition.md:649-651` |
| What the green run is allowed to be read as proving | `spec-trace` checks the specification's internal cross-references — that each clause's claimed rule and case exist and resolve. It is not a judgement that a doc comment discharges its obligation well, and the statement says so in its own words rather than letting green stand for more than it is. | `xtask/src/spec_trace.rs` module docs; `xtask/src/main.rs:304-317` (the step's own comment) |
| The comparison is a set difference over clause ids | Ids are stable and never renumbered, so the comparison is over names, never over line numbers or a `git diff` of the specification. A 521-line divergence cannot invalidate an entry; it can only add one. | `spec/SPECIFICATION.md:280`; `_decomposition.md:516-518`, `:653-655`; `project.md:248` |
| The enumeration is located, not reconstructed | The one in-tree enumeration is HS-P0020's; the statement cites it at the path and line it occupies on the merged tree and records how it was found. Planned landing site `xtask/src/lint_narrative.rs` does not exist in this worktree today, so location is a step with an outcome, not an assumption. | `checked-documentation-surface/project.md:223-226`; `checked-documentation-surface/frozen-documentation-must-pin/spec.md`; `xtask/src/` listing |
| No second enumeration is created, in code or in prose | The statement's table cites ids read from the one enumeration; it does not restate the set as an authority of its own, and it adds no regex, prefix list or parser. A second list is the defect the sibling's design already refuses. | `checked-documentation-surface/frozen-documentation-must-pin/spec.md` (`## Behavior and interfaces`, "Assertion 1"); `checked-documentation-surface/spec-trace-clause-id-accessor/spec.md` |
| Both directions of the difference are stated, as different sentences | *A candidate the enumeration does not classify* means the document grew; *an enumerated entry that is no longer a candidate* means the enumeration went stale. Two bugs, two sentences, ids named on each side — never "the sets disagree" and never a bare count. | `frozen-documentation-must-pin/spec.md` EC-007, EC-008; `standards/rust/81-checks-that-cannot-be-types.md:335-341` |
| The empty result is a stated result, not an omitted section | If nothing was added, the record still shows both directions computed and empty, with the enumeration's entries listed as still present and still discharged. An absent section reads identically to a check nobody ran. | `initiative.md:452-454`; `_decomposition.md:651-655` |
| Every difference carries a disposition, with an owner and a destination | Two admissible values: **pinned** — handed back to the single enumeration, HS-P0020's act, with the classifying change identified; or **routed** — an item under the `support` initiative, with the reason it could not be pinned inside this closeout. "Noted" is not a disposition. | `project.md:102-105`, `:202` (AC-013); `.redkiln/config.yaml:5`; `_decomposition.md:422-427` |
| The gate consequence of a live addition is stated in the record | While the pin's candidate scan is live, an unclassified candidate fails the gate — so an addition must be classified before `terminal-gate-run` takes `cargo xtask ci`, and the routed arm covers follow-up work rather than the classification. Written down so a later reader meets it before the gate goes red. | `frozen-documentation-must-pin/spec.md` EC-007; `.redkiln/config.yaml:60`; `_storymap.md:156-160`; `_decomposition.md:638-640` (ordering 4) |
| Every row is self-contained and every state is a word | Each row names its clause id, its side of the difference, its disposition and its evidence, and is legible read alone. No tick, emoji, colour, strikethrough, ordering or empty cell carries meaning; the corpus precedent is the open-questions index's literal status words. | `_decomposition.md:266-273` (AC-UX-09, AC-UX-10), `:120-130`; `.kb/maps/open-questions-index.md:136-143` |
| The verdict is also a sentence | The load-bearing outcome — the pin is complete, or these ids were added and were disposed of thus — appears in prose as well as in the table, because a verdict that exists only in a cell is lost in a quote or a diff. | `_decomposition.md:142-145` |
| Nothing this story writes moves an anchor someone else cited | The statement is a new file in this story's directory. No existing section is reflowed, no id renamed, no cited `file:line` shifted — IQ-3, and the same reason the comparison is over ids rather than positions. | `_decomposition.md:180-187` (IQ-3) |
| No decision atom, and no ADR as a by-product | A finding that looks like it wants a decision is recorded as a routed gap in prose. No file under `.kb/decisions/` is added or modified, which `redkiln validate --kb`'s immutability check would catch anyway. | `.kb/governance/rewrite-the-referent-never-the-reasoning.md`; `_decomposition.md:396-400` (AC-A09), `:892-897` |
| The statement is reachable from the ledger and citable by one downstream row | Cited from this story's `_ledger.md` per criterion with a real `file:line`, and written so `dod-scenario-ledger`'s DoD-11 row can cite it without paraphrase. | `.redkiln/config.yaml:67`; `_storymap.md:56`, `:94-98`; `_decomposition.md:429-436` |

## Data and migrations

**N/A, in the strict sense: no schema, no store, no persisted state, no serialised
envelope, no released artifact, and no `.kb/` frontmatter.** This story compiles no Rust —
`_design.md` records that this project changes no public API and adds no `pub` item
(`:10-20`) — so there is no semver promise and nothing in anyone's dependency graph to
migrate.

Two adjacent facts are worth stating so they are not mistaken for migrations.

**The specification is read, never written.** No `cargo xtask spec-trace --write` is run and
the generated §7.1–§7.2 region is untouched, so the committed-versus-computed equality the
gate already checks is unaffected. The one *document* this story could have been tempted to
migrate — `spec/SPECIFICATION.md` — is read-only by the architecture brief's diff surface
(`_decomposition.md:422-427`), and pinning a newly discovered documentation MUST is
HS-P0020's act rather than an edit made here.

**Clause ids are stable, so there is no renumbering to absorb.** `spec/SPECIFICATION.md:280`
makes ids names rather than positions; the 521-line divergence between this branch and
`initiative/from-contract-to-published-library` therefore cannot require any record in this
tree to be rewritten. That property is what turns the whole risk into a one-directional
set difference — an addition — and it is why this story is a written comparison rather than
a data migration.

## Acceptance criteria

Six criteria, framed from the intent of the two readers this project actually has — **U2**,
the closeout reviewer at the ingest approval gate and at the pull request, and **U1**, the
next initiative's charter author (`_decomposition.md:43-56`). Every one is observable by a
reviewer with the merged tree in front of them, and together they cover the project criteria
this story traces to: **AC-013** (`project.md:202`) and **AC-012** (`project.md:201`).

The working name for the companion record is `_clause-completeness.md`, in this story's own
directory. The file name is the implementer's to change; the directory and the fact that
`_ledger.md` cites it are not.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** a closeout reviewer (U2) handed a merge commit whose sibling diverges from `spec/SPECIFICATION.md` by 521 lines (`_decomposition.md:89-103`), who must decide whether the specification's cross-references still resolve *on that tree* rather than on the one planning ran in, **WHEN** they open `_clause-completeness.md`, **THEN** its first screen names the merge commit by **sha** and names the checkout it was observed in, and carries the outcome of `cargo xtask spec-trace` on that tree **verbatim** — the command line, the tree, and what it printed — never the word "green" standing in for a transcript. **AND** the record states in its own words what a passing run does *not* prove — that it is a document-internal cross-reference check, not a judgement that a doc comment discharges its obligation well — read off `xtask/src/spec_trace.rs`'s module documentation rather than asserted. *(serves AC-013, AC-012; AC-A06, AC-UX-12)* | **Tier 3** — `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:48`, the `reachability_static` grain the testing brief assigns to AC-013 at `_decomposition.md:873`) run corpus-wide on the merged tree, exit zero, transcript pasted into `_clause-completeness.md`. **Tier 1** — `rg -n "<sha>" .bklg/docs-that-teach/durable-audience-closeout/post-merge-clause-completeness/` finds the sha in the record, and `git cat-file -t <sha>` resolves it to a real commit (AC-012's "also" tier, `_decomposition.md:872`). **Tier 2** — reviewer confirms the limits paragraph exists and matches `xtask/src/spec_trace.rs`'s module docs. |
| AC-002 | **GIVEN** a reviewer who must trust that "HS-P0020's pinned set" is a real object in the tree and not a phrase inherited from a plan — the planned landing site `xtask/src/lint_narrative.rs` does not exist in this worktree today — **WHEN** they follow the record's citation, **THEN** it lands on the **one** enumeration (`checked-documentation-surface/project.md:223-226`) at the path and line it actually occupies on the merged tree, and the record states the search that found it so a later reader can repeat the location step without knowing HS-P0020's internals; **and if it is absent**, that is recorded as a named finding, with the candidate set derived against the stated derivation rule **for the record only**. **AND** the diff introduces no second enumeration of the set — no regex, no prefix list, no hand-copied id table presented as authoritative — because a fourth list of the clause families is the one that can half-land (`frozen-documentation-must-pin/spec.md` AC-002). *(serves AC-013)* | **Tier 1** — the cited path passes `test -f` on the merged tree and the cited line contains the enumeration; `rg -n "lint_narrative" xtask/` reproduces the location step. **Tier 2** — reviewer confirms the record's table cites ids *read from* that enumeration and claims no authority of its own; `rg -n "^\|\s*(ES|VT|CB|AP)-" .bklg/docs-that-teach/durable-audience-closeout/post-merge-clause-completeness/` shows ids only inside rows that name their source, and the diff contains no clause-id regex or prefix array. |
| AC-003 | **GIVEN** U2, who must be able to tell *the document grew* from *the enumeration went stale* — two different bugs with two different owners — **WHEN** they read the set difference, **THEN** they meet **two separate sentences**, each naming the clause ids on its side: candidates the enumeration does not classify, and enumerated entries that are no longer candidates. Never "the sets disagree", never a bare count. **AND** when nothing was added — the expected outcome — the record still shows **both** directions computed and empty, with the enumeration's entries listed as still present and still discharged, because an absent section reads identically to a check nobody ran. *(serves AC-013)* | **Tier 2** — content review against `frozen-documentation-must-pin/spec.md` AC-005 and EC-007/EC-008, and against `standards/rust/81-checks-that-cannot-be-types.md:335-341` (RS-81-5, "name which side moved"). **Tier 1** — `rg -n "sets disagree\|counts disagree" ` over the story directory returns nothing; both direction headings are present under `rg -n "^#+ "` whether or not either is populated. |
| AC-004 | **GIVEN** the residual risk handed forward to this project **by name** at HS-P0020's own boundary (`checked-documentation-surface/project.md:273-279`), **WHEN** the reviewer reads any difference the statement found, **THEN** every one carries a disposition that is exactly one of two words with an owner and a destination attached: **pinned** — handed back to the single enumeration as HS-P0020's act, with the classifying change identified — or **routed** — an item under the `support` initiative (`.redkiln/config.yaml:5`), named together with the reason it could not be pinned inside this closeout. "Noted for a future pass" is not a disposition and appears nowhere. **AND** the record states the gate consequence a later reader would otherwise meet as a red build: while the pin's derived candidate scan is live an unclassified candidate is a **gate failure** (`frozen-documentation-must-pin/spec.md` EC-007), so an addition must be classified before `terminal-gate-run` takes `cargo xtask ci` (`.redkiln/config.yaml:60`; `_storymap.md:156-160`) — the routed arm covers the follow-up work, never the classification itself. *(serves AC-013)* | **Tier 2** — reviewer reads every difference row's disposition cell and confirms it names an owner and a destination. **Tier 1** — `rg -ni "noted for\|future pass\|TBD\|to be decided"` over the story directory returns nothing in a disposition cell. **Tier 4 (downstream, not this PR)** — if a difference exists and is unclassified, `cargo xtask ci` is red, which `terminal-gate-run` observes under AC-016. |
| AC-005 | **GIVEN** U2 reading one row at a time inside a pull-request diff, in a corpus whose only automated reader is `redkiln validate --kb`'s frontmatter check, **WHEN** they read any single row of the record out of context, **THEN** it names its clause id, its side of the difference, its disposition, its owner or destination and its evidence, and loses nothing (AC-UX-10, `_decomposition.md:270-273`); **AND** every state anywhere in this story's diff is a literal **word** — no ✅/❌, no colour, no glyph, no strikethrough, no ordering and no empty cell carries meaning (AC-UX-09, `:266-269`, `:120-130`), the shape `.kb/maps/open-questions-index.md:136-143` already fixes; **AND** the load-bearing verdict is stated as a **sentence** outside the table as well as inside it (`_decomposition.md:142-145`); **AND** the record is *composed* from the corpus's own primitives rather than dumped — one `#`, sections at `##` with no skipped level, every link naming its destination (WCAG 2.4.4, `:132-136`), one table of at most five columns with no nested table, and prose wrapped to the ≤ 97-column width this spec's own non-table body already uses. *(serves AC-013; discharges AC-UX-09, AC-UX-10 and the accessibility floor for this artefact)* | **Tier 2** — one checklist pass against `_decomposition.md:115-145` (accessibility floor) and AC-UX-09/AC-UX-10; the same checklist the UX brief owns, not a second one (AC-TB-06). **Tier 1** — `rg -n "✅\|❌\|🟢\|~~"` over the story directory returns nothing; `rg -n "^#+ "` shows one `#` and no skipped level; `rg -n "\[here\]\|\[this\]\|see above"` returns nothing. |
| AC-006 | **GIVEN** the downstream reader this milestone exists to serve — `dod-scenario-ledger`, held `blocked_by` this story precisely so its **DoD-11** row can cite scenario 11's fresh observation (`_storymap.md:56`; `initiative.md:452-454`) — **WHEN** that ledger's single row is written, **THEN** it can cite this statement by path and `file:line` **without paraphrasing it**, because the statement is reachable from this story's `_ledger.md` (mandatory under `require_ledger: true`, `.redkiln/config.yaml:67`) with a real `file:line` per criterion rather than a bare filename. **AND** nothing this story writes moves an anchor someone else already cited: the diff adds files under `.bklg/docs-that-teach/durable-audience-closeout/post-merge-clause-completeness/**` and reflows no pre-existing section (IQ-3, `_decomposition.md:180-187`). *(serves AC-012, AC-013; AC-A06)* | **Tier 3** — mount-point walk: open `_ledger.md`, follow each row's `evidence` `file:line`, land inside `_clause-completeness.md` on the sentence it claims. **Tier 1** — `git diff --name-only` lists no path outside the story glob; `git diff` shows no `-` line in any pre-existing file; `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) green on the prose-only diff. |

## Interaction quality

This story renders **no screen**: `_design.md` records `# no items — no public API surface,
no rendered UI surface` (`:45-47`) and the project's user-facing surface is four markdown
artefacts (`:26-35`). "No surface" is not "no obligations" — the UX brief's invariants are
written for exactly this medium and bind here as written. Every invariant below is carried by
an **AC-### row in the table above**; this section only says which row carries it and how it
is falsified. Nothing here is a free-standing bullet, because a bullet in this section gets no
ledger row and is therefore never gated.

**State invariants**

| Invariant (UX brief id) | Carried by | Falsifier |
| --- | --- | --- |
| **In place, not a context jump** (IQ-1, `_decomposition.md:153-158`) | AC-001, AC-003 | A record whose verdict or whose set difference reads "see HS-P0020's pin" or "see the discovery corpus". The whole point of U2's existence is that they should not have to open another project's internals to learn whether this one is complete. |
| **Non-occlusion — a summary may precede a ledger, never replace it** (IQ-2, `:160-168`) | AC-003, AC-005 | A record that states "the pin is still complete" and shows no enumeration of what was compared, or that lists only the differences and omits the entries that were unchanged. |
| **Preserved position — the text analogue of preserved focus and scroll** (IQ-3, `:180-187`) | AC-006 | `git diff` showing a `-` line anywhere outside this story's directory; any reflow that shifts a `file:line` another artefact cites. This is the same property that makes the comparison a set difference over ids rather than over positions. |
| **Reversibility** (IQ-4, `:189-199`) | AC-002, AC-004 | A record that classifies a candidate itself instead of handing it back — an act this project cannot undo, because the enumeration is HS-P0020's and correction there is by supersession, not edit (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`). The routed arm exists so that the *only* irreversible-looking option always has a named destination. |
| **Reachable without prior knowledge — the keyboard-reachability analogue** (IQ-5, `:201-205`) | AC-006 | A statement findable only by knowing the story slug: not cited from `_ledger.md`, or cited by filename with no line, so `dod-scenario-ledger`'s DoD-11 row has to paraphrase it. |
| **Preserved selection — prior citations keep meaning what they meant** (IQ-6, `:207-212`) | AC-002 | A second enumeration in this record, which forks the set: a reader who cites "the pinned set" then has two objects to mean and no way to tell which. |

**Composition invariants** — taken from the signed-off `_design.md` and the primitive
inventory the UX brief fixes (`_decomposition.md:74-95`). There is no CSS in this repository
and the brief says so outright; the honest primitive layer for a text corpus is the heading
skeleton, the map/index row formats and the status-word vocabulary, and inventing a shape
beside them is this corpus's equivalent of bespoke CSS.

| Invariant | Carried by | The real bar |
| --- | --- | --- |
| **Presentation exists at all** — the record is composed, not a dumped transcript | AC-005, AC-001 | A verdict sentence, a two-direction difference under named headings, a disposition table, and the raw `spec-trace` output in a fenced block. A file that is only the pasted terminal output satisfies every "the command was run" assertion and fails this. |
| **Composition and placement** | AC-006 | The record lives in this story's own directory and is mounted by citation from `_ledger.md`; a record referenced by nothing is this project's analogue of a component rendered into no tree (`_decomposition.md:429-436`). |
| **Transience** — persistent chrome vs revealed vs opened on demand | AC-001, AC-005 | Persistent: the merged sha and the verdict sentence, at the top, met on first load. Revealed: the per-clause disposition table. Opened on demand: the verbatim `spec-trace` transcript, last, under its own heading — evidence a reader goes to, never something interleaved with the verdict. |
| **Density budget, with its real numbers** | AC-005 | **One** table for the difference (five columns: clause id, side, disposition, owner/destination, evidence); no nested table; one `#` and headings no deeper than `###`; prose wrapped at ≤ 97 columns, the measured width of this spec's own non-table body lines. A sixth column or a second parallel table is a re-enumeration wearing a layout. |
| **Hierarchy** | AC-005 | Verdict first, then the two directions, then dispositions, then evidence. Hierarchy is carried by position and heading level alone — no bold-as-status, no colour, no glyph (`_decomposition.md:120-130`). |
| **The design's named anti-patterns** (`_decomposition.md:100-107`) | AC-005, AC-002, AC-003, AC-004 | A bespoke per-artefact table vocabulary instead of the corpus's row shape (AC-005); a status carried by an emoji, a tick, a colour word or an empty cell (AC-005); a hand-written enumeration standing beside the one in the tree (AC-002); an omitted section standing for an empty result (AC-003); "noted" standing for a disposition (AC-004). |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| EC-001 | The merge commit does not exist — `merge-forward-baseline` has not landed, or the working tree is this planning worktree's pre-merge copy | **Halt loudly and do not proceed.** Every observation in this story is defined against the merged sha (`_decomposition.md:383-387`, ordering 1 at `:630-632`). A statement written against the pre-merge tree is worse than no statement, because it reads identically to a correct one. Report the missing dependency rather than running a second merge — the merge is `merge-forward-baseline`'s act. |
| EC-002 | The located enumeration is **absent** on the merged tree — HS-P0020 landed a different shape, or has not landed | Record it as a **named finding** in the statement, with what was searched for and where. Derive the candidate set against the stated derivation rule — (a) `[FROZEN]` and (b) the obligation falls on the contract's own documentation — **for the record only**, explicitly marked as a derivation rather than an enumeration, and add no code. Route the gap under the `support` initiative per AC-004. Do not create the enumeration here: that is HS-P0020's, and creating it is the second-list defect (AC-002). |
| EC-003 | `cargo xtask spec-trace` **fails** on the merged tree | Record the failure verbatim — this is the outcome the check exists to be able to produce, and a story that can only report green is decorative. Then classify it: a broken cross-reference is a routed defect with an owner, not something this story repairs inside `spec/SPECIFICATION.md`, which is read-only here (`_decomposition.md:422-427`). AC-013 requires the run to pass on the assembled tree, so the story is not done — it is *blocked with a named cause*, which is a different thing from failing silently. |
| EC-004 | The difference is non-empty in the **addition** direction: the merge brought a candidate the enumeration does not classify | Two consequences, both stated. First: while the pin's candidate scan is live this is already a **gate failure** (`frozen-documentation-must-pin/spec.md` EC-007), so it must be classified before `terminal-gate-run` (AC-016). Second: the classification is HS-P0020's act — hand it back with the classifying change identified (**pinned**), or route the follow-up under `support` with the reason (**routed**). Never classify it inside this project's diff. |
| EC-005 | The difference is non-empty in the **staleness** direction: an enumerated entry is no longer a candidate | A different sentence and a different owner from EC-004. The document did not grow; the enumeration drifted. Record both ids and both attributions; do not collapse the two into one row or one count (`standards/rust/81-checks-that-cannot-be-types.md:335-341`). |
| EC-006 | A difference cannot be disposed of inside this closeout at all | The **routed** arm, with its destination named: an item under the `support` initiative (`.redkiln/config.yaml:5`) and the reason it could not be pinned here. Silence, "noted", and an empty disposition cell are each a defect under AC-004. |
| EC-007 | A finding looks like it wants a **decision** | Write it as a routed gap in prose. No file under `.kb/decisions/` is added or modified, and no ADR is authored as closeout exhaust (AC-A09, `_decomposition.md:396-400`; `.kb/governance/rewrite-the-referent-never-the-reasoning.md`). `redkiln validate --kb`'s accepted-decision immutability check would catch a stray touch structurally (`_decomposition.md:892-897`), which is why no dedicated check is written for it here. |
| EC-008 | The statement is written but nothing cites it | A record in a story directory referenced by nothing is unmounted (`_decomposition.md:429-436`). `_ledger.md` must cite it per criterion with a real `file:line`; AC-006 is the row that fails. |

## Non-functional

| id | Requirement | Why it is real here |
| --- | --- | --- |
| NF-001 | **No compilation cost is added.** No crate, feature, dependency or gate step is introduced; `cargo xtask spec-trace` runs as already wired (`xtask/src/main.rs:315`, `:317-326`, list head `:105`), and nothing is compiled that was not compiled before. | The gate's runtime is a shared budget every story spends from. A closeout re-check that adds a step makes every future run slower for one observation taken once. |
| NF-002 | **Reproducibility from the record alone.** A reader with the sha, the command line and the located enumeration's path can re-run the whole observation without this spec, this story or any knowledge of HS-P0020's internals. | AC-UX-12's stated reason for naming the tree: *so a later reader can re-run them* (`_decomposition.md:278-281`). |
| NF-003 | **No anchor churn.** The diff adds files; it does not reflow, renumber or re-title anything a sibling artefact cites by `file:line`. | IQ-3 (`:180-187`), and the same property — stable names over positions — that makes the whole comparison possible. |
| NF-004 | **No invented tooling.** Every command named in this spec resolves to a real entry in `.redkiln/config.yaml` or `xtask/src/main.rs`, or to a documented CLI verb in CLAUDE.md's "Commands". | AC-TB-02 (`_decomposition.md:827-829`). A record citing a command that does not exist cannot be re-run, which defeats NF-002. |
| NF-005 | **No `.kb/` write of any kind** — no atom, no map append, no `_intake/` staging. | Those belong to the `product-layer-promotion` milestone (`_storymap.md:53-55`); `.kb/decisions/` is untouched by anyone in this project (AC-A09). |
| NF-006 | **The record is legible in a diff and in a quote.** Load-bearing claims survive being extracted from their table. | `_decomposition.md:142-145` — a verdict that exists only as a table cell is a verdict that will be lost in a quote or a diff. |

## Implementation notes (non-prescriptive)

Shape suggestions, not instructions. The bar is the acceptance table; how you get there is yours.

- **Order that costs nothing to get right.** Confirm the merged sha first (`git rev-parse HEAD`
  in the merged checkout, cross-checked against `merge-forward-baseline`'s recorded sha), then
  locate the enumeration, then run `spec-trace`, then write. Writing before locating is how a
  planned path gets cited as a real one.
- **Locating the enumeration is a step with an outcome, not an assumption.** `xtask/src/`
  currently holds `affected.rs`, `constitution.rs`, `lib.rs`, `lint_constitution.rs`,
  `lints.rs`, `main.rs`, `package.rs`, `proof.rs`, `reserve.rs`, `spec_trace.rs` — no
  `lint_narrative.rs`. Search the merged tree (`rg -n "lint_narrative" xtask/`, then read the
  module) and record the search alongside the result, so a later reader repeats the *step*
  rather than trusting your answer.
- **Run the reachability grain, not just the one step.** `.redkiln/config.yaml:48` pairs
  `cargo xtask lints` with `cargo xtask spec-trace`; running the pair is what the testing brief
  assigns to AC-013 (`_decomposition.md:873`), and `lints` is cheap.
- **Quote the transcript; do not summarise it.** Paste the command line, the working directory
  or checkout name, and the output. "Green" is the summary the record exists to replace.
- **Write the empty result as a result.** If nothing was added — the expected case — the record
  is not shorter by a section. Both directions are computed and shown empty, and the
  enumeration's entries are listed as still present and still discharged.
- **Say what green does not prove in your own words.** Read `xtask/src/spec_trace.rs`'s module
  documentation and the step's own comment (`xtask/src/main.rs:304-317`) and paraphrase
  honestly. Pasting the module docs verbatim would be a second copy that later drifts; the
  point is a sentence a reader can act on.
- **Keep the two directions in two headings.** It is the cheapest way to make the "which side
  moved" discipline structural rather than a promise, and it means an empty direction is
  visibly empty rather than absent.
- **The ledger citation is the mount.** Cite `_clause-completeness.md` with a real `file:line`
  per criterion — the line the claim is on, not the file's first line. `dod-scenario-ledger`'s
  DoD-11 row will copy your citation, so write the statement's sentences to be citable whole.

## Tests and CI (merge gate)

Tiers are the testing brief's (`_decomposition.md:704-711`); AC-013's assignment is Tier 3
primary with Tier 2 for the written statement (`:873`), and AC-012's is Tier 1 with Tier 4 as
the "also" (`:872`). **No new Rust test coverage is created or implied** — AC-TB-08 (`:846-851`)
— because this story compiles nothing.

| Tier | Command or path | Proves |
| --- | --- | --- |
| **1 — static / shape** | `rg -n "<sha>"` over the story directory; `git cat-file -t <sha>` | AC-001: the record names the merged tree, and the sha it names resolves to a real commit rather than a plausible-looking string (`_decomposition.md:872`). |
| **1 — static / shape** | `test -f` on the enumeration path the record cites; `rg -n "lint_narrative" xtask/` | AC-002: the citation lands on a file that exists on the merged tree, and the recorded search reproduces the location step. |
| **1 — static / shape** | `rg -ni "sets disagree\|counts disagree\|noted for\|future pass\|TBD"` over the story directory | AC-003, AC-004: no collapsed-difference sentence, and no non-disposition standing in for a disposition. |
| **1 — static / shape** | `rg -n "✅\|❌\|🟢\|~~"` and `rg -n "^#+ "` over the story directory | AC-005: no state carried by glyph, colour or strikethrough; one `#`, no skipped heading level (`_decomposition.md:120-136`). |
| **1 — static / shape** | `git diff --name-only` and `git diff` | AC-006, NF-003, NF-005: nothing changed outside `.bklg/docs-that-teach/durable-audience-closeout/post-merge-clause-completeness/**`, no `-` line in any pre-existing file, no `.kb/` write. |
| **2 — content review** (a human, one claim at a time, against the UX brief's own checklist — AC-TB-06) | Read `_clause-completeness.md` against IQ-1…IQ-6 and AC-UX-09, AC-UX-10, AC-UX-12 (`_decomposition.md:147-212`, `:266-281`) | AC-003, AC-004, AC-005: both directions stated as different sentences; every difference disposed of with an owner and a destination; every row self-contained; the verdict also a sentence. This is the only tier that can fail a well-formed record that says nothing — the analogue of the brief's named wrong implementation at `:728-732`. |
| **3 — integration / mount-point** | `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:48`), corpus-wide on the merged tree | AC-001: the specification's cross-references still resolve after the merge. This is the merge-gate step 3 the testing brief names for AC-013 (`:750-753`, `:873`). |
| **3 — integration / mount-point** | Walk `_ledger.md` → each row's `evidence` `file:line` → the cited sentence in `_clause-completeness.md` | AC-006: the statement is reachable and citable, not a file in a directory. The same walk `dod-scenario-ledger`'s DoD-11 row will perform. |
| **story grain (always runs)** | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | The prose-only diff breaks nothing. Expected to be a near-no-op: no Rust changed. |
| **downstream, not this PR** | `cargo xtask ci` (`.redkiln/config.yaml:60`), run by `terminal-gate-run` | AC-016, and the enforcement behind EC-004: an addition left unclassified while the pin's candidate scan is live keeps the terminal gate red. Named here so the coupling is visible before it fires, not discovered by a red build. |

## Risks and coupling (PR-scoped)

| Risk | Why it bites here | Counterweight in this spec |
| --- | --- | --- |
| **The observation is taken against the wrong tree.** | This planning worktree is mid-merge; a `spec-trace` run here would pass and mean nothing (`_decomposition.md:306-310`). The failure is invisible: a correct-looking record against a pre-merge copy. | AC-001 requires the sha in the record and Tier 1 resolves it; EC-001 halts rather than proceeds. |
| **The planned path is cited as a real one.** | `xtask/src/lint_narrative.rs` is named in HS-P0020's spec and does not exist in this worktree. Copying it into the record from the plan produces a dead citation that reads authoritative. | AC-002 makes location a step with a recorded search; EC-002 gives the absent case a defined outcome instead of an improvisation. |
| **A second enumeration is created "just for the record".** | The most natural way to write a set-difference table is to list the set — and that list then drifts from the one in the tree, which is precisely the defect the sibling's design already refuses (`frozen-documentation-must-pin/spec.md` AC-002). | AC-002's second clause; the PR boundary's explicit exclusion; the Tier 1 grep for an id list that names no source. |
| **The empty result is written as an omission.** | The expected outcome is "nothing was added", and the cheapest way to express it is to say nothing. That is indistinguishable from a check nobody ran. | AC-003's second clause; Tier 2 review; both direction headings present regardless. |
| **Scope creep into HS-P0020's pin.** | An implementer who finds an unclassified candidate will be tempted to classify it — it is a one-line edit and the gate is red. That edit is another project's act and lands outside this story's boundary. | The single-glob PR boundary; AC-004's two dispositions with owners; EC-004 stating both consequences explicitly. |
| **Coupling to `merge-forward-baseline`.** | Slice-mate, same context, first in merge order. If the merge's sha is not recorded where this story can read it, every downstream record in the project names a different tree. | The Integration contract consumes the sha as a value; `_storymap.md:144-147` fixes the order; EC-001 is the halt. |
| **Coupling to `dod-scenario-ledger`.** | It is `blocked_by` this story so its DoD-11 row can cite this statement (`_storymap.md:56`). A statement written to be read but not to be *cited* forces a paraphrase, and a paraphrased verdict is a second verdict. | AC-006 states the citability requirement in the downstream reader's terms, and the Tier 3 walk exercises it. |
| **An ADR written as a by-product.** | A closeout that surfaces something interesting is exactly where the corpus's most-refused practice appears (`project.md:251`). | EC-007; AC-A09; `validate --kb`'s immutability check catches a stray touch structurally. |

## Dependencies

**Blocks on** — `merge-forward-baseline` (`_storymap.md:48`). The hard edge, not a preference:
this story observes the commit that story produces, names its sha, and runs no `git merge` of
its own. It is the same milestone and the same implementer context, and ordering 1 —
*merge before everything* — is what the boundary enforces (`_decomposition.md:630-632`;
`_storymap.md:144-147`).

**Unlocks** — `dod-scenario-ledger` (`_storymap.md:56`), which is `blocked_by` this story so
its **DoD-11** row can cite scenario 11 as a fresh observation on the assembled tree rather
than an inherited assertion. Downstream of that, `scenario-two-fault-injection` and
`terminal-gate-run` (`:57-58`) — the latter is where an addition left unclassified by AC-004
would surface as a red `cargo xtask ci`.

**Not a dependency, and worth saying so.** `reconciliation-ledger`, `charter-open-question-disposition`
and `design-tension-audit` all depend on `merge-forward-baseline` too, and none of them depends
on this story. The milestone boundary is `merged-tree-baseline` → `closeout-adjudication`
(`_storymap.md:148-151`), not a chain through this record.

Upstream of the whole project, and inherited rather than restated: HS-P0024 directly, and
HS-P0020 transitively through it (`project.md:224-229`) — which is why HS-P0020's pin is
expected to exist on the merged tree at all.

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. The Context pack above is self-sufficient for
starting; open these at the moment named.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/checked-documentation-surface/frozen-documentation-must-pin/spec.md` | The other side of this story's whole subject: the derivation rule, the single `const`, the derived candidate scan, and EC-007/EC-008 — the two messages this record's two directions inherit. Its AC-005 is literally the merge-forward case this story re-checks. | Before writing the set difference (AC-003) and before deciding a disposition (AC-004). | AC-002, AC-003, AC-004 |
| `.bklg/docs-that-teach/checked-documentation-surface/project.md` | AC-008 at `:223-226` — the set is enumerated by clause id in **exactly one file**, which is why this story reads and never re-lists — and `:273-279`, where the residual risk is handed to HS-P0025 by name. | Before citing "the pinned set" for the first time; re-read `:273-279` when writing the verdict sentence. | AC-002, AC-004 |
| `xtask/src/spec_trace.rs` | Its module documentation is the source for what a green run does and does not check. AC-001's honesty clause is a paraphrase of this file, not an assertion of the spec's own. | When writing the "what green does not prove" paragraph. | AC-001 |
| `xtask/src/main.rs` | The step as already wired: `:105` (the `REQUIRED` list head), `:315` (`name: "specification traceability"`), `:317-326` (args), `:671-675` and `:738` (dispatch and help). Read to confirm you are invoking, not adding. | Before running the command, and again if you feel tempted to add a step. | AC-001 |
| `spec/SPECIFICATION.md` | `:280` is the clause that makes the whole method valid — ids are stable names, never renumbered — and the document behind any candidate's text. | Before comparing anything; then per candidate, to read its clause. | AC-002, AC-003 |
| `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md` | The three briefs in one file: architecture §1 (`:410-427`, the read-only diff surface), §5 (`:630-655`, the orderings and DR-12's re-check), UX IQ-1…IQ-8 (`:147-224`) and AC-UX-09/10/12 (`:266-281`), testing tiers (`:704-711`) and the AC→tier row for AC-013 (`:873`). | §1 and §5 before starting; the UX block before writing the record; the testing block before claiming a tier. | AC-001, AC-003, AC-005, AC-006 |
| `.bklg/docs-that-teach/durable-audience-closeout/project.md` | AC-013 and AC-012 verbatim (`:202`, `:201`), DR-12 (`:172-175`), the out-of-scope line that keeps pinning with HS-P0020 (`:102-105`), and the risk row naming this exact residual (`:248`). | Before writing the acceptance evidence into `_ledger.md`. | AC-001, AC-004 |
| `.bklg/docs-that-teach/durable-audience-closeout/_storymap.md` | This story's row (`:49`), the milestone rationale (`:62-65`), merge order (`:144-147`), and the downstream edge that makes AC-006 concrete (`:56`, `:156-160`). | When confirming the slice boundary and the downstream citation obligation. | AC-006 |
| `.bklg/docs-that-teach/durable-audience-closeout/merge-forward-baseline/spec.md` | The slice-mate that produces the sha. It is where the merged commit is recorded and what "the merged tree" resolves to. | First, before any observation. | AC-001 |
| `.bklg/docs-that-teach/_decomposition.md` | The initiative-level merge-forward decision and the residual risk it accepted on the record (`:89-103`, `:116-123`) — the argument that made parallel work safe, and the one gap it left open. | When writing the executive framing of the verdict, so the record explains *why* this check exists. | AC-003, AC-004 |
| `.bklg/docs-that-teach/initiative.md` | DoD scenario **11** verbatim (`:452-454`) and the closing criterion (`:662-663`) — the sentence `dod-scenario-ledger` will cite this record against. | Before writing the verdict sentence, so it answers scenario 11's wording rather than adjacent wording. | AC-006 |
| `.bklg/docs-that-teach/durable-audience-closeout/_design.md` | The signed-off design: `## Items` empty by decision (`:10-20`, `:45-47`), sign-off recorded (`:91-93`). It is the authority for "this story renders no surface" — and therefore for why the composition invariants apply to markdown instead. | Before writing anything that looks like a UI decision, and before claiming "no surface means no obligations". | AC-005 |
| `.bklg/docs-that-teach/durable-audience-closeout/_grounding.md` | The verified-anchor record, including the confirmation that **no Accepted decision atom governs this work** (`:18-31`). It is what stops a search for a documentation ADR that does not exist. | If you find yourself hunting for an ADR to cite. | AC-004 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | What actually binds a correction here: a new atom that supersedes, never an edit — and the refusal of a decision written as closeout exhaust. | When a finding looks like it wants a decision (EC-007). | AC-004 |
| `.kb/maps/open-questions-index.md` | `:136-143` is the corpus's own status-word-first row shape — the precedent AC-005's "every state is a word" is copied from rather than invented. | When laying out the disposition table. | AC-005 |
| `standards/rust/81-checks-that-cannot-be-types.md` | `:335-341` (RS-81-5) — a failure must name *which side moved*. The rule this record's two-direction discipline is the prose analogue of. | When writing the two direction sentences. | AC-003 |
| `.redkiln/config.yaml` | The real grains: `:40` (`affected_gate`), `:48` (`reachability_static`, this story's Tier 3), `:60` (the terminal `e2e`), `:67` (`require_ledger`), `:5` (the `support` initiative, the routed arm's destination). | Before naming any command or any routing destination. | AC-001, AC-004, AC-006 |
| `.bklg/docs-that-teach/checked-documentation-surface/spec-trace-clause-id-accessor/spec.md` | The resolver `crate::spec_trace::clause_ids` that HS-P0020's pin calls once per run — context for why no second parser or regex is admissible anywhere near this set. | If you are tempted to write a clause-id matcher of any kind. | AC-002 |

## Clarifications resolved during spec

- **The six AC ids are exactly the ones the first pass declared.** AC-001 … AC-006, none added
  and none dropped. AC-013 is carried by AC-001 through AC-005 (the run, the located
  enumeration, the two-direction difference, the disposition, and the record's composition);
  AC-012 is carried by AC-001 (the sha is named and resolves) and AC-006 (the record is
  reachable and citable). The `_ledger.md` rows match this set one for one.
- **"Renders no surface" does not mean "no interaction-quality bar".** `_design.md` declares
  no items (`:45-47`), which removes the *API/screen* review, not the UX brief's invariants —
  those were written for markdown artefacts and name this project's four of them explicitly
  (`:26-35`). Resolved in favour of binding them as composition ACs on the record (AC-005),
  rather than declaring the section N/A and shipping a transcript in a file.
- **The composition primitives are the text corpus's, not an invented design system.** There
  is no CSS in this repository and the UX brief says so (`_decomposition.md:76-81`). The
  density budget is therefore stated in the units this medium has — one table, five columns,
  heading depth, wrap width — and the wrap number (≤ 97 columns) was **measured** on this
  spec's own non-table body rather than asserted.
- **Verification is commands and reviewer reads, not new Rust tests.** AC-TB-08 (`:846-851`)
  forbids implying new Rust coverage, and this story compiles nothing. Every `verifying_test`
  in the ledger is therefore a real command from `.redkiln/config.yaml` or `xtask/src/main.rs`
  plus a named reviewer read against a checklist that already exists (IQ-1…IQ-8,
  AC-UX-01…AC-UX-12) — the brief's own instruction not to maintain a second checklist
  (AC-TB-06).
- **A failing `spec-trace` is an outcome, not an unwritten case.** EC-003 gives it a defined
  behaviour — record it verbatim, route the defect, and treat the story as blocked with a named
  cause — because AC-013 requires the run to pass and a story that can only report green would
  be decorative under CLAUDE.md's "a rule that no adapter can fail" corollary.
- **The routed arm's destination is named concretely.** `.redkiln/config.yaml:5` declares the
  `support` initiative; "routed" therefore means an item there, not an abstraction. Resolved so
  that AC-004 can be falsified by reading a cell rather than by judging a tone.
- **The gate consequence is stated in the record, not only in this spec.** While HS-P0020's
  derived candidate scan is live, an unclassified candidate is already a gate failure — so
  "routed" can never mean "deferred past `terminal-gate-run`". Written into AC-004 and EC-004
  so a later reader meets the coupling before a red build teaches it to them.
- **No ADR, and no decision atom, in any arm.** Confirmed against `_grounding.md:18-31`: no
  Accepted decision atom governs documentation or gate structure, so citing one would be
  manufacturing authority, and authoring one would be the practice the corpus was built to
  refuse. Findings that want a decision are routed gaps in prose (EC-007).
