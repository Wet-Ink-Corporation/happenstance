---
item: HS-S0025
stage: discover
created: 2026-08-12T13:01:47.132Z
updated: 2026-08-12T13:01:47.132Z
template_sig: 86ce4036
rendered_sig: 3fb107e6
---

# Discover — A given/when/then DSL that cannot hide its own filter

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice: land the given/when/then DSL over `MemoryEventStore` in `crates/happenstance/`, seeding and nominating through `happenstance_core::Query` only, **whose failure message names the events seeded but not selected** by the model's query — a filter that hides what it filtered re-opens the hazard ADR-0020 closes — demonstrated by driving a caller's retry loop against `FaultyStore<S>` with no database | `_storymap.md:56` (M4 row) | The failure message is the feature. A DSL that only reports expected/actual is the mutant, named in the slice itself |
| AC-009 — a given/when/then DSL seeds a `MemoryEventStore`, invokes the decision and asserts on emitted events or the error | `project.md:192-196` | Split by crate with `misbehaving-testkit-stores`: the DSL lives in `happenstance` because it is application-facing and decodes (`_decomposition.md:424`) |
| `depends_on: command-loop` — supplies `commit`/`commit_with`, `Retry`, `Committed` and `CommandError`, which `when(...)` drives and whose error the assertion reports | `_storymap.md:56`; `_design.md:585-586` | `Given::when` returns `Result<Decision<B::Event>, CommandError<MemoryStoreError, D>>` |
| `depends_on: decision-model-composition` — supplies the tuple `Boundary` impls, so the DSL is generic over `B: Boundary` and a *composed* boundary is testable, not only a single model | `_storymap.md:56`; `_design.md:575-578` | `given<B: Boundary>(boundary: B)` is the signature; composition is in scope for the DSL from day one |
| `depends_on: misbehaving-testkit-stores` — supplies `FaultyStore<S>`, which is how the DSL demonstrates a caller's retry loop **without a database** | `_storymap.md:56`, `:120-123` | The retry demonstration is part of this story's acceptance, and its fixture is a sibling's |
| AC-U11, the interaction-quality invariant this story is built on: a `DecisionModel`'s derived `Query` is **inspectable** — reachable as a value and assertable in a test, not private machinery inside `read` — and the DSL's failure message names the events **seeded but not selected**, because that set is exactly where a fold/query divergence would be invisible | `_decomposition.md:164-169` | *"A DSL that reports only 'expected X, got nothing' hides its own filter"* |
| The binding message shape: four labelled regions in order — `expected:`, `actual:`, `selected by the model's query:`, `seeded but NOT selected:` — each capped at **8 event rows** with `… and N more`, truncating `selected` **first** and `seeded but NOT selected` **last**, because the last region carries the diagnosis; then one line naming the derived query | `_design.md:188-208`, `:802-806`, `:866-869` | The truncation order is a design decision, not an implementation detail |
| Empty state: `given(…)` with no seeded events prints `seeded but NOT selected: (nothing was seeded)` — **the region stays, and says so** | `_design.md:918` | Silence is the failure mode this repository has already paid for once (AC-U20) |
| The binding surface: `given<B: Boundary>(boundary) -> Given<B>`; `Given::event<E: DomainEvent>(self, event) -> Result<Self, CodecError>`; `Given::when<D, F>(self, decide) -> Result<Decision<B::Event>, …>`; `Decision::then(self, expected: &[E])` and `then_refused(self)`. `Decision` is `#[non_exhaustive]` and `#[must_use]` with the consequence written into the message | `_design.md:574-599` | The `#[must_use]` message is *"a Decision is not appended until it is committed; dropping it discards the events the decision produced"* |
| Placement: `happenstance::testing` is a **real, named module**, not root re-exports, gated `#[cfg(feature = "memory")]` (on by default) — test-time vocabulary mixed into the root's item table doubles the page a P1 reader scans | `_design.md:690-695`, `:826` | One region on the landing page points to the module; the module is one click away, not interleaved |
| One filter vocabulary: the model's derived query, a projection's nomination and the DSL's seeding all speak `happenstance_core::Query`. No second filter type, no string-matching helper, no `&[&str]` of event-type names in the public surface | `_decomposition.md:112-116` (AC-U04) | The DSL must nominate through `Boundary::query()`, never through a query it writes itself |
| Beat 2 of the journey today forces *"pick a database"* and *"does the domain model work"* into one decision; `MemoryEventStore` plus the DSL is what separates them | `_decomposition.md:57-59`; `project.md`, AC-009 | This is the story's user-facing purpose, and it is why the module is *revealed* rather than opened-on-demand (`_design.md:826`) |

## Questions

**Answered here.**

- *Does the DSL write its own query?* No. It nominates through `Boundary::query()` — the same
  derivation the production path uses (AC-U04, `_decomposition.md:112-116`). A DSL that
  writes its own query tests a query the application never runs.
- *Is the derived query reachable as a value?* Yes, and it must be: AC-U11 requires it
  inspectable and assertable, not private machinery inside `read`
  (`_decomposition.md:164-169`). `Boundary::query` being public is what makes the fourth
  message region computable at all.
- *Where does the DSL live?* `crates/happenstance/src/testing/`, a named module gated on the
  default `memory` feature (`_design.md:690-695`). Not the testkit: it decodes, and it is
  application-facing.
- *What does a failing `then` print?* Four regions, in the fixed order and with the fixed
  truncation policy above (`_design.md:190-203`, `:802-806`).

**Deferred to `spec`.**

- *The assertion vocabulary beyond `then` / `then_refused`.* Deliberately not prescribed
  (`_decomposition.md:712-723`). Discovery's constraint: whatever is added must preserve the
  four-region failure message, because the message is the acceptance criterion.
- *Whether `Given::event` takes one event or many.* The design shows one per call returning
  `Result<Self, CodecError>` (`_design.md:581`); a batch spelling is spec's to add if the
  ergonomics warrant it.
- *How the "long label" state renders.* The design fixes the rule — a long event type wraps
  and the marker column stays left-aligned (`_design.md:922`) — the exact rendering is spec's.

**Not blocked.** Neither `trybuild` nor `MemoryProjectionStore` is an input to this story.
It is, however, the most dependency-dense story in the project — three in-project edges —
so it cannot start until M3 and `misbehaving-testkit-stores` have landed.

## Decision

The problem this slice solves is that today a developer cannot answer *"does my domain model
work?"* without first answering *"which database?"* — the two are one decision, and the
first hour of the journey this project moves is spent on the wrong one. This story gives
them a way to seed a `MemoryEventStore`, run a real decision through the real command loop,
and assert on the events it emitted or the error it returned, with no database and no
`async` ceremony beyond an `.await`. And it does one thing more, which is the reason the
story exists in the shape it does: when the assertion fails, the message names the events
that were **seeded but not selected by the model's own query**, because that set is exactly
where a fold/query divergence hides — the same divergence ADR-0020 closes structurally, here
made visible at the one moment a developer is looking. The spec for this story covers
`given`/`when`/`then`'s signatures and the `happenstance::testing` module's placement and
feature gate, the `#[must_use]` on `Decision` with its consequence in the message, the
four-region failure output with its ordering, its 8-row caps and its truncation order, the
empty-state text that keeps the fourth region present when nothing was seeded, nomination
strictly through `Boundary::query()`, and the retry demonstration driving a caller's loop
against `FaultyStore<S>` with no database. No `[FROZEN]` clause is amended.

## The wrong implementation

**The mutant: a DSL whose `then` renders expected and actual only.**

```rust
pub fn then(self, expected: &[E]) {
    assert_eq!(self.emitted, expected);      // or a hand-rolled equivalent
}
```

This is what "a given/when/then DSL" means to almost everyone, it is three lines, and it
passes every check in the repository. `cargo xtask ci` is green. Every DSL unit test passes.
AC-009 reads as satisfied: a DSL seeds a `MemoryEventStore`, invokes the decision, and
asserts on emitted events or the error. The rendered message is idiomatic Rust and looks
exactly like every `assert_eq!` failure a reader has seen.

It is wrong because it hides the filter, and the filter is where the bug is. Concretely: a
`Seats` model whose `EVENT_TYPES` lists `CourseDefined` and `StudentSubscribed` but not
`StudentUnsubscribed` — the mutant from `domain-event-and-decision-model`, which the compiler
cannot reject. A developer writes the obvious test: seed a `Defined { capacity: 2 }`, two
`Subscribed`, one `Unsubscribed`, then subscribe a fourth student and expect success. The
derived query never nominates the unsubscribe, so `taken` stays at 2, the decision refuses,
and the message says:

```
assertion failed
expected: [StudentSubscribed]
actual:   refused: course c1 is full (2/2)
```

Every word of that is true, and every word of it points at the **decision** — so the
developer goes and reads their capacity arithmetic, which is correct. The information that
would end the investigation in one second — *you seeded a `StudentUnsubscribed` and your
model's query did not select it* — is computable, is held by the DSL at the moment it
panics, and is thrown away. The DSL has become an instrument that reliably misdirects, which
is worse than no instrument, and it does so on the one class of bug this entire project
exists to eliminate.

The four-region message is what rejects it (`_design.md:190-203`), and this story's spec must
include a test that *asserts on the panic message* — driving the divergent model above and
requiring the string `seeded but NOT selected:` and the unselected event's type to appear.
Without that test, the fourth region is a paragraph in a design document rather than
behaviour.

**A second mutant: a DSL that seeds and reads with its own query.**

```rust
pub async fn when<D, F>(self, decide: F) -> … {
    let query = Query::all();                       // "seed everything, read everything"
    let (events, last) = read_decision_model(&self.store, &query).await?;
    // fold them all into the boundary, then decide
}
```

It passes everything, and it passes *more* than the correct implementation does — because
`Query::all()` selects the unsubscribe the model's own query would have missed, the divergent
model above folds correctly under test and the test goes green. The DSL then certifies a
domain model that will fail in production, since the production path reads through
`Boundary::query()` and the append condition is built from it. A test harness whose read
path differs from the application's read path is not a test harness; it is a second
implementation that happens to agree more often. AC-U04 forbids it in general terms — one
filter vocabulary, no second filter type — and here it is the specific difference between
proving something and proving nothing.

**A third mutant: `Decision` without `#[must_use]`.** `given(b).event(e)?.when(decide)
.await?;` compiles and passes with the `then` never called — the test asserts nothing and
reports success. `#[must_use]` with the consequence in the message
(`_design.md:589-590`) is the whole of the defence, and the `?` at the end of `when` makes
the omission easy to write and invisible in review.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

The two judgement boxes. **Literal positions:** this story adds no conformance rule. Its
tests assert on emitted domain events, on error variants and on panic-message text; the DSL
seeds through the store and never asserts a `SequencePosition` at all — and the one place a
position could leak into an assertion, the retry demonstration, asserts on
`Committed.attempts` instead. Ticked, checked rather than assumed. **Frozen clauses:**
nothing under `crates/happenstance-core/src/**` is touched. `Query`, `MemoryEventStore` and
`AppendError` are consumed as the frozen contract defines them; the DSL's contribution is to
*display* what the frozen `Query` selected, not to change what it selects.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
