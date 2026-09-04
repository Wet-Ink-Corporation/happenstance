# happenstance-testkit

The conformance suite for [happenstance](https://github.com/Wet-Ink-Corporation/happenstance)
event store adapters. Ninety rules, each tracing to a MUST in the
[Dynamic Consistency Boundary specification](https://dcb.events/specification/)
and each shown to reject a named wrong implementation before it was trusted to
pass.

[![Buy me a coffee](https://img.shields.io/badge/buy%20me%20a%20coffee-support-yellow?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ryanbritton)

> **Status: early, and the reason has moved.** It used to be that
> deliberately-broken adapters passed the suite. That is no longer true of the
> event-store family: every rule now has at least one wrong store in this
> crate's own `tests/` that fails it, and a meta-test asserts each of those
> stores fails *exactly* the rules its registry row claims — so a rule that
> stopped discriminating is a red build rather than a green one.
>
> What is still early is everything around that. **`happenstance-sqlite` has
> run this suite** — its own front page says so in those words, *an adapter,
> and it has run the suite* (`crates/happenstance-sqlite/src/lib.rs:3`), and
> `tests/conformance.rs` mounts it three times — but every other storage crate
> is still a skeleton.
> The `ProjectionStore`
> suite is now all seventeen rules the specification names, each with a wrong
> store in this crate's `tests/` that fails it — but the port it checks is still
> `[PROVISIONAL]` and ships behind an off-by-default feature, because both
> fixtures that clear the suite are instruments this workspace wrote rather than
> adapters over storage it does not control. Several axes of the instrument
> portfolio also have no implementation at their far end — see
> [the specification](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/spec/SPECIFICATION.md)
> §6.2 and §6.5, which name them rather than summarising them.

## Use

```rust,ignore
happenstance_testkit::event_store_conformance!(MyFixture::new());
```

That expands to one `#[tokio::test]` per rule, so a failure names the rule rather
than a line number.

The expression builds a **fixture**, not a store. One fixture instance is one
isolated backing store; each `connect()` on it returns one handle onto that
store, and two instances share nothing. The distinction is the whole reason
`Fixture` is a trait rather than a closure: a bare `Fn() -> S` could not say
which of the two it meant, so no rule could call it twice — which foreclosed
durability, reopen and every genuinely multi-connection check at once.

A fixture also declares what it can do, as `Capability` associated constants. A
rule needing a capability your fixture declines still runs as a test and prints
your stated reason; it is never silently dropped from the binary, because a rule
that is absent is indistinguishable in CI output from a rule that passed.
`fixtures::MemoryFixture` is the worked example to read first.

### The model-based suite

```rust,ignore
happenstance_testkit::event_store_model_conformance!(MyFixture::new());
```

A second, **additive** family, behind this crate's off-by-default `proptest`
feature and absent on `wasm32-unknown-unknown`. It takes the same fixture and
runs your store through generated sequences of appends, conditional appends and
reads, checking each one against a model of what the log should be — and
*predicting* every conditional append's outcome before it calls you.

It replaces none of the named rules and catches nothing whose content is
concurrency, durability, a second handle or the empty batch. What it catches is
the combinatorial middle the named rules cannot enumerate: `query × from ×
backwards × limit × condition`, over a log it built rather than one it chose.

Positions are never generated. The generator emits a *symbolic* anchor — first,
middle, head, or one beyond the head — resolved at execution against what your
store actually assigned, which is why the same test runs unchanged against a
store whose positions are dense from 1 and one whose positions step by seven
from 4,096.

### The concurrency suite

```rust,ignore
happenstance_testkit::event_store_concurrency_conformance!(MyFixture::new());
```

A third, **additive**, and **opt-in** family. It starts eight contenders on real
OS threads against one backing store and checks five things the sequential rules
structurally cannot: exactly one winner among eight handlers that decided from
one snapshot; four *disjoint* consistency boundaries admitting exactly four
commits; positions unique under concurrent appends; `append` returning the
caller's own last position rather than the store's head; and a reader that never
sees a batch part-written.

Invoke it only if your handle is `Send` — the bound is
`F::Store: EventStore + Send`, and a `!Send` adapter is not expected to run it.
It is absent on `wasm32-unknown-unknown`, which has no threads to race on.

**There is no timeout in it, anywhere, and that is deliberate.** No rule in this
crate reads a clock or measures elapsed time; a wall-clock deadline passes on the
author's machine and fails on a loaded runner, and a flaky conformance suite is
worse than none. The consequence is worth knowing before you see it: an adapter
that *deadlocks* under contention hangs your CI job rather than failing a named
rule. If a run stops with no output, that is what happened.

Two contenders' `Result`s never meet, either. `EventStore::Error` carries no
`Send` bound, so each contender collapses its outcome to *committed*, *rejected*
or *failed, here is the message* before its thread ends.

### The projection suite

```rust,ignore
happenstance_testkit::projection_store_conformance!(MyProjectionFixture::new());
```

A fourth family, and the only one that checks a **different port**. It takes a
`ProjectionFixture` rather than a `Fixture` — one isolated projection store per
instance, each `connect()` one handle onto it — and expands to one test per
projection rule through its own single enumeration.

`ProjectionFixture::Store` is bound on `ProjectionProbe`, not on
`ProjectionStore`, and that is the load-bearing part. The probe is the write seam
the suite drives your read model through; without it, generic code holding your
batch can only commit it or roll it back, and the rule carrying this port's whole
reason for existing degenerates into a checkpoint test that a store writing
*only* checkpoints passes. Implement it beside your `ProjectionStore` impl —
it lives in `happenstance-core` behind the off-by-default `conformance` feature,
so it costs one flag on a dependency you already have and no new edge in your
dependency graph.

Pick a harness exactly as you would for the event-store family: the default arm
is `#[tokio::test]`, `__emit_projection_blocking` needs no runtime at all, and
`__emit_projection_wasm` routes a skipped rule's stated reason to `console_log!`
rather than to stdout, which does not exist on `wasm32-unknown-unknown`. The
default module name differs from the event-store family's, so one file may invoke
both. `fixtures::MemoryProjectionFixture` is the worked example.

**All seventeen rules §4.11 names, today**, each with a wrong store in this
crate's own `tests/` that fails it and is asserted to fail *exactly* the rules its
registry row declares. The port is still `[PROVISIONAL]` and lives behind
`happenstance-core`'s off-by-default `unstable-projection` feature: this suite is
what will freeze it, and what would clear that bar is an adapter over storage
this workspace does not control — which neither fixture shipped here is.

### The benchmark harness — which is *not* the bar

```rust,ignore
happenstance_testkit::event_store_benchmarks!(MyFixture::new());
```

Behind this crate's off-by-default `bench` feature, and absent on
`wasm32-unknown-unknown`. It is the one family that checks nothing: it adds no
conformance rule, it changes no adapter's bar, and **no result it produces can
fail a merge** — there is no threshold in it, at any budget. The specification
requires performance to be measured by a separate harness which is not part of
the conformance bar, and this is that harness.

Where the four families differ:

| Family | Feature | Checks | Can fail a merge |
| --- | --- | --- | --- |
| `event_store_conformance!` | — | the event-store bar | yes, and that is the point |
| `event_store_model_conformance!` | `proptest` | the same bar, over generated sequences | yes |
| `event_store_concurrency_conformance!` | — (opt-in, `Send`) | the bar under contention | yes |
| `event_store_benchmarks!` | `bench` | **nothing** — it measures | **no** |

Three scenarios: append throughput over a batch of *n*; conditional append under
*k* contenders, reporting the committed and rejected counts separately so that a
run in which nobody collided is legible as such; and replay of *N* events, once
unfiltered and once behind a tag filter. *n*, *k* and *N* are yours, at the call
site, because no constant here could be right for both an in-process `Vec` and a
file under a write lock.

**You bring the clock, and with it the measurement crate.** Nothing in this
crate's `src/` may read one, so the harness reports counts — events appended,
batches acknowledged, committed, rejected, refused, failed, events matched on
replay — and the per-scenario wrapper is the `emit` parameter, exactly as it is
for the other three families. `criterion`, `divan` or a line of CSV goes in
*your* `dev-dependencies`; this crate's stay `happenstance-core` and
`futures-core`. `tests/memory_benchmarks.rs` writes such an emitter and runs the
whole family against `fixtures::MemoryFixture` on every `cargo test --features
bench`, so the harness ships having been executed rather than merely compiled.

## Why this exists as a published crate

A claim about behaviour is worth exactly as much as the test that checks it.
Publishing the suite means an adapter author outside this repository can make the
same claim on the same terms, and it means the reference store is measured by the
same instrument as everyone else.

Two rules govern what goes in it, both learned expensively:

**A rule no adapter can fail is decorative.** Before a rule is added, a plausible
wrong implementation it rejects has to be named — and written into
`tests/mutation_coverage/mutants.rs`, with a row in that file's sibling
`REGISTRY` declaring the exact set of rules it fails. (A *concurrency* rule's
wrong store goes in `tests/mutation_coverage/racers.rs` and `RACERS`, because a
store that fails only a racing rule fails none of the named rules and cannot have
a `REGISTRY` row. A *projection* rule's goes in the sibling registry under
`tests/projection_mutation_coverage/`, which is a second registry rather than a
second table in the first one — a count printed beside the wrong target is a
number about neither.) That is not a convention:
`mutation_coverage::every_rule_has_a_mutant` fails until the row exists, so the
rule is demonstrated to fail before it is trusted to pass, and
`projection_mutation_coverage::every_projection_rule_has_a_mutant` says the same
thing for the other port. `CONTRIBUTING.md`'s "Conformance rules" section is the
checklist.

**Never assert on literal position values.** The specification permits gaps, and a
conformant adapter may leave them. Rules compare against positions the store
actually assigned — and `GappedPositionStore`, a *conformant* store assigning
positions in steps of seven from 4,096, is what fails a rule that forgets.

## Versioning

Adding a rule is a semver-*minor* change that can turn a passing adapter's CI red.
Treat that as a breaking change in practice and pin this crate exactly. Its
version is independent of the rest of the workspace for that reason.

## Licence

MIT OR Apache-2.0.
