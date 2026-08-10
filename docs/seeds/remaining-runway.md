# Seed — finishing happenstance

Raw material for `/redkiln:initiative`. This states a problem and a vision. It
deliberately does **not** decompose the work, name projects, or propose a design:
that is `/redkiln:plan`'s to decide, from its own grounding.

## Where this stands

happenstance is a storage-agnostic, DCB-compliant event sourcing library for Rust.
`happenstance-core` defines the contract, adapter crates implement it, and
`happenstance-testkit` decides whether they did.

Five bodies of work are finished, and they were sequenced by blast radius so that
the expensive-to-revise decisions were settled first. The ground is clear and three
crate names are owned. The two-flavour async port design — one definition without a
`Send` bound, a second derived — is proved rather than argued, with every rule green
against a `!Send` store on `wasm32`. Six skeleton crates compile on their real
targets with real associated types, chosen so that each sits at a different end of
some axis the contract might be wrong about. The conformance suite has been made
into an instrument: every rule has a registered wrong implementation that fails it,
and every wrong implementation fails exactly the rules it declares. The `EventStore`
contract and the values crossing it are frozen, against those skeletons and that
suite. The wire format is frozen, with negative controls asserted by name.

## The problem

What is finished is the half that can be settled by reasoning against a type
checker. What is left is the half that can only be settled by contact — with a
database, with a consumer, with a second implementation that disagrees, and with a
user who has installed the crate and cannot be asked to accept a correction.

Six things are true today and should not be:

**A port with no implementations is not a design, it is a guess.**
`ProjectionStore` is specified and `grep -rn "ProjectionStore for"` matches nothing
in the workspace. Its clauses are marked provisional for exactly that reason. The
invariant it exists to protect — that a read-model write and its checkpoint land in
one transaction — is documented on the trait and enforced by nothing.

**Nothing has used the contract the way an application would.** `happenstance`, the
crate people will actually install, is a five-line facade. A consumer is what
discovers contract defects, and there is no consumer.

**The contract has never met a database.** Every implementation that has passed the
suite is in memory or has `todo!()` bodies — and a `todo!()` body type-checks
against any signature at all, so "it compiles" has been evidence of nothing.
Durability and concurrency are the two properties memory gives away free.

**Every implementation so far has the same shape.** They all serialise their writers
and assign positions under a lock. A contract proved against four of those has been
proved against one storage shape wearing four hats. Two skeletons exist precisely
because they cannot do that — one assigns positions outside the transaction, one has
no connection and no cursor at all — and neither has been built.

**Nothing is published, so every promise is still private.** The minimum supported
compiler, the public surface, the semver commitment: each is an opinion nobody is
relying on. Some classes of mistake only become real, and become permanent, after a
release exists.

**Replication is unanswered, and so is what a store may forget.** A position is a
statement about one store's log and cannot cross a boundary unchanged. Whether a
receiving store re-checks the conditions a writer asserted is the central open
question. Separately, real stores do not keep everything forever — retention,
lawful deletion and crypto-shredding all produce a log with holes — and a reader
that quietly builds a wrong answer from a truncated log is the failure mode nothing
currently detects.

## The vision

A library an application author can `cargo add` and rely on, and that an adapter
author can implement against with a bar that tells them when they are done.

Concretely, that means: a contract that has been used, not just specified; adapters
that have run the conformance suite and passed it, across storage shapes that
genuinely disagree with each other; a published release whose promises are real; and
honest answers — not silence — on replication and on what happens when a store no
longer holds its whole history.

## Who it is for

**Application authors** doing dynamic consistency boundary event sourcing in Rust,
who today have no storage-agnostic option and must couple their domain to a
particular database.

**Adapter authors**, including this project's own, who need an executable definition
of "correct" rather than a prose specification to interpret.

**The local-first and edge case specifically.** The design carries a `!Send` port
flavour at real cost so that a store can run inside a Cloudflare Durable Object on
`wasm32`. That constraint is load-bearing and shapes everything; work that quietly
drops it has changed the product.

## What must remain true

These are constraints on any answer, not preferences:

- No `#[async_trait]`, ever. It injects `+ Send` and deletes the `wasm32` target.
- No `serde` in `happenstance-core`'s default features; payloads stay opaque bytes.
- An adapter that compiles but has not run the conformance suite is not an adapter.
- A rule no adapter can fail is decorative. Before adding one, name the wrong
  implementation it rejects.
- Evidence beats argument: a design is settled against something that compiles, or
  against a measurement, and the artefact that proves it must be one that **would
  not exist if the design were wrong**. A green gate is a precondition for looking
  at the evidence, never the evidence itself.
- Where any document and `docs/architecture/SPECIFICATION.md` disagree, the
  specification wins. A frozen clause changes by a new decision record, not an edit.

## Supporting material

Read these rather than trusting the summary above; several are long and all of them
are more specific.

| Path | What it carries |
| --- | --- |
| `docs/RUNBOOK.md` | The plan of record: what each remaining body of work is for, what would prove it, and what must not be reordered. Also the record of an adversarial pass that found eighteen defects in an earlier version of itself. |
| `docs/architecture/SPECIFICATION.md` | 200 numbered clauses across three ports, each with a maturity marker, the rule that checks it, and the wrong implementation it forbids. Current truth. |
| `docs/adr/` | Seventeen decision records — why each decision was taken, and when. History rather than current truth. |
| `docs/scenarios/E2E-CASES.md` | The end-to-end cases the library must satisfy, stated as observable behaviour. |
| `docs/evaluation/` | The research and the adversarial reviews the design rests on, including a pressure test that refuted an earlier plan's headline claim by compiling it. |
| `docs/experiments/` | Measurements, reproducible and deliberately outside the gate — including what four Postgres position-visibility mechanisms actually cost, and a rustc crash this architecture reaches by construction. |
| `docs/adapter-shapes.md` | What the six skeletons told the type checker. |
| `CLAUDE.md` | The operating constraints, and who the work is being done with. |

## Deliberately not decided here

How this decomposes. The runbook sequences the remaining work as nine phases with a
dependency graph and an estimate, and that sequencing is evidence worth reading —
but whether a phase is a project, whether several collapse into one, and where the
release boundaries fall are decomposition questions, and they belong to planning
rather than to this seed.
