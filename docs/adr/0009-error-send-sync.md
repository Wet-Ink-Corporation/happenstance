# ADR-0009: `Error` stays unbounded, and the strength goes in a marker

- **Status:** accepted
- **Date:** 2026-08-06
- **Settles:** ES-6, which has been `[DEFERRED]` since the specification was
  assembled, and PS-35's second half for `ProjectionStore`
- **Extends:** [ADR-0008](0008-one-derivation-for-both-ports.md), which recorded
  two facts for this decision and deliberately did not take it

## Context

Both ports declare the same thing, one line apart in two files:

```rust
type Error: core::error::Error + 'static;   // store.rs:100, projection.rs:73
```

No `Send`, no `Sync`. ES-6 asks whether that should change, and it is the highest
blast-radius open question in the workspace because ES-5 makes it a one-way door:
the bound cannot be added to one flavour later, so it is semver-visible and must
be decided before publish.

Both prior planning documents decided it by argument, in opposite directions,
from the same file. This ADR decides it by compiling four things, because the
question turned out to have a shape neither of them had noticed.

**Why it could not be settled before now.** The two in-tree "confirmations" were
free by construction: `MemoryStoreError` is uninhabited (`memory.rs:143-145`) and
`SqliteEventStoreError` was a single placeholder variant. Neither could fail the
bound if the bound were wrong. Phase 2 built the instrument that can —
`happenstance-cloudflare`, whose error holds an `Rc<str>` and is genuinely
`!Send`.

### Four things that were compiled

**1. The bound is nearly free, and the exception is the whole point.** Adding
`+ Send + Sync` to `store.rs:100` and running
`cargo check --workspace --all-features`: **every crate compiles except
`happenstance-cloudflare`**, which fails with four `error[E0277]`.

```text
error[E0277]: `Rc<str>` cannot be shared between threads safely
   --> crates\happenstance-cloudflare\src\event_store.rs:147:18
    |
147 |     type Error = CloudflareEventStoreError;
    = help: within `CloudflareEventStoreError`, the trait `Sync` is not
            implemented for `Rc<str>`
note: required by a bound in `happenstance_core::EventStore::Error`
   --> crates\happenstance-core\src\store.rs:100:45
```

Two sites × `Send` and `Sync`. The second site is `send_shape.rs:138`, and it
reports the bound against **`SendEventStore::Error` as well** from one edit to
one declaration — ES-5's claim that the associated type cannot be varied between
flavours, observed rather than argued. So the answer to the queue's second
question, *may the two flavours differ in it?*, is **no**, and it is not a policy
choice: there is no mechanism.

The one crate that fails is the `wasm32` target the entire two-trait design
exists to serve. A bound that costs nothing except the reason the design exists
costs everything.

**2. The specification's premise for ES-6 is half wrong.** The clause reasons
that "an adapter error holding a `JsValue` or an `Rc<str>` satisfies [the
unbounded type]". `wasm-bindgen` 0.2.126 carries

```rust
#[cfg(not(target_feature = "atomics"))]
unsafe impl Send for JsValue {}
#[cfg(not(target_feature = "atomics"))]
unsafe impl Sync for JsValue {}
```

and Workers builds `wasm32-unknown-unknown` **without** atomics. So `JsValue` —
and therefore `worker::Error`, including its `Internal(JsValue)` variant — is
`Send + Sync` there. **The `JsValue` half costs nothing; the `Rc` half costs
everything.** That is why the instrument holds an `Rc<str>` rather than mimicking
the `unsafe impl`: an instrument whose `!Send`-ness disappears under a `cfg`
cannot falsify a bound. It is also not available to us — this workspace sets
`unsafe_code = "forbid"`, so an adapter can only ever *inherit* that escape
hatch by holding a `JsValue`, never write it.

**3. Stringifying a `JsValue` loses a capability, not information the caller
needs — and the question was aimed slightly wrong anyway.** `JsThrow` keeps the
thrown value and can read its properties; `StringifiedThrow` keeps `String(value)`
and cannot. On the one question the port makes a caller ask — *was this a
conflict?* — they answer identically, because a Durable Object surfaces SQLite's
own text through the thrown `Error`'s `message` and exposes no numeric code. What
is lost is *forward* compatibility: a caller holding the live value can read a
field nobody has thought of yet.

But the conflict signal never travels in `Self::Error` on **any** adapter,
because `happenstance-core` lifts it into `AppendError::ConditionViolated`. The
adapter must classify the constraint violation *before* `Self::Error` is
constructed, whatever `Self::Error` is. `CloudflareEventStoreError` has no
`ConditionViolated` variant, and that absence is the finding made structural.

**4. The decisive one: the derived flavour does not imply a `Send` error, so
ES-6's named rule cannot be written against today's port for *any* adapter.**
`send_shape::send_flavour::SendStoreWithLocalError` implements `SendEventStore`
— `Send` store, `Send` stream, `Send` future — with a `!Send` `Error`, and
compiles.

`Send` on a future is a property of the values held **across** a suspension
point. `Send` on that future's `Output` is a property of a different type
entirely, and a future with no suspension points is `Send` whatever it returns.
So a rule that spawns an adapter's `append` future and reports "it compiled" has
checked the wrong obligation and passes against an error that can never cross a
`JoinHandle`. `store_error_crosses_a_join_handle` has to be taken literally: the
*error* crosses it.

Against today's port there is no bound to write it at. That is what makes
"leave ES-6 deferred" untenable — it defers a clause behind a rule that is not
merely unwritten but **unwritable**, which is the decorative-rule failure with an
extra step.

## Decision

**`Error` keeps its bound exactly as it is: `core::error::Error + 'static`, on
both ports, on both flavours. The stronger property becomes a separate marker
trait, and generic code that needs it asks for it.**

```rust
pub trait ThreadSafeEventStore: SendEventStore<Error: Send + Sync> {}
impl<S> ThreadSafeEventStore for S where S: SendEventStore<Error: Send + Sync> {}
```

Three properties, all compiled:

**It works.** `MemoryEventStore` satisfies it, and
`store_error_crosses_a_join_handle` — written against the marker, spawning
`append` and awaiting the `JoinHandle` so the *error type* crosses the boundary —
compiles and passes.

**It bites.** `SendStoreWithLocalError`, which satisfies every `Send` obligation
the derived flavour states, is rejected:

```text
error[E0277]: `Rc<str>` cannot be sent between threads safely
    = help: within `CloudflareEventStoreError`, the trait `Send` is not
            implemented for `Rc<str>`
note: required by a bound in `needs_a_thread_safe_store`
```

So ES-6's rule is writable after all, and the wrong implementation it rejects
already exists in the tree rather than needing to be invented for it.

**It does not belong in `happenstance-core`.** Both probes above define the
marker in a *downstream* crate, and both work. A local trait with a blanket impl
over a foreign one is ordinary coherence, so **the contract crate need not grow
anything at all**, and an application that wants the property can declare it
without waiting for us. Whether `happenstance-core` should ship the marker anyway,
as a convenience, is a naming-and-surface question for phase 4 — not a
capability question, which is what ES-6 was.

**The same answer covers `ProjectionStore` (PS-35).** Its declaration is
identical, ES-5's no-mechanism finding applies to `SendProjectionStore` the same
way, and the marker construction is a supertrait bound with a blanket impl, which
is not a provided method and so does not meet ADR-0008's GAT obstruction. One
scheme, both ports, again — and this time the consequences do not diverge, which
is worth saying because ADR-0008's did.

## Consequences

**Good.** The `wasm32` target keeps a `!Send` error, which is the concrete thing
ADR-0001 was defending in the abstract. Nothing pays for a native concern.

**Good.** ES-6 moves from `[DEFERRED]` to `[FROZEN]`, and its rule becomes
writable with a failing implementation already in the tree. The specification's
premise is corrected in the same change (see amendments below).

**Good.** No breaking change, and nothing to time against publication. A marker
added later is additive; the bound would not have been.

**Bad, and stated rather than hidden.** Generic code gets a second bound to
choose between, and choosing wrong is silent until someone spawns. CLAUDE.md's
fourth constraint — *bind `EventStore`, not `SendEventStore`* — now has a third
option under it, and the guidance is: bind `EventStore` unless you spawn, bind
`SendEventStore` if you spawn the future, bind the marker if the **error** has to
survive the spawn.

**Bad.** An adapter author whose error is accidentally `!Send` learns it from a
caller's bound rather than from their own crate. That is strictly better than
today, where they learn it from nobody, but it is not the same as being told at
the definition.

**Neutral.** Phase 1's `spawns_from_generic` collapses its read to a `usize`
"because `S::Error` has no `Send` bound". With the marker available, the test it
could not write becomes writable. It is not rewritten here: phase 3 owns the
suite.

## Alternatives rejected

- **Add `Send + Sync` to `Error`.** One line, and it passes the whole workspace
  except the crate the design exists for. Four `error[E0277]`, and no way to
  scope the bound to the native flavour because ES-5 has no mechanism. This is
  the option the evaluation's verdict imposed and the pressure test withdrew; the
  compiled reason is now on record rather than the argument.

- **Leave ES-6 deferred.** The honest-looking option, and the one the exit
  criterion explicitly permits. Rejected because finding 4 changes what the
  deferral means: it would defer behind a rule that cannot be written at all, and
  a clause waiting on an impossible experiment is not deferred, it is abandoned.

- **A second associated type**, `type SendError: Send + Sync` alongside `Error`.
  Doubles every adapter's error surface, and `trait_variant` copies associated
  types verbatim so the `!Send` flavour would carry it too — the exact failure
  ADR-0008 recorded for `type Batch: Send`.

- **Ship the marker in `happenstance-core` as part of this decision.** Tempting,
  and it may well be right. Rejected *here* because the probes show it is not
  necessary, and phase 2 is not authorised to change the contract crate. It is
  phase 4's, as a surface question, with the capability question already answered.

## Amendments this decision owes the specification

- **ES-6's premise is half wrong** and is corrected: `JsValue` is `Send + Sync`
  on non-`atomics` `wasm32`, so `worker::Error` is not the hazard. `Rc` is. The
  clause's conclusion is unaffected; its reason changes.
- **ES-6's rule needs its bound named.** `store_error_crosses_a_join_handle` must
  be stated as belonging to an opt-in marker-bound rule group, and must assert on
  the future's `Output` rather than on the future — a rule that only checks the
  future is `Send` passes against a `!Send` error and is decorative. ADR-0008
  flagged this; finding 4 is the compiled demonstration.
- **ES-6 moves to `[FROZEN]`**, and PS-35 with it.
