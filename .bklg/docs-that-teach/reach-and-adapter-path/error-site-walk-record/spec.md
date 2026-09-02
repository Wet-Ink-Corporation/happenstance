---
item: HS-S0157
stage: spec
created: 2026-08-17T13:16:14.395Z
updated: 2026-08-17T13:16:14.395Z
template_sig: 87bbf1d0
rendered_sig: 1d02b8fe
---

# Spec — The observed error-site walk

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` |
| Project (gold source) | `.bklg/docs-that-teach/reach-and-adapter-path/project.md` |
| This spec | `.bklg/docs-that-teach/reach-and-adapter-path/error-site-walk-record/spec.md` |
| Key brief — architecture + UX (one file) | `.bklg/docs-that-teach/reach-and-adapter-path/_decomposition.md` |
| Key brief — grounding (dependency state) | `.bklg/docs-that-teach/reach-and-adapter-path/_grounding.md` |
| Signed-off design (**binding**) | `.bklg/docs-that-teach/reach-and-adapter-path/_design.md` |
| Story map (slice, one-line, dependencies) | `.bklg/docs-that-teach/reach-and-adapter-path/_storymap.md` |
| This story's discover artifact | `.bklg/docs-that-teach/reach-and-adapter-path/error-site-walk-record/discover.md` |
| Roadmap pointer | none. `RUNBOOK.md` is the library's phase plan and carries no documentation-reach phase; the plan of record for this work is the initiative above. |

**No testing brief exists for this project, deliberately** — `.bklg/docs-that-teach/_decomposition.md`
("Warranted briefs") withholds it because AC-004, AC-005 and AC-009 are *observed walks*, not
automated checks. This story is the reason that omission is defensible: it is the instrument.

## One-line PR slice

A person who did not write the rewrite reproduces the E0034 collision, reaches the reasoning account
keyboard-only from the file and the message alone, and leaves a dated record naming each hop and
their relationship to the work.

## Executive summary

This PR lands **one artefact and one link**: a dated, first-person walk record at
`.bklg/docs-that-teach/reach-and-adapter-path/error-site-walk-record/walk-record.md`, and its
registration in the project item so the project's Definition-of-Done item 4 resolves against a file
rather than a memory.

The delta over what already exists: slice `adapter-error-site` will have *installed* the error-site
explanation and the reasoning account (`store-error-site-rewrite`, `adapter-reasoning-account`).
Nothing yet **observes** that the installed path is walkable, and the two stories that built it
cannot supply that observation — `_storymap.md`'s backbone row **B6** puts the walk in its own slice
precisely so the author is not their own witness. This story is that separate pair of hands.

It changes no code, no public item and no rendered surface. It produces evidence, and the evidence
is falsifiable in the one direction that matters: if the walk fails, the record says so and the
slice it walked is not done.

## Context pack

Everything below is a decision already taken elsewhere and binding here. Read this section and you
can start; the anchors are for depth, not for orientation.

**1. The walk is the deliverable, and an asserted walk is a failed story.** `discover.md` names the
wrong implementation in one line: *"A walk asserted rather than observed, by someone who already
knew where the explanation was."* Project `DR-10` states the reason — *"A walk that happened and was
not written down is indistinguishable from one that did not"* (`project.md`, Derived requirements).
So the unit of work is a **transcript of an event that occurred**, written during or immediately
after it, not a summary composed afterwards from knowledge of how the surface was built.

**2. The walker is a non-author, and the claim stops exactly there.** Project `AC-009` requires *"the
observer is not the author of the rewrite"*. `project.md`'s risk table and `_storymap.md`'s standing
constraints both bound the claim in the same words: *"these walkers are non-authors, not
non-insiders… Evidence that the path exists is not evidence that a stranger finds it — that claim is
HS-P0024's alone (BR-14)."* The record must therefore make **two** statements, and the second is the
one that is easy to omit: who walked it and their relationship to the work, **and** an explicit
disclaimer that this is not non-insider evidence. Overclaiming here silently pre-empts a sibling
project's only instrument.

**3. The cold start is the whole experiment.** Initiative DoD scenario 10 fixes the starting
conditions: *"Reproducing the known trait-resolution failure, the explanation is reached from the
file and message in front of the reader, observed by someone doing exactly that and nothing else."*
"Nothing else" is a constraint on the walker, not a flourish: no opening the backlog, no reading
this spec's siblings, no asking the person who wrote the rewrite, no prior knowledge of where the
reasoning account was filed. Persona 2's measured today-state is the baseline the walk is testing
against — the searched string `TraitVariantBlanketType` exists six times in the workspace and *none*
of them is `store.rs` (`_discovery/distillation/personas-and-journeys.md:167-181`).

**4. The collision is reproduced, not quoted.** `_design.md`'s `## The doctest` binds how the
transcript comes into being for the rewrite story: *"compiling a deliberate double-import at the
pinned toolchain (`rust-toolchain.toml`, `channel = "1.97.1"`) and pasting from that run's stderr"*.
The same rule governs this walk, for a different reason: a walker who quotes the rewrite's own fence
back at itself has verified nothing. They must produce their own `error[E0034]` from their own
`cargo build`, and the record carries the scratch source verbatim and the exact command, so a third
party can re-run it. The scratch **does not get committed** — it does not compile, and the PR
boundary below has no `crates/**` glob for exactly that reason.

**5. Invariant 1 is observed before the hop, not after it.** The UX brief's first
interaction-quality invariant is *"In-place before context-jump… `store.rs`'s module doc must
unblock the reader by itself (state B1); the pointer to the reasoning account is an *offer*, taken
at B2"* (`_decomposition.md`, Interaction-quality invariants). The walk therefore has **two ordered
observations**, and collapsing them destroys the instrument: (i) can the walker get compiling from
`store.rs` alone, before following anything? (ii) *then*, does the offer land in one hop? A record
that only reports (ii) cannot falsify the invariant the design's whole `store.rs` composition order
was built around (`_design.md`, `## Composition`, "(f) is last and (d) is before it").

**6. Keyboard-only, and the record says which keys.** The accessibility floor is not advisory here:
*"Each of the three observed walks (AC-004, AC-005, AC-009) is performed keyboard-only, and the
dated record says so"* (`_decomposition.md`, accessibility floor). UX-AC-09 adds the walker's name
and relationship. The affordances available are rustdoc-native and enumerated in the same brief —
search (`S` or `/`), collapse (`+` / `-`), plain links — plus the two `#[doc(alias)]` keys
`_design.md` establishes (`E0034`, `TraitVariantBlanketType`). Naming the affordance used at each
hop is what distinguishes a keyboard walk from a mouse walk retold in the past tense.

**7. A dead end is a finding, and it is recorded as one.** Interaction invariant 5 — *"No dead
ends… recorded as a failed walk, not quietly re-walked"* — applies to this record verbatim. If the
pointer does not resolve, or the account is not reachable in one hop, the failed walk is the
artefact this PR lands. A second attempt after a fix is a **second dated entry**, never an edit of
the first. This is the one behaviour that makes the record capable of failing, and a record that
cannot fail is decorative.

**8. This story renders no surface and adds no navigation.** `_design.md`'s `## Surfaces` declares
five; this story renders none of them. It observes two — `store-module-error-site` and
`adapter-reasoning-account` — that its two dependencies build. `_storymap.md`'s standing constraints
apply anyway: no widget, no raw HTML, no folded content, self-describing link text, no bare URL into
this repository's own tree. The record is prose and a table.

**9. Nothing in the gate checks any of this, and that is stated rather than papered over.**
`_design.md`'s `## Density budget` names the gap in its own words: *"Nothing in `cargo xtask ci`
verifies keyboard-only reachability or self-describing link text; AC-004, AC-005 and AC-009 are
dated observed walks by design, and the absence of a linter is stated rather than papered over."*
The verification for this story is therefore a **human reading the record against its own stated
protocol**, plus `cargo xtask ci --fast` proving the tree is unchanged in the ways this story
promises it is unchanged.

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `capability` — a user-observable slice; the user is Persona 2 and what they observe is whether the path built by slice `adapter-error-site` actually carries them. |
| **Slice / milestone** | `adapter-error-walk`. **Sole member** — this slice has no slice-mates, by design: `_storymap.md`'s backbone row B6 separates every walk from the surface it walks so the implement stage cannot hand both to one context. |
| **Mount point** | `.bklg/docs-that-teach/reach-and-adapter-path/project.md` — the project item's body, where the record is linked under `## Companions` and where Definition-of-done item 4 (*"The three walks (AC-004, AC-005, AC-009) each have a dated record in this project's artefacts, each naming its walker and their relationship to the work"*) stops being a promise and starts resolving. Body prose only; the CLI owns the frontmatter. |
| **Wires into** | The two surfaces its dependencies land: `crates/happenstance-core/src/store.rs`'s `## Import one flavour, not both` section (elements (b)–(f) of `_design.md`'s `## Composition`) and the reasoning account page in HS-P0020's pinned tree. It consumes them **as a reader**, at their rendered paths, not as source. Also: `rust-toolchain.toml`'s `channel = "1.97.1"` pin, which fixes the diagnostic wording the walk reproduces; and the two `#[doc(alias)]` keys (`E0034`, `TraitVariantBlanketType`) as one of the permitted arrival paths. |
| **Renders surfaces** | **none.** This story renders no surface id from `_design.md`'s `## Surfaces`. It *observes* `store-module-error-site` and `adapter-reasoning-account`. Naming it as a renderer of either would double-count work `store-error-site-rewrite` and `adapter-reasoning-account` own. |
| **Public items** | **none.** `_design.md`'s `## Items` block declares no public item for the whole project; this story adds no code at all. |
| **Conformance rule(s)** | **none, and it is not adapter-observable.** No port, type or behaviour changes, so nothing in `crates/happenstance-testkit/src/suite.rs` can observe this story. Per `CLAUDE.md`, adding a rule here would be decorative — no adapter could fail it. |
| **Clause(s)** | **none amended, none discharged.** No `spec/SPECIFICATION.md` clause changes. The `[FROZEN]` documentation MUSTs are `store-error-site-rewrite`'s obligation (project AC-010); this story runs `cargo xtask spec-trace` only to confirm it did not disturb them. |
| **Advances DoD scenario** | **Initiative DoD 10** — *"The adapter author's error meets its explanation. Reproducing the known trait-resolution failure, the explanation is reached from the file and message in front of the reader, observed by someone doing exactly that and nothing else."* This story is the *entire* observation half of that scenario; the installation half is slice `adapter-error-site`. It also closes project **DoD item 4** for one of its three walks. |

**Delivered mounted, not as an isolated component.** A walk record sitting unlinked in a story folder
is the documentation-medium equivalent of a component that compiles and is never rendered: nothing
reads it, nothing depends on it, and the project's DoD item 4 goes on failing while a green file
sits on disk. The link from `project.md` is the mount, and it is part of this PR.

## PR boundary

`redkiln verify --grain story` reads the first fenced block under this heading and fails on any file
changed outside it.

```
.bklg/docs-that-teach/reach-and-adapter-path/error-site-walk-record/**
.bklg/docs-that-teach/reach-and-adapter-path/project.md
```

**In this PR**

- `walk-record.md` in this story's folder: the dated first-person record — protocol, reproduction
  stanza, hop table, the two ordered observations of interaction invariant 1, the keyboard-only
  statement, the walker's name and relationship, and the bounded-claim disclaimer.
- Any *failed* walk, as its own dated entry in the same file, if the path did not carry the walker.
- The link from `project.md`'s `## Companions` (body prose only — never the YAML frontmatter, which
  the `PreToolUse` hook denies anyway).
- This story's own `_ledger.md` and stage artifacts.

**Explicitly not in this PR**

- **Any file under `crates/`.** The scratch double-import that reproduces `error[E0034]` does not
  compile and is never committed; it lives in the record as verbatim source plus the command that
  ran it. The absence of a `crates/**` glob above is what makes that checkable.
- **Any fix to what the walk finds.** If the walker hits a dead end, a missing fragment or a hop
  that takes two, the finding is recorded and routed to the owning story (`store-error-site-rewrite`
  for the pointer, `adapter-reasoning-account` for the destination) — it is not repaired here. A
  witness who edits the thing they are witnessing has destroyed the observation.
- **The other two walks.** AC-004 and AC-005 belong to slice `reach-walks`
  (`front-door-walk-record`, `second-question-walk-records`), which waits on HS-P0022. This slice
  carries no HS-P0022 edge and must not acquire one.
- **The non-insider claim.** HS-P0024 `comprehension-evidence` owns the friction log and the only
  evidence that supports it (BR-14).
- **Any change to `spec/SPECIFICATION.md`, `standards/`, `xtask/` or `.kb/`.**

**Merge DoD one-liner.** Merged when `walk-record.md` carries a dated, keyboard-only, first-person
record whose walker is named and is not the author of either dependency, whose reproduction stanza
re-runs, whose hop table ends at the reasoning account in one hop *after* an in-place unblock, and
whose link from `project.md` makes project DoD item 4 resolve — with `cargo xtask ci --fast` green
and no file under `crates/` touched.

## Behavior and interfaces

The "interface" in this medium is the record's own shape: the fields a reader needs in order to
re-run the walk and disagree with it. The table below is the contract.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The record exists at one known path, dated** | `walk-record.md` in this story's folder, carrying an ISO date for the walk itself (not the commit date). One file; a failed walk and its later re-walk are two dated entries inside it, newest last. | `project.md` DoD item 4; `.bklg/docs-that-teach/reach-and-adapter-path/error-site-walk-record/` |
| **The walker is named, with their relationship to the work** | Name, and one sentence: which of `store-error-site-rewrite` / `adapter-reasoning-account` they did **not** author (both), and what they knew about the surface before starting. | `project.md` AC-009; `_decomposition.md` UX-AC-09 |
| **The claim is bounded in the record's own words** | An explicit line: this is evidence that the path exists, not that a stranger finds it; the walker is an insider. No sentence in the record may imply otherwise. | `project.md` risk table, "AC-005 and AC-004 walkers are non-authors, not non-insiders"; `_storymap.md` standing constraints; `initiative.md` BR-14 |
| **The collision is reproduced first-hand** | A reproduction stanza: the verbatim scratch source that imports both `EventStore` and `SendEventStore` and calls `read`, the exact `cargo` invocation, the toolchain string as `rustc -V` printed it, and the resulting `error[E0034]` stderr pasted whole. Not quoted from `store.rs`. | `rust-toolchain.toml` (`channel = "1.97.1"`); `_design.md` `## The doctest`; `crates/happenstance-core/src/store.rs:31-45` |
| **The scratch is never committed** | The record's stanza is the only trace. Enforced by the PR boundary carrying no `crates/**` glob. | this spec, `## PR boundary` |
| **The cold start is stated as a protocol, before the walk** | What the walker was permitted to have open at t=0 — the failing build's output and `store.rs` — and what they were forbidden: the backlog, this spec, the sibling stories, the author. | `initiative.md` DoD 10; `discover.md`, "The wrong implementation" |
| **Observation A: the in-place unblock, before any hop** | The record answers, in order and before any hop is taken: did `store.rs` alone say which call was ambiguous and how to resolve it? Quote the sentence that did it (or record that none did). Only then does the walk continue. | `_decomposition.md` invariant 1 and UX-AC-05; `_design.md` `## Composition`, `store-module-error-site` rows (c)/(d)/(f) |
| **Observation B: the hop, counted** | The reasoning account is reached in **one** hop from `store.rs`. Two hops is a finding, not a rounding error. | `project.md` AC-007; `_design.md` `## What it costs a caller`, "One hop, never two" |
| **Every hop is a row: what was on screen, what was pressed, where it landed** | A hop table — ordinal, surface, the affordance used (link / rustdoc search `S` or `/` / `#[doc(alias)]` key / fragment), and the destination *passage*, not just the page. | `_decomposition.md` accessibility floor and UX-AC-08's passage-not-page rule; `_design.md` `## Pattern decision`, `#[doc(alias)]` |
| **Keyboard-only, and it says which keys** | One explicit statement that no pointing device was used, plus the keys named per hop. A record that says "keyboard-only" and names no key has asserted the floor rather than observed it. | `_decomposition.md`, accessibility floor ("performed keyboard-only, and the dated record says so"); UX-AC-09 |
| **The search path is exercised, because Persona 2 searches before they read** | At least one hop or one recorded attempt uses a search string the reader would actually type — `E0034` or `TraitVariantBlanketType` — and the record states what it returned. | `_decomposition.md` invariant 6; `_discovery/distillation/personas-and-journeys.md:167-181` |
| **Arrival is verified against the destination, not assumed** | On landing, the record names the account's stated answered-need and the position marker of the section landed on ("n of 6"), which is how a mid-sequence arrival is proved to be legible. | `_design.md` `## Composition`, `adapter-reasoning-account` regions 1/4; `_decomposition.md` invariant 8 |
| **The `MemoryEventStore` caveat is checked on arrival** | The walker records whether the account says, where `memory.rs` is first sequenced, that `MemoryEventStore` is the conformance oracle and reference implementation and **not** an adapter. Absent, that is a finding against `adapter-reasoning-account`. | `_design.md` `## Composition` region 3; `project.md` risk table, final row; `crates/happenstance-core/src/memory.rs` |
| **A dead end is recorded as a failed walk** | If any hop fails, the entry is closed as a failure with the finding routed by story slug. No quiet re-walk; a later successful attempt is a new dated entry that cites the failed one. | `_decomposition.md` invariant 5; `_design.md` `## The states the API must express`, "Refused" |
| **The record is mounted** | `project.md`'s `## Companions` links it; the link text names the record and the walk it carries — no "here", no bare URL. | `project.md` `## Companions`; `_decomposition.md` accessibility floor, self-describing link text |
| **Nothing else in the tree moved** | `cargo xtask ci --fast` green (project DoD item 1) and `cargo xtask spec-trace` clean, run to prove this story disturbed neither — not to prove anything about the record. | `CLAUDE.md`, Commands; `project.md` DoD items 1 and 2 |

**Acceptance-criteria ids this story enumerates** (the table is authored in the second pass, against
exactly this set): `AC-001` reproduction · `AC-002` walker identity and bounded claim · `AC-003`
cold start · `AC-004` hop table ending at the account in one hop · `AC-005` keyboard-only with keys
named · `AC-006` in-place unblock observed *before* the hop · `AC-007` dated record mounted so
project DoD item 4 resolves · `AC-008` a dead end recorded as a failed walk, never re-walked
quietly.

## Data and migrations

**N/A — no schema, no store, no persisted state.** This story adds no code, no type, no event, no
projection and no manifest entry; `_design.md`'s `## Items` block already declares that the whole
project changes no public item, and this story is the one member of it that touches no `crates/`
file at all.

The nearest thing to "data" here is the record's own shape, and it is deliberately **not** a schema:
no frontmatter beyond what the CLI owns on the items, no YAML block, no id namespace. It is prose
plus one hop table, so that a reader who cannot run `cargo` can still read it and disagree. The
project's one build-time data structure — the pointer register, `_design.md`'s `## Transience
policy` — belongs to `pointer-policy-and-inventory`, and this story neither adds a row to it nor
reads it.

## Acceptance criteria

Nine criteria. Eight are the set the first pass enumerated; `AC-009` was added in this pass and the
reason is under `## Clarifications resolved during spec`. Each is framed as Persona 2's goal
crossing the whole path — the compiler diagnostic, the file they have open, the account one hop
away — because a criterion framed as a capability ("the record has a hop table") is satisfied by a
walk nobody took.

Every row traces to project **AC-009** (`project.md`, Acceptance criteria) and, through it, to
initiative DoD scenario 10. "Verification" names a check someone other than the author can run.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | GIVEN Persona 2 has just hit the two-flavour collision and searches the string rustc printed, WHEN the walker reproduces it themselves at the pinned toolchain rather than quoting the rewrite back at itself, THEN `walk-record.md` carries a reproduction stanza with the verbatim scratch source, the exact cargo invocation, `rustc -V` as printed, and the whole `error[E0034]` stderr including both `= note:` candidate lines and `TraitVariantBlanketType` — and no file under `crates/` is added by this PR. | Re-run: recreate the recorded scratch outside the repo, run the recorded command at `rust-toolchain.toml`'s `channel = "1.97.1"`, and diff the stderr against the pasted block — the error code, both `= note:` lines and the type name must match. Mechanical: `rg -F 'TraitVariantBlanketType' .bklg/docs-that-teach/reach-and-adapter-path/error-site-walk-record/walk-record.md` returns a hit; `rg -cF '= note:'` returns at least 2; `git diff --name-only main...HEAD -- crates/` is empty. |
| AC-002 | GIVEN the project may not stand in for HS-P0024's non-insider instrument, WHEN the record makes its claim, THEN it names the walker, states in one sentence that they authored neither `store-error-site-rewrite` nor `adapter-reasoning-account` and what they knew of the surface beforehand, and carries an explicit bounded-claim line saying this is evidence that the path exists and not that a stranger finds it — with no sentence anywhere in the record implying otherwise. | Authorship cross-check: `git log --format='%an %ae' -- crates/happenstance-core/src/store.rs` and the account page's path over this initiative's commits must not list the named walker. Human read of the whole record for any sentence that generalises to a stranger, an outsider, a new user or a first-time reader; project `AC-009` and the `project.md` risk-table row *AC-005 and AC-004 walkers are non-authors, not non-insiders* are the bar. |
| AC-003 | GIVEN initiative DoD scenario 10 fixes the starting conditions at the file and the message and nothing else, WHEN the walk begins, THEN the record states the cold-start protocol before its first hop: what the walker had open at t=0 (their own failing build output and `crates/happenstance-core/src/store.rs`) and what was forbidden (the backlog, this spec, the sibling stories, the author, and prior knowledge of where the account was filed), with any contamination disclosed rather than omitted. | Document order: the protocol stanza precedes the reproduction stanza, which precedes Observation A, which precedes the hop table — checkable from the heading outline alone. Human read against `initiative.md` DoD 10 and `discover.md`'s named wrong implementation (*a walk asserted rather than observed, by someone who already knew where the explanation was*). |
| AC-004 | GIVEN Persona 2 wants the reasoning behind the two-flavour split and not only the fix, WHEN they take the offer at the end of `## Import one flavour, not both`, THEN the record's hop table reaches the adapter reasoning account in exactly one hop and proves arrival at the passage: it names the account's stated answered-need, the position marker of the section landed on (`n of 6`), and whether the caveat that `MemoryEventStore` is the conformance oracle and reference implementation and not an adapter was present where `memory.rs` is first sequenced — two hops, or an arrival that cannot be located in the sequence, is recorded as a finding. | Count the rows whose source is `store.rs`: exactly one, and its destination is the account. The quoted answered-need and position marker must appear verbatim on the rendered account page (`rg -F` against its path in HS-P0020's pinned tree). Compare the caveat's presence against `_design.md` `## Composition`, `adapter-reasoning-account` regions 1, 3 and 4. |
| AC-005 | GIVEN a keyboard-only reader who searches before they read, WHEN each hop is taken, THEN the record states that no pointing device was used and names the affordance and the keys per hop (plain link, rustdoc search `S` or `/`, a `#[doc(alias)]` key, or a fragment), and at least one hop or recorded attempt uses a string the reader would really type — `E0034` or `TraitVariantBlanketType` — with what it returned written down. | No hop row has an empty affordance cell; at least one row or attempt names a search string and its result. Human read against `_decomposition.md`'s accessibility floor (*each of the three observed walks is performed keyboard-only, and the dated record says so*) and UX-AC-09. A record that says keyboard-only and names no key fails this row. |
| AC-006 | GIVEN invariant 1, that getting unstuck must never require leaving the surface the reader is on, WHEN the walker reaches `store.rs` and before any hop is taken, THEN the record answers first whether `store.rs` alone named which call was ambiguous and how to resolve it, quoting the sentence that did it verbatim from the file (or recording that none did) — and that observation appears in document order ahead of the hop table. | `rg -F '<the quoted sentence>' crates/happenstance-core/src/store.rs` returns a hit on the merged tree, proving the quote is the file's and not a paraphrase. Heading order puts Observation A above the hop table. Falsification of the invariant behind it, per `_decomposition.md` invariant 1: with the account hypothetically deleted, the quoted sentence still unblocks the reader. |
| AC-007 | GIVEN that a walk which was not written down is indistinguishable from one that did not happen, WHEN the PR merges, THEN `walk-record.md` exists in this story's folder carrying the ISO date of the walk itself, and `project.md`'s `## Companions` links it with self-describing link text naming the record and the walk it carries, so project Definition-of-done item 4 resolves against a file — with nothing else in `project.md` changed. | `test -f .bklg/docs-that-teach/reach-and-adapter-path/error-site-walk-record/walk-record.md`; the date is an ISO date and is stated to be the walk's, not the commit's. `rg -n 'error-site-walk-record/walk-record.md' .bklg/docs-that-teach/reach-and-adapter-path/project.md` returns a hit inside `## Companions`. `git diff main...HEAD -- .bklg/docs-that-teach/reach-and-adapter-path/project.md` shows only the added bullet, and no frontmatter line. |
| AC-008 | GIVEN invariant 5 and `_design.md`'s Refused state, WHEN a hop fails, resolves nowhere, or takes two, THEN that entry is closed in the record as a failed walk with the finding routed by owning story slug (`store-error-site-rewrite` for the pointer, `adapter-reasoning-account` for the destination) and nothing repaired in this PR, and any later attempt is a new dated entry citing the failed one — the file's history shows entries appended, never a closed entry edited. | The record's protocol stanza declares this rule before the first entry, so it binds whether or not a failure occurred. `git log -p -- .../walk-record.md` shows additions only below the last closed entry; any hunk that modifies a closed entry's lines fails this row. If a failure occurred, its finding names an owning story slug and `git diff --name-only main...HEAD` shows no file of that story's touched in this PR. |
| AC-009 | GIVEN the standing composition constraints every story in this project inherits, WHEN the record and its mount are rendered by GitHub's Markdown renderer, THEN the record is composed prose plus one hop table with a header row and its named columns, and neither it nor the `project.md` link introduces raw HTML, an inline `style=` attribute, a folded or tabbed element, a *See also* / *Next steps* / *Further reading* block, a fewer-than-three-row table used as a navigation device, link text reading *here* / *this* / *docs*, or a bare URL into this repository's own tree. | Mechanical: `rg -n '<details\|<table\|<div\|style='` over both changed markdown files returns nothing, as does `rg -in '^#+ *(see also\|next steps\|further reading)'`. Human read against `_design.md` `## Anti-patterns` items 3, 5, 6, 9 and 10, `_storymap.md`'s standing constraints, and `_design.md` `## Density budget` (visible link text at least 3 words, a noun phrase naming the destination). Rendered check: open both files' GitHub preview and confirm the hop table renders as a table with every column headed. |

**Coverage of the traced project AC.** Project `AC-009` has three clauses and each has rows:
*someone reproduces the E0034 collision* → AC-001; *reaches the explanation from the file and the
message in front of them and nothing else* → AC-003, AC-004, AC-005, AC-006; *dated record; the
observer is not the author of the rewrite* → AC-002, AC-007. AC-008 and AC-009 are the two rows that
make the record capable of failing and capable of being read — without them the criterion is
satisfiable by a green file nobody can falsify.

## Interaction quality

The blocking invariants, in two families. **Every one is carried by an `AC-###` row in the table
above** — this section says which row carries it and how it is falsified, and adds no requirement of
its own. A bullet here would never reach the ledger and would never be gated.

### State invariants

| Invariant | Carried by | How it is verified, and what falsifies it |
| --- | --- | --- |
| **In-place before context-jump** (`_decomposition.md` invariant 1; UX-AC-05) | **AC-006**, ordered before **AC-004** | The record's Observation A quotes the `store.rs` sentence that unblocked the reader *before* any hop is taken. Falsified by a record whose first substantive observation is the hop — that record cannot tell you whether the page works without the link, which is the whole shape `_design.md`'s composition order ((f) last, (d) before it) was built to protect. |
| **Non-occlusion — the mount displaces nothing** (invariant 2) | **AC-007** | `git diff` of `project.md` shows one added bullet inside `## Companions` and no other changed line. Falsified by any pre-existing line of `project.md` moving for reasons other than the addition, or by any frontmatter line changing at all. |
| **Preserved selection and copy fidelity** (invariant 7) | **AC-001** | The reproduction stanza is contiguous plain text inside a fence: no line numbers, no gutter, no rendered-only markup, no reflowed long line. Falsified by pasting the rendered stanza into a diff against real rustc output and finding non-compiler characters — and note `_design.md`'s F4: rustc's longest line is 109 characters and *must* be allowed to scroll horizontally rather than be wrapped to fit. |
| **Position preserved on mid-sequence arrival** (invariant 8; `_design.md`'s staged-disclosure mitigation) | **AC-004** | The record names the `n of 6` position marker of the section it landed on, proving position was learned from the heading rather than from scrolling up. Falsified by an arrival the record can only describe as *the account*, with no locatable passage. |
| **Reversibility — every hop is walk-backable** (invariant 4) | **AC-004**, with **AC-008** | The record names the destination passage and what that passage assumes was read first, so the hop can be retraced without browser history. Falsified by arriving somewhere and being unable to name the page that should have preceded it — which is recorded as a finding, not smoothed over. |
| **No dead ends; a refusal is recorded, not re-walked** (invariant 5; `_design.md`'s Refused state) | **AC-008** | Append-only history plus a routed finding. Falsified by a `git log -p` hunk that edits a closed entry, or by a repair to a dependency's file landing in this PR. |
| **Keyboard reachability, observed not asserted** (accessibility floor; UX-AC-09) | **AC-005** | Affordance and keys named per hop, plus one real search string and its result. Falsified by the phrase *keyboard-only* with no key named anywhere in the record. |

### Composition invariants

Taken from `_design.md`, which is binding. This story renders **no** surface from its `## Surfaces`
block — it renders one new markdown artefact and one bullet on an existing item — so the invariants
that apply are the project-wide ones every story inherits, and they are collected in **AC-009**
except where noted.

| Invariant | Carried by | The real number or named anti-pattern |
| --- | --- | --- |
| **Presentation exists at all** | **AC-009** | The hop table is a rendered table with a header row and four named columns (ordinal · surface · affordance and keys · destination passage), not a run-on paragraph and not bare markup. A record whose hops are prose sentences satisfies every other row here and is unreadable as evidence. |
| **Composition and placement** | **AC-003**, **AC-006**, **AC-009** | Fixed document order inside each dated entry: protocol → reproduction → Observation A → hop table → arrival check → findings → claims and bounds. The order *is* the instrument; `_decomposition.md` invariant 1 is only observable if Observation A precedes the hops. |
| **Transience** | **AC-009** | Everything in the record is **persistent chrome**. `_design.md` `## Transience policy` installs zero revealed-on-demand affordances project-wide, and anti-pattern 9 forbids a `<details>`, a tab strip, an accordion or any `[+]` in content this project adds. A folded reproduction stanza is folded evidence. |
| **Density budget, with numbers** | **AC-009**, with **AC-001** | Visible link text **at least 3 words**, a noun phrase naming the destination (`_design.md` `## Density budget`, minimum-legible-size rules); the hop table has **all four columns filled on every row**; the reproduction stanza is **never trimmed to fit** — `_design.md`'s F3 resolved copy fidelity over the density budget at the design gate, and that resolution governs this record's transcript exactly as it governs `store.rs`'s fence. |
| **Hierarchy** | **AC-009** | The dated entry heading is primary; the hop table is secondary; the bounded-claim line is *not* recessive and may not be a footnote — it is the sentence that stops this record pre-empting HS-P0024, so it sits in the body of the entry. Heading ladder unskipped (`##` then `###`), per the accessibility floor. |
| **Named anti-patterns** | **AC-009**, with **AC-002** for the last one | `_design.md` items **3** (no *here* / *this* / *docs* / bare URL), **5** (no *See also* / *Next steps* / *Further reading* block), **6** (no fewer-than-three-row table used as a navigation device), **9** (nothing folded or tabbed), **10** (no raw HTML, no inline `style=`). And the project's own: a record that reads as evidence a *stranger* could find the path is the overclaim `project.md`'s risk table forbids, caught by **AC-002**. |

**Why an unstyled render would not pass.** Every mechanical assertion in AC-001 through AC-008 —
every `rg`, every `git diff`, every existence check — passes on a record that is one unpunctuated
paragraph with the right strings in it. AC-009 is what fails that record: the hop table must *be* a
table, the entry order must be the order, and nothing may be folded away. In this medium the
equivalent of an unstyled component is a wall of prose that contains all the evidence and surfaces
none of it.

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | The account is not reachable because slice `adapter-error-site` has not merged, or HS-P0020's pinned tree does not yet host the account at a routable path (`_grounding.md`, "Sibling-project dependency state"; `_design.md` `## Surfaces`, `route: TBD`). | **Halt; do not walk.** This is a blocked story, not a failed walk — walking a surface that does not exist yet produces a record that measures scheduling. Record the block against the dependency slug and stop. Simulating the destination is the wrong implementation `discover.md` names. |
| **EC-002** | The pointer resolves nowhere, or the account is two hops away, or a hop lands on a page rather than a passage. | Close the entry as a **failed walk** (AC-008). Route the finding by slug — `store-error-site-rewrite` owns the pointer, `adapter-reasoning-account` owns the destination — and change nothing in either. A later attempt after their fix is a new dated entry citing this one. |
| **EC-003** | The scratch does not produce `error[E0034]`, or rustc's wording differs from the fence `store-error-site-rewrite` installed. | Record what rustc actually printed, with `rustc -V`, and treat the divergence as a finding against the rewrite — *not* as a reason to paste the rewrite's fence into the record. If the code itself is no longer `E0034`, that is larger than this story: the constitution's compiled `rust,compile_fail,E0034` fences (`standards/rust/20-two-flavour-ports.md`) would be failing too, and the finding escalates to the project. |
| **EC-004** | No eligible walker exists — everyone available authored one of the two dependencies. | **Halt and escalate to the project.** Do not let an author witness their own work; `_storymap.md`'s backbone row B6 exists to make that impossible and this is the condition it anticipates. The project's DoD item 4 stays open rather than being satisfied by a compromised record. |
| **EC-005** | The scratch crate, or any other file, lands under `crates/`. | PR-boundary violation. `redkiln verify --grain story` reads the fenced block under `## PR boundary` and fails; the fix is to delete the file and keep the source in the record's stanza, never to widen the boundary. |
| **EC-006** | The walker turns out to have seen the reasoning account, or been told where it is, before t=0. | Disclose it in the protocol stanza in the walker's own words and state what it likely bought them. A disclosed contamination is a weaker record; an undisclosed one is a false one. If the contamination is total — they were shown the destination — the entry is closed as void and a different walker is recruited. |
| **EC-007** | `cargo xtask ci --fast` or `cargo xtask spec-trace` fails on the merged result. | Not this story's finding to fix unless this story caused it — this PR touches no code. Investigate, and if the cause is the dependency slice, route it there under EC-002's rule rather than repairing it here. |

## Non-functional

| id | Requirement | Why, and how it is met |
| --- | --- | --- |
| **NF-001** | The record is readable and falsifiable **without a Rust toolchain**. | It is prose plus one table. A reviewer who cannot run `cargo` can still check hop counts, keys named, the bounded-claim line and the mount. The reproduction stanza is the one part that rewards a toolchain, and it carries everything needed to re-run rather than assuming the reader has the tree. |
| **NF-002** | The record must survive the project's closeout as the **only** record of this observation. | `_design.md` states the perceptual review is a standing skip and that its written resolution is the only record there will ever be; the same is true here — no screenshot harness runs and no CI step re-observes. So the record is written to be read by someone with none of this context, and names its own sources by path. |
| **NF-003** | This story adds **zero** gate time. | No code, no test, no CI step. `cargo xtask ci --fast` runs exactly what it ran before; the two gate commands in `## Tests and CI` are run to prove the tree is undisturbed, not to check the record. |
| **NF-004** | A future reader can tell **drift from regression**. | The reproduction stanza pins `rustc -V` as printed and `rust-toolchain.toml`'s channel, so a later mismatch is attributable: same toolchain and different output is a regression; a different toolchain is drift, which `_design.md`'s note that diagnostic phrasing moves already covers. |
| **NF-005** | The record names exactly one person, and names them because the criterion requires it. | Project `AC-009` and DR-10 require the walker and their relationship to the work. No other individual is named, and no judgement is recorded about anyone's authoring — findings are routed to **story slugs**, not to people. |

## Implementation notes (non-prescriptive)

Shape, not prescription. The implementer may reach the same evidence another way; what is fixed is
the acceptance table above.

- **Recruit before you prepare.** EC-004 is discovered at the end if you build the harness first.
  Confirm a walker who authored neither dependency, then freeze the tree at a commit sha and record
  it in the entry — a walk against an unspecified working tree is not re-runnable.
- **Hand the walker exactly two things**: their own failing build output, and the path
  `crates/happenstance-core/src/store.rs`. Nothing else, per AC-003. In practice that means not
  sending them this spec, and not answering questions during the walk. Save the questions; they are
  findings.
- **The scratch lives outside the repository.** A `cargo new` in a temporary directory with a path
  dependency on `crates/happenstance-core`, or any equivalent that keeps the failing source out of
  the worktree, satisfies the PR boundary by construction rather than by remembering to delete
  something. Paste the source into the record; do not commit it.
- **Write during, not after.** The hop table is a live log. Reconstructed hop tables are how "I
  pressed `S` and searched for `E0034`" becomes "I found it", which is the assertion `discover.md`
  rejects.
- **A plausible entry skeleton**, offered because the ordering is load-bearing and is easy to
  reshuffle into uselessness: `### Walk of <ISO date>` → *Walker and relationship* → *Cold-start
  protocol* → *Reproduction* → *Observation A: in place, before any hop* → *Hops* (the table) →
  *Arrival check* → *Findings, routed by slug* → *What this record does and does not claim*.
- **Mount it in the same commit that writes it.** A record that lands unlinked is the unrendered
  component this spec's integration contract warns about; the `## Companions` bullet is part of the
  deliverable, not a follow-up.
- **If the walk succeeds cleanly, resist enriching it.** The temptation at the end is to add what the
  walker "would have" hit. The record is a transcript of one event.

## Tests and CI (merge gate)

No testing brief exists for this project and that omission is deliberate
(`.bklg/docs-that-teach/_decomposition.md`, "Warranted briefs"): AC-004, AC-005 and AC-009 are
**observed walks**, and `_design.md` `## Density budget` states in its own words that nothing in
`cargo xtask ci` verifies keyboard-only reachability or self-describing link text. The gate for this
story is therefore three tiers of real check, two gate commands that prove only non-disturbance, and
one honest gap.

| tier | command / path | proves |
| --- | --- | --- |
| **Observation** (the instrument) | `.bklg/docs-that-teach/reach-and-adapter-path/error-site-walk-record/walk-record.md`, read against its own protocol stanza by someone who is not the walker | AC-002, AC-003, AC-004, AC-005, AC-006 — that the walk happened, cold, keyboard-only, in place before the hop, and landed on a locatable passage. The only tier that can falsify the path itself. |
| **Reproduction** (re-runnable) | recreate the recorded scratch outside the repo; `cargo build`; compare stderr against the pasted stanza; `rustc -V` against `rust-toolchain.toml` | AC-001 — the collision is first-hand and current, not quoted from `store.rs`. The one check in this story a machine can settle outright. |
| **Mechanical assertion** | `rg -F 'TraitVariantBlanketType' <walk-record.md>` · `rg -cF '= note:' <walk-record.md>` · `rg -F '<the quoted store.rs sentence>' crates/happenstance-core/src/store.rs` · `rg -n 'error-site-walk-record/walk-record.md' .bklg/docs-that-teach/reach-and-adapter-path/project.md` · `rg -n '<details\|<table\|<div\|style='` over both changed files | AC-001, AC-006, AC-007, AC-009 — the strings that must be present are present, the quote is the file's own, the mount exists, and no forbidden composition was introduced. |
| **History** | `git log -p -- .../walk-record.md` · `git diff --name-only main...HEAD` · `git diff main...HEAD -- .../project.md` | AC-007, AC-008 — entries are appended and never edited, no file under `crates/` moved, and `project.md` changed by exactly one bullet with no frontmatter touched. |
| **Repo gate** | `cargo xtask ci --fast` (project DoD item 1) and `cargo xtask spec-trace` (DoD item 2) | That this story disturbed nothing. It proves nothing *about* the record — stated explicitly so a green gate is never mistaken for a walked path. |
| **Backlog gate** | `redkiln verify --grain story` against `## PR boundary` and `_ledger.md`; `redkiln validate --kb && redkiln doctor` | Every `AC-###` in this spec has a ledger row carrying real evidence, and no file outside the boundary changed. |
| **Named gap** | *none exists* | Keyboard-only reachability and self-describing link text have **no** automated check in this repository (`_design.md` `## Density budget`, gap 2). That is why this story exists as an observation, and why the Observation tier is the gate and not a formality. |

## Risks and coupling (PR-scoped)

| Risk / coupling | Note |
| --- | --- |
| **The walker pool is small and entirely insider** | `project.md`'s risk table is explicit: everyone available is an insider, and insider knowledge routes a logger around rough spots. The mitigation is the cold-start protocol (AC-003) plus the bounded-claim line (AC-002), not a pretence. The residual risk is that this walk is *easier* than a stranger's would be — which is exactly why the claim stops at "the path exists". |
| **Overclaiming pre-empts HS-P0024** | The cheapest failure mode here is one enthusiastic sentence. `comprehension-evidence` (BR-14) owns the only method that supports a non-insider claim; a record that implies one has spent a sibling project's evidence in advance and cannot un-spend it. AC-002 is the guard and it is a human read, because no `rg` catches a tone. |
| **The witness is tempted to fix what they find** | Half of a good walk is noticing; the other half is not touching. A repair in this PR destroys the observation and silently changes what the record describes. The PR boundary carries no `crates/**` glob, so the temptation is at least visible as a boundary violation. |
| **Dependency slice not merged** | This story sits behind `adapter-reasoning-account` and `store-error-site-rewrite`, which sit behind HS-P0020's clause-id pin and its pinned tree (`_storymap.md`, Merge order step 2). If either is outstanding, EC-001 applies and this story blocks. It carries **no** HS-P0022 edge and must not acquire one — that is the parallelism `_storymap.md` says the map exists to express. |
| **Toolchain drift between the rewrite and the walk** | `store-error-site-rewrite` pastes rustc 1.97.1's output; this story reproduces it later. If the pin moves in between, the two diverge legitimately and EC-003 governs. NF-004's `rustc -V` line is what makes the divergence attributable rather than alarming. |
| **The authorship check is weaker than it looks** | `git log` attribution is fragile under squashes, co-authored trailers and pair work. It is a cross-check, not the criterion; the criterion is the walker's own stated relationship to the work (AC-002), and a squashed history means the reviewer asks rather than infers. |
| **`project.md` is a live item and the CLI owns its frontmatter** | The mount is body prose in `## Companions` only. A `PreToolUse` hook denies frontmatter edits (`CLAUDE.md`, Where the work lives), so an accidental attempt is not a silent corruption — but it is a wasted cycle. |
| **A clean walk reads as a formality** | If everything works first time the record looks like a rubber stamp, and the next reader may treat the next walk as one. AC-008's append-only failure protocol is declared *before* the first entry precisely so the record's capacity to fail is visible even in the entry where it did not fail. |

## Dependencies

**Blocks on** — both are `_storymap.md`'s slice `adapter-error-site`, and both must have merged:

- **`store-error-site-rewrite`** — supplies the surface the walk starts on: rustc's real `error[E0034]`
  block with its `= note:` candidate lines and `TraitVariantBlanketType`, the plain-words naming of
  the ambiguous call, the in-place fix ahead of any hop, and the one guarded pointer. Without it
  AC-001's search string is absent from the file (it is absent today) and AC-006 has nothing to
  quote.
- **`adapter-reasoning-account`** — supplies the destination as a *registered page* in HS-P0020's
  pinned tree, with its answered-need line, its numbered reading order, the `n of 6` position markers
  and the `MemoryEventStore`-is-not-an-adapter caveat. Without it AC-004 has nowhere to land and
  EC-001 applies.

**Unlocks** — no story in this project takes an edge from this one. What it releases is evidence:

- **Project Definition-of-done item 4** for one of its three walks (`project.md`). The other two are
  `front-door-walk-record` and `second-question-walk-records` in slice `reach-walks`, which are
  siblings, not dependents, and wait on HS-P0022 rather than on this.
- **Initiative DoD scenario 10**, whose observation half is entirely this story.
- **HS-P0024 `comprehension-evidence`**, which needs the assembled surface before a friction log
  measures teaching rather than assembly (`project.md`, Dependencies). This record tells it whether
  the adapter path was assembled; it deliberately tells it nothing about whether a stranger walks it.

## Anchors (progressive disclosure)

Linked, not pasted. The `## Context pack` above is sufficient to start; open these at the moment
named, and only then.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/reach-and-adapter-path/_design.md` | The signed-off design, binding. `## Composition` fixes the order of `store.rs`'s elements (c)→(d)→(f) and the reasoning account's regions 1–5 — which is what makes Observation A separable from the hop, and what the arrival check compares against. `## Anti-patterns` and `## Density budget` are AC-009's actual content, including the F3 resolution that copy fidelity beats the budget. | Before writing the entry skeleton (AC-006's ordering, AC-009's rules), and again at the arrival check. | AC-004, AC-006, AC-009 |
| `.bklg/docs-that-teach/reach-and-adapter-path/_decomposition.md` | The architecture + UX brief. Its nine interaction-quality invariants each name their own falsification, and its accessibility floor is where *keyboard-only, and the record says so* is written; UX-AC-05, UX-AC-06 and UX-AC-09 are the UX-grain statements of AC-006, AC-001 and AC-005. | Before the walk, to brief the walker on what is being observed; and when writing the record's interaction rows. | AC-005, AC-006, AC-008 |
| `.bklg/docs-that-teach/reach-and-adapter-path/project.md` | Gold source of project AC-009 and DR-10, of Definition-of-done item 4 (the mount's reason for existing), and of the risk-table row that bounds the claim to non-authors rather than non-insiders. Also the `## Companions` list this PR appends to. | Before writing the walker-identity and bounded-claim stanzas, and when mounting. | AC-002, AC-007 |
| `.bklg/docs-that-teach/reach-and-adapter-path/_storymap.md` | Backbone row **B6** is why this story is not a checkbox inside the slice it observes — *putting a walk in the same slice as the surface it walks would make the author their own witness*. Its standing constraints are the composition rules AC-009 collects. | At recruitment (EC-004), and when checking AC-009's constraint list is complete. | AC-002, AC-009 |
| `.bklg/docs-that-teach/initiative.md` | DoD scenario 10 fixes the cold-start conditions verbatim (*observed by someone doing exactly that and nothing else*); BR-14 is the scoping discipline the bounded-claim line obeys, and BR-15 states the gap the walk is testing. | When writing the cold-start protocol stanza, and when wording the claim. | AC-002, AC-003 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | Persona 2's measured today-state at lines 167-181: `TraitVariantBlanketType` appears six times in the workspace and **none** of them is `store.rs`. That is the baseline the walk is measured against, and journey steps 2→3 are the hop being observed. | Before the walk, to know what "improved" would look like; and when interpreting a search that returns contributor-facing hits. | AC-001, AC-003 |
| `crates/happenstance-core/src/store.rs` | The surface the walk starts on. AC-006's quoted sentence must exist in this file verbatim, and the heading ladder (`:3, 21, 31, 47`) is where a reader lands from a fragment. Today's trimmed excerpt at `:31-45` is the before-state the rewrite replaces. | At Observation A, to verify the quote is the file's own rather than a paraphrase. | AC-001, AC-006 |
| `crates/happenstance-core/src/memory.rs` | The subject of the caveat the walker checks on arrival: `:16-31` is where it says `MemoryEventStore` is the conformance suite's oracle and the reference implementation. If the account omits that where `memory.rs` is first sequenced, the record files a finding. | Only at the arrival check, when confirming the caveat is present and in position. | AC-004 |
| `rust-toolchain.toml` | The `channel = "1.97.1"` pin that fixes the diagnostic wording. It is what makes the reproduction re-runnable and what makes a later mismatch attributable to drift rather than regression. | When running the reproduction, and when recording `rustc -V`. | AC-001 |
| `.bklg/docs-that-teach/reach-and-adapter-path/design/mock.html` | The measurements behind the transcript: finding **F2** (rustc emits a `help:` hunk per candidate, so the honest fence is 14 source lines) and **F4** (the longest line is 109 characters and overflows at *both* viewports). The reference for what a faithful, un-reflowed transcript looks like. | If the reproduction stanza is long or wide and the instinct is to trim it — open this before yielding. | AC-001, AC-009 |
| `.bklg/docs-that-teach/reach-and-adapter-path/_grounding.md` | "Sibling-project dependency state" records, verified rather than assumed, that HS-P0020 and HS-P0021 had no `_design.md` at grounding time — the concrete shape of EC-001, and the reason the account may not yet be a routable page. | At preflight, before recruiting a walker, to decide whether this story is blocked rather than failed. | AC-004 |
| `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` | Staged disclosure's documented failure mode — a reader *can land past the setup with no signal they missed it* — is exactly what the `n of 6` position marker in the arrival check tests for. It also carries the anti-pattern that rules out any bespoke navigation widget. | When writing the arrival check, and if a finding tempts a navigational fix. | AC-004, AC-009 |

## Clarifications resolved during spec

1. **A ninth criterion was added, and the first pass's enumeration predates it.** The sentence
   closing `## Behavior and interfaces` lists `AC-001`–`AC-008`; this pass added **AC-009**, the
   composition criterion. The reason is structural rather than stylistic: `_design.md`'s named
   anti-patterns (3, 5, 6, 9, 10), `_storymap.md`'s standing constraints, and the density budget's
   link-text minimum all *apply* to the artefacts this PR lands, and none of them had a row. Every
   mechanical assertion in AC-001–AC-008 passes on a record that is one unbroken paragraph with the
   right strings in it — AC-009 is the row that fails it. The ledger carries nine rows to match, and
   nothing in the front half is otherwise changed.
2. **This story renders no `_design.md` surface, and is still bound by the design.** `## Surfaces`
   declares five; this story renders none and *observes* two. The composition family therefore binds
   through the project-wide constraints rather than through a surface's own composition table, and
   `## Interaction quality` says so explicitly so a reviewer does not go looking for a surface id
   that is deliberately absent.
3. **The claim's ceiling is settled and is not this story's to raise.** The record is evidence that
   the path exists for a non-author. It is not evidence that a stranger finds it. `project.md`'s risk
   table, `_storymap.md`'s standing constraints and `initiative.md`'s BR-14 all say this in the same
   words, and the spec makes it AC-002 rather than a note, because a note does not reach the ledger.
4. **A blocked walk and a failed walk are different artefacts.** EC-001 (the destination does not
   exist yet) halts the story; EC-002 (the destination exists and the path does not carry the walker)
   produces a recorded failed walk, which is a *successful* execution of this story. The distinction
   was not stated in the front half and is easy to collapse, with the effect of recording a
   scheduling fact as a teaching failure.
5. **The scratch's location is a boundary decision, not a preference.** Keeping the failing
   double-import outside the worktree makes EC-005 unreachable by construction rather than by
   discipline, which is why `## PR boundary` carries no `crates/**` glob and why
   `git diff --name-only main...HEAD -- crates/` is an AC-001 check rather than a review note.
6. **No conformance rule and no specification clause is in play.** Confirmed against `CLAUDE.md`'s
   rule that a rule no adapter can fail is decorative: this story changes no port, type or behaviour,
   so nothing in `crates/happenstance-testkit/` could observe it. `cargo xtask spec-trace` runs to
   prove the tree is undisturbed, not to discharge anything.
