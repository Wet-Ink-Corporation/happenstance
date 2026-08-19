---
item: HS-S0021
stage: discover
created: 2026-08-12T13:01:41.596Z
updated: 2026-08-12T13:01:41.596Z
template_sig: 86ce4036
rendered_sig: a46c969a
---

# Discover — Consistency boundaries compose at compile time

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice: land the declarative macro over `(P1, P2)`, `(P1, P2, P3)`, … that OR's each model's query fragment at compile time — `$crate`-qualified paths, most-literal-first arms — with a unit test asserting the composed `Query` is the union of the members' own | `_storymap.md:52` (M2 row) | The unit test is named in the slice. What "union" means precisely is the whole risk of this story |
| AC-004 — tuples of decision models produce one OR'd query at compile time, exercised by the worked example's multi-model case | `project.md:175-177` | "At compile time" means N is fixed by the tuple's arity; there is no runtime `Vec<Box<dyn …>>` |
| `depends_on: domain-event-and-decision-model` — supplies `DecisionModel`, `DomainEvent`, and the sealed `Boundary` trait whose blanket impl the tuple impls sit beside | `_storymap.md:52`, `:116-117`; `_design.md:423-435` | Composition is `impl Boundary for (B1, B2)`, so the trait must exist first. There is no other ordering |
| The binding shape: `impl Boundary for (B1, B2)…` by an **internal** `macro_rules!`, arity **2..=8**. An exported `compose!` macro was rejected — a macro in the caller's face is ceremony DT-2 is being measured on; the ceiling is 8 because rustdoc renders one impl block per arity and 16 buries the page | `_design.md:639`; `_design.md:753-757` (the macro is not public) | The caller writes a tuple. That is the entire caller-visible surface, and it is zero syntax |
| `Boundary` is **sealed** — the tuple impls and the blanket impl must stay the only ones, because a third implementor could hand-maintain a query | `_design.md:735` | Sealing is what makes composition safe to be the only mechanism |
| Macro discipline: every path a macro expansion emits is `$crate::`-qualified (RS-41-1) and arms are ordered most-literal-first so the arm that matches is the one the caller meant (RS-41-3) | `_decomposition.md:132-138` (AC-U07); `_design.md:639` | A guarantee whose diagnostic names a file the user did not write is a guarantee they cannot act on |
| `QueryItem::new` takes `types` **and** `tags` together and returns `Result<_, InvalidQuery>`; `Query::Items` is `#[non_exhaustive]` so `Query::from_items` / `from_item` are the only ways in | `crates/happenstance-core/src/query.rs:50-62`; `_decomposition.md:520-524` | A `QueryItem` is a **conjunction**. Composition is a disjunction *over items*, never a merge *within* one |
| The canonical two-item case, in the tree today: `subscribe` builds `Query::from_items([...])` with one item for the course's capacity and everyone holding a seat, and a second item for this student's own history — three things that would be three aggregates, one append condition covering all of it | `examples/course-subscriptions/src/main.rs:113-125` | This is precisely a two-model composition written by hand, and it is the mount AC-004 is exercised against |
| Composing several models into one query is what makes a dynamic consistency boundary *dynamic* | `project.md:77-79` (in-scope bullet); `crates/happenstance/src/lib.rs:43-46` | This is not a convenience. It is the DCB mechanism, and the roadmap bullet it replaces says so |
| The composed boundary must also **absorb**: `Boundary::absorb` decodes a `SequencedEvent` into each member's fold | `_design.md:431-435` | A tuple impl has two jobs, not one — the query *and* the fold. Getting the query right and the fold wrong is a live failure mode |
| Mount: every new item is `pub use`d at the crate root and the matching module-doc bullet becomes an intra-doc link in place | `_storymap.md:39-45`; `_decomposition.md:452-459` | Tuple composition is *revealed* via one line in the vocabulary region, not a name of its own (`_design.md:825`) |

## Questions

**Answered here.**

- *Macro or trait impls?* Both, in the only sane arrangement: an internal `macro_rules!`
  emits `impl Boundary for (B1, B2)`… for arities 2 through 8. The macro is not exported
  (`_design.md:753-757`); the caller writes a tuple.
- *Why cap at 8?* rustdoc renders one impl block per arity, and 16 of them buries the item
  page — a rendering cost paid by every reader, against an arity nobody in the worked
  example approaches (`_design.md:639`).
- *Is `Boundary::query`'s `Result` composed or short-circuited?* Composed: the tuple's
  `query()` is `Result<Query, InvalidQuery>` like every other `Boundary`, and the first
  member that refuses propagates. There is no arrangement in which a member's refusal is
  silently dropped.

**Deferred to `spec`.**

- *The exact assertion the unit test makes.* Discovery fixes the property — the composed
  `Query`'s items are the concatenation of the members' items, each unchanged — and the
  spec fixes its expression. The property matters more than the form; see the mutant below.
- *Whether the tuple impls are documented as one entry or eight.* A rendering decision
  inside the design's *revealed* disposition for composition (`_design.md:825`).
- *The arity the worked example actually uses.* `examples/course-subscriptions/src/main.rs:113-125`
  is a two-item query today; whether the rewrite composes two models or three is
  `worked-example-on-typed-layer`'s call.

**Not blocked.** Neither `trybuild` nor `MemoryProjectionStore` is an input to this story.

## Decision

The problem this slice solves is that a dynamic consistency boundary is only dynamic if
several independently-modelled concerns can be checked as **one** condition — the course's
capacity, the seats currently held, and this one student's history are three models and one
append condition — and today a caller assembles that by hand, writing `QueryItem`s in the
right shape and hoping. This story makes the composition a property of the type: a tuple of
`Boundary` values *is* a `Boundary`, its query is the disjunction of its members' queries,
and the arity is fixed at compile time by the tuple itself, so there is no caller-visible
syntax at all. The spec for this story covers the internal `macro_rules!` and its arity
range (2..=8), the `$crate`-qualification and most-literal-first arm ordering that keep any
diagnostic pointing at the caller's own code, the tuple impl's second obligation —
`absorb`, dispatching a decoded event to every member that nominated it — the propagation
rule for a member's `InvalidQuery`, the unit test asserting the composed query is the
members' items concatenated and individually unchanged, and the crate-root export plus the
one-line vocabulary entry that makes composition discoverable. No `[FROZEN]` clause is
amended; `Query` and `QueryItem` are consumed exactly as the contract defines them.

## The wrong implementation

**The mutant: a composition that merges its members into one `QueryItem` instead of
disjoining them into several.**

```rust
impl<B1: Boundary, B2: Boundary> Boundary for (B1, B2) {
    fn query(&self) -> Result<Query, InvalidQuery> {
        let mut types = Vec::new();
        let mut tags  = Vec::new();
        for q in [self.0.query()?, self.1.query()?] {
            // collect every type and every tag the members mentioned
        }
        Query::from_item(QueryItem::new(types, Tags::from_iter(tags)?)?)
    }
}
```

Everything passes. It compiles. `QueryItem::new` accepts it — the types are valid and the
tags are valid. It produces a `Query` for every tuple of every arity. And the obvious unit
test passes with room to spare: *"the composed query mentions every event type each member
mentions"* is **true**, and *"the composed query mentions every tag each member mentions"*
is also true. `cargo xtask ci` is green, clippy is silent, the doctest renders, and the
worked example — if it happens to compose two models scoped to the same tag set — produces
byte-identical behaviour.

It is wrong because a `QueryItem` is a **conjunction of its types and its tags**
(`crates/happenstance-core/src/query.rs:50-62`), and merging two items into one turns an OR
into an AND. Take the composition the tree already contains by hand
(`examples/course-subscriptions/src/main.rs:114-125`): a seats model scoped
`("course","c1")` and a student-history model scoped `("course","c1"), ("student","s1")`.
Merged, the single item requires **course=c1 AND student=s1**, so the `CourseDefined`
event — which carries only a `course` tag — is no longer selected. `capacity` folds to
`None`, the handler bails with *"course does not exist"* for a course that plainly exists,
and the `AppendCondition` derived from that query protects a boundary strictly narrower
than the decision actually depended on. The example is written as two items for exactly
this reason, and the merge is the single most natural-looking way to get it wrong.

**The test that rejects it, and which the story must ship:** assert that the composed
`Query`'s **item count equals the sum of the members' item counts**, and that each member's
own `query()` items appear in the composition **unchanged** — not merely that the union of
event types matches. A test written against the type set alone passes the mutant, which
makes it decorative in exactly the sense this repository forbids: name the wrong
implementation it rejects, and if it rejects none, it is not worth adding.

**A second mutant: the macro emits unqualified paths.** `Query::from_items(...)` rather
than `$crate::Query::from_items(...)`. It compiles in every test in this workspace, because
every test module has `happenstance::Query` in scope. It fails in a downstream crate that
imported `happenstance::Boundary` without `Query`, and the error names a `Query` the user
never wrote in a location they cannot see — RS-41-1's failure, and AC-U07's
(`_decomposition.md:132-138`). Nothing in this repository's own gate can catch it, because
the gate never compiles a consumer that lacks the import.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

The two judgement boxes. **Literal positions:** this story adds no conformance rule, and
its one unit test asserts on `Query` structure — items and their contents — and never
observes a `SequencePosition`. Ticked as vacuously true, checked rather than assumed.
**Frozen clauses:** this story adds tuple impls in `happenstance`, where the specification
has no clause namespace (`_design.md:31-36`). `Query` and `QueryItem` are consumed through
their public constructors exactly as frozen; nothing under `crates/happenstance-core/src/**`
is touched.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
