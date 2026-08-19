---
item: HS-S0038
stage: discover
created: 2026-08-12T13:02:01.493Z
updated: 2026-08-12T13:02:01.493Z
template_sig: 86ce4036
rendered_sig: d959802e
---

# Discover — The real read stream, lazy, paged, and snapshot-bounded

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: replace the read path's `todo!()`s with a still-lazy paged `SqliteReadStream` that captures ADR-0011's position ceiling no later than the first `poll_next`, composes it with `ReadOptions::to` and `backwards`, and keeps `resume_from` inclusive. | `_storymap.md`, *Slices* table, row `durable-event-store` / `lazy-read-with-snapshot-ceiling` | Four obligations, and the middle two are the ones a working implementation can omit without any test noticing. |
| **AC-001** — the suite runs, whole. This story's half is "read carries the largest rule family". | `project.md`, AC-001; `_storymap.md`, *Coverage* | Most of `for_each_event_store_rule!` is read-shaped: query semantics, read options, ordering, limits, isolation (`crates/happenstance-testkit/src/registry.rs:105-217`). |
| `dependsOn: schema-migration-and-identity` (HS-S0036) — it supplies the `event` table, `event_tag` keyed `(tag, position)` with the covering `event_type`, and the position ordering the page query walks. | `_storymap.md`, *Merge order* item 2 | The covering column is what keeps a type-constrained page query off a join back to `event`; without it the read path is correct and slow, which no rule can see. |
| ADR-0011: "an adapter issuing more than one statement per `read` must capture a position ceiling no later than the first poll and bound every later statement by it." | `.kb/decisions/0011-read-laziness-and-isolation.md` | This is **discharge of an accepted ADR, not a design choice** (`_decomposition.md`, *Architecture brief*, AC-A03). |
| `PAGE_SIZE = 512` makes this adapter unambiguously "more than one statement per read" for any log over 512 events — and `ReadCursor` has **no ceiling field** today. | `crates/happenstance-sqlite/src/event_store.rs:81`, `:309-328` | The field must be added. `_grounding.md` says so explicitly: "the architecture brief should treat adding it as required by ADR-0011, not as a design choice." |
| The ceiling composes with, and does not replace, `ReadOptions::to`: the effective upper bound is the tighter of the caller's `to` and the ceiling, and under `backwards` the ceiling bounds the *starting* end. | `_decomposition.md`, *Architecture brief*, §4 | `read_to_is_inclusive`, `read_from_and_to_bound_a_closed_window` and `read_to_under_backwards_bounds_the_older_end` (`crates/happenstance-testkit/src/registry.rs:133-135`) are what notice a confusion between the two bounds. |
| `resume_from` stays **inclusive**, and the skeleton's own doc records the historical bug: the field was `resume_after`, seeded from an inclusive `ReadOptions::from` and advanced to `last.position`, so every page boundary re-read a row. `AppendCondition`'s `after` is the exclusive one. | `crates/happenstance-sqlite/src/event_store.rs:313-323`, `:347-350` | "Do not 'tidy' the two into one sense" (`_decomposition.md`, *Architecture brief*, §4). The bug is invisible for any log that fits in one page. |
| `advance()` is already written and already correct for budget and direction; the ceiling is an extra `WHERE` term, not a change to its arithmetic. | `crates/happenstance-sqlite/src/event_store.rs:351-368`; `_decomposition.md`, *Architecture brief*, §4 | Reduces the diff and names what must **not** be rewritten. |
| ADR-0001 / ADR-0008 and `CLAUDE.md` constraint 3: `read` returns the stream at the top level and is **not** `async`; `read` itself must execute nothing. | `CLAUDE.md`, *Binding constraints* 3; `crates/happenstance-sqlite/src/event_store.rs:203-215` | Lazy means lazy: the `spawn_blocking` hop is deferred into `poll_next`, so the ceiling cannot be captured in `read` — it must be captured on the **first** `fetch_page`, under the same connection lock, before the first page's rows are selected. |
| `SqliteReadStream`'s doc explains why no `Statement`, `Rows` or `Transaction` may appear in a field, and `tests/shapes.rs` is the guard. | `crates/happenstance-sqlite/src/event_store.rs:248-271`; `crates/happenstance-sqlite/tests/shapes.rs` | Every field this story adds must keep `SqliteReadStream: Send + Unpin` true. A `SequencePosition` ceiling costs it nothing; a held cursor would cost it everything. |
| The runtime seam: `SqliteEventStoreError::NoRuntime` exists because `Handle::try_current()` turns a would-be `spawn_blocking` panic under a non-tokio executor into an ordinary stream error — and under the concurrency family a failed read is reported as a **sighting**, `"a concurrent read failed: {err}"`. | `crates/happenstance-sqlite/src/event_store.rs:26-32`, `:178`; `crates/happenstance-testkit/src/concurrency.rs:945-953` | ADR-0022's seam (a captured `Handle`) is consumed by this story. Without it the read path fails as a conformance verdict about atomicity rather than as an obvious wrong. |
| The visibility invariant is global: once any reader has observed an event at position P, no later read may yield an event at or below P that was not already visible. | `.kb/decisions/0013-position-assignment-and-visibility.md` | The snapshot ceiling is what makes a *paged* read compatible with that invariant across a second connection's commits mid-replay. |

## Questions

Open questions to resolve before specifying.

1. **What is the ceiling's value — the store head at first poll, or something
   cheaper?** Deferred to `spec`. Discover fixes the property: captured no later
   than the first `poll_next`, under the same connection lock as the first page's
   `SELECT`, and applied as an upper bound to every subsequent page statement.
   Reading `head` in a separate statement *before* the first page opens a window
   the ceiling exists to close.
2. **Does `PAGE_SIZE` stay 512?** Deferred to `spec` and flagged in the
   architecture brief §11 — the constant's own comment calls 512 "a placeholder
   until it is measured". Discover adds one constraint the planning stage can see:
   whatever it becomes, the correctness of paging must be **testable**, and every
   log the conformance suite builds is far smaller than any plausible page size.
   See *The wrong implementation*.
3. **How does the ceiling interact with `backwards`?** Answered as shape: under
   `backwards` the ceiling bounds the *older* end's counterpart — the starting
   end of the walk — and the caller's `to` still bounds the older end. The spec
   must write both directions out explicitly rather than expressing one and
   assuming symmetry; `read_to_under_backwards_bounds_the_older_end` is the rule
   that discriminates.
4. **Where does the ceiling live — `ReadCursor` or `SqliteReadStream`?** Deferred
   to `spec`. Architecture brief §4 says `ReadCursor` gains one field; the hard
   constraint discover fixes is only that whatever holds it keeps
   `SqliteReadStream: Send + Unpin` true, which `tests/shapes.rs` already guards.
5. **The wide-query decomposition.** *Not this story's.*
   `wide-query-chunked-not-refused` (HS-S0039) owns chunking and the k-way merge,
   and depends on this one. This story must leave the page query in a shape a
   merge can be built on — ordered by position, bounded by the same ceiling and
   `resume_from` — rather than one that assumes a single statement.
6. **The append-condition SQL strategy.** Not touched here. The read path shares
   the tag index with the probe, so a change to the layout would reach this story,
   but the choice is ADR-0022's.

## Decision

`read` is the largest rule family in the suite and the only port method that is
*not* `async`, which means its laziness is structural rather than stylistic: the
stream must execute nothing until polled, and it must then page without letting a
concurrent writer bleed into a replay already in progress. ADR-0011 already
requires the mechanism — a position ceiling captured no later than the first poll
— and the skeleton does not carry one, so adding it is discharge rather than
design. The spec will cover: replacing `fetch_page`'s `todo!()` with a real paged
`SELECT` ordered by position; a ceiling field captured on the first `fetch_page`
under the connection lock and applied as an upper bound to every later page;
composition with `ReadOptions::from`/`to`/`limit`/`backwards` such that the
effective bound is the tighter of caller and ceiling in each direction;
`resume_from` kept **inclusive**, stepping strictly past the last position
returned, with the skeleton's own recorded historical bug left un-"tidied";
`advance()`'s existing arithmetic untouched; the `spawn_blocking` hop deferred
into `poll_next` with ADR-0022's captured `Handle`; and no `Statement`, `Rows` or
`Transaction` held across a poll boundary, guarded by
`crates/happenstance-sqlite/tests/shapes.rs`. This story adds **no conformance
rule**, so the literal-position bar is vacuous — and it is the story most exposed
to the *opposite* error, since positions come from `AUTOINCREMENT` and are gappy:
paging must step past the last position **observed**, never `last + 1`. No
`[FROZEN]` clause is amended; ADR-0011 is consumed, not revisited.

## The wrong implementation

**The mutant: a paged read with no snapshot ceiling.** `fetch_page` selects the
next `PAGE_SIZE` rows above `resume_from` and stops when a page comes back short.
It is simpler, it is what the skeleton's existing `ReadCursor` fields already
support without adding one, and — this is the part that matters — **every
conformance rule in the suite passes against it.**

The reason is arithmetic rather than luck. `PAGE_SIZE` is 512
(`crates/happenstance-sqlite/src/event_store.rs:81`) and no rule in
`crates/happenstance-testkit/src/suite.rs` builds a log anywhere near that size;
the two rules that actually test read isolation —
`read_result_is_stable_under_concurrent_append` and
`query_items_share_one_snapshot` (`crates/happenstance-testkit/src/registry.rs:213-214`)
— seed two events and append one more mid-stream
(`suite.rs:5696-5736`). With two events, `fetch_page` runs **once**, the whole log
arrives in the first page, and a late append cannot possibly be picked up because
there is no second statement to pick it up with. The rules are correct and the
adapter satisfies them for a reason unrelated to the property they are asserting.
The defect appears only above 512 events with a second connection committing
mid-replay — a configuration nothing in the workspace constructs — and it is a
violation of ADR-0011 and, transitively, of ADR-0013's global visibility
invariant.

**Where the control must live.** Not in the testkit's `mutation_coverage/`: those
mutants are `Rc`/`RefCell` stores built from `correct.rs`'s primitives with one
step perturbed, with "no channel, no I/O and no custom `Future` in the correct
core" (`crates/happenstance-testkit/tests/mutation_coverage/harness.rs:24-30`),
and they hold their whole log in a `Vec` — pagination is not a thing they have.
The control belongs to this story, in
`crates/happenstance-sqlite/tests/`: an adapter-owned test that seeds strictly
more than `PAGE_SIZE` events, takes a `read` stream, polls it once, appends
through a **second connection** onto the same file, drains the rest, and asserts
the drained positions are exactly the positions that existed at first poll —
compared against the positions the store actually assigned, never against a
literal range, because `AUTOINCREMENT` leaves gaps. If `PAGE_SIZE` becomes
configurable under `cfg(test)`, the same test runs in milliseconds; if it does
not, it seeds 513 events and still runs in milliseconds. Either way the assertion
must not be "the suite is green".

**The second mutant, invisible for the same reason: `resume_from` treated as
exclusive.** Seed it from `ReadOptions::from`, advance it to `last.position`, and
every page boundary after the first either re-reads or skips one row depending on
which half of the confusion you land in. This is not hypothetical — it is the bug
the skeleton's own doc comment records having already happened once
(`crates/happenstance-sqlite/src/event_store.rs:313-323`). With every suite log
under one page, no page boundary is ever crossed and every ordering, limit and
window rule passes. The same over-`PAGE_SIZE` adapter test is what fails it, and
that is the argument for writing that test in **this** story rather than deferring
it to the fixture story: by then the suite is green and there is nothing left to
prompt anyone to ask.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
