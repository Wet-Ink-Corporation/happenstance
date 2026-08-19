---
item: HS-S0092
stage: spec
created: 2026-08-12T13:47:30.259Z
updated: 2026-08-12T13:47:30.259Z
template_sig: 87bbf1d0
rendered_sig: 11b61733
---

# Spec — The first screen carries the lead claim, and no sentence on it is false

## Scope lock

| | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project card | `.bklg/from-contract-to-published-library/publication-and-positioning/project.md` |
| **This spec** | `.bklg/from-contract-to-published-library/publication-and-positioning/landing-copy-and-status-truth/spec.md` |
| Signed-off design (**binding**) | `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` |
| Briefs (ux / testing / deployment) | `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` |
| Story map (this row, milestone `published-surface-copy`) | `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md`:64 |
| Sign-off mock (51 frames, all seven surfaces) | `.bklg/from-contract-to-published-library/publication-and-positioning/design/mock.html` |
| Roadmap pointer | `RUNBOOK.md`:163 — phase 12's row and its proof artefact |

Traces to project **AC-007** (the published crate looks finished — the *copy* half) and
**AC-011** (the first-contact decisions are recorded, not defaulted — the *surface carries it*
half). Depends on `first-contact-design-resolutions`, `crate-set-decision`,
`projection-port-ship-shape`.

## One-line PR slice

Carry DT-1's lead claim onto the packaged `happenstance` README's first screen and DT-5's
maturity treatment and DT-6's peer positioning into the slots that already exist
(`README.md`:218-225 Prior art, `README.md`:79-98 status table), correct the four stale strings
(`README.md`:11-16, `:104-109`, `:85-90`, `crates/happenstance/README.md`:6-11), spell *passes the
conformance suite* differently from *compiles*, never suppress the 49 provisional clauses while
listing the frozen ones, and check every link and held anchor mechanically rather than by reading.

## Executive summary

This PR lands the **status-truth and positioning half of the published surface**: the first screen
of all three packaged READMEs, the maturity census sentence, the peer statement, the status table's
vocabulary, the `description` field, and a new mandatory gate step that makes the link/anchor
discipline mechanical instead of read.

**Pointer, not restatement.** The decisions are already made and are not reopened here: DT-1, DT-4,
DT-5 and DT-6 are resolved in `_design.md` (`## Pattern decision`), the crate set is three
(`_decomposition.md`, *Deployment brief → The crate-set decision*), and the composition, density
budget and anti-patterns are `_design.md`'s `## Composition`, `## Density budget` and
`## Anti-patterns`. This story **implements** them.

**The delta this PR adds on top of those decisions** is four things nothing upstream owns:

1. The copy itself, in the slots that already exist — no new section on any surface.
2. A **status vocabulary** that distinguishes *published and passes the conformance suite* from
   *in tree and compiles*, applied per row against each adapter project's committed report.
3. Two further false-at-publish status sentences the UX brief's list of four does not name, found
   while grounding this spec: `crates/happenstance-core/README.md`:8-9 (*"every storage adapter is
   a documented stub"*) and `crates/happenstance-testkit/README.md`:16-17 (*"No adapter has run
   this suite"*). Both are status claims on packaged surfaces and both are false on the tree this
   release ships, so IQ-7 puts them in this story with the other four.
4. The **mechanical** half of AC-UX-011 and AP-9: a new `xtask` file-reading check, mounted in the
   mandatory gate, that fails on a relative link on a packaged surface, a dead intra-document
   anchor, a lost `#status` / `#licence` anchor, a resurrected stale string, or a census count that
   disagrees with `spec/SPECIFICATION.md`'s §1.3 figure.

## Context pack

The load-bearing decisions this story must honour. Each is stated as a decision, not as a reading
list; the deeper artifacts sit behind the anchors table the second pass appends.

### Who is on the other side, and what that costs the copy

The reader is Persona 4, the **evaluator** — and `_design.md` settled that this is *the first
fifteen minutes of the application author's journey, not a fifth persona*
(`_design.md`, DT-1's persona question). Three properties of that moment bind every sentence below:
the reader has **one sitting**, they **cannot run the conformance suite**, and they will not return
from a context jump. So: **0 hops to read a claim, ≤ 1 hop to its evidence, and that hop lands on a
specific anchor** (IQ-1); and **no claim may have "run our CI" or "clone the repo" as its only
evidence** (IQ-5). A scroll is not a hop; a link is.

### DT-1 — the lead claim, already chosen

**Resolved (a): lead with storage-agnostic proof, stated as an act the reader can perform rather
than as an adjective.** The packaged `happenstance` README's first screen leads with *what this is
and how you can check it*: a contract for storage plus a **published** conformance suite that
decides who meets it. `README.md`:3-6 already says this and is the model sentence.

Do not re-decide it. **(b) the edge story lost** because the constrained-runtime reader can
self-identify from one line in Guarantees while the general evaluator cannot self-identify from an
edge lead at all — that asymmetry is the whole argument. **(c) two entry points lost** because
crates.io renders exactly one README per crate, so "two entry points" means two crates' front
pages, and ADR-0006's crate split is by **role** (application author vs adapter author), not by
runtime (`.kb/decisions/0006-bare-name-to-the-typed-layer.md`).

### DT-5 — the maturity census, in one dated sentence

**Resolved (c): publish a summary that defines all four words inline and links the ledger.** One
paragraph, ≤ 3 rendered lines, exactly four counts, four inline definitions, exactly one link,
**dated and scoped to the version** so it reads as a snapshot rather than a standing claim. Sited
immediately after the status callout on `happenstance` and `happenstance-core`; below the fold on
`happenstance-testkit`, whose callout already carries a sharper maturity signal.

Two hard edges. **(b) frozen-only is forbidden, not merely worse** — listing 139 frozen guarantees
while suppressing the existence of 49 provisional ones is IQ-2's filter hiding what it filters
(AP-4). And the four numbers are **taken from `spec/SPECIFICATION.md`:219-222 at the publish
commit, never typed from memory**; the count is **49**, never 46 — `RUNBOOK.md`:4495's anchor is
the stale artifact. **No other sentence on any surface repeats a count** (AP-3), so a future
release edits one sentence per README rather than N.

This also closes a live defect: `crates/happenstance/README.md`:53-56 already uses the words
*"a maturity marker"* while the vocabulary they belong to is defined nowhere a consumer reaches.
IQ-6 forbids orphan vocabulary, and *"leave it as is"* was never an option.

### DT-6 — the peer statement, dated, in the slot that already exists

**Resolved (a): state it** — in full in the existing **Prior art** section (`README.md`:218-225) on
the GitHub surface, and as a **one-sentence, one-link reduced form** on the packaged `happenstance`
README. Not a new section on either. **Prose only, one sentence per peer, never a matrix** (AP-5):
a matrix implies completeness and a row goes stale invisibly, whereas a stale sentence reads as a
stale sentence. Each sentence says what the peer is good at, then what happenstance's *different
bet* is — never a ranking; `README.md`:180's *"if you want DCB on Postgres today, `disintegrate`
below is the mature choice"* is the model and is not to be rewritten. The date is **re-read at the
publish commit, not at decision time**: the nearest live peer shipped the day before intake.

### The packaging boundary is the placement rule, and it behaves like coherence

`include_str!` resolves against the file tree and a path reaching outside the package does not
exist inside a packaged `.crate` (`README.md`:131-137). Therefore:

- Copy that must reach a registry reader lives in **`crates/<name>/README.md`** — that is what
  `readme = "README.md"` packages (`crates/happenstance/Cargo.toml`:12) and what
  `xtask/src/package.rs`:88-94's `REQUIRED_FILES` asserts is contained. **Write the crate READMEs
  first**; the repo-root README is the *third* surface, compiled by `xtask`, which is never
  published.
- **Every link on a packaged surface is absolute; every link on the GitHub root is repo-relative.**
  The tree already splits this way (`crates/happenstance/README.md`:48,54 vs `README.md`:83-90).
  Copying a region across the boundary without rewriting its links produces a dead link on a page
  that can never be edited — AP-6, and the reason AC-UX-011 says *checked mechanically, not read*.

### The first screen is full at 0.2.0

The binding viewport is **1024×768**, where crates.io stacks the crate metadata above the README
and leaves ≈ 340 px ≈ **14 rendered lines**. The allocation is identity 3 + blank 1 + callout 3 +
blank 1 + triad 6 = 14, and the sign-off records that at real Markdown block metrics the five
regions measure **≈ 343 px against 340** — they fit only with each callout tightened to two
rendered lines and each triad bullet to one. So: **any sixth region, or a callout that grows back
to three lines, pushes the triad's third bullet across the fold and AP-1 fails.**

When the budget is exceeded the demotion order is fixed and is not the implementer's to reorder:
**badges first** (removed from the first screen outright — they are the only region that is
entirely decoration and they render as nothing with images blocked, AP-14), **then the maturity
census** (below the triad; its definitions may never be dropped to make it fit, IQ-6), **then the
identity line compresses from two sentences to one**. The status callout and the disambiguation
triad **never** yield — if those three moves are not enough, something was added that should not
have been.

### Status truth: the vocabulary is the deliverable, not the glyph

`CLAUDE.md`'s bar is the rule: *an adapter that compiles but has not run the suite is not an
adapter*. The status table must therefore never spell these the same way:

1. **published and passes the conformance suite** — the only state that supports a compliance claim;
2. **published, no suite applies** (the testkit itself; the facade);
3. **in tree, compiles, has not passed the suite** — `xtask/src/package.rs`:20-22 names the live
   instance: `cargo package -p happenstance-sqlite --list` exits 0 and lists seven files, none of
   them required;
4. **skeleton / `todo!()`** — an instrument, not a target;
5. **deliberately out of this release train** (`happenstance-sync`, `happenstance-sync-testkit`) —
   a crate absent from the registry with no explanation reads as abandonment, so this state needs
   words.

Every cell carries **glyph *and* words** (AP-2); `README.md`:83-90 already complies and that
compliance is to be preserved, not re-derived. The 8-row table stays **GitHub-only** (AP-11): it
costs more than the entire first-screen budget, its content is repo-shaped, and its relative links
die at the packaging boundary. On crates.io the disambiguation triad carries *"which crate"*; the
table carries *"how far along is each crate"* on GitHub.

### Truth at the publish commit is an observable property of the diff

IQ-7: **no sentence on any published surface may be false on the tree that shipped**, and this is
the documentation analogue of AC-015. Six known-false sentences, four named by the UX brief and two
found while grounding this spec:

| Where | The sentence that goes false | Named by |
| --- | --- | --- |
| `README.md`:11-16 | *"Nothing is published yet."* | UX brief; AP-9 |
| `README.md`:104-109 | `happenstance = "0.1"` + *"That version does not resolve yet… the only way to try this is a git dependency"* | UX brief; AP-9 |
| `README.md`:85-90 | 🔲 rows for crates upstream projects have made real, and a `happenstance` row reading *"a facade over `happenstance-core` today"* | UX brief; AP-9 |
| `crates/happenstance/README.md`:6-11 | *"this crate is currently a facade… adds nothing yet"* | UX brief; AP-9 |
| `crates/happenstance-core/README.md`:8-9 | *"every storage adapter is a documented stub"* | **this spec** (IQ-7) |
| `crates/happenstance-testkit/README.md`:16-17 | *"No adapter has run this suite"* | **this spec** (IQ-7) |

The `description` field is a published sentence like any other and is re-read on the same footing:
it is **the only text crates.io shows in a search result and on the crate card** — the README never
renders there — so it is the lead claim in miniature. Today's is **131 characters** (budget: ≤ 120)
and claims *typed events, decision models and projection runners*, which is a claim about a tree
this release may not ship.

### Anchors are load-bearing identifiers, not headings

IQ-3: every heading anchor a published surface exposes is a deep link somebody may already hold,
and a published page cannot be edited afterwards. **Either the anchor survives, or every inbound
reference is updated in the same change.** Two live inbound anchors are known and must survive:
`README.md`:9 → `#licence` and `README.md`:16 → `#status`. The house precedent is already
normative — `spec/SPECIFICATION.md`:210-211 retains a demoted clause's ID *"so that citations
resolve"*.

### What this story consumes from its three blockers, and must not re-decide

- **`crate-set-decision`** — three crates ship: `happenstance-core`, `happenstance`,
  `happenstance-testkit`. The disambiguation triad therefore has **exactly three entries**; a
  fourth entry means a fourth published crate, which the deployment brief rejected by name
  (`RUNBOOK.md`:4450-4451's four-crate alternative). The status table still lists the unpublished
  crates — with state 4 or state 5 words, never a glyph alone.
- **`projection-port-ship-shape`** — PS-3's arm is **not** decided here. Whichever arm landed, the
  copy names the port **with its gate** rather than omitting it: an item that vanishes reads as
  *"not supported"*, which is a different and false claim (IQ-2). The `doc(cfg)` treatment itself
  and the manifest block belong to the slice-mate.
- **`first-contact-design-resolutions`** — DT-1/4/5/6 and the evaluator-persona question are
  settled and recorded in an ingested decision atom. This story **authors no decision atom**; it
  carries the settled answers onto the surface. Hand-writing atoms under `.kb/` is forbidden
  (`CLAUDE.md`; the first attempt was reverted at `0269720`).

### The slice boundary, so three stories do not write the same paragraph

`published-surface-copy` is one slice because it is **one page**. What this story does **not**
write: the compliance claim block and the positions-and-gaps promise (R6 and the DT-4 bullet →
`compliance-claim-and-gaps-promise`); the Guarantees block's MSRV/`wasm32`/minor-bump content, the
`[package.metadata.docs.rs]` block and the doctest identity of the quick-start fence (R9, R7 →
`guarantees-and-docs-rs-presentation`). This story owns R1–R5, R10, R11 and the GitHub landing's
status table and quick-start prose. The mechanical check this story mounts then covers the
slice-mates' copy as it lands — that is the point of landing it first (merge order 3.1).

### Reversibility is bought before the act, never after

`cargo yank` removes a version from resolution and **leaves every rendered page exactly as it was**
(`standards/rust/51-features-and-no-std.md`:226-231). So every claim here is worded so that it can
be **superseded by a later dated claim** rather than needing a silent rewrite — state the fact, the
scope and the date — and every check that can be mechanical is mechanical *before* publish. The
rendered read itself is `rendered-page-preflight`'s (AC-007), strictly before `publish-0-2-0`.

## Integration contract

**Archetype**: `capability` — a user-observable slice; a reader on crates.io sees its whole output.

**Slice / milestone**: `published-surface-copy`. Slice-mates, implemented in the same context:
`compliance-claim-and-gaps-promise`, `guarantees-and-docs-rs-presentation`. Merge order within the
slice: this story is **3.1**, first.

**Mount point** — `crates/happenstance/README.md`, the source of record for the primary designed
surface `crates-io-happenstance`. It is mounted, not merely written, by two existing wires that
must both stay true:

- `readme = "README.md"` at `crates/happenstance/Cargo.toml`:12 — what `cargo package` puts inside
  the `.crate` and what crates.io renders;
- `#![cfg_attr(doctest, doc = include_str!("../README.md"))]` at `crates/happenstance/src/lib.rs`:10
  — the crate compiles its own README's fences, so copy that breaks an example breaks the build.

**Second mount, for the mechanical half**: `xtask/src/main.rs`'s `REQUIRED` step list plus the
unconditional file-reading block in `xtask/src/affected.rs`:116-125. A check that exists as a
module but is in neither list is not mounted — it is a function nobody calls.

**Wires into** (real siblings and contracts consumed, by path):

- `crates/happenstance-core/README.md`, `crates/happenstance-testkit/README.md` — the two secondary
  packaged surfaces, same skeleton with the two differences `_design.md` names.
- `README.md` — the GitHub landing surface: status callout, 8-row status table, Quick start prose,
  Prior art.
- `crates/happenstance/Cargo.toml` — the `description` field (shared file with the slice-mate that
  adds `[package.metadata.docs.rs]`; append, do not restructure).
- `spec/SPECIFICATION.md`:219-222 — the census the DT-5 sentence's four numbers are read from. Read
  only; this story amends no clause.
- `xtask/src/package.rs`:88-94 (`REQUIRED_FILES`) and `:408-457` (the `#[cfg(test)] mod tests`
  shape: a fixture string, a named wrong implementation, an assertion that it is rejected) —
  the precedent the new module follows.
- `xtask/src/lints.rs` — the five existing file-reading lints, each of which documents what it does
  *not* verify; the new check adopts that convention.

**Renders surfaces** (ids from `_design.md`'s `## Surfaces`): `crates-io-happenstance` *(primary)*,
`crates-io-happenstance-core`, `crates-io-happenstance-testkit`, `github-landing`. The three
`docs-rs-*` surfaces are **not** changed by this story — they render from `//!` module docs and
belong to the slice-mate.

**Conformance rule(s)**: **none, and deliberately.** This story changes no port, ships no adapter
and is not adapter-observable; `happenstance_testkit::event_store_conformance!` is out of scope
here (testing brief, *Fixtures and seams to mock*). Its instrument is the new `xtask` file-reading
check plus the human rendered-page read that `rendered-page-preflight` performs.

**Clause(s)**: none discharged, none amended. `spec/SPECIFICATION.md` is read for its §1.3 census
only. Amending a `[FROZEN]` clause would take a new ADR and a re-plan (project DR-15).

**Advances DoD scenario**: initiative **DoD 10** — *"The published crate looks finished… the
registry page carries licence, description and README as rendered, checked by looking at them"*
(`initiative.md`:387-389). This story makes the page worth looking at and makes its links checkable
without looking; `rendered-page-preflight` performs the look, and `publish-0-2-0` turns it green.

## PR boundary

The paths this story may touch. `redkiln verify --grain story` reads the first fenced block under
this heading and fails on any file changed outside it.

```
crates/happenstance/README.md
crates/happenstance-core/README.md
crates/happenstance-testkit/README.md
crates/happenstance/Cargo.toml
README.md
xtask/src/docs_copy.rs
xtask/src/main.rs
xtask/src/affected.rs
.bklg/from-contract-to-published-library/publication-and-positioning/landing-copy-and-status-truth/**
```

**In this PR**

- R1–R3 on all three packaged READMEs: identity line, dated status callout, disambiguation triad —
  inside the 14-line first-screen budget.
- R4, the DT-5 census sentence, on `happenstance` and `happenstance-core`; below the fold on
  `happenstance-testkit`.
- R5: badges moved **below** the census on packaged surfaces; unchanged at the top on GitHub.
- R10 (peer statement: one sentence + one link, packaged; dated full form in `README.md`:218-225)
  and R11 (the Design/spec paragraph rewritten so *"maturity marker"* is no longer orphan
  vocabulary).
- The six false-at-publish sentences in the Context pack's table, and the `description` field.
- The GitHub status table's vocabulary and rows, and the Quick start's version + *"does not resolve
  yet"* prose.
- `xtask/src/docs_copy.rs`: the new file-reading check, its `#[cfg(test)] mod tests`, its `REQUIRED`
  step entry, its dispatch arm, its line in `xtask/src/main.rs`:8-24's module doc, and its call in
  `xtask/src/affected.rs`'s unconditional block. **Mounting it is not scope drift.**

**Explicitly not in this PR**

- The compliance claim block and the positions-and-gaps promise → `compliance-claim-and-gaps-promise`.
- The Guarantees block's MSRV / minor-bump / `wasm32` bullets, `[package.metadata.docs.rs]`, and
  making the quick-start fence the crate's own doctest → `guarantees-and-docs-rs-presentation`.
- Any `//!` module doc or `doc(cfg)` attribute; any Rust API change of any kind.
- Reading the **rendered** crates.io/docs.rs pages, and the accessibility-floor observation →
  `rendered-page-preflight`. The publish itself → `publish-0-2-0`.
- Any decision atom, any edit under `.kb/`, any edit to `spec/SPECIFICATION.md` or `RUNBOOK.md`'s
  ledger, and the surface-diff / clause-audit instruments (milestone
  `publish-time-gate-instruments`). Where those stories also edit `xtask/src/main.rs`'s `REQUIRED`
  list and module doc, **append; never restructure** — the storymap already names that paragraph as
  a merge conflict by construction.

**Merge DoD**: `cargo xtask affected --base main` is green (it runs the new check unconditionally),
every AC below is observed on the source files, and the new check is shown to fail on each seeded
wrong implementation it names.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **First screen, primary surface** | `crates-io-happenstance` renders, above the fold at 1024×768 and in this order: `#` H1 + one-line identity (≤ 2 rendered lines) → dated status blockquote (≤ 2–3 lines) → *"Which crate do I want?"* `##` triad (≤ 6 lines, exactly 3 entries). Nothing else. Total ≤ 14 rendered lines | `_design.md` `## Composition`, `## Density budget`, `## What a user meets first`; `crates/happenstance/README.md`:13-20 (the triad's existing form) |
| **DT-1's lead claim** | The identity line states the claim as an act the reader can perform: a contract for storage **plus a published conformance suite that decides who meets it**. Not an adjective, not a feature list, not the edge story | `_design.md` DT-1; `README.md`:3-6 (model sentence); `.kb/decisions/0006-bare-name-to-the-typed-layer.md` |
| **Maturity census (DT-5)** | One paragraph, no heading, bold only on the four numbers, ≤ 3 rendered lines: 200 IDs / 139 frozen / 49 provisional / 10 deferred / 2 demoted, **each of the four words defined inline**, dated and version-scoped, exactly one link to the ledger. Appears **exactly once per surface**; no other sentence repeats a count | `_design.md` DT-5, `## Hierarchy` (R4 = secondary), `## Density budget`; `spec/SPECIFICATION.md`:219-222 |
| **Non-occlusion** | No treatment lists frozen guarantees while suppressing the existence of the 49 provisional clauses; no maturity term appears on a consumer surface without a consumer-reachable definition | AP-4, AP-3, IQ-2, IQ-6; `spec/SPECIFICATION.md`:213-217; `.kb/decisions/0010-the-suite-must-prove-itself.md` (the same discipline in the testkit) |
| **Badge demotion** | On the three packaged surfaces badges sit **below** the census; on GitHub they stay at the top. Every badge's meaning is also stated in text, so the page reads correctly with images blocked | AP-14, AP-7; `_design.md` `## Transience policy`, `## States` (*Images blocked*); `README.md`:8-9 |
| **Peer statement (DT-6)** | Full form stays in `README.md`:218-225 and gains *"As of YYYY-MM-DD"*, re-read at the publish commit; packaged surfaces carry one sentence + one absolute link. Prose only — no matrix, no ranking, no new section | `_design.md` DT-6; AP-5; `README.md`:180 (the model sentence) |
| **Status vocabulary** | Every status row spells *published and passes the conformance suite*, *published, no suite applies*, *in tree and compiles*, *skeleton*, and *out of this release train* differently. Each row's state comes from the owning adapter project's committed report, never from the fact that it builds | UX brief *States this surface must render* (`_decomposition.md`:97-116); `CLAUDE.md` (*an adapter that compiles… is not an adapter*); `xtask/src/package.rs`:20-22 |
| **Status table siting** | The 8-row / 3-column table appears on `github-landing` only; every cell carries glyph **and** words; the Role cell stays ≤ 90 chars and the Status cell is glyph + 2–5 words | AP-11, AP-2; `_design.md` `## Density budget`, `## States` (*Overflow*, *Long label*); `README.md`:79-98 |
| **Truth at the publish commit** | The six sentences in the Context pack's table are corrected; none of the four AP-9 strings survives anywhere on a published surface | IQ-7, AP-9; `_decomposition.md`:118-129; `crates/happenstance-core/README.md`:8-9; `crates/happenstance-testkit/README.md`:16-17 |
| **`description` field** | ≤ 120 characters, one sentence, true standalone of the published tree, and consistent with the identity line it is the miniature of. Today: 131 characters | `_design.md` `## Shape decision` (the `description` row), `## Density budget`; `crates/happenstance/Cargo.toml`:3 |
| **Link resolution** | Every link on `crates/*/README.md` is an absolute URL; every link on the repo-root `README.md` is repo-relative. Link text is meaningful standalone — never *"here"*, *"this"*, or a bare URL | AP-6; `_design.md` `## Placement and re-export`; `crates/happenstance/README.md`:48,54 vs `README.md`:83-90 |
| **Held anchors** | `#status` and `#licence` still resolve on the root README after this change, or every inbound reference moves in the same commit | IQ-3; `README.md`:9,16; `spec/SPECIFICATION.md`:210-211 |
| **The mechanical check** | New `xtask/src/docs_copy.rs` with a module doc that states what it does *not* verify. Five checks over the four surface files: (1) no relative link in `crates/*/README.md`; (2) every intra-document `#anchor` resolves to a heading in the same file; (3) `#status` and `#licence` exist in `README.md`; (4) none of the four AP-9 strings appears; (5) the four census numbers equal `spec/SPECIFICATION.md`'s §1.3 figures. Self-sufficient: it parses §1.3 itself rather than waiting on `clause-maturity-audit`, which is in an unordered milestone | `xtask/src/lints.rs` (the five-lint convention); `xtask/src/spec_trace.rs`:39-57 (why §1.3 stays hand-computed); testing brief AC-TEST-002 |
| **The check is mounted and can fail** | A `Step` with `probe: None` in `xtask/src/main.rs`'s `REQUIRED` (mandatory — there is no external tool to probe for), a dispatch arm, a sentence in the module doc at `xtask/src/main.rs`:8-24, a call in `xtask/src/affected.rs`'s unconditional block (this story's diff is mostly Markdown, which affects no package), and `#[cfg(test)] mod tests` carrying a named wrong implementation per check | `xtask/src/main.rs`:73-103 (`Step`, and what `probe: None` means), `:105` (`REQUIRED` opens), `:515-532` (`package-check`'s entry as the model), `xtask/src/affected.rs`:116-125; `xtask/src/package.rs`:408-457 (test shape); `CLAUDE.md` (*a rule that no adapter can fail is decorative*) |
| **Surfaces not touched** | No `//!` module doc, no `doc(cfg)`, no Rust item, no feature. The three `docs-rs-*` surfaces render unchanged by this story | `_design.md` `## Items` (this project's API surface is nearly empty by construction); project `## Out of scope` |

## Data and migrations

**N/A — this story ships no schema, no stored data and no migration.** It edits four Markdown files,
one manifest field and one `xtask` module; nothing it touches is read by a running system, and there
is no persisted state anywhere in its blast radius.

Two migration-shaped concerns exist and are handled as checks rather than as migrations, because
the thing being "migrated" is an identifier a third party may already hold:

1. **Anchor stability** — a renamed heading is a broken deep link on a page that can never be edited
   after publish. Treated as an invariant (`#status`, `#licence` survive, or every inbound reference
   moves in the same commit), enforced by check (3) of `docs_copy`, not by a redirect. The house
   precedent is `spec/SPECIFICATION.md`:210-211, which retains a demoted clause's ID *"so that
   citations resolve"*.
2. **Version-scoped claims** — the census sentence and the peer statement are dated and scoped to
   `0.2.0` precisely so a later release **supersedes** them with a new dated claim rather than
   silently rewriting an old one. That is the only "migration path" this surface has: a yank removes
   a version from resolution and leaves the rendered page exactly as it was
   (`standards/rust/51-features-and-no-std.md`:226-231).

## Acceptance criteria

Nine criteria. Each is framed from the **reader's** goal — Persona 4, the evaluator, one-shot and
time-boxed, who cannot run the suite (`_discovery/distillation/personas-and-journeys.md`:249-266,
:333-338) — crossing the whole stack from the source file, through the packaging boundary, to the
rendered page. Every project AC this story traces to is covered: **AC-007** by AC-001, AC-002,
AC-004, AC-005, AC-008, AC-009; **AC-011** by AC-001, AC-002, AC-003.

Two verification instruments recur, and they are different things:

- **`xtask/src/docs_copy.rs`'s five gate checks** — the mechanical half, mounted in `REQUIRED`.
  Each fails the gate on a seeded wrong implementation named in its own `#[cfg(test)] mod tests`,
  in the `xtask/src/package.rs`:408-457 shape.
- **`xtask/src/docs_copy.rs::tests`' structural assertions over the *real* surface files**, resolved
  through `affected::workspace_root()`. These are `#[test]`s rather than gate checks; they fail
  through `cargo test -p xtask`, which `cargo xtask affected --base main` runs because this diff
  touches `xtask/src/`. See *Clarifications resolved during spec* for why the split exists.

The **rendered** page is not read here — that is `rendered-page-preflight` (AC-007), strictly later.
Where a criterion below needs a picture, the picture it is checked against is the **signed-off mock**
at `.bklg/from-contract-to-published-library/publication-and-positioning/design/mock.html`, which
exists today; the `design/reference/*.png` frames `_design.md` tables do **not** exist yet and are
that story's drop location.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** an evaluator who reached `https://crates.io/crates/happenstance` from a registry search with one sitting and no way to run the conformance suite, **WHEN** they read the first screen at 1024×768 and stop there, **THEN** they can state both what the library is — a contract for storage plus a *published* conformance suite that decides who meets it, stated as an act they can perform rather than as an adjective — and which of the three crates is theirs, because `crates/happenstance/README.md` renders R1 identity, then R2 dated status callout, then the R3 *Which crate do I want?* triad, in that order, with no other region above or between them. | `xtask/src/docs_copy.rs::tests::first_screen_regions_are_in_order` over the real file (wrong implementation: a badge line or the census paragraph inserted above the triad — AP-1, AP-14); recorded comparison against the `crates-io-happenstance@1024x768` frame of `…/publication-and-positioning/design/mock.html` |
| **AC-002** | **GIVEN** a reader who has just been told that the specification carries a maturity marker per clause, **WHEN** they look for what those marks mean without leaving the page, **THEN** one dated, version-scoped paragraph of at most three rendered lines defines frozen, provisional, deferred and demoted **inline**, carries exactly the four counts read from `spec/SPECIFICATION.md`:219-222 at the publish commit (200 IDs, 139 frozen, 49 provisional, 10 deferred, 2 demoted — never 46), carries exactly one link to the ledger, appears exactly once on that surface, and the R11 Design paragraph no longer leaves *maturity marker* as orphan vocabulary. | `docs_copy` check (5), census-numbers-equal-§1.3 (wrong implementation: a fixture asserting 46, and a fixture whose §1.3 figure disagrees with the parsed clause set); `xtask/src/docs_copy.rs::tests::census_appears_once_and_defines_its_terms` (wrong implementation: a second sentence elsewhere on the page repeating a count — AP-3; a frozen-only treatment with no mention of the 49 — AP-4) |
| **AC-003** | **GIVEN** an evaluator who is comparing happenstance against the peers they already found, **WHEN** they reach the Prior art section on GitHub or the peer sentence on the packaged README, **THEN** they read prose — one sentence per peer, saying what that peer is good at and then what happenstance's different bet is, never a ranking and never a matrix — carrying an *As of YYYY-MM-DD* date re-read at the publish commit rather than at decision time, in the slot that already exists rather than in a new section, and the packaged surface carries the reduced form of one sentence plus one absolute link. | `xtask/src/docs_copy.rs::tests::prior_art_is_dated_prose` (wrong implementations: a table row naming a peer — AP-5; an undated section; a new `##` heading beside the existing Prior art); `docs_copy` check (1) for the packaged sentence's link being absolute |
| **AC-004** | **GIVEN** a reader deciding whether an adapter is usable, **WHEN** they read the status table on the GitHub landing page, **THEN** *published and passes the conformance suite*, *published, no suite applies*, *in tree and compiles*, *skeleton*, and *deliberately out of this release train* are each spelled differently, every cell carries a glyph **and** words, each row's state is taken from the owning adapter project's committed report rather than from the fact that the crate builds, and the table appears on **no** packaged surface. | `xtask/src/docs_copy.rs::tests::status_rows_carry_words_and_distinguish_states` (wrong implementations: a ✅ or 🔲 alone in a Status cell — AP-2; the same wording for a crate that passed the suite and one that only compiles, the live instance being `happenstance-sqlite` per `xtask/src/package.rs`:20-22); `tests::status_table_is_github_only` (wrong implementation: the 8-row table copied onto `crates/happenstance/README.md` — AP-11) |
| **AC-005** | **GIVEN** a reader arriving at the published `0.2.0` page, which can never be edited afterwards, **WHEN** they read any sentence on any of the four surfaces or the crate card in a search result, **THEN** no sentence is false of the tree that shipped: none of the six known-false sentences in the Context pack's table survives anywhere, and `crates/happenstance/Cargo.toml`'s `description` is one sentence of at most 120 characters that is true standalone of the published tree and is the identity line in miniature. | `docs_copy` check (4), forbidden-strings (wrong implementation: each of the four AP-9 strings seeded back into a fixture, plus the two this spec adds at `crates/happenstance-core/README.md`:8-9 and `crates/happenstance-testkit/README.md`:16-17); `xtask/src/docs_copy.rs::tests::description_is_within_budget_and_present` reading the real manifest (wrong implementation: today's 131-character string) |
| **AC-006** | **GIVEN** somebody who already holds a deep link into this documentation — `README.md#status`, `README.md#licence`, or a link that was copied across the packaging boundary — **WHEN** the copy for `0.2.0` lands, **THEN** both held anchors still resolve or every inbound reference moved in the same commit, every link on `crates/*/README.md` is an absolute URL, every link on the repo-root `README.md` is repo-relative, every intra-document `#anchor` resolves to a heading in the same file, and no link text is *here*, *this*, or a bare URL. | `docs_copy` checks (1) no-relative-link-on-a-packaged-surface, (2) every-intra-document-anchor-resolves, (3) `#status` and `#licence` exist in `README.md` — each with its own seeded wrong implementation (a relative `crates/…` path on a packaged fixture; an anchor pointing at a heading that was renamed; a fixture whose Status heading was retitled) |
| **AC-007** | **GIVEN** a maintainer whose change is entirely Markdown and therefore maps to no workspace package, **WHEN** they run the story gate `cargo xtask affected --base main`, **THEN** the copy check runs anyway — unconditionally, in the file-reading block, as a `probe: None` step in `REQUIRED` with a dispatch arm and a sentence in the module doc at `xtask/src/main.rs`:8-24 — so that the discipline is mechanical rather than read, and each of its five checks is demonstrated to fail on the wrong implementation it names. | `cargo xtask affected --base main` on a docs-only diff (proves the unconditional mount at `xtask/src/affected.rs`:116-125); `cargo test -p xtask` for `docs_copy`'s `#[cfg(test)] mod tests`; `cargo xtask ci --fast` for the `REQUIRED` mount; `steps_named` at `xtask/src/main.rs`:816-826 panics if the step name is absent, which is what makes an unmounted check impossible to land quietly |
| **AC-008** | **GIVEN** a reader whose client blocks images, or who is navigating by headings with assistive technology, **WHEN** they read any of the four surfaces, **THEN** the page is complete without the badges — every badge's meaning is also stated in text and on the three packaged surfaces the badges sit below the census rather than under the H1 — and the structure is navigable: exactly one `#` H1 per document, no skipped heading levels, a header row on every table, a declared language on every fence, and no raw HTML, inline style, script, custom colour, image carrying meaning or animated media anywhere. | `xtask/src/docs_copy.rs::tests::published_surfaces_carry_no_raw_html_or_media` and `::tests::heading_structure_is_navigable` (wrong implementations: an `<img>`/`<br>`/`<details>` tag on a packaged fixture — AP-7; two `#` H1s; an `##` following an `#` skipped to `###`; a fence with no language; a badge row above the status callout — AP-14) |
| **AC-009** | **GIVEN** that the first screen is **full** at `0.2.0` — five regions measuring ≈343 px against a 340 px budget at real Markdown block metrics (`_design.md` `## Mock`) — **WHEN** any of this story's copy lands, **THEN** the first screen still holds exactly five regions in the 14-line allocation (identity 3 + blank 1 + callout 3 + blank 1 + triad 6), the identity line is at most 2 rendered lines, the callout at most 2, the triad at most 6 with **exactly three** entries matching the three-crate decision, at most one primary-ranked element (one blockquote) sits above the fold, and if the budget is exceeded the fixed demotion order — badges, then the census, then compressing the identity line — is followed with the callout and the triad never yielding. | `xtask/src/docs_copy.rs::tests::first_screen_stays_within_budget` over the real file, using `_design.md` `## Density budget`'s stated model (24 px per rendered line, ~95 chars per line at 800 px, H1 ≈ 2.1 lines, H2 ≈ 2.8 lines including margins) with its limits documented in the module doc (wrong implementations: a fourth triad entry — AP-1 and a fourth published crate; a status callout grown back to three rendered lines; a sixth region above the fold); `::tests::one_primary_element_above_the_fold` (wrong implementation: a second blockquote or a table above the fold — AP-15) |

## Interaction quality

RFC §6.7/D6. Every invariant below is carried by an **AC-### row in the table above**, never by a
bullet here — `redkiln verify` extracts ACs by matching a leading `| AC-001 |` cell or an
`- AC-001:` bullet, so an invariant stated only as prose in this section would never be gated. This
section says **which** AC carries each invariant and how it is verified.

The medium translation is `_design.md`'s and the UX brief's, not invented here: the reader's
*interaction* is navigating a claim to its evidence and back inside one sitting, so *in place* means
0 hops, *revealed* means below the fold on the same page, and *opened on demand* means one link.

### STATE invariants

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** (IQ-1): every claim on the first screen is readable at 0 hops; its evidence is ≤ 1 hop and that hop lands on a specific anchor, never a repository root or *see the specification* | **AC-001** (the triad and the lead claim are on the page), **AC-002** (the census defines its own terms inline; the ledger is the single link), **AC-003** (the packaged peer sentence is 1 sentence + 1 link) | `tests::first_screen_regions_are_in_order`; `tests::census_appears_once_and_defines_its_terms`; `docs_copy` check (2), which is what makes *lands on a specific anchor* checkable |
| **Non-occlusion — a filter must not hide what it filters** (IQ-2, AP-4): no treatment lists frozen guarantees while suppressing the existence of the 49 provisional clauses; no state is rendered as an absence | **AC-002** (frozen-only is forbidden, not merely worse), **AC-004** (a crate that is deliberately out of the release train gets words, not a blank cell) | `tests::census_appears_once_and_defines_its_terms`; `tests::status_rows_carry_words_and_distinguish_states` |
| **Preserved position — a held anchor is a preserved scroll position** (IQ-3): `#status` and `#licence` survive, or every inbound reference moves in the same commit | **AC-006** | `docs_copy` checks (2) and (3), with the renamed-heading fixture as the wrong implementation |
| **Reversibility** (IQ-4): a yank leaves the rendered page exactly as it was, so every claim is worded to be *superseded by a later dated claim* — fact, scope, date — rather than silently rewritten | **AC-002** (dated and version-scoped), **AC-003** (*As of YYYY-MM-DD*, re-read at the publish commit), **AC-005** (true of the tree that shipped, so the successor supersedes rather than corrects) | `tests::census_appears_once_and_defines_its_terms` and `tests::prior_art_is_dated_prose` both assert the date's presence; the *re-read at the publish commit* half is a merge-DoD obligation, listed under **Risks** |
| **Reachable without running anything** (IQ-5): no claim's only evidence is *run our CI* or *clone the repo* | **AC-002** (the ledger is a reachable document), **AC-005** (the `description` is true standalone), **AC-006** (every evidence link resolves) | `docs_copy` checks (1)–(3); the *no build required* half is a review obligation, not a mechanical one, and is stated as such |
| **Keyboard reachability, in this medium: plain-text and heading navigability** — the decision must be reachable from the text alone, with images blocked and by heading traversal | **AC-008** | `tests::published_surfaces_carry_no_raw_html_or_media`, `tests::heading_structure_is_navigable` |

### COMPOSITION invariants

Taken from `_design.md`, which is **binding** — this story implements it and does not re-decide it.

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — no control ships as bare markup. Every status cell is glyph **and** words; every badge's meaning also exists in text; the triad's bullets lead with the audience in bold so the reader scans for themselves | **AC-004**, **AC-008**, **AC-001** | `tests::status_rows_carry_words_and_distinguish_states`; `tests::published_surfaces_carry_no_raw_html_or_media`; `tests::first_screen_regions_are_in_order` asserts the triad's bullet form, not merely its presence |
| **Composition and placement** — R1 → R2 → R3 in that order, nothing between them; badges below the census on packaged surfaces; the 8-row table on GitHub only; the packaging boundary decides which surface a region lives on at all | **AC-001**, **AC-004**, **AC-008** | `tests::first_screen_regions_are_in_order`; `tests::status_table_is_github_only`; `docs_copy` check (1) is the boundary's mechanical half |
| **Transience** — persistent chrome: identity, callout, triad (all 7 surfaces). Revealed on scroll: the census at 1024×768, the badges on crates.io. Opened on demand: the ledger, the full peer statement from a packaged surface, the status table from a packaged surface | **AC-001** (chrome), **AC-002** (revealed), **AC-003** and **AC-004** (opened on demand) | `tests::first_screen_regions_are_in_order` fixes what is chrome; `tests::status_table_is_github_only` and `tests::prior_art_is_dated_prose` fix what is one hop away |
| **Density budget, with its real numbers** — 340 px ≈ **14 rendered lines** at 1024×768; allocation 3 + 1 + 3 + 1 + 6; measured ≈343 px, so the screen is **full**; identity ≤ 2 lines, callout ≤ 2, triad ≤ 6 and ≤ 3 entries, census ≤ 3 lines with 4 counts and 1 link, `description` ≤ 120 chars, status Role cell ≤ 90 chars and Status cell glyph + 2–5 words | **AC-009** (first screen), **AC-002** (census), **AC-005** (`description`), **AC-004** (table cells) | `tests::first_screen_stays_within_budget` with the block model and its documented limits; `tests::description_is_within_budget_and_present`; `tests::status_rows_carry_words_and_distinguish_states` asserts the cell budgets |
| **Hierarchy** — at most one primary-ranked element per screenful; `#` H1 used once; the callout is the only blockquote in the first screen so the indent rail *is* the signal; the census is secondary prose with bold on the four numbers only; Prior art is recessive by design | **AC-009**, **AC-008**, **AC-002**, **AC-003** | `tests::one_primary_element_above_the_fold`; `tests::heading_structure_is_navigable`; `tests::census_appears_once_and_defines_its_terms` (no heading, bold on numbers only); `tests::prior_art_is_dated_prose` (no bold, no table) |
| **The design's named anti-patterns**, each bound to the AC that rejects it | AP-1 → AC-001/AC-009; AP-2 → AC-004; AP-3, AP-4 → AC-002; AP-5 → AC-003; AP-6 → AC-006; AP-7 → AC-008; AP-9 → AC-005; AP-11 → AC-004; AP-14 → AC-008; AP-15 → AC-009 | each named as the seeded wrong implementation in the verification column of the AC that carries it — which is what stops any of them being decorative |

**AP-8, AP-10, AP-12 and AP-13 are deliberately unclaimed here.** They belong to the slice-mates
(`guarantees-and-docs-rs-presentation` for AP-8 and AP-12, `compliance-claim-and-gaps-promise` for
AP-10 and AP-13). Claiming them would put two stories on one paragraph.

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | The census numbers in the copy disagree with `spec/SPECIFICATION.md`:219-222 at the commit under test | `docs_copy` check (5) **fails the gate**, naming both figures and the file it read them from. It never rounds, never prefers the copy, and never falls back to a hard-coded 200/139/49/10/2 |
| **EC-002** | `spec/SPECIFICATION.md`'s §1.3 census sentence cannot be parsed — reworded, renumbered, or the counts moved | **Fail closed with a stated reason**, in `.kb/decisions/0010-the-suite-must-prove-itself.md`'s discipline: *a skip is reported, never silent*. A parser that cannot find its input must not report green, because that is exactly the state in which the copy is unverified |
| **EC-003** | A relative link appears on a packaged surface, or an intra-document `#anchor` resolves to no heading in the same file | `docs_copy` checks (1)/(2) fail, naming the file, the line and the link text. A dead link on a published page renders as nothing visible, which is why this is mechanical (AP-6) |
| **EC-004** | `#status` or `#licence` no longer resolves in `README.md` | Check (3) fails. The escape hatch is not an allow-list: it is updating every inbound reference in the same commit, at which point the check is edited in the same commit too and the diff shows both halves together |
| **EC-005** | One of the six known-false strings is resurrected on any surface, by this story or by a slice-mate landing after it | Check (4) fails. This is the reason merge order puts this story **first** in the slice: the check exists before the copy that could reintroduce the strings |
| **EC-006** | A blocker has not landed — the crate set is not three, or PS-3 has no recorded verdict | **Stop and report; do not default.** The triad's entry count is a function of the crate set (`crate-set-decision`), and naming the projection port without its gate is a false claim (IQ-2). Writing copy against a guess is precisely the failure `_storymap.md`'s *decisions come first* ordering exists to prevent |
| **EC-007** | The first screen exceeds its budget once the copy is real | Apply the fixed demotion order — badges, then the census, then compressing the identity line to one sentence. If those three moves are not enough, something was added that should not have been: **delete the addition**, never the callout or the triad |
| **EC-008** | A README edit breaks the crate's own doctest (`crates/happenstance/src/lib.rs`:10 compiles `../README.md`'s fences) | `cargo test -p happenstance --doc` fails and the gate is red. This story does not intentionally touch the quick-start fence — it belongs to `guarantees-and-docs-rs-presentation` — so a failure here means an edit strayed outside the PR's stated intent |
| **EC-009** | `docs_copy` is written but not mounted | `steps_named` (`xtask/src/main.rs`:816-826) panics on a name that resolves to nothing, and the module doc at `:8-24` is reviewed in the same diff. An unmounted check is a function nobody calls, which is a decorative gate under `CLAUDE.md`'s rule |

## Non-functional

| id | Requirement | Why, and where it comes from |
| --- | --- | --- |
| **NF-001** | The new check is **file-reading only**: no network, no registry call, no rendered-Markdown dependency, and no new third-party crate in `xtask`'s manifest | It runs in the story gate on every diff. `xtask/src/lints.rs`'s five existing checks are the convention, and the testing brief's *Fixtures and seams to mock* forbids a network round-trip in a unit test |
| **NF-002** | It is **self-sufficient**: it parses §1.3 itself rather than consuming `clause-maturity-audit`'s output | `clause-maturity-audit` sits in `publish-time-gate-instruments`, which is unordered with this milestone (`_storymap.md`, *Merge order*). A dependency on it would make this story's gate green-or-red by accident of merge sequence |
| **NF-003** | Its module doc states, in prose, **what it does not verify** — that it cannot see how a host wraps a line, cannot know whether a link's target still exists on the far side of the internet, and cannot read the rendered page at all | `xtask/src/main.rs`:37-41 makes this the house rule: *a check whose limits are undocumented is read as a guarantee* |
| **NF-004** | Nothing on a published surface sets a colour, size, font or layout; both docs.rs themes and both crates.io themes stay legible because we add nothing | `_design.md` `## States` (*Theme*), `## Density budget` (*Minimum legible size*). A colour-coded meaning fails in at least one theme **permanently** for that version number |
| **NF-005** | Copy is written to `crates/*/README.md` **first**; the repo-root `README.md` is the third surface | `include_str!` cannot reach outside a package (`README.md`:131-137). Copy written only at the root never reaches a registry reader |
| **NF-006** | No decision atom, no `.kb/` edit, no `spec/SPECIFICATION.md` or `RUNBOOK.md` ledger edit in this PR | `CLAUDE.md`: hand-writing atoms produces the layout of the process without the process (reverted at `0269720`); atoms come from `/redkiln:kb-ingest`, and the ones this story consumes are already `first-contact-design-resolutions`' |
| **NF-007** | `redkiln validate --kb` and `redkiln doctor` stay clean at this story's checkpoint — exactly six `template-drift` advisories, zero `dependency-cycle` | Testing brief AC-TEST-003; `CLAUDE.md`'s backlog-CI assertion that the drift set is **exactly** those six |

## Implementation notes (non-prescriptive)

- **Write the crate READMEs first, then the root.** The packaging boundary is the placement rule, and
  the temptation to draft at the root and copy inward is exactly what produces AP-6.
- **Read the four census numbers from `spec/SPECIFICATION.md` at the commit you are on, not from this
  spec.** The figures quoted here (200/139/49/10/2) are true at the commit this spec was written
  against; the check exists because a number in prose is not a source of truth.
- **`lint_steps()` is a third mount worth weighing** (`xtask/src/main.rs`:799-808). It selects steps
  from `REQUIRED` **by name**, and `steps_named` panics on a name that resolves to nothing — so
  adding the copy check there also puts it behind `.redkiln/config.yaml`'s `reachability_static`
  (`cargo xtask lints && cargo xtask spec-trace`) with no way to mistype it silently. The Integration
  contract mandates the `REQUIRED` and `affected.rs` mounts; this one is a judgement call, and the
  argument for it is that `reachability_static` fires at an integration seam this story's slice-mates
  cross.
- **Reuse `package.rs`'s string-fixture seam** (`xtask/src/package.rs`:417-423's `METADATA` constant
  is the shape): synthetic surface strings for the wrong implementations, plus a small number of
  assertions that read the real files through the workspace root for the structural criteria.
- **The status table's per-row state is sourced, not inferred.** Each row's wording comes from the
  owning adapter project's committed report. Where no report exists, the row is state 3, 4 or 5 —
  never state 1. `cargo package -p happenstance-sqlite --list` exiting 0 is not evidence of anything
  except containment (`xtask/src/package.rs`:20-22).
- **Where you touch `xtask/src/main.rs`, append.** The `REQUIRED` list and the module doc at `:8-24`
  are edited by two stories in the sibling milestone as well; `_storymap.md` names that paragraph as
  a merge conflict by construction. Add an entry and a sentence; do not restructure either.
- **The date in the peer statement and the census is re-read at the publish commit.** Writing today's
  date and letting it ride is the failure DT-6's mitigation exists against — the nearest live peer
  shipped the day before intake.

## Tests and CI (merge gate)

Grounded in the testing brief (`_decomposition.md`, *Testing brief*), whose shape for this project is
**static checks and unit tests on `xtask`**, with the single true e2e reserved for AC-008's stranger
install in another story.

| Tier | Command / path | Proves |
| --- | --- | --- |
| unit (wrong-implementation) | `cargo test -p xtask` → `xtask/src/docs_copy.rs`'s `#[cfg(test)] mod tests` | Each of the five gate checks rejects the wrong implementation it names — a relative link on a packaged fixture, a dead intra-document anchor, a renamed Status heading, a resurrected AP-9 string, a seeded §1.3 disagreement. AC-TEST-002's shape, following `xtask/src/package.rs`:408-457 |
| unit (structural, over the real files) | `cargo test -p xtask` → `docs_copy::tests::first_screen_regions_are_in_order`, `::first_screen_stays_within_budget`, `::one_primary_element_above_the_fold`, `::census_appears_once_and_defines_its_terms`, `::status_rows_carry_words_and_distinguish_states`, `::status_table_is_github_only`, `::prior_art_is_dated_prose`, `::description_is_within_budget_and_present`, `::published_surfaces_carry_no_raw_html_or_media`, `::heading_structure_is_navigable` | AC-001, AC-002, AC-003, AC-004, AC-005, AC-008, AC-009 against the real surface files rather than a fixture |
| static (gate step) | `cargo xtask docs-copy` (new dispatch arm) | The five checks run standalone, which is how a failure is reproduced without re-running the whole gate |
| story gate | `cargo xtask affected --base main` (`.redkiln/config.yaml`'s `affected_gate`) | AC-007's unconditional mount: this diff is mostly Markdown and maps to few packages, so the file-reading block at `xtask/src/affected.rs`:116-125 is what gates it. It also compiles and tests `xtask`, which is affected because `xtask/src/` changed |
| doctest | `cargo test -p happenstance --doc` | EC-008: the crate compiles its own README's fences via `crates/happenstance/src/lib.rs`:10, so prose edits cannot silently break the example |
| integration gate (project grain) | `cargo xtask ci --fast` (`.redkiln/config.yaml`'s `integration_scoped`) | The new step is in `REQUIRED`, so `--fast` picks it up — it is not one of the four things `--fast` drops. Also fmt, clippy `-D warnings`, the four `wasm32` steps, docs, `spec-trace`, `package-check` |
| static (containment) | `cargo xtask package-check` (inside the gate) | `REQUIRED_FILES` — each publishable crate's `.crate` still contains both licences and its README after the README edits (`xtask/src/package.rs`:88-94). Containment only; presentation is never proved here (`:4-18`) |
| process | `redkiln validate --kb && redkiln doctor` | NF-006/NF-007: this story authored no atom, and the advisory set is still exactly six `template-drift`, zero `dependency-cycle` |
| **not this story's tier** | the rendered crates.io and docs.rs pages, read by a human | Deferred to `rendered-page-preflight` (AC-007). `xtask/src/package.rs`:4-18 is explicit that no automated check substitutes for it, and the testing brief keeps AC-007 as human observation on purpose |

**Merge DoD**: `cargo xtask affected --base main` green; every AC above observed on the source files;
each of the five checks demonstrated failing on its seeded wrong implementation; and the two dates
(census, peer statement) re-read at the commit being merged rather than inherited from this spec.

## Risks and coupling (PR-scoped)

| Risk | Likelihood / impact | Mitigation, in this PR |
| --- | --- | --- |
| **`xtask/src/main.rs`'s `REQUIRED` list and module doc are edited by four stories across two milestones** | High / Medium | Append one `Step` and one sentence; never reorder or restructure. `_storymap.md` names this paragraph as a merge conflict by construction, which is why the instruction is in the PR boundary as well as here |
| **A slice-mate reintroduces a corrected sentence or a relative link while writing its own copy** | Medium / High | This story is merge order **3.1**, first in the slice, precisely so the mechanical check exists before the copy that could break it. That is the whole argument for the ordering |
| **The four census numbers go stale between this commit and the publish commit** | Medium / High (permanent on a published page) | Check (5) reconciles against `spec/SPECIFICATION.md` on every gate run, so the numbers cannot drift silently; and the sentence is dated and version-scoped so a later release supersedes rather than corrects |
| **The peer statement's date is inherited rather than re-read** | Medium / Medium | It is a merge-DoD item above, not merely prose guidance. The nearest live peer shipped the day before intake, so an inherited date is likely to be wrong on arrival |
| **A blocker's answer changes after the copy is written** — the crate set moves, or PS-3 resolves the other way | Low / High | The triad's entry count and the port's naming are the two places a change would land; both are single sentences, and EC-006 makes *stop and report* the required behaviour rather than a default |
| **The first screen has ≈3 px of headroom** | High / Medium | AC-009 makes the budget a gated assertion rather than an aspiration, and the demotion order is fixed and not the implementer's to reorder |
| **The structural tests read real files and could become brittle to innocent rewording** | Medium / Low | Assert on **structure** — block order, entry counts, presence of a date, glyph-plus-words — never on exact sentences. A test that pins prose would make every future edit a test edit, which is how a check becomes something people delete |
| **`_design.md`'s `design/reference/*.png` frames do not exist** | Certain / Low | Named in *Clarifications* rather than worked around. The signed-off `design/mock.html` is the picture this story compares against; the PNGs are `rendered-page-preflight`'s drop location |
| **Coupling to `spec/SPECIFICATION.md`'s §1.3 wording** | Medium / Medium | The parser composes with `xtask/src/spec_trace.rs`'s existing approach (§1.3 is checked, never generated — `:39-57`) and fails closed with a stated reason when its input is unrecognisable (EC-002), rather than passing |

## Dependencies

**Blocks on** (must be merged first — matches this story's `depends_on`):

| Story slug | What this story consumes from it |
| --- | --- |
| `first-contact-design-resolutions` | DT-1's lead claim, DT-5's census treatment, DT-6's peer statement, and the evaluator-persona resolution. This story carries them onto the surface; it re-decides none of them and authors no atom |
| `crate-set-decision` | Three crates, therefore **exactly three** triad entries and a status table that names the unpublished crates in state 4 or state 5 words rather than omitting them |
| `projection-port-ship-shape` | Whether the projection port is frozen or gated, so the copy can name it **with its gate**. The `doc(cfg)` treatment and the manifest block belong to the slice-mate, not here |

**Unlocks**:

| Story slug | What it takes from this story |
| --- | --- |
| `compliance-claim-and-gaps-promise` | The same four surfaces, with the mechanical check already in place to catch a relative link or a resurrected stale string in R6 and the DT-4 bullet |
| `guarantees-and-docs-rs-presentation` | The same, for R9's Guarantees bullets and the docs.rs manifest block; also the intact quick-start fence this story leaves untouched |
| `rendered-page-preflight` | A surface worth reading, and a class of defect (dead links, held anchors, stale strings, census drift) already excluded mechanically so the human read spends itself on what only a human can see |
| `publish-0-2-0` | Copy that is true of the tree being published, on the one act that cannot be undone |

## Anchors (progressive disclosure)

Load-bearing depth is **deferred, not optional**. Each row says why the artifact is load-bearing and
the moment to open it. Every path was confirmed to exist.

| Anchor | Why it is load-bearing | When to open it | Serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` | The **binding** signed-off design: `## Composition` (R1–R12), `## Density budget` (the 14-line allocation and the fixed demotion order), `## Hierarchy`, `## Transience policy`, `## States`, `## Anti-patterns` AP-1…AP-15. This story implements it and may not contradict it | Before writing the first line of copy, and again before deciding anything about placement or emphasis | AC-001, AC-002, AC-003, AC-004, AC-008, AC-009 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/design/mock.html` | The 51-frame sign-off mock, dimensioned to the density budget with the fold rule drawn on each clipped frame. It is the picture the composition ACs are checked against, since the published page does not exist yet | When implementing AC-001 and AC-009, to see where the fold actually falls before counting lines | AC-001, AC-009 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` | The UX brief's five status states and the four named stale strings, IQ-1…IQ-7, and the testing brief's per-AC instrument table and its *name the wrong implementation* rule | Before writing the status table (AC-004) and before writing any `docs_copy` test (AC-007) | AC-004, AC-005, AC-007 |
| `spec/SPECIFICATION.md` | §1.3's census at `:219-222` — the source of the four numbers — and `:213-217`, why a provisional marker with no falsifier is a defect, and `:210-211`, the house precedent for retaining an identifier so citations resolve | When writing the census sentence and when implementing check (5) and check (3) | AC-002, AC-006 |
| `crates/happenstance/README.md` | The primary packaged surface and the mount point. `:6-11` is a stale string to correct, `:13-20` the triad's existing form, `:53-56` the orphan *maturity marker* defect | First, before any other file — the packaging boundary makes this the surface that reaches a registry reader | AC-001, AC-002, AC-005 |
| `crates/happenstance-core/README.md` | Secondary packaged surface; `:8-9` carries a sixth false-at-publish sentence this spec adds, and `:14-22` is the triad pointing *away* to `happenstance` | While correcting the false sentences (AC-005) and mirroring the skeleton | AC-005, AC-001 |
| `crates/happenstance-testkit/README.md` | Secondary packaged surface; `:16-17` is the other sentence this spec adds, and `:9-22` is the sharper callout that lets the census sit below the fold here | Same moment as the file above | AC-005, AC-002 |
| `README.md` | The GitHub landing surface: `:11-16` status callout, `:79-98` the 8-row table, `:83-90` the glyph-and-words rows to preserve, `:104-109` the stale quick-start prose, `:180` the model peer sentence, `:218-225` the Prior art slot, `:9`/`:16` the two held anchors, `:131-137` why the root is the third surface | After the three crate READMEs, when correcting the table, the quick start and the Prior art date | AC-003, AC-004, AC-005, AC-006 |
| `crates/happenstance/Cargo.toml` | `:3` the 131-character `description` to rewrite; `:12` `readme = "README.md"`, the wire that makes the README a published surface at all | When implementing AC-005; note the slice-mate appends `[package.metadata.docs.rs]` to the same file | AC-005 |
| `xtask/src/main.rs` | `:8-24` the module doc that states in prose what the gate proves; `:72-103` `Step` and what `probe: None` means; `:105` where `REQUIRED` opens; `:515-532` `package-check`'s entry as the model; `:799-826` `lint_steps`/`steps_named`, whose panic makes an unmounted step impossible to land quietly | Before mounting the check — read the `Step` doc first so `probe: None` is a deliberate choice rather than a copy | AC-007 |
| `xtask/src/affected.rs` | `:116-125`, the unconditional file-reading block. A Markdown-only diff maps to no package, so this is the only place a copy check can be gated at story grain | At the same moment as the file above; the two mounts are one obligation | AC-007 |
| `xtask/src/package.rs` | `:4-18` why containment is not presentation, `:20-22` the live *compiles but has not passed the suite* instance, `:88-94` `REQUIRED_FILES`, `:408-457` the `#[cfg(test)] mod tests` shape (fixture string, named wrong implementation, assertion that it is rejected) | Before writing any test in `docs_copy` (AC-007) and before wording the status table's state-3 row (AC-004) | AC-004, AC-007 |
| `xtask/src/lints.rs` | The five existing file-reading lints, each documenting what it does *not* verify. The new module adopts that convention rather than inventing one | While writing `docs_copy`'s module doc (NF-003) | AC-007 |
| `xtask/src/spec_trace.rs` | `:39-57` — why §1.3 is checked and never generated. Check (5) composes with this posture instead of writing a second parser or generating the census | When implementing check (5) | AC-002, AC-007 |
| `.kb/decisions/0006-bare-name-to-the-typed-layer.md` | ADR-0006: the crate split is by **role** (application author vs adapter author), not by runtime — which is what the triad encodes and why DT-1's option (c) is structurally incoherent | Before writing the triad's three entries, if the temptation to split by runtime appears | AC-001 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | *A declined capability still runs the rule and reports the stated reason, never vanishes* — the same discipline this story applies to prose, and the model for EC-002's fail-closed behaviour | When wording an absence (AC-002, AC-004) and when deciding what check (5) does with an unparseable input | AC-002, AC-004 |
| `standards/rust/51-features-and-no-std.md` | `:226-231` — a yank removes a version from resolution and leaves every rendered page exactly as it was. The reason every claim here is dated and supersedable rather than editable | Before wording any claim that will need to change in a later release | AC-002, AC-003 |
| `standards/rust/70-rustdoc-obligations.md` | RS-70-5 at `:243-313` — *name the alternative that lost, once*. The house voice for every claim on this surface, and the register the peer sentences are written in | While writing the peer statement and the status vocabulary | AC-003, AC-004 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | Persona 4 at `:249-313`: one sitting, cannot run the suite, compares against named peers; `:333-338` why they are one-shot in a way the other three are not; `:349-355` why not to over-derive from them | Before AC-001's copy, to keep the lead claim aimed at the moment rather than at a feature list | AC-001, AC-003 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md` | `:64` this story's row and one-line slice; the merge order at `:151-184` that puts this story first in its slice, and the reason | When sequencing against the slice-mates, and before touching a file a sibling also edits | AC-007 |
| `.redkiln/config.yaml` | The `verify:` block — which command fires at which grain, and what `--fast` drops. It is why the `REQUIRED` mount is sufficient and a probe would be an escape hatch | When mounting the step and when reading the Tests and CI table | AC-007 |
| `CLAUDE.md` | *An adapter that compiles but has not run the suite is not an adapter* — the sentence the status vocabulary exists to honour; and *a rule that no adapter can fail is decorative*, the rule every seeded wrong implementation discharges | Before writing the status rows and before writing the tests | AC-004, AC-007 |

## Clarifications resolved during spec

1. **The AC set is exactly the nine the first pass decided** — AC-001…AC-009. Nothing added, nothing
   dropped; the ledger matches one row per id.
2. **Five gate checks, and structural assertions on top of them.** The front half fixed
   `docs_copy`'s gate checks at **five**, and that number stands. The composition criteria (AC-001,
   AC-009) and several truth criteria still need to be gated, so they ride as `#[test]`s in the same
   module that read the **real** surface files through the workspace root — not as a sixth check.
   The distinction is real: a gate check is a step a maintainer runs and reproduces standalone; a
   test fails through `cargo test -p xtask`, which the story gate runs because this diff touches
   `xtask/src/`. Both are mechanical, and neither is a human read.
3. **The rendered-page comparison target is `design/mock.html`, not `design/reference/*.png`.**
   `_design.md` tables seven reference captures; the directory contains **only** `mock.html` today.
   That is not a defect in the design — capture is manual and those paths are where
   `rendered-page-preflight` drops its reads (`_design.md`, `## Mock`). This story therefore compares
   against the signed-off mock frame and records that it did.
4. **Two false-at-publish sentences beyond the UX brief's four** are in scope here, as the front half
   recorded: `crates/happenstance-core/README.md`:8-9 and `crates/happenstance-testkit/README.md`:16-17.
   Both are status claims on packaged surfaces, so leaving them to no story would leave IQ-7 half
   discharged on two of the three crates a reader can `cargo add`.
5. **`lint_steps()` is offered as a third mount, not mandated.** The Integration contract binds the
   `REQUIRED` and `affected.rs` mounts. Adding the step's name to `lint_steps()` additionally puts it
   behind `reachability_static`; `steps_named`'s panic makes a typo impossible to land. Left to the
   implementer with the argument stated, because it changes which seam the check fires at and that is
   a judgement about the gate rather than about the copy.
6. **AP-8, AP-10, AP-12 and AP-13 are not this story's.** They map to the slice-mates' regions (R6,
   R7, R9 and the `doc(cfg)` treatment). Claiming them here would put two stories on one paragraph,
   which is the failure the slice boundary in the Context pack exists to prevent.
7. **No conformance rule, no clause, no atom.** Restated because it is the most likely accidental
   scope creep on a story that touches `xtask`: the instrument added here is a file-reading gate
   check, not a conformance rule, and `happenstance_testkit::event_store_conformance!` is out of
   scope (testing brief, *Fixtures and seams to mock*).
