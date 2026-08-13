---
item: "HS-S0020"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — DomainEvent and DecisionModel, mounted at the crate root

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two things about this ledger are worth stating so an implementer does not work around them.

**Module paths are the implementer's, and the test names are not.** `_decomposition.md`'s
architecture brief says outright that module names inside `crates/happenstance/src/` are the
implementer's choice, so `verifying_test` names the **crate-relative directory plus the test
function**. Renaming a module is fine; renaming or deleting a named test is a ledger edit and is not.

**Three rows are verified in part by looking at a rendered page** — AC-005, AC-006 and AC-007 — and
that is not a weaker bar, it is the only bar that can catch the failure mode. A trait that compiles
with no doc comment at all satisfies every type-level assertion in this spec; the composition
criteria are what make that fail (`_design.md`, `## Anti-patterns`; `.redkiln/config.yaml`'s comment
on why the design stage is kept while `design.capture` is absent). Their evidence is the reviewed
`cargo doc` output plus the mechanical assertions named alongside it.

```yaml
- id: AC-001
  criterion: >-
    GIVEN P1 has a domain — courses and subscriptions — and wants to model one consistency boundary
    before choosing a database, WHEN they write one enum of events and one struct that folds them,
    implementing `DomainEvent` (`const EVENT_TYPES: &'static [EventType]`, `event_type(&self) ->
    EventType` by value, `tags(&self) -> Tags`) and `DecisionModel` (`type Event: DomainEvent`,
    `scope(&self) -> &Tags`, `apply(&mut self, event: Self::Event)`, supertrait `Clone` and not
    `Default`), THEN the fold is a `match` over their own enum that the compiler makes exhaustive —
    adding a variant is an error in *their* file — the model holds its own validated `Tags` so
    `Tags::from_pairs`' fallibility is paid once in the constructor they already write, and every
    signature matches `_design.md`'s `## Signatures` row for row with no store, no adapter and no
    database anywhere in the program.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs (pub use at the crate root, beside pub use happenstance_core::*;)"
  verifying_test: "crates/happenstance/src/ unit tests fold_is_exhaustive_over_domain_enum, scope_returns_the_models_own_tags, decision_model_is_clone_not_default — cargo test -p happenstance --all-features; signatures reviewed against .bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md `## Signatures`"

- id: AC-002
  criterion: >-
    GIVEN P1's fear is a defect they discover in production, and today a DCB handler names its event
    set twice — once in the `Query` at `examples/course-subscriptions/src/main.rs:114-125`, once in
    the fold at `:128-173`, with nothing checking the two agree — WHEN they ask their model which
    events it reads, THEN they call `Boundary::query(&self) -> Result<Query, InvalidQuery>` and get a
    `Query` derived from `Self::Event::EVENT_TYPES` plus `scope()`, as an ordinary value they can
    bind, print and assert on; and WHEN they try to supply their own divergent query instead, THEN
    there is nowhere to put it — `Boundary` is sealed behind a private supertrait,
    blanket-implemented for every `DecisionModel`, so the second place the event set could be named
    does not exist in the type system.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs (pub use Boundary at the crate root)"
  verifying_test: "crates/happenstance/src/ unit tests query_is_derived_from_event_types_and_scope, query_is_a_value_a_test_can_assert_on; compile_fail doctest boundary_cannot_be_implemented_outside_the_crate paired with the compiling blanket-impl example on the same item page — cargo test -p happenstance --all-features"

- id: AC-003
  criterion: >-
    GIVEN DT-2 resolved toward explicit declaration — more caught at build time, ceremony paid in
    `const` items — and P1 is new to idiomatic Rust, WHEN they declare a `DomainEvent` with an empty
    `EVENT_TYPES`, THEN the program does not compile and the diagnostic is a post-monomorphisation
    const-eval error naming the event-type set, not a trait-resolution error and not a failure on
    their first read; and WHEN their boundary is well-formed but constrains nothing, THEN `query()`
    returns `Err(InvalidQuery::UnconstrainedItem)` — a refusal stated out loud rather than silently
    widened to `Query::all()` — with no `unwrap`, no `expect`, and no edit anywhere under
    `crates/happenstance-core/src/**`.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs (DomainEvent's item page, reached from the crate root)"
  verifying_test: "bare `compile_fail` doctest on DomainEvent's item page in crates/happenstance/src/, paired with a compiling example (RS-62-1); crates/happenstance/src/ unit test unconstrained_boundary_is_an_error_not_query_all; clippy -D warnings via cargo xtask affected --base main; git diff --name-only main showing no path under crates/happenstance-core/"

- id: AC-004
  criterion: >-
    GIVEN P1 wants their events to be Rust values rather than `format!("…").into_bytes()`, and
    their log will one day hold event types this model does not know, WHEN they encode through
    `DomainEvent::encode<C: Codec>` / `decode<C: Codec>` and fold a read batch through
    `Boundary::absorb<C: Codec>(&mut self, &SequencedEvent, &C)`, THEN an event the query never
    nominated is skipped by nomination rather than by a silent type check, an event that was
    nominated and cannot be decoded returns `CodecError::UnknownEventType` because that is a genuine
    `EVENT_TYPES`/fold disagreement, and every codec failure carries the underlying error as a typed
    `#[source]` on a `#[non_exhaustive]` enum — never a `String`, and never an associated `Error`
    type that would add a third parameter to every downstream signature.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs (pub use Codec, CodecError at the crate root; absorb on Boundary)"
  verifying_test: "crates/happenstance/src/ unit tests absorb_skips_an_unnominated_event, absorb_returns_unknown_event_type_when_a_nominated_event_cannot_be_decoded, absorb_applies_a_decoded_event_to_the_fold, codec_error_carries_a_typed_source — cargo test -p happenstance --all-features; reviewed against standards/rust/30-error-taxonomy.md RS-30-2"

- id: AC-005
  criterion: >-
    GIVEN P4 has one bounded sitting on docs.rs and P1 arrives via `cargo add happenstance`, WHEN
    they land on the crate's page and write their first `use`, THEN every item this story adds is
    reachable as `happenstance::DomainEvent`, `happenstance::DecisionModel`, `happenstance::Boundary`,
    `happenstance::Codec`, `happenstance::CodecError` — `pub use`d at the crate root beside the
    surviving `pub use happenstance_core::*;` (`crates/happenstance/src/lib.rs:75`), with module
    paths kept private structure and no new item shadowing a contract name (`Query`, `EventStore`,
    `Tags`, `Event`) — every intra-doc link on the page resolves under default features and under
    `--no-default-features`, and the manifest carries `[package.metadata.docs.rs]` with `lib.rs`
    carrying `#![cfg_attr(docsrs, feature(doc_cfg))]` so no future gated item can ever render on
    docs.rs without its badge.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs:75 (the crate root re-export block) and crates/happenstance/Cargo.toml"
  verifying_test: "cargo doc -p happenstance --no-deps and cargo doc -p happenstance --no-deps --no-default-features, both warning-free (docs step of cargo xtask ci --fast); crates/happenstance/src/ unit test contract_names_are_not_shadowed; manifest reviewed against crates/happenstance-core/Cargo.toml:54-56"

- id: AC-006
  criterion: >-
    GIVEN the crate-root page today tells a reader that `DomainEvent` and `DecisionModel` are
    "Planned, and specified in `spec/SPECIFICATION.md`" (`crates/happenstance/src/lib.rs:41-46`),
    WHEN a reader lands on that page after this PR, THEN those two bullets have been rewritten in
    place — same region, same order, same discriminator prose, the bold lead-in term now an intra-doc
    link to the real item, with the remaining three bullets left as honest promises — so the reader
    is never shown a roadmap entry beside the shipped item it describes and is never sent to a second
    page to find it; and every item this story lands carries composed presentation, not bare markup:
    a first-sentence summary that is a complete claim, an `# Errors` section on every fallible
    function naming the condition rather than the type, and — on each of the four unusual constructs
    (the seal, the blanket impl, the per-monomorphisation `const`, the by-value `event_type()`) — the
    alternative that lost, named once, where the reader is.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs:35-52 (the module doc's vocabulary region)"
  verifying_test: "human review of the rendered cargo doc page at the report gate against _design.md `## Composition` region 4 and anti-patterns 1, 2, 14, plus _decomposition.md AC-U03/AC-U15; mechanically rg -n \"Planned, and specified\" crates/happenstance/src/lib.rs (three surviving bullets) and rg -n \"DomainEvent|DecisionModel\" crates/happenstance/src/lib.rs showing both only as intra-doc links"

- id: AC-007
  criterion: >-
    GIVEN P4 reads the page in a 1024x768 window and P1 copies the first fence they see, WHEN the
    vocabulary doctest and the item pages render, THEN the page fits the budget it was designed to:
    doc-comment prose ≤ 80 columns, code inside a doc fence ≤ 72 columns (so the fence never scrolls
    horizontally at 1024px), the doctest ≤ 35 visible lines with ≤ 2 hidden lines and those only
    harness (a runtime wrapper and an `Ok::<(), E>(())` closer, never API), first sentence of every
    doc comment ≤ 80 characters and a complete claim, every public identifier this story adds ≤ 24
    characters, and the crate-root module doc ≤ 130 lines total — so nothing this story authors
    renders as a truncated item-table row or as a fence a reader must scroll.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs (module doc + the vocabulary doctest) and the item pages under crates/happenstance/src/"
  verifying_test: "crates/happenstance/src/ unit test doc_density_budget_holds (reads the crate's own sources and asserts the column/line budgets over /// and //! lines and fenced-block bodies); plus human review of the rendered page against _design.md `## Density budget` and anti-patterns 3, 4, 5 — anti-pattern 5 scoped to this story's own items per spec.md Clarification 2"

- id: AC-008
  criterion: >-
    GIVEN P1 wants to build a boundary, inspect it and throw it away without consequence, and GIVEN
    the one agreement the compiler cannot check is that every variant's `event_type()` is a member of
    `EVENT_TYPES`, WHEN they define events, fold a model, derive a query and encode a payload, THEN
    nothing performs I/O, nothing touches a store, no function this story adds takes an `EventStore`
    at all, and no value is mutated outside the caller's own `&mut` — the append remains the single
    irreversible act and it is not in this PR; and THEN the residual agreement is tested rather than
    asserted away, by a crate-internal test over this story's fixture domain proving every variant's
    `event_type()` is in `EVENT_TYPES`, with the ceremony cost left recorded and un-"fixed" (no
    derive macro is added here).
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs (the crate root surface: no item it exports takes a store)"
  verifying_test: "crates/happenstance/src/ unit tests every_variant_event_type_is_declared and nothing_in_the_vocabulary_touches_a_store — cargo test -p happenstance --all-features; plus rg -n \"async fn|SendEventStore\" crates/happenstance/src/ returning nothing new, and the diff containing no new crate under crates/ (no happenstance-macros)"
```
