# What to read before you write an adapter

> **Answers:** `explanation` — In what order do I read what already exists, to build an adapter?

Six sources, in the order an adapter author needs them — each cited, none restated.

1. **The runnable loop, and what it is not** — `crates/happenstance-core/src/memory.rs`, "The
   reference implementation". First because it runs, caveated because it is not an adapter.
2. **The recipe, with a non-`memory` example** — `standards/rust/91-adapter-authoring-recipe.md`,
   RS-91-1..4 and its compiled `PgStore`. Second, for the other end of the storage axis.
3. **The same four steps in contributor voice** — `CONTRIBUTING.md`, "Writing an adapter". Third
   because it is the shortest, and it names the bar an adapter must clear.
4. **Why there are two flavours** — `standards/rust/20-two-flavour-ports.md`, RS-20-1. Fourth, not
   first: you meet this as `error[E0034]` before you meet it as a rule.
5. **What removes `Send`, and what removes `Sync`** —
   `standards/rust/25-what-removes-send-and-sync.md`, RS-25-1. Fifth, because entry 4 leaves you
   asking it.
6. **Why a provided method is never `async fn`** — `CONTRIBUTING.md`, "Adding a method to a port".
   Last, because it only bites once you are adding one.

## 1 of 6 — the runnable loop, and what it is not

The walk-through under "The reference implementation" in
`crates/happenstance-core/src/memory.rs` is the only place the whole read-decide-append loop runs
end to end, and it is a doctest, so it cannot drift from the API it demonstrates. Read it for the
shape of a call sequence.

**And it is not an adapter.** That file's own three-reasons list says what `MemoryEventStore` is:
the oracle the conformance suite is validated against, the thing that makes this crate's examples
runnable, and the store application code is written against before a real one exists. It serialises
its writers under a lock and assigns dense positions inside that lock — one storage shape, and not
a required one (VT-11). Generalising `RwLock<Vec<_>>` into "the shape an adapter takes" is the
wrong lesson this page exists to prevent. The other end of that axis is the `PgStore` in
`standards/rust/91-adapter-authoring-recipe.md`, which assigns positions outside the transaction,
and `references/adapter-shapes.md` records what six skeletons told the type checker.

## 2 of 6 — the recipe, with a non-`memory` example

`standards/rust/91-adapter-authoring-recipe.md` is six steps in order, and RS-91-1 is the one every
step after it depends on. It carries a compiled `PgStore` — the only adapter example in this tree
that is not `memory` — so the shape you copy is not the shape you were just shown. Its step 4 is
where the fixture contract (CF-15) arrives.

## 3 of 6 — the same four steps in contributor voice

`CONTRIBUTING.md`'s "Writing an adapter" is the shortest of the three statements and the one that
names the bar an adapter must clear. Read it after the recipe atom rather than before: the atom
carries the compiled example, and this carries the obligation.

## 4 of 6 — why there are two flavours

`standards/rust/20-two-flavour-ports.md` is where the rule lives — RS-20-1 — and it carries the
compiled negative that fails the build if importing both flavours ever stops producing
`error[E0034]`. The decision underneath it is ADR-0001,
`.kb/decisions/0001-async-port-flavours.md`. This page states neither the rule nor the decision,
because those two documents already do.

## 5 of 6 — what removes `Send`, and what removes `Sync`

Entry 4 leaves you holding a question it does not answer: which of your own types will refuse the
`Send` flavour. `standards/rust/25-what-removes-send-and-sync.md` is that answer — RS-25-1 first —
and its `Load when:` line is written as the diagnostics you will already be looking at. Open it the
moment a store will not satisfy `SendEventStore`.

## 6 of 6 — why a provided method is never `async fn`

`CONTRIBUTING.md`'s "Adding a method to a port" is last because it only bites once you are adding
one, and it is the shortest of the six. What one derivation and a provided body owe each other is
ADR-0008, `.kb/decisions/0008-one-derivation-for-both-ports.md`.

## What this page does not cover

It does not tell you whether this port is right for your store, it does not decide your
append-condition strategy, and it restates none of the six sources — it orders them.

One hop from here, and only one: every entry above assumes you already know why an append re-reads
what it decided on, and that is [why an append re-reads its condition](append-conditions.md).
