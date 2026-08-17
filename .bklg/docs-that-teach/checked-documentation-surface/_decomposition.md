---
item: HS-P0020
stage: briefs
created: 2026-08-17
updated: 2026-08-17
---

# Briefs — The Checked Documentation Surface

This file holds every brief for HS-P0020. Each brief is one `##` section; read the
project charter (`.bklg/docs-that-teach/checked-documentation-surface/project.md`)
and the grounding pass (`_grounding.md`) alongside it.

## Architecture brief

### Intent

Build the substrate that makes narrative prose checkable, and wire it into the gate
that already exists rather than beside it. Concretely: a narrative tree whose path is
a constant in `xtask/src/`, every Rust fence in that tree compiled against the real
workspace crates by a **mandatory** member of `REQUIRED`, a companion file-reading
check that closes the three holes the compiler cannot see (an orphaned page, a fence
that opted out, a citation that does not resolve), the frozen documentation MUSTs
enumerated by clause id in one place and mechanically resolved, and the whole thing
observed *failing* on a page broken on purpose.

The scope boundary is architectural, not rhetorical: this brief specifies a machine
that proves **code inside prose still compiles against the library**. It specifies
nothing that proves a page teaches, and every artefact it produces must say so in its
own documentation (`AC-010`, project DoD item 8).

No Accepted decision atom under `.kb/decisions/` governs gate structure, documentation
trees or fence compiling — verified in `_grounding.md` ("Precedence and non-goals").
The binding constraints here are conventions with teeth: `standards/rust/80-the-gate.md`
(RS-80-1 to RS-80-4), `standards/rust/81-checks-that-cannot-be-types.md` (RS-81-1,
RS-81-3, RS-81-5), and `CLAUDE.md`. **No tension with an Accepted ADR exists, and this
project needs no ADR.** Two deliberate divergences from in-repo *precedent* are flagged
in Notes 5 and 9; both are conventions, not decisions, so neither needs an amendment —
but both must be stated in the new module's own documentation, not left in this brief.

### Acceptance Criteria

Architectural obligations, one per project AC. Each names where the work mounts.

- **AC-001 — pinned, not conventional.** The tree's path is a `const` in the new
  checker module, in the shape of `ATOM_DIR` / `ROUTER` / `HARNESS`
  (`xtask/src/lint_constitution.rs:55,58,61`). Two mechanisms make a move fail, and
  both are required: the directory read is `?`-propagated with `.with_context()` so a
  missing directory is an error naming the expected path, **and** an empty-tree `bail!`
  mirrors `lint_constitution.rs:174-176` (`"{ATOM_DIR} holds no atoms, so every check
  below is vacuous"`). A pinned constant without the vacuity guard passes green over a
  tree someone emptied.
- **AC-002 — every fence compiled, mandatorily.** A `Step` in `REQUIRED`
  (`xtask/src/main.rs:105`) with `probe: None` (RS-80-1, RS-80-2;
  `standards/rust/80-the-gate.md:11-17,98-104`), `--locked` on any invocation that
  resolves dependencies (RS-80-4, `:245-251`), and rustdoc flags in that step's own
  `env` (RS-80-3, `:181-187`) — never ambient. The compiling mechanism is the doctest
  harness in the **lib** target (Note 1, CR-1), so the step is
  `cargo test --locked -p xtask --doc`, which is the existing step at
  `xtask/src/main.rs:488-492`; see Note 3 for whether it is extended or joined.
- **AC-003 — seen to fail, by page and location.** Discharged architecturally by
  **one module per page** (Note 3). The testing brief owns the procedure and the
  record; the architecture obligation is that the mechanism chosen can name a page at
  all, which the naive spelling cannot.
- **AC-004 — no silent opt-out.** A fence-discipline check in the same shape as
  `check_fences` (`xtask/src/lint_constitution.rs:601-643`): untagged fences rejected,
  info-string parts enumerated and an unrecognised part a hard error. The `ignore` rule
  itself diverges from precedent — see Note 5, which is a decision this brief takes.
- **AC-005 — no orphan pages.** The bidirectional harness check of
  `check_harness` (`xtask/src/lint_constitution.rs:423-458`), copied in both
  directions: every page in the tree has an `include_str!` **and** a `mod` in the
  harness, and every `mod` in the harness names a page that still exists
  (`:445-458`). The reverse half is the one that matters, for the reason stated there —
  `cfg(doctest)` hides an orphaned module from every step but `cargo test`.
- **AC-006 — hidden content inside the check, or absent.** Architecturally this is a
  static-source question, not a runtime-DOM one: whatever the ux brief and `_design.md`
  decide, the enforcement lands in the same fence walk as AC-004. If hidden content is
  forbidden, the check rejects the markers that produce it; if it is permitted, the
  falsification runs inside a non-default panel. No new machinery either way.
- **AC-007 — citations resolve.** Clause ids are resolved through `spec_trace`, not
  re-parsed (Note 6). A dangling id is a problem pushed onto the same `Vec<String>` the
  other checks use, so one run reports all of them (`lint_constitution.rs:169-198`).
- **AC-008 — the frozen documentation MUSTs are pinned.** An enumerated, commented
  `const` in the shape of `RULE_FILES` / `WIRE_TESTS` (`xtask/src/spec_trace.rs:85-105`)
  plus a check that each id exists in `spec/SPECIFICATION.md` and each named discharge
  site still contains it (Note 7). The set is **re-derived**, not copied.
- **AC-009 — clean checkout, no manual step.** Everything above is reachable from
  `cargo xtask ci` and from `cargo xtask ci --fast` (both select out of `REQUIRED`;
  `xtask/src/main.rs:828-829`), and from `cargo xtask affected --base main`, which
  `.redkiln/config.yaml:40` wires as the story grain. The `affected` path is the one
  most likely to be missed — Note 1, CR-5.
- **AC-010 — the limits are on the record.** A "What this does not verify" section in
  the new module's own docs, first rather than last, in the shape of
  `xtask/src/lint_constitution.rs:9-28` and `xtask/src/constitution.rs:20-36`
  (RS-81-1). The required contents are enumerated in Note 10.

### Notes

#### 1. Composition roots — there are five, all inside `xtask/`, and missing any one ships a decorative check

This is the integration section. `xtask` has **two targets that do not share modules**,
and the single most likely implementation error is mounting a capability in the wrong
one.

- **CR-1 — the doctest root: `xtask/src/lib.rs:28`.** `lib.rs` declares exactly
  `mod constitution;` and carries the repository README under
  `#![cfg_attr(doctest, doc = include_str!("../../README.md"))]` (`lib.rs:21`). The
  compiling harness for the narrative tree mounts **here**, as a sibling module (e.g.
  `mod narrative;` → `xtask/src/narrative.rs`). `cargo test -p xtask --doc` compiles the
  **lib** target's doctests only. A `mod narrative;` added to `main.rs` instead lands in
  the *bin* crate, compiles clean, and its fences are never compiled by anything — the
  exact silent-pass shape BR-02 exists to prevent.
- **CR-2 — the bin module list: `xtask/src/main.rs:64-70`.** The *checker* (AC-001,
  004, 005, 007, 008) is a bin-crate module declared alongside `mod lint_constitution;`.
  Checker and harness are therefore two files in two targets; see Note 2 for the seam
  that lets one verify the other.
- **CR-3 — the gate array: `xtask/src/main.rs:105` (`REQUIRED`).** `Step` is the
  contract (`main.rs:72-103`); the `probe` doc comment at `:89-102` is the normative
  statement of what `None` means. Steps must be added here and nowhere else (RS-80-1),
  because `--fast`, `cargo xtask wasm` and `cargo xtask lints` all select out of this
  array.
- **CR-4 — the dispatch and the by-name subsets: `xtask/src/main.rs:642-707`,
  `:718+`, `:799-808`, `:816-826`.** A new subcommand needs an arm in `main()`'s match
  mirroring `Some("lint-constitution") => …` (`:689-697`), a line in `print_help()`, and
  — if it belongs to the file-reading family — a name in `lint_steps()` (`:799-808`).
  `steps_named` panics on a name absent from `REQUIRED` (`:816-826`), which is the
  intended failure: it makes a half-mounted step a build-time bug rather than a silent
  omission.
- **CR-5 — the story-grain selector: `xtask/src/affected.rs:212-224` and
  `is_inert`'s `INERT` list at `:249-260`.** **This is the landmine.** `docs/` is on the
  inert list (`affected.rs:250`) and `a_docs_only_change_selects_nothing`
  (`affected.rs:650-654`) asserts it. `README.md` and `standards/rust/` are the two
  exceptions, selected into `xtask` precisely because that crate compiles them as
  doctests (`affected.rs:212-221`, and the reasoning at `:245-247`: adding
  `standards/rust/` to `INERT` "would silently un-compile the corpus"). If the narrative
  tree lands under `docs/` — or anywhere else outside a workspace member — the same arm
  must be extended in the same change, with a unit test mirroring
  `a_constitution_atom_selects_xtask` (`affected.rs:688-696`) and one mirroring
  `the_constitution_arm_does_not_widen_to_all_prose` (`:699-701`). Without it,
  `cargo xtask affected --base main` selects no package on a narrative-page change and
  the story gate compiles nothing on the very pull request that broke the page.

`affected::run` also runs a fixed list of file-reading checks unconditionally
(`affected.rs:120-125`), for the reason its module docs give at `:28-36`. Note 9 records
the decision about whether the new checker joins that list.

#### 2. The cross-target seam: the checker reads the harness as text, it does not link it

`check_harness` reads `xtask/src/constitution.rs` with
`fs::read_to_string(root.join(HARNESS))` (`xtask/src/lint_constitution.rs:424-425`,
`HARNESS` at `:61`) and greps it for `include_str!("../../{ATOM_DIR}/{file}")` and
`mod {module} {`. That is how a check living in the **bin** crate proves a fact about a
file in the **lib** crate without the two ever linking. Copy it exactly: the new checker
holds a `HARNESS`-shaped constant naming the new lib-side module file and matches the
same two literal forms. The module-name derivation (`NN-slug.md` → `nn_slug`) is done by
`atoms()` (`lint_constitution.rs:208-246`); whatever page-naming scheme the tree adopts,
the derivation must be a pure function in the checker so both directions of the orphan
check agree by construction.

#### 3. Mechanism: `#[cfg(doctest)] mod` + `include_str!`, one module per page

Take the mechanism the workspace already runs at scale, for reasons that are decisive
rather than merely convenient:

- **`mdbook test` cannot satisfy AC-002.** It is an external binary, so it is a
  `probe:` shape by default — and a probe means "skip when absent"
  (`main.rs:89-102`, RS-80-2), which DR-03 forbids. Wiring it with `probe: None`
  instead makes every clean checkout without `mdbook` fail the mandatory gate, and
  mdBook's own failure mode when its toolchain is absent is an opaque
  `No such file or directory` rather than a legible skip
  (`_discovery/research/02-…`, finding at line 39). Either way it breaks AC-009's
  "clean checkout, no manual step".
- **No dependency is added.** DR-12's constraint is already recorded in
  `xtask/Cargo.toml:16-21`, and the dev-dependencies at `:22-33` (`happenstance`,
  `happenstance-core`, `happenstance-testkit`, plus `bytes`, `futures-*`, `tokio`,
  `trait-variant`) are exactly what a fence needs to call real workspace types — the
  same set `standards/rust/91-adapter-authoring-recipe.md` already uses.
- **One module per page is what discharges AC-003, and it is not a style choice.**
  `xtask/src/constitution.rs:11-18` states the argument: several `#![doc =
  include_str!]` attributes on one module concatenate into a single doc string, so a
  failure in the nineteenth file reports a line counted from the first, "which maps to
  no file a reader can open." One module per page makes the failure read
  `xtask::narrative::<page> (line 42)` with 42 relative to the page. This is the direct
  answer to the degraded-diagnostics risk in the charter's risk table and to
  `_discovery/research/02-…`'s finding at line 41.
- **Residual degradation, to be recorded not routed around.** The reported *file* is
  still the harness (`xtask/src/narrative.rs`), not the markdown path. The module name
  resolves the page and the line resolves the location, which is what AC-003 asks for;
  the file name does not. Say so in AC-010's list.
- **The render is not the check.** If a rendered HTML surface is wanted, it is a
  separate concern from the compiling step and belongs to the deployment brief. Do not
  let a renderer become the thing that compiles the fences: that re-introduces the
  binary-probe problem above and makes the gate's coverage depend on a tool's presence.

Whether the harness is a new `xtask/src/narrative.rs` or an extension of
`xtask/src/constitution.rs` is the implementer's call, but the recommendation is a new
file with a new `REQUIRED` step: the existing step's name — `"the constitution's
examples compile"` (`main.rs:488`) — is a claim about one corpus, and a step whose name
no longer describes what failed is the first step in a check becoming unreadable. Two
steps also keep the two trees' failures attributable at the gate's own output level.

#### 4. Data flow — two independent passes over one tree, deliberately

```
narrative tree (pinned by const)
  │
  ├─► [bin] checker module  ── read_dir ─► pages ─► parse fences ─┐
  │        reads xtask/src/<harness>.rs as TEXT                   ├─► Vec<String> problems
  │        reads spec/SPECIFICATION.md via spec_trace             │   └─► bail!("{n} problem(s)")
  │                                                               ┘
  └─► [lib] harness  ── include_str! under #[cfg(doctest)] ─► rustdoc ─► rustc + dev-deps
```

The two passes never meet at runtime, which is the whole reason AC-005 exists: the
compiler pass cannot see a page nobody registered, and the checker pass cannot see
whether a registered page compiles. Each is the other's blind spot, and the harness
registration check is the bridge. Report **all** problems rather than the first, as
`lint_constitution::run` does (`:169-198`) — a check that stops at the first problem
turns one review cycle into six.

#### 5. AC-004 — the enumerated allowance list, and why it diverges from `check_fences`

**Decision: adopt the enumerated list, and keep everything else about `check_fences`.**

The existing rule requires an `<!-- ignore: <reason> -->` comment on the line above
(`xtask/src/lint_constitution.rs:638-643`). AC-004 asks for something different: "an
enumerated allowance list that the same check reads." Take AC-004's shape, as a
`const` of `(page-relative path, line-or-anchor, reason)` tuples in the checker,
because:

- an inline comment is reviewable only in the diff that introduces it, while a `const`
  array is one place a reviewer can read in full and a later reader can audit without
  a `git log`;
- a stale allowance is detectable. The same reverse sweep as `check_harness` applies:
  an allowance naming a fence that no longer exists must be a problem, or the list
  accumulates permission for code nobody has.

Everything else in `check_fences` is kept as-is and for its stated reasons: an untagged
fence is rejected because rustdoc compiles it as Rust anyway (`:609-619`); the
info-string parts are matched exhaustively so an unrecognised part is a hard error
rather than a silent pass (`:621-637`); a `compile_fail` error code must be named in
the prose (`:644-660`).

**Flag this in the new module's docs as a deliberate divergence**, with a sentence
saying why the narrative tree gets the stricter shape — otherwise the next contributor
reads two different `ignore` rules in one repository and assumes one is a mistake.
Nothing accepted is contradicted: `standards/rust/` conventions are not ADRs, and
`lint_constitution.rs` is untouched by this project.

#### 6. AC-007 — resolve clause ids through `spec_trace`, do not re-parse

`parse_clauses` (`xtask/src/spec_trace.rs:1357`) and `struct Clause` (`:218`) are
private. The house pattern for exposing one narrow fact out of that module is
`pub(crate) fn all_rules(root: &Path) -> Result<BTreeSet<String>>`
(`spec_trace.rs:1746`), and it exists next to the parser precisely so "a list of files
kept next to the function that parses them cannot drift from it" (`:83-84`). So: add a
sibling `pub(crate) fn clause_ids(root: &Path) -> Result<BTreeSet<String>>` built from
`parse_clauses`, and have the narrative checker call it. Do not copy a regex.

The recognised prefixes must come from `SECTIONS` (`spec_trace.rs:122-160`) — `VT-`,
`WF-`, `ES-`, `PS-`, `SY-`, `CF-` — which its own doc comment names as "the single place
the six clause families are enumerated… Three lists that must agree, kept as one so that
adding a family cannot half-land." A literal prefix list in the new checker is a fourth
list that will not agree.

Two parser cautions, both learned in-house and both cited so they are not re-learned:
`lint_constitution.rs:29-44` explains why its citation parser **hard-errors** on a span
it cannot read rather than skipping it — "a citation this parser declines to read is a
citation nothing verifies." That reasoning transfers verbatim; take the hard-error
posture, not `spec_trace::citations`'s permissive skip. And clause ids are stable and
never renumbered (`spec/SPECIFICATION.md:280`), which is exactly why this check survives
the 521-line divergence on `initiative/from-contract-to-published-library`
(`_decomposition.md`, "The sibling branch").

#### 7. AC-008 — the clause-id pin as an instrument, not a note

Shape it on `RULE_FILES` / `WIRE_TESTS` (`xtask/src/spec_trace.rs:85-105`): an
enumerated, heavily commented `const` array, each entry carrying the clause id and the
discharge site it is discharged at. The check then asserts three things:

1. every pinned id resolves in `spec/SPECIFICATION.md` (via `clause_ids` from Note 6);
2. every named discharge site file exists and the clause id still appears in it — the
   candidate sites from the evidence pass are
   `crates/happenstance-core/src/store.rs:146-165` (ES-23), `store.rs:167-193` (ES-24),
   `crates/happenstance-core/src/tag.rs:29-47` (VT-15), `tag.rs:255-260` (VT-17),
   `crates/happenstance-core/src/event.rs:404-418` (VT-3, ES-17),
   `crates/happenstance-core/src/append.rs:29-47` (ES-40), plus ES-19
   (`references/evaluation/phase-4-5-reconciliation.md:119-142`);
3. the derived count agrees with a hand-written count kept next to it.

Point 3 is RS-81-5 — "derive the fact, keep the hand-written intention, and make the
failure say which one moved" (`standards/rust/81-checks-that-cannot-be-types.md:335`) —
and it is the same argument `spec_trace` makes for *checking* §1.3 while refusing to
generate it (`spec_trace.rs:38-56`): a number a human computed by reading the document
is the only independent evidence that the parser reads it the way a person does.

**The set is re-derived, not copied.** `RUNBOOK.md:3777-3845` states nine
(`:3837`); the evidence document enumerates eight distinct ids plus two rows explicitly
*not* MUST-discharges (`references/evaluation/phase-4-5-reconciliation.md:136-142`). The
arithmetic does not close from the evidence alone. Re-derive against
`spec/SPECIFICATION.md` as it stands, pin whatever is found, and write the reconciliation
down — including, if that is the answer, that the count was nine and is now eight and
why. A `const` whose comment says "nine, per RUNBOOK" while holding eight entries is the
defect BR-10 exists to prevent, one level up.

Downstream, any later touch of one of those doc comments is bound by
`.kb/governance/rewrite-the-referent-never-the-reasoning.md`: rewriting the *referent*
of a discharging comment is permitted, rewriting its *reasoning* is a re-discharge.
HS-P0023 is the project that will do it and sits behind this pin in the DAG.

#### 8. What must not move

- The precedence chain in `standards/rust/README.md:23-29` is not extended. The
  narrative tree sits inside it and adds no tier (`initiative.md:218-219`).
- Nothing here amends, discharges or restates a `SPECIFICATION.md` clause. AC-007 and
  AC-008 *read* the document; they never write it. `cargo xtask spec-trace` remains the
  only writer of §7.1/§7.2, and only under `--write`.
- CLAUDE.md's five binding constraints are untouched: no `#[async_trait]`, no `serde`
  in `happenstance-core`'s default features, `read` stays non-`async`, generic code
  binds `EventStore`, and the MSRV is not moved. Fences that demonstrate the library
  must respect them, and the fence that violates one is caught by the compile step
  rather than by a reviewer.
- `xtask/src/lint_constitution.rs` and `xtask/src/constitution.rs` are not refactored to
  share code with the new checker on this project's time. RS-81-3 scopes a scanner to
  the directory whose behaviour it constrains; a shared abstraction over two trees makes
  one error message answer two questions, and the shape is cheap to copy.

#### 9. Decisions left open to the implementer, with the consequence attached

- **Where the tree lives** (`docs/` repurposed vs. a new sibling). Architecturally
  indifferent, with two consequences that hold either way: the `affected.rs` arm in
  Note 1 CR-5 must be added in the same change, and `docs/README.md:25-29` — which
  already states the pin-by-path rule in prose and lists the trees the gate reads — must
  be updated in the same change or it becomes false. If the tree lands under `docs/`,
  the `a_docs_only_change_selects_nothing` test (`affected.rs:650-654`) must be
  re-examined rather than deleted: it may need a path that is still genuinely inert.
- **Whether the new checker joins `affected::run`'s unconditional list**
  (`affected.rs:120-125`). Precedent is ambiguous — `lint_constitution` is *not* on that
  list, while the five lints and `spec_trace` are. Recommendation: add it, on the module
  docs' own argument at `affected.rs:28-36` (it is a file read that finishes in the time
  cargo takes to decide `xtask` is up to date, and a prose-only story is exactly the
  case a package-shaped gate reads nothing for). If it is added, note the divergence
  from `lint-constitution`'s absence rather than leaving two inconsistent precedents.
- **Step naming.** Lower-case and descriptive, in the register of `"documentation"`
  (`main.rs:290`), `"specification traceability"` (`:315`) and `"the constitution's
  examples compile"` (`:488`). Whatever is chosen becomes a string other code depends
  on by value (`lint_steps`, `steps_named`), so choose once.
- **Page-file naming and module derivation.** Free, but must be a pure function shared
  by both directions of the orphan check (Note 2).

#### 10. AC-010 — the exact list the "What this does not verify" section must carry

State these first in the module docs, not last, for the reason `lint_constitution.rs:11-13`
gives ("a check whose limits are undocumented is read as a guarantee"):

1. **Compiles but no longer demonstrates.** The step cannot detect code that still
   compiles while no longer showing what the surrounding prose claims — the initiative's
   own worked example (`_discovery/research/02-…`, finding at line 46). This is the
   headline limit and the reason HS-P0024's friction log is not substitutable.
2. **Doctests do not receive the workspace lint set.** `cargo clippy` does not lint
   doctests at all, so `unwrap_used = "deny"` and the pedantic group are unenforced
   inside every fence (`xtask/src/constitution.rs:27-29`).
3. **The `RUSTDOCFLAGS` gap, stated as measured rather than as cited.** The repository's
   own probe found `RUSTDOCFLAGS=-D warnings` recovers rustc's *default-on* lints inside
   a doctest (`constitution.rs:31-36`), while the upstream reports say `cargo test --doc`
   drops the variable (rust-lang/cargo#13697 → rust-lang/rust#67533,
   `_discovery/research/02-…` line 45). Re-run the probe for the new step and record
   what it actually does here; do not copy either claim. Whatever it does, the recovery
   is partial and switching mechanisms does not fix it.
4. **The reported file is the harness, not the markdown** (Note 3).
5. **A Rust example tagged `text` is invisible.** The fence check rejects *untagged*
   fences because rustdoc compiles them as Rust; a fence deliberately tagged `text` is
   neither compiled nor flagged. That is the back door AC-004's allowance list narrows
   but does not close.
6. **It says nothing about teaching.** One sentence, unhedged, so that no downstream
   project can read a green step as evidence of comprehension (project DoD item 8).

RS-81-1 asks for more than the sentence: prove each blind spot in the check's own tests
where it can be proved. The testing brief owns that; the architectural obligation is
that the module has a place for it.

#### 11. Anchors

`xtask/src/main.rs:72-103,105,290-301,488-492,642-707,799-826` ·
`xtask/src/lib.rs:21,28` · `xtask/src/constitution.rs:1-44` ·
`xtask/src/lint_constitution.rs:9-44,55-61,169-198,208-246,423-458,601-673` ·
`xtask/src/spec_trace.rs:1-57,67,85-105,122-160,218,1357,1746` ·
`xtask/src/affected.rs:26-44,112-140,200-260,650-700` · `xtask/Cargo.toml:9-36` ·
`docs/README.md:25-29` · `.redkiln/config.yaml:28-40` ·
`standards/rust/80-the-gate.md:11-17,98-104,181-187,245-251` ·
`standards/rust/81-checks-that-cannot-be-types.md:11,209,335` ·
`standards/rust/README.md:23-29` · `spec/SPECIFICATION.md:280` ·
`.kb/governance/rewrite-the-referent-never-the-reasoning.md` ·
`RUNBOOK.md:918-925,3777-3845` ·
`references/evaluation/phase-4-5-reconciliation.md:119-142` ·
`.bklg/docs-that-teach/_discovery/research/02-compiled-prose-tooling-mdbook-test-doc-comment-skeptic-doc-i.md`

## UX brief

### Intent

This project's reader-facing question is narrow and unusual: **not what the pages
say, but whether what a page shows is the same thing the gate checked.** Every other
project in this initiative authors teaching; this one decides which *forms* teaching
is allowed to take, because a form the gate cannot see is a form that can lie.

The brief is held to contract grain. It states what must be true of the reading
experience and what would falsify it. It does **not** resolve DT-7 — that is
`_design.md`'s, and `design.capture` is deliberately absent from
`.redkiln/config.yaml`, so the perceptual review is a standing skip and the written
resolution is the only record there will ever be.

**The medium is not a web app.** The surfaces are rustdoc's rendered output and
whatever narrative surface AC-001 pins by path. The "design system" is the
ecosystem's own rendering primitives — rustdoc's per-item collapse/expand, its
search index, mdBook's sidebar TOC — plus whatever the hosting decision adds. The
strongest UX requirement here is a negative one, and the dossier already establishes
it: **before adding any affordance, confirm the medium does not already render the
equivalent for free** (`_discovery/distillation/interaction-patterns.md`,
Anti-patterns, final bullet).

---

### The tension this project owns: DT-7

> Whether adapter- or feature-scoped content uses hidden panels.
> *(a) tabs or folds; (b) always-visible, longer pages; (c) separate pages per scope.*

DT-7 is the only design tension assigned to HS-P0020, and it lands here rather than
in a content project because the deciding factor is mechanical, not editorial:
whether an inactive panel is inside the checked surface. AC-006 is its acceptance
criterion, and story `hidden-content-resolution` is where it is discharged.

**What the evidence actually establishes, and what it does not.**

| Claim | Status |
| --- | --- |
| `mdbook-tabs` is real, maintained, installable tooling | Established (`interaction-patterns.md`, "Tabs", Proven in the wild) |
| The fanout suits tabs — six-plus adapters, short crate-name labels | Established; this is why the pattern is tempting rather than obviously wrong |
| An inactive panel's code sample is included in `mdbook test`, the search index, or Ctrl-F/print | **Unverified.** Nothing in the plugin's own documentation states it either way |
| Tabs are among the most-copied and most-broken UI patterns on the web | Established; a 2024 accessibility guide catalogs six recurring anti-patterns, including inactive panels hidden by CSS opacity that remain in the accessibility tree and are announced anyway |
| Hiding essential information behind an accordion is a named failure mode | Established — NN/g states it by name: valuable content hidden under a fold "may be missed altogether" |

The unverified row is the whole decision. An unconfirmed property is not a safe one,
and this repository has already shipped exactly this shape once: a documentation gate
step that printed `skipped` on all three runners while two documents vouched for it
(`RUNBOOK.md:918-925`). A tab whose inactive panel is silently unchecked is that
failure reproduced at the documentation layer.

**The precedent that makes this sharper than a style question.** A reviewer here
once added a sixth event type, updated a fold, and forgot the query it should have
stayed in sync with — an invariant stated once visibly and once behind a fold, with
nothing catching the drift. A page that states a constraint in prose and again inside
a collapsed admonition reproduces that shape exactly.

**What would settle it** — and the dossier names this itself: deliberately break the
code inside a non-default panel and check whether the gate catches it. That is the
same falsification standard AC-003 already imposes on every other page, which means
DT-7 needs no new instrument, only the existing one pointed at a hidden branch.

---

### Reader states this project must make true

Framed as what the person is trying to do. Mechanism is named only where the
mechanism *is* the experience.

| State | What the reader is trying to do | What must be true | What is true today |
| --- | --- | --- | --- |
| Reading any fenced example | Trust that the code in front of them still compiles against the crate they installed | Every fence on the page was compiled by the gate, mandatorily and without a probe that can skip | No narrative tree exists; nothing compiles prose |
| Meeting an opted-out fence | Know that an uncompiled block is uncompiled | An `ignore`-class fence is either rejected or appears on an enumerated allowance list a human approved | `ignore` has already returned by the back door once, in this repository's own testkit doctests |
| Meeting hidden content | Not be taught a constraint that the check never saw | Either hidden branches are provably inside the checked surface, or they carry no load-bearing claim | Undecided; DT-7 open |
| Following a clause citation | Land on the clause, not on a paraphrase of it | The citation resolves, and a dangling one fails the gate | No citation-resolution check exists for narrative pages |
| Arriving from a clean clone | Read rendered pages without knowing a build step exists | The narrative surface builds and renders as part of `cargo xtask ci`, not as a step someone remembers | Nothing to build |

### Falsifiers

Each is observable, and each would mean this project did not deliver its reader
contract even if every test were green.

- A fence renders on a page and no compiler ever read it.
- A reader expands a fold, tab or panel and finds a `MUST`, an invariant, or a
  constraint that the visible-by-default text does not also carry.
- Breaking the code inside a hidden branch leaves the gate green.
- A page cites a clause id that does not exist, and the gate passes.
- The narrative surface requires a manual build step, so a clean checkout renders
  something different from what CI checked.
- An affordance is added — a nav widget, a breadcrumb, a custom index — that rustdoc
  or the sidebar TOC already provided.

### What this brief deliberately does not decide

- **The hosting shape** (docs.rs-only versus a separate rendered surface). Owned by
  this project, but by the architecture brief and AC-001, not here.
- **Which need each page answers, and the fold-line rule itself.** HS-P0021
  `page-need-discipline` owns DT-8 and BR-11's editorial half; this brief owns only
  the demonstration that a hidden branch is or is not mechanically checked. The seam
  is stated in both projects' non-goals: one owns the rule, the other owns the machine.
- **Any teaching content.** HS-P0022 and HS-P0023 author pages; this project decides
  what forms a page may use.

### Grounding

`_discovery/distillation/interaction-patterns.md` — "Progressive disclosure",
"Tabs (adapter-scoped or feature-scoped content)", "Collapsible admonitions /
accordions", Anti-patterns bullets 2-4 and the final bullet, Open design tension 1 ·
`_discovery/research/05-interaction-pattern-prior-art.md` ·
`RUNBOOK.md:918-925` (the step that looked wired and was not) ·
`spec/SPECIFICATION.md:280` (clause ids are stable, never renumbered) ·
`.redkiln/config.yaml` (`design.capture` absent — perceptual review is a skip)

---

## Testing brief

### Intent

This project has no runtime behaviour to unit-test in the ordinary sense — it
*is* a static check, so the check's own correctness is the product, and the
project's Definition of Done items 2 and 3 (`project.md` DoD) demand the check be
**seen to fail**, not merely asserted to. Two things must be proven and neither
substitutes for the other: (1) the new checker module rejects the specific wrong
implementations AC-002–AC-008 name, the way `lint_constitution.rs`'s own `mod
tests` proves `parse_citation` rejects a nested-backtick citation
(`xtask/src/lint_constitution.rs:833-841`); and (2) the whole gate, run end to
end from a clean checkout, fails on a page broken on purpose and recovers when
the page is fixed (AC-003, DoD-2) — a `cargo test` green bar over the checker's
own unit tests is not evidence of this, because it never invokes `cargo xtask
ci` itself. CLAUDE.md's rule for the conformance suite states the same
discipline this project must apply to itself: *"Before adding [a rule], name a
plausible wrong implementation it rejects, and write that implementation into
the testkit's own `tests/` if one does not already exist there"* — the
mechanism differs (this is a bin-crate lint, not a `testkit` rule) but the
standard transfers directly, and RS-81-1 states the Rust-specific form of it:
"Prove the check's blind spot in its own tests, then state it in its own
documentation" (`standards/rust/81-checks-that-cannot-be-types.md:11`).

### Acceptance Criteria

Test mix, mapped one project AC at a time. "Tier" follows this repository's
existing four: **static** (a lint/grep-class check with its own `#[cfg(test)]`
unit tests, house style at `xtask/src/lint_constitution.rs:828-878`),
**compile** (a doctest actually compiled by rustdoc — the mechanism itself, not
a test *about* it), **gate-integration** (`cargo xtask ci` or a named subset,
run and observed), **end-to-end/fixture** (a deliberately broken fixture page
walked through the real gate and reverted).

- **AC-001 (pinned tree).** *Static.* A unit test in the new checker module
  constructs a temp directory without the pinned path and asserts the
  `.with_context()` error names the expected path (mirrors the shape at
  `lint_constitution.rs:174-176`'s `bail!`); a second unit test asserts the
  empty-tree case is a hard error, not a vacuous pass — this is exactly RS-81-1's
  "prove the blind spot," because a pinned constant with no vacuity guard is the
  named wrong implementation the architecture brief's AC-001 note already calls
  out. *Gate-integration:* `cargo xtask affected --base main` after a change
  under the tree's path selects the `xtask` package (Note 1 CR-5's obligation) —
  proven by a unit test in `affected.rs` in the shape of
  `a_constitution_atom_selects_xtask` (`xtask/src/affected.rs:688-696`), not by a
  new integration harness.
- **AC-002 (every fence compiled).** *Compile*, primarily: the mechanism
  itself is `cargo test --locked -p xtask --doc` compiling every
  `#[cfg(doctest)] mod` in the lib target (CR-1). Proof that this actually
  exercises real workspace types is a fixture page whose fence calls a
  `happenstance-core` item — deleting that item and re-running the step must
  fail; this is the fixture half of AC-003 below, not a separate test. *Static:*
  a unit test asserting the checker's harness-reading half (Note 2) finds the
  new `mod` for a page that exists, mirroring `check_harness`'s own coverage.
- **AC-003 (seen to fail).** *End-to-end/fixture*, and the only AC in this
  project whose proof is a recorded procedure rather than a `#[test]` function.
  Author one fixture page under the pinned tree whose Rust fence calls a real
  `happenstance-core` (or `happenstance`) item with a claim that is true today;
  run `cargo xtask ci` (or, faster while iterating, `cargo test --locked -p
  xtask --doc`) and observe green; edit the fixture so the claim is false (e.g.
  change an assertion, or reference a signature that no longer matches); re-run
  and observe the step fail, and capture that the failure names the page's
  module and a line inside it (Note 3's residual-degradation point: the file
  named is the harness, not the markdown, so the recorded evidence must show
  what the failure *actually* prints, not what an idealized failure would
  print); revert; observe green again. Both the failing run's output and the
  recovered run's output are recorded in this project's own artefacts, per DoD-2
  ("A gate that has only ever been green is decorative"). This is not
  substitutable by a unit test that calls the checker function directly,
  because AC-003 as written in `project.md` requires observing `cargo xtask ci`
  itself fail, and a unit test never invokes the gate.
- **AC-004 (no silent opt-out).** *Static*, following `check_fences`'s own test
  shape (`lint_constitution.rs:601-673`, tested implicitly through the corpus
  today but exercisable directly): unit tests for (a) an untagged fence is
  rejected, (b) an `ignore`-tagged fence *not* on the allowance `const` is
  rejected naming file and line, (c) an `ignore`-tagged fence that *is* on the
  allowance list passes, (d) a stale allowance entry — naming a fence that no
  longer exists — is itself a problem (Note 5's reverse-sweep requirement,
  same shape as AC-005's reverse check below). Each is a named wrong
  implementation: (b) rejects a checker that only special-cases the comment
  form and silently accepts an unlisted `ignore`; (d) rejects a checker that
  never sweeps the allowance list against current fences, which is exactly how
  `check_harness`'s *reverse* half earns its keep (`lint_constitution.rs:445-446`
  documents the asymmetry this test mirrors).
- **AC-005 (no orphan pages).** *Static*, bidirectional, mirroring
  `check_harness` (`lint_constitution.rs:423-458`) in both directions: (a) a
  page file present in the tree with no matching `include_str!`/`mod` in the
  harness is a problem; (b) a `mod` in the harness naming a page that does not
  exist in the tree is a problem. Both directions need their own test — a
  checker that only implements (a) is the "plausible wrong implementation"
  CLAUDE.md's rule asks to be named and written down, and this project's
  fixture set should include a temp harness stub exercising (b) directly,
  since (b) is unreachable from any real gate run once the harness and tree
  agree (it only fires on a bug in a future edit).
- **AC-006 (hidden content).** *Static* if `_design.md` forbids hidden/tabbed
  content outright — a lint asserting the forbidden markers do not appear
  anywhere under the pinned tree, tested the same way as AC-004. *Fixture*, in
  addition, if `_design.md` instead permits it behind DT-7's falsification: one
  fixture page with a broken claim placed *inside* a non-default panel,
  walked through the same observed-fail/observed-recover procedure as AC-003,
  because DoD-3 states this explicitly ("the same falsification has been run
  *inside* a hidden branch"). Which sub-case applies is the ux brief's call, not
  this brief's; this brief covers both branches so neither is silently skipped.
- **AC-007 (citations resolve).** *Static*, and the unit test is aimed
  specifically at the "hard-error, don't skip" posture Note 6 calls for: one
  test asserting a citation the checker cannot parse is a problem rather than a
  silent non-match (mirroring `lint_constitution.rs:29-44`'s stated reasoning
  and its own `a_citation_with_a_nested_backtick_does_not_parse` test at
  `:834-841`, which is the direct template — copy its shape, not its content).
  A second test asserts a citation naming a real clause id (via the new
  `clause_ids` sibling function, Note 6) passes, and a third asserts a
  nonexistent id fails naming the page. Because `clause_ids` is a thin wrapper
  over `parse_clauses`, no new parser tests are owed here — `spec_trace.rs`'s
  own suite already covers the parse; this project's tests cover only the new
  call site.
- **AC-008 (clause-id pin).** *Static*, three tests mirroring the three
  assertions Note 7 specifies: (1) every pinned id resolves against
  `clause_ids(root)` — feed a fixture `SPECIFICATION.md` slice missing one
  pinned id and assert the checker reports it; (2) every named discharge site
  still contains its clause id — feed a fixture source file with the id
  stripped and assert a problem naming the site; (3) the derived count equals
  the hand-written count next to the `const` — feed a mismatched pair and
  assert the failure names *which one moved* (RS-81-5's explicit requirement,
  `standards/rust/81-checks-that-cannot-be-types.md:335-341`: "make the failure
  say which one moved," not just that they disagree). Before any of these
  tests are written, the set itself must be re-derived against
  `spec/SPECIFICATION.md` as it stands (Note 7's "re-derived, not copied") —
  that re-derivation is evidence-gathering, not a test, and belongs in the
  brief/implementation record rather than in `#[test]` code.
- **AC-009 (clean checkout, no manual step).** *Gate-integration*, three
  separate invocations, because each is wired differently and a step present
  in one and absent from another is exactly the "invisible to `--fast`"
  failure RS-80-1 names: (1) `cargo xtask ci` reaches the new step —
  proven by running it and observing the step's name in the output; (2) `cargo
  xtask ci --fast` — proven the same way, since AC-002's step is mandatory
  (`probe: None`) and `run_fast` runs the entire `REQUIRED` array unfiltered,
  skipping only `OPTIONAL` (`main.rs:853-860`); (3) `cargo xtask affected --base main`
  after a tree-only change selects a run that includes the new step — this is
  the CR-5 landmine from the architecture brief and is the single
  highest-value integration test in this project, because CLAUDE.md's
  `verify:` block wires `affected_gate` as the *actual* story-grain check every
  future story in this initiative runs (`.redkiln/config.yaml:40`), so a gap
  here is invisible until a downstream story's own gate silently compiles
  nothing.
- **AC-010 (limits documented).** *Static*, and the test is documentation
  presence plus, per RS-81-1's "then state it," a matching test proving the
  limit is real rather than asserted: for the compiles-but-no-longer-demonstrates
  blind spot, no test is possible (it is a semantic gap, not a mechanical one)
  — document it as inherently untestable and say so, following
  `xtask/src/constitution.rs:20-36`'s own worked example, which documents this
  exact class of limit without a test backing it because none is possible. For
  the `RUSTDOCFLAGS` gap, the test *is* the re-run of the probe the module docs
  cite (Note 10, item 3): re-run `constitution.rs`'s own probe methodology
  against the new step and record the actual result rather than copying either
  the upstream issue's claim or the existing module's finding verbatim. For the
  `text`-tagged invisible-fence gap: a fixture page with a broken claim inside a
  ` ```text ` fence, run through the gate, observed to pass — proving the
  documented limit is real, not merely plausible, which is RS-81-1's whole
  point ("execute" the limit, don't just assert it).

### Notes

**Fixture pages live inside this project, not in the corpus.** `project.md`'s
risk table is explicit: "This project may write *fixture* pages for the
falsification; it may not write the corpus." The AC-003/AC-006/AC-010 fixture
pages above are throwaway or permanently-retained *test* material — analogous
to `crates/happenstance-testkit/fixtures/MemoryFixture` as the reference
implementation CLAUDE.md names for the conformance suite — not narrative
content HS-P0021/22/23 are responsible for. Whether they are deleted after the
falsification is recorded or kept permanently as regression fixtures (the
stronger choice, since AC-003's procedure should be re-runnable, not just
once-run) is the implementer's call; keeping them is consistent with DoD-2's
"observed to fail" needing to remain observable rather than becoming a claim
no one can re-verify.

**Merge-gate commands**, in the order a contributor would actually run them
while iterating on this project, narrowest to widest:

```console
cargo test --locked -p xtask --doc          # AC-002/003/006/010's compile mechanism alone
cargo test -p xtask                          # this project's own #[cfg(test)] unit tests (AC-001,004,005,007,008)
cargo xtask lint-constitution                # confirms this project did not regress the existing checker (Note 8)
cargo xtask affected --base main             # AC-009's story-grain path; the CR-5 landmine check
cargo xtask ci --fast                        # the bar for a non-terminal project (project.md DoD-6)
cargo xtask ci                                # the full gate; run before the project is called done (DoD-6)
```

`cargo xtask ci` is the merge gate of record (CLAUDE.md, "Commands"); `--fast`
is the interim bar this non-terminal project is held to during implementation,
per `.redkiln/config.yaml` and DoD-6.

**Fixtures and seams to mock — there are almost none, deliberately.** Unlike
the conformance suite (which mocks a store behind `EventStore`), this
project's checks read real files and compile real code; the whole point of
AC-002 is that nothing about the fence's compilation is faked. The one
legitimate seam is the *content* of fixture pages, which stand in for the real
narrative corpus that does not yet exist when this project is implemented
(HS-P0021–23 have not landed). Do not mock `spec/SPECIFICATION.md` itself for
AC-007/AC-008's tests beyond what Note "AC-007" and "AC-008" above already
specify (small in-memory string slices passed to `clause_ids`/the discharge-site
check, not a parallel fixture file) — `spec_trace.rs`'s own tests already prove
the parser against the real document, and duplicating that here would be the
"shared abstraction over two trees" RS-81-3's directory-scoping argument (and
this project's own Note 8) warns against.

**What this brief does not cover.** DT-7's *choice* (forbid vs. prove-safe) is
the ux brief's call, not this one's — this brief specifies the test shape for
both branches so that whichever is chosen has a template ready. Whether the
comprehension claim holds — whether a reader actually learns anything — is
HS-P0024's instrument (the friction log) and is explicitly not testable by
anything in this brief; AC-010's item 6 and DoD item 8 exist precisely so this
project's green tests are never read as evidence of that.

## Deployment brief

### Intent

There is no running service, no user-facing runtime, and no version to publish
independently — `happenstance-core`, `happenstance` and `happenstance-testkit`
are the only publishable crates (CLAUDE.md's `cargo package --list` gate step,
`package.rs`'s `PUBLISHABLE` const per RS-81-5's worked example above), and
`xtask` itself carries `publish = false` implicitly by being a workspace
tooling crate never listed there. "Deployment" for this project means exactly
one thing: **the pinned narrative tree and its render, if any, exist and are
current on every clean checkout, with no manual step, as an ordinary part of
`cargo xtask ci`** (AC-009, DoD-1). This is the only project in the initiative
that earns a deployment brief (`_decomposition.md`, "Warranted briefs per
project"), precisely because it is the only one that decides *where the
checked surface physically lives*.

### Acceptance Criteria

- **AC-009 is this brief's whole scope.** "Hosting and render shape" is called
  out in `project.md`'s in-scope list as this project's own decision — "because
  it is inseparable from which tree is pinned by path and built by the gate"
  (`_decomposition.md`, "Hosting shape") — and is explicitly **undecided** by
  the charter, the decomposition, and the grounding pass (`_grounding.md`
  tension 3). This brief does not decide it either; it states the two live
  options, their deployment consequences, and the decision surface `_design.md`
  must close, per the architecture brief's Note 9.
  - **Option A — `docs/` repurposed, docs.rs-adjacent.** If the pinned tree
    replaces or extends `docs/` (currently near-empty,
    `docs/README.md:1-36`), the "render" may be nothing beyond what `cargo doc`
    already produces plus the doctest-compile step — no separate hosting
    artefact, no separate release step. `docs/README.md:25-29` already states
    the pin-by-path convention this option inherits directly.
  - **Option B — a new sibling tree with its own render.** If a
    human-readable rendered surface is wanted beyond what rustdoc produces
    (e.g. an mdBook-shaped static site), that render is **a separate concern
    from the compiling step** per the architecture brief's Note 3 ("The render
    is not the check... Do not let a renderer become the thing that compiles
    the fences"). Whatever renders it must still be reachable from `cargo
    xtask ci` with `probe: None` if the render's *presence* is asserted by any
    conformance check, or must be explicitly out of the mandatory gate (and
    that exclusion documented) if it is presentation-only.
  - Either way, **AC-009's clean-checkout requirement is unconditional**: no
    `README.md` instruction telling a human to run a separate command. The
    precedent this project exists not to repeat is named directly in
    `project.md`'s risk table: `RUNBOOK.md:918-925`, a probe-gated step that
    printed `skipped` on all three CI runners while two documents vouched for
    it.
- **Config gating / feature flags — N/A.** Nothing in this project is
  runtime-configurable. The checker is a compile-time `xtask` module; there is
  no environment variable, feature flag, or config file that changes its
  behaviour at run time. (`RUSTDOCFLAGS` is set in the `Step`'s own `env`, not
  read from ambient configuration — RS-80-3 — which is a build-time constant,
  not a deployment-time gate.)
- **Migration / backfill — N/A.** No data store, no schema, no persisted
  state. The only "existing content" this project's own scope produces is its
  fixture pages (testing brief), which are test material, not user data.
- **Rollback posture.** The rollback unit is the commit that adds the pinned
  tree, the checker module, and the `REQUIRED` step — ordinary `git revert`,
  because nothing here is stateful and nothing is published independently. The
  one rollback hazard worth naming explicitly: if a *later* project
  (HS-P0021–23) has already authored pages inside the pinned tree by the time
  a rollback of this project is considered, reverting this project's `REQUIRED`
  step without also reverting or re-homing those pages leaves them unchecked —
  which is precisely the "retro-fitted check" failure mode the architecture
  brief's dependency ordering (`project.md`, "How this advances the
  initiative") exists to prevent by sequencing this project first. In
  practice this means: do not roll back HS-P0020 in isolation once HS-P0021+
  has merged content into the tree it pins.
- **CI implication.** One new `Step` in `REQUIRED` (mandatory,
  `probe: None`), one new `mod` in the `xtask` bin crate (checker) and one new
  `mod` in the `xtask` lib crate (harness) — see the architecture brief's Note
  1 for the exact composition roots. No new CI job, no new runner
  configuration, no new secret: the step runs wherever `cargo xtask ci`
  already runs (`CLAUDE.md`'s command list; this repository's existing `ci.yml`
  workflow, unchanged in shape). If Option B's render needs a tool absent from
  the current CI image (e.g. `mdbook`), that tool becomes a new CI
  installation step — but the architecture brief's Note 3 already forecloses
  routing the *compiling* mechanism through such a tool (`probe:` shape,
  DR-03), so at most this affects a presentation-only, non-mandatory step, and
  that step's absence-handling must use `probe: Some(...)` deliberately, with
  the skip message legible (the `RUNBOOK.md:918-925` incident is exactly a
  probe whose skip was not legible enough for anyone to notice).
- **Release path if a published version changes — N/A, with one caveat.**
  `happenstance-core`, `happenstance` and `happenstance-testkit` are the only
  crates this workspace publishes, and none of this project's changes touch
  their public API (the fences call existing public items; they do not add
  any). The caveat: if fixture pages call an item that a *later*, unrelated
  change removes or renames, the AC-002 gate step is what catches it — that is
  a correctness property of this project's design, not a deployment concern,
  and is already covered by the testing brief's AC-002 entry. No crate version
  bump, changelog entry, or `CHANGELOG.md` line is owed by this project on its
  own, since nothing it adds is part of any published crate's surface
  (`xtask` is workspace tooling, never published).

### Notes

**Why "deployment" is this narrow, stated once so it is not re-litigated per
AC.** This is a `publish = false` tooling and documentation project inside a
Cargo workspace, not a service with environments, traffic, or a rollout
sequence — the vocabulary of feature flags, canaries and staged rollout does
not map onto anything this project produces. The one genuine deployment
decision is hosting/render shape (Option A vs. B above), and it is deferred to
`_design.md` on purpose: the architecture brief's Note 9 lists it as a decision
"left open to the implementer, with the consequence attached," and this brief
exists to make sure that consequence is read before the choice is made, not
after.

**What must not move, restated for this brief specifically.** Per the
architecture brief's Note 8, this project adds no tier to
`standards/rust/README.md:23-29`'s precedence chain and amends no
`SPECIFICATION.md` clause. Neither is a "deployment" concern in the ordinary
sense, but both bound what Option A/B may legitimately mean: neither hosting
choice may make the rendered surface a second normative source competing with
`spec/SPECIFICATION.md` — AC-007/AC-008 already ensure citations resolve
*into* the specification, and the render, whichever shape it takes, is a view
onto checked prose, not an independent claim surface.
