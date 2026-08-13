---
item: HS-S0024
stage: discover
created: 2026-08-12T13:01:45.622Z
updated: 2026-08-12T13:01:45.622Z
template_sig: 86ce4036
rendered_sig: ccdd1e0e
---

# Discover — FaultyStore<S> and GappyMemoryStore in the testkit

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice: land `FaultyStore<S>` and `GappyMemoryStore` in `crates/happenstance-testkit/src/` — **two types per flavour, not one** — reachable from the testkit root, each shipped with the wrong implementation it rejects: a retry loop that branches on `Some(conflicting_position)`, and a handler that assumes `position + 1` | `_storymap.md:55` (M4 row) | The slice states the mutants. A fixture that cannot fail a named wrong implementation is decorative, and this repository has already paid for one |
| AC-009 — a consumer can test a decision **and** can test misbehaviour; `FaultyStore<S>` and `GappyMemoryStore` exist in `happenstance-testkit` and a handler assuming `position + 1` fails against the latter | `project.md:192-196` | AC-009 is split by crate: the stores are this story's, the DSL is `given-when-then-dsl`'s (`_storymap.md:83`) |
| `depends_on`: none | `_storymap.md:55`; manifest `dependsOn: []` | This is why M4 leads with it, and why M4 can be merged in parallel with M5 (`_storymap.md:120-135`) |
| **`FaultyStore` is two types.** `trait_variant` emits a blanket `impl<T: SendPort> Port for T`, so one type cannot carry both flavours; the diagnostic is `error[E0119]`. Load `standards/rust/20-two-flavour-ports.md` (RS-20-4) and `21-send-is-not-inherited.md` **before** writing either wrapper rather than rediscovering it from the compiler | `_decomposition.md:572-584`; `_design.md:650`, `:608-609` | `FaultyStore<S: EventStore>` and `SendFaultyStore<S: SendEventStore>` are both named in the design's item list |
| The binding surface: `FaultyStore::new(inner)`, `violate_next(n)` — *"fails the next `n` appends with `ConditionViolated`, reporting `conflicting_position: None` — which a remote store legitimately does"* — and `fail_next_read(n)`; `GappyMemoryStore::with_stride(NonZeroU64)` — *"assigns positions with a stride, so `position + 1` is never the next one"* | `_design.md:606-626` | `violate_next` reporting `None` is not an implementation detail. It is the property that makes the fixture able to fail the command loop's mutant |
| `conflicting_position` is `Option` because an adapter with no interactive transaction reports `None` legitimately, and *"a retry loop that branches on this field being `Some` works against an in-process store and stops working against a remote one"* | `crates/happenstance-core/src/error.rs:135-147` | The contract names the mutant. `FaultyStore` is the instrument that makes it fail in-process |
| `head()` answers "am I caught up?" by comparison, never by subtraction, *because positions may have gaps* — which is also why `GappyMemoryStore` exists, and why the conformance suite may not assert literal positions | `_decomposition.md:515-519`; `CLAUDE.md`, *The rule that matters* | The gap fixture and the no-literal-positions rule are the same fact seen from two sides |
| These land in the **testkit**, not in `happenstance`: they are instruments for other people's tests, and the testkit carries an independent version for exactly this class of change (CF-32), asserted by a gate step | `_decomposition.md:437-441`, `:424`; `_design.md:696-700` | Putting them in `happenstance` couples an application's dependency graph to a test double |
| The in-tree pattern to follow: `crates/happenstance-testkit/src/fixtures.rs` already holds `MemoryFixture`, the reference `Fixture` implementation, and the crate's capability-declension idiom — a declined capability reports its stated reason rather than vanishing from the binary | `_grounding.md:124-132`; `CLAUDE.md`, *The rule that matters*; `_decomposition.md:481-486` | A new fixture-shaped type follows `MemoryFixture`'s shape; a declined capability says why (RS-40-5, AC-U20) |
| A rule that no adapter can fail is decorative: before adding one, name a plausible wrong implementation it rejects, and write that implementation into the testkit's own `tests/` if one is not already there | `CLAUDE.md`, *The rule that matters*, first corollary | This story's whole value is the two named mutants. Shipping the types without them ships nothing |
| `GappyMemoryStore` is the fixture AC-006's cousin needs too: an off-by-one on a checkpoint resume is *"invisible for an idempotent projection, corrupting for a counter"* | `_decomposition.md:509-514` | The gap fixture serves the projection runner as well as the command path; both consumers are in this project |

## Questions

**Answered here.**

- *One type or two for `FaultyStore`?* Two, and it is coherence rather than taste:
  `trait_variant`'s blanket impl makes a single type carrying both flavours `error[E0119]`
  (`_decomposition.md:572-584`; `_design.md:650`).
- *Does `GappyMemoryStore` need a `Send` twin?* The design lists only `GappyMemoryStore`
  (`_design.md:362-366`, `:619-626`), gated on the testkit's `memory` feature. If a
  `Send`-flavour consumer appears, the same RS-20-4 rule applies and it is a second type,
  not a second impl.
- *Why does `violate_next` report `None`?* Because that is the case no in-process store can
  produce and the case a remote store produces routinely
  (`crates/happenstance-core/src/error.rs:135-147`). A `FaultyStore` that reported `Some`
  would be a fixture that cannot fail the mutant it exists to catch.
- *Are these conformance rules?* No. They are **fixtures** — misbehaving stores other people
  drive their own tests with. No rule is added to the conformance suite by this story, and
  no adapter's CI turns red because of it.

**Deferred to `spec`.**

- *The stride's default, and whether gaps are uniform or arbitrary.* `with_stride` is the
  named constructor (`_design.md:625`); whether a second, irregular-gap constructor ships is
  spec-grain. Discovery's constraint: whatever it is, no test asserts the resulting
  positions literally.
- *Whether either type also implements `Fixture` and joins `event_store_conformance!`.*
  `MemoryFixture` is the pattern if so (`_decomposition.md:481-486`); the design does not
  require it, and a fixture that declines a capability must say why rather than disappear.
- *The testkit's own version bump.* CF-32 gives it an independent number; the alpha's value
  is settled at `0.2.0-alpha.1` by the design's sign-off (`_design.md:1252`) and executed by
  `publish-0-2-0-alpha-1`.

**Not blocked.** Neither `trybuild` nor `MemoryProjectionStore` is an input to this story.

## Decision

The problem this slice solves is that every store a consumer of this library can currently
test against behaves perfectly: `MemoryEventStore` always names the conflicting event and
always assigns the next position, so a caller's retry loop and a caller's checkpoint
arithmetic are both untestable — not hard to test, *untestable*, because the input that
distinguishes correct from incorrect never occurs. This story ships the two stores that
produce those inputs on demand, in the crate whose job is instruments for other people's
tests: one that fails appends with `ConditionViolated` and reports no conflicting position,
and one that assigns positions with a stride so that `position + 1` is never the next one.
The spec for this story covers both types in both flavours where coherence requires it
(`FaultyStore<S: EventStore>` and `SendFaultyStore<S: SendEventStore>`, per RS-20-4), their
builder surfaces and `#[must_use]` markers, their crate-root reachability from
`crates/happenstance-testkit/src/lib.rs`, the `memory` feature gate on `GappyMemoryStore`,
the doc comments naming what each fixture is *for* — including the named wrong
implementation each rejects — and, critically, the two mutant tests themselves living in the
testkit's own `tests/`, so the fixtures are demonstrated failing something rather than
merely existing. No `[FROZEN]` clause is amended, and no conformance rule is added.

## The wrong implementation

**The mutant: a `FaultyStore` that misbehaves conveniently.**

```rust
pub fn violate_next(self, n: u32) -> Self { /* … */ }

// inside the injected failure:
Err(AppendError::ConditionViolated(ConditionViolated {
    conflicting_position: Some(self.inner.head().await?),   // "more useful for tests"
    ..Default::default()
}))
```

It is entirely plausible — reporting the position looks strictly more informative, and it
makes the fixture's own unit tests easier to write. And it passes everything: the type
compiles, `cargo xtask ci` is green, a retry loop driven against it retries and succeeds,
the DSL's retry demonstration passes, and AC-009 reads as satisfied because
`FaultyStore<S>` exists in `happenstance-testkit` and injects append failures.

It is wrong because the fixture then has **exactly the same behaviour as
`MemoryEventStore`** on the one axis it was built to vary. The mutant it is supposed to
catch — the command loop branching on `Some(conflicting_position)`
(`crates/happenstance-core/src/error.rs:135-147`) — survives untouched, and the repository
now contains an instrument that proves nothing while looking like proof. That is precisely
the corollary CLAUDE.md draws from experience: a rule no adapter can fail is decorative, and
before adding one you name the wrong implementation it rejects. The design pins the correct
behaviour in the constructor's own doc — `conflicting_position: None`, *"which a remote store
legitimately does"* (`_design.md:613-615`) — and this story's spec must carry a test that
drives `commit` against `FaultyStore::…violate_next(1)` and asserts it **succeeds** on the
second attempt. Under the convenient mutant that test passes for the wrong reason, so the
test must additionally assert the reported `conflicting_position` is `None`.

**A second mutant, and it is the one this repository forbids by name: a `GappyMemoryStore`
test that asserts literal positions.**

```rust
let positions = /* … append three events … */;
assert_eq!(positions, [1, 3, 5]);   // stride 2
```

Deterministic, readable, green. And it is the exact habit the conformance suite forbids —
*"never assert on literal position values… compare against positions the store actually
assigned"* (`CLAUDE.md`, *The rule that matters*) — planted in the crate whose job is to
teach adapter authors how to test. It also fails to test the thing that matters: the point
of the fixture is that a handler computing `previous + 1` **disagrees with the store**, and
`[1, 3, 5]` asserts the store's private allocation policy instead. The assertion that earns
its place captures the positions the store assigned and asserts that some consecutive pair
differs by more than one — and, in the paired mutant test, that a handler written as
`expected = previous + 1` fails against it.

**A third mutant: one type with both flavour impls.** Writing `impl<S: EventStore> EventStore
for FaultyStore<S>` *and* `impl<S: SendEventStore> SendEventStore for FaultyStore<S>` does
not compile — `error[E0119]`, because `trait_variant` already emits the blanket impl
(`_decomposition.md:572-584`). It is named here not because it survives a check but because
it is the shape an author reaches for first, and the cost of rediscovering it from the
compiler is an afternoon spent arguing with coherence instead of five minutes reading
RS-20-4.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

The two judgement boxes, and the first is this story's central discipline rather than a
formality. **Literal positions:** no conformance rule is added — these are fixtures, not
rules, so no adapter's CI can turn red because of them — and the box is therefore
vacuously true of the suite. It is ticked on a stronger basis than vacuity: the one test
this story writes that observes positions at all is `GappyMemoryStore`'s, and discovery has
fixed its form as a comparison against the positions the store actually assigned, with the
literal-assertion form named above as the mutant to reject. **Frozen clauses:** nothing
under `crates/happenstance-core/src/**` is touched; `AppendError`, `ConditionViolated` and
`SequencePosition` are consumed as the frozen contract defines them, and `conflicting_position`
being `Option` is the clause behaviour these fixtures exist to *exercise*, not to change.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
