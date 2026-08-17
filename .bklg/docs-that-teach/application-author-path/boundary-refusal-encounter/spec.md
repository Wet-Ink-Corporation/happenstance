---
item: HS-S0185
stage: spec
created: 2026-08-17T13:16:33.036Z
updated: 2026-08-17T13:16:33.036Z
template_sig: 87bbf1d0
rendered_sig: e7c47148
---

# Spec — The opening encounter reaches a refused append

## Scope lock

| | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` |
| Project | `.bklg/docs-that-teach/application-author-path/project.md` |
| This spec | `.bklg/docs-that-teach/application-author-path/boundary-refusal-encounter/spec.md` |
| Key briefs | `.bklg/docs-that-teach/application-author-path/_decomposition.md` — UX brief `:11-444`, testing brief `:448-650` |
| Signed-off design (**binding**) | `.bklg/docs-that-teach/application-author-path/_design.md` (approved `:1039`) |
| Grounding | `.bklg/docs-that-teach/application-author-path/_grounding.md` |
| Story map / merge order | `.bklg/docs-that-teach/application-author-path/_storymap.md` (this story `:57`) |
| Substrate design (consumed, not built) | `.bklg/docs-that-teach/checked-documentation-surface/_design.md` — `TREE`/`HARNESS`/`IGNORE_ALLOWANCES`/`HIDDEN_MARKERS` at `:85-107`, D1 "the markdown is the render" at `:165-198`, page composition at `:279-318` |
| Blocking dependency records | `.bklg/docs-that-teach/application-author-path/merge-forward-preflight/spec.md`, `.bklg/docs-that-teach/application-author-path/tension-resolutions/spec.md` |
| Roadmap pointer | `RUNBOOK.md` — this initiative is documentation work and adds no phase; the roadmap is not amended here |

## One-line PR slice

Author the opening encounter into HS-P0020's pinned tree in DT-4's resolved shape, so a
reader following it from step one runs a program whose own output carries
`AppendError::ConditionViolated` (ES-25) against a real `MemoryEventStore`, with every step
anchor answerable cold and no hidden `#`-prefixed line carrying any part of the boundary.

## Executive summary

**What this PR lands.** Two reader-facing surfaces and their mounts: `docs/first-encounter.md`
— one page, three steps, each a complete runnable program — registered in HS-P0020's harness
and indexed in `docs/README.md`'s narrative table; and the rewrite of
`crates/happenstance/src/lib.rs`'s `# Using it today` section into `## Watch a boundary
refuse`, whose fence is an executing program that prints a refusal before it asserts one.
Nothing else. The falsification drill that makes the boundary load-bearing to the repository
is the slice-mate's (`boundary-falsification-drill`, AC-005); the bridge and the worked-example
handoff are the next slice's.

**Pointer, then delta.** The *what* is already decided and is not restated here: DT-4's three
steps and their anchors (`_design.md:134-140`), the composition of both surfaces
(`_design.md:415-472`), the density budget (`:532-617`), the transience policy (`:507-528`) and
the fifteen anti-patterns (`:831-882`). This story implements that design; it does not re-open
it. What this spec adds is five things that were **measured against the real API in this
worktree**, each of which turns a piece of the plan into something an implementer would
otherwise discover with a red compiler or a red reviewer:

1. **`Tags::from_iter([("course", "c1")])` does not compile.** `FromIterator` on `Tags` is over
   `Tag`, not over pairs (`crates/happenstance-core/src/tag.rs:479`). The verified spelling is
   `Tags::from_pairs([("course", "c1")])?` (`:304-306`, with the crate's own doctest at
   `:297-303`). The design's fence sketch (`_design.md:331-334`) uses the former; the design
   itself delegates "the remaining call spellings" to the implementer against the merged tree
   (`_design.md:366-371`), so this is a spelling correction inside the delegation, not a design
   change.
2. **The printed payoff cannot be a bare `Debug`.** `ConditionViolated`'s `Display` never
   contains the token `ConditionViolated` — it renders "append condition violated: the store
   already contains a matching event … rebuild the decision model and retry"
   (`crates/happenstance-core/src/error.rs:168-182`) — while the bare `{refused:?}` line is 92
   characters (`_design.md` finding F-3, `:997`), which breaches the 68-column fence budget
   (`:552-562`) and trips anti-pattern 4. Anti-pattern 10 requires the token to be *in the
   program's printed output*. The only shape that satisfies all three is a `println!` reached
   from inside a matched `Err(AppendError::ConditionViolated(_))` arm — a line that is
   unreachable unless the boundary actually refused.
3. **The taught vocabulary reaches the whole binding shape.** `read_decision_model` is
   available as `happenstance::read_decision_model` through the facade's glob re-export
   (`crates/happenstance/src/lib.rs:75` → `crates/happenstance-core/src/lib.rs:117`), so
   `_design.md`'s corrected fence is writable entirely under `use happenstance::{…}` with no
   `happenstance_core` import (UX-011, anti-pattern 13).
4. **The mount is two-sided, and only one side exists today.** The crate root mounts itself:
   its doctest is already swept by the `"tests"` REQUIRED step (`xtask/src/main.rs:143-155`).
   The tree pages mount only through `xtask/src/narrative.rs`, which HS-P0020 owns and which is
   **not in the tree yet** (`xtask/src/` holds ten modules, none of them `narrative.rs`). A page
   the harness does not `include_str!` is HS-P0020's *unregistered* problem state
   (`checked-documentation-surface/_design.md:561-562`) — the documentation equivalent of built
   but unmounted.
5. **Step 3's program ships twice, deliberately.** `include_str!` cannot cross the package
   boundary of a published crate (`crates/happenstance/src/lib.rs:7-9`), so the crate root and
   the tree page each carry their own copy, and **both are compiled and executed**
   (`_design.md:707-726`). Drift between them can be silent about identity; it cannot be silent
   about correctness.

## Context pack

Everything below is a decision this story must honour. Deeper material stays behind the
signposted anchors; nothing here needs another file to be actionable.

**The measured defect this story closes.** Today the crate root's only fence constructs a
`MemoryEventStore` and asserts the log is empty (`crates/happenstance/src/lib.rs:58-67`),
inside a `# async fn example()` wrapper that is never awaited (`:61-66`) — it type-checks a
program nobody runs, thirty lines below prose claiming that composing decision models "is the
mechanism that makes a dynamic consistency boundary *dynamic*" (`:44-46`). Both statements are
true; together they teach syntax while claiming semantics (`project.md:66-77`). The fix is not
more prose. It is a fence that **executes** and whose execution refuses an append.

**DT-4 is resolved and binding: staged, minimal-first, three steps, no fourth "whole program"
artifact** (`_design.md:122-168`). Step 1 appends two events and reads them back (`Event`,
`Tags`, `Query`, `read`); step 2 appends under an `AppendCondition` that is satisfied (`Guard`,
`AppendCondition`, `after`); step 3 reads, lets another writer win the race, and watches the
guarded append be **refused** (`_design.md:134-140`). The evidence is Carroll's training-wheels
finding — a controlled, replicated result that a constrained start builds a better model
*measured after the constraint is lifted* — not a preference. The rejected option (b), one
complete program with commentary, is the current defect's own shape.

**The failure mode of that choice is mitigated structurally, not advisorily**, and each
mitigation is a thing this story builds: (1) **no step is a fragment** — each fence is a
complete program that compiles and runs alone, which removes the interdependence the
literature's named failure mode is about; (2) **an in-body two-line step header block** under
every step heading — line 1 names what the previous step established, line 2 is a one-hop link
to step one — in the body specifically because rustdoc's sidebar TOC is removed entirely below
700px and a mitigation that disappears on a phone is not one; (3) **the payoff sits in the step
most likely to be landed on cold** — step 3 is the refusal, so a reader arriving from search
gets the thing the library is for on the page they landed on (`_design.md:142-159`).

**DT-1 binds this story by what it forbids.** The prior model is named in exactly one place in
the whole page set — the bridge's "Where your streams went" heading, which
`invariant-to-appendcondition-bridge` authors — and **the crate root and every step of the
opening encounter name no prior model at all** (`_design.md:108-118`). Their material is the
invariant and the refusal. Concretely: the words "aggregate", "your aggregates", "one stream
per entity" and "which stream" do not appear on any page this story writes (anti-pattern 6,
`_design.md:852-854`). Where a page here relies on the anchor decision it links to that
heading; it does not re-argue it (UX-009, DR-07).

**DT-6 binds this story by exclusion.** The wrong-side contrast is the bridge page's, not this
one's, and this project ships **zero** uncompiled fences and puts nothing on HS-P0020's
enumerated allowance list (`_design.md:206-208`). So every fence this story authors is compiled
*and executed*: no `ignore`, no `no_run`, no `compile_fail`. `no_run` is the concrete failure
mode named in the testing brief — it passes the compiled-fence tier while silently never
reaching the executed tier, which is "compiles forever while quietly ceasing to demonstrate its
own claim", the initiative's first-ranked risk (`_decomposition.md:616-625`, `project.md:335`).

**The refusal must be observable, not narrated** (IQ-7, `_decomposition.md:293-297`). The test
is destructive: delete every sentence around the fence and a reader who runs the code still
sees the boundary refuse. That is why the fence prints before it asserts — an assertion is
invisible when it passes.

**The boundary must be visible in the render, not merely present in the source** (UX-003,
IQ-2.1, DR-02). Rustdoc's `#` prefix does not hide a line behind an affordance; it removes it
from the DOM entirely — verified on the current render, where only the four visible lines of
the crate-root example exist in `index.html` and the five hidden ones are absent with no hover,
focus or toggle that recovers them (`_design.md:524`). Hiding is therefore permitted **only**
for a `use` block already shown visibly earlier on the same page, `Ok(())`, and struct-literal
filler; it is **forbidden** for any `Query`, `QueryItem`, `Tags`, `Guard`, `AppendCondition`,
the append call, the read call, and every assertion. A reader must be able to rebuild the
boundary from what they can see.

**AC-006's shape, stated as a positive obligation.** No page here constructs an empty
consistency boundary at a point where the surrounding prose claims a real one. The guard's
query is tagged to the *invariant being held*, not to the row being written, and both the tag
join and the `after` are load-bearing — remove either and the scenario stops refusing. The
already-written cautionary case is CF-7 (`spec/SPECIFICATION.md:7266`, `[FROZEN]`): an adapter
that drops the tag join matches more than it should. This story's pages do not restate that;
they cite it if they need it.

**The normative voice is borrowed, never re-issued.** ES-25 is the clause the whole encounter
makes concrete — the store MUST reject the append if and only if it holds a matching event
strictly beyond the guard's `after`, and a rejection MUST be reported as
`AppendError::ConditionViolated` (`spec/SPECIFICATION.md`, `#### ES-25` at `:3695` in this
worktree's pre-merge copy; `[FROZEN]`). Cite the id, never the prose (AC-011, DR-09,
anti-pattern 11). Clause **ids** are stable and are never renumbered (`:280`); **lines are
not**, and the merge moves them — the authority for every line in this spec after the merge is
`merge-forward-preflight`'s baseline record, not this file.

**The mount, and what "mounted" means in this medium.** HS-P0020 pinned the tree as `docs/`
and decided **the markdown is the render** — no mdBook, no site, no second render step
(`checked-documentation-surface/_design.md:165-198`). A page becomes part of the checked
surface by being `include_str!`-registered as its own `#[cfg(doctest)] mod` in
`xtask/src/narrative.rs`, declared from `xtask/src/lib.rs` — one module per page, so a failure
names the page (`:491-499`, the same lesson `xtask/src/constitution.rs:11-18` already paid
for). A page in `docs/` that the harness does not name is *unregistered*, and a harness entry
naming a page that does not exist is a *dangling registration*; both are checker problem states
(`:561-563`). Registering the page **is** the mount, and it is the difference between a page
the gate compiles and a file that happens to be in the repository.

**The substrate is consumed, never re-implemented.** If `xtask/src/narrative.rs` is not in the
tree when this story is implemented, the response is to **halt and route to HS-P0020**, not to
build a parallel tree, a second renderer, or a page outside it — that is out of contract
(`_decomposition.md:118-131`). The same holds for HS-P0021's answered-need notation: this
design fixes its *position* (above everything, above the first fence) and its *cardinality*
(exactly one) because those are composition decisions; its *form* is HS-P0021's and inventing a
second notation is a defect (DR-14, `_design.md:900-903`).

**Two gates already run against this story whether or not anyone types them**: `cargo xtask
affected --base main` at the story grain (`.redkiln/config.yaml:40`) and `cargo xtask ci
--fast` at this non-terminal project's integration grain (`:55`). The step that actually
executes this story's fences is `"tests"` — `cargo test --locked --workspace --all-features --
--show-output` (`xtask/src/main.rs:143-155`) — which sweeps doctests, so the crate-root fence
is executed by machinery that already exists.

**The persona-journey slice.** Backbone activity A2 — *"Show me the thing this library is for,
working"* (`_storymap.md:42`), Persona 1's UI-1. Their stated fear is silent wrongness: "a
mental model that looks right, compiles, runs, and is quietly wrong". Every choice above is
answerable to that fear rather than to page count; the initiative's own non-goal is that pages
written is not the measure.

**What this story may not decide.** `_design.md` is signed off by a human (`:1039`) and binding
(`:10-12`). Where implementation contradicts it, this story records the contradiction and
routes it — substrate gaps to HS-P0020, pointer and reach gaps to HS-P0023, comprehension
doubts to HS-P0024, incidental bugs to the `support` initiative (`.redkiln/config.yaml:5`) —
per project DoD item 9 (`project.md:300-302`). It does not amend the design, and it does not
re-open DT-1, DT-4, DT-5 or DT-6.

## Integration contract

- **Archetype**: `capability` — a user-observable slice through every layer the reader has: the
  page, the fence, the executed program, the refusal in its output, and the gate step that
  keeps all four honest.
- **Slice / milestone**: `opening-encounter`. Slice-mate: `boundary-falsification-drill`
  (HS-S0186), which lands the falsification drill as a `###` under this story's step 3
  (`_design.md:468-471`) and makes the boundary load-bearing to the repository. The two are
  implemented in one context and mounted as one integrated surface: an encounter without the
  drill is precisely the failure this initiative exists to prevent (`_storymap.md:70-74`).
- **Mount point**: **`xtask/src/narrative.rs`** — HS-P0020's narrative harness, one
  `#[cfg(doctest)] mod` per page, declared from `xtask/src/lib.rs`
  (`checked-documentation-surface/_design.md:95-96`, `:491-493`). Adding this page's
  `include_str!` module is the mount; without it the page's fences are compiled by nothing and
  the checker reports *unregistered* (`:561-562`). **Co-mount**, for the second surface:
  `crates/happenstance/src/lib.rs` — the crate root is its own render path (rustdoc →
  `/happenstance/index.html`, docs.rs after publish) and its doctest is already swept by the
  `"tests"` REQUIRED step, so that half mounts itself the moment the fence is written. The
  index row in `docs/README.md`'s narrative table (`checked-documentation-surface/_design.md:306-318`)
  is the third wiring touch and is what makes the page reachable rather than merely compiled.
- **Wires into** (real contracts, by path, all verified in this worktree):
  - `crates/happenstance/src/lib.rs:75` — `pub use happenstance_core::*;`, the facade through
    which the taught vocabulary `use happenstance::{…}` resolves (ADR-0006,
    `.kb/decisions/0006-bare-name-to-the-typed-layer.md`).
  - `crates/happenstance-core/src/store.rs:213-217` — `EventStore::append(&self, events:
    &[Event], condition: Option<&AppendCondition>) -> Result<SequencePosition,
    AppendError<Self::Error>>`; `:119-123` — `read` returns the stream at the top level and is
    not `async`; `:321-331` — `read_decision_model(store, query) -> (Vec<SequencedEvent>,
    Option<SequencePosition>)`, whose second element is exactly what `after_opt` expects
    (`:132-139` says so in the crate's own words). Bind **`EventStore`**, never
    `SendEventStore`, and import only one of the two names (CLAUDE.md binding constraint 4).
  - `crates/happenstance-core/src/append.rs:143-146` — `AppendCondition::new(Query)`; `:201` —
    `after_opt(Option<SequencePosition>)`; `:131` — `Guard` is `#[non_exhaustive]` with **no
    constructor**, which is why the binding fence never writes `Guard::new`
    (`_design.md:354-364`).
  - `crates/happenstance-core/src/query.rs:56` — `QueryItem::new(types, Tags)`; `:185` —
    `Query::from_items`; `crates/happenstance-core/src/tag.rs:304-306` — `Tags::from_pairs`.
  - `crates/happenstance-core/src/error.rs:133-184` — `ConditionViolated`, its
    `conflicting_position` hint and its hand-written `Display`.
  - `crates/happenstance/Cargo.toml` — the `[dev-dependencies]` `tokio` (`macros`, `rt`,
    `rt-multi-thread`) that makes `#[tokio::main]` in a doctest possible. It arrives with the
    merge, already present on the sibling; `_design.md`'s `## Items` row calling it `added` is
    dispositioned to `unchanged` in `merge-forward-preflight`'s baseline record
    (`merge-forward-preflight/spec.md:106-113`).
  - `xtask/src/main.rs:105-...` (`const REQUIRED`) and `:143-155` (the `"tests"` step) — the
    machinery that executes the crate-root fence today.
  - `xtask::narrative::{TREE, HARNESS, IGNORE_ALLOWANCES, HIDDEN_MARKERS}` — HS-P0020's pinned
    constants (`checked-documentation-surface/_design.md:85-107`). This story consumes `TREE`
    and `HARNESS`, adds **nothing** to `IGNORE_ALLOWANCES`, and trips none of `HIDDEN_MARKERS`.
  - HS-P0021's answered-need notation, consumed in its own form (`_design.md:900-903`).
  - `.redkiln/config.yaml:40,55` — `affected_gate` and `integration_scoped`.
- **Design-system primitives consumed**: there is no CSS token layer here and none may be
  invented (`_decomposition.md:112-116`). The primitives are rustdoc's own chrome, plain
  CommonMark, the `docs/README.md:12-24` two-column routing-table shape, the
  `#[cfg(doctest)] mod` + `include_str!` surfacing shape (`xtask/src/constitution.rs:39-50`),
  and clause citation by stable id. Zero CSS, zero JS, zero affordance the base medium does not
  already render (UX-012, IQ-6).
- **Renders surfaces**: `crate-root-encounter` and `opening-encounter` — surface ids from
  `_design.md:45-65`. States rendered by this story: `default`, `refusal-rendered`,
  `hidden-line-audit`, `no-unresolved-brackets`, `narrow-700` on the first; `default`,
  `step-one`, `step-two-cold`, `step-three-cold`, `refusal-rendered`, `narrow-700` on the
  second. The `falsification-drill` state is the slice-mate's. `conceptual-bridge` and
  `worked-example-handoff` are **not** rendered here.
- **Public items**: **none** — this project adds, changes and removes no public Rust API item
  (`_design.md:266-268`). The `## Items` rows this story implements are the doc rows:
  `happenstance` (`kind: doc`, `change: signature-changed`, `clause: ES-25`,
  `_design.md:272-276`), whose rendered surface this story rewrites, and the `tokio`
  dev-dependency row (`:284-288`) which the merge already satisfies. `overview.md` (`:278-282`)
  is `surface-course-subscriptions`'s, not this story's.
- **Conformance rule(s)**: **none added, none amended, and this is not adapter-observable.** No
  port, no store, no fixture and no `suite.rs` rule changes here — naming one would be
  decorative by CLAUDE.md's own test. The rules that already observe the behaviour this
  encounter *demonstrates* are `condition_matches_on_tags`
  (`crates/happenstance-testkit/src/suite.rs:4607`, CF-7's rule) and the
  `condition_after_*` family (`:4531`, `:4551`, `:4572`); this story teaches what they check,
  it does not extend them. What observes this story is the gate: the `"tests"` step for the
  crate-root fence, HS-P0020's compiled-fence and checker steps for the tree page, and `cargo
  xtask spec-trace` for the clause citation.
- **Clause(s)**: **none discharged, none amended** — the initiative is additive and discharges
  no clause (`project.md:155-157`). ES-25 (`[FROZEN]`) is **cited** as the clause the refusal
  satisfies; VT-30 (`[PROVISIONAL]`) is cited only if the page describes the guard shape, and
  then without implying it is frozen (`_design.md:400-405`); CF-7 (`[FROZEN]`) is cited only as
  provenance for why the guard is tag-based. Changing any of the three would take a new ADR,
  not an edit; nothing here comes near one.
- **Advances DoD scenario**: initiative **DoD-3** — *"the opening encounter runs and
  demonstrates a boundary"* (`initiative.md`, DoD 3): a reader following the first-fifteen-
  minutes path start to finish reaches a running program in which an append is refused because
  a consistency boundary held, and the refusal is visible in what they see. This story is the
  one that reaches it. It also makes **DoD-4** reachable (the slice-mate closes it) and feeds
  **DoD-1** — the narrative material building and rendering as part of the gate rather than as
  a separate manual step — because these are the first real pages in the pinned tree.

## PR boundary

```
docs/first-encounter.md
docs/README.md
xtask/src/narrative.rs
crates/happenstance/src/lib.rs
.bklg/docs-that-teach/application-author-path/boundary-refusal-encounter/**
```

`redkiln verify --grain story` reads the first fenced block above and fails on any file changed
outside it. Three of the five entries are wiring the Integration contract names — the page, its
index row, and its harness registration — and touching them to mount this slice is not scope
drift. The fence is deliberately narrower than `docs/**`: a second page appearing under the
pinned tree from this story would be a page nobody decided to write.

**In this PR**

- `docs/first-encounter.md` — one page, three steps, in `_design.md`'s fixed per-step order
  (`:453-467`): heading, two-line step header block, one or two sentences of setup, the fence,
  the output block, the clause citation last. Step 1's header line 1 is replaced by the
  answered-need line and its line 2 is omitted.
- The page's `#[cfg(doctest)] mod` registration in `xtask/src/narrative.rs`, one module for
  this file, and its row in `docs/README.md`'s narrative table.
- The rewrite of `crates/happenstance/src/lib.rs`'s doc: `# Using it today` becomes `## Watch a
  boundary refuse`, its fence becomes an executing `#[tokio::main]` program that prints then
  asserts, the planned-surface roadmap moves **below** the fence, and the four unresolved
  `[happenstance_core]` bracket pairs are resolved to real intra-doc links (`_design.md:415-445`,
  anti-pattern 2). ADR-0006's reasoning at `:20-25` is preserved verbatim
  (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`).
- The answered-need line on each surface, in HS-P0021's notation, above everything.

**Explicitly not in this PR**

- **The falsification drill.** The exact edit, the exact failure, the exact revert, and the
  repository check that fails when the query is removed are `boundary-falsification-drill`'s
  (AC-005). This story leaves the `###` slot at the bottom of step 3 and writes nothing into it.
- **The bridge and the handoff.** `## Where your streams went`, the mapping table, the
  four-step narration, the wrong-side contrast and the `overview.md` extraction belong to
  `invariant-to-appendcondition-bridge` and `surface-course-subscriptions`.
- **The set-wide audits.** The fence inventory, the clause spot-check and the answered-need
  walk are `page-set-assurance`'s two stories; this story satisfies its own per-page half so
  those audits can pass, and does not perform them.
- **Any edit to `_design.md`.** Its sign-off is a human's. Contradictions are recorded and
  routed, never folded in (`project.md:300-302`).
- **Any change to `xtask/src/main.rs`, `spec/SPECIFICATION.md`, `crates/happenstance/Cargo.toml`
  or any crate under `crates/` other than the crate-root doc.** The `tokio` dev-dependency
  arrives with the merge; the compiled-fence step and the checker step are HS-P0020's; no
  clause is amended.
- **Any addition to `docs/README.md:12-24`'s pointer-out table.** DT-10 and every pointer policy
  are HS-P0023's (`_design.md:749-751`). The narrative table row is the index's own registration
  and is not that table.
- **Any entry on `IGNORE_ALLOWANCES`, any `ignore`/`no_run`/`compile_fail` fence, any
  `HIDDEN_MARKERS` token, any diagram, any CSS or JS.**

**Merge DoD one-liner** — a reader who opens `docs/first-encounter.md` at step one and follows
it to step three runs a program whose own printed output carries `ConditionViolated`; the same
program runs on the crate root as the first code a docs.rs visitor meets; every fence on both
surfaces is compiled **and executed** by the gate with none opted out; and no line that
constructs the boundary or observes its refusal is hidden from the render.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The encounter is three independently runnable steps** | One page, three `##` steps, no index page in front of them and no fourth "whole program" artifact. Each step's fence is a complete program that compiles and runs on its own; the steps are cumulative in *teaching*, never in *execution*. New concepts, in order: step 1 `Event`/`Tags`/`Query`/`read`; step 2 `Guard`/`AppendCondition`/`after`; step 3 the boundary and ES-25. | `_design.md:122-140`, `:447-467` |
| **Every step answers a cold arrival** | Two lines immediately under each step heading, before any prose or code: line 1 names what the previous step established, line 2 links to step one in one hop. In-body, never in chrome — rustdoc's sidebar is removed entirely below 700px. Step 1 carries the answered-need line in line 1's place and omits line 2. | `_design.md:142-159`, `:453-459`; `_decomposition.md:261-267` (IQ-3), `:337-339` (UX-005) |
| **The refusal is the program's own output** | Step 3 reads a decision model, lets another writer append first, then attempts the guarded append and **prints** the refusal before asserting it. The destructive test: delete every sentence around the fence and a reader who runs the code still sees the boundary refuse. | `_decomposition.md:293-297` (IQ-7), `:322-324` (UX-001); `project.md:242-246` (AC-004) |
| **The printed line carries the token, inside 68 columns, and cannot lie** | `Display` on `ConditionViolated` renders "append condition violated: the store already contains a matching event…" and never contains the token `ConditionViolated`; the bare `{refused:?}` line is 92 characters and scrolls at every viewport. Anti-pattern 10 requires the token in the printed output and anti-pattern 4 forbids the scrollbar, so the print is reached from inside a matched `Err(AppendError::ConditionViolated(_))` arm — unreachable unless the boundary really refused — and the fence still prints before it asserts, which is the binding shape. | `crates/happenstance-core/src/error.rs:168-182`; `_design.md:997` (F-3), `:552-562`, `:845`, `:864-866`, `:317-352` |
| **The guard is built from the read, not from the append** | `read_decision_model(&store, &query)` returns the events and the last position actually observed; that `Option<SequencePosition>` is what `AppendCondition::new(query).after_opt(last)` expects. The position returned by `append` is **not** a sound `after` — the contract crate says so in its own words. `Guard` is `#[non_exhaustive]` with no constructor, so no fence writes `Guard::new`. | `crates/happenstance-core/src/store.rs:132-139`, `:321-331`; `crates/happenstance-core/src/append.rs:131`, `:143-146`, `:201`; `_design.md:354-364` |
| **Verified call spellings** | `Tags::from_pairs([("course", "c1")])?` — `FromIterator` on `Tags` is over `Tag`, so `Tags::from_iter([(…, …)])` does not compile. `QueryItem::new(["SeatHeld"], tags)?`, `Query::from_items([…])?`, `Event::new(type, data: impl Into<Bytes>)?`, `store.append(&[event], Some(&condition))`. The design delegates remaining spellings to the implementer against the merged tree; these are that delegation discharged against the API as it stands. | `crates/happenstance-core/src/tag.rs:304-306`, `:479`; `crates/happenstance-core/src/query.rs:56`, `:185`; `crates/happenstance-core/src/event.rs:351`; `crates/happenstance-core/src/store.rs:213-217`; `_design.md:366-371` |
| **The taught vocabulary is `use happenstance::{…}`** | Every fence on both surfaces imports from the facade; none imports `happenstance_core`. The whole binding shape resolves through it, `read_decision_model` included, because the facade is a glob re-export. `EventStore` is bound, never `SendEventStore`, and only one of the two names is in scope per fence. | `crates/happenstance/src/lib.rs:75`; `crates/happenstance-core/src/lib.rs:117`; `.kb/decisions/0006-bare-name-to-the-typed-layer.md`; `_decomposition.md:358-361` (UX-011); CLAUDE.md binding constraint 4 |
| **No hidden line carries the boundary** | `#`-prefixed lines are absent from the rendered DOM entirely — no hover, no toggle recovers them. Permitted for a `use` block already shown visibly earlier on the same page, `Ok(())`, and struct-literal filler. Forbidden for any `Query`, `QueryItem`, `Tags`, `Guard`, `AppendCondition`, the append call, the read call, and every assertion. The audit is performed by reading the rendered page and rebuilding the boundary from what is visible. | `_design.md:524`; `_decomposition.md:243-251` (IQ-2.1), `:329-332` (UX-003) |
| **No empty boundary where prose claims a real one** | The guard's query is tagged to the invariant being held, not to the row being written; both the tag join and the `after` are load-bearing. Nowhere in this story's material is an empty tag set correct, so none appears — and the page therefore never needs the "here is why an empty one is right" escape. CF-7 is cited, never restated, if the page motivates why the boundary is tag-based. | `project.md:250-252` (AC-006), `:172-176` (DR-02); `_design.md:816-821`; `spec/SPECIFICATION.md:7266` |
| **Every fence is compiled and executed; none opts out** | No `ignore`, no `no_run`, no `compile_fail` on any fence this story writes, and nothing is added to `IGNORE_ALLOWANCES`. `no_run` is the named failure mode: it passes the compiled tier and silently never reaches the executed one. `compile_fail` is separately rejected for this material — a wrong guard **compiles perfectly well**, which is the entire hazard. | `_design.md:206-208`, `:929-935`; `_decomposition.md:616-625`; `checked-documentation-surface/_design.md:98-100` |
| **The page is mounted, not merely written** | The page is registered as its own `#[cfg(doctest)] mod` in `xtask/src/narrative.rs` (one module per page, so a failure names the page) and carries a row in `docs/README.md`'s narrative table. An unregistered page and a dangling registration are both checker problem states. If the harness is absent when this story is implemented, halt and route to HS-P0020 — do not create a parallel tree or a second renderer. | `checked-documentation-surface/_design.md:95-96`, `:491-499`, `:561-563`, `:306-318`; `xtask/src/constitution.rs:11-18`; `_decomposition.md:118-131` |
| **The crate root meets the reader with the refusal first** | `# Using it today` becomes `## Watch a boundary refuse`: two sentences, then the fence — the first code on the page — then one sentence pointing at step one. The planned-surface roadmap moves below it, the adapter-author redirect stays last, and ADR-0006's status section is preserved with its reasoning intact. The single structural change is that the fence moves up and the roadmap moves down. | `_design.md:415-445`; `crates/happenstance/src/lib.rs:27-71`; `.kb/governance/rewrite-the-referent-never-the-reasoning.md` |
| **The crate-root fence executes** | `#[tokio::main] async fn main() -> Result<(), Box<dyn core::error::Error>>`, replacing today's `# async fn example()` wrapper that is never awaited. It is swept by the existing `"tests"` REQUIRED step, so this half of the story needs no new gate wiring. The `tokio` dev-dependency arrives with the merge. | `crates/happenstance/src/lib.rs:58-67`; `_design.md:299-306`, `:392`; `xtask/src/main.rs:143-155`; `merge-forward-preflight/spec.md:106-113` |
| **Step 3's program exists twice, and both copies run** | `include_str!` resolves against the file tree at compile time and a path escaping the package would not resolve once published, so the crate root cannot include the tree's copy. Both copies are compiled and executed and both assert the same `ConditionViolated`. Consolidating them is a substrate question routed to HS-P0020, not a liberty taken here. | `_design.md:707-726`; `crates/happenstance/src/lib.rs:7-9`; `project.md:300-302` |
| **Composition and density hold as measured** | Fences ≤ **68 columns** (72 is where `overflow-x` engages at 1024×768); ≤ **24 rendered lines** on a step page, with the crate-root encounter exempted at **32** because the program written against the real API is 31 lines with one line eligible for hiding; `##` headings ≤ **22 characters** or the sidebar clips them with an ellipsis; paragraphs ≤ **435 characters**; seven elements per step. When a step overflows, yield in the recorded order — fence comments, then setup sentences, then a `use` line already shown visibly, then split — and never the answered-need line, the step header block, any line constructing the boundary, the append call, the refusal assertion, the output block or the citation. | `_design.md:532-617`, `:995-998`, `:1012-1015` |
| **No affordance is introduced** | No tabs, accordions, disclosure triangles, nav widgets, badges, buttons, CSS or JS, on either surface. The correct answer to "enumerate the interactive affordances this project introduced" is none. Nothing this story writes lands inside a `details` that is not `open`. HS-P0020's `HIDDEN_MARKERS` rejects the markers mechanically inside `docs/`. | `_decomposition.md:287-291` (IQ-6), `:362-365` (UX-012); `_design.md:519`, `:858-860`; `checked-documentation-surface/_design.md:102-106` |
| **The prior model is not named here** | Neither surface uses "aggregate", "your aggregates", "one stream per entity" or "which stream". Where the anchor decision is relied on, the page links to the bridge's `#where-your-streams-went` heading rather than re-arguing it. Anchors are stable, human-readable and unnumbered, so inserting a step breaks no inbound link. | `_design.md:108-118`, `:852-854`; `_decomposition.md:269-276` (IQ-4), `:350-352` (UX-009) |
| **One answered-need, above the first fence** | Exactly one per surface, in HS-P0021's notation, above everything — never in a fold, never a tooltip, never a badge. Its form is consumed as given; a second notation is a defect. The slot HS-P0020 reserves directly under a page's H1 is where it goes on the tree page. | `_decomposition.md:299-303` (IQ-8), `:348-349` (UX-008); `_design.md:520`, `:900-903`; `checked-documentation-surface/_design.md:283-289` |
| **Normative claims are citations** | ES-25 is cited by id as the clause the refusal satisfies, inline, as the last sentence of the step; no sentence containing "MUST" or "MUST NOT" appears that is not a link to a clause id; `cargo xtask spec-trace` resolves every citation this story adds. Ids are stable; lines are not — post-merge lines come from the baseline record. | `spec/SPECIFICATION.md:3695` (ES-25, `[FROZEN]`), `:280`; `_design.md:466`, `:867-868`; `_decomposition.md:344-347` (UX-007); `xtask/src/main.rs:143-155` |
| **Content-level accessibility holds** | One `h1` per page, no skipped heading levels, link text meaningful standing alone, DCB expanded on first use per page, and every fence copy-faithful — no shell prompts inside a Rust fence, no elided `…` in a block claimed to be checked. Nothing animates. No automated check exists for this floor; the anti-pattern list is the reviewer-run substitute and the gap is recorded, not mistaken for coverage. | `_decomposition.md:193-225`, `:366-368` (UX-013); `_design.md:909-914` |

**Interfaces, explicitly.** This story defines and changes **no Rust interface**. It *consumes*
the ones named under "Wires into" and adds two artifacts and one registration: a markdown page,
a rewritten crate-root doc comment, and one `#[cfg(doctest)] mod` line in the harness. The only
"API" it authors is the one the reader copies — which is why the verified call spellings above
are a contract row and not a footnote: a fence a reader cannot paste is a defect that the
compiler will catch on the author's machine and nowhere else.

## Data and migrations

**N/A — no schema, no persistence, no migration.** Nothing here reads or writes a durable
store. The only store constructed anywhere in this story's material is an in-process
`happenstance::MemoryEventStore::new()`, created fresh inside each fence and dropped when the
program ends; it has no schema, no file, no connection and no state that outlives the doctest.
No `Cargo.toml` is edited (the `tokio` dev-dependency arrives with the merge and is not resolved
by downstream consumers), no feature is added or removed, no MSRV floor moves, and no `cfg` or
target is touched.

**Deliberately no fixture and no seam.** The testing brief inverts the usual instruction: do
not introduce one. `happenstance_testkit::fixtures::MemoryFixture` and
`event_store_conformance!` are the conformance suite's own instruments for grading an
*adapter*, and sweeping them over one demonstration scenario would prove the wrong thing at
real CI cost. A mock, stub or hand-rolled fake of `EventStore` is forbidden outright: a stubbed
store returning a canned `AppendError::ConditionViolated` regardless of what was appended would
satisfy "the page compiles and runs" while destroying the one thing the demonstration exists
for — and it is the exact shape of Persona 1's stated fear (`_decomposition.md:578-610`).

The one migration-shaped thing is **content migration** on the crate root: the `# Using it
today` section is replaced and the roadmap is reordered. The test is
`.kb/governance/rewrite-the-referent-never-the-reasoning.md` — does the edit change what the
document *asserts*? ADR-0006's argument for why the bare name sits on the typed layer
(`crates/happenstance/src/lib.rs:20-25`) is reasoning that still stands and is preserved
verbatim; what is replaced is a fence that asserts an empty log and demonstrates nothing. Line
citations elsewhere in the planning corpus that point into the old section are repaired at
closeout's reference reconciliation, not here (`_design.md:736-741`).

## Acceptance criteria

Eight criteria, each stated from the reader's intent rather than as a capability. The persona
throughout is **Persona 1, the application author** — the reader who arrives with an existing
event-sourcing vocabulary and whose stated fear is "a mental model that looks right, compiles,
runs, and is quietly wrong"
(`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:99-106`), on backbone
activity **A2**, *"Show me the thing this library is for, working"* (`_storymap.md:42`). "The
gate" below means `cargo xtask ci --fast` (`.redkiln/config.yaml:55`); "the executed tier" means
the `"tests"` REQUIRED step, `cargo test --locked --workspace --all-features -- --show-output`
(`xtask/src/main.rs:143-155`).

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | GIVEN Persona 1 opens `docs/first-encounter.md` at step one wanting the thing the library is *for* rather than a tour of its types, WHEN they read the three steps in order and run step 3's program unchanged, THEN the program **they ran** prints a line of its own output containing the token `ConditionViolated` — reached from inside a matched `Err(AppendError::ConditionViolated(_))` arm, so the line is unreachable unless the store really refused — and the page's output block directly beneath the fence shows exactly that line. The refusal is therefore something the reader *watched happen*, not something a sentence told them: delete every sentence around the fence and the demonstration survives (IQ-7). | The executed tier — step 3's fence runs under the `"tests"` step via its `xtask/src/narrative.rs` registration, and its `assert!(matches!(refused, Err(AppendError::ConditionViolated(_))))` passes with `-- --show-output` showing the printed line. Tier-5 reviewer confirms the page's output block is byte-equal to that captured stdout (`_decomposition.md:481-484`). |
| AC-002 | GIVEN Persona 1 who has just run `cargo add happenstance` and lands on docs.rs — the page the measured defect is about — WHEN they read `/happenstance/index.html` top to bottom, THEN the first code they meet is `## Watch a boundary refuse`'s fence; it is an executing `#[tokio::main]` program rather than the never-awaited `# async fn example()` wrapper at `crates/happenstance/src/lib.rs:58-67`; its printed refusal sits directly beneath it; the planned-surface roadmap has moved **below** it and the adapter-author redirect is still last; ADR-0006's reasoning at `:20-25` survives verbatim; and the rendered page contains **zero** literal `[happenstance_core]` bracket pairs where four stood before. | The executed tier already sweeps the crate-root doctest (`xtask/src/main.rs:143-155`) — a fence that does not execute cannot pass it. `cargo doc --locked --workspace --all-features --no-deps --document-private-items` with `RUSTDOCFLAGS=-D warnings` (`xtask/src/main.rs:290-301`) for the intra-doc links. Then a render read of `target/doc/happenstance/index.html`: count `[<code>` occurrences (zero) and confirm region order inside `details.top-doc > div.docblock` against `_design.md:415-440`. |
| AC-003 | GIVEN Persona 1 arriving **cold** on `#step-three` from a search result — the arrival DT-4's chosen shape is documented to fail at — WHEN they open that anchor and read only what is on screen, THEN the first element under the heading is the two-line step header block naming what step 2 established and linking to step one in one hop; the fence below it is a *complete program that compiles and runs on its own* rather than a fragment of its predecessors; and the same holds opening `#step-two` cold. Step one carries the answered-need line in line 1's place and omits line 2. The reader can name what they missed and reach it in one click at 1440×900 and at ≤700px, where rustdoc removes the sidebar TOC entirely. | Tiers 2/3 reject the interdependence mechanically: each step's fence is its own compiled-and-executed unit, so a fence relying on a previous step's bindings fails to build. Tier-5 reviewer walk — open `#step-two` and `#step-three` cold in a fresh session at both widths against `_design.md:142-159` and `:453-459`, applying anti-pattern 9 (`_design.md:868-870`). |
| AC-004 | GIVEN Persona 1 whose stated fear is a boundary that looks right and is quietly empty, WHEN they read step 3's fence and then remove **either** the tag join from the guard's `Query` **or** the `after_opt(upto)` from the `AppendCondition`, THEN the scenario stops refusing — both are load-bearing — and nowhere on either surface does prose claim a real consistency boundary over code constructing an empty one. Concretely: the guard's query is tagged to the invariant being held **and the racing append carries the same tags**, because `QueryItem::matches` is `type_ok && tags.contains_all(&self.tags)` (`crates/happenstance-core/src/query.rs:112-115`) — an untagged event does not match a tagged query item, and a scenario built that way would refuse nothing while appearing to. | Mechanical half: the executed tier. AC-001's assertion fails outright if the guard is not load-bearing, because an unmatched query yields `Ok` rather than `ConditionViolated`. Prose half: tier 5 — a reviewer reads every sentence adjacent to a fence on both surfaces and confirms no boundary claim sits over `Tags::empty()` or an omitted `after` (`_decomposition.md:488-490`; the crate's own statement of the defect is `crates/happenstance-core/src/query.rs:126-129`). |
| AC-005 | GIVEN Persona 1 reading the **rendered** page rather than its markdown source — the only artefact they will ever see — WHEN they attempt to rebuild the boundary from what is visible on screen, THEN they can: no `#`-prefixed line on either surface carries a `Query`, `QueryItem`, `Tags`, `Guard`, `AppendCondition`, the append call, the read call, or any assertion. Hiding is used only for a `use` block already shown visibly earlier on the same page, `Ok(())`, and struct-literal filler. This is binary rather than a fold — `#`-prefixed lines are absent from the rendered DOM entirely, with no hover, focus or toggle that recovers them (`_design.md:524`). | Tier 5 with a mechanical aid: enumerate every hidden line in the authored spans of `crates/happenstance/src/lib.rs` and `docs/first-encounter.md` (`rg -n '^\s*(///\|//!)?\s*# '`) and classify each against the permitted list; then run IQ-2.1's own destructive test (`_decomposition.md:243-251`) — read `target/doc/happenstance/index.html` and the rendered step page and write out the boundary from the visible lines alone. |
| AC-006 | GIVEN Persona 1 will only ever meet this page if the repository keeps it alive, and a maintainer must not be able to break it silently, WHEN the gate runs on this branch, THEN `docs/first-encounter.md` is registered as its own `#[cfg(doctest)] mod` in `xtask/src/narrative.rs` (one module per page, so a failure names the page) and carries its row in `docs/README.md`'s narrative table — neither *unregistered* nor a *dangling registration* — and **every** fence this story authored on both surfaces is compiled *and executed*, with no `ignore`, no `no_run`, no `compile_fail`, and nothing added to `IGNORE_ALLOWANCES`. A page in `docs/` the harness does not name is the documentation equivalent of built-but-unmounted. | Tier 2 (HS-P0020's compiled-fence REQUIRED step, consumed) plus tier 3 (the `"tests"` step). Mechanically: `rg -n 'first-encounter' xtask/src/narrative.rs docs/README.md` returns both wirings; a search for `ignore`/`no_run`/`compile_fail` fence attributes across `docs/first-encounter.md` and the authored crate-root span returns nothing; `git diff` shows no addition to `IGNORE_ALLOWANCES`. Then `cargo xtask ci --fast` green (`.redkiln/config.yaml:55`). |
| AC-007 | GIVEN Persona 1 reading on a 1024×768 laptop and on a phone, whose attention is the scarce resource this project spends, WHEN they render either surface, THEN the composition the signed-off design fixed is what they meet: no fence exceeds **68 columns** (72 is where `overflow-x` engages on the 696px fence at 1024×768); no step fence exceeds **24 rendered lines** and the crate-root fence **32**; no `##` heading exceeds **22 characters** (the 200px sidebar TOC clips with an ellipsis — it never wraps); no paragraph exceeds **435 characters**; each step carries exactly the seven budgeted elements; exactly **one** answered-need line sits above everything and above the first fence on each surface, in HS-P0021's notation and no second notation; the pages introduce **zero** interactive affordances — no tab strip, accordion, disclosure triangle, nav widget, badge, button, CSS, JS or diagram, and nothing lands inside a `details` that is not `open`; and neither surface contains the words "aggregate", "your aggregates", "one stream per entity" or "which stream". | Tier 5 against `_design.md:532-617` and `:831-882`, every check performable on a screenshot: widest-line and rendered-line counts read off `target/doc/happenstance/index.html` and the rendered step page at 1024×768; heading lengths counted; a case-insensitive search for the four forbidden phrases over both authored spans returning nothing; IQ-6's affordance enumeration whose correct answer is none (`_decomposition.md:287-291`); and an answered-need count of exactly one per surface, above the first fence. |
| AC-008 | GIVEN Persona 1 needs to know that what they just watched is the library's *promise* and not this page's opinion, WHEN they reach the last sentence of step 3 and of the crate-root section, THEN the normative weight is a citation to **ES-25** by its stable clause id — the clause requiring the store to reject the append if and only if it holds a matching event strictly beyond the guard's `after`, and to report the rejection as `AppendError::ConditionViolated` — carried as an ordinary inline link in last position, never a tooltip or popover; no sentence containing "MUST" or "MUST NOT" appears on either surface that is not a link to a clause id; and the citation is provenance the reader may skip, not required reading — the encounter completes without opening `spec/SPECIFICATION.md` or the crate source. | Tier 1: `cargo xtask spec-trace` (REQUIRED, `xtask/src/main.rs:315-327`; also inside `reachability_static`, `.redkiln/config.yaml:48`) resolves every citation this story adds and fails loudly on an id that does not. Tier 5: every "MUST" occurrence in the authored spans confirmed to be a clause link (anti-pattern 11, `_design.md:865-866`), plus IQ-1's falsification — strike every off-page link and confirm the encounter still completes (`_decomposition.md:234-239`). |

**Coverage of the traced project ACs.** **AC-004** (`project.md:242-246`, initiative DoD-3 — the
refusal reaches the reader in the program's own output) is discharged by AC-001 on the tree page
and AC-002 on the crate root, with AC-005 supplying the half a source-level assertion cannot: that
the boundary is *visible*, not merely present. **AC-006** (`project.md:250-252` — no empty boundary
where prose claims a real one) is discharged by AC-004 here, whose mechanical half rides the same
assertion AC-001 does, plus AC-005's rendered-page reconstruction. **AC-002** (`project.md:229-231`
— DT-4's disclosure shape and its documented failure mode) is *owned* by `tension-resolutions`,
which recorded the resolution; this story is where the resolution is **realized**, and AC-003 is
the criterion that proves the mitigation works on the arrival it was designed for (`_storymap.md`,
Coverage table, AC-002 row). AC-006 and AC-008 are not traced project ACs — they are what makes the
other six observable to the *repository* rather than to a reader alone, and AC-008 discharges this
story's own share of project AC-011, whose set-wide audit belongs to
`fence-inventory-and-clause-audit`.

## Interaction quality

This story renders two surfaces — `crate-root-encounter` and `opening-encounter` — so both
families of RFC §6.7/D6 invariants apply in full, and the composition family is taken from the
project's **signed-off** `_design.md` (binding, `:10-12`; sign-off `:1039`). Every invariant that
applies is carried by an `AC-###` **row in the table above**. This section only says which row
carries which invariant and how it is checked. Nothing here is an additional obligation: a bullet
here with no AC id would get no ledger row, would never be gated, and would never be tested.

**STATE invariants.**

- **In place, not a context jump — AC-003 and AC-008.** The reader completes the encounter without
  leaving it. AC-003 keeps a mid-sequence arrival *on the step they landed on*: the header block
  answers "what did I miss" in body text rather than sending them to an index page, of which there
  is deliberately none (`_design.md:441-445`). AC-008 keeps the normative weight as provenance
  rather than a required detour into `spec/SPECIFICATION.md` or the crate source (IQ-1's
  falsification, `_decomposition.md:234-239`).
- **Non-occlusion — AC-005, reinforced by AC-007.** The strongest form here is not a fold but
  rustdoc's `#` prefix, which is *binary*: the line is gone from the DOM with nothing that recovers
  it. AC-005 is the check, and its test is reconstructive rather than declarative — rebuild the
  boundary from what renders. AC-007 covers the second form: nothing this story authors lands
  inside a `details` that is not `open`, and the output block — which *is* the claim (IQ-7) — is
  never hidden.
- **Preserved place, focus and selection — AC-003.** Every step and named concept has a stable,
  human-readable, **unnumbered** anchor, so inserting a step later breaks no inbound link (IQ-4,
  `_decomposition.md:269-276`). Anchors land on the heading, not the page top — verified at ≤700px
  too, where rustdoc's `scroll-margin-top: 45px` keeps the target clear of the topbar
  (`_design.md:511`). *Focus and scroll restoration have no analogue here, and that is a
  consequence rather than an omission*: this story introduces no control that could reset either,
  which is AC-007's clause.
- **Reversibility — AC-002 here; the reader-facing half is the slice-mate's.** IQ-5's reversibility
  in the reader's hands (remove the boundary → watch a check fail → revert → watch it pass) is
  `boundary-falsification-drill`'s AC-005, and this story leaves the `###` slot at the bottom of
  step 3 empty for it (`_design.md:468-471`). What *is* reversible here is the content migration:
  AC-002 requires ADR-0006's reasoning at `crates/happenstance/src/lib.rs:20-25` to survive the
  rewrite verbatim, so the edit changes what the document *demonstrates* without changing what it
  *asserts* (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`).
- **Keyboard reachability — AC-007.** By construction rather than by testing: every affordance on
  these pages is one rustdoc already renders and already makes keyboard-reachable, and AC-007's
  enumeration clause is what keeps that true. IQ-6's test is "enumerate the interactive affordances
  this project introduced"; the correct answer is none, and AC-007 fails if it is not.

**COMPOSITION invariants** (from `_design.md` — `## Composition` `:408-505`, `## Transience policy`
`:507-530`, `## Density budget` `:532-617`, `## Hierarchy` `:621-676`, `## Anti-patterns`
`:831-882`).

- **Presentation exists at all — AC-002 and AC-006.** This is the invariant an unstyled render
  passes and a real reader fails, and in this medium it has a precise form: *the page must render,
  and the fence must render as a fence*. A markdown file that is correct and never included by the
  harness renders nowhere — AC-006, where registration **is** the mount. A crate-root doc edit that
  is correct and never built renders nowhere — AC-002, which requires the doc build to run and the
  composed region `#main-content details.top-doc > div.docblock` (`_design.md:46-49`, read out of
  the real `index.html`) to be present. Neither a green `cargo test` nor a passing text assertion is
  evidence of this; only the render is.
- **Composition and placement — AC-002 (crate root) and AC-003 (step pages).** AC-002 fixes the
  crate root's region order: summary, answered-need line, `## Status`, `## Watch a boundary refuse`
  with the fence as the first code on the page, the roadmap *below* it, the redirect last
  (`_design.md:415-440`). The single structural change is that the fence moves up and the roadmap
  moves down — a reader landing on docs.rs meets the refusal before they meet the plan. AC-003 fixes
  the per-step order: heading, header block, ≤2 setup sentences, fence, output block, citation
  (`_design.md:453-467`).
- **Transience — AC-005 and AC-007.** The design's transience table classifies every control the
  reader meets, rustdoc's own included. What this story must hold: the answered-need line and the
  step header block are **persistent, in-body** — the header block specifically because chrome
  disappears below 700px and a mitigation that vanishes on a phone is not one; every fence and every
  output block is **persistent**, never behind a control; hidden doctest lines are **not a
  transience class at all but binary**, which is AC-005. Nothing is *opened-on-demand*, because
  nothing here may be.
- **Density budget, with its real numbers — AC-007.** 68 columns; 24 rendered lines per step fence
  and 32 on the crate-root fence (the exemption granted at the design gate because the program
  written against the real API is 31 rendered lines with exactly one eligible for hiding, finding
  F-4); 22-character `##` headings; 435-character paragraphs; seven elements per step. When a step
  overflows, the yield order is fence comments → setup sentences → a `use` line already shown
  visibly → split, and **what never yields** is the answered-need line, the step header block, any
  line constructing the boundary, the append call, the refusal assertion, the output block and the
  citation (`_design.md:601-617`). A fence that cannot fit with those intact has a scenario that is
  too large, and the scenario shrinks — never the boundary.
- **Hierarchy — AC-002 and AC-001.** Only three channels exist here (position in reading order,
  heading level, form), because this project may not introduce a single style rule. On the crate
  root the **primary** element is the refusal, carried by position (first `##` after the status
  section, first code on the page) and reinforced by form (the only fence, output directly beneath);
  the roadmap and the adapter redirect are **recessive**, carried by position at the end
  (`_design.md:625-640`). AC-001 carries the same hierarchy on step 3, where the payoff deliberately
  sits in the step most likely to be landed on cold.
- **Named anti-patterns.** All fifteen are assigned, none left to judgement: **1** (a block labelled
  not-compiled or exempt) → AC-006; **2** (a literal `[bracket]` pair — four exist today) → AC-002;
  **3** (a collapsed disclosure triangle) → AC-007; **4** (a horizontal scrollbar at 1024×768) →
  AC-007; **5** (a sidebar TOC entry clipped with an ellipsis) → AC-007; **6** (the words
  "aggregate" / "one stream per entity" / "which stream") → AC-007; **8** (any control outside
  rustdoc's chrome) → AC-007; **9** (a step whose first element is not the header block) → AC-003;
  **10** (prose claiming a refusal the output does not contain) → AC-001 and AC-005; **11** (a
  "MUST" sentence that is not a clause link) → AC-008; **12** (two answered-needs, or one below the
  first fence) → AC-007; **13** (`use happenstance_core::` in a fence) → AC-007's vocabulary clause;
  **14** (a diagram, chart or image) → AC-007; **15** (a step whose program prints nothing) →
  AC-001 for step 3 and AC-003 for steps 1 and 2, whose fences must also print. **7** (the
  wrong-model block last, or above the correct one) **cannot fire here** — the wrong-side contrast
  is the bridge page's and this story authors none. That is the PR boundary holding, not a check
  passing, and it is stated so a later reviewer does not read silence as coverage.

## Error conditions

| id | condition | required response |
| --- | --- | --- |
| EC-001 | `xtask/src/narrative.rs` does not exist when this story is implemented — it does not exist today; `xtask/src/` holds ten modules and none is it. | **Halt and route to HS-P0020.** Do not create a parallel tree, a second renderer, or a page outside the pinned one; that is out of contract (`_decomposition.md:118-131`). The crate-root half (AC-002) mounts itself and may proceed; AC-001, AC-003 and AC-006 wait. Record the halt rather than degrading the criterion. |
| EC-002 | HS-P0020's compiled-fence step turns out to **type-check without executing** doctests. | AC-001 is not satisfied by compilation. Per the testing brief (`_decomposition.md:641-650`) and `project.md:334-339`, this project then adds its own `#[tokio::test]` against a real `MemoryEventStore` — in the manner of `examples/course-subscriptions/src/main.rs`'s commit helper — wired into the `"tests"` REQUIRED step, rather than accepting a weaker criterion. Route the substrate gap to HS-P0020; do **not** route AC-001 away. |
| EC-003 | HS-P0021's answered-need notation does not exist when this story is implemented. | Record the gap and route to HS-P0021 (`_design.md:900-903`). Do **not** invent a second notation — DR-14's named defect. AC-007's answered-need clause is then blocked on HS-P0021, not satisfied by a placeholder. The *position* (above everything) and *cardinality* (exactly one) are this design's and stand regardless. |
| EC-004 | The step-3 program as written does **not** refuse: the append returns `Ok`. | This is the AC-004 defect caught by its own assertion, and the first hypothesis is the one this spec measured — the racing append carries no tags, so `QueryItem::matches` never matches it (`crates/happenstance-core/src/query.rs:112-115`). Fix the scenario so the tag join and the `after` are both load-bearing. Do **not** widen the query to `Query::All` and do **not** drop `after_opt` to make the assertion pass; either produces a page that refuses for the wrong reason, which is the exact defect AC-004 exists to catch. |
| EC-005 | The refusal is raised but the printed line breaches the 68-column budget, or does not contain the token `ConditionViolated`. | Both are already known and both have one answer: `Display` on `ConditionViolated` never contains the token (`crates/happenstance-core/src/error.rs:168-182`), and the bare `{refused:?}` line is 92 characters (`_design.md:997`). Print from inside a matched `Err(AppendError::ConditionViolated(_))` arm. Do not relax the budget (anti-pattern 4) and do not narrate the token in prose (anti-pattern 10). |
| EC-006 | A fence cannot be brought under 24 rendered lines (step) or 32 (crate root) with every non-yielding element intact. | Apply the recorded yield order (`_design.md:601-617`) and stop as soon as it fits. If it still does not fit, the *scenario* is too large — shrink the scenario. Do **not** hide a boundary line with `#`, do not delete the output block, and do not quietly relax a number: the design gate rejected this project once for exactly that (`_design.md:1041-1044`). If the merged API genuinely cannot fit, record it as a design gap and route it to the sign-off owner. |
| EC-007 | A clause id cited by this story does not resolve under `cargo xtask spec-trace`. | Escalate; do not repoint. Ids are stable and never renumbered (`spec/SPECIFICATION.md:280`), so a missing id is a semantic change rather than line drift. Take the id and its merged line from `merge-forward-preflight`'s `_baseline.md § Anchors` — that record, not this file, is the authority for every line after the merge — and route a genuine gap to HS-P0023. |
| EC-008 | Implementation contradicts a **binding** statement in `_design.md` — a composition, transience, density or anti-pattern rule the merged tree makes impossible. | Record the contradiction in this story's own directory and route it per project DoD item 9 (`project.md:300-302`): substrate to HS-P0020, pointer and reach to HS-P0023, comprehension to HS-P0024, incidental bugs to the `support` initiative (`.redkiln/config.yaml:5`). **Never edit `_design.md`** — its sign-off is a human's, and folding a contradiction in silently is re-litigating it. |
| EC-009 | The two copies of step 3's program — crate root and tree page — drift apart. | Expected and bounded, not a bug to design around: `include_str!` cannot cross the published package boundary (`crates/happenstance/src/lib.rs:7-9`), so both copies exist and **both are executed** (`_design.md:707-726`). Drift can be silent about *identity*; it cannot be silent about *correctness*, because each copy asserts the same `ConditionViolated`. Consolidation through a shared `crates/happenstance/examples/` file is a substrate question for HS-P0020, not a liberty taken here. |
| EC-010 | `redkiln verify --grain story` reports a file changed outside the PR boundary fence. | A real scope violation, not a fence to widen. The five entries are the two surfaces, their two wirings and this story's own item directory; a sixth file — especially a second page under `docs/` — is a page nobody decided to write. Revert it, or route it as its own story. |

## Non-functional

| id | requirement | why |
| --- | --- | --- |
| NF-001 | **Zero opt-outs, permanently.** No fence this story authors carries `ignore`, `no_run` or `compile_fail`, and `IGNORE_ALLOWANCES` gains no entry. | `no_run` passes the compiled tier and silently never reaches the executed one — "compiles forever while quietly ceasing to demonstrate its own claim", the initiative's first-ranked risk (`_decomposition.md:616-625`, `project.md:335`). `compile_fail` is separately wrong for this material: a wrong guard **compiles perfectly well**, which is the entire hazard (`_design.md:929-935`). |
| NF-002 | **The page is self-sufficient cold.** A reader arriving at any step anchor completes that step without opening `spec/SPECIFICATION.md`, the crate source, or another page. | IQ-1's falsification and DR-08. Progressive disclosure only pays if the disclosed layer is complete at the point of disclosure; a citation the reader *must* follow is a context jump wearing a link. |
| NF-003 | **No new dependency, no `Cargo.toml` edit, no feature change, no MSRV movement, no `cfg` or target change.** | The `tokio` dev-dependency arrives with the merge and is dispositioned `unchanged` (`merge-forward-preflight/spec.md:106-113`); dev-dependencies are not resolved by downstream consumers (`_design.md:761`). A documentation story must not become a semver-relevant or MSRV-relevant event (CLAUDE.md constraint 5, ADR-0029). |
| NF-004 | **Zero CSS, zero JS, zero bespoke affordance, zero image.** | UX-012 / IQ-6, and anti-patterns 8 and 14. Adding one would require HS-P0020's DT-7 resolved first (`_decomposition.md:287-291`). The correct count of introduced affordances is none, and that is a criterion (AC-007), not an aspiration. |
| NF-005 | **The taught vocabulary is `use happenstance::{…}` on every fence**, with `EventStore` bound and `SendEventStore` never in scope alongside it. | ADR-0006 gave the bare name to the typed layer for exactly this reader (`.kb/decisions/0006-bare-name-to-the-typed-layer.md`), and the whole binding shape — `read_decision_model` included — resolves through the facade's glob re-export (`crates/happenstance/src/lib.rs:75`). CLAUDE.md constraint 4 forbids both trait names in one module: a fence that breaks it makes method calls ambiguous on the *reader's* machine, not the author's. |
| NF-006 | **The gate cost of this story is bounded to steps that already run.** No new CI step, no new job, no new tool probe. | The crate-root fence rides the existing `"tests"` step; the tree page rides HS-P0020's step; the citation rides `spec-trace`. The project is `terminal: false`, so `cargo xtask ci --fast` is the applicable bar (`.redkiln/config.yaml:55`) and the whole-gate re-observation is HS-P0025's at closeout. |
| NF-007 | **The content-level accessibility floor holds, and the absence of a checker for it is recorded rather than papered over.** One `h1` per page, no skipped heading levels, link text meaningful standing alone, DCB expanded on first use per page, fences copy-faithful (no shell prompts inside a Rust fence, no elided `…` in a block claimed to be checked), nothing animates. | No automated check exists for any of it (`_design.md:915-921`, gap 5); anti-patterns 3, 5, 9 and 12 are the reviewer-run substitute. Naming the gap is what keeps it from being mistaken for coverage — a lint's natural home is HS-P0021's AC-008. |

## Implementation notes (non-prescriptive)

- **Order that keeps each failure attributable.** Write the crate-root fence *first* and run
  `cargo test -p happenstance --doc` on it alone. It is the half that mounts itself, it is the
  binding shape both copies share, and it is where the API will disagree with the plan. Only then
  author the three steps, register the page, and add the index row. The other order means
  diagnosing an API surprise and a missing harness at the same time.
- **Start from `_design.md:312-386`'s binding fence, then discharge its known holes.** The design
  states the shape is binding and delegates the remaining call spellings to the implementer against
  the merged tree (`:366-371`). Three deltas this spec measured: `Tags::from_iter([(…)])` →
  `Tags::from_pairs([("course", "c1")])?` (`crates/happenstance-core/src/tag.rs:304-306`, `:479`);
  `read_decision_model` is missing from the sketch's `use` block though the body calls it; and
  `Guard` is *in* that block but never named in the body — an unused import, and one line of the
  68-column budget bought back by dropping it.
- **The racing append needs tags.** `Event::new("SeatHeld", …)?` alone carries none, and
  `QueryItem::matches` is `type_ok && tags.contains_all(&self.tags)`
  (`crates/happenstance-core/src/query.rs:112-115`), so an untagged event never matches the guard's
  tagged query, the condition is never violated, and the assertion fails.
  `Event::new(…)?.with_tags(tags)` (`crates/happenstance-core/src/event.rs:372`) is what makes the
  tag join load-bearing — AC-004 stated as code rather than as prose.
- **Error type at the boundary of `main`.** `read_decision_model` returns `Result<_, S::Error>` and
  `MemoryEventStore::Error` is `MemoryStoreError` (`crates/happenstance-core/src/memory.rs:294`),
  while `append` returns `AppendError<Self::Error>`. `Box<dyn core::error::Error>` absorbs both if
  each is `Error + 'static`; if it does not compile cleanly inside the budget, prefer matching
  explicitly over widening the signature — the `match` is what AC-001 needs anyway.
- **The `println!` is the payoff, and where it sits is the design decision.** Print from inside the
  matched `Err(AppendError::ConditionViolated(_))` arm so the line is unreachable unless the store
  really refused, then assert. Printing before asserting is the binding shape
  (`_design.md:312-352`), because an assertion is invisible when it passes.
- **Capture the output block; never compose it.** Run the program and take stdout verbatim into the
  fenced text block beneath the fence. A hand-written output block is prose claiming a refusal
  (anti-pattern 10) with extra steps.
- **Measure the budget on the render, not on the source.** `cargo doc -p happenstance --no-deps` and
  read `target/doc/happenstance/index.html` — `_design.md:1025-1027` names it as the cheapest way to
  retake every composition number, and the 68 / 24 / 32 / 22 / 435 figures were all derived there.
  Source lines and rendered lines are not the same count once hidden lines are removed.
- **Heading length is a real constraint, not a style note.** The budget is on the heading *text*:
  `Watch a boundary refuse` is 23 characters, one over. Check every `##` against 22 before writing
  the fence under it; eleven of the sixteen headings the design names already exceed it, which is
  why the retitles in `## Composition` are structural rather than cosmetic. Record the resolution
  taken; do not silently keep an over-budget heading.
- **Take every clause line from `merge-forward-preflight`'s `_baseline.md § Anchors`.** Every line
  number in this spec's front half was read on the *pre-merge* tree. Ids are stable; lines are not.
- **Coordinate with the slice-mate before touching step 3's tail.** `boundary-falsification-drill`
  authors the `###` under step 3 (`_design.md:468-471`); leave the slot and do not pre-empt its
  wording. The two stories are implemented in one context and mounted as one surface.
- **If something is wrong with the design rather than with the implementation, write it down and
  route it.** That is the sanctioned move (`project.md:300-302`), and it is faster than arguing with
  a human sign-off in a commit message.

## Tests and CI (merge gate)

Tiers are the testing brief's five (`_decomposition.md:462-476`), unchanged; this story adds no
tier and no command of its own.

| tier | command / path | proves |
| --- | --- | --- |
| **1 — Structural** | `cargo xtask spec-trace` (REQUIRED, `xtask/src/main.rs:315-327`; also inside `reachability_static`, `.redkiln/config.yaml:48`) | Every clause id this story cites resolves — the mechanical half of AC-008, and the check that fails loudly rather than silently on a renumbered or deleted clause. |
| **1 — Structural** | `cargo doc --locked --workspace --all-features --no-deps --document-private-items` with `RUSTDOCFLAGS=-D warnings` (REQUIRED, `xtask/src/main.rs:290-301`) | The crate root builds and every intra-doc link resolves — AC-002's four-brackets-to-zero clause, and the render without which AC-002's composition claims are unobservable. |
| **2 — Compiled fence** | HS-P0020's narrative REQUIRED step, reached through this page's `#[cfg(doctest)] mod` in `xtask/src/narrative.rs` | Every fence on `docs/first-encounter.md` type-checks against the real workspace crates — AC-006's mount, and the step that makes an *unregistered* page a checker problem state rather than an invisible one. |
| **3 — Executed** | `cargo test --locked --workspace --all-features -- --show-output` (REQUIRED step `"tests"`, `xtask/src/main.rs:143-155`) | The boundary actually refuses on both copies: AC-001 (step 3's printed `ConditionViolated`), AC-002 (the crate-root fence executes rather than merely type-checking), AC-004's mechanical half (an unmatched query yields `Ok` and fails the assertion), AC-003 (each step's fence runs standalone). The step to run locally while iterating. |
| **3 — Executed, fallback** | A `#[tokio::test]` against a real `MemoryEventStore` in the manner of `examples/course-subscriptions/src/main.rs`'s commit helper, swept by the same `"tests"` step | Only if EC-002 fires — HS-P0020's step type-checks without executing. Named here so the response to that discovery is a *stronger* check, not a weaker criterion (`_decomposition.md:641-650`). |
| **4 — Human-observed falsification** | `boundary-falsification-drill` (HS-S0186), project DoD item 3 | **Not this story's**, and recorded rather than omitted. Tier 4 is the slice-mate's AC-005 and is what makes this story's boundary load-bearing to the repository. The slice is not done until it has run in both directions. |
| **5 — Reviewer walk** | A read of the rendered pages against `_design.md` `## Composition`, `## Transience policy`, `## Density budget`, `## Hierarchy` and `## Anti-patterns`, at 1440×900, 1024×768 and ≤700px | AC-003 (cold arrival on `#step-two` / `#step-three`), AC-005 (rebuild the boundary from visible lines), AC-007 (every density number, the affordance enumeration, the vocabulary check), AC-004's prose half, AC-008's no-restated-clause half. `design.capture` is deliberately absent from `.redkiln/config.yaml`, so this recorded walk is the only record these checks will ever have. |
| **Story grain** | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | The checkpoint bar: what this diff could break. It runs the file-reading lints and `spec-trace` unconditionally, which matters here precisely because a documentation story maps to few packages. |
| **Integration grain** | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | The bar this non-terminal project is held to (project DoD item 5). Green on the slice is what AC-006 rides. |
| **Ledger / boundary** | `redkiln verify --grain story` | Every AC in `_ledger.md` satisfied with cited evidence, and no file changed outside the PR boundary fence (EC-010). |

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | mitigation in this PR |
| --- | --- | --- |
| **The harness does not exist yet.** `xtask/src/narrative.rs` is not in the tree, and three of this story's eight ACs ride it. | High / High | EC-001: halt and route to HS-P0020, land the crate-root half, build no parallel tree. The story is authored so the two halves fail independently rather than together. |
| **The compiled-fence step type-checks without executing**, and AC-001 is satisfied by compilation alone — the initiative's first-ranked risk wearing the substrate's clothes. | Medium / High | EC-002 and the tier-3 fallback row: this project adds its own `#[tokio::test]` rather than accepting the weaker criterion. `project.md:334-339` is explicit that this proof does not route away. |
| **The scenario refuses for the wrong reason** — a query broad enough that anything violates it, or an `after` doing all the work. Both would pass AC-001's assertion. | Medium / High | AC-004 states both the tag join and the `after` as load-bearing and names the measured trap (an untagged racing append never matches). The falsification that settles it mechanically is the slice-mate's AC-005, which is why the two are one slice. |
| **The two copies of step 3 drift.** | Medium / Low | EC-009. Bounded by construction: both execute, both assert the same error. Identity drift is visible to a reviewer; correctness drift is survivable by neither copy. |
| **The design's fence sketch does not compile as written** — three deltas already measured, and the merged tree may hold more. | High / Low | The design delegates remaining spellings to the implementer against the merged tree (`_design.md:366-371`), so these are corrections inside a delegation rather than design changes. Implementation notes carry all three. |
| **The 68 / 24 / 32 budget collides with the program the design also specifies.** | Medium / Medium | EC-006 and the recorded yield order. The crate-root exemption at 32 exists because this collision already happened once at the design gate and was resolved by recording the reason rather than relaxing the number quietly. |
| **Line citations in this spec rot on merge** — every one was read pre-merge. | High / Low | Ids over lines throughout; `merge-forward-preflight`'s `_baseline.md § Anchors` is the post-merge authority, and this story's `depends_on` makes that record exist before authoring starts. |
| **Coupling to `tension-resolutions`' sign-off.** `_design.md` is binding and this story implements rather than re-decides it. | Low / High | The PR boundary fence excludes `_design.md`. EC-008 routes contradictions instead of folding them in. |
| **Coupling to HS-P0021's notation.** AC-007's answered-need clause consumes a form that does not exist yet. | Medium / Medium | EC-003: position and cardinality are this design's and stand; the form is consumed as given, and inventing a second notation is DR-14's named defect. |
| **File overlap with HS-P0016.** Both touch `crates/happenstance/src/lib.rs`. | Medium / Medium | The seam is purpose, not paragraph (`project.md:349`): this story touches the encounter section, the roadmap's position and the unresolved brackets, and touches no landing copy, status vocabulary or registry metadata. |

## Dependencies

**Blocks on** — both are `preflight-and-anchor` stories in this project, and both must have landed
before authoring starts:

- **`merge-forward-preflight`** (HS-S0183) — the merge forward from
  `initiative/from-contract-to-published-library`, without which every page here is written against
  this worktree's stale `crates/happenstance/src/lib.rs` rather than the 237-line merged file it
  actually rewrites (DR-13, project AC-014). Its `_baseline.md § Anchors` is also the post-merge
  authority for every clause line this spec cites, and its `§ Dispositions` is where the `tokio`
  row's `added` → `unchanged` correction lives (`merge-forward-preflight/spec.md:106-113`).
- **`tension-resolutions`** (HS-S0184) — `_design.md` itself, signed off (`:1039`). DT-4's staged
  three-step shape, DT-1's forbidden vocabulary, DT-6's zero-exemption rule, the composition, the
  transience policy, the density budget and the fifteen anti-patterns are all consumed here as
  settled. Authoring before it lands means authoring against an unsettled anchor.

**Unlocks** — every remaining story in this project except the two above:

- **`boundary-falsification-drill`** (slice-mate, HS-S0186) — writes the `###` this story leaves
  empty at the bottom of step 3 and makes the boundary load-bearing to the repository. Mounted with
  this story as one integrated surface; the slice is not done until both are in.
- **`invariant-to-appendcondition-bridge`** — depends on this story for the vocabulary and for the
  refusal the bridge carries a reader *toward*, and owns the one place the prior model is named.
- **`surface-course-subscriptions`** — reached transitively, through the bridge.
- **`fence-inventory-and-clause-audit`** and **`answered-need-and-anchor-review`** — both name this
  story explicitly in their `depends_on`; their set-wide audits are unprovable until this story's
  per-page half exists. This story satisfies its own half (AC-006, AC-007, AC-008) so those audits
  can pass; it does not perform them.

Outside the DAG: **HS-P0020 `checked-documentation-surface`** must have landed the pinned tree, the
narrative harness and the compiled-fence step, and **HS-P0021 `page-need-discipline`** must have
landed the answered-need notation. Neither is a story-level `depends_on` edge — they are
project-level dependencies (`project.md:317-325`) whose halt-and-route responses are EC-001 and
EC-003.

## Anchors (progressive disclosure)

Everything load-bearing is above. This table is the deferred layer: each row says *why* the
artifact is load-bearing and *when* to open it, so retrieval is scheduled rather than
discretionary. Link; do not paste.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/application-author-path/_design.md` | The signed-off, **binding** design. `## Composition` `:408-505` fixes region order on both surfaces; `## Transience policy` `:507-530` classifies every control including the binary hidden-line rule; `## Density budget` `:532-617` carries the 68 / 24 / 32 / 22 / 435 numbers and the yield order; `## Anti-patterns` `:831-882` is the fifteen-item checklist a reviewer runs. | Before the first line of either surface, and again before each tier-5 walk. Re-read `:601-617` the moment a fence overflows. | AC-007 |
| `.bklg/docs-that-teach/application-author-path/_design.md` (`## Signatures`, `:312-386`) | The binding fence shape, written out with hidden lines shown so the UX-003 audit is performable against the design itself — plus the design-gate correction (finding F-5) that killed `store.head(&seats)` and `Guard::new`. Everything the implementer types starts here. | First, before writing any code; then again with the three measured deltas from Implementation notes in hand. | AC-002 |
| `.bklg/docs-that-teach/application-author-path/_decomposition.md` | Two briefs in one file: the UX brief `:11-444` (IQ-1…IQ-9, each carrying its own falsification, plus UX-001…UX-013) and the testing brief `:448-650` (the five tiers, the named wrong page each rejects, and the fixtures instruction that inverts the usual one). | Open the IQ block before the tier-5 walk; open the testing brief before deciding what proves an AC — especially `:641-650` if EC-002 fires. | AC-005 |
| `.bklg/docs-that-teach/checked-documentation-surface/_design.md` | HS-P0020's substrate, consumed and never rebuilt: `TREE` / `HARNESS` / `IGNORE_ALLOWANCES` / `HIDDEN_MARKERS` at `:85-107`, "the markdown is the render" at `:165-198`, the one-module-per-page registration at `:491-499`, the narrative index table at `:306-318`, and the *unregistered* / *dangling registration* problem states at `:561-563`. | Immediately before wiring the mount — and immediately if `xtask/src/narrative.rs` is missing (EC-001). | AC-006 |
| `crates/happenstance-core/src/store.rs` | The append/read contract: `append` at `:213-217`, `read` returning the stream at the top level and not `async` at `:119-123`, and `read_decision_model` at `:321-331`, whose second return value is exactly what `after_opt` expects — stated in the crate's own words at `:132-139`. This is why the guard is built from the read and never from the append's returned position. | While writing the fence body, before reaching for `head()`. | AC-001 |
| `crates/happenstance-core/src/query.rs` | `QueryItem::matches` at `:112-115` is `type_ok && tags.contains_all(&self.tags)` — the reason an untagged racing append is silently accepted. `:126-129` is the crate stating the empty-boundary defect in its own words: a condition on a query with no items is "a conditional append that is silently unconditional, which is a lost update with no diagnostic anywhere." | Before writing the guard and the racing append; again the moment EC-004 fires. | AC-004 |
| `crates/happenstance-core/src/error.rs` | `ConditionViolated` at `:133-184`, including the hand-written `Display` at `:168-182` that never contains the token `ConditionViolated`. The single fact that determines the shape of the `println!`. | Before writing the print line. | AC-001 |
| `crates/happenstance-core/src/append.rs` | `AppendCondition::new(Query)` at `:143-146`, `after_opt` at `:201`, and `Guard` at `:131` — `#[non_exhaustive]` with **no** constructor, which is why no fence writes `Guard::new` and why `Guard` may not belong in the `use` block at all. | While writing the condition, and when finalising the import list against the 68-column budget. | AC-004 |
| `crates/happenstance/src/lib.rs` | The surface being rewritten and the defect being fixed: the never-awaited `# async fn example()` wrapper at `:58-67`, ADR-0006's reasoning to preserve verbatim at `:20-25`, the facade glob at `:75`, and the `include_str!` package-boundary note at `:7-9` that forces two copies of step 3. | Read all four spans before the first edit; re-read `:20-25` before the diff is final. | AC-002 |
| `spec/SPECIFICATION.md` | ES-25 (`[FROZEN]`, `#### ES-25` at `:3695` pre-merge) is the clause the encounter makes concrete; `:280` records that ids are stable and never renumbered; CF-7 at `:7266` (`[FROZEN]`) is the provenance for why the guard is tag-based. Cite by id, never by prose. | When writing the closing citation of step 3 and of the crate-root section — with `merge-forward-preflight`'s `_baseline.md § Anchors` open beside it for post-merge lines. | AC-008 |
| `.bklg/docs-that-teach/application-author-path/merge-forward-preflight/spec.md` | The blocking dependency's own record: `:106-113` dispositions the `tokio` row from `added` to `unchanged`, and the `_baseline.md § Anchors` / `§ Composition baseline` sections it authors are the post-merge authority for every line and every measured composition number this spec quotes pre-merge. | Before trusting any line number in this spec, and before re-measuring anything. | AC-007 |
| `xtask/src/main.rs` | `const REQUIRED` from `:105`, the `"tests"` step at `:143-155` (the machinery that already executes the crate-root doctest), the documentation step at `:290-301`, and `spec-trace` at `:315-327`. This is what "mounted" means for the crate-root half. | When confirming the crate-root half needs no new gate wiring, and when choosing the local command to iterate with. | AC-002 |
| `xtask/src/constitution.rs` | The `#[cfg(doctest)] mod` + `include_str!` surfacing shape already in this repository — `:11-18` for why one module per file, `:39-50` for the shape itself. The harness registration is this pattern, not a new invention. | When writing the `xtask/src/narrative.rs` entry. | AC-006 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The test the crate-root content migration must pass: does the edit change what the document *asserts*? Replacing a fence that demonstrates nothing is a referent change; deleting ADR-0006's argument would be a reasoning change. | Before deleting any sentence from `crates/happenstance/src/lib.rs`. | AC-002 |
| `.kb/decisions/0006-bare-name-to-the-typed-layer.md` | Accepted decision atom. Why the bare name sits on the typed layer, and therefore why `use happenstance::{…}` is the taught vocabulary and `use happenstance_core::` is anti-pattern 13 on every page here. | Once, before the first `use` block; again if a fence seems to need the other crate's name. | AC-007 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | Persona 1's journey and, at `:99-106`, the stated fear every criterion above is answerable to: "a mental model that looks right, compiles, runs, and is quietly wrong." The reason a mock store is forbidden and the reason AC-004 exists at all. | When a criterion starts to feel like ceremony — this is what it is for. | AC-004 |
| `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` | The evidence base behind the design's choices: Carroll's training-wheels result at the root of DT-4, the staged-disclosure failure mode at `:161-166`, the load-bearing-content-behind-a-fold pattern at `:404-410`, the repeat-per-page defect at `:547-566`. | Only if tempted to re-open DT-4 or DT-1 — read it, then don't. | AC-003 |
| `examples/course-subscriptions/src/main.rs` | The repository's own working DCB scenario against a real `MemoryEventStore`, and the shape the tier-3 fallback `#[tokio::test]` would copy if EC-002 fires. Read it as evidence rather than as a template — it is the hazard this initiative was seeded by. | If EC-002 fires; or when a scenario detail (tags, event types, the commit loop) needs a precedent. | AC-001 |
| `docs/README.md` | The pinned tree's index. `:12-24` is the two-column routing-table shape the narrative row must match — and is also the pointer-out table this story must **not** add to, since DT-10 is HS-P0023's. | When adding the index row: to copy the shape, and to see the line this story may not cross. | AC-006 |
| `.bklg/docs-that-teach/application-author-path/project.md` | The project charter: AC-004 at `:242-246`, AC-006 at `:250-252`, AC-002 at `:229-231`, DoD item 9's routing rule at `:300-302`, and the risks table whose first row is this story's central hazard. | When deciding whether something found here is this story's to fix or to route. | AC-001 |
| `.bklg/docs-that-teach/application-author-path/_storymap.md` | Backbone activity A2 at `:42`, this story's row at `:57`, and the "why the slices are cut here" note explaining why the encounter and the drill are one slice rather than two. | When coordinating with `boundary-falsification-drill`, or when tempted to land the page without it. | AC-003 |
| `.bklg/docs-that-teach/application-author-path/_grounding.md` | What the project verified in the tree before planning — the same-tree check every claim in the briefs rests on. | If a cited path or fact in this spec disagrees with the tree in front of you. | AC-005 |

## Clarifications resolved during spec

1. **The AC set is the front half's eight, unchanged.** AC-001 … AC-008 exactly as enumerated in the
   handoff. Nothing was added or dropped, and `_ledger.md` carries exactly these eight ids.
2. **The design's binding fence, as written, would not refuse — and this is a spelling correction,
   not a design change.** The sketch at `_design.md:331-334` appends `Event::new("SeatHeld", ...)`
   with no tags while guarding on a query tagged `("course", "c1")`. `QueryItem::matches` is
   `type_ok && tags.contains_all(&self.tags)` (`crates/happenstance-core/src/query.rs:112-115`), so
   the racing event never matches, the condition is never violated, and the program's own assertion
   fails. The fix is `.with_tags(tags)` on the racing append
   (`crates/happenstance-core/src/event.rs:372`). This sits inside the delegation the design itself
   makes — "the remaining call spellings … are the implementer's to take from the merged tree"
   (`:366-371`) — and it is exactly the class of defect AC-004 exists to catch, so it is recorded as
   a criterion (AC-004) and an error condition (EC-004) rather than as an amendment. **`_design.md`
   is not edited.**
3. **Two further spellings in the same delegation.** `read_decision_model` is absent from the
   sketch's `use` block although the body calls it — it resolves through the facade glob
   (`crates/happenstance/src/lib.rs:75`) but still needs importing; and `Guard` is *in* that block
   yet never named in the body, an unused import and one line of the 68-column budget. Both are
   implementer-level and are carried in Implementation notes rather than as criteria.
4. **`Watch a boundary refuse` is 23 characters, one over the 22-character heading budget.** The
   design names the heading in `## Composition` and separately sets the budget in `## Density
   budget`, and the two do not agree by one character. This spec does not silently re-title a
   heading a human signed off, and does not silently relax a number the design gate already refused
   to relax once. AC-007 carries the budget as written; Implementation notes require the resolution
   to be *recorded* — and if it cannot be resolved without changing the heading, EC-008 routes it to
   the sign-off owner.
5. **Tier 4 is named and assigned away rather than omitted.** All five testing-brief tiers appear in
   the Tests table; tier 4 (the human-observed falsification) is `boundary-falsification-drill`'s
   AC-005 and is marked as such. Silently dropping a tier would make the mapping unauditable, which
   is what the brief's "n/a tiers are stated, not omitted" rule exists to prevent.
6. **IQ-5 (reversibility) splits across the slice.** Reader-facing reversibility — remove the
   boundary, watch a check fail, revert, watch it pass — is the slice-mate's. What this story carries
   under that heading is the *content migration's* reversibility: AC-002 requires ADR-0006's
   reasoning to survive the crate-root rewrite verbatim. Both are stated; neither is claimed by the
   wrong story.
7. **Anti-pattern 7 is recorded as unfireable here, not as satisfied.** The wrong-model contrast is
   the bridge page's and this story authors none, so the anti-pattern cannot fire. That is the PR
   boundary holding rather than a check passing, and Interaction quality says so explicitly so a
   later reviewer does not read silence as coverage. The same reasoning covers focus and scroll
   restoration, which have no analogue because AC-007 forbids the controls that would need them.
8. **AC-006 and AC-008 are not traced project ACs, and that is deliberate.** The story traces to
   project AC-002, AC-004 and AC-006. AC-006 (mounted) and AC-008 (clause citation) here are what
   make the traced three observable to the *repository* rather than to a reader alone — AC-008 in
   particular discharges this story's per-page share of project AC-011, whose set-wide audit is
   `fence-inventory-and-clause-audit`'s. Neither widens scope; both sit inside the PR boundary fence
   already drawn.
9. **`xtask/src/narrative.rs` still does not exist in this worktree** — `xtask/src/` holds ten
   modules and none is it. The mount point is named anyway, because it is the mount point the design
   pins; EC-001 is the response, and it is halt-and-route, not improvise.
