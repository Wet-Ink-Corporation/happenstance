---
title: "Integration — The Checked Documentation Surface"
initiative_slug: docs-that-teach
project_slug: checked-documentation-surface
terminal: false
dod_green: true
reachability_ok: true
deferred_scenarios:
  - "DoD-1 @smoke — the teaching survives a clean checkout (fresh clone, full gate, narrative builds and renders inside it)"
  - "DoD-2 @smoke — a deliberately broken page fails the gate, by name (re-observed on the assembled tree)"
  - "DoD-3 @smoke — the opening encounter runs and demonstrates a boundary"
  - "DoD-4 @smoke — the boundary claim is checked, not narrated"
  - "DoD-5 — a non-author, non-insider reader completes a stated scenario and it is recorded"
  - "DoD-6 — every stumble in that log has a disposition"
  - "DoD-7 @smoke — the reader reaches the teaching from the front door"
  - "DoD-8 — every page's answered need is stated and singular"
  - "DoD-9 — the evaluator's second question is walked"
  - "DoD-10 — the adapter author's error meets its explanation"
  - "DoD-11 — the frozen documentation MUSTs are still discharged (re-checked after every doc comment the initiative touches)"
  - "DoD-12 — no page has become a second specification"
  - "DoD-13 — nothing load-bearing is hidden from the check (re-observed on the assembled tree)"
  - "DoD-14 — the discipline is on disk and cited"
  - "DoD-15 — the audience is durable and reconciled"
---

# Integration — The Checked Documentation Surface

`HS-P0020`, the **non-terminal** first project of `docs-that-teach`. This audit
proves *this project's* integration bar and nothing wider: its capabilities are
mounted and reachable in the running gate, and its own gate is green. The
fifteen whole-initiative Definition-of-Done journeys belong to the terminal
project `HS-P0025 durable-audience-closeout` (`project.md:132`) and are listed
as deferred below, not counted here.

Every command in this record was executed in this worktree at `99bbdf2`, on a
tree `git status --porcelain` reports clean. Nothing below is quoted from a
story's evidence file except where the row says so.

## Project integration bar

### The gate commands `.redkiln/config.yaml` wires

| Bar | Command | Ran? | Result |
| --- | --- | --- | --- |
| `integration_scoped` — the non-terminal bar | `cargo xtask ci --fast` | executed | **pass**, exit 0 — `all required checks passed (--fast: 4 optional step(s) not run)` |
| the full gate, project DoD item 6 (`project.md:256-258`) | `cargo xtask ci` | executed | **pass**, exit 0 — `all checks passed`, 26 banners, **0** lines matching `^skipped` |
| `affected_gate` — the story grain | `cargo xtask affected --base main` | executed | **pass**, exit 0 — `affected gate passed`; 373 file(s) changed, one package selected: `xtask` |
| `reachability_static` — the cheap tripwire | `cargo xtask lints` | executed | **pass**, exit 0 — seven banners including `=== every narrative page is checked ===` / `2 pages, all consistent` |
| `reachability_static`, second half | `cargo xtask spec-trace` | executed | **pass**, exit 0 — 200 clauses, 95 rules, 58 e2e cases, **358 citations checked**; `traceability: no problems found` |

Both narrative steps printed under their own banners in the full run:
`=== the narrative tree's examples compile ===` gave `1 page(s)' examples
enumerated` and `1 passed; 0 failed`; `=== every narrative page is checked ===`
gave `2 pages, all consistent`.

### The project's own acceptance criteria (`project.md:196-233`)

Each row is a scenario **this project owns**. Rows marked *falsified live* were
broken on purpose in this worktree during this audit, observed failing, and
reverted — `git diff --stat HEAD` is empty afterwards.

| AC | Scenario | Ran? | Result |
| --- | --- | --- | --- |
| AC-001 | The tree is pinned, not conventional — moving it without editing `xtask/src/` fails, naming the expected path | executed, **falsified live** | **pass**. `git mv docs docs-moved` then `cargo xtask narrative` gave exit 1, `xtask failed: reading docs: The system cannot find the path specified. (os error 3)` — naming the pinned constant `TREE` (`xtask/src/lint_narrative.rs:231`). Restored by `git mv docs-moved docs`. |
| AC-002 | Every fence compiled against the real crates, mandatorily (`REQUIRED`, `probe: None`) | executed | **pass**. `cargo xtask narrative-doctests` gave exit 0, `1 page(s)' examples enumerated`, and `narrative::append_conditions (line 9) ... ok`. The step is `REQUIRED` (`xtask/src/main.rs:497-510`, inside the array at `:107`) with `probe: None` (`:509`). The observed-fail half — renaming `store.len()` to `store.length()` — is recorded at `pinned-narrative-tree-and-compiling-step/_ledger.md` AC-002. |
| AC-003 | The check has been **seen to fail** on a deliberately broken claim, then reverted green | executed at `6368e2b` by `observed-failure-falsification` | **pass**. `_falsification.md` run 2: `cargo xtask ci` exit **1**, an assertion panic reading `left: 0 / right: 1`, naming `docs/append-conditions.md - narrative::append_conditions (line 9)`. Run 3 after revert: exit **0**, `all checks passed`, 26 banners, 0 skipped. Both halves written down; the failing half is an *assertion panic*, which is what establishes the fences are run and not merely compiled. |
| AC-004 | No silent opt-out — an `ignore`-class fence fails with file and line unless enumerated | executed, **falsified live** | **pass**. A probe page carrying a `rust,ignore` fence produced `docs/zz-probe.md:5 — an 'ignore' fence needs an 'IGNORE_ALLOWANCES' entry naming it; a comment above the fence does not permit it, because a comment is reviewable only in the diff that introduced it`. `IGNORE_ALLOWANCES` ships empty (`xtask/src/lint_narrative.rs:296`). |
| AC-005 | No orphan pages — a page in the tree the mechanism cannot reach fails | executed, **falsified live** | **pass**. The same probe page produced **two** distinct problems: `xtask/src/narrative.rs — does not include zz-probe.md; its examples are never compiled` and `xtask/src/narrative.rs — no 'mod zz_probe'; one module per page is what keeps a doctest failure's line number relative to the page`. |
| AC-006 | Hidden content is inside the check, or absent; DT-7 resolved in `_design.md` | executed, **falsified live** | **pass**, on the *absent* branch, mechanically enforced. `_design.md:204-228` D2 resolves DT-7 — always-visible by default, a page per scope past the threshold, panels **mechanically forbidden** — signed off with no conditions (`_design.md:786`). A probe page with a details/summary pair produced `docs/zz-hidden.md:3 — '<details' is a hidden panel; DT-7 forbids it in docs` and the same for the summary tag at `:4`. There is deliberately no allowance list (`xtask/src/lint_narrative.rs:306-315`). |
| AC-007 | Citations resolve — a dangling clause id fails; a real one passes | executed, **falsified live** | **pass**. The probe page's `ES-9999` produced `docs/zz-probe.md:3 — cites 'ES-9999', which SPECIFICATION.md does not define`. The positive half stands green in the run: `docs/append-conditions.md:7` cites `ES-40` and the checker reports `2 pages, all consistent`. |
| AC-008 | The frozen documentation MUSTs are pinned in exactly one place, and `spec-trace` passes over the tree | executed | **pass**. One enumeration: `FROZEN_DOC_MUSTS` at `xtask/src/lint_narrative.rs:1346`, every entry `Pinned` with site and verbatim anchor or `Excluded` with a reason; guarded by `guard_pin` (`:1630`) and checked by `check_pin` (`:1735`), both on `check`'s path (`:1822`, `:1832`). `cargo xtask spec-trace` gave exit 0 and `traceability: no problems found`. The pin is in place **before** any project rewrites a `happenstance-core` doc comment — `HS-P0023` sits behind this one. The re-derivation closing the nine-versus-eight arithmetic is at `frozen-documentation-must-pin/_rederivation.md`. |
| AC-009 | Clean checkout, no manual step; hosting/render shape recorded | executed **on this checkout**, not on a fresh clone | **pass at this project's grain**. `cargo xtask ci` gave exit 0 with both narrative banners inside it and no manual step. `_design.md:169-179` D1 records the hosting/render decision — *the markdown is the render*, no mdBook, no site, no build step — with the two rejected alternatives and why each lost. The **fresh-clone** half is DoD-1 and is deferred to the terminal project. |
| AC-010 | The limits are on the record, naming the compiles-but-no-longer-demonstrates blind spot and the `RUSTDOCFLAGS` gap | executed | **pass**. `xtask/src/narrative.rs:3-88` opens with `# What this does not verify` and states **seven** limits, including limit 1 (an example that no longer demonstrates its claim) and limit 3 (`RUSTDOCFLAGS=-D warnings` reaches nothing inside a narrative fence) — **measured, not cited**: three transcripts on the pinned 1.97.1 toolchain in `documented-blind-spots-and-their-proofs/_limits-evidence.md:35-215`. `xtask/src/lint_narrative.rs` states limit 5, the one that holds in the fence walk. The set is pinned by `NOTE_TEN` (`xtask/src/lint_narrative.rs:4301`) with owner, claim and instrument per limit, additive-only, run by `cargo test -p xtask` inside the gate. |

### The project's boundary Definition of Done (`project.md:240-260`)

| # | Item | Ran? | Result |
| --- | --- | --- | --- |
| 1 | `cargo xtask ci` green with the narrative build and render inside it as an ordinary step | executed | **pass** — exit 0, both narrative banners printed |
| 2 | The falsification run, observed failing, reverted, observed green, both halves written down | executed | **pass** — `_falsification.md` runs 1/2/3 |
| 3 | Hidden-branch falsification, or the design record states no hidden content carries a load-bearing claim | executed | **pass** — `_design.md` D2 states it *and* `HIDDEN_MARKERS` enforces it; falsified live above |
| 4 | `_design.md` records DT-7's resolution and the hosting/render shape, with the alternatives that lost | executed | **pass** — D1 (`:169-179`) and D2 (`:204-228`), signed off at `:786` |
| 5 | Frozen documentation MUSTs enumerated by clause id in one place; `spec-trace` passes | executed | **pass** — see AC-008 |
| 6 | `cargo xtask ci --fast` is the non-terminal bar; the full gate run before the project is called done | executed | **pass** — both run, both exit 0 |
| 7 | Every new check carries a "what this does not verify" section; `lint-constitution` and `cargo test -p xtask --doc` still pass | executed | **pass** — `=== the Rust constitution is internally consistent ===` gave `27 atoms, all consistent`; `=== the constitution's examples compile ===` ran 168 + 63 doctests, 0 failed |
| 8 | Nothing in this project asserts that the surface proves a page teaches | executed | **pass** — limit 6 is stated unhedged in **both** modules and held by `NOTE_TEN`'s hedge scan (`xtask/src/lint_narrative.rs:4346-4347`, `:4458-4480`); `tests::nothing_in_the_module_claims_a_page_teaches` (`:2377`) |

## Deferred to terminal project

The fifteen whole-initiative Definition-of-Done journeys
(`.bklg/docs-that-teach/initiative.md:406-468`) are owned by `HS-P0025
durable-audience-closeout`, which the out-of-scope table assigns "re-observing
all fifteen DoD scenarios on the assembled tree" (`project.md:132`). None of
them can pass here: DoD-3, 4, 7, 8, 9, 10, 12 and 14 require teaching content
this project is **forbidden** to write (`project.md:294`), DoD-5 and 6 require
`HS-P0024`'s friction log, and DoD-15 requires `HS-P0025`'s persona atoms.
Running them now would fail by design.

| # | Journey | Owner | Precursor discharged here |
| --- | --- | --- | --- |
| 1 | @smoke — the teaching survives a clean checkout | HS-P0025 | the gate half: `cargo xtask ci` exit 0 with both narrative steps inside it. The **fresh-clone** half is not this project's. |
| 2 | @smoke — a deliberately broken page fails the gate, by name | HS-P0025 | executed here at `6368e2b` on the fixture page (`_falsification.md`); re-observed on the assembled tree by the terminal project |
| 3 | @smoke — the opening encounter runs and demonstrates a boundary | HS-P0022, then HS-P0025 | none — content this project may not write |
| 4 | @smoke — the boundary claim is checked, not narrated | HS-P0022, then HS-P0025 | the *machine*: any `rust` fence in `docs/` is compiled **and run** (`_falsification.md` P2). The claim it must check does not exist yet. |
| 5 | a non-author, non-insider reader completes a stated scenario | HS-P0024, then HS-P0025 | none — and deliberately so; a green gate here is a precondition, never a substitute (`project.md:60-66`) |
| 6 | every stumble in that log has a disposition | HS-P0024, then HS-P0025 | none |
| 7 | @smoke — the reader reaches the teaching from the front door | HS-P0023, then HS-P0025 | none |
| 8 | every page's answered need is stated and singular | HS-P0021, then HS-P0025 | the reserved slot only: an empty `answered-need` comment on both pages |
| 9 | the evaluator's second question is walked | HS-P0023, then HS-P0025 | none |
| 10 | the adapter author's error meets its explanation | HS-P0023, then HS-P0025 | none |
| 11 | the frozen documentation MUSTs are still discharged | HS-P0025 | the pin and its resolution, executed: `FROZEN_DOC_MUSTS` plus `spec-trace` green. The re-check *after every doc comment the initiative touches* is the terminal project's. |
| 12 | no page has become a second specification | HS-P0021, then HS-P0025 | the machine half: every clause id a page cites resolves (AC-007, falsified live). The *defers rather than restates* half is HS-P0021's reviewer check. |
| 13 | nothing load-bearing is hidden from the check | HS-P0025 | executed here on the whole tree as it stands: `HIDDEN_MARKERS` rejects every panel token by file and line, falsified live |
| 14 | the discipline is on disk and cited | HS-P0021, then HS-P0025 | none |
| 15 | the audience is durable and reconciled | HS-P0025 | none |

## Reachability map

Every capability this project delivered, traced to the composition root that
runs it — `REQUIRED` at `xtask/src/main.rs:107`, dispatch at `:742` and `:746`,
`lint_steps()` at `:874-884`, and `xtask/src/affected.rs:112`. Nothing below is
reachable only through a test.

| Capability | Mount point | Reachable? |
| --- | --- | --- |
| The narrative tree, pinned by path | `xtask/src/lint_narrative.rs:231` `TREE = "docs"`, read by `check` (`:1816`), called by `run` (`:1803`), dispatched at `main.rs:746`, wired as a `REQUIRED` step at `main.rs:540-553` | **yes** — proved by moving the tree and reading the error |
| The compiled-prose harness, one `#[cfg(doctest)] mod` per page | `xtask/src/narrative.rs:123` `mod append_conditions`, `:133` `mod text_fences`, declared by `mod narrative;` at `xtask/src/lib.rs:36` — the **lib** target, beside `mod constitution;` (`:28`) | **yes** — `cargo xtask narrative-doctests` listed and ran `narrative::append_conditions` |
| The compiling step, mandatory | `xtask/src/main.rs:497-510`, in `REQUIRED`, `probe: None` (`:509`), `RUSTDOCFLAGS=-D warnings` (`:508`) | **yes** — banner printed in the full run |
| The page-checker step, mandatory | `xtask/src/main.rs:540-553`, in `REQUIRED`, `probe: None` (`:552`); also in `lint_steps()` (`:882`) | **yes** — banner printed in the full run and by `cargo xtask lints` |
| Subcommand surface | `xtask/src/main.rs:742` `narrative-doctests`, `:746` `narrative`; help at `:829-837` | **yes** — both invoked directly this session |
| Story-grain selection of a prose-only change | `xtask/src/affected.rs:223-251` (the `docs/` arm selects `xtask`) and its removal from `is_inert` (`:283-289`) | **yes** — `cargo xtask affected --base main` selected `xtask`; `tests::the_narrative_tree_is_no_longer_inert` (`:771`) guards the shadowed-prefix failure mode |
| The checker on the affected gate's unconditional list | `xtask/src/affected.rs:131` `crate::lint_narrative::run()?` | **yes** — `2 pages, all consistent` printed under `=== the file-reading checks ===` |
| Fence discipline and the enumerated allowance list | `check_fences` (`lint_narrative.rs:762`), `check_allowances` (`:936`), `IGNORE_ALLOWANCES` (`:296`, empty), reached from `problems` (`:1772`) and `check` (`:1816`) | **yes** — falsified live |
| Hidden-content rejection (DT-7) | `HIDDEN_MARKERS` (`:327`), `check_hidden_markers` (`:997`), reached from `check_page` (`:1239`) | **yes** — falsified live |
| Registration and orphan detection | `check_registration` (`:528`), reading `HARNESS` (`:249`) as text | **yes** — falsified live, two distinct problems |
| Clause-id accessor | `xtask/src/spec_trace.rs:1803` `pub(crate) fn clause_ids`, consumed once per run at `lint_narrative.rs:1821` | **yes** — consumed, not dead: the gate is `-D warnings`, so a re-added `#[expect(dead_code)]` would fail the build |
| Citation resolution | `clause_citations` (`:1084`), `check_citations` (`:1189`), reached from `check_page` (`:1239`) | **yes** — falsified live |
| The frozen-documentation MUST pin | `FROZEN_DOC_MUSTS` (`:1346`), `guard_pin` (`:1630`), `check_pin` (`:1735`), all on `check`'s path (`:1822`, `:1832`) | **yes** — on the run path, green |
| The stated limits ("what this does not verify") | `xtask/src/narrative.rs:3-88` (limits 1-4, 6, 7) and `xtask/src/lint_narrative.rs` module docs (limits 5, 6); pinned by `NOTE_TEN` (`:4301`) | **yes** — production module docs, held by tests the gate runs |
| The `text`-fence fixture behind limit 5 | `docs/text-fences.md`, registered at `xtask/src/narrative.rs:133-135` | **yes** — walked on every run; produces no doctest, which is the point |
| The tree index, deliberately unregistered | `docs/README.md`; exempt from registration only (`lint_narrative.rs:242`), and a Rust-class fence on it is refused outright | **yes** — walked and scanned on every run |
| The falsification observation | `observed-failure-falsification/_falsification.md`; its finding **F2** landed in code as limit 7 (`xtask/src/narrative.rs`) and as `NOTE_TEN`'s seventh entry | **yes** — the observation is wired back into an enforced limit, not left as prose |

Nothing was found constructed-but-unmounted, exported-but-unconsumed, or
reachable only through a test.

## Missing dependencies

**None.** Every dependency this project's own scenarios need exists and was
exercised. `HS-P0020` has no inbound edge (`project.md:264-265`), and the
dependencies the deferred journeys need — teaching content, the friction log,
the persona atoms — are owed by later projects rather than by this one.

## Notes carried forward, not blocking

**FU-1's two stale comments.** `project.md:354-395` opens `FU-1` for the gate's
step-ordering defect that `_falsification.md` **F2** measured: a broken
narrative fence fails under `=== tests ===` at `REQUIRED` index 2, so neither of
the tree's own banners prints. The *limit* is discharged — it is limit 7 in
`xtask/src/narrative.rs`, enumerated in `NOTE_TEN` and held by two tests. What
is not repaired is prose: `xtask/src/main.rs:486-487` still reads "the ordering
is the whole of what keeps a broken narrative fence attributed to the narrative
corpus", and `:536-538` "their order is the whole of what keeps a broken
narrative fence under the narrative banner". `431c8b0` corrected the same
sentence in `xtask/src/narrative_doctests.rs` and deliberately did not touch
`main.rs`, whose step definitions sit outside every story's PR boundary.

Not blocking: no behaviour depends on either comment, and the corrected
statement is the one a reader is sent to. Recorded here because `FU-1`'s owner
line reads **unassigned**, which is the shape `project.md:346-348` itself warns
about — a finding whose routing names a direction rather than an addressee is a
finding nobody has. It needs an owner before the initiative closes.

## Integration verdict

**dod-green** — for this non-terminal project's bar, which is the only bar it
owns.

All ten project acceptance criteria and all eight boundary Definition-of-Done
items were **executed** and **passed**; none is skipped, `fixme`d or
flag-gated. Five of them — AC-001, AC-004, AC-005, AC-006 and AC-007 — were
adversarially falsified *live in this worktree* during this audit rather than
read off a story's evidence file, and the tree was restored clean afterwards.
Every capability the project delivered is mounted in the real composition root
and was observed running there. The affected-package gate, the `--fast`
non-terminal gate and the full gate are all green, with **0** skipped steps.

The fifteen whole-initiative journeys are correctly deferred to `HS-P0025`; they
depend on content, a friction log and persona atoms that later projects owe, and
none of them is a dependency this project should have provided.

One thing this record deliberately does **not** claim, because the project's own
top-ranked risk is that someone reads it that way: a green gate here proves that
the code inside the prose compiles and runs, and proves **nothing whatever**
about whether any page teaches anybody anything. That instrument is `HS-P0024`'s
friction log, and no run recorded above substitutes for it.
