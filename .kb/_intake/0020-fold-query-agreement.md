# Staged: ADR-0020 — fold/query agreement, and DT-2's signature answer

**Staged 2026-08-15** for the next `/redkiln:kb-ingest` wave. **Not an atom.**
Long-form record:
[`references/adr/0020-fold-query-agreement.md`](../../references/adr/0020-fold-query-agreement.md).

Stage in **one** wave with `0021-payload-evolution-and-codec-tag.md`. Give the wave
a suffixed id if a wave has already run today, so the second cannot overwrite the
first's audit trail. Do **not** sweep `.kb/_intake/README.md` in with the glob — it
is the directory's own documentation, and ingesting it mints an atom about the
staging area.

---

## The op: create one `decision` atom

`.kb/decisions/0020-fold-query-agreement.md`. The decision was taken by a human at
the `/redkiln:plan` design sign-off gate on 2026-08-12 and is recorded here at
phase 7, **before** the code it governs exists — which is project AC-016's whole
content.

## Proposed frontmatter

The ingest run authors the frontmatter; this is the proposal it authors from.

```yaml
id: kb-decision-0020
title: >-
  A decision model folds a domain enum, and its query is derived on a sealed trait the caller
  cannot override
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0020
reversibility: medium
phase: 7
supersedes: null
superseded_by: null
summary: >-
  A DCB handler names its event set twice - once in the query, once in the fold - and nothing
  checks the two agree; the worked example does exactly this today and the gate is green either
  way. The resolution is structural rather than detective: query() is not on DecisionModel at
  all. The derivation is Boundary::query, on a sealed trait blanket-implemented for every
  DecisionModel and macro-implemented for tuples of arity 2..=8, so there is nowhere to put a
  hand-maintained query. It returns Result<Query, InvalidQuery> because QueryItem::new is
  fallible and happenstance-core is frozen; the other route to that error, an empty EVENT_TYPES,
  is turned into a compile error by a const item evaluated per-monomorphisation rather than left
  to the first read. DecisionModel::scope(&self) -> &Tags, so the validation is paid once in the
  constructor the caller already writes, Tags::from_pairs being the only and fallible way in.
  DecisionModel: Clone, not Default, so the command loop re-folds from the pristine model on
  retry without a default-constructible Tags re-opening the invalid-value hole. The residual Err
  is kept and re-read as "this boundary constrains nothing, which is Query::all() and must be
  said out loud"; commit and commit_with absorb it so the first program writes no extra
  question-mark. No unwrap and no edit to the frozen crate: the shortfall is logged as defect
  candidate D-1, that happenstance-core has no infallible QueryItem constructor for pre-validated
  inputs, with VT-18 as its nearest clause subject and a decision record as its route. Rejected:
  a provided method on DecisionModel (overridable, so hand-maintained again); a free
  derive_query::<M>() (ignorable); an infallible query() (unreachable without an unwrap);
  fallible-without-the-const-assertion (defers a compile-time-decidable mistake to the first
  read); scope() -> Tags by value (unwrap inside an infallible signature); scope() -> &[(&str,
  &str)] (revalidates, moves the error to the read); a Default supertrait (re-opens the invalid
  Tags hole); an exported compose! macro (caller-visible ceremony where a tuple would do).
  DT-2 resolves toward explicit declaration, and the price is on the record: 2.4:1 mapping
  ceremony to domain logic in the first program, carried as the falsifiable prediction that
  AC-013's verdict lands "happenstance-macros is in scope for 0.1" - a consequence, not a second
  decision.
depends_on:
  - kb-decision-0003
  - kb-decision-0006
related:
  - kb-decision-0007
  - kb-open-question-projection-id-unvalidated-001
source_paths:
  - .kb/_intake/0020-fold-query-agreement.md
  - references/adr/0020-fold-query-agreement.md
  - .bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md
  - crates/happenstance-core/src/query.rs
  - crates/happenstance-core/src/tag.rs
  - examples/course-subscriptions/src/main.rs
last_reviewed: 2026-08-15
```

Every `source_paths` entry resolves as staged. `kb-open-question-projection-id-unvalidated-001`
is cited as `related` only — it is the atom that owns the unvalidated-`ProjectionId`
question whose doc comment supplies this decision's two-constructor sentence, and
it is **not** resolved, edited or deleted by this wave.

## The hazard the atom must state, against real code

`examples/course-subscriptions/src/main.rs:114-125` builds a two-item `Query`
naming `COURSE_DEFINED`, `STUDENT_SUBSCRIBED` and `STUDENT_UNSUBSCRIBED`. At
`:140-155` the fold names the same three a second time, with a `_ => {}` arm that
absorbs any difference. Nothing — not the compiler, not clippy, not the conformance
suite — signals a divergence between them. A query selecting a type the fold
ignores widens the boundary (spurious conflicts, safe); a fold interpreting a type
the query never selects narrows the log the decision is made on (the append
condition protects less than the handler assumes, and that one corrupts).

## The claims the atom carries — five, and they are one shape

1. **`query()` is not on `DecisionModel`.** The derivation is `Boundary::query`, on
   a **sealed** trait blanket-implemented for every `DecisionModel` and
   macro-implemented for tuples of arity 2..=8. The blanket impl is what makes the
   derivation free; the seal is what makes it the *only* one (RS-40-2,
   `standards/rust/40-public-surface-and-evolution.md:73`).
2. **`Boundary::query(&self) -> Result<Query, InvalidQuery>`**, and an *empty*
   `EVENT_TYPES` is a **compile error** via a `const` item evaluated
   per-monomorphisation (RS-61-4, `standards/rust/61-compile-time-assertions.md:249`).
   The reader is meant to see a post-monomorphisation const-eval error naming the
   event-type set, not a trait-resolution error.
3. **`DecisionModel::scope(&self) -> &Tags`** — the model *holds* validated tags.
   `Tags::from_pairs` (`crates/happenstance-core/src/tag.rs:304`) is fallible and
   is the only way in, so the validation is paid once, in the constructor the caller
   already writes.
4. **`DecisionModel: Clone`, not `Default`.**
5. **Composition is `impl Boundary for (B1, B2)`…** by an *internal* `macro_rules!`,
   arity 2..=8. Zero caller-visible syntax; the ceiling is 8 because rustdoc renders
   one impl block per arity.

## Where the fallibility went — and the residual

`QueryItem::new` is fallible (`crates/happenstance-core/src/query.rs:56`) and
`happenstance-core` is frozen, so an infallible `query()` is unreachable without an
`unwrap`. The disposition: `Err` is **kept** and re-read as
`InvalidQuery::UnconstrainedItem` = *"this boundary constrains nothing, which is
`Query::all()`"* (`crates/happenstance-core/src/error.rs:99-102`); the other route
to it becomes a compile error; `commit`/`commit_with` absorb it into
`CommandError::Boundary`; **no `unwrap`, no edit to `happenstance-core`.**

**Defect candidate D-1**, for AC-012's log: *`happenstance-core` has no infallible
`QueryItem` constructor for pre-validated inputs.* Nearest clause subject **VT-18**
(`spec/SPECIFICATION.md:1371-1375`). Routed to a decision record, never a line edit.

## Rejected alternatives the atom must name, each with what it admits

- a **provided method** `query()` on `DecisionModel` — overridable, so the query is
  hand-maintained again and DR-02 becomes a convention;
- a free **`derive_query::<M>()`** — the caller can ignore it and pass their own
  query to the read;
- an **infallible `query()`** — unreachable without an `unwrap` in library code;
- **fallible with no `const` assertion** — a compile-time-decidable mistake deferred
  to the first read, while still charging explicit declaration's ceremony;
- **`scope() -> Tags` by value** — a fallible construction inside an infallible
  signature, which forces an `unwrap`;
- **`scope() -> &[(&str, &str)]`** — revalidates on every read and moves the error
  to the read;
- **`Default` as a supertrait** — re-opens "how do I get an invalid `Tags`";
- an **exported `compose!` macro** — caller-visible ceremony where a tuple would do.

**Exactly one path is licensed**, and the atom says so in the contract crate's own
words: *"two constructors enforcing different rules is the defect that makes an
invalid value reachable through the weaker one"*
(`crates/happenstance-core/src/projection.rs:152-154`).

## Consequences the atom carries

DT-2 resolves toward **explicit declaration**. The measured price in the signed-off
first program is **11 lines of domain to 26 of mapping ceremony — 2.4:1** — recorded
as a **falsifiable prediction** that AC-013's verdict lands *"`happenstance-macros`
is in scope for 0.1"*, checked against the rewritten example rather than the
doctest, and owned by the project's closeout (`RUNBOOK.md:525`). It is a
consequence, **not** a second decision: the atom states no *must* about the derive.

`EVENT_TYPES` ↔ `event_type()` agreement is **not** compiler-enforced and is tested
by `assert_domain_event::<E>(&[…])` — the residual named rather than hidden.

## Two citations repaired before staging

Recorded rather than quietly fixed, because a wrong `crates/**` range is caught by
nothing but a reader:

- the two-constructor sentence is at `crates/happenstance-core/src/projection.rs:152-154`,
  not `:47-61` as `_decomposition.md:544-546` and `_design.md` `## Shape decision`
  both cite;
- AC-013's `happenstance-macros` row is at `RUNBOOK.md:525`, not `:524`.

Both are **repairs**, not amendments: the set of implementations admitted is
unchanged.

## Open questions

**None is resolved by this atom.** Nothing under `.kb/open-questions/` has this
subject; `kb-open-question-projection-id-unvalidated-001` is cited as `related` and
left untouched. Whether `happenstance-macros` ships is a prediction here and a
verdict at project closeout, not a decision.

## Map rows

- `.kb/maps/decision-map.md` — one row: ADR-0020, atom link, title, `accepted`,
  phase 7, supersession `—`.
- `.kb/maps/domain-map.md` — an entry under the typed layer.

## Not proposed

No change under `crates/**`, `examples/**`, `xtask/**` or `spec/SPECIFICATION.md`;
no maturity marker moved; no `[FROZEN]` clause line-edited; no supersession of any
existing atom; no `reference` atom asserted — whether the long record also warrants
one is the wave's adjudication, and the corpus's precedent (`ADR-0029`) discharges
it with `source_paths` alone.
