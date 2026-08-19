# The live borrowed projection batch the port no longer admits

`LiveHandleProjectionStore` bound a **genuinely borrowed, genuinely live**
transaction handle — `type Batch<'a> = GraphWriteHandle<'a>` — to
`ProjectionStore` on the **`Send`** flavour, with real bodies rather than
`todo!()`. It compiled. That is the whole reason it exists, and the whole reason
it is here rather than deleted.

## Why it moved, and by whose decision

[ADR-0017](../../.kb/decisions/0017-what-a-projection-batch-owns.md) made
`ProjectionStore::Batch` an owned associated type with no lifetime parameter.
Under `type Batch;` this impl cannot be written as it stands: there is no
lifetime to bind `GraphWriteHandle<'a>` to.

The atom names its disposition and this directory executes that arm verbatim:

> `LiveHandleProjectionStore` is not deleted — it moves to
> `experiments/live-handle-projection-batch/`, because it is the only compiled
> evidence against this decision's own first claim, and `experiments/` is this
> workspace's home for a reproducible measurement kept outside the gate.

Nothing was chosen here. Deleting the only compiled evidence *against* a
decision, in the same change that executes the decision, is how a port gets
frozen against its own hypothesis.

## What it refuted

`spec/SPECIFICATION.md` §4.2 argued the GAT lifetime off the port on `Send`
grounds: `type Batch<'a> = rusqlite::Transaction<'a>` compiles on the bare
flavour and fails on `SendProjectionStore` twice over, because `Connection` is
`Send` and not `Sync` and `Transaction<'_>` is not `Send` at all.

Both halves of that are properties of **rusqlite**, not of live handles.
LadybugDB's `Connection` is `Send` *and* `Sync`, so `&Self` is `Send` and
`GraphWriteHandle` is `Send`. Neither objection survives, and this file is the
counter-example that shows it — `Send`-ness was never the obstacle.

ADR-0017 accepted that refutation and rested the clause on two *other*
transcripts instead, both preserved in this file's module documentation:

1. **`error[E0195]`** on every impl that spells its concrete batch type rather
   than the literal `Self::Batch<'_>` — the trap with nothing to copy from, and
   the entire explanation for zero adapters.
2. **A `DefId::expect_local` ICE** on any store that itself carries a lifetime,
   proving today's GAT port is implementable only by stores that outlive every
   batch they hand out. The minimised reproduction is a separate experiment:
   [`../rustc-ice-gat-foreign-trait/`](../rustc-ice-gat-foreign-trait/).

So the decision stands, and it stands on evidence this file does not contradict.
What this file contradicts is the argument the specification *first* reached
for — which is exactly the kind of thing that must survive the decision it lost
to.

## What is here

[`live_handle.rs`](live_handle.rs) — the module verbatim as it stood at the last
commit in which it compiled, moved with `git mv` so `git log --follow` reaches
its history. It is **not** in any crate's module tree and is **not** built by
`cargo xtask ci`: it names `ProjectionStore` with a GAT that the port no longer
declares, so it cannot compile against the current contract crate and is kept as
a record rather than as a target.

To read it against the port it was written for, check out the commit before the
one that moved it:

```console
git log --follow --oneline -- experiments/live-handle-projection-batch/live_handle.rs
git show <the commit before the move>:crates/happenstance-ladybug/src/live_handle.rs
```

## What would bring it back

A projection-store port that admits a borrowed batch without the two costs
above — no `E0195` for an implementer spelling its own type, and no region
obligation that ICEs for a store carrying a lifetime. ADR-0017 records PS-15's
falsifier in the same spirit: a zero-cost type-level construction that names an
*instance*, composes with `async fn`, and still lets a batch sit in a
collection. If either appears, this directory is where the argument against the
current shape is already written down.
