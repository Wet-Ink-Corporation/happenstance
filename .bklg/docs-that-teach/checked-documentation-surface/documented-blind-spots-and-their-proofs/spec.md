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
- **Renders surfaces** (ids from `_design.md` `## Surfaces`): it **changes none of the five**. It
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

## Acceptance criteria

The persona throughout is the one `_design.md` `## What a user meets first` names: **a
contributor who opens one of this project's new modules**, and behind them **the author of a
downstream project (HS-P0021 – HS-P0024) deciding what a green gate bought them**. The journey
these criteria serve is *Survive the second question*
(`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md`, carried into
`initiative.md:289-301`): the reader's first question — "does the gate check my prose?" — was
answered by the four milestones before this one; the second question is "then what is it still
blind to?", and a criterion here is met only when that reader gets the answer without leaving the
file they opened.

| id | criterion | verification |
| -- | --------- | ------------ |
| AC-001 | **GIVEN** a contributor opens `xtask/src/narrative.rs` to find out what the compiling step actually buys them, **WHEN** they read from the top of the file, **THEN** the very first `#`-level heading in the module docs is `# What this does not verify` — persistent prose composed as a real rustdoc heading with a bulleted body, never a `//` comment, never a bullet nested under a "how it works" preamble, and never behind a `<details>` — **AND** the `documentation` step stays green under `RUSTDOCFLAGS=-D warnings`, which the section earns by carrying no intra-doc link to a `#[cfg(doctest)]` page module (those modules do not exist under `cargo doc`, so such a link is a hard error). | unit: the section-shape test in the checker module reads `HARNESS` as text through `workspace_root()` and asserts the first `#` heading and its position (`xtask/src/lint_constitution.rs:423-425` is the technique); gate: `cargo xtask ci --fast` → `documentation` (`xtask/src/main.rs:284-302`) |
| AC-002 | **GIVEN** the same contributor opens the bin-crate narrative **checker** module instead — the half that reads pages rather than compiling them — **WHEN** they read from the top, **THEN** they meet `# What this does not verify` first there too, and its reference to the harness's four limits is a **plain path in prose**, not an intra-doc link, because the bin crate has no path to the lib target's modules (`xtask/src/lint_constitution.rs:5` is the standing proof that such a link fails nothing and therefore protects nothing). | unit: the same section-shape test, second arm, reading the checker module's own file; review: no `` [`crate::…`] `` spanning the target boundary |
| AC-003 | **GIVEN** a downstream author who must decide whether a green run licenses them to stop checking a claim by hand, **WHEN** they read both sections, **THEN** they find all six of Note 10's limits — each stated **where it actually holds** (1–4 in the harness, 5 in the checker, 6 unhedged in both) and each written in the precedent's two-part bullet shape: what is not verified, and what the real instrument is — so no limit is discoverable only by reading the other target's file. | unit: a per-limit presence test asserting each module carries its assigned limits and does **not** restate the others; source: `_decomposition.md` Note 10 (`:351-380`); shape: `xtask/src/lint_constitution.rs:15-28` |
| AC-004 | **GIVEN** a reader who wants to know whether the headline gap — code that still compiles while no longer demonstrating the surrounding claim — is covered by anything, **WHEN** they read limit 1, **THEN** it says the gap exists, says **no mechanical test can close it and why** (it is semantic, not mechanical), and names HS-P0024's friction log as the non-substitutable instrument — **AND** no test, fixture, metric or count anywhere in this PR gestures at covering it, because a decorative instrument is what makes a reader delete the real one. | unit: the presence test asserts limit 1 carries an explicit inherently-untestable clause; review + diff: no test named for limit 1; precedent: `xtask/src/constitution.rs:20-25` |
| AC-005 | **GIVEN** a contributor who has read two mutually contradicting claims about `RUSTDOCFLAGS` in this repository and upstream, **WHEN** they read limit 3, **THEN** it states what the probe **did**, re-run against the narrative compile step as `REQUIRED` declares it on the pinned 1.97.1 toolchain, cites `_limits-evidence.md` for the three verbatim transcripts (default-on lint under the step's `RUSTDOCFLAGS`; the same fence with it removed; a clippy-only lint), and names the upstream reports as context rather than as the finding — **AND** every probe fence is reverted in the same PR, so the transcripts persist and the fences do not. | evidence: `_limits-evidence.md` holds three transcripts each naming its exact command and `rust-toolchain.toml`'s pin; unit: the presence test asserts limit 3 cites the record path; gate: `cargo xtask ci --fast` green with no probe fence remaining |
| AC-006 | **GIVEN** a reader who is told a Rust example tagged `text` is invisible to the check, **WHEN** they want to know whether that is a real hole or a cautious sentence, **THEN** a retained fixture page under `TREE` whose ` ```text ` fence is deliberately false about the library has been walked through the **whole gate** and observed to **pass** — the compile step never sees it, the fence walk permits the tag, no problem line is emitted — with the run recorded verbatim in `_limits-evidence.md`, reachable by one non-interactive command with no manual step. | end-to-end: `cargo xtask ci --fast` over the tree containing the fixture, output recorded; narrower: `cargo test --locked -p xtask --doc` and the checker's own subcommand, both green |
| AC-007 | **GIVEN** a future contributor who teaches the fence walk to inspect `text` fences and closes limit 5, **WHEN** they run the gate, **THEN** a `#[cfg(test)]` test that runs the **real** checker over the fixture and asserts *no problem is reported for the `text` fence* fails, with a message saying the limits section is now wrong and limit 5 must be deleted in the same change — so the section rots loudly rather than silently. | unit: the positive pinning test in the checker module's `#[cfg(test)]` block, run by the `tests` step (`xtask/src/main.rs:143`); its failure message is asserted by review against RS-81-1 |
| AC-008 | **GIVEN** a contributor who has just watched the gate fail and is trying to map the filename in the output onto the page they broke, **WHEN** they read limit 4, **THEN** its wording is **quoted from the run** `observed-failure-falsification` recorded — which file the failure names, that the page is resolved by module name and the line inside the page — and where that record disagrees with the architecture brief's forecast (Note 3), the limit says what the record says. | evidence: the wording is traceable to `.bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/`'s recorded output; review: forecast-vs-record reconciliation stated in `_limits-evidence.md` |
| AC-009 | **GIVEN** a later change that quietly weakens the honesty section — moves it below another heading, drops a limit, or hedges limit 6 with a "but the gate does check…" clause — **WHEN** the gate runs, **THEN** it fails, because the section tests were written against those three **named wrong implementations** rather than against the mere existence of a heading. | unit: three negative tests (or three assertions with distinguishable messages) in the checker module, each rejecting one named wrong implementation; standard: `standards/rust/81-checks-that-cannot-be-types.md:11`; test-shape house style `xtask/src/lint_constitution.rs:828-878` |
| AC-010 | **GIVEN** the project is being called done, **WHEN** a reviewer asks whether *every* delivered check has its limit on the record, **THEN** `_limits-evidence.md` carries the reconciliation — pinned tree, empty-tree guard, bidirectional registration, fence discipline, allowance sweep, hidden markers, citation resolution, frozen-MUST pin, count agreement — each with its limit in the section or a written reason it has none, plus a written disposition for the `compile_fail` candidate seventh (added, or not-applicable because the delivered fence walk does not permit `compile_fail`). | review artefact: the reconciliation table in `_limits-evidence.md`, one row per delivered check; source: `_decomposition.md` Note 10 and `_design.md` `## The states the API must express` |
| AC-011 | **GIVEN** any reader of anything this project produced — module docs, the fixture page, `docs/README.md`, a step name, a commit message — **WHEN** they look for a claim that the surface proves a page teaches, **THEN** there is none: limit 6 is one unhedged sentence in both modules, and no badge, tick, shield or "verified" mark appears anywhere, nor any `book.toml`, `book/`, `site/` or `.css` that would create a second rendered surface able to carry one. | review: prose scan across the PR diff (`project.md` DoD item 8; `_design.md` `## Anti-patterns` 9 and 10); unit: the presence test asserts limit 6 present and unhedged in both modules |
| AC-012 | **GIVEN** a reader who lands on `docs/README.md` and follows a row to the new fixture page, **WHEN** the page renders at the narrow width, **THEN** it is a conforming `narrative-page` in the signed-off design's terms: registered in the harness in both directions, no `HIDDEN_MARKERS` token anywhere, its repo-relative path ≤ 32 characters with no third directory level, H1 ≤ 40 characters, fences ≤ 80 columns, page ≤ 250 source lines, an index row in the two-column table above the pointer-out table — and it adds one row without reordering the rows already there. | unit: the checker's own registration/marker/path checks run over the fixture as part of `cargo xtask ci --fast`; review against `_design.md` `## Density budget`, `## Composition`, `## Anti-patterns` 1 and 11 |

**Coverage of the traced project AC.** AC-010 of `project.md` — *the limits are on the record* —
is discharged in the two halves `_storymap.md`'s Coverage row requires: *static presence* by
AC-001, AC-002, AC-003, AC-009, AC-011 and *executed proof where a proof is possible* by AC-005,
AC-006, AC-007, AC-008, with AC-004 discharging the row's explicit carve-out ("the
compiles-but-no-longer-demonstrates gap is documented as inherently untestable") and AC-010 and
AC-012 closing the project-level obligations DoD items 7 and 8 leave carried across every story.

## Interaction quality

Two media, and neither is a web page: rustdoc's rendered module docs and plain CommonMark under
`TREE`. So the state invariants below are named in the terms those media actually have, and every
one that applies is a **row in the table above** — this section only says which id carries it.

**State invariants**

| Invariant | Its meaning in this medium | Carried by | Verified by |
| --------- | -------------------------- | ---------- | ----------- |
| In place, not a context jump | The answer to "what is this blind to?" is in the file the contributor already opened. `_limits-evidence.md` is cited as provenance for a measurement, never as the place the limit is stated — a limit that can only be read in a backlog folder is a context jump. | AC-001, AC-002, AC-003, AC-005 | the section-shape and per-limit presence tests read the module files themselves |
| Non-occlusion | Nothing a reader must act to reveal. The limits section is persistent chrome in both modules, and the fixture page carries no `HIDDEN_MARKERS` token — `_design.md` D2 rejects folds outright, with no allowance list. | AC-001, AC-002, AC-012 | the presence tests assert position; the checker's `HIDDEN_MARKERS` walk runs over the fixture |
| Preserved focus / scroll / selection | The markdown analogue: source order is render order and this story appends. The fixture adds one index row and one harness registration without reordering the rows or modules already present, so a reader's place in `docs/README.md` survives the change. | AC-012 | review of the diff against `_design.md` `## Composition`; the harness registration check |
| Reversibility | Two directions. The probe fences are transient and reverted inside the same PR; and `git revert` of this story alone restores the pre-PR state without disturbing the machine the earlier milestones built. | AC-005, AC-006 | `cargo xtask ci --fast` green with no probe fence remaining; the retained fixture is the only permanent artefact and its removal is a documented two-file change |
| Keyboard reachability | No interactive control exists anywhere in this story. The equivalent obligation is that every proof is reachable by one non-interactive command on a clean checkout — no tool to install, no manual step, no `probe:` shape. | AC-006, AC-007 | `cargo xtask ci --fast` and `cargo test -p xtask` reach both proofs (`project.md` DoD items 6 and 7) |

**Composition invariants** — taken from the signed-off `_design.md`, which this story implements
and does not re-decide.

| Invariant | The design's number or rule | Carried by | Verified by |
| --------- | --------------------------- | ---------- | ----------- |
| Presentation exists at all | The section is composed prose a renderer sets apart — a level-1 rustdoc heading with a bulleted body, in the shape `lint_constitution.rs:10-28` already uses. A `//` comment, or six sentences run together in a paragraph, satisfies every "the words are present" grep and fails this. | AC-001, AC-002, AC-003 | the section-shape test asserts heading level and bulleted structure, not just substring presence |
| Composition and placement | `## What a user meets first`: the limits section is the *first* thing in the new modules' docs, "not the last". Each limit sits in the module where it holds; cross-references are plain paths. | AC-001, AC-002, AC-003 | position asserted by test; module assignment asserted per limit |
| Transience | `## Transience policy`: fenced examples and the claim band are persistent chrome; problem lines are opened on demand *by failing*; there is no third state where content is present but hidden. The limits section inherits the first policy. | AC-001, AC-002, AC-012 | presence tests plus the `HIDDEN_MARKERS` walk over the fixture |
| Density budget, with the real numbers | `## Density budget`: repo-relative page path **≤ 32 characters** (the 48-character location prefix minus the 16-character doctest prefix), H1 **≤ 40 characters**, fence **≤ 80 columns**, page **≤ 250 source lines**, inline scope band **≤ 3 scopes × ≤ 25 lines**, index table **exactly 2 columns**. Doc-comment prose wraps at **≤ 90 columns**, the corpus p90 + headroom. | AC-012, and NF-005 for the prose wrap | the checker enforces the path budget at the pinning check; the rest is reviewed against the design |
| Hierarchy | `## Hierarchy`: nothing is carried by colour, weight or size — only position, heading level and adjacency. In the module docs the level-1 limits heading is primary and its bullets secondary; on the fixture page the H1 and fence band are primary, the claim band secondary by adjacency, the scope band recessive by position. | AC-001, AC-002, AC-003, AC-012 | heading-level assertions in test; page composition reviewed against `_design.md` `## Composition` |
| Anti-pattern 9 — no badge, tick, shield or "verified" mark | The design's honesty constraint, and this is the story most tempted to breach it. | AC-011 | prose scan of the whole diff |
| Anti-pattern 10 — no `book/`, `site/`, `book.toml` or `.css` | A second rendered surface is a place a teaching claim could live unchecked. | AC-011 | file-listing check over the diff |
| Anti-pattern 1 — no disclosure triangle, tab strip or collapsed callout under `TREE` | DT-7's answer, enforced by `HIDDEN_MARKERS`. | AC-012 | the checker's marker walk over the fixture |
| Anti-pattern 11 — no third directory level, no filename over 32 characters | The density rule that is a gate rule because it protects the terminal surface. | AC-012 | the checker's pinning check |
| Anti-pattern 5 — no untagged fence | The fixture's fence is explicitly tagged `text`; that tag is the *subject* of the proof, not an opt-out. | AC-006 | the fence walk runs over the fixture and reports nothing |

Anti-patterns 2, 3, 4, 6, 7 and 8 are not engaged: this story authors no navigation affordance,
no "Run" button, and emits no problem line of its own — AC-007 asserts the absence of one.

## Error conditions

| id | Condition | Required handling |
| -- | --------- | ----------------- |
| EC-001 | The re-run `RUSTDOCFLAGS` probe returns a **third** answer, agreeing with neither `constitution.rs:31-36` nor the upstream reports. | Record it verbatim and state it. Do not reconcile it against either prior claim, and do not soften limit 3 into "may not". Note 10 item 3 asks for the measurement precisely because the two available claims disagree. |
| EC-002 | The probe cannot be run as wired — the narrative compile step is absent, its `env` differs from what `REQUIRED` declares, or the toolchain is not `rust-toolchain.toml`'s pin. | Stop. Limit 3 may not be written from citation. A hand-typed approximation of the step measures a different step, and a measurement without its toolchain is a claim about nothing. |
| EC-003 | The `text` fixture is **flagged** by the fence walk — limit 5 is already closed by a dependency's implementation. | Limit 5 is false and is deleted, with the disposition recorded in `_limits-evidence.md` and AC-006/AC-007 re-scoped in the ledger with an explicit note. Do **not** weaken the fence walk to make the documented limit true; that is a change to `fence-discipline-and-allowance-list`'s story. |
| EC-004 | `observed-failure-falsification`'s recorded output disagrees with architecture brief Note 3 about which file the failure names. | The record wins and limit 4 is reworded to match it. Note the divergence in `_limits-evidence.md` so the forecast's failure is itself on the record. |
| EC-005 | A delivered check has a limit Note 10 does not enumerate. | Add it as a seventh (or eighth) bullet with its own evidence line. Never drop, soften or reorder one of the six to make room. |
| EC-006 | The fixture page's repo-relative path exceeds 32 characters, or would need a third directory level. | Rename the page. Do not raise the budget: it is derived from the compile surface's uncounted 16-character doctest prefix (`_design.md` finding 3), and raising it starves the terminal surface. |
| EC-007 | The harness's limits section carries an intra-doc link to a `#[cfg(doctest)]` page module or to the bin crate. | Hard error under the `documentation` step's `-D warnings`. Replace with a plain path in prose. `lint_constitution.rs:5` is the standing example of the link that nothing catches — do not add a second. |
| EC-008 | A probe fence survives into the merged commit. | Gate failure, or worse a silent pass leaving an unexplained deliberately-wrong example in the tree. The transcripts are the artefact; the fences are not. |
| EC-009 | The completeness reconciliation finds a delivered check whose limit nobody can state. | This story fails. A check with an unstated limit is the guarantee-by-silence RS-81-1 names; resolve it by writing the limit, not by omitting the check from the table. |

## Non-functional

| id | Requirement | Why, and where it comes from |
| -- | ----------- | ---------------------------- |
| NF-001 | **Zero new dependencies** in `xtask/Cargo.toml`. | DR-12's standing trade at `xtask/Cargo.toml:16-21` keeps heavy database crates out of the dev graph because every `cargo xtask ci` would build them. A doc-presence check needs nothing but `std` and the file reads the checker already does. |
| NF-002 | **Gate time**: one additional page in the harness's doctest set and one additional file in the checker's line scan. No new process spawn, no new step. | `_design.md` `## What it costs a caller`; the cost class `xtask/src/affected.rs:28-36` already argues finishes inside the time cargo takes to decide `xtask` is up to date. |
| NF-003 | **MSRV, wasm32 and features unaffected** — no `cfg`, no feature gate, no target-specific code. | `_design.md` `## What it costs a caller`. ADR-0029's 1.97.1 floor is named by the probe record as the toolchain of measurement, not moved by it. |
| NF-004 | **Evidence is re-runnable**: every transcript in `_limits-evidence.md` names its exact command, its working directory and the toolchain pin, so a future reader can re-measure rather than re-trust. | `project.md` DoD item 2's standard — both halves written down — applied to this story's own measurements. |
| NF-005 | **Prose wrap ≤ 90 columns** in the doc comments, fences ≤ 80. | `_design.md` `## Density budget`: p90 of the existing prose corpus is 81; the fence figure covers ~92% of the constitution's fences unchanged. |
| NF-006 | **No published surface, no semver promise, no changelog line.** `xtask` is `publish = false`. | `_design.md` `## Visibility and stability`; the deployment brief for the project as a whole. |
| NF-007 | **The section stays short enough to be read.** Six bullets, each two to four sentences, in the shape the precedent uses — long enough to name the real instrument, short enough that a contributor reads all six rather than skimming to the code. | `xtask/src/lint_constitution.rs:10-28` is 19 lines for its own limits; that is the working length, and `_design.md` `## Transience policy` gives the reason ("a green check that says ten lines trains people to skip it") one level up. |

## Implementation notes (non-prescriptive)

- **Land the slice-mate first inside the shared context.** `observed-failure-falsification`'s
  recorded output is limit 4's input (AC-008). Writing limit 4 before that run exists means
  writing it from Note 3's forecast, which EC-004 exists because it may be wrong.
- **Write the reconciliation before the section.** AC-010's walk over the delivered checks is the
  thing that tells you whether Note 10's six are still six. Doing it last turns it into a
  rubber stamp on a list that was already written.
- **Copy the shape; do not share the code.** `lint_constitution.rs`'s limits section and its
  `check_harness` text-reading arm are cheap to copy. RS-81-3 scopes a scanner to the directory
  whose behaviour it constrains, and `_storymap.md` explicitly rules out a shared abstraction
  here — one error message answering two questions is the cost.
- **The presence test is a text read, not a rustdoc assertion.** The bin crate's docs are rendered
  by nothing in the gate; only the lib target is documented. Reach both files through
  `workspace_root()` the way `lint_constitution.rs:423-425` reaches the other target's file.
- **Make the failure messages carry the instruction.** AC-007's message should tell the reader
  what to do (delete limit 5 in this change), not merely what failed. That is what makes the
  coupling loud rather than annoying.
- **Keep the evidence companion out of the module docs.** The transcripts belong in
  `_limits-evidence.md`; the module cites it by path. Pasting a transcript into a doc comment
  puts a wrapped terminal capture inside a `-D warnings` render and buries the six bullets.
- **The `compile_fail` disposition is cheap and easy to skip.** One sentence either way, in the
  reconciliation table. If the delivered fence walk does not permit `compile_fail` at all, say
  so and move on; if it does, the limit is real and becomes a seventh bullet.

## Tests and CI (merge gate)

Grounded in the testing brief's merge-gate command list (`_decomposition.md:689-703`), narrowest
to widest.

| Tier | Command / path | Proves |
| ---- | -------------- | ------ |
| Unit — section shape | `cargo test -p xtask` → the checker module's `#[cfg(test)]` block | AC-001, AC-002: the heading is first and composed, in both module files, read as text |
| Unit — per-limit presence | `cargo test -p xtask` → the same block | AC-003, AC-004, AC-005, AC-011: all six limits present in their assigned module, limit 1 carrying its inherently-untestable clause, limit 3 citing the record, limit 6 unhedged in both |
| Unit — named wrong implementations | `cargo test -p xtask` → three negative assertions | AC-009: a moved section, a dropped limit and a hedged limit 6 each fail, with distinguishable messages |
| Unit — the pinning positive test | `cargo test -p xtask` → the checker run over the fixture | AC-007: the real checker reports **no** problem for the `text` fence; closing limit 5 breaks this test loudly |
| Doctest compile | `cargo test --locked -p xtask --doc` | AC-006 half: the fixture's `text` fence is never handed to rustdoc, and the page's registration compiles; also the mechanism the probe measures |
| Checker step | the narrative checker subcommand as `narrative-checker-mounted-with-pinned-path` named it (read, not reworded) | AC-006, AC-012: the fixture is registered, marker-free, inside the path budget, and emits no problem line |
| Existing-checker regression | `cargo xtask lint-constitution` | Note 8: this story did not regress the constitution checker whose shape it copies |
| Story grain | `cargo xtask affected --base main` | `.redkiln/config.yaml:28-40`'s wired story grain selects `xtask` for this change, so the gate that runs on this PR is not compiling nothing |
| Gate — non-terminal bar | `cargo xtask ci --fast` | The merge DoD: `documentation` (`-D warnings` over the rendered harness docs, AC-001/EC-007), `tests`, both narrative steps, and the whole-gate observation of AC-006 |
| Gate — full | `cargo xtask ci` | `project.md` DoD item 6: run before the project is called done; this is the project's terminal story, so it is run here |
| Recorded evidence (not a `#[test]`) | `_limits-evidence.md` in this story's folder | AC-005 (three probe transcripts with commands and toolchain), AC-006 (the `text`-fence walk), AC-008 (the forecast-vs-record reconciliation), AC-010 (the completeness table and the `compile_fail` disposition) |
| Review (prose, not mechanical) | the PR diff | AC-011: no badge, tick, shield or "verified" mark, no second rendered surface, and no sentence anywhere claiming the surface proves a page teaches |

The last two tiers are deliberately not tests. `project.md` DoD item 8 is a review check on prose
by construction, and AC-005/AC-006's evidence is a measurement whose value is that a human read
the output — turning either into an assertion would produce a rule no implementation can fail,
which is the decorative-check defect this project's own design section names.

## Risks and coupling (PR-scoped)

| Risk | Likelihood / impact | Handling inside this PR |
| ---- | ------------------- | ----------------------- |
| The section is written as a list of sentences nobody can fail | Medium / High | AC-009 is the whole answer: three named wrong implementations, each with a test. This is the risk `discover.md` names as the wrong implementation of this very story. |
| The probe is approximated rather than run as wired | Medium / High | EC-002 stops it. The transcript must show the step's own command and `env`, taken from `REQUIRED`, on the pinned toolchain — a hand-typed `cargo test --doc` measures a different step. |
| A dependency's delivered check has already closed limit 5 | Low / Medium | EC-003: delete the limit, record the disposition, and do not weaken the walk. The ledger row carries the note so the change is visible rather than quiet. |
| Limit 4 is written from the forecast because the record was not read | Medium / Medium | AC-008 binds the wording to the record and EC-004 fixes the tie-break. Landing the slice-mate first inside the shared context is the practical mitigation. |
| The honesty sentence acquires a hedge under review pressure | Low / High | AC-011 plus the AC-009 negative test for a hedged limit 6. This is the initiative's top-ranked risk (`initiative.md:500`) and the one this story exists to close. |
| The fixture page drifts into teaching content | Low / Medium | `project.md`'s risk table draws the line: fixture pages yes, corpus no. The page's only job is to carry a deliberately false `text` fence and say so. |
| The `documentation` step fails on an intra-doc link into a doctest-only module | Medium / Low | EC-007, and it is a fast failure. The measured fact — `target/doc/xtask/constitution/` holds only `index.html` — is already in the front half so the implementer does not rediscover it. |
| Coupling to step names owned by another story | Medium / Low | Read them, never reword them. `steps_named` panics on a name absent from `REQUIRED` (`xtask/src/main.rs:799-826`), so a reworded name is a build-time bug rather than a silent one — but it is still a change to another story's decision. |
| The sibling branch `initiative/from-contract-to-published-library` moves a file this story touches | Low / Medium | `project.md`'s cross-branch note: merge forward before the pull request. Nothing this story writes depends on a line number in `spec/SPECIFICATION.md`. |

## Dependencies

**Blocks on** — all four are hard edges, and each supplies something this story states rather
than merely following:

| Story slug | What this story needs from it |
| ---------- | ----------------------------- |
| `fence-discipline-and-allowance-list` | The fence walk itself, and the allowance list whose back door limit 5 describes. Until it exists, "a fence tagged `text` is neither compiled nor flagged" is a prediction about an unwritten function. |
| `narrative-citation-resolution` | One of the delivered checks AC-010's reconciliation walks; its limits (or its written absence of one) belong in the section. |
| `frozen-documentation-must-pin` | The last of the delivered checks, and the one HS-P0023 is sequenced behind. The reconciliation cannot be complete before it lands. |
| `observed-failure-falsification` | The recorded verbatim failure output that limit 4 is quoted from (AC-008), and the slice-mate implemented in the same context. |

Transitively this story also stands on `pinned-narrative-tree-and-compiling-step` (the compile
step the probe measures and the harness the fixture registers in) and
`narrative-checker-mounted-with-pinned-path` (the checker module whose docs carry limit 5), both
of which are already ancestors of the four edges above.

**Unlocks** — no story. This is the project's final story (`_storymap.md` `## Merge order` 4.10),
and what it unblocks is the *project*: HS-P0021 `page-need-discipline`, HS-P0022
`application-author-path` and HS-P0023 `reach-and-adapter-path` inherit a gate whose limits are
stated, and HS-P0024 `comprehension-evidence` inherits the written statement that its friction log
is not substitutable by anything this project built.

## Anchors (progressive disclosure)

Link, do not paste. Everything above is the distilled core; each row below is deferred depth with
the moment to open it.

| Anchor | Why it is load-bearing | When to open | Serves |
| ------ | ---------------------- | ------------ | ------ |
| `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` | Note 10 (`:351-380`) is the **only** enumeration of the six limits and is normative; the testing brief's AC-010 entry (`:658-673`) is what makes limit 1 untestable-by-decision and limits 3 and 5 executable. | Before writing the first bullet of either section — this is the source of truth the section is transcribed from. | AC-003, AC-004, AC-005, AC-006 |
| `.bklg/docs-that-teach/checked-documentation-surface/_design.md` | Signed off 2026-08-17 and **binding**: `## What a user meets first` fixes the section's position, `## Anti-patterns` 9/10/1/11 the forbidden moves, `## Density budget` the real numbers (≤ 32 path, ≤ 40 H1, ≤ 80 fence, ≤ 250 lines), `## Composition` the fixture page's region order. | Before creating the fixture page and before the composition ACs are reviewed. Re-read `## Transience policy` if any part of the section is about to be made conditional. | AC-011, AC-012, AC-001, AC-002 |
| `xtask/src/lint_constitution.rs` | The worked precedent for everything structural: `:10-28` the limits-section shape and `:11-13` the reason it is first; `:423-425` reading the other target's file as text; `:5` the intra-doc link that has never failed a `-D warnings` gate, which is the proof rustdoc cannot police the bin crate; `:828-878` the house test shape. | Immediately before writing the section, and again before writing the presence tests. | AC-001, AC-002, AC-003, AC-009 |
| `xtask/src/constitution.rs` | `:20-36` is the in-repo precedent for a limit documented with **no** test because none is possible, and `:31-36` is the earlier `RUSTDOCFLAGS` probe finding this story re-runs and may contradict. | Before writing limit 1 (to copy the register) and before designing the probe (to reproduce its methodology). | AC-004, AC-005 |
| `xtask/src/main.rs` | `:284-302` the `documentation` step whose `-D warnings` the harness section must survive; `:488-492` the exact `Step` shape the narrative compile step copied, which the probe must invoke as wired; `:143` the `tests` step that runs the new proofs; `:799-826` why step names are read and never reworded. | Before running the probe, and before touching anything that looks like a step name. | AC-005, AC-007, AC-001 |
| `standards/rust/81-checks-that-cannot-be-types.md` | RS-81-1 at `:11` is the rule this whole story implements — *prove the blind spot in the check's own tests, then state it* — and states the failure mode: a reader deleting the real instrument because the grep looks like it covers the ground. | Before deciding whether a limit needs a test, and before writing AC-009's negative tests. | AC-004, AC-007, AC-009 |
| `standards/rust/70-rustdoc-obligations.md` | The house rules for what a module's docs owe a reader, including the limits obligation this section discharges; keeps the six bullets in the repository's register rather than inventing a new one. | While drafting the section's prose. | AC-003, NF-007 |
| `.bklg/docs-that-teach/_discovery/research/02-compiled-prose-tooling-mdbook-test-doc-comment-skeptic-doc-i.md` | The upstream half of limit 3 (cargo#13697 → rustc#67533) and the `compile_fail` candidate seventh limit — error-code assertions being nightly-only. Cited as context, never as the finding. | When writing limit 3's context clause and the `compile_fail` disposition. | AC-005, AC-010 |
| `.bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/spec.md` | The slice-mate that produces limit 4's raw material; its ACs define what the recorded output contains and where it lands. | Before writing limit 4 — after that story's run exists, not before. | AC-008 |
| `.bklg/docs-that-teach/checked-documentation-surface/_storymap.md` | The AC-010 Coverage row states the grain of proof ("static presence plus executed proof where a proof is possible"); `## Merge order` 4.10 states why this story is last; the "Two project-level obligations" paragraph carries DoD items 7 and 8. | At the start, to confirm the story's boundary, and at the end, to confirm the coverage claim. | AC-010, AC-011 |
| `.bklg/docs-that-teach/checked-documentation-surface/project.md` | DoD items 7 and 8 in full (`:258-260`), and the risk table row that ranks green-read-as-teachable as the largest risk this project can cause (`:286`). | Before the final prose scan of the diff. | AC-011, AC-010 |
| `.bklg/docs-that-teach/initiative.md` | `:289-301` the four journeys, including *Survive the second question* that frames every AC here; `:500` the risk ranking; DoD 2 and DoD 5/6, which this story bounds and protects respectively. | When framing or re-checking an acceptance criterion against user intent. | AC-001, AC-006, AC-011 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | The three personas the ACs are written from, with the qualification that none has been directly observed — which is itself why the friction log is the non-substitutable instrument limit 1 names. | Before writing limit 1's "the real instrument is" clause. | AC-004 |
| `rust-toolchain.toml` | The pinned 1.97.1 the probe record must name. A measurement without its toolchain is a claim about nothing (ADR-0029 is why the pin and the MSRV currently coincide). | While recording the probe transcripts. | AC-005 |
| `xtask/Cargo.toml` | `:16-21` records DR-12's standing trade — why no new dependency enters the dev graph for a documentation check. | If any part of this story starts to want a crate. | NF-001 |
| `docs/README.md` | The index the fixture page gets a row in, and `:25-29` the paragraph naming the trees the gate reads — which must stay true. | When adding the fixture page. | AC-012 |
| `.redkiln/config.yaml` | `:28-40` wires `cargo xtask affected --base main` and `cargo xtask ci --fast` as the story and integration grains, so those commands run whether or not anyone types them. | Before claiming the merge DoD. | AC-006, AC-012 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | Checked and **not engaged**: it governs doc comments that discharge a `SPECIFICATION.md` clause, and this story writes `xtask` module docs that discharge none. Recorded so the next reader does not re-open the question. | Only if a limit's wording starts to restate a clause. | AC-003 |

## Clarifications resolved during spec

1. **The twelve AC ids are exactly the front half's twelve, unchanged.** Nothing was added or
   dropped in this pass. The mapping settled here is: static presence AC-001/002/003/009/011,
   executed proof AC-005/006/007/008, the deliberate non-instrument AC-004, the project-level
   obligations AC-010/012.
2. **Which module carries which limit was a real fork, and it is closed.** Limits 1–4 are
   properties of the compile mechanism and live in the harness; limit 5 is a property of the
   fence walk and lives in the checker; limit 6 lives unhedged in both. Restating all six in both
   places was rejected in the front half as two things to update and one that goes stale, and
   AC-003 now tests the negative direction too — a module must **not** restate the limits assigned
   to the other.
3. **"Keyboard reachability" has no literal meaning here, and was not skipped.** It is recorded
   as the obligation that every proof is reachable by one non-interactive command on a clean
   checkout, which is the same property `project.md` DoD items 1 and 6 are made of, and it is
   carried by AC-006 and AC-007 rather than left as an unowned bullet.
4. **The 250-line and 40-character caps are review rules, not gate rules — and that is
   deliberate.** `_design.md` `## Open questions` 2 records why: enforcing them would put this
   project inside HS-P0021's page-need discipline. Only the path-length budget is enforced by the
   checker, because it protects the terminal surface. AC-012 therefore mixes one mechanically
   checked bound with several reviewed ones, and the verification column says which is which.
5. **EC-003 can invalidate an AC, and the ledger must not hide it.** If limit 5 turns out to be
   already closed, AC-006 and AC-007 lose their subject. The resolution is to record the
   disposition in `_limits-evidence.md` and annotate the affected ledger rows with the evidence of
   closure — not to silently satisfy them against a fixture that no longer proves anything.
6. **No anchor was cited that does not exist.** Every path in the anchors table was confirmed
   present in this worktree before it was written down. `xtask/src/narrative.rs` is the story's
   **mount point** and is created by `pinned-narrative-tree-and-compiling-step`; it is named in the
   Integration contract, and deliberately not listed as an anchor, because it does not exist yet
   in this tree.
