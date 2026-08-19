---
item: HS-S0018
stage: discover
created: 2026-08-12T13:01:38.419Z
updated: 2026-08-12T13:01:38.419Z
template_sig: 86ce4036
rendered_sig: 916fd149
---

# Discover — ADR-0020 — fold/query agreement, and DT-2's signature answer

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice: stage into `.kb/_intake/` the decision that a decision model folds a domain enum and *derives* its `Query` from `EVENT_TYPES` + tag constraints, resolving DT-2's concrete form, and hand off to `/redkiln:kb-ingest` so ADR-0020 exists as an atom **before** the code it governs | `_storymap.md:49` (M1 row) | This story writes a record, not code. Its output is intake material plus a human handoff; nothing under `crates/` is touched |
| AC-001 — the fold and the query cannot disagree; `apply` takes `Self::Event`, `query()` is derived, and ADR-0020 records why this shape and not the alternatives | `project.md:159-164` | The atom must name the losing alternatives explicitly, not merely assert the chosen shape |
| AC-014 — DT-2 resolved on the record, with the error-timing versus first-hour-cost trade stated | `project.md:211-213` | DT-2 was resolved in `_design.md` and approved; this story's job is to make that resolution *durable and immutable*, not to re-open it |
| AC-016 — both ADRs exist as decision atoms naming the alternatives that lost, with `redkiln validate --kb` clean | `project.md:218-220` | Atoms are authored by the ingest path, never hand-written into `.kb/decisions/` |
| `depends_on`: none | `_storymap.md:49`; manifest `dependsOn: []` | M1 is first precisely because it has no in-project input; it is the sequencing constraint every M2+ story inherits |
| DT-2's concrete technical form: *is `DecisionModel::query()` infallible, and if so where did the validation go?* | `_decomposition.md:526-546` (architecture brief, *The one signature question DT-2 actually is*) | The brief poses it and refuses to answer it; the answer is `_design.md`'s and is now settled |
| The resolution, binding: `query()` is **not on `DecisionModel`**. The derivation is `Boundary::query`, on a **sealed** trait blanket-implemented for every `DecisionModel`; it returns `Result<Query, InvalidQuery>`; an empty `EVENT_TYPES` is a **compile** error; `DecisionModel::scope(&self) -> &Tags` holds already-validated tags | `_design.md:635-637`, `_design.md:423-435`, `_design.md:1245-1248` (Sign-off item 1) | The atom records *this* shape. A sealed trait is the mechanism: there is nowhere to put a hand-maintained query |
| `DecisionModel: Clone`, not `Default` — the loop re-folds from a pristine model on retry | `_design.md:638` | A supertrait choice with a retry-correctness reason; belongs in the record, not only in the code |
| Why the hazard is real today: `subscribe` names `[COURSE_DEFINED, STUDENT_SUBSCRIBED, STUDENT_UNSUBSCRIBED]` in a `QueryItem`, then folds the same three by hand in a `match` on `sequenced.event_type().as_str()` with a `_ => {}` catch-all | `examples/course-subscriptions/src/main.rs:114-125` and `:140-156` | Two hand-written statements of one event set, with a catch-all arm that silently absorbs divergence. No compiler, clippy or conformance signal exists against it |
| `QueryItem::new` is fallible for well-formed inputs — it returns `InvalidQuery::UnconstrainedItem` when both `types` and `tags` are empty | `crates/happenstance-core/src/query.rs:50-62` | The `Result` in `Boundary::query` is inherited from the frozen crate, not chosen |
| `Tags::from_pairs` is fallible and is the only way in | `crates/happenstance-core/src/tag.rs:304-310` | `scope(&self) -> &Tags` pays the validation once, in a constructor the caller already writes — DT-2's "explicit declaration" pole |
| `EventType::from_static` is `const`, so `const EVENT_TYPES: &'static [EventType]` needs no derive today | `crates/happenstance-core/src/event.rs:107-115` | The trait can be designed against a hand-written expansion of what a derive would emit, and does not move when a derive lands |
| Defect candidate **D-1**: `happenstance-core` offers no infallible `QueryItem` constructor for pre-validated inputs, so every derived query carries an unreachable `Result`. Clause: VT-18 (which it partially contradicts) | `_design.md:652-672`; VT-18 at `spec/SPECIFICATION.md:1371` and `[FROZEN]` at `:8544` | Discovered *here*, in the ADR's own reasoning. It is routed, never fixed by a line edit; the owning story is `defect-log-and-macros-verdict` |
| Atoms are authored by `/redkiln:kb-ingest` from `.kb/_intake/`; hand-writing produced "the directory layout of the process without the process" and was reverted at `0269720`. An accepted atom is **immutable** | `_decomposition.md:643-656`; `_grounding.md:76-80` | The ingest is a human handoff. A correction after the code lands is a *second* atom, which is why AC-016's "written first" is a sequencing obligation |

## Questions

**Answered here.**

- *Is `query()` a method on `DecisionModel`?* No. It is `Boundary::query` on a sealed
  trait (`_design.md:635`). A provided method on `DecisionModel` is overridable and an
  overridden derivation is a hand-maintained query — DR-02's prohibition (`project.md:143`)
  restated as a type. A free `derive_query::<M>()` loses for the same reason: a caller can
  ignore it.
- *Where did the fallibility go?* Two places, deliberately. `Boundary::query` keeps the
  `Result` and gives `InvalidQuery::UnconstrainedItem` a real meaning — *this boundary
  constrains nothing, which is `Query::all()` and must be said out loud*. The other route
  to it, an empty `EVENT_TYPES`, becomes a **compile** error via a `const` item evaluated
  per monomorphisation (`_design.md:636`, `:652-665`).
- *Does the caller pay for the `Result`?* No. `commit`/`commit_with` absorb it into
  `CommandError::Boundary`, so the first program writes no extra `?` for it
  (`_design.md:664-665`).
- *Is DT-2 re-opened by this story?* No. It was resolved and signed off on 2026-08-12
  with no conditions (`_design.md:1226-1230`). This story records it.

**Deferred, with an owner.**

- *Where the codec tag lives* — `Event::metadata` versus `Tags`. Explicitly not ADR-0020's;
  it is the sibling M1 story `adr-0021-payload-evolution-and-codec-tag`, and the public
  surface is invariant under the choice (`_design.md:674-679`).
- *D-1's routing* — recorded here as discovered, routed by `defect-log-and-macros-verdict`
  (AC-012). This story must not "fix" it.
- *Whether the ceremony this shape costs justifies a derive* — AC-013, measured against the
  **rewritten example**, not against this decision (`_design.md:1104-1111`).
- *The ingest itself* — `/redkiln:kb-ingest` is human-invoked and is planned as a handoff at
  the point both M1 records are ready, not as a step inside this story's implementation
  (`_decomposition.md:643-648`).

**Not blocked.** Neither `trybuild` nor `MemoryProjectionStore` is an input to this story.

## Decision

The problem this slice solves is that a DCB command handler names its event set twice —
once in the `Query` it reads with and once in the fold it decides with — and nothing in
this repository can tell when the two drift apart; `examples/course-subscriptions/src/main.rs`
demonstrates the divergence today, complete with a `_ => {}` arm that swallows it. ADR-0020
is the durable record that the typed layer closes the hazard *structurally*: a decision
model folds a domain enum (so the `match` is exhaustive over `Self::Event`), and its query
is derived on a **sealed** `Boundary` trait, so there is no location in the type system
where a divergent query can be written. The spec for this story covers the intake document's
content and shape: the decision statement, the two alternatives that lost (a provided
`DecisionModel::query()` method, and a free `derive_query::<M>()` helper) with the reason
each loses, DT-2's resolution toward explicit declaration with the error-timing versus
first-hour-cost trade stated in the audience's terms, the `Result<Query, InvalidQuery>`
return with `UnconstrainedItem` given meaning and the empty-`EVENT_TYPES` case promoted to
a compile error, the `Clone` supertrait's retry rationale, the residual that
`EVENT_TYPES` ↔ `event_type()` agreement is *tested* rather than compiler-enforced, and
defect candidate D-1 recorded as discovered-and-routed. It also covers the handoff: the
file lands in `.kb/_intake/` for the same `/redkiln:kb-ingest` wave as ADR-0021, and the
matching open-question atom is resolved rather than deleted. No `[FROZEN]` clause is
amended by this story.

## The wrong implementation

**The mutant: ADR-0020 records the derivation as a *provided method* on `DecisionModel`.**

```rust
pub trait DecisionModel: Clone {
    type Event: DomainEvent;
    fn scope(&self) -> &Tags;
    fn apply(&mut self, event: Self::Event);

    /// Derived from `Self::Event::EVENT_TYPES` and `scope()`.
    fn query(&self) -> Result<Query, InvalidQuery> { /* the derivation */ }
}
```

This satisfies **every** existing check and every acceptance criterion read literally. The
query *is* derived. The default body is the correct derivation. The worked example compiles
and runs. `cargo xtask ci` is green, `cargo xtask spec-trace` is untouched, `redkiln
validate --kb` is clean, and the atom is accepted — and therefore immutable, so the mistake
is permanent and correctable only by a second atom.

It is wrong because a provided method is **overridable**, and an override is a
hand-maintained query with the trait's blessing. Concretely: a model whose `Self::Event`
carries `Defined | Subscribed | Unsubscribed` writes

```rust
fn query(&self) -> Result<Query, InvalidQuery> {
    Query::from_item(QueryItem::new(["CourseDefined", "StudentSubscribed"], self.scope().clone())?)
}
```

because the author is "only interested in seats taken". The fold's `match` stays exhaustive
over three variants and still compiles. Nothing reads back an unsubscribe, so `taken` is
never decremented — the model refuses a subscription to a course with a free seat — and,
worse, the `AppendCondition` built from that query does not cover unsubscribes, so a
concurrent unsubscribe does not invalidate the decision and the append succeeds against
state the model never saw. That is a silent DCB failure, and it is exactly the hazard
`examples/course-subscriptions/src/main.rs:114-156` shows today, restored one layer up. No
test in this repository can catch it: the only thing anything asserts is the model's own
`query()` against itself, and it agrees with itself perfectly.

The sealed `Boundary` trait is what rejects it — a caller cannot implement `Boundary`, so
the derivation has exactly one implementation and there is nowhere to put a second
(`_design.md:635`, `:735`).

**A second mutant, specific to a decision record rather than to code: ADR-0020 written
*after* `domain-event-and-decision-model` lands.** The atom is valid, the frontmatter
conforms, `redkiln validate --kb` passes and the supersession graph is intact — and the
record documents what was built instead of deciding what to build, which is the failure
AC-016's "written first" exists to prevent and the reason M1 precedes M2 in the merge order
(`_decomposition.md:653-656`; `_storymap.md:109-115`). This mutant is invisible to every
mechanical check and is caught only by the story order being honoured.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

Two boxes carry a per-story judgement rather than a reflex tick, and the judgement is
written here. **Literal positions:** this story adds no conformance rule and no test —
it produces an intake document — so the box is vacuously true and is ticked on that
basis. **Frozen clauses:** ADR-0020 governs `happenstance`, where the specification has
no clause namespace at all (`_design.md:31-36`); it amends nothing. The one frozen clause
this story's reasoning *touches* is VT-18, and it touches it by recording defect candidate
D-1 against it and routing it to a decision record — which is the box's rule observed, not
an exception to it.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
