---
item: HS-S0156
stage: spec
created: 2026-08-17T13:16:13.954Z
updated: 2026-08-17T13:16:13.954Z
template_sig: 87bbf1d0
rendered_sig: e961fb61
---

# Spec — rustc's own E0034 output at the site where it fires

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` |
| Project (the spine this story serves) | `.bklg/docs-that-teach/reach-and-adapter-path/project.md` |
| This spec | `.bklg/docs-that-teach/reach-and-adapter-path/store-error-site-rewrite/spec.md` |
| Briefs (architecture + ux, one `##` each) | `.bklg/docs-that-teach/reach-and-adapter-path/_decomposition.md` |
| **Signed-off design — binding** | `.bklg/docs-that-teach/reach-and-adapter-path/_design.md` |
| Sign-off mock (38 labelled frames) | `.bklg/docs-that-teach/reach-and-adapter-path/design/mock.html` |
| Story map (slice cut, merge order, standing constraints) | `.bklg/docs-that-teach/reach-and-adapter-path/_storymap.md` |
| Grounding (verified sibling-project state) | `.bklg/docs-that-teach/reach-and-adapter-path/_grounding.md` |
| Roadmap pointer | none. `RUNBOOK.md` sequences library phases, not documentation projects; this initiative's ordering lives in `_decomposition.md`'s dependency graph. |

The design file is **binding and already signed off** (`_design.md`, `## Sign-off`,
Ryan Britton, 2026-08-17). This story implements the `store-module-error-site`
surface it resolved. It does not re-decide composition, transience, density,
hierarchy or the anti-pattern list, and it may not contradict them.

## One-line PR slice

Rewrite `crates/happenstance-core/src/store.rs`'s module doc behind HS-P0020's
clause-id pin: restore rustc's real `error[E0034]` block including its `= note:`
candidate lines and `TraitVariantBlanketType`, state the *narrow* unchecked limit,
keep the in-place fix ahead of any hop, add the one guarded pointer to the reasoning
account, and run `cargo xtask spec-trace` over the result.

## Executive summary

**What this PR lands.** Six elements inside one existing section of one module doc
comment — `## Import one flavour, not both` at `crates/happenstance-core/src/store.rs:31-45`
— plus three `#[doc(alias)]` attributes on the two traits in the same file, plus the
re-anchoring of `spec/SPECIFICATION.md`'s line citations that the insertion displaces.

**The delta, stated as a diff and not as prose.** Today the section carries a
*trimmed* `error[E0034]` transcript (`store.rs:36-41`, six lines: the error line, the
call line, the `^^^^ multiple` caret). Verified by direct search: no `= note:`
candidate lines anywhere in the file, and `rg TraitVariantBlanketType crates/ standards/`
returns **nothing workspace-wide**. After this PR the section runs (a) the existing
one-sentence cause, (b) the reproduced full transcript, (c) new prose naming the
ambiguity in words, (d) the existing in-place fix, (e) the new narrow-limit sentence,
(f) the recessive pointer to the reasoning account — in that order, which
`_design.md`'s `## Composition` makes binding.

**Why this is not a copy edit.** Three things make it a slice rather than a
paragraph. The restored transcript is the string a stuck reader *searches* for, so
its absence is the gap and its verbatim presence is the fix (UX brief, invariant 6).
The insertion is large enough — the section goes 13 → ≈34 source lines — to push
every line-anchored `store.rs:NNN` citation in `spec/SPECIFICATION.md` past
`spec_trace`'s twelve-line anchor window, which is a gate failure this story must
land the repair for rather than discover. And the edit touches a `happenstance-core`
doc comment, which is the one thing in this project sequenced behind HS-P0020's
clause-id pin.

Everything not in that list — the front door, the onward links, the reasoning
account's own content, the observed walk — belongs to other stories. See
`## PR boundary`.

## Context pack

The distilled, load-bearing decisions. Read this before touching anything; the
deeper artefacts are behind the anchors and are opened just-in-time, not up front.

### The persona slice this realizes

Persona 2, the adapter author, at journey states **B0 → B2** (UX brief, Journey B).
B0: the error fires and their *first* move is a search — Ctrl-F in the buffer they
have open, or the message pasted into a search box. B1: they get compiling again.
B2: they want to tell "the port is wrong for me" apart from "I have not understood
the port yet". This story owns B0 and B1 outright and hands off B2 at the last
sentence. It does **not** own B3 — reading the account — which is
`adapter-reasoning-account`'s.

### Decision 1 — in-place before context-jump, and the order is load-bearing

A reader who never follows the pointer must still be correctly unstuck. That is UX
invariant 1, and `_design.md` turns it into a physical ordering rule: element (d),
the import-one-flavour fix and the fully-qualified escape hatch that live at
`store.rs:43-45` today, survives in substance and **precedes** element (f), the
pointer. **Nothing may be inserted between (c) and (d)** — the design states that as
a hard constraint, because separating the diagnosis from its fix is what turns a
self-sufficient page into one that requires a hop. Anti-pattern 12: "the `store.rs`
pointer appearing **before** the import-rule fix in reading order."

### Decision 2 — the transcript is reproduced, never transcribed

The block is produced by compiling a deliberate double-import at the pinned
toolchain (`rust-toolchain.toml`, `channel = "1.97.1"`) and pasting from *that run's*
stderr (AC-A05). `references/evaluation/review-dx-ergonomics.md:404-414` says what
the block should **contain** — both `= note:` candidate lines and the `help:` line —
and is explicitly **not** the text to copy: it predates the MSRV raise recorded in
`.kb/decisions/0029-msrv-raised-to-1-97-1.md`. Keep the excerpt to the parts that are
stable across toolchains — the error code and the candidate item names — because
diagnostic phrasing moves and pretending otherwise is a claim the page cannot keep.

Two strings are the deliverable and neither may be trimmed: the two lines beginning
`= note:`, and `TraitVariantBlanketType`. `_design.md`'s density row for this surface
says so in the yield order, and anti-patterns 7 and 8 are the screenshot form of it.

### Decision 3 — copy fidelity beats the density budget, decided at the gate

The mock produced six findings and sign-off resolved the sharpest, **F3**, in favour
of copy fidelity: a reproduced transcript edited to fit a budget no longer matches
what the reader has on screen, which destroys the one thing the error-site
explanation exists to do. Consequences the implementer inherits and may not re-open:

- the section cap is **36 source lines** (raised from 30), against a measured
  projection of ≈34 — F2;
- dropping candidate #1's `help:` hunk is yield rule (3), **already exercised at
  design time**; nothing further may be cut;
- the "≤ 96 characters per fence line" budget is recorded as a **failed
  measurement, not a target** — rustc's longest line measures 109 characters and
  lays out at 1060px against a 936px content box, so it overflows and takes a
  horizontal scrollbar at *both* 1440x900 and 1024x768 — F4. Anti-pattern 16's
  exception is therefore load-bearing everywhere, and **a later reader must not
  "fix" this by reflowing the transcript.**

### Decision 4 — the caret line may not be the only carrier

`^^^^ multiple` is spatial. A screen reader linearises it into meaningless
characters and so does a plain-text search hit. Element (c) is the non-spatial
carrier and is required content, not decoration: it names the method (`read`), names
both `EventStore` and `SendEventStore`, and states in words that both are in scope
and which call is therefore ambiguous. Anti-pattern 15 is the falsification: "colour,
syntax highlighting, or the caret line `^^^^` as the only carrier of which call is
ambiguous."

### Decision 5 — the limit stated is the narrow one, because the broad one is false

`project.md`'s risk table says a ```` ```text ```` fence "is invisible to every gate
step", and for that fence it is true. But the error **code** is already gate-checked
twice elsewhere: `standards/rust/20-two-flavour-ports.md:179` and
`standards/rust/00-prime-directives.md:242` each carry a ```` ```rust,compile_fail,E0034 ````
fence, both compiled by the constitution step (`cargo test --locked -p xtask --doc`,
`RUSTDOCFLAGS=-D warnings`) through `xtask/src/constitution.rs`. So element (e) says:
*the code is asserted by a compiled negative in the constitution; the wording of the
notes and the internal name `TraitVariantBlanketType` are rustc 1.97.1's rendering
and are asserted by nothing.* "Nothing checks this" is wrong **in the direction that
discards a real guard** (AC-A06).

Mechanical corollary: `TraitVariantBlanketType` inside the fence is inert, but the
moment it appears in surrounding **prose** it needs backticks — `clippy::doc_markdown`
fires on CamelCase, `clippy.toml`'s `doc-valid-idents` is for proper nouns and this is
an identifier, and RS-70-3 forbids the `allow` repair.

### Decision 6 — the pointer's form is constrained by a gate step, not by taste

Element (f) may **not** be an intra-doc link. `rustdoc::broken_intra_doc_links = "deny"`
(`Cargo.toml:134`) is enforced across three rustdoc builds including
`documentation (no default features)` (`xtask/src/main.rs:502`), and the destination
is a narrative page, not an item — so no link form resolves in every feature
configuration. `store.rs:76-78` already records this exact class of bug in this exact
file, in the sanctioned named-but-unlinked form. That form is the fallback; the
preferred rung is an in-tree markdown link inside HS-P0020's pinned tree. The
href ladder (`_design.md`, `## Pattern decision`, rule 3) is applied per pointer at
implementation and recorded in the register row: intra-doc **>** in-tree markdown link
**>** named-unlinked cross-reference **>** *nothing*. **A bare URL is forbidden to this
project outright** (rule 2(iv)) — AC-003 admits no pointer whose only guard is memory.

The pointer is also permitted only because all four of rule 2's gates hold here, and
this is the *only* evidenced stall this project has: (i) BR-15 is a recorded reader
failure at this item; (ii) the front door provably cannot reach it — a reader arrives
at `happenstance_core::store` from a diagnostic or a search box, and `pub mod store`
renders its own page (`crates/happenstance-core/src/lib.rs:99`); (iii) it is
subordinate, one line, after the fix, with no heading of its own; (iv) its form comes
from the guarded rungs of the ladder.

### Decision 7 — the clause pin is read *before* the first edit, and that order is structural

N-6 states the protocol and it is not advisory. (1) HS-P0020's clause-id pin (its
AC-008) is committed and readable — until then, **the first edit to `store.rs` may not
be made**; copy may be drafted, implementation may not start. (2) Read the pinned set
and identify which frozen documentation MUSTs the existing `store.rs:1-52` text
discharges. (3) Edit. (4) Run `cargo xtask spec-trace` and record the result.

The **primary** test applied is `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`,
not the governance atom `project.md` names first: a correction is a **repair** if the
set of implementations the clause admits is unchanged, and otherwise a **gap**, which
is an ADR's business. Restoring the `= note:` lines is a repair by that test — it
changes no implementation any clause admits. `.kb/governance/rewrite-the-referent-never-the-reasoning.md`
is the secondary test, and the story must say what it is doing: all three of that
atom's worked instances are ADR-record renames, so applying it to a doc comment is an
**analogy sanctioned by name in `CLAUDE.md`**, not a fourth worked instance in the atom.

If HS-P0020 lands without an enumerated clause-id set, this story **blocks** rather
than proceeding on judgement (`project.md`, risk table).

### Decision 8 — the insertion breaks `spec-trace`, and repairing that is in scope

Verified in this worktree, and the single most likely way this PR reds the gate:
`spec/SPECIFICATION.md` carries line-anchored citations into
`crates/happenstance-core/src/store.rs` at lines *below* the module doc — among them
`store.rs:93-268` (line 371), `store.rs:101`, `store.rs:119-123`, `store.rs:131-139`,
`store.rs:213-217`, `store.rs:321-331`. The module doc ends at line 52 and `use
alloc::vec::Vec;` begins at 54. Inserting ≈21 source lines above that point shifts
every one of those anchors by more than `ANCHOR_SLACK`, which is **12**
(`xtask/src/spec_trace.rs:391` — deliberately wider than the constitution lint's ten,
precisely so "a doc comment growing above an item must not red the gate", but not
this wide).

So: `cargo xtask spec-trace` is not a post-hoc confirmation here, it is a step that
will fail and must be repaired in the same PR. `spec-trace --write` re-anchors
(`xtask/src/main.rs:671-673`); a re-run in check mode is what proves it. **This is why
`spec/SPECIFICATION.md` is inside the PR boundary even though rustc never opens it** —
and every re-anchored citation must still point at the same subject, which is the
thing a reviewer checks rather than the line numbers.

### Decision 9 — heading ladder and hierarchy are frozen at four entries

`# Why there are two traits` (`:3`), `## What that means in practice` (`:21`),
`## Import one flavour, not both` (`:31`), `## Naming` (`:47`). Every change lands
**inside** the third. No fifth entry, and no `##` whose entire body is one sentence
(anti-pattern 13). Hierarchy for this surface, from `_design.md`: the fence and the
in-place fix are **primary**; the plain-words sentence and the limit sentence are
**secondary**; **the pointer is recessive** — carried by order within the section and
by its having no heading. That inverts the usual instinct deliberately: the moment the
pointer stops being recessive, invariant 1 fails.

### Decision 10 — three `#[doc(alias)]` keys, by a stated rule, establishing a convention

`#[doc(alias)]` has **zero occurrences** in `crates/`, `standards/` or `xtask/`
(re-verified). This story is where the convention is established, so it says so. The
rule: an alias is permitted only where the string a reader searches is one that rustc,
the specification, or a recorded reader question **actually emits**, and is not the
item's own name or a substring of it. Exactly three attributes qualify today:
`#[doc(alias = "E0034")]` on `EventStore` and on `SendEventStore`, and
`#[doc(alias = "TraitVariantBlanketType")]` on `SendEventStore`. Rejected: synonym
farming (`eventstore`, `es`, `event-store`), aliases for concepts rather than strings,
and aliases as a substitute for the pointer at the stall — an alias moves a reader who
is already searching and does nothing for the reader who is reading. Aliases get **no
register row**: an alias is an attribute on an item, so deleting the item deletes the
alias and it cannot rot independently.

### Decision 11 — this story files register row P3, and the register is not a page

`_design.md`'s density table pins five register rows for the project. **P3 is this
story's**: surface `crates/happenstance-core/src/store.rs` module doc → destination
the reasoning account, form "in-tree markdown link, or the named-unlinked form live
at `store.rs:77`", guard "HS-P0020's registration check that the page exists at that
path". Filing it is not optional bookkeeping — `_storymap.md`'s coverage table is
explicit: *"A story that installs a pointer and files no row has not finished."* The
register itself is **build-time data, never a rendered page** (anti-pattern 14); its
shape and home are `pointer-policy-and-inventory`'s to bind, modelled on `SUMMARIES`
at `xtask/src/lint_constitution.rs:64` and `check_summaries` at `:463-473`.

### Standing constraints inherited, not re-decided

From `_storymap.md`, "Standing constraints every story inherits", and `_design.md`'s
anti-pattern list: **no public item, signature, feature or manifest change** (the
`prelude` proposal at N-9 is the live example of what is out of boundary and owed an
ADR); no widget, no raw HTML, no inline `style=`, no folded/tabbed/collapsed content,
no skipped heading levels; self-describing link text — no "here", no bare URL into
this repository's own tree; and no "See also" / "Next steps" / "Further reading" block
appended anywhere. From `CLAUDE.md`'s binding constraints, none of which this story
touches and all of which it must not disturb: no `#[async_trait]`; no `serde` in
`happenstance-core`'s defaults; `read` returns the stream at the top level; generic
code binds `EventStore`, not `SendEventStore`.

## Integration contract

This story is delivered **mounted** — the doc comment renders through a gate-built
rustdoc surface, or it has not shipped.

- **Archetype**: `capability` — a user-observable slice. The observable is what a
  reader sees at `happenstance_core::store` and what `rg` finds in the file they have
  open.
- **Slice / milestone**: `adapter-error-site`. **Slice-mates**: `adapter-reasoning-account`
  (implemented in the same context, ahead of this story — the pointer's destination
  must exist before the pointer is installed). Both edits must sit inside HS-P0020's
  checked surface (project DoD item 5), which is the second reason they are one slice.
- **Mount point**: `crates/happenstance-core/src/store.rs` — the module `//!` doc
  comment at lines 1-52. It is mounted as a rendered page by
  `crates/happenstance-core/src/lib.rs:99` (`pub mod store;` — which is what gives the
  module its own page and its own deep-linkable fragments), and built by the three
  rustdoc passes in `xtask/src/main.rs`, of which
  `documentation (no default features)` (`xtask/src/main.rs:502`) is the one that
  constrains the pointer's form. Composition root CR-2 in the architecture brief.
- **Wires into**:
  - `crates/happenstance-core/src/store.rs:93` — `#[trait_variant::make(SendEventStore: Send)]`
    on `pub trait EventStore`: the derivation that *causes* the E0034 collision this
    page documents, and the two items the three `doc(alias)` attributes attach to.
  - `crates/happenstance-core/src/store.rs:76-78` — the live named-but-unlinked
    cross-reference, in this same file, that is the fallback form for element (f).
  - `Cargo.toml:134` — `rustdoc::broken_intra_doc_links = "deny"`, the lint that makes
    Decision 6 a build failure rather than a review note.
  - `standards/rust/20-two-flavour-ports.md:179` and
    `standards/rust/00-prime-directives.md:242` — the compiled
    `rust,compile_fail,E0034` negatives that element (e) cites as the real guard.
  - `xtask/src/spec_trace.rs:391` (`ANCHOR_SLACK`) and `xtask/src/main.rs:671-673`
    (`spec-trace [--write]`) — the citation-drift machinery this edit trips and repairs.
  - `rust-toolchain.toml` (`channel = "1.97.1"`) — the toolchain the transcript is
    reproduced on.
  - The pointer register landed by `pointer-policy-and-inventory` — consumed here to
    file row P3.
  - The reasoning account's bound path, landed by `adapter-reasoning-account` — the
    destination element (f) names.
- **Renders surfaces**: `store-module-error-site` (`_design.md`, `## Surfaces`), route
  `target/doc/happenstance_core/store/index.html`, selector `#main-content > .docblock`.
  The selector is the same generator's as the verified `happenstance` one and
  **must be confirmed after a `cargo doc -p happenstance-core`** — `_design.md` flags
  that `target/doc/happenstance_core/` was not built at design time. The four declared
  states are `section-import-one-flavour-not-both`, `e0034-fence-full-width`,
  `e0034-fence-narrow-viewport`, `heading-outline`. No other surface is rendered or
  changed by this story.
- **Public items**: **none.** `_design.md`'s `## Items` block declares `path: ""` for
  the whole project with its reason. The three `#[doc(alias)]` attributes are added and
  are deliberately **not** public API in the semver sense — they add no path a caller
  can write and are erased before type-checking, so the cost is identical on both port
  flavours and on `wasm32`.
- **Conformance rule(s)**: **none, and this is not adapter-observable.** No rule in
  `crates/happenstance-testkit/src/suite.rs` observes a doc comment, and no port,
  signature or bound changes here, so there is nothing an adapter could implement
  wrongly. Stating it rather than leaving it blank is the point: `CLAUDE.md` requires a
  story that changes a port to name a rule, and this story's answer is that it changes
  no port. The compiled `rust,compile_fail,E0034` fences in the constitution are the
  nearest thing to a rule for this behaviour and they already exist; this story cites
  them and adds none.
- **Clause(s)**: **amends none.** No `SPECIFICATION.md` clause is edited and no
  `[FROZEN]` clause changes, so no ADR is owed. What this story *does* to the
  specification is mechanical and bounded: it re-anchors the line numbers of existing
  citations into `store.rs` that the insertion displaces (Decision 8), leaving every
  cited subject identical. The clause ids this edit must leave discharged are the
  pinned set from HS-P0020's AC-008, which does not exist yet — the story records the
  set it read as one of AC-A04's three artefacts, and blocks if there is none.
- **Advances DoD scenario**: initiative **DoD-10** — "The adapter author's error meets
  its explanation." This story lands the explanation and the hop; the *observation*
  that closes DoD-10 is `error-site-walk-record`'s, in the `adapter-error-walk` slice.
  It also moves **DoD-11** (frozen documentation MUSTs still discharged; the
  cross-reference step passes over the tree after every doc comment this work touched)
  and **DoD-12** (no page has become a second specification) toward green. Project DoD
  item 2 lands with this story and nothing else.

## PR boundary

### In this PR

- The rewrite of `## Import one flavour, not both` in
  `crates/happenstance-core/src/store.rs` — elements (b), (c), (e), (f) added or
  rewritten, (a), (d), (g) preserved, in the design's binding order.
- The three `#[doc(alias)]` attributes on `EventStore` and `SendEventStore` in the
  same file.
- Re-anchoring the `store.rs` line citations in `spec/SPECIFICATION.md` that the
  insertion displaces, and the evidence that every re-anchored citation still names
  the same subject.
- Register row **P3** filed against the inventory landed by
  `pointer-policy-and-inventory`, with its form and its guard.
- This story's own backlog folder: the ledger, the recorded clause-id set, the
  toolchain the transcript was reproduced on, and the `cargo xtask spec-trace` result.

### Explicitly not in this PR

- The reasoning account's **content, path or registration** — `adapter-reasoning-account`'s,
  and a slice-mate. This story consumes the bound path and installs one sentence
  pointing at it.
- The pointer **policy, the register's shape, and the widget-rejection record** —
  `pointer-policy-and-inventory`'s. This story applies the policy and files one row.
- Anything on `crates/happenstance/src/lib.rs` or `crates/happenstance/README.md` —
  `front-door-pointer`'s. The adapter pointer is **forbidden** on the bare crate's
  front door: `crates/happenstance/src/lib.rs:20-25` already states ADR-0006's
  conclusion and `:69-71` routes adapter authors to `happenstance_core`, so installing
  it there would contradict the page it sits on.
- The **observed walk** — `error-site-walk-record`'s, and it must be a different pair
  of hands. Walking your own rewrite makes the author their own witness, which is the
  one thing AC-009 forbids.
- Any **`xtask/src/` step**. This project does not own that tree (N-1); the
  `check_summaries`-shaped hook and the `doc(alias)`-string assertion are asks handed
  to HS-P0020, not work done here.
- A **`prelude` module**. Evidenced and live (`references/evaluation/review-dx-ergonomics.md:421-422`
  calls it "the single highest-leverage doc fix in the crate"), and out of boundary: it
  is a public API addition with a semver surface, owed an ADR. Its absence here is a
  scoping decision, not a rejection.
- Any change to `crates/happenstance-testkit/`, any adapter crate, or any
  `SPECIFICATION.md` clause **text**.

### Merge DoD

`cargo xtask ci --fast` green with the rewrite in place, `cargo xtask spec-trace`
green over the resulting tree, the rendered `happenstance_core::store` page showing
both `= note:` lines and `TraitVariantBlanketType`, the in-place fix still ahead of
the pointer, and row P3 filed with a non-empty guard.

### Paths

`redkiln verify --grain story` reads the first fenced block under this heading and
fails on any file changed outside it. The register's home is
`pointer-policy-and-inventory`'s to bind; if it lands outside the globs below, this
boundary is widened **here, deliberately, before the edit** — never at gate time.

```
crates/happenstance-core/src/store.rs
spec/SPECIFICATION.md
spec/E2E-CASES.md
standards/rust/*.md
xtask/src/pointers.rs
.bklg/docs-that-teach/reach-and-adapter-path/**
```

**Three globs were added at implementation, and each is a consequence of the insertion
rather than new scope.**

1. `xtask/src/pointers.rs` — the register's home, which `pointer-policy-and-inventory`
   bound after this spec was written. The paragraph above provided for exactly this:
   the register landed outside the original globs, so the boundary is widened here
   rather than at gate time. Row **P3** is filed there and nothing else in that file's
   policy is re-decided.
2. `spec/E2E-CASES.md` and `standards/rust/*.md` — Decision 8 forecast the citation
   drift and named only `spec/SPECIFICATION.md`. It is **`cargo xtask spec-trace` *and*
   `cargo xtask lint-constitution`** that read line-anchored `store.rs:NNN` citations,
   and the second reads `standards/rust/`, which `spec-trace` never opens. Both trees
   are gated, both are displaced by the same insertion, and repairing only one leaves
   the other red. The edits there are line numbers and nothing else — no rule, no
   clause, no `**Evidence.**` subject changes.

`references/**` also carries `store.rs:NNN` citations and is **deliberately not
repaired**: it is evidence kept for citation, binding nothing, read by no checker, and
its line numbers were already historical. Stated so the omission is a decision.

## Behavior and interfaces

The observable behaviour, element by element. "Evidence path" is where the claim is
grounded today — the thing to read, not the thing to copy.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **(a) the cause sentence survives** | "Because a `SendEventStore` is also an `EventStore`, bringing **both** names into scope makes method-call syntax ambiguous" stays as it is. Untouched. | `crates/happenstance-core/src/store.rs:33-34` |
| **(b) the transcript is rustc's own** | The ```` ```text ```` fence is replaced by the stderr of a deliberate double-import compiled at `channel = "1.97.1"`. It carries the `error[E0034]` line, the call site, the caret, **both** `= note:` candidate lines, and `TraitVariantBlanketType`. ≈14 source lines, up from 6. Candidate #1's `help:` hunk is dropped — yield rule (3), already exercised at design time; nothing further may be cut. | `store.rs:36-41` (today, trimmed) · `rust-toolchain.toml` · `references/evaluation/review-dx-ergonomics.md:404-414` (what it must *contain*) · `_design.md` `## Composition` row (b), F2/F3 |
| **(b′) the fence stays uncompiled and unreflowed** | It remains a `text` fence — nothing compiles a diagnostic transcript, and wrapping it in a `rust` fence to buy a green tick is the `ignore`-fence anti-pattern wearing a compiler. Its longest line is 109 characters and it takes a horizontal scrollbar at both viewports. That is expected, not a defect. | `standards/rust/62-doctests-and-harnesses.md` (governs *compiled* fences) · `_design.md` `## The doctest`, F4, anti-pattern 16 |
| **(b″) the transcript is contiguous, selectable plain text** | No line numbers, no gutter decoration, no rendered-only markup inside the fence, so a reader can paste it into a diff against their own terminal output and find only compiler characters. | UX brief, invariant 7 |
| **(c) the ambiguity is named in words** | ≈3 new lines immediately after the fence: names the method (`read`), names both `EventStore` and `SendEventStore`, and states that both being in scope is what makes the call ambiguous. Required content, not fixed wording. The caret line may not be the only statement of it. | `_design.md` `## Signatures`, element (c) · UX brief, "Position and ASCII art never alone" · anti-pattern 15 |
| **(d) the in-place fix is preserved and precedes the pointer** | The import rule and the fully-qualified escape hatch (`SendEventStore::read(&store, &query, options)`) stay in substance. **Nothing is inserted between (c) and (d).** Deleting the reasoning account entirely must leave `store.rs` still telling a reader how to resolve E0034. | `store.rs:43-45` · UX brief, invariant 1 (with its falsification) · UX-AC-05 · anti-pattern 12 |
| **(e) the narrow limit is stated** | ≈3 new lines: the fence is `text` and nothing compiles it; the error **code** is asserted by a compiled `rust,compile_fail,E0034` example in the constitution; the notes' wording and `TraitVariantBlanketType` are rustc 1.97.1's rendering and are asserted by nothing. Saying "nothing checks this" fails this row. | `standards/rust/20-two-flavour-ports.md:179` · `standards/rust/00-prime-directives.md:242` · `xtask/src/constitution.rs` · AC-A06 |
| **(e′) CamelCase in prose is backticked** | `TraitVariantBlanketType` is inert inside the fence but needs backticks the moment it appears in prose. `clippy::doc_markdown` fires otherwise, and RS-70-3 forbids repairing it with an `allow`. | `standards/rust/70-rustdoc-obligations.md:154` · `clippy.toml` |
| **(f) one recessive pointer, correctly formed** | ≈2 lines, last sentence of the section, no heading of its own, link text a self-describing noun phrase of ≥ 3 words naming the destination and the need. Form taken from the href ladder's highest **available** rung; **never** an intra-doc link, **never** a bare URL. If only the "nothing" rung is available, the pointer is not installed and the story escalates. | `_design.md` `## Signatures` P3 (both the linked and the named-unlinked wordings) · `store.rs:76-78` (the live fallback form) · `Cargo.toml:134` · `xtask/src/main.rs:502` · AC-A02 |
| **(g) `## Naming` survives** | Untouched. The heading ladder stays at exactly four entries and no `##` gains a one-sentence body. | `store.rs:47-52` · anti-pattern 13 |
| **three `#[doc(alias)]` keys** | `E0034` on `EventStore` and on `SendEventStore`; `TraitVariantBlanketType` on `SendEventStore`. Establishes the workspace convention (zero occurrences today) and says so. No register row — an alias cannot rot independently of its item. | `store.rs:93-94` · `_design.md` `## Pattern decision`, the alias rule · N-3 |
| **section density** | ≤ **36** source lines, against a measured projection of ≈34. If exceeded, the yield order is: (1) the limit sentence compresses to one clause, (2) the connective prose around the pointer, (3) the fence's non-`= note:` context lines. **The two `= note:` lines and `TraitVariantBlanketType` never yield.** | `_design.md` `## Density budget`, `store-module-error-site` row |
| **search is the acceptance instrument** | `rg TraitVariantBlanketType crates/happenstance-core/src/store.rs` returns a hit. Today it returns nothing — verified across `crates/` and `standards/` — and that is precisely the gap. | UX brief, invariant 6 · UX-AC-06 |
| **register row P3** | Filed against the inventory with surface, destination, form (the ladder rung actually taken) and guard. No empty guard cell. | `_design.md` `## Density budget`, the five-row table · `_storymap.md` coverage, AC-003 row |
| **clause-pin protocol** | Read HS-P0020's pinned clause-id set **before the first edit**; record three artefacts — the set read, the toolchain the transcript was reproduced on, the `spec-trace` result. Block if the set does not exist. | N-6 · AC-A04 · `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` (primary test) · `.kb/governance/rewrite-the-referent-never-the-reasoning.md` (secondary, by sanctioned analogy) |
| **citation re-anchoring** | Every displaced `store.rs:NNN` citation in `spec/SPECIFICATION.md` is re-anchored and still names the same subject; `cargo xtask spec-trace` is green in check mode afterwards. | `xtask/src/spec_trace.rs:391` (`ANCHOR_SLACK = 12`) · `xtask/src/main.rs:671-673` · `spec/SPECIFICATION.md:371` and the other `store.rs:NNN` sites |
| **nothing else moves** | No public item, signature, feature or manifest change. No widget, fold, tab, raw HTML or inline style. No heading skipped. No "See also" block. No `unwrap`/`expect`. The four `CLAUDE.md` binding constraints are untouched. | `_storymap.md` standing constraints · `_design.md` `## Anti-patterns` 1-18 and the standing list · `CLAUDE.md` "Binding constraints" |

### The acceptance criteria this story enumerates

Nine, in the order the behaviour above lands them, framed from the adapter author's
intent: **AC-001** (rustc's own transcript, searchable and verbatim) · **AC-002** (the
ambiguity named in words, never spatially alone) · **AC-003** (unstuck without a hop,
fix ahead of pointer) · **AC-004** (the narrow limit, not the broad one) · **AC-005**
(one recessive, correctly-formed, resolving pointer) · **AC-006** (register row P3
filed with a named guard) · **AC-007** (clause pin read first; three artefacts
recorded; `spec-trace` green, including the re-anchoring) · **AC-008** (the surface
obeys the design and the discipline — ladder, density, anti-patterns, no second
specification) · **AC-009** (the three `doc(alias)` keys, by the stated rule).

## Data and migrations

**N/A — no runtime data, no schema, no stored state.** This story edits a doc comment,
adds three attributes that are erased before type-checking, and files one register row.
There is no database, no serialized format, no wire envelope and no persisted artefact
in scope; `happenstance-core` carries no `serde` in its default features by binding
constraint and nothing here changes that.

One thing that *is* migration-shaped, and is handled in `## Behavior and interfaces`
rather than here: the line-anchored citations in `spec/SPECIFICATION.md` are data about
`store.rs`'s layout, and the insertion invalidates them. The migration is
`cargo xtask spec-trace --write` followed by a check-mode re-run, with a human
confirming each re-anchored citation still names the same subject. It is a
documentation repair, not a data migration, and it produces no versioned artefact.

## Acceptance criteria

Nine, framed from the adapter author's intent rather than from the capability that
serves it — a criterion that reads "the fence contains two `= note:` lines" is
satisfiable by a page nobody can use. Persona and journey states are Journey B's
(UX brief, `_decomposition.md`); the personas themselves are drafts and carry that
qualification (`_discovery/distillation/personas-and-journeys.md`).

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an adapter author (Persona 2) at journey state **B0**, whose build just failed with `error[E0034]` and whose first move is a *search* — Ctrl-F in the buffer they already have open, or the message pasted into a search box — **WHEN** they search for any string rustc actually emitted, including a `= note:` candidate line or `TraitVariantBlanketType`, **THEN** `crates/happenstance-core/src/store.rs` is a hit, because the `text` fence in `## Import one flavour, not both` carries rustc 1.97.1's own stderr for a deliberate double-import verbatim — both `= note:` lines and `TraitVariantBlanketType` included — as contiguous, selectable plain text with no line numbers, gutter decoration or rendered-only markup inside the fence. | `rg -n "= note:" crates/happenstance-core/src/store.rs` returns **two** hits and `rg -n TraitVariantBlanketType crates/happenstance-core/src/store.rs` returns at least one — both return nothing today, which is the gap. Provenance: the reproduction command and the `rustc --version` of the run the block was pasted from, recorded in this story's folder (AC-007's second artefact). Render: `cargo doc -p happenstance-core --no-deps`, then the fence at `target/doc/happenstance_core/store/index.html` selected, copied and diffed against that same stderr — only compiler characters. |
| AC-002 | **GIVEN** the same reader, *reading* rather than searching — or reading with a screen reader, which linearises `^^^^ multiple` into meaningless characters — **WHEN** they reach the end of the transcript, **THEN** the sentences immediately following it state **in words** which call is ambiguous: the method (`read`), both `EventStore` and `SendEventStore` by name, and that both being in scope is the cause — so the diagnosis survives with colour, syntax highlighting and spatial position all removed. | Read `crates/happenstance-core/src/store.rs` element (c) with the fence deleted: the remaining prose still names `read`, both trait names, and the cause. `rg -n "EventStore" crates/happenstance-core/src/store.rs` confirms both names appear in the section's prose, not only inside the fence. Falsifies anti-pattern 15 (`_design.md`, `## Anti-patterns`). |
| AC-003 | **GIVEN** a reader at **B1** who wants only to get compiling again and will never follow a link, **WHEN** they read the section top to bottom, **THEN** they reach the import rule and the fully-qualified escape hatch (`SendEventStore::read(&store, &query, options)`) **before** any pointer, with nothing inserted between the plain-words diagnosis (c) and the fix (d) — and deleting the reasoning account entirely leaves `store.rs` still telling them how to resolve E0034. | Source order in `crates/happenstance-core/src/store.rs`: the line carrying `SendEventStore::read(&store,` precedes the line carrying the pointer's link text. The falsification is executed, not asserted: with the destination page removed from the working tree, re-read the section and confirm a reader is still unstuck. Anti-pattern 12; UX invariant 1; UX-AC-05. |
| AC-004 | **GIVEN** a reader who has just been handed a compiler transcript inside a documentation comment and is deciding how far to trust it, **WHEN** they read on, **THEN** the page tells them the **narrow** truth — the fence is `text` and nothing compiles it; the error *code* is asserted by a compiled `rust,compile_fail,E0034` example in the constitution; the wording of the notes and the internal name `TraitVariantBlanketType` are rustc 1.97.1's rendering and are asserted by nothing — and never the broad falsehood "nothing checks this". | `rg -n "compile_fail,E0034" standards/rust/20-two-flavour-ports.md standards/rust/00-prime-directives.md` returns both fences, proving the guard element (e) cites is real; `cargo test --locked -p xtask --doc` (the constitution step, `xtask/src/constitution.rs`) proves it is compiled. Read element (e) and confirm it names both halves. AC-A06. |
| AC-005 | **GIVEN** a reader at **B2** who is compiling again but still cannot tell *"the port is wrong for me"* from *"I have not understood the port yet"* — Persona 2's stated goal verbatim — **WHEN** they finish the section, **THEN** its last sentence offers exactly **one** hop to the adapter reasoning account: recessive, carrying no heading of its own, with self-describing link text of ≥ 3 words naming the destination and the need, in the highest **available** rung of the href ladder, never an intra-doc link and never a bare URL — and it resolves in all three gate rustdoc builds. | `cargo xtask ci --fast` green, which runs `documentation (no default features)` (`xtask/src/main.rs:502`) under `rustdoc::broken_intra_doc_links = "deny"` (`Cargo.toml:134`) — the step that fails if an intra-doc link was reached for. Count the pointers in the section: exactly one. Read the link text: no "here", "this", "docs", no `https://`. If only the ladder's "nothing" rung is available, the pointer is **not installed** and EC-002 fires. |
| AC-006 | **GIVEN** a maintainer six months on who must find every pointer this project installed without reading every file, **WHEN** they read the pointer register landed by `pointer-policy-and-inventory`, **THEN** row **P3** is present, naming the surface (`crates/happenstance-core/src/store.rs` module doc), the destination, the ladder rung **actually** taken, and a **non-empty** guard stating what fails the build if the destination moves. | The register file (path bound by `pointer-policy-and-inventory`) contains a P3 entry; its guard cell is non-empty. `cargo xtask lints` — the reachability-static command `.redkiln/config.yaml` wires — green over the tree that carries it. Anti-pattern 18; `_storymap.md` coverage, AC-003 row: *"A story that installs a pointer and files no row has not finished."* |
| AC-007 | **GIVEN** the repository owner, who must know a doc-comment rewrite inside `happenstance-core` did not silently un-discharge a frozen documentation MUST, **WHEN** they read this story's folder after the merge, **THEN** three artefacts are recorded — the HS-P0020 clause-id set read **before** the first edit, the toolchain the transcript was reproduced on, and the `cargo xtask spec-trace` result — and `spec-trace` is green in **check** mode over the resulting tree, with every displaced `store.rs:NNN` citation in `spec/SPECIFICATION.md` re-anchored to the **same subject**. | `cargo xtask spec-trace` exits 0. The three artefacts exist in `.bklg/docs-that-teach/reach-and-adapter-path/store-error-site-rewrite/`. Re-anchoring is reviewed line by line against `git diff spec/SPECIFICATION.md`: each changed citation's named subject is unchanged. The repair-vs-gap test of `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` is applied and its verdict written down; a verdict of *gap* fires EC-009. |
| AC-008 | **GIVEN** a reviewer holding `_design.md` who cannot read Rust, **WHEN** they open the rendered `happenstance_core::store` page at 1440x900 and at 1024x768 and read the diff beside it, **THEN** the surface obeys the signed-off design: exactly four heading entries and no fifth, no `##` whose entire body is one sentence, the section ≤ **36** source lines against a measured ≈ 34, every added element persistent chrome with no fold, tab, accordion or `<details>`, no raw HTML and no inline `style=`, no "See also" / "Next steps" / "Further reading" block, no skipped heading level, and nothing clipped rather than reflowed — the `error[E0034]` fence's horizontal scrollbar excepted and **expected at both viewports**. | `cargo doc -p happenstance-core --no-deps` then read `target/doc/happenstance_core/store/index.html` at both widths (confirm the `#main-content > .docblock` selector first — `_design.md` flags it as unverified for this crate), and confirm the module doc is **not** rendered inside a collapsed `details.toggle.top-doc` body. Source-line count of the section from the diff. Anti-patterns 5, 9, 10, 13, 16 each checked as a screenshot/diff question. |
| AC-009 | **GIVEN** a reader whose only handle is the string rustc printed and who types it into rustdoc's search box rather than into the file, **WHEN** they search `E0034` or `TraitVariantBlanketType` on `happenstance_core`'s documentation, **THEN** the search index carries them to the two traits — three `#[doc(alias)]` attributes added under a **stated** rule (the string is one rustc, the specification, or a recorded reader question actually emits, and is not the item's own name or a substring of it), establishing that convention in a workspace that has zero occurrences of the attribute today, and recording the rejected alternatives. | `rg -n 'doc\(alias' crates/happenstance-core/src/store.rs` returns exactly **three** and `rg -n 'doc\(alias' crates/ standards/ xtask/` returns only those three. `cargo xtask ci --fast` green (the attributes compile in all three rustdoc builds and on `wasm32`). Rustdoc search exercised by hand on the built page. The rule and its rejections are written in the same change. **No register row** — an alias cannot rot independently of its item. |

**Traceability to the project's spine** (`project.md`, `## Acceptance criteria`;
`_storymap.md`, `## Coverage`). Every project AC traced to this story is covered, and
no criterion is orphaned:

| Project AC | This story's ACs | Note |
| --- | --- | --- |
| Project AC-003 — every installed pointer resolves, and something checks it | AC-005, AC-006 | Split ownership per `_storymap.md`: the register and the guard *rule* are `pointer-policy-and-inventory`'s; this story owns row **P3** and the form its own pointer takes. |
| Project AC-006 — `store.rs` carries rustc's own error output | AC-001, AC-002, AC-004, AC-009 | Sole owner. AC-001 is the string, AC-002 the non-spatial carrier the accessibility floor requires, AC-004 the uncompiled-status statement the criterion names, AC-009 the search reach for the same two strings. |
| Project AC-007 — the error site points at the reasoning account in one hop | AC-003, AC-005 | Sole owner of the pointer; the destination is `adapter-reasoning-account`'s. AC-003 is the half that keeps the hop optional. |
| Project AC-010 — frozen documentation MUSTs still discharged | AC-007 | Sole owner. This is the only story that edits a `happenstance-core` doc comment, which is why it and nothing else sits behind HS-P0020's pin. |
| Project AC-012 — pages obey the discipline and the specification | AC-004, AC-008 | The doc comment is one of the three surfaces this project authors. No clause is restated; the one normative-adjacent claim (what is and is not checked) is a citation into the constitution that resolves. |

## Interaction quality

RFC §6.7/D6. These are blocking invariants, and **every one of them is carried by an
`AC-###` row in the table above** — `redkiln verify` extracts criteria by matching a
leading `| AC-001 |` cell or an `- AC-001:` bullet, so an invariant written only as
prose here would get no ledger row, never be gated and never be tested. This section
says *which* AC carries each invariant and how it is verified; it introduces none.

The medium is a rendered rustdoc page, so "focus", "scroll" and "selection" are real
and observable here even though there is no application: the reader's focus is a
search hit, their scroll is a fragment landing, and their selection is the copy they
paste into a diff.

### State invariants

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In-place before context-jump** — getting unstuck never requires leaving the surface | **AC-003** | The falsification is run, not asserted: remove the destination page and confirm the section still resolves E0034. UX invariant 1; anti-pattern 12. |
| **Non-occlusion** — the addition displaces nothing it points at, and nothing pre-existing is reworded | **AC-008** (heading ladder, `## Naming` and elements (a)/(d)/(g) untouched) and **AC-007** (the displaced `SPECIFICATION.md` citations are repaired to the same subject, not left mis-pointing) | `git diff` shows changed lines only inside `## Import one flavour, not both` plus the three attributes plus the re-anchored citations. UX invariant 2 applied to this surface. |
| **Preserved selection and copy** — the transcript survives being selected and pasted | **AC-001** | Copy the rendered fence, diff against the reproduction run's stderr, find only compiler characters. UX invariant 7 — the invariant sign-off chose over the density budget (F3). |
| **Preserved focus and scroll on a fragment landing** — a reader arriving at `#import-one-flavour-not-both` from a search result lands inside a section that reads correctly from its own top, with the four-entry ladder unchanged around them | **AC-008** | `_design.md`, `## States`, "Entered mid-sequence" row. Checked on the built page by opening the fragment directly. |
| **Reversibility** — the one hop is walk-backable; the reader can tell what they were assumed to have read | **AC-005** | The pointer names the destination *and* the need, so the reader knows what they are leaving for. The destination's half of this invariant (stating what it assumes) is `adapter-reasoning-account`'s, and the observed walk that proves the pair is `error-site-walk-record`'s — **not** claimed here. UX invariant 4. |
| **Keyboard reachability** — every reach installed is a plain link or a rustdoc-native affordance | **AC-005** (a plain link or a named cross-reference; no widget, no hover-only affordance) and **AC-009** (rustdoc search, reached by `S` or `/`) | Nothing in `cargo xtask ci` checks this and `_design.md` says so plainly (`## Density budget`, named gap 2). The keyboard-only *observation* is `error-site-walk-record`'s, which is why it is a different story and a different pair of hands. |
| **No dead end** — the hop terminates somewhere that answers or names the next hop | **AC-005**, bounded | This story installs the pointer only after the destination exists (slice-mate ordering). If the destination is not bound, EC-002 fires and the pointer is not installed. UX invariant 5. |

### Composition invariants

Taken from the signed-off `_design.md`, which is binding on this story
(`## Sign-off`, Ryan Britton, 2026-08-17). An unstyled render satisfies every `rg`
assertion in the table above perfectly; these are the criteria that make that fail.

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — the doc comment is *rendered*, not merely written; it is a real rustdoc page with rustdoc's own class contract, not source anybody has to open a `.rs` file to read | **AC-008** | `cargo doc -p happenstance-core --no-deps`, then `target/doc/happenstance_core/store/index.html` opened. The `#main-content > .docblock` selector is confirmed for *this* crate before it is relied on — `_design.md` flags it as inherited from the verified `happenstance` page and unbuilt for `happenstance_core` at design time. A story that lands the text and never builds the page has not delivered the surface. |
| **Composition and placement** — the seven elements (a)–(g) in the design's binding order, with nothing between (c) and (d) | **AC-003** (the ordering constraint) and **AC-005** (the pointer last, no heading of its own) | `_design.md`, `## Composition`, the `store-module-error-site` table. Checked from the diff by reading the section in source order. |
| **Transience** — everything added is **persistent chrome**; this project installs zero revealed-on-demand and zero opened-on-demand affordances of its own | **AC-008** | `_design.md`, `## Transience policy`: the fence, the plain-words sentence, the fix, the limit sentence and the pointer are each named persistent chrome with their reason. Verified by the absence of any `<details>`, tab strip or accordion in the diff, **and** by confirming on the built page that the module doc is not rendered inside a collapsed `details.toggle.top-doc` body — the one renderer-owned collapse that could hide (b) and (f) without anyone choosing it. |
| **Density budget, with its real numbers** — section ≤ **36** source lines against a measured projection of **≈ 34** (a 2 + b 14 + c 3 + d 3 + e 3 + f 2); fence longest line **109** characters laying out at **1060px** against a **936px** content box | **AC-008** | Source-line count from the diff. The width overflow is recorded as a **failed measurement, not a target** (F4): the fence takes a horizontal scrollbar at *both* 1440x900 and 1024x768, and that is expected. If the cap is exceeded the yield order is (1) the limit sentence compresses to one clause, (2) the connective prose around the pointer, (3) the fence's non-`= note:` context lines — and **the two `= note:` lines and `TraitVariantBlanketType` never yield**, which is why AC-001 and AC-008 are separate rows rather than one. |
| **Hierarchy** — the fence and the in-place fix are **primary**; the plain-words sentence and the limit sentence **secondary**; the pointer **recessive** | **AC-001** and **AC-003** (primary), **AC-002** and **AC-004** (secondary), **AC-005** (recessive) | `_design.md`, `## Hierarchy`. The distinction is carried by order within the section and by the pointer having no heading — no bold, no callout, no device. The moment the pointer stops being recessive, invariant 1 fails. |
| **The design's named anti-patterns** — each checkable by someone who cannot read Rust | 5, 9, 10, 13, 16 → **AC-008**; 7, 8 → **AC-001**; 12 → **AC-003**; 15 → **AC-002**; 14, 18 → **AC-006**; 3 → **AC-005** | `_design.md`, `## Anti-patterns`. Anti-patterns 1, 2, 4, 6, 11 and 17 belong to other surfaces and other stories and are **not** this story's to satisfy; stating that is what stops a reviewer scoring this diff against a README rule. |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | HS-P0020's clause-id pin (its AC-008) is not committed, or lands without an **enumerated** set | **Block.** The first edit to `crates/happenstance-core/src/store.rs` may not be made; copy may be drafted. Do not proceed on judgement — `project.md`'s risk table and N-6 step 1 both make this structural. Escalate to the project, which escalates to the initiative. |
| **EC-002** | The reasoning account's path is not bound, or every rung of the href ladder above *nothing* is unavailable | **The pointer is not installed** — no placeholder href, no "coming soon". Elements (b)–(e) and the three `doc(alias)` attributes still ship; they have no dependency on the account. AC-005 and AC-006 are then unsatisfiable and the story reports them so rather than filing a P3 row with an empty guard (which would itself be anti-pattern 18). `_design.md`, `## States`, "Empty" row. |
| **EC-003** | `cargo xtask spec-trace` fails after the insertion, because the displaced `store.rs:NNN` citations now sit outside `ANCHOR_SLACK` (**12**, `xtask/src/spec_trace.rs:391`) | **Expected, and repaired in the same PR.** Run `cargo xtask spec-trace --write` (`xtask/src/main.rs:671-673`), then re-run in **check** mode; then review each re-anchored citation by hand and confirm the named subject is unchanged. A green check mode alone does not satisfy AC-007. |
| **EC-004** | `spec-trace --write` **declines** — it cannot find a citation's subject within the window and refuses to guess | Do not widen the slack and do not hand-edit a line number to silence it. `xtask/src/spec_trace.rs` is deliberately built so that "a heuristic that cannot tell its own mistakes from the corpus's must decline rather than guess". Re-anchor that citation by hand in the named-subject-plus-path form of `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`, and record which citations needed it. |
| **EC-005** | `clippy::doc_markdown` fires on `TraitVariantBlanketType` once it appears in prose | Add backticks. **Do not** add an `allow` — RS-70-3 (`standards/rust/70-rustdoc-obligations.md:154`) forbids that repair — and do not add the identifier to `clippy.toml`'s `doc-valid-idents`, which is for proper nouns. Inside the `text` fence the string is inert and needs nothing. |
| **EC-006** | `rustdoc::broken_intra_doc_links = "deny"` (`Cargo.toml:134`) fails one of the three rustdoc builds | Someone reached for an intra-doc link to a narrative page. Drop to the next rung of the ladder. The `documentation (no default features)` build (`xtask/src/main.rs:502`) is the one most likely to catch it, and `store.rs:76-78` already records this exact class of bug in this exact file. |
| **EC-007** | The register landed by `pointer-policy-and-inventory` lives outside this spec's `## PR boundary` globs | Widen the boundary **here, deliberately, before the edit** — never at gate time, and never by disabling the boundary check. `redkiln verify --grain story` reads the first fenced block under that heading. |
| **EC-008** | The `#main-content > .docblock` selector does not match on `target/doc/happenstance_core/store/index.html` | Record the real selector and correct it in this story's folder rather than in `_design.md` (an accepted design file is not edited by an implementing story). The design already flags this as unverified for this crate; discovering the true value is expected work, not a defect. |
| **EC-009** | Applying `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` returns **gap**, not **repair** — the edit would change the set of implementations some `[FROZEN]` clause admits | **Stop.** That is an ADR's business, not a documentation story's (`CLAUDE.md`: changing a `[FROZEN]` clause requires a new ADR, not an edit). Record the finding and escalate; do not write the ADR as a side effect of this story. |
| **EC-010** | The reproduction at the pinned toolchain emits a transcript materially different from the one `_design.md`'s mock reproduced | **The compiler wins.** Paste what this run emitted, keep both `= note:` lines and `TraitVariantBlanketType`, and record the divergence and the `rustc --version` alongside AC-007's second artefact. `references/evaluation/review-dx-ergonomics.md:404-414` says what the block must *contain*; it is not the text to copy. |
| **EC-011** | The section exceeds **36** source lines | Apply the yield order in `_design.md`'s density row, in that order, and stop at the first that fits. Never trim a `= note:` line, `TraitVariantBlanketType`, or the in-place fix. If nothing above them yields enough, the section is over budget and the overrun is reported rather than paid for out of AC-001. |

## Non-functional

| id | Requirement | Where it is grounded |
| --- | --- | --- |
| **NF-001** | **No new gate step, and no edit to `xtask/src/`.** This project does not own that tree. The `check_summaries`-shaped register hook and the `doc(alias)`-string assertion are asks handed to HS-P0020, recorded as asks — not implemented here. | Architecture brief N-1, N-4; `_design.md`, `## Visibility and stability` |
| **NF-002** | **No semver surface.** No public item, signature, feature or manifest changes. The three `#[doc(alias)]` attributes add no path a caller can write and are erased before type-checking, so the cost is identical on both port flavours and on `wasm32`. | `_design.md`, `## Items` and `## What it costs a caller`; `_storymap.md` standing constraints |
| **NF-003** | **The four binding constraints are untouched and must remain so:** no `#[async_trait]`; no `serde` in `happenstance-core`'s default features; `EventStore::read` returns the stream at the top level and is not `async`; generic code binds `EventStore`, not `SendEventStore`. This story edits a doc comment and adds attributes; it must not disturb any of them, and the two `memory.rs` tests that assert the third must not be touched. | `CLAUDE.md`, "Binding constraints" 1-4; ADR-0001, ADR-0008 |
| **NF-004** | **Accessibility floor, WCAG AA as it lands in a text medium:** nothing carries meaning in colour or in spatial position alone; nothing animates; theme contrast is rustdoc's and is not defeated (hence no raw HTML, no inline `style=`); link text is self-describing; the heading ladder is unskipped. | UX brief, "Accessibility floor"; carried by AC-002, AC-005, AC-008 |
| **NF-005** | **No new dependency, no toolchain move.** The MSRV stays at 1.97.1; nothing here needs let-chains or any feature above the floor. The pinned toolchain is used to *reproduce* a diagnostic, not to raise anything. | `.kb/decisions/0029-msrv-raised-to-1-97-1.md`; `rust-toolchain.toml` |
| **NF-006** | **The durability claim is bounded and stated as such.** Diagnostic phrasing moves between toolchains, so the page keeps the excerpt on the parts that are stable — the error code and the candidate item names — and does not claim to be a byte-exact transcript of every toolchain. This is the mitigation, not a pretence of checking. | `project.md` risk table, "The restored E0034 excerpt is text"; UX brief, "The `text`-fence limit is accepted, not closed"; AC-004 |
| **NF-007** | **The register stays build-time data.** This story files one row and renders nothing: a page listing every pointer is the second navigation surface DT-10 warns about, and its only reader would be its maintainer. Cap: ≤ 8 rows at project close, ≤ 20 ever. | `_design.md`, `## Transience policy` and `## Density budget`; anti-pattern 14 |
| **NF-008** | **Gate cost is unchanged.** No step is added, no compile unit grows, and the only new work in the gate is `spec-trace` re-reading citations it already read. The story's own bar is `cargo xtask ci --fast` (the non-terminal project bar), not the full gate. | `.redkiln/config.yaml`, `integration_scoped`; `project.md` DoD item 1 |

## Implementation notes (non-prescriptive)

Not instructions — the shape of the work, so the implementer spends their judgement
where it is actually needed.

- **Order is the whole risk here.** N-6's protocol is the first thing to run, not the
  last thing to check: read the clause-id set, decide repair-versus-gap, *then* touch
  the file. The three artefacts AC-007 wants are produced by doing it in that order and
  are unreconstructible afterwards.
- **Reproduce the transcript before writing anything else.** Everything downstream —
  the density count, the yield decision, element (c)'s wording, the alias strings — is
  a function of what rustc actually printed at 1.97.1. A convenient way to provoke it is
  a scratch file that imports both `EventStore` and `SendEventStore` and calls
  `store.read(…)` through method syntax; the scratch file is not committed, its stderr
  and its `rustc --version` are.
- **Expect `spec-trace` to go red, and budget for it.** This is the step most likely to
  make the PR look broken when it is working exactly as designed. Run it before and
  after so the delta is attributable, and keep the `--write` diff separate from the
  prose diff in review if that helps a reviewer see that no subject changed.
- **Element (c) is required *content*, not required *wording*.** The design fixes what
  it must name — the method, both trait names, the cause — and deliberately leaves the
  sentence to the author. Do not treat `_design.md`'s `## Signatures` block for (c) as
  copy to paste; it is a specification of content.
- **Element (f)'s wording is fixed, its href is not.** `_design.md`'s `## Signatures`
  gives both the linked and the named-unlinked P3 wordings. Pick the rung, then take the
  matching sentence; do not blend them.
- **The alias rule is a deliverable.** Zero occurrences today means this change is where
  the convention is established, so the rule and its rejected alternatives are written
  down in the same change rather than left as three attributes a later reader has to
  reverse-engineer.
- **Slice-mate ordering matters.** `adapter-reasoning-account` lands first inside the
  same context; its bound path is an input here. Installing the pointer before the
  destination exists is the "Empty" state and EC-002 governs it.
- **What is deliberately left open:** the exact sentence of (c) and (e); which context
  lines of the transcript survive if EC-011 fires; whether the `spec-trace` repair is one
  commit or two. None of those is a design decision and none needs escalating.

## Tests and CI (merge gate)

**This project takes no `testing` brief, on purpose** — `_decomposition.md`'s warranted-brief
table omits it, and `project.md`'s out-of-scope table says why: the instrument for a
reading surface is an observed walk, and automated checking of the three walks is owned
by nobody, deliberately. So the tiers below are the repository's real gate plus three
file-reading assertions, and the *walk* that closes DoD-10 is a different story's
(`error-site-walk-record`). Nothing here pretends a doctest can perceive a page.

| Tier | Command / path | Proves |
| --- | --- | --- |
| **Reproduction** | Compile a deliberate double-import at the pinned toolchain (`rust-toolchain.toml`, `channel = "1.97.1"`); capture stderr and `rustc --version` into this story's folder | The transcript is rustc's own output at the toolchain the workspace pins, not a transcription of a transcription that predates the MSRV raise — **AC-001**, AC-A05 |
| **Search assertions** (the reader's actual first interaction) | `rg -n "= note:" crates/happenstance-core/src/store.rs` → 2 hits · `rg -n TraitVariantBlanketType crates/happenstance-core/src/store.rs` → ≥ 1 · `rg -n 'doc\(alias' crates/ standards/ xtask/` → exactly 3, all in `store.rs` | The strings the reader searches are in the file they already have open, and the alias set is exactly the three the rule admits — **AC-001**, **AC-009**. Each of the first two returns **nothing** on `main`, which is what makes them non-decorative |
| **Guard-is-real assertion** | `rg -n "compile_fail,E0034" standards/rust/20-two-flavour-ports.md standards/rust/00-prime-directives.md` → 2 hits | The narrow limit element (e) states is true: the error *code* really is asserted by a compiled negative elsewhere — **AC-004** |
| **Constitution doctests** | `cargo test --locked -p xtask --doc` with `RUSTDOCFLAGS=-D warnings` (`xtask/src/constitution.rs`) | Those two `compile_fail,E0034` fences are compiled, so the guard AC-004 cites is a build failure and not a comment — **AC-004** |
| **Story grain** | `cargo xtask affected --base main` — the command `.redkiln/config.yaml` wires to `affected_gate` | fmt, clippy `-D warnings` (which is where EC-005's `doc_markdown` fires), the tests for the affected packages, the five file-reading lints, and `spec-trace` — the whole story-grain bar in one command — **AC-005**, **AC-007**, **AC-008** |
| **Citation integrity** | `cargo xtask spec-trace` in **check** mode, after `--write` and after a hand review of each re-anchored citation | Every `store.rs:NNN` citation in `spec/SPECIFICATION.md` still resolves *and* still names the same subject — **AC-007**. Check mode alone is necessary and not sufficient; the subject review is the other half |
| **Reachability, static** | `cargo xtask lints && cargo xtask spec-trace` — `.redkiln/config.yaml`'s `reachability_static` | The register row and the pinned-tree references are read by something, not just written — **AC-006** |
| **Render** | `cargo doc -p happenstance-core --no-deps`, then `target/doc/happenstance_core/store/index.html` opened at 1440x900 and 1024x768 | The doc comment is a *page*: selector confirmed, ladder intact, fence scrolling rather than clipping, module doc not inside a collapsed toggle — **AC-002**, **AC-008**. The only tier that can fail on presentation |
| **Link-form gate** | `cargo xtask ci --fast` — includes `documentation (no default features)` (`xtask/src/main.rs:502`) under `rustdoc::broken_intra_doc_links = "deny"` (`Cargo.toml:134`) and all four wasm32 steps | The pointer's form resolves in every feature configuration, and the three attributes cost nothing on `wasm32` — **AC-005**, NF-002 |
| **Falsification, executed** | With the destination page removed from the working tree, re-read `## Import one flavour, not both` | A reader who never follows the pointer is still correctly unstuck — **AC-003**, UX invariant 1. Run it; do not assert it |
| **Merge bar** | `cargo xtask ci --fast` green on the merged result (`project.md` DoD item 1); `redkiln verify --grain story` with `require_ledger: true` and `require_commit_provenance: true` | The non-terminal project bar, plus every AC in `_ledger.md` flipped with cited evidence and a recorded work commit |

**Not a tier, and named so nobody adds it:** a `rust` fence around the diagnostic to buy
a green tick. Nothing compiles a compiler transcript; wrapping one in a `rust` fence is
the `ignore`-fence anti-pattern wearing a compiler
(`standards/rust/62-doctests-and-harnesses.md` governs *compiled* fences, and a
diagnostic is not one).

## Risks and coupling (PR-scoped)

| Risk / coupling | Scope | Mitigation in this PR |
| --- | --- | --- |
| **The `spec-trace` breakage looks like a defect and gets "fixed" wrongly** | This PR | The most likely failure mode of this change is a reviewer or a later agent widening `ANCHOR_SLACK` or hand-editing a line number instead of re-anchoring. EC-003/EC-004 state the repair; the spec says up front that the red is expected and in scope. |
| **A later reader "fixes" the fence by reflowing it** | Beyond this PR | The 109-character line is recorded in `_design.md` as a *failed measurement, not a target*, and anti-pattern 16's exception is load-bearing at both viewports. This spec repeats it (AC-008, NF-006) precisely because the temptation recurs every time someone opens the file at a narrow width. |
| **The clause pin never lands** | Blocks the PR | EC-001: block, do not proceed on judgement. This is why the story sits behind HS-P0020 in the DAG rather than behind a note (`project.md` risk table). |
| **The destination path is bound late and the pointer's rung changes under it** | This PR, slice-internal | The pointer's *sentence* is href-independent (UX-AC-02); only the rung is late-bound, and the rung actually taken is what row P3 records. Slice-mate `adapter-reasoning-account` lands first inside the same context, which is the whole reason the two are one slice. |
| **The register's home is not yet bound, so the PR boundary may be wrong** | This PR | `## PR boundary` says the boundary is widened here, deliberately, before the edit — never at gate time. EC-007. |
| **Author-as-witness** | Excluded by construction | The observed walk that closes DoD-10 is `error-site-walk-record`'s and must be a different pair of hands (`_storymap.md`, B6: *"putting a walk in the same slice as the surface it walks would make the author their own witness"*). This story must not report a walk. |
| **Doc-comment growth above an item is exactly what `ANCHOR_SLACK` was widened for — and this exceeds it** | This PR | Stated rather than discovered: the slack is 12 and deliberately wider than the constitution lint's ten *precisely* so a growing doc comment does not red the gate; this insertion is ≈ 21 source lines and outruns it. The repair is in scope; the design of the slack is not being questioned. |
| **`crates/happenstance-core/src/store.rs` is shared with nothing, but `spec/SPECIFICATION.md` is shared with everything** | This PR | The edit to the specification is mechanical (line numbers only) and no clause **text** changes. Any pressure to also amend a clause is EC-009 and leaves this story. |
| **The three `doc(alias)` strings can go stale if rustc renames the internal type** | Beyond this PR | Named, not mitigated: `_design.md` records that the cheap check (an `rg` assertion that every alias string also appears in the fence) does not exist and that this project does not own `xtask/src/`. It is offered to HS-P0020 and is **not counted as a guard**. |
| **Merge-forward with `initiative/from-contract-to-published-library`** | Not this story's | That branch diverges on `crates/happenstance/` — the front-door surfaces — which this story does not touch. `crates/happenstance-core/src/store.rs` is not in that seam. Recorded so the merge-forward rule is not applied here by reflex. |

## Dependencies

**Blocks on** (`depends_on`, and both are real edges, not courtesy):

- **`pointer-policy-and-inventory`** — supplies the register this story files row **P3**
  into, the permitted pointer forms, the href ladder as binding rather than advisory,
  and the widget-rejection record this story inherits as a constraint. Without it there
  is nowhere to file the row, and AC-006 is unsatisfiable.
- **`adapter-reasoning-account`** — supplies the destination. It is a **slice-mate**,
  implemented in the same context and ahead of this story, because a pointer installed
  before its destination exists is the "Empty" state and must not be installed at all.

**External gates, outside this initiative's control** (`_storymap.md`, merge order,
slice 2): HS-P0020's clause-id pin must be committed and readable before the *first*
edit to `crates/happenstance-core/src/store.rs` (EC-001), and its pinned tree must
exist for the account to be a registered page rather than a loose file (AC-A03).
This story carries **no** HS-P0022 edge — that is the parallelism AC-A08 requires
the map to express, and collapsing it costs the project its only concurrency.

**Unlocks**:

- **`error-site-walk-record`** — the observed, keyboard-only walk that closes initiative
  DoD-10. It runs as soon as this slice has merged and does not wait on the reach half.

**Does not block, and is not blocked by**: `front-door-pointer`,
`evaluator-onward-links`, `front-door-walk-record`, `second-question-walk-records` —
the reach half, which waits on HS-P0022's pages existing.

## Anchors (progressive disclosure)

Deferred depth, not optional depth. The `## Context pack` above is self-sufficient for
starting; each row below is opened at the moment named, and nothing here is pasted
inline. Every path was confirmed present in this worktree before it was cited.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/reach-and-adapter-path/_design.md` | The signed-off, **binding** design: `## Composition`'s element order, `## Density budget`'s 36/34/109 numbers and yield order, `## Hierarchy`'s primary/secondary/recessive split, `## Signatures`' exact P3 wordings, and the eighteen anti-patterns. This story implements it and may not contradict it. | **Before the first edit**, and again before counting lines against the cap. | AC-003, AC-005, AC-008 |
| `.bklg/docs-that-teach/reach-and-adapter-path/design/mock.html` | The 38-frame sign-off mock. Its `store-module-error-site` frames are the only picture of what this section looks like at both viewports, and its closing section carries F2/F3/F4 — the findings that raised the cap and recorded the fence width as unmeetable. | When the section's rendered length or the fence's overflow is in question, and before deciding anything yields. | AC-008 |
| `crates/happenstance-core/src/store.rs` | The mount point itself: the four-entry heading ladder, the trimmed fence at `:36-41`, the in-place fix at `:43-45` that must survive, the live named-but-unlinked cross-reference at `:76-78` that is the pointer's fallback form, and the two traits at `:93` the aliases attach to. | **First**, and continuously. | AC-001, AC-002, AC-003, AC-005, AC-009 |
| `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` | The **primary** test N-6 applies, and the one `project.md` does not name first: a correction is a *repair* if the set of implementations the clause admits is unchanged, otherwise a *gap* and an ADR's business. It also gives the safe form for an obligation already met. | **Before the first edit**, immediately after reading the clause-id set. | AC-007 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The secondary test — "does the edit change what the document asserts, not whether it changes the document". All three of its worked instances are ADR-record renames, so applying it to a doc comment is an analogy sanctioned by name in `CLAUDE.md`, and the story must say so rather than cite it as literal precedent. | Alongside the playbook, at the same moment. | AC-007 |
| `xtask/src/spec_trace.rs` | `ANCHOR_SLACK = 12` at `:391` is the number this insertion outruns, and the surrounding code is why the tool **declines** rather than guessing when it cannot find a subject. Reading it is what stops EC-004 being "fixed" by widening the constant. | When `spec-trace` first goes red — which it will. | AC-007 |
| `xtask/src/main.rs` | `documentation (no default features)` at `:502` is the rustdoc build that constrains the pointer's form; `spec-trace [--write]` is dispatched at `:671-673`; the `probe` contract at `:85-102` is why a step that runs and finds a problem is never a skip. | Before choosing the pointer's rung, and when running the repair. | AC-005, AC-007 |
| `standards/rust/70-rustdoc-obligations.md` | RS-70-2 (`:95`) is the sanctioned named-but-unlinked form the fallback rung uses; RS-70-3 (`:154`) forbids repairing `doc_markdown` with an `allow`, which is EC-005's only correct resolution. | Before writing element (f), and the moment clippy complains about CamelCase. | AC-005 |
| `standards/rust/20-two-flavour-ports.md` and `standards/rust/00-prime-directives.md` | The two `rust,compile_fail,E0034` fences at `:179` and `:242` respectively — the compiled negatives that make element (e)'s narrow claim true. Element (e) **cites** these; it does not duplicate them. | While writing element (e), to confirm the guard before claiming it. | AC-004 |
| `xtask/src/constitution.rs` | The step that compiles those two fences (`cargo test --locked -p xtask --doc`, `RUSTDOCFLAGS=-D warnings`), and the file that explains why `lint-constitution` reads a *directory* rather than trusting `cfg(doctest)` — the registration hole this project inherits. | With the two constitution atoms, when substantiating AC-004. | AC-004 |
| `xtask/src/lint_constitution.rs` | `SUMMARIES` at `:64` and `check_summaries` at `:463-473` are the working precedent the pointer register is modelled on — a named list of surfaces crossed with a pinned constant, failing the build with the reason in the message. Shape only; the register itself is `pointer-policy-and-inventory`'s. | When filing row P3, to check the row carries what the precedent's failure message would need. | AC-006 |
| `references/evaluation/review-dx-ergonomics.md` | `:404-414` says what the restored block must **contain** (both `= note:` lines, the `help:` line) and is explicitly *not* the text to copy — it predates the MSRV raise. `:421-422` is the `prelude` proposal, live and evidenced and out of boundary. | Before reproducing the transcript, to know what a complete block looks like; and if anyone proposes the `prelude` inside this PR. | AC-001 |
| `.kb/decisions/0029-msrv-raised-to-1-97-1.md` | Why 1.97.1 is the toolchain the transcript is reproduced on, and why the older reference text cannot be trusted verbatim. | Once, before the reproduction run. | AC-001 |
| `.kb/decisions/0001-async-port-flavours.md` | The decision the E0034 collision is a *consequence* of. The section documents the collision; it must not restate or relitigate the decision (DR-8). | If tempted to explain *why* two flavours exist inside `store.rs` — that belongs to the reasoning account. | AC-004 |
| `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` | The named-subject-plus-path citation form, for the citations `spec-trace --write` declines to move automatically. | Only if EC-004 fires. | AC-007 |
| `spec/SPECIFICATION.md` | The displaced citations themselves — `store.rs:93-268` at line 371, plus `store.rs:101`, `:119-123`, `:131-139`, `:213-217`, `:321-331` and others. The subjects are what must survive re-anchoring; the line numbers are not. | After the insertion, during the `spec-trace` repair. | AC-007 |
| `.bklg/docs-that-teach/reach-and-adapter-path/_decomposition.md` | The UX brief's nine interaction-quality invariants with their falsifications and the accessibility floor; the architecture brief's N-3 form table, N-6 protocol, N-7 narrow limit and AC-A01..A08. The invariants above are distilled from it; the falsifications are not. | Before AC-002 and AC-003, whose falsifications are stated there and executed here. | AC-002, AC-003 |
| `.bklg/docs-that-teach/reach-and-adapter-path/_storymap.md` | The slice cut, the merge order with its hard external gates, and the coverage table's split-ownership rule — including *"a story that installs a pointer and files no row has not finished"*. | When scoping doubt arises about what belongs to this story versus a slice-mate. | AC-006 |
| `.bklg/docs-that-teach/reach-and-adapter-path/_grounding.md` | The verified state of the sibling projects this story's external gates depend on — HS-P0020 and HS-P0021 at `stage: storymap` with no `_design.md` at grounding time. It is what makes EC-001 a live condition rather than a formality. | Before assuming the clause pin exists. | AC-007 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | Persona 2's goal in their own words — telling *"the port is wrong for me"* from *"I have not understood the port yet"* — and the qualification that no persona has been directly observed, which bounds every "the reader wants…" in this spec. | When judging whether a sentence serves the reader or the author. | AC-005 |
| `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` | The evidence behind the built-in-surface-first rule and the finding that the gap "is evidence of a missing *link*, not a missing *widget*" — the reason no navigation affordance may be proposed here. | If any structural addition beyond a sentence starts to look attractive. | AC-005 |
| `.bklg/docs-that-teach/reach-and-adapter-path/project.md` | The project spine AC-001..AC-012, the seven-item DoD, and the risk table entry that makes "block rather than proceed on judgement" the required behaviour when the clause pin is absent. | At scoping questions and at EC-001. | AC-007 |
| `Cargo.toml` | `rustdoc::broken_intra_doc_links = "deny"` at `:134` — the line that turns Decision 6 from a preference into a build failure. | Before writing element (f). | AC-005 |
| `rust-toolchain.toml` | `channel = "1.97.1"` — the toolchain the transcript must be reproduced on, and the one AC-007's second artefact records. | At the reproduction run. | AC-001 |
| `crates/happenstance-core/src/lib.rs` | `pub mod store;` at `:99` — the declaration that gives the module its own rendered page and its own deep-linkable fragments, which is *why* a reader can land here with the crate root unseen and why a front-door-only pointer policy was rejected. | When questioning whether this surface needs its own pointer at all. | AC-005 |
| `.redkiln/config.yaml` | The real gate commands this story is measured by — `affected_gate`, `reachability_static`, `integration_scoped` — plus `require_ledger` and `require_commit_provenance`, and lines 75-81 recording why the perceptual review is a standing skip. | Before running anything, and before claiming the story is done. | AC-008 |

## Clarifications resolved during spec

1. **The AC set is exactly the nine the front half enumerated** — AC-001 … AC-009, in
   the order the `## Behavior and interfaces` table lands them. None added, none dropped.
   The ledger matches them one for one.
2. **Every traced project AC is covered, and the two shared ones are covered at their
   split.** AC-003 and AC-012 are shared with sibling stories by `_storymap.md`'s
   coverage table; this story's ACs claim only its half — row P3 and the form of its own
   pointer for AC-003, the doc comment as one authored surface for AC-012 — and say so
   in the traceability table rather than implying sole ownership.
3. **The interaction-quality invariants are AC rows, not bullets.** Every state and
   composition invariant that applies is carried by a numbered AC in the table; the
   `## Interaction quality` section only maps them. This is deliberate: `redkiln verify`
   extracts criteria from a leading `| AC-001 |` cell, so an invariant that lived only as
   prose there would never be gated. Composition is why AC-008 exists as its own row —
   every `rg` assertion in this spec passes against a page that never rendered.
4. **Reversibility and keyboard reachability are carried, but their *observation* is
   another story's.** AC-005 requires the pointer to be a plain, self-describing,
   keyboard-reachable link and to name the need it answers; it does not claim a walk.
   The keyboard-only observed walk is `error-site-walk-record`'s and must be a different
   pair of hands. Claiming it here would make the author their own witness, which
   `_storymap.md` singles out as the one thing B6 exists to prevent.
5. **The `spec-trace` breakage is in scope, and is stated as an expected red rather than
   a risk.** It is EC-003 with a defined repair and AC-007's third artefact, not a
   contingency — because the failure mode worth guarding against is not the red, it is
   somebody widening `ANCHOR_SLACK` to make it green.
6. **`spec-trace` green in check mode is necessary and not sufficient for AC-007.** The
   second half — every re-anchored citation still names the same subject — is a human
   review of `git diff spec/SPECIFICATION.md`, and the AC says so, because a tool that
   re-anchors to the wrong subject exits 0.
7. **No conformance rule is added, and the absence is stated rather than left blank.**
   `CLAUDE.md` requires a story that changes a port to name a rule; this story's answer
   is that it changes no port, no signature and no bound, so nothing an adapter could
   implement wrongly is observable here. The nearest thing to a rule for this behaviour —
   the two compiled `compile_fail,E0034` fences — already exists and is cited, not
   duplicated.
8. **No ADR is written or implied.** Two candidates were live and both are recorded as
   out of boundary rather than decided: the `prelude` module (a public API addition with
   a semver surface) and any amendment to a `[FROZEN]` clause (EC-009). Recording the gap
   and letting the runbook's ADR pass decide it is the standing discipline; a spec that
   settles one as a side effect is the failure mode.
9. **The design file is not edited by this story.** Where implementation discovers the
   design was wrong about a fact — the `#main-content > .docblock` selector for
   `happenstance_core` is the flagged candidate (EC-008) — the correction is recorded in
   this story's folder. An accepted, signed-off design is not amended by the story
   implementing it.
10. **The verification column names real commands over real paths.** There is no
    automated test that can perceive prose, so the tiers are the repository's own gate
    (`cargo xtask affected`, `spec-trace`, `ci --fast`, the constitution doctests) plus
    three `rg` assertions that return **nothing** on `main` — which is what keeps them
    from being decorative — plus one render pass and one executed falsification. The
    project takes no `testing` brief and this spec does not invent one.
