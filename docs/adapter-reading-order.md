# What to read before you write an adapter

> **Answers:** `explanation` — What should I read, and in what order, before writing an adapter?

This page is for you if you want to connect happenstance to a storage engine it
does not support yet. That connection is called an **adapter**: a type that
implements the `EventStore` contract over your database, so application code
written against happenstance runs on it unchanged.

Everything you need to write one is already written down, but it is spread over
six places. This page puts them in the order you will need them and says why
each one matters. By the end you will know what an adapter has to promise, the
shape it usually takes, and why the Rust traits look the way they do.

That last point matters. When the compiler stops you, you need to tell "the
contract is wrong for my store" apart from "I have not understood the contract
yet". The sources below are what let you tell the difference.

1. **The runnable loop, and what it is not.** The in-memory store's walk-through.
   First, because it runs; with a warning, because it is not a model adapter.
2. **The recipe, with a non-memory example.** The adapter authoring recipe, and
   its compiled PostgreSQL-shaped store.
3. **The same four steps, as a checklist.** `CONTRIBUTING.md`, which names the bar
   an adapter must clear.
4. **Why there are two flavours.** Why the contract is two traits,
   `EventStore` and `SendEventStore`.
5. **What removes `Send`, and what removes `Sync`.** Which of your own types will
   decide your flavour for you.
6. **Why a provided method is never `async fn`.** Only needed once you add a
   method to the contract itself.

## 1 of 6 — the runnable loop, and what it is not

**Why read it:** it is the only place the whole read, decide, append loop runs
from start to finish against a real store implementation.

Read "The reference implementation" in
[`crates/happenstance-core/src/memory.rs`](../crates/happenstance-core/src/memory.rs).
It is compiled and run as a test, so it cannot fall out of step with the API it
shows. Read it for the order of the calls an application makes.

**But it is not an adapter to copy.** The file itself lists three reasons
`MemoryEventStore` exists: it is the reference the conformance suite is checked
against, it makes the examples runnable, and it lets you write application code
before a real store exists.

It also has one particular storage shape. It handles one writer at a time under a
lock, and numbers events with no gaps while holding that lock. That is one way
to build a store, and not a required one: positions may have gaps
([VT-11](../spec/SPECIFICATION.md#vt-11--positions-are-unique-strictly-increasing-in-assignment-order-and-may-have-gaps)).
Copying `RwLock<Vec<_>>` as "the shape an adapter takes" is the mistake this
entry is here to prevent.

For a store at the other extreme, see the `PgStore` in the recipe (entry 2),
which assigns positions outside its transaction.
[`references/adapter-shapes.md`](../references/adapter-shapes.md) records what
six early adapter sketches taught the type checker.

## 2 of 6 — the recipe, with a non-`memory` example

**Why read it:** it is the step-by-step method, with an example that looks like a
real database adapter rather than the in-memory store.

[`standards/rust/91-adapter-authoring-recipe.md`](../standards/rust/91-adapter-authoring-recipe.md)
is six steps, in order. The first, RS-91-1 (implement the strongest flavour your
own types allow), is the one every later step depends on.

It carries a compiled `PgStore`, the only adapter example in the repository that
is not the in-memory store, so the shape you copy is not the one entry 1 just
showed you. Its step 4 is where the **fixture** arrives: the small type that
tells the conformance suite how to open your store
([CF-15](../spec/SPECIFICATION.md#63-the-fixture-contract)).

## 3 of 6 — the same four steps in contributor voice

**Why read it:** it is the shortest statement of the steps, and it states the bar
your adapter has to clear before anyone calls it an adapter.

Read "Writing an adapter" in
[`CONTRIBUTING.md`](../CONTRIBUTING.md#writing-an-adapter) after the recipe, not
before. The recipe carries the compiled example; this carries the obligations.
In short: implement the contract, write a fixture, run the conformance suite,
and make it pass.

One obligation there is not checked by any test, because no test can read a doc
comment. Your store has to say what happens when an `append` future is dropped
before it finishes
([ES-23](../spec/SPECIFICATION.md#es-23--cancellation-outcome-is-unspecified-and-that-is-the-contract)).
Answer it in your store module under a `# Cancellation` heading.

Each published adapter keeps its answer in place with a small test you can copy:
`crates/happenstance-<name>/tests/cancellation_statement.rs`, for `sqlite`,
`cloudflare`, `postgres` and `neon`.

If your adapter lives in this repository, also add your test to the list called
`FROZEN_DOC_MUSTS` in `xtask/src/lint_narrative.rs`, which names those four
targets. `cargo test -p xtask` fails until the list and the files on disk agree.

## 4 of 6 — why there are two flavours

**Why read it:** you will probably meet the two traits first as a confusing
compiler error, `error[E0034]`, "multiple applicable items in scope". Knowing why
they exist makes that error obvious.

Here is the idea in plain terms. In Rust, a value is `Send` if it can safely be
moved to another thread. Most server code runs on a multi-threaded runtime such
as tokio, which may move a task between threads, so it needs the store and the
futures it returns to be `Send`. But some runtimes are single-threaded, and the
natural types there are not `Send`. A Cloudflare Durable Object, compiled to
`wasm32`, is the main example.

So the contract is defined once and comes in two flavours.
`SendEventStore` promises `Send`; `EventStore`, the bare flavour, does not. Every
`SendEventStore` is automatically an `EventStore`. An adapter implements the
strongest one it can, and application code binds the weaker `EventStore`, so it
accepts every store.

The `E0034` error appears when both trait names are imported into one module:
each supplies a `read` method, and the compiler cannot tell which you meant.
Import only the one you bind on.

The rule itself, RS-20-1, lives in
[`standards/rust/20-two-flavour-ports.md`](../standards/rust/20-two-flavour-ports.md),
with a compiled example that fails the build if importing both ever stops
producing `E0034`. Why the project chose this design over the alternatives is
in [ADR-0001](../.kb/decisions/0001-async-port-flavours.md).

## 5 of 6 — what removes `Send`, and what removes `Sync`

**Why read it:** entry 4 tells you there are two flavours, but not which one your
own types will allow. This one does.

[`standards/rust/25-what-removes-send-and-sync.md`](../standards/rust/25-what-removes-send-and-sync.md)
answers it, starting with RS-25-1: an `Rc` makes a type `!Send`, and a `RefCell`
makes it `!Sync` while leaving it `Send`. (`Sync` means a shared reference to the
value can be used from several threads at once.)

Its "Load when" line is written as the compiler errors you will already be
looking at, so you can find it by the error. Open it the moment your store will
not satisfy `SendEventStore`.

## 6 of 6 — why a provided method is never `async fn`

**Why read it:** you only need it if you add a method with a default body to the
contract itself. It comes last because most adapter authors never do.

Read "Adding a method to a port" in
[`CONTRIBUTING.md`](../CONTRIBUTING.md#adding-a-method-to-a-port). The short
version: the tool that derives the `Send` flavour copies a default body without
rewriting it, so an `async fn` default turns into a compile error, and you write
it as a function returning a future instead.

It is the shortest of the six. The decision behind it is
[ADR-0008](../.kb/decisions/0008-one-derivation-for-both-ports.md).

## What this page does not cover

This page does not tell you whether the contract suits your store, and it does
not choose how your store checks an append condition. It puts the six sources in
order and explains why each matters; the sources themselves hold the detail.

Every entry above assumes you know why an append re-checks what its decision
read. If that is not yet clear, read
[why an append re-reads its condition](append-conditions.md) first.

When your store implements the contract,
[pass the conformance suite](pass-conformance-suite.md) takes you from there to
a passing test run.
