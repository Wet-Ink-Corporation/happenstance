---
item: HS-P0020
stage: design
created: "2026-08-17T05:10:00.000Z"
updated: "2026-08-17T05:10:00.000Z"
---

# API surface design — The Checked Documentation Surface

This project has a surface, and it is not the one the template's preamble expects. It adds
no `pub` item to any publishable crate (`_decomposition.md`, `## Deployment brief`: *"none
of this project's changes touch their public API"*). What it does add is **two rendered
things a person reads**: markdown pages in a tree the gate compiles, and the gate's own
output when that compile fails. Both are composed surfaces with a hierarchy, a density
budget and a set of states, and no stage before this one was permitted to decide any of it.

So the template's API sections are answered honestly and briefly, and the interaction
sections carry the weight. `design.capture` is deliberately undeclared in
`.redkiln/config.yaml` (verified: a comment at line 15 and an explanatory note at 75–81, no
active `design:` block), which makes the perceptual review a permanent skip — **this file is
the only record these decisions will ever have.**

**What this design is composed from, and what does not exist.** There is no design system in
this repository, and this is verified rather than assumed: no `book.toml`, no `*.css`, no
`*.scss` anywhere in the tree; `docs/` holds exactly one file, `docs/README.md`. Every
visual primitive on the markdown side is owned by whatever renders the repository's
markdown and by rustdoc's shipped theme; this project ships no theme, no stylesheet and no
plugin, and `## Anti-patterns` forbids introducing one. The one place a real, verified,
in-repo primitive inventory *does* exist is the gate's terminal output, and this design
composes from it exactly:

| Primitive | Verified at | Form |
| --- | --- | --- |
| Step banner | `xtask/src/main.rs:864` | `\n=== {step name} ===` on stdout |
| Probe skip line | `xtask/src/main.rs:876` | `skipped: \`{probe}\` did not succeed` |
| Success summary | `xtask/src/lint_constitution.rs:192` | `  {n} atoms, all consistent` — two-space indent |
| Problem line | `xtask/src/lint_constitution.rs:605-660` | `  {path}:{line} — {message}` on stderr |
| Terminal count | `xtask/src/lint_constitution.rs:199` | `bail!("{n} problem(s) in {DIR}")` |
| Vacuity guard | `xtask/src/lint_constitution.rs:175-177` | `bail!` on an empty corpus, before any check runs |
| Routing table | `docs/README.md:12-24` | two columns, `\| Looking for \| It is at \|` |

Nothing below names a primitive outside that table, rustdoc's own shipped affordances, or
plain CommonMark.

## Items

No public API is added, changed or removed. One crate-internal item and four pinned
constants are added; they are listed because they are the surface a *contributor* writes
against, and because the constants are the mechanism by which every form decision below is
enforced rather than merely recommended.

```yaml
- path: "xtask::spec_trace::clause_ids"
  kind: "fn"
  change: "added"
  feature: "default"           # xtask is publish = false; no feature gates it
  clause: "n/a — reads SPECIFICATION.md, discharges no clause"
- path: "xtask::narrative::TREE"
  kind: "const"
  change: "added"
  feature: "default"
  clause: "n/a — AC-001, the pinned path"
- path: "xtask::narrative::HARNESS"
  kind: "const"
  change: "added"
  feature: "default"
  clause: "n/a — AC-005, the registration bridge"
- path: "xtask::narrative::IGNORE_ALLOWANCES"
  kind: "const"
  change: "added"
  feature: "default"
  clause: "n/a — AC-004, the enumerated allowance list"
- path: "xtask::narrative::HIDDEN_MARKERS"
  kind: "const"
  change: "added"
  feature: "default"
  clause: "n/a — AC-006, DT-7's answer as a rejected token set"
```

`HIDDEN_MARKERS` is this file's decision made executable. Everything else in `## Pattern
decision` would be a preference without it.

## Signatures

```rust
/// The clause ids `spec/SPECIFICATION.md` defines, resolved through the existing parser.
///
/// Sibling of `all_rules`, in the same file and for the same stated reason: a list kept
/// next to the function that parses it cannot drift from it (`spec_trace.rs:83-84`).
pub(crate) fn clause_ids(root: &std::path::Path) -> anyhow::Result<std::collections::BTreeSet<String>>;

/// The narrative tree, pinned by path. Moving it without editing this line fails the gate.
const TREE: &str = "docs";

/// The lib-crate harness whose `include_str!` lines register every page.
const HARNESS: &str = "xtask/src/narrative.rs";

/// Fences permitted to opt out of the compiler, enumerated: (page path, line-or-anchor, reason).
/// Swept in reverse — an allowance naming a fence that no longer exists is itself a problem.
const IGNORE_ALLOWANCES: &[(&str, &str, &str)] = &[];

/// Hidden-content markers rejected anywhere under `TREE`. DT-7's resolution, as a token set.
/// There is deliberately no allowance list for these; see `_design.md`, `## Pattern decision`.
const HIDDEN_MARKERS: &[&str] = &[
    "<details", "<summary", "{{#tabs", "{{#tab ", "{{#endtabs", "```admonish", "<!-- tab",
];
```

## Surfaces

Six surfaces. Two are markdown a reader meets, two are the gate's output a contributor
meets, one is the index that connects the first two, and one is recorded because this
project deliberately does **not** touch it.

`selector` is the isolating locator *in that surface's own medium*. For the markdown
surfaces it is `null` and that is a finding, not an omission: this repository controls no
DOM — rustdoc and whatever renders the repository's markdown generate their own markup, and
naming a class here would be inventing a primitive. `design.capture` is undeclared, so no
harness will address this block; it is the manifest of what exists, and the design review
reads it by opening the paths.

```yaml
- id: narrative-tree-index
  route: "docs/README.md"
  selector: null                 # no repo-controlled DOM; the host renders the markdown
  pattern: "two-column routing table, extended (docs/README.md:12-24)"
  states: [populated, empty-tree, long-label, narrow-70col]

- id: narrative-page
  route: "docs/<page>.md"
  selector: null
  pattern: "always-visible single-need page; no hidden panels (DT-7 (b))"
  states: [default, scoped-inline, long-page-overflow, long-label, narrow-70col]

- id: narrative-scoped-page
  route: "docs/<topic>/<scope>.md"
  selector: null
  pattern: "one page per scope, reached from the index (DT-7 (c))"
  states: [default, orphan-unregistered, narrow-70col]

- id: gate-narrative-compile-step
  route: "cargo xtask ci -> === the narrative tree's examples compile ==="
  selector: "stdout banner (main.rs:864) + rustdoc's doctest report"
  pattern: "one #[cfg(doctest)] module per page, so a failure names the page"
  states: [pass, fail-broken-fence, fail-removed-item, fail-many]

- id: gate-narrative-checker-step
  route: "cargo xtask narrative  (same step inside cargo xtask ci)"
  selector: "stderr problem lines, two-space indent (lint_constitution.rs:189-198)"
  pattern: "accumulate every problem, report in source order, count last"
  states: [pass, fail-one, fail-many, fail-empty-tree, fail-hidden-marker, fail-long-path]

- id: rustdoc-reference-surface
  route: "target/doc/happenstance/index.html  (docs.rs after publish)"
  selector: null                 # rustdoc's own shipped theme, search and item collapse
  pattern: "unchanged — this project adds no affordance to it"
  states: [unchanged]
```

## Pattern decision

Two decisions. The first is the hosting/render shape AC-009 and DR-10 leave to this file;
the second is DT-7, the only interaction tension assigned to HS-P0020.

### D1 — Hosting and render shape: **the markdown is the render**

**Chosen.** The pinned tree is `docs/`, repurposed (Deployment brief, Option A). The
"render" is the markdown itself, as the repository's own host renders it, plus rustdoc for
the reference layer. **No second rendered surface is built by anything** — no mdBook, no
static site, no theme, no plugin, no `book/` output directory.

| Rejected | Why it lost |
| --- | --- |
| **mdBook static site** (Deployment brief, Option B) | It is an external binary, therefore a `probe:` shape, and `probe: Some(...)` means *skip when absent* (`main.rs:89-102`) — which DR-03 forbids for the compiling step and which the architecture brief already forecloses (Note 3). Wiring it `probe: None` instead breaks AC-009's clean checkout. Building it as a presentation-only step *outside* the mandatory gate reproduces `RUNBOOK.md:918-925` exactly: a step that printed `skipped` on all three runners while two documents vouched for it. Verified from-zero cost: no `book.toml` and no stylesheet exists anywhere in the tree. |
| **Render narrative pages into a published crate's rustdoc** (`#[doc = include_str!]` on a `pub mod guide` in `crates/happenstance/`) | This is the one shape that would put narrative prose on docs.rs, and it loses on a mechanism this repository has already written down: `include_str!` resolves against the file tree, so a published crate carrying a path outside its own package "would fail `cargo test` for anyone who ran it" (`xtask/src/lib.rs:8-16`). Adopting it means moving the tree inside `crates/happenstance/`, which contradicts the architecture brief's CR-1 pinning of the harness to `xtask/src/lib.rs`. **Deferred, not dead** — see `## Open questions`. |
| **A published site (Pages, or a `cargo doc` bundle) as the canonical reader surface** | Same tool problem, plus an artifact that can diverge from the bytes CI checked. |

**What the choice buys.** The UX brief's fifth falsifier is *"the narrative surface requires
a manual build step, so a clean checkout renders something different from what CI checked."*
Under this decision that is not guarded by a check — it is unfalsifiable by construction,
because the reader reads the same bytes rustdoc compiled. There is no render step to skip.

**What it costs, stated rather than hidden.** No sidebar TOC, no search across narrative
pages, no prev/next. Those are precisely the affordances mdBook gives for free, and giving
them up is the price of the paragraph above. The mitigations are structural, not
compensatory: the index (`docs/README.md`) carries the ordering explicitly in the routing
table it already has, and the density budget caps pages short enough that a per-page TOC is
not the missing thing. Hand-rolling either is forbidden below — the dossier's own rule is
that before adding a navigation affordance you confirm the medium does not already render
the equivalent (`interaction-patterns.md:436-441`), and the honest reading here is that the
gap is a missing *link*, not a missing widget.

**Consequences that must land in the same change** (architecture brief, Note 9 and CR-5):
`docs/` is on `affected.rs`'s `INERT` list (`affected.rs:250`) and
`a_docs_only_change_selects_nothing` (`affected.rs:650-654`) asserts it, so the selection arm
must be extended with tests in both directions, and that assertion re-pointed at a path that
is still genuinely inert. `docs/README.md:25-29` states the pin-by-path rule and lists the
trees the gate reads; it becomes false unless updated in the same commit.

### D2 — DT-7, hidden panels: **(b) always-visible by default, (c) a page per scope past a stated size, and (a) mechanically forbidden**

> DT-7 — *whether adapter- or feature-scoped content uses hidden panels. (a) tabs or folds;
> (b) always-visible, longer pages; (c) separate pages per scope.*
> (`initiative.md:491`; AC-006; story `hidden-content-resolution`.)

**Chosen.** Scoped divergence is written as **visible level-3 subsections under one level-2
heading** while it stays small, and becomes **one page per scope** past the threshold below.
Hidden panels are rejected by the checker, by file and line, using `HIDDEN_MARKERS` in the
same fence walk as AC-004 — the enforcement site the architecture brief and `_storymap.md`
both name.

**The threshold, so this is checkable rather than tasteful.** Scoped content stays inline
while it is **≤ 3 scopes** *and* **≤ 25 rendered lines per scope**; past either bound it
splits into one page per scope. Six-plus adapters therefore land in (c) by construction,
which is where the fanout was always pointing.

**Why (a) loses, including the conditional version of it.** The dossier is explicit that the
fanout suits tabs and that this is why the pattern is tempting rather than obviously wrong
(`interaction-patterns.md:218-225`). It is equally explicit that whether an inactive panel's
code sample reaches `mdbook test`, the search index, or Ctrl-F/print is **stated nowhere in
the plugin's own documentation** (`:213-216`) — an unverified property, not a confirmed safe
one. Three consequences settle it:

1. **D1 removes the mechanism.** With no mdBook there is no `mdbook-tabs`. The only
   disclosure construct plain markdown offers is `<details>`/`<summary>`, and adopting it
   would mean betting the checked surface on rustdoc's doctest extractor reaching inside a
   raw-HTML block — a property no tool documents either.
2. **Even a passing falsification would not hold.** AC-006 permits (a) if a claim broken
   inside a non-default panel is *observed* to fail the gate. That observation establishes
   the property for one construct, one extractor and one toolchain, and nothing would notice
   it regressing: the fixture page keeps passing whichever way extraction goes. That is a
   rule no implementation can fail, which is the decorative-check defect CLAUDE.md names for
   conformance rules, one level up. Rejecting the markers instead gives a rule with a named
   wrong implementation — a page carrying a `<details>` fold — that fails today, on every
   runner, deterministically.
3. **Compilation is only one of three unverified properties.** Search, Ctrl-F and print stay
   unverified even if the compile question resolved. A reader who cannot find a constraint
   is in the same position as a reader taught a stale one.

Behind all three is the in-house precedent the whole project exists to not repeat
(`RUNBOOK.md:918-925`), and the code-layer version of it the dossier cites: an invariant
stated once visibly and once behind a fold, with nothing catching the drift
(`interaction-patterns.md:244-252`).

**No allowance list for hidden markers.** AC-004's enumerated allowance list exists because
the need for an uncompiled fence is real and enumerable. A hidden-panel allowance would be
permission to reintroduce an unverified property one page at a time, and the reverse sweep
that makes the `ignore` list safe cannot help: a stale allowance is detectable, an
unverified mechanism is not.

**Mitigations for the chosen patterns' own documented failure modes.** Choosing a pattern
while ignoring its known failure mode is the defect this section exists to prevent, and both
options carry one.

- **(b)'s failure mode** — conflating hierarchical progressive disclosure with staged
  disclosure, so a page a reader should skim becomes one they must walk start to end
  (`interaction-patterns.md:185-189`). *Mitigation:* scoped subsections are level-3 under one
  level-2 heading, in a fixed alphabetical-by-crate-name order, each self-contained. The band
  is a menu, not a sequence, and its order carries no meaning a reader must infer.
- **(c)'s failure mode** — the axum counter-case: a flat folder with no ordering and no
  audience, and a contributor who "was not sure in which order to try to read them"
  (`interaction-patterns.md:99-104`). *Mitigation:* a scoped page is never reachable only by
  directory listing. It is a row in the index's routing table with its scope in the label,
  its H1 names the scope, and it links back exactly once to the adapter-agnostic page it
  specialises. That back-link is the only cross-page navigation this project authors.

**What this does not decide, and must not.** DT-8 — where the line falls between a safe
aside and a load-bearing constraint — is HS-P0021's (`initiative.md:492`). A blanket marker
ban makes DT-8 moot *inside the pinned tree*, and that interaction is stated here rather than
discovered: HS-P0021's rule still governs every surface outside it, and if that project finds
a genuinely non-normative use for disclosure it petitions by adding an allowance shaped like
`IGNORE_ALLOWANCES`, in its own change, with its own falsification. It does not get one by
default.

## Composition

Regions in order, and where each control sits relative to the others.

### `narrative-page` — the page a reader meets

Source order is render order; there is no layout engine to arrange anything else.

1. **H1 title.** One line. ≤ 40 characters (see `## Density budget` — the index's left column
   is what bounds it).
2. **The answered-need slot.** The single line immediately under the H1 is **reserved** and
   left to HS-P0021: BR-04's named answered-need is that project's discipline (DT-2/DT-8),
   and this project owns the machine, not the rule. Fixture pages carry a placeholder line
   here; the checker does not read it. Reserving the slot rather than filling it is the whole
   seam between the two projects, made positional.
3. **The claim band.** Prose, with `SPECIFICATION.md` clause ids inline in the sentence that
   depends on them — not collected in a footer. AC-007 resolves them wherever they sit, and
   a citation next to its claim is one a reviewer can check without scrolling.
4. **The fence band.** Fenced `rust` blocks, interleaved with the claim band rather than
   gathered at the end. Each fence is preceded by the sentence it demonstrates, because the
   step's headline blind spot is code that still compiles while no longer demonstrating the
   surrounding claim (architecture brief, Note 10 item 1) — adjacency is the only thing that
   makes that blind spot reviewable by a human.
5. **The scope band, if any.** One level-2 heading, level-3 subsections beneath it in
   alphabetical order — or, past the threshold, a single paragraph linking out to the scoped
   pages. Always last before the closing pointer: scoped content is the part most readers
   skip, and putting it above the fence band would make every reader walk past six adapters
   to reach the example.
6. **The closing pointer.** At most one link outward. Its *policy* is DT-10 and belongs to
   HS-P0023; what this file fixes is that there is one slot for it and it is at the bottom.

### `narrative-tree-index` — `docs/README.md`

The file already exists and already has the shape (verified, `docs/README.md:12-24`). It
gains rows; it does not gain regions.

1. The existing statement of what the directory is for.
2. **The narrative table** — the two-column routing table, extended: left column the page's
   name (which is its H1), right column the link. Scoped pages appear as their own rows with
   the scope in the label, never as a nested list under a parent row.
3. The existing "it is at" table pointing out of `docs/`.
4. The existing paragraph at `:25-29` naming the trees the gate reads by path — **updated in
   the same change** to name the narrative tree, or it becomes false.

Ordering rule: the narrative table sits **above** the pointer-out table. A reader who opened
`docs/` wants the pages; a reader who wants the specification is being redirected, and
redirection goes second.

### `gate-narrative-checker-step` — the failure report

The composition a contributor meets, assembled entirely from the verified primitives above.

```
=== the narrative tree's examples compile ===            ← main.rs:864, stdout
<rustdoc's own doctest report>

=== every narrative page is checked ===                  ← the checker's own banner
  docs/adapters/sqlite.md:41 — `<details` is a hidden panel; DT-7 forbids it in docs
  docs/append-conditions.md:12 — cites `ES-99`, which SPECIFICATION.md does not define
  docs/append-conditions.md:88 — an untagged fence is compiled as Rust; tag it `rust` or `text`
  3 problem(s) in docs

xtask failed: 3 problem(s) in docs                       ← main.rs:712, from the step
Error: the narrative pages check failed with exit status: 1   ← main.rs:887, from the parent
```

Three composition rules, each with a reason:

- **Two steps, two banners.** The compile and the checker are separate `REQUIRED` steps so a
  reader can tell from the banner alone which half failed. Sharing a banner with the
  constitution's step would make one name answer two questions (architecture brief, Note 3).
- **Every problem, in source order, before the count.** `lint_constitution::run` accumulates
  and prints all (`:169-198`); "a check that stops at the first problem turns one review
  cycle into six" (`_decomposition.md:218-219`). Source order — path then line — so the list
  reads in the same order as the tree a contributor is about to edit.
- **The count is last and carries the directory.** `bail!("{n} problem(s) in {TREE}")` is the
  only line that scrolls into view when the list is long, so it is the line that must say
  where to look.

## Transience policy

Every control, with why it is where it is. Nothing here is persistent because nobody decided
otherwise.

| Control | Policy | Why |
| --- | --- | --- |
| Page H1 | **Persistent chrome** | It is the page's identity and the index's left column; a page whose title only exists in the index is a page a reader cannot confirm they landed on. |
| Answered-need slot | **Persistent chrome (reserved, unfilled)** | Reserved by position so HS-P0021 can fill it without relayout. Empty in fixture pages; never repurposed. |
| Clause citations | **Persistent chrome, inline** | Deliberately *not* revealed on demand. A citation behind a control is a claim whose provenance a reviewer must act to see, and AC-007 exists because provenance is the thing that rots. |
| Fenced examples | **Persistent chrome** | The checked artifact. Anything that can hide one can hide the only part of the page the gate reads. |
| Scoped subsections | **Persistent chrome while inline; a separate page past the threshold** | The two states of DT-7's answer. There is no third state where they are present but hidden. |
| Back-link on a scoped page | **Persistent chrome, exactly one** | (c)'s mitigation. One link, at a fixed position, is a link; several become the bespoke navigation widget the dossier forbids. |
| Closing pointer outward | **Persistent chrome, at most one slot** | Slot fixed here; policy is DT-10, HS-P0023's. |
| Table of contents, breadcrumb, prev/next | **Never present** | Whatever renders the markdown already supplies outline navigation, or the page is short enough not to need one. Hand-rolling it is the dossier's own anti-pattern (`:436-441`). |
| Playground / "Run" button | **Never present** | It compiles a different, unconnected copy of the code, not the one the gate checked (`interaction-patterns.md:80-84`). |
| rustdoc's item collapse/expand, search, per-item nav | **Persistent, inherited, untouched** | It ships with the medium and costs nothing per adapter added (`interaction-patterns.md:57-59`). This project adds nothing to it and removes nothing from it. |
| Step banner | **Persistent chrome** | Printed before the work starts, so it is also the loading state (see `## States`). |
| Problem lines | **Opened on demand — by failing** | They exist only in the failure state. A check that narrates its progress on success makes the one line that matters harder to find. |
| Success summary | **Persistent chrome, one line** | `  {n} pages, all consistent`. One line, because a green check that says nothing is indistinguishable from a check that did not run — the `RUNBOOK.md:918-925` shape — and a green check that says ten lines trains people to skip it. |
| Probe skip line | **Never present** | `probe: None`. Structural, not stylistic: DR-03. |

## Density budget

Real numbers, derived from this repository's own corpus rather than chosen. The measurement
is reproducible: line-length distribution over `standards/rust/*.md`, prose and fence bands
separated — prose p50 69, p90 81, p99 150; fence p50 26, p90 74, p99 91, max 147; page
lengths min 127, median 269, max 409 source lines across 28 files.

### The markdown surfaces

| Budget | Value | Derivation / what it protects |
| --- | --- | --- |
| Narrow content width | **70 columns** | The 1024×768 case. Everything below is sized so nothing overflows it. |
| Wide content width | **~100 columns** | The 1440×900 case. Headroom, not a target — a page must not *need* it. |
| Fence line width | **≤ 80 columns** | Covers ~92% of the existing constitution corpus's fences unchanged (p90 74, p99 91), so it is a modest tightening of a real corpus rather than an invention. A fence wider than this scrolls horizontally at the narrow width, and a scrolling example is one a reader mis-copies. |
| Prose source wrap | **≤ 90 columns** | p90 of the existing prose is 81. Tables and single-token URLs are exempt — they cannot be wrapped without breaking them. |
| Page length | **≤ 250 source lines** | Below the constitution's median atom (269) and above its shortest (127): a narrative page may be as substantial as a reference atom, not more so. Past it, the page is answering more than one need (`interaction-patterns.md:397-403`) or carrying scoped content that belongs on a scoped page. |
| Inline scope band | **≤ 3 scopes × ≤ 25 rendered lines** | DT-7's threshold. 25 lines is about one screenful of the fence band at the narrow width; 3 keeps the band skimmable without a TOC. |
| Page H1 | **≤ 40 characters** | It is the index table's left column. Corrected from 60 by the mock's finding 4: at 70 columns, with the widest right-column cell at 22 characters and 7 characters of table overhead, the left column holds **41**, not 60. 60 is the figure for the ~100-column wide width, and the index is sized against the narrow width by the row above — so the budget drops rather than the index changing what it is sized against. |
| Index table | **exactly 2 columns** | What `docs/README.md:12-24` already uses, and the only table shape that survives 70 columns. A third column is the first thing that would force horizontal scroll. |

**What yields first when the budget is exceeded**, in order: (1) prose is cut — it is not the
checked artifact; (2) a scope band splits into scoped pages; (3) the page splits. **A fence
never yields.** It is never elided, never truncated with `…`, never wrapped mid-token, and
never replaced by a prose description of itself — every one of those turns the one
gate-checked thing on the page into prose wearing syntax highlighting
(`interaction-patterns.md:455-459`).

**Minimum legible size for the primary label.** The primary label is the page's H1 on the
markdown side and `{path}:{line}` on the terminal side, and neither has a font size this
project controls. The equivalent floor is that both must survive the narrow width intact: an
H1 ≤ 40 characters does not wrap in the index, and a location prefix ≤ 48 characters does not
wrap in an 80-column terminal.

**The location-prefix budget is measured on two surfaces, not one.** The mock's finding 3
showed the 48-character budget covers the checker's own output and misses the compile
surface: `cargo test -p xtask --doc -- --list` names a doctest
`xtask\src\../../<page> - narrative::<mod> (line N)`, and that 16-character
`xtask\src\../../` prefix is uncounted, so `docs/adapters/happenstance-cloudflare.md`
reaches 56 characters there while fitting the checker at 40. **Resolved: the budget is
≤ 48 characters *inclusive of the 16-character doctest prefix*, i.e. ≤ 32 characters of
repo-relative path.** The checker enforces the stricter number, because a path that fits
the compile surface necessarily fits the checker and the reverse is not true. This is why
anti-pattern 12 caps a page filename at 32 characters and forbids a third directory level.

### The terminal surface

| Budget | Value | Derivation / what it protects |
| --- | --- | --- |
| Line width | **80 columns** | The CI log's baseline. Longer lines soft-wrap rather than truncate, which is survivable — but only if the important part is first. |
| Location prefix | **≤ 48 characters** including `:{line}` | So the location is never pushed off the first visual row by soft-wrap. |
| Consequent path budget | **`docs/` + ≤ 2 directory levels, filename ≤ 32 chars** | Falls directly out of the line above, and is the reason it is a *gate* rule and not a style note: `docs/adapters/happenstance-cloudflare.md:1234` is 44 characters and fits; a third level would not. |
| Problem list | **unbounded — never truncated** | See `## States`, overflow. |
| Success output | **1 line** | |

## Hierarchy

Per region: primary / secondary / recessive, and what carries the distinction. Nothing here
is carried by colour, weight or size — the media do not offer those under this project's
control. Everything is carried by **position, heading level, and adjacency**, which are the
only three levers plain markdown and a terminal both have.

**`narrative-page`**

- *Primary* — the H1 and the fence band. Carried by heading level (there is exactly one H1)
  and by the fence's own visual block, which every renderer sets apart for free.
- *Secondary* — the claim band and its inline citations. Carried by adjacency: a claim sits
  immediately above the fence that demonstrates it, so the pair reads as one unit.
- *Recessive* — the scope band and the closing pointer. Carried by position (last) and, for
  scoped content past the threshold, by being on another page entirely.

**`narrative-tree-index`**

- *Primary* — the narrative table. Carried by position: first table on the page.
- *Secondary* — the existing pointer-out table. Carried by position: second.
- *Recessive* — the pin-by-path paragraph at `:25-29`. Carried by position (last) and register
  (it addresses a contributor, not a reader).

**`gate-narrative-checker-step`**

- *Primary* — `{path}:{line}`. Carried by being first on the line, and by the em dash that
  separates it from the message — the existing separator, `lint_constitution.rs:605-660`.
- *Secondary* — the message. Carried by position after the em dash.
- *Recessive* — the two-space indent, the banner and the count. The indent is what makes the
  problem lines read as a body under the banner rather than as top-level output; the count is
  a summary, and summaries go last.

## States

| State | `narrative-page` / index | Gate surfaces |
| --- | --- | --- |
| **Empty** | A page in the tree with no fences is legal — not every teaching page carries code. A *tree* with no pages is not: the checker `bail!`s before any check runs, mirroring `lint_constitution.rs:175-177` (`"{TREE} holds no pages, so every check below is vacuous"`). A vacuous green is the failure mode this whole project exists to refuse. | Same `bail!`. Exit is failure, not a `skipped` line. |
| **Loading** | N/A — static markdown. | Inherited and deliberate: the banner prints before the work (`main.rs:864`), so a slow step shows its name and then silence. **Do not add** a spinner, a dot ticker or a per-page progress line — noise on the happy path is what trains people to stop reading the output. |
| **Error** | A page cannot be in an error state; it is bytes. The tree's error states are the gate's. | The failure report composed in `## Composition`: banner, every problem in source order, count last. Never a bare non-zero exit. |
| **Overflow** | Page over 250 lines, or a scope band over threshold → split (see the yield order). This is enforced by the author and reviewed, not by the checker; making it a gate rule would be this project taking HS-P0021's job. | 40 problems print as 40 lines. **No truncation, no "and 37 more".** The count line is what makes a long list navigable, and a truncated list is the six-review-cycle failure `_decomposition.md:218-219` names. |
| **Long label** | An H1 over 40 characters wraps the index table's left column; caught at review. A *path* over the 48-character prefix budget (32 characters repo-relative, once the doctest prefix is counted) is caught by the **checker**, at the pinning check, before any line is emitted — the density rule becomes a gate rule precisely so the terminal surface cannot be starved by a directory someone nested one level too deep. | A message long enough to soft-wrap is acceptable; a message that pushes the location off the first row is not, and cannot happen while the location is first. |
| **Narrow viewport** | 70 columns: prose reflows, fences fit by budget, the two-column table fits. The only construct that can overflow is a table, which is why the index's column count is fixed at two. | An 80-column log: guaranteed by the location-prefix budget. |
| **Hidden marker present** | Not a render state — the page never reaches a reader. | `docs/<page>.md:<line> — \`<details\` is a hidden panel; DT-7 forbids it in {TREE}`, one line per occurrence, no allowance path. |

## Shape decision

| Item | Chosen shape | Rejected (and why) | Evidence | Resolves |
| --- | --- | --- | --- | --- |
| Hosting / render | `docs/` repurposed; the markdown is the render | mdBook site (probe shape, DR-03; from-zero scaffold — no `book.toml`, no CSS in the tree); rustdoc-embedded narrative (`include_str!` across the package boundary is a published-crate hazard); a published site (divergence from checked bytes) | Deployment brief Options A/B; `main.rs:89-102`; `xtask/src/lib.rs:8-16`; `RUNBOOK.md:918-925` | AC-009, DR-10 — hosting shape, previously undecided by charter, decomposition and grounding |
| Hidden content | (b) inline-visible ≤ 3 scopes × ≤ 25 lines, else (c) a page per scope; (a) rejected by `HIDDEN_MARKERS` | (a) tabs/folds — three unverified properties, and a conditional adoption yields a rule nothing can fail | `interaction-patterns.md:213-216, 218-225, 244-252`; `RUNBOOK.md:918-925` | **DT-7**, AC-006, DR-08 |
| Hidden-marker allowance list | None | An allowance list, symmetric with `IGNORE_ALLOWANCES` — a reverse sweep detects a stale allowance but cannot detect an unverified mechanism | architecture brief Note 5 | AC-006 |
| Fence/claim arrangement | Interleaved, claim immediately above its fence | Fences gathered at the end of the page | architecture brief Note 10 item 1 (the compiles-but-no-longer-demonstrates blind spot) | AC-010's headline limit, made reviewable |
| Problem reporting | Accumulate all, source order, count last | Fail fast on the first problem | `lint_constitution.rs:169-198`; `_decomposition.md:218-219` | AC-001/004/005/007 output shape |
| Step count | Two steps, two banners | One step shared with the constitution's | architecture brief Note 3 | AC-002/AC-005 attributability |

## Placement and re-export

Nothing is re-exported and nothing is public. The placement decisions that matter are the two
targets, and getting them backwards is the architecture brief's named most-likely error
(Note 1):

- **`xtask/src/narrative.rs`, declared from `xtask/src/lib.rs`** — the harness, one
  `#[cfg(doctest)] mod` per page. A `mod narrative;` added to `main.rs` instead lands in the
  *bin* crate, compiles clean, and its fences are compiled by nothing.
- **The checker, declared from `xtask/src/main.rs:64-70`** — bin crate, alongside `mod
  lint_constitution;`, plus its `REQUIRED` entry, its dispatch arm, its `print_help` line and
  its `lint_steps` membership. `steps_named` panics on a name absent from `REQUIRED`
  (`main.rs:816-826`), which makes a half-mounted step a build-time bug.
- The two never link. The checker reads the harness **as text**, exactly as `check_harness`
  reads `xtask/src/constitution.rs` (`lint_constitution.rs:424-425`).
- `clause_ids` is `pub(crate)` in `spec_trace.rs`, beside `all_rules` (`:1746`), because that
  is where the parser lives and a list kept away from its parser drifts from it (`:83-84`).

Coherence has nothing to say here — there is no trait, no generic and no foreign type. The
placement constraint is Cargo's target boundary, not the orphan rule.

## Visibility and stability

| Item | Visibility | `#[non_exhaustive]` / sealed | Feature | Semver promise |
| --- | --- | --- | --- | --- |
| `clause_ids` | `pub(crate)` | n/a | default | **None.** `xtask` is `publish = false`; it is never in anyone's dependency graph. |
| `TREE`, `HARNESS` | private `const` | n/a | default | None — but `TREE`'s *value* is a repository-wide contract: moving `docs/` without editing this line fails the gate (AC-001). |
| `IGNORE_ALLOWANCES` | private `const` | n/a | default | None. Grows only by review; swept in reverse so a stale entry is a problem. |
| `HIDDEN_MARKERS` | private `const` | n/a | default | None. Shrinking it re-opens DT-7 and requires a new design record, not an edit. |
| The narrative pages | n/a — markdown | n/a | n/a | None yet. Nothing is published; if the tree ever becomes part of a published crate, D1's second rejected option is the decision that has to be reopened first. |

## What it costs a caller

The "caller" here is a contributor, and both port flavours are irrelevant — nothing in this
project touches a port, an async fn or a `Send` bound, so ADR-0001 is untouched by
construction.

- **Build cost: zero new dependencies.** The mechanism is `include_str!` under
  `#[cfg(doctest)]` against the dev-dependencies `xtask/Cargo.toml:22-33` already declares.
  This is the whole reason D1 rejected mdBook: DR-12's standing trade at
  `xtask/Cargo.toml:16-21` deliberately keeps `rusqlite` and `sqlx` out of the dev graph
  because every `cargo xtask ci` would build them, and a narrative renderer would be the same
  bargain with less to show for it.
- **Gate time:** one added doctest compile over the tree, plus one file-reading check whose
  cost is a `read_dir` and a line scan — the class `affected.rs:28-36` already argues
  finishes inside the time cargo takes to decide `xtask` is up to date.
- **Authoring cost, and it is real:** an author writing adapter-scoped content pays the
  threshold. Six adapters means six pages and six index rows instead of one tab strip. That
  is the price of D2 and it is not nothing; the dossier says so directly
  (`interaction-patterns.md:218-220`).
- **Reader cost:** no search across narrative pages and no sidebar. Paid to D1.
- **MSRV, wasm32, features:** unaffected. No `cfg`, no feature, no target.

## What a user meets first

Two readers, two front doors, and this is the only place the two are stated together.

- **A reader** lands on `docs/README.md` — the index, whose first table is the narrative one.
  From there, one page. Not on the front page and deliberately: the scoped adapter pages,
  which are reached from the page they specialise, and the specification, which is a
  redirection rather than a destination.
- **A contributor** meets the checker's **"What this does not verify"** section, which is the
  *first* thing in the new modules' docs, not the last —
  `lint_constitution.rs:11-13`'s reason applies verbatim: a check whose limits are
  undocumented is read as a guarantee. All six limits are enumerated in the architecture
  brief's Note 10; `documented-blind-spots-and-their-proofs` owns their contents.
- **Neither meets a claim that this surface proves a page teaches.** DoD item 8, and it is a
  design constraint as much as a prose one: there is no badge, no "verified" mark and no
  green tick anywhere in either surface.

## The states the API must express

The checker's `Vec<String>` of problems must be able to express each of these distinctly —
enumerated here so they are designed in rather than discovered as a missing message:

*absent* (the tree is gone — name the expected path, AC-001) · *empty* (the tree exists and
holds no pages — `bail!` before checking, never a vacuous pass) · *unregistered* (a page the
harness does not `include_str!`, AC-005) · *dangling registration* (the harness names a page
that no longer exists — the reverse direction of the same check) · *untagged fence* ·
*unrecognised info string* (exhaustively matched, so an unknown part is a hard error) ·
*opted out* (`ignore` not on the allowance list) · *stale allowance* (an entry naming a fence
that is gone) · *hidden marker present* (DT-7) · *unresolvable clause id* (AC-007) ·
*unreadable citation* (hard error, never a skip — `lint_constitution.rs:29-44`) · *pinned
MUST no longer at its discharge site* (AC-008) · *count disagreement* (derived vs.
hand-written, RS-81-5) · *path over the prefix budget* (the density rule as a gate rule).

## Anti-patterns

Concrete forbidden moves. The first nine are checkable against a screenshot by someone who
cannot read the code; the last two are checkable against a directory listing.

Standing, and never re-litigated here: no `#[async_trait]`; no `serde` in
`happenstance-core`'s defaults; `read` returns the stream at the top level; generic code
binds `EventStore`, not `SendEventStore`; no `unwrap`/`expect` in library code.

1. **A disclosure triangle, a tab strip, or a collapsed callout anywhere on a page under
   `docs/`.** If a screenshot shows something a reader must click to read, DT-7 has been
   reversed without a design record.
2. **A hand-rolled table of contents, breadcrumb trail, or prev/next footer** on a narrative
   page.
3. **A code block with a horizontal scrollbar** at the narrow width.
4. **A "Run" or "Play" button on a fence.** It compiles a different copy than the gate did.
5. **A fence with no language tag**, or one visibly marked `ignore` with no allowance entry.
6. **A page title that wraps to a second line in the index table.**
7. **A gate failure whose first visual row does not begin with `path:line`.**
8. **A truncated problem list** — any "… and N more" in the checker's output.
9. **A badge, tick, shield or "verified" mark** asserting the documentation is checked for
   correctness or comprehension.
10. **A `book/`, `site/`, `_site/` or `target/book` directory**, a `book.toml`, or a `.css`
    file appearing anywhere in the repository. D1 says the render is the source; any of these
    means a second render exists that can diverge from it.
11. **A third directory level under `docs/`**, or a page filename over 32 characters — the
    location prefix budget, and the reason it is enforced rather than suggested.

## The doctest

There is no public API to demonstrate, so the substitute for the mock is the **fixture page**
— the smallest artifact that exercises every form decision above and is compiled by the gate.
This is the page `pinned-narrative-tree-and-compiling-step` creates and
`observed-failure-falsification` breaks. Written out here so the composition is reviewable
before it is built:

````markdown
# Appending under a condition

*<!-- answered-need: reserved for HS-P0021 -->*

An append condition is checked against the same boundary the query read, so a
writer that saw a consistent view cannot be overtaken between reading and
appending (ES-40).

```rust
use happenstance_core::MemoryEventStore;

let store = MemoryEventStore::new();
assert_eq!(store.len(), 0);
```

`memory` is a private module (`crates/happenstance-core/src/lib.rs:103`); the type is
re-exported at `:122`, so `happenstance_core::MemoryEventStore` is the resolving path.
Corrected here from the mock's finding 1 — the earlier spelling did not compile, and this
fixture is the literal artifact `pinned-narrative-tree-and-compiling-step` and
`observed-failure-falsification` build against.

## Per-adapter notes

### happenstance-sqlite

One writer at a time; positions are assigned under the store's lock.

### happenstance-postgres

Positions are assigned outside the transaction, which is the axis this
workspace keeps an adapter at the other end of.
````

Four things this fixture pins, each of which the design review can check by eye: the reserved
answered-need slot is present and empty; the clause id sits inside the sentence that depends
on it, not in a footer; the fence follows the claim it demonstrates; the scope band is last,
visible, level-3, alphabetical, and two scopes — inside the threshold, so it correctly stays
inline rather than splitting.

The **negative** fixture matters as much and is `hidden-content-resolution`'s: the same page
with a `<details>` wrapper around the scope band, which must fail the checker by file and
line. It is the named wrong implementation for `HIDDEN_MARKERS`, and without it the rule is
decorative.

## Open questions

Recorded rather than settled, because each belongs to a stage or a project that is not this
one.

1. **Narrative prose on docs.rs.** D1's second rejected option — `#[doc = include_str!]` on a
   `pub mod` inside `crates/happenstance/` — is the only shape that would put teaching prose
   where a `cargo add` user actually looks, and it lost here on the package-boundary hazard
   and on CR-1, not on merit. Reach is BR-08/DT-10 and HS-P0023's; if that project wants it,
   it reopens this row rather than working around it.
2. **Whether the 250-line page cap and the 40-character H1 cap should be gate rules.** They
   are review rules today, because enforcing them would put this project inside HS-P0021's
   page-need discipline. The path-length budget *is* enforced, because it protects the
   terminal surface rather than the editorial one — the line between the two is drawn there
   deliberately and is the row most likely to move.
3. **What happens the first time a genuinely non-normative disclosure use appears** (an
   install-command variant, the dossier's own third option at `interaction-patterns.md:477-479`).
   The answer is an allowance shaped like `IGNORE_ALLOWANCES`, with its own falsification, in
   its own change. Nothing pre-approves it.

## Mock

| Mock | Path | Viewports | Themes | Notes |
| --- | --- | --- | --- | --- |
| Static sign-off mock | [`design/mock.html`](design/mock.html) | 1440x900, 1024x768 | light | 46 frames: all six surfaces × all 23 declared states × 2 viewports. Self-contained — no network fetch of any kind. |

**What it is composed from.** There is no design system here, and the mock proves it rather
than assuming it: `git ls-files` matches no `.css`/`.scss`/`.sass`/`.less`/`.styl`, there is
no `book.toml`, and `docs/` holds one file. So the mock inlines **the only stylesheet this
repository produces** — `target/doc/static.files/rustdoc-17e0aaed.css` plus
`normalize-9960930a.css`, generated by the gate's own `cargo doc` step — verbatim, and mirrors
rustdoc's real emitted markup and class contracts (`.docblock`, `.docblock table td`,
`.example-wrap`, `pre.rust.rust-example-rendered > code`, `span.kw`/`.macro`/`.number`,
`.item-table`, `.main-heading`, `#copy-path`, `.search-input`, `details.toggle.top-doc`),
copied from `target/doc/happenstance/index.html`. Seven woff2 faces are embedded as `data:`
URIs; rustdoc's own `@font-face` blocks are stripped because their `url()`s are relative to
that directory. The terminal surfaces are composed only from the primitives in this file's
opening table, re-verified line by line. The page carries a provenance table naming every
token, class and line it used and the check that resolved it.

**Density is measured, not asserted.** Each markdown frame renders from one page model that
emits both the markdown source and the rustdoc-mirrored HTML, so the line counts, prose
widths and fence widths printed under a frame are counted off the source pane beneath it. The
`long-page-overflow` frame is a genuine 256-line, 6-scope page; the `narrow-70col` frame
carries a genuine 144-column fence in a 70-column pane; the terminal frames soft-wrap at 80
columns the way a terminal does rather than scrolling.

**Reference captures** the design review will compare against:

| Surface | 1440x900 | 1024x768 |
| --- | --- | --- |
| `narrative-tree-index` | `design/reference/narrative-tree-index@1440x900.png` | `design/reference/narrative-tree-index@1024x768.png` |
| `narrative-page` | `design/reference/narrative-page@1440x900.png` | `design/reference/narrative-page@1024x768.png` |
| `narrative-scoped-page` | `design/reference/narrative-scoped-page@1440x900.png` | `design/reference/narrative-scoped-page@1024x768.png` |
| `gate-narrative-compile-step` | `design/reference/gate-narrative-compile-step@1440x900.png` | `design/reference/gate-narrative-compile-step@1024x768.png` |
| `gate-narrative-checker-step` | `design/reference/gate-narrative-checker-step@1440x900.png` | `design/reference/gate-narrative-checker-step@1024x768.png` |
| `rustdoc-reference-surface` | `design/reference/rustdoc-reference-surface@1440x900.png` | `design/reference/rustdoc-reference-surface@1024x768.png` |

Paths are relative to `.bklg/docs-that-teach/checked-documentation-surface/`. They are the
capture targets, not committed files: `design.capture` is undeclared, so nothing will produce
them automatically and the mock's own frames carry the same ids
(`<surface>--<state>--<viewport>`) for a manual capture.

**Six findings the mock returned, and they belong in the sign-off conversation.** Drawing this
resolved every primitive against a file, and six claims above did not survive. They are
written out in full at the top of `mock.html`; in brief:

1. **The fixture at `## The doctest` does not compile.** It opens
   `use happenstance_core::memory::MemoryEventStore;`, but `memory` is a *private* module
   (`crates/happenstance-core/src/lib.rs:103`; the type is re-exported at `:122`). The
   resolving path is `happenstance_core::MemoryEventStore`. The artifact offered as the
   substitute for a mock fails the step this project adds — drawn as
   `gate-narrative-compile-step / fail-broken-fence`.
2. **`## Composition`'s last line is one xtask never prints.** `Error: 3 problem(s) in docs`
   is not a primitive; `eprintln!("\nxtask failed: {err:#}")` at `xtask/src/main.rs:712` is,
   and under `cargo xtask ci` a second line follows from the parent —
   `bail!("{} failed with {status}", step.name)` at `:887` — because each step is spawned as
   its own `cargo run -p xtask`.
3. **The 48-character location-prefix budget does not cover the compile surface.** Observed
   from `cargo test -p xtask --doc -- --list`: a doctest is named
   `xtask\src\../../<page> - narrative::<mod> (line N)`. The 16-character `xtask\src\../../`
   prefix is uncounted, so `docs/adapters/happenstance-cloudflare.md` reaches 56 characters
   there while fitting on the checker surface.
4. **The 60-character H1 budget is derived from the wrong width.** At 70 columns, with the
   widest right-column cell at 22 characters and 7 characters of table overhead, the left
   column holds **41** — not 60. 60 is the figure for the ~100-column wide width. Either the
   budget drops to ~40 or the index stops being sized against the narrow width.
5. **Two citations in the opening primitive table are off by two lines** — the success summary
   is `lint_constitution.rs:192` (stated 194) and the terminal `bail!` is `:199` (stated 198).
   The strings are otherwise exact, and nothing in the gate reads this file.
6. **`=== narrative pages ===` breaks the house naming convention.** Every `REQUIRED` name is
   a claim sentence (`xtask/src/main.rs:466`), including this project's own first step. A noun
   phrase reads as a category, and the banner is the only thing telling a reader which half
   failed.

Findings 1, 2 and 5 are corrections. Findings 3, 4 and 6 are decisions this file has to
re-take, and none of them can be settled by the implementer without reopening this record.

**All six are now folded back into the binding sections above, at the design gate on
2026-08-17.** The record of what changed, so a reader of this section is not left thinking
these are still open:

| # | Disposition |
| --- | --- |
| 1 | **Corrected.** `## The doctest` now reads `use happenstance_core::MemoryEventStore;`, with the private-module reasoning stated inline. |
| 2 | **Corrected.** `## Composition`'s failure report now ends with the real two-line tail — `xtask failed: …` from `main.rs:712` and the parent's `bail!` from `:887` — instead of the invented `Error: 3 problem(s) in docs`. |
| 3 | **Decided.** The location-prefix budget is ≤ 48 characters **inclusive of the 16-character `xtask\src\../../` doctest prefix**, i.e. ≤ 32 characters of repo-relative path. The checker enforces the stricter number because a path fitting the compile surface necessarily fits the checker, and not the reverse. Recorded in `## Density budget`. |
| 4 | **Decided.** The H1 budget drops to **≤ 40 characters** rather than the index ceasing to be sized against the narrow width — the narrow width is what the whole budget is built on, and exempting one row from it would make the rest arbitrary. Propagated to `## Composition`, `## Density budget`, `## States` and `## Open questions`. |
| 5 | **Corrected.** The two citations now read `lint_constitution.rs:192` and `:199`. |
| 6 | **Decided.** The checker's banner becomes the claim sentence `=== every narrative page is checked ===`, matching the convention every `REQUIRED` name follows (`xtask/src/main.rs:466`) and this project's own compile step. |

**`design/mock.html` deliberately still shows the pre-correction figures**, and must not be
"fixed" to match the table above. It draws the H1 at 60 characters and prints the old
`=== narrative pages ===` banner because it is the dated instrument that *disproved* those
numbers — its finding 4 is only legible against the frame that produced it. Editing the mock
to agree with the corrected design would delete the evidence and leave the findings section
asserting a contradiction nobody can see. The design is the specification; the mock is the
measurement that corrected it, and the two are supposed to disagree here.

## Sign-off

**Pending.** No perceptual review will ever run against this project — `design.capture` is
undeclared by design — so a human reading this file is the only gate these decisions have.
Two rows are the ones worth disagreeing with cheaply, now: **D1**, which trades a sidebar,
a search index and a rendered site for a surface that cannot diverge from what CI checked;
and **D2**, which trades tabs at a fanout tabs genuinely suit for a rule that fails
deterministically today.

| Approved by | Date | Conditions |
| --- | --- | --- |
| Ryan Britton (repository owner) | 2026-08-17 | Approved with no conditions, after all six mock findings were folded back into the binding sections. Both rows flagged above were read and accepted as written: **D1** (the markdown is the render — no mdBook, no site, no build step) and **D2/DT-7** (tabs and folds rejected outright, enforced by `HIDDEN_MARKERS` with deliberately no allowance list). The three decisions the mock reopened were taken at this gate rather than deferred: H1 budget 60 → **40** characters, location prefix ≤ 48 **inclusive of the 16-character doctest prefix** (≤ 32 repo-relative), and the banner restated as the claim sentence `=== every narrative page is checked ===`. |

Recorded via `redkiln advance HS-P0020 --verdict approved --stay --apply`, which clears the
review gate without moving the project off `design` — its stories are scaffolded next, and
`implement.md` owns the transition into `implementation`.
