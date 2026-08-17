---
item: HS-S0145
stage: spec
created: 2026-08-17T13:16:06.300Z
updated: 2026-08-17T13:16:06.300Z
template_sig: 87bbf1d0
rendered_sig: 78356bbb
---

# Spec — What the check does not verify is stated first, and executed where it can be

## Scope lock

| What | Where |
| ---- | ----- |
| Initiative | `.bklg/docs-that-teach/initiative.md` (BR-01, BR-02; DoD 2; the risk "a green documentation gate is mistaken for evidence of teachability", `initiative.md:500`) |
| Project | `.bklg/docs-that-teach/checked-documentation-surface/project.md` (AC-010; DR-11; DoD items 7 and 8) |
| This spec | `.bklg/docs-that-teach/checked-documentation-surface/documented-blind-spots-and-their-proofs/spec.md` |
| Key briefs | `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` — **architecture brief Note 10** (the exact six-limit list, and the only place it is enumerated), Note 3 (the harness/markdown pointer degradation), Note 1 CR-1/CR-2 (the two targets); **testing brief, AC-010 entry** (which limits are testable and which is not) |
| Signed-off design | `.bklg/docs-that-teach/checked-documentation-surface/_design.md` — binding. `## What a user meets first` ("the checker's **What this does not verify** section, which is the *first* thing in the new modules' docs"), `## Anti-patterns` 9 (no badge, tick or "verified" mark), `## Items` / `## Signatures` (the constants whose docs this story writes) |
| Story map row | `.bklg/docs-that-teach/checked-documentation-surface/_storymap.md` — milestone `falsification-and-limits`, last row; and the Coverage row for AC-010 |
| Roadmap pointer | `.bklg/docs-that-teach/checked-documentation-surface/_storymap.md` `## Merge order` step 4.10 — this is the project's **final** story, "the project's final honesty pass" |
| Discover | `.bklg/docs-that-teach/checked-documentation-surface/documented-blind-spots-and-their-proofs/discover.md` (named wrong implementation: "a limitations section that lists blind spots in prose with no accompanying demonstration") |

## One-line PR slice

Put a "What this does not verify" section first in the new modules' own docs carrying all six
enumerated limits, re-run the `RUSTDOCFLAGS` probe against the new step and record what it
actually does, and walk a `text`-tagged broken fixture through the gate to prove that limit is
real rather than asserted.

## Executive summary

The four milestones before this one built a machine and watched it fail. This PR lands the
sentence that stops the machine from being over-read, and — where a sentence can be executed —
the execution.

Delta over what is already green when this PR opens:

- **A section, positioned.** Both modules this project added carry `# What this does not
  verify` as the **first** heading in their module docs, holding Note 10's six limits, each
  stated at the module where it actually holds. Today those modules carry whatever minimum
  their own story owed for what it added; the *contents* were deferred here on purpose,
  because the list of limits is only complete once every check that has a limit exists
  (`_storymap.md` `## Merge order` 4.10).
- **One limit measured instead of cited.** Limit 3 is the `RUSTDOCFLAGS` gap, and this
  repository holds two claims about it that do not agree: `xtask/src/constitution.rs:31-36`
  records a probe in which `RUSTDOCFLAGS=-D warnings` *did* recover rustc's default-on lints
  inside a doctest, while the upstream reports say `cargo test --doc` drops the variable
  (cargo#13697 → rustc#67533, `_discovery/research/02-…`). Note 10 item 3 forbids copying
  either. The probe is re-run against the *new* step, on the pinned 1.97.1 toolchain, and the
  transcript is what the docs then state.
- **One limit executed instead of asserted.** Limit 5 — a Rust example tagged `text` is
  neither compiled nor flagged — is walked: a fixture page whose `text` fence is deliberately
  false goes through the whole gate and is **observed to pass**. RS-81-1's requirement is that
  the blind spot be proved in the check's own tests and then stated, not stated and then
  believed (`standards/rust/81-checks-that-cannot-be-types.md:11`).
- **A coupling that makes the section rot loudly.** Presence and ordering are enforced by a
  test that can fail, and the `text` fixture is pinned by a positive test through the real
  checker — so a later change that *closes* limit 5 breaks a test whose failure message says
  the limits section is now wrong.

What this PR deliberately does **not** add: any claim, anywhere, that the surface proves a page
teaches (project DoD item 8), and any instrument that pretends to cover limit 1.

## Context pack

Everything below is a decision this story must honor, not a reading list. The deeper artifacts
sit behind the anchors.

**The persona slice.** The "user" of this project is a **contributor running the gate** and a
**reviewer reading its output** (`_storymap.md` preamble) — this project ships no runtime
surface. This story adds a third reader who matters more than either: the **author of a
downstream project** (HS-P0021 – HS-P0024) who opens the checker to find out what a green run
bought them. `_design.md` `## What a user meets first` fixes what that person meets: the limits
section, first, "not the last". The journey is one hop long and it is the whole story — open the
module, read what it cannot see, before reading what it can.

**The limits are Note 10's six, and the list is additive-only.** Architecture brief Note 10 is
the only place they are enumerated, and it is normative here: (1) code that still compiles while
no longer demonstrating the surrounding claim; (2) doctests do not receive the workspace `[lints]`
set and `cargo clippy` does not lint doctests at all; (3) the `RUSTDOCFLAGS` gap, *stated as
measured*; (4) the reported file is the harness, not the markdown; (5) a Rust example tagged
`text` is invisible to the fence check; (6) one unhedged sentence that this says nothing about
teaching. A seventh may be **added** if the delivered checks have one Note 10 predates — the
research already names the live candidate, `compile_fail` on stable checking only *that*
compilation failed and not *why* (`_discovery/research/02-…`) — but none of the six may be
dropped, softened, or moved out of first position.

**Limit 1 is inherently untestable, and must say so.** The testing brief's AC-010 entry is
explicit: for the compiles-but-no-longer-demonstrates gap "no test is possible (it is a semantic
gap, not a mechanical one) — document it as inherently untestable and say so", following
`xtask/src/constitution.rs:20-36`, which documents exactly this class of limit with no test
behind it because none exists. Writing a test that gestures at limit 1 is worse than writing
none: it is the "reader deleting the *real* instrument because the grep looks like it already
covers the ground" failure RS-81-1 names. The real instrument is HS-P0024's friction log, and
the section names it as non-substitutable.

**Two modules, two Cargo targets, and only one of them is ever rendered.** `xtask` has a lib
target (`xtask/src/lib.rs`, whose `mod constitution;` at `:28` is the doctest root) and a bin
target (`xtask/src/main.rs:64-70`), and they share no modules (architecture brief Note 1, CR-1
and CR-2). Measured in this worktree: `cargo doc -p xtask --no-deps --document-private-items`
produced `target/doc/xtask/` containing only `constitution/` — the **lib** target. The bin
crate's module docs are rendered by nothing in the gate. Three consequences this story is
built on:

1. The checker's limits section cannot be enforced by rustdoc. Presence, ordering and contents
   must be checked by something that reads the module **as text** — the same cross-target
   technique `check_harness` already uses to prove a fact about a file in the other target
   (`xtask/src/lint_constitution.rs:423-425`).
2. The proof is already in the tree that this is true: `lint_constitution.rs:5` writes
   `` [`crate::constitution`] `` from the *bin* crate, where `crate::constitution` does not
   exist, and the gate's `documentation` step denies every rustdoc warning
   (`RUSTDOCFLAGS=-D warnings`, `xtask/src/main.rs:284-302`) and has never failed on it.
3. The harness's docs *are* rendered and *are* held to `-D warnings`. Also measured:
   `target/doc/xtask/constitution/` holds only `index.html` — the `#[cfg(doctest)]` page
   modules do not exist under `cargo doc`. So the limits section in the harness may not carry
   an intra-doc link to a page module; that is a broken link, and a broken intra-doc link is a
   hard error under that step, not a warning.

**Cross-references between the two sections are plain paths in prose, not intra-doc links**, for
the same reason. State each limit where it holds — 1, 2, 3 and 4 are properties of the compile
mechanism and belong to the harness; 5 is a property of the fence walk and belongs to the
checker; 6 belongs to both, unhedged, in both — and let the other side name its counterpart by
path. Restating all six in both places would be two things to update and one that goes stale.

**What this story does not re-decide, because a dependency or the design already fixed it.**
The tree is `docs/`, pinned as `TREE` (`_design.md` D1 and `## Signatures`). DT-7 is closed —
hidden markers are rejected outright, with no allowance list (`_design.md` D2). The allowance
list's shape, the citation resolver, and the frozen-MUST pin are the three checks whose limits
this story enumerates; their behaviour belongs to `fence-discipline-and-allowance-list`,
`narrative-citation-resolution` and `frozen-documentation-must-pin`. The checker module's file
name, its step name, its subcommand and its `lint_steps` membership are
`narrative-checker-mounted-with-pinned-path`'s and are consumed here exactly as delivered —
step names are strings other code depends on by value, and `steps_named` panics on a name absent
from `REQUIRED` (`xtask/src/main.rs:799-826`), so this story reads them and never rewords them.

**Limit 4 is quoted from a run, not predicted.** Note 3 records that a fence failure inside an
`include_str!` page is reported against the *harness* file, with the page resolved by module
name and the line resolved inside the page. `observed-failure-falsification` has by now run the
real thing and recorded the verbatim output. Limit 4's wording is taken from that record. If the
recorded output disagrees with Note 3, **the record wins and the limit says what the record
says** — the whole point of that story is that the evidence outranks the forecast.

**No ADR, and no governance trigger.** The grounding pass verified that no Accepted decision
atom under `.kb/decisions/` governs gate structure, documentation trees or fence compiling, and
found no tension with one (architecture brief `### Intent`). This story writes module docs in
`xtask`, which discharges no `SPECIFICATION.md` clause, so
`.kb/governance/rewrite-the-referent-never-the-reasoning.md` is not engaged — the doc comments
it governs are `crates/happenstance-core/`'s, and they belong to `frozen-documentation-must-pin`
and to HS-P0023.

**The honesty constraint is a design constraint, not only a prose one.** `_design.md`
`## Anti-patterns` 9 forbids "a badge, tick, shield or 'verified' mark asserting the
documentation is checked for correctness or comprehension", and `## What a user meets first`
closes with "neither meets a claim that this surface proves a page teaches". This story is the
last one that could introduce such a claim and the one most tempted to, because it is the story
that gets to describe what the machine does.

## Integration contract

- **Slice / milestone**: `falsification-and-limits`. Slice-mate: `observed-failure-falsification`
  (implemented with this story in one context, and landing first inside it — its recorded output
  is this story's input for limit 4).
- **Archetype**: `capability`. The user-observable slice is the section a contributor reads plus
  the two proofs that keep it honest; both are reachable from `cargo xtask ci`, not from a test
  harness only.
- **Mount point**: `xtask/src/narrative.rs` — the lib-target harness declared from
  `xtask/src/lib.rs` beside `mod constitution;` (`xtask/src/lib.rs:28`). This is the real render
  path for this story's surface and the only file it touches whose module docs a gate step
  actually renders (`documentation`, `xtask/src/main.rs:284-302`, measured above). **Second
  mount, in the other target**: the bin-crate narrative checker module declared in
  `xtask/src/main.rs:64-70` — its limits section and, in its own `#[cfg(test)]` block, the tests
  that make this story's obligations fail when they stop being true.
- **Wires into**:
  - `xtask/src/main.rs:284-302` — the `documentation` step, the one that holds rendered lib docs
    to `-D warnings`, and therefore the step the harness's section must survive.
  - `xtask/src/main.rs:488-492` — `"the constitution's examples compile"`, the exact `Step`
    shape (`cargo test --locked -p xtask --doc`, step-scoped `RUSTDOCFLAGS`, `probe: None`) that
    `pinned-narrative-tree-and-compiling-step` copied for the narrative tree; the probe re-run
    invokes the narrative step *as wired*, not a hand-typed approximation of it.
  - `xtask/src/main.rs:143` — the `tests` step, which is what runs the new `#[cfg(test)]` proofs
    in the gate.
  - `xtask/src/lint_constitution.rs:10-28` — the limits-section shape being copied, and `:11-13`
    the reason it is first.
  - `xtask/src/lint_constitution.rs:423-425` — reading the other target's file as text, the
    technique the presence test uses.
  - `xtask/src/constitution.rs:20-36` — the worked precedent for all of it: an untestable limit
    documented without a test, and the earlier `RUSTDOCFLAGS` probe this story re-runs.
  - `xtask/src/narrative.rs` — the harness, for the `include_str!` registration the retained
    fixture page needs; an unregistered page fails the orphan check
    (`_storymap.md` AC-005 row).
  - `docs/` — `TREE`, where the `text` fixture page lands, under the repo-relative path budget
    (≤ 32 characters; `_design.md` `## Density budget`, and `## Anti-patterns` 11).
  - `rust-toolchain.toml` — the pinned 1.97.1 the probe record must name, because a measurement
    without its toolchain is a claim about nothing.
- **Renders surfaces** (ids from `_design.md` `## Surfaces`): it **changes none of the six**. It
  adds one instance of `narrative-page` (the `text` fixture, state `default`) and it observes
  `gate-narrative-compile-step` and `gate-narrative-checker-step` in their `pass` states — the
  proof of limit 5 is precisely that both stay green. Composition, transience policy, density
  budget, hierarchy and states are `_design.md`'s and are not re-decided here.
- **Public items** (`_design.md` `## Items`): none added, changed or removed. This story authors
  the **module documentation** of the two modules that hold `TREE`, `HARNESS`,
  `IGNORE_ALLOWANCES` and `HIDDEN_MARKERS`, and adds no item of its own. `xtask` is
  `publish = false`; there is no semver promise to make.
- **Conformance rule(s)**: none, and this is not adapter-observable. Nothing here touches a
  port, an `EventStore` flavour, a `Send` bound or the testkit — `_design.md` `## What it costs a
  caller` states it: ADR-0001 is untouched by construction. The equivalent instrument is the
  `#[cfg(test)]` block in the checker module, and the named wrong implementations it rejects are
  enumerated in the acceptance criteria.
- **Clause(s)**: none discharged, amended or restated. Architecture brief Note 8 binds this:
  AC-007 and AC-008 *read* `spec/SPECIFICATION.md`; nothing in this project writes it. No
  `[FROZEN]` clause is touched, so no ADR is owed.
- **Advances DoD scenario**: initiative **DoD 2** — "a deliberately broken page fails the gate,
  by name" — which this story completes rather than starts: DoD 2's observation is
  `observed-failure-falsification`'s, and this story is what bounds what that observation is
  allowed to mean, which is the whole reason `initiative.md:500` ranks
  green-mistaken-for-teachable as a High/High risk. It also keeps **DoD 5/6** (the friction log
  and its dispositions) non-substitutable by writing the non-substitutability down, and it
  discharges project **DoD items 7 and 8** for the project as a whole.

**Delivered mounted, not as an isolated component**: every obligation in this PR is reachable
from `cargo xtask ci` on a clean checkout with no manual step — the presence tests run in the
`tests` step, the harness's section is rendered and lint-denied by the `documentation` step, and
the `text` fixture is compiled and walked by the two narrative steps on every run. A limits
section that only a human who opens the file can be relied on to check is the shape this story
exists to refuse.

## PR boundary

```
xtask/src/narrative.rs
xtask/src/*.rs
docs/**
.bklg/docs-that-teach/checked-documentation-surface/documented-blind-spots-and-their-proofs/**
```

`xtask/src/*.rs` rather than a single named file because the checker module's file name is
`narrative-checker-mounted-with-pinned-path`'s to choose and this story consumes it as
delivered; the honest boundary is "the two new modules, and nothing else in `xtask/src/`".
`docs/**` carries `TREE` and the retained fixture page. The story's own backlog folder carries
the evidence companion and, at the second pass, its ledger.

**In this PR**

- `# What this does not verify` as the first heading of both new modules' docs, holding the six
  limits, each stated where it holds, with a plain-path cross-reference to the other side.
- The re-run `RUSTDOCFLAGS` probe: three observations against the narrative compile step as
  wired, on 1.97.1, recorded verbatim in
  `.bklg/docs-that-teach/checked-documentation-surface/documented-blind-spots-and-their-proofs/_limits-evidence.md`,
  with limit 3's prose stating the measured outcome and citing that record.
- One retained `text`-fence fixture page under `TREE`, registered in the harness, walked through
  the full gate and observed to pass, with the walk recorded in the same companion.
- `#[cfg(test)]` tests in the checker module: the section-shape tests, and the positive test
  that pins the `text` fixture as unflagged.
- Limit 4's wording taken from `observed-failure-falsification`'s recorded output.
- The completeness reconciliation: every check the project delivered has its limit in the
  section, and the `compile_fail` candidate has a written disposition.

**Explicitly not in this PR**

- Any change to `HIDDEN_MARKERS`, `IGNORE_ALLOWANCES`, `TREE`, `HARNESS`, the clause-id
  resolver, the frozen-MUST pin, the fence walk's rules, or any step's name, args or `env`. If a
  limit is inconvenient, it is documented, not closed — closing one is a change to the story
  that owns it.
- Any teaching page. The fixture is test material; the corpus is HS-P0021/22/23's
  (`project.md` risk table).
- Any new `REQUIRED` step, subcommand, `print_help` line or `lint_steps` member. This story
  mounts into steps that already exist.
- Any test, badge or sentence that claims coverage of limit 1, or that the surface proves a page
  teaches.
- `mdbook`, a `book.toml`, a stylesheet, or a second rendered surface (`_design.md`
  `## Anti-patterns` 10).
- Any new dependency in `xtask/Cargo.toml` (DR-12, `xtask/Cargo.toml:16-21`).

**Merge DoD**: `cargo xtask ci --fast` is green, the six limits are first in both modules and
each is either executed or documented as inherently untestable with its reason, and the
`RUSTDOCFLAGS` probe record and the `text`-fence walk are in the story's own artefacts as
verbatim transcripts naming the toolchain and the exact commands.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| -------------------- | ------- | ------------- |
| The section is first, in both modules | `# What this does not verify` is the first `#`-level heading in the module docs of the harness and of the checker. Not a bullet inside another section, not below a "how it works" preamble. The reason is quoted from the precedent, not re-argued: "a check whose limits are undocumented is read as a guarantee". | `xtask/src/lint_constitution.rs:10-13`; `_design.md` `## What a user meets first` |
| The six limits, each where it holds | Limits 1–4 in the harness (they are properties of the compile mechanism), limit 5 in the checker (a property of the fence walk), limit 6 unhedged in both. Each limit is one bullet in the shape `lint_constitution.rs:15-28` uses: what is not verified, and what the real instrument is. | `_decomposition.md` architecture brief Note 10; `xtask/src/lint_constitution.rs:15-28` |
| Additive-only, with a disposition for the candidate seventh | A limit may be added if a delivered check has one Note 10 predates; none may be dropped. The one live candidate is `compile_fail` asserting only *that* compilation failed, not *why* (the error-code form is nightly-only and "unlikely to be stabilized"). Its disposition — added, or not-applicable because the delivered fence walk does not permit `compile_fail` — is written down either way. | `.bklg/docs-that-teach/_discovery/research/02-compiled-prose-tooling-mdbook-test-doc-comment-skeptic-doc-i.md`; `xtask/src/lint_constitution.rs:644-660` |
| Cross-target references are plain paths, never intra-doc links | The bin crate has no path to the lib's private modules, and the `#[cfg(doctest)]` page modules do not exist under `cargo doc` at all — measured: `target/doc/xtask/constitution/` holds only `index.html`. A link to either is a broken intra-doc link, which the `documentation` step turns into a hard error. | `xtask/src/main.rs:284-302`; `xtask/src/lib.rs:21,28`; `xtask/src/constitution.rs:41-44` |
| Presence is enforced by a text-reading test, because rustdoc cannot see the checker | `cargo doc -p xtask --no-deps --document-private-items` documents the **lib** target only; the bin crate's docs are rendered by nothing in the gate — which is why `lint_constitution.rs:5`'s `` [`crate::constitution`] `` has never failed a gate that denies every rustdoc warning. So the test reads both module files as text through `workspace_root()`, exactly as `check_harness` reads the other target's file. | `xtask/src/lint_constitution.rs:5,423-425`; `xtask/src/main.rs:64-70,284-302` |
| The presence test names its wrong implementations | It must reject: a section moved below another heading; a module missing one of its limits; limit 6 present but hedged away (a "but the gate does check…" clause on the teaching sentence). A test that only asserts the heading exists is decorative — RS-81-1's standard, and CLAUDE.md's "name a plausible wrong implementation it rejects" one level up. | `standards/rust/81-checks-that-cannot-be-types.md:11`; `xtask/src/lint_constitution.rs:828-878` (test-shape house style) |
| Limit 3 is measured against the step as wired | Three observations, each a verbatim transcript: (a) the narrative compile step run exactly as `REQUIRED` declares it, with a fence carrying a rustc **default-on** lint violation (`non_snake_case`, the same trigger the earlier probe used) — does `-D warnings` reach it? (b) the same fence with the step's `RUSTDOCFLAGS` removed, for the contrast; (c) a fence violating a workspace `[lints]`/clippy rule (`unwrap_used`) — expected unenforced, because clippy does not lint doctests at all. The probe fences are temporary and reverted; the transcripts are the artefact. | `xtask/src/constitution.rs:26-36`; `xtask/src/main.rs:488-492`; `rust-toolchain.toml` |
| Limit 3's prose states the measurement, not either citation | The two available claims disagree — the in-repo probe found partial recovery, the upstream reports say the variable is dropped by `cargo test --doc` (cargo#13697 → rustc#67533, still open). The docs say what the re-run did, on 1.97.1, and cite the record; the upstream issues are named as context, never as the finding. | `_decomposition.md` Note 10 item 3; `_discovery/research/02-…` |
| Limit 5 is walked, not asserted | A retained fixture page under `TREE` carries a ` ```text ` fence whose content is deliberately false about the library, and prose saying so. The whole gate runs and **passes**: the compile step never sees the fence, the fence walk permits the `text` tag, and no problem line is emitted. Recorded verbatim. | `_decomposition.md` testing brief AC-010 entry; `_storymap.md` AC-010 Coverage row |
| The `text` fixture is registered and budgeted | It gets its `include_str!`/`mod` in the harness or the orphan check fails it, its repo-relative path stays ≤ 32 characters with no third directory level, and it follows whatever treatment `pinned-narrative-tree-and-compiling-step` established for the first fixture page rather than inventing a second convention. | `_design.md` `## Density budget`, `## Anti-patterns` 11; `_storymap.md` AC-005 row |
| A positive test pins the limit so closing it breaks loudly | A `#[cfg(test)]` test runs the real checker over the fixture and asserts **no problem** is reported for the `text` fence. If a later change teaches the fence walk to inspect `text` fences, this test fails — and its failure message says the limits section is now wrong and limit 5 must be deleted in the same change. | `standards/rust/81-checks-that-cannot-be-types.md:11`; `xtask/src/lint_constitution.rs:601-673` |
| Limit 1 is documented as inherently untestable, with no instrument pretending otherwise | One bullet stating the gap, one sentence stating that no mechanical test can close it and why (it is semantic, not mechanical), and one naming HS-P0024's friction log as the non-substitutable instrument. No test, no fixture, no metric. The precedent documents exactly this class of limit with nothing behind it. | `xtask/src/constitution.rs:20-25`; `_decomposition.md` testing brief AC-010 entry; `project.md` risk table row 1 |
| Limit 4 is quoted from the observed run | The wording comes from `observed-failure-falsification`'s recorded verbatim output — which file the failure actually names, how the page is resolved (module name) and how the location is resolved (line inside the page). If the record disagrees with Note 3's forecast, the record wins and the limit is reworded to match it. | `.bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/`; `_decomposition.md` Note 3 |
| Nothing claims the surface proves teaching | Limit 6 is one unhedged sentence in both modules. No badge, tick, shield or "verified" mark; no "documentation is checked" phrasing in `docs/README.md`, the fixture page, a step name, or a commit message. This is a review check on prose in every story and this story's own AC. | `_design.md` `## Anti-patterns` 9, `## What a user meets first`; `project.md` DoD item 8; `_storymap.md` "Two project-level obligations" |
| The completeness reconciliation is written down | Walking the delivered checks — pinned tree, empty-tree guard, bidirectional registration, fence discipline, allowance sweep, hidden markers, citation resolution, frozen-MUST pin, count agreement — each either has its limit in the section or is recorded as having none, with the reason. A check with an unstated limit fails this story. | `_design.md` `## The states the API must express`; `_decomposition.md` Note 10 |
| The gate the section must survive | `cargo xtask ci --fast` green — including `documentation` (`-D warnings` over the rendered harness docs), `tests` (the new `#[cfg(test)]` proofs), `the constitution is internally consistent`, `the constitution's examples compile`, and the two narrative steps. Plus `cargo xtask affected --base main` selecting `xtask` for this change, which is the story grain `.redkiln/config.yaml` wires. | `xtask/src/main.rs:143,284-302,466,488-492`; `.redkiln/config.yaml:28-40`; `project.md` DoD items 6 and 7 |
| Evidence lives in the story's own artefacts | `_limits-evidence.md` in this story's folder holds the three probe transcripts (with commands and toolchain), the `text`-fence walk, the completeness reconciliation, and the `compile_fail` disposition. The module docs cite it by path; a limit whose measurement lives only in a terminal someone closed is a limit nobody can re-check. | `project.md` DoD item 2's standard ("both halves written down"); `_storymap.md` AC-010 Coverage row |

## Data and migrations

**N/A.** There is no data store, no schema, no persisted runtime state and no published surface
in this story — the deployment brief states it for the project as a whole ("no data store, no
schema, no persisted state"), and `xtask` is workspace tooling that is never published, so no
crate version bump, changelog line or migration is owed.

Two artefact-shaped consequences are worth stating so they are not mistaken for data work:

- **The probe fences are transient; the transcripts are permanent.** The `non_snake_case` and
  `unwrap_used` fences exist only for the duration of the three observations and are reverted in
  the same PR. What persists is `_limits-evidence.md`. Leaving a probe fence behind would either
  fail the gate or, worse, pass it and become an unexplained example.
- **The `text` fixture page is permanent, and its removal is a two-file change.** It is retained
  rather than deleted so limit 5's proof stays re-runnable (testing brief: "keeping them is
  consistent with DoD-2's 'observed to fail' needing to remain observable"). Removing it means
  removing its harness registration in the same change, or the orphan check fails — and it means
  deleting the positive test that pins it, which is the signal that limit 5's status changed.

Rollback is `git revert` of this commit, with the deployment brief's one hazard restated: do not
revert this project's steps in isolation once HS-P0021+ has merged content into the tree they
pin. Reverting *this* story alone is safe and leaves the machine intact — it removes the honesty
section and its two proofs, which is exactly the state the project was in before this PR and the
reason the project is not done without it.
