---
id: kb-reference-port-traits-compiled-findings-001
title: What the compiler said about the two port flavours
kind: reference
status: accepted
authority_tier: note
summary: >-
  The register of findings established by compiling EventStore and ProjectionStore rather than
  by reasoning about them, gathered from four ADRs written 2026-08-05 to 2026-08-08 against
  trait-variant 0.1.3 on rustc 1.97.1. LocalMemoryEventStore passes all twenty-seven conformance
  rules natively and under wasm-bindgen-test on wasm32, and its direct impl sits beside the
  blanket impl in a downstream crate with no error[E0119]. RefCell is Send and surrenders only
  Sync — Rc does the Send work, so CF-28 names the wrong type. JsValue is Send + Sync on
  non-atomics wasm32; Rc in happenstance-cloudflare's error is the real hazard, so ES-6's premise
  is half wrong. #[tokio::test] drives a !Send store because it expands to Runtime::block_on. The
  associated type cannot be varied between flavours — five spellings, five diagnostics. And
  dynosaur does not erase this port (error[E0277]), against ADR-0001's stated consequence; a
  hand-written wrapper does, with no unsafe.
depends_on: []
related:
  - kb-decision-0001
  - kb-decision-0008
  - kb-decision-0009
  - kb-decision-0010
  - kb-decision-0011
  - kb-open-question-es-6-unwritable-rule-001
  - kb-open-question-provisional-falsifiers-001
source_paths:
  - .kb/_intake/0001-async-port-flavours.md
  - .kb/_intake/0008-one-derivation-for-both-ports.md
  - .kb/_intake/0009-error-send-sync.md
  - .kb/_intake/0010-the-suite-must-prove-itself.md
  - .kb/_intake/0011-read-laziness-and-isolation.md
  - references/adr/0001-async-port-flavours.md
  - references/adr/0008-one-derivation-for-both-ports.md
  - references/adr/0009-error-send-sync.md
  - references/adr/0010-the-suite-must-prove-itself.md
  - references/adr/0011-read-laziness-and-isolation.md
  - crates/happenstance-core/src/store.rs
  - crates/happenstance-core/src/projection.rs
  - crates/happenstance-testkit/src/fixtures.rs
  - references/evaluation/phase-4-reconciliation.md
last_reviewed: 2026-08-10
---

# What the compiler said about the two port flavours

## What this is a pointer to

`happenstance-core` defines `EventStore` and `ProjectionStore` once and derives a `Send`
flavour of each with `#[trait_variant::make]` (ADR-0001, ADR-0008). Four ADRs, written across
four days, settled parts of that design by compiling small programs rather than by arguing from
the trait's text, and several of the resulting facts were then cited again by later ADRs. This
atom is the register those facts live in — cite it by id rather than re-deriving the compile.
The full reasoning for each stays in its own ADR; nothing here is a second copy of an argument,
only of the finding.

## The findings

**The bare, `!Send` flavour has a real implementer, and it clears the suite.**
`LocalMemoryEventStore` — `Rc<RefCell<Vec<SequencedEvent>>>`, implementing bare `EventStore` and
nothing else, in `happenstance-testkit`'s own `tests/` — passes all twenty-seven conformance
rules under three harnesses natively and under `wasm-bindgen-test` on `wasm32-unknown-unknown`
in CI. This is ADR-0001's own stated lift condition, met in-tree (ADR-0008).

**The direct impl and the blanket impl coexist with no coherence conflict.** `LocalMemoryEventStore`'s
direct `impl EventStore` sits beside `trait_variant`'s generated `impl<T: SendEventStore> EventStore
for T` in a genuinely downstream crate, and neither triggers `error[E0119]`. This is the coherence
claim ADR-0001 makes about the derivation's shape, compiled outside the crate that declares it.

**`RefCell` alone is `Send`; it surrenders only `Sync`.** A `!Send` reference store built purely
from `RefCell<Vec<_>>` would prove nothing about the `Send` flavour — `Rc` is what actually
withholds `Send`. CF-28's wording names the wrong type and should name `Rc` (ADR-0008).

**`#[tokio::test]` already drives a `!Send` store.** `tokio::spawn` requires `Send`; the attribute
expands to `Runtime::block_on`, which does not. CF-23's case for parameterising the test wrapper
stands, but its justification is `wasm32` portability, not `Send`-ness (ADR-0008, ADR-0010).

**`JsValue` is `Send + Sync` on non-`atomics` `wasm32`.** `wasm-bindgen` 0.2.126 carries
`unsafe impl Send for JsValue` and `unsafe impl Sync for JsValue`, gated on the absence of the
`atomics` target feature, and Workers builds without atomics. ES-6's premise — that a `JsValue` in
an adapter error is what makes it `!Send` — is therefore half wrong: the `JsValue` half of the
concern costs nothing, and the real hazard is an `Rc` held inside the error, as
`happenstance-cloudflare`'s `Rc<str>`-holding error demonstrates by failing `+ Send + Sync` with
four `error[E0277]` when the bound is added workspace-wide (ADR-0009).

**The associated `Error` type cannot be varied between the two flavours.** Five spellings were
tried and all five fail: naming the associated type inside the `#[trait_variant::make]` attribute
dies at `expected '+'`; a supertrait owning a stricter `Error` yields two `Error`s and `E0221`; a
where-clause on the bare trait is copied onto *both* flavours by `mk_variant`'s `..tr.clone()` and
additionally breaks the generated blanket impl with `error[E0275]: overflow evaluating the
requirement`. ES-5's conclusion holds; its cited mechanism was incomplete and should also name
`mk_variant` (ADR-0008, ADR-0009).

**`dynosaur` does not erase this port as ADR-0001 stated it would.** Attempting it fails with
`error[E0277]: dyn Stream cannot be unpinned`, because the macro's generated wrapper produces
`Box<dyn Stream>` and `Box<T>` is only a `Stream` when `T: Unpin`. A **hand-written** object-safe
wrapper trait with a blanket impl over `EventStore`, boxing and pinning at both `read` and
`append`, compiles and round-trips at runtime with **no `unsafe`** — because `Pin<Box<dyn Stream +
'a>>` is itself `Stream` via the standard blanket impl, and `Pin<Box<T>>` is `Unpin` whatever `T`
is. This corrects ADR-0001's consequences paragraph; the correction is ADR-0011's (E11).

## Provenance

Every claim above was reproduced against `trait-variant 0.1.3` on rustc 1.97.1, the pinned
toolchain (ADR-0029). The `LocalMemoryEventStore` and coherence findings are ADR-0001's lift
evidence, confirmed at ADR-0008; the `RefCell`/`Rc` and `#[tokio::test]` findings are ADR-0008's
and ADR-0010's, reached independently and stated identically; the `JsValue`/`Rc` finding is
ADR-0009's; the associated-type finding is ADR-0008's, extended by ADR-0009's compiled
confirmation that the same bound reports against `SendEventStore::Error` as well; the `dynosaur`
correction is ADR-0011's E11.
