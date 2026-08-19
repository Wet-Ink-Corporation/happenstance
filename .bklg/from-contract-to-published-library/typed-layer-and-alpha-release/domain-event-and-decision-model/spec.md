---
item: HS-S0020
stage: spec
created: 2026-08-12T13:46:16.232Z
updated: 2026-08-12T13:46:16.232Z
template_sig: 87bbf1d0
rendered_sig: e2f3d0fb
---

# Spec — DomainEvent and DecisionModel, mounted at the crate root

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md` |
| This spec | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/domain-event-and-decision-model/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md` — Architecture brief (*Composition roots*, AC-A01/AC-A02/AC-A04; *The one signature question DT-2 actually is*), UX brief (AC-U01…AC-U06, AC-U11, AC-U14, AC-U15, AC-U19), Testing brief (AC-001, AC-004, AC-014 rows) |
| Signed-off design (**binding**) | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` — `## Items`, `## Signatures`, `## Shape decision`, `## Placement and re-export`, `## Visibility and stability`, `## Composition`, `## Density budget`, `## Anti-patterns`, `## The doctest`, `## Sign-off` (Approved, 2026-08-12) |
| Governing decision record | ADR-0020, staged by `adr-0020-fold-query-agreement` (M1) → `.kb/decisions/0020-fold-query-agreement.md` via `/redkiln:kb-ingest`; its spec at `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/adr-0020-fold-query-agreement/spec.md` |
| Story map (this row) | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_storymap.md:51` (M2 `typed-vocabulary`), merge order step 2 at `:116-117` |
| Roadmap pointer | `RUNBOOK.md:3993-4007` — phase 7's fold/query hazard and the derived query; `RUNBOOK.md:4002-4004` — design against a hand-written expansion of what a derive would emit |
| Mount root | `crates/happenstance/src/lib.rs` (`:75` the glob re-export, `:35-52` the roadmap bullets this story starts replacing) |

## One-line PR slice

Land `DomainEvent` with `const EVENT_TYPES: &'static [EventType]` (const-constructible already — `crates/happenstance-core/src/event.rs:108`) and `DecisionModel` with `apply(&mut self, Self::Event)` plus the derived `query()` ADR-0020 chose, exported at the crate root with the complete first-program doctest, so a model whose fold and query disagree is unwritable.

## Executive summary

This PR turns the five-line facade into a crate with a vocabulary. `crates/happenstance/src/lib.rs`
is 76 lines and line 75 is `pub use happenstance_core::*;`; two of the five bullets under
*"Planned, and specified in `spec/SPECIFICATION.md`"* (`:41-46`) stop being promises and become
intra-doc links to real items in place.

The delta is four public items and one private module: **`DomainEvent`**, **`DecisionModel`**,
the sealed **`Boundary`** (blanket-implemented for every `DecisionModel`, carrying the *derived*
`query()` and `absorb()`), and the **`Codec`**/`CodecError` vocabulary those signatures are
written in — plus `sealed`, which is private and is the whole of why a hand-maintained query has
nowhere to live. Everything is `pub use`d at the crate root beside the surviving glob
(`_design.md`, *Placement and re-export*).

**Nothing here is re-decided.** ADR-0020 settled the shape at the M1 milestone and a human signed
`_design.md` off on 2026-08-12; this story is the shape those records license, built, mounted, and
tested at its chosen failure mode (`_decomposition.md`, Testing brief, AC-014 row). Where this spec
looks like it is deciding something, it is drawing a *slice seam* — which of M2's neighbours owns an
item the binding design lists — and each of those is stated with the alternative that lost, under
*Context pack → What lands here and what does not*.

Pointer, not restatement: the project's scope, ACs and risks are at `project.md`; the surface is at
`_design.md`; the reasoning behind the shape is ADR-0020's record. None is re-litigated here.

## Context pack

**Read this section and you can start. Everything deeper is a signposted anchor below.**

### The hazard, in this tree, today

A DCB handler names its event set **twice** — once in the query it reads with, once in the fold that
interprets what came back — and nothing checks the two agree.
`examples/course-subscriptions/src/main.rs:114-125` builds a two-item `Query` naming
`COURSE_DEFINED`, `STUDENT_SUBSCRIBED`, `STUDENT_UNSUBSCRIBED`, and the fold immediately below it
(`:128-173`) re-interprets those same names, with no compiler, clippy or conformance signal against a
divergence. A query that selects an event the fold ignores silently weakens a consistency boundary.

**This story is the mechanism that makes that defect unwritable.** Not by adding a check — by
removing the second place. The query is *derived* from the same `const EVENT_TYPES` the fold is
exhaustive over, and the trait carrying the derivation is sealed, so there is nowhere to put a
divergent one (project `DR-01`, `DR-02`; `RUNBOOK.md:3993-4007`).

### The shape ADR-0020 licenses — five decisions, not five options

Take these as given. Each is a row of `_design.md`'s `## Shape decision`, each names the alternative
that lost, and contradicting one is a defect in this story rather than a design change.

1. **`query()` is not on `DecisionModel`.** The derivation is **`Boundary::query`**, on a *sealed*
   trait blanket-implemented for every `DecisionModel` and (in the slice-mate) macro-implemented for
   tuples of arity 2..=8. A *provided method* on `DecisionModel` lost because a caller can override
   it, and an overridden derivation is a hand-maintained query — exactly `DR-02`'s prohibition. A free
   `derive_query::<M>()` lost because a caller can ignore it and build their own.
   (`standards/rust/40-public-surface-and-evolution.md:73`, RS-40-2 — put a derivable convenience on
   the blanket ext trait.)
2. **`Boundary::query(&self) -> Result<Query, InvalidQuery>`**, and an *empty* `EVENT_TYPES` is a
   **compile error**, not a runtime one. Infallible is unreachable without an `unwrap`, because
   `QueryItem::new` is fallible (`crates/happenstance-core/src/query.rs:56`) and `happenstance-core`
   is frozen. Fallible *without* the const assertion lost too: it defers a compile-time-decidable
   mistake to the first read, which is DT-2's minimal-ceremony pole. The assertion is a `const` item
   evaluated per-monomorphisation (`standards/rust/61-compile-time-assertions.md:249`, RS-61-4).
3. **`DecisionModel::scope(&self) -> &Tags`** — the model *holds* validated tags. `Tags::from_pairs`
   is fallible and is the only way in (`crates/happenstance-core/src/tag.rs:304`), so the validation
   is paid **once**, in the constructor the caller already writes. `-> Tags` by value would force an
   `unwrap` inside an infallible signature; `&[(&str, &str)]` would revalidate on every read and move
   the error to the read.
4. **`DecisionModel: Clone`, not `Default`.** `Default` would force `scope`'s validated `Tags` into a
   default-constructible field and re-open the invalid-value hole the contract crate already names in
   its own words (`crates/happenstance-core/src/projection.rs:47-61`). `Clone` is what lets M3's
   command loop re-fold from the **pristine** model on retry (AC-U13).
5. **`DomainEvent::event_type(&self) -> EventType`, by value.** `-> &'static EventType` lost:
   `EventType` carries a `Cow<'static, str>`, so const promotion does not apply and the impl would
   have to index `EVENT_TYPES` **by position** — two places to get wrong, which is the hazard AC-001
   exists to close (`crates/happenstance-core/src/event.rs:38-58`, `:108`).

### Where the fallibility went — and the residual, which is a recorded defect, not a shrug

`Boundary::query` builds a `QueryItem` out of values that are *already* validated and still meets a
`Result`. This is the single most likely thing for an implementer to "fix" wrongly, so it is settled
before you start:

- **`Err` is kept and given a real meaning.** `InvalidQuery::UnconstrainedItem`
  (`crates/happenstance-core/src/error.rs:99-102`) reads as *"this boundary constrains nothing, which
  is `Query::all()` and must be said out loud"* — a refusal worth making.
- **The other route to it — an empty `EVENT_TYPES` — is a compile error** (decision 2 above).
- **`commit`/`commit_with` absorb the `Result`** into `CommandError::Boundary` so the first program
  writes no extra `?`. That absorption is M3's (`command-loop`); this story must leave the `Result`
  in place and must not pre-empt it.
- **No `unwrap`. No edit to `happenstance-core`.** The shortfall is **defect candidate D-1** —
  *`happenstance-core` has no infallible `QueryItem` constructor for pre-validated inputs* — already
  recorded by ADR-0020 and routed to a decision record under project AC-012's route, never a line edit
  of a frozen crate (project AC-A02; `_design.md`, *The residual*).

If implementing this story surfaces a *second* such shortfall, it is logged the same way — a defect
entry naming its clause ID — and handed to `defect-log-and-macros-verdict` (M7). It is never absorbed
quietly and never fixed under `crates/happenstance-core/src/**`.

### What lands here and what does not — the three slice seams, decided

The binding design lists items across milestones; three of them sit exactly on M2's edge, and each is
decided here with the alternative that lost. These are seam calls, not design changes.

- **`Codec` and `CodecError` land in this PR** (the trait and the error type only).
  `DomainEvent::encode<C: Codec>` / `decode<C: Codec>` are part of the signed-off `DomainEvent`
  signature, so the trait cannot compile without them. *Rejected: ship `DomainEvent` without
  `encode`/`decode` and add them in M3.* Adding a required method to an existing trait is the move
  RS-40-1 forbids (`standards/rust/40-public-surface-and-evolution.md`), and it would break a
  `Boundary` this story seals. **Not in this PR, and `codec-and-feature-forwarding`'s (M3):** `Json`,
  `Cbor`, `Postcard`, the `[features]` block that gates them, where ADR-0021 sites the codec tag, and
  the per-codec round-trip tests. `Codec::TAG` is a `&'static str` whatever ADR-0021 decides — the
  surface is invariant under that choice (`_design.md`, *Open, and deliberately not settled here*).
- **`Boundary`'s blanket impl for `DecisionModel` lands here; the tuple impls do not.**
  `(B1, B2)`…`(B1, …, B8)` and the internal `macro_rules!` behind them are the slice-mate
  `decision-model-composition`'s (`_storymap.md:52`). This story lands `Boundary`, `sealed::Sealed`,
  the blanket impl and the seal such that the slice-mate adds impls and nothing else.
- **The crate-root page's *first program* completes in M3, not here.** `_design.md`'s
  `## Composition` region 2 is the complete first program, and it calls `commit` — which does not
  exist until `command-loop`. So this story lands a **vocabulary doctest** (define the events, define
  the model, derive the query, assert it) plus the `compile_fail` companion on `DomainEvent`'s item
  page, and replaces the `DomainEvent` and `DecisionModel` roadmap bullets in place. *Rejected: hold
  the module-doc rewrite until M3.* AC-U03 forbids a reader meeting a roadmap bullet beside a real
  item, and a bullet that says "Planned" about a shipped trait is worse than either state.

Two further items the design lists are explicitly **not** claimed here, so that no item is orphaned
and none is claimed twice: `happenstance::testing::assert_domain_event`, `testing::given` and
`testing::Decision` are `given-when-then-dsl`'s (M4) — the whole `testing` module is created once, by
that story. This story covers the same residual with crate-internal unit tests over its own fixture
domain (see *Behavior and interfaces*, the agreement row).

### The one thing the compiler cannot do, stated so it is not discovered as a surprise

`EVENT_TYPES` ↔ `event_type()` agreement is **not** compiler-enforced and cannot be: a variant may
return a type absent from `EVENT_TYPES`, and no `const` sees the match arms
(`_design.md`, `## Shape decision`, the agreement row). What *is* enforced is the fold's
exhaustiveness over `Self::Event`, which is the half that matters for the DCB hazard. The residual is
tested, not asserted away, and it is also the measurement `RUNBOOK.md:524` reads AC-013's verdict
off — so do not "solve" it here with a macro. If the mapping ceremony feels excessive, that feeling is
the data point AC-013 wants, recorded at closeout, not a licence to add a derive
(`_design.md:1104-1111` records the design's own falsifiable prediction of 2.4:1).

### The mount, and why an unmounted trait is not this story

`crates/happenstance/src/lib.rs` is the crate's **only render path** (`_decomposition.md`,
*Composition roots*, item 1). Three constraints bind:

- **`pub use happenstance_core::*;` (`:75`) stays, and every new item lands beside it.** No new item
  may shadow a contract name — no `happenstance::Query`, no `happenstance::EventStore` of ours. The
  module doc already promises *"everything the contract crate exports is available here under the
  same paths"* (`:53-56`); a shadowing name is a silent breaking change to a published facade and an
  ambiguity in every existing doctest (AC-A01, and `_design.md` anti-pattern 14).
- **Module paths are private structure; the root is the surface.** Callers write
  `happenstance::DomainEvent`, never `happenstance::domain::DomainEvent`
  (`_design.md`, *Placement and re-export*).
- **The roadmap bullets are the render path, edited in place.** Bullets at `:41-46` (`DomainEvent`,
  `DecisionModel`) become intra-doc links to the real items, same order, same discriminator prose;
  the `Codec` bullet at `:38-40` stays a promise until M3 lands the codecs a reader can name
  (AC-U03; `_decomposition.md`, *Where the code lands*).

### The persona-journey slice

Activity **A2** of the project's backbone — *"Model a consistency boundary: define typed events, fold
a decision model"* — closing **Beat 1** of the journey *"Choose a contract before a database"*
(`_storymap.md:24`). The reader served is **P1**, the application author in their first hour, and the
audience constraint is in force: fluent in the domain, **new to idiomatic Rust**
(`CLAUDE.md`, *Who you are working with*). That is what makes AC-U15 a hard requirement rather than
politeness — sealing, blanket impls and a per-monomorphisation `const` are each unusual constructs,
and each carries the alternative that lost in its own doc comment, once
(`standards/rust/70-rustdoc-obligations.md`, RS-70-5).

### What this story must not do

Write `commit`, `Retry`, `Committed` or `CommandError` (M3's). Write the tuple `Boundary` impls
(the slice-mate's). Write `Json`/`Cbor`/`Postcard`, add a codec feature, or decide where the codec tag
lives (M3's, governed by ADR-0021). Create `happenstance::testing` (M4's). Touch
`crates/happenstance-core/src/**`, `spec/SPECIFICATION.md`, `examples/`, or `xtask/`. Add
`#[async_trait]` — nothing in this story is async at all, which is the cheapest possible way to
satisfy that constraint. Re-open a signed-off `_design.md` row.

## Integration contract

- **Archetype**: `capability` — a user-observable slice. After this PR an application author can
  define typed events and a decision model against `happenstance` alone and see the derived query,
  with no adapter and no database.
- **Slice / milestone**: **M2 `typed-vocabulary`** (`_storymap.md:51-52`, merge order step 2 at
  `:116-117`). **Slice-mate**: `decision-model-composition`, implemented in the same context and
  mounted as one integrated surface — it adds the tuple `Boundary` impls to the seal this story
  builds, and the two land at the crate root together.
- **Mount point**: **`crates/happenstance/src/lib.rs`** — the composition root and the crate's only
  render path (`_decomposition.md`, *Composition roots*, item 1). Every new item is `pub use`d there
  beside the surviving `pub use happenstance_core::*;` (`:75`) **and** linked from the module doc; the
  `DomainEvent` and `DecisionModel` bullets at `:41-46` are rewritten in place. A trait that compiles
  in a module nothing re-exports is unmounted by construction — it is absent from the crate's item
  table, unreachable by intra-doc link, and invisible to the `cargo add` user this project exists to
  serve.
- **Wires into**:
  - `crates/happenstance-core/src/event.rs` — `EventType::from_static` (`:108`, `const`, which is why
    `const EVENT_TYPES` needs no derive today), `EventType`'s `Cow` payload (`:38-58`), `Event`
    (`:321`) and `SequencedEvent` (`:484-496`), which `Boundary::absorb` reads.
  - `crates/happenstance-core/src/query.rs` — `QueryItem::new` (`:56`, fallible) and
    `Query::from_items`, the constructors the derivation is built out of and does not change.
  - `crates/happenstance-core/src/tag.rs` — `Tags::from_pairs` (`:304`), the only (fallible) way in,
    which is why `scope` returns `&Tags`.
  - `crates/happenstance-core/src/error.rs` — `InvalidQuery` (`:93-105`), whose `UnconstrainedItem`
    variant (`:99-102`) the derivation's `Err` arm re-reads.
  - `crates/happenstance/Cargo.toml` — the manifest, for the `serde`/`thiserror` dependencies and the
    `[package.metadata.docs.rs]` block the crate does not have today (`_design.md`, *Manifest
    changes*). Both `serde` and `thiserror` are already `[workspace.dependencies]`
    (`Cargo.toml:48`, `:70`), so no new external crate enters the graph decision.
  - `standards/rust/40-public-surface-and-evolution.md` (RS-40-1, RS-40-2 at `:73`),
    `standards/rust/13-sealing-and-exhaustiveness.md` (the seal),
    `standards/rust/61-compile-time-assertions.md` (RS-61-4 at `:249`),
    `standards/rust/30-error-taxonomy.md` (RS-30-2, a typed `#[source]`, never a `String`),
    `standards/rust/62-doctests-and-harnesses.md` (RS-62-1, pair every `compile_fail` with a compiling
    one and do not trust its error code) and `standards/rust/70-rustdoc-obligations.md` (RS-70-2,
    RS-70-5) — the house rules the shape is built on. Pull these three to five atoms, not the corpus.
- **Renders surfaces**: **`crate-root-rustdoc`** (partially, and this story is the first to touch it)
  — regions 4 *The vocabulary* and, for the two items it lands, region 3's discriminator prose stay as
  designed while the `DomainEvent`/`DecisionModel` bullets become links. Region 2 — the *first
  program* — is **staged, not completed**: `first-program-doctest` renders in full only when
  `command-loop` (M3) lands `commit`, and this story ships the vocabulary doctest plus the
  `compile_fail` companion `_design.md`'s `## The doctest` places on `DomainEvent`'s item page. The
  page's density budget binds every line this story writes: doc prose ≤ 80 columns, code inside a doc
  fence ≤ 72, first sentence of every doc comment ≤ 80 characters and a complete claim, public item
  identifier ≤ 24 characters (`_design.md`, *Density budget*). Not rendered here: `crate-readme`,
  `worked-example-transcript`, `dsl-failure-message`, `compile-fail-diagnostic`.
- **Conformance rule(s)**: **none, and deliberately.** `DomainEvent`, `DecisionModel`, `Boundary` and
  the derived `Query` live entirely above the port; no store can observe whether a query was derived
  or hand-written, so a rule added to `crates/happenstance-testkit/src/suite.rs` here would be one no
  adapter could fail — decorative by this repository's own standard (`CLAUDE.md`, *A rule that no
  adapter can fail is decorative*). The instruments are unit tests beside the code, the two doctests,
  and the compile-time `const` assertion (`_decomposition.md`, Testing brief, AC-001 and AC-014 rows).
- **Clause(s)**: **none discharged, none amended.** `spec/SPECIFICATION.md`'s namespaces are `VT-`,
  `ES-`, `PS-`, `SY-`, `WF-`, `CF-`; there is no typed-layer namespace, so this surface's only
  contract is `_design.md` plus ADR-0020 (`_design.md`, header: *The specification is silent about
  this layer*). `VT-3` (payload and metadata stay opaque to every store) is the clause this story must
  not disturb: payloads remain `Bytes` at the port and all encoding stays above it. Any defect found
  routes to a decision record, never a clause edit (project AC-A02, AC-012).
- **Advances DoD scenario**: initiative **DoD 2** — *"@smoke — the compiler protects the domain"*
  (`initiative.md:363-365`). This story builds the protection (the exhaustive fold over a domain enum
  and the derived query); `compile-fail-proof-artefact` (M6) is what *observes* it, and
  `worked-example-on-typed-layer` (M6) carries it into initiative **DoD 1**. At project grain it is
  the first half of **AC-001** (the second half, the record, is ADR-0020's) and the built consequence
  of **AC-014**.

## PR boundary

**In this PR**

- `crates/happenstance/src/**` — the new modules holding `DomainEvent`, `DecisionModel`, the sealed
  `Boundary` with its blanket impl, `Codec`/`CodecError`, the private `sealed` module and the
  per-monomorphisation `const` assertion, plus their unit tests.
- `crates/happenstance/src/lib.rs` — the mount: `pub use` of every new item beside the glob, the
  module-doc edit (two bullets → intra-doc links), the vocabulary doctest and
  `#![cfg_attr(docsrs, feature(doc_cfg))]`.
- `crates/happenstance/Cargo.toml` — the `serde` and `thiserror` dependencies (both already
  `[workspace.dependencies]`) and the `[package.metadata.docs.rs]` block, which this crate lacks
  while `happenstance-core` and `happenstance-testkit` both have it. Landing it with the first
  items rather than with M3's first *gated* item means no gated item ever renders on docs.rs
  without its badge (`_design.md`, *Manifest changes*; AC-U14).
- `crates/happenstance/tests/**` if a behaviour is better proven outside the crate's own unit scope.
- This story's own backlog folder — `_ledger.md` and the implementation report.

**Explicitly not in this PR**

- **`crates/happenstance-core/src/**`, in any form.** Everything below this crate is `[FROZEN]` and
  already compiles; a convenience edit here is the unrecorded contract change AC-A02 forbids. The
  residual `Result` is the standing temptation and its answer is D-1, not a new constructor.
- **`commit`, `commit_with`, `Retry`, `Committed`, `CommandError`** — `command-loop`'s (M3).
- **The tuple `Boundary` impls and the internal composition `macro_rules!`** —
  `decision-model-composition`'s (the slice-mate).
- **`Json` / `Cbor` / `Postcard`, the codec `[features]`, and the codec tag's home** —
  `codec-and-feature-forwarding`'s (M3), under ADR-0021.
- **`happenstance::testing` (`given`, `Given`, `Decision`, `assert_domain_event`)** —
  `given-when-then-dsl`'s (M4).
- **`examples/course-subscriptions/**`** — `worked-example-on-typed-layer`'s (M6). The example keeps
  compiling against the unchanged glob re-export throughout this PR; nothing here may require it to
  change.
- **`xtask/**`, `spec/SPECIFICATION.md`, `CHANGELOG.md`, `crates/happenstance/README.md`** — the gate
  steps, clause verdicts and release surfaces belong to M5–M7. No new gate step is added here.
- Re-deciding any row of the signed-off `_design.md`, or adding a derive macro in answer to the
  ceremony ratio (AC-013's verdict is the project closeout's).

**Allowed globs** — `redkiln verify --grain story` reads the first fenced block under this heading and
fails on any file changed outside it.

```
crates/happenstance/src/**
crates/happenstance/tests/**
crates/happenstance/Cargo.toml
Cargo.lock
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/domain-event-and-decision-model/**
```

**`Cargo.lock` was added to the fence on 2026-08-16, and it is a repair rather than a
widening.** The fence already authorises `crates/happenstance/Cargo.toml` — and this story
is *required* to add `serde` and `thiserror` there — while cargo rewrites the workspace
lockfile as the mechanical consequence of any dependency change. The boundary therefore
permitted the cause and forbade the effect, and no implementation could satisfy both;
`redkiln verify --grain story` rejected this story's checkpoint on `Cargo.lock` alone, a
three-line delta that is exactly those two dependencies. Four sibling specs in this project
— `codec-and-feature-forwarding`, `projection-trait-and-runner`,
`compile-fail-proof-artefact` and `publish-0-2-0-alpha-1` — already carry the entry, so the
omission was an inconsistency inside one planning pass, not a policy. The addition admits
the lockfile and nothing else: `[workspace.dependencies]` and every other manifest stay
outside.

**Merge DoD one-liner** — `DomainEvent`, `DecisionModel`, the sealed `Boundary` and the
`Codec`/`CodecError` vocabulary exist exactly as `_design.md`'s `## Signatures` writes them, are
`pub use`d at the crate root beside the surviving glob with the two roadmap bullets replaced in place
by resolving intra-doc links, a divergent query has nowhere to live and an empty `EVENT_TYPES` fails
to compile; `cargo xtask affected --base main` and `cargo xtask ci --fast` are green, with
`cargo doc` warning-free under default *and* `--no-default-features`.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| An application declares its event set **once**, as a `const` | `pub trait DomainEvent: Sized` with `const EVENT_TYPES: &'static [EventType]`, `fn event_type(&self) -> EventType`, `fn tags(&self) -> Tags`, `fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError>` and `fn decode<C: Codec>(codec: &C, event_type: &EventType, data: &Bytes) -> Result<Self, CodecError>`. No derive and no macro is needed today because `EventType::from_static` is already `const` | `_design.md` `## Signatures`; `crates/happenstance-core/src/event.rs:108` |
| `event_type()` returns by value, and that is deliberate | A `Cow::Borrowed` clone, no allocation. `-> &'static EventType` would force the impl to index `EVENT_TYPES` **by position**, re-creating the two-places hazard this story closes; the rejected alternative is named once in the trait's own doc comment (RS-70-5) | `crates/happenstance-core/src/event.rs:38-58`; `_design.md` `## Shape decision`, `DomainEvent::event_type` row |
| The fold is exhaustive over a domain enum, and the compiler enforces it | `pub trait DecisionModel: Clone` with `type Event: DomainEvent`, `fn scope(&self) -> &Tags` and `fn apply(&mut self, event: Self::Event)`. `apply` takes the enum by value, so a caller's `match` is exhaustive and a new variant is a compile error in the caller's own code | `_design.md` `## Signatures`; project `DR-01`; `RUNBOOK.md:3993-3998` |
| Validation is paid once, in the constructor the caller already writes | `scope(&self) -> &Tags`: the model *holds* validated `Tags`. `Tags::from_pairs` is fallible and is the only way in, so no infallible signature ever hides an `unwrap` | `crates/happenstance-core/src/tag.rs:304`; `_design.md` `## Shape decision`, `DecisionModel::scope` row |
| `Clone`, not `Default`, is the supertrait | `Default` would force validated `Tags` into a default-constructible field and re-open the invalid-value hole the contract crate condemns in its own words; `Clone` is what lets M3's loop re-fold from the pristine model on retry | `crates/happenstance-core/src/projection.rs:47-61`; AC-U13 |
| The query is **derived**, and there is nowhere to put a hand-maintained one | `pub trait Boundary: sealed::Sealed` with `fn query(&self) -> Result<Query, InvalidQuery>`, blanket-implemented for every `DecisionModel` from `Event::EVENT_TYPES` + `scope()`. Sealed via a private supertrait so the blanket impl and (later) the tuple impls stay the only implementors | `_design.md` `## Signatures`, `## Visibility and stability` (`Boundary` row); `standards/rust/40-public-surface-and-evolution.md:73` (RS-40-2); `standards/rust/13-sealing-and-exhaustiveness.md` |
| The derived query is **inspectable**, not private machinery | `query()` returns a value a test can assert on and a caller can print — the non-occlusion rule. A derivation reachable only inside a `read` call hides its own filter, which is the hazard ADR-0020 closes | AC-U11 (`_decomposition.md`, UX brief); `_design.md` `## Shape decision`, `Boundary::query` row |
| An empty `EVENT_TYPES` fails to **compile** | A `const` item evaluated per-monomorphisation, so the reader meets a post-monomorphisation const-eval error naming the event-type set rather than a trait-resolution error or a first-read failure | `standards/rust/61-compile-time-assertions.md:249` (RS-61-4); `_design.md` `## The doctest`, *The `compile_fail` companion* |
| The `compile_fail` fence is spelled bare and paired | `compile_fail`, never `compile_fail,E0080`: rustdoc on 1.97.1 silently ignores an error code it cannot match, so the stricter-looking spelling is the weaker check. Paired with a compiling example on the same item page. **This is not AC-002's instrument** — that is `trybuild`'s, blocked, and M6's | `crates/happenstance-core/src/event.rs:95-106` (the same warning, about the same construct); `standards/rust/62-doctests-and-harnesses.md` RS-62-1 |
| The residual `Result` keeps its meaning and gains no `unwrap` | The `Err` arm is `InvalidQuery::UnconstrainedItem` = *"this boundary constrains nothing, which is `Query::all()` and must be said out loud"*, documented under `# Errors` by condition rather than by type. Defect candidate **D-1** stays routed to a decision record | `crates/happenstance-core/src/error.rs:99-102`; `crates/happenstance-core/src/query.rs:56`; `_design.md`, *The residual* |
| A nominated event is folded through the codec, and a real disagreement is an error | `fn absorb<C: Codec>(&mut self, event: &SequencedEvent, codec: &C) -> Result<(), CodecError>` on `Boundary`: decode via `DomainEvent::decode`, then `apply`. An event the query never nominated is skipped **by nomination**, not by a silent type check; an event that *was* nominated and cannot be decoded returns `CodecError::UnknownEventType`, because that is a genuine `EVENT_TYPES`/fold disagreement | `_design.md` `## Signatures`, `## The states the API must express` (*Absent*); `crates/happenstance-core/src/event.rs:484-496` |
| `absorb` is on `Boundary` from birth, not added later | `Boundary` is sealed and the crate publishes an alpha this project cuts; adding a required method to an existing trait is the move RS-40-1 forbids, so the sealed trait ships complete even though its only callers land in M3 and M5 | `standards/rust/40-public-surface-and-evolution.md` (RS-40-1); `_design.md` `## Items` |
| The encoding vocabulary exists, with no codec yet | `pub trait Codec` with `const TAG: &'static str`, `encode<T: serde::Serialize>` and `decode<T: serde::de::DeserializeOwned>`; `#[non_exhaustive] pub enum CodecError` with `UnknownTag`, `UnknownEventType`, `Encode` and `Decode`, each boxed error carried as a typed `#[source]` — never a `String`, and never an associated `Error` type (which would add a third type parameter to `CommandError`, `absorb`, `Decision` and the runner) | `_design.md` `## Signatures`, `## Shape decision` (`Codec::Error` row); `standards/rust/30-error-taxonomy.md` RS-30-2 |
| `serde` arrives in `happenstance`, and only there | The typed layer's job is encoding, so `serde` becomes a real dependency of this crate — which ADR-0003 permits precisely because it constrains `happenstance-core`, and ADR-0006 made this the typed layer. `happenstance-core`'s default features are untouched, and the existing `serde` **feature** in `crates/happenstance/Cargo.toml:25` keeps its current meaning (forwarding the contract crate's envelope `serde`); it is not repurposed, because a feature only ever adds | project `DR-04`; AC-A04; `crates/happenstance/Cargo.toml:22-26`; `Cargo.toml:48` |
| `EVENT_TYPES` ↔ `event_type()` agreement is tested, not asserted away | Not compiler-enforceable: a variant may return a type absent from `EVENT_TYPES` and no `const` sees the match arms. Covered here by a crate-internal unit test over this story's own fixture domain asserting every variant's `event_type()` is in `EVENT_TYPES`. The **public** `testing::assert_domain_event` is M4's; this story does not create `happenstance::testing` | `_design.md` `## Shape decision`, the agreement row; `_storymap.md:56` |
| Every new item is reachable from the crate root | `pub use` at the crate root beside `pub use happenstance_core::*;`; module paths stay private structure. No new item shadows a contract name (`Query`, `EventStore`, `Tags`, `Event`) — the crate's own doc promises the contract's paths are unchanged | `crates/happenstance/src/lib.rs:53-56`, `:75`; AC-A01; `_design.md` anti-pattern 14 |
| The module doc stops promising and starts linking | The `DomainEvent` and `DecisionModel` bullets at `crates/happenstance/src/lib.rs:41-46` become intra-doc links to the real items, in the same order and with the same discriminator prose. The `Codec`, command-loop and runner bullets stay promises until their stories land | AC-U03 (`_decomposition.md`, UX brief); `_design.md` `## Composition`, region 4 |
| Every intra-doc link resolves in **every** feature configuration | The mock found that links to gated items warn when the gate is off. Nothing this story adds is gated, so the rule is cheap here and must stay true: `cargo doc` is warning-free under default features and under `--no-default-features` | `standards/rust/70-rustdoc-obligations.md` RS-70-2; `_design.md` `## Mock`, finding 2 |
| Docs.rs renders gates from the moment gates exist | `[package.metadata.docs.rs] all-features = true` + `rustdoc-args = ["--cfg", "docsrs"]` in `crates/happenstance/Cargo.toml` and `#![cfg_attr(docsrs, feature(doc_cfg))]` in `lib.rs` — present in both sibling crates, absent from this one today | `crates/happenstance-core/Cargo.toml:54-56`; `_design.md`, *Manifest changes*; AC-U14 |
| Unusual constructs carry the alternative that lost | Sealing, the blanket impl, the per-monomorphisation `const` and the by-value `event_type()` each state in their own doc comment what the alternative was and why it lost — once, where the reader is. The audience is fluent in the domain and new to idiomatic Rust | `standards/rust/70-rustdoc-obligations.md` RS-70-5; `CLAUDE.md`, *Who you are working with*; AC-U15 |
| Every fallible public function documents its **conditions** | An `# Errors` section naming what makes it fail, not the error type's name | `standards/rust/70-rustdoc-obligations.md`; `_design.md` `## Signatures` (the `# Errors` blocks as written) |
| Nothing here is async, and nothing binds a port | No `#[async_trait]` (the constraint is satisfied by there being no future at all), no `EventStore`/`SendEventStore` bound in any signature this story adds, and no change to `read`'s shape. The four `wasm32` gate steps are unaffected; the fifth step compiling `happenstance` itself is `edge-flavour-and-wasm-claim`'s (M7) | `CLAUDE.md` binding constraints 1, 3, 4; AC-A06; `_storymap.md:62` |
| Everything before the append stays pure | Defining events, folding a model, deriving a query and encoding a payload perform no I/O and mutate no store. This story adds no function that touches a store at all | AC-U12 (`_decomposition.md`, UX brief) |

## Data and migrations

**N/A — no schema, no store, no persisted data, no wire format change.** This story adds four traits,
one error enum and a private sealed marker to a library crate. `MemoryEventStore` is the only store in
play anywhere in this project (`RUNBOOK.md:3977-3979`), and this story does not touch it.

Three migration-shaped obligations are worth naming so they are not mistaken for absent ones:

- **The payload's on-the-wire shape is not decided here.** `DomainEvent::encode`/`decode` are generic
  over `Codec`, and which bytes a `Json` codec actually writes — plus where the codec tag is
  recorded, `Event::metadata` (`crates/happenstance-core/src/event.rs:379`) or `Tags` — is
  ADR-0021's and lands with `codec-and-feature-forwarding` (M3). The one constraint that binds
  regardless, and that this story must not undermine: **no adapter may need to understand the tag**,
  and payloads stay opaque `Bytes` at the port (`VT-3`; ADR-0003).
- **`EventType` version suffixes and upcasting are ADR-0021's too.** Nothing in this story's
  signatures presumes either answer; `const EVENT_TYPES` is a list of `EventType` values whatever
  naming convention that record settles on.
- **The manifest change is additive and reversible.** Adding `serde`/`thiserror` and the docs.rs
  metadata to `crates/happenstance/Cargo.toml` changes no existing feature's meaning — in particular
  `default-features = false` continues to mean exactly what it means for `happenstance-core`, which
  is the failure mode the manifest's own comment (`crates/happenstance/Cargo.toml:15-19`) exists to
  prevent.

The one genuinely one-way act in this project is `cargo publish` (M7), and nothing this story writes
is published until then — which is precisely the window in which a signature can still be changed for
free.

## Acceptance criteria

Eight criteria, each written from the reader's intent rather than from the item that satisfies it.
The reader is **P1, the application author in their first hour** — *"model a consistency boundary
once, against a contract, and defer which database to a decision they can revisit"*, afraid of
*"being the one who discovers a contract defect in production"* (`_decomposition.md`, UX brief,
persona table) — walking **Activity A2 / Beat 1** of the journey *"Choose a contract before a
database"* (`_storymap.md:24`). **P4, the evaluator**, appears in AC-006 and AC-007, because their
whole encounter is one bounded sitting on a rendered page and *"they do not get a second pass"*.

The two project criteria this story traces to are **AC-001** (*the fold and the query cannot
disagree*) and **AC-014** (*DT-2 resolved on the record* — the built half; the record itself is
`adr-0020-fold-query-agreement`'s). AC-001 is carried by rows AC-001, AC-002 and AC-008 below;
AC-014 is carried by AC-003. Every other row is what makes those two reachable by an actual reader
rather than merely true of the type checker.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** P1 has a domain — courses and subscriptions — and wants to model one consistency boundary before choosing a database, **WHEN** they write one enum of events and one struct that folds them, implementing `DomainEvent` (`const EVENT_TYPES: &'static [EventType]`, `event_type(&self) -> EventType` **by value**, `tags(&self) -> Tags`) and `DecisionModel` (`type Event: DomainEvent`, `scope(&self) -> &Tags`, `apply(&mut self, event: Self::Event)`, supertrait `Clone` and **not** `Default`), **THEN** the fold is a `match` over their own enum that the compiler makes exhaustive — adding a variant is an error in *their* file — the model holds its own validated `Tags` so `Tags::from_pairs`' fallibility is paid once in the constructor they already write, and every signature matches `_design.md`'s `## Signatures` row for row with no store, no adapter and no database anywhere in the program. | Unit tests in `crates/happenstance/src/` over this story's own fixture domain: `fold_is_exhaustive_over_domain_enum` (a two-variant enum whose `apply` has no `_ =>` arm), `scope_returns_the_models_own_tags` and `decision_model_is_clone_not_default` (a `fn assert_clone<M: DecisionModel + Clone>()` witness). Run by `cargo test -p happenstance --all-features`, inside `cargo xtask affected --base main` and `cargo xtask ci --fast`. Signature conformance is reviewed line-by-line against `_design.md` `## Signatures` at the report gate. |
| AC-002 | **GIVEN** P1's fear is a defect they discover in production, and today a DCB handler names its event set twice — once in the `Query` at `examples/course-subscriptions/src/main.rs:114-125`, once in the fold at `:128-173`, with nothing checking the two agree — **WHEN** they ask their model which events it reads, **THEN** they call `Boundary::query(&self) -> Result<Query, InvalidQuery>` and get a `Query` **derived** from `Self::Event::EVENT_TYPES` plus `scope()`, as an ordinary value they can bind, print and assert on; and **WHEN** they try to supply their own divergent query instead, **THEN** there is nowhere to put it — `Boundary` is sealed behind a private supertrait, blanket-implemented for every `DecisionModel`, so the second place the event set could be named does not exist in the type system. | Unit tests in `crates/happenstance/src/`: `query_is_derived_from_event_types_and_scope` (the derived `Query`'s items equal one `QueryItem` per `EVENT_TYPES` entry carrying `scope()`'s tags — asserted against the values the model actually declared, never against literal positions); `query_is_a_value_a_test_can_assert_on` (binds the result and inspects it outside any read); and a `compile_fail` doctest `boundary_cannot_be_implemented_outside_the_crate` paired with the compiling blanket-impl example on the same item page (RS-62-1). |
| AC-003 | **GIVEN** DT-2 resolved toward **explicit declaration** — more caught at build time, ceremony paid in `const` items — and P1 is new to idiomatic Rust, **WHEN** they declare a `DomainEvent` with an empty `EVENT_TYPES`, **THEN** the program does not compile and the diagnostic is a post-monomorphisation const-eval error naming the event-type set, not a trait-resolution error and not a failure on their first read; and **WHEN** their boundary is well-formed but constrains nothing, **THEN** `query()` returns `Err(InvalidQuery::UnconstrainedItem)` — a refusal stated out loud rather than silently widened to `Query::all()` — with no `unwrap`, no `expect`, and no edit anywhere under `crates/happenstance-core/src/**`. | A `compile_fail` doctest on `DomainEvent`'s item page spelled **bare** `compile_fail` (never `compile_fail,E0080` — rustdoc on 1.97.1 silently ignores an unmatched code, per `crates/happenstance-core/src/event.rs:95-106`), paired with a compiling example. Unit test `unconstrained_boundary_is_an_error_not_query_all` asserting the `Err` arm. Absence of `unwrap`/`expect` is enforced by `clippy -D warnings` inside `cargo xtask affected`; absence of core edits by `git diff --name-only main` showing no path under `crates/happenstance-core/`. |
| AC-004 | **GIVEN** P1 wants their events to be Rust values rather than `format!("…").into_bytes()`, and their log will one day hold event types this model does not know, **WHEN** they encode through `DomainEvent::encode<C: Codec>` / `decode<C: Codec>` and fold a read batch through `Boundary::absorb<C: Codec>(&mut self, &SequencedEvent, &C)`, **THEN** an event the query never nominated is skipped **by nomination** rather than by a silent type check, an event that *was* nominated and cannot be decoded returns `CodecError::UnknownEventType` because that is a genuine `EVENT_TYPES`/fold disagreement, and every codec failure carries the underlying error as a typed `#[source]` on a `#[non_exhaustive]` enum — never a `String`, and never an associated `Error` type that would add a third parameter to every downstream signature. | Unit tests in `crates/happenstance/src/` against a stub `Codec` fixture: `absorb_skips_an_unnominated_event`, `absorb_returns_unknown_event_type_when_a_nominated_event_cannot_be_decoded`, `absorb_applies_a_decoded_event_to_the_fold`, and `codec_error_carries_a_typed_source` (a `fn assert_source<E: core::error::Error>()` witness plus an `Error::source()` chain assertion). Reviewed against `standards/rust/30-error-taxonomy.md` RS-30-2. |
| AC-005 | **GIVEN** P4 has one bounded sitting on docs.rs and P1 arrives via `cargo add happenstance`, **WHEN** they land on the crate's page and write their first `use`, **THEN** every item this story adds is reachable as `happenstance::DomainEvent`, `happenstance::DecisionModel`, `happenstance::Boundary`, `happenstance::Codec`, `happenstance::CodecError` — `pub use`d at the crate root **beside** the surviving `pub use happenstance_core::*;` (`crates/happenstance/src/lib.rs:75`), with module paths kept private structure and **no** new item shadowing a contract name (`Query`, `EventStore`, `Tags`, `Event`) — every intra-doc link on the page resolves under default features **and** under `--no-default-features`, and the manifest carries `[package.metadata.docs.rs]` with `lib.rs` carrying `#![cfg_attr(docsrs, feature(doc_cfg))]` so no future gated item can ever render on docs.rs without its badge. | `cargo doc -p happenstance --no-deps` and `cargo doc -p happenstance --no-deps --no-default-features`, both **warning-free** (rustdoc's `broken_intra_doc_links` is deny-by-gate through `cargo xtask ci --fast`'s docs step). A unit test `contract_names_are_not_shadowed` asserting `happenstance::Query` and `happenstance::EventStore` still resolve to the `happenstance_core` items (a type-equality witness, e.g. `fn same<T>(_: T) {}` over both paths). `crates/happenstance/Cargo.toml` reviewed against `crates/happenstance-core/Cargo.toml:54-56`. |
| AC-006 | **GIVEN** the crate-root page today tells a reader that `DomainEvent` and `DecisionModel` are *"Planned, and specified in `spec/SPECIFICATION.md`"* (`crates/happenstance/src/lib.rs:41-46`), **WHEN** a reader lands on that page after this PR, **THEN** those two bullets have been rewritten **in place** — same region, same order, same discriminator prose, the bold lead-in term now an intra-doc link to the real item, with the remaining three bullets left as honest promises — so the reader is never shown a roadmap entry beside the shipped item it describes and is never sent to a second page to find it; and every item this story lands carries **composed presentation**, not bare markup: a first-sentence summary that is a complete claim, an `# Errors` section on every fallible function naming the *condition* rather than the type, and — on each of the four unusual constructs (the seal, the blanket impl, the per-monomorphisation `const`, the by-value `event_type()`) — the alternative that lost, named once, where the reader is. | Human review of the rendered page at the report gate against `_design.md` `## Composition` region 4 and anti-patterns 1, 2 and 14, plus `_decomposition.md`'s AC-U03 and AC-U15. Mechanically: `rg -n "Planned, and specified" crates/happenstance/src/lib.rs` must still match (the three surviving bullets), and `rg -n "DomainEvent|DecisionModel" crates/happenstance/src/lib.rs` must show both terms only as intra-doc links; the links actually resolving is AC-005's warning-free `cargo doc`. `cargo xtask ci --fast`'s docs step is the gate. |
| AC-007 | **GIVEN** P4 reads the page in a 1024x768 window and P1 copies the first fence they see, **WHEN** the vocabulary doctest and the item pages render, **THEN** the page fits the budget it was designed to: doc-comment prose ≤ **80** columns, code inside a doc fence ≤ **72** columns (so the fence **never** scrolls horizontally at 1024px), the doctest ≤ **35** visible lines with ≤ **2** hidden lines and those only harness (a runtime wrapper and an `Ok::<(), E>(())` closer, never API), first sentence of every doc comment ≤ **80** characters and a complete claim, every public identifier this story adds ≤ **24** characters, and the crate-root module doc ≤ **130** lines total — so nothing this story authors renders as a truncated item-table row or as a fence a reader must scroll. | A crate-internal test `doc_density_budget_holds` reading `crates/happenstance/src/**` with `include_str!`/`env!("CARGO_MANIFEST_DIR")` and asserting the four line/column budgets over `///` and `//!` lines and over fenced-block bodies (the same file-reading-lint shape `cargo xtask lints` already uses); plus human review of the rendered page against `_design.md` `## Density budget` and anti-patterns 3, 4 and 5. **Anti-pattern 5 is scoped to this story's own items** — see *Clarifications*. |
| AC-008 | **GIVEN** P1 wants to build a boundary, inspect it and throw it away without consequence, and **GIVEN** the one agreement the compiler cannot check is that every variant's `event_type()` is a member of `EVENT_TYPES`, **WHEN** they define events, fold a model, derive a query and encode a payload, **THEN** nothing performs I/O, nothing touches a store, no function this story adds takes an `EventStore` at all, and no value is mutated outside the caller's own `&mut` — the append remains the single irreversible act and it is not in this PR; and **THEN** the residual agreement is *tested* rather than asserted away, by a crate-internal test over this story's fixture domain proving every variant's `event_type()` is in `EVENT_TYPES`, with the ceremony cost left recorded and un-"fixed" (no derive macro is added here). | Unit tests in `crates/happenstance/src/`: `every_variant_event_type_is_declared` (iterate the fixture domain's variants, assert membership in `EVENT_TYPES` — the public `testing::assert_domain_event` is M4's and is **not** created here) and `nothing_in_the_vocabulary_touches_a_store` (a compile-level witness: the module's public functions are callable in a `#[test]` with no runtime and no store constructed). Purity is additionally evidenced by there being no `async fn` and no `EventStore`/`SendEventStore` bound in the diff — `rg -n "async fn|SendEventStore" crates/happenstance/src/` returns nothing new. |

## Interaction quality

RFC §6.7/D6, in the medium this project actually has. There is no screen: the "interface" is a **type
surface plus one rendered text surface** (`_decomposition.md`, UX brief, *There is no screen*), so
each invariant below is the library reading of its web original, taken from the signed-off
`_design.md` this story renders a part of. **Every invariant that applies is already an `AC-###` row
in the table above** — this section only says which row carries it and how it is checked. Nothing
here is a new obligation.

**STATE invariants.**

| Invariant (library reading) | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** — the roadmap bullets are *edited where they stand*, keeping region, order and discriminator prose, rather than a new section appearing beside a stale one (`_design.md` `## Composition` region 4; AC-U03) | AC-006 | human review of the rendered page against anti-pattern 1; `rg` assertions on `lib.rs` |
| **Non-occlusion — a filter must not hide what it filters.** The derived `Query` is a value a caller binds, prints and asserts on, never private machinery inside a `read` call (AC-U11) | AC-002 | `query_is_a_value_a_test_can_assert_on` |
| **No second page for a first task** — every type in the vocabulary doctest resolves from `happenstance`'s own root; application authors are never pointed down to `happenstance-core` to finish (AC-U02) | AC-005 | warning-free `cargo doc` in both feature configurations; human review of the doctest's `use` line |
| **Preserved state across retry** — `Clone`, not `Default`, is the supertrait precisely so M3's loop re-folds from the **pristine** model rather than a mutated one (AC-U13). This story preserves the property; M3 exercises it | AC-001 | `decision_model_is_clone_not_default` |
| **Preserved scroll** — literal here: at 1024px the first fence must not acquire a horizontal scrollbar, which is what the 72-column in-fence budget buys (anti-pattern 3) | AC-007 | `doc_density_budget_holds` + rendered-page review |
| **Reversibility** — everything before the append is pure and discardable; the manifest change is additive and does not alter what `default-features = false` means (AC-U12; AC-A04) | AC-008, AC-005 | `nothing_in_the_vocabulary_touches_a_store`; manifest diff review |
| **Reachability without a search engine** (the CLI/library reading of keyboard reachability) — every item reachable from the crate root by intra-doc link, no link resolving in only some feature configurations, no interactive or TTY step anywhere (AC-U14, RS-70-2) | AC-005 | `cargo doc` under default **and** `--no-default-features`, both warning-free |

**COMPOSITION invariants**, taken from the signed-off `_design.md` (Approved 2026-08-12, `## Sign-off`).
This story renders part of the `crate-root-rustdoc` surface, so these bind.

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all.** Every public item ships composed rustdoc — a complete-claim summary sentence, an `# Errors` block by condition on every fallible function, and the rejected alternative on each unusual construct. A trait that compiles with no doc comment satisfies every type-level assertion in this spec and still fails this row | AC-006 | rendered-page review against RS-70-2/RS-70-5 and `_design.md` `## Signatures`' `# Errors` blocks |
| **Composition and placement.** Root re-export beside the surviving glob; module paths stay private structure; the bullets stay in region 4 in their existing order (`_design.md` `## Placement and re-export`, `## Composition`) | AC-005, AC-006 | `contract_names_are_not_shadowed`; page review |
| **Transience.** `DomainEvent` and `DecisionModel` are **persistent chrome** — named on the landing page. `Boundary`, `CodecError` are **revealed** — item pages reached from the signature that returns them, never promoted onto the landing page (`_design.md` `## Transience policy`) | AC-006 | page review: the landing page names exactly the persistent-chrome items this story lands |
| **Density budget, with its real numbers.** 80 / 72 / ≤ 35 visible + ≤ 2 hidden / ≤ 80-character first sentence / ≤ 24-character identifier / ≤ 130-line module doc (`_design.md` `## Density budget`) | AC-007 | `doc_density_budget_holds` |
| **Hierarchy.** The fence is primary by position and by being the only fence in its region; the bold lead-in term *is* the link, so nothing is emphasised that is not also reachable; features and adapter pointers stay recessive and last (`_design.md` `## Hierarchy`) | AC-006, AC-007 | page review |
| **Named anti-patterns** 1 (a "Planned" list survives), 2 (something other than one sentence above the first fence), 3 (the fence scrolls at 1024px), 4 (the rendered program is materially shorter than the designed one — ceremony hidden behind `# `), 5 (an item-table row ending in an ellipsis, **scoped** to this story's items), 14 (a second `Query`/`EventStore`/`Tags`/`Event` on the page) | AC-005, AC-006, AC-007 | each checked against the rendered page at the report gate; 14 also mechanically by `contract_names_are_not_shadowed` |

Anti-patterns 6 through 13 and 15 belong to surfaces this story does not render — feature gates, the
README, the terminal transcript, the DSL failure message and the compile-fail fixture — and are
listed here only so their absence is not read as an oversight.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | `DomainEvent` is implemented with an **empty** `EVENT_TYPES` | Compile error, from a `const` item evaluated per-monomorphisation (RS-61-4), whose message names the event-type set. Never a runtime error and never a first-read failure. Carried by AC-003 |
| EC-002 | A well-formed boundary that constrains **nothing** calls `query()` | `Err(InvalidQuery::UnconstrainedItem)` (`crates/happenstance-core/src/error.rs:99-102`), documented under `# Errors` as *"this boundary constrains nothing, which is `Query::all()` and must be said out loud"*. Never silently widened, never `unwrap`ped, never absorbed here — M3's `commit`/`commit_with` is what absorbs it into `CommandError::Boundary` |
| EC-003 | `absorb` meets an event the model's query **never nominated** | Skipped, and skipped **by nomination** — not by a type check that happens to fail. Not an error: this is the *Absent* state `_design.md` `## The states the API must express` enumerates |
| EC-004 | `absorb` meets an event that **was** nominated and cannot be decoded | `Err(CodecError::UnknownEventType)`. This is a genuine `EVENT_TYPES`/fold disagreement and must be loud; degrading it to a skip would re-open exactly the hazard AC-001 closes |
| EC-005 | A codec fails to encode or decode a payload | `CodecError::Encode` / `CodecError::Decode`, carrying the codec's own error as a typed boxed `#[source]` (RS-30-2). Never a `String`; never an associated `Error` type on `Codec`, which would add a third type parameter to `CommandError`, `absorb`, `Decision` and the runner downstream |
| EC-006 | A downstream crate writes `impl Boundary for MyType` | Compile error: `Boundary`'s private `sealed::Sealed` supertrait is unnameable outside `happenstance`. This is the mechanism, not a lint — a divergent query must have nowhere to live (DR-02) |
| EC-007 | A doc link resolves under one feature configuration only | `cargo doc` warns, and the docs step of `cargo xtask ci --fast` fails. Nothing this story adds is gated, so this stays cheap — the obligation is to keep it that way, not to discover it later (RS-70-2; `_design.md` `## Mock`, finding 2) |
| EC-008 | Implementation reveals a **second** shortfall in the frozen contract (a D-1 sibling) | Recorded as a defect entry naming its clause ID and routed to `defect-log-and-macros-verdict` (M7) under project AC-012's route. Never absorbed quietly; never fixed by an edit under `crates/happenstance-core/src/**` (AC-A02) |

## Non-functional

| id | requirement | why, and how it is held |
| --- | --- | --- |
| NF-001 | **Nothing async, nothing `Send`-bound.** No `#[async_trait]`, no future, no `EventStore`/`SendEventStore` bound in any signature this story adds, no change to `read`'s shape | The cheapest possible satisfaction of binding constraints 1, 3 and 4 (`CLAUDE.md`) is to introduce no async surface at all. The four `wasm32` gate steps are unaffected; the question of a **fifth** step compiling `happenstance` itself is AC-A06's and belongs to `edge-flavour-and-wasm-claim` (M7) |
| NF-002 | **No revalidation on a hot path, and no allocation in `event_type()`.** `scope()` returns `&Tags` so validation is paid once in the caller's constructor; `event_type()` returns a `Cow::Borrowed` clone | The alternatives (`-> Tags` by value, `&[(&str, &str)]`) each move an error or a cost to the read (`_design.md` `## Shape decision`) |
| NF-003 | **No new external crate enters the dependency graph.** `serde` and `thiserror` are already `[workspace.dependencies]` (`Cargo.toml:48`, `:70`) | `standards/rust/50-dependency-hygiene.md`. A new third-party crate at this seam would be a graph decision this story is not licensed to take |
| NF-004 | **The MSRV stays 1.97.1** and `rust-toolchain.toml`'s pin is unchanged | Moving the floor is an ADR-recorded trade (ADR-0029), never a side effect. Nothing in this story's shape needs anything newer |
| NF-005 | **`cargo doc` is warning-free under default features and `--no-default-features`**, and `clippy -D warnings` is clean | Both are steps of `cargo xtask ci --fast`, which is this project's integration bar (`.redkiln/config.yaml:55`) |
| NF-006 | **No `unwrap`, no `expect`, no `panic!` in library code** | The residual `Result` is the standing temptation and its answer is defect candidate D-1, not an `unwrap` (`standards/rust/00-prime-directives.md`) |
| NF-007 | **The manifest change is additive.** `default-features = false` on `happenstance` continues to mean exactly what it means for `happenstance-core`, and the existing `serde` feature keeps its current forwarding meaning | AC-A04; the failure mode the manifest's own comment (`crates/happenstance/Cargo.toml:15-19`) exists to prevent, and *a feature only ever adds* (RS-51-1) |
| NF-008 | **The example keeps compiling, untouched.** `examples/course-subscriptions/` still builds against the unchanged glob re-export throughout this PR | Its rewrite is `worked-example-on-typed-layer`'s (M6). A story that forces the example to change has crossed its own PR boundary |

## Implementation notes (non-prescriptive)

Module names inside `crates/happenstance/src/` are the implementer's — the architecture brief says so
explicitly (`_decomposition.md`, *Where the code lands*). What follows is shape, not instruction.

- **A workable order.** `sealed` (private, one line) → `Codec`/`CodecError` (the vocabulary the
  `DomainEvent` signature is written in) → `DomainEvent` → `DecisionModel` → `Boundary` with its
  blanket impl, `query` and `absorb` → the crate-root `pub use` block and the module-doc edit → the
  vocabulary doctest and the `compile_fail` companion → the density test. The mount is not a final
  step to remember; it is what makes the preceding six exist as far as a reader is concerned.
- **The seal.** A private module with an empty public trait, and a blanket `impl<M: DecisionModel>
  Sealed for M {}` sitting beside the blanket `impl<M: DecisionModel> Boundary for M {}`. Pull
  `standards/rust/13-sealing-and-exhaustiveness.md` before writing it; the slice-mate will add tuple
  impls to both, so leave the seal shaped for that rather than closed against it.
- **The `const` assertion.** RS-61-4's per-monomorphisation form
  (`standards/rust/61-compile-time-assertions.md:249`) — a `const` item inside a generic context,
  forced by being referenced from the trait's own default body or from the blanket impl. A plain
  `const _: () = assert!(…)` at module scope does not see the implementor's associated const and
  will silently assert nothing.
- **The fixture domain.** The unit tests need a small two-variant `DomainEvent` + `DecisionModel`
  pair. Keep it crate-internal (`#[cfg(test)]`), and keep it distinct from the doctest's `Enrolment`
  so a change to one does not quietly repair the other. The public `testing::assert_domain_event`
  that would generalise it is M4's and must not be created here.
- **The doctest is a *vocabulary* doctest.** `_design.md`'s `## The doctest` calls `commit`, which
  does not exist until M3. Land the part that compiles today — define the events, define the model,
  derive the query, assert on it — written so M3 extends it rather than replaces it. Do not stub
  `commit` to make the full program compile early.
- **Three atoms, not the corpus.** The router's own instruction is one to three
  (`standards/rust/README.md`). For this story: `13-sealing-and-exhaustiveness`,
  `40-public-surface-and-evolution`, `70-rustdoc-obligations` — then `61-compile-time-assertions`
  and `62-doctests-and-harnesses` when you reach the `const` assertion and the `compile_fail` fence,
  and `30-error-taxonomy` when you write `CodecError`.
- **When the frozen contract is inconvenient**, write the defect down (clause ID + routing) and keep
  going. D-1 is already recorded; a second finding gets the same treatment and goes to M7. The one
  thing that is never available is editing `crates/happenstance-core/src/**`.
- **Do not answer the ceremony ratio.** `_design.md:1104-1111` predicts 2.4:1 and predicts AC-013's
  verdict from it. That prediction is checked against the *rewritten example* at project closeout. A
  derive macro added here would destroy the measurement AC-013 is waiting for.

## Tests and CI (merge gate)

Grounded in `_decomposition.md`'s Testing brief (the AC-001 and AC-014 rows, and the grain table at
its *Intent*) and in `.redkiln/config.yaml`'s `verify:` block, which wires these commands whether or
not anyone types them.

| tier | command / path | proves |
| --- | --- | --- |
| Unit | `cargo test -p happenstance --all-features`; tests beside the code in `crates/happenstance/src/` | AC-001 (exhaustive fold, `&Tags` scope, `Clone` not `Default`), AC-002 (the derivation's contents), AC-003 (the `Err` arm's meaning), AC-004 (`absorb`'s three outcomes, the typed `#[source]`), AC-005 (no contract name shadowed), AC-008 (the `EVENT_TYPES`/`event_type()` agreement, purity) |
| Doctest | collected by `cargo test --workspace --all-features`; the vocabulary doctest in `crates/happenstance/src/lib.rs`, item-page examples in `crates/happenstance/src/` | That the vocabulary is *usable*, not merely correct: the first program a reader copies compiles. This is the tier `_design.md`'s AC-U01 artefact is judged in |
| Compile-fail (doctest) | `compile_fail` fences on `DomainEvent` and `Boundary`'s item pages, paired with compiling ones (RS-62-1) | AC-003's empty-`EVENT_TYPES` error and AC-002's seal. **Not AC-002-the-project-criterion's instrument** — that is `trybuild`'s, still blocked, and `compile-fail-proof-artefact`'s (M6) |
| Density / file-reading | `doc_density_budget_holds` in `crates/happenstance/src/`, run by the unit tier | AC-007's six numeric budgets over authored doc comments and fences |
| Docs | `cargo doc -p happenstance --no-deps` and `… --no-default-features`, both warning-free — the docs step inside `cargo xtask ci --fast` | AC-005 (links resolve in every feature configuration; docs.rs metadata present), AC-006 (the intra-doc links land) |
| Static / reachability | `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:48`) | That no citation rots. This story discharges and amends **no** clause, so `spec-trace` must be green *unchanged* — a diff here would mean the story crossed into `spec/SPECIFICATION.md`, which its allowed globs forbid |
| Story gate | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | fmt + `clippy -D warnings` + tests for `happenstance` and its dependents, re-derived from git so untracked work is seen. The per-story bar |
| Integration bar | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | This project's declared integration bar: `REQUIRED` without `OPTIONAL` — all four `wasm32` steps stay in, the two feature powersets, `cargo deny` and the nightly `--cfg docsrs` build do not. `project.md` DoD 6 |
| Ledger | `redkiln verify --grain story` over `_ledger.md` (`require_ledger: true`, `.redkiln/config.yaml:67`) | That AC-001…AC-008 are each satisfied with cited evidence — not that a suite went green |
| Not this story's tier | feature powerset (`cargo hack`), `cargo deny`, `trybuild`, the example's execution test, `cargo xtask proof-artefact` | Named so their absence is deliberate: the powerset is a pre-publish concern (`_decomposition.md`, *The feature powerset is not this project's gate*), and the rest belong to M3, M6 and M7 |

## Risks and coupling (PR-scoped)

| risk / coupling | why it bites here | handling |
| --- | --- | --- |
| **The slice-mate builds on the seal.** `decision-model-composition` adds tuple `Boundary` impls to `sealed::Sealed` and `Boundary` in the same context | A seal written closed — private trait *and* a blanket impl that conflicts with tuple impls — makes the slice-mate's story unimplementable without reopening this one | Write the blanket impl over `M: DecisionModel` (not over a broader bound), and leave `Sealed` implementable inside the crate for tuples. The two stories are reviewed as one integrated surface |
| **M3 absorbs the `Result` this story leaves in place** | An implementer who "helpfully" makes `query()` infallible, or adds a `query_unchecked`, breaks `command-loop`'s `CommandError::Boundary` arm before it is written | The `Result` is a signed-off design row and an AC (AC-003). Changing it is a design change, not an implementation choice |
| **`Boundary` is sealed and ships with `absorb` whose only callers arrive in M3/M5** | A sealed trait cannot grow a required method later without the breaking change RS-40-1 forbids — and the crate publishes an alpha this project cuts | `absorb` lands **now**, complete, even though nothing calls it in this PR. An unused-method warning is not a reason to defer it |
| **The density budget is checked on a page this story does not solely own** | `_design.md` `## Mock` finding 3: `ConditionViolated`, `EventId`, `read_decision_model` and friends already exceed the 24-character and 80-character budgets, and every one arrives through `pub use happenstance_core::*` and is frozen by AC-A02 | AC-007 and anti-pattern 5 are **scoped to this story's own items**. See *Clarifications* — this is a scope call, not a relaxation |
| **The ceremony ratio invites a derive** | 2.4:1 against domain logic is uncomfortable to write, and `happenstance-macros` is a real option | It is AC-013's verdict, measured over the *rewritten example* at project closeout (`RUNBOOK.md:524`). Adding a derive here destroys the measurement. Record the discomfort; do not act on it |
| **The doctest cannot yet be the designed first program** | `_design.md`'s `## The doctest` calls `commit`; a reader who lands mid-M2 meets a partial page | Deliberate and bounded: the vocabulary doctest lands now, `first-program-doctest` completes at M3. AC-006 ensures the page is never *inconsistent* — no shipped item is described as "Planned" |
| **`serde` becomes a real dependency of `happenstance` for the first time** | ADR-0003 reads as a prohibition until the crate name is read carefully | DR-04 and AC-A04 settle it: ADR-0003 constrains `happenstance-core`, and ADR-0006 made this the typed layer whose job is encoding. `happenstance-core`'s default features are untouched, and the `--no-default-features` doc build of the contract crate already in the gate is the standing guard |
| **Blocked on M1's ingest wave** | ADR-0020 must exist as an accepted atom **before** the code it governs (AC-016 is a sequencing obligation), and `/redkiln:kb-ingest` is a human handoff, not a step inside a story | This story's `depends_on` names `adr-0020-fold-query-agreement`. If the atom is not yet at `.kb/decisions/0020-fold-query-agreement.md` when implementation starts, stop and escalate rather than proceeding from the staged intake document |

## Dependencies

**Blocks on**

- **`adr-0020-fold-query-agreement`** (M1, HS-S0018) — the decision record this story is the built
  consequence of. It resolves DT-2's concrete form and licenses every row of the shape above; the
  atom must exist under `.kb/decisions/` before this code is written (project AC-016; merge order
  step 1, `_storymap.md:109-115`).

**Unlocks**

- **`decision-model-composition`** (M2, the slice-mate) — adds tuple `Boundary` impls for arities
  2..=8 to the seal this story builds (`_storymap.md:52`).
- **`command-loop`** (M3) — `commit`/`commit_with` are written against `DecisionModel`, `Boundary`
  and the `Result` this story leaves in place (`_storymap.md:54`).
- Transitively, everything downstream of those two: `given-when-then-dsl` (M4),
  `worked-example-on-typed-layer` (M6) and `compile-fail-proof-artefact` (M6), which is the
  instrument that finally *observes* the protection this story builds.

**Not a dependency, deliberately.** `codec-and-feature-forwarding` (M3) is not blocking: this story
lands the `Codec` **trait** and `CodecError` only, because the signed-off `DomainEvent` signature is
written in them, and adding a required method to an already-sealed trait later is the move RS-40-1
forbids. `trybuild` and `MemoryProjectionStore` are blocked inputs elsewhere in this project and
touch nothing here.

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. Everything needed to *start* is in the Context pack
above; open these at the moment named. Every path was confirmed present in the worktree.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` | The **binding** signed-off surface: `## Signatures` is the exact text of every item this story writes, and `## Shape decision` carries the alternative that lost for each. Contradicting a row is a defect, not a design change | Before writing the first trait, and again beside every doc comment | AC-001, AC-002, AC-003, AC-004 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/adr-0020-fold-query-agreement/spec.md` | The M1 story that mints ADR-0020; its AC-002/AC-004/AC-005 rows enumerate the licensed shape, where the fallibility went and every rejected alternative. The atom itself lands at `.kb/decisions/0020-fold-query-agreement.md` via the ingest wave — read the atom if it exists, this spec if the wave has not run | At preflight, to confirm the record exists before the code it governs | AC-002, AC-003 |
| `crates/happenstance-core/src/event.rs` | `EventType::from_static` at `:108` is `const`, which is the whole reason `const EVENT_TYPES` needs no derive today; `:38-58` is the `Cow` payload that makes `event_type()` by value the right shape; `:95-106` is this repository's own warning about bare `compile_fail`; `:484-496` is the `SequencedEvent` `absorb` reads | Writing `DomainEvent`, and again writing the `compile_fail` fence | AC-001, AC-003, AC-004 |
| `crates/happenstance-core/src/query.rs` | `QueryItem::new` at `:56` is fallible and frozen — the source of the residual `Result`, and the reason an infallible `query()` is unreachable without an `unwrap` | Writing `Boundary::query` | AC-002, AC-003 |
| `crates/happenstance-core/src/error.rs` | `InvalidQuery` at `:93-105`, whose `UnconstrainedItem` variant (`:99-102`) is the `Err` arm's real meaning — *"this boundary constrains nothing"* | Writing `query()`'s `# Errors` block | AC-003 |
| `crates/happenstance-core/src/tag.rs` | `Tags::from_pairs` at `:304` is the only, fallible, way in — which is why `scope()` returns `&Tags` and validation is paid once in the caller's constructor | Writing `DecisionModel::scope` | AC-001 |
| `crates/happenstance-core/src/projection.rs` | `:47-61` is the contract crate's own words about the invalid-value hole a `Default` supertrait would re-open — the argument for `Clone`, in the repository's own voice | Writing `DecisionModel`'s supertrait, and its doc comment | AC-001 |
| `crates/happenstance/src/lib.rs` | The mount and the only render path: `:75` the surviving glob, `:41-46` the two bullets rewritten in place, `:53-56` the promise that contract paths are unchanged, `:10` the doctest-gated README pattern to extend rather than replace | At the mount step, before any `pub use` is written | AC-005, AC-006 |
| `crates/happenstance/Cargo.toml` | `:15-19`'s comment states the failure mode the additive manifest change must not create; `:22-26` is the existing `serde` feature whose meaning must not be repurposed | Adding `serde`/`thiserror` and the docs.rs metadata block | AC-005 |
| `standards/rust/13-sealing-and-exhaustiveness.md` | The seal is the mechanism that makes a hand-maintained query unwritable; getting the private-supertrait shape wrong makes the whole AC decorative | Before writing `sealed` and the blanket impl | AC-002 |
| `standards/rust/40-public-surface-and-evolution.md` | RS-40-1 (never add a required method to an existing trait — why `absorb` ships now) and RS-40-2 at `:73` (put a derivable convenience on a blanket ext trait — why `query` is on `Boundary`) | Before deciding where any method lives | AC-002, AC-004 |
| `standards/rust/61-compile-time-assertions.md` | RS-61-4 at `:249` is the per-monomorphisation `const` form; the module-scope form silently asserts nothing about an implementor's associated const | Writing the empty-`EVENT_TYPES` compile error | AC-003 |
| `standards/rust/62-doctests-and-harnesses.md` | RS-62-1: pair every `compile_fail` with a compiling example and do not trust its error code — the reason the fence is spelled bare | Writing the `compile_fail` companion | AC-003, AC-007 |
| `standards/rust/30-error-taxonomy.md` | RS-30-2: a typed `#[source]`, never a `String` — and why `Codec` gets no associated `Error` type | Writing `CodecError` | AC-004 |
| `standards/rust/70-rustdoc-obligations.md` | RS-70-2 (links must resolve in every feature configuration) and RS-70-5 (name the alternative that lost, once, where the reader is) — the two rules AC-005 and AC-006 are made of | Writing every doc comment, and before the final `cargo doc` pass | AC-005, AC-006 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md` | The four briefs: AC-A01…AC-A06 (Architecture), AC-U01…AC-U20 (UX), and the Testing brief's grain table that this story's CI section is grounded in | When a question is about *seams or instruments* rather than signatures | AC-005, AC-007, AC-008 |
| `examples/course-subscriptions/src/main.rs` | The hazard in this tree today: the two-item `Query` at `:114-125` and the fold at `:128-173` naming the same event set twice. It must keep compiling untouched through this PR | Before writing the derivation, to see what it removes | AC-002, AC-008 |
| `RUNBOOK.md` | `:3993-4007` states phase 7's fold/query hazard and the derived query in the plan's own terms; `:524` is AC-013's ceremony-ratio criterion, which this story must record and not pre-empt | When tempted to add a derive macro, or to check the phase's intent | AC-008 |

## Clarifications resolved during spec

1. **The AC set is exactly the front half's eight**, AC-001…AC-008, unchanged. They partition as:
   AC-001, AC-002 and AC-008 carry project **AC-001** (the fold and the query cannot disagree);
   AC-003 carries project **AC-014**'s built half (DT-2's chosen failure mode; the *record* is
   `adr-0020-fold-query-agreement`'s); AC-004 lands the vocabulary the signed-off `DomainEvent`
   signature is written in; AC-005, AC-006 and AC-007 are the mount and the rendered surface,
   without which the first three are true of the type checker and invisible to a reader.
2. **Anti-pattern 5 and the identifier/summary budgets are scoped to this story's own items.**
   `_design.md` `## Mock`, finding 3, records that `ConditionViolated`, `EventId`,
   `read_decision_model` and `MIN_SUPPORTED_EVENTS_PER_BATCH` already exceed the 24-character and
   80-character budgets, that **every one arrives through `pub use happenstance_core::*`**, and that
   every one is frozen by AC-A02 — and it says outright *"either the anti-pattern gains a scope or it
   is not checkable."* This spec takes the first branch, for this story only. *Rejected: check the
   budget over the whole rendered page* — that would make AC-007 unpassable by any change this story
   is permitted to make, which is a gate that blocks the wrong work.
3. **The doctest that lands is a vocabulary doctest, not `_design.md`'s complete first program.**
   That program calls `commit`, which is `command-loop`'s (M3). Stated in the Context pack as a slice
   seam; repeated here because it is the most likely thing to read as a shortfall against AC-U01. The
   full program lands with M3 and `first-program-doctest` renders then.
4. **Two `compile_fail` fences appear in this spec, and neither is project AC-002's instrument.**
   AC-002-the-project-criterion needs `trybuild` and its negative control, which remain blocked
   (`_decomposition.md`, *`trybuild` has no home yet*) and belong to `compile-fail-proof-artefact`
   (M6). The fences here are RS-62-1-paired doctests proving the seal and the empty-`EVENT_TYPES`
   error to *a reader*, which is a different claim and a weaker instrument, stated as such.
5. **The ADR-0020 anchor points at the M1 story's `spec.md`, not at an atom path.**
   `.kb/decisions/0020-fold-query-agreement.md` does not exist in the worktree yet — it is minted by
   the `/redkiln:kb-ingest` wave M1 hands off to — and this spec cites only paths that resolve today.
   Read the atom once it exists; the story spec is the standing description of what it will say.
6. **No conformance rule and no clause edit.** Settled in the Integration contract and repeated here
   because both are load-bearing negatives: a testkit rule about a derived query is one no adapter
   could fail (decorative by `CLAUDE.md`'s own standard), and `spec/SPECIFICATION.md` has no
   typed-layer namespace to discharge a clause in. `cargo xtask spec-trace` must be green *unchanged*.
7. **`Codec` and `CodecError` land here despite `codec-and-feature-forwarding` owning codecs.** The
   trait and the error type only — no `Json`, no `Cbor`, no `Postcard`, no `[features]` block, no
   codec-tag siting. `DomainEvent::encode`/`decode` are part of the signed-off signature and the
   trait cannot compile without them, and RS-40-1 forbids adding them to a sealed trait afterwards.
8. **Nothing was added to or dropped from the front half.** The scope lock, one-line slice, executive
   summary, context pack, integration contract, PR boundary, behaviour table and data section are
   preserved verbatim; the allowed globs and the merge DoD one-liner they declare are what this back
   half is written against.
