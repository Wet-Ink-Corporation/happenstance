---
item: HS-P0020
stage: storymap
created: 2026-08-17T03:21:37.428Z
updated: 2026-08-17T03:21:37.428Z
template_sig: 1c63534a
rendered_sig: 34d8957b
---

# Story Map — The Checked Documentation Surface

Ten stories across four milestones. The "user" throughout is a **contributor running the
gate** and a **reviewer reading its output** — this project ships no runtime surface, so
every slice is observable through `cargo xtask ci`, `cargo xtask ci --fast` or
`cargo xtask affected --base main` (`.redkiln/config.yaml:28-40` wires the last two as the
story and integration grains).

One framing rule governs every row below, taken from the charter's DoD item 8 and repeated
here because it is the initiative's top-ranked risk: **this project proves code inside
prose still compiles against the library; it proves nothing about whether the prose
teaches.** No story may be written up as evidence of comprehension. HS-P0024
`comprehension-evidence` owns that, and is not substitutable by anything here.

## Backbone

The activities, left to right, in the order a page moves through the machine.

| # | Activity | The contributor's outcome | Stories |
| - | -------- | ------------------------- | ------- |
| A | **Put the prose where the gate can see it** | A narrative tree exists at one path, and moving it is a compile error rather than a convention breach | `pinned-narrative-tree-and-compiling-step`, `narrative-tree-story-grain-selection`, `narrative-checker-mounted-with-pinned-path` |
| B | **Compile every fence against the real library** | Deleting a public item a page calls fails the gate, on every runner, with no tool to install | `pinned-narrative-tree-and-compiling-step` |
| C | **Refuse the ways prose escapes the check** | A page nobody registered, a fence that opted out, and a hidden panel are all failures, not silent passes | `narrative-checker-mounted-with-pinned-path`, `fence-discipline-and-allowance-list`, `hidden-content-resolution` |
| D | **Resolve every claim into the specification** | A clause id a page cites either exists or the gate says so; the frozen documentation MUSTs are pinned by id in one place | `spec-trace-clause-id-accessor`, `narrative-citation-resolution`, `frozen-documentation-must-pin` |
| E | **Prove the check fails, and say what it cannot see** | The gate has been watched failing on a broken page and recovering; its blind spots are written down and, where possible, executed | `observed-failure-falsification`, `documented-blind-spots-and-their-proofs` |

Activity E is the one that makes the other four worth anything. `RUNBOOK.md:918-925` is the
in-house precedent for a step that was wired, vouched for by two documents, and printed
`skipped` on all three runners; a gate that has only ever been green is decorative.

## Slices

Four milestones. Each is one integrated, mounted surface handed to a single implementer
context; cross-milestone `depends_on` edges are acyclic and are the merge order.

| Milestone | Story | Archetype | One-line slice | depends_on | traces_to |
| --------- | ----- | --------- | -------------- | ---------- | --------- |
| `compiled-narrative-tree` | `pinned-narrative-tree-and-compiling-step` | capability | Create the narrative tree at its decided path with a real fixture page, compile every Rust fence in it as one `#[cfg(doctest)] mod` per page in `xtask/src/narrative.rs` mounted at `xtask/src/lib.rs`, and wire that compile as a `REQUIRED` `Step` with `probe: None`, `--locked` and step-scoped `RUSTDOCFLAGS`. | — | AC-002, AC-009 |
| `compiled-narrative-tree` | `narrative-tree-story-grain-selection` | capability | Extend `xtask/src/affected.rs`'s selection arm so a change confined to the narrative tree selects the `xtask` package, with unit tests in both directions, so the story-grain gate does not compile nothing on the very PR that broke a page. | `pinned-narrative-tree-and-compiling-step` | AC-009 |
| `narrative-checker-discipline` | `narrative-checker-mounted-with-pinned-path` | capability | Add the bin-crate checker module holding the tree and harness paths as `const`s, erroring by name on a missing tree and `bail!`ing on an empty one, cross-checking page↔harness registration in both directions, and mount it as its own `REQUIRED` step, subcommand, help line and `lint_steps` member. | `pinned-narrative-tree-and-compiling-step` | AC-001, AC-005 |
| `narrative-checker-discipline` | `fence-discipline-and-allowance-list` | capability | Walk every fence in the tree: reject untagged fences, enumerate info-string parts exhaustively so an unrecognised part is a hard error, reject `ignore`-class fences unless they appear on an enumerated allowance `const`, and sweep that list in reverse so a stale allowance is itself a problem. | `narrative-checker-mounted-with-pinned-path` | AC-004 |
| `narrative-checker-discipline` | `hidden-content-resolution` | capability | Close DT-7 in `_design.md` and enforce the answer in the same fence walk — either the markers that produce tabs/folds/panels are rejected under the pinned tree, or a claim broken inside a non-default panel is observed to fail the gate and the run recorded. | `fence-discipline-and-allowance-list` | AC-006 |
| `specification-pin` | `spec-trace-clause-id-accessor` | foundation | Add `pub(crate) fn clause_ids(root: &Path) -> Result<BTreeSet<String>>` beside `all_rules` in `xtask/src/spec_trace.rs`, built from the existing `parse_clauses` and the existing `SECTIONS` family list, so the two consumers below resolve clause ids through the parser instead of a fourth prefix list that will not agree. | — | AC-007, AC-008 |
| `specification-pin` | `narrative-citation-resolution` | capability | Parse the clause ids a narrative page cites with the hard-error-on-unreadable posture, resolve each through `clause_ids`, and report every dangling id as a problem naming the page — a page citing a real id passes, one citing a nonexistent id fails. | `narrative-checker-mounted-with-pinned-path`, `spec-trace-clause-id-accessor` | AC-007 |
| `specification-pin` | `frozen-documentation-must-pin` | capability | Re-derive the frozen documentation MUSTs against `spec/SPECIFICATION.md` as it stands, enumerate them by clause id in exactly one commented `const` naming each discharge site, and check three things — the ids resolve, each site still contains its id, and the derived count matches the hand-written one, naming which moved. | `narrative-checker-mounted-with-pinned-path`, `spec-trace-clause-id-accessor` | AC-008 |
| `falsification-and-limits` | `observed-failure-falsification` | capability | Break one claim in the fixture page, run the gate, record the failure output verbatim including which file it actually names, revert, run again, record green — both halves in this project's own artefacts. | `pinned-narrative-tree-and-compiling-step`, `narrative-checker-mounted-with-pinned-path` | AC-003 |
| `falsification-and-limits` | `documented-blind-spots-and-their-proofs` | capability | Put a "What this does not verify" section first in the new modules' own docs carrying all six enumerated limits, re-run the `RUSTDOCFLAGS` probe against the new step and record what it actually does, and walk a `text`-tagged broken fixture through the gate to prove that limit is real rather than asserted. | `fence-discipline-and-allowance-list`, `narrative-citation-resolution`, `frozen-documentation-must-pin`, `observed-failure-falsification` | AC-010 |

### Why these four milestones and not others

- **`compiled-narrative-tree`** is one surface because the tree, the `include_str!`
  harness, the `mod narrative;` in `xtask/src/lib.rs` and the `REQUIRED` step are the same
  fact stated in four files. Splitting "build the harness" from "wire the step" would ship a
  `mod` in the bin crate whose fences nothing ever compiles — the exact silent-pass shape
  BR-02 exists to prevent (`_decomposition.md` architecture brief, Note 1 CR-1). The
  `affected.rs` arm is a separate story because it is a separate selector with its own unit
  tests, but it must land in the same context: `docs/` is on the `INERT` list and
  `a_docs_only_change_selects_nothing` asserts it, so a tree under `docs/` that nobody
  taught `affected` about selects no package at all.
- **`narrative-checker-discipline`** is one surface because all three stories add checks to
  one bin-crate module, push onto one `Vec<String>` of problems, and are reported by one
  `bail!`. `hidden-content-resolution` sits here rather than with the falsifications
  precisely because the architecture brief puts its enforcement "in the same fence walk as
  AC-004" — implementing it in a different context would edit one function twice.
- **`specification-pin`** is a distinct surface because it reads a *different* document.
  Its foundation story lands first: `clause_ids` is real in-tree substrate with no gate
  step of its own, and both capability stories in the slice consume and demonstrate it.
  Nothing outside this initiative owns it — it is a sibling of an accessor that already
  exists in the same file for the same reason.
- **`falsification-and-limits`** is last because it is evidence, and evidence gathered
  before the machine is finished is evidence about a different machine. `AC-003` requires
  observing `cargo xtask ci` itself fail, which no unit test can substitute for.

### Stories this map deliberately does not contain

- **Any teaching page.** The fixture pages in `pinned-narrative-tree-and-compiling-step`,
  `hidden-content-resolution` and `documented-blind-spots-and-their-proofs` are test
  material. The corpus belongs to HS-P0021 / HS-P0022 / HS-P0023, per the charter's risk
  table ("This project may write *fixture* pages for the falsification; it may not write
  the corpus").
- **A `page-need` metadata check.** If HS-P0021's discipline turns out to need a
  machine-checkable per-page field, the check may land in this module later. Adding it now
  would give this project a second job — the exact defect BR-04 exists to make visible.
- **An ADR.** The grounding pass verified that no Accepted decision atom under
  `.kb/decisions/` governs gate structure, documentation trees or fence compiling, and no
  tension with one was found. The two divergences from in-repo *precedent* — the enumerated
  allowance list in `fence-discipline-and-allowance-list`, and joining
  `affected::run`'s unconditional list in `narrative-checker-mounted-with-pinned-path` —
  are conventions, not decisions, and are discharged by a sentence in the new module's own
  docs rather than by an amendment.
- **A refactor of `lint_constitution.rs` or `constitution.rs`** to share code with the new
  checker. RS-81-3 scopes a scanner to the directory whose behaviour it constrains; the
  shape is cheap to copy and a shared abstraction makes one error message answer two
  questions.

## Coverage

Every project AC is claimed by at least one story, and each AC has exactly one story that
**owns** it. AC-007, AC-008 and AC-009 appear twice, and in each case the second row is a
genuinely different responsibility rather than a duplicate.

| Project AC | Owning story | Also touched by | Grain of proof |
| ---------- | ------------ | --------------- | -------------- |
| AC-001 — the tree is pinned, not conventional | `narrative-checker-mounted-with-pinned-path` | — | static: the missing-tree error names the expected path; the empty-tree case is a hard error, not a vacuous pass |
| AC-002 — every fence compiled against the real crates, mandatorily | `pinned-narrative-tree-and-compiling-step` | — | compile: `cargo test --locked -p xtask --doc`; removing a public item a fixture page calls fails the step |
| AC-003 — the check has been seen to fail | `observed-failure-falsification` | — | end-to-end: `cargo xtask ci` observed failing and then green, both outputs recorded verbatim |
| AC-004 — no silent opt-out | `fence-discipline-and-allowance-list` | — | static: untagged rejected; unlisted `ignore` rejected by file and line; listed `ignore` passes; stale allowance is a problem |
| AC-005 — no orphan pages | `narrative-checker-mounted-with-pinned-path` | — | static, bidirectional: unregistered page fails, and a registered module naming a deleted page fails |
| AC-006 — hidden content inside the check, or absent | `hidden-content-resolution` | — | static if forbidden; fixture falsification inside a non-default panel if permitted. DT-7 resolved in `_design.md` either way |
| AC-007 — citations resolve | `narrative-citation-resolution` | `spec-trace-clause-id-accessor` (the resolver it calls) | static: real id passes, nonexistent id fails naming the page, unparseable citation is a hard error |
| AC-008 — the frozen documentation MUSTs are pinned | `frozen-documentation-must-pin` | `spec-trace-clause-id-accessor` (the resolver it calls) | static ×3 plus a written re-derivation; `cargo xtask spec-trace` passes over the tree as it stands |
| AC-009 — clean checkout, no manual step | `pinned-narrative-tree-and-compiling-step` (`ci`, `ci --fast`, and the hosting/render shape in `_design.md`) | `narrative-tree-story-grain-selection` (`affected --base main`) | gate-integration on three separately-wired invocation paths |
| AC-010 — the limits are on the record | `documented-blind-spots-and-their-proofs` | — | static presence plus executed proof where a proof is possible; the compiles-but-no-longer-demonstrates gap is documented as inherently untestable |

**No AC is orphaned and no responsibility is owned twice.** The three shared ACs are split
by mechanism, not by layer: AC-007/AC-008's second row is the shared resolver both checks
call rather than a second implementation of either check, and AC-009's second row is the
`affected` selector, which is wired independently of `REQUIRED` and is the one path a
`REQUIRED` membership test cannot reach.

Two project-level obligations are carried by every story rather than traced to one, and are
restated here so nothing silently drops them:

- **DoD item 7** — every new check in `xtask/src/` carries a "what this does not verify"
  section, and `cargo xtask lint-constitution` plus `cargo test -p xtask --doc` still pass.
  `documented-blind-spots-and-their-proofs` owns the *contents* of that section; each story
  owes the section's existence for whatever it adds.
- **DoD item 8** — nothing in this project asserts anywhere that the surface proves a page
  teaches. This is a review check on prose in every story, not a testable AC.

## Merge order

Milestones in dependency order; stories in the order they land inside each.

1. **`compiled-narrative-tree`**
   1. `pinned-narrative-tree-and-compiling-step` — the tree exists, its fences compile, the
      step is mandatory. Nothing else in the project can be observed until this is green.
   2. `narrative-tree-story-grain-selection` — the story grain sees the tree. Landing this
      second, in the same context, is deliberate: until it lands, every subsequent story's
      own `affected` gate is blind to prose-only changes.
2. **`narrative-checker-discipline`** (after 1)
   3. `narrative-checker-mounted-with-pinned-path` — the checker module and its second
      `REQUIRED` step exist, with the pinned constants, the vacuity guard, the
      accumulate-all-problems shape and the bidirectional registration check.
   4. `fence-discipline-and-allowance-list` — the fence walk, into the module story 3
      created.
   5. `hidden-content-resolution` — DT-7's answer enforced inside that same fence walk.
3. **`specification-pin`** (after 2; independent of milestone 4)
   6. `spec-trace-clause-id-accessor` — **foundation, first in this milestone and before
      both of its consumers.** It has no gate step of its own and is proven by the two
      stories below, which is the whole reason it is not left as a copied regex.
   7. `narrative-citation-resolution`
   8. `frozen-documentation-must-pin` — this is the story HS-P0023 is sequenced behind. The
      pin must be in place before any project rewrites a `happenstance-core` doc comment,
      which is why it lands here and not at closeout.
4. **`falsification-and-limits`** (after 2 and 3)
   9. `observed-failure-falsification` — run against the assembled gate, not a partial one,
      so the recorded output is the output a future contributor will actually see.
   10. `documented-blind-spots-and-their-proofs` — last, because the list of limits is only
       complete once every check that has a limit exists. This story is the project's final
       honesty pass and the one that keeps a green gate from being read as evidence of
       teachability.

`cargo xtask ci --fast` is the bar each non-terminal story meets; the full `cargo xtask ci`
is run before the project is called done (charter DoD item 6, `.redkiln/config.yaml`).
