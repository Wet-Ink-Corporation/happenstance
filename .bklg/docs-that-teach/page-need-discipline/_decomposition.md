---
item: HS-P0021
stage: briefs
created: 2026-08-17
updated: 2026-08-17
---

# Briefs — Page-Need Discipline

This file holds every brief for HS-P0021. Each brief is one `##` section; read the
project charter (`.bklg/docs-that-teach/page-need-discipline/project.md`) and the
grounding pass (`_grounding.md`) alongside it. Three briefs are warranted —
`architecture`, `ux`, `testing` — and no `deployment` brief: this project ships no
rendered surface (`.bklg/docs-that-teach/_decomposition.md:318`).

## Architecture brief

### Intent

Two deliverables and one seam.

1. **A rules tree.** A router plus one rule-atom per rule, in a new directory
   sibling to `standards/rust/`, whose path is a `const` in `xtask/src/` rather
   than a convention. It states the enumerated set of answered-needs (DT-2/DT-3),
   the fold line (DT-8), the declaration form, and the citation rule — in the
   trigger-table + `Do`/`Not`/`Rejects` shape `standards/rust/` already runs at
   twenty-seven-atom scale (`standards/rust/README.md:1-21`,
   `standards/rust/00-prime-directives.md:1-16,19,30,54,78`).
2. **One checker.** A bin-crate module in `xtask/src/`, modelled directly on
   `xtask/src/lint_constitution.rs`, mounted as a mandatory `Step` in `REQUIRED`,
   that reads the **pages** tree HS-P0020 pins and fails by file and line when a
   page declares zero needs, two needs, or a need outside the enumerated set — and
   that reads its **own** tree for the same self-consistency facts
   `lint_constitution` already checks about the constitution.

The seam is the declaration form (DR-05): the pages this checker reads are pinned,
rendered and compiled by HS-P0020, and the form in which a page states its one need
must be readable by *both* that surface and this lint. It is a jointly-owned
contract, and Note 2 states it as one.

**Scope boundary, architectural rather than rhetorical.** This brief specifies a
machine that proves a page has *declared* exactly one need from a closed set. It
specifies nothing that proves the page *answers* only that need — that is DR-07's
written reviewer procedure, owned by the ux brief, and the checker's own
documentation must say so first rather than last (RS-81-1,
`standards/rust/81-checks-that-cannot-be-types.md:11`).

**No Accepted decision atom constrains this work.** All seventeen under
`.kb/decisions/` were read by title in `_grounding.md` ("Accepted decision atoms
constraining this project"); none governs documentation trees, gate structure or
narrative-content conventions. The binding authority here is sub-ADR and is:
`CLAUDE.md`; the precedence block at `standards/rust/README.md:23-29`; the
pin-by-path rule stated in prose at `docs/README.md:25-28`; and
`standards/rust/80-the-gate.md` / `81-checks-that-cannot-be-types.md`.
**No tension with an Accepted ADR exists, and this project needs no ADR** — writing
one would itself breach the initiative's non-goal against extending the precedence
chain (`project.md`, "Out of scope"). Three deliberate divergences from in-repo
*precedent* are taken in Notes 4 and 6; all three are conventions, not decisions,
so none needs an amendment — but each must be stated in the new module's own
documentation, not left in this brief.

### Acceptance Criteria

Architectural obligations, one per project AC. Each names where the work mounts.

- **AC-001 — the home is pinned, not conventional.** The rules tree's path and its
  router are `const`s in the new checker module, in the exact shape of
  `ATOM_DIR` / `ROUTER` (`xtask/src/lint_constitution.rs:55,58`). Both guards are
  required, not one: the directory read is `?`-propagated with `.with_context()`
  naming the expected path (`lint_constitution.rs:212`), **and** an empty-tree
  `bail!` mirrors `lint_constitution.rs:174-176` — *"{ATOM_DIR} holds no atoms, so
  every check below is vacuous"*. A pinned constant without the vacuity guard
  reports green over a tree someone emptied, which is precisely the decorative-step
  failure `RUNBOOK.md:920-925` records. The same pair of guards applies to the
  **pages** tree the checker reads (Note 2).
- **AC-002 — inside the precedence chain, without extending it.** Discharged
  architecturally by never opening `standards/rust/README.md` for writing. The only
  code in the workspace that writes that file is `check_router`'s `Mode::Write` arm
  (`xtask/src/lint_constitution.rs:334-384`), which is scoped to its own `ROUTER`
  const and is not touched by this project. The new router *states* its rank
  relative to `standards/rust/README.md:23-29` in prose; the design stage picks the
  rank (the grounding's reading is at or near the constitution-atom tier, since
  this is a project convention of the same kind — `_grounding.md`, "Non-goals and
  the precedence-chain boundary").
- **AC-003 / AC-004 / AC-005 — DT-2, DT-3, DT-8 resolved.** These are `_design.md`
  decisions, not code. The architectural obligation attached to them is single:
  **the enumerated need set exists exactly once, as a `const` in the checker, and
  the router's table is generated from it.** Use the `<!-- BEGIN GENERATED -->` /
  `<!-- END GENERATED -->` region and the `Mode::Write` rewrite
  (`lint_constitution.rs:388-421,334-384`), so `--write` regenerates and plain
  `check` reports the disagreement. The reason is stated in-house at
  `xtask/src/spec_trace.rs:122-160`: a vocabulary spelled in more than one place is
  *"three lists that must agree"*, and adding a member half-lands. A need set
  written once in prose and once in Rust is that defect with two members.
- **AC-006 — every governed page declares exactly one need.** The parse is a pure
  function over one page's text returning the declaration and its line number —
  the shape `Rule` and `Fence` already carry (`lint_constitution.rs:143-163`) — so
  the same function serves the counting check, the enumeration check and the
  checker's own tests without a second parser. The pages tree is HS-P0020's; see
  Note 2 for why its path is *shared*, not re-declared.
- **AC-007 — seen to fail, by file and line.** The architectural obligation is that
  the mechanism *can* name a page and a line at all: every problem is a formatted
  `"{PAGE_DIR}/{file}:{line} — …"` string pushed onto one `Vec<String>`, and the
  run reports **all** of them before `bail!`-ing with the count
  (`lint_constitution.rs:169-198`). A check that stops at the first problem turns
  one review cycle into six. The testing brief owns the observation and the ledger
  record; this brief owns the fact that the message is legible when it happens.
- **AC-008 — the wrong page is in the checker's own tests.** The named wrong
  implementations (a page with two declarations; a page with none; a page naming an
  unenumerated need) live in a `#[cfg(test)] mod` in the checker, exercising the
  pure parse and count functions over **synthetic strings**, not over a broken file
  committed into the real tree. This is the convention `xtask/src/affected.rs`
  already uses — its whole test module drives `affected_packages` over synthetic
  path lists (`affected.rs:640-705`) — and it is what lets AC-008's rejection be
  permanent while AC-007's observation is a one-off temporary edit.
- **AC-009 — the reviewer procedure is non-author-performable.** No code. The
  architectural obligation is that the checker's module docs name this limit
  **first** (RS-81-1) and point at the procedure, in the shape of
  `lint_constitution.rs:9-28`. See Note 7 for the required contents.
- **AC-010 — the citation rule, without a second clause parser.** This project owns
  the *rule* ("a page cites a `spec/SPECIFICATION.md` clause id and never restates
  it") and the spot-check procedure; HS-P0020 owns the mechanical resolution check
  and, per its own brief, exposes it as a `pub(crate) fn clause_ids(root: &Path)`
  sibling to `spec_trace::all_rules` (`xtask/src/spec_trace.rs:1746`;
  `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md`, Note 6).
  **Do not build a second one here.** If this checker needs a clause id at all, it
  calls that function. Clause ids are stable and never renumbered
  (`spec/SPECIFICATION.md:280`), which is what makes a citation survive refactors
  a line reference would not.
- **AC-011 — cited by what it governs, and reachable.** Two mounting points, both
  required. The rules tree gets a row in `docs/README.md`'s "Looking for / It is at"
  table (`docs/README.md:12-24`) and is named in the paragraph at
  `docs/README.md:25-28` that enumerates the trees the gate reads — **or that
  paragraph becomes false in the same commit that makes it so.** And at least one
  page in HS-P0020's tree links the rules tree as the reason it is shaped as it is;
  which page is the ux brief's call, but the link target must be a path the checker
  already pins, so a tree move breaks the build rather than the link.
- **AC-012 — staged, never hand-authored.** The playbook atom is written to
  `.kb/_intake/<name>.md` with `kind: playbook` and `authority_tier: guideline`;
  nothing is written into `.kb/` proper, because an accepted decision is never
  edited and hand-authoring the layout of the process without the process was
  reverted once already (`.kb/decisions/README.md:9-13`; `CLAUDE.md`, `0269720`).
  **The Definition of Done's `redkiln validate --kb` line cannot discharge this
  criterion's frontmatter half** — that validator skips any `_`-prefixed directory
  by design (`.kb/_intake/README.md:21-25`), so a green run proves only that
  nothing landed in the checked tree. The ledger evidence must be a manual
  field-by-field check against `.kb/README.md:18-26` (`id`, `title`, `kind`,
  `status`, `authority_tier`, `summary`, `depends_on`/`related`, `source_paths`,
  `last_reviewed`). Named as tension 2 in `_grounding.md`.

### Notes

#### 1. Composition roots — five, and four of them are in one file

This is the integration section. Every capability below mounts into machinery that
already exists and runs; nothing here is a new subsystem. `xtask` has **two targets
that do not share modules**, and the first root states which one this project is
*not* in.

- **CR-0 — the doctest root this project does NOT mount into: `xtask/src/lib.rs:28`.**
  `lib.rs` declares `mod constitution;`, and `cargo test -p xtask --doc` compiles
  the **lib** target's doctests only. HS-P0020's page harness mounts there. This
  project's rules tree does **not**: its examples are markdown page fragments, not
  Rust. Stating this as a root rather than an omission matters, because it inverts
  a rule copied carelessly — see Note 4.
- **CR-1 — the bin module list: `xtask/src/main.rs:64-70`.** The checker is a
  bin-crate module declared alongside `mod lint_constitution;`. One file, one
  `pub(crate) fn run(mode: Mode) -> Result<()>` mirroring
  `lint_constitution::run` (`lint_constitution.rs:170`).
- **CR-2 — the gate array: `xtask/src/main.rs:105` (`REQUIRED`).** One new `Step`.
  `Step`'s contract is at `main.rs:72-103`, and the `probe` doc comment at
  `:89-102` is the normative statement of what `None` means. **`probe: None`** —
  this is a file read with no external tool, so a probe would be a lie (RS-80-1,
  RS-80-2; `standards/rust/80-the-gate.md:11,98`). The invocation is
  `cargo run --locked --quiet -p xtask -- <task-name>`, copied line for line from
  the constitution step at `main.rs:462-479`; `--locked` because it resolves
  dependencies (RS-80-4, `80-the-gate.md:245`). No `env` — there is no rustdoc in
  this step, so RS-80-3 does not apply.
- **CR-3 — dispatch and the by-name subsets: `xtask/src/main.rs:689-700`,
  `:718`, `:799-808`, `:816-826`.** Three edits, and omitting any one half-mounts
  the step:
  - a match arm in `main()` mirroring `Some("lint-constitution") => match … Mode::Check / Some("--write") => Mode::Write` (`:689-700`) — keep the `--write` arm if the router's generated region is adopted (AC-003), and keep the unknown-flag branch;
  - a line in `print_help()` (`:718`);
  - **a name in `lint_steps()` (`:799-808`)** — this check is a member of the
    file-reading family that function's doc comment describes (`:795-798`), so
    `cargo xtask lints` must run it. `steps_named` panics on a name absent from
    `REQUIRED` (`:816-826`), which is the intended failure: a name added to
    `lint_steps` and not to `REQUIRED` is a build-time bug rather than a silent
    omission. The project's Definition of Done runs `cargo xtask lints` under
    `verify.reachability_static`, so this is a load-bearing line, not a courtesy.
- **CR-4 — the story-grain selector: `xtask/src/affected.rs:118-125` and
  `is_inert`'s `INERT` at `:249-260`. This is the landmine, and it is shaped
  differently here than in HS-P0020.**
  - `affected::run` runs a fixed list of file-reading checks **unconditionally**
    before it computes affected packages (`affected.rs:118-125`), for the reason
    its module docs give: *"the packages these read are not the packages the diff
    touched."* **Add the new checker to that list.** This project's entire
    deliverable is prose; a prose-only pull request must not read nothing. Note the
    divergence from `lint_constitution`'s *absence* from that list rather than
    leaving two inconsistent precedents — the five lints and `spec_trace` are on it.
  - `affected_packages` has one arm for prose outside every member
    (`affected.rs:214-221`): `README.md` and `standards/rust/` select `xtask`
    *because their examples compile as that crate's doctests* — the reasoning is
    written out at `:245-247`, and it does **not** transfer. Anything unrecognised
    widens to the whole workspace (`:222-223`, asserted by
    `an_unrecognised_path_widens_rather_than_narrows` at `:644-648`). A new tree at
    `standards/<name>/` is today unrecognised: `standards/` is not on `INERT` and
    the arm matches the literal prefix `standards/rust/`. So a rules-tree edit
    widens the gate to every package — safe, and slow, and wrong for the reason
    `INERT`'s own doc gives: *"deliberately a list"* (`:232-234`).
  - **The correct treatment is the pair, in the same change**: add the new tree's
    exact prefix to `INERT`, and rely on CR-4's unconditional-list entry to check
    it. That combination makes a rules-only change run the checker and compile
    nothing, which is exactly right. Two unit tests, mirroring the two that already
    guard the constitution's arm: one asserting the new tree selects no package
    (shape of `the_relocated_trees_stay_inert`, `:662-673`), and one asserting
    `standards/rust/` still selects `xtask` (`a_constitution_atom_selects_xtask`,
    `:688-696`), so a lazily-broadened `"standards/"` prefix cannot silently
    un-compile the constitution.

#### 2. The seam with HS-P0020 — the declaration form, stated as a contract

DR-05 is the only genuine cross-project coupling and the charter's top risk. Fix it
as three obligations rather than as a conversation:

1. **One path constant for the pages tree, not two.** HS-P0020's checker pins the
   pages tree by path (its brief's AC-001). This checker reads the same tree. Two
   `const`s naming one directory is the same defect `spec_trace.rs:122-160` names
   for clause families, and it is worse here because the second one drifts silently
   the day the tree moves. Make HS-P0020's page-tree constant `pub(crate)` and
   reference it. This is *not* in tension with RS-81-3
   (`standards/rust/81-checks-that-cannot-be-types.md:209`, "scope the scan to the
   directory whose behaviour the check constrains"): RS-81-3 forbids one scanner
   ranging over two trees, not two scanners agreeing on where one tree is.
2. **The declaration is machine-readable in the same document a human reads.**
   Not a sidecar index, not a filename convention. The model is
   `check_fences`'s inline `<!-- ignore: <reason> -->` requirement
   (`lint_constitution.rs:637-643`): *"without one it is indistinguishable from an
   example that stopped compiling."* A need declared in a sidecar is
   indistinguishable from a need nobody thought about. The concrete spelling is
   `_design.md`'s, constrained by whatever HS-P0020's hosting shape can render —
   record the form **and the hosting assumption it rests on**, so a change in the
   latter visibly invalidates the former.
3. **Parse once, in a pure function, in this checker.** HS-P0020 renders the
   declaration; this project *judges* it. If the render needs to read it too, it
   reads the same page text, not a derived artifact.

#### 3. Data flow

```text
rules tree (pinned by const, this project)      pages tree (pinned by const, HS-P0020)
       │                                                       │
       │  read_dir ─► rule atoms ─► shape / router checks      │  read_dir ─► pages
       │                    │                                  │        │
       │      NEEDS const ──┴──► generated router region       │        └─► parse declaration
       │        (Mode::Write rewrites; check reports)          │              (pure fn, line-carrying)
       │                    │                                  │                      │
       └────────────────────┴──────────────────────────────────┴──────────────────────┘
                                            │
                             one Vec<String> problems ─► all reported ─► bail!("{n} problem(s)")
```

The `NEEDS` const is the hinge: it is the enumeration the rules tree documents *and*
the set a page's declaration is validated against. Anything that reads it from
prose instead re-introduces the two-lists defect. Report every problem before
failing, as `lint_constitution::run` does (`:190-198`).

#### 4. Divergence 1 — the untagged-fence rule inverts, and copying it unexamined is the trap

`check_fences` rejects an untagged fence with *"an untagged fence is compiled as
Rust; tag it `rust` or `text`"* (`lint_constitution.rs:611-616`). That reason is
true only because `standards/rust/` **is** registered in the doctest harness
(`xtask/src/constitution.rs`, checked bidirectionally by `check_harness`,
`lint_constitution.rs:423-458`). This project's rules tree is deliberately **not**
registered (CR-0), so nothing compiles its fences and the rustdoc hazard does not
exist.

The consequence is the opposite of harmless: a fence tagged `rust` in a rule atom
is a Rust claim that **nothing in the workspace checks**. So the rule for this tree
is stricter, not looser — **reject a `rust`-tagged fence outright**, with a message
saying why (its examples are page fragments; a Rust example belongs in
`standards/rust/`, where it is compiled). Keep the untagged rejection as well, so a
future decision to register the tree cannot be undermined by fences written under
the assumption that nothing reads them.

There is no `check_harness` equivalent for this tree, and its absence must be
stated in the module docs alongside the reason — otherwise the next reader sees a
checker that looks like `lint_constitution` with a check missing.

#### 5. Divergence 2 — a generated router region for a tree the gate does not compile

`check_router`'s generated region (`lint_constitution.rs:334-421`) exists so the
router's table cannot disagree with the atoms. Reuse it verbatim, including the
`Mode::Write` arm and the *"run `cargo xtask <task> --write`"* problem message
(`:379-382`). The divergence worth naming: for `standards/rust/` the region is a
convenience over a corpus the compiler also reads, whereas here it is the **only**
mechanism preventing the enumerated need set from existing twice. Say that in the
module docs, because it changes how seriously a reviewer should treat a `--write`
diff.

The link check in the same function (`:344-356`) — every `.md` a router links must
resolve — is copied as-is and is what discharges half of AC-011's reachability.

#### 6. Divergence 3 — no shared abstraction with `lint_constitution`

`xtask/src/lint_constitution.rs` and `xtask/src/constitution.rs` are **not**
refactored to share code with the new checker on this project's time. RS-81-3
(`standards/rust/81-checks-that-cannot-be-types.md:209`) scopes a scanner to the
directory whose behaviour it constrains; a shared abstraction over two trees makes
one error message answer two questions, and the shape is cheap to copy. This
matches HS-P0020's Note 8 and keeps three checkers independently readable.

#### 7. What the checker's "What this does not verify" section must carry

State these **first** in the module docs, for the reason `lint_constitution.rs:11-13`
gives — *"a check whose limits are undocumented is read as a guarantee"* — and
prove each in the checker's own tests where it can be proved (RS-81-1,
`81-checks-that-cannot-be-types.md:11`):

1. **It checks that a need is *declared*, never that the page *answers* it.** One
   sentence, unhedged, naming DR-07's reviewer procedure as the instrument for the
   rest. This is the headline limit and project DoD item 8's whole subject.
2. **It does not judge whether the enumerated set is the right set.** DT-2 and
   DT-3's resolution is `_design.md`'s; the checker enforces membership in whatever
   set is written down.
3. **The fold rule (DT-8) is enforced only to the extent the chosen markers are
   textual.** If the hosting shape produces hidden content by a marker the checker
   can see, the fold rule is checkable; if it produces it by rendering behaviour,
   it is reviewer-only. Say which, once the hosting shape is known — do not leave
   the reader to infer it.
4. **It does not resolve clause ids.** That is HS-P0020's check (AC-010 above);
   this checker's citation rule is prose the reviewer applies. A page could cite a
   clause id that exists and still restate its content, and no byte count sees it.
5. **A rules tree that is empty passes every check below the vacuity guard** — which
   is why the guard is `bail!` and not a warning (AC-001).
6. **Length is not quality.** If any minimum-length heuristic is adopted from
   `MIN_REJECTS_CHARS` / `MIN_CHARS_PER_RULE` (`lint_constitution.rs:82`), carry
   the same caveat it carries: *"a long and vacuous `**Rejects.**` passes here. The
   instrument for that is an adversarial reader, not a byte count"* (`:14-19`).

#### 8. What must not move

- **`standards/rust/README.md:23-29`** is not edited. The new tree states its rank
  inside the chain and adds no tier (`project.md`, AC-002; initiative non-goal).
- **`xtask/src/lint_constitution.rs` and `xtask/src/constitution.rs`** are not
  touched (Note 6). `standards/rust/`'s arm in `affected.rs:214-221` is not
  broadened to `standards/` (CR-4).
- **Nothing is written into `.kb/`.** The playbook atom is staged under
  `.kb/_intake/` and ingested by HS-P0025 (AC-012). `.kb/maps/domain-map.md` is not
  edited by this project at all — a new domain area is added during
  `/redkiln:kb-ingest`'s Maps phase (`.kb/maps/README.md:34-36`), and the
  charter's stronger phrasing ("an appended section, never an edit") is this
  project's inference rather than a quoted rule (`_grounding.md`, tension 4).
- **Nothing here amends, discharges or restates a `SPECIFICATION.md` clause.** The
  discipline requires pages to *cite*; the checker never writes the specification,
  and `cargo xtask spec-trace` remains the only writer of its generated sections.
- **`.redkiln/templates/` is untouched**, so `redkiln doctor` still reports exactly
  six `template-drift` advisories; `redkiln adopt --templates` is never run
  (`CLAUDE.md`).
- **CLAUDE.md's binding constraints are untouched.** No port, no crate, no feature
  in this project's diff.

#### 9. Decisions left to the implementer, with the consequence attached

- **The tree's directory name.** Free, but it becomes a `const` two files depend on
  by value and a prefix in `INERT`; choose once, and add the `affected.rs` prefix
  and its two tests in the same change (CR-4).
- **The step's name string.** Lower-case and descriptive, in the register of
  `"the Rust constitution is internally consistent"` (`main.rs:466`) and
  `"specification traceability"`. It is depended on by value in `lint_steps`
  (`:799-808`) and `steps_named` panics on a mismatch (`:816-826`).
- **Whether the checker also reads the rules tree's rule *shape*** (a `SECTIONS`
  analogue to `lint_constitution.rs:67-81`'s five markers). Recommended: yes, and
  keep it to structural markers. It is the mechanism that makes AC-008's
  "named wrong implementation per rule" a checked property of the corpus rather
  than an authoring habit — the same argument `check_shape` makes at `:477`.
- **Whether the page-need index is generated into the router.** Attractive, and it
  crosses the project seam: the index's source is HS-P0020's tree. If adopted, it
  is a second generated region with the same `Mode::Write` treatment, and it makes
  the rules tree change on every page addition. Defer unless the ux brief wants it.
- **Whether a rule atom may carry a `> **Load when:**` line the checker enforces.**
  `load_when` (`lint_constitution.rs:247-257`) is a two-line parse and it is what
  makes the router's generated region possible; adopting it is nearly free.

#### 10. Anchors

`xtask/src/main.rs:64-70,72-103,105,462-479,689-700,718,795-808,816-826` ·
`xtask/src/lib.rs:28` ·
`xtask/src/lint_constitution.rs:9-28,55-61,67-81,82,143-163,169-198,208-246,247-257,334-421,423-458,477,601-643` ·
`xtask/src/affected.rs:118-125,214-223,232-260,640-705` ·
`xtask/src/spec_trace.rs:122-160,1746` ·
`xtask/src/constitution.rs` ·
`standards/rust/README.md:1-21,23-29` ·
`standards/rust/00-prime-directives.md:1-16,19,30,54,78` ·
`standards/rust/80-the-gate.md:11,98,181,245` ·
`standards/rust/81-checks-that-cannot-be-types.md:11,209,335` ·
`docs/README.md:12-24,25-28` ·
`spec/SPECIFICATION.md:280` ·
`.kb/decisions/README.md:9-13` · `.kb/playbooks/README.md:7-19,29-31` ·
`.kb/_intake/README.md:7-8,21-25` · `.kb/README.md:18-26` ·
`.kb/maps/README.md:34-36` · `.kb/maps/domain-map.md:35` ·
`.kb/governance/rewrite-the-referent-never-the-reasoning.md` ·
`RUNBOOK.md:920-925,931-935` ·
`.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md:397-403,404-410,484-497,498-512,568-584` ·
`.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` ·
`.bklg/docs-that-teach/page-need-discipline/_grounding.md`

## UX brief

### Intent

Make **"which need does this page answer"** something a reader knows *before* they
start reading, an author cannot forget, a lint can read, and a reviewer who did not
write the page can check — without inventing a single bespoke widget, style or
control to do it.

This project ships **no rendered surface of its own** (hence no `deployment` brief,
`.bklg/docs-that-teach/_decomposition.md:318`). Its user-facing surfaces are four,
and every one of them is made of primitives this repository already has:

| # | Surface | Who is at it | What they are trying to do |
| --- | --- | --- | --- |
| S1 | The **declaration** at the top of a governed page | the three reader personas | decide, in seconds, whether this page is the one that answers their question |
| S2 | The **discipline tree** — a router plus rule atoms | the next page author, and the reviewer | find the one rule that governs the thing they are about to write, without reading the corpus |
| S3 | The **lint's terminal output** | the author who just broke the rule | learn what is wrong, where, and how to undo it, in one run |
| S4 | The **reviewer procedure** | a non-author reviewer | reach the same verdict the author would, from the page alone |

The three reader personas are drafts, not promoted `.kb/product/` atoms —
`.kb/product/` currently holds only `README.md`, and
`_discovery/distillation/personas-and-journeys.md:26-35` says why ("a persona nobody
researched is a stock photo with a name" is not met until a non-author reader walks
one). HS-P0025 owns promotion (initiative AC-14, DoD-15). Cite the distillation, not
`.kb/product/`.

Each persona's stake in S1 is different, and the differences are load-bearing:

- **The evaluator** (`personas-and-journeys.md:225-281`) reads for twenty minutes and
  has *no second attempt*: "their entire journey happens inside one reading session,
  with no second attempt if the first one fails silently" (`:305-309`). Their named
  gap is not missing content, it is *"nowhere for that question to go"* (`:241`). S1
  is what turns a list of pages into a set of answers they can triage; DT-3 decides
  whether the routing page that serves them is itself a first-class need.
- **The application author** (`:56-144`) is afraid of *silent wrongness* — "a mental
  model that looks right, compiles, runs, and is quietly wrong" (`:99-106`). For them
  a page that quietly answers two needs is the same defect one level up: it reads as
  complete and is not.
- **The adapter author** (`:148-221`) has the sharpest in-place failure on record: the
  `E0034` explanation exists, but only in three contributor-facing documents, *"none
  of them `crates/happenstance-core/src/store.rs`, the file the reader is looking at
  at the moment they need it"* (`:174-181`). That is this brief's in-place-vs-jump
  invariant stated as a measured defect rather than as a principle.

The perceptual design review is a **skip** — `design:` is deliberately absent from
`.redkiln/config.yaml` (`:75-81`), and nobody will look at a screenshot of this work.
So every invariant below is written to be *checked in text or at a terminal*, not
perceived. An interaction-quality claim nobody can run is exactly the decorative
shape `RUNBOOK.md:920-925` and `:931-935` record this repository paying for twice.

**Authority.** No Accepted decision atom governs documentation trees or narrative
conventions — all seventeen under `.kb/decisions/` were read by title
(`_grounding.md`, "Accepted decision atoms constraining this project"). The binding
authority here is `standards/rust/README.md:23-29`'s precedence chain,
`docs/README.md:25-28`'s pin-by-path convention and `CLAUDE.md`. **No tension with an
Accepted ADR exists and this project needs no ADR.** Two deliberate divergences from
in-repo *precedent* are named in Notes 4 and 6.

### Acceptance Criteria

UX obligations, each traced to a project AC and each stated so it can be run.

- **UX-001 — the need is known before the page is read.** Every governed page's
  declaration is the **first content on the rendered page**, above the first
  paragraph, visible by default, in text. Test: open any governed page, read nothing
  but the region above the first prose paragraph, and name the need. Traces AC-006,
  AC-007; initiative AC-07 ("any reader can tell what a page is for *before* reading
  it" — the word *before* is the requirement, not decoration).
- **UX-002 — the need is a word, never a colour, badge or icon alone.** The
  declaration renders as one token from the closed set DT-2/DT-3 fix, spelled out.
  Colour, weight or an icon may *accompany* it and may never *carry* it. Test: render
  the page in greyscale and read the need; view the raw markdown in a plain-text pager
  and read the same need. Traces AC-004, AC-006; this is the WCAG 1.4.1
  colour-never-alone floor, and the repository already practises it — the constitution
  distinguishes a right example from a wrong one with the literal words **Do** and
  **Not**, not with styling (`standards/rust/README.md:101-107`).
- **UX-003 — the declaration can never be occluded.** The declaration itself, and
  anything DT-8 classifies as load-bearing, may never sit inside a fold, an inactive
  tab, a collapsed accordion or a `<details>`. Test: the DT-8 rule's own checkable
  form, applied to the label — *if the collapsed section were deleted, would the page
  still teach the constraint correctly?*
  (`_discovery/distillation/interaction-patterns.md:404-410`). Traces AC-005, AC-006.
- **UX-004 — the router filters without hiding what it filters.** The discipline's
  router carries **both** an intent-keyed trigger table (the filter) **and** a complete
  index of every rule (the thing filtered), on the same page, the complete index
  present whether or not the filter matched. Test: a rule that no trigger row points at
  is still reachable by reading the router top to bottom. Traces AC-001. This is the
  shape `standards/rust/README.md` already has — `## Start here` at `:45-58` is the
  filter, `## Index` at `:61-97` is the unfiltered whole, and the second is generated
  from the atoms so it cannot fall behind the first.
- **UX-005 — precedence is answered where the reader lands.** The router states the
  discipline's rank *inside* the five-tier chain (`standards/rust/README.md:23-29`) in
  its own text, not by sending the reader to another file to work it out, and does not
  add a tier. Test: `git diff main -- standards/rust/README.md` is empty, and the
  router's own text answers "does this beat a clause?" without a click. Traces AC-002.
- **UX-006 — the rule is reachable in place from the thing it governs, and back.**
  At least one governed page names the discipline as the reason it is shaped as it is,
  as a resolving link *on that page*, and the discipline links to the material it
  governs. Test: from a governed page, reach the governing rule in one navigation
  step, keyboard only. Traces AC-011, DR-10, initiative DoD-14. The failure this
  forbids is the measured one: an explanation that exists somewhere the reader has no
  reason to look (`personas-and-journeys.md:174-181`).
- **UX-007 — one run, every problem, each with a path, a line and a repair.** The
  lint reports **all** violations in a single invocation, never the first; every
  message carries `path:line`, what is wrong, and why it matters or what to do. Test:
  break three pages in three different ways, run the gate once, and count three named
  problems. Traces AC-007, AC-008. The primitives are already written: report-all at
  `xtask/src/lint_constitution.rs:169-198`, the message shape at `:431-441`, `:480-484`
  and `:497-501`, the embedded repair instruction at `:375-379`, the success line at
  `:192`, the vacuity bail at `:176`.
- **UX-008 — every state the reader or author can enter, they can leave.** AC-007's
  observation is run in both directions and both are recorded: the two-need edit and,
  separately, the unenumerated-need edit each make the gate fail by name; each revert
  returns the gate to green with no residue (no cache to clear, no generated file left
  dirty, no manual step). Test: `git checkout --` the edited page, re-run, green,
  `git status` clean. Traces AC-007. The failing half is the one that matters; the
  reversible half is what makes the check safe to use.
- **UX-009 — the reviewer procedure is performable by a stranger, in reading order.**
  The written procedure is executable start to finish from the rendered page plus the
  discipline, with no access to the author, no repository archaeology and no mouse. It
  yields a verdict, not an impression. Test: a person who did not write the page runs
  it over a page carrying two needs and reaches the same verdict as the author would.
  Traces AC-009, DR-07, initiative DoD-8 ("a check a reviewer can actually perform
  rather than one that depends on the author's memory").
- **UX-010 — a normative claim visibly sends the reader to the normative voice.**
  Where a page makes a normative claim, the clause id is the **visible link text**, so
  the reader can see they are being handed to `spec/SPECIFICATION.md` rather than to a
  paraphrase of it. Clause ids are stable names and are never renumbered
  (`spec/SPECIFICATION.md:280`), so the visible token stays true across refactors.
  Test: the spot check walks the set and finds no normative sentence whose authority is
  invisible. Traces AC-010, DR-09, initiative AC-12 / DoD-12. HS-P0020 owns the
  mechanical half — *that the id resolves*; this AC owns *that the reader can see it is
  a citation*.
- **UX-011 — the accessibility floor, stated as a burden of proof.** WCAG 2.2 AA for
  anything this project's rules permit on a rendered page: keyboard reachable, colour
  never alone (UX-002), no author-added motion and `prefers-reduced-motion` honoured by
  anything that animates, and every governed page's content reachable by the browser's
  own find and print. **An unverified property counts as unmet.** Test: for each
  disclosure or tab mechanism the rules permit, a recorded check that its content sits
  in the accessibility tree correctly, is keyboard operable, and is found by Ctrl-F and
  by print. Traces AC-005 and the DT-8 rule. Evidence and rationale in Note 3.
- **UX-012 — a reader loads one rule, not the corpus.** The router lets a reader reach
  a single rule from a stated intent; the discipline carries a stated ceiling on rules
  per atom and bytes per atom, and the ceiling is enforced rather than aspirational.
  Test: name a task, follow one trigger row, read one file, stop. Traces AC-001 and the
  charter's own risk ("the discipline is measured by how many rules it has").
  Precedent: `MAX_RULES_PER_ATOM = 6` (`xtask/src/lint_constitution.rs:88`),
  `MAX_ATOM_BYTES = 16_384` (`:95`), and the message that explains why — *"an agent
  loading this pays for all of it"* (`:503-508`).

### Notes

#### 1. The design system is textual and it already exists — compose it, do not invent one

There is no CSS layer to reach for and none is wanted. The primitive set this project
composes is the constitution's document grammar plus the gate's diagnostic grammar.
Both are enforced, which is what makes them a system rather than a habit.

**Document primitives (`standards/rust/`), with the check that holds each one up:**

| Primitive | Where it is defined | What enforces it |
| --- | --- | --- |
| One-paragraph scope + "load one to three atoms, never the corpus" | `standards/rust/README.md:1-8` | convention |
| Band table — a stable numeric namespace | `README.md:10-21` | the numbering is the filename |
| Precedence block quote | `README.md:23-29` | UX-005; not to be edited |
| Intent-keyed "Start here" table (`You are… / Load`) | `README.md:45-58` | convention |
| Generated index between `<!-- BEGIN GENERATED -->` / `<!-- END GENERATED -->` | `README.md:61-97` | `check_router`, `xtask/src/lint_constitution.rs:375-379` — equality, with `--write` as the repair |
| `# NN — Title` | atom head | `check_shape`, `lint_constitution.rs:480-484` |
| `> **Load when:** …` — *the router's source of truth* | `00-prime-directives.md:1-8` | `check_shape`, `:486-489`; fed to the index by `load_when` (`:247`) |
| `> **See also:** …` | `00-prime-directives.md:5-8` | `check_shape`, `:491-492` |
| `## RS-NN-N. <imperative sentence>` | `00-prime-directives.md:19` | `check_shape`, `:494-495` |
| Five fixed sections — **Why** · **Do** · **Not** · **Rejects** · **Evidence** | `README.md:101-107`; worked at `00-prime-directives.md:30,54,78` | `check_rules` |
| Ceilings on rules and bytes | `lint_constitution.rs:88,95` | `check_shape`, `:497-508` |
| A "what this does not verify" section, stated **first** | `lint_constitution.rs:9-28` | RS-81-1 |

The single most reusable idea in that table is the **generated region**. The router's
index is derived from the atoms and checked for equality, with the repair printed in
the failure message — so a router that has fallen behind its corpus is a gate failure
with a one-command fix, not a stale page nobody notices. If this project's router
carries an index of *pages and their declared needs*, generate it the same way (the
architecture brief takes the same position for the need set at its AC-003/004/005). A
hand-maintained index of who-declares-what is a second copy of the truth, and
`check_summaries` already states the general form of that failure: *"a summary that
does not point at the corpus is a second copy of it, and one of the two will be
stale"* (`lint_constitution.rs:466-470`).

**Diagnostic primitives (`xtask/src/lint_constitution.rs`), which are S3 in full:**

- `{path}:{line} — {what is wrong}; {why it matters, or what to do}` — one line, the
  location first, the consequence attached (`:431-441`, `:480-484`, `:497-501`).
- Report **all** problems, then `bail!("{n} problem(s) in {DIR}")` (`:169-198`). A
  check that stops at the first problem turns one review cycle into six.
- A green run says what it checked: `"  {n} atoms, all consistent"` (`:192`).
- A vacuous run is a **failure**, not a pass: `"{ATOM_DIR} holds no atoms, so every
  check below is vacuous"` (`:176`). Copy this. A page-need lint over an empty or
  moved tree that prints "0 pages, all consistent" is the decorative-gate failure
  (`RUNBOOK.md:920-925`) with this project's name on it.
- The repair goes **in** the message: *"run `cargo xtask lint-constitution --write`"*
  (`:375-379`).

**Rendering primitives — the ecosystem's, not ours.** Whatever surface HS-P0020
selects, the reader-facing affordances are rustdoc's and mdBook's: keyboard-driven
search, `#[doc(alias)]`, intra-doc links, and rustdoc's per-item `[+]`/`[-]` collapse
togglable page-wide with the `+`/`-` keys (`interaction-patterns.md:40-65`); mdBook's
persistent sidebar TOC. The dossier's anti-pattern list is explicit that adding a
bespoke navigation widget on top of these is a mistake, and that the evaluator's real
gap is *"a missing link, not a missing widget"* (`interaction-patterns.md:436-443`,
`:384-391`). **This project's rules must not require an affordance the medium does not
already render.**

#### 2. Interaction-quality invariants, as testable requirements

These are the five the brief is held to. Each is stated with the concrete thing it
forbids in *this* project, because "humane" is not checkable and the forbidden shape
is.

1. **In place, not a context jump.** The reader learns what a page is for on that
   page (UX-001) and reaches its governing rule from that page (UX-006). Forbidden:
   the S1 declaration living in a sidecar manifest, a front-matter block the renderer
   strips, or an index page the reader must already know about. This is the adapter
   author's measured defect generalised — the answer existed, three documents away
   from where they were standing (`personas-and-journeys.md:174-181`).
   *Corollary for the author (S3):* the lint names `path:line` so the fix happens
   where the author already is, rather than sending them to a report.
2. **Non-occlusion — a filter must not hide what it filters.** The router's trigger
   table narrows; it must not be the only way through (UX-004). The complete index
   stays present and complete alongside it, exactly as `## Start here` and `## Index`
   coexist in `standards/rust/README.md:45-97`. Forbidden: a router that lists only
   "common" rules; a need-index that shows only pages currently in violation.
   *Corollary at the terminal:* reporting only the first problem occludes the other
   two — hence UX-007's report-all.
3. **Preserved focus, scroll and selection.** Any disclosure the rules permit must
   leave the reader where they were: toggling `[+]`/`[-]` must not scroll the page or
   drop focus, and the browser's find, print and the page's own search must reach
   content in every state. Forbidden: a mechanism whose content is invisible to
   Ctrl-F. This is not hypothetical — the dossier records that nothing in
   `mdbook-tabs`' own documentation states whether an inactive panel is included in
   `mdbook test`, the page's search index, or Ctrl-F/print, and names it *"an
   unverified property, not a confirmed safe one"* (`interaction-patterns.md:214-216`).
   UX-011 converts that into a rule: unverified is unmet.
4. **Reversibility.** Everything a reader opens, they can close; everything an author
   breaks, they can revert with an instruction they were given at the moment of
   failure (UX-008, and the `--write` precedent at `lint_constitution.rs:375-379`).
   Forbidden: a lint whose only remedy is reading its source; a disclosure that cannot
   be re-collapsed; a check that leaves a generated file dirty after a revert.
5. **Keyboard reachability.** Every affordance the rules permit is operable without a
   pointer (UX-006, UX-009, UX-011). If tabs are permitted at all, the full
   `tablist`/`tab`/`tabpanel` triad plus roving `tabindex` is mandatory — the dossier
   records W3C APG and WebAIM agreeing this is required *and* that it is "the part
   most implementations skip," alongside inactive panels hidden with CSS opacity that
   stay in the accessibility tree and get announced anyway; tabs are called *"one of
   the most-copied and most-broken UI patterns on the web"*
   (`interaction-patterns.md:209-216`). The cheapest way to satisfy this invariant is
   for DT-8 to narrow what is permitted rather than for an implementer to build the
   triad — a design-stage input, not a decision this brief takes.

#### 3. The accessibility floor, and why it is written as a burden of proof

The floor is WCAG 2.2 AA plus three named specifics: colour never alone (UX-002),
reduced motion (no author-added animation; anything that animates honours
`prefers-reduced-motion`), and keyboard reachability (invariant 5). Nothing exotic.

What is unusual is UX-011's second sentence — **an unverified property counts as
unmet** — and it is earned rather than borrowed. The dossier's tabs entry records the
`mdbook-tabs` unknown verbatim, then notes that *"this workspace has already shipped
one gate step that looked wired and wasn't (a documentation step that printed warnings
and exited 0 because nothing read its output)"* (`interaction-patterns.md:220-225`) —
the incident itself at `RUNBOOK.md:931-935`. Treating an unmeasured accessibility
property as satisfied is that same move. So the rule the discipline states is: *a
mechanism whose accessibility or findability behaviour has not been checked in this
repository is not permitted, and checking it is a recorded observation, not an
assurance from its upstream documentation.*

Note the seam: **this project states the rule; HS-P0020 owns the demonstration** that
a hidden branch is inside the checked surface (DT-7, BR-11's demonstration half,
initiative DoD-13). Do not implement a fold-checker here.

#### 4. Divergence from precedent, named rather than defaulted

**`check_fences`' inline opt-out is the weaker precedent, and this project should say
so.** `xtask/src/lint_constitution.rs:601` requires an inline
`<!-- ignore: <reason> -->` comment for any opted-out fence. HS-P0020's architecture
brief has already decided to diverge from it toward an enumerated `const` allowance
list (its Note 5), which exceeds that precedent. This project's declaration mechanism
(DR-05) should follow the *inline* half of the precedent and not the sidecar half: the
need a page declares belongs **in the same document a human reads**, machine-readable
there, because the reader-facing requirement (UX-001) and the lint-facing requirement
are then satisfied by one artefact that cannot drift from itself. Two artefacts — a
rendered label and a separate manifest — reproduce BR-11's already-realised drift
shape (`_grounding.md`, "DT-8 … primary evidence") at the metadata layer.

**Say this in the discipline's own text**, not only here. A contributor who reads an
inline `ignore` comment rule in one tree, an allowance `const` in another and a
declaration in a third needs each to state why it is shaped as it is, or they will
assume two of the three are mistakes.

#### 5. What this brief does not decide, and who does

- **The declaration's literal form** — front matter, a heading convention, an HTML
  comment, a first-line token. That is DR-05, jointly constrained by HS-P0020's
  hosting choice (the charter's first risk row: mdBook has no native per-page front
  matter). This brief fixes the *requirements* the form must meet — UX-001, UX-002,
  UX-003 and Note 4's inline-not-sidecar constraint — precisely so that a change of
  hosting shape visibly invalidates the mechanism without invalidating the
  requirement.
- **The need vocabulary itself** — DT-2 and DT-3, resolved in `_design.md` with the
  rejected options named. This brief requires only that the set be **closed** (UX-002
  cannot check a token against an open set) and that findability's status inside or
  outside it be a stated decision, because an unslotted landing page is otherwise
  flagged as answering a second need by the very rule this project is writing
  (`interaction-patterns.md:568-584`).
- **The fold line** — DT-8. This brief consumes whatever DT-8 decides via UX-003 and
  invariant 5; it does not pre-empt it. The evidence's own warning applies: *"use good
  judgment" is the same non-answer that let the code-layer invariant drift in the first
  place* (`interaction-patterns.md:484-497`).
- **Anything about whether a page teaches.** UX-001 through UX-012 check that a page
  *declares*, and that the declaration is legible, singular and reachable. None checks
  comprehension; that is HS-P0024's friction log and comprehension session. Say so in
  the discipline's own "what this does not verify" section, in the shape
  `lint_constitution.rs:9-28` already uses, first rather than last — because a check
  whose limits are undocumented is read as a guarantee.

#### 6. Deliberate omission — no design tokens, no component inventory, no viewports

A conventional UX brief would name a token layer and a viewport set here. This one
does not, and the omission is a decision rather than a gap: the repository has no CSS
layer, no component library and no `design.capture` (`.redkiln/config.yaml:75-81`),
and the rendered chrome belongs to rustdoc or mdBook — HS-P0020's territory, and the
dossier's own anti-pattern against layering a bespoke widget over a medium that
already renders the function (`interaction-patterns.md:436-443`). The primitive layer
this brief binds the implementer to is the one in Note 1: the constitution's document
grammar and the gate's diagnostic grammar, both enforced by
`xtask/src/lint_constitution.rs` and therefore real. Composing those is mandatory;
hand-rolling a parallel grammar for the same job is the deviation to flag.

#### 7. Anchors

`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:26-35,56-144,99-106,148-221,174-181,225-281,241,285-315` ·
`.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md:40-65,197-225,209-216,214-216,220-225,366-391,397-403,404-410,436-443,484-497,568-584` ·
`.bklg/docs-that-teach/initiative.md:339,344,346,347,380-382,395-400,442-465` ·
`.bklg/docs-that-teach/_decomposition.md:178,183,185,186,208,213,310-331` ·
`.bklg/docs-that-teach/page-need-discipline/project.md` ·
`.bklg/docs-that-teach/page-need-discipline/_grounding.md` ·
`standards/rust/README.md:1-8,10-21,23-29,45-58,61-97,99-112,114-127` ·
`standards/rust/00-prime-directives.md:1-8,19,30,54,78` ·
`standards/rust/70-rustdoc-obligations.md:243` ·
`xtask/src/lint_constitution.rs:9-28,55,58,61,88,95,169-198,247,375-379,423-458,466-470,477-508,601` ·
`docs/README.md:25-28` · `spec/SPECIFICATION.md:280` ·
`.redkiln/config.yaml:40,48,55,67,75-81` ·
`RUNBOOK.md:920-925,931-935` ·
`.kb/product/README.md` · `.kb/playbooks/README.md:7-19,29-31` ·
`.kb/_intake/README.md:7-8,21-25` · `.kb/decisions/README.md:9-13`

## Testing brief

### Intent

This project ships two static instruments, not a service: the rules tree's own
self-consistency (mirroring `check_router` / `check_shape` / `check_harness` at
`xtask/src/lint_constitution.rs:334,477,423`) and the page-need lint that reads a
tree it does not own (HS-P0020's pinned pages tree, architecture brief Note 2).
Neither has runtime behaviour in the ordinary sense, so — exactly as the sibling
`checked-documentation-surface` testing brief states for its own checker — **the
check's own correctness is the product**, and `CLAUDE.md`'s conformance-suite rule
applies here by the same transfer that brief already names: *"Before adding [a
rule], name a plausible wrong implementation it rejects, and write that
implementation into the testkit's own `tests/` if one does not already exist
there."* RS-81-1 states the Rust-specific form: "prove the check's blind spot in
its own tests, then state it in its own documentation"
(`standards/rust/81-checks-that-cannot-be-types.md:11`).

One thing does **not** transfer from HS-P0020's testing brief: there is no
**compile** tier here. Divergence 1 (architecture brief, Note 4) rejects a
`rust`-tagged fence in this tree outright, precisely because nothing registers it
with the doctest harness (`xtask/src/lib.rs:28`, CR-0) — so nothing in this
project's own tree is ever compiled, and no test should assume rustdoc exercises
anything written here. What replaces it is a fifth tier this project's siblings
did not need as prominently: **procedural (ledger-recorded)** — a human-executed
check, captured as cited evidence in `_ledger.md` rather than as a `#[test]`,
because `.redkiln/config.yaml`'s `require_ledger: true` (project.md, Definition of
done) already treats a recorded manual verification as first-class proof, and two
of this project's own AC-### (AC-009, and half of AC-010 and AC-012) are
judgements no function signature can carry.

**The seam with HS-P0020 constrains when some tests can run for real.** The
page-parsing function's *unit* tests never need real content — they run over
synthetic markdown strings from day one (AC-006, AC-008 below). But AC-006's and
AC-011's *gate-integration* halves ("every governed page…", "at least one page
links back…") can only be observed once real pages exist, which is why this
project is first in merge order and those two entries are marked accordingly:
proven at this project's own merge with whatever pages exist then, and
re-proven — not re-designed — as HS-P0022/23 add more.

### Acceptance Criteria

Test mix, mapped one project AC at a time. Tiers: **static** (a `#[cfg(test)]`
unit test, house style at `xtask/src/lint_constitution.rs:828-878`),
**gate-integration** (`cargo xtask ci`/`affected`/`lints`, run and observed),
**end-to-end/fixture** (a page deliberately broken and walked through the real
gate, then reverted), **procedural** (a human runs a written step; the ledger
records what happened, not a green CLI run standing in for it).

- **AC-001 (decided home, on disk).** *Static:* two tests on the checker's own
  directory-reading function, exercising real (not synthetic-string) filesystem
  state — this project's `atoms()`-equivalent takes `root: &Path` exactly as
  `lint_constitution.rs:208` does, so a test can point it at a fabricated root.
  **No existing precedent does this**: `lint_constitution.rs`'s own `mod tests`
  (`:828-878`) exercises only pure string functions, never the real
  directory-read path, so this is new test infrastructure, not a copy. Build the
  fabricated root with `std::env::temp_dir()` plus manual `fs::create_dir_all` /
  `fs::write` — **no `tempfile` dependency exists in `xtask/Cargo.toml`
  (confirmed: only `anyhow` under `[dependencies]`) and none should be added for
  this alone.** Test (a): a missing directory produces a `.with_context()` error
  naming the pinned path (mirrors the context shape at `:212`); test (b): an
  existing-but-empty directory `bail!`s with the vacuity message, not a silent
  "0 rules, all consistent" (mirrors `:176`'s wording pattern exactly — a
  checker that prints success over an empty tree is the named wrong
  implementation `RUNBOOK.md:920-925` already cost this repository once). The
  same pair, against the **pages**-tree guard (the shared `pub(crate)` constant
  from architecture brief Note 2), makes four tests, not two — a checker that
  guards only its own tree and trusts the pages tree unconditionally is a
  plausible wrong implementation the architecture brief's AC-001 names directly.
  *Gate-integration:* the CR-4 landmine (architecture brief Note 1) needs two
  more tests in `xtask/src/affected.rs`'s own `mod tests`, mirroring
  `the_relocated_trees_stay_inert` (`affected.rs:662-673`) and
  `a_constitution_atom_selects_xtask` (`:688-696`): one asserting the new tree's
  prefix selects no package, one asserting `standards/rust/` still selects
  `xtask` after the new `INERT` entry is added (guards against a lazily
  broadened `"standards/"` prefix un-compiling the constitution — the exact
  failure `an_unrecognised_path_widens_rather_than_narrows`, `:645-648`, exists
  to catch one level up). *Procedural:* `_design.md` naming the rejected homes
  and their cost is read at authoring time and cited in the ledger; it is prose
  judgement, not a function under test.
- **AC-002 (inside the chain, without extending it).** *Procedural/gate-state:*
  `git diff main -- standards/rust/README.md` is empty — a one-line repo-state
  check, run once and its (empty) output pasted into the ledger, the same shape
  UX-005 already specifies as its own test. This is not a `#[test]`: nothing in
  this project's own diff can be trusted to prove a *different* file was never
  touched except inspecting the diff directly. *Static, secondary:* a
  `check_shape`-style test asserts the router's own text contains a stated
  precedence sentence — structural presence only, not a judgement on which rank
  is correct (that is `_design.md`'s call per architecture brief AC-002).
- **AC-003 / AC-004 / AC-005 (DT-2, DT-3, DT-8 resolved; the `NEEDS` const has
  exactly one generated router region).** *Procedural:* the resolutions
  themselves, with rejected options named, are `_design.md`'s prose and are
  cited in the ledger, not code-tested — matching HS-P0020's own testing brief's
  stance on its sibling `_design.md`-owned decisions. *Static:* the
  generated-region equality check (`check_router`'s shape,
  `lint_constitution.rs:334-384` — itself **not** covered by that file's own
  `mod tests` today, so again new infrastructure, not a copy) gets a unit test
  feeding a synthetic `NEEDS`-const/router-table pair that disagrees by exactly
  one member, asserting the check fails **and names which member moved**
  (RS-81-5, `81-checks-that-cannot-be-types.md:335`: "make the failure say which
  one moved," the same bar the sibling brief's AC-008 already applies to
  `spec_trace`). A second test asserts `Mode::Write` regenerates the region to
  match. Named wrong implementation: a checker that asserts the two are
  *non-empty* rather than *equal* — it passes silently the day a member is
  added to the `const` and not the table, which is the exact "half-lands"
  defect `xtask/src/spec_trace.rs:122-160` names for clause families and the
  architecture brief's AC-003 imports directly.
- **AC-006 (every governed page declares exactly one need).** *Static:* the
  pure parse function (Rule/Fence-shaped per architecture brief AC-006) is
  tested over synthetic `&str` literals — zero declarations, one, two, one
  recognised declaration plus prose that merely *mentions* a need word
  elsewhere in the body (must not false-positive to two) — mirroring
  `rules_are_split_at_the_next_heading`'s shape (`lint_constitution.rs:871-877`)
  exactly: construct a string, call the pure function, assert on the returned
  value and its line number. No file I/O in this tier. *Gate-integration:*
  once HS-P0020's pages tree carries real content, the full check run over it
  reports zero violations — this is project.md's own AC-006 language ("walking
  the narrative tree as it stands at this project's merge") and is necessarily
  late-binding: record it as ledger evidence at this project's own merge, not
  as a `#[test]` that could pass on an empty tree. If the pages tree is empty
  at that point, the run must **fail** on AC-001's vacuity guard rather than
  report "0 pages, all consistent" — the UX brief's own framing of that exact
  failure shape (`_decomposition.md`, UX brief Note 1: *"0 pages, all
  consistent" is the decorative-gate failure with this project's name on it*).
- **AC-007 (the check has been seen to fail).** *End-to-end/fixture* — the one
  AC in this project proven by a recorded procedure, never a `#[test]`, for
  the identical reason HS-P0020's own AC-003 is: a unit test that calls the
  checker function directly never invokes `cargo xtask ci`, and DoD's own line
  is explicit that this observation belongs in the ledger, "not merely
  asserted." Concretely: (1) edit a governed page (a fixture page this project
  owns if none exist yet in the implementation window, or a real page once
  HS-P0020/22/23 have merged) to declare two needs; run `cargo xtask ci` (or
  the named step alone while iterating) and capture that the failure names the
  file and the line; (2) separately, edit a page to declare a need outside the
  enumerated set and capture the same; (3) revert both edits, re-run, capture
  green and a clean `git status` (UX-008's reversibility test, same
  procedure). All three captured outputs — two failing, one recovered — go
  into the ledger verbatim, not paraphrased, per `RUNBOOK.md:920-925`'s own
  lesson that a document merely *vouching* for a check is not evidence the
  check runs.
- **AC-008 (a named wrong page in the checker's own tests).** *Static only* —
  explicitly **not** a committed broken file. Architecture brief AC-008 is
  itself the test specification: three named wrong pages, each an in-memory
  `&str` literal inside a `#[cfg(test)] mod`, mirroring `affected.rs`'s own
  convention of driving `affected_packages` over synthetic path lists rather
  than real files (`affected.rs:640-705`) — two declarations, zero
  declarations, one unenumerated declaration — each asserted to produce a
  problem naming the correct line number *within the literal*. Because these
  are permanent regression tests rather than the AC-007 one-off observation,
  they are what lets AC-008 stay checked on every future `cargo test -p
  xtask` run without anyone re-breaking a real page.
- **AC-009 (reviewer procedure, non-author-performable).** *Procedural
  (ledger-recorded)*, and not code-testable — this is exactly the judgement
  the architecture brief's own scope boundary excludes from the checker ("the
  page *answers* only that need," not merely declares it). Concretely: a
  person who did not author a given page executes the written procedure
  (DR-07) against it, reaches a verdict, and both the identity of the walker
  (not the author) and the verdict are recorded in the ledger; project.md's
  own AC-009 additionally requires a walk of the **full** set finding no page
  carrying two needs, which is itself a procedural sweep, not a batch of unit
  tests. No `#[test]` substitutes for this, and the checker's own module docs
  must say so first (RS-81-1; architecture brief AC-009/Note 7 item 1).
- **AC-010 (citation rule, spot check runs).** *Static:* a structural-presence
  test — the discipline's own rule text states "cite, never restate" —
  mirroring `check_shape`'s section-presence assertions rather than any
  content judgement. The mechanical half (does a cited id actually resolve)
  is explicitly **not** retested here: it is HS-P0020's `clause_ids` function,
  a sibling of `spec_trace::all_rules` (`xtask/src/spec_trace.rs:1746`,
  confirmed at that line), and architecture brief AC-010 forbids building a
  second parser — so, exactly as HS-P0020's own testing brief states for its
  analogous case, "no new parser tests are owed here." *Procedural:* the
  paraphrase spot check — does a page that cites a real, resolving clause id
  nonetheless restate its content — is a reviewer walk, ledger-recorded, the
  same category as AC-009: architecture brief Note 7 item 4 states plainly
  "no byte count sees it."
- **AC-011 (cited by what it governs, reachable).** *Static:* the
  link-resolution half of `check_router` (`lint_constitution.rs:344-356`,
  "every `.md` a router links must resolve") is reused verbatim per
  architecture brief Divergence 2 — a unit test with a synthetic router
  string containing one dangling link asserts a problem is reported. A second
  static test asserts `docs/README.md`'s table (`:12-23`) carries a row for
  the new tree **and** that the paragraph at `:25-28` — "Two of those are read
  by the gate rather than only by people… Moving either tree means editing
  `xtask/src/` in the same change" — names three trees, not two, once this
  project lands (confirmed today: that paragraph names exactly `spec/*` and
  `standards/rust/`, so the count itself is a checkable regression signal — a
  string-contains/count assertion, not prose judgement). *Gate-integration,
  cross-project, deferred:* "at least one page in HS-P0020's tree links this
  discipline as the reason it is shaped as it is" (project.md AC-011, DoD-14)
  can only be checked once that project's pages exist. Record it as a ledger
  cross-check at whichever of the two projects merges second — and do **not**
  build a shared scanner that ranges over both trees to automate it: that is
  precisely the "shared abstraction over two trees" RS-81-3 forbids
  (architecture brief Note 6, `81-checks-that-cannot-be-types.md:209`), so this
  half stays a one-time procedural confirmation, not a permanent check.
- **AC-012 (staged, never hand-authored; valid frontmatter).** *Static/gate-
  state:* `git diff main -- .kb ':!.kb/_intake'` is empty — the mechanical
  negative half, same one-line shape as AC-002's, proving nothing landed
  outside the intake path. This is what `redkiln validate --kb` green
  actually proves too, and no more: `.kb/_intake/README.md:21-25` states the
  validator skips any `_`-prefixed directory by design, so a green run is
  silent about the staged file's own frontmatter. *Procedural, and this is the
  criterion's real bar:* a manual field-by-field check of the staged atom
  against `.kb/README.md:18-26`'s table — `id`, `title`, `kind: playbook`,
  `status`, `authority_tier: guideline`, `summary`, `depends_on`/`related`,
  `source_paths`, `last_reviewed` — recorded in the ledger. This is
  `_grounding.md`'s tension 2 stated as a test requirement rather than left
  implicit: the Definition of Done's `redkiln validate --kb` line must not be
  read as discharging AC-012's frontmatter half, and this brief's job is to
  make sure the ledger entry does not quietly cite that green run as if it
  did.

### Notes

**Merge-gate commands**, narrowest to widest. `<step-name>` stands for whatever
name is chosen for the new bin-crate module per architecture brief Note 9 — it
is depended on by value in three places (`main.rs`'s dispatch, `lint_steps()`,
`steps_named`'s panic), so once chosen it is not a free-text argument anywhere
in this list.

```console
cargo test -p xtask                          # this project's own #[cfg(test)] unit tests: AC-001, 003-006, 008, 010, 011
cargo run --locked -p xtask -- <step-name>   # the checker alone, while iterating
cargo xtask lints                            # confirms the file-reading lint family reaches the new step (CR-3, main.rs:799-808)
cargo xtask affected --base main             # AC-001's CR-4 landmine check; the story-grain path (verify.affected_gate)
git diff main -- standards/rust/README.md    # AC-002's repo-state check
git diff main -- .kb ':!.kb/_intake'         # AC-012's negative half
cargo xtask ci --fast                        # the bar for a non-terminal project (project.md DoD, verify.integration_scoped)
cargo xtask ci                                # the full gate; run before the project is called done
redkiln validate --kb && redkiln doctor      # AC-012's mechanical half only; six template-drift advisories, not more or fewer
```

`cargo xtask ci` is the merge gate of record (`CLAUDE.md`, "Commands"); `--fast`
is the interim bar this non-terminal project is held to during implementation
(`.redkiln/config.yaml`'s `verify.integration_scoped`, project.md DoD).

**Fixtures and seams to mock — almost none, and for a different reason than
HS-P0020's near-none.** That project's checks compile real code, so nothing
about a fence's compilation may be faked. This project's checks read prose and
judge structure, so the seams are narrower still:

- Every AC-006/AC-008 unit test runs over an in-memory `&str`, never a file —
  the parse function is pure (architecture brief AC-006), so there is nothing
  to mock and nothing to fake; a synthetic string *is* the fixture.
- AC-001's four directory-guard tests are the one place this project touches a
  real filesystem in a unit test, and they touch a **fabricated** root under
  `std::env::temp_dir()`, never the workspace's real `standards/rust` or the
  real pages tree — no dependency addition, per the note under AC-001.
- AC-003/004/005's generated-region test uses a synthetic `NEEDS` const and a
  synthetic router-table string, for the same reason `spec_trace.rs`'s own
  clause-parsing tests do not read the real 200-clause specification for every
  case: a two-or-three-member disagreement is easier to construct and to read
  than a diff against the real corpus.
- AC-010 deliberately mocks nothing new: it calls HS-P0020's real `clause_ids`
  function (or small in-memory string slices passed to it, matching that
  project's own testing brief's stance) rather than standing up a parallel
  fixture specification file — duplicating that coverage here is the same
  "shared abstraction over two trees" RS-81-3 warns against, one layer up.
- AC-007's fixture pages, if this project must author its own ahead of
  HS-P0020/22/23 landing content, are throwaway or permanently-retained *test*
  material analogous to `happenstance-testkit`'s `MemoryFixture` — not
  narrative content this project is responsible for authoring (project.md's
  own scope boundary: this project's tree is the *rules*, not the *pages*).

**What this brief does not cover.** The *content* of DT-2/DT-3/DT-8's
resolutions is `_design.md`'s job, cited here but not re-argued or re-tested;
this brief tests only that the resolution, once written, cannot silently
drift from the `NEEDS` const it must equal (AC-003/004/005). Whether a page
that declares one need actually *teaches* it is outside every tier above —
that is HS-P0024's friction log and comprehension session, and the checker's
own "what this does not verify" section must say so first, per RS-81-1 and
architecture brief Note 7 item 1. The mechanical clause-id resolution suite
(does an id exist in `spec/SPECIFICATION.md`) is HS-P0020's testing brief's
territory, reused and not duplicated here (AC-010). The demonstration that a
hidden branch sits inside the checked surface at all (DT-7) is HS-P0020's
testing brief's AC-006, not this project's — this brief states only what may
never be folded (AC-005/UX-003), never whether a fold is itself checked.
