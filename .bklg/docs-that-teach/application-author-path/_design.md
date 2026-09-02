---
item: HS-P0022
stage: design
created: 2026-08-16
updated: 2026-08-16
---

# Interaction design — The Application Author's Path

The resolved **reader-facing surface** for this project, signed off by a human before any
story spec is written. Everything below is binding on the implementer.

**Both halves of the bundled stage are live here, and that is unusual.** This repository
repurposes `_design.md` to ask for the public API surface, because a library's screen is
the surface a `cargo add` user meets. This project is the one project in the initiative
where the *literal* reading also applies: its deliverable is pages a person reads. So this
file carries the API-surface sections the template asks for (`## Items` through
`## The doctest`, mostly short, because this project adds no public Rust item) **and** the
composition sections a reader-facing surface needs (`## Surfaces` through `## States`),
which no earlier stage was permitted to decide. The UX brief is explicit that layout,
medium and page composition are this file's
(`.bklg/docs-that-teach/application-author-path/_decomposition.md:22-24`).

**No capture runs.** `design.capture` is undeclared in `.redkiln/config.yaml` — there is no
app to screenshot — so the perceptual review is skipped rather than silently passed. The
written record below is the only record these choices will ever have (project DoD item 1).
`## Anti-patterns` is therefore written so a person who cannot read Rust can check each one
against a rendered page.

**Everything numeric below was measured, not assumed.** `cargo doc -p happenstance
--no-deps` was run in this worktree and the resulting `target/doc/happenstance/index.html`
and `target/doc/static.files/rustdoc-17e0aaed.css` were read. Every selector, breakpoint,
width and font size cited is from that render on the pinned 1.97.1 toolchain. Where a value
comes from HS-P0020's undecided substrate instead, it is marked `TBD-HS-P0020` and not
guessed.

---

## Surfaces

Four surfaces. One is real and addressable today; three land in the pinned narrative tree
HS-P0020 owns and whose path does not exist yet (`{tree}` below is that unpinned root — see
`## Gaps in the substrate`).

```yaml
- id: crate-root-encounter
  route: "/happenstance/index.html"          # target/doc/happenstance/index.html locally; docs.rs/happenstance/latest/happenstance/ published
  selector: "#main-content details.top-doc > div.docblock"
  states: [default, refusal-rendered, hidden-line-audit, narrow-700, no-unresolved-brackets]

- id: opening-encounter
  route: "{tree}/first-encounter/"           # TBD-HS-P0020 pins {tree} as a constant in xtask/src/
  selector: "#main-content .docblock"        # verified for the rustdoc render shape only
  states: [default, step-one, step-two-cold, step-three-cold, refusal-rendered, falsification-drill, narrow-700]

- id: conceptual-bridge
  route: "{tree}/carry-your-invariant/"      # TBD-HS-P0020
  selector: "#main-content .docblock"        # verified for the rustdoc render shape only
  states: [default, mapping-table, wrong-side-contrast, anchor-note, narrow-700, overflow-fence]

- id: worked-example-handoff
  route: "{tree}/read-the-worked-example/"   # TBD-HS-P0020
  selector: "#main-content .docblock"        # verified for the rustdoc render shape only
  states: [default, surfaced-verbatim, vocabulary-seam, narrow-700]
```

`selector` is honest about its reach. `#main-content` and `details.top-doc > div.docblock`
were read out of the rendered `index.html` and are rustdoc's own; they hold for surfaces
2–4 **only if** HS-P0020 pins a rustdoc-rendered tree. If it pins a different renderer, the
three selectors must be re-resolved before any capture is meaningful — and since
`design.capture` is absent, no capture will run in the meantime, so the risk is a stale
manifest rather than a missed review.

---

## Pattern decision

Four tensions, three decisions (DT-5 and DT-6 are one decision wearing two numbers, and the
dossier says so at `interaction-patterns.md:611-615`).

### DT-1 — which prior mental model the teaching argues against

**Chosen: invariant-first, with the stream-per-entity prior named exactly once, at the seam
where it actually bites.** Formally option (c) from the initiative's tension table, bounded
by a single, located use of option (b).

The reader does not arrive holding a *pain*. They arrive holding an *invariant they cannot
place*. That is what AC-008's bridge already starts from — "a cross-entity rule stated in
the reader's existing vocabulary" — so the anchor is not a rhetorical choice bolted on
front; it is the material the bridge is already made of. The prior model is named at
precisely one moment: when the reader's cross-entity invariant fails to fit inside any one
stream and the reflex question — *which stream does this go in?* — is the next thing they
will think. Answering that question once, where it is asked, is the whole use the field's
device has here.

- **Rejected (a), anchor against the DDD-aggregate reader.** It is the pattern every
  DCB-adjacent source in the survey uses, and it lost on statistics: the dossier records
  that EventStoreDB, "the incumbent most readers will have prior exposure to, presents
  stream-per-entity as an unmarked default," making "one stream per entity" the more common
  prior, not "aggregates are the old model" (`interaction-patterns.md:547-566`). Anchoring
  against aggregates costs a vocabulary term the reader may not carry, to relieve a pain
  they may not have.
- **Rejected pure (c), never name a prior at all.** It forfeits the field's most reliable
  device and, worse, leaves the reflex question unanswered — the reader asks it whether or
  not the page acknowledges it. Silence there is not neutrality; it is an unanswered
  question the reader carries into every subsequent page.

**The one recorded location.** The reader-facing home is the `conceptual-bridge` surface's
section **"Where your streams went"** — a stable, non-numbered anchor (`#where-your-streams-went`,
rustdoc's own slug of the heading text; the slugging behaviour is verified in the rendered
`index.html`, e.g. `id="using-it-today"`). Every other page that relies on the anchor
decision **links to that heading** and does not re-argue it. This file is the decision's
provenance; that heading is its single reader-facing statement. That discharges UX-009,
DR-07 and AC-009 with one anchor rather than a repeated paragraph.

**Consequence that binds the other surfaces.** The `crate-root-encounter` and
`opening-encounter` surfaces name no prior model at all. Their material is the invariant and
the refusal. This is checkable and it is in `## Anti-patterns`.

*Resolves:* DT-1 (initiative tension table), AC-001, DR-06, DR-07, UI-4.

### DT-4 — how the opening encounter discloses complexity

**Chosen: (a) staged, minimal-first — three steps, each a complete runnable program, with
no fourth "whole program" artifact.**

The evidence is a controlled, replicated finding rather than a preference: Carroll's
training-wheels studies found that users initially constrained to a core feature set built
better mental models than users given full access, *measured after the constraint was
lifted* (`interaction-patterns.md:144-159`). The defect this project is fixing has exactly
the shape the finding predicts underperforms.

The three steps, fixed here so the spec does not have to invent them:

| Step | Anchor | What the reader's program does | New concept |
| --- | --- | --- | --- |
| 1 | `#append-and-read-back` | Appends two events and reads them back, printing them | `Event`, `Tags`, `Query`, `read` |
| 2 | `#a-condition-that-holds` | Appends under an `AppendCondition` that is satisfied; the append succeeds | `Guard`, `AppendCondition`, `after` |
| 3 | `#a-condition-that-refuses` | A second decision reads, something else appends first, the guarded append is **refused**; the program prints `AppendError::ConditionViolated` | the boundary, and ES-25 |

**Mitigating the named failure mode.** The literature names it directly: staged disclosure
"is problematic when the steps are interdependent and users must alternate between them" —
a reader arriving mid-sequence from search "can land past the setup with no signal they
missed it" (`interaction-patterns.md:161-166`). Three mitigations, all structural rather
than advisory:

1. **No step is a fragment.** Each step's fence is a complete program that compiles and runs
   on its own. The steps are cumulative *in teaching*, never *in execution*, which removes
   the interdependence the failure mode is about rather than warning against it.
2. **An in-body step header block**, not a sidebar affordance. Two lines above each step's
   first fence: one sentence naming what the previous step established, and a one-hop link
   to step one. It is in the body specifically because rustdoc's own TOC — the obvious place
   for it — is `nav.sidebar`, which is removed entirely below 700px (verified:
   `@media (max-width:700px)` in `rustdoc-17e0aaed.css`). A mitigation that disappears on a
   phone is not a mitigation.
3. **The payoff sits in the step most likely to be landed on cold.** Step 3 is the refusal.
   A reader who arrives there from search gets the thing the library is for on the page they
   landed on, with the header block telling them what they skipped. The failure mode is
   converted into the best available landing rather than merely signposted.

- **Rejected (b), one complete program with commentary.** It is the current defect's own
  shape: `crates/happenstance/src/lib.rs:58-67` today is a single block that constructs a
  store and asserts the log is empty, and the anti-pattern list names "an opening or
  canonical example that front-loads full complexity before naming a single concept"
  (`interaction-patterns.md:448-454`). It also has no answer to the mid-sequence reader
  beyond "scroll up".

*Resolves:* DT-4, AC-002, DR-04, IQ-3, UX-005.

### DT-5 + DT-6 — how the shift is drawn, and how honest its old side is

One joint resolution, as AC-003 requires.

**DT-5, chosen: (c) narrate the cycle — a three-column mapping table plus a fixed four-step
numbered narration. No diagram ships from this project.**

- The device is convergent across three independent sources on exactly this content and
  costs nothing to build: dcb.events' own write cycle, Marten's design docs (numbered list
  *and* a spec-concept-to-implementation mapping table), and a Chronicle event-store writeup
  (`interaction-patterns.md:317-340`).
- **Rejected (a), diagram both models.** Three costs, each real. It is original work nobody
  in the field has completed — dcb.events itself diagrams the model it is retiring and has
  no equivalent for the one it introduces (`interaction-patterns.md:296-307`). It requires
  adopting a renderer with zero precedent in this workspace (zero diagrams exist today, and
  no diagram tooling is wired into `cargo xtask ci`). And a diagram is "prose the gate can
  render but not typecheck" (`interaction-patterns.md:520-526`) — spending this project's
  lowest-confidence budget (BR-18, a `Could`) on its least checkable artifact.
- **Rejected (b), draw neither and narrate nothing.** It leaves the shift taught only in
  running prose, which is what the mapping-table evidence exists to improve on.
- **Mitigating (c)'s known weakness** — it is weaker for a spatial reader. The mapping table
  is the spatial device text can carry, so it is given a fixed shape rather than a loose one:
  exactly three columns, in this order, on every page that shows it.

  | Your words | This library's words | Where you saw it |
  | --- | --- | --- |
  | "the rule spans two entities" | a `Query` with one `QueryItem` per entity's tag | step 3, line 6 |

  And the four-step narration uses the same four step names — *tag, query, fold, guard* —
  everywhere it appears in the page set. Synonym drift across pages is the way a narration
  loses to a diagram, and a fixed vocabulary is the cheap fix.
- **AC-013 is discharged explicitly, not vacuously by silence.** No diagram ships; the
  reviewer records that this is how it went (storymap coverage row AC-013). UX-014's
  obligation — that any shipped diagram carries a narration beside it — is why this choice
  costs nothing a diagram would not also cost.

**DT-6, chosen: (a) real, compiled, executed code — in the narrowest form the contrast can
take. This project ships zero uncompiled fences and puts nothing on HS-P0020's enumerated
allowance list.**

The wrong side is not a second, deliberately inferior implementation maintained forever.
It is **one fence on the bridge page that differs from the correct fence by one expression**:
an `AppendCondition` whose guard query is **tagged too narrowly** — scoped to the entity the
command is *writing* rather than to the invariant it must *hold* — so the conflicting event
is not in the guard's query at all. It does not assert that the wrong code is wrong in prose;
it **asserts the outcome the reader should fear**: the append is *accepted*, and the invariant
is violated with no error.

**This is the mirror of CF-7, not CF-7 itself, and the distinction is the whole point.**
CF-7 (`spec/SPECIFICATION.md:7266`, `[FROZEN]`) names the *broadening* failure: an adapter
that drops the tag join from its condition probe matches more than it should and "rejects
every command touching any course… a total-availability failure certified as conformant"
(`:7286-7288`). A type-only guard therefore **over-refuses**. It is the wrong wrong-model for
this page: a reader who writes one sees `ConditionViolated` — a loud, safe, obvious failure —
which teaches nothing about the hazard the boundary exists to prevent. Asserting `is_ok()`
against it would assert on a call that returns `Err`, and the fence would simply fail.

The hazard worth teaching is the quiet one, and it is the opposite move: **narrowing** the
guard so the query misses the event that would have refused it. Tag the guard to the student
when the invariant spans the course's seats, and the seat-exhaustion event never enters the
guard's query — the append succeeds, capacity is exceeded, and nothing in the type system,
the API or the store objects. That is the failure a reader can actually ship, and it is the
one the correct fence's tag join exists to prevent.

**Corrected at the design gate on 2026-08-17**, after the first resolution specified a
type-only guard asserting acceptance — a construction that is backward against CF-7 and would
have shipped a failing assertion. The first mock detected the contradiction and silently
substituted a wrong-entity-tagged fence to make its frame pass, without amending this section;
the second run's finding F-6 derived the same correction independently. It is folded in here
rather than left for a third discovery.

That inverts DT-6's stated cost. The maintenance liability is not "keep a worse design
compiling"; it is "keep an assertion about what the library does not protect you from". If
the library ever changes so a too-narrowly-tagged guard *does* refuse, the fence fails and a
person looks — which is the correct outcome, not a chore.

- **Rejected (b), illustrative and explicitly exempt.** It reproduces the `ignore`-fence
  shape this repository has already found once in its own doctests
  (`interaction-patterns.md:269-279`, `:418-423`), and writing the narrow exemption AC-003
  would then demand is how the exemption becomes the loophole the dossier names at
  `interaction-patterns.md:543-545`. Declining to open the list at all is strictly stronger
  than opening it narrowly.
- **Rejected "no contrast at all", prose plus clause citation only.** It leaves AC-006's
  teaching — no empty boundary where prose claims a real one — with no positive
  demonstration, and CF-7's already-written cautionary case ("rejects every command touching
  any course, and passes all twenty-seven rules while doing it") is the reader's exact fear
  made concrete and is wasted by not showing it.
- **Mitigating (a)'s failure mode** (the contrast occluding the thing it contrasts, IQ-2.3):
  the wrong fence is marked wrong at its start *and* at its end, is never the last code on
  its page, never appears above the correct version in reading order, and is never inside a
  `details`. All four are in `## Anti-patterns` as screenshot checks.

*Resolves:* DT-5, DT-6, AC-003, AC-013, DR-10, DR-11, DR-12.

---

## Items

**This project adds, changes and removes no public Rust API item.** That claim is examined
rather than assumed, per the template's own warning.

```yaml
- path: "happenstance"                                    # the crate root's rendered documentation
  kind: "doc"
  change: "signature-changed"                             # the rendered surface changes; no item does
  feature: "default"
  clause: "ES-25"                                         # spec/SPECIFICATION.md:3693 [FROZEN]

- path: "examples/course-subscriptions/src/overview.md"   # new file; publish = false crate
  kind: "doc"
  change: "added"
  feature: "n/a"
  clause: "n/a"

- path: "happenstance [dev-dependencies] tokio"           # required to make the refusal fence execute
  kind: "feature"
  change: "added"
  feature: "dev-dependency"
  clause: "ES-25"
```

Three notes, because two of those rows are easy to wave through.

**The crate root's rendered docs are this crate's screen**, and this project rewrites them.
`.kb/governance/rewrite-the-referent-never-the-reasoning.md` is the test: ADR-0006's argument
for why the bare name sits on the typed layer (`crates/happenstance/src/lib.rs:20-25`) is
reasoning that still stands and is preserved verbatim. What is replaced is the
`# Using it today` section's fence, which asserts an empty log and demonstrates nothing.

**The `tokio` dev-dependency is not incidental.** `crates/happenstance/Cargo.toml` declares
**no** dev-dependencies at all today, which is exactly why the existing fence wraps its body
in `# async fn example()` and never calls it — verified at
`crates/happenstance/src/lib.rs:61-66`. That fence type-checks an async function that is
never awaited. AC-004 and IQ-7 require the refusal to be the *program's own output*, so the
fence must execute, so the crate needs a runtime under `[dev-dependencies]`. `tokio` with
`macros` + `rt` is the workspace dependency `examples/course-subscriptions/Cargo.toml`
already uses.

**`overview.md` is an extraction, not a rewrite.** See `## Placement and re-export`.

---

## Signatures

No public signature changes. The two code artifacts this project *does* fix in place are
written out here so review is against the thing rather than a description of it.

```rust
// The crate root's replacement fence — the whole of it, hidden lines included, so the
// hidden-line audit (UX-003) can be performed against this file before it is written.
// Nothing hidden constructs the boundary or asserts its refusal.
//
// ```
// use happenstance::{
//     AppendCondition, AppendError, Event, EventStore, Guard,
//     MemoryEventStore, Query, QueryItem, Tags,
// };
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn core::error::Error>> {
//     let store = MemoryEventStore::new();
//     let seats = Query::from_items([QueryItem::new(
//         ["SeatHeld"],
//         Tags::from_iter([("course", "c1")]),
//     )?])?;
//
//     // Read the boundary, note where it ended, then let someone else win the race.
//     let (_held, upto) = read_decision_model(&store, &seats).await?;
//     store.append(&[Event::new("SeatHeld", ...)?], None).await?;
//
//     let refused = store
//         .append(
//             &[Event::new("SeatHeld", ...)?],
//             Some(&AppendCondition::new(seats).after_opt(upto)),
//         )
//         .await;
//
//     println!("{refused:?}");                       // the refusal, in the program's output
//     assert!(matches!(refused, Err(AppendError::ConditionViolated(_))));
//     Ok(())
// }
// ```
```

**The shape above is binding and was corrected at the design gate on 2026-08-17** (finding
F-5). The earlier sketch called two things that do not exist: `store.head(&seats)` —
`EventStore::head` takes **no argument** and returns `Option<SequencePosition>` for the whole
log (`crates/happenstance-core/src/store.rs:248`) — and `Guard::new(seats, upto)`, where
`Guard` is `#[non_exhaustive]` with no constructor and `AppendCondition::new` takes a single
`Query`, not a list of guards (`crates/happenstance-core/src/append.rs:132,146,237`). The
verified spelling is `read_decision_model` + `AppendCondition::new(query).after_opt(last)`,
which is also the pairing the contract crate's own documentation names
(`store.rs:315-316`, `:321-331`). It is one line shorter than the old sketch and structurally
different, so "the shape is binding" is now a true statement rather than one the real API
cannot satisfy.

The remaining call spellings (`Query::from_items`, `Event::new`'s payload argument) are
the implementer's to take from the **merged** `crates/happenstance/src/lib.rs`, not from this
worktree's 75-line copy — AC-014 and DR-13 gate authoring on that merge, and the API this
fence calls is the merged one. What is binding here is the *shape*: every line that
constructs the boundary or observes its refusal is visible, and the program prints before it
asserts.

```rust
// The wrong-side contrast on the bridge page. One expression different from the correct
// fence above it, and it asserts the outcome the reader should fear.
//
// ```
// // WRONG — a type-only guard on a cross-entity invariant. Read on for why. (CF-7)
// let guard_query = Query::from_items([QueryItem::of_types(["SeatHeld"])?])?;
// // ... same append, same condition shape ...
// assert!(accepted.is_ok(), "a type-only guard does not refuse — this is the bug");
// // END WRONG. The correct version is the fence above this one.
// ```
```

---

## Shape decision

| Item | Chosen shape | Rejected (and why) | Evidence | Resolves |
| --- | --- | --- | --- | --- |
| Crate-root fence | An executing `#[tokio::main]` program that prints then asserts | A `# async fn` wrapper that is never awaited — the current shape, which type-checks and demonstrates nothing | `crates/happenstance/src/lib.rs:61-66`; AC-004, IQ-7 | The measured gap the project names |
| Step count | Three, each independently runnable | Five-plus steps (starvation at depth for a mid-sequence reader); one step (DT-4 option b) | `interaction-patterns.md:144-166` | DT-4 |
| Mid-sequence signal | In-body step header block | Sidebar TOC position — removed entirely below 700px | verified `@media (max-width:700px)` in `rustdoc-17e0aaed.css` | IQ-3, UX-005 |
| Prior-model anchor | One named heading on the bridge page, linked from elsewhere | A paragraph repeated per page (the exact defect BR-07 exists to prevent) | `interaction-patterns.md:547-566` | DT-1, AC-009 |
| The shift's visual | Three-column mapping table + four-step narration | A Mermaid event-model diagram (new dependency, no CI precedent, untypecheckable) | `interaction-patterns.md:317-340`, `:520-526` | DT-5, AC-013 |
| The wrong side | One compiled, executed fence asserting the feared acceptance | An `ignore`-class fence on an allowance list | `interaction-patterns.md:269-279`, CF-7 at `spec/SPECIFICATION.md:7266` | DT-6, AC-003, AC-007 |
| Worked-example surfacing | Extract to `overview.md`, include from both readers | Paraphrase into the tree page (AC-010 forbids); link-only with no surfacing | `crates/happenstance/src/lib.rs:10`; `xtask/src/constitution.rs:11-18` | AC-010 |

**Provisional clauses carried, not resolved.** VT-30 (`spec/SPECIFICATION.md:1813`) is
`[PROVISIONAL]` and the bridge lands the reader on the shape it describes. No page may imply
that shape is frozen, and no page may be written in a way that would need rewriting if VT-30
changes — cite the id, describe the guard the reader wrote, never restate the clause
(AC-011, DR-09). ES-25 (`:3693`) and CF-7 (`:7266`) are `[FROZEN]` and are cited as such.

---

## Composition

The arrangement, region by region, top to bottom in reading order. This answers where each
element sits **relative to the others** — the question the briefs deliberately did not.

### `crate-root-encounter` (verified DOM; this is the one surface whose regions are real)

Rustdoc's own frame, unmodified, in this order:

1. **`nav.sidebar`, 200px, left, `position: sticky`, `height: 100vh`** — carries the crate
   name, version, "All Items", the auto-built section TOC (`#rustdoc-toc ul.top-toc`,
   generated from `##` headings), and the item-kind index. **This project writes none of it
   and must not try to.** It writes the `##` headings the TOC is built from, which is the
   only handle it has on the sidebar and is why heading length is a density budget item.
2. **`main > .width-limiter`, max-width 960px** — the content column.
   1. **`.main-heading > h1`** — "Crate happenstance". Rustdoc's; not ours.
   2. **`details.toggle.top-doc[open] > .docblock`** — everything this project writes on this
      surface, in this order:
      1. The one-line crate summary (existing: "DCB-compliant event sourcing, with
         batteries."). Unchanged.
      2. **The answered-need line**, in HS-P0021's notation, immediately below the summary
         and **above every `##`**. It is above the first fence by construction because it is
         above everything (IQ-8, UX-008).
      3. `## Status: a facade over happenstance_core` — ADR-0006's reasoning, preserved.
         Retitled only to remove the unresolved bracket pair (see `## States`).
      4. **`## Watch a boundary refuse`** — replaces `## Using it today`. Two sentences, then
         the fence, then one sentence pointing at step one of `opening-encounter`. The fence
         is the first code on the page and the refusal is inside it.
      5. `## What arrives here, and what stays below` — the planned-surface list, unchanged,
         **moved below** the fence. It is orientation for a reader who already believes the
         library does something; today it sits above the only code on the page, which is
         backwards.
      6. The adapter-author redirect sentence, last. Unchanged in substance.
   3. Rustdoc's item tables (`## Modules`, `## Structs`, …). Rustdoc's; not ours.

The single structural change to this surface is **the fence moves up and the roadmap moves
down**. That is the composition decision: a reader who lands on docs.rs meets the refusal
before they meet the plan.

> **Amended 2026-08-19 — BC-002. Region 4 above is struck, and the region list is left
> standing as the audit trail.** The crate root does **not** carry a
> `## Watch a boundary refuse` / `## A boundary refuses` section. The refusal is authored
> once, on `docs/first-encounter.md`, and the crate root carries the answered-need line
> (region 2, unchanged) plus **one sentence directly beneath the existing fence pointing at
> step one**. Regions 1, 2, 3, 5 and 6 are unchanged.
>
> **Why the design was right when it was signed off and is wrong now.** It was written against
> a `crates/happenstance/src/lib.rs` whose first code was a never-awaited
> `# async fn example()` wrapper under `## Using it today`. Merging
> `initiative/from-contract-to-published-library` forward (`a5c0f30`) replaced that page
> wholesale, and the merged page brings its own signed-off assertions with it:
> `crates/happenstance/tests/doc_budget.rs:157` requires the crate root to carry **exactly one**
> fence — *"a second one would demote the first, which is the page's primary hierarchy
> signal"* — and `MODULE_DOC_LINES = 130` caps the module doc, which the answered-need line and
> pointer take to exactly 130. The section could therefore exist only by deleting HS-P0016's
> `commit` landing program, which `project.md`'s risk table places outside this project's seam
> (*"does not touch landing copy"*), which two vocabulary bullets name, and which is the crate's
> one demonstration of the typed layer ADR-0006 gave the bare name to.
>
> **What is lost, stated rather than glossed.** The composition decision above — *a reader who
> lands on docs.rs meets the refusal before they meet the plan* — is now met only in the weaker
> sense that the roadmap does not survive at all and the fence is already first. That reader
> meets a `commit` program, not a refusal, and reaches the refusal one hop later. The stronger
> reading is not delivered and is not deemed delivered.
>
> Routed as BC-002 by `boundary-refusal-encounter`, which recorded the contradiction under
> EC-008 and stopped rather than folding it in or editing this file. Decided by the `_design.md`
> sign-off owner on 2026-08-19 and recorded here rather than in a commit message.

### `opening-encounter`

One page, three steps, no index page in front of them. An index would be a fourth page whose
answered-need is "find the other three", which is a findability need this project does not
own (DT-3 and DT-10 are HS-P0021's and HS-P0023's) and which would need its own
answered-need line.

Per step, in fixed order:

1. `## <step heading>` — the stable anchor.
2. **The step header block**, two lines, immediately under the heading and above everything
   else: line 1 names what the previous step established; line 2 is the one-hop link to step
   one. On step one, line 1 is replaced by the answered-need line and line 2 is omitted.
3. One or two sentences of setup. Never more — the fence is the teaching.
4. **The fence.** Complete, runnable, ≤ 24 rendered lines on a step page (see
   `## Density budget`; the crate-root encounter is exempted at 32, because the program
   written against the real API is 31 lines and cannot be reduced).
5. **The output block** — what the program prints, as a fenced text block. On step 3 this
   block contains `ConditionViolated` and it is the payoff.
6. The clause citation, inline, as the last sentence: a link carrying the clause id.

Step 3 additionally carries, after item 6, **the falsification drill** as its own `###`:
the exact edit, the exact failure output, the exact revert (IQ-5, UX-002, AC-005). It is at
the bottom of step 3 rather than on a page of its own because it is only performable by
someone who has just run step 3.

### `conceptual-bridge`

In reading order, and the order is load-bearing:

1. The answered-need line.
2. **`## Your rule, in your words`** — the cross-entity invariant in ordinary event-sourcing
   vocabulary. Prose only, no code, no library terms.
3. **`## Where your streams went`** — the DT-1 anchor, and the only place in the whole page
   set where a prior model is named. Three or four sentences.
4. **`## Tag, query, fold, guard`** — the four-step narration, then the three-column mapping
   table directly beneath it. Table under narration, not beside it: at 764px there is no
   room to place them side by side (see `## Density budget`) and a two-column layout would be
   custom CSS, which UX-012 forbids.
5. **`## The guard you would write`** — the correct fence. First code on the page.
6. **`## What a type-only guard misses`** — the wrong-side contrast fence, marked at both
   ends, **strictly after** §5.
7. **`## Back to the working version`** — one paragraph and a link up to §5's anchor, so the
   last thing in reading order is the correct model. This section exists solely so the wrong
   fence is never the last code on the page (IQ-2.3), and it is cheap.

### `worked-example-handoff`

1. The answered-need line.
2. **Orientation, two sentences** — what the reader is about to read and why (UI-5's "they
   know before they leave").
3. **The vocabulary-seam sentence, one sentence, immediately before the link** — the example
   imports `happenstance_core` directly (`examples/course-subscriptions/src/main.rs:24-27`)
   and that is correct for its own purpose. Named, not hidden (UX-011, IQ-9). It sits before
   the link rather than after it because a reader who follows the link does not come back.
4. **The surfaced module doc**, verbatim, as an include of `overview.md`.
5. The link to the example source, last.

---

## Transience policy

Every control on every surface, classified with a reason. "Persistent because nobody decided
otherwise" is the defect this section exists to catch, so rustdoc's own chrome is classified
here too even though this project does not author it — a control this project cannot change
is still a control the reader meets.

| Control | Class | Reason |
| --- | --- | --- |
| `nav.sidebar` + `#rustdoc-toc` | **Persistent chrome** (medium's) | Rustdoc renders it; `position: sticky`, `height: 100vh`, 200px. Below 700px it is removed and replaced by a 45px `rustdoc-topbar`. This project adds nothing to it and depends on nothing in it. |
| `rustdoc-topbar` | **Persistent chrome** (medium's), ≤700px only | Verified: `--topbar-height: 45px` and `*[id]{scroll-margin-top:var(--topbar-height)}` in the `max-width:700px` block, so anchors still land correctly under it. Nothing to do. |
| `#copy-path`, `rustdoc-toolbar`, settings, help, search | **Persistent chrome** (medium's) | `#copy-path` is `visibility:hidden` below 700px — rustdoc's own decision. |
| `details.toggle.top-doc` | **Persistent, expanded** | Verified rendered as `<details class="toggle top-doc" open>`. Everything this project writes on the crate root lives inside it and is therefore visible without interaction. Binding: no content this project authors may land inside a `details` that is not `open`. |
| The answered-need line | **Persistent** | IQ-8/UX-008 require it above the first fence in reading order. It is never inside a fold, never a tooltip, never a badge in a corner. |
| The step header block | **Persistent, in-body** | The mid-sequence mitigation cannot live in chrome that disappears at 700px. This is the entire reason it is body text and not a sidebar item. |
| Every fence | **Persistent** | A fence is the teaching. None is behind a control. |
| The program's output block | **Persistent** | IQ-7: the refusal is observable, not narrated. Hiding the output is hiding the claim. |
| Hidden doctest lines (`#` prefix) | **Not a transience class in this medium — binary** | Verified, and stronger than the brief assumed: the rendered `index.html` contains **only** the four visible lines of the crate-root example; the five `#`-prefixed lines are absent from the DOM entirely. There is no hover, no focus, no toggle that recovers them. Permitted for: the `use` block *only where already shown once earlier on the same page*, `Ok(())`, and struct-literal filler. **Forbidden for** any `Query`, `QueryItem`, `Tags`, `Guard`, `AppendCondition`, the append call, the `head`/read call, and every assertion (UX-003, IQ-2.1, DR-02). |
| The wrong-side contrast | **Persistent, never opened-on-demand** | Putting it behind a fold would be the load-bearing-content-behind-a-fold anti-pattern (`interaction-patterns.md:404-410`) applied to the one block on the page most likely to be misread. Visible, marked at both ends, bracketed by correct code. |
| Clause citations | **Persistent, inline, recessive** | Never a tooltip or popover — that would put a normative reference behind a hover, which fails on touch and on print. Last position in a sentence, ordinary link. |
| The falsification drill | **Persistent, `###` under step 3** | AC-005's proof is a repository check; the reader-facing instructions are the same content and get the same visibility. |
| Anything else | **Does not exist** | IQ-6's test is "enumerate the interactive affordances this project introduced" and the correct answer is none. No tabs, no accordions, no nav widget, no CSS, no JS (UX-012). |

---

## Density budget

Real numbers, all measured from this worktree's render on the 1.97.1 toolchain
(`target/doc/static.files/rustdoc-17e0aaed.css`, `target/doc/happenstance/index.html`).

### Rendered geometry

| | 1440×900 | 1024×768 |
| --- | --- | --- |
| Sidebar | 200px (`--desktop-sidebar-width`), sticky, full height | 200px, sticky, full height |
| Content box | 960px (`.width-limiter{max-width:960px}` binds) | 824px (viewport − sidebar; the 960 cap does not bind) |
| Text column | **900px** (box − `main` padding `45px` left, `15px` right) | **764px** |
| Fence inner width | **892px** (column − `.content{margin:0.25em 0.5em}`, `.docblock{margin-left:24px}`, `pre{padding:14px}` × 2) | **696px** |
| Body type | 16px / 24px line-height (`body{font:1rem/1.5}`) | same |
| Code type | **16px / 24px** line-height | same |
| Headings | h1 24px, h2 22px, h3 20px; `h1..h4{margin:25px 0 15px}` + 6px padding-bottom | same |
| Vertical budget above the fold | 900px | 768px |

### The budgets that follow

- **Fence width: 68 columns, hard.** *(Re-derived at the design gate on 2026-08-17 from
  finding F-1; the earlier figure of 84 rested on a code type size that does not apply.)*
  Code renders at **16px, not 14px**: the cited `code{font-size:0.875rem}` is scoped to
  `.item-info code`, and `normalize-9960930a.css`'s `code,kbd,samp{font-size:1em}` governs
  `.docblock` fences. At 16px Source Code Pro (≈9.6px advance) the **696px** fence at
  1024×768 holds **72** characters before `overflow-x` engages; 68 leaves four of headroom.
  The 1440×900 case holds 92. **This is tighter than the repository's own rustfmt setting** —
  `rustfmt.toml` declares only `edition = "2024"`, so `max_width` is the default 100, which
  overflows the code column at 1024×768 by roughly 13 characters. Doctest fences are not
  rustfmt'd, so this budget is the author's to keep, and keeping it means breaking `use`
  blocks and long call chains by hand.
- **Fence height: 24 rendered lines on a step page, hard; the crate-root encounter is
  exempted at 32.** *(Re-derived at the design gate on 2026-08-17 from findings F-1 and
  F-4.)* At the corrected 24px line-height, 24 lines + 28px padding = **604px**, not 532px,
  leaving 164px at 1024×768 for the heading, the step header block and the setup sentence.
  That still fits a step page, and step 3's program remains the binding case at roughly 20
  lines. **The crate-root refusal program does not fit and cannot be made to.** Written
  against the real API it is 31 rendered lines with exactly one line (`Ok(())`) eligible for
  hiding, so a 24-line ceiling there is not a budget but a contradiction — the design would
  be forbidding the program it also specifies. The ceiling is therefore 32 on
  `crate-root-encounter` alone, with the reason recorded rather than the number quietly
  relaxed: the crate root is the one surface where the fence *is* the page, and it has no
  step header block or setup prose competing for the fold.
- **`##` heading length: 22 characters — and the failure it prevents is *truncation*, not
  wrapping.** *(Corrected at the design gate on 2026-08-17 from finding F-2.)* The sidebar
  TOC is built from `##` headings and is 200px wide with
  `--sidebar-elems-left-padding: 24px`, leaving ~170px. A longer entry does **not** wrap:
  `.sidebar-elems .block li a{white-space:nowrap;text-overflow:ellipsis;overflow:hidden}`
  (`target/doc/static.files/rustdoc-17e0aaed.css`) clips it with an ellipsis. The budget
  stays, because a heading silently cut mid-word in the only navigation the page has is a
  real defect a reader meets; but it is now stated as the thing that actually happens.
  Eleven of the sixteen headings this design names exceed it, so the retitles in
  `## Composition` are structural, not cosmetic.
- **Paragraph length: 435 characters.** At 764px and ~8.8px per character of 16px Source
  Serif, that is five rendered lines. A sixth line is the point at which a reader scanning
  for the fence starts skipping paragraphs entirely.
- **Per-step budget: one heading + two header lines + two setup sentences + one fence + one
  output block + one citation sentence.** Seven elements. An eighth means the step is doing
  two things and should be two steps — but three steps is also the ceiling (DT-4), so in
  practice an eighth element means something is cut.
- **Mapping table: three columns, never four.** At 764px, four columns give ~180px each,
  which is ~20 characters of 16px prose per cell before wrapping — narrower than the terms
  being mapped. Three columns give ~245px, ~28 characters, which holds `a Query with one
  QueryItem per tag` in two lines.
- **Minimum legible size for the primary label: 14px.** Nothing this project authors renders
  below rustdoc's smallest text (code at 0.875rem), because this project introduces no CSS
  and therefore cannot make anything smaller. The primary label — the answered-need line — is
  body text at 16px.

### What yields first, in order

When a step exceeds its budget, cut in this order and stop as soon as it fits:

1. **Explanatory comments inside the fence.** The prose above the fence can carry them.
2. **The setup sentences** above the fence, down to one.
3. **A `use` line**, by hiding it with `#` — **but only if the same import was already shown
   visibly earlier on the same page**, and never for a type that participates in the
   boundary.
4. **Split the step.** Only after the above, and only if the split leaves both halves
   independently runnable — which at three steps means the step was mis-scoped.

**What never yields, at any width:** the answered-need line, the step header block, any line
that constructs a `Query`/`Tags`/`Guard`/`AppendCondition`, the append call, the refusal
assertion, the output block, and the clause citation. A fence that cannot fit 84×24 with
those intact is a fence whose *scenario* is too large, and the scenario shrinks — not the
boundary.

---

## Hierarchy

Per region: what is primary, what is secondary, what is recessive, and — the part that
usually goes unstated — **what carries the distinction**, given that this project may not
introduce a single style rule.

The only three channels available are **position in reading order**, **heading level**, and
**form** (fence vs. table vs. body vs. link). Colour is not a channel here and never will be
(the accessibility floor's "colour is never the only channel" rule is satisfied trivially,
because this project authors no colour).

### `crate-root-encounter`

- **Primary — the refusal.** Carried by position: `## Watch a boundary refuse` is the first
  `##` after the status section and the fence is the first code on the page. Reinforced by
  form: it is the only fence, and the printed output sits directly beneath it.

  > **Amended 2026-08-19 — BC-002.** Struck. On the merged page the primary element is
  > HS-P0016's `commit` program, which is the first code and the only fence; the refusal is
  > not on this surface at all. What this project adds here is recessive by comparison: the
  > answered-need line above everything, and a pointer beneath the fence. The refusal's
  > hierarchy claim now belongs to `opening-encounter`, which is where the refusal is.
- **Secondary — the answered-need line and ADR-0006's reasoning.** Carried by position
  (above everything, and immediately after, respectively) and by heading level (`##`).
- **Recessive — the planned-surface roadmap and the adapter-author redirect.** Carried by
  position: below the fence, at the end. Both were primary by accident before this project;
  demoting them is the composition change.

### `opening-encounter`

- **Primary — the fence and its output block**, in each step. Carried by form and by the
  fact that everything else in the step is one or two lines long.
- **Secondary — the step header block.** Carried by position (first thing under the heading)
  and by brevity. It must be findable in a half-second by a reader who arrived cold; it must
  not compete with the fence for a reader who arrived in order. Two lines does both.
- **Recessive — the clause citation.** Carried by position (last sentence) and form (an
  ordinary inline link). It is provenance, not required reading — IQ-1 requires that striking
  every off-page link leaves the argument complete.

### `conceptual-bridge`

- **Primary — the correct fence (§5) and the mapping table (§4).** Carried by form and by
  being the only two non-prose elements before §6.
- **Secondary — the DT-1 anchor section (§3) and the four-step narration (§4).** Carried by
  heading level and by position ahead of the code.
- **Recessive — the wrong-side contrast (§6).** This is the deliberate inversion, and it is
  the whole of IQ-2.3: the contrast is *visually equal* to the correct fence (same form, same
  medium, no dimming available) but *hierarchically below* it, and the only channels that can
  carry "below" are position and framing. So it gets both — after the correct version, and
  bracketed by an opening marker, a closing marker, and §7's return to the working version.
- **Recessive — clause citations**, as above.

### `worked-example-handoff`

- **Primary — the surfaced module doc.** Carried by length and form: it is the bulk of the
  page and it is someone else's voice, verbatim.
- **Secondary — the orientation sentences.** Position: above it.
- **Recessive — the vocabulary-seam sentence.** One sentence, immediately before the link.
  Recessive is correct here: it is a caveat a reader needs at the moment of leaving, not a
  topic. Making it a `###` would turn a friction into a subject.

---

## States

Every state the surfaces can be in, and what each renders. The template's "states the API
must express" question and the composition question have the same answer here, because on a
documentation surface the states *are* the reader's situations.

| State | What renders |
| --- | --- |
| **Empty** | Has no analogue for prose, but its real form does: **a step whose program produces no output.** Forbidden. Every step's program prints — step 1 prints the events it read back, step 2 prints the accepted append's position, step 3 prints the refusal. A program that only asserts is the current defect's shape (`crates/happenstance/src/lib.rs:58-67` asserts an empty log and prints nothing). |
| **Loading** | Does not exist. Every surface is static text rendered at build time. Rustdoc's search is the only asynchronous thing on the page and it is the medium's, untouched. |
| **Error — the taught one** | `AppendError::ConditionViolated`, in the step-3 program's own stdout, in a fenced output block directly under the fence. Cited to ES-25 (`spec/SPECIFICATION.md:3693`, `[FROZEN]`), never restated. |
| **Error — the drilled one** | The falsification drill's expected `cargo test` failure, quoted exactly as the reader will see it, so they can recognise *the* failure rather than a build error (UX-002, IQ-5). Followed by the exact revert and the passing output. |
| **Error — the feared one** | On the bridge page only: the wrong-side fence asserting that a type-only guard **accepts** an append that should have been refused (CF-7, `spec/SPECIFICATION.md:7266`, `[FROZEN]`). |
| **Overflow — width** | A fence over 68 columns gets a horizontal scrollbar inside `.example-wrap > pre` at 1024×768 (72 is where it actually engages, at the corrected 16px code type). The medium handles it; the budget forbids reaching it. Checkable on a screenshot. |
| **Overflow — height** | A fence over 24 rendered lines pushes its own output block below the fold at 768px, which breaks the one thing step 3 exists for. The budget forbids it; the yield order in `## Density budget` says what goes. |
| **Long label** | A `##` over 22 characters is **clipped with an ellipsis** in the 200px sidebar TOC — `white-space:nowrap` means it never wraps. Eleven of the sixteen headings this design names do this today. Every heading this project ships is inside the budget; the retitles are listed in `## Composition`. |
| **Narrow viewport (≤700px)** | The sidebar is removed and a 45px `rustdoc-topbar` replaces it; `scroll-margin-top: 45px` keeps every anchor landing correctly under it (both verified). **Nothing this project authors depends on the sidebar being visible** — which is precisely why the mid-sequence mitigation is an in-body block. At ≤464px rustdoc applies a further breakpoint; nothing this project writes changes at it. |
| **Mid-sequence cold arrival** | Any step heading opened directly: the step header block is the first thing under the heading, naming what the previous step established and linking to step one in one hop (IQ-3, UX-005). |
| **Handed off** | The reader has read the orientation and the vocabulary-seam sentence *before* the link, and lands on the example knowing it imports `happenstance_core` and why that is correct for it. |
| **Unresolved reference** | The state to eliminate, and it exists today: the rendered crate root contains **four** literal `[happenstance_core]` bracket pairs where an intra-doc link failed to resolve — verified by counting `[<code>` in `target/doc/happenstance/index.html`, and `cargo doc -p happenstance --no-deps` printed no warning about them under that invocation. Zero is the target on every page this project authors, and it is a screenshot check. |
| **Print / no-JS** | Rustdoc ships `@media print` and a `noscript` stylesheet. Everything this project authors is static prose, fences and tables, so all of it prints and all of it renders with JS off. This is a consequence of introducing no affordance, not a separate design. |
| **Reduced motion** | Nothing animates. Rustdoc itself guards its transitions behind `@media not (prefers-reduced-motion)` (verified). This project introduces nothing that could violate it, so there is nothing to test. |

---

## Placement and re-export

Where each artifact lives, and what forced it there.

**The crate-root fence lives in `crates/happenstance/src/lib.rs`** — not in the pinned tree
with a pointer — because the docs.rs reader who ran `cargo add happenstance` lands there
first and the whole measured defect is what that page shows them.

**The three steps live in HS-P0020's pinned tree**, not in the crate root, because three
runnable programs at ~20 lines each would be ~70 lines of fence on a page that also has to
be a crate index.

**Step 3's program therefore exists twice** — once on the crate root, once as step 3. This
is a real cost and the compiler cannot close it. `crates/happenstance/src/lib.rs:7-9` records
why, in this repository's own words: `include_str!` resolves against the file tree at compile
time, and a path escaping the package "would not resolve once published". The pinned tree is
outside `crates/happenstance/`, so a published crate root cannot include a file from it. The
mitigation is that **both copies are compiled and executed** — the crate root's by
`cargo test --doc` under the `"tests"` REQUIRED step, the tree's by HS-P0020's compiled-fence
step — and both assert the same `ConditionViolated`. Drift between them can be silent about
*identity*; it cannot be silent about *correctness*. The consolidation that would remove even
that is a shared file under `crates/happenstance/examples/` included by both, and it depends
on what HS-P0020's mechanism can include; it is routed there as a substrate question rather
than assumed here (project DoD item 9).

**`examples/course-subscriptions/src/overview.md` is new, and `main.rs`'s `//!` block becomes
`#![doc = include_str!("overview.md")]`.** This is the exact shape already wired at
`crates/happenstance/src/lib.rs:10` and it makes AC-010's "surfaced, not paraphrased" true
*by construction* — one file of bytes, rendered in two places — rather than by a reviewer
diffing prose. The extraction changes no assertion the example makes, which is the test
`.kb/governance/rewrite-the-referent-never-the-reasoning.md` sets. **The example's imports are
not touched**, per the UX brief's Notes.

Its cost, stated: planning artifacts cite `examples/course-subscriptions/src/main.rs:1-19`
(`_grounding.md`, `project.md`, `_storymap.md`), and those citations go stale when the prose
moves. `cargo xtask spec-trace` does not read the example, so no gate breaks; the closeout's
reference reconciliation is where those pointers are repaired. If HS-P0020's mechanism can
include a source file's module doc directly, the extraction becomes unnecessary and should be
dropped — another substrate question routed there.

**Verbatim surfacing uses one module per included file.** Where the `#[cfg(doctest)] mod X {
#![doc = include_str!(…)] }` shape is used to *check* an included file, it is one module per
file, because concatenated includes report a failure "at a line number counted from the
first, which maps to no file a reader can open" (`xtask/src/constitution.rs:11-18`). That
constraint travels with whichever mechanism HS-P0020 pins.

**Nothing is added to `docs/README.md:12-23`.** The signpost table is the repository's only
routing primitive and it is tempting, but DT-10 and every pointer policy are HS-P0023's. This
project makes material reachable; it does not decide where the pointer lives.

---

## Visibility and stability

| Item | Visibility | `#[non_exhaustive]` / sealed | Feature | Semver promise |
| --- | --- | --- | --- | --- |
| `happenstance` crate-root docs | public (rendered) | n/a | default | None. Documentation is not API; rewording carries no semver obligation. But the fence is compiled and executed by the gate, so it cannot rot silently — which is the only promise it needs to make. |
| `overview.md` | file in a `publish = false` crate | n/a | n/a | None. Never packaged. Its stability obligation is byte-identity across its two renders, and that is structural. |
| `tokio` dev-dependency on `happenstance` | `[dev-dependencies]` | n/a | `macros`, `rt` | None to consumers — dev-dependencies are not resolved by downstreams. No MSRV impact: `tokio` declares `rust-version = "1.85"`, below this workspace's 1.97.1 floor (ADR-0029). |
| Pages in the pinned tree | public (rendered) | n/a | TBD-HS-P0020 | None. They are documentation. |

Nothing here is `pub` because nobody decided otherwise, for the simple reason that nothing
here is `pub` at all. The corresponding failure in this medium — a page that exists because
nobody decided it should not — is guarded by the answered-need discipline (AC-012) and by the
decision above **not** to add an index page in front of the three steps.

---

## What it costs a caller

- **One dev-dependency** on `happenstance` (`tokio`, `macros` + `rt`). Zero cost to a
  consumer; a small cost to a fresh `cargo test` in this workspace, where `tokio` is already
  compiled for the example and the testkit.
- **Both port flavours.** The fences bind `EventStore`, never `SendEventStore` — the weaker
  requirement, which accepts both flavours (CLAUDE.md binding constraint 4). Only one of the
  two names is imported per fence, because both in scope makes method calls ambiguous. A
  fence that bound `SendEventStore` would teach the application author the bound that closes
  off `wasm32`, which is the opposite of what the two-trait design is for.
- **No allocation, no lifetime, no object-safety change.** No API is touched.
- **A reader's time, budgeted.** Three steps, each one screen at 1024×768. The initiative's
  own non-goal is "Volume. Pages written is not the measure", and the density budget above is
  what that non-goal looks like in numbers.
- **Two copies of one ~20-line program**, forever, until HS-P0020's mechanism makes one
  possible. Costed and justified under `## Placement and re-export`.

---

## What a user meets first

**On docs.rs: the crate summary, then the answered-need line, then — before anything else
with code in it — a program that refuses an append.** That single reordering is the project.

**In the narrative tree: step one**, which appends two events and reads them back, and which
names four types. Not step three, and not the bridge.

**What is deliberately not on the front page.** The planned-surface roadmap (`Codec`,
`DomainEvent`, `DecisionModel`, the command loop, the typed projection runner) moves below
the fence — a reader who does not yet believe the library does anything is not served by a
list of what it will do. The bridge is one link away, not inlined. The worked example is two
links away, behind the bridge, because the reader who has not seen a refusal has no use for
three invariants that do not fit an aggregate. And the specification is nowhere in the
reading path — clause citations are provenance, and IQ-1's falsification is that striking
every off-page link leaves the teaching complete.

---

## The states the API must express

No API is added, so the enumeration is of the states the *material* must make representable
— which is the same discipline applied to the same risk (a state discovered late, as a gap
in a page rather than a missing enum variant):

- **Boundary held, append accepted** — step 2.
- **Boundary held, append refused** — step 3, and the whole point.
- **Boundary absent where prose claims one** — must be representable so the page can say why
  an empty tag set is correct where it genuinely is (DR-02, AC-006); nowhere in this
  project's material is it correct, so the answer is that no such construction appears.
- **Boundary present but wrong** — the type-only guard on a cross-entity invariant. Made
  representable by DT-6's compiled contrast, and by CF-7.
- **Boundary removed** — the falsification drill's middle state, in which a repository check
  fails.
- **Boundary restored** — the drill's end state, in which it passes and the reader's tree is
  clean. Reversibility is a state, not a footnote.
- **Provisional shape** — VT-30 is `[PROVISIONAL]`, so the material must be able to say
  "one or more guards, each with its own boundary" without implying the shape is frozen.

---

## Anti-patterns

Concrete forbidden moves. Every one is phrased so a person who cannot read Rust can check it
against a screenshot of the rendered page.

1. **A code block visibly labelled as not compiled, not checked, `ignore`, or exempt.** This
   project ships zero. If one appears on any of these four surfaces, DT-6 was re-litigated
   without a new decision.
2. **A square-bracket pair around a code-styled word in body text** — e.g. a literal
   `[happenstance_core]` rather than a link. It means an intra-doc link did not resolve. Four
   exist on the crate root today; the target is zero on every page this project touches.
3. **A collapsed disclosure triangle anywhere in this project's content.** If a reader must
   click before they can see a `Query`, a `Tags`, an `AppendCondition`, an assertion, a
   `MUST`, or an invariant, the page is defective.
4. **A code block with a horizontal scrollbar at 1024×768.** Over 68 columns.
5. **A sidebar table-of-contents entry clipped with an ellipsis.** The `##` heading is over
   22 characters and rustdoc has truncated it mid-phrase. *Reworded at the design gate on
   2026-08-17 (finding F-2): this previously read "wraps onto a second line", which
   `white-space:nowrap` makes physically impossible — a check no page could ever fail, and
   therefore decorative by CLAUDE.md's own test. Truncation is the failure that can actually
   occur, and it is visible on a screenshot.*
6. **The words "aggregate", "your aggregates", "one stream per entity" or "which stream" on
   the crate root or on any step of the opening encounter.** The prior model is named on
   exactly one page — the bridge's "Where your streams went" — and nowhere else.
7. **The wrong-model code block appearing last on its page, or above the correct version.**
   Scroll to the bottom of the bridge: the last code you see must be the correct guard, and
   the block marked wrong must have a marker at its top *and* at its bottom.
8. **Any tab strip, accordion, breadcrumb bar, navigation rail, banner, badge or button that
   is not part of rustdoc's own chrome.** Compare against a screenshot of any other page on
   the same site: if this project's page has a control the others do not, it is forbidden.
9. **A step page whose first element under the heading is not the two-line step header
   block.** Open any step's anchor cold: if you cannot tell within one screen which step you
   are on and how to reach step one in one click, the mid-sequence mitigation is missing.
10. **Prose claiming a refusal where the code block's own printed output does not contain
    `ConditionViolated`.** Cover every sentence around the fence with your hand; the refusal
    must still be on the screen.
11. **A sentence containing "MUST" or "MUST NOT" that is not a link to a clause id.** A page
    that states a normative rule in its own words has become a second, weaker specification.
12. **Two answered-need statements on one page, or one that appears below the first code
    block.** Exactly one, and it is above everything.
13. **A code block whose import line reads `use happenstance_core::`** on any page this
    project authors. The taught vocabulary is `use happenstance::{…}` (ADR-0006). The one
    permitted appearance of the other crate's name is the single vocabulary-seam sentence on
    the handoff page, and it is prose, not a fence.
14. **A diagram, chart, or image.** DT-5 resolved to narration; none ships from this project.
15. **A step whose program prints nothing.** Every fence's output block must have content.

Standing, and never re-litigated here: no `#[async_trait]`; no `serde` in
`happenstance-core`'s defaults; `read` returns the stream at the top level; generic code binds
`EventStore`, not `SendEventStore`; no `unwrap`/`expect` in library code — and, in a doctest,
no `no_run` and no `ignore` on the boundary-refusal fence, which would let it type-check
forever without ever being asked to refuse anything.

---

## Gaps in the substrate

Named rather than assumed into existence, per the grounding step. Each is a dependency this
project consumes and does not build, and each has a route.

1. **The pinned narrative tree does not exist.** No path constant is in `xtask/src/` today
   and HS-P0020 is at stage `briefs` with no signed-off `_design.md`. Three surfaces' routes
   are `{tree}/…` for that reason. Nothing in this design depends on *which* path is pinned —
   only that one is. → HS-P0020 (AC-001 there).
2. **The hosting/render shape is undecided** (dossier tension 8; HS-P0020's DT-7). Every
   selector for surfaces 2–4 is verified for the rustdoc render shape only. If a different
   renderer is pinned, the selectors are re-resolved; the composition, hierarchy and
   anti-patterns above are renderer-independent and survive. The one number that would move
   is the 22-character heading budget, which is rustdoc's 200px sidebar. → HS-P0020.
3. **HS-P0021's answered-need notation does not exist.** This design fixes the notation's
   *position* (above everything, above the first fence) and its *cardinality* (exactly one)
   because those are composition decisions. Its *form* is HS-P0021's and is consumed as
   given; inventing a second notation is a defect (DR-14). → HS-P0021.
4. **The compiled-fence REQUIRED step and the enumerated allowance list do not exist.** DT-6
   is resolved so that this project needs the allowance list for nothing at all, which
   removes one coupling. The compiled-fence step is still required for the tree's fences. If
   it turns out to type-check without *executing* doctests, AC-005's proof moves into this
   project's own `#[tokio::test]` rather than weakening. → HS-P0020.
5. **No automated check exists for the content-level accessibility floor** — one `h1`, no
   skipped heading levels, meaningful link text, copy-faithful fences. All four are checkable
   on page source and none is checked by any tool today. This design does not invent one;
   items 3, 5, 9 and 12 of `## Anti-patterns` are the reviewer-run substitute, and the gap is
   recorded here so it is not mistaken for coverage. → recorded; a lint for it is HS-P0021's
   natural home (their AC-008 already requires a lint with a named wrong page).
6. **Four unresolved intra-doc references render as literal brackets on the crate root today,
   and `cargo doc` did not warn.** Found while measuring, not while looking for it. It is
   inside this project's own rewrite scope (anti-pattern 2), so it is fixed here rather than
   routed — but the fact that the gate did not catch it is a substrate observation.
   → HS-P0020, as an input to what its documentation step must deny.

---

## The doctest

The runnable example a user would copy is written out under `## Signatures` above, in the
shape that binds and with its hidden lines shown so the UX-003 audit can be performed against
this file. It is not repeated here.

**One `compile_fail` case is deliberately *not* used.** The wrong-side contrast would be the
obvious candidate — pin the bad guard with `compile_fail` and let the compiler enforce the
lesson. It is rejected because a type-only guard **compiles perfectly well**; that is the
entire danger CF-7 describes. A `compile_fail` fence would teach the reader that the compiler
catches this, which is precisely the false comfort Persona 1's stated fear is about: "a mental
model that looks right, compiles, runs, and is quietly wrong". The fence therefore compiles,
runs, and asserts the wrong outcome — the only shape that tells the truth.

---

## Mock

| Mock | Path | Viewports | Themes | Notes |
| --- | --- | --- | --- | --- |
| Static sign-off mock, 41 frames | `.bklg/docs-that-teach/application-author-path/design/mock.html` | 1440×900, 1024×768, 700×900 | light | Self-contained: `normalize-9960930a.css` and `rustdoc-17e0aaed.css` inlined verbatim, thirteen `woff2` faces embedded as `data:` URIs, **zero network requests**. Open it from the file system. |

**It is a rendered rustdoc page, which is the only screen this project has.**
`design.capture` is absent and no app exists, so nothing will screenshot these surfaces
automatically. The substitute is this page: every frame is rustdoc's own markup, in rustdoc's
own classes, under rustdoc's own stylesheet as `cargo doc -p happenstance --no-deps` emitted it
on the pinned 1.97.1 toolchain. Nothing is approximated with ad-hoc CSS. The only stylesheet of
the page's own is frame chrome — captions, the fold rule, the dashed annotation boxes — and it
is delimited by a comment and built from rustdoc's own custom properties, so it cannot be
mistaken for a style these pages would ship. The surfaces still introduce no CSS and no
JavaScript (UX-012).

### Frames

| Surface | States rendered | Viewports |
| --- | --- | --- |
| `crate-root-encounter` | `default`, `refusal-rendered`, `hidden-line-audit`, `no-unresolved-brackets` | 1440×900, 1024×768 |
| `crate-root-encounter` | `narrow-700`; plus `baseline-today` (not a declared state — today's render, at 1440 only) | 700×900; 1440×900 |
| `opening-encounter` | `default`, `step-one`, `step-two-cold`, `step-three-cold`, `refusal-rendered`, `falsification-drill` | 1440×900, 1024×768 |
| `opening-encounter` | `narrow-700` | 700×900 |
| `conceptual-bridge` | `default`, `anchor-note`, `mapping-table`, `wrong-side-contrast`, `overflow-fence` | 1440×900, 1024×768 |
| `conceptual-bridge` | `narrow-700` | 700×900 |
| `worked-example-handoff` | `default`, `surfaced-verbatim`, `vocabulary-seam` | 1440×900, 1024×768 |
| `worked-example-handoff` | `narrow-700` | 700×900 |

`narrow-700` is drawn once per surface at 700×900 rather than at the two required viewports,
because it is a state defined by a breakpoint rather than by a size a reviewer chose; the
`@media (max-width:700px)` declarations are transplanted verbatim into a scoped block, since a
media query reads the browser viewport and cannot see a 700px-wide frame. Two deviations are
forced by that and are labelled in the stylesheet: `position:fixed` becomes `absolute`, and
`calc(100vh - …)` becomes `calc(100% - …)`.

### Reference captures the design review will use

| Surface | 1440×900 | 1024×768 |
| --- | --- | --- |
| `crate-root-encounter` | `.bklg/docs-that-teach/application-author-path/design/reference/crate-root-encounter@1440x900.png` | `.bklg/docs-that-teach/application-author-path/design/reference/crate-root-encounter@1024x768.png` |
| `opening-encounter` | `.bklg/docs-that-teach/application-author-path/design/reference/opening-encounter@1440x900.png` | `.bklg/docs-that-teach/application-author-path/design/reference/opening-encounter@1024x768.png` |
| `conceptual-bridge` | `.bklg/docs-that-teach/application-author-path/design/reference/conceptual-bridge@1440x900.png` | `.bklg/docs-that-teach/application-author-path/design/reference/conceptual-bridge@1024x768.png` |
| `worked-example-handoff` | `.bklg/docs-that-teach/application-author-path/design/reference/worked-example-handoff@1440x900.png` | `.bklg/docs-that-teach/application-author-path/design/reference/worked-example-handoff@1024x768.png` |

Each is the surface's `default` state. The narrow renders capture to
`.bklg/docs-that-teach/application-author-path/design/reference/<surface>@700x900.png`.

### What building it found

Seven things, each of which only exists once the real stylesheet and the real API are in the
room, and each of which is a decision the approver now owns. They are written out in full in
the mock's own **What this mock found** panel; in brief:

| # | Finding | Bears on |
| --- | --- | --- |
| F-1 | Code renders at **16px/24px**, not 14px/21px — the cited `code{font-size:0.875rem}` is `.item-info code`, and `normalize` sets `code,pre{font-size:1em}`. `.content{margin:0.25em 0.5em}` and `.docblock{margin-left:24px}` were also missed. Recomputed, the fence interior is 892px / 696px and holds **92 / 72 columns**, not 84–87; a 24-line fence is 604px, not 532px. | `## Density budget` |
| F-2 | Sidebar TOC entries **truncate with an ellipsis** — `.sidebar-elems .block li a{white-space:nowrap;text-overflow:ellipsis;overflow:hidden}` — they cannot wrap, so anti-pattern 5 as worded can never fire. The 22-character figure also assumes 16px, but `.sidebar{font-size:0.875rem}`. Eleven of the sixteen headings named in this file are over 22 characters. | `## Anti-patterns` 5, `## States` (long label), `## Composition` |
| F-3 | The payoff line is 92 characters of `Debug`: `Err(ConditionViolated(ConditionViolated { conflicting_position: … }))`. It scrolls at every viewport and buries the word the project turns on. | `## States` (error — the taught one) |
| F-4 | Written against the real API, the refusal program is **31 rendered lines**. Exactly one line (`Ok(())`) is eligible to be hidden, so hiding cannot reach 24. | `## Density budget`, `## Transience policy` |
| F-5 | The fence in `## Signatures` calls `store.head(&seats)` and `Guard::new(seats, upto)`. `EventStore::head` takes no argument; `Guard` is `#[non_exhaustive]` with no constructor; `AppendCondition::new` takes a `Query`. Every fence in the mock uses the verified spelling instead. | `## Signatures` |
| F-6 | DT-6's contrast asserts that a **type-only** guard is *accepted*. CF-7's named wrong implementation drops the tag join and therefore **over**-rejects — it "rejects every command touching any course". The mock draws the guard that really does under-refuse: one tagged with the row being written rather than with the invariant being held. Still one expression different; still compiles, runs and passes. | `## Pattern decision` (DT-6), `## Signatures` |
| F-7 | Three of the four surfaces are drawn inside rustdoc's chrome because it is the only render shape this repository has. If HS-P0020 pins a different renderer, only their composition survives. | `## Gaps in the substrate` 2 |

### Dispositions — all seven folded back

**Recorded at the design gate on 2026-08-17.** This project was rejected once at this gate
precisely because a `## Mock` section listed findings the binding sections above it still
contradicted. Every finding below is now reconciled in the binding text; none is left for an
implementer or an approver to rediscover.

| # | Disposition |
| --- | --- |
| F-1 | **Corrected.** `## Density budget` now records code type at **16px/24px** and the fence interior at **892px / 696px**. The fence-width budget is re-derived to **68 columns** (72 is where `overflow-x` actually engages at 1024×768), replacing the 84 that rested on the wrong type size. Anti-pattern 4 and the `Overflow — width` state updated to match. |
| F-2 | **Reworded, and the rule kept.** Sidebar entries **truncate with an ellipsis**; `white-space:nowrap` makes wrapping impossible, so anti-pattern 5 as written could never fire — a decorative check by CLAUDE.md's own test. It now names truncation, which is real, screenshot-visible, and affects eleven of the sixteen headings this design lists. The 22-character budget stays: a heading silently cut mid-phrase in the page's only navigation is a defect a reader meets. |
| F-3 | **Carried to `## States`** as the taught error, unchanged in substance: the 92-character `Debug` payoff line scrolls at every viewport and buries `ConditionViolated`. The output block, not the `Debug` string, is what must carry the word — already the rule in `## Composition` item 5. |
| F-4 | **Resolved by exempting one surface, with the reason stated.** The crate-root refusal program is 31 rendered lines against the real API, with one line eligible for hiding, so a 24-line ceiling there forbids the program the design also specifies — a contradiction, not a tight budget. The ceiling is 32 on `crate-root-encounter` alone; step pages keep 24. |
| F-5 | **Corrected.** `## Signatures` now uses the verified spelling `read_decision_model(&store, &seats)` + `AppendCondition::new(seats).after_opt(upto)`. The old sketch called `EventStore::head` with an argument (it takes none, `store.rs:248`) and `Guard::new` (no constructor; `#[non_exhaustive]`, `append.rs:132,237`). "The shape is binding" is now true of a shape the API can express. |
| F-6 | **DT-6 re-decided, not re-worded.** This was the blocking defect. The wrong side is a guard **tagged too narrowly** — scoped to what the command writes rather than to the invariant it holds — so the conflicting event never enters the guard's query and the append is accepted. A type-only guard is the *opposite* failure: CF-7 (`spec/SPECIFICATION.md:7266`, quoted at `:7286-7288`) says dropping the tag join makes the query broader and "rejects every command touching any course", so it over-refuses and would have made the fence's `is_ok()` assertion fail. |
| F-7 | **No action; tracked.** Recorded in `## Gaps in the substrate` — if HS-P0020 pins a renderer other than rustdoc, only these surfaces' composition survives, not their chrome. HS-P0020's approved design resolves this to "the markdown is the render", so the step-page surfaces are markdown and only `crate-root-encounter` is genuinely rustdoc-framed. |

As with HS-P0020, **`design/mock.html` deliberately still shows the pre-correction figures**
and must not be synced to the table above: it is the instrument that produced these findings,
and editing it would delete the evidence they rest on.

The mock also carries a **column ruler** in the `overflow-fence` frame, marking columns 72, 84
and 100 against a real fence, so F-1's number can be read off the render rather than argued
from arithmetic. `cargo doc -p happenstance --no-deps` remains the cheapest way to re-take any
of these measurements after a change.

---

## Sign-off

**Pending.** No page in this project may be authored until this file is signed off (AC-001,
project DoD item 1) and until the merge forward from
`initiative/from-contract-to-published-library` is complete and recorded (AC-014, DR-13).

| Approver | Date | Conditions |
| --- | --- | --- |
| Ryan Britton (repository owner) | 2026-08-17 | Approved on the second pass. **Two conditions, both blocking on other work rather than on this file:** (1) no page in this project may be authored until the merge forward from `initiative/from-contract-to-published-library` is complete and recorded (AC-014, DR-13) — `crates/happenstance/src/lib.rs` is 76 lines here and 237 there, and authoring against the stale copy means redoing the work at merge; (2) `crate-root-encounter` is rustdoc-framed, but the three step-page surfaces render as markdown under HS-P0020's approved "the markdown is the render" decision, so their chrome assumptions do not carry. Two calls were made on the approver's behalf and are recorded rather than buried: **DT-6's wrong side is a too-narrowly-tagged guard that under-refuses**, not the type-only guard that over-refuses (CF-7, `spec/SPECIFICATION.md:7286-7288`); and **F-4 relaxed the crate-root fence ceiling to 32 lines** rather than shrinking a 31-line program the same document specifies. |

**This project was rejected once at this gate** (2026-08-17) because DT-6's resolution was
backward against CF-7, `## Signatures` called two methods that do not exist, and the density
budget asserted figures its own mock had disproved — including an anti-pattern describing a
failure `white-space:nowrap` makes physically impossible. A re-run of the design workflow
reproduced the same shape: its mock re-derived the corrections and its binding sections again
did not carry them, and its mock agent reported success without rewriting `design/mock.html`.
The seven findings were therefore folded back by hand, and the dispositions table under
`## Mock` records each one. That table, not this row, is the durable record of what changed.

Recorded via `redkiln advance HS-P0022 --verdict approved --stay --apply`.

Conditions the approver should weigh explicitly, because each is a place this design commits
where it could have deferred:

- DT-6 resolves to **zero** entries on HS-P0020's allowance list. That is a stronger
  commitment than AC-003 requires and it removes a dependency; it also means that if a fence
  turns out to be genuinely unwritable as compiled code, this design must be reopened rather
  than an exemption written.
- The extraction of `examples/course-subscriptions/src/main.rs:1-19` into `overview.md` edits
  a file the UX brief says not to edit, in a way that changes no assertion it makes. It buys
  AC-010's verbatim bar by construction and costs a set of stale line citations in planning
  artifacts.
- Step 3's program exists twice, on purpose, with a stated reason the compiler cannot fix
  today.
