---
item: "HS-S0184"
stage: implement
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Resolution record — DT-1, DT-4 and DT-5+DT-6, certified against the merged tree

`_design.md` was approved on 2026-08-17 and carries all four resolutions. This record is not a
restatement of that approval; it is the three things the approval does not carry, and each one
would otherwise be hit mid-authoring, one story at a time:

1. **A probe that could have failed, run for each resolution.** A tier-5 story whose evidence
   is "the design says so" is the decorative check CLAUDE.md names. Every certification below
   names a probe, and the probe's transcript is pasted, not summarised.
2. **The values the design mandates and leaves blank** — final heading texts, the emitted
   fragment ids, and which surfaces the 22-character budget actually binds.
3. **A per-resolution status for every sign-off condition and every merge disposition**, so
   nothing found is absorbed silently (project DoD item 9).

**Nothing here re-decides.** Where a measurement contradicts a binding statement, it is
recorded with a status and routed. `_design.md` is byte-identical to its signed-off state.

**The probes ran outside the repository**, in a scratch crate that depends on
`crates/happenstance` by path (EC-008, NF-006). Nothing landed in the tree: the three fences
are compiled *and executed* as doctests, the geometry is read off rustdoc's own render of
them, and only the transcripts are committed. To rebuild it: a `[workspace]`-isolated crate
outside the repository with `happenstance = { path = "…/crates/happenstance", features =
["std", "memory", "json"] }` and a `tokio` dev-dependency, then `cargo test --doc` and
`cargo doc --no-deps`.

This record consumes `merge-forward-preflight/_baseline.md` rather than re-running its
archaeology: `§ Anchors` supplies every clause line cited below, `§ Dispositions` supplies the
merge contradictions this record statuses, and `§ Composition baseline` supplies the crate
root's re-measured figures.

## § DT-1

**The decision, as `_design.md:81-120` records it.** Invariant-first — formally option (c),
bounded by a **single located use** of option (b), the stream-per-entity prior, named at the
one moment the reflex question *which stream does this go in?* arises. It was decided on
frequency evidence, not taste: EventStoreDB "presents stream-per-entity as an unmarked
default", making that the more common prior rather than "aggregates are the old model"
(`interaction-patterns.md:547-566`). "Explicitly none" was treated as a real answer and
rejected on its merits, which is DR-06's own requirement (`project.md:191-193`).

**The one reader-facing home.** A heading on the **`conceptual-bridge`** surface —
**`Where your streams went`** — not this file and not `_design.md`. This file is the
decision's provenance; that heading is its single reader-facing statement. Its emitted
fragment id is **`#where-your-streams-went`**, confirmed by render rather than assumed
(§ Anchor table row 10). Every other page **links** it and does not re-argue it.

**Probe: the anti-pattern 6 baseline.** Anti-pattern 6 forbids "aggregate", "your
aggregates", "one stream per entity" and "which stream" on the crate root and on any step of
the opening encounter (`_design.md:852-854`). The baseline that later audit runs against, taken
on the merged tree:

```
$ git grep -n -i -E "aggregate|one stream per entity|which stream" -- crates/happenstance/src/lib.rs
$                                                            # no match: 0
$ git grep -n -i -E "aggregate|one stream per entity|which stream" -- docs/
$                                                            # no match: 0
$ git grep -n -i -E "aggregate|one stream per entity|which stream" -- examples/course-subscriptions/src/main.rs
examples/course-subscriptions/src/main.rs:3://! Three invariants, none of which fits inside a single aggregate:
examples/course-subscriptions/src/main.rs:11://! make `Course` the aggregate and invariant 3 needs a read model plus a saga,
examples/course-subscriptions/src/main.rs:12://! or make the pair the aggregate and invariant 2 has nowhere to live.
                                                             # 3 occurrences
```

Zero on `crates/happenstance/src/lib.rs`. Zero across the pinned narrative tree `docs/`.
**Three** in the worked example's module doc — and that is a finding rather than a violation,
because it is worth being precise about: anti-pattern 6's scope is the crate root and the
opening encounter's steps, and the worked example's doc lands on `worked-example-handoff`,
which is neither. But AC-010 requires that doc **surfaced verbatim**, so the word DT-1 spends
its single budget on will also appear three times on the handoff page, outside the anchor
DT-1 fixed. Statused in `§ Conditions and dispositions` and routed to
`surface-course-subscriptions`; not resolved here, because resolving it means either editing
the example's prose (which the UX brief forbids) or paraphrasing it (which AC-010 forbids).

**Verdict: `holds`.** One decision, one location, one prior named once; the anchor string is
confirmed against a render; the baseline is zero on both surfaces the anti-pattern governs.

## § DT-4

**The decision, as `_design.md:122-168` records it.** Staged, minimal-first: three steps, each
a complete runnable program, with no fourth "whole program" artifact. Evidence is Carroll's
training-wheels finding — users constrained to a core feature set built better mental models,
*measured after the constraint was lifted* (`interaction-patterns.md:144-159`).

**The mitigation is the decision, and all three parts are certified individually** — the
literature's named failure mode is a reader arriving mid-sequence from search
(`interaction-patterns.md:161-166`), and DT-4 answers it structurally rather than with a
warning sentence:

1. **No step is a fragment.** Each step's fence is a complete program. The steps are
   cumulative *in teaching* and never *in execution*, which removes the interdependence the
   failure mode is about rather than warning against it. **Certified by measurement**: step 3
   below is a whole program — `use` block, `#[tokio::main]`, `main`, `Ok(())` — with nothing
   hidden, and it runs.
2. **An in-body step header block**, two lines, not a sidebar affordance, because rustdoc's
   `nav.sidebar` is removed entirely below 700px (`_design.md:394`, `:521`) and a mitigation
   that disappears on a phone is not one. This survives the merge *more* strongly than it was
   argued: the three step pages are markdown under HS-P0020's landed "the markdown is the
   render" decision, and a markdown page has no sidebar at any width, so an in-body block is
   now the only place the signal can live at all.
3. **The payoff sits in the step most likely to be landed on cold.** Step 3 is the refusal, so
   cold arrival becomes the best available landing rather than a signposted loss.

**Per-step composition, the order the page must render** (`_design.md:454-471`, seven
elements): heading → step header block (two lines) → one or two setup sentences → the fence →
the output block → the clause citation. Step 3 additionally carries the falsification drill as
its own `###` after the citation. An eighth element means something is cut.

**Probe: step 3's program, written against the merged API and measured off a render.** The
program is `A condition that refuses` — a decision reads a boundary, someone else appends
first, the guarded append is refused, and the program prints
`AppendError::ConditionViolated`. Compiled and executed as a doctest; rendered by rustdoc on
the pinned 1.97.1 toolchain; measured from the DOM, not from arithmetic:

```
     1  19  use happenstance::{
     2  52      AppendCondition, AppendError, Event, EventStore,
     3  66      MemoryEventStore, Query, QueryItem, Tags, read_decision_model,
     4   2  };
     5   0
     6  14  #[tokio::main]
     7  60  async fn main() -> Result<(), Box<dyn core::error::Error>> {
     8  40      let store = MemoryEventStore::new();
     9  53      let held = Tags::from_pairs([("course", "c1")])?;
    10  59      let item = QueryItem::new(["SeatHeld"], held.clone())?;
    11  43      let seats = Query::from_items([item])?;
    12   0
    13  68      let (_taken, upto) = read_decision_model(&store, &seats).await?;
    14  67      let seat = Event::new("SeatHeld", &b"{}"[..])?.with_tags(held);
    15  47      store.append(&[seat.clone()], None).await?;
    16   0
    17  64      let condition = AppendCondition::new(seats).after_opt(upto);
    18  64      let refused = store.append(&[seat], Some(&condition)).await;
    19   0
    20  28      println!("{refused:?}");
    21  29      assert!(matches!(refused,
    22  49          Err(AppendError::ConditionViolated(_))));
    23  10      Ok(())
    24   1  }

rendered lines: 24    widest line: 68 columns    hidden lines: 0

$ cargo run --quiet --example refuse
Err(ConditionViolated(ConditionViolated { conflicting_position: Some(SequencePosition(1)) }))
```

**24 rendered lines against a 24-line step budget; 68 columns against a 68-column budget.**
Both exactly at the ceiling, with zero headroom and nothing hidden — so the fence satisfies
the transience policy (`_design.md:524`) trivially: there is no hidden line to audit, and every
line that builds a `Query`, `Tags`, an `AppendCondition`, the read call, the append call and
the assertion is visible.

Finding F-4 is what makes this measurement non-decorative rather than a formality: the
crate-root program measured **31** rendered lines against the same ceiling and forced an
exemption to 32 (`_design.md:1015`). The same question asked of a step page had a different
answer, and it had to be asked to find that out.

**One number worth carrying to the page.** The printed payoff line is **93 characters** of
`Debug` — `Err(ConditionViolated(ConditionViolated { conflicting_position:
Some(SequencePosition(1)) }))` — which scrolls at every viewport and buries the word the whole
project turns on. F-3 measured 92 pre-merge; it is 93 here. `## Composition` item 5 already
requires the **output block**, not the `Debug` string, to carry the word, and step 3's page
must honour that: the block under the fence says `ConditionViolated` in a form a reader can
see without scrolling.

**Verdict: `holds`, measured with zero headroom.** DT-4's premise — three steps, each a
complete runnable program — and the density budget are **not** in tension on a step page.
EC-003 did not fire. The zero headroom is itself the finding to carry: any line added to step
3's program breaks one of the two budgets, so `boundary-refusal-encounter` should treat this
program as the shape rather than as a starting sketch.

## § DT-5+DT-6

One joint resolution, as project AC-003's own wording requires (`project.md:239-241`,
`_design.md:170-262`).

**DT-5, as recorded.** Narrate the cycle: a three-column mapping table in a fixed column order
— `| Your words | This library's words | Where you saw it |`, **three columns, never four**,
because four give ~180px at 764px, narrower than the terms being mapped
(`_design.md:592-595`) — plus a four-step numbered narration using the same four names
everywhere in the page set: **tag, query, fold, guard**. Synonym drift across pages is how a
narration loses to a diagram; a fixed vocabulary is the cheap fix. **No diagram ships from this
project**, and **AC-013 is discharged explicitly** — recorded here, in writing, that the
project went that way — rather than vacuously by nobody mentioning it (`_storymap.md:120`).
Diagramming was rejected on three costs: it is original work nobody in the field has completed,
it needs a renderer with zero precedent in this workspace, and it is "prose the gate can render
but not typecheck" (`interaction-patterns.md:296-307`, `:520-526`).

**DT-6, as recorded.** Real, compiled, executed code in the narrowest form the contrast can
take: one fence differing from the correct one by a single expression — a guard **tagged too
narrowly**, scoped to what the command *writes* rather than to the invariant it must *hold* —
asserting the outcome the reader should fear, that the append is **accepted** and the invariant
is violated with no error. This project puts **zero** entries on HS-P0020's enumerated
allowance list and ships zero uncompiled fences, which removes a dependency and means an
unwritable fence **reopens the design** rather than buying an exemption (`_design.md:1055-1058`).

**This is the mirror of CF-7, not CF-7 itself.** CF-7 (`spec/SPECIFICATION.md:7670`,
`[FROZEN]`; line from `_baseline.md § Anchors`) names the **broadening** failure — an adapter
that drops the tag join from its condition probe matches more than it should and "rejects every
command touching any course" (`:7690`). That over-refuses, and an `is_ok()` assertion against
it would simply fail. The hazard worth teaching is the quiet one, and it is the opposite move.

**Probe, and it is the load-bearing one.** The wrong-side fence was written against the merged
API, compiled, and **run**. It imports from `happenstance` (ADR-0006), binds `EventStore` and
never `SendEventStore` (CLAUDE.md constraint 4, `_design.md:776-780`), hides nothing, and is
marked wrong at both ends:

```
     1  19  use happenstance::{
     2  39      AppendCondition, Event, EventStore,
     3  66      MemoryEventStore, Query, QueryItem, Tags, read_decision_model,
     4   2  };
     5   0
     6  14  #[tokio::main]
     7  60  async fn main() -> Result<(), Box<dyn core::error::Error>> {
     8  40      let store = MemoryEventStore::new();
     9  53      let held = Tags::from_pairs([("course", "c1")])?;
    10  64      // WRONG - tagged to the student, not to the course's seats.
    11  56      let narrow = Tags::from_pairs([("student", "s1")])?;
    12  53      let item = QueryItem::new(["SeatHeld"], narrow)?;
    13  43      let seats = Query::from_items([item])?;
    14  68      let (_taken, upto) = read_decision_model(&store, &seats).await?;
    15  68      let other = Event::new("SeatHeld", &b"{}"[..])?.with_tags(held);
    16  40      store.append(&[other], None).await?;
    17  64      let condition = AppendCondition::new(seats).after_opt(upto);
    18  51      let seat = Event::new("SeatHeld", &b"{}"[..])?;
    19  65      let accepted = store.append(&[seat], Some(&condition)).await;
    20  29      println!("{accepted:?}");
    21  65      assert!(accepted.is_ok(), "the narrow guard did not refuse");
    22  53      // END WRONG. The correct fence is the one above.
    23  10      Ok(())
    24   1  }

rendered lines: 24    widest line: 68 columns    hidden lines: 0

$ cargo test --doc
running 3 tests
test src\lib.rs - (line 8) ... ok
test src\lib.rs - minimal_wrong_side (line 83) ... ok
test src\lib.rs - (line 42) ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo run --quiet --example narrow_min
Ok(SequencePosition(2))
```

The contrast is only a contrast against its neighbour, so both halves are stated together. The
correct fence — § DT-4's step-3 program, one expression different: its guard query is tagged
`course=c1`, the invariant it must hold — **refuses** the same append on the same store:

```
$ cargo run --quiet --example refuse        # the correct guard
Err(ConditionViolated(ConditionViolated { conflicting_position: Some(SequencePosition(1)) }))

$ cargo run --quiet --example narrow_min    # the same program, guard tagged too narrowly
Ok(SequencePosition(2))
```

One expression apart, and `ConditionViolated` becomes `Ok`. That is the whole resolution, and
it is a result rather than a prediction.

**`assert!(accepted.is_ok())` PASSES.** The conflicting `SeatHeld` — tagged `course=c1`, the
seat someone else took — never enters a guard query tagged `student=s1`, so the store accepts
the append at position 2, capacity is exceeded, and nothing in the type system, the API or the
store objects. That is the failure a reader can actually ship, and it is exactly what the
correct fence's tag join exists to prevent. **EC-001 did not fire**; finding F-6's correction
is confirmed rather than re-broken.

**One reconciliation, and it is worth stating precisely.** The design's sketch of this fence
(`_design.md:373-384`) is five lines with an elision — `// ... same append, same condition
shape ...`. That shape cannot compile, and it cannot be made to compile by hiding the elided
lines, because they are the append call and the condition, both of which the transience policy
forbids hiding (`_design.md:524`). Written out honestly the fence is a whole program, and the
first honest draft measured **31 rendered lines and 75 columns** against the bridge page's
24 and 68. Reduced by the design's own yield order — comments cut to one marker line at each
end, the interior blank lines removed, the tag construction bound rather than inlined, and the
assertion message shortened — it lands at exactly **24 lines and 68 columns**, above.
Nothing was hidden, no marker was dropped, no budget was relaxed, and it still compiles, runs
and passes. `_design.md` is untouched; the reconciliation is recorded here.

**DT-6's zero-allowance commitment, verified rather than restated.** This project ships zero
uncompiled fences and puts zero entries on HS-P0020's enumerated allowance list. Every fence
in this record compiled and executed. **UX-015 is therefore recorded as vacuously satisfied,
with its reason** rather than omitted: it governs what an exempt fence must carry, and no
exempt fence exists — an obligation dropped because it did not fire is indistinguishable from
one forgotten.

**Verdict: `reconciled`, with the reconciliation stated.** DT-5 and DT-6 both hold as
decisions; DT-6's fence needed to be written as a complete program rather than as the design's
elided sketch, and at its minimum it fits both budgets exactly.

## § Anchor table

One row per `##` heading `_design.md`'s `## Composition` (`:408-504`) names, across all four
surfaces. Every fragment id below was **read off a render** — the same method `_design.md:110-112`
uses by example — never predicted: each heading text was put through rustdoc on the pinned
1.97.1 toolchain in the out-of-tree probe and the emitted `id="…"` copied out.

**The budget's scope, reconciled against sign-off condition 2** (`_design.md:1039`). The
22-character budget, the `Long label` state and anti-pattern 5 all derive from **rustdoc's
200px sidebar TOC** clipping with `white-space:nowrap; text-overflow:ellipsis`
(`_design.md:575-584`). Sign-off condition 2 records that only `crate-root-encounter` is
rustdoc-framed and the three step-page surfaces render as markdown, which the merge has since
made concrete: the narrative tree is `docs/`, pinned at `xtask/src/narrative.rs:124`, and
markdown pages have no 200px sidebar at any width. So the budget **binds
`crate-root-encounter` and does not bind the other three** — and the consequence for
anti-pattern 5 is that on a markdown surface it is a check no page can ever fail, which is
decorative by CLAUDE.md's own test. It is therefore **scoped to `crate-root-encounter`**, not
deleted and not left universal. The number 22 is unchanged; only the set of surfaces it
governs is stated, which the sign-off condition already implied and nobody had written down.

That reconciliation has a consequence worth naming: `Where your streams went` is 23 characters,
one over the budget, and it is the DT-1 anchor `_design.md:108-114` cites by string. Because
the budget does not bind `conceptual-bridge`, **no retitle is needed and the design's own cited
anchor survives intact.** Retitling it would have moved the one string every other page links.

| surface (heading level) | final heading text | chars | 22-char budget binds? | emitted fragment id | how the id was obtained |
| --- | --- | --- | --- | --- | --- |
| `crate-root-encounter` (`##`) | `Status: a facade over happenstance_core` | 39 | **yes** | `#status-a-facade-over-happenstance_core` | read off a rustdoc render. **Superseded**: the merged crate root no longer carries this section; ADR-0006's reasoning moved into `What arrives here, and what stays below` (`crates/happenstance/src/lib.rs:72-77`). Routed to `boundary-refusal-encounter` |
| `crate-root-encounter` (`##`) | `Watch a boundary refuse` | 23 | **yes** | `#watch-a-boundary-refuse` | read off a rustdoc render. **One over budget — retitled to the row below**, which is the retitle `_design.md:582-584` mandates and does not supply |
| `crate-root-encounter` (`##`) | `A boundary refuses` | 18 | **yes** | `#a-boundary-refuses` | read off a rustdoc render. **The supplied final title.** Keeps "boundary" and "refuses"; drops only the imperative "Watch a" |
| `crate-root-encounter` (`##`) | `What arrives here, and what stays below` | 39 | **yes** | `#what-arrives-here-and-what-stays-below` | read off a rustdoc render, and confirmed identical in the real `target/doc/happenstance/index.html`. **Over budget, and its content changed at the merge** — it is now the encoding discriminator, not the planned-surface roadmap the design describes. No retitle supplied here: choosing one is choosing what the section is for. Routed to `boundary-refusal-encounter` |
| `opening-encounter` step 1 (`##`) | `Append and read back` | 20 | no (markdown) | `#append-and-read-back` | read off a rustdoc render; **matches the anchor `_design.md:137` already fixed** |
| `opening-encounter` step 2 (`##`) | `A condition that holds` | 22 | no (markdown) | `#a-condition-that-holds` | read off a rustdoc render; matches `_design.md:138` |
| `opening-encounter` step 3 (`##`) | `A condition that refuses` | 24 | no (markdown) | `#a-condition-that-refuses` | read off a rustdoc render; matches `_design.md:139` |
| `opening-encounter` drill (`###` under step 3) | `Try it wrong, then put it back` | 30 | no (markdown) | `#try-it-wrong-then-put-it-back` | read off a rustdoc render. The falsification drill's own heading, which `_design.md:468-471` requires as a `###` and does not name |
| `conceptual-bridge` §2 (`##`) | `Your rule, in your words` | 24 | no (markdown) | `#your-rule-in-your-words` | read off a rustdoc render |
| `conceptual-bridge` §3 (`##`) | `Where your streams went` | 23 | no (markdown) | `#where-your-streams-went` | read off a rustdoc render. **The DT-1 anchor**, confirmed to be the exact string `_design.md:108-114` cites |
| `conceptual-bridge` §4 (`##`) | `Tag, query, fold, guard` | 23 | no (markdown) | `#tag-query-fold-guard` | read off a rustdoc render |
| `conceptual-bridge` §5 (`##`) | `The guard you would write` | 25 | no (markdown) | `#the-guard-you-would-write` | read off a rustdoc render |
| `conceptual-bridge` §6 (`##`) | `What a type-only guard misses` | 29 | no (markdown) | `#what-a-type-only-guard-misses` | read off a rustdoc render. **Contradicted by the design's own DT-6 correction**: F-6 re-decided the wrong side to a *too-narrowly-tagged* guard, and a type-only guard is the opposite failure. `## Composition` was not updated. Superseded by the row below |
| `conceptual-bridge` §6 (`##`) | `What a narrow guard misses` | 26 | no (markdown) | `#what-a-narrow-guard-misses` | read off a rustdoc render. **The supplied final title**, matching what DT-6 actually resolved to |
| `conceptual-bridge` §7 (`##`) | `Back to the working version` | 27 | no (markdown) | `#back-to-the-working-version` | read off a rustdoc render |

**`worked-example-handoff` has no `##` heading**, and that is stated rather than left as an
empty space in the table: `_design.md:493-503` composes it as five prose slots — the
answered-need line, two orientation sentences, the vocabulary-seam sentence, the surfaced
module doc, and the link — with no section headings of its own. The surfaced module doc brings
its own `#` heading (`What is not in this file, and used to be`) as part of the verbatim
material, which is the example's, not this project's.

**On the markdown surfaces, the id above is the rustdoc slug and the renderer is GitHub's.**
The two agree on every heading in this table, and the agreement is checked rather than assumed:
this repository already links markdown headings by fragment, and the slugging is verified by
example the same way `_design.md:110-112` verified rustdoc's. `standards/rust/00-prime-directives.md:158`
links `spec/SPECIFICATION.md#es-1--one-definition-two-flavours-and-generic-code-binds-the-weaker-one`,
and that file's `:2508` reads `#### ES-1 — One definition, two flavours, and generic code binds
the weaker one` — lowercased, punctuation dropped, spaces to hyphens, the em dash gone and its
two flanking spaces left as `--`. None of the headings above contains an em dash or a character
the two renderers treat differently, so no row is left provisional and EC-006 does not fire. A
heading added later that does contain one must be re-checked against that rule.

## § Consumption map

What each remaining story in this project loads from this record and from `_design.md`, so
that "each page cites the one place" (AC-009) is a check a reviewer runs against a table rather
than against a memory.

| story | `_design.md` sections binding it | anchor rows it renders or links | the one DT-1 citation string |
| --- | --- | --- | --- |
| `boundary-refusal-encounter` (HS-S0185) | `## Pattern decision` DT-4 (`:122-168`), `## Composition` crate root + steps (`:413-471`), `## Transience policy` (`:507-531`), `## Density budget` (`:532-620`), `## Hierarchy` (`:632-652`), anti-patterns 2, 4, 6, 8, 9, 10, 12, 13, 15 | **renders** `A boundary refuses`, `Append and read back`, `A condition that holds`, `A condition that refuses`; **links** `Where your streams went` — the first page to link the anchor before the page that renders it exists | `[Where your streams went](…#where-your-streams-went)` |
| `boundary-falsification-drill` (HS-S0186) | `## States` (error — the drilled one, `:689`), `## Composition` step 3's `###` (`:468-471`), `## Transience policy` falsification-drill row (`:527`) | **renders** `Try it wrong, then put it back` under `A condition that refuses` | not cited; the drill names no prior model |
| `invariant-to-appendcondition-bridge` (HS-S0187) | `## Pattern decision` DT-1 (`:81-120`) and DT-5+DT-6 (`:170-262`), `## Signatures` wrong-side fence (`:373-384`), `## Composition` bridge (`:473-491`), `## Hierarchy` bridge (`:654-665`), anti-patterns 1, 7, 11, 14 | **renders** `Your rule, in your words`, `Where your streams went`, `Tag, query, fold, guard`, `The guard you would write`, `What a narrow guard misses`, `Back to the working version` — in that reading order, §6 strictly after §5 | it **is** the anchor; every other page links to it |
| `surface-course-subscriptions` (HS-S0188) | `## Composition` handoff (`:493-503`), `## Placement and re-export` (`:703-751`), `## Hierarchy` handoff (`:667-674`), anti-pattern 13 | renders **no** row from the table above; owns the vocabulary-seam slot, whose premise the merge deleted, and the three `aggregate` occurrences the surfaced module doc brings with it | `[Where your streams went](…#where-your-streams-went)`, if it names a prior model at all |
| `fence-inventory-and-clause-audit` (HS-S0189) | `## Anti-patterns` (`:831-885`), `## Gaps in the substrate` (`:886-919`), `## The doctest` (`:923-935`) | audits every row; renders none | not cited |
| `answered-need-and-anchor-review` (HS-S0190) | `## Composition` all four surfaces (`:408-504`), anti-patterns 5 (as scoped above), 12 | checks that every rendered heading and every inbound link matches this table exactly, and that no page re-derives the DT-1 anchor | verifies the string on every page that uses it |

## § Conditions and dispositions

Three fields per row — what the design says, what the tree says, and a status of `holds`,
`reconciled` or `raised as reopen condition` — because anything longer starts re-arguing a
signed-off decision. Both sign-off conditions (`_design.md:1039`) and all four merge
dispositions from `_baseline.md § Dispositions` are here, plus what certifying found.

`_design.md` is **byte-identical** to its signed-off state
(`git diff --stat 3fd3866 HEAD -- .bklg/docs-that-teach/application-author-path/_design.md`
returns empty).

| # | what the design says | what the tree says | status |
| --- | --- | --- | --- |
| C1 | Sign-off condition 1: no page may be authored until the merge forward is complete and recorded (`_design.md:1039`) | merge `a5c0f30` landed, `cargo xtask ci --fast` green on its source, and the baseline is recorded at `merge-forward-preflight/_baseline.md` | holds — the condition is discharged, and this record's own probes ran against the merged API |
| C2 | Sign-off condition 2: `crate-root-encounter` is rustdoc-framed, the three step-page surfaces render as markdown, "so their chrome assumptions do not carry" (`_design.md:1039`) | the binding sections above still state rustdoc selectors for all four surfaces (`:39-72`) and a `##` budget derived entirely from rustdoc's 200px sidebar (`:575-584`). The merge makes the condition concrete: the tree is `docs/`, markdown, pinned at `xtask/src/narrative.rs:124` | reconciled — the 22-character budget and anti-pattern 5 are **scoped to `crate-root-encounter`** in § Anchor table, with the reason stated per surface. The number is unchanged; only the set of surfaces it governs is now written down |
| D1 | `tokio` is a dev-dependency this project **adds**, because the crate declares none (`_design.md:284-306`) | already present at `crates/happenstance/Cargo.toml:55` with `macros`, `rt`, `rt-multi-thread` | holds — for DT-4 and DT-6 the row's status is irrelevant and its *reason* is what binds: a fence must **execute**, not merely type-check. Every fence in this record executed |
| D2 | the vocabulary-seam sentence exists because the example imports `happenstance_core` (`_design.md:497-502`) | the example imports `happenstance` at `main.rs:34` | reconciled for this record's purposes — no probe or anchor row depends on the premise; the sentence's fate is `surface-course-subscriptions`', per `_baseline.md § Dispositions` row 2 |
| D3 | AC-010 surfaces the example's module doc `:1-19` (`_design.md:728-741`) | the module doc runs `:1-28` | holds — no resolution depends on the span; § Consumption map points `surface-course-subscriptions` at the merged one |
| D4 | the worked example carries no test target (`project.md:75-77`) | `tests/runs.rs`, `tests/ui.rs` and a `trybuild` dev-dependency exist | holds — it strengthens DT-6's zero-allowance position rather than weakening it: the example's own fences are now swept by the `"tests"` REQUIRED step (`xtask/src/main.rs:158`) |
| F1 | `## Composition` §6 names the bridge's wrong-side section `## What a type-only guard misses` (`_design.md:487-488`) | `## Pattern decision` DT-6 was re-decided at the design gate (finding F-6) to a **too-narrowly-tagged** guard, and a type-only guard is the *opposite*, over-refusing failure. `## Composition` was never updated, so two binding sections of the same signed-off file disagree | reconciled — this record supplies `What a narrow guard misses` (26 chars, `#what-a-narrow-guard-misses`) as §6's final title, which is the design's own *intent* under F-6. `_design.md` is not edited (EC-007); the disagreement is raised to the sign-off owner as a note on `invariant-to-appendcondition-bridge` |
| F2 | the wrong-side fence is sketched as five lines with an elision, `// ... same append, same condition shape ...` (`_design.md:373-384`), and must compile and run with zero allowance-list entries (`:206-208`) | written honestly it is a whole program; the elided lines are the append call and the condition, both forbidden to hide (`:524`). First honest draft: 31 lines, 75 columns. Reduced by the design's own yield order: **24 lines, 68 columns**, nothing hidden, both markers intact, compiles, runs, `is_ok()` passes | reconciled — the fence is written as a complete program at its minimum. No budget relaxed, no exemption written, no reopen needed |
| F3 | anti-pattern 6 forbids "aggregate" on the crate root and every step (`_design.md:852-854`); DT-1 names the prior model exactly once | zero on `crates/happenstance/src/lib.rs`, zero across `docs/`, **three** in `examples/course-subscriptions/src/main.rs:3,11,12` — inside the module doc AC-010 requires surfaced verbatim on `worked-example-handoff`, a surface anti-pattern 6 does not govern | reconciled — the anti-pattern is not violated, and the interaction is real: the word will appear three times on the handoff page, outside DT-1's anchor. Routed to `surface-course-subscriptions`, which cannot paraphrase (AC-010) or edit the example's prose (UX brief) and must therefore decide how the seam is framed |
| F4 | the payoff line is 92 characters of `Debug` and buries `ConditionViolated` (finding F-3, `_design.md:997`) | measured on the merged API: **93** characters — `Err(ConditionViolated(ConditionViolated { conflicting_position: Some(SequencePosition(1)) }))` | holds — one character worse, and the mitigation is unchanged: the output block, not the `Debug` string, carries the word (`## Composition` item 5) |
| F5 | step 3's program is "roughly 20 lines" and remains the binding case for the 24-line budget (`_design.md:568`) | measured: **24** rendered lines, **68** columns, zero hidden — exactly at both ceilings | holds, with zero headroom. Recorded as a constraint on `boundary-refusal-encounter` rather than as slack: one added line breaks one of the two budgets |

**Nothing is `raised as reopen condition`.** Every probe that could have failed passed, and the
two contradictions found (F1 and F2) are executions of the design's own stated intent rather
than departures from it. Had the wrong-side fence refused, or had it been unwritable, EC-001
and EC-002 would have made this a reopen and nothing downstream could have started on it.

**The story gate's own output, recorded rather than smoothed.**
`redkiln verify --grain story --item HS-S0184` at this story's checkpoint:

```
redkiln: verify HS-S0184 (story): FAIL
redkiln:   [ok]   affected-gate
redkiln:   [FAIL] boundary — changed outside declared boundary: <~230 paths>
redkiln:   [ok]   ledger
redkiln:   [FAIL] provenance — 6 file(s) changed inside this story's declared
                  boundary and links.commits is empty
```

Both failures are the slice-mate's EC-007, one story over: the branch carries merge `a5c0f30`,
so every file its **second parent** brings is reported as changed outside this story's fence,
which a glob cannot express. This story's authored diff is two files, both inside
`tension-resolutions/`, confirmed by `git status --porcelain`. `links.commits` is written by
`redkiln record-links` after the checkpoint exists, and the CLI is the only writer of item
frontmatter. Neither was answered by widening the fence. `[ok] ledger` is the check with
content here, and it is green.

## Re-deriving this record

```
$ git grep -n -i -E "aggregate|one stream per entity|which stream" -- crates/happenstance/src/lib.rs docs/ examples/course-subscriptions/src/main.rs
$ cd <a scratch crate outside the repository, depending on crates/happenstance by path>
$ cargo test --doc                 # all three fences compile AND run
$ cargo doc --no-deps              # then read line counts, widths and id="…" off the DOM
$ cargo run --quiet --example refuse
$ cargo run --quiet --example narrow_min
$ cargo xtask lints && cargo xtask spec-trace
$ cargo xtask affected --base main
```

Every clause line this record cites comes from `merge-forward-preflight/_baseline.md
§ Anchors`, id first and line second, and is not re-derived here — a second, competing set of
numbers is the exact defect that table exists to prevent.
