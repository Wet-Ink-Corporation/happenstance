---
item: HS-S0020
stage: discover
created: 2026-08-12T13:01:40.390Z
updated: 2026-08-12T13:01:40.390Z
template_sig: 86ce4036
rendered_sig: "44921233"
---

# Discover — DomainEvent and DecisionModel, mounted at the crate root

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice: land `DomainEvent` with `const EVENT_TYPES: &'static [EventType]` and `DecisionModel` with `apply(&mut self, Self::Event)` plus the derived `query()` ADR-0020 chose, exported at the crate root with the complete first-program doctest, so a model whose fold and query disagree is unwritable | `_storymap.md:51` (M2 row) | Three obligations in one slice: the types, the root export, and the doctest. The doctest is not decoration — it is the artefact DT-2 is judged on |
| AC-001 — `DecisionModel::apply` takes `Self::Event` (a `DomainEvent` enum) and `query()` is derived from `EVENT_TYPES` plus tag constraints; a model whose fold and query disagree is unwritable | `project.md:159-164` | "Unwritable" is a claim about the type system, and this story is where it becomes true or does not |
| AC-014 — DT-2 resolved on the record; `adr-0020-fold-query-agreement` owns the record, this story is the shape it produces plus the unit test of the chosen failure mode | `project.md:211-213`; `_storymap.md:95-100` | The failure mode chosen must be *tested*, not merely asserted in prose |
| `depends_on: adr-0020-fold-query-agreement` — supplies the sealed-`Boundary` decision, the `Result<Query, InvalidQuery>` return, the empty-`EVENT_TYPES`-is-a-compile-error rule, and the `Clone` supertrait rationale, as an accepted and immutable atom | `_storymap.md:51`, `:109-117`; `_design.md:635-638` | This story implements a decision it does not own. Re-deciding any of it means a *second* atom, not an edit |
| The binding signatures: `DomainEvent { const EVENT_TYPES; event_type(&self) -> EventType; tags(&self) -> Tags; encode<C: Codec>; decode<C: Codec> }` and `DecisionModel: Clone { type Event: DomainEvent; scope(&self) -> &Tags; apply(&mut self, Self::Event) }` | `_design.md:390-419` | `query()` is **absent** from both, by design. It lives on `Boundary` (`_design.md:423-435`) |
| `event_type()` returns `EventType` **by value**, not `&'static EventType`: `EventType` carries a `Cow<'static, str>` so const promotion does not apply, and a reference-returning signature would force the impl to index `EVENT_TYPES` **by position** — two places to get wrong | `_design.md:640`; `crates/happenstance-core/src/event.rs:95-115` | The rejected alternative reintroduces exactly the hazard AC-001 closes |
| The residual, stated in the design and not hidden: `EVENT_TYPES` ↔ `event_type()` agreement is **not compiler-enforced**. It is tested by `assert_domain_event::<E>(&[…every variant…])`, and that residual *is* AC-013's measurement | `_design.md:641`, `:601-604` | This story must ship the test and must not claim a guarantee it does not have |
| `Tags::from_pairs` is the only way to build `Tags` and it is fallible | `crates/happenstance-core/src/tag.rs:300-310` | `scope(&self) -> &Tags` pays validation once in the caller's own constructor; a `-> Tags` built per call would force an `unwrap` inside an infallible signature (`_design.md:637`) |
| `EventType::from_static` is `const` and panics at compile time on an empty or over-long value; its own doc records why `compile_fail` is spelled bare rather than `compile_fail,E0080` | `crates/happenstance-core/src/event.rs:95-115` | `const EVENT_TYPES` needs no derive today, and the doctest conventions for this crate's compile-fail companion are already set in-tree |
| Composition roots this story must mount into: `crates/happenstance/src/lib.rs` — every new item `pub use`d at the crate root beside the surviving `pub use happenstance_core::*;` (`:75`), and the module doc's "Planned" bullets at `:35-52` become intra-doc links to the real items in place | `_decomposition.md:452-475` (*Composition roots* 1); `_storymap.md:39-45`; `crates/happenstance/src/lib.rs:35-52,75` | "Build it" and "wire it in" are not separate stories. A story that does not replace its own roadmap bullet is not done |
| The density budget the doctest is held to: ≤ 35 visible lines, ≤ 72 columns inside a doc fence, ≤ 12 prose lines above the first fence, and *the domain yields, never the ceremony* | `_design.md:845-855`, `:1016-1018` | The mock measured the specified program at **81 visible lines and 107 columns** — over both budgets (`_design.md:1196-1200`). That is a known, accepted finding, not a surprise to rediscover |
| No item may shadow a contract name — no `happenstance::Query`, no `happenstance::EventStore` of our own | `_decomposition.md:355-362` (AC-A01); `_design.md:702-707` | A shadowing name is a silent breaking change to a published facade and an ambiguity in every existing doctest |

## Questions

**Answered here.**

- *Where does `query()` live?* On `Boundary`, sealed, blanket-implemented for every
  `DecisionModel` — not on `DecisionModel` (`_design.md:635`). Settled by ADR-0020; this
  story consumes it.
- *Does `DomainEvent` get a derive macro in this story?* No. `EventType::from_static` is
  already `const`, so `const EVENT_TYPES` needs no macro
  (`crates/happenstance-core/src/event.rs:107-115`), and the trait is deliberately designed
  against a hand-written expansion of what a derive *would* emit so it does not move when
  one lands (`project.md:68-70`). Whether a derive ships at all is AC-013's, owned by
  `defect-log-and-macros-verdict`.
- *Is `EVENT_TYPES` ↔ `event_type()` agreement enforced by the compiler?* No, and the story
  must say so where a reader will see it. The instrument is `assert_domain_event`, and the
  enumeration it takes is itself hand-maintained — which is the honest statement of the
  residual (`_design.md:641`).

**Deferred to `spec`.**

- *Module layout inside `crates/happenstance/src/`.* Deliberately not prescribed
  (`_decomposition.md:712-723`); the only structural constraints are the root re-export and
  the roadmap-bullet replacement.
- *How the first program is brought inside its budget.* The mock recorded 81 lines against a
  35-line budget (`_design.md:1196-1200`). The yield order is fixed — the *domain* yields,
  never the ceremony — but which domain simplification is taken is the spec's call.
- *The `const` item that turns an empty `EVENT_TYPES` into a compile error.* Its exact form
  (a per-monomorphisation `const` assertion, RS-61-4) is spec-grain (`_design.md:662-663`).

**Not blocked.** Neither `trybuild` nor `MemoryProjectionStore` is an input to this story.
The `compile_fail` doctest companion on `DomainEvent`'s item page is explicitly **not**
AC-002's instrument (`_design.md:1130-1131`); AC-002 belongs to
`compile-fail-proof-artefact`, which is the story that is blocked.

## Decision

The problem this slice solves is that today the crate a user `cargo add`s is five lines —
`pub use happenstance_core::*;` at `crates/happenstance/src/lib.rs:75` — so there is no
vocabulary in which a consistency boundary can be *stated*, only one in which it can be
hand-assembled from opaque bytes and a `Query` written twice. This story lands the two
traits that give it one, and it lands them in the shape ADR-0020 chose: a `DomainEvent`
whose event types are a `const` list and whose payload knows how to encode itself, and a
`DecisionModel` that folds `Self::Event` — a domain enum, so the `match` is exhaustive —
and holds already-validated `Tags` as its scope. The spec for this story covers the exact
signatures of both traits and their doc comments (each carrying the alternative that lost,
per RS-70-5), the crate-root `pub use` of every new item beside the surviving glob
re-export, the in-place conversion of the module doc's five "Planned" bullets at
`crates/happenstance/src/lib.rs:35-52` into intra-doc links, the complete first-program
doctest with no elisions beyond the `#[tokio::main]` wrapper and the `Ok::<(), E>(())`
closer, the `assert_domain_event` residual test and the honest statement of what it does
*not* enforce, and the unit test of DT-2's chosen failure mode — a rejected tag pair fails
at the model's constructor, not at the first read. No `[FROZEN]` clause is amended.

## The wrong implementation

**The mutant: a `DomainEvent` whose `event_type()` returns a type absent from
`EVENT_TYPES`.**

```rust
enum Enrolment { Defined { .. }, Subscribed { .. }, Unsubscribed { .. } }

impl DomainEvent for Enrolment {
    const EVENT_TYPES: &'static [EventType] = &[
        EventType::from_static("CourseDefined"),
        EventType::from_static("StudentSubscribed"),
        // the third was never added
    ];

    fn event_type(&self) -> EventType {
        match self {
            Self::Defined { .. }     => EventType::from_static("CourseDefined"),
            Self::Subscribed { .. }  => EventType::from_static("StudentSubscribed"),
            Self::Unsubscribed { .. } => EventType::from_static("StudentUnsubscribed"),
        }
    }
    // …
}
```

This is what a third variant looks like six months after the first two. Every check passes:
the `match` is exhaustive, so the compiler is satisfied; `EVENT_TYPES` is a valid `const`
list, so `from_static`'s compile-time refusal never fires; `Boundary::query` derives a
perfectly valid `Query` from the two types it was given and is self-consistent;
`cargo xtask ci` is green; the worked example compiles and runs; clippy has nothing to say.
The store happily accepts `StudentUnsubscribed` events, because `append` does not consult
`EVENT_TYPES`.

It is wrong because the derived query — and therefore the `AppendCondition` built from it —
has silently stopped covering a third of the domain. The model never reads an unsubscribe,
so `taken` is never decremented and the boundary refuses subscriptions to a course with a
free seat; and because the condition does not name `StudentUnsubscribed`, a concurrent
unsubscribe does **not** invalidate a stale decision, so an append lands against state the
model never saw. That is the DCB failure this project exists to make unwritable, arriving
through the one seam the type system genuinely cannot close.

The instrument that rejects it is `assert_domain_event::<Enrolment>(&[…every variant…])`
(`_design.md:601-604`), and this story must ship both the function *and* a test in
`crates/happenstance/` that calls it over a domain enum with the mutant applied, so the
check is demonstrated rejecting something rather than merely existing. The design is
explicit that the residual is not compiler-enforced (`_design.md:641`) — advertising it as
if it were is the second half of this mutant.

**A second mutant: `fn scope(&self) -> Tags` built on each call.**

```rust
fn scope(&self) -> Tags {
    Tags::from_pairs([("course", self.course.as_str())]).unwrap()
}
```

It compiles, it satisfies the trait if the trait is written that way, and every test in the
tree passes — because every test uses a literal course id like `"c1"`, which `Tag::key_value`
accepts. `Tags::from_pairs` is fallible (`crates/happenstance-core/src/tag.rs:304-310`) and
the `unwrap` is the tell: the first caller-supplied course id carrying a character the tag
validator refuses panics inside a library, in a function whose signature promises it cannot
fail. `_design.md:637` rejects exactly this shape — the model *holds* validated `Tags` and
returns a reference, so the validation is paid once, in the constructor the caller was
already writing, and the error arrives where the caller can act on it. That is DT-2's
resolution expressed as a signature, and a `-> Tags` return quietly undoes it while
satisfying every check in the repository.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

The two judgement boxes. **Literal positions:** this story adds no conformance rule. It adds
unit tests and a doctest, none of which observes a `SequencePosition` at all — the
first-program doctest asserts on `committed.attempts`, not on a position
(`_design.md:1101`). Ticked as vacuously true, and the vacuity is checked rather than
assumed. **Frozen clauses:** this story adds items to `happenstance`, where the
specification has no clause namespace (`_design.md:31-36`), and consumes `EventType`,
`Tags` and `Query` from the frozen contract without amending any of them. Nothing under
`crates/happenstance-core/src/**` is touched.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
